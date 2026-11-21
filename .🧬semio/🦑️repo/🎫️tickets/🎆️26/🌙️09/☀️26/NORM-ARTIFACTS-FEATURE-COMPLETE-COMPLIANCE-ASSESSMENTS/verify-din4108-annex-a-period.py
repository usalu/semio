from pathlib import Path
import json
import math
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
artifact = next((workspace/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108").glob("🏅*/🔖*/🪆*/✳*"))
case = json.loads((artifact/"🧪️tests/🧫️fixtures/💧annex-a-2024/🔣️.json").read_text())
checks = artifact/"🧬️schema/💡️inferences/⚖️checks/💧️part-3-moisture-protection"
bare = (checks/"🦀️.rs").read_text().split("/// ✅️ DIN 4108-3 period balance",1)[0]
member = checks/"📊️annex-a-period/🦀️.rs"
bare = bare.replace('#[path = "📇️catalogue/🦀️.rs"]\npub mod catalogue;', '')
bare = bare.replace('#[path = "📐️construction-cases/🦀️.rs"]\npub mod construction_cases;', '')
bare = bare.replace('#[path = "📊️annex-a-period/🦀️.rs"]',f'#[path = {json.dumps(str(member),ensure_ascii=False)}]')
material = checks/"📏️material-limits/🦀️.rs"
bare = bare.replace('#[path = "📏️material-limits/🦀️.rs"]',f'#[path = {json.dumps(str(material),ensure_ascii=False)}]')
catalogue = (checks/"📇️catalogue/🦀️.rs").read_text()
basis_logic = catalogue[catalogue.index("fn site_scope_value"):]

layers = ",\n".join(f'LayerDocument {{ thickness_m:{r["d"]}, mu:{float(r["mu"])}, lambda:{r["lam"]}, segments:vec![] }}' for r in case["layers"])
source = ticket/"🗑️generated/din4108-annex-a-period.rs"
binary = source.with_suffix("")
source.write_text("""
#![allow(dead_code)]
struct Segment { fraction: f64, mu: f64 }
struct LayerDocument { thickness_m: f64, mu: f64, lambda: f64, segments: Vec<Segment> }
fn layer_resistance(layer: &LayerDocument) -> f64 { layer.thickness_m / layer.lambda }
mod part_3 {
""" + bare + """
}
use part_3::*;
use std::collections::BTreeMap;
struct CaseValues { numbers: BTreeMap<String, f64>, texts: BTreeMap<String, String> }
struct DesignCase { entry_id: String, values: CaseValues }
""" + "\n".join(line for line in catalogue.splitlines() if line.startswith(("pub const SITE_ENTRY","pub const ROOF_ENTRY","const MATERIAL_ENTRY"))) + "\nuse material_limits::CondensateMaterials;\n" + basis_logic + f"""
fn site(elevation_m: f64) -> DesignCase {{
    DesignCase {{
        entry_id: SITE_ENTRY.into(),
        values: CaseValues {{
            numbers: BTreeMap::from([("site_elevation".into(),elevation_m)]),
            texts: BTreeMap::from([("period_conditions".into(),"confirmed".into())]),
        }},
    }}
}}
fn main() {{
    let roof = DesignCase {{
        entry_id: ROOF_ENTRY.into(),
        values: CaseValues {{
            numbers: BTreeMap::new(),
            texts: BTreeMap::from([("roof_element".into(),"roof".into()),("roof_exposure".into(),"unshaded-dark".into())]),
        }},
    }};
    let context = period_basis([site(700.0),roof], "roof");
    assert_eq!(context.site_scope, Some(true));
    assert_eq!(context.roof_exposure, Some(AnnexADryingExposure::UnshadedDarkRoof));
    assert_eq!(period_basis([site(700.001)], "wall").site_scope, Some(false));
    assert_eq!(period_basis([site(1.0),site(701.0)], "wall").site_scope, None);
    let materials = || DesignCase {{
        entry_id: MATERIAL_ENTRY.into(),
        values: CaseValues {{
            numbers: BTreeMap::from([("condensate_timber_increase".into(),5.0),("condensate_wood_product_increase".into(),3.0)]),
            texts: BTreeMap::from([("condensate_element".into(),"wall".into()),("condensate_capillarity".into(),"limited".into()),("condensate_damage".into(),"resistant".into())]),
        }},
    }};
    let entry = period_basis([materials()], "wall").materials.unwrap();
    assert!(entry.limited_capillarity && entry.materials_resist_damage);
    assert_eq!(entry.timber_increase_pp, 5.0);
    assert_eq!(period_basis([materials()], "roof").materials, None);
    assert_eq!(period_basis([materials(),materials()], "wall").materials, None);
    let layers = [{layers}];
    let balance = annex_a_period_balance(&layers, AnnexADryingExposure::WallOrAtticCeiling).unwrap();
    let profile = annex_a_winter_profile(&layers).unwrap();
    assert_eq!(profile.p_pa[0], 1168.0);
    assert_eq!(profile.p_pa.last().copied(), Some(321.0));
    for plane in &balance.planes {{
        println!("{{}}\\t{{:.12}}",plane.interface_index,plane.condensed_kg_m2);
    }}
    println!("total\\t{{:.12}}\\t{{:.12}}",balance.condensed_kg_m2,balance.evaporation_capacity_kg_m2);
    let covered = annex_a_period_balance(&layers, AnnexADryingExposure::ShadedOrLightRoof).unwrap();
    let exposed = annex_a_period_balance(&layers, AnnexADryingExposure::UnshadedDarkRoof).unwrap();
    assert_eq!(covered.evaporation_capacity_kg_m2,balance.evaporation_capacity_kg_m2);
    assert!(exposed.evaporation_capacity_kg_m2 > covered.evaporation_capacity_kg_m2);
    assert_eq!(annex_a_elevation_in_scope(f64::NAN), None);
    for v in [-25.0,0.0,699.999,700.0,700.001,2030.0] {{
        println!("altitude\\t{{v}}\\t{{}}",annex_a_elevation_in_scope(v).unwrap());
    }}
}}
""")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(binary)],check=True,cwd=workspace)
actual = [line.split("\t") for line in subprocess.check_output([binary],text=True).splitlines()]
db=sqlite3.connect(":memory:")
db.execute("CREATE TABLE layer (n INTEGER PRIMARY KEY,d REAL,mu REAL,lambda REAL)")
db.executemany("INSERT INTO layer VALUES (?,?,?,?)", ((i,r["d"],r["mu"],r["lam"]) for i,r in enumerate(case["layers"],1)))
pressure_sql = """
    WITH resistance(C) AS (SELECT sum(d/lambda)+0.29 FROM layer),
    states AS (
     SELECT n, sum(d*mu) OVER(ORDER BY n) as sd,
       20 - 25 * (sum(d/lambda) OVER(ORDER BY n) + 0.25)/C AS t
     FROM layer, resistance
    )
    SELECT sd, 610.5*exp(iif(t>=0,17.269*t/(237.3+t),21.875*t/(265.5+t))) FROM states WHERE n IN (2,4) ORDER BY n
"""
(x1,p1),(x2,p2)=db.execute(pressure_sql).fetchall()
sd = db.execute("SELECT sum(d*mu) FROM layer").fetchone()[0]
mass = db.execute("""
  SELECT 0.0015552 * ((1168-:p1)/:x1-(:p1-:p2)/(:x2-:x1)),
         0.0015552 * ((:p1-:p2)/(:x2-:x1)-(:p2-321)/(:sd-:x2))
""",dict(x1=x1,x2=x2,p1=p1,p2=p2,sd=sd)).fetchone()
capacity = db.execute("""
   SELECT :mc2 + :tc1 * :t2 + (:tc1 + 1.0e-7/(:sd-:x1)) * (7776000-:t2)
""",dict(mc2=mass[1],tc1=1e-7/x1,t2=mass[1]/(1e-7/(sd-x2)),sd=sd,x1=x1)).fetchone()[0]
for index,(plane,expected,published) in enumerate(zip(case["interfaces"],mass,case["interface_mass"])):
    assert int(actual[index][0])==plane
    assert math.isclose(float(actual[index][1]),expected,abs_tol=1e-10)
    assert math.isclose(expected,published,abs_tol=0.004)
