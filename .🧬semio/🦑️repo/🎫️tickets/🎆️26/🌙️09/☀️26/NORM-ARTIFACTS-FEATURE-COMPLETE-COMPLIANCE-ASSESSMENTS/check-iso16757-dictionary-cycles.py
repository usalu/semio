"""🕸️ Compares the production ISO subtype cycle check with SQLite reachability."""
from pathlib import Path
import csv
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
work = ticket/"🗑️generated"
src = Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️check-builders/🧩️part-libraries/🦀️.rs")
source = src.read_text().split("    pub fn detect_subtype_cycle",1)[1].split("    pub fn filter_controlled_values",1)[0]
p=work/"iso16757_dictionary_cycle.rs"
p.write_text('''
use std::collections::HashMap;
use std::io::{stdin,BufRead};
#[derive(PartialEq)]
enum RelationshipKind { IsSubtypeOf, Other }
struct Relationship { kind: RelationshipKind, source_id: String, target_id: String }
struct Dictionary { relationships: Vec<Relationship> }
fn detect_subtype_cycle''' + source + '''
fn main() {
    for row in stdin().lock().lines() {
        let row=row.unwrap();
        let edges=row.split('\\t').nth(1).unwrap();
        let mut relationships=Vec::new();
        for edge in edges.split(';').filter(|x| !x.is_empty()) {
            let (source_id,target_id)=edge.split_once('>').unwrap();
            relationships.push(Relationship {kind:RelationshipKind::IsSubtypeOf,source_id:source_id.into(),target_id:target_id.into()});
        }
        relationships.push(Relationship {kind:RelationshipKind::Other,source_id:"other".into(),target_id:"other".into()});
        println!("{}",u8::from(detect_subtype_cycle(&Dictionary{relationships})));
    }
}
''')
binary=work/"iso16757_dictionary_cycle"
subprocess.run(["rustc","--edition=2024",str(p),"-o",str(binary)],check=True)
rows=(ticket/"iso16757-dictionary-cycles.tsv").read_text().splitlines()[1:]
actual=subprocess.run([str(binary)],input="\n".join(rows),text=True,capture_output=True,check=True).stdout.splitlines()
db=sqlite3.connect(":memory:")
db.execute("CREATE TABLE edge(a TEXT,b TEXT)")
query = """
WITH RECURSIVE reachable(a,b) AS (
    SELECT a,b FROM edge
    UNION
    SELECT reachable.a,edge.b FROM reachable JOIN edge ON reachable.b=edge.a
)
SELECT EXISTS(SELECT 1 FROM reachable WHERE a=b)
"""
assert len(actual)==len(rows)
for (name,encoded,expected),raw in zip(csv.reader(rows,delimiter="\t"),actual):
    db.execute("DELETE FROM edge")
    db.executemany("INSERT INTO edge VALUES(?,?)",[part.split(">") for part in encoded.split(";") if part])
    oracle, = db.execute(query).fetchone()
    assert int(raw)==oracle==int(expected),(name,raw,oracle,expected)
print(f"[DEBUG] ISO 16757 subtype graph Rust and SQLite agreed on {len(rows)} cases")
