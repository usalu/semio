"""📣️ F9 (audit-s4-tools): moves the user-reachable wfc refusals from anonymous `Fault::from("wfc-…")` (code `app.message`) onto named
`wfc.<variant>.<area>.<name>` codes, and gives wfc ×5, remodel and process3d an en/de `fault_notices()` table on their
`ArtifactEditor` impl (design §20.12). Anchor-counted; refuses a file changed during the run.
Usage: python3 🧪️s4-strokes-fault-notices.py"""
import pathlib
import re
import sys

PLUGINS = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"

RENAMES = {
    "🀄️wfc/🗿️artifacts/🖼️bitmap": {
        "wfc-bitmap-stroke-phase-invalid": "wfc.bitmap.stroke.phase-invalid",
        "wfc-bitmap-stroke-points-invalid": "wfc.bitmap.stroke.points-invalid",
        "wfc-bitmap-stroke-points-required": "wfc.bitmap.stroke.points-required",
        "wfc-bitmap-stroke-stream-requires-window-transient": "wfc.bitmap.stroke.window-required",
        "wfc-bitmap-unknown-palette-color": "wfc.bitmap.palette.unknown-color",
        "wfc-bitmap-view-state-required": "wfc.bitmap.window.view-required",
        "wfc-bitmap-window-stale": "wfc.bitmap.window.stale",
        "wfc-bitmap-input-window-required": "wfc.bitmap.input.window-required",
        "wfc-bitmap-input-window-kind-required": "wfc.bitmap.input.window-kind-required",
        "wfc-bitmap-output-window-required": "wfc.bitmap.output.window-required",
        "wfc-bitmap-output-window-kind-required": "wfc.bitmap.output.window-kind-required",
        "wfc-bitmap-pin-solution-contradiction": "wfc.bitmap.solution.contradiction",
        "wfc-bitmap-pin-solution-pixels-not-base64": "wfc.bitmap.solution.pixels-invalid",
    },
    "🀄️wfc/🗿️artifacts/🔲️grid2d": {
        "wfc-grid2d-window-required": "wfc.grid2d.window.required",
        "wfc-grid2d-window-kind-required": "wfc.grid2d.window.kind-required",
        "wfc-grid2d-no-tile-to-pin": "wfc.grid2d.tile.none-armed",
    },
    "🀄️wfc/🗿️artifacts/🧱️grid3d": {
        "wfc-grid3d-window-required": "wfc.grid3d.window.required",
        "wfc-grid3d-window-kind-required": "wfc.grid3d.window.kind-required",
        "wfc.grid3d.tile.no-armed-tile": "wfc.grid3d.tile.none-armed",
    },
}

