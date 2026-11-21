import json
import math
import sqlite3
import subprocess
from pathlib import Path

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
artifact = next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599").glob("🏅*/🔖*/🪆*/✳*"))
fixture = artifact/"🧪️tests/🧫️fixtures/🏢annex-2-reference/🔣️.json"
data = json.loads(fixture.read_text())
module = artifact/"🧬️schema/💡️inferences/⚖️checks/🏢annex-2-reference/🦀️.rs"
generated = ticket/"🗑️generated"
binary = generated/"din18599-annex2-reference"
source = binary.with_suffix(".rs")
model = (artifact/"🧬️schema/📸️snapshot/🦀️.rs").read_text()
enums = "\n".join("#[derive(Clone, Copy, PartialEq)]\n"+ model[model.index("pub enum "+name):].split("\n}\n",1)[0]+"\n}" for name in ("ElementKind","Adjacency"))
rows = ",\n".join(f'(ElementKind::{r["kind"]}, Adjacency::{r["adjacency"]}, {float(r["setpointC"])})' for r in data["cases"])
source.write_text(enums + f"""
struct EnvelopeElement {{ kind: ElementKind, adjacency: Adjacency, area_m2: f64 }}
#[path = {json.dumps(str(module), ensure_ascii=False)}] mod annex;
use annex::*;
fn main() {{
    let cases = [{rows}];
    let text = |number: Option<f64>| number.map_or_else(|| "-".to_owned(), |value| format!("{{value}}"));
    for (kind, adjacency, setpoint) in cases {{
        match annex2_reference_values(kind, adjacency, setpoint) {{
            Some(row) => println!("{{}}\\t{{}}\\t{{}}\\t{{}}\\t{{}}", row.clause, row.u_value_w_m2k, text(row.solar_transmittance), text(row.visible_transmittance), row.bridge_surcharge_w_m2k),
            None => println!("scope"),
        }}
    }}
    let exterior = EnvelopeElement {{ kind: ElementKind::Wall, adjacency: Adjacency::Outdoor, area_m2: 20.0 }};
    let curtain = EnvelopeElement {{ kind: ElementKind::CurtainWall, adjacency: Adjacency::Outdoor, area_m2: 80.0 }};
    let internal = EnvelopeElement {{ kind: ElementKind::Wall, adjacency: Adjacency::Heated, area_m2: 20.0 }};
    assert_eq!(annex2_reference_bridge_w_k(&[&exterior, &curtain, &internal], 19.0), 1.0);
    assert_eq!(annex2_reference_bridge_w_k(&[&exterior, &curtain, &internal], 18.0), 2.0);
}}
""")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=workspace)
actual = subprocess.check_output([binary],text=True).splitlines()
db=sqlite3.connect(":memory:")
db.execute("CREATE TABLE envelope (kind TEXT,adjacency TEXT,clause TEXT,warm_u REAL,cool_u REAL,warm_g REAL,cool_g REAL,warm_t REAL,cool_t REAL)")
for r in data["rows"]:
    db.execute("INSERT INTO envelope VALUES (?,?,?,?,?,?,?,?,?)",
               (r["kind"],r["adjacency"],r["clause"],*r["transmittance"],*(r["solar"] or (None,None)),*(r["visible"] or (None,None))))
assert len(actual) == len(data["cases"])
for row,out in zip(data["cases"],actual):
    expected=db.execute("""
        SELECT clause,
               iif(:setpointC >=19,warm_u,cool_u),
               iif(:setpointC >=19,warm_g,cool_g),
               iif(:setpointC >=19,warm_t,cool_t),
               CASE WHEN clause='1.2' THEN 0.0 WHEN :setpointC>=19 THEN 0.05 ELSE 0.10 END
        FROM envelope WHERE kind=:kind AND adjacency=:adjacency AND :setpointC>=12
    """,row).fetchone()
    if expected is None:
        assert out=="scope",row
    else:
        clause,*numeric=out.split("\t")
        assert clause==expected[0],row
        for a,b in zip(numeric,expected[1:]):
            assert a=="-" if b is None else math.isclose(float(a),b),row
print(f"[DEBUG] {len(actual)} Annex 2 envelope cases matched independent SQLite; the two bridge-area cases also passed.")
