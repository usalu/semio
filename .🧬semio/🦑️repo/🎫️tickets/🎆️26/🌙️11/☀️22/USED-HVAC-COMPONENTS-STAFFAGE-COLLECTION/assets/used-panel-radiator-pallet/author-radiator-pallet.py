"""🛒 Authoritative generator of the used panel radiator pallet: eight type 22 radiators 600 x 1000 mm standing on edge on two battens in a tube rack on a Euro pallet, seven strapped, the eighth with its wall brackets taped on and its thermostat head, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-panel-radiator-pallet"


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


RAD_X = (0.06, 1.06)
RAD_Y = (0.224, 0.824)
RAD_D = 0.100
R1_Z = (-0.4015, -0.3015)
STACK_Z0 = -0.30
PANEL_T = 0.020
COVER_T = 0.0015
GRILLE_T = 0.014
GRILLE_RECESS = 0.002
HEAD = (1.075, 1.15, 0.764, 0.026)
VALVE = (1.06, 1.075, 0.764, 0.015)
CAP = (1.06, 1.08, 0.764, 0.015)
NIPPLES = (0.985, 1.035)
NIPPLE = (0.199, 0.229, 0.012)
BRACKETS = ((0.19, 0.23), (0.89, 0.93))
BRACKET_Y = (0.254, 0.414)
BRACKET_Z = (-0.4365, -0.4015)
STRAPS = ((0.39, 0.44), (0.71, 0.76))
STRAP_T = 0.0015
BATTENS = ((0.18, 0.26), (0.86, 0.94))
BATTEN_Y = (0.144, 0.224)
POSTS_X = ((0.01, 0.04), (1.165, 1.195))
POST_Z = ((-0.40, -0.37), (0.37, 0.40))
RACK_TOP = (0.77, 0.80)
RACK_BASE = (0.144, 0.174)
TUBE = 0.03
BLOCKS_X = ((0.0, 0.145), (0.5275, 0.6725), (1.055, 1.2))
ROWS_Z = ((-0.40, -0.30), (-0.0725, 0.0725), (0.30, 0.40))
DECK_Z = ((-0.40, -0.255), (-0.21375, -0.11375), (-0.0725, 0.0725), (0.11375, 0.21375), (0.255, 0.40))
PAL_Y = (0.0, 0.022, 0.100, 0.122, 0.144)

ORIGIN = {"floor": (0.0, 0.0, 0.0), "pallet": (0.0, 0.133, 0.0), "stack": (0.06, 0.524, 0.05), "carrier": (0.06, 0.524, -0.3515), "radiator": (0.06, 0.524, -0.3515)}

RHO_WOOD, RHO_STEEL = 540.0, 7850.0
V_PALLET = (1.2 * 0.022 * sum(b - a for a, b in DECK_Z) + 3 * 0.8 * 0.145 * 0.022
            + 0.078 * 0.145 * sum(b - a for a, b in ROWS_Z) * 3 + 1.2 * 0.022 * sum(b - a for a, b in ROWS_Z))
M_PALLET = V_PALLET * RHO_WOOD + 0.6
M_BATTENS = 2 * 0.08 * 0.08 * 0.80 * 450.0
TUBE_A = 2.14e-4
M_RACK = 2 * TUBE_A * RHO_STEEL * (2 * (RACK_TOP[0] - 0.144) + 0.80 + 0.74)
M_RAD = 28.5
M_BRACKET = 0.30
M_HEAD = 0.22
M_STRAP = 0.45
M_CAP = 0.01
M_POOL = M_PALLET + M_BATTENS + M_RACK
M_STACK = 7 * (M_RAD + M_CAP) + 2 * M_STRAP
M_R1 = M_RAD + M_HEAD + 2 * M_BRACKET
M_TOTAL = M_POOL + M_STACK + M_R1
WATER_KG = 5.8
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


class Part:
    """🧱 One physical part of one scale: its 3D solid in the bone frame and its three 2D projections from the same numbers."""

    def __init__(self, pid, bone, kind, geo, material, tone, region, body=True, role="body", extra=None, reps=1, dz=0.0):
        self.id, self.bone, self.kind, self.geo, self.material, self.tone, self.region = pid, bone, kind, geo, material, tone, region
        self.body, self.role, self.extra, self.reps, self.dz = body, role, extra or {}, reps, dz

    def solid(self):
        """🧊 The volume solid of the part in the rest frame of its bone, repeated along z where the part is a row."""
        ox, oy, oz = ORIGIN[self.bone]
        g = self.geo
        base = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            s = dict(base, kind="box", size=[r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2 - oz)]})
            if self.extra.get("radius"):
                s["cornerRadius"] = self.extra["radius"]
        elif self.kind == "cylx":
            x0, x1, cy, cz, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(x1 - x0), r], [r6(x1 - x0), 0]], axis=[1, 0, 0], origin=[r6(x0 - ox), r6(cy - oy), r6(cz - oz)])
        elif self.kind == "cyly":
            cx, cz, y0, y1, r = g
            s = dict(base, kind="revolve", profile=[[0, 0], [0, r], [r6(y1 - y0), r], [r6(y1 - y0), 0]], axis=[0, 1, 0], origin=[r6(cx - ox), r6(y0 - oy), r6(cz - oz)])
        elif self.kind == "loop":
            x0, x1, (ya, yb, za, zb), t = g
            outer = [[r6(ya - t - oy), r6(za - t - oz)], [r6(yb + t - oy), r6(za - t - oz)], [r6(yb + t - oy), r6(zb + t - oz)], [r6(ya - t - oy), r6(zb + t - oz)]]
            hole = [[r6(ya - oy), r6(za - oz)], [r6(ya - oy), r6(zb - oz)], [r6(yb - oy), r6(zb - oz)], [r6(yb - oy), r6(za - oz)]]
            s = dict(base, kind="extrude", plane="yz", profile=outer, holes=[hole], **{"from": r6(x0 - ox), "to": r6(x1 - ox)})
        if self.reps > 1:
            s["repeat"] = {"count": self.reps, "translation": [0, 0, r6(self.dz)]}
        for key in ("slender", "sheet", "texture", "tags"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def bounds(self, k=0):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of instance k of the part at rest."""
        g, d = self.geo, k * self.dz
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
        elif self.kind == "cylx":
            x0, x1, cy, cz, r = g
            y0, y1, z0, z1 = cy - r, cy + r, cz - r, cz + r
        elif self.kind == "cyly":
            cx, cz, y0, y1, r = g
            x0, x1, z0, z1 = cx - r, cx + r, cz - r, cz + r
        else:
            x0, x1, (ya, yb, za, zb), t = g
            y0, y1, z0, z1 = ya - t, yb + t, za - t, zb + t
        return (x0, y0, z0 + d), (x1, y1, z1 + d)

    def shapes(self, view):
        """✏ The filled 2D shapes of every instance of the part in one viewpoint, in that viewpoint's frame."""
        out = []
        ox, oy, _ = ORIGIN[self.bone]
        for k in range(self.reps):
            (x0, y0, z0), (x1, y1, z1) = self.bounds(k)
            depth = {"elevation": 5 - z0, "front": 5 + x1, "plan": 5 + y1}[view]
            sid = self.id if self.reps == 1 else f"{self.id}-{k}"
            s = {"id": sid, "bone": self.bone, "role": self.role, "z": round(depth, 4), "filled": True, "tone": self.tone, "material": self.material}
            if isinstance(self.region, list):
                s["regions"] = self.region
            else:
                s["region"] = self.region

            def rect(x, y, w, h):
                return dict(s, kind="rect", x=r6(x), y=r6(y), w=r6(w), h=r6(h))

            def disc(cx, cy, r):
                return dict(s, kind="ellipse", cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r))

            if view == "elevation":
                if self.kind == "cylx" or self.kind == "cyly" or self.kind == "box" or self.kind == "loop":
                    out.append(rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0))
            elif view == "front":
                if self.kind == "cylx":
                    out.append(disc((z0 + z1) / 2, (y0 + y1) / 2, (y1 - y0) / 2))
                elif self.kind == "loop":
                    _, _, (ya, yb, za, zb), t = self.geo
                    out.append(dict(s, kind="path", d=f"M {r6(za - t)} {r6(ya - t)} L {r6(zb + t)} {r6(ya - t)} L {r6(zb + t)} {r6(yb + t)} L {r6(za - t)} {r6(yb + t)} Z M {r6(za)} {r6(ya)} L {r6(za)} {r6(yb)} L {r6(zb)} {r6(yb)} L {r6(zb)} {r6(ya)} Z"))
                else:
                    out.append(rect(z0, y0, z1 - z0, y1 - y0))
            else:
                if self.kind == "cyly":
                    out.append(disc((x0 + x1) / 2, (z0 + z1) / 2, (x1 - x0) / 2))
                else:
                    out.append(rect(x0, z0, x1 - x0, z1 - z0))
        return out


