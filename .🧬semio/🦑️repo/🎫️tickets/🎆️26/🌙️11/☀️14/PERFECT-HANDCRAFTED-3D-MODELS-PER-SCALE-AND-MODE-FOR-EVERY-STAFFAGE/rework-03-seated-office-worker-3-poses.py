"""🪑 Stage 3 of the seated office worker: poses and the typing clip on the corrected rig. The pelvis sits back to the lumbar pad, the lean is shared by lumbar spine and thorax, and both arms are solved so that the heel of the hand lies on the palm rest (or desk, thigh, armrest, occiput) and the fingertip on its key.

Run: `python3 rework-03-seated-office-worker-3-poses.py` writes the candidate; `--write` copies it into the catalog after `r3-validate.ts --file`.
"""
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
C = candidate_path(A)
if "--write" in sys.argv:
    cand = read(C)
    doc = read(catalog_path(A))
    doc["asset"]["poses"], doc["asset"]["clips"] = cand["asset"]["poses"], cand["asset"]["clips"]
    write(catalog_path(A), doc)
    sys.exit(0)

doc = read(catalog_path(A))
a = doc["asset"]
P = {p["id"]: p for p in a["poses"]}
HEEL, TIP = (0.012, -0.019, 0), (0.1583, -0.0627, 0.0095)
INDEX = (0.1515, -0.0615, 0.029)
ARM = lambda s: [(f"arm-{s}", "angleDeg"), (f"arm-{s}", "abductionDeg"), (f"arm-{s}", "twistDeg"), (f"forearm-{s}", "angleDeg"), (f"hand-{s}", "angleDeg"), (f"hand-{s}", "abductionDeg")]
side = lambda p, s: (p[0], p[1], -p[2] if s == "l" else p[2])
z = lambda p, s: (p[0], p[1], p[2] if s == "l" else -p[2])


def save():
    write(C, doc)


def solve(pose, s, heel, tip, fixed=None, tip_point=TIP, elbow=None):
    targets = [(f"hand-{s}", side(HEEL, s), z(heel, s)), (f"hand-{s}", side(tip_point, s), z(tip, s))]
    if elbow:
        targets.append((f"forearm-{s}", (0, 0, 0), z(elbow, s), (0.15, 0.15, 0.15)))
    result, worst = ik(C, pose, ARM(s), targets, fixed)
    print(f"[DEBUG] {pose} {s} worst residual {worst:.2f} mm {result}")
    return result


KEY_HEEL, KEY_TIP = (0.441, 0.7412, 0.100), (0.585, 0.7412, 0.048)
t = P["typing"]["bones"]
t.update({"hip-shift": {"dx": -0.065}, "lumbar": {"angleDeg": -4}, "torso": {"angleDeg": -2}})
save()
for s in "lr":
    for bone, fields in solve("typing", s, KEY_HEEL, KEY_TIP).items():
        t[bone].update(fields)
save()

r = P["reading-screen"]["bones"]
r.update({"hip-shift": {"dx": -0.075}, "lumbar": {"angleDeg": 1}, "torso": {"angleDeg": 2}})
save()
for bone, fields in solve("reading-screen", "r", (0.355, 0.7215, 0.075), (0.493, 0.7412, 0.02), tip_point=INDEX).items():
    r[bone].update(fields)
for bone, fields in solve("reading-screen", "l", (0.25, 0.630, 0.105), (0.40, 0.607, 0.088)).items():
    r[bone].update(fields)
save()

b = P["leaning-back"]["bones"]
b.update({"hip-shift": {"dx": -0.08}, "lumbar": {"angleDeg": 5}, "torso": {"angleDeg": 7}})
save()
occiput = fk(C, [("head", (0.03, 0.095, 0.075)), ("head", (0.06, 0.095, -0.072))], "leaning-back")
for bone, fields in solve("leaning-back", "l", occiput[0], occiput[1]).items():
    b[bone].update(fields)
for bone, fields in solve("leaning-back", "r", (0.11, 0.6815, 0.268), (0.26, 0.668, 0.262), elbow=(-0.10, 0.775, 0.25)).items():
    b[bone].update(fields)
save()

STATES = {"home": ((0, 0), (0, 0)), "lift": ((0.001, 0.003), (0.003, 0.014)), "upper": ((0.019, 0.002), (0.019, 0.003))}
PLAN = {0: ("home", "home", -2), 0.6: ("home", "lift", -2), 1.2: ("lift", "home", -2), 1.8: ("home", "home", -2.8), 2.4: ("home", "upper", -2), 3: ("upper", "home", -2), 3.6: ("home", "home", -1.4), 4.2: ("home", "lift", -2)}
clip = next(c for c in a["clips"] if c["id"] == "typing")
for key in clip["keys"]:
    left, right, torso = PLAN[key["t"]]
    key["bones"]["torso"] = {"angleDeg": torso}
    fixed = {"torso": {"angleDeg": torso}, "neck": key["bones"]["neck"], "head": key["bones"]["head"]}
    for s, state in (("l", left), ("r", right)):
        d = STATES[state]
        heel = (KEY_HEEL[0] + d[0][0], KEY_HEEL[1] + d[0][1], KEY_HEEL[2])
        tip = (KEY_TIP[0] + d[1][0], KEY_TIP[1] + d[1][1], KEY_TIP[2])
        for bone, fields in solve("typing", s, heel, tip, fixed).items():
            key["bones"][bone] = fields
save()
print("candidate", C)
