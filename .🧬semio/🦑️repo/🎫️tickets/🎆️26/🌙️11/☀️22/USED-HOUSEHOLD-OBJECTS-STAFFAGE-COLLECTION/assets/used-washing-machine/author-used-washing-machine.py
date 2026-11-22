"""🧺 Author script of the used front-loading washing machine: states every hand-chosen number and writes `🔣️.json` next to it.

World frame of the staffage (x ahead = the door face, y up, z left of the machine), stored pose = rest pose. Every bone but the door
rests with angle 0, so a bone-local point is the world point minus the bone origin; the door bone is twisted 90 degrees so that its
joint axis is the vertical hinge (local u = x, v = -z, w = y). Each scale has its own parts table; the solids and the three 2D views of
a part are written from the same numbers, and the hand-drawn traces of use are added per view and scale.

@see ../../brief.md
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
@see ./author_texts.py
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

BONES = {"floor": (0.0, 0.0, 0.0), "pallet": (-0.40, 0.0, 0.0), "lift": (-0.29, 0.164, 0.0), "travel": (-0.29, 0.164, 0.0), "cabinet": (-0.29, 0.164, 0.0), "drum": (-0.12, 0.624, 0.0), "door": (0.262, 0.624, -0.18)}
DOOR_Y, DOOR_Z = 0.624, 0.0
X_BACK, X_FRONT, Y_BOT, Y_TOP, Z_SIDE = -0.29, 0.25, 0.164, 0.969, 0.2975
PALLET_X_BOTTOM = [(-0.40, -0.30), (-0.05, 0.05), (0.30, 0.40)]
PALLET_Z_ROWS = [(-0.30, -0.20), (-0.05, 0.05), (0.20, 0.30)]
PALLET_X_TOP = [(-0.40, -0.30), (-0.225, -0.125), (-0.05, 0.05), (0.125, 0.225), (0.30, 0.40)]
PALLET_DENSITY = 450
FEET = [(x, z) for x in (-0.19, 0.19) for z in (-0.25, 0.25)]
BOLTS = [(0.88, 0.20), (0.88, -0.20), (0.30, 0.0)]
COIL_Y, COIL_Z = 0.56, 0.0


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounds a number to micrometres for the document."""
    return round(v + 0.0, 6)


def loc(bone, x, y, z):
    """📍️ The bone-local point of a world point at rest."""
    o = BONES[bone]
    if bone == "door":
        return [r6(x - o[0]), r6(-(z - o[2])), r6(y - o[1])]
    return [r6(x - o[0]), r6(y - o[1]), r6(z - o[2])]


def base(sid, bone, mat, role, tone, **extra):
    """🧱️ The common members of a solid."""
    d = {"id": sid, "bone": bone, "material": mat, "role": role, "tone": tone}
    d.update(extra)
    return d


def box(sid, bone, mat, role, tone, x0, x1, y0, y1, z0, z1, **extra):
    """📦️ A box given by its world bounds at rest."""
    c = loc(bone, (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2)
    size = [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)]
    if bone == "door":
        size = [r6(x1 - x0), r6(z1 - z0), r6(y1 - y0)]
    d = base(sid, bone, mat, role, tone, **extra)
    d.update({"kind": "box", "size": size, "transform": {"translation": c}})
    return d


def cyl(sid, bone, mat, role, tone, axis, a0, a1, r, p, q, **extra):
    """🛢️ A solid cylinder along world `axis` (x or y) from `a0` to `a1` of radius `r` through the cross position (p, q)."""
    d = base(sid, bone, mat, role, tone, **extra)
    if axis == "x":
        o = loc(bone, 0, p, q)
        o[0] = 0.0
        prof = [[r6(a0 - BONES[bone][0]), 0], [r6(a0 - BONES[bone][0]), r], [r6(a1 - BONES[bone][0]), r], [r6(a1 - BONES[bone][0]), 0]]
        vec = [1, 0, 0]
    else:
        o = loc(bone, p, 0, q)
        o[1] = 0.0
        prof = [[r6(a0 - BONES[bone][1]), 0], [r6(a0 - BONES[bone][1]), r], [r6(a1 - BONES[bone][1]), r], [r6(a1 - BONES[bone][1]), 0]]
        vec = [0, 1, 0]
    d.update({"kind": "revolve", "profile": prof, "axis": vec, "origin": o})
    return d


def ringx(sid, bone, mat, role, tone, a0, a1, r0, r1, cy, cz, chamfer=0.0, **extra):
    """⭕️ A ring about a world x axis through (cy, cz) as a closed-loop revolve, optionally chamfered on its outer front edge."""
    d = base(sid, bone, mat, role, tone, **extra)
    o = loc(bone, 0, cy, cz)
    o[0] = 0.0
    u0, u1 = r6(a0 - BONES[bone][0]), r6(a1 - BONES[bone][0])
    if chamfer:
        prof = [[u0, r0], [u0, r1], [r6(u1 - chamfer), r1], [u1, r6(r1 - chamfer)], [u1, r0], [u0, r0]]
    else:
        prof = [[u0, r0], [u0, r1], [u1, r1], [u1, r0], [u0, r0]]
    d.update({"kind": "revolve", "profile": prof, "axis": [1, 0, 0], "origin": o})
    return d


def capsule(sid, bone, mat, role, tone, p0, p1, r, **extra):
    """💊️ A capsule between two world points."""
    d = base(sid, bone, mat, role, tone, **extra)
    d.update({"kind": "capsule", "p0": loc(bone, *p0), "p1": loc(bone, *p1), "r0": r, "r1": r})
    return d


def ext(sid, bone, mat, role, tone, plane, profile, frm, to, holes=None, **extra):
    """🧊️ An extrude in bone-local profile coordinates."""
    d = base(sid, bone, mat, role, tone, **extra)
    d.update({"kind": "extrude", "plane": plane, "profile": [[r6(a), r6(b)] for a, b in profile], "from": r6(frm), "to": r6(to)})
    if holes:
        d["holes"] = [[[r6(a), r6(b)] for a, b in h] for h in holes]
    return d


