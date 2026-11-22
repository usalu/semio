"""🎸️ Authors `used-electric-guitar-and-combo` from nothing: every hand-chosen number of the rig, the four scale models, the 2D drawings, thermal, structure, poses and clips, written to `🔣️.json` next to this file.

Run: `python3 author-guitar-combo.py` (regenerates the whole document). Geometry kit: `author-geometry.py`.

@see ../../brief.md — the binding brief of the wave
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — typed twin of the contract
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("geometry", os.path.join(HERE, "author-geometry.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)
OUT = os.path.join(HERE, "🔣️.json")
AREAS_FILE = os.path.join(HERE, "author-areas.json")

SCALES = [50, 100, 200, 500]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
LEAN = 6.0
SIN, COS = math.sin(math.radians(LEAN)), math.cos(math.radians(LEAN))


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


AMP_X0, AMP_W, AMP_D = 0.0, 0.620, 0.260
AMP_FOOT, AMP_H = 0.012, 0.490
AMP_TOP = AMP_FOOT + AMP_H
PLY = 0.018
ZF, ZB = -AMP_D / 2, AMP_D / 2
STAND_X, STAND_Z = 0.900, 0.095
GUITAR_Y0 = 0.150
GUITAR_LIFT_AXIS = 85.0
SCALE_LENGTH = 0.648
NUT_U = 0.800
BRIDGE_U = NUT_U - SCALE_LENGTH
HEAD_TIP_U = 0.990
BODY_T = 0.045
TOP_W = -BODY_T / 2
GUARD_T = 0.0023
GUARD_W = TOP_W - GUARD_T
BOARD_TOP = -0.0325
BOARD_BOTTOM = -0.0265
HEAD_FACE, HEAD_BACK = -0.0280, -0.0140
STRING_W = -0.0345
JACK_UV = (0.060, -0.128)
AMP_JACK = (0.565, 0.436)
FACE_Z = -0.106
PLUG_L = 0.065
CABLE_R = 0.0035
CABLE_LENGTHS = [0.24, 0.24, 0.32, 0.24]
CABLE_DZ = [0.010, 0.030, 0.040, 0.010]
CORD_IEC = (0.075, 0.200, ZB + 0.035)
CORD_L1, CORD_L2, CORD_DZ2 = 0.196, 0.135, 0.155
STRING_TENSION_N = 461.0
GRAVITY = 9.80665

BODY_CONTROL = [
    (0.000, 0.000), (0.006, -0.060), (0.025, -0.110), (0.055, -0.146), (0.095, -0.161), (0.135, -0.157), (0.170, -0.139),
    (0.200, -0.120), (0.226, -0.115), (0.255, -0.122), (0.285, -0.134), (0.312, -0.137), (0.335, -0.129), (0.352, -0.114),
    (0.362, -0.094), (0.359, -0.074), (0.348, -0.058), (0.340, -0.044), (0.345, -0.030), (0.345, 0.030), (0.347, 0.042),
    (0.355, 0.056), (0.373, 0.069), (0.392, 0.085), (0.401, 0.101), (0.397, 0.117), (0.380, 0.128), (0.350, 0.135),
    (0.318, 0.140), (0.290, 0.139), (0.262, 0.128), (0.232, 0.118), (0.205, 0.122), (0.175, 0.139), (0.140, 0.157),
    (0.100, 0.1625), (0.060, 0.150), (0.028, 0.121), (0.008, 0.070),
]
GUARD_CONTROL = [
    (0.343, -0.036), (0.333, -0.062), (0.300, -0.096), (0.250, -0.101), (0.200, -0.092), (0.150, -0.096), (0.112, -0.118),
    (0.085, -0.126), (0.068, -0.108), (0.074, -0.080), (0.100, -0.058), (0.112, -0.040), (0.178, -0.040), (0.178, 0.064),
    (0.200, 0.100), (0.250, 0.108), (0.300, 0.112), (0.330, 0.090), (0.343, 0.040),
]
HEAD_CONTROL = [
    (0.800, -0.0215), (0.812, -0.030), (0.830, -0.040), (0.852, -0.043), (0.880, -0.034), (0.905, -0.024), (0.935, -0.020),
    (0.965, -0.012), (0.985, 0.002), (0.990, 0.020), (0.984, 0.036), (0.962, 0.045), (0.925, 0.047), (0.880, 0.046),
    (0.845, 0.042), (0.820, 0.032), (0.800, 0.0215),
]
PICKUPS = [(0.315, 0.0), (0.252, 0.0), (0.193, 10.0)]
KNOBS = [(0.140, -0.075), (0.113, -0.092), (0.086, -0.109)]
TUNER_U0, TUNER_DU = 0.838, 0.025
TUNER_V0, TUNER_DV = 0.029, 0.0018
STRING_GAUGES = [0.046, 0.036, 0.026, 0.017, 0.013, 0.010]


def body_ring(per):
    """🎸️ The double-cutaway body outline (u along the guitar, v across, bass side +v), counter-clockwise."""
    return G.ccw(G.catmull(BODY_CONTROL, per))


def guard_ring(per):
    """🤍️ The pickguard outline, counter-clockwise."""
    return G.ccw(G.catmull(GUARD_CONTROL, per))


def head_ring():
    """🪕️ The six-in-line headstock outline from the nut to the tip, counter-clockwise."""
    return G.ccw(G.catmull(HEAD_CONTROL, 2))


def neck_half_width(u):
    """↔️ Half width of the neck: 56 mm at the heel to 42 mm at the nut."""
    t = (u - 0.345) / (NUT_U - 0.345)
    return 0.028 + (0.021 - 0.028) * max(0.0, min(1.0, t))


def neck_back_depth(u):
    """⤵️ Depth of the maple shaft below the fretboard glue line: 19 mm at the 12th-fret end to 16 mm at the nut."""
    t = (u - 0.345) / (NUT_U - 0.345)
    return 0.019 + (0.016 - 0.019) * max(0.0, min(1.0, t))


def fret_u(n):
    """🎼️ Position of fret n along the guitar (equal temperament, 648 mm scale)."""
    return NUT_U - SCALE_LENGTH * (1 - 2 ** (-n / 12))


BONES = [
    {"id": "floor", "parent": None, "title": T("Floor of the shop", "Boden des Ladens"), "x": 0.0, "y": 0.0, "angleDeg": 0, "lengthM": 0.05, "joint": {"type": "fixed"}},
    {"id": "amp", "parent": "floor", "title": T("Combo amplifier cabinet", "Gehäuse des Combo-Verstärkers"), "x": AMP_X0, "y": 0.0, "angleDeg": 0, "lengthM": AMP_W,
     "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0.0, "maxM": 0.3}, "mass": {"kg": 21.9, "comX": 0.31, "comY": 0.27, "distribution": "point", "source": "authored-estimate", "reference": "combo-catalogue"}},
    {"id": "stand", "parent": "floor", "title": T("Tripod guitar stand with neck yoke", "Dreibein-Gitarrenständer mit Halsgabel"), "x": STAND_X, "y": 0.0, "z": STAND_Z, "angleDeg": 90, "lengthM": 0.915,
     "joint": {"type": "fixed"}, "mass": {"kg": 1.6, "comX": 0.33, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "stand-catalogue"}},
    {"id": "guitar-lift", "parent": "floor", "title": T("Lift of the guitar out of the stand", "Hub der Gitarre aus dem Ständer"), "x": STAND_X, "y": GUITAR_Y0, "angleDeg": 0, "lengthM": 0.05,
     "joint": {"type": "prismatic", "axisDeg": GUITAR_LIFT_AXIS, "minM": 0.0, "maxM": 0.9}},
    {"id": "guitar", "parent": "guitar-lift", "title": T("Solid-body electric guitar, from the tail strap button to the headstock tip", "Massivkorpus-E-Gitarre, vom hinteren Gurtknopf bis zur Kopfspitze"),
     "x": 0.0, "y": 0.0, "z": 0.0, "angleDeg": 90, "abductionDeg": LEAN, "lengthM": HEAD_TIP_U, "joint": {"type": "revolute", "minDeg": -80, "maxDeg": 10},
     "mass": {"kg": 3.6, "comX": 0.30, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "guitar-catalogue"}},
]


def cable_world_rest():
    """🔌️ Rest chain of the instrument cable, solved by the hanging-chain search below: list of 2D joint points."""
    return None


def plug_tail_world(frames_):
    """🎯️ World position of the tail of the plug in the guitar jack for a set of frames (the cable end)."""
    jack = G.to_world(frames_["guitar"], (JACK_UV[0], JACK_UV[1], TOP_W))
    return (jack[0], jack[1], jack[2] - PLUG_L)


def amp_plug_tail(frames_):
    """🎯️ World position of the tail of the plug in the amplifier input."""
    return G.to_world(frames_["amp"], (AMP_JACK[0], AMP_JACK[1], FACE_Z - PLUG_L))


def hanging_chain(p0, target, lengths, floor=CABLE_R):
    """⛓️ Joint points of a four-link chain from p0 to target that minimise the height of its links (a cable hanging under gravity), every joint at or above the floor."""
    best = None
    l1, l2, l3, l4 = lengths
    for i in range(0, 241):
        a1 = math.radians(-150 + i * 0.75)
        pa = (p0[0] + l1 * math.cos(a1), p0[1] + l1 * math.sin(a1))
        if pa[1] < floor - 1e-9:
            continue
        for j in range(0, 481):
            a2 = math.radians(-180 + j * 0.75)
            pb = (pa[0] + l2 * math.cos(a2), pa[1] + l2 * math.sin(a2))
            if pb[1] < floor - 1e-9:
                continue
            dxy = (target[0] - pb[0], target[1] - pb[1])
            d = math.hypot(*dxy)
            if d > l3 + l4 - 1e-6 or d < abs(l3 - l4) + 1e-6:
                continue
            base = math.atan2(dxy[1], dxy[0])
            off = math.acos(max(-1.0, min(1.0, (l3 * l3 + d * d - l4 * l4) / (2 * l3 * d))))
            for a3 in (base - off, base + off):
                pc = (pb[0] + l3 * math.cos(a3), pb[1] + l3 * math.sin(a3))
                if pc[1] < floor - 1e-9:
                    continue
                energy = l1 * (p0[1] + pa[1]) + l2 * (pa[1] + pb[1]) + l3 * (pb[1] + pc[1]) + l4 * (pc[1] + target[1])
                if best is None or energy < best[0] - 1e-12:
                    best = (energy, [p0, pa, pb, pc, target])
    return best[1]


def angles_of(points):
    """📐️ Absolute angles in degrees of the links of a 2D chain."""
    return [math.degrees(math.atan2(points[k + 1][1] - points[k][1], points[k + 1][0] - points[k][0])) for k in range(len(points) - 1)]


def wrap(a):
    """🔄️ An angle wrapped to -180..180 degrees."""
    while a > 180:
        a -= 360
    while a <= -180:
        a += 360
    return a


POSE_BASE = {
    "stored": {},
    "warming": {},
    "carried": {"amp": {"dy": 0.20}, "guitar-lift": {"dx": 0.25 * math.cos(math.radians(GUITAR_LIFT_AXIS)), "dy": 0.25 * math.sin(math.radians(GUITAR_LIFT_AXIS))}, "guitar": {"angleDeg": 2.4}},
    "played": {"guitar-lift": {"dx": 0.85 * math.cos(math.radians(GUITAR_LIFT_AXIS)), "dy": 0.85 * math.sin(math.radians(GUITAR_LIFT_AXIS))}, "guitar": {"angleDeg": -70.0}},
}


def build_rig():
    """🦴️ The full rig: amplifier, stand, guitar, the four links of the instrument cable (rest from the hanging chain) and the two links of the mains cord."""
    base = [dict(b) for b in BONES]
    rest = G.frames(base)
    p0 = amp_plug_tail(rest)
    tgt = plug_tail_world(rest)
    chain = hanging_chain((p0[0], p0[1]), (tgt[0], tgt[1]), CABLE_LENGTHS)
    absolute = angles_of(chain)
    bones = base
    prev = None
    for k in range(4):
        bid = f"cable-{k + 1}"
        if k == 0:
            bone = {"id": bid, "parent": "amp", "x": AMP_JACK[0], "y": AMP_JACK[1], "z": FACE_Z - PLUG_L, "angleDeg": round(absolute[0], 4)}
        else:
            bone = {"id": bid, "parent": prev, "x": CABLE_LENGTHS[k - 1], "y": 0.0, "z": CABLE_DZ[k - 1], "angleDeg": round(wrap(absolute[k] - absolute[k - 1]), 4)}
        bone.update({"title": T(f"Instrument cable, link {k + 1} of 4", f"Instrumentenkabel, Glied {k + 1} von 4"), "lengthM": CABLE_LENGTHS[k],
                     "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180},
                     "mass": {"kg": 0.03, "comX": CABLE_LENGTHS[k] / 2, "comY": 0.0, "distribution": "uniform", "source": "authored-estimate", "reference": "cable-catalogue"}})
        bones.append(bone)
        prev = bid
    bones.append({"id": "mains-cord-1", "parent": "amp", "title": T("Mains cord, drop from the inlet", "Netzkabel, Abgang vom Gerätestecker"), "x": CORD_IEC[0], "y": CORD_IEC[1], "z": CORD_IEC[2], "angleDeg": -90,
                  "lengthM": CORD_L1, "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}, "mass": {"kg": 0.03, "comX": CORD_L1 / 2, "comY": 0.0, "distribution": "uniform", "source": "authored-estimate", "reference": "cable-catalogue"}})
    bones.append({"id": "mains-cord-2", "parent": "mains-cord-1", "title": T("Mains cord, run on the floor to the plug", "Netzkabel, Lauf am Boden zum Stecker"), "x": CORD_L1, "y": 0.0, "z": 0.0, "angleDeg": 90,
                  "lengthM": CORD_L2, "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}, "mass": {"kg": 0.05, "comX": CORD_L2 / 2, "comY": 0.0, "distribution": "uniform", "source": "authored-estimate", "reference": "cable-catalogue"}})
    return bones, absolute


RIG_BONES, CABLE_REST_ABS = build_rig()


def pose_offsets(pose_id):
    """🎛️ Bone offsets of a pose, the cable links solved as a hanging chain to the plug of the guitar in that pose."""
    offs = {k: dict(v) for k, v in POSE_BASE[pose_id].items()}
    if pose_id in ("stored", "warming"):
        return offs
    fr = G.frames(RIG_BONES, offs)
    p0 = amp_plug_tail(fr)
    tgt = plug_tail_world(fr)
    chain = hanging_chain((p0[0], p0[1]), (tgt[0], tgt[1]), CABLE_LENGTHS)
    ab = angles_of(chain)
    for k in range(4):
        rel_now = ab[0] if k == 0 else wrap(ab[k] - ab[k - 1])
        rel_rest = CABLE_REST_ABS[0] if k == 0 else wrap(CABLE_REST_ABS[k] - CABLE_REST_ABS[k - 1])
        offs[f"cable-{k + 1}"] = {"angleDeg": round(wrap(rel_now - rel_rest), 3)}
    if pose_id == "carried":
        offs["mains-cord-2"] = {"angleDeg": -90.0}
    return offs


REST = G.frames(RIG_BONES)
ROTATING = {"guitar", "cable-1", "cable-2", "cable-3", "cable-4", "mains-cord-1", "mains-cord-2"}


def world_solid(sid, bone, kind, material, role, tone, **geometry):
    """🌍️ A solid authored in world rest metres for a bone whose rest frame is a signed permutation of the world axes (amplifier, stand)."""
    fr = REST[bone]
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    if kind == "box":
        centre, size = geometry.pop("centre"), geometry.pop("size")
        local_c = G.to_local(fr, centre)
        axes = [G.apply_t(fr[1], (1, 0, 0)), G.apply_t(fr[1], (0, 1, 0)), G.apply_t(fr[1], (0, 0, 1))]
        local_size = [0.0, 0.0, 0.0]
        for world_axis, ax in enumerate(axes):
            for i in range(3):
                if abs(ax[i]) > 0.5:
                    local_size[i] = size[world_axis]
        s["size"] = [G.r5(x) for x in local_size]
        if "cornerRadius" in geometry:
            s["cornerRadius"] = geometry.pop("cornerRadius")
        s["transform"] = {"translation": [G.r5(x) for x in local_c]}
    elif kind == "capsule":
        s["p0"] = [G.r5(x) for x in G.to_local(fr, geometry.pop("p0"))]
        s["p1"] = [G.r5(x) for x in G.to_local(fr, geometry.pop("p1"))]
        s["r0"], s["r1"] = geometry.pop("r0"), geometry.pop("r1")
    elif kind == "revolve":
        s["profile"] = [[G.r5(a), G.r5(b)] for a, b in geometry.pop("profile")]
        s["axis"] = [G.r5(x) for x in G.apply_t(fr[1], geometry.pop("axis"))]
        s["origin"] = [G.r5(x) for x in G.to_local(fr, geometry.pop("origin"))]
    elif kind == "ellipsoid":
        centre, radii = geometry.pop("centre"), geometry.pop("radii")
        axes = [G.apply_t(fr[1], (1, 0, 0)), G.apply_t(fr[1], (0, 1, 0)), G.apply_t(fr[1], (0, 0, 1))]
        local_r = [0.0, 0.0, 0.0]
        for world_axis, ax in enumerate(axes):
            for i in range(3):
                if abs(ax[i]) > 0.5:
                    local_r[i] = radii[world_axis]
        s["radii"] = [G.r5(x) for x in local_r]
        s["transform"] = {"translation": [G.r5(x) for x in G.to_local(fr, centre)]}
    s.update(geometry)
    return s


def local_solid(sid, bone, kind, material, role, tone, **geometry):
    """🏠️ A solid authored directly in the bone-local frame (u, v, w)."""
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(geometry)
    return s


def P(solid, region, views="efp", hatch=None, thermal="ref", struct=None, custom=None, role2d=None, tone2d=None, filled=True, regions2d=None):
    """🧩️ One part: its body solid, its thermal region, the views it is drawn in and its structural labels."""
    return {"solid": solid, "region": region, "views": views, "hatch": hatch, "thermal": thermal, "struct": struct or [], "custom": custom or {}, "role2d": role2d, "tone2d": tone2d, "filled": filled, "regions2d": regions2d}


def box_xyz(sid, bone, material, role, tone, x0, x1, y0, y1, z0, z1, r=0.0, **extra):
    """📦️ A world-aligned box between two corners (amplifier and stand bones)."""
    g = {"centre": ((x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2), "size": (x1 - x0, y1 - y0, z1 - z0)}
    if r > 0:
        g["cornerRadius"] = r
    g.update(extra)
    return world_solid(sid, bone, "box", material, role, tone, **g)


def amp_parts(s):
    """🔊️ The 1x12 valve combo at scale 1:s."""
    parts = []
    a = "amp"
    fz = -0.114
    if s == 500:
        parts.append(P(box_xyz("amp-block", a, "tolex", "body", 0.2, 0.0, AMP_W, 0.0, AMP_TOP, ZF, ZB, 0.0), "amp-cabinet", struct=[("amp", "member")]))
        parts.append(P(box_xyz("amp-front", a, "grille-cloth", "body", 0.55, 0.004, AMP_W - 0.004, 0.004, 0.40, ZF - 0.0015, ZF, 0.0, sheet=True), "grille", views="e", hatch=None))
        parts.append(P(box_xyz("amp-panel", a, "steel-chassis", "body", 0.75, 0.004, AMP_W - 0.004, 0.40, 0.49, ZF - 0.0015, ZF, 0.0, sheet=True), "chassis", views="e"))
        return parts
    bottom = AMP_FOOT if s <= 100 else 0.0
    outer = [(0.0, bottom), (AMP_W, bottom), (AMP_W, AMP_TOP), (0.0, AMP_TOP)]
    inner = [(PLY, bottom + PLY), (AMP_W - PLY, bottom + PLY), (AMP_W - PLY, AMP_TOP - PLY), (PLY, AMP_TOP - PLY)]
    sleeve = local_solid("amp-sleeve", a, "extrude", "tolex", "body", 0.18, plane="xy", profile=G.pts2(outer), holes=[G.pts2(list(reversed(inner)))], **{"from": ZF, "to": ZB})
    parts.append(P(sleeve, "amp-cabinet", struct=[("amp", "member")],
                   custom={"elevation": {"kind": "path", "d": G.path_d([outer, inner])}}))
    if s == 50:
        baffle_hole = [(0.31 + 0.140 * math.cos(2 * math.pi * k / 40), 0.212 + 0.140 * math.sin(2 * math.pi * k / 40)) for k in range(40)]
        baffle_outer = [(PLY, 0.030), (AMP_W - PLY, 0.030), (AMP_W - PLY, 0.398), (PLY, 0.398)]
        parts.append(P(local_solid("amp-baffle", a, "extrude", "birch-ply", "body", 0.3, plane="xy", profile=G.pts2(baffle_outer), holes=[G.pts2(list(reversed(baffle_hole)))], **{"from": -0.112, "to": -0.097}), "grille", views=""))
        parts.append(P(box_xyz("amp-grille-cloth", a, "grille-cloth", "body", 0.62, PLY, AMP_W - PLY, 0.030, 0.398, fz, -0.112, sheet=True), "grille", views="e", hatch="grille-weave"))
        parts.append(P(box_xyz("amp-faceplate", a, "steel-chassis", "body", 0.8, 0.030, 0.590, 0.398, 0.480, -0.106, -0.104, sheet=True), "chassis", views="e"))
        parts.append(P(box_xyz("amp-chassis", a, "steel-chassis", "body", 0.5, 0.040, 0.580, 0.398, AMP_TOP - PLY, -0.104, 0.060), "chassis", views="", struct=[("amp", "member")]))
        parts.append(P(box_xyz("amp-mains-transformer", a, "steel-chassis", "body", 0.3, 0.070, 0.160, 0.320, 0.402, -0.030, 0.055), "chassis", views=""))
        parts.append(P(box_xyz("amp-output-transformer", a, "steel-chassis", "body", 0.3, 0.500, 0.570, 0.335, 0.402, -0.020, 0.045), "chassis", views=""))
        power = [(0, 0), (0, 0.0175), (0.028, 0.0175), (0.030, 0.0165), (0.095, 0.0165), (0.101, 0.012), (0.105, 0.004), (0.105, 0)]
        parts.append(P(world_solid("amp-power-valves", a, "revolve", "valve-glass", "body", 0.85, profile=power, axis=(0, -1, 0), origin=(0.40, 0.402, 0.035), repeat={"count": 2, "translation": [0.06, 0.0, 0.0]}), "valves", views="", thermal="own"))
        pre = [(0, 0), (0, 0.0105), (0.012, 0.0105), (0.050, 0.0105), (0.056, 0.006), (0.058, 0)]
        parts.append(P(world_solid("amp-preamp-valves", a, "revolve", "valve-glass", "body", 0.85, profile=pre, axis=(0, -1, 0), origin=(0.16, 0.402, 0.035), repeat={"count": 3, "translation": [0.05, 0.0, 0.0]}), "valves", views="", thermal="own"))
        spk = [(0, 0), (0, 0.153), (0.008, 0.153), (0.012, 0.140), (0.060, 0.055), (0.063, 0.050), (0.105, 0.050), (0.110, 0.045), (0.110, 0)]
        parts.append(P(world_solid("amp-speaker", a, "revolve", "speaker", "body", 0.3, profile=spk, axis=(0, 0, 1), origin=(0.31, 0.212, -0.097)), "speaker", views="", thermal="own"))
        parts.append(P(box_xyz("amp-back-lower", a, "tolex", "body", 0.22, PLY, AMP_W - PLY, 0.030, 0.250, 0.112, ZB), "amp-cabinet", views=""))
        parts.append(P(box_xyz("amp-back-upper", a, "tolex", "body", 0.22, PLY, AMP_W - PLY, 0.400, AMP_TOP - PLY, 0.112, ZB), "amp-cabinet", views="e"))
        for k, (y, z) in enumerate([(0.031, -0.111), (0.031, 0.111), (0.483, -0.111), (0.483, 0.111)]):
            parts.append(P(box_xyz(f"amp-corner-{k + 1}", a, "chrome-steel", "detail", 0.7, -0.002, 0.040, y - 0.021, y + 0.021, z - 0.021, z + 0.021, 0.008, repeat={"count": 2, "translation": [0.582, 0.0, 0.0]}), "amp-cabinet",
                           views="efp" if y > 0.1 else "ef"))
        for k, z in enumerate((-0.09, 0.09)):
            parts.append(P(world_solid(f"amp-feet-{k + 1}", a, "revolve", "rubber", "detail", 0.1, profile=[(0, 0), (0, 0.016), (0.012, 0.016), (0.012, 0)], axis=(0, 1, 0), origin=(0.06, 0.0, z), repeat={"count": 2, "translation": [0.50, 0.0, 0.0]}), "amp-cabinet", views="ef"))
        parts.append(P(box_xyz("amp-handle-caps", a, "chrome-steel", "detail", 0.75, 0.210, 0.240, AMP_TOP, AMP_TOP + 0.010, -0.017, 0.017, 0.003, repeat={"count": 2, "translation": [0.17, 0.0, 0.0]}), "amp-cabinet"))
        arch_out = [(0.215 + 0.19 * t, AMP_TOP + 0.006 + 0.026 * math.sin(math.pi * t)) for t in [k / 12 for k in range(13)]]
        arch_in = [(0.215 + 0.19 * t, AMP_TOP + 0.006 + 0.026 * math.sin(math.pi * t) - 0.008) for t in [k / 12 for k in range(12, -1, -1)]]
        arch = arch_out + arch_in
        parts.append(P(local_solid("amp-handle-strap", a, "extrude", "leather", "body", 0.15, plane="xy", profile=G.pts2(arch), **{"from": -0.0145, "to": 0.0145}), "amp-cabinet",
                       custom={"elevation": {"kind": "poly", "points": G.pts2(arch), "closed": True}}))
        knob = [(0, 0), (0, 0.0125), (0.004, 0.0125), (0.006, 0.0115), (0.020, 0.0105), (0.020, 0)]
        parts.append(P(world_solid("amp-knobs", a, "revolve", "knob-plastic", "detail", 0.08, profile=knob, axis=(0, 0, -1), origin=(0.13, 0.436, -0.104), repeat={"count": 8, "translation": [0.05, 0.0, 0.0]}), "chassis", views="e"))
        parts.append(P(world_solid("amp-jewel-lamp", a, "revolve", "valve-glass", "detail", 0.95, profile=[(0, 0), (0, 0.009), (0.008, 0.009), (0.012, 0.005), (0.012, 0)], axis=(0, 0, -1), origin=(0.045, 0.445, -0.104)), "chassis", views="e"))
        parts.append(P(local_solid("amp-toggles", a, "capsule", "chrome-steel", "texture", 0.8, p0=[0.072, 0.430, -0.106], p1=[0.072, 0.442, -0.118], r0=0.002, r1=0.0015, texture=True, repeat={"count": 2, "translation": [0.025, 0.0, 0.0]}), "chassis", views=""))
        parts.append(P(local_solid("amp-tolex-flap", a, "box", "tolex", "texture", 0.45, size=[0.060, 0.032, 0.0015], texture=True, transform={"translation": [0.050, 0.040, ZF - 0.0012], "rotation": [0.0, 0.0, 0.0, 1.0]}), "amp-cabinet", views=""))
        parts.append(P(box_xyz("amp-iec-inlet", a, "knob-plastic", "detail", 0.1, 0.060, 0.090, 0.188, 0.212, ZB, ZB + 0.008), "chassis", views="", thermal="ref"))
        return parts
    if s == 100:
        parts.append(P(box_xyz("amp-front-panel", a, "grille-cloth", "body", 0.62, PLY, AMP_W - PLY, bottom + PLY, 0.402, fz, -0.097), "grille", views="e", hatch="grille-weave"))
        parts.append(P(box_xyz("amp-control-chassis", a, "steel-chassis", "body", 0.8, 0.030, 0.590, 0.402, AMP_TOP - PLY, -0.106, 0.060), "chassis", views="e", struct=[("amp", "member")]))
        parts.append(P(box_xyz("amp-transformers", a, "steel-chassis", "body", 0.3, 0.070, 0.160, 0.320, 0.402, -0.030, 0.055, repeat={"count": 2, "translation": [0.43, 0.012, 0.0]}), "chassis", views=""))
        parts.append(P(world_solid("amp-power-valves", a, "revolve", "valve-glass", "body", 0.85, profile=[(0, 0), (0, 0.0168), (0.105, 0.0168), (0.105, 0)], axis=(0, -1, 0), origin=(0.40, 0.402, 0.035), repeat={"count": 2, "translation": [0.06, 0.0, 0.0]}), "valves", views="", thermal="own"))
        parts.append(P(world_solid("amp-preamp-valves", a, "revolve", "valve-glass", "body", 0.85, profile=[(0, 0), (0, 0.0105), (0.058, 0.0105), (0.058, 0)], axis=(0, -1, 0), origin=(0.16, 0.402, 0.035), repeat={"count": 3, "translation": [0.05, 0.0, 0.0]}), "valves", views="", thermal="own"))
        spk = [(0, 0), (0, 0.152), (0.012, 0.140), (0.062, 0.050), (0.110, 0.050), (0.110, 0)]
        parts.append(P(world_solid("amp-speaker", a, "revolve", "speaker", "body", 0.3, profile=spk, axis=(0, 0, 1), origin=(0.31, 0.212, -0.097)), "speaker", views="", thermal="own"))
        parts.append(P(box_xyz("amp-back-panels", a, "tolex", "body", 0.22, PLY, AMP_W - PLY, bottom + PLY, 0.250, 0.112, ZB), "amp-cabinet", views=""))
        parts.append(P(box_xyz("amp-back-top", a, "tolex", "body", 0.22, PLY, AMP_W - PLY, 0.400, AMP_TOP - PLY, 0.112, ZB), "amp-cabinet", views="e"))
        for k, z in enumerate((-0.09, 0.09)):
            parts.append(P(box_xyz(f"amp-feet-{k + 1}", a, "rubber", "detail", 0.1, 0.044, 0.076, 0.0, AMP_FOOT, z - 0.016, z + 0.016, repeat={"count": 2, "translation": [0.50, 0.0, 0.0]}), "amp-cabinet", views="ef"))
        parts.append(P(box_xyz("amp-handle", a, "leather", "body", 0.15, 0.210, 0.410, AMP_TOP, AMP_TOP + 0.030, -0.017, 0.017, 0.006), "amp-cabinet"))
        parts.append(P(box_xyz("amp-knob-row", a, "knob-plastic", "detail", 0.08, 0.1175, 0.4925, 0.4235, 0.4485, -0.124, -0.106), "chassis", views="e"))
        return parts
    parts.append(P(box_xyz("amp-front", a, "grille-cloth", "body", 0.6, PLY, AMP_W - PLY, PLY, AMP_TOP - PLY, -0.125, -0.095), "grille", views="e",
                   custom={"elevation": {"kind": "rect", "x": PLY, "y": PLY, "w": AMP_W - 2 * PLY, "h": AMP_TOP - 2 * PLY}}))
    parts.append(P(box_xyz("amp-chassis-block", a, "steel-chassis", "body", 0.45, 0.040, 0.580, 0.320, AMP_TOP - PLY, -0.095, 0.060), "chassis", views="", struct=[("amp", "member")],
                   regions2d=None))
    parts.append(P(box_xyz("amp-power-valves", a, "valve-glass", "body", 0.85, 0.3835, 0.4765, 0.290, 0.320, 0.018, 0.052), "valves", views="", thermal="own"))
    parts.append(P(world_solid("amp-speaker", a, "revolve", "speaker", "body", 0.3, profile=[(0, 0), (0, 0.150), (0.060, 0.055), (0.105, 0.055), (0.105, 0)], axis=(0, 0, 1), origin=(0.31, 0.212, -0.095)), "speaker", views="", thermal="own"))
    parts.append(P(box_xyz("amp-back-panels", a, "tolex", "body", 0.22, PLY, AMP_W - PLY, PLY, 0.250, 0.100, ZB), "amp-cabinet", views="", struct=[("amp", "member")]))
    parts.append(P(box_xyz("amp-back-top", a, "tolex", "body", 0.22, PLY, AMP_W - PLY, 0.400, AMP_TOP - PLY, 0.100, ZB), "amp-cabinet", views=""))
    parts.append(P(box_xyz("amp-handle", a, "leather", "body", 0.15, 0.215, 0.405, AMP_TOP, AMP_TOP + 0.032, -0.016, 0.016), "amp-cabinet"))
    return parts


def amp_face_strip_2d(s):
    """🎛️ The control strip of the front at 1:200: the faceplate band drawn over the merged front board."""
    return {"kind": "rect", "x": 0.030, "y": 0.402, "w": 0.560, "h": 0.078}


def guitar_parts(s):
    """🎸️ The solid-body guitar at scale 1:s, every solid in the guitar frame (u along the guitar, v across with the bass side +v, w through the body with the top at -w)."""
    g = "guitar"
    parts = []
    if s == 50:
        ring = body_ring(4)
        r = 0.008
        levels = []
        for sign in (-1, 1):
            for phi in ((90, 60, 30, 0) if sign < 0 else (0, 30, 60, 90)):
                w = sign * (BODY_T / 2 - r + r * math.sin(math.radians(phi)))
                levels.append((G.inset(ring, r * (1 - math.cos(math.radians(phi)))), w))
        verts, tris = G.stacked_polyhedron(levels)
        parts.append(P(local_solid("guitar-body", g, "polyhedron", "alder-poly", "body", 0.42, vertices=verts, triangles=tris), "guitar-body", struct=[("guitar", "member")],
                       custom={"elevation": {"kind": "poly", "points": G.pts2(ring), "closed": True}}))
    else:
        ring = body_ring({100: 2, 200: 1, 500: 1}[s])
        if s == 500:
            ring = G.ccw([(0.0, 0.0), (0.03, -0.12), (0.10, -0.1615), (0.20, -0.12), (0.30, -0.137), (0.362, -0.094), (0.345, -0.03), (0.345, 0.03), (0.401, 0.101), (0.35, 0.135), (0.29, 0.139), (0.21, 0.12), (0.10, 0.1625), (0.03, 0.12)])
        parts.append(P(local_solid("guitar-body", g, "extrude", "alder-poly", "body", 0.42, plane="xy", profile=G.pts2(ring), **{"from": TOP_W, "to": -TOP_W}), "guitar-body", struct=[("guitar", "member")],
                       custom={"elevation": {"kind": "poly", "points": G.pts2(ring), "closed": True}}, regions2d=["guitar-body", "guitar-hardware"] if s == 500 else None))
    if s <= 200:
        gr = guard_ring(3 if s == 50 else 1)
        parts.append(P(local_solid("guitar-guard", g, "extrude", "pickguard-abs", "body", 0.9, plane="xy", profile=G.pts2(gr), sheet=True, **{"from": GUARD_W, "to": TOP_W}), "guitar-body",
                       custom={"elevation": {"kind": "poly", "points": G.pts2(gr), "closed": True}}, role2d="detail"))
    if s == 50:
        for k, (u, ang) in enumerate(PICKUPS):
            q = [0.0, 0.0, math.sin(math.radians(ang) / 2), math.cos(math.radians(ang) / 2)]
            parts.append(P(local_solid(f"guitar-pickup-{k + 1}", g, "box", "pickguard-abs", "detail", 0.95, size=[0.017, 0.070, 0.009], cornerRadius=0.004, transform={"translation": [u, 0.0, GUARD_W - 0.0045], "rotation": q}), "guitar-body"))
        bridge = [(TOP_W, 0.125), (TOP_W, 0.172), (TOP_W - 0.010, 0.172), (TOP_W - 0.010, 0.142), (TOP_W - 0.003, 0.137), (TOP_W - 0.003, 0.125)]
        parts.append(P(local_solid("guitar-bridge", g, "extrude", "chrome-steel", "detail", 0.75, plane="zx", profile=G.pts2(bridge), **{"from": -0.036, "to": 0.036}), "guitar-hardware"))
        knob = [(0, 0), (0, 0.0095), (0.015, 0.0085), (0.017, 0.007), (0.017, 0)]
        for k, (u, v) in enumerate(KNOBS):
            parts.append(P(local_solid(f"guitar-knob-{k + 1}", g, "revolve", "pickguard-abs", "detail", 0.92, profile=G.pts2(knob), axis=[0, 0, -1], origin=[u, v, GUARD_W]), "guitar-body"))
        parts.append(P(local_solid("guitar-jack-cup", g, "ellipsoid", "chrome-steel", "detail", 0.7, radii=[0.022, 0.015, 0.004], transform={"translation": [JACK_UV[0], JACK_UV[1], TOP_W]}), "guitar-hardware"))
        parts.append(P(local_solid("guitar-neck-heel", g, "box", "maple-neck", "body", 0.72, size=[0.078, 0.056, 0.019], transform={"translation": [0.311, 0.0, BOARD_BOTTOM + 0.0095]}), "guitar-neck", views="e"))
        sections = []
        for u in (0.330, 0.345, 0.45, 0.56, 0.68, 0.78, NUT_U + 0.008):
            hw, dp = neck_half_width(u), neck_back_depth(u)
            ring_vw = []
            for k in range(13):
                al = math.pi * k / 12
                c = math.cos(al)
                sgn = 1 if math.sin(al) >= 0 else -1
                ring_vw.append((hw * (abs(c) ** 0.8) * (1 if c >= 0 else -1), BOARD_BOTTOM + dp * (abs(math.sin(al)) ** 0.9) * sgn))
            sections.append((u, G.ccw(ring_vw)))
        nv, nt = G.lofted_polyhedron(sections)
        parts.append(P(local_solid("guitar-neck", g, "polyhedron", "maple-neck", "body", 0.72, vertices=nv, triangles=nt), "guitar-neck", struct=[("guitar", "member")]))
        board = [(0.328, -0.0285), (NUT_U, -0.0210), (NUT_U, 0.0210), (0.328, 0.0285)]
        parts.append(P(local_solid("guitar-fretboard", g, "extrude", "rosewood", "body", 0.2, plane="xy", profile=G.pts2(board), **{"from": BOARD_TOP, "to": BOARD_BOTTOM}), "guitar-neck"))
        parts.append(P(local_solid("guitar-nut", g, "box", "bone", "detail", 0.95, size=[0.004, 0.042, 0.004], transform={"translation": [NUT_U + 0.002, 0.0, BOARD_TOP - 0.002]}), "guitar-neck"))
        hr = head_ring()
        parts.append(P(local_solid("guitar-head", g, "extrude", "maple-neck", "body", 0.68, plane="xy", profile=G.pts2(hr), **{"from": HEAD_FACE, "to": HEAD_BACK}), "guitar-neck",
                       custom={"elevation": {"kind": "poly", "points": G.pts2(hr), "closed": True}}))
        post = [(0, 0), (0, 0.005), (0.004, 0.005), (0.004, 0.003), (0.016, 0.003), (0.016, 0)]
        parts.append(P(local_solid("guitar-tuner-posts", g, "revolve", "nickel-steel", "detail", 0.78, profile=G.pts2(post), axis=[0, 0, -1], origin=[TUNER_U0, TUNER_V0, HEAD_FACE], repeat={"count": 6, "translation": [TUNER_DU, TUNER_DV, 0.0]}), "guitar-hardware"))
        parts.append(P(local_solid("guitar-tuner-keys", g, "box", "nickel-steel", "detail", 0.78, size=[0.012, 0.036, 0.010], cornerRadius=0.003, transform={"translation": [TUNER_U0, TUNER_V0 + 0.018, HEAD_BACK + 0.005]}, repeat={"count": 6, "translation": [TUNER_DU, TUNER_DV, 0.0]}), "guitar-hardware"))
        button = [(0, 0), (0, 0.006), (0.012, 0.006), (0.014, 0.0)]
        parts.append(P(local_solid("guitar-strap-tail", g, "revolve", "chrome-steel", "detail", 0.75, profile=G.pts2(button), axis=[-1, 0, 0], origin=[0.002, 0.0, 0.0]), "guitar-hardware"))
        parts.append(P(local_solid("guitar-strap-horn", g, "revolve", "chrome-steel", "detail", 0.75, profile=G.pts2(button), axis=[1, 0, 0], origin=[0.399, 0.101, 0.0]), "guitar-hardware"))
        parts.append(P(local_solid("guitar-neck-plate", g, "box", "chrome-steel", "detail", 0.7, size=[0.051, 0.064, 0.002], sheet=True, transform={"translation": [0.300, 0.0, -TOP_W + 0.001]}), "guitar-hardware", views="fp"))
        strings = []
        for k, gauge in enumerate(STRING_GAUGES):
            vs = 0.0262 - k * 0.0105
            vn = 0.0175 - k * 0.0070
            post_u = TUNER_U0 + TUNER_DU * k
            post_v = TUNER_V0 + TUNER_DV * k
            rad = gauge * 0.0254 / 2
            strings.append(P(local_solid(f"guitar-string-{k + 1}", g, "capsule", "nickel-steel", "texture", 0.75, p0=[BRIDGE_U, vs, STRING_W], p1=[NUT_U, vn, STRING_W], r0=rad, r1=rad, texture=True, slender=True), "guitar-hardware", views=""))
        parts.extend(strings)
        for n in range(1, 23):
            u = fret_u(n)
            hw = neck_half_width(u) if u > 0.345 else 0.0285
            parts.append(P(local_solid(f"guitar-fret-{n}", g, "box", "nickel-steel", "texture", 0.8, size=[0.0024, 2 * hw - 0.001, 0.0012], texture=True, transform={"translation": [u, 0.0, BOARD_TOP - 0.0006]}), "guitar-hardware", views=""))
        return parts
    if s == 100:
        neck_profile = [(0.300, -0.028), (NUT_U + 0.005, -0.021), (NUT_U + 0.005, 0.021), (0.300, 0.028)]
        parts.append(P(local_solid("guitar-neck", g, "extrude", "maple-neck", "body", 0.6, plane="xy", profile=G.pts2(neck_profile), **{"from": BOARD_TOP, "to": BOARD_TOP + 0.022}), "guitar-neck", struct=[("guitar", "member")],
                       custom={"elevation": {"kind": "poly", "points": G.pts2([(0.328, -0.0285), (NUT_U + 0.005, -0.021), (NUT_U + 0.005, 0.021), (0.328, 0.0285)]), "closed": True}}))
        hr = head_ring()
        parts.append(P(local_solid("guitar-head", g, "extrude", "maple-neck", "body", 0.68, plane="xy", profile=G.pts2(hr), **{"from": HEAD_FACE - 0.0005, "to": HEAD_BACK + 0.0005}), "guitar-neck",
                       custom={"elevation": {"kind": "poly", "points": G.pts2(hr), "closed": True}}))
        parts.append(P(local_solid("guitar-tuner-row", g, "box", "nickel-steel", "detail", 0.78, size=[0.140, 0.020, 0.030], transform={"translation": [0.900, 0.040, (HEAD_FACE + HEAD_BACK) / 2]}), "guitar-hardware"))
        parts.append(P(local_solid("guitar-bridge", g, "box", "chrome-steel", "detail", 0.75, size=[0.047, 0.072, 0.015], transform={"translation": [0.1485, 0.0, TOP_W - 0.0075 + 0.0002]}), "guitar-hardware"))
        return parts
    if s == 200:
        neck_head = [(0.300, -0.028), (0.80, -0.021), (0.83, -0.040), (0.88, -0.034), (0.965, -0.012), (0.990, 0.020), (0.962, 0.045), (0.88, 0.046), (0.80, 0.021), (0.300, 0.028)]
        parts.append(P(local_solid("guitar-neck-head", g, "extrude", "maple-neck", "body", 0.62, plane="xy", profile=G.pts2(neck_head), **{"from": BOARD_TOP, "to": BOARD_TOP + 0.022}), "guitar-neck", struct=[("guitar", "member")],
                       custom={"elevation": {"kind": "poly", "points": G.pts2(neck_head), "closed": True}}))
        parts.append(P(local_solid("guitar-bridge", g, "box", "chrome-steel", "detail", 0.75, size=[0.047, 0.072, 0.022], transform={"translation": [0.1485, 0.0, TOP_W - 0.011 + 0.0002]}), "guitar-hardware", views="e"))
        return parts
    neck_head = [(0.300, -0.028), (0.80, -0.022), (0.85, -0.042), (0.965, -0.012), (0.990, 0.020), (0.95, 0.046), (0.80, 0.022), (0.300, 0.028)]
    parts.append(P(local_solid("guitar-neck-head", g, "extrude", "maple-neck", "body", 0.62, plane="xy", profile=G.pts2(neck_head), **{"from": BOARD_TOP, "to": BOARD_TOP + 0.022}), "guitar-neck", struct=[("guitar", "member")],
                   custom={"elevation": {"kind": "poly", "points": G.pts2(neck_head), "closed": True}}))
    return parts


def cradle_height():
    """🛏️ Height of the cradle tubes so that their 12 mm foam just carries the lowest point of the body over each arm (from the 1:50 body polyhedron)."""
    body = next(p["solid"] for p in guitar_parts(50) if p["solid"]["id"] == "guitar-body")
    fr = REST["guitar"]
    lows = []
    for dx in (-0.085, 0.085):
        ys = [G.to_world(fr, v)[1] for v in body["vertices"] if abs(G.to_world(fr, v)[0] - (STAND_X + dx)) < 0.006]
        lows.append(min(ys))
    return round(min(lows) - 0.012 - 0.0003, 4)


def stand_parts(s):
    """🗼️ The tripod guitar stand at scale 1:s (world metres, stand frame u up, v back along -x, w lateral)."""
    st = "stand"
    X, Z = STAND_X, STAND_Z
    CY = CRADLE_Y
    parts = []
    hub_y = 0.13
    leg_local = {"p0": [hub_y, 0.0, 0.0], "p1": [0.009, 0.0, 0.27]}
    rep3 = {"count": 3, "rotationAxis": [1, 0, 0], "rotationRad": round(2 * math.pi / 3, 6), "origin": [0.0, 0.0, 0.0]}
    parts.append(P(local_solid("stand-legs", st, "capsule", "stand-steel", "body", 0.15, p0=leg_local["p0"], p1=leg_local["p1"] if s < 500 else [0.009, 0.0, 0.282], r0=0.007, r1=0.007, slender=True, repeat=rep3), "stand", struct=[("stand", "member")]))
    if s == 50:
        parts.append(P(world_solid("stand-mast", st, "capsule", "stand-steel", "body", 0.15, p0=(X, 0.105, Z), p1=(X, 0.62, Z), r0=0.0095, r1=0.0095, slender=True), "stand", struct=[("stand", "member")]))
        parts.append(P(world_solid("stand-mast-top", st, "capsule", "stand-steel", "body", 0.15, p0=(X, 0.58, Z), p1=(X, 0.905, Z), r0=0.008, r1=0.008, slender=True), "stand", struct=[("stand", "member")]))
        parts.append(P(box_xyz("stand-clamp", st, "stand-steel", "detail", 0.2, X - 0.015, X + 0.015, 0.600, 0.630, Z - 0.015, Z + 0.015, 0.004), "stand"))
        parts.append(P(world_solid("stand-clamp-knob", st, "capsule", "rubber", "detail", 0.1, p0=(X, 0.615, Z + 0.014), p1=(X, 0.615, Z + 0.034), r0=0.008, r1=0.008), "stand"))
        parts.append(P(world_solid("stand-hub", st, "revolve", "stand-steel", "detail", 0.2, profile=[(0, 0), (0, 0.017), (0.060, 0.017), (0.060, 0)], axis=(0, 1, 0), origin=(X, 0.090, Z)), "stand", struct=[("stand", "joint")]))
        parts.append(P(local_solid("stand-feet", st, "ellipsoid", "rubber", "detail", 0.08, radii=[0.009, 0.016, 0.016], transform={"translation": [0.009, 0.0, 0.27]}, repeat=rep3), "stand"))
        parts.append(P(world_solid("stand-cradle-bar", st, "capsule", "stand-steel", "body", 0.15, p0=(X - 0.085, CY, 0.080), p1=(X + 0.085, CY, 0.080), r0=0.007, r1=0.007), "stand", struct=[("stand", "member")]))
        parts.append(P(box_xyz("stand-cradle-clamp", st, "stand-steel", "detail", 0.2, X - 0.016, X + 0.016, CY - 0.016, CY + 0.016, 0.078, Z + 0.0095), "stand"))
        for k, dx in enumerate((-0.085, 0.085)):
            parts.append(P(world_solid(f"stand-cradle-arm-{k + 1}", st, "capsule", "stand-steel", "body", 0.15, p0=(X + dx, CY, 0.080), p1=(X + dx, CY, -0.040), r0=0.007, r1=0.007), "stand", struct=[("stand", "member")]))
            parts.append(P(world_solid(f"stand-cradle-tip-{k + 1}", st, "capsule", "foam", "detail", 0.1, p0=(X + dx, CY, -0.040), p1=(X + dx, CY + 0.030, -0.052), r0=0.009, r1=0.009), "stand"))
            parts.append(P(world_solid(f"stand-cradle-foam-{k + 1}", st, "capsule", "foam", "detail", 0.1, p0=(X + dx, CY, 0.062), p1=(X + dx, CY, -0.034), r0=0.012, r1=0.012), "stand", struct=[("stand", "member")]))
        parts.append(P(world_solid("stand-yoke-stem", st, "capsule", "stand-steel", "body", 0.15, p0=(X, 0.900, Z), p1=(X, 0.915, 0.082), r0=0.008, r1=0.008), "stand", struct=[("stand", "member")]))
        parts.append(P(world_solid("stand-yoke-base", st, "capsule", "foam", "body", 0.1, p0=(X - 0.035, 0.915, 0.080), p1=(X + 0.035, 0.915, 0.080), r0=0.010, r1=0.010), "stand", struct=[("stand", "member")]))
        for k, dx in enumerate((-0.035, 0.035)):
            parts.append(P(world_solid(f"stand-yoke-prong-{k + 1}", st, "capsule", "foam", "body", 0.1, p0=(X + dx, 0.915, 0.080), p1=(X + dx, 0.915, 0.036), r0=0.010, r1=0.010), "stand", struct=[("stand", "member")]))
        return parts
    parts.append(P(world_solid("stand-mast", st, "capsule", "stand-steel", "body", 0.15, p0=(X, 0.105, Z), p1=(X, 0.905, Z), r0=0.0095, r1=0.0095, slender=True), "stand", struct=[("stand", "member")]))
    rc = 0.012 if s <= 200 else 0.012
    parts.append(P(world_solid("stand-cradle", st, "capsule", "foam", "body", 0.12, p0=(X - 0.085, CY, 0.080), p1=(X + 0.085, CY, 0.080), r0=0.009, r1=0.009), "stand", struct=[("stand", "member")]))
    for k, dx in enumerate((-0.085, 0.085)):
        parts.append(P(world_solid(f"stand-cradle-arm-{k + 1}", st, "capsule", "foam", "body", 0.12, p0=(X + dx, CY, 0.080), p1=(X + dx, CY + 0.006, -0.045), r0=rc, r1=rc), "stand", struct=[("stand", "member")]))
    parts.append(P(world_solid("stand-yoke", st, "capsule", "foam", "body", 0.12, p0=(X - 0.035, 0.915, 0.080), p1=(X + 0.035, 0.915, 0.080), r0=0.010, r1=0.010), "stand", struct=[("stand", "member")]))
    for k, dx in enumerate((-0.035, 0.035)):
        parts.append(P(world_solid(f"stand-yoke-prong-{k + 1}", st, "capsule", "foam", "body", 0.12, p0=(X + dx, 0.915, 0.080), p1=(X + dx, 0.915, 0.036), r0=0.010, r1=0.010), "stand", struct=[("stand", "member")] if s == 100 else []))
    if s <= 100:
        parts.append(P(world_solid("stand-hub", st, "revolve", "stand-steel", "detail", 0.2, profile=[(0, 0), (0, 0.017), (0.060, 0.017), (0.060, 0)], axis=(0, 1, 0), origin=(X, 0.090, Z)), "stand", struct=[("stand", "joint")]))
    return parts


CRADLE_Y = cradle_height()


def cable_parts(s):
    """🔌️ The instrument cable (four links, both plugs) and the mains cord (two links, inlet connector and plug) at scale 1:s."""
    parts = []
    for k in range(4):
        bid = f"cable-{k + 1}"
        p0 = [0.0, 0.0, 0.0]
        p1 = [CABLE_LENGTHS[k], 0.0, CABLE_DZ[k]]
        parts.append(P(local_solid(f"cable-link-{k + 1}", bid, "capsule", "pvc-cable", "body", 0.1, p0=p0, p1=p1, r0=CABLE_R, r1=CABLE_R, slender=True), "cable", struct=[(bid, "member")]))
    if s == 500:
        parts[0]["solid"]["p0"] = [0.0, 0.0, round(-(FACE_Z - PLUG_L) + ZF, 5)]
        tip = parts[3]["solid"]["p1"]
        parts[3]["solid"]["p1"] = [tip[0], tip[1], round(tip[2] + PLUG_L, 5)]
    else:
        plug = [(0, 0), (0, 0.0045), (0.020, 0.0045), (0.022, 0.0065), (0.062, 0.0065), (0.065, 0.004), (0.065, 0)]
        parts.append(P(local_solid("cable-plug-amp", "cable-1", "revolve", "nickel-steel", "detail", 0.75, profile=G.pts2(plug), axis=[0, 0, 1], origin=[0.0, 0.0, 0.0]), "cable", struct=[("cable-1", "member")] if s >= 200 else []))
        tip = [CABLE_LENGTHS[3], 0.0, CABLE_DZ[3]]
        parts.append(P(local_solid("cable-plug-guitar", "cable-4", "revolve", "nickel-steel", "detail", 0.75, profile=G.pts2(plug), axis=[0, 0, 1], origin=tip), "cable", struct=[("cable-4", "member")] if s >= 200 else []))
    parts.append(P(local_solid("mains-cord-drop", "mains-cord-1", "capsule", "pvc-cable", "body", 0.1, p0=[0.0, 0.0, 0.0] if s <= 100 else [0.0, 0.0, -0.035], p1=[CORD_L1, 0.0, 0.0], r0=0.004, r1=0.004, slender=True), "cable", struct=[("mains-cord-1", "member")]))
    run_end = [CORD_L2, 0.0, CORD_DZ2] if s <= 200 else [CORD_L2 + 0.03, 0.0, CORD_DZ2 + 0.034]
    parts.append(P(local_solid("mains-cord-run", "mains-cord-2", "capsule", "pvc-cable", "body", 0.1, p0=[0.0, 0.0, 0.0], p1=run_end, r0=0.004, r1=0.004, slender=True), "cable", struct=[("mains-cord-2", "member")]))
    if s <= 200:
        parts.append(P(local_solid("mains-plug", "mains-cord-2", "box", "knob-plastic", "detail", 0.15, size=[0.050, 0.035, 0.046], cornerRadius=0.006, transform={"translation": [CORD_L2 + 0.022, 0.0135, CORD_DZ2 + 0.012]}), "cable"))
    if s <= 100:
        parts.append(P(local_solid("mains-connector", "mains-cord-1", "box", "knob-plastic", "detail", 0.12, size=[0.024, 0.030, 0.035], cornerRadius=0.003, transform={"translation": [0.0, 0.0, -0.0175]}), "cable"))
    return parts


def all_parts(s):
    """🗃️ Every part of the asset at scale 1:s."""
    return amp_parts(s) + guitar_parts(s) + stand_parts(s) + cable_parts(s)


def copies(solid):
    """🧬️ The sample points of each repeat copy of a solid, in its bone frame."""
    rep = solid.get("repeat")
    if not rep or rep["count"] == 1:
        return [G.placed_points(solid)]
    single = dict(solid)
    single.pop("repeat")
    base = G.placed_points(single)
    out = []
    for k in range(rep["count"]):
        pts = []
        for p in base:
            q = p
            for _ in range(k):
                if rep.get("rotationRad"):
                    origin = rep.get("origin", (0.0, 0.0, 0.0))
                    q = G.add(G.axis_rot(rep["rotationAxis"], rep["rotationRad"], G.sub(q, origin)), origin)
                if rep.get("translation"):
                    q = G.add(q, rep["translation"])
            pts.append(q)
        out.append(pts)
    return out


def elevation_local(bone, world_pts):
    """🖼️ World points as elevation paper coordinates of a bone (its (u, v) plane projected on the paper)."""
    o, R = REST[bone]
    pu, pv = (R[0][0], R[0][1]), (R[1][0], R[1][1])
    det = pu[0] * pv[1] - pu[1] * pv[0]
    out = []
    for p in world_pts:
        x, y = p[0] - o[0], p[1] - o[1]
        out.append(((x * pv[1] - y * pv[0]) / det, (pu[0] * y - pu[1] * x) / det))
    return out


VIEW_SCORE = {"elevation": lambda pts: -min(p[2] for p in pts), "front": lambda pts: max(p[0] for p in pts), "plan": lambda pts: max(p[1] for p in pts)}


def solid_spec(bone, local_pts, solid):
    """🌀️ The lateral sweep of a part on a rotating bone: its w span as rest lateral positions, with the drift along a cable link."""
    zo = REST[bone][0][2]
    wmin = min(p[2] for p in local_pts)
    wmax = max(p[2] for p in local_pts)
    spec = {"kind": "extruded", "sweep": "w", "z0": G.r5(zo + wmin), "z1": G.r5(zo + wmax)}
    if solid["kind"] == "capsule" and bone.startswith(("cable-", "mains-")) and abs(solid["p1"][2] - solid["p0"][2]) > 1e-6 and solid["p0"][0] == 0.0:
        r = solid["r0"]
        z_start = zo + solid["p0"][2]
        z_end = zo + solid["p1"][2]
        spec = {"kind": "extruded", "sweep": "w", "z0": G.r5(z_start - r), "z1": G.r5(z_start + r), "zTip": [G.r5(z_end - r), G.r5(z_end + r)]}
    return spec


def shapes_of_part(part, scale, counters):
    """✏️ The 2D shapes of one part in elevation, front and plan (hull of its projected solid, or the drawn profile where it is given)."""
    s = part["solid"]
    bone = s["bone"]
    out = {"elevation": [], "front": [], "plan": []}
    role = part["role2d"] or ("detail" if s["role"] == "detail" and scale == 50 else "body")
    tone = part["tone2d"] if part["tone2d"] is not None else s["tone"]
    fr = REST[bone]
    groups = copies(s)
    for view, key in (("elevation", "e"), ("front", "f"), ("plan", "p")):
        if key not in part["views"]:
            continue
        for k, local_pts in enumerate(groups):
            world = [G.to_world(fr, p) for p in local_pts]
            sid = s["id"] if len(groups) == 1 else f"{s['id']}-{k + 1}"
            counters[view] += 1
            zkey = round(3.0 + VIEW_SCORE[view](world) * (2.0 if view != "plan" else 1.0) + counters[view] * 1e-4, 4)
            meta = {"id": sid, "bone": bone, "role": role, "z": zkey, "filled": part["filled"], "tone": tone, "material": s["material"]}
            if part["regions2d"]:
                meta["regions"] = part["regions2d"]
            else:
                meta["region"] = part["region"]
            if part["hatch"] and view == "elevation":
                meta["hatch"] = part["hatch"]
            custom = part["custom"].get(view)
            if custom is not None and len(groups) == 1:
                geo = dict(custom)
            elif view == "elevation":
                geo = {"kind": "poly", "points": G.pts2(G.hull(elevation_local(bone, world))), "closed": True}
            elif bone in ROTATING:
                geo = {"kind": "poly", "points": G.pts2(G.hull([(p[0], p[1]) for p in local_pts])), "closed": True, "solid": solid_spec(bone, local_pts, s)}
            elif view == "front":
                geo = {"kind": "poly", "points": G.pts2(G.hull([(p[2], p[1]) for p in world])), "closed": True}
            else:
                geo = {"kind": "poly", "points": G.pts2(G.hull([(p[0], p[2]) for p in world])), "closed": True}
            if custom is not None and view != "elevation" and bone in ROTATING and "solid" not in geo:
                geo["solid"] = solid_spec(bone, local_pts, s)
            meta.update(geo)
            out[view].append(meta)
    return out


def stroke(sid, bone, role, z, pts, region, material, tone=0.3, closed=False, filled=False, pen=None):
    """〰️ A hand-drawn 2D stroke or patch in bone-local elevation coordinates."""
    d = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "region": region, "material": material, "kind": "poly", "points": G.pts2(pts), "closed": closed}
    if pen is not None:
        d["penMm"] = pen
    return d


def amp_elev(points_world):
    """🔊️ World (x, y) points in the elevation frame of the amplifier."""
    return elevation_local("amp", [(p[0], p[1], 0.0) for p in points_world])


def detail_strokes(scale):
    """🖊️ The hand-drawn details of one scale: strings, frets, inlays, wear, tolex tears, grille piping and knob pointers."""
    out = {"elevation": [], "front": [], "plan": []}
    e = out["elevation"]
    if scale in (50, 100):
        e.append(stroke("amp-piping", "amp", "detail", 4.6, [(PLY + 0.002, 0.032), (AMP_W - PLY - 0.002, 0.032), (AMP_W - PLY - 0.002, 0.396), (PLY + 0.002, 0.396)], "grille", "grille-cloth", tone=0.85, closed=True))
        e.append(stroke("amp-tolex-tear", "amp", "detail", 5.4, [(0.020, 0.024), (0.081, 0.024), (0.075, 0.046), (0.052, 0.058), (0.030, 0.050)], "amp-cabinet", "tolex", tone=0.62, closed=True, filled=True))
        e.append(stroke("amp-tolex-scuff-top", "amp", "detail", 5.41, [(0.140, AMP_TOP - 0.004), (0.205, AMP_TOP - 0.008), (0.262, AMP_TOP - 0.005)], "amp-cabinet", "tolex", tone=0.6))
    if scale == 50:
        for k in range(8):
            x = 0.13 + 0.05 * k
            ang = math.radians(220 - 35 * k)
            e.append(stroke(f"amp-knob-pointer-{k + 1}", "amp", "texture", 6.2 + k * 0.001, [(x, 0.436), (x + 0.010 * math.cos(ang), 0.436 + 0.010 * math.sin(ang))], "chassis", "knob-plastic", tone=0.95))
        e.append(stroke("amp-toggle-power", "amp", "texture", 6.1, [(0.072, 0.430), (0.072, 0.444)], "chassis", "chrome-steel", tone=0.8))
        e.append(stroke("amp-toggle-standby", "amp", "texture", 6.11, [(0.097, 0.430), (0.097, 0.444)], "chassis", "chrome-steel", tone=0.8))
        e.append(stroke("amp-input-high", "amp", "detail", 6.12, [(0.565 + 0.008 * math.cos(2 * math.pi * k / 12), 0.436 + 0.008 * math.sin(2 * math.pi * k / 12)) for k in range(12)], "chassis", "chrome-steel", tone=0.7, closed=True))
        e.append(stroke("amp-input-low", "amp", "detail", 6.13, [(0.530 + 0.008 * math.cos(2 * math.pi * k / 12), 0.436 + 0.008 * math.sin(2 * math.pi * k / 12)) for k in range(12)], "chassis", "chrome-steel", tone=0.7, closed=True))
        e.append(stroke("amp-legend-line", "amp", "texture", 6.14, [(0.035, 0.467), (0.585, 0.467)], "chassis", "steel-chassis", tone=0.4))
    g = "guitar"
    if scale <= 100:
        for k, gauge in enumerate(STRING_GAUGES):
            vs = 0.0262 - k * 0.0105
            vn = 0.0175 - k * 0.0070
            pu = TUNER_U0 + TUNER_DU * k
            pv = TUNER_V0 + TUNER_DV * k
            if scale == 50:
                e.append(stroke(f"guitar-string-{k + 1}", g, "texture", 7.5 + k * 0.001, [(BRIDGE_U, vs), (NUT_U, vn), (pu, pv)], "guitar-hardware", "nickel-steel", tone=0.8))
        if scale == 50:
            for n in range(1, 23):
                u = fret_u(n)
                hw = neck_half_width(u) if u > 0.345 else 0.0285
                e.append(stroke(f"guitar-fret-{n}", g, "texture", 7.4 + n * 0.0001, [(u, -hw + 0.0005), (u, hw - 0.0005)], "guitar-hardware", "nickel-steel", tone=0.85))
            for n in (3, 5, 7, 9, 15, 17, 19, 21):
                u = (fret_u(n) + fret_u(n - 1)) / 2
                e.append(stroke(f"guitar-inlay-{n}", g, "texture", 7.45 + n * 0.0001, [(u + 0.003 * math.cos(2 * math.pi * k / 10), 0.003 * math.sin(2 * math.pi * k / 10)) for k in range(10)], "guitar-neck", "rosewood", tone=0.9, closed=True, filled=True))
            u12 = (fret_u(12) + fret_u(11)) / 2
            for dv in (-0.011, 0.011):
                e.append(stroke(f"guitar-inlay-12-{'b' if dv > 0 else 't'}", g, "texture", 7.46 + dv, [(u12 + 0.003 * math.cos(2 * math.pi * k / 10), dv + 0.003 * math.sin(2 * math.pi * k / 10)) for k in range(10)], "guitar-neck", "rosewood", tone=0.9, closed=True, filled=True))
            for n in (1, 2, 3, 5, 7):
                u0, u1 = fret_u(n - 1) - 0.004, fret_u(n) + 0.004
                e.append(stroke(f"guitar-board-wear-{n}", g, "texture", 7.42 + n * 0.0001, [(u0, -0.006), (u0, 0.010), (u1, 0.012), (u1, -0.004)], "guitar-neck", "rosewood", tone=0.38, closed=True, filled=True))
            e.append(stroke("guitar-forearm-wear", g, "texture", 7.2, [(0.300, 0.128), (0.262, 0.120), (0.232, 0.110), (0.250, 0.096), (0.292, 0.110)], "guitar-body", "alder-poly", tone=0.62, closed=True, filled=True))
            e.append(stroke("guitar-buckle-rash", g, "texture", 7.21, [(0.150, 0.150), (0.115, 0.156), (0.090, 0.152)], "guitar-body", "alder-poly", tone=0.65))
            for k, (u, v) in enumerate([(0.010, 0.080), (0.040, -0.135), (0.170, 0.139), (0.362, -0.090), (0.392, 0.086)]):
                e.append(stroke(f"guitar-edge-nick-{k + 1}", g, "texture", 7.22 + k * 0.001, [(u - 0.004, v), (u + 0.004, v * 0.985)], "guitar-body", "alder-poly", tone=0.75))
            for k in range(3):
                e.append(stroke(f"guitar-guard-scratch-{k + 1}", g, "texture", 7.35 + k * 0.001, [(0.200 + 0.02 * k, -0.075), (0.235 + 0.02 * k, -0.060)], "guitar-body", "pickguard-abs", tone=0.6))
            e.append(stroke("guitar-switch", g, "detail", 7.36, [(0.165, -0.092), (0.176, -0.078)], "guitar-body", "pickguard-abs", tone=0.9))
        if scale == 100:
            for k, (u, ang) in enumerate(PICKUPS):
                c, sn = math.cos(math.radians(ang)), math.sin(math.radians(ang))
                corners = [(-0.0085, -0.035), (0.0085, -0.035), (0.0085, 0.035), (-0.0085, 0.035)]
                e.append(stroke(f"guitar-pickup-{k + 1}", g, "detail", 7.3 + k * 0.001, [(u + x * c - y * sn, x * sn + y * c) for x, y in corners], "guitar-body", "pickguard-abs", tone=0.95, closed=True))
            for k, (u, v) in enumerate(KNOBS):
                e.append(stroke(f"guitar-knob-{k + 1}", g, "detail", 7.32 + k * 0.001, [(u + 0.0095 * math.cos(2 * math.pi * j / 10), v + 0.0095 * math.sin(2 * math.pi * j / 10)) for j in range(10)], "guitar-body", "pickguard-abs", tone=0.92, closed=True))
    return out


HATCHES = {50: [{"id": "grille-weave", "title": T("Woven grille cloth", "Gewebte Bespannung"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}],
           100: [{"id": "grille-weave", "title": T("Woven grille cloth", "Gewebte Bespannung"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.45, "penMm": 0.1}],
           200: [], 500: []}
PENS = {50: {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.35},
        100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
        200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25},
        500: {"body": 0.2, "detail": None, "texture": None, "glazing": 0.13, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def geometry_of(scale):
    """🗺️ The three views of one scale: every drawn part plus the hand-drawn details."""
    counters = {"elevation": 0, "front": 0, "plan": 0}
    views = {"elevation": [], "front": [], "plan": []}
    for part in all_parts(scale):
        sh = shapes_of_part(part, scale, counters)
        for v in views:
            views[v].extend(sh[v])
    if scale == 200:
        strip = {"id": "amp-control-strip", "bone": "amp", "role": "detail", "z": 4.7, "filled": True, "tone": 0.8, "region": "chassis", "material": "steel-chassis"}
        strip.update(amp_face_strip_2d(scale))
        views["elevation"].append(strip)
    if scale == 500:
        views["elevation"].append({"id": "amp-control-band", "bone": "amp", "role": "body", "z": 4.8, "filled": True, "tone": 0.75, "region": "chassis", "material": "steel-chassis", "kind": "rect", "x": 0.004, "y": 0.40, "w": AMP_W - 0.008, "h": 0.09})
    det = detail_strokes(scale)
    for v in views:
        views[v].extend(det[v])
    return views


def body_solids(scale):
    """🧱️ The body list of one scale."""
    return [p["solid"] for p in all_parts(scale)]


def thermal_solids(scale):
    """🌡️ The thermal list of one scale: references to the body where its surface is the region surface, own solids for the hidden valves and speaker, and the plume outlet."""
    out = []
    for part in all_parts(scale):
        s = part["solid"]
        if part["thermal"] is None:
            continue
        if part["thermal"] == "own":
            own = dict(s)
            own["id"] = "th-" + s["id"]
            own["region"] = part["region"]
            out.append(own)
        else:
            out.append({"id": "th-" + s["id"], "of": s["id"], "region": part["region"]})
    if scale == 500:
        out.append({"id": "th-valves-inside", "kind": "box", "bone": "amp", "material": "valve-glass", "role": "body", "tone": 0.85, "size": [0.10, 0.080, 0.080], "transform": {"translation": [0.43, 0.345, 0.03]}, "region": "valves"})
        out.append({"id": "th-speaker-inside", "kind": "revolve", "bone": "amp", "material": "speaker", "role": "body", "tone": 0.3, "profile": [[0, 0], [0, 0.15], [0.10, 0.06], [0.10, 0]], "axis": [0, 0, 1], "origin": [0.31, 0.212, -0.125], "region": "speaker"})
        out.append({"id": "th-guitar-hardware", "kind": "box", "bone": "guitar", "material": "chrome-steel", "role": "detail", "tone": 0.75, "size": [0.047, 0.072, 0.0015], "transform": {"translation": [0.1485, 0.0, TOP_W - 0.00075]}, "region": "guitar-hardware", "layer": "plating"})
    if scale == 100:
        out.append({"id": "th-guitar-strings", "kind": "box", "bone": "guitar", "material": "nickel-steel", "role": "detail", "tone": 0.78, "size": [SCALE_LENGTH, 0.016, 0.0015], "transform": {"translation": [(BRIDGE_U + NUT_U) / 2, 0.0, STRING_W]}, "region": "guitar-hardware", "layer": "plating"})
    if scale == 200:
        out.append({"id": "th-guitar-tuners", "kind": "box", "bone": "guitar", "material": "nickel-steel", "role": "detail", "tone": 0.78, "size": [0.14, 0.012, 0.0015], "transform": {"translation": [0.90, 0.040, HEAD_FACE - 0.00075]}, "region": "guitar-hardware", "layer": "plating"})
    plume_w = {50: 0.06, 100: 0.06, 200: 0.06, 500: 0.08}[scale]
    plume_z = ZB if scale < 500 else ZB + 0.0
    out.append({"id": "th-valve-plume", "kind": "box", "bone": "amp", "material": "valve-glass", "role": "body", "tone": 0.85, "size": [0.20, 0.002, plume_w], "transform": {"translation": [0.43, (0.400 if scale < 500 else AMP_TOP) - 0.001 + (0.0 if scale < 500 else 0.002), plume_z - plume_w / 2 - (0.0 if scale < 500 else 0.0)]},
                "region": "valves", "sheet": True, "tags": ["valve-plume"]})
    return out


SUPPORT_PAD = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
MASS_R = {50: 0.02, 100: 0.03, 200: 0.04, 500: 0.08}


def structure_solids(scale):
    """🏗️ The structure list of one scale: members, joints, the tripod support and the masses."""
    out = []
    for part in all_parts(scale):
        s = part["solid"]
        for member, kind in part["struct"]:
            out.append({"id": f"st-{s['id']}", "of": s["id"], "member": member, "part": kind})
    pad = SUPPORT_PAD[scale]
    out.append({"id": "support-stand-feet", "kind": "box", "bone": "stand", "material": "rubber", "role": "detail", "tone": 0.3, "size": [pad, max(0.04, 2 * pad), max(0.04, 2 * pad)], "transform": {"translation": [pad / 2, 0.0, 0.27]},
                "repeat": {"count": 3, "rotationAxis": [1, 0, 0], "rotationRad": round(2 * math.pi / 3, 6), "origin": [0.0, 0.0, 0.0]}, "member": "stand", "part": "support", "anchor": "stand-feet"})
    r = MASS_R[scale]
    out.append({"id": "mass-amp", "kind": "ellipsoid", "bone": "amp", "material": "steel-chassis", "role": "detail", "tone": 0.15, "radii": [r, r, r], "transform": {"translation": [0.31, 0.27, 0.0]}, "member": "amp", "part": "mass"})
    out.append({"id": "mass-guitar", "kind": "ellipsoid", "bone": "guitar", "material": "alder-poly", "role": "detail", "tone": 0.15, "radii": [r, r, min(r, 0.02)], "transform": {"translation": [0.30, 0.0, 0.0]}, "member": "guitar", "part": "mass"})
    out.append({"id": "mass-stand", "kind": "ellipsoid", "bone": "stand", "material": "stand-steel", "role": "detail", "tone": 0.15, "radii": [r, r, r], "transform": {"translation": [0.33, 0.0, 0.0]}, "member": "stand", "part": "mass"})
    return out


MATERIALS = [
    ("alder-poly", "Alder body under aged polyurethane finish", "Erlenkorpus unter gealterter Polyurethan-Lackierung", 0.90, 0.62, 450, 0.12, 1400, "literature", "wood-handbook-2021"),
    ("maple-neck", "Hard maple neck, satin finish worn at the hand", "Ahornhals, seidenmatt, an der Hand abgegriffen", 0.90, 0.45, 705, 0.16, 1400, "literature", "wood-handbook-2021"),
    ("rosewood", "Rosewood fretboard, oiled, worn between the first frets", "Palisandergriffbrett, geölt, zwischen den ersten Bünden abgespielt", 0.90, 0.8, 850, 0.17, 1400, "literature", "wood-handbook-2021"),
    ("pickguard-abs", "Three-ply ABS pickguard and covers, yellowed", "Dreilagiges ABS-Schlagbrett und Kappen, vergilbt", 0.92, 0.35, 1050, 0.17, 1400, "authored-estimate", "iso-10456-2007"),
    ("bone", "Bone nut", "Knochensattel", 0.92, 0.3, 1900, 0.5, 1300, "authored-estimate", "iso-10456-2007"),
    ("nickel-steel", "Nickel-plated steel, tarnished (strings, tuners)", "Vernickelter Stahl, angelaufen (Saiten, Mechaniken)", 0.18, 0.5, 7850, 45, 470, "literature", "incropera-2018"),
    ("chrome-steel", "Chrome-plated steel, dulled by use", "Verchromter Stahl, durch Gebrauch matt", 0.12, 0.4, 7850, 45, 470, "literature", "incropera-2018"),
    ("tolex", "Black vinyl tolex over 18 mm plywood, scuffed", "Schwarzes Vinyl-Tolex auf 18-mm-Sperrholz, abgeschabt", 0.92, 0.9, 680, 0.15, 1600, "authored-estimate", "iso-10456-2007"),
    ("birch-ply", "Birch plywood baffle", "Schallwand aus Birkensperrholz", 0.90, 0.6, 680, 0.15, 1600, "literature", "iso-10456-2007"),
    ("grille-cloth", "Woven grille cloth over the baffle", "Gewebte Frontbespannung auf der Schallwand", 0.90, 0.7, 400, 0.06, 1300, "authored-estimate", "iso-10456-2007"),
    ("steel-chassis", "Zinc-plated steel chassis and transformers", "Verzinktes Stahlchassis und Transformatoren", 0.35, 0.5, 7850, 50, 480, "literature", "incropera-2018"),
    ("valve-glass", "Soda-lime glass of the valve bulbs", "Kalk-Natron-Glas der Röhrenkolben", 0.92, 0.1, 2500, 1.0, 840, "literature", "incropera-2018"),
    ("speaker", "Paper cone, steel basket and ferrite magnet", "Papiermembran, Stahlkorb und Ferritmagnet", 0.90, 0.8, 4900, 4.0, 700, "authored-estimate", "incropera-2018"),
    ("knob-plastic", "Black thermoset knobs and plugs", "Schwarze Duroplast-Knöpfe und Stecker", 0.92, 0.9, 1300, 0.2, 1400, "authored-estimate", "iso-10456-2007"),
    ("leather", "Leather handle strap", "Ledergriff", 0.95, 0.8, 860, 0.16, 1500, "authored-estimate", "iso-10456-2007"),
    ("rubber", "Rubber feet and caps", "Gummifüße und Kappen", 0.92, 0.9, 1100, 0.15, 1800, "authored-estimate", "iso-10456-2007"),
    ("stand-steel", "Black powder-coated steel tube", "Schwarz pulverbeschichtetes Stahlrohr", 0.90, 0.9, 7850, 50, 480, "literature", "incropera-2018"),
    ("foam", "Polyurethane foam sleeves", "Polyurethan-Schaumhüllen", 0.90, 0.9, 60, 0.035, 1400, "authored-estimate", "iso-10456-2007"),
    ("pvc-cable", "PVC-jacketed cable", "Kabel mit PVC-Mantel", 0.92, 0.9, 1400, 0.17, 1000, "authored-estimate", "iso-10456-2007"),
]


def materials():
    """🧱️ The material table with sources."""
    return [{"id": m[0], "title": T(m[1], m[2]), "emissivity": m[3], "solarAbsorptivity": m[4], "densityKgM3": m[5], "conductivityWmK": m[6], "specificHeatJKgK": m[7], "source": m[8], "reference": m[9]} for m in MATERIALS]


REGION_DEFS = [
    ("guitar-body", T("Guitar body, pickguard and plastic parts", "Gitarrenkorpus, Schlagbrett und Kunststoffteile"), "surface", "passive", "alder-poly", 0.30, "vertical-plate", 14000, None),
    ("guitar-neck", T("Neck, fretboard and headstock", "Hals, Griffbrett und Kopfplatte"), "surface", "passive", "maple-neck", 0.04, "cylinder", 10000, None),
    ("guitar-hardware", T("Strings, bridge, tuners and plated hardware", "Saiten, Steg, Mechaniken und verchromte Teile"), "surface", "passive", "nickel-steel", 0.03, "cylinder", 3700, None),
    ("amp-cabinet", T("Tolex-covered cabinet, back panels, handle and corners", "Tolex-bezogenes Gehäuse, Rückwände, Griff und Ecken"), "surface", "passive", "tolex", 0.30, "vertical-plate", 19600, None),
    ("grille", T("Grille cloth on the baffle", "Bespannung auf der Schallwand"), "surface", "passive", "grille-cloth", 0.37, "vertical-plate", 16000, None),
    ("chassis", T("Chassis with faceplate, knobs and transformers", "Chassis mit Frontplatte, Knöpfen und Transformatoren"), "surface", "operating", "steel-chassis", 0.12, "vertical-plate", 8400, {"key": "power", "heatW": 55}),
    ("valves", T("Power and preamp valves", "End- und Vorstufenröhren"), "surface", "operating", "valve-glass", 0.03, "cylinder", 3000, {"key": "power", "heatW": 100}),
    ("speaker", T("Twelve-inch speaker", "Zwölf-Zoll-Lautsprecher"), "surface", "operating", "speaker", 0.15, "vertical-plate", 9800, {"key": "power", "heatW": 25}),
    ("stand", T("Stand tubes, yoke and cradle foam", "Ständerrohre, Halsgabel und Schaum der Auflage"), "surface", "passive", "stand-steel", 0.02, "cylinder", 3800, None),
    ("cable", T("Instrument cable, plugs and mains cord", "Instrumentenkabel, Stecker und Netzkabel"), "surface", "passive", "pvc-cable", 0.02, "cylinder", 4000, None),
]


def load_areas():
    """📐️ The region areas measured on the 1:50 model by `volumes-thermal-coverage` (written by `author-measure-areas.py`), else the hand estimates."""
    if os.path.exists(AREAS_FILE):
        with open(AREAS_FILE, encoding="utf8") as f:
            return json.load(f)
    return {"guitar-body": 0.205, "guitar-neck": 0.075, "guitar-hardware": 0.03, "amp-cabinet": 1.10, "grille": 0.22, "chassis": 0.12, "valves": 0.036, "speaker": 0.17, "stand": 0.11, "cable": 0.035}


def regions():
    """🌡️ The thermal regions with their areas, models and the one nameplate of the asset."""
    areas = load_areas()
    out = []
    for rid, title, kind, model, material, char, geom, cap, operating in REGION_DEFS:
        r = {"id": rid, "title": title, "kind": kind, "model": model, "material": material, "areaM2": round(areas[rid], 4), "characteristicLengthM": char, "geometry": geom, "skyViewFactor": 0.4,
             "layers": [], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "incropera-2018"}
        if rid == "guitar-hardware":
            r["layers"] = [{"id": "plating", "title": T("Plated steel skin 1.5 mm", "Verchromte Stahlhaut 1,5 mm"), "kind": "metal", "material": "chrome-steel", "thicknessM": 0.0015, "resistanceM2KW": 0.00003, "heatCapacityJM2K": 5500, "source": "literature", "reference": "incropera-2018"}]
        if operating:
            r["operating"] = operating
            r["reference"] = "valve-amp-heat"
        if rid == "chassis":
            r["ratedPowerW"] = 180
            r["ratedPowerBasis"] = "electrical-input"
            r["reference"] = "combo-catalogue"
        if rid == "guitar-body":
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        if rid == "valves":
            r["hidden"] = T("The valves hang from the chassis inside the cabinet; the front elevation shows the grille and the faceplate, they are seen only through the open back.", "Die Röhren hängen im Gehäuse am Chassis; die Vorderansicht zeigt Bespannung und Frontplatte, sichtbar sind sie nur durch die offene Rückseite.")
        if rid == "speaker":
            r["hidden"] = T("The speaker sits behind the grille cloth on the baffle; it is seen only through the open back.", "Der Lautsprecher sitzt hinter der Bespannung an der Schallwand; sichtbar ist er nur durch die offene Rückseite.")
        out.append(r)
    return out


def anchors():
    """📍️ Contacts, supports at the grips and the strap, load points of the strings and the hands, and the valve vent."""
    return [
        {"id": "stand-feet", "bone": "stand", "x": 0.0, "y": 0.0, "z": 0.0, "kind": "contact", "title": T("Three rubber feet of the tripod on the floor", "Drei Gummifüße des Dreibeins auf dem Boden"), "support": "contact", "footprintLengthM": 0.47, "footprintWidthM": 0.405, "frictionCoefficient": 0.5},
        {"id": "amp-feet", "bone": "amp", "x": 0.31, "y": 0.0, "z": 0.0, "kind": "contact", "title": T("Four rubber feet of the combo on the floor", "Vier Gummifüße des Combos auf dem Boden"), "support": "contact", "footprintLengthM": 0.50, "footprintWidthM": 0.18, "frictionCoefficient": 0.6},
        {"id": "amp-handle-grip", "bone": "amp", "x": 0.31, "y": AMP_TOP + 0.032, "z": 0.0, "kind": "support", "title": T("Hand on the strap handle", "Hand am Tragegriff"), "support": "pin", "footprintLengthM": 0.09, "footprintWidthM": 0.03},
        {"id": "neck-grip", "bone": "guitar", "x": 0.62, "y": 0.0, "z": -0.021, "kind": "support", "title": T("Hand around the neck", "Hand um den Hals"), "support": "pin", "footprintLengthM": 0.09, "footprintWidthM": 0.04},
        {"id": "strap-horn", "bone": "guitar", "x": 0.413, "y": 0.101, "z": 0.0, "kind": "support", "title": T("Strap on the horn button, over the shoulder", "Gurt am Hornknopf, über der Schulter"), "support": "pin", "footprintLengthM": 0.05, "footprintWidthM": 0.05},
        {"id": "strap-tail", "bone": "guitar", "x": -0.012, "y": 0.0, "z": 0.0, "kind": "support", "title": T("Strap on the tail button", "Gurt am hinteren Knopf"), "support": "pin", "footprintLengthM": 0.05, "footprintWidthM": 0.05},
        {"id": "bridge-saddles", "bone": "guitar", "x": BRIDGE_U, "y": 0.0, "z": STRING_W, "kind": "load", "title": T("String pull on the bridge saddles", "Saitenzug an den Stegreitern")},
        {"id": "nut-pull", "bone": "guitar", "x": NUT_U, "y": 0.0, "z": STRING_W, "kind": "load", "title": T("String pull on the nut", "Saitenzug am Sattel")},
        {"id": "fretting-hand", "bone": "guitar", "x": 0.70, "y": 0.0, "z": BOARD_TOP, "kind": "load", "title": T("Fingers of the fretting hand on the strings", "Finger der Greifhand auf den Saiten")},
        {"id": "picking-forearm", "bone": "guitar", "x": 0.25, "y": 0.135, "z": TOP_W, "kind": "load", "title": T("Picking forearm on the upper bout", "Unterarm der Anschlagshand auf der oberen Zarge")},
        {"id": "valve-vent", "bone": "amp", "x": 0.43, "y": 0.40, "z": ZB, "kind": "emitter", "title": T("Warm air leaving the open back above the valves", "Warme Luft, die über den Röhren aus der offenen Rückseite steigt")},
    ]


def string_loads(pose_id, offs):
    """🎻️ The pull of the six strings on bridge and nut, 461 N along the guitar in the pose."""
    fr = G.frames(RIG_BONES, offs)
    u = fr["guitar"][1][0]
    f = STRING_TENSION_N
    common = {"source": "literature", "reference": "string-tension-chart"}
    return [
        dict({"id": "string-pull-bridge", "kind": "imposed", "title": T("Six strings 10-46 pulling the bridge towards the nut", "Sechs Saiten 10-46 ziehen den Steg zum Sattel"), "anchor": "bridge-saddles", "fxN": round(f * u[0], 2), "fyN": round(f * u[1], 2), "fzN": round(f * u[2], 2)}, **common),
        dict({"id": "string-pull-nut", "kind": "imposed", "title": T("Six strings 10-46 pulling the nut towards the bridge", "Sechs Saiten 10-46 ziehen den Sattel zum Steg"), "anchor": "nut-pull", "fxN": round(-f * u[0], 2), "fyN": round(-f * u[1], 2), "fzN": round(-f * u[2], 2)}, **common),
    ]


def binding(power, plume, played=False, carried=False):
    """🌬️ The thermal binding of a pose: every region and the valve plume."""
    regs = {}
    for rid, *_ in REGION_DEFS:
        regs[rid] = {"exposedFraction": 1, "sunlitFraction": 0}
    regs["amp-cabinet"] = {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.0 if carried else 0.01, "tempC": 20}}
    regs["stand"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 20}}
    regs["cable"] = {"exposedFraction": 0.8, "sunlitFraction": 0, "contact": {"fraction": 0.2, "tempC": 20}}
    if played:
        regs["guitar-body"] = {"exposedFraction": 0.8, "sunlitFraction": 0, "contact": {"fraction": 0.2, "tempC": 33}}
        regs["guitar-neck"] = {"exposedFraction": 0.85, "sunlitFraction": 0, "contact": {"fraction": 0.15, "tempC": 33}}
        regs["guitar-hardware"] = {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 33}}
    if carried:
        regs["guitar-neck"] = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 33}}
        regs["amp-cabinet"] = {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 33}}
    return {"operating": {"power": power}, "regions": regs, "emitters": {"valve-plume": plume}}


def poses():
    """🧍️ The named poses: stored at the dealer, warming up, carried out and played standing."""
    out = []
    specs = [
        ("stored", T("Stored at the dealer", "Im Laden abgestellt"), T("The guitar stands in the tripod stand next to the switched-off combo, the instrument cable plugged in and lying on the floor, the mains cord unplugged at the wall.", "Die Gitarre steht im Dreibeinständer neben dem ausgeschalteten Combo, das Instrumentenkabel steckt und liegt am Boden, das Netzkabel liegt ausgesteckt an der Wand."),
         0.0, 0.0, ["stand-feet", "amp-feet"], [], False, False),
        ("warming", T("Valves warming in standby", "Röhren heizen im Standby"), T("Mains on, standby on: only the heaters glow and the transformers hum, about 40 W, the valves warm up behind the open back while the guitar still stands in the stand.", "Netz ein, Standby ein: nur die Heizfäden glühen und die Transformatoren brummen, etwa 40 W; die Röhren werden hinter der offenen Rückseite warm, während die Gitarre noch im Ständer steht."),
         0.22, 0.25, ["stand-feet", "amp-feet"], [], False, False),
        ("carried", T("Carried out of the shop", "Aus dem Laden getragen"), T("The combo hangs from its strap handle at knuckle height, 0.20 m off the floor, and the guitar is drawn 0.25 m up out of the stand by the neck; the stand stays behind, the cable stays plugged.", "Der Combo hängt am Tragegriff in Fingerknöchelhöhe 0,20 m über dem Boden, die Gitarre wird am Hals 0,25 m aus dem Ständer gezogen; der Ständer bleibt stehen, das Kabel bleibt eingesteckt."),
         0.0, 0.0, ["stand-feet", "amp-handle-grip", "neck-grip"], [], False, True),
        ("played", T("Played standing, amplifier on", "Im Stehen gespielt, Verstärker an"), T("The guitar hangs on the strap at hip height, neck raised 20 degrees, the hands on the neck and the upper bout; the combo plays at full rated input, 180 W, and its valves heat the air leaving the open back.", "Die Gitarre hängt am Gurt in Hüfthöhe, der Hals um 20 Grad angehoben, die Hände an Hals und oberer Zarge; der Combo spielt mit voller Nennaufnahme von 180 W, seine Röhren heizen die Luft, die aus der offenen Rückseite steigt."),
         1.0, 1.0, ["stand-feet", "amp-feet", "strap-horn", "strap-tail"], ["hands"], True, False),
    ]
    for pid, title, desc, power, plume, contacts, extra, played, carried in specs:
        offs = pose_offsets(pid)
        loads = string_loads(pid, offs)
        if "hands" in extra:
            fr = G.frames(RIG_BONES, offs)
            wv = fr["guitar"][1][2]
            loads.append({"id": "fretting-press", "kind": "live", "title": T("Fretting fingers pressing the strings, 6 N", "Greiffinger drücken die Saiten, 6 N"), "anchor": "fretting-hand", "fxN": round(6 * wv[0], 2), "fyN": round(6 * wv[1], 2), "fzN": round(6 * wv[2], 2), "source": "authored-estimate", "reference": "playing-forces"})
            loads.append({"id": "forearm-rest", "kind": "live", "title": T("Picking forearm resting on the upper bout, 12 N", "Unterarm der Anschlagshand ruht auf der oberen Zarge, 12 N"), "anchor": "picking-forearm", "fxN": 0.0, "fyN": -12.0, "fzN": 0.0, "source": "authored-estimate", "reference": "playing-forces"})
        out.append({"id": pid, "title": title, "description": desc, "bones": offs, "ground": "auto", "contacts": contacts, "thermal": binding(power, plume, played, carried), "loads": loads})
    return out


def clips(pose_list):
    """🎞️ Carry, warm-up of the valves and the strum of the played guitar."""
    played = next(p for p in pose_list if p["id"] == "played")
    strum_angle = played["bones"]["guitar"]["angleDeg"] + 1.5
    return [
        {"id": "carry", "title": T("Carry the combo and lift the guitar out", "Combo tragen und Gitarre herausheben"), "description": T("The hand takes the strap handle and lifts the 22 kg combo off its feet while the other hand draws the guitar up out of the yoke and the cradle; the stand stays behind.", "Die Hand nimmt den Tragegriff und hebt den 22 kg schweren Combo von den Füßen, die andere zieht die Gitarre aus Halsgabel und Auflage; der Ständer bleibt stehen."),
         "durationS": 4, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.6, "pose": "stored", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}, {"t": 4, "pose": "carried", "ease": "step"}],
         "contacts": [{"anchor": "stand-feet", "fromS": 0, "toS": 4}, {"anchor": "amp-feet", "fromS": 0, "toS": 0.6}, {"anchor": "amp-handle-grip", "fromS": 0.5, "toS": 4}, {"anchor": "neck-grip", "fromS": 0.5, "toS": 4}],
         "phases": [{"id": "grip", "fromS": 0, "toS": 0.6, "title": T("Taking hold", "Zugreifen")}, {"id": "lift", "fromS": 0.6, "toS": 3.0, "title": T("Lifting", "Anheben")}, {"id": "hold", "fromS": 3.0, "toS": 4, "title": T("Carrying", "Tragen")}],
         "events": [{"t": 0.6, "kind": "contact-off", "title": T("Combo leaves the floor", "Combo verlässt den Boden"), "target": "amp-feet"}, {"t": 0.5, "kind": "contact-on", "title": T("Hands on handle and neck", "Hände an Griff und Hals"), "target": "amp-handle-grip"}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "emitter:valve-plume", "ease": "step", "keys": [{"t": 0, "v": 0}]}]},
        {"id": "valves-warm-up", "title": T("Valves warm up, then play", "Röhren heizen auf, dann spielen"), "description": T("Mains on with standby: for half a minute only the heaters glow; standby off at 35 s, the guitar is taken onto the strap and played, and the valves, transformers and speaker warm the cabinet over four minutes.", "Netz ein mit Standby: eine halbe Minute glühen nur die Heizfäden; bei 35 s Standby aus, die Gitarre kommt an den Gurt und wird gespielt, Röhren, Transformatoren und Lautsprecher wärmen das Gehäuse über vier Minuten."),
         "durationS": 240, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "step"}, {"t": 5, "pose": "warming", "ease": "step"}, {"t": 35, "pose": "warming", "ease": "smooth"}, {"t": 45, "pose": "played", "ease": "step"}, {"t": 240, "pose": "played", "ease": "step"}],
         "contacts": [{"anchor": "stand-feet", "fromS": 0, "toS": 240}, {"anchor": "amp-feet", "fromS": 0, "toS": 240}, {"anchor": "strap-horn", "fromS": 40, "toS": 240}, {"anchor": "strap-tail", "fromS": 40, "toS": 240}],
         "phases": [{"id": "off", "fromS": 0, "toS": 5, "title": T("Off", "Aus")}, {"id": "heaters", "fromS": 5, "toS": 35, "title": T("Standby, heaters glowing", "Standby, Heizfäden glühen")}, {"id": "pick-up", "fromS": 35, "toS": 45, "title": T("Standby off, guitar onto the strap", "Standby aus, Gitarre an den Gurt")}, {"id": "playing", "fromS": 45, "toS": 240, "title": T("Playing, cabinet warming", "Spielen, Gehäuse wird warm")}],
         "events": [{"t": 5, "kind": "operating", "title": T("Mains switch on, standby", "Netzschalter ein, Standby"), "target": "power"}, {"t": 35, "kind": "operating", "title": T("Standby off", "Standby aus"), "target": "power"}, {"t": 40, "kind": "contact-on", "title": T("Strap on the shoulder", "Gurt auf der Schulter"), "target": "strap-horn"}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0.22}, {"t": 35, "v": 1}]}, {"target": "emitter:valve-plume", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0.1}, {"t": 35, "v": 0.3}, {"t": 120, "v": 1}]}]},
        {"id": "strum", "title": T("Strumming", "Anschlagen"), "description": T("Down and up strokes at 120 beats per minute rock the hanging guitar by one and a half degrees on its strap.", "Ab- und Aufschläge bei 120 Schlägen pro Minute wiegen die hängende Gitarre um anderthalb Grad am Gurt."),
         "durationS": 0.5, "fps": 30, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "smooth"}, {"t": 0.25, "pose": "played", "ease": "smooth", "bones": {"guitar": {"angleDeg": round(strum_angle, 3)}}}, {"t": 0.5, "pose": "played", "ease": "smooth"}],
         "contacts": [{"anchor": "stand-feet", "fromS": 0, "toS": 0.5}, {"anchor": "amp-feet", "fromS": 0, "toS": 0.5}, {"anchor": "strap-horn", "fromS": 0, "toS": 0.5}, {"anchor": "strap-tail", "fromS": 0, "toS": 0.5}],
         "phases": [{"id": "down", "fromS": 0, "toS": 0.25, "title": T("Down stroke", "Abschlag")}, {"id": "up", "fromS": 0.25, "toS": 0.5, "title": T("Up stroke", "Aufschlag")}],
         "events": [{"t": 0.0, "kind": "motion", "title": T("Down stroke", "Abschlag")}, {"t": 0.25, "kind": "motion", "title": T("Up stroke", "Aufschlag")}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 1}]}, {"target": "emitter:valve-plume", "ease": "step", "keys": [{"t": 0, "v": 1}]}]},
    ]


REFERENCES = [
    ("hornbostel-sachs-1914", "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Zeitschrift für Ethnologie 46, 553-590; MIMO Consortium (2011) revision: electric guitar 321.322-8 (box lute, plucked, electro-mechanical pick-up), amplifier and loudspeaker as electrophone accessories."),
    ("guitar-catalogue", "Catalogue-typical data of double-cutaway solid-body electric guitars of the S type: body 400 x 325 x 45 mm in alder, bolt-on maple neck, 648 mm (25.5 in) scale, 42 mm nut, 22 frets, six-in-line headstock, overall 990 mm, 3.4 to 3.8 kg."),
    ("string-tension-chart", "D'Addario & Co. (2019). String Tension Specifications, EXL110 nickel-wound 10-46: 16.2, 15.4, 16.6, 18.4, 19.5, 17.5 lb at 25.5 in scale, total 103.6 lb = 461 N."),
    ("combo-catalogue", "Catalogue-typical data of 1x12 open-back 50 W valve combos: about 620 x 500 x 260 mm, 21 to 24 kg, two EL34 or 6L6 power valves, three ECC83 preamp valves, mains input 160 to 200 W, 12 in speaker, tolex-covered plywood cabinet."),
    ("valve-amp-heat", "Philips/Mullard valve data: EL34 heater 6.3 V 1.5 A, anode dissipation up to 25 W each; ECC83 heater 6.3 V 0.3 A; bulb glass of power valves 150 to 250 C in operation (manufacturer data sheets)."),
    ("stand-catalogue", "Catalogue-typical data of tripod guitar stands with neck yoke: tubular steel 14 to 19 mm, height 0.75 to 0.95 m adjustable, foot circle 0.5 to 0.6 m, foam cradle at 0.15 m, 1.4 to 1.8 kg."),
    ("cable-catalogue", "Catalogue-typical data of 6.35 mm instrument cables (3 m, 7 mm PVC jacket, 0.1 kg) and IEC mains cords (2 m, 8 mm, 0.15 kg)."),
    ("playing-forces", "Fretting forces of 1 to 3 N per string and forearm rest loads of 10 to 15 N, authored estimates from guitar ergonomics literature (e.g. Grosshauser, T. et al. (2012), Proc. NIME)."),
    ("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook, FPL-GTR-282. USDA (densities and thermal conductivities of alder, hard maple and rosewood)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of plated and oxidised metals, glass, convection and plumes)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values."),
]


def model_bbox(scale):
    """📦️ World bounding box of the body solids of a scale in the rest pose."""
    lo, hi = [1e9] * 3, [-1e9] * 3
    for s in body_solids(scale):
        if s.get("texture"):
            continue
        fr = REST[s["bone"]]
        for p in G.placed_points(s):
            w = G.to_world(fr, p)
            for i in range(3):
                lo[i], hi[i] = min(lo[i], w[i]), max(hi[i], w[i])
    return lo, hi


def mass_total():
    """⚖️ Sum of the bone masses."""
    return round(sum(b["mass"]["kg"] for b in RIG_BONES if "mass" in b), 3)


SCALE_TEXT = {
    50: {
        "silhouette": T("At 1:50 the set is 23 mm high and 23 mm long: in the side elevation the combo shows its front with the grille cloth in its piping, the gold faceplate with eight knobs, jewel lamp, two toggles and two inputs, the strap handle, chrome corners and the torn tolex at the lower left; beside it the guitar stands face-on in the tripod stand, every part of the double-cutaway body, the pickguard with three pickups and three knobs, bridge, neck with frets and dot inlays, the six-in-line head with its tuners, and the instrument cable drops from the input to the floor and rises to the jack.",
                        "Bei 1:50 ist das Ensemble 23 mm hoch und 23 mm lang: in der Seitenansicht zeigt der Combo seine Front mit der Bespannung im Keder, die goldene Frontplatte mit acht Knöpfen, Kontrolllampe, zwei Kippschaltern und zwei Eingängen, den Tragegriff, Chromecken und das eingerissene Tolex unten links; daneben steht die Gitarre frontal im Dreibeinständer, jedes Teil des Doppel-Cutaway-Korpus, das Schlagbrett mit drei Tonabnehmern und drei Knöpfen, Steg, Hals mit Bünden und Punkteinlagen, die Sechserreihen-Kopfplatte mit Mechaniken, und das Instrumentenkabel fällt vom Eingang zum Boden und steigt zur Buchse."),
        "retained": T("Every solid of the model is drawn where it is seen: cabinet sleeve as a frame around grille and faceplate, corners, feet, handle caps and strap, knobs, lamp, stand tubes, cradle foam, yoke, both plugs, every cable link.", "Jeder Körper des Modells ist gezeichnet, wo er zu sehen ist: Gehäusemantel als Rahmen um Bespannung und Frontplatte, Ecken, Füße, Griffkappen und Riemen, Knöpfe, Lampe, Ständerrohre, Schaum der Auflage, Halsgabel, beide Stecker, jedes Kabelglied."),
        "merged": T("Nothing merged; the tuner keys behind the head are drawn as one outline per key.", "Nichts zusammengefasst; die Stimmflügel hinter der Kopfplatte sind je Flügel eine Kontur."),
        "omitted": T("Interior parts (valves, transformers, speaker, back panels) are not drawn in the side elevation, front or plan: they lie inside the cabinet and are seen only through the open back.", "Innenteile (Röhren, Transformatoren, Lautsprecher, Rückwände) sind in Seitenansicht, Vorderansicht und Grundriss nicht gezeichnet: sie liegen im Gehäuse und sind nur durch die offene Rückseite zu sehen."),
        "thinElements": T("Strings, frets, knob pointers and toggles are texture strokes at the true positions (648 mm scale, frets by equal temperament); the pickguard, grille cloth and faceplate are thin sheets drawn as filled shapes.", "Saiten, Bünde, Knopfzeiger und Kippschalter sind Texturstriche an den wahren Lagen (648 mm Mensur, Bünde gleichstufig); Schlagbrett, Bespannung und Frontplatte sind dünne Platten als gefüllte Formen."),
        "hatching": T("The grille cloth carries a 0.5 mm cross-hatch of its weave.", "Die Bespannung trägt eine 0,5-mm-Kreuzschraffur ihres Gewebes."),
        "styles": T("Body 0.25 mm, details 0.18 mm, texture 0.13 mm; tones from the black tolex and rosewood to the faded red alder, the yellowed white guard and the worn plating.", "Körper 0,25 mm, Details 0,18 mm, Textur 0,13 mm; Töne vom schwarzen Tolex und Palisander bis zum verblassten roten Erlenkorpus, dem vergilbten weißen Schlagbrett und der abgegriffenen Verchromung."),
        "overlays": T("Traces of use are drawn as their own shapes: torn tolex flap, scuffed top edge, forearm wear on the upper bout, buckle rash, five edge nicks, scratches on the guard, the worn patches of the fretboard between the first frets.", "Gebrauchsspuren sind eigene Formen: eingerissener Tolexlappen, abgeschabte Oberkante, Unterarmabrieb an der oberen Zarge, Gürtelschnallenkratzer, fünf Kantenmacken, Kratzer im Schlagbrett, die abgespielten Stellen des Griffbretts zwischen den ersten Bünden."),
    },
    100: {
        "silhouette": T("At 1:100 the set is 11.5 mm high: the combo front with grille, faceplate and knob row, handle and feet; the guitar in the stand with body, pickguard, outlined pickups and knobs, bridge, neck and head with a merged tuner row; the stand and both cables.", "Bei 1:100 ist das Ensemble 11,5 mm hoch: die Combofront mit Bespannung, Frontplatte und Knopfreihe, Griff und Füße; die Gitarre im Ständer mit Korpus, Schlagbrett, umrissenen Tonabnehmern und Knöpfen, Steg, Hals und Kopf mit zusammengefasster Mechanikreihe; Ständer und beide Kabel."),
        "retained": T("Cabinet frame, grille, faceplate, handle, feet, body outline, guard, neck, head, stand tubes, cradle and yoke, cable links and plugs.", "Gehäuserahmen, Bespannung, Frontplatte, Griff, Füße, Korpuskontur, Schlagbrett, Hals, Kopf, Ständerrohre, Auflage und Halsgabel, Kabelglieder und Stecker."),
        "merged": T("Eight knobs into one knob row, six tuners into one row on the bass edge, fretboard into the neck, cradle bar, arms and foam into three tubes, the mast into one tube.", "Acht Knöpfe zu einer Knopfreihe, sechs Mechaniken zu einer Reihe an der Bassseite, Griffbrett in den Hals, Auflagebügel, Arme und Schaum zu drei Rohren, der Mast zu einem Rohr."),
        "omitted": T("Strings, frets, inlays, knob pointers, toggles, lamp and the 2 mm corner protectors are below this scale; pickups and knobs remain as outlines only.", "Saiten, Bünde, Einlagen, Knopfzeiger, Kippschalter, Lampe und die 2 mm vorstehenden Eckschoner liegen unter diesem Maßstab; Tonabnehmer und Knöpfe bleiben nur als Umrisse."),
        "thinElements": T("Pickguard and grille cloth as filled thin shapes; the cables at their 7 and 8 mm true width.", "Schlagbrett und Bespannung als gefüllte dünne Formen; die Kabel in ihrer wahren Breite von 7 und 8 mm."),
        "hatching": T("Grille cloth with 0.45 mm single hatch.", "Bespannung mit 0,45-mm-Einfachschraffur."),
        "styles": T("Body 0.25 mm, details 0.13 mm, no texture pen.", "Körper 0,25 mm, Details 0,13 mm, kein Texturstift."),
        "overlays": T("The torn tolex and the scuffed top edge remain as the only traces of use that read at 1:100.", "Eingerissenes Tolex und abgeschabte Oberkante bleiben als einzige Gebrauchsspuren, die bei 1:100 lesbar sind."),
    },
    200: {
        "silhouette": T("At 1:200 the set is 5.7 mm high: the combo as cabinet with front board, control band and handle, the guitar as body, guard and one neck-and-head outline in the stand, the stand as mast, legs, cradle and yoke, the cables as lines.", "Bei 1:200 ist das Ensemble 5,7 mm hoch: der Combo als Gehäuse mit Frontbrett, Bedienband und Griff, die Gitarre als Korpus, Schlagbrett und eine Kontur aus Hals und Kopf im Ständer, der Ständer als Mast, Beine, Auflage und Halsgabel, die Kabel als Linien."),
        "retained": T("Cabinet, front board, control band, handle, body, guard, neck and head, bridge, stand, cables.", "Gehäuse, Frontbrett, Bedienband, Griff, Korpus, Schlagbrett, Hals und Kopf, Steg, Ständer, Kabel."),
        "merged": T("Grille and faceplate into one front board 30 mm deep, feet into the cabinet, neck and head into one outline, tuners into the head, cradle and yoke into tubes.", "Bespannung und Frontplatte zu einem 30 mm tiefen Frontbrett, Füße ins Gehäuse, Hals und Kopf zu einer Kontur, Mechaniken in den Kopf, Auflage und Halsgabel zu Rohren."),
        "omitted": T("Knobs, pickups, strap buttons, plugs' strain reliefs and all traces of use are below 30 mm.", "Knöpfe, Tonabnehmer, Gurtknöpfe, Knickschutz der Stecker und alle Gebrauchsspuren liegen unter 30 mm."),
        "thinElements": T("The guitar edge-on and the stand tubes keep their true thickness, printed with the 0.25 mm body pen.", "Die Gitarre von der Kante und die Ständerrohre behalten ihre wahre Dicke, gedruckt mit dem 0,25-mm-Körperstift."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Body 0.25 mm, details 0.13 mm.", "Körper 0,25 mm, Details 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
    500: {
        "silhouette": T("At 1:500 the set is 2.3 mm high: the combo as one block with its front and control band, the guitar as body and neck outline in the stand, the stand as mast, legs and the cradle and yoke bars, the cables as single lines.", "Bei 1:500 ist das Ensemble 2,3 mm hoch: der Combo als ein Block mit Front und Bedienband, die Gitarre als Korpus- und Halskontur im Ständer, der Ständer als Mast, Beine und die Stäbe von Auflage und Halsgabel, die Kabel als einzelne Linien."),
        "retained": T("Amp block, front, control band, guitar body and neck, stand mast, legs, cradle and yoke, cables.", "Verstärkerblock, Front, Bedienband, Gitarrenkorpus und Hals, Ständermast, Beine, Auflage und Halsgabel, Kabel."),
        "merged": T("The whole cabinet with its feet, back panels and handle into one block; the guard and hardware into the body outline (regions merged); neck and head into one outline.", "Das ganze Gehäuse mit Füßen, Rückwänden und Griff zu einem Block; Schlagbrett und Beschläge in die Korpuskontur (Regionen zusammengefasst); Hals und Kopf zu einer Kontur."),
        "omitted": T("Handle (33 mm), knobs, plugs, hub and every trace of use.", "Griff (33 mm), Knöpfe, Stecker, Nabe und alle Gebrauchsspuren."),
        "thinElements": T("Stand tubes and cables keep their true width in the model and print as 0.2 mm lines.", "Ständerrohre und Kabel behalten im Modell ihre wahre Breite und drucken als 0,2-mm-Linien."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Body 0.2 mm only.", "Nur Körperstift 0,2 mm."),
        "overlays": T("None.", "Keine."),
    },
}

VOLUME_TEXT = {
    50: [T("Own 1:50 model. Combo: a tolex sleeve of 18 mm plywood 620 x 490 x 260 mm open at front and back, the baffle with its 280 mm speaker hole behind a 1.5 mm grille cloth sheet, the zinc-plated chassis behind the 2 mm faceplate with eight knobs and the jewel lamp, two transformers and five valves hanging from the chassis, the 12 in speaker as a revolved basket and magnet, the lower and upper back panels with the open slot between, eight chrome corners, four feet, the arched leather handle on chrome caps, a peeled tolex flap as texture.",
           "Eigenes 1:50-Modell. Combo: ein Tolex-Mantel aus 18-mm-Sperrholz, 620 x 490 x 260 mm, vorn und hinten offen, die Schallwand mit 280-mm-Lautsprecherausschnitt hinter einer 1,5-mm-Bespannung, das verzinkte Chassis hinter der 2-mm-Frontplatte mit acht Knöpfen und Kontrolllampe, zwei Transformatoren und fünf Röhren am Chassis, der 12-Zoll-Lautsprecher als gedrehter Korb mit Magnet, untere und obere Rückwand mit dem offenen Schlitz dazwischen, acht Chromecken, vier Füße, der gewölbte Ledergriff auf Chromkappen, ein abstehender Tolexlappen als Textur."),
         T("Guitar: the body as a polyhedron of the double-cutaway outline with an 8 mm edge radius, the 2.3 mm pickguard sheet, three pickups, three knobs, the stepped bridge, jack cup, the D-profile maple neck lofted from 56 to 42 mm, the 6 mm rosewood board, bone nut, the head, six tuner posts and keys, both strap buttons and the neck plate; strings and frets as texture.",
           "Gitarre: der Korpus als Polyeder der Doppel-Cutaway-Kontur mit 8 mm Kantenradius, das 2,3-mm-Schlagbrett, drei Tonabnehmer, drei Knöpfe, der gestufte Steg, Buchsenschale, der D-förmige Ahornhals von 56 auf 42 mm geloftet, das 6-mm-Palisandergriffbrett, Knochensattel, der Kopf, sechs Mechanikachsen und Flügel, beide Gurtknöpfe und die Halsplatte; Saiten und Bünde als Textur."),
         T("Stand and cables: mast in two tubes, clamp with knob, hub, three legs by a 120 degree repeat with rubber feet, the cradle bar with arms, foam and upturned tips, the yoke with foam prongs; four instrument cable links of 7 mm with both plugs on the link axes, the mains cord in two links with connector and plug.",
           "Ständer und Kabel: Mast aus zwei Rohren, Klemme mit Knopf, Nabe, drei Beine als 120-Grad-Wiederholung mit Gummifüßen, der Auflagebügel mit Armen, Schaum und hochgebogenen Enden, die Halsgabel mit Schaumzinken; vier Glieder des 7-mm-Instrumentenkabels mit beiden Steckern auf den Gliedachsen, das Netzkabel in zwei Gliedern mit Gerätestecker und Netzstecker.")],
    100: [T("Own 1:100 model for a 15 mm feature: the sleeve, one front panel of grille and baffle, one block of chassis and faceplate, a knob row, both transformers by one repeat, power valves as prisms, the preamp valves as one bar, a simplified speaker, back panels, feet, a block handle; the corner protectors (2 mm proud) are merged into the sleeve.",
            "Eigenes 1:100-Modell für 15 mm kleinste Einzelheit: der Mantel, eine Frontplatte aus Bespannung und Schallwand, ein Block aus Chassis und Frontplatte, eine Knopfreihe, beide Transformatoren als eine Wiederholung, Endröhren als Prismen, die Vorstufenröhren als ein Riegel, ein vereinfachter Lautsprecher, Rückwände, Füße, ein Blockgriff; die 2 mm vorstehenden Eckschoner sind in den Mantel aufgegangen."),
          T("Guitar as body prism, guard sheet, neck with the board merged in, head, a tuner row and the bridge block; pickups and knobs only drawn. Stand as one mast, legs, cradle bar and arms, yoke and hub; cables unchanged at their true section.",
            "Gitarre als Korpusprisma, Schlagbrettplatte, Hals mit eingeschlossenem Griffbrett, Kopf, eine Mechanikreihe und der Stegblock; Tonabnehmer und Knöpfe nur gezeichnet. Ständer als ein Mast, Beine, Auflagebügel und Arme, Halsgabel und Nabe; Kabel unverändert im wahren Querschnitt.")],
    200: [T("Own 1:200 model for a 30 mm feature: the cabinet without feet, one 30 mm front board, the chassis block with the transformers, the two power valves as one prism, the speaker, two back panels 30 mm thick around the open slot, the handle block.",
            "Eigenes 1:200-Modell für 30 mm kleinste Einzelheit: das Gehäuse ohne Füße, ein 30 mm tiefes Frontbrett, der Chassisblock mit den Transformatoren, die zwei Endröhren als ein Prisma, der Lautsprecher, zwei 30 mm dicke Rückwände um den offenen Schlitz, der Griffblock."),
          T("Guitar as body prism, guard sheet, one neck-and-head prism at the true 22 mm thickness and the bridge; stand as mast, legs, cradle and yoke tubes; cables at true section with the plugs.",
            "Gitarre als Korpusprisma, Schlagbrettplatte, ein Prisma aus Hals und Kopf in der wahren Dicke von 22 mm und der Steg; Ständer als Mast, Beine, Rohre von Auflage und Halsgabel; Kabel im wahren Querschnitt mit den Steckern.")],
    500: [T("Own 1:500 model for a 75 mm feature: the combo as one closed block 620 x 502 x 260 mm with the grille and the control band as sheets on its front; valves and speaker as solids inside it in the thermal list only.",
            "Eigenes 1:500-Modell für 75 mm kleinste Einzelheit: der Combo als ein geschlossener Block 620 x 502 x 260 mm mit Bespannung und Bedienband als Platten auf der Front; Röhren und Lautsprecher als Körper in seinem Inneren nur in der thermischen Liste."),
          T("Guitar as body prism and one neck-and-head prism, stand as mast, three legs, cradle and yoke bars, the cables as four and two links whose first and last links run to the amplifier front and the guitar jack.",
            "Gitarre als Korpusprisma und ein Prisma aus Hals und Kopf, Ständer als Mast, drei Beine, Stäbe von Auflage und Halsgabel, die Kabel als vier und zwei Glieder, deren erstes und letztes bis zur Verstärkerfront und zur Gitarrenbuchse reichen.")],
}


def representation(scale):
    """🗺️ The authored drawing of one scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": geometry_of(scale), "decisions": SCALE_TEXT[scale]}


