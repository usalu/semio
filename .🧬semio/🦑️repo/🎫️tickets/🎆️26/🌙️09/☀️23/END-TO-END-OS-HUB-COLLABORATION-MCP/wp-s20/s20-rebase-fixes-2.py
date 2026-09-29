"""🧯️ S20 faults overlay, session 15: the compile reds of the union check `s15-union-1` (L1's 228 crates on the rebased
overlay, log `.🧬semio/🌐hub/s14-s20-overlay-build/logs/0929-194444-s15-union-1.txt`) — process3d's flag-value refusal (a
round-3a site calling the retired `process3d_action_fault`), draw geometry/fill free-text `ok_or("…")` refusals FH2 left,
os-mcp gateway faults built without `parameters`/`texts`, and test helpers still building faults from text.
Idempotent (an edit whose result is present is skipped; its anchor must match exactly once otherwise).
Usage: python3 s20-rebase-fixes-2.py [--dry-run]"""
from __future__ import annotations

import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
P = "✏️s/🔌️plugins"
PROCESS = f"{P}/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
DRAW = f"{P}/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any"
GEOMETRY = f"{DRAW}/🧬️schema/🧮️geometry/✏️editing/🦀️.rs"
FILL = f"{DRAW}/🧬️schema/🎨️fill/🦀️.rs"
DRAW_EDITOR = f"{DRAW}/✏️editor/🦀️.rs"
DRAW_GESTURE_TEST = f"{DRAW}/✏️editor/🧪️tests/🔬️gesture-operation-owner/🦀️.rs"
MCP = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"
REMODEL_TEST = f"{P}/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎞️import-frames/🧪️tests/🔬️unit/🦀️.rs"
PUZZLE = f"{P}/🧩️puzzle/🗿️artifacts"
TESTS = {
    "2d": f"{PUZZLE}/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "3d": f"{PUZZLE}/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "5d": f"{PUZZLE}/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "5d-retirement": f"{PUZZLE}/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️puzzle5d-retained-retirement-laws/🦀️.rs",
    "note": f"{P}/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
}
GENERATION3D_TEST = f"{P}/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs"

PAGE_TEXT = "Fault::from(String::from_utf8_lossy(page.bytes()).into_owned())"
PAGE_RECORD = (
    "semio_framework_plugin::app::TypedOperationFault::decode(page.bytes()).map_or_else("
    "|| Fault::new(semio_framework_plugin::FaultOrigin::Framework, semio_framework_plugin::FaultCode::new(\"interactive-job.fault-page-invalid\"), \"the fault page carries no typed record\"), "
    "|record| Fault::new(record.origin, semio_framework_plugin::FaultCode::received(record.code), record.message))"
)


def test_fault(code: str, text: str) -> str:
    return f'Fault::new(semio_framework_plugin::FaultOrigin::Framework, semio_framework_plugin::FaultCode::new("{code}"), {text})'


GEOMETRY_SENTENCES = {
    "A contour must start with a move": "contour-invalid", "Missing path node": "node-missing", "Missing previous anchor": "node-missing",
    "Invalid contour": "contour-invalid", "Choose endpoints of open contours": "join-endpoints-required", "Missing target endpoint": "node-missing",
    "Missing endpoint": "node-missing", "Missing contour": "contour-invalid", "Missing segment start": "node-missing",
    "Missing segment end": "node-missing", "Missing next endpoint": "node-missing", "This node has no selected handle": "handle-missing",
    "Select a segment to split": "segment-required", "Invalid split geometry": "geometry-invalid", "Invalid arc geometry": "geometry-invalid",
    "Select an anchor": "anchor-required", "Cannot move points through a singular transform": "transform-singular",
}
FILL_SENTENCES = {"Missing gradient stop": "stop-missing", "Enable a fill first": "fill-required", "Choose a hexadecimal color": "color-invalid"}

