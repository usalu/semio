import importlib.util, json, re
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/"
spec = importlib.util.spec_from_file_location("lib", T + "🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc()
D.edit("horse", "geometry", lambda o: o["id"] in ("mane-1", "mane-2", "mane-3", "mane-strands", "mane-strands-2"), lambda o: {**o, "lod": 3})
lum = D.get("roe-deer", "rig", lambda o: o["id"] == "lumbar")[0]
def shift(d, dx):
    out = []; xs = True
    for t in d.split():
        if re.match(r"^[MLCZ]$", t): out.append(t); xs = True; continue
        v = float(t)
        if xs: v = round(v + dx, 4)
        out.append(f"{v:g}"); xs = not xs
    return " ".join(out)
def mv(o):
    if o["kind"] == "path": return {**o, "bone": "lumbar", "d": shift(o["d"], -lum["x"])}
    return o
D.edit("roe-deer", "geometry", lambda o: o["id"] == "rump" and o["bone"] == "pelvis", mv)
D.save()
D = lib.Doc()
D.edit("horse", "geometry", lambda o: o["id"] in ("hip-line", "rib-lines", "withers-line", "flank-hair", "stifle-line", "flank-fold") and o["lod"] < 3, lambda o: {**o, "lod": 3}); D.save()
