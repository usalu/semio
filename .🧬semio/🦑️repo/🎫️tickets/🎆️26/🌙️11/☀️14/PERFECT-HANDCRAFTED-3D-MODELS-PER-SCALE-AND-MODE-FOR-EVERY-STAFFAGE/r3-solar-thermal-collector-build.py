"""☀️ Round 3 author of machinery/solar-thermal-collector: every solid, bone, anchor and 2D shape stated by hand from one set of construction dimensions.

Usage: python3 r3-solar-thermal-collector-build.py            writes T/volumes/machinery/solar-thermal-collector.json and the candidate document 🗑️generated/r3-solar-thermal-collector/🔣️.json
       python3 r3-solar-thermal-collector-build.py --apply    re-reads the catalog file and replaces the changed members and `volumes` in one write
"""
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
R = os.path.abspath(os.path.join(T, "..", "..", "..", "..", "..", "..", ".."))
CATALOG = os.path.join(R, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🏗️machinery", "solar-thermal-collector", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "machinery", "solar-thermal-collector.json")
MEMBERS = ["provenance", "rig", "anchors", "representations", "thermal", "volumes"]

C, S = math.cos(math.radians(35)), math.sin(math.radians(35))
TAN = S / C
LENGTH, WIDTH, DEPTH = 2.0, 1.0, 0.1
LOW = 0.18
HALF = (LENGTH * C + DEPTH * S) / 2
U0 = 0.36
XR = round(-HALF + U0 * C + 0.75, 5)
YH = round(LOW + LENGTH * S - U0 * S, 5)
XF = round(XR + (LENGTH - 2 * U0) * C, 5)
YF = round(YH - (LENGTH - 2 * U0) * S, 5)
ALU, GLASS, COAT, CU, WOOL, STEEL = "anodised-aluminium", "solar-glass", "selective-absorber", "copper", "mineral-wool", "galvanised-steel"
GLASS_AT = (0.005, 0.0969)
HEADER_AT = (0.045, 0.0648)
SHEET_AT = (0.015, 0.0051)
PITCH = [-0.4 + 0.1 * i for i in range(9)]
Q35 = [0.0, 0.0, round(-math.sin(math.radians(17.5)), 5), round(math.cos(math.radians(17.5)), 5)]


def n(value):
    return round(value + 0.0, 5)


def hu(t):
    return n(t - U0)


def back(x):
    return n(YH - (x - XR) * TAN)


def world(t, v):
    return n(XR - 0.75 + (t - U0) * C + v * S), n(YH - (t - U0) * S + v * C)


def text(en, de):
    return {"en": en, "de": de}


def solid(sid, kind, bone, material, role, tone, **rest):
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **rest}


def box(sid, bone, material, tone, size, at, role="body", rotation=None, repeat=None, **rest):
    transform = {"translation": [n(a) for a in at]}
    if rotation:
        transform["rotation"] = rotation
    out = solid(sid, "box", bone, material, role, tone, size=[n(a) for a in size], transform=transform, **rest)
    if repeat:
        out["repeat"] = {"count": repeat[0], "translation": [n(a) for a in repeat[1]]}
    return out


def extrude(sid, bone, material, tone, profile, plane, start, end, role="body", holes=None, repeat=None, **rest):
    out = solid(sid, "extrude", bone, material, role, tone, profile=[[n(a), n(b)] for a, b in profile], plane=plane, **rest)
    if holes:
        out["holes"] = [[[n(a), n(b)] for a, b in hole] for hole in holes]
    out["from"], out["to"] = n(start), n(end)
    if repeat:
        out["repeat"] = {"count": repeat[0], "translation": [n(a) for a in repeat[1]]}
    return out


def revolve(sid, bone, material, tone, profile, axis, origin, role="body", repeat=None, **rest):
    out = solid(sid, "revolve", bone, material, role, tone, profile=[[n(a), n(b)] for a, b in profile], axis=axis, origin=[n(a) for a in origin], **rest)
    if repeat:
        out["repeat"] = {"count": repeat[0], "translation": [n(a) for a in repeat[1]]}
    return out


def ring(t0, t1, half):
    return [(-half, hu(t0)), (half, hu(t0)), (half, hu(t1)), (-half, hu(t1))]


def along(points):
    return [(hu(t), v) for t, v in points]


def lateral(points, side):
    return [(v, side * w) for v, w in points]


LEG = 0.025
RAIL = [(0, 0.425), (0, 0.45), (LEG, 0.45), (LEG, 0.448), (0.002, 0.448), (0.002, 0.425)]
KNEE = [(-0.0125, 0.421), (-0.0125, 0.446), (0.0125, 0.446), (0.0125, 0.444), (-0.0105, 0.444), (-0.0105, 0.421)]
POST_REAR_TOP, POST_FRONT_TOP = 1.1, 0.365
CLEAT_REAR_LOW, CLEAT_FRONT_LOW = 1.03, 0.3
KNEE_FOOT, KNEE_HEAD = (XR + 0.32, 0.0125), (XR + 0.003, 0.33)
KNEE_LENGTH = math.hypot(KNEE_HEAD[0] - KNEE_FOOT[0], KNEE_HEAD[1] - KNEE_FOOT[1])
KNEE_ANGLE = math.atan2(KNEE_HEAD[1] - KNEE_FOOT[1], KNEE_HEAD[0] - KNEE_FOOT[0])
KNEE_Q = [0.0, 0.0, round(math.sin(KNEE_ANGLE / 2), 5), round(math.cos(KNEE_ANGLE / 2), 5)]
BRACE_LOW, BRACE_HIGH, BRACE_HALF = 0.21, 1.0, 0.431
BRACE_LENGTH = math.hypot(2 * BRACE_HALF, BRACE_HIGH - BRACE_LOW)
BRACE_ANGLE = math.atan2(2 * BRACE_HALF, BRACE_HIGH - BRACE_LOW)
BRACE_Q = [round(math.sin(BRACE_ANGLE / 2), 5), 0.0, 0.0, round(math.cos(BRACE_ANGLE / 2), 5)]
BRACE_Q_MIRROR = [-BRACE_Q[0], 0.0, 0.0, BRACE_Q[3]]


def angle_post(x, side):
    return [(side * 0.448, x - 0.0125), (side * 0.448, x + 0.0125), (side * 0.446, x + 0.0125), (side * 0.446, x - 0.0105), (side * 0.423, x - 0.0105), (side * 0.423, x - 0.0125)]


def strut(x, half, foot):
    return [(x - half, foot), (x + half, foot), (x + half, back(x + half)), (x - half, back(x - half))]


def cleat(x, low, thickness, outer):
    drop = thickness / C
    x0, x1, x2 = x - 0.0125 - outer, x - 0.0125, x + 0.065
    return [(x0, low), (x1, low), (x1, back(x1) - drop), (x2, back(x2) - drop), (x2, back(x2)), (x0, back(x0))]


def knee_outline(half, reach):
    ux, uy = math.cos(KNEE_ANGLE), math.sin(KNEE_ANGLE)
    ends = [(KNEE_FOOT[0] - reach * ux, KNEE_FOOT[1] - reach * uy), (KNEE_HEAD[0] + reach * ux, KNEE_HEAD[1] + reach * uy)]
    return [(ends[0][0] + half * uy, ends[0][1] - half * ux), (ends[1][0] + half * uy, ends[1][1] - half * ux), (ends[1][0] - half * uy, ends[1][1] + half * ux), (ends[0][0] - half * uy, ends[0][1] + half * ux)]


def side_frame(half, rail):
    return [(0, 0), (1.5, 0), (1.5, rail), (XF + half, rail), (XF + half, back(XF + half)), (XF - half, back(XF - half)), (XF - half, rail), (XR + half, rail), (XR + half, back(XR + half)), (XR - half, back(XR - half)), (XR - half, rail), (0, rail)]


def braces(rear_u, front_u):
    size = [0.002, BRACE_LENGTH + 0.02, 0.02]
    middle = (BRACE_LOW + BRACE_HIGH) / 2
    return [
        box("brace-rising-left", "stand", STEEL, 0.5, size, [rear_u, middle, 0], rotation=BRACE_Q, slender=True),
        box("brace-rising-right", "stand", STEEL, 0.5, size, [front_u, middle, 0], rotation=BRACE_Q_MIRROR, slender=True),
    ]


def pin(sid, axis, origin, repeat=None):
    return revolve(sid, "stand", STEEL, 0.4, [(0, 0.0065), (0.016, 0.0065)], axis, origin, role="detail", repeat=repeat)


def harp_section():
    out = [(0.00025, -0.455), (0.00025, 0.455), (-0.00025, 0.455)]
    reach = math.acos((0.005 - 0.00015) / 0.005)
    for w in reversed(PITCH):
        for k in range(13):
            a = math.pi / 2 - reach - (2 * math.pi - 2 * reach) * k / 12
            out.append((-0.0051 + 0.005 * math.sin(a), w + 0.005 * math.cos(a)))
    return out + [(-0.00025, -0.455)]


