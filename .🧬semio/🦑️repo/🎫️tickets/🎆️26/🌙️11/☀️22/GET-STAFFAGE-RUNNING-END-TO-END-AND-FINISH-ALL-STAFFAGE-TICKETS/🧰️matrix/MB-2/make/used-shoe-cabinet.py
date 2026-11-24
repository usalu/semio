"""👟️ Writes the repair ops of `used-shoe-cabinet`: the plan of 1:50 and 1:100 gets the carcass block under the top (the side elevation profile swept over the 0.80 m width), so that the plan of the cabinet tilted and laid on the trolley shows its body and the foot contacts lie on it (66 mm off before). The carcass block is covered by the top in the rest pose; the plan of 1:200 and 1:500 stays one drawing (allowlisted, D47)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write
a = asset("used-shoe-cabinet")
sweep = {"kind": "extruded", "sweep": "w", "z0": -0.4, "z1": 0.4}
ops = []
for scale in (50, 100):
    ops.append({"op": "shape-add", "scale": str(scale), "view": "plan", "after": "back", "shape": like(shape(a, scale, "elevation", "side-r" if scale == 50 else "frame"), id="frame", z=2, solid=sweep)})
    ops.append({"op": "note", "scale": str(scale), "decision": "retained", "en": "Plan: the carcass block under the top, which shows when the cabinet is tilted.", "de": "Draufsicht: der Korpusblock unter dem Deckel, der sichtbar wird, wenn der Schrank gekippt ist."})
write("used-shoe-cabinet", ops)
