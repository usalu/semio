import sys
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️H4")
from lib import load, save

d = load("standing-adult")
for s in d["asset"]["representations"]["500"]["geometry"]["elevation"]:
    if s["id"] == "sil-hand":
        s["d"] = "M -0.014 -0.065 L 0.17 -0.065 C 0.195 -0.065 0.195 0.065 0.17 0.065 L -0.014 0.065 C -0.039 0.065 -0.039 -0.065 -0.014 -0.065 Z"
save("standing-adult", d)

d = load("standing-phone-user")
for s in d["asset"]["representations"]["500"]["geometry"]["elevation"]:
    if s["id"] == "phone" and s["kind"] == "rect":
        s["y"], s["h"] = -0.05, 0.1
save("standing-phone-user", d)

d = load("sleeping-person")
for sc, rep in d["asset"]["representations"].items():
    if rep["status"] != "authored":
        continue
    for s in rep["geometry"]["elevation"]:
        if s["id"] == "ear" and s["kind"] == "ellipse":
            s["rx"], s["ry"] = 0.034, 0.055
        if s["id"] == "eye" and s["kind"] == "ellipse":
            s["rx"], s["ry"] = 0.016, 0.012
save("sleeping-person", d)
print("features ok")

KEEP = ("kind", "d", "points", "closed", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rotationDeg", "x0", "y0", "x1", "y1", "r0", "r1")
d = load("standing-phone-user")
for sc, rep in d["asset"]["representations"].items():
    if rep["status"] != "authored":
        continue
    g = rep["geometry"]
    e = next((s for s in g["elevation"] if s["bone"] == "phone" and s["role"] == "body"), None)
    if e is None:
        print("no elevation phone at", sc); continue
    zr = (-0.19, -0.116)
    for v in ("front", "plan"):
        for i, f in enumerate(g[v]):
            if f["id"] == "phone":
                n = {k: val for k, val in f.items() if k not in KEEP and k not in ("followTip", "solid")}
                n.update({k: val for k, val in e.items() if k in KEEP})
                n["solid"] = {"kind": "extruded", "z0": zr[0], "z1": zr[1]}
                g[v][i] = n
save("standing-phone-user", d)
print("phone solids ok")

d = load("standing-phone-user")
g = d["asset"]["representations"]["500"]["geometry"]
for v in ("front", "plan"):
    g[v] = [s for s in g[v] if s["id"] != "phone"]
save("standing-phone-user", d)
print("phone merged into hand at 1:500")