def ref(sid, of, **extra):
    """🔗️ A reference to a body solid."""
    d = {"id": sid, "of": of}
    d.update(extra)
    return d


def circle_pts(ca, cb, r, n=48):
    """🔵️ A polygon of `n` points on a circle."""
    return [(ca + r * math.cos(2 * math.pi * k / n), cb + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def shp(sid, bone, role, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    """✏️ A 2D shape; its paint order `z` is set by its place in the view list."""
    d = {"id": sid, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone}
    if region:
        d["region"] = region
    if regions:
        d["regions"] = regions
    if material:
        d["material"] = material
    if hatch:
        d["hatch"] = hatch
    if pen:
        d["penMm"] = pen
    d.update(geom)
    return d


def erect(bone, x0, x1, y0, y1):
    """📐️ An elevation rectangle from world bounds."""
    o = BONES[bone]
    return {"kind": "rect", "x": r6(x0 - o[0]), "y": r6(y0 - o[1]), "w": r6(x1 - x0), "h": r6(y1 - y0)}


def wrect(a0, a1, b0, b1):
    """📐️ A front (z, y) or plan (x, z) rectangle in world rest coordinates."""
    return {"kind": "rect", "x": r6(a0), "y": r6(b0), "w": r6(a1 - a0), "h": r6(b1 - b0)}


def epoly(bone, pts, closed=True):
    """🔷️ An elevation polygon from world points."""
    o = BONES[bone]
    return {"kind": "poly", "points": [[r6(x - o[0]), r6(y - o[1])] for x, y in pts], "closed": closed}


def wpoly(pts, closed=True):
    """🔷️ A front or plan polygon in world rest coordinates."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def ell(cx, cy, r, ry=None):
    """⭕️ A circle or ellipse in world rest coordinates."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(r), "ry": r6(ry if ry is not None else r)}


def ring_d(cx, cy, r):
    """〰️ The path of one circle ring."""
    return f"M {r6(cx + r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx - r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx + r)} {r6(cy)} Z"


def rect_d(a0, a1, b0, b1):
    """〰️ The path of one rectangle ring."""
    return f"M {r6(a0)} {r6(b0)} L {r6(a1)} {r6(b0)} L {r6(a1)} {r6(b1)} L {r6(a0)} {r6(b1)} Z"


def path(d):
    """〰️ A path geometry."""
    return {"kind": "path", "d": d}


def lines_d(segs):
    """〰️ The path of open strokes, each a list of points."""
    return " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in s) for s in segs)


def epath(bone, segs):
    """〰️ Open elevation strokes from world points."""
    o = BONES[bone]
    return path(lines_d([[(x - o[0], y - o[1]) for x, y in s] for s in segs]))


def number(lst):
    """🔢️ Sets the paint order of a view list: later is nearer."""
    for k, s in enumerate(lst):
        s["z"] = k + 1
    return lst


class Part:
    """🧩️ One part of a scale: its solid and the three 2D shapes drawn from the same numbers."""

    def __init__(self, sid, bone, mat, role, tone, region, kind, dims, views="efp", hatch=None, ebone=None, solid_extra=None, shape_tone=None):
        self.sid, self.bone, self.mat, self.role, self.tone, self.region, self.kind, self.dims = sid, bone, mat, role, tone, region, kind, dims
        self.views, self.hatch, self.ebone, self.solid_extra, self.shape_tone = views, hatch, ebone or bone, solid_extra or {}, shape_tone
        self.drawing_only, self.draw_bounds = False, None

    def solid(self):
        """🧊️ The body solid of the part."""
        k, d, e = self.kind, self.dims, self.solid_extra
        if k == "box":
            return box(self.sid, self.bone, self.mat, self.role, self.tone, *d, **e)
        if k == "cylx":
            a0, a1, r, cy, cz = d
            return cyl(self.sid, self.bone, self.mat, self.role, self.tone, "x", a0, a1, r, cy, cz, **e)
        if k == "cyly":
            a0, a1, r, cx, cz = d
            return cyl(self.sid, self.bone, self.mat, self.role, self.tone, "y", a0, a1, r, cx, cz, **e)
        if k == "ringx":
            a0, a1, r0, r1, cy, cz, ch = d
            return ringx(self.sid, self.bone, self.mat, self.role, self.tone, a0, a1, r0, r1, cy, cz, ch, **e)
        raise ValueError(k)

    def bounds(self):
        """📏️ The world bounds (x0, x1, y0, y1, z0, z1) of the part."""
        k, d = self.kind, self.dims
        if k == "box":
            return d
        if k == "cylx":
            a0, a1, r, cy, cz = d
            return (a0, a1, cy - r, cy + r, cz - r, cz + r)
        if k == "cyly":
            a0, a1, r, cx, cz = d
            return (cx - r, cx + r, a0, a1, cz - r, cz + r)
        a0, a1, r0, r1, cy, cz, ch = d
        return (a0, a1, cy - r1, cy + r1, cz - r1, cz + r1)

    def shapes(self):
        """✏️ The elevation, front and plan shapes of the part (None where the view does not draw it)."""
        x0, x1, y0, y1, z0, z1 = self.draw_bounds or self.bounds()
        kw = dict(region=self.region, material=self.mat, hatch=self.hatch)
        tone = self.shape_tone if self.shape_tone is not None else self.tone
        el = shp(self.sid, self.ebone, self.role, True, tone, erect(self.ebone, x0, x1, y0, y1), **kw) if "e" in self.views else None
        if self.kind in ("cylx",):
            a0, a1, r, cy, cz = self.dims
            fg = ell(cz, cy, r)
        elif self.kind == "ringx":
            a0, a1, r0, r1, cy, cz, ch = self.dims
            fg = path(ring_d(cz, cy, r1) + " " + ring_d(cz, cy, r0))
        else:
            fg = wrect(z0, z1, y0, y1)
        if self.kind == "cyly":
            a0, a1, r, cx, cz = self.dims
            pg = ell(cx, cz, r)
        else:
            pg = wrect(x0, x1, z0, z1)
        fr = shp(self.sid, self.bone, self.role, True, tone, fg, **kw) if "f" in self.views else None
        pl = shp(self.sid, self.bone, self.role, True, tone, pg, **kw) if "p" in self.views else None
        return el, fr, pl


class FrontPlate(Part):
    """🚪️ The front sheet of the cabinet below the fascia with the porthole opening of the door ring, an extrude along u."""

    def __init__(self, sid, x0, x1, y0, y1, zh):
        super().__init__(sid, "cabinet", "enamelled-steel-aged", "body", 0.88, "cabinet", "box", (x0, x1, y0, y1, -zh, zh), views="")
        self.x0, self.x1, self.y0, self.y1, self.zh = x0, x1, y0, y1, zh

    def solid(self):
        """🧊️ The plate with the 0.34 m opening."""
        o = BONES["cabinet"]
        prof = [(self.y0 - o[1], -self.zh), (self.y1 - o[1], -self.zh), (self.y1 - o[1], self.zh), (self.y0 - o[1], self.zh)]
        hole = [(DOOR_Y - o[1] + b, DOOR_Z + a) for a, b in circle_pts(0, 0, 0.17)]
        return ext(self.sid, "cabinet", "enamelled-steel-aged", "body", 0.88, "yz", prof, self.x0 - o[0], self.x1 - o[0], holes=[hole])


def pallet_parts50():
    """🪵️ The EPAL 6 half pallet board by board: three bottom boards, nine blocks, three stringer boards and five top boards."""
    out = []
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        out.append(Part(f"bottom-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.5, "pallet", "box", (a, b, 0, 0.022, -0.30, 0.30), hatch="grain"))
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        for j, (c, e) in enumerate(PALLET_Z_ROWS):
            out.append(Part(f"block-{k + 1}-{j + 1}", "pallet", "pallet-softwood", "body", 0.45, "pallet", "box", (a, b, 0.022, 0.100, c, e)))
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        out.append(Part(f"stringer-{k + 1}", "pallet", "pallet-softwood", "body", 0.5, "pallet", "box", (-0.40, 0.40, 0.100, 0.122, a, b), hatch="grain"))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        out.append(Part(f"top-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.55, "pallet", "box", (a, b, 0.122, 0.144, -0.30, 0.30), hatch="grain"))
    return out


def pallet_mass():
    """⚖️ The pallet mass from the 1:50 board volumes and the density, with its centre height."""
    vol, moment = 0.0, 0.0
    for p in pallet_parts50():
        x0, x1, y0, y1, z0, z1 = p.bounds()
        v = (x1 - x0) * (y1 - y0) * (z1 - z0)
        vol += v
        moment += v * (y0 + y1) / 2
    return vol * PALLET_DENSITY, moment / vol


def machine_parts(scale):
    """🧺️ The washing machine parts of a scale (cabinet, worktop, fascia, door, drum, back, hoses, feet)."""
    P = []
    if scale <= 100:
        for k, (x, z) in enumerate(FEET):
            P.append(Part(f"foot-{k + 1}", "cabinet", "steel-foot", "detail", 0.25, "cabinet", "cyly", (0.144, 0.164, 0.018, x, z)))
        t = 0.008 if scale == 50 else 0.015
        xb = X_BACK if scale == 50 else -0.285
        P.append(Part("cabinet-shell", "cabinet", "enamelled-steel-aged", "body", 0.88, "cabinet", "box", (X_BACK, X_FRONT, Y_BOT, Y_TOP, -Z_SIDE, Z_SIDE), views="ep"))
        P[-1].drawing_only = True
        P.append(Part("cabinet-side-r", "cabinet", "enamelled-steel-aged", "body", 0.88, "cabinet", "box", (xb, X_FRONT, Y_BOT, Y_TOP, -Z_SIDE, -Z_SIDE + t), views=""))
        P.append(Part("cabinet-side-l", "cabinet", "enamelled-steel-aged", "body", 0.88, "cabinet", "box", (xb, X_FRONT, Y_BOT, Y_TOP, Z_SIDE - t, Z_SIDE), views=""))
        P.append(Part("cabinet-base", "cabinet", "enamelled-steel-aged", "body", 0.8, "cabinet", "box", (xb, X_FRONT, Y_BOT, Y_BOT + t, -Z_SIDE + t, Z_SIDE - t), views=""))
        P.append(FrontPlate("cabinet-front", X_FRONT - t, X_FRONT, Y_BOT + t, 0.864, Z_SIDE - t))
        P.append(Part("tub", "cabinet", "pp-tub", "detail", 0.4, None, "cylx", (-0.16 if scale == 50 else -0.15, X_FRONT - t, 0.27 if scale == 50 else 0.26, DOOR_Y, DOOR_Z), views=""))
        P.append(Part("back-panel", "cabinet", "galvanised-steel", "body", 0.6, "back-panel", "box", (-0.30, xb, Y_BOT, Y_TOP, -Z_SIDE, Z_SIDE), views="ep"))
        for k, (y, z) in enumerate(BOLTS):
            P.append(Part(f"transit-bolt-{k + 1}", "cabinet", "galvanised-steel", "detail", 0.3, "back-panel", "cylx", (-0.322, -0.30, 0.016, y, z), views="ep"))
        P.append(Part("hose-coil", "cabinet", "pvc-hose", "body", 0.35, "hoses", "ringx", (-0.336, -0.30, 0.15, 0.19, COIL_Y, COIL_Z, 0.0), views="ep", hatch="corrugation" if scale == 50 else None))
        P.append(Part("worktop", "cabinet", "melamine-chipboard-aged", "body", 0.82, "worktop", "box", (-0.30, 0.265, Y_TOP, 0.994, -0.30, 0.30), solid_extra={"cornerRadius": 0.004}))
        P.append(Part("fascia", "cabinet", "abs-aged", "body", 0.8, "fascia", "box", (X_FRONT - t, 0.265, 0.864, Y_TOP, -Z_SIDE + t, Z_SIDE - t)))
        P[-1].draw_bounds = (X_FRONT, 0.265, 0.864, Y_TOP, -Z_SIDE, Z_SIDE)
        if scale == 50:
            P.append(Part("drawer-front", "cabinet", "abs-aged", "detail", 0.75, "fascia", "box", (0.265, 0.275, 0.875, 0.955, -0.285, -0.085)))
        P.append(Part("programme-knob", "cabinet", "abs-aged", "detail", 0.6, "fascia", "cylx", (0.265, 0.29, 0.03, 0.915, 0.18)))
        P.append(Part("door-ring", "door", "abs-chrome-worn", "body", 0.55, "door", "ringx", (0.25, 0.285, 0.125, 0.17, DOOR_Y, DOOR_Z, 0.01 if scale == 50 else 0.0), ebone="cabinet"))
        P.append(Part("door-handle", "door", "abs-chrome-worn", "detail", 0.45, "door", "box", (0.27, 0.29, 0.585, 0.665, 0.162, 0.182), ebone="cabinet"))
        P.append(Part("door-hinge", "door", "abs-chrome-worn", "detail", 0.4, "door", "box", (X_FRONT, 0.274, 0.57, 0.68, -0.195, -0.165), ebone="cabinet"))
    return P


def door_glass(sid, scale):
    """🪟️ The door glass bowl: a dome 36 mm proud of the ring seat, a revolve about the door axis."""
    d = base(sid, "door", "door-glass", "glazing", 0.3)
    u0 = r6(0.26 - BONES["door"][0])
    prof = [[u0, 0], [u0, 0.125], [r6(u0 + 0.02), 0.125], [r6(u0 + 0.032), 0.08], [r6(u0 + 0.036), 0]] if scale == 50 else [[u0, 0], [u0, 0.125], [r6(u0 + 0.025), 0.125], [r6(u0 + 0.036), 0.06], [r6(u0 + 0.036), 0]]
    d.update({"kind": "revolve", "profile": prof, "axis": [1, 0, 0], "origin": loc("door", 0, DOOR_Y, DOOR_Z)})
    d["origin"][0] = 0.0
    return d


def drum_solid(sid, r):
    """🥁️ The stainless drum inside the tub, a cylinder on the drum bone (hidden behind the cabinet and seen through the glass)."""
    d = base(sid, "drum", "stainless-steel", "detail", 0.35)
    d.update({"kind": "revolve", "profile": [[0, 0], [0, r], [0.33, r], [0.33, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]})
    return d


def bearing(sid, r):
    """⚙️ The drum bearing and shaft at the rear of the drum (joint of the drum member)."""
    d = base(sid, "drum", "stainless-steel", "body", 0.3, member="drum", part="joint")
    d.update({"kind": "revolve", "profile": [[-0.08, 0], [-0.08, r], [0.0, r], [0.0, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]})
    return d


def mass_marker(bone, x, y, w, r):
    """⚖️ A mass marker at the centre of mass of a bone (bone-local)."""
    d = base(f"mass-{bone}", bone, "steel-foot", "body", 0.2, member=bone, part="mass")
    d.update({"kind": "ellipsoid", "radii": [r, r, r], "transform": {"translation": [r6(x), r6(y), r6(w)]}})
    return d


def vent_solid(t, x_face):
    """💨️ The emitter outlet of the tub vent at the detergent drawer, `t` thick, its face at `x_face`."""
    s = box("outlet-tub-vent", "cabinet", "abs-aged", "detail", 0.7, x_face - t, x_face, 0.905, 0.925 if t < 0.02 else 0.905 + t, -0.20, -0.17 if t < 0.03 else -0.20 + t, tags=["tub-vent"], region="fascia")
    return s


REGION_OF = {"bottom-board": "pallet", "block": "pallet", "stringer": "pallet", "top-board": "pallet", "deck": "pallet", "bearer": "pallet", "runner": "pallet", "pallet-slab": "pallet", "foot": "cabinet", "cabinet": "cabinet", "back-panel": "back-panel", "transit-bolt": "back-panel", "hose": "hoses", "worktop": "worktop", "fascia": "fascia", "drawer": "fascia", "programme-knob": "fascia", "door": "door"}


def thermal_refs(body):
    """🌡️ The thermal references of a body list by the region of each solid's id prefix (the hidden drum is no surface)."""
    out = []
    for s in body:
        if s["bone"] == "drum" or s["id"] == "tub":
            continue
        key = next(k for k in sorted(REGION_OF, key=len, reverse=True) if s["id"].startswith(k))
        out.append(ref(f"th-{s['id']}", s["id"], region=REGION_OF[key]))
    return out


PALLET_KG, PALLET_COMY = pallet_mass()
CABINET_KG, DRUM_KG = 65.5, 6.5
CABINET_COM = (0.27, 0.36)
DRUM_COM = (0.165, 0.0)


def structure_list(body, feet_joint, drum_r, mass_r, extra_members=()):
    """🏗️ The structural list: pallet members, the cabinet shell as frame with worktop, feet as joints, drum member and bearing, masses."""
    st = []
    for s in body:
        if s["bone"] == "pallet":
            st.append(ref(f"m-{s['id']}", s["id"], member="pallet", part="member"))
    ids = {s["id"] for s in body}
    for s in body:
        if s["id"].startswith("cabinet-") or s["id"] in ("worktop", "back-panel", "tub"):
            st.append(ref(f"m-{s['id']}", s["id"], member="cabinet", part="member"))
    st += list(extra_members)
    if feet_joint:
        for s in body:
            if s["id"].startswith("foot-"):
                st.append(ref(f"j-{s['id']}", s["id"], member="cabinet", part="joint"))
    st.append(ref("m-drum", "drum", member="drum", part="member"))
    st.append(bearing("j-drum-bearing", drum_r))
    st.append(mass_marker("pallet", 0.40, PALLET_COMY, 0.0, mass_r))
    st.append(mass_marker("cabinet", CABINET_COM[0], CABINET_COM[1], 0.0, mass_r))
    st.append(mass_marker("drum", DRUM_COM[0], DRUM_COM[1], 0.0, mass_r))
    return st


def scale50():
    """🔍️ The 1:50 model: pallet board by board, feet, cabinet shell with rounded edges, raised back panel, three refitted transit bolts, the taped hose coil and the drain hose hook, worktop, fascia, drawer, knob, door ring with chamfer, glass bowl, handle, hinge and the drum."""
    parts = pallet_parts50() + machine_parts(50)
    body = [p.solid() for p in parts if not p.drawing_only]
    body.append(capsule("hose-hook", "cabinet", "pvc-hose", "detail", 0.3, (-0.312, 0.64, -0.15), (-0.312, 0.93, -0.15), 0.012))
    body.append(door_glass("door-glass", 50))
    body.append(drum_solid("drum", 0.235))
    th = thermal_refs(body) + [vent_solid(0.008, 0.275)]
    st = structure_list(body, True, 0.03, 0.03)
    return parts, body, th, st


def scale100():
    """🔍️ The 1:100 model: pallet runners for stringer and blocks, the back panel and the drawer front merged into shell and fascia, no hose hook."""
    parts = pallet_parts100() + machine_parts(100)
    body = [p.solid() for p in parts if not p.drawing_only]
    body.append(door_glass("door-glass", 100))
    body.append(drum_solid("drum", 0.235))
    th = thermal_refs(body) + [vent_solid(0.015, 0.265)]
    st = structure_list(body, True, 0.03, 0.035)
    return parts, body, th, st


RUNNER = [(0, 0.122), (0.80, 0.122), (0.80, 0.022), (0.70, 0.022), (0.70, 0.100), (0.45, 0.100), (0.45, 0.022), (0.35, 0.022), (0.35, 0.100), (0.10, 0.100), (0.10, 0.022), (0, 0.022)]


def pallet_parts100():
    """🪵️ The 1:100 pallet: bottom and top boards kept, stringer and blocks of each row as one notched runner (drawn as a Part only for its 2D bounds)."""
    out = []
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        out.append(Part(f"bottom-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.5, "pallet", "box", (a, b, 0, 0.022, -0.30, 0.30)))
    for k, (a, b) in enumerate(PALLET_Z_ROWS):
        out.append(RunnerPart(f"runner-{k + 1}", a, b))
    for k, (a, b) in enumerate(PALLET_X_TOP):
        out.append(Part(f"top-board-{k + 1}", "pallet", "pallet-softwood", "body", 0.55, "pallet", "box", (a, b, 0.122, 0.144, -0.30, 0.30)))
    return out


class RunnerPart(Part):
    """🛷️ A notched pallet runner: one extrude of stringer board and three blocks."""

    def __init__(self, sid, z0, z1):
        super().__init__(sid, "pallet", "pallet-softwood", "body", 0.48, "pallet", "box", (-0.40, 0.40, 0.022, 0.122, z0, z1))
        self.z0, self.z1 = z0, z1

    def solid(self):
        """🧊️ The runner extrude along w."""
        return ext(self.sid, "pallet", "pallet-softwood", "body", 0.48, "xy", RUNNER, self.z0, self.z1)

    def shapes(self):
        """✏️ The runner outline in elevation, its end in front and its strip in plan."""
        el = shp(self.sid, "pallet", "body", True, 0.48, {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in RUNNER], "closed": True}, region="pallet", material="pallet-softwood")
        fr = shp(self.sid, "pallet", "body", True, 0.48, wrect(self.z0, self.z1, 0.022, 0.122), region="pallet", material="pallet-softwood")
        pl = shp(self.sid, "pallet", "body", True, 0.48, wrect(-0.40, 0.40, self.z0, self.z1), region="pallet", material="pallet-softwood")
        return el, fr, pl


def scale200():
    """🔍️ The 1:200 model: pallet as deck and three bearers, the machine as one cabinet block from pallet to worktop, a 30 mm fascia band, the door as one disc, the hose coil and the drum."""
    parts = [Part("pallet-deck", "pallet", "pallet-softwood", "body", 0.55, "pallet", "box", (-0.40, 0.40, 0.100, 0.144, -0.30, 0.30))]
    for k, (a, b) in enumerate(PALLET_X_BOTTOM):
        parts.append(Part(f"bearer-{k + 1}", "pallet", "pallet-softwood", "body", 0.48, "pallet", "box", (a, b, 0, 0.100, -0.30, 0.30)))
    parts += [
        Part("cabinet-block", "cabinet", "enamelled-steel-aged", "body", 0.86, "cabinet", "box", (-0.30, X_FRONT, 0.144, 0.994, -0.30, 0.30)),
        Part("fascia", "cabinet", "abs-aged", "body", 0.8, "fascia", "box", (X_FRONT, 0.28, 0.864, Y_TOP, -Z_SIDE, Z_SIDE)),
        Part("door-disc", "door", "abs-chrome-worn", "body", 0.5, "door", "cylx", (X_FRONT, 0.29, 0.17, DOOR_Y, DOOR_Z), ebone="cabinet"),
        Part("hose-coil", "cabinet", "pvc-hose", "body", 0.35, "hoses", "ringx", (-0.336, -0.30, 0.15, 0.19, COIL_Y, COIL_Z, 0.0), views="ep"),
    ]
    body = [p.solid() for p in parts if not p.drawing_only]
    body.append(drum_solid("drum", 0.235))
    th = [ref("th-pallet-deck", "pallet-deck", region="pallet")] + [ref(f"th-bearer-{k + 1}", f"bearer-{k + 1}", region="pallet") for k in range(3)]
    th += hollow_thermal(0.03, 0.864, 0.964, 0.28) + [
        ref("th-door-disc", "door-disc", region="door"),
        ref("th-hose-coil", "hose-coil", region="hoses"),
        vent(0.25, 0.28, 0.905, 0.935, -0.20, -0.17),
    ]
    st = structure_list(body, False, 0.04, 0.04)
    return parts, body, th, st


def vent(x0, x1, y0, y1, z0, z1):
    """💨️ The emitter outlet of the tub vent at the detergent drawer, by its world bounds."""
    return box("outlet-tub-vent", "cabinet", "abs-aged", "detail", 0.7, x0, x1, y0, y1, z0, z1, tags=["tub-vent"], region="fascia")


def hollow_thermal(t, y_fascia, y_top, x_fascia):
    """🌡️ The coarse thermal model of the machine as the closed shell it is: sides, base and front with the door opening (cabinet), back wall, fascia and worktop, each `t` thick, so no inner face counts as outer surface."""
    o = BONES["cabinet"]
    xb, xf, yb, zs = -0.30 + t, X_FRONT, 0.144, 0.30
    prof = [(yb + t - o[1], -(zs - t)), (y_fascia - o[1], -(zs - t)), (y_fascia - o[1], zs - t), (yb + t - o[1], zs - t)]
    hole = [(DOOR_Y - o[1] + b, DOOR_Z + a) for a, b in circle_pts(0, 0, 0.17)]
    return [
        box("th-cabinet-side-r", "cabinet", "enamelled-steel-aged", "body", 0.86, xb, xf, yb, y_top, -zs, -zs + t, region="cabinet"),
        box("th-cabinet-side-l", "cabinet", "enamelled-steel-aged", "body", 0.86, xb, xf, yb, y_top, zs - t, zs, region="cabinet"),
        box("th-cabinet-base", "cabinet", "enamelled-steel-aged", "body", 0.86, xb, xf, yb, yb + t, -zs + t, zs - t, region="cabinet"),
        ext("th-cabinet-front", "cabinet", "enamelled-steel-aged", "body", 0.86, "yz", prof, xf - t - o[0], xf - o[0], holes=[hole], region="cabinet"),
        box("th-fascia", "cabinet", "abs-aged", "body", 0.8, xf - t, x_fascia if x_fascia > xf else xf, y_fascia, y_top, -zs + t, zs - t, region="fascia"),
        box("th-back-panel", "cabinet", "galvanised-steel", "body", 0.6, -0.30, xb, yb, y_top, -zs, zs, region="back-panel"),
        box("th-worktop", "cabinet", "melamine-chipboard-aged", "body", 0.82, -0.30, xf, y_top, 0.994, -zs, zs, region="worktop"),
    ]


def scale500():
    """🔍️ The 1:500 model: pallet as one slab and the machine as one block from the pallet to the top of the worktop."""
    parts = [
        Part("pallet-slab", "pallet", "pallet-softwood", "body", 0.5, "pallet", "box", (-0.40, 0.40, 0, 0.144, -0.30, 0.30)),
        Part("cabinet-block", "cabinet", "enamelled-steel-aged", "body", 0.86, "cabinet", "box", (-0.30, X_FRONT, 0.144, 0.994, -0.30, 0.30)),
    ]
    body = [p.solid() for p in parts if not p.drawing_only]
    body.append(drum_solid("drum", 0.235))
    th = [ref("th-pallet-slab", "pallet-slab", region="pallet")]
    th += hollow_thermal(0.075, 0.844, 0.919, X_FRONT) + [
        cyl("th-door", "door", "abs-chrome-worn", "body", 0.5, "x", X_FRONT, 0.325, 0.17, DOOR_Y, DOOR_Z, region="door"),
        box("th-hoses", "cabinet", "pvc-hose", "body", 0.35, -0.375, -0.30, 0.37, 0.75, -0.19, 0.19, region="hoses"),
        vent(0.175, X_FRONT, 0.844, 0.919, -0.20, -0.125),
    ]
    st = structure_list(body, False, 0.075, 0.075)
    return parts, body, th, st


def base_views(parts):
    """✏️ The part shapes of the three views in list order."""
    el, fr, pl = [], [], []
    for p in parts:
        e, f, q = p.shapes()
        if e:
            el.append(e)
        if f:
            fr.append(f)
        if q:
            pl.append(q)
    return el, fr, pl


def cabinet_front(hole_r, y0, y1):
    """🧱️ The cabinet front with the door opening (the glass radius), so the drum shows through the glazing."""
    return path(rect_d(-Z_SIDE, Z_SIDE, y0, y1) + " " + ring_d(DOOR_Z, DOOR_Y, hole_r))


def drum_front(paddles):
    """🥁️ The drum face behind the glass with its three lifters, on the drum bone as transverse profiles (v, w) swept along the drum axis, so they turn with the drum and tilt with the cabinet."""
    sweep = {"kind": "extruded", "sweep": "u", "z0": 0.31, "z1": 0.33}
    out = [shp("drum-face", "drum", "detail", True, 0.3, ell(0, 0, 0.124), material="stainless-steel")]
    out[0]["solid"] = dict(sweep)
    if paddles:
        for k in range(3):
            t = 2 * math.pi * k / 3 + math.pi / 2
            pts = []
            for rr, ww in ((0.055, -0.016), (0.118, -0.012), (0.118, 0.012), (0.055, 0.016)):
                pts.append((rr * math.sin(t) + ww * math.cos(t), rr * math.cos(t) - ww * math.sin(t)))
            s = shp(f"drum-lifter-{k + 1}", "drum", "detail", True, 0.5, wpoly(pts), material="stainless-steel")
            s["solid"] = dict(sweep)
            out.append(s)
    return out


def glass_elev(scale):
    """🪟️ The side outline of the glass bowl proud of the ring (on the cabinet drawing)."""
    if scale == 50:
        pts = [(0.285, DOOR_Y - 0.11), (0.29, DOOR_Y - 0.08), (0.296, DOOR_Y), (0.29, DOOR_Y + 0.08), (0.285, DOOR_Y + 0.11)]
    else:
        pts = [(0.285, DOOR_Y - 0.09), (0.296, DOOR_Y - 0.06), (0.296, DOOR_Y + 0.06), (0.285, DOOR_Y + 0.09)]
    return shp("door-glass", "cabinet", "glazing", True, 0.3, epoly("cabinet", pts), region="door", material="door-glass")


def views50(parts):
    """✏️ The 1:50 drawings: the parts with the traces of use drawn on them."""
    el, fr, pl = base_views(parts)
    el.append(shp("hose-hook", "cabinet", "detail", True, 0.3, erect("cabinet", -0.324, -0.300, 0.628, 0.942), region="hoses", material="pvc-hose"))
    el.append(glass_elev(50))
    el += [
        shp("side-scratches", "cabinet", "texture", False, 0.5, epath("cabinet", [[(-0.20, 0.52), (-0.02, 0.47)], [(-0.15, 0.40), (0.04, 0.37)], [(0.05, 0.62), (0.16, 0.60)]]), region="cabinet"),
        shp("side-dent", "cabinet", "texture", False, 0.5, {"kind": "ellipse", "cx": r6(0.08 + 0.29), "cy": r6(0.30 - 0.164), "rx": 0.045, "ry": 0.03}, region="cabinet"),
        shp("rust-edge", "cabinet", "texture", True, 0.45, epoly("cabinet", [(0.20, 0.164), (0.25, 0.164), (0.25, 0.19), (0.23, 0.182)]), region="cabinet"),
        shp("coil-tape", "cabinet", "texture", True, 0.7, erect("cabinet", -0.338, -0.300, 0.70, 0.745), region="hoses"),
        shp("coil-tape-low", "cabinet", "texture", True, 0.7, erect("cabinet", -0.338, -0.300, 0.375, 0.42), region="hoses"),
        shp("worktop-chip", "cabinet", "texture", True, 0.6, epoly("cabinet", [(0.245, 0.994), (0.265, 0.994), (0.265, 0.979)]), region="worktop"),
    ]
    fr.insert(0, shp("cabinet-front", "cabinet", "body", True, 0.88, cabinet_front(0.125, Y_BOT, 0.864), region="cabinet", material="enamelled-steel-aged"))
    fr = fr[:1] + drum_front(True) + fr[1:]
    fr += [
        shp("door-glass", "door", "glazing", True, 0.3, ell(DOOR_Z, DOOR_Y, 0.125), region="door", material="door-glass"),
        shp("door-seal-limescale", "door", "texture", True, 0.75, path(ring_d(DOOR_Z, DOOR_Y, 0.125) + " " + ring_d(DOOR_Z, DOOR_Y, 0.108)), region="door", hatch="limescale"),
        shp("door-tape", "cabinet", "texture", True, 0.7, wpoly([(0.12, DOOR_Y + 0.02), (0.26, DOOR_Y + 0.02), (0.26, DOOR_Y - 0.02), (0.12, DOOR_Y - 0.02)]), region="door"),
        shp("display-window", "cabinet", "detail", True, 0.25, wrect(-0.05, 0.10, 0.895, 0.935), region="fascia", material="abs-aged"),
        shp("buttons", "cabinet", "texture", False, 0.4, path(" ".join(ring_d(z, 0.88, 0.007) for z in (-0.04, 0.0, 0.04, 0.08))), region="fascia"),
        shp("knob-pointer", "cabinet", "texture", False, 0.3, path(f"M 0.18 0.915 L 0.18 0.94"), region="fascia"),
        shp("drawer-grip", "cabinet", "texture", False, 0.4, path("M -0.27 0.885 L -0.20 0.885"), region="fascia"),
        shp("detergent-residue", "cabinet", "texture", True, 0.55, wpoly([(-0.285, 0.875), (-0.22, 0.875), (-0.24, 0.868), (-0.27, 0.866)]), region="fascia"),
        shp("pump-flap", "cabinet", "detail", False, 0.5, path(rect_d(0.13, 0.27, 0.185, 0.265)), region="cabinet"),
        shp("label-residue", "cabinet", "texture", True, 0.75, wpoly([(0.16, 0.80), (0.27, 0.80), (0.27, 0.835), (0.18, 0.84)]), region="cabinet"),
        shp("front-scuffs", "cabinet", "texture", False, 0.5, path("M -0.26 0.22 L -0.17 0.205 M -0.25 0.30 L -0.20 0.295"), region="cabinet"),
        shp("rust-corner", "cabinet", "texture", True, 0.45, wpoly([(-Z_SIDE, Y_BOT), (-0.25, Y_BOT), (-0.27, 0.185), (-Z_SIDE, 0.19)]), region="cabinet"),
    ]
    pl += [
        shp("cup-ring", "cabinet", "texture", False, 0.5, ell(-0.05, 0.12, 0.04), region="worktop"),
        shp("bottle-ring", "cabinet", "texture", False, 0.5, ell(0.10, -0.14, 0.035), region="worktop"),
        shp("worktop-tape", "cabinet", "texture", True, 0.7, wrect(0.20, 0.265, -0.03, 0.01), region="worktop"),
        shp("worktop-chip", "cabinet", "texture", True, 0.6, wpoly([(0.245, 0.30), (0.265, 0.30), (0.265, 0.275)]), region="worktop"),
        shp("worktop-scratches", "cabinet", "texture", False, 0.5, path("M -0.20 -0.05 L -0.05 -0.09 M -0.12 0.20 L 0.02 0.17"), region="worktop"),
    ]
    return number(el), number(fr), number(pl)


def views100(parts):
    """✏️ The 1:100 drawings: the parts with the decisive traces (tape, limescale ring, label residue)."""
    el, fr, pl = base_views(parts)
    el.append(glass_elev(100))
    el.append(shp("coil-tape", "cabinet", "texture", True, 0.7, erect("cabinet", -0.338, -0.300, 0.69, 0.75), region="hoses"))
    fr.insert(0, shp("cabinet-front", "cabinet", "body", True, 0.88, cabinet_front(0.125, Y_BOT, 0.864), region="cabinet", material="enamelled-steel-aged"))
    fr = fr[:1] + drum_front(True) + fr[1:]
    fr += [
        shp("door-glass", "door", "glazing", True, 0.3, ell(DOOR_Z, DOOR_Y, 0.125), region="door", material="door-glass"),
        shp("door-seal-limescale", "door", "texture", True, 0.75, path(ring_d(DOOR_Z, DOOR_Y, 0.125) + " " + ring_d(DOOR_Z, DOOR_Y, 0.105)), region="door"),
        shp("door-tape", "cabinet", "texture", True, 0.7, wpoly([(0.12, DOOR_Y + 0.025), (0.26, DOOR_Y + 0.025), (0.26, DOOR_Y - 0.025), (0.12, DOOR_Y - 0.025)]), region="door"),
        shp("drawer-outline", "cabinet", "detail", False, 0.5, path(rect_d(-0.285, -0.085, 0.875, 0.955)), region="fascia"),
        shp("display-window", "cabinet", "detail", True, 0.25, wrect(-0.05, 0.10, 0.895, 0.935), region="fascia", material="abs-aged"),
        shp("label-residue", "cabinet", "texture", True, 0.75, wpoly([(0.16, 0.80), (0.27, 0.80), (0.27, 0.835), (0.18, 0.84)]), region="cabinet"),
    ]
    pl += [
        shp("cup-ring", "cabinet", "texture", False, 0.5, ell(-0.05, 0.12, 0.04), region="worktop"),
        shp("worktop-tape", "cabinet", "texture", True, 0.7, wrect(0.13, 0.225, -0.045, 0.01), region="worktop"),
    ]
    return number(el), number(fr), number(pl)


def views200(parts):
    """✏️ The 1:200 drawings: deck and bearers, the cabinet block, fascia band, door disc, coil; the glass as one stroke."""
    el, fr, pl = base_views(parts)
    for s in fr:
        if s["id"] == "cabinet-block":
            s["regions"] = ["cabinet", "worktop", "back-panel"]
            s.pop("region", None)
    for s in el + pl:
        if s["id"] == "cabinet-block":
            s["regions"] = ["cabinet", "worktop", "back-panel"]
            s.pop("region", None)
    fr.append(shp("door-glass", "door", "detail", False, 0.3, ell(DOOR_Z, DOOR_Y, 0.12), region="door", material="door-glass"))
    fr.append(shp("worktop-joint", "cabinet", "detail", False, 0.5, path(f"M {-0.30} {Y_TOP} L 0.30 {Y_TOP}"), region="worktop"))
    return number(el), number(fr), number(pl)


UNIT_REGIONS_500 = ["cabinet", "worktop", "back-panel", "fascia", "door", "hoses"]


def views500(parts):
    """✏️ The 1:500 drawings: pallet slab and machine block, the door as one circle stroke on the front."""
    el, fr, pl = base_views(parts)
    for s in el + fr + pl:
        if s["id"] == "cabinet-block":
            s["regions"] = UNIT_REGIONS_500
            s.pop("region", None)
    fr.append(shp("door-outline", "cabinet", "detail", False, 0.3, ell(DOOR_Z, DOOR_Y, 0.17), region="door", material="abs-chrome-worn"))
    return number(el), number(fr), number(pl)


exec(open(os.path.join(HERE, "author_texts.py"), encoding="utf-8").read())


def build():
    """🏗️ Assembles the asset document and writes it."""
    vols, reps = {}, {}
    builders = {50: (scale50, views50), 100: (scale100, views100), 200: (scale200, views200), 500: (scale500, views500)}
    for den, (vb, rb) in builders.items():
        parts, body, th, st = vb()
        el, fr, pl = rb(parts)
        vols[str(den)] = {"status": "authored", "denominator": den, "body": body, "thermal": th, "structure": st, "decisions": VOLUME_DECISIONS[den]}
        reps[str(den)] = {"status": "authored", "denominator": den, "validRange": VALID[den], "pens": PENS[den], "hatches": HATCHES[den], "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": REP_DECISIONS[den]}
    asset = {
        "id": "used-washing-machine",
        "category": "appliances",
        "title": T("Used Washing Machine", "Gebrauchte Waschmaschine"),
        "summary": SUMMARY,
        "tags": ["used", "household", "second-hand", "utility", "bathroom", "appliance", "washing-machine", "front-loader", "laundry", "enamelled-steel", "glass", "pallet", "transit-bolts", "limescale"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": 0.994, "widthM": 0.8, "depthM": 0.6, "massKg": r6(PALLET_KG + CABINET_KG + DRUM_KG)},
        "materials": MATERIALS,
        "rig": RIG,
        "volumes": vols,
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": THERMAL,
        "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("[DEBUG] wrote", OUT, {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in vols.items()}, "mass", asset["dimensions"]["massKg"], "pallet", round(PALLET_KG, 3))


if __name__ == "__main__":
    build()
