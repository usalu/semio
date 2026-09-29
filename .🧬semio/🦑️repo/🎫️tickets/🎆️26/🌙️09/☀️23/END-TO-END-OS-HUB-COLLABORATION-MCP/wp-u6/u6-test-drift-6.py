#!/usr/bin/env python3
"""🧪️ U6 set A6 — the pre-existing docx/xlsx lib-test compile drift (test-only, rule 22; L1's T1 baseline: docx 10, xlsx 5).

* docx base editor tests: the admission fixture lives in `✏️editor/📬️preparation/🧫️fixtures/🧵️admission` — the include walked one
  directory too far (`../../../` from `🧪️tests/🔬️unit`); `VcsArtifactApp::snapshot()` answers the snapshot by value now → `run(&…)`
  and a by-value comparison.
* docx strict/transitional editor tests: `set_run_text` is not imported by those editors → the leaf's crate path.
* xlsx base editor test: `set_shared_string` likewise → the leaf's crate path.
* xlsx base/strict/transitional edit-window laws: the rendered cell binding carries the canonical cell address revision
  (`cell_address::xlsx_cell_address(..).revision`, the same authority the base editor law uses), not the retired `xlsx_cell_revision`
  scheme (unreachable from these modules and no longer what production binds).

Usage: u6-test-drift-6.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/test-drift-6")
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
DOCX = ART + "📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/"
XLSX = ART + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/"
RUN_TEXT = ("DocxMutation::SetRunText(set_run_text::SetRunText {", "DocxMutation::SetRunText(crate::schema::mutations::set_run_text::SetRunText {")
REVISION = (
    'if value.as_str() == xlsx_cell_revision(&XlsxCellValue::Number(1.0), &[])));\n',
    'if value.as_str() == crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&document, "Sheet1", 1, 0).expect("rendered cell address").revision));\n',
)
SETS = {
    DOCX + "🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [
        ('include_str!("../../../📬️preparation/🧫️fixtures/🧵️admission/🔣️.json")', 'include_str!("../../📬️preparation/🧫️fixtures/🧵️admission/🔣️.json")', 2),
        ("    let original_address = run(app.snapshot().unwrap()).address;\n", "    let original_address = run(&app.snapshot().unwrap()).address;\n", 1),
        ('    assert_eq!(run(app.snapshot().unwrap()).text, "after");\n', '    assert_eq!(run(&app.snapshot().unwrap()).text, "after");\n', 3),
        ("    let current_address = run(app.snapshot().unwrap()).address;\n", "    let current_address = run(&app.snapshot().unwrap()).address;\n", 1),
        ("    assert_eq!(app.snapshot().unwrap(), &opened);\n", "    assert_eq!(app.snapshot().unwrap(), opened);\n", 1),
    ],
    DOCX + "📏️strict/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [(*RUN_TEXT, 1)],
    DOCX + "🔄️transitional/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [(*RUN_TEXT, 1)],
    XLSX + "🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [
        ("&XlsxMutation::SetSharedString(set_shared_string::SetSharedString {", "&XlsxMutation::SetSharedString(crate::standards::v_ecma_376::subsets::base::schema::mutations::set_shared_string::SetSharedString {", 2),
    ],
    **{XLSX + f"{subset}/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [(*REVISION, 1)] for subset in ["🧱️base", "🔒️strict", "🌉️transitional"]},
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/🪆️subsets/', 1)[-1][:70]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
