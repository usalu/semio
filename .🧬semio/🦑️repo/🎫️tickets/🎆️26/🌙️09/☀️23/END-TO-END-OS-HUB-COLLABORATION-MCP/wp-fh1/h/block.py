"""🧱️ FH1 family H — block 2d/3d/5d: literal codes + parameters, declarations on each editor."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/"
old_other = '''            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \\
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),'''
new_other = '''            other => Err(app_fault("app.command.unsupported").with_parameter("action", other)),'''
edits = []
for artifact, dims in [("◻️2d", "2d"), ("🖐️5d", "5d")]:
    f = f"{artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
    edits.append((f, old_other, new_other))
    edits.append((f, '''            .action_destructive("edit")
            .build_definition()''', '''            .action_destructive("edit")
''' + declare([f"block{dims}-retained-command-tool-mismatch", "app.command.unsupported"], "            ") + '''            .build_definition()'''))
f3 = "🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
edits.append((f3, 'other => Err(app_fault("block3d.unhandled-action")),', 'other => Err(app_fault("block3d.unhandled-action").with_parameter("action", other)),'))
edits.append((f3, '''        .action_destructive("edit")
        .build_definition()''', '''        .action_destructive("edit")
''' + declare(["block3d-retained-command-tool-mismatch", "block3d.unhandled-action", "block3d-window-preview-work-repeated", "block3d-window-preview-requires-view", "block3d-window-preview-kind-mismatch", "block3d-window-preview-requires-concrete-window", "block3d-window-preview-route-mismatch"], "        ") + '''        .build_definition()'''))
apply(R, edits)
