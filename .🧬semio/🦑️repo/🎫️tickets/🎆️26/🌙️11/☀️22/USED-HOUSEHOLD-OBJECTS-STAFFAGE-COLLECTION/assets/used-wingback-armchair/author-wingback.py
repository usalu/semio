"""🪑 Authoritative author script of the staffage asset `used-wingback-armchair`: states every hand-chosen number and writes `🔣️.json` next to it.

Run: `python3 author-wingback.py` (standard library only). See ../../brief.md and the report ../../reports/used-wingback-armchair.md.
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
TB = 0.012


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r5(v):
    """🎯 Rounds a number (or nested list) to 5 decimals."""
    if isinstance(v, (list, tuple)):
        return [r5(x) for x in v]
    return round(float(v), 5) + 0.0


BONES = [
    dict(id="ground", parent=None, title=T("Floor reference", "Bodenbezug"), x=0, y=0, angleDeg=0, lengthM=0, joint={"type": "fixed"}),
    dict(id="blanket", parent="ground", title=T("Folded moving blanket (stays behind)", "Gefaltete Möbeldecke (bleibt liegen)"), x=-0.05, y=TB / 2, angleDeg=0, lengthM=1.5, joint={"type": "prismatic", "axisDeg": 0, "minM": -1.65, "maxM": 0}),
    dict(id="lift", parent="ground", title=T("Lift of the armchair off the blanket", "Anheben des Sessels von der Decke"), x=0.0645, y=TB, angleDeg=0, lengthM=0, joint={"type": "prismatic", "axisDeg": 90, "minM": -TB, "maxM": 1.0}),
    dict(id="chair", parent="lift", title=T("Tipping pivot at the rear edge of the rear feet", "Kipppunkt an der Hinterkante der Hinterfüße"), x=0, y=0, angleDeg=0, lengthM=0, joint={"type": "revolute", "minDeg": -10, "maxDeg": 90}),
    dict(id="frame", parent="chair", title=T("Seat frame with arms: beech rails, arm posts and arm rails, padded and covered", "Sitzrahmen mit Armlehnen: Buchenzargen, Armpfosten und Armholme, gepolstert und bezogen"), x=0.0355, y=0.15, angleDeg=0, lengthM=0.7, joint={"type": "fixed"}),
    dict(id="leg-rear", parent="frame", title=T("Rear legs (pair), turned beech, raked back 10 degrees", "Hinterbeine (Paar), gedrechselte Buche, 10 Grad nach hinten ausgestellt"), x=0.00645, y=0, angleDeg=-100, lengthM=0.15232, joint={"type": "fixed"}),
    dict(id="leg-front", parent="frame", title=T("Front legs (pair), turned beech", "Vorderbeine (Paar), gedrechselte Buche"), x=0.64, y=0, angleDeg=-90, lengthM=0.15, joint={"type": "fixed"}),
    dict(id="back", parent="frame", title=T("Back with wings: beech posts, top rail and plywood wing boards, reclined 6 degrees", "Rücken mit Ohren: Buchenpfosten, Kopfholm und Sperrholz-Ohrenbretter, 6 Grad geneigt"), x=0.02, y=0, angleDeg=96, lengthM=0.955, joint={"type": "fixed"}),
    dict(id="seat", parent="frame", title=T("Sprung seat: Bonnell spring unit under coir and velour, sagged", "Federkernsitz: Bonnell-Federkern unter Kokos und Velours, durchgesessen"), x=0.14, y=0.15, angleDeg=0, lengthM=0.55, joint={"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 0}),
    dict(id="stool", parent="ground", title=T("Matching footstool", "Passender Fußhocker"), x=0.9, y=TB, angleDeg=0, lengthM=0.4, joint={"type": "prismatic", "axisDeg": 90, "minM": -TB, "maxM": 0}),
]
BONE = {b["id"]: b for b in BONES}


def frame_of(bone_id):
    """🧭 Rest frame (origin, angle in degrees) of a bone in world metres."""
    b = BONE[bone_id]
    if b["parent"] is None:
        return (b["x"], b["y"]), b["angleDeg"]
    (px, py), pa = frame_of(b["parent"])
    c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
    return (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"]), pa + b["angleDeg"]


def to_local(bone_id, pts):
    """📐 World rest points (x, y) to bone-local (u, v)."""
    (ox, oy), a = frame_of(bone_id)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [r5([c * (x - ox) + s * (y - oy), -s * (x - ox) + c * (y - oy)]) for x, y in pts]


def to_world(bone_id, pts):
    """🌍 Bone-local (u, v) points to world rest (x, y)."""
    (ox, oy), a = frame_of(bone_id)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [(ox + c * u - s * v, oy + s * u + c * v) for u, v in pts]


def C(pts):
    """🪑 Chair-local points (floor of the chair = 0) to world rest points on the blanket."""
    return [(x, y + TB) for x, y in pts]


def shape(sid, bone, kind_fields, role, z, tone, region=None, material=None, filled=True, solid=None, hatch=None, pen=None, regions=None):
    """✏️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": round(z, 3), "filled": filled, "tone": tone}
    s.update(kind_fields)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if solid:
        s["solid"] = solid
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    return s


def poly(pts, closed=True):
    """🔷 Poly geometry."""
    return {"kind": "poly", "points": [r5(p) for p in pts], "closed": closed}


