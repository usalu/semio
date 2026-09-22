#!/usr/bin/env python3
"""🧱️ Bridges the three editable window-KIT verbs in every stdio editor that composes one.

`TextWindowKit`/`TableWindowKit`/`TreeWindowKit` mint `replace-text`/`set-cell`/`set-node` as
`ActionKind::Mutation` palette rows on every composing app, stamped `Migrated`, so the Actions rail
renders them and a human can press them — but each stdio editor's `command_from_action` answered
`stdio.<kind>.unhandled-action` for exactly those ids, so pressing the rail row could only ever be
refused. The framework's own bridge-conformance check skips the three ids, which is why no test saw
it. Idempotent and anchored: it inserts ONE match arm before the `other =>` arm of each editor's
`*_command_from_action`, and does nothing when the arm is already there.
"""
import io
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")

# (relative editor file, action id, arm body)
CASES = [
    ("📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs", "set-cell",
     'Ok(CsvEditorCommand::SetCell { row: semio_s_artifact_stdio_contract::window_kit_index_argument(args, &["row"], 0), column: semio_s_artifact_stdio_contract::window_kit_index_argument(args, &["column"], 0), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") })'),
    ("📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🦀️.rs", "set-cell",
     'Ok(TsvEditorCommand::SetCell { row: semio_s_artifact_stdio_contract::window_kit_index_argument(args, &["row"], 0), column: semio_s_artifact_stdio_contract::window_kit_index_argument(args, &["column"], 0), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") })'),
    ("🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs", "replace-text",
     'Ok(TxtEditorCommand::ReplaceText { text: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["text"], "") })'),
    ("📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs", "replace-text",
     'Ok(MdEditCommand::ReplaceText { text: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["text"], "") })'),
    ("🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🦀️.rs", "replace-text",
     'Ok(HtmlEditCommand::ReplaceText { text: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["text"], "") })'),
    ("🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "set-node",
     'Ok(JsonAnyEditorCommand::SetNode { node_id: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["nodeId", "node_id", "id"], ""), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") })'),
    ("🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs", "set-node",
     'Ok(JsonIJsonIJsonEditorCommand::SetNode { node_id: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["nodeId", "node_id", "id"], ""), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") })'),
    ("📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "set-node",
     'Ok(XmlAnyEditorCommand::SetNode { node_id: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["nodeId", "node_id", "id"], ""), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") })'),
    ("📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/✏️editor/🦀️.rs", "set-node",
     'Ok(XmlValidEditorCommand::SetNode { node_id: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["nodeId", "node_id", "id"], ""), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") })'),
]

ANCHOR = "        other => Err(Fault::new(\n"

changed = 0
for relative, action_id, body in CASES:
    path = ROOT / relative
    source = io.open(path, encoding="utf-8").read()
    arm = f'        "{action_id}" => {body},\n'
    if arm in source:
        print(f"skip (already bridged) {relative}")
        continue
    if source.count(ANCHOR) != 1:
        print(f"ANCHOR {source.count(ANCHOR)}x in {relative}", file=sys.stderr)
        sys.exit(1)
    io.open(path, "w", encoding="utf-8").write(source.replace(ANCHOR, arm + ANCHOR, 1))
    changed += 1
    print(f"bridged {action_id} in {relative}")
print(f"changed {changed} file(s)")
