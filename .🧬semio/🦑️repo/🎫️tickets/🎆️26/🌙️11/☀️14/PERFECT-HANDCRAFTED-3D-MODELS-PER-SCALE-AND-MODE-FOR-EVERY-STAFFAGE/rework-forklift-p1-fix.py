"""🚜 Forklift, pass 1 (second round), second chisel: the drive axle housings are body solids of their own around the hidden motor and brake cores (hidden thermal regions keep their own solids), fork shanks 45 mm so the pallet stands against them, pallet bones keep their members at 1:500.

Run from the ticket folder after rework-forklift-p1-coarse.py: python3 rework-forklift-p1-fix.py
"""
import json, copy
exec(open("rework-10-lib.py").read())
S = Source("forklift")
OLD = json.load(open("🗑️generated/rework-forklift/source-before.json"))["volumes"]
M2 = lambda dz: {"count": 2, "translation": [0, 0, dz]}
CORE = ("motor", "gear-brake", "terminal-box", "motor-mount")
for scale in ("50", "100", "200", "500"):
    v = S.v[scale]
    v["body"] = [s for s in v["body"] if s["id"] not in CORE]
    v["thermal"] = [t for t in v["thermal"] if t.get("of") not in CORE and t["id"] not in CORE]
    cores = [copy.deepcopy(t) for t in OLD[scale]["thermal"] if t["id"] in CORE]
    for t in cores:
        if scale == "50" and t["id"] == "motor":
            t["repeat"] = M2(-0.34)
        if scale in ("50", "100") and t["id"] == "gear-brake":
            t["repeat"] = M2(-0.685)
    v["thermal"] += cores
    v["structure"] = [m for m in v["structure"] if m["id"] not in ("m-motor", "m-gear-brake", "m-motor-mount")]
    if scale in ("50", "100"):
        housings = [cyl("axle-motor", "axle-f", "steel", [0, 0, -0.3], [0, 0, 1], 0.13, 0.6, tone=0.3, role="body"), cyl("axle-gear", "axle-f", "cast-iron", [0, 0, 0.3], [0, 0, 1], 0.11, 0.085, tone=0.35, role="body", repeat=M2(-0.685))]
    else:
        housings = [cyl("axle-motor", "axle-f", "steel", [0, 0, -0.39], [0, 0, 1], 0.13, 0.78, tone=0.3, role="body")]
    for h in housings:
        S.put(int(scale), [h], region="chassis-paint")
        v["structure"].append({"id": "m-" + h["id"], "of": h["id"], "member": "axle-f", "part": "member"})
for scale in ("50", "100"):
    for s in S.v[scale]["body"]:
        if s["id"] in ("tine-l", "tine-r"):
            s["profile"] = [[round(x + 0.005, 5), y] if (0 <= x <= 0.015 and y >= 0) else [x, y] for x, y in s["profile"]]
v = S.v["500"]
v["structure"] += [copy.deepcopy(m) for m in OLD["500"]["structure"] if m["id"] in ("pallet-base", "pallet-column")]
S.save()
