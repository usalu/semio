from csv import reader
from pathlib import Path
import re
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
artifact = next((root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998").glob("🏅*/🔖*/🪆*/✳*"))
fixture = artifact / "🧪️tests/🧫️fixtures/🛤️fault-crossing.tsv"
with fixture.open() as stream:
    records = reader(stream, delimiter="\t")
    fields = next(records)[1:-1]
    cases = list(records)

query = """
WITH crossing(basis, soil, wa, wb, ba, bb, ta, tb) AS (VALUES (?, ?, ?, ?, ?, ?, ?, ?)),
sides(position, distance, rule) AS (
  SELECT 1, wa, 50 FROM crossing UNION ALL SELECT 2, wb, 50 FROM crossing
  UNION ALL SELECT 3, ba, 50 FROM crossing UNION ALL SELECT 4, bb, 50 FROM crossing
  UNION ALL SELECT 5, ta, NULL FROM crossing WHERE soil = 'different'
  UNION ALL SELECT 6, tb, NULL FROM crossing WHERE soil = 'different'
)
SELECT CASE WHEN basis <> 'confirmed' THEN 'U'
  ELSE (SELECT group_concat(flag, '') FROM (
    SELECT CASE
      WHEN distance IS NULL OR distance < 0 OR distance >= 1e999 THEN 'U'
      WHEN rule IS NULL THEN 'I'
      WHEN distance < rule THEN '0' ELSE '1' END AS flag
    FROM sides ORDER BY position
  )) || CASE WHEN soil IN ('different', 'suitable') THEN '' ELSE 'U' END
END FROM crossing
"""
with sqlite3.connect(":memory:") as database:
    for name, basis, soil, *values, expected in cases:
        distances = [float(value) if value else None for value in values]
        assert database.execute(query, [basis, soil, *distances]).fetchone()[0] == expected, name

evaluator = (artifact / "🧬️schema/📐️formulas/📇️catalogue/🧮️evaluators/🦀️.rs").read_text()
production = []
for name in ("pipe_basis", "pipe_fault_crossing"):
    implementation = re.search(rf"(?:pub )?fn {name}\(.*?^}}", evaluator, re.S | re.M)
    assert implementation, f"Missing production evaluator {name}"
    production.append(implementation[0])
row_model = evaluator.split("pub struct Row {", 1)[1].split("/// 🧾", 1)[0]
directory = ticket / "🗑️generated/en1998-fault-crossing"
directory.mkdir(parents=True, exist_ok=True)
source = directory / "assess.rs"
binary = directory / "assess"
source.write_text(r'''
use std::{collections::BTreeMap, io::{self, BufRead}};
pub enum QuantityKind { Dimensionless, Length }
pub enum CheckStatus { UnverifiedBasis, Info }
pub struct Inputs<'a>(BTreeMap<&'a str, &'a str>);
impl Inputs<'_> {
    fn t(&self, key: &str) -> Result<&str, String> { self.0.get(key).copied().ok_or_else(||key.to_string()) }
    fn n(&self, key: &str) -> Result<f64, String> { self.t(key)?.parse().map_err(|_|key.to_string()) }
}
pub type Outcome = Result<Vec<Row>, String>;
pub struct Row {''' + row_model + "\n".join(production) + r'''
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields = line.split('|').map(|token| token.split_once('=').unwrap()).collect();
        let rows = pipe_fault_crossing(&Inputs(fields)).unwrap();
        for row in rows {
            let status = match row.status {
                Some(CheckStatus::UnverifiedBasis) => 'U',
                Some(CheckStatus::Info) => 'I',
                None if row.demand <= row.capacity => '1',
                None => '0',
            };
            print!("{status}");
        }
        println!();
    }
}
''')
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=root)
submitted = "".join("|".join(f"{key}={value}" for key, value in zip(fields, case[1:-1])) + "\n"
                    for case in cases)
actual = subprocess.check_output([binary], input=submitted, text=True).splitlines()
assert actual == [case[-1] for case in cases], actual
print(f"[DEBUG] EN 1998-4 fault-crossing evaluator and SQLite matched {len(cases)} cases.")
