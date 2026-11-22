"""🌀 Authoritative generator of the used dry-rotor inline pump: every hand-chosen number of the DN 80 inline circulator with its 3 kW IE3 motor, its two blanking plates and its half pallet, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-dry-rotor-inline-pump"


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


DECK = 0.144
PX = 0.40
AX = DECK + 0.16
ORIGIN = {"floor": (0.0, 0.0), "pallet": (0.0, 0.0), "pump": (0.0, DECK), "carrier": (0.0, 0.0), "blanks": (0.0, DECK)}

PALLET_L, PALLET_W = 0.80, 0.60
BOARD_T, BLOCK_H, BOARD_W = 0.022, 0.078, 0.10
ROWS_Z = (-0.25, 0.0, 0.25)
COLS_X = (0.05, 0.40, 0.75)
TOP_Z = (-0.25, -0.125, 0.0, 0.125, 0.25)
CHOCKS = (((0.22, DECK, -0.10), (0.29, DECK + 0.06, 0.10)), ((0.51, DECK, -0.10), (0.58, DECK + 0.06, 0.10)))

FOOT = ((PX - 0.11, DECK, -0.09), (PX + 0.11, DECK + 0.025, 0.09))
PEDESTAL = ((PX - 0.07, DECK + 0.025, -0.06), (PX + 0.07, 0.214, 0.06))
VOLUTE = (0.214, 0.374, 0.135)
NECK_R, PORT_HALF, FLANGE_T, FLANGE_R = 0.05, 0.18, 0.022, 0.10
BOLT_PCD_R, NUT_R, NUT_T = 0.08, 0.012, 0.013
COVER = (0.374, 0.392, 0.142)
LANTERN = (0.392, 0.52, 0.085)
LUG = (0.105, 0.48, 0.025, 0.012, 0.014)
MFLANGE = (0.52, 0.536, 0.125)
MOTOR = (0.536, 0.80, 0.10)
COWL = (0.80, 0.92, 0.104)
TBOX = ((PX - 0.06, 0.60, -0.165), (PX + 0.06, 0.72, -0.085))
GLAND = (PX, -0.125, 0.575, 0.60, 0.012)
CABLE = (PX, -0.125, 0.545, 0.575, 0.008)
PLUG = (0.147, 0.350, 0.368, 0.009)
BLANK_T, HEAD_R, HEAD_T = 0.012, 0.013, 0.010
BOLTS_YZ = [(r6(AX + BOLT_PCD_R * math.sin(a)), r6(BOLT_PCD_R * math.cos(a))) for a in (math.pi / 4, 3 * math.pi / 4, 5 * math.pi / 4, 7 * math.pi / 4)]

WOOD = 470.0
V_PALLET = 3 * PALLET_L * BOARD_W * BOARD_T + 9 * BOARD_W * BLOCK_H * BOARD_W + 3 * PALLET_W * BOARD_W * BOARD_T + 5 * PALLET_L * BOARD_W * BOARD_T
V_CHOCKS = 2 * 0.07 * 0.06 * 0.20
M_PALLET = (V_PALLET + V_CHOCKS) * WOOD + 0.3
M_BLANKS = 2 * math.pi * FLANGE_R ** 2 * BLANK_T * 7850.0 + 8 * 0.12
PUMP_MASSES = {
    "housing": (38.0, (PX, 0.29)),
    "internals": (4.0, (PX, 0.40)),
    "cover-lantern": (12.0, (PX, 0.43)),
    "motor": (26.0, (PX, 0.71)),
    "nuts-lugs-plugs": (0.24 + 0.4 + 0.1, (PX, 0.39)),
    "cable": (0.1, (PX, 0.56)),
}
M_PUMP = sum(m for m, _ in PUMP_MASSES.values())
COM_PUMP = (sum(m * c[0] for m, c in PUMP_MASSES.values()) / M_PUMP, sum(m * c[1] for m, c in PUMP_MASSES.values()) / M_PUMP)
M_TOTAL = M_PALLET + M_BLANKS + M_PUMP
WATER_KG = 5.0

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


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
        elif self.kind == "ring":
            cx, cy, cz, ro, ri, t = g
            n = 20
            outer = [[r6(cx - ox + ro * math.cos(2 * math.pi * k / n)), r6(cy - oy + ro * math.sin(2 * math.pi * k / n))] for k in range(n)]
            s = dict(base, kind="extrude", plane="xy", profile=outer, **{"from": r6(cz - t / 2), "to": r6(cz + t / 2)})
            if ri:
                s["holes"] = [[[r6(cx - ox + ri * math.cos(2 * math.pi * k / n)), r6(cy - oy + ri * math.sin(2 * math.pi * k / n))] for k in range(n)][::-1]]
        for key in ("slender", "sheet", "texture", "repeat", "tags"):
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
        cx, cy, cz, ro, ri, t = g
        return (cx - ro, cy - ro, cz - t / 2), (cx + ro, cy + ro, cz + t / 2)

    def shape(self, view):
        """✏ The filled 2D shape of the part in one viewpoint, in that viewpoint's frame."""
        ox, oy = ORIGIN[self.bone]
        (x0, y0, z0), (x1, y1, z1) = self.bounds()
        depth = {"elevation": 5 - z0, "front": 5 + x1, "plan": 5 + y1}[view]
        s = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(depth, 4), "filled": True, "tone": self.tone, "material": self.material, "region": self.region}
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
            return rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)
        if view == "front":
            if self.kind == "cylx":
                return circle(g[3], g[2], g[4])
            return rect(z0, y0, z1 - z0, y1 - y0)
        if self.kind == "cyly":
            cx, cz, _, _, r = g
            return circle(cx, cz, r)
        return rect(x0, z0, x1 - x0, z1 - z0)


