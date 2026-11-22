"""🖊️ Author pass 1 (second round), animals/horse: layering of the redrawn trunk shapes in the 2D drawings.

Usage: python3 rework-p1-horse-2d.py
The trunk is one shape on the lumbar bone now: in the plan it takes the coat tone of the body (it was the half-hidden loin before),
and the withers ridge lies under it in the elevation and the front so only the part that stands above the back is drawn.
"""
import os
import types

HERE = os.path.dirname(os.path.abspath(__file__))
carry = types.ModuleType("carry")
carry.__file__ = os.path.join(HERE, "rework-07-2d.py")
with open(carry.__file__, encoding="utf-8") as handle:
    exec(compile(handle.read().split("\ndef main():")[0], carry.__file__, "exec"), carry.__dict__)


def mutate(representations, asset):
    for scale in ("50", "100", "200", "500"):
        geometry = representations[scale]["geometry"]
        for shape in geometry["plan"]:
            if shape["id"] == "loin":
                shape["tone"] = 0.72
        for view in ("elevation", "front"):
            for shape in geometry[view]:
                if shape["id"] == "withers":
                    shape["z"] = 2.9


carry.splice(os.path.join(carry.CATALOG, "horse", "🔣️.json"), "representations", mutate)
print("[DEBUG] layering written")
