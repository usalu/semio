"""🎯️ Arm inverse kinematics for the HW-02 figures: finds the offsets of arm, forearm and hand (angle, abduction, twist) that bring the
hand frame to a wanted world frame, by coordinate descent from a coarse grid inside the joint limits of the rig.

The chain is the staffage one (README "Rest placement of a bone"): R(child) = R(parent)·R(rest)·R(offset) with R = Rw(angle)·Rv(abduction)·Ru(twist);
the rest frames of arm, forearm and hand of the base figure are identical (forearm and hand rest straight on the arm), so the chain needs
only the rest frame of the arm `A0` (with zero arm offsets), the shoulder point and the two lengths.
"""
import itertools, math, random
import rot

LIMITS = {"A": (-60, 170), "B": (4, 90), "C": (-60, 60), "F": (0, 150), "t": (-90, 90), "h": (-60, 60), "b": (-45, 45)}


def chain(x, A0, shoulder, LA, LF):
    A, B, C, F, t, h, b = x
    arm = rot.mul(A0, rot.R(A, B, C))
    elbow = [shoulder[i] + arm[i][0] * LA for i in range(3)]
    fore = rot.mul(arm, rot.R(F, 0, t))
    wrist = [elbow[i] + fore[i][0] * LF for i in range(3)]
    hand = rot.mul(fore, rot.R(h, b, 0))
    return hand, wrist, elbow


def angle_between(Ra, Rb):
    M = rot.mul(rot.T(Ra), Rb)
    tr = max(-1.0, min(3.0, M[0][0] + M[1][1] + M[2][2]))
    return math.degrees(math.acos(max(-1.0, min(1.0, (tr - 1) / 2))))


def solve(A0, shoulder, LA, LF, target_R, target_o, side, weight_dir=1.0, elbow_out=300.0, elbow_min=0.19, seed=1):
    """Offsets {arm, forearm, hand} of side `l` or `r` whose hand frame is target_R at origin target_o (world). The left side is solved
    with positive abduction outwards; the right side mirrors abduction and twist signs through `side`."""
    sg = 1 if side == "l" else -1
    rnd = random.Random(seed)

    def cost(x):
        X = (x[0], sg * x[1], sg * x[2], x[3], sg * x[4], x[5], sg * x[6])
        hand, wrist, elbow = chain(X, A0, shoulder, LA, LF)
        e = math.dist(wrist, target_o)
        o = angle_between(hand, target_R)
        pen = sum(max(0, lo - v) + max(0, v - hi) for v, (lo, hi) in zip(x, LIMITS.values()))
        return 400 * e + weight_dir * o + 10 * pen + elbow_out * max(0.0, elbow_min - sg * elbow[2])

    best = None
    for x in itertools.product(range(-30, 121, 30), range(4, 65, 20), (0,), range(0, 151, 30), range(-90, 91, 45), range(-60, 61, 30), range(-40, 41, 20)):
        c = cost(x)
        if best is None or c < best[0]:
            best = (c, list(x))
    c, x = best
    step = 8.0
    while step > 0.05:
        improved = False
        for i in range(7):
            for d in (-step, step):
                y = list(x)
                y[i] = max(LIMITS[list(LIMITS)[i]][0], min(LIMITS[list(LIMITS)[i]][1], y[i] + d))
                cy = cost(y)
                if cy < c:
                    c, x, improved = cy, y, True
        if not improved:
            step /= 2
    X = (x[0], sg * x[1], sg * x[2], x[3], sg * x[4], x[5], sg * x[6])
    hand, wrist, elbow = chain(X, A0, shoulder, LA, LF)
    r = lambda v: round(v, 2)
    return ({f"arm-{side}": {"angleDeg": r(X[0]), "abductionDeg": r(X[1]), "twistDeg": r(X[2])},
             f"forearm-{side}": {"angleDeg": r(X[3]), "twistDeg": r(X[4])},
             f"hand-{side}": {"angleDeg": r(X[5]), "abductionDeg": r(X[6])}},
            {"wristError": round(math.dist(wrist, target_o), 4), "orientationErrorDeg": round(angle_between(hand, target_R), 2), "elbow": [round(v, 3) for v in elbow]})


def look_at(u, w_hint):
    """A right-handed frame (columns u, v, w) with u along `u` and w as close as possible to `w_hint`."""
    n = math.sqrt(sum(c * c for c in u))
    u = [c / n for c in u]
    d = sum(w_hint[i] * u[i] for i in range(3))
    w = [w_hint[i] - d * u[i] for i in range(3)]
    n = math.sqrt(sum(c * c for c in w))
    w = [c / n for c in w]
    v = [w[1] * u[2] - w[2] * u[1], w[2] * u[0] - w[0] * u[2], w[0] * u[1] - w[1] * u[0]]
    return rot.mat([u, v, w])
