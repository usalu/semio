"""✋️ The directed steps that draw a hand as its volume models it: for every bone below a hand bone that carries body solids at the scale, `prism|<view>|<bone>` (the bone solid by solid), then `refit|<view>|<hand shape>` for the palm; prints the CMD string for `auto.ts`. `python3 handcmd.py <slug> <scale> <hand bone> [views, default elevation,front,plan] [--no-refit]`"""
import json
import os
import sys

slug, scale, hand = sys.argv[1:4]
views = (sys.argv[4] if len(sys.argv) > 4 and not sys.argv[4].startswith("--") else "elevation,front,plan").split(",")
root = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 9))
asset = json.load(open(os.path.join(root, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", slug, "🔣️.json"), encoding="utf8"))["asset"]
parent = {bone["id"]: bone.get("parent") for bone in asset["rig"]["bones"]}


def below(bone):
    """🌿️ Whether a bone hangs below the hand bone."""
    while bone is not None:
        if bone == hand:
            return True
        bone = parent.get(bone)
    return False


carrying = []
for solid in asset["volumes"][scale]["body"]:
    if solid["bone"] != hand and below(solid["bone"]) and solid["bone"] not in carrying and not solid.get("texture"):
        carrying.append(solid["bone"])
steps = []
for view in views:
    steps += [f"prism|{view}|{bone}" for bone in carrying]
    if "--no-refit" not in sys.argv:
        steps += [f"refit|{view}|{shape['id']}" for shape in asset["representations"][scale]["geometry"][view] if shape["bone"] == hand and shape.get("filled") is not False and any(s["id"] == shape["id"] for s in asset["volumes"][scale]["body"])]
print(";".join(steps))
