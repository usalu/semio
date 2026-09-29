"""🕸️ FH1 family H — dag: literal codes + parameters, declarations on the editor and viewer."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/"
T = "✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"
edits = [
    (T, '.map_err(|error| Fault::from(format!("dag.layout-run.layered-load: {error}")))?', '.map_err(|_| app_fault("dag.layout-run.layered-load"))?'),
    (T, '.map_err(|error| Fault::from(format!("dag.layout-run.layered-layout: {error}")))?', '.map_err(|_| app_fault("dag.layout-run.layered-layout"))?'),
    (T, '.map_err(|error| Fault::from(format!("dag.layout-run: {error:?}")))?', '.map_err(|_| app_fault("dag.layout-run"))?'),
    ("✏️editor/🦀️.rs", 'other => Err(app_fault("dag.unhandled-action")),', 'other => Err(app_fault("dag.unhandled-action").with_parameter("action", other)),'),
]
editor = ["dag.layout-run.layered-load", "dag.layout-run.layered-layout", "dag.layout-run.layered-fixture", "dag.layout-run", "dag-retained-document-route-mismatch", "dag-retained-config-route-mismatch", "dag-retained-command-tool-mismatch", "dag.unhandled-action", "dag.child-projection"]
edits.append(("✏️editor/🦀️.rs", '''            .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("setActiveExample")
            .build_definition()''', '''            .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("setActiveExample")
''' + declare(editor, "            ") + '''            .build_definition()'''))
edits.append(("👁️viewer/🦀️.rs", '''        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()''', '''        .window_kind_def(main::definition())
        .default_layout(view::layout())
''' + declare(["dag.child-projection"], "        ", qualified=True) + '''        .build_definition()'''))
apply(R, edits)
