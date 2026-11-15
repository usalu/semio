import sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
def fix(a):
    g = a["representations"]["200"]["geometry"]
    for s in g["front"]:
        if s["id"] == "back-2":
            s["bone"] = "frame"
    if not any(s["id"] == "skirt" for s in g["plan"]):
        g["plan"].append({"id": "skirt", "bone": "frame", "role": "body", "z": 1, "filled": True, "tone": 0.45, "region": "base-rail", "material": "upholstery-fabric", "kind": "rect", "x": 0.15, "y": -0.43, "w": 1.7, "h": 0.86})
    return 2
print(L.edit_asset("🛋️furniture", "three-seat-sofa", fix))
