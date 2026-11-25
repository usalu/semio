"""🧻️ Ops writer of `reclaimed-trapezoidal-sheet-stack`: the two legs of the round sling lie on the pack at rest and stand up by 90 degrees when the sling is tensioned, but the front drew them as flat end views (1:50, 1:100) or not at all (1:200, 1:500) and the plan as flat outlines or strokes, so the front ended 369 to 646 mm below the side elevation. Every leg becomes its side elevation outline swept over the 50 mm width of the sling in front and plan; where the side elevation draws a stroke (1:200, 1:500) the leg is a stroke as a solid, a sliver about that stroke: `python3 make/reclaimed-trapezoidal-sheet-stack.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("reclaimed-trapezoidal-sheet-stack")
LEGS = ("sling-leg-a", "sling-leg-b")
ops = solidify(a, set(LEGS), twins={f"{leg}-{end}": f"{leg}-side" for leg in LEGS for end in ("end", "top")}, quiet=True)
for scale in scales(a):
    for leg in LEGS:
        side = shape(a, scale, "elevation", f"{leg}-side")
        if is_closed(side):
            continue
        stroke = sliver(side, side["points"], -0.025, 0.025)
        ops.append(op(scale, "plan", f"{leg}-top", like(stroke, id=f"{leg}-top", z=shape(a, scale, "plan", f"{leg}-top")["z"])))
        ops.append({"op": "shape-add", "scale": scale, "view": "front", "after": views(a, scale)["front"][-1]["id"] if not has(a, scale, "front", f"{leg}-end") else f"{leg}-end", "shape": like(stroke, id=f"{leg}-end", z=6.5)})
write("reclaimed-trapezoidal-sheet-stack", ops)