def body50():
    wide, across = [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]
    stand = [
        extrude("rail-left", "stand", STEEL, 0.5, lateral(RAIL, 1), "yz", 0, 1.5),
        extrude("rail-right", "stand", STEEL, 0.5, lateral(RAIL, -1), "yz", 0, 1.5),
        extrude("post-rear-left", "stand", STEEL, 0.5, angle_post(XR, 1), "zx", 0.002, POST_REAR_TOP),
        extrude("post-rear-right", "stand", STEEL, 0.5, angle_post(XR, -1), "zx", 0.002, POST_REAR_TOP),
        extrude("post-front-left", "stand", STEEL, 0.5, angle_post(XF, 1), "zx", 0.002, POST_FRONT_TOP),
        extrude("post-front-right", "stand", STEEL, 0.5, angle_post(XF, -1), "zx", 0.002, POST_FRONT_TOP),
        extrude("knee-left", "stand", STEEL, 0.5, lateral(KNEE, 1), "yz", -0.003, KNEE_LENGTH + 0.003, transform={"translation": [n(KNEE_FOOT[0]), KNEE_FOOT[1], 0.0], "rotation": KNEE_Q}),
        extrude("knee-right", "stand", STEEL, 0.5, lateral(KNEE, -1), "yz", -0.003, KNEE_LENGTH + 0.003, transform={"translation": [n(KNEE_FOOT[0]), KNEE_FOOT[1], 0.0], "rotation": KNEE_Q}),
        extrude("cleat-rear", "stand", STEEL, 0.5, cleat(XR, CLEAT_REAR_LOW, 0.003, 0.003), "xy", -0.448, -0.423, repeat=(2, [0, 0, 0.871])),
        extrude("cleat-front", "stand", STEEL, 0.5, cleat(XF, CLEAT_FRONT_LOW, 0.003, 0.003), "xy", -0.448, -0.423, repeat=(2, [0, 0, 0.871])),
        *braces(XR - 0.0135, XR - 0.0095),
        pin("bolt-foot-right", wide, [XR, 0.0125, -0.4555], repeat=(2, [XF - XR, 0, 0])),
        pin("bolt-foot-left", wide, [XR, 0.0125, 0.4395], repeat=(2, [XF - XR, 0, 0])),
        pin("bolt-knee-right", wide, [KNEE_FOOT[0], KNEE_FOOT[1], -0.4555], repeat=(2, [KNEE_HEAD[0] - KNEE_FOOT[0], KNEE_HEAD[1] - KNEE_FOOT[1], 0])),
        pin("bolt-knee-left", wide, [KNEE_FOOT[0], KNEE_FOOT[1], 0.4395], repeat=(2, [KNEE_HEAD[0] - KNEE_FOOT[0], KNEE_HEAD[1] - KNEE_FOOT[1], 0])),
        pin("bolt-cleat-rear-right", across, [XR - 0.0215, CLEAT_REAR_LOW + 0.015, -0.4355], repeat=(2, [0, 0.028, 0])),
        pin("bolt-cleat-rear-left", across, [XR - 0.0215, CLEAT_REAR_LOW + 0.015, 0.4355], repeat=(2, [0, 0.028, 0])),
        pin("bolt-cleat-front-right", across, [XF - 0.0215, CLEAT_FRONT_LOW + 0.015, -0.4355], repeat=(2, [0, 0.028, 0])),
        pin("bolt-cleat-front-left", across, [XF - 0.0215, CLEAT_FRONT_LOW + 0.015, 0.4355], repeat=(2, [0, 0.028, 0])),
        pin("bolt-brace-rising-left", across, [XR - 0.0195, BRACE_LOW, -BRACE_HALF], repeat=(2, [0, BRACE_HIGH - BRACE_LOW, 2 * BRACE_HALF])),
        pin("bolt-brace-rising-right", across, [XR - 0.0195, BRACE_LOW, BRACE_HALF], repeat=(2, [0, BRACE_HIGH - BRACE_LOW, -2 * BRACE_HALF])),
    ]
    tray = [(0.0015, 0), (1.9985, 0), (1.9985, 0.0953), (1.9835, 0.0953), (1.9835, 0.0938), (1.997, 0.0938), (1.997, 0.0015), (0.003, 0.0015), (0.003, 0.0938), (0.0165, 0.0938), (0.0165, 0.0953), (0.0015, 0.0953)]
    side = [(0, 0.497), (0, 0.4985), (0.0953, 0.4985), (0.0953, 0.4835), (0.0938, 0.4835), (0.0938, 0.497)]
    cover = [(0.0865, 0.4985), (0.0865, 0.5), (0.1, 0.5), (0.1, 0.48), (0.0985, 0.48), (0.0985, 0.4985)]
    cover_upper = [(0, 0.0865), (0.0015, 0.0865), (0.0015, 0.0985), (0.02, 0.0985), (0.02, 0.1), (0, 0.1)]
    cover_lower = [(2, 0.0865), (2, 0.1), (1.98, 0.1), (1.98, 0.0985), (1.9985, 0.0985), (1.9985, 0.0865)]
    wool = [(0.003, 0.0015), (1.997, 0.0015), (1.997, 0.052), (1.94, 0.052), (1.94, 0.0598), (0.06, 0.0598), (0.06, 0.052), (0.003, 0.052)]
    collar = [(0, 0.011), (0, 0.018), (0.008, 0.018), (0.008, 0.011), (0, 0.011)]
    collector = [
        extrude("tray", "housing", ALU, 0.35, along(tray), "xy", -0.497, 0.497),
        extrude("tray-side-left", "housing", ALU, 0.35, lateral(side, 1), "yz", hu(0.0015), hu(1.9985)),
        extrude("tray-side-right", "housing", ALU, 0.35, lateral(side, -1), "yz", hu(0.0015), hu(1.9985)),
        extrude("cover-side-left", "housing", ALU, 0.35, lateral(cover, 1), "yz", hu(0), hu(2)),
        extrude("cover-side-right", "housing", ALU, 0.35, lateral(cover, -1), "yz", hu(0), hu(2)),
        extrude("cover-end-upper", "housing", ALU, 0.35, along(cover_upper), "xy", -0.5, 0.5),
        extrude("cover-end-lower", "housing", ALU, 0.35, along(cover_lower), "xy", -0.5, 0.5),
        extrude("insulation", "housing", WOOL, 0.8, along(wool), "xy", -0.497, 0.497),
        box("glass", "glass", GLASS, 0.92, [1.99, 0.0032, 0.99], [0.995, 0, 0], role="glazing"),
        extrude("absorber", "absorber", COAT, 0.2, harp_section(), "yz", 0, 1.88),
        revolve("riser-end-upper", "absorber", CU, 0.55, [(0, 0.005), (0.015, 0.005)], across, [-0.015, -0.0051, -0.4], role="detail", repeat=(9, [0, 0, 0.1])),
        revolve("riser-end-lower", "absorber", CU, 0.55, [(0, 0.005), (0.015, 0.005)], across, [1.88, -0.0051, -0.4], role="detail", repeat=(9, [0, 0, 0.1])),
        revolve("header-upper", "headers", CU, 0.4, [(0, 0.011), (1.0, 0.011)], wide, [0, 0, -0.5]),
        revolve("header-lower", "headers", CU, 0.4, [(0, 0.011), (1.0, 0.011)], wide, [1.91, 0, -0.5]),
        revolve("collar-upper", "headers", CU, 0.4, collar, wide, [0, 0, -0.5], role="detail", repeat=(2, [0, 0, 0.992])),
        revolve("collar-lower", "headers", CU, 0.4, collar, wide, [1.91, 0, -0.5], role="detail", repeat=(2, [0, 0, 0.992])),
    ]
    return stand + collector


REGION_OF = {
    "stand": ["rail", "post", "knee", "cleat", "brace", "bolt", "side-frame"],
    "housing": ["tray", "cover-", "insulation", "collector"],
    "glazing": ["glass"],
    "absorber": ["absorber", "riser"],
    "header-upper": ["header-upper", "collar-upper"],
    "header-lower": ["header-lower", "collar-lower"],
}


def region_of(sid):
    for region in ("stand", "header-upper", "header-lower", "absorber", "housing", "glazing"):
        if any(sid.startswith(prefix) for prefix in REGION_OF[region]):
            return region
    raise SystemExit("no region for " + sid)


def outlet(low, high):
    return box("th-absorber-plume-outlet", "glass", GLASS, 0.9, [0.19, high - low, 0.5], [0.11, (low + high) / 2 - GLASS_AT[1], 0], region="absorber", tags=["absorber-plume"])


def thermal_of(body, low, high):
    return [{"id": "th-" + s["id"], "of": s["id"], "region": region_of(s["id"])} for s in body] + [outlet(low, high)]


def ref(sid, member, part="member"):
    return {"id": "st-" + sid, "of": sid, "member": member, "part": part}


def mark(out, member, part, **rest):
    out.update({"member": member, "part": part, **rest})
    return out


def disc(sid, bone, material, tone, at, radius, thickness):
    return revolve(sid, bone, material, tone, [(0, radius), (thickness, radius)], [0.0, 1.0, 0.0], at)


def sleeves(outer):
    loop = [(0, 0.011), (0, outer), (0.08, outer), (0.08, 0.011), (0, 0.011)]
    return mark(revolve("mass-headers", "headers", CU, 0.4, loop, [0.0, 0.0, 1.0], [0, 0, -0.04], repeat=(2, [1.91, 0, 0])), "headers", "mass")


def stand_lumps():
    return [
        mark(box("mass-stand-rear", "stand", STEEL, 0.5, [LEG, 0.14, LEG], [XR, 0.6, -0.4375], repeat=(2, [0, 0, 0.875])), "stand", "mass"),
        mark(box("mass-stand-rail", "stand", STEEL, 0.5, [0.14, LEG, LEG], [0.66, LEG / 2, -0.4375], repeat=(2, [0, 0, 0.875])), "stand", "mass"),
        mark(box("mass-stand-front", "stand", STEEL, 0.5, [LEG, 0.06, LEG], [XF, 0.18, -0.4375], repeat=(2, [0, 0, 0.875])), "stand", "mass"),
    ]


