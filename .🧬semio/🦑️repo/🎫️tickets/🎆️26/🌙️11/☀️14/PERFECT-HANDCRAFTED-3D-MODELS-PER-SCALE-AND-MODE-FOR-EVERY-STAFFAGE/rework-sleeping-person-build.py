"""🛌 Hand-chosen rig girdles and skeleton of humans/sleeping-person (second round, pass 1).

Usage: python3 rework-sleeping-person-build.py candidate|publish
candidate writes 🗑️generated/rework-sleeping-person/cand/🔣️.json (catalog document + rig + volumes source with the edits);
publish writes the rig into the catalog document and the solids into volumes/humans/sleeping-person.json.
Frames of the lateral bones are the ones `rig-rest.ts` printed for the rig below (see rig-lateral-rest.md 1.2).
"""
import json
import math
import sys

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json"
SOURCE = T + "/volumes/humans/sleeping-person.json"
CANDIDATE = T + "/🗑️generated/rework-sleeping-person/cand/🔣️.json"

FRAMES = {
    "torso": ([0.9545, 0.553, 0], [-1, 0, 0], [0, -1, 0], [0, 0, 1]),
    "pelvis": ([1.05, 0.553, 0], [-1, 0, 0], [0, -1, 0], [0, 0, 1]),
    "clavicle-l": ([0.5175, 0.619, 0.024], [0.0258, -0.4778, 0.8781], [-0.9996, 0, 0.0294], [-0.0140, -0.8785, -0.4776]),
    "clavicle-r": ([0.5175, 0.619, -0.024], [0.0258, -0.4778, -0.8781], [-0.9996, 0, -0.0294], [0.0140, 0.8785, -0.4776]),
    "ilium-l": ([0.98, 0.498, 0.045], [0.7111, 0.5588, 0.4267], [-0.5145, 0, 0.8575], [0.4791, -0.8293, 0.2875]),
    "ilium-r": ([0.98, 0.498, -0.045], [0.7111, 0.5588, -0.4267], [-0.5145, 0, -0.8575], [-0.4791, 0.8293, 0.2875]),
}
BONE = {"material": "bone", "role": "body", "tone": 0.85}
LIMB = [0.5, 0.5, 0.5, 0.5]


def r5(values):
    return [round(v, 5) + 0.0 for v in values]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a))
    return [c / n for c in a]


def torso(a, b, c):
    """🫁 World point of a torso point: a towards the head from the lumbosacral joint, b posterior, c left."""
    return [0.9545 - a, 0.553 - b, c]


def pelvis(a, b, c):
    """🦴 World point of a pelvis point: a towards the head from the line of the hip joints, b posterior, c left."""
    return [1.05 - a, 0.553 - b, c]


def tdir(a, b, c):
    return [-a, -b, c]


def local(bone, point):
    o, u, v, w = FRAMES[bone]
    d = sub(point, o)
    return [dot(d, u), dot(d, v), dot(d, w)]


def localdir(bone, direction):
    _, u, v, w = FRAMES[bone]
    return [dot(direction, u), dot(direction, v), dot(direction, w)]


def quaternion(x, y, z):
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return r5(q)


def mirror(point, side):
    return [point[0], point[1], point[2] * side]


def solid(identity, bone, part="member", **fields):
    return {"id": identity, "bone": bone, **BONE, **fields, "member": bone, "part": part}


def loft(identity, bone, stations, side=1, **extra):
    """🪵 Loft through world stations (point, rx, ry, section x direction or None); round sections need no direction."""
    points = [mirror(s[0], side) for s in stations]
    sections = []
    for i, station in enumerate(stations):
        tangent = unit(sub(points[min(i + 1, len(points) - 1)], points[max(i - 1, 0)]))
        section = {"at": r5(local(bone, points[i])), "rx": station[1], "ry": station[2]}
        if station[3] is not None:
            axis = mirror(station[3], side)
            x = unit(sub(axis, [tangent[k] * dot(axis, tangent) for k in range(3)]))
            y = cross(tangent, x)
            section["rotation"] = quaternion(localdir(bone, x), localdir(bone, y), localdir(bone, tangent))
        sections.append(section)
    return solid(identity, bone, kind="loft", sections=sections, capStart=True, capEnd=True, **extra)


