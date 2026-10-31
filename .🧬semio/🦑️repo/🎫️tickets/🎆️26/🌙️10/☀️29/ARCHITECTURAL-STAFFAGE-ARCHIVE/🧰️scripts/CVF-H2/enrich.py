"""1:50 (and hatch at 1:100) enrichment: material hatches, clothing creases computed from each shape's own outline, face detail.
All parameters are authored here per bone/region; coordinates are derived from the shape polygons so every stroke starts on the outline (a join, never a gap)."""
import sys, math, copy
from io_ import *
import geom
def ext_at(sh, x, half=0.01):
    cmds, _ = geom.to_cmds(sh); pts = geom.flat(cmds, 14)
    ys = [y for (px, y) in pts if abs(px - x) <= half]
    if not ys:
        ys = [p[1] for p in sorted(pts, key=lambda p: abs(p[0]-x))[:4]]
    return min(ys), max(ys)
def xrange_of(sh):
    cmds, _ = geom.to_cmds(sh); pts = geom.flat(cmds, 10); xs = [p[0] for p in pts]; return min(xs), max(xs)
def stroke(id_, bone, z, pts, region, tone=0.3):
    return {"id": id_, "bone": bone, "role": "detail", "z": round(z, 3), "filled": False, "tone": tone, "kind": "poly", "points": [[round(x, 4), round(y, 4)] for x, y in pts], "closed": False, "region": region}
