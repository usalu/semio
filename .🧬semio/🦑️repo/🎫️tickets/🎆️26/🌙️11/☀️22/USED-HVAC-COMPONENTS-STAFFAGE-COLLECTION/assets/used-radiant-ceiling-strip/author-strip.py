"""☀ Authoritative generator of the used radiant ceiling heating strip: every hand-chosen number of the 0.9 x 6.0 m four-tube industrial hall panel on its three bearers, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file; texts, materials, thermal model and decisions come from author-strip-texts.py."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-radiant-ceiling-strip"


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


ORIGIN = {"floor": (0.0, 0.0), "bearers": (0.0, 0.0), "panel": (0.0, 0.10)}

L = 6.0
HALF = 0.45
SHEET = 0.00125
Y0 = 0.10
UP = 0.05
INS = 0.04
INS_X = (0.10, 5.90)
BEARER_XS = (0.60, 3.00, 5.40)
BEARER_S, BEARER_HALF = 0.10, 0.55
TUBE_Z = (-0.3375, -0.1125, 0.1125, 0.3375)
TUBE_R, TUBE_RI = 0.014, 0.0125
HDR_X = (0.0788, 5.9212)
HDR_R, HDR_RI, HDR_Z = 0.0212, 0.0186, 0.40
TUBE_Y = Y0 + SHEET + TUBE_R
HDR_Y = Y0 + SHEET + HDR_R
STUBS = [("flow", HDR_X[0], 0.30), ("return", HDR_X[1], -0.30)]
STUB_R, STUB_RI, STUB_Y, CAP_R, CAP_Y = 0.01685, 0.01425, (0.130, 0.200), 0.022, 0.215
TRAV_XS = (0.75, 2.25, 3.75, 5.25)
TRAV_W, TRAV_Y, TRAV_Z = 0.04, (0.15, 0.17), 0.47
EYE_Z, EYE_RO, EYE_RI, EYE_T = 0.46, 0.0225, 0.0125, 0.010
EYE_Y = TRAV_Y[1] + EYE_RO
LINK_R, LINK_W = 0.010, 0.003
LINK_Y = EYE_Y + EYE_RI - LINK_W + LINK_R
TOP = LINK_Y + LINK_R + LINK_W

STEEL = 7850.0
A_TUBE = math.pi * (TUBE_R ** 2 - TUBE_RI ** 2)
A_HDR = math.pi * (HDR_R ** 2 - HDR_RI ** 2)
A_RHS = 0.04 * 0.02 - 0.036 * 0.016
M_BEARER = len(BEARER_XS) * BEARER_S * BEARER_S * 2 * BEARER_HALF * 450.0
MASSES = {
    "plate": (L * 2 * HALF * SHEET * STEEL, (3.0, 0.1006)),
    "upstands": (2 * L * UP * SHEET * STEEL, (3.0, 0.125)),
    "end-plates": (2 * (2 * HALF - 2 * SHEET) * (UP - SHEET) * SHEET * STEEL, (3.0, 0.125)),
    "tubes": (4 * (HDR_X[1] - HDR_X[0]) * A_TUBE * STEEL, (3.0, TUBE_Y)),
    "headers": (2 * (2 * HDR_Z * A_HDR + 2 * math.pi * HDR_R ** 2 * 0.0026) * STEEL, (3.0, HDR_Y)),
    "stubs": (2 * ((STUB_Y[1] - STUB_Y[0]) * math.pi * (STUB_R ** 2 - STUB_RI ** 2) * STEEL + 0.08), (3.0, 0.18)),
    "insulation": ((INS_X[1] - INS_X[0]) * 2 * (HALF - SHEET) * INS * 30.0, (3.0, Y0 + SHEET + INS / 2)),
    "traverses": (len(TRAV_XS) * 2 * TRAV_Z * A_RHS * STEEL, (3.0, 0.16)),
    "eye-nuts": (8 * 0.10, (3.0, EYE_Y)),
    "chain-links": (8 * 0.012, (3.0, LINK_Y)),
}
M_PANEL = sum(m for m, _ in MASSES.values())
COM_PANEL = (sum(m * c[0] for m, c in MASSES.values()) / M_PANEL, sum(m * c[1] for m, c in MASSES.values()) / M_PANEL)
M_TOTAL = M_BEARER + M_PANEL
WATER_L = 4 * (HDR_X[1] - HDR_X[0]) * math.pi * TUBE_RI ** 2 * 1000 + 2 * 2 * HDR_Z * math.pi * HDR_RI ** 2 * 1000

SEC_PARTS = [(2 * HALF * SHEET, SHEET / 2, 2 * HALF * SHEET ** 3 / 12), (2 * UP * SHEET, UP / 2, 2 * SHEET * UP ** 3 / 12)] + [(A_TUBE, SHEET + TUBE_R, math.pi / 4 * (TUBE_R ** 4 - TUBE_RI ** 4))] * 4
SEC_A = sum(a for a, _, _ in SEC_PARTS)
SEC_YC = sum(a * y for a, y, _ in SEC_PARTS) / SEC_A
SEC_I = sum(i + a * (y - SEC_YC) ** 2 for a, y, i in SEC_PARTS)

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


class Part:
    """🧱 One physical part of one scale: its 3D solid in the bone frame and its three 2D projections from the same numbers."""

    def __init__(self, pid, bone, kind, geo, material, tone, region, views=("elevation", "front", "plan"), body=True, role="body", extra=None):
        self.id, self.bone, self.kind, self.geo, self.material, self.tone, self.region = pid, bone, kind, geo, material, tone, region
        self.views, self.body, self.role, self.extra = views, body, role, extra or {}

    def solid(self):
        """🧊 The volume solid of the part in the rest frame of its bone."""
        ox, oy = ORIGIN[self.bone]
        g = self.geo
        base = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            s = dict(base, kind="box", size=[r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]})
            if self.extra.get("radius"):
                s["cornerRadius"] = self.extra["radius"]
        elif self.kind == "cylx":
            x0, x1, cy, cz, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(x1 - x0), r], [r6(x1 - x0), 0]], axis=[1, 0, 0], origin=[r6(x0 - ox), r6(cy - oy), cz])
        elif self.kind == "cyly":
            cx, cz, y0, y1, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(y1 - y0), r], [r6(y1 - y0), 0]], axis=[0, 1, 0], origin=[r6(cx - ox), r6(y0 - oy), cz])
        elif self.kind == "cylz":
            cx, cy, z0, z1, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(z1 - z0), r], [r6(z1 - z0), 0]], axis=[0, 0, 1], origin=[r6(cx - ox), r6(cy - oy), r6(z0)])
        elif self.kind == "ring":
            cx, cy, cz, ro, ri, t = g
            n = 20
            outer = [[r6(cx - ox + ro * math.cos(2 * math.pi * k / n)), r6(cy - oy + ro * math.sin(2 * math.pi * k / n))] for k in range(n)]
            s = dict(base, kind="extrude", plane="xy", profile=outer, **{"from": r6(cz - t / 2), "to": r6(cz + t / 2)})
            if ri:
                s["holes"] = [[[r6(cx - ox + ri * math.cos(2 * math.pi * k / n)), r6(cy - oy + ri * math.sin(2 * math.pi * k / n))] for k in range(n)][::-1]]
        elif self.kind == "torx":
            cx, cy, cz, R, w = g
            n = 12
            prof = [[r6(w * math.cos(2 * math.pi * k / n)), r6(R + w * math.sin(2 * math.pi * k / n))] for k in range(n)]
            s = dict(base, kind="revolve", profile=prof + [prof[0]], axis=[1, 0, 0], origin=[r6(cx - ox), r6(cy - oy), cz])
        for key in ("slender", "sheet", "texture", "repeat", "tags", "sectionFaces"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def bounds(self):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of the part at rest."""
        g = self.geo
        if self.kind == "box":
            return g
        if self.kind == "cylx":
            x0, x1, cy, cz, r = g
            return (x0, cy - r, cz - r), (x1, cy + r, cz + r)
        if self.kind == "cyly":
            cx, cz, y0, y1, r = g
            return (cx - r, y0, cz - r), (cx + r, y1, cz + r)
        if self.kind == "cylz":
            cx, cy, z0, z1, r = g
            return (cx - r, cy - r, z0), (cx + r, cy + r, z1)
        if self.kind == "ring":
            cx, cy, cz, ro, ri, t = g
            return (cx - ro, cy - ro, cz - t / 2), (cx + ro, cy + ro, cz + t / 2)
        if self.kind == "torx":
            cx, cy, cz, R, w = g
            return (cx - w, cy - R - w, cz - R - w), (cx + w, cy + R + w, cz + R + w)

    def shape(self, view):
        """✏ The filled 2D shape of the part in one viewpoint, in that viewpoint's frame."""
        ox, oy = ORIGIN[self.bone]
        (x0, y0, z0), (x1, y1, z1) = self.bounds()
        depth = {"elevation": 5 - z0, "front": 5 + x1, "plan": 5 + y1}[view]
        s = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(depth, 4), "filled": True, "tone": self.tone, "material": self.material}
        if isinstance(self.region, list):
            s["regions"] = self.region
        else:
            s["region"] = self.region
        g = self.geo

        def rect(x, y, w, h):
            return dict(s, kind="rect", x=r6(x), y=r6(y), w=r6(w), h=r6(h))

        def circle(cx, cy, r, ri=None):
            if ri:
                return dict(s, kind="path", d=ring_path(cx, cy, r, ri))
            return dict(s, kind="ellipse", cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r))

        if view == "elevation":
            if self.kind == "ring":
                cx, cy, cz, ro, ri, t = g
                return circle(cx - ox, cy - oy, ro, ri)
            if self.kind == "cylz":
                cx, cy, _, _, r = g
                return circle(cx - ox, cy - oy, r)
            return rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)
        if view == "front":
            if self.kind == "cylx":
                return circle(g[3], g[2], g[4])
            if self.kind == "torx":
                cx, cy, cz, R, w = g
                return circle(cz, cy, R + w, R - w)
            return rect(z0, y0, z1 - z0, y1 - y0)
        if self.kind == "cyly":
            cx, cz, _, _, r = g
            return circle(cx, cz, r)
        return rect(x0, z0, x1 - x0, z1 - z0)


