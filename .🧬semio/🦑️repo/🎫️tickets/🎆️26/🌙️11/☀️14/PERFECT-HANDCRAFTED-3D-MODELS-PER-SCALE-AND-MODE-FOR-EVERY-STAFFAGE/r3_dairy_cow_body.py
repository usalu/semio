"""🐮 The body of the handcrafted dairy cow (Holstein-Friesian, 650 kg, 1.45 m at the withers) at 1:50: every station stated by hand against the reference table of `r3-animals-dairy-cow.md`.

World stations are (x ahead of the hip joint, h above the ground, z to the left, half height, half width, exponent); bone stations are (u along the bone, v across it in the sagittal plane, w lateral, sagittal radius, lateral radius).
@see ./r3_dairy_cow_kit.py
"""
import math

from r3_dairy_cow_kit import pieces, at_x

WHITE = {"material": "white-hair", "role": "body", "tone": 0.9}
FARWHITE = {"material": "white-hair", "role": "body", "tone": 0.9}
BLACK = {"material": "black-hair", "role": "body", "tone": 0.14}
FLANK = {"material": "white-hair", "role": "body", "tone": 0.84}
SKIN = {"material": "bare-skin", "role": "body", "tone": 0.66}
NOSE = {"material": "bare-skin", "role": "body", "tone": 0.3}
HORN = {"material": "claw-horn", "role": "body", "tone": 0.32}
EYE = {"material": "bare-skin", "role": "detail", "tone": 0.04}
DARK = {"material": "bare-skin", "role": "detail", "tone": 0.08}
SIDES = ((1, "far", "f"), (-1, "near", "n"))
JOINT = 0.62

BARREL = [
    (-0.05, 1.02, 0, 0.20, 0.17, 2.2),
    (0.10, 1.0, 0, 0.27, 0.25, 2.2),
    (0.28, 0.985, 0, 0.325, 0.32, 2.2),
    (0.45, 0.975, 0, 0.355, 0.355, 2.2),
    (0.60, 0.97, 0, 0.36, 0.365, 2.2),
    (0.78, 0.98, 0, 0.36, 0.35, 2.2),
    (0.95, 0.995, 0, 0.345, 0.315, 2.2),
    (1.08, 1.005, 0, 0.34, 0.28, 2.15),
    (1.20, 1.01, 0, 0.335, 0.25, 2.15),
    (1.30, 1.02, 0, 0.31, 0.215, 2.15),
    (1.38, 1.03, 0, 0.26, 0.17, 2.15),
    (1.44, 1.04, 0, 0.19, 0.12, 2.15),
]
UPPER = [
    (0.30, 1.29, 0, 0.13, 0.265, 5.0),
    (0.45, 1.285, 0, 0.13, 0.215, 3.6),
    (0.62, 1.28, 0, 0.125, 0.19, 3.0),
    (0.80, 1.275, 0, 0.125, 0.165, 2.8),
    (0.95, 1.28, 0, 0.125, 0.14, 2.6),
    (1.08, 1.30, 0, 0.135, 0.115, 2.6),
    (1.16, 1.315, 0, 0.14, 0.10, 2.6),
    (1.26, 1.30, 0, 0.135, 0.09, 2.6),
    (1.36, 1.26, 0, 0.12, 0.08, 2.6),
]
RUMP = [
    (-0.30, 1.25, 0, 0.10, 0.14, 2.6),
    (-0.24, 1.23, 0, 0.19, 0.19, 2.8),
    (-0.10, 1.19, 0, 0.255, 0.235, 3.0),
    (0.05, 1.17, 0, 0.285, 0.265, 3.2),
    (0.20, 1.16, 0, 0.285, 0.275, 3.4),
    (0.30, 1.15, 0, 0.275, 0.275, 3.4),
    (0.38, 1.14, 0, 0.26, 0.25, 3.0),
]
TRUNK = (BARREL, UPPER, RUMP)


def inside(x, h, z, tables=TRUNK):
    for table in tables:
        if table[0][0] <= x <= table[-1][0]:
            s = at_x(table, x)
            if (abs(h - s[1]) / s[3]) ** s[5] + (abs(z) / s[4]) ** s[5] <= 1:
                return True
    return False


def skin(x, theta, centre=1.05, tables=TRUNK):
    """📍 The point of the trunk surface at x on the ray that leaves (h = centre, z = 0) at `theta` degrees from straight up towards the left side, and the outward normal there in the cross-section."""
    def reach(deg):
        dy, dz = math.cos(math.radians(deg)), math.sin(math.radians(deg))
        lo, hi = 0.0, 0.7
        for _ in range(40):
            mid = (lo + hi) / 2
            if inside(x, centre + mid * dy, mid * dz, tables):
                lo = mid
            else:
                hi = mid
        return (x, centre + lo * dy, lo * dz)

    p, a, b = reach(theta), reach(theta - 2), reach(theta + 2)
    t = (b[1] - a[1], b[2] - a[2])
    n = (0.0, -t[1], t[0]) if theta >= 0 or True else None
    length = math.hypot(n[1], n[2]) or 1
    n = (0.0, n[1] / length, n[2] / length)
    if n[1] * math.cos(math.radians(theta)) + n[2] * math.sin(math.radians(theta)) < 0:
        n = (0.0, -n[1], -n[2])
    return p, n


def patch(k, name, bone, rows, look, group, region, proud=0.007, tables=TRUNK, thick=0.014):
    """🐄 A coat patch wrapped round the trunk: rows are (theta, x, half length along x), joined in steps of at most 8 degrees so that the strap follows the curved skin; the first and last row dive under the skin, so the patch edge is a feathered line."""
    dense = []
    for a, b in zip(rows, rows[1:]):
        n = max(1, math.ceil(abs(b[0] - a[0]) / 8))
        dense += [tuple(a[j] + (b[j] - a[j]) * i / n for j in range(3)) for i in range(n)]
    dense.append(rows[-1])
    out = []
    for j, (theta, x, half) in enumerate(dense):
        p, n = skin(x, theta, tables=tables)
        fore, aft = skin(min(x + half, 1.46), theta, tables=tables)[0], skin(max(x - half, -0.3), theta, tables=tables)[0]
        across = (fore[0] - aft[0], fore[1] - aft[1], fore[2] - aft[2])
        dot = sum(a * b for a, b in zip(across, n))
        across = tuple(a - dot * b for a, b in zip(across, n))
        out.append((p, n, across, half, -0.012 if j in (0, len(dense) - 1) else proud))
    k.strap(name, bone, out, look, group, region, thick=thick, decal=True)


