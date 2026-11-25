"""🧤️ Prints the ops that make one shape the front outline swept through the thickness the elevation shows, in the three views: `python3 fs.py <scales,comma> <id>[@bone] [elevation-too]` gives `front-sweep` (../MR/fit.ts: the front outline kept, swept along v over the tapered thickness of the elevation twin), `copy` of the front into the plan (and into the elevation with `elevation-too`) and `measure`. For trials with variants.sh."""
import json, sys
ops = []
shape, bone = (sys.argv[2].split("@") + [sys.argv[2]])[:2]
for scale in sys.argv[1].split(","):
    base = {"scale": scale, "id": shape, "bone": bone}
    ops += [{"op": "front-sweep", **base}, {"op": "copy", **base, "from": "front", "to": ["elevation", "plan"] if "elevation-too" in sys.argv else ["plan"]}, {"op": "measure", **base}]
print(json.dumps(ops))