def ring_path(cx, cy, r, ri):
    """⭕ An SVG annulus: outer circle and the bore as a hole."""
    return (f"M {r6(cx - r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx + r)} {r6(cy)} A {r} {r} 0 1 0 {r6(cx - r)} {r6(cy)} Z "
            f"M {r6(cx - ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx + ri)} {r6(cy)} A {ri} {ri} 0 1 1 {r6(cx - ri)} {r6(cy)} Z")


def pallet_parts(scale):
    """🪵 The half pallet 800 x 600 x 144 mm and its two chocks: boards, blocks and stringers at 1:50 and 1:100, deck and runners at 1:200."""
    P = []
    if scale == 200:
        for k, zc in enumerate(ROWS_Z):
            P.append(Part(f"runner-{k}", "pallet", "box", ((0.0, 0.0, zc - BOARD_W / 2), (PALLET_L, BOARD_T + BLOCK_H, zc + BOARD_W / 2)), "softwood", 0.55, "pallet"))
        P.append(Part("deck", "pallet", "box", ((0.0, BOARD_T + BLOCK_H, -PALLET_W / 2), (PALLET_L, DECK, PALLET_W / 2)), "softwood", 0.62, "pallet"))
    else:
        for k, zc in enumerate(ROWS_Z):
            P.append(Part(f"bottom-board-{k}", "pallet", "box", ((0.0, 0.0, zc - BOARD_W / 2), (PALLET_L, BOARD_T, zc + BOARD_W / 2)), "softwood", 0.6, "pallet"))
        for i, xc in enumerate(COLS_X):
            for k, zc in enumerate(ROWS_Z):
                P.append(Part(f"block-{i}-{k}", "pallet", "box", ((xc - BOARD_W / 2, BOARD_T, zc - BOARD_W / 2), (xc + BOARD_W / 2, BOARD_T + BLOCK_H, zc + BOARD_W / 2)), "softwood", 0.5, "pallet"))
            P.append(Part(f"stringer-{i}", "pallet", "box", ((xc - BOARD_W / 2, BOARD_T + BLOCK_H, -PALLET_W / 2), (xc + BOARD_W / 2, DECK - BOARD_T, PALLET_W / 2)), "softwood", 0.58, "pallet"))
        for k, zc in enumerate(TOP_Z):
            P.append(Part(f"top-board-{k}", "pallet", "box", ((0.0, DECK - BOARD_T, zc - BOARD_W / 2), (PALLET_L, DECK, zc + BOARD_W / 2)), "softwood", 0.64, "pallet",
                          extra={"radius": 0.003} if scale == 50 else {}))
    for k, (lo, hi) in enumerate(CHOCKS):
        P.append(Part(f"chock-{k}", "pallet", "box", (lo, hi), "softwood", 0.68, "pallet"))
    return P


