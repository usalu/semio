"""Validate the published Table 8.6 hollow-section details and production catalogue outputs with SQLite."""
import json
import math
from pathlib import Path
import re
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
root = next(path.parent for path in ticket.parents if path.name == ".🧬semio")
artifact = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
schema = next(artifact.glob("🏅*/🔖*/🪆*/✳*/🧬*"))
catalogue = schema / "💡️inferences/⚖️checks/📇️catalogue"
fixture = json.loads((catalogue / "🧪️tests/table-8-6.json").read_text())["cases"]
evaluators = catalogue / "🧮️evaluators"
implementation = (evaluators / "🦀️.rs").read_text()
nominal = implementation[implementation.index("fn nominal_direct_checks("):implementation.index("/// 🚉 EN 1993-1-9 Table 8.8")]
paths = ticket / "🗑️generated"
paths.mkdir(exist_ok=True)
binary = paths / "en1993-table-8-6-probe"
source = binary.with_suffix(".rs")
source.write_text(r"""
use std::{collections::BTreeMap, io::{self, BufRead}};
enum QuantityKind { Stress }
struct Row { demand:f64, capacity:f64 }
impl Row {
    fn new(_: &str, _: &str, _: &str, _: QuantityKind, demand:f64, capacity:f64, _: String) -> Self { Self { demand, capacity } }
}
type Outcome = Result<Vec<Row>, String>;
struct Inputs<'a>(BTreeMap<&'a str, &'a str>);
impl Inputs<'_> {
    fn t(&self, key: &str) -> Result<&str,String> { self.0.get(key).copied().ok_or_else(|| key.to_owned()) }
    fn n(&self, key: &str) -> Result<f64,String> { self.t(key)?.parse().map_err(|_| key.to_owned()) }
}
""" + nominal + f'\n#[path = "{evaluators / "🛞️hollow-section/🦀️.rs"}"]\nmod node;\n' + r"""
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let args = Inputs(line.split('\t').map(|field| field.split_once('=').unwrap()).collect());
        match node::hollow_section_fatigue(&args) {
            Ok(rows) => println!("{}", rows.iter().map(|r| format!("{:.13},{:.13}",r.demand / 1e6,r.capacity / 1e6)).collect::<Vec<_>>().join(";")),
            Err(_) => println!("invalid"),
        }
    }
}
""")
subprocess.run(
    ["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc", "--edition", "2024", str(source), "-o", str(binary)],
    check=True, cwd=root,
)
params = [{**choices, **numbers} for _, choices, numbers, _ in fixture]
payload = "".join("\t".join(f"{key}={value}" for key, value in row.items()) + "\n" for row in params)
rust = subprocess.check_output([binary], input=payload, text=True).splitlines()
assert len(rust) == len(params)
db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE detail (id TEXT PRIMARY KEY, base REAL, thick REAL)")
db.executemany("INSERT INTO detail VALUES (?,?,?)", (
    ("flattened",71,71), ("slotted",71,63), ("chs-butt",71,90), ("rhs-butt",56,71), ("non-load-bearing",71,71),
    ("chs-butt-endplate",50,56), ("rhs-butt-endplate",45,50), ("chs-fillet-endplate",40,None), ("rhs-fillet-endplate",36,None),
))
query = """
WITH classification AS (
SELECT CASE
 WHEN id = 'slotted' THEN IIF(:hollow_slot_angle <= 45,base,thick)
 WHEN :hollow_wall <= 8 THEN base ELSE thick END AS category
FROM detail WHERE id = :hollow_detail
AND :hollow_execution = 'confirmed' AND :hollow_wall > 0 AND :hollow_wall <= 12.5
AND :hollow_range >= 0 AND :hollow_frequent >= 0 AND min(:gamma_ff,:gamma_mf,:fy) > 0
AND CASE id
  WHEN 'flattened' THEN :hollow_diameter > 0 AND :hollow_diameter < 200
  WHEN 'slotted' THEN :hollow_slot_angle BETWEEN 0 AND 180 AND min(:hollow_weld_shear,:hollow_weld_frequent_shear) >= 0
  WHEN 'non-load-bearing' THEN :hollow_attachment_width > 0 AND :hollow_attachment_width <= 100
  ELSE id NOT IN ('chs-butt','rhs-butt') OR
    (:hollow_weld_dimension > 0 AND :hollow_reinforcement >= 0 AND 10*:hollow_reinforcement <= :hollow_weld_dimension)
END
)
SELECT :hollow_range*:gamma_ff, category/:gamma_mf,
       :hollow_frequent, 1.5*:fy,
       :hollow_weld_shear*:gamma_ff, 80/:gamma_mf,
       :hollow_weld_frequent_shear, 1.5*:fy/:sqrt3
FROM classification WHERE category IS NOT NULL
"""
for input_row, observed, (name, *_, expected) in zip(params, rust, fixture):
    result = db.execute(query, {**input_row,"sqrt3":math.sqrt(3)}).fetchone()
    if not expected:
        assert result is None and observed == "invalid", (name, observed, result)
        continue
    assert result is not None, name
    oracle = list(result if input_row["hollow_detail"] == "slotted" else result[:4])
    wanted = [amount for demand, capacity, _ in expected for amount in (demand,capacity)]
    numbers = [float(value) for value in re.split("[;,]", observed)]
    assert len(oracle) == len(wanted) and all(math.isclose(x, y, rel_tol=1e-12) for x,y in zip(oracle,wanted)), (name, result, expected)
    assert len(numbers) == len(wanted) and all(math.isclose(x, y, rel_tol=1e-12) for x,y in zip(numbers,wanted)), (name, observed, expected)
    assert ["pass" if numbers[i] <= numbers[i+1] else "fail" for i in range(0,len(numbers),2)] == [row[2] for row in expected], name
print(f"[DEBUG] EN 1993-1-9 Table 8.6 {len(fixture)} cases matched production Rust and SQLite.")