def ring_path(cx, cy, r, ri):
    """⭕ An SVG annulus: outer circle and the bore as a hole."""
    return (f"M {r6(cx - r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx + r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx - r)} {r6(cy)} Z "
            f"M {r6(cx - ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx + ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx - ri)} {r6(cy)} Z")


def parts_of(scale):
    """🗂 The hand-chosen part list of one scale: every part the real strip has that this scale can show, generalised for its smallest feature."""
    P = []
    for k, xc in enumerate(BEARER_XS):
        P.append(Part(f"bearer-{k}", "bearers", "box", ((xc - BEARER_S / 2, 0.0, -BEARER_HALF), (xc + BEARER_S / 2, BEARER_S, BEARER_HALF)), "softwood", 0.62, "bearers",
                      views=("elevation", "front", "plan") if k == 0 else ("elevation", "plan"), extra={"radius": 0.004} if scale == 50 else {}))
    if scale in (50, 100):
        P.append(Part("radiant-plate", "panel", "box", ((0.0, Y0, -HALF), (L, Y0 + SHEET, HALF)), "coated-steel", 0.86, "plate", views=("front", "plan"), extra={"sheet": True}))
        for side, name in ((-1, "right"), (1, "left")):
            zs = sorted((side * HALF, side * (HALF - SHEET)))
            P.append(Part(f"upstand-{name}", "panel", "box", ((0.0, Y0, zs[0]), (L, Y0 + UP, zs[1])), "coated-steel", 0.86, "plate",
                          views=("elevation", "front", "plan") if name == "right" else ("front", "plan"), extra={"sheet": True}))
        for name, xa in (("rear", 0.0), ("front", L - SHEET)):
            P.append(Part(f"end-plate-{name}", "panel", "box", ((xa, Y0 + SHEET, -HALF + SHEET), (xa + SHEET, Y0 + UP, HALF - SHEET)), "coated-steel", 0.8, "plate",
                          views=("front", "plan") if name == "front" else ("plan",), extra={"sheet": True}))
        P.append(Part("insulation", "panel", "box", ((INS_X[0], Y0 + SHEET, -HALF + SHEET), (INS_X[1], Y0 + SHEET + INS, HALF - SHEET)), "mineral-wool", 0.7, "insulation", views=("elevation", "plan")))
        for k, z in enumerate(TUBE_Z):
            P.append(Part(f"tube-{k}", "panel", "cylx", (HDR_X[0], HDR_X[1], TUBE_Y, z, TUBE_R), "steel-tube", 0.3, "tubes", views=(), extra={"slender": True}))
        for k, x in enumerate(HDR_X):
            P.append(Part(f"header-{k}", "panel", "cylz", (x, HDR_Y, -HDR_Z, HDR_Z, HDR_R), "coated-steel", 0.8, "headers", views=("plan",)))
    else:
        top = Y0 + (UP if scale == 200 else 0.075)
        reg = ["plate", "insulation", "headers"] if scale == 200 else ["plate", "insulation", "headers", "traverses"]
        P.append(Part("panel-trough", "panel", "box", ((0.0, Y0, -HALF), (L, top, HALF)), "coated-steel", 0.84, reg))
        if scale == 200:
            for k, x in enumerate(HDR_X):
                P.append(Part(f"header-{k}", "panel", "cylz", (x, HDR_Y, -HDR_Z, HDR_Z, HDR_R), "coated-steel", 0.8, "headers", views=(), body=False))
    if scale == 500:
        for name, x, z in STUBS:
            P.append(Part(f"{name}-stub", "panel", "box", ((x - 0.045, TOP - 0.09, z - 0.045), (x + 0.045, TOP, z + 0.045)), "steel", 0.4, "headers", views=("elevation", "front", "plan") if name == "flow" else ("elevation", "plan")))
    if scale != 500:
        for name, x, z in STUBS:
            if scale == 50:
                P.append(Part(f"{name}-stub", "panel", "cyly", (x, z, STUB_Y[0], STUB_Y[1], STUB_R), "steel", 0.4, "headers", views=("elevation", "front", "plan") if name == "flow" else ("elevation", "plan")))
                P.append(Part(f"{name}-cap", "panel", "cyly", (x, z, STUB_Y[1], CAP_Y, CAP_R), "steel", 0.22, "headers", views=("elevation", "front", "plan") if name == "flow" else ("elevation", "plan")))
            else:
                P.append(Part(f"{name}-stub", "panel", "cyly", (x, z, STUB_Y[0], CAP_Y, CAP_R), "steel", 0.4, "headers", views=("elevation", "front", "plan") if name == "flow" else ("elevation", "plan")))
        ty1 = TRAV_Y[1] if scale != 200 else TRAV_Y[0] + 0.03
        for k, x in enumerate(TRAV_XS):
            P.append(Part(f"traverse-{k}", "panel", "box", ((x - TRAV_W / 2, TRAV_Y[0], -TRAV_Z), (x + TRAV_W / 2, ty1, TRAV_Z)), "galvanised-steel", 0.5, "traverses",
                          views=("elevation", "front", "plan") if k == 0 else ("elevation", "plan")))
    if scale in (50, 100):
        for k, x in enumerate(TRAV_XS):
            for side, name in ((-1, "right"), (1, "left")):
                z = side * EYE_Z
                vis = tuple(v for v, ok in (("elevation", name == "right"), ("front", k == 0), ("plan", True)) if ok)
                if scale == 50:
                    P.append(Part(f"eye-nut-{k}-{name}", "panel", "ring", (x, EYE_Y, z, EYE_RO, EYE_RI, EYE_T), "galvanised-steel", 0.35, "traverses", views=vis))
                    P.append(Part(f"cut-link-{k}-{name}", "panel", "torx", (x, LINK_Y, z, LINK_R, LINK_W), "galvanised-steel", 0.3, "traverses", views=vis))
                else:
                    P.append(Part(f"eye-nut-{k}-{name}", "panel", "ring", (x, EYE_Y, z, EYE_RO, None, 0.016), "galvanised-steel", 0.35, "traverses", views=vis))
    return P


