"""🤳 Solves the arm offsets of the three poses of the standing phone user on the corrected rig (`r3_standing_phone_user_rig`): texting (phone on the midline in front of the lower chest, both hands mirrored on it), phone call (phone at the right ear, left forearm across the waist) and photo (phone upright at eye height at arm's length, both hands mirrored). Each solution is the arm posture closest to a stated natural prior that meets the stated phone or hand target. Prints the offsets; the chosen values are written by hand into `r3-standing-phone-user-roots.py`.

Usage: python3 r3-standing-phone-user-grip.py
"""
import math

import r3_standing_phone_user_rig as rig

LEGS = {"thigh": -1, "shank": -5, "foot": 6}
NAMES = ["arm.angle", "arm.abduction", "arm.twist", "forearm.angle", "forearm.twist", "hand.angle", "hand.abduction", "hand.twist", "phone.angle"]
WEIGHT = [0.02, 0.05, 0.02, 0.01, 0.01, 0.2, 0.2, 0.1, 0.05]
LIMIT = [(-60, 170), (-90, 90), (-90, 90), (0, 150), (-90, 90), (-60, 60), (-35, 35), (-30, 30), (-75, 30)]
N = 9


def offsets(spine, right, left=None):
    out = {"torso": (spine[0],), "neck": (spine[1],), "head": (spine[2],)}
    for side in "lr":
        for part, deg in LEGS.items():
            out[part + "-" + side] = (deg,)
    for side, sign, p in (("r", 1, right), ("l", -1, left or right)):
        m = 1 if left and side == "l" else sign
        out["arm-" + side] = (p[0], m * p[1], m * p[2])
        out["forearm-" + side] = (p[3], 0, m * p[4])
        out["hand-" + side] = (p[5], m * p[6], m * p[7])
    out["phone"] = (right[8],)
    return out


def unit_xy(deg):
    r = math.radians(deg)
    return (math.cos(r), math.sin(r), 0)


def phone_target(centre, u, w):
    def res(frames):
        f = frames["phone"]
        c = rig.at(f, 0.0735)
        return [(c[i] - centre[i]) * 20 for i in range(3)] + [(f["u"][i] - u[i]) * 3 for i in range(3)] + [(f["w"][i] - w[i]) * 3 for i in range(3)]
    return res


def hand_target(side, wrist, u, w):
    def res(frames):
        f = frames["hand-" + side]
        return [(f["o"][i] - wrist[i]) * 20 for i in range(3)] + [(f["u"][i] - u[i]) * 2 for i in range(3)] + [(f["w"][i] - w[i]) * 2 for i in range(3)]
    return res


def solve(prior, target, build, free=True, weight=WEIGHT):
    def residual(p):
        return target(rig.chain(build(p))) + [(p[i] - prior[i]) * weight[i] * 0.05 for i in range(N)]
    p, lam = list(prior), 1e-3
    for _ in range(500):
        r0 = residual(p)
        J = []
        for i in range(N):
            q = list(p)
            q[i] += 1e-4
            J.append([(a - b) / 1e-4 for a, b in zip(residual(q), r0)])
        M = [[sum(J[i][k] * J[j][k] for k in range(len(r0))) + (lam if i == j else 0) for j in range(N)] + [-sum(J[i][k] * r0[k] for k in range(len(r0)))] for i in range(N)]
        for i in range(N):
            piv = max(range(i, N), key=lambda r: abs(M[r][i]))
            M[i], M[piv] = M[piv], M[i]
            for r in range(N):
                if r != i:
                    k = M[r][i] / M[i][i]
                    M[r] = [a - k * b for a, b in zip(M[r], M[i])]
        q = [min(max(p[i] + M[i][N] / M[i][i], LIMIT[i][0]), LIMIT[i][1]) for i in range(N)]
        if not free:
            q[8] = prior[8]
        if sum(x * x for x in residual(q)) < sum(x * x for x in r0):
            p, lam = q, lam * 0.5
        else:
            lam *= 4
    p = [round(v, 1) for v in p]
    return p, max(abs(x) for x in target(rig.chain(build(p))))


def show(title, p, worst, frames, bones):
    print("==", title, " worst target residual %.4f" % worst)
    print("  " + ", ".join("%s %.1f" % (n, v) for n, v in zip(NAMES, p)))
    for b in bones:
        f = frames[b]
        print("  %-10s o %s u %s w %s" % (b, tuple(round(x, 4) for x in f["o"]), tuple(round(x, 3) for x in f["u"]), tuple(round(x, 3) for x in f["w"])))


