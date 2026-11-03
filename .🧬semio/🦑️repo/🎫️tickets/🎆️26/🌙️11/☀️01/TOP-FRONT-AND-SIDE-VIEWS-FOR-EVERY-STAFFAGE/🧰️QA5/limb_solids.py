import sys
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️H4")
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️QA5")
from lib import load, save
from elev_solids import xs_of

KEEP = ("kind", "d", "points", "closed", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rotationDeg", "x0", "y0", "x1", "y1", "r0", "r1")
JOBS = {
    "standing-adult": ("500", ["sil-arm-l", "sil-arm-r", "sil-forearm-l", "sil-forearm-r"]),
    "tour-guide": ("500", ["sil-arm-l", "sil-arm-r", "sil-forearm-l", "sil-forearm-r"]),
    "worker-carrying-box": ("500", ["sil-thigh-l", "sil-thigh-r", "sil-shank-l", "sil-shank-r"]),
}
for aid, (sc, ids) in JOBS.items():
    doc = load(aid); g = doc["asset"]["representations"][sc]["geometry"]
    for i, f in enumerate(g["front"]):
        if f["id"] not in ids:
            continue
        swap = f["bone"][:-1] + ("r" if f["bone"].endswith("l") else "l")
        e = next((s for s in g["elevation"] if s["bone"] == f["bone"] and s["role"] == "body"), None) or next((s for s in g["elevation"] if s["bone"] == swap and s["role"] == "body"), None)
        xs = xs_of(f)
        if e is None or not xs:
            print("skip", aid, f["id"]); continue
        n = {k: v for k, v in f.items() if k not in KEEP and k not in ("followTip", "solid")}
        n.update({k: v for k, v in e.items() if k in KEEP})
        n["solid"] = {"kind": "extruded", "z0": round(min(xs), 4), "z1": round(max(xs), 4)}
        g["front"][i] = n
    if aid == "standing-adult":
        for sd in ("l", "r"):
            arm = next(f for f in g["front"] if f["id"] == f"sil-arm-{sd}")
            eh = next(s for s in g["elevation"] if s["id"] == "sil-hand")
            if any(f["id"] == f"sil-hand-{sd}" for f in g["front"]):
                continue
            n = {k: v for k, v in arm.items() if k not in KEEP and k not in ("followTip", "solid", "id", "bone")}
            n.update({k: v for k, v in eh.items() if k in KEEP})
            n.update({"id": f"sil-hand-{sd}", "bone": f"hand-{sd}", "z": 4.55, "solid": dict(arm["solid"])})
            g["front"].append(n)
    save(aid, doc); print(aid, "ok")
