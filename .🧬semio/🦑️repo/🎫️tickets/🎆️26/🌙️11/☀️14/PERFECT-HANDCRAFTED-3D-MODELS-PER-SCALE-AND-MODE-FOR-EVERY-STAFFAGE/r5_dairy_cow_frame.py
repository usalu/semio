"""🦴 The skeleton of the dairy cow as the structure list of every scale: skull, mandible, vertebral column with its processes, thirteen pairs of ribs, sternum, pelvis girdle, limb bones down to the claw bones, caudal vertebrae, the cartilage of the ears and the median ligament that carries the udder; joints at the rig joints, masses at the centres of mass of pelvis, loin and chest.

`lod` 0 = 1:50 (every bone), 1 = 1:100, 2 = 1:200, 3 = 1:500 (the load path as bars). World points are (x ahead of the hip joint, h above the ground, z to the left).
@see ./r3_dairy_cow_body.py
"""
import math

from r3_dairy_cow_body import BARREL, SIDES
from r3_dairy_cow_kit import at_x

BONE = {"material": "bone", "role": "body", "tone": 0.82}
JOINT = {"material": "bone", "role": "detail", "tone": 0.5}
MASS = {"material": "bare-skin", "role": "detail", "tone": 0.3}
FEATURE = (0.0075, 0.015, 0.03, 0.075)
T_JOINT, T_TIP = (0.62, 1.374), (1.30, 1.184)
RIBS = [
    (0, 0.80, None), (3, 0.76, None), (6, 0.73, None), (8, 0.71, None), (10, 0.70, None), (12, 0.695, None), (14, 0.695, None), (16, 0.70, None),
    (18, 0.70, 300), (20, 0.72, 280), (22, 0.76, 262), (25, 0.82, 250), (28, 0.90, 240),
]
TOPLINE = [(0.17, 1.42), (0.45, 1.412), (0.62, 1.402), (0.8, 1.397), (0.95, 1.402), (1.08, 1.43), (1.16, 1.45), (1.26, 1.43), (1.36, 1.375)]
NECK = [(1.30, 1.184), (1.35, 1.135), (1.42, 1.105), (1.5, 1.1), (1.58, 1.125), (1.67, 1.165), (1.76, 1.215), (1.86, 1.27)]


def top(x):
    for a, b in zip(TOPLINE, TOPLINE[1:]):
        if a[0] <= x <= b[0]:
            return a[1] + (b[1] - a[1]) * (x - a[0]) / (b[0] - a[0])
    return TOPLINE[0][1] if x < TOPLINE[0][0] else TOPLINE[-1][1]


def thoracic(i):
    """📍 Centre of the thoracic vertebra i (1 at the neck, 13 at the loin)."""
    f = (13.5 - i) / 13
    return (T_JOINT[0] + (T_TIP[0] - T_JOINT[0]) * f, T_JOINT[1] + (T_TIP[1] - T_JOINT[1]) * f)


