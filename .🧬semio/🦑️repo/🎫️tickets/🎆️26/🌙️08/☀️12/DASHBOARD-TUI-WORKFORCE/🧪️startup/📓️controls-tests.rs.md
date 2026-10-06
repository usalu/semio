use super::*;

#[test]
fn native_leader_bytes_match_the_shared_control_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⌨️controls/🔣️.json")).unwrap();
    let bytes: Vec<u8> = fixture["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut parser = ui_tui::tui::ansi::AnsiParser::new(); let mut events = Vec::new(); parser.feed(&bytes, &mut events);
    let projection: Vec<_> = events.into_iter().map(|event| match event { Event::Key(KeyEvent { key: Key::Char(character), mods: modifiers }) => serde_json::json!({"key": character.to_string(), "ctrl": modifiers & mods::CTRL != 0}), _ => panic!("unexpected control event") }).collect();
    assert_eq!(serde_json::Value::Array(projection), fixture["events"]);
}

#[test]
fn filtered_command_selection_survives_inventory_updates() {
    let (mut dashboard, mut tui) = dashboard(Locale::English);
    let leaf = CommandLeaf::Repo(RepoAction::GoalsList);
    dashboard.commands = vec![("build / other".into(), leaf.clone()), ("test / first".into(), leaf.clone()), ("test / second".into(), leaf.clone())];
    dashboard.replace_view(&mut tui, 0, "launcher");
    if let Some(WidgetState::Wizard(state)) = tui.scene.node_mut(dashboard.windows[0].focus).widget() { state.filter = "test".into(); state.selected = 1; }
    dashboard.commands.insert(0, ("test / added".into(), leaf));
    dashboard.refresh_views(&mut tui);
    if let NodeContent::Widget(WidgetState::Wizard(state)) = &tui.scene.node(dashboard.windows[0].focus).content { assert_eq!(state.options[state.visible_indices()[state.selected]], "test / second"); } else { panic!("launcher missing"); }
}
