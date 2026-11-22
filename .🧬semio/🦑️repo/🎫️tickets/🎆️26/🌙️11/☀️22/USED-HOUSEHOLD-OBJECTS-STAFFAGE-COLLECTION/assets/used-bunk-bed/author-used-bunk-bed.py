#!/usr/bin/env python3
"""🛏️ Authoritative generator of the staffage asset `used-bunk-bed`: a used solid-beech children's bunk bed for two 90 x 200 cm mattresses, dismantled into a strapped bundle on a moving blanket, carried by two movers and set up again in a children's room.

Every number of the asset is stated here; the script writes `🔣️.json` next to itself. Parts are prisms of their true section in world rest coordinates (the assembled bed: x along the bed from the head end, y up, z to the left, the ladder on the right), each bound to the bone that carries it; the same prism is the body solid of a scale and, as an extruded `solid`, its elevation, front and plan shape.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
G = 9.80665
SCALES = ["50", "100", "200", "500"]

POST = 0.057
POST_H = 1.60
X_HEAD = (0.0, POST)
X_FOOT = (2.067, 2.124)
Z_POST = (0.457, 0.514)
Z_RAIL = (0.470, 0.498)
Z_LEDGER = (0.448, 0.470)
Z_GUARD = (0.478, 0.498)
Y_RAIL = {"low": (0.120, 0.230), "up": (1.040, 1.150)}
Y_LEDGER = {"low": (0.175, 0.205), "up": (1.095, 1.125)}
Y_SLAT = {"low": (0.205, 0.223), "up": (1.125, 1.143)}
Y_MAT = {"low": (0.223, 0.323), "up": (1.143, 1.243)}
X_MAT = (0.062, 2.062)
Z_MAT = 0.45
SLAT_N = 15
SLAT_W = 0.068
SLAT_X0 = 0.075
SLAT_SPAN = 1.974
SLAT_PITCH = (SLAT_SPAN - SLAT_W) / (SLAT_N - 1)
TYPE_N = 8
TYPE_W = 0.136
TYPE_PITCH = (SLAT_SPAN - TYPE_W) / (TYPE_N - 1)
Z_SLAT = 0.465
Y_GUARD_LOW = (1.215, 1.295)
Y_GUARD_UP = (1.360, 1.440)
X_GUARD = {"l": (0.057, 2.067), "r": (0.057, 1.667)}
END_RAILS = {"bottom": (0.120, 0.230), "mid": (0.420, 0.500), "upper": (1.040, 1.150), "top": (1.440, 1.530)}
Y_INFILL = (1.150, 1.440)
X_STILES = [(1.629, 1.667), (2.029, 2.067)]
Z_LADDER = (-0.555, -0.498)
LADDER_H = 1.47
RUNG_TOPS = [0.245, 0.480, 0.715, 0.950]
RUNG_T = 0.028
X_BLANKET = (-0.088, 1.062, 2.212)
Z_BLANKET = 0.60
T_BLANKET = 0.008
STRAP_X = [0.5325, 1.5525]
STRAP_W = 0.035
STRAP_T = 0.0012
STRAP_Y = (0.248, 0.8094)
STRAP_Z = 0.514

DENSITY = {"beech-lacquered": 700, "beech-raw": 700, "spruce-raw": 450, "pu-foam": 30, "mattress-cover": 300, "felt-blanket": 125, "pes-webbing": 1100, "pp-webbing": 900, "steel-zinc": 7850}

STORED = {
    "mattress-low": {"dy": 0.008 - Y_MAT["low"][0]},
    "mattress-up": {"dy": 0.108 - Y_MAT["up"][0]},
    "slats-low": {"dy": 0.208 - 0.203},
    "slats-up": {"dy": 0.228 - 1.123},
    "rail-l-low": {"dy": 0.3632 - Y_RAIL["low"][0]},
    "rail-r-low": {"dy": 0.3632 - Y_RAIL["low"][0]},
    "rail-l-up": {"dy": 0.4732 - Y_RAIL["up"][0]},
    "rail-r-up": {"dy": 0.4732 - Y_RAIL["up"][0]},
    "guard-l": {"dy": 0.5832 - Y_GUARD_LOW[0]},
    "guard-r": {"dy": 0.5832 - Y_GUARD_LOW[0]},
    "head-sx": {"dx": 0.262},
    "head-sy": {"dy": 0.3062},
    "head-frame": {"angleDeg": -90},
    "foot-sx": {"dx": 1.862 - X_FOOT[1]},
    "foot-sy": {"dy": 0.3632},
    "foot-frame": {"angleDeg": 90},
    "ladder-sx": {"dx": 0.327 - X_STILES[0][0]},
    "ladder-sy": {"dy": 0.3487 + 0.336},
    "ladder": {"angleDeg": -90, "twistDeg": 90},
}
LIFT = {"bundle-lift": {"dy": 0.55}}
SETUP = {
    "blanket": {"dx": 2.338},
    "blanket-fold": {"angleDeg": 180},
    "strap-a-sx": {"dx": 2.27 - STRAP_X[0]},
    "strap-a-sy": {"dy": 0.0585 - STRAP_Y[0]},
    "strap-a": {"angleDeg": -90},
    "strap-b-sx": {"dx": 2.835 - STRAP_X[1]},
    "strap-b-sy": {"dy": 0.0585 - STRAP_Y[0]},
    "strap-b": {"angleDeg": -90},
}


def T(en, de):
    """📝️ One bilingual text."""
    return {"en": en, "de": de}


def r(value, digits=5):
    """🔢️ A number rounded for the document."""
    out = round(value, digits)
    return 0.0 if out == 0 else out


BONES = [
    ("ground", None, 0, 0, 0, 0, 0, {"type": "fixed"}, T("Floor reference", "Bodenbezug")),
    ("blanket", "ground", X_BLANKET[0], 0, 0, 0, X_BLANKET[1] - X_BLANKET[0], {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.6}, T("Moving blanket, head half (slides out from under the bundle)", "Möbeldecke, Kopfhälfte (zieht unter dem Bündel heraus)")),
    ("blanket-fold", "blanket", X_BLANKET[1] - X_BLANKET[0], T_BLANKET, 0, 0, X_BLANKET[2] - X_BLANKET[1], {"type": "revolute", "minDeg": 0, "maxDeg": 180}, T("Moving blanket, foot half on the fold", "Möbeldecke, Fußhälfte am Falz")),
    ("mattress-low", "ground", X_MAT[0], Y_MAT["low"][0], 0, 0, X_MAT[1] - X_MAT[0], {"type": "prismatic", "axisDeg": 90, "minM": -0.3, "maxM": 0.02}, T("Lower mattress", "Untere Matratze")),
    ("mattress-up", "ground", X_MAT[0], Y_MAT["up"][0], 0, 0, X_MAT[1] - X_MAT[0], {"type": "prismatic", "axisDeg": 90, "minM": -1.1, "maxM": 0.02}, T("Upper mattress", "Obere Matratze")),
    ("slats-low", "ground", SLAT_X0, 0.203, 0, 0, SLAT_SPAN, {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.1}, T("Lower roll-slat base", "Unterer Rollrost")),
    ("slats-up", "ground", SLAT_X0, 1.123, 0, 0, SLAT_SPAN, {"type": "prismatic", "axisDeg": 90, "minM": -1.0, "maxM": 0.02}, T("Upper roll-slat base", "Oberer Rollrost")),
    ("bundle-lift", "ground", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.8}, T("Lift of the strapped frame bundle by the two movers", "Anheben des verzurrten Rahmenbündels durch die beiden Träger")),
    ("rail-l-low", "bundle-lift", 0.057, Y_RAIL["low"][0], 0.484, 0, 2.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.4}, T("Lower side rail, left (wall side), with ledger", "Unterer Seitenholm links (Wandseite) mit Auflageleiste")),
    ("rail-r-low", "bundle-lift", 0.057, Y_RAIL["low"][0], -0.484, 0, 2.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.4}, T("Lower side rail, right (ladder side), with ledger", "Unterer Seitenholm rechts (Leiterseite) mit Auflageleiste")),
    ("rail-l-up", "bundle-lift", 0.057, Y_RAIL["up"][0], 0.484, 0, 2.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.7, "maxM": 0.05}, T("Upper side rail, left, with ledger", "Oberer Seitenholm links mit Auflageleiste")),
    ("rail-r-up", "bundle-lift", 0.057, Y_RAIL["up"][0], -0.484, 0, 2.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.7, "maxM": 0.05}, T("Upper side rail, right, with ledger", "Oberer Seitenholm rechts mit Auflageleiste")),
    ("guard-l", "bundle-lift", 0.057, Y_GUARD_LOW[0], 0.488, 0, 2.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.8, "maxM": 0.05}, T("Fall guard, left, full length", "Absturzsicherung links, durchgehend")),
    ("guard-r", "bundle-lift", 0.057, Y_GUARD_LOW[0], -0.488, 0, 1.61, {"type": "prismatic", "axisDeg": 90, "minM": -0.8, "maxM": 0.05}, T("Fall guard, right, up to the ladder opening", "Absturzsicherung rechts bis zum Leitereinstieg")),
    ("head-sx", "bundle-lift", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": -0.5, "maxM": 0.5}, T("Head end frame, travel along the bed", "Kopfteil, Weg längs des Betts")),
    ("head-sy", "head-sx", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.5}, T("Head end frame, travel up", "Kopfteil, Weg nach oben")),
    ("head-frame", "head-sy", 0, 0, 0, 90, POST_H, {"type": "revolute", "minDeg": -90, "maxDeg": 0}, T("Head end frame: two posts, four cross rails and the infill board", "Kopfteil: zwei Pfosten, vier Querriegel und die Füllung")),
    ("foot-sx", "bundle-lift", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": -0.5, "maxM": 0.5}, T("Foot end frame, travel along the bed", "Fußteil, Weg längs des Betts")),
    ("foot-sy", "foot-sx", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.5}, T("Foot end frame, travel up", "Fußteil, Weg nach oben")),
    ("foot-frame", "foot-sy", X_FOOT[1], 0, 0, 90, POST_H, {"type": "revolute", "minDeg": 0, "maxDeg": 90}, T("Foot end frame: two posts, four cross rails and the infill board", "Fußteil: zwei Pfosten, vier Querriegel und die Füllung")),
    ("ladder-sx", "bundle-lift", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": -1.5, "maxM": 0.5}, T("Ladder, travel along the bed", "Leiter, Weg längs des Betts")),
    ("ladder-sy", "ladder-sx", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.8}, T("Ladder, travel up", "Leiter, Weg nach oben")),
    ("ladder", "ladder-sy", X_STILES[0][0], 0, -0.219, 90, LADDER_H, {"type": "revolute", "minDeg": -90, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 90}, T("Vertical ladder with two stiles and four flat rungs", "Senkrechte Leiter mit zwei Holmen und vier Flachsprossen")),
    ("strap-a-sx", "bundle-lift", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.0}, T("Head-side lashing strap, travel along", "Kopfseitiger Zurrgurt, Weg längs")),
    ("strap-a-sy", "strap-a-sx", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -0.3, "maxM": 0}, T("Head-side lashing strap, travel down", "Kopfseitiger Zurrgurt, Weg nach unten")),
    ("strap-a", "strap-a-sy", STRAP_X[0], STRAP_Y[0], 0, 0, STRAP_W, {"type": "revolute", "minDeg": -90, "maxDeg": 0}, T("Head-side lashing strap with ratchet", "Kopfseitiger Zurrgurt mit Ratsche")),
    ("strap-b-sx", "bundle-lift", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.0}, T("Foot-side lashing strap, travel along", "Fußseitiger Zurrgurt, Weg längs")),
    ("strap-b-sy", "strap-b-sx", 0, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -0.3, "maxM": 0}, T("Foot-side lashing strap, travel down", "Fußseitiger Zurrgurt, Weg nach unten")),
    ("strap-b", "strap-b-sy", STRAP_X[1], STRAP_Y[0], 0, 0, STRAP_W, {"type": "revolute", "minDeg": -90, "maxDeg": 0}, T("Foot-side lashing strap with ratchet", "Fußseitiger Zurrgurt mit Ratsche")),
]
BONE = {b[0]: b for b in BONES}


def origin(bone):
    """📍️ World rest origin of a bone (every parent rests unrotated)."""
    b = BONE[bone]
    if b[1] is None:
        return (0.0, 0.0, 0.0)
    p = origin(b[1])
    return (p[0] + b[2], p[1] + b[3], p[2] + b[4])


def local(bone, X, Y):
    """🧭️ World rest (x, y) to the bone's (u, v)."""
    ox, oy, _ = origin(bone)
    if BONE[bone][5] == 90:
        return (r(Y - oy), r(-(X - ox)))
    return (r(X - ox), r(Y - oy))


