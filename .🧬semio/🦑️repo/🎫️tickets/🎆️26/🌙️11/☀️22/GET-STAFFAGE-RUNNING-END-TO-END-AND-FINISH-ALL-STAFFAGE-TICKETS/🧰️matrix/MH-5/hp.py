"""✋️ Prints the ops that make one shape the same prism in the three views: `python3 hp.py <scales,comma> <id>[,<id>…] [front|plan] [full]` gives, per scale and id (the bone has the same name unless `id@bone` is written), `extrude` of the named view (default front; `full` adds `"span": "full"`) from the elevation outline, `copy` of it into the two other views and `measure`. For trials with variants.sh."""
import json, sys
scales, ids = sys.argv[1].split(","), sys.argv[2].split(",")
view = next((a for a in sys.argv[3:] if a in ("front", "plan")), "front")
ops = []
for scale in scales:
    for entry in ids:
        shape, bone = (entry.split("@") + [entry])[:2]
        base = {"scale": scale, "id": shape, "bone": bone}
        ops += [{"op": "extrude", **base, "view": view, **({"span": "full"} if "full" in sys.argv else {})}, {"op": "copy", **base, "from": view, "to": [v for v in ("elevation", "front", "plan") if v != view]}, {"op": "measure", **base}]
print(json.dumps(ops))
