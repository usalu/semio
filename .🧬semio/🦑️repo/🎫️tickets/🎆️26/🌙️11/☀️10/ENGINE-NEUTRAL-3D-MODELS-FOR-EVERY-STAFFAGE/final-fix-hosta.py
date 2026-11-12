"""hosta-clump summer-full: the outer blades touch the soil as the pose describes (the lowest drawn point was 0.043 m above it, so the drawn height 0.585 m fell 35 mm short of heightM 0.62 m, whose top 0.628 m above the soil it matches): blade-r1 and blade-l1 droop 5 degrees more (lowest point 0.043 -> 0.017 m, widthM stays within 14 mm)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE")
from catalog_edit import S, edit
path = S + "🌳️vegetation/hosta-clump/🔣️.json"
def fix(o):
    if o.get("id") != "summer-full" or "bones" not in o or "blade-r1" in o["bones"]: return None
    bones = {}
    for k, v in o["bones"].items():
        bones[k] = v
        if k == "petiole-r1": bones["blade-r1"] = {"angleDeg": -5}
        if k == "petiole-l1": bones["blade-l1"] = {"angleDeg": 5}
    o["bones"] = bones
    return o
print(edit(path, lambda t: '"id": "summer-full"' in t and '"bones"' in t and '"petiole-r1"' in t and t.count('"id": "summer-full"') == 1 and '"clips"' not in t, fix))
