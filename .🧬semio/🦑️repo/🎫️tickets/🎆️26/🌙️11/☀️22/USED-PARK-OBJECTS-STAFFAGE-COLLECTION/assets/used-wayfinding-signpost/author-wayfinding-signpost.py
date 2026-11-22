"""🪧 Authoring script of `used-wayfinding-signpost` (run it to regenerate `🔣️.json`). Hand-chosen geometry of a municipal park finger signpost (world rest metres: x ahead, y up, z left; origin on the post axis at ground level), the 3D rest frames of its bones and the helpers that turn the one set of numbers into bone-local 2D shapes and 3D solids.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Front and plan views", "Rest placement of a bone", "Axial twist", "Volume"
"""
import json
import math
import pathlib

R_POST = 0.03805
T_POST = 0.0032
Y_POST0 = -0.70
Y_POST1 = 2.96
Y_CAP1 = 2.975
R_CAP = 0.0395
R_FOOT = 0.20
Y_FOOT0 = -0.80
Y_RUST = 0.12
R_RING = 0.047
H_RING = 0.10
FINGER_L = 0.65
FINGER_STRAIGHT = 0.575
FINGER_H = 0.15
FINGER_T = 0.025
R_IN = 0.07
HOLDER_R = (0.045, 0.165)
HOLDER_H = 0.12
HOLDER_T = 0.033
EAR = {"r": (R_RING - 0.003, R_RING + 0.03), "t": 0.012}
SLIDE = 0.085
R_COLLAR_100 = R_POST + 0.0155
R_SLEEVE_500 = 0.0455
FINGERS = [
    {"id": "finger-1", "clamp": "clamp-1", "y": 2.25, "tau": -40.0, "true": -75.0, "region": "finger-1-foil", "material": "sign-foil-weathered", "cut500": 0.03},
    {"id": "finger-2", "clamp": "clamp-2", "y": 2.425, "tau": 160.0, "true": 160.0, "region": "finger-2-graffiti", "material": "sign-foil-sprayed", "cut500": 0.0},
    {"id": "finger-3", "clamp": "clamp-3", "y": 2.60, "tau": 30.0, "true": 30.0, "region": "finger-3-foil", "material": "sign-foil-weathered", "cut500": 0.03},
    {"id": "finger-4", "clamp": "clamp-4", "y": 2.775, "tau": -150.0, "true": -150.0, "region": "finger-4-foil", "material": "sign-foil-weathered", "cut500": 0.0},
]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SEGMENTS = {50: 64, 100: 48, 200: 32, 500: 24}
ZINC, RUST, CONC, ALU = "zinc-weathered", "steel-rusted", "concrete-weathered", "aluminium-cast-coated"
SIGN_STEP = 0.175


def r5(value):
    """🔢 Rounds a coordinate to 0.01 mm and removes negative zero."""
    out = round(value, 5)
    return 0.0 if out == 0 else out


def tx(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def add(a, b):
    """➕ Sum of two 3-vectors."""
    return tuple(p + q for p, q in zip(a, b))


def mul(k, a):
    """✖️ A 3-vector scaled by k."""
    return tuple(k * p for p in a)


def dot(a, b):
    """⚫ Dot product of two 3-vectors."""
    return sum(p * q for p, q in zip(a, b))


def frames():
    """🦴 The rest frames (origin, u, v, w) of every bone in world coordinates, composed as the product does: footing down, post up, each clamp a vertical bone twisted to its finger's direction, each finger along -v of its clamp."""
    out = {"footing": ((0.0, 0.0, 0.0), (0.0, -1.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0))}
    out["post"] = ((0.0, Y_POST0, 0.0), (0.0, 1.0, 0.0), (-1.0, 0.0, 0.0), (0.0, 0.0, 1.0))
    for f in FINGERS:
        t = math.radians(f["tau"])
        o = (0.0, f["y"], 0.0)
        out[f["clamp"]] = (o, (0.0, 1.0, 0.0), (-math.cos(t), 0.0, -math.sin(t)), (-math.sin(t), 0.0, math.cos(t)))
        out[f["id"]] = ((R_IN * math.cos(t), f["y"], R_IN * math.sin(t)), (math.cos(t), 0.0, math.sin(t)), (0.0, 1.0, 0.0), (-math.sin(t), 0.0, math.cos(t)))
    return out


FR = frames()


def to_world(bone, u, v, w=0.0):
    """🌍 Bone-local (u, v, w) of `bone` at rest to world (x, y, z)."""
    o, eu, ev, ew = FR[bone]
    return add(o, add(mul(u, eu), add(mul(v, ev), mul(w, ew))))


def local(bone, p):
    """📐 World rest point p = (x, y, z) to bone-local (u, v, w) of `bone`."""
    o, eu, ev, ew = FR[bone]
    d = tuple(a - b for a, b in zip(p, o))
    return (r5(dot(d, eu)), r5(dot(d, ev)), r5(dot(d, ew)))


def luv(bone, x, y):
    """📍 World rest point in the sagittal plane (x, y, 0) to the bone-local (u, v) of a sagittal bone."""
    u, v, _ = local(bone, (x, y, 0.0))
    return (u, v)


def hull(points):
    """🪢 Convex hull (counter-clockwise, no repeated point) of 2D points by the monotone chain."""
    pts = sorted(set((r5(a), r5(b)) for a, b in points))
    if len(pts) <= 2:
        return pts
    cross = lambda o, a, b: (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def shape(sid, bone, role, z, filled, tone, geom, region=None, material=None, hatch=None, pen=None, solid=None):
    """✏️ One authored 2D shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    for key, val in (("region", region), ("material", material), ("hatch", hatch), ("penMm", pen), ("solid", solid)):
        if val:
            out[key] = val
    return out


def rect(x0, y0, x1, y1):
    """⬛ Rectangle geometry from two corners."""
    return {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}


def poly(points, closed=True):
    """🔺 Polygon or polyline geometry."""
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def ellipse(cx, cy, rx, ry):
    """🥚 Ellipse geometry."""
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(rx), "ry": r5(ry)}


def path(d):
    """〰️ SVG path geometry."""
    return {"kind": "path", "d": d}


def solid(sid, kind, bone, material, role, tone, **fields):
    """🧊 One volume solid."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def rev(sid, bone, material, role, tone, profile, axis, origin, **extra):
    """🌀 A revolve solid."""
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(b)] for a, b in profile], axis=axis, origin=[r5(c) for c in origin], **extra)


def ext(sid, bone, material, role, tone, profile, plane, frm, to, **extra):
    """🧱 An extrude solid."""
    return solid(sid, "extrude", bone, material, role, tone, profile=[[r5(a), r5(b)] for a, b in profile], plane=plane, **{"from": r5(frm), "to": r5(to)}, **extra)


def box(sid, bone, material, role, tone, size, centre, **extra):
    """📦 A box solid centred at `centre` (bone-local)."""
    return solid(sid, "box", bone, material, role, tone, size=[r5(s) for s in size], transform={"translation": [r5(c) for c in centre]}, **extra)


def ell(sid, bone, material, radii, centre, **extra):
    """🔵 A mass marker ellipsoid."""
    return solid(sid, "ellipsoid", bone, material, "detail", 0.15, radii=[r5(r) for r in radii], transform={"translation": [r5(c) for c in centre]}, **extra)


def annulus_profile(a0, a1, r0, r1):
    """🔘 A closed revolve loop of a rectangle in (axial, radial): a collar or a band."""
    return [(a0, r0), (a1, r0), (a1, r1), (a0, r1), (a0, r0)]


def arrow(s, f=None):
    """➡️ The finger outline in its own (u, v) plane from its inner end in the holder: straight body to 575 mm, then the 45-degree arrow point at mid height; at 1:500 the point of the two fingers that set the lateral reach is set back by their `cut500` so that the 75 mm thick finger keeps the real extent of the asset."""
    cut = f["cut500"] if s == 500 and f else 0.0
    u0 = R_SLEEVE_500 - 0.005 - R_IN if s == 500 else 0.0
    return [(u0, 0.0), (FINGER_STRAIGHT - cut, 0.0), (FINGER_L - cut, FINGER_H / 2), (FINGER_STRAIGHT - cut, FINGER_H), (u0, FINGER_H)]


def swept(bone, prof_uv, half_t):
    """🪟 The 2D geometry and `solid` placement of a prism of the bone's (u, v) profile, `half_t` either side along w: the lateral span is stated in rest world z about the bone origin, so the product projects it right in every view and pose."""
    zo = FR[bone][0][2]
    return poly(prof_uv), {"kind": "extruded", "sweep": "w", "z0": r5(zo - half_t), "z1": r5(zo + half_t)}


class Scale:
    """📏 Collects the 2D shapes and 3D solids of one authored scale, built from the one set of numbers above."""

    def __init__(self, s):
        self.s = s
        self.n = SEGMENTS[s]
        self.elev, self.front, self.plan = [], [], []
        self.body, self.thermal, self.structure = [], [], []

    def th(self, sid, region):
        """🌡️ A thermal reference to a body solid."""
        self.thermal.append({"id": "th-" + sid, "of": sid, "region": region})

    def st(self, sid, member, part, **extra):
        """🏗️ A structural reference to a body solid."""
        self.structure.append({"id": "st-" + sid, "of": sid, "member": member, "part": part, **extra})

    def swept3(self, sid, bone, z, tone, prof_uv, half_t, region, material):
        """🪟 A prism on a twisted bone in the three views, each a `solid` of the same profile."""
        g, sol = swept(bone, prof_uv, half_t)
        for view in (self.elev, self.front, self.plan):
            view.append(shape(sid, bone, "body", z, True, tone, g, region, material, solid=sol))

    def three(self, sid, bone, role, z, tone, elev, front, plan, region, material, hatch=None):
        """🖼️ The same part in the three views (elevation bone-local, front and plan in world rest coordinates)."""
        self.elev.append(shape(sid, bone, role, z, True, tone, elev, region, material, hatch))
        self.front.append(shape(sid, bone, role, z, True, tone, front, region, material, hatch))
        self.plan.append(shape(sid, bone, role, z, True, tone, plan, region, material, hatch))


