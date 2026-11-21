"""🔆 Compare DIN EN 16798 lighting and equipment-noise references with SQLite."""
from pathlib import Path
import sqlite3
import subprocess

ticket=Path(__file__).resolve().parent
workspace=next(p.parent for p in ticket.parents if p.name==".🧬semio")
a=next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798").glob("🏅*/🔖*/🪆*/✳*"))
text=(a/"🧬️schema/💡️inferences/⚖️checks/📜️annex-parameters-and-parts/🦀️.rs").read_text()
section=text[text.index("    pub fn illuminance_min_lx"):text.index("    pub const ADAPTIVE_RUNNING")]
binary=ticket/"🗑️generated/din16798-lighting-and-noise"
source=binary.with_suffix(".rs")
source.write_text("""
#![allow(private_interfaces)]
use std::io::{self, BufRead};
#[derive(Clone, Copy)]
enum OccupancyType { Office, Meeting, Classroom, Residential, Retail, Kitchen, Corridor }
enum AnnexChoice { En, De }
enum ComfortCategory { I, II, III, IV }
""" + section + r"""
fn main() {
    use OccupancyType::*;
    for row in io::stdin().lock().lines() {
        let row = row.unwrap();
        let parts: Vec<_> = row.split('\t').collect();
        let annex = if parts[0] == "de" { AnnexChoice::De } else { AnnexChoice::En };
        let usage = match parts[1] {
            "office" => Office, "meeting" => Meeting, "classroom" => Classroom, "residential" => Residential,
            "retail" => Retail, "kitchen" => Kitchen, "corridor" => Corridor, _ => panic!("usage"),
        };
        let category = match parts[2] {
            "I" => ComfortCategory::I, "II" => ComfortCategory::II, "III" => ComfortCategory::III, _ => ComfortCategory::IV,
        };
        let display = |value: Option<f64>| value.map_or("-".into(), |value| value.to_string());
        println!("{}\t{}", display(illuminance_min_lx(usage)), display(acoustic_limit_db(annex, usage, category)));
    }
}
""")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=workspace)
rows=(a/"🧪️tests/🧫️fixtures/🔆️lighting-and-noise/🔣️.tsv").read_text().splitlines()[1:]
actual=subprocess.check_output([binary],input="\n".join(rows)+"\n",text=True).splitlines()
assert len(actual)==len(rows)
db=sqlite3.connect(":memory:")
db.executescript("""
CREATE TABLE tasks (use_type TEXT PRIMARY KEY, illuminance INT);
INSERT INTO tasks VALUES ('office',500),('meeting',500),('classroom',500);
CREATE TABLE rooms (use_type TEXT PRIMARY KEY, en1 INT, en2 INT, en3 INT, de1 INT, de2 INT, de3 INT);
INSERT INTO rooms VALUES
 ('office',30,35,40,30,35,35),('meeting',30,35,40,30,35,40),('classroom',30,34,38,30,35,35),
 ('residential',30,35,40,30,35,35),('retail',35,40,45,40,45,50),
 ('kitchen',45,50,55,50,55,60),('corridor',35,40,45,35,40,45);
""")
for row, measured in zip(rows,actual):
    annex, usage, category, lux, noise=row.split("\t")
    value=db.execute("""
        SELECT illuminance, CASE :annex || :category
            WHEN 'enI' THEN en1 WHEN 'enII' THEN en2 WHEN 'enIII' THEN en3
            WHEN 'deI' THEN de1 WHEN 'deII' THEN de2 WHEN 'deIII' THEN de3 END
        FROM rooms LEFT JOIN tasks USING (use_type) WHERE use_type=:usage
    """,{"annex":annex,"usage":usage,"category":category}).fetchone()
    expected=tuple("-" if v is None else str(v) for v in value)
    assert tuple(measured.split("\t"))==expected==(lux,noise), row
print(f"[DEBUG] {len(rows)} lighting and equipment-noise cases matched production Rust and SQLite.")
