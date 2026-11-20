"""🦴 Round 3 root fixes of humans/skateboarder: the rig to the anthropometric shares (hip spacing, shoulder breadth, segment lengths, L5/S1 behind the hips, C7 and the head), the board to catalogue geometry (truck axles, deck height), the anchors on the new sole, tail, hand and face, and every pose re-solved by inverse kinematics for a real sideways stance: the rider stands across the deck (regular stance, left foot to the nose, facing the toe edge at -z), front foot angled over the front bolts, rear foot across the rear bolts or on the tail; while pushing the hips and the front foot turn to the nose and the right foot pushes on the ground beside the board.

Reads `🗑️generated/r3-skateboarder/asset.before.json`, writes `🗑️generated/r3-skateboarder/root.json` (rig, anchors, poses, structure masses) and prints the residuals and joint angles.
"""
import copy, importlib.util, json, math, os

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-skateboarder")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-skateboarder-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
FK = mod("fk")
HIP_LIFT0 = R.DECK_Y + 0.6


def bone(rig, i):
    return next(b for b in rig["bones"] if b["id"] == i)


SWAY = {"id": "hip-sway", "parent": "board", "title": {"en": "Hip sway across the board (towards the toe or heel edge)", "de": "Hüftverlagerung quer zum Brett (zur Zehen- oder Fersenkante)"}, "x": 0, "y": 0, "angleDeg": 90, "lengthM": 0.01, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0}}


GIRDLE = {"id": R.GIRDLE, "parent": "torso", "title": {"en": "Shoulder girdle (from the spine forward to the breastbone; carries the hood and the plan of the shoulders)", "de": "Schultergürtel (von der Wirbelsäule nach vorn zum Brustbein; trägt die Kapuze und den Grundriss der Schultern)"}, "x": R.GIRDLE_X, "y": 0, "angleDeg": -90, "lengthM": R.GIRDLE_LEN, "joint": {"type": "fixed"}}


GAZE = {"id": R.GAZE, "parent": "head", "title": {"en": "Line of sight (from the middle of the head forward; carries the nose and the plan of head and beanie)", "de": "Blicklinie (von der Kopfmitte nach vorn; trägt die Nase und den Grundriss von Kopf und Mütze)"}, "x": R.GAZE_X, "y": 0, "angleDeg": -90, "lengthM": R.GAZE_LEN, "joint": {"type": "fixed"}}


def fix_rig(rig):
    if not any(b["id"] == R.GIRDLE for b in rig["bones"]):
        i = next(k for k, b in enumerate(rig["bones"]) if b["id"] == "head")
        rig["bones"].insert(i + 1, copy.deepcopy(GIRDLE))
        rig["bones"].insert(i + 1, copy.deepcopy(GAZE))
    if not any(b["id"] == "hip-sway" for b in rig["bones"]):
        i = next(k for k, b in enumerate(rig["bones"]) if b["id"] == "hip-lift")
        rig["bones"].insert(i, copy.deepcopy(SWAY))
    hl = bone(rig, "hip-lift")
    hl["parent"], hl["x"], hl["y"], hl["angleDeg"] = "hip-sway", 0.6, 0, -90
    hl["joint"]["axisDeg"] = 0
    bone(rig, "wheel-f")["x"], bone(rig, "wheel-r")["x"] = R.AXLE_X, -R.AXLE_X
    for w in ("wheel-f", "wheel-r"):
        bone(rig, w)["y"] = round(R.WHEEL_R - R.DECK_Y, 4)
        bone(rig, w)["lengthM"] = R.WHEEL_R
    bd = bone(rig, "board")
    bd["mass"]["comY"] = -0.013
    pv = bone(rig, "pelvis")
    pv["lengthM"] = R.PELVIS
    pv["mass"]["comX"], pv["mass"]["comY"] = 0.047, 0.022
    to = bone(rig, "torso")
    to["x"], to["y"], to["lengthM"] = R.PELVIS, R.SACRUM_BACK, R.TORSO
    to["mass"]["comX"], to["mass"]["comY"] = round(0.4567 * R.TORSO, 4), -0.045
    hd = bone(rig, "head")
    hd["x"], hd["angleDeg"], hd["lengthM"] = R.TORSO, R.HEAD_TILT, R.HEAD
    hd["mass"]["comX"], hd["mass"]["comY"] = 0.1235, -0.035
    for s in "lr":
        side = 1 if s == "l" else -1
        ua, fa, ha = bone(rig, f"upper-arm-{s}"), bone(rig, f"forearm-{s}"), bone(rig, f"hand-{s}")
        ua["x"], ua["y"], ua["z"] = R.SHOULDER_X, -R.SHOULDER_FWD, side * R.SHOULDER_Z
        ua["lengthM"], ua["mass"]["comX"] = R.UPPER_ARM, round(0.5772 * R.UPPER_ARM, 4)
        fa["x"], fa["lengthM"], fa["mass"]["comX"] = R.UPPER_ARM, R.FOREARM, round(0.4574 * R.FOREARM, 4)
        ha["x"], ha["lengthM"], ha["mass"]["comX"] = R.FOREARM, R.HAND, 0.07
        th, sh, ft, tp = (bone(rig, f"{n}-{s}") for n in ("thigh", "shank", "foot", "toe"))
        th["z"] = side * R.HIP_Z
        th["lengthM"], th["mass"]["comX"] = R.THIGH, round(0.4095 * R.THIGH, 4)
        sh["x"], sh["lengthM"], sh["mass"]["comX"] = R.THIGH, R.SHANK, round(0.4459 * R.SHANK, 4)
        ft["x"], ft["angleDeg"], ft["lengthM"] = R.SHANK, R.FOOT_REST, R.FOOT
        ft["mass"]["comX"], ft["mass"]["comY"] = 0.052, -0.022
        ft["joint"]["minDeg"], ft["joint"]["maxDeg"] = round(R.FOOT_DROP - 50, 1), round(R.FOOT_DROP + 40, 1)
        tp["x"], tp["y"], tp["angleDeg"], tp["lengthM"] = R.FOOT, 0.0, round(R.FOOT_DROP, 3), R.TOE


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


