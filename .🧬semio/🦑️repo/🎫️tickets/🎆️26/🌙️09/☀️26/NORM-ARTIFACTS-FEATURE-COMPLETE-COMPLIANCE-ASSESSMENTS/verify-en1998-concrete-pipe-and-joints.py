from csv import reader
from pathlib import Path
import re
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
artifact = next((root/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998").glob("🏅*/🔖*/🪆*/✳*"))
evaluator = (artifact/"🧬️schema/📐️formulas/📇️catalogue/🧮️evaluators/🦀️.rs").read_text()
production = "\n".join(re.search(pattern, evaluator, re.S | re.M)[0] for pattern in
    (r"fn pipe_basis\(.*?^}", r"pub fn pipe_concrete\(.*?^}", r"pub fn pipe_joint\(.*?^}"))
row_model = evaluator.split("pub struct Row {", 1)[1].split("/// 🧾", 1)[0]
directory = ticket/"🗑️generated/en1998-concrete-joints"
source = directory/"assess.rs"
binary = directory/"assess"
source.write_text(r'''
use std::{collections::BTreeMap, io::{self, BufRead}};
pub enum QuantityKind { Dimensionless }
pub enum CheckStatus { UnverifiedBasis }
pub struct Inputs<'a>(&'a str, BTreeMap<&'a str, f64>);
impl Inputs<'_> {
    fn t(&self, _key: &str) -> Result<&str, String> { Ok(self.0) }
    fn n(&self, key: &str) -> Result<f64, String> { self.1.get(key).copied().ok_or_else(|| key.to_string()) }
}
pub type Outcome = Result<Vec<Row>, String>;
pub struct Row {''' + row_model + production + r'''
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut values = line.split('|');
        let mode = values.next().unwrap();
        let basis = values.next().unwrap();
        let values = values.filter_map(|token| {
            let (key, value) = token.split_once('=').unwrap();
            value.parse::<f64>().ok().map(|number| (key, number))
        }).collect();
        let inputs = Inputs(basis, values);
        let rows = match mode { "concrete" => pipe_concrete(&inputs), "joint" => pipe_joint(&inputs), _ => panic!("unknown mode") }.unwrap();
        let status: String = rows.iter().map(|row| match row.status {
            Some(CheckStatus::UnverifiedBasis) => 'U',
            None if row.demand <= row.capacity => '1',
            None => '0',
        }).collect();
        println!("{status}");
    }
}
''')
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=root)
db = sqlite3.connect(":memory:")
counts = []
for mode, file, query in [
    ("concrete", "🧱️concrete.tsv", """WITH limits(basis,c,cl,st,stl,sc,scl,d,dl) AS (VALUES (?,?,?,?,?,?,?,?,?))
             SELECT (c<=cl)||(st<=stl)||(sc<=scl)||(d<=dl) FROM limits
             WHERE basis='confirmed' AND min(c,st,sc,d)>=0 AND min(cl,stl,scl,dl)>0
               AND max(c,cl,st,stl,sc,scl,d,dl)<1e999"""),
    ("joint", "🔗️joints.tsv", """WITH response(basis,a,r,lim) AS (VALUES (?,?,?,?))
             SELECT cast(r<=lim AS text) FROM response
             WHERE basis='confirmed' AND abs(a)<1e999 AND r>=0 AND lim>0 AND max(r,lim)<1e999"""),
]:
    p = artifact/"🧪️tests/🧫️fixtures/🧱️concrete-pipes"/file
    with p.open() as stream:
        entries = reader(stream, delimiter="\t")
        keys = next(entries)[2:-1]
        rows = list(entries)
    submitted = "".join("|".join([mode, row[1], *(f"{key}={value}" for key, value in zip(keys,row[2:-1]))])+"\n" for row in rows)
    statuses = subprocess.check_output([binary],input=submitted,text=True).splitlines()
    assert len(statuses) == len(rows)
    for row, rust in zip(rows,statuses):
        reference = db.execute(query,[row[1],*(float(value) if value else None for value in row[2:-1])]).fetchone()
        assert rust == (reference[0] if reference else "U") == row[-1], row
    counts.append(len(rows))
print(f"[DEBUG] EN 1998-4 concrete pipe and joint evaluators matched SQLite in {counts[0]} and {counts[1]} cases.")
