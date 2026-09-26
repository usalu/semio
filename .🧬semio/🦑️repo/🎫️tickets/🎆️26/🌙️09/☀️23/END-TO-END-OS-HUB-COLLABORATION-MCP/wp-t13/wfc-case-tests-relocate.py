#!/usr/bin/env python3
"""🧳️ Moves the in-crate test modules that sat in wfc CASE directories (grid2d, grid3d mutate cases) into their
mutation roots' unit-test files, so the case directories can hold real adapters (F10). Refuses a second run."""
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts")
write = "--write" in sys.argv
problems, edits = [], []


def edit(path, old, new):
    text = path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        problems.append(f"{path}: anchor found {text.count(old)} times: {old[:80]!r}")
        return
    edits.append((path, text.replace(old, new)))


grid2d = ROOT / "🔲️grid2d"
edit(grid2d / "🦀️.rs", '\n#[cfg(test)]\n#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔲️mutate-grid2d-1/🦀️.rs"]\nmod mutate_grid2d_1;\n', "\n")
unit2d = grid2d / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
text2d = unit2d.read_text(encoding="utf-8")
if "mod committed_tree" in text2d:
    problems.append(f"{unit2d}: committed_tree already present")
else:
    edits.append((unit2d, text2d.rstrip("\n") + '''

//#region 🌳️CommittedTree
/// 🌳️ Every declared kind owns exactly one committed vector directory holding at least one vector — the tree the
/// per-kind fixture tests and the `🔲️mutate-grid2d-1` case replay, so a kind cannot drop out of both unnoticed.
mod committed_tree {
    use super::KINDS;

    fn fixtures() -> std::path::PathBuf {
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../")).join("🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
    }

    #[test]
    fn the_committed_tree_covers_every_declared_kind() {
        let kinds: Vec<std::path::PathBuf> = std::fs::read_dir(fixtures()).expect("fixture root reads").filter_map(Result::ok).map(|entry| entry.path()).filter(|path| path.is_dir()).collect();
        assert_eq!(kinds.len(), KINDS.len());
        for kind in &kinds {
            let vectors = std::fs::read_dir(kind).expect("kind reads").filter_map(Result::ok).filter(|entry| entry.path().is_dir()).count();
            assert!(vectors > 0, "{} holds no committed vector", kind.display());
        }
    }
}
//#endregion 🌳️CommittedTree
'''))

grid3d = ROOT / "🧱️grid3d"
edit(grid3d / "🦀️.rs", '\n#[cfg(test)]\n#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-wfc-grid3d-1/🦀️.rs"]\nmod mutate_wfc_grid3d_1;\n', "\n")
case3d = grid3d / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-wfc-grid3d-1/🦀️.rs"
unit3d = grid3d / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
source = case3d.read_text(encoding="utf-8")
if "pub fn adapter()" in source:
    problems.append(f"{case3d}: already an adapter")
else:
    body = source.split("use crate::schema::mutations::KINDS;\n", 1)
    if len(body) != 2:
        problems.append(f"{case3d}: KINDS import anchor missing")
    else:
        moved = body[1]
        moved = moved.replace('include_str!("../../🧬️schema/🧬️mutations/', 'include_str!("../../')
        moved = moved.replace('include_str!("../../🔮️oracles/🔣️.json")', 'include_str!("../../../../🔮️oracles/🔣️.json")')
        moved = moved.replace('include_str!("🥒️.feature")', 'include_str!("../../../../🧪️tests/🧩️mutate-wfc-grid3d-1/🥒️.feature")')
        moved = moved.replace('include_str!("🐍️.py")', 'include_str!("../../../../🧪️tests/🧩️mutate-wfc-grid3d-1/🐍️.py")')
        leftovers = [m for m in re.findall(r'include_str!\("([^"]+)"\)', moved) if not m.startswith("../../")]
        if leftovers:
            problems.append(f"unrewritten include paths: {leftovers}")
        indented = "\n".join(("    " + line) if line.strip() else "" for line in moved.strip("\n").split("\n"))
        text3d = unit3d.read_text(encoding="utf-8")
        if "mod roster" in text3d:
            problems.append(f"{unit3d}: roster already present")
        else:
            edits.append((unit3d, text3d.rstrip("\n") + '''

//#region 🏷️Roster
/// 🏷️ The mutation VOCABULARY the enum declares, the kebab roster `KINDS` publishes, the per-kind manifests on disk,
/// the payload schemas, the oracle catalog, the TypeScript twin and the `🧩️mutate-wfc-grid3d-1` case's feature table
/// and Python oracle are one and the same set, in one order. The committed quintets themselves are replayed per kind by
/// each mutation's own fixture test and by that case.
mod roster {
    use super::KINDS;

''' + indented + "\n}\n//#endregion 🏷️Roster\n"))

for path, _ in edits:
    print(("write " if write else "dry-run ") + str(path))
for problem in problems:
    print("problem:", problem)
if problems:
    sys.exit(1)
if write:
    for path, text in edits:
        path.write_text(text, encoding="utf-8")