def solve(x0, residual, iters=120, damp=1e-4):
    x = list(x0)
    for _ in range(iters):
        r = residual(x)
        if max(abs(q) for q in r) < 1e-8:
            break
        J = []
        for k in range(len(x)):
            y = list(x)
            y[k] += 1e-4
            rk = residual(y)
            J.append([(rk[i] - r[i]) / 1e-4 for i in range(len(r))])
        n = len(x)
        A = [[sum(J[a][i] * J[b][i] for i in range(len(r))) + (damp if a == b else 0) for b in range(n)] for a in range(n)]
        g = [sum(J[a][i] * r[i] for i in range(len(r))) for a in range(n)]
        dx = gauss(A, [-q for q in g])
        step = max(abs(q) for q in dx)
        if step > 8:
            dx = [q * 8 / step for q in dx]
        x = [x[k] + dx[k] for k in range(n)]
    return x, max(abs(q) for q in residual(x))


def unit(a):
    n = math.sqrt(sum(q * q for q in a))
    return [q / n for q in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def board_point(root, p):
    """🌐 A board-frame point (x along the deck, y up, z left; origin on the ground under the deck centre at rest) in the world of a pose root."""
    a = math.radians(root.get("angleDeg", 0))
    return [root.get("x", 0) + p[0] * math.cos(a) - p[1] * math.sin(a), root.get("y", 0) + p[0] * math.sin(a) + p[1] * math.cos(a), p[2]]


def board_dir(root, d):
    a = math.radians(root.get("angleDeg", 0))
    return [d[0] * math.cos(a) - d[1] * math.sin(a), d[0] * math.sin(a) + d[1] * math.cos(a), d[2]]


def deck_surface(x):
    """🛹 Deck top point and normal in the board frame at x (centre line)."""
    y = R.DECK_TOP + R.deck_y(x)
    e = 1e-4
    s = (R.deck_y(x + e) - R.deck_y(x - e)) / (2 * e)
    return y, unit([-s, 1.0, 0.0])


def foot_target(mid, heading_deg, on, root, plantar=0.0):
    """🦶 Ankle position and foot frame of a shoe whose sole middle lies at `mid` (x, z on the deck or the ground), its toes `heading_deg` from -z towards +x (0 = across the deck to the toe edge, 90 = to the nose), on 'deck' or 'ground', the sole lifted at the heel by `plantar` degrees about the ball."""
    h = math.radians(heading_deg)
    hd = [math.sin(h), 0.0, -math.cos(h)]
    if on == "deck":
        y, n = deck_surface(mid[0])
        p = [mid[0], y, mid[1]]
    else:
        n = [0.0, 1.0, 0.0]
        p = [mid[0], mid[1] if len(mid) > 2 else 0.0, mid[-1]] if len(mid) > 2 else [mid[0], 0.0, mid[1]]
    hd = unit([hd[i] - dot(hd, n) * n[i] for i in range(3)])
    side = cross(hd, n)
    half = (R.SHOE_BACK + R.BALL_FWD + R.SHOE_TOE) / 2
    ball_sole = [p[i] + hd[i] * (R.BALL_FWD + R.SHOE_BACK - half) for i in range(3)]
    a = math.radians(plantar)
    hd2 = [hd[i] * math.cos(a) - n[i] * math.sin(a) for i in range(3)]
    n2 = [n[i] * math.cos(a) + hd[i] * math.sin(a) for i in range(3)]
    ankle = [ball_sole[i] - hd2[i] * R.BALL_FWD + n2[i] * R.ANKLE for i in range(3)]
    u = unit([hd2[i] * R.BALL_FWD - n2[i] * (R.ANKLE - R.BALL_UP) for i in range(3)])
    v = unit([n2[i] - dot(n2, u) * u[i] for i in range(3)])
    return board_point(root, ankle), board_dir(root, u), board_dir(root, v), board_dir(root, side)


def leg(rig, offs, root, s, target, guess):
    ankle, u_t, v_t, side_t = target
    names = [f"thigh-{s}", f"shank-{s}", f"foot-{s}"]
    offs[f"toe-{s}"] = {"angleDeg": 0}

    def put(x):
        offs[names[0]] = {"angleDeg": x[0], "abductionDeg": x[1], "twistDeg": x[2]}
        offs[names[1]] = {"angleDeg": x[3]}
        offs[names[2]] = {"angleDeg": x[4], "abductionDeg": x[6], "twistDeg": x[5]}

    def res(x):
        put(x)
        F = FK.chain(rig, offs, root)
        f = F[names[2]]
        return [f["o"][i] - ankle[i] for i in range(3)] + [(f["u"][i] - u_t[i]) * 0.2 for i in range(3)] + [dot(f["v"], side_t) * 0.2]

    best = None
    for g in (guess, [guess[0] + 15, guess[1], guess[2] + 20] + guess[3:], [guess[0] - 10, guess[1], guess[2] - 20] + guess[3:]):
        x, err = solve(g, res, iters=200)
        if best is None or err < best[1]:
            best = (x, err)
    x, err = best
    put([round(q, 4) for q in x])
    r = res([round(q, 4) for q in x])
    if max(abs(q) for q in r) > 1e-3:
        print("   [DEBUG] leg", s, [round(q, 4) for q in r], [round(q, 1) for q in x])
    return err


def arm(rig, offs, root, s, d1, d2, hand=0.0):
    """💪 Upper arm along d1 and forearm along d2 (unit vectors in torso terms: down, out, forward), the hand straight on."""
    side = 1 if s == "l" else -1
    F = FK.chain(rig, offs, root)
    t = F["torso"]
    dirs = []
    for d in (d1, d2):
        w = [-t["u"][i] * d[0] + t["w"][i] * side * d[1] - t["v"][i] * d[2] for i in range(3)]
        dirs.append(unit(w))
    names = [f"upper-arm-{s}", f"forearm-{s}"]
    offs[f"hand-{s}"] = {"angleDeg": hand}

    def put(x):
        offs[names[0]] = {"angleDeg": x[0], "abductionDeg": x[1], "twistDeg": x[2]}
        offs[names[1]] = {"angleDeg": x[3]}

    def res(x):
        put(x)
        F2 = FK.chain(rig, offs, root)
        return [F2[names[0]]["u"][i] - dirs[0][i] for i in range(3)] + [F2[names[1]]["u"][i] - dirs[1][i] for i in range(3)]

    best = None
    for tw in (0.0, side * 60.0, -side * 60.0, side * 120.0):
        x, err = solve([10.0, side * 10.0, tw, 20.0], res)
        if 0 <= x[3] <= 150 and (best is None or err < best[1]):
            best = (x, err)
    put([round(q, 4) for q in best[0]])
    return best[1]


def pelvis(rig, offs, root, face_deg, lean_deg):
    """🧭 Pelvis turned so the rider faces `face_deg` from -z towards +x (world horizontal) and leans `lean_deg` forward, whatever the pitch of the board."""
    f = [math.sin(math.radians(face_deg)), 0.0, -math.cos(math.radians(face_deg))]
    side = [-f[2], 0.0, f[0]]
    L = math.radians(lean_deg)
    u_t = [f[i] * math.sin(L) + (1.0 if i == 1 else 0.0) * math.cos(L) for i in range(3)]

    def put(x):
        offs["pelvis"] = {"angleDeg": x[0], "abductionDeg": x[1], "twistDeg": x[2]}

    def res(x):
        put(x)
        F = FK.chain(rig, offs, root)
        p = F["pelvis"]
        return [p["u"][i] - u_t[i] for i in range(3)] + [dot(p["v"], side)]

    best = None
    for tw in (-90.0, -45.0, 0.0):
        x, err = solve([0.0, 0.0, tw], res)
        fwd = [-v for v in FK.chain(rig, offs, root)["pelvis"]["v"]]
        if dot(fwd, f) > 0 and (best is None or err < best[1]):
            best = (x, err)
    put([round(q, 4) for q in best[0]])
    return best[1]


def look(rig, offs, root, direction):
    """👀 Head turned and nodded so the face looks along a world direction, the ears level (no roll)."""
    d = unit(direction)

    def put(x):
        offs["head"] = {"angleDeg": x[0], "abductionDeg": x[1], "twistDeg": x[2]}

    def res(x):
        put(x)
        F = FK.chain(rig, offs, root)
        f = F["head"]
        a = math.radians(-R.HEAD_TILT)
        face = [-f["v"][i] * math.cos(a) + f["u"][i] * math.sin(a) for i in range(3)]
        return [face[i] - d[i] for i in range(3)] + [f["w"][1]]

    best = None
    for tw in (0.0, 60.0, -60.0):
        x, err = solve([0.0, 0.0, tw], res)
        score = err + (0 if -50 <= x[0] <= 45 and abs(x[2]) <= 90 else 10) + abs(x[1]) * 1e-4
        if best is None or score < best[2]:
            best = (x, err, score)
    put([round(q, 4) for q in best[0]])
    return best[1]


POSES = {
    "cruising": dict(hip=(-0.005, 0.995, 0.03), face=2, lean=4, torso=-6, torso_twist=0, feet={"l": ((0.19, -0.02), 30, "deck", 0), "r": ((-0.215, -0.022), 8, "deck", 0)}, arms={"l": ((1, 0.17, 0.04), (1, 0.24, 0.32)), "r": ((1, 0.16, 0.0), (1, 0.22, 0.26))}, look=(1.0, -0.28, -0.12)),
    "crouch": dict(hip=(-0.04, 0.70, 0.11), face=8, lean=24, torso=-14, torso_twist=0, feet={"l": ((0.075, -0.025), 40, "deck", 0), "r": ((-0.335, -0.025), 6, "deck", 0)}, arms={"l": ((1, 0.25, 0.45), (1, 0.15, 0.9)), "r": ((1, 0.45, -0.05), (1, 0.55, 0.3))}, look=(1.0, -0.45, -0.1)),
    "landing": dict(hip=(-0.01, 0.76, 0.10), face=10, lean=22, torso=-16, torso_twist=0, feet={"l": ((0.20, -0.02), 30, "deck", 0), "r": ((-0.215, -0.022), 8, "deck", 0)}, arms={"l": ((1, 0.7, 0.25), (1, 0.6, 0.6)), "r": ((1, 0.75, 0.0), (1, 0.65, 0.3))}, look=(1.0, -0.4, -0.1)),
    "ollie-air": dict(hip=(-0.02, 0.64, 0.13), face=8, lean=16, torso=-12, torso_twist=0, feet={"l": ((0.25, -0.02), 50, "deck", 0), "r": ((-0.20, -0.02), 6, "deck", 0)}, arms={"l": ((1, 0.9, 0.3), (1, 0.8, 0.5)), "r": ((1, 0.95, 0.0), (1, 0.85, 0.2))}, look=(1.0, -0.5, -0.1), root={"angleDeg": 6, "y": 0.40}),
    "pop": dict(hip=(0.12, 0.88, 0.05), face=8, lean=12, torso=-4, torso_twist=0, feet={"l": ((0.06, -0.02), 40, "deck", 0), "r": ((-0.355, -0.02), 6, "deck", 0)}, arms={"l": ((0.4, 0.5, 0.8), (0.1, 0.4, 1.0)), "r": ((1, 0.6, -0.2), (1, 0.7, 0.2))}, look=(1.0, -0.2, -0.1), root={"angleDeg": 30}),
    "pushing": dict(hip=(0.02, 0.80, -0.04), face=76, lean=16, torso=-16, torso_twist=0, feet={"l": ((0.175, 0.0), 82, "deck", 0), "r": ((-0.13, -0.215), 86, "ground", 28)}, arms={"l": ((1, 0.25, 0.35), (1, 0.15, 0.7)), "r": ((1, 0.25, -0.35), (1, 0.2, 0.0))}, look=(1.0, -0.2, 0.0)),
    "push-plant": dict(hip=(0.03, 0.83, -0.035), face=74, lean=12, torso=-14, torso_twist=0, feet={"l": ((0.175, 0.0), 82, "deck", 0), "r": ((0.06, -0.215), 84, "ground", 0)}, arms={"l": ((1, 0.25, -0.2), (1, 0.2, 0.15)), "r": ((1, 0.25, 0.3), (1, 0.15, 0.65))}, look=(1.0, -0.2, 0.0)),
    "push-lift": dict(hip=(0.0, 0.80, -0.04), face=76, lean=18, torso=-18, torso_twist=0, feet={"l": ((0.175, 0.0), 82, "deck", 0), "r": ((-0.33, -0.19), 86, "ground", 50)}, arms={"l": ((1, 0.25, 0.5), (1, 0.15, 0.85)), "r": ((1, 0.25, -0.5), (1, 0.2, -0.2))}, look=(1.0, -0.2, 0.0)),
    "push-swing": dict(hip=(0.0, 0.85, -0.03), face=72, lean=12, torso=-14, torso_twist=0, feet={"l": ((0.175, 0.0), 82, "deck", 0), "r": ((-0.16, 0.10, -0.17), 80, "ground", 22)}, arms={"l": ((1, 0.25, 0.1), (1, 0.2, 0.4)), "r": ((1, 0.25, 0.0), (1, 0.2, 0.35))}, look=(1.0, -0.2, 0.0)),
    "push-reach": dict(hip=(-0.01, 0.88, -0.01), face=56, lean=8, torso=-10, torso_twist=0, feet={"l": ((0.175, 0.0), 78, "deck", 0), "r": ((-0.14, 0.06, -0.15), 70, "ground", 12)}, arms={"l": ((1, 0.3, 0.1), (1, 0.3, 0.35)), "r": ((1, 0.3, 0.0), (1, 0.3, 0.25))}, look=(1.0, -0.22, -0.05)),
}

LEG_GUESS = {"deck": [30.0, 0.0, 0.0, -50.0, 20.0, 0.0, 0.0], "ground": [-10.0, 0.0, 0.0, -30.0, 10.0, 0.0, 0.0]}


def ground_root(rig, offs, root, anchors, contacts):
    """⬇️ The root height that puts the lowest of the named contact anchors on y = 0 (for authored-ground poses)."""
    F = FK.chain(rig, offs, root)
    ys = [FK.pt(F[a["bone"]], a["x"], a["y"], a.get("z", 0))[1] for a in anchors if a["id"] in contacts]
    return root.get("y", 0) - min(ys)


def solve_pose(rig, p, spec, anchors):
    offs = {}
    root = dict(spec.get("root", {}))
    if root:
        root.setdefault("x", 0.0)
        root.setdefault("y", 0.0)
    for it in range(3 if root else 1):
        a = root.get("angleDeg", 0.0)
        hx, hy, hz = spec["hip"]
        L = math.hypot(hy - R.DECK_Y, hz)
        offs["hip-sway"] = {"abductionDeg": round(math.degrees(math.atan2(hz, hy - R.DECK_Y)), 4)}
        offs["hip-lift"] = {"dx": round(L - 0.6, 4)}
        offs["hip-shift"] = {"dx": round(hx, 4)}
        errs = [pelvis(rig, offs, root, spec["face"], spec["lean"])]
        offs["torso"] = {"angleDeg": spec["torso"], "twistDeg": spec["torso_twist"]}
        for s in "lr":
            mid, head, on, pl = spec["feet"][s]
            tgt = foot_target(mid, head, on, root if on == "deck" else {}, pl)
            errs.append(leg(rig, offs, root, s, tgt, LEG_GUESS[on]))
            offs[f"toe-{s}"] = {"angleDeg": float(pl)}
        for s in "lr":
            d1, d2 = spec["arms"][s]
            errs.append(arm(rig, offs, root, s, d1, d2, -6.0))
        errs.append(look(rig, offs, root, spec["look"]))
        if root:
            if p["id"] == "pop":
                root["y"] = round(ground_root(rig, offs, root, anchors, ["tail"]), 4)
    p["bones"] = offs
    if root:
        p["root"] = {k: v for k, v in root.items() if k != "x" or v != 0.0}
    return max(errs), offs, root


def report(rig, p, root):
    F = FK.chain(rig, p["bones"], root)
    lim = {b["id"]: b["joint"] for b in rig["bones"]}
    out = []
    for b, o in p["bones"].items():
        j = lim[b]
        if j["type"] == "revolute":
            a = o.get("angleDeg", 0)
            if (j.get("minDeg") is not None and a < j["minDeg"] - 1e-6) or (j.get("maxDeg") is not None and a > j["maxDeg"] + 1e-6):
                out.append(f"LIMIT {b} {a:.1f} not in [{j.get('minDeg')}, {j.get('maxDeg')}]")
    hip = F["pelvis"]["o"]
    for s in "lr":
        sh = F[f"shank-{s}"]["u"]
        fu = F[f"foot-{s}"]
        d = math.radians(R.FOOT_DROP)
        sole = [fu["u"][i] * math.cos(d) + fu["v"][i] * math.sin(d) for i in range(3)]
        knee = math.degrees(math.acos(max(-1, min(1, dot(F[f"thigh-{s}"]["u"], sh)))))
        dors = 90 - math.degrees(math.acos(max(-1, min(1, dot([-q for q in sh], sole)))))
        out.append(f"{s}: knee flexion {knee:.0f}, ankle dorsiflexion {dors:.0f}")
    return out, hip


def main():
    doc = json.load(open(os.path.join(OUT, "asset.before.json")))
    a = doc["asset"]
    rig = copy.deepcopy(a["rig"])
    fix_rig(rig)
    anchors = fix_anchors(copy.deepcopy(a["anchors"]))
    worst = 0.0
    for p in a["poses"]:
        err, offs, root = solve_pose(rig, p, POSES[p["id"]], anchors)
        worst = max(worst, err)
        bad, hip = report(rig, p, root)
        print(f"pose {p['id']:12s} residual {err:.1e} root {p.get('root')} hip ({hip[0]:+.3f},{hip[1]:+.3f},{hip[2]:+.3f})")
        for b in ("thigh-l", "shank-l", "foot-l", "thigh-r", "shank-r", "foot-r", "upper-arm-l", "forearm-l", "upper-arm-r", "forearm-r", "head"):
            print(f"    {b:12s} {offs[b]}")
        for x in bad:
            print("   ", x)
    print(f"worst residual {worst:.1e}")
    json.dump({"rig": rig, "anchors": anchors, "poses": a["poses"]}, open(os.path.join(OUT, "root.json"), "w"), ensure_ascii=False)


HEAD_ANCHORS = {"mouth": (0.071, 0.153), "nostrils": (0.091, 0.160), "crown": (R.HEAD, 0.0)}


def foot_local(x, y):
    d = math.radians(R.FOOT_DROP)
    return (x * math.cos(d) - y * math.sin(d), x * math.sin(d) + y * math.cos(d))


def fix_anchors(anchors):
    for an in anchors:
        i = an["id"]
        if i.endswith("-contact") and i.startswith("wheel"):
            an["x"], an["y"] = 0.0, -R.WHEEL_R
        elif i == "tail":
            x = R.TAIL_TIP + 0.006
            an["x"], an["y"] = round(x, 4), round(R.deck_y(x) - R.DECK_T / 2, 4)
        elif i.startswith("contact-"):
            b = foot_local(R.BALL_FWD, -R.ANKLE)
            an["x"], an["y"] = round(b[0], 4), round(b[1], 4)
        elif i.startswith("heel-"):
            b = foot_local(-0.030, -R.ANKLE)
            an["x"], an["y"] = round(b[0], 4), round(b[1], 4)
        elif i.startswith("grip-"):
            an["x"], an["y"] = 0.085, 0.0
        elif i == "board-load":
            an["y"] = round(R.DECK_T / 2, 4)
        elif i in HEAD_ANCHORS:
            if i == "crown":
                an["x"], an["y"] = R.HEAD, 0.0
            else:
                yu, zf = HEAD_ANCHORS[i]
                a = math.radians(R.HEAD_TILT)
                u = (-math.sin(a), math.cos(a))
                n = (math.cos(a), math.sin(a))
                x = zf * u[0] + yu * u[1]
                y = -(zf * n[0] + yu * n[1])
                an["x"], an["y"] = round(x, 4), round(y, 4)
    return anchors


if __name__ == "__main__":
    main()
