"""🧬 Round 3 volumes of humans/skateboarder: per authored scale the body (from `-body50` / `-coarse`), the thermal list (region surfaces, nested layer shells at 1:50, an open face skin below the beanie, breath and plume outlets on the surface) and the structure list (the rider's skeleton with spinal curvature, ribcage, clavicles, scapulae, pelvis and long bones, joints at the rig joints, masses at the centres of mass; the deck plate, trucks, axles and wheels of the board), with bilingual decisions.

`build()` returns the `volumes` member.
"""
import importlib.util, json, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-skateboarder-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


B = mod("body50")
C = mod("coarse")
TX = mod("texts")
R, K = B.R, B.K
SIDES = B.SIDES
LAYERS = {"hoodie-torso": [("hoodie", 0.004), ("tshirt", 0.0008), ("torso-air", 0.004)], "hoodie-sleeves": [("sleeve", 0.004), ("sleeve-air", 0.005)], "trousers": [("twill", 0.0007), ("leg-air", 0.006)], "shoes": [("upper", 0.003), ("shoe-air", 0.008), ("sock", 0.002)], "beanie": [("beanie-knit", 0.004), ("beanie-air", 0.004)]}
SHELLED = ("hoodie", "sleeve-", "forearm-", "trousers-seat", "thigh-", "shank-", "shoe-l", "shoe-r", "shoe-toe-", "beanie")
SKIPPED = ("grip-", "hoodie-hem", "hoodie-pocket", "drawcord-", "hem-", "shoe-stripe-", "brow-", "head", "neck", "jaw", "lips", "nose", "ear-", "eye-")
FL = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}
BONE = "skin"


def strip(solids):
    return [{k: v for k, v in s.items() if not k.startswith("_")} for s in solids]


def emitters(den):
    """💨 Outlets on the surface: under the nose tip, between the lips, and the plume patch on the beanie crown (region hoodie-torso, the emitter's own)."""
    t = {50: 0.0039, 100: 0.0075, 200: 0.015, 500: 0.0375}[den]
    w = {50: 0.0045, 100: 0.0075, 200: 0.015, 500: 0.0375}[den]
    crown = B.hp(0.262 + (0.004 if den == 500 else 0.0), 0.034)
    out = [
        K.ellipsoid("th-emitter-nose-breath", "head", [t, t, max(0.009, w)], B.hp(0.088, 0.170 if den <= 100 else 0.168), B.SKIN, 0.8, role="detail"),
        K.ellipsoid("th-emitter-mouth-breath", "head", [t, t, max(0.012, w)], B.hp(0.071, 0.157 if den <= 100 else 0.162), B.SKIN, 0.6, role="detail"),
        K.ellipsoid("th-emitter-plume", "head", [t, max(0.045, t), max(0.045, t)], crown, B.KNIT, 0.3, role="detail", rotation=K.rot_w(R.HEAD_TILT + 90)),
    ]
    for s, r, e in zip(out, ("face", "face", "hoodie-torso"), ("nose-breath", "mouth-breath", "plume")):
        s["region"] = r
        s["tags"] = [e]
    return out


FACE_TOP = {50: 0.172, 100: 0.172, 200: 0.168, 500: 0.168}


def face_skin(den):
    """🙂 The skin of face, ears and neck that the air reaches: the head's own sections from the hoodie collar up to the beanie rim, closed in the collar; the face at the rim is a cut through the skin that continues as the scalp under the beanie (a hair-kind region), so it is a section face and no surface (C25 a)."""
    rows = [r for r in B.NECK if r[0] >= -0.010] + [r for r in B.HEAD if 0.030 <= r[0] <= FACE_TOP[den]]
    if den >= 200:
        rows = [B.NECK[1], B.NECK[4], B.HEAD[3], B.HEAD[5], B.HEAD[7]]
    s = B.head_stack("th-face", rows, B.SKIN, 0.8, caps=(True, True), sectionFaces=["+u"])
    s["region"] = "face"
    return s


def armhole(s, i):
    """🧥 The sleeve as the air meets it: the body's sleeve from the armhole seam down, without the shoulder dome that only keeps the joint closed inside the trunk of the hoodie."""
    t = {q: v for q, v in s.items() if not q.startswith("_")}
    t["id"] = i
    t["sections"] = [q for q in s["sections"] if q["at"][0] >= ARMHOLE]
    return t


ARMHOLE = -0.031


