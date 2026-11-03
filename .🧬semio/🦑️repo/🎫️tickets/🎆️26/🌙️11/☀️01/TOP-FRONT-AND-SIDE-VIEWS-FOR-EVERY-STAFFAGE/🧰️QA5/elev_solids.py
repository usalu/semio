import sys, math
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️H4")
from lib import load, save


def xs_of(s):
    k = s["kind"]
    if k == "rect": return [s["x"], s["x"] + s["w"]]
    if k == "poly": return [p[0] for p in s["points"]]
    if k == "ellipse": return [s["cx"] - s["rx"], s["cx"] + s["rx"]]
    if k == "capsule": return [min(s["x0"] - s["r0"], s["x1"] - s["r1"]), max(s["x0"] + s["r0"], s["x1"] + s["r1"])]
    return None


def affected(a):
    own = set()
    for p in a["poses"]:
        for b, o in p["bones"].items():
            if o.get("abductionDeg") or o.get("twistDeg"): own.add(b)
    for c in a["clips"]:
        for k in c["keys"]:
            for b, o in (k.get("bones") or {}).items():
                if o.get("abductionDeg") or o.get("twistDeg"): own.add(b)
    par = {b["id"]: b.get("parent") for b in a["rig"]["bones"]}
    out = set()
    for b in par:
        c = b
        while c:
            if c in own: out.add(b); break
            c = par[c]
    return out


for aid in sys.argv[1:]:
    doc = load(aid); a = doc["asset"]; aff = affected(a)
    d = a["dimensions"]; limit = max(0.05, 0.1 * max(d["heightM"], d["widthM"], d["depthM"]))
    n = 0
    for sc, rep in a["representations"].items():
        if rep["status"] != "authored": continue
        g = rep["geometry"]
        for s in g["elevation"]:
            s.pop("solid", None) if s.get("solid", {}).get("_qa5") else None
            if s.get("solid") or s.get("role") == "ground" or s["bone"] not in aff: continue
            tw = [f for f in g["front"] if f["id"] == s["id"] and f["bone"] == s["bone"] and "solid" not in f]
            if not tw: continue
            xs = xs_of(tw[0])
            if not xs or max(xs) - min(xs) <= limit: continue
            s["solid"] = {"kind": "extruded", "z0": round(min(xs), 4), "z1": round(max(xs), 4)}
            n += 1
    save(aid, doc); print(aid, "elevation solids", n)