def capsule(identity, bone, p0, p1, r0, r1, side=1, **extra):
    return solid(identity, bone, kind="capsule", p0=r5(local(bone, mirror(p0, side))), p1=r5(local(bone, mirror(p1, side))), r0=r0, r1=r1, **extra)


def ellipsoid(identity, bone, centre, radii, axes=None, side=1, part="member"):
    """🥚 Ellipsoid at a world centre; axes = world directions of the first two radii (third follows)."""
    transform = {"translation": r5(local(bone, mirror(centre, side)))}
    if axes is not None:
        x = unit(mirror(axes[0], side))
        y0 = mirror(axes[1], side)
        y = unit(sub(y0, [x[k] * dot(y0, x) for k in range(3)]))
        transform["rotation"] = quaternion(localdir(bone, x), localdir(bone, y), localdir(bone, cross(x, y)))
    return solid(identity, bone, part, kind="ellipsoid", radii=radii, transform=transform)


def own(identity, bone, part="member", **fields):
    """✍️ Solid written directly in the frame of a sagittal bone."""
    return solid(identity, bone, part, **fields)


def spine_line(a):
    """🧬 Posterior offset b of the vertebral bodies along the torso: lumbar lordosis, thoracic kyphosis."""
    table = [(-0.012, 0.036), (0.06, 0.031), (0.12, 0.036), (0.175, 0.047), (0.24, 0.059), (0.30, 0.065), (0.37, 0.062), (0.43, 0.046), (0.4825, 0.020)]
    for (a0, b0), (a1, b1) in zip(table, table[1:]):
        if a <= a1:
            t = max(0.0, (a - a0) / (a1 - a0))
            return b0 + (b1 - b0) * t * t * (3 - 2 * t)
    return table[-1][1]


def beaded_spine(feature):
    """🦴 Lumbar (5) and thoracic (12) vertebral bodies with their discs as one beaded column."""
    lumbar = [(-0.010 + 0.037 * k, 0.037, 0.0175 - 0.0005 * k, 0.024 - 0.0006 * k) for k in range(5)]
    thoracic = [(0.175 + 0.0246 * k, 0.0246, 0.0148 - 0.0004 * k, 0.0172 - 0.0004 * k) for k in range(12)]
    sections = []
    for a, height, rx, ry in lumbar + thoracic:
        disc = height * 0.2
        for at, shrink in ((a + disc * 0.5, 0.82), (a + disc, 1.0), (a + height - disc * 0.5, 1.0)):
            sections.append({"at": r5([at, spine_line(at), 0]), "rx": round(max(rx * shrink, feature / 2), 5), "ry": round(max(ry * shrink, feature / 2), 5), "rotation": LIMB})
    sections.append({"at": r5([0.4825, spine_line(0.4825), 0]), "rx": 0.0095, "ry": 0.012, "rotation": LIMB})
    return own("thoracolumbar-spine", "torso", kind="loft", sections=sections, capStart=True, capEnd=True)


def smooth_spine(radius_scale, floor):
    stations = [(-0.012, 0.0175, 0.024), (0.06, 0.017, 0.023), (0.175, 0.015, 0.019), (0.30, 0.0135, 0.016), (0.43, 0.011, 0.014), (0.4825, 0.0095, 0.012)]
    return own("spine", "torso", kind="loft", capStart=True, capEnd=True, sections=[
        {"at": r5([a, spine_line(a), 0]), "rx": round(max(rx * radius_scale, floor), 5), "ry": round(max(ry * radius_scale, floor), 5), "rotation": LIMB} for a, rx, ry in stations])