def pump_parts(scale):
    """🌀 The pump itself: cast housing with foot, ports and flanges, cover, lantern with lugs, motor with flange, terminal box and fan cowl."""
    P = []
    if scale == 200:
        P.append(Part("foot", "pump", "box", ((FOOT[0][0], DECK, FOOT[0][2]), (FOOT[1][0], VOLUTE[0], FOOT[1][2])), "cast-iron", 0.3, "casing"))
        P.append(Part("volute", "pump", "cyly", (PX, 0.0, VOLUTE[0], COVER[1], 0.138), "cast-iron", 0.32, "casing"))
        P.append(Part("necks", "pump", "cylx", (PX - PORT_HALF + 0.03, PX + PORT_HALF - 0.03, AX, 0.0, NECK_R), "cast-iron", 0.32, "casing"))
        P.append(Part("flange-suction", "pump", "cylx", (PX - PORT_HALF, PX - PORT_HALF + 0.03, AX, 0.0, FLANGE_R), "cast-iron", 0.28, "casing"))
        P.append(Part("flange-discharge", "pump", "cylx", (PX + PORT_HALF - 0.03, PX + PORT_HALF, AX, 0.0, FLANGE_R), "cast-iron", 0.28, "casing"))
        P.append(Part("lantern", "pump", "cyly", (PX, 0.0, LANTERN[0], 0.506, LANTERN[2]), "cast-iron", 0.36, "lantern"))
        P.append(Part("motor-flange", "pump", "cyly", (PX, 0.0, 0.506, MFLANGE[1], MFLANGE[2]), "painted-aluminium", 0.46, "motor"))
    else:
        P.append(Part("foot", "pump", "box", FOOT, "cast-iron", 0.3, "casing", extra={"radius": 0.004} if scale == 50 else {}))
        P.append(Part("pedestal", "pump", "box", PEDESTAL, "cast-iron", 0.3, "casing"))
        P.append(Part("volute", "pump", "cyly", (PX, 0.0, VOLUTE[0], VOLUTE[1], VOLUTE[2]), "cast-iron", 0.32, "casing"))
        P.append(Part("necks", "pump", "cylx", (PX - PORT_HALF + FLANGE_T, PX + PORT_HALF - FLANGE_T, AX, 0.0, NECK_R), "cast-iron", 0.32, "casing"))
        P.append(Part("flange-suction", "pump", "cylx", (PX - PORT_HALF, PX - PORT_HALF + FLANGE_T, AX, 0.0, FLANGE_R), "cast-iron", 0.28, "casing"))
        P.append(Part("flange-discharge", "pump", "cylx", (PX + PORT_HALF - FLANGE_T, PX + PORT_HALF, AX, 0.0, FLANGE_R), "cast-iron", 0.28, "casing"))
        P.append(Part("cover", "pump", "cyly", (PX, 0.0, COVER[0], COVER[1], COVER[2]), "cast-iron", 0.34, "lantern"))
        P.append(Part("lantern", "pump", "cyly", (PX, 0.0, LANTERN[0], LANTERN[1], LANTERN[2]), "cast-iron", 0.36, "lantern"))
        P.append(Part("motor-flange", "pump", "cyly", (PX, 0.0, MFLANGE[0], MFLANGE[1], MFLANGE[2]), "painted-aluminium", 0.46, "motor"))
        lx, ly, lro, lri, lt = LUG
        for k, sx in enumerate((-1, 1)):
            if scale == 50:
                P.append(Part(f"lug-{k}", "pump", "ring", (PX + sx * lx, ly, 0.0, lro, lri, lt), "cast-iron", 0.3, "lantern"))
            else:
                P.append(Part(f"lug-{k}", "pump", "ring", (PX + sx * lx, ly, 0.0, lro, None, 0.016), "cast-iron", 0.3, "lantern"))
    if scale == 50:
        for side, x0, x1 in (("suction", PX - PORT_HALF + FLANGE_T, PX - PORT_HALF + FLANGE_T + NUT_T), ("discharge", PX + PORT_HALF - FLANGE_T - NUT_T, PX + PORT_HALF - FLANGE_T)):
            for k, (y, z) in enumerate(BOLTS_YZ):
                P.append(Part(f"nut-{side}-{k}", "pump", "cylx", (x0, x1, y, z, NUT_R), "steel", 0.16, "casing"))
        px, py0, py1, pr = PLUG
        for k, sx in enumerate((-1, 1)):
            P.append(Part(f"gauge-plug-{k}", "pump", "cyly", (PX + sx * px, 0.0, py0, py1, pr), "brass", 0.44, "casing"))
    P.append(Part("motor", "pump", "cyly", (PX, 0.0, MOTOR[0], MOTOR[1], MOTOR[2]), "painted-aluminium", 0.5, "motor"))
    P.append(Part("terminal-box", "pump", "box", TBOX, "painted-aluminium", 0.46, "terminal-box", extra={"radius": 0.004} if scale == 50 else {}))
    if scale == 50:
        P.append(Part("cable-gland", "pump", "cyly", GLAND, "polyamide", 0.15, "terminal-box"))
        P.append(Part("cable-stub", "pump", "cyly", CABLE, "pvc", 0.1, "terminal-box"))
    P.append(Part("fan-cowl", "pump", "cyly", (PX, 0.0, COWL[0], COWL[1], COWL[2]), "steel-sheet", 0.42, "fan-cowl"))
    return P


def blank_parts(scale):
    """🛑 The two blanking plates on the port flanges with their M16 bolt heads (merged into the plate at 1:100, a 30 mm disc at 1:200)."""
    P = []
    if scale == 200:
        spans = (("suction", PX - PORT_HALF - 0.03, PX - PORT_HALF), ("discharge", PX + PORT_HALF, PX + PORT_HALF + 0.03))
    elif scale == 100:
        spans = (("suction", PX - PORT_HALF - BLANK_T - HEAD_T, PX - PORT_HALF), ("discharge", PX + PORT_HALF, PX + PORT_HALF + BLANK_T + HEAD_T))
    else:
        spans = (("suction", PX - PORT_HALF - BLANK_T, PX - PORT_HALF), ("discharge", PX + PORT_HALF, PX + PORT_HALF + BLANK_T))
    for side, x0, x1 in spans:
        P.append(Part(f"blank-{side}", "blanks", "cylx", (x0, x1, AX, 0.0, FLANGE_R), "steel", 0.2, "blanking-plates"))
    if scale == 50:
        for side, x0, x1 in (("suction", PX - PORT_HALF - BLANK_T - HEAD_T, PX - PORT_HALF - BLANK_T), ("discharge", PX + PORT_HALF + BLANK_T, PX + PORT_HALF + BLANK_T + HEAD_T)):
            for k, (y, z) in enumerate(BOLTS_YZ):
                P.append(Part(f"bolt-{side}-{k}", "blanks", "cylx", (x0, x1, y, z, HEAD_R), "steel", 0.12, "blanking-plates"))
    return P


def parts_of(scale):
    """🗂 The hand-chosen part list of one scale: every part the real pump has that this scale can show, generalised for its smallest feature."""
    return pallet_parts(scale) + pump_parts(scale) + blank_parts(scale)


