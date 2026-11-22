"""🤳 Stage 3 of humans/leaning-person: the right hand gets the skeleton of the left one, the phone grip becomes a pose.

Before, the right finger bones were measured from the curled grip (index 35 mm, little 42 mm, distal joints resting at 110 degrees).
Now both hands share lengths and rest placement (mirror rule: z, abductionDeg, twistDeg negated) and every pose flexes the right
fingers around the phone: proximal phalanges flat behind the phone, the middle phalanges hooked round its long edge, the little
finger curled round its lower corner, the thumb on the screen with its end joint pressed 12 degrees back. Loaded by rework-leaning-person-build.py after stage 2.
"""

FINGERS = ("index", "middle", "ring", "little")
REST_FLEXION = {"proximal": 14.0, "middle": 22.0, "distal": 22.0}
LIMITS = {"proximal": (-35, 76), "middle": (-25, 90), "distal": (-25, 60)}
GRIP = {
    "index": {"proximal": 3, "middle": 80, "distal": 30},
    "middle": {"proximal": 2, "middle": 78, "distal": 30},
    "ring": {"proximal": 3, "middle": 80, "distal": 28},
    "little": {"proximal": -5, "middle": 25, "distal": 20},
}
THUMB = ("metacarpal", "proximal", "distal")


def apply(a, m):
    bones = a["rig"]["bones"]
    by = {b["id"]: b for b in bones}
    grip = {}
    for seg in THUMB:
        right, left = by[f"thumb-{seg}-r"], by[f"thumb-{seg}-l"]
        old = m.rest_matrix(right["angleDeg"], right.get("abductionDeg", 0), right.get("twistDeg", 0))
        new = m.rest_matrix(left["angleDeg"], -left.get("abductionDeg", 0), -left.get("twistDeg", 0))
        angle, abduction, twist = m.rest_angles(m.mul(m.tr(new), old))
        grip[right["id"]] = {"angleDeg": round(angle, 2), "abductionDeg": round(abduction, 2), "twistDeg": round(twist, 2)}
    grip["thumb-distal-r"]["angleDeg"] = -12.0
    for s in ("l", "r"):
        by[f"thumb-distal-{s}"]["joint"].update({"minDeg": -25, "maxDeg": 80})
    for finger in FINGERS:
        for seg, rest in REST_FLEXION.items():
            grip[f"{finger}-{seg}-r"] = {"angleDeg": round(GRIP[finger][seg] - rest, 2)}
            for s in ("l", "r"):
                by[f"{finger}-{seg}-{s}"]["joint"].update({"minDeg": LIMITS[seg][0], "maxDeg": LIMITS[seg][1]})
    for finger in ("thumb",) + FINGERS:
        for seg in THUMB if finger == "thumb" else REST_FLEXION:
            right, left = by[f"{finger}-{seg}-r"], by[f"{finger}-{seg}-l"]
            for key in ("x", "y", "angleDeg", "lengthM"):
                right[key] = left[key]
            for key in ("z", "abductionDeg", "twistDeg"):
                if key in left:
                    right[key] = -left[key] if left[key] else 0.0
                else:
                    right.pop(key, None)
    print("[DEBUG] right-hand grip offsets:", grip)
    for pose in a["poses"]:
        pose["bones"].update({k: dict(v) for k, v in grip.items()})
    for vol in a["volumes"].values():
        for name in ("body", "thermal", "structure"):
            for s in vol[name]:
                bone = by.get(s.get("bone", ""))
                if s.get("kind") == "capsule" and bone and s["bone"].endswith("-r") and s["bone"].split("-")[0] in ("thumb",) + FINGERS and s["p0"] == [0, 0, 0] or (s.get("kind") == "capsule" and bone and s["bone"].endswith("-r") and s["bone"].split("-")[0] in ("thumb",) + FINGERS and all(abs(c) < 1e-9 for c in s["p0"])):
                    s["p1"] = [bone["lengthM"], 0, 0]
                if s.get("id", "").startswith("metacarpal-") and s["id"].endswith("-r"):
                    first = by[s["id"].replace("metacarpal-", "").replace("-r", "-proximal-r")]
                    s["p1"] = [first["x"], first["y"], first["z"]]
