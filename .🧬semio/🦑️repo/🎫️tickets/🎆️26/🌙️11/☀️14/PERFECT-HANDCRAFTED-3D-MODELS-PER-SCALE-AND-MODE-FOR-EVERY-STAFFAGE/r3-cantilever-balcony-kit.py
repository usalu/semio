"""🧱 Round 3 cantilever balcony: the real dimensions of the strip and the small solid and shape writers every scale script states its parts with."""
import math

FALL = 0.015
SLAB_L = 1.5
ROOT_H = 0.2
POST_X = 1.38
RAIL_H = 1.1
HAND_R = 0.03015
HAND_T = 0.0029
HAND_U = RAIL_H - HAND_R
PAD_TOP = 0.0925
PLATE_T = 0.012
POST_TOP = 0.994
CAP_T = 0.008
CHORD_LOW = 0.085
CHORD_TOP = 0.955
BALUSTER_R = 0.006
BALUSTER_W = [0.12589, 0.23278, 0.33967, 0.44656]
ANCHOR = 0.055
TENSION_V = 0.044
TOP_BAR_DROP = 0.054
CROSS_TOP_DROP = 0.064
BOTTOM_V = -0.066
CROSS_BOTTOM_V = -0.058
BAR_W = [-0.4, -0.2, 0.0, 0.2, 0.4]
BEARING_W = [-0.3, -0.1, 0.1, 0.3]
SHEAR_W = [-0.35, -0.15, 0.15, 0.35]
CROSS_TOP_X = [0.15, 0.4, 0.65, 0.9, 1.15, 1.4]
CROSS_BOTTOM_X = [0.1, 0.35, 0.6, 0.85, 1.1, 1.35]


def r5(x):
    return round(x, 5) + 0.0


def top(x):
    """📐 The slab top in the slab frame at x: 0.1 at the wall, falling 1.5 % to the edge."""
    return 0.1 - FALL * x


POST_V = top(POST_X)
PLATE_U0 = PAD_TOP - POST_V
PLATE_U1 = PLATE_U0 + PLATE_T
SLAB_PROFILE = [[0.0, -0.1], [1.405, -0.1], [1.41, -0.085], [1.425, -0.085], [1.43, -0.1], [1.49, -0.1], [1.5, -0.09], [1.5, r5(top(1.5) - 0.01)], [1.49, r5(top(1.49))], [0.0, 0.1]]
SLAB_WEDGE = [[0.0, -0.1], [1.5, -0.1], [1.5, r5(top(1.5))], [0.0, 0.1]]
PAD_PROFILE = [[1.295, r5(top(1.295))], [1.465, r5(top(1.465))], [1.465, PAD_TOP], [1.295, PAD_TOP]]


def polygon(points):
    """⚖️ Area and centroid of a simple polygon."""
    a = cx = cy = 0.0
    for k in range(len(points)):
        (x0, y0), (x1, y1) = points[k], points[(k + 1) % len(points)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), cx / (6 * a), cy / (6 * a)


def arc(c, r, a0, a1, n=6):
    return [[r5(c[0] + r * math.cos(math.radians(a0 + (a1 - a0) * k / n))), r5(c[1] + r * math.sin(math.radians(a0 + (a1 - a0) * k / n)))] for k in range(n + 1)]


def rounded(half, r, n=4):
    """🔲 A square of the half width with corner radius r, counter-clockwise."""
    out = []
    for cx, cy, a0 in ((half - r, half - r, 0), (-(half - r), half - r, 90), (-(half - r), -(half - r), 180), (half - r, -(half - r), 270)):
        out.extend(arc((cx, cy), r, a0, a0 + 90, n))
    return out


def hexagon(across_flats):
    R = across_flats / math.sqrt(3)
    return [[r5(R * math.cos(math.radians(60 * k))), r5(R * math.sin(math.radians(60 * k)))] for k in range(6)]


def rail(dx, dy, w=0.0):
    """🧭 A point given ahead of and above the post foot in the railing frame (u up, v towards the wall, w lateral)."""
    return [r5(dy), r5(-dx), r5(w)]


TENSION_PATH = [[-0.25, -0.1], [-0.25, 0.009]] + arc((-0.22, 0.014), 0.03, 180, 90) + [[-0.215, TENSION_V], [0.7, TENSION_V]]
SHEAR_PATH = [[-0.28, 0.03], [-0.15, 0.03], [-0.13, 0.027], [0.0, -0.058], [0.02, BOTTOM_V + 0.004], [0.04, BOTTOM_V], [0.45, BOTTOM_V]]
EDGE_PATH = [[1.11, r5(top(1.11) - TOP_BAR_DROP)], [1.431, r5(top(1.431) - TOP_BAR_DROP)]] + arc((1.436, r5(top(1.436) - TOP_BAR_DROP - 0.024)), 0.024, 90, 0) + [[1.46, -0.037]] + arc((1.436, -0.042), 0.024, 0, -90) + [[1.431, BOTTOM_V], [1.11, BOTTOM_V]]
TOP_BAR = [[0.03, r5(top(0.03) - TOP_BAR_DROP)], [1.44, r5(top(1.44) - TOP_BAR_DROP)]]
BOTTOM_BAR = [[0.03, BOTTOM_V], [1.44, BOTTOM_V]]


