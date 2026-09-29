"""📜️ C12: one-shot, idempotent amendment of `c12-sdk-composition-patch.py` (coordinator 11:1x, flat per-key cost — the history
half). Reading the history of a long coalesced typing run cost O(run) per key: the rebuilt view copied every printed op line of
the growing row, the panel joined them all into its description, and every dirty `HistoryEntry` export cloned them. A row now
previews its edit's LAST `HISTORY_ROW_OPERATION_PREVIEW` operations and records the operation count it printed at
(`CommandView::op_count`): a sealed row's preview is reused while its edit still has that many operations, the growing row is
re-printed in O(preview). Adds the SDK law and the `op_count` of every hand-built `CommandView`. Usage: python3 <this>"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PATCH = os.path.join(HERE, "c12-sdk-composition-patch.py")
text = open(PATCH, encoding="utf-8").read()
if "PREVIEW_CONST_OLD = " in text:
    print("already amended")
    raise SystemExit(0)


def swap(old, new):
    global text
    assert text.count(old) == 1, (old[:90], text.count(old))
    text = text.replace(old, new)


swap("""   `an_amended_edit_revision_is_its_own_from_scratch_digest` pins purity (the amended revision equals the revision a store
   loaded from the same envelope derives).
""", """   `an_amended_edit_revision_is_its_own_from_scratch_digest` pins purity (the amended revision equals the revision a store
   loaded from the same envelope derives).
   (d) The history half: reading the history of a long coalesced run cost O(run) per key (the rebuilt view copied every
   printed op line of the growing row, the panel joined them all, every dirty `HistoryEntry` export cloned them). A row now
   previews its edit's LAST `HISTORY_ROW_OPERATION_PREVIEW` (8) operations and records the operation count it printed at
   (`CommandView::op_count`): a sealed row's preview is reused while its edit still has that many operations, the growing
   row is re-printed in O(preview). SDK law `a_long_coalesced_gesture_previews_its_newest_operations_only`.
""")

swap('''STORE_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"
''', '''STORE_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"
PANEL_KIT_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs"
PREVIEW_EVAL_LAWS = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs"
''')

NEW_HUNKS = r'''
PREVIEW_CONST_OLD = """        /// 📜️ This entry's edit's forward operations, printed via `OpText::print_op` — empty for
        /// cursor-motion entries and for a dangling `edit_id` (document replaced mid-session).
        pub op_lines: Vec<String>,
"""
PREVIEW_CONST_NEW = """        /// 📜️ The LAST [`HISTORY_ROW_OPERATION_PREVIEW`] forward operations of this entry's edit, printed via
        /// `OpText::print_op`, newest last — a bounded preview, so reading the history costs the same per key at the end of a
        /// long coalesced typing run as at its start; empty for cursor-motion entries and for a dangling `edit_id` (document
        /// replaced mid-session).
        pub op_lines: Vec<String>,
        /// 🔢️ Forward operations of this entry's edit when `op_lines` was printed — a sealed row's preview is reused only while
        /// its edit still has exactly that many.
        pub op_count: usize,
"""
PREVIEW_DECL_ANCHOR = """    /// 📜️ Checkpoint/alternative history summary exposed to apps — the swimlane columns, the
    /// undo/redo availability, the current checkout position, and the merged command+operation timeline.
"""
PREVIEW_DECL = """    /// 📜️ Operations a history row previews: its edit's newest ones (see `CommandView::op_lines`).
    pub const HISTORY_ROW_OPERATION_PREVIEW: usize = 8;

"""
PRINTED_OLD = """            let printed: HashMap<&str, &[String]> = previous.map_or_else(HashMap::new, |previous| previous.commands.iter().filter_map(|row| row.edit_id.as_deref().map(|edit_id| (edit_id, row.op_lines.as_slice()))).collect());"""
PRINTED_NEW = """            let printed: HashMap<&str, (usize, &[String])> = previous.map_or_else(HashMap::new, |previous| previous.commands.iter().filter_map(|row| row.edit_id.as_deref().map(|edit_id| (edit_id, (row.op_count, row.op_lines.as_slice())))).collect());"""
ROW_OLD = """                let mut op_lines = Vec::new();
                if let Some(edit) = edit {
                    let retained = entry.edit_id.as_deref().and_then(|edit_id| printed.get(edit_id).copied()).filter(|lines| lines.len() <= edit.forwards.len()).unwrap_or_default();
                    op_lines.extend_from_slice(retained);
                    for op in edit.forwards.iter().skip(retained.len()) {
                        op_lines.push(op.print_op());
                    }
                }"""
ROW_NEW = """                let mut op_lines = Vec::new();
                let mut op_count = 0;
                if let Some(edit) = edit {
                    op_count = edit.forwards.len();
                    match entry.edit_id.as_deref().and_then(|edit_id| printed.get(edit_id).copied()).filter(|(count, _)| *count == op_count) {
                        Some((_, lines)) => op_lines.extend_from_slice(lines),
                        None => op_lines.extend(edit.forwards[op_count.saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|op| op.print_op())),
                    }
                }"""
ROW_FIELD_OLD = """                    op_lines,
                    applied,
                    revertible,"""
ROW_FIELD_NEW = """                    op_lines,
                    op_count,
                    applied,
                    revertible,"""
ARM_OLD = """                                if command.op_lines.len() > edit.forwards.len() {
                                    return false;
                                }
                                for operation in edit.forwards.iter().skip(command.op_lines.len()) {
                                    command.op_lines.push(operation.print_op());
                                }
                                true"""
ARM_NEW = """                                if command.op_count > edit.forwards.len() {
                                    return false;
                                }
                                if command.op_count != edit.forwards.len() {
                                    command.op_lines = edit.forwards[edit.forwards.len().saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|operation| operation.print_op()).collect();
                                    command.op_count = edit.forwards.len();
                                }
                                true"""
HISTORY_LAW_ANCHOR = """        assert_eq!(set_label_entries.len(), 1, "a coalesced gesture must grow one entry's op_lines, not append new entries");
    }
