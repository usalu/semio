"""Quick self-check of the analysis module (ticket CONCRETE-BIM-CLIP-EXAMPLES). The TESTS agent writes the real tests."""

import importlib.util
import json
import math
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip")


sys.path.insert(0, "C:/git/semio")
PACKAGE = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model"
core = importlib.import_module(PACKAGE + ".🐍️")
analysis = importlib.import_module(PACKAGE + ".🧮️analysis.🐍️")
m = core.load_model()
fail = []


def check(name, ok, detail=""):
    print(("ok   " if ok else "FAIL ") + name, detail)
    if not ok:
        fail.append(name)


a = analysis.translate_structure(m)
c = a["counts"]
check("reexported by core", core.translate_structure is not None and core.tributary is not None and core.clash_check is not None)
check("24 shells", c["shells"] == 24 == len(a["shells"]), c)
check("35 slab nodes + 25 base nodes", (c["slab_nodes"], c["base_nodes"], c["nodes"]) == (35, 25, 60) == (35, 25, len(a["nodes"])))
check("25 curve, 24 surface members", (c["curve_members"], c["surface_members"], c["members"]) == (25, 24, 49))
check("25 supports on base nodes", len(a["supports"]) == 25 and all(s["node"].startswith("F-") for s in a["supports"]))
check("node ids unique", len({n["id"] for n in a["nodes"]}) == len(a["nodes"]))
ids = {n["id"] for n in a["nodes"]}
check("members and shells reference nodes", all(set(x["nodes"]) <= ids for x in a["members"] + a["shells"]))
check("mid-surface z", all(n["position"][2] == a["mid_surface"]["z"] for n in a["nodes"] if n["role"] == "slab") and a["mid_surface"]["z"] == 3.375, a["mid_surface"])
check("shell area sum = gross slab area", math.isclose(sum(s["area"] for s in a["shells"]), a["slab"]["gross_area"]), a["slab"])
check("source guids kept", all(x["source_guid"] and len(x["source_guid"]) == 22 for x in a["members"]))
check("class names", {x["ifc_class"] for x in a["members"]} == {"IfcStructuralCurveMember", "IfcStructuralSurfaceMember"})
check("loads", math.isclose(a["loads"]["total"]["value"], 25 * 0.25 + 1.5 + 2.0) and a["loads"]["self_weight"]["value"] == 6.25, {k: v["value"] for k, v in a["loads"].items() if isinstance(v, dict)})
check("assumptions bilingual", all(x["en"] and x["de"] and x["basis"] for x in a["assumptions"]), [x["id"] for x in a["assumptions"]])
check("losses bilingual with tags", a["losses"] and all(l["source_tag"] and l["en"] and l["de"] for l in a["losses"]), len(a["losses"]))
print([(g["id"], g["count"]) for g in a["loss_groups"]])

for level in ("first", "roof"):
    t = analysis.tributary(m, level)
    slab_area = core.quantities(t["slab_tag"], m)["area"]
    check(f"{level}: areas sum to slab area", math.isclose(t["total_area"], slab_area, rel_tol=1e-9) and t["balanced"], (t["total_area"], slab_area))
    check(f"{level}: total reaction = q x area", math.isclose(t["total_reaction"], t["area_load"] * slab_area, rel_tol=1e-9), (t["total_reaction"], t["area_load"]))
    check(f"{level}: 25 pilotis, kinds", len(t["pilotis"]) == 25 and sorted({p["kind"] for p in t["pilotis"]}) == ["corner", "edge", "interior"])
    check(f"{level}: cell areas = polygon areas", all(math.isclose(p["area"], analysis._area(p["cell"]) if p["cell"] else 0.0, abs_tol=1e-6) for p in t["pilotis"]))
    print(level, "q", t["area_load"], {k: v["value"] for k, v in t["load_breakdown"].items()}, "total", t["total_reaction"], "A", slab_area)
t1 = analysis.tributary(m)
print("kinds", {k: sum(1 for p in t1["pilotis"] if p["kind"] == k) for k in ("interior", "edge", "corner")}, "B3", next(p for p in t1["pilotis"] if p["tag"] == "B3")["area"], "A1", next(p for p in t1["pilotis"] if p["tag"] == "A1")["area"])
roof = analysis.tributary(m, "roof")
print("roof row C", [(p["tag"], p["area"], p["reaction"]) for p in roof["pilotis"] if p["row"] == "C"])