def trunk(k, name, table, groups, dome, look=WHITE):
    for mid, first, last, rows in pieces(table, [(JOINT, 0)]):
        rear = mid < JOINT
        k.loft(f"{name}-{'rear' if rear else 'front'}", "lumbar" if rear else "thorax", rows, look, groups[0] if rear else groups[1], "coat-trunk", world=True, dome=(dome[0] if first else 0, dome[1] if last else 0))


def foot(k, bone, side, name, zc, length, half):
    """🦶 Pastern, two dewclaws and the two claws of a cloven hoof in the frame of the pastern bone (u from the fetlock towards the toe at 52 degrees below the horizontal, v forward and up)."""
    toe, rise = (0.616, 0.788, 0), (-0.788, 0.616)
    fetlock = 0.134

    def point(x, y, z):
        return (x * toe[0] + (y - fetlock) * rise[0], x * toe[1] + (y - fetlock) * rise[1], z)

    k.loft(f"{name}-pastern", bone, [(-0.01, 0, zc, 0.04, 0.043), (0.03, 0.002, zc, 0.034, 0.04), (0.07, 0.004, zc, 0.034, 0.046), (0.1, 0.004, zc, 0.03, 0.048)], WHITE, f"{name}-hoof", "lower-legs", side=side, dome=(0.012, 0))
    for j, off in (("inner", -1), ("outer", 1)):
        zo = zc + off * (half + 0.003)
        tip = zc + off * (half + 0.006)
        rows = [(-0.018, 0.038, zo, half * 0.85), (0.015, 0.058, zo, half), (0.048, 0.068, zo, half), (length - 0.04, 0.046, zo, half * 0.9), (length - 0.012, 0.014, tip, half * 0.6)]
        st = [point(x, top / 2, z) + (top / 2, r, 3.0) for x, top, z, r in rows]
        k.loft(f"{name}-claw-{j}", bone, st, HORN, f"{name}-hoof", "hooves", axis=toe, side=side, dome=(0.006, 0.008))
        k.ell(f"{name}-dewclaw-{j}", bone, point(-0.03, 0.105, zc + off * 0.02), (0.016, 0.012, 0.011), HORN, f"{name}-hoof", "hooves", along=(rise[0], rise[1], 0), side=side)


def simple_foot(k, bone, side, name, zc, length, half, tall=0.066):
    """🦶 Pastern and the cloven hoof as one block in the frame of the pastern bone."""
    def point(x, y):
        return (x * 0.616 + (y - 0.134) * -0.788, x * 0.788 + (y - 0.134) * 0.616)

    k.loft(f"{name}-pastern", bone, [point(-0.004, 0.134) + (zc, 0.042, 0.044), point(0.02, 0.1) + (zc, 0.038, 0.044), point(0.045, 0.062) + (zc, 0.038, 0.048)], WHITE, f"{name}-hoof", "lower-legs", side=side, dome=(0.012, 0))
    rows = [(-0.018, tall * 0.6, half * 0.85), (0.03, tall, half), (length - 0.04, tall * 0.7, half * 0.9), (length - 0.012, tall * 0.25, half * 0.62)]
    k.loft(f"{name}-hoof", bone, [point(x, top / 2) + (zc, top / 2, r, 3.0) for x, top, r in rows], HORN, f"{name}-hoof", "hooves", axis=(0.616, 0.788, 0), side=side, dome=(0.006, 0.008))


