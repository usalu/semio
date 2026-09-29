"""🎪️ FH1 family H — demonstrator playground: literal codes + parameters, declarations on the editor."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
edits = [
    ("🎮️commands/🎨️set-active-example/🦀️.rs", '.map_err(|error| Fault::from(error.to_string()))?.schema', '.map_err(|_| semio_framework_plugin::app_fault("playground.example.unparsable"))?.schema'),
    ("🦀️.rs", '''            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \\
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),''', '''            other => Err(app_fault("app.command.unsupported").with_parameter("action", other)),'''),
]
editor = ["playground.example.unparsable", "playground-command-tool-mismatch", "playground-command-payload-too-large", "app.command.unsupported"]
edits.append(("🦀️.rs", '''        .action_destructive("changeSchema")
        .build_definition()''', '''        .action_destructive("changeSchema")
''' + declare(editor, "        ") + '''        .build_definition()'''))
apply(R, edits)
