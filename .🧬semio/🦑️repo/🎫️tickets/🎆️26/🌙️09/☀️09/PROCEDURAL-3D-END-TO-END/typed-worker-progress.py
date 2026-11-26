"""🧾️ Compare the mounted worker's progress boundary to SQLite across all four receipt fields."""
from pathlib import Path
import re
import sqlite3
import subprocess
import sys

out = Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
receipt = Path("🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs").read_text()
worker = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs").read_text()
record = re.search(r"pub struct RetainedCloneProgress \{[^}]+\}", receipt)[0]
stop = re.search(r"if ([^\n]+?)\|\|semio_plugin_reactor_app::remaining_plugin_turn_grant\(turn_grant,spent\).maximum_items==0\{break;\}", worker)[1]
source = out/"worker.rs"
source.write_text("#[derive(Default,PartialEq)]\n" + record + """
fn main() {
 for mask in 0..16 {
  let progress = RetainedCloneProgress{copied_items:mask&1,copied_bytes:mask&2,retained_capacity_bytes:mask&4,released_bytes:mask&8};
  for left in 0..=1 {println!("{} {} {}",mask,left,STOP || left==0);}
 }
}
""".replace("STOP", stop))
binary = out/("worker.exe" if sys.platform=="win32" else "worker")
subprocess.run(["repo","tools","exec","--light","rustc","--","--edition","2024",str(source),"-o",str(binary)], check=True)
rows=subprocess.check_output([binary],text=True).splitlines()
db=sqlite3.connect(":memory:")
for line in rows:
    mask,left,stop = line.split()
    expected,=db.execute("select ?=0 or ?=0",(int(mask),int(left))).fetchone()
    assert (stop=="true")==bool(expected),line
assert len(rows)==32
print("32 compiled worker progress boundaries agree with SQLite")
