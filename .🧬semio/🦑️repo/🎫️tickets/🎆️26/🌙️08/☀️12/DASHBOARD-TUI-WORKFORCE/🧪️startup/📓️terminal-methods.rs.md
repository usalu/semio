    fn refresh_wizard_for(&mut self, tui: &mut Tui, idx: usize) {
        let (widget, options) = match &self.windows[idx].body {
            WindowBody::Launcher { widget } => (*widget, self.commands.iter().map(|(label, _)| label.clone()).collect()),
            WindowBody::Overview { widget } => {
                let mut options = vec![self.text("New task — type to find a command", "Neue Aufgabe — Befehl suchen"), self.text("Settings", "Einstellungen"), self.text("Refresh commands", "Befehle aktualisieren"), self.text("Cancel discovery", "Suche abbrechen")];
                options.extend(self.sessions.values().map(|session| format!("[{}{}] {} {}", match session.status { SessionStatus::Running => self.text("running", "läuft"), SessionStatus::Stopping => self.text("stopping", "stoppt"), SessionStatus::Exited => self.text("exited", "beendet"), SessionStatus::Failed => self.text("failed", "fehlgeschlagen") }, session.code.map(|code| format!(" · exit {code}")).unwrap_or_default(), session.command.cmd, session.command.args.join(" "))));
                (*widget, options)
            }
            WindowBody::Settings { widget } => (*widget, vec![format!("{}: {}", self.text("Language", "Sprache"), self.preferences.language), format!("{}: {}", self.text("Appearance", "Darstellung"), self.preferences.appearance), format!("{}: {}", self.text("Terminology", "Begriffe"), self.preferences.terminology), format!("{}: {}", self.text("Preferred renderer", "Bevorzugter Renderer"), self.preferences.renderer), format!("{}: {}", self.text("Output layout", "Ausgabelayout"), self.preferences.layout), format!("{}: {}", self.text("Save scope", "Speicherbereich"), if self.shared_preferences { self.text("workspace shared", "Arbeitsbereich gemeinsam") } else { self.text("local only", "nur lokal") }), self.text("Back to tasks", "Zurück zu Aufgaben")]),
            _ => return,
        };
        if let Some(WidgetState::Wizard(state)) = tui.scene.node_mut(widget).widget() {
            let selected = state.options.get(state.selected).cloned();
            state.options = options;
            state.selected = selected.and_then(|label| state.options.iter().position(|option| *option == label)).unwrap_or(0);
            state.offset = 0;
        }
    }

    fn refresh_views(&mut self, tui: &mut Tui) { for index in 0..self.windows.len() { self.refresh_wizard_for(tui, index); } }

    fn queue_preference(&mut self, change: crate::preferences::Change) {
        if self.saving_preferences.is_some() { self.connection_status = self.text("Saving preferences; try again shortly", "Einstellungen werden gespeichert; gleich erneut versuchen"); return; }
        let path = if self.shared_preferences { crate::preferences::shared_path(&self.root) } else { self.preference_path.clone() };
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || { let result = crate::preferences::append(&path, &change); let _ = sender.send((change, result)); });
        self.saving_preferences = Some(receiver);
        self.connection_status = self.text("Saving preferences", "Einstellungen speichern");
    }

    fn poll_inventory(&mut self, tui: &mut Tui) -> bool {
        let mut changed = false;
        if let Some((change, result)) = self.saving_preferences.as_ref().and_then(|receiver| receiver.try_recv().ok()) {
            self.saving_preferences = None;
            match result {
                Ok(_) => {
                    self.preferences.apply(&change);
                    self.locale = if self.preferences.language == "de" { Locale::German } else { Locale::English };
                    self.light = self.preferences.appearance == "light";
                    tui.set_appearance(if self.light { AppearanceName::Light } else { AppearanceName::Dark });
                    self.connection_status = self.text("Preferences saved", "Einstellungen gespeichert");
                    self.refresh_views(tui);
                }
                Err(error) => self.connection_status = error.to_string(),
            }
            changed = true;
        }
        let updates: Vec<_> = self.inventory.as_ref().map(|job| job.receiver.try_iter().collect()).unwrap_or_default();
        for update in updates {
            match update {
                crate::inventory::Update::Ready(tree, status) => {
                    self.commands = crate::inventory::commands(&tree); self.tree = tree;
                    self.inventory_status = status; self.refresh_views(tui);
                }
                crate::inventory::Update::Finished => { self.inventory = None; }
                crate::inventory::Update::Failed(error) => self.inventory_status = error,
            }
            changed = true;
        }
        changed
    }

    fn refresh_inventory(&mut self) {
        self.inventory = Some(crate::inventory::start(self.root.clone(), true));
        self.inventory_status = self.text("Discovering commands", "Befehle suchen");
    }

    fn cancel_inventory(&mut self) {
        self.inventory = None;
        self.inventory_status = self.text("Discovery cancelled; available commands retained", "Suche abgebrochen; verfügbare Befehle bleiben");
    }

    fn replace_view(&mut self, tui: &mut Tui, index: usize, mode: &str) {
        let id = self.windows[index].id.clone(); let chrome = self.windows[index].chrome;
        match self.windows[index].body { WindowBody::Overview { widget } | WindowBody::Launcher { widget } | WindowBody::Settings { widget } => tui.scene.remove(widget), WindowBody::Output { .. } => return }
        self.windows[index] = self.attach_view(tui, chrome, &id, mode);
        self.focused = id; self.terminal_input = false;
        tui.set_focus(Some(self.windows[index].focus)); self.remount(tui);
    }

    fn show_home(&mut self, tui: &mut Tui) {
        if let Some(window) = self.windows.iter().find(|window| matches!(window.body, WindowBody::Overview { .. })) {
            self.focused = window.id.clone(); self.terminal_input = false; tui.set_focus(Some(window.focus));
            activate_stack_tab(&mut self.layout, &self.focused); self.remount(tui);
            return;
        }
        let id = format!("w{}", self.next_serial); self.next_serial += 1;
        push_window_to_stack(&mut self.layout, &self.focused, WindowLayoutWindowNode { window_kind_id: id.clone(), title: None, corner: None });
        let window = self.add_wizard_window(tui, id.clone(), "Tasks"); self.windows.push(window);
        self.replace_view(tui, self.windows.len() - 1, "overview");
    }

