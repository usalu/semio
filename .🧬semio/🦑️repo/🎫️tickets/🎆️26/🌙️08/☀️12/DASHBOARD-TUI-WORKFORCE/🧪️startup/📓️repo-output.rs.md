    fn show_repo_output(&mut self, tui: &mut Tui, win_id: &str, action: &RepoAction) {
        let Ok(executable) = std::env::current_exe() else { return; };
        let Ok(encoded) = serde_json::to_string(action) else { return; };
        self.spawn_output(tui, win_id, CommandSpec { cmd: executable.display().to_string(), args: vec!["repo-view".into(), "--action".into(), encoded], cwd: self.root.clone(), env: Vec::new() });
    }

