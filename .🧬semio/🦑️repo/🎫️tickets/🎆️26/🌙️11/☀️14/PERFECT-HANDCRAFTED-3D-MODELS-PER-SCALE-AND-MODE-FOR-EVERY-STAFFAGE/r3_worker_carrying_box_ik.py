"""🤲 Arm solver of the round-3 worker carrying a box: finds, for one pose, the shoulder, elbow, forearm-twist and wrist offsets that put the knuckles of the hand (anchor `grip`) on the hand hole of the box with the hand hooked across the slot, and the offsets of the box bone that keep the box level and square to the walking direction.

Newton iteration with a numeric Jacobian on the product's own chain (`r3_worker_carrying_box_kit.solve`); the right arm is the mirror of the left.
"""
import math

import r3_worker_carrying_box_kit as k

GRIP = 0.0759
FEET = ["contact-l", "contact-r", "heel-l", "heel-r"]


def gauss(a, b):
    n = len(b)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(m[r][c]))
        m[c], m[p] = m[p], m[c]
        if abs(m[c][c]) < 1e-12:
            m[c][c] = 1e-12
        for r in range(n):
            if r != c:
                f = m[r][c] / m[c][c]
                for j in range(c, n + 1):
                    m[r][j] -= f * m[c][j]
    return [m[i][n] / m[i][i] for i in range(n)]


def newton(residual, q, steps=200, damp=1e-7):
    for _ in range(steps):
        r = residual(q)
        if max(abs(x) for x in r) < 1e-9:
            break
        n = len(q)
        jac = [[0.0] * n for _ in r]
        for j in range(n):
            p = q[:]
            p[j] += 1e-4
            rp = residual(p)
            for i in range(len(r)):
                jac[i][j] = (rp[i] - r[i]) / 1e-4
        jt = [[jac[i][j] for i in range(len(r))] for j in range(n)]
        a = [[sum(jt[i][x] * jac[x][j] for x in range(len(r))) + (damp if i == j else 0) for j in range(n)] for i in range(n)]
        g = [-sum(jt[i][x] * r[x] for x in range(len(r))) for i in range(n)]
        step = gauss(a, g)
        scale = min(1.0, 25.0 / max(1e-9, max(abs(s) for s in step)))
        q = [q[i] + scale * step[i] for i in range(n)]
    return q, residual(q)


def hand_target(phi, psi):
    """✋ Axes of the left hand hooked into the slot: fingers down, leaning `phi` degrees forward about the lateral axis and `psi` degrees with the wrist away from the box about the walking direction."""
    f, s = math.radians(phi), math.radians(psi)
    rz = lambda v: (v[0] * math.cos(f) - v[1] * math.sin(f), v[0] * math.sin(f) + v[1] * math.cos(f), v[2])
    rx = lambda v: (v[0], v[1] * math.cos(s) - v[2] * math.sin(s), v[1] * math.sin(s) + v[2] * math.cos(s))
    return tuple(rx(rz(v)) for v in ((0.0, -1.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0)))


def wrap(deg):
    return (deg + 180.0) % 360.0 - 180.0