def pad(height, width, centre):
    return mark(box("base-contact-pad", "stand", STEEL, 0.5, [1.5, height, width], [0.75, height / 2, -centre], repeat=(2, [0, 0, 2 * centre])), "stand", "support", anchor="base-contact")


def masses(puck, plate_low, plate, sleeve):
    return [
        mark(disc("mass-housing", "housing", ALU, 0.35, [hu(1.0), 0.008, 0], 0.09, 0.03), "housing", "mass"),
        mark(disc("mass-glass", "glass", GLASS, 0.92, [0.995, -puck / 2, 0], 0.13, puck), "glass", "mass"),
        mark(disc("mass-absorber", "absorber", COAT, 0.2, [0.94, plate_low, 0], 0.1, plate), "absorber", "mass"),
        sleeves(sleeve),
        *stand_lumps(),
    ]


def structure50():
    members = [ref(s, "stand") for s in ("rail-left", "rail-right", "post-rear-left", "post-rear-right", "post-front-left", "post-front-right", "knee-left", "knee-right", "brace-rising-left", "brace-rising-right")]
    upper = [(0.0015, 0), (0.003, 0), (0.003, 0.0938), (0.0165, 0.0938), (0.0165, 0.0953), (0.0015, 0.0953)]
    lower = [(1.9985, 0), (1.9985, 0.0953), (1.9835, 0.0953), (1.9835, 0.0938), (1.997, 0.0938), (1.997, 0)]
    members += [ref("tray-side-left", "housing"), ref("tray-side-right", "housing")]
    members += [mark(extrude("end-rail-upper", "housing", ALU, 0.35, along(upper), "xy", -0.497, 0.497), "housing", "member"), mark(extrude("end-rail-lower", "housing", ALU, 0.35, along(lower), "xy", -0.497, 0.497), "housing", "member")]
    members += [ref("glass", "glass"), ref("absorber", "absorber"), ref("header-upper", "headers"), ref("header-lower", "headers")]
    joints = [ref("cleat-rear", "housing", "joint"), ref("cleat-front", "housing", "joint"), ref("bolt-foot-right", "stand", "joint"), ref("bolt-foot-left", "stand", "joint"), ref("bolt-knee-right", "stand", "joint"), ref("bolt-knee-left", "stand", "joint")]
    joints += [ref("riser-end-upper", "absorber", "joint"), ref("riser-end-lower", "absorber", "joint"), ref("collar-upper", "headers", "joint"), ref("collar-lower", "headers", "joint")]
    return members + joints + masses(0.0075, -0.0101, 0.0101, 0.016) + [pad(0.0075, 0.029, 0.4375)]


def knee_bar(size):
    middle = [(KNEE_FOOT[0] + KNEE_HEAD[0]) / 2, (KNEE_FOOT[1] + KNEE_HEAD[1]) / 2, -0.4375]
    return box("knee", "stand", STEEL, 0.5, [KNEE_LENGTH, size, LEG], middle, rotation=KNEE_Q, repeat=(2, [0, 0, 0.875]))


def body100():
    tray = [(0, 0), (2, 0), (2, 0.1), (1.98, 0.1), (1.98, 0.0985), (1.9985, 0.0985), (1.9985, 0.0015), (0.0015, 0.0015), (0.0015, 0.0985), (0.02, 0.0985), (0.02, 0.1), (0, 0.1)]
    side = [(0.0015, 0.4985), (0.0015, 0.5), (0.1, 0.5), (0.1, 0.48), (0.0985, 0.48), (0.0985, 0.4985)]
    return [
        box("rail", "stand", STEEL, 0.5, [1.5, LEG, LEG], [0.75, LEG / 2, -0.4375], repeat=(2, [0, 0, 0.875])),
        extrude("post-rear", "stand", STEEL, 0.5, strut(XR, 0.0125, LEG), "xy", -0.45, -0.425, repeat=(2, [0, 0, 0.875])),
        extrude("post-front", "stand", STEEL, 0.5, strut(XF, 0.0125, LEG), "xy", -0.45, -0.425, repeat=(2, [0, 0, 0.875])),
        knee_bar(LEG),
        *braces(XR - 0.0135, XR + 0.0135),
        extrude("tray", "housing", ALU, 0.35, along(tray), "xy", -0.4985, 0.4985),
        extrude("tray-side-left", "housing", ALU, 0.35, lateral(side, 1), "yz", hu(0), hu(2)),
        extrude("tray-side-right", "housing", ALU, 0.35, lateral(side, -1), "yz", hu(0), hu(2)),
        box("insulation", "housing", WOOL, 0.8, [1.997, 0.0523, 0.997], [hu(1.0), 0.02765, 0]),
        box("glass", "glass", GLASS, 0.92, [1.997, 0.0032, 0.997], [0.995, 0, 0], role="glazing"),
        box("absorber-plate", "absorber", COAT, 0.2, [1.88, 0.0164, 0.91], [0.94, -0.00795, 0]),
        revolve("header-upper", "headers", CU, 0.4, [(0, 0.011), (0.997, 0.011)], [0.0, 0.0, 1.0], [0, 0, -0.4985]),
        revolve("header-lower", "headers", CU, 0.4, [(0, 0.011), (0.997, 0.011)], [0.0, 0.0, 1.0], [1.91, 0, -0.4985]),
    ]


def node(sid, x, y):
    return mark(box(sid, "stand", STEEL, 0.5, [0.035, 0.035, 0.035], [x, y, -0.4375], repeat=(2, [0, 0, 0.875])), "stand", "joint")


def structure100():
    members = [ref(s, "stand") for s in ("rail", "post-rear", "post-front", "knee", "brace-rising-left", "brace-rising-right")]
    members += [mark(extrude("frame", "housing", ALU, 0.35, ring(0, 2, 0.5), "zx", 0, 0.0953, holes=[ring(0.0015, 1.9985, 0.4985)]), "housing", "member")]
    members += [ref("glass", "glass"), ref("header-upper", "headers"), ref("header-lower", "headers")]
    members += [mark(revolve("riser", "absorber", CU, 0.55, [(0, 0.005), (1.91, 0.005)], [1.0, 0.0, 0.0], [-0.015, -0.0051, -0.4], slender=True, repeat=(9, [0, 0, 0.1])), "absorber", "member")]
    joints = [node("joint-foot-rear", XR, LEG), node("joint-foot-front", XF, LEG), node("joint-cleat-rear", XR, back(XR) - 0.0175), node("joint-cleat-front", XF, back(XF) - 0.0175)]
    return members + joints + masses(0.015, -0.01615, 0.0164, 0.018) + [pad(0.015, 0.029, 0.4375)]


def body200():
    tray = [(0, 0), (2, 0), (2, 0.1), (1.97, 0.1), (1.97, 0.04), (0.03, 0.04), (0.03, 0.1), (0, 0.1)]
    return [
        extrude("side-frame", "stand", STEEL, 0.5, side_frame(0.015, 0.03), "xy", -0.45, -0.42, repeat=(2, [0, 0, 0.87])),
        box("knee", "stand", STEEL, 0.5, [KNEE_LENGTH, 0.03, 0.03], [(KNEE_FOOT[0] + KNEE_HEAD[0]) / 2, (KNEE_FOOT[1] + KNEE_HEAD[1]) / 2, -0.435], rotation=KNEE_Q, repeat=(2, [0, 0, 0.87])),
        extrude("tray", "housing", ALU, 0.35, along(tray), "xy", -0.47, 0.47),
        box("tray-side", "housing", ALU, 0.35, [2.0, 0.1, 0.03], [hu(1.0), 0.05, -0.485], repeat=(2, [0, 0, 0.97])),
        box("glass", "glass", GLASS, 0.92, [1.94, 0.03, 0.94], [0.995, -0.0119, 0]),
        box("absorber", "absorber", COAT, 0.2, [1.86, 0.03, 0.91], [0.94, -0.0149, 0]),
        revolve("header-upper", "headers", CU, 0.4, [(0, 0.015), (0.94, 0.015)], [0.0, 0.0, 1.0], [0.01, -0.0098, -0.47]),
        revolve("header-lower", "headers", CU, 0.4, [(0, 0.015), (0.94, 0.015)], [0.0, 0.0, 1.0], [1.9, -0.0098, -0.47]),
    ]


def frame_ring(inset):
    return mark(extrude("frame-ring", "housing", ALU, 0.35, ring(0, 2, 0.5), "zx", 0, 0.1, holes=[ring(inset, 2 - inset, 0.5 - inset)]), "housing", "member")


def structure200():
    out = [ref("side-frame", "stand"), ref("knee", "stand")]
    out += [mark(s, "stand", "member") for s in braces(XR - 0.016, XR + 0.016)]
    out += [frame_ring(0.03)]
    out += [ref("glass", "glass"), ref("header-upper", "headers"), ref("header-lower", "headers")]
    out += [mark(revolve("riser", "absorber", CU, 0.55, [(0, 0.005), (1.89, 0.005)], [1.0, 0.0, 0.0], [-0.005, -0.0149, -0.4], slender=True, repeat=(9, [0, 0, 0.1])), "absorber", "member")]
    out += [mark(box("mass-collector", "housing", ALU, 0.35, [0.3, 0.03, 0.3], [hu(1.0), 0.055, 0]), "housing", "mass")]
    out += [mark(box("mass-stand", "stand", STEEL, 0.5, [0.036, 0.2, 0.036], [XR, 0.6, -0.435], repeat=(2, [0, 0, 0.87])), "stand", "mass"), pad(0.034, 0.034, 0.435)]
    return out


