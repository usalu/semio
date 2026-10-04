"""🧮️ S3-W2A W2A-2 tests: the laws that pinned config-lane history rows invert to L4 (design §20.13), and every CommandView
literal loses its config field (plugin crate, wgpu shell test, generation3d preview-eval test)."""
import pathlib
import re

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
CONTRACT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
LITERALS = [
    CONTRACT,
    ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs",
    ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs",
    ROOT / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs",
]

def between(text, start, end):
    if text.count(start) != 1:
        raise SystemExit(f"start anchor count {text.count(start)}: {start[:100]!r}")
    head = text.index(start)
    tail = text.index(end, head)
    return head, tail + len(end)

contract = CONTRACT.read_text(encoding="utf-8")
replacements = [
    ("    /// ⏪️ The MULTI-lane half of the same law, and the regression guard S10 §4.2's `||` needed.\n",
     "        assert!(!applied(false, false, false, false), \"a row applied nowhere is not applied\");\n    }\n",
     """    /// ⏪️ A row follows its parent document lane, else its children (design §20.13, L4: a config-lane edit is never part
    /// of a row). 💠️lowpoly `addPrimitive` and 🎥️shooting `addShot` once read applied for ever after an undo because a
    /// config lane answered for them (ticket 26/09/18 S11 §3.4b, S12 §4); no lane but the document and its children can.
    #[test]
    fn a_row_follows_its_parent_document_lane_and_else_its_children() {
        use crate::app::history_row_applied as applied;
        assert!(!applied(true, false, true), "a row that published a parent edit is UNDONE once that edit is retracted, however live its child edit is");
        assert!(!applied(true, false, false), "a parent-lane row with nothing applied anywhere is undone");
        assert!(applied(true, true, false), "a parent-lane row whose document edit is applied is applied");
        assert!(applied(true, true, true), "a multi-lane row is applied while its parent edit is");
        assert!(applied(false, false, true), "a CHILD-only row answers with its children — 🌊️flow's addWidget and 🎬️sequence's addStep");
        assert!(!applied(false, false, false), "a row applied nowhere is not applied");
    }
"""),
    ("    /// ✅️ A row's `applied` must ask every lane the row can publish into, not only the parent\n",
     "        assert!(select.revertible, \"and it stays revertible, the clause that already consulted all three lanes\");\n        close_reserved_app(&mut app);\n    }\n",
     """    /// ⚖️ LAW (design §20.13, L4): a config-lane edit is never a history row and undo never steps over it — the history
    /// lists the document row alone (every row, no `edit_id` filter), and `undo` retracts the newest DOCUMENT edit while the
    /// config the select wrote stays.
    #[semio_framework_async_macros::async_test]
    async fn a_config_lane_edit_is_never_a_history_row_and_undo_never_steps_over_it() {
        let mut app = contract_app().await;
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta()).await.expect("select");
        assert_eq!(app.test_config().await.selected, Some("node-1".to_string()), "the select wrote the config lane");
        let history = app.test_history().await;
        assert_eq!(history.commands.iter().map(|entry| entry.action_id.as_str()).collect::<Vec<_>>(), vec!["increment"], "only the document row is listed");
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.count, 0, "undo retracts the document edit, never the config one");
        assert_eq!(app.test_config().await.selected, Some("node-1".to_string()), "undo never steps over the config lane");
        close_reserved_app(&mut app);
    }
"""),
    ("    /// 🔄️ Keep the two selects from folding into one row by dispatching an unrelated Mutation between them.\n",
     "        assert_eq!(after.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));\n    }\n",
     """    /// ⏪️ Revert-to-command walks the DOCUMENT lane alone (design §20.13, L4): "leave the TARGET row applied, undo everything
    /// after it" — reverting to the first increment undoes the second and leaves the config the interleaved selects wrote.
    #[semio_framework_async_macros::async_test]
    async fn revert_to_command_walks_the_document_lane_and_leaves_config_alone() {
        let mut app = contract_app().await;
        app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta()).await.expect("select a");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta()).await.expect("select b");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment again");
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 2, "two document rows and no config row");
        let first = history.commands.iter().min_by_key(|entry| entry.seq).expect("the first increment row");
        assert!(first.revertible, "a document row is revertible");
        reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": first.seq })))).await;
        assert_eq!(app.test_snapshot().await.count, 1, "reverting to the first increment undoes the second");
        assert_eq!(app.test_config().await.selected, Some("b".to_string()), "the config lane is never walked");
        let after = app.test_history().await;
        assert_eq!(after.commands.len(), 3, "the revert appends one History-kind row");
        assert_eq!(after.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
    }
"""),
    ("    async fn an_op_less_view_action_is_logged_with_edit_id_none_and_count_one() {\n",
     "        assert_eq!(entry.count, 1);\n    }\n",
     """    async fn a_config_only_view_action_is_not_a_history_row() {
        let mut app = contract_app().await;
        app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta()).await.expect("select");
        assert!(app.test_history().await.commands.is_empty(), "a config-only View is never a row (design §20.13, L4)");
        assert_eq!(app.test_config().await.selected, Some("node-1".to_string()), "while the config lane holds its edit");
    }
"""),
    ("    async fn consecutive_identical_view_dispatches_are_distinct_history_entries() {\n",
     "        assert!(history.commands.iter().all(|entry| entry.count == 1));\n    }\n",
     """    async fn consecutive_config_only_view_dispatches_add_no_history_rows() {
        let mut app = contract_app().await;
        for id in ["node-1", "node-2", "node-3"] {
            app.dispatch_typed(TestCommand::Select { id: Some(id.into()) }, &meta()).await.expect("select");
        }
        assert!(app.test_history().await.commands.is_empty(), "three config-only Views add no row");
        assert_eq!(app.test_config().await.selected, Some("node-3".to_string()));
    }
"""),
    ("    async fn view_dispatches_remain_distinct_across_interleaved_entries() {\n",
     "        assert_eq!(select_counts, vec![1, 1, 1]);\n    }\n",
     """    async fn interleaved_config_only_views_leave_only_the_document_row() {
        let mut app = contract_app().await;
        app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta()).await.expect("select a");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta()).await.expect("select b");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("c".into()) }, &meta()).await.expect("select c");
        let history = app.test_history().await;
        assert_eq!(history.commands.iter().map(|entry| (entry.action_id.as_str(), entry.count)).collect::<Vec<_>>(), vec![("increment", 1)]);
    }
"""),
]
for start, end, new in replacements:
    head, tail = between(contract, start, end)
    if tail - head > 6000:
        raise SystemExit(f"span too long for {start[:60]!r}: {tail - head}")
    contract = contract[:head] + new + contract[tail:]
simple = [
    ("    /// And the command log recorded the child's edit id under the `config_edit_ids` precedent.\n",
     "    /// And the command log recorded the child's edit id in `child_edit_ids`.\n"),
    ("        app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect(\"render before\");\n        app.dispatch_typed(TestCommand::Select { id: Some(\"x\".into()) }, &meta()).await.expect(\"select\");\n",
     "        app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect(\"render before\");\n        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ \"commandId\": \"os.setThemeId\", \"label\": \"Set Theme\" })))).await;\n"),
]
for old, new in simple:
    if contract.count(old) != 1:
        raise SystemExit(f"anchor count {contract.count(old)}: {old[:100]!r}")
    contract = contract.replace(old, new)
CONTRACT.write_text(contract, encoding="utf-8")

removed = 0
for path in LITERALS:
    text = path.read_text(encoding="utf-8")
    new, count = re.subn(r"\n[ \t]*config_edit_id: None,(?=\n[ \t]*child_edit_ids:)", "", text)
    removed += count
    path.write_text(new, encoding="utf-8")
print(f"L4 tests: {len(replacements)} laws rewritten, {len(simple)} edits, {removed} literal fields removed")
