#!/usr/bin/env python3
"""🧾️ Inventory of an asset for the loft round: per scale and bone the drawn shapes of each view (id, kind, solid) beside the body solids of the volume (id, kind, weights): `inv.py <file.json> [scale…]`."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]
for s in sys.argv[2:] or sorted(a["representations"], key=int):
    g = a["representations"][s]["geometry"]; body = a["volumes"][s]["body"]
    bones = [b["id"] for b in a["rig"]["bones"]] if "rig" in a and "bones" in a["rig"] else sorted({x["bone"] for v in g.values() for x in v} | {x["bone"] for x in body})
    print("=== 1:" + s)
    for b in bones:
        d = lambda x: x["id"] + (":" + (x["solid"]["kind"][:3] + x["solid"].get("sweep", "") if x.get("solid") else x["kind"] if x["kind"] in ("loft", "member") else "flat")) + ("*" if x.get("weights") or x.get("followTip") else "")
        rows = {v: [d(x) for x in g[v] if x["bone"] == b] for v in ("elevation", "front", "plan") if v in g}
        vs = [x["id"] + ":" + x["kind"] + ("*" if x.get("weights") else "") + ("R" if x.get("repeat") else "") for x in body if x["bone"] == b]
        if vs or any(rows.values()): print(b, "| E", *rows.get("elevation", []), "| F", *rows.get("front", []), "| P", *rows.get("plan", []), "| V", *vs)