def body500():
    return [
        extrude("side-frame", "stand", STEEL, 0.5, side_frame(0.0375, 0.075), "xy", -0.45, -0.375, repeat=(2, [0, 0, 0.825])),
        box("collector", "housing", ALU, 0.4, [2.0, 0.1, 1.0], [hu(1.0), 0.05, 0]),
    ]


def thermal500():
    return [
        {"id": "th-side-frame", "of": "side-frame", "region": "stand"},
        box("th-housing", "housing", ALU, 0.4, [2.0, 0.0953, 1.0], [hu(1.0), 0.04765, 0], region="housing", sectionFaces=["+v"]),
        box("th-glass", "glass", GLASS, 0.92, [2.0, 0.0032, 1.0], [0.995, 0, 0], region="glazing", layer="glazing-skin", sectionFaces=["-v"]),
        box("th-absorber", "absorber", COAT, 0.2, [1.88, 0.0005, 0.91], [0.94, 0, 0], region="absorber", layer="absorber-skin"),
        revolve("th-header-upper", "headers", CU, 0.4, [(0, 0.011), (0.997, 0.011)], [0.0, 0.0, 1.0], [0, 0, -0.4985], region="header-upper", slender=True),
        revolve("th-header-lower", "headers", CU, 0.4, [(0, 0.011), (0.997, 0.011)], [0.0, 0.0, 1.0], [1.91, 0, -0.4985], region="header-lower", slender=True),
        outlet(0.027, 0.102),
    ]


def structure500():
    return [
        ref("side-frame", "stand"),
        frame_ring(0.075),
        mark(box("glass-pane", "glass", GLASS, 0.92, [1.85, 0.075, 0.85], [0.995, -0.0344, 0]), "glass", "member"),
        mark(revolve("riser", "absorber", CU, 0.55, [(0, 0.005), (1.91, 0.005)], [1.0, 0.0, 0.0], [-0.015, -0.0051, -0.4], slender=True, repeat=(3, [0, 0, 0.4])), "absorber", "member"),
        mark(revolve("header-pipe-upper", "headers", CU, 0.4, [(0, 0.011), (0.85, 0.011)], [0.0, 0.0, 1.0], [0, 0, -0.425], slender=True), "headers", "member"),
        mark(revolve("header-pipe-lower", "headers", CU, 0.4, [(0, 0.011), (0.85, 0.011)], [0.0, 0.0, 1.0], [1.91, 0, -0.425], slender=True), "headers", "member"),
        mark(box("mass-collector", "housing", ALU, 0.4, [0.3, 0.075, 0.3], [hu(1.0), 0.05, 0]), "housing", "mass"),
        mark(box("mass-stand", "stand", STEEL, 0.5, [0.085, 0.2, 0.085], [XR, 0.6, -0.4125], repeat=(2, [0, 0, 0.825])), "stand", "mass"),
        pad(0.08, 0.085, 0.4125),
    ]


DECISIONS = {
    "50": [
        text(
            "Body, built like the real collector in the frame of the housing bone (u down the slope from the upper cleat, v out of the aperture, w across): a folded 1.5 mm aluminium tray (floor and end walls in one section, two side walls, a 15 mm glass ledge all round), a clamping cover angle on each edge (20 mm top leg, 12 mm skirt), 58 mm mineral wool with a recessed channel under each header, a 3.2 mm solar glass pane of 1.99 m by 0.99 m on the ledge, and behind a 25 mm air gap the harp: a 0.5 mm selective sheet of 1.88 m by 0.91 m welded along nine 10 mm copper risers at 100 mm pitch (one fin-and-tube section) that end in two 22 mm headers; the headers end flush with the frame in four connection sockets. The stand is cold-formed galvanised 25 x 25 x 2 mm angle: two base rails, a rear and a front post per rail, a knee brace from each rear post to its rail, a folded 3 mm cleat (one leg on the post, one under the tray back) at each post top, two crossed 20 x 2 mm flat braces between the rear posts and M8 bolts at feet, knee braces, cleats and cross braces. Every sheet keeps its real thickness.",
            "Körper, gebaut wie der echte Kollektor im Bezugssystem des Wannenknochens (u die Schräge hinab ab der oberen Konsole, v aus der Apertur heraus, w quer): eine gekantete 1,5-mm-Aluminiumwanne (Boden und Stirnwände in einem Profil, zwei Seitenwände, umlaufend 15 mm Glasauflage), an jeder Kante ein klemmender Abdeckwinkel (20 mm Deckschenkel, 12 mm Schürze), 58 mm Mineralwolle mit einer vertieften Rinne unter jedem Sammler, eine 3,2 mm dicke Solarglasscheibe von 1,99 m mal 0,99 m auf der Auflage und hinter 25 mm Luftspalt die Harfe: ein 0,5 mm dickes selektives Blech von 1,88 m mal 0,91 m verschweißt mit neun 10-mm-Kupfersteigrohren im Abstand von 100 mm (ein Blech-Rohr-Profil), die in zwei 22-mm-Sammlern enden; die Sammler enden bündig mit dem Rahmen in vier Anschlussmuffen. Das Gestell besteht aus kaltgeformtem verzinktem Winkel 25 x 25 x 2 mm: zwei Fußschienen, je Schiene ein hinterer und ein vorderer Pfosten, je hinterem Pfosten eine Kopfstrebe zur Schiene, an jedem Pfostenkopf eine gekantete 3-mm-Konsole (ein Schenkel am Pfosten, einer unter dem Wannenrücken), zwei gekreuzte Flachstahlstreben 20 x 2 mm zwischen den hinteren Pfosten und M8-Schrauben an Füßen, Kopfstreben, Konsolen und Kreuzstreben. Jedes Blech behält seine wahre Dicke.",
        ),
        text(
            "Thermal: every region is the set of body solids that carry its surface (housing: tray, cover angles and mineral wool; glazing: the pane; absorber: sheet and risers; one region per header with its sockets; stand: every steel part); each region has one layer, so no nested shells exist. The plume outlet is a 0.19 m by 0.5 m patch of one smallest feature in the glass below the upper cover leg, where the warm boundary layer leaves the pane. Structure: the real load path. The pane rests on the ledge of four frame rails (the two side walls and two end rails; the floor sheet and the wool carry nothing and are absent), the harp spans from header to header, the headers hang in their sockets; four cleats pass the collector to the posts, the knee braces triangulate the rear posts with the rails, the crossed flats brace the rear posts sideways, and two contact strips under the rails carry the anchor base-contact. Masses are discs in the wool, the pane and the riser plane, two sleeves on the headers and three lumps per side frame placed so that their centroid is the centre of mass of the stand.",
            "Thermik: Jede Region ist die Menge der Körper, die ihre Oberfläche tragen (housing: Wanne, Abdeckwinkel und Mineralwolle; glazing: die Scheibe; absorber: Blech und Steigrohre; je Sammler eine Region samt Muffen; stand: jedes Stahlteil); jede Region hat eine Schicht, daher gibt es keine verschachtelten Schalen. Der Fahnenauslass ist ein Feld von 0,19 m mal 0,5 m und einer kleinsten Einzelheit Dicke im Glas unter dem oberen Deckschenkel, wo die warme Grenzschicht die Scheibe verlässt. Tragwerk: der echte Lastpfad. Die Scheibe liegt auf der Auflage von vier Rahmenholmen (die beiden Seitenwände und zwei Stirnholme; Bodenblech und Wolle tragen nichts und fehlen), die Harfe spannt von Sammler zu Sammler, die Sammler hängen in ihren Muffen; vier Konsolen geben den Kollektor an die Pfosten ab, die Kopfstreben bilden mit hinteren Pfosten und Schienen Dreiecke, die gekreuzten Flachstähle steifen die hinteren Pfosten seitlich aus, und zwei Kontaktstreifen unter den Schienen tragen den Anker base-contact. Massen sind Scheiben in der Wolle, in der Glasscheibe und in der Steigrohrebene, zwei Hülsen auf den Sammlern und je Seitenrahmen drei Klumpen, deren Schwerpunkt der Massenmittelpunkt des Gestells ist.",
        ),
    ],
    "100": [
        text(
            "Authored for the 15 mm feature: tray, ledge and cover angle merge into one 1.5 mm shell with 20 mm lips (one section across the width and two side walls), the pane reaches from wall to wall under the lips, the wool is one flat slab of 52 mm, sheet, risers and their bedding are one absorber plate of 16.5 mm, the headers stay 22 mm pipes between the walls (sockets, cleats and bolts are below the feature). The stand is solid 25 mm bars: per side a rail, two posts cut to the slope of the tray back and the knee brace; the crossed flats stay as slender parts at their true 20 x 2 mm.",
            "Für die Einzelheit von 15 mm verfasst: Wanne, Auflage und Abdeckwinkel gehen in einer 1,5 mm dicken Schale mit 20-mm-Lippen auf (ein Profil über die Breite und zwei Seitenwände), die Scheibe reicht unter den Lippen von Wand zu Wand, die Wolle ist eine ebene Platte von 52 mm, Blech, Steigrohre und ihre Bettung sind eine Absorberplatte von 16,5 mm, die Sammler bleiben 22-mm-Rohre zwischen den Wänden (Muffen, Konsolen und Schrauben liegen unter der Einzelheit). Das Gestell besteht aus vollen 25-mm-Stäben: je Seite eine Schiene, zwei auf die Schräge des Wannenrückens geschnittene Pfosten und die Kopfstrebe; die gekreuzten Flachstähle bleiben als schlanke Teile in wahrer Größe 20 x 2 mm.",
        ),
        text(
            "Thermal: references to the body solids per region and the outlet patch on the glass. Structure: rails, posts, knee and cross braces, the frame ring of the tray walls, the pane, the nine risers at true size and the headers are the members; four foot and cleat nodes per side frame are the joints; discs in wool, pane and plate, two sleeves on the headers and three lumps per side frame are the masses; two contact strips are the support.",
            "Thermik: Verweise auf die Körper je Region und das Auslassfeld auf dem Glas. Tragwerk: Schienen, Pfosten, Kopf- und Kreuzstreben, der Rahmenring der Wannenwände, die Scheibe, die neun Steigrohre in wahrer Größe und die Sammler sind die Stäbe; je Seitenrahmen vier Fuß- und Konsolknoten sind die Gelenke; Scheiben in Wolle, Glas und Platte, zwei Hülsen auf den Sammlern und je Seitenrahmen drei Klumpen sind die Massen; zwei Kontaktstreifen sind das Auflager.",
        ),
    ],
    "200": [
        text(
            "Authored for the 30 mm feature: each side of the stand is one frame (rail and both posts merged, 30 mm thick) with a 30 mm knee bar; the collector is a tray with a 40 mm floor (sheet and wool) and 30 mm walls that holds a 30 mm absorber slab between two 30 mm header bars and a 30 mm glass slab flush with the rim (the air gap closes). The crossed flats are below the feature and leave the body.",
            "Für die Einzelheit von 30 mm verfasst: Jede Gestellseite ist ein Rahmen (Schiene und beide Pfosten verschmolzen, 30 mm dick) mit einer 30-mm-Kopfstrebe; der Kollektor ist eine Wanne mit 40 mm Boden (Blech und Wolle) und 30 mm Wänden, die eine 30-mm-Absorberplatte zwischen zwei 30-mm-Sammlerstäben und eine randbündige 30-mm-Glasplatte aufnimmt (der Luftspalt schließt sich). Die gekreuzten Flachstähle liegen unter der Einzelheit und verlassen den Körper.",
        ),
        text(
            "Thermal: references to the body solids and the outlet patch on the glass. Structure: the two side frames with their knee bars, the crossed flats as slender members at true size, a 30 mm frame ring for the housing, the glass slab, the nine risers as slender members and the header bars; one mass for the collector and one lump per rear post; two contact strips.",
            "Thermik: Verweise auf die Körper und das Auslassfeld auf dem Glas. Tragwerk: die beiden Seitenrahmen mit ihren Kopfstreben, die gekreuzten Flachstähle als schlanke Stäbe in wahrer Größe, ein 30-mm-Rahmenring für die Wanne, die Glasplatte, die neun Steigrohre als schlanke Stäbe und die Sammlerstäbe; eine Masse für den Kollektor und je hinterem Pfosten ein Klumpen; zwei Kontaktstreifen.",
        ),
    ],
    "500": [
        text(
            "Authored for the 75 mm feature and already minimal: one slab of 2.0 m by 1.0 m by 0.1 m for the whole collector and one 75 mm frame per side of the stand (rail and both posts merged).",
            "Für die Einzelheit von 75 mm verfasst und bereits minimal: eine Platte von 2,0 m mal 1,0 m mal 0,1 m für den ganzen Kollektor und je Gestellseite ein 75-mm-Rahmen (Schiene und beide Pfosten verschmolzen).",
        ),
        text(
            "Thermal: the slab is split into the housing block and, on the sun side, the glazing as its 3.2 mm layer; the absorber sheet (its 0.5 mm layer) and the two 22 mm headers (slender) are real internals at their true places; the outlet patch is one feature thick and sits in the upper end of the slab. Structure: the two side frames, a 75 mm frame ring for the housing with the pane as a slab inside it, three of the nine risers and the two header pipes as slender members; one mass for the collector, one lump per rear post, two contact strips.",
            "Thermik: Die Platte teilt sich in den Wannenblock und auf der Sonnenseite die Verglasung als ihre 3,2-mm-Schicht; das Absorberblech (seine 0,5-mm-Schicht) und die beiden 22-mm-Sammler (schlank) sind echte Innenteile an ihrem wahren Ort; das Auslassfeld ist eine Einzelheit dick und sitzt im oberen Ende der Platte. Tragwerk: die beiden Seitenrahmen, ein 75-mm-Rahmenring für die Wanne mit der Scheibe als Platte darin, drei der neun Steigrohre und die beiden Sammlerrohre als schlanke Stäbe; eine Masse für den Kollektor, je hinterem Pfosten ein Klumpen, zwei Kontaktstreifen.",
        ),
    ],
}


