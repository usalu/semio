"""Verify restored EN 1993 fatigue catalogue inputs and existing concrete case outputs."""
import json
import math
from pathlib import Path
import re
import subprocess


ticket = Path(__file__).resolve().parent
root = next(path.parent for path in ticket.parents if path.name == ".🧬semio")
artifact = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
schema = next(artifact.glob("🏅*/🔖*/🪆*/✳*/🧬*"))
catalogue = schema / "💡️inferences/⚖️checks/📇️catalogue"
evaluators = catalogue / "🧮️evaluators/🦀️.rs"
implementation = evaluators.read_text()
checks = implementation[implementation.index("const RUNWAY_DETAILS"):implementation.index("const CHS_LATTICE_FACTORS")]
entries = {}
for entry, keys in re.findall(r'id: "(C93-\d+)", .*?inputs: &\[(.*?)\], eval:', (catalogue / "🦀️.rs").read_text()):
    entries[entry] = set(re.findall(r'"([^"]+)"', keys))
catalogue_inputs = json.loads((catalogue / "📚️inputs/🔣️.json").read_text())["inputs"]
for entry, keys in entries.items():
    for key in keys:
        assert any(item["key"] == key and item["scope"] in ("", entry) for item in catalogue_inputs), (entry,key)

paths = ticket / "🗑️generated"
paths.mkdir(exist_ok=True)
binary = paths / "en1993-deck-fatigue-probe"
source = binary.with_suffix(".rs")
source.write_text(r"""
use std::{collections::BTreeMap, io::{self, BufRead}};
enum QuantityKind { Stress }
struct Row { demand:f64, capacity:f64 }
impl Row {
    fn new(_: &str, _: &str, _: &str, _: QuantityKind, demand:f64, capacity:f64, _: String) -> Self { Self { demand, capacity } }
    fn ratio(_: &str, _: &str, _: &str, demand:f64, _: String) -> Self { Self { demand: demand * 1e6, capacity: 1e6 } }
}
type Outcome = Result<Vec<Row>, String>;
struct Inputs<'a>(BTreeMap<&'a str, &'a str>);
impl Inputs<'_> {
    fn t(&self, key: &str) -> Result<&str,String> { self.0.get(key).copied().ok_or_else(|| key.to_owned()) }
    fn n(&self, key: &str) -> Result<f64,String> { self.t(key)?.parse().map_err(|_| key.to_owned()) }
}
""" + checks + r"""
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let (entry, fields) = line.split_once('\t').unwrap();
        let args = Inputs(fields.split('\t').map(|field| field.split_once('=').unwrap()).collect());
        let result = match entry {
            "C93-18" => runway_junction_fatigue(&args),
            "C93-19" => open_stringer_fatigue(&args),
            "C93-20" => continuous_open_stringer_fatigue(&args),
            "C93-21" => closed_stringer_fatigue(&args),
            "C93-22" => closed_stringer_cutout_fatigue(&args),
            "C93-23" => closed_stringer_splice_fatigue(&args),
            "C93-24" => closed_stringer_deck_weld_fatigue(&args),
            _ => panic!("unexpected entry"),
        };
        match result {
            Ok(rows) => println!("{}", rows.iter().map(|r| format!("{:.13},{:.13}",r.demand / 1e6,r.capacity / 1e6)).collect::<Vec<_>>().join(";")),
            Err(_) => println!("invalid"),
        }
    }
}
""")
subprocess.run(
    ["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc", "--edition", "2024", str(source), "-o", str(binary)],
    check=True, cwd=root,
)

def fixture(number):
    return json.loads((catalogue / f"🧪️tests/table-8-{number}.json").read_text())["cases"]

cases = [(entry, {**choices, **values}, expected) for entry, choices, values, expected in fixture(8)]
for entry, choice, values, expected in fixture(9):
    key = "deck_open_end_basis" if entry == "C93-19" else "deck_open_cross_basis"
    cases.append((entry, {key: choice, **values}, expected))
for detail, location, qualification, equivalent, frequent, fy, gamma_ff, gamma_mf, demand, capacity, status, frequent_status in fixture(10):
    values = dict(runway_detail=detail, runway_location=location, runway_basis=qualification,
                  runway_range=equivalent, runway_frequent=frequent, fy=fy, gamma_ff=gamma_ff, gamma_mf=gamma_mf)
    expected = [(demand, capacity, status), (frequent, 1.5*fy, frequent_status)] if demand is not None else []
    cases.append(("C93-18", values, expected))

payload = "".join(entry + "\t" + "\t".join(f"{key}={value}" for key, value in values.items()) + "\n" for entry, values, _ in cases)
rust = subprocess.check_output([binary], input=payload, text=True).splitlines()
assert len(rust) == len(cases)
for observed, (entry, values, expected) in zip(rust, cases):
    if not expected:
        assert observed == "invalid", (entry,values,observed)
        continue
    wanted = [number for demand, capacity, _ in expected for number in (demand,capacity)]
    numbers = [float(value) for value in re.split("[;,]", observed)]
    assert len(numbers) == len(wanted) and all(math.isclose(x, y, rel_tol=1e-12) for x,y in zip(numbers,wanted)), (entry,observed,expected)
    assert ["pass" if numbers[i] <= numbers[i+1] else "fail" for i in range(0,len(numbers),2)] == [row[2] for row in expected], entry
for number in [8,9,10]:
    subprocess.run(["repo","tools","exec","--light","python3","--",str(ticket / f"verify-en1993-table-8-{number}.py"),str(catalogue / f"🧪️tests/table-8-{number}.json")],check=True,cwd=root)
print(f"[DEBUG] All {len(entries)} EN 1993 catalogue entries have input definitions. All {len(cases)} runway and deck fixture scenarios matched production Rust and SQLite.")
