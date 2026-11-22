"""🕳 Authoritative generator of the used floor trench convector: every hand-chosen number of the 2.0 m x 260 x 110 mm natural-convection trench heater with its copper-aluminium element, the rolled-up aluminium grille beside it and the two bearers, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-trench-convector"


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


ORIGIN = {"floor": (0.0, 0.0), "bearers": (0.0, 0.0), "grille": (0.0, 0.0), "convector": (0.0, 0.08)}

BEARER_S = 0.08
BEARER_XC = (0.15, 1.85)
BEARER_Z = (-0.17, 0.45)
YB = 0.08
SH = 0.00125
TX = (0.01, 1.99)
TZ = 0.12
YT = 0.188
FRAME_T = 0.002
FRAME_Z = (0.105, 0.13)
FRAME_X = 0.025
HX_X = (0.10, 1.90)
HX_Y = (0.10, 0.16)
HX_Z = (-0.11, 0.03)
TUBE_Y = 0.13
TUBE_Z = (-0.075, -0.005)
TUBE_R = 0.0075
TUBE_X = (0.085, 1.935)
CAP_X = (1.935, 1.958)
CAP_R = 0.011
BRACKETS_X = (0.30, 1.00, 1.70)
SIDE_BRACKETS = [(0.15, -1), (0.15, 1), (1.85, -1), (1.85, 1)]
ROLL_C = (1.85, 0.198)
ROLL_R, ROLL_RI = 0.118, 0.05
ROLL_Z = (0.16, 0.41)
GRILLE_BAR_H, GRILLE_PITCH = 0.018, 0.016
FIN_PITCH, FIN_T, FIN_N = 0.006, 0.00025, 300

M_BEARERS = 2 * BEARER_S * BEARER_S * (BEARER_Z[1] - BEARER_Z[0]) * 450.0
TROUGH_AREA = (TX[1] - TX[0]) * 2 * TZ + 2 * (TX[1] - TX[0]) * (YT - YB) + 2 * 2 * TZ * (YT - YB)
MASSES = {
    "trough": (TROUGH_AREA * SH * 7850.0, (1.0, 0.03)),
    "frame": ((2.0 * 2 * FRAME_Z[1] - (2.0 - 2 * FRAME_X) * 2 * FRAME_Z[0]) * FRAME_T * 2700.0, (1.0, 0.109)),
    "fins": (FIN_N * 0.14 * 0.06 * FIN_T * 2700.0, (1.0, 0.05)),
    "tubes": (2 * 1.85 * 0.000044 * 8930.0 + 0.08, (1.0, 0.05)),
    "element-brackets": (3 * 0.2, (1.0, 0.01)),
    "tails-caps": (0.30, (1.94, 0.05)),
    "side-brackets": (4 * 0.06, (1.0, 0.015)),
    "screed-remnants": (0.40, (0.75, 0.02)),
}
M_CONVECTOR = sum(m for m, _ in MASSES.values())
COM_CONV = (sum(m * c[0] for m, c in MASSES.values()) / M_CONVECTOR, sum(m * c[1] for m, c in MASSES.values()) / M_CONVECTOR)
GRILLE_BARS = round(2.0 / GRILLE_PITCH)
M_GRILLE = GRILLE_BARS * 0.25 * 0.162 + 0.40 + 0.05
M_TOTAL = M_BEARERS + M_CONVECTOR + M_GRILLE
WATER_KG = 0.55
ROLL_D_CHECK = math.sqrt(4 * 2.0 * GRILLE_BAR_H / math.pi + (2 * ROLL_RI) ** 2)

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
            x0, x1, cy, cz, r, ri = g
            prof = [[0, ri], [0, r], [r6(x1 - x0), r], [r6(x1 - x0), ri], [0, ri]] if ri else [[0, 0], [0, r], [r6(x1 - x0), r], [r6(x1 - x0), 0]]
            s = dict(base, kind="revolve", profile=prof, axis=[1, 0, 0], origin=[r6(x0 - ox), r6(cy - oy), cz])
        elif self.kind == "cyly":
            cx, cz, y0, y1, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(y1 - y0), r], [r6(y1 - y0), 0]], axis=[0, 1, 0], origin=[r6(cx - ox), r6(y0 - oy), cz])
        elif self.kind == "cylz":
            cx, cy, z0, z1, r, ri = g
            prof = [[0, ri], [0, r], [r6(z1 - z0), r], [r6(z1 - z0), ri], [0, ri]] if ri else [[0, 0], [0, r], [r6(z1 - z0), r], [r6(z1 - z0), 0]]
            s = dict(base, kind="revolve", profile=prof, axis=[0, 0, 1], origin=[r6(cx - ox), r6(cy - oy), z0])
        elif self.kind == "ringzx":
            x0, x1, z0, z1, w, y0, y1 = g
            outer = [[z0, r6(x0 - ox)], [z1, r6(x0 - ox)], [z1, r6(x1 - ox)], [z0, r6(x1 - ox)]]
            inner = [[r6(z0 + w), r6(x0 + w - ox)], [r6(z0 + w), r6(x1 - w - ox)], [r6(z1 - w), r6(x1 - w - ox)], [r6(z1 - w), r6(x0 + w - ox)]]
            s = dict(base, kind="extrude", plane="zx", profile=outer, holes=[inner], **{"from": r6(y0 - oy), "to": r6(y1 - oy)})
        for key in ("slender", "sheet", "texture", "envelope", "repeat", "tags", "sectionFaces"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def bounds(self):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of the part at rest."""
        g = self.geo
        if self.kind == "box":
            return g
        if self.kind == "cylx":
            x0, x1, cy, cz, r, _ = g
            return (x0, cy - r, cz - r), (x1, cy + r, cz + r)
        if self.kind == "cyly":
            cx, cz, y0, y1, r = g
            return (cx - r, y0, cz - r), (cx + r, y1, cz + r)
        if self.kind == "cylz":
            cx, cy, z0, z1, r, _ = g
            return (cx - r, cy - r, z0), (cx + r, cy + r, z1)
        if self.kind == "ringzx":
            x0, x1, z0, z1, _, y0, y1 = g
            return (x0, y0, z0), (x1, y1, z1)

    def shape(self, view):
        """✏ The filled 2D shape of the part in one viewpoint: the elevation in the bone frame, front and plan in the rest frame of the world."""
        ox, oy = ORIGIN[self.bone]
        (x0, y0, z0), (x1, y1, z1) = self.extra.get("span") or self.bounds()
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
            if self.kind == "cylz":
                cx, cy, _, _, r, ri = g
                return circle(cx - ox, cy - oy, r, ri)
            return rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)
        if view == "front":
            if self.kind == "cylx":
                _, _, cy, cz, r, ri = g
                return circle(cz, cy, r, ri)
            return rect(z0, y0, z1 - z0, y1 - y0)
        if self.kind == "cyly":
            cx, cz, _, _, r = g
            return circle(cx, cz, r)
        if self.kind == "ringzx":
            return dict(s, kind="path", d=frame_path(x0, x1, z0, z1, g[4]))
        return rect(x0, z0, x1 - x0, z1 - z0)


