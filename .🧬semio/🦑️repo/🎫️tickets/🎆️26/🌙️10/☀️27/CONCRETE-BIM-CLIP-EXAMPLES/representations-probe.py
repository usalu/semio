"""Probe model values the representations scene reads."""
import importlib, json, os, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
b = M.load_model()
for k in ("project", "site", "building"):
    print("[DEBUG]", k, json.dumps(b[k], ensure_ascii=False)[:400])
print("[DEBUG] storeys", json.dumps(b["storeys"], ensure_ascii=False)[:600])
e = M.element("W-S", b)
print("[DEBUG] wall", json.dumps({k: v for k, v in e.items() if k != "material"}, ensure_ascii=False)[:1500])
print("[DEBUG] layers", json.dumps(e["material"], ensure_ascii=False)[:900])
for kind in ("axis", "footprint", "body", "plan", "section", "elevation", "3d"):
    r = M.represent("W-S", kind, b)
    print("[DEBUG] rep", kind, json.dumps(r, ensure_ascii=False)[:900])
print("[DEBUG] q", M.quantities("W-S", b))
print("[DEBUG] ev", M.demo_event("sill", b))
g = M.building_geometry(b)
print("[DEBUG] wall geo", [w for w in g["walls"] if w["tag"] == "W-S"])
print("[DEBUG] storeys geo", g["storeys"], [ (s["tag"], s["top"], s["thickness"]) for s in g["slabs"]])
print("[DEBUG] rel", json.dumps(M.relations_of("W-S", b), ensure_ascii=False)[:1200])
print("[DEBUG] info", M.information("W-S", "Pset_WallCommon.IsExternal", "en", b))
w = M.element("RW-S", b)
print("[DEBUG] window name", w["name"], w["storey"])
for pid in ("Pset_WallCommon.FireRating", "Pset_WallCommon.LoadBearing"):
    try:
        print("[DEBUG] info", M.information("W-S", pid, "de", b))
    except Exception as exc:
        print("[DEBUG] info err", pid, exc)
print("[DEBUG] type", M.element("TY-WALL", b)["name"])
