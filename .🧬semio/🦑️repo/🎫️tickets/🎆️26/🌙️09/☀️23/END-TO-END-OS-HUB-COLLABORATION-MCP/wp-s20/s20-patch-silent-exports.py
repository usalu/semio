"""🔇 S20 window-3 prepared patch: exports that had nothing to write say so instead of succeeding silently.

- remodel `exportQcReport` with no QC result returned an empty success (io-matrix 27 19:5x: no file, no refusal) → refused
  `remodeling.qc-report.missing`; an encoding error no longer writes an empty file (`unwrap_or_default`) and the file is
  named and typed as the JSON it is (`remodeling-qc-report.json`, `application/json`, was `.ops` / `text/plain`).
- shooting `exportActiveShot` / `exportAllShots` with no active asset or no shot returned an empty success → refused
  `shooting.export.nothing-to-export`.

Usage: python3 s20-patch-silent-exports.py [--dry-run]   (idempotent)
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
DRY = "--dry-run" in sys.argv
REMODEL = ROOT / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧾️export-qc-report/🦀️.rs"
SHOOTING = ROOT / "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖨️export/🦀️.rs"

HUNKS = [
    (REMODEL, "remodel imports",
     "use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};\n",
     "use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};\n"),
    (REMODEL, "remodel refusal",
     '''    match &doc.snapshot.results.qc {
        Some(qc) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: "remodeling-qc-report.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(qc).unwrap_or_default(), encoding: None })),
        None => Ok(Emit::default()),
    }
''',
     '''    let qc = doc.snapshot.results.qc.as_ref().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("remodeling.qc-report.missing"), "Run the quality check before exporting its report."))?;
    let data = serde_json::to_string_pretty(qc).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("remodeling.qc-report.encode"), format!("The quality report could not be written: {error}")))?;
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: "remodeling-qc-report.json".into(), mime_type: "application/json".into(), data, encoding: None }))
'''),
    (SHOOTING, "shooting imports",
     "use semio_framework_plugin::{ArtifactView, ConfigView, DslValue, Effect, Emit, Fault, IconRenderExportItem};\n",
     "use semio_framework_plugin::{ArtifactView, ConfigView, DslValue, Effect, Emit, Fault, FaultCode, FaultOrigin, IconRenderExportItem};\n"),
    (SHOOTING, "shooting refusal",
     '''        if !items.is_empty() {
            return Ok(Emit::effect(Effect::IconRenderExport { items }));
        }
    }
    Ok(Emit::default())
}
''',
     '''        if !items.is_empty() {
            return Ok(Emit::effect(Effect::IconRenderExport { items }));
        }
    }
    Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.export.nothing-to-export"), if all { "Choose an asset with at least one shot before exporting." } else { "Choose an asset and a shot before exporting." }))
}
'''),
]


def main() -> None:
    texts = {path: path.read_text() for path in {hunk[0] for hunk in HUNKS}}
    notes = []
    for path, name, old, new in HUNKS:
        text = texts[path]
        if text.count(old) == 1 and new not in text:
            texts[path] = text.replace(old, new)
            notes.append(f"apply     {name}")
        elif new in text:
            notes.append(f"applied   {name}")
        else:
            notes.append(f"CONFLICT  {name}: anchor found {text.count(old)}×")
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    if not DRY:
        for path, text in texts.items():
            path.write_text(text)
    print("dry-run" if DRY else "applied")


if __name__ == "__main__":
    main()
