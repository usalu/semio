import pathlib

engine = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine")
fixture = engine / "🧫️fixtures/⏯️native-guest-journey/🔣️.json"
fixture.write_text("""{
  "$comment": "⏯️ Language-agnostic journey of one real guest mounted by a native shell on the kernel thread, with no hub: the open relay's arguments, the verb it authors, the framework reserved verbs it then drives (undo, redo, the selection and clipboard verbs) and what the guest's own ledger must show after each. Read by the native `a_native_guest_*` laws and by the two-user hub law, so all of them drive the one kind the staged native runtime carries. block2d has no clipboard producer of its own, so its framework clipboard routes answer empty and leave the ledger as it was.",
  "variant": "block2d",
  "profile": "release",
  "pluginId": "block",
  "artifactRef": "s.block.block2d@1/*",
  "appId": "s.block.block2d@1/*#editor",
  "schema": "block.2d",
  "verb": "addHandleKind",
  "undo": "undo",
  "redo": "redo",
  "clipboard": ["selectAll", "copy", "paste"],
  "expectedEditsAfterVerb": 1,
  "expectedAppliedAfterUndo": 0,
  "expectedAppliedAfterRedo": 1,
  "expectedEditsAfterClipboard": 0
}
""")

laws = engine / "🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs"
text = laws.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


replace(
    """    verb: String,
    undo: String,
    expected_edits_after_verb: usize,
    expected_applied_after_undo: usize,
}
""",
    """    verb: String,
    undo: String,
    redo: String,
    clipboard: Vec<String>,
    expected_edits_after_verb: usize,
    expected_applied_after_undo: usize,
    expected_applied_after_redo: usize,
    expected_edits_after_clipboard: usize,
}
""",
)

start = text.index("/// ⏪️ The same journey, then the fixture's undo: it settles and reverts exactly the authored edit.")
end = text.index("/// 🌱️ One native wgpu shell creates a hub-bound artifact through its own creation door against a")
text = text[:start] + """/// 🧮️ How many of `ledger`'s edits are `verb`'s and applied.
#[cfg(not(target_arch = "wasm32"))]
fn applied_count(ledger: &[(String, bool)], verb: &str) -> usize {
    ledger.iter().filter(|(action, applied)| action == verb && *applied).count()
}

/// ⏪️ [`native_guest_authored`], then the fixture's undo, asserted to settle and to revert exactly the
/// authored edit. Answers the shell for a law that continues.
#[cfg(not(target_arch = "wasm32"))]
fn native_guest_undone(journey: &NativeGuestJourney) -> ShellState {
    let mut shell = native_guest_authored(journey);
    let (undo, latency) = author_edit(&mut shell, &journey.undo);
    let undone = applied_edits(&mut shell);
    println!("native-guest-journey undo outcome={undo:?} latency={latency:?} ledger {undone:?}");
    assert_eq!(undo, Ok(()), "undo settles");
    assert_eq!(applied_count(&undone, &journey.verb), journey.expected_applied_after_undo, "undo reverted exactly the authored edit");
    shell
}

/// ⏪️ The journey's edit, then its undo. Undo is a framework reserved tool verb: the guest admits it and
/// spawns a `semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND` job, so this law holds the native
/// kernel to starting that job live, stepping it to its end and settling the guest's completion turn
/// — the one reserved-job mechanism the React host drives in `driveReservedToolJob` (ticket 26/09/23
/// slice WG8, §1.4).
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_undoes_its_authored_edit_without_a_hub() {
    let _ = native_guest_undone(&native_guest_journey());
}

/// ⏩️ The journey's edit and undo, then the fixture's redo: a second reserved tool job on the same
/// instance, which settles and applies exactly the undone edit again.
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_redoes_its_undone_edit_without_a_hub() {
    let journey = native_guest_journey();
    let mut shell = native_guest_undone(&journey);
    let (redo, latency) = author_edit(&mut shell, &journey.redo);
    let redone = applied_edits(&mut shell);
    println!("native-guest-journey redo outcome={redo:?} latency={latency:?} ledger {redone:?}");
    assert_eq!(redo, Ok(()), "redo settles");
    assert_eq!(applied_count(&redone, &journey.verb), journey.expected_applied_after_redo, "redo applied exactly the undone edit again");
}

/// 📋️ The journey's edit, then the fixture's selection and clipboard verbs in order (select all, copy,
/// paste): each is a framework reserved verb — the selection one a spawned reserved tool job, the
/// clipboard ones run inside the guest's turn — and each settles, leaving the edit ledger exactly as the
/// fixture declares.
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_copies_and_pastes_its_selection_without_a_hub() {
    let journey = native_guest_journey();
    let mut shell = native_guest_authored(&journey);
    let before = applied_edits(&mut shell);
    for verb in &journey.clipboard {
        let (outcome, latency) = author_edit(&mut shell, verb);
        println!("native-guest-journey clipboard verb={verb} outcome={outcome:?} latency={latency:?}");
        assert_eq!(outcome, Ok(()), "{verb} settles");
    }
    let after = applied_edits(&mut shell);
    println!("native-guest-journey clipboard ledger {before:?} -> {after:?}");
    assert_eq!(after.len(), before.len() + journey.expected_edits_after_clipboard, "the clipboard verbs left the declared edits");
    assert_eq!(applied_count(&after, &journey.verb), applied_count(&before, &journey.verb), "the authored edit stays applied");
}

""" + text[end:]

replace(
    "    assert_eq!(after.iter().filter(|(action, applied)| *action == journey.verb && *applied).count(), before.iter().filter(|(action, applied)| *action == journey.verb && *applied).count() + journey.expected_edits_after_verb);\n",
    "    assert_eq!(applied_count(&after, &journey.verb), applied_count(&before, &journey.verb) + journey.expected_edits_after_verb);\n",
)

laws.write_text(text)
print("ok")
