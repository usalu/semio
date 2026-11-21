"""🧪 Compare Table 8.5 weld classifications and verification ranges with SQLite."""
import json
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
root = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
family = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
catalogue = next(family.glob("🏅*/🔖*/🪆*/✳*/🧬*/💡*/⚖*/📇*catalogue"))
fixture = json.loads((catalogue / "🧪️tests/table-8-5.json").read_text())["cases"]
vocabulary = json.loads((catalogue / "📚️inputs/🔣️.json").read_text())["inputs"]
defaults = {row["key"]: row["example"] for row in vocabulary if row.get("scope") == "C93-27" or not row.get("scope") and row["key"] in {"gamma_ff", "gamma_mf", "fy"}}
evaluator = catalogue / "🧮️evaluators"
shared = (evaluator / "🦀️.rs").read_text()
model = shared[shared.index("pub struct Row {"):shared.index("/// 🟦️ EN 1993-1-7")]
direct = shared[shared.index("fn nominal_direct_checks("):shared.index("/// 🚉 EN 1993-1-9 Table 8.8")]
generated = ticket / "🗑️generated"
binary = generated / "en1993-table-8-5-probe"
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
""" + model + direct + f'\n#[path = "{evaluator / "🪢️load-carrying-weld/🦀️.rs"}"]\nmod weld;\n' + r"""
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
        match weld::load_carrying_weld_fatigue(&Inputs(&case)) {
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
db.execute("CREATE TABLE toe (length_max REAL, thickness_max REAL, category REAL)")
db.executemany("INSERT INTO toe VALUES (?,?,?)", [
    (50, 1e9, 80), (80, 1e9, 71), (100, 1e9, 63), (120, 1e9, 56),
    (1e9, 20, 56), (200, 1e9, 50), (1e9, 30, 50), (300, 1e9, 45),
    (1e9, 50, 45), (1e9, 1e9, 40),
])
db.execute("CREATE TABLE cover (max_t REAL, thinner REAL, at_least_as_thick REAL)")
db.executemany("INSERT INTO cover VALUES (?,?,?)", [(20,56,50),(30,50,45),(50,45,40),(1e9,40,36)])
classification = """
WITH dimension_class AS (
    SELECT max(category) AS category FROM toe
    WHERE :weld85_length > 0 AND :weld85_length <> 50 AND :weld85_length <= length_max
      AND :weld85_plate > 0 AND :weld85_plate <= thickness_max
), cover_class AS (
    SELECT IIF(:weld85_cover < :weld85_plate, thinner, at_least_as_thick) AS category
    FROM cover WHERE :weld85_plate > 0 AND :weld85_plate <= max_t AND :weld85_cover > 0
    ORDER BY max_t LIMIT 1
)
SELECT CASE :weld85_detail
  WHEN 'toe' THEN (SELECT category FROM dimension_class)
  WHEN 'modified-toe' THEN (SELECT category FROM dimension_class)
  WHEN 'root' THEN (SELECT category FROM dimension_class)
  WHEN 'lap-main' THEN (SELECT category FROM dimension_class)
  WHEN 'lap-cover' THEN 45
  WHEN 'cover-end' THEN IIF(:weld85_length < 300, (SELECT category FROM dimension_class), (SELECT category FROM cover_class))
  WHEN 'cover-reinforced' THEN 56
  WHEN 'continuous-shear' THEN 0
  WHEN 'lap-shear' THEN 0
  WHEN 'ring-butt' THEN 71
  WHEN 'ring-fillet' THEN 40
END AS category
WHERE :weld85_basis = 'confirmed'
  AND (:weld85_detail NOT IN ('toe','modified-toe','root') OR
       (:weld85_intermediate > 0 AND 100 * :weld85_misalignment BETWEEN 0 AND 15 * :weld85_intermediate))
  AND (:weld85_detail NOT IN ('lap-main','lap-cover','lap-shear') OR :weld85_end_distance >= 10)
  AND (:weld85_detail <> 'cover-reinforced' OR
       (:weld85_cover > 0 AND :weld85_reinforcing_length >= 5 * :weld85_cover
        AND (:weld85_cover <= 20 OR 4 * :weld85_slope < 1)))
  AND (:weld85_detail <> 'ring-butt' OR :weld85_penetration BETWEEN 0.8 AND 1)
"""
stress_rows = """
SELECT :gamma_ff * :weld85_range, :category / :gamma_mf, :weld85_frequent, 1.5 * :fy,
       :gamma_ff * :weld85_root, 36 / :gamma_mf, :weld85_root_frequent, 1.5 * :fy,
       :gamma_ff * :weld85_shear, 80 / :gamma_mf, :weld85_shear_frequent, 1.5 * :fy / :sqrt3,
       pow(:gamma_ff * :weld85_root * :gamma_mf / 36, 3) + pow(:gamma_ff * :weld85_shear * :gamma_mf / 80, 5), 1
"""
db.create_function("pow", 2, math.pow)
for data, actual, (name, _, _, expected) in zip(params, observed, fixture):
    raw = db.execute(classification, data).fetchone()
    category = raw[0] if raw else None
    if not expected:
        assert category is None and actual == "invalid", (name, category, actual)
        continue
    assert category is not None and actual != "invalid", (name, category, actual)
    detail = data["weld85_detail"]
    values = db.execute(stress_rows, {**data, "category":category, "sqrt3":math.sqrt(3)}).fetchone()
    oracle = list(values[:4]) if category else []
    if detail == "root":
        oracle.extend(values[4:])
    elif detail in {"lap-main", "lap-cover", "continuous-shear", "lap-shear"}:
        oracle.extend(values[8:12])
    rust = [float(v) for pair in actual.split(";") for v in pair.split(",")]
    want = [value for a,b,_ in expected for value in (a,b)]
    for result in [rust, oracle]:
        assert len(result) == len(want) and all(math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12) for a,b in zip(result,want)), (name,result,want)
        statuses = ["pass" if a <= b else "fail" for a,b in zip(result[::2],result[1::2])]
        assert statuses == [row[2] for row in expected], (name,statuses)
print(f"[DEBUG] EN 1993-1-9 Table 8.5 {len(fixture)} scenarios matched production Rust and independent SQLite.")