def rect(x0, x1, y0, y1):
    """⬛️ A rectangle as world points, counter-clockwise."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def arc(cx, cy, rad, a0, a1, n):
    """🌙️ Points of an arc in world coordinates."""
    return [(cx + rad * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + rad * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def rounded(x0, x1, y0, y1, rad, n, dip=None):
    """🛏️ A rounded rectangle; `dip` = (xa, xb, depth) sinks the top edge in a smooth hollow."""
    top = []
    if dip is not None:
        xa, xb, depth = dip
        steps = 10
        top = [(xb - (xb - xa) * k / steps, y1 - depth * 0.5 * (1 - math.cos(2 * math.pi * k / steps))) for k in range(steps + 1)]
    pts = arc(x0 + rad, y0 + rad, rad, 180, 270, n) + arc(x1 - rad, y0 + rad, rad, 270, 360, n) + arc(x1 - rad, y1 - rad, rad, 0, 90, n) + top + arc(x0 + rad, y1 - rad, rad, 90, 180, n)
    return pts


def piece(pid, bone, pts, z0, z1, mat, region, tone, **kw):
    """🧱️ One prism of the asset: world rest outline in (x, y), lateral span z0..z1, material, thermal region and tone."""
    return dict(id=pid, bone=bone, pts=pts, z0=z0, z1=z1, mat=mat, region=region, tone=tone, **kw)


def box(pid, bone, x0, x1, y0, y1, z0, z1, mat, region, tone, **kw):
    """📦️ A rectangular prism."""
    return piece(pid, bone, rect(x0, x1, y0, y1), z0, z1, mat, region, tone, **kw)


def both(z):
    """↔️ The left and right span of a symmetric pair."""
    return [("l", z[0], z[1]), ("r", -z[1], -z[0])]


def end_frame(scale, end):
    """🪟️ Posts, cross rails and infill of one end frame at one scale."""
    bone = f"{end}-frame"
    xs = X_HEAD if end == "head" else X_FOOT
    xm = (xs[0] + xs[1]) / 2
    out = []
    for side, z0, z1 in both(Z_POST):
        if scale == "50":
            inner = xs[1] if end == "head" else xs[0]
            outer = xs[0] if end == "head" else xs[1]
            c = 0.006
            top = [(xs[1], POST_H - c), (xs[1] - c, POST_H), (xs[0] + c, POST_H), (xs[0], POST_H - c)]
            if end == "head" and side == "r":
                top = [(xs[1], POST_H - 0.016), (xs[1] - 0.014, POST_H - 0.016), (xs[1] - 0.020, POST_H), (xs[0] + c, POST_H), (xs[0], POST_H - c)]
            out.append(piece(f"{end}-post-{side}-foot", bone, rect(xs[0], xs[1], 0, 0.05), z0, z1, "beech-lacquered", "frame", 0.4))
            out.append(piece(f"{end}-post-{side}", bone, [(xs[0], 0.05), (xs[1], 0.05)] + top, z0, z1, "beech-lacquered", "frame", 0.5))
        else:
            out.append(box(f"{end}-post-{side}", bone, xs[0], xs[1], 0, POST_H, z0, z1, "beech-lacquered", "frame", 0.5))
    t = 0.014
    if scale in ("50", "100"):
        for name, (y0, y1) in END_RAILS.items():
            out.append(box(f"{end}-rail-{name}", bone, xm - t, xm + t, y0, y1, -Z_POST[0], Z_POST[0], "beech-lacquered", "frame", 0.56))
        out.append(box(f"{end}-infill", bone, xm - 0.008, xm + 0.008, Y_INFILL[0], Y_INFILL[1], -Z_POST[0], Z_POST[0], "beech-lacquered", "frame", 0.62))
    elif scale == "200":
        for name in ("bottom", "mid"):
            y0, y1 = END_RAILS[name]
            out.append(box(f"{end}-rail-{name}", bone, xm - t, xm + t, y0, y1, -Z_POST[0], Z_POST[0], "beech-lacquered", "frame", 0.56))
        out.append(box(f"{end}-board", bone, xm - t, xm + t, END_RAILS["upper"][0], END_RAILS["top"][1], -Z_POST[0], Z_POST[0], "beech-lacquered", "frame", 0.58))
    else:
        out.append(box(f"{end}-rail-bottom", bone, xm - t, xm + t, END_RAILS["bottom"][0], END_RAILS["bottom"][1], -Z_POST[0], Z_POST[0], "beech-lacquered", "frame", 0.56))
        out.append(box(f"{end}-board", bone, xm - t, xm + t, END_RAILS["upper"][0], END_RAILS["top"][1], -Z_POST[0], Z_POST[0], "beech-lacquered", "frame", 0.58))
    return out


def rails(scale):
    """📏️ The four side rails with their ledgers."""
    out = []
    for level in ("low", "up"):
        for side, z0, z1 in both(Z_RAIL):
            bone = f"rail-{side}-{level}"
            y0, y1 = Y_RAIL[level]
            lz = Z_LEDGER if side == "l" else (-Z_LEDGER[1], -Z_LEDGER[0])
            if scale in ("50", "100"):
                out.append(box(f"rail-{side}-{level}", bone, 0.057, 2.067, y0, y1, z0, z1, "beech-lacquered", "frame", 0.55, hatch="grain-h"))
                out.append(box(f"ledger-{side}-{level}", bone, 0.057, 2.067, Y_LEDGER[level][0], Y_LEDGER[level][1], lz[0], lz[1], "beech-raw", "frame", 0.6))
            elif scale == "200":
                zz = (lz[0], z1) if side == "l" else (z0, lz[1])
                out.append(box(f"rail-{side}-{level}", bone, 0.057, 2.067, y0, y1, zz[0], zz[1], "beech-lacquered", "frame", 0.55))
            else:
                zz = (0.423, 0.498) if side == "l" else (-0.498, -0.423)
                out.append(box(f"rail-{side}-{level}", bone, 0.057, 2.067, y0, y1, zz[0], zz[1], "beech-lacquered", "frame", 0.55))
    return out


def guards(scale):
    """🚧️ The two fall guards: two boards held by end battens."""
    out = []
    for side, z0, z1 in both(Z_GUARD):
        bone = f"guard-{side}"
        x0, x1 = X_GUARD[side]
        if scale == "500":
            out.append(box(f"guard-{side}", bone, x0, x1, Y_GUARD_LOW[0], Y_GUARD_UP[1], (0.423 if side == "l" else -0.498), (0.498 if side == "l" else -0.423), "beech-lacquered", "frame", 0.55))
            continue
        if scale == "200":
            out.append(box(f"guard-{side}", bone, x0, x1, Y_GUARD_LOW[0], Y_GUARD_UP[1], z0, z1, "beech-lacquered", "frame", 0.6 if side == "r" else 0.55))
            continue
        replaced = side == "r"
        grain = "grain-h" if scale in ("50", "100") else None
        out.append(box(f"guard-{side}-low", bone, x0, x1, Y_GUARD_LOW[0], Y_GUARD_LOW[1], z0, z1, "spruce-raw" if replaced else "beech-lacquered", "frame", 0.72 if replaced else 0.55, hatch=grain))
        out.append(box(f"guard-{side}-up", bone, x0, x1, Y_GUARD_UP[0], Y_GUARD_UP[1], z0, z1, "beech-lacquered", "frame", 0.55, hatch=grain))
        out.append(box(f"guard-{side}-batten-head", bone, x0, x0 + 0.04, Y_GUARD_LOW[1], Y_GUARD_UP[0], z0, z1, "beech-lacquered", "frame", 0.5))
        out.append(box(f"guard-{side}-batten-foot", bone, x1 - 0.04, x1, Y_GUARD_LOW[1], Y_GUARD_UP[0], z0, z1, "beech-lacquered", "frame", 0.5))
    return out


def ladder(scale):
    """🪜️ Stiles with rounded tops and the four flat rungs."""
    out = []
    for k, (x0, x1) in enumerate(X_STILES):
        name = "head" if k == 0 else "foot"
        if scale != "500":
            rad = (x1 - x0) / 2
            pts = [(x0, 0), (x1, 0)] + arc((x0 + x1) / 2, LADDER_H - rad, rad, 0, 180, {"50": 6, "100": 4, "200": 2}[scale])
        else:
            pts = rect(x0, x1, 0, LADDER_H)
        out.append(piece(f"stile-{name}", "ladder", pts, Z_LADDER[0], Z_LADDER[1], "beech-lacquered", "frame", 0.5))
    if scale != "500":
        t = RUNG_T if scale in ("50", "100") else 0.030
        for k, top in enumerate(RUNG_TOPS):
            out.append(box(f"rung-{k + 1}", "ladder", X_STILES[0][1], X_STILES[1][0], top - t, top, Z_LADDER[0], Z_LADDER[1], "beech-lacquered", "frame", 0.7))
    return out


def slats(scale):
    """🪵️ The two roll-slat bases: slats on two webbing tapes, or a plate at the coarse scales."""
    out = []
    for level in ("low", "up"):
        bone = f"slats-{level}"
        yb = 0.203 if level == "low" else 1.123
        if scale == "50":
            out.append(box(f"slat-{level}", bone, SLAT_X0, SLAT_X0 + SLAT_W, yb + 0.002, yb + 0.020, -Z_SLAT, Z_SLAT, "beech-raw", "slats", 0.66, repeat=SLAT_N))
            for side, z0, z1 in [("l", 0.28, 0.32), ("r", -0.32, -0.28)]:
                out.append(box(f"tape-{level}-{side}", bone, SLAT_X0, SLAT_X0 + SLAT_SPAN, yb, yb + 0.002, z0, z1, "pp-webbing", "slats", 0.3, sheet=True))
        elif scale == "100":
            out.append(box(f"slat-{level}", bone, SLAT_X0, SLAT_X0 + TYPE_W, yb, yb + 0.020, -Z_SLAT, Z_SLAT, "beech-raw", "slats", 0.66, repeat=TYPE_N, pitch=TYPE_PITCH, envelope=True))
        else:
            out.append(box(f"slats-{level}", bone, SLAT_X0, SLAT_X0 + SLAT_SPAN, yb, yb + 0.020, -Z_SLAT, Z_SLAT, "beech-raw", "slats", 0.66, sheet=True))
    return out


def mattresses(scale):
    """🛌️ The two thin foam mattresses, the lying hollows modelled where the scale shows them."""
    out = []
    for level in ("low", "up"):
        y0, y1 = Y_MAT[level]
        region = f"mattress-{level}"
        if scale == "50":
            dip = (0.55, 1.45, 0.012 if level == "low" else 0.016)
            pts = rounded(X_MAT[0], X_MAT[1], y0, y1, 0.025, 4, dip)
        elif scale == "100":
            pts = rounded(X_MAT[0], X_MAT[1], y0, y1, 0.03, 2, (0.55, 1.45, 0.016) if level == "up" else None)
        elif scale == "200":
            pts = rounded(X_MAT[0], X_MAT[1], y0, y1, 0.03, 1)
        else:
            pts = rect(X_MAT[0], X_MAT[1], y0, y1)
        out.append(piece(f"mattress-{level}", region, pts, -Z_MAT, Z_MAT, "mattress-cover", region, 0.82 if level == "low" else 0.86))
    return out


def blanket(scale):
    """🟦️ The moving blanket in two halves about its fold."""
    return [
        box("blanket-head", "blanket", X_BLANKET[0], X_BLANKET[1], 0, T_BLANKET, -Z_BLANKET, Z_BLANKET, "felt-blanket", "blanket", 0.3, sheet=True, hatch="felt" if scale in ("50", "100") else None),
        box("blanket-foot", "blanket-fold", X_BLANKET[1], X_BLANKET[2], 0, T_BLANKET, -Z_BLANKET, Z_BLANKET, "felt-blanket", "blanket", 0.3, sheet=True, hatch="felt" if scale in ("50", "100") else None),
    ]


def straps(scale):
    """🎗️ The two lashing straps as closed loops around the frame bundle, each with its ratchet."""
    out = []
    for name, x0 in zip(("a", "b"), STRAP_X):
        bone = f"strap-{name}"
        x1 = x0 + STRAP_W
        y0, y1 = STRAP_Y
        zo = STRAP_Z + STRAP_T
        if scale == "500":
            out.append(box(f"strap-{name}-top", bone, x0, x1, y1 - STRAP_T, y1, -zo, zo, "pes-webbing", "straps", 0.42, slender=True))
            continue
        out.append(box(f"strap-{name}-bottom", bone, x0, x1, y0, y0 + STRAP_T, -zo, zo, "pes-webbing", "straps", 0.42, sheet=True))
        out.append(box(f"strap-{name}-top", bone, x0, x1, y1 - STRAP_T, y1, -zo, zo, "pes-webbing", "straps", 0.42, sheet=True))
        out.append(box(f"strap-{name}-left", bone, x0, x1, y0 + STRAP_T, y1 - STRAP_T, STRAP_Z, zo, "pes-webbing", "straps", 0.42, sheet=True))
        out.append(box(f"strap-{name}-right", bone, x0, x1, y0 + STRAP_T, y1 - STRAP_T, -zo, -STRAP_Z, "pes-webbing", "straps", 0.42, sheet=True))
        xc = (x0 + x1) / 2
        out.append(box(f"ratchet-{name}", bone, xc - 0.025, xc + 0.025, 0.50, 0.60, zo, zo + 0.03, "steel-zinc", "straps", 0.32))
    return out


def pieces(scale):
    """🧩️ Every prism of one scale."""
    return blanket(scale) + mattresses(scale) + slats(scale) + end_frame(scale, "head") + end_frame(scale, "foot") + rails(scale) + guards(scale) + ladder(scale) + straps(scale)


def copies(p):
    """🔁️ The world x offsets of the copies of a piece."""
    return [k * p.get("pitch", SLAT_PITCH) for k in range(p.get("repeat", 1))]


def profile(p, dx=0.0):
    """✏️ The bone-local profile of a piece (copy shifted by dx)."""
    return [list(local(p["bone"], x + dx, y)) for x, y in p["pts"]]


def body_solid(p):
    """🧊️ The body solid of a piece: its profile extruded over its lateral span."""
    bz = origin(p["bone"])[2]
    s = {"id": p["id"], "kind": "extrude", "bone": p["bone"], "material": p["mat"], "role": p.get("role", "body"), "tone": p["tone"], "profile": profile(p), "plane": "xy", "from": r(p["z0"] - bz), "to": r(p["z1"] - bz)}
    for flag in ("sheet", "slender", "envelope"):
        if p.get(flag):
            s[flag] = True
    if p.get("repeat", 1) > 1:
        s["repeat"] = {"count": p["repeat"], "translation": [r(p.get("pitch", SLAT_PITCH)), 0, 0]}
    return s


def shape_of(p, view, dx=0.0, suffix=""):
    """🖌️ The drawn shape of a piece (or one copy) in one view, placed as an extruded solid of its profile."""
    zc = (p["z0"] + p["z1"]) / 2
    xs = [x for x, _ in p["pts"]]
    ys = [y for _, y in p["pts"]]
    depth = {"elevation": 5 - zc, "front": 1 + (min(xs) + max(xs)) / 2 + dx, "plan": 1 + (min(ys) + max(ys)) / 2}[view]
    s = {"id": p["id"] + suffix, "bone": p["bone"], "role": p.get("role", "body"), "z": r(depth, 3), "filled": True, "tone": p["tone"], "kind": "poly", "points": profile(p, dx), "closed": True, "region": p["region"], "material": p["mat"]}
    if p.get("hatch"):
        s["hatch"] = p["hatch"]
    s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r(p["z0"]), "z1": r(p["z1"])}
    return s


def shapes(scale, view):
    """🗂️ The drawn shapes of one scale and view, the prisms first, then the strokes."""
    out = []
    for p in pieces(scale):
        n = p.get("repeat", 1)
        for k, dx in enumerate(copies(p)):
            out.append(shape_of(p, view, dx, f"-{k + 1:02d}" if n > 1 else ""))
    return out + strokes(scale, view)


def stroke(sid, bone, role, pts, z, closed=False, tone=0.3, region=None, pen=None):
    """〰️ An open or closed unfilled stroke in bone-local coordinates."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "kind": "poly", "points": [[r(a), r(b)] for a, b in pts], "closed": closed}
    if region:
        s["region"] = region
    if pen:
        s["penMm"] = pen
    return s


