from csv import reader
from pathlib import Path
import json
from math import isclose
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
artifact = next((root/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998").glob("🏅*/🔖*/🪆*/✳*"))
module = artifact/"🧬️schema/📐️formulas/🌊️pipe-wave/🦀️.rs"
with (artifact/"🧪️tests/🧫️fixtures/🌊️buried-pipe-wave/🔣️.tsv").open() as fixture:
    rows = reader(fixture, delimiter="\t")
    next(rows)
    fixtures = [dict(case=row[0], inputs=list(map(float,row[1:10])), expected=list(map(float,row[10:])) or None) for row in rows]
binary = ticket/"🗑️generated/en1998-buried-pipe-wave"
source = binary.with_suffix(".rs")
source.write_text(f'#[path = {json.dumps(str(module),ensure_ascii=False)}] mod pipe_wave;\n' + r'''
use pipe_wave::*;
use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let [velocity, a_long, c_long, a_trans, c_trans, radius, thickness, modulus, available] =
            line.split_whitespace().map(|s| s.parse::<f64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
        let actual = bonded_pipe_wave_response(PipeWaveInput {
            peak_longitudinal_velocity_m_s: velocity, peak_longitudinal_acceleration_m_s2: a_long,
            longitudinal_wave_speed_m_s: c_long, peak_transverse_acceleration_m_s2: a_trans,
            transverse_wave_speed_m_s: c_trans, pipe_radius_m: radius, wall_thickness_m: thickness,
            elastic_modulus_pa: modulus, available_bond_shear_pa: available,
        });
        if let Some(row) = actual {
            println!("[{:.17},{:.17},{:.17},{},{:.17},{:.17},{:.17}]", row.axial_strain, row.curvature_per_m,
                row.required_bond_shear_pa, row.bond_sufficient, row.envelope_strain,
                row.tensile_strain_limit, row.compressive_strain_limit);
        } else {
            println!("null");
        }
    }
}
''')
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=root)
arguments = "".join(" ".join(map(str,map(float,case["inputs"])))+"\n" for case in fixtures)
output = subprocess.check_output([binary],input=arguments,text=True).splitlines()
assert len(output) == len(fixtures)
db = sqlite3.connect(":memory:")
for case,rust in zip(fixtures, output):
    actual = json.loads(rust)
    values = dict(zip(("v","al","cl","at","ct","r","s","e","b"),map(float,case["inputs"])))
    reference = db.execute("""
        WITH inputs(v,al,cl,at,ct,r,s,e,b) AS (VALUES (:v,:al,:cl,:at,:ct,:r,:s,:e,:b)),
        result AS (
          SELECT v/cl AS axial, at/(ct*ct) AS curvature, s*e*al/(cl*cl) AS tau,
                 b, r, min(0.01, s/(5*r)) AS eps_c
          FROM inputs WHERE min(v,al,at,b)>=0 AND min(cl,ct,r,s,e)>0 AND r>s
        )
        SELECT axial, curvature, tau, b>=tau, axial+r*curvature, 0.03, eps_c FROM result
    """, values).fetchone()
    expected = case["expected"]
    assert (actual is None) == (reference is None) == (expected is None), case
    if reference is None:
        continue
    for i,(x,y,z) in enumerate(zip(actual,reference,expected)):
        assert (x == y == z) if i==3 else (isclose(x,y,rel_tol=2e-12,abs_tol=1e-15) and isclose(x,z,rel_tol=2e-12,abs_tol=1e-15)), (case,i,x,y,z)
print(f"[DEBUG] EN 1998-4 Annex B and §6.5.2 {len(fixtures)} welded steel pipe cases matched SQLite.")