def details_of(scale):
    """🖋 Detail and texture strokes of one scale: cooling fins, lantern windows with the coupling, cowl grille and dent, type plate, casting seam, flow arrow, rust and wood grain."""
    e, f, p = [], [], []
    if scale == 200:
        return e, f, p
    oy = DECK

    def line(lst, sid, bone, pts, role="detail", region=None, tone=0.25, z=9.0, closed=False, filled=False):
        s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
        if region:
            s["region"] = region
        lst.append(s)

    fins = (-0.075, -0.05, -0.025, 0.0, 0.025, 0.05, 0.075) if scale == 50 else (-0.05, 0.0, 0.05)
    for k, dx in enumerate(fins):
        line(e, f"fin-{k}", "pump", [(PX + dx, MOTOR[0] + 0.012 - oy), (PX + dx, MOTOR[1] - 0.006 - oy)], role="texture" if scale == 50 else "detail", region="motor", z=7.9)
        line(f, f"fin-front-{k}", "pump", [(dx, MOTOR[0] + 0.012), (dx, MOTOR[1] - 0.006)], role="texture" if scale == 50 else "detail", region="motor", z=6.0)
    win = [(PX - 0.04, 0.405 - oy), (PX + 0.04, 0.405 - oy), (PX + 0.04, 0.505 - oy), (PX - 0.04, 0.505 - oy)]
    line(e, "lantern-window", "pump", win, region="lantern", tone=0.08, closed=True, filled=True, z=7.95)
    line(f, "lantern-window-front", "pump", [(-0.04, 0.405), (0.04, 0.405), (0.04, 0.505), (-0.04, 0.505)], region="lantern", tone=0.08, closed=True, filled=True, z=5.9)
    line(e, "cowl-rim", "pump", [(PX - COWL[2], COWL[0] + 0.012 - oy), (PX + COWL[2], COWL[0] + 0.012 - oy)], region="fan-cowl", z=7.9)
    line(p, "cowl-grille-outer", "pump", [(PX + 0.085 * math.cos(a), 0.085 * math.sin(a)) for a in [2 * math.pi * k / 24 for k in range(24)]], region="fan-cowl", closed=True, z=6.0)
    if scale == 50:
        line(e, "coupling", "pump", [(PX - 0.022, 0.44 - oy), (PX + 0.022, 0.44 - oy), (PX + 0.022, 0.475 - oy), (PX - 0.022, 0.475 - oy)], region="lantern", tone=0.4, closed=True, filled=True, z=7.96)
        line(f, "coupling-front", "pump", [(-0.022, 0.44), (0.022, 0.44), (0.022, 0.475), (-0.022, 0.475)], region="lantern", tone=0.4, closed=True, filled=True, z=5.91)
        line(e, "shaft", "pump", [(PX, 0.405 - oy), (PX, 0.44 - oy)], region="lantern", tone=0.5, z=7.97)
        line(e, "type-plate", "pump", [(PX - 0.04, 0.61 - oy), (PX + 0.04, 0.61 - oy), (PX + 0.04, 0.64 - oy), (PX - 0.04, 0.64 - oy)], region="terminal-box", tone=0.35, closed=True, filled=True, z=8.6)
        line(e, "type-plate-text", "pump", [(PX - 0.03, 0.625 - oy), (PX + 0.03, 0.625 - oy)], role="texture", region="terminal-box", z=8.61)
        line(e, "terminal-lid-seam", "pump", [(PX - 0.06, 0.705 - oy), (PX + 0.06, 0.705 - oy)], region="terminal-box", z=8.6)
        line(e, "cowl-dent", "pump", [(PX + 0.05, 0.86 - oy), (PX + 0.065, 0.872 - oy), (PX + 0.08, 0.866 - oy)], role="texture", region="fan-cowl", tone=0.5, z=7.91)
        line(e, "volute-seam", "pump", [(PX - VOLUTE[2], 0.30 - oy), (PX + VOLUTE[2], 0.30 - oy)], role="texture", region="casing", z=7.9)
        line(e, "flow-arrow", "pump", [(PX - 0.05, 0.25 - oy), (PX + 0.04, 0.25 - oy), (PX + 0.025, 0.26 - oy), (PX + 0.04, 0.25 - oy), (PX + 0.025, 0.24 - oy)], region="casing", tone=0.5, z=7.91)
        for k, x in enumerate((PX - PORT_HALF + 0.006, PX + PORT_HALF - 0.008)):
            line(e, f"flange-rust-{k}", "pump", [(x, AX - 0.08 - oy), (x + 0.002, AX - 0.03 - oy)], role="texture", region="casing", tone=0.5, z=7.9)
        for k, y in enumerate((0.007, 0.015)):
            line(e, f"board-grain-{k}", "pallet", [(0.0, y), (0.37, y + 0.001), (PALLET_L, y - 0.001)], role="texture", region="pallet", z=6.5)
        line(e, "top-board-grain", "pallet", [(0.0, DECK - 0.011), (0.5, DECK - 0.010), (PALLET_L, DECK - 0.012)], role="texture", region="pallet", z=6.5)
        line(p, "cowl-grille-mid", "pump", [(PX + 0.055 * math.cos(a), 0.055 * math.sin(a)) for a in [2 * math.pi * k / 20 for k in range(20)]], role="texture", region="fan-cowl", closed=True, z=6.01)
        line(p, "cowl-grille-inner", "pump", [(PX + 0.025 * math.cos(a), 0.025 * math.sin(a)) for a in [2 * math.pi * k / 16 for k in range(16)]], role="texture", region="fan-cowl", closed=True, z=6.02)
        for k, zc in enumerate(TOP_Z):
            line(p, f"nails-{k}", "pallet", [(0.05, zc), (0.40, zc), (0.75, zc)], role="texture", region="pallet", z=5.2)
    return e, f, p


