"""Validate the published Table 8.7 joint details and production catalogue outputs with SQLite."""
import csv
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
fixture = json.loads((catalogue / "🧪️tests/table-8-7.json").read_text())["cases"]
evaluators = catalogue / "🧮️evaluators"
implementation = (evaluators / "🦀️.rs").read_text()
nominal = implementation[implementation.index("fn nominal_direct_checks("):implementation.index("/// 🚉 EN 1993-1-9 Table 8.8")]
lattice = implementation[implementation.index("const CHS_LATTICE_FACTORS"):implementation.index("/// 🔲 EN 1993-1-9:2005 with AC:2009, Tables 4.1")]
paths = ticket / "🗑️generated"
paths.mkdir(exist_ok=True)
binary = paths / "en1993-table-8-7-probe"
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
""" + nominal + lattice + f'\n#[path = "{evaluators / "🧱️lattice-node/🦀️.rs"}"]\nmod node;\n' + r"""
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let args = Inputs(line.split('\t').map(|field| field.split_once('=').unwrap()).collect());
        match node::lattice_node_fatigue(&args) {
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
db.execute("CREATE TABLE fatigue_detail (connection TEXT PRIMARY KEY, low REAL, high REAL, upper_ratio REAL)")
db.executemany("INSERT INTO fatigue_detail VALUES (?, ?, ?, ?)", (
    ("chs-gap",45,90,2), ("rhs-gap",36,71,2), ("overlap-k",56,71,1.4), ("overlap-n",50,71,1.4),
))
db.execute("CREATE TABLE factors (section TEXT, joint TEXT, member TEXT, k REAL)")
with (schema / "🧪️tests/⚖️compliance/fatigue-lattice-factors.tsv").open() as file:
    db.executemany("INSERT INTO factors VALUES (?,?,?,?)", [
        (row["section"], row["joint"].replace("-kt",""), member, row[member] or None)
        for row in csv.DictReader(file, delimiter="\t") for member in ("chord","diagonal","vertical")
    ])
query = """
WITH geo AS (
 SELECT :node_t0 * 1.0 / :node_ti AS ratio,
        :node_brace_width * 1.0 / :node_chord_width AS beta,
        CASE :node_section WHEN 'rhs' THEN :node_chord_height ELSE :node_chord_width END AS depth,
        :node_chord_width - :node_brace_width AS diff
 WHERE :node_section IN ('chs','rhs') AND :node_t0 BETWEEN 0.00001 AND 8
 AND :node_ti BETWEEN 0.00001 AND 8 AND :node_t0 >= :node_ti
 AND :node_chord_width > 0 AND :node_chord_width <= CASE :node_section WHEN 'rhs' THEN 200 ELSE 300 END
 AND :node_chord_width <= 25 * :node_ti AND :node_angle BETWEEN 35 AND 50
 AND abs(:node_eop) <= :node_chord_width * 0.02
),
classification AS (
 SELECT low + (high-low) * min(1, (ratio - 1) / (upper_ratio - 1)) AS category
 FROM fatigue_detail, geo
 WHERE connection = CASE WHEN :node_joint LIKE 'gap-%' THEN :node_section || '-gap' ELSE :node_joint END
   AND :node_geometry = 'confirmed'
   AND (:node_member IN ('chord','diagonal') OR (:node_member='vertical' AND :node_joint LIKE '%-n'))
   AND beta BETWEEN CASE :node_section WHEN 'rhs' THEN 0.4 ELSE 0.25 END AND 1
   AND depth > 0 AND :node_eip BETWEEN -depth/2 AND depth/4
   AND (CASE WHEN :node_joint LIKE 'gap-%' THEN
           :node_gap > 0 AND (:node_section='chs' OR (
             :node_gap >= 2*:node_t0 AND :node_gap BETWEEN diff/2 AND diff*1.1))
         ELSE :node_overlap BETWEEN 0.3 AND 1 END)
),
magnification AS (
 SELECT 1.0 AS factor WHERE :node_stress_basis = 'complete'
 UNION ALL
 SELECT k FROM factors,geo
 WHERE :node_stress_basis = 'pinned' AND section=:node_section AND joint=:node_joint AND member=:node_member
   AND k IS NOT NULL AND ratio<=4
   AND :node_beta BETWEEN CASE section WHEN 'rhs' THEN 0.4 ELSE 0.3 END AND 0.6
   AND :node_gamma BETWEEN CASE section WHEN 'rhs' THEN 6.25 ELSE 12 END AND CASE section WHEN 'rhs' THEN 12.5 ELSE 30 END
)
SELECT :node_equiv_range * :gamma_ff * factor, category / :gamma_mf, :node_frequent_range, 1.5 * :fy
FROM classification, magnification
WHERE :node_equiv_range >= 0 AND :node_frequent_range >= 0 AND :gamma_ff > 0 AND :gamma_mf > 0 AND :fy > 0
"""
for input_row, observed, (name, *_, expected) in zip(params, rust, fixture):
    result = db.execute(query, input_row).fetchone()
    if not expected:
        assert result is None and observed == "invalid", (name, observed, result)
        continue
    assert result is not None, name
    oracle = list(result)
    wanted = [amount for demand, capacity, _ in expected for amount in (demand,capacity)]
    numbers = [float(value) for value in re.split("[;,]", observed)]
    assert all(math.isclose(x, y, rel_tol=1e-12) for x,y in zip(oracle,wanted)), (name, result, expected)
    assert len(numbers) == len(wanted) and all(math.isclose(x, y, rel_tol=1e-12) for x,y in zip(numbers,wanted)), (name, observed, expected)
    assert ["pass" if numbers[i] <= numbers[i+1] else "fail" for i in range(0,len(numbers),2)] == [row[2] for row in expected], name
print(f"[DEBUG] EN 1993-1-9 Table 8.7 {len(fixture)} cases matched production Rust and SQLite.")