def solve_arms(asset, bones, grip, phi=0.0, psi=0.0, elbow=(-1.0, -0.5, 0.3)):
    """💪 Offsets of both arms and both box bones for the pose offsets `bones` (legs, trunk, head) and the left grip point `grip` = (x, y, z) in the grounded world.

    The wrist follows from the grip and the hand axes; the elbow lies on the circle that the two arm lengths leave, on the side the direction `elbow` points to; shoulder flexion, spread and twist and the elbow flexion follow in closed form from the product's chain, the forearm twist and the two wrist angles by iteration."""
    target = hand_target(phi, psi)
    rig = {b["id"]: b for b in asset["rig"]["bones"]}
    l1, l2 = rig["forearm-l"]["x"], rig["hand-l"]["x"]
    start = k.grounded(asset, bones, FEET)
    parent = start["torso"]
    s0 = start["upper-arm-l"]["o"]
    wrist = k.sub3(grip, k.comb(target[0], GRIP, k.X, 0))
    span = k.sub3(wrist, s0)
    d = math.sqrt(k.dot(span, span))
    if d >= l1 + l2 - 1e-6:
        raise SystemExit("grip out of reach: %.4f m of %.4f m" % (d, l1 + l2))
    n = k.unit(span)
    along = (l1 * l1 - l2 * l2 + d * d) / (2 * d)
    off = math.sqrt(max(l1 * l1 - along * along, 0.0))
    side = k.unit(k.sub3(elbow, k.comb(n, k.dot(elbow, n), k.X, 0)))
    e = k.add(k.add(s0, k.comb(n, along, k.X, 0)), k.comb(side, off, k.X, 0))
    d1, d2 = k.unit(k.sub3(e, s0)), k.unit(k.sub3(wrist, e))
    b1 = math.asin(k.clamp(k.dot(d1, parent["w"]), -1, 1))
    flat = k.unit(k.sub3(d1, k.comb(parent["w"], k.dot(d1, parent["w"]), k.X, 0)))
    total = math.atan2(k.dot(flat, parent["v"]), k.dot(flat, parent["u"]))
    a1 = wrap(math.degrees(total) - rig["upper-arm-l"]["angleDeg"])
    v1 = k.comb(parent["u"], -math.sin(total), parent["v"], math.cos(total))
    w2 = k.comb(flat, -math.sin(b1), parent["w"], math.cos(b1))
    best = None
    for turn in (0.0, math.pi):
        t1 = math.atan2(-k.dot(d2, w2), k.dot(d2, v1)) + turn
        vt = k.comb(v1, math.cos(t1), w2, -math.sin(t1))
        a2 = math.degrees(math.atan2(k.dot(d2, vt), k.dot(d2, d1)))
        if best is None or (0 <= a2 <= 150 and not 0 <= best[1] <= 150):
            best = (wrap(math.degrees(t1)), a2)
    t1, a2 = best
    head = [a1, math.degrees(b1), t1, a2]

    def offsets(q, side):
        s = 1 if side == "l" else -1
        out = dict(bones)
        out["upper-arm-" + side] = {"angleDeg": head[0], "abductionDeg": s * head[1], "twistDeg": s * head[2]}
        out["forearm-" + side] = {"angleDeg": head[3], "twistDeg": s * q[0]}
        out["hand-" + side] = {"angleDeg": q[1], "abductionDeg": s * q[2]}
        return out

    def residual(q):
        f = k.grounded(asset, offsets(q, "l"), FEET)["hand-l"]
        rot = (0.0, 0.0, 0.0)
        for axis, want in zip(("u", "v", "w"), target):
            rot = k.add(rot, k.cross(f[axis], want))
        return list(rot)

    q, r = None, None
    for guess in ([0.0, -20.0, 0.0], [0.0, 20.0, 0.0], [60.0, 0.0, 0.0], [-60.0, 0.0, 0.0], *([t, w, a] for t in (-150.0, -100.0, -50.0, 50.0, 100.0, 150.0) for w in (-40.0, 0.0, 40.0) for a in (-40.0, 40.0))):
        q, r = newton(residual, guess)
        f = k.grounded(asset, offsets(q, "l"), FEET)["hand-l"]
        if max(abs(x) for x in r) < 2e-6 and k.dot(f["u"], target[0]) > 0.99:
            break
    else:
        raise SystemExit("wrist solver did not converge: %s residual %s" % (q, r))
    f = k.grounded(asset, offsets(q, "l"), FEET)["hand-l"]
    g = k.add(f["o"], k.comb(f["u"], GRIP, k.X, 0))
    miss = max(abs(g[i] - grip[i]) for i in range(3))
    if miss > 1e-4:
        raise SystemExit("arm misses the grip by %.4f m (joint limit): %s %s" % (miss, head, q))

    def box_residual(b):
        o = offsets(q, "l")
        o["box-l"] = {"angleDeg": b[0], "abductionDeg": b[1], "twistDeg": b[2]}
        f = k.grounded(asset, o, FEET)["box-l"]
        rot = k.add(k.add(k.cross(f["u"], k.X), k.cross(f["v"], k.Y)), k.cross(f["w"], k.Z))
        return list(rot)

    b, rb = newton(box_residual, [-phi, 0.0, 0.0])
    if max(abs(x) for x in rb) > 1e-6:
        raise SystemExit("box solver did not converge: %s residual %s" % (b, rb))
    out = {}
    for side, s in (("l", 1), ("r", -1)):
        arm = offsets(q, side)
        for bone in ("upper-arm-", "forearm-", "hand-"):
            out[bone + side] = {key: round(value, 3) for key, value in arm[bone + side].items() if key == "angleDeg" or abs(value) > 5e-4}
        box = {"angleDeg": round(b[0], 3), "abductionDeg": round(s * b[1], 3), "twistDeg": round(s * b[2], 3)}
        out["box-" + side] = {key: value for key, value in box.items() if key == "angleDeg" or abs(value) > 5e-4}
    return out