def circle(cx, cy, rad, n=12):
    """⭕️ A closed polygon circle."""
    return [(cx + rad * math.cos(2 * math.pi * k / n), cy + rad * math.sin(2 * math.pi * k / n)) for k in range(n)]


def strokes(scale, view):
    """🖍️ The traces of use drawn as strokes in the side elevation: crayon on the ladder-side rails, the worn sticker and the mattress zip seams (the parts that only translate between poses, so the strokes stay on their faces)."""
    out = []
    if scale not in ("50", "100"):
        return out
    fine = scale == "50"
    if view == "elevation":
        zig = [(0.40 + 0.02 * k, 0.035 + (0.04 if k % 2 else 0)) for k in range(12 if fine else 6)]
        out.append(stroke("crayon-zigzag", "rail-r-low", "texture", zig, 6.2, region="frame"))
        out.append(stroke("crayon-sun", "rail-r-low", "texture", circle(1.25, 0.055, 0.024), 6.2, closed=True, region="frame"))
        if fine:
            rays = []
            for k in range(8):
                a = 2 * math.pi * k / 8
                out.append(stroke(f"crayon-ray-{k + 1}", "rail-r-low", "texture", [(1.25 + 0.03 * math.cos(a), 0.055 + 0.03 * math.sin(a)), (1.25 + 0.045 * math.cos(a), 0.055 + 0.045 * math.sin(a))], 6.2, region="frame"))
            out.append(stroke("crayon-name", "rail-r-low", "texture", [(1.60, 0.04), (1.63, 0.075), (1.65, 0.04), (1.68, 0.07), (1.71, 0.045), (1.74, 0.075), (1.77, 0.04), (1.80, 0.06)], 6.2, region="frame"))
            for k in range(4):
                out.append(stroke(f"tally-{k + 1}", "rail-r-up", "texture", [(0.16 + 0.016 * k, 0.03), (0.16 + 0.016 * k, 0.08)], 6.2, region="frame"))
            out.append(stroke("tally-cross", "rail-r-up", "texture", [(0.15, 0.035), (0.225, 0.075)], 6.2, region="frame"))
            out.append({"id": "sticker-remnant", "bone": "rail-r-low", "role": "detail", "z": 6.25, "filled": True, "tone": 0.9, "kind": "poly", "points": [[0.95, 0.03], [1.01, 0.03], [1.01, 0.07], [0.985, 0.08], [0.95, 0.075]], "closed": True, "region": "frame"})
        for level in ("low", "up"):
            out.append(stroke(f"zip-{level}", f"mattress-{level}", "detail", [(0.03, 0.05), (1.97, 0.05)], 3.5, region=f"mattress-{level}"))
    return out