def thermal(den, body):
    out = []
    for s in body:
        r = s["_region"]
        sid = s["id"]
        if sid.startswith("sleeve-") and den <= 100:
            e = armhole(s, f"th-{sid}-sleeve" if den == 50 else f"th-{sid}")
            e["region"] = r
            if den == 50:
                e["layer"] = "sleeve"
                inner = K.shrink({q: v for q, v in s.items() if not q.startswith("_")}, 0.004, f"th-{sid}-sleeve-air")
                inner["region"], inner["layer"], inner["role"] = r, "sleeve-air", "body"
                out.append(inner)
            out.append(e)
            continue
        if sid.startswith(("grip-",)) or (den == 50 and sid.startswith(SKIPPED) and r != "face" and sid not in ("hoodie",)):
            continue
        if r == "face" and sid in ("head", "neck", "jaw"):
            continue
        if den == 50 and r in LAYERS and sid.startswith(SHELLED) and s["kind"] != "revolve":
            depth = 0.0
            shells = []
            for k, (lid, th) in enumerate(LAYERS[r]):
                if k == 0:
                    shells.append({"id": f"th-{sid}-{lid}", "of": sid, "region": r, "layer": lid})
                else:
                    sh = K.shrink({q: v for q, v in s.items() if not q.startswith("_")}, depth, f"th-{sid}-{lid}")
                    sh["region"], sh["layer"], sh["role"] = r, lid, "body"
                    sh.pop("texture", None)
                    shells.append(sh)
                depth += th
            out += shells[::-1]
        else:
            e = {"id": f"th-{sid}", "of": sid, "region": r}
            if den == 50 and r in LAYERS:
                e["layer"] = LAYERS[r][0][0]
            out.append(e)
    out.append(face_skin(den))
    if den == 500:
        for tx, tn in ((R.AXLE_X, "f"), (-R.AXLE_X, "r")):
            ay = -R.DECK_Y + R.WHEEL_R
            tr = K.box(f"th-truck-{tn}", "board", [0.076, 0.076, 0.134], [tx, -0.040, 0.0], B.ALU, 0.68)
            tr["region"] = "trucks"
            wh = K.box(f"th-wheels-{tn}", "board", [0.076, 0.076, 0.200], [tx, ay + 0.011, 0.0], B.PU, 0.95)
            wh["region"] = "wheels"
            out += [tr, wh]
    return out + emitters(den)


def member(s, m, part):
    s["member"] = m
    s["part"] = part
    return s


