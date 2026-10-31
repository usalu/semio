"""wheel-loader generator (20 t class, 3.5 m3 bucket, facing right, origin mid-wheelbase on the ground)."""
import math
from common import *

WR = 0.875
XR, XF = -1.7, 1.7
P0 = (1.1, 2.0)
ARM_L = 2.7
ARM0 = -18.4
BUCKET0 = 20 - ARM0
C0 = (0.45, 1.0)
LIFT_PIN = (1.0, -0.3)
TILT_O = (0.9, 0.38)
TILT_PIN = (-0.45, 0.5)
LIFT_BARREL = 1.1
TILT_BARREL = 0.9

_arm = trs(P0[0], P0[1], ARM0)
_buc = mmul(_arm, trs(ARM_L, 0, BUCKET0))
_q0 = ap(_arm, LIFT_PIN)
LIFT_REST = math.degrees(math.atan2(_q0[1] - C0[1], _q0[0] - C0[0]))
LIFT_D0 = math.hypot(_q0[0] - C0[0], _q0[1] - C0[1])
_o0 = ap(_arm, TILT_O)
_t0 = ap(_buc, TILT_PIN)
TILT_REST = math.degrees(math.atan2(_t0[1] - _o0[1], _t0[0] - _o0[0])) - ARM0
TILT_D0 = math.hypot(_t0[0] - _o0[0], _t0[1] - _o0[1])
LIFT_ROD = LIFT_D0 - LIFT_BARREL
TILT_ROD = TILT_D0 - TILT_BARREL

rig = Rig("rear-frame")
rig.add("rear-frame", None, 0, 0, 0, 3.5, joint_fixed(), "Rear frame with engine, hood and counterweight", "Hinterwagen mit Motor, Haube und Gegengewicht", 8190, (-2.2, 1.3))
rig.add("front-frame", "rear-frame", 0, 0, 0, 2.4, joint_fixed(), "Front frame with cab and lift-arm towers (articulation joint is a vertical axis, not an elevation joint)", "Vorderwagen mit Kabine und Hubarmstützen (das Knickgelenk hat eine senkrechte Achse und ist kein Gelenk der Ansicht)", 4100, (0.6, 1.2))
rig.add("lift-arm", "front-frame", P0[0], P0[1], ARM0, ARM_L, joint_rev(-30, 75), "Lift arm", "Hubarm", 1700, (1.4, 0))
rig.add("bucket", "lift-arm", ARM_L, 0, BUCKET0, 1.6, joint_rev(-75, 70), "Bucket 3.5 m3", "Schaufel 3,5 m3", 1550, (0.6, -0.3))
rig.add("lift-cyl", "front-frame", C0[0], C0[1], LIFT_REST, LIFT_BARREL, joint_rev(-40, 60), "Lift cylinder barrel", "Hubzylinder, Rohr", 230, (0.55, 0))
rig.add("lift-rod", "lift-cyl", LIFT_BARREL, 0, 0, LIFT_ROD, joint_pris(0, -0.5, 0.9), "Lift cylinder rod", "Hubzylinder, Kolbenstange", 140, (0.5, 0))
rig.add("tilt-cyl", "lift-arm", TILT_O[0], TILT_O[1], TILT_REST, TILT_BARREL, joint_rev(-80, 80), "Tilt cylinder barrel", "Kippzylinder, Rohr", 120, (0.45, 0))
rig.add("tilt-rod", "tilt-cyl", TILT_BARREL, 0, 0, TILT_ROD, joint_pris(0, -0.5, 0.9), "Tilt cylinder rod", "Kippzylinder, Kolbenstange", 70, (0.4, 0))
rig.add("wheel-r", "rear-frame", XR, WR, 0, WR, joint_rev(), "Rear wheels", "Hinterräder", 2200, (0, 0))
rig.add("wheel-f", "front-frame", XF, WR, 0, WR, joint_rev(), "Front wheels", "Vorderräder", 2200, (0, 0))
G = Geo(rig)
BW = rig.W


