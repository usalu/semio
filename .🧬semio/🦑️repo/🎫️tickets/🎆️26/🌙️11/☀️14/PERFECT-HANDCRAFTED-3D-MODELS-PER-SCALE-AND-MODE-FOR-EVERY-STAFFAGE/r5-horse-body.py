"""🐎 Round 5 horse: the body of every scale, restated by hand on the corrected rig (r5-horse-rig.py). Stations are places on a 1.61 m warmblood in the rest pose (x ahead from the hip joint, y up from the ground, z lateral, far side +); every number is read off the reference table in r3-animals-horse.md, "Round 5 handcraft".

One trunk skin from the buttocks to the breast (no seams along the back), a withers ridge over the thoracic processes, a neck that is 0.38 m wide and 0.52 m deep where it leaves the shoulders, legs that taper through real joint swellings (knee, hock, fetlock) instead of balls, hooves with a sloping coronet and heel bulbs, a mane that lies on the crest and falls to the near side, a tail with a dock and a full, curved hair fall.

Loaded by r5-horse-model.py; `body(n)` returns the body list of scale 1:n.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-horse-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
loft, along, ellipsoid, SIDES, world, F = K.loft, K.along, K.ellipsoid, K.SIDES, K.world, K.F

COAT, HAIR, SKIN, EYE, HORN = "short-coat", "mane-tail-hair", "bare-skin", "eye-surface", "hoof-horn"
T_COAT, T_HAIR, T_HORN, T_SKIN, T_EYE = 0.62, 0.2, 0.3, 0.22, 0.04
HEAD_A = F["head"][2]
TRUNK_WEIGHTS = [{"bone": "pelvis", "weight": 0.25}, {"bone": "lumbar", "weight": 0.5}, {"bone": "thorax", "weight": 0.25}]


def tu(x, top, under, rz, e=None):
    """🫁 A vertical trunk section from its top line, its under line and its half width."""
    return (x, (top + under) / 2, 0.0, (top - under) / 2, rz, 0.0, e)


def flat(y, xc, rx, rz, z=0.0, e=None):
    """🥞 A horizontal section: height, centre ahead, half length, half width."""
    return (xc, y, z, rx, rz, -90.0, e)


def span(toe, heel, rz, z, e=None):
    """🦶 A hoof section through its toe point and its heel point (the coronet rises towards the toe)."""
    (xt, yt), (xh, yh) = toe, heel
    tilt = math.degrees(math.atan2(yt - yh, xt - xh))
    return ((xt + xh) / 2, (yt + yh) / 2, z, math.hypot(xt - xh, yt - yh) / 2, rz, tilt - 90.0, e)


TRUNK = {
    50: [(-0.33, 1.36, 1.25, 0.05), (-0.3, 1.42, 1.16, 0.13), (-0.26, 1.465, 1.1, 0.18), (-0.19, 1.51, 1.05, 0.23), (-0.1, 1.553, 1.025, 0.262), (0.0, 1.583, 1.03, 0.277), (0.1, 1.598, 1.045, 0.28), (0.18, 1.588, 1.045, 0.278),
         (0.27, 1.556, 1.03, 0.27), (0.36, 1.533, 1.002, 0.268), (0.45, 1.518, 0.968, 0.273), (0.55, 1.506, 0.928, 0.283), (0.65, 1.5, 0.898, 0.291), (0.76, 1.5, 0.874, 0.296), (0.88, 1.506, 0.86, 0.293), (0.98, 1.52, 0.853, 0.284),
         (1.07, 1.538, 0.853, 0.27), (1.16, 1.546, 0.862, 0.252), (1.25, 1.537, 0.89, 0.228), (1.33, 1.5, 0.95, 0.2), (1.39, 1.45, 1.02, 0.168), (1.44, 1.38, 1.1, 0.12), (1.465, 1.3, 1.17, 0.06)],
    100: [(-0.33, 1.36, 1.25, 0.05), (-0.28, 1.445, 1.13, 0.16), (-0.17, 1.52, 1.045, 0.24), (0.0, 1.583, 1.03, 0.277), (0.14, 1.595, 1.045, 0.279), (0.3, 1.548, 1.022, 0.27),
          (0.48, 1.514, 0.957, 0.275), (0.68, 1.5, 0.89, 0.293), (0.9, 1.508, 0.858, 0.292), (1.08, 1.539, 0.853, 0.268), (1.24, 1.538, 0.886, 0.231), (1.36, 1.478, 0.985, 0.186), (1.44, 1.38, 1.1, 0.12), (1.465, 1.3, 1.17, 0.06)],
    200: [(-0.33, 1.37, 1.24, 0.06), (-0.24, 1.48, 1.08, 0.2), (-0.05, 1.57, 1.025, 0.27), (0.14, 1.595, 1.045, 0.279), (0.4, 1.526, 0.99, 0.27), (0.7, 1.5, 0.885, 0.294), (1.0, 1.523, 0.852, 0.282), (1.22, 1.54, 0.88, 0.235), (1.38, 1.46, 1.01, 0.175), (1.46, 1.32, 1.15, 0.075)],
    500: [(-0.33, 1.38, 1.22, 0.075), (-0.2, 1.5, 1.06, 0.225), (0.08, 1.596, 1.04, 0.279), (0.45, 1.518, 0.968, 0.273), (0.8, 1.502, 0.868, 0.295), (1.12, 1.542, 0.856, 0.262), (1.36, 1.478, 0.985, 0.186), (1.46, 1.32, 1.15, 0.0755)],
}
WITHERS = {
    50: [(0.7, 1.46, 1.39, 0.03), (0.8, 1.5, 1.4, 0.046), (0.92, 1.548, 1.41, 0.058), (1.02, 1.59, 1.42, 0.062), (1.1, 1.612, 1.43, 0.062), (1.18, 1.605, 1.43, 0.064), (1.26, 1.574, 1.42, 0.072), (1.32, 1.535, 1.41, 0.08)],
    100: [(0.66, 1.497, 1.39, 0.055), (0.9, 1.54, 1.41, 0.07), (1.1, 1.612, 1.43, 0.072), (1.24, 1.582, 1.42, 0.08), (1.32, 1.535, 1.41, 0.085)],
    200: [(0.7, 1.5, 1.39, 0.06), (1.05, 1.6, 1.42, 0.075), (1.16, 1.608, 1.43, 0.075), (1.32, 1.535, 1.41, 0.085)],
    500: [(0.72, 1.49, 1.38, 0.0755), (1.08, 1.61, 1.42, 0.08), (1.3, 1.55, 1.41, 0.09)],
}


def trunk(n):
    """🐴 The trunk skin from the buttocks to the breast, bound to the lumbar spine and blended over pelvis, loin and thorax (they never bend against each other in any pose or clip), and the withers ridge over the long thoracic processes."""
    b = [loft("trunk", "lumbar", [tu(*s) for s in TRUNK[n]], exp=2.35 if n <= 200 else 2.2, extra={"weights": TRUNK_WEIGHTS})]
    if n in WITHERS:
        b.append(loft("withers", "thorax", [tu(*s) for s in WITHERS[n]], exp=2.0))
    if n == 50:
        for name, sfx, m in SIDES:
            b.append(loft(f"{name}-breast", "thorax", [(1.28, 1.13, 0.07, 0.07, 0.07, 0.0), (1.35, 1.11, 0.072, 0.088, 0.072, 0.0), (1.41, 1.1, 0.07, 0.084, 0.068, 0.0), (1.445, 1.105, 0.064, 0.06, 0.05, 0.0)], exp=2.4, mirror=m))
    elif n == 100:
        for name, sfx, m in SIDES:
            b.append(loft(f"{name}-breast", "thorax", [(1.28, 1.13, 0.07, 0.07, 0.07, 0.0), (1.38, 1.105, 0.072, 0.088, 0.07, 0.0), (1.445, 1.105, 0.064, 0.06, 0.05, 0.0)], exp=2.4, mirror=m))
    return b


NECK = {
    50: [(1.24, 1.3, 0.0, 0.22, 0.2, 30.0), (1.31, 1.35, 0.0, 0.262, 0.195, 33.0), (1.39, 1.452, 0.0, 0.238, 0.168, 43.0), (1.47, 1.56, 0.0, 0.2, 0.14, 50.0), (1.55, 1.665, 0.0, 0.167, 0.118, 52.0),
         (1.61, 1.742, 0.0, 0.148, 0.106, 52.0), (1.665, 1.81, 0.0, 0.136, 0.099, 51.0), (1.73, 1.885, 0.0, 0.122, 0.092, 49.0), (1.785, 1.944, 0.0, 0.112, 0.087, 46.0), (1.835, 1.991, 0.0, 0.106, 0.083, 42.0), (1.89, 2.037, 0.0, 0.1, 0.079, 35.0), (1.93, 2.05, 0.0, 0.07, 0.06, 26.0)],
}
NECK[100] = [NECK[50][i] for i in (0, 1, 2, 4, 5, 6, 8, 9, 10, 11)]
NECK[200] = [NECK[50][i] for i in (0, 1, 3, 5, 6, 8, 10, 11)]


def shrink(st, k=0.85):
    """🧶 A neck section shrunk about its centre: the hidden end of a neck piece inside its neighbour."""
    return (st[0], st[1], st[2], st[3] * k, st[4] * k, st[5])


def neck(n):
    """🦒 The neck on neck-1, neck-2 and neck-3: pieces that share their seam sections and end hidden in the neighbour; 0.39 m wide and 0.52 m deep where it leaves the shoulders, 0.17 m wide at the throat."""
    s = NECK[50]
    root = loft("neck-root", "neck-1", [(1.2, 1.26, 0.0, 0.1, 0.1, 35.0), (1.26, 1.3, 0.0, 0.19, 0.18, 35.0), (1.34, 1.38, 0.0, 0.2, 0.175, 40.0), (1.42, 1.46, 0.0, 0.17, 0.15, 46.0), (1.47, 1.52, 0.0, 0.1, 0.09, 50.0)], exp=2.2, extra={"weights": [{"bone": "thorax", "weight": 0.5}, {"bone": "neck-1", "weight": 0.5}]})
    throat = ellipsoid("throatlatch", "head", (1.895, 1.975, 0.0), (0.085, 0.07, 0.068), angle=-15.0, extra={"weights": [{"bone": "neck-3", "weight": 0.5}, {"bone": "head", "weight": 0.5}]})
    if n == 500:
        return [root, throat,
            loft("neck-a", "neck-1", [shrink(s[1], 0.8), s[2], s[3], s[5], shrink(s[6])], exp=2.2),
            loft("neck-b", "neck-2", [shrink(s[4]), s[5], s[8], s[10], s[11]], exp=2.2),
        ]
    base = (s[1][0] - 0.03, s[1][1] - 0.035, 0.0, s[1][3] * 0.45, s[1][4] * 0.45, s[1][5])
    a = {50: [base, shrink(s[1], 0.8), s[2], s[3], s[4], s[5]], 100: [base, shrink(s[1], 0.8), s[2], s[4], s[5]], 200: [base, shrink(s[1], 0.8), s[3], s[5]]}[n]
    b = {50: [s[5], s[6], s[7], s[8]], 100: [s[5], s[6], s[8]], 200: [s[5], s[8]]}[n]
    c = {50: [s[8], s[9], s[10], s[11]], 100: [s[8], s[9], s[10], s[11]], 200: [s[8], s[10], s[11]]}[n]
    return [root, throat,
        loft("neck-1", "neck-1", a + [shrink(s[6])], exp=2.2),
        loft("neck-2", "neck-2", [shrink(s[4])] + b + [shrink(s[9])], exp=2.2),
        loft("neck-3", "neck-3", [shrink(s[7])] + c, exp=2.2),
    ]


HEAD = {
    50: [(-0.06, -0.045, 0.045, 0.05), (-0.02, -0.044, 0.088, 0.09), (0.03, -0.044, 0.11, 0.108), (0.1, -0.056, 0.13, 0.12), (0.19, -0.068, 0.138, 0.122), (0.27, -0.066, 0.124, 0.106), (0.35, -0.052, 0.1, 0.09),
         (0.43, -0.038, 0.082, 0.08), (0.5, -0.034, 0.075, 0.074), (0.55, -0.035, 0.07, 0.072), (0.578, -0.038, 0.058, 0.062), (0.594, -0.042, 0.04, 0.046)],
    100: [(-0.035, -0.045, 0.075, 0.076), (0.05, -0.047, 0.115, 0.11), (0.18, -0.066, 0.137, 0.122), (0.32, -0.058, 0.108, 0.096), (0.45, -0.036, 0.08, 0.078), (0.55, -0.035, 0.071, 0.072), (0.592, -0.04, 0.046, 0.052)],
    200: [(-0.035, -0.045, 0.076, 0.076), (0.08, -0.052, 0.124, 0.115), (0.22, -0.066, 0.135, 0.118), (0.42, -0.042, 0.088, 0.083), (0.56, -0.036, 0.07, 0.072), (0.595, -0.04, 0.048, 0.052)],
    500: [(-0.05, -0.05, 0.05, 0.05), (0.02, -0.06, 0.12, 0.1), (0.15, -0.105, 0.172, 0.12), (0.3, -0.09, 0.14, 0.1), (0.45, -0.064, 0.105, 0.085), (0.58, -0.055, 0.08, 0.07), (0.62, -0.055, 0.0375, 0.04)],
}
JAW = {
    50: [(-0.12, -0.045, 0.035, 0.045), (-0.08, -0.06, 0.075, 0.085), (-0.03, -0.072, 0.092, 0.1), (0.06, -0.076, 0.09, 0.096), (0.16, -0.066, 0.066, 0.07), (0.26, -0.055, 0.05, 0.05), (0.34, -0.054, 0.044, 0.045), (0.395, -0.054, 0.042, 0.042), (0.42, -0.05, 0.028, 0.03)],
    100: [(-0.1, -0.05, 0.064, 0.072), (-0.02, -0.072, 0.092, 0.098), (0.12, -0.07, 0.074, 0.078), (0.27, -0.055, 0.05, 0.054), (0.405, -0.054, 0.04, 0.05)],
    200: [(-0.1, -0.05, 0.064, 0.072), (0.0, -0.072, 0.09, 0.096), (0.2, -0.062, 0.058, 0.064), (0.405, -0.054, 0.04, 0.05)],
}


def head(n):
    """🐎 Skull and jaw: 0.59 m long, 0.31 m deep at the jowl, 0.24 m wide across the brows, 0.14 m across the nostrils; eyes, brows, cheek crests, nostrils, ears, forelock by scale."""
    b = [loft("head", "head", along("head", HEAD[n]), exp=2.35)]
    if n <= 200:
        b.append(loft("jaw", "jaw", along("jaw", JAW[n]), exp=2.2))
        r = (0.016, 0.048, 0.054) if n <= 100 else (0.022, 0.054, 0.056)
        b.append(ellipsoid("muzzle", "head", world("head", 0.584 if n <= 100 else 0.582, -0.042) + (0.0,), r, material=SKIN, tone=0.36, angle=HEAD_A))
    for name, sfx, m in SIDES:
        ear = f"ear-{sfx}"
        if n <= 100:
            b.append(ellipsoid(f"{name}-eye", "head", world("head", 0.2, 0.004) + (0.112,), (0.017, 0.013, 0.006 if n == 50 else 0.0065), material=EYE, tone=T_EYE, role="detail", angle=HEAD_A, mirror=m))
        if n == 50:
            b.append(loft(f"{name}-cheek-crest", "head", along("head", [(0.21, -0.058, 0.009, 0.007, 0.106), (0.27, -0.06, 0.01, 0.008, 0.099), (0.33, -0.056, 0.009, 0.007, 0.089), (0.38, -0.05, 0.006, 0.005, 0.08)]), mirror=m))
            b.append(ellipsoid(f"{name}-nostril", "head", world("head", 0.556, 0.004) + (0.056,), (0.024, 0.014, 0.015), material=SKIN, tone=0.1, role="detail", angle=HEAD_A + 25, mirror=m))
            b.append(loft(f"{name}-ear", ear, along(ear, [(-0.05, 0.0, 0.021, 0.019, 0.056), (-0.01, 0.0, 0.026, 0.023, 0.06), (0.035, 0.0, 0.033, 0.028, 0.066), (0.075, 0.0, 0.031, 0.026, 0.072), (0.115, 0.0, 0.023, 0.019, 0.077), (0.148, 0.0, 0.012, 0.01, 0.08), (0.166, 0.0, 0.004, 0.004, 0.082)]), tone=0.56, mirror=m))
            b.append(loft(f"{name}-ear-hollow", ear, along(ear, [(0.02, -0.021, 0.008, 0.012, 0.074), (0.05, -0.025, 0.011, 0.016, 0.08), (0.09, -0.022, 0.01, 0.013, 0.084), (0.125, -0.015, 0.005, 0.006, 0.084)]), tone=0.3, role="detail", mirror=m))
        elif n == 100:
            b.append(loft(f"{name}-ear", ear, along(ear, [(-0.05, 0.0, 0.021, 0.019, 0.056), (0.03, 0.0, 0.032, 0.028, 0.066), (0.1, 0.0, 0.027, 0.022, 0.075), (0.166, 0.0, 0.007, 0.007, 0.082)]), tone=0.56, mirror=m))
        elif n == 200:
            b.append(loft(f"{name}-ear", ear, along(ear, [(-0.05, 0.0, 0.021, 0.019, 0.056), (0.04, 0.0, 0.033, 0.028, 0.067), (0.165, 0.0, 0.009, 0.009, 0.082)]), tone=0.56, mirror=m))
        else:
            b.append(loft(f"{name}-ear", ear, along(ear, [(-0.05, 0.0, 0.034, 0.0375, 0.062), (0.05, 0.0, 0.04, 0.0375, 0.07), (0.165, 0.0, 0.02, 0.0375, 0.08)]), tone=0.56, mirror=m))
    if n == 50:
        b.append(loft("forelock", "head", along("head", [(-0.035, 0.064, 0.012, 0.034), (0.04, 0.084, 0.015, 0.046), (0.11, 0.088, 0.013, 0.042), (0.17, 0.082, 0.009, 0.026), (0.21, 0.076, 0.004, 0.009)]), material=HAIR, tone=T_HAIR))
    elif n == 100:
        b.append(loft("forelock", "head", along("head", [(-0.03, 0.064, 0.015, 0.034), (0.06, 0.086, 0.016, 0.046), (0.14, 0.086, 0.012, 0.034), (0.2, 0.077, 0.007, 0.011)]), material=HAIR, tone=T_HAIR))
    return b


def crest(st):
    """📐 The top of a neck section: where the crest line runs."""
    x, y, _, rs, rl, phi = st
    return (x - rs * math.sin(math.radians(phi)), y + rs * math.cos(math.radians(phi)), rs, rl, phi)


def mane_rows(stations, hang, thick, sheet):
    """💇 Mane over neck sections: `sheet` False gives the band of hair on the crest (3 cm proud of it); True gives the hair that falls to the near side and lies on the neck surface, `hang` metres long and `thick` through, its section turned to the slope of the neck side."""
    out = []
    for st, k in stations:
        x, y, rs, rl, phi = crest(st)
        nx, ny = -math.sin(math.radians(phi)), math.cos(math.radians(phi))
        if not sheet:
            out.append((x - nx * 0.012, y - ny * 0.012, -0.01, 0.042 * k, 0.034 * k, phi, 2.2))
            continue
        th = math.radians(42.0)
        down = rs * (1 - math.cos(th))
        side = rl * math.sin(th) + 0.5 * thick
        roll = math.degrees(math.atan2(rl * math.cos(th), rs * math.sin(th)))
        out.append((x - nx * down, y - ny * down, -side, 0.5 * hang * k, 0.5 * thick, phi, 2.0, roll))
    return out


def hair(n):
    """💇 Forelock aside: the mane on the crest in one piece per neck bone, overlapping, and the tail: a dock with its hair root and a full hair fall that bows away from the hindquarters and narrows to the tip."""
    b = []
    hang, thick = {50: (0.17, 0.024), 100: (0.17, 0.03), 200: (0.18, 0.032)}.get(n, (0, 0))
    N = NECK[50]
    pick = {50: [(N[1], 0.45), (N[2], 0.8), (N[3], 1.0), (N[4], 1.0), (N[5], 1.0), (N[6], 1.0), (N[7], 0.95), (N[8], 0.9), (N[9], 0.85), (N[10], 0.75), (N[11], 0.5)],
            100: [(N[1], 0.45), (N[2], 0.85), (N[4], 1.0), (N[5], 1.0), (N[6], 1.0), (N[8], 0.9), (N[10], 0.75), (N[11], 0.5)],
            200: [(N[1], 0.45), (N[3], 1.0), (N[5], 1.0), (N[7], 0.95), (N[9], 0.85), (N[11], 0.5)]}.get(n)
    if n in (50, 100):
        cut = {50: ((0, 5), (3, 9), (7, 11)), 100: ((0, 4), (2, 6), (5, 8))}[n]
        for sheet, tag in ((False, "crest"), (True, "fall")):
            rows = mane_rows([(st, k if not sheet or i >= 2 else 0.35 * k) for i, (st, k) in enumerate(pick)], hang, thick, sheet)
            for (i, j), bone, k in zip(cut, ("neck-1", "neck-2", "neck-3"), (3, 2, 1)):
                b.append(loft(f"mane-{tag}-{k}", bone, rows[i:j], material=HAIR, tone=T_HAIR))
    elif n == 200:
        rows = mane_rows(pick, hang, thick, False)
        b += [loft("mane-3", "neck-1", rows[0:3], material=HAIR, tone=T_HAIR), loft("mane-2", "neck-2", rows[1:4], material=HAIR, tone=T_HAIR), loft("mane-1", "neck-3", rows[3:6], material=HAIR, tone=T_HAIR)]
    tail = {
        50: ([(-0.05, 0.012, 0.04, 0.042), (0.02, 0.0, 0.046, 0.05), (0.09, -0.012, 0.056, 0.058), (0.16, -0.022, 0.066, 0.066), (0.25, -0.027, 0.072, 0.073)],
             [(-0.02, -0.024, 0.06, 0.062), (0.03, -0.028, 0.074, 0.076), (0.11, -0.03, 0.079, 0.082), (0.19, -0.027, 0.08, 0.086), (0.25, -0.022, 0.078, 0.086)],
             [(-0.02, -0.02, 0.066, 0.074), (0.03, -0.018, 0.077, 0.087), (0.12, -0.011, 0.072, 0.088), (0.2, -0.006, 0.066, 0.084), (0.27, -0.003, 0.06, 0.08)],
             [(-0.02, -0.003, 0.052, 0.07), (0.03, -0.001, 0.058, 0.077), (0.1, 0.002, 0.05, 0.068), (0.16, 0.005, 0.038, 0.054), (0.21, 0.005, 0.024, 0.034), (0.235, 0.004, 0.01, 0.015)]),
        100: ([(-0.05, 0.012, 0.04, 0.042), (0.09, -0.012, 0.056, 0.058), (0.25, -0.027, 0.072, 0.073)],
              [(-0.02, -0.024, 0.06, 0.062), (0.03, -0.028, 0.074, 0.076), (0.15, -0.029, 0.08, 0.085), (0.25, -0.022, 0.078, 0.086)],
              [(-0.02, -0.02, 0.066, 0.074), (0.03, -0.018, 0.077, 0.087), (0.16, -0.008, 0.069, 0.086), (0.27, -0.003, 0.06, 0.08)],
              [(-0.02, -0.003, 0.052, 0.07), (0.03, -0.001, 0.058, 0.077), (0.14, 0.004, 0.043, 0.06), (0.235, 0.004, 0.015, 0.02)]),
        200: ([(-0.05, 0.012, 0.042, 0.044), (0.25, -0.027, 0.072, 0.073)],
              [(-0.02, -0.024, 0.06, 0.062), (0.04, -0.028, 0.075, 0.078), (0.25, -0.022, 0.078, 0.086)],
              [(-0.02, -0.02, 0.066, 0.074), (0.04, -0.017, 0.077, 0.087), (0.27, -0.003, 0.06, 0.08)],
              [(-0.02, -0.003, 0.052, 0.07), (0.04, 0.0, 0.057, 0.075), (0.235, 0.004, 0.02, 0.03)]),
        500: ([(-0.05, 0.0, 0.045, 0.046), (0.25, -0.025, 0.075, 0.075)],
              [(-0.02, -0.022, 0.065, 0.066), (0.25, -0.02, 0.078, 0.084)],
              [(-0.02, -0.018, 0.068, 0.075), (0.27, -0.004, 0.062, 0.078)],
              [(-0.02, -0.004, 0.055, 0.07), (0.235, 0.004, 0.0375, 0.0375)]),
    }[n]
    for i, rows in enumerate(tail):
        bone = f"tail-{i + 1}"
        if i < 3:
            t, v, rs, rl = rows[-1]
            rows = rows + [(K.LEN[bone] + 0.035, v, rs * 0.86, rl * 0.86)]
        b.append(loft(bone, bone, along(bone, rows), material=HAIR, tone=T_HAIR, exp=2.1))
    if n == 50:
        b.append(loft("dock", "tail-1", along("tail-1", [(-0.06, 0.03, 0.04, 0.046), (0.0, 0.026, 0.038, 0.042), (0.08, 0.02, 0.03, 0.033), (0.16, 0.016, 0.022, 0.024)]), exp=2.0))
    return b


FORE_HOOF = [((1.315, 0.0), (1.182, 0.0), 0.064), ((1.297, 0.026), (1.18, 0.013), 0.062), ((1.279, 0.052), (1.176, 0.027), 0.059), ((1.257, 0.084), (1.17, 0.043), 0.054)]
HIND_HOOF = [((0.112, 0.0), (-0.013, 0.0), 0.06), ((0.095, 0.027), (-0.014, 0.013), 0.058), ((0.078, 0.054), (-0.017, 0.028), 0.055), ((0.057, 0.087), (-0.021, 0.045), 0.05)]
FZ, HZ = 0.142, 0.146


def hoof(sid, bone, rows, z, keep, m, rim=False):
    """🧱 A hoof: horizontal sole, walls parallel to the pastern axis, coronet rising towards the toe; `rim` adds the coronary band."""
    secs = [span(t, h, rz, z, 2.3) for i, (t, h, rz) in enumerate(rows) if i in keep]
    out = [loft(sid, bone, secs, material=HORN, tone=T_HORN, exp=2.3, mirror=m)]
    if rim:
        t, h, rz = rows[-1]
        out.append(loft(sid + "-coronet", bone, [span((t[0] + 0.004, t[1] - 0.012), (h[0] - 0.004, h[1] - 0.008), rz + 0.004, z), span((t[0] - 0.006, t[1] + 0.006), (h[0] + 0.004, h[1] + 0.004), rz - 0.004, z)], tone=0.5, exp=2.3, mirror=m))
    return out


def fore50(name, sfx, m):
    sc, hu, ra, cf, pf, hf = (f"{k}-f{sfx}" for k in ("scapula", "humerus", "radius", "cannon", "pastern", "hoof"))
    b = [
        loft(f"{name}-shoulder", sc, along(sc, [(-0.03, 0.0, 0.045, 0.024, 0.08), (0.05, 0.004, 0.095, 0.04, 0.11), (0.15, 0.006, 0.13, 0.05, 0.135), (0.26, 0.004, 0.135, 0.056, 0.152), (0.36, 0.0, 0.118, 0.058, 0.166), (0.44, -0.004, 0.09, 0.056, 0.176), (0.5, -0.004, 0.06, 0.048, 0.178)]), exp=2.0, mirror=m),
        loft(f"{name}-arm", hu, along(hu, [(-0.07, -0.005, 0.042, 0.04, 0.18), (-0.02, -0.02, 0.068, 0.054, 0.19), (0.03, -0.045, 0.088, 0.06, 0.188), (0.08, -0.07, 0.1, 0.064, 0.19), (0.16, -0.09, 0.102, 0.064, 0.19), (0.24, -0.085, 0.085, 0.058, 0.19), (0.3, -0.072, 0.06, 0.05, 0.188), (0.35, -0.055, 0.04, 0.04, 0.186)]), exp=2.1, mirror=m),
        loft(f"{name}-elbow", ra, along(ra, [(-0.085, -0.035, 0.022, 0.026, 0.186), (-0.05, -0.048, 0.033, 0.036, 0.187), (-0.01, -0.046, 0.034, 0.04, 0.186), (0.04, -0.03, 0.03, 0.038, 0.184)]), exp=2.0, mirror=m),
        loft(f"{name}-forearm", ra, along(ra, [(-0.05, -0.005, 0.07, 0.058, 0.186), (0.03, 0.0, 0.078, 0.06, 0.184), (0.12, 0.004, 0.07, 0.055, 0.178), (0.22, 0.004, 0.056, 0.046, 0.17), (0.3, 0.0, 0.046, 0.04, 0.162), (0.35, -0.002, 0.045, 0.042, 0.157), (0.385, -0.002, 0.05, 0.048, 0.155), (0.4, -0.002, 0.044, 0.042, 0.155)]), exp=2.15, mirror=m),
        loft(f"{name}-fore-cannon", cf, along(cf, [(-0.085, -0.004, 0.03, 0.028, 0.156), (-0.04, -0.002, 0.046, 0.044, 0.156, 2.6), (-0.005, 0.0, 0.05, 0.049, 0.155, 2.7), (0.035, -0.005, 0.047, 0.046, 0.154, 2.6), (0.075, -0.008, 0.038, 0.03, 0.152, 2.4), (0.15, -0.009, 0.036, 0.027, 0.15, 2.4),
                                                (0.225, -0.01, 0.037, 0.028, 0.148, 2.4), (0.27, -0.013, 0.044, 0.035, 0.146), (0.305, -0.013, 0.05, 0.04, 0.145), (0.335, -0.006, 0.044, 0.038, 0.144)]), mirror=m),
        ellipsoid(f"{name}-fore-knee-back", cf, world(cf, 0.0, -0.036) + (0.155,), (0.026, 0.016, 0.024), mirror=m),
        ellipsoid(f"{name}-fore-chestnut", ra, world(ra, 0.29, -0.01) + (0.12,), (0.022, 0.013, 0.006), material=SKIN, tone=0.15, role="detail", mirror=m),
        loft(f"{name}-fore-pastern", pf, along(pf, [(-0.035, -0.014, 0.046, 0.038, 0.144), (0.03, -0.004, 0.034, 0.031, 0.143), (0.08, -0.006, 0.031, 0.03, 0.142), (0.115, -0.01, 0.034, 0.036, 0.142), (0.142, -0.012, 0.038, 0.044, 0.142)]), mirror=m),
        ellipsoid(f"{name}-fore-ergot", pf, world(pf, 0.0, -0.046) + (0.144,), (0.011, 0.009, 0.011), material=SKIN, tone=0.25, role="detail", mirror=m),
    ]
    b += hoof(f"{name}-fore-hoof", hf, FORE_HOOF, FZ, (0, 1, 2, 3), m, rim=True)
    tx, ty = FORE_HOOF[3][1]
    b.append(ellipsoid(f"{name}-fore-heel", hf, (tx + 0.022, ty - 0.012, FZ), (0.017, 0.018, 0.042), tone=0.4, mirror=m))
    return b


def hind50(name, sfx, m):
    th, sh, ch, ph, hh = (f"{k}-h{sfx}" for k in ("thigh", "shank", "cannon", "pastern", "hoof"))
    b = [
        loft(f"{name}-thigh", th, [flat(1.45, -0.04, 0.16, 0.06, 0.125), flat(1.34, -0.045, 0.255, 0.1, 0.172), flat(1.22, -0.035, 0.27, 0.108, 0.184), flat(1.1, -0.01, 0.255, 0.108, 0.19), flat(1.0, 0.025, 0.222, 0.098, 0.19), flat(0.93, 0.04, 0.19, 0.088, 0.188), flat(0.87, 0.035, 0.12, 0.072, 0.185), flat(0.83, 0.035, 0.06, 0.05, 0.182)], exp=2.4, mirror=m),
        loft(f"{name}-gaskin", sh, along(sh, [(-0.06, -0.035, 0.09, 0.066, 0.188), (0.04, -0.05, 0.106, 0.072, 0.184), (0.14, -0.052, 0.1, 0.066, 0.178), (0.24, -0.048, 0.08, 0.05, 0.17), (0.32, -0.04, 0.07, 0.045, 0.163), (0.39, -0.032, 0.064, 0.044, 0.158), (0.44, -0.022, 0.056, 0.042, 0.156)]), mirror=m),
        loft(f"{name}-hock-tendon", sh, [(-0.04, 0.775, 0.17, 0.026, 0.022), (-0.075, 0.71, 0.162, 0.018, 0.016), (-0.105, 0.66, 0.156, 0.015, 0.014), (-0.122, 0.625, 0.155, 0.013, 0.013)], exp=2.0, mirror=m),
        loft(f"{name}-hind-cannon", ch, along(ch, [(-0.095, -0.03, 0.03, 0.028, 0.16), (-0.04, -0.018, 0.058, 0.045, 0.159), (-0.005, -0.012, 0.064, 0.047, 0.158, 2.8), (0.04, -0.01, 0.058, 0.043, 0.155, 2.8), (0.09, -0.012, 0.043, 0.032, 0.152, 2.5), (0.17, -0.014, 0.038, 0.028, 0.15, 2.4),
                                                (0.26, -0.014, 0.038, 0.028, 0.149, 2.4), (0.315, -0.016, 0.047, 0.036, 0.148), (0.355, -0.016, 0.052, 0.041, 0.147), (0.385, -0.008, 0.046, 0.039, 0.147)]), mirror=m),
        loft(f"{name}-hock-point", ch, [(-0.05, 0.53, 0.155, 0.036, 0.03), (-0.085, 0.575, 0.155, 0.032, 0.026), (-0.11, 0.61, 0.155, 0.025, 0.021), (-0.122, 0.628, 0.155, 0.013, 0.012)], exp=2.0, mirror=m),
        ellipsoid(f"{name}-hind-chestnut", ch, world(ch, 0.1, -0.005) + (0.122,), (0.018, 0.012, 0.006), material=SKIN, tone=0.15, role="detail", mirror=m),
        loft(f"{name}-hind-pastern", ph, along(ph, [(-0.035, -0.014, 0.047, 0.039, 0.147), (0.03, -0.004, 0.035, 0.032, 0.146), (0.085, -0.006, 0.032, 0.031, 0.146), (0.125, -0.01, 0.035, 0.037, 0.146), (0.155, -0.012, 0.039, 0.043, 0.146)]), mirror=m),
        ellipsoid(f"{name}-hind-ergot", ph, world(ph, 0.0, -0.047) + (0.147,), (0.011, 0.009, 0.011), material=SKIN, tone=0.25, role="detail", mirror=m),
    ]
    b += hoof(f"{name}-hind-hoof", hh, HIND_HOOF, HZ, (0, 1, 2, 3), m, rim=True)
    tx, ty = HIND_HOOF[3][1]
    b.append(ellipsoid(f"{name}-hind-heel", hh, (tx + 0.022, ty - 0.012, HZ), (0.017, 0.018, 0.038), tone=0.4, mirror=m))
    return b


def fore100(name, sfx, m):
    sc, hu, ra, cf, pf, hf = (f"{k}-f{sfx}" for k in ("scapula", "humerus", "radius", "cannon", "pastern", "hoof"))
    return [
        loft(f"{name}-shoulder", sc, along(sc, [(-0.03, 0.0, 0.045, 0.026, 0.08), (0.12, 0.006, 0.125, 0.05, 0.13), (0.3, 0.002, 0.13, 0.056, 0.158), (0.5, -0.004, 0.062, 0.048, 0.178)]), exp=2.0, mirror=m),
        loft(f"{name}-arm", hu, along(hu, [(-0.07, -0.005, 0.042, 0.04, 0.18), (0.0, -0.03, 0.078, 0.058, 0.192), (0.12, -0.08, 0.102, 0.064, 0.19), (0.24, -0.085, 0.085, 0.058, 0.19), (0.35, -0.055, 0.04, 0.04, 0.186)]), exp=2.1, mirror=m),
        loft(f"{name}-forearm", ra, along(ra, [(-0.06, -0.03, 0.068, 0.056, 0.186), (0.03, -0.004, 0.08, 0.06, 0.184), (0.2, 0.004, 0.06, 0.048, 0.172), (0.33, -0.002, 0.045, 0.04, 0.159), (0.39, 0.0, 0.048, 0.046, 0.153)]), exp=2.15, mirror=m),
        loft(f"{name}-fore-cannon", cf, along(cf, [(-0.085, -0.004, 0.03, 0.028, 0.156), (-0.035, -0.002, 0.046, 0.044, 0.156), (0.0, -0.004, 0.051, 0.05, 0.155, 2.7), (0.07, -0.008, 0.039, 0.031, 0.152, 2.5), (0.2, -0.01, 0.036, 0.028, 0.149, 2.4), (0.3, -0.013, 0.05, 0.04, 0.145), (0.335, -0.006, 0.044, 0.038, 0.144)]), mirror=m),
        loft(f"{name}-fore-pastern", pf, along(pf, [(-0.035, -0.014, 0.046, 0.038, 0.144), (0.06, -0.005, 0.032, 0.031, 0.143), (0.142, -0.012, 0.038, 0.044, 0.142)]), mirror=m),
    ] + hoof(f"{name}-fore-hoof", hf, FORE_HOOF, FZ, (0, 2, 3), m)


def hind100(name, sfx, m):
    th, sh, ch, ph, hh = (f"{k}-h{sfx}" for k in ("thigh", "shank", "cannon", "pastern", "hoof"))
    return [
        loft(f"{name}-thigh", th, [flat(1.45, -0.04, 0.16, 0.06, 0.125), flat(1.3, -0.042, 0.262, 0.104, 0.178), flat(1.12, -0.015, 0.26, 0.108, 0.19), flat(0.98, 0.022, 0.21, 0.096, 0.19), flat(0.88, 0.035, 0.13, 0.075, 0.185), flat(0.83, 0.035, 0.06, 0.05, 0.182)], exp=2.4, mirror=m),
        loft(f"{name}-gaskin", sh, along(sh, [(-0.06, -0.04, 0.088, 0.066, 0.188), (0.08, -0.055, 0.1, 0.07, 0.182), (0.24, -0.05, 0.08, 0.05, 0.17), (0.39, -0.035, 0.066, 0.044, 0.158), (0.44, -0.022, 0.056, 0.042, 0.156)]), mirror=m),
        loft(f"{name}-hind-cannon", ch, along(ch, [(-0.095, -0.03, 0.03, 0.028, 0.16), (-0.04, -0.018, 0.058, 0.045, 0.159), (-0.005, -0.012, 0.066, 0.047, 0.158, 2.8), (0.09, -0.012, 0.043, 0.032, 0.152, 2.5), (0.22, -0.014, 0.038, 0.028, 0.149, 2.4), (0.355, -0.016, 0.052, 0.041, 0.147), (0.385, -0.008, 0.046, 0.039, 0.147)]), mirror=m),
        loft(f"{name}-hock-point", ch, [(-0.05, 0.53, 0.155, 0.036, 0.03), (-0.09, 0.582, 0.155, 0.03, 0.025), (-0.122, 0.628, 0.155, 0.015, 0.014)], exp=2.0, mirror=m),
        loft(f"{name}-hind-pastern", ph, along(ph, [(-0.035, -0.014, 0.047, 0.039, 0.147), (0.065, -0.005, 0.033, 0.032, 0.146), (0.155, -0.012, 0.039, 0.043, 0.146)]), mirror=m),
    ] + hoof(f"{name}-hind-hoof", hh, HIND_HOOF, HZ, (0, 2, 3), m)


def fore200(name, sfx, m):
    sc, hu, ra, cf, pf, hf = (f"{k}-f{sfx}" for k in ("scapula", "humerus", "radius", "cannon", "pastern", "hoof"))
    return [
        loft(f"{name}-shoulder", sc, along(sc, [(-0.03, 0.0, 0.05, 0.03, 0.085), (0.22, 0.004, 0.135, 0.054, 0.148), (0.5, -0.004, 0.065, 0.048, 0.178)]), exp=2.0, mirror=m),
        loft(f"{name}-arm", hu, along(hu, [(-0.07, -0.005, 0.045, 0.042, 0.18), (0.12, -0.08, 0.104, 0.064, 0.19), (0.35, -0.055, 0.042, 0.042, 0.186)]), exp=2.1, mirror=m),
        loft(f"{name}-forearm", ra, along(ra, [(-0.06, -0.03, 0.068, 0.056, 0.186), (0.05, -0.002, 0.079, 0.06, 0.183), (0.39, 0.0, 0.048, 0.046, 0.153)]), exp=2.15, mirror=m),
        loft(f"{name}-fore-cannon", cf, along(cf, [(-0.08, -0.004, 0.032, 0.03, 0.156), (0.0, -0.004, 0.052, 0.05, 0.155), (0.15, -0.01, 0.037, 0.03, 0.15), (0.31, -0.012, 0.05, 0.04, 0.145)]), mirror=m),
        loft(f"{name}-fore-pastern", pf, along(pf, [(-0.035, -0.014, 0.046, 0.038, 0.144), (0.142, -0.012, 0.038, 0.044, 0.142)]), mirror=m),
    ] + hoof(f"{name}-fore-hoof", hf, FORE_HOOF, FZ, (0, 3), m)


def hind200(name, sfx, m):
    th, sh, ch, ph, hh = (f"{k}-h{sfx}" for k in ("thigh", "shank", "cannon", "pastern", "hoof"))
    return [
        loft(f"{name}-thigh", th, [flat(1.45, -0.04, 0.16, 0.06, 0.125), flat(1.24, -0.035, 0.27, 0.108, 0.184), flat(1.0, 0.02, 0.22, 0.098, 0.19), flat(0.85, 0.035, 0.1, 0.066, 0.183)], exp=2.4, mirror=m),
        loft(f"{name}-gaskin", sh, along(sh, [(-0.06, -0.045, 0.088, 0.066, 0.188), (0.12, -0.056, 0.1, 0.068, 0.18), (0.42, -0.035, 0.06, 0.042, 0.157)]), mirror=m),
        loft(f"{name}-hind-cannon", ch, along(ch, [(-0.09, -0.03, 0.032, 0.03, 0.16), (-0.01, -0.014, 0.066, 0.047, 0.158), (0.15, -0.014, 0.038, 0.03, 0.15), (0.36, -0.016, 0.052, 0.041, 0.147)]), mirror=m),
        loft(f"{name}-hind-pastern", ph, along(ph, [(-0.035, -0.014, 0.047, 0.039, 0.147), (0.155, -0.012, 0.039, 0.043, 0.146)]), mirror=m),
    ] + hoof(f"{name}-hind-hoof", hh, HIND_HOOF, HZ, (0, 3), m)


def fore500(name, sfx, m):
    hu, ra, cf = (f"{k}-f{sfx}" for k in ("humerus", "radius", "cannon"))
    t0, h0, _ = FORE_HOOF[0]
    return [
        loft(f"{name}-fore-upper", hu, [(1.15, 1.48, 0.1, 0.06, 0.04), (1.24, 1.33, 0.145, 0.13, 0.06), (1.32, 1.13, 0.17, 0.14, 0.065), (1.22, 0.94, 0.183, 0.09, 0.055), (1.17, 0.86, 0.185, 0.05, 0.045)], exp=2.1, mirror=m),
        loft(f"{name}-forearm", ra, along(ra, [(-0.06, -0.02, 0.07, 0.054, 0.186), (0.06, -0.002, 0.078, 0.058, 0.182), (0.39, 0.0, 0.0475, 0.045, 0.153)]), mirror=m),
        loft(f"{name}-fore-lower", cf, [(1.165, 0.52, 0.154, 0.05, 0.048, -90.0), (1.16, 0.33, 0.149, 0.0375, 0.0375, -90.0), (1.16, 0.18, 0.145, 0.05, 0.042, -90.0), (1.215, 0.075, 0.142, 0.052, 0.05, -90.0), ((t0[0] + h0[0]) / 2, 0.0, FZ, (t0[0] - h0[0]) / 2, 0.064, -90.0)], mirror=m),
    ]


def hind500(name, sfx, m):
    th, sh, ch = (f"{k}-h{sfx}" for k in ("thigh", "shank", "cannon"))
    t0, h0, _ = HIND_HOOF[0]
    return [
        loft(f"{name}-hind-upper", th, [flat(1.45, -0.04, 0.16, 0.06, 0.125), flat(1.22, -0.035, 0.27, 0.108, 0.184), flat(0.98, 0.022, 0.205, 0.095, 0.19), flat(0.9, 0.032, 0.15, 0.08, 0.186), flat(0.83, 0.035, 0.08, 0.06, 0.182)], exp=2.4, mirror=m),
        loft(f"{name}-gaskin", sh, along(sh, [(-0.06, -0.045, 0.088, 0.066, 0.188), (0.12, -0.056, 0.1, 0.068, 0.18), (0.43, -0.035, 0.062, 0.044, 0.157)]), mirror=m),
        loft(f"{name}-hind-lower", ch, [(-0.06, 0.6, 0.158, 0.066, 0.048, -90.0), (-0.035, 0.38, 0.15, 0.0375, 0.0375, -90.0), (-0.03, 0.19, 0.147, 0.05, 0.042, -90.0), (0.02, 0.075, 0.146, 0.052, 0.048, -90.0), ((t0[0] + h0[0]) / 2, 0.0, HZ, (t0[0] - h0[0]) / 2, 0.06, -90.0)], mirror=m),
    ]


LEGS = {50: (fore50, hind50), 100: (fore100, hind100), 200: (fore200, hind200), 500: (fore500, hind500)}


def body(n):
    """🧩 The body list of scale 1:n."""
    b = trunk(n) + neck(n) + head(n) + hair(n)
    fore, hind = LEGS[n]
    for name, sfx, m in SIDES:
        b += fore(name, sfx, m) + hind(name, sfx, m)
    return b
