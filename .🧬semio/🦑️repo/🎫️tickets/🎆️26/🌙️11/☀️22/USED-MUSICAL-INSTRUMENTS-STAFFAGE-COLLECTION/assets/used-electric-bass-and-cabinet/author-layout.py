"""📐 Layout of the used electric bass and cabinet: every hand-chosen dimension, the rest frames of the bones and the derived contact geometry (where the bass sits in its stand), from which `author-bass.py` builds every scale.

Sources of the numbers: the reference table of `../../reports/used-electric-bass-and-cabinet.md`.
"""
import importlib.util
import math
import pathlib

HERE = pathlib.Path(__file__).parent
_spec = importlib.util.spec_from_file_location("geom", HERE / "author-geometry.py")
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

CAB_D, CAB_H, CAB_W = 0.42, 0.66, 0.62
CASTOR_H, WHEEL_R, WHEEL_W, TRAIL = 0.095, 0.0375, 0.025, 0.025
CASTOR_U = (0.07, 0.35)
CASTOR_W = 0.255
HEAD_D, HEAD_H, HEAD_W, HEAD_FEET = 0.30, 0.24, 0.55, 0.010
HEAD_U0 = 0.06

SCALE_LENGTH = 0.864
BASS_LEAN = 12.0
BASS_DEG = 90.0 + BASS_LEAN
BODY_T = 0.045
NUT_S, BOARD_END_S = 0.949, 0.352
NECK_HALF_NUT, NECK_HALF_END = 0.02075, 0.03175
V_BOARD_TOP, V_BOARD_BOTTOM, V_NECK_BACK = -0.0335, -0.0255, -0.0075
V_HEAD_FACE, V_HEAD_BACK = -0.0255, -0.0115
BRIDGE_S = 0.075
TUNER_S = (0.985, 1.035, 1.085, 1.135)
POST_W = -0.037
YOKE_S = 0.62

STAND_X, STAND_Z = 0.55, -0.52
TUBE_R, FOAM_R, CAP_R = 0.008, 0.012, 0.011
ARM_W, ARM_Y = 0.13, 0.195
CROSS_Y = 0.60
REAR_FOOT_U = -0.33

BODY_CONTROL = [
    (0.000, 0.000), (0.010, 0.075), (0.045, 0.140), (0.105, 0.168), (0.175, 0.160), (0.240, 0.130), (0.300, 0.120),
    (0.355, 0.118), (0.400, 0.102), (0.428, 0.080), (0.430, 0.062), (0.415, 0.050), (0.418, 0.036), (0.430, 0.000),
    (0.440, -0.034), (0.468, -0.050), (0.497, -0.068), (0.500, -0.088), (0.475, -0.108), (0.420, -0.126), (0.350, -0.138),
    (0.280, -0.140), (0.210, -0.150), (0.130, -0.165), (0.060, -0.152), (0.012, -0.085),
]

HEADSTOCK = [
    (0.945, 0.0210), (0.975, 0.0185), (1.020, 0.0190), (1.080, 0.0240), (1.130, 0.0320), (1.157, 0.0270), (1.165, 0.0100),
    (1.161, -0.0150), (1.146, -0.0400), (1.120, -0.0540), (1.060, -0.0570), (1.000, -0.0550), (0.970, -0.0450), (0.953, -0.0300), (0.945, -0.0212),
]

PICKGUARD = [
    (0.135, -0.105), (0.150, -0.124), (0.200, -0.132), (0.270, -0.126), (0.322, -0.118), (0.343, -0.095), (0.346, 0.036),
    (0.328, 0.046), (0.280, 0.060), (0.230, 0.074), (0.180, 0.080), (0.150, 0.066), (0.136, 0.040),
]


def body_outline(per=5):
    """🎸 The face outline of the body (s along the bass from the tail, w lateral) as a closed counter-clockwise ring."""
    return G.ccw(G.catmull(BODY_CONTROL, per))


def neck_half(s):
    """📏 Half the neck width at s, tapering from the end of the fretboard to the nut."""
    t = (s - BOARD_END_S) / (NUT_S - BOARD_END_S)
    return NECK_HALF_END + (NECK_HALF_NUT - NECK_HALF_END) * t


def neck_outline(s0, s1, n=6):
    """📐 The planform of neck or fretboard between s0 and s1 with a rounded heel."""
    left = [(s0 + (s1 - s0) * k / n, neck_half(s0 + (s1 - s0) * k / n)) for k in range(n + 1)]
    right = [(s, -h) for s, h in reversed(left)]
    heel = [(s0 - 0.008 * math.sin(math.pi * k / 6), -neck_half(s0) * math.cos(math.pi * k / 6)) for k in range(1, 6)]
    return G.ccw(left + right + heel)


def outline_s_at(ring, w, low=True):
    """🔎 The smallest (or largest) s where the ring crosses the lateral position w."""
    hits = []
    for i in range(len(ring)):
        (s0, w0), (s1, w1) = ring[i], ring[(i + 1) % len(ring)]
        if (w0 - w) * (w1 - w) <= 0 and w0 != w1:
            hits.append(s0 + (s1 - s0) * (w - w0) / (w1 - w0))
    return min(hits) if low else max(hits)