def ring_path(cx, cy, r, ri):
    """⭕ An SVG annulus: outer circle and the bore as a hole."""
    return (f"M {r6(cx - r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx + r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx - r)} {r6(cy)} Z "
            f"M {r6(cx - ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx + ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx - ri)} {r6(cy)} Z")


def frame_path(x0, x1, z0, z1, w):
    """🔲 An SVG frame in plan: the outer rectangle and the opening as a hole."""
    return (f"M {r6(x0)} {r6(z0)} L {r6(x1)} {r6(z0)} L {r6(x1)} {r6(z1)} L {r6(x0)} {r6(z1)} Z "
            f"M {r6(x0 + w)} {r6(z0 + w)} L {r6(x0 + w)} {r6(z1 - w)} L {r6(x1 - w)} {r6(z1 - w)} L {r6(x1 - w)} {r6(z0 + w)} Z")


SHEET = {"sheet": True}


def parts_of(scale):
    """🗂 The hand-chosen part list of one scale: every part the real trench convector has that this scale can show, generalised for its smallest feature."""
    P = []
    for k, xc in enumerate(BEARER_XC):
        P.append(Part(f"bearer-{k}", "bearers", "box", ((xc - BEARER_S / 2, 0.0, BEARER_Z[0]), (xc + BEARER_S / 2, BEARER_S, BEARER_Z[1])), "softwood", 0.62, "bearers",
                      extra={"radius": 0.004} if scale == 50 else {}))
    cx, cy = ROLL_C
    if scale == 500:
        P.append(Part("grille-roll", "grille", "cylz", (cx, cy, ROLL_Z[0], ROLL_Z[1], ROLL_R, None), "anodised-aluminium", 0.55, "grille"))
        P.append(Part("trough", "convector", "box", ((0.0, YB, -FRAME_Z[1]), (2.0, YT + FRAME_T, FRAME_Z[1])), "coated-steel", 0.2, ["trough", "frame", "heat-exchanger", "tails"]))
        return P
    P.append(Part("grille-roll", "grille", "cylz", (cx, cy, ROLL_Z[0], ROLL_Z[1], ROLL_R, ROLL_RI), "anodised-aluminium", 0.55, "grille"))
    x0, x1 = TX
    P.append(Part("trough-bottom", "convector", "box", ((x0, YB, -TZ), (x1, YB + SH, TZ)), "coated-steel", 0.2, "trough", views=("front", "plan"), extra=SHEET))
    P.append(Part("trough-side-window", "convector", "box", ((x0, YB + SH, -TZ), (x1, YT, -TZ + SH)), "coated-steel", 0.2, "trough", views=("elevation", "plan"), extra=SHEET))
    P.append(Part("trough-side-room", "convector", "box", ((x0, YB + SH, TZ - SH), (x1, YT, TZ)), "coated-steel", 0.2, "trough", views=("plan",), extra=SHEET))
    P.append(Part("trough-end-blind", "convector", "box", ((x0, YB + SH, -TZ + SH), (x0 + SH, YT, TZ - SH)), "coated-steel", 0.2, "trough", views=("elevation", "plan"), extra=SHEET))
    P.append(Part("trough-end-connection", "convector", "box", ((x1 - SH, YB + SH, -TZ + SH), (x1, YT, TZ - SH)), "coated-steel", 0.2, "trough", views=("elevation", "front", "plan"), extra=SHEET))
    P.append(Part("frame", "convector", "ringzx", (0.0, 2.0, -FRAME_Z[1], FRAME_Z[1], FRAME_Z[1] - FRAME_Z[0], YT, YT + FRAME_T), "anodised-aluminium", 0.5, "frame", extra=SHEET))
    hx_y0 = HX_Y[0] if scale != 200 else YB + SH
    if scale == 50:
        half = FIN_N // 2
        for k, name in enumerate(("blind", "connection")):
            xs = HX_X[0] + k * half * FIN_PITCH
            xf = xs + (FIN_PITCH - FIN_T) / 2
            P.append(Part(f"fins-{name}", "convector", "box", ((xf, HX_Y[0], HX_Z[0]), (xf + FIN_T, HX_Y[1], HX_Z[1])), "aluminium-fins", 0.42, "heat-exchanger", views=("elevation", "plan"),
                          extra={"sheet": True, "repeat": {"count": half, "translation": [FIN_PITCH, 0, 0]}, "span": ((xs, HX_Y[0], HX_Z[0]), (xs + half * FIN_PITCH, HX_Y[1], HX_Z[1]))}))
    else:
        P.append(Part("fin-pack", "convector", "box", ((HX_X[0], hx_y0, HX_Z[0]), (HX_X[1], HX_Y[1], HX_Z[1])), "aluminium-fins", 0.42, "heat-exchanger", views=("elevation", "plan"), extra={"envelope": True}))
    if scale == 50:
        for k, z in enumerate(TUBE_Z):
            P.append(Part(f"tube-{k}", "convector", "cylx", (TUBE_X[0], TUBE_X[1], TUBE_Y, z, TUBE_R, None), "copper", 0.38, "heat-exchanger", views=("plan",)))
        P.append(Part("return-bend", "convector", "box", ((TUBE_X[0] - 0.012, TUBE_Y - TUBE_R, TUBE_Z[0] - TUBE_R), (TUBE_X[0], TUBE_Y + TUBE_R, TUBE_Z[1] + TUBE_R)), "copper", 0.38, "heat-exchanger", views=("plan",)))
        for k, z in enumerate(TUBE_Z):
            P.append(Part(f"tail-cap-{k}", "convector", "cylx", (CAP_X[0], CAP_X[1], TUBE_Y, z, CAP_R, None), "brass", 0.3, "tails", views=("elevation", "plan") if k == 0 else ("plan",)))
        P.append(Part("bleed-valve", "convector", "cyly", (1.915, TUBE_Z[0], TUBE_Y + TUBE_R, 0.152, 0.005), "brass", 0.3, "tails", views=("plan",)))
    if scale == 100:
        P.append(Part("return-bend", "convector", "box", ((TUBE_X[0] - 0.012, TUBE_Y - TUBE_R, TUBE_Z[0] - TUBE_R), (HX_X[0], TUBE_Y + TUBE_R, TUBE_Z[1] + TUBE_R)), "copper", 0.38, "heat-exchanger", views=("plan",)))
        P.append(Part("tube-ends", "convector", "box", ((HX_X[1], TUBE_Y - TUBE_R, TUBE_Z[0] - TUBE_R), (CAP_X[0], TUBE_Y + TUBE_R, TUBE_Z[1] + TUBE_R)), "copper", 0.38, "heat-exchanger", views=("plan",)))
        for k, z in enumerate(TUBE_Z):
            P.append(Part(f"tail-cap-{k}", "convector", "box", ((CAP_X[0], TUBE_Y - CAP_R, z - CAP_R), (CAP_X[1], TUBE_Y + CAP_R, z + CAP_R)), "brass", 0.3, "tails", views=("elevation", "plan") if k == 0 else ("plan",)))
    if scale != 200:
        for k, xb in enumerate(BRACKETS_X):
            P.append(Part(f"element-bracket-{k}", "convector", "box", ((xb - 0.015, YB + SH, HX_Z[0]), (xb + 0.015, HX_Y[0], HX_Z[1])), "galvanised-steel", 0.3, "trough", views=("plan",)))
        for k, (xb, side) in enumerate(SIDE_BRACKETS):
            zs = sorted((side * TZ, side * (TZ + 0.025)))
            P.append(Part(f"levelling-bracket-{k}", "convector", "box", ((xb - 0.02, YB, zs[0]), (xb + 0.02, YB + 0.05, zs[1])), "galvanised-steel", 0.32, "trough",
                          views=("elevation", "plan") if side < 0 else ("plan",)))
    else:
        P.append(Part("tails", "convector", "box", ((HX_X[1], TUBE_Y - 0.015, -0.09), (CAP_X[1], TUBE_Y + 0.015, 0.01)), "copper", 0.36, "tails", views=("elevation", "plan")))
    return P


