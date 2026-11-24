"""🚋️ Writes the hand-made repair ops of `tram` into ops/tram.json (the first step of make/tram.sh).

Front view, pantograph: the drawing had a bar of 80 mm up the middle of the upper arm, from the knee to the collector head. The body has no such bar: the upper arm is the V of two tubes (drawn already) and, at 1:50 only, the thin guide rod of 18 mm between them. At 1:50 the bar becomes that rod, at 1:100 and 1:200 it is removed. The guide rod of the lower arm (22 mm, 60 mm to the left of the arm) is drawn in the front view of 1:50. The collector head of the 1:50 front was one bar of 50 mm over 1.10 m; it is drawn as the body has it: the carbon strips (22 mm, 1.05 m) on their carriers (12 mm, 1.10 m). All are prisms on their bones, so they follow the pantograph when it folds.
Side elevation, pantograph: the lower arm is a tube that tapers from 84 to 68 mm; it was a stroke of 0.7 mm (35 mm) at 1:50 and 0.3 mm (30 mm) at 1:100 and is drawn at its thickness. The tube is five to eleven smallest features thick at these scales and no longer declared `slender` there: the guard compares a slender solid as a stroke that runs half a width past its round ends (requests.md, silhouette SC), which stood 45 mm past the knee. The same holds for the two tubes of the upper arm (40 mm, a stroke of 35 and 30 mm before). The two pins of the guide rod of the lower arm (44 and 40 mm) are drawn at 1:50."""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, ".."))
from lib import asset, shape, solid

SLUG = "tram"
a = asset(SLUG)
ops = []
upper = shape(a, 50, "front", "pan-upper")
rod = solid(a, 50, "pan-upper-rod")
bar = lambda like, sid, s, z: {**{k: v for k, v in like.items() if k not in ("x0", "y0", "x1", "y1", "r0", "r1", "solid", "id", "z", "role")}, "id": sid, "role": "detail", "z": z, "x0": s["p0"][0], "y0": s["p0"][1], "x1": s["p1"][0], "y1": s["p1"][1], "r0": s["r0"], "r1": s["r1"], "solid": {"kind": "extruded", "sweep": "w", "z0": round(s["p0"][2] - s["r0"], 5), "z1": round(s["p0"][2] + s["r0"], 5)}}
ops.append({"op": "shape-remove", "scale": "50", "view": "front", "id": "pan-upper"})
ops.append({"op": "shape-add", "scale": "50", "view": "front", "after": "pan-lower-sil", "shape": bar(upper, "pan-upper-rod", rod, 3.1)})
ops.append({"op": "shape-add", "scale": "50", "view": "front", "after": "pan-lower-sil", "shape": bar(upper, "pan-lower-rod", solid(a, 50, "pan-lower-rod"), 3.02) | {"bone": "pan-lower"}})
head = shape(a, 50, "front", "pan-head")
strip, carrier = solid(a, 50, "pan-strip-1"), solid(a, 50, "pan-carrier-1")
slab = lambda sid, s, z, extra: {**{k: v for k, v in head.items() if k not in ("x0", "y0", "x1", "y1", "r0", "r1", "kind")}, **extra, "id": sid, "z": z, "kind": "rect", "x": round(-0.17 - s["size"][0] / 2, 5), "y": round(s["transform"]["translation"][1] - s["size"][1] / 2, 5), "w": round(0.34 + s["size"][0], 5), "h": s["size"][1], "solid": {"kind": "extruded", "sweep": "w", "z0": -s["size"][2] / 2, "z1": s["size"][2] / 2}}
ops.append({"op": "shape-add", "scale": "50", "view": "front", "after": "pan-head", "shape": slab("pan-head", strip, head["z"], {})})
ops.append({"op": "shape-add", "scale": "50", "view": "front", "after": "pan-head", "shape": slab("pan-carrier", carrier, round(head["z"] - 0.02, 2), {"tone": 0.3, "material": "pan-alloy"})})
for scale in ("100", "200"):
    ops.append({"op": "shape-remove", "scale": scale, "view": "front", "id": "pan-upper"})
for scale in ("50", "100"):
    old, arm = shape(a, scale, "elevation", "pan-lower"), solid(a, scale, "pan-lower-arm")
    ops.append({"op": "set", "path": ["volumes", scale, "body", {"id": "pan-lower-arm"}, "slender"]})
    ops.append({"op": "shape", "scale": scale, "view": "elevation", "id": "pan-lower", "shape": {**{k: v for k, v in old.items() if k not in ("points", "closed", "penMm", "kind")}, "kind": "capsule", "filled": True, "x0": arm["p0"][0], "y0": arm["p0"][1], "x1": arm["p1"][0], "y1": arm["p1"][1], "r0": arm["r0"], "r1": arm["r1"]}})
    old, arm = shape(a, scale, "elevation", "pan-upper"), solid(a, scale, "pan-upper-arm-l")
    for side in ("l", "r"):
        ops.append({"op": "set", "path": ["volumes", scale, "body", {"id": f"pan-upper-arm-{side}"}, "slender"]})
    ops.append({"op": "shape", "scale": scale, "view": "elevation", "id": "pan-upper", "shape": {**{k: v for k, v in old.items() if k not in ("points", "closed", "penMm", "kind")}, "kind": "capsule", "filled": True, "x0": arm["p0"][0], "y0": arm["p0"][1], "x1": arm["p1"][0], "y1": arm["p1"][1], "r0": arm["r0"], "r1": arm["r1"]}})
knee = shape(a, 50, "elevation", "pan-joint-knee")
for sid in ("pan-rod-pin-base", "pan-rod-pin-knee"):
    pin = solid(a, 50, sid)
    ops.append({"op": "shape-add", "scale": "50", "view": "elevation", "after": "pan-joint-knee", "shape": {**knee, "id": sid, "role": "detail", "z": 3.06, "cx": pin["origin"][0], "cy": pin["origin"][1], "rx": pin["profile"][1][1], "ry": pin["profile"][1][1]}})
subprocess.run([sys.executable, os.path.join(HERE, "..", "hand.py"), SLUG], input=json.dumps(ops), text=True, check=True)