def solid(i, kind, bone, material, tone, role="body", **extra):
    return {"id": i, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **extra}


def box(i, bone, material, tone, size, at, role="body", **extra):
    return solid(i, "box", bone, material, tone, role, size=[r5(s) for s in size], transform={"translation": [r5(c) for c in at]}, **extra)


def prism(i, bone, material, tone, profile, plane, lo, hi, role="body", **extra):
    return solid(i, "extrude", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], plane=plane, **{"from": r5(lo), "to": r5(hi)}, **extra)


def rod(i, bone, material, tone, origin, axis, length, radius, role="body", **extra):
    """🥢 A solid round bar from origin along axis."""
    n = math.sqrt(sum(c * c for c in axis))
    return solid(i, "revolve", bone, material, tone, role, profile=[[0.0, r5(radius)], [r5(length), r5(radius)]], axis=[round(c / n, 6) for c in axis], origin=[r5(c) for c in origin], **extra)


def tube(i, bone, material, tone, origin, axis, length, radius, wall, role="body", **extra):
    """🧻 A hollow round tube with its real wall."""
    inner = r5(radius - wall)
    return solid(i, "revolve", bone, material, tone, role, profile=[[0.0, inner], [r5(length), inner], [r5(length), r5(radius)], [0.0, r5(radius)], [0.0, inner]], axis=list(axis), origin=[r5(c) for c in origin], **extra)


def bar(i, bone, material, tone, path, w, radius, role="body", **extra):
    """🪝 A bent reinforcing bar: round sections along a path in the plane of the section at the lateral position w."""
    return solid(i, "loft", bone, material, tone, role, sections=[{"at": [r5(x), r5(y), r5(w)], "rx": radius, "ry": radius} for x, y in path], capStart=True, capEnd=True, **extra)


def repeat(s, count, step):
    s["repeat"] = {"count": count, "translation": [r5(c) for c in step]}
    return s


def cut(s, *faces):
    s["sectionFaces"] = list(faces)
    return s


def th(s, region, layer=None):
    s["region"] = region
    if layer:
        s["layer"] = layer
    return s


def th_of(i, of, region, layer=None, faces=None):
    out = {"id": i, "of": of, "region": region}
    if layer:
        out["layer"] = layer
    if faces:
        out["sectionFaces"] = list(faces)
    return out


def st(s, member, part="member", anchor=None):
    s["member"] = member
    s["part"] = part
    if anchor:
        s["anchor"] = anchor
    return s


def st_of(i, of, member, part="member", anchor=None):
    out = {"id": i, "of": of, "member": member, "part": part}
    if anchor:
        out["anchor"] = anchor
    return out


def text(en, de):
    return {"en": en, "de": de}


def shape(i, bone, role, z, filled, tone, geometry, region=None, regions=None, material=None, hatch=None, pen=None):
    out = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen:
        out["penMm"] = pen
    out.update(geometry)
    return out


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}


def poly(points, closed=True):
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def disc(cx, cy, r):
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(r), "ry": r5(r)}


def skins(slab, chamfer):
    """🌡️ The three 80 mm storage skins of the slab as wedges that meet on mitres inside the concrete, so each exposes only its own outer face."""
    tip = top(1.5)
    if chamfer:
        top_skin = [[0.0, 0.1], [0.16, r5(top(0.16) - 0.08)], [1.34, 0.0], [1.5, r5(tip - 0.01)], [1.49, r5(top(1.49))]]
        soffit = [p for p in SLAB_PROFILE[:7]] + [[1.34, -0.02], [0.16, -0.02]]
        edge = [[1.5, r5(tip - 0.01)], [1.34, 0.0], [1.34, -0.02], [1.5, -0.09]]
    else:
        top_skin = [[0.0, 0.1], [0.16, r5(top(0.16) - 0.08)], [1.34, 0.0], [1.5, r5(tip)]]
        soffit = [[0.0, -0.1], [1.5, -0.1], [1.34, -0.02], [0.16, -0.02]]
        edge = [[1.5, r5(tip)], [1.34, 0.0], [1.34, -0.02], [1.5, -0.1]]
    return [
        th(cut(prism("th-slab-top", slab, "concrete", 0.75, top_skin, "xy", -0.5, 0.5), "-v", "-w", "+w"), "slab-top"),
        th(cut(prism("th-slab-soffit", slab, "concrete", 0.6, soffit, "xy", -0.5, 0.5), "+v", "-w", "+w"), "slab-soffit"),
        th(cut(prism("th-slab-edge", slab, "concrete", 0.68, edge, "xy", -0.5, 0.5), "-u", "+v", "-v", "-w", "+w"), "slab-edge"),
    ]
