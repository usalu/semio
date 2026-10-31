# [scratch CV-A] structural self check of the six v2 animal files (stand-in until the catalog check CLI supports v2)
import json, sys
from cva import *
bad = 0
def err(a, m):
    global bad; bad += 1; print("  ERR", a, m)
for aid in ["labrador-dog", "domestic-cat", "horse", "pigeon", "roe-deer", "dairy-cow"]:
    a = json.load(open(f"{S}/🗂️catalog/🐕️animals/{aid}/🔣️.json"))["asset"]
    bones = {b["id"] for b in a["rig"]["bones"]}; regs = {r["id"]: r for r in a["thermal"]["regions"]}
    refs = {r["id"] for r in a["provenance"]["references"]}
    th = a["thermal"]; pats = [p["id"] for p in th["respiration"]["patterns"]]
    em = {e["id"] for e in th["emitters"]}; nb = {e["id"] for e in th["emitters"] if e["kind"] != "breath"}
    anchors = {x["id"] for x in a["anchors"]}
    def walk(x, path=""):
        if isinstance(x, dict):
            if "reference" in x and x["reference"] not in refs: err(aid, f"ref {x['reference']} at {path}")
            for k, v in x.items(): walk(v, path+"/"+k)
        elif isinstance(x, list):
            for i, v in enumerate(x): walk(v, path+f"/{i}")
    walk(a)
    if abs(sum(r["body"]["metabolicShare"] for r in regs.values() if "body" in r)-1) > 1e-6: err(aid, "meta")
    if abs(sum(r["body"]["skinFraction"] for r in regs.values() if "body" in r)-1) > 1e-6: err(aid, "skin")
    tot = sum(r["areaM2"] for r in regs.values()); print(aid, "area", round(tot, 3), "body skin", a["thermal"]["body"]["skinAreaM2"])
    for e in th["emitters"]:
        if e["anchor"] not in anchors: err(aid, "emitter anchor " + e["anchor"])
        if "region" in e and e["region"] not in regs: err(aid, "emitter region")
    if th["respiration"].get("noseEmitter") and th["respiration"]["noseEmitter"] not in em: err(aid, "nose emitter")
    if th["respiration"].get("mouthEmitter") and th["respiration"]["mouthEmitter"] not in em: err(aid, "mouth emitter")
    for m in th["respiration"]["motion"]:
        if m["bone"] not in bones: err(aid, "motion bone")
    for p in a["poses"]:
        t = p["thermal"]
        if set(t["regions"]) != set(regs): err(aid, f"pose {p['id']} regions")
        if set(t["emitters"]) != nb: err(aid, f"pose {p['id']} emitters")
        if t["breathing"] not in pats: err(aid, f"pose {p['id']} breathing")
        for c in p["contacts"]:
            if c not in anchors: err(aid, "contact")
    for c in a["clips"]:
        for t in c["tracks"]:
            tg = t["target"]
            if tg.startswith("emitter:") and tg[8:] not in em: err(aid, f"clip {c['id']} track {tg}")
            if tg.startswith("region:") and tg.split(":")[1] not in regs: err(aid, f"clip {c['id']} track {tg}")
            if tg == "breathing" and max(k["v"] for k in t["keys"]) >= len(pats): err(aid, "breathing index")
        for e in c["events"]:
            if e["kind"] in ("inhale", "exhale"): err(aid, "event")
    dm = a["dimensions"]
    pose = a["presentation"]["defaultPose"]; W = pose_world(a | {"geometry": {}}, pose)
    for k, rep in a["representations"].items():
        if rep["status"] == "unavailable":
            print("  ", k, "unavailable", rep["alternatives"]); continue
        sh = rep["geometry"]["elevation"]; pens = rep["pens"]
        used = set()
        for s in sh:
            if s["bone"] not in bones: err(aid, f"{k} {s['id']} bone")
            if pens[s["role"]] is None: err(aid, f"{k} {s['id']} role {s['role']} has null pen")
            for r in ([s["region"]] if "region" in s else []) + s.get("regions", []):
                if r not in regs: err(aid, f"{k} {s['id']} region {r}")
                used.add(r)
            if s["filled"] and s["role"] == "body" and not (s.get("region") or s.get("regions")): err(aid, f"{k} {s['id']} unbound")
        miss = set(regs) - used
        if miss: err(aid, f"{k} regions not drawn {sorted(miss)}")
        bb = bbox_of(sh, W); w_, h_ = bb[2]-bb[0], bb[3]-max(bb[1], 0)
        okw = abs(w_-dm["widthM"]) <= max(0.02*dm["widthM"], 0.015); okh = abs(h_-dm["heightM"]) <= max(0.02*dm["heightM"], 0.015)
        print("  ", k, len(sh), "shapes", f"w {w_:.3f}/{dm['widthM']} h {h_:.3f}/{dm['heightM']}", "ok" if okw and okh else "DIM-MISMATCH")
        if not (okw and okh): bad += 1
print("errors", bad)