NOTICES = {
    ("🀄️wfc/🗿️artifacts/🖼️bitmap", "impl ArtifactEditor for BitmapEditor {"): [
        ("wfc.bitmap.stroke.phase-invalid", "This stroke step is not known.", "Dieser Strichschritt ist unbekannt."),
        ("wfc.bitmap.stroke.points-invalid", "The stroke's cells are not valid.", "Die Zellen des Strichs sind ungültig."),
        ("wfc.bitmap.stroke.points-required", "A stroke needs at least one cell.", "Ein Strich braucht mindestens eine Zelle."),
        ("wfc.bitmap.stroke.window-required", "A streamed stroke needs an open input window.", "Ein gestreamter Strich braucht ein geöffnetes Eingabefenster."),
        ("wfc.bitmap.palette.unknown-color", "This colour is not in the palette.", "Diese Farbe ist nicht in der Palette."),
        ("wfc.bitmap.window.view-required", "This action needs an open window.", "Diese Aktion braucht ein geöffnetes Fenster."),
        ("wfc.bitmap.window.stale", "The window of this action is no longer open.", "Das Fenster dieser Aktion ist nicht mehr geöffnet."),
        ("wfc.bitmap.input.window-required", "Focus the input window for this action.", "Für diese Aktion das Eingabefenster fokussieren."),
        ("wfc.bitmap.input.window-kind-required", "This action belongs to the input window.", "Diese Aktion gehört zum Eingabefenster."),
        ("wfc.bitmap.output.window-required", "Focus the output window for this action.", "Für diese Aktion das Ausgabefenster fokussieren."),
        ("wfc.bitmap.output.window-kind-required", "This action belongs to the output window.", "Diese Aktion gehört zum Ausgabefenster."),
        ("wfc.bitmap.solution.contradiction", "A contradicting solve cannot be pinned.", "Eine widersprüchliche Lösung lässt sich nicht fixieren."),
        ("wfc.bitmap.solution.pixels-invalid", "The solved pixels cannot be read.", "Die gelösten Pixel lassen sich nicht lesen."),
    ],
    ("🀄️wfc/🗿️artifacts/🔲️grid2d", "impl ArtifactEditor for Grid2dEditor {"): [
        ("wfc.grid2d.window.required", "This action needs an open grid window.", "Diese Aktion braucht ein geöffnetes Rasterfenster."),
        ("wfc.grid2d.window.kind-required", "This action is not available in this window.", "Diese Aktion ist in diesem Fenster nicht verfügbar."),
        ("wfc.grid2d.tile.none-armed", "Arm a tile before pinning a cell.", "Vor dem Fixieren einer Zelle eine Kachel wählen."),
    ],
    ("🀄️wfc/🗿️artifacts/🧱️grid3d", "impl ArtifactEditor for Grid3dEditor {"): [
        ("wfc.grid3d.window.required", "This action needs an open grid window.", "Diese Aktion braucht ein geöffnetes Rasterfenster."),
        ("wfc.grid3d.window.kind-required", "This action is not available in this window.", "Diese Aktion ist in diesem Fenster nicht verfügbar."),
        ("wfc.grid3d.tile.none-armed", "Arm a tile before pinning a cell.", "Vor dem Fixieren einer Zelle eine Kachel wählen."),
    ],
    ("🀄️wfc/🗿️artifacts/◻️2d", "impl ArtifactEditor for Wfc2dEditor {"): [
        ("wfc2d.tile.unknown-tile", "This tile does not exist.", "Diese Kachel existiert nicht."),
        ("wfc2d.tile.unknown-pin", "This pin does not exist.", "Diese Fixierung existiert nicht."),
        ("wfc2d.slot.unknown-slot", "This slot does not exist.", "Dieser Steckplatz existiert nicht."),
        ("wfc2d.rule.unknown-rule", "This rule does not exist.", "Diese Regel existiert nicht."),
        ("wfc2d.edge.unknown-edge", "This edge does not exist.", "Diese Kante existiert nicht."),
        ("wfc2d.id.taken", "This id is already in use.", "Diese Kennung ist bereits vergeben."),
        ("wfc2d.example.unknown", "This example does not exist.", "Dieses Beispiel existiert nicht."),
        ("wfc2d.retained.extent", "The document is too large for this action.", "Das Dokument ist für diese Aktion zu groß."),
    ],
    ("🀄️wfc/🗿️artifacts/🧊️3d", "impl ArtifactEditor for Wfc3dEditor {"): [
        ("wfc3d.tile.unknown-pin", "This pin does not exist.", "Diese Fixierung existiert nicht."),
        ("wfc3d.example.unknown", "This example does not exist.", "Dieses Beispiel existiert nicht."),
        ("wfc3d.action.unknown", "This action is not known.", "Diese Aktion ist unbekannt."),
        ("wfc3d.retained.extent", "The document is too large for this action.", "Das Dokument ist für diese Aktion zu groß."),
    ],
    ("📸️remodel/🗿️artifacts/📸️remodeling", "impl ArtifactEditor for RemodelingPlayApp {"): [
        ("remodeling.import.open", "An import is still running in this window.", "In diesem Fenster läuft noch ein Import."),
        ("remodeling.import.window-required", "A streamed import needs an open window.", "Ein gestreamter Import braucht ein geöffnetes Fenster."),
        ("remodeling.stream.unknown-camera", "This camera is not calibrated in the document.", "Diese Kamera ist im Dokument nicht kalibriert."),
        ("remodeling.qc-report.missing", "There is no quality report to export yet.", "Es gibt noch keinen Qualitätsbericht zum Exportieren."),
        ("remodeling.retained.extent", "The document is too large for this action.", "Das Dokument ist für diese Aktion zu groß."),
    ],
    ("🏭️process/🗿️artifacts/🧊️process3d", "impl ArtifactEditor for Process3dPlayApp {"): [
        ("process3d.action.invalid", "This action's arguments are not valid.", "Die Argumente dieser Aktion sind ungültig."),
        ("process3d.media.export", "The model cannot be exported.", "Das Modell lässt sich nicht exportieren."),
    ],
}


def named(code: str) -> str:
    return f'semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("{code}"), "{code}")'


originals: dict[pathlib.Path, str] = {}
results: dict[pathlib.Path, str] = {}


def text(path: pathlib.Path) -> str:
    if path not in results:
        originals[path] = path.read_text()
        results[path] = originals[path]
    return results[path]


renamed = 0
for crate, mapping in RENAMES.items():
    for path in sorted((PLUGINS / crate / ANY).rglob("*.rs")):
        body = path.read_text()
        if not any(old in body for old in mapping):
            continue
        updated = text(path)
        for old, new in mapping.items():
            updated, count = re.subn(r'(?:semio_framework_plugin::)?Fault::from\("' + re.escape(old) + r'"\)', named(new), updated)
            renamed += count
            if old in updated:
                sys.exit(f"unconverted {old} in {path}")
        results[path] = updated

for (crate, anchor), rows in NOTICES.items():
    path = PLUGINS / crate / ANY / "✏️editor/🦀️.rs"
    updated = text(path)
    if updated.count(anchor) != 1:
        sys.exit(f"anchor count != 1 in {path}: {anchor}")
    if "fn fault_notices()" in updated:
        sys.exit(f"{path} already declares fault_notices")
    table = "\n".join(f'            ("{code}", LocalizedLabel::native("{en}", "{de}")),' for code, en, de in rows)
    method = (
        f"{anchor}\n"
        "    /// 📢️ The localized notices of this editor's user-reachable refusals (design §20.12).\n"
        "    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {\n"
        "        use semio_framework_ui_locale::LocalizedLabel;\n"
        f"        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); {len(rows)}]> = std::sync::LazyLock::new(|| {{\n"
        "            [\n"
        f"{table}\n"
        "            ]\n"
        "        });\n"
        "        &*NOTICES\n"
        "    }\n"
    )
    results[path] = updated.replace(anchor, method, 1)

for path, updated in results.items():
    if path.read_text() != originals[path]:
        sys.exit(f"file changed during the run: {path}")
for path, updated in results.items():
    if updated != originals[path]:
        path.write_text(updated)
print(f"renamed={renamed} files={sum(1 for p in results if results[p] != originals[p])}")
