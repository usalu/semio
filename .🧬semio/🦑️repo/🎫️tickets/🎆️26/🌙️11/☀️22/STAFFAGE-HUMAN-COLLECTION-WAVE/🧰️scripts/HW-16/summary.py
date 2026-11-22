"""📋 Prints a structural summary of a human asset: keys, bones, poses, clips, materials, regions, shapes and solids per scale."""
import json, sys
H = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
for slug in sys.argv[1:]:
    a = json.load(open(f"{H}/{slug}/🔣️.json"))["asset"]
    print("=====", slug, list(a.keys()))
    print("dimensions", a.get("dimensions"))
    print("bones", [(b["id"], b.get("parent")) for b in a["rig"]["bones"]])
    print("rig keys", list(a["rig"].keys()))
    print("poses", [(p["id"], p.get("key", False)) for p in a["poses"]])
    print("clips", [(c["id"], c.get("durationS"), c.get("loop")) for c in a["clips"]])
    print("materials", [m["id"] for m in a["materials"]])
    print("thermal keys", list(a["thermal"].keys()))
    print("regions", [(r["id"], r.get("model"), r.get("areaM2")) for r in a["thermal"]["regions"]])
    print("emitters", [(e["id"], e.get("kind")) for e in a["thermal"].get("emitters", [])])
    print("structure keys", list(a["structure"].keys()))
    print("presentation", a.get("presentation"))
    for k, r in a["representations"].items():
        if r.get("status") != "authored": print("repr", k, r.get("status")); continue
        g = r["geometry"]
        print("repr", k, {v: len(g.get(v, [])) for v in g}, "keys", list(r.keys()))
    for k, v in a["volumes"].items():
        print("vol", k, v.get("status"), {m: len(v.get(m, [])) for m in ("body", "thermal", "structure")})
    print("sizes", {k: len(json.dumps(v)) for k, v in a.items()})
