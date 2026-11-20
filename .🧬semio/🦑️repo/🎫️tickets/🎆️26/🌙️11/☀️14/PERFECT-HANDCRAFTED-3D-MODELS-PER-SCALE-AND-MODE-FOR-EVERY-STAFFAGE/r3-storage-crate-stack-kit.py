"""📦 Round 3 storage crate stack: the 600 x 400 x 300 rotating stack-nest crate stated by hand (crate frame: u up along the bone, v = -world x, w = world z; profiles are (v, w))."""
import math

PITCH = 0.30
NEST = 0.22
LIFT = 0.02
U0 = 0.012
SLOPE = 0.1 / 0.258
HV, HW, R = 0.265, 0.165, 0.025
T = 0.003
HAND = {"g": 0.06, "u0": 0.215, "u1": 0.25}
RIM = {"hv": 0.30, "hw": 0.20, "r": 0.03, "t": 0.003}
FLANGE_HOLE = {"hv": 0.289, "hw": 0.1805, "r": 0.02}
COM_U = 0.13

FINE = {"p1": (0.065, 0.125, 0.040), "p3": (-0.06, 0.06, 0.032), "edge1": 0.247, "edge3": 0.148, "ledge_u": 0.232}
TINY = {"p1": (0.055, 0.130, 0.080), "p3": (-0.04, 0.04, 0.075), "edge1": 0.2110, "edge3": 0.1028, "ledge_u": 0.225}
COLLAR = {"200": 0.27, "500": 0.225}

def f(u):
    """📐 Scale of the wall outline at height u against the outline at the foot of the wall (u = U0)."""
    return 1 + SLOPE * (u - U0)


def r6(x):
    """🔢 Rounds to 0.1 mm for clean source numbers."""
    return round(x + 0.0, 5)


def arc(cv, cw, r, a0, a1, n=6):
    """◜ Points of a corner arc from angle a0 to a1 (degrees), n segments, end point excluded."""
    return [(cv + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cw + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n)]


def outline(hv, hw, r, notch1=None, notch3=None, cut=None, n=6):
    """🔲 Counter-clockwise rounded rectangle (half sizes hv, hw, radius r) with the stacking channels: notch1 = (w0, w1, v) indents both end walls to |v| = v over w0..w1, notch3 = (v0, v1, w) indents the -w side wall to w = -w over v0..v1; cut = g adds points at w = +-g on both end walls (hand-hole gaps)."""
    c = 0.004
    pts = []
    right = [(hv, -hw + r)]
    if cut:
        right.append((hv, -cut))
        right.append((hv, cut))
    if notch1:
        w0, w1, v = notch1
        right += [(hv, w0), (v, w0 + min(c, (w1 - w0) / 4)), (v, w1 - min(c, (w1 - w0) / 4)), (hv, w1)]
    pts += right
    pts += arc(hv - r, hw - r, r, 0, 90, n)
    pts += arc(-hv + r, hw - r, r, 90, 180, n)
    left = []
    if notch1:
        w0, w1, v = notch1
        left += [(-hv, w1), (-v, w1 - min(c, (w1 - w0) / 4)), (-v, w0 + min(c, (w1 - w0) / 4)), (-hv, w0)]
    if cut:
        left += [(-hv, cut), (-hv, -cut)]
    pts += left
    pts += arc(-hv + r, -hw + r, r, 180, 270, n)
    if notch3:
        v0, v1, w = notch3
        pts += [(v0, -hw), (v0 + min(c, (v1 - v0) / 4), -w), (v1 - min(c, (v1 - v0) / 4), -w), (v1, -hw)]
    pts += arc(hv - r, -hw + r, r, 270, 360, n)
    return [[r6(a), r6(b)] for a, b in pts]


def wall_outline(p, inset=0.0, hv=HV, hw=HW, r=R, cut=None, n=6):
    """🧱 Outline of the wall (or of a shape set in by `inset`, positive inwards) with the scale's channels p; the channel span grows by the inset so that the offset stays a constant distance."""
    w0, w1, d1 = p["p1"]
    v0, v1, d3 = p["p3"]
    return outline(hv - inset, hw - inset, max(r - inset, 0.004), (w0 - inset, w1 + inset, hv - d1 - inset), (v0 - inset, v1 + inset, hw - d3 - inset), cut, n)


def centroid(poly):
    """⊕ Area centroid of a simple polygon."""
    a = cx = cy = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        k = x0 * y1 - x1 * y0
        a += k
        cx += (x0 + x1) * k
        cy += (y0 + y1) * k
    return (cx / (3 * a), cy / (3 * a))


def band_piece(p, side, t=T, n=6):
    """🫳 One half of the hand-hole band (side +1: the +w half, -1: the -w half): the wall ring between the two hand holes as one simple polygon, the outer path and then the inner path back."""
    g = HAND["g"]
    outer = wall_outline(p, 0.0, cut=g, n=n)
    inner = wall_outline(p, t, cut=g, n=n)

    def path(poly, hv):
        a = poly.index([r6(hv), r6(g * side)])
        b = poly.index([r6(-hv), r6(g * side)])
        return poly[a:b + 1] if side > 0 else poly[b:] + poly[:a + 1]

    return path(outer, HV) + list(reversed(path(inner, HV - t)))
