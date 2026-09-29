"""🎥️ FH1 family H — shooting: literal codes + parameters, declarations on the editor."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
edits = [
    ("🎮️commands/🖨️export/🦀️.rs", '    Err(app_fault("shooting.export.nothing-to-export"))\n', '    Err(if all { app_fault("shooting.export.no-shots") } else { app_fault("shooting.export.nothing-to-export") })\n'),
    ("🦀️.rs", 'T::from_value(value).map_err(|_| app_fault("app.command.invalid-args"))', 'T::from_value(value).map_err(|_| app_fault("app.command.invalid-args").with_parameter("action", action))'),
    ("🦀️.rs", '_ => return Err(app_fault("app.command.unsupported")),', '_ => return Err(app_fault("app.command.unsupported").with_parameter("action", action)),'),
]
editor = ["shooting.export.nothing-to-export", "shooting.export.no-shots", "app.command.invalid-args", "app.command.unsupported", "shooting.retained.route", "shooting.retained.tool-mismatch", "shooting.retained.extent"]
edits.append(("🦀️.rs", '''            .action_destructive("importSnapshotJson")
            .build_definition()''', '''            .action_destructive("importSnapshotJson")
''' + declare(editor, "            ") + '''            .build_definition()'''))
apply(R, edits)
