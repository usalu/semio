import importlib.util, re
spec = importlib.util.spec_from_file_location("lib", "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc(); A = "horse"
def shift(d, dx):
    out = []; xs = True
    for t in d.split():
        if re.match(r"^[MLCZ]$", t): out.append(t); xs = True; continue
        v = float(t)
        if xs: v = round(v + dx, 4)
        out.append(f"{v:g}"); xs = not xs
    return " ".join(out)
THIGH_OLD = "L 0.173 -0.1 L 0.063 -0.291 Z"
THIGH_NEW = "C 0.3 -0.005 0.205 -0.045 0.173 -0.1 C 0.13 -0.165 0.075 -0.225 0.066 -0.275 C 0.062 -0.31 0.1 -0.334 0.15 -0.341 Z"
GASK_OLD = "L 0.56 0.014 L 0.57 -0.053 C"
GASK_NEW = "C 0.547 0.058 0.566 0.036 0.568 0.01 C 0.57 -0.015 0.573 -0.035 0.57 -0.053 C"
D.edit(A, "geometry", lambda o: o["id"] in ("near-thigh", "far-thigh"), lambda o: {**o, "d": o["d"].replace("L 0.37 0.041", "L 0.37 0.041").replace(THIGH_OLD, THIGH_NEW)})
D.edit(A, "geometry", lambda o: o["id"] in ("near-gaskin", "far-gaskin"), lambda o: {**o, "d": o["d"].replace(GASK_OLD, GASK_NEW)})
D.edit(A, "geometry", lambda o: o["id"] == "rump", lambda o: {**o, "bone": "lumbar", "d": shift(o["d"], -0.2)})
D.save()
D = lib.Doc()
D.edit(A, "geometry", lambda o: o["id"] in ("near-thigh", "near-gaskin"), lambda o: {**o, "z": 3.5 if o["id"] == "near-thigh" else 3.52}); D.save()
