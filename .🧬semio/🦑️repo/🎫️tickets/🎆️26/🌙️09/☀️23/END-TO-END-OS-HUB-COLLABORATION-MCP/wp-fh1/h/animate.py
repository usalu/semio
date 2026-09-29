"""🎞️ FH1 family H — animate: literal codes (the video export error answers its own refusal), declarations."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/"
X = "✏️editor/🎮️commands/🎥️export-video-from-deck/🦀️.rs"
E = "✏️editor/🦀️.rs"
edits = [
    ("✏️editor/⚙️engine/🦀️.rs", '''    /// 🔤️ The fault code a command answers this error with.
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoSceneHashes => "animate.video.export.no-scenes",
            Self::SourceKind { .. } => "animate.video.export.source-kind",
            Self::Program(_) => "animate.video.export.program",
        }
    }''', '''    /// 🧯️ The refusal a command answers this error with.
    pub fn fault(&self) -> semio_framework::Fault {
        match self {
            Self::NoSceneHashes => semio_framework::app_fault("animate.video.export.no-scenes"),
            Self::SourceKind { .. } => semio_framework::app_fault("animate.video.export.source-kind"),
            Self::Program(_) => semio_framework::app_fault("animate.video.export.program"),
        }
    }'''),
    (X, '''fn refused(error: PresentationVideoExportError) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(error.code()), error.to_string())
}

fn scene_json_fault(detail: String) -> Fault {
    app_fault("animate.video.export.scene-json")
}
''', '''fn refused(error: PresentationVideoExportError) -> Fault {
    error.fault()
}
'''),
    (X, "use semio_framework_plugin::{app_fault, ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};", "use semio_framework_plugin::{app_fault, ArtifactView, ConfigView, Effect, Emit, Fault};"),
    (X, '''.map_err(|error| scene_json_fault(format!("export-video-from-deck scene is not JSON: {error}")))?;''', '''.map_err(|_| app_fault("animate.video.export.scene-json"))?;'''),
    (X, '''.map_err(|error| scene_json_fault(format!("export-video-from-deck scene is not a presentation scene: {error}")))?;''', '''.map_err(|_| app_fault("animate.video.export.scene-json"))?;'''),
    (E, 'return Err(Fault::from("presentation import-media admits a decoded media value, never a wire payload"));', 'return Err(app_fault("animate.import-media.wire-payload"));'),
    (E, 'return Err(Fault::from("presentation import-media requires media input"));', 'return Err(app_fault("animate.import-media.input-missing"));'),
    (E, '''            let value = args.and_then(|value| value.get(key)).cloned().ok_or_else(|| Fault::from(format!("presentation {action} requires a '{key}' block")))?;
            dsl::from_dsl_value(value).map_err(|error| Fault::from(format!("invalid presentation {action} '{key}': {error}")))''', '''            let value = args.and_then(|value| value.get(key)).cloned().ok_or_else(|| app_fault("animate.command.argument-missing").with_parameter("action", action).with_parameter("key", key))?;
            dsl::from_dsl_value(value).map_err(|_| app_fault("animate.command.argument-invalid").with_parameter("action", action).with_parameter("key", key))'''),
    (E, '''.map_err(|error| Fault::from(format!("invalid presentation addTile 'crop': {error}")))?''', '''.map_err(|_| app_fault("animate.command.argument-invalid").with_parameter("action", "addTile").with_parameter("key", "crop"))?'''),
    (E, 'other => Err(Fault::from(format!("presentation: unhandled action id {other}"))),', 'other => Err(app_fault("app.command.unsupported").with_parameter("action", other)),'),
]
editor = ["animate.video.export.no-scenes", "animate.video.export.source-kind", "animate.video.export.program", "animate.video.export.scene-json", "animate.video.export.program-too-large", "animate-presentation-retained-route-mismatch", "animate-presentation-command-tool-mismatch", "animate-presentation-command-payload-too-large", "animate.child-projection", "animate.import-media.wire-payload", "animate.import-media.input-missing", "animate.command.argument-missing", "animate.command.argument-invalid", "app.command.unsupported"]
edits.append((E, '''            .action_destructive("copyPrompt")
            .build_definition()''', '''            .action_destructive("copyPrompt")
''' + declare(editor, "            ") + '''            .build_definition()'''))
edits.append(("👁️viewer/🦀️.rs", '''        .window_kind_def(tile_editor::definition())
        .default_layout(view::layout())
        .build_definition()''', '''        .window_kind_def(tile_editor::definition())
        .default_layout(view::layout())
''' + declare(["animate.child-projection"], "        ", qualified=True) + '''        .build_definition()'''))
apply(R, edits)
