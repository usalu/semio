"""🧪 Compare Table 8.3 butt-weld classifications and verification ranges with SQLite."""
import json
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
root = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
family = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
catalogue = next(family.glob("🏅*/🔖*/🪆*/✳*/🧬*/💡*/⚖*/📇*catalogue"))
fixture = json.loads((catalogue / "🧪️tests/table-8-3.json").read_text())["cases"]
vocabulary = json.loads((catalogue / "📚️inputs/🔣️.json").read_text())["inputs"]
defaults = {row["key"]: row["example"] for row in vocabulary if row.get("scope") == "C93-29" or not row.get("scope") and row["key"] in {"gamma_ff", "gamma_mf", "fy"}}
evaluator = catalogue / "🧮️evaluators"
shared = (evaluator / "🦀️.rs").read_text()
model = shared[shared.index("pub struct Row {"):shared.index("/// 🟦️ EN 1993-1-7")]
direct = shared[shared.index("fn nominal_direct_checks("):shared.index("/// 🚉 EN 1993-1-9 Table 8.8")]
generated = ticket / "🗑️generated"
binary = generated / "en1993-table-8-3-probe"
source = binary.with_suffix(".rs")
source.write_text(r"""
#![allow(dead_code)]
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
""" + model + direct + f'\n#[path = "{evaluator / "🪢️load-carrying-weld/🦀️.rs"}"]\nmod load_carrying_weld;\n#[path = "{evaluator / "🧱️attachment-fatigue/🦀️.rs"}"]\nmod attachment_fatigue;\n#[path = "{evaluator / "🪡️transverse-butt-weld/🦀️.rs"}"]\nmod weld;\n' + r"""
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
        match weld::transverse_butt_weld_fatigue(&Inputs(&case)) {
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
db.create_function("pow", 2, math.pow)
db.execute("CREATE TABLE weld (id TEXT, category REAL, thickness INTEGER, slope REAL, convexity REAL)")
db.executemany("INSERT INTO weld VALUES (?,?,?,?,?)", [
    ("ground-plate",112,1,None,None), ("ground-built-up",112,1,None,None), ("ground-rolled",112,1,None,None),
    ("ground-tapered",112,1,.25,None), ("profile-plate",90,1,None,.1), ("profile-rolled",90,1,None,.1),
    ("profile-tapered",90,1,.25,.1), ("ground-cope",90,1,None,None), ("as-welded-built-up",80,1,None,.2),
    ("as-welded-cope",80,1,None,.1), ("as-welded-plate",80,1,None,.2), ("unbacked-rolled",63,0,None,None),
    ("single-sided",36,0,None,None), ("single-sided-inspected",71,1,None,None),
    ("backed",71,1,None,None), ("backed-tapered",71,1,.25,None), ("backed-lower-class",50,1,.25,None),
    ("unequal-section",71,1,.5,None), ("intersecting",40,0,None,None), ("intersecting-rounded",None,0,None,None),
])
db.execute("CREATE TABLE base_toe (length_max REAL, thick_max REAL, category REAL)")
db.executemany("INSERT INTO base_toe VALUES (?,?,?)", [
    (50,1e9,80),(80,1e9,71),(100,1e9,63),(120,1e9,56),(1e9,20,56),
    (200,1e9,50),(1e9,30,50),(300,1e9,45),(1e9,50,45),(1e9,1e9,40)])
query = """
WITH radius_class AS (
 SELECT CASE WHEN :weld83_radius>150 OR 3*:weld83_radius>=:weld83_length THEN 90
             WHEN 6*:weld83_radius>=:weld83_length THEN 71 ELSE 50 END AS category
 WHERE :weld83_radius>0 AND :weld83_length>0
), candidates AS (
 SELECT id,
  COALESCE(category, CASE :weld83_cross_reference
   WHEN 'table84' THEN (SELECT category FROM radius_class)
   WHEN 'table85' THEN (SELECT max(category) FROM base_toe WHERE :weld83_length > 0 AND :weld83_length<>50
                        AND :weld83_length<=length_max AND :weld83_thickness<=thick_max)
  END) AS category,
  IIF(thickness=1,pow(25.0/max(25.0,:weld83_thickness),0.2),1) AS thickness_factor,
  IIF(id='unequal-section',1+6.0*:weld83_offset/:weld83_thickness*pow(:weld83_thickness,1.5)/(pow(:weld83_thickness,1.5)+pow(:weld83_thicker,1.5)),1) AS eccentricity,
  CASE :weld83_flange WHEN 'as-welded' THEN 40 WHEN 'rounded' THEN (SELECT category FROM radius_class) END AS orthogonal
 FROM weld WHERE id=:weld83_detail AND :weld83_basis='confirmed' AND :weld83_thickness>0
  AND (slope IS NULL OR :weld83_slope BETWEEN 0 AND slope)
  AND (convexity IS NULL OR (:weld83_weld_width>0 AND :weld83_reinforcement BETWEEN 0 AND convexity*:weld83_weld_width))
  AND (id NOT IN ('backed','backed-tapered') OR (:weld83_fit='good' AND :weld83_end_distance>10))
  AND (id <> 'backed-lower-class' OR (:weld83_fit='uncertain' OR :weld83_end_distance<=10))
  AND (id <> 'unequal-section' OR (:weld83_thicker>=:weld83_thickness AND :weld83_offset>=0))
)
SELECT :gamma_ff*:weld83_range, category*thickness_factor/(:gamma_mf*eccentricity), :weld83_frequent, 1.5*:fy,
       :gamma_ff*:weld83_perpendicular_range, orthogonal/:gamma_mf, :weld83_perpendicular_frequent, 1.5*:fy
FROM candidates WHERE category IS NOT NULL
"""
for row, rust, (name, _, _, expected) in zip(params, observed, fixture):
    oracle = db.execute(query, row).fetchone()
    if not expected:
        assert oracle is None and rust == "invalid", (name, oracle, rust)
        continue
    assert oracle is not None, name
    if row["weld83_detail"] not in {"intersecting", "intersecting-rounded"}:
        oracle = oracle[:4]
    want = [amount for a,b,_ in expected for amount in (a,b)]
    actual = [float(v) for pair in rust.split(";") for v in pair.split(",")]
    for result in [oracle,actual]:
        assert len(result) == len(want) and all(math.isclose(a,b,rel_tol=1e-12) for a,b in zip(result,want)), (name,result,want)
        assert ["pass" if a<=b else "fail" for a,b in zip(result[::2],result[1::2])] == [r[2] for r in expected], name
print(f"[DEBUG] EN 1993-1-9 Table 8.3 {len(fixture)} scenarios matched production Rust and independent SQLite.")