def radiator_parts(scale, pid, bone, z0, reps, region, rest_region):
    """♨ The parts of one type 22 radiator (or a row of them): two profiled water panels, the convector between them, the top grille and the two side covers at 1:50; one box above."""
    z1 = z0 + RAD_D
    x0, x1 = RAD_X
    y0, y1 = RAD_Y
    P = []
    if scale == 50:
        P.append(Part(f"{pid}-panel-front", bone, "box", ((x0 + COVER_T, y0, z0), (x1 - COVER_T, y1, z0 + PANEL_T)), "painted-steel", 0.86, region, extra={"radius": 0.004}, reps=reps, dz=RAD_D))
        P.append(Part(f"{pid}-panel-rear", bone, "box", ((x0 + COVER_T, y0, z1 - PANEL_T), (x1 - COVER_T, y1, z1)), "painted-steel", 0.86, region, extra={"radius": 0.004}, reps=reps, dz=RAD_D))
        P.append(Part(f"{pid}-convector", bone, "box", ((x0 + COVER_T, y0 + 0.005, z0 + PANEL_T), (x1 - COVER_T, y1 - GRILLE_T, z1 - PANEL_T)), "painted-steel", 0.55, rest_region, reps=reps, dz=RAD_D))
        P.append(Part(f"{pid}-grille", bone, "box", ((x0 + COVER_T, y1 - GRILLE_T, z0 + PANEL_T), (x1 - COVER_T, y1 - GRILLE_RECESS, z1 - PANEL_T)), "painted-steel", 0.8, region, reps=reps, dz=RAD_D))
        for side, (a, b) in (("left", (x0, x0 + COVER_T)), ("right", (x1 - COVER_T, x1))):
            P.append(Part(f"{pid}-cover-{side}", bone, "box", ((a, y0, z0), (b, y1, z1)), "painted-steel", 0.84, region, extra={"sheet": True}, reps=reps, dz=RAD_D))
    elif reps == 1:
        P.append(Part(pid, bone, "box", ((x0, y0, z0), (x1, y1, z1)), "painted-steel", 0.86, region))
    else:
        P.append(Part(pid, bone, "box", ((x0, y0, z0), (x1, y1, z0 + reps * RAD_D)), "painted-steel", 0.86, region))
    return P


