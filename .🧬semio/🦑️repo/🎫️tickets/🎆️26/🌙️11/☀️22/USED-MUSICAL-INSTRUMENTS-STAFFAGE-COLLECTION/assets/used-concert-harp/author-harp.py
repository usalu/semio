"""🎼️ Authoritative generator of the staffage asset `used-concert-harp`: a used 47-string concert pedal harp of 1.85 m and
36 kg on a castor trolley under its padded cover, as a used-instrument dealer keeps it; rolled on the trolley, uncovered and
upright, tilted back onto the harpist's right shoulder and played, with the pedal action. Running it rewrites `🔣️.json`.

@see ../../brief.md
@see author_harp_geometry.py
@see author_harp_parts.py
@see author_harp_texts.py
"""

import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from author_harp_geometry import (  # noqa: E402
    BASE_X0, BASE_X1, BASE_Y0, BASE_Y1, BASE_Z, BOARD_T, CARPET_T, CASTOR_X, CASTOR_Z, COL_TOP, COL_X, COL_Y0, COLUMN_FOOT,
    COVER_HEM, COVER_TURN_Z, CROWN, DECK_T, DECK_Y0, FEET_X, FEET_Z, FOOT_R, NECK_HALF, PEDAL_LEN, PEDAL_NOTCH_DEG,
    PEDAL_PIVOT, PEDALS, PLATE_T, RIM, SB_LEN, SHAFT_R0, SHAFT_R1, SHAFT_Y0, SHAFT_Y1, STRIP_T, STRIP_W, T_DECK, TILT_DEG,
    TROLLEY_HW, TROLLEY_LEN, TROLLEY_X0, WHEEL_R, WHEEL_W, dirv, neck_profile, plate_profile, r6, sb_dp, sb_hw, sb_point,
    strings,
)
from author_harp_parts import PEDAL_REST, Parts, Rig, bone, circle, cover_outline, cover_plane, foot_rings, rect, rig_core, rnd, sb_ring  # noqa: E402
import author_harp_texts as TX  # noqa: E402

SLUG = "used-concert-harp"
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
G = 9.80665
TENSION_N = 8000.0
ROLL_M = 1.2
TROLLEY_PARK = 0.95


def t(en, de):
    """📝️ One bilingual text."""
    return {"en": en, "de": de}


def rot(axis, ang, p):
    """🔁️ Rotates p about the unit axis by ang radians (right-handed numbers)."""
    c, s = math.cos(ang), math.sin(ang)
    x, y, z = axis
    d = x * p[0] + y * p[1] + z * p[2]
    cr = (y * p[2] - z * p[1], z * p[0] - x * p[2], x * p[1] - y * p[0])
    return tuple(p[i] * c + cr[i] * s + axis[i] * d * (1 - c) for i in range(3))


A_HW, B_HW, C_HW = cover_plane()


def hw_world(x, y):
    """↔️ Lateral half width of the cover at a world rest point."""
    return A_HW + B_HW * x + C_HW * (y - T_DECK)


GRAD = math.hypot(B_HW, C_HW)
COVER_ANGLE = math.degrees(math.atan2(C_HW, B_HW))
N_LEFT = (-B_HW, -C_HW, 1.0)


def cover_turn():
    """🛏️️ Axis direction and twist angle of the cover pivot that turn the right panel of the padded cover face down onto the floor."""
    nn = math.sqrt(sum(x * x for x in N_LEFT))
    n = tuple(x / nn for x in N_LEFT)
    ph = math.atan2(-n[0], 1 + n[1])
    ax = (math.cos(ph), math.sin(ph), 0.0)
    best = min(((sum((q - r) ** 2 for q, r in zip(rot(ax, -math.radians(j / 100.0), n), (0.0, -1.0, 0.0))), j / 100.0) for j in range(-12000, -6000)))
    return math.degrees(ph), best[1]


