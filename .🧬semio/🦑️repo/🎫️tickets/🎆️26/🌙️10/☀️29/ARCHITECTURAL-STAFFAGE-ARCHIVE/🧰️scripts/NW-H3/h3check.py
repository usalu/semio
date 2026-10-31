"""Local mirror of the cheap catalog rules (extent, joints, loops, drawn regions, fractions, masses, hatch/pen references) so that the
shared-machine official check is needed only a few times. The official check stays authoritative."""
from h3lib import *


def merged_bones(asset, key, poses):
    pose = poses[key["pose"]]
    m = {}
    for bid, o in (pose["bones"]).items():
        m[bid] = dict(o)
    for bid, o in (key.get("bones") or {}).items():
        m.setdefault(bid, {}).update(o)
    return m


def check(asset, verbose=True):
    errs = []
    a = asset
    poses = {p["id"]: p for p in a["poses"]}
    rids = [r["id"] for r in a["thermal"]["regions"]]
    hidden = {r["id"] for r in a["thermal"]["regions"] if r.get("hidden")}
    dimh, dimw = a["dimensions"]["heightM"], a["dimensions"]["widthM"]
    dp = poses[a["presentation"]["defaultPose"]]
    W = fk(a, {"bones": dp["bones"], "root": {"y": ground_y(a, dp["bones"], dp["contacts"])}})
    for k, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        els = rep["geometry"]["elevation"]
        bb = bbox_world(els, W)
        h, w = bb[3] - bb[1], bb[2] - bb[0]
        ok = lambda x, n: abs(x - n) <= max(0.02 * n, 0.015)
        if not ok(h, dimh): errs.append(f"{k}: height {h:.3f} vs {dimh}")
        if not ok(w, dimw): errs.append(f"{k}: width {w:.3f} vs {dimw}")
        drawn = set()
        for s in els:
            drawn.add(s.get("region"))
            drawn |= set(s.get("regions") or [])
            if rep["pens"].get(s["role"]) is None and not s.get("penMm"): errs.append(f"{k}: shape {s['id']} role {s['role']} undrawn")
            if s.get("hatch") and s["hatch"] not in [h_["id"] for h_ in rep["hatches"]]: errs.append(f"{k}: hatch {s['hatch']}")
        for r in rids:
            if r not in drawn and r not in hidden: errs.append(f"{k}: region {r} undrawn")
        ids = [s["id"] for s in els]
        if len(ids) != len(set(ids)): errs.append(f"{k}: duplicate ids")
        for s in els:
            if s["bone"] not in {b["id"] for b in a["rig"]["bones"]}: errs.append(f"{k}: bone {s['bone']}")
        if verbose: print(f"  rep {k}: {len(els)} shapes, h {h:.3f} w {w:.3f} bbox {[round(v,3) for v in bb]}")
    for p in a["poses"]:
        ji = joint_issues(a, p["bones"])
        if ji: errs.append(f"pose {p['id']}: joints {ji}")
        t = p["thermal"]
        for r in rids:
            if r not in t["regions"]: errs.append(f"pose {p['id']}: region {r} unbound")
    for c in a["clips"]:
        for i, key in enumerate(c["keys"]):
            ji = joint_issues(a, merged_bones(a, key, poses))
            if ji: errs.append(f"clip {c['id']} key {i}: {ji}")
        if c["loop"] and len(c["keys"]) > 1 and c["keys"][-1]["t"] >= c["durationS"]:
            h0, h1 = merged_bones(a, c["keys"][0], poses), merged_bones(a, c["keys"][-1], poses)
            for b in a["rig"]["bones"]:
                if abs(h0.get(b["id"], {}).get("angleDeg", 0) - h1.get(b["id"], {}).get("angleDeg", 0)) > 1e-9 or abs(h0.get(b["id"], {}).get("dy", 0) - h1.get(b["id"], {}).get("dy", 0)) > 1e-9 or abs(h0.get(b["id"], {}).get("dx", 0) - h1.get(b["id"], {}).get("dx", 0)) > 1e-9:
                    errs.append(f"clip {c['id']}: loop closing {b['id']}")
    th = a["thermal"]
    bodyr = [r for r in th["regions"] if r.get("body") and not r["body"].get("subject")]
    if abs(sum(r["body"]["metabolicShare"] for r in bodyr) - 1) > 1e-3: errs.append("metabolic share")
    if abs(sum(r["body"]["skinFraction"] for r in bodyr) - 1) > 1e-3: errs.append("skin fraction")
    m = total_mass(a["rig"])
    if abs(m - a["dimensions"]["massKg"]) > 0.01: errs.append(f"mass {m} vs {a['dimensions']['massKg']}")
    for pat in th["respiration"]["patterns"]:
        if abs(pat["inhaleFraction"] + pat["pauseAfterInhaleFraction"] + pat["exhaleFraction"] + pat["pauseAfterExhaleFraction"] - 1) > 1e-6: errs.append("phase " + pat["id"])
    print("LOCAL CHECK:", "OK" if not errs else f"{len(errs)} problems")
    for e in errs[:40]: print("  -", e)
    return errs