def cyl_offsets(arm_off, bucket_off):
    W = rig.world({"lift-arm": {"angleDeg": arm_off}, "bucket": {"angleDeg": bucket_off}})
    q = ap(W["lift-arm"], LIFT_PIN)
    c0 = ap(W["front-frame"], C0)
    a = math.degrees(math.atan2(q[1] - c0[1], q[0] - c0[0]))
    fw = math.degrees(math.atan2(W["front-frame"][1], W["front-frame"][0]))
    lift_ang = a - fw - LIFT_REST
    d1 = math.hypot(q[0] - c0[0], q[1] - c0[1])
    o = ap(W["lift-arm"], TILT_O)
    t = ap(W["bucket"], TILT_PIN)
    b = math.degrees(math.atan2(t[1] - o[1], t[0] - o[0]))
    aw = math.degrees(math.atan2(W["lift-arm"][1], W["lift-arm"][0]))
    tilt_ang = b - aw - TILT_REST
    d2 = math.hypot(t[0] - o[0], t[1] - o[1])
    return {"lift-cyl": {"angleDeg": lift_ang}, "lift-rod": {"dx": d1 - LIFT_D0}, "tilt-cyl": {"angleDeg": tilt_ang}, "tilt-rod": {"dx": d2 - TILT_D0}}


def bones_for(arm_off, bucket_off, wheel=0.0):
    d = {"lift-arm": {"angleDeg": arm_off}, "bucket": {"angleDeg": bucket_off}}
    d.update(cyl_offsets(arm_off, bucket_off))
    if wheel:
        d["wheel-r"] = {"angleDeg": wheel}
        d["wheel-f"] = {"angleDeg": wheel}
    return {k: {kk: round(vv, 4) for kk, vv in v.items()} for k, v in d.items()}


def lip_y(arm_off, bucket_off):
    W = rig.world({"lift-arm": {"angleDeg": arm_off}, "bucket": {"angleDeg": bucket_off}})
    return min(ap(W["bucket"], p)[1] for p in BUCKET_PTS)


BUCKET_PTS = [(-0.45, -0.6), (1.5, -0.8), (1.62, -0.62), (1.05, 0.05), (0.15, 0.6), (-0.35, 0.55), (-0.55, 0.0)]


def lw(bone, pts):
    return [ap(BW[bone], p) for p in pts]


def solve_arm_for_lip0(bucket_off):
    lo, hi = -30.0, 20.0
    for _ in range(40):
        mid = (lo + hi) / 2
        if lip_y(mid, bucket_off) > 0:
            hi = mid
        else:
            lo = mid
    return (lo + hi) / 2


def arm_poly(den):
    up = [(-0.25, 0.32), (1.0, 0.48), (2.0, 0.38), (2.85, 0.25)]
    lo = [(2.85, -0.25), (2.0, -0.22), (1.0, -0.28), (-0.25, -0.32)]
    return lw("lift-arm", up + lo)


def bucket_poly(den):
    if den == 500:
        return lw("bucket", [(-0.45, -0.6), (1.5, -0.8), (1.62, -0.62), (1.05, 0.05), (0.15, 0.6), (-0.35, 0.55), (-0.55, 0.0)])
    return lw("bucket", BUCKET_PTS)