def build_rig():
    """🌳️ The whole rig at rest: the core bones and the cover pivot solved so that the lying cover rests on the floor."""
    core = rig_core()
    outline = [(x, y + T_DECK) for x, y in cover_outline(40)]
    gx, gy = B_HW / GRAD, C_HW / GRAD
    s = [p[0] * gx + p[1] * gy for p in outline]
    o = outline[s.index(min(s))]
    length = max(s) - min(s)
    phi, alpha = cover_turn()
    ax = (math.cos(math.radians(phi)), math.sin(math.radians(phi)), 0.0)
    xs = 0.02
    z = COVER_TURN_Z
    corners = [(x, y, hw_world(x, y)) for x, y in outline]
    for _ in range(40):
        rp = [rot(ax, -math.radians(alpha), p) for p in corners]
        e = [rot(ax, -math.radians(alpha), v) for v in ((1, 0, 0), (0, 1, 0), (0, 0, 1))]
        ryx, ryy, ryz = e[0][1], e[1][1], e[2][1]
        yc = (ryx * xs + ryz * z - min(q[1] for q in rp)) / (1 - ryy)
        oc = (xs, yc, z)
        world = [tuple(oc[i] + rot(ax, -math.radians(alpha), tuple(p[i2] - oc[i2] for i2 in range(3)))[i] for i in range(3)) for p in [(x, y, sgn * hw_world(x, y)) for x, y in outline for sgn in (1, -1)]]
        zmin = min(w[2] for w in world)
        if abs(zmin - 0.33) < 1e-5:
            break
        z += (0.33 - zmin) / 1.9
    ct = bone("cover-turn", "floor", xs, yc, phi, 0.05, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": r6(math.floor(alpha) - 4), "twistMaxDeg": 0}, z)
    cr = math.radians(phi)
    dx, dy = o[0] - xs, o[1] - yc
    cv = bone("cover", "cover-turn", dx * math.cos(cr) + dy * math.sin(cr), -dx * math.sin(cr) + dy * math.cos(cr), COVER_ANGLE - phi, length, {"type": "fixed"})
    return core + [ct, cv], r6(alpha), (xs, yc, z)


BONES, COVER_TWIST, COVER_PIVOT = build_rig()
RIG = Rig(BONES)
BASS = strings()[0]


def cover_hw_u(u):
    """↔️ Lateral half width of the cover at a distance u along its bone."""
    (ox, oy), _, _ = RIG.f["cover"]
    return hw_world(ox, oy) + GRAD * u


def harp(hx, hy):
    """🪜️ A harp-frame point in world rest coordinates."""
    return (hx, hy + T_DECK)


def lw(bid, pts):
    """🌍️ Local points of a bone in world rest coordinates."""
    return [RIG.world(bid, p) for p in pts]


def body_parts(scale):
    """🧊️ The parts of one scale: every solid of the body with its plain drawings (the cover is drawn with `solid` shapes)."""
    P = Parts(RIG, scale)
    x0, x1, hw = TROLLEY_X0, TROLLEY_X0 + TROLLEY_LEN, TROLLEY_HW
    deck_top = DECK_Y0 + DECK_T
    if scale == 50:
        P.plate_v("deck", "trolley", "plywood-birch", 0.72, "trolley", rect(x0, -hw, x1, hw), DECK_Y0, deck_top)
        P.plate_v("carpet", "trolley", "carpet-felt", 0.32, "trolley", rect(x0 + 0.004, -hw + 0.004, x1 - 0.004, hw - 0.004), deck_top, deck_top + CARPET_T, extra={"sheet": True})
    elif scale in (100, 200):
        top = deck_top + CARPET_T + (RIM if scale == 200 else 0.0)
        P.plate_v("deck", "trolley", "plywood-birch", 0.72, "trolley", rect(x0, -hw, x1, hw), DECK_Y0, top)
    else:
        P.plate_v("trolley-block", "trolley", "plywood-birch", 0.7, None, rect(x0, -hw, x1, hw), 0.0, deck_top + CARPET_T, regions=["trolley", "castors"], extra={"pen": 0.2})
    if scale in (50, 100):
        y0, y1 = deck_top + CARPET_T, deck_top + CARPET_T + RIM
        P.prism("rim-rear", "trolley", "pine-batten", 0.55, "trolley", rect(x0, y0, x0 + RIM, y1), -hw, hw)
        P.prism("rim-front", "trolley", "pine-batten", 0.55, "trolley", rect(x1 - RIM, y0, x1, y1), -hw, hw)
        P.prism("rim-left", "trolley", "pine-batten", 0.55, "trolley", rect(x0 + RIM, y0, x1 - RIM, y1), hw - RIM, hw)
        P.prism("rim-right", "trolley", "pine-batten", 0.55, "trolley", rect(x0 + RIM, y0, x1 - RIM, y1), -hw, -hw + RIM)
    if scale != 500:
        for cid, cx in (("rear", CASTOR_X[0]), ("front", CASTOR_X[1])):
            for side, zc in (("l", CASTOR_Z), ("r", -CASTOR_Z)):
                ww = WHEEL_W if scale != 200 else 0.03
                P.prism(f"wheel-{cid}-{side}", f"castors-{cid}", "polyurethane-grey", 0.22, "castors", circle(cx, WHEEL_R, WHEEL_R, {50: 20, 100: 16, 200: 12}[scale]), zc - ww / 2, zc + ww / 2)
                if scale == 50:
                    P.prism(f"castor-plate-{cid}-{side}", "trolley", "steel-zinc", 0.45, "castors", rect(cx - 0.03, 0.06, cx + 0.03, DECK_Y0), zc - 0.03, zc + 0.03)
                    P.prism(f"castor-yoke-{cid}-{side}", "trolley", "steel-zinc", 0.45, "castors", rect(cx - 0.018, 2 * WHEEL_R, cx + 0.018, 0.06), zc - 0.016, zc + 0.016)
                elif scale == 100:
                    P.prism(f"castor-yoke-{cid}-{side}", "trolley", "steel-zinc", 0.45, "castors", rect(cx - 0.022, 2 * WHEEL_R, cx + 0.022, DECK_Y0), zc - 0.02, zc + 0.02)
                else:
                    P.prism(f"castor-yoke-{cid}-{side}", "trolley", "steel-zinc", 0.45, "castors", rect(cx - 0.02, 0.038, cx + 0.02, DECK_Y0), zc - 0.02, zc + 0.02)
    T = T_DECK
    if scale == 50:
        P.prism("base", "harp", "maple-lacquer", 0.62, "base", rect(BASE_X0, T + BASE_Y0, BASE_X1, T + 0.115), -BASE_Z, BASE_Z)
        P.prism("base-moulding", "harp", "maple-lacquer", 0.66, "base", rect(BASE_X0 + 0.008, T + 0.115, BASE_X1 - 0.008, T + BASE_Y1), -BASE_Z + 0.008, BASE_Z - 0.008)
    elif scale == 100:
        P.prism("base", "harp", "maple-lacquer", 0.62, "base", rect(BASE_X0, T + BASE_Y0, BASE_X1, T + BASE_Y1), -BASE_Z, BASE_Z)
    elif scale == 200:
        P.prism("base", "harp", "maple-lacquer", 0.62, "base", rect(BASE_X0, T, BASE_X1, T + BASE_Y1), -BASE_Z, BASE_Z)
    else:
        P.prism("base", "harp", "maple-lacquer", 0.62, None, rect(-0.10, T, BASE_X1, T + BASE_Y1), -BASE_Z, BASE_Z, regions=["base", "pedals"], extra={"pen": 0.2})
    if scale in (50, 100):
        prof = [(T, FOOT_R - 0.002), (T + 0.006, FOOT_R), (T + BASE_Y0 + 0.002, FOOT_R)] if scale == 50 else [(T, FOOT_R), (T + BASE_Y0 + 0.002, FOOT_R)]
        for fx, fn in zip(FEET_X, ("rear", "front")):
            for side, zc in (("l", FEET_Z), ("r", -FEET_Z)):
                P.turned(f"foot-{fn}-{side}", "harp", "rubber-black", 0.15, "base", prof, fx, zc)
    for name, zk in PEDALS:
        bid = f"pedal-{name}"
        if scale == 50:
            P.prism(f"pedal-bar-{name}", bid, "steel-nickel", 0.42, "pedals", lw(bid, rect(0.17, -0.008, 0.268, 0.008)), zk - 0.006, zk + 0.006)
            P.prism(f"pedal-toe-{name}", bid, "pedal-rubber", 0.2, "pedals", lw(bid, rect(0.262, -0.011, 0.302, 0.009)), zk - 0.009, zk + 0.009)
        elif scale == 100:
            P.prism(f"pedal-{name}", bid, "steel-nickel", 0.42, "pedals", lw(bid, rect(0.17, -0.009, 0.302, 0.009)), zk - 0.0075, zk + 0.0075)
    if scale == 200:
        P.prism("pedals-left", "harp", "steel-nickel", 0.42, "pedals", rect(-0.102, T + 0.08, -0.025, T + 0.11), 0.062, 0.146)
        P.prism("pedals-right", "harp", "steel-nickel", 0.42, "pedals", rect(-0.102, T + 0.08, -0.025, T + 0.11), -0.186, -0.062)
    foot = {50: COLUMN_FOOT, 100: [(0.13, 0.058), (0.16, 0.052), (0.2, 0.048), (0.25, 0.036)], 200: [(0.13, 0.058), (0.25, 0.04)]}
    crown = {50: CROWN, 100: [(1.6, 0.036), (1.66, 0.06), (1.74, 0.075), (1.8, 0.066), (1.85, 0.045)], 200: [(1.6, 0.038), (1.7, 0.072), (1.78, 0.072), (1.85, 0.05)]}
    if scale != 500:
        P.turned("column-foot", "column", "gold-leaf-worn", 0.78, "gilding", [(T + y, r) for y, r in foot[scale]], COL_X, 0.0)
        P.turned("column-shaft", "column", "maple-lacquer", 0.6, "column", [(T + SHAFT_Y0 - 0.002, SHAFT_R0), (T + SHAFT_Y1 + 0.002, SHAFT_R1)], COL_X, 0.0)
        P.turned("crown", "column", "gold-leaf-worn", 0.8, "gilding", [(T + y, r) for y, r in crown[scale]], COL_X, 0.0)
    else:
        P.turned("column", "column", "maple-lacquer", 0.62, None, [(T + y, r) for y, r in [(0.13, 0.058), (0.25, 0.04), (0.27, 0.038), (1.6, 0.038), (1.7, 0.075), (1.79, 0.075), (1.85, 0.05)]], COL_X, 0.0, regions=["column", "gilding"])
    neck = [harp(x, y) for x, y in neck_profile()]
    if scale == 500:
        P.prism("neck", "neck", "maple-lacquer", 0.58, None, neck, -0.038, 0.038, regions=["neck", "action", "strings"])
    else:
        P.prism("neck", "neck", "maple-lacquer", 0.58, "neck", neck, -NECK_HALF, NECK_HALF)
        plate = [harp(x, y) for x, y in plate_profile()]
        if scale == 200:
            plate = plate[::2] + ([plate[-1]] if len(plate) % 2 == 0 else [])
        P.prism("plate-left", "neck", "brass-tarnished", 0.5, "action", plate, NECK_HALF, NECK_HALF + PLATE_T, extra={"sheet": True, "sideOnly": scale == 200})
        P.prism("plate-right", "neck", "brass-tarnished", 0.5, "action", plate, -NECK_HALF - PLATE_T, -NECK_HALF, extra={"sheet": True, "sideOnly": scale == 200})
    n_ring, s_slab = {50: (24, 0.0), 100: (16, 0.008), 200: (12, 0.024), 500: (8, 0.067)}[scale]
    P.shell("soundbox-back", "soundbox", "maple-lacquer", 0.55, "soundbox", [sb_ring(0.0, "back", s_slab, n_ring), sb_ring(SB_LEN, "back", s_slab, n_ring)])
    P.shell("soundboard", "soundbox", "spruce-varnished", 0.8, "soundboard", [sb_ring(0.0, "slab", s_slab, n_ring), sb_ring(SB_LEN, "slab", s_slab, n_ring)])
    P.shell("soundbox-foot", "soundbox", "maple-lacquer", 0.5, "soundbox", foot_rings(n_ring))
    if scale == 50:
        P.prism("centre-strip", "soundbox", "maple-lacquer", 0.45, "soundboard", lw("soundbox", rect(0.05, -BOARD_T - STRIP_T, 1.42, -BOARD_T)), -STRIP_W, STRIP_W)
    keep = {50: lambda i: True, 100: lambda i: i % 7 in (0, 3), 200: lambda i: i % 7 == 0, 500: lambda i: False}[scale]
    for i, note, ft, top, d, mat in strings():
        if keep(i):
            P.wire(f"string-{note.lower()}", "strings", mat, 0.35 if note[0] in "CF" else 0.5, "strings", (ft[0], ft[1] + T, 0.0), (ft[0], top + T, 0.0), d)
    n_cov = {50: 28, 100: 20, 200: 14, 500: 10}[scale]
    outline = [(x, y + T) for x, y in cover_outline(n_cov)]
    loc = [RIG.local("cover", p) for p in outline]
    zb = RIG.f["cover"][2]
    grow = COVER_GROW.get(scale, 0.0)
    rings = [[(u, v, -zb + cover_hw_u(u) + grow) for u, v in loc], [(u, v, -zb - cover_hw_u(u) - grow) for u, v in loc]]
    from author_harp_parts import polyhedron
    P.add("cover", "cover", "padded-nylon", 0.26, "cover", polyhedron(rings), None, None, None, extra={"profile": loc})
    return P


COVER_GROW = {}


def shape(sid, bid, role, z, filled, tone, geom, region=None, regions=None, material=None, pen=None, solid=None, hatch=None):
    """✏️️ One 2D shape."""
    s = {"id": sid, "bone": bid, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    s.update(geom)
    if solid is not None:
        s["solid"] = solid
    return s


def poly(points, closed=True):
    """〰️️ A poly geometry."""
    return {"kind": "poly", "points": rnd(points), "closed": closed}


ZORDER = {"trolley": 1.0, "castors": 1.2, "base": 2.0, "pedals": 2.4, "soundbox": 3.0, "soundboard": 3.1, "strings": 3.4, "column": 4.0, "gilding": 4.2, "neck": 5.0, "action": 5.1, "cover": 9.0}


def drawings(scale, parts):
    """🖊️ The three views of one scale: the exact projection of every part plus the hand-drawn detail and texture."""
    views = {"elevation": [], "front": [], "plan": []}
    pen = PENS[scale]
    for it in parts.items:
        reg = it["region"] or (it["regions"] or ["x"])[0]
        z = ZORDER.get(reg, 2.0)
        common = {"region": it["region"], "regions": it["regions"], "material": it["material"]}
        if it["id"] == "cover":
            (ox, oy), _, _ = RIG.f["cover"]
            hw0 = cover_hw_u(0.0) + COVER_GROW.get(scale, 0.0)
            hwl = cover_hw_u(RIG_LEN["cover"]) + COVER_GROW.get(scale, 0.0)
            solid = {"kind": "extruded", "sweep": "w", "z0": r6(-hw0), "z1": r6(hw0), "zTip": [r6(-hwl), r6(hwl)]}
            for v in views:
                views[v].append(shape("cover", "cover", "body", z, True, it["tone"], poly(it["extra"]["profile"]), solid=solid, hatch="quilt" if scale in (50, 100) else None, **common))
            continue
        if it["solid"]["kind"] == "capsule":
            views["elevation"].append(shape(it["id"], it["bone"], "detail", z, False, it["tone"], poly(it["elev"], False), pen=pen["detail"] or pen["body"], **common))
            continue
        for v, ring in (("elevation", it["elev"]), ("front", it["front"]), ("plan", it["plan"])):
            if v != "elevation" and it["extra"].get("sideOnly"):
                continue
            views[v].append(shape(it["id"], it["bone"], "body", z, True, it["tone"], poly(ring), pen=it["extra"].get("pen"), **common))
    wires = [it for it in parts.items if it["solid"]["kind"] == "capsule"]
    if wires:
        b0 = min(it["front"][0][1] for it in wires)
        b1 = max(it["front"][1][1] for it in wires)
        views["front"].append(shape("strings-plane", "strings", "detail", 3.4, False, 0.4, poly([(0.0, b0), (0.0, b1)], False), region="strings", pen=pen["detail"] or pen["body"]))
        xs = [it["plan"][0][0] for it in wires]
        views["plan"].append(shape("strings-plane", "strings", "detail", 3.4, False, 0.4, poly([(min(xs), 0.0), (max(xs), 0.0)], False), region="strings", pen=pen["detail"] or pen["body"]))
    for v, extra in TX.details(scale, RIG, harp, lw, cover_hw_u).items():
        views[v] += extra
    return views


PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.13, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.18},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {50: [{"id": "quilt", "title": t("Quilted padding of the cover", "Gesteppte Polsterung der Hülle"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.6, "penMm": 0.1}], 100: [{"id": "quilt", "title": t("Quilted padding of the cover", "Gesteppte Polsterung der Hülle"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1}], 200: [], 500: []}
RIG_LEN = {b["id"]: b["lengthM"] for b in BONES}


def solid_json(it, prefix=""):
    """🧊️ The body solid of a part."""
    s = {"id": prefix + it["id"], "bone": it["bone"], "material": it["material"], "role": it["role"], "tone": it["tone"]}
    s.update(it["solid"])
    s.update(it["extra"].get("sheet") and {"sheet": True} or {})
    return s


def thermal_only(scale):
    """🌡️️ Thermal solids that the body of a coarse scale merges: strings, action plates, crown, pedals and castors kept as their own surfaces."""
    out = []
    T = T_DECK
    P = Parts(RIG, scale)
    if scale in (100, 200):
        for i, note, ft, top, d, mat in strings():
            if (scale == 100 and i % 7 not in (0, 3)) or (scale == 200 and i % 7 != 0):
                P.wire(f"th-string-{note.lower()}", "strings", mat, 0.4, "strings", (ft[0], ft[1] + T, 0.0), (ft[0], top + T, 0.0), d)
    if scale == 500:
        for i, note, ft, top, d, mat in strings():
            if i % 3 == 0:
                P.wire(f"th-string-{note.lower()}", "strings", mat, 0.4, "strings", (ft[0], ft[1] + T, 0.0), (ft[0], top + T, 0.0), d)
        plate = [harp(x, y) for x, y in plate_profile()][::2]
        P.prism("th-plate-left", "neck", "brass-tarnished", 0.5, "action", plate, 0.038, 0.038 + PLATE_T, extra={"sheet": True})
        P.prism("th-plate-right", "neck", "brass-tarnished", 0.5, "action", plate, -0.038 - PLATE_T, -0.038)
        P.items[-1]["extra"] = {"sheet": True}
        P.turned("th-crown", "column", "gold-leaf-worn", 0.8, "gilding", [(T + y, r) for y, r in [(1.62, 0.04), (1.7, 0.074), (1.79, 0.074), (1.849, 0.049)]], COL_X, 0.0)
        P.prism("th-pedals", "harp", "steel-nickel", 0.42, "pedals", rect(-0.1, T + 0.054, -0.024, T + 0.131), -0.186, 0.146)
        for cid, cx in (("rear", CASTOR_X[0]), ("front", CASTOR_X[1])):
            for side, zc in (("l", CASTOR_Z), ("r", -CASTOR_Z)):
                P.prism(f"th-castor-{cid}-{side}", "trolley", "polyurethane-grey", 0.22, "castors", rect(cx - 0.038, 0.0, cx + 0.038, 0.076), zc - 0.038, zc + 0.038)
    for it in P.items:
        s = solid_json(it)
        s["region"] = it["region"]
        out.append(s)
    return out


def thermal_list(scale, parts):
    """🌡️️ The thermal list: references to the body solids by region plus the surfaces the coarse body merges."""
    out = []
    for it in parts.items:
        reg = it["region"] or it["regions"][0]
        out.append({"id": f"th-{it['id']}", "of": it["id"], "region": reg})
    return out + thermal_only(scale)


def sphere(sid, bid, material, centre_world, r, member, z=0.0):
    """⚫️ A mass marker sphere at a world rest point."""
    q = RIG.local(bid, centre_world)
    zb = RIG.f[bid][2]
    return {"id": sid, "kind": "ellipsoid", "bone": bid, "material": material, "role": "detail", "tone": 0.3, "radii": [r6(r)] * 3, "transform": {"translation": [r6(q[0]), r6(q[1]), r6(z - zb)]}, "member": member, "part": "mass"}


def block(sid, bid, material, centre_world, size, z, member, part, anchor=None):
    """▪️ A structural block at a world rest point."""
    q = RIG.local(bid, centre_world)
    zb = RIG.f[bid][2]
    s = {"id": sid, "kind": "box", "bone": bid, "material": material, "role": "detail", "tone": 0.3, "size": [r6(x) for x in size], "transform": {"translation": [r6(q[0]), r6(q[1]), r6(z - zb)]}, "member": member, "part": part}
    if anchor:
        s["anchor"] = anchor
    return s


def structure_list(scale, parts):
    """🏗️️ The structural idealisation of one scale: members along the bones, joints where they meet, supports at the castor contacts and masses at the centres."""
    f = FEATURE[scale]
    out = []
    ids = {it["id"]: it for it in parts.items}
    member_of = {"deck": "trolley", "trolley-block": "trolley", "base": "harp", "column-shaft": "column", "column": "column", "neck": "neck", "soundbox-back": "soundbox", "cover": "cover"}
    for pid, mem in member_of.items():
        if pid in ids:
            out.append({"id": f"st-{pid}", "of": pid, "member": mem, "part": "member"})
    for pid, it in ids.items():
        if pid.startswith("foot-") or pid in ("soundbox-foot", "column-foot") or pid.startswith("castor-yoke") or pid.startswith("wheel-"):
            mem = {"soundbox-foot": "soundbox", "column-foot": "column"}.get(pid, it["bone"] if pid.startswith("wheel-") else ("harp" if pid.startswith("foot-") else "trolley"))
            out.append({"id": f"st-{pid}", "of": pid, "member": mem, "part": "joint"})
        if pid == "carpet":
            out.append({"id": "st-carpet", "of": pid, "member": "trolley", "part": "joint"})
        if pid.startswith("pedal-bar-") or (scale == 100 and pid.startswith("pedal-")):
            out.append({"id": f"st-{pid}", "of": pid, "member": it["bone"], "part": "member"})
    if scale in (200, 500):
        for name, zk in PEDALS:
            bid = f"pedal-{name}"
            a, b = lw(bid, [(0.0, 0.0), (0.27, 0.0)])
            q0, q1 = RIG.local(bid, a), RIG.local(bid, b)
            out.append({"id": f"st-pedal-{name}", "kind": "box", "bone": bid, "material": "steel-nickel", "role": "detail", "tone": 0.4, "size": [0.27, 0.016, 0.012], "transform": {"translation": [0.135, 0.0, 0.0]}, "slender": True, "member": bid, "part": "member"})
    T = T_DECK
    sel = {50: lambda i: i % 2 == 0, 100: lambda i: i % 4 == 0, 200: lambda i: i % 7 == 0, 500: lambda i: i % 15 == 0}[scale]
    for i, note, ft, top, d, mat in strings():
        if sel(i):
            a, b = RIG.local("strings", (ft[0], ft[1] + T, 0.0)), RIG.local("strings", (ft[0], top + T, 0.0))
            out.append({"id": f"st-string-{note.lower()}", "kind": "capsule", "bone": "strings", "material": mat, "role": "detail", "tone": 0.4, "p0": [r6(x) for x in a], "p1": [r6(x) for x in b], "r0": r6(d / 2), "r1": r6(d / 2), "slender": True, "member": "strings", "part": "member"})
    out.append(block("st-joint-neck-column", "neck", "maple-lacquer", harp(0.46, 1.76), [max(0.05, f), max(0.08, f), max(0.048, f)], 0.0, "neck", "joint"))
    out.append(block("st-joint-neck-soundbox", "neck", "maple-lacquer", harp(-0.47, 1.49), [max(0.06, f), max(0.06, f), max(0.048, f)], 0.0, "neck", "joint"))
    out.append(block("st-joint-column-base", "column", "maple-lacquer", harp(COL_X, 0.135), [max(0.04, f), max(0.1, f), max(0.1, f)], 0.0, "column", "joint"))
    if scale in (200, 500):
        out.append(block("st-joint-harp-deck", "harp", "maple-lacquer", harp(0.26, f / 2 - 0.003), [0.5, f, 0.36], 0.0, "harp", "joint"))
    for cid, cx in (("rear", CASTOR_X[0]), ("front", CASTOR_X[1])):
        for side, zc in (("l", CASTOR_Z), ("r", -CASTOR_Z)):
            s = max(f, 0.012)
            out.append(block(f"st-support-{cid}-{side}", "trolley", "polyurethane-grey", (cx, s / 2), [s, s, s], zc, "trolley", "support", anchor=f"castor-{cid}-{side}"))
            if scale == 500:
                out.append(block(f"st-castor-{cid}-{side}", "trolley", "steel-zinc", (cx, 0.04), [f, 0.08, f], zc, "trolley", "joint"))
    for bid, (mx, my) in MASS_AT.items():
        r = max(f / 2, 0.012)
        out.append(sphere(f"st-mass-{bid}", bid, "maple-lacquer", RIG.world(bid, (mx, my)), r, bid, 0.0))
    return out


def cover_centroid():
    """⚖️️ Centroid of the side outline of the cover in its bone frame (the centre of its mass)."""
    loc = [RIG.local("cover", (x, y + T_DECK)) for x, y in cover_outline(40)]
    a = cx = cy = 0.0
    for p, q in zip(loc, loc[1:] + loc[:1]):
        c = p[0] * q[1] - q[0] * p[1]
        a += c
        cx += (p[0] + q[0]) * c
        cy += (p[1] + q[1]) * c
    return (cx / (3 * a), cy / (3 * a))


MASS_AT = {"trolley": (0.34, 0.012), "harp": (0.265, 0.075), "column": (0.84, 0.0), "neck": (0.45, 0.05), "soundbox": (0.62, 0.075), "strings": (0.55, 0.33), "cover": cover_centroid()}
MASS_Z = {"cover": 0.0}


def dimensions(reps):
    """📏️ Overall envelope of the stored pose from the 1:50 drawings and the cover prism."""
    g = reps["50"]["geometry"]
    xs, ys, zs = [], [], []
    for s in g["plan"]:
        if s["kind"] == "poly" and "solid" not in s:
            xs += [p[0] for p in s["points"]]
            zs += [p[1] for p in s["points"]]
    for s in g["front"]:
        if s["kind"] == "poly" and "solid" not in s:
            ys += [p[1] for p in s["points"]]
    for x, y in cover_outline(28):
        h = hw_world(x, y + T_DECK)
        xs.append(x)
        ys.append(y + T_DECK)
        zs += [h, -h]
    return {"heightM": r6(max(ys) - min(ys)), "widthM": r6(max(xs) - min(xs)), "depthM": r6(max(zs) - min(zs))}


def volume(scale):
    """🧊️ The authored volume of one scale."""
    parts = body_parts(scale)
    return parts, {"status": "authored", "denominator": scale, "body": [solid_json(it) for it in parts.items], "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": TX.VOLUME_DECISIONS[scale]}


def representation(scale, parts):
    """🖨️️ The authored representation of one scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": drawings(scale, parts), "decisions": TX.DECISIONS[scale]}


def main():
    """📦️ Builds and writes the asset document."""
    vols, reps = {}, {}
    for s in SCALES:
        parts, vol = volume(s)
        vols[str(s)] = vol
        reps[str(s)] = representation(s, parts)
    dims = dimensions(reps)
    asset = TX.asset(SLUG, BONES, RIG, reps, vols, COVER_TWIST, COVER_PIVOT, dims, MASS_AT, cover_hw_u)
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] wrote {SLUG}: twist {COVER_TWIST} pivot {COVER_PIVOT} dims {asset['dimensions']}")


if __name__ == "__main__":
    main()