soil = next(e for e in m["elements"] if e["tag"] == "SL-R")["guid"]
new = core.apply_event({"type": "LayerThicknessChanged", "object_guid": soil, "layer": "soil", "value": 0.3, "unit": "m"}, m)
roof_new = analysis.tributary(new, "roof")
check("soil event raises roof load and reactions", roof_new["area_load"] > roof["area_load"] and roof_new["total_reaction"] > roof["total_reaction"], (roof["area_load"], roof_new["area_load"]))
check("first-floor load unchanged by roof event", analysis.tributary(new)["area_load"] == t1["area_load"])
check("source revision follows structure", analysis.translate_structure(new)["source_revision"] == 2 and a["source_revision"] == 1)

clashes = analysis.clash_check(m)
check("exactly one clash", len(clashes) == 1, [(x["a_tag"], x["b_tag"], x["distance"], x["required"], x["point"], x["level"]) for x in clashes])
check("all pairs ok except one", len(analysis.clash_check(m, include_passed=True)) == 4 and sum(1 for x in analysis.clash_check(m, include_passed=True) if x["status"] == "violation") == 1)

for kind in analysis.LINK_KINDS:
    values = analysis.link_values(m, kind)
    check(f"link_values {kind}", bool(values) and all(isinstance(v, (int, float)) for v in values.values()), len(values))
print("issues base", core.consistency_issues(m))
issues = core.consistency_issues(new)
print("issues new", [(i["code"], i.get("key"), i["expected"], i["actual"]) for i in issues][:6], len(issues))
fixed = core.resolve_link(new, "LK-ROOF")
print("issues after resolve", [(i["code"], i["id"]) for i in core.consistency_issues(fixed)])

filled = json.loads(json.dumps(m))
for link in filled["links"]:
    link["used"] = analysis.link_values(filled, link["kind"])
check("filled links are consistent", core.consistency_issues(filled) == [])
changed = core.apply_event({"type": "LayerThicknessChanged", "object_guid": soil, "layer": "soil", "value": 0.3, "unit": "m"}, filled)
codes = [(i["code"], i.get("key")) for i in core.consistency_issues(changed)]
check("roof event makes the roof link stale in revision and values", ("stale-link", None) in codes and ("stale-value", "area_load") in codes and ("stale-value", "reaction_C1") in codes, codes[:5])
check("pilotis with no roof load stay consistent", ("stale-value", "reaction_C4") not in codes)
resolved = core.resolve_link(changed, "LK-ROOF")
check("resolve_link clears issues", core.consistency_issues(resolved) == [])
window = core.apply_event(core.demo_event("sill", filled), filled)
check("opening event leaves analysis links consistent", core.consistency_issues(window) == [], core.consistency_issues(window))
for language in ("en", "de"):
    print(language, [x[language] for x in a["assumptions"] if x["id"] in ("loads", "linear-elastic")])

for level in ("first", "roof"):
    t = analysis.tributary(m, level)
    slab = next(s for s in core.building_geometry(m)["slabs"] if s["tag"] == t["slab_tag"])
    step = 0.025
    xs_, ys_ = t["cuts"]["x"], t["cuts"]["y"]
    sums = {}
    inside = lambda pt, rect: rect[0] < pt[0] < rect[2] and rect[1] < pt[1] < rect[3]
    rects = [[min(p[0] for p in v), min(p[1] for p in v), max(p[0] for p in v), max(p[1] for p in v)] for v in slab["voids"]]
    for ix in range(round(21.5 / step)):
        for iy in range(round(19 / step)):
            pt = ((ix + 0.5) * step, (iy + 0.5) * step)
            if any(inside(pt, r) for r in rects):
                continue
            col = sum(pt[0] > c for c in xs_[1:-1])
            row = sum(pt[1] > c for c in ys_[1:-1])
            sums[(row, col)] = sums.get((row, col), 0) + step * step
    brute = {p["tag"]: sums.get((core.building_geometry(m)["grid"]["y"].index(p["y"]), core.building_geometry(m)["grid"]["x"].index(p["x"])), 0.0) for p in t["pilotis"]}
    check(f"{level}: brute-force sampling agrees with clipped cells", all(abs(brute[p["tag"]] - p["area"]) < 0.01 for p in t["pilotis"]), max(abs(brute[p["tag"]] - p["area"]) for p in t["pilotis"]))
