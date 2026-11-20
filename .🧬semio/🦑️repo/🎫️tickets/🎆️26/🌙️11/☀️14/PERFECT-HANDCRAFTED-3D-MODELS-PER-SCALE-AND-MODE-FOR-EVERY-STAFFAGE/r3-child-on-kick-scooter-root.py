"""🦿 Round 3 roots of humans/child-on-kick-scooter: the corrected rig (child proportions, real scooter), anchors and every pose and clip key re-solved by inverse kinematics on that rig.

Reads the catalog asset (never writes it) and writes `🗑️generated/r3-child-on-kick-scooter/root.json` = { rig, anchors, poses, clips, report }.
Usage: python3 r3-child-on-kick-scooter-root.py [--old <asset json of the committed rig>]
"""
import copy, importlib.util, json, math, os, sys

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-child-on-kick-scooter")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-child-on-kick-scooter-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
FK = mod("fk")

FRONT_BALL = (R.FRAME_X + R.DECK_X1 - 0.012 - R.TOE_L, R.DECK_TOP, 0.020)
REAR_DECK_BALL = (R.FRAME_X + 0.118, R.DECK_TOP, -0.020)
PUSH_Z = -0.125
REAR_HEEL_UP = 38.0
GRIP = (R.FRAME_X + R.BAR_X, R.BAR_Y)
OLD_DECK = 0.110
OLD_FOOT_REST_DIR = -30.538
OLD_DECK_BALL_X = 0.025


def norm(a):
    while a > 180:
        a -= 360
    while a <= -180:
        a += 360
    return a


def new_rig(rig):
    rig = copy.deepcopy(rig)
    b = {x["id"]: x for x in rig["bones"]}
    b["pelvis"]["y"] = R.HIP_Y
    b["pelvis"]["lengthM"] = R.PELVIS_L
    b["torso"]["lengthM"] = R.TORSO_L
    b["torso"]["mass"]["comX"] = round(0.45 * R.TORSO_L, 4)
    b["neck"]["x"] = R.TORSO_L
    b["neck"]["lengthM"] = R.NECK_L
    b["head"]["x"] = R.NECK_L
    b["head"]["lengthM"] = R.HEAD_L
    b["head"]["mass"]["comX"] = 0.05
    for s, z in (("l", 1), ("r", -1)):
        b[f"arm-{s}"]["x"] = R.SHOULDER_U
        b[f"arm-{s}"]["z"] = z * R.SHOULDER_Z
        b[f"thigh-{s}"]["z"] = z * R.HIP_Z
        b[f"thigh-{s}"]["lengthM"] = R.THIGH_L
        b[f"thigh-{s}"]["mass"]["comX"] = round(0.4095 * R.THIGH_L, 4)
        b[f"shank-{s}"]["x"] = R.THIGH_L
        b[f"shank-{s}"]["lengthM"] = R.SHANK_L
        b[f"shank-{s}"]["mass"]["comX"] = round(0.4459 * R.SHANK_L, 4)
        b[f"foot-{s}"]["x"] = R.SHANK_L
        b[f"foot-{s}"]["angleDeg"] = R.FOOT_REST
        b[f"foot-{s}"]["lengthM"] = R.FOOT_L
        b[f"foot-{s}"]["mass"]["comX"] = 0.049
        b[f"toe-{s}"]["x"] = R.FOOT_L
        b[f"toe-{s}"]["angleDeg"] = R.TOE_REST
        b[f"toe-{s}"]["lengthM"] = R.TOE_L
    b["frame"]["x"] = R.FRAME_X
    b["frame"]["y"] = -R.HIP_Y
    b["frame"]["lengthM"] = R.WB
    b["frame"]["mass"]["comX"] = 0.33
    b["frame"]["mass"]["comY"] = 0.22
    b["wheel-front"]["x"] = R.WB
    return rig


def new_anchors(anchors):
    out = copy.deepcopy(anchors)
    a = {x["id"]: x for x in out}
    hx, hy = R.foot_local(R.HEEL_PAD_X, -R.ANKLE_Y)
    for s in "lr":
        a[f"heel-{s}"].update(x=round(hx, 4), y=round(hy, 4), footprintLengthM=0.05, footprintWidthM=0.05)
        a[f"ball-{s}"].update(x=R.FOOT_L, y=0, footprintLengthM=0.07, footprintWidthM=0.075)
        a[f"grip-{s}"].update(x=R.GRIP_LOCAL[0], y=R.GRIP_LOCAL[1])
    a["nose"].update(x=-0.016, y=-0.1)
    a["mouth"].update(x=-0.036, y=-0.092)
    a["crown"].update(x=0.1466, y=0.0177)
    a["wheel-front-contact"]["x"] = R.WB
    for w in ("wheel-rear-contact", "wheel-front-contact"):
        a[w].update(footprintLengthM=0.012, footprintWidthM=0.02)
    a["deck"].update(x=0.25, y=R.DECK_TOP)
    a["fender"].update(x=R.FENDER_TOP[0], y=R.FENDER_TOP[1])
    a["backpack"].update(x=0.18, y=0.135)
    return out