class Scale:
    """🗂️ Collects body solids and 2D shapes of one scale."""

    def __init__(self, den):
        self.den = den
        self.body = []
        self.views = {"elevation": [], "front": [], "plan": []}
        self.regions = {}

    def add_view(self, view, s):
        self.views[view].append(s)

    def xy(self, pid, bone, world_profile, z0, z1, mat, tone, region=None, role="body", sheet=False, texture=False, hatch=None, pen=None, views=("elevation", "front", "plan"), body=True, regions=None):
        """🧱 A part extruded across the lateral axis from its side profile: body extrude (plane xy) and the same profile as a `solid` shape in every view."""
        prof = to_local(bone, world_profile)
        if body:
            sol = {"id": pid, "kind": "extrude", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": prof, "plane": "xy", "from": r5(z0), "to": r5(z1)}
            if sheet:
                sol["sheet"] = True
            if texture:
                sol["texture"] = True
            self.body.append(sol)
            if region:
                self.regions[pid] = region
        zc = (z0 + z1) / 2
        xs = [p[0] for p in world_profile]
        ys = [p[1] for p in world_profile]
        order = {"elevation": 5 - 5 * zc, "front": 2 + 2 * max(xs), "plan": 2 + 2 * max(ys)}
        for v in views:
            self.add_view(v, shape(pid, bone, poly(prof), role, order[v], tone, region, mat, True, {"kind": "extruded", "sweep": "w", "z0": r5(z0), "z1": r5(z1)}, hatch if v == "plan" else None, pen, regions))

    def revolve(self, pid, bone, profile, axis, axis_x, axis_z, mat, tone, region=None, role="body", views=("elevation", "front", "plan"), body=True, regions=None, pen=None):
        """🌀 A turned part about the bone axis `u` or the bone normal `v` (offset `axis_x` along world x): body revolve and the outline as a revolved `solid` shape."""
        prof = [r5(p) for p in profile]
        if body:
            if axis == "u":
                sol = {"id": pid, "kind": "revolve", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": prof, "axis": [1, 0, 0], "origin": r5([0, 0, axis_z])}
            else:
                sol = {"id": pid, "kind": "revolve", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": prof, "axis": [0, 1, 0], "origin": r5([axis_x, 0, axis_z])}
            self.body.append(sol)
            if region:
                self.regions[pid] = region
        pts = [p for p in prof if p[1] > 0]
        if axis == "u":
            outline = [(a, r) for a, r in pts] + [(a, -r) for a, r in reversed(pts)]
        else:
            outline = [(axis_x + r, a) for a, r in pts] + [(axis_x - r, a) for a, r in reversed(pts)]
        (ox, oy), ang = frame_of(bone)
        wpts = to_world(bone, outline)
        order = {"elevation": 5 - 5 * axis_z, "front": 2 + 2 * max(p[0] for p in wpts), "plan": 2 + 2 * axis_z}
        sol2 = {"kind": "revolved", "axisBone": bone, "axis": axis, "axisX": r5(axis_x if axis == "v" else 0), "axisZ": r5(axis_z), "mirror": True}
        for v in views:
            self.add_view(v, shape(pid, bone, poly(outline), role, order[v], tone, region, mat, True, sol2, None, pen, regions))

    def roll(self, pid, bone, xw0, xw1, yw, r, half_w, zc, mat, tone, region, views=("elevation", "front", "plan")):
        """🥖 A padded arm roll: a capsule along the forward axis, its round section widened laterally to `half_w`; elevation stadium, front ellipse, plan rounded rectangle."""
        (u0, v0), (u1, v1) = to_local(bone, [(xw0, yw), (xw1, yw)])
        k = half_w / r
        self.body.append({"id": pid, "kind": "capsule", "bone": bone, "material": mat, "role": "body", "tone": tone, "p0": r5([u0, v0, 0]), "p1": r5([u1, v1, 0]), "r0": r5(r), "r1": r5(r), "transform": {"scale": r5([1, 1, k]), "translation": r5([0, 0, zc])}})
        self.regions[pid] = region
        if "elevation" in views:
            self.add_view("elevation", shape(pid, bone, {"kind": "capsule", "x0": r5(u0), "y0": r5(v0), "x1": r5(u1), "y1": r5(v1), "r0": r5(r), "r1": r5(r)}, "body", 5 - 5 * zc, tone, region, mat))
        if "front" in views:
            self.add_view("front", shape(pid, bone, {"kind": "ellipse", "cx": r5(zc), "cy": r5(yw), "rx": r5(half_w), "ry": r5(r)}, "body", 2 + 2 * (xw1 + r), tone, region, mat))
        if "plan" in views:
            self.add_view("plan", shape(pid, bone, {"kind": "rect", "x": r5(xw0 - r), "y": r5(zc - half_w), "w": r5(xw1 - xw0 + 2 * r), "h": r5(2 * half_w), "r": r5(min(r, half_w) * 0.95)}, "body", 2 + 2 * (yw + r), tone, region, mat))

    def stroke(self, view, sid, bone, world_pts, role, tone, closed=False, local=False, z=9):
        """〰️ A detail or texture stroke (elevation in bone-local, front and plan in world rest coordinates)."""
        pts = world_pts if local else (to_local(bone, world_pts) if view == "elevation" else world_pts)
        self.add_view(view, shape(sid, bone, poly(pts, closed), role, z, tone, filled=False))

    def mark(self, view, sid, bone, kind_fields, role, tone, z=9):
        """⭕ A filled detail mark."""
        self.add_view(view, shape(sid, bone, kind_fields, role, z, tone))


V = "velour-cotton"
VF = "velour-faded"
BEECH = "beech-stained"


def chair_xz(points):
    """🔁 Chair-local (x, y) to world for side profiles."""
    return C(points)


BACK_PROFILE_50 = [(0.11, 0.15), (0.0784, 0.42), (0.0434, 0.72), (0.005, 1.05), (0.008, 1.075), (0.025, 1.093), (0.05, 1.10), (0.08, 1.098), (0.11, 1.088), (0.135, 1.07), (0.15, 1.045), (0.16, 1.0), (0.175, 0.9), (0.19, 0.8), (0.205, 0.7), (0.225, 0.6), (0.243, 0.5), (0.25, 0.45), (0.246, 0.38), (0.232, 0.3), (0.205, 0.15)]
WING_PROFILE_50 = [(0.0625, 0.555), (0.31, 0.555), (0.34, 0.6), (0.368, 0.68), (0.383, 0.77), (0.379, 0.86), (0.356, 0.94), (0.305, 1.005), (0.225, 1.052), (0.135, 1.074), (0.045, 1.08), (0.012, 1.0)]
ARM_PROFILE_50 = [(0.11, 0.15), (0.80, 0.15), (0.809, 0.2), (0.811, 0.5), (0.808, 0.56), (0.0625, 0.56)]
BOX_PROFILE_50 = [(0.11, 0.15), (0.795, 0.15), (0.806, 0.19), (0.809, 0.245), (0.802, 0.3), (0.13, 0.3)]
SEAT_PROFILE_50 = [(0.22, 0.295), (0.80, 0.295), (0.811, 0.33), (0.81, 0.39), (0.796, 0.424), (0.766, 0.44), (0.70, 0.436), (0.58, 0.425), (0.46, 0.418), (0.36, 0.422), (0.29, 0.432), (0.25, 0.443), (0.22, 0.443)]
STOOL_PAD_50 = [(0.904, 0.268), (1.296, 0.268), (1.30, 0.33), (1.291, 0.384), (1.262, 0.398), (1.18, 0.394), (1.10, 0.389), (1.02, 0.394), (0.938, 0.398), (0.909, 0.384), (0.90, 0.33)]
STOOL_BOX_50 = [(0.90, 0.16), (1.30, 0.16), (1.30, 0.27), (0.90, 0.27)]
BLANKET_50 = [(0, -0.006), (1.494, -0.006), (1.4985, -0.0045), (1.50, 0.0), (1.4985, 0.0045), (1.494, 0.006), (0.0, 0.006)]
LEG_50 = [(0, 0), (0, 0.023), (0.007, 0.023), (0.011, 0.0205), (0.015, 0.0235), (0.02, 0.022), (0.128, 0.0155), (0.128, 0)]
FERRULE_50 = [(0.128, 0), (0.128, 0.0155), (0.146, 0.0155), (0.15, 0.0145), (0.15, 0)]
GLIDE_50 = [(0.128, 0), (0.128, 0.0158), (0.14, 0.0165), (0.147, 0.016), (0.15, 0.0145), (0.15, 0)]
STOOL_LEG_50 = [(0, 0), (0, 0.0135), (0.01, 0.0135), (0.014, 0.015), (0.128, 0.019), (0.136, 0.0215), (0.144, 0.019), (0.16, 0.0195), (0.16, 0)]


def scaled_leg(profile, length):
    """📏 Stretches a 0.15 m leg profile to a leg bone of `length`."""
    k = length / 0.15
    return [(a * k, r) for a, r in profile]


def blanket_part(s, profile_local, sheet, hatch, z_pen=None, front_pen=None):
    """🧺 The folded quilted moving blanket under chair and stool."""
    wp = to_world("blanket", profile_local)
    s.xy("blanket", "blanket", wp, -0.5, 0.5, "moving-blanket", 0.78, "blanket", role="body", sheet=sheet, hatch=hatch, pen=z_pen)


def legs_chair(s, prof_front, prof_rear, ferrule=None, glide=None, ferrule_rear=None):
    """🦵 Four turned beech legs with their brass ferrules (one replaced by a plastic glide)."""
    L = BONE["leg-rear"]["lengthM"]
    for side, z in (("l", 0.33), ("r", -0.33)):
        s.revolve(f"leg-front-{side}", "leg-front", prof_front, "u", 0, z, BEECH, 0.28, "legs-beech")
        s.revolve(f"leg-rear-{side}", "leg-rear", scaled_leg(prof_rear, L) if prof_rear is not None else prof_rear, "u", 0, z, BEECH, 0.28, "legs-beech")
    if ferrule is not None:
        s.revolve("ferrule-front-l", "leg-front", ferrule, "u", 0, 0.33, "brass-aged", 0.62, role="detail")
        s.revolve("glide-front-r", "leg-front", glide, "u", 0, -0.33, "pe-glide", 0.12, role="detail")
        for side, z in (("l", 0.33), ("r", -0.33)):
            s.revolve(f"ferrule-rear-{side}", "leg-rear", scaled_leg(ferrule, L), "u", 0, z, "brass-aged", 0.62, role="detail")


def stool_legs(s, prof, tone=0.28):
    """🦶 Four turned beech legs of the footstool."""
    for xi, xl in (("rear", 0.04), ("front", 0.36)):
        for side, z in (("l", 0.21), ("r", -0.21)):
            s.revolve(f"stool-leg-{xi}-{side}", "stool", prof, "v", xl, z, BEECH, tone, "legs-beech")


def build_50():
    """🔍 1:50: the full construction for 7.5 mm."""
    s = Scale(50)
    blanket_part(s, BLANKET_50, True, "quilting")
    legs_chair(s, LEG_50, LEG_50, FERRULE_50, GLIDE_50)
    s.xy("seat-box", "frame", C(BOX_PROFILE_50), -0.265, 0.265, V, 0.42, "cover-seat")
    s.xy("arm-l", "frame", C(ARM_PROFILE_50), 0.265, 0.385, VF, 0.62, "cover-faded")
    s.xy("arm-r", "frame", C(ARM_PROFILE_50), -0.385, -0.265, V, 0.46, "cover-body")
    s.roll("roll-l", "frame", 0.115, 0.80, 0.575 + TB, 0.05, 0.065, 0.325, VF, 0.66, "cover-faded")
    s.roll("roll-r", "frame", 0.115, 0.80, 0.575 + TB, 0.05, 0.065, -0.325, V, 0.5, "cover-body")
    s.xy("back", "back", C(BACK_PROFILE_50), -0.30, 0.30, V, 0.4, "cover-body")
    s.xy("wing-l", "back", C(WING_PROFILE_50), 0.30, 0.385, VF, 0.6, "cover-faded")
    s.xy("wing-r", "back", C(WING_PROFILE_50), -0.385, -0.30, V, 0.44, "cover-body")
    s.xy("seat", "seat", C(SEAT_PROFILE_50), -0.265, 0.265, V, 0.52, "cover-seat")
    s.body.append({"id": "piping-seat", "kind": "capsule", "bone": "seat", "material": V, "role": "detail", "tone": 0.36, "slender": True, "p0": r5(to_local("seat", C([(0.789, 0.433)]))[0] + [-0.262]), "p1": r5(to_local("seat", C([(0.789, 0.433)]))[0] + [0.262]), "r0": 0.0045, "r1": 0.0045})
    s.regions["piping-seat"] = "cover-seat"
    s.xy("stool-box", "stool", C(STOOL_BOX_50), -0.25, 0.25, V, 0.44, "stool-cover")
    s.xy("stool-pad", "stool", C(STOOL_PAD_50), -0.248, 0.248, V, 0.54, "stool-cover")
    stool_legs(s, STOOL_LEG_50)
    tag = [(0.8295, 0.445), (0.8305, 0.445), (0.8305, 0.537), (0.8295, 0.537)]
    s.xy("price-tag", "frame", C(tag), -0.352, -0.298, "cardboard", 0.94, None, role="detail", sheet=True)
    piping = to_local("seat", C([(0.789, 0.433)]))[0]
    s.mark("elevation", "piping-seat", "seat", {"kind": "ellipse", "cx": piping[0], "cy": piping[1], "rx": 0.0045, "ry": 0.0045}, "detail", 0.36, z=9.5)
    s.mark("front", "piping-seat", "seat", {"kind": "rect", "x": -0.262, "y": r5(0.433 + TB - 0.0045), "w": 0.524, "h": 0.009}, "detail", 0.36, z=9.5)
    s.stroke("elevation", "seam-arm-r", "frame", C([(0.12, 0.535), (0.79, 0.535)]), "detail", 0.3)
    s.stroke("elevation", "wear-roll-r", "frame", C([(0.70, 0.612), (0.77, 0.618), (0.83, 0.6)]), "texture", 0.3)
    s.stroke("elevation", "wear-roll-r2", "frame", C([(0.74, 0.598), (0.81, 0.592)]), "texture", 0.3)
    s.stroke("elevation", "piping-wing-r", "back", C(WING_PROFILE_50[1:9]), "detail", 0.3)
    s.stroke("elevation", "sag-crease", "seat", C([(0.30, 0.425), (0.46, 0.414), (0.62, 0.422)]), "texture", 0.3)
    s.stroke("elevation", "scuff-front-r", "leg-front", C([(0.733, 0.03), (0.738, 0.05)]), "texture", 0.3)
    s.stroke("elevation", "scuff-rear-r", "leg-rear", C([(0.083, 0.06), (0.089, 0.085)]), "texture", 0.3)
    s.stroke("elevation", "stool-sag", "stool", C([(0.95, 0.392), (1.10, 0.385), (1.25, 0.392)]), "texture", 0.3)
    s.stroke("front", "seam-arm-l", "frame", C([(0.271, 0.535), (0.379, 0.535)]), "detail", 0.3)
    s.stroke("front", "seam-arm-r", "frame", C([(-0.379, 0.535), (-0.271, 0.535)]), "detail", 0.3)
    s.stroke("front", "wear-front-r", "frame", C([(-0.37, 0.6), (-0.33, 0.612), (-0.29, 0.6)]), "texture", 0.3)
    s.stroke("front", "seam-box", "frame", C([(-0.255, 0.3), (0.255, 0.3)]), "detail", 0.3)
    s.stroke("front", "stool-seam", "stool", C([(-0.24, 0.27), (0.24, 0.27)]), "detail", 0.3)
    s.stroke("front", "tag-string", "frame", C([(-0.34, 0.53), (-0.325, 0.528), (-0.31, 0.53)]), "detail", 0.3)
    s.stroke("plan", "stain-seat", "seat", [(0.52, 0.06), (0.55, 0.085), (0.585, 0.075), (0.59, 0.045), (0.56, 0.03), (0.525, 0.04)], "texture", 0.3, closed=True)
    s.stroke("plan", "wear-roll-r", "frame", [(0.60, -0.30), (0.75, -0.31), (0.82, -0.325)], "texture", 0.3)
    s.stroke("plan", "wear-roll-l", "frame", [(0.62, 0.30), (0.80, 0.32)], "texture", 0.3)
    s.stroke("plan", "seat-front-seam", "seat", [(0.79, -0.26), (0.79, 0.26)], "detail", 0.3)
    s.stroke("plan", "stool-sag", "stool", [(0.98, -0.18), (1.10, -0.14), (1.22, -0.18)], "texture", 0.3)
    return s


def build_100():
    """🔎 1:100: the same parts for 15 mm; piping, price tag, ferrules and wear strokes go."""
    s = Scale(100)
    blanket_part(s, [(0, -0.006), (1.495, -0.006), (1.50, 0.0), (1.495, 0.006), (0, 0.006)], True, "quilting-100")
    leg = [(0, 0), (0, 0.023), (0.018, 0.022), (0.15, 0.0155), (0.15, 0)]
    legs_chair(s, leg, leg)
    s.xy("seat-box", "frame", C([(0.11, 0.15), (0.797, 0.15), (0.808, 0.22), (0.802, 0.3), (0.13, 0.3)]), -0.265, 0.265, V, 0.42, "cover-seat")
    arm = [(0.11, 0.15), (0.80, 0.15), (0.81, 0.21), (0.81, 0.56), (0.0625, 0.56)]
    s.xy("arm-l", "frame", C(arm), 0.265, 0.385, VF, 0.62, "cover-faded")
    s.xy("arm-r", "frame", C(arm), -0.385, -0.265, V, 0.46, "cover-body")
    s.roll("roll-l", "frame", 0.115, 0.80, 0.575 + TB, 0.05, 0.065, 0.325, VF, 0.66, "cover-faded")
    s.roll("roll-r", "frame", 0.115, 0.80, 0.575 + TB, 0.05, 0.065, -0.325, V, 0.5, "cover-body")
    back = [(0.11, 0.15), (0.005, 1.05), (0.02, 1.09), (0.065, 1.10), (0.115, 1.087), (0.148, 1.05), (0.165, 0.96), (0.19, 0.8), (0.222, 0.62), (0.248, 0.47), (0.24, 0.35), (0.205, 0.15)]
    s.xy("back", "back", C(back), -0.30, 0.30, V, 0.4, "cover-body")
    wing = [(0.0625, 0.555), (0.31, 0.555), (0.355, 0.63), (0.382, 0.76), (0.372, 0.89), (0.31, 1.0), (0.2, 1.058), (0.07, 1.079), (0.012, 1.0)]
    s.xy("wing-l", "back", C(wing), 0.30, 0.385, VF, 0.6, "cover-faded")
    s.xy("wing-r", "back", C(wing), -0.385, -0.30, V, 0.44, "cover-body")
    seat = [(0.22, 0.295), (0.80, 0.295), (0.812, 0.36), (0.797, 0.425), (0.765, 0.44), (0.62, 0.428), (0.46, 0.418), (0.32, 0.427), (0.25, 0.443), (0.22, 0.443)]
    s.xy("seat", "seat", C(seat), -0.265, 0.265, V, 0.52, "cover-seat")
    s.xy("stool-box", "stool", C([(0.90, 0.16), (1.30, 0.16), (1.30, 0.268), (0.90, 0.268)]), -0.25, 0.25, V, 0.44, "stool-cover")
    s.xy("stool-pad", "stool", C([(0.903, 0.266), (1.297, 0.266), (1.30, 0.34), (1.285, 0.392), (1.245, 0.398), (1.10, 0.389), (0.955, 0.398), (0.915, 0.392), (0.90, 0.34)]), -0.248, 0.248, V, 0.54, "stool-cover")
    stool_legs(s, [(0, 0), (0, 0.0135), (0.13, 0.019), (0.16, 0.02), (0.16, 0)])
    s.stroke("elevation", "seam-arm-r", "frame", C([(0.13, 0.535), (0.78, 0.535)]), "detail", 0.3)
    s.stroke("front", "seam-box", "frame", C([(-0.25, 0.3), (0.25, 0.3)]), "detail", 0.3)
    s.stroke("plan", "seat-front-seam", "seat", [(0.785, -0.255), (0.785, 0.255)], "detail", 0.3)
    return s


def build_200():
    """🗺️ 1:200: 30 mm; arm panel and roll are one rounded arm, the seat dish and the stool pad crown are generalised, legs are plain tapers."""
    s = Scale(200)
    blanket_part(s, [(0, -0.006), (1.50, -0.006), (1.50, 0.006), (0, 0.006)], True, None)
    leg = [(0, 0), (0, 0.023), (0.15, 0.0165), (0.15, 0)]
    legs_chair(s, leg, leg)
    s.xy("seat-box", "frame", C([(0.11, 0.15), (0.80, 0.15), (0.808, 0.3), (0.13, 0.3)]), -0.26, 0.26, V, 0.42, "cover-seat")
    arm = [(0.11, 0.15), (0.81, 0.15), (0.82, 0.52), (0.845, 0.565), (0.835, 0.61), (0.80, 0.625), (0.115, 0.625), (0.0625, 0.56)]
    s.xy("arm-l", "frame", C(arm), 0.26, 0.39, VF, 0.62, "cover-faded")
    s.xy("arm-r", "frame", C(arm), -0.39, -0.26, V, 0.46, "cover-body")
    back = [(0.11, 0.15), (0.005, 1.05), (0.03, 1.095), (0.09, 1.10), (0.145, 1.06), (0.175, 0.9), (0.215, 0.65), (0.248, 0.46), (0.205, 0.15)]
    s.xy("back", "back", C(back), -0.30, 0.30, V, 0.4, "cover-body")
    wing = [(0.0625, 0.6), (0.33, 0.6), (0.378, 0.72), (0.375, 0.88), (0.30, 1.01), (0.14, 1.075), (0.012, 1.0)]
    s.xy("wing-l", "back", C(wing), 0.30, 0.39, VF, 0.6, "cover-faded")
    s.xy("wing-r", "back", C(wing), -0.39, -0.30, V, 0.44, "cover-body")
    s.xy("seat", "seat", C([(0.22, 0.295), (0.805, 0.295), (0.81, 0.4), (0.775, 0.44), (0.25, 0.44), (0.22, 0.44)]), -0.26, 0.26, V, 0.52, "cover-seat")
    s.xy("stool-body", "stool", C([(0.90, 0.16), (1.30, 0.16), (1.30, 0.36), (1.27, 0.398), (0.93, 0.398), (0.90, 0.36)]), -0.25, 0.25, V, 0.5, "stool-cover")
    stool_legs(s, [(0, 0), (0, 0.0165), (0.16, 0.02), (0.16, 0)])
    return s


def build_500():
    """🗾 1:500: 75 mm; legs are 75 mm blocks, the stool one block on four, the blanket a 12 mm sheet."""
    s = Scale(500)
    s.xy("blanket", "blanket", to_world("blanket", [(0, -0.006), (1.50, -0.006), (1.50, 0.006), (0, 0.006)]), -0.5, 0.5, "moving-blanket", 0.78, "blanket", sheet=True, pen=0.2)
    L = BONE["leg-rear"]["lengthM"]
    for side, z in (("l", 0.33), ("r", -0.33)):
        s.xy(f"leg-front-{side}", "leg-front", to_world("leg-front", [(0, -0.0375), (0.15, -0.0375), (0.15, 0.0375), (0, 0.0375)]), z - 0.0375, z + 0.0375, BEECH, 0.28, "legs-beech")
        s.xy(f"leg-rear-{side}", "leg-rear", to_world("leg-rear", [(0, -0.0375), (L, -0.0375), (L, 0.0375), (0, 0.0375)]), z - 0.0375, z + 0.0375, BEECH, 0.28, "legs-beech")
    s.xy("seat-box", "frame", C([(0.11, 0.15), (0.81, 0.15), (0.81, 0.3), (0.13, 0.3)]), -0.26, 0.26, V, 0.42, "cover-seat")
    arm = [(0.11, 0.15), (0.83, 0.15), (0.845, 0.6), (0.81, 0.625), (0.11, 0.625), (0.0625, 0.56)]
    s.xy("arm-l", "frame", C(arm), 0.26, 0.39, VF, 0.62, "cover-faded")
    s.xy("arm-r", "frame", C(arm), -0.39, -0.26, V, 0.46, "cover-body")
    s.xy("back", "back", C([(0.11, 0.15), (0.005, 1.05), (0.05, 1.10), (0.13, 1.08), (0.25, 0.45), (0.205, 0.15)]), -0.30, 0.30, V, 0.4, "cover-body")
    wing = [(0.0625, 0.6), (0.34, 0.6), (0.38, 0.82), (0.30, 1.01), (0.10, 1.08), (0.012, 1.0)]
    s.xy("wing-l", "back", C(wing), 0.30, 0.39, VF, 0.6, "cover-faded")
    s.xy("wing-r", "back", C(wing), -0.39, -0.30, V, 0.44, "cover-body")
    s.xy("seat", "seat", C([(0.22, 0.295), (0.81, 0.295), (0.80, 0.44), (0.22, 0.44)]), -0.26, 0.26, V, 0.52, "cover-seat")
    s.xy("stool-body", "stool", C([(0.90, 0.0), (1.30, 0.0), (1.30, 0.398), (0.90, 0.398)]), -0.25, 0.25, V, 0.5, "stool-cover")
    return s


def box(sid, bone, centre_world, size, mat, tone=0.3, role="body", extra=None, rotate=False):
    """📦 A box in bone-local coordinates centred on a world rest point (axes aligned with the bone)."""
    (u, v) = to_local(bone, [centre_world[:2]])[0]
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": r5(size), "transform": {"translation": r5([u, v, centre_world[2]])}}
    if extra:
        s.update(extra)
    return s


def structure(den, body):
    """🏗️ The structural idealisation of one scale: the hidden beech frame, spring unit and plywood wings as members, the legs and blanket as their own members, joints, supports and masses."""
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[den]
    slim = den >= 200
    out = []

    def mem(s, member, part="member", extra=None):
        s = dict(s)
        s["member"] = member
        s["part"] = part
        if extra:
            s.update(extra)
        out.append(s)

    sl = {"slender": True} if slim else {}
    rail_h = 0.10
    mem(box("rail-front", "frame", C([(0.78, 0.20)])[0] + (0,), [0.025, rail_h, 0.645], "beech-frame", extra=sl), "frame")
    mem(box("rail-back", "frame", C([(0.135, 0.20)])[0] + (0,), [0.025, rail_h, 0.645], "beech-frame", extra=sl), "frame")
    for side, z in (("l", 0.31), ("r", -0.31)):
        mem(box(f"rail-side-{side}", "frame", C([(0.455, 0.20)])[0] + (z,), [0.66, rail_h, 0.025], "beech-frame", extra=sl), "frame")
        mem(box(f"arm-post-{side}", "frame", C([(0.78, 0.365)])[0] + (z,), [0.04, 0.39, 0.025], "beech-frame", extra=sl), "frame")
        mem(box(f"arm-rail-{side}", "frame", C([(0.46, 0.57)])[0] + (0.325 if z > 0 else -0.325,), [0.66, 0.03 if den < 200 else 0.06, 0.06], "beech-frame", extra=sl), "frame")
    for side, z in (("l", 0.28), ("r", -0.28)):
        mem({"id": f"back-post-{side}", "kind": "box", "bone": "back", "material": "beech-frame", "role": "body", "tone": 0.3, "size": [0.93, 0.06, 0.03], "transform": {"translation": r5([0.465, -0.035, z])}, **sl}, "back")
    mem({"id": "back-top-rail", "kind": "box", "bone": "back", "material": "beech-frame", "role": "body", "tone": 0.3, "size": [0.04 if den < 200 else 0.06, 0.05, 0.70], "transform": {"translation": r5([0.905, -0.05, 0])}, **({"slender": True} if slim else {})}, "back")
    mem({"id": "back-bottom-rail", "kind": "box", "bone": "back", "material": "beech-frame", "role": "body", "tone": 0.3, "size": [0.05 if den < 200 else 0.06, 0.05, 0.56], "transform": {"translation": r5([0.06, -0.04, 0])}, **({"slender": True} if slim else {})}, "back")
    wing_board = [(0.08, 0.57), (0.30, 0.57), (0.355, 0.66), (0.365, 0.80), (0.33, 0.93), (0.25, 1.02), (0.10, 1.06), (0.035, 1.0)]
    for side, z in (("l", 0.341), ("r", -0.341)):
        mem({"id": f"wing-board-{side}", "kind": "extrude", "bone": "back", "material": "birch-plywood", "role": "body", "tone": 0.3, "profile": to_local("back", C(wing_board)), "plane": "xy", "from": r5(z - 0.006), "to": r5(z + 0.006), "sheet": True}, "back")
    sp = to_local("seat", C([(0.51, 0.325)]))[0]
    mem({"id": "spring-unit", "kind": "box", "bone": "seat", "material": "spring-steel", "role": "body", "tone": 0.3, "size": [0.52, 0.15, 0.50], "transform": {"translation": r5([sp[0], sp[1], 0])}}, "seat")
    for b in body:
        bid = b["id"]
        if bid in ("leg-front-l", "leg-front-r"):
            mem({"id": "m-" + bid, "of": bid}, "leg-front")
        if bid in ("leg-rear-l", "leg-rear-r"):
            mem({"id": "m-" + bid, "of": bid}, "leg-rear")
        if bid.startswith("stool-leg-"):
            mem({"id": "m-" + bid, "of": bid}, "stool")
        if bid == "blanket":
            mem({"id": "m-blanket", "of": "blanket"}, "blanket")
    if den < 200:
        for side, z in (("l", 0.2375), ("r", -0.2375)):
            mem(box(f"stool-rail-{side}", "stool", C([(1.10, 0.20)])[0] + (z,), [0.40, 0.08, 0.025], "beech-frame"), "stool")
        for xi, x in (("rear", 0.9125), ("front", 1.2875)):
            mem(box(f"stool-rail-{xi}", "stool", C([(x, 0.20)])[0] + (0,), [0.025, 0.08, 0.45], "beech-frame"), "stool")
        mem(box("stool-board", "stool", C([(1.10, 0.246)])[0] + (0,), [0.38, 0.012, 0.48], "birch-plywood", extra={"sheet": True} if den == 100 else None), "stool")
    else:
        mem(box("stool-frame", "stool", C([(1.10, 0.205)])[0] + (0,), [0.40, 0.09, 0.50], "beech-frame"), "stool")
    if den == 500:
        for xi, xl in (("rear", 0.04), ("front", 0.36)):
            for side, z in (("l", 0.21), ("r", -0.21)):
                mem(box(f"stool-leg-{xi}-{side}", "stool", C([(0.9 + xl, 0.08)])[0] + (z,), [0.075, 0.16, 0.075], BEECH), "stool")
    L = BONE["leg-rear"]["lengthM"]
    for side, z in (("l", 0.33), ("r", -0.33)):
        out.append({"id": f"joint-leg-front-{side}", "kind": "box", "bone": "leg-front", "material": "beech-frame", "role": "detail", "tone": 0.25, "size": r5([max(f, 0.012), max(f, 0.04), max(f, 0.04)]), "transform": {"translation": r5([0, 0, z])}, "member": "leg-front", "part": "joint"})
        out.append({"id": f"joint-leg-rear-{side}", "kind": "box", "bone": "leg-rear", "material": "beech-frame", "role": "detail", "tone": 0.25, "size": r5([max(f, 0.012), max(f, 0.04), max(f, 0.04)]), "transform": {"translation": r5([0, 0, z])}, "member": "leg-rear", "part": "joint"})
        out.append({"id": f"joint-back-{side}", "kind": "box", "bone": "back", "material": "beech-frame", "role": "detail", "tone": 0.25, "size": r5([max(f, 0.03), max(f, 0.06), max(f, 0.03)]), "transform": {"translation": r5([max(f, 0.03) / 2, -0.035, 0.28 if z > 0 else -0.28])}, "member": "back", "part": "joint"})
    for xi, x in (("rear", 0.27), ("front", 0.75)):
        for side, z in (("l", 0.24), ("r", -0.24)):
            jp = to_local("seat", C([(x, 0.25 + max(f, 0.02) / 2)]))[0]
            out.append({"id": f"joint-seat-{xi}-{side}", "kind": "box", "bone": "seat", "material": "spring-steel", "role": "detail", "tone": 0.25, "size": r5([max(f, 0.04), max(f, 0.02), max(f, 0.04)]), "transform": {"translation": r5([jp[0], jp[1], z])}, "member": "seat", "part": "joint"})
    for xi, xl in (("rear", 0.04), ("front", 0.36)):
        for side, z in (("l", 0.21), ("r", -0.21)):
            out.append({"id": f"joint-stool-{xi}-{side}", "kind": "box", "bone": "stool", "material": "beech-frame", "role": "detail", "tone": 0.25, "size": r5([max(f, 0.04), max(f, 0.012), max(f, 0.04)]), "transform": {"translation": r5([xl, 0.16 - max(f, 0.012) / 2, z])}, "member": "stool", "part": "joint"})
    sh = max(f, 0.0075)
    for xi, xl in (("rear", 0.04), ("front", 0.36)):
        for side, z in (("l", 0.21), ("r", -0.21)):
            out.append({"id": f"support-stool-{xi}-{side}", "kind": "box", "bone": "stool", "material": "beech-frame", "role": "ground", "tone": 0.2, "size": r5([max(f, 0.03), sh, max(f, 0.03)]), "transform": {"translation": r5([xl, sh / 2, z])}, "member": "stool", "part": "support", "anchor": f"stool-foot-{xi}-{side}"})
    for ai, u in (("a", 0.375), ("b", 1.125)):
        out.append({"id": f"support-blanket-{ai}", "kind": "box", "bone": "blanket", "material": "moving-blanket", "role": "ground", "tone": 0.2, "size": [0.3, 0.006, 0.3], "sheet": True, "transform": {"translation": r5([u, -0.003, 0])}, "member": "blanket", "part": "support", "anchor": f"blanket-floor-{ai}"})
    mr = max(f, 0.03) / 2
    for b in BONES:
        m = b.get("mass")
        if not m:
            continue
        zc = 0.33 if b["id"] in ("leg-front", "leg-rear") else 0
        out.append({"id": f"mass-{b['id']}", "kind": "ellipsoid", "bone": b["id"], "material": "beech-frame", "role": "detail", "tone": 0.4, "radii": r5([mr, mr, mr]), "transform": {"translation": r5([m["comX"], m["comY"], zc])}, "member": b["id"], "part": "mass"})
    return out


MASS = {}


def masses():
    """⚖️ Section × density masses of every part, summed per bone (the bone masses of the rig)."""
    rho = {"beech": 720, "ply": 680, "brass": 8500}
    parts = {
        "frame": [("4 beech rails 25 x 100 mm (2 x 0.66 m, 2 x 0.645 m)", (2 * 0.66 + 2 * 0.645) * 0.025 * 0.10 * rho["beech"]), ("2 arm posts 40 x 25 x 390 mm", 2 * 0.04 * 0.025 * 0.39 * rho["beech"]), ("2 arm rails 30 x 60 x 660 mm", 2 * 0.03 * 0.06 * 0.66 * rho["beech"]), ("arm and seat-box padding (coir, cotton wadding)", 3.0), ("velour cover of arms and seat box", 0.36)],
        "back": [("2 back posts 30 x 60 x 930 mm", 2 * 0.03 * 0.06 * 0.93 * rho["beech"]), ("top rail 40 x 50 x 700 mm", 0.04 * 0.05 * 0.70 * rho["beech"]), ("bottom rail 50 x 50 x 560 mm", 0.05 * 0.05 * 0.56 * rho["beech"]), ("2 wing boards 12 mm birch plywood, 0.15 m2 each", 2 * 0.15 * 0.012 * rho["ply"]), ("jute webbing, coir and wadding of back and wings", 1.25), ("velour cover of back and wings", 0.42)],
        "seat": [("Bonnell spring unit, 20 springs and border wire", 3.2), ("coir and cotton wadding", 2.2), ("jute webbing", 0.35), ("velour seat cover", 0.25)],
        "leg-front": [("2 turned beech legs, frustum 46 to 31 mm over 128 mm", 2 * math.pi * 0.128 / 3 * (0.023 ** 2 + 0.023 * 0.0155 + 0.0155 ** 2) * rho["beech"]), ("brass ferrule (left) and plastic glide (right)", 0.025 + 0.008)],
        "leg-rear": [("2 turned beech legs, frustum 46 to 31 mm over 130 mm", 2 * math.pi * 0.130 / 3 * (0.023 ** 2 + 0.023 * 0.0155 + 0.0155 ** 2) * rho["beech"]), ("2 brass ferrules", 2 * 0.025)],
        "stool": [("4 beech rails 25 x 80 mm (2 x 0.40 m, 2 x 0.45 m)", (2 * 0.40 + 2 * 0.45) * 0.025 * 0.08 * rho["beech"]), ("4 turned beech legs 40 to 27 mm x 160 mm", 4 * math.pi * 0.16 / 3 * (0.0195 ** 2 + 0.0195 * 0.0135 + 0.0135 ** 2) * rho["beech"]), ("12 mm birch plywood top 0.38 x 0.48 m", 0.38 * 0.48 * 0.012 * rho["ply"]), ("PU foam 30 kg/m3, 0.39 x 0.49 x 0.13 m", 0.39 * 0.49 * 0.13 * 30), ("velour cover", 0.25)],
        "blanket": [("quilted moving blanket 1.5 x 2.0 m at 550 g/m2", 1.5 * 2.0 * 0.55)],
    }
    for k, v in parts.items():
        MASS[k] = (round(sum(x for _, x in v), 3), v)
    return MASS


def main():
    """🚀 Writes the asset document."""
    masses()
    com = {"frame": (0.36, 0.23, "uniform"), "back": (0.5, -0.06, "point"), "seat": (0.27, 0.07, "point"), "leg-front": (0.075, 0, "point"), "leg-rear": (0.076, 0, "point"), "stool": (0.2, 0.26, "point"), "blanket": (0.75, 0, "uniform")}
    refs = {"frame": "wood-handbook-2021", "back": "wood-handbook-2021", "seat": "semio-used-household", "leg-front": "wood-handbook-2021", "leg-rear": "wood-handbook-2021", "stool": "wood-handbook-2021", "blanket": "semio-used-household"}
    for bid, (cx, cy, dist) in com.items():
        BONE[bid]["mass"] = {"kg": MASS[bid][0], "comX": cx, "comY": cy, "distribution": "point" if dist == "point" else "uniform", "source": "authored-estimate", "reference": refs[bid]}
    BONE["leg-front"]["section"] = {"areaM2": 7.55e-4, "secondMomentM4": 4.53e-8, "extremeFibreM": 0.0155, "material": BEECH, "title": T("Turned beech leg, 31 mm at its thinnest", "Gedrechseltes Buchenbein, an der dünnsten Stelle 31 mm")}
    BONE["leg-rear"]["section"] = dict(BONE["leg-front"]["section"])
    BONE["frame"]["section"] = {"areaM2": 0.005, "secondMomentM4": 4.17e-6, "extremeFibreM": 0.05, "material": "beech-frame", "title": T("Two beech side rails 25 x 100 mm", "Zwei Buchenseitenzargen 25 x 100 mm")}
    BONE["back"]["section"] = {"areaM2": 0.0036, "secondMomentM4": 1.08e-6, "extremeFibreM": 0.03, "material": "beech-frame", "title": T("Two beech back posts 30 x 60 mm", "Zwei Buchenrückenpfosten 30 x 60 mm")}
    BONE["stool"]["section"] = {"areaM2": 0.004, "secondMomentM4": 2.13e-6, "extremeFibreM": 0.04, "material": "beech-frame", "title": T("Two beech stool rails 25 x 80 mm", "Zwei Buchenhockerzargen 25 x 80 mm")}
    total = round(sum(v[0] for v in MASS.values()), 2)
    scales = {50: build_50(), 100: build_100(), 200: build_200(), 500: build_500()}
    doc = assemble(scales, total)
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
    print("[DEBUG] mass total", total, {k: v[0] for k, v in MASS.items()})



REFERENCES = [
    ("en-12520-2015", "EN 12520:2015 Furniture - Strength, durability and safety - Requirements for domestic seating (static seat, back and arm loads applied through the methods of EN 1728)."),
    ("en-1728-2012", "EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability (static seat load, back load, arm downward load; loading positions)."),
    ("en-1022-2018", "EN 1022:2018 Furniture - Seating - Determination of stability (forward and rearward overturning of domestic seating)."),
    ("en-1991-1-1-2002", "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (Annex A densities of timber and wood-based materials)."),
    ("en-338-2016", "EN 338:2016 Structural timber - Strength classes (hardwood class D35 used for air-dry beech: bending strength 35 N/mm2, mean modulus of elasticity 12 kN/mm2, mean density 640-720 kg/m3)."),
    ("en-636-2012", "EN 636:2012+A1:2015 Plywood - Specifications (birch plywood for the wing boards and the stool top, 12 mm)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, density and specific heat of timber, wood-based panels, textiles and metals)."),
    ("din-33402-2-2020", "DIN 33402-2:2020-12 Ergonomics - Human body dimensions - Part 2: Values (seated popliteal height, buttock-knee length of adults; 75 kg reader)."),
    ("en-14465-2003", "EN 14465:2003+A1:2006 Textiles - Upholstery fabrics - Specification and methods of test (pile fabrics such as cotton velour; abrasion and pilling of seat covers)."),
    ("iso-105-b02-2014", "ISO 105-B02:2014 Textiles - Tests for colour fastness - Part B02: Colour fastness to artificial light: Xenon arc fading lamp test (light fading of dyed cotton pile fabrics, the cause of the bleached window side)."),
    ("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service (European beech about 720 kg/m3 air-dry, modulus of elasticity about 14 GPa, specific heat of wood)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (surface emissivities of textiles, wood and oxidised brass; thermal penetration depth)."),
    ("iso-7730-2005", "ISO 7730:2005 Ergonomics of the thermal environment - Analytical determination and interpretation of thermal comfort (clothing surface temperature of a seated person about 30 degC)."),
    ("semio-used-household", "Semio staffage content, used household objects: dimensions and masses authored from catalogue-typical West German upholstered wingback armchairs (Ohrensessel) with matching footstools of the 1960s as offered second-hand (width 0.75-0.80 m, depth 0.80-0.90 m, height 1.05-1.15 m, seat height 0.42-0.46 m, 25-32 kg), Bonnell spring seats and quilted removal blankets 1.5 x 2.0 m of 500-600 g/m2; parameters tagged authored-estimate are not taken from a single datasheet."),
]


def materials():
    """🧱 The materials of the asset."""
    M = lambda i, en, de, e, a, rho, k, cp, src, ref, E=None, fs=None: {k2: v2 for k2, v2 in {"id": i, "title": T(en, de), "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp, "elasticModulusPa": E, "strengthPa": fs, "source": src, "reference": ref}.items() if v2 is not None}
    return [
        M(V, "Cotton velour cover over coir and wadding (aged, pile flattened)", "Baumwollvelours über Kokos und Watte (gealtert, Flor plattgedrückt)", 0.92, 0.72, 120, 0.05, 1350, "authored-estimate", "en-14465-2003"),
        M(VF, "Sun-bleached cotton velour (window side)", "Sonnengebleichter Baumwollvelours (Fensterseite)", 0.92, 0.55, 120, 0.05, 1350, "authored-estimate", "iso-105-b02-2014"),
        M(BEECH, "Turned beech, dark stain, lacquer worn", "Gedrechselte Buche, dunkel gebeizt, Lack abgegriffen", 0.9, 0.7, 720, 0.18, 1600, "literature", "wood-handbook-2021", 1.4e10, 1.0e8),
        M("beech-frame", "Beech frame timber (hidden, strength class D35)", "Buchen-Gestellholz (verdeckt, Festigkeitsklasse D35)", 0.9, 0.55, 720, 0.18, 1600, "literature", "en-338-2016", 1.2e10, 3.5e7),
        M("birch-plywood", "Birch plywood 12 mm", "Birkensperrholz 12 mm", 0.9, 0.55, 680, 0.17, 1600, "literature", "en-636-2012", 9.0e9, 4.0e7),
        M("spring-steel", "Spring steel wire (Bonnell unit)", "Federstahldraht (Bonnell-Federkern)", 0.3, 0.6, 7850, 50, 470, "literature", "incropera-2018", 2.06e11, 1.4e9),
        M("brass-aged", "Tarnished brass ferrule", "Angelaufene Messingkappe", 0.35, 0.5, 8500, 110, 380, "literature", "incropera-2018"),
        M("pe-glide", "Black polyethylene slip-on glide (replacement)", "Schwarzer Polyethylen-Aufsteckgleiter (Ersatz)", 0.9, 0.9, 950, 0.4, 1900, "literature", "iso-10456-2007"),
        M("moving-blanket", "Quilted cotton-polyester moving blanket", "Gesteppte Möbeldecke aus Baumwolle und Polyester", 0.9, 0.65, 92, 0.045, 1300, "authored-estimate", "semio-used-household"),
        M("cardboard", "Cardboard price tag", "Preisschild aus Karton", 0.9, 0.3, 700, 0.07, 1300, "literature", "iso-10456-2007"),
    ]


def anchors():
    """📍 Contacts, supports and load points."""
    L = BONE["leg-rear"]["lengthM"]
    A = []
    for ai, u in (("a", 0.375), ("b", 1.125)):
        A.append({"id": f"blanket-floor-{ai}", "bone": "blanket", "x": u, "y": -0.006, "kind": "contact", "title": T(f"Blanket on the floor ({'rear' if ai == 'a' else 'front'} half)", f"Decke auf dem Boden ({'hintere' if ai == 'a' else 'vordere'} Hälfte)"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.75, "footprintWidthM": 1.0, "frictionCoefficient": 0.6})
    for side, z, en, de in (("l", 0.33, "left", "links"), ("r", -0.33, "right", "rechts")):
        A.append({"id": f"foot-front-{side}", "bone": "leg-front", "x": 0.15, "y": 0, "z": z, "kind": "contact", "title": T(f"Front foot, {en}", f"Vorderfuß, {de}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.031, "footprintWidthM": 0.031, "frictionCoefficient": 0.35})
        A.append({"id": f"foot-rear-{side}", "bone": "leg-rear", "x": r5(L), "y": 0, "z": z, "kind": "contact", "title": T(f"Rear foot, {en}", f"Hinterfuß, {de}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.031, "footprintWidthM": 0.031, "frictionCoefficient": 0.35})
    for xi, xl, en, de in (("rear", 0.04, "rear", "hinten"), ("front", 0.36, "front", "vorn")):
        for side, z, en2, de2 in (("l", 0.21, "left", "links"), ("r", -0.21, "right", "rechts")):
            A.append({"id": f"stool-foot-{xi}-{side}", "bone": "stool", "x": xl, "y": 0, "z": z, "kind": "contact", "title": T(f"Footstool foot, {en} {en2}", f"Hockerfuß, {de} {de2}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.027, "footprintWidthM": 0.027, "frictionCoefficient": 0.4})
    for side, z, en, de in (("l", 0.33, "left", "linke"), ("r", -0.33, "right", "rechte")):
        A.append({"id": f"grip-{side}", "bone": "back", "x": 0.905, "y": -0.05, "z": z, "kind": "support", "title": T(f"Carrier's {en} hand at the top of the {en} wing", f"{de.capitalize()} Hand des Trägers oben am {'linken' if side == 'l' else 'rechten'} Ohr"), "support": "pin", "footprintLengthM": 0.09, "footprintWidthM": 0.08, "frictionCoefficient": 0.6})
    A.append({"id": "carry-back", "bone": "back", "x": 0.45, "y": 0.015, "kind": "contact", "title": T("Outer back resting on the carrier's back", "Rückseite liegt auf dem Rücken des Trägers"), "support": "contact", "normalDeg": 61, "footprintLengthM": 0.3, "footprintWidthM": 0.3, "frictionCoefficient": 0.6})
    sl = to_local("seat", C([(0.49, 0.425)]))[0]
    A.append({"id": "seat-load", "bone": "seat", "x": sl[0], "y": sl[1], "z": 0, "kind": "load", "title": T("Sitting point on the sprung seat", "Sitzpunkt auf dem Federkernsitz")})
    bl = to_local("back", C([(0.236, 0.56)]))[0]
    A.append({"id": "back-load", "bone": "back", "x": bl[0], "y": bl[1], "z": 0, "kind": "load", "title": T("Lumbar lean on the inner back", "Anlehnen im Lendenbereich des Rückens")})
    al = to_local("frame", C([(0.55, 0.625)]))[0]
    for side, z, en, de in (("l", 0.325, "left", "linke"), ("r", -0.325, "right", "rechte")):
        A.append({"id": f"arm-load-{side}", "bone": "frame", "x": al[0], "y": al[1], "z": z, "kind": "load", "title": T(f"Forearm on the {en} arm", f"Unterarm auf der {de}n Armlehne")})
    st = to_local("stool", C([(1.12, 0.39)]))[0]
    A.append({"id": "stool-load", "bone": "stool", "x": st[0], "y": st[1], "z": 0, "kind": "load", "title": T("Calves and heels on the footstool", "Waden und Fersen auf dem Hocker")})
    return A


REGIONS = ["cover-seat", "cover-body", "cover-faded", "legs-beech", "stool-cover", "blanket"]


def binding(over=None, sun=None):
    """🌬️ Region bindings of a pose: exposed 1, unlit, blanket half on the floor, with overrides."""
    b = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGIONS}
    b["blanket"] = {"exposedFraction": 0.5, "sunlitFraction": 0}
    for k, v in (over or {}).items():
        b[k] = v
    return {"regions": b, "emitters": {}}


def reader_loads(k=1.0):
    """🧑 A 75 kg reader: seat, lumbar lean, both forearms and the calves on the footstool."""
    return [
        {"id": "reader-seat", "kind": "occupant", "title": T("Reader on the seat (75 kg person, seat share)", "Lesende Person auf dem Sitz (75 kg, Sitzanteil)"), "anchor": "seat-load", "fxN": 0, "fyN": round(-560 * k, 1), "source": "authored-estimate", "reference": "din-33402-2-2020"},
        {"id": "reader-back", "kind": "occupant", "title": T("Reader leaning back", "Lesende Person lehnt sich an"), "anchor": "back-load", "fxN": round(-100 * k, 1), "fyN": 0, "source": "authored-estimate", "reference": "en-1728-2012"},
        {"id": "reader-arm-l", "kind": "occupant", "title": T("Left forearm on the arm", "Linker Unterarm auf der Armlehne"), "anchor": "arm-load-l", "fxN": 0, "fyN": round(-40 * k, 1), "source": "authored-estimate", "reference": "din-33402-2-2020"},
        {"id": "reader-arm-r", "kind": "occupant", "title": T("Right forearm on the arm", "Rechter Unterarm auf der Armlehne"), "anchor": "arm-load-r", "fxN": 0, "fyN": round(-40 * k, 1), "source": "authored-estimate", "reference": "din-33402-2-2020"},
        {"id": "reader-feet", "kind": "occupant", "title": T("Calves and heels on the footstool", "Waden und Fersen auf dem Hocker"), "anchor": "stool-load", "fxN": 0, "fyN": round(-95 * k, 1), "source": "authored-estimate", "reference": "din-33402-2-2020"},
    ]


HOME = {"lift": {"dy": -TB}, "stool": {"dy": -TB}, "blanket": {"dx": -1.65}}
ALL_FEET = ["foot-front-l", "foot-front-r", "foot-rear-l", "foot-rear-r"]
STOOL_FEET = ["stool-foot-rear-l", "stool-foot-rear-r", "stool-foot-front-l", "stool-foot-front-r"]
BLANKET = ["blanket-floor-a", "blanket-floor-b"]


def poses():
    """🧍 Named and key poses."""
    seated = {"cover-seat": {"exposedFraction": 0.6, "sunlitFraction": 0.1, "contact": {"fraction": 0.4, "tempC": 30}}, "cover-body": {"exposedFraction": 0.93, "sunlitFraction": 0.15, "contact": {"fraction": 0.07, "tempC": 30}}, "cover-faded": {"exposedFraction": 0.97, "sunlitFraction": 0.7, "contact": {"fraction": 0.03, "tempC": 30}}, "legs-beech": {"exposedFraction": 1, "sunlitFraction": 0.2}, "stool-cover": {"exposedFraction": 0.88, "sunlitFraction": 0.2, "contact": {"fraction": 0.12, "tempC": 30}}}
    empty_home = {"cover-seat": {"exposedFraction": 1, "sunlitFraction": 0.3}, "cover-body": {"exposedFraction": 1, "sunlitFraction": 0.15}, "cover-faded": {"exposedFraction": 1, "sunlitFraction": 0.7}, "legs-beech": {"exposedFraction": 1, "sunlitFraction": 0.2}, "stool-cover": {"exposedFraction": 1, "sunlitFraction": 0.3}}
    carrying = {"cover-body": {"exposedFraction": 0.92, "sunlitFraction": 0, "contact": {"fraction": 0.08, "tempC": 30}}}
    P = lambda i, en, de, den, dde, bones, contacts, th, loads, key=False: {k: v for k, v in {"id": i, "title": T(en, de), "description": T(den, dde), "key": True if key else None, "bones": bones, "ground": "auto", "contacts": contacts, "thermal": th, "loads": loads}.items() if v is not None}
    return [
        P("stored", "Stored on a moving blanket", "Auf einer Möbeldecke gelagert", "As the second-hand store keeps it: the armchair and its footstool stand emptied and cleaned on a folded quilted moving blanket in the depot, out of the sun.", "Wie das Gebrauchtwarenkaufhaus ihn aufbewahrt: Sessel und Hocker stehen geleert und gereinigt auf einer gefalteten, gesteppten Möbeldecke im Lager, ohne Sonne.", {}, BLANKET + ALL_FEET + STOOL_FEET, binding({"blanket": {"exposedFraction": 0.45, "sunlitFraction": 0}}), []),
        P("tip-back-key", "Tipped back onto the rear feet", "Auf die Hinterfüße gekippt", "The carrier grips the tops of both wings from behind and tips the chair back 22 degrees over the rear edges of its rear feet; the front feet leave the blanket.", "Der Träger greift von hinten oben an beide Ohren und kippt den Sessel um 22 Grad über die Hinterkanten der Hinterfüße; die Vorderfüße lösen sich von der Decke.", {"chair": {"angleDeg": 22}}, BLANKET + ["foot-rear-l", "foot-rear-r", "grip-l", "grip-r"] + STOOL_FEET, binding({"blanket": {"exposedFraction": 0.45, "sunlitFraction": 0}}), [], True),
        P("shoulder-key", "Swinging onto the back", "Auf den Rücken geschwungen", "Half way: the chair is lifted 0.45 m and tipped 42 degrees while the carrier turns under it; its outer back meets his back.", "Auf halbem Weg: Der Sessel ist 0,45 m angehoben und um 42 Grad gekippt, während der Träger sich darunter dreht; die Rückseite trifft auf seinen Rücken.", {"lift": {"dy": 0.45}, "chair": {"angleDeg": 42}}, BLANKET + ["grip-l", "grip-r", "carry-back"] + STOOL_FEET, binding(carrying), [], True),
        P("carried", "Carried on one person's back", "Von einer Person auf dem Rücken getragen", "One carrier walks it out: the chair lies tipped 55 degrees on his bent back (outer back on his back, about 1.0 to 1.4 m above the floor, legs pointing up and forward), both hands holding the tops of the wings over his shoulders; blanket and footstool stay behind.", "Eine Person trägt ihn hinaus: Der Sessel liegt um 55 Grad gekippt auf ihrem gebeugten Rücken (Rückseite auf dem Rücken, etwa 1,0 bis 1,4 m über dem Boden, Beine nach oben und vorn), beide Hände halten über den Schultern die Ohren oben fest; Decke und Hocker bleiben zurück.", {"lift": {"dy": 0.85}, "chair": {"angleDeg": 55}}, BLANKET + ["grip-l", "grip-r", "carry-back"] + STOOL_FEET, binding(carrying), []),
        P("in-use-empty", "Back in a living room", "Wieder in einem Wohnzimmer", "Set down on the floor of a living room beside the window, the footstool in front of it, the blanket pulled out and left folded behind it.", "Im Wohnzimmer neben dem Fenster auf den Boden gestellt, der Hocker davor, die Decke herausgezogen und gefaltet dahinter liegen gelassen.", dict(HOME), ALL_FEET + STOOL_FEET + BLANKET, binding(empty_home), []),
        P("sitting-key", "Dropping into the seat", "Ins Polster fallen", "The reader drops into the seat: the sagged spring unit gives 44 mm at the peak before it settles.", "Die lesende Person lässt sich in den Sitz fallen: der durchgesessene Federkern gibt in der Spitze 44 mm nach, bevor er sich setzt.", dict(HOME, seat={"dy": -0.044}), ALL_FEET + STOOL_FEET + BLANKET, binding(seated), reader_loads(), True),
        P("in-use", "In use with a reader", "Im Gebrauch mit einer lesenden Person", "A 75 kg reader sits back in it with both forearms on the arms and the calves on the footstool; the spring seat sinks 35 mm, the sun from the window falls on the bleached left side.", "Eine lesende Person mit 75 kg sitzt zurückgelehnt darin, beide Unterarme auf den Armlehnen und die Waden auf dem Hocker; der Federkernsitz sinkt 35 mm ein, die Sonne aus dem Fenster fällt auf die ausgebleichte linke Seite.", dict(HOME, seat={"dy": -0.035}), ALL_FEET + STOOL_FEET + BLANKET, binding(seated), reader_loads()),
        P("seat-proof-load", "Seat proof load before resale", "Sitz-Prüflast vor dem Weiterverkauf", "The reuse depot checks the old spring seat with the static seat load of the domestic seating standard on the sitting point; the springs bottom out at 50 mm and the frame carries it.", "Das Wiederverwendungslager prüft den alten Federkernsitz mit der statischen Sitzlast der Norm für Wohnsitzmöbel am Sitzpunkt; die Federn gehen bei 50 mm auf Block und der Rahmen trägt.", dict(HOME, seat={"dy": -0.05}), ALL_FEET + STOOL_FEET + BLANKET, binding(), [{"id": "proof-seat", "kind": "imposed", "title": T("Static seat test load (1600 N)", "Statische Sitzprüflast (1600 N)"), "anchor": "seat-load", "fxN": 0, "fyN": -1600, "source": "authored-estimate", "reference": "en-12520-2015"}]),
    ]


def clips():
    """🎞️ The carry clip and the clip of its life in use."""
    span = lambda a, f, t: {"anchor": a, "fromS": f, "toS": t}
    carry = {"id": "carry", "title": T("Tip back and carry out on the back", "Ankippen und auf dem Rücken hinaustragen"), "description": T("The carrier tips the chair back over its rear feet, lifts it and swings it onto his bent back, holding it by the tops of the wings; the blanket and the footstool stay in the depot.", "Der Träger kippt den Sessel über die Hinterfüße an, hebt ihn an und schwingt ihn auf seinen gebeugten Rücken, die Hände oben an den Ohren; Decke und Hocker bleiben im Lager."),
             "durationS": 3.2, "fps": 30, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.9, "pose": "tip-back-key", "ease": "smooth"}, {"t": 2.0, "pose": "shoulder-key", "ease": "smooth"}, {"t": 3.2, "pose": "carried", "ease": "smooth"}],
             "contacts": [span(a, 0, 3.2) for a in BLANKET + STOOL_FEET] + [span("foot-front-l", 0, 0.25), span("foot-front-r", 0, 0.25), span("foot-rear-l", 0, 1.1), span("foot-rear-r", 0, 1.1), span("grip-l", 0.2, 3.2), span("grip-r", 0.2, 3.2), span("carry-back", 1.6, 3.2)],
             "phases": [{"id": "tip-back", "fromS": 0, "toS": 0.9, "title": T("Tipping back over the rear feet", "Ankippen über die Hinterfüße")}, {"id": "lift", "fromS": 0.9, "toS": 2.0, "title": T("Lifting and turning under it", "Anheben und Darunterdrehen")}, {"id": "on-the-back", "fromS": 2.0, "toS": 3.2, "title": T("Settling on the carrier's back", "Auf dem Rücken des Trägers zurechtrücken")}],
             "events": [{"t": 0.2, "kind": "contact-on", "title": T("Hands grip the left wing", "Hände greifen das linke Ohr"), "target": "grip-l"}, {"t": 0.2, "kind": "contact-on", "title": T("Hands grip the right wing", "Hände greifen das rechte Ohr"), "target": "grip-r"}, {"t": 0.25, "kind": "contact-off", "title": T("Left front foot leaves the blanket", "Linker Vorderfuß löst sich von der Decke"), "target": "foot-front-l"}, {"t": 0.25, "kind": "contact-off", "title": T("Right front foot leaves the blanket", "Rechter Vorderfuß löst sich von der Decke"), "target": "foot-front-r"}, {"t": 1.1, "kind": "contact-off", "title": T("Left rear foot lifts off", "Linker Hinterfuß hebt ab"), "target": "foot-rear-l"}, {"t": 1.1, "kind": "contact-off", "title": T("Right rear foot lifts off", "Rechter Hinterfuß hebt ab"), "target": "foot-rear-r"}, {"t": 1.6, "kind": "contact-on", "title": T("Outer back meets the carrier's back", "Rückseite trifft auf den Rücken des Trägers"), "target": "carry-back"}, {"t": 3.2, "kind": "motion", "title": T("Ready to walk out", "Bereit zum Hinaustragen")}],
             "tracks": []}
    sit = {"id": "sit-down-to-read", "title": T("Sit down to read", "Zum Lesen hinsetzen"), "description": T("A reader drops into the sagged spring seat (44 mm peak), settles at 35 mm, leans back, rests the forearms and puts the feet up on the footstool; the clothed body warms the velour where it sits.", "Eine lesende Person lässt sich in den durchgesessenen Federkernsitz fallen (44 mm Spitze), setzt sich bei 35 mm, lehnt sich an, legt die Unterarme ab und die Füße auf den Hocker; der bekleidete Körper wärmt den Velours, wo er aufliegt."),
           "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
           "keys": [{"t": 0, "pose": "in-use-empty", "ease": "in"}, {"t": 1.2, "pose": "sitting-key", "ease": "out"}, {"t": 2.0, "pose": "in-use", "ease": "smooth"}, {"t": 4.0, "pose": "in-use", "ease": "smooth"}],
           "contacts": [span(a, 0, 4.0) for a in ALL_FEET + STOOL_FEET + BLANKET],
           "phases": [{"id": "drop", "fromS": 0, "toS": 1.2, "title": T("Dropping into the seat", "Ins Polster fallen")}, {"id": "settle", "fromS": 1.2, "toS": 2.4, "title": T("Settling and leaning back", "Einsinken und Anlehnen")}, {"id": "feet-up", "fromS": 2.4, "toS": 4.0, "title": T("Feet up on the footstool", "Füße auf den Hocker")}],
           "events": [{"t": 0.6, "kind": "load-on", "title": T("Reader loads the seat", "Person belastet den Sitz"), "target": "reader-seat"}, {"t": 1.2, "kind": "thermal", "title": T("Clothed body starts warming the seat velour", "Bekleideter Körper beginnt den Sitzvelours zu wärmen")}, {"t": 1.6, "kind": "load-on", "title": T("Reader leans back", "Person lehnt sich an"), "target": "reader-back"}, {"t": 2.4, "kind": "load-on", "title": T("Feet go up on the footstool", "Füße kommen auf den Hocker"), "target": "reader-feet"}],
           "tracks": [{"target": "load:reader-seat", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.6, "v": 0.2}, {"t": 1.2, "v": 1.25}, {"t": 2.0, "v": 1}, {"t": 4.0, "v": 1}]},
                      {"target": "load:reader-back", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.6, "v": 0}, {"t": 2.2, "v": 1}, {"t": 4.0, "v": 1}]},
                      {"target": "load:reader-arm-l", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.4, "v": 0}, {"t": 2.2, "v": 1}, {"t": 4.0, "v": 1}]},
                      {"target": "load:reader-arm-r", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.4, "v": 0}, {"t": 2.2, "v": 1}, {"t": 4.0, "v": 1}]},
                      {"target": "load:reader-feet", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2.4, "v": 0}, {"t": 3.2, "v": 1}, {"t": 4.0, "v": 1}]}]}
    return [carry, sit]


AREA = {"cover-seat": 1.39, "cover-body": 2.6, "cover-faded": 1.32, "legs-beech": 0.15, "stool-cover": 0.75, "blanket": 3.05}


def layer(lid, en, de, kind, mat, t, k, rho, cp, src, ref):
    """🧶 One region layer: resistance t/k, heat capacity rho cp min(t, sqrt(alpha 3600 s))."""
    alpha = k / (rho * cp)
    hc = rho * cp * min(t, math.sqrt(alpha * 3600))
    return {"id": lid, "title": T(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": round(t / k, 4), "heatCapacityJM2K": round(hc), "source": src, "reference": ref}


def thermal():
    """🌡️ Passive regions of covers, legs and blanket."""
    pad = lambda mat: layer("padded-cover", "Velour over 40 mm coir and cotton wadding", "Velours über 40 mm Kokos und Baumwollwatte", "insulation", mat, 0.04, 0.05, 120, 1350, "authored-estimate", "iso-10456-2007")
    R = lambda i, en, de, kind, mat, geom, lc, svf, layers, extra=None: dict({"id": i, "title": T(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": AREA[i], "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": layers, "heatCapacityJM2K": sum(l["heatCapacityJM2K"] for l in layers), "source": "authored-estimate", "reference": "iso-10456-2007"}, **(extra or {}))
    regions = [
        R("cover-seat", "Velour of the sprung seat and the seat front", "Velours des Federkernsitzes und der Sitzfront", "surface", V, "horizontal-plate-up", 0.5, 0.8, [pad(V)]),
        R("cover-body", "Velour of back, right arm and right wing", "Velours von Rücken, rechter Armlehne und rechtem Ohr", "surface", V, "vertical-plate", 0.8, 0.5, [pad(V)], {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        R("cover-faded", "Sun-bleached velour of the left arm and left wing", "Sonnengebleichter Velours der linken Armlehne und des linken Ohrs", "surface", VF, "vertical-plate", 0.6, 0.5, [pad(VF)]),
        R("legs-beech", "Turned beech legs of chair and footstool", "Gedrechselte Buchenbeine von Sessel und Hocker", "surface", BEECH, "cylinder", 0.04, 0.3, [layer("beech", "Beech, 20 mm to the core", "Buche, 20 mm bis zum Kern", "timber", BEECH, 0.02, 0.18, 720, 1600, "literature", "iso-10456-2007")]),
        R("stool-cover", "Velour of the footstool", "Velours des Hockers", "surface", V, "horizontal-plate-up", 0.45, 0.8, [pad(V)]),
        R("blanket", "Quilted moving blanket", "Gesteppte Möbeldecke", "surface", "moving-blanket", "horizontal-plate-up", 1.0, 0.6, [layer("quilt", "Two folds of 6 mm quilt", "Zwei Lagen 6-mm-Steppdecke", "insulation", "moving-blanket", 0.012, 0.045, 92, 1300, "authored-estimate", "semio-used-household")]),
    ]
    return {"environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.15, "pressurePa": 101325},
            "passengers": [], "operating": [], "regions": regions, "emitters": [],
            "assumptions": [
                T("Passive upholstery, timber and textile surfaces: absorbed sun, long-wave exchange, convection and conduction with heat capacity; no emitters, no power (the armchair states 0 W, passive, on the cover region).", "Passive Polster-, Holz- und Textilflächen: absorbierte Sonne, Langwellenaustausch, Konvektion und Leitung mit Wärmekapazität; keine Emitter, keine Leistung (der Sessel gibt an der Bezugsregion 0 W, passiv, an)."),
                T("The covers are one layer each: cotton velour over 40 mm coir and cotton wadding (0.05 W/mK, 120 kg/m3 effective), so the surface follows air and sun within minutes; heat capacity rho cp min(thickness, sqrt(alpha 3600 s)).", "Die Bezüge sind je eine Schicht: Baumwollvelours über 40 mm Kokos und Baumwollwatte (0,05 W/mK, effektiv 120 kg/m3), die Oberfläche folgt Luft und Sonne daher in Minuten; Wärmekapazität rho cp min(Dicke, sqrt(alpha 3600 s))."),
                T("Aged surfaces: decades of window light bleached the left arm and wing (solar absorptance 0.55 against 0.72 of the unfaded velour, after the light-fading behaviour of dyed cotton pile in ISO 105-B02; authored estimate); the flattened pile on the arm tops keeps the emissivity of 0.92 of textiles; worn dark-stained beech 0.90 and 0.70; tarnished brass 0.35.", "Gealterte Oberflächen: Jahrzehnte Fensterlicht haben linke Armlehne und linkes Ohr gebleicht (solarer Absorptionsgrad 0,55 gegenüber 0,72 des ungebleichten Velours, nach dem Lichtechtheitsverhalten gefärbten Baumwollflors in ISO 105-B02; Schätzwert); der plattgedrückte Flor der Armauflagen behält den Emissionsgrad 0,92 von Textilien; abgegriffene dunkel gebeizte Buche 0,90 und 0,70; angelaufenes Messing 0,35."),
                T("Reference case: a living room at 21 degC with a 350 W/m2 sun patch from the window; the stored and carried poses are out of the sun. A seated reader covers 0.4 of the seat, 0.07 of the back covers and 0.12 of the footstool with clothing at about 30 degC (ISO 7730); the carrier's back covers 0.08 of the back covers.", "Bezugsfall: ein Wohnzimmer mit 21 degC und einem Sonnenfleck von 350 W/m2 aus dem Fenster; Lager- und Tragepose liegen ohne Sonne. Eine sitzende Person bedeckt 0,4 des Sitzes, 0,07 der Rückenbezüge und 0,12 des Hockers mit Kleidung bei etwa 30 degC (ISO 7730); der Rücken des Trägers bedeckt 0,08 der Rückenbezüge."),
                T("The folded blanket lies with its underside on the floor (half its area exposed); ferrules, the plastic glide and the cardboard price tag belong to no region.", "Die gefaltete Decke liegt mit der Unterseite auf dem Boden (die Hälfte ihrer Fläche frei); Messingkappen, Kunststoffgleiter und Preisschild aus Karton gehören zu keiner Region."),
            ]}


def structure_model():
    """🏗️ Supports and assumptions."""
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": BLANKET + STOOL_FEET,
            "assumptions": [
                T("The armchair is a rigid beech frame: four 25 x 100 mm seat rails, arm posts and arm rails (seat frame bone), two 30 x 60 mm back posts with top and bottom rail and two 12 mm plywood wing boards (back bone), four turned legs screwed under the rail corners; the Bonnell spring unit bears on the front and back rails and sinks on its prismatic joint.", "Der Sessel ist ein starres Buchengestell: vier Sitzzargen 25 x 100 mm, Armpfosten und Armholme (Knochen Sitzrahmen), zwei Rückenpfosten 30 x 60 mm mit Kopf- und Fußholm und zwei Ohrenbretter aus 12-mm-Sperrholz (Knochen Rücken), vier gedrechselte Beine unter den Zargenecken verschraubt; der Bonnell-Federkern liegt auf vorderer und hinterer Zarge und sinkt auf seinem Schiebegelenk ein."),
                T("Masses are section x density of every part (beech 720 kg/m3, birch plywood 680 kg/m3) plus the upholstery and spring masses of catalogue-typical 1960s wingback chairs: seat frame 10.3 kg, back 8.5 kg, seat 6.0 kg, legs 0.25 and 0.27 kg per pair, footstool 5.3 kg, blanket 1.65 kg.", "Die Massen sind Querschnitt x Dichte jedes Teils (Buche 720 kg/m3, Birkensperrholz 680 kg/m3) plus die Polster- und Federmassen katalogtypischer Ohrensessel der 1960er Jahre: Sitzrahmen 10,3 kg, Rücken 8,5 kg, Sitz 6,0 kg, Beine 0,25 und 0,27 kg je Paar, Hocker 5,3 kg, Decke 1,65 kg."),
                T("Stored and in use the chair stands on four foot contacts, the footstool on four, the blanket on two floor contacts; carried, the chair hangs on the two hands of the carrier (pins at the tops of the wings) and rests with its outer back on his back (a contact whose normal leans 61 degrees). Grip and back reactions are rigid-body resultants, not hand or spine forces.", "Gelagert und im Gebrauch steht der Sessel auf vier Fußkontakten, der Hocker auf vier, die Decke auf zwei Bodenkontakten; getragen hängt der Sessel an den beiden Händen des Trägers (Gelenklager oben an den Ohren) und liegt mit der Rückseite auf dessen Rücken (ein Kontakt mit um 61 Grad geneigter Normale). Griff- und Rückenreaktionen sind Starrkörperresultierende, keine Hand- oder Wirbelsäulenkräfte."),
                T("The reader (75 kg) is 560 N on the seat, 2 x 40 N on the arms, 95 N on the footstool and a 100 N lean on the lumbar back; the 1600 N seat proof load follows the static seat test of EN 1728 as used by EN 12520 and is an authored estimate to be checked against the standard text. Sink depths of the sagged spring seat (35 mm settled, 44 mm peak, 50 mm on block) are authored estimates.", "Die lesende Person (75 kg) ist 560 N auf dem Sitz, 2 x 40 N auf den Armlehnen, 95 N auf dem Hocker und 100 N Anlehnung im Lendenbereich; die Sitzprüflast 1600 N folgt dem statischen Sitztest nach EN 1728, wie ihn EN 12520 verwendet, und ist ein Schätzwert, am Normtext zu prüfen. Die Einsinktiefen des durchgesessenen Federkerns (35 mm gesetzt, 44 mm Spitze, 50 mm auf Block) sind Schätzwerte."),
            ], "clinical": False}


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {50: [{"id": "quilting", "title": T("Diamond quilting of the moving blanket", "Rautensteppung der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 2.0, "penMm": 0.13}], 100: [{"id": "quilting-100", "title": T("Diamond quilting of the moving blanket", "Rautensteppung der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.0, "penMm": 0.13}], 200: [], 500: []}

REP_DECISIONS = {
    50: dict(
        silhouette=T("Drawn from the 1:50 body: the wingback chair 0.78 m wide and 1.10 m high with its reclined back, the curved wings, two padded arms with front rolls, the sagged spring seat, four turned beech legs with ferrules, the footstool beside it and the folded moving blanket under both. Side elevation from the right, front from ahead, plan from above.", "Aus dem 1:50-Körper gezeichnet: der Ohrensessel 0,78 m breit und 1,10 m hoch mit geneigtem Rücken, geschwungenen Ohren, zwei gepolsterten Armlehnen mit Frontrollen, dem durchgesessenen Federkernsitz, vier gedrechselten Buchenbeinen mit Kappen, dem Hocker daneben und der gefalteten Möbeldecke unter beiden. Seitenansicht von rechts, Vorderansicht von vorn, Grundriss von oben."),
        retained=T("The 22 mm dish of the sagged seat, the 8 mm front piping, leg bead and brass ferrules, the replaced plastic glide, the price tag, worn arm-roll fronts, scuffs on the legs, a stain on the seat, the stool pad dip.", "Die 22-mm-Mulde des durchgesessenen Sitzes, der 8-mm-Frontkeder, Beinwulst und Messingkappen, der ersetzte Kunststoffgleiter, das Preisschild, abgewetzte Armrollenfronten, Kratzer an den Beinen, ein Fleck auf dem Sitz, die Delle im Hockerpolster."),
        merged=T("Near and far legs, arms and wings coincide in side elevation and are drawn once in front of each other; the hidden beech frame and the spring unit lie inside the covers.", "Nahe und ferne Beine, Armlehnen und Ohren fallen in der Seitenansicht zusammen und liegen voreinander; das verdeckte Buchengestell und der Federkern liegen in den Bezügen."),
        omitted=T("Velour pile, staples, webbing, dust cover, the price written on the tag.", "Veloursflor, Klammern, Gurte, Staubschutz, der Preis auf dem Schild."),
        thinElements=T("Piping 9 mm and the tag string are detail strokes; seams 0.25 mm detail lines; wear, scuffs and stain 0.13 mm texture strokes.", "Keder 9 mm und die Schnur des Schildes sind Detailstriche; Nähte 0,25-mm-Detaillinien; Abrieb, Kratzer und Fleck 0,13-mm-Texturstriche."),
        hatching=T("Diamond quilting of the blanket as a 45 degree cross hatch of 100 mm in plan; upholstery by tone.", "Rautensteppung der Decke als 45-Grad-Kreuzschraffur von 100 mm im Grundriss; Polster durch Ton."),
        styles=T("Tone 0.40 back, 0.44-0.50 right arm and wing, 0.60-0.66 bleached left arm and wing, 0.52 seat, 0.28 legs, 0.62 brass, 0.78 blanket.", "Ton 0,40 Rücken, 0,44-0,50 rechte Armlehne und rechtes Ohr, 0,60-0,66 gebleichte linke Armlehne und linkes Ohr, 0,52 Sitz, 0,28 Beine, 0,62 Messing, 0,78 Decke."),
        overlays=T("One shape per part with its region, so thermal and structural overlays follow seat, covers, bleached side, legs, footstool and blanket separately.", "Eine Form je Teil mit ihrer Region, damit thermische und statische Überlagerungen Sitz, Bezügen, gebleichter Seite, Beinen, Hocker und Decke getrennt folgen.")),
    100: dict(
        silhouette=T("Redrawn for 1:100 (15 mm): the same chair, footstool and blanket with fewer points: back, wings, arms with rolls, the dished seat, tapered legs.", "Für 1:100 (15 mm) neu gezeichnet: derselbe Sessel, Hocker und dieselbe Decke mit weniger Punkten: Rücken, Ohren, Armlehnen mit Rollen, der durchgesessene Sitz, konische Beine."),
        retained=T("The seat dish (22 mm), arm rolls, the stool pad dip, the seat seam.", "Die Sitzmulde (22 mm), Armrollen, die Delle im Hockerpolster, die Sitznaht."),
        merged=T("Ferrules and glide are part of the leg outline; the leg bead is smoothed.", "Kappen und Gleiter gehören zum Beinumriss; der Beinwulst ist geglättet."),
        omitted=T("Piping, price tag, wear and scuff strokes, the stain.", "Keder, Preisschild, Abrieb- und Kratzstriche, der Fleck."),
        thinElements=T("Seams as 0.18 mm detail lines.", "Nähte als 0,18-mm-Detaillinien."),
        hatching=T("Blanket quilting as a 100 mm cross hatch (1.0 mm on paper).", "Deckensteppung als 100-mm-Kreuzschraffur (1,0 mm auf dem Papier)."),
        styles=T("Tones as at 1:50.", "Töne wie bei 1:50."),
        overlays=T("One shape per part with its region.", "Eine Form je Teil mit ihrer Region.")),
    200: dict(
        silhouette=T("Redrawn for 1:200 (30 mm): each arm is one rounded block with its front roll, the seat a crowned slab, the footstool one block on four legs, the legs plain tapers.", "Für 1:200 (30 mm) neu gezeichnet: jede Armlehne ein gerundeter Block mit Frontrolle, der Sitz eine bombierte Platte, der Hocker ein Block auf vier Beinen, die Beine schlichte Konen."),
        retained=T("Reclined back with crest, the wing curve, the arm roll front, seat crown.", "Geneigter Rücken mit Kopfrolle, die Ohrenkurve, die Armrollenfront, die Sitzbombierung."),
        merged=T("Arm panel and roll into one arm; stool box and pad into one block.", "Armwange und Rolle zu einer Armlehne; Hockerkasten und Polster zu einem Block."),
        omitted=T("The seat dish and the pad dip (below 30 mm), seams, quilting.", "Sitzmulde und Polsterdelle (unter 30 mm), Nähte, Steppung."),
        thinElements=T("The 12 mm blanket stays a filled strip under the 0.25 mm body pen.", "Die 12-mm-Decke bleibt ein gefüllter Streifen unter dem 0,25-mm-Körperstift."),
        hatching=T("None.", "Keine."),
        styles=T("Tones as at 1:50.", "Töne wie bei 1:50."),
        overlays=T("One shape per part with its region.", "Eine Form je Teil mit ihrer Region.")),
    500: dict(
        silhouette=T("Redrawn for 1:500 (75 mm): back, wings and arms as plain blocks, the seat a slab, four 75 mm leg blocks, the footstool one block down to the blanket, the blanket a 12 mm strip.", "Für 1:500 (75 mm) neu gezeichnet: Rücken, Ohren und Armlehnen als schlichte Blöcke, der Sitz eine Platte, vier 75-mm-Beinblöcke, der Hocker ein Block bis auf die Decke, die Decke ein 12-mm-Streifen."),
        retained=T("Recline of the back, the wing outline, seat and arm heights, the footstool beside the chair.", "Neigung des Rückens, der Ohrenumriss, Sitz- und Armhöhe, der Hocker neben dem Sessel."),
        merged=T("Chair legs thickened to 75 mm blocks; arm roll into the arm block; stool pad, box and legs into one block, because the 245 mm between its 75 mm feet would print as a 0.065 mm white gap at 1:1000.", "Sesselbeine zu 75-mm-Blöcken verdickt; Armrolle im Armblock; Hockerpolster, -kasten und -beine zu einem Block, weil die 245 mm zwischen 75-mm-Füßen bei 1:1000 als 0,065-mm-Lücke drucken würden."),
        omitted=T("Turning of the legs, crowns, roundings, every detail line.", "Drechselung der Beine, Bombierungen, Rundungen, jede Detaillinie."),
        thinElements=T("The blanket strip carries a 0.2 mm outline so it prints at 1:1000.", "Der Deckenstreifen hat einen 0,2-mm-Umriss, damit er bei 1:1000 druckt."),
        hatching=T("None.", "Keine."),
        styles=T("Tones as at 1:50.", "Töne wie bei 1:50."),
        overlays=T("One shape per part with its region.", "Eine Form je Teil mit ihrer Region.")),
}

VOL_DECISIONS = {
    50: [T("Body, authored for 7.5 mm: the folded quilted moving blanket (1.50 x 1.00 m, 12 mm, rounded fold) under chair and footstool; four turned beech legs (46 mm under the frame with a bead at 11-15 mm, 31 mm above the foot), the left front and both rear legs with tarnished brass ferrules, the right front one with a black plastic glide that replaced its lost ferrule; the seat box between the arms with its bulging front border; two arm panels 0.12 m wide with padded rolls widened to 0.13 m and rounded fronts at 0.85 m; the back reclined 6 degrees with its lumbar belly and rolled crest at 1.10 m; two curved wings 85 mm thick; the sprung seat with its 22 mm sag dish and 9 mm front piping; the footstool box and its crowned pad with a 9 mm dip on four turned legs; a cardboard price tag hanging off the right arm roll.", "Körper, für 7,5 mm gebaut: die gefaltete, gesteppte Möbeldecke (1,50 x 1,00 m, 12 mm, gerundeter Falz) unter Sessel und Hocker; vier gedrechselte Buchenbeine (unter dem Rahmen 46 mm mit Wulst bei 11-15 mm, über dem Fuß 31 mm), das linke vordere und beide hinteren mit angelaufenen Messingkappen, das rechte vordere mit einem schwarzen Kunststoffgleiter statt der verlorenen Kappe; der Sitzkasten zwischen den Armlehnen mit ausgebeulter Vorderkante; zwei Armwangen 0,12 m breit mit gepolsterten, auf 0,13 m verbreiterten Rollen und gerundeten Fronten bei 0,85 m; der um 6 Grad geneigte Rücken mit Lendenbauch und gerollter Kopfkante bei 1,10 m; zwei geschwungene Ohren 85 mm dick; der Federkernsitz mit 22-mm-Mulde und 9-mm-Frontkeder; Hockerkasten und bombiertes Polster mit 9-mm-Delle auf vier gedrechselten Beinen; ein Preisschild aus Karton an der rechten Armrolle."),
         T("Thermal: every covered part references its body solid in its region (seat and seat box, back with right arm and wing, the bleached left arm and wing, legs of chair and footstool, footstool covers, blanket); ferrules, glide and tag belong to no region. Structure: the hidden beech frame (four 25 x 100 mm rails, two arm posts and 30 x 60 mm arm rails, two 30 x 60 mm back posts with top and bottom rail, two 12 mm plywood wing boards), the spring unit on the rails, the legs and the blanket as their own members, joints at the leg tops, the back foot and the spring bearings, supports at the permanent contacts (blanket on the floor, footstool feet; the chair feet, the grips and the outer back are pose contacts that switch on and off), mass markers at the bone centres.", "Thermik: jedes bezogene Teil verweist auf seinen Körper in seiner Region (Sitz und Sitzkasten, Rücken mit rechter Armlehne und rechtem Ohr, die gebleichte linke Armlehne und das linke Ohr, Beine von Sessel und Hocker, Hockerbezüge, Decke); Kappen, Gleiter und Schild gehören zu keiner Region. Statik: das verdeckte Buchengestell (vier Zargen 25 x 100 mm, zwei Armpfosten und Armholme 30 x 60 mm, zwei Rückenpfosten 30 x 60 mm mit Kopf- und Fußholm, zwei Ohrenbretter aus 12-mm-Sperrholz), der Federkern auf den Zargen, Beine und Decke als eigene Stäbe, Gelenke an den Beinköpfen, am Rückenfuß und an den Federauflagen, Auflager an den ständigen Kontakten (Decke auf dem Boden, Hockerfüße; Sesselfüße, Griffe und Rückseite sind Posenkontakte, die an- und abschalten), Massenmarken in den Knochenschwerpunkten.")],
    100: [T("Body, authored anew for 15 mm: legs as four-point tapers with the ferrule in the outline (no bead), arm panels and rolls kept, back and wings with fewer stations, the seat dish (22 mm) kept, the footstool pad dip kept, the blanket a declared 12 mm sheet with a rounded fold; piping, price tag, ferrules and glide go (below 15 mm or flush).", "Körper, für 15 mm neu gebaut: Beine als Konen aus vier Punkten mit der Kappe im Umriss (ohne Wulst), Armwangen und Rollen bleiben, Rücken und Ohren mit weniger Stationen, die Sitzmulde (22 mm) bleibt, die Polsterdelle des Hockers bleibt, die Decke ein erklärtes 12-mm-Blatt mit gerundetem Falz; Keder, Preisschild, Kappen und Gleiter entfallen (unter 15 mm oder bündig)."),
          T("Thermal references the body per region; structure as at 1:50 with the 12 mm plywood stool top and wing boards declared as sheets.", "Thermik verweist je Region auf den Körper; Statik wie bei 1:50, Hockerplatte und Ohrenbretter aus 12-mm-Sperrholz als Blätter erklärt.")],
    200: [T("Body, authored anew for 30 mm: each arm is one rounded block (panel and roll merged, 0.13 m wide to 0.625 m with the front roll), the seat a crowned slab without dish, the back and the wings with six to nine stations, legs as plain tapers 46 to 33 mm, the footstool one block on four 32-40 mm legs, the blanket a 12 mm sheet.", "Körper, für 30 mm neu gebaut: jede Armlehne ein gerundeter Block (Wange und Rolle vereint, 0,13 m breit bis 0,625 m mit Frontrolle), der Sitz eine bombierte Platte ohne Mulde, Rücken und Ohren mit sechs bis neun Stationen, Beine als schlichte Konen 46 bis 33 mm, der Hocker ein Block auf vier Beinen 32-40 mm, die Decke ein 12-mm-Blatt."),
          T("Thermal references the body per region; structure: rails, posts and rails of the frame keep their true 25-60 mm sections as declared slender members, wing boards and blanket as sheets, the stool frame one 0.40 x 0.09 x 0.50 m member, joints and supports at least 30 mm.", "Thermik verweist je Region auf den Körper; Statik: Zargen, Pfosten und Holme behalten ihre wahren Querschnitte von 25-60 mm als erklärte schlanke Stäbe, Ohrenbretter und Decke als Blätter, das Hockergestell ein Stab 0,40 x 0,09 x 0,50 m, Gelenke und Auflager mindestens 30 mm.")],
    500: [T("Body, authored anew for 75 mm: back, wings and arms as plain blocks with the recline and the wing outline, the seat and seat box as two slabs, the legs thickened to four 75 mm blocks, the footstool one block down to the blanket (its leg space would print as a 0.065 mm gap at 1:1000), the blanket a 12 mm sheet. The model is minimal: nothing smaller can be left out without losing the chair, its footstool or its blanket.", "Körper, für 75 mm neu gebaut: Rücken, Ohren und Armlehnen als schlichte Blöcke mit Neigung und Ohrenumriss, Sitz und Sitzkasten als zwei Platten, die Beine zu vier 75-mm-Blöcken verdickt, der Hocker ein Block bis auf die Decke (sein Beinraum würde bei 1:1000 als 0,065-mm-Lücke drucken), die Decke ein 12-mm-Blatt. Das Modell ist minimal: nichts Kleineres kann entfallen, ohne Sessel, Hocker oder Decke zu verlieren."),
          T("Thermal references the blocks per region; structure: slender rails, posts and back rails at their true sections, sheets for wing boards and blanket, the leg blocks as leg members, the four stool legs as 75 mm members inside the stool block, 75 mm joints, supports and mass markers.", "Thermik verweist je Region auf die Blöcke; Statik: schlanke Zargen, Pfosten und Rückenholme in wahren Querschnitten, Blätter für Ohrenbretter und Decke, die Beinblöcke als Beinstäbe, die vier Hockerbeine als 75-mm-Stäbe im Hockerblock, 75-mm-Gelenke, -Auflager und -Massenmarken.")],
}


def assemble(scales, total):
    """🧩 The asset document from the scales."""
    reps, vols = {}, {}
    for den, s in scales.items():
        lo, hi = RANGES[den]
        reps[str(den)] = {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[den], "hatches": HATCHES[den], "geometry": s.views, "decisions": REP_DECISIONS[den]}
        thermal_list = [{"id": "th-" + sid, "of": sid, "region": reg} for sid, reg in s.regions.items()]
        vols[str(den)] = {"status": "authored", "denominator": den, "body": s.body, "thermal": thermal_list, "structure": structure(den, s.body), "decisions": VOL_DECISIONS[den]}
    asset = {
        "id": "used-wingback-armchair",
        "category": "furniture",
        "title": T("Used Wingback Armchair", "Gebrauchter Ohrensessel"),
        "summary": T("A second-hand West German wingback armchair of the 1960s with its matching footstool, as a flat clearance leaves it in the reuse depot: worn cotton velour bleached on the window side, a sprung seat sagged by 22 mm, turned beech legs with tarnished brass ferrules (one replaced by a plastic glide), a price tag on the arm; stored on a folded moving blanket, carried out by one person on his back and back in use with a 75 kg reader.", "Ein gebrauchter westdeutscher Ohrensessel der 1960er Jahre mit passendem Hocker, wie ihn eine Haushaltsauflösung im Wiederverwendungslager hinterlässt: abgewetzter Baumwollvelours, auf der Fensterseite ausgeblichen, ein um 22 mm durchgesessener Federkernsitz, gedrechselte Buchenbeine mit angelaufenen Messingkappen (eine durch einen Kunststoffgleiter ersetzt), ein Preisschild an der Armlehne; auf einer gefalteten Möbeldecke gelagert, von einer Person auf dem Rücken hinausgetragen und wieder im Gebrauch mit einer lesenden Person von 75 kg."),
        "tags": ["used", "household", "second-hand", "living", "armchair", "wingback", "seating", "upholstery", "velour", "beech", "footstool", "moving-blanket", "1960s"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES],
                       "notes": T("A 1960s wingback armchair 0.78 x 0.85 x 1.10 m (seat height 0.44 m at the front edge, 0.418 m in the sagged middle, seat depth 0.53 m, seat width 0.53 m between the arms, arm height 0.625 m) with its footstool 0.50 x 0.40 x 0.40 m, both standing on a folded moving blanket 1.50 x 1.00 x 0.012 m; the stored envelope is 1.50 x 1.00 x 1.112 m and 32.4 kg in all (chair 25.4 kg, footstool 5.3 kg, blanket 1.65 kg). Dimensions, masses and sink depths are authored estimates from catalogue-typical chairs of the period; standards give loads and material data. Persons are loads and supports, not drawn.", "Ein Ohrensessel der 1960er Jahre, 0,78 x 0,85 x 1,10 m (Sitzhöhe 0,44 m an der Vorderkante, 0,418 m in der durchgesessenen Mitte, Sitztiefe 0,53 m, Sitzbreite 0,53 m zwischen den Armlehnen, Armhöhe 0,625 m) mit seinem Hocker 0,50 x 0,40 x 0,40 m, beide auf einer gefalteten Möbeldecke 1,50 x 1,00 x 0,012 m; die Lagerhülle misst 1,50 x 1,00 x 1,112 m und wiegt insgesamt 32,4 kg (Sessel 25,4 kg, Hocker 5,3 kg, Decke 1,65 kg). Abmessungen, Massen und Einsinktiefen sind Schätzwerte nach katalogtypischen Sesseln der Zeit; Normen liefern Lasten und Stoffwerte. Personen sind Lasten und Auflager, nicht gezeichnet.")},
        "dimensions": {"heightM": 1.112, "widthM": 1.5, "depthM": 1.0, "massKg": total},
        "materials": materials(),
        "rig": {"kind": "structural", "root": "ground", "bones": BONES},
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(),
        "structure": structure_model(),
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    main()