def body50(k):
    trunk(k, "barrel", BARREL, ("hindbody", "chest"), (0.03, 0.05))
    trunk(k, "back", UPPER, ("hindbody", "chest"), (0.03, 0.04))
    k.loft("rump", "lumbar", RUMP, WHITE, "hindbody", "coat-trunk", world=True, dome=(0.035, 0.03))
    k.ell("waist", "lumbar", (JOINT, 0.985, 0), (0.05, 0.345, 0.35), WHITE, "hindbody", "coat-trunk", world=True)
    k.loft("sacral-crest", "lumbar", [(-0.26, 1.405, 0, 0.03, 0.045), (-0.1, 1.435, 0, 0.03, 0.05), (0.06, 1.44, 0, 0.028, 0.05), (0.24, 1.425, 0, 0.022, 0.045)], BLACK, "patch-rump", "coat-patches", world=True, dome=(0.03, 0.04))
    k.loft("brisket", "thorax", [(1.12, 0.735, 0, 0.065, 0.1), (1.26, 0.735, 0, 0.08, 0.11), (1.36, 0.775, 0, 0.09, 0.1), (1.42, 0.84, 0, 0.09, 0.08), (1.45, 0.92, 0, 0.07, 0.06)], BLACK, "patch-chest", "coat-patches", world=True, dome=(0.05, 0.03))
    patch(k, "patch-saddle", "thorax", [(-124, 0.8, 0.05), (-112, 0.82, 0.12), (-90, 0.86, 0.19), (-60, 0.88, 0.21), (-25, 0.9, 0.18), (0, 0.9, 0.17), (25, 0.9, 0.18), (60, 0.87, 0.2), (88, 0.84, 0.17), (106, 0.8, 0.1), (116, 0.78, 0.045)], BLACK, "patch-barrel", "coat-patches")
    patch(k, "patch-croup", "lumbar", [(-70, 0.0, 0.07), (-58, 0.02, 0.16), (-35, 0.03, 0.21), (0, 0.03, 0.22), (35, 0.03, 0.21), (54, 0.01, 0.15), (66, -0.01, 0.06)], BLACK, "patch-rump", "coat-patches")
    patch(k, "patch-chest", "thorax", [(92, 1.3, 0.04), (104, 1.3, 0.08), (140, 1.3, 0.1), (180, 1.3, 0.1), (220, 1.3, 0.1), (256, 1.3, 0.08), (268, 1.3, 0.04)], BLACK, "patch-chest", "coat-patches")
    for side, name, s in SIDES:
        patch(k.thermal, f"{name}-rumen-flank", "lumbar", [(side * 40, 0.44, 0.05), (side * 52, 0.45, 0.1), (side * 70, 0.46, 0.115), (side * 88, 0.46, 0.105), (side * 104, 0.45, 0.08), (side * 116, 0.44, 0.045)], FLANK, "rumen-flank", "flank-rumen", proud=0.003)
        k.ell(f"{name}-hook", "lumbar", (0.29, 1.385, 0.236), (0.07, 0.03, 0.024), WHITE, "hindbody", "coat-trunk", world=True, side=side)
        k.ell(f"{name}-pin", "lumbar", (-0.272, 1.285, 0.142), (0.045, 0.04, 0.03), WHITE, "hindbody", "coat-trunk", world=True, side=side)
        k.loft(f"{name}-thigh", f"thigh-h{s}", [(0.03, 1.33, 0.2, 0.19, 0.06), (0.0, 1.2, 0.215, 0.255, 0.078), (0.02, 1.05, 0.225, 0.255, 0.085), (0.07, 0.92, 0.225, 0.215, 0.082), (0.12, 0.82, 0.22, 0.175, 0.074), (0.13, 0.74, 0.21, 0.13, 0.062)], WHITE, f"{name}-thigh", "coat-hindlimbs", world=True, axis=(0, -1, 0), exp=2.4, side=side, dome=(0.04, 0.05))
        k.ell(f"{name}-stifle", f"thigh-h{s}", (0.262, 0.80, 0.232), (0.07, 0.055, 0.05), WHITE, f"{name}-thigh", "coat-hindlimbs", world=True, along=(0, 1, 0), side=side)
        k.loft(f"{name}-gaskin", f"shank-h{s}", [(-0.02, -0.01, 0.226, 0.125, 0.07), (0.08, -0.018, 0.217, 0.12, 0.068), (0.2, -0.028, 0.204, 0.1, 0.056), (0.32, -0.035, 0.192, 0.075, 0.042), (0.42, -0.04, 0.181, 0.066, 0.038), (0.49, -0.03, 0.175, 0.066, 0.042)], WHITE, f"{name}-gaskin", "coat-hindlimbs", side=side, dome=(0.05, 0.03))
        k.loft(f"{name}-hock-point", f"metatarsus-h{s}", [(0.04, -0.03, 0.175, 0.042, 0.034), (-0.035, -0.058, 0.175, 0.034, 0.03), (-0.07, -0.07, 0.175, 0.024, 0.024)], WHITE, f"{name}-hind-cannon", "coat-hindlimbs", axis="auto", side=side, dome=(0, 0.02))
        k.loft(f"{name}-hind-cannon", f"metatarsus-h{s}", [(0.0, -0.012, 0.175, 0.064, 0.046), (0.05, -0.008, 0.175, 0.052, 0.04), (0.1, -0.002, 0.175, 0.037, 0.034), (0.2, -0.002, 0.175, 0.035, 0.033), (0.255, 0, 0.175, 0.04, 0.04), (0.29, 0.002, 0.175, 0.044, 0.045)], WHITE, f"{name}-hind-cannon", "lower-legs", side=side, dome=(0.012, 0.012))
        foot(k, f"hoof-h{s}", side, f"{name}-hind", 0.175, 0.112, 0.025)
        k.loft(f"{name}-scapula", f"scapula-f{s}", [(0.02, 0, 0.05, 0.1, 0.06), (0.1, 0, 0.1, 0.13, 0.06), (0.25, 0.0, 0.162, 0.125, 0.06), (0.38, 0.005, 0.164, 0.1, 0.058), (0.46, 0.01, 0.16, 0.08, 0.055)], WHITE, f"{name}-scapula", "coat-forelimbs", side=side, dome=(0.04, 0.05))
        k.loft(f"{name}-humerus", f"humerus-f{s}", [(-0.02, 0, 0.18, 0.07, 0.052), (0.1, -0.02, 0.18, 0.09, 0.058), (0.22, -0.025, 0.18, 0.092, 0.058), (0.33, -0.012, 0.184, 0.08, 0.054), (0.38, -0.005, 0.19, 0.068, 0.05)], WHITE, f"{name}-humerus", "coat-forelimbs", side=side, dome=(0.05, 0.03))
        k.ell(f"{name}-olecranon", f"forearm-f{s}", (-0.06, -0.055, 0.193), (0.05, 0.036, 0.03), WHITE, f"{name}-forearm", "coat-forelimbs", side=side)
        k.loft(f"{name}-forearm", f"forearm-f{s}", [(-0.02, -0.005, 0.198, 0.075, 0.052), (0.06, -0.005, 0.193, 0.068, 0.05), (0.16, 0, 0.183, 0.052, 0.042), (0.27, 0, 0.171, 0.04, 0.036), (0.345, 0.003, 0.164, 0.042, 0.042), (0.39, 0.004, 0.16, 0.046, 0.047)], WHITE, f"{name}-forearm", "coat-forelimbs", side=side, dome=(0.04, 0.03))
        k.loft(f"{name}-fore-cannon", f"cannon-f{s}", [(0.0, 0.004, 0.16, 0.047, 0.048), (0.035, 0.004, 0.16, 0.044, 0.046), (0.07, 0, 0.158, 0.033, 0.036), (0.17, 0, 0.155, 0.031, 0.034), (0.225, 0, 0.152, 0.037, 0.04), (0.26, 0, 0.15, 0.042, 0.045)], WHITE, f"{name}-fore-cannon", "lower-legs", side=side, dome=(0.012, 0.012))
        foot(k, f"hoof-f{s}", side, f"{name}-fore", 0.15, 0.118, 0.027)
        k.loft(f"{name}-udder", "udder", [(-0.07, 0.79, 0.045, 0.05, 0.035), (-0.04, 0.74, 0.055, 0.12, 0.07), (0.03, 0.68, 0.065, 0.135, 0.09), (0.14, 0.64, 0.07, 0.105, 0.1), (0.26, 0.63, 0.07, 0.09, 0.1), (0.37, 0.635, 0.065, 0.065, 0.085), (0.46, 0.645, 0.05, 0.04, 0.06), (0.51, 0.65, 0.035, 0.02, 0.035)], SKIN, "udder", "udder", world=True, exp=2.3, side=side, dome=(0.015, 0.02))
        k.cap(f"teat-front-{name}", "udder", (0.3, 0.56, 0.088), (0.305, 0.492, 0.09), 0.015, 0.011, SKIN, f"teat-front-{name}", "udder", world=True, side=side)
        k.cap(f"teat-rear-{name}", "udder", (0.1, 0.555, 0.062), (0.102, 0.492, 0.063), 0.014, 0.011, SKIN, f"teat-rear-{name}", "udder", world=True, side=side)
        k.loft(f"{name}-ear", f"ear-{s}", [(0.0, 0, 0.085, 0.022, 0.014), (0.02, 0, 0.13, 0.045, 0.011), (0.04, 0, 0.18, 0.055, 0.009), (0.06, 0, 0.23, 0.046, 0.008), (0.075, 0, 0.265, 0.022, 0.0075)], BLACK, f"{name}-ear", "ears", axis="auto", side=side, dome=(0, 0.012))
        k.ell(f"{name}-brow", "head", (0.155, 0.045, 0.094), (0.04, 0.024, 0.018), BLACK, "head", "coat-head-neck", side=side)
        k.ell(f"{name}-eye", "head", (0.172, 0.022, 0.1085), (0.014, 0.0095, 0.004), EYE, f"{name}-eye", "eyes", side=side)
        k.ell(f"{name}-nostril", "head", (0.53, 0.012, 0.036), (0.012, 0.012, 0.016), DARK, f"{name}-nostril", "muzzle", side=side)
    k.loft("neck", "neck", [(-0.1, -0.13, 0, 0.28, 0.14), (0.08, -0.12, 0, 0.27, 0.12), (0.2, -0.105, 0, 0.215, 0.1), (0.35, -0.1, 0, 0.172, 0.085), (0.5, -0.09, 0, 0.138, 0.078), (0.62, -0.08, 0, 0.118, 0.075), (0.7, -0.077, 0, 0.1, 0.075)], BLACK, "neck", "coat-head-neck", exp=2.2, dome=(0.03, 0.03))
    k.loft("neck-crest", "neck", [(0.0, 0.1, 0, 0.05, 0.06), (0.15, 0.09, 0, 0.045, 0.05), (0.35, 0.065, 0, 0.04, 0.045), (0.55, 0.035, 0, 0.04, 0.05), (0.68, 0.02, 0, 0.04, 0.06)], BLACK, "neck", "coat-head-neck", dome=(0.04, 0.03))
    k.loft("dewlap", "neck", [(1.87, 1.1, 0, 0.022, 0.012), (1.74, 1.04, 0, 0.04, 0.016), (1.6, 0.97, 0, 0.055, 0.02), (1.49, 0.9, 0, 0.065, 0.024), (1.42, 0.83, 0, 0.065, 0.028)], BLACK, "neck", "coat-head-neck", world=True, axis="auto", dome=(0.02, 0.04))
    k.loft("head", "head", [(-0.03, -0.01, 0, 0.065, 0.08, 2.6), (0.03, -0.008, 0, 0.086, 0.1, 2.8), (0.1, -0.008, 0, 0.092, 0.113, 3.0), (0.17, -0.01, 0, 0.09, 0.115, 3.0), (0.25, -0.012, 0, 0.078, 0.094, 2.6), (0.33, -0.014, 0, 0.068, 0.078, 2.5), (0.41, -0.016, 0, 0.062, 0.07, 2.6), (0.47, -0.018, 0, 0.062, 0.076, 3.2), (0.505, -0.02, 0, 0.06, 0.078, 3.6)], BLACK, "head", "coat-head-neck", dome=(0.03, 0.018))
    k.ell("poll", "head", (0.0, 0.05, 0), (0.045, 0.04, 0.085), BLACK, "head", "coat-head-neck")
    k.ell("blaze", "head", (0.13, 0.076, 0), (0.09, 0.013, 0.042), WHITE, "blaze", "coat-head-neck", decal=True)
    k.ell("muzzle", "head", (0.514, -0.018, 0), (0.028, 0.052, 0.07), NOSE, "muzzle", "muzzle")
    k.loft("jaw", "jaw", [(-0.07, -0.02, 0, 0.065, 0.07), (0.01, -0.04, 0, 0.088, 0.088), (0.1, -0.035, 0, 0.076, 0.078), (0.2, -0.022, 0, 0.056, 0.06), (0.28, -0.014, 0, 0.044, 0.052), (0.335, -0.01, 0, 0.038, 0.056)], BLACK, "jaw", "coat-head-neck", exp=2.4, dome=(0.03, 0.02))
    k.loft("tail-1", "tail-1", [(-0.05, 0.035, 0, 0.045, 0.05), (0.03, 0.005, 0, 0.04, 0.044), (0.12, -0.002, 0, 0.032, 0.034), (0.3, 0, 0, 0.024, 0.026)], BLACK, "tail-1", "coat-tail", axis="auto", dome=(0.03, 0.012))
    k.loft("tail-2", "tail-2", [(0, 0, 0, 0.024, 0.026), (0.22, 0, 0, 0.019, 0.02), (0.45, 0, 0, 0.016, 0.017)], BLACK, "tail-2", "coat-tail", dome=(0.012, 0.01))
    k.loft("tail-3", "tail-3", [(0, 0, 0, 0.016, 0.017), (0.08, 0, 0, 0.018, 0.018)], BLACK, "tail-3", "coat-tail", dome=(0.01, 0.01))
    k.loft("tail-switch", "tail-3", [(0.07, 0, 0, 0.02, 0.02), (0.13, 0, 0, 0.034, 0.03), (0.2, 0, 0, 0.042, 0.036), (0.27, 0, 0, 0.034, 0.03), (0.33, 0, 0, 0.015, 0.014)], WHITE, "tail-switch", "coat-tail", dome=(0.01, 0.012))


