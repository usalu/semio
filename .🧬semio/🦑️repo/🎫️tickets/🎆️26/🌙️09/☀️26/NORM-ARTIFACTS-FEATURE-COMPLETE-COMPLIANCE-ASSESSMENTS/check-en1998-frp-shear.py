"""🧵️ Cross-checks EN 1998-3 FRP shear computations against SQLite."""
from pathlib import Path
import csv
import math
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
work = ticket / "🗑️generated"
work.mkdir(exist_ok=True)
p = Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📐️formulas/🧱️retrofit/🧵️frp-shear/🦀️.rs")
program = work / "en1998_frp.rs"
main = '''
use std::io::{self, BufRead};
fn main() {
    for row in io::stdin().lock().lines() {
        let row = row.unwrap();
        let f: Vec<_> = row.split('\\t').collect();
        let a: Vec<f64> = f[2..].iter().map(|v| v.parse().unwrap()).collect();
        let result = if f[0] == "circular" {
            circular_shear_n(a[0],a[1],a[4]).map(|shear| [0.0,0.0,0.0,shear])
        } else {
            let arrangement = match f[0] {
                "closed" => Arrangement::Closed { corner_radius_mm:a[10],web_width_mm:a[11],ultimate_mpa:a[12] },
                "u" => Arrangement::OpenU,
                "side" => Arrangement::Sides,
                _ => unreachable!(),
            };
            let layout = if f[1] == "sheet" { Layout::Sheet { web_height_mm:a[9] } }
                else { Layout::Strips { width_mm:a[7],spacing_mm:a[8] } };
            FrpRectangular { depth_mm:a[0], thickness_mm:a[1], strut_deg:a[2], fibre_deg:a[3], modulus_mpa:a[4],
                tensile_mpa:a[5], debonding_factor:a[6], arrangement, layout }.evaluate()
                .map(|r| [r.bond_length_mm,r.debonding_mpa,r.effective_mpa,r.shear_n])
        };
        if let Some([a,b,c,d]) = result { println!("{a:.16}\\t{b:.16}\\t{c:.16}\\t{d:.16}"); } else { println!("none"); }
    }
}
'''
program.write_text(p.read_text() + "\n" + main)
binary = work/"en1998_frp"
subprocess.run(["rustc", "--edition=2024", str(program), "-o", str(binary)], check=True)
vectors = (ticket/"en1998-frp-shear-cases.tsv").read_text().splitlines()[1:]
actual = subprocess.run([str(binary)], input="\n".join(vectors), check=True, text=True, capture_output=True).stdout.splitlines()
db = sqlite3.connect(":memory:")
for name, fn in (("sqrt", lambda x: math.sqrt(x) if x >= 0.0 else None), ("sin", math.sin), ("cos", math.cos), ("rad", math.radians)):
    db.create_function(name, 1, lambda value, fn=fn: fn(value) if value is not None else None)
db.create_function("pi", 0, lambda: math.pi)
query = """
WITH i(arr,form,d,t,beta0,alpha0,ef,fc,g,w0,s0,h,r,b,fu) AS (VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)),
geometry AS (
  SELECT *, .9*d z, rad(beta0) theta, rad(alpha0) beta,
  CASE WHEN form='sheet' THEN MIN(.9*d,h)*sin(rad(beta0+alpha0))/sin(rad(beta0)) ELSE w0 END w
  FROM i
),
layout AS (SELECT *, CASE WHEN form='sheet' THEN 1.0 ELSE w/s0 END rs FROM geometry),
factors AS (SELECT *, 1-2/pi() k, sqrt(1.5*(2-rs)/(1+w/100)) kb FROM layout),
bond AS (SELECT *, sqrt(.6*ef*fc*kb/t)/g fdd, sqrt(ef*t/sqrt(7.2*fc*kb)) le FROM factors),
distances AS (
  SELECT *, z-le*sin(beta) plain, kb*ef*sin(beta)/(3*fdd) slip FROM bond
),
strength AS (
  SELECT *, CASE arr
    WHEN 'closed' THEN fdd*(1-k*(1-plain/z)/2)+.5*MAX(0,(.2+1.6*r/b)*fu-fdd)*plain/z
    WHEN 'u' THEN fdd*(1-k*(1-plain/z))
    WHEN 'side' THEN fdd*(plain+slip)/z*(1-sqrt(k*slip/(plain+slip)))*(1-sqrt(k*slip/(plain+slip)))
  END fe FROM distances
)
SELECT le,fdd,fe,2*z*t*fe*CASE WHEN arr='side' THEN rs*sin(beta)/sin(theta)
    ELSE rs*rs*(cos(theta)*sin(beta)/sin(theta)+cos(beta)) END
FROM strength WHERE arr<>'circular' AND d>0 AND t>0 AND ef>0 AND fc>0 AND g>0
 AND beta0>0 AND beta0<90 AND alpha0>0 AND alpha0<=90 AND rs>0 AND rs<=1 AND w>0 AND fe>0
 AND (arr<>'closed' OR (b>0 AND r>=0 AND r<=.5*b AND fu>0)) AND (arr<>'side' OR plain>0)
UNION ALL SELECT 0,0,0,pi()*d*t*ef*.002 FROM i WHERE arr='circular' AND d>0 AND t>0 AND ef>0
"""
assert len(actual) == len(vectors)
for test,(row,raw) in enumerate(zip(csv.reader(vectors,delimiter="\t"),actual),1):
    expected = db.execute(query, [*row[:2], *map(float,row[2:])]).fetchone()
    if expected is None:
        assert raw == "none", (test, raw)
    else:
        values = list(map(float,raw.split("\t")))
        assert len(values) == len(expected) and all(math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12) for a,b in zip(values,expected)), (test,values,expected)
print(f"[DEBUG] EN 1998 FRP shear Rust and SQLite agreed on {len(vectors)} cases")
