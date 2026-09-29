"""🪵️ FH1 family H — sourcing curation: literal codes + parameters, declarations on the editor and viewer."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
A = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/"
R = "🏅️standards/🔖️1/🪆️subsets/✳️any/"
C = R + "✏️editor/🎮️commands/"
edits = [
    (C + "🎬️set-active-example/🦀️.rs", '.ok_or_else(|| app_fault("sourcing.example.unknown"))?', '.ok_or_else(|| app_fault("sourcing.example.unknown").with_parameter("example", id))?'),
    (C + "🎬️set-active-example/🦀️.rs", '.map_err(|error| Fault::from(error.to_string()))?', '.map_err(|_| app_fault("sourcing.example.unparsable"))?'),
    (C + "🗿️set-artifact-json/🦀️.rs", 'return Err(Fault::from("sourcing.invalid-payload: document JSON exceeds byte, depth, string, or cardinality limit"));', 'return Err(app_fault("sourcing.document-json.limit"));'),
    (C + "🗿️set-artifact-json/🦀️.rs", 'Err(_) => Err(Fault::from("sourcing.invalid-payload: document schema mismatch")),', 'Err(_) => Err(app_fault("sourcing.document-json.schema")),'),
    (C + "🗿️set-artifact-json/🦀️.rs", 'use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};', 'use semio_framework_plugin::{app_fault, ArtifactView, ConfigView, Emit, Fault};'),
    (R + "✏️editor/👥️presence/🦀️.rs", '.map_err(semio_framework_plugin::Fault::from)', '.map_err(|_| semio_framework_plugin::app_fault("sourcing.presence.close"))'),
    (R + "✏️editor/👥️presence/🦀️.rs", 'Err((reason, terminal)) => { self.terminal = Some(terminal); return Err(semio_framework_plugin::Fault::from(reason)); }', 'Err((_, terminal)) => { self.terminal = Some(terminal); return Err(semio_framework_plugin::app_fault("sourcing.presence.close")); }'),
    (R + "✏️editor/🦀️.rs", 'other => return Err(app_fault("app.command.unsupported")),', 'other => return Err(app_fault("app.command.unsupported").with_parameter("action", other)),'),
    ("🦀️.rs", '.map_err(|error| semio_framework_plugin::Fault::from(format!("sourcing curation child projection failed: {error}")))', '.map_err(|_| semio_framework_plugin::app_fault("sourcing.curation.child-projection"))'),
]
editor = ["sourcing-grid-window-required", "sourcing-grid-window-stale", "sourcing-grid-window-kind-required", "sourcing.example.unknown", "sourcing.example.unparsable", "sourcing.document-json.limit", "sourcing.document-json.schema", "sourcing.presence.close", "app.command.unsupported", "sourcing-curation-retained-route-mismatch", "sourcing-curation-command-tool-mismatch", "sourcing.curation.child-projection"]
edits.append((R + "✏️editor/🦀️.rs", '''only that window's view changes.", ''', None))
apply(A, edits[:-1])
import re
O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"
f = O + A + R + "✏️editor/🦀️.rs"
text = open(f).read()
anchor = re.search(r'(            \.action_describe\("setGridInstanceDisplay", [^\n]*\n)(            \.build_definition\(\)\n)', text)
assert anchor, "anchor"
text = text[:anchor.end(1)] + declare(editor, "            ") + text[anchor.end(1):]
open(f, "w").write(text)
apply(A, [(R + "👁️viewer/🦀️.rs", '''        .window_kind_def(pool::definition())
        .default_layout(view::layout())
        .build_definition()''', '''        .window_kind_def(pool::definition())
        .default_layout(view::layout())
''' + declare(["sourcing.curation.child-projection"], "        ", qualified=True) + '''        .build_definition()''')])
