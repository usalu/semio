#!/usr/bin/env python3
"""🏭️ Check the VDI product attribute routes and example identifiers."""
import csv
from pathlib import Path
import re
import shutil
import sqlite3
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(path.parent for path in ticket.parents if path.name == ".🧬semio")
artifact = workspace / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805"
standard = next(artifact.glob("*standards/*1/*subsets/*any"))
schema = standard / "🧬️schema"
actual_source = (schema / "📸️snapshot/🧮️sheet-attributes/🦀️.rs").read_text()
record_parser = actual_source[actual_source.index("pub fn attributes_from_records"):actual_source.index("/// 📄️ One semicolon-delimited native record.")]
work = ticket / "🗑️generated"
work.mkdir(exist_ok=True)
shim = '''
#![allow(dead_code)]
use std::collections::BTreeMap;
pub struct SheetId(u16);
#[derive(PartialEq)]
pub struct RecordFamilyId(String);
impl RecordFamilyId { const R100: &'static str = "100"; }
pub struct NativeRecord { family: RecordFamilyId, fields: Vec<String> }
pub struct ValveHeatingAttributes;
impl ValveHeatingAttributes {
    fn from_kvs_m3_h(_: u16, _: f64, _: String, _: String, _: f64, _: f64) -> Self { Self }
}
pub struct RadiatorAttributes { standard_output_w:f64, heat_exponent_n:f64, length_m:f64, height_m:f64, depth_m:f64, connection_type:String }
pub struct PumpHeatingAttributes { dn_suction:u16, dn_discharge:u16, nominal_flow_m3_s:f64, nominal_head_m:f64, motor_power_w:f64, hydraulic_efficiency:f64, qh_curve_ref:Option<String> }
pub struct HeatGeneratorAttributes { nominal_heat_output_w:f64, fuel_type:String, flow_temp_max_c:f64, return_temp_min_c:f64 }
pub struct GenericAttributes;
impl GenericAttributes { fn from_map(_: &BTreeMap<String, String>) -> Self { Self } }
pub enum SheetAttributes { ValveHeating(ValveHeatingAttributes), Radiator(RadiatorAttributes), PumpHeating(PumpHeatingAttributes), HeatGenerator(HeatGeneratorAttributes), Generic(GenericAttributes) }
'''
runner = r'''
fn main() {
    use std::io::{self, BufRead};
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let input: Vec<_> = line.split('\t').collect();
        let record = NativeRecord { family: RecordFamilyId("210".into()), fields: vec!["210".into(), input[1].into(), input[2].into()] };
        match attributes_from_records(SheetId(input[0].parse().unwrap()), &[record]) {
            SheetAttributes::ValveHeating(_) => println!("valve"),
            SheetAttributes::Radiator(_) => println!("radiator"),
            SheetAttributes::PumpHeating(p) => println!("pump\t{:.17e}", p.nominal_flow_m3_s),
            SheetAttributes::HeatGenerator(_) => println!("generator"),
            SheetAttributes::Generic(_) => println!("generic"),
        }
    }
}
'''
f = work / "vdi3805_typed_published_sheets.rs"
f.write_text(shim + record_parser + runner)
exe = f.with_suffix(".exe")
subprocess.run([shutil.which("rustc"), "--edition=2021", str(f), "-o", str(exe)], check=True)
fixture = schema / "💡️inferences/🧪️tests/🔬️unit/🧫️fixtures/typed-sheet-routing.tsv"
with fixture.open() as stream:
    rows = list(csv.DictReader(stream, delimiter="\t"))
request = "".join(f"{r['sheet']}\t{r['flow_key']}\t{r['input_m3_s']}\n" for r in rows)
actual = subprocess.run([exe], input=request, text=True, capture_output=True, check=True).stdout.splitlines()
assert len(actual) == len(rows)
with sqlite3.connect(":memory:") as reference:
    reference.execute("CREATE TABLE typed(part INTEGER PRIMARY KEY, kind TEXT NOT NULL)")
    reference.executemany("INSERT INTO typed VALUES(?,?)", [(2, "valve"), (3, "generator"), (4, "pump"), (6, "radiator")])
    for actual_row, expected_row in zip(actual, rows):
        actual_kind, *flow = actual_row.split("\t")
        expected_kind, = reference.execute("SELECT coalesce((SELECT kind FROM typed WHERE part=?), 'generic')", (int(expected_row["sheet"]),)).fetchone()
        assert actual_kind == expected_kind == expected_row["kind"], expected_row
        if actual_kind == "pump":
            expected_flow, = reference.execute("SELECT CAST(? AS REAL)", (expected_row["input_m3_s"],)).fetchone()
            assert float(flow[0]) == expected_flow == float(expected_row["expected_m3_s"]), expected_row

for sheet, article, kind in [(3, "HG-24", "heatGenerator"), (4, "PMP-50", "pumpHeating"), (5, "AT-100", "generic"), (6, "RAD-600", "radiator"), (16, "FSD-100", "generic")]:
    for suffix in ("", "-fail"):
        path = standard / "🖼️assets" / f"blatt-{sheet}{suffix}" / "🗣️.dsl.semio"
        text = path.read_text()
        assert re.search(rf'^  {article} \{{[^\n]+ \] {sheet} \[', text, re.M), path
        assert re.search(rf'^  {article} {sheet} \[', text, re.M), path
        assert re.search(rf'kind="{kind}"', text), path
        clean = re.sub(r'"(?:[^"\\]|\\.)*"', '""', text)
        pending = []
        delimiters = {']': '[', '}': '{'}
        for character in clean:
            if character in '[{':
                pending.append(character)
            elif character in ']}':
                assert pending and pending.pop() == delimiters[character], path
        assert not pending, path
print(f"[DEBUG] {len(rows)} typed VDI routes matched Rust and SQLite. All ten catalogue assets have matching sheet, product variant and index.")