def skeleton(den):
    """🦴 The rider's skeleton at one scale, authored per scale: lumbar lordosis and thoracic kyphosis, ribcage, clavicles and scapulae on the shoulder girdle (one bar from shoulder to shoulder at the coarse scales), pelvis, cervical spine, skull and mandible, femur with head and neck, patella, tibia and fibula, tarsus and toes, humerus, ulna and radius, metacarpus; coarser scales merge and thicken to the floor."""
    fl = FL[den]
    out = []
    m = lambda s, b, part="member": out.append(member(s, b, part))

    def rr(r):
        return max(r, fl / 2 + 0.0002)

    spine = [(0.0, 0.0), (0.060, -0.012), (0.130, -0.014), (0.200, -0.002), (0.280, 0.016), (0.360, 0.022), (0.440, 0.012), (R.TORSO, 0.0)]
    if den >= 200:
        spine = [(0.0, 0.0), (0.130, -0.014), (0.330, 0.020), (R.TORSO, 0.0)]
    m(K.tube("spine", "torso", [([u, v, 0.0], rr(0.021 - 0.007 * u / 0.5), rr(0.023 - 0.008 * u / 0.5)) for u, v in spine], BONE, 0.9), "torso")
    rib = [(0.150, 0.115, 0.030, 0.118), (0.220, 0.126, 0.045, 0.134), (0.290, 0.126, 0.050, 0.134), (0.350, 0.112, 0.050, 0.120), (0.400, 0.088, 0.045, 0.092), (0.432, 0.062, 0.036, 0.060)]
    if den == 200:
        rib = [rib[0], rib[2], rib[4], rib[5]]
    if den == 500:
        rib = [(0.150, 0.110, 0.030, 0.110), (0.300, 0.120, 0.050, 0.125), (0.430, 0.064, 0.040, 0.060)]
    m(K.stack("ribcage", "torso", [(u, fr, bk, hw, 2.2) for u, fr, bk, hw in rib], BONE, 0.9), "torso")
    if den <= 100:
        for s, n in SIDES:
            m(K.onto_girdle(K.tube(f"clavicle-{n}", "torso", [([0.420, -0.085, s * 0.022], rr(0.0065), rr(0.0065)), ([0.436, -0.072, s * 0.100], rr(0.0065), rr(0.0065)), ([0.448, -0.040, s * 0.170], rr(0.0070), rr(0.0070))], BONE, 0.9), R.GIRDLE, R.GIRDLE_X), R.GIRDLE)
            m(K.onto_girdle(K.ellipsoid(f"scapula-{n}", "torso", [0.075, rr(0.0075), 0.050], [0.340, 0.075, s * 0.095], BONE, 0.9), R.GIRDLE, R.GIRDLE_X), R.GIRDLE)
    else:
        m(K.onto_girdle(K.tube("shoulder-girdle", "torso", [([0.440, -0.036, -0.168], rr(0.011), rr(0.011)), ([0.424, -0.070, 0.0], rr(0.013), rr(0.013)), ([0.440, -0.036, 0.168], rr(0.011), rr(0.011))], BONE, 0.9, hint=K.U), R.GIRDLE, R.GIRDLE_X), R.GIRDLE)
    pel = [(-0.065, 0.062, -0.030, 0.045), (-0.020, 0.070, 0.070, 0.120), (0.030, 0.052, 0.100, 0.136), (0.080, 0.022, 0.100, 0.140), (0.110, 0.000, 0.080, 0.110)]
    if den >= 200:
        pel = [pel[0], pel[2], pel[4]]
    m(K.stack("pelvis-bone", "pelvis", [(u, a, b, c, 2.2) for u, a, b, c in pel], BONE, 0.9), "pelvis")
    m(K.tube("cervical", "head", [(B.hp(-0.010, 0.0), rr(0.012), rr(0.013)), (B.hp(0.045, 0.012), rr(0.011), rr(0.012)), (B.hp(0.095, 0.030), rr(0.011), rr(0.012))], BONE, 0.9), "head")
    sk = [(0.080, 0.150, -0.030, 0.050), (0.110, 0.152, -0.038, 0.064), (0.160, 0.152, -0.042, 0.070), (0.205, 0.136, -0.038, 0.066), (0.232, 0.090, -0.010, 0.040), (0.240, 0.050, 0.010, 0.012)]
    if den >= 200:
        sk = [sk[0], sk[2], sk[4]]
    m(B.head_stack("skull", [(y, a, b, c, 2.0) for y, a, b, c in sk], BONE, 0.9), "head")
    if den <= 100:
        m(B.head_stack("mandible", [(0.022, 0.140, 0.100, 0.016, 2.0), (0.040, 0.148, 0.040, 0.046, 2.0), (0.072, 0.140, 0.020, 0.054, 2.0)], BONE, 0.9), "head")
    for s, n in SIDES:
        m(K.tube(f"femur-{n}", f"thigh-{n}", [([0.0, 0.0, 0.0], rr(0.022), rr(0.022)), ([0.030, 0.002, s * 0.040], rr(0.016), rr(0.016)), ([0.080, 0.006, s * 0.038], rr(0.015), rr(0.015)), ([0.300, 0.006, s * 0.010], rr(0.0135), rr(0.0135)), ([0.400, 0.0, 0.0], rr(0.025), rr(0.036))], BONE, 0.9, hint=K.V), f"thigh-{n}")
        if den <= 100:
            m(K.ellipsoid(f"patella-{n}", f"thigh-{n}", [0.024, rr(0.010), 0.022], [0.410, 0.044, 0.0], BONE, 0.9), f"thigh-{n}")
        m(K.tube(f"tibia-{n}", f"shank-{n}", [([0.0, 0.004, 0.0], rr(0.022), rr(0.036)), ([0.045, 0.010, 0.0], rr(0.016), rr(0.018)), ([0.390, 0.004, -s * 0.004], rr(0.012), rr(0.013)), ([0.432, 0.002, -s * 0.008], rr(0.014), rr(0.020))], BONE, 0.9, hint=K.V), f"shank-{n}")
        if den <= 100:
            m(K.tube(f"fibula-{n}", f"shank-{n}", [([0.035, -0.014, s * 0.026], rr(0.0060), rr(0.0060)), ([0.250, -0.012, s * 0.024], rr(0.0050), rr(0.0050)), ([0.438, -0.008, s * 0.030], rr(0.0070), rr(0.0070))], BONE, 0.9), f"shank-{n}")
        heel = B.foot_local(-0.045, -0.062)
        mid = B.foot_local(0.040, -0.040)
        ball = B.foot_local(R.BALL_FWD, -R.ANKLE + R.BALL_UP)
        m(K.tube(f"tarsus-{n}", f"foot-{n}", [([heel[0], heel[1], 0.0], rr(0.018), rr(0.018)), ([0.0, -0.014, 0.0], rr(0.022), rr(0.019)), ([mid[0], mid[1], 0.0], rr(0.032), rr(0.014)), ([ball[0], ball[1], 0.0], rr(0.036), rr(0.010))], BONE, 0.9, hint=K.W), f"foot-{n}")
        if den <= 100:
            m(K.tube(f"phalanges-{n}", f"toe-{n}", [([0.0, 0.0, 0.0], rr(0.032), rr(0.009)), ([0.050, -0.006, 0.0], rr(0.026), rr(0.007))], BONE, 0.9, hint=K.W), f"toe-{n}")
        m(K.tube(f"humerus-{n}", f"upper-arm-{n}", [([0.0, 0.0, 0.0], rr(0.022), rr(0.022)), ([0.040, 0.0, s * 0.004], rr(0.013), rr(0.013)), ([0.290, 0.0, 0.0], rr(0.0115), rr(0.0115)), ([0.330, 0.0, 0.0], rr(0.016), rr(0.031))], BONE, 0.9, hint=K.W), f"upper-arm-{n}")
        if den <= 100:
            m(K.tube(f"ulna-{n}", f"forearm-{n}", [([-0.022, -0.004, -s * 0.012], rr(0.011), rr(0.011)), ([0.250, 0.0, -s * 0.016], rr(0.006), rr(0.006))], BONE, 0.9), f"forearm-{n}")
            m(K.tube(f"radius-{n}", f"forearm-{n}", [([0.005, 0.004, s * 0.012], rr(0.0065), rr(0.0065)), ([0.255, 0.010, s * 0.010], rr(0.0105), rr(0.0105))], BONE, 0.9), f"forearm-{n}")
        else:
            m(K.tube(f"forearm-bones-{n}", f"forearm-{n}", [([-0.020, 0.0, 0.0], rr(0.018), rr(0.011)), ([0.255, 0.004, 0.0], rr(0.020), rr(0.010))], BONE, 0.9, hint=K.V), f"forearm-{n}")
        m(K.tube(f"metacarpus-{n}", f"hand-{n}", [([0.000, 0.002, 0.0], rr(0.022), rr(0.010)), ([0.035, 0.004, 0.0], rr(0.032), rr(0.008)), ([0.090, 0.002, 0.0], rr(0.035), rr(0.008))], BONE, 0.9, hint=K.V), f"hand-{n}")
    return out


