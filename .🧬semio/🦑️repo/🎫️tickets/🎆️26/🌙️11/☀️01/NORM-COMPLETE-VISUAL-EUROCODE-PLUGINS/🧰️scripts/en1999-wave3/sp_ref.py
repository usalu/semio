import sys, math, json
sys.path.insert(0, "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔌️adapters/sectionproperties")
import importlib.util
spec = importlib.util.spec_from_file_location("spa", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔌️adapters/sectionproperties/🐍️.py")
spa = importlib.util.module_from_spec(spec); spec.loader.exec_module(spa)
sys.path.insert(0, ".")
import run_proto as rp
for k, (o, h) in rp.cases.items():
    r = spa.polygon_holes({"outer": o, "holes": h, "mesh": 4.0e-6})
    print(k, {a: float(f"{r[a]:.5g}") for a in ("A", "Iy", "Iz", "It", "Iw", "y_sc", "z_sc")})