def volumes():
    b50, b100, b200 = body50(), body100(), body200()
    lists = {
        "50": (b50, thermal_of(b50, 0.0925, 0.1), structure50()),
        "100": (b100, thermal_of(b100, 0.085, 0.1), structure100()),
        "200": (b200, thermal_of(b200, 0.072, 0.102), structure200()),
        "500": (body500(), thermal500(), structure500()),
    }
    return {k: {"status": "authored", "denominator": int(k), "body": b, "thermal": t, "structure": s, "decisions": DECISIONS[k]} for k, (b, t, s) in lists.items()}


def shape(sid, bone, role, z, tone, geometry, region=None, material=None, regions=None, hatch=None):
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, **geometry}
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if hatch:
        out["hatch"] = hatch
    return out


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": n(x0), "y": n(y0), "w": n(x1 - x0), "h": n(y1 - y0)}


def poly(points):
    return {"kind": "poly", "points": [[n(a), n(b)] for a, b in points], "closed": True}


def circle(cx, cy, radius):
    return {"kind": "ellipse", "cx": n(cx), "cy": n(cy), "rx": radius, "ry": radius}


MERGED = ["housing", "glazing", "absorber", "header-lower", "header-upper"]


def in_glass(t, v):
    return hu(t) - hu(GLASS_AT[0]), v - GLASS_AT[1]


def in_sheet(t, v):
    return t - HEADER_AT[0] - SHEET_AT[0], v - HEADER_AT[1] - SHEET_AT[1]


def elevation(scale):
    a, b = hu(0), hu(2)
    wall = {50: 0.0075, 100: 0.015}.get(scale)
    half = {50: 0.0125, 100: 0.0125, 200: 0.015, 500: 0.0375}[scale]
    rail = {50: LEG, 100: LEG, 200: 0.03, 500: 0.075}[scale]
    out = [shape("base-rail", "stand", "body", 3, 0.5, rect(0, 0, 1.5, rail), "stand", STEEL)]
    if scale == 50:
        out += [
            shape("front-strut", "stand", "body", 3, 0.5, rect(XF - half, 0.002, XF + half, POST_FRONT_TOP), "stand", STEEL),
            shape("rear-strut", "stand", "body", 3, 0.5, rect(XR - half, 0.002, XR + half, POST_REAR_TOP), "stand", STEEL),
            shape("knee-brace", "stand", "body", 3.02, 0.5, poly(knee_outline(half, 0.003)), "stand", STEEL),
            shape("front-cleat", "stand", "body", 3.04, 0.5, poly(cleat(XF, CLEAT_FRONT_LOW, 0.0075, 0.0075)), "stand", STEEL),
            shape("rear-cleat", "stand", "body", 3.04, 0.5, poly(cleat(XR, CLEAT_REAR_LOW, 0.0075, 0.0075)), "stand", STEEL),
        ]
    else:
        out += [
            shape("front-strut", "stand", "body", 3, 0.5, poly(strut(XF, half, rail)), "stand", STEEL),
            shape("rear-strut", "stand", "body", 3, 0.5, poly(strut(XR, half, rail)), "stand", STEEL),
        ]
        if scale != 500:
            out += [shape("knee-brace", "stand", "body", 3.02, 0.5, poly(knee_outline(half, 0)), "stand", STEEL)]
    if wall is None:
        return out + [shape("collector-box", "housing", "body", 3.4, 0.45, rect(a, 0, b, 0.1), regions=MERGED, material=ALU)]
    tray = [(a, 0), (b, 0), (b, 0.1), (b - wall, 0.1), (b - wall, wall), (a + wall, wall), (a + wall, 0.1), (a, 0.1)]
    if scale == 50:
        wool = [(a + wall, wall), (b - wall, wall), (b - wall, 0.052), (hu(1.94), 0.052), (hu(1.94), 0.0598), (hu(0.06), 0.0598), (hu(0.06), 0.052), (a + wall, 0.052)]
    else:
        wool = [(a + wall, wall), (b - wall, wall), (b - wall, 0.0538), (a + wall, 0.0538)]
    glass_low = 0.1 - wall
    g0, g1 = in_glass(wall, glass_low), in_glass(2 - wall, 0.1)
    out += [
        shape("tray-side-wall", "housing", "body", 3.1, 0.55, rect(a, 0, b, 0.1), "housing", ALU),
        shape("tray", "housing", "body", 3.2, 0.35, poly(tray), "housing", ALU),
        shape("insulation", "housing", "body", 3.3, 0.8, poly(wool), "housing", WOOL, hatch="insulation" if scale == 50 else None),
        shape("glass", "glass", "glazing", 3.4, 0.92, rect(g0[0], g0[1], g1[0], g1[1]), "glazing", GLASS),
    ]
    if scale == 50:
        s0, s1 = in_sheet(0.06, 0.0698), in_sheet(1.94, 0.0773)
        r0, r1 = in_sheet(0.045, 0.0598), in_sheet(1.955, 0.0698)
        out += [
            shape("absorber-plate", "absorber", "body", 3.5, 0.2, rect(s0[0], s0[1], s1[0], s1[1]), "absorber", COAT),
            shape("riser-strip", "absorber", "detail", 3.55, 0.55, rect(r0[0], r0[1], r1[0], r1[1]), "absorber", CU),
        ]
    else:
        s0, s1 = in_sheet(0.06, 0.0538), in_sheet(1.94, 0.0703)
        out += [shape("absorber-strip", "absorber", "body", 3.5, 0.2, rect(s0[0], s0[1], s1[0], s1[1]), "absorber", COAT)]
    out += [
        shape("header-upper", "headers", "body", 3.6, 0.4, circle(0, 0, 0.011), "header-upper", CU),
        shape("header-lower", "headers", "body", 3.6, 0.4, circle(1.91, 0, 0.011), "header-lower", CU),
    ]
    return out


