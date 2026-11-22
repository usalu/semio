"""🌬️ Cross-checks the two DIN 4108-3 ventilated-roof branches with SQLite."""
import csv
from pathlib import Path
import sqlite3
import subprocess

ticket=Path(__file__).resolve().parent
g=ticket/"🗑️generated"
src=Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/💧️part-3-moisture-protection/📐️construction-cases/🌬️ventilated-roof/🦀️.rs")
p=g/"din4108_ventilated_roof.rs"
p.write_text(src.read_text()+'''
use std::io::{stdin,BufRead};
fn main() {
    for line in stdin().lock().lines() {
        let line=line.unwrap();
        let a:Vec<_>=line.split('\\t').collect();
        let n=|i:usize| a[i].parse().unwrap();
        let roof=VentilatedRoof {
            slope_deg:n(1), channel_roof_area_m2:n(2), channel_width_m:n(3), path_length_m:n(4), free_height_m:n(5),
            inlet:Opening {roof_area_m2:n(6),length_m:n(7),free_m2:n(8)},
            outlet:Opening {roof_area_m2:n(9),length_m:n(10),free_m2:n(11)},
            outlet_type:match a[12] {"opposite"=>Outlet::OppositeEdge,"mono"=>Outlet::EaveOrMonoPitch,"ridge"=>Outlet::RidgeOrHip,_=>unreachable!()},
            inboard_sd_m:n(13),inboard_r_fraction:n(14),
        };
        println!("{}", match roof.evaluate(){Some(r) if r.complies=>"pass",Some(_)=>"fail",None=>"unknown"});
    }
}
''')
bin=g/"din4108_ventilated_roof"
subprocess.run(["rustc","--edition=2024",str(p),"-o",str(bin)],check=True)
vectors=(ticket/"din4108-ventilated-roof-cases.tsv").read_text().splitlines()[1:]
actual=subprocess.run([str(bin)],input="\n".join(vectors),text=True,capture_output=True,check=True).stdout.splitlines()
db=sqlite3.connect(":memory:")
q="""
WITH i(slope,channel_area,channel_width,run,h,a_area,a_length,a_free,b_area,b_length,b_free,outlet,sd,r) AS (VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?)),
required AS (
 SELECT *,
 CASE WHEN slope<5 THEN MAX(.05,channel_area/(500*channel_width)) ELSE .02 END h_min,
 MAX(a_area/500,.02*a_length) a_min,
 CASE WHEN slope>=5 AND outlet='ridge' THEN MAX(b_area/2000,.005*b_length) ELSE MAX(b_area/500,.02*b_length) END b_min,
 CASE WHEN slope<5 THEN 100 ELSE 2 END sd_min
 FROM i
)
SELECT CASE
 WHEN slope<0 OR slope>=90 OR h<0 OR sd<0 OR a_area<=0 OR a_length<=0 OR a_free<0 OR b_area<=0 OR b_length<=0 OR b_free<0 THEN 'unknown'
 WHEN slope<5 AND (channel_area<=0 OR channel_width<=0 OR run<=0 OR r<0 OR r>1) THEN 'unknown'
 WHEN h<h_min OR a_free<a_min OR b_free<b_min OR sd<sd_min THEN 'fail'
 WHEN slope<5 AND (outlet<>'opposite' OR run>10 OR r>.2) THEN 'fail'
 WHEN slope>=5 AND outlet='opposite' THEN 'fail'
 ELSE 'pass' END FROM required
"""
assert len(vectors)==len(actual)
for row,value in zip(csv.reader(vectors,delimiter="\t"),actual):
    args=[*map(float,row[1:12]),row[12],*map(float,row[13:15])]
    oracle,=db.execute(q,args).fetchone()
    assert value==oracle==row[-1],(row[0],value,oracle,row[-1])
print(f"[DEBUG] DIN 4108-3 Rust and SQLite agreed on all {len(vectors)} ventilated roof cases")