def skull(scale):
    cranium = own("skull", "head", kind="ellipsoid", radii=[0.062, 0.084, 0.068], transform={"translation": [0.064, 0.010, 0]})
    if scale >= 200:
        jaw = own("mandible", "head", kind="ellipsoid", radii=[0.040, 0.0375, 0.046], transform={"translation": [-0.040, -0.042, 0]})
        return [cranium, jaw]
    base = own("skull-base", "head", kind="ellipsoid", radii=[0.022, 0.060, 0.064], transform={"translation": [0.012, 0.005, 0]})
    face = own("facial-skeleton", "head", kind="loft", capStart=True, capEnd=True, sections=[
        {"at": [0.034, -0.052, 0], "rx": 0.026, "ry": 0.056, "rotation": LIMB},
        {"at": [0.010, -0.058, 0], "rx": 0.026, "ry": 0.060, "rotation": LIMB},
        {"at": [-0.014, -0.060, 0], "rx": 0.024, "ry": 0.046, "rotation": LIMB},
        {"at": [-0.034, -0.060, 0], "rx": 0.022, "ry": 0.034, "rotation": LIMB}])
    r = 0.0075 if scale == 100 else 0.0065
    jaw = own("mandible", "head", kind="loft", capStart=True, capEnd=True, sections=[
        {"at": [0.008, -0.006, 0.060], "rx": r, "ry": r}, {"at": [-0.024, -0.008, 0.049], "rx": r + 0.002, "ry": r},
        {"at": [-0.052, -0.006, 0.046], "rx": r + 0.002, "ry": r}, {"at": [-0.070, -0.042, 0.031], "rx": r + 0.002, "ry": r},
        {"at": [-0.080, -0.066, 0], "rx": r + 0.003, "ry": r + 0.001},
        {"at": [-0.070, -0.042, -0.031], "rx": r + 0.002, "ry": r}, {"at": [-0.052, -0.006, -0.046], "rx": r + 0.002, "ry": r},
        {"at": [-0.024, -0.008, -0.049], "rx": r + 0.002, "ry": r}, {"at": [0.008, -0.006, -0.060], "rx": r, "ry": r}])
    if scale == 100:
        return [cranium, base, face, jaw]
    arches = [own(f"zygomatic-{s}", "head", kind="capsule", p0=[0.012, -0.058, 0.058 * m], p1=[0.012, -0.004, 0.066 * m], r0=0.005, r1=0.005) for s, m in (("l", 1), ("r", -1))]
    return [cranium, base, face, jaw, *arches]


def cervical(scale):
    if scale >= 100:
        r = {100: 0.012, 200: 0.015, 500: 0.0375}[scale]
        return own("cervical-spine", "neck", kind="loft", capStart=True, capEnd=True, sections=[
            {"at": [-0.01, 0.015, 0], "rx": r, "ry": r}, {"at": [0.05, 0.02, 0], "rx": r, "ry": r}, {"at": [0.108, 0.012, 0], "rx": r, "ry": r}])
    sections = []
    for k in range(7):
        a = -0.010 + 0.0166 * k
        b = 0.015 + 0.006 * math.sin(math.pi * (k + 0.5) / 7)
        rx, ry = 0.0085 + 0.0003 * (6 - k), 0.0105 + 0.0006 * (6 - k)
        sections += [{"at": r5([a + 0.002, b, 0]), "rx": round(rx * 0.8, 5), "ry": round(ry * 0.8, 5), "rotation": LIMB},
                     {"at": r5([a + 0.005, b, 0]), "rx": rx, "ry": ry, "rotation": LIMB}, {"at": r5([a + 0.0145, b, 0]), "rx": rx, "ry": ry, "rotation": LIMB}]
    return own("cervical-spine", "neck", kind="loft", sections=sections, capStart=True, capEnd=True)