def volume_of(p):
    """📐️ The volume of a piece in cubic metres (shoelace area times span times copies)."""
    pts = p["pts"]
    a = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        a += x0 * y1 - x1 * y0
    return abs(a) / 2 * (p["z1"] - p["z0"]) * p.get("repeat", 1)


def centroid(p):
    """⚖️ The world rest centroid (x, y) of a piece and its copies."""
    pts = p["pts"]
    a = cx = cy = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    cx, cy = cx / (3 * a), cy / (3 * a)
    n = p.get("repeat", 1)
    return (cx + p.get("pitch", SLAT_PITCH) * (n - 1) / 2, cy)


def masses():
    """🏋️ Mass and centre of mass of every bone from the 1:50 prisms (volume times density)."""
    acc = {}
    for p in pieces("50"):
        m = volume_of(p) * DENSITY[p["mat"]]
        cx, cy = centroid(p)
        e = acc.setdefault(p["bone"], [0.0, 0.0, 0.0])
        e[0] += m
        e[1] += m * cx
        e[2] += m * cy
    return {b: (e[0], e[1] / e[0], e[2] / e[0]) for b, e in acc.items()}


def rig():
    """🦴️ The rig with bone masses at their real centres."""
    ms = masses()
    out = []
    for bid, parent, x, y, z, angle, length, joint, title in BONES:
        b = {"id": bid, "parent": parent, "title": title, "x": r(x), "y": r(y)}
        if z:
            b["z"] = r(z)
        b.update({"angleDeg": angle, "lengthM": r(length), "joint": joint})
        if bid in ms:
            kg, cx, cy = ms[bid]
            u, v = local(bid, cx, cy)
            b["mass"] = {"kg": r(kg, 3), "comX": r(u, 4), "comY": r(v, 4), "distribution": "point", "source": "authored-estimate", "reference": "wood-handbook-2021" if "mattress" not in bid else "iso-845"}
        out.append(b)
    return out


