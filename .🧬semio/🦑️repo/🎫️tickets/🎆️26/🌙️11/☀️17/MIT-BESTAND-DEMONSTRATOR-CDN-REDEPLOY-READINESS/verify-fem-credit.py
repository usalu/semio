"""🏗️ Exercise the FEM registry's actual reserve and release methods at both address widths."""
import json
import os
from pathlib import Path
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
repo = ticket.parents[6]
session = next(repo.glob("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/**/✏️editor/🧵️session"))
law = json.loads((session / "🧪️tests/🧫️fixtures/🔣️.json").read_bytes())["shellCredit"]
code = (session / "📍️snapshot-admission-and-registry/🦀️.rs").read_text()
methods = code[code.index("    pub(super) fn reserve_credit"):code.index("    pub(super) fn retain_retirement")].replace("pub(super) fn", "fn")
out = ticket / "🗑️generated"
source = out / "fem_credit.rs"
oracle = sqlite3.connect(":memory:")
for bits in (32, 64):
    expected = law[f"admitted{bits}"]
    maximum = law["wasmShellBytes"]
    ceiling = min(2**bits - 1, len(expected) * maximum)
    for index, value in enumerate(expected, 1):
        assert bool(oracle.execute("select ? * ? <= ?", (index, maximum, ceiling)).fetchone()[0]) == value
    checked = methods.replace("admitted_items: usize", "admitted_items: Word").replace("admitted_bytes: usize", "admitted_bytes: Word")
    source.write_text(f"""type Word=u{bits};
const SESSION_SHELL_CAPACITY:Word=64;
const SESSION_MAXIMUM_ITEMS:Word=1000;
const SESSION_MAXIMUM_BYTES:Word={maximum};
#[derive(Default)] struct Registry {{ credit_items:[Word;4],credit_bytes:[Word;4],reserved_items:Word,reserved_bytes:Word }}
impl Registry {{ {checked} }}
fn main() {{ let mut arena=Registry::default();
for (index,expected) in {str(expected).lower()}.into_iter().enumerate() {{
assert_eq!(arena.reserve_credit(index as u16,1,SESSION_MAXIMUM_BYTES),expected);
assert_eq!(arena.credit_bytes[index],if expected {{SESSION_MAXIMUM_BYTES}} else {{0}});
}} arena.release_credit(0);assert!(arena.reserve_credit(0,1,SESSION_MAXIMUM_BYTES)); }}
""")
    binary = out / ("fem-credit.exe" if os.name == "nt" else "fem-credit")
    subprocess.run(["rustc", "--edition=2024", "-Adead_code", str(source), "-o", str(binary)], check=True)
    subprocess.run([str(binary)], check=True)
    print(f"{bits}-bit release and reservation agreed with SQLite")