UNDER = 0.026


def left_under(frames):
    """🤲 The frame the left hand must take: the mirror image of the right hand, `UNDER` metres further behind the phone, so that the left hand cradles the fingers of the right."""
    r, n = frames["hand-r"], frames["phone"]["w"]
    m = lambda v: (v[0], v[1], -v[2])
    o = m(r["o"])
    return tuple(o[i] - UNDER * n[i] for i in range(3)), m(r["u"]), tuple(-x for x in m(r["w"]))


def mirrored(p):
    return [p[0], -p[1], -p[2], p[3], -p[4], p[5], -p[6], -p[7], 0]


TEXT_SPINE, CALL_SPINE, PHOTO_SPINE = (-3, -18, -22), (0, -2, 3), (1, 0, 2)
TUCKED = [0.02, 0.4, 0.02, 0.01, 0.01, 0.2, 0.2, 0.1, 0.05]
text, worst = solve([8, -5, -32.7, 101.4, 78.7, -4.3, 20.1, -1.7, 0], phone_target((0.290, 1.190, 0), unit_xy(32), unit_xy(122)), lambda p: offsets(TEXT_SPINE, p), free=False, weight=TUCKED)
fr = rig.chain(offsets(TEXT_SPINE, text))
wrist, u, w = left_under(fr)
text_l, worst_l = solve(mirrored(text), hand_target("l", wrist, u, w), lambda p: offsets(TEXT_SPINE, text, p[:8]), free=False, weight=TUCKED)
fr = rig.chain(offsets(TEXT_SPINE, text, text_l))
show("texting right", text, worst, fr, ("arm-r", "forearm-r", "hand-r", "phone", "head"))
show("texting left (signs as written)", text_l, worst_l, fr, ("arm-l", "forearm-l", "hand-l"))
print("  phone centre", tuple(round(x, 4) for x in rig.at(fr["phone"], 0.0735)), "vertex", tuple(round(x, 4) for x in rig.at(fr["head"], 0.149)))

head = rig.chain(offsets(CALL_SPINE, text))["head"]
ear = rig.at(head, 0.015, 0.016, -0.0785)
axis = unit_xy(180 - 50)
centre = (ear[0] - axis[0] * 0.055, ear[1] - axis[1] * 0.035, ear[2] - 0.014)
call, worst = solve([50, -15, -10, 148, 0, 0, 0, 0, -65], phone_target(centre, axis, (0, 0, 1)), lambda p: offsets(CALL_SPINE, p, [5, 5, 60, 95, 0, 0, 0, 0, 0]))
fold, worst2 = solve([8, 6, 70, 95, 0, 10, 0, 0, 0], hand_target("l", (0.170, 1.075, -0.020), (0.10, -0.05, -0.99), (0.99, 0.0, 0.10)), lambda p: offsets(CALL_SPINE, call, p[:8]))
fr = rig.chain(offsets(CALL_SPINE, call, fold))
show("phone-call right arm", call, worst, fr, ("arm-r", "forearm-r", "hand-r", "phone"))
print("  right ear", tuple(round(x, 4) for x in ear), "phone centre", tuple(round(x, 4) for x in rig.at(fr["phone"], 0.0735)))
show("phone-call left arm (signs as written)", fold, worst2, fr, ("arm-l", "forearm-l", "hand-l"))

photo, worst = solve([50, -10, -30, 80, 60, 0, 0, 0, 0], phone_target((0.400, 1.545, 0), unit_xy(80), unit_xy(170)), lambda p: offsets(PHOTO_SPINE, p), free=False)
fr = rig.chain(offsets(PHOTO_SPINE, photo))
wrist, u, w = left_under(fr)
photo_l, worst_l = solve(mirrored(photo), hand_target("l", wrist, u, w), lambda p: offsets(PHOTO_SPINE, photo, p[:8]), free=False)
fr = rig.chain(offsets(PHOTO_SPINE, photo, photo_l))
show("photo-pose right", photo, worst, fr, ("arm-r", "forearm-r", "hand-r", "phone", "head"))
show("photo-pose left (signs as written)", photo_l, worst_l, fr, ("arm-l", "forearm-l", "hand-l"))
