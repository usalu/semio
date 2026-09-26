
/// ⌨️ LAW: a typing run far longer than the store's fixed applied-edit ledger (64) — the SDK typing run (1137 typed
/// characters with pauses, caret moves and corrections) typed inside the snapshot's `notes` string, so every changed key
/// delivers the whole snapshot JSON the vcs text editor shows — saves every key through the retained `textEdit` job, and ONE
/// undo reverts the whole run, ONE redo restores it (ticket 26/09/23 F1: the retained edit work emitted one uncoalesced edit
/// per keystroke).
#[semio_framework_async_macros::async_test]
async fn a_typing_run_longer_than_the_edit_ledger_saves_and_undoes_as_one_step() {
    let run = semio_framework_plugin::artifact_app_laws::typing_run();
    assert!(run.texts.len() > 64 * 10, "the run must outlast the edit ledger many times over");
    let mut instance = app().await;
    dispatch(&mut instance, VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: "notes".into(), value: run.initial.clone() })).await;
    let start = instance.snapshot().expect("snapshot");
    assert_eq!(start.notes, run.initial);
    for (index, notes) in run.texts.iter().enumerate() {
        let mut next = start.clone();
        next.notes = notes.clone();
        let typed = dispatch(&mut instance, VcsCommand::TextEdit(text_edit::TextEdit { text: serde_json::to_string(&next).expect("snapshot json") })).await;
        assert!(typed.edited_document(), "keystroke {index} was not saved");
    }
    assert_eq!(instance.snapshot().expect("snapshot").notes, run.expected);
    for (verb, expected) in [("undo", &run.initial), ("redo", &run.expected)] {
        let admitted = instance.handle_action(verb, None, &meta("local")).await.unwrap_or_else(|fault| panic!("{verb} admission: {fault:?}"));
        settle_reserved(&mut instance, admitted).await;
        assert_eq!(&instance.snapshot().expect("snapshot").notes, expected, "one {verb} moves the whole run");
    }
}
