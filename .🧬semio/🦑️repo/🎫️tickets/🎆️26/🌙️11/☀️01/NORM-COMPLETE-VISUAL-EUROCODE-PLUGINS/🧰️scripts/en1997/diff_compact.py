import sys, re
sys.path.insert(0, ".")
from spec import *
p = S + "/🧬️schema/🔺️diff/🦀️.rs"
s = open(p, encoding="utf8").read()
M = "semio_s_artifact_norm_en1997_model"
EXIST = {
    "layers": ("SoilLayer", "En1997LayersPatch", [("oedometric_modulus", "f64"), ("phi_prime_deg", "f64")]),
    "footings": ("SpreadFoundation", "En1997FootingsPatch", [("width", "f64"), ("embedment", "f64")]),
    "piles": ("Pile", "En1997PilesPatch", [("length", "f64"), ("count", "u32")]),
    "retaining_walls": ("RetainingWall", "En1997RetainingWallsPatch", [("base_width", "f64")]),
    "slopes": ("Slope", "En1997SlopesPatch", [("angle_deg", "f64")]),
    "uplift_cases": ("UpliftCase", "En1997UpliftCasesPatch", [("permanent_stabilizing", "f64"), ("permanent_destabilizing", "f64"), ("variable_destabilizing", "f64"), ("pore_pressure", "f64"), ("total_stress", "f64")]),
}
RT = {"s": "String", "n": "f64", "u": "u32"}
for e in ENTITIES:
    EXIST[e.coll] = (e.struct, "En1997" + pascal(e.coll) + "Patch", [(f.name, RT[f.kind]) for f in e.fields])
count = 0
for coll, (st, pn, fields) in EXIST.items():
    start = s.index(f"/// 🩹️ Sparse patch of the `{coll}` row")
    end = s.index("protocol::list_delta! {", start)
    fl = ", ".join(f"{n}: {t}" for n, t in fields)
    macro = f'''protocol::row_patch! {{
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `{coll}` row addressed by its key (scalar leaves; nested rows travel with the whole-row update).
    pub {pn} of {M}::{st} {{ set {{ {fl} }} }}
}}

'''
    s = s[:start] + macro + s[end:]
    count += 1
s = s.replace("use semio_s_artifact_norm_en1997_model::En1997Snapshot;", "use semio_s_artifact_norm_en1997_model::En1997Snapshot;", 1)
open(p, "w", encoding="utf8").write(s)
print(count, len(s.split("\n")))
