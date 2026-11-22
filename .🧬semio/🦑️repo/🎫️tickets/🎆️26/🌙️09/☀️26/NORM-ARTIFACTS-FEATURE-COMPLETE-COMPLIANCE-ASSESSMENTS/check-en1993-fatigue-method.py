"""🕵️ Runs the production fatigue method selector and an independent SQLite projection."""
from pathlib import Path
import csv
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
g = ticket/"🗑️generated"
g.mkdir(exist_ok=True)
a = next(Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1993"))
src = a/"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🧮️evaluators/🕵️fatigue-method/🦀️.rs"
s = src.read_text().split("pub fn evaluate", 1)[0].replace("use super::{Inputs, Outcome, Row};", "")
program = g/"en1993_fatigue_method.rs"
program.write_text(s + '''
use std::io::{BufRead, stdin};
fn main() {
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let row: Vec<_> = line.split('\\t').collect();
        println!("{:?}", selection(row[0], row[1], row[2], row[3]));
    }
}
''')
binary = g/"en1993_fatigue_method"
subprocess.run(["rustc", "--edition=2024", str(program), "-o", str(binary)], check=True)
vectors = (ticket/"en1993-fatigue-method-cases.tsv").read_text().splitlines()[1:]
output = subprocess.run([str(binary)], check=True, input="\n".join(vectors), capture_output=True, text=True).stdout.splitlines()
database = sqlite3.connect(":memory:")
q = """
WITH inputs(method, rapid, maintenance, basis) AS (VALUES(?,?,?,?))
SELECT CASE
    WHEN method = 'safeLife' THEN 'Pass'
    WHEN method <> 'damageTolerant' THEN 'Unverified'
    WHEN rapid = 'yes' THEN 'SafeLifeRequired'
    WHEN rapid <> 'no' THEN 'Unverified'
    WHEN maintenance = 'no' THEN 'MaintenanceRequired'
    WHEN maintenance = 'yes' AND basis IN ('redistribution', 'steel') THEN 'Pass'
    ELSE 'Unverified' END FROM inputs
"""
assert len(output) == len(vectors)
for row, actual in zip(csv.reader(vectors, delimiter="\t"), output):
    *inputs, expected = row
    oracle, = database.execute(q, inputs).fetchone()
    assert expected == actual == oracle, (inputs, actual, oracle, expected)
print(f"[DEBUG] EN 1993 fatigue method Rust and SQLite agreed on {len(vectors)} cases")