assert actual[2][0]=="total"
assert math.isclose(float(actual[2][1]),sum(mass),abs_tol=1e-10)
assert math.isclose(float(actual[2][2]),capacity,abs_tol=1e-10)
document_plane_values = (1003.0, 411.0)
rounded_b = db.execute("""
  SELECT 0.0015552 * ((1168-:p1)/:x1-(:p1-:p2)/(:x2-:x1)),
         0.0015552 * ((:p1-:p2)/(:x2-:x1)-(:p2-321)/(:sd-:x2))
""",dict(x1=x1,x2=x2,p1=document_plane_values[0],p2=document_plane_values[1],sd=sd)).fetchone()
assert all(math.isclose(a,b,abs_tol=0.0012) for a,b in zip(rounded_b,case["interface_mass"]))
assert math.isclose(capacity,case["total_evaporable"],abs_tol=0.002)
for (label,elevation,result),(elevation_expected,scope) in zip(actual[3:],case["elevations"]):
    assert label=="altitude" and float(elevation)==elevation_expected
    independent=db.execute("SELECT ? <= 700",(elevation_expected,)).fetchone()[0]
    assert (result=="true")==scope==bool(independent)
print("[DEBUG] DIN 4108-3 Annex B interface masses and total drying matched independent SQLite. Six elevation cases , both roof exposure branches, and persisted case resolution passed.")