BARREL100 = [(-0.04, 1.02, 0, 0.21, 0.18, 2.2), (0.2, 0.99, 0, 0.3, 0.29, 2.2), (0.45, 0.975, 0, 0.355, 0.355, 2.2), (0.62, 0.97, 0, 0.36, 0.365, 2.2), (0.9, 0.99, 0, 0.35, 0.33, 2.2), (1.15, 1.008, 0, 0.337, 0.26, 2.15), (1.33, 1.022, 0, 0.3, 0.2, 2.15), (1.44, 1.04, 0, 0.19, 0.12, 2.15)]
UPPER100 = [(0.3, 1.29, 0, 0.13, 0.26, 4.5), (0.62, 1.28, 0, 0.125, 0.19, 3.0), (0.95, 1.28, 0, 0.125, 0.14, 2.6), (1.16, 1.315, 0, 0.14, 0.1, 2.6), (1.36, 1.26, 0, 0.12, 0.08, 2.6)]
RUMP100 = [(-0.3, 1.25, 0, 0.1, 0.14, 2.6), (-0.2, 1.22, 0, 0.215, 0.2, 2.9), (0.05, 1.17, 0, 0.285, 0.265, 3.2), (0.3, 1.15, 0, 0.275, 0.275, 3.4), (0.38, 1.14, 0, 0.26, 0.25, 3.0)]
TRUNK100 = (BARREL100, UPPER100, RUMP100)
REAR200 = [(-0.3, 1.25, 0, 0.11, 0.14, 2.6), (-0.2, 1.2, 0, 0.235, 0.2, 2.8), (0.0, 1.13, 0, 0.325, 0.265, 2.9), (0.25, 1.05, 0, 0.385, 0.31, 2.8), (0.45, 1.02, 0, 0.395, 0.35, 2.6), (0.62, 1.01, 0, 0.395, 0.36, 2.5)]
FRONT200 = [(0.62, 1.01, 0, 0.395, 0.36, 2.5), (0.9, 1.02, 0, 0.385, 0.325, 2.4), (1.15, 1.06, 0, 0.395, 0.25, 2.3), (1.32, 1.05, 0, 0.33, 0.2, 2.3), (1.44, 1.03, 0, 0.21, 0.13, 2.3)]
TRUNK200 = (REAR200, FRONT200)
REAR500 = [(-0.3, 1.22, 0, 0.15, 0.16, 2.6), (-0.1, 1.15, 0, 0.3, 0.25, 2.8), (0.25, 1.05, 0, 0.385, 0.32, 2.7), (0.62, 1.01, 0, 0.395, 0.36, 2.5)]
FRONT500 = [(0.62, 1.01, 0, 0.395, 0.36, 2.5), (1.0, 1.03, 0, 0.395, 0.3, 2.4), (1.3, 1.05, 0, 0.34, 0.2, 2.3), (1.44, 1.03, 0, 0.22, 0.13, 2.3)]