def parts_of(scale):
    """🗂 The hand-chosen part list of one scale: every part the real pallet has that this scale can show, generalised for its smallest feature."""
    P = []
    if scale == 200:
        for i, (a, b) in enumerate(BLOCKS_X):
            for j, (c, d) in enumerate(ROWS_Z):
                P.append(Part(f"pallet-foot-{i}{j}", "pallet", "box", ((a, PAL_Y[0], c), (b, PAL_Y[2], d)), "softwood", 0.6, "pallet"))
        P.append(Part("pallet-deck", "pallet", "box", ((0.0, PAL_Y[2], -0.40), (1.2, PAL_Y[4], 0.40)), "softwood", 0.66, "pallet"))
    else:
        rad = {"radius": 0.003} if scale == 50 else {}
        for j, (c, d) in enumerate(ROWS_Z):
            P.append(Part(f"pallet-bottom-board-{j}", "pallet", "box", ((0.0, PAL_Y[0], c), (1.2, PAL_Y[1], d)), "softwood", 0.58, "pallet", extra=rad))
        for i, (a, b) in enumerate(BLOCKS_X):
            for j, (c, d) in enumerate(ROWS_Z):
                P.append(Part(f"pallet-block-{i}{j}", "pallet", "box", ((a, PAL_Y[1], c), (b, PAL_Y[2], d)), "softwood", 0.5, "pallet", extra=rad))
            if scale == 50:
                P.append(Part(f"pallet-stringer-{i}", "pallet", "box", ((a, PAL_Y[2], -0.40), (b, PAL_Y[3], 0.40)), "softwood", 0.62, "pallet", extra=rad))
        if scale == 50:
            for j, (c, d) in enumerate(DECK_Z):
                P.append(Part(f"pallet-deck-board-{j}", "pallet", "box", ((0.0, PAL_Y[3], c), (1.2, PAL_Y[4], d)), "softwood", 0.66, "pallet", extra=rad))
        else:
            P.append(Part("pallet-deck", "pallet", "box", ((0.0, PAL_Y[2], -0.40), (1.2, PAL_Y[4], 0.40)), "softwood", 0.66, "pallet"))
    for k, (a, b) in enumerate(BATTENS):
        P.append(Part(f"batten-{k}", "pallet", "box", ((a, BATTEN_Y[0], -0.40), (b, BATTEN_Y[1], 0.40)), "softwood", 0.7, "pallet", extra={"radius": 0.003} if scale == 50 else {}))
    for e, (a, b) in enumerate(POSTS_X):
        end = "left" if e == 0 else "right"
        for s, (c, d) in enumerate(POST_Z):
            P.append(Part(f"rack-post-{end}-{s}", "pallet", "box", ((a, RACK_BASE[0], c), (b, RACK_TOP[0], d)), "rack-steel", 0.3, "rack"))
        P.append(Part(f"rack-top-{end}", "pallet", "box", ((a, RACK_TOP[0], -0.40), (b, RACK_TOP[1], 0.40)), "rack-steel", 0.3, "rack"))
        P.append(Part(f"rack-base-{end}", "pallet", "box", ((a, RACK_BASE[0], -0.37), (b, RACK_BASE[1], 0.37)), "rack-steel", 0.3, "rack"))
    P += radiator_parts(scale, "radiator", "radiator", R1_Z[0], 1, "radiator", ["radiator", "convector"])
    P += radiator_parts(scale, "stack", "stack", STACK_Z0, 7, "stack", "stack")
    hx0, hx1, hy, hr = HEAD
    vx0, vx1, vy, vr = VALVE
    zc1 = (R1_Z[0] + R1_Z[1]) / 2
    if scale == 200:
        P.append(Part("valve-body", "radiator", "cylx", (1.06, 1.09, vy, zc1, vr), "brass", 0.35, "valve"))
        P.append(Part("thermostat-head", "radiator", "cylx", (1.09, hx1, hy, zc1, hr), "abs-plastic", 0.9, "thermostat-head"))
    else:
        P.append(Part("valve-body", "radiator", "cylx", (vx0, vx1, vy, zc1, vr), "brass", 0.35, "valve"))
        P.append(Part("thermostat-head", "radiator", "cylx", (hx0, hx1, hy, zc1, hr), "abs-plastic", 0.9, "thermostat-head"))
        cx0, cx1, cy, cr = CAP
        P.append(Part("stack-valve-cap", "stack", "cylx", (cx0, cx1, cy, STACK_Z0 + RAD_D / 2, cr), "abs-plastic", 0.12, "stack", reps=7, dz=RAD_D))
        ny0, ny1, nr = NIPPLE
        if scale == 50:
            for k, x in enumerate(NIPPLES):
                P.append(Part(f"radiator-nipple-{k}", "radiator", "cyly", (x, zc1, ny0, ny1, nr), "brass", 0.3, "valve"))
                P.append(Part(f"stack-nipple-{k}", "stack", "cyly", (x, STACK_Z0 + RAD_D / 2, ny0, ny1, nr), "abs-plastic", 0.15, "stack", reps=7, dz=RAD_D))
        else:
            na, nb = NIPPLES[0] - nr, NIPPLES[1] + nr
            P.append(Part("radiator-connections", "radiator", "box", ((na, ny0, zc1 - nr), (nb, RAD_Y[0], zc1 + nr)), "brass", 0.3, "valve"))
            P.append(Part("stack-connections", "stack", "box", ((na, ny0, STACK_Z0 + RAD_D / 2 - nr), (nb, RAD_Y[0], STACK_Z0 + RAD_D / 2 + nr)), "abs-plastic", 0.15, "stack", reps=7, dz=RAD_D))
    for k, (a, b) in enumerate(BRACKETS):
        P.append(Part(f"bracket-{k}", "radiator", "box", ((a, BRACKET_Y[0], BRACKET_Z[0]), (b, BRACKET_Y[1], BRACKET_Z[1])), "painted-steel", 0.7, "brackets"))
    za, zb, t = STACK_Z0, STACK_Z0 + 7 * RAD_D, STRAP_T
    for k, (a, b) in enumerate(STRAPS):
        for run, lo, hi in (("top", (a, RAD_Y[1], za - t), (b, RAD_Y[1] + t, zb + t)), ("bottom", (a, RAD_Y[0] - t, za - t), (b, RAD_Y[0], zb + t)),
                            ("near", (a, RAD_Y[0], za - t), (b, RAD_Y[1], za)), ("far", (a, RAD_Y[0], zb), (b, RAD_Y[1], zb + t))):
            P.append(Part(f"strap-{k}-{run}", "stack", "box", (lo, hi), "polyester", 0.2, "straps", extra={"sheet": True}))
    return P


