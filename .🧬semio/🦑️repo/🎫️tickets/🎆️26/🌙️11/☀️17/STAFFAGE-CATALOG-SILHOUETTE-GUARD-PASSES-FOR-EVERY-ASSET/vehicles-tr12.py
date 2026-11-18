import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
for sc in (50, 100):
    try: low = d.obj(d.shape(sc, "elevation", "pan-lower"))
    except KeyError: continue
    up = d.obj(d.shape(sc, "elevation", "pan-upper"))
    base = {k: low[k] for k in ("role", "z", "tone", "region", "material") if k in low}
    shapes = [{"kind": "ellipse", "cx": cx, "cy": 0, "rx": 0.03, "ry": 0.03, "id": sid, "bone": bone, "filled": True, **base} for sid, bone, cx in (("pan-joint-base", low["bone"], 0), ("pan-joint-knee", low["bone"], 1.9), ("pan-joint-tip", up["bone"], 1.9))]
    n = d.shape(sc, "elevation", "pan-upper")
    ind = d.indent(n)
    txt = "".join(",\n" + ind + m.json.dumps(o, ensure_ascii=False, indent=1).replace("\n", "\n" + ind) for o in shapes)
    d.edits.append((n["e"], n["e"], txt))
d.commit()
