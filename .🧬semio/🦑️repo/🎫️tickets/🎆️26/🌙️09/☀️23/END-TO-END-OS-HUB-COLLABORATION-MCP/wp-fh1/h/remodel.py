"""📸️ FH1 family H — remodel: literal codes + parameters, declarations on the editor, laws assert codes."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
from texts import declare
R = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
edits = [
    ("🎮️commands/🌱️add-stream/🦀️.rs", 'camera_id => return Err(app_fault("remodeling.stream.unknown-camera")),', 'camera_id => return Err(app_fault("remodeling.stream.unknown-camera").with_parameter("camera", camera_id)),'),
    ("🦀️.rs", '''    fn unknown(action: &str) -> Fault {
        app_fault("app.command.unsupported")
    }''', '''    fn unknown(action: &str) -> Fault {
        app_fault("app.command.unsupported").with_parameter("action", action)
    }'''),
    ("🎮️commands/🌱️add-stream/🧪️tests/🔬️unit/🦀️.rs", '    assert!(fault.message.contains("remodeling.stream.unknown-camera"), "{fault:?}");', '    assert_eq!(fault.code.0, "remodeling.stream.unknown-camera", "{fault:?}");'),
    ("🎮️commands/🎞️import-frames/🧪️tests/🔬️unit/🦀️.rs", '    assert!(fault.message.contains("Run the quality check before exporting its report."), "{fault:?}");\n', ''),
]
editor = ["remodeling-frames-window-required", "remodeling-frames-window-kind-required", "remodeling-report-window-required", "remodeling-report-window-kind-required", "remodeling-model-window-required", "remodeling-model-window-kind-required", "remodeling-window-stale", "remodeling-window-view-required", "remodeling.stream.unknown-camera", "remodeling.qc-report.missing", "remodeling.qc-report.encode", "app.command.unsupported", "remodeling.retained.route", "remodeling.retained.tool-mismatch", "remodeling.retained.extent", "remodeling.reconstruction.provisional-count"]
edits.append(("🦀️.rs", '''remodeling-qc-report.ops file on the user's machine; without a report nothing happens.", "Schreibt den Qualitätsbericht der Rekonstruktion in eine heruntergeladene Datei remodeling-qc-report.ops auf dem Rechner des Nutzers; ohne Bericht geschieht nichts."))''', '''remodeling-qc-report.ops file on the user's machine; without a report the export is refused.", "Schreibt den Qualitätsbericht der Rekonstruktion in eine heruntergeladene Datei remodeling-qc-report.ops auf dem Rechner des Nutzers; ohne Bericht wird der Export abgelehnt."))'''))
edits.append(("🦀️.rs", '''ersetzt Abweichendes, anhand der Beispiel-Id."))
            .build_definition()''', '''ersetzt Abweichendes, anhand der Beispiel-Id."))
''' + declare(editor, "            ") + '''            .build_definition()'''))
apply(R, edits)
