import json, sys, math
from dump import load
sys.stdout.reconfigure(encoding="utf8")
IDS = "japanese-maple lombardy-poplar london-plane-pollarded miscanthus-grass multi-stem-birch scots-pine small-leaved-lime weeping-willow".split()

def restworld(a):
    w = {}
    for b in a["rig"]["bones"]:
        p = b["parent"]
        if p is None:
            w[b["id"]] = (b["x"], b["y"], b["angleDeg"])
        else:
            px, py, pa = w[p]
            r = math.radians(pa)
            w[b["id"]] = (px + b["x"] * math.cos(r) - b["y"] * math.sin(r), py + b["x"] * math.sin(r) + b["y"] * math.cos(r), pa + b["angleDeg"])
    return w

for i in IDS:
    a = load("🌳️vegetation", i)
    print("=====", i, a["dimensions"], a["presentation"])
    w = restworld(a)
    for b in a["rig"]["bones"]:
        x, y, ang = w[b["id"]]
        print("  bone %-14s parent %-12s world(%.2f,%.2f) ang %.1f len %.2f" % (b["id"], b["parent"], x, y, ang, b["lengthM"]))
    print("  poses", [p["id"] for p in a["poses"]], "clips", [(c["id"], c.get("durationS")) for c in a["clips"]])
    for sc in ["50", "100", "200", "500"]:
        r = a["representations"][sc]
        g = r["geometry"]["elevation"]
        sh = g["shapes"] if isinstance(g, dict) else g
        print("  scale", sc, r["status"], "pens", r["pens"], "hatches", [h["id"] for h in r["hatches"]], "n", len(sh))
        for s in sh:
            extra = ""
            if s["kind"] == "path":
                extra = "pathlen %d" % len(s["d"])
            print("     %-22s bone %-12s role %-7s z %-5s tone %-5s %-7s %s reg %s mat %s %s" % (s["id"], s["bone"], s["role"], s.get("z"), s.get("tone"), s["kind"], "F" if s.get("filled") else "-", s.get("region") or s.get("regions"), s.get("material"), extra))
    print("  symbols", [(s["id"], s["denominators"], list(s["geometry"].keys())) for s in a["symbols"]])
