"""🌲️ Stone pine at 1:500: the stem solid `trunk-1` started 0.2 m below the ground (its lowest ring at u = -0.6 on a bone whose origin stands 0.4 m above the ground), which made the body 0.164 m taller than the declared height. The lowest ring is set on the ground (u = -0.4); the top tuft `pl-top-t10` keeps the transform it had before this repair. Rewrites the two `solid-set` ops of `repair/stone-pine.json` (idempotent, values from the state before the repair in 🗑️generated/silhouette/SV-2/backup).
python3 fix-stone-pine-ground.py"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
BACKUP = os.path.join(HERE, "..", "..", "🗑️generated", "silhouette", "SV-2", "backup", "stone-pine.json")
body = {s["id"]: s for s in json.load(open(BACKUP))["asset"]["volumes"]["500"]["body"]}
vertices = [[-0.4 if v[0] < -0.5 else v[0], v[1], v[2]] for v in body["trunk-1"]["vertices"]]
path = os.path.join(HERE, "repair", "stone-pine.json")
ops = [o for o in json.load(open(path)) if not (o["op"] == "solid-set" and o.get("scale") == "500" and o.get("id") in ("trunk-1", "pl-top-t10"))]
ops.append({"op": "solid-set", "scale": "500", "id": "trunk-1", "set": {"vertices": vertices}})
ops.append({"op": "solid-set", "scale": "500", "id": "pl-top-t10", "set": {"transform": body["pl-top-t10"]["transform"]}})
open(path, "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
print("[DEBUG] stone-pine: trunk-1 of 1:500 set on the ground, pl-top-t10 as before")
