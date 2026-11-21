#!/usr/bin/env python3
"""🔩 Compare the EN 1993-1-9 weld toe and lattice cases against SQLite."""
import csv
import math
import sqlite3
import subprocess
from pathlib import Path


artifact = Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993")
schema = next(artifact.glob("*standards/*1/*subsets/*any")) / "🧬️schema"
fixtures = schema / "🧪️tests/⚖️compliance"
evaluators = schema / "💡️inferences/⚖️checks/📇️catalogue/🧮️evaluators/🦀️.rs"
work = Path(__file__).resolve().parent / "🗑️generated"
work.mkdir(exist_ok=True)
implementation = evaluators.read_text()
hot_spot = implementation[implementation.index("const HOT_SPOT_DETAILS"):implementation.index("const RUNWAY_DETAILS")]
lattice = implementation[implementation.index("const CHS_LATTICE_FACTORS"):implementation.index("/// 🪵 EN 1993-5")]
shim = r'''
use std::{collections::HashMap, io::{self, BufRead}};
#[derive(Clone, Copy)]
pub enum QuantityKind { Stress }
pub struct Row { key: &'static str, demand: f64, capacity: f64 }
impl Row {
    pub fn new(key: &'static str, _: &str, _: &str, _: QuantityKind, demand: f64, capacity: f64, _: String) -> Self {
        Self { key, demand, capacity }
    }
}
pub type Outcome = Result<Vec<Row>, String>;
pub struct Inputs<'a>(HashMap<&'a str, &'a str>);
impl Inputs<'_> {
    fn t(&self, key: &str) -> Result<&str, String> { self.0.get(key).copied().ok_or_else(|| key.into()) }
    fn n(&self, key: &str) -> Result<f64, String> { self.t(key)?.parse().map_err(|_| key.into()) }
}
'''
main = r'''
fn main() {
    for input in io::stdin().lock().lines() {
        let input = input.unwrap();
        let (kind, fields) = input.split_once('\t').unwrap();
        let args = Inputs(fields.split('\t').map(|raw| raw.split_once('=').unwrap()).collect());
        let outcome = match kind {
            "hot-spot" => hot_spot_fatigue(&args),
            "lattice" => lattice_joint_fatigue(&args),
            _ => panic!("unknown case"),
        };
        if let Ok(rows) = outcome {
            let entries: Vec<_> = rows.iter().map(|r| format!("{},{:.17e},{:.17e}", r.key, r.demand, r.capacity)).collect();
            println!("{}", entries.join("\t"));
        } else {
            println!("invalid");
        }
    }
}
'''
code = work / "en1993_fatigue_catalogue.rs"
code.write_text(shim + hot_spot + lattice + main)
exe = code.with_suffix(".exe")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc", "--edition=2021", str(code), "-o", str(exe)], check=True)

reference = sqlite3.connect(":memory:")
reference.execute("CREATE TABLE hot_spot (name TEXT PRIMARY KEY, category REAL, angle INTEGER)")
reference.execute("CREATE TABLE lattice (section TEXT, joint TEXT, member TEXT, k REAL, PRIMARY KEY(section,joint,member))")
with (fixtures / "fatigue-hot-spot-classes.tsv").open() as rows:
    hot_spots = list(csv.DictReader(rows, delimiter="\t"))
with (fixtures / "fatigue-lattice-factors.tsv").open() as rows:
    lattices = list(csv.DictReader(rows, delimiter="\t"))
reference.executemany("INSERT INTO hot_spot VALUES(?,?,?)", [(r["detail"], r["category_mpa"], r["angle_restricted"]) for r in hot_spots])
reference.executemany("INSERT INTO lattice VALUES(?,?,?,?)", [(r["section"], r["joint"], member, r[member] or None) for r in lattices for member in ("chord", "vertical", "diagonal")])

