"""🪜️ person-carrying-ladder: two drawings that put the views out of step.

1:50 front: the right toe cap `shoe-toe-r` was bound to the right shank, so in the stride it swung with the shank and reached 96 mm under the ground while the side elevation stays on it; it becomes a solid of the toe bone `toe-r`: the side profile of the toe cap (its side twin) swept over the width the left toe cap draws, mirrored (an outline traced for the shank cannot simply be handed to the toe: tried, it stands 60 mm too high in the default stride).
1:200 and 1:500 plan: only the upper rail of the ladder was drawn (the ladder is carried on edge, the lower rail lies under the upper one). Tipped up by 15 degrees the lower rail reaches 93 mm further ahead than the upper one in the side elevation, and the plan did not show it. The plan gets the lower rail as the same solid the side elevation draws (1:200) and as the lower 25 mm of the frame (1:500), painted under the upper rail.
Writes the ops `<out>`: `python3 ladder.py <document json> <out ops json>`."""
import json
import sys

asset = json.load(open(sys.argv[1], encoding="utf8"))["asset"]
geometry = lambda scale, view: asset["representations"][scale]["geometry"][view]
find = lambda scale, view, shape_id: next(s for s in geometry(scale, view) if s["id"] == shape_id)
traced = find("50", "front", "shoe-toe-r")
left = [p[0] for p in find("50", "front", "shoe-toe-l")["points"]]
cap = {key: value for key, value in traced.items() if key not in ("points", "bone", "followTip")}
ops = [{"op": "shape", "scale": "50", "view": "front", "id": "shoe-toe-r", "shape": {**cap, "bone": "toe-r", "points": find("50", "elevation", "shoe-toe-r")["points"], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": round(-max(left), 4), "z1": round(-min(left), 4)}}}]
upper200 = find("200", "plan", "rail-upper")
ops.append({"op": "shape-add", "scale": "200", "view": "plan", "after": "rail-upper", "shape": {**find("200", "elevation", "rail-lower"), "z": round(upper200["z"] - 0.05, 2)}})
upper500 = find("500", "plan", "rail-upper")
ops.append({"op": "shape-add", "scale": "500", "view": "plan", "after": "rail-upper", "shape": {**upper500, "id": "rail-lower", "z": round(upper500["z"] - 0.05, 2), "points": [[-1.44, 0], [1.44, 0], [1.44, 0.025], [-1.44, 0.025]]}})
open(sys.argv[2], "w", encoding="utf8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False, separators=(",", ":")) for o in ops) + "\n]\n")
print(f"[DEBUG] {len(ops)} ops")