def details_of(scale):
    """🖋 The detail and texture strokes of one scale: rim seam, screed remnants, label, roll windings, fin pitch, dents and grain."""
    e, f, p = [], [], []
    if scale in (200, 500):
        return e, f, p
    oy = YB

    def line(lst, sid, bone, pts, role="detail", region=None, tone=0.25, z=9.0, closed=False, filled=False):
        s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
        if region:
            s["region"] = region
        lst.append(s)

    line(e, "rim-edge", "convector", [(0.0, YT - oy), (2.0, YT - oy)], region="frame", z=8.5)
    line(f, "rim-edge-front", "convector", [(-FRAME_Z[1], YT), (FRAME_Z[1], YT)], region="frame", z=6.0)
    line(p, "rim-inner-window", "convector", [(FRAME_X, -FRAME_Z[0]), (2.0 - FRAME_X, -FRAME_Z[0])], region="frame", z=7.0)
    line(p, "rim-inner-room", "convector", [(FRAME_X, FRAME_Z[0]), (2.0 - FRAME_X, FRAME_Z[0])], region="frame", z=7.0)
    if scale == 50:
        for k, (a, b, h) in enumerate(((0.32, 0.71, 0.035), (0.98, 1.22, 0.03), (1.46, 1.63, 0.032))):
            line(e, f"screed-remnant-{k}", "convector", [(a, 0.0), (b, 0.0), (b - 0.03, h * 0.6), ((a + b) / 2, h), (a + 0.02, h * 0.7)], role="texture", region="trough", tone=0.55, closed=True, filled=True, z=8.2)
        line(e, "label", "convector", [(1.62, 0.055), (1.74, 0.055), (1.74, 0.085), (1.62, 0.085)], role="texture", region="trough", tone=0.8, closed=True, filled=True, z=8.3)
        line(e, "label-text", "convector", [(1.635, 0.07), (1.725, 0.07)], role="texture", region="trough", z=8.4)
        cx, cy = ROLL_C
        for k, rr in enumerate((0.068, 0.086, 0.104)):
            line(e, f"roll-winding-{k}", "grille", [(cx + rr * math.cos(2 * math.pi * j / 24), cy + rr * math.sin(2 * math.pi * j / 24)) for j in range(25)], role="texture", region="grille", z=7.0)
        for k, xc in enumerate(BEARER_XC):
            line(f, f"bearer-grain-{k}", "bearers", [(BEARER_Z[0], 0.03 + 0.02 * k), (0.15, 0.032 + 0.02 * k), (BEARER_Z[1], 0.029 + 0.02 * k)], role="texture", region="bearers", z=5.5)
        for k in range(9):
            x = HX_X[0] + 0.1 + 0.2 * k
            line(p, f"fin-pitch-{k}", "convector", [(x, HX_Z[0]), (x, HX_Z[1])], role="texture", region="heat-exchanger", tone=0.3, z=6.2)
        for k, x in enumerate((0.55, 1.31)):
            line(p, f"dented-fins-{k}", "convector", [(x, HX_Z[1]), (x + 0.02, HX_Z[1] - 0.025), (x + 0.05, HX_Z[1])], region="heat-exchanger", tone=0.2, z=6.3)
        for k in range(12):
            zz = ROLL_Z[0] + 0.02 * (k + 1)
            line(p, f"grille-bar-{k}", "grille", [(ROLL_C[0] - ROLL_R, zz), (ROLL_C[0] + ROLL_R, zz)], role="texture", region="grille", z=6.5)
        for k, (lst, pts) in enumerate(((p, [(ROLL_C[0] - ROLL_R, 0.275), (ROLL_C[0] + ROLL_R, 0.275), (ROLL_C[0] + ROLL_R, 0.295), (ROLL_C[0] - ROLL_R, 0.295)]),
                                        (f, [(0.275, ROLL_C[1] - ROLL_R), (0.295, ROLL_C[1] - ROLL_R), (0.295, ROLL_C[1] + ROLL_R), (0.275, ROLL_C[1] + ROLL_R)]))):
            line(lst, f"strap-{k}", "grille", pts, role="texture", region="grille", tone=0.35, z=9.5, closed=True)
        line(p, "dust-cold-channel", "convector", [(0.2, 0.06), (0.7, 0.075), (1.3, 0.058), (1.8, 0.07)], role="texture", region="trough", tone=0.5, z=5.4)
    return e, f, p


