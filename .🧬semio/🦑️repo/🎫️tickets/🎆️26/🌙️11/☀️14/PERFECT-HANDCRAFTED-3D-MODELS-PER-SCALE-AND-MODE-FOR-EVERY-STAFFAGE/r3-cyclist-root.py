"""🦴 Round 3 root fixes of humans/cyclist: rig segment lengths to the anthropometric shares, hip spacing, foot and head joints, anchors on the new shoe, hand and head, and every pose and clip key re-solved by inverse kinematics (feet on the real pedals 132 mm off the midplane, hands round the swept-back grips 250 mm off the midplane, torso 4 degrees more upright while riding).

Reads `🗑️generated/r3-cyclist/asset.before.json`, writes `🗑️generated/r3-cyclist/root.json` (rig, anchors, poses, clips, dimensions) and prints the residuals.
"""
import copy, importlib.util, json, math, os

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-cyclist")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-cyclist-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
FK = mod("fk")
RIDING = {"riding-upright", "coasting", "standing-on-pedals"}
UPRIGHT = {"riding-upright", "coasting"}
TORSO_DELTA = 4.0
FRAME_COM = (0.51, 0.59)
HEAD_ANCHORS = {"mouth": (-0.024, -0.113), "nose": (0.0, -0.117), "crown": (0.181, 0.037)}


def bone(rig, i):
    return next(b for b in rig["bones"] if b["id"] == i)


def fix_rig(rig):
    for s in "lr":
        th, sh, ft, to = (bone(rig, f"{n}-{s}") for n in ("thigh", "shank", "foot", "toe"))
        th["mass"]["comX"] = round(th["mass"]["comX"] * R.THIGH / th["lengthM"], 4)
        sh["mass"]["comX"] = round(sh["mass"]["comX"] * R.SHANK / sh["lengthM"], 4)
        ft["mass"]["comX"] = round(0.43 * R.FOOT, 4)
        ft["mass"]["comY"] = -0.012
        th["lengthM"], sh["x"], sh["lengthM"] = R.THIGH, R.THIGH, R.SHANK
        ft["x"], ft["lengthM"], ft["angleDeg"] = R.SHANK, R.FOOT, R.FOOT_REST
        to["x"], to["angleDeg"], to["lengthM"] = R.FOOT, R.TOE_REST, 0.066
        to["mass"]["comX"], to["mass"]["comY"] = 0.03, -0.012
        th["z"] = R.HIP_Z if s == "l" else -R.HIP_Z
    fr = bone(rig, "frame")
    fr["mass"]["comX"], fr["mass"]["comY"] = FRAME_COM
    nk, hd = bone(rig, "neck"), bone(rig, "head")
    nk["mass"]["comX"] = round(R.NECK / 2, 4)
    nk["lengthM"], hd["x"], hd["lengthM"] = R.NECK, R.NECK, R.HEAD
    hd["mass"]["comX"], hd["mass"]["comY"] = 0.052, -0.012


def solve(F0, unknowns, residual, iters=60):
    x = list(unknowns)
    for _ in range(iters):
        r = residual(x)
        if max(abs(q) for q in r) < 1e-7:
            break
        J = []
        for k in range(len(x)):
            y = list(x)
            y[k] += 1e-4
            rk = residual(y)
            J.append([(rk[i] - r[i]) / 1e-4 for i in range(len(r))])
        n = len(x)
        A = [[sum(J[a][i] * J[b][i] for i in range(len(r))) + (1e-6 if a == b else 0) for b in range(n)] for a in range(n)]
        g = [sum(J[a][i] * r[i] for i in range(len(r))) for a in range(n)]
        dx = gauss(A, [-q for q in g])
        step = max(abs(q) for q in dx)
        if step > 10:
            dx = [q * 10 / step for q in dx]
        x = [x[k] + dx[k] for k in range(n)]
    return x, max(abs(q) for q in residual(x))


