"""🌡️ Compare DIN EN 16798 thermal design tables and Figure B.1 with SQLite."""
import json
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
artifact = next((workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798").glob("🏅*/🔖*/🪆*/✳*"))
root = artifact / "🧪️tests/🧫️fixtures"
adaptive = json.loads((root / "🌡️comfort-vectors/🔣️.json").read_text())["adaptive"]
seasonal = [line.split("\t") for line in (root / "🌡️design-temperatures/🔣️.tsv").read_text().splitlines()[1:]]
code = (artifact / "🧬️schema/💡️inferences/⚖️checks/📜️annex-parameters-and-parts/🦀️.rs").read_text()
season = code[code.index("    pub fn seasonal_design_temperatures"):code.index("    pub fn humidity_band")]
bands = code[code.index("    pub const ADAPTIVE_RUNNING_MEAN_RANGE_C"):code.index("    /// 😌️ ISO 7730")]
binary = ticket / "🗑️generated/din16798-thermal-design"
source = binary.with_suffix(".rs")
source.write_text("""
#![allow(private_interfaces)]
use std::io::{self, BufRead};
enum AnnexChoice { En, De }
enum OccupancyType { Office, Residential, Meeting, Classroom, Retail, Kitchen, Corridor }
enum ComfortCategory { I, II, III, IV }
""" + season + bands + r"""
fn main() {
    use OccupancyType::*;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let columns: Vec<_> = line.split('\t').collect();
        let category = match columns[2] {
            "1" | "I" => ComfortCategory::I,
            "2" | "II" => ComfortCategory::II,
            "3" | "III" => ComfortCategory::III,
            "4" | "IV" => ComfortCategory::IV,
            _ => panic!("category"),
        };
        let result = if columns[0] == "adaptive" {
            adaptive_operative_bounds(columns[1].parse().unwrap(), category)
        } else {
            let annex = if columns[0] == "de" { AnnexChoice::De } else { AnnexChoice::En };
            let usage = match columns[1] {
                "office" => Office, "residential" => Residential, "meeting" => Meeting, "classroom" => Classroom,
                "retail" => Retail, "kitchen" => Kitchen, "corridor" => Corridor,
                _ => panic!("usage"),
            };
            seasonal_design_temperatures(annex, usage, category)
        };
        match result {
            Some((low, high)) => println!("{low:.10}\t{high:.10}"),
            None => println!("scope"),
        }
    }
}
""")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc", "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=workspace)
input_rows = [f"adaptive\t{row['theta_rm']}\t{row['category']}" for row in adaptive] + ["\t".join(row[:3]) for row in seasonal]
observed = subprocess.check_output([binary], input="\n".join(input_rows) + "\n", text=True).splitlines()
assert len(observed) == len(input_rows)
db = sqlite3.connect(":memory:")
db.executescript("""
CREATE TABLE thermal (category INTEGER PRIMARY KEY, office_heat REAL, living_heat REAL, cooling REAL, lower_offset REAL, upper_offset REAL);
INSERT INTO thermal VALUES (1,21,21,25.5,-3,2),(2,20,20,26,-4,3),(3,19,18,27,-5,4),(4,18,16,28,NULL,NULL);
""")
for row, actual in zip(adaptive, observed):
    lo, hi = db.execute("""
        SELECT 18.8 + :theta * .33 + lower_offset, 18.8 + :theta * .33 + upper_offset
        FROM thermal WHERE category = :category AND :theta > 10 AND :theta < 30
        """, {"theta": row["theta_rm"], "category": row["category"]}).fetchone() or (None, None)
    if lo is None:
        assert hi is None and row["lower"] is None and row["upper"] is None and actual == "scope", row
    else:
        assert math.isclose(row["lower"], lo, abs_tol=1e-9) and math.isclose(row["upper"], hi, abs_tol=1e-9), row
        assert all(math.isclose(float(x), y, abs_tol=1e-9) for x,y in zip(actual.split("\t"), (lo, hi))), (row, actual)
for row, actual in zip(seasonal, observed[len(adaptive):]):
    annex, usage, category, heating, cooling = row
    result = db.execute("""
        SELECT CASE WHEN :usage = 'residential' THEN living_heat ELSE office_heat END, cooling
        FROM thermal WHERE category = :cat
        AND (:usage IN ('office','meeting','classroom') OR (:annex = 'en' AND :usage = 'residential'))
        """, {"annex": annex, "usage": usage, "cat": ("I", "II", "III", "IV").index(category) + 1}).fetchone()
    if heating == "scope":
        assert result is None and actual == "scope", (row, actual)
    else:
        assert tuple(map(float, actual.split("\t"))) == result == (float(heating), float(cooling)), row
print(f"[DEBUG] {len(adaptive)} Figure B.1 scenarios and {len(seasonal)} seasonal design scenarios matched production Rust and SQLite.")