axis_distance = math.hypot(1.40 - 1.25, 0.35 - 0.0)
check("clash distance equals closed form", math.isclose(clashes[0]["distance"], axis_distance - 0.05 - 0.15, abs_tol=1e-8), axis_distance)
print("roof reactions", sorted(((p["reaction"], p["tag"]) for p in roof["pilotis"]), reverse=True)[:8], "zero:", [p["tag"] for p in roof["pilotis"] if p["reaction"] == 0])

zones = {z["id"]: z for z in roof["zones"]}
check("roof zones planted + solarium", set(zones) == {"planted", "solarium"}, {k: (v["area"], v["area_load"]) for k, v in zones.items()})
check("solarium zone is the 70 m2 space", math.isclose(zones["solarium"]["area"], 70.0, abs_tol=1e-6))
check("zone areas sum to roof area", math.isclose(sum(z["area"] for z in zones.values()), roof["slab_area"], rel_tol=1e-9))
check("zone loads", math.isclose(zones["planted"]["area_load"], 5.0 + 0.2 + 1.32 + 1.8 + 1.0) and math.isclose(zones["solarium"]["area_load"], 5.0 + 0.2 + 1.32 + 2.0) and "soil" not in zones["solarium"]["load_breakdown"])
check("mean area load = total / area", math.isclose(roof["area_load"] * roof["slab_area"], roof["total_reaction"], rel_tol=1e-9) and math.isclose(roof["load_breakdown"]["total"]["value"], roof["area_load"], rel_tol=1e-9))
check("per piloti reaction = sum over zones", all(math.isclose(p["reaction"], sum(v["reaction"] for v in p["zones"].values()), rel_tol=1e-9) and math.isclose(p["area"], sum(v["area"] for v in p["zones"].values()), abs_tol=1e-8) for p in roof["pilotis"]))
check("zone outline minus holes = zone area", all(math.isclose(sum(analysis._area(l) for l in z["outlines"]) - sum(analysis._area(l) for l in z["holes"]), z["area"], rel_tol=1e-6) for z in zones.values()), {k: (len(v["outlines"]), len(v["holes"])) for k, v in zones.items()})
rects = [[min(p[0] for p in v), min(p[1] for p in v), max(p[0] for p in v), max(p[1] for p in v)] for v in next(s for s in core.building_geometry(m)["slabs"] if s["kind"] == "ROOF")["voids"]]
sol = [0.5, 4.5, 7.5, 14.5]
step, planted_sampled = 0.025, 0.0
for ix in range(round(21.5 / step)):
    for iy in range(round(19 / step)):
        pt = ((ix + 0.5) * step, (iy + 0.5) * step)
        if not any(r[0] < pt[0] < r[2] and r[1] < pt[1] < r[3] for r in rects + [sol]):
            planted_sampled += step * step
check("planted area by sampling", abs(planted_sampled - zones["planted"]["area"]) < 0.05, (planted_sampled, zones["planted"]["area"]))
before = {p["tag"]: p for p in roof["pilotis"]}
after = {p["tag"]: p for p in roof_new["pilotis"]}
dq = 0.2 * 18.0
check("soil event: reaction change = 3.6 x planted cell area for every piloti", all(math.isclose(after[t]["reaction"] - before[t]["reaction"], dq * before[t]["zones"]["planted"]["area"], abs_tol=1e-6) for t in before))
untouched = [t for t in before if before[t]["zones"]["planted"]["area"] == 0]
check("pilotis without planted cell area keep their reaction", untouched and all(after[t]["reaction"] == before[t]["reaction"] for t in untouched), untouched)
check("solarium zone unchanged by soil", {z["id"]: z for z in roof_new["zones"]}["solarium"]["area_load"] == zones["solarium"]["area_load"])
check("total reaction change = 3.6 x planted area", math.isclose(roof_new["total_reaction"] - roof["total_reaction"], dq * zones["planted"]["area"], rel_tol=1e-9))
print("planted-free pilotis", untouched, "| soil change per piloti", sorted(((round(after[t]["reaction"] - before[t]["reaction"], 1), t) for t in before), reverse=True)[:6])
print("row C", [(t, round(after[t]["reaction"] - before[t]["reaction"], 1)) for t in before if t[0] == "C"])

print("FAILED" if fail else "ALL OK", fail)
sys.exit(1 if fail else 0)