def finger_t(s):
    """📏 The modelled finger thickness at 1:s: the true 25 mm profile, thickened to the smallest feature where it is below it."""
    return max(FINGER_T, FEATURE[s])


def build(s):
    """🏗️ All shapes and solids of scale 1:s."""
    k = Scale(s)
    fine = s == 50
    hatch = "concrete-stipple" if s <= 100 else None

    k.three("footing", "footing", "body", 1, 0.62, poly([luv("footing", x, y) for x, y in ((-R_FOOT, Y_FOOT0), (R_FOOT, Y_FOOT0), (R_FOOT, 0.0), (-R_FOOT, 0.0))]), rect(-R_FOOT, Y_FOOT0, R_FOOT, 0.0), ellipse(0, 0, R_FOOT, R_FOOT), "footing-concrete", CONC, hatch)
    k.body.append(rev("footing", "footing", CONC, "body", 0.62, [(-Y_FOOT0, 0), (-Y_FOOT0, R_FOOT), (0, R_FOOT), (0, R_POST), (-Y_POST0, R_POST), (-Y_POST0, 0)], [1, 0, 0], [0, 0, 0]))
    k.th("footing", "footing-concrete")
    k.st("footing", "footing", "member")

    top = Y_POST1 if fine else Y_CAP1
    k.three("post-foot", "post", "body", 2, 0.3, poly([luv("post", x, y) for x, y in ((-R_POST, 0.0), (R_POST, 0.0), (R_POST, Y_RUST), (-R_POST, Y_RUST))]), rect(-R_POST, 0.0, R_POST, Y_RUST), ellipse(0, 0, R_POST, R_POST), "post-rust", RUST)
    k.body.append(rev("post-foot", "post", RUST, "body", 0.3, [(0, 0), (0, R_POST), (Y_RUST - Y_POST0, R_POST), (Y_RUST - Y_POST0, 0)], [1, 0, 0], [0, 0, 0]))
    k.th("post-foot", "post-rust")
    k.st("post-foot", "post", "member")
    k.three("post", "post", "body", 2.1, 0.48, poly([luv("post", x, y) for x, y in ((-R_POST, Y_RUST), (R_POST, Y_RUST), (R_POST, top), (-R_POST, top))]), rect(-R_POST, Y_RUST, R_POST, top), ellipse(0, 0, R_POST, R_POST), "post-zinc", ZINC)
    k.body.append(rev("post", "post", ZINC, "body", 0.48, [(Y_RUST - Y_POST0, 0), (Y_RUST - Y_POST0, R_POST), (top - Y_POST0, R_POST), (top - Y_POST0, 0)], [1, 0, 0], [0, 0, 0]))
    k.th("post", "post-zinc")
    k.st("post", "post", "member")
    if fine:
        cap = [(0, R_CAP), (0.004, R_CAP), (0.008, 0.034), (0.0115, 0.025), (0.0138, 0.014), (0.015, 0)]
        u0 = Y_POST1 - Y_POST0
        k.body.append(rev("post-cap", "post", ZINC, "body", 0.4, [(u0, 0)] + [(u0 + a, r) for a, r in cap], [1, 0, 0], [0, 0, 0]))
        side = [(r, Y_POST1 + a) for a, r in cap[:-1]] + [(0, Y_CAP1)] + [(-r, Y_POST1 + a) for a, r in reversed(cap[:-1])]
        k.elev.append(shape("post-cap", "post", "body", 2.2, True, 0.4, poly([luv("post", x, y) for x, y in side]), "post-zinc", ZINC))
        k.front.append(shape("post-cap", "post", "body", 2.2, True, 0.4, poly(side), "post-zinc", ZINC))
        k.plan.append(shape("post-cap", "post", "body", 9.5, True, 0.4, ellipse(0, 0, R_CAP, R_CAP), "post-zinc", ZINC))
        k.th("post-cap", "post-zinc")
        k.st("post-cap", "post", "member")

    if s == 500:
        y0, y1 = FINGERS[0]["y"] + FINGER_H / 2 - H_RING / 2, FINGERS[-1]["y"] + FINGER_H / 2 + H_RING / 2
        octa = [(R_SLEEVE_500 * math.cos(math.pi * j / 4), R_SLEEVE_500 * math.sin(math.pi * j / 4)) for j in range(8)]
        k.elev.append(shape("clamp-sleeve", "post", "body", 4.6, True, 0.42, poly([luv("post", x, y) for x, y in ((-R_SLEEVE_500, y0), (R_SLEEVE_500, y0), (R_SLEEVE_500, y1), (-R_SLEEVE_500, y1))]), "fittings", ALU))
        k.front.append(shape("clamp-sleeve", "post", "body", 4.6, True, 0.42, rect(-R_SLEEVE_500, y0, R_SLEEVE_500, y1), "fittings", ALU))
        k.plan.append(shape("clamp-sleeve", "post", "body", 4.6, True, 0.42, poly(octa), "fittings", ALU))
        k.body.append(ext("clamp-sleeve", "post", ALU, "body", 0.42, [(-x, z) for x, z in octa], "yz", y0 - Y_POST0, y1 - Y_POST0))
        k.th("clamp-sleeve", "fittings")
    for i, f in enumerate(FINGERS):
        fid, cid = f["id"], f["clamp"]
        ym = f["y"] + FINGER_H / 2
        zf = 5 + i
        if s <= 100:
            rid = "ring-" + fid[-1]
            ro = R_RING if fine else R_COLLAR_100
            y0, y1 = ym - H_RING / 2, ym + H_RING / 2
            k.three(rid, "post", "body", zf - 0.4, 0.42, poly([luv("post", x, y) for x, y in ((-ro, y0), (ro, y0), (ro, y1), (-ro, y1))]), rect(-ro, y0, ro, y1), ellipse(0, 0, ro, ro), "fittings", ALU)
            k.body.append(rev(rid, "post", ALU, "body", 0.42, annulus_profile(y0 - Y_POST0, y1 - Y_POST0, R_POST, ro), [1, 0, 0], [0, 0, 0]))
            k.th(rid, "fittings")
            k.st(rid, "post", "joint")
        if fine:
            eid = "ears-" + fid[-1]
            eu0, eu1 = FINGER_H / 2 - H_RING / 2, FINGER_H / 2 + H_RING / 2
            k.swept3(eid, cid, zf - 0.3, 0.36, [(eu0, EAR["r"][0]), (eu1, EAR["r"][0]), (eu1, EAR["r"][1]), (eu0, EAR["r"][1])], EAR["t"] / 2, "fittings", ALU)
            k.body.append(box(eid, cid, ALU, "body", 0.36, [eu1 - eu0, EAR["r"][1] - EAR["r"][0], EAR["t"]], [(eu0 + eu1) / 2, sum(EAR["r"]) / 2, 0]))
            k.th(eid, "fittings")
            k.st(eid, cid, "joint")
        hu0, hu1 = FINGER_H / 2 - HOLDER_H / 2, FINGER_H / 2 + HOLDER_H / 2
        if s <= 200:
            hid = "holder-" + fid[-1]
            hr0 = HOLDER_R[0] if fine else R_POST - 0.001
            k.swept3(hid, cid, zf - 0.2, 0.4, [(hu0, -HOLDER_R[1]), (hu1, -HOLDER_R[1]), (hu1, -hr0), (hu0, -hr0)], HOLDER_T / 2, "fittings", ALU)
            k.body.append(box(hid, cid, ALU, "body", 0.4, [hu1 - hu0, HOLDER_R[1] - hr0, HOLDER_T], [(hu0 + hu1) / 2, -(HOLDER_R[1] + hr0) / 2, 0]))
            k.th(hid, "fittings")
            k.st(hid, cid, "member")
        t = finger_t(s)
        prof = arrow(s, f)
        k.swept3(fid, fid, zf, 0.82, prof, t / 2, f["region"], f["material"])
        k.body.append(ext(fid, fid, f["material"], "body", 0.82, prof, "xy", -t / 2, t / 2))
        k.th(fid, f["region"])
        k.st(fid, fid, "member", **({"members": [cid]} if s == 500 else {}))
    return k


