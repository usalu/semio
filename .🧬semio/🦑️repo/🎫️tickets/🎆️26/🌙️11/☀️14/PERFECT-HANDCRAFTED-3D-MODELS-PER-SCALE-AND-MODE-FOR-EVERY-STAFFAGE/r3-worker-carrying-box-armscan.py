"""🔎 Scans arm solutions of `humans/worker-carrying-box` for one pose over grip positions and hand leans (design aid of the round-3 root fixes).

Usage: python3 r3-worker-carrying-box-armscan.py <pose> <x0,x1,..> <y0,y1,..> <phi0,phi1,..> [psi] [swivel] [bone=angle ...]
A grip x with a leading "t" is measured from the toe tip plus half the box depth (t0.02 = box 20 mm ahead of the toes).
"""
import json
import sys

sys.path.insert(0, ".")
import r3_worker_carrying_box_ik as ik
import r3_worker_carrying_box_kit as k

src = open("r3-worker-carrying-box-roots.py").read().replace("\nmain()\n", "\n")
ns = {"__file__": "r3-worker-carrying-box-roots.py"}
exec(compile(src, "roots", "exec"), ns)
a = json.load(open(ns["ASSET"]))["asset"]
ns["fix_rig"](a)
ns["fix_anchors"](a)
pose = next(p for p in a["poses"] if p["id"] == sys.argv[1])
rest = {b: {"angleDeg": o["angleDeg"]} for b, o in pose["bones"].items() if not b.startswith(("upper-arm", "forearm", "hand", "box"))}
for item in sys.argv[7:]:
    bone, value = item.split("=")
    for b in ([bone + "-l", bone + "-r"] if bone + "-l" in rest else [bone]):
        rest[b] = {"angleDeg": float(value)}
base = k.grounded(a, rest, ik.FEET)
toe = base["toe-l"]["o"][0] + 0.099 * base["toe-l"]["u"][0]
print("shoulder", [round(c, 3) for c in base["upper-arm-l"]["o"]], "hip", round(base["pelvis"]["o"][1], 3), "knee", [round(c, 3) for c in base["shank-l"]["o"]], "toe tip", round(toe, 3))
psi = float(sys.argv[5]) if len(sys.argv) > 5 else 3
elbow = tuple(float(v) for v in sys.argv[6].split(",")) if len(sys.argv) > 6 and sys.argv[6] != "0" else (-1.0, -0.5, 0.3)
for x in [toe + 0.15 + float(v[1:]) if v.startswith("t") else float(v) for v in sys.argv[2].split(",")]:
    for y in map(float, sys.argv[3].split(",")):
        for phi in map(float, sys.argv[4].split(",")):
            try:
                s = ik.solve_arms(a, rest, (x, y, ns["GRIP_Z"]), phi, psi, elbow)
                f = k.grounded(a, {**rest, **s}, ik.FEET)
                print(round(x, 3), y, phi, json.dumps({b: v for b, v in s.items() if b.endswith("-l")}), "elbow", [round(c, 3) for c in f["forearm-l"]["o"]])
            except SystemExit as e:
                print(round(x, 3), y, phi, "none", e)
