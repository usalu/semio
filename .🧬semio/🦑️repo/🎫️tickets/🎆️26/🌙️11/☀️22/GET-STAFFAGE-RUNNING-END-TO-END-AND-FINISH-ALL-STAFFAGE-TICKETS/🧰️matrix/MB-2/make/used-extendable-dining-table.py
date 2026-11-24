"""🍽️ Writes the repair ops of `used-extendable-dining-table`: the front of 1:50 gets the traces of use its side elevation already shows (chair scuffs on the end apron and the near leg, the folds of the cardboard shim), the front of 1:100 loses the grain hatch its band omits, and the plan of 1:200 shows the broken corner of the right top (38 x 28 mm, 0.19 mm on paper) that 1:500 cannot print."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write, GEOMETRY
a = asset("used-extendable-dining-table")
scuffs = like(shape(a, 50, "elevation", "scuffs-r"), id="scuffs-end", d="M -0.24 0.611 L -0.205 0.621 M -0.03 0.608 L 0.01 0.617 M 0.0 0.619 L 0.03 0.626 M 0.19 0.612 L 0.23 0.62 M 0.352 0.31 L 0.39 0.319 M 0.35 0.345 L 0.384 0.352")
folds = like(shape(a, 50, "elevation", "shim-folds"), d="M -0.416 0.004 L -0.324 0.004 M -0.416 0.008 L -0.324 0.008 M -0.416 0.012 L -0.324 0.012")
chipped = like(shape(a, 200, "plan", "top-r"), drop=GEOMETRY, kind="poly", points=shape(a, 100, "plan", "top-r")["points"], closed=True)
ops = [
    {"op": "shape-add", "scale": "50", "view": "front", "after": "end-apron-r", "shape": scuffs},
    {"op": "shape-add", "scale": "50", "view": "front", "after": "shim", "shape": folds},
    {"op": "note", "scale": "50", "decision": "retained", "en": "Front: the chair scuffs on the end apron and the near leg and the folds of the shim.", "de": "Vorderansicht: die Stuhlschrammen an der Stirnzarge und am nahen Bein und die Falten der Unterlage."},
    {"op": "shape", "scale": "200", "view": "plan", "id": "top-r", "shape": chipped},
    {"op": "note", "scale": "200", "decision": "retained", "en": "Plan: the broken corner of the right top (38 x 28 mm).", "de": "Draufsicht: die abgebrochene Ecke der rechten Plattenhälfte (38 x 28 mm)."},
]
if "hatch" in shape(a, 100, "front", "end-apron-r") and "hatch" not in shape(a, 100, "elevation", "apron-r"):
    ops.append({"op": "shape", "scale": "100", "view": "front", "id": "end-apron-r", "shape": like(shape(a, 100, "front", "end-apron-r"), drop=("hatch",))})
write("used-extendable-dining-table", ops)