def traces(k):
    """🩹 Traces of use and weather as detail and texture strokes: stickers and their residue on the post, the felt-tip and spray tags on finger 2, the legends, strimmer scars and rust runs at the post foot, moss on the shaded footing head and cracks in it."""
    s = k.s
    if s >= 500:
        return
    pz, pr = ("post-zinc", ZINC), ("post-rust", RUST)
    pu = lambda y: y - Y_POST0
    if s <= 100:
        for f in FINGERS:
            fid = f["id"]
            k.elev.append(shape(fid + "-legend", fid, "detail" if s == 200 else "texture", 5 + FINGERS.index(f) + 0.1, False, 0.3, path(f"M 0.17 0.095 L 0.5 0.095 M 0.17 0.055 L 0.44 0.055"), f["region"], f["material"]))
            if s == 50:
                k.elev.append(shape(fid + "-pictogram", fid, "texture", 5 + FINGERS.index(f) + 0.1, False, 0.3, path("M 0.03 0.035 L 0.11 0.035 L 0.11 0.115 L 0.03 0.115 Z"), f["region"], f["material"]))
    if s == 200:
        k.elev.append(shape("graffiti-tag", "finger-2", "detail", 6.2, False, 0.2, path("M 0.12 0.03 C 0.2 0.13 0.27 0.0 0.35 0.11 C 0.41 0.02 0.47 0.12 0.53 0.04"), "finger-2-graffiti", "sign-foil-sprayed"))
        return
    k.elev.append(shape("graffiti-tag", "finger-2", "texture", 6.2, False, 0.2, path("M 0.12 0.03 C 0.2 0.13 0.27 0.0 0.35 0.11 C 0.41 0.02 0.47 0.12 0.53 0.04 M 0.18 0.12 L 0.24 0.02 M 0.42 0.13 C 0.45 0.09 0.49 0.1 0.5 0.06"), "finger-2-graffiti", "sign-foil-sprayed"))
    k.elev.append(shape("stickers", "post", "texture", 2.3, False, 0.6, path(f"M {pu(1.42)} -0.034 L {pu(1.5)} -0.034 L {pu(1.5)} 0.03 L {pu(1.42)} 0.03 Z M {pu(1.55)} -0.03 L {pu(1.6)} -0.03 L {pu(1.6)} 0.034 L {pu(1.55)} 0.034 Z M {pu(1.66)} -0.02 L {pu(1.72)} -0.02 L {pu(1.72)} 0.025 L {pu(1.66)} 0.025 Z"), *pz))
    k.front.append(shape("stickers", "post", "texture", 2.3, False, 0.6, path("M -0.033 1.45 L 0.03 1.45 L 0.03 1.53 L -0.033 1.53 Z M -0.025 1.58 L 0.034 1.58 L 0.034 1.62 L -0.025 1.62 Z"), *pz))
    k.elev.append(shape("rust-runs", "post", "texture", 2.3, False, 0.25, path(" ".join(f"M {pu(f['y'] + 0.025)} {v} L {pu(f['y'] - 0.06)} {v}" for f, v in zip(FINGERS, (0.012, -0.018, 0.02, -0.006)))), *pz))
    if s == 50:
        k.elev.append(shape("sticker-residue", "post", "texture", 2.3, False, 0.6, path(f"M {pu(1.31)} -0.03 L {pu(1.345)} -0.012 L {pu(1.33)} 0.01 M {pu(1.78)} 0.0 L {pu(1.8)} 0.02 M {pu(1.25)} 0.015 L {pu(1.27)} 0.03"), *pz))
        k.elev.append(shape("strimmer-scars", "post", "texture", 2.3, False, 0.3, path(f"M {pu(0.03)} -0.03 L {pu(0.035)} 0.02 M {pu(0.06)} -0.025 L {pu(0.07)} 0.03 M {pu(0.095)} -0.02 L {pu(0.1)} 0.01"), *pr))
        k.front.append(shape("strimmer-scars", "post", "texture", 2.3, False, 0.3, path("M -0.03 0.04 L 0.025 0.045 M -0.025 0.075 L 0.03 0.07"), *pr))
        k.front.append(shape("sticker-residue", "post", "texture", 2.3, False, 0.6, path("M -0.03 1.33 L -0.01 1.35 L 0.012 1.34 M 0.0 1.78 L 0.02 1.8"), *pz))
        k.elev.append(shape("felt-tip-tag", "finger-1", "texture", 5.2, False, 0.2, path("M 0.46 0.02 C 0.49 0.05 0.51 0.01 0.54 0.035"), "finger-1-foil", "sign-foil-weathered"))
        k.elev.append(shape("foil-crazing", "finger-4", "texture", 8.2, False, 0.4, path("M 0.5 0.14 L 0.53 0.12 L 0.56 0.135 M 0.36 0.01 L 0.4 0.025"), "finger-4-foil", "sign-foil-weathered"))
        k.plan.append(shape("footing-cracks", "footing", "texture", 1.1, False, 0.4, path("M -0.19 0.04 L -0.12 0.02 L -0.08 0.05 M -0.06 -0.17 L -0.04 -0.11 M 0.12 0.14 L 0.08 0.1"), "footing-concrete", CONC))
        k.plan.append(shape("moss", "footing", "texture", 1.1, False, 0.4, path("M -0.17 -0.06 C -0.18 -0.02 -0.18 0.02 -0.17 0.06 M -0.15 -0.08 C -0.16 -0.03 -0.16 0.03 -0.15 0.08"), "footing-concrete", CONC))


def masses():
    """⚖️ Bone masses from section x density (kg) and their centres in world rest metres; one source for the rig, the mass markers and the report."""
    rho_s, rho_c, rho_a = 7850.0, 2300.0, 2700.0
    a_post = math.pi * (R_POST ** 2 - (R_POST - T_POST) ** 2)
    out = {}
    v_conc = math.pi * R_FOOT ** 2 * (-Y_FOOT0) - a_post * (-Y_POST0)
    out["footing"] = [(v_conc * rho_c, 0.0, Y_FOOT0 / 2)]
    ring = math.pi * (R_RING ** 2 - R_POST ** 2) * H_RING * 2650 * 0.55
    out["post"] = [(a_post * rho_s * (Y_POST1 - Y_POST0), 0.0, (Y_POST0 + Y_POST1) / 2), (0.05, 0.0, Y_POST1 + 0.006)] + [(ring, 0.0, f["y"] + FINGER_H / 2) for f in FINGERS]
    a_profile = (FINGER_H * FINGER_T - (FINGER_H - 0.004) * (FINGER_T - 0.004)) + 2 * (FINGER_T - 0.004) * 0.0015
    for f in FINGERS:
        t = math.radians(f["tau"])
        holder = (HOLDER_R[1] - HOLDER_R[0]) * HOLDER_H * HOLDER_T * 2650 * 0.4
        ears = (EAR["r"][1] - EAR["r"][0]) * H_RING * EAR["t"] * 2650
        bolts = 2 * 0.045
        rh = (HOLDER_R[0] + HOLDER_R[1]) / 2
        out[f["clamp"]] = [(holder, rh * math.cos(t), f["y"] + FINGER_H / 2), (ears + bolts, -sum(EAR["r"]) / 2 * math.cos(t), f["y"] + FINGER_H / 2)]
        m_prof = a_profile * rho_a * (FINGER_STRAIGHT + (FINGER_L - FINGER_STRAIGHT) / 2)
        m_caps = 0.06
        m_foil = 2 * 0.092 * 0.25
        uc = (FINGER_STRAIGHT / 2 * FINGER_STRAIGHT + (FINGER_STRAIGHT + (FINGER_L - FINGER_STRAIGHT) / 3) * (FINGER_L - FINGER_STRAIGHT) / 2) / (FINGER_STRAIGHT + (FINGER_L - FINGER_STRAIGHT) / 2)
        out[f["id"]] = [(m_prof + m_caps + m_foil, (R_IN + uc) * math.cos(t), f["y"] + FINGER_H / 2)]
    total = {}
    for bone, parts in out.items():
        m = sum(p[0] for p in parts)
        total[bone] = (m, sum(p[0] * p[1] for p in parts) / m, sum(p[0] * p[2] for p in parts) / m)
    return out, total, a_profile, a_post


def com_local(bone, total):
    """🎯 The centre of mass of a bone in its local (u, v): the world centre is taken on the bone's own line of action (x along its direction, y)."""
    m, cx, cy = total[bone]
    o, eu, ev, ew = FR[bone]
    if bone.startswith("finger") or bone.startswith("clamp"):
        f = next(f for f in FINGERS if bone in (f["id"], f["clamp"]))
        t = math.radians(f["tau"])
        rr = cx / math.cos(t)
        p = (rr * math.cos(t), cy, rr * math.sin(t))
    else:
        p = (cx, cy, 0.0)
    u, v, _ = local(bone, p)
    return u, v


REFERENCES = [
    ("en-12899-1", "EN 12899-1:2007 Fixed, vertical road traffic signs, Part 1: Fixed signs (wind load classes WL1-WL9, aerodynamic force coefficient 1.2 of sign faces, point load and temporary deflection classes of sign plates and supports)."),
    ("fgsv-hbr", "FGSV (1998, revised 2023). Hinweise zur wegweisenden Beschilderung fuer den Radverkehr (HBR), and FGSV Empfehlungen fuer Fussgaengerverkehrsanlagen (EFA 2002): finger signs on 76.1 mm posts, clear height 2.25 m over shared foot and cycle paths, fingers stacked on separate pipe clamps."),
    ("en-10219-2", "EN 10219-2:2019 Cold formed welded steel structural hollow sections, Part 2: Tolerances, dimensions and sectional properties (CHS 76.1 x 3.2: A = 7.33 cm2, I = 48.8 cm4, 5.75 kg/m)."),
    ("en-755-9", "EN 755-9:2016 Aluminium and aluminium alloys, extruded rod/bar, tube and profiles, Part 9: Profiles, tolerances on dimensions and form (EN AW-6060 T66 hollow-chamber sign profiles, 2 mm walls)."),
    ("en-1706", "EN 1706:2020 Aluminium and aluminium alloys, Castings: chemical composition and mechanical properties (EN AC-44300 die-cast pipe clamps and finger holders)."),
    ("en-12899-1-sheeting", "EN 12899-1:2007 Table 6 and DIN 67520: retroreflective sheeting class RA1/RA2 on sign faces; ageing loses colour and retroreflection on the sun-facing side."),
    ("en-iso-1461", "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (zinc consumed first at the ground line, where water and de-icing grit stand)."),
    ("en-1991-1-4", "DIN EN 1991-1-4:2010-12 with DIN EN 1991-1-4/NA:2010-12 Eurocode 1: Wind actions (wind zone 2 inland, peak velocity pressure 0.65 kN/m2 up to 10 m)."),
    ("en-1993-1-1", "EN 1993-1-1:2005 Eurocode 3: Design of steel structures, Part 1-1 (E = 210 GPa, S235 fy = 235 MPa, density 7850 kg/m3)."),
    ("din-en-206", "DIN EN 206:2021-06 with DIN 1045-2:2023 Concrete (C20/25 for unreinforced point footings of street furniture, exposure XC2/XF1)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products, hygrothermal properties: design values of conductivity, specific heat and density of steel, aluminium and concrete."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of galvanised, rusted and painted metals, concrete)."),
    ("duffie-beckman-2013", "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (solar absorptance of new and weathered galvanised steel, white and dark paints, concrete)."),
    ("sign-catalogue", "Catalogue-typical data of German municipal signage suppliers, 2000-2024, for pedestrian and park finger signposts: galvanised steel post 76.1 mm with domed cap set 0.7 m into a C20/25 point footing, one die-cast aluminium pipe clamp with fork holder per finger, finger signs 650 x 150 x 25 mm in extruded aluminium hollow-chamber profile with white retroreflective sheeting and printed legend, about 1.3 kg each; a product type, no brand."),
]