def shoulder(scale, s, m):
    bone = f"clavicle-{s}"
    out = []
    if scale == 50:
        ahead = tdir(0, -1, 0)
        sc, ac = torso(0.437, -0.066, 0.024), torso(0.433, 0.008, 0.160)
        along = sub(ac, sc)
        bow = [(0.0, 0.0, 0.009), (0.27, 0.008, 0.007), (0.52, 0.006, 0.006), (0.8, -0.007, 0.006), (1.0, 0.0, 0.007)]
        out.append(loft(bone, bone, [([sc[k] + along[k] * t + ahead[k] * f for k in range(3)], r, r, None) for t, f, r in bow], m))
        lateral = tdir(0, -0.42, 0.906)
        blade = [((0.272, 0.068, 0.086), 0.009), ((0.315, 0.064, 0.097), 0.027), ((0.365, 0.061, 0.103), 0.038), ((0.403, 0.060, 0.102), 0.040), ((0.427, 0.062, 0.094), 0.026)]
        out.append(loft(f"scapula-{s}", bone, [(torso(*p), half, 0.004, lateral) for p, half in blade], m))
        out.append(loft(f"scapular-spine-{s}", bone, [(torso(0.392, 0.082, 0.070), 0.005, 0.005, None), (torso(0.411, 0.074, 0.115), 0.006, 0.006, None),
                                                      (torso(0.427, 0.046, 0.158), 0.007, 0.007, None), (torso(0.436, 0.014, 0.166), 0.008, 0.008, None)], m))
        out.append(ellipsoid(f"glenoid-{s}", bone, torso(0.400, 0.030, 0.137), [0.019, 0.014, 0.007], (tdir(1, 0, 0), tdir(0, 1, 0)), m))
        out.append(capsule(f"coracoid-{s}", bone, torso(0.413, 0.028, 0.130), torso(0.419, -0.014, 0.124), 0.006, 0.005, m))
        return out
    radius = {100: 0.0075, 200: 0.015, 500: 0.006}[scale]
    extra = {"slender": True} if scale == 500 else {}
    out.append(capsule(bone, bone, torso(0.437, -0.066, 0.024), torso(0.433, 0.008, 0.160), radius, radius, m, **extra))
    if scale == 100:
        lateral = tdir(0, -0.42, 0.906)
        blade = [((0.275, 0.066, 0.088), 0.012), ((0.365, 0.060, 0.103), 0.038), ((0.425, 0.060, 0.096), 0.028)]
        out.append(loft(f"scapula-{s}", bone, [(torso(*p), half, 0.0075, lateral) for p, half in blade], m))
        out.append(capsule(f"scapular-spine-{s}", bone, torso(0.400, 0.072, 0.080), torso(0.434, 0.018, 0.164), 0.0075, 0.0075, m))
    return out


def hip(scale, s, m):
    bone = f"ilium-{s}"
    up = tdir(0.92, 0, 0.39)
    if scale == 500:
        return [capsule(bone, bone, pelvis(0.085, 0.045, 0.085), pelvis(-0.03, 0.02, 0.07), 0.0375, 0.0375, m)]
    if scale == 200:
        return [loft(bone, bone, [(pelvis(0.070, -0.040, 0.120), 0.035, 0.015, up), (pelvis(0.068, 0.005, 0.120), 0.045, 0.015, up), (pelvis(0.074, 0.060, 0.070), 0.035, 0.015, up)], m),
                capsule(f"ischium-{s}", bone, pelvis(0.030, 0.004, 0.104), pelvis(-0.058, 0.036, 0.062), 0.016, 0.016, m)]
    thick = 0.005 if scale == 50 else 0.0075
    crest = [((0.070, -0.052, 0.112), 0.018), ((0.068, -0.028, 0.126), 0.038), ((0.068, 0.002, 0.120), 0.046), ((0.070, 0.035, 0.102), 0.042), ((0.075, 0.062, 0.068), 0.032), ((0.078, 0.074, 0.046), 0.020)]
    if scale == 100:
        crest = [crest[0], crest[2], crest[4], crest[5]]
    rod = 0.009 if scale == 50 else 0.0085
    out = [loft(bone, bone, [(pelvis(*p), half, thick, up) for p, half in crest], m),
           loft(f"ischium-{s}", bone, [(pelvis(0.012, 0.012, 0.090), 0.016, 0.016, None), (pelvis(-0.030, 0.026, 0.078), 0.014, 0.014, None), (pelvis(-0.064, 0.040, 0.062), 0.016, 0.016, None)], m),
           loft(f"pubis-{s}", bone, [(pelvis(0.006, -0.022, 0.078), 0.011, 0.011, None), (pelvis(-0.014, -0.046, 0.045), rod, rod, None), (pelvis(-0.032, -0.060, 0.010), rod + 0.002, rod + 0.002, None),
                                     (pelvis(-0.054, -0.036, 0.024), rod, rod, None), (pelvis(-0.066, 0.020, 0.052), rod, rod, None)], m)]
    if scale <= 100:
        out.append(ellipsoid(f"acetabulum-{s}", bone, pelvis(0.004, 0.004, 0.098), [0.030, 0.030, 0.012], (tdir(1, 0, 0), tdir(0, 1, 0)), m))
    return out


