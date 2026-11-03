import sys
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️H4")
from lib import load, save

KEEP = ("kind", "d", "points", "closed", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rotationDeg", "x0", "y0", "x1", "y1", "r0", "r1")
PARTS = {
    "duvet-chest": (-0.19, 0.19), "duvet-hips": (-0.185, 0.185), "duvet-thighs": (-0.17, 0.17), "duvet-shins": (-0.15, 0.15),
    "head": (-0.075, 0.075), "hair": (-0.078, 0.078), "neck": (-0.04, 0.04),
    "sleeve-r": (-0.2, -0.1), "forearm-r": (-0.19, -0.11), "hand-r": (-0.18, -0.12),
}
LEFT = {"sleeve-r": "sleeve-l", "forearm-r": "forearm-l", "hand-r": "hand-l"}
BONES = {"sleeve-r": "arm-l", "forearm-r": "forearm-l", "hand-r": "hand-l"}
doc = load("sleeping-person"); a = doc["asset"]
for sc, rep in a["representations"].items():
    if rep["status"] != "authored":
        continue
    g = rep["geometry"]
    elev = {s["id"]: s for s in g["elevation"]}
    todo = [(i, s) for i, s in elev.items() if i in PARTS]
    for v in ("front", "plan"):
        drop_bones = {s["bone"] for _, s in todo} | {"arm-l", "forearm-l", "hand-l"}
        g[v] = [s for s in g[v] if not (s["bone"] in drop_bones and s["role"] == "body" and not s["id"].startswith(("pillow", "duvet-skirt", "foot", "sil-foot")))]
    for i, s in todo:
        z0, z1 = PARTS[i]
        s["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}
        for v in ("front", "plan"):
            n = {k: val for k, val in s.items() if k not in ("solid",)}
            n["solid"] = dict(s["solid"])
            g[v].append(n)
        if i in LEFT:
            for v in ("front", "plan"):
                n = {k: val for k, val in s.items() if k != "solid"}
                n.update({"id": LEFT[i], "bone": BONES[i], "solid": {"kind": "extruded", "z0": -z1, "z1": -z0}})
                g[v].append(n)
    for k in list(a["representations"]):
        pass
save("sleeping-person", doc); print("sleeper 3d ok")