def materials():
    """🧱 The materials with weathered surface data."""
    return [
        {"id": ZINC, "title": tx("Hot-dip galvanised steel, weathered", "Feuerverzinkter Stahl, bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.78, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "duffie-beckman-2013"},
        {"id": RUST, "title": tx("Steel tube with the zinc consumed at the ground line, rusted", "Stahlrohr mit an der Geländelinie aufgezehrter Verzinkung, verrostet"), "emissivity": 0.8, "solarAbsorptivity": 0.86, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 2.1e11, "strengthPa": 2.0e8, "source": "literature", "reference": "incropera-2018"},
        {"id": CONC, "title": tx("Concrete C20/25, weathered footing head", "Beton C20/25, bewitterter Fundamentkopf"), "emissivity": 0.92, "solarAbsorptivity": 0.68, "densityKgM3": 2300, "conductivityWmK": 1.65, "specificHeatJKgK": 1000, "elasticModulusPa": 3.0e10, "strengthPa": 2.0e7, "source": "literature", "reference": "iso-10456-2007"},
        {"id": ALU, "title": tx("Die-cast aluminium EN AC-44300, anthracite powder coat chalked by the sun", "Aluminiumdruckguss EN AC-44300, anthrazit pulverbeschichtet, von der Sonne gekreidet"), "emissivity": 0.88, "solarAbsorptivity": 0.82, "densityKgM3": 2650, "conductivityWmK": 130, "specificHeatJKgK": 900, "elasticModulusPa": 7.1e10, "strengthPa": 1.5e8, "source": "literature", "reference": "incropera-2018"},
        {"id": "sign-foil-weathered", "title": tx("Aluminium sign profile with white retroreflective sheeting, faded and crazed", "Aluminium-Schildprofil mit weißer retroreflektierender Folie, ausgeblichen und gerissen"), "emissivity": 0.88, "solarAbsorptivity": 0.42, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "elasticModulusPa": 7.0e10, "strengthPa": 1.6e8, "source": "authored-estimate", "reference": "duffie-beckman-2013"},
        {"id": "sign-foil-sprayed", "title": tx("Aluminium sign profile, sheeting sprayed over with a black and silver graffiti tag", "Aluminium-Schildprofil, Folie mit schwarz-silbernem Graffiti-Tag übersprüht"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "elasticModulusPa": 7.0e10, "strengthPa": 1.6e8, "source": "authored-estimate", "reference": "duffie-beckman-2013"},
    ]


def rig(total, a_profile, a_post):
    """🦴 The mechanical rig: footing, post, one twisted pipe-clamp bone per finger (the loose lowest one revolves about the post) and the four fingers (the graffitied one slides out of its fork holder)."""
    def mass(bone, distribution="point", ref="sign-catalogue"):
        m = total[bone][0]
        u, v = com_local(bone, total)
        return {"kg": round(m, 3), "comX": u, "comY": v, "distribution": distribution, "source": "literature" if bone in ("footing", "post") else "authored-estimate", "reference": ref}
    sec = lambda a, i, e, mat, en, de: {"areaM2": a, "secondMomentM4": i, "extremeFibreM": e, "material": mat, "title": tx(en, de)}
    i_post = math.pi / 4 * (R_POST ** 4 - (R_POST - T_POST) ** 4)
    i_prof = (FINGER_T * FINGER_H ** 3 - (FINGER_T - 0.004) * (FINGER_H - 0.004) ** 3) / 12
    bones = [
        {"id": "footing", "parent": None, "title": tx("Concrete point footing, from its head at ground level down to its base", "Betonpunktfundament, vom Kopf in Geländehöhe hinab zur Sohle"), "x": 0, "y": 0, "angleDeg": -90, "lengthM": -Y_FOOT0, "joint": {"type": "fixed"}, "mass": mass("footing", ref="din-en-206"), "section": sec(r5(math.pi * R_FOOT ** 2), r5(math.pi * R_FOOT ** 4 / 4), R_FOOT, CONC, "Concrete C20/25, 400 mm diameter", "Beton C20/25, Durchmesser 400 mm")},
        {"id": "post", "parent": "footing", "title": tx("Galvanised signpost tube, from its embedded foot up to the domed cap", "Verzinktes Pfostenrohr, vom eingespannten Fuß bis zur Kuppelkappe"), "x": -Y_POST0, "y": 0, "angleDeg": 180, "lengthM": r5(Y_POST1 - Y_POST0), "joint": {"type": "fixed"}, "mass": mass("post", "uniform", "en-10219-2"), "section": sec(r5(a_post), float(f"{i_post:.4g}"), R_POST, ZINC, "CHS 76.1 x 3.2 (EN 10219-2)", "KHP 76,1 x 3,2 (EN 10219-2)")},
    ]
    for f in FINGERS:
        loose = f["id"] == "finger-1"
        swap = f["id"] == "finger-2"
        n = f["id"][-1]
        bones.append({"id": f["clamp"], "parent": "post", "title": tx(f"Pipe clamp {n} with its fork holder, turned on the post to the direction of finger {n}" + (" (loose: it turns about the post)" if loose else ""), f"Rohrschelle {n} mit Gabelhalter, am Pfosten in die Richtung von Finger {n} gedreht" + (" (locker: sie dreht sich um den Pfosten)" if loose else "")), "x": r5(f["y"] - Y_POST0), "y": 0, "angleDeg": 0, "twistDeg": f["tau"], "lengthM": FINGER_H, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -40, "twistMaxDeg": 5} if loose else {"type": "fixed"}, "mass": mass(f["clamp"], ref="en-1706")})
        bones.append({"id": f["id"], "parent": f["clamp"], "title": tx(f"Finger sign {n}, from its inner end in the holder to the arrow point" + (" (slides out of the holder for swapping)" if swap else ""), f"Fingerschild {n}, vom inneren Ende im Halter bis zur Pfeilspitze" + (" (gleitet zum Tausch aus dem Halter)" if swap else "")), "x": 0, "y": -R_IN, "angleDeg": -90, "lengthM": FINGER_L, "joint": {"type": "prismatic", "minM": 0, "maxM": 0.1, "axisDeg": -90} if swap else {"type": "fixed"}, "mass": mass(f["id"], ref="en-755-9"), "section": sec(float(f"{a_profile:.4g}"), float(f"{i_prof:.4g}"), FINGER_H / 2, f["material"], "Aluminium hollow-chamber profile 150 x 25 x 2 mm (EN 755-9)", "Aluminium-Hohlkammerprofil 150 x 25 x 2 mm (EN 755-9)")})
    return {"kind": "mechanical", "root": "footing", "bones": bones}


def anchors():
    """📍 The support at the footing base, the depot workers' hands under finger 2 and the load points of the poses."""
    out = [
        {"id": "footing-base", "bone": "footing", "x": -Y_FOOT0, "y": 0, "kind": "support", "title": tx("Footing base held by the soil (fixed)", "Fundamentsohle im Boden gehalten (eingespannt)"), "support": "fixed", "footprintLengthM": 0.4, "footprintWidthM": 0.4},
        {"id": "hand-inner", "bone": "finger-2", "x": 0.2, "y": 0, "kind": "load", "title": tx("Depot worker's left hand under finger 2, 107 mm inside its centre of mass", "Linke Hand des Bauhofmitarbeiters unter Finger 2, 107 mm innerhalb seines Schwerpunkts")},
        {"id": "hand-outer", "bone": "finger-2", "x": 0.413, "y": 0, "kind": "load", "title": tx("Depot worker's right hand under finger 2, 106 mm outside its centre of mass", "Rechte Hand des Bauhofmitarbeiters unter Finger 2, 106 mm außerhalb seines Schwerpunkts")},
        {"id": "wind-post", "bone": "post", "x": r5(1.1 - Y_POST0), "y": 0, "kind": "load", "title": tx("Centre of the wind area of the post below the fingers", "Mitte der Windfläche des Pfostens unter den Fingern"), },
        {"id": "lean-post", "bone": "post", "x": r5(0.85 - Y_POST0), "y": 0, "kind": "load", "title": tx("Where a leaning bicycle's handlebar touches the post", "Wo der Lenker eines angelehnten Fahrrads den Pfosten berührt")},
        {"id": "tip-finger-1", "bone": "finger-1", "x": FINGER_STRAIGHT, "y": FINGER_H / 2, "kind": "load", "title": tx("Hand at the tip of the loose finger 1", "Hand an der Spitze des lockeren Fingers 1")},
    ]
    for f in FINGERS:
        out.append({"id": "wind-" + f["id"], "bone": f["id"], "x": 0.31, "y": FINGER_H / 2, "kind": "load", "title": tx(f"Centre of the wind area of {f['id'].replace('-', ' ')}", f"Mitte der Windfläche von Finger {f['id'][-1]}")})
    return out


def area_estimates():
    """📐 Total outer areas of the regions (m2), hand-summed from the 1:50 parts and checked against the flood measure of the validator."""
    side = lambda r, h: 2 * math.pi * r * h
    disc = lambda r: math.pi * r * r
    face = FINGER_STRAIGHT * FINGER_H + 0.5 * (FINGER_L - FINGER_STRAIGHT) * FINGER_H
    perim = FINGER_H + 2 * FINGER_STRAIGHT + 2 * math.hypot(FINGER_L - FINGER_STRAIGHT, FINGER_H / 2)
    in_holder = HOLDER_R[1] - R_IN
    finger = 2 * face + perim * FINGER_T - 2 * in_holder * HOLDER_H - 2 * in_holder * FINGER_T * 0 - HOLDER_H * FINGER_T
    ring = side(R_RING, H_RING) + side(R_POST, H_RING) + 2 * (disc(R_RING) - disc(R_POST))
    holder = 2 * (HOLDER_R[1] - HOLDER_R[0]) * (HOLDER_H + HOLDER_T) + HOLDER_H * HOLDER_T - FINGER_H * FINGER_T * 0
    ears = 2 * (EAR["r"][1] - EAR["r"][0]) * (H_RING + EAR["t"]) + H_RING * EAR["t"]
    out = {
        "footing-concrete": side(R_FOOT, -Y_FOOT0) + disc(R_FOOT) + disc(R_FOOT) - disc(R_POST),
        "post-rust": side(R_POST, Y_RUST - Y_POST0),
        "post-zinc": side(R_POST, Y_POST1 - Y_RUST) + disc(R_CAP) * 1.25 + side(R_CAP, 0.004),
        "fittings": 4 * (ring + holder + ears),
    }
    for f in FINGERS:
        out[f["region"]] = finger
    return out


def regions(area):
    """🔥 The thermal regions: passive weathered surfaces in the park sun, no rated power anywhere."""
    lay = lambda lid, en, de, kind, mat, t, lam, rho, cp, src, ref: {"id": lid, "title": tx(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": r5(t / lam) if t / lam >= 1e-5 else float(f"{t / lam:.3g}"), "heatCapacityJM2K": round(rho * cp * t), "source": src, "reference": ref}
    def reg(rid, en, de, mat, geom, lc, svf, layer, src="literature", ref="duffie-beckman-2013"):
        return {"id": rid, "title": tx(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": round(area[rid], 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"], "source": src, "reference": ref}
    sheet = lambda mat, src: lay("profile-wall", "Aluminium profile wall 2 mm", "Aluminium-Profilwand 2 mm", "metal", mat, 0.002, 200, 2700, 900, src, "en-755-9")
    out = [
        reg("post-zinc", "Galvanised post with domed cap, stickers and their residue at eye height", "Verzinkter Pfosten mit Kuppelkappe, Aufkleber und ihre Reste auf Augenhöhe", ZINC, "cylinder", 0.076, 0.5, lay("tube-wall", "Tube wall 3.2 mm", "Rohrwand 3,2 mm", "metal", ZINC, T_POST, 50, 7850, 470, "literature", "en-10219-2")),
        reg("post-rust", "Post foot, zinc consumed at the ground line and rusted, strimmer-scarred", "Pfostenfuß, Verzinkung an der Geländelinie aufgezehrt und verrostet, von der Motorsense zerkratzt", RUST, "cylinder", 0.076, 0.5, lay("rusted-wall", "Rusted tube wall 3.2 mm", "Verrostete Rohrwand 3,2 mm", "metal", RUST, T_POST, 50, 7850, 470, "literature", "incropera-2018"), ref="incropera-2018"),
        reg("fittings", "Die-cast pipe clamps and fork holders, chalked anthracite powder coat", "Druckguss-Rohrschellen und Gabelhalter, gekreidete anthrazitfarbene Pulverbeschichtung", ALU, "cylinder", 0.1, 0.5, lay("casting-wall", "Casting wall 6 mm", "Gusswand 6 mm", "metal", ALU, 0.006, 130, 2650, 900, "literature", "en-1706"), ref="incropera-2018"),
        reg("footing-concrete", "Concrete point footing, head flush with the water-bound path surface", "Betonpunktfundament, Kopf bündig mit der wassergebundenen Wegedecke", CONC, "horizontal-plate-up", 0.4, 0.9, lay("footing-core", "Concrete to the footing axis 200 mm", "Beton bis zur Fundamentachse 200 mm", "masonry", CONC, 0.2, 1.65, 2300, 1000, "literature", "iso-10456-2007"), ref="iso-10456-2007"),
    ]
    names = {
        "finger-1": ("Finger 1, loose and turned 35 degrees off its destination, faded sheeting with a felt-tip tag", "Finger 1, locker und 35 Grad von seinem Ziel weggedreht, ausgeblichene Folie mit Filzstift-Tag"),
        "finger-2": ("Finger 2, sheeting sprayed over with a graffiti tag, due for swapping", "Finger 2, Folie mit Graffiti-Tag übersprüht, zum Tausch fällig"),
        "finger-3": ("Finger 3, faded white sheeting with its legend", "Finger 3, ausgeblichene weiße Folie mit Beschriftung"),
        "finger-4": ("Finger 4, top finger, crazed sheeting with bird droppings on the upper edge", "Finger 4, oberster Finger, gerissene Folie mit Vogelkot an der Oberkante"),
    }
    for f in FINGERS:
        en, de = names[f["id"]]
        out.append(reg(f["region"], en, de, f["material"], "vertical-plate", FINGER_H, 0.5, sheet(f["material"], "authored-estimate"), src="authored-estimate"))
    return out


SUN = {
    "in-service": {"post-zinc": 0.45, "post-rust": 0.4, "fittings": 0.45, "footing-concrete": 0.1, "finger-1-foil": 0.5, "finger-2-graffiti": 0.5, "finger-3-foil": 0.45, "finger-4-foil": 0.5},
    "maintenance": {"post-zinc": 0.45, "post-rust": 0.4, "fittings": 0.45, "footing-concrete": 0.1, "finger-1-foil": 0.5, "finger-2-graffiti": 0.4, "finger-3-foil": 0.45, "finger-4-foil": 0.5},
    "in-use": {"post-zinc": 0.4, "post-rust": 0.3, "fittings": 0.45, "footing-concrete": 0.1, "finger-1-foil": 0.45, "finger-2-graffiti": 0.5, "finger-3-foil": 0.45, "finger-4-foil": 0.5},
}


def binding(state):
    """🌞 Region bindings of a pose: summer noon in the park; each vertical finger has one face to the sun, the footing is in the soil but for its head, the post foot stands in the trodden gravel."""
    out = {}
    for rid, f in SUN[state].items():
        b = {"exposedFraction": 1, "sunlitFraction": f}
        if rid == "footing-concrete":
            b = {"exposedFraction": 0.1, "sunlitFraction": 0.1, "contact": {"fraction": 0.9, "tempC": 14}}
        if rid == "post-rust":
            b = {"exposedFraction": 0.15, "sunlitFraction": f * 0.15, "contact": {"fraction": 0.85, "tempC": 14}}
        out[rid] = b
    return {"regions": out, "emitters": {}}


def wind_loads():
    """🌬️ Design gust from ahead (towards -x) on the post and on each finger, normal to its face: q_p 0.65 kN/m2 (DIN EN 1991-1-4, zone 2 inland) times c_f 1.2 of a sign face (EN 12899-1) times its area times the share the wind sees."""
    q, cf = 650.0, 1.2
    a = FINGER_STRAIGHT * FINGER_H + 0.5 * (FINGER_L - FINGER_STRAIGHT) * FINGER_H
    out = [{"id": "wind-on-post", "kind": "wind", "title": tx("Design gust on the post (0.65 kN/m2 x 1.0 x 0.17 m2)", "Bemessungsböe auf den Pfosten (0,65 kN/m² x 1,0 x 0,17 m²)"), "anchor": "wind-post", "fxN": -round(q * 1.0 * 2 * R_POST * 2.25), "fyN": 0, "source": "literature", "reference": "en-1991-1-4"}]
    for f in FINGERS:
        t = math.radians(f["tau"])
        k = q * cf * a * math.sin(t)
        fx, fz = k * -math.sin(t), k * math.cos(t)
        out.append({"id": "wind-on-" + f["id"], "kind": "wind", "title": tx(f"Design gust on {f['id'].replace('-', ' ')}, normal to its face ({abs(k):.0f} N)", f"Bemessungsböe auf Finger {f['id'][-1]}, normal zur Fläche ({abs(k):.0f} N)"), "anchor": "wind-" + f["id"], "fxN": round(fx, 1), "fyN": 0, "fzN": round(fz, 1), "source": "literature", "reference": "en-12899-1"})
    return out


def poses():
    """🧍 In service in a gust, finger 2 pulled out of its holder by the depot, and a visitor turning the loose finger 1 back while a bicycle leans on the post."""
    base = {"ground": "authored", "contacts": ["footing-base"]}
    half = round(round(masses()[1]["finger-2"][0], 3) * 9.80665 / 2, 1)
    hands = [{"id": f"carry-{side}", "kind": "equipment", "title": tx(f"The worker's {en} hand carrying half of finger 2", f"Die {de} Hand des Mitarbeiters trägt die Hälfte von Finger 2"), "anchor": f"hand-{side}", "fxN": 0, "fyN": half, "source": "authored-estimate"} for side, en, de in (("inner", "left", "linke"), ("outer", "right", "rechte"))]
    t1 = math.radians(FINGERS[0]["tau"] - 35)
    turn = [
        {"id": "finger-turn", "kind": "imposed", "title": tx("A tall visitor's hand turning the loose finger 1 back to its destination (0.10 kN at the tip)", "Die Hand eines großen Besuchers dreht den lockeren Finger 1 auf sein Ziel zurück (0,10 kN an der Spitze)"), "anchor": "tip-finger-1", "fxN": round(100 * math.sin(t1), 1), "fyN": 0, "fzN": round(-100 * math.cos(t1), 1), "source": "authored-estimate"},
        {"id": "bicycle-lean", "kind": "imposed", "title": tx("A bicycle leaning with its handlebar on the post (40 N)", "Ein Fahrrad lehnt mit dem Lenker am Pfosten (40 N)"), "anchor": "lean-post", "fxN": -40, "fyN": 0, "source": "authored-estimate"},
    ]
    return [
        {"id": "in-service", "title": tx("In service", "Im Einsatz"), "description": tx("Standing at the fork of two park paths in its concrete point footing, four fingers on their pipe clamps, the lowest one loose and turned 35 degrees off its destination, a design gust from ahead.", "An der Gabelung zweier Parkwege im Betonpunktfundament, vier Finger an ihren Rohrschellen, der unterste locker und 35 Grad von seinem Ziel weggedreht, eine Bemessungsböe von vorn."), "bones": {}, **base, "thermal": binding("in-service"), "loads": wind_loads()},
        {"id": "maintenance", "title": tx("Graffitied finger pulled out for swapping", "Übersprühter Finger zum Tausch herausgezogen"), "description": tx("A depot worker on a platform ladder has loosened the two bolts of the fork holder and slides the graffitied finger 2 out along its axis, carrying it on both hands, before the new finger goes in; the post and the other fingers stay.", "Ein Bauhofmitarbeiter auf einer Podestleiter hat die zwei Schrauben des Gabelhalters gelöst und zieht den übersprühten Finger 2 entlang seiner Achse heraus, auf beiden Händen getragen, bevor der neue Finger eingesetzt wird; Pfosten und übrige Finger bleiben."), "bones": {"finger-2": {"dy": -SLIDE}}, "ground": "authored", "contacts": ["footing-base"], "thermal": binding("maintenance"), "loads": hands},
        {"id": "hands-under", "key": True, "title": tx("Hands under the finger", "Hände unter dem Finger"), "description": tx("Key of the swap clip: both hands carry finger 2 while it still sits in its holder.", "Schlüssel des Tausch-Clips: beide Hände tragen Finger 2, solange er noch im Halter steckt."), "bones": {}, **base, "thermal": binding("in-service"), "loads": hands},
        {"id": "in-use", "title": tx("Read and turned by visitors", "Von Besuchern gelesen und gedreht"), "description": tx("A cyclist has leaned the bicycle on the post to read the fingers, and a tall visitor reaches up and turns the loose lowest finger back to its true destination, 35 degrees round the post.", "Eine Radfahrerin hat das Fahrrad an den Pfosten gelehnt, um die Finger zu lesen, und ein großer Besucher greift hinauf und dreht den lockeren untersten Finger um 35 Grad um den Pfosten auf sein wahres Ziel zurück."), "bones": {"clamp-1": {"twistDeg": -35}}, **base, "thermal": binding("in-use"), "loads": turn},
    ]


def clips():
    """🎞 The depot's sign swap and the loose finger turned back and creeping round again."""
    return [
        {"id": "maintenance", "title": tx("Swapping the graffitied finger", "Tausch des übersprühten Fingers"), "description": tx("The worker loosens the holder bolts, slides finger 2 out along its axis on both hands, holds it while the new finger is handed up, and slides the finger home and tightens the bolts.", "Der Mitarbeiter löst die Halterschrauben, zieht Finger 2 auf beiden Händen entlang seiner Achse heraus, hält ihn, während der neue Finger hinaufgereicht wird, schiebt den Finger ein und zieht die Schrauben an."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 0.4, "pose": "hands-under", "ease": "smooth"}, {"t": 2, "pose": "maintenance", "ease": "smooth"}, {"t": 4, "pose": "maintenance", "ease": "smooth"}, {"t": 5.6, "pose": "hands-under", "ease": "smooth"}, {"t": 6, "pose": "in-service", "ease": "smooth"}],
         "contacts": [{"anchor": "footing-base", "fromS": 0, "toS": 6}],
         "phases": [{"id": "pull-out", "fromS": 0, "toS": 2, "title": tx("Pulling out", "Herausziehen")}, {"id": "hold", "fromS": 2, "toS": 4, "title": tx("Holding while the new finger is handed up", "Halten, während der neue Finger hinaufgereicht wird")}, {"id": "slide-home", "fromS": 4, "toS": 6, "title": tx("Sliding home and tightening", "Einschieben und festziehen")}],
         "events": [{"t": 0.2, "kind": "motion", "title": tx("Holder bolts loosened", "Halterschrauben gelöst")}, {"t": 0.4, "kind": "load-on", "title": tx("Both hands under the finger", "Beide Hände unter dem Finger"), "target": "carry-inner"}, {"t": 5.6, "kind": "load-off", "title": tx("Hands off, bolts tightened", "Hände weg, Schrauben angezogen"), "target": "carry-inner"}],
         "tracks": []},
        {"id": "rotating", "title": tx("Turning the loose finger", "Drehen des lockeren Fingers"), "description": tx("A visitor reaches up and turns the loose lowest finger 35 degrees round the post to its true destination; over the next gusts the slack clamp lets it creep back to where it stood.", "Ein Besucher greift hinauf und dreht den lockeren untersten Finger um 35 Grad um den Pfosten auf sein wahres Ziel; in den nächsten Böen lässt die lockere Schelle ihn dorthin zurückkriechen, wo er stand."), "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.6, "pose": "in-use", "ease": "smooth"}, {"t": 2.6, "pose": "in-use", "ease": "smooth"}, {"t": 5, "pose": "in-service", "ease": "smooth"}],
         "contacts": [{"anchor": "footing-base", "fromS": 0, "toS": 5}],
         "phases": [{"id": "turn", "fromS": 0, "toS": 1.6, "title": tx("Turning back", "Zurückdrehen")}, {"id": "read", "fromS": 1.6, "toS": 2.6, "title": tx("Pointing true", "Zeigt richtig")}, {"id": "creep", "fromS": 2.6, "toS": 5, "title": tx("Creeping back in the gusts", "Kriecht in den Böen zurück")}],
         "events": [{"t": 0.1, "kind": "load-on", "title": tx("Hand on the finger tip", "Hand an der Fingerspitze"), "target": "finger-turn"}, {"t": 1.8, "kind": "load-off", "title": tx("Hand off", "Hand weg"), "target": "finger-turn"}],
         "tracks": [{"target": "load:finger-turn", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.3, "v": 1}, {"t": 1.6, "v": 1}, {"t": 1.9, "v": 0}, {"t": 5, "v": 0}]}, {"target": "load:bicycle-lean", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 5, "v": 1}]}]},
    ]


