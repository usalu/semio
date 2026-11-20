"""🧰 Round 3 part writer of `furniture/folding-deck-chair`: every part is stated once (bone, side profile, lateral spans) and written as its 3D solid, its thermal and structural references and its true orthographic 2D shapes of the three viewpoints (ruling E1)."""
from r3_folding_deck_chair_geom import *


class Scale:
    """📒 The lists of one scale: `body`, `thermal`, `structure` solids and the `elevation`, `front`, `plan` shapes."""

    def __init__(self, denominator, pen_body, pen_small, pen_sheet):
        self.denominator, self.pen_body, self.pen_small, self.pen_sheet = denominator, pen_body, pen_small, pen_sheet
        self.body, self.thermal, self.structure, self.decisions = [], [], [], []
        self.elevation, self.front, self.plan = [], [], []

    def shape(self, view, ident, bone, role, z, tone, region, material, geometry, pen, span=None):
        s = {"id": ident, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "region": region, "material": material}
        s.update(geometry)
        if span is not None:
            s["solid"] = {"kind": "extruded", "sweep": "w", "z0": rnd(min(span)), "z1": rnd(max(span))}
        s["penMm"] = pen
        getattr(self, view).append(s)

    def drawn(self, ident, bone, geometry, spans, material, region, zs, tone2d, small=False, role=("body", "body", "body"), elevation=True):
        pen = self.pen_small if small else self.pen_body
        if elevation:
            self.shape("elevation", ident, bone, role[0], zs[0], tone2d[0], region, material, geometry, pen)
        for view, z, tone, r in (("front", zs[1], tone2d[1], role[1]), ("plan", zs[2], tone2d[2], role[2])):
            for i, span in enumerate(spans):
                self.shape(view, ident if len(spans) == 1 else "%s-%d" % (ident, i), bone, r, z, tone, region, material, geometry, pen, span)

    def member(self, ident, of, member, part="member"):
        self.structure.append({"id": ident, "of": of, "member": member, "part": part})

    def prism(self, ident, bone, profile, spans, material, tone, region, zs, tone2d, geometry=None, small=False, member=None, elevation=True, role=("body", "body", "body")):
        """🪵 A part of constant side profile between lateral faces: one extrude (mirrored by `repeat` for a pair of rails), its thermal reference, optionally its structural member, and its shapes."""
        z0, z1 = spans[0]
        solid = {"id": ident, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": pts(profile), "plane": "xy", "from": rnd(z0), "to": rnd(z1)}
        if len(spans) == 2:
            solid["repeat"] = {"count": 2, "translation": [0.0, 0.0, rnd(spans[1][0] - z0)]}
        self.body.append(solid)
        self.thermal.append({"id": "th-" + ident, "of": ident, "region": region})
        if member:
            self.member("m-" + ident, ident, member)
        self.drawn(ident, bone, geometry or {"kind": "poly", "points": pts(profile), "closed": True}, spans, material, region, zs, tone2d, small, role, elevation)

    def turned(self, ident, bone, centre, profile, z_start, sign, material, tone, region, zs, tone2d, radius2d, member=None, part="member"):
        """🔩 A turned part about a lateral axis through `centre`: one revolve of an [axial, radial] profile starting at `z_start` and running towards `sign` z."""
        self.body.append({"id": ident, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": pts(profile), "axis": [0.0, 0.0, float(sign)], "origin": [rnd(centre[0]), rnd(centre[1]), rnd(z_start)]})
        self.thermal.append({"id": "th-" + ident, "of": ident, "region": region})
        if member:
            self.member(("m-" if part == "member" else "j-") + ident, ident, member, part)
        geometry = {"kind": "ellipse", "cx": rnd(centre[0]), "cy": rnd(centre[1]), "rx": radius2d, "ry": radius2d}
        span = (z_start, z_start + sign * profile[-1][0])
        return geometry, span

    def sheet(self, ident, bone, profile, half, stripes, region, zs, tones, tone2d, elevation_role="detail"):
        """🏳️ A canvas sheet of one side profile across the sling width: alternating light and dark stripes as two repeated extrudes (or one plain extrude), and its shapes with the dark stripes as texture."""
        if stripes:
            w = 2 * half / stripes
            for name, start, count, tone in (("light", -half, (stripes + 1) // 2, tones[0]), ("dark", -half + w, stripes // 2, tones[1])):
                self.body.append({"id": "%s-%s" % (ident, name), "kind": "extrude", "bone": bone, "material": CA, "role": "body", "tone": tone, "profile": pts(profile), "plane": "xy", "from": rnd(start), "to": rnd(start + w), "repeat": {"count": count, "translation": [0.0, 0.0, rnd(2 * w)]}})
                self.thermal.append({"id": "th-%s-%s" % (ident, name), "of": "%s-%s" % (ident, name), "region": region})
        else:
            self.body.append({"id": ident, "kind": "extrude", "bone": bone, "material": CA, "role": "body", "tone": tones[0], "profile": pts(profile), "plane": "xy", "from": rnd(-half), "to": rnd(half)})
            self.thermal.append({"id": "th-" + ident, "of": ident, "region": region})
        geometry = {"kind": "poly", "points": pts(profile), "closed": True}
        self.shape("elevation", ident, bone, elevation_role, zs[0], tone2d[0], region, CA, geometry, self.pen_sheet)
        for view, z, tone in (("front", zs[1], tone2d[1]), ("plan", zs[2], tone2d[2])):
            self.shape(view, ident, bone, "body", z, tone, region, CA, geometry, self.pen_sheet, (-half, half))
            if stripes:
                w = 2 * half / stripes
                for i in range(stripes // 2):
                    z0 = -half + w * (2 * i + 1)
                    self.shape(view, "%s-stripe-%d" % (ident, i), bone, "texture", z + 0.05, tones[1], region, CA, geometry, self.pen_sheet, (z0, z0 + w))

    def own(self, ident, bone, material, geometry, member, part, transform=None, repeat=None, anchor=None, slender=False):
        """🧱 A solid that exists only in the structure list (joint sleeve, mass, support pad, sling member)."""
        solid = {"id": ident, "bone": bone, "material": material, "role": "ground" if part == "support" else "detail", "tone": 0.3}
        solid.update(geometry)
        if transform:
            solid["transform"] = transform
        if repeat:
            solid["repeat"] = repeat
        if slender:
            solid["slender"] = True
        solid["member"], solid["part"] = member, part
        if anchor:
            solid["anchor"] = anchor
        self.structure.append(solid)

    def sleeve_joint(self, ident, bone, centre, radius, span, member):
        """⚙️ A joint drawn as a short sleeve about a lateral axis, mirrored to the other side."""
        z0, z1 = span
        self.own(ident, bone, SH, {"kind": "revolve", "profile": [[0.0, radius], [rnd(z1 - z0), radius]], "axis": [0.0, 0.0, 1.0], "origin": [rnd(centre[0]), rnd(centre[1]), rnd(z0)]}, member, "joint", repeat={"count": 2, "translation": [0.0, 0.0, rnd(-(z0 + z1))]})

    def rod_joint(self, ident, bone, centre, radius, half, member):
        self.own(ident, bone, SH, {"kind": "revolve", "profile": [[0.0, radius], [rnd(2 * half), radius]], "axis": [0.0, 0.0, 1.0], "origin": [rnd(centre[0]), rnd(centre[1]), rnd(-half)]}, member, "joint", slender=2 * half / (2 * radius) >= 20)

    def mass(self, ident, bone, centre, radii, span):
        """⚖️ The mass of a frame as two half masses inside its two rails at the centre of mass of the frame."""
        z = (span[0] + span[1]) / 2
        self.own(ident, bone, BE, {"kind": "ellipsoid", "radii": list(radii)}, bone, "mass", transform={"translation": [rnd(centre[0]), rnd(centre[1]), rnd(z)]}, repeat={"count": 2, "translation": [0.0, 0.0, rnd(-2 * z)]})

    def support(self, anchor, bone, z, length, height, width):
        """🟩 The contact pad under one foot: level with the ground at rest, its underside on y = 0 below the foot round."""
        _, deg = REST[bone]
        u, v = foot_anchor(bone)
        lift = local((0.0, 0.0), deg, (0.0, height / 2))
        self.own("support-" + anchor, bone, SH, {"kind": "box", "size": [length, height, width]}, bone, "support", transform={"translation": [rnd(u + lift[0]), rnd(v + lift[1]), rnd(z)], "rotation": quaternion_z(-deg)}, anchor=anchor)

    def decide(self, en, de):
        self.decisions.append({"en": en, "de": de})

    def volume(self):
        return {"status": "authored", "denominator": self.denominator, "body": self.body, "thermal": self.thermal, "structure": self.structure, "decisions": self.decisions}


def pairs(span):
    return [span, (-span[1], -span[0])]


def rect(u0, u1, v0, v1):
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]
