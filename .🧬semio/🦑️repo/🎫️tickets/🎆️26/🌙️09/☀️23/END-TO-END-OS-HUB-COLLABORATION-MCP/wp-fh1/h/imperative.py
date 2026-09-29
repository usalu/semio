"""📜️ FH1 family H — imperative: literal codes + parameters, declarations on the editor (+ extension codes) and viewer."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/"
edits = [
    ("✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs", '.map_err(|error| Fault::from(format!("imperative-example-unparsable: {error}")))?', '.map_err(|_| semio_framework_plugin::app_fault("imperative-example-unparsable"))?'),
    ("✏️editor/🦀️.rs", 'other => Err(app_fault("imperative.unhandled-action")),', 'other => Err(app_fault("imperative.unhandled-action").with_parameter("action", other)),'),
]
editor = ["imperative-example-unparsable", "imperative.unhandled-action", "imperative-retained-command-tool-mismatch-or-capacity", "procedure.child-projection", "extension.evaluate"]
edits.append(("✏️editor/🦀️.rs", '''            .action_destructive("setActiveExample")
            .build_definition()''', '''            .action_destructive("setActiveExample")
''' + declare(editor, "            ") + '''            .build_definition()'''))
edits.append(("👁️viewer/🦀️.rs", '''        .window_kind_def(script::definition())
        .default_layout(view::layout())
        .build_definition()''', '''        .window_kind_def(script::definition())
        .default_layout(view::layout())
''' + declare(["procedure.child-projection"], "        ", qualified=True) + '''        .build_definition()'''))
apply(R, edits)