def horn_tip(ring):
    """🦄 The point of the bass-side horn: the largest s on the negative-w side."""
    return max((p for p in ring if p[1] < -0.04), key=lambda p: p[0])


def frames_rest(delta):
    """🦴 The rest frames of every bone; `delta` is the lateral offset of the bass centre line from the stand centre that lets both cradle arms carry the body."""
    f = {}
    f["floor"] = G.Frame(0, 0, 0, 0)
    f["stand"] = G.Frame(STAND_X, 0, STAND_Z, 0)
    f["cabinet-roll"] = G.Frame(0, 0, 0, 0)
    f["cabinet"] = G.Frame(-CAB_D / 2, CASTOR_H, 0, 0)
    for name, u, w in (("castor-front-l", CASTOR_U[1], CASTOR_W), ("castor-front-r", CASTOR_U[1], -CASTOR_W), ("castor-rear-l", CASTOR_U[0], CASTOR_W), ("castor-rear-r", CASTOR_U[0], -CASTOR_W)):
        ox = -CAB_D / 2 + u
        f[name] = G.Frame(ox, CASTOR_H, w, -90)
        f[name.replace("castor", "wheel")] = G.Frame(ox - TRAIL, CASTOR_H - (CASTOR_H - WHEEL_R), w, 0)
    f["head"] = G.Frame(-CAB_D / 2 + HEAD_U0, CASTOR_H + CAB_H, 0, 0)
    return f


def place_bass():
    """🪑 Solves where the bass rests in the stand: the lateral offset so that both cradle arms (at ±ARM_W from the stand centre) meet the body at the same height, then the height of the bass origin so that the lowest back corner of the body at the arms sits on the foam."""
    ring = body_outline()
    best = None
    for k in range(-250, 251):
        d = k * 0.0001
        sl = outline_s_at(ring, ARM_W - d)
        sr = outline_s_at(ring, -ARM_W - d)
        if best is None or abs(sl - sr) < best[0]:
            best = (abs(sl - sr), d, sl, sr)
    _, delta, sl, sr = best
    s_contact = (sl + sr) / 2
    c, s = math.cos(math.radians(BASS_DEG)), math.sin(math.radians(BASS_DEG))
    foam_top = ARM_Y + FOAM_R
    oy = foam_top - (s_contact * s + (BODY_T / 2) * c)
    ox_local = 0.030
    return {"delta": delta, "s_contact": s_contact, "origin": (STAND_X + ox_local, oy, STAND_Z + delta), "ring": ring}


BASS = place_bass()


def bass_frame():
    """🎸 The rest frame of the bass bone (and of its prismatic carrier, which is unrotated at the same origin)."""
    o = BASS["origin"]
    return G.Frame(o[0], o[1], o[2], BASS_DEG)


def stand_points():
    """🧷 The tube end points of the stand in stand-local metres, derived from where the bass rests: frame tubes 20 mm behind the body back, arms under the body, yoke behind the neck."""
    fb = bass_frame()
    fs = G.Frame(STAND_X, 0, STAND_Z, 0)
    back_off = BODY_T / 2 + TUBE_R + 0.012

    def bass_to_stand(s, v, w):
        return fs.local(*fb.world(s, v, w))

    a = bass_to_stand(0.0, back_off, 0.0)
    b = bass_to_stand(1.0, back_off, 0.0)
    slope = (b[0] - a[0]) / (b[1] - a[1])
    frame_x = lambda y: a[0] + slope * (y - a[1])
    pts = {"frame_x": frame_x}
    pts["foot"] = (frame_x(TUBE_R), TUBE_R)
    pts["top"] = (frame_x(CROSS_Y), CROSS_Y)
    pts["arm_root"] = (frame_x(ARM_Y), ARM_Y)
    face = bass_to_stand(BASS["s_contact"], -BODY_T / 2, 0.0)
    back = bass_to_stand(BASS["s_contact"], BODY_T / 2, 0.0)
    pts["contact_x"] = (back[0], face[0])
    pts["lip_x"] = face[0] + 0.010
    yoke = bass_to_stand(YOKE_S, V_NECK_BACK + FOAM_R, 0.0)
    pts["yoke"] = yoke
    half = neck_half(YOKE_S)
    pts["yoke_half"] = half
    tip = bass_to_stand(YOKE_S, V_NECK_BACK + FOAM_R - 0.050, 0.0)
    pts["yoke_tip"] = tip
    pts["rear_foot"] = (REAR_FOOT_U, TUBE_R)
    return pts


if __name__ == "__main__":
    print("[DEBUG] bass", {k: v for k, v in BASS.items() if k != "ring"})
    sp = stand_points()
    print("[DEBUG] stand", {k: v for k, v in sp.items() if k != "frame_x"})
    fb = bass_frame()
    ring = BASS["ring"]
    ys = [fb.world(s, v, w)[1] for s, w in ring for v in (-0.0225, 0.0225)]
    print("[DEBUG] body y", min(ys), max(ys), "tip y", fb.world(1.165, 0, 0)[1], "area", G.area(ring), "horn", horn_tip(ring))
    xs = [fb.world(s, v, w)[0] for s, w in ring for v in (-0.0225, 0.0225)]
    print("[DEBUG] body x", min(xs), max(xs))