def leg_ik(rig, offsets, root, side, ankle, pitch, guess):
    names = [f"thigh-{side}", f"shank-{side}", f"foot-{side}"]
    x = list(guess)

    def res(v):
        o = dict(offsets)
        o[names[0]] = {"angleDeg": v[0], "abductionDeg": v[1]}
        o[names[1]] = {"angleDeg": v[2], "abductionDeg": v[3]}
        o[names[2]] = {"angleDeg": v[4], "abductionDeg": v[5]}
        f = FK.chain(rig, o, root)[names[2]]
        p = math.radians(pitch)
        return [(f["o"][0] - ankle[0]) * 100, (f["o"][1] - ankle[1]) * 100, (f["o"][2] - ankle[2]) * 100, f["u"][1] - math.sin(p), f["u"][2], f["w"][1]]

    for it in range(60):
        r = res(x)
        e = max(abs(q) for q in r)
        if e < 1e-9:
            break
        J = []
        for k in range(6):
            y = list(x)
            y[k] += 1e-4
            rk = res(y)
            J.append([(rk[i] - r[i]) / 1e-4 for i in range(6)])
        A = [[J[c][rw] for c in range(6)] + [-r[rw]] for rw in range(6)]
        for c in range(6):
            piv = max(range(c, 6), key=lambda q: abs(A[q][c]))
            A[c], A[piv] = A[piv], A[c]
            if abs(A[c][c]) < 1e-12:
                raise SystemExit(f"singular leg ik {side}")
            for q in range(6):
                if q != c:
                    m = A[q][c] / A[c][c]
                    A[q] = [A[q][i] - m * A[c][i] for i in range(7)]
        d = [A[c][6] / A[c][c] for c in range(6)]
        step = min(1.0, 25.0 / max(1e-9, max(abs(q) for q in d)))
        x = [x[i] + step * d[i] for i in range(6)]
    if e > 1e-6:
        raise SystemExit(f"leg ik {side} did not converge: {e}")
    return x


def planar_leg_guess(hip, ankle, pitch):
    dx, dy = ankle[0] - hip[0], ankle[1] - hip[1]
    d = min(math.hypot(dx, dy), R.THIGH_L + R.SHANK_L - 1e-6)
    a = math.atan2(dy, dx)
    k = math.acos((R.THIGH_L ** 2 + d * d - R.SHANK_L ** 2) / (2 * R.THIGH_L * d))
    th = math.degrees(a + k)
    kx, ky = hip[0] + R.THIGH_L * math.cos(a + k), hip[1] + R.THIGH_L * math.sin(a + k)
    sh = math.degrees(math.atan2(ankle[1] - ky, ankle[0] - kx))
    return [norm(th + 90), 0, norm(sh - th), 0, norm(pitch - sh - R.FOOT_REST), 0]


def arm_ik(shoulder, wrist):
    dx, dy = wrist[0] - shoulder[0], wrist[1] - shoulder[1]
    d = math.hypot(dx, dy)
    if d > R.ARM_L + R.FOREARM_L - 1e-6:
        raise SystemExit(f"arm cannot reach: {d}")
    a = math.atan2(dy, dx)
    k = math.acos((R.ARM_L ** 2 + d * d - R.FOREARM_L ** 2) / (2 * R.ARM_L * d))
    up = math.degrees(a - k)
    ex, ey = shoulder[0] + R.ARM_L * math.cos(a - k), shoulder[1] + R.ARM_L * math.sin(a - k)
    fo = math.degrees(math.atan2(wrist[1] - ey, wrist[0] - ex))
    return up, fo, (ex, ey)


def smooth(t):
    t = max(0.0, min(1.0, t))
    return t * t * (3 - 2 * t)