def front(scale):
    foot = ({50: 0.423, 100: 0.425, 200: 0.42, 500: 0.375}[scale], 0.45)
    low_top = world(2, 0.1)[1]
    top = world(0, 0.1)[1]
    out = [
        shape("front-feet-left", "stand", "body", 2.7, 0.5, rect(foot[0], 0, foot[1], 0.19), "stand", STEEL),
        shape("front-feet-right", "stand", "body", 2.7, 0.5, rect(-foot[1], 0, -foot[0], 0.19), "stand", STEEL),
    ]
    if scale == 500:
        return out + [shape("front-box", "housing", "body", 3.2, 0.4, rect(-0.5, LOW, 0.5, top), regions=MERGED, material=ALU)]
    strip = 0.03 if scale == 200 else 0.02
    sill = world(2 - strip, 0.1)[1]
    head = top - 0.03 if scale == 200 else world(strip, 0.1)[1]
    inner = 0.5 - strip
    glass_role = "body" if scale == 200 else "glazing"
    out += [
        shape("front-end-face", "housing", "body", 3.2, 0.35, rect(-0.5, LOW, 0.5, sill), "housing", ALU),
        shape("front-side-left", "housing", "body", 3.2, 0.35, rect(inner, sill, 0.5, top), "housing", ALU),
        shape("front-side-right", "housing", "body", 3.2, 0.35, rect(-0.5, sill, -inner, top), "housing", ALU),
        shape("front-top-rail", "housing", "body", 3.2, 0.35, rect(-inner, head, inner, top), "housing", ALU),
        shape("front-glass", "glass", glass_role, 3.3, 0.92, rect(-inner, sill, inner, head), "glazing", GLASS),
    ]
    if scale == 200:
        sheet_top, axis = world(0.07, 0.07)[1], world(0.055, 0.055)[1]
        out += [
            shape("front-absorber", "absorber", "body", 3.4, 0.2, rect(-0.455, sill, 0.455, sheet_top), "absorber", COAT),
            shape("front-header-upper", "headers", "body", 3.6, 0.4, rect(-0.47, axis - 0.015, 0.47, axis + 0.015), "header-upper", CU),
        ]
        return out
    sheet_top, axis = world(0.06, 0.07015)[1], world(0.045, 0.0648)[1]
    seam = 0.005 if scale == 50 else 0.0075
    out += [shape("front-absorber", "absorber", "body", 3.4, 0.2, rect(-0.455, sill, 0.455, sheet_top), "absorber", COAT)]
    out += [shape("front-riser-%d" % (i + 1), "absorber", "detail", 3.5, 0.55, rect(w - seam, sill, w + seam, sheet_top), "absorber", CU) for i, w in enumerate(PITCH)]
    out += [shape("front-header-upper", "headers", "body", 3.6, 0.4, rect(-inner, sheet_top, inner, axis + 0.011), "header-upper", CU)]
    return out


def plan(scale):
    if scale == 500:
        return [shape("plan-box", "housing", "body", 3.2, 0.4, rect(-HALF, -0.5, HALF, 0.5), regions=MERGED, material=ALU)]
    strip = 0.03 if scale == 200 else 0.02
    inner = 0.5 - strip
    glass_role = "body" if scale == 200 else "glazing"
    g0, g1 = world(strip, 0.1)[0], world(2 - strip, 0.1)[0]
    out = [
        shape("plan-box", "housing", "body", 3.2, 0.35, rect(-HALF, -0.5, HALF, 0.5), "housing", ALU),
        shape("plan-glass", "glass", glass_role, 3.3, 0.92, rect(g0, -inner, g1, inner), "glazing", GLASS),
    ]
    if scale == 200:
        s0, s1 = world(0.07, 0.07)[0], world(1.87, 0.07)[0]
        upper, lower = world(0.055, 0.055)[0], world(1.945, 0.055)[0]
        out += [
            shape("plan-absorber", "absorber", "body", 3.4, 0.2, rect(s0, -0.455, s1, 0.455), "absorber", COAT),
            shape("plan-header-lower", "headers", "body", 3.6, 0.4, rect(lower - 0.015, -0.47, lower + 0.015, 0.47), "header-lower", CU),
            shape("plan-header-upper", "headers", "body", 3.6, 0.4, rect(g0, -0.47, g0 + 0.03, 0.47), "header-upper", CU),
        ]
        return out
    s0, s1 = world(0.06, 0.07015)[0], world(1.94, 0.07015)[0]
    upper, lower = world(0.045, 0.0648)[0], world(1.955, 0.0648)[0]
    seam = 0.005 if scale == 50 else 0.0075
    out += [shape("plan-absorber", "absorber", "body", 3.4, 0.2, rect(s0, -0.455, s1, 0.455), "absorber", COAT)]
    out += [shape("plan-riser-%d" % (i + 1), "absorber", "detail", 3.5, 0.55, rect(s0, w - seam, s1, w + seam), "absorber", CU) for i, w in enumerate(PITCH)]
    out += [
        shape("plan-header-lower", "headers", "body", 3.6, 0.4, rect(s1, -inner, lower + 0.011, inner), "header-lower", CU),
        shape("plan-header-upper", "headers", "body", 3.6, 0.4, rect(g0, -inner, s0, inner), "header-upper", CU),
    ]
    return out