def frame(k, lod):
    f = FEATURE[lod]
    g = lambda r: max(r, f / 2 + 0.0002)

    def bar(i, bone, p0, p1, r0, r1=None, side=1):
        if lod == 3:
            k.loft(i, bone, [tuple(p0) + (g(r0), g(r0)), tuple(p1) + (g(r1 or r0), g(r1 or r0))], BONE, bone, "member", world=True, axis="auto", side=side)
        else:
            k.cap(i, bone, p0, p1, g(r0), g(r1 if r1 is not None else r0), BONE, bone, "member", world=True, side=side)

    def along(i, bone, rows, side=1, exp=2.0, dome=(0, 0)):
        k.loft(i, bone, [(u, v, w, g(a), g(b)) for u, v, w, a, b in rows], BONE, bone, "member", side=side, exp=exp, dome=dome)

    def path(i, bone, rows, side=1):
        k.loft(i, bone, [(x, h, z, g(a), g(b)) for x, h, z, a, b in rows], BONE, bone, "member", world=True, axis="auto", side=side)

    spine = lambda r: [(x, h, 0, g(r), g(r * 1.15)) for x, h in (T_JOINT, thoracic(10), thoracic(7), thoracic(4), T_TIP)]
    k.loft("spine-thoracic", "thorax", spine(0.03), BONE, "thorax", "member", world=True, axis="auto")
    k.loft("spine-lumbar", "lumbar", [(0.17, 1.374, 0, g(0.034), g(0.04)), (0.4, 1.374, 0, g(0.034), g(0.04)), (0.62, 1.374, 0, g(0.032), g(0.038))], BONE, "lumbar", "member", world=True)
    k.loft("sacrum", "pelvis", [(0.17, 1.374, 0, g(0.036), g(0.05)), (0.04, 1.385, 0, g(0.03), g(0.04)), (-0.1, 1.39, 0, g(0.022), g(0.028)), (-0.25, 1.385, 0, g(0.016), g(0.018))], BONE, "pelvis", "member", world=True, axis="auto")
    if lod == 0:
        for i in range(1, 14):
            x, h = thoracic(i)
            lean = max(0.0, 0.075 - 0.007 * i)
            bar(f"process-t{i}", "thorax", (x, h + 0.02, 0), (x - lean, top(x - lean) - 0.018, 0), 0.013, 0.01)
            k.solids[-1]["lateralScale"] = 0.6
        for i in range(1, 7):
            x = 0.62 - 0.075 * (i - 0.5)
            bar(f"process-l{i}", "lumbar", (x, 1.39, 0), (x + 0.01, top(x) - 0.016, 0), 0.016, 0.014)
            k.solids[-1]["lateralScale"] = 0.5
            k.loft(f"transverse-l{i}", "lumbar", [(x, 1.372, -0.19, 0.008, 0.016), (x + 0.004, 1.378, -0.1, 0.008, 0.02), (x + 0.006, 1.38, 0, 0.012, 0.022), (x + 0.004, 1.378, 0.1, 0.008, 0.02), (x, 1.372, 0.19, 0.008, 0.016)], BONE, "lumbar", "member", world=True, axis=(0, 0, 1))
        k.plate("sacral-crest", "pelvis", [(0.16, 1.39), (0.14, 1.432), (-0.02, 1.445), (-0.1, 1.425), (-0.1, 1.395)], -0.006, 0.006, BONE, "pelvis", "member")
    else:
        t = g(0.009)
        fin = [(1.3, 1.2), (1.3, 1.36), (1.2, 1.43), (1.1, 1.425), (0.95, 1.39), (0.64, 1.39), (0.64, 1.36), (1.0, 1.29)] if lod < 3 else [(1.3, 1.2), (1.2, 1.42), (1.05, 1.42), (0.64, 1.39), (0.64, 1.36)]
        k.plate("dorsal-processes", "thorax", fin, -t, t, BONE, "thorax", "member")
        if lod < 3:
            k.plate("lumbar-processes", "lumbar", [(0.6, 1.36), (0.6, 1.398), (0.19, 1.405), (0.19, 1.36)], -t, t, BONE, "lumbar", "member")
            k.loft("lumbar-wings", "lumbar", [(0.2, 1.378, 0, g(0.008), 0.19), (0.4, 1.378, 0, g(0.008), 0.19), (0.6, 1.378, 0, g(0.008), 0.17)], BONE, "lumbar", "member", world=True, exp=4.0)
    picks = (range(1, 14), (1, 3, 5, 7, 9, 11, 13), (2, 6, 10, 13), (4, 10))[lod]
    half = ((0.016, 0.007), (0.018, 0.0077), (0.02, 0.0152), (0.0377, 0.0377))[lod]
    for i in picks:
        tilt, bottom, sweep = RIBS[i - 1]
        x, h = thoracic(i)
        r = (h - bottom) / 2
        cx = x - r * math.sin(math.radians(tilt))
        width = at_x(BARREL, min(cx, 1.42))[4] - 0.03 - (0.06 if i == 1 else 0.03 if i == 2 else 0.03 if i > 8 else 0)
        if sweep is None:
            k.hoop(f"rib-{i}", "thorax", (x, h - 0.01), bottom, width, half, tilt, BONE, "thorax", "member")
            continue
        ti = math.radians(tilt)
        up, axis = (math.sin(ti), math.cos(ti), 0.0), (math.cos(ti), -math.sin(ti), 0.0)
        r = (h - 0.01 - bottom) / 2
        centre = (x - r * up[0], h - 0.01 - r * up[1])
        rows = []
        steps = int(sweep // 20)
        for j in range(-steps // 2, steps // 2 + 1):
            phi = math.radians(j * sweep / steps)
            c, sn = math.cos(phi), math.sin(phi)
            p = (centre[0] + r * c * up[0], centre[1] + r * c * up[1], width * sn)
            n = (c / r * up[0], c / r * up[1], sn / width)
            rows.append((p, n, axis, half[0], half[1]))
        k.strap(f"rib-{i}", "thorax", rows, BONE, "thorax", "member", thick=half[1])
    if lod < 3:
        k.loft("sternum", "thorax", [(1.385, 0.83, 0, g(0.02), g(0.022)), (1.3, 0.75, 0, g(0.022), g(0.03)), (1.18, 0.705, 0, g(0.018), g(0.035)), (1.0, 0.7, 0, g(0.014), g(0.03))], BONE, "thorax", "member", world=True, axis="auto")
    if lod < 2:
        for j, ((x0, h0), (x1, h1)) in enumerate(zip(NECK, NECK[1:])):
            n = 7 - j
            if n == 1:
                k.ell("atlas", "neck", ((x0 + x1) / 2, (h0 + h1) / 2, 0), (0.045, 0.034, 0.07), BONE, "neck", "member", world=True, along=(x1 - x0, h1 - h0, 0))
            else:
                k.ell(f"cervical-{n}", "neck", ((x0 + x1) / 2, (h0 + h1) / 2, 0), (math.hypot(x1 - x0, h1 - h0) / 2 + 0.004, g(0.034), g(0.04)), BONE, "neck", "member", world=True, along=(x1 - x0, h1 - h0, 0))
    else:
        path("cervical-column", "neck", [(x, h, 0, 0.034, 0.04) for x, h in NECK[::2] + [NECK[-1]]])
    if lod < 3:
        along("skull", "head", [(-0.01, 0.0, 0, 0.045, 0.06), (0.08, 0.004, 0, 0.066, 0.086), (0.17, 0.0, 0, 0.062, 0.096), (0.26, -0.01, 0, 0.05, 0.062), (0.38, -0.02, 0, 0.04, 0.046), (0.47, -0.026, 0, 0.03, 0.046), (0.5, -0.028, 0, 0.02, 0.04)], exp=2.4)
    else:
        along("skull", "head", [(-0.01, 0.0, 0, 0.045, 0.06), (0.17, 0.0, 0, 0.062, 0.09), (0.5, -0.028, 0, 0.0377, 0.04)])
    if lod == 0:
        for side, name, s in SIDES:
            along(f"{name}-mandible", "jaw", [(-0.05, -0.02, 0.062, 0.055, 0.01), (0.02, -0.05, 0.06, 0.06, 0.01), (0.12, -0.045, 0.05, 0.035, 0.01), (0.24, -0.03, 0.03, 0.02, 0.01), (0.33, -0.02, 0.014, 0.014, 0.012)], side=side)
            k.ell(f"{name}-orbit", "head", (0.17, 0.02, 0.088), (0.03, 0.026, 0.014), BONE, "head", "member", side=side)
    else:
        along("mandible", "jaw", [(-0.05, -0.02, 0, 0.05, 0.06), (0.02, -0.05, 0, 0.055, 0.06), (0.14, -0.04, 0, 0.03, 0.045), (0.33, -0.02, 0, 0.016, 0.02)])
    along("caudal-1", "tail-1", [(0, 0, 0, 0.02, 0.022), (0.3, 0, 0, 0.013, 0.014)])
    along("caudal-2", "tail-2", [(0, 0, 0, 0.013, 0.014), (0.45, 0, 0, 0.008, 0.008)])
    along("caudal-3", "tail-3", [(0, 0, 0, 0.008, 0.008), (0.3, 0, 0, 0.005, 0.005)])
    k.plate("udder-ligament", "udder", [(0.0, 0.86), (0.4, 0.74), (0.38, 0.62), (0.02, 0.6)], -g(0.006), g(0.006), BONE, "udder", "member")
    for side, name, s in SIDES:
        k.loft(f"{name}-ear-cartilage", f"ear-{s}", [(0.0, 0, 0.085, g(0.012), g(0.006)), (0.03, 0, 0.16, g(0.03), g(0.005)), (0.06, 0, 0.23, g(0.026), g(0.005)), (0.072, 0, 0.26, g(0.012), g(0.005))], BONE, f"ear-{s}", "member", axis="auto", side=side)
        hip, hook, pin = (0.0, 1.118, 0.15), (0.285, 1.38, 0.225), (-0.265, 1.275, 0.135)
        if lod < 2:
            bar(f"{name}-iliac-crest", "pelvis", (0.2, 1.405, 0.05), hook, 0.016, 0.02, side=side)
            path(f"{name}-ilium", "pelvis", [(0.25, 1.385, 0.14, 0.012, 0.085), (0.16, 1.3, 0.15, 0.014, 0.05), (0.07, 1.2, 0.152, 0.02, 0.03), (0.0, 1.125, 0.15, 0.036, 0.034)], side=side)
            bar(f"{name}-ischium", "pelvis", hip, pin, 0.024, 0.03, side=side)
        else:
            bar(f"{name}-hip-bone", "pelvis", hook, pin, 0.03, 0.03, side=side)
        if lod == 0:
            bar(f"{name}-pubis", "pelvis", (0.03, 1.07, 0.02), (0.01, 1.1, 0.13), 0.016, side=side)
        t, sh, mt, hf = f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"hoof-h{s}"
        sc, hu, fa, ca, ff = f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"cannon-f{s}", f"hoof-f{s}"
        if lod < 3:
            along(f"{name}-femur", t, [(-0.02, 0.0, 0.155, 0.034, 0.034), (0.02, -0.01, 0.17, 0.04, 0.03), (0.1, 0, 0.175, 0.024, 0.024), (0.32, 0, 0.19, 0.024, 0.024), (0.405, 0.005, 0.2, 0.045, 0.04), (0.435, 0.005, 0.2, 0.04, 0.04)], side=side)
            along(f"{name}-tibia", sh, [(0.0, 0, 0.2, 0.042, 0.04), (0.05, 0, 0.198, 0.03, 0.028), (0.18, -0.004, 0.188, 0.02, 0.02), (0.34, -0.004, 0.178, 0.022, 0.024), (0.38, 0, 0.176, 0.03, 0.03)], side=side)
            along(f"{name}-metatarsus", mt, [(0.0, -0.004, 0.175, 0.034, 0.03), (0.06, -0.002, 0.175, 0.026, 0.024), (0.14, 0, 0.175, 0.018, 0.018), (0.29, 0, 0.175, 0.018, 0.02), (0.349, 0, 0.175, 0.024, 0.028)], side=side)
            k.loft(f"{name}-calcaneus", mt, [(0.02, -0.025, 0.175, g(0.02), g(0.016)), (-0.03, -0.05, 0.175, g(0.018), g(0.015)), (-0.07, -0.066, 0.175, g(0.016), g(0.015))], BONE, mt, "member", axis="auto", side=side)
            along(f"{name}-scapula", sc, [(0.05, 0, 0.07, 0.08, 0.008), (0.15, 0, 0.125, 0.08, 0.008), (0.3, 0, 0.162, 0.05, 0.009), (0.4, 0.0, 0.17, 0.028, 0.012), (0.46, 0.005, 0.172, 0.036, 0.022)], side=side, exp=2.6)
            along(f"{name}-humerus", hu, [(-0.01, 0, 0.172, 0.04, 0.036), (0.05, -0.005, 0.176, 0.034, 0.03), (0.14, 0, 0.18, 0.024, 0.024), (0.28, 0, 0.186, 0.024, 0.024), (0.35, 0, 0.19, 0.038, 0.036)], side=side)
            along(f"{name}-radius", fa, [(0.0, 0.004, 0.19, 0.034, 0.034), (0.06, 0.004, 0.186, 0.024, 0.026), (0.22, 0.002, 0.174, 0.02, 0.024), (0.36, 0.002, 0.163, 0.026, 0.032), (0.385, 0.002, 0.16, 0.026, 0.032)], side=side)
            k.loft(f"{name}-olecranon", fa, [(0.06, -0.026, 0.19, g(0.014), g(0.012)), (-0.01, -0.04, 0.19, g(0.02), g(0.014)), (-0.075, -0.06, 0.19, g(0.022), g(0.016))], BONE, fa, "member", axis="auto", side=side)
            along(f"{name}-metacarpus", ca, [(0.0, 0.002, 0.16, 0.03, 0.032), (0.04, 0.002, 0.16, 0.026, 0.03), (0.09, 0, 0.158, 0.017, 0.02), (0.21, 0, 0.153, 0.017, 0.021), (0.26, 0, 0.15, 0.022, 0.029)], side=side)
        else:
            for i, bone, w0, w1 in ((f"{name}-femur", t, 0.155, 0.2), (f"{name}-tibia", sh, 0.2, 0.176), (f"{name}-metatarsus", mt, 0.175, 0.175), (f"{name}-scapula", sc, 0.08, 0.172), (f"{name}-humerus", hu, 0.172, 0.19), (f"{name}-radius", fa, 0.19, 0.16), (f"{name}-metacarpus", ca, 0.16, 0.15)):
                length = k.bones[bone]["lengthM"]
                k.loft(i, bone, [(0, 0, w0, 0.0377, 0.0377), (length, 0, w1, 0.0377, 0.0377)], BONE, bone, "member", side=side)
        if lod == 0:
            k.ell(f"{name}-patella", t, (0.41, 0.05, 0.2), (0.03, 0.016, 0.022), BONE, t, "member", side=side)
            k.ell(f"{name}-greater-trochanter", t, (-0.04, -0.035, 0.19), (0.035, 0.028, 0.022), BONE, t, "member", side=side)
            k.ell(f"{name}-tarsus", mt, (0.01, -0.004, 0.175), (0.036, 0.034, 0.032), BONE, mt, "member", side=side)
            k.ell(f"{name}-carpus", ca, (0.0, 0.002, 0.16), (0.03, 0.03, 0.034), BONE, ca, "member", side=side)
            along(f"{name}-scapular-spine", sc, [(0.06, 0.012, 0.085, 0.008, 0.008), (0.2, 0.012, 0.15, 0.008, 0.012), (0.38, 0.012, 0.18, 0.008, 0.012)], side=side)
        for bone, zc in ((hf, 0.175), (ff, 0.15)):
            if lod == 0:
                for d, off in (("inner", -1), ("outer", 1)):
                    k.cap(f"{bone}-pastern-{d}", bone, (0.0, 0.0, zc + off * 0.018), (0.062, 0.004, zc + off * 0.022), 0.013, 0.011, BONE, bone, "member", side=side)
                    k.cap(f"{bone}-coronet-{d}", bone, (0.066, 0.004, zc + off * 0.022), (0.094, 0.002, zc + off * 0.025), 0.011, 0.011, BONE, bone, "member", side=side)
                    k.loft(f"{bone}-claw-bone-{d}", bone, [(0.092, 0.004, zc + off * 0.025, 0.014, 0.012), (0.12, -0.012, zc + off * 0.027, 0.012, 0.011), (0.142, -0.03, zc + off * 0.028, 0.006, 0.006)], BONE, bone, "member", axis="auto", side=side)
            else:
                k.loft(f"{bone}-digits", bone, [(0.0, 0.0, zc, g(0.02), g(0.022)), (0.11 if lod < 3 else 0.09, -0.006, zc, g(0.016), g(0.03))], BONE, bone, "member", side=side)
    joints = {"pelvis": (0, 0.03), "lumbar": (0, 0.03), "thorax": (0, 0.03), "neck": (0, 0.028), "head": (0, 0.03), "jaw": (0, 0.02), "tail-1": (0, 0.018), "tail-2": (0, 0.012), "tail-3": (0, 0.008), "udder": (0, 0.014), "ear-n": (-0.085, 0.01), "ear-f": (0.085, 0.01)}
    for s, sign in (("f", 1), ("n", -1)):
        joints.update({f"thigh-h{s}": (sign * 0.15, 0.034), f"shank-h{s}": (sign * 0.2, 0.036), f"metatarsus-h{s}": (sign * 0.175, 0.03), f"hoof-h{s}": (sign * 0.175, 0.024), f"scapula-f{s}": (sign * 0.06, 0.016), f"humerus-f{s}": (sign * 0.172, 0.034), f"forearm-f{s}": (sign * 0.19, 0.032), f"cannon-f{s}": (sign * 0.16, 0.028), f"hoof-f{s}": (sign * 0.15, 0.024)})
    for bone, (z, r) in joints.items():
        if lod >= 2 and (r < 0.024 or bone in ("pelvis", "lumbar")):
            continue
        if lod == 3 and r < 0.032:
            continue
        if lod == 1 and r < 0.012:
            continue
        x, h, _ = k.world(bone, 0, 0)
        k.ell(f"joint-{bone}", bone, (x, h, z), (g(r),) * 3, JOINT, bone, "joint", world=True)
    for bone, r in (("pelvis", 0.07), ("lumbar", 0.08), ("thorax", 0.09)):
        m = k.bones[bone]["mass"]
        x, h, _ = k.world(bone, m["comX"], m["comY"])
        k.ell(f"mass-{bone}", bone, (x, h, 0), (g(r),) * 3, MASS, bone, "mass", world=True)
