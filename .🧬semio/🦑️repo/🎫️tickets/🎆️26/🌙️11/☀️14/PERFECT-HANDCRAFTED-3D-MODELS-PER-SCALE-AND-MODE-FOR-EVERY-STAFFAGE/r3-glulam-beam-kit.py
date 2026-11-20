"""🪵 Round 3 glulam beam kit: the lamella lay-up shared by the 2D root script and the volume script.

Fifteen 40 mm lamellae (EN 14080 GL24h, at most 45 mm), each finger-jointed from boards of about 1.4 to 4.6 m (sawn spruce lengths),
joints staggered between neighbouring lamellae and kept out of the middle third of the outer tension lamella.
The finger profile is visible on the narrow face of the lamella (the beam side); at 1:50 it is the two-finger symbol
of 20 mm finger length and 20 mm pitch (real 15/3.8 or 20/6.2 mm is below the printable minimum).

See: r3-components-glulam-beam.md, round3-brief.md
"""

LENGTH = 8.0
HALF_DEPTH = 0.3
HALF_WIDTH = 0.08
LAMELLA = 0.04
COUNT = 15
FINGER = 0.02
PLATE = (0.12, 0.02)
BEARINGS = (0.06, 7.94)

JOINTS = [
    [1.85, 6.30],
    [3.10],
    [0.95, 5.35],
    [4.05],
    [2.35, 6.75],
    [5.70],
    [1.40, 4.90],
    [3.55, 7.35],
    [2.70, 6.10],
    [4.60],
    [1.15, 5.45],
    [3.85, 7.05],
    [2.05, 6.55],
    [4.35],
    [2.60, 5.90],
]


def r6(v):
    """🎯 Round to a micrometre."""
    return round(v + 0.0, 6)


def bottom(k):
    """⬇️ Bone-local v of the underside of lamella k (0 = soffit lamella)."""
    return r6(-HALF_DEPTH + LAMELLA * k)


def zig(k, x):
    """⚡ The finger profile of lamella k at joint x, from the underside up: two fingers of FINGER length."""
    y0 = bottom(k)
    return [[r6(x - FINGER / 2 + (FINGER if i % 2 else 0)), r6(y0 + LAMELLA * i / 4)] for i in range(5)]


def boards(k):
    """🪚 The board outlines (x, v) of lamella k between its ends and finger joints, counter-clockwise."""
    y0, y1 = bottom(k), r6(bottom(k) + LAMELLA)
    cuts = [None] + JOINTS[k] + [None]
    out = []
    for a, b in zip(cuts, cuts[1:]):
        right = zig(k, b) if b is not None else [[LENGTH, y0], [LENGTH, y1]]
        left = list(reversed(zig(k, a))) if a is not None else [[0.0, y1], [0.0, y0]]
        out.append(right + left)
    return out


def top_trace(x):
    """🔝 Where the finger joint at x meets the top face of the top lamella."""
    return zig(COUNT - 1, x)[-1][0]