def body100(k):
    """🐄 1:100, smallest feature 15 mm: the trunk in three masses with fewer sections, coat patches as single straps, one mass per limb segment with the joint knobs merged in, one block per cloven hoof, the udder as one bag with four teats, the head without brows and nostrils."""
    trunk(k, "barrel", BARREL100, ("hindbody", "chest"), (0.03, 0.05))
    trunk(k, "back", UPPER100, ("hindbody", "chest"), (0.03, 0.04))
    k.loft("rump", "lumbar", RUMP100, WHITE, "hindbody", "coat-trunk", world=True, dome=(0.035, 0.03))
    k.loft("brisket", "thorax", [(1.12, 0.735, 0, 0.065, 0.1), (1.3, 0.75, 0, 0.085, 0.11), (1.42, 0.84, 0, 0.09, 0.08), (1.45, 0.92, 0, 0.07, 0.06)], BLACK, "patch-chest", "coat-patches", world=True, dome=(0.05, 0.03))
    patch(k, "patch-saddle", "thorax", [(-124, 0.8, 0.05), (-108, 0.83, 0.14), (-60, 0.88, 0.21), (0, 0.9, 0.17), (60, 0.87, 0.2), (100, 0.82, 0.13), (116, 0.78, 0.05)], BLACK, "patch-barrel", "coat-patches", tables=TRUNK100)
    patch(k, "patch-croup", "lumbar", [(-70, 0.0, 0.07), (-56, 0.02, 0.17), (0, 0.03, 0.22), (52, 0.01, 0.16), (66, -0.01, 0.06)], BLACK, "patch-rump", "coat-patches", tables=TRUNK100)
    patch(k, "patch-chest", "thorax", [(92, 1.3, 0.04), (106, 1.3, 0.09), (180, 1.3, 0.1), (254, 1.3, 0.09), (268, 1.3, 0.04)], BLACK, "patch-chest", "coat-patches", tables=TRUNK100)
    for side, name, s in SIDES:
        patch(k.thermal, f"{name}-rumen-flank", "lumbar", [(side * 40, 0.44, 0.05), (side * 56, 0.45, 0.1), (side * 80, 0.46, 0.115), (side * 104, 0.45, 0.08), (side * 116, 0.44, 0.045)], FLANK, "rumen-flank", "flank-rumen", proud=0.003, tables=TRUNK100)
        k.loft(f"{name}-thigh", f"thigh-h{s}", [(0.03, 1.33, 0.2, 0.19, 0.06), (0.0, 1.15, 0.22, 0.26, 0.082), (0.06, 0.95, 0.225, 0.23, 0.084), (0.14, 0.8, 0.222, 0.19, 0.078), (0.15, 0.73, 0.21, 0.13, 0.062)], WHITE, f"{name}-thigh", "coat-hindlimbs", world=True, axis=(0, -1, 0), exp=2.4, side=side, dome=(0.04, 0.05))
        k.loft(f"{name}-gaskin", f"shank-h{s}", [(-0.02, -0.01, 0.226, 0.125, 0.07), (0.14, -0.022, 0.21, 0.112, 0.062), (0.32, -0.035, 0.192, 0.075, 0.042), (0.49, -0.032, 0.175, 0.068, 0.042)], WHITE, f"{name}-gaskin", "coat-hindlimbs", side=side, dome=(0.05, 0.03))
        k.loft(f"{name}-hind-cannon", f"metatarsus-h{s}", [(-0.04, -0.04, 0.175, 0.05, 0.036), (0.02, -0.014, 0.175, 0.064, 0.044), (0.1, -0.002, 0.175, 0.037, 0.034), (0.22, -0.002, 0.175, 0.036, 0.034), (0.29, 0.002, 0.175, 0.044, 0.045)], WHITE, f"{name}-hind-cannon", "lower-legs", side=side, dome=(0.012, 0.012))
        k.loft(f"{name}-scapula", f"scapula-f{s}", [(0.02, 0, 0.05, 0.1, 0.06), (0.14, 0, 0.12, 0.13, 0.06), (0.32, 0.0, 0.164, 0.11, 0.06), (0.46, 0.01, 0.16, 0.08, 0.055)], WHITE, f"{name}-scapula", "coat-forelimbs", side=side, dome=(0.04, 0.05))
        k.loft(f"{name}-humerus", f"humerus-f{s}", [(-0.02, 0, 0.18, 0.07, 0.052), (0.16, -0.022, 0.18, 0.094, 0.058), (0.33, -0.02, 0.184, 0.085, 0.054), (0.4, -0.03, 0.19, 0.07, 0.048)], WHITE, f"{name}-humerus", "coat-forelimbs", side=side, dome=(0.05, 0.03))
        k.loft(f"{name}-forearm", f"forearm-f{s}", [(-0.03, -0.018, 0.198, 0.085, 0.05), (0.08, -0.004, 0.192, 0.064, 0.048), (0.27, 0, 0.171, 0.04, 0.036), (0.39, 0.004, 0.16, 0.046, 0.047)], WHITE, f"{name}-forearm", "coat-forelimbs", side=side, dome=(0.04, 0.03))
        k.loft(f"{name}-fore-cannon", f"cannon-f{s}", [(0.0, 0.004, 0.16, 0.047, 0.048), (0.07, 0, 0.158, 0.033, 0.036), (0.19, 0, 0.154, 0.032, 0.035), (0.26, 0, 0.15, 0.042, 0.045)], WHITE, f"{name}-fore-cannon", "lower-legs", side=side, dome=(0.012, 0.012))
        simple_foot(k, f"hoof-h{s}", side, f"{name}-hind", 0.175, 0.112, 0.05)
        simple_foot(k, f"hoof-f{s}", side, f"{name}-fore", 0.15, 0.118, 0.054)
        k.cap(f"teat-front-{name}", "udder", (0.3, 0.56, 0.088), (0.305, 0.492, 0.09), 0.015, 0.012, SKIN, f"teat-front-{name}", "udder", world=True, side=side)
        k.cap(f"teat-rear-{name}", "udder", (0.1, 0.555, 0.062), (0.102, 0.492, 0.063), 0.015, 0.012, SKIN, f"teat-rear-{name}", "udder", world=True, side=side)
        k.loft(f"{name}-ear", f"ear-{s}", [(0.0, 0, 0.085, 0.022, 0.014), (0.03, 0, 0.155, 0.052, 0.01), (0.06, 0, 0.23, 0.046, 0.008), (0.075, 0, 0.265, 0.022, 0.0078)], BLACK, f"{name}-ear", "ears", axis="auto", side=side, dome=(0, 0.012))
        k.ell(f"{name}-eye", "head", (0.172, 0.022, 0.108), (0.012, 0.009, 0.0077), EYE, f"{name}-eye", "eyes", side=side)
    k.loft("udder", "udder", [(-0.07, 0.79, 0, 0.05, 0.08), (0.0, 0.7, 0, 0.14, 0.15), (0.14, 0.64, 0, 0.105, 0.17), (0.3, 0.63, 0, 0.085, 0.165), (0.44, 0.64, 0, 0.045, 0.115), (0.51, 0.65, 0, 0.02, 0.06)], SKIN, "udder", "udder", world=True, exp=2.6, dome=(0.015, 0.02))
    k.loft("neck", "neck", [(-0.1, -0.11, 0, 0.3, 0.14), (0.08, -0.1, 0, 0.29, 0.12), (0.28, -0.085, 0, 0.21, 0.092), (0.5, -0.07, 0, 0.158, 0.078), (0.7, -0.06, 0, 0.118, 0.075)], BLACK, "neck", "coat-head-neck", exp=2.2, dome=(0.03, 0.03))
    k.loft("dewlap", "neck", [(1.87, 1.1, 0, 0.022, 0.012), (1.67, 1.005, 0, 0.048, 0.018), (1.49, 0.9, 0, 0.065, 0.024), (1.42, 0.83, 0, 0.065, 0.028)], BLACK, "neck", "coat-head-neck", world=True, axis="auto", dome=(0.02, 0.04))
    k.loft("head", "head", [(-0.04, 0.0, 0, 0.07, 0.085, 2.6), (0.05, 0.0, 0, 0.094, 0.106, 2.8), (0.17, -0.01, 0, 0.09, 0.115, 3.0), (0.3, -0.013, 0, 0.072, 0.082, 2.5), (0.42, -0.016, 0, 0.062, 0.07, 2.6), (0.505, -0.02, 0, 0.06, 0.078, 3.6)], BLACK, "head", "coat-head-neck", dome=(0.03, 0.018))
    k.ell("blaze", "head", (0.13, 0.076, 0), (0.09, 0.013, 0.042), WHITE, "blaze", "coat-head-neck", decal=True)
    k.ell("muzzle", "head", (0.514, -0.018, 0), (0.028, 0.052, 0.07), NOSE, "muzzle", "muzzle")
    k.loft("jaw", "jaw", [(-0.07, -0.02, 0, 0.065, 0.07), (0.02, -0.04, 0, 0.088, 0.088), (0.18, -0.025, 0, 0.06, 0.063), (0.335, -0.01, 0, 0.038, 0.056)], BLACK, "jaw", "coat-head-neck", exp=2.4, dome=(0.03, 0.02))
    k.loft("tail-1", "tail-1", [(-0.05, 0.035, 0, 0.045, 0.05), (0.06, 0.002, 0, 0.038, 0.04), (0.3, 0, 0, 0.024, 0.026)], BLACK, "tail-1", "coat-tail", axis="auto", dome=(0.03, 0.012))
    k.loft("tail-2", "tail-2", [(0, 0, 0, 0.024, 0.026), (0.45, 0, 0, 0.016, 0.017)], BLACK, "tail-2", "coat-tail", dome=(0.012, 0.01))
    k.loft("tail-3", "tail-3", [(0, 0, 0, 0.016, 0.017), (0.08, 0, 0, 0.018, 0.018)], BLACK, "tail-3", "coat-tail", dome=(0.01, 0.01))
    k.loft("tail-switch", "tail-3", [(0.07, 0, 0, 0.02, 0.02), (0.2, 0, 0, 0.042, 0.036), (0.33, 0, 0, 0.015, 0.014)], WHITE, "tail-switch", "coat-tail", dome=(0.01, 0.012))