def details_of(scale):
    """🖋 The open and closed detail and texture strokes of one scale: lip fold, type label, dust, blanket joints and the cut links."""
    e, f, p = [], [], []
    if scale >= 200:
        return e, f, p
    oy = Y0

    def line(lst, sid, bone, pts, role="detail", region=None, tone=0.25, z=9.0, closed=False, filled=False):
        s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
        if region:
            s["region"] = region
        lst.append(s)

    line(e, "upstand-lip-fold", "panel", [(0.0, Y0 + UP - 0.008 - oy), (L, Y0 + UP - 0.008 - oy)], region="plate", z=8.5)
    line(p, "insulation-edge-rear", "panel", [(INS_X[0], -HALF + SHEET), (INS_X[0], HALF - SHEET)], region="insulation", z=7.0)
    line(p, "insulation-edge-front", "panel", [(INS_X[1], -HALF + SHEET), (INS_X[1], HALF - SHEET)], region="insulation", z=7.0)
    if scale == 50:
        line(e, "type-label", "panel", [(0.34, 0.012), (0.46, 0.012), (0.46, 0.038), (0.34, 0.038)], region="plate", tone=0.35, closed=True, filled=True, z=8.6)
        line(e, "type-label-text", "panel", [(0.355, 0.025), (0.445, 0.025)], role="texture", region="plate", z=8.7)
        for k, x in enumerate((1.1, 2.7, 4.3, 5.6)):
            line(e, f"dust-streak-{k}", "panel", [(x, 0.004), (x + 0.35, 0.006 + 0.002 * (k % 2))], role="texture", region="plate", tone=0.45, z=8.4)
        for k, xc in enumerate(BEARER_XS):
            line(e, f"bearer-grain-{k}", "bearers", [(xc - 0.03, 0.02), (xc + 0.02, 0.05), (xc - 0.01, 0.085)], role="texture", region="bearers", z=6.5)
        for k, x in enumerate((1.55, 3.0, 4.45)):
            line(p, f"blanket-joint-{k}", "panel", [(x, -HALF + SHEET), (x, HALF - SHEET)], role="texture", region="insulation", z=7.1)
        line(p, "foil-tape", "panel", [(INS_X[0], 0.0), (INS_X[1], 0.0)], role="texture", region="insulation", z=7.1)
        for k, x in enumerate(TRAV_XS):
            line(e, f"link-cut-{k}", "panel", [(x - 0.004, LINK_Y + LINK_R + LINK_W - oy - 0.004), (x + 0.004, LINK_Y + LINK_R + LINK_W - oy - 0.001)], role="texture", region="traverses", z=9.5)
        line(f, "header-hidden", "panel", [(-HDR_Z, HDR_Y), (HDR_Z, HDR_Y)], role="texture", region="plate", z=5.5)
    line(f, "lip-fold-front", "panel", [(-HALF + 0.002, Y0 + UP - 0.008), (-HALF + 0.002, Y0 + UP)], region="plate", z=11.5)
    return e, f, p


