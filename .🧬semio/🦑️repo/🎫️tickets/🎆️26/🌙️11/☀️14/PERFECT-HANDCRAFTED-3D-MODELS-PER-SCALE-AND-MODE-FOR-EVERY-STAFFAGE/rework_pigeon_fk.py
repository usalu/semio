"""🦴 Forward kinematics of the pigeon rig (rest placement and pose offsets) in world axes [ahead, up, left]."""
import json, math

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/pigeon/🔣️.json"


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def tr(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def app(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def rot(angle=0.0, abduction=0.0, twist=0.0):
    """🧭 Rw(angle) · Rv(abduction) · Ru(twist); columns are u, v, w in the parent frame."""
    a, b, t = (math.radians(x) for x in (angle, abduction, twist))
    rw = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    rv = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    ru = [[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]]
    return mul(mul(rw, rv), ru)


def euler(m):
    """📐 Inverse of rot: angle, abduction, twist in degrees."""
    b = math.asin(max(-1, min(1, m[2][0])))
    a = math.atan2(m[1][0], m[0][0])
    t = math.atan2(-m[2][1], m[2][2])
    return [math.degrees(x) for x in (a, b, t)]


def frames(bones, offsets=None):
    """🌍 Bone id -> (origin, rotation) with rotation columns u, v, w in world axes."""
    offsets = offsets or {}
    out = {}
    for bone in bones:
        o = offsets.get(bone["id"], {})
        local = mul(rot(bone["angleDeg"], bone.get("abductionDeg", 0), bone.get("twistDeg", 0)), rot(o.get("angleDeg", 0), o.get("abductionDeg", 0), o.get("twistDeg", 0)))
        if bone["parent"] is None:
            out[bone["id"]] = ([bone.get("x", 0), bone.get("y", 0), bone.get("z", 0)], local)
            continue
        po, pr = out[bone["parent"]]
        d = app(pr, [bone.get("x", 0), bone.get("y", 0), bone.get("z", 0)])
        out[bone["id"]] = ([po[i] + d[i] for i in range(3)], mul(pr, local))
    return out


def load():
    doc = json.load(open(CATALOG))
    return doc, doc.get("asset", doc)


if __name__ == "__main__":
    _, asset = load()
    f = frames(asset["rig"]["bones"])
    for k in ("humerus-n", "s1-n", "p10-f", "toes-n"):
        o, r = f[k]
        print(k, [round(x, 4) for x in o], "u", [round(r[i][0], 3) for i in range(3)], "w", [round(r[i][2], 3) for i in range(3)])
