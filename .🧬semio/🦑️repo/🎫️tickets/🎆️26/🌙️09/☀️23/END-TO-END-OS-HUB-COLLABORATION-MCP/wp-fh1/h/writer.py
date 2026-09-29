"""✒️ FH1 family H — writer: literal codes + parameters, declarations on the editor and the viewer."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/"
C = "✏️editor/🎮️commands/"
stub = "Err(semio_framework_plugin::app_fault(\"writer.main-window-required\"))"
edits = [
    (C + "⚙️set-font-px/🦀️.rs", 'Err(Fault::from("writer window settings require the retained exact-window reducer"))', stub),
    (C + "📏️set-line-height/🦀️.rs", 'Err(Fault::from("writer window settings require the retained exact-window reducer"))', stub),
    (C + "📐️set-tab-size/🦀️.rs", 'Err(Fault::from("writer window settings require the retained exact-window reducer"))', stub),
    (C + "🔢️toggle-line-numbers/🦀️.rs", 'Err(Fault::from("writer window settings require the retained exact-window reducer"))', stub),
    (C + "✍️commit-rename/🦀️.rs", 'Err(Fault::from("writer rename requires the retained exact-window selection"))', stub),
    (C + "🎥️set-camera/🦀️.rs", 'Err(Fault::from("writer camera changes require the retained exact-window reducer"))', stub),
    (C + "💬️engagement-input/🦀️.rs", 'Err(Fault::from("writer engagement draft requires the retained exact-window reducer"))', stub),
    (C + "📤️engagement-submit/🦀️.rs", 'Err(Fault::from("Writer engagement submission requires its retained concrete-window operation owner"))', stub),
    (C + "🔍️lint-document/🦀️.rs", 'Err(Fault::from("writer lint generation requires the retained exact-window reducer"))', stub),
    (C + "🗂️set-editor-selection/🦀️.rs", 'Err(Fault::from("writer editor selection requires the retained exact-window reducer"))', stub),
    ("✏️editor/🦀️.rs", '.ok_or_else(|| Fault::from("writer setCamera requires a camera"))?', '.ok_or_else(|| app_fault("writer.camera.missing"))?'),
    ("✏️editor/🦀️.rs", '.map_err(|error| Fault::from(format!("invalid writer setCamera camera: {error}")))?', '.map_err(|_| app_fault("writer.camera.invalid"))?'),
    ("✏️editor/🦀️.rs", '.map_err(|_| Fault::from("writer textSplice start exceeds u32"))?', '.map_err(|_| app_fault("writer.text-splice.start-range"))?'),
    ("✏️editor/🦀️.rs", '''other => Err(Fault::from(format!("writer setEditorSetting has no setting '{other}' (fontPx/lineHeight/tabSize)"))),''', '''other => Err(app_fault("writer.editor-setting.unknown").with_parameter("setting", other)),'''),
    ("✏️editor/🦀️.rs", 'other => Err(Fault::from(format!("writer: unhandled action id {other}"))),', 'other => Err(app_fault("app.command.unsupported").with_parameter("action", other)),'),
    ("🚪️io/🧬️mutations/💾️binary/🦀️.rs", 'Err(error) => Err(semio_framework::app_fault("artifact-store.initializer-close")),', 'Err(_) => Err(semio_framework::app_fault("artifact-store.initializer-close")),'),
]
editor = ["writer.main-window-required", "writer.child-projection", "writer.camera.missing", "writer.camera.invalid", "writer.text-splice.start-range", "writer.editor-setting.unknown", "app.command.unsupported", "writer-command-tool-mismatch", "artifact-store.initializer-close"]
viewer = ["writer.child-projection", "artifact-store.initializer-close"]
edits.append(("✏️editor/🦀️.rs", "            .action_destructive(\"openDocument\")\n            .build_definition()", "            .action_destructive(\"openDocument\")\n" + declare(editor, "            ") + "            .build_definition()"))
edits.append(("👁️viewer/🦀️.rs", ".default_layout(view::layout()).build_definition()", ".default_layout(view::layout())\n" + declare(viewer, "        ", qualified=True) + "        .build_definition()"))
apply(R, edits, dry="--dry-run" in sys.argv)