exec(open(os.path.join(HERE, "author-pump-texts.py"), encoding="utf8").read())


def representation(scale):
    """🖨 The authored representation of one scale: the projected part shapes, the detail strokes, pens, hatches and decisions."""
    geo = {"elevation": [], "front": [], "plan": []}
    for part in parts_of(scale):
        for view in part.views:
            sh = part.shape(view)
            if scale == 50 and view == "front" and (part.id.startswith("top-board") or part.id.startswith("bottom-board")):
                sh["hatch"] = "end-grain"
            geo[view].append(sh)
    for view, shapes in geo.items():
        for k, sh in enumerate(shapes):
            sh["z"] = round(sh["z"] + 0.0001 * k, 4)
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    hatches = [{"id": "end-grain", "title": T("Sawn end grain of the pallet boards", "Gesägtes Hirnholz der Palettenbretter"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}] if scale == 50 else []
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": DECISIONS_2D[scale]}


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: every body part by reference to its region and the fan intake disc on the cowl grille."""
    out = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in parts]
    d = max(FEATURE[scale], 0.008)
    out.append({"id": "th-fan-intake", "kind": "revolve", "bone": "pump", "material": "steel-sheet", "role": "detail", "tone": 0.3, "profile": [[0, 0], [0, 0.09], [d, 0.09], [d, 0]],
                "axis": [0, 1, 0], "origin": [PX, r6(COWL[1] - DECK), 0], "region": "fan-cowl", "tags": ["motor-fan"]})
    return out


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: pallet deck and runners, the pump housing, lantern and motor as one member chain, the blanking plates, the foot bearings as joints and the centres of mass."""
    names = {p.id for p in parts}
    out = [
        {"id": "member-pallet-deck", "kind": "box", "bone": "pallet", "material": "softwood", "role": "body", "tone": 0.6, "size": [PALLET_L, r6(DECK - BOARD_T - BLOCK_H), PALLET_W],
         "transform": {"translation": [PALLET_L / 2, r6((DECK + BOARD_T + BLOCK_H) / 2), 0]}, "member": "pallet", "part": "member"},
    ]
    for k, zc in enumerate(ROWS_Z):
        out.append({"id": f"member-pallet-runner-{k}", "kind": "box", "bone": "pallet", "material": "softwood", "role": "body", "tone": 0.6, "size": [PALLET_L, BOARD_T + BLOCK_H, BOARD_W],
                    "transform": {"translation": [PALLET_L / 2, r6((BOARD_T + BLOCK_H) / 2), zc]}, "member": "pallet", "part": "member"})
    for pid in ("foot", "pedestal", "volute", "necks", "flange-suction", "flange-discharge", "cover", "lantern", "motor-flange", "motor"):
        if pid in names:
            out.append({"id": f"member-{pid}", "of": pid, "member": "pump", "part": "member"})
    for pid in ("blank-suction", "blank-discharge"):
        out.append({"id": f"member-{pid}", "of": pid, "member": "blanks", "part": "member"})
    pad = max(FEATURE[scale], 0.008)
    for k, (x, z) in enumerate(((PX - 0.09, -0.07), (PX - 0.09, 0.07), (PX + 0.09, -0.07), (PX + 0.09, 0.07))):
        out.append({"id": f"joint-foot-{k}", "kind": "box", "bone": "pump", "material": "cast-iron", "role": "detail", "tone": 0.5, "size": [max(0.04, 2 * pad), pad, max(0.04, 2 * pad)],
                    "transform": {"translation": [r6(x), 0.0, z]}, "member": "pump", "part": "joint"})
    mr = max(0.03, FEATURE[scale])
    out.append({"id": "mass-pump", "kind": "ellipsoid", "bone": "pump", "material": "cast-iron", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [r6(COM_PUMP[0]), r6(COM_PUMP[1] - DECK), 0]}, "member": "pump", "part": "mass"})
    out.append({"id": "mass-blanks", "kind": "ellipsoid", "bone": "blanks", "material": "steel", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [PX, r6(AX - DECK), 0]}, "member": "blanks", "part": "mass"})
    out.append({"id": "mass-pallet", "kind": "ellipsoid", "bone": "pallet", "material": "softwood", "role": "detail", "tone": 0.5, "radii": [mr, mr, mr],
                "transform": {"translation": [PALLET_L / 2, 0.08, 0]}, "member": "pallet", "part": "mass"})
    return out