DRAWN = {
    "50": {
        "silhouette": text(
            "A 2.0 m by 1.0 m flat-plate collector facing ahead, tilted by 35 degrees on a light steel stand, 34 mm wide and 28 mm tall on paper: the section through the slope shows the aluminium tray with mineral wool, the glass cover, the absorber sheet on its cut riser and a header circle at each end, behind them the far side wall and the far side frame of the stand.",
            "Ein 2,0 m mal 1,0 m großer, nach vorn gerichteter Flachkollektor, 35 Grad geneigt auf einem leichten Stahlgestell, auf Papier 34 mm breit und 28 mm hoch: der Schnitt durch die Neigungsebene zeigt die Aluminiumwanne mit Mineralwolle, die Glasabdeckung, das Absorberblech auf seinem geschnittenen Steigrohr und je einen Sammlerkreis an den Enden, dahinter die hintere Seitenwand und den hinteren Seitenrahmen des Gestells.",
        ),
        "retained": text(
            "Tray section, mineral wool with hatch and header channels, glass, absorber sheet, the riser in the cut, both headers, base rail, both posts with their cleats and the knee brace. The front view, seen from ahead onto the aperture, adds the low end wall, the cover frame, the absorber with nine riser seams, the upper header above the sheet and the front posts under the low edge; the plan shows the same through the glass with both headers.",
            "Wannenschnitt, Mineralwolle mit Schraffur und Sammlerrinnen, Glas, Absorberblech, das Steigrohr im Schnitt, beide Sammler, Fußschiene, beide Pfosten mit ihren Konsolen und die Kopfstrebe. Die Vorderansicht, von vorn auf die Apertur gesehen, ergänzt die untere Stirnwand, den Deckrahmen, den Absorber mit neun Steigrohrnähten, den oberen Sammler über dem Blech und die vorderen Pfosten unter der Unterkante; der Grundriss zeigt dasselbe durch das Glas mit beiden Sammlern.",
        ),
        "merged": text(
            "Glass ledge and cover angle merge into the tray section.",
            "Glasauflage und Abdeckwinkel gehen im Wannenschnitt auf.",
        ),
        "omitted": text(
            "The sealing gasket, the bolt heads of 13 mm, the flush connection sockets and the crossed braces, which stand edge-on behind the rear post in the side view and behind the collector in the front view.",
            "Die Dichtung, die 13 mm großen Schraubenköpfe, die bündigen Anschlussmuffen und die gekreuzten Streben, die in der Seitenansicht hochkant hinter dem hinteren Pfosten und in der Vorderansicht hinter dem Kollektor stehen.",
        ),
        "thinElements": text(
            "Sheets are drawn at the smallest printable 7.5 mm, growing into the collector: the 1.5 mm tray, the 3.2 mm glass (flush with the top), the 0.5 mm absorber sheet (upwards from the risers) and the 3 mm cleats, so the 25 mm air gap reads 15 mm; risers 10 mm, headers 22 mm and the 25 mm angles of the stand are real size.",
            "Bleche sind mit den kleinsten druckbaren 7,5 mm gezeichnet und wachsen in den Kollektor hinein: die 1,5-mm-Wanne, das 3,2-mm-Glas (bündig mit der Oberkante), das 0,5-mm-Absorberblech (von den Steigrohren nach oben) und die 3-mm-Konsolen, sodass der 25-mm-Luftspalt 15 mm misst; Steigrohre 10 mm, Sammler 22 mm und die 25-mm-Winkel des Gestells haben reale Größe.",
        ),
        "hatching": text("Mineral wool in the section: lines at 45 degrees.", "Mineralwolle im Schnitt: Linien unter 45 Grad."),
        "styles": text(
            "Darker tones carry hot or heavy parts, lighter tones thin parts and parts beyond the cut; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
            "Dunklere Töne tragen heiße oder schwere Teile, hellere Töne dünne Teile und Teile hinter dem Schnitt; Körperstift 0.18 mm, Detail 0.13 mm, Boden 0.35 mm.",
        ),
        "overlays": text(
            "Housing, glazing, absorber, two headers and stand are six regions: in the thermal view the absorber and the upper header glow hot behind the cooler glass, the lower header stays cooler while the pump runs.",
            "Wanne, Glas, Absorber, zwei Sammler und Gestell sind sechs Regionen: in der Thermalansicht glühen Absorber und oberer Sammler heiß hinter dem kühleren Glas, der untere Sammler bleibt bei laufender Pumpe kühler.",
        ),
    },
    "100": {
        "silhouette": text(
            "A 17 mm by 14 mm collector facing ahead: tray, wool, glass, one absorber strip and a header circle at each end on two posts and a base rail.",
            "Ein 17 mm mal 14 mm großer, nach vorn gerichteter Kollektor: Wanne, Wolle, Glas, ein Absorberstreifen und je ein Sammlerkreis an den Enden auf zwei Pfosten und einer Fußschiene.",
        ),
        "retained": text(
            "Tray, plain wool block, glass, absorber strip, both headers, base rail, posts and knee brace; front and plan keep the cover frame, the nine riser seams and the headers.",
            "Wanne, einfacher Wollblock, Glas, Absorberstreifen, beide Sammler, Fußschiene, Pfosten und Kopfstrebe; Vorderansicht und Grundriss behalten den Deckrahmen, die neun Steigrohrnähte und die Sammler.",
        ),
        "merged": text(
            "Absorber sheet, risers and their bedding are one strip of 16.5 mm; post and cleat are one strut cut to the slope.",
            "Absorberblech, Steigrohre und ihre Bettung sind ein Streifen von 16,5 mm; Pfosten und Konsole sind eine auf die Schräge geschnittene Stütze.",
        ),
        "omitted": text(
            "The wool hatch, the header channels, the connection sockets, gasket, bolts and crossed braces.",
            "Die Wollschraffur, die Sammlerrinnen, die Anschlussmuffen, Dichtung, Schrauben und Kreuzstreben.",
        ),
        "thinElements": text(
            "Tray and glass are drawn 15 mm thick, riser seams 15 mm wide; headers 22 mm and the 25 mm bars of the stand are real size.",
            "Wanne und Glas sind 15 mm dick gezeichnet, Steigrohrnähte 15 mm breit; Sammler 22 mm und die 25-mm-Stäbe des Gestells haben reale Größe.",
        ),
        "hatching": text("None.", "Keine."),
        "styles": text(
            "Darker tones carry hot or heavy parts, lighter tones thin parts and parts beyond the cut; body pen 0.18 mm, detail 0.13 mm, ground 0.25 mm.",
            "Dunklere Töne tragen heiße oder schwere Teile, hellere Töne dünne Teile und Teile hinter dem Schnitt; Körperstift 0.18 mm, Detail 0.13 mm, Boden 0.25 mm.",
        ),
        "overlays": text("Same six regions as at 1:50.", "Dieselben sechs Regionen wie bei 1:50."),
    },
    "200": {
        "silhouette": text(
            "An 8 mm by 7 mm collector facing ahead: one tilted slab on two posts and a base rail.",
            "Ein 8 mm mal 7 mm großer, nach vorn gerichteter Kollektor: eine geneigte Platte auf zwei Pfosten und einer Fußschiene.",
        ),
        "retained": text(
            "Slab, base rail, two posts and the knee brace; front and plan add the rim, the glass field, the absorber and the headers.",
            "Platte, Fußschiene, zwei Pfosten und die Kopfstrebe; Vorderansicht und Grundriss ergänzen den Rand, das Glasfeld, den Absorber und die Sammler.",
        ),
        "merged": text(
            "Housing, glass, absorber and both headers are one slab in the side view (merged regions).",
            "Wanne, Glas, Absorber und beide Sammler sind in der Seitenansicht eine Platte (zusammengeführte Regionen).",
        ),
        "omitted": text(
            "Wool, risers, the air gap, connection sockets, crossed braces and all section detail.",
            "Wolle, Steigrohre, der Luftspalt, Anschlussmuffen, Kreuzstreben und alle Schnittdetails.",
        ),
        "thinElements": text(
            "Rim, header bars, posts, knee brace and rail are drawn 30 mm (the real angles are 25 mm); the upper rim of the front view is drawn 30 mm although the slope foreshortens it to 17 mm.",
            "Rand, Sammlerstäbe, Pfosten, Kopfstrebe und Schiene sind 30 mm gezeichnet (die echten Winkel messen 25 mm); der obere Rand der Vorderansicht ist 30 mm gezeichnet, obwohl die Neigung ihn auf 17 mm verkürzt.",
        ),
        "hatching": text("None.", "Keine."),
        "styles": text(
            "Darker tones carry hot or heavy parts, lighter tones thin parts; body pen 0.13 mm, ground 0.25 mm.",
            "Dunklere Töne tragen heiße oder schwere Teile, hellere Töne dünne Teile; Körperstift 0.13 mm, Boden 0.25 mm.",
        ),
        "overlays": text("The slab lists five regions as one merged form.", "Die Platte führt fünf Regionen als eine zusammengeführte Form."),
    },
    "500": {
        "silhouette": text(
            "A 3.4 mm by 2 mm by 2.8 mm collector facing ahead: one merged tilted slab on a base rail and two posts in the side view, one merged block on two feet in the front view and one merged rectangle in the top view.",
            "Ein 3,4 mm mal 2 mm mal 2,8 mm großer, nach vorn gerichteter Kollektor: eine zusammengeführte geneigte Platte auf Fußschiene und zwei Pfosten in der Seitenansicht, ein zusammengeführter Block auf zwei Füßen in der Vorderansicht und ein zusammengeführtes Rechteck in der Draufsicht.",
        ),
        "retained": text(
            "Slab, base rail and both posts in the side view; both feet and the slab face in the front view; the full footprint in the top view. The tilt and the three real dimensions of 1.70 m, 1.00 m and 1.41 m are kept.",
            "Platte, Fußschiene und beide Pfosten in der Seitenansicht; beide Füße und Plattenfläche in der Vorderansicht; die volle Grundfläche in der Draufsicht. Die Neigung und die drei wahren Maße 1,70 m, 1,00 m und 1,41 m bleiben erhalten.",
        ),
        "merged": text(
            "Housing, glass, absorber and both headers are one form listing five regions; rail and posts of each side are one frame.",
            "Wanne, Glas, Absorber und beide Sammler sind eine Form mit fünf Regionen; Schiene und Pfosten jeder Seite sind ein Rahmen.",
        ),
        "omitted": text(
            "Glass field, absorber outline, headers, wool, risers, knee and crossed braces and all section detail.",
            "Glasfeld, Absorberumriss, Sammler, Wolle, Steigrohre, Kopf- und Kreuzstreben und alle Schnittdetails.",
        ),
        "thinElements": text(
            "Posts and rail are drawn 75 mm (0.15 mm on paper), the smallest printable feature; the real angles are 25 mm.",
            "Pfosten und Schiene sind 75 mm (0,15 mm auf Papier) gezeichnet, die kleinste druckbare Einzelheit; die echten Winkel messen 25 mm.",
        ),
        "hatching": text("None.", "Keine."),
        "styles": text(
            "Body pen 0.13 mm, ground 0.25 mm; darker tones carry the housing, lighter tones the thin parts.",
            "Körperstift 0,13 mm, Boden 0,25 mm; dunklere Töne tragen die Wanne, hellere Töne die dünnen Teile.",
        ),
        "overlays": text("The slab lists five regions as one merged form in every view.", "Die Platte führt in jeder Ansicht fünf Regionen als eine zusammengeführte Form."),
    },
}

TAKEOFF = [
    ("rails", 2 * 1.5 * 0.754, 0.75, 0.007),
    ("rear posts", 2 * (POST_REAR_TOP - 0.002) * 0.754, XR, 0.551),
    ("front posts", 2 * (POST_FRONT_TOP - 0.002) * 0.754, XF, 0.184),
    ("knee braces", 2 * (KNEE_LENGTH + 0.006) * 0.754, (KNEE_FOOT[0] + KNEE_HEAD[0]) / 2, (KNEE_FOOT[1] + KNEE_HEAD[1]) / 2),
    ("cross braces", 2 * (BRACE_LENGTH + 0.02) * 0.314, XR - 0.0115, 0.605),
    ("rear cleats", 2 * 0.18 * 0.025 * 0.003 * 7850, XR + 0.02, 1.09),
    ("front cleats", 2 * 0.17 * 0.025 * 0.003 * 7850, XF + 0.02, 0.355),
    ("bolts", 20 * 0.018, 0.45, 0.45),
]


def stand_mass():
    total = sum(m for _, m, _, _ in TAKEOFF)
    return total, sum(m * x for _, m, x, _ in TAKEOFF) / total, sum(m * y for _, m, _, y in TAKEOFF) / total


