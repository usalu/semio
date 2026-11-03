import sys
sys.path.insert(0, ".")
from spec import *

def num(x):
    if isinstance(x, int) or float(x).is_integer():
        return str(int(x))
    return repr(float(x))

def unitsuffix(f):
    return {"m": "m", "N": "N", "Pa": "Pa", "J": "J"}.get(f.unit, "") if f.kind == "n" else ""

def col_type(f):
    if f.kind == "s": return "TEXT"
    if f.kind == "u": return "UINT"
    return "QTY" if unitsuffix(f) else "NUM"

def val(f, v):
    if f.kind == "s": return v
    if f.kind == "u": return str(int(v))
    return num(v) + unitsuffix(f)

def kebab(s): return s.replace("_", "-")

def row_text(e, row):
    cells = [row["id"]]
    for f in e.fields: cells.append(val(f, row[f.name]))
    if e.nested:
        n = e.nested
        items = []
        for sub in row[n[0]]:
            parts = [f"id={sub['id']}"] + [f"{kebab(f.name)}={val(f, sub[f.name])}" for f in n[2].fields]
            items.append("{" + " ".join(parts) + "}")
        cells.append("[ " + " ".join(items) + " ]")
    return " ".join(cells)

def section(e, rows):
    cols = ["id:TEXT"] + [f"{kebab(f.name)}:{col_type(f)}" for f in e.fields]
    if e.nested: cols.append(f"{kebab(e.nested[0])}:TABLE")
    out = f"{kebab(e.coll)} [{' '.join(cols)}] {{\n"
    for r in rows: out += "  " + row_text(e, r) + "\n"
    return out + "}\n"

DEMO = {
 "anchors": [dict(id="anchor-A1", wall_id="ewall-E1", tendon="strand", head_depth=1.0, inclination_deg=15.0, free_length=8.0, fixed_length=6.0, fixed_diameter=0.15, bond_stress=200000.0, design_force=300000.0, tendon_yield_force=700000.0, min_free_length=4.5, test_factor=1.10, tests=[dict(id="atest-1", pull_out_resistance=650000.0), dict(id="atest-2", pull_out_resistance=640000.0), dict(id="atest-3", pull_out_resistance=660000.0)])],
 "embedded_walls": [dict(id="ewall-E1", kind="freeEarth", retained_height=4.0, embedment=6.0, prop_depth=1.0, thickness=0.6, backfill_gamma=19000.0, phi_deg=32.0, delta_active_deg=16.0, delta_passive_deg=10.0, surcharge=10000.0, undrained_strength=0.0)],
 "pile_groups": [dict(id="group-G1", pile_id="pile-P1", rows=2, columns=2, spacing=1.8, cap_vertical=1800000.0, cap_moment_x=100000.0, cap_moment_y=60000.0, single_resistance=1400000.0, shaft_cohesion_sum=400000.0, base_cohesion=120000.0)],
 "lab_tests": [dict(id="lab-T1", layer_id="layer-sand", kind="directShear", void_ratio=0.65, preload_stress=100000.0, drainage_height=0.01, consolidation_time=120.0, points=[dict(id="pt-1", normal_stress=50000.0, shear_stress=32500.0), dict(id="pt-2", normal_stress=100000.0, shear_stress=65000.0), dict(id="pt-3", normal_stress=200000.0, shear_stress=130000.0)])],
 "layer_waters": [dict(id="water-gravel", layer_id="layer-gravel", piezometric_depth=4.0, permeability=0.001, head_difference=1.0, flow_length=6.0)],
}
import copy
BAD = copy.deepcopy(DEMO)
BAD["embedded_walls"][0]["embedment"] = 1.0
a = BAD["anchors"][0]; a["design_force"] = 900000.0; a["free_length"] = 2.0; a["tests"] = [dict(id="atest-1", pull_out_resistance=400000.0)]
BAD["pile_groups"][0]["cap_vertical"] = 9000000.0; BAD["pile_groups"][0]["single_resistance"] = 1000000.0
BAD["lab_tests"][0]["points"] = [dict(id="pt-1", normal_stress=50000.0, shear_stress=20000.0), dict(id="pt-2", normal_stress=100000.0, shear_stress=38000.0), dict(id="pt-3", normal_stress=200000.0, shear_stress=76000.0)]
BAD["layer_waters"][0]["head_difference"] = 6.0; BAD["layer_waters"][0]["flow_length"] = 2.0

def block(data):
    return "".join(section(e, data[e.coll]) for e in ENTITIES)

for name, data in [("🎬️demo", DEMO), ("🏗️compliant", DEMO), ("🚨noncompliant", BAD)]:
    p = f"{S}/🖼️assets/{name}/🗣️.dsl.semio"
    s = open(p, encoding="utf8").read()
    if "anchors [" in s: continue
    s = s.rstrip("\n") + "\n" + block(data)
    open(p, "w", encoding="utf8").write(s)
print("ok")