def sacrum(scale):
    if scale >= 200:
        r = 0.0375 if scale == 500 else 0.016
        return [own("sacrum", "pelvis", kind="ellipsoid", radii=[0.055, r, 0.05], transform={"translation": [0.045, 0.058, 0]})]
    t = 0.012 if scale == 50 else 0.011
    return [own("sacrum", "pelvis", kind="loft", capStart=True, capEnd=True, sections=[
        {"at": [0.096, 0.034, 0], "rx": 0.018, "ry": 0.026, "rotation": LIMB}, {"at": [0.080, 0.056, 0], "rx": t + 0.002, "ry": 0.052, "rotation": LIMB},
        {"at": [0.045, 0.070, 0], "rx": t, "ry": 0.042, "rotation": LIMB}, {"at": [0.012, 0.074, 0], "rx": t - 0.002, "ry": 0.024, "rotation": LIMB},
        {"at": [-0.012, 0.066, 0], "rx": 0.0065 if scale == 50 else 0.0075, "ry": 0.010, "rotation": LIMB}])]


def legs(s, m):
    """🦵 1:50 knee and lower leg: head, trochanter and condyles of the femur, patella on the condyles, fibula from tibial condyle to lateral malleolus."""
    thigh, shank = f"thigh-{s}", f"shank-{s}"
    return [
        own(f"femur-head-{s}", thigh, kind="ellipsoid", radii=[0.022, 0.022, 0.022], transform={"translation": [0, 0, 0]}),
        own(f"greater-trochanter-{s}", thigh, kind="ellipsoid", radii=[0.022, 0.016, 0.014], transform={"translation": [0.034, -0.004, 0.056 * m]}),
        own(f"femur-condyles-{s}", thigh, kind="ellipsoid", radii=[0.020, 0.030, 0.039], transform={"translation": [0.388, -0.004, 0]}),
        own(f"patella-{s}", thigh, kind="ellipsoid", radii=[0.021, 0.010, 0.021], transform={"translation": [0.382, 0.033, 0]}),
        own(f"tibia-plateau-{s}", shank, kind="ellipsoid", radii=[0.015, 0.026, 0.037], transform={"translation": [0.016, -0.002, 0]}),
        own(f"fibula-{s}", shank, kind="loft", capStart=True, capEnd=True, sections=[
            {"at": [0.030, -0.014, 0.034 * m], "rx": 0.010, "ry": 0.010}, {"at": [0.070, -0.012, 0.033 * m], "rx": 0.006, "ry": 0.006},
            {"at": [0.300, -0.008, 0.029 * m], "rx": 0.006, "ry": 0.006}, {"at": [0.395, -0.005, 0.029 * m], "rx": 0.0075, "ry": 0.0075}, {"at": [0.428, -0.004, 0.030 * m], "rx": 0.010, "ry": 0.010}]),
        own(f"medial-malleolus-{s}", shank, kind="ellipsoid", radii=[0.016, 0.012, 0.010], transform={"translation": [0.412, 0, -0.024 * m]}),
    ]


REMOVED = {"skull", "skull-base", "carpus-l", "carpus-r", "metacarpals-l", "metacarpals-r", "mandible", "facial-skeleton", "zygomatic-l", "zygomatic-r", "cervical-spine", "thoracolumbar-spine", "spine", "pelvic-ring", "pelvic-girdle", "sacrum"}
for _s in ("l", "r"):
    REMOVED |= {f"{n}-{_s}" for n in ("clavicle", "scapula", "scapular-spine", "glenoid", "coracoid", "ilium", "ischium", "pubis", "acetabulum", "patella", "fibula", "femur-head", "greater-trochanter", "femur-condyles", "tibia-plateau", "medial-malleolus", "joint-clavicle", "joint-ilium", "metacarpal-1", "metacarpal-2", "metacarpal-3", "metacarpal-4")}
KEEP_LEGS_COARSE = {f"{n}-{s}" for n in ("patella", "fibula") for s in ("l", "r")}


