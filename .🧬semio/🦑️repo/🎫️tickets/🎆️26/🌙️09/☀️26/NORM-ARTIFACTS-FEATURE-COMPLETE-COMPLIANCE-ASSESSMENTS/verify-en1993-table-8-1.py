"""🧪 Compare Table 8.1 member and fastener classifications and verification ranges with SQLite."""
import json
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
root = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
family = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
catalogue = next(family.glob("🏅*/🔖*/🪆*/✳*/🧬*/💡*/⚖*/📇*catalogue"))
fixture = json.loads((catalogue / "🧪️tests/table-8-1.json").read_text())["cases"]
vocabulary = json.loads((catalogue / "📚️inputs/🔣️.json").read_text())["inputs"]
defaults = {row["key"]: row["example"] for row in vocabulary if row.get("scope") == "C93-31" or not row.get("scope") and row["key"] in {"gamma_ff", "gamma_mf", "fy"}}
evaluator = catalogue / "🧮️evaluators"
shared = (evaluator / "🦀️.rs").read_text()
model = shared[shared.index("pub struct Row {"):shared.index("/// 🟦️ EN 1993-1-7")]
direct = shared[shared.index("fn nominal_direct_checks("):shared.index("/// 🚉 EN 1993-1-9 Table 8.8")]
generated = ticket / "🗑️generated"
binary = generated / "en1993-table-8-1-probe"
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
""" + model + direct + f'\n#[path = "{evaluator / "🔩️plain-and-fastened-fatigue/🦀️.rs"}"]\nmod weld;\n' + r"""
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut case = DesignCase::default();
        for field in line.split('\t') {
            let (key, value) = field.split_once('=').unwrap();
            if key == "detail81_bolt_grade" {
                case.values.texts.insert(key.into(), value.into());
            } else if let Ok(value) = value.parse() {
                case.values.numbers.insert(key.into(), value);
            } else {
                case.values.texts.insert(key.into(), value.into());
            }
        }
        match weld::plain_and_fastened_fatigue(&Inputs(&case)) {
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
db.execute("CREATE TABLE categories (id TEXT PRIMARY KEY, ordinary REAL, weathering REAL, shear INTEGER, clearance INTEGER)")
db.executemany("INSERT INTO categories VALUES (?,?,?,?,?)", [
    ("rolled-plate",160,140,0,0),("rolled-profile",160,140,0,0),("seamless",160,140,0,0),
    ("cut-machined",140,125,0,0),("cut-dressed",125,112,0,0),("rolled-shear",100,100,1,0),("seamless-shear",100,100,1,0),
    ("double-preloaded",112,112,0,1),("double-fitted",90,90,0,1),("single-preloaded",90,90,0,1),
    ("holes",90,90,0,1),("single-fitted",80,80,0,1),("clearance-no-preload",50,50,0,1),
    ("bolt-tension",50,50,0,0),("bolt-shear",100,100,1,0),
])
query = """
WITH check_data AS (
 SELECT shear, CASE WHEN id='bolt-tension' THEN ordinary*pow(30.0/max(30.0,:detail81_diameter),.25)
                    WHEN :detail81_steel='weathering' THEN weathering ELSE ordinary END AS category,
        IIF(id IN ('rolled-shear','seamless-shear'), :detail81_shear_force*:detail81_first_moment/(:detail81_inertia*:detail81_thickness*1.0), :detail81_range) AS equivalent,
        IIF(id IN ('rolled-shear','seamless-shear'), :detail81_frequent_force*:detail81_first_moment/(:detail81_inertia*:detail81_thickness*1.0), :detail81_frequent) AS frequent
 FROM categories WHERE id=:detail81_detail AND :detail81_basis='confirmed'
   AND (clearance=0 OR (2*min(:detail81_end_parallel,:detail81_edge_transverse)>=3*:detail81_diameter
     AND (:detail81_pitch_parallel=0 OR 2*:detail81_pitch_parallel>=5*:detail81_diameter)
     AND (:detail81_pitch_transverse=0 OR 2*:detail81_pitch_transverse>=5*:detail81_diameter)))
   AND (id<>'clearance-no-preload' OR :detail81_reversal='none')
   AND (id<>'bolt-tension' OR :detail81_diameter>0)
   AND (id NOT IN ('rolled-shear','seamless-shear') OR (:detail81_inertia>0 AND :detail81_thickness>0))
   AND (id<>'bolt-shear' OR (:detail81_threads='outside' AND
        (:detail81_bolt_type='fitted' OR (:detail81_reversal='none' AND :detail81_bolt_grade IN ('5.6','8.8','10.9')))))
)
SELECT equivalent*:gamma_ff, category/:gamma_mf, frequent, 1.5*:fy/IIF(shear=1,:sqrt3,1)
FROM check_data
"""
for row,rust,(name,_,_,expected) in zip(params,observed,fixture):
    oracle=db.execute(query,{**row,"sqrt3":math.sqrt(3)}).fetchone()
    if not expected:
        assert oracle is None and rust=="invalid",(name,oracle,rust)
        continue
    assert oracle is not None and rust != "invalid", (name,oracle,rust)
    want=[v for a,b,_ in expected for v in (a,b)]
    actual=[float(v) for pair in rust.split(";") for v in pair.split(",")]
    for result in (oracle,actual):
        assert len(result)==len(want) and all(math.isclose(a,b,rel_tol=1e-12) for a,b in zip(result,want)),(name,result,want)
        assert ["pass" if a<=b else "fail" for a,b in zip(result[::2],result[1::2])] == [r[2] for r in expected],name
print(f"[DEBUG] EN 1993-1-9 Table 8.1 {len(fixture)} scenarios matched production Rust and independent SQLite.")
