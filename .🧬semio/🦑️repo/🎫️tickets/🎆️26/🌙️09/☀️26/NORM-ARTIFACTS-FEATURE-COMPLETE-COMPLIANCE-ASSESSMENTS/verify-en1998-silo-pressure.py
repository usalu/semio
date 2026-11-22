from csv import DictReader
from math import cos, isclose, radians
from pathlib import Path
import re
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
en1998 = next((root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998").glob("🏅*/🔖*/🪆*/✳*"))
with (en1998 / "🧪️tests/🧫️fixtures/🌾️silo-pressure.tsv").open() as stream:
    cases = list(DictReader(stream, delimiter="\t"))
fields = ("height", "width", "offset", "factor", "weight", "angle", "static")
query = """
WITH input(wall,slope,h,d,x,a,g,t,p) AS (VALUES (?,?,?,?,?,?,?,?,?)),
eligible AS (
 SELECT *, a*g*min(h,d/2,3*x)/coalesce(cosd(slope),1) AS reference
 FROM input
 WHERE h>0 AND d>0 AND g>0 AND x>=0 AND a>=0 AND p>=0
   AND (slope IS NULL AND x<=h OR slope>0 AND slope<90)
   AND max(h,d,x,a,g,p)<1e999 AND (wall<>'circular' OR t>=0 AND t<360)
),
results AS (
 SELECT *, reference * CASE wall WHEN 'circular' THEN cosd(t)
   WHEN 'leeward' THEN 1 WHEN 'windward' THEN -1 ELSE 0 END AS extra
 FROM eligible
)
SELECT CASE WHEN p<reference THEN 'redistribute' ELSE 'ready' END,
 reference, CASE WHEN p>=reference THEN extra END, CASE WHEN p>=reference THEN p+extra END
FROM results
"""

def assert_case(actual, case):
    assert actual[0] == case["status"], (case["case"], actual)
    for key, value in zip(("reference", "additional", "total"), actual[1:]):
        expected = case[key]
        if expected:
            assert isclose(float(value), float(expected), abs_tol=1e-8), (case["case"], key, value)
        else:
            assert value in (None, ""), (case["case"], key, value)

with sqlite3.connect(":memory:") as database:
    database.create_function("cosd", 1, lambda t: None if t is None else cos(radians(t)))
    for case in cases:
        source = [case["station"], float(case["slope"]) if case["slope"] else None,
                  *(float(case[k]) for k in fields)]
        assert_case(database.execute(query, source).fetchone() or ("invalid", None, None, None), case)

kernel = en1998 / "🧬️schema/📐️formulas/🌾️silo-pressure/🦀️.rs"
assert kernel.is_file(), "Missing EN 1998 production silo-pressure kernel"
directory = ticket / "🗑️generated/en1998-silo-pressure"
directory.mkdir(parents=True, exist_ok=True)
source = directory / "assess.rs"
binary = directory / "assess"
catalogue = (en1998 / "🧬️schema/📐️formulas/📇️catalogue/🧮️evaluators/🦀️.rs").read_text()
evaluator = re.search(r"pub fn silo_wall_pressure\(.*?^}", catalogue, re.S | re.M)[0]
row_model = catalogue.split("pub struct Row {", 1)[1].split("/// 🧾", 1)[0]
source.write_text(f'#[path = "{kernel}"] mod production;\n' + r'''
mod part_4 { pub use crate::production::*; }
use production::{SiloPressure, SiloPressureInput, SiloWall, SiloWallSection, granular_silo_pressure};
use std::io::{self, BufRead};
pub enum QuantityKind { Pressure }
pub enum CheckStatus { UnverifiedBasis, Info }
pub struct Inputs<'a>(&'a str, &'a str, Option<f64>, [f64; 7]);
impl Inputs<'_> {
    fn t(&self, key: &str) -> Result<&str, String> {
        match key {
            "silo_pressure_basis" => Ok(self.0),
            "silo_pressure_wall" => Ok(self.1),
            "silo_pressure_section" => Ok(if self.2.is_some() { "hopper" } else { "flat-bottom" }),
            _ => Err(key.to_owned()),
        }
    }
    fn n(&self, key: &str) -> Result<f64, String> {
        if key == "silo_pressure_slope" { return self.2.ok_or_else(||key.to_owned()); }
        let keys = ["fill", "width", "height", "acceleration", "weight", "azimuth", "static"];
        let short = key.strip_prefix("silo_pressure_").unwrap();
        keys.iter().position(|name| *name == short).map(|i| self.3[i]).ok_or_else(||key.to_owned())
    }
}
pub type Outcome = Result<Vec<Row>, String>;
pub struct Row {''' + row_model + evaluator + r'''
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut input = line.split('|');
        let station = input.next().unwrap();
        let slope: Option<f64> = input.next().unwrap().parse().ok();
        let numbers =
            input.map(|field| field.parse().unwrap()).collect::<Vec<f64>>().try_into().unwrap();
        let [fill_height_m, width_m, height_m, acceleration_ratio, bulk_weight_n_m3, angle, static_pressure_pa] = numbers;
        let wall = match station {
            "circular" => SiloWall::Circular { azimuth_deg: angle },
            "leeward" => SiloWall::Leeward,
            "windward" => SiloWall::Windward,
            "parallel" => SiloWall::Parallel,
            _ => panic!("Unexpected station"),
        };
        let section = match slope {
            Some(inclination_deg) => SiloWallSection::Hopper { inclination_deg },
            None => SiloWallSection::VerticalFlatBottom,
        };
        let rows = silo_wall_pressure(&Inputs("confirmed", station, slope, numbers)).unwrap();
        let pending = silo_wall_pressure(&Inputs("unconfirmed", station, slope, numbers)).unwrap();
        assert!(matches!(pending.as_slice(), [Row { key: "basis", status: Some(CheckStatus::UnverifiedBasis), .. }]));
        match granular_silo_pressure(SiloPressureInput {
            fill_height_m, width_m, height_m, acceleration_ratio, bulk_weight_n_m3, static_pressure_pa, wall, section,
        }) {
            Some(SiloPressure::Applicable { reference_pa, additional_pa, total_pa }) => {
                assert!(rows.iter().all(|row| matches!(row.status, Some(CheckStatus::Info))));
                let values: Vec<_> = rows.into_iter().map(|r| r.demand).collect();
                assert_eq!(values, [reference_pa, additional_pa, total_pa]);
                println!("ready|{reference_pa}|{additional_pa}|{total_pa}");
            }
            Some(SiloPressure::Redistribution { reference_pa }) => {
                assert!(matches!(rows.as_slice(), [Row { key: "redistribution", status: Some(CheckStatus::UnverifiedBasis), .. }]));
                println!("redistribute|{reference_pa}||");
            }
            None => {
                assert!(matches!(rows.as_slice(), [Row { key: "basis", status: Some(CheckStatus::UnverifiedBasis), .. }]));
                println!("invalid|||");
            }
        }
    }
}
''')
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=root)
submitted = "".join("|".join((case["station"], case["slope"], *(case[field] for field in fields))) + "\n"
                    for case in cases)
actual = subprocess.check_output([binary], input=submitted, text=True).splitlines()
assert len(actual) == len(cases)
for row, case in zip(actual, cases):
    assert_case(row.split("|"), case)
print(f"[DEBUG] EN 1998-4 Rust silo pressure kernel, catalogue evaluator and SQLite matched all {len(cases)} scenes.")