def structure(scale, old):
    removed = REMOVED if scale == 50 else REMOVED - KEEP_LEGS_COARSE
    kept = [s for s in old if s["id"] not in removed]
    new = [*skull(scale), cervical(scale), beaded_spine(0.0075) if scale == 50 else smooth_spine(*{100: (1.0, 0.0075), 200: (1.15, 0.015), 500: (1.0, 0.0375)}[scale]), *sacrum(scale)]
    for s, m in (("l", 1), ("r", -1)):
        new += shoulder(scale, s, m) + hip(scale, s, m)
        if scale == 50:
            new += legs(s, m)
        if scale <= 200:
            radius = {50: 0.012, 100: 0.012, 200: 0.015}[scale]
            new.append(solid(f"joint-clavicle-{s}", f"clavicle-{s}", "joint", kind="ellipsoid", radii=[radius] * 3, transform={"translation": [0, 0, 0]}))
            new.append(solid(f"joint-ilium-{s}", f"ilium-{s}", "joint", kind="ellipsoid", radii=[radius] * 3, transform={"translation": [0, 0, 0]}))
    for item in kept:
        if item["id"].startswith("support-quilt-bearing") and scale >= 100:
            item["size"] = {100: [0.03, 0.009, 0.009], 200: [0.06, 0.05, 0.06], 500: [0.075, 0.075, 0.075]}[scale]
            item["transform"]["translation"][1] = item["size"][1] / 2
        if item["id"] == "member-pillow" and scale == 50:
            item["transform"]["translation"][1] = 0.02
    if scale == 50:
        new += hands(kept)
    return new + kept


def hands(kept):
    """🖐️ 1:50 hand skeleton: carpus and one metacarpal per finger ray up to the knuckle where the authored ray starts."""
    by_id = {item["id"]: item for item in kept}
    out = []
    for s in ("l", "r"):
        bone = f"hand-{s}"
        out.append(own(f"carpus-{s}", bone, kind="ellipsoid", radii=[0.017, 0.009, 0.027], transform={"translation": [0.016, -0.004, 0]}))
        for k in (1, 2, 3, 4):
            knuckle = by_id[f"finger-ray-{k}-{s}"]["sections"][0]["at"]
            out.append(own(f"metacarpal-{k}-{s}", bone, kind="capsule", p0=[0.026, -0.003, round(knuckle[2] * 0.6, 5)], p1=list(knuckle), r0=0.0055, r1=0.005))
    return out