def solve(rig, spec):
    """🧮 One pose from its targets: pelvis height, trunk angles, right-foot ball target with heel lift and toe direction, wheel angles."""
    root = {"x": 0, "y": round(spec["py"] - R.HIP_Y, 4)}
    off = {"pelvis": {"angleDeg": 0}, "torso": {"angleDeg": spec["torso"]}, "neck": {"angleDeg": spec["neck"]}, "head": {"angleDeg": spec["head"]}}
    F = FK.chain(rig, off, root)
    tw = math.degrees(math.atan2(F["torso"]["u"][1], F["torso"]["u"][0]))
    hd = math.radians(R.HAND_DIR)
    gx, gy = R.GRIP_LOCAL
    wrist = (GRIP[0] - (gx * math.cos(hd) - gy * math.sin(hd)), GRIP[1] - (gx * math.sin(hd) + gy * math.cos(hd)))
    sh = F["arm-l"]["o"]
    up, fo, elbow = arm_ik((sh[0], sh[1]), wrist)
    arm = {"angleDeg": round(norm(up - (tw - 180)), 4)}
    fore = {"angleDeg": round(norm(fo - up), 4)}
    hand = {"angleDeg": round(norm(R.HAND_DIR - fo), 4)}
    legs = {}
    for side, ball, lift, toe in (("l", FRONT_BALL, 0.0, 0.0), ("r", spec["ball"], spec["lift"], spec["toe"])):
        pitch = R.FOOT_DIR - lift
        p = math.radians(pitch)
        ankle = (ball[0] - R.FOOT_L * math.cos(p), ball[1] - R.FOOT_L * math.sin(p), ball[2])
        hip = F[f"thigh-{side}"]["o"]
        v = leg_ik(rig, off, root, side, ankle, pitch, planar_leg_guess(hip, ankle, pitch))
        legs[side] = ([round(q, 4) for q in v], round(max(-5.0, min(65.0, toe + lift)), 4))
    bones = {}
    bones["pelvis"] = {"angleDeg": 0}
    bones["torso"] = {"angleDeg": spec["torso"]}
    bones["neck"] = {"angleDeg": spec["neck"]}
    bones["head"] = {"angleDeg": spec["head"]}
    for s in "lr":
        if s == "r":
            bones["arm-r"], bones["forearm-r"], bones["hand-r"] = dict(arm), dict(fore), dict(hand)
        else:
            bones["arm-l"], bones["forearm-l"], bones["hand-l"] = dict(arm), dict(fore), dict(hand)
        v, toe = legs[s]
        bones[f"thigh-{s}"] = {"angleDeg": v[0], "abductionDeg": v[1]}
        bones[f"shank-{s}"] = {"angleDeg": v[2], "abductionDeg": v[3]}
        bones[f"foot-{s}"] = {"angleDeg": v[4], "abductionDeg": v[5]}
        bones[f"toe-{s}"] = {"angleDeg": toe}
    ordered = {}
    for k in ["pelvis", "torso", "neck", "head", "arm-l", "forearm-l", "hand-l", "thigh-l", "shank-l", "foot-l", "toe-l", "arm-r", "forearm-r", "hand-r", "thigh-r", "shank-r", "foot-r", "toe-r"]:
        ordered[k] = bones[k]
    ordered["aid-lift"] = {"dx": round(-root["y"], 4), "dy": 0}
    ordered["aid-slide"] = {"dx": 0, "dy": 0}
    ordered["wheel-rear"] = {"angleDeg": spec["wheel"]}
    ordered["wheel-front"] = {"angleDeg": spec["wheel"]}
    return root, ordered


def limits(rig, bones, name, notes):
    by = {b["id"]: b for b in rig["bones"]}
    for k, o in bones.items():
        j = by[k]["joint"]
        if j["type"] == "revolute" and "angleDeg" in o:
            if (j.get("minDeg") is not None and o["angleDeg"] < j["minDeg"] - 1e-9) or (j.get("maxDeg") is not None and o["angleDeg"] > j["maxDeg"] + 1e-9):
                notes.append(f"LIMIT {name} {k} {o['angleDeg']} outside {j.get('minDeg')}..{j.get('maxDeg')}")


def clearance(rig, root, bones):
    F = FK.chain(rig, bones, root)
    worst = 9.0
    for s in "lr":
        k = F[f"shank-{s}"]["o"]
        fx = F["frame"]["o"][0]
        t = (k[1] - R.HT_BOTTOM[1]) / R.AXIS[1]
        sx = fx + R.HT_BOTTOM[0] + R.AXIS[0] * t
        d = math.hypot((sx - k[0]) * R.AXIS[1], k[2])
        if abs(k[2]) < 0.06 or True:
            worst = min(worst, (sx - k[0]) * R.AXIS[1] if abs(k[2]) < 0.062 else d)
    return worst - 0.046 - 0.017


