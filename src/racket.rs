use zed_extension_api::{self as zed, LanguageServerId, Result};

struct RacketExtension;

impl RacketExtension {
    fn language_server_binary_path(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("racket") {
            return Ok(path);
        }
        return Err(format!("Could not find racket executable. Please ensure that Racket is installed and available in your PATH.").into());
    }
}

impl zed::Extension for RacketExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["-l".into(), "racket-langserver".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(RacketExtension);
