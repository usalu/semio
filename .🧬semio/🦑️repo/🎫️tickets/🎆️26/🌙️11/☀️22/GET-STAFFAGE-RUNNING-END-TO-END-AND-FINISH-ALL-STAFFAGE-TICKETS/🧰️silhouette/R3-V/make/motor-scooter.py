"""🛵️ Writes the hand-made repair ops of `motor-scooter` into ops/motor-scooter.json (after the outlines of make/motor-scooter.sh).

Plan of 1:50 and 1:100: the front shock absorber stands outside the mudguard on the right of the wheel and was not drawn; at 1:50 its spring, its damper tube and its two mounting pins are drawn below the mudguard (the tube above the spring, so that no drawn strip is narrower than 0.2 mm on paper), at 1:100 the shock is one prism of the spring diameter over the length of the damper. The two hand levers were one prism over the whole box of the lever at 1:50 (it closed the 15 mm between grip and lever) and missing at 1:100: they are drawn along the sections of the body, 14 to 16 mm thick at 1:50 and 20 mm (0.2 mm on paper) at 1:100. The body of 1:500 was 0.717 m wide where the scooter is 0.74 m over its mirrors (the other three bodies are): the mirror heads of 1:500 stand 11 mm further out. Every part is a prism on the steering bone: its outline in the plane of the bone axis and the lateral axis, swept through its thickness along the bone normal, so it turns with the steering and reads right in the three views."""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, ".."))
from lib import asset, shape, solid

SLUG = "motor-scooter"
a = asset(SLUG)
ops = []
r5 = lambda n: round(n, 5)


def prism(scale, after, sid, base, z, tone, points, v0, v1):
    """🧱️ The op of a prism on the steering bone: `points` in (u, w), swept along v from `v0` to `v1`."""
    return {"op": "shape-add", "scale": scale, "view": "plan", "after": after, "shape": {"kind": "poly", "id": sid, "bone": "steer", "role": "detail", "z": z, "filled": True, "tone": tone, **base, "penMm": 0.13, "points": [[r5(u), r5(w)] for u, w in points], "closed": True, "solid": {"kind": "extruded", "sweep": "v", "z0": r5(v0), "z1": r5(v1)}}}


box = lambda u0, u1, w0, w1: [(u0, w0), (u1, w0), (u1, w1), (u0, w1)]
IRON = {"region": "steer-assembly", "material": "cast-iron"}
ALLOY = {"region": "steer-assembly", "material": "alloy"}
for side, sign in (("l", 1), ("r", -1)):
    tip = solid(a, "500", f"mirror-{side}")["p1"]
    ops.append({"op": "solid-set", "scale": "500", "id": f"mirror-{side}", "set": {"p1": [tip[0], tip[1], sign * 0.325]}})
for scale, pins in (("50", True), ("100", False)):
    plan = a["representations"][scale]["geometry"]["plan"]
    body, spring, top, foot = (solid(a, scale, f"shock-f-{k}") for k in ("body", "spring", "top", "foot"))
    after = next(s["id"] for s in plan if s["bone"] == "steer")
    z = shape(a, scale, "plan", "mudguard")["z"] - 0.5
    (u0, v, w), u1, r = body["p0"], body["p1"][0], body["r0"]
    (s0, sv, sw), (length, sr) = spring["origin"], spring["profile"][1]
    if pins:
        parts = [("shock-f-spring", 0.2, z, box(s0 - length, s0, sw - sr, sw + sr), sv - sr, sv + sr), ("shock-f-body", 0.4, z + 0.1, box(u1 - r, u0 + r, w - r, w + r), v - r, v + r)]
        for sid, s in (("shock-f-top", top), ("shock-f-foot", foot)):
            (p0, pv, pw), (length, pr) = s["origin"], s["profile"][1]
            parts.append((sid, 0.4, z + 0.2, box(p0 - pr, p0 + pr, pw - length, pw), pv - pr, pv + pr))
    else:
        parts = [("shock-f", 0.3, z, box(u1 - r, u0 + r, sw - sr, sw + sr), sv - sr, sv + sr)]
    for sid, tone, zz, points, v0, v1 in parts:
        ops.append(prism(scale, after, sid, IRON, round(zz, 2), tone, points, v0, v1))
        after = sid
    for side in ("l", "r"):
        sec = solid(a, scale, f"lever-{side}")["sections"]
        grip = shape(a, scale, "plan", f"grip-{side}")
        if scale == "50":
            ops.append({"op": "shape-remove", "scale": scale, "view": "plan", "id": f"lever-{side}"})
        after = f"grip-{side}"
        for n, (i, j) in enumerate([(0, 1), (1, 2), (2, 3)] if scale == "50" else [(0, 1), (1, 3)], 1):
            (ui, vi, wi), (uj, vj, wj) = sec[i]["at"], sec[j]["at"]
            ri, rj = (0.01, 0.01) if scale == "100" else (sec[i]["rx"], sec[j]["rx"])
            rv = 0.01 if scale == "100" else max(sec[i]["ry"], sec[j]["ry"])
            sid = f"lever-{side}-{n}"
            ops.append(prism(scale, after, sid, ALLOY, round(grip["z"] - 0.4 + n / 10, 2), 0.74, [(ui - ri, wi), (ui + ri, wi), (uj + rj, wj), (uj - rj, wj)], min(vi, vj) - rv, max(vi, vj) + rv))
            after = sid
subprocess.run([sys.executable, os.path.join(HERE, "..", "hand.py"), SLUG], input=json.dumps(ops), text=True, check=True)