HATCH = {"weave": {"id": "weave", "title": T2("Woven fabric", "Webstoff"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1},
         "knit": {"id": "knit", "title": T2("Knitted fabric", "Strickstoff"), "pattern": "lines", "angleDeg": 80, "spacingMm": 0.6, "penMm": 0.1},
         "shell": {"id": "shell", "title": T2("Coated shell fabric", "Beschichtetes Obermaterial"), "pattern": "lines", "angleDeg": -35, "spacingMm": 0.7, "penMm": 0.1}}
CLOTH = {"leg-clothing": "weave", "torso-clothing": "knit", "arm-clothing": "knit", "coat": "shell", "vest": "shell"}
def enrich(name, hatch_scales=(50,), folds=True, face=True, hatch_map=None):
    doc = load(name); a = doc["asset"]; bl = {b["id"]: b["lengthM"] for b in a["rig"]["bones"]}
    hm = dict(CLOTH); hm.update(hatch_map or {})
    for sc in hatch_scales:
        rep = a["representations"][str(sc)]; ids = {h["id"] for h in rep["hatches"]}
        used = set()
        for s in rep["geometry"]["elevation"]:
            if not s["filled"] or s["role"] != "body" or s["kind"] == "ellipse": continue
            h = hm.get(s.get("region"))
            if h and s["bone"].split("-")[0] in ("torso", "upper", "forearm", "thigh", "shank", "pelvis") and not s["id"].startswith("thigh-skin") and not s["id"].startswith("sock") and not s["id"].startswith("sleeve-hem"):
                if sc == 100 and s["bone"].startswith(("upper", "forearm")): continue
                s["hatch"] = h; used.add(h)
        for h in used:
            if h not in ids: rep["hatches"].append(copy.deepcopy(HATCH[h]) if False else dict(HATCH[h]))
    rep = a["representations"]["50"]; els = rep["geometry"]["elevation"]; byid = {s["id"]: s for s in els}; add = []
    if folds:
        for s in list(els):
            if not s["filled"] or s["role"] != "body" or s.get("region") not in CLOTH: continue
            sid, bone, reg = s["id"], s["bone"], s["region"]; L = bl[bone]
            if sid.startswith(("thigh-skin", "sock")): continue
            if bone.startswith("thigh"):
                x0, x1 = xrange_of(s)
                for j, f in enumerate((0.84, 0.91)):
                    x = x0 + (x1 - x0) * f; lo, hi = ext_at(s, x); w = hi - lo
                    add.append(stroke(f"{sid}-crease-{j}", bone, s["z"] + 0.005, [(x - 0.004, lo - 0.002), (x + 0.012, lo + 0.34 * w), (x + 0.03 + 0.01 * j, lo + 0.5 * w)], reg))
                x = x0 + (x1 - x0) * 0.12; lo, hi = ext_at(s, x); w = hi - lo
                add.append(stroke(f"{sid}-hipfold", bone, s["z"] + 0.005, [(x, hi + 0.002), (x + 0.03, hi - 0.4 * w), (x + 0.07, hi - 0.55 * w)], reg))
            elif bone.startswith("shank") and reg == "leg-clothing":
                x0, x1 = xrange_of(s); x = x0 + (x1 - x0) * 0.08; lo, hi = ext_at(s, x); w = hi - lo
                add.append(stroke(f"{sid}-kneefold", bone, s["z"] + 0.005, [(x, hi + 0.002), (x + 0.02, hi - 0.35 * w)], reg))
                x = x0 + (x1 - x0) * 0.9; lo, hi = ext_at(s, x); w = hi - lo
                add.append(stroke(f"{sid}-hemfold", bone, s["z"] + 0.005, [(x, lo - 0.002), (x - 0.02, lo + 0.33 * w)], reg))
            elif bone.startswith("forearm"):
                x0, x1 = xrange_of(s)
                for j, f in enumerate((0.05, 0.14)):
                    x = x0 + (x1 - x0) * f; lo, hi = ext_at(s, x); w = hi - lo
                    add.append(stroke(f"{sid}-elbow-{j}", bone, s["z"] + 0.005, [(x - 0.003, hi + 0.002), (x + 0.012, hi - 0.4 * w), (x + 0.03, hi - 0.55 * w)], reg))
            elif bone.startswith("upper-arm"):
                x0, x1 = xrange_of(s); x = x0 + (x1 - x0) * 0.1; lo, hi = ext_at(s, x); w = hi - lo
                add.append(stroke(f"{sid}-shoulder", bone, s["z"] + 0.005, [(x, lo - 0.002), (x + 0.02, lo + 0.4 * w), (x + 0.045, lo + 0.55 * w)], reg))
            elif bone == "torso" and sid == "torso":
                x0, x1 = xrange_of(s)
                x = x0 + (x1 - x0) * 0.17; lo, hi = ext_at(s, x); w = hi - lo
                add.append(stroke(f"{sid}-waistfold-0", bone, s["z"] + 0.005, [(x, lo - 0.002), (x + 0.012, lo + 0.28 * w), (x + 0.035, lo + 0.42 * w)], reg))
                add.append(stroke(f"{sid}-waistfold-1", bone, s["z"] + 0.005, [(x + 0.025, hi + 0.002), (x + 0.04, hi - 0.25 * w), (x + 0.065, hi - 0.38 * w)], reg))
                x = x0 + (x1 - x0) * 0.72; lo, hi = ext_at(s, x); w = hi - lo
                add.append(stroke(f"{sid}-chestfold", bone, s["z"] + 0.005, [(x, lo - 0.002), (x + 0.015, lo + 0.3 * w), (x + 0.04, lo + 0.42 * w)], reg))
    if face and "head" in byid:
        h = byid["head"]
        add.append(stroke("nostril", "head", 5.65, [(0.1505, -0.1075), (0.1445, -0.1015), (0.1395, -0.1012)], "head-skin", 0.2))
        add.append(stroke("jaw-line", "head", 5.65, [(0.071, -0.0075), (0.092, -0.052), (0.118, -0.0805)], "head-skin", 0.5))
        add.append(stroke("lower-lip", "head", 5.65, [(0.1165, -0.0905), (0.1085, -0.0838)], "head-skin", 0.3))
    used_ids = set(byid)
    for s in add:
        if s["id"] not in used_ids: els.append(s); used_ids.add(s["id"])
    save(name, doc); return len(add)
if __name__ == "__main__":
    for n in sys.argv[1:]: print(n, enrich(n))