MATERIALS = [
    {"id": "beech-lacquered", "title": T("Solid beech, clear lacquer aged and yellowed", "Buche massiv, Klarlack gealtert und vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 700, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 10000000000.0, "strengthPa": 35000000.0, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "beech-raw", "title": T("Beech, unfinished (slats, ledgers)", "Buche, unbehandelt (Leisten, Auflageleisten)"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 700, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 10000000000.0, "strengthPa": 35000000.0, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "spruce-raw", "title": T("Spruce board, unfinished (replacement guard board)", "Fichtenbrett, unbehandelt (Ersatzbrett der Absturzsicherung)"), "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "literature", "reference": "en-338-2016"},
    {"id": "pu-foam", "title": T("Polyurethane foam RG 30, worn", "Polyurethan-Schaum RG 30, durchgelegen"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 30, "conductivityWmK": 0.04, "specificHeatJKgK": 1400, "elasticModulusPa": 8000.0, "strengthPa": 4000.0, "source": "authored-estimate", "reference": "iso-845"},
    {"id": "mattress-cover", "title": T("Mattress: knitted polyester cover over the foam core (effective)", "Matratze: Polyester-Strickbezug über dem Schaumkern (effektiv)"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 36, "conductivityWmK": 0.04, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "iso-845"},
    {"id": "felt-blanket", "title": T("Moving blanket, recycled cotton felt, two layers", "Möbeldecke, Recycling-Baumwollvlies, zweilagig"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 125, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "pes-webbing", "title": T("Polyester lashing webbing 35 mm", "Polyester-Zurrgurtband 35 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1100, "conductivityWmK": 0.2, "specificHeatJKgK": 1100, "elasticModulusPa": 3000000000.0, "strengthPa": 300000000.0, "source": "authored-estimate", "reference": "en-12195-2"},
    {"id": "pp-webbing", "title": T("Polypropylene tape of the roll-slat base", "Polypropylenband des Rollrosts"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 900, "conductivityWmK": 0.22, "specificHeatJKgK": 1800, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "steel-zinc", "title": T("Steel, zinc plated (ratchets, bed bolts)", "Stahl, verzinkt (Ratschen, Bettbolzen)"), "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "literature", "reference": "iso-10456-2007"},
]
for m in MATERIALS:
    DENSITY[m["id"]] = m["densityKgM3"]
DENSITY["mattress-cover"] = 36


REFERENCES = [
    {"id": "en-747-1-2012", "citation": "EN 747-1:2012+A1:2015 Furniture - Bunk beds and high beds - Part 1: Safety, strength and durability requirements (safety barrier top at least 260 mm above the upper surface of the bed base and at least 160 mm above the top of the mattress, access opening 300 to 400 mm, distance between ladder treads 200 to 300 mm and constant, ladder width at least 300 mm, bunk beds for children from six years on the upper bed)."},
    {"id": "en-747-2-2012", "citation": "EN 747-2:2012+A1:2015 Furniture - Bunk beds and high beds - Part 2: Test methods (static loads on the safety barrier, the bed base and the ladder treads; durability of the base)."},
    {"id": "en-1725", "citation": "EN 1725:1998 Domestic furniture - Beds and mattresses - Safety requirements and test methods (static load tests of bed bases)."},
    {"id": "en-1334-1996", "citation": "EN 1334:1996 Domestic furniture - Beds and mattresses - Methods of measurement and recommended tolerances (nominal mattress size 900 x 2000 mm; the clear length and width of the bed exceed the mattress by 10 to 20 mm)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: General actions - densities, self-weight, imposed loads for buildings."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (D35: f_m,k 35 MPa, E_0,mean 10 GPa; C24: f_m,k 24 MPa, E_0,mean 11 GPa, rho_mean 420 kg/m3); EN 1912 assigns European beech LS10 to D35."},
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service (European beech about 0.69 to 0.72 g/cm3 at 12 % moisture; spruce about 0.45 g/cm3)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (timber 700 kg/m3: 0.18 W/mK, 1600 J/kgK; timber 450-500 kg/m3: 0.12-0.13 W/mK; flexible foams, textiles, steel 50 W/mK)."},
    {"id": "iso-845", "citation": "ISO 845:2006 Cellular plastics and rubbers - Determination of apparent density (flexible PU mattress foams RG 25 to RG 40 have 25 to 40 kg/m3; a 900 x 2000 x 100 mm RG 30 core weighs about 5.4 kg, with cover about 6.5 kg)."},
    {"id": "iso-3386-1", "citation": "ISO 3386-1:1986 Polymeric materials, cellular flexible - Determination of stress-strain characteristics in compression (compression stress at 40 % strain of 3 to 5 kPa for RG 30 mattress foam; after years of use the lying zone softens and keeps a hollow of 10 to 20 mm)."},
    {"id": "en-12195-2", "citation": "EN 12195-2:2000 Load restraint assemblies on road vehicles - Safety - Part 2: Web lashing made from man-made fibres (light 25 to 35 mm polyester lashing straps with ratchet, about 0.4 kg each)."},
    {"id": "iso-11228-1", "citation": "ISO 11228-1:2021 Ergonomics - Manual handling - Part 1: Lifting, lowering and carrying (reference mass 25 kg per person under ideal conditions; two-person carrying of a long load at hand height)."},
    {"id": "kiggs-2013", "citation": "Robert Koch-Institut (2013). Referenzperzentile für anthropometrische Maßzahlen und Blutdruck aus der Studie zur Gesundheit von Kindern und Jugendlichen in Deutschland (KiGGS) 2003-2006, 2nd ed. (median body mass about 19 kg at 5 years and 26 kg at 8 years, body length about 1.11 m and 1.29 m)."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment (clothed body surface temperatures around 30 to 33 degC in comfort conditions; 32 degC used for the contact of a lying child)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of lacquered wood and textiles about 0.9, of zinc-plated steel 0.2-0.3; solar absorptance of light wood 0.35-0.5)."},
    {"id": "used-bunk-bed-catalogue", "citation": "Catalogue-typical data of solid beech children's bunk beds for 90 x 200 cm mattresses sold in Germany since the 1990s (types, not brands): posts 57 x 57 mm, overall about 210 x 103 x 160 cm, side rails about 28 x 110 mm on bed bolts, roll-slat bases of 13 to 17 beech slats on two tapes, fall guard of two boards on the upper bed, vertical ladder hooked onto the side rail, divisible into two single beds; shipping mass 80 to 110 kg; second-hand offers in reuse depots dismantled and strapped (authored-estimate)."},
    {"id": "semio-used-household", "citation": "Semio staffage content, ticket USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION (2026-11-22): traces of use, storage state and carrying are authored from typical flat clearances and second-hand department stores; parameters tagged authored-estimate are not taken from a datasheet."},
]


def regions():
    """🌡️ The thermal regions with areas from the 1:50 model."""
    def layer(lid, title, kind, mat, t, lam, rho, cp, src="literature", ref="iso-10456-2007"):
        """🧶️ One region layer from thickness, conductivity, density and heat capacity."""
        return {"id": lid, "title": title, "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": r(t / lam, 6), "heatCapacityJM2K": r(rho * cp * t, 1), "source": src, "reference": ref}
    frame = layer("beech-member", T("Lacquered beech, half of 28 mm", "Lackierte Buche, Hälfte von 28 mm"), "timber", "beech-lacquered", 0.014, 0.18, 700, 1600)
    slat = layer("beech-slat", T("Beech slat, half of 18 mm", "Buchenleiste, Hälfte von 18 mm"), "timber", "beech-raw", 0.009, 0.18, 700, 1600)
    foam = lambda lvl: layer(f"foam-{lvl}", T("Foam core with cover, half of 100 mm", "Schaumkern mit Bezug, Hälfte von 100 mm"), "insulation", "mattress-cover", 0.05, 0.04, 36, 1400, "authored-estimate", "iso-845")
    felt = layer("felt", T("Two layers of felt, 8 mm", "Zwei Lagen Vlies, 8 mm"), "clothing", "felt-blanket", 0.008, 0.045, 125, 1300, "authored-estimate", "iso-10456-2007")
    web = layer("webbing", T("Polyester webbing 1.2 mm", "Polyestergurtband 1,2 mm"), "polymer", "pes-webbing", 0.0012, 0.2, 1100, 1100, "authored-estimate", "en-12195-2")
    def reg(rid, title, mat, area, lc, geom, svf, lay, **kw):
        """🔥️ One passive surface region."""
        d = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": mat, "areaM2": area, "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"]}
        d.update(kw)
        d.update({"source": "authored-estimate", "reference": "iso-10456-2007"})
        return d
    return [
        reg("frame", T("Lacquered beech frame: end frames, side rails, guards and ladder", "Lackiertes Buchengestell: Kopf- und Fußteil, Seitenholme, Absturzsicherungen und Leiter"), "beech-lacquered", AREA["frame"], 0.11, "vertical-plate", 0.5, frame, ratedPowerW=0, ratedPowerBasis="passive"),
        reg("slats", T("Roll-slat bases", "Rollroste"), "beech-raw", AREA["slats"], 0.068, "horizontal-plate-up", 0.3, slat),
        reg("mattress-low", T("Lower mattress cover", "Bezug der unteren Matratze"), "mattress-cover", AREA["mattress-low"], 0.9, "horizontal-plate-up", 0.6, foam("low")),
        reg("mattress-up", T("Upper mattress cover", "Bezug der oberen Matratze"), "mattress-cover", AREA["mattress-up"], 0.9, "horizontal-plate-up", 0.7, foam("up")),
        reg("blanket", T("Moving blanket", "Möbeldecke"), "felt-blanket", AREA["blanket"], 1.2, "horizontal-plate-up", 0.8, felt),
        reg("straps", T("Lashing straps and ratchets", "Zurrgurte und Ratschen"), "pes-webbing", AREA["straps"], 0.035, "vertical-plate", 0.5, web),
    ]


AREA = {"frame": 8.15, "slats": 3.39, "mattress-low": 4.4, "mattress-up": 4.4, "blanket": 5.5, "straps": 0.485}
REGION_IDS = ["frame", "slats", "mattress-low", "mattress-up", "blanket", "straps"]


def binding(exposed, contact=None):
    """🌬️ The region binding of a pose: exposed share per region, contacts of lying children."""
    out = {}
    for rid in REGION_IDS:
        b = {"exposedFraction": exposed[rid], "sunlitFraction": 0}
        if contact and rid in contact:
            b["contact"] = {"fraction": contact[rid], "tempC": 32}
        out[rid] = b
    return {"regions": out, "emitters": {}}


ANCHORS = [
    {"id": "blanket-head-l", "bone": "blanket", "x": 0.2, "y": 0, "z": 0.42, "kind": "contact", "title": T("Blanket on the floor, head end left", "Decke auf dem Boden, Kopfende links"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.4, "footprintWidthM": 0.36, "frictionCoefficient": 0.4},
    {"id": "blanket-head-r", "bone": "blanket", "x": 0.2, "y": 0, "z": -0.42, "kind": "contact", "title": T("Blanket on the floor, head end right", "Decke auf dem Boden, Kopfende rechts"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.4, "footprintWidthM": 0.36, "frictionCoefficient": 0.4},
    {"id": "blanket-foot-l", "bone": "blanket-fold", "x": 0.95, "y": -T_BLANKET, "z": 0.42, "kind": "contact", "title": T("Blanket on the floor, foot end left", "Decke auf dem Boden, Fußende links"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.4, "footprintWidthM": 0.36, "frictionCoefficient": 0.4},
    {"id": "blanket-foot-r", "bone": "blanket-fold", "x": 0.95, "y": -T_BLANKET, "z": -0.42, "kind": "contact", "title": T("Blanket on the floor, foot end right", "Decke auf dem Boden, Fußende rechts"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.4, "footprintWidthM": 0.36, "frictionCoefficient": 0.4},
    {"id": "grip-head", "bone": "bundle-lift", "x": 0.30, "y": 0.2492, "kind": "contact", "title": T("Hands of the head-end mover under the bundle", "Hände des Trägers am Kopfende unter dem Bündel"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.1, "footprintWidthM": 0.9, "frictionCoefficient": 0.6},
    {"id": "grip-foot", "bone": "bundle-lift", "x": 1.82, "y": 0.2492, "kind": "contact", "title": T("Hands of the foot-end mover under the bundle", "Hände des Trägers am Fußende unter dem Bündel"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.1, "footprintWidthM": 0.9, "frictionCoefficient": 0.6},
    {"id": "head-post-l", "bone": "head-frame", "x": 0, "y": -0.0285, "z": 0.4855, "kind": "contact", "title": T("Head post, left, on the floor", "Kopfpfosten links auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.057, "footprintWidthM": 0.057, "frictionCoefficient": 0.4},
    {"id": "head-post-r", "bone": "head-frame", "x": 0, "y": -0.0285, "z": -0.4855, "kind": "contact", "title": T("Head post, right, on the floor", "Kopfpfosten rechts auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.057, "footprintWidthM": 0.057, "frictionCoefficient": 0.4},
    {"id": "foot-post-l", "bone": "foot-frame", "x": 0, "y": 0.0285, "z": 0.4855, "kind": "contact", "title": T("Foot post, left, on the floor", "Fußpfosten links auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.057, "footprintWidthM": 0.057, "frictionCoefficient": 0.4},
    {"id": "foot-post-r", "bone": "foot-frame", "x": 0, "y": 0.0285, "z": -0.4855, "kind": "contact", "title": T("Foot post, right, on the floor", "Fußpfosten rechts auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.057, "footprintWidthM": 0.057, "frictionCoefficient": 0.4},
    {"id": "ladder-foot-head", "bone": "ladder", "x": 0, "y": -0.019, "z": -0.3075, "kind": "contact", "title": T("Ladder stile on the floor, head side", "Leiterholm auf dem Boden, kopfseitig"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.038, "footprintWidthM": 0.057, "frictionCoefficient": 0.4},
    {"id": "ladder-foot-foot", "bone": "ladder", "x": 0, "y": -0.419, "z": -0.3075, "kind": "contact", "title": T("Ladder stile on the floor, foot side", "Leiterholm auf dem Boden, fußseitig"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.038, "footprintWidthM": 0.057, "frictionCoefficient": 0.4},
    {"id": "rung-1", "bone": "ladder", "x": RUNG_TOPS[0], "y": -0.219, "z": -0.3075, "kind": "load", "title": T("First rung", "Erste Sprosse")},
    {"id": "rung-3", "bone": "ladder", "x": RUNG_TOPS[2], "y": -0.219, "z": -0.3075, "kind": "load", "title": T("Third rung", "Dritte Sprosse")},
    {"id": "guard-grip", "bone": "guard-r", "x": 1.45, "y": 0.225, "kind": "load", "title": T("Hand on the top board of the right guard", "Hand am oberen Brett der rechten Absturzsicherung")},
]

POST_CONTACTS = ["head-post-l", "head-post-r", "foot-post-l", "foot-post-r", "ladder-foot-head", "ladder-foot-foot"]
BLANKET_CONTACTS = ["blanket-head-l", "blanket-head-r", "blanket-foot-l", "blanket-foot-r"]


def left_behind_kg():
    """🧺️ Mass that stays on the floor while the movers carry the frame bundle."""
    ms = masses()
    return sum(ms[b][0] for b in ("blanket", "blanket-fold", "mattress-low", "mattress-up", "slats-low", "slats-up"))


def aids_kg():
    """🧺️ Mass of the blanket and straps lying on the floor beside the set-up bed."""
    ms = masses()
    return sum(ms[b][0] for b in ("blanket", "blanket-fold", "strap-a", "strap-b"))


def child(level):
    """👧️ The distributed load of a child lying on one mattress (KiGGS median mass over its body length)."""
    kg, frm, to = (19.0, 0.40, 1.51) if level == "low" else (26.0, 0.30, 1.59)
    return {"id": f"child-{level}", "kind": "occupant", "title": T(f"Child lying on the {'lower' if level == 'low' else 'upper'} mattress ({kg:.0f} kg)", f"Kind liegt auf der {'unteren' if level == 'low' else 'oberen'} Matratze ({kg:.0f} kg)"), "distributed": {"bone": f"mattress-{level}", "fromM": frm, "toM": to, "wxNPerM": 0, "wyNPerM": r(-kg * G / (to - frm), 2)}, "fxN": 0, "fyN": 0, "source": "literature", "reference": "kiggs-2013"}


def floor_load(lid, bone, x, kg, en, de):
    """🪨️ The floor reaction under parts that lie on the floor while the rest of the asset is carried or stands on its own feet."""
    return {"id": lid, "kind": "imposed", "title": T(en, de), "bone": bone, "x": x, "y": 0, "fxN": 0, "fyN": r(kg * G, 1), "source": "authored-estimate", "reference": "semio-used-household"}


def poses():
    """🧍️ Stored, carried, set up and in use, with the key poses of the two clips."""
    stored_exp = {"frame": 0.85, "slats": 0.45, "mattress-low": 0.6, "mattress-up": 0.55, "blanket": 0.35, "straps": 1}
    carried_exp = {"frame": 0.9, "slats": 0.6, "mattress-low": 0.6, "mattress-up": 0.55, "blanket": 0.35, "straps": 1}
    setup_exp = {"frame": 1, "slats": 0.15, "mattress-low": 0.75, "mattress-up": 0.75, "blanket": 0.55, "straps": 1}
    lift = {**STORED, **LIFT}
    half = {**STORED, "bundle-lift": {"dy": 0.25}}
    floor_left = floor_load("floor-under-left-parts", "blanket", 1.15, left_behind_kg(), "Floor reaction under the blanket, mattresses and slat bases left behind", "Bodenreaktion unter Decke, Matratzen und Rollrosten, die liegen bleiben")
    floor_aids = floor_load("floor-under-aids", "blanket", 0.575, aids_kg(), "Floor reaction under the folded blanket and the straps beside the bed", "Bodenreaktion unter der gefalteten Decke und den Gurten neben dem Bett")
    climber1 = {"id": "climber-rung-1", "kind": "occupant", "title": T("Child (26 kg) standing on the first rung", "Kind (26 kg) steht auf der ersten Sprosse"), "anchor": "rung-1", "fxN": 0, "fyN": r(-26 * G, 1), "source": "literature", "reference": "kiggs-2013"}
    climber3 = {"id": "climber-rung-3", "kind": "occupant", "title": T("Child on the third rung (70 % of 26 kg)", "Kind auf der dritten Sprosse (70 % von 26 kg)"), "anchor": "rung-3", "fxN": 0, "fyN": r(-0.7 * 26 * G, 1), "source": "literature", "reference": "kiggs-2013"}
    grip = {"id": "climber-hand", "kind": "occupant", "title": T("The child's hand pulls on the guard (30 % of 26 kg)", "Hand des Kindes zieht an der Absturzsicherung (30 % von 26 kg)"), "anchor": "guard-grip", "fxN": 0, "fyN": r(-0.3 * 26 * G, 1), "source": "literature", "reference": "kiggs-2013"}
    def pose(pid, title, desc, bones, contacts, exp, loads, key=False, ground="auto", contact=None):
        """📥️ One pose."""
        p = {"id": pid, "title": title, "description": desc}
        if key:
            p["key"] = True
        p.update({"bones": bones, "ground": ground})
        if ground == "authored":
            p["root"] = {"x": 0, "y": 0, "angleDeg": 0}
        p.update({"contacts": contacts, "thermal": binding(exp, contact), "loads": loads})
        return p
    return [
        pose("stored", T("Dismantled and strapped on the moving blanket", "Zerlegt und verzurrt auf der Möbeldecke"), T("As the reuse depot keeps it: the two mattresses and the roll-slat bases lie on the moving blanket, the head and foot frames flat on them, the side rails and fall guards on edge over the posts, the ladder flat in the middle, two lashing straps around the frame bundle.", "So lagert es das Gebrauchtwarenlager: die beiden Matratzen und die Rollroste liegen auf der Möbeldecke, Kopf- und Fußteil flach darauf, Seitenholme und Absturzsicherungen hochkant über den Pfosten, die Leiter flach in der Mitte, zwei Zurrgurte um das Rahmenbündel."), STORED, BLANKET_CONTACTS, stored_exp, []),
        pose("carry-grip-key", T("Movers take hold (key)", "Träger fassen an (Schlüssel)"), T("Both movers have their hands under the strapped frame bundle.", "Beide Träger haben die Hände unter dem verzurrten Rahmenbündel."), STORED, BLANKET_CONTACTS + ["grip-head", "grip-foot"], stored_exp, [floor_left], key=True),
        pose("carry-lift-key", T("Bundle half lifted (key)", "Bündel halb angehoben (Schlüssel)"), T("The frame bundle is 0.25 m above the parts left on the blanket.", "Das Rahmenbündel ist 0,25 m über den Teilen, die auf der Decke bleiben."), half, ["grip-head", "grip-foot"], carried_exp, [floor_left], key=True, ground="authored"),
        pose("carried", T("Frame bundle carried by two movers", "Rahmenbündel von zwei Trägern getragen"), T("Two movers carry the strapped frame bundle (end frames, rails, guards, ladder) level at knuckle height; the mattresses, the slat bases and the blanket stay behind for the second trip.", "Zwei Träger tragen das verzurrte Rahmenbündel (Kopf- und Fußteil, Holme, Absturzsicherungen, Leiter) waagerecht auf Handhöhe; Matratzen, Rollroste und Decke bleiben für den zweiten Gang liegen."), lift, ["grip-head", "grip-foot"], carried_exp, [floor_left], ground="authored"),
        pose("set-up", T("Set up again, empty", "Wieder aufgebaut, leer"), T("Assembled in a children's room: both slat bases on their ledgers, the mattresses on them, the ladder hooked onto the right side, the folded blanket with the two straps on the floor at the foot end.", "Aufgebaut in einem Kinderzimmer: beide Rollroste auf ihren Auflageleisten, die Matratzen darauf, die Leiter rechts eingehängt, die gefaltete Decke mit den beiden Gurten am Fußende auf dem Boden."), SETUP, POST_CONTACTS, setup_exp, [floor_aids]),
        pose("bedtime-key", T("Younger child in the lower bed (key)", "Jüngeres Kind im unteren Bett (Schlüssel)"), T("The five-year-old lies in the lower bed; the older child stands at the ladder.", "Das fünfjährige Kind liegt im unteren Bett; das ältere steht an der Leiter."), SETUP, POST_CONTACTS, setup_exp, [floor_aids, child("low")], key=True, contact={"mattress-low": 0.12}),
        pose("climb-rung-1-key", T("Older child on the first rung (key)", "Älteres Kind auf der ersten Sprosse (Schlüssel)"), T("The eight-year-old stands with both feet on the first rung.", "Das achtjährige Kind steht mit beiden Füßen auf der ersten Sprosse."), SETUP, POST_CONTACTS, setup_exp, [floor_aids, child("low"), climber1], key=True, contact={"mattress-low": 0.12}),
        pose("climb-rung-3-key", T("Older child on the third rung (key)", "Älteres Kind auf der dritten Sprosse (Schlüssel)"), T("Feet on the third rung, one hand on the top board of the guard.", "Füße auf der dritten Sprosse, eine Hand am oberen Brett der Absturzsicherung."), SETUP, POST_CONTACTS, setup_exp, [floor_aids, child("low"), climber3, grip], key=True, contact={"mattress-low": 0.12}),
        pose("in-use", T("In use: two children asleep", "In Gebrauch: zwei Kinder schlafen"), T("A five-year-old (19 kg) in the lower bed and an eight-year-old (26 kg) in the upper bed lie in the hollows their predecessors left in the mattresses.", "Ein fünfjähriges Kind (19 kg) im unteren und ein achtjähriges (26 kg) im oberen Bett liegen in den Mulden, die ihre Vorgänger in den Matratzen hinterlassen haben."), SETUP, POST_CONTACTS, setup_exp, [floor_aids, child("low"), child("up")], contact={"mattress-low": 0.12, "mattress-up": 0.14}),
    ]


def clips():
    """🎞️ The carry clip and the bedtime clip."""
    carry_contacts = [{"anchor": a, "fromS": 0, "toS": 1.5} for a in BLANKET_CONTACTS] + [{"anchor": a, "fromS": 1.0, "toS": 4.0} for a in ("grip-head", "grip-foot")]
    climb_contacts = [{"anchor": a, "fromS": 0, "toS": 8.0} for a in POST_CONTACTS]
    return [
        {"id": "carry", "title": T("Carry the frame bundle away", "Rahmenbündel forttragen"), "description": T("The movers take hold under the strapped frame bundle, lift it off the mattresses and slat bases and carry it at knuckle height; the blanket stays on the floor.", "Die Träger fassen unter das verzurrte Rahmenbündel, heben es von Matratzen und Rollrosten ab und tragen es auf Handhöhe; die Decke bleibt auf dem Boden."), "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "carry-grip-key", "ease": "smooth"}, {"t": 2.4, "pose": "carry-lift-key", "ease": "smooth"}, {"t": 4.0, "pose": "carried", "ease": "smooth"}],
         "contacts": carry_contacts, "phases": [{"id": "take-hold", "fromS": 0, "toS": 1.2, "title": T("Take hold", "Anfassen")}, {"id": "lift", "fromS": 1.2, "toS": 4.0, "title": T("Lift and carry", "Anheben und tragen")}],
         "events": [{"t": 1.0, "kind": "contact-on", "title": T("Hands under the bundle", "Hände unter dem Bündel"), "target": "grip-head"}, {"t": 1.5, "kind": "load-on", "title": T("The bundle leaves the blanket", "Das Bündel verlässt die Decke"), "target": "floor-under-left-parts"}, {"t": 1.5, "kind": "contact-off", "title": T("The movers carry the bundle", "Die Träger tragen das Bündel"), "target": "blanket-head-l"}],
         "tracks": [{"target": "load:floor-under-left-parts", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 1.45, "v": 0}, {"t": 1.5, "v": 1}, {"t": 4.0, "v": 1}]}]},
        {"id": "bedtime-climb", "title": T("Bedtime: climbing into the upper bed", "Schlafenszeit: ins obere Bett klettern"), "description": T("The older child climbs the ladder rung by rung, holds the guard and lies down in the upper bed while the younger one already lies below.", "Das ältere Kind steigt Sprosse für Sprosse die Leiter hinauf, hält sich an der Absturzsicherung und legt sich ins obere Bett, während das jüngere schon unten liegt."), "durationS": 8.0, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "bedtime-key", "ease": "smooth"}, {"t": 2.5, "pose": "climb-rung-1-key", "ease": "smooth"}, {"t": 5.0, "pose": "climb-rung-3-key", "ease": "smooth"}, {"t": 8.0, "pose": "in-use", "ease": "smooth"}],
         "contacts": climb_contacts, "phases": [{"id": "climb", "fromS": 0, "toS": 5.0, "title": T("Climb", "Hinaufsteigen")}, {"id": "lie-down", "fromS": 5.0, "toS": 8.0, "title": T("Lie down", "Hinlegen")}],
         "events": [{"t": 2.5, "kind": "load-on", "title": T("Feet on the first rung", "Füße auf der ersten Sprosse"), "target": "climber-rung-1"}, {"t": 5.0, "kind": "load-on", "title": T("Hand on the guard", "Hand an der Absturzsicherung"), "target": "climber-hand"}, {"t": 8.0, "kind": "thermal", "title": T("Upper mattress takes up the warmth of the child", "Obere Matratze nimmt die Wärme des Kindes auf"), "target": "child-up"}],
         "tracks": []},
    ]


def pens(scale):
    """✒️ Pens per scale."""
    return {"50": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, "100": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, "200": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, "500": {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]


def hatches(scale):
    """▤️ Hatches per scale."""
    if scale == "50":
        return [{"id": "grain-h", "title": T("Grain along the board", "Maserung längs des Bretts"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}, {"id": "felt", "title": T("Felt", "Vlies"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13}]
    if scale == "100":
        return [{"id": "grain-h", "title": T("Grain along the board", "Maserung längs des Bretts"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.13}, {"id": "felt", "title": T("Felt", "Vlies"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.7, "penMm": 0.13}]
    return []


VALID = {"50": (20, 75), "100": (75, 120), "200": (120, 350), "500": (350, 1000)}

REP_DECISIONS = {
    "50": {
        "silhouette": T("True orthographic views of the 1:50 model in every pose; stored the bundle is 46 x 17 mm in the side elevation: blanket, two mattresses with their worn hollows, two slat bases, the end frames flat, rails and guards on edge, the ladder flat, two straps with ratchets.", "Wahre orthogonale Ansichten des Modells 1:50 in jeder Pose; gelagert ist das Bündel in der Seitenansicht 46 x 17 mm: Decke, zwei Matratzen mit ihren Liegemulden, zwei Rollroste, Kopf- und Fußteil flach, Holme und Absturzsicherungen hochkant, die Leiter flach, zwei Gurte mit Ratschen."),
        "retained": T("Every slat of the roll-slat bases and their two tapes, ledgers, post chamfers and the chipped head post, the darkened post feet, rounded stile tops, four flat rungs, guard battens, the lying hollows (12 and 16 mm), straps 1.2 mm.", "Jede Leiste der Rollroste und ihre zwei Bänder, Auflageleisten, Pfostenfasen und der abgeplatzte Kopfpfosten, die dunklen Pfostenfüße, gerundete Holmköpfe, vier Flachsprossen, Leisten der Absturzsicherung, die Liegemulden (12 und 16 mm), Gurte 1,2 mm."),
        "merged": T("Bed bolts and their barrel nuts are in the rails; the ratchet is one block.", "Bettbolzen und Quermuttern stecken in den Holmen; die Ratsche ist ein Block."),
        "omitted": T("Ladder hooks (4 mm steel), dowels, bolt holes, the mattress zip pull.", "Leiterhaken (4 mm Stahl), Dübel, Bolzenlöcher, der Reißverschlussgriff der Matratze."),
        "thinElements": T("Straps (1.2 mm), blanket (8 mm) and slat tapes (2 mm) are declared sheets at their true thickness; nothing is thickened.", "Gurte (1,2 mm), Decke (8 mm) und Rostbänder (2 mm) sind erklärte Bahnen in wahrer Dicke; nichts ist verdickt."),
        "hatching": T("Grain lines along the side rails and guard boards at 0.6 mm, felt dots on the blanket at 0.8 mm.", "Maserungslinien längs der Seitenholme und Bretter im Abstand 0,6 mm, Vliespunkte auf der Decke im Abstand 0,8 mm."),
        "styles": T("Mattresses 0.82 and 0.86 (yellowed covers), rungs 0.7 (lacquer worn off), replacement spruce board 0.72, beech 0.5 to 0.62, post feet 0.4 (scuffed), blanket 0.3.", "Matratzen 0,82 und 0,86 (vergilbte Bezüge), Sprossen 0,7 (Lack abgetreten), Ersatzbrett aus Fichte 0,72, Buche 0,5 bis 0,62, Pfostenfüße 0,4 (abgestoßen), Decke 0,3."),
        "overlays": T("Traces of use as strokes: a crayon zigzag, a sun and a name on the lower right rail, tally marks on the upper right rail, a sticker remnant, the zip seams of the mattresses; stains and quilting are not drawn as strokes because the plan would paint them over the parts stacked above.", "Gebrauchsspuren als Striche: ein Wachsmalzickzack, eine Sonne und ein Name am unteren rechten Holm, Strichliste am oberen rechten Holm, ein Aufkleberrest, die Reißverschlussnähte der Matratzen; Flecken und Steppung sind nicht als Striche gezeichnet, weil der Grundriss sie über die darüber gestapelten Teile malen würde."),
    },
    "100": {
        "silhouette": T("The same object at 1:100: the stored bundle 23 x 8.5 mm; rails, ledgers, guards and every slat at true section.", "Dasselbe Objekt in 1:100: das gelagerte Bündel 23 x 8,5 mm; Holme, Auflageleisten, Absturzsicherungen und jede Leiste im wahren Querschnitt."),
        "retained": T("Fifteen slats per base, ledgers, rounded stile tops, four rungs, the upper lying hollow (16 mm), straps and ratchets.", "Fünfzehn Leisten je Rost, Auflageleisten, gerundete Holmköpfe, vier Sprossen, die obere Liegemulde (16 mm), Gurte und Ratschen."),
        "merged": T("Slats and their tapes are one 20 mm slat; post foot and shaft are one post.", "Leisten und ihre Bänder sind eine 20-mm-Leiste; Pfostenfuß und Schaft sind ein Pfosten."),
        "omitted": T("Post chamfers and the chip (6 mm), the 12 mm lower hollow, tally marks, the name and the sticker.", "Pfostenfasen und die Abplatzung (6 mm), die 12-mm-Mulde unten, Strichliste, Name und Aufkleber."),
        "thinElements": T("Blanket and straps stay declared sheets at true thickness.", "Decke und Gurte bleiben erklärte Bahnen in wahrer Dicke."),
        "hatching": T("Grain at 0.5 mm on rails and boards, felt dots at 0.7 mm.", "Maserung im Abstand 0,5 mm auf Holmen und Brettern, Vliespunkte im Abstand 0,7 mm."),
        "styles": T("As at 1:50.", "Wie in 1:50."),
        "overlays": T("Crayon zigzag and sun on the lower right rail, zip seams of the mattresses.", "Wachsmalzickzack und Sonne am unteren rechten Holm, Reißverschlussnähte der Matratzen."),
    },
    "200": {
        "silhouette": T("The bundle at 1:200, 11.5 x 4.2 mm: blanket, mattresses, slat bases as plates, frames, rails, guards, ladder, straps.", "Das Bündel in 1:200, 11,5 x 4,2 mm: Decke, Matratzen, Rollroste als Platten, Rahmen, Holme, Absturzsicherungen, Leiter, Gurte."),
        "retained": T("Posts, bottom and middle cross rails, the gaps of the guards (65 mm), four rungs.", "Pfosten, unterer und mittlerer Querriegel, die Lücken der Absturzsicherungen (65 mm), vier Sprossen."),
        "merged": T("Each slat base is one 20 mm plate; each rail with its ledger is one 50 mm member; the upper cross rail, infill and top rail of an end frame are one board.", "Jeder Rollrost ist eine 20-mm-Platte; jeder Holm mit seiner Auflageleiste ein 50-mm-Bauteil; oberer Querriegel, Füllung und oberer Riegel eines Endteils sind ein Brett."),
        "omitted": T("Rounded stile tops, lying hollows, mattress radii, all strokes of use.", "Gerundete Holmköpfe, Liegemulden, Matratzenradien, alle Gebrauchsspuren."),
        "thinElements": T("Slat plates, blanket and straps are declared sheets at true thickness; rungs are 30 mm.", "Rostplatten, Decke und Gurte sind erklärte Bahnen in wahrer Dicke; Sprossen sind 30 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as at 1:50.", "Töne wie in 1:50."),
        "overlays": T("None.", "Keine."),
    },
    "500": {
        "silhouette": T("The bundle at 1:500, 4.6 x 1.6 mm: blanket line, mattress block, the frames, rail and guard walls, the ladder stiles.", "Das Bündel in 1:500, 4,6 x 1,6 mm: Deckenlinie, Matratzenblock, die Rahmen, Holm- und Brüstungswände, die Leiterholme."),
        "retained": T("Posts, the bottom rail and the upper board of each end frame, the large opening between them, both stiles.", "Pfosten, der untere Riegel und das obere Brett jedes Endteils, die große Öffnung dazwischen, beide Holme."),
        "merged": T("Rails with ledgers and each guard are 75 mm members; the slat bases are plates; the middle cross rail joins the opening.", "Holme mit Auflageleisten und jede Absturzsicherung sind 75-mm-Bauteile; die Rollroste sind Platten; der mittlere Querriegel geht in der Öffnung auf."),
        "omitted": T("Rungs (28 mm), straps and ratchets (35 mm), every stroke.", "Sprossen (28 mm), Gurte und Ratschen (35 mm), jeder Strich."),
        "thinElements": T("Posts (57 mm) and end boards (28 mm) keep their true section; blanket and slat plates are sheets.", "Pfosten (57 mm) und Endbretter (28 mm) behalten ihren wahren Querschnitt; Decke und Rostplatten sind Bahnen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as at 1:50.", "Töne wie in 1:50."),
        "overlays": T("None.", "Keine."),
    },
}

VOL_DECISIONS = {
    "50": [T("Body for 7.5 mm: every part as the prism of its true section, the same prism that the three drawings extrude; fifteen slats per base as a repeat on their two 2 mm tapes; posts split into the scuffed foot and the shaft with its chamfers and the chip; mattresses with their lying hollows.", "Körper für 7,5 mm: jedes Teil als Prisma seines wahren Querschnitts, dasselbe Prisma, das die drei Zeichnungen extrudieren; fünfzehn Leisten je Rost als Wiederholung auf ihren zwei 2-mm-Bändern; Pfosten geteilt in den abgestoßenen Fuß und den Schaft mit Fasen und Abplatzung; Matratzen mit ihren Liegemulden."), T("Thermal: every body solid is a surface of its region (frame, slats, two mattresses, blanket, straps). Structure: every timber part, mattress, blanket half and strap run is the member of its bone; steel bed bolts are joints at the rail ends; the asset has no permanent support, its supports are the contacts of each pose.", "Thermik: jeder Körper ist Oberfläche seiner Region (Gestell, Roste, zwei Matratzen, Decke, Gurte). Tragwerk: jedes Holzteil, jede Matratze, jede Deckenhälfte und jedes Gurtstück ist Stab seines Knochens; die Bettbolzen aus Stahl sind Gelenke an den Holmenden; das Objekt hat kein dauerndes Auflager, seine Auflager sind die Kontakte jeder Pose.")],
    "100": [T("Body for 15 mm: slats keep their 68 x 18 mm section and absorb the tapes (20 mm); posts without chamfers; mattress radii 30 mm, only the 16 mm hollow of the upper one stays.", "Körper für 15 mm: Leisten behalten ihren Querschnitt 68 x 18 mm und nehmen die Bänder auf (20 mm); Pfosten ohne Fasen; Matratzenradien 30 mm, nur die 16-mm-Mulde der oberen bleibt."), T("Thermal and structure follow the body; bed bolts remain as joints.", "Thermik und Tragwerk folgen dem Körper; Bettbolzen bleiben als Gelenke.")],
    "200": [T("Body for 30 mm: slat bases are 20 mm plates (declared sheets), rails merge with their ledgers into 50 mm members, the upper part of each end frame is one board, rungs are 30 mm.", "Körper für 30 mm: Rollroste sind 20-mm-Platten (erklärte Bahnen), Holme verschmelzen mit ihren Auflageleisten zu 50-mm-Bauteilen, der obere Teil jedes Endteils ist ein Brett, Sprossen sind 30 mm."), T("Thermal and structure follow the body; bed bolts are left out.", "Thermik und Tragwerk folgen dem Körper; Bettbolzen entfallen.")],
    "500": [T("Body for 75 mm: rails and guards become 75 mm members, each end frame is two posts, the bottom rail and the upper board, the ladder two stiles; rungs and straps are below the feature and left out of the body.", "Körper für 75 mm: Holme und Absturzsicherungen werden 75-mm-Bauteile, jedes Endteil besteht aus zwei Pfosten, dem unteren Riegel und dem oberen Brett, die Leiter aus zwei Holmen; Sprossen und Gurte liegen unter dem Merkmal und entfallen im Körper."), T("Thermal: the straps keep their region as two slender top runs; structure: rungs stay as slender members of the ladder, straps carry no member at this scale.", "Thermik: die Gurte behalten ihre Region als zwei schlanke obere Stränge; Tragwerk: Sprossen bleiben als schlanke Stäbe der Leiter, Gurte tragen in diesem Maßstab keinen Stab.")],
}

STRUCTURE_BONES = {"blanket", "blanket-fold", "mattress-low", "mattress-up", "slats-low", "slats-up", "rail-l-low", "rail-r-low", "rail-l-up", "rail-r-up", "guard-l", "guard-r", "head-frame", "foot-frame", "ladder"}


def volume(scale):
    """🔧️ The authored volume of one scale: body prisms, their thermal surfaces and the structural idealisation."""
    ps = pieces(scale)
    body = [body_solid(p) for p in ps]
    thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in ps]
    structure = [{"id": f"st-{p['id']}", "of": p["id"], "member": p["bone"], "part": "member"} for p in ps if p["bone"] in STRUCTURE_BONES]
    if scale in ("50", "100"):
        for level in ("low", "up"):
            for side in ("l", "r"):
                bone = f"rail-{side}-{level}"
                bz = origin(bone)[2]
                zc = (Z_RAIL[0] + Z_RAIL[1]) / 2 * (1 if side == "l" else -1)
                for end, x in (("head", 0.057 + 0.012), ("foot", 2.067 - 0.012)):
                    u, v = local(bone, x, (Y_RAIL[level][0] + Y_RAIL[level][1]) / 2)
                    structure.append({"id": f"bolt-{side}-{level}-{end}", "kind": "box", "bone": bone, "material": "steel-zinc", "role": "detail", "tone": 0.3, "size": [0.024, 0.03, 0.02] if scale == "50" else [0.03, 0.03, 0.028], "transform": {"translation": [r(u), r(v), r(zc - bz)]}, "member": bone, "part": "joint"})
    if scale == "500":
        bx, by, bz = origin("ladder")
        for k, top in enumerate(RUNG_TOPS):
            u0, v0 = local("ladder", X_STILES[0][1], top - RUNG_T)
            u1, v1 = local("ladder", X_STILES[1][0], top)
            structure.append({"id": f"st-rung-{k + 1}", "kind": "box", "bone": "ladder", "material": "beech-lacquered", "role": "body", "tone": 0.7, "slender": True, "size": [r(abs(u1 - u0)), r(abs(v1 - v0)), r(Z_LADDER[1] - Z_LADDER[0])], "transform": {"translation": [r((u0 + u1) / 2), r((v0 + v1) / 2), r((Z_LADDER[0] + Z_LADDER[1]) / 2 - bz)]}, "member": "ladder", "part": "member"})
    return {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal, "structure": structure, "decisions": VOL_DECISIONS[scale]}


def representation(scale):
    """🗺️ The authored representation of one scale."""
    lo, hi = VALID[scale]
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pens(scale), "hatches": hatches(scale), "geometry": {v: shapes(scale, v) for v in ("elevation", "front", "plan")}, "decisions": REP_DECISIONS[scale]}


def dimensions():
    """📏️ Envelope of the stored bundle (the default pose) and the total mass."""
    total = sum(m[0] for m in masses().values())
    return {"heightM": r(STRAP_Y[1], 3), "widthM": r(X_BLANKET[2] - X_BLANKET[0], 3), "depthM": r(2 * Z_BLANKET, 3), "massKg": r(total, 1)}


def asset():
    """🏺️ The whole asset."""
    return {
        "id": "used-bunk-bed",
        "category": "furniture",
        "title": T("Used Children's Bunk Bed", "Gebrauchtes Etagenbett"),
        "summary": T("A used solid-beech bunk bed for two 90 x 200 cm mattresses, 2.12 x 1.06 x 1.60 m set up, with a vertical ladder, fall guards and two thin foam mattresses with lying hollows; crayon on the rails, a replaced spruce guard board, a chipped post. Stored dismantled and strapped on a moving blanket, carried as a frame bundle by two movers, set up again for two children.", "Ein gebrauchtes Etagenbett aus massiver Buche für zwei Matratzen 90 x 200 cm, aufgebaut 2,12 x 1,06 x 1,60 m, mit senkrechter Leiter, Absturzsicherungen und zwei dünnen Schaumstoffmatratzen mit Liegemulden; Wachsmalstifte an den Holmen, ein ersetztes Fichtenbrett, ein abgeplatzter Pfosten. Zerlegt und verzurrt auf einer Möbeldecke gelagert, als Rahmenbündel von zwei Trägern getragen und für zwei Kinder wieder aufgebaut."),
        "tags": ["used", "household", "second-hand", "children", "bedroom", "bed", "bunk-bed", "beech", "solid-wood", "mattress", "ladder", "dismantled", "moving-blanket", "interior"],
        "provenance": {"author": "Semio staffage content (used household objects)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(
            "Solid beech bunk bed of the common German type: four 57 x 57 mm posts 1.60 m, end frames with four 28 mm cross rails and an infill board, side rails 28 x 110 mm on bed bolts with 22 x 30 mm ledgers, roll-slat bases of fifteen 68 x 18 mm beech slats on two tapes, fall guards of two 20 x 80 mm boards (top 1.44 m, 297 mm above the base and 197 mm above the mattress, EN 747-1), vertical ladder 0.438 m wide with four flat rungs every 235 mm, 362 mm access opening. Mattresses 900 x 2000 x 100 mm RG 30 foam. Masses from the 1:50 prisms times density. Traces of use and storage state are authored-estimate.",
            "Massives Buchen-Etagenbett der verbreiteten deutschen Bauart: vier Pfosten 57 x 57 mm, 1,60 m, Endteile mit vier 28-mm-Querriegeln und Füllung, Seitenholme 28 x 110 mm auf Bettbolzen mit Auflageleisten 22 x 30 mm, Rollroste aus fünfzehn Buchenleisten 68 x 18 mm auf zwei Bändern, Absturzsicherungen aus zwei Brettern 20 x 80 mm (Oberkante 1,44 m, 297 mm über dem Rost und 197 mm über der Matratze, EN 747-1), senkrechte Leiter 0,438 m breit mit vier Flachsprossen im Abstand 235 mm, Einstieg 362 mm. Matratzen 900 x 2000 x 100 mm aus RG-30-Schaum. Massen aus den Prismen 1:50 mal Dichte. Gebrauchsspuren und Lagerzustand sind Schätzwerte.")},
        "dimensions": dimensions(),
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "ground", "bones": rig()},
        "anchors": ANCHORS,
        "representations": {s: representation(s) for s in SCALES},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {
            "environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
            "passengers": [], "operating": [], "regions": regions(), "emitters": [],
            "assumptions": [
                T("Reference case: an unheated reuse depot or a children's room at 18 degC, still air, no sun. All regions are passive; the bed draws no power (frame region rated 0 W, passive).", "Referenzfall: ein unbeheiztes Gebrauchtwarenlager oder ein Kinderzimmer bei 18 degC, ruhige Luft, keine Sonne. Alle Regionen sind passiv; das Bett nimmt keine Leistung auf (Region Gestell mit 0 W, passiv)."),
                T("Aged surfaces: the clear lacquer of the beech has yellowed (solar absorptance 0.50 instead of about 0.40 new), the mattress covers are yellowed (0.55 instead of 0.45); emissivities 0.9 stay (Incropera); authored-estimate.", "Gealterte Oberflächen: der Klarlack der Buche ist vergilbt (solarer Absorptionsgrad 0,50 statt etwa 0,40 neu), die Matratzenbezüge vergilbt (0,55 statt 0,45); die Emissionsgrade 0,9 bleiben (Incropera); Schätzwert."),
                T("Lying children cover 12 % (lower) and 14 % (upper) of the mattress surface at 32 degC (ISO 7730 clothed surface); the worn foam (RG 30, about 0.04 W/mK) insulates them from the slats.", "Liegende Kinder bedecken 12 % (unten) und 14 % (oben) der Matratzenoberfläche bei 32 degC (bekleidete Oberfläche nach ISO 7730); der durchgelegene Schaum (RG 30, etwa 0,04 W/mK) isoliert sie gegen die Leisten."),
            ],
        },
        "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
            T("The asset has no permanent support: stored it lies on the blanket (four floor contacts), carried it hangs in the hands of two movers (two contacts under the frame bundle), set up it stands on its four posts and the two ladder stiles.", "Das Objekt hat kein dauerndes Auflager: gelagert liegt es auf der Decke (vier Bodenkontakte), getragen hängt es in den Händen zweier Träger (zwei Kontakte unter dem Rahmenbündel), aufgebaut steht es auf seinen vier Pfosten und den beiden Leiterholmen."),
            T("The solver treats the asset as one rigid body. Where parts lie on the floor while the rest is carried (blanket, mattresses, slat bases) or while the bed stands on its feet (folded blanket and straps), the floor reaction under them is applied as an upward load at their centre, so the hands and the posts carry exactly the rest.", "Der Löser behandelt das Objekt als einen starren Körper. Wo Teile auf dem Boden liegen, während der Rest getragen wird (Decke, Matratzen, Rollroste), oder während das Bett auf seinen Füßen steht (gefaltete Decke und Gurte), wirkt die Bodenreaktion unter ihnen als aufwärts gerichtete Last in ihrem Schwerpunkt, so dass Hände und Pfosten genau den Rest tragen."),
            T("Carried frame bundle about 72 kg, 36 kg per mover: above the 25 kg reference mass of ISO 11228-1, acceptable only for trained movers over short distances. Members are beech D35 (EN 338, EN 1912) with their true sections; mattresses and blanket are soft members that only pass compression on.", "Getragenes Rahmenbündel etwa 72 kg, 36 kg je Träger: über der Referenzmasse von 25 kg nach ISO 11228-1, nur für geübte Träger auf kurzen Wegen vertretbar. Die Stäbe sind Buche D35 (EN 338, EN 1912) in wahrem Querschnitt; Matratzen und Decke sind weiche Stäbe, die nur Druck weitergeben."),
            T("Occupant loads: KiGGS medians, 19 kg (5 years) and 26 kg (8 years) spread over the lying length; climbing on the ladder 26 kg on the first rung, then 70 % on the third rung and 30 % on the guard. EN 747-2 test loads are not modelled.", "Personenlasten: KiGGS-Mediane, 19 kg (5 Jahre) und 26 kg (8 Jahre) über die Liegelänge verteilt; beim Steigen 26 kg auf der ersten Sprosse, dann 70 % auf der dritten Sprosse und 30 % an der Absturzsicherung. Prüflasten nach EN 747-2 sind nicht modelliert."),
        ], "clinical": False},
        "volumes": {s: volume(s) for s in SCALES},
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


def main():
    """🖨️ Writes the asset document next to this script."""
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    ms = masses()
    print("[DEBUG] masses", {b: round(m[0], 2) for b, m in ms.items()}, "total", round(sum(m[0] for m in ms.values()), 2), "left", round(left_behind_kg(), 2))


if __name__ == "__main__":
    main()
