"""🏇 Round 5 horse: the handcrafted volumes of every scale on the corrected rig: body from r5-horse-body.py, thermal by reference with outlets, and the skeleton as the load path (vertebral column dorsal under the topline, withers from the thoracic processes, a curved rib basket, limb bones between the real joint centres). Each station is a measured place on a 1.61 m warmblood in the rest pose (x ahead from the hip joint, y up from the ground, z lateral); see r3-animals-horse.md, "Round 5 handcraft".

Usage: python3 r5-horse-model.py [50 100 200 500]   (writes the named scales into volumes/animals/horse.json, default all)
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-horse-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
loft, along, ellipsoid, capsule, plate, hoop, SIDES, world = K.loft, K.along, K.ellipsoid, K.capsule, K.plate, K.hoop, K.SIDES, K.world

COAT, HAIR, SKIN, EYE, HORN = "short-coat", "mane-tail-hair", "bare-skin", "eye-surface", "hoof-horn"
T_COAT, T_HAIR, T_HORN, T_SKIN, T_EYE = 0.62, 0.2, 0.3, 0.22, 0.04


def tu(x, top, under, rz, e=None):
    """A vertical trunk section from its top line, its under line and its half width."""
    return (x, (top + under) / 2, 0.0, (top - under) / 2, rz, 0.0, e)


def flat(y, xc, rx, rz, z=0.0, e=None):
    """A horizontal section (hoof, thigh): centre, half length, half width."""
    return (xc, y, z, rx, rz, -90.0, e)


HIND_Z = {"thigh": 0.15, "shank": 0.19, "cannon": 0.157, "pastern": 0.147, "hoof": 0.146}
FORE_Z = {"scapula": 0.12, "humerus": 0.19, "radius": 0.186, "cannon": 0.156, "pastern": 0.144, "hoof": 0.142}
T_BONE, T_JOINT, T_MASS = 0.85, 0.5, 0.35


def st(solid, member, part="member"):
    solid["member"] = member
    solid["part"] = part
    return solid


def bone_loft(sid, bone, rows, mirror=1, exp=2.0):
    return st(loft(sid, bone, along(bone, rows), tone=T_BONE, exp=exp, mirror=mirror), bone)


def bar(sid, bone, t0, t1, r0, r1, z0=0.0, z1=None, mirror=1, lateral=None, v0=0.0, v1=0.0):
    a, c = world(bone, t0, v0), world(bone, t1, v1)
    return st(capsule(sid, bone, (a[0], a[1], z0), (c[0], c[1], z0 if z1 is None else z1), r0, r1, tone=T_BONE, mirror=mirror, lateral=lateral), bone)


def wbar(sid, bone, p0, p1, r0, r1, mirror=1, lateral=None):
    return st(capsule(sid, bone, p0, p1, r0, r1, tone=T_BONE, mirror=mirror, lateral=lateral), bone)


def joint(bone, r, z=0.0, mirror=1, tag=""):
    x, y = world(bone, 0.0, 0.0)
    return st(ellipsoid(f"j-{bone}{tag}", bone, (x, y, z), (r, r, r), tone=T_JOINT, mirror=mirror), bone, "joint")


def masses(r):
    out = []
    for bone, rr in (("thorax", r[0]), ("pelvis", r[1]), ("lumbar", r[2])):
        m = next(x for x in K.ASSET["rig"]["bones"] if x["id"] == bone)["mass"]
        x, y = world(bone, m["comX"], m["comY"])
        out.append(st(ellipsoid(f"mass-{bone}", bone, (x, y, 0.0), (rr, rr, rr), tone=T_MASS), bone, "mass"))
    return out


RIBS50 = [(1.335, 1.275, 0.945, 0.15, 8), (1.288, 1.292, 0.915, 0.165, 10), (1.24, 1.308, 0.895, 0.18, 12), (1.193, 1.322, 0.888, 0.193, 14), (1.145, 1.335, 0.884, 0.205, 16), (1.098, 1.346, 0.882, 0.216, 18), (1.05, 1.356, 0.882, 0.226, 20), (1.003, 1.365, 0.884, 0.236, 22),
          (0.955, 1.373, 0.9, 0.245, 24, 300), (0.908, 1.38, 0.915, 0.252, 26, 290), (0.86, 1.386, 0.93, 0.258, 28, 280), (0.813, 1.392, 0.95, 0.262, 30, 268), (0.765, 1.397, 0.975, 0.265, 31, 256), (0.718, 1.401, 1.0, 0.266, 32, 244), (0.67, 1.405, 1.03, 0.264, 33, 232), (0.623, 1.408, 1.065, 0.26, 34, 220), (0.575, 1.411, 1.1, 0.254, 35, 208), (0.528, 1.413, 1.14, 0.246, 36, 196)]
RIBS100 = [(1.32, 1.28, 0.935, 0.155, 9), (1.225, 1.312, 0.892, 0.184, 13), (1.13, 1.338, 0.884, 0.208, 17), (1.035, 1.36, 0.883, 0.229, 21), (0.94, 1.375, 0.905, 0.247, 25, 296), (0.845, 1.388, 0.94, 0.259, 29, 276), (0.75, 1.398, 0.985, 0.265, 32, 252), (0.655, 1.406, 1.04, 0.263, 34, 228), (0.56, 1.412, 1.11, 0.252, 36, 204)]
RIBS200 = [(1.29, 1.29, 0.92, 0.165, 10), (1.12, 1.34, 0.89, 0.21, 18), (0.95, 1.374, 0.905, 0.245, 25, 296), (0.78, 1.395, 0.97, 0.262, 31, 258), (0.6, 1.41, 1.08, 0.255, 35, 214)]
RIBS500 = [(1.2, 1.32, 0.91, 0.19, 14), (0.8, 1.393, 0.97, 0.255, 30, 262)]
SPINES50 = [(1.335, 1.42, 0.015), (1.288, 1.5, 0.04), (1.24, 1.565, 0.06), (1.193, 1.595, 0.075), (1.145, 1.6, 0.08), (1.098, 1.586, 0.08), (1.05, 1.568, 0.075), (1.003, 1.548, 0.065), (0.955, 1.542, 0.055), (0.908, 1.525, 0.045), (0.86, 1.51, 0.035), (0.813, 1.5, 0.025), (0.765, 1.495, 0.015), (0.718, 1.492, 0.008), (0.67, 1.49, 0.0), (0.623, 1.49, -0.005), (0.575, 1.492, -0.008), (0.528, 1.494, -0.01),
            (0.47, 1.5, -0.012), (0.42, 1.505, -0.012), (0.37, 1.512, -0.012), (0.32, 1.52, -0.012), (0.27, 1.53, -0.01), (0.22, 1.545, -0.008)]


def spine_y(x):
    """🦴 Height of the centre of the vertebral bodies: sacrum falling with the croup, lumbar lordosis, thoracic column sinking between the shoulder blades to T1."""
    pts = [(-0.24, 1.415), (-0.1, 1.44), (0.05, 1.456), (0.17, 1.46), (0.33, 1.432), (0.5, 1.39), (0.7, 1.368), (0.9, 1.345), (1.1, 1.31), (1.25, 1.275), (1.36, 1.245)]
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return pts[-1][1]


def trunk_frame(lod):
    """The axial skeleton behind the neck: vertebral column, dorsal processes, ribs, sternum, pelvis girdle."""
    f = (0.0375, 0.015, 0.0075, 0.00375)[lod]
    rb = max(0.03, f)
    out = []
    col = lambda xs, r: [(x, spine_y(x), 0.0, max(r, f), max(r * 1.15, f), 0.0 if i in (0, len(xs) - 1) else None) for i, x in enumerate(xs)]
    if lod == 3:
        xs = [1.36 - 0.0478 * i for i in range(19)] + [0.5 - 0.055 * i for i in range(1, 7)]
        for i, (a, c) in enumerate(zip(xs, xs[1:])):
            bone, tag = ("thorax", f"t{i + 1}") if i < 18 else ("lumbar", f"l{i - 17}")
            xm = (a + c) / 2
            out.append(st(ellipsoid(f"vertebra-{tag}", bone, (xm, spine_y(xm), 0.0), (0.0235 if i < 18 else 0.0245, 0.027 if i < 18 else 0.03, 0.03 if i < 18 else 0.034), tone=T_BONE, angle=K.math.atan2(spine_y(a) - spine_y(c), a - c) / K.RAD), bone))
    else:
        out.append(st(loft("spine-thoracic", "thorax", col([0.5, 0.7, 0.9, 1.1, 1.25, 1.36], rb), tone=T_BONE, exp=2.0), "thorax"))
        out.append(st(loft("spine-lumbar", "lumbar", col([0.17, 0.33, 0.5], rb * 1.12), tone=T_BONE, exp=2.0), "lumbar"))
    out.append(st(loft("sacrum", "pelvis", [(0.17, spine_y(0.17), 0.0, max(0.034, f), max(0.045, f), 180.0), (0.05, spine_y(0.05), 0.0, max(0.03, f), max(0.04, f), 180.0), (-0.1, spine_y(-0.1), 0.0, max(0.02, f), max(0.024, f), 180.0), (-0.24, spine_y(-0.24), 0.0, max(0.016, f), max(0.018, f), 180.0)], tone=T_BONE, exp=2.0), "pelvis"))
    if lod == 3:
        for i, (x, tip, lean) in enumerate(SPINES50):
            bone = "thorax" if i < 18 else "lumbar"
            out.append(wbar(f"process-{'t' if i < 18 else 'l'}{i + 1 if i < 18 else i - 17}", bone, (x, spine_y(x) + 0.018, 0.0), (x - lean, tip - 0.014, 0.0), 0.015, 0.012, lateral=0.5))
        out.append(st(loft("lumbar-wings", "lumbar", [(0.19, spine_y(0.19), 0.0, 0.008, 0.11, 0.0), (0.34, spine_y(0.34), 0.0, 0.008, 0.125, 0.0), (0.49, spine_y(0.49), 0.0, 0.008, 0.11, 0.0)], tone=T_BONE, exp=4.0), "lumbar"))
        out.append(wbar("sacral-crest", "pelvis", (0.17, 1.5, 0.0), (-0.08, 1.5, 0.0), 0.016, 0.012, lateral=0.6))
    else:
        t = max(0.009, f)
        fin = [(1.37, 1.31), (1.335, 1.42), (1.22, 1.58), (1.12, 1.6), (1.0, 1.56), (0.86, 1.51), (0.7, 1.49), (0.52, 1.494), (0.52, 1.43), (0.9, 1.41), (1.2, 1.36)]
        out.append(st(plate("dorsal-processes", "thorax", fin if lod > 0 else [(1.36, 1.31), (1.2, 1.59), (1.02, 1.59), (0.52, 1.495), (0.52, 1.43), (1.1, 1.38)], -t, t, tone=T_BONE), "thorax"))
        if lod > 0:
            out.append(st(plate("lumbar-processes", "lumbar", [(0.52, 1.495), (0.36, 1.512), (0.2, 1.548), (0.2, 1.46), (0.52, 1.43)], -t, t, tone=T_BONE), "lumbar"))
    ribs = (RIBS500, RIBS200, RIBS100, RIBS50)[lod]
    half = ((0.0375, 0.0375), (0.016, 0.015), (0.013, 0.01), (0.011, 0.008))[lod]
    for i, row in enumerate(ribs):
        x, _, bottom, hw, tilt = row[:5]
        top = spine_y(x) + 0.01
        r = (top - bottom) / 2
        out.append(st(hoop(f"rib-{i + 1}", "thorax", (x - r * K.math.sin(tilt * K.RAD), top - r * K.math.cos(tilt * K.RAD)), r, hw - half[1] - 0.012, half, tilt=tilt + K.F["thorax"][2], sweep=row[5] if len(row) > 5 else None, tone=T_BONE), "thorax"))
    if lod > 0:
        out.append(st(loft("sternum", "thorax", [(1.4, 1.07, 0.0, max(0.02, f), max(0.022, f)), (1.3, 0.97, 0.0, max(0.02, f), max(0.026, f)), (1.2, 0.915, 0.0, max(0.018, f), max(0.028, f)), (1.02, 0.892, 0.0, max(0.014, f), max(0.03, f))], tone=T_BONE, exp=2.0), "thorax"))
    for name, sfx, m in SIDES:
        if lod > 0:
            out.append(st(loft(f"{name}-ilium", "pelvis", [(0.0, 1.314, 0.135, max(0.034, f), max(0.03, f), 50.0), (0.06, 1.385, 0.13, max(0.026, f), max(0.026, f), 50.0), (0.12, 1.455, 0.124, max(0.016, f), max(0.055, f), 50.0), (0.165, 1.508, 0.122, max(0.012, f), max(0.074, f), 50.0), (0.185, 1.53, 0.118, max(0.012, f), max(0.066, f), 50.0)], tone=T_BONE, exp=2.6, mirror=m), "pelvis"))
            out.append(wbar(f"{name}-iliac-crest", "pelvis", (0.15, 1.54, 0.045), (0.245, 1.45, 0.195), max(0.016, f), max(0.02, f), mirror=m))
            out.append(wbar(f"{name}-ischium", "pelvis", (0.0, 1.314, 0.135), (-0.25, 1.3, 0.11), max(0.022, f), max(0.032, f), mirror=m))
        else:
            out.append(wbar(f"{name}-hip-bone", "pelvis", (0.2, 1.46, 0.15), (-0.2, 1.29, 0.11), 0.0375, 0.0375, mirror=m))
    if lod == 3:
        out.append(wbar("pubis", "pelvis", (0.0, 1.262, -0.12), (0.0, 1.262, 0.12), 0.018, 0.018))
        out.append(wbar("ischial-arch", "pelvis", (-0.22, 1.27, -0.1), (-0.22, 1.27, 0.1), 0.016, 0.016))
    return out


def head_neck_frame(lod):
    f = (0.0375, 0.015, 0.0075, 0.00375)[lod]
    out = []
    g = lambda r: max(r, f)
    out.append(st(loft("skull", "head", along("head", [(0.0, -0.022, g(0.045), g(0.05)), (0.08, -0.03, g(0.07), g(0.066)), (0.17, -0.036, g(0.07), g(0.072)), (0.25, -0.034, g(0.055), g(0.052)), (0.4, -0.026, g(0.04), g(0.036)), (0.52, -0.03, g(0.034), g(0.034)), (0.578, -0.04, g(0.024), g(0.03))] if lod > 0 else [(0.0, -0.03, 0.06, 0.055), (0.15, -0.04, 0.075, 0.07), (0.57, -0.035, 0.0375, 0.0375)]), tone=T_BONE, exp=2.0), "head"))
    if lod == 3:
        for name, sfx, m in SIDES:
            out.append(st(loft(f"{name}-mandible", "jaw", along("jaw", [(-0.09, -0.03, 0.045, 0.011, 0.058), (-0.03, -0.06, 0.07, 0.011, 0.06), (0.06, -0.07, 0.048, 0.011, 0.054), (0.18, -0.056, 0.024, 0.011, 0.038), (0.3, -0.047, 0.018, 0.011, 0.022), (0.4, -0.046, 0.016, 0.013, 0.012)]), tone=T_BONE, exp=2.0, mirror=m), "jaw"))
            out.append(st(ellipsoid(f"{name}-orbit", "head", world("head", 0.2, 0.002) + (0.078,), (0.03, 0.026, 0.014), tone=T_BONE, angle=K.F["head"][2], mirror=m), "head"))
    else:
        out.append(st(loft("mandible", "jaw", along("jaw", [(-0.09, -0.03, g(0.045), g(0.05)), (-0.03, -0.06, g(0.068), g(0.058)), (0.12, -0.062, g(0.036), g(0.046)), (0.4, -0.046, g(0.016), g(0.02))]), tone=T_BONE, exp=2.0), "jaw"))
    w = lambda bone, t: world(bone, t * K.LEN[bone], 0.0)
    cerv = [("c1-atlas", "neck-3", w("neck-3", 0.98), w("neck-3", 0.78), 0.034, 1.7), ("c2-axis", "neck-3", w("neck-3", 0.72), w("neck-3", 0.04), 0.03, 1.2), ("c3", "neck-2", w("neck-2", 0.97), w("neck-2", 0.52), 0.03, 1.3), ("c4", "neck-2", w("neck-2", 0.46), w("neck-2", 0.02), 0.031, 1.3),
            ("c5", "neck-1", w("neck-1", 0.98), w("neck-1", 0.68), 0.032, 1.35), ("c6", "neck-1", w("neck-1", 0.64), w("neck-1", 0.36), 0.033, 1.4), ("c7", "neck-1", w("neck-1", 0.32), w("neck-1", 0.03), 0.033, 1.35)]
    if lod >= 2:
        for i, bone, a, c, r, lat in cerv:
            out.append(wbar(i, bone, a + (0.0,), c + (0.0,), g(r), g(r), lateral=lat))
    else:
        for i, bone, a, c in (("cervical-upper", "neck-3", w("neck-3", 0.98), w("neck-3", 0.04)), ("cervical-middle", "neck-2", w("neck-2", 0.97), w("neck-2", 0.02)), ("cervical-lower", "neck-1", w("neck-1", 0.98), w("neck-1", 0.03))):
            out.append(wbar(i, bone, a + (0.0,), c + (0.0,), g(0.032), g(0.032)))
    for name, sfx, m in SIDES:
        out.append(bar(f"{name}-ear-cartilage", f"ear-{sfx}", 0.0, 0.14, g(0.009) if lod > 0 else 0.0375, g(0.005) if lod > 0 else 0.0375, 0.058, 0.075, mirror=m))
    tail = [(0.02, 0.016), (0.016, 0.009), (0.006, 0.005), (0.005, 0.004)]
    for i, (r0, r1) in enumerate(tail):
        bone = f"tail-{i + 1}"
        out.append(bar(f"caudal-{i + 1}" if i < 2 else f"tail-hair-core-{i - 1}", bone, 0.0, K.LEN[bone], g(r0), g(r1)))
    return out


def limb_frame(lod):
    f = (0.0375, 0.015, 0.0075, 0.00375)[lod]
    g = lambda r: max(r, f)
    out = []
    for name, sfx, m in SIDES:
        sc, hu, ra, cf, pf, hf = (f"{k}-f{sfx}" for k in ("scapula", "humerus", "radius", "cannon", "pastern", "hoof"))
        th, sh, ch, ph, hh = (f"{k}-h{sfx}" for k in ("thigh", "shank", "cannon", "pastern", "hoof"))
        if lod == 0:
            L = K.LEN
            out.append(bar(f"{name}-scapula", sc, 0.06, L[sc], f, f, 0.12, 0.19, mirror=m))
            out.append(bar(f"{name}-humerus", hu, 0.0, L[hu], f, f, 0.19, 0.185, mirror=m))
            out.append(bar(f"{name}-radius", ra, 0.0, L[ra], f, f, 0.185, 0.155, mirror=m))
            out.append(bar(f"{name}-fore-cannon", cf, 0.0, L[cf], f, f, 0.155, 0.144, mirror=m))
            out.append(bar(f"{name}-fore-pastern", pf, 0.0, 0.11, f, f, 0.144, 0.142, mirror=m))
            out.append(bar(f"{name}-femur", th, 0.0, L[th], f, f, 0.15, 0.19, mirror=m))
            out.append(bar(f"{name}-tibia", sh, 0.0, L[sh], f, f, 0.19, 0.156, mirror=m))
            out.append(bar(f"{name}-hind-cannon", ch, 0.0, L[ch], f, f, 0.156, 0.147, mirror=m))
            out.append(bar(f"{name}-hind-pastern", ph, 0.0, 0.12, f, f, 0.147, 0.146, mirror=m))
            continue
        out.append(st(loft(f"{name}-scapula", sc, along(sc, [(0.03, 0.0, 0.07, g(0.008), 0.11), (0.16, 0.0, 0.076, g(0.008), 0.15), (0.34, 0.0, 0.042, g(0.01), 0.18), (0.43, 0.0, 0.028, g(0.016), 0.188), (0.483, 0.0, 0.035, g(0.026), 0.19)]), tone=T_BONE, exp=2.0, mirror=m), sc))
        if lod == 3:
            out.append(bar(f"{name}-scapular-spine", sc, 0.06, 0.4, 0.008, 0.008, 0.125, 0.19, mirror=m, v0=0.008, v1=0.004))
        out.append(st(loft(f"{name}-humerus", hu, along(hu, [(-0.005, 0.0, g(0.036), g(0.036), 0.19), (0.05, -0.006, g(0.04), g(0.034), 0.19), (0.13, 0.0, g(0.024), g(0.024), 0.19), (0.255, 0.0, g(0.024), g(0.024), 0.188), (0.33, 0.0, g(0.036), g(0.04), 0.186)]), tone=T_BONE, exp=2.0, mirror=m), hu))
        out.append(st(loft(f"{name}-radius", ra, along(ra, [(0.0, 0.0, g(0.034), g(0.04), 0.186), (0.08, 0.0, g(0.023), g(0.03), 0.18), (0.3, 0.0, g(0.021), g(0.028), 0.162), (0.37, 0.0, g(0.03), g(0.038), 0.156)]), tone=T_BONE, exp=2.0, mirror=m), ra))
        if lod >= 2:
            out.append(wbar(f"{name}-olecranon", ra, (1.145, 0.87, 0.186), (1.1, 0.935, 0.188), g(0.02), g(0.016), mirror=m))
        out.append(st(loft(f"{name}-carpus", cf, along(cf, [(-0.035, 0.0, g(0.03), g(0.04), 0.156), (0.0, 0.0, g(0.034), g(0.044), 0.155), (0.04, 0.0, g(0.028), g(0.038), 0.154)]), tone=T_BONE, exp=3.0, mirror=m), cf))
        out.append(st(loft(f"{name}-fore-cannon-bone", cf, along(cf, [(0.03, 0.0, g(0.022), g(0.026), 0.154), (0.12, 0.0, g(0.017), g(0.02), 0.151), (0.25, 0.0, g(0.017), g(0.02), 0.147), (0.303, 0.0, g(0.025), g(0.028), 0.145)]), tone=T_BONE, exp=2.0, mirror=m), cf))
        out.append(st(loft(f"{name}-femur", th, along(th, [(-0.015, 0.0, g(0.03), g(0.03), 0.15), (0.03, -0.012, g(0.045), g(0.04), 0.165), (0.12, 0.0, g(0.027), g(0.027), 0.17), (0.3, 0.0, g(0.027), g(0.027), 0.182), (0.385, 0.0, g(0.045), g(0.046), 0.188), (0.425, 0.0, g(0.036), g(0.044), 0.19)]), tone=T_BONE, exp=2.0, mirror=m), th))
        out.append(st(loft(f"{name}-tibia", sh, along(sh, [(0.0, -0.006, g(0.036), g(0.042), 0.19), (0.07, -0.004, g(0.026), g(0.028), 0.186), (0.26, 0.0, g(0.022), g(0.022), 0.172), (0.415, -0.004, g(0.022), g(0.034), 0.158)]), tone=T_BONE, exp=2.0, mirror=m), sh))
        out.append(st(ellipsoid(f"{name}-tarsus", ch, world(ch, 0.025, -0.004) + (0.157,), (g(0.045), g(0.036), g(0.038)), tone=T_BONE, angle=K.F[ch][2], mirror=m), ch))
        if lod >= 2:
            out.append(st(ellipsoid(f"{name}-patella", th, (0.212, 0.975, 0.19), (g(0.02), g(0.03), g(0.024)), tone=T_BONE, mirror=m), th))
            out.append(wbar(f"{name}-calcaneus", ch, (-0.05, 0.545, 0.157), (-0.105, 0.618, 0.157), g(0.02), g(0.014), mirror=m))
        out.append(st(loft(f"{name}-hind-cannon-bone", ch, along(ch, [(0.05, 0.0, g(0.026), g(0.026), 0.155), (0.16, 0.0, g(0.019), g(0.021), 0.151), (0.31, 0.0, g(0.019), g(0.021), 0.148), (0.355, 0.0, g(0.027), g(0.03), 0.147)]), tone=T_BONE, exp=2.0, mirror=m), ch))
        for tag, pb, hb, z, tip in (("fore", pf, hf, 0.143, 1.2485), ("hind", ph, hh, 0.146, 0.0495)):
            n = K.LEN[pb]
            if lod == 3:
                x, y = world(pb, 0.0, -0.036)
                out.append(st(ellipsoid(f"{name}-{tag}-sesamoids", pb, (x, y, z), (0.014, 0.02, 0.026), tone=T_BONE, mirror=m), pb))
                out.append(bar(f"{name}-{tag}-long-pastern", pb, 0.012, n * 0.62, 0.019, 0.016, z, mirror=m))
                out.append(bar(f"{name}-{tag}-short-pastern", pb, n * 0.72, n - 0.002, 0.018, 0.019, z, mirror=m))
            else:
                out.append(bar(f"{name}-{tag}-pastern-bones", pb, 0.012, n - 0.004, g(0.019), g(0.018), z, mirror=m))
            out.append(st(loft(f"{name}-{tag}-coffin-bone", hb, [flat(0.01, tip - 0.004, g(0.046), g(0.042), z), flat(0.04, tip - 0.018, g(0.032), g(0.032), z), flat(0.066, tip - 0.032, g(0.018), g(0.022), z)], tone=T_BONE, exp=2.2, mirror=m), hb))
    return out


JOINT_R = {"pelvis": 0.03, "lumbar": 0.03, "thorax": 0.028, "neck-1": 0.03, "neck-2": 0.028, "neck-3": 0.028, "head": 0.028, "jaw": 0.016, "ear": 0.009, "tail-1": 0.018, "tail-2": 0.015, "tail-3": 0.008, "tail-4": 0.006,
           "scapula": 0.016, "humerus": 0.034, "radius": 0.034, "cannon-f": 0.03, "pastern-f": 0.026, "hoof-f": 0.02, "thigh": 0.034, "shank": 0.04, "cannon-h": 0.034, "pastern-h": 0.027, "hoof-h": 0.02}


def joints(lod):
    f = (0.045, 0.02, 0.0075, 0.00375)[lod]
    out = []
    axial = ["pelvis", "lumbar", "thorax", "neck-1", "neck-2", "neck-3", "head", "jaw", "tail-1", "tail-2", "tail-3", "tail-4"]
    if lod == 0:
        axial = ["neck-1", "head"]
    elif lod == 1:
        axial = ["lumbar", "thorax", "neck-1", "neck-2", "neck-3", "head", "jaw", "tail-1"]
    for bone in axial:
        x, y = world(bone, 0.0, 0.0)
        s = ellipsoid(f"j-{bone}", bone, (x, y, 0.0), (max(JOINT_R[bone], f),) * 3, tone=T_JOINT)
        out.append(st(s, bone, "joint"))
    for name, sfx, m in SIDES:
        if lod >= 2:
            out.append(joint(f"ear-{sfx}", max(JOINT_R["ear"], f), 0.058, m))
        for k, z in FORE_Z.items():
            if (lod <= 1 and k in ("scapula", "hoof")) or (lod == 0 and k == "pastern"):
                continue
            if k == "scapula":
                x, y = world(f"scapula-f{sfx}", 0.06, 0.0)
                out.append(st(ellipsoid(f"j-scapula-f{sfx}", f"scapula-f{sfx}", (x, y, 0.1), (max(0.014, f),) * 3, tone=T_JOINT, mirror=m), f"scapula-f{sfx}", "joint"))
                continue
            out.append(joint(f"{k}-f{sfx}", max(JOINT_R.get(k, JOINT_R.get(k + "-f")), f), z, m))
        for k, z in HIND_Z.items():
            if (lod <= 1 and k == "hoof") or (lod == 0 and k == "pastern"):
                continue
            out.append(joint(f"{k}-h{sfx}", max(JOINT_R.get(k, JOINT_R.get(k + "-h")), f), z, m))
    return out


def structure(lod):
    return trunk_frame(lod) + head_neck_frame(lod) + limb_frame(lod) + joints(lod) + masses(((0.075, 0.065, 0.055), (0.07, 0.06, 0.05), (0.065, 0.056, 0.046), (0.06, 0.052, 0.043))[lod])


def emitters(n):
    f = n * 0.00015
    a = K.F["head"][2]
    out = []
    for name, sfx, m in SIDES:
        s = ellipsoid(f"th-breath-nose-{name}", "head", world("head", 0.556, 0.006) + (0.057,), (0.018 + f, 0.01 + f / 2, 0.01 + f / 2), material=SKIN, tone=0.8, angle=a + 25, mirror=m)
        s.update({"region": "coat-head", "tags": ["emitter", "breath-nose"]})
        out.append(s)
    x, y = world("thorax", 0.44, 0.2)
    s = ellipsoid("th-plume-back", "thorax", (x, 1.5, 0.0), (0.3 - 2 * f, max(0.012, f / 2), 0.13 - f), tone=0.8)
    s.update({"region": "coat-trunk", "tags": ["emitter", "plume-back"]})
    out.append(s)
    return out


def thermal_only(n):
    """Regions the body of a coarse scale no longer shows, as their own thermal solids at the size floor."""
    f = n * 0.00015
    a = K.F["head"][2]
    out = []
    reg = lambda s, r, layer=None: (s.update({"region": r, **({"layer": layer} if layer else {})}), s)[1]
    if n >= 200:
        for name, sfx, m in SIDES:
            out.append(reg(ellipsoid(f"th-{name}-eye", "head", world("head", 0.2, 0.0) + (0.1,), (f / 2, f / 2, f / 2), material=EYE, tone=T_EYE, mirror=m), "eyes"))
    if n >= 500:
        out.append(reg(ellipsoid("th-muzzle", "head", world("head", 0.55, -0.045) + (0.0,), (0.04, 0.055, 0.045), material=SKIN, tone=T_SKIN, angle=a), "muzzle-skin"))
        out.append(reg(loft("th-mane", "neck-2", [(1.3, 1.6, 0.0, 0.05, 0.0375, 44.0), (1.6, 1.875, 0.0, 0.055, 0.0375, 50.0), (1.88, 2.05, 0.0, 0.04, 0.0375, 30.0)], material=HAIR, tone=T_HAIR), "mane"))
        for name, sfx, m in SIDES:
            out.append(reg(loft(f"th-{name}-fore-hoof", f"cannon-f{sfx}", [flat(0.0, 1.2485, 0.067, 0.064, 0.142), flat(0.08, 1.214, 0.05, 0.05, 0.142)], material=HORN, tone=T_HORN, mirror=m), "hooves", "horn"))
            out.append(reg(loft(f"th-{name}-hind-hoof", f"cannon-h{sfx}", [flat(0.0, 0.0495, 0.0625, 0.06, 0.146), flat(0.08, 0.018, 0.047, 0.046, 0.146)], material=HORN, tone=T_HORN, mirror=m), "hooves", "horn"))
    return out


def region_of(s):
    i, bone = s["id"], s["bone"]
    if s["material"] == HAIR:
        return ("tail" if bone.startswith("tail") else "mane"), None
    if s["material"] == HORN:
        return "hooves", "horn"
    if s["material"] == EYE:
        return "eyes", None
    if i == "muzzle":
        return "muzzle-skin", None
    if "nostril" in i:
        return None, None
    if bone.startswith("ear"):
        return "ears", "coat"
    if bone in ("head", "jaw"):
        return "coat-head", "coat"
    if bone in ("pelvis", "lumbar", "thorax") or bone.startswith("neck"):
        return "coat-trunk", "coat"
    if bone[:5] in ("scapu", "humer", "radiu") or bone.startswith(("cannon-f", "pastern-f")):
        return "coat-forelimbs", "coat"
    return "coat-hindlimbs", "coat"


def thermal_refs(body):
    out = []
    for s in body:
        region, layer = region_of(s)
        if region is None:
            continue
        ref = {"id": "th-" + s["id"], "of": s["id"], "region": region}
        if layer:
            ref["layer"] = layer
        out.append(ref)
    return out


DECISIONS = {
    50: [
        {"en": "Body at 1:50: a 1.61 m warmblood modelled mass by mass: croup, loin and barrel as three overlapping lofts with a separate withers ridge, points of the hip and two breast muscles; shoulder blade, upper arm and triceps, forearm, knee with the accessory carpal, cannon, fetlock, pastern and hoof on every foreleg; thigh with stifle, gaskin, hock with its point, cannon, fetlock, pastern and hoof on every hind leg; a neck in three overlapping pieces that is 0.51 m deep at the shoulder and 0.2 m at the throat; a head with jaw, brows, cheek crests, eyes, nostrils and a bare muzzle pad; ears with their hollows, forelock, a mane lying on the near side and a tail in four pieces.", "de": "Körper bei 1:50: ein Warmblut von 1,61 m, Masse für Masse modelliert: Kruppe, Lende und Rumpf als drei überlappende Lofts mit eigenem Widerristkamm, Hüfthöckern und zwei Brustmuskeln; Schulterblatt, Oberarm und Trizeps, Unterarm, Vorderfußwurzel mit Erbsenbein, Röhre, Fesselkopf, Fessel und Huf an jedem Vorderbein; Oberschenkel mit Knie, Unterschenkel, Sprunggelenk mit Fersenhöcker, Röhre, Fesselkopf, Fessel und Huf an jedem Hinterbein; ein Hals in drei überlappenden Stücken, an der Schulter 0,51 m und an der Kehle 0,2 m tief; ein Kopf mit Unterkiefer, Augenbögen, Jochleisten, Augen, Nüstern und nacktem Maulspiegel; Ohren mit Muschel, Schopf, eine zur nahen Seite fallende Mähne und ein Schweif in vier Stücken."},
        {"en": "Thermal at 1:50: every region is the surface of its body solids by reference, the coat regions with their single coat layer and the hooves with their horn layer; the breath leaves through two outlet patches on the nostrils and the plume of the back through a patch lying on the back behind the withers.", "de": "Thermik bei 1:50: jede Region ist die Oberfläche ihrer Körper per Verweis, die Fellregionen mit ihrer einen Fellschicht und die Hufe mit ihrer Hornschicht; der Atem tritt durch zwei Auslassflächen auf den Nüstern aus und die Rückenfahne durch eine Fläche, die hinter dem Widerrist auf dem Rücken liegt."},
        {"en": "Structure at 1:50: the skeleton: skull with orbits and two mandible halves, seven cervical vertebrae, the thoracic and lumbar column with 24 dorsal processes and the lumbar transverse processes, 18 pairs of ribs as hoops (eight closed at the sternum, ten open below), sternum, sacrum, iliac crests, ilia, ischia and pubis, shoulder blades with their spines, humeri, radii with olecranon, carpi, cannon bones, sesamoids, long and short pastern bones, coffin bones, femora, patellae, tibiae, tarsi with calcaneus, the caudal vertebrae and thin hair cores for the two lower tail bones; joints as small balls of the size of the real joint and three masses at the centres of trunk, loin and pelvis. The asset declares no structural supports, so there are no support solids.", "de": "Tragwerk bei 1:50: das Skelett: Schädel mit Augenhöhlen und zwei Unterkieferhälften, sieben Halswirbel, Brust- und Lendenwirbelsäule mit 24 Dornfortsätzen und den Querfortsätzen der Lende, 18 Rippenpaare als Reifen (acht am Brustbein geschlossen, zehn unten offen), Brustbein, Kreuzbein, Darmbeinkämme, Darmbeine, Sitzbeine und Schambein, Schulterblätter mit Gräte, Oberarmbeine, Speichen mit Ellbogenhöcker, Vorderfußwurzeln, Röhrbeine, Gleichbeine, Fessel- und Kronbeine, Hufbeine, Oberschenkelbeine, Kniescheiben, Schienbeine, Sprunggelenke mit Fersenbein, die Schweifwirbel und dünne Haarkerne für die zwei unteren Schweifknochen; Gelenke als kleine Kugeln in der Größe des echten Gelenks und drei Massen in den Schwerpunkten von Rumpf, Lende und Becken. Das Asset nennt keine Auflager, daher gibt es keine Auflagerkörper."},
    ],
    100: [
        {"en": "Body at 1:100: drawn anew with the 15 mm limit: the withers are part of the barrel, triceps and upper arm are one mass, the stifle belongs to the thigh, knee, hock with its point and the fetlocks are swellings of the cannon and pastern lofts; brows, cheek crests, nostrils, ear hollows, hip points and breast muscles are left out; eyes, muzzle pad, forelock, three mane pieces and four tail pieces stay.", "de": "Körper bei 1:100: mit der Grenze von 15 mm neu gezeichnet: der Widerrist gehört zum Rumpf, Trizeps und Oberarm sind eine Masse, das Knie gehört zum Oberschenkel, Vorderfußwurzel, Sprunggelenk mit Fersenhöcker und die Fesselköpfe sind Verdickungen der Röhren- und Fessel-Lofts; Augenbögen, Jochleisten, Nüstern, Ohrmuscheln, Hüfthöcker und Brustmuskeln entfallen; Augen, Maulspiegel, Schopf, drei Mähnenstücke und vier Schweifstücke bleiben."},
        {"en": "Thermal at 1:100: the regions refer to the merged body solids; the outlet patches of breath and plume are sized for this scale.", "de": "Thermik bei 1:100: die Regionen verweisen auf die zusammengefassten Körper; die Auslassflächen von Atem und Fahne sind für diesen Maßstab bemessen."},
        {"en": "Structure at 1:100: the skeleton with nine rib hoops, the dorsal processes as two fins, one mandible, seven cervical vertebrae, pelvis without pubis, and pastern bones merged per leg; sesamoids, orbits and scapular spines are left out.", "de": "Tragwerk bei 1:100: das Skelett mit neun Rippenreifen, den Dornfortsätzen als zwei Kämmen, einem Unterkiefer, sieben Halswirbeln, Becken ohne Schambein und je Bein zusammengefassten Zehenknochen; Gleichbeine, Augenhöhlen und Schulterblattgräten entfallen."},
    ],
    200: [
        {"en": "Body at 1:200: main masses with the 30 mm limit: three trunk lofts, three neck pieces, head, jaw and muzzle, two ears, one mane sheet over the whole crest, four tail pieces and every leg in its five or six segments with three sections each; eyes and forelock are no longer solids of the body.", "de": "Körper bei 1:200: Hauptmassen mit der Grenze von 30 mm: drei Rumpf-Lofts, drei Halsstücke, Kopf, Unterkiefer und Maul, zwei Ohren, ein Mähnenblatt über den ganzen Kamm, vier Schweifstücke und jedes Bein in seinen fünf oder sechs Abschnitten mit je drei Schnitten; Augen und Schopf sind keine Körper mehr."},
        {"en": "Thermal at 1:200: body references, and the eyes as two balls at the size floor because the body no longer shows them.", "de": "Thermik bei 1:200: Körperverweise, dazu die Augen als zwei Kugeln an der Größengrenze, weil der Körper sie nicht mehr zeigt."},
        {"en": "Structure at 1:200: spine in three pieces with two fins, five rib hoops, skull, mandible, the neck column in three bars, ilium and ischium, the long bones of every leg, tarsus, carpus, merged pastern bones and coffin bones; joints only where the limbs bend.", "de": "Tragwerk bei 1:200: Wirbelsäule in drei Stücken mit zwei Kämmen, fünf Rippenreifen, Schädel, Unterkiefer, die Halswirbelsäule in drei Stäben, Darm- und Sitzbein, die Röhrenknochen jedes Beins, Sprunggelenk, Vorderfußwurzel, zusammengefasste Zehenknochen und Hufbeine; Gelenke nur dort, wo die Gliedmaßen beugen."},
    ],
    500: [
        {"en": "Body at 1:500: the horse at a glance with the 75 mm limit: croup, loin and barrel, a neck in two pieces, one head mass, two ears, the tail, and each leg as shoulder or thigh mass, forearm or gaskin and one lower piece that carries cannon, pastern and hoof.", "de": "Körper bei 1:500: das Pferd auf einen Blick mit der Grenze von 75 mm: Kruppe, Lende und Rumpf, ein Hals in zwei Stücken, eine Kopfmasse, zwei Ohren, der Schweif und jedes Bein als Schulter- oder Schenkelmasse, Unterarm oder Unterschenkel und ein unteres Stück, das Röhre, Fessel und Huf trägt."},
        {"en": "Thermal at 1:500: body references, and mane, muzzle, eyes and the four hooves as their own solids at the size floor so that every region keeps its place.", "de": "Thermik bei 1:500: Körperverweise, dazu Mähne, Maul, Augen und die vier Hufe als eigene Körper an der Größengrenze, damit jede Region ihren Ort behält."},
        {"en": "Structure at 1:500: the load path only: the spine as three beams with the withers fin, two rib hoops, hip bones, skull and mandible, neck bars, and every leg as bars from shoulder blade or femur to the pastern, with joints at the main limb joints.", "de": "Tragwerk bei 1:500: nur der Lastpfad: die Wirbelsäule als drei Balken mit dem Widerristkamm, zwei Rippenreifen, Hüftbeine, Schädel und Unterkiefer, Halsstäbe und jedes Bein als Stäbe vom Schulterblatt oder Oberschenkelbein bis zur Fessel, mit Gelenken an den Hauptgelenken der Gliedmaßen."},
    ],
}

r5 = importlib.util.spec_from_file_location("r5body", os.path.join(HERE, "r5-horse-body.py"))
R5 = importlib.util.module_from_spec(r5)
r5.loader.exec_module(R5)
BODY = {n: (lambda n=n: R5.body(n)) for n in (50, 100, 200, 500)}
LOD = {50: 3, 100: 2, 200: 1, 500: 0}


def main():
    path = os.path.join(HERE, "volumes", "animals", "horse.json")
    doc = json.load(open(path, encoding="utf-8"))
    scales = [int(a) for a in sys.argv[1:]] or sorted(BODY)
    for n in scales:
        vol = doc["volumes"][str(n)]
        vol["body"] = BODY[n]()
        vol["thermal"] = thermal_refs(vol["body"]) + thermal_only(n) + emitters(n)
        vol["structure"] = structure(LOD[n])
        vol["decisions"] = DECISIONS[n]
        print(n, {k: len(vol[k]) for k in ("body", "thermal", "structure")})
    json.dump(doc, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
