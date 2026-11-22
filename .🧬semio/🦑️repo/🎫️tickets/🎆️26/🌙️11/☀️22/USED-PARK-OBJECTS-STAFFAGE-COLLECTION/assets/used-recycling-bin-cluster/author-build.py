"""♻️ Authoritative generator of `used-recycling-bin-cluster`: a weathered three-bay recycling station of a German municipal park.

Every number below is hand-chosen in world metres (x ahead, y up, z left, ground y = 0) and converted to the bone frames; the script writes
`🔣️.json` next to itself. Run: `python3 author-build.py`.

@see ../../brief.md
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")


def T(en, de):
    """🌍 Bilingual text."""
    return {"en": en, "de": de}


def r6(v):
    """✂️ Rounds a number for the document."""
    return round(v + 0.0, 6)


PAD_X, PAD_Y, PAD_Z = 0.28, 0.08, 0.85
POSTS = {"post-r": -0.726, "post-cr": -0.242, "post-cl": 0.242, "post-l": 0.726}
BINS = {"bin-r": -0.484, "bin-c": 0.0, "bin-l": 0.484}
LID_OF = {"bin-r": "lid-r", "bin-c": "lid-c", "bin-l": "lid-l"}
PIVOT_Y = 0.88
HINGE = (-0.19, 0.962)
BOTTLE = (0.05, 0.08, 0.80)
POST_TOP = 1.10

BONES = [
    {"id": "pad", "parent": None, "title": T("Precast concrete pad, from its centre to the front edge", "Betonfertigteilplatte, von ihrer Mitte bis zur Vorderkante"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.28, "joint": {"type": "fixed"}},
]
POST_TITLES = {"post-r": ("Right end post", "Rechter Endpfosten"), "post-cr": ("Post between grey and blue bay", "Pfosten zwischen grauem und blauem Feld"), "post-cl": ("Post between blue and yellow bay", "Pfosten zwischen blauem und gelbem Feld"), "post-l": ("Left end post", "Linker Endpfosten")}
for pid, z in POSTS.items():
    BONES.append({"id": pid, "parent": "pad", "title": T(POST_TITLES[pid][0] + ", SHS 60 x 60 x 3 up from the base plate to the cap", POST_TITLES[pid][1] + ", Hohlprofil 60 x 60 x 3 von der Fußplatte bis zur Kappe"), "x": 0, "y": PAD_Y, "z": z, "angleDeg": 90, "lengthM": round(POST_TOP - PAD_Y, 6), "joint": {"type": "fixed"}})
BIN_TITLES = {"bin-r": ("Grey residual-waste tipping bin on its trunnion axis", "Grauer Restmüll-Kippbehälter an seiner Zapfenachse"), "bin-c": ("Blue paper tipping bin on its trunnion axis", "Blauer Papier-Kippbehälter an seiner Zapfenachse"), "bin-l": ("Yellow packaging tipping bin on its trunnion axis", "Gelber Verpackungs-Kippbehälter an seiner Zapfenachse")}
LID_TITLES = {"lid-r": ("Replaced dark grey lid on its rear hinge", "Ersetzter dunkelgrauer Deckel am hinteren Scharnier"), "lid-c": ("Faded blue lid on its rear hinge", "Ausgeblichener blauer Deckel am hinteren Scharnier"), "lid-l": ("Faded yellow lid on its rear hinge", "Ausgeblichener gelber Deckel am hinteren Scharnier")}
for bid, z in BINS.items():
    BONES.append({"id": bid, "parent": "pad", "title": T(BIN_TITLES[bid][0], BIN_TITLES[bid][1]), "x": 0, "y": PIVOT_Y, "z": z, "angleDeg": 0, "lengthM": 0.18, "joint": {"type": "revolute", "minDeg": -110, "maxDeg": 0}})
    lid = LID_OF[bid]
    BONES.append({"id": lid, "parent": bid, "title": T(LID_TITLES[lid][0], LID_TITLES[lid][1]), "x": HINGE[0], "y": round(HINGE[1] - PIVOT_Y, 6), "angleDeg": 0, "lengthM": 0.382, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 100}})
BONES.append({"id": "bottle", "parent": "pad", "title": T("Empty wine bottle left beside the station, up its axis", "Leere Weinflasche neben der Station abgestellt, entlang ihrer Achse"), "x": BOTTLE[0], "y": BOTTLE[1], "z": BOTTLE[2], "angleDeg": 90, "lengthM": 0.30, "joint": {"type": "fixed"}})

FRAMES = {}
for b in BONES:
    if b["parent"] is None:
        FRAMES[b["id"]] = (b["x"], b["y"], b.get("z", 0.0), b["angleDeg"])
    else:
        px, py, pz, pa = FRAMES[b["parent"]]
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        FRAMES[b["id"]] = (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"], pz + b.get("z", 0.0), pa + b["angleDeg"])
MOVING = {b["id"] for b in BONES if b["id"].startswith(("bin-", "lid-"))}


def local(bone, x, y):
    """📐 World (x, y) to the bone-local (u, v) of the rest frame."""
    ox, oy, _, a = FRAMES[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = x - ox, y - oy
    return (r6(c * dx + s * dy), r6(-s * dx + c * dy))


def lw(bone, z):
    """↔️ World z to the bone-local w."""
    return r6(z - FRAMES[bone][2])


def rect(x0, y0, x1, y1):
    """⬛ Counter-clockwise rectangle."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def rounded(a0, b0, a1, b1, r, n=4):
    """🔲 Counter-clockwise rounded rectangle in a profile plane."""
    pts = []
    for cx, cy, start in ((a1 - r, b0 + r, -90), (a1 - r, b1 - r, 0), (a0 + r, b1 - r, 90), (a0 + r, b0 + r, 180)):
        for k in range(n + 1):
            t = math.radians(start + 90 * k / n)
            pts.append((r6(cx + r * math.cos(t)), r6(cy + r * math.sin(t))))
    return pts


def circle(cx, cy, r, n=20):
    """⭕ Counter-clockwise circle polygon."""
    return [(r6(cx + r * math.cos(2 * math.pi * k / n)), r6(cy + r * math.sin(2 * math.pi * k / n))) for k in range(n)]


def ccw(poly):
    """🔄 Orients a polygon counter-clockwise."""
    a = sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly)))
    return poly if a > 0 else list(reversed(poly))


