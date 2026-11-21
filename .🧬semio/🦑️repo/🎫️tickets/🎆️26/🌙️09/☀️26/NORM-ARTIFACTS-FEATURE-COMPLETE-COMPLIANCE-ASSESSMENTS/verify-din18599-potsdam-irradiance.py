"""☀️ Check Potsdam Annex E lookup against SQLite and a published §23 worked design."""
from pathlib import Path
import math
import sqlite3
import subprocess

ticket=Path(__file__).resolve().parent
workspace=next(p.parent for p in ticket.parents if p.name==".🧬semio")
a=next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599").glob("🏅*/🔖*/🪆*/✳*"))
model=(a/"🧬️schema/📸️snapshot/🦀️.rs").read_text()
horizontal=next(line for line in model.splitlines() if line.startswith("pub const POTSDAM_HORIZONTAL_IRRADIANCE_W_M2"))
text=(a/"🧬️schema/💡️inferences/⚖️checks/☀️potsdam-irradiance/🦀️.rs").read_text()
binary=ticket/"🗑️generated/din18599-potsdam-irradiance"
source=binary.with_suffix(".rs")
source.write_text("use std::io::{self, BufRead};\n"+horizontal+"\n"+text[text.index("const TILTS_DEG:"):]+r"""
fn main() {
    for row in io::stdin().lock().lines() {
        let row = row.unwrap();
        let parts: Vec<_> = row.split('\t').collect();
        let month = parts[0].parse::<usize>().unwrap() - 1;
        match potsdam_irradiance_w_m2(month, parts[1].parse().unwrap(), parts[2].parse().unwrap()) {
            Some(value) => println!("{value:.12}"),
            None => println!("scope"),
        }
    }
}
""")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=workspace)
data=(a/"🧪️tests/🧫️fixtures/☀️potsdam-irradiance/🔣️.tsv").read_text().splitlines()[1:]
observed=subprocess.check_output([binary],input="\n".join(data)+"\n",text=True).splitlines()
assert len(observed)==len(data)
db=sqlite3.connect(":memory:")
db.execute("CREATE TABLE west (month INT PRIMARY KEY, horizontal REAL, tilt30 REAL, days INT)")
db.executemany("INSERT INTO west VALUES (?,?,?,?)", zip(
    range(1,13),
    (29,44,97,189,221,241,210,180,127,77,31,17),
    (25,40,90,172,202,219,188,165,120,70,29,16),
    (31,28,31,30,31,30,31,31,30,31,30,31),
))
for row, actual in zip(data,observed):
    month, azimuth, tilt, expected, energy=row.split("\t")
    if expected=="scope":
        assert actual=="scope", row
    else:
        assert math.isclose(float(actual),float(expected),abs_tol=1e-9), row
    if energy!="-":
        independent=db.execute("""
           SELECT (horizontal * 2 + tilt30) / 3.0, round((horizontal * 2 + tilt30) / 3.0 * days * 24 * .000675 * .166 * 602, 2)
           FROM west WHERE month=?
        """, (month,)).fetchone()
        assert math.isclose(float(actual),independent[0],abs_tol=1e-9) and float(energy)==independent[1], row
print(f"[DEBUG] {len(data)} irradiance cases passed production Rust and all 12 worked PV yields matched independent SQLite.")