def details_of(scale):
    """🖋 The open and closed detail and texture strokes of one scale: profile channels of the front panels, grille slots, tape over the brackets, grain, strap buckles, wear and old labels."""
    e, f, p = [], [], []
    ox, oy, _ = ORIGIN["radiator"]
    px, py, _ = ORIGIN["pallet"]

    def line(lst, sid, bone, pts, role="detail", region=None, tone=0.25, z=9.0, closed=False, filled=False):
        s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
        if region:
            s["region"] = region
        lst.append(s)

    zn = 5 - R1_Z[0] + 0.001
    if scale == 50:
        for k in range(1, 18):
            y = RAD_Y[0] + k * 0.6 / 18
            line(e, f"panel-channel-{k}", "radiator", [(RAD_X[0] + 0.01 - ox, y - oy), (RAD_X[1] - 0.01 - ox, y - oy)], role="texture", region="radiator", tone=0.7, z=zn)
        for k, (a, b) in enumerate(BRACKETS):
            for j, yy in enumerate((BRACKET_Y[0] + 0.03, BRACKET_Y[1] - 0.03)):
                line(e, f"tape-{k}-{j}", "radiator", [(a - 0.03 - ox, yy - 0.012 - oy), (b + 0.03 - ox, yy - 0.012 - oy), (b + 0.03 - ox, yy + 0.012 - oy), (a - 0.03 - ox, yy + 0.012 - oy)],
                     role="texture", region="brackets", tone=0.55, z=5 - BRACKET_Z[0] + 0.002, closed=True)
        line(e, "old-label", "radiator", [(0.12 - ox, 0.70 - oy), (0.26 - ox, 0.70 - oy), (0.26 - ox, 0.78 - oy), (0.12 - ox, 0.78 - oy)], role="texture", region="radiator", tone=0.95, z=zn + 0.001, closed=True, filled=True)
        line(e, "label-text", "radiator", [(0.14 - ox, 0.74 - oy), (0.24 - ox, 0.74 - oy)], role="texture", region="radiator", z=zn + 0.002)
        for k, x in enumerate((0.33, 0.58, 0.81)):
            line(e, f"scuff-{k}", "radiator", [(x - ox, 0.30 - oy), (x + 0.05 - ox, 0.315 - oy)], role="texture", region="radiator", tone=0.5, z=zn + 0.001)
        line(e, "batten-grain-0", "pallet", [(0.185 - px, 0.16 - py), (0.255 - px, 0.20 - py)], role="texture", region="pallet", z=5.402)
        for j in range(3):
            line(e, f"block-grain-{j}", "pallet", [(BLOCKS_X[j][0] + 0.02 - px, 0.05 - py), (BLOCKS_X[j][1] - 0.02 - px, 0.06 - py)], role="texture", region="pallet", z=5.401)
        line(e, "deck-board-grain", "pallet", [(0.02 - px, 0.133 - py), (1.18 - px, 0.134 - py)], role="texture", region="pallet", z=5.401)
        for k in range(1, 12):
            x = RAD_X[0] + k * 1.0 / 12
            line(p, f"grille-slot-{k}", "radiator", [(x, R1_Z[0] + PANEL_T + 0.006), (x, R1_Z[1] - PANEL_T - 0.006)], role="texture", region="radiator", tone=0.5, z=5.83)
        for k, (a, b) in enumerate(STRAPS):
            line(f, f"strap-buckle-{k}", "stack", [(0.40, 0.50), (0.4015, 0.50), (0.4015, 0.56), (0.40, 0.56)], role="texture", region="straps", z=6.3, closed=True)
            line(p, f"strap-buckle-plan-{k}", "stack", [(a, 0.4015), (b, 0.4015)], role="texture", region="straps", z=5.83)
        line(f, "valve-head-scale", "radiator", [(-0.3515 - 0.018, 0.764), (-0.3515 + 0.018, 0.764)], role="texture", region="thermostat-head", z=6.2)
    if scale in (50, 100):
        line(e, "radiator-grille-edge", "radiator", [(RAD_X[0] - ox, RAD_Y[1] - GRILLE_T - oy), (RAD_X[1] - ox, RAD_Y[1] - GRILLE_T - oy)], region="radiator", z=zn)
        line(p, "radiator-panel-joint", "radiator", [(RAD_X[0], R1_Z[0] + PANEL_T), (RAD_X[1], R1_Z[0] + PANEL_T)], region="radiator", z=5.83)
        for k in range(1, 8):
            zz = STACK_Z0 + k * RAD_D
            if k < 7:
                line(p, f"stack-joint-{k}", "stack", [(RAD_X[0], zz), (RAD_X[1], zz)], region="stack", z=5.83)
        for k in range(1, 7):
            zz = STACK_Z0 + k * RAD_D
            line(f, f"stack-joint-front-{k}", "stack", [(zz, RAD_Y[0]), (zz, RAD_Y[1])], region="stack", z=6.07)
    return e, f, p