def gauss(A, b):
    n = len(b)
    M = [A[i][:] + [b[i]] for i in range(n)]
    for c in range(n):
        p = max(range(c, n), key=lambda i: abs(M[i][c]))
        M[c], M[p] = M[p], M[c]
        for i in range(n):
            if i != c and M[c][c] != 0:
                f = M[i][c] / M[c][c]
                M[i] = [M[i][k] - f * M[c][k] for k in range(n + 1)]
    return [M[i][n] / M[i][i] for i in range(n)]


def world_angle(f):
    return math.degrees(math.atan2(f["u"][1], f["u"][0]))


def leg(rig, offs, root, s, target, sole_deg, fixed_foot=None):
    side = 1 if s == "l" else -1
    names = [f"thigh-{s}", f"shank-{s}", f"foot-{s}"]
    for n in names:
        offs.setdefault(n, {"angleDeg": 0})
    offs[f"toe-{s}"] = {"angleDeg": 0}

    def put(x):
        offs[names[0]]["angleDeg"] = x[0]
        offs[names[0]]["abductionDeg"] = x[1]
        offs[names[0]]["twistDeg"] = x[2]
        offs[names[1]]["angleDeg"] = x[3]
        offs[names[2]]["angleDeg"] = x[4] if fixed_foot is None else fixed_foot

    def res(x):
        put(x)
        F = FK.chain(rig, offs, root)
        b = FK.pt(F[f"foot-{s}"], R.FOOT)
        toe = F[f"toe-{s}"]
        hip, knee, ankle = F[names[0]]["o"], F[names[1]]["o"], F[names[2]]["o"]
        r = [b[0] - target[0], b[1] - target[1], b[2] - target[2], knee[2] - (hip[2] + (ankle[2] - hip[2]) * R.THIGH / (R.THIGH + R.SHANK))]
        return r if fixed_foot is not None else r + [(world_angle(toe) - sole_deg) * 0.01]

    x0 = [offs[names[0]].get("angleDeg", 0), offs[names[0]].get("abductionDeg", 0) or side * 0.0001, offs[names[0]].get("twistDeg", 0), offs[names[1]].get("angleDeg", 0)]
    if fixed_foot is None:
        x0.append(offs[names[2]].get("angleDeg", 0))
    x, err = solve(None, x0, res)
    put([round(q, 4) for q in x] + ([] if fixed_foot is None else [0]))
    for k in ("abductionDeg", "twistDeg"):
        if abs(offs[names[0]][k]) < 1e-3:
            offs[names[0]].pop(k)
    return err


FOOT_LO, FOOT_HI = -55.0, 40.0


def pedal_leg(rig, offs, root, s):
    """🚲 Foot on a free pedal: the sole lies on the pedal; where the ankle would leave its range the foot stops at the limit and the pedal turns to the sole."""
    side = 1 if s == "l" else -1
    o, a = pedal_frame(rig, offs, root, s)
    err = leg(rig, offs, root, s, R.pedal_target(o, a, side), a)
    f = offs[f"foot-{s}"]["angleDeg"]
    if FOOT_LO <= f <= FOOT_HI:
        return err
    lim = min(max(f, FOOT_LO), FOOT_HI)
    for _ in range(6):
        err = leg(rig, offs, root, s, R.pedal_target(o, a, side), a, lim)
        F = FK.chain(rig, offs, root)
        sole = world_angle(F[f"toe-{s}"])
        offs[f"pedal-{s}"]["angleDeg"] = round(offs[f"pedal-{s}"]["angleDeg"] + sole - a, 4)
        o, a = pedal_frame(rig, offs, root, s)
    offs[f"foot-{s}"]["angleDeg"] = lim
    return err


def arm(rig, offs, root, s):
    side = 1 if s == "l" else -1
    names = [f"arm-{s}", f"forearm-{s}"]
    for n in names:
        offs.setdefault(n, {"angleDeg": 0})
    offs[f"hand-{s}"] = {"angleDeg": -8.0}
    g = R.grip_centre(side)

    def put(x):
        offs[names[0]]["angleDeg"] = x[0]
        offs[names[0]]["abductionDeg"] = x[1]
        offs[names[1]]["angleDeg"] = x[2]

    def res(x):
        put(x)
        F = FK.chain(rig, offs, root)
        p = FK.pt(F[f"hand-{s}"], R.GRIP_LOCAL[0], R.GRIP_LOCAL[1])
        return [p[0] - g[0], p[1] - g[1], p[2] - g[2]]

    x0 = [offs[names[0]].get("angleDeg", 0), side * 4.0, offs[names[1]].get("angleDeg", 60)]
    x, err = solve(None, x0, res)
    put([round(q, 4) for q in x])
    return err