def body500():
    s = []
    s.append(G.poly("rear-body", "rear-frame", "body", 2, [(-3.5, 0.62), (-3.5, 2.05), (-3.35, 2.3), (-3.0, 2.4), (-1.1, 2.4), (-0.9, 2.2), (-0.9, 0.62)], tone=0.45, regions=["engine-bay", "counterweight"], material="paint-steel"))
    s.append(G.poly("front-body", "front-frame", "body", 2, [(-0.9, 0.62), (2.35, 0.62), (2.35, 1.2), (1.5, 1.45), (1.5, 2.2), (1.0, 2.2), (1.0, 1.45), (-0.9, 1.45)], tone=0.45, regions=["front-frame"], material="paint-steel"))
    s.append(G.poly("cab", "front-frame", "body", 2.2, [(-0.9, 1.45), (-0.9, 3.28), (-0.8, 3.4), (0.7, 3.4), (0.82, 3.32), (0.98, 2.2), (1.0, 1.45)], tone=0.55, region="cab", material="paint-steel"))
    s.append(G.poly("cab-glass", "front-frame", "glazing", 2.4, [(0.78, 2.1), (0.66, 3.2), (-0.6, 3.2), (-0.6, 2.1)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.rect("exhaust-stack", "rear-frame", "body", 2.1, -1.32, 2.3, 0.18, 0.85, tone=0.2, region="exhaust-stack", material="exhaust-steel"))
    s.append(G.poly("lift-arm", "lift-arm", "body", 3, arm_poly(500), tone=0.35, regions=["lift-arm", "hydraulics"], material="paint-steel"))
    s.append(G.poly("bucket", "bucket", "body", 3.2, bucket_poly(500), tone=0.25, region="bucket", material="wear-steel"))
    s.append(G.ell("wheel-r", "wheel-r", "body", 4, XR, WR, WR, tone=0.12, region="tyres", material="rubber"))
    s.append(G.ell("wheel-f", "wheel-f", "body", 4, XF, WR, WR, tone=0.12, region="tyres", material="rubber"))
    return s


def body200():
    s = []
    s.append(G.poly("hood", "rear-frame", "body", 2, [(-2.6, 0.62), (-2.6, 1.9), (-3.0, 2.4), (-1.1, 2.4), (-0.9, 2.2), (-0.9, 0.62)], tone=0.45, region="engine-bay", material="paint-steel"))
    s.append(G.poly("counterweight", "rear-frame", "body", 2.05, [(-3.5, 0.62), (-3.5, 2.05), (-3.35, 2.3), (-3.0, 2.4), (-2.6, 1.9), (-2.6, 0.62)], tone=0.3, region="counterweight", material="cast-iron"))
    s.append(G.poly("front-frame", "front-frame", "body", 2, [(-0.9, 0.62), (2.35, 0.62), (2.35, 1.2), (1.5, 1.45), (1.5, 2.2), (1.0, 2.2), (1.0, 1.45), (-0.9, 1.45)], tone=0.45, region="front-frame", material="paint-steel"))
    s.append(G.poly("cab", "front-frame", "body", 2.2, [(-0.9, 1.45), (-0.9, 3.28), (-0.8, 3.4), (0.7, 3.4), (0.82, 3.32), (0.98, 2.2), (1.0, 1.45)], tone=0.55, region="cab", material="paint-steel"))
    s.append(G.poly("cab-glass-front", "front-frame", "glazing", 2.4, [(0.78, 2.1), (0.66, 3.2), (0.2, 3.2), (0.2, 2.1)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.poly("cab-glass-side", "front-frame", "glazing", 2.4, [(0.0, 2.1), (0.0, 3.2), (-0.6, 3.2), (-0.6, 2.1)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.rect("exhaust-stack", "rear-frame", "body", 2.1, -1.32, 2.3, 0.18, 0.85, tone=0.2, region="exhaust-stack", material="exhaust-steel"))
    s.append(G.poly("lift-arm", "lift-arm", "body", 3, arm_poly(200), tone=0.35, region="lift-arm", material="paint-steel"))
    s.append(G.poly("lift-cyl-barrel", "lift-cyl", "detail", 2.9, lw("lift-cyl", [(0, -0.12), (LIFT_BARREL, -0.12), (LIFT_BARREL, 0.12), (0, 0.12)]), tone=0.5, region="hydraulics", material="paint-steel"))
    s.append(G.poly("lift-rod", "lift-rod", "detail", 2.95, lw("lift-rod", [(0, -0.05), (LIFT_ROD, -0.05), (LIFT_ROD, 0.05), (0, 0.05)]), tone=0.8, region="hydraulics", material="chrome"))
    s.append(G.poly("tilt-cyl-barrel", "tilt-cyl", "detail", 3.3, lw("tilt-cyl", [(0, -0.1), (TILT_BARREL, -0.1), (TILT_BARREL, 0.1), (0, 0.1)]), tone=0.5, region="hydraulics", material="paint-steel"))
    s.append(G.poly("tilt-rod", "tilt-rod", "detail", 3.35, lw("tilt-rod", [(0, -0.045), (TILT_ROD, -0.045), (TILT_ROD, 0.045), (0, 0.045)]), tone=0.8, region="hydraulics", material="chrome"))
    s.append(G.poly("bucket", "bucket", "body", 3.2, bucket_poly(200), tone=0.25, region="bucket", material="wear-steel"))
    for b, x in (("wheel-r", XR), ("wheel-f", XF)):
        s.append(G.ell(f"tyre-{b[-1]}", b, "body", 4, x, WR, WR, tone=0.12, region="tyres", material="rubber"))
        s.append(G.ell(f"rim-{b[-1]}", b, "detail", 4.2, x, WR, 0.5, tone=0.6, region="tyres", material="cast-iron"))
    return s


def body100(fine):
    s = body200()
    s = [x for x in s if x["id"] not in ("cab-glass-front", "cab-glass-side")]
    s.append(G.poly("cab-glass-front", "front-frame", "glazing", 2.4, [(0.78, 2.1), (0.66, 3.2), (0.2, 3.2), (0.2, 2.1)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.poly("cab-glass-side", "front-frame", "glazing", 2.4, [(0.0, 2.1), (0.0, 3.2), (-0.6, 3.2), (-0.6, 2.1)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.poly("handrail", "rear-frame", "detail", 2.3, [(-2.9, 2.4), (-2.9, 2.62), (-1.4, 2.62), (-1.4, 2.4)], closed=False, tone=0.3, filled=False, region="engine-bay", material="paint-steel"))
    s.append(G.poly("grille", "rear-frame", "detail", 2.3, [(-3.46, 1.0), (-3.2, 1.0), (-3.2, 1.8), (-3.46, 1.8)], tone=0.2, region="counterweight", material="cast-iron"))
    s.append(G.poly("access-step", "front-frame", "detail", 1.9, [(-0.9, 0.9), (-0.5, 0.9), (-0.5, 1.1), (-0.9, 1.1)], tone=0.3, region="front-frame", material="paint-steel"))
    s.append(G.poly("bucket-edge", "bucket", "detail", 3.4, lw("bucket", [(1.3, -0.77), (1.62, -0.62), (1.5, -0.8)]), tone=0.1, region="bucket", material="wear-steel"))
    for b, x in (("wheel-r", XR), ("wheel-f", XF)):
        s.append(G.ell(f"hub-{b[-1]}", b, "detail", 4.3, x, WR, 0.14, tone=0.3, region="tyres", material="cast-iron"))
        for k in range(6):
            a = math.radians(90 + 60 * k)
            s.append(G.ell(f"bolt-{b[-1]}-{k + 1}", b, "detail", 4.3, x + 0.3 * math.cos(a), WR + 0.3 * math.sin(a), 0.045, tone=0.0, filled=False, region="tyres", material="cast-iron"))
    if fine:
        for b, x in (("wheel-r", XR), ("wheel-f", XF)):
            lugs = []
            n = 20
            for k in range(n):
                a0 = 360 / n * k
                lugs += [(x + (WR - 0.09) * math.cos(math.radians(a0)), WR + (WR - 0.09) * math.sin(math.radians(a0))), (x + (WR) * math.cos(math.radians(a0 + 2)), WR + (WR) * math.sin(math.radians(a0 + 2))), (x + (WR) * math.cos(math.radians(a0 + 11)), WR + (WR) * math.sin(math.radians(a0 + 11))), (x + (WR - 0.09) * math.cos(math.radians(a0 + 13)), WR + (WR - 0.09) * math.sin(math.radians(a0 + 13)))]
            s.append(G.poly(f"lugs-{b[-1]}", b, "detail", 3.95, lugs, tone=0.05, filled=False, region="tyres", material="rubber"))
        s.append(G.poly("head-lamp", "front-frame", "detail", 2.5, [(2.3, 0.95), (2.35, 0.95), (2.35, 1.15), (2.3, 1.15)], tone=0.9, region="front-frame", material="glass"))
        s.append(G.poly("mirror", "front-frame", "detail", 2.5, [(0.85, 3.25), (1.1, 3.25), (1.1, 3.0), (0.95, 3.0)], tone=0.2, region="cab", material="paint-steel"))
        s.append(G.poly("door-handle", "front-frame", "detail", 2.5, [(-0.2, 2.0), (0.2, 2.0), (0.2, 2.05), (-0.2, 2.05)], tone=0.2, region="cab", material="paint-steel"))
        s.append(G.line("hood-louvres", "rear-frame", "detail", 2.3, [(-2.3, 2.1), (-1.2, 2.1)], tone=0.3, region="engine-bay", material="paint-steel"))
        s.append(G.line("hood-louvres-2", "rear-frame", "detail", 2.3, [(-2.3, 1.85), (-1.2, 1.85)], tone=0.3, region="engine-bay", material="paint-steel"))
    return s
