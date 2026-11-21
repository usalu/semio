"""🧪 Compare Table 8.4 attachment classifications and verification ranges with SQLite."""
import json
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
root = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
family = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
catalogue = next(family.glob("🏅*/🔖*/🪆*/✳*/🧬*/💡*/⚖*/📇*catalogue"))
fixture = json.loads((catalogue / "🧪️tests/table-8-4.json").read_text())["cases"]
vocabulary = json.loads((catalogue / "📚️inputs/🔣️.json").read_text())["inputs"]
defaults = {row["key"]: row["example"] for row in vocabulary if row.get("scope") == "C93-28" or not row.get("scope") and row["key"] in {"gamma_ff", "gamma_mf", "fy"}}
evaluator = catalogue / "🧮️evaluators"
shared = (evaluator / "🦀️.rs").read_text()
model = shared[shared.index("pub struct Row {"):shared.index("/// 🟦️ EN 1993-1-7")]
direct = shared[shared.index("fn nominal_direct_checks("):shared.index("/// 🚉 EN 1993-1-9 Table 8.8")]
generated = ticket / "🗑️generated"
binary = generated / "en1993-table-8-4-probe"
source = binary.with_suffix(".rs")
source.write_text(r"""
use std::{collections::BTreeMap, io::{self, BufRead}};
#[derive(Clone, Copy)]
pub enum QuantityKind { Stress, Dimensionless }
#[derive(Default)]
pub struct Values {
    numbers: BTreeMap<String, f64>,
    texts: BTreeMap<String, String>,
}
#[derive(Default)]
pub struct DesignCase { values: Values }
""" + model + direct + f'\n#[path = "{evaluator / "🧱️attachment-fatigue/🦀️.rs"}"]\nmod weld;\n' + r"""
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut case = DesignCase::default();
        for field in line.split('\t') {
            let (key, value) = field.split_once('=').unwrap();
            if let Ok(value) = value.parse() {
                case.values.numbers.insert(key.into(), value);
            } else {
                case.values.texts.insert(key.into(), value.into());
            }
        }
        match weld::attachment_fatigue(&Inputs(&case)) {
            Ok(rows) => println!("{}", rows.iter().map(|row| {
                let scale = if matches!(row.kind, QuantityKind::Stress) { 1e6 } else { 1.0 };
                format!("{:.13},{:.13}", row.demand / scale, row.capacity / scale)
            }).collect::<Vec<_>>().join(";")),
            Err(_) => println!("invalid"),
        }
    }
}
""")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=root)
params = [{**defaults, **choices, **numbers} for _, choices, numbers, _ in fixture]
payload = "".join("\t".join(f"{k}={v}" for k, v in row.items()) + "\n" for row in params)
observed = subprocess.check_output([binary], input=payload, text=True).splitlines()
assert len(observed) == len(fixture)
db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE general_lengths (maximum REAL, category REAL)")
db.executemany("INSERT INTO general_lengths VALUES (?,?)", [(50,80),(80,71),(100,63),(1e9,56)])
db.execute("CREATE TABLE fixed (id TEXT PRIMARY KEY, category REAL)")
db.executemany("INSERT INTO fixed VALUES (?,?)", [("sloped",71),("gusset-reinforced",80),("gusset-as-welded",40),("stud-base",80)])
expression = """
WITH classification AS (
 SELECT category FROM general_lengths WHERE :weld84_detail = 'longitudinal'
 AND :weld84_length BETWEEN 0 AND maximum AND :weld84_length > 0
 AND :weld84_thickness > 0 AND :weld84_thickness < :weld84_height
 UNION ALL
 SELECT category FROM general_lengths
 WHERE :weld84_detail IN ('transverse','beam-stiffener','web-stiffener','box-diaphragm')
 AND maximum <= 80 AND :weld84_length > 0 AND maximum >= :weld84_length
 UNION ALL
 SELECT category FROM fixed WHERE id = :weld84_detail AND (
 (id = 'sloped' AND :weld84_length > 100 AND :weld84_angle >= 0 AND :weld84_angle < 45)
 OR (id = 'gusset-reinforced' AND :weld84_radius > 150 AND :weld84_reinforced_length > :weld84_radius)
 OR id IN ('gusset-as-welded','stud-base'))
 UNION ALL
 SELECT CASE WHEN :weld84_radius > 150 OR 3 * :weld84_radius >= :weld84_length THEN 90
             WHEN 6 * :weld84_radius >= :weld84_length THEN 71 ELSE 50 END
 WHERE :weld84_detail = 'gusset-edge' AND :weld84_length > 0 AND :weld84_radius > 0
)
SELECT :weld84_range * :gamma_ff, max(category) / :gamma_mf, :weld84_frequent, 1.5 * :fy
FROM classification
WHERE :weld84_basis = 'confirmed'
HAVING max(category) IS NOT NULL
"""
for row, rust, (name, _, _, expected) in zip(params, observed, fixture):
    oracle = db.execute(expression, row).fetchone()
    if not expected:
        assert oracle is None and rust == "invalid", (name, oracle, rust)
        continue
    want = [amount for a,b,_ in expected for amount in (a,b)]
    actual = [float(v) for pair in rust.split(";") for v in pair.split(",")]
    for result in [oracle,actual]:
        assert result is not None and len(result) == len(want), (name,result,want)
        assert all(math.isclose(a,b,rel_tol=1e-12) for a,b in zip(result,want)), (name,result,want)
        assert ["pass" if a<=b else "fail" for a,b in zip(result[::2],result[1::2])] == [r[2] for r in expected], name
print(f"[DEBUG] EN 1993-1-9 Table 8.4 {len(fixture)} scenarios matched production Rust and independent SQLite.")
