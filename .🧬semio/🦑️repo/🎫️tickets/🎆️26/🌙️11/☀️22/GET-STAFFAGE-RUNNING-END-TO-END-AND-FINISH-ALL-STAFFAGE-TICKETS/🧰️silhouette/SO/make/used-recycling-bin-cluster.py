"""♻️ Writes the repair ops of `used-recycling-bin-cluster`: at 1:50 the front draws the trunnion bosses and pins that carry each tipping bin between its posts (six of each in the volume; the gap between bin and post stood open in the drawing, 16 mm from the pins)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, solid, write
a = asset("used-recycling-bin-cluster")
BOSS = {"bin-r": (-0.284, -0.692), "bin-c": (0.2, -0.208), "bin-l": (0.684, 0.276)}
PIN = {"bin-r": (-0.286, -0.699), "bin-c": (0.198, -0.215), "bin-l": (0.682, 0.269)}
ops = []
for bone in ("bin-r", "bin-c", "bin-l"):
    for k, side in enumerate(("l", "r")):
        boss, pin = solid(a, 50, f"{bone}-boss-{side}"), solid(a, 50, f"{bone}-pin-{side}")
        look = {"bone": bone, "role": "detail", "filled": True, "kind": "rect"}
        ops.append({"op": "shape-add", "scale": "50", "view": "front", "after": f"{bone}-tub", "shape": {"id": f"{bone}-boss-{side}", **look, "tone": boss["tone"], "material": boss["material"], "region": None, "x": BOSS[bone][k], "y": 0.855, "w": 0.008, "h": 0.05, "z": 1.86}})
        ops.append({"op": "shape-add", "scale": "50", "view": "front", "after": f"{bone}-boss-{side}", "shape": {"id": f"{bone}-pin-{side}", **look, "tone": pin["tone"], "material": pin["material"], "region": None, "x": PIN[bone][k], "y": 0.868, "w": 0.017, "h": 0.024, "z": 1.87}})
tub = {s["bone"]: s for s in a["representations"]["50"]["geometry"]["front"] if s["id"].endswith("-tub")}
for o in ops:
    o["shape"]["region"] = tub[o["shape"]["bone"]]["region"]
write("used-recycling-bin-cluster", ops)