def representation(scale):
    """🖨 The authored representation of one scale: the projected part shapes, the detail strokes, pens, hatches and decisions."""
    parts = parts_of(scale)
    geo = {"elevation": [], "front": [], "plan": []}
    for part in parts:
        for view in ("elevation", "front", "plan"):
            if scale == 200 and view == "front" and part.id.startswith("strap-") and part.id.endswith(("-near", "-far")):
                continue
            for sh in part.shapes(view):
                if scale == 50 and view == "elevation" and part.id.startswith("batten-"):
                    sh["hatch"] = "end-grain"
                if scale == 50 and view in ("elevation", "front") and part.id.startswith("pallet-block-"):
                    sh["h"] = r6(PAL_Y[3] - PAL_Y[1])
                geo[view].append(sh)
    if scale != 50:
        ox, oy, _ = ORIGIN["radiator"]
        geo["elevation"].append({"id": "convector-behind", "bone": "radiator", "role": "body", "z": round(5 - R1_Z[0] - PANEL_T, 4), "filled": True, "tone": 0.55, "material": "painted-steel", "region": "convector",
                                 "kind": "rect", "x": r6(RAD_X[0] + COVER_T - ox), "y": r6(RAD_Y[0] + 0.005 - oy), "w": r6(1.0 - 2 * COVER_T), "h": r6(0.6 - 0.005 - GRILLE_T)})
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    hatches = [{"id": "end-grain", "title": T("Sawn end grain of the softwood battens", "Gesägtes Hirnholz der Nadelholz-Kanthölzer"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}] if scale == 50 else []
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": DECISIONS_2D[scale]}


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: every visible region by reference, the convector of the reinstalled radiator (hidden box above 1:50) and the warm-air outlet patch on its grille."""
    out = []
    ox, oy, oz = ORIGIN["radiator"]
    for part in parts:
        region = part.region if isinstance(part.region, str) else part.region[0]
        out.append({"id": f"th-{part.id}", "of": part.id, "region": region})
    x0, x1 = RAD_X
    out.append({"id": "th-convector", "kind": "box", "bone": "radiator", "material": "painted-steel", "role": "body", "tone": 0.55,
                "size": [r6(x1 - x0 - 2 * COVER_T), r6(RAD_Y[1] - GRILLE_T - RAD_Y[0] - 0.005), r6(RAD_D - 2 * PANEL_T)],
                "transform": {"translation": [r6((x0 + x1) / 2 - ox), r6((RAD_Y[0] + 0.005 + RAD_Y[1] - GRILLE_T) / 2 - oy), 0]}, "region": "convector"})
    pt = FEATURE[scale]
    out.append({"id": "th-convection-outlet", "kind": "box", "bone": "radiator", "material": "painted-steel", "role": "detail", "tone": 0.8, "size": [0.9, pt, r6(RAD_D - 2 * PANEL_T)],
                "transform": {"translation": [0.5, r6(RAD_Y[1] + pt / 2 - oy), 0]}, "region": "radiator", "tags": ["convection-outlet"]})
    return out


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: pallet, battens and rack as the members of the pallet bone, the strapped row and the loose radiator as box members, their bearings on the battens as joints, and the centres of mass."""
    out = []
    for part in parts:
        if part.bone == "pallet":
            out.append({"id": f"member-{part.id}", "of": part.id, "member": "pallet", "part": "member"})
    sx, sy, sz = ORIGIN["stack"]
    rx, ry, rz = ORIGIN["radiator"]
    out.append({"id": "member-stack", "kind": "box", "bone": "stack", "material": "painted-steel", "role": "body", "tone": 0.3, "size": [1.0, 0.6, 0.7],
                "transform": {"translation": [0.5, 0, 0]}, "member": "stack", "part": "member"})
    out.append({"id": "member-radiator", "kind": "box", "bone": "radiator", "material": "painted-steel", "role": "body", "tone": 0.3, "size": [1.0, 0.6, RAD_D],
                "transform": {"translation": [0.5, 0, 0]}, "member": "radiator", "members": ["carrier"], "part": "member"})
    pad = max(FEATURE[scale], 0.008)
    for k, (a, b) in enumerate(BATTENS):
        xc = (a + b) / 2 - RAD_X[0]
        out.append({"id": f"joint-stack-batten-{k}", "kind": "box", "bone": "stack", "material": "steel", "role": "detail", "tone": 0.5, "size": [0.08, pad, 0.70],
                    "transform": {"translation": [r6(xc), r6(RAD_Y[0] - sy), 0]}, "member": "stack", "part": "joint"})
        out.append({"id": f"joint-radiator-batten-{k}", "kind": "box", "bone": "radiator", "material": "steel", "role": "detail", "tone": 0.5, "size": [0.08, pad, RAD_D],
                    "transform": {"translation": [r6(xc), r6(RAD_Y[0] - ry), 0]}, "member": "radiator", "part": "joint"})
    mr = max(0.03, FEATURE[scale])
    py = ORIGIN["pallet"][1]
    out.append({"id": "mass-pallet", "kind": "ellipsoid", "bone": "pallet", "material": "softwood", "role": "detail", "tone": 0.5, "radii": [mr, min(mr, 0.035), mr],
                "transform": {"translation": [0.6, r6(0.075 - py), 0]}, "member": "pallet", "part": "mass"})
    for k, (a, b) in enumerate(BATTENS):
        out.append({"id": f"mass-batten-{k}", "kind": "ellipsoid", "bone": "pallet", "material": "softwood", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                    "transform": {"translation": [r6((a + b) / 2), r6(sum(BATTEN_Y) / 2 - py), 0]}, "member": "pallet", "part": "mass"})
    for end, (a, b) in zip(("left", "right"), POSTS_X):
        out.append({"id": f"mass-rack-{end}", "kind": "ellipsoid", "bone": "pallet", "material": "rack-steel", "role": "detail", "tone": 0.5, "radii": [0.015, mr, 0.015],
                    "transform": {"translation": [r6((a + b) / 2), r6(0.46 - py), 0]}, "member": "pallet", "part": "mass"})
    out.append({"id": "mass-stack", "kind": "ellipsoid", "bone": "stack", "material": "painted-steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [0.5, 0, 0]}, "member": "stack", "part": "mass"})
    out.append({"id": "mass-radiator", "kind": "ellipsoid", "bone": "radiator", "material": "painted-steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, min(mr, 0.04)],
                "transform": {"translation": [r6(COM_R1[0]), r6(COM_R1[1]), 0]}, "member": "radiator", "part": "mass"})
    return out


def volume(scale):
    """🧊 The authored volume of one scale: body from the part list, thermal and structure lists, and the decisions."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts if p.body]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


COM_POOL = ((M_PALLET * 0.6 + M_BATTENS * 0.56 + M_RACK * 0.6) / M_POOL, (M_PALLET * 0.075 + M_BATTENS * 0.184 + M_RACK * 0.46) / M_POOL)
COM_R1 = ((M_RAD * 0.5 + M_HEAD * 1.05 + M_BRACKET * (0.15 + 0.85)) / M_R1, (M_RAD * 0.0 + M_HEAD * 0.24 + 2 * M_BRACKET * (-0.19)) / M_R1)

exec(open(os.path.join(HERE, "author-radiator-pallet-texts.py"), encoding="utf8").read())


def binding(pose):
    """🌬 The region binding of a pose: everything indoors, nothing sunlit, timber on the floor, radiator faces against their neighbours where they touch."""
    out = {}
    for r in [x["id"] for x in THERMAL["regions"]]:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "pallet":
            b = {"exposedFraction": 0.78, "sunlitFraction": 0, "contact": {"fraction": 0.22, "tempC": 18}}
        if r == "stack":
            b = {"exposedFraction": 0.45 if pose == "stored" else 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.55 if pose == "stored" else 0.4, "tempC": 18}}
        if r == "radiator" and pose == "stored":
            b = {"exposedFraction": 0.55, "sunlitFraction": 0, "contact": {"fraction": 0.45, "tempC": 18}}
        if r == "radiator" and pose.startswith("reinstalled"):
            b = {"exposedFraction": 0.97, "sunlitFraction": 0, "contact": {"fraction": 0.03, "tempC": 20}}
        if r == "brackets" and pose.startswith("reinstalled"):
            b = {"exposedFraction": 0.75, "sunlitFraction": 0, "contact": {"fraction": 0.25, "tempC": 20}}
        out[r] = b
    return out


def document():
    """📄 The whole asset document."""
    po, so, ro = ORIGIN["pallet"], ORIGIN["stack"], ORIGIN["radiator"]
    rig = {"kind": "mechanical", "root": "floor", "bones": [
        {"id": "floor", "parent": None, "title": T("Yard floor frame (massless, fixed root)", "Hallenboden-Bezug (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}},
        {"id": "pallet", "parent": "floor", "title": T("Euro pallet 1200 x 800 x 144 mm with two battens 80 x 80 mm and the two tube end frames of the rack (stay behind)", "Europalette 1200 x 800 x 144 mm mit zwei Kanthölzern 80 x 80 mm und den zwei Rohr-Stirnrahmen des Gestells (bleiben stehen)"),
         "x": po[0], "y": po[1], "angleDeg": 0, "lengthM": 1.2, "joint": {"type": "fixed"},
         "mass": {"kg": r6(M_POOL), "comX": r6(COM_POOL[0]), "comY": r6(COM_POOL[1] - po[1]), "distribution": "uniform", "source": "literature", "reference": "en-13698-1"}},
        {"id": "stack", "parent": "pallet", "title": T("Seven strapped type 22 radiators standing on edge, with their valve caps and the two straps", "Sieben verzurrte Heizkörper Typ 22 hochkant, mit ihren Ventilkappen und den zwei Spanngurten"),
         "x": r6(so[0] - po[0]), "y": r6(so[1] - po[1]), "z": so[2], "angleDeg": 0, "lengthM": 1.0, "joint": {"type": "fixed"},
         "mass": {"kg": r6(M_STACK), "comX": 0.5, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "radiator-catalogues"},
         "section": {"areaM2": r6(7 * 0.0042), "secondMomentM4": r6(7 * 2 * 1.25e-3 * 0.6 ** 3 / 12), "extremeFibreM": 0.3, "material": "painted-steel", "title": T("Seven pairs of 1.25 mm steel panels 0.6 m high", "Sieben Paare Stahlplatten 1,25 mm, 0,6 m hoch")}},
        {"id": "carrier", "parent": "floor", "title": T("Travel of the loose radiator along the floor (massless)", "Weg des losen Heizkörpers über den Boden (masselos)"),
         "x": ro[0], "y": ro[1], "z": ro[2], "angleDeg": 0, "lengthM": 0.05, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.6}},
        {"id": "radiator", "parent": "carrier", "title": T("The loose type 22 radiator with thermostat head and its two wall brackets taped on (lifted by hand, reinstalled on its brackets)", "Der lose Heizkörper Typ 22 mit Thermostatkopf und seinen zwei angeklebten Wandkonsolen (von Hand gehoben, an seinen Konsolen wieder eingebaut)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 1.0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.10, "maxM": 0.80},
         "mass": {"kg": r6(M_R1), "comX": r6(COM_R1[0]), "comY": r6(COM_R1[1]), "distribution": "uniform", "source": "authored-estimate", "reference": "radiator-catalogues"},
         "section": {"areaM2": 0.0042, "secondMomentM4": r6(2 * 1.25e-3 * 0.6 ** 3 / 12), "extremeFibreM": 0.3, "material": "painted-steel", "title": T("Two 1.25 mm steel panels 0.6 m high", "Zwei Stahlplatten 1,25 mm, 0,6 m hoch")}},
    ]}
    anchors = []
    for k, (x, z) in enumerate(((0.07, -0.35), (0.07, 0.35), (1.13, -0.35), (1.13, 0.35))):
        anchors.append({"id": f"pallet-ground-{k}", "bone": "pallet", "x": x, "y": r6(-po[1]), "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.145, "footprintWidthM": 0.10, "frictionCoefficient": 0.5,
                        "title": T(f"Pallet bottom board {'near' if z < 0 else 'far'} {'left' if x < 0.5 else 'right'} on the hall floor", f"Palettenbodenbrett {'vorn' if z < 0 else 'hinten'} {'links' if x < 0.5 else 'rechts'} auf dem Hallenboden")})
    for k, x in enumerate((0.10, 0.90)):
        anchors.append({"id": f"grip-{k}", "bone": "radiator", "x": x, "y": 0.30, "z": 0, "kind": "support", "support": "pin",
                        "title": T(f"{'Left' if k == 0 else 'Right'} hand grip on the top grille", f"{'Linker' if k == 0 else 'Rechter'} Handgriff am Abdeckgitter")})
    for k, (a, b) in enumerate(BRACKETS):
        anchors.append({"id": f"wall-bracket-{k}", "bone": "radiator", "x": r6((a + b) / 2 - ro[0]), "y": r6((BRACKET_Y[0] + BRACKET_Y[1]) / 2 - ro[1]), "z": r6(BRACKET_Z[0] - ro[2]), "kind": "support", "support": "pin",
                        "title": T(f"{'Left' if k == 0 else 'Right'} wall bracket screwed to the wall", f"{'Linke' if k == 0 else 'Rechte'} Wandkonsole an der Wand verschraubt")})
    anchors.append({"id": "grille-centre", "bone": "radiator", "x": 0.5, "y": 0.30, "z": 0, "kind": "emitter", "title": T("Middle of the top grille of the loose radiator", "Mitte des Abdeckgitters des losen Heizkörpers")})
    for k, x in enumerate(NIPPLES):
        anchors.append({"id": f"connection-{k}", "bone": "radiator", "x": r6(x - ro[0]), "y": r6(NIPPLE[0] - ro[1]), "z": 0, "kind": "load",
                        "title": T(f"{'Return' if k == 0 else 'Flow'} connection G 3/4 at the bottom", f"{'Rücklauf' if k == 0 else 'Vorlauf'}anschluss G 3/4 unten")})
    ground = [f"pallet-ground-{k}" for k in range(4)]
    walls = ["wall-bracket-0", "wall-bracket-1"]
    loads = [
        {"id": "radiator-water", "kind": "carried", "title": T("Heating water 5.8 l in the two panels", "Heizwasser 5,8 l in den zwei Platten"), "bone": "radiator", "x": 0.5, "y": 0, "fxN": 0, "fyN": r6(-WATER_KG * 9.81), "source": "authored-estimate", "reference": "radiator-catalogues"},
        {"id": "flow-pipe", "kind": "imposed", "title": T("Connection pipe and H-block on the flow tail, 30 N", "Anschlussrohr und Hahnblock am Vorlaufanschluss, 30 N"), "anchor": "connection-1", "fxN": 0, "fyN": -30, "source": "authored-estimate"},
        {"id": "return-pipe", "kind": "imposed", "title": T("Connection pipe on the return tail, 30 N", "Anschlussrohr am Rücklaufanschluss, 30 N"), "anchor": "connection-0", "fxN": 0, "fyN": -30, "source": "authored-estimate"},
    ]
    REIN = {"carrier": {"dx": 1.45}, "radiator": {"dy": -0.074}}
    poses = [
        {"id": "stored", "title": T("Stored on the pallet", "Auf der Palette gelagert"), "description": T(
            "Drained and capped, eight faded white type 22 radiators stand on edge on two battens in the tube rack of a Euro pallet; seven are strapped together, the eighth stands loose in front of them with its two wall brackets taped on and its thermostat head still fitted.",
            "Entleert und verschlossen stehen acht verblichene weiße Heizkörper Typ 22 hochkant auf zwei Kanthölzern im Rohrgestell einer Europalette; sieben sind zusammengegurtet, der achte steht lose davor, mit seinen zwei angeklebten Wandkonsolen und noch aufgesetztem Thermostatkopf."),
         "bones": {}, "ground": "authored", "contacts": ground, "thermal": {"operating": {"heating": 0}, "regions": binding("stored"), "emitters": {"convection-outlet": 0}}, "loads": []},
        {"id": "lifted", "title": T("Loose radiator lifted out by hand", "Loser Heizkörper von Hand herausgehoben"), "description": T(
            f"Two people lift the loose {M_R1:.1f} kg radiator by its top grille 0.70 m up out of the rack; the strapped seven, the battens, the rack and the pallet stay where they are.",
            f"Zwei Personen heben den losen, {M_R1:.1f} kg schweren Heizkörper am Abdeckgitter 0,70 m aus dem Gestell; die sieben verzurrten, die Kanthölzer, das Gestell und die Palette bleiben stehen."),
         "bones": {"radiator": {"dy": 0.70}}, "ground": "authored", "contacts": ground + ["grip-0", "grip-1"], "thermal": {"operating": {"heating": 0}, "regions": binding("lifted"), "emitters": {"convection-outlet": 0}}, "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled, heating at rated output", "Wieder eingebaut, Nennleistung"), "description": T(
            "The loose radiator hangs on its two brackets 0.15 m above the floor, 1.45 m to the right of the pallet, refilled and heating at 75/65/20 degC with 1600 W; warm air rises from its top grille.",
            "Der lose Heizkörper hängt an seinen zwei Konsolen 0,15 m über dem Boden, 1,45 m rechts der Palette, wieder befüllt und heizt mit 75/65/20 °C und 1600 W; warme Luft steigt aus seinem Abdeckgitter."),
         "bones": REIN, "ground": "authored", "contacts": ground + walls, "thermal": {"operating": {"heating": 1}, "regions": binding("reinstalled"), "emitters": {"convection-outlet": 1}}, "loads": loads},
        {"id": "reinstalled-off", "title": T("Reinstalled, filled and cold", "Wieder eingebaut, befüllt und kalt"), "description": T(
            "The same radiator on its brackets with the thermostat closed: filled with 5.8 l, at room temperature.",
            "Derselbe Heizkörper an seinen Konsolen mit geschlossenem Thermostat: mit 5,8 l befüllt, auf Raumtemperatur."),
         "bones": REIN, "ground": "authored", "contacts": ground + walls, "thermal": {"operating": {"heating": 0}, "regions": binding("reinstalled-off"), "emitters": {"convection-outlet": 0}}, "loads": loads},
    ]
    clips = [
        {"id": "lift", "title": T("Lift the loose radiator out", "Den losen Heizkörper herausheben"), "description": T(
            "Two people take the loose radiator by its top grille after two seconds and lift it 0.70 m up out of the rack in four seconds.",
            "Zwei Personen fassen den losen Heizkörper nach zwei Sekunden am Abdeckgitter und heben ihn in vier Sekunden 0,70 m aus dem Gestell."),
         "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in ground] + [{"anchor": "grip-0", "fromS": 2, "toS": 8}, {"anchor": "grip-1", "fromS": 2, "toS": 8}],
         "phases": [{"id": "grip", "fromS": 0, "toS": 2, "title": T("Taking hold", "Anfassen")}, {"id": "lift", "fromS": 2, "toS": 6, "title": T("Lifting", "Heben")}, {"id": "hold", "fromS": 6, "toS": 8, "title": T("Holding", "Halten")}],
         "events": [{"t": 2.1, "kind": "contact-off", "title": T("Radiator leaves the battens", "Heizkörper verlässt die Kanthölzer"), "target": "radiator"}],
         "tracks": []},
        {"id": "heat-up", "title": T("Heating up after reinstallation", "Aufheizen nach dem Wiedereinbau"), "description": T(
            "The refilled radiator starts cold; the thermostat opens after half a minute and the panels warm from the top flow connection downwards until they reach 75/65 degC after 15 minutes.",
            "Der wieder befüllte Heizkörper startet kalt; das Thermostat öffnet nach einer halben Minute, und die Platten erwärmen sich vom Vorlauf oben nach unten, bis sie nach 15 Minuten 75/65 °C erreichen."),
         "durationS": 1200, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 30, "pose": "reinstalled", "ease": "step"}, {"t": 1200, "pose": "reinstalled", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 1200} for a in ground + walls],
         "phases": [{"id": "cold", "fromS": 0, "toS": 30, "title": T("Cold", "Kalt")}, {"id": "warm-up", "fromS": 30, "toS": 930, "title": T("Warming up", "Aufheizen")}, {"id": "rated", "fromS": 930, "toS": 1200, "title": T("Rated output", "Nennleistung")}],
         "events": [{"t": 30, "kind": "operating", "title": T("Thermostat opens", "Thermostat öffnet"), "target": "heating"}, {"t": 930, "kind": "operating", "title": T("Panels reach 75/65 degC", "Platten erreichen 75/65 °C"), "target": "heating"}],
         "tracks": [{"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 930, "v": 1}, {"t": 1200, "v": 1}]},
                    {"target": "emitter:convection-outlet", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 930, "v": 1}, {"t": 1200, "v": 1}]}]},
    ]
    unavailable = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T(
        "At 1:500 the loaded pallet is 2.4 mm long, 1.6 mm wide and 1.6 mm high on paper, under the 3 mm a recognisable drawing needs, and the 100 mm radiators, the 22 mm deck boards and the 30 mm rack tubes shrink to 0.2, 0.04 and 0.06 mm, below the printable feature; draw it at 1:200 or larger.",
        "Bei 1:500 ist die beladene Palette auf Papier 2,4 mm lang, 1,6 mm breit und 1,6 mm hoch, unter den 3 mm, die eine erkennbare Zeichnung braucht, und die 100 mm tiefen Heizkörper, die 22 mm Deckbretter und die 30 mm Gestellrohre schrumpfen auf 0,2, 0,04 und 0,06 mm, unter das druckbare Merkmal; bei 1:200 oder größer zeichnen."),
        "limitingFeature": T("Radiator 100 mm deep", "Heizkörper 100 mm tief"), "limitingSizeM": 0.10, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    asset = {
        "id": SLUG, "category": "components", "title": T("Used Panel Radiator Pallet", "Palette gebrauchter Plattenheizkörper"), "summary": SUMMARY,
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "radiator", "panel-radiator", "type-22", "steel", "pallet", "timber", "stacked", "operating", "thermal"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": 0.8255, "widthM": 1.2, "depthM": 0.838, "massKg": r6(M_TOTAL)},
        "materials": MATERIALS, "rig": rig,
        "volumes": {"50": volume(50), "100": volume(100), "200": volume(200), "500": {"status": "unavailable", "denominator": 500}},
        "anchors": anchors,
        "representations": {"50": representation(50), "100": representation(100), "200": representation(200), "500": unavailable},
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
    print(f"[DEBUG] pallet {M_PALLET:.2f} battens {M_BATTENS:.2f} rack {M_RACK:.2f} pool {M_POOL:.2f}; stack {M_STACK:.2f}; radiator {M_R1:.2f}; total {M_TOTAL:.2f} kg; com pool {COM_POOL}; com r1 {COM_R1}")
