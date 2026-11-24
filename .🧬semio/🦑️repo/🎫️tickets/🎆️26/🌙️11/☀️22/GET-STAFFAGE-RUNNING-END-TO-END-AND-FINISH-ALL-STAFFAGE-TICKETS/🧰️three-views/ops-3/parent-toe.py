"""🥾️ parent-with-child-on-shoulders, 1:500: the side elevation drew the boots without their toe caps (the boot ended at the toe joint, 0.146 m ahead of the ankle), while the plan and the volume carry the toe cap to 0.229 m; in the stride the plan therefore reached 84 mm further ahead than the side elevation (tolerance 64 mm). The side outline of each boot is drawn to the toe tip with the toe-cap outline of 1:200 merged in (a separate toe shape would be 0.14 mm high on paper). Writes the ops `<out>`: `python3 parent-toe.py <document json> <out ops json>`."""
import json
import sys

asset = json.load(open(sys.argv[1], encoding="utf8"))["asset"]
side = asset["representations"]["500"]["geometry"]["elevation"]
joint = next(b for b in asset["rig"]["bones"] if b["id"] == "toe-r")
cap = [[0.082588, 0], [0.090529, 0.011912], [0.090529, 0.035735], [0.070676, 0.051618], [0.027, 0.065509]]
ops = []
for boot_id in ("boot-l", "boot-r"):
    boot = next(s for s in side if s["id"] == boot_id)
    points = boot["points"]
    start = next(i for i, p in enumerate(points) if p[0] > 0.13)
    end = max(i for i, p in enumerate(points) if p[0] > 0.13)
    merged = points[:start] + [[round(joint["x"] + x, 6), round(joint["y"] + y, 6)] for x, y in cap] + points[end + 1:]
    ops.append({"op": "shape", "scale": "500", "view": "elevation", "id": boot_id, "shape": {**boot, "points": merged}})
open(sys.argv[2], "w", encoding="utf8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False, separators=(",", ":")) for o in ops) + "\n]\n")
print(f"[DEBUG] {len(ops)} boots drawn to the toe tip")
