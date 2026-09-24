"""🖊️ AC1018's mutation vocabulary is a glob re-export of AC1024's `DwgMutation`, so its own leaf directories
(`📸️set-snapshot`, `🏷️set-version-info`) were dead duplicates the bridges measured instead of the dispatched leaves.
Deletes them, drops the crate-root mounts of the duplicated set-snapshot diff/inverse/mutation helpers (nothing calls
them), and moves the one AC1018-specific fixture case (the auxiliary save counter) to the vocabulary-level
`🧪️tests/🔬️fixture` module, where it keeps exercising AC1024's leaf through the AC1018 module path. `--dry` reports."""
import os, shutil, sys
root = "/Users/ueli/Documents/semio/"
dry = "--dry" in sys.argv
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/"
M = ART + "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
case_old = M + "📸️set-snapshot/🧪️tests/🔢️bumps-the-auxiliary-save-counter/🦀️.rs"
case_new = M + "🧪️tests/🔬️fixture/🦀️.rs"
actions, problems = [], []

def replace(rel, old, new):
    text = open(root + rel, encoding="utf-8").read()
    if text.count(old) != 1: problems.append(f"{rel}: expected 1 of {old[:70]!r}, found {text.count(old)}"); return None
    return text.replace(old, new)

case = open(root + case_old, encoding="utf-8").read()
if case.count('include_str!("../../../../../🧫️fixtures/') != 5: problems.append("case: expected 5 fixture includes")
case = case.replace('include_str!("../../../../../🧫️fixtures/', 'include_str!("../../../../🧫️fixtures/')
case = case.replace("//! 🧪️ `📸️set-snapshot` fixture — `🔢️bumps-the-auxiliary-save-counter` (AC1018).", "//! 🔢️ AC1018 `📸️set-snapshot` fixture case — `bumps-the-auxiliary-save-counter`.", 1)

mount_old = """/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases for the AC1018 tree, wired from this tree's own
/// mutations root so `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves
/// against this file's own directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🔢️bumps-the-auxiliary-save-counter/🦀️.rs"]
mod set_snapshot_bumps_the_auxiliary_save_counter;"""
mount_new = """/// 🧪️ The AC1018 fixture cases: this tree owns no leaves (its vocabulary is AC1024's), so its own drawings
/// exercise AC1024's leaves through this module path.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod tests_fixture;"""
vocab = replace(M + "🦀️.rs", mount_old, mount_new)

crate_old = """                        #[path = "."]
                        pub mod set_snapshot {
                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
"""
crate = replace(ART + "🦀️.rs", crate_old, "")

if problems:
    for p in problems: print("PROBLEM", p)
    sys.exit(1)
actions = [f"write {case_new}", f"edit {M}🦀️.rs", f"edit {ART}🦀️.rs", f"delete {M}📸️set-snapshot", f"delete {M}🏷️set-version-info"]
if not dry:
    os.makedirs(os.path.dirname(root + case_new), exist_ok=True)
    open(root + case_new, "w", encoding="utf-8").write(case)
    open(root + M + "🦀️.rs", "w", encoding="utf-8").write(vocab)
    open(root + ART + "🦀️.rs", "w", encoding="utf-8").write(crate)
    shutil.rmtree(root + M + "📸️set-snapshot")
    shutil.rmtree(root + M + "🏷️set-version-info")
for a in actions: print(("would " if dry else "") + a)
