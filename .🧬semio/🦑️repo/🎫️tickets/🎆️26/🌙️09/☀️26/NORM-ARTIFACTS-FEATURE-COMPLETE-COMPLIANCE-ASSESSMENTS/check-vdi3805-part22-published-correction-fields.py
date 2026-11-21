"""🧪 Compare the reported Part 22 correction fields with a SQLite oracle."""
import csv
import json
import math
from pathlib import Path
import re
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(path.parent for path in ticket.parents if path.name == ".🧬semio")
artifact = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805"
inferences = next(artifact.glob("🏅*/🔖*/🪆*/✳*/🧬*/💡*inferences"))
owner = inferences / "🧱️sheet-checks/🌡️air-to-air-heat-pump"
document = json.loads((owner / "📜️published-corrections.json").read_text())
rules = {row["id"]: row for row in document["corrections"]}
path = inferences / "🧪️tests/🔬️unit/🧫️fixtures/part22-published-correction-fields.tsv"
with path.open(newline="") as source:
    cases = list(csv.reader(source,delimiter="\t"))[1:]
generated = ticket / "🗑️generated"
binary = generated / "verify-vdi3805-part22-published-fields"
rust = binary.with_suffix(".rs")
rust.write_text(f'#[path = "{owner / "⚖️published-corrections.rs"}"]\nmod corrections;\n' + r"""
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut row = line.split('\t');
        let date = row.next().unwrap().parse().unwrap();
        let family = row.next().unwrap();
        let input: Vec<_> = row.map(str::to_owned).collect();
        let result = corrections::published_fields(family, &input, date).unwrap();
        println!("{}", result.invalid.iter().map(usize::to_string).collect::<Vec<_>>().join(","));
    }
}
""")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(rust),"-o",str(binary)],check=True,cwd=workspace)
actual = subprocess.check_output([binary],text=True,input="".join("\t".join([stamp,family,*fields])+"\n" for stamp,family,_,_,*fields in cases)).splitlines()
assert len(actual)==len(cases)
db = sqlite3.connect(":memory:")
db.create_function("is_integer",1,lambda value: int(value is not None and re.fullmatch(r"[+-]?[0-9]+",value) is not None))
db.create_function("is_number",1,lambda value: int(value is not None and re.fullmatch(r"[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?",value) is not None and math.isfinite(float(value))))
db.execute("CREATE TABLE fields (position INTEGER PRIMARY KEY, value TEXT, int_value INTEGER, n_valid INTEGER)")
def oracle(date, family, candidate):
    db.execute("DELETE FROM fields")
    db.executemany("INSERT INTO fields VALUES (?,?,CASE WHEN is_integer(?) THEN CAST(? AS INTEGER) END,is_number(?))",
                   [(position,value.strip(),value.strip(),value.strip(),value.strip()) for position,value in enumerate(candidate,1)])
    violations = []
    def get(position, cond, bindings=()):
        valid = db.execute("SELECT "+cond+" FROM fields WHERE position=?",(*bindings,position)).fetchone()
        if not valid or valid[0]!=1:
            violations.append(position)
    get(1, "value=?", (family,))
    if family in {"110","400","710.10"}:
        spec = rules[{"110":"22-0010","400":"22-0011","710.10":"22-0001"}[family]]
        current = date>=int(spec["effective"].replace("-",""))
        interval = list(spec["current"] if current else spec["prior"])
        expr = "int_value BETWEEN ? AND ?"
        if family=="110":
            expr += " OR int_value BETWEEN ? AND ?"
            interval += spec["manufacturerRange"]
        elif family=="710.10":
            get(2, "int_value BETWEEN 1 AND 2147483647")
        get(spec["field"],expr,tuple(interval))
    elif family=="710.09":
        get(2,"int_value BETWEEN 1 AND 2147483647")
        get(3,"int_value IN (1,2)")
        for pos in (4,5,6,8):
            get(pos,"value='' OR n_valid")
        get(7,"value='' OR int_value BETWEEN 1 AND 2147483647")
        violations.extend(range(9,len(candidate)+1))
    else:
        get(2,"int_value BETWEEN 1 AND 99999")
        if len(candidate)>=21 and candidate[20].strip():
            get(21,"n_valid AND (SELECT int_value<>0 FROM fields WHERE position=20)")
        if date>=202201:
            get(40,"value='' OR int_value IN (1,2,3,4,5,99)")
            get(41,"value<>'' OR COALESCE((SELECT int_value<>99 FROM fields WHERE position=40),1)")
        current = date>=202601
        for pos in (34,35):
            if current or len(candidate)>=pos:
                get(pos,"value='' OR n_valid")
        for pos,values in [(36,[1,2]),(39,[0,1])]:
            if current or len(candidate)>=pos:
                get(pos,"value='' OR int_value IN ("+",".join("?" for _ in values)+")",values)
        if current or len(candidate)>=37:
            classes = ["A1","A2","A3","B1","B2","B3"] + (["A2L"] if current else [])
            get(37,"value='' OR value IN ("+",".join("?" for _ in classes)+")",classes)
        if current:
            get(38,"1")
            get(42,"value='' OR int_value BETWEEN 0 AND 6")
            get(43,"value='' OR n_valid")
        else:
            for pos in range(42,min(43,len(candidate))+1):
                get(pos,"value=''")
    return ",".join(str(position) for position in sorted(set(violations)))
for observed,(stamp,family,name,expected,*candidate) in zip(actual,cases):
    independent=oracle(int(stamp),family,candidate)
    assert observed==independent==expected,(name,observed,independent,expected)
print(f"[DEBUG] VDI 3805 Part 22 {len(cases)} published correction field cases matched production Rust and independent SQLite.")
