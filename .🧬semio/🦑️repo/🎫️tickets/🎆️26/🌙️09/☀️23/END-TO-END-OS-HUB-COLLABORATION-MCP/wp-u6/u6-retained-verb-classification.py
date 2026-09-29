#!/usr/bin/env python3
"""🧵️ U6 test-only set (rule 22): the edit-window laws of the now-classified semio `set-vertex` (same class as LB2 p4).

The production half (brep/mesh `set_vertex_action()` + wav `extra_actions()` stamping `Migrated` where declared) landed by a peer at
2026-09-29 03:31:58, identical to the fix prepared here; what remains is the test half below.

Measured 01:5x (`.🧬semio/🌐hub/s14-u6-logs/a2-check-test.txt`): building the editor definition panics
`app-definition.interactive-job-classification: unclassified interactive command …` for
* semio brep + mesh `set-vertex` — declared once by `set_vertex_action()` with `ActionDefinition::bounded_catalog` (unclassified),
  used by the editor (`.action_with`) AND by the edit window, which REPLACES the mesh kit's own (classified) rows with it
  (`framework.window.mesh:set-vertex`); 10 tests;
* wav `insert-frame`, `insert-channel`, `set-sample-rate` — `edit_audio::extra_actions()` appends them to the structural table
  window after the kit scaffold classified its rows (`framework.window.table:*`); 2 tests.
All five ARE retained routes (brep/mesh register the `set-vertex` retained command roster, wav's `edit_audio::TOOL_IDS` carries the
three ids), so the declaration stamps `Migrated` where the action is built — every app using it assembles, the editor and the
window agree. Test half: the brep/mesh edit-window laws asserted the window declares NO action (true before the window carried
`set-vertex`); they now assert exactly the classified `set-vertex`.

Usage: u6-retained-verb-classification.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/retained-verb-classification")
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
SEMIO = ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/"
WINDOW_TEST_OLD = "    assert!(def.actions.is_empty());\n"
WINDOW_TEST_NEW = """    let [action] = def.actions.as_slice() else { panic!("the edit window carries exactly the set-vertex action") };
    assert_eq!(action.id, "set-vertex");
    assert_eq!(action.semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated);
"""

SETS = {
    SEMIO + "🧊️brep/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [(WINDOW_TEST_OLD, WINDOW_TEST_NEW, 1)],
    SEMIO + "🔺️mesh/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [(WINDOW_TEST_OLD, WINDOW_TEST_NEW, 1)],
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
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/🪆️subsets/', 1)[-1]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
