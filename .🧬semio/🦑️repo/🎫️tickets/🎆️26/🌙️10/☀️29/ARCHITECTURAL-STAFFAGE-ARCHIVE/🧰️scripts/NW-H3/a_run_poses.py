from a_run_rig import *
from gait import geom_of, leg_state, ankle_from
GEN_DIR = GEN
G = geom_of(a)
AL = G["alpha"]


def blade_leg(off, root, P, d_deg):
    """two-link IK of thigh and shank where the second link is the rigid socket-plus-blade body for a prescribed tip flexion d"""
    d = math.radians(d_deg)
    q = (SOC + B1 * math.cos(math.radians(-15)), B1 * math.sin(math.radians(-15)))
    ang = math.radians(-15 + 82) + d
    ps = (q[0] + B2 * math.cos(ang), q[1] + B2 * math.sin(ang))
    l2 = math.hypot(*ps)
    psi = math.atan2(ps[1], ps[0])
    W = Wof(a, off, root)
    h = (W["thigh-r"][4], W["thigh-r"][5])
    l1 = bone(rig, "shank-r")["x"]
    t1, t2, clipped = ik_two(h, l1, l2, P, +1)
    set_world(a, off, W, "thigh-r", t1)
    W = Wof(a, off, root)
    set_world(a, off, W, "shank-r", t2 - psi)
    off.setdefault("blade-r", {})["angleDeg"] = 0.0
    off.setdefault("blade-tip-r", {})["angleDeg"] = round(d_deg, 4)
    return clipped


def intact_leg(off, root, ankle, d_rad, flex):
    clipped = solve_chain(a, off, root, "thigh-l", "shank-l", ankle, +1)
    W = Wof(a, off, root)
    set_world(a, off, W, "foot-l", -AL + d_rad)
    off.setdefault("toe-l", {})["angleDeg"] = round(min(max(flex, 0), 60), 4)
    return clipped


def standing(spec):
    off = {k: {"angleDeg": v} for k, v in spec.get("trunk", {}).items()}
    ypel = spec["ypel"]
    root = {"x": 0.0, "y": ypel - HIP}
    W = Wof(a, off, root)
    hx = W["thigh-l"][4]
    intact_leg(off, root, (hx + spec["xl"], G["ankle"]), 0.0, 0.0)
    blade_leg(off, root, (hx + spec["xr"], 0.0), spec.get("defl", 0.0))
    for sd, v in spec.get("arms", {}).items():
        for bid, ang in zip((f"arm-{sd}", f"forearm-{sd}", f"hand-{sd}"), v):
            off.setdefault(bid, {})["angleDeg"] = ang
    return pose_clean(off), root


def reach_ok(off, root):
    return True


# ---- running cycle
N, T_, V = 21, 0.7, 3.4
STR = V * T_
SL = dict(heel=None, b0=0.0, b1=0.31, X0=0.32 / STR, clear=0.20, stride=STR, lin=True,
          delta_keys=[(0, -8), (0.12, -2), (0.2, -6), (0.31, -30), (0.5, -42), (0.8, -18), (1.0, -8)])
STIP = dict(b0=0.0, b1=0.30, clear=0.24, stride=STR)


def run_frame(i, yoff=0.0):
    u = i / N
    t = i * T_ / N
    xp = V * t
    # left leg (intact), landing at u = 0
    Xl = SL["X0"] * STR
    (al, dl) = leg_state(G, u, SL, Xl)
    # right (blade) leg, landing at u = 0.5
    ur = (u - 0.5) % 1.0
    Xr = SL["X0"] * STR + (V * T_ * 0.5 if u >= 0.5 else -V * T_ * 0.5)
    if ur < STIP["b1"]:
        Pr = (Xr, 0.0)
        s = ur / STIP["b1"]
        defl = -11 * math.sin(math.pi * s) ** 1.2
    else:
        s = (ur - STIP["b1"]) / (1 - STIP["b1"])
        P0 = (Xr, 0.0)
        P1 = (Xr + STR, 0.0)
        Pr = (P0[0] + (P1[0] - P0[0]) * (0.5 * smooth(s) + 0.5 * s), STIP["clear"] * math.sin(math.pi * s) ** 1.2)
        defl = 0.0
    th = 0
    return xp, u, al, dl, Pr, th, defl


