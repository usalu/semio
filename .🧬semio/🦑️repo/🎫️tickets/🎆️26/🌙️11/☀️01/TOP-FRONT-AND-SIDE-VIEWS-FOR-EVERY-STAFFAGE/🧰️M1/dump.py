import json, sys, io, os
S = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
OUT = r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\M1"
os.makedirs(OUT, exist_ok=True)


def dump(aid):
    d = json.load(open(os.path.join(S, aid, "🔣️.json"), encoding="utf8"))
    a = d["asset"]
    L = []
    w = L.append
    w("DIM " + json.dumps(a["dimensions"]))
    w("PRES " + json.dumps(a["presentation"], ensure_ascii=False))
    for b in a["rig"]["bones"]:
        j = b.get("joint", {})
        w("BONE %s parent=%s x=%s y=%s ang=%s len=%s joint=%s" % (b["id"], b["parent"], b["x"], b["y"], b["angleDeg"], b["lengthM"], json.dumps(j)))
    for p in a["poses"]:
        w("POSE %s key=%s root=%s bones=%s" % (p["id"], p.get("key"), json.dumps(p.get("root")), json.dumps(p["bones"])))
    for c in a["clips"]:
        w("CLIP %s dur=%s loop=%s rootMotion=%s" % (c["id"], c["durationS"], c["loop"], json.dumps(c.get("rootMotion"))))
        for k in c["keys"]:
            w("   KEY " + json.dumps(k, ensure_ascii=False)[:900])
        for t in c.get("tracks", []) or []:
            w("   TRACK " + json.dumps(t, ensure_ascii=False)[:300])
    for sc, r in a["representations"].items():
        w("REP %s status=%s" % (sc, r["status"]))
        if r["status"] != "authored":
            continue
        for k in ("pens", "hatches", "validRange"):
            w("  %s %s" % (k, json.dumps(r.get(k), ensure_ascii=False)))
        for k, v in r["decisions"].items():
            w("  DEC %s en: %s" % (k, v["en"]))
        for s in r["geometry"]["elevation"]:
            w("  EL " + json.dumps(s, ensure_ascii=False, separators=(",", ":")))
    open(os.path.join(OUT, aid + ".txt"), "w", encoding="utf8").write("\n".join(L) + "\n")


for aid in sys.argv[1:]:
    dump(aid)
