import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "office-swivel-chair")
for side, (z0, z1) in (("left", (0.235, 0.28)), ("right", (-0.28, -0.235))):
    o = d.obj(d.shape(100, "elevation", "armrest-post-" + side))
    o["solid"] = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
    d.insert_after(d.shape(100, "plan", "backrest-shell"), o) if side == "left" else d.insert_after(d.shape(100, "plan", "armrest-pad-left"), o)
for side, (z0, z1) in (("left", (0.236, 0.258)), ("right", (-0.258, -0.236))):
    n = d.shape(50, "plan", "armrest-post-" + side)
    o = d.obj(n); o["id"] = "armrest-bracket-" + side
    o["d"] = "M 0.033 0.05 L 0.033 0.02 L 0.053 0.02 L 0.053 0.05 Z"
    o["solid"] = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
    d.insert_after(n, o)
d.commit()
