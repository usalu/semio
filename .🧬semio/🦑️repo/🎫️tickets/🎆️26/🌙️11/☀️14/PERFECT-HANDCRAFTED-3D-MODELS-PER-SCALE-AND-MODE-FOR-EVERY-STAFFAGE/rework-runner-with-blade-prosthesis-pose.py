"""🙌 runner-with-blade-prosthesis: `hands-on-hips` re-posed so that the heels of the hands rest on the iliac crests and the fingers lie on the front of the hips (they ran into the lumbar spine before).

The wrist, the elbow preference and the hand direction are stated in world rest coordinates [ahead, up, left] for the left side; the offsets are solved through the rig and mirrored to the right.

Usage: python3 rework-runner-with-blade-prosthesis-pose.py [--apply]     without --apply: candidate `🗑️generated/rework-runner-with-blade-prosthesis/pose/🔣️.json`
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
SLUG = "runner-with-blade-prosthesis"
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", SLUG, "🔣️.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-" + SLUG, "pose", "🔣️.json")
spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)

WRIST = (0.0, 1.06, 0.215)
ELBOW_TOWARDS = (-0.62, 0.1, 0.78)
HAND_ALONG = (0.45, -0.8, -0.2)
PALM_AWAY = (0.5, 0.2, 1.0)


def unit(a):
    n = math.sqrt(sum(c * c for c in a))
    return [c / n for c in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def basis(u, w):
    u = unit(u)
    w = unit([w[i] - dot(w, u) * u[i] for i in range(3)])
    v = cross(w, u)
    return [[u[i], v[i], w[i]] for i in range(3)]


def offset(bones, offsets, bone, wanted):
    zero = rig.frames(bones, {key: value for key, value in offsets.items() if key != bone})[bone][1]
    a, b, t = rig.angles(rig.mul(rig.transpose(zero), wanted))
    return {key: value for key, value in (("angleDeg", round(a, 1) + 0.0), ("abductionDeg", round(b, 1) + 0.0), ("twistDeg", round(t, 1) + 0.0)) if key == "angleDeg" or value != 0}


def repose(asset):
    bones = asset["rig"]["bones"]
    lengths = {bone["id"]: bone["lengthM"] for bone in bones}
    pose = next(pose for pose in asset["poses"] if pose["id"] == "hands-on-hips")
    offsets = pose["bones"]
    for bone in ("arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"):
        offsets[bone] = {"angleDeg": 0}
    shoulder = rig.frames(bones, offsets)["arm-l"][0]
    a, f = lengths["arm-l"], lengths["forearm-l"]
    axis = [WRIST[i] - shoulder[i] for i in range(3)]
    d = math.sqrt(dot(axis, axis))
    axis = unit(axis)
    along = (a * a - f * f + d * d) / (2 * d)
    radius = math.sqrt(a * a - along * along)
    out = unit([ELBOW_TOWARDS[i] - dot(ELBOW_TOWARDS, axis) * axis[i] for i in range(3)])
    elbow = [shoulder[i] + along * axis[i] + radius * out[i] for i in range(3)]
    upper, lower = [elbow[i] - shoulder[i] for i in range(3)], [WRIST[i] - elbow[i] for i in range(3)]
    hinge = cross(upper, lower)
    offsets["arm-l"] = offset(bones, offsets, "arm-l", basis(upper, hinge))
    offsets["forearm-l"] = offset(bones, offsets, "forearm-l", basis(lower, hinge))
    offsets["hand-l"] = offset(bones, offsets, "hand-l", basis(HAND_ALONG, PALM_AWAY))
    for bone in ("arm", "forearm", "hand"):
        offsets[f"{bone}-r"] = {key: (-value if key in ("abductionDeg", "twistDeg") else value) for key, value in offsets[f"{bone}-l"].items()}
    solved = rig.frames(bones, offsets)
    print("elbow", [round(c, 3) for c in solved["forearm-l"][0]], "wrist", [round(c, 3) for c in solved["hand-l"][0]], "wrist right", [round(c, 3) for c in solved["hand-r"][0]])
    print(json.dumps({key: value for key, value in offsets.items() if key[:-2] in ("clavicle", "arm", "forearm", "hand")}))


def main():
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"]["poses"])
    repose(document["asset"])
    if "--apply" not in sys.argv:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        return
    text = rig.splice(text, "poses", before, document["asset"]["poses"])
    if open(CATALOG, encoding="utf8").read() != original:
        raise SystemExit("the catalog file changed: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    print("applied")


if __name__ == "__main__":
    main()
