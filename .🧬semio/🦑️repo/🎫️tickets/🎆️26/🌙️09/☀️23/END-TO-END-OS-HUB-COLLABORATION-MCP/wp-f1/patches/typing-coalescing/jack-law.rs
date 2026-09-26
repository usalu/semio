/// 🔎️ The query the jack editor window shows (its text-editor scene buffer).
async fn jack_query(app: &mut JackTestApp, editor: &ViewModel) -> String {
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, editor).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene").buffer
}

/// ⌨️ A typing run far longer than the store's fixed applied-edit ledger (64) — 1137 typed characters with pauses, caret moves
/// and corrections, one full-text `text-edit` per changed key — keeps saving (every keystroke amends the run's ONE coalesced
/// window-config edit), and ONE undo reverts the whole run, ONE redo restores it (ticket 26/09/23 F1: the 65th character was
/// refused with `batched publication requires preinstalled fixed applied and revision capacity`).
#[semio_framework_async_macros::async_test]
async fn a_typing_run_longer_than_the_edit_ledger_keeps_saving_and_undoes_as_one_step() {
    let run = artifact_app_laws::typing_run();
    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    let meta_of = || semio_framework_plugin::ActionMeta { view_state: Some(editor.clone()), ..meta("local") };
    let before = jack_query(&mut app, &editor).await;
    for (index, text) in run.texts.iter().enumerate() {
        app.dispatch_typed(TrinityJackCommand::TextEdit { text: text.clone() }, &meta_of()).await.unwrap_or_else(|error| panic!("keystroke {index} was refused: {error:?}"));
        drive_query_ownership_operations(&mut app).await.unwrap_or_else(|error| panic!("keystroke {index} did not publish: {error}"));
    }
    assert_eq!(jack_query(&mut app, &editor).await, run.expected);
    for (verb, expected) in [("undo", &before), ("redo", &run.expected)] {
        let admitted = app.handle_action(verb, None, &meta_of()).await.unwrap_or_else(|fault| panic!("{verb} admission: {fault:?}"));
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("{verb} settles: {fault:?}"));
        drive_query_ownership_operations(&mut app).await.unwrap_or_else(|error| panic!("{verb} publishes: {error}"));
        assert_eq!(&jack_query(&mut app, &editor).await, expected, "one {verb} moves the whole run");
    }
}