def volume(scale):
    """🧊️ The authored 3D model of one scale."""
    return {"status": "authored", "denominator": scale, "body": body_solids(scale), "thermal": thermal_solids(scale), "structure": structure_solids(scale), "decisions": VOLUME_TEXT[scale]}


def asset():
    """🎸️ The whole asset."""
    lo, hi = model_bbox(50)
    pose_list = poses()
    return {
        "id": "used-electric-guitar-and-combo",
        "category": "appliances",
        "title": T("Used Electric Guitar with Combo Amp", "Gebrauchte E-Gitarre mit Combo-Verstärker"),
        "summary": T(f"A second-hand double-cutaway solid-body electric guitar, 0.99 m with a 648 mm scale, 3.6 kg, standing in a tripod stand next to a 1x12 open-back 50 W valve combo of 620 x 502 x 260 mm and 22 kg, plugged in with an instrument cable: worn fretboard, nicked edges and forearm wear on the guitar, torn tolex and a scuffed top on the combo, which draws 180 W and warms its valves to about 200 C when played.",
                     f"Eine gebrauchte Doppel-Cutaway-Massivkorpus-E-Gitarre, 0,99 m mit 648 mm Mensur, 3,6 kg, steht im Dreibeinständer neben einem offenen 1x12-50-W-Röhrencombo von 620 x 502 x 260 mm und 22 kg, mit einem Instrumentenkabel eingesteckt: abgespieltes Griffbrett, Kantenmacken und Unterarmabrieb an der Gitarre, eingerissenes Tolex und eine abgeschabte Oberseite am Combo, der beim Spielen 180 W aufnimmt und seine Röhren auf etwa 200 C erwärmt."),
        "tags": ["used", "instrument", "music", "second-hand", "strings", "amplification", "stage", "electric-guitar", "valve-amplifier", "combo", "guitar-stand", "alder", "maple", "tolex", "electrical", "heat-source"],
        "provenance": {"author": "Semio staffage content (used instruments wave, row 09)", "method": "handcrafted", "created": "2026-11-22",
                       "references": [{"id": r[0], "citation": r[1]} for r in REFERENCES],
                       "notes": T("Types, not brands: a double-cutaway S-type solid body of the 1980s-2000s student and workshop class and a 1x12 50 W open-back valve combo with front-mounted chassis. Every number is in the reference table of the report (K/reports/used-electric-guitar-and-combo.md); dimensions are catalogue-typical, masses summed from section and density, the 180 W mains input and the heat split (valves 100 W, chassis and transformers 55 W, speaker 25 W) are authored estimates within the catalogue range. Nothing is measured.",
                                  "Typen, keine Marken: ein Doppel-Cutaway-Massivkorpus vom S-Typ der Schüler- und Werkstattklasse der 1980er- bis 2000er-Jahre und ein offener 1x12-50-W-Röhrencombo mit frontseitigem Chassis. Jede Zahl steht in der Referenztabelle des Berichts (K/reports/used-electric-guitar-and-combo.md); Maße katalogtypisch, Massen aus Querschnitt und Dichte summiert, die 180 W Netzaufnahme und die Wärmeaufteilung (Röhren 100 W, Chassis und Transformatoren 55 W, Lautsprecher 25 W) sind Schätzwerte im Katalogbereich. Nichts ist gemessen.")},
        "dimensions": {"heightM": round(hi[1] - lo[1], 4), "widthM": round(hi[0] - lo[0], 4), "depthM": round(hi[2] - lo[2], 4), "massKg": mass_total()},
        "materials": materials(),
        "rig": {"kind": "mechanical", "root": "floor", "bones": RIG_BONES},
        "anchors": anchors(),
        "representations": {str(s): representation(s) for s in SCALES},
        "symbols": [],
        "poses": pose_list,
        "clips": clips(pose_list),
        "thermal": {"environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
                    "passengers": [], "operating": [{"id": "power", "title": T("Mains power of the combo", "Netzleistung des Combos"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.22, "title": T("Standby, heaters only (about 40 W)", "Standby, nur Heizung (etwa 40 W)")}, {"v": 1, "title": T("Playing at rated input (180 W)", "Spielen mit Nennaufnahme (180 W)")}]}],
                    "regions": regions(),
                    "emitters": [{"id": "valve-plume", "kind": "plume", "title": T("Warm air rising from the valves out of the open back", "Warme Luft, die von den Röhren aus der offenen Rückseite steigt"), "anchor": "valve-vent", "directionDeg": 90, "diameterM": 0.15, "region": "valves", "source": "authored-estimate", "reference": "valve-amp-heat"}],
                    "assumptions": [T("Every watt of the 180 W mains input ends as heat in the room (the sound power of a few watts is neglected): valves 100 W, chassis with transformers 55 W, speaker voice coil 25 W at level 1; in standby (level 0.22) the same split scales to about 40 W.", "Jedes Watt der 180 W Netzaufnahme endet als Wärme im Raum (die wenigen Watt Schallleistung sind vernachlässigt): Röhren 100 W, Chassis mit Transformatoren 55 W, Schwingspule 25 W bei Stufe 1; im Standby (Stufe 0,22) skaliert dieselbe Aufteilung auf etwa 40 W."),
                                    T("The guitar is passive (0 W stated explicitly); in the played pose a fifth of its body and a seventh of its neck touch the player at 33 C.", "Die Gitarre ist passiv (0 W ausdrücklich); in der Spielpose berühren ein Fünftel ihres Korpus und ein Siebtel ihres Halses den Spieler mit 33 C."),
                                    T("Aged surfaces: tarnished nickel and dulled chrome radiate more than new plating (emissivity 0.18 and 0.12 against 0.05 to 0.08 when new), scuffed tolex and worn finish are matt (0.9).", "Gealterte Oberflächen: angelaufenes Nickel und mattes Chrom strahlen mehr als neue Beschichtung (Emissionsgrad 0,18 und 0,12 statt 0,05 bis 0,08 neu), abgeschabtes Tolex und abgegriffener Lack sind matt (0,9).")]},
        "structure": {"gravityMS2": GRAVITY, "dynamics": "quasi-static", "supports": ["stand-feet"],
                      "assumptions": [T("The set is solved as one rigid body on equal-stiffness supports: the tripod always, the combo feet when it stands, the hands at the handle and the neck when carried, the two strap buttons when played; the string pull of 461 N acts on bridge and nut as an equal and opposite pair that compresses and bends the neck.", "Das Ensemble wird als ein starrer Körper auf Lagern gleicher Steifigkeit gelöst: immer das Dreibein, die Combofüße, wenn er steht, die Hände an Griff und Hals beim Tragen, die zwei Gurtknöpfe beim Spielen; der Saitenzug von 461 N wirkt auf Steg und Sattel als gleich großes Gegenpaar, das den Hals drückt und biegt."),
                                      T("Masses: combo 21.9 kg (cabinet 6.8, baffle and back 2.7, tolex 0.6, speaker 3.5, chassis with transformers 7.5, valves 0.3, hardware 0.5), guitar 3.6 kg, stand 1.6 kg, cables 0.22 kg.", "Massen: Combo 21,9 kg (Gehäuse 6,8, Schallwand und Rückwand 2,7, Tolex 0,6, Lautsprecher 3,5, Chassis mit Transformatoren 7,5, Röhren 0,3, Beschläge 0,5), Gitarre 3,6 kg, Ständer 1,6 kg, Kabel 0,22 kg.")],
                      "clinical": False},
        "volumes": {str(s): volume(s) for s in SCALES},
        "presentation": {"defaultPose": "stored", "defaultClip": "valves-warm-up", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


def main():
    """💾️ Writes the asset document."""
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(OUT, "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False)
    a = doc["asset"]
    print("[DEBUG] dimensions", a["dimensions"], "bones", len(a["rig"]["bones"]))
    for s in SCALES:
        v = a["volumes"][str(s)]
        g = a["representations"][str(s)]["geometry"]
        print(f"[DEBUG] 1:{s} body {len(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])} shapes e {len(g['elevation'])} f {len(g['front'])} p {len(g['plan'])}")


if __name__ == "__main__":
    main()
