from pathlib import Path
import json
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
artifact = next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108").glob("🏅*/🔖*/🪆*/✳*"))
data = json.loads((artifact/"🧪️tests/🧫️fixtures/📐annex-a-constructions/🔣️.json").read_text())
module = artifact/"🧬️schema/💡️inferences/⚖️checks/💧️part-3-moisture-protection/📐️construction-cases/🦀️.rs"
binary = ticket/"🗑️generated/din4108-annex-a-constructions"
source = binary.with_suffix(".rs")
source.write_text(f'#[path = {json.dumps(str(module),ensure_ascii=False)}] mod construction;\n' + r'''
use construction::*;
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line=line.unwrap();
        let (kind, fields) = line.split_once('\t').unwrap();
        let columns: Vec<f64> = fields.split('\t').map(|text| text.parse().unwrap()).collect();
        let outcome = match (kind, columns.as_slice()) {
            ("perimeter", &[fraction]) => perimeter_slab_limit(fraction),
            ("wall", &[resistance, inward_sd]) => interior_wall_limit(resistance, inward_sd),
            ("roof", &[outward_sd, inward_sd, fraction]) => unventilated_roof_limit(outward_sd, inward_sd, fraction),
            ("attic", &[area, openings, upper, insulation, layer, fraction]) => attic_ceiling_limits(AtticCeiling {
                attic_area_m2: area, ventilation_openings_m2: openings, upper_sd_m: upper,
                insulation_sd_m: insulation, airtight_layer_sd_m: layer,
                room_side_resistance_fraction: fraction,
            }).map(|limits| limits.meets_limits),
            ("slab", &[sd]) => internal_slab_sd_recommendation(sd),
            _ => unreachable!(),
        };
        println!("{}", outcome.map_or("scope", |result| if result { "true" } else { "false" }));
    }
}
''')
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=workspace)
db = sqlite3.connect(":memory:")
rows = [
    *(("attic", *row["inputs"]) for row in data["attic"]),
    *(("slab", sd) for sd, _ in data["slab"]),
    *(("perimeter", fraction) for fraction, _ in data["perimeter"]),
    *(("wall", resistance, float("nan") if sd is None else sd) for resistance, sd, _ in data["wall"]),
    *(("roof", *row["inputs"]) for row in data["roof_table3"]),
]
wire = "\n".join("\t".join(map(str,row)) for row in rows)+"\n"
observed = subprocess.check_output([binary],input=wire,text=True).splitlines()
assert len(observed) == len(rows)
for row,actual in zip(data["attic"],observed):
    area, openings, upper, insulation, air_seal, fraction = row["inputs"]
    expected = db.execute("""
    SELECT CASE WHEN :area<=0 OR :openings<0 OR :upper<0 OR :insulation<0 OR :seal<0 OR :fraction NOT BETWEEN 0 AND 1
          THEN NULL ELSE :openings >= :area*0.002 AND :upper <= (CASE WHEN :insulation>=1500 THEN 25 ELSE 2 END)
          AND :seal>=10 AND :fraction<=0.2 END
    """,dict(area=area,openings=openings,upper=upper,insulation=insulation,seal=air_seal,fraction=fraction)).fetchone()[0]
    assert expected == row["outcome"],row
    assert actual == ("scope" if expected is None else str(bool(expected)).lower()),row
for (sd,outcome),actual in zip(data["slab"],observed[len(data["attic"]):len(data["attic"])+len(data["slab"])]):
    expected = db.execute("SELECT CASE WHEN ? >= 0 THEN ? >= 20 END",(sd,sd)).fetchone()[0]
    assert expected == outcome and actual == ("scope" if expected is None else str(bool(expected)).lower()),sd

def wire_result(value):
    return "scope" if value is None else str(bool(value)).lower()


start = len(data["attic"]) + len(data["slab"])
for (fraction, outcome), actual in zip(data["perimeter"], observed[start:]):
    expected = db.execute("SELECT CASE WHEN ? BETWEEN 0 AND 1 THEN ? <= .20 END",(fraction,fraction)).fetchone()[0]
    assert expected == outcome and actual == wire_result(expected), fraction
start += len(data["perimeter"])
for (resistance, sd, outcome), actual in zip(data["wall"], observed[start:]):
    expected = db.execute("""
        SELECT CASE WHEN :r>1 THEN 0 WHEN :r>0 AND :r<=.5 THEN 1
          WHEN :r>.5 AND :sd>=0 THEN :sd>=.5 END
    """,dict(r=resistance,sd=sd)).fetchone()[0]
    assert expected == outcome and actual == wire_result(expected), (resistance, sd)
start += len(data["wall"])
for row, actual in zip(data["roof_table3"], observed[start:]):
    outside, inside, fraction = row["inputs"]
    expected = db.execute("""
        SELECT CASE WHEN :external>=0 AND :internal>=0 AND :fraction BETWEEN 0 AND 1 THEN
          :fraction<=.20 AND :external<=2 AND :internal >=
          CASE WHEN :external<=.1 THEN 1 WHEN :external<=.3 THEN 2 ELSE :external*6 END END
    """,dict(external=outside,internal=inside,fraction=fraction)).fetchone()[0]
    assert expected == row["outcome"] and actual == wire_result(expected), row
print(f"[DEBUG] {len(observed)} attic ceiling, floor slab, inboard wall insulation and Table 3 roof cases matched independent SQLite.")