GIRDLE = {
    "clavicle": {"parent": "torso", "x": 0.437, "y": -0.066, "z": 0.024, "angleDeg": 93.0941, "abductionDeg": 61.4135, "twistDeg": -176.4774, "lengthM": 0.1549, "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -30, "twistMaxDeg": 30}},
    "ilium": {"parent": "pelvis", "x": 0.07, "y": 0.055, "z": 0.045, "angleDeg": -141.8428, "abductionDeg": 25.2575, "twistDeg": -71.466, "lengthM": 0.09843, "joint": {"type": "revolute", "minDeg": -4, "maxDeg": 4}},
}
LIMBS = {
    "arm": {"parent": "clavicle", "x": 0.15633, "y": -0.03175, "z": 0.0049, "angleDeg": -88.5199, "abductionDeg": -0.8048, "twistDeg": 118.5306},
    "thigh": {"parent": "ilium", "x": 0.09843, "y": 0, "z": 0, "angleDeg": -36.8396, "abductionDeg": 30.9873, "twistDeg": 70.4069},
}
TITLES = {"clavicle-l": ("Left clavicle and shoulder blade", "Linkes Schlüsselbein mit Schulterblatt"), "clavicle-r": ("Right clavicle and shoulder blade", "Rechtes Schlüsselbein mit Schulterblatt"),
          "ilium-l": ("Left hip bone", "Linkes Hüftbein"), "ilium-r": ("Right hip bone", "Rechtes Hüftbein")}


def sided(fields, side):
    return {k: (-v if k in ("z", "abductionDeg", "twistDeg") else v) for k, v in fields.items()} if side < 0 else dict(fields)


def rig(asset):
    bones = [b for b in asset["rig"]["bones"] if b["id"] not in TITLES]
    at = [b["id"] for b in bones].index("head") + 1
    new = []
    for s, m in (("l", 1), ("r", -1)):
        for name, fields in GIRDLE.items():
            placed = sided({k: v for k, v in fields.items() if k not in ("parent", "joint", "lengthM")}, m)
            joint = dict(fields["joint"])
            if m < 0 and "twistMinDeg" in joint:
                joint["twistMinDeg"], joint["twistMaxDeg"] = -fields["joint"]["twistMaxDeg"], -fields["joint"]["twistMinDeg"]
            identity = f"{name}-{s}"
            new.append({"id": identity, "parent": fields["parent"], "title": {"en": TITLES[identity][0], "de": TITLES[identity][1]}, **placed, "lengthM": fields["lengthM"], "joint": joint})
    bones[at:at] = new
    for bone in bones:
        name, _, s = bone["id"].rpartition("-")
        if name in LIMBS and s in ("l", "r"):
            fields = sided({k: v for k, v in LIMBS[name].items() if k != "parent"}, 1 if s == "l" else -1)
            ordered = {}
            for key, value in bone.items():
                if key in ("abductionDeg", "twistDeg", "z"):
                    continue
                if key == "parent":
                    ordered["parent"] = f"{LIMBS[name]['parent']}-{s}"
                elif key == "y":
                    ordered["y"], ordered["z"] = fields["y"], fields["z"]
                elif key == "angleDeg":
                    ordered["angleDeg"], ordered["abductionDeg"], ordered["twistDeg"] = fields["angleDeg"], fields["abductionDeg"], fields["twistDeg"]
                else:
                    ordered[key] = fields.get(key, value)
            bone.clear()
            bone.update(ordered)
    asset["rig"]["bones"] = bones


def thermal(asset):
    """🌡️ The face region also holds ears and neck: 6.5 % of the DuBois area (head and neck 8.9 % with the hair), taken from the trunk share."""
    regions = {region["id"]: region for region in asset["thermal"]["regions"]}
    regions["face-skin"]["body"]["skinFraction"] = 0.065
    regions["face-skin"]["areaM2"] = 0.1132
    regions["top"]["body"]["skinFraction"] = 0.496


DECISIONS = {
    "50": ("Skeleton and girdles at 1:50: the arms hang from clavicle bones that run from the sternoclavicular joint beside the manubrium back and out to the acromion (155 mm, 0.36 m across the acromia) and carry the S-shaped clavicle, the shoulder blade with its spine, acromion, glenoid and coracoid; the thighs hang from hip bones that run from the sacroiliac joint to the hip joint (0.174 m between the hip centres) and carry the iliac wing, acetabulum, ischium and pubis meeting at the symphysis. The skull is a cranium on its base with facial skeleton, zygomatic arches and a mandible hinged below the arches; seven cervical, twelve thoracic and five lumbar bodies with their discs follow lordosis and kyphosis onto a wedge-shaped sacrum with coccyx; the femur has head, neck, trochanter and condyles with the patella on them, the tibia its plateau and medial malleolus, the fibula runs from the lateral condyle to the lateral malleolus, the hand a carpus and four metacarpals up to the knuckles.",
           "Skelett und Gürtel bei 1:50: die Arme hängen an Schlüsselbeinknochen, die vom Sternoklavikulargelenk neben dem Manubrium nach hinten außen zum Akromion laufen (155 mm, 0,36 m Schulterbreite über die Akromien) und das S-förmige Schlüsselbein sowie das Schulterblatt mit Gräte, Akromion, Gelenkpfanne und Rabenschnabelfortsatz tragen; die Oberschenkel hängen an Hüftbeinen, die vom Iliosakralgelenk zum Hüftgelenk laufen (0,174 m zwischen den Hüftzentren) und Darmbeinschaufel, Hüftpfanne, Sitzbein und Schambein bis zur Symphyse tragen. Der Schädel ist ein Hirnschädel auf seiner Basis mit Gesichtsschädel, Jochbögen und einem unter den Jochbögen eingehängten Unterkiefer; sieben Hals-, zwölf Brust- und fünf Lendenwirbelkörper mit Bandscheiben folgen Lordose und Kyphose bis auf ein keilförmiges Kreuzbein mit Steißbein; der Oberschenkelknochen hat Kopf, Hals, Rollhügel und Kondylen mit der Kniescheibe darauf, das Schienbein sein Plateau und den Innenknöchel, das Wadenbein läuft vom äußeren Kondylus zum Außenknöchel, die Hand hat Handwurzel und vier Mittelhandknochen bis zu den Knöcheln."),
    "100": ("Skeleton and girdles at 1:100: the same clavicle and hip bones as at 1:50 carry a straight clavicle, the shoulder blade as one plate with its spine, the iliac wing, acetabulum, ischium and pubic arch at 15 mm; cranium on its base with face block and mandible, the spine as one bar on the 1:50 curve, the sacrum as a wedge.",
            "Skelett und Gürtel bei 1:100: dieselben Schlüsselbein- und Hüftbeinknochen wie bei 1:50 tragen ein gerades Schlüsselbein, das Schulterblatt als eine Platte mit Gräte, Darmbeinschaufel, Hüftpfanne, Sitzbein und Schambogen in 15 mm; Hirnschädel auf seiner Basis mit Gesichtsblock und Unterkiefer, die Wirbelsäule als ein Stab auf der Kurve von 1:50, das Kreuzbein als Keil."),
    "200": ("Skeleton and girdles at 1:200: clavicle bars on the clavicle bones, each hip bone as one wing with the ischium below it, the sacrum between them, cranium and jaw as two masses, the spine as one curved bar of 30 mm; the hem bearings are blocks that reach the duvet webs.",
            "Skelett und Gürtel bei 1:200: Schlüsselbeinstäbe auf den Schlüsselbeinknochen, jedes Hüftbein als eine Schaufel mit dem Sitzbein darunter, dazwischen das Kreuzbein, Hirnschädel und Kiefer als zwei Massen, die Wirbelsäule als ein gekrümmter Stab von 30 mm; die Auflager des Saums sind Blöcke, die bis an die Stege der Decke reichen."),
    "500": ("Skeleton and girdles at 1:500: each hip bone is one 75 mm bar from the iliac crest to the ischium, the sacrum one mass between them, the clavicles slender bars from sternum to shoulder like the ribs, cranium and jaw two masses; the hem bearings are 75 mm blocks.",
            "Skelett und Gürtel bei 1:500: jedes Hüftbein ist ein Stab von 75 mm vom Darmbeinkamm zum Sitzbein, das Kreuzbein eine Masse dazwischen, die Schlüsselbeine schlanke Stäbe vom Brustbein zur Schulter wie die Rippen, Hirnschädel und Kiefer zwei Massen; die Auflager des Saums sind Blöcke von 75 mm."),
}


def decide(volumes):
    for scale, (en, de) in DECISIONS.items():
        kept = [d for d in volumes[scale]["decisions"] if not d["en"].startswith("Skeleton and girdles at")]
        volumes[scale]["decisions"] = kept + [{"en": en, "de": de}]


def main(mode):
    document = json.load(open(CATALOG))
    source = json.load(open(SOURCE))
    rig(document["asset"])
    thermal(document["asset"])
    for scale in ("50", "100", "200", "500"):
        source["volumes"][scale]["structure"] = structure(int(scale), source["volumes"][scale]["structure"])
    decide(source["volumes"])
    if mode == "publish":
        fresh = json.load(open(CATALOG))
        fresh["asset"]["rig"] = document["asset"]["rig"]
        fresh["asset"]["thermal"] = document["asset"]["thermal"]
        text = open(CATALOG).read()
        first = text.find("\n")
        indent = None if first < 0 else len(text[first + 1:]) - len(text[first + 1:].lstrip(" "))
        with open(CATALOG, "w") as handle:
            json.dump(fresh, handle, ensure_ascii=False, indent=indent)
            handle.write("\n" if text.endswith("\n") else "")
        json.dump(source, open(SOURCE, "w"), ensure_ascii=False)
        print("published rig and source")
    else:
        document["asset"]["volumes"] = source["volumes"]
        json.dump(document, open(CANDIDATE, "w"), ensure_ascii=False)
        print("candidate", CANDIDATE)


main(sys.argv[1] if len(sys.argv) > 1 else "candidate")
