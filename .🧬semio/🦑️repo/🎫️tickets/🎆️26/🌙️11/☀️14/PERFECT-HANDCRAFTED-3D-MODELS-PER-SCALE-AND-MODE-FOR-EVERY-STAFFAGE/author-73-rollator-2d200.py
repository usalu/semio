"""Author 73: rollator 1:200 front: the lower rail (3D has it, drawing missed it) added as the 1:50 front draws it."""
from author_73_lib import *
slug = "person-with-rollator"
def f(g, rep):
    if any(s["id"] == "lower-rail" for s in g["front"]): return
    i = next(i for i, s in enumerate(g["front"]) if s["id"] == "handlebar")
    g["front"].insert(i + 1, {"kind": "rect", "x": -0.27, "y": 0.24, "w": 0.54, "h": 0.025, "id": "lower-rail", "bone": "frame", "role": "body", "z": 9.05, "filled": True, "tone": 0.4, "region": "frame", "penMm": 0.1})
backup(slug, "before-200")
print(patch_rep(slug, 200, f))
