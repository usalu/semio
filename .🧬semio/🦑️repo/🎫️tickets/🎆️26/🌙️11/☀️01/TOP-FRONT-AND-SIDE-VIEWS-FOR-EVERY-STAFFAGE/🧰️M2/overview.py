import json, sys, os
root = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
out = r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\M2"
os.makedirs(out, exist_ok=True)
def j(x): return json.dumps(x, ensure_ascii=False, separators=(",", ":"))
for aid in sys.argv[1:]:
    d = json.load(open(os.path.join(root, aid, "🔣️.json"), encoding="utf-8"))["asset"]
    L = []
    L.append("DIMS " + j(d["dimensions"]) + " default " + j(d["presentation"]))
    L.append("ROOT " + str(d["rig"]["root"]))
    for b in d["rig"]["bones"]:
        L.append("BONE " + j({k: b[k] for k in ("id", "parent", "x", "y", "angleDeg", "lengthM", "joint") if k in b}))
    L.append("ANCHORS " + j([{k: a[k] for k in ("id", "bone", "x", "y") if k in a} for a in d["anchors"]]))
    for p in d["poses"]:
        L.append("POSE " + p["id"] + " bones=" + j(p.get("bones")) + " ground=" + str(p.get("ground")) + " root=" + j(p.get("root")))
    for c in d["clips"]:
        bt = [t for t in c.get("tracks", []) if t["target"].startswith("bone")]
        L.append("CLIP " + c["id"] + " dur=" + str(c["durationS"]) + " loop=" + str(c.get("loop")) + " rootMotion=" + str(c.get("rootMotion")) + " keys=" + j(c.get("keys")) + " boneTracks=" + j(bt))
        L.append("   targets " + j(sorted({t["target"] for t in c.get("tracks", [])})))
    for sc, r in d["representations"].items():
        L.append("REP " + sc + " " + r["status"])
        if r["status"] != "authored":
            continue
        g = r["geometry"]
        L.append("  decisions.silhouette " + j(r["decisions"]["silhouette"]["en"]) + " pens " + j(r["pens"]) + " hatches " + j([h["id"] for h in r["hatches"]]))
        for s in g["elevation"]:
            L.append("  E " + j(s)[:420])
    open(os.path.join(out, aid + ".txt"), "w", encoding="utf-8").write("\n".join(L))
    print(aid, len(L))