def solve_frame(i):
    u = i / N
    xp, u, al, dl, Pr, th, defl = run_frame(i)
    y_top = HIP - 0.006
    bob = 0.062
    nominal = y_top - bob * (1 + math.cos(4 * math.pi * (u - 0.19))) / 2
    ypel = nominal
    for _ in range(60):
        off = {}
        root = {"x": xp, "y": ypel - HIP}
        c1 = intact_leg(off, root, al, dl, max(0.0, -math.degrees(dl)) if False else max(0.0, -math.degrees(dl) - 0))
        c2 = blade_leg(off, root, Pr, defl)
        if not (c1 or c2):
            break
        ypel -= 0.004
    return off, root, xp, ypel, u


def arms_run(off, u):
    for sd, ph in (("r", 0.0), ("l", 0.5)):
        sw = 38 * math.cos(2 * math.pi * (u - ph)) - 6
        off[f"arm-{sd}"] = {"angleDeg": round(sw, 3)}
        off[f"forearm-{sd}"] = {"angleDeg": round(88 + 10 * math.cos(2 * math.pi * (u - ph)), 3)}
        off[f"hand-{sd}"] = {"angleDeg": 0.0}
    off["torso"] = {"angleDeg": round(-8 + 1.5 * math.sin(4 * math.pi * u), 3)}
    off["neck"] = {"angleDeg": 5.0}
    off["head"] = {"angleDeg": -1.0}


FR = []
for i in range(N):
    off, root, xp, ypel, u = solve_frame(i)
    FR.append([off, root, xp, ypel, u])
# swing feet follow the shank (blend ankle offset between toe-off and the next strike frame), toes relaxed
swing_l = [i for i in range(N) if FR[i][4] >= SL["b1"]]
f0 = FR[(swing_l[0] - 1) % N][0]["foot-l"]["angleDeg"]
f1 = FR[(swing_l[-1] + 1) % N][0]["foot-l"]["angleDeg"]
t0 = FR[(swing_l[0] - 1) % N][0]["toe-l"]["angleDeg"]
for j, i in enumerate(swing_l):
    s = smooth((j + 1) / (len(swing_l) + 1))
    FR[i][0]["foot-l"]["angleDeg"] = round(f0 + (f1 - f0) * s + 8 * math.sin(math.pi * s), 4)
    FR[i][0]["toe-l"]["angleDeg"] = round(t0 * (1 - s), 4)
for fr in FR:
    arms_run(fr[0], fr[4])
    fr[0] = pose_clean(fr[0])


def contacts_at(i):
    u = i / N
    c = []
    if u < SL["b1"]:
        c.append("ball-l")
    if (u - 0.5) % 1.0 < STIP["b1"]:
        c.append("blade-toe")
    return c


STAND = {
    "standing-ready": dict(ypel=HIP - 0.012, xl=0.05, xr=0.17, trunk={"torso": -3, "neck": 3, "head": 0}, arms={"l": (6, 18, 0), "r": (-4, 20, 0)}),
    "hands-on-hips": dict(ypel=HIP - 0.012, xl=0.03, xr=0.16, trunk={"torso": 2, "neck": 4, "head": 14}, arms={"l": (-30, 118, -10), "r": (-30, 118, -10)}),
}
POSES = {k: standing(v) for k, v in STAND.items()}
if __name__ == "__main__":
    for k, (off, root) in POSES.items():
        print(k, joint_issues(a, off), round(root["y"], 4), round(ground_y(a, off, ["heel-l", "ball-l", "blade-toe"]), 4))
        skeleton_png(a, off, root, f"{GEN_DIR}/sk-r-{k}.png", den=40, view=(-0.6, -0.05, 0.9, 1.95))
    bad = []
    for i, (off, root, xp, ypel, u) in enumerate(FR):
        ji = joint_issues(a, off)
        if ji:
            bad.append((i, ji))
    print("run joint problems", bad[:8])
    for i in (0, 4, 8, 11, 15, 19):
        off, root, xp, ypel, u = FR[i]
        W = Wof(a, off, root)
        print(i, round(ypel, 3), "ballL", [round(v, 3) for v in anchor_world(a, W, "ball-l")], "tip", [round(v, 3) for v in anchor_world(a, W, "blade-toe")])
    skeleton_png(a, FR[2][0], FR[2][1], f"{GEN_DIR}/sk-r-run2.png", den=40, view=(-0.6, -0.05, 1.2, 1.95))
    skeleton_png(a, FR[15][0], FR[15][1], f"{GEN_DIR}/sk-r-run15.png", den=40, view=(1.8, -0.05, 3.6, 1.95)) if False else None