PENS = {
    50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7},
    100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def rep_decisions(s):
    """📝 The eight authored decisions of the 2D representation at 1:s."""
    return {
        50: dict(
            silhouette=tx("Side elevation 75.5 mm high on paper: the 0.40 m footing below ground, the 76.1 mm post with its rusted foot and domed cap, four die-cast pipe clamps with their bolted ears and fork holders, and four 650 x 150 mm arrow fingers pointing to four destinations, each drawn in its own plane and foreshortened by its direction, the lowest turned 35 degrees off. Front and plan are the same parts seen from ahead and from above, every finger the true outline of its 25 mm profile in that view.", "Seitenansicht 75,5 mm hoch auf dem Papier: das 0,40-m-Fundament unter Gelände, der 76,1-mm-Pfosten mit verrostetem Fuß und Kuppelkappe, vier Druckguss-Rohrschellen mit verschraubten Ohren und Gabelhaltern und vier Pfeilfinger 650 x 150 mm zu vier Zielen, jeder in seiner eigenen Ebene gezeichnet und nach seiner Richtung verkürzt, der unterste um 35 Grad verdreht. Vorder- und Draufsicht zeigen dieselben Teile von vorn und von oben, jeder Finger als wahre Kontur seines 25-mm-Profils in dieser Ansicht."),
            retained=tx("Every part of the object; the legends and pictograms of the fingers, the stickers and their residue, the graffiti and felt-tip tags, rust runs, strimmer scars, moss and cracks as strokes.", "Jedes Teil des Gegenstands; Beschriftungen und Piktogramme der Finger, Aufkleber und ihre Reste, Graffiti- und Filzstift-Tag, Rostläufer, Kratzer der Motorsense, Moos und Risse als Striche."),
            merged=tx("Nothing merged; the embedded post lies behind the footing and is not drawn separately.", "Nichts verschmolzen; der eingespannte Pfosten liegt hinter dem Fundament und wird nicht eigens gezeichnet."),
            omitted=tx("The end caps of the profiles (2 mm), the T-slot of the holder and the M10 bolts of ears and holders (their heads stand 7 mm proud and would be drawn wrong on the loose clamp as it turns).", "Die Endkappen der Profile (2 mm), die T-Nut des Halters und die M10-Schrauben von Ohren und Haltern (ihre Köpfe stehen 7 mm vor und würden an der lockeren, sich drehenden Schelle falsch gezeichnet)."),
            thinElements=tx("The 2 mm profile walls and the sheeting are the filled outline of each finger; legends, pictograms, stickers, tags, rust runs and strimmer scars are 0.13 mm texture strokes.", "Die 2-mm-Profilwände und die Folie sind die gefüllte Kontur jedes Fingers; Beschriftungen, Piktogramme, Aufkleber, Tags, Rostläufer und Kratzer der Motorsense sind 0,13-mm-Texturstriche."),
            hatching=tx("Concrete stipple (dots at 0.8 mm) on the footing in all three views.", "Betonpunktierung (Punkte im Abstand 0,8 mm) auf dem Fundament in allen drei Ansichten."),
            styles=tx("Fingers 0.82, post 0.48, rusted foot 0.3, clamps and holders 0.36-0.42, footing 0.62; body pen 0.5, detail 0.25, texture 0.13.", "Finger 0,82, Pfosten 0,48, rostiger Fuß 0,3, Schellen und Halter 0,36-0,42, Fundament 0,62; Körperstift 0,5, Detail 0,25, Textur 0,13."),
            overlays=tx("post-zinc: post and cap; post-rust: the foot to 0.12 m; fittings: rings, ears and holders; finger-1-foil, finger-2-graffiti, finger-3-foil, finger-4-foil: one per finger; footing-concrete.", "post-zinc: Pfosten und Kappe; post-rust: der Fuß bis 0,12 m; fittings: Ringe, Ohren und Halter; finger-1-foil, finger-2-graffiti, finger-3-foil, finger-4-foil: je Finger; footing-concrete."),
        ),
        100: dict(
            silhouette=tx("Side elevation 37.8 mm high: footing, post with rusted foot, four 15 mm clamp collars, four fork holders reaching the post and the four foreshortened fingers; front and plan likewise.", "Seitenansicht 37,8 mm hoch: Fundament, Pfosten mit rostigem Fuß, vier 15-mm-Schellenkragen, vier Gabelhalter bis zum Pfosten und die vier verkürzten Finger; Vorder- und Draufsicht ebenso."),
            retained=tx("Footing, post, rust band, holders, fingers; one legend line per finger, the stickers, graffiti tag and rust runs as strokes.", "Fundament, Pfosten, Rostband, Halter, Finger; je Finger eine Beschriftungslinie, Aufkleber, Graffiti-Tag und Rostläufer als Striche."),
            merged=tx("The 9 mm clamp rings and the 12 mm ears into one 15 mm collar per clamp; the holders reach the post; the cap into the post.", "Die 9-mm-Schellenringe und die 12-mm-Ohren in je einen 15-mm-Kragen; die Halter reichen bis zum Pfosten; die Kappe in den Pfosten."),
            omitted=tx("Bolts, pictograms, felt-tip tag, strimmer scars, sticker residue, moss and cracks.", "Schrauben, Piktogramme, Filzstift-Tag, Kratzer der Motorsense, Aufkleberreste, Moos und Risse."),
            thinElements=tx("Legends and stickers are 0.13 mm texture strokes.", "Beschriftungen und Aufkleber sind 0,13-mm-Texturstriche."),
            hatching=tx("Concrete stipple on the footing.", "Betonpunktierung auf dem Fundament."),
            styles=tx("Tones as 1:50; body pen 0.35, detail 0.18, texture 0.13.", "Töne wie 1:50; Körperstift 0,35, Detail 0,18, Textur 0,13."),
            overlays=tx("Regions as 1:50; collars and holders carry fittings.", "Bereiche wie 1:50; Kragen und Halter tragen fittings."),
        ),
        200: dict(
            silhouette=tx("Side elevation 18.9 mm high: footing, post with its rust band, the four 33 mm fork holders on the post and the four fingers drawn at 30 mm thickness.", "Seitenansicht 18,9 mm hoch: Fundament, Pfosten mit Rostband, die vier 33-mm-Gabelhalter am Pfosten und die vier Finger mit 30 mm Dicke gezeichnet."),
            retained=tx("The masses that read at 30 mm: footing, post, rust band, holders, fingers; the legends and the graffiti tag as detail lines.", "Die bei 30 mm lesbaren Massen: Fundament, Pfosten, Rostband, Halter, Finger; Beschriftungen und Graffiti-Tag als Detaillinien."),
            merged=tx("The clamp rings and collars into the holders, which reach the post.", "Die Schellenringe und Kragen in die Halter, die bis zum Pfosten reichen."),
            omitted=tx("Every texture stroke.", "Jeder Texturstrich."),
            thinElements=tx("The 25 mm fingers are drawn at the 30 mm feature.", "Die 25-mm-Finger sind mit dem 30-mm-Merkmal gezeichnet."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.25, detail 0.13.", "Töne wie 1:50; Körperstift 0,25, Detail 0,13."),
            overlays=tx("Regions as 1:50; the holders carry fittings.", "Bereiche wie 1:50; die Halter tragen fittings."),
        ),
        500: dict(
            silhouette=tx("Side elevation 7.6 mm high: footing, post, rust band, one 91 mm clamp sleeve and four fingers at the 75 mm print floor.", "Seitenansicht 7,6 mm hoch: Fundament, Pfosten, Rostband, eine 91-mm-Schellenhülse und vier Finger an der 75-mm-Druckgrenze."),
            retained=tx("Footing, post, rust band, clamp sleeve, fingers.", "Fundament, Pfosten, Rostband, Schellenhülse, Finger."),
            merged=tx("The four clamps and holders into one octagonal sleeve 91 mm across over the clamp zone, which the fingers enter; cap into the post; the arrow points of fingers 1 and 3 set back 30 mm so the thickened fingers keep the real lateral reach.", "Die vier Schellen und Halter in eine achteckige Hülse von 91 mm über dem Schellenbereich, in die die Finger laufen; Kappe in den Pfosten; die Pfeilspitzen der Finger 1 und 3 um 30 mm zurückgesetzt, damit die verdickten Finger die wahre seitliche Ausladung behalten."),
            omitted=tx("Every stroke below the 75 mm feature.", "Jeder Strich unter dem 75-mm-Merkmal."),
            thinElements=tx("The fingers (25 mm) would be 0.05 mm on paper: they are drawn and modelled 75 mm thick.", "Die Finger (25 mm) wären auf dem Papier 0,05 mm: sie sind 75 mm dick gezeichnet und modelliert."),
            hatching=tx("None.", "Keine."),
            styles=tx("Tones as 1:50; body pen 0.18.", "Töne wie 1:50; Körperstift 0,18."),
            overlays=tx("Regions as 1:50; the holders carry fittings.", "Bereiche wie 1:50; die Halter tragen fittings."),
        ),
    }[s]


def vol_decisions(s):
    """🧊 The volume decisions at 1:s."""
    return {
        50: [tx("Body 1:50 for 7.5 mm: augered C20/25 footing 0.40 x 0.80 m with the post bore, CHS 76.1 post split at 0.12 m into the rusted foot and the galvanised shaft, domed cap, four 9 mm die-cast clamp rings, their 12 mm bolted ears and 120 mm fork holders on the twisted clamp bones, and four 650 x 150 x 25 mm arrow fingers as extrusions of their true outline, each on its own bone in its own direction.", "Körper 1:50 für 7,5 mm: gebohrtes Fundament C20/25 0,40 x 0,80 m mit Pfostenbohrung, KHP-76,1-Pfosten bei 0,12 m in den rostigen Fuß und den verzinkten Schaft geteilt, Kuppelkappe, vier 9-mm-Druckguss-Schellenringe, ihre 12-mm-Schraubohren und 120-mm-Gabelhalter an den gedrehten Schellenknochen, und vier Pfeilfinger 650 x 150 x 25 mm als Extrusionen ihrer wahren Kontur, jeder auf seinem Knochen in seiner Richtung."), tx("Thermal 1:50: every body solid by reference in its region; structure 1:50: footing, post, holders and fingers as members, rings and ears as joints, a mass marker per bone and the fixed block under the footing base.", "Thermik 1:50: jeder Körper als Verweis in seinem Bereich; Tragwerk 1:50: Fundament, Pfosten, Halter und Finger als Stäbe, Ringe und Ohren als Gelenke, je Knochen eine Massenmarke und der Einspannblock unter der Fundamentsohle.")],
        100: [tx("Body 1:100 for 15 mm: the rings (9 mm) and ears (12 mm) merged into one 15 mm collar per clamp, the fork holders reaching the post, the cap into the post; footing, rust band and the four 25 mm fingers as at 1:50.", "Körper 1:100 für 15 mm: Ringe (9 mm) und Ohren (12 mm) in je einen 15-mm-Kragen verschmolzen, die Gabelhalter bis zum Pfosten, die Kappe im Pfosten; Fundament, Rostband und die vier 25-mm-Finger wie 1:50."), tx("Thermal 1:100 by reference; structure 1:100: members as at 1:50, the holders as the members of the clamp bones, the collars as joints, masses and the footing block.", "Thermik 1:100 als Verweis; Tragwerk 1:100: Stäbe wie 1:50, die Halter als Stäbe der Schellenknochen, die Kragen als Gelenke, Massen und der Fundamentblock.")],
        200: [tx("Body 1:200 for 30 mm: the collars merged into the 33 mm fork holders, which reach the post; the fingers thickened from 25 to 30 mm; footing, rusted foot and post.", "Körper 1:200 für 30 mm: die Kragen in die 33-mm-Gabelhalter verschmolzen, die bis zum Pfosten reichen; die Finger von 25 auf 30 mm verdickt; Fundament, rostiger Fuß und Pfosten."), tx("Thermal 1:200 by reference, only the total area is compared; structure 1:200: footing, post, holders and fingers as members, masses and the footing block.", "Thermik 1:200 als Verweis, verglichen wird nur die Gesamtfläche; Tragwerk 1:200: Fundament, Pfosten, Halter und Finger als Stäbe, Massen und der Fundamentblock.")],
        500: [tx("Body 1:500 for 75 mm: footing, rusted foot, post (76.1 mm, at the print floor), the four clamps and holders as one octagonal sleeve 91 mm across, and the four fingers at 75 mm thickness running into it; the points of fingers 1 and 3 set back 30 mm to keep the real lateral reach.", "Körper 1:500 für 75 mm: Fundament, rostiger Fuß, Pfosten (76,1 mm, an der Druckgrenze), die vier Schellen mit Haltern als eine achteckige Hülse von 91 mm und die vier Finger mit 75 mm Dicke, die in sie laufen; die Spitzen der Finger 1 und 3 um 30 mm zurückgesetzt, um die wahre seitliche Ausladung zu halten."), tx("Thermal 1:500 by reference, total area only; structure 1:500: footing, post and fingers as members, each finger member also spanning its clamp bone (members), masses and the footing block.", "Thermik 1:500 als Verweis, nur die Gesamtfläche; Tragwerk 1:500: Fundament, Pfosten und Finger als Stäbe, jeder Fingerstab spannt auch über seinen Schellenknochen (members), Massen und der Fundamentblock.")],
    }[s]


def structure_extras(k, total):
    """🏗️ Coarse clamp blocks and true-section fingers, the mass markers and the support block of scale k."""
    s, f = k.s, FEATURE[k.s]
    mr = {50: 0.02, 100: 0.025, 200: 0.03, 500: 0.04}[s]
    for bone in total:
        u, v = com_local(bone, total)
        w = min(mr, finger_t(s) / 2) if bone.startswith("finger") else mr
        k.structure.append(ell("mass-" + bone, bone, CONC if bone == "footing" else ZINC, [mr, mr, w], [u, v, 0], member=bone, part="mass"))
    k.structure.append(box("support-footing-base", "footing", CONC, "detail", 0.1, [f, 2 * R_FOOT, 2 * R_FOOT], [-Y_FOOT0 - f / 2, 0, 0], member="footing", part="support", anchor="footing-base"))


def bounds():
    """📦 World bounding box of the 1:50 body from the solid vertices (fingers, holders, ears and the round parts)."""
    xs, ys, zs = [-R_FOOT, R_FOOT], [Y_FOOT0, Y_CAP1], [-R_FOOT, R_FOOT]
    for f in FINGERS:
        for u, v in arrow(50):
            for w in (-FINGER_T / 2, FINGER_T / 2):
                p = to_world(f["id"], u, v, w)
                xs.append(p[0])
                zs.append(p[2])
    return (min(xs), max(xs)), (min(ys), max(ys)), (min(zs), max(zs))


def document():
    """📄 The whole asset document."""
    parts, total, a_profile, a_post = masses()
    area = area_estimates()
    reps, vols = {}, {}
    for s in (50, 100, 200, 500):
        k = build(s)
        traces(k)
        structure_extras(k, total)
        hatches = [{"id": "concrete-stipple", "title": tx("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8 if s == 50 else 0.6, "penMm": 0.13}] if s <= 100 else []
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": BANDS[s][0], "maxDenominator": BANDS[s][1]}, "pens": PENS[s], "hatches": hatches, "geometry": {"elevation": k.elev, "front": k.front, "plan": k.plan}, "decisions": rep_decisions(s)}
        vols[str(s)] = {"status": "authored", "denominator": s, "body": k.body, "thermal": k.thermal, "structure": k.structure, "decisions": vol_decisions(s)}
    mass_total = round(sum(t[0] for t in total.values()), 2)
    bx, by, bz = bounds()
    m = {b: f"{total[b][0]:.2f}" for b in total}
    fingers_kg = sum(total[f['id']][0] for f in FINGERS)
    clamps_kg = sum(total[f['clamp']][0] for f in FINGERS)
    asset = {
        "id": "used-wayfinding-signpost",
        "category": "objects",
        "title": tx("Used Wayfinding Signpost", "Gebrauchter Wegweiser"),
        "summary": tx("A municipal park finger signpost after years at a path fork: a galvanised 76.1 mm steel post 2.98 m high, rusted at the ground line and plastered with stickers, set 0.7 m into a concrete point footing, carries four 650 mm aluminium arrow fingers on die-cast pipe clamps; one finger is sprayed over with a graffiti tag and due for swapping, and the lowest has worked loose and points 35 degrees off its destination.", "Ein kommunaler Parkwegweiser nach Jahren an einer Weggabelung: ein verzinkter 76,1-mm-Stahlpfosten, 2,98 m hoch, an der Geländelinie verrostet und mit Aufklebern beklebt, 0,7 m tief in einem Betonpunktfundament, trägt vier 650-mm-Pfeilfinger aus Aluminium an Druckguss-Rohrschellen; ein Finger ist mit einem Graffiti-Tag übersprüht und zum Tausch fällig, der unterste hat sich gelockert und zeigt 35 Grad an seinem Ziel vorbei."),
        "tags": ["used", "park", "public-space", "signage", "wayfinding", "finger-signpost", "galvanised-steel", "aluminium", "pipe-clamp", "graffiti", "stickers", "concrete-footing", "street-furniture"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": tx(f"Catalogue-typical park finger signpost: post CHS 76.1 x 3.2 to 2.96 m with a domed cap, set 0.70 m into a 0.40 x 0.80 m C20/25 point footing; four fingers 650 x 150 x 25 mm (EN AW-6060 hollow-chamber profile, 2 mm walls) on die-cast clamps at lower edges 2.25, 2.425, 2.60 and 2.775 m (clear height 2.25 m); directions 320 (loose, true 285), 160, 30 and 210 degrees from ahead towards the left. Masses from section x density: footing {m['footing']} kg, post with cap and rings {m['post']} kg, clamps and holders {clamps_kg:.2f} kg, fingers {fingers_kg:.2f} kg. Nothing is measured.", f"Katalogtypischer Parkwegweiser: Pfosten KHP 76,1 x 3,2 bis 2,96 m mit Kuppelkappe, 0,70 m tief in einem Punktfundament C20/25 0,40 x 0,80 m; vier Finger 650 x 150 x 25 mm (Hohlkammerprofil EN AW-6060, 2-mm-Wände) an Druckgussschellen mit Unterkanten 2,25, 2,425, 2,60 und 2,775 m (lichte Höhe 2,25 m); Richtungen 320 (locker, richtig 285), 160, 30 und 210 Grad von vorn nach links. Massen aus Querschnitt x Dichte: Fundament {m['footing']} kg, Pfosten mit Kappe und Ringen {m['post']} kg, Schellen und Halter {clamps_kg:.2f} kg, Finger {fingers_kg:.2f} kg. Nichts ist gemessen.")},
        "dimensions": {"heightM": round(by[1] - by[0], 4), "widthM": round(bx[1] - bx[0], 4), "depthM": round(bz[1] - bz[0], 4), "massKg": mass_total},
        "materials": materials(),
        "rig": rig(total, a_profile, a_post),
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {
            "minimumAirPassageM": 0.025,
            "environment": {"airTempC": 27, "meanRadiantTempC": 31, "skyTempC": 14, "groundTempC": 33, "relativeHumidity": 0.45, "windMS": 1.2, "windDirectionDeg": 180, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
            "passengers": [], "operating": [], "emitters": [],
            "regions": regions(area),
            "assumptions": [
                tx("Passive weathered surfaces at summer noon in a park: absorbed sun, long-wave exchange with sky and ground, convection and the heat capacity of the thin walls; the signpost draws no power (0 W) and carries no rating.", "Passive verwitterte Flächen zur Sommermittagszeit im Park: absorbierte Sonne, Langwellenaustausch mit Himmel und Boden, Konvektion und die Wärmekapazität der dünnen Wände; der Wegweiser nimmt keine Leistung auf (0 W) und trägt keine Nennleistung."),
                tx("Weathered zinc absorbs and emits more than new zinc (solar absorptance about 0.65 new, 0.78 weathered; emissivity about 0.1 new, 0.28 weathered); the faded white sheeting reads about 0.42 (0.3 new), the sprayed graffiti finger 0.68, so finger 2 runs some 10 K warmer than its neighbours in the sun; the chalked anthracite castings 0.82.", "Bewittertes Zink absorbiert und strahlt mehr als neues (solarer Absorptionsgrad etwa 0,65 neu, 0,78 bewittert; Emissionsgrad etwa 0,1 neu, 0,28 bewittert); die ausgeblichene weiße Folie liegt bei etwa 0,42 (0,3 neu), der übersprühte Finger 2 bei 0,68, er wird in der Sonne rund 10 K wärmer als seine Nachbarn; die gekreideten anthrazitfarbenen Gussteile 0,82."),
                tx("Each finger is a vertical plate with one face to the sun (sunlit fraction about 0.5); the footing is in soil contact at 14 degC over 90 % of its surface and the embedded post foot over 85 %. Fractions are scenario data.", "Jeder Finger ist eine senkrechte Platte mit einer Seite zur Sonne (besonnter Anteil etwa 0,5); das Fundament steht auf 90 % seiner Fläche bei 14 °C im Erdreich, der eingespannte Pfostenfuß auf 85 %. Die Anteile sind Szenariodaten."),
                tx("The 25 mm gaps between the stacked fingers are open to the air: thermal.minimumAirPassageM is 25 mm.", "Die 25-mm-Spalte zwischen den gestapelten Fingern sind luftoffen: thermal.minimumAirPassageM beträgt 25 mm."),
            ],
        },
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["footing-base"], "assumptions": [
            tx("The post is clamped in the point footing and the soil holds the footing: the restraint is idealised as one fixed support at the footing base; the bedding along the 0.8 m depth is not resolved.", "Der Pfosten ist im Punktfundament eingespannt und das Erdreich hält das Fundament: die Einspannung ist als ein festes Auflager an der Fundamentsohle idealisiert; die Bettung über die 0,8 m Tiefe ist nicht aufgelöst."),
            tx("Each finger is bolted in the fork holder of its pipe clamp, and the clamp grips the post by friction; the lowest clamp has lost its preload and turns about the post under 0.1 kN at the tip, which the rig gives as a twist joint of -40 to +5 degrees.", "Jeder Finger ist im Gabelhalter seiner Rohrschelle verschraubt, und die Schelle hält am Pfosten durch Reibung; die unterste Schelle hat ihre Vorspannung verloren und dreht sich unter 0,1 kN an der Spitze um den Pfosten, im Rig ein Drehgelenk von -40 bis +5 Grad."),
            tx("Wind after DIN EN 1991-1-4 with the German annex, zone 2 inland, peak pressure 0.65 kN/m2; force coefficient 1.2 of a sign face after EN 12899-1, applied normal to each finger by the share of its face the wind from ahead sees; 1.0 on the post below the fingers. Hand and bicycle loads are authored estimates.", "Wind nach DIN EN 1991-1-4 mit deutschem Anhang, Zone 2 Binnenland, Böengeschwindigkeitsdruck 0,65 kN/m²; Kraftbeiwert 1,2 einer Schildfläche nach EN 12899-1, normal zu jedem Finger nach dem Anteil seiner Fläche, den der Wind von vorn sieht; 1,0 auf den Pfosten unter den Fingern. Hand- und Fahrradlasten sind Schätzungen."),
            tx("In the swapping pose finger 2 still sits 10 mm in its holder and rests on the worker's two hands, placed either side of its centre of mass and applied as their carrying forces (6.6 N each, together the 13 N weight of the finger), because supports of the archive are springs of equal stiffness that would share the weight of the whole signpost with the footing.", "In der Tauschpose steckt Finger 2 noch 10 mm im Halter und liegt auf den zwei Händen des Mitarbeiters beiderseits seines Schwerpunkts, angesetzt als ihre Tragkräfte (je 6,6 N, zusammen die 13 N Gewicht des Fingers), weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht des ganzen Wegweisers mit dem Fundament teilen würden."),
        ], "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "rotating", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    out = pathlib.Path(__file__).parent / "🔣️.json"
    doc = document()
    out.write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    _, total, _, _ = masses()
    print("[DEBUG] wrote", out, "mass", doc["asset"]["dimensions"], {b: round(t[0], 2) for b, t in total.items()})