def body200(k):
    """🐄 1:200, smallest feature 30 mm: one mass per rig bone; hindquarters with loin and the chest are one egg each, the two large coat patches stay as straps, every leg segment is one loft, pastern and hoof are one piece, the udder a bag without teats, the head with the muzzle and the eyes merged in."""
    k.loft("hindbody", "lumbar", REAR200, WHITE, "hindbody", "coat-trunk", world=True, dome=(0.04, 0))
    k.loft("chest", "thorax", FRONT200, WHITE, "chest", "coat-trunk", world=True, dome=(0, 0.06))
    patch(k, "patch-saddle", "thorax", [(-118, 0.82, 0.06), (-60, 0.88, 0.2), (0, 0.9, 0.17), (60, 0.87, 0.2), (112, 0.8, 0.06)], BLACK, "patch-barrel", "coat-patches", tables=TRUNK200, thick=0.0155)
    patch(k, "patch-croup", "lumbar", [(-66, 0.0, 0.07), (-36, 0.02, 0.2), (0, 0.03, 0.22), (36, 0.02, 0.2), (62, 0.0, 0.07)], BLACK, "patch-rump", "coat-patches", tables=TRUNK200, thick=0.0155)
    for side, name, s in SIDES:
        patch(k.thermal, f"{name}-rumen-flank", "lumbar", [(side * 42, 0.44, 0.05), (side * 62, 0.45, 0.11), (side * 96, 0.45, 0.09), (side * 114, 0.44, 0.045)], FLANK, "rumen-flank", "flank-rumen", proud=0.003, tables=TRUNK200)
        k.loft(f"{name}-thigh", f"thigh-h{s}", [(0.02, 1.3, 0.2, 0.2, 0.07), (0.02, 1.08, 0.222, 0.255, 0.086), (0.13, 0.82, 0.222, 0.19, 0.078), (0.15, 0.73, 0.21, 0.13, 0.062)], WHITE, f"{name}-thigh", "coat-hindlimbs", world=True, axis=(0, -1, 0), exp=2.4, side=side, dome=(0.04, 0.05))
        k.loft(f"{name}-gaskin", f"shank-h{s}", [(-0.02, -0.01, 0.226, 0.125, 0.07), (0.2, -0.028, 0.204, 0.1, 0.056), (0.49, -0.032, 0.175, 0.068, 0.042)], WHITE, f"{name}-gaskin", "coat-hindlimbs", side=side, dome=(0.05, 0.03))
        k.loft(f"{name}-hind-cannon", f"metatarsus-h{s}", [(-0.02, -0.03, 0.175, 0.055, 0.04), (0.1, -0.002, 0.175, 0.037, 0.034), (0.29, 0.002, 0.175, 0.044, 0.045)], WHITE, f"{name}-hind-cannon", "lower-legs", side=side, dome=(0.012, 0.012))
        k.loft(f"{name}-scapula", f"scapula-f{s}", [(0.02, 0, 0.05, 0.1, 0.06), (0.2, 0, 0.14, 0.13, 0.06), (0.46, 0.01, 0.16, 0.08, 0.055)], WHITE, f"{name}-scapula", "coat-forelimbs", side=side, dome=(0.04, 0.05))
        k.loft(f"{name}-humerus", f"humerus-f{s}", [(-0.02, 0, 0.18, 0.07, 0.052), (0.2, -0.022, 0.18, 0.094, 0.058), (0.4, -0.03, 0.19, 0.07, 0.048)], WHITE, f"{name}-humerus", "coat-forelimbs", side=side, dome=(0.05, 0.03))
        k.loft(f"{name}-forearm", f"forearm-f{s}", [(-0.03, -0.018, 0.198, 0.085, 0.05), (0.2, 0, 0.18, 0.048, 0.04), (0.39, 0.004, 0.16, 0.046, 0.047)], WHITE, f"{name}-forearm", "coat-forelimbs", side=side, dome=(0.04, 0.03))
        k.loft(f"{name}-fore-cannon", f"cannon-f{s}", [(0.0, 0.004, 0.16, 0.047, 0.048), (0.12, 0, 0.156, 0.033, 0.036), (0.26, 0, 0.15, 0.042, 0.045)], WHITE, f"{name}-fore-cannon", "lower-legs", side=side, dome=(0.012, 0.012))
        simple_foot(k, f"hoof-h{s}", side, f"{name}-hind", 0.175, 0.112, 0.05)
        simple_foot(k, f"hoof-f{s}", side, f"{name}-fore", 0.15, 0.118, 0.054)
        k.loft(f"{name}-ear", f"ear-{s}", [(0.0, 0, 0.085, 0.025, 0.016), (0.035, 0, 0.17, 0.05, 0.0155), (0.075, 0, 0.26, 0.025, 0.0155)], BLACK, f"{name}-ear", "ears", axis="auto", side=side, dome=(0, 0.012))
        k.thermal.ell(f"{name}-eye", "head", (0.172, 0.02, 0.1), (0.0152, 0.0152, 0.0152), EYE, f"{name}-eye", "eyes", side=side)
    k.thermal.ell("muzzle", "head", (0.514, -0.018, 0), (0.028, 0.052, 0.07), NOSE, "muzzle", "muzzle")
    k.loft("udder", "udder", [(-0.07, 0.79, 0, 0.05, 0.08), (0.02, 0.69, 0, 0.14, 0.155), (0.26, 0.63, 0, 0.09, 0.17), (0.51, 0.65, 0, 0.02, 0.06)], SKIN, "udder", "udder", world=True, exp=2.6, dome=(0.015, 0.02))
    k.loft("neck", "neck", [(-0.1, -0.11, 0, 0.3, 0.14), (0.1, -0.1, 0, 0.285, 0.115), (0.4, -0.08, 0, 0.18, 0.082), (0.7, -0.06, 0, 0.118, 0.075)], BLACK, "neck", "coat-head-neck", exp=2.2, dome=(0.03, 0.03))
    k.loft("head", "head", [(-0.04, 0.0, 0, 0.07, 0.085, 2.6), (0.12, -0.004, 0, 0.094, 0.114, 3.0), (0.33, -0.014, 0, 0.068, 0.078, 2.5), (0.52, -0.02, 0, 0.06, 0.078, 3.4)], BLACK, "head", "coat-head-neck", dome=(0.03, 0.02))
    k.loft("jaw", "jaw", [(-0.07, -0.02, 0, 0.065, 0.07), (0.02, -0.04, 0, 0.088, 0.088), (0.335, -0.01, 0, 0.038, 0.056)], BLACK, "jaw", "coat-head-neck", exp=2.4, dome=(0.03, 0.02))
    k.loft("tail-1", "tail-1", [(-0.05, 0.035, 0, 0.045, 0.05), (0.3, 0, 0, 0.024, 0.026)], BLACK, "tail-1", "coat-tail", axis="auto", dome=(0.03, 0.012))
    k.loft("tail-2", "tail-2", [(0, 0, 0, 0.024, 0.026), (0.45, 0, 0, 0.0155, 0.017)], BLACK, "tail-2", "coat-tail", dome=(0.012, 0.01))
    k.loft("tail-3", "tail-3", [(0, 0, 0, 0.0155, 0.017), (0.18, 0, 0, 0.04, 0.034), (0.32, 0, 0, 0.0155, 0.0155)], WHITE, "tail-3", "coat-tail", dome=(0.01, 0.012))


