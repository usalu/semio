"""🧩 Shape and solid builders of `used-water-play-table`: one `Scale` collects the three 2D views and the body, thermal and structure lists of one authored scale from the numbers of `author_geometry`.

@see ./author_geometry.py — the hand-chosen numbers
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — `Shape`, `VolumeSolid`
"""
from author_geometry import BONES, local, r5

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SEGMENTS = {50: 64, 100: 48, 200: 32, 500: 24}


def tx(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def shape(sid, bone, role, z, filled, tone, geom, region=None, material=None, hatch=None, pen=None, solid=None):
    """✏️ One authored 2D shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    for key, value in (("region", region), ("material", material), ("hatch", hatch), ("penMm", pen), ("solid", solid)):
        if value is not None:
            out[key] = value
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


def ring_path(outer, inner):
    """🔲 A filled ring as one path: the outer and the inner rectangle (x0, y0, x1, y1), even-odd."""
    def sub(b):
        x0, y0, x1, y1 = (r5(c) for c in b)
        return f"M {x0} {y0} L {x1} {y0} L {x1} {y1} L {x0} {y1} Z"
    return path(sub(outer) + " " + sub(inner))


def solid(sid, kind, bone, material, role, tone, **fields):
    """🧊 One volume solid."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def rev(sid, bone, material, role, tone, profile, axis, origin, **extra):
    """🌀 A revolve solid."""
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(b)] for a, b in profile], axis=axis, origin=[r5(c) for c in origin], **extra)


def ext(sid, bone, material, role, tone, profile, plane, frm, to, holes=None, **extra):
    """🧱 An extrude solid."""
    fields = {"profile": [[r5(a), r5(b)] for a, b in profile], "plane": plane, "from": r5(frm), "to": r5(to)}
    if holes:
        fields["holes"] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    fields.update(extra)
    return solid(sid, "extrude", bone, material, role, tone, **fields)


def box(sid, bone, material, role, tone, size, centre, rotation=None, **extra):
    """📦 A box solid centred at `centre` (bone-local), optionally turned by a quaternion."""
    t = {"translation": [r5(c) for c in centre]}
    if rotation:
        t["rotation"] = rotation
    return solid(sid, "box", bone, material, role, tone, size=[r5(s) for s in size], transform=t, **extra)


def ell(sid, bone, material, radii, centre, **extra):
    """🔵 A mass marker ellipsoid."""
    return solid(sid, "ellipsoid", bone, material, "detail", 0.15, radii=[r5(r) for r in radii], transform={"translation": [r5(c) for c in centre]}, **extra)


def annulus(a0, a1, r0, r1):
    """🔘 A closed revolve loop of a rectangle in (axial, radial): a ring, collar or tube wall."""
    return [(a0, r0), (a1, r0), (a1, r1), (a0, r1), (a0, r0)]


def revolved(bone, axis="u"):
    """🌀 The revolved 2D solid about the axis `axis` through the origin of `bone`."""
    return {"kind": "revolved", "axisBone": bone, "axis": axis, "axisX": 0, "axisZ": 0, "mirror": True}


def extruded(z0, z1):
    """🧱 The extruded 2D solid swept along w between the rest lateral positions z0 and z1."""
    return {"kind": "extruded", "sweep": "w", "z0": r5(z0), "z1": r5(z1)}


class Scale:
    """📏 Collects the 2D shapes and 3D solids of one authored scale."""

    def __init__(self, s):
        self.s = s
        self.f = FEATURE[s]
        self.n = SEGMENTS[s]
        self.elev, self.front, self.plan = [], [], []
        self.body, self.thermal, self.structure = [], [], []

    def th(self, sid, region, **extra):
        """🌡️ A thermal reference to a body solid."""
        self.thermal.append({"id": "th-" + sid, "of": sid, "region": region, **extra})

    def st(self, sid, member, part, **extra):
        """🏗️ A structural reference to a body solid."""
        self.structure.append({"id": "st-" + sid, "of": sid, "member": member, "part": part, **extra})

    def block(self, sid, bone, b, region, material, tone, views="efp", struct=None, member=None, hatch=None, sheet=False, order=None, body=True):
        """📦 A world-aligned box part (x0, x1, y0, y1, z0, z1) on a bone that rests along or across x: the three drawings, the body box, its thermal reference and optionally its structural reference."""
        x0, x1, y0, y1, z0, z1 = b
        oz = BONES[bone]["z"]
        u0, v0 = local(bone, x0, y0)
        u1, v1 = local(bone, x1, y1)
        cu, cv = (u0 + u1) / 2, (v0 + v1) / 2
        size = [abs(u1 - u0), abs(v1 - v0), z1 - z0]
        oe, of, op = order if order else (2.0 - z0, 2.0 + x1, 2.0 + y1)
        if "e" in views:
            self.elev.append(shape(sid, bone, "body", r5(oe), True, tone, rect(u0, v0, u1, v1), region, material, hatch))
        if "f" in views:
            self.front.append(shape(sid, bone, "body", r5(of), True, tone, rect(z0, y0, z1, y1), region, material, hatch))
        if "p" in views:
            self.plan.append(shape(sid, bone, "body", r5(op), True, tone, rect(x0, z0, x1, z1), region, material, hatch))
        if body:
            extra = {"sheet": True} if sheet else {}
            self.body.append(box(sid, bone, material, "body", tone, size, [cu, cv, (z0 + z1) / 2 - oz], **extra))
            self.th(sid, region)
            if struct:
                self.st(sid, member or bone, struct)
