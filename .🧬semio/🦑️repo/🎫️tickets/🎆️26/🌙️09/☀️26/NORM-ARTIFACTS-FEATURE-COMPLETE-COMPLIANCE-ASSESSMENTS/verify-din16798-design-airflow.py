"""🧪 Compare the published design airflow scenarios with production Rust and SQLite."""
import csv
import math
from pathlib import Path
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(p.parent for p in ticket.parents if p.name == ".🧬semio")
family = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798"
artifact = next(family.glob("🏅*/🔖*/🪆*/✳*"))
p = artifact / "🧬️schema/💡️inferences/⚖️checks/📜️annex-parameters-and-parts/🦀️.rs"
text = p.read_text()
co2_start = text.index("    pub fn co2_above_outdoor_ppm(")
co2_end = text.index("    pub fn illuminance_min_lx", co2_start)
start = text.index("    pub fn outdoor_air_per_person_l_s(")
end = text.index("    /// 💨 ISO 7730 draught rate", start)
p = artifact / "🧪️tests/🧫️fixtures/🌬️design-airflow/🔣️.tsv"
with p.open(newline="") as source:
    cases = list(csv.reader(source,delimiter="\t"))[1:]
generated = ticket / "🗑️generated"
binary = generated / "din16798-design-airflow"
rust = binary.with_suffix(".rs")
rust.write_text(r"""
#![allow(private_interfaces)]
use std::io::{self, BufRead};
#[derive(Clone,Copy,PartialEq)]
enum AnnexChoice { En, De }
#[derive(Clone,Copy,PartialEq)]
enum OccupancyType { Office, Residential, Classroom }
#[derive(Clone,Copy,PartialEq)]
enum ComfortCategory { I, II, III, IV }
#[derive(Clone,Copy,PartialEq)]
enum PollutionClass { VeryLow, Low, NonLow }
#[derive(Clone,Copy,PartialEq)]
enum VentMethod { Method1PerceivedAirQuality, Method2LimitConcentration, Method3PredefinedRates }
""" + text[co2_start:co2_end] + text[start:end] + r"""
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let cols: Vec<_> = line.split('\t').collect();
        let annex = match cols[0] {
            "en" => AnnexChoice::En,
            "de" => AnnexChoice::De,
            _ => panic!(),
        };
        let method = match cols[1] {
            "method1" => VentMethod::Method1PerceivedAirQuality,
            "method2" => VentMethod::Method2LimitConcentration,
            "method3" => VentMethod::Method3PredefinedRates,
            _ => panic!(),
        };
        let usage = match cols[2] {
            "office" => OccupancyType::Office,
            "residential" => OccupancyType::Residential,
            "classroom" => OccupancyType::Classroom,
            _ => panic!(),
        };
        let pollution = match cols[3] {
            "very_low" => PollutionClass::VeryLow,
            "low" => PollutionClass::Low,
            "non_low" => PollutionClass::NonLow,
            _ => panic!(),
        };
        let category = match cols[4] {
            "I" => ComfortCategory::I,
            "II" => ComfortCategory::II,
            "III" => ComfortCategory::III,
            "IV" => ComfortCategory::IV,
            _ => panic!(),
        };
        let delta: f64 = cols[7].parse().unwrap();
        if line.ends_with("na9") {
            assert_eq!(co2_above_outdoor_ppm(annex, category), delta);
        }
        match required_outdoor_air_for_method(annex,method,cols[5].parse().unwrap(),cols[6].parse().unwrap(),category,usage,pollution,delta) {
            Ok(value) => println!("{value:.12}"),
            Err(OutdoorAirBasisGap::OutsideDefaultTable) => println!("scope"),
            Err(OutdoorAirBasisGap::GermanMethod3) => println!("german_method3"),
            Err(OutdoorAirBasisGap::GermanResidentialBasis) => println!("german_residential"),
            Err(OutdoorAirBasisGap::GermanVeryLowPollutionEvidence) => println!("german_approval"),
            Err(OutdoorAirBasisGap::NoPositiveCo2Gradient) => println!("gradient"),
        }
    }
}
""")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(rust),"-o",str(binary)],check=True,cwd=workspace)
observed = subprocess.check_output([binary],text=True,input="".join("\t".join(row[1:-1])+("\tna9" if row[0].startswith("german-na9-") else "")+"\n" for row in cases)).splitlines()
assert len(observed)==len(cases)
db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE category (name TEXT PRIMARY KEY, qp REAL, very_low REAL, low REAL, non_low REAL, office_qp REAL, office_qa REAL, adapted_qp REAL, adapted_qa REAL, residential_qp REAL)")
db.executemany("INSERT INTO category VALUES (?,?,?,?,?,?,?,?,?,?)", [
    ("I",10,.5,1,2,20,2,3.5,.25,10),("II",7,.35,.7,1.4,14,1.4,2.5,.15,7),
    ("III",4,.2,.4,.8,8,.8,1.5,.1,4),("IV",2.5,.15,.3,.6,5.5,.55,None,None,None),
])
sql = """
SELECT CASE
  WHEN :annex='de' AND :method='method3' THEN 'german_method3'
  WHEN :annex='de' AND :usage='residential' THEN 'german_residential'
  WHEN :annex='de' AND :method='method1' AND :pollution='very_low' THEN 'german_approval'
  WHEN :method='method1' AND :usage='residential' THEN 3.6*(:people*adapted_qp+:area*adapted_qa)
  WHEN :method='method1' THEN 3.6*(:people*qp+:area*CASE :pollution WHEN 'very_low' THEN very_low WHEN 'low' THEN low ELSE non_low END)
  WHEN :method='method3' AND :usage='residential' THEN 3.6*:people*residential_qp
  WHEN :method='method3' AND :usage='office' AND :pollution='low' THEN 3.6*max(:people*office_qp,:area*office_qa)
  WHEN :method='method2' AND :people=0 THEN 0
  WHEN :method='method2' AND :delta>0 THEN (20*0.001)*:people/(:delta/1000000.0)
  WHEN :method='method2' THEN 'gradient'
 END
FROM category WHERE name=:category
"""
for stated,(name,annex,method,usage,pollution,category,people,area,delta,target) in zip(observed,cases):
    result=db.execute(sql,dict(annex=annex,method=method,usage=usage,pollution=pollution,category=category,people=int(people),area=float(area),delta=float(delta))).fetchone()[0]
    if target in {"scope","gradient","german_method3","german_residential","german_approval"}:
        assert ("scope" if result is None else result)==stated==target,(name,result,stated,target)
    else:
        want=float(target)
        assert result is not None and math.isclose(result,want,rel_tol=1e-12,abs_tol=1e-12),(name,result,want)
        assert math.isclose(float(stated),want,rel_tol=1e-12,abs_tol=1e-12),(name,stated,want)
print(f"[DEBUG] DIN EN 16798-1 design airflow matched SQLite and production Rust on {len(cases)} scenarios.")
