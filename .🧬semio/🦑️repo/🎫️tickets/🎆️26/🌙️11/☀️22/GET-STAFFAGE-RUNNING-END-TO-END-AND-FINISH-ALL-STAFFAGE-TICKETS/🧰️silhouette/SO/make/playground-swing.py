"""🛝️ Writes the repair ops of `playground-swing`: at 1:50 the through bolts of the apex sleeves lie horizontal like the apex bolts beside them (head ahead, nut behind) instead of standing 13.5 mm above the declared 2.50 m, and the hanger bands have the wall of the sleeves (outer radius 57.15 mm instead of 58.5 mm), so the body is 2.5072 m high, within one smallest feature; the three views draw the same bolts and bands. The front draws the hook shank between the hanger pin and the first chain link at its own 13 mm and starts the 27 mm chain band at the lower eye (the band ran up to the pin and stood 17 mm from the shank) and the two cheeks of each hanger fork apart, as the volume has them."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
Q = [0.0, 0.707107, 0.0, 0.707107]
BAND = [[0.0, 0.05], [0.0, 0.05515], [0.002, 0.05715], [0.048, 0.05715], [0.05, 0.05515], [0.05, 0.05]]
DETAIL = {"bone": "beam", "role": "detail", "z": 3.15, "filled": True, "tone": 0.25, "region": "frame-galv", "material": "steel-galvanised", "kind": "rect"}
ops = [
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "sleeve-bolt", "set": {"points": [[0.0555, -0.011], [0.0635, -0.011], [0.0635, 0.011], [0.0555, 0.011]]}},
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "sleeve-nut", "set": {"points": [[-0.0655, -0.011], [-0.0555, -0.011], [-0.0555, 0.011], [-0.0655, 0.011]]}},
]
for side, z, lateral in (("l", 0.875, 0.8655), ("r", -0.875, -0.8845)):
    ops += [
        {"op": "solid-set", "scale": "50", "id": f"sleeve-bolt-{side}", "set": {"transform": {"translation": [0.0555, 0.0, z], "rotation": Q}}},
        {"op": "solid-set", "scale": "50", "id": f"sleeve-nut-{side}", "set": {"transform": {"translation": [-0.0555, 0.0, z], "rotation": Q}}},
        {"op": "shape-set", "scale": "50", "view": "front", "id": f"sleeve-bolt-{side}", "set": {"y": 2.439, "h": 0.022}},
        {"op": "shape-remove", "scale": "50", "view": "front", "id": f"sleeve-nut-{side}"},
        {"op": "shape", "scale": "50", "view": "plan", "id": f"sleeve-bolt-{side}", "shape": {"id": f"sleeve-bolt-{side}", **DETAIL, "x": 1.0555, "y": lateral, "w": 0.008, "h": 0.019}},
        {"op": "shape-add", "scale": "50", "view": "plan", "after": f"sleeve-bolt-{side}", "shape": {"id": f"sleeve-nut-{side}", **DETAIL, "x": 0.9345, "y": lateral, "w": 0.01, "h": 0.019}},
    ]
for side in ("left", "right"):
    ops += [
        {"op": "solid-set", "scale": "50", "id": f"hanger-band-{side}", "set": {"profile": BAND}},
        {"op": "shape-set", "scale": "50", "view": "front", "id": f"hanger-band-{side}", "set": {"y": 2.39285, "h": 0.1143}},
        {"op": "shape-set", "scale": "50", "view": "plan", "id": f"hanger-band-{side}", "set": {"x": 0.94285, "w": 0.1143}},
    ]
K = (0.236953 - 0.186466) / (2.386 - 0.4675)
for side, sign in (("left", 1), ("right", -1)):
    edge = lambda x, y: round(sign * (x - (2.386 - y) * K), 6)
    ops += [
        {"op": "shape-set", "scale": "50", "view": "front", "id": f"chain-{side}", "set": {"points": [[edge(0.236953, 2.325), 2.325], [edge(0.264153, 2.325), 2.325], [sign * 0.213666, 0.4675], [sign * 0.186466, 0.4675]]}},
        {"op": "shape-add", "scale": "50", "view": "front", "after": f"chain-{side}", "shape": {"id": f"hook-shank-{side}", "bone": f"chain-{side}", "role": "detail", "z": 2.5, "filled": True, "tone": 0.35, "region": "chain-steel", "material": "steel-galvanised", "kind": "capsule", "x0": edge(0.250553, 2.36), "y0": 2.36, "x1": edge(0.250553, 2.325), "y1": 2.325, "r0": 0.0065, "r1": 0.0065}},
    ]
FORK = {"bone": "beam", "role": "detail", "z": 3.15, "filled": True, "tone": 0.3, "region": "frame-galv", "material": "steel-galvanised", "kind": "rect", "y": 2.34, "w": 0.008, "h": 0.062}
for side, inner, outer in (("left", 0.229, 0.263), ("right", -0.237, -0.271)):
    ops += [
        {"op": "shape", "scale": "50", "view": "front", "id": f"hanger-fork-{side}", "shape": {"id": f"hanger-fork-{side}", **FORK, "x": inner}},
        {"op": "shape-add", "scale": "50", "view": "front", "after": f"hanger-fork-{side}", "shape": {"id": f"hanger-fork-{side}-outer", **FORK, "x": outer}},
    ]
write("playground-swing", ops)
