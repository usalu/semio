from csv import DictReader
import json
import math
from pathlib import Path
import re
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
artifact = next((root/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998").glob("🏅*/🔖*/🪆*/✳*"))
formulas = artifact/"🧬️schema/📐️formulas"
evaluator = (formulas/"📇️catalogue/🧮️evaluators/🦀️.rs").read_text()
production = "\n".join(re.search(pattern, evaluator, re.S | re.M)[0] for pattern in (r"fn pipe_basis\(.*?^}", r"pub fn pipe_deformation\(.*?^}"))
row_model = evaluator.split("pub struct Row {", 1)[1].split("/// 🧾", 1)[0]
fixture = artifact/"🧪️tests/🧫️fixtures/📏️differential-pipe/🔣️.tsv"
with fixture.open() as stream:
    rows = list(DictReader(stream, delimiter="\t"))

directory = ticket/"🗑️generated/en1998-case19"
source = directory/"assess.rs"
binary = directory/"assess"
limits = formulas/"📏️pipe-strain/🦀️.rs"
directory.mkdir(exist_ok=True)
source.write_text(f'#[path = {json.dumps(str(limits), ensure_ascii=False)}] mod limits;\n' + r'''
mod part_4 { pub use crate::limits::welded_pipe_strain_limits; }
use std::io::{self, BufRead};
#[derive(Debug)]
pub enum QuantityKind { Dimensionless }
#[derive(Debug)]
pub enum CheckStatus { UnverifiedBasis }
pub struct Inputs<'a>(&'a str, [Option<f64>; 5]);
impl Inputs<'_> {
    fn t(&self, key: &str) -> Result<&str, String> {
        assert_eq!(key, "pipe_deformation_basis");
        Ok(self.0)
    }
    fn n(&self, key: &str) -> Result<f64, String> {
        let index = ["radius", "thickness", "tension", "compression", "global"].iter()
            .position(|suffix| key.strip_prefix("pipe_deformation_") == Some(*suffix)).unwrap();
        self.1[index].ok_or_else(|| format!("Missing {key}"))
    }
}
pub type Outcome = Result<Vec<Row>, String>;
pub struct Row {''' + row_model + production + r'''
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut values = line.split('|');
        let basis = values.next().unwrap();
        let numbers = std::array::from_fn(|_| values.next().unwrap().parse::<f64>().ok());
        let rows = pipe_deformation(&Inputs(basis, numbers)).unwrap();
        if rows[0].status.is_some() {
            println!("U");
            continue;
        }
        assert_eq!(rows.iter().map(|row| row.key).collect::<Vec<_>>(), ["tension", "local-buckling", "global-buckling"]);
        let status: String = rows.iter().map(|row| if row.demand <= row.capacity { '1' } else { '0' }).collect();
        println!("{}|{:.17}", status, rows[1].capacity);
    }
}
''')
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=root)
keys = ["basis", "radius", "thickness", "tension", "compression", "global"]
inputs = "".join("|".join(row[key] for key in keys) + "\n" for row in rows)
outputs = subprocess.check_output([binary], input=inputs, text=True).splitlines()
assert len(outputs) == len(rows)
oracle = sqlite3.connect(":memory:")
for row, output in zip(rows, outputs):
    parameters = [row["basis"], *(float(row[key]) if row[key] else None for key in keys[1:])]
    reference = oracle.execute("""
       WITH input(b, r, t, e_t, e_c, buckling) AS (VALUES (?, ?, ?, ?, ?, ?)),
       valid AS (SELECT e_t, e_c, buckling, min(.01, t / (5 * r)) AS e_lim
                  FROM input WHERE b = 'confirmed' AND r > t AND t > 0
                    AND min(e_t, e_c, buckling) >= 0 AND max(r, e_t, e_c, buckling) < 1e999)
       SELECT (e_t <= .03) || (e_c <= e_lim) || (buckling <= 1), e_lim FROM valid
    """, parameters).fetchone()
    actual, _, limit = output.partition("|")
    if reference is None:
        assert actual == row["results"] == "U", row
    else:
        assert actual == row["results"] == reference[0], row
        assert all(math.isclose(float(limit), value, abs_tol=1e-15)
                   for value in (reference[1], float(row["compression_limit"]))), row
print(f"[DEBUG] EN 1998-4 §6.5.3 actual evaluator agrees with SQLite across {len(rows)} vectors.")