def pedal_frame(rig, offs, root, s):
    F = FK.chain(rig, offs, root)
    p = F[f"pedal-{s}"]
    return p["o"], world_angle(p)


def fix_bones(rig, offs, root, riding, upright, ground_feet=None):
    if upright:
        offs["torso"] = {"angleDeg": round(offs.get("torso", {}).get("angleDeg", 0) + TORSO_DELTA, 4)}
    errs = []
    for s in "lr":
        side = 1 if s == "l" else -1
        if riding:
            errs.append(pedal_leg(rig, offs, root, s))
        elif ground_feet is not None:
            x, z = ground_feet[s]
            errs.append(leg(rig, offs, root, s, (x, R.BALL_SHOE, z), 0.0))
        errs.append(arm(rig, offs, root, s))
    return max(errs)


def main():
    doc = json.load(open(os.path.join(OUT, "asset.before.json")))
    a = doc["asset"]
    old_rig = copy.deepcopy(a["rig"])
    rig = a["rig"]
    fix_rig(rig)
    poses = {p["id"]: p for p in a["poses"]}
    worst = 0
    for p in a["poses"]:
        ground = None
        if p["id"] not in RIDING:
            F = FK.chain(old_rig, p["bones"], p.get("root", {}))
            ground = {s: (FK.pt(F[f"foot-{s}"], 0.1795)[0], (1 if s == "l" else -1) * 0.20) for s in "lr"}
        e = fix_bones(rig, p["bones"], p.get("root", {}), p["id"] in RIDING, p["id"] in UPRIGHT, ground)
        worst = max(worst, e)
        print(f"pose {p['id']:22s} residual {e:.2e}  arm-l {p['bones']['arm-l']}  forearm-l {p['bones']['forearm-l']}  thigh-l {p['bones']['thigh-l']} thigh-r {p['bones']['thigh-r']} feet {p['bones']['foot-l']} {p['bones']['foot-r']}")
    for c in a["clips"]:
        n = 0
        for k in c["keys"]:
            if "bones" not in k:
                continue
            pose = poses[k["pose"]]
            e = fix_bones(rig, k["bones"], pose.get("root", {}), k["pose"] in RIDING, k["pose"] in UPRIGHT)
            worst = max(worst, e)
            n += 1
        print(f"clip {c['id']:22s} keys solved {n}")
    print(f"worst residual {worst:.2e}")
    anchors = a["anchors"]
    heel = rot((R.HEEL_X + 0.012, -R.ANKLE_SHOE), R.FOOT_DROP)
    ball = rot((R.ANKLE_TO_BALL_X, -R.ANKLE_SHOE), R.FOOT_DROP)
    for an in anchors:
        if an["id"].startswith("heel-"):
            an["x"], an["y"] = round(heel[0], 4), round(heel[1], 4)
        if an["id"].startswith("ball-"):
            an["x"], an["y"] = round(ball[0], 4), round(ball[1], 4)
        if an["id"].startswith("grip-"):
            an["x"], an["y"] = R.GRIP_LOCAL
        if an["id"] == "saddle":
            an["x"], an["y"] = R.SADDLE_X, R.SADDLE_TOP
        if an["id"] in HEAD_ANCHORS:
            an["x"], an["y"] = HEAD_ANCHORS[an["id"]]
    json.dump({"rig": rig, "anchors": anchors, "poses": a["poses"], "clips": a["clips"]}, open(os.path.join(OUT, "root.json"), "w"), ensure_ascii=False)


def rot(p, deg):
    r = math.radians(deg)
    return (p[0] * math.cos(r) - p[1] * math.sin(r), p[0] * math.sin(r) + p[1] * math.cos(r))


if __name__ == "__main__":
    main()