"""
HISTORY_LAW = """
    /// 📜️ LAW (ticket 26/09/23 C12): a coalesced gesture's history row previews only its edit's newest
    /// `HISTORY_ROW_OPERATION_PREVIEW` operations — the newest one last — however long the gesture runs, so reading the history
    /// costs the same per key at the end of a long typing run as at its start.
    #[semio_framework_async_macros::async_test]
    async fn a_long_coalesced_gesture_previews_its_newest_operations_only() {
        let mut app = contract_app().await;
        let mut value = String::new();
        for key in 0..64u8 {
            value.push(char::from(b'a' + key % 26));
            app.dispatch_typed(TestCommand::SetLabel { value: value.clone() }, &meta()).await.expect("setLabel");
            let _ = app.test_history().await;
        }
        let history = app.test_history().await;
        let row = history.commands.iter().find(|entry| entry.action_id == "setLabel").expect("the gesture's row");
        assert_eq!(row.op_count, 64, "the row counts every operation of its edit");
        assert_eq!(row.op_lines.len(), HISTORY_ROW_OPERATION_PREVIEW, "the row previews a bounded tail of its operations");
        assert!(row.op_lines.last().is_some_and(|line| line.contains(&value)), "the newest operation closes the preview: {:?}", row.op_lines);
    }
"""
'''
swap('\nDAG_OLD = ', NEW_HUNKS + '\nDAG_OLD = ')

swap('''        (RETIRE_CHECK_OLD, RETIRE_CHECK_NEW),''', '''        (RETIRE_CHECK_OLD, RETIRE_CHECK_NEW),
        (PREVIEW_CONST_OLD, PREVIEW_CONST_NEW),
        (PREVIEW_DECL_ANCHOR, PREVIEW_DECL + PREVIEW_DECL_ANCHOR),
        (PRINTED_OLD, PRINTED_NEW),
        (ROW_OLD, ROW_NEW),
        (ROW_FIELD_OLD, ROW_FIELD_NEW),
        (ARM_OLD, ARM_NEW),''')
swap('''    SDK_LAWS: [(SDK_LAW_ANCHOR, SDK_LAW_ANCHOR + SDK_LAW)],''', '''    SDK_LAWS: [
        (SDK_LAW_ANCHOR, SDK_LAW_ANCHOR + SDK_LAW),
        (HISTORY_LAW_ANCHOR, HISTORY_LAW_ANCHOR + HISTORY_LAW),
        ('\\n                    op_lines: vec!["set-count value=1".into()],\\n', '\\n                    op_lines: vec!["set-count value=1".into()],\\n                    op_count: 1,\\n'),
        ("\\n                    op_lines: Vec::new(),\\n", "\\n                    op_lines: Vec::new(),\\n                    op_count: 0,\\n"),
        ('\\n                op_lines: vec![format!("register-mesh vertices=[{}]", "1.0 ".repeat(1_024))],\\n', '\\n                op_lines: vec![format!("register-mesh vertices=[{}]", "1.0 ".repeat(1_024))],\\n                op_count: 1,\\n'),
        ("\\n            op_lines: Vec::new(),\\n", "\\n            op_lines: Vec::new(),\\n            op_count: 0,\\n", "every"),
    ],
    PANEL_KIT_LAWS: [("\\n            op_lines: Vec::new(),\\n", "\\n            op_lines: Vec::new(),\\n            op_count: 0,\\n", "every")],
    PREVIEW_EVAL_LAWS: [("\\n        op_lines: Vec::new(),\\n", "\\n        op_lines: Vec::new(),\\n        op_count: 0,\\n")],''')

swap('''    for old, new in hunks:
        if new in text:
            plan.append("present")
            continue
        if text.count(old) != 1:
            problems.append((relative.rsplit("/", 2)[-2], old[:70], text.count(old)))
            continue
        text = text.replace(old, new)
        plan.append("hunk")''', '''    for old, new, *every in hunks:
        if new in text:
            plan.append("present")
            continue
        if text.count(old) != 1 and not (every and text.count(old) > 1):
            problems.append((relative.rsplit("/", 2)[-2], old[:70], text.count(old)))
            continue
        text = text.replace(old, new)
        plan.append("hunk")''')

open(PATCH, "w", encoding="utf-8").write(text)
print("amended")
