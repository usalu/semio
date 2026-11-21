from pathlib import Path
import math
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
artifact = next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599").glob("🏅*/🔖*/🪆*/✳*"))
from_balance = (artifact/"🧬️schema/💡️inferences/⚖️checks/🔥️derived-balance/🦀️.rs").read_text()
function = from_balance[from_balance.index("pub fn residential_dhw_useful_kwh_a("):].split("\n}\n", 1)[0] + "\n}\n"
binary = ticket/"🗑️generated/din18599-residential-dhw"
source = binary.with_suffix(".rs")
source.write_text(function + """
use std::io::{self, BufRead};

fn main() {
    for row in io::stdin().lock().lines() {
        let row = row.unwrap();
        let mut cells = row.split('\\t');
        let area = cells.next().unwrap().parse().unwrap();
        let dwellings = cells.next().unwrap().parse().unwrap();
        match residential_dhw_useful_kwh_a(area, dwellings) {
            Some(value) => println!("{value:.12}"),
            None => println!("scope"),
        }
    }
}
""")
subprocess.run(
    ["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
     "--edition", "2024", str(source), "-o", str(binary)],
    check=True, cwd=workspace,
)
dataset = (artifact/"🧪️tests/🧫️fixtures/🚿residential-hot-water/🔣️.tsv").read_text().splitlines()[1:]
observed = subprocess.check_output([binary], input="\n".join(dataset)+"\n", text=True).splitlines()
assert len(observed) == len(dataset)
db = sqlite3.connect(":memory:")
for row, result in zip(dataset, observed):
    area_text, dwelling_text, expected = row.split("\t")
    area, count = float(area_text), int(dwelling_text)
    independent = db.execute("""
       SELECT CASE WHEN :n < 1 OR :a <= 0 OR :a > 1.7976931348623157e308 THEN NULL
                   WHEN :a > 160.0 * :n THEN :a * 8.5
                   ELSE (330.0 * :n - :a) * :a / (20.0 * :n)
              END
    """, {"n": count, "a": area}).fetchone()[0]
    if expected == "scope":
        assert result == "scope" and independent is None, row
    else:
        assert math.isclose(float(result), float(expected), rel_tol=0, abs_tol=1e-9), row
        assert math.isclose(independent, float(expected), rel_tol=0, abs_tol=1e-9), row
print(f"[DEBUG] {len(dataset)} residential DHW cases matched the production Rust formula and independent SQLite.")