def body500(k):
    """🐄 1:500, smallest feature 75 mm: hindquarters and chest as two eggs in one mixed coat tone, neck and head as two wedges, thigh and shoulder masses, each lower leg in two tapered pieces that end in the hoof, the udder as one bag and the tail in three pieces as thick as it is drawn."""
    grey = {"material": "white-hair", "role": "body", "tone": 0.55}
    k.loft("hindbody", "lumbar", REAR500, grey, "hindbody", "coat-trunk", world=True, dome=(0.05, 0))
    k.loft("chest", "thorax", FRONT500, grey, "chest", "coat-trunk", world=True, dome=(0, 0.06))
    for side, name, s in SIDES:
        k.loft(f"{name}-thigh", f"thigh-h{s}", [(0.02, 1.28, 0.2, 0.2, 0.075), (0.04, 1.02, 0.222, 0.25, 0.088), (0.15, 0.74, 0.21, 0.14, 0.066)], WHITE, f"{name}-thigh", "coat-hindlimbs", world=True, axis=(0, -1, 0), exp=2.4, side=side, dome=(0.04, 0.05))
        k.loft(f"{name}-hind-leg", f"shank-h{s}", [(0.0, -0.012, 0.22, 0.12, 0.068), (0.25, -0.03, 0.198, 0.085, 0.05), (0.5, -0.03, 0.175, 0.066, 0.042)], WHITE, f"{name}-hind-leg", "coat-hindlimbs", side=side, dome=(0.04, 0.03))
        k.loft(f"{name}-hind-lower", f"metatarsus-h{s}", [(-0.02, -0.02, 0.175, 0.058, 0.042), (0.12, 0, 0.175, 0.038, 0.036), (0.29, 0.002, 0.175, 0.044, 0.045)], WHITE, f"{name}-hind-lower", "lower-legs", side=side, dome=(0.012, 0.012))
        k.loft(f"{name}-fore-upper", f"humerus-f{s}", [(-0.1, 0.16, 0.14, 0.1, 0.06), (0.05, 0.02, 0.178, 0.12, 0.06), (0.25, -0.022, 0.182, 0.095, 0.058), (0.4, -0.03, 0.19, 0.07, 0.048)], WHITE, f"{name}-fore-upper", "coat-forelimbs", side=side, dome=(0.05, 0.03))
        k.loft(f"{name}-fore-leg", f"forearm-f{s}", [(-0.03, -0.018, 0.198, 0.085, 0.05), (0.2, 0, 0.18, 0.05, 0.042), (0.39, 0.004, 0.16, 0.047, 0.046)], WHITE, f"{name}-fore-leg", "coat-forelimbs", side=side, dome=(0.04, 0.03))
        k.loft(f"{name}-fore-lower", f"cannon-f{s}", [(0.0, 0.004, 0.16, 0.047, 0.046), (0.12, 0, 0.156, 0.036, 0.037), (0.26, 0, 0.15, 0.042, 0.045)], WHITE, f"{name}-fore-lower", "lower-legs", side=side, dome=(0.012, 0.012))
        c, sn = math.cos(math.radians(34)), math.sin(math.radians(34))
        for leg, bone, zc, length, half, at in (("hind", f"metatarsus-h{s}", 0.175, 0.112, 0.05, 0.29), ("fore", f"cannon-f{s}", 0.15, 0.118, 0.054, 0.26)):
            mark = len(k.solids)
            simple_foot(k, "hoof-" + ("h" if leg == "hind" else "f") + s, side, f"{name}-{leg}", zc, length, half, tall=0.076)
            for solid in k.solids[mark:]:
                solid["bone"] = bone
                for sec in solid["sections"]:
                    u, v = sec["at"][0], sec["at"][1]
                    sec["at"][0], sec["at"][1] = round(at + u * c - v * sn, 5), round(u * sn + v * c, 5)
                    q = sec["rotation"]
                    h = math.radians(34) / 2
                    r = (0.0, 0.0, math.sin(h), math.cos(h))
                    sec["rotation"] = [round(v, 6) for v in (r[3] * q[0] + r[0] * q[3] + r[1] * q[2] - r[2] * q[1], r[3] * q[1] - r[0] * q[2] + r[1] * q[3] + r[2] * q[0], r[3] * q[2] + r[0] * q[1] - r[1] * q[0] + r[2] * q[3], r[3] * q[3] - r[0] * q[0] - r[1] * q[1] - r[2] * q[2])]
                k.meta[solid["id"]]["group"] = f"{name}-{leg}-lower"
        patch(k.thermal, f"{name}-rumen-flank", "lumbar", [(side * 44, 0.44, 0.06), (side * 80, 0.45, 0.11), (side * 112, 0.44, 0.06)], FLANK, "rumen-flank", "flank-rumen", proud=0.003, tables=(REAR500, FRONT500))
        k.loft(f"{name}-ear", f"ear-{s}", [(0.0, 0, 0.06, 0.03, 0.02), (0.035, 0, 0.17, 0.05, 0.0155), (0.075, 0, 0.26, 0.025, 0.0155)], BLACK, "ear-span", "ears", axis="auto", side=side, dome=(0, 0.012))
        k.solids[-1]["texture"] = True
        k.thermal.ell(f"{name}-eye", "head", (0.172, 0.0, 0.085), (0.038, 0.038, 0.038), EYE, "eye", "eyes", side=side)
    k.thermal.ell("muzzle", "head", (0.5, -0.03, 0), (0.038, 0.06, 0.072), NOSE, "muzzle", "muzzle")
    patch(k.thermal, "patch-saddle", "thorax", [(-116, 0.84, 0.07), (-60, 0.88, 0.2), (0, 0.9, 0.18), (60, 0.88, 0.2), (112, 0.84, 0.07)], BLACK, "patch", "coat-patches", tables=(REAR500, FRONT500))
    patch(k.thermal, "patch-croup", "lumbar", [(-60, 0.02, 0.08), (0, 0.03, 0.2), (58, 0.02, 0.08)], BLACK, "patch", "coat-patches", tables=(REAR500, FRONT500))
    k.loft("udder", "udder", [(-0.06, 0.77, 0, 0.07, 0.09), (0.05, 0.68, 0, 0.135, 0.16), (0.3, 0.63, 0, 0.088, 0.165), (0.51, 0.65, 0, 0.04, 0.06)], SKIN, "udder", "udder", world=True, exp=2.6, dome=(0.02, 0.02))
    k.loft("neck", "neck", [(-0.1, -0.11, 0, 0.3, 0.14), (0.25, -0.09, 0, 0.225, 0.095), (0.7, -0.06, 0, 0.12, 0.075)], BLACK, "neck", "coat-head-neck", exp=2.2, dome=(0.03, 0.03))
    k.loft("head", "head", [(-0.04, -0.03, 0, 0.1, 0.09, 2.6), (0.14, -0.045, 0, 0.135, 0.114, 2.8), (0.35, -0.035, 0, 0.09, 0.078, 2.5), (0.52, -0.03, 0, 0.066, 0.076, 3.0)], BLACK, "head", "coat-head-neck", dome=(0.03, 0.02))
    k.loft("tail-1", "tail-1", [(-0.05, 0.035, 0, 0.045, 0.05), (0.3, 0, 0, 0.03, 0.03)], BLACK, "tail-1", "coat-tail", axis="auto", dome=(0.03, 0.012))
    k.loft("tail", "tail-2", [(0, 0, 0, 0.03, 0.03), (0.45, 0, 0, 0.03, 0.03)], BLACK, "tail", "coat-tail", dome=(0.012, 0.01))
    k.loft("tail-3", "tail-3", [(0, 0, 0, 0.03, 0.03), (0.18, 0, 0, 0.042, 0.04), (0.32, 0, 0, 0.03, 0.03)], BLACK, "tail-3", "coat-tail", dome=(0.01, 0.012))
