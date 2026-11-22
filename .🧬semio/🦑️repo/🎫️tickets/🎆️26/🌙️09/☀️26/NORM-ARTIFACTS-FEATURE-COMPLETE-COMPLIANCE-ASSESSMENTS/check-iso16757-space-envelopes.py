"""📦️ Compares ISO space-envelope predicates with SQLite for finite boxes."""
from pathlib import Path
import csv
import sqlite3
import subprocess
import sys

ticket = Path(__file__).resolve().parent
work = ticket/"🗑️generated"
work.mkdir(exist_ok=True)
source = Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📐️part-2-geometry/📦️space-envelopes/🦀️.rs")
p = work/"iso16757-envelope.rs"
p.write_text(source.read_text() + '''
use std::io::{self, BufRead};
fn main() {
    for row in io::stdin().lock().lines() {
        let row = row.unwrap();
        let f: Vec<_> = row.split('\\t').skip(1).take(12).map(|v| v.parse::<f64>().unwrap()).collect();
        let a = f[0..3].try_into().unwrap();
        let b = f[3..6].try_into().unwrap();
        let c = f[6..9].try_into().unwrap();
        let d = f[9..12].try_into().unwrap();
        println!("{}\\t{}\\t{}", u8::from(envelope_volume(a,b).is_some()),u8::from(envelope_volume(c,d).is_some()),u8::from(contains_box(a,b,c,d)));
    }
}
''')
binary = work/"iso16757-envelope"
subprocess.run(["rustc","--edition=2024",str(p),"-o",str(binary)],check=True)
vectors = (ticket/"iso16757-space-envelope-cases.tsv").read_text().splitlines()[1:]
actual = subprocess.run([str(binary)],input="\n".join(vectors),text=True,capture_output=True,check=True).stdout.splitlines()
db = sqlite3.connect(":memory:")
db.create_function("finite",1,lambda x: x is not None and abs(x)<=sys.float_info.max)
sql = """
WITH b(ax,ay,az,bx,by,bz,cx,cy,cz,dx,dy,dz) AS (VALUES(?,?,?,?,?,?,?,?,?,?,?,?)),
validity AS (
  SELECT *,
    COALESCE(ax<bx AND ay<by AND az<bz AND finite((bx-ax)*(by-ay)*(bz-az)) AND (bx-ax)*(by-ay)*(bz-az)>0,0) a,
    COALESCE(cx<dx AND cy<dy AND cz<dz AND finite((dx-cx)*(dy-cy)*(dz-cz)) AND (dx-cx)*(dy-cy)*(dz-cz)>0,0) b
  FROM b
)
SELECT a,b,(a AND b AND ax<=cx AND ay<=cy AND az<=cz AND dx<=bx AND dy<=by AND dz<=bz) FROM validity
"""
assert len(vectors)==len(actual)
for row, line in zip(csv.reader(vectors,delimiter="\t"),actual):
    golden = tuple(map(int,row[13:]))
    oracle = db.execute(sql,[float(v) for v in row[1:13]]).fetchone()
    result = tuple(map(int,line.split("\t")))
    assert result==oracle==golden,(row[0],result,oracle,golden)
print(f"[DEBUG] ISO 16757 Rust and SQLite agreed on {len(vectors)} envelope cases")