def main():
    asset = json.load(open(FK.ASSET))["asset"]
    if "--old" in sys.argv:
        old = json.load(open(sys.argv[sys.argv.index("--old") + 1]))["asset"]
    else:
        old = asset
    backup = os.path.join(OUT, "asset.before.json")
    if os.path.exists(backup):
        old = json.load(open(backup))["asset"]
    rig = new_rig(old["rig"])
    notes = []
    poses = []
    for p in old["poses"]:
        Fo = FK.chain(old["rig"], p["bones"], p.get("root", {}))
        py_old = Fo["pelvis"]["o"][1]
        fr, tr = Fo["foot-r"], Fo["toe-r"]
        ball_old = FK.pt(fr, old["rig"]["bones"][[b["id"] for b in old["rig"]["bones"]].index("foot-r")]["lengthM"])
        dir_old = math.degrees(math.atan2(fr["u"][1], fr["u"][0]))
        toe_old = math.degrees(math.atan2(tr["u"][1], tr["u"][0]))
        lift_old = OLD_FOOT_REST_DIR - dir_old
        pid = p["id"]
        if pid.startswith("kick-"):
            i = int(pid[5:])
            s = 0.0 if i <= 7 else smooth((i - 7) / 9) if i <= 16 else 1.0 if i <= 37 else 1 - smooth((i - 37) / 7)
        elif pid == "gliding":
            s = 1.0
        else:
            s = 0.0
        torso = round(-12 + 0.75 * (p["bones"]["torso"]["angleDeg"] + 18), 4)
        neck = round(0.75 * p["bones"]["neck"]["angleDeg"], 4)
        spec = {"py": round(min(0.725, 0.70 + 1.6 * (py_old - 0.640)), 4), "torso": torso, "neck": neck, "head": round(-(torso + neck) - 2, 4), "wheel": p["bones"]["wheel-rear"].get("angleDeg", 0)}
        if pid == "braking":
            spec["ball"] = (R.FRAME_X + R.FENDER_TOP[0] + 0.01, R.FENDER_TOP[1] - 0.004, -0.012)
            spec["lift"] = 12.0
            spec["toe"] = -12.0
        else:
            x = ball_old[0] + s * (REAR_DECK_BALL[0] - OLD_DECK_BALL_X)
            y = max(0.0, ball_old[1] - s * (OLD_DECK - R.DECK_TOP))
            z = PUSH_Z + (REAR_DECK_BALL[2] - PUSH_Z) * s * s
            spec["ball"] = (x, y, z)
            spec["lift"] = lift_old + REAR_HEEL_UP * s
            spec["toe"] = toe_old
        root, bones = solve(rig, spec)
        limits(rig, bones, pid, notes)
        c = clearance(rig, root, bones)
        if c < 0.01:
            notes.append(f"CLEARANCE {pid} knee to stem {c:.3f}")
        q = copy.deepcopy(p)
        q["root"] = root
        q["bones"] = bones
        poses.append(q)
    by = {q["id"]: q for q in poses}
    clips = copy.deepcopy(old["clips"])
    for c in clips:
        for k in c["keys"]:
            if "bones" in k:
                wheel = k["bones"].get("wheel-rear", {}).get("angleDeg")
                k["bones"] = copy.deepcopy(by[k["pose"]]["bones"])
                if wheel is not None:
                    k["bones"]["wheel-rear"] = {"angleDeg": wheel}
                    k["bones"]["wheel-front"] = {"angleDeg": wheel}
    os.makedirs(OUT, exist_ok=True)
    json.dump({"rig": rig, "anchors": new_anchors(old["anchors"]), "poses": poses, "clips": clips, "notes": notes}, open(os.path.join(OUT, "root.json"), "w"), ensure_ascii=False, indent=1)
    for n in notes:
        print(n)
    for pid in ("gliding", "pushing-off", "braking", "kick-00", "kick-07", "kick-12", "kick-40"):
        q = by[pid]
        F = FK.chain(rig, q["bones"], q["root"])
        print(pid, "root", q["root"], "clear", round(clearance(rig, q["root"], q["bones"]), 3))
        for b in ("head", "arm-l", "forearm-l", "hand-l", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r"):
            f = F[b]
            tip = FK.pt(f, f["len"])
            print(f"   {b:10s} o=({f['o'][0]:+.3f},{f['o'][1]:+.3f},{f['o'][2]:+.3f}) tip=({tip[0]:+.3f},{tip[1]:+.3f},{tip[2]:+.3f}) {q['bones'][b]}")


if __name__ == "__main__":
    main()