def volume(scale):
    """🧊 The authored volume of one scale: body from the part list, thermal and structure lists, and the decisions."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts if p.body]
    if scale == 50:
        body.append({"id": "type-plate", "kind": "box", "bone": "pump", "material": "aluminium", "role": "detail", "tone": 0.35, "size": [0.08, 0.03, 0.0015],
                     "transform": {"translation": [PX, r6(0.625 - DECK), r6(TBOX[0][2] - 0.00075)]}, "texture": True, "sheet": True})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


def binding(regions, on_floor=False):
    """🌬 The region binding of a pose: everything indoors, nothing sunlit, the pallet on its bottom boards and the pump foot on its bearing face."""
    out = {}
    for r in regions:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "pallet":
            b = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 15}}
        if r == "casing" and on_floor:
            b = {"exposedFraction": 0.92, "sunlitFraction": 0, "contact": {"fraction": 0.08, "tempC": 15}}
        out[r] = b
    return out


def document():
    """📄 The whole asset document."""
    regions = [r["id"] for r in THERMAL["regions"]]
    rig = {"kind": "mechanical", "root": "floor", "bones": [
        {"id": "floor", "parent": None, "title": T("Hall floor frame (massless, fixed root)", "Hallenboden-Bezug (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}},
        {"id": "pallet", "parent": "floor", "title": T("Half pallet 800 x 600 x 144 mm with two chocks (stays on the floor when the pump is lifted)", "Halbpalette 800 x 600 x 144 mm mit zwei Keilhölzern (bleibt beim Anheben der Pumpe am Boden)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": PALLET_L, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.4},
         "mass": {"kg": r6(M_PALLET), "comX": PALLET_L / 2, "comY": 0.08, "distribution": "uniform", "source": "literature", "reference": "epal-half-pallet"},
         "section": {"areaM2": r6(PALLET_W * (DECK - BOARD_T - BLOCK_H)), "secondMomentM4": r6(PALLET_W * (DECK - BOARD_T - BLOCK_H) ** 3 / 12), "extremeFibreM": r6((DECK - BOARD_T - BLOCK_H) / 2), "material": "softwood",
                     "title": T("Pallet deck of stringers and top boards, 44 mm", "Palettendeck aus Querbrettern und Deckbrettern, 44 mm")}},
        {"id": "pump", "parent": "floor", "title": T("Inline pump with cast housing, lantern, IE3 motor and fan cowl (lifted by its two lantern lugs)", "Inlinepumpe mit Gussgehäuse, Laterne, IE3-Motor und Lüfterhaube (an ihren zwei Laternenösen gehoben)"),
         "x": 0, "y": DECK, "angleDeg": 0, "lengthM": 0.60, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.20, "maxM": 0.60},
         "mass": {"kg": r6(M_PUMP), "comX": r6(COM_PUMP[0]), "comY": r6(COM_PUMP[1] - DECK), "distribution": "point", "source": "authored-estimate", "reference": "inline-pump-catalogues"}},
        {"id": "carrier", "parent": "floor", "title": T("Massless carrier of the blanking plates (moves them to the pallet when the pump is reinstalled)", "Masseloser Träger der Blindscheiben (bringt sie beim Wiedereinbau auf die Palette)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.4}},
        {"id": "blanks", "parent": "carrier", "title": T("Two steel blanking plates 200 x 12 mm with four M16 bolts each", "Zwei Stahl-Blindscheiben 200 x 12 mm mit je vier Schrauben M16"),
         "x": 0, "y": DECK, "angleDeg": 0, "lengthM": 0.60, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.10, "maxM": 0.60},
         "mass": {"kg": r6(M_BLANKS), "comX": PX, "comY": r6(AX - DECK), "distribution": "point", "source": "literature", "reference": "en-1092-1"}},
    ]}
    anchors = []
    for k, (x, z) in enumerate(((0.05, -0.25), (0.05, 0.25), (0.75, -0.25), (0.75, 0.25))):
        anchors.append({"id": f"pallet-ground-{k}", "bone": "pallet", "x": x, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.10, "footprintWidthM": 0.10, "frictionCoefficient": 0.5,
                        "title": T(f"Bottom board {'right' if z < 0 else 'left'} under the {'rear' if x < 0.4 else 'front'} block on the hall floor", f"Bodenbrett {'rechts' if z < 0 else 'links'} unter dem {'hinteren' if x < 0.4 else 'vorderen'} Klotz auf dem Hallenboden")})
    for k, (x, z) in enumerate(((PX - 0.09, -0.07), (PX - 0.09, 0.07), (PX + 0.09, -0.07), (PX + 0.09, 0.07))):
        anchors.append({"id": f"pump-foot-{k}", "bone": "pump", "x": r6(x), "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.4,
                        "title": T(f"Pump foot, {'right' if z < 0 else 'left'} {'rear' if x < PX else 'front'} corner", f"Pumpenfuß, Ecke {'rechts' if z < 0 else 'links'} {'hinten' if x < PX else 'vorn'}")})
    for k, sx in enumerate((-1, 1)):
        anchors.append({"id": f"lug-{k}", "bone": "pump", "x": r6(PX + sx * LUG[0]), "y": r6(LUG[1] + LUG[3] - DECK), "z": 0, "kind": "support", "support": "pin",
                        "title": T(f"{'Rear' if k == 0 else 'Front'} lifting lug on the lantern: sling shackle", f"{'Hintere' if k == 0 else 'Vordere'} Hebeöse an der Laterne: Schäkel des Gehänges")})
    anchors.append({"id": "fan-intake", "bone": "pump", "x": PX, "y": r6(COWL[1] - DECK), "z": 0, "kind": "emitter", "title": T("Grille of the fan cowl, where the motor fan draws its cooling air", "Gitter der Lüfterhaube, durch das der Motorlüfter die Kühlluft ansaugt")})
    anchors.append({"id": "suction-flange", "bone": "pump", "x": r6(PX - PORT_HALF), "y": r6(AX - DECK), "z": 0, "kind": "load", "title": T("Suction flange DN 80, where the return pipe of the riser is bolted", "Saugflansch DN 80, an dem das Rohr des Steigstrangs angeschraubt wird")})
    anchors.append({"id": "discharge-flange", "bone": "pump", "x": r6(PX + PORT_HALF), "y": r6(AX - DECK), "z": 0, "kind": "load", "title": T("Discharge flange DN 80, where the riser continues", "Druckflansch DN 80, an dem der Steigstrang weitergeht")})
    feet = [f"pump-foot-{k}" for k in range(4)]
    ground = [f"pallet-ground-{k}" for k in range(4)]
    service_loads = [
        {"id": "pump-water", "kind": "carried", "title": T("Heating water 5 l in the housing", "Heizwasser 5 l im Gehäuse"), "bone": "pump", "x": PX, "y": r6(AX - DECK), "z": 0, "fxN": 0, "fyN": r6(-WATER_KG * 9.81), "source": "authored-estimate", "reference": "inline-pump-catalogues"},
        {"id": "suction-pipe", "kind": "imposed", "title": T("Pipe and water weight of the riser on the suction flange, 0.25 kN", "Rohr- und Wassergewicht des Strangs am Saugflansch, 0,25 kN"), "anchor": "suction-flange", "fxN": 0, "fyN": -250, "source": "authored-estimate", "reference": "en-iso-5199"},
        {"id": "discharge-pipe", "kind": "imposed", "title": T("Pipe and water weight of the riser on the discharge flange, 0.25 kN", "Rohr- und Wassergewicht des Strangs am Druckflansch, 0,25 kN"), "anchor": "discharge-flange", "fxN": 0, "fyN": -250, "source": "authored-estimate", "reference": "en-iso-5199"},
    ]
    cold = {"pump": 0, "heating": 0}
    reinstalled_bones = {"pump": {"dy": -DECK}, "pallet": {"dx": 1.2}, "carrier": {"dx": 1.2}, "blanks": {"dy": r6(-(AX - FLANGE_R - DECK))}}
    poses = [
        {"id": "stored", "title": T("Stored on its pallet", "Auf der Palette gelagert"), "description": T(
            "Drained, the pump stands upright with its cast foot between two chocks on a half pallet in the hall of the component exchange; both port flanges are closed by bolted steel blanking plates, the motor cable is cut at the gland.",
            "Entleert steht die Pumpe aufrecht mit ihrem gegossenen Fuß zwischen zwei Keilhölzern auf einer Halbpalette in der Halle der Bauteilbörse; beide Stutzenflansche sind mit angeschraubten Stahl-Blindscheiben verschlossen, das Motorkabel ist an der Verschraubung abgeschnitten."),
         "bones": {}, "ground": "authored", "contacts": ground, "thermal": {"operating": cold, "regions": binding(regions), "emitters": {"motor-fan": 0}}, "loads": []},
        {"id": "lifted", "title": T("Lifted on a two-leg sling", "Am Zweistranggehänge angehoben"), "description": T(
            f"A two-leg sling with shackles in the two lugs of the lantern lifts the {M_PUMP + M_BLANKS:.0f} kg pump with its blanking plates 0.40 m above the pallet; the pallet and its chocks stay on the floor.",
            f"Ein Zweistranggehänge mit Schäkeln in den zwei Ösen der Laterne hebt die {M_PUMP + M_BLANKS:.0f} kg schwere Pumpe mit ihren Blindscheiben 0,40 m über die Palette; Palette und Keilhölzer bleiben am Boden."),
         "bones": {"pump": {"dy": 0.40}, "blanks": {"dy": 0.40}}, "ground": "authored", "contacts": ["lug-0", "lug-1"], "thermal": {"operating": cold, "regions": binding(regions), "emitters": {"motor-fan": 0}}, "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled in the riser, running at design", "Im Steigstrang wieder eingebaut, Nennbetrieb"), "description": T(
            "Standing on its foot on the plant room floor and bolted between the flanges at the foot of a heating riser, the pump delivers 50 m3/h against 14 m at 1450 rpm with heating water of 70 degC; the fan draws cooling air through the cowl grille, the blanking plates stand on edge on the pallet beside it.",
            "Auf ihrem Fuß auf dem Boden der Heizzentrale stehend und zwischen die Flansche am Fuß eines Heizungssteigstrangs geschraubt, fördert die Pumpe 50 m3/h gegen 14 m bei 1450 1/min mit Heizwasser von 70 °C; der Lüfter saugt Kühlluft durch das Haubengitter, die Blindscheiben stehen hochkant auf der Palette daneben."),
         "bones": reinstalled_bones, "ground": "authored", "contacts": feet, "thermal": {"operating": {"pump": 1, "heating": 1}, "regions": binding(regions, True), "emitters": {"motor-fan": 1}}, "loads": service_loads},
        {"id": "reinstalled-off", "title": T("Reinstalled, filled and stopped", "Wieder eingebaut, befüllt und stillstehend"), "description": T(
            "The same plant room state with the motor switched off and the heating cold: the 5 l of water and the riser load the housing, everything is at room temperature.",
            "Derselbe Zustand in der Heizzentrale mit abgeschaltetem Motor und kalter Heizung: die 5 l Wasser und der Strang belasten das Gehäuse, alles hat Raumtemperatur."),
         "bones": reinstalled_bones, "ground": "authored", "contacts": feet, "thermal": {"operating": cold, "regions": binding(regions, True), "emitters": {"motor-fan": 0}}, "loads": service_loads},
    ]
    clips = [
        {"id": "lift", "title": T("Lift off the pallet", "Von der Palette abheben"), "description": T(
            "The sling takes the weight after two seconds and the hoist lifts the pump with its blanking plates 0.40 m clear of the pallet in six seconds.",
            "Das Gehänge übernimmt nach zwei Sekunden das Gewicht, und der Kettenzug hebt die Pumpe mit ihren Blindscheiben in sechs Sekunden 0,40 m über die Palette."),
         "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}, {"t": 10, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 2.2} for a in ground] + [{"anchor": "lug-0", "fromS": 2, "toS": 10}, {"anchor": "lug-1", "fromS": 2, "toS": 10}],
         "phases": [{"id": "sling-on", "fromS": 0, "toS": 2, "title": T("Slinging", "Anschlagen")}, {"id": "hoist", "fromS": 2, "toS": 8, "title": T("Hoisting", "Heben")}, {"id": "hold", "fromS": 8, "toS": 10, "title": T("Holding", "Halten")}],
         "events": [{"t": 2.2, "kind": "contact-off", "title": T("Pump foot leaves the pallet", "Pumpenfuß verlässt die Palette"), "target": "pump"}],
         "tracks": []},
        {"id": "run-up", "title": T("Start-up after reinstallation", "Anfahren nach dem Wiedereinbau"), "description": T(
            "The refilled pump starts after half a minute; the motor warms up under its losses while the heating water of the riser comes up to 70 degC over 25 minutes.",
            "Die wieder befüllte Pumpe läuft nach einer halben Minute an; der Motor erwärmt sich unter seinen Verlusten, während das Heizwasser des Strangs in 25 Minuten auf 70 °C kommt."),
         "durationS": 1800, "fps": 1, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 30, "pose": "reinstalled", "ease": "step"}, {"t": 1800, "pose": "reinstalled", "ease": "step"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 1800} for a in feet],
         "phases": [{"id": "stopped", "fromS": 0, "toS": 30, "title": T("Stopped", "Stillstand")}, {"id": "warm-up", "fromS": 30, "toS": 1530, "title": T("Warming up", "Aufwärmen")}, {"id": "design", "fromS": 1530, "toS": 1800, "title": T("Design duty", "Auslegungsbetrieb")}],
         "events": [{"t": 30, "kind": "ignition", "title": T("Motor starts", "Motor läuft an"), "target": "pump"}, {"t": 1530, "kind": "operating", "title": T("Heating water reaches 70 degC", "Heizwasser erreicht 70 °C"), "target": "heating"}],
         "tracks": [{"target": "operating:pump", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 1}, {"t": 1800, "v": 1}]},
                    {"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 0}, {"t": 1530, "v": 1}, {"t": 1800, "v": 1}]},
                    {"target": "emitter:motor-fan", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 30, "v": 1}, {"t": 1800, "v": 1}]}]},
    ]
    unavailable = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T(
        "At 1:500 the pump on its pallet is 1.6 mm long, 1.2 mm wide and 1.8 mm high on paper, under the 3 mm a recognisable drawing needs, and the 22 mm flanges, the 16 mm motor flange and the 12 mm blanking plates shrink below 0.05 mm; draw it at 1:200 or larger.",
        "Bei 1:500 ist die Pumpe auf ihrer Palette auf Papier 1,6 mm lang, 1,2 mm breit und 1,8 mm hoch, unter den 3 mm, die eine erkennbare Zeichnung braucht, und die 22 mm Flansche, der 16 mm Motorflansch und die 12 mm Blindscheiben schrumpfen unter 0,05 mm; bei 1:200 oder größer zeichnen."),
        "limitingFeature": T("Port flange 22 mm thick", "Stutzenflansch 22 mm dick"), "limitingSizeM": 0.022, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    asset = {
        "id": SLUG, "category": "components", "title": T("Used Dry-Rotor Inline Pump", "Gebrauchte Trockenläufer-Inlinepumpe"), "summary": SUMMARY,
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "plumbing", "pump", "inline-pump", "dry-rotor", "electric-motor", "cast-iron", "flange", "pallet", "timber", "operating", "thermal"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": COWL[1], "widthM": PALLET_L, "depthM": PALLET_W, "massKg": r6(M_TOTAL)},
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
    print(f"[DEBUG] pump {M_PUMP:.2f} kg com {COM_PUMP[0]:.4f} {COM_PUMP[1]:.4f}; blanks {M_BLANKS:.2f} kg; pallet {M_PALLET:.2f} kg; total {M_TOTAL:.2f} kg")
