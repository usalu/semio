import sys; sys.path.insert(0,".")
from lib import *
cat, id = sys.argv[1:3]
a = asset(load(cat), id)
what = sys.argv[3:] or ["bones","geo","poses","anchors"]
if "bones" in what:
    for b in a["rig"]["bones"]: print("B", b["id"], b["parent"], b["x"], b["y"], b["angleDeg"], b["lengthM"], json.dumps(b["joint"]), (b.get("mass") or {}).get("kg"))
if "geo" in what:
    for s in a["geometry"]["elevation"]:
        d={k:v for k,v in s.items() if k not in("id","bone","role","lod","z","filled","tone","region","material")}
        print("S", s["id"], s["bone"], s["role"], s["lod"], s["z"], s["region"] if "region" in s else "", json.dumps(d)[:150])
if "poses" in what:
    for p in a["poses"]: print("P", p["id"], p.get("key"), json.dumps(p["bones"]), p.get("root"), p["contacts"], [ (l["id"],l.get("fyN"),l.get("fxN"),l.get("bone")) for l in p["loads"]])
if "clips" in what:
    for c in a["clips"]: print("C", c["id"], c["durationS"], [(k["t"],k["pose"],k.get("bones")) for k in c["keys"]], [t["target"] for t in c["tracks"]])
if "anchors" in what:
    for n in a["anchors"]: print("A", json.dumps(n,ensure_ascii=False)[:260])
if "thermal" in what:
    for r in a["thermal"]["regions"]: print("R", r["id"], r["areaM2"], r.get("model"))
    print("E", [e["id"] for e in a["thermal"]["emitters"]])
if "materials" in what:
    for m in a["materials"]: print("M", m["id"])
