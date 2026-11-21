import json
import math
import sqlite3
import subprocess
from pathlib import Path

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
artifact = next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599").glob("🏅*/🔖*/🪆*/✳*"))
fixture = artifact/"🧪️tests/🧫️fixtures/🏢annex-3-envelope/🔣️.json"
data = json.loads(fixture.read_text())
module = artifact/"🧬️schema/💡️inferences/⚖️checks/🏢annex-3-envelope/🦀️.rs"
binary = ticket/"🗑️generated/din18599-annex3-envelope"
source = binary.with_suffix(".rs")
source.write_text("""
#![allow(dead_code)]
use std::io::{self, BufRead};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Adjacency { Outdoor, Unheated, Ground, Heated, UnheatedAttic }
#[derive(Clone, Copy)]
enum ElementKind { Wall, Roof, Floor, Door, Window, CurtainWall, RoofGlazing, RoofDome, GlazedDoor, GroundSlabPerimeter, GroundSlabInterior, RoofWindow, RooflightStrip }
struct EnvelopeElement { id: String, kind: ElementKind, adjacency: Adjacency, area_m2: f64, u_value_w_m2k: f64, zone_id: String, tilt_deg: f64 }
struct Zone { id: String, theta_i_heat_c: f64 }
struct Din18599Snapshot { elements: Vec<EnvelopeElement>, zones: Vec<Zone> }
""" + f"""
#[path = {json.dumps(str(module), ensure_ascii=False)}]
mod annex;
use annex::*;
""" + r"""
fn integration() {
    let element = |kind, adjacency, area, u, zone: &str| EnvelopeElement {
        id: String::new(), kind, adjacency, area_m2: area, u_value_w_m2k: u,
        zone_id: zone.into(), tilt_deg: 90.0,
    };
    use ElementKind::*;
    use Adjacency::*;
    let mut doc = Din18599Snapshot {
        zones: vec![Zone { id: "warm".into(), theta_i_heat_c: 19.0 }, Zone {id: "cool".into(), theta_i_heat_c: 18.0 }],
        elements: vec![
            element(GroundSlabPerimeter, Ground, 40.0, 0.20, "warm"),
            element(GroundSlabInterior, Ground, 60.0, 99.0, "warm"),
            element(Roof, UnheatedAttic, 60.0, 0.3, "warm"),
            element(GlazedDoor, Outdoor, 1.0, 1.7, "warm"),
            element(CurtainWall, Outdoor, 1.0, 2.0, "cool"),
            element(RoofGlazing, Outdoor, 1.0, 2.9, "cool"),
            element(RoofDome, Outdoor, 1.0, 3.1, "cool"),
        ],
    };
    let (groups, missing) = annex3_envelope_groups(&doc);
    assert!(!missing && groups.len() == 4);
    assert_eq!(groups[0].elements.len(), 2);
    assert!((groups[0].mean_u().unwrap() - 0.22).abs() < 1e-12);
    assert_eq!(groups[1].mean_u(), Some(1.7));
    assert_eq!(groups[2].class, Annex3EnvelopeClass::CurtainWall);
    assert_eq!(groups[2].limit(), 3.0);
    assert_eq!(groups[3].mean_u(), Some(3.0));
    doc.elements[0].kind = Floor;
    let (groups, _) = annex3_envelope_groups(&doc);
    assert_eq!(groups[0].mean_u(), None);
}

fn main() {
    integration();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut cells = line.split('\t');
        let key = cells.next().unwrap();
        let class = Annex3EnvelopeClass::ALL.into_iter().find(|value| format!("{value:?}") == key).unwrap();
        let theta = cells.next().unwrap().parse().unwrap();
        let numbers: Vec<_> = cells.map(|cell| cell.parse::<f64>().unwrap()).collect();
        let elements = numbers.chunks_exact(3).map(|row| (row[0], row[1], row[2]));
        let text = |value: Option<f64>| value.map_or_else(|| "scope".to_string(), |v| format!("{v:.12}"));
        println!("{}\t{}", text(annex3_limit(class, theta)), text(annex3_mean_u(elements)));
    }
}
""")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], cwd=workspace, check=True)
lines = ["\t".join(map(str, (case["envelopeClass"], case["setpointC"], *(v for row in case["components"] for v in row)))) for case in data]
observed = subprocess.check_output([binary], text=True, input="\n".join(lines)+"\n").splitlines()
assert len(data) == len(observed)
db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE transmittance (class TEXT PRIMARY KEY, warm REAL, reduced REAL)")
db.executemany("INSERT INTO transmittance VALUES (?,?,?)",
               [("Opaque", .28, .5), ("Transparent", 1.5, 2.8), ("CurtainWall", 1.5, 3.0), ("RoofGlazing", 2.5, 3.1)])
db.execute("CREATE TABLE surface (area REAL, u REAL, factor REAL)")
for case, output in zip(data, observed):
    expected_limit = db.execute("""
        SELECT CASE WHEN :theta >= 19 THEN warm WHEN :theta >= 12 THEN reduced END
        FROM transmittance WHERE class = :category
    """, {"theta":case["setpointC"], "category":case["envelopeClass"]}).fetchone()[0]
    db.executemany("INSERT INTO surface VALUES (?,?,?)", case["components"])
    expected_mean = db.execute("""
        SELECT CASE WHEN min(area) > 0 AND min(u) >= 0 AND sum(factor NOT IN (0.5,1)) = 0
                    THEN sum(area*u*factor) / sum(area) END
        FROM surface
    """).fetchone()[0]
    db.execute("DELETE FROM surface")
    for expected, fixture_value, rust_value in zip((expected_limit, expected_mean), (case["limitU"], case["meanU"]), output.split("\t")):
        if expected is None:
            assert fixture_value is None and rust_value == "scope", case
        else:
            assert math.isclose(expected, fixture_value, abs_tol=1e-12)
            assert math.isclose(expected, float(rust_value), abs_tol=1e-12)
print(f"[DEBUG] {len(data)} Annex 3 cases matched the production Rust calculation and independent SQLite. The full input grouping and unclassified ground-slab cases also passed.")