cases = []
for row in hot_spots:
    base = dict(hot_spot_detail=row["detail"], hot_spot_crack="toe", hot_spot_basis="confirmed", hot_spot_angle=45, hot_spot_range=65, hot_spot_frequent=110, gamma_ff=1.10, gamma_mf=1.15, fy=355)
    updates = ({}, {"hot_spot_angle": 60}, {"hot_spot_angle": 60.01}, {"hot_spot_crack": "root"}, {"hot_spot_basis": "unconfirmed"}, {"hot_spot_range": 0}, {"hot_spot_range": 125, "hot_spot_frequent": 550})
    for update in updates:
        case = {**base, **update}
        args = (case["hot_spot_range"], case["gamma_ff"], case["gamma_mf"], case["hot_spot_crack"], case["hot_spot_basis"], case["hot_spot_angle"], row["detail"])
        expected = reference.execute("SELECT ? * ?, category / ? FROM hot_spot WHERE ? = 'toe' AND ? = 'confirmed' AND (angle = 0 OR ? BETWEEN 0 AND 60) AND name = ?", args).fetchone()
        rows = [("fatigue", *expected), ("frequent-range", case["hot_spot_frequent"], 1.5 * case["fy"])] if expected else []
        cases.append(("hot-spot", case, rows))

for row in lattices:
    chs = row["section"] == "chs"
    low_beta, low_gamma, high_gamma = (0.30, 12., 30.) if chs else (0.40, 6.25, 12.5)
    for member in ("chord", "vertical", "diagonal"):
        base = dict(lattice_section=row["section"], lattice_joint=row["joint"], lattice_member=member, lattice_basis="confirmed", lattice_beta=0.5, lattice_gamma=10 if not chs else 15, lattice_tau=0.6, lattice_angle=45, lattice_range=40, delta_sigma_c=71, gamma_ff=1.10, gamma_mf=1.15)
        updates = ({}, {"lattice_beta": low_beta, "lattice_gamma": low_gamma, "lattice_tau": 0.25, "lattice_angle": 30}, {"lattice_beta": 0.60, "lattice_gamma": high_gamma, "lattice_tau": 1., "lattice_angle": 60}, {"lattice_beta": low_beta - 0.0001}, {"lattice_gamma": high_gamma + 0.0001}, {"lattice_tau": 0.2499}, {"lattice_angle": 60.01}, {"lattice_basis": "unconfirmed"}, {"lattice_range": 125})
        for update in updates:
            case = {**base, **update}
            args = (case["lattice_range"], case["gamma_ff"], case["delta_sigma_c"], case["gamma_mf"], row["section"], row["joint"], member, case["lattice_basis"], case["lattice_beta"], low_beta, case["lattice_gamma"], low_gamma, high_gamma, case["lattice_tau"], case["lattice_angle"])
            expected = reference.execute("SELECT k * ? * ?, ? / ? FROM lattice WHERE section = ? AND joint = ? AND member = ? AND k IS NOT NULL AND ? = 'confirmed' AND ? BETWEEN ? AND 0.60 AND ? BETWEEN ? AND ? AND ? BETWEEN 0.25 AND 1 AND ? BETWEEN 30 AND 60", args).fetchone()
            cases.append(("lattice", case, [("fatigue", *expected)] if expected else []))

request = "".join(kind + "\t" + "\t".join(f"{k}={v}" for k, v in case.items()) + "\n" for kind, case, _ in cases)
actual = subprocess.run([exe], input=request, text=True, capture_output=True, check=True).stdout.splitlines()
assert len(actual) == len(cases)
for (case_kind, case, expected), result in zip(cases, actual):
    rows = [] if result == "invalid" else [line.split(",") for line in result.split("\t")]
    assert len(rows) == len(expected), (case_kind, case, expected, result)
    for (key, demand, capacity), (expected_key, expected_demand, expected_capacity) in zip(rows, expected):
        assert key == expected_key, (case_kind, case, key, expected_key)
        assert math.isclose(float(demand), expected_demand * 1e6, rel_tol=1e-13), (case_kind, case, key, demand, expected_demand)
        assert math.isclose(float(capacity), expected_capacity * 1e6, rel_tol=1e-13), (case_kind, case, key, capacity, expected_capacity)
print(f"[DEBUG] EN 1993-1-9 weld toe and lattice detail tables: {len(hot_spots) + 3 * len(lattices)} source cells and {len(cases)} evaluator cases matched SQLite")
