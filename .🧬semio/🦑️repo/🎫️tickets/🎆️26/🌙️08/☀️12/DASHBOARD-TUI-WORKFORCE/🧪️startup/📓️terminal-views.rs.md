    fn attach_view(&self, tui: &mut Tui, chrome: NodeId, id: &str, mode: &str) -> DashboardWindow {
        let options = if mode == "launcher" { self.commands.iter().map(|(label, _)| label.clone()).collect() } else { vec![self.text("New task — type to find a command", "Neue Aufgabe — Befehl suchen"), self.text("Settings", "Einstellungen"), self.text("Refresh commands", "Befehle aktualisieren"), self.text("Cancel discovery", "Suche abbrechen")] };
        let mut state = WizardState::new(options);
        if mode == "launcher" { state.steps = vec![(self.text("Type to search · Enter runs · Backspace returns", "Tippen zum Suchen · Enter startet · Rücktaste zurück"), String::new())]; }
        let widget = tui.scene.add(chrome, Node::new(NodeContent::Widget(WidgetState::Wizard(state))));
        tui.scene.node_mut(widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
        if let Some(ChromeState::Window(window)) = tui.scene.node_mut(chrome).chrome() { window.title = match mode { "launcher" => self.text("Commands", "Befehle"), "settings" => self.text("Settings", "Einstellungen"), _ => self.text("Tasks", "Aufgaben") }; }
        DashboardWindow { id: id.into(), chrome, body: match mode { "launcher" => WindowBody::Launcher { widget }, "settings" => WindowBody::Settings { widget }, _ => WindowBody::Overview { widget } }, focus: widget }
    }

    fn attach_wizard(&self, tui: &mut Tui, chrome: NodeId, id: &str) -> DashboardWindow { self.attach_view(tui, chrome, id, "launcher") }