def joints(den):
    fl = FL[den]
    r = lambda x: max(x, fl / 2 + 0.0002)
    js = [("torso", 0.020), ("head", 0.016)]
    for s, n in SIDES:
        js += [(f"thigh-{n}", 0.024), (f"shank-{n}", 0.028), (f"foot-{n}", 0.020), (f"upper-arm-{n}", 0.024), (f"forearm-{n}", 0.020)]
        if den <= 200:
            js.append((f"hand-{n}", 0.015))
        if den <= 100:
            js.append((f"toe-{n}", 0.012))
    js += [("wheel-f", 0.006), ("wheel-r", 0.006)] if den <= 200 else []
    out = []
    for b, rad in js:
        if b.startswith("wheel"):
            s = K.capsule(f"joint-{b}", b, [0.0, 0.0, -R.AXLE_HALF + 0.004], [0.0, 0.0, R.AXLE_HALF - 0.004], r(rad), r(rad), B.ALU, 0.5)
        else:
            s = K.ellipsoid(f"joint-{b}", b, [r(rad), r(rad), r(rad)], [0.0, 0.0, 0.0], BONE, 0.5)
        out.append(member(s, b, "joint"))
    return out


def masses(den, rig):
    fl = FL[den]
    out = []
    for b in rig["bones"]:
        mm = b.get("mass")
        if mm is None or (mm["kg"] < 4.0 and b["id"] != "board"):
            continue
        rad = max(fl / 2 + 0.0002, 0.0065 * mm["kg"] ** (1 / 3))
        cy = mm["comY"] if b["id"] != "board" else 0.0
        out.append(member(K.ellipsoid(f"mass-{b['id']}", b["id"], [rad, rad if b["id"] != "board" else max(fl / 2 + 0.0002, 0.0045), rad], [mm["comX"], cy, 0.0], B.ALU if b["id"] == "board" else BONE, 0.3), b["id"], "mass"))
    return out


BOARD_MEMBERS = ("deck", "hanger-", "baseplate-", "bushing", "kingpin-", "pivot-", "axle-", "truck-", "wheel-", "board")


def board_structure(den, body):
    """🛹 The board's load path: the deck plate carries the feet to the baseplates, the hangers, bushings and pivots to the axles and the wheels to the ground; at 1:500 the board is one slab."""
    return [{"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"} for s in body if s["_region"] in ("deck", "trucks", "wheels") and s["id"].startswith(BOARD_MEMBERS)]


def build(rig=None):
    if rig is None:
        rig = json.load(open(os.path.join(T, "🗑️generated", "r3-skateboarder", "root.json")))["rig"]
    bodies = {50: B.build(), 100: C.body100(), 200: C.body200(), 500: C.body500()}
    vol = {}
    for den, body in bodies.items():
        st = strip(skeleton(den)) + strip(joints(den)) + strip(masses(den, rig)) + board_structure(den, body)
        vol[str(den)] = {"status": "authored", "denominator": den, "body": strip(body), "thermal": strip(thermal(den, body)), "structure": st, "decisions": TX.DECISIONS[den]}
    return vol