class Scale:
    """🗂️ One authored scale: body solids, the 2D shapes of the three views, thermal references and structure solids."""

    def __init__(self, denominator):
        self.d = denominator
        self.body, self.thermal, self.structure = [], [], []
        self.views = {"elevation": [], "front": [], "plan": []}
        self.order = 1.0

    def z(self):
        self.order += 0.01
        return round(self.order, 3)

    def shape(self, view, s):
        s.setdefault("z", self.z())
        self.views[view].append(s)

    def part(self, sid, bone, solid, region, material, role, tone, draw=None, sheet=False, texture=False, member=None, joint=None):
        s = {"id": sid, "bone": bone, "material": material, "role": role, "tone": tone}
        s.update(solid)
        if sheet:
            s["sheet"] = True
        if texture:
            s["texture"] = True
        self.body.append(s)
        self.thermal.append({"id": "th-" + sid, "of": sid, "region": region})
        if member:
            self.structure.append({"id": "m-" + sid, "of": sid, "member": member, "part": "member"})
        if joint:
            self.structure.append({"id": "j-" + sid, "of": sid, "member": joint, "part": "joint"})
        for view, shp in (draw or {}).items():
            base = {"id": sid, "bone": bone, "role": role, "filled": True, "tone": tone, "region": region, "material": material}
            base.update(shp)
            self.shape(view, base)

    def prism(self, sid, bone, poly, z0, z1, region, material, role, tone, views=("elevation", "front", "plan"), **kw):
        poly = ccw(poly)
        prof = [local(bone, x, y) for x, y in poly]
        solid = {"kind": "extrude", "plane": "xy", "profile": [list(p) for p in prof], "from": lw(bone, z0), "to": lw(bone, z1)}
        xs, ys = [p[0] for p in poly], [p[1] for p in poly]
        draw = {}
        if "elevation" in views:
            draw["elevation"] = {"kind": "poly", "points": [list(p) for p in prof], "closed": True}
        for v in ("front", "plan"):
            if v not in views:
                continue
            if bone in MOVING:
                draw[v] = {"kind": "poly", "points": [list(p) for p in prof], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}}
            elif v == "front":
                draw[v] = {"kind": "rect", "x": r6(z0), "y": r6(min(ys)), "w": r6(z1 - z0), "h": r6(max(ys) - min(ys))}
            else:
                draw[v] = {"kind": "rect", "x": r6(min(xs)), "y": r6(z0), "w": r6(max(xs) - min(xs)), "h": r6(z1 - z0)}
        self.part(sid, bone, solid, region, material, role, tone, draw, **kw)

    def cylw(self, sid, bone, cx, cy, r, z0, z1, region, material, role, tone, views=("elevation", "front", "plan"), **kw):
        u, v = local(bone, cx, cy)
        solid = {"kind": "extrude", "plane": "xy", "profile": [list(p) for p in circle(u, v, r)], "from": lw(bone, z0), "to": lw(bone, z1)}
        draw = {}
        if "elevation" in views:
            draw["elevation"] = {"kind": "ellipse", "cx": u, "cy": v, "rx": r6(r), "ry": r6(r)}
        for vw in ("front", "plan"):
            if vw not in views:
                continue
            if bone in MOVING:
                draw[vw] = {"kind": "ellipse", "cx": u, "cy": v, "rx": r6(r), "ry": r6(r), "solid": {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}}
            elif vw == "front":
                draw[vw] = {"kind": "rect", "x": r6(z0), "y": r6(cy - r), "w": r6(z1 - z0), "h": r6(2 * r)}
            else:
                draw[vw] = {"kind": "rect", "x": r6(cx - r), "y": r6(z0), "w": r6(2 * r), "h": r6(z1 - z0)}
        self.part(sid, bone, solid, region, material, role, tone, draw, **kw)

    def tub(self, sid, bone, x0, x1, y0, y1, z0, z1, wall, rad, region, material, role, tone, views=("elevation", "front", "plan"), **kw):
        w0, w1 = lw(bone, z0), lw(bone, z1)
        u0, _ = local(bone, x0, y0)
        u1, _ = local(bone, x1, y0)
        _, v0 = local(bone, x0, y0)
        _, v1 = local(bone, x0, y1)
        outer = rounded(w0, u0, w1, u1, rad)
        inner = rounded(w0 + wall, u0 + wall, w1 - wall, u1 - wall, rad - wall)
        solid = {"kind": "extrude", "plane": "zx", "profile": [list(p) for p in outer], "holes": [[list(p) for p in inner]], "from": v0, "to": v1}
        prof = [list(local(bone, x, y)) for x, y in rect(x0, y0, x1, y1)]
        draw = {}
        if "elevation" in views:
            draw["elevation"] = {"kind": "poly", "points": prof, "closed": True}
        for vw in ("front", "plan"):
            if vw in views:
                draw[vw] = {"kind": "poly", "points": prof, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}}
        self.part(sid, bone, solid, region, material, role, tone, draw, **kw)

    def vtube(self, sid, bone, y0, y1, half, wall, region, material, role, tone, hollow=True, **kw):
        bx, by, bz, _ = FRAMES[bone]
        outer = rounded(-half, -half, half, half, 2 * wall if hollow else 0.004)
        solid = {"kind": "extrude", "plane": "yz", "profile": [list(p) for p in outer], "from": r6(y0 - by), "to": r6(y1 - by)}
        if hollow:
            solid["holes"] = [[list(p) for p in rounded(-half + wall, -half + wall, half - wall, half - wall, wall)]]
        draw = {
            "elevation": {"kind": "poly", "points": [list(local(bone, x, y)) for x, y in rect(bx - half, y0, bx + half, y1)], "closed": True},
            "front": {"kind": "rect", "x": r6(bz - half), "y": r6(y0), "w": r6(2 * half), "h": r6(y1 - y0)},
            "plan": {"kind": "rect", "x": r6(bx - half), "y": r6(bz - half), "w": r6(2 * half), "h": r6(2 * half)},
        }
        self.part(sid, bone, solid, region, material, role, tone, draw, **kw)

    def bottle(self, sid, profile, region, material, role, tone, **kw):
        bx, by, bz, _ = FRAMES["bottle"]
        solid = {"kind": "revolve", "profile": [[r6(h), r6(r)] for h, r in profile], "axis": [1, 0, 0], "origin": [0, 0, 0]}
        half = [(h, r) for h, r in profile if r > 0]
        ring = [(h, r) for h, r in half] + [(h, -r) for h, r in reversed(half)]
        el = [[r6(h), r6(-r)] for h, r in ring]
        fr = [[r6(bz + r), r6(by + h)] for h, r in ring]
        rmax = max(r for _, r in profile)
        draw = {"elevation": {"kind": "poly", "points": ccw(el), "closed": True}, "front": {"kind": "poly", "points": ccw(fr), "closed": True}, "plan": {"kind": "ellipse", "cx": r6(bx), "cy": r6(bz), "rx": r6(rmax), "ry": r6(rmax)}}
        self.part(sid, "bottle", solid, region, material, role, tone, draw, member="bottle", **kw)

    def stroke(self, view, sid, bone, pts, role="texture", tone=0.3, closed=False, filled=False, region=None, pen=None, world=False):
        if view == "elevation" and world:
            pts = [local(bone, x, y) for x, y in pts]
        if role == "texture" and self.d >= 200:
            role = "detail"
        s = {"id": sid, "bone": bone, "role": role, "filled": filled, "tone": tone, "kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}
        if region:
            s["region"] = region
        if pen:
            s["penMm"] = pen
        self.shape(view, s)


MAT = [
    {"id": "concrete-weathered", "title": T("Precast concrete C30/37, weathered", "Betonfertigteil C30/37, verwittert"), "emissivity": 0.94, "solarAbsorptivity": 0.68, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 3.3e10, "strengthPa": 3.0e7, "source": "literature", "reference": "levinson-akbari-2002"},
    {"id": "steel-powder-coated", "title": T("Hot-dip galvanised S235 steel, anthracite powder coat, chalked", "Feuerverzinkter Stahl S235, anthrazit pulverbeschichtet, abgekreidet"), "emissivity": 0.9, "solarAbsorptivity": 0.86, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "berdahl-2008"},
    {"id": "steel-rusted", "title": T("Steel with flaked coating and surface rust", "Stahl mit abgeplatzter Beschichtung und Flugrost"), "emissivity": 0.82, "solarAbsorptivity": 0.82, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "literature", "reference": "incropera-2018"},
    {"id": "hdpe-yellow-faded", "title": T("HDPE, signal yellow, UV-faded and scratched", "HDPE, signalgelb, UV-ausgeblichen und verkratzt"), "emissivity": 0.92, "solarAbsorptivity": 0.42, "densityKgM3": 950, "conductivityWmK": 0.42, "specificHeatJKgK": 1900, "elasticModulusPa": 1.0e9, "strengthPa": 2.5e7, "source": "literature", "reference": "ashby-2011"},
    {"id": "hdpe-blue-faded", "title": T("HDPE, gentian blue, UV-faded and scratched", "HDPE, enzianblau, UV-ausgeblichen und verkratzt"), "emissivity": 0.92, "solarAbsorptivity": 0.74, "densityKgM3": 950, "conductivityWmK": 0.42, "specificHeatJKgK": 1900, "elasticModulusPa": 1.0e9, "strengthPa": 2.5e7, "source": "literature", "reference": "ashby-2011"},
    {"id": "hdpe-grey-faded", "title": T("HDPE, slate grey, UV-faded and scratched", "HDPE, schiefergrau, UV-ausgeblichen und verkratzt"), "emissivity": 0.92, "solarAbsorptivity": 0.72, "densityKgM3": 950, "conductivityWmK": 0.42, "specificHeatJKgK": 1900, "elasticModulusPa": 1.0e9, "strengthPa": 2.5e7, "source": "literature", "reference": "ashby-2011"},
    {"id": "hdpe-grey-new", "title": T("HDPE, anthracite grey, new replacement part", "HDPE, anthrazitgrau, neues Ersatzteil"), "emissivity": 0.92, "solarAbsorptivity": 0.86, "densityKgM3": 950, "conductivityWmK": 0.42, "specificHeatJKgK": 1900, "elasticModulusPa": 1.0e9, "strengthPa": 2.5e7, "source": "literature", "reference": "ashby-2011"},
    {"id": "glass-green", "title": T("Green soda-lime container glass", "Grünes Kalk-Natron-Behälterglas"), "emissivity": 0.92, "solarAbsorptivity": 0.55, "densityKgM3": 2500, "conductivityWmK": 1.0, "specificHeatJKgK": 750, "elasticModulusPa": 7.0e10, "source": "literature", "reference": "en-572-1-2012"},
]
DENS = {m["id"]: m["densityKgM3"] for m in MAT}

REGIONS = {
    "pad-concrete": ("Concrete pad, weathered, gum spots and moss on the shaded rear edge", "Betonplatte, verwittert, Kaugummiflecken und Moos an der schattigen Hinterkante", "surface", "concrete-weathered", "horizontal-plate-up", 0.95, 0.08, 2.0, "masonry", 0.28),
    "frame-coated": ("Powder-coated posts, caps and trunnion pins, chalked and scratched", "Pulverbeschichtete Pfosten, Kappen und Lagerzapfen, abgekreidet und verkratzt", "surface", "steel-powder-coated", "vertical-plate", 0.5, 0.003, 50, "metal", 0.06),
    "post-feet-rust": ("Post feet, base plates and dowel nuts with flaked coating and rust", "Pfostenfüße, Fußplatten und Dübelmuttern mit abgeplatzter Beschichtung und Rost", "surface", "steel-rusted", "vertical-plate", 0.6, 0.008, 50, "metal", 0.14),
    "bin-yellow": ("Yellow packaging tub, faded and scratched", "Gelber Verpackungsbehälter, ausgeblichen und verkratzt", "surface", "hdpe-yellow-faded", "vertical-plate", 0.5, 0.004, 0.42, "polymer", 0.5),
    "bin-blue": ("Blue paper tub, faded, graffiti tag on the front", "Blauer Papierbehälter, ausgeblichen, Graffiti-Tag vorne", "surface", "hdpe-blue-faded", "vertical-plate", 0.5, 0.004, 0.42, "polymer", 0.5),
    "bin-grey": ("Grey residual-waste tub, faded, mower dent at the front corner", "Grauer Restmüllbehälter, ausgeblichen, Mäherdelle an der Vorderecke", "surface", "hdpe-grey-faded", "vertical-plate", 0.5, 0.004, 0.42, "polymer", 0.5),
    "lid-yellow": ("Yellow lid, faded, sticker residue", "Gelber Deckel, ausgeblichen, Aufkleberreste", "surface", "hdpe-yellow-faded", "horizontal-plate-up", 0.9, 0.005, 0.42, "polymer", 0.38),
    "lid-blue": ("Blue lid, faded and scratched", "Blauer Deckel, ausgeblichen und verkratzt", "surface", "hdpe-blue-faded", "horizontal-plate-up", 0.9, 0.005, 0.42, "polymer", 0.38),
    "lid-grey-new": ("Replaced anthracite lid, unweathered", "Ersetzter anthrazitfarbener Deckel, unverwittert", "surface", "hdpe-grey-new", "horizontal-plate-up", 0.9, 0.005, 0.42, "polymer", 0.38),
    "bottle-glass": ("Empty green wine bottle beside the station", "Leere grüne Weinflasche neben der Station", "surface", "glass-green", "cylinder", 0.5, 0.003, 1.0, "glass", 0.077),
}
AREA = {"pad-concrete": 2.27, "frame-coated": 0.91, "post-feet-rust": 0.27, "bin-yellow": 0.91, "bin-blue": 0.91, "bin-grey": 0.91, "lid-yellow": 0.28, "lid-blue": 0.28, "lid-grey-new": 0.28, "bottle-glass": 0.07}
AREA_FILE = os.path.join(HERE, "author-areas.json")
if os.path.exists(AREA_FILE):
    AREA.update(json.load(open(AREA_FILE)))
BIN_REGION = {"bin-r": ("bin-grey", "hdpe-grey-faded", 0.42), "bin-c": ("bin-blue", "hdpe-blue-faded", 0.38), "bin-l": ("bin-yellow", "hdpe-yellow-faded", 0.82)}
LID_REGION = {"lid-r": ("lid-grey-new", "hdpe-grey-new", 0.24), "lid-c": ("lid-blue", "hdpe-blue-faded", 0.5), "lid-l": ("lid-yellow", "hdpe-yellow-faded", 0.88)}
FRAME_TONE, RUST_TONE, PAD_TONE, GLASS_TONE = 0.22, 0.32, 0.72, 0.4

BOTTLE_FINE = [(0.0, 0.0), (0.0, 0.034), (0.004, 0.0385), (0.198, 0.0385), (0.212, 0.035), (0.228, 0.022), (0.240, 0.0148), (0.284, 0.0148), (0.287, 0.0158), (0.300, 0.0158), (0.300, 0.0)]
BOTTLE_100 = [(0.0, 0.0), (0.0, 0.0385), (0.200, 0.0385), (0.235, 0.0175), (0.300, 0.0175), (0.300, 0.0)]
BOTTLE_200 = [(0.0, 0.0), (0.0, 0.0385), (0.205, 0.0385), (0.240, 0.0165), (0.300, 0.0165), (0.300, 0.0)]
BOTTLE_500 = [(0.0, 0.0), (0.0, 0.0385), (0.300, 0.0385), (0.300, 0.0)]


def build_scale(d):
    """🧱 Builds the handcrafted parts of one scale."""
    S = Scale(d)
    pad_poly = [(-PAD_X, 0), (PAD_X, 0), (PAD_X, 0.07), (0.27, PAD_Y), (-0.27, PAD_Y), (-PAD_X, 0.07)] if d <= 100 else rect(-PAD_X, 0, PAD_X, PAD_Y)
    S.prism("pad", "pad", pad_poly, -PAD_Z, PAD_Z, "pad-concrete", "concrete-weathered", "body", PAD_TONE, member="pad")
    half = 0.03 if d < 500 else 0.04
    for pid, zc in POSTS.items():
        if d == 50:
            S.prism(pid + "-plate", pid, rect(-0.07, PAD_Y, 0.07, 0.088), zc - 0.07, zc + 0.07, "post-feet-rust", "steel-rusted", "body", RUST_TONE, joint=pid)
            for k, (dx, dz) in enumerate(((-0.05, -0.05), (0.05, -0.05), (-0.05, 0.05), (0.05, 0.05))):
                S.prism(f"{pid}-nut-{k}", pid, rect(dx - 0.0095, 0.088, dx + 0.0095, 0.098), zc + dz - 0.0095, zc + dz + 0.0095, "post-feet-rust", "steel-rusted", "detail", 0.28, views=("elevation", "front", "plan") if dz < 0 else ("plan",))
            S.vtube(pid + "-foot", pid, 0.088, 0.18, half, 0.003, "post-feet-rust", "steel-rusted", "body", RUST_TONE, member=pid)
            S.vtube(pid + "-shaft", pid, 0.18, 1.092, half, 0.003, "frame-coated", "steel-powder-coated", "body", FRAME_TONE, member=pid)
            S.prism(pid + "-cap", pid, rect(-half, 1.092, half, POST_TOP), zc - half, zc + half, "frame-coated", "steel-powder-coated", "body", 0.18)
        elif d == 100:
            S.prism(pid + "-plate", pid, rect(-0.07, PAD_Y, 0.07, 0.095), zc - 0.07, zc + 0.07, "post-feet-rust", "steel-rusted", "body", RUST_TONE, joint=pid)
            S.vtube(pid + "-foot", pid, 0.095, 0.18, half, 0.003, "post-feet-rust", "steel-rusted", "body", RUST_TONE, member=pid)
            S.vtube(pid + "-shaft", pid, 0.18, 1.085, half, 0.003, "frame-coated", "steel-powder-coated", "body", FRAME_TONE, member=pid)
            S.prism(pid + "-cap", pid, rect(-half, 1.085, half, POST_TOP), zc - half, zc + half, "frame-coated", "steel-powder-coated", "body", 0.18)
        else:
            S.vtube(pid + "-foot", pid, PAD_Y, 0.18, half, 0.003, "post-feet-rust", "steel-rusted", "body", RUST_TONE, hollow=False, member=pid)
            S.vtube(pid + "-shaft", pid, 0.18, POST_TOP, half, 0.003, "frame-coated", "steel-powder-coated", "body", FRAME_TONE, hollow=False, member=pid)
    for bid, zc in BINS.items():
        region, mat, tone = BIN_REGION[bid]
        lid = LID_OF[bid]
        lregion, lmat, ltone = LID_REGION[lid]
        if d <= 100:
            top = 0.96 if d == 50 else 0.935
            S.tub(bid + "-tub", bid, -0.18, 0.18, 0.46, top, zc - 0.20, zc + 0.20, 0.004, 0.03, region, mat, "body", tone, member=bid)
            S.prism(bid + "-floor", bid, rect(-0.176, 0.46, 0.176, 0.464), zc - 0.196, zc + 0.196, region, mat, "body", tone, views=(), sheet=True)
            if d == 50:
                for side, (a, b) in (("l", (0.20, 0.208)), ("r", (-0.208, -0.20))):
                    S.cylw(f"{bid}-boss-{side}", bid, 0.0, PIVOT_Y, 0.025, zc + a, zc + b, region, mat, "detail", tone - 0.08, views=("elevation",))
            for side, (a, b) in (("l", (0.198, 0.215)), ("r", (-0.215, -0.198))):
                S.cylw(f"{bid}-pin-{side}", bid, 0.0, PIVOT_Y, 0.012, zc + a, zc + b, "frame-coated", "steel-powder-coated", "detail", 0.2, views=("elevation",), joint=bid)
            if d == 50:
                S.prism(lid + "-plate", lid, rect(-0.192, 0.960, 0.192, 0.965), zc - 0.208, zc + 0.208, lregion, lmat, "body", ltone, sheet=True, member=lid)
                w0, w1 = -0.208, 0.208
                u = lambda x: r6(x - HINGE[0])
                skirt = [(w0, u(-0.18)), (w1, u(-0.18)), (w1, u(0.192)), (w0, u(0.192))]
                skirt = [(w0, u(-0.18)), (w0 + 0.004, u(-0.18)), (w0 + 0.004, u(0.188)), (w1 - 0.004, u(0.188)), (w1 - 0.004, u(-0.18)), (w1, u(-0.18)), (w1, u(0.192)), (w0, u(0.192))]
                sk = {"kind": "extrude", "plane": "zx", "profile": [[r6(a), r6(b)] for a, b in ccw(skirt)], "from": r6(0.935 - HINGE[1]), "to": r6(0.960 - HINGE[1])}
                prof = [list(local(lid, x, y)) for x, y in rect(-0.18, 0.935, 0.192, 0.960)]
                S.part(lid + "-skirt", lid, sk, lregion, lmat, "body", ltone - 0.04, {"elevation": {"kind": "poly", "points": prof, "closed": True}, "front": {"kind": "poly", "points": prof, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r6(zc - 0.208), "z1": r6(zc + 0.208)}}}, member=lid)
            else:
                S.prism(lid + "-plate", lid, rect(-0.192, 0.935, 0.192, 0.965), zc - 0.208, zc + 0.208, lregion, lmat, "body", ltone, member=lid)
            S.prism(lid + "-grip", lid, rect(0.192, 0.930, 0.207, 0.950), zc - 0.08, zc + 0.08, lregion, lmat, "detail", ltone - 0.1)
            S.cylw(lid + "-barrel", lid, HINGE[0], HINGE[1], 0.01, zc - 0.18, zc + 0.18, lregion, lmat, "detail", ltone - 0.1, views=("elevation", "plan"), joint=lid)
        else:
            split = 0.935 if d == 200 else 0.89
            wide = 0.212 if d == 200 else 0.202
            S.prism(bid + "-tub", bid, rect(-0.18, 0.46, 0.18, split), zc - wide, zc + wide, region, mat, "body", tone, member=bid)
            S.prism(lid + "-plate", lid, rect(-0.192, split, 0.192, 0.965), zc - 0.208 if d == 200 else zc - wide, zc + 0.208 if d == 200 else zc + wide, lregion, lmat, "body", ltone, member=lid)
    prof = {50: BOTTLE_FINE, 100: BOTTLE_100, 200: BOTTLE_200, 500: BOTTLE_500}[d]
    S.bottle("bottle", prof, "bottle-glass", "glass-green", "body", GLASS_TONE)
    return S