AREAS = {"housing": 5.27, "glazing": 3.96, "absorber": 3.88, "header-lower": 0.073, "header-upper": 0.073, "stand": 0.82}
NOTES = text(
    "A typical glazed flat-plate collector of 2.0 m by 1.0 m by 0.1 m (gross area 2.0 m2, absorber sheet 1.71 m2) and 40 kg with its stand: a folded 1.5 mm anodised aluminium tray with 58 mm of mineral wool on the back, a 3.2 mm low-iron solar glass cover held by a clamping cover angle, behind a 25 mm air gap a selective absorber sheet of 0.5 mm on nine 10 mm copper risers at 0.1 m pitch that end in two 22 mm copper headers (inlet below, outlet above) whose flush ends are the four connections, mounted at 35 degrees on a 6.4 kg stand of cold-formed galvanised steel angle 25 x 25 x 2 mm: two base rails, two posts per rail with folded cleats, a knee brace at each rear post and two crossed flat braces between the rear posts. The masses are tray 10 kg, glass 14 kg, absorber 8 kg, headers 1.6 kg and stand 6.4 kg. The optical efficiency is 0.78 and the loss coefficients a1 = 3.6 W/(m2 K) and a2 = 0.015 W/(m2 K2), so the stagnation temperature at 1000 W/m2 is about 150 degC and the useful gain at 60 degC mean fluid temperature about 1 kW (ISO 9806 style). In the thermal model the absorber takes the solar share from the environment (effective absorptivity 0.86 behind the glass), and the operating key flow removes 620 W at the pumped state, which is the mean useful gain under the model's solar geometry factor; the key fluid heats the headers (inlet 8 W, outlet 22 W, three times that when the fluid stands and vapour fills them). Reference environment for the thermal view: summer-heatwave-calm. The collector faces ahead: the low edge is at +x, the elevation is the section through the slope, the front view looks from ahead onto the aperture and the low edge, the plan from above. The origin is the centre of the base rails on the roof.",
    "Ein typischer verglaster Flachkollektor von 2,0 m mal 1,0 m mal 0,1 m (Bruttofläche 2,0 m2, Absorberblech 1,71 m2) und 40 kg samt Gestell: eine gekantete, 1,5 mm dicke eloxierte Aluminiumwanne mit 58 mm Mineralwolle am Rücken, eine 3,2 mm dicke eisenarme Solarglasabdeckung unter einem klemmenden Abdeckwinkel, hinter 25 mm Luftspalt ein selektives Absorberblech von 0,5 mm auf neun 10 mm dicken Kupfer-Steigrohren im Abstand von 0,1 m, die in zwei 22 mm dicken Kupfer-Sammelrohren enden (Vorlauf unten, Rücklauf oben), deren bündige Enden die vier Anschlüsse sind, mit 35 Grad Neigung auf einem 6,4 kg schweren Gestell aus kaltgeformtem verzinktem Stahlwinkel 25 x 25 x 2 mm montiert: zwei Fußschienen, je Schiene zwei Pfosten mit gekanteten Konsolen, je hinterem Pfosten eine Kopfstrebe sowie zwei gekreuzte Flachstahlstreben zwischen den hinteren Pfosten. Die Massen sind Wanne 10 kg, Glas 14 kg, Absorber 8 kg, Sammler 1,6 kg und Gestell 6,4 kg. Der optische Wirkungsgrad beträgt 0,78 und die Verlustkoeffizienten a1 = 3,6 W/(m2 K) und a2 = 0,015 W/(m2 K2), sodass die Stillstandstemperatur bei 1000 W/m2 etwa 150 degC und der Nutzgewinn bei 60 degC mittlerer Fluidtemperatur etwa 1 kW beträgt (Auslegung nach ISO 9806). Im Thermalmodell nimmt der Absorber den Solaranteil aus der Umgebung auf (wirksamer Absorptionsgrad 0,86 hinter dem Glas), und der Betriebsschlüssel flow entzieht 620 W im gepumpten Zustand, dem mittleren Nutzgewinn bei dem Solargeometriefaktor des Modells; der Schlüssel fluid heizt die Sammler (Vorlauf 8 W, Rücklauf 22 W, das Dreifache bei stehendem Fluid und Dampf in den Sammlern). Bezugsumgebung für die Thermalansicht: summer-heatwave-calm. Der Kollektor blickt nach vorn: die Unterkante liegt bei +x, die Ansicht ist der Schnitt durch die Neigungsebene, die Vorderansicht blickt von vorn auf die Apertur und die Unterkante, der Grundriss von oben. Der Ursprung ist die Mitte der Fußschienen auf dem Dach.",
)
REFERENCE = {
    "id": "collector-data-sheets",
    "citation": "Manufacturer data of 2 m2 class flat-plate collectors and their flat-roof stands, read 2026-11-20: Romstal Vision installation manual (gross area 2.00 m2, 1235 x 1625 x 85 mm, 3.2 mm glass, 22 mm connections); Bongioanni flat-plate collector (nine riser pipes, 22 mm header tubes, mineral wool); TWL flat-roof stand for one FK200 collector (aluminium, 3.82 kg) and Engel Helios flat-roof holder (galvanised steel, 4 kg) as the weight class of a light stand.",
}


def bone(bid, parent, x, y, angle, length, title, mass):
    return {"id": bid, "parent": parent, "x": x, "y": y, "angleDeg": angle, "title": title, "lengthM": length, "joint": {"type": "fixed"}, "mass": {"kg": mass[0], "comX": mass[1], "comY": mass[2], "distribution": "point", "source": "authored-estimate"}}


def apply(asset):
    old = {b["id"]: b for b in asset["rig"]["bones"]}
    _, com_x, com_y = stand_mass()
    asset["rig"]["bones"] = [
        bone("stand", None, -0.75, 0, 0, 1.5, old["stand"]["title"], (6.4, n(round(com_x, 3)), n(round(com_y, 3)))),
        bone("housing", "stand", XR, YH, -35, n(LENGTH - U0), old["housing"]["title"], (10, hu(1.0), 0.023)),
        bone("glass", "housing", hu(GLASS_AT[0]), GLASS_AT[1], 0, 1.99, old["glass"]["title"], (14, 0.995, 0)),
        bone("headers", "housing", hu(HEADER_AT[0]), HEADER_AT[1], 0, 1.91, old["headers"]["title"], (1.6, 0.955, 0)),
        bone("absorber", "headers", SHEET_AT[0], SHEET_AT[1], 0, 1.88, old["absorber"]["title"], (8, 0.94, -0.003)),
    ]
    for anchor in asset["anchors"]:
        if anchor["id"] == "base-contact":
            anchor["x"], anchor["y"] = 0.75, 0
        if anchor["id"] == "glass-top":
            anchor["x"], anchor["y"] = 0.11, 0.0016
            anchor["title"] = text("Upper strip of the glass", "Oberer Streifen des Glases")
    for key, rep in asset["representations"].items():
        scale = int(key)
        rep["geometry"] = {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
        rep["decisions"] = DRAWN[key]
    for region in asset["thermal"]["regions"]:
        region["areaM2"] = AREAS[region["id"]]
        if region["id"] in ("header-lower", "header-upper"):
            region["characteristicLengthM"] = 0.022
        if region["id"] == "glazing":
            layer = region["layers"][0]
            layer["title"] = text("Low-iron solar glass 3.2 mm", "Eisenarmes Solarglas 3,2 mm")
            layer["thicknessM"], layer["resistanceM2KW"], layer["heatCapacityJM2K"] = 0.0032, 0.0032, 6000
            region["heatCapacityJM2K"] = 6000
    import runpy
    runpy.run_path(os.path.join(T, "equipment-fine-ink.py"))["apply"](asset)
    references = [r for r in asset["provenance"]["references"] if r["id"] != REFERENCE["id"]]
    asset["provenance"]["references"] = references + [REFERENCE]
    asset["provenance"]["notes"] = NOTES
    asset["volumes"] = volumes()
    return asset


def member(source, key):
    found = re.search(r'\n  "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = re.sub(r"\b(\d)e-05\b", r"0.0000\1", json.dumps(value, indent=1, ensure_ascii=False)).split("\n")
    return "\n".join(line if i == 0 else "  " + line for i, line in enumerate(lines))


def inline(value):
    if not isinstance(value, list):
        return False
    plain = lambda item: not isinstance(item, (dict, list))
    return all(plain(item) for item in value) or (all(isinstance(item, list) and all(plain(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)


def spliced(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict) and value:
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + spliced(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list) and value and not inline(value):
        return "[\n" + ",\n".join(inner + spliced(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = json.loads(source)
    apply(document["asset"])
    total, com_x, com_y = stand_mass()
    print("stand take-off %.2f kg, centre of mass (%.3f, %.3f) in the stand frame" % (total, com_x, com_y))
    for name, mass, _, _ in TAKEOFF:
        print("  %-14s %.2f kg" % (name, mass))
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": document["asset"]["volumes"]}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = member(source, key)
            if json.loads(source[start:end]) != before["asset"][key]:
                raise SystemExit("member %s not located, not written" % key)
            source = source[:start] + (spliced(document["asset"][key], 1) if key == "volumes" else dumped(document["asset"][key])) + source[end:]
        check = json.loads(source)
        if check != document:
            raise SystemExit("written text does not parse to the candidate")
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        folder = os.path.join(T, "🗑️generated", "r3-solar-thermal-collector")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=1)
        print("candidate written:", os.path.join(folder, "🔣️.json"))
