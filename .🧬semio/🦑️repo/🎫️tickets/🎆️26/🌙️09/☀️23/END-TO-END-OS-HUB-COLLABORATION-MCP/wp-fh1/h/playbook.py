"""📖️ FH1 family H — playbook + its procedural module app: literal codes + parameters, declarations."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/📖️playbook/"
E = "🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
M = "🧩️extensions/🌀️procedural/🦀️.rs"
edits = [
    (E + "🎮️commands/🧬️set-active-example/🦀️.rs", '.map_err(|error| Fault::from(format!("playbook-example-unparsable: {error}")))?', '.map_err(|_| semio_framework_plugin::app_fault("playbook-example-unparsable"))?'),
    (E + "🦀️.rs", 'other => Err(app_fault("playbook.unhandled-action")),', 'other => Err(app_fault("playbook.unhandled-action").with_parameter("action", other)),'),
    (M, '''other => Err(Fault::from(format!("action '{other}' is not supported by {MODULE_APP_ID}"))),''', '''other => Err(app_fault("app.command.unsupported").with_parameter("action", other)),'''),
]
editor = ["playbook-example-unparsable", "playbook.unhandled-action", "playbook-retained-command-tool-mismatch-or-capacity"]
module = ["playbook.module.procedural.tool-mismatch", "app.command.unsupported"]
edits.append((E + "🦀️.rs", '''        .action_destructive("setActiveExample")
        .build_definition()''', '''        .action_destructive("setActiveExample")
''' + declare(editor, "        ") + '''        .build_definition()'''))
edits.append((M, '''            .action_interactive_job(ACTION_IMPORT_SOLID, InteractiveJobClassification::Migrated).await,''', '''            .action_interactive_job(ACTION_IMPORT_SOLID, InteractiveJobClassification::Migrated).await
''' + declare(module, "            ", awaited=True).rstrip("\n") + ",\n"))
apply(R, edits)
