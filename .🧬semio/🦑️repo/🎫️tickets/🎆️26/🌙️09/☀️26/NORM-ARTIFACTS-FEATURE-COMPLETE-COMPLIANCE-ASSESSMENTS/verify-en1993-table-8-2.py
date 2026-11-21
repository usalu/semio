"""🧪 Compare Table 8.2 longitudinal weld classifications and verification ranges with SQLite."""
import json
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
root = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
family = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
catalogue = next(family.glob("🏅*/🔖*/🪆*/✳*/🧬*/💡*/⚖*/📇*catalogue"))
fixture = json.loads((catalogue / "🧪️tests/table-8-2.json").read_text())["cases"]
vocabulary = json.loads((catalogue / "📚️inputs/🔣️.json").read_text())["inputs"]
defaults = {row["key"]: row["example"] for row in vocabulary if row.get("scope") == "C93-30" or not row.get("scope") and row["key"] in {"gamma_ff", "gamma_mf", "fy"}}
evaluator = catalogue / "🧮️evaluators"
shared = (evaluator / "🦀️.rs").read_text()
model = shared[shared.index("pub struct Row {"):shared.index("/// 🟦️ EN 1993-1-7")]
direct = shared[shared.index("fn nominal_direct_checks("):shared.index("/// 🚉 EN 1993-1-9 Table 8.8")]
generated = ticket / "🗑️generated"
binary = generated / "en1993-table-8-2-probe"
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
""" + model + direct + f'\n#[path = "{evaluator / "🧱️attachment-fatigue/🦀️.rs"}"]\nmod attachment_fatigue;\n#[path = "{evaluator / "🛠️built-up-weld/🦀️.rs"}"]\nmod weld;\n' + r"""
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
        match weld::built_up_weld_fatigue(&Inputs(&case)) {
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
db.execute("CREATE TABLE base_lengths (maximum REAL, category REAL)")
db.executemany("INSERT INTO base_lengths VALUES (?,?)", [(50,80),(80,71),(100,63),(1e9,56)])
query = """
WITH selected AS (
 SELECT CASE
   WHEN :weld82_detail IN ('automatic-butt','automatic-fillet') AND :weld82_stop<>'ordinary' THEN 125
   WHEN :weld82_detail='two-sided-stops' AND :weld82_stop<>'none' THEN 112
   WHEN :weld82_detail='single-backed' THEN IIF(:weld82_stop='none',112,100)
   WHEN :weld82_detail IN ('manual','single-sided') THEN 100
   WHEN :weld82_detail='repair' THEN CASE :weld82_repair WHEN 'restored-125' THEN 125 WHEN 'restored-112' THEN 112 ELSE 100 END
   WHEN :weld82_detail='intermittent' AND :weld82_gap>0 AND 2*:weld82_gap<=5*:weld82_web_height THEN 80
   WHEN :weld82_detail='cope-hole' AND (:weld82_cope_height<60 OR (:weld82_cope_height=60 AND :weld82_cope_basis='english')) THEN 71
   WHEN :weld82_detail='cope-hole' AND :weld82_cope_height>60
     AND :weld82_attachment_thickness<:weld82_attachment_height
       THEN (SELECT max(category) FROM base_lengths WHERE :weld82_attachment_length<=maximum)
   WHEN :weld82_detail='longitudinal-butt' THEN IIF(:weld82_finish='ground-ndt',125,IIF(:weld82_stop='none',112,90))
   WHEN :weld82_detail='hollow-seam' AND :weld82_wall>0 THEN IIF(:weld82_stop='none',IIF(:weld82_wall<=12.5,140,125),90)
 END AS category
 WHERE :weld82_basis='confirmed' AND (:weld82_detail<>'automatic-fillet' OR :weld82_ends<>'pending')
)
SELECT :weld82_range*:gamma_ff, category/:gamma_mf, :weld82_frequent, 1.5*:fy,
       :weld82_ends_utilization, 1
FROM selected WHERE category IS NOT NULL
"""
for row,rust,(name,_,_,expected) in zip(params,observed,fixture):
    oracle = db.execute(query,row).fetchone()
    if not expected:
        assert oracle is None and rust == "invalid", (name,oracle,rust)
        continue
    assert oracle is not None, name
    if row["weld82_detail"] != "automatic-fillet" or row["weld82_ends"] != "assessed":
        oracle = oracle[:4]
    want = [v for a,b,_ in expected for v in (a,b)]
    actual = [float(v) for pair in rust.split(";") for v in pair.split(",")]
    for result in [oracle,actual]:
        assert len(result)==len(want) and all(math.isclose(a,b,rel_tol=1e-12) for a,b in zip(result,want)), (name,result,want)
        assert ["pass" if a<=b else "fail" for a,b in zip(result[::2],result[1::2])] == [r[2] for r in expected], name
print(f"[DEBUG] EN 1993-1-9 Table 8.2 {len(fixture)} scenarios matched production Rust and independent SQLite.")