def representation(scale):
    """🖨 The authored representation of one scale: the projected part shapes, the detail strokes, pens, hatches and decisions."""
    parts = parts_of(scale)
    geo = {"elevation": [], "front": [], "plan": []}
    merged = scale in (100, 200)
    for part in parts:
        for view in part.views:
            if merged and part.id.startswith("trough-") and not (view == "plan" and part.id == "trough-bottom"):
                continue
            if merged and part.id == "frame" and view != "plan":
                continue
            sh = part.shape(view)
            if scale == 50 and view == "elevation" and part.id.startswith("bearer-"):
                sh["hatch"] = "end-grain"
            if scale in (50, 100) and view == "plan" and (part.id == "fin-pack" or part.id.startswith("fins-")):
                sh["hatch"] = "fin-pack"
            if merged and part.id == "frame":
                sh["regions"] = ["frame", "trough"]
                sh.pop("region", None)
            geo[view].append(sh)
    if merged:
        common = {"bone": "convector", "role": "body", "filled": True, "tone": 0.2, "material": "coated-steel", "regions": ["trough", "frame"]}
        geo["elevation"].append(dict(common, id="trough-body", z=5.13, kind="rect", x=0.0, y=0.0, w=2.0, h=r6(YT + FRAME_T - YB)))
        geo["front"].append(dict(common, id="trough-body", z=7.0, kind="rect", x=-FRAME_Z[1], y=YB, w=r6(2 * FRAME_Z[1]), h=r6(YT + FRAME_T - YB)))
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
            500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]
    hatches = []
    if scale == 50:
        hatches.append({"id": "end-grain", "title": T("Sawn end grain of the softwood bearers", "Gesägtes Hirnholz der Nadelholz-Kanthölzer"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1})
    if scale in (50, 100):
        hatches.append({"id": "fin-pack", "title": T("Aluminium fins of the heat exchanger, seen from above", "Aluminiumlamellen des Wärmetauschers, von oben gesehen"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.4 if scale == 50 else 0.5, "penMm": 0.1})
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": DECISIONS_2D[scale]}


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: every region by reference to its body solids, the hidden element inside the 1:500 trough and the warm-air outlet over the fins."""
    out = []
    for part in parts:
        region = part.region if isinstance(part.region, str) else part.region[0]
        out.append({"id": f"th-{part.id}", "of": part.id, "region": region})
    if scale == 500:
        out.append({"id": "th-frame", "kind": "box", "bone": "convector", "material": "anodised-aluminium", "role": "body", "tone": 0.5, "size": [2.0, FRAME_T, r6(2 * FRAME_Z[1])],
                    "transform": {"translation": [1.0, r6(YT + FRAME_T / 2 - YB), 0]}, "region": "frame", "sheet": True})
        out.append({"id": "th-tails", "kind": "box", "bone": "convector", "material": "copper", "role": "body", "tone": 0.36, "size": [0.078, 0.075, 0.10],
                    "transform": {"translation": [r6(CAP_X[1] - 0.039), r6((YT + FRAME_T - YB) / 2), -0.04]}, "region": "tails"})
        out.append({"id": "th-element", "kind": "box", "bone": "convector", "material": "aluminium-fins", "role": "body", "tone": 0.42, "size": [r6(HX_X[1] - HX_X[0]), r6(HX_Y[1] - YB), r6(HX_Z[1] - HX_Z[0])],
                    "transform": {"translation": [1.0, r6((YB + HX_Y[1]) / 2 - YB), r6((HX_Z[0] + HX_Z[1]) / 2)]}, "region": "heat-exchanger"})
    pt = {50: 0.002, 100: 0.004, 200: 0.006, 500: 0.007}[scale]
    top = HX_Y[1] if scale != 500 else YT + FRAME_T
    out.append({"id": "th-warm-air-outlet", "kind": "box", "bone": "convector", "material": "aluminium-fins", "role": "detail", "tone": 0.42, "size": [r6(HX_X[1] - HX_X[0]), r6(pt), r6(HX_Z[1] - HX_Z[0])],
                "transform": {"translation": [1.0, r6(top + pt / 2 - YB), r6((HX_Z[0] + HX_Z[1]) / 2)]}, "region": "heat-exchanger", "sheet": True, "tags": ["warm-air", "trough-plume"]})
    return out


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: the two bearers, the trough as a folded steel channel, the element, the rolled grille, the bearings of the trough on the bearers and the centres of mass."""
    out = []
    for p in parts:
        if p.bone == "bearers":
            out.append({"id": f"member-{p.id}", "of": p.id, "member": "bearers", "part": "member"})
        elif p.id in ("trough", "trough-bottom", "trough-side-window", "trough-side-room"):
            out.append({"id": f"member-{p.id}", "of": p.id, "member": "convector", "part": "member"})
        elif p.id == "grille-roll":
            out.append({"id": f"member-{p.id}", "of": p.id, "member": "grille", "part": "member"})
    pad = max(FEATURE[scale], 0.008)
    if scale in (50, 100):
        for k, (x, z) in enumerate(((0.15, -0.10), (0.15, 0.10), (1.85, -0.10), (1.85, 0.10))):
            out.append({"id": f"joint-bearing-{k}", "kind": "box", "bone": "convector", "material": "galvanised-steel", "role": "detail", "tone": 0.5, "size": [max(0.04, 2 * pad), pad, max(0.04, 2 * pad)],
                        "transform": {"translation": [x, r6(pad / 2), z]}, "member": "convector", "part": "joint"})
    else:
        for k, x in enumerate(BEARER_XC):
            out.append({"id": f"joint-bearing-line-{k}", "kind": "box", "bone": "convector", "material": "galvanised-steel", "role": "detail", "tone": 0.5, "size": [r6(max(0.04, 2 * pad)), r6(pad), r6(2 * TZ)],
                        "transform": {"translation": [x, r6(pad / 2), 0]}, "member": "convector", "part": "joint"})
    mr = max(0.025, FEATURE[scale] / 2)
    out.append({"id": "mass-convector", "kind": "ellipsoid", "bone": "convector", "material": "coated-steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [r6(COM_CONV[0]), r6(max(COM_CONV[1], mr)), 0]}, "member": "convector", "part": "mass"})
    mb = min(mr, BEARER_S / 2)
    for k, xc in enumerate(BEARER_XC):
        out.append({"id": f"mass-bearer-{k}", "kind": "ellipsoid", "bone": "bearers", "material": "softwood", "role": "detail", "tone": 0.5, "radii": [mb, mb, mb],
                    "transform": {"translation": [xc, BEARER_S / 2, r6((BEARER_Z[0] + BEARER_Z[1]) / 2)]}, "member": "bearers", "part": "mass"})
    mg = min(mr, (ROLL_R - ROLL_RI) / 2) if scale != 500 else mr
    gy = ROLL_C[1] - (ROLL_R + ROLL_RI) / 2 if scale != 500 else ROLL_C[1]
    out.append({"id": "mass-grille", "kind": "ellipsoid", "bone": "grille", "material": "anodised-aluminium", "role": "detail", "tone": 0.5, "radii": [mg, mg, mg],
                "transform": {"translation": [ROLL_C[0], r6(gy), r6((ROLL_Z[0] + ROLL_Z[1]) / 2)]}, "member": "grille", "part": "mass"})
    return out


def volume(scale):
    """🧊 The authored volume of one scale: body from the part list, thermal and structure lists, and the decisions."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts if p.body]
    if scale == 50:
        body.append({"id": "type-label", "kind": "box", "bone": "convector", "material": "label-film", "role": "detail", "tone": 0.8, "size": [0.12, 0.03, 0.0005],
                     "transform": {"translation": [1.68, 0.07, r6(-TZ - 0.00025)]}, "texture": True, "sheet": True})
        for k, (a, b, h) in enumerate(((0.32, 0.71, 0.035), (0.98, 1.22, 0.03), (1.46, 1.63, 0.032))):
            body.append({"id": f"screed-remnant-{k}", "kind": "box", "bone": "convector", "material": "cement-screed", "role": "detail", "tone": 0.55, "size": [r6(b - a), r6(h * 0.8), 0.001],
                         "transform": {"translation": [r6((a + b) / 2), r6(h * 0.4), r6(-TZ - 0.0005)]}, "texture": True, "sheet": True})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


exec(open(os.path.join(HERE, "author-trench-convector-texts.py"), encoding="utf8").read())


def binding(regions, installed=False):
    """🌬 The region binding of a pose: everything indoors, nothing sunlit, the timber on the floor and, reinstalled, the trough bottom on the slab."""
    out = {}
    for r in regions:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "bearers":
            b = {"exposedFraction": 0.85, "sunlitFraction": 0, "contact": {"fraction": 0.15, "tempC": 20}}
        if r == "trough":
            b = {"exposedFraction": 0.75, "sunlitFraction": 0, "contact": {"fraction": 0.25, "tempC": 20}} if installed else {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 20}}
        out[r] = b
    return out


def document():
    """📄 The whole asset document."""
    regions = [r["id"] for r in THERMAL["regions"]]
    rig = {"kind": "mechanical", "root": "floor", "bones": [
        {"id": "floor", "parent": None, "title": T("Yard floor frame (massless, fixed root)", "Hallenboden-Bezug (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.01, "joint": {"type": "fixed"}},
        {"id": "bearers", "parent": "floor", "title": T("Two squared softwood bearers 80 x 80 x 620 mm across the trough (stay on the floor when the trough is lifted)", "Zwei Nadelholz-Kanthölzer 80 x 80 x 620 mm quer unter der Wanne (bleiben beim Anheben am Boden liegen)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 1.89, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.5},
         "mass": {"kg": r6(M_BEARERS), "comX": 1.0, "comY": 0.04, "distribution": "uniform", "source": "literature", "reference": "en-338"},
         "section": {"areaM2": r6(BEARER_S ** 2), "secondMomentM4": r6(BEARER_S ** 4 / 12), "extremeFibreM": r6(BEARER_S / 2), "material": "softwood", "title": T("Squared timber 80 x 80 mm", "Kantholz 80 x 80 mm")}},
        {"id": "grille", "parent": "bearers", "title": T("Roll-up aluminium grille 2000 x 250 mm, rolled to a coil of 236 mm and strapped (stays with the bearers)", "Rollrost aus Aluminium 2000 x 250 mm, zu einer Rolle von 236 mm gewickelt und umreift (bleibt bei den Kanthölzern)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.25, "joint": {"type": "fixed"},
         "mass": {"kg": r6(M_GRILLE), "comX": ROLL_C[0], "comY": ROLL_C[1], "distribution": "point", "source": "authored-estimate", "reference": "trench-convector-catalogues"}},
        {"id": "convector", "parent": "floor", "title": T("Trench convector: steel trough with frame, copper-aluminium element, capped tails (carried by two people at its ends)", "Unterflurkonvektor: Stahlwanne mit Rahmen, Kupfer-Aluminium-Wärmetauscher, verschlossene Anschlüsse (von zwei Personen an den Enden getragen)"),
         "x": 0, "y": YB, "angleDeg": 0, "lengthM": 2.0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.08, "maxM": 0.50},
         "mass": {"kg": r6(M_CONVECTOR), "comX": r6(COM_CONV[0]), "comY": r6(COM_CONV[1]), "distribution": "uniform", "source": "authored-estimate", "reference": "trench-convector-catalogues"},
         "section": {"areaM2": r6(TROUGH_SECTION_A), "secondMomentM4": r6(TROUGH_SECTION_I), "extremeFibreM": r6(TROUGH_SECTION_E), "material": "coated-steel", "title": T("Folded steel trough 240 x 108 mm, 1.25 mm sheet", "Abgekantete Stahlwanne 240 x 108 mm, Blech 1,25 mm")}},
    ]}
    anchors = []
    for k, (x, z) in enumerate(((0.15, BEARER_Z[0] + 0.04), (0.15, BEARER_Z[1] - 0.04), (1.85, BEARER_Z[0] + 0.04), (1.85, BEARER_Z[1] - 0.04))):
        anchors.append({"id": f"bearer-ground-{k}", "bone": "bearers", "x": x, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.08, "footprintWidthM": 0.08, "frictionCoefficient": 0.5,
                        "title": T(f"{'Blind-end' if x < 1 else 'Connection-end'} bearer, {'window' if z < 0 else 'room'} end on the yard floor", f"Kantholz am {'Blindende' if x < 1 else 'Anschlussende'}, {'fensterseitiges' if z < 0 else 'raumseitiges'} Ende auf dem Hallenboden")})
    for k, (x, z) in enumerate(((0.15, -0.10), (0.15, 0.10), (1.85, -0.10), (1.85, 0.10))):
        anchors.append({"id": f"trough-foot-{k}", "bone": "convector", "x": x, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.08, "footprintWidthM": 0.04, "frictionCoefficient": 0.4,
                        "title": T(f"Trough bottom over the {'blind-end' if x < 1 else 'connection-end'} levelling bracket, {'window' if z < 0 else 'room'} side", f"Wannenboden über dem Nivellierwinkel am {'Blindende' if x < 1 else 'Anschlussende'}, {'Fensterseite' if z < 0 else 'Raumseite'}")})
    for k, x in enumerate((0.0, 2.0)):
        anchors.append({"id": f"grip-{k}", "bone": "convector", "x": x, "y": r6(YT + FRAME_T - YB), "z": 0, "kind": "support", "support": "pin",
                        "title": T(f"{'Blind' if k == 0 else 'Connection'} end of the frame where one carrier holds the trough", f"{'Blindende' if k == 0 else 'Anschlussende'} des Rahmens, an dem ein Träger die Wanne hält")})
    anchors.append({"id": "warm-air-outlet", "bone": "convector", "x": 1.0, "y": r6(HX_Y[1] - YB), "z": r6((HX_Z[0] + HX_Z[1]) / 2), "kind": "emitter", "title": T("Top of the fin pack where the warm air rises out of the trough", "Oberseite des Lamellenpakets, wo die warme Luft aus der Wanne steigt")})
    for k, z in enumerate(TUBE_Z):
        anchors.append({"id": f"tail-{k}", "bone": "convector", "x": CAP_X[0], "y": r6(TUBE_Y - YB), "z": z, "kind": "load", "title": T(f"{'Flow' if k == 0 else 'Return'} tail where the connection hose is screwed on", f"{'Vorlauf' if k == 0 else 'Rücklauf'}anschluss, an dem der Anschlussschlauch angeschraubt wird")})
    ground = [f"bearer-ground-{k}" for k in range(4)]
    feet = [f"trough-foot-{k}" for k in range(4)]
    service_loads = [
        {"id": "element-water", "kind": "carried", "title": T("Heating water 0.55 l in the two copper tubes", "Heizwasser 0,55 l in den zwei Kupferrohren"), "bone": "convector", "x": 1.0, "y": r6(TUBE_Y - YB), "z": -0.04, "fxN": 0, "fyN": r6(-WATER_KG * 9.81), "source": "authored-estimate", "reference": "trench-convector-catalogues"},
        {"id": "flow-hose", "kind": "imposed", "title": T("Flexible flow connection hose on the flow tail, 25 N", "Flexibler Vorlauf-Anschlussschlauch am Vorlaufanschluss, 25 N"), "anchor": "tail-0", "fxN": 0, "fyN": -25, "source": "authored-estimate"},
        {"id": "return-hose", "kind": "imposed", "title": T("Flexible return connection hose on the return tail, 25 N", "Flexibler Rücklauf-Anschlussschlauch am Rücklaufanschluss, 25 N"), "anchor": "tail-1", "fxN": 0, "fyN": -25, "source": "authored-estimate"},
    ]
    off = {"heating": 0}
    off_em = {"warm-air": 0, "trough-plume": 0}
    on_em = {"warm-air": 1, "trough-plume": 1}
    poses = [
        {"id": "stored", "title": T("Stored on bearers, grille rolled up beside", "Auf Kanthölzern gelagert, Rollrost daneben"), "description": T(
            "Drained, with both tails capped, the trough lies on two squared timbers across it in the hall of the component exchange; the roll-up grille, taken out of the frame, lies strapped as a coil on the connection-end bearer beside the trough.",
            "Entleert und mit beiden verschlossenen Anschlüssen liegt die Wanne auf zwei quer darunter liegenden Kanthölzern in der Halle der Bauteilbörse; der aus dem Rahmen genommene Rollrost liegt umreift als Rolle auf dem Kantholz am Anschlussende neben der Wanne."),
         "bones": {}, "ground": "authored", "contacts": ground, "thermal": {"operating": off, "regions": binding(regions), "emitters": off_em}, "loads": []},
        {"id": "lifted", "title": T("Carried by two people", "Von zwei Personen getragen"), "description": T(
            f"Two workers hold the {M_CONVECTOR:.0f} kg trough at the two ends of its frame and carry it 0.30 m above the bearers; the bearers and the grille coil stay on the floor.",
            f"Zwei Arbeiter halten die {M_CONVECTOR:.0f} kg schwere Wanne an den beiden Rahmenenden und tragen sie 0,30 m über den Kanthölzern; Kanthölzer und Rostrolle bleiben am Boden."),
         "bones": {"convector": {"dy": 0.30}}, "ground": "authored", "contacts": ["grip-0", "grip-1"], "thermal": {"operating": off, "regions": binding(regions), "emitters": off_em}, "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled, heating at 75/65 degC", "Wieder eingebaut, Heizbetrieb 75/65 °C"), "description": T(
            "Set on its levelling brackets on the raw slab in front of a glazed facade, connected and filled, the element runs at 75 degC flow and 65 degC return; warm air rises out of the open trough over the fins. The grille coil and the bearers lie beside it until the screed is laid and the grille is rolled into the frame.",
            "Auf seinen Nivellierwinkeln auf der Rohdecke vor einer verglasten Fassade aufgestellt, angeschlossen und befüllt, läuft der Wärmetauscher mit 75 °C Vorlauf und 65 °C Rücklauf; warme Luft steigt über den Lamellen aus der offenen Wanne. Rostrolle und Kanthölzer liegen daneben, bis der Estrich eingebracht und der Rost in den Rahmen eingerollt ist."),
         "bones": {"convector": {"dy": -0.08}, "bearers": {"dx": 2.30}}, "ground": "authored", "contacts": feet, "thermal": {"operating": {"heating": 1}, "regions": binding(regions, True), "emitters": on_em}, "loads": service_loads},
        {"id": "reinstalled-off", "title": T("Reinstalled, filled and cold", "Wieder eingebaut, befüllt und kalt"), "description": T(
            "The same installed state with the valve closed: the element is filled and at room temperature, no air moves.",
            "Derselbe Einbauzustand bei geschlossenem Ventil: der Wärmetauscher ist befüllt und raumwarm, keine Luft bewegt sich."),
         "bones": {"convector": {"dy": -0.08}, "bearers": {"dx": 2.30}}, "ground": "authored", "contacts": feet, "thermal": {"operating": off, "regions": binding(regions, True), "emitters": off_em}, "loads": service_loads},
    ]
    clips = [
        {"id": "lift", "title": T("Lift off the bearers", "Von den Kanthölzern abheben"), "description": T(
            "The two carriers take hold of the frame ends for two seconds and lift the trough 0.30 m clear of its bearers in four seconds.",
            "Die zwei Träger fassen zwei Sekunden lang die Rahmenenden und heben die Wanne in vier Sekunden 0,30 m über ihre Kanthölzer."),
         "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 2.2} for a in ground] + [{"anchor": "grip-0", "fromS": 2, "toS": 8}, {"anchor": "grip-1", "fromS": 2, "toS": 8}],
         "phases": [{"id": "grip", "fromS": 0, "toS": 2, "title": T("Taking hold", "Anfassen")}, {"id": "lift", "fromS": 2, "toS": 6, "title": T("Lifting", "Anheben")}, {"id": "hold", "fromS": 6, "toS": 8, "title": T("Holding", "Halten")}],
         "events": [{"t": 2.2, "kind": "contact-off", "title": T("Trough leaves the bearers", "Wanne verlässt die Kanthölzer"), "target": "convector"}],
         "tracks": []},
        {"id": "heat-up", "title": T("Valve opens after reinstallation", "Ventil öffnet nach dem Wiedereinbau"), "description": T(
            "The filled element starts at room temperature; the valve opens after half a minute and the light copper-aluminium element reaches 75/65 degC within five minutes while the warm-air flow over the fins builds up.",
            "Der befüllte Wärmetauscher startet raumwarm; das Ventil öffnet nach einer halben Minute, und der leichte Kupfer-Aluminium-Wärmetauscher erreicht innerhalb von fünf Minuten 75/65 °C, während sich die Warmluftströmung über den Lamellen aufbaut."),
         "durationS": 600, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 30, "pose": "reinstalled", "ease": "step"}, {"t": 600, "pose": "reinstalled", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 600} for a in feet],
         "phases": [{"id": "cold", "fromS": 0, "toS": 30, "title": T("Valve closed", "Ventil geschlossen")}, {"id": "warm-up", "fromS": 30, "toS": 330, "title": T("Warming up", "Aufheizen")}, {"id": "design", "fromS": 330, "toS": 600, "title": T("Design temperatures", "Auslegungstemperaturen")}],
         "events": [{"t": 30, "kind": "operating", "title": T("Valve opens", "Ventil öffnet"), "target": "heating"}, {"t": 330, "kind": "operating", "title": T("Element at 75/65 degC", "Wärmetauscher bei 75/65 °C"), "target": "heating"}],
         "tracks": [{"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 330, "v": 1}, {"t": 600, "v": 1}]},
                    {"target": "emitter:warm-air", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 330, "v": 1}, {"t": 600, "v": 1}]},
                    {"target": "emitter:trough-plume", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 400, "v": 1}, {"t": 600, "v": 1}]}]},
    ]
    asset = {
        "id": SLUG, "category": "components", "title": T("Used Trench Convector", "Gebrauchter Unterflurkonvektor"), "summary": SUMMARY,
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "convector", "trench-convector", "floor-heater", "galvanised-steel", "copper", "aluminium", "grille", "timber", "operating", "thermal"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": r6(ROLL_C[1] + ROLL_R), "widthM": 2.0, "depthM": r6(BEARER_Z[1] - BEARER_Z[0]), "massKg": r6(M_TOTAL)},
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
    print(f"[DEBUG] convector {M_CONVECTOR:.2f} kg com {COM_CONV[0]:.4f} {COM_CONV[1]:.4f}; grille {M_GRILLE:.2f} kg ({GRILLE_BARS} bars, coil {ROLL_D_CHECK:.3f} m); bearers {M_BEARERS:.2f} kg; total {M_TOTAL:.2f} kg; trough sheet {TROUGH_AREA:.3f} m2")