EDITS: list[tuple[str, str, str]] = [
    (PROCESS,
     'ok_or_else(|| process3d_action_fault(action, "requires the boolean \'enabled\' it sets"))',
     'ok_or_else(|| app_fault("process3d.action.flag-value-required").with_parameter("action", action).with_parameter("flag", "enabled"))'),
    (PROCESS,
     '            .fault("process3d.action.unknown", LocalizedLabel::native("The action {action} is not available in the process editor.", "Die Aktion {action} ist im Prozesseditor nicht verfügbar."))\n',
     '            .fault("process3d.action.unknown", LocalizedLabel::native("The action {action} is not available in the process editor.", "Die Aktion {action} ist im Prozesseditor nicht verfügbar."))\n'
     '            .fault("process3d.action.flag-value-required", LocalizedLabel::native("The action {action} needs {flag} set to on or off; choose it and try again.", "Die Aktion {action} benötigt für {flag} den Wert ein oder aus; wählen Sie ihn und versuchen Sie es erneut."))\n'),
    *[(GEOMETRY, f'.ok_or("{sentence}")', f'.ok_or_else(|| app_fault("drawing.path.{code}"))') for sentence, code in GEOMETRY_SENTENCES.items()],
    *[(FILL, f'.ok_or("{sentence}")', f'.ok_or_else(|| app_fault("drawing.fill.{code}"))') for sentence, code in FILL_SENTENCES.items()],
    (DRAW_EDITOR,
     '            .fault("drawing.fill.edit-invalid",',
     '            .fault("drawing.fill.fill-required", LocalizedLabel::native("Turn on a fill first.", "Aktivieren Sie zuerst eine Füllung."))\n'
     '            .fault("drawing.fill.edit-invalid",'),
    (DRAW_EDITOR,
     '            .fault("drawing.fill.stop-position-invalid",',
     '            .fault("drawing.fill.stop-missing", LocalizedLabel::native("The gradient stop no longer exists; select a stop again.", "Der Verlaufspunkt existiert nicht mehr; wählen Sie erneut einen Punkt aus."))\n'
     '            .fault("drawing.fill.stop-position-invalid",'),
    (DRAW_EDITOR,
     '            .fault("drawing.path.locked",',
     '            .fault("drawing.path.join-endpoints-required", LocalizedLabel::native("Choose the end points of two open contours to join them.", "Wählen Sie die Endpunkte zweier offener Konturen, um sie zu verbinden."))\n'
     '            .fault("drawing.path.locked",'),
    (DRAW_EDITOR,
     '            .fault("drawing.path.translation-invalid",',
     '            .fault("drawing.path.transform-singular", LocalizedLabel::native("The shape is scaled to zero size, so its points cannot be moved; reset its scale first.", "Die Form ist auf die Größe null skaliert, daher lassen sich ihre Punkte nicht verschieben; setzen Sie zuerst ihre Skalierung zurück."))\n'
     '            .fault("drawing.path.translation-invalid",'),
    (DRAW_GESTURE_TEST,
     'fault: Fault::new(FaultOrigin::Framework, FaultCode::new("test.completion-rejected"), "injected completion rejection"),',
     'fault: Fault::new(semio_framework_plugin::FaultOrigin::Framework, semio_framework_plugin::FaultCode::new("test.completion-rejected"), "injected completion rejection"),'),
    (MCP,
     ', view_lanes.join(" and ")),\n        });',
     ', view_lanes.join(" and ")),\n            ..Fault::default()\n        });'),
    (MCP,
     'message: format!("abandoned command owner: {}: {}", fault.code.0, fault.message) })?;',
     'message: format!("abandoned command owner: {}: {}", fault.code.0, fault.message), ..Fault::default() })?;'),
    (MCP,
     'the instance was discarded and the next command starts on a fresh one") });',
     'the instance was discarded and the next command starts on a fresh one"), ..Fault::default() });'),
    (REMODEL_TEST,
     'let fault = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect_err("an export with no report is refused");',
     'let Err(fault) = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await else { panic!("an export with no report is refused") };'),
    (TESTS["2d"], PAGE_TEXT, PAGE_RECORD),
    (TESTS["2d"], 'Err(Fault::from("puzzle2d test operation did not settle"))', "Err(" + test_fault("test.operation-unsettled", '"puzzle2d test operation did not settle"') + ")"),
    (TESTS["note"], "return Err(" + PAGE_TEXT + ");", "return Err(" + PAGE_RECORD + ");"),
    (TESTS["note"], 'Err(Fault::from("Note test operation did not settle"))', "Err(" + test_fault("test.operation-unsettled", '"Note test operation did not settle"') + ")"),
    (TESTS["5d"], 'Err(Fault::from("puzzle5d test operation did not settle"))', "Err(" + test_fault("test.operation-unsettled", '"puzzle5d test operation did not settle"') + ")"),
    (TESTS["5d"], 'Err(Fault::from("puzzle5d traced operation did not settle"))', "Err(" + test_fault("test.operation-unsettled", '"puzzle5d traced operation did not settle"') + ")"),
    (TESTS["5d-retirement"], 'let mut fault = Fault::from("injected completion rejection");', "let mut fault = " + test_fault("test.completion-rejected", '"injected completion rejection"') + ";"),
    (TESTS["3d"], 'Some(reason) => Err(Fault::from(format!("close drained non-terminal while blocked on {reason}"))),', "Some(reason) => Err(" + test_fault("test.close-blocked", 'format!("close drained non-terminal while blocked on {reason}")') + "),"),
    (GENERATION3D_TEST, 'FaultCode::new(format!("extension.{capability}.bad-request"))', 'FaultCode::received(format!("extension.{capability}.bad-request"))'),
]
EVERY = {(TESTS["5d"], PAGE_TEXT, PAGE_RECORD)}


def main(dry_run: bool) -> None:
    texts: dict[str, str] = {}
    for rel, old, new in [*EDITS, *EVERY]:
        text = texts.setdefault(rel, (OVERLAY / rel).read_text())
        if (rel, old, new) in EVERY:
            count = text.count(old)
            texts[rel] = text.replace(old, new)
            print(f"every ×{count} {rel.split('/')[-4]} {new[:60]}")
            continue
        if new in text and (old in new or old not in text):
            continue
        count = text.count(old)
        assert count >= 1, (rel, old[:90])
        if old.startswith(".ok_or(\""):
            texts[rel] = text.replace(old, new)
        else:
            assert count == 1, (rel, old[:90], count)
            texts[rel] = text.replace(old, new)
        print(f"apply ×{count} {rel.split('/')[-3]} {new[:70]}")
    if not dry_run:
        for rel, text in texts.items():
            if (OVERLAY / rel).read_text() != text:
                (OVERLAY / rel).write_text(text)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
