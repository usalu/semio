"""↔️ Prints the ops of a prism taken from another band with another lateral span: `python3 span.py <scale> <id> <from scale> <z0> <z1>` gives `copy-scale` of the shape and a `set` of the solid `extruded`, sweep w, z0, z1 in the three views, then `measure`. For trials with variants.sh."""
import json, sys
scale, shape, source, z0, z1 = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4]), float(sys.argv[5])
base = {"scale": scale, "id": shape, "bone": shape}
print(json.dumps([{"op": "copy-scale", **base, "from": source}] + [{"op": "set", **base, "view": view, "set": {"solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}} for view in ("elevation", "front", "plan")] + [{"op": "measure", **base}]))