ANCHORS = [
    {"id": "pad-rear-right", "bone": "pad", "x": -0.2, "y": 0.0, "z": -0.7, "kind": "contact", "title": T("Pad bearing, rear right", "Plattenauflager hinten rechts"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.16, "footprintWidthM": 0.3, "frictionCoefficient": 0.6},
    {"id": "pad-rear-left", "bone": "pad", "x": -0.2, "y": 0.0, "z": 0.7, "kind": "contact", "title": T("Pad bearing, rear left", "Plattenauflager hinten links"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.16, "footprintWidthM": 0.3, "frictionCoefficient": 0.6},
    {"id": "pad-front-right", "bone": "pad", "x": 0.2, "y": 0.0, "z": -0.7, "kind": "contact", "title": T("Pad bearing, front right", "Plattenauflager vorne rechts"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.16, "footprintWidthM": 0.3, "frictionCoefficient": 0.6},
    {"id": "pad-front-left", "bone": "pad", "x": 0.2, "y": 0.0, "z": 0.7, "kind": "contact", "title": T("Pad bearing, front left", "Plattenauflager vorne links"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.16, "footprintWidthM": 0.3, "frictionCoefficient": 0.6},
]
SUPPORTS = [a["id"] for a in ANCHORS]


def structure_extras(S):
    """🏗️ Supports at the pad bearings, joint stand-ins where the scale omits the real joint parts, and mass markers."""
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[S.d]
    for a in ANCHORS:
        S.structure.append({"id": "s-" + a["id"], "kind": "box", "bone": "pad", "material": "concrete-weathered", "role": "detail", "tone": 0.1, "size": [0.16, r6(f), 0.3], "transform": {"translation": [a["x"], r6(f / 2), a["z"]]}, "member": "pad", "part": "support", "anchor": a["id"]})
    if S.d >= 200:
        for pid, zc in POSTS.items():
            S.structure.append({"id": "j-" + pid + "-base", "kind": "box", "bone": pid, "material": "steel-rusted", "role": "detail", "tone": 0.2, "size": [r6(f), r6(f), r6(f)], "transform": {"translation": [r6(f / 2), 0, 0]}, "member": pid, "part": "joint"})
        for bid, zc in BINS.items():
            for side in (1, -1):
                post = min(POSTS, key=lambda p: abs(POSTS[p] - (zc + side * 0.242)))
                S.structure.append({"id": f"j-{bid}-trunnion-{'l' if side > 0 else 'r'}", "kind": "box", "bone": bid, "material": "steel-powder-coated", "role": "detail", "tone": 0.2, "size": [r6(f), r6(f), r6(f)], "transform": {"translation": [0, 0, r6(side * (0.242 - (0.03 if S.d < 500 else 0.04)) - side * f / 2 + side * 0.0)]}, "member": bid, "part": "joint"})
    rm = max(f * 0.7, 0.012)
    for bone, (cx, cy) in MASS_CENTRES.items():
        S.structure.append({"id": "mass-" + bone, "kind": "ellipsoid", "bone": bone, "material": "steel-powder-coated", "role": "detail", "tone": 0.15, "radii": [r6(rm), r6(rm), r6(rm)], "transform": {"translation": [cx, cy, 0]}, "member": bone, "part": "mass"})


def solid_volume(s):
    """📦 Volume of an extrude or revolve body solid (for the masses at 1:50)."""
    def area(p):
        return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2
    if s["kind"] == "extrude":
        a = area(s["profile"]) - sum(area(h) for h in s.get("holes", []))
        return a * abs(s["to"] - s["from"])
    if s["kind"] == "revolve":
        p = s["profile"]
        v = 0.0
        for (h0, r0), (h1, r1) in zip(p, p[1:]):
            v += math.pi * (h1 - h0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
        return abs(v)
    raise ValueError(s["kind"])


def centroid(s):
    """🎯 Bone-local (u, v) centroid of an extrude or revolve solid."""
    if s["kind"] == "revolve":
        p = s["profile"]
        num = den = 0.0
        for (h0, r0), (h1, r1) in zip(p, p[1:]):
            dv = math.pi * (h1 - h0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
            num += dv * (h0 + h1) / 2
            den += dv
        return (num / den if den else 0.0, 0.0)
    if s["plane"] == "xy":
        p = s["profile"]
        return (sum(q[0] for q in p) / len(p), sum(q[1] for q in p) / len(p))
    if s["plane"] == "zx":
        p = s["profile"]
        return (sum(q[1] for q in p) / len(p), (s["from"] + s["to"]) / 2)
    return ((s["from"] + s["to"]) / 2, 0.0)


FINE = build_scale(50)
MASSES = {}
for s in FINE.body:
    m = solid_volume(s) * DENS[s["material"]]
    b = s["bone"]
    cu, cv = centroid(s)
    tot, su, sv = MASSES.get(b, (0.0, 0.0, 0.0))
    MASSES[b] = (tot + m, su + m * cu, sv + m * cv)
BOTTLE_GLASS_KG = 0.45
MASSES["bottle"] = (BOTTLE_GLASS_KG, BOTTLE_GLASS_KG * 0.12, 0.0)
MASS_CENTRES = {b: (r6(su / tot), r6(sv / tot)) for b, (tot, su, sv) in MASSES.items()}
TOTAL_KG = round(sum(t for t, _, _ in MASSES.values()), 2)
for b in BONES:
    if b["id"] in MASSES:
        kg, su, sv = MASSES[b["id"]]
        b["mass"] = {"kg": round(kg, 3), "comX": MASS_CENTRES[b["id"]][0], "comY": MASS_CENTRES[b["id"]][1], "distribution": "point", "source": "literature" if b["id"] != "bottle" else "authored-estimate", "reference": {"pad": "en-1991-1-1-2002", "bottle": "ashby-2011"}.get(b["id"], "en-10219-2-2019" if b["id"].startswith("post") else "ashby-2011")}
    if b["id"].startswith("post-"):
        b["section"] = {"areaM2": 0.000661, "secondMomentM4": 3.49e-07, "extremeFibreM": 0.03, "material": "steel-powder-coated", "title": T("SHS 60 x 60 x 3 (EN 10219-2)", "Quadratrohr 60 x 60 x 3 (EN 10219-2)")}
    if b["id"] == "pad":
        b["section"] = {"areaM2": 0.136, "secondMomentM4": 7.253e-05, "extremeFibreM": 0.04, "material": "concrete-weathered", "title": T("Precast pad 1.70 x 0.08 m", "Fertigteilplatte 1,70 x 0,08 m")}


def traces(S):
    """🩹 Traces of use and weather drawn as detail and texture strokes where they are below the smallest modelled feature."""
    fine = S.d == 50
    S.stroke("elevation", "rust-streak-0", "post-r", [(-0.026, 0.10), (-0.024, 0.16), (-0.027, 0.21)], world=True, region="post-feet-rust")
    S.stroke("elevation", "rust-streak-1", "post-r", [(0.012, 0.098), (0.015, 0.15)], world=True, region="post-feet-rust")
    S.stroke("elevation", "moss-rear", "pad", [(-0.278, 0.072), (-0.272, 0.079), (-0.262, 0.080), (-0.250, 0.080)], region="pad-concrete")
    S.stroke("elevation", "scratch-side-0", "bin-r", [(-0.12, 0.62), (0.05, 0.70)], world=True, region="bin-grey")
    S.stroke("elevation", "scratch-side-1", "bin-r", [(-0.06, 0.54), (0.10, 0.58)], world=True, region="bin-grey")
    S.stroke("elevation", "dent-side", "bin-r", [(0.18, 0.50), (0.172, 0.53), (0.18, 0.56)], role="detail", world=True, region="bin-grey")
    for bid, zc in BINS.items():
        region = BIN_REGION[bid][0]
        S.stroke("front", f"label-{bid}", bid, [(zc - 0.09, 0.80), (zc + 0.09, 0.80), (zc + 0.09, 0.88 - 0.01), (zc - 0.09, 0.87)], role="detail", closed=True, region=region)
        S.stroke("front", f"scratch-{bid}-0", bid, [(zc - 0.15, 0.58), (zc - 0.02, 0.66)], region=region)
        if fine:
            S.stroke("front", f"scratch-{bid}-1", bid, [(zc + 0.04, 0.52), (zc + 0.16, 0.55)], region=region)
    S.stroke("front", "graffiti-bin-c", "bin-c", [(-0.12, 0.66), (-0.08, 0.72), (-0.04, 0.65), (0.0, 0.73), (0.04, 0.66), (0.09, 0.71), (0.13, 0.68)], region="bin-blue")
    S.stroke("front", "sticker-bin-l", "bin-l", [(0.36, 0.70), (0.42, 0.71), (0.415, 0.75), (0.355, 0.745)], closed=True, region="bin-yellow")
    S.stroke("front", "dent-bin-r", "bin-r", [(-0.684, 0.47), (-0.66, 0.50), (-0.63, 0.47)], role="detail", region="bin-grey")
    for pid, zc in POSTS.items():
        S.stroke("front", f"rust-{pid}", pid, [(zc - 0.02, 0.10), (zc - 0.018, 0.16)], region="post-feet-rust")
    S.stroke("front", "stain-pad", "pad", [(-0.6, 0.02), (-0.4, 0.03), (-0.1, 0.018), (0.3, 0.03), (0.6, 0.02)], region="pad-concrete")
    S.stroke("plan", "moss-pad-rear", "pad", [(-0.272, -0.80), (-0.272, -0.3), (-0.268, 0.2), (-0.272, 0.80)], region="pad-concrete")
    gum = [(0.22, -0.55), (0.235, -0.12), (0.21, 0.18), (0.245, 0.62), (0.23, 0.30)] if fine else [(0.22, -0.55), (0.21, 0.18), (0.245, 0.62)]
    for k, (x, z) in enumerate(gum):
        S.shape("plan", {"id": f"gum-{k}", "bone": "pad", "role": "texture" if S.d < 200 else "detail", "filled": False, "tone": 0.3, "kind": "ellipse", "cx": x, "cy": z, "rx": 0.009, "ry": 0.009, "region": "pad-concrete"})
    for lid, (lregion, _, _) in LID_REGION.items():
        zc = BINS["bin-" + lid.split("-")[1]]
        if lid != "lid-r":
            S.stroke("plan", f"pictogram-{lid}", lid, [(-0.02, zc - 0.07), (0.12, zc - 0.07), (0.12, zc + 0.07), (-0.02, zc + 0.07)], role="detail", closed=True, region=lregion)
    if fine:
        S.stroke("plan", "sticker-lid-l", "lid-l", [(-0.12, 0.60), (-0.07, 0.61), (-0.075, 0.645), (-0.125, 0.64)], closed=True, region="lid-yellow")
    S.stroke("plan", "scratch-lid-c", "lid-c", [(-0.14, 0.09), (0.10, -0.06)], region="lid-blue")


SUN = {
    "in-service": {"pad-concrete": (1, 0.7), "frame-coated": (1, 0.45), "post-feet-rust": (1, 0.35), "bin-yellow": (1, 0.45), "bin-blue": (1, 0.45), "bin-grey": (1, 0.45), "lid-yellow": (1, 0.95), "lid-blue": (1, 0.95), "lid-grey-new": (1, 0.95), "bottle-glass": (1, 0.5)},
    "maintenance": {"pad-concrete": (1, 0.65), "frame-coated": (1, 0.45), "post-feet-rust": (1, 0.35), "bin-yellow": (1, 0.45), "bin-blue": (1, 0.55), "bin-grey": (1, 0.45), "lid-yellow": (1, 0.95), "lid-blue": (1, 0.5), "lid-grey-new": (1, 0.95), "bottle-glass": (1, 0.5)},
    "in-use": {"pad-concrete": (1, 0.6), "frame-coated": (1, 0.45), "post-feet-rust": (1, 0.3), "bin-yellow": (1, 0.45), "bin-blue": (1, 0.45), "bin-grey": (1, 0.45), "lid-yellow": (1, 0.55), "lid-blue": (1, 0.95), "lid-grey-new": (1, 0.95), "bottle-glass": (1, 0.4)},
}


def binding(pose):
    """🌞 Thermal binding of a pose: every region exposed, sunlit share per orientation."""
    return {"regions": {k: {"exposedFraction": e, "sunlitFraction": s} for k, (e, s) in SUN[pose].items()}, "emitters": {}}


CONTENTS = {"bin-l": (4.0, "Lightweight packaging (4 kg)", "Leichtverpackungen (4 kg)"), "bin-c": (9.0, "Paper and cardboard (9 kg)", "Papier und Pappe (9 kg)"), "bin-r": (14.0, "Residual waste (14 kg)", "Restmüll (14 kg)")}


def contents(bins):
    """🗑️ Waste contents as live loads at the centre of each tub floor."""
    out = []
    for bid in bins:
        kg, en, de = CONTENTS[bid]
        out.append({"id": "contents-" + bid, "kind": "live", "title": T(en, de), "bone": bid, "x": 0.0, "y": -0.30, "fxN": 0.0, "fyN": r6(-kg * 9.80665), "source": "authored-estimate"})
    return out


POSES = [
    {"id": "in-service", "title": T("In service", "Im Einsatz"), "description": T("All three tipping bins hang closed between the posts on the dowelled pad; the empty bottle stands beside the left end post because glass belongs in the bottle bank.", "Alle drei Kippbehälter hängen geschlossen zwischen den Pfosten auf der verdübelten Platte; die leere Flasche steht neben dem linken Endpfosten, weil Glas in den Altglascontainer gehört."), "bones": {}, "ground": "auto", "contacts": SUPPORTS, "thermal": binding("in-service"), "loads": []},
    {"id": "maintenance", "title": T("Emptying by the depot", "Leerung durch den Bauhof"), "description": T("The depot worker has unlocked the blue paper bin and tipped it forward 105 degrees on its trunnions; he holds the lid open at 95 degrees with his other hand and the paper falls into the collection sack, the other two bins stay closed.", "Der Bauhofmitarbeiter hat den blauen Papierbehälter entriegelt und an seinen Zapfen um 105 Grad nach vorne gekippt; er hält den Deckel mit der anderen Hand bei 95 Grad offen und das Papier fällt in den Sammelsack, die beiden anderen Behälter bleiben geschlossen."), "bones": {"bin-c": {"angleDeg": -105}, "lid-c": {"angleDeg": 95}}, "ground": "auto", "contacts": SUPPORTS, "thermal": binding("maintenance"), "loads": contents(["bin-l", "bin-r"]) + [{"id": "worker-hand", "kind": "imposed", "title": T("Worker holding the tipped bin at its rim (40 N)", "Mitarbeiter hält den gekippten Behälter am Rand (40 N)"), "bone": "bin-c", "x": 0.18, "y": 0.06, "fxN": 0.0, "fyN": 40.0, "source": "authored-estimate"}, {"id": "worker-lid-hand", "kind": "imposed", "title": T("Worker holding the lid open (6 N)", "Mitarbeiter hält den Deckel offen (6 N)"), "bone": "lid-c", "x": 0.39, "y": -0.02, "fxN": 0.0, "fyN": 6.0, "source": "authored-estimate"}]},
    {"id": "in-use", "title": T("Visitor using the packaging bin", "Besucher nutzt den Verpackungsbehälter"), "description": T("A park visitor holds the faded yellow lid open at 70 degrees to drop a drinks carton; all three bins carry their typical contents.", "Ein Parkbesucher hält den ausgeblichenen gelben Deckel bei 70 Grad offen, um einen Getränkekarton einzuwerfen; alle drei Behälter tragen ihren typischen Inhalt."), "bones": {"lid-l": {"angleDeg": 70}}, "ground": "auto", "contacts": SUPPORTS, "thermal": binding("in-use"), "loads": contents(["bin-l", "bin-c", "bin-r"]) + [{"id": "visitor-hand", "kind": "occupant", "title": T("Visitor's hand holding the lid open (8 N)", "Hand des Besuchers hält den Deckel offen (8 N)"), "bone": "lid-l", "x": 0.39, "y": -0.02, "fxN": 0.0, "fyN": 8.0, "source": "authored-estimate"}]},
]

FULL = [{"anchor": a, "fromS": 0, "toS": 0} for a in SUPPORTS]


def spans(dur):
    """🦶 Pad contacts over the whole clip."""
    return [{"anchor": a, "fromS": 0, "toS": dur} for a in SUPPORTS]


CLIPS = [
    {"id": "emptying", "title": T("Emptying the paper bin", "Leeren des Papierbehälters"), "description": T("The depot worker opens the blue lid, tips the bin forward on its trunnions until the paper falls out, holds it, and lets it swing back and close.", "Der Bauhofmitarbeiter öffnet den blauen Deckel, kippt den Behälter an seinen Zapfen nach vorne, bis das Papier herausfällt, hält ihn und lässt ihn zurückschwingen und schließen."), "durationS": 9, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.5, "pose": "in-service", "ease": "smooth", "bones": {"lid-c": {"angleDeg": 95}}}, {"t": 4, "pose": "maintenance", "ease": "smooth"}, {"t": 6, "pose": "maintenance", "ease": "smooth"}, {"t": 8, "pose": "in-service", "ease": "smooth", "bones": {"lid-c": {"angleDeg": 95}}}, {"t": 9, "pose": "in-service", "ease": "smooth"}],
     "contacts": spans(9), "phases": [{"id": "open-lid", "fromS": 0, "toS": 1.5, "title": T("Opening the lid", "Deckel öffnen")}, {"id": "tip", "fromS": 1.5, "toS": 4, "title": T("Tipping forward", "Nach vorne kippen")}, {"id": "empty", "fromS": 4, "toS": 6, "title": T("Paper falls out", "Papier fällt heraus")}, {"id": "return", "fromS": 6, "toS": 9, "title": T("Swinging back and closing", "Zurückschwingen und schließen")}],
     "events": [{"t": 4, "kind": "motion", "title": T("Bin fully tipped", "Behälter ganz gekippt")}, {"t": 6, "kind": "load-off", "title": T("Worker lets go", "Mitarbeiter lässt los"), "target": "worker-hand"}, {"t": 9, "kind": "motion", "title": T("Lid closed", "Deckel geschlossen")}], "tracks": []},
    {"id": "dropping-litter", "title": T("Dropping a carton into the packaging bin", "Einwerfen eines Kartons in den Verpackungsbehälter"), "description": T("A visitor lifts the yellow lid to 70 degrees, drops a carton and lets the lid fall shut on the rim.", "Ein Besucher hebt den gelben Deckel auf 70 Grad, wirft einen Karton ein und lässt den Deckel auf den Rand fallen."), "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.2, "pose": "in-use", "ease": "smooth"}, {"t": 2.6, "pose": "in-use", "ease": "in"}, {"t": 3.1, "pose": "in-service", "ease": "smooth"}, {"t": 4, "pose": "in-service", "ease": "smooth"}],
     "contacts": spans(4), "phases": [{"id": "lift", "fromS": 0, "toS": 1.2, "title": T("Lifting the lid", "Deckel anheben")}, {"id": "drop", "fromS": 1.2, "toS": 2.6, "title": T("Dropping the carton", "Karton einwerfen")}, {"id": "close", "fromS": 2.6, "toS": 4, "title": T("Lid falls shut", "Deckel fällt zu")}],
     "events": [{"t": 1.2, "kind": "load-on", "title": T("Hand holds the lid", "Hand hält den Deckel"), "target": "visitor-hand"}, {"t": 2.6, "kind": "load-off", "title": T("Hand lets go", "Hand lässt los"), "target": "visitor-hand"}, {"t": 3.1, "kind": "contact-on", "title": T("Lid hits the rim", "Deckel schlägt auf den Rand")}], "tracks": []},
]


def thermal_model():
    """🌡️ Passive regions in summer park sun, every one with its weathered material."""
    regions = []
    for rid, (en, de, kind, mat, geom, svf, thick, lam, lkind, char) in REGIONS.items():
        m = next(x for x in MAT if x["id"] == mat)
        hc = round(m["densityKgM3"] * m["specificHeatJKgK"] * thick)
        regions.append({"id": rid, "title": T(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": AREA[rid], "characteristicLengthM": char, "geometry": geom, "skyViewFactor": svf,
                        "layers": [{"id": rid + "-layer", "title": T(f"{m['title']['en']}, {thick * 1000:g} mm", f"{m['title']['de']}, {str(thick * 1000).rstrip('0').rstrip('.').replace('.', ',')} mm"), "kind": lkind, "material": mat, "thicknessM": thick, "resistanceM2KW": r6(thick / lam), "heatCapacityJM2K": hc, "source": m["source"], "reference": m["reference"]}],
                        "heatCapacityJM2K": hc, "source": "literature", "reference": m["reference"]})
    return {"environment": {"airTempC": 27, "meanRadiantTempC": 32, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 200, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325},
            "operating": [], "emitters": [], "regions": regions, "passengers": [], "minimumAirPassageM": 0.004,
            "assumptions": [
                T("Passive surfaces on a sunny summer afternoon in the park: absorbed sun, long-wave exchange with sky and gravel, convection and heat capacity; nothing is heated from inside and the station has no electrical element, so its rated power is 0 W.", "Passive Oberflächen an einem sonnigen Sommernachmittag im Park: absorbierte Sonne, langwelliger Austausch mit Himmel und Kies, Konvektion und Wärmekapazität; nichts wird von innen beheizt und die Station hat kein elektrisches Element, ihre Nennleistung ist 0 W."),
                T("Weathered surfaces differ from new ones: UV-faded yellow HDPE absorbs about 0.42 of the sun, faded blue 0.74, faded slate grey 0.72, while the replaced anthracite lid still absorbs 0.86 and is the hottest plastic part; chalked anthracite powder coat 0.86, rusted feet 0.82, aged concrete 0.68 (Berdahl et al. 2008, Levinson and Akbari 2002).", "Verwitterte Oberflächen unterscheiden sich von neuen: UV-ausgeblichenes gelbes HDPE absorbiert etwa 0,42 der Sonne, ausgeblichenes Blau 0,74, ausgeblichenes Schiefergrau 0,72, der ersetzte anthrazitfarbene Deckel noch 0,86 und ist das heißeste Kunststoffteil; abgekreidete anthrazitfarbene Pulverbeschichtung 0,86, rostige Füße 0,82, gealterter Beton 0,68 (Berdahl et al. 2008, Levinson und Akbari 2002)."),
                T("Sunlit shares are scenario data: lids lie horizontal and see the sun almost completely, the vertical tubs and posts about half, the pad is partly shaded by the bins; the waste inside the tubs is not modelled thermally.", "Die besonnten Anteile sind Szenariodaten: die Deckel liegen waagrecht und sehen die Sonne fast ganz, die senkrechten Behälter und Pfosten etwa zur Hälfte, die Platte wird von den Behältern teils beschattet; der Abfall in den Behältern wird thermisch nicht modelliert."),
                T("The narrowest real air passage is the 4 mm running gap between the lid skirt and the tub wall.", "Der engste reale Luftdurchgang ist der 4-mm-Laufspalt zwischen Deckelschürze und Behälterwand."),
            ]}


STRUCTURE = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": SUPPORTS, "clinical": False, "assumptions": [
    T("The precast pad bears on the compacted water-bound gravel, idealised as four unilateral contact pads under its corners; each post is dowelled to the pad through its 140 x 140 x 8 mm base plate with four M12 anchors and acts as a cantilever SHS 60 x 60 x 3 (EN 10219-2, S235).", "Die Fertigteilplatte liegt auf der verdichteten wassergebundenen Decke, idealisiert als vier einseitige Kontaktflächen unter ihren Ecken; jeder Pfosten ist über seine Fußplatte 140 x 140 x 8 mm mit vier M12-Dübeln auf der Platte verankert und wirkt als Kragstütze aus Quadratrohr 60 x 60 x 3 (EN 10219-2, S235)."),
    T("Each tub hangs on two 24 mm trunnion pins in bushes of the neighbouring posts, 80 mm below its rim, so its centre of mass lies below the axis and it hangs upright and tips easily; lids hinge on their rear barrels and rest on the rim.", "Jeder Behälter hängt an zwei 24-mm-Lagerzapfen in Buchsen der benachbarten Pfosten, 80 mm unter seinem Rand, so dass sein Schwerpunkt unter der Achse liegt, er senkrecht hängt und sich leicht kippen lässt; die Deckel drehen an ihren hinteren Scharnierwalzen und liegen auf dem Rand auf."),
    T("Contents are live loads at the tub floor centre (4, 9 and 14 kg, authored estimates of a full 70 litre bin of each fraction); wind on the 1.5 m wide station (DIN EN 1991-1-4) is not a governing case for the pad on gravel and is not applied.", "Inhalte sind Nutzlasten in der Mitte des Behälterbodens (4, 9 und 14 kg, Schätzwerte eines vollen 70-Liter-Behälters je Fraktion); Wind auf die 1,5 m breite Station (DIN EN 1991-1-4) ist für die Platte auf Kies nicht maßgebend und wird nicht angesetzt."),
]}

PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

REP_DECISIONS = {
    50: dict(
        silhouette=T("Side elevation 11.2 x 22 mm, front 34 x 22 mm, plan 34 x 11.2 mm: a chamfered precast pad, four SHS posts on rusted base plates with dowel nuts, three hollow HDPE tipping bins on trunnion pins with bosses, three lids with skirt, grip and hinge barrel, and the empty bottle beside the left post. Bins and lids are extruded solids on their bones, so a tipped bin and an open lid read right in front and plan.", "Seitenansicht 11,2 x 22 mm, Vorderansicht 34 x 22 mm, Draufsicht 34 x 11,2 mm: eine gefaste Fertigteilplatte, vier Quadratrohrpfosten auf rostigen Fußplatten mit Dübelmuttern, drei hohle HDPE-Kippbehälter an Lagerzapfen mit Naben, drei Deckel mit Schürze, Griff und Scharnierwalze und die leere Flasche neben dem linken Pfosten. Behälter und Deckel sind extrudierte Körper an ihren Knochen, so dass ein gekippter Behälter und ein offener Deckel in Vorder- und Draufsicht richtig erscheinen."),
        retained=T("Pad chamfers, base plates and the front dowel nuts, rusted post feet, caps, trunnion bosses and pins, lid skirts, grips and barrels, bottle shoulder, neck and lip; traces as strokes: rust streaks, scratches, the mower dent, the graffiti tag, sticker residue, faded labels, moss along the rear edge, gum spots.", "Plattenfasen, Fußplatten und die vorderen Dübelmuttern, rostige Pfostenfüße, Kappen, Zapfennaben und Zapfen, Deckelschürzen, Griffe und Walzen, Flaschenschulter, Hals und Mündung; Spuren als Striche: Roststreifen, Kratzer, die Mäherdelle, das Graffiti-Tag, Aufkleberreste, ausgeblichene Etiketten, Moos an der Hinterkante, Kaugummiflecken."),
        merged=T("The four posts overlap in the side elevation and the three bins overlap there as well; nothing else is merged.", "Die vier Pfosten decken sich in der Seitenansicht, ebenso die drei Behälter; sonst ist nichts verschmolzen."),
        omitted=T("Bush welds, the locking latch (10 mm), drain holes in the tub floors and the rear dowel nuts in front and side views (hidden).", "Buchsenschweißnähte, die Verriegelung (10 mm), Ablauflöcher in den Behälterböden und die hinteren Dübelmuttern in Vorder- und Seitenansicht (verdeckt)."),
        thinElements=T("Lid plate 5 mm (0.1 mm) and skirt walls 4 mm are filled shapes carried by the 0.5 mm body pen; scratches, streaks and moss are 0.13 mm texture strokes, labels and the dent 0.25 mm detail strokes.", "Deckelplatte 5 mm (0,1 mm) und Schürzenwände 4 mm sind gefüllte Formen, getragen von der 0,5-mm-Körperlinie; Kratzer, Streifen und Moos sind 0,13-mm-Texturstriche, Etiketten und die Delle 0,25-mm-Detailstriche."),
        hatching=T("No hatch: the concrete reads by its light tone, the rust by a darker tone of the feet.", "Keine Schraffur: der Beton liest sich über seinen hellen Ton, der Rost über einen dunkleren Ton der Füße."),
        styles=T("Tones from black to white: anthracite posts 0.22, rusted feet 0.32, concrete 0.72, faded yellow 0.82 to 0.88, faded blue 0.38 to 0.5, slate grey tubs 0.42, replaced lid 0.24, bottle 0.4.", "Töne von Schwarz bis Weiß: anthrazitfarbene Pfosten 0,22, rostige Füße 0,32, Beton 0,72, ausgeblichenes Gelb 0,82 bis 0,88, ausgeblichenes Blau 0,38 bis 0,5, schiefergraue Behälter 0,42, ersetzter Deckel 0,24, Flasche 0,4."),
        overlays=T("Thermal colours per region, structural members on posts, tubs, lids, pad and bottle.", "Thermische Farben je Bereich, Tragglieder an Pfosten, Behältern, Deckeln, Platte und Flasche.")),
    100: dict(
        silhouette=T("Side elevation 5.6 x 11 mm, front 17 x 11 mm, plan 17 x 5.6 mm: pad with chamfers, posts on 15 mm base plates, hollow tubs with pins, lids as 30 mm blocks with grip and barrel, the bottle with its neck.", "Seitenansicht 5,6 x 11 mm, Vorderansicht 17 x 11 mm, Draufsicht 17 x 5,6 mm: Platte mit Fasen, Pfosten auf 15-mm-Fußplatten, hohle Behälter mit Zapfen, Deckel als 30-mm-Blöcke mit Griff und Walze, die Flasche mit Hals."),
        retained=T("Base plates, rusted feet, caps, pins, grips, barrels, bottle neck; traces: rust streaks, a scratch per bin, the dent, the graffiti tag, sticker residue, labels, moss, three gum spots.", "Fußplatten, rostige Füße, Kappen, Zapfen, Griffe, Walzen, Flaschenhals; Spuren: Roststreifen, ein Kratzer je Behälter, die Delle, das Graffiti-Tag, Aufkleberreste, Etiketten, Moos, drei Kaugummiflecken."),
        merged=T("Lid plate and skirt are one 30 mm block that ends the tub at 0.935 m; the 8 mm base plate is drawn 15 mm thick.", "Deckelplatte und Schürze sind ein 30-mm-Block, der den Behälter bei 0,935 m abschließt; die 8-mm-Fußplatte ist 15 mm dick gezeichnet."),
        omitted=T("Dowel nuts, trunnion bosses, the second scratch per bin, lid sticker residue.", "Dübelmuttern, Zapfennaben, der zweite Kratzer je Behälter, Aufkleberreste am Deckel."),
        thinElements=T("Lid blocks are 0.3 mm; traces are 0.1 mm texture and 0.18 mm detail strokes.", "Deckelblöcke sind 0,3 mm; Spuren sind 0,1-mm-Textur- und 0,18-mm-Detailstriche."),
        hatching=T("No hatch.", "Keine Schraffur."),
        styles=T("Tones as at 1:50.", "Töne wie bei 1:50."),
        overlays=T("As at 1:50.", "Wie bei 1:50.")),
    200: dict(
        silhouette=T("Side elevation 2.8 x 5.5 mm, front 8.5 x 5.5 mm: pad, solid posts with their rust zone, tub blocks widened to the posts, lid blocks, a two-step bottle.", "Seitenansicht 2,8 x 5,5 mm, Vorderansicht 8,5 x 5,5 mm: Platte, volle Pfosten mit Rostzone, an die Pfosten verbreiterte Behälterblöcke, Deckelblöcke, eine zweistufige Flasche."),
        retained=T("Post rust zones, lids as separate blocks, bottle shoulder; traces: rust streaks, one scratch per bin, labels, the graffiti tag, the dent, moss, gum spots.", "Rostzonen der Pfosten, Deckel als eigene Blöcke, Flaschenschulter; Spuren: Roststreifen, ein Kratzer je Behälter, Etiketten, das Graffiti-Tag, die Delle, Moos, Kaugummiflecken."),
        merged=T("Tubs are solid blocks 0.424 m wide that close the 12 mm gaps to the posts, where the pins were; base plates go into the rust zone; caps into the posts; pad chamfers are dropped.", "Behälter sind volle Blöcke von 0,424 m Breite, die die 12-mm-Spalte zu den Pfosten an der Stelle der Zapfen schließen; Fußplatten gehen in der Rostzone auf, Kappen in den Pfosten; die Plattenfasen entfallen."),
        omitted=T("Grips, barrels, pins, nuts, sticker residue.", "Griffe, Walzen, Zapfen, Muttern, Aufkleberreste."),
        thinElements=T("Detail strokes 0.13 mm; no texture pen at this scale, the remaining traces are detail strokes.", "Detailstriche 0,13 mm; kein Texturstift bei diesem Maßstab, die übrigen Spuren sind Detailstriche."),
        hatching=T("No hatch.", "Keine Schraffur."),
        styles=T("Tones as at 1:50.", "Töne wie bei 1:50."),
        overlays=T("As at 1:50.", "Wie bei 1:50.")),
    500: dict(
        silhouette=T("Side elevation 1.1 x 2.2 mm, front 3.4 x 2.2 mm: pad, 80 mm posts, three bin blocks with their 75 mm lid bands, the bottle as a 77 mm cylinder.", "Seitenansicht 1,1 x 2,2 mm, Vorderansicht 3,4 x 2,2 mm: Platte, 80-mm-Pfosten, drei Behälterblöcke mit ihren 75-mm-Deckelbändern, die Flasche als 77-mm-Zylinder."),
        retained=T("Three colour bins with their lids, four posts with their rust zone, pad, bottle.", "Drei Farbbehälter mit Deckeln, vier Pfosten mit Rostzone, Platte, Flasche."),
        merged=T("Posts are drawn 80 mm so that they print; bins widen to touch them; the lid is the top 75 mm band of each bin so it still opens.", "Pfosten sind 80 mm gezeichnet, damit sie drucken; die Behälter verbreitern sich bis an sie; der Deckel ist das oberste 75-mm-Band jedes Behälters, so dass er sich noch öffnet."),
        omitted=T("All traces of use, which are below 0.15 mm here.", "Alle Gebrauchsspuren, die hier unter 0,15 mm liegen."),
        thinElements=T("None.", "Keine."),
        hatching=T("No hatch.", "Keine Schraffur."),
        styles=T("Tones as at 1:50.", "Töne wie bei 1:50."),
        overlays=T("As at 1:50.", "Wie bei 1:50.")),
}

VOL_DECISIONS = {
    50: T("1:50 after a municipal three-fraction tipping-bin station: chamfered precast pad 1.70 x 0.56 x 0.08 m; per post a 140 x 140 x 8 mm base plate, four M12 dowel nuts, a hollow SHS 60 x 60 x 3 with 6 mm outer corner radius split into the rusted foot (to 0.18 m) and the coated shaft, and an 8 mm cap plate; per bin a hollow 4 mm HDPE tub 0.40 x 0.36 x 0.50 m with 30 mm corner radius, a 4 mm floor sheet, two 8 mm trunnion bosses and two 24 mm steel pins reaching 3 mm into the post walls; per lid a 5 mm plate sheet, a U skirt 25 mm deep with 4 mm walls open at the hinge, a 15 x 20 mm grip and the 20 mm hinge barrel; the bottle as one revolve of a 0.75 litre Bordeaux bottle with its sealed inside not modelled.", "1:50 nach einer kommunalen Dreifach-Kippbehälterstation: gefaste Fertigteilplatte 1,70 x 0,56 x 0,08 m; je Pfosten eine Fußplatte 140 x 140 x 8 mm, vier M12-Dübelmuttern, ein hohles Quadratrohr 60 x 60 x 3 mit 6 mm Außenradius, geteilt in den rostigen Fuß (bis 0,18 m) und den beschichteten Schaft, und eine 8-mm-Kappenplatte; je Behälter eine hohle 4-mm-HDPE-Wanne 0,40 x 0,36 x 0,50 m mit 30 mm Eckradius, ein 4-mm-Bodenblech, zwei 8-mm-Zapfennaben und zwei 24-mm-Stahlzapfen, die 3 mm in die Pfostenwände reichen; je Deckel eine 5-mm-Platte, eine 25 mm tiefe U-Schürze mit 4-mm-Wänden, zum Scharnier offen, ein Griff 15 x 20 mm und die 20-mm-Scharnierwalze; die Flasche als ein Drehkörper einer 0,75-Liter-Bordeauxflasche, ihr verschlossenes Inneres nicht modelliert."),
    100: T("1:100 is its own model: base plates 15 mm, nuts and bosses left out (below 15 mm), posts still hollow with a 15 mm cap, tubs hollow up to 0.935 m under a 30 mm lid block that merges plate and skirt, pins lengthened to 17 mm, grip and barrel kept, the bottle neck straightened.", "1:100 ist ein eigenes Modell: Fußplatten 15 mm, Muttern und Naben entfallen (unter 15 mm), Pfosten noch hohl mit 15-mm-Kappe, Behälter hohl bis 0,935 m unter einem 30-mm-Deckelblock aus Platte und Schürze, Zapfen auf 17 mm verlängert, Griff und Walze bleiben, der Flaschenhals begradigt."),
    200: T("1:200: posts are solid 60 mm bars in two parts (rust zone to 0.18 m, coated shaft with the cap), tubs are solid blocks widened to 0.424 m so that they bear on the posts where the pins are, lids 30 mm blocks, chamfers, pins, grips and barrels left out; the bottle keeps a 33 mm neck.", "1:200: Pfosten sind volle 60-mm-Stäbe in zwei Teilen (Rostzone bis 0,18 m, beschichteter Schaft mit Kappe), Behälter sind volle, auf 0,424 m verbreiterte Blöcke, die dort an den Pfosten anliegen, wo die Zapfen sind, Deckel 30-mm-Blöcke, Fasen, Zapfen, Griffe und Walzen entfallen; die Flasche behält einen 33-mm-Hals."),
    500: T("1:500: posts thickened to 80 mm so that they reach the 75 mm feature, bins 0.404 m between them, each lid the top 75 mm band of its bin on its own bone, the bottle a 77 mm cylinder; the model is minimal: every remaining solid stands for a region or a moving bone.", "1:500: Pfosten auf 80 mm verstärkt, damit sie das 75-mm-Merkmal erreichen, Behälter 0,404 m zwischen ihnen, jeder Deckel das oberste 75-mm-Band seines Behälters an seinem eigenen Knochen, die Flasche ein 77-mm-Zylinder; das Modell ist minimal: jeder verbleibende Körper steht für einen Bereich oder einen beweglichen Knochen."),
}

REFERENCES = [
    {"id": "en-10219-2-2019", "citation": "DIN EN 10219-2:2019-07 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (SHS 60 x 60 x 3: A 6.61 cm2, I 34.9 cm4, 5.19 kg/m)."},
    {"id": "en-1991-1-1-2002", "citation": "DIN EN 1991-1-1:2010-12 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (concrete 24 kN/m3, steel 77 kN/m3)."},
    {"id": "en-1991-1-4-2005", "citation": "DIN EN 1991-1-4:2010-12 Eurocode 1: Actions on structures - Part 1-4: General actions - Wind actions."},
    {"id": "en-1992-1-1-2004", "citation": "DIN EN 1992-1-1:2011-01 Eurocode 2: Design of concrete structures - Part 1-1 (C30/37: fck 30 MPa, Ecm 33 GPa)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity and heat capacity of concrete, steel, plastics, glass)."},
    {"id": "ashby-2011", "citation": "Ashby, M. F. (2011). Materials Selection in Mechanical Design, 4th ed., Butterworth-Heinemann (HDPE density 950 kg/m3, modulus about 1 GPa; container glass mass of a 0.75 l bottle about 0.45 kg)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of oxidised and painted steel, glass)."},
    {"id": "berdahl-2008", "citation": "Berdahl, P., Akbari, H., Levinson, R., Miller, W. A. (2008). Weathering of roofing materials - An overview. Construction and Building Materials 22(4), 423-433 (solar reflectance of coatings and polymers changes with UV exposure and soiling)."},
    {"id": "levinson-akbari-2002", "citation": "Levinson, R., Akbari, H. (2002). Effects of composition and exposure on the solar reflectance of portland cement concrete. Cement and Concrete Research 32(11), 1679-1698."},
    {"id": "en-572-1-2012", "citation": "DIN EN 572-1:2016-06 Glass in building - Basic soda-lime silicate glass products - Part 1: Definitions and general physical and mechanical properties (density 2500 kg/m3, E 70 GPa, conductivity 1 W/mK)."},
    {"id": "din-18040-1-2010", "citation": "DIN 18040-1:2010-10 Barrier-free building - Design principles - Part 1: Publicly accessible buildings (operating elements at 0.85 m, reach range up to about 1.05 m; insertion height of the lids 0.96 m)."},
    {"id": "verpackg-2017", "citation": "Gesetz über das Inverkehrbringen, die Rücknahme und die hochwertige Verwertung von Verpackungen (Verpackungsgesetz - VerpackG) of 5 July 2017 (separate collection of lightweight packaging, the yellow fraction; glass goes to bottle banks, never into a litter bin)."},
]

for b in BONES:
    if b["id"].startswith(("bin-", "lid-")):
        pass

SCALES = {}
for d in (50, 100, 200, 500):
    S = FINE if d == 50 else build_scale(d)
    if d < 500:
        traces(S)
    structure_extras(S)
    SCALES[d] = S

representations = {}
volumes = {}
for d in (500, 200, 100, 50):
    S = SCALES[d]
    representations[str(d)] = {"status": "authored", "denominator": d, "validRange": {"minDenominator": RANGES[d][0], "maxDenominator": RANGES[d][1]}, "pens": PENS[d], "hatches": [], "geometry": S.views, "decisions": REP_DECISIONS[d]}
    volumes[str(d)] = {"status": "authored", "denominator": d, "body": S.body, "thermal": S.thermal, "structure": S.structure, "decisions": [VOL_DECISIONS[d]]}

PAD_KG_DE = f"{MASSES['pad'][0]:.1f}".replace(".", ",")
asset = {
    "id": "used-recycling-bin-cluster",
    "category": "objects",
    "title": T("Used Park Recycling Bin Cluster", "Gebrauchte Wertstoffsammelstation"),
    "summary": T("A three-fraction tipping-bin station of a German municipal park after years of service: four anthracite SHS posts dowelled to a precast pad carry a faded yellow packaging bin, a blue paper bin with a graffiti tag and a grey residual-waste bin with a replaced lid, scratched HDPE, rusted post feet, moss and gum on the pad, and an empty wine bottle left beside it because glass does not go in.",
             "Eine Dreifach-Kippbehälterstation eines kommunalen Parks nach Jahren im Einsatz: vier anthrazitfarbene Quadratrohrpfosten, auf eine Fertigteilplatte gedübelt, tragen einen ausgeblichenen gelben Verpackungsbehälter, einen blauen Papierbehälter mit Graffiti-Tag und einen grauen Restmüllbehälter mit ersetztem Deckel, verkratztes HDPE, rostige Pfostenfüße, Moos und Kaugummi auf der Platte und eine leere Weinflasche daneben, weil Glas nicht hineingehört."),
    "tags": ["used", "park", "public-space", "waste", "recycling", "litter-bin", "tipping-bin", "hdpe", "steel", "concrete", "street-furniture", "maintenance"],
    "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T(f"Catalogue-typical municipal tipping-bin station (authored estimate, no brand): 70 litre HDPE tubs 0.40 x 0.36 x 0.50 m hung at 0.88 m on trunnions, rim and insertion height 0.96 m (DIN 18040-1 reach range), posts SHS 60 x 60 x 3 at 0.484 m centres (EN 10219-2), base plates 140 x 140 x 8 mm, precast pad 1.70 x 0.56 x 0.08 m. Masses from section times density of the 1:50 solids: total {TOTAL_KG} kg, of which the pad is {MASSES['pad'][0]:.1f} kg. Geometry and dimensions are literature and authored estimates; nothing is measured.",
                             f"Katalogtypische kommunale Kippbehälterstation (Schätzung, keine Marke): 70-Liter-HDPE-Wannen 0,40 x 0,36 x 0,50 m, bei 0,88 m an Zapfen gehängt, Rand- und Einwurfhöhe 0,96 m (Greifbereich DIN 18040-1), Pfosten Quadratrohr 60 x 60 x 3 im Abstand 0,484 m (EN 10219-2), Fußplatten 140 x 140 x 8 mm, Fertigteilplatte 1,70 x 0,56 x 0,08 m. Massen aus Querschnitt mal Dichte der 1:50-Körper: gesamt {str(TOTAL_KG).replace('.', ',')} kg, davon die Platte {PAD_KG_DE} kg. Geometrie und Abmessungen sind Literatur- und Schätzwerte; nichts ist gemessen.")},
    "dimensions": {"heightM": POST_TOP, "widthM": 2 * PAD_X, "depthM": 2 * PAD_Z, "massKg": TOTAL_KG},
    "materials": MAT,
    "rig": {"kind": "mechanical", "root": "pad", "bones": BONES},
    "anchors": ANCHORS,
    "representations": representations,
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": thermal_model(),
    "structure": STRUCTURE,
    "volumes": volumes,
    "presentation": {"defaultPose": "in-service", "defaultClip": "emptying", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}
doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
with open(OUT, "w") as fh:
    json.dump(doc, fh, ensure_ascii=False, indent=1)
print("[DEBUG] wrote", OUT, "mass", TOTAL_KG, {k: round(v[0], 3) for k, v in MASSES.items()}, {d: (len(s.body), len(s.thermal), len(s.structure)) for d, s in SCALES.items()})
