"""[DEBUG] probe the consistency model data for the Round 2 scene rebuild."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
base = m.load_model()
ev = m.demo_event("roof_soil", base)
new = m.apply_event(ev, base)
alt = m.apply_event(m.demo_event("roof_soil_alt", base), base)
tb, tn, ta = m.tributary(base, "roof"), m.tributary(new, "roof"), m.tributary(alt, "roof")
print("[DEBUG] q", [z["area_load"] for z in tb["zones"]], [z["area_load"] for z in tn["zones"]], [z["area_load"] for z in ta["zones"]])
ob = {p["tag"]: p for p in tb["pilotis"]}
delta = sorted(((p["reaction"] - ob[p["tag"]]["reaction"], p["tag"], round(p["area"], 2)) for p in tn["pilotis"]), reverse=True)
print("[DEBUG] delta", [(round(d, 1), t, a) for d, t, a in delta])
print("[DEBUG] sum", sum(d for d, _, _ in delta), tn["total_reaction"] - tb["total_reaction"])
g = m.building_geometry(base)
print("[DEBUG] slab keys", [(s["tag"], s["kind"], list(s.keys())) for s in g["slabs"]][:2])
sl = next(s for s in g["slabs"] if s["kind"] == "ROOF")
print("[DEBUG] voids", sl.get("voids"))
print("[DEBUG] ids", {k: v for k, v in m.ids_check(base, "IfcSlab Pset_SlabCommon.LoadBearing exists").items() if k in ("expected", "found", "checked", "passed", "failed", "passed_tags")})
an = m.translate_structure(new)
print("[DEBUG] assumptions", [(a["id"], a["en"], a["de"]) for a in an["assumptions"]])
print("[DEBUG] loss groups", [(a.get("id"), a.get("en")) for a in an["loss_groups"]][:4])
print("[DEBUG] counts", an["counts"])
for t in ("A3", "C3", "D3"):
    p = next(q for q in tn["pilotis"] if q["tag"] == t)
    xs = [c[0] for c in p["cell"]]
    ys = [c[1] for c in p["cell"]]
    print("[DEBUG] cell", t, min(xs), max(xs), min(ys), max(ys), p["cell"], p["pieces"])
print("[DEBUG] meta", {x["id"]: (x["author"], x["revision"]) for x in base["models"]} if isinstance(base["models"], list) else {k: (v["author"], v["revision"]) for k, v in base["models"].items()})
print("[DEBUG] new revs", [m.partial_model(k, new)["revision"] for k in ("architecture", "structure", "services")], m.element("SL-R", new)["metadata"])
print("[DEBUG] total", tb["total_reaction"], tn["total_reaction"], ta["total_reaction"], tb["slab_area"], tn["slab_area"])
print("[DEBUG] pset", m.element("SL-R", base)["psets"])
print("[DEBUG] step", [r for r in m.step_excerpt("SL-R")["lines"] if "IFCSLAB" in r])
