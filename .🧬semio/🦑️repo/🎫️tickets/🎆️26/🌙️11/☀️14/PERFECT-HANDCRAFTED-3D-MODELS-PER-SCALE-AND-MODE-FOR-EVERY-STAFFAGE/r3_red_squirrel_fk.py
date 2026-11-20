"""🦴 Round 3 red squirrel: forward kinematics of the staffage rig in three dimensions (the product's `chain`: sagittal angle, abduction about the bone normal, bone.z), ground `auto`, and a numeric two-angle inverse kinematics for limb chains."""
import math


def sc(deg):
    r = math.radians(deg)
    return math.sin(r), math.cos(r)


def comb(a, s, b, t):
    return [a[i] * s + b[i] * t for i in range(3)]


def chain(bones, offsets, root=(0.0, 0.0, 0.0)):
    rs, rc = sc(root[2])
    base = dict(o=[root[0], root[1], 0.0], u=[rc, rs, 0.0], v=[-rs, rc, 0.0], w=[0.0, 0.0, 1.0])
    out = {}
    for b in bones:
        p = out[b["parent"]] if b["parent"] else base
        off = offsets.get(b["id"], {})
        along = comb(p["u"], b["x"], p["v"], b["y"])
        side = b.get("z", 0.0)
        o = [p["o"][i] + along[i] + p["w"][i] * side for i in range(3)]
        ts, tc = sc(b["angleDeg"] + off.get("angleDeg", 0.0))
        u1 = comb(p["u"], tc, p["v"], ts)
        v1 = comb(p["u"], -ts, p["v"], tc)
        ps, pc = sc(off.get("abductionDeg", 0.0))
        u2 = comb(u1, pc, p["w"], ps)
        w2 = comb(u1, -ps, p["w"], pc)
        out[b["id"]] = dict(o=o, u=u2, v=v1, w=w2, L=b["lengthM"])
    return out


def at(frame, u, v=0.0, w=0.0):
    return [frame["o"][i] + frame["u"][i] * u + frame["v"][i] * v + frame["w"][i] * w for i in range(3)]


def tip(frame):
    return at(frame, frame["L"])


def grounded(bones, anchors, pose):
    """The frames of a pose with ground `auto` applied (lowest listed contact anchor on y = 0)."""
    root = pose.get("root", {})
    r = (root.get("x", 0.0), root.get("y", 0.0), root.get("angleDeg", 0.0))
    f = chain(bones, pose["bones"], r)
    if pose.get("ground") == "auto" and pose.get("contacts"):
        by = {a["id"]: a for a in anchors}
        low = min(at(f[by[c]["bone"]], by[c]["x"], by[c]["y"], by[c].get("z", 0.0))[1] for c in pose["contacts"])
        f = chain(bones, pose["bones"], (r[0], r[1] - low, r[2]))
    return f


def solve2(fn, target, guess, steps=400):
    """Newton iteration on two unknowns: fn(a, b) -> (x, y) reaches target."""
    a, b = guess
    for _ in range(steps):
        x, y = fn(a, b)
        ex, ey = target[0] - x, target[1] - y
        if abs(ex) + abs(ey) < 1e-9:
            break
        h = 1e-3
        xa, ya = fn(a + h, b)
        xb, yb = fn(a, b + h)
        j11, j21, j12, j22 = (xa - x) / h, (ya - y) / h, (xb - x) / h, (yb - y) / h
        det = j11 * j22 - j12 * j21
        if abs(det) < 1e-12:
            a += 1.0
            continue
        da, db = (ex * j22 - ey * j12) / det, (j11 * ey - j21 * ex) / det
        m = max(abs(da), abs(db), 1e-12)
        k = min(1.0, 6.0 / m)
        a += da * k
        b += db * k
    return a, b
