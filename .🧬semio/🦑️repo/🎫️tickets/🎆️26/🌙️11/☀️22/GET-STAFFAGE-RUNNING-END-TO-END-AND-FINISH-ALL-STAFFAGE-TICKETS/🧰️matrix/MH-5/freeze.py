#!/usr/bin/env python3
"""🧊️ Keeps a hand-picked candidate: `python3 freeze.py <id> '<ops json>' [--why text]` applies the ops of `ops/<id>.json` and then the given ops to a copy of the backup (the asset as it was before this batch), and appends to `ops/<id>.json` one `set` op per shape the given ops changed, with the resulting geometry, or one `add` op per shape they created (so the repair is exact and applying it twice changes nothing, also for fits with visible slack and for `scale-profile`). Ops that change anchors, poses or clips are appended as they are. The catalog is not written."""
import json, os, subprocess, sys, glob

HOME = os.path.expanduser("~")
B = sorted(glob.glob(HOME + "/Library/Caches/semio/runtimes/bun/*/bun"))[0]
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T = S + "/🧪️tests/🔭️multiview-coverage"
U = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
W, GEN = U + "/🧰️matrix/MH-5", U + "/🗑️generated/matrix/MH-5"
KEYS = ["kind", "points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"]
asset_id, text = sys.argv[1], sys.argv[2]
why = sys.argv[sys.argv.index("--why") + 1] if "--why" in sys.argv else "hand-picked candidate"
ops_file = f"{W}/ops/{asset_id}.json"
accepted = json.load(open(ops_file)) if os.path.exists(ops_file) else []
os.makedirs(GEN + "/freeze", exist_ok=True)
base, out, tmp = f"{GEN}/freeze/{asset_id}.base.json", f"{GEN}/freeze/{asset_id}.out.json", f"{GEN}/freeze/{asset_id}.ops.json"
json.dump(accepted, open(tmp, "w"))
subprocess.run([W + "/b.sh", W + "/fit.ts", f"{GEN}/backup/{asset_id}.json", "--ops", tmp, "--out", base], cwd=T, capture_output=True)
new_ops = json.loads(text)
json.dump(new_ops, open(tmp, "w"))
print(subprocess.run([W + "/b.sh", W + "/fit.ts", base, "--ops", tmp, "--out", out], cwd=T, capture_output=True, text=True).stdout[-1500:])
before, after = json.load(open(base))["asset"], json.load(open(out))["asset"]
added = [op for op in new_ops if op["op"] in ("anchor", "anchor-split")]
for scale, rep in after["representations"].items():
    if rep.get("status") != "authored":
        continue
    for view in ("elevation", "front", "plan"):
        old = {(s["id"], s["bone"]): s for s in before["representations"][scale]["geometry"][view]}
        for shape in rep["geometry"][view]:
            if (shape["id"], shape["bone"]) not in old:
                added.append({"op": "add", "scale": scale, "view": view, "id": shape["id"], "why": why, "shape": shape})
            elif old.get((shape["id"], shape["bone"])) != shape:
                added.append({"op": "set", "scale": scale, "view": view, "id": shape["id"], "bone": shape["bone"], "why": why, "set": {key: shape.get(key) for key in KEYS}})
os.makedirs(os.path.dirname(ops_file), exist_ok=True)
json.dump(accepted + added, open(ops_file, "w"), indent=1)
print(f"{asset_id}: {len(added)} ops appended ({len(accepted) + len(added)} in all)")
