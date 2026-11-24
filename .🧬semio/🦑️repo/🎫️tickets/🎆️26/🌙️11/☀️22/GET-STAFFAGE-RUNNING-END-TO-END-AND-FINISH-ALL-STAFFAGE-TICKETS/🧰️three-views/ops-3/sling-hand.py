"""🖐️ person-with-arm-sling, 1:50 and 1:100 plan: the left hand is a traced outline of the hanging hand, a plate that the hand bone carries. In `glance-at-watch` the forearm is raised ahead and the plate turns edge-on: the side elevation shows the hand 0.18 m long, the plan 8 mm (tolerance 54 mm). The traced outline stays as it is (it is the plan of the hanging hand, finger by finger); under it the plan gets the body of the hand as a solid: palm and the fingers up to the middle joints as the side profile, swept over the thickness of the palm (the loft of the volume: 19 mm either side of the bone, which rests at z 0.221). It lies inside the traced outline while the hand hangs and gives the hand its length when the arm is raised.
Writes the ops `<out>`: `python3 sling-hand.py <document json> <out ops json>`."""
import json
import sys

asset = json.load(open(sys.argv[1], encoding="utf8"))["asset"]
BODY = [[-0.004, -0.0275], [0.0337, -0.0409], [0.0954, -0.0411], [0.14, -0.0233], [0.14, 0.016], [0.0949, 0.0377], [0.0621, 0.0452], [0.0117, 0.0347], [-0.0036, 0.0291]]
ops = []
for scale in ("50", "100"):
    outline = next(s for s in asset["representations"][scale]["geometry"]["plan"] if s["id"] == "hand-l")
    body = {**outline, "id": "hand-l-body", "z": round(outline["z"] - 0.05, 2), "points": BODY, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": 0.205, "z1": 0.237}}
    ops.append({"op": "shape-add", "scale": scale, "view": "plan", "after": "hand-l", "shape": body})
open(sys.argv[2], "w", encoding="utf8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False, separators=(",", ":")) for o in ops) + "\n]\n")
print(f"[DEBUG] {len(ops)} ops")
