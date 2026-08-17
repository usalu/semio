from pathlib import Path

puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())
f5 = puzzle / "🗿️artifacts/🖐️5d"

# Fix diff HasId
diff = f5 / "🔺️diff/🦀️component.rs"
text = diff.read_text()
broken = """impl HasId for crate::artifacts::puzzle5d::Puzzle5dPart {
            anchor: Puzzle5dPartAnchor::Fixed, fn id(&self) -> &str { &self.id } }
impl HasId for crate::artifacts::puzzle5d::Puzzle5dFastener { fn id(&self) -> &str { &self.id }, x: 0.0, y: 0.0 }"""
fixed = """impl HasId for crate::artifacts::puzzle5d::Puzzle5dPart { fn id(&self) -> &str { &self.id } }
impl HasId for crate::artifacts::puzzle5d::Puzzle5dFastener { fn id(&self) -> &str { &self.id } }"""
if broken not in text:
    raise SystemExit("broken HasId block not found")
text = text.replace(broken, fixed)
text = text.replace("use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dPart, Puzzle5dSnapshot, Puzzle5dPartAnchor};",
                    "use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dPart, Puzzle5dSnapshot};")
diff.write_text(text)
print("fixed diff")

# Delete compose
compose = f5 / "⚙️engine/🌉️compose"
import shutil
if compose.exists():
    shutil.rmtree(compose)
    print("deleted compose")
else:
    print("compose already gone")

# Engine reexport
engine = f5 / "⚙️engine/🦀️component.rs"
etext = engine.read_text()
etext = etext.replace(
"""//! 📚️ Sibling topic files: `🦀️transfer.rs` (the copy/paste closure rules and the translate/replace-kind
//! helpers), `🦀️compose.rs` (the semio-compose Design → `Puzzle5dSnapshot` importer).
""",
"""//! 📚️ Sibling topic files: `🦀️transfer.rs` (the copy/paste closure rules and the translate/replace-kind
//! helpers). Compose design import was removed in PUZZLE-DESIGN-PARITY Wave 1 (parity harness is Wave 5).
""")
etext = etext.replace("pub use crate::artifacts::puzzle5d::engine::compose::import_compose_design_json;\n", "")
engine.write_text(etext)
print("engine updated")

# glue.rs compose mod (needed so deleted path does not break the crate; B1 owns glue — see report)
glue = puzzle / "📦️packages/🦀️rust/📦️glue.rs"
g = glue.read_text()
old = """            #[path = \"../../🗿️artifacts/🖐️5d/⚙️engine/✂️transfer/🦀️component.rs\"]
            pub mod transfer;
            #[path = \"../../🗿️artifacts/🖐️5d/⚙️engine/🌉️compose/🦀️component.rs\"]
            pub mod compose;
"""
new = """            #[path = \"../../🗿️artifacts/🖐️5d/⚙️engine/✂️transfer/🦀️component.rs\"]
            pub mod transfer;
"""
if old not in g:
    raise SystemExit("glue compose block not found:\n" + g[g.find("transfer"):g.find("transfer")+400])
glue.write_text(g.replace(old, new))
print("glue compose mod removed")
