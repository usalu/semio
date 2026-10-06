    fn handle_wizard_signal(&mut self, tui: &mut Tui, win_id: &str, signal: WidgetSignal) {
        let Some(index) = self.windows.iter().position(|window| window.id == win_id) else { return; };
        match signal {
            WidgetSignal::Activated(selected) => match self.windows[index].body {
                WindowBody::Overview { .. } => match selected {
                    0 => self.replace_view(tui, index, "launcher"),
                    1 => { self.replace_view(tui, index, "settings"); self.refresh_wizard_for(tui, index); }
                    2 => self.refresh_inventory(),
                    3 => self.cancel_inventory(),
                    _ => {
                        if let Some(session) = self.sessions.values().nth(selected - 4).cloned() {
                            self.hidden.remove(&session.session_id); self.update_session(tui, session.clone());
                            if let Some(window) = self.windows.iter().find(|window| matches!(&window.body, WindowBody::Output { session: Some(info), .. } if info.session_id == session.session_id)) {
                                self.focused = window.id.clone(); self.terminal_input = false;
                                activate_stack_tab(&mut self.layout, &self.focused); tui.set_focus(Some(window.focus)); self.remount(tui);
                            }
                            self.restore_sessions();
                        }
                    }
                },
                WindowBody::Launcher { .. } => {
                    if let Some((_, leaf)) = self.commands.get(selected).cloned() {
                        match leaf { CommandLeaf::Process(mut spec) => { self.preferences.bind(&mut spec); self.spawn_output(tui, win_id, spec); }, CommandLeaf::Repo(action) => self.show_repo_output(tui, win_id, &action) }
                    }
                }
                WindowBody::Settings { .. } => {
                    let mut change = crate::preferences::Change::default();
                    match selected {
                        0 => change.language = Some(if self.preferences.language == "en" { "de" } else { "en" }.into()),
                        1 => change.appearance = Some(if self.light { "dark" } else { "light" }.into()),
                        2 => change.terminology = Some(if self.preferences.terminology == "native" { "reuse" } else { "native" }.into()),
                        3 => change.renderer = Some(match self.preferences.renderer.as_str() { "react" => "wgpu-wasm", "wgpu-wasm" => "wgpu-native", _ => "react" }.into()),
                        4 => change.layout = Some(match self.preferences.layout.as_str() { "tabs" => "columns", "columns" => "rows", _ => "tabs" }.into()),
                        5 => { self.shared_preferences = !self.shared_preferences; self.refresh_wizard_for(tui, index); return; }
                        _ => { self.replace_view(tui, index, "overview"); self.refresh_wizard_for(tui, index); return; }
                    }
                    self.queue_preference(change);
                }
                _ => {}
            },
            WidgetSignal::NavigateBack => { self.replace_view(tui, index, "overview"); self.refresh_wizard_for(tui, index); }
            _ => {}
        }
    }