def representation(scale):
    """🖨 The authored representation of one scale: the projected part shapes, the detail strokes, pens, hatches and decisions."""
    parts = parts_of(scale)
    geo = {"elevation": [], "front": [], "plan": []}
    for part in parts:
        for view in part.views:
            sh = part.shape(view)
            if scale == 50 and view == "elevation" and part.id.startswith("bearer-"):
                sh["hatch"] = "end-grain"
            geo[view].append(sh)
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
            500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]
    hatches = [{"id": "end-grain", "title": T("Sawn end grain of the softwood bearers", "Gesägtes Hirnholz der Nadelholz-Kanthölzer"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}] if scale == 50 else []
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": DECISIONS_2D[scale]}


def tsolid(sid, kind_box, region, material, tone, layer=None, tags=None, role="body"):
    """🌡 One own thermal box solid in the panel frame from world bounds."""
    (x0, y0, z0), (x1, y1, z1) = kind_box
    s = {"id": sid, "kind": "box", "bone": "panel", "material": material, "role": role, "tone": tone, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)],
         "transform": {"translation": [r6((x0 + x1) / 2), r6((y0 + y1) / 2 - Y0), r6((z0 + z1) / 2)]}, "region": region}
    if layer:
        s["layer"] = layer
    if tags:
        s["tags"] = tags
    return s


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: the sheet shells of the radiant plate, the wool blanket, the hidden water tubes, the headers, the traverses, the bearers and the plume patch."""
    out = []
    names = {p.id for p in parts}
    if "radiant-plate" in names:
        for pid in ("radiant-plate", "upstand-right", "upstand-left", "end-plate-rear", "end-plate-front"):
            out.append({"id": f"th-{pid}", "of": pid, "region": "plate"})
        out.append({"id": "th-insulation", "of": "insulation", "region": "insulation"})
    else:
        out.append(tsolid("th-radiant-plate", ((0.0, Y0, -HALF), (L, Y0 + SHEET, HALF)), "plate", "coated-steel", 0.86, "sheet"))
        out.append(tsolid("th-upstand-right", ((0.0, Y0, -HALF), (L, Y0 + UP, -HALF + SHEET)), "plate", "coated-steel", 0.86, "sheet"))
        out.append(tsolid("th-upstand-left", ((0.0, Y0, HALF - SHEET), (L, Y0 + UP, HALF)), "plate", "coated-steel", 0.86, "sheet"))
        out.append(tsolid("th-end-plate-rear", ((0.0, Y0 + SHEET, -HALF + SHEET), (SHEET, Y0 + UP, HALF - SHEET)), "plate", "coated-steel", 0.8, "sheet"))
        out.append(tsolid("th-end-plate-front", ((L - SHEET, Y0 + SHEET, -HALF + SHEET), (L, Y0 + UP, HALF - SHEET)), "plate", "coated-steel", 0.8, "sheet"))
        out.append(tsolid("th-insulation", ((INS_X[0], Y0 + SHEET, -HALF + SHEET), (INS_X[1], Y0 + SHEET + INS, HALF - SHEET)), "insulation", "mineral-wool", 0.7, "wool" if scale == 500 else None))
    if scale in (50, 100):
        for k, z in enumerate(TUBE_Z):
            out.append({"id": f"th-tube-{k}", "kind": "revolve", "bone": "panel", "material": "steel-tube", "role": "body", "tone": 0.3, "profile": [[0, 0], [0, TUBE_R], [r6(HDR_X[1] - HDR_X[0]), TUBE_R], [r6(HDR_X[1] - HDR_X[0]), 0]],
                        "axis": [1, 0, 0], "origin": [HDR_X[0], r6(TUBE_Y - Y0), z], "region": "tubes", "slender": True})
    else:
        out.append(tsolid("th-tube-layer", ((HDR_X[0], Y0 + SHEET, -0.3515), (HDR_X[1], Y0 + SHEET + 2 * TUBE_R, 0.3515)), "tubes", "steel-tube", 0.3, "wall"))
    for part in parts:
        if part.id.startswith(("header-", "flow-", "return-", "traverse-", "eye-nut-", "cut-link-", "bearer-")) and not (scale == 500 and part.id.startswith(("flow-", "return-"))):
            region = part.region if isinstance(part.region, str) else part.region[0]
            if part.body:
                out.append({"id": f"th-{part.id}", "of": part.id, "region": region})
            else:
                s = part.solid()
                s.update({"id": f"th-{part.id}", "region": region})
                out.append(s)
    if scale == 500:
        for k, x in enumerate(HDR_X):
            s = Part(f"header-{k}", "panel", "cylz", (x, HDR_Y, -HDR_Z, HDR_Z, HDR_R), "coated-steel", 0.8, "headers").solid()
            s.update({"id": f"th-header-{k}", "region": "headers", "layer": "tube"})
            out.append(s)
        for k, x in enumerate(TRAV_XS):
            out.append(tsolid(f"th-traverse-{k}", ((x - TRAV_W / 2, Y0 + UP, -TRAV_Z + 0.02), (x + TRAV_W / 2, Y0 + UP + 0.02, TRAV_Z - 0.02)), "traverses", "galvanised-steel", 0.5, "steel"))
    pt = max(FEATURE[scale], 0.004)
    ytop = Y0 + SHEET + INS
    yc = ytop + pt / 2 if scale != 500 else Y0 + 0.0375
    out.append({"id": "th-plume-patch", "kind": "box", "bone": "panel", "material": "mineral-wool", "role": "detail", "tone": 0.7, "size": [0.40, pt, 0.40],
                "transform": {"translation": [3.0, r6(yc - Y0), 0]}, "region": "insulation", "tags": ["convection-plume"]})
    return out


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: the three bearers, the panel as one composite member, the traverses, the bearings on the bearers as joints, and the centres of mass."""
    out = []
    names = {p.id for p in parts}
    for k in range(len(BEARER_XS)):
        out.append({"id": f"member-bearer-{k}", "of": f"bearer-{k}", "member": "bearers", "part": "member"})
    ptop = UP if scale != 500 else 0.075
    out.append({"id": "member-panel", "kind": "box", "bone": "panel", "material": "coated-steel", "role": "body", "tone": 0.3, "size": [L, ptop, 2 * HALF],
                "transform": {"translation": [L / 2, r6(ptop / 2), 0]}, "member": "panel", "part": "member"})
    if scale in (50, 100):
        for k, z in enumerate(TUBE_Z):
            out.append({"id": f"member-tube-{k}", "kind": "revolve", "bone": "panel", "material": "steel-tube", "role": "detail", "tone": 0.2, "profile": [[0, 0], [0, TUBE_R], [r6(HDR_X[1] - HDR_X[0]), TUBE_R], [r6(HDR_X[1] - HDR_X[0]), 0]],
                        "axis": [1, 0, 0], "origin": [HDR_X[0], r6(TUBE_Y - Y0), z], "slender": True, "member": "panel", "part": "member"})
    for k in range(len(TRAV_XS)):
        if f"traverse-{k}" in names:
            out.append({"id": f"member-traverse-{k}", "of": f"traverse-{k}", "member": "panel", "part": "member"})
    pad = max(FEATURE[scale], 0.008)
    for k, x in enumerate(BEARER_XS):
        out.append({"id": f"joint-bearing-{k}", "kind": "box", "bone": "panel", "material": "steel", "role": "detail", "tone": 0.5, "size": [max(BEARER_S, 2 * pad), pad, 2 * HALF],
                    "transform": {"translation": [x, 0.0, 0]}, "member": "panel", "part": "joint"})
    mr = max(0.02, FEATURE[scale] / 2)
    out.append({"id": "mass-panel", "kind": "ellipsoid", "bone": "panel", "material": "coated-steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [r6(COM_PANEL[0]), r6(max(COM_PANEL[1] - Y0, mr)), 0]}, "member": "panel", "part": "mass"})
    out.append({"id": "mass-bearers", "kind": "ellipsoid", "bone": "bearers", "material": "softwood", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [3.0, 0.05, 0]}, "member": "bearers", "part": "mass"})
    return out


def volume(scale):
    """🧊 The authored volume of one scale: body from the part list, thermal and structure lists, and the decisions."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts if p.body]
    if scale == 50:
        body.append({"id": "type-label", "kind": "box", "bone": "panel", "material": "aluminium", "role": "detail", "tone": 0.35, "size": [0.12, 0.026, 0.0005],
                     "transform": {"translation": [0.40, 0.025, -HALF - 0.00025]}, "texture": True, "sheet": True})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


exec(open(os.path.join(HERE, "author-strip-texts.py"), encoding="utf8").read())


def binding(regions, hanging=False):
    """🌬 The region binding of a pose: everything indoors, nothing sunlit, the plate and the bearers on their bearing faces when stored."""
    out = {}
    for r in regions:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "bearers":
            b = {"exposedFraction": 0.75, "sunlitFraction": 0, "contact": {"fraction": 0.25, "tempC": 15}}
        if r == "plate" and not hanging:
            b = {"exposedFraction": 0.96, "sunlitFraction": 0, "contact": {"fraction": 0.04, "tempC": 15}}
        out[r] = b
    return out


def document():
    """📄 The whole asset document."""
    regions = [r["id"] for r in THERMAL["regions"]]
    rig = {"kind": "mechanical", "root": "floor", "bones": [
        {"id": "floor", "parent": None, "title": T("Yard floor frame (massless, fixed root)", "Hallenboden-Bezug (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "fixed"}},
        {"id": "bearers", "parent": "floor", "title": T("Three squared softwood bearers 100 x 100 x 1100 mm (stay on the floor when the strip is lifted)", "Drei Nadelholz-Kanthölzer 100 x 100 x 1100 mm (bleiben beim Anheben am Boden liegen)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 5.45, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 7.0},
         "mass": {"kg": r6(M_BEARER), "comX": 3.0, "comY": 0.05, "distribution": "uniform", "source": "literature", "reference": "en-338"},
         "section": {"areaM2": 0.01, "secondMomentM4": r6(BEARER_S ** 4 / 12), "extremeFibreM": 0.05, "material": "softwood", "title": T("Squared timber 100 x 100 mm", "Kantholz 100 x 100 mm")}},
        {"id": "panel", "parent": "floor", "title": T("Radiant ceiling strip: coated steel plate with upstands, four tubes, two headers, wool blanket and four suspension traverses (lifted by its eye nuts)", "Deckenstrahlplatte: beschichtetes Stahlblech mit Aufkantungen, vier Rohren, zwei Sammlern, Wollmatte und vier Aufhängetraversen (an ihren Ringmuttern gehoben)"),
         "x": 0, "y": Y0, "angleDeg": 0, "lengthM": L, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.10, "maxM": 6.0},
         "mass": {"kg": r6(M_PANEL), "comX": r6(COM_PANEL[0]), "comY": r6(COM_PANEL[1] - Y0), "distribution": "uniform", "source": "authored-estimate", "reference": "radiant-panel-catalogues"},
         "section": {"areaM2": r6(SEC_A), "secondMomentM4": float(f"{SEC_I:.4e}"), "extremeFibreM": r6(UP - SEC_YC), "material": "coated-steel", "title": T("Plate 900 x 1.25 mm, two 50 mm upstands and four tubes 28 x 1.5 mm", "Blech 900 x 1,25 mm, zwei Aufkantungen 50 mm und vier Rohre 28 x 1,5 mm")}},
    ]}
    anchors = []
    k = 0
    for xc in BEARER_XS:
        for z in (-0.50, 0.50):
            anchors.append({"id": f"bearer-ground-{k}", "bone": "bearers", "x": xc, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.10, "footprintWidthM": 0.10, "frictionCoefficient": 0.5,
                            "title": T(f"Bearer at {xc:.2f} m, {'right' if z < 0 else 'left'} end on the yard floor", f"Kantholz bei {xc:.2f} m, {'rechtes' if z < 0 else 'linkes'} Ende auf dem Hallenboden")})
            k += 1
    eyes = []
    for t, x in enumerate(TRAV_XS):
        for side, name in ((-1, "right"), (1, "left")):
            aid = f"eye-{t}-{name}"
            eyes.append(aid)
            anchors.append({"id": aid, "bone": "panel", "x": x, "y": r6(TRAV_Y[1] - Y0), "z": side * EYE_Z, "kind": "support", "support": "pin",
                            "title": T(f"M10 eye nut at the {name} end of traverse {t + 1}: suspension chain or sling hook", f"Ringmutter M10 am {'rechten' if name == 'right' else 'linken'} Ende der Traverse {t + 1}: Aufhängekette oder Anschlaghaken")})
    anchors.append({"id": "insulation-top", "bone": "panel", "x": 3.0, "y": r6(SHEET + INS), "z": 0, "kind": "emitter", "title": T("Middle of the blanket top", "Mitte der Mattenoberseite")})
    for name, x, z in STUBS:
        anchors.append({"id": f"{name}-connection", "bone": "panel", "x": x, "y": r6(CAP_Y - Y0), "z": z, "kind": "load", "title": T(f"Capped {name} stub, where the {name} hose is connected", f"Verschlossener {'Vorlauf' if name == 'flow' else 'Rücklauf'}stutzen, an dem der {'Vorlauf' if name == 'flow' else 'Rücklauf'}schlauch angeschlossen wird")})
    ground = [f"bearer-ground-{k}" for k in range(6)]
    sling = [f"eye-{t}-{n}" for t in (0, 3) for n in ("right", "left")]
    service_loads = [
        {"id": "panel-water", "kind": "carried", "title": T(f"Heating water {WATER_L:.1f} l in the four tubes and two headers", f"Heizwasser {WATER_L:.1f} l in den vier Rohren und zwei Sammlern"), "bone": "panel", "x": 3.0, "y": r6(TUBE_Y - Y0), "z": 0, "fxN": 0, "fyN": r6(-WATER_L * 0.98 * 9.81), "source": "authored-estimate", "reference": "radiant-panel-catalogues"},
        {"id": "flow-hose", "kind": "imposed", "title": T("Flexible flow hose hanging on the flow stub, 50 N", "Vorlauf-Schlauch am Vorlaufstutzen, 50 N"), "anchor": "flow-connection", "fxN": 0, "fyN": -50, "source": "authored-estimate"},
        {"id": "return-hose", "kind": "imposed", "title": T("Flexible return hose hanging on the return stub, 50 N", "Rücklauf-Schlauch am Rücklaufstutzen, 50 N"), "anchor": "return-connection", "fxN": 0, "fyN": -50, "source": "authored-estimate"},
    ]
    off = {"heating": 0}
    hung = {"panel": {"dy": 5.90}, "bearers": {"dx": 6.60}}
    poses = [
        {"id": "stored", "title": T("Stored on bearers", "Auf Kanthölzern gelagert"), "description": T(
            "Drained and capped, the strip lies radiant face down with its 6 m plate on three squared timbers in the hall of the component exchange, the suspension chains cut off at the eye nuts of its four traverses.",
            "Entleert und verschlossen liegt die Platte mit der Strahlfläche nach unten mit ihrem 6 m langen Blech auf drei Kanthölzern in der Halle der Bauteilbörse, die Aufhängeketten an den Ringmuttern ihrer vier Traversen abgetrennt."),
         "bones": {}, "ground": "authored", "contacts": ground, "thermal": {"operating": off, "regions": binding(regions), "emitters": {"convection-plume": 0}}, "loads": []},
        {"id": "lifted", "title": T("Lifted on the spreader sling", "Am Traversengehänge angehoben"), "description": T(
            f"A four-leg sling under a spreader beam hangs the {M_PANEL:.0f} kg strip 0.40 m above its bearers by the eye nuts of the two outer traverses, 4.5 m apart, so the 6 m plate does not sag; the bearers stay on the floor.",
            f"Ein vierstrangiges Gehänge unter einer Traverse hält die {M_PANEL:.0f} kg schwere Platte an den Ringmuttern der beiden äußeren Traversen, 4,5 m auseinander, 0,40 m über ihren Kanthölzern, sodass das 6 m lange Blech nicht durchhängt; die Kanthölzer bleiben am Boden."),
         "bones": {"panel": {"dy": 0.40}}, "ground": "authored", "contacts": sling, "thermal": {"operating": off, "regions": binding(regions, True), "emitters": {"convection-plume": 0}}, "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled at 6 m, radiating", "In 6 m wieder eingebaut, strahlend"), "description": T(
            "Hung again by eight chains from the roof of an industrial hall with its radiant face 6.0 m above the floor, filled and fed at 75/65 degC: the plate radiates about 2.6 kW down into the 15 degC hall and a warm convection plume rises over it; the bearers lie beside it on the floor.",
            "An acht Ketten wieder unter das Dach einer Industriehalle gehängt, die Strahlfläche 6,0 m über dem Boden, befüllt und mit 75/65 °C versorgt: die Platte strahlt etwa 2,6 kW nach unten in die 15 °C kalte Halle, und über ihr steigt eine warme Konvektionsfahne auf; die Kanthölzer liegen daneben am Boden."),
         "bones": hung, "ground": "authored", "contacts": eyes, "thermal": {"operating": {"heating": 1}, "regions": binding(regions, True), "emitters": {"convection-plume": 1}}, "loads": service_loads},
        {"id": "reinstalled-off", "title": T("Reinstalled at 6 m, filled and cold", "In 6 m wieder eingebaut, befüllt und kalt"), "description": T(
            f"The same hall state with the heating circuit off: the {WATER_L:.0f} l of water and the two hoses load the eight chains, everything is at hall temperature.",
            f"Derselbe Zustand in der Halle bei abgeschaltetem Heizkreis: die {WATER_L:.0f} l Wasser und die zwei Schläuche belasten die acht Ketten, alles hat Hallentemperatur."),
         "bones": hung, "ground": "authored", "contacts": eyes, "thermal": {"operating": off, "regions": binding(regions, True), "emitters": {"convection-plume": 0}}, "loads": service_loads},
    ]
    clips = [
        {"id": "lift", "title": T("Lift off the bearers", "Von den Kanthölzern abheben"), "description": T(
            "The spreader sling takes the weight after two seconds and the crane lifts the strip 0.40 m clear of its bearers in six seconds.",
            "Das Traversengehänge übernimmt nach zwei Sekunden das Gewicht, und der Kran hebt die Platte in sechs Sekunden 0,40 m über ihre Kanthölzer."),
         "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}, {"t": 10, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 2.2} for a in ground] + [{"anchor": a, "fromS": 2, "toS": 10} for a in sling],
         "phases": [{"id": "sling-on", "fromS": 0, "toS": 2, "title": T("Slinging", "Anschlagen")}, {"id": "hoist", "fromS": 2, "toS": 8, "title": T("Hoisting", "Heben")}, {"id": "hold", "fromS": 8, "toS": 10, "title": T("Holding", "Halten")}],
         "events": [{"t": 2.2, "kind": "contact-off", "title": T("Plate leaves the bearers", "Blech verlässt die Kanthölzer"), "target": "panel"}],
         "tracks": []},
        {"id": "heat-up", "title": T("Heating up after reinstallation", "Aufheizen nach dem Wiedereinbau"), "description": T(
            "The refilled strip hangs cold; the heating circuit opens after one minute and the plate, the tubes and the headers warm up until the water reaches 75/65 degC after 15 minutes and the plate radiates at full output.",
            "Die wieder befüllte Platte hängt kalt; der Heizkreis öffnet nach einer Minute, und Blech, Rohre und Sammler erwärmen sich, bis das Wasser nach 15 Minuten 75/65 °C erreicht und die Platte mit voller Leistung strahlt."),
         "durationS": 1200, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 60, "pose": "reinstalled", "ease": "step"}, {"t": 1200, "pose": "reinstalled", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 1200} for a in eyes],
         "phases": [{"id": "cold", "fromS": 0, "toS": 60, "title": T("Cold", "Kalt")}, {"id": "warm-up", "fromS": 60, "toS": 960, "title": T("Warming up", "Aufheizen")}, {"id": "design", "fromS": 960, "toS": 1200, "title": T("Design temperatures", "Auslegungstemperaturen")}],
         "events": [{"t": 60, "kind": "operating", "title": T("Heating circuit opens", "Heizkreis öffnet"), "target": "heating"}, {"t": 960, "kind": "operating", "title": T("Flow reaches 75 degC", "Vorlauf erreicht 75 °C"), "target": "heating"}],
         "tracks": [{"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 960, "v": 1}, {"t": 1200, "v": 1}]},
                    {"target": "emitter:convection-plume", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 960, "v": 1}, {"t": 1200, "v": 1}]}]},
    ]
    asset = {
        "id": SLUG, "category": "components", "title": T("Used Radiant Ceiling Heating Strip", "Gebrauchte Deckenstrahlplatte"), "summary": SUMMARY,
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "radiant-panel", "ceiling-panel", "industrial-hall", "sheet-steel", "steel-tube", "mineral-wool", "timber", "operating", "thermal"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": r6(TOP), "widthM": L, "depthM": 2 * BEARER_HALF, "massKg": r6(M_TOTAL)},
        "materials": MATERIALS, "rig": rig,
        "volumes": {"50": volume(50), "100": volume(100), "200": volume(200), "500": volume(500)},
        "anchors": anchors,
        "representations": {"50": representation(50), "100": representation(100), "200": representation(200), "500": representation(500)},
        "symbols": [], "poses": poses, "clips": clips, "thermal": THERMAL, "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(f"[DEBUG] panel {M_PANEL:.2f} kg com {COM_PANEL[0]:.4f} {COM_PANEL[1]:.4f}; bearers {M_BEARER:.2f} kg; total {M_TOTAL:.2f} kg; water {WATER_L:.2f} l; section A {SEC_A:.6f} I {SEC_I:.4e} yc {SEC_YC:.4f}")
    print("[DEBUG] " + ", ".join(f"{k} {m:.2f}" for k, (m, _) in MASSES.items()))
