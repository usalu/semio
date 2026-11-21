#!/usr/bin/env python3
"""🔩 Compare EN 1993-1-9 fatigue factor and damage code with SQLite."""
import csv
import math
import re
import shutil
import sqlite3
import subprocess
from pathlib import Path


root = Path.cwd()
artifact = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
standard = next(artifact.glob("*standards/*1/*subsets/*any"))
schema = standard / "🧬️schema"
checks = schema / "💡️inferences/⚖️checks"
work = Path(__file__).resolve().parent / "🗑️generated"
work.mkdir(exist_ok=True)
parts = (checks / "📏️parts/🦀️.rs").read_text()
part_1_9 = parts.split("pub mod part_1_9 {", 1)[1].split("\npub mod part_1_11", 1)[0]
curves = (checks / "🧮️clauses/⏳️fatigue-fire/🦀️.rs").read_text().split("/// 🧾 Shear fatigue", 1)[0]
registry = (artifact.parent.parent / "⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table/en1993/🦀️.rs").read_text()
lines = [line for line in registry.splitlines() if '"ndp.en1993.1-9.gamma-mf.' in line or '"ndp.en1993.1-9.sn.' in line]
values = {}
for line in lines:
    key = re.search(r'"(ndp\.en1993\.1-9\.[^"]+)"', line).group(1)
    numbers = re.findall(r"Number\(([-+eE.0-9]+)\)", line)
    assert numbers and len(numbers) <= 2, line
    values[key] = numbers if len(numbers) == 2 else numbers * 2
rust_arms = "\n".join(f'"{key}" => if annex == AnnexChoice::En {{ {en} }} else {{ {de} }},' for key, (en, de) in values.items())
main = r'''
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let cells: Vec<_> = line.split('\t').collect();
        let method = match cells[0] {
            "damage-tolerant" => FatigueMethod::DamageTolerant,
            "safe-life" => FatigueMethod::SafeLife,
            _ => panic!("bad method"),
        };
        let consequence = match cells[1] {
            "low" => FatigueConsequence::Low,
            "high" => FatigueConsequence::High,
            _ => panic!("bad consequence"),
        };
        let annex = if cells[2] == "en" { AnnexChoice::En } else { AnnexChoice::De };
        let gamma = part_1_9::gamma_mf(method, consequence, annex);
        let range = cells[3].parse::<f64>().unwrap();
        let cycles = cells[4].parse::<f64>().unwrap();
        let damage = part_1_9::miner_damage(&[(range * 1e6, cycles)], 100e6, gamma);
        println!("{}\t{gamma:.17e}\t{damage:.17e}", part_1_9::gamma_mf_id(method, consequence));
    }
}
'''
code = '''
extern crate self as semio_s_artifact_norm_en1993_model;
extern crate self as semio_s_plugin_norm_compliance;
#[derive(Clone, Copy)]
pub enum FatigueMethod { DamageTolerant, SafeLife }
#[derive(Clone, Copy)]
pub enum FatigueConsequence { Low, High }
#[derive(Clone, Copy, PartialEq)]
pub enum AnnexChoice { En, De }
pub fn ndp_number(key: &str, annex: AnnexChoice) -> f64 {
    match key {
REPLACE_ARMS
        _ => panic!("unknown registry key: {}", key),
    }
}
pub fn ndp_common(key: &str) -> f64 {
    let en = ndp_number(key, AnnexChoice::En);
    assert_eq!(en, ndp_number(key, AnnexChoice::De));
    en
}
''' .replace("REPLACE_ARMS", rust_arms)
source = work / "en1993_fatigue_factors.rs"
source.write_text(code + "\nmod clauses { pub mod fatigue_fire {\n" + curves.replace(curves.splitlines()[0], "", 1) + "\n}}\npub mod part_1_9 {" + part_1_9 + main)
exe = source.with_suffix(".exe")
subprocess.run([shutil.which("rustc"), "--edition=2021", str(source), "-o", str(exe)], check=True)
with (schema / "🧪️tests/⚖️compliance/fatigue-resistance-factors.tsv").open() as fixture:
    factors = list(csv.DictReader(fixture, delimiter="\t"))
cases = [(f["method"], f["consequence"], annex, stress, cycles, float(f[annex]))
         for f in factors for annex in ("en", "de") for stress in (0., 30., 35., 42., 54., 63., 75., 95., 100., 150.)
         for cycles in (1., 1e5, 2e6, 5e6)]
request = "".join("\t".join(map(str, case[:5])) + "\n" for case in cases)
actual = subprocess.run([exe], input=request, text=True, capture_output=True, check=True).stdout.splitlines()
assert len(actual) == len(cases)
reference = sqlite3.connect(":memory:")
reference.create_function("pow", 2, math.pow, deterministic=True)
query = """WITH detail AS (SELECT ? * ? AS stress, ? AS cycles),
limits AS (SELECT 100 * pow(0.4, 1.0 / 3) AS constant_amplitude)
SELECT CASE WHEN stress > constant_amplitude THEN cycles / (2e6 * pow(100 / stress, 3))
            WHEN stress > constant_amplitude * pow(0.05, 0.2) THEN cycles / (5e6 * pow(constant_amplitude / stress, 5))
            ELSE 0 END
FROM detail CROSS JOIN limits"""
for (method, consequence, annex, stress, cycles, gamma), row in zip(cases, actual):
    key, got_gamma, got_damage = row.split("\t")
    expected_damage = reference.execute(query, (stress, gamma, cycles)).fetchone()[0]
    assert key == f"ndp.en1993.1-9.gamma-mf.{method}.{consequence}", key
    assert gamma == float(got_gamma), row
    assert math.isclose(expected_damage, float(got_damage), rel_tol=1e-13, abs_tol=1e-14), (row, expected_damage)
print(f"[DEBUG] EN 1993-1-9 Table 3.1 and Annex A: {len(cases)} production Rust cases matched SQLite")
