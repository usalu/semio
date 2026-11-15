import json, glob, os, sys, collections, statistics
ROOT = sys.argv[1]
OUT = sys.argv[2]
SCALES = ["50", "100", "200", "500"]
VPS = ["elevation", "front", "plan"]
def load(p):
    return json.load(open(p, encoding="utf-8"))
def sig(shape):
    return json.dumps({k: v for k, v in shape.items() if k != "id"}, sort_keys=True, ensure_ascii=False)
def compare(coarse, fine):
    a = [(vp, s) for vp in VPS for s in coarse["geometry"][vp]]
    b = [(vp, s) for vp in VPS for s in fine["geometry"][vp]]
    sa = collections.Counter(vp + "|" + sig(s) for vp, s in a)
    sb = collections.Counter(vp + "|" + sig(s) for vp, s in b)
    shared = sum(min(sa[k], sb[k]) for k in sa)
    if sa == sb: return "identical", shared, len(a)
    if len(a) > 0 and shared == len(a): return "subset", shared, len(a)
    return "distinct", shared, len(a)
records = []
for p in sorted(glob.glob(os.path.join(ROOT, "*", "*", "🔣️.json"))):
    cat = os.path.basename(os.path.dirname(os.path.dirname(p)))
    doc = load(p)
    asset = doc["asset"] if "asset" in doc else doc
    r = {"cat": cat, "id": asset["id"], "path": os.path.relpath(p, ROOT), "scales": {}, "copies": []}
    for s in SCALES:
        rep = asset["representations"][s]
        if rep["status"] == "authored":
            g = rep["geometry"]
            shapes = [sh for vp in VPS for sh in g[vp]]
            roles = collections.Counter(sh["role"] for sh in shapes)
            kinds = collections.Counter(sh["kind"] for sh in shapes)
            regs = set()
            for sh in shapes:
                if "region" in sh: regs.add(sh["region"])
                for x in sh.get("regions", []): regs.add(x)
            r["scales"][s] = {"status": "authored", "vp": {vp: len(g[vp]) for vp in VPS}, "total": len(shapes), "roles": dict(roles), "kinds": dict(kinds), "regions": sorted(regs), "hatches": len(rep["hatches"]), "sections": sorted((rep.get("sections") or {}).keys()), "member": kinds.get("member", 0), "solid": sum(1 for sh in shapes if "solid" in sh), "ids": [sh["id"] for sh in shapes], "validRange": rep["validRange"], "pens": rep["pens"], "bodyPen": rep["pens"]["body"], "detailPen": rep["pens"]["detail"], "decisions": rep["decisions"], "_rep": rep}
        else:
            r["scales"][s] = {"status": "unavailable", "cause": rep["cause"], "limitingFeature": rep["limitingFeature"]["en"], "limitingSizeM": rep["limitingSizeM"], "alternatives": [a["kind"] for a in rep["alternatives"]]}
    auth = [s for s in SCALES if r["scales"][s]["status"] == "authored"]
    for i, c in enumerate(auth):
        for f in auth[i + 1:]:
            kind, shared, n = compare(r["scales"][c]["_rep"], r["scales"][f]["_rep"])
            r["copies"].append({"coarse": c, "fine": f, "kind": kind, "shared": shared, "coarseShapes": n})
    for s in SCALES:
        if "_rep" in r["scales"].get(s, {}): del r["scales"][s]["_rep"]
    v = asset.get("volume")
    if v is None: r["volume"] = None
    else:
        sol = v["solids"]
        r["volume"] = {"status": v["status"], "solids": len(sol), "maxDen": dict(collections.Counter(str(x["maxDenominator"]) for x in sol if "maxDenominator" in x)), "noMaxDen": sum(1 for x in sol if "maxDenominator" not in x), "region": sum(1 for x in sol if "region" in x), "regions": sum(1 for x in sol if "regions" in x), "withRegionOrRegions": sum(1 for x in sol if "region" in x or "regions" in x), "kinds": dict(collections.Counter(x["kind"] for x in sol)), "roles": dict(collections.Counter(x["role"] for x in sol)), "decisions": len(v["decisions"]), "solidsRegionIds": sorted({x["region"] for x in sol if "region" in x} | {y for x in sol for y in x.get("regions", [])})}
    t = asset["thermal"]
    r["thermal"] = {"regions": len(t["regions"]), "emitters": len(t["emitters"]), "passengers": len(t.get("passengers", [])), "operating": len(t.get("operating", [])), "kinds": dict(collections.Counter(x["model"] for x in t["regions"])), "layersPerRegion": [len(x["layers"]) for x in t["regions"]], "hidden": sum(1 for x in t["regions"] if "hidden" in x)}
    st = asset["structure"]
    r["structure"] = {"supports": len(st["supports"]), "poseLoads": sum(len(p.get("loads", [])) for p in asset["poses"]), "dynamics": st["dynamics"]}
    r["presentation_defaultScale"] = asset["presentation"]["defaultScale"]
    records.append(r)
json.dump(records, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("assets", len(records))
