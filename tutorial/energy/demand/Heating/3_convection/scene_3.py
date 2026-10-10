"""🔥 Heating Module 3 — Lüftung (ventilation heat loss).

One example house runs through every beat: its section gives the gross volume
V_e and, after the construction is subtracted, the net air volume V; that V,
the air change rate n, the volumetric heat capacity c_Luft and Δθ build
Φ_V = V · n · c_Luft · Δθ (DIN EN 12831-1). Formulas and units are typeset with
the shared ``math_*`` helpers, every shown quantity counts up from 0 and every
partial value is computed from the drawn geometry.
"""

import numpy as np
from manim import *

from pathlib import Path as _Path
import sys as _sys

_TUTORIAL_ROOT = next(
    p for p in _Path(__file__).resolve().parents
    if (p / "manim_fonts.py").is_file()
)
if str(_TUTORIAL_ROOT) not in _sys.path:
    _sys.path.insert(0, str(_TUTORIAL_ROOT))

from manim_fonts import (
    apply_scene_style, scene_title, play_scene_title,
    beat_subtitle, BEAT_SUBTITLE_FADE, body_text,
    SUBTITLE_FONT_SIZE, BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, PASTEL_WHITE, PASTEL_CYAN, PASTEL_TEAL, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_RED, PASTEL_BLUE, PASTEL_GREEN,
    highlight_param, math_label, math_readout, math_panel, de_num,
    smooth_path, flow_guides, flow_animation, dim_arrow,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language,
    house_section, room_section, window_glyph, open_window, radiator, person_glyph, seated_person_glyph,
    lamp_glyph, clock_glyph, thermometer_glyph, ripples,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 3: Lüftung"


#region Beat helpers
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in
    ``1_introduction/scene_1.py`` and ``2_conduction/scene_2.py`` so every module
    footnotes its norm identically. Module 3 is the ventilation-loss chain of
    DIN EN 12831-1 (Φ_V = V · n · c_Luft · Δθ); the systems beat cites DIN 1946-6.
    Beats whose diagram already prints the standard in full get no corner chip.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=PASTEL_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Moving air
def _fan(pos, color, radius: float = 0.18):
    """🌀 Ring plus three blades — marks a stream as fan-driven."""
    ring = Circle(radius=radius, color=color, stroke_width=2.2).set_fill(P_DEEP_DARK, 1.0)
    blades = VGroup(*[Line(ORIGIN, RIGHT * radius * 0.78, color=color, stroke_width=2.2).rotate(a, about_point=ORIGIN)
                      for a in (0.0, TAU / 3, 2 * TAU / 3)])
    return VGroup(ring, blades).move_to(pos)


def _spin(fans, rt: float, period: float = 0.9, sign: float = -1.0):
    """🔄 Turn the blades of every ``_fan`` for ``rt`` seconds."""
    return [Rotate(f[1], angle=sign * TAU * rt / period, about_point=f[0].get_center(), rate_func=linear) for f in fans]


def _streams(streams, rt: float, *, speed: float = 0.8, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


def _live_flow(streams, rt: float, *, speed: float = 0.8, waves: int = 4, radius: float = 0.065):
    """🎨 Particle streams coloured by ``color_fn(t)`` along the path every frame — supply air warms while η rises."""
    dots, meta = VGroup(), []
    for paths, color_fn in streams:
        for path in paths:
            for w in range(waves):
                dot = Ellipse(width=radius * 2.8, height=radius * 1.15, stroke_width=0, fill_opacity=0.0)
                dots.add(dot.move_to(path.point_from_proportion(0.0)))
                meta.append([path, w / waves, color_fn, 0.0])

    def update(group, alpha):
        for dot, m in zip(group, meta):
            path, offset, color_fn, angle = m
            t = (alpha * speed * rt + offset) % 1.0
            a, b = path.point_from_proportion(max(0.0, t - 0.02)), path.point_from_proportion(min(1.0, t + 0.02))
            m[3] = float(np.arctan2(b[1] - a[1], b[0] - a[0]))
            dot.rotate(m[3] - angle).move_to(path.point_from_proportion(t))
            dot.set_fill(color_fn(t), opacity=0.92 * max(0.0, min(1.0, t / 0.08, (1.0 - t) / 0.10)))

    return UpdateFromAlphaFunc(dots, update, remover=True, rate_func=linear)


def _fade_flow(flow):
    """🌫️ Thin a running particle stream out to nothing over its run time — play it after ``flow``."""
    return UpdateFromAlphaFunc(flow.mobject, lambda m, a: [d.set_fill(opacity=d.get_fill_opacity() * (1.0 - a)) for d in m],
                               rate_func=linear)
#endregion


#region Geometry helpers
def _shoelace(points) -> float:
    """📐 Area of a simple polygon from its corners.

    https://en.wikipedia.org/wiki/Shoelace_formula
    """
    if len(points) < 3:
        return 0.0
    p = np.array([np.array(q, dtype=float)[:2] for q in points])
    x, y = p[:, 0], p[:, 1]
    return 0.5 * abs(float(np.dot(x, np.roll(y, -1)) - np.dot(y, np.roll(x, -1))))


def _clip_below(points, y_cut: float) -> list:
    """✂️ Part of a polygon below the line ``y = y_cut``.

    https://en.wikipedia.org/wiki/Sutherland%E2%80%93Hodgman_algorithm
    """
    pts = [np.array(p, dtype=float)[:2] for p in points]
    out = []
    for a, b in zip(pts, pts[1:] + pts[:1]):
        a_in, b_in = a[1] <= y_cut, b[1] <= y_cut
        if a_in:
            out.append(a)
        if a_in != b_in:
            out.append(a + (y_cut - a[1]) / (b[1] - a[1]) * (b - a))
    return out


def _clip_above(points, y_cut: float) -> list:
    """🪞 Part of a polygon above the line ``y = y_cut`` — ``_clip_below`` mirrored."""
    flipped = _clip_below([(float(p[0]), -float(p[1])) for p in points], -y_cut)
    return [np.array([p[0], -p[1]]) for p in flipped]


def _level_for(polys, frac: float) -> float:
    """🌊 Height of the fill line below which ``frac`` of the polygons' area lies (bisection)."""
    ys = [float(np.array(p, dtype=float)[1]) for poly in polys for p in poly]
    lo, hi = min(ys), max(ys)
    total = sum(_shoelace(p) for p in polys)
    for _ in range(40):
        mid = 0.5 * (lo + hi)
        if sum(_shoelace(_clip_below(p, mid)) for p in polys) < frac * total:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def _to_scene(points, origin, scale: float) -> list:
    """🗺️ Metre coordinates of a drawing → scene points."""
    o = np.array(origin, dtype=float)
    return [o + scale * np.array([float(p[0]), float(p[1]), 0.0]) for p in points]


def _poly(points, origin, scale: float, **style):
    """🔷 Polygon from metre coordinates; an empty mobject while it has no area yet."""
    if len(points) < 3:
        return VMobject()
    return Polygon(*_to_scene(points, origin, scale), **style)


def _house_section(wall: float = 0.35, slab: float = 0.30, decke: float = 0.22, roof: float = 0.30):
    """🏠 The example house in section — outer outline, construction pieces and the two air rooms, in metres.

    Footprint 10 m × 8 m, eaves 6 m, ridge 8,5 m, Geschossdecke at 3 m. The roof
    thickness is measured perpendicular to the pitch, so the inner roof line drops
    by ``roof · √(1 + slope²)``. Gross and net volume both follow from the polygons.
    https://de.wikipedia.org/wiki/DIN_V_18599
    """
    w, depth, eave, ridge, floor2 = 10.0, 8.0, 6.0, 8.5, 3.0
    slope = (ridge - eave) / (w / 2)
    drop = roof * np.sqrt(1 + slope ** 2)

    def roof_in(x: float) -> float:
        return eave + slope * min(x, w - x) - drop

    outer = [(0, 0), (w, 0), (w, eave), (w / 2, ridge), (0, eave)]
    eg = [(wall, slab), (w - wall, slab), (w - wall, floor2), (wall, floor2)]
    og = [(wall, floor2 + decke), (w - wall, floor2 + decke), (w - wall, roof_in(w - wall)),
          (w / 2, ridge - drop), (wall, roof_in(wall))]
    pieces = {
        "slab": [(0, 0), (w, 0), (w, slab), (0, slab)],
        "decke": [(wall, floor2), (w - wall, floor2), (w - wall, floor2 + decke), (wall, floor2 + decke)],
        "wall_l": [(0, slab), (wall, slab), (wall, roof_in(wall)), (0, eave - drop)],
        "wall_r": [(w - wall, slab), (w, slab), (w, eave - drop), (w - wall, roof_in(w - wall))],
        "roof_l": [(0, eave), (w / 2, ridge), (w / 2, ridge - drop), (0, eave - drop)],
        "roof_r": [(w / 2, ridge), (w, eave), (w, eave - drop), (w / 2, ridge - drop)],
    }
    a_e = _shoelace(outer)
    a_net = _shoelace(eg) + _shoelace(og)
    d_i = depth - 2 * wall
    return {
        "w": w, "depth": depth, "eave": eave, "ridge": ridge, "wall": wall, "slab": slab,
        "outer": outer, "rooms": [eg, og], "pieces": pieces,
        "A_e": a_e, "V_e": a_e * depth, "A": a_net, "d_i": d_i, "V": a_net * d_i,
        "top_in": ridge - drop,
    }


def _pf_house(house, origin, scale: float, *, eave_m: float = 0.5):
    """🏡 The example house in the Physical Fundamentals line style — outer and inner wall faces, eaves, hatched ground
    and a sash in each wall of both storeys; the outline stays the gross-volume polygon.
    """
    o, s, w = np.array(origin, dtype=float), scale, house["w"]
    slope = (house["ridge"] - house["eave"]) / (w / 2)
    outline = Polygon(*_to_scene(house["outer"], o, s), color=PASTEL_WHITE, stroke_width=1.8)
    inner = VGroup(*[Polygon(*_to_scene(r, o, s), color=PASTEL_WHITE, stroke_width=1.2, stroke_opacity=0.8)
                     for r in house["rooms"]])
    eaves = VGroup(*[Line(*_to_scene([(x0, house["eave"]), (x0 + d * eave_m, house["eave"] - slope * eave_m)], o, s),
                          color=PASTEL_WHITE, stroke_width=1.8) for x0, d in ((0.0, -1.0), (w, 1.0))])
    ground = Line(o + LEFT * 0.4, o + RIGHT * (w * s + 0.2), color=PASTEL_TEAL, stroke_width=2.2)
    hatch = VGroup(*[Line(p, p + DOWN * 0.1 + LEFT * 0.07, color=PASTEL_TEAL, stroke_width=1.1)
                     for p in [ground.point_from_proportion(u) for u in np.linspace(0.06, 0.94, 11)]])
    windows, masks = [], VGroup()
    for side, x_m in (("l", house["wall"] / 2), ("r", w - house["wall"] / 2)):
        for sill_m, lintel_m in ((1.2, 2.5), (4.1, 5.3)):
            x, sill, lintel = o[0] + x_m * s, o[1] + sill_m * s, o[1] + lintel_m * s
            masks.add(Rectangle(width=house["wall"] * s + 0.04, height=lintel - sill, stroke_width=0,
                                fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to([x, (sill + lintel) / 2, 0]))
            win = window_glyph(x, sill, lintel, depth=0.8 * (lintel - sill), color=PASTEL_CYAN)
            win["side"] = side
            windows.append(win)
    group = VGroup(ground, hatch, outline, inner, eaves, masks, *[win["group"] for win in windows])
    return {"outline": outline, "inner": inner, "eaves": eaves, "ground": VGroup(ground, hatch), "windows": windows,
            "group": group, "x_l": o[0], "x_r": o[0] + w * s}


def _open_sash(window, *, run_time: float = 0.9, close: bool = False):
    """🚪 Swing a ``window_glyph`` sash into the room — rightwards in a left wall, leftwards in a right wall."""
    angle = (-PI / 2 if window.get("side", "l") == "l" else PI / 2) * (-1 if close else 1)
    return Rotate(window["sash"], angle=angle, axis=UP, about_point=window["hinge"], run_time=run_time)
#endregion


#region Shared numbers
RHO_AIR, CP_AIR = 1.2, 1005.0
C_LUFT = round(RHO_AIR * CP_AIR / 3600.0, 2)
N_RATE, D_THETA = 0.5, 20.0
HOUSE = _house_section()
V_NET = round(HOUSE["V"], -1)
PHI_V = V_NET * N_RATE * C_LUFT * D_THETA
ETA_WRG = 0.8
#endregion


#region Instrument glyphs
def _clock(center, r: float, minutes, *, color=PASTEL_CYAN):
    """🕐 Physical Fundamentals ``clock_glyph`` whose minute hand follows ``minutes`` (a ``ValueTracker``) — one turn is one hour."""
    dial = clock_glyph(center, r=r, color=color)
    face = dial["face"].set_fill(P_DEEP_DARK, opacity=0.9)
    ticks = dial["group"][1]

    def hand():
        mid, rad = face.get_center(), face.width / 2
        a = TAU * minutes.get_value() / 60.0
        return Line(mid, mid + 0.8 * rad * np.array([np.sin(a), np.cos(a), 0.0]), color=color, stroke_width=3)

    hand_mob = always_redraw(hand)
    pin = always_redraw(lambda: Dot(face.get_center(), radius=0.035, color=color))
    return {"face": face, "group": VGroup(face, ticks, hand_mob, pin), "static": VGroup(face, ticks)}


def _vbar(x: float, y0: float, height: float, frac, *, width: float = 0.26, color=PASTEL_CYAN):
    """📊 Vertical fill gauge from ``y0`` upward; ``frac()`` returns 0…1 every frame."""
    track = Rectangle(width=width, height=height, color=PASTEL_WHITE, stroke_width=2)
    track.set_fill(P_DEEP_DARK, opacity=1.0).move_to([x, y0 + height / 2, 0])

    def fill():
        h = max(0.004, float(np.clip(frac(), 0, 1)) * (height - 0.06))
        return Rectangle(width=width - 0.06, height=h, stroke_width=0, fill_color=color,
                         fill_opacity=0.85).move_to([x, y0 + 0.03 + h / 2, 0])

    return VGroup(track, always_redraw(fill))
#endregion


#region Air change scale
def _n_scale(x: float, y_lo: float, y_hi: float, *, lo: float = 0.1, hi: float = 30.0, zero_drop: float = 0.45):
    """📶 Vertical log scale of the air change rate n in 1/h with a broken-off "≈ 0" end below 0,1."""
    span = np.log10(hi) - np.log10(lo)

    def y_of(value: float) -> float:
        if value <= 0:
            return y_lo - zero_drop
        return y_lo + (np.log10(value) - np.log10(lo)) / span * (y_hi - y_lo)

    axis = Line([x, y_lo, 0], [x, y_hi, 0], color=PASTEL_TEAL, stroke_width=3)
    stub = Line([x, y_lo - zero_drop, 0], [x, y_lo - 0.55 * zero_drop, 0], color=PASTEL_TEAL, stroke_width=3)
    gap = y_lo - 0.36 * zero_drop
    brk = VGroup(*[Line([x - 0.11, gap + dy - 0.05, 0], [x + 0.11, gap + dy + 0.05, 0], color=PASTEL_TEAL, stroke_width=2)
                   for dy in (-0.05, 0.05)])
    ticks, labels = VGroup(), VGroup()
    values = [d * 10.0 ** e for e in (-1, 0, 1) for d in range(1, 10)] + [20.0, 30.0]
    for v in sorted(set(v for v in values if lo <= v <= hi)):
        y = y_of(v)
        major = v in (0.1, 1.0, 10.0)
        ticks.add(Line([x - (0.17 if major else 0.08), y, 0], [x, y, 0], color=PASTEL_WHITE,
                       stroke_width=2.0 if major else 1.1))
        if major:
            labels.add(math_label(de_num(v, 1 if v < 1 else 0), np.array([x - 0.26, y - 0.07, 0]),
                                  size=LABEL_FONT_SIZE, edge="right"))
    ticks.add(Line([x - 0.17, y_of(0), 0], [x, y_of(0), 0], color=PASTEL_WHITE, stroke_width=2.0))
    labels.add(math_label("0", np.array([x - 0.26, y_of(0) - 0.07, 0]), size=LABEL_FONT_SIZE, edge="right"))
    title = math_label(r"n\;[\mathrm{h^{-1}}]", np.array([x + 0.1, y_hi + 0.22, 0]), size=LABEL_FONT_SIZE,
                       color=PASTEL_TEAL, edge="right")
    return {"y_of": y_of, "x": x, "group": VGroup(axis, stub, brk, ticks, labels, title)}


def _scale_marks(scale, entries, *, x_text: float, gap: float = 0.08, bottom: float = -1.72, top: float = 2.32):
    """🏷️ Markers on the n scale with labels dodged apart so neighbouring uses stay readable."""
    x = scale["x"]
    dots, labels, leaders, ys = [], [], [], []
    for value, src, color in entries:
        y = scale["y_of"](value)
        dots.append(Dot([x, y, 0], radius=0.075, color=color))
        labels.append(math_label(src, size=BODY_FONT_SIZE, color=color))
        ys.append(y)
    placed = list(ys)
    order = sorted(range(len(placed)), key=lambda i: -placed[i])
    for a, b in zip(order, order[1:]):
        placed[b] = min(placed[b], placed[a] - (labels[a].height + labels[b].height) / 2 - gap)
    under = bottom - (placed[order[-1]] - labels[order[-1]].height / 2)
    if under > 0:
        placed = [p + under for p in placed]
    over = placed[order[0]] + labels[order[0]].height / 2 - top
    if over > 0:
        placed = [p - over for p in placed]
    for i, label in enumerate(labels):
        label.move_to([x_text + label.width / 2, placed[i], 0])
        leaders.append(Line([x + 0.09, ys[i], 0], [x_text - 0.08, placed[i], 0], color=entries[i][2],
                            stroke_width=1.3, stroke_opacity=0.6))
    return dots, labels, leaders


def _mini_room(color, w: float = 1.6, h: float = 1.1, t: float = 0.1):
    """🏛️ Physical Fundamentals room in miniature — ceiling and floor slabs between two side walls."""
    style = dict(color=color, stroke_width=1.8, fill_color=color, fill_opacity=0.14)
    return VGroup(*[Rectangle(width=w + 2 * t, height=t, **style).move_to([0, sy * (h + t) / 2, 0]) for sy in (1, -1)],
                  *[Rectangle(width=t, height=h, **style).move_to([sx * (w + t) / 2, 0, 0]) for sx in (-1, 1)])


def _use_glyph(kind: str, color):
    """🏢 Small room for one use — storage, passive house, office, classroom, kitchen, operating room — drawn with the
    Physical Fundamentals people, lamp and slabs; ``.air`` holds the paths its room air takes.
    """
    parts = VGroup(_mini_room(color))
    fy = -0.55

    def path(*pts):
        return smooth_path([np.array([x, y, 0.0]) for x, y in pts])

    air = []
    if kind == "lager":
        for cx, cy in ((-0.38, fy + 0.2), (0.04, fy + 0.2), (-0.17, fy + 0.6)):
            box = Square(0.38, color=GREY_B, stroke_width=2).move_to([cx, cy, 0])
            parts.add(box, Line(box.get_corner(DL), box.get_corner(UR), color=GREY_B, stroke_width=1.4),
                      Line(box.get_corner(UL), box.get_corner(DR), color=GREY_B, stroke_width=1.4))
    elif kind == "passiv":
        unit = Rectangle(width=0.5, height=0.24, color=PASTEL_GREEN, stroke_width=2).move_to([-0.2, 0.36, 0])
        parts.add(Line(unit.get_left(), [-0.8, 0.36, 0], color=PASTEL_BLUE, stroke_width=2.4),
                  Line(unit.get_right(), [0.8, 0.36, 0], color=PASTEL_ORANGE, stroke_width=2.4),
                  unit, _fan(unit.get_center(), PASTEL_GREEN, radius=0.08),
                  person_glyph([0.35, fy + 0.3, 0], color=PASTEL_ORANGE, scale=0.9))
        air = [path((-0.3, 0.22), (-0.55, -0.05), (-0.35, -0.4)), path((0.65, -0.4), (0.62, 0.02), (0.1, 0.22))]
    elif kind == "buero":
        parts.add(Line([-0.08, -0.12, 0], [0.66, -0.12, 0], color=GREY_B, stroke_width=3),
                  *[Line([x, -0.12, 0], [x, fy, 0], color=GREY_B, stroke_width=2) for x in (-0.02, 0.6)],
                  Rectangle(width=0.36, height=0.26, color=PASTEL_CYAN, stroke_width=2).move_to([0.36, 0.06, 0]),
                  seated_person_glyph([-0.42, fy, 0], color=color, scale=1.15, chair_color=GREY_B)["group"])
        air = [path((-0.72, y), (0.0, y - 0.04), (0.72, y)) for y in (0.36, 0.46)]
    elif kind == "klasse":
        parts.add(Rectangle(width=1.0, height=0.3, color=PASTEL_GREEN, stroke_width=2).set_fill("#163a2a", 0.9)
                  .move_to([0, 0.32, 0]),
                  *[seated_person_glyph([cx, fy, 0], color=color, scale=0.72, chair_color=GREY_B)["group"]
                    for cx in (-0.6, -0.12, 0.36)])
        air = [path((-0.72, y), (0.0, y + 0.03), (0.72, y)) for y in (0.0, 0.08)]
    elif kind == "kueche":
        stove = Rectangle(width=0.9, height=0.34, color=GREY_B, stroke_width=2).move_to([0, fy + 0.17, 0])
        pot = Rectangle(width=0.32, height=0.2, color=PASTEL_WHITE, stroke_width=2).next_to(stove, UP, buff=0)
        hood = Polygon([-0.42, 0.5, 0], [0.42, 0.5, 0], [0.28, 0.3, 0], [-0.28, 0.3, 0], color=GREY_B, stroke_width=2)
        parts.add(stove, pot, hood)
        air = [path((x, -0.0), (x + 0.04, 0.14), (x * 0.6, 0.3)) for x in (-0.09, 0.05)]
    elif kind == "op":
        parts.add(Line([-0.45, -0.2, 0], [0.45, -0.2, 0], color=PASTEL_WHITE, stroke_width=4),
                  Line([0, -0.2, 0], [0, fy, 0], color=GREY_B, stroke_width=3),
                  lamp_glyph([0, 0.55, 0], drop=0.08)["group"].set_color(PASTEL_YELLOW),
                  person_glyph([0.6, fy + 0.29, 0], color=PASTEL_CYAN, scale=0.88))
        air = [path((x, 0.5), (x, -0.12)) for x in (-0.66, -0.42, 0.34)]
    parts.air = air
    return parts
#endregion


#region Ventilation room section
def _vent_room(ox: float, floor_y: float, k: float, title: str, color):
    """🪟 Room section with exterior wall, window opening, radiator, thermostat and the reveal (Laibung).

    ``ox`` is the inner face of the back wall, ``k`` scene units per metre. The
    room is 4 m wide and 2,5 m high; the window sits 0,9 m … 2,2 m above the floor
    in a 0,35 m exterior wall with the radiator below it.
    """
    w, h, t = 4.0 * k, 2.5 * k, 0.35 * k
    x_in, x_out = ox + w, ox + w + t
    sill, lintel, ceil = floor_y + 0.9 * k, floor_y + 2.2 * k, floor_y + h
    style = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)
    floor = Rectangle(width=w + t + 0.14, height=0.16, **style)
    floor.move_to([ox - 0.14 + (w + t + 0.14) / 2, floor_y - 0.08, 0])
    ceiling = floor.copy().move_to([floor.get_center()[0], ceil + 0.08, 0])
    back = Rectangle(width=0.14, height=h, **style).move_to([ox - 0.07, floor_y + h / 2, 0])
    brust = Rectangle(width=t, height=sill - floor_y, **style).move_to([(x_in + x_out) / 2, (floor_y + sill) / 2, 0])
    sturz = Rectangle(width=t, height=ceil - lintel, color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_ORANGE,
                      fill_opacity=0.7).move_to([(x_in + x_out) / 2, (lintel + ceil) / 2, 0])
    laibung = Rectangle(width=t, height=lintel - sill, stroke_width=0, fill_color=PASTEL_ORANGE, fill_opacity=0.14)
    laibung.move_to([(x_in + x_out) / 2, (sill + lintel) / 2, 0])
    window = window_glyph((x_in + x_out) / 2, sill, lintel, depth=0.42, color=PASTEL_CYAN)
    window["side"] = "r"
    rad = radiator([x_in - 0.62, floor_y + 0.33, 0], color=PASTEL_RED)
    valve = Circle(radius=0.065, color=PASTEL_WHITE, stroke_width=2).set_fill(P_DEEP_DARK, 1.0)
    valve.move_to([x_in - 0.08, rad[0].get_top()[1] - 0.06, 0])
    head = math_label(title, size=BODY_FONT_SIZE, color=color)
    head.move_to([ox + w / 2, ceil + 0.36, 0])
    return {
        "x_in": x_in, "x_out": x_out, "sill": sill, "lintel": lintel, "ceil": ceil, "floor": floor_y,
        "ox": ox, "w": w, "k": k, "rad": rad, "valve": valve, "window": window, "sash": window["sash"],
        "sturz": sturz, "laibung": laibung, "head": head,
        "shell": VGroup(floor, ceiling, back, brust, sturz, laibung),
        "group": VGroup(floor, ceiling, back, brust, laibung, sturz, window["group"], rad, valve),
    }


def _valve_dial(room, opening):
    """🌡️ Thermostat dial mark — turns with ``opening()`` from closed (0) to fully open (1)."""
    def mark():
        c = room["valve"].get_center()
        a = PI / 2 - 0.9 * PI * float(np.clip(opening(), 0, 1))
        return Line(c, c + 0.06 * np.array([np.cos(a), np.sin(a), 0]), color=PASTEL_YELLOW, stroke_width=2)
    return always_redraw(mark)
#endregion


#region Energy bar rows
def _energy_row(x0: float, y: float, unit: float, parts, *, height: float = 0.5):
    """🧱 Horizontal stacked bar redrawn every frame from ``(value_fn, colour)`` parts in Wh."""
    def build():
        row, cursor = VGroup(), x0
        for fn, colour in parts:
            w = max(0.0, float(fn())) * unit
            if w > 0.004:
                row.add(Rectangle(width=w, height=height, stroke_width=1.2, color=colour, fill_color=colour,
                                  fill_opacity=0.72).move_to([cursor + w / 2, y, 0]))
            cursor += w
        return row
    return always_redraw(build)
#endregion


#region Beat1 – Building and ventilation
class Beat1_GebaeudeKonvektion(Scene):
    NARRATION = [
        ("intro",
         "Warm indoor air rises and escapes through gaps — cold outdoor air slips in to replace it.",
         "Warme Innenluft steigt und entweicht durch Fugen — kalte Außenluft strömt nach."),
        ("zones",
         "Inside stays warm; outside stays cold. Air exchanging through the openings is ventilation, not the U-value.",
         "Innen bleibt warm, außen bleibt kalt. Der Austausch durch die Öffnungen ist Lüftung — nicht der U-Wert."),
        ("flow",
         "Watch the particles: heat leaves with the orange stream while the blue stream cools the room.",
         "Beobachten Sie die Partikel: Wärme geht mit dem orangen Strom, der blaue Strom kühlt den Raum."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Das Gebäude & Lüftung", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        N = self.NARRATION
        hs = house_section(np.array([0.95, -0.2, 0.0]), scale=1.15)
        bl, br, tl = hs["bottom_left"], hs["bottom_right"], hs["top_left"]
        x_r, t = br[0], 0.1
        air = Polygon(bl + RIGHT * t + UP * 0.02, br + LEFT * t + UP * 0.02, hs["top_right"] + LEFT * t + DOWN * 0.06,
                      hs["roof_peak"] + DOWN * 0.16, tl + RIGHT * t + DOWN * 0.06,
                      stroke_width=0, fill_color=PASTEL_ORANGE, fill_opacity=0.0).set_z_index(-1)
        y_mid = hs["level_1"].get_center()[1]
        intake, exhaust = [], []
        for win, (yf, yc) in zip(hs["windows"], ((bl[1], y_mid), (y_mid, tl[1]))):
            x_w, sill, lintel = win["x"], win["sill"], win["lintel"]
            for dy in (0.0, 0.11):
                intake.append(smooth_path([np.array([x, y + dy, 0.0]) for x, y in (
                    (x_w - 1.5, sill - 0.08), (x_w - 0.5, sill + 0.08), (x_w + 0.35, sill + 0.02),
                    (x_w + 1.1, yf + 0.2), (x_r - 0.9, yf + 0.18), (x_r - 0.3, yf + 0.6))]))
                exhaust.append(smooth_path([np.array([x, y - dy, 0.0]) for x, y in (
                    (x_r - 0.3, yf + 0.75), (x_r - 0.55, yc - 0.18), (x_w + 1.4, yc - 0.15),
                    (x_w + 0.4, lintel - 0.06), (x_w - 0.4, lintel - 0.06), (x_w - 1.0, lintel + 0.28))]))
        guides = VGroup(flow_guides(intake[::2], PASTEL_BLUE, opacity=0.22),
                        flow_guides(exhaust[::2], PASTEL_ORANGE, opacity=0.22))

        def breeze(rt, waves=3):
            return [_streams([(intake, PASTEL_BLUE, PASTEL_ORANGE), (exhaust, PASTEL_ORANGE)], rt, speed=0.45, waves=waves)]

        txt_innen = Text("Innen", font_size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        txt_warm = Text("Warm", font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE)
        label_inside = VGroup(txt_innen, txt_warm).arrange(DOWN, buff=0.12)
        label_inside.move_to([0.5 * (bl[0] + x_r) + 0.25, 0.5 * (bl[1] + y_mid), 0])
        txt_aussen = Text("Außen", font_size=BODY_FONT_SIZE, color=PASTEL_BLUE)
        txt_kalt = Text("Kalt", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE)
        label_outside = VGroup(txt_aussen, txt_kalt).arrange(DOWN, buff=0.12).move_to([-4.3, y_mid, 0])

        self.play(Create(hs["group"]), run_time=1.5)
        self.add(air)
        self.play(*[open_window(w) for w in hs["windows"]], FadeIn(guides), run_time=0.9)
        self.play(*breeze(1.2), run_time=1.2)
        hold_for(self, N, "intro", used=1.5 + 0.9 + 1.2, during=breeze)

        caption = swap_caption(self, caption, subtitle_text(N, "zones"))
        self.play(FadeIn(label_inside), FadeIn(label_outside), air.animate.set_fill(PASTEL_ORANGE, opacity=0.12),
                  *breeze(1.0), run_time=1.0)
        hold_for(self, N, "zones", during=breeze)

        caption = swap_caption(self, caption, subtitle_text(N, "flow"))
        self.play(*[ShowPassingFlash(g.copy().set_stroke(width=4, opacity=0.9), time_width=0.4) for g in guides[0]],
                  *[ShowPassingFlash(g.copy().set_stroke(width=4, opacity=0.9), time_width=0.4) for g in guides[1]],
                  *breeze(1.2, 5), run_time=1.2)
        hold_for(self, N, "flow", during=lambda rt: breeze(rt, 5))

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Gross volume V_e and net air volume V
class Beat2_Innenvolumen(Scene):
    """📦 Section of the example house: gross volume from outer dimensions, then construction subtracted to the net air volume."""

    NARRATION = [
        ("intro",
         "We draw the house in section: ten metres wide, six metres to the eaves, eight and a half to the ridge.",
         "Wir zeichnen das Haus im Schnitt: 10 m breit, 6 m bis zur Traufe, 8,5 m bis zum First."),
        ("gross",
         "With outer dimensions the section is 72.5 square metres; times 8 metres depth gives the gross volume V e of 580 cubic metres.",
         "Mit Außenmaßen hat der Schnitt 72,5 m², mal 8 m Tiefe: das Bruttovolumen V-e mit 580 m³."),
        ("focus",
         "Outdoor air fades away — we heat only the air inside. But walls, slabs and roof are not air.",
         "Die Außenluft verblasst — geheizt wird nur die Luft innen. Wände, Decken und Dach sind aber keine Luft."),
        ("subtract",
         "Subtracting the construction leaves about 60 square metres of section and 7.3 metres inner depth.",
         "Ohne die Konstruktion bleiben rund 60 m² Schnittfläche und 7,3 m Innentiefe."),
        ("fill",
         "The two air spaces filled up are the net volume V — about 440 cubic metres.",
         "Die beiden Lufträume ergeben das Nettovolumen V — rund 440 m³."),
        ("ratio",
         "So V is about 0.76 times V e — the flat rate of DIN V 18599-1 and the GEG.",
         "V ist also rund 0,76 mal V-e — so setzen es DIN V 18599-1 und GEG pauschal an."),
        ("label",
         "V is the net volume — the starting point for ventilation heat loss.",
         "V ist das Nettovolumen — Ausgangspunkt für den Lüftungswärmeverlust."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        H = HOUSE

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Bruttovolumen und Nettovolumen", title)
        din = _din_ref("DIN V 18599-1 · DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        #region outline and dimensions
        S, O = 0.37, np.array([-5.75, -1.12, 0.0])
        depth_vec = np.array([0.44, 0.32, 0.0])
        outer_pts = _to_scene(H["outer"], O, S)
        pf = _pf_house(H, O, S)
        outline = pf["outline"]
        x_l, x_r = outer_pts[0][0], outer_pts[1][0]
        w_dim = dim_arrow([x_l, O[1] - 0.24, 0], [x_r, O[1] - 0.24, 0], color=PASTEL_YELLOW)
        w_lbl = math_label(rf"{de_num(H['w'])}\,\mathrm{{m}}", np.array([(x_l + x_r) / 2, O[1] - 0.52, 0]),
                           size=LABEL_FONT_SIZE, color=PASTEL_YELLOW)
        e_dim = dim_arrow([x_l - 0.3, O[1], 0], [x_l - 0.3, outer_pts[4][1], 0], color=PASTEL_YELLOW)
        e_lbl = math_label(rf"{de_num(H['eave'])}\,\mathrm{{m}}", size=LABEL_FONT_SIZE, color=PASTEL_YELLOW)
        e_lbl.next_to(e_dim, LEFT, buff=0.1)
        r_x = x_r + depth_vec[0] + 0.3
        r_dim = dim_arrow([r_x, O[1], 0], [r_x, outer_pts[3][1], 0], color=PASTEL_YELLOW)
        r_lbl = math_label(rf"{de_num(H['ridge'], 1)}\,\mathrm{{m}}", size=LABEL_FONT_SIZE, color=PASTEL_YELLOW)
        r_lbl.next_to(r_dim, RIGHT, buff=0.1)
        out_paths = [smooth_path([np.array(p) + d for d in ([-0.16, -0.03, 0], [0.06, 0.03, 0], [0.28, -0.01, 0])])
                     for p in ([-6.6, 1.75, 0], [-5.2, 2.2, 0], [-6.75, 0.2, 0], [-2.35, 2.25, 0], [-1.05, 1.95, 0],
                               [-0.75, -0.35, 0], [-1.15, -0.95, 0], [-6.75, -0.75, 0])]

        def outdoor(rt):
            return [_streams([(out_paths, PASTEL_BLUE)], rt, speed=0.55, waves=2)]

        self.play(Create(pf["group"]), *outdoor(1.4), run_time=1.4)
        self.play(GrowFromCenter(w_dim), FadeIn(w_lbl), GrowFromCenter(e_dim), FadeIn(e_lbl),
                  GrowFromCenter(r_dim), FadeIn(r_lbl), *outdoor(1.2), run_time=1.2)
        hold_for(self, N, "intro", used=2.6, during=outdoor)
        #endregion

        #region gross section area and depth
        caption = swap_caption(self, caption, subtitle_text(N, "gross"))
        level = ValueTracker(0.001)
        depth = ValueTracker(0.0)
        x0 = 0.25

        def area_e():
            return _shoelace(_clip_below(H["outer"], level.get_value()))

        gross_fill = always_redraw(lambda: _poly(_clip_below(H["outer"], level.get_value()), O, S, stroke_width=0,
                                                 fill_color=PASTEL_TEAL, fill_opacity=0.28))
        a_read = math_readout(lambda: rf"A_{{\mathrm{{e}}}} = {de_num(area_e(), 1)}\,\mathrm{{m^{{2}}}}",
                              np.array([x0, 1.85, 0]), size=BODY_FONT_SIZE, color=PASTEL_TEAL)
        self.add(gross_fill, a_read)
        self.play(level.animate.set_value(H["ridge"]), *outdoor(2.0), run_time=2.0, rate_func=linear)

        def back():
            off = depth_vec * depth.get_value() / H["depth"]
            pts = [p + off for p in outer_pts]
            edges = [Line(outer_pts[i], pts[i], color=PASTEL_WHITE, stroke_width=1.4, stroke_opacity=0.45) for i in (1, 2, 3, 4)]
            return VGroup(Polygon(*pts, color=PASTEL_WHITE, stroke_width=1.4, stroke_opacity=0.45), *edges)

        back_mob = always_redraw(back)
        ve_read = math_readout(
            lambda: rf"V_{{\mathrm{{e}}}} = {de_num(H['A_e'], 1)} \cdot {de_num(depth.get_value(), 1)} = "
                    rf"{de_num(H['A_e'] * depth.get_value())}\,\mathrm{{m^{{3}}}}",
            np.array([x0, 1.28, 0]), size=BODY_FONT_SIZE, color=PASTEL_WHITE)
        self.add(back_mob, ve_read)
        self.play(depth.animate.set_value(H["depth"]), *outdoor(1.8), run_time=1.8, rate_func=linear)
        ve_read.clear_updaters()
        d_start = outer_pts[1] + RIGHT * 0.12 + DOWN * 0.1
        d_dim = dim_arrow(d_start, d_start + depth_vec, color=PASTEL_YELLOW)
        d_lbl = math_label(rf"{de_num(H['depth'])}\,\mathrm{{m}}", size=LABEL_FONT_SIZE, color=PASTEL_YELLOW)
        d_lbl.next_to(d_dim.get_center(), DR, buff=0.04)
        self.play(GrowFromCenter(d_dim), FadeIn(d_lbl), *outdoor(0.6), run_time=0.6)
        hold_for(self, N, "gross", during=outdoor)
        #endregion

        #region construction lights up
        caption = swap_caption(self, caption, subtitle_text(N, "focus"))
        pieces = {name: _poly(pts, O, S, stroke_width=1, color=PASTEL_YELLOW, fill_color=PASTEL_YELLOW, fill_opacity=0.0)
                  for name, pts in H["pieces"].items()}
        self.add(*pieces.values())
        last = outdoor(1.2)[0]
        self.play(last, _fade_flow(last), gross_fill.animate.set_opacity(0.0), run_time=1.2)
        self.remove(gross_fill)
        self.play(*[p.animate.set_fill(PASTEL_YELLOW, opacity=0.75) for p in pieces.values()], run_time=1.0)
        hold_for(self, N, "focus")
        #endregion

        #region subtract construction
        caption = swap_caption(self, caption, subtitle_text(N, "subtract"))
        removed = ValueTracker(0.0)
        a_net_read = math_readout(
            lambda: rf"A = {de_num(H['A_e'], 1)} - {de_num(removed.get_value(), 1)} = "
                    rf"{de_num(H['A_e'] - removed.get_value(), 1)}\,\mathrm{{m^{{2}}}}",
            np.array([x0, 0.62, 0]), size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        self.add(a_net_read)
        for group in (("wall_l", "wall_r"), ("slab", "decke"), ("roof_l", "roof_r")):
            share = sum(_shoelace(H["pieces"][g]) for g in group)
            self.play(*[Indicate(pieces[g], color=PASTEL_WHITE, scale_factor=1.0) for g in group], run_time=0.5)
            self.play(*[FadeOut(pieces[g], scale=0.8) for g in group],
                      removed.animate.increment_value(share), run_time=0.9)
        inner_d = ValueTracker(H["depth"])
        di_read = math_readout(
            lambda: rf"d_{{\mathrm{{i}}}} = {de_num(H['depth'])} - 2 \cdot {de_num(H['wall'], 2)} = "
                    rf"{de_num(inner_d.get_value(), 2)}\,\mathrm{{m}}",
            np.array([x0, 0.05, 0]), size=BODY_FONT_SIZE, color=PASTEL_WHITE)
        self.add(di_read)
        self.play(inner_d.animate.set_value(H["d_i"]), run_time=1.0)
        hold_for(self, N, "subtract")
        #endregion

        #region net air volume fills
        caption = swap_caption(self, caption, subtitle_text(N, "fill"))
        lvl = ValueTracker(H["slab"])
        rooms = H["rooms"]

        def net_area():
            return sum(_shoelace(_clip_below(r, lvl.get_value())) for r in rooms)

        off_front = depth_vec * H["wall"] / H["depth"]
        off_back = depth_vec * (H["depth"] - H["wall"]) / H["depth"]

        def prism(pts, *, fill: float, stroke: float):
            if len(pts) < 3:
                return VGroup()
            front = [p + off_front for p in _to_scene(pts, O, S)]
            back = [p + off_back for p in _to_scene(pts, O, S)]
            faces = VGroup(Polygon(*back, stroke_color=PASTEL_ORANGE, stroke_width=stroke, stroke_opacity=0.55,
                                   fill_color=PASTEL_ORANGE, fill_opacity=fill * 0.45))
            for a, b in zip(range(len(front)), list(range(1, len(front))) + [0]):
                faces.add(Polygon(front[a], front[b], back[b], back[a], stroke_width=0,
                                  fill_color=PASTEL_ORANGE, fill_opacity=fill * 0.35))
            faces.add(*[Line(f, k, color=PASTEL_ORANGE, stroke_width=stroke, stroke_opacity=0.7) for f, k in zip(front, back)])
            faces.add(Polygon(*front, stroke_color=PASTEL_ORANGE, stroke_width=stroke, fill_color=PASTEL_ORANGE, fill_opacity=fill))
            return faces

        room_fill = always_redraw(lambda: VGroup(*[
            prism(_clip_below(r, lvl.get_value()), fill=0.5, stroke=0.0) for r in rooms
        ]))
        room_edges = VGroup(*[prism(r, fill=0.0, stroke=1.6) for r in rooms])
        v_read = math_readout(
            lambda: rf"V = {de_num(net_area(), 1)} \cdot {de_num(H['d_i'], 2)} = {de_num(net_area() * H['d_i'])}\,\mathrm{{m^{{3}}}}",
            np.array([x0, -0.52, 0]), size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        self.add(room_fill, v_read)
        self.play(Create(room_edges), pf["group"].animate.set_stroke(opacity=0.5), run_time=0.6)
        self.play(lvl.animate.set_value(H["top_in"]), run_time=2.4, rate_func=linear)
        og_c = np.mean(_to_scene(rooms[1], O, S), axis=0)
        v_sym = math_label("V", np.array([og_c[0], og_c[1] - 0.12, 0]) + off_front, size=FORMULA_FONT_SIZE, color=PASTEL_WHITE)
        loops = []
        for r in rooms:
            pts = [p + off_front for p in _to_scene(r, O, S)]
            mid = np.mean(pts, axis=0)
            ring = [mid + 0.72 * (p - mid) for p in pts]
            loops.append(smooth_path(ring + ring[:1]))

        def warm_air(rt):
            return [_streams([(loops, PASTEL_ORANGE)], rt, speed=0.22, waves=5)]

        self.play(FadeIn(v_sym, scale=1.3), *warm_air(0.5), run_time=0.5)
        hold_for(self, N, "fill", during=warm_air)
        #endregion

        #region ratio bars and formula
        caption = swap_caption(self, caption, subtitle_text(N, "ratio"))
        ratio = H["V"] / H["V_e"]
        bx, blen = 1.05, 3.9
        bar_e = Rectangle(width=blen, height=0.22, color="#8A9BA8", stroke_width=1.6).set_fill("#8A9BA8", 0.25)
        bar_e.move_to([bx + blen / 2, -1.0, 0])
        bar_v = Rectangle(width=blen * ratio, height=0.22, color=PASTEL_ORANGE, stroke_width=1.6).set_fill(PASTEL_ORANGE, 0.6)
        bar_v.move_to([bx + blen * ratio / 2, -1.36, 0])
        lbl_e = math_label(r"V_{\mathrm{e}}", np.array([bx - 0.12, -1.07, 0]), size=LABEL_FONT_SIZE, edge="right")
        lbl_v = math_label("V", np.array([bx - 0.12, -1.43, 0]), size=LABEL_FONT_SIZE, color=PASTEL_ORANGE, edge="right")
        val_e = math_label(rf"{de_num(H['V_e'])}\,\mathrm{{m^{{3}}}}", np.array([bx + blen + 0.14, -1.07, 0]),
                           size=LABEL_FONT_SIZE, edge="left")
        val_v = math_label(rf"{de_num(H['V'])}\,\mathrm{{m^{{3}}}}", np.array([bx + blen * ratio + 0.14, -1.43, 0]),
                           size=LABEL_FONT_SIZE, color=PASTEL_ORANGE, edge="left")
        self.play(TransformFromCopy(outline, bar_e), FadeIn(lbl_e), FadeIn(val_e), *warm_air(1.0), run_time=1.0)
        self.play(TransformFromCopy(room_edges, bar_v), FadeIn(lbl_v), FadeIn(val_v), *warm_air(1.0), run_time=1.0)
        row, box, items = math_panel([
            ("v", "V", PASTEL_ORANGE), (None, r"\approx", None),
            ("f", de_num(ratio, 2), PASTEL_YELLOW), (None, r"\cdot", None),
            ("ve", r"V_{\mathrm{e}}", PASTEL_WHITE), (None, r"=", None),
            ("num", rf"{de_num(ratio, 2)} \cdot {de_num(H['V_e'])} \approx {de_num(V_NET)}\,\mathrm{{m^{{3}}}}", PASTEL_WHITE),
        ])
        self.play(TransformFromCopy(v_sym, items["v"]), TransformFromCopy(lbl_e, items["ve"]), *warm_air(0.8), run_time=0.8)
        self.play(FadeIn(VGroup(*[m for m in row if m is not items["v"] and m is not items["ve"]])),
                  Create(box), *warm_air(0.7), run_time=0.7)
        ring = highlight_param(items, "f", color=PASTEL_YELLOW)
        self.play(Create(ring), *warm_air(0.4), run_time=0.4)
        hold_for(self, N, "ratio", during=warm_air)
        self.play(FadeOut(ring), *warm_air(0.2), run_time=0.2)
        #endregion

        caption = swap_caption(self, caption, subtitle_text(N, "label"))
        ring = highlight_param(items, "v", color=PASTEL_ORANGE)
        self.play(Create(ring), Indicate(v_sym, color=PASTEL_ORANGE), *warm_air(0.6), run_time=0.6)
        hold_for(self, N, "label", during=warm_air)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Air change rate n: animated exchange and a scale of uses
AIR_CHANGE_USES = [
    ("lager", 0.0, r"\text{Lagerraum ohne Nutzung}", GREY_B),
    ("passiv", 0.3, r"\text{Passivhaus mit Lüftungsanlage}", PASTEL_GREEN),
    ("buero", 2.0, r"\text{Büro}", PASTEL_CYAN),
    ("klasse", 4.0, r"\text{Klassenzimmer}", PASTEL_YELLOW),
    ("kueche", 15.0, r"\text{Gewerbeküche}", PASTEL_ORANGE),
    ("op", 20.0, r"\text{Operationssaal}", PASTEL_RED),
]


class Beat3_Luftwechselrate(Scene):
    """🔄 Fresh air replaces room air at the rate n; typical uses morph onto a logarithmic n scale."""

    NARRATION = [
        ("volume",
         "Start again from the heated volume V inside the house: 440 cubic metres.",
         "Wieder vom beheizten Volumen V im Haus ausgehen: 440 m³."),
        ("rate",
         "The air change rate n tells how often that volume is exchanged per hour.",
         "Die Luftwechselrate n sagt, wie oft dieses Volumen pro Stunde ausgetauscht wird."),
        ("hour",
         "At n equals 0.5 per hour, 220 cubic metres of fresh air flow in during one hour.",
         "Bei n = 0,5 pro Stunde strömen in einer Stunde 220 m³ frische Luft herein."),
        ("one_change",
         "One air change means the entire volume is replaced — here after two hours.",
         "Ein Luftwechsel heißt: das ganze Volumen ist ersetzt — hier nach zwei Stunden."),
        ("product",
         "Volume times air change rate — V times n — sets the ventilation airflow.",
         "Volumen mal Luftwechselrate — V mal n — bestimmt den Lüftungsvolumenstrom."),
        ("home",
         "Our 0.5 per hour is the minimum air change for dwellings in DIN EN 12831.",
         "Unsere 0,5 pro Stunde sind der Mindestluftwechsel für Wohnungen nach DIN EN 12831."),
        ("low",
         "An unheated storeroom without use needs almost no air change; a passive house with ventilation unit about 0.3.",
         "Ein unbeheizter Lagerraum ohne Nutzung braucht kaum Luftwechsel, ein Passivhaus mit Lüftungsanlage etwa 0,3."),
        ("mid",
         "An office needs about 2, a classroom about 4 air changes per hour.",
         "Ein Büro braucht etwa 2, ein Klassenzimmer etwa 4 Luftwechsel pro Stunde."),
        ("high",
         "A commercial kitchen about 15, an operating room about 20 per DIN 1946-4 — its air is renewed every three minutes.",
         "Eine Gewerbeküche etwa 15, ein Operationssaal nach DIN 1946-4 etwa 20 — dort ist die Luft alle 3 Minuten neu."),
    ]


    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Luftwechselrate n", title)
        din = _din_ref("DIN EN 12831-1 · DIN 1946-4")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "volume"))
        self.play(FadeIn(caption), run_time=0.3)

        #region house with room air
        S, O = 0.36, np.array([-6.3, -1.3, 0.0])
        outer = HOUSE["outer"]
        pf = _pf_house(HOUSE, O, S)
        win_in, win_out = pf["windows"][0], pf["windows"][3]
        frac = ValueTracker(0.0)

        def fill_y():
            return _level_for([outer], frac.get_value()) if frac.get_value() > 1e-3 else -1.0

        warm = always_redraw(lambda: _poly(_clip_above(outer, fill_y()), O, S, stroke_width=0, fill_color=PASTEL_ORANGE,
                                           fill_opacity=0.32))
        fresh = always_redraw(lambda: _poly(_clip_below(outer, fill_y()), O, S, stroke_width=0, fill_color=PASTEL_BLUE,
                                            fill_opacity=0.45))
        v_lbl = math_label(rf"V = {de_num(V_NET)}\,\mathrm{{m^{{3}}}}", np.array([O[0] + 5 * S, O[1] + 6.1 * S, 0]),
                           size=BODY_FONT_SIZE, color=PASTEL_WHITE)
        self.add(fresh)
        self.play(Create(pf["group"]), FadeIn(warm), FadeIn(v_lbl), run_time=1.6)
        hold_for(self, N, "volume", used=1.6)
        #endregion

        #region clock and readouts
        caption = swap_caption(self, caption, subtitle_text(N, "rate"))
        hours = ValueTracker(0.0)
        minutes = ValueTracker(0.0)
        clock = _clock([-1.0, 0.95, 0], 0.62, minutes)
        t_read = math_readout(lambda: rf"t = {de_num(hours.get_value(), 1)}\,\mathrm{{h}}", np.array([-1.0, 0.0, 0]),
                              size=BODY_FONT_SIZE, color=PASTEL_CYAN, edge="center")
        x0 = 0.55
        n_lbl = math_label(rf"n = {de_num(N_RATE, 1)}\,\mathrm{{h^{{-1}}}}", np.array([x0, 1.75, 0]),
                           size=BODY_FONT_SIZE, color=PASTEL_CYAN, edge="left")
        x_i, y_i = win_in["x"], win_in["center"][1]
        x_o, y_o = win_out["x"], win_out["center"][1]
        in_paths = [smooth_path([np.array([x_i + dx, y_i + dy, 0]) for dx, dy in ((-0.8, -0.08), (0.0, -0.04), (0.9, -0.28))]
                                + [np.array([x_i + 2.2, O[1] + 0.25, 0])])]
        out_paths = [smooth_path([np.array([x_o + dx, y_o + dy, 0]) for dx, dy in ((-1.4, -0.22), (0.0, 0.02), (0.8, 0.37))])]
        streams = [(in_paths, PASTEL_BLUE), (out_paths, PASTEL_ORANGE)]
        guides = VGroup(flow_guides(in_paths, PASTEL_BLUE, opacity=0.22), flow_guides(out_paths, PASTEL_ORANGE, opacity=0.22))

        def exchange(rt):
            return [_streams(streams, rt, speed=0.6, waves=4)]

        self.play(FadeIn(clock["group"]), FadeIn(t_read), FadeIn(n_lbl), _open_sash(win_in, run_time=1.0),
                  _open_sash(win_out, run_time=1.0), run_time=1.0)
        self.play(FadeIn(guides), *exchange(0.8), run_time=0.8)
        hold_for(self, N, "rate", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(N, "hour"))
        fresh_read = math_readout(
            lambda: rf"V_{{\mathrm{{frisch}}}} = V \cdot n \cdot t = {de_num(V_NET * N_RATE * hours.get_value())}\,\mathrm{{m^{{3}}}}",
            np.array([x0, 1.05, 0]), size=BODY_FONT_SIZE, color=PASTEL_BLUE)
        pct_read = math_readout(lambda: rf"{de_num(100 * frac.get_value())}\,\%\;\text{{der Raumluft getauscht}}",
                                np.array([x0, 0.45, 0]), size=LABEL_FONT_SIZE, color=PASTEL_BLUE)
        self.add(fresh_read, pct_read)

        def run_hour(h_to: float):
            rt = 3.2 * (h_to - hours.get_value())
            self.play(_streams(streams, rt, speed=0.75, waves=6), hours.animate.set_value(h_to),
                      minutes.animate.set_value(60.0 * h_to), frac.animate.set_value(min(1.0, N_RATE * h_to)),
                      run_time=rt, rate_func=linear)

        run_hour(1.0)
        flow_read = math_label(rf"\dot{{V}} = \frac{{{de_num(V_NET * N_RATE)}\,\mathrm{{m^{{3}}}}}}{{1\,\mathrm{{h}}}} = "
                               rf"{de_num(V_NET * N_RATE)}\,\mathrm{{m^{{3}}/h}}", np.array([x0, -0.35, 0]),
                               size=BODY_FONT_SIZE, color=PASTEL_TEAL, edge="left")
        snap = math_label(rf"V_{{\mathrm{{frisch}}}} = V \cdot n \cdot t = {de_num(V_NET * N_RATE)}\,\mathrm{{m^{{3}}}}",
                          np.array([x0, 1.05, 0]), size=BODY_FONT_SIZE, color=PASTEL_BLUE, edge="left")
        self.play(TransformFromCopy(snap, flow_read), *exchange(0.9), run_time=0.9)
        hold_for(self, N, "hour", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(N, "one_change"))
        run_hour(2.0)
        full = math_label(rf"t = \frac{{1}}{{n}} = {de_num(1 / N_RATE)}\,\mathrm{{h}}", np.array([-1.0, -0.65, 0]),
                          size=BODY_FONT_SIZE, color=PASTEL_CYAN)
        self.play(FadeIn(full, shift=UP * 0.1), Indicate(fresh, color=PASTEL_CYAN, scale_factor=1.0), *exchange(0.8),
                  run_time=0.8)
        hold_for(self, N, "one_change", during=exchange)
        #endregion

        #region formula panel
        caption = swap_caption(self, caption, subtitle_text(N, "product"))
        row, box, items = math_panel([
            ("v", "V", PASTEL_ORANGE), (None, r"\cdot", None), ("n", "n", PASTEL_CYAN), (None, "=", None),
            ("vd", r"\dot{V}", PASTEL_TEAL), (None, r"\;[\mathrm{m^{3}/h}]", None),
        ])
        self.play(TransformFromCopy(v_lbl, items["v"]), TransformFromCopy(n_lbl, items["n"]),
                  TransformFromCopy(flow_read, items["vd"]), *exchange(1.0), run_time=1.0)
        self.play(FadeIn(row), Create(box), *exchange(0.5), run_time=0.5)
        hold_for(self, N, "product", during=exchange)
        #endregion

        #region scale of uses
        caption = swap_caption(self, caption, subtitle_text(N, "home"))
        self.remove(fresh)
        fresh_static = _poly(outer, O, S, stroke_width=0, fill_color=PASTEL_BLUE, fill_opacity=0.45)
        self.add(fresh_static)
        self.play(*[FadeOut(m) for m in (fresh_read, pct_read, flow_read, full, t_read, n_lbl, guides, warm, clock["group"])],
                  run_time=0.6)
        scale = _n_scale(0.6, -0.62, 2.0, zero_drop=0.4)
        stage = np.array([-3.0, 0.4, 0.0])
        clock_pos = np.array([-5.6, 0.4, 0.0])
        house = VGroup(pf["group"], fresh_static, v_lbl)
        hours.set_value(0.0)
        minutes.set_value(0.0)
        clock["group"].scale(0.62).move_to(clock_pos)
        self.play(FadeIn(scale["group"]), house.animate.scale_to_fit_height(1.35).move_to(stage), run_time=1.2)
        self.play(FadeIn(clock["group"]), run_time=0.4)

        entries = [(N_RATE, r"\text{Wohnung} \approx 0{,}5", PASTEL_ORANGE)] + [
            (n, rf"{name} \approx {de_num(n, 1) if 0 < n < 1 else de_num(n)}", c) for _k, n, name, c in AIR_CHANGE_USES
        ]
        dots, labels, leaders = _scale_marks(scale, entries, x_text=1.2, bottom=-1.15)
        count = ValueTracker(0.0)

        def stage_label(name, color):
            top = math_label(name, size=BODY_FONT_SIZE, color=color).move_to(stage + UP * 0.95)
            val = math_readout(lambda: rf"n = {de_num(count.get_value(), 1)}\,\mathrm{{h^{{-1}}}}",
                               stage + DOWN * 1.05, size=BODY_FONT_SIZE, color=color, edge="center")
            return top, val

        def show_use(glyph, name, value, color, idx):
            count.set_value(0.0)
            minutes.set_value(0.0)
            top, val = stage_label(name, color)
            paths = []
            if glyph is not house:
                d = stage - glyph.get_center()
                glyph.shift(d)
                paths = [p.copy().shift(d) for p in glyph.air]

            def room_air(rt):
                return [_streams([(paths, color)], rt, speed=0.3 + 0.25 * np.sqrt(value), waves=3)] if paths else []

            if glyph is not house:
                self.play(FadeIn(glyph, scale=0.8), FadeIn(top), *room_air(0.6), run_time=0.6)
            else:
                self.play(FadeIn(top), run_time=0.4)
            self.add(val)
            self.play(count.animate.set_value(value), minutes.animate.set_value(60.0), *room_air(1.2), run_time=1.2,
                      rate_func=linear)
            val.clear_updaters()
            self.play(ReplacementTransform(glyph, dots[idx]), ReplacementTransform(VGroup(top, val), labels[idx]),
                      Create(leaders[idx]), run_time=1.0)

        show_use(house, r"\text{Wohnung}", N_RATE, PASTEL_ORANGE, 0)
        hold_for(self, N, "home")
        for key, idxs in (("low", (0, 1)), ("mid", (2, 3)), ("high", (4, 5))):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            for i in idxs:
                kind, value, name, color = AIR_CHANGE_USES[i]
                show_use(_use_glyph(kind, color), name, value, color, i + 1)
            if key == "high":
                renew = math_label(rf"\frac{{60\,\mathrm{{min}}}}{{{de_num(20.0)}}} = {de_num(60 / 20.0)}\,\mathrm{{min}}",
                                   stage + DOWN * 0.1, size=BODY_FONT_SIZE, color=PASTEL_RED)
                self.wait(1.0)
                self.play(TransformFromCopy(labels[6], renew), run_time=1.5)
                self.play(Indicate(renew, color=PASTEL_RED, scale_factor=1.12), run_time=0.8)
            hold_for(self, N, key)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Volumetric heat capacity of air
class Beat4_SpezWaermekapazitaet(Scene):
    """🧊 One cubic metre of air (1,2 kg) warmed by 1 K: the energy counted from 0 is c_Luft."""

    NARRATION = [
        ("cube",
         "Take one cubic meter of air — the unit volume we must heat.",
         "Nehmen wir einen Kubikmeter Luft — das Einheitsvolumen, das wir heizen."),
        ("mass",
         "One cubic metre of air has a mass of about 1.2 kilograms.",
         "Ein Kubikmeter Luft hat eine Masse von etwa 1,2 Kilogramm."),
        ("coil",
         "A heating element warms that cubic meter from below.",
         "Ein Heizelement erwärmt diesen Kubikmeter von unten."),
        ("heat",
         "From 20 to 21 degrees it takes 1.2 times 1005, that is 1206 joules.",
         "Von 20 auf 21 °C braucht es 1,2 mal 1005, also 1206 Joule."),
        ("c_luft",
         "The energy per cubic metre and kelvin is the volumetric heat capacity of air, written c Luft — about 0.34 watt-hours.",
         "Die Energie pro Kubikmeter und Kelvin ist die volumenbezogene Wärmekapazität der Luft, geschrieben c-Luft — etwa 0,34 Wattstunden."),
        ("product",
         "So the product grows: V times n times c Luft.",
         "Das Produkt wächst: V mal n mal c-Luft."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Volumenbezogene Wärmekapazität der Luft", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "cube"))
        self.play(FadeIn(caption), run_time=0.3)

        top_eq = math_label(r"V \cdot n", size=BODY_FONT_SIZE).next_to(subtitle, DOWN, buff=0.22)
        v_src, n_src = top_eq.copy(), top_eq.copy()
        self.add(top_eq)

        #region cube
        c_center = UP * 0.55
        top_pt, tr_pt, br_pt = c_center + UP, c_center + RIGHT * 0.866 + UP * 0.5, c_center + RIGHT * 0.866 + DOWN * 0.5
        bot_pt, bl_pt, tl_pt = c_center + DOWN, c_center + LEFT * 0.866 + DOWN * 0.5, c_center + LEFT * 0.866 + UP * 0.5

        def faces(cold: bool):
            cols = (PASTEL_BLUE, PASTEL_BLUE, PASTEL_BLUE) if cold else (PASTEL_ORANGE, PASTEL_ORANGE, PASTEL_ORANGE)
            edge = PASTEL_CYAN if cold else PASTEL_ORANGE
            return [
                Polygon(c_center, tl_pt, top_pt, tr_pt, fill_color=cols[0], fill_opacity=0.35 if cold else 0.45,
                        stroke_color=edge, stroke_width=2),
                Polygon(c_center, tl_pt, bl_pt, bot_pt, fill_color=cols[1], fill_opacity=0.45 if cold else 0.55,
                        stroke_color=edge, stroke_width=2),
                Polygon(c_center, tr_pt, br_pt, bot_pt, fill_color=cols[2], fill_opacity=0.55 if cold else 0.65,
                        stroke_color=edge, stroke_width=2),
            ]

        face_top, face_left, face_right = faces(True)
        cube = VGroup(face_top, face_left, face_right)
        cube_label = math_label(r"1\,\mathrm{m^{3}}", size=BODY_FONT_SIZE).move_to(face_top.get_center() + UP * 0.15)
        self.play(FadeIn(cube, shift=UP * 0.3), FadeIn(cube_label), run_time=2.0)
        hold_for(self, N, "cube", used=2.0 + 0.3)
        #endregion

        #region mass
        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        xl = -6.45
        mass = ValueTracker(0.0)
        rho_lbl = math_label(rf"\rho = {de_num(RHO_AIR, 1)}\,\mathrm{{kg/m^{{3}}}}", np.array([xl, 1.75, 0]),
                             size=BODY_FONT_SIZE, color=PASTEL_CYAN, edge="left")
        m_read = math_readout(lambda: rf"m = \rho \cdot 1\,\mathrm{{m^{{3}}}} = {de_num(mass.get_value(), 2)}\,\mathrm{{kg}}",
                              np.array([xl, 1.2, 0]), size=BODY_FONT_SIZE, color=PASTEL_CYAN)
        self.play(FadeIn(rho_lbl), run_time=0.5)
        self.add(m_read)
        self.play(mass.animate.set_value(RHO_AIR), Indicate(cube, color=PASTEL_CYAN, scale_factor=1.04), run_time=1.4)
        hold_for(self, N, "mass")
        #endregion

        #region coil and warming
        caption = swap_caption(self, caption, subtitle_text(N, "coil"))
        coil = radiator([0, -0.84, 0], color=PASTEL_RED).scale(1.05)
        coil_label = Text("Heizelement", font_size=LABEL_FONT_SIZE, color=PASTEL_RED).next_to(coil, DOWN, buff=0.1)
        heat_spots = [np.array([x, coil.get_top()[1] + 0.03, 0]) for x in (-0.36, 0.0, 0.36)]
        rise = [smooth_path([np.array([s * x0, y0, 0]), np.array([s * (x0 - 0.08), 0.15, 0]), np.array([s * x0, y1, 0])])
                for s in (-1, 1) for x0, y0, y1 in ((0.55, -0.05, 0.62), (0.2, -0.26, 0.45))]

        def warming(rt, convect=True):
            heat = [ripples(heat_spots, r_max=0.42, rings=2, cycles=rt / 1.2, color=PASTEL_RED)]
            return heat + ([_streams([(rise, PASTEL_BLUE, PASTEL_ORANGE)], rt, speed=0.45, waves=3)] if convect else [])

        cp_lbl = math_label(rf"c_{{p}} = {de_num(CP_AIR)}\,\mathrm{{J/(kg\,K)}}", np.array([xl, 0.55, 0]),
                            size=BODY_FONT_SIZE, color=PASTEL_RED, edge="left")
        self.play(Create(coil), FadeIn(coil_label), FadeIn(cp_lbl), run_time=1.5)
        hold_for(self, N, "coil", during=lambda rt: warming(rt, convect=False))

        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        theta = ValueTracker(20.0)
        th_x = -1.65
        thermo = thermometer_glyph(np.array([th_x, -0.22, 0]), height=1.8, color=PASTEL_RED, level=0.35)
        thermo["column"].add_updater(lambda m: thermo["level"].set_value(0.35 + 0.45 * (theta.get_value() - 20.0)))
        th_read = math_readout(lambda: rf"\theta = {de_num(theta.get_value(), 1)}\,°\mathrm{{C}}", np.array([xl, -0.1, 0]),
                               size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        q_read = math_readout(
            lambda: rf"Q = {de_num(RHO_AIR, 1)} \cdot {de_num(CP_AIR)} \cdot {de_num(theta.get_value() - 20.0, 1)} = "
                    rf"{de_num(RHO_AIR * CP_AIR * (theta.get_value() - 20.0))}\,\mathrm{{J}}",
            np.array([xl, -0.75, 0]), size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        wh_read = math_readout(
            lambda: rf"= {de_num(RHO_AIR * CP_AIR * (theta.get_value() - 20.0) / 3600.0, 3)}\,\mathrm{{Wh}}",
            np.array([xl + 0.42, -1.3, 0]), size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        self.play(FadeIn(thermo["group"]), FadeIn(thermo["column"]), *warming(0.4, convect=False), run_time=0.4)
        self.add(th_read, q_read, wh_read)
        warm = faces(False)
        self.play(Transform(face_top, warm[0]), Transform(face_left, warm[1]), Transform(face_right, warm[2]),
                  theta.animate.set_value(21.0), *warming(3.0), run_time=3.0, rate_func=linear)
        hold_for(self, N, "heat", during=warming)
        #endregion

        #region c_Luft
        caption = swap_caption(self, caption, subtitle_text(N, "c_luft"))
        xr = 1.55
        c_name = Text("Volumenbezogene Wärmekapazität", font_size=LABEL_FONT_SIZE, color=PASTEL_GREEN)
        c_name.move_to([xr + c_name.width / 2, 1.75, 0])
        c_def = math_label(r"c_{\mathrm{Luft}} = \rho \cdot c_{p}", np.array([xr, 1.15, 0]), size=BODY_FONT_SIZE,
                           color=PASTEL_GREEN, edge="left")
        c_j = math_label(rf"= {de_num(RHO_AIR * CP_AIR)}\,\mathrm{{J/(m^{{3}}K)}}", np.array([xr + 0.75, 0.55, 0]),
                         size=BODY_FONT_SIZE, color=PASTEL_GREEN, edge="left")
        c_wh = math_label(rf"\approx {de_num(C_LUFT, 2)}\,\mathrm{{Wh/(m^{{3}}K)}}", np.array([xr + 0.75, -0.05, 0]),
                          size=BODY_FONT_SIZE, color=PASTEL_GREEN, edge="left")
        q_snap = math_label(rf"{de_num(RHO_AIR * CP_AIR)}\,\mathrm{{J}}", size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        q_snap.move_to(q_read.get_right() + LEFT * q_snap.width / 2)
        wh_snap = math_label(rf"{de_num(RHO_AIR * CP_AIR / 3600.0, 3)}\,\mathrm{{Wh}}", size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        wh_snap.move_to(wh_read.get_right() + LEFT * wh_snap.width / 2)
        self.play(FadeIn(c_name), FadeIn(c_def), *warming(0.7), run_time=0.7)
        self.play(TransformFromCopy(q_snap, c_j), *warming(0.9), run_time=0.9)
        self.play(TransformFromCopy(wh_snap, c_wh), *warming(0.9), run_time=0.9)
        hold_for(self, N, "c_luft", during=warming)
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "product"))
        row, box, items = math_panel([
            ("v", "V", PASTEL_ORANGE), (None, r"\cdot", None), ("n", "n", PASTEL_WHITE), (None, r"\cdot", None),
            ("c", r"c_{\mathrm{Luft}}", PASTEL_GREEN), (None, r"\;[\mathrm{W/K}]", None),
        ])
        self.play(ReplacementTransform(v_src, items["v"]), ReplacementTransform(n_src, items["n"]),
                  TransformFromCopy(c_def, items["c"]), FadeOut(top_eq), *warming(1.2), run_time=1.2)
        self.play(FadeIn(VGroup(*[m for m in row if all(m is not items[k] for k in ("v", "n", "c"))])),
                  Create(box), *warming(0.5), run_time=0.5)
        ring = highlight_param(items, "c", color=PASTEL_GREEN)
        self.play(Create(ring), *warming(0.4), run_time=0.4)
        hold_for(self, N, "product", during=warming)
        self.play(FadeOut(ring), *warming(0.2), run_time=0.2)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Ventilation heat loss Φ_V
class Beat5_Lueftungsverlust(Scene):
    """🌬️ Φ_V = V · n · c_Luft · Δθ with every factor of the example house counted up from 0."""

    NARRATION = [
        ("intro",
         "Ventilation heat loss closes the equation with the temperature difference delta theta.",
         "Der Lüftungswärmeverlust schließt die Gleichung mit der Temperaturdifferenz Delta-Theta."),
        ("delta",
         "Delta theta is indoor temperature minus outdoor temperature — the driving force.",
         "Delta-Theta ist Innentemperatur minus Außentemperatur — die Triebkraft."),
        ("formula",
         "Phi V equals V times n times c Luft times delta theta — in watts, per DIN EN 12831-1.",
         "Phi-V ist V mal n mal c-Luft mal Delta-Theta — in Watt, nach DIN EN 12831-1."),
        ("v",
         "V is the net volume of our house: 440 cubic metres.",
         "V ist das Nettovolumen unseres Hauses: 440 Kubikmeter."),
        ("n",
         "n is the air change rate: 0.5 per hour.",
         "n ist die Luftwechselrate: 0,5 pro Stunde."),
        ("c",
         "c Luft is the volumetric heat capacity of air — 0.34 watt-hours per cubic metre and kelvin.",
         "c-Luft ist die volumenbezogene Wärmekapazität der Luft — 0,34 Wh pro Kubikmeter und Kelvin."),
        ("dt",
         "And delta theta is the temperature difference: 20 kelvin.",
         "Und Delta-Theta ist die Temperaturdifferenz: 20 Kelvin."),
        ("result",
         "Together: 440 times 0.5 times 0.34 times 20 — about 1500 watts of ventilation heat loss.",
         "Zusammen: 440 mal 0,5 mal 0,34 mal 20 — rund 1500 Watt Lüftungswärmeverlust."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Lüftungswärmeverlust", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        #region warm cube
        sf, c_center = 0.85, UP * 0.85
        top_pt, tr_pt, br_pt = c_center + UP * sf, c_center + (RIGHT * 0.866 + UP * 0.5) * sf, c_center + (RIGHT * 0.866 + DOWN * 0.5) * sf
        bot_pt, bl_pt, tl_pt = c_center + DOWN * sf, c_center + (LEFT * 0.866 + DOWN * 0.5) * sf, c_center + (LEFT * 0.866 + UP * 0.5) * sf
        cube = VGroup(
            Polygon(c_center, tl_pt, top_pt, tr_pt, fill_color=PASTEL_ORANGE, fill_opacity=0.35, stroke_color=PASTEL_ORANGE, stroke_width=2),
            Polygon(c_center, tl_pt, bl_pt, bot_pt, fill_color=PASTEL_ORANGE, fill_opacity=0.45, stroke_color=PASTEL_ORANGE, stroke_width=2),
            Polygon(c_center, tr_pt, br_pt, bot_pt, fill_color=PASTEL_ORANGE, fill_opacity=0.55, stroke_color=PASTEL_ORANGE, stroke_width=2),
        )
        cube_label = math_label(r"1\,\mathrm{m^{3}}", size=BODY_FONT_SIZE).move_to(cube[0].get_center() + UP * 0.15)
        loss_label = Text("Lüftungsverlust", font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE).next_to(cube, UP, buff=0.2)
        prev_visuals = VGroup(cube, cube_label, loss_label)
        out_paths = [smooth_path([np.array([x, y + dy, 0]) for x, y in ((0.35, 0.95), (1.0, 1.15), (2.0, 1.5))])
                     for dy in (0.0, -0.14)]
        in_paths = [smooth_path([np.array([x, y + dy, 0]) for x, y in ((-2.0, 0.15), (-1.0, 0.35), (-0.35, 0.62))])
                    for dy in (0.0, 0.14)]

        def leak(rt):
            return [_streams([(out_paths, PASTEL_ORANGE), (in_paths, PASTEL_BLUE, PASTEL_ORANGE)], rt, speed=0.5, waves=4)]

        self.play(FadeIn(prev_visuals), *leak(0.8), run_time=0.8)
        hold_for(self, N, "intro", used=0.8, during=leak)
        last = leak(0.8)[0]
        self.play(FadeOut(prev_visuals), last, _fade_flow(last), run_time=0.8)
        #endregion

        #region temperature difference
        caption = swap_caption(self, caption, subtitle_text(N, "delta"))
        th_i = ValueTracker(0.0)
        y_i, y_e, xt = 1.15, 0.05, 0.75
        t_inside = VGroup(
            math_readout(lambda: rf"\theta_{{\mathrm{{i}}}} = {de_num(th_i.get_value())}\,°\mathrm{{C}}",
                         np.array([xt, y_i - 0.08, 0]), size=BODY_FONT_SIZE, color=PASTEL_RED),
            Text("Innentemperatur", font_size=LABEL_FONT_SIZE, color=PASTEL_RED).move_to([xt + 3.3, y_i, 0]),
        )
        t_outside = VGroup(
            math_label(r"\theta_{\mathrm{e}} = 0\,°\mathrm{C}", np.array([xt, y_e - 0.08, 0]), size=BODY_FONT_SIZE,
                       color=PASTEL_BLUE, edge="left"),
            Text("Außentemperatur", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE).move_to([xt + 3.3, y_e, 0]),
        )
        dt_brace = BraceBetweenPoints([xt - 0.3, y_e - 0.12, 0], [xt - 0.3, y_i + 0.12, 0], direction=LEFT, color=PASTEL_YELLOW)
        dt_read = math_readout(lambda: rf"\Delta\theta = \theta_{{\mathrm{{i}}}} - \theta_{{\mathrm{{e}}}} = {de_num(th_i.get_value())}\,\mathrm{{K}}",
                               np.array([xt - 2.25, 0.52, 0]), size=BODY_FONT_SIZE, color=PASTEL_YELLOW, edge="center")
        dt_sub = Text("Temperaturdifferenz", font_size=LABEL_FONT_SIZE, color=PASTEL_YELLOW).move_to([xt - 2.25, 0.02, 0])
        th_in = thermometer_glyph(np.array([6.0, y_i - 0.42, 0]), height=0.8, color=PASTEL_RED, level=0.12)
        th_in["column"].add_updater(lambda m: th_in["level"].set_value(0.12 + 0.8 * th_i.get_value() / D_THETA))
        th_out = thermometer_glyph(np.array([6.0, y_e - 0.42, 0]), height=0.8, color=PASTEL_BLUE, level=0.12)
        t_inside.add(th_in["group"], th_in["column"])
        t_outside.add(th_out["group"], th_out["column"])
        self.play(FadeIn(t_outside, shift=LEFT * 0.2), FadeIn(t_inside[1:], shift=LEFT * 0.2), GrowFromCenter(dt_brace),
                  FadeIn(dt_sub), run_time=1.0)
        self.add(t_inside[0], dt_read)
        self.play(th_i.animate.set_value(D_THETA), run_time=1.4)
        hold_for(self, N, "delta")
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        row, box, items = math_panel([
            ("phi", r"\Phi_{\mathrm{V}}", PASTEL_WHITE), (None, "=", None),
            ("v", "V", PASTEL_ORANGE), (None, r"\cdot", None), ("n", "n", PASTEL_CYAN), (None, r"\cdot", None),
            ("c", r"c_{\mathrm{Luft}}", PASTEL_GREEN), (None, r"\cdot", None), ("dt", r"\Delta\theta", PASTEL_YELLOW),
            (None, r"\;[\mathrm{W}]", None),
        ])
        dt_snap = math_label(r"\Delta\theta", size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        dt_snap.move_to(dt_read.get_left() + RIGHT * dt_snap.width / 2)
        self.remove(dt_read)
        self.add(dt_snap)
        self.play(ReplacementTransform(dt_snap, items["dt"]),
                  FadeOut(t_inside), FadeOut(t_outside), FadeOut(dt_brace), FadeOut(dt_sub), run_time=1.0)
        self.play(FadeIn(VGroup(*[m for m in row if m is not items["dt"]])), Create(box), run_time=0.6)
        hold_for(self, N, "formula")
        #endregion

        #region factor cards counted from 0
        specs = [
            ("v", "V", "Nettovolumen", V_NET, 0, r"\mathrm{m^{3}}", PASTEL_ORANGE),
            ("n", "n", "Luftwechselrate", N_RATE, 1, r"\mathrm{h^{-1}}", PASTEL_CYAN),
            ("c", r"c_{\mathrm{Luft}}", "Wärmekapazität Luft", C_LUFT, 2, r"\mathrm{Wh/(m^{3}K)}", PASTEL_GREEN),
            ("dt", r"\Delta\theta", "Temperaturdifferenz", D_THETA, 0, r"\mathrm{K}", PASTEL_YELLOW),
        ]
        trackers = {key: ValueTracker(0.0) for key, *_ in specs}
        card_w, gap = 2.85, 0.22
        xs = [(-1.5 * (card_w + gap)) + i * (card_w + gap) for i in range(4)]
        cy = 0.95
        cards = {}
        for (key, sym, name, value, digits, unit, color), x in zip(specs, xs):
            frame = RoundedRectangle(width=card_w, height=1.62, corner_radius=0.1, color=color, stroke_width=1.8)
            frame.set_fill(P_DEEP_DARK, 0.82).move_to([x, cy, 0])
            head = math_label(sym, np.array([x, cy + 0.32, 0]), size=FORMULA_FONT_SIZE, color=color)
            cap = Text(name, font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE).move_to([x, cy - 0.12, 0])
            val = math_readout(lambda key=key, digits=digits, unit=unit: rf"{de_num(trackers[key].get_value(), digits)}\,{unit}",
                               np.array([x, cy - 0.55, 0]), size=BODY_FONT_SIZE, color=color, edge="center")
            cards[key] = (VGroup(frame, head, cap), val, value)

        for key, *_ in specs:
            static, val, value = cards[key]
            ring = highlight_param(items, key, color=dict((s[0], s[6]) for s in specs)[key])
            self.play(Create(ring), FadeIn(static, shift=UP * 0.1), run_time=0.5)
            self.add(val)
            self.play(trackers[key].animate.set_value(value), run_time=1.0)
            caption = swap_caption(self, caption, subtitle_text(N, key))
            hold_for(self, N, key)
            self.play(FadeOut(ring), run_time=0.2)
        #endregion

        #region result
        caption = swap_caption(self, caption, subtitle_text(N, "result"))
        k = ValueTracker(0.0)
        phi_read = math_readout(
            lambda: rf"\Phi_{{\mathrm{{V}}}} = {de_num(V_NET)} \cdot {de_num(N_RATE, 1)} \cdot {de_num(C_LUFT, 2)} \cdot "
                    rf"{de_num(D_THETA * k.get_value())} = {de_num(V_NET * N_RATE * C_LUFT * D_THETA * k.get_value())}\,\mathrm{{W}}",
            np.array([0.0, -0.55, 0]), size=FORMULA_FONT_SIZE, color=PASTEL_RED, edge="center")
        self.add(phi_read)
        ring = highlight_param(items, "phi", color=PASTEL_RED)
        self.play(k.animate.set_value(1.0), Create(ring), run_time=1.6)
        hold_for(self, N, "result")
        self.play(FadeOut(ring), run_time=0.2)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Heat recovery η_WRG
class Beat6_Waermerueckgewinnung(Scene):
    """♻️ η_WRG sweeps 0 → 1 → 0,8: the heat exchanger warms the supply air and the remaining loss shrinks."""

    NARRATION = [
        ("without",
         "Without heat recovery, the full ventilation loss applies — eta equals zero.",
         "Ohne Wärmerückgewinnung gilt der volle Lüftungsverlust — Eta gleich null."),
        ("with",
         "With heat recovery, multiply by one minus eta WRG — only the unrecovered fraction remains.",
         "Mit WRG multiplizieren wir mit eins minus Eta-WRG — nur der Restverlust bleibt."),
        ("sweep",
         "The exhaust air passes its heat to the fresh air: the higher eta, the warmer the supply air.",
         "Die Abluft gibt ihre Wärme an die Außenluft ab: je höher Eta, desto wärmer die Zuluft."),
        ("bars",
         "At eighty percent recovery, only twenty percent remains: about 300 instead of 1500 watts.",
         "Bei achtzig Prozent Rückgewinnung bleiben zwanzig Prozent: rund 300 statt 1500 Watt."),
        ("eta",
         "Eta WRG is typically seventy to ninety percent for modern systems.",
         "Eta-WRG liegt typisch bei siebzig bis neunzig Prozent bei modernen Anlagen."),
        ("phi",
         "Phi V is then the reduced ventilation heating load in watts.",
         "Phi-V ist dann die reduzierte Lüftungsheizlast in Watt."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmerückgewinnung (WRG)", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "without"))
        self.play(FadeIn(caption), run_time=0.3)

        #region formulas
        row_old, box_old, _items_old = math_panel([
            ("phi", r"\Phi_{\mathrm{V}}", PASTEL_RED), (None, "=", None),
            ("v", "V", PASTEL_ORANGE), (None, r"\cdot", None), ("n", "n", PASTEL_WHITE), (None, r"\cdot", None),
            ("c", r"c_{\mathrm{Luft}}", PASTEL_GREEN), (None, r"\cdot", None), ("dt", r"\Delta\theta", PASTEL_YELLOW),
            (None, r"\;[\mathrm{W}]", None),
        ])
        old_note = math_label(r"\text{ohne WRG:}\; \eta_{\mathrm{WRG}} = 0", size=LABEL_FONT_SIZE, color=GREY_A)
        old_note.next_to(box_old, UP, buff=0.12)
        self.play(FadeIn(row_old), Create(box_old), FadeIn(old_note), run_time=1.3)
        hold_for(self, N, "without", used=1.3 + 0.3)

        row, box, items = math_panel([
            ("phi", r"\Phi_{\mathrm{V}}", PASTEL_RED), (None, "=", None),
            ("v", "V", PASTEL_ORANGE), (None, r"\cdot", None), ("n", "n", PASTEL_WHITE), (None, r"\cdot", None),
            ("eta", r"(1-\eta_{\mathrm{WRG}})", PASTEL_GREEN), (None, r"\cdot", None),
            ("c", r"c_{\mathrm{Luft}}", PASTEL_GREEN), (None, r"\cdot", None), ("dt", r"\Delta\theta", PASTEL_YELLOW),
            (None, r"\;[\mathrm{W}]", None),
        ])
        caption = swap_caption(self, caption, subtitle_text(N, "with"))
        self.play(ReplacementTransform(row_old, row), ReplacementTransform(box_old, box), FadeOut(old_note), run_time=1.2)
        ring_eta = highlight_param(items, "eta", color=PASTEL_GREEN)
        self.play(Create(ring_eta), run_time=0.4)
        hold_for(self, N, "with")
        self.play(FadeOut(ring_eta), run_time=0.2)
        #endregion

        #region heat exchanger and live bars
        caption = swap_caption(self, caption, subtitle_text(N, "sweep"))
        eta = ValueTracker(0.0)
        hx_c = np.array([0.0, 1.25, 0.0])
        hx = Rectangle(width=1.3, height=0.9, color=PASTEL_WHITE, stroke_width=2.4).set_fill(P_DEEP_DARK, 1.0).move_to(hx_c)
        plates = VGroup(*[Line([x, hx.get_bottom()[1] + 0.07, 0], [x, hx.get_top()[1] - 0.07, 0], color=PASTEL_TEAL,
                               stroke_width=1.2, stroke_opacity=0.55) for x in np.linspace(-0.5, 0.5, 6)])
        hx_lbl = Text("WRG", font_size=LABEL_FONT_SIZE, color=PASTEL_YELLOW).next_to(hx, UP, buff=0.12)
        y_top, y_bot, x_far = hx_c[1] + 0.25, hx_c[1] - 0.25, 3.3

        def mix(frac):
            return interpolate_color(ManimColor(PASTEL_BLUE), ManimColor(PASTEL_ORANGE), float(np.clip(frac, 0, 1)))

        def through(start, end_fn):
            return lambda t: interpolate_color(ManimColor(start), end_fn(), float(np.clip((t - 0.4) / 0.2, 0, 1)))

        walls = VGroup(*[Line([sx * x_far, y + dy, 0], [sx * 0.65, y + dy, 0], color=PASTEL_WHITE, stroke_width=1.4,
                              stroke_opacity=0.6) for sx in (-1, 1) for y in (y_top, y_bot) for dy in (-0.12, 0.12)])
        fans = [_fan([-2.0, y_top, 0], PASTEL_WHITE, radius=0.15), _fan([2.0, y_bot, 0], PASTEL_WHITE, radius=0.15)]
        ducts = VGroup(walls, *fans)
        air = [
            ([smooth_path([np.array([x_far, y_top, 0]), np.array([-x_far, y_top, 0])])],
             through(PASTEL_ORANGE, lambda: mix(1 - eta.get_value()))),
            ([smooth_path([np.array([-x_far, y_bot, 0]), np.array([x_far, y_bot, 0])])],
             through(PASTEL_BLUE, lambda: mix(eta.get_value()))),
        ]

        def exchanger(rt):
            return [_live_flow(air, rt, speed=0.35, waves=7), *_spin(fans, rt)]

        lx_r, lx_l = x_far + 0.15, -x_far - 0.15
        labels = VGroup(
            math_label(r"\text{Abluft}\; 20\,°\mathrm{C}", np.array([lx_r, y_top - 0.07, 0]), size=LABEL_FONT_SIZE,
                       color=PASTEL_ORANGE, edge="left"),
            math_label(r"\text{Außenluft}\; 0\,°\mathrm{C}", np.array([lx_l, y_bot - 0.07, 0]), size=LABEL_FONT_SIZE,
                       color=PASTEL_BLUE, edge="right"),
        )
        zu_read = math_readout(lambda: rf"\text{{Zuluft}}\; {de_num(D_THETA * eta.get_value())}\,°\mathrm{{C}}",
                               np.array([lx_r, y_bot - 0.07, 0]), size=LABEL_FONT_SIZE, color=PASTEL_GREEN)
        fort_read = math_readout(lambda: rf"\text{{Fortluft}}\; {de_num(D_THETA * (1 - eta.get_value()))}\,°\mathrm{{C}}",
                                 np.array([lx_l, y_top - 0.07, 0]), size=LABEL_FONT_SIZE, color=GREY_A, edge="right")
        bx, blen, yb1, yb2 = -2.3, 4.6, -0.3, -1.0
        bar1 = Rectangle(width=blen, height=0.42, color=PASTEL_RED, stroke_width=1.2).set_fill(PASTEL_RED, 0.6)
        bar1.move_to([bx + blen / 2, yb1, 0])
        bar1_lbl = Text("ohne WRG", font_size=LABEL_FONT_SIZE, color=PASTEL_RED)
        bar1_lbl.move_to([bx - 0.2 - bar1_lbl.width / 2, yb1, 0])
        bar1_val = math_label(rf"{de_num(PHI_V)}\,\mathrm{{W}}", np.array([bx + blen + 0.18, yb1 - 0.08, 0]),
                              size=BODY_FONT_SIZE, color=PASTEL_RED, edge="left")

        def split():
            e = float(np.clip(eta.get_value(), 0, 1))
            parts = VGroup()
            if e > 0.002:
                parts.add(Rectangle(width=blen * e, height=0.42, stroke_width=1.2, color=PASTEL_GREEN, fill_color=PASTEL_GREEN,
                                    fill_opacity=0.6).move_to([bx + blen * e / 2, yb2, 0]))
            if e < 0.998:
                parts.add(Rectangle(width=blen * (1 - e), height=0.42, stroke_width=1.2, color=PASTEL_RED, fill_color=PASTEL_RED,
                                    fill_opacity=0.6).move_to([bx + blen * e + blen * (1 - e) / 2, yb2, 0]))
            return parts

        bar2 = always_redraw(split)
        eta_read = math_readout(lambda: rf"\eta_{{\mathrm{{WRG}}}} = {de_num(eta.get_value(), 2)}",
                                np.array([bx - 0.2, yb2 - 0.08, 0]), size=BODY_FONT_SIZE, color=PASTEL_GREEN, edge="right")
        phi_read = math_readout(lambda: rf"\Phi_{{\mathrm{{V}}}} = {de_num(PHI_V * (1 - eta.get_value()))}\,\mathrm{{W}}",
                                np.array([bx + blen + 0.18, yb2 - 0.08, 0]), size=BODY_FONT_SIZE, color=PASTEL_RED)
        self.play(FadeIn(hx), FadeIn(plates), FadeIn(hx_lbl), FadeIn(ducts), FadeIn(labels), FadeIn(zu_read),
                  FadeIn(fort_read), run_time=1.0)
        self.play(FadeIn(bar1), FadeIn(bar1_lbl), FadeIn(bar1_val), run_time=0.6)
        self.add(bar2, eta_read, phi_read)
        self.play(eta.animate(rate_func=smooth).set_value(1.0), *exchanger(2.4), run_time=2.4)
        self.play(eta.animate(rate_func=smooth).set_value(ETA_WRG), *exchanger(1.2), run_time=1.2)
        hold_for(self, N, "sweep", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "bars"))
        saved_lbl = math_label(rf"{de_num(100 * ETA_WRG)}\,\%\;\text{{eingespart}}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        saved_lbl.move_to([bx + blen * ETA_WRG / 2, yb2, 0])
        rest_lbl = math_label(rf"{de_num(100 * (1 - ETA_WRG))}\,\%", size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
        rest_lbl.move_to([bx + blen * ETA_WRG + blen * (1 - ETA_WRG) / 2, yb2, 0])
        self.play(FadeIn(saved_lbl), FadeIn(rest_lbl), *exchanger(2.4), run_time=2.4)
        hold_for(self, N, "bars", during=exchanger)
        #endregion

        #region cards
        eta_snap = math_label(rf"\eta_{{\mathrm{{WRG}}}} = {de_num(ETA_WRG, 2)}", size=BODY_FONT_SIZE, color=PASTEL_GREEN)
        eta_snap.move_to(eta_read.get_right() + LEFT * eta_snap.width / 2)
        phi_snap = math_label(rf"\Phi_{{\mathrm{{V}}}} = {de_num(PHI_V * (1 - ETA_WRG))}\,\mathrm{{W}}", size=BODY_FONT_SIZE,
                              color=PASTEL_RED)
        phi_snap.move_to(phi_read.get_left() + RIGHT * phi_snap.width / 2)
        self.remove(eta_read, phi_read)
        self.add(eta_snap, phi_snap)
        self.play(*[FadeOut(m) for m in (hx, plates, hx_lbl, ducts, labels, zu_read, fort_read)], run_time=0.8)

        def card(lines, color, x):
            body = VGroup(*lines).arrange(DOWN, buff=0.12)
            frame = RoundedRectangle(width=4.6, height=body.height + 0.4, corner_radius=0.1, color=color, stroke_width=1.8)
            frame.set_fill(P_DEEP_DARK, 0.82).set_z_index(-1)
            return VGroup(frame, body).move_to([x, 1.2, 0])

        card_eta = card([
            math_label(r"\eta_{\mathrm{WRG}}", size=FORMULA_FONT_SIZE, color=PASTEL_GREEN),
            Text("Wärmerückgewinnungsgrad", color=PASTEL_WHITE, font_size=LABEL_FONT_SIZE),
            math_label(r"\text{typisch}\; 0{,}70 \ldots 0{,}90", size=LABEL_FONT_SIZE, color=GREY_A),
            Text("DIN EN 13141-7 / DIN 1946-6", color=PASTEL_CYAN, font_size=LABEL_FONT_SIZE),
        ], PASTEL_GREEN, -2.6)
        card_phi = card([
            math_label(r"\Phi_{\mathrm{V}}", size=FORMULA_FONT_SIZE, color=PASTEL_RED),
            Text("Reduzierte Lüftungsheizlast", color=PASTEL_WHITE, font_size=LABEL_FONT_SIZE),
            math_label(rf"{de_num(PHI_V)} \cdot (1 - {de_num(ETA_WRG, 1)}) \approx {de_num(PHI_V * (1 - ETA_WRG))}\,\mathrm{{W}}",
                       size=LABEL_FONT_SIZE, color=GREY_A),
            Text("DIN EN 12831-1", color=PASTEL_CYAN, font_size=LABEL_FONT_SIZE),
        ], PASTEL_RED, 2.6)

        caption = swap_caption(self, caption, subtitle_text(N, "eta"))
        ring_eta2 = highlight_param(items, "eta", color=PASTEL_GREEN)
        self.play(Create(ring_eta2), FadeIn(card_eta[0]), TransformFromCopy(eta_snap, card_eta[1][0]),
                  FadeIn(card_eta[1][1:], shift=UP * 0.15), run_time=1.0)
        hold_for(self, N, "eta")
        self.play(FadeOut(ring_eta2), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(N, "phi"))
        ring_phi = highlight_param(items, "phi", color=PASTEL_RED)
        self.play(Create(ring_phi), FadeIn(card_phi[0]), TransformFromCopy(phi_snap, card_phi[1][0]),
                  FadeIn(card_phi[1][1:], shift=UP * 0.15), run_time=1.0)
        hold_for(self, N, "phi")
        self.play(FadeOut(ring_phi), run_time=0.2)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 – Purge ventilation versus tilted window
ROOM_A, ROOM_H, ROOM_D = 4.0 * 5.0, 2.5, 5.0
T_STOSS, T_KIPP = 5.0, 60.0
P_RAD, RAD_SHARE = 500.0, 0.5
M_LAIBUNG, C_MASONRY, DT_LAIBUNG = 160.0, 0.84, 5.0


class Beat7_KippOderStoss(Scene):
    """⏱️ Same 50 m³ of fresh air: wide-open window in 5 min versus tilted window in 60 min — and what each costs."""

    NARRATION = [
        ("room",
         "A room of 20 square metres and 2.5 metres height holds 50 cubic metres of air; inside 20, outside 0 degrees.",
         "Ein Raum mit 20 m² und 2,5 m Höhe enthält 50 m³ Luft. Innen 20 °C, außen 0 °C."),
        ("open",
         "Two ways to air it: the thermostat turned down and the window wide open — or the window tilted.",
         "Zwei Arten zu lüften: Thermostat zu und Fenster weit auf — oder Fenster gekippt."),
        ("five",
         "After five minutes the wide-open window has exchanged the whole air once; the tilted one only about eight percent.",
         "Nach fünf Minuten ist beim weit offenen Fenster die ganze Luft getauscht, beim gekippten erst etwa 8 %."),
        ("stoss",
         "Purge ventilation only reheats the air: 0.34 times 50 times 20 is 340 watt-hours. The walls stay warm.",
         "Beim Stoßlüften wird nur die Luft nachgeheizt: 0,34 · 50 · 20 = 340 Wh. Die Wände bleiben warm."),
        ("hour",
         "The tilted window needs about an hour for one full air change.",
         "Das gekippte Fenster braucht für einen vollen Luftwechsel etwa eine Stunde."),
        ("thermostat",
         "Cold air falls onto the thermostat below the window; the valve opens and half of 500 watts flows straight out: 250 watt-hours.",
         "Kalte Luft fällt auf das Thermostat unter dem Fenster. Das Ventil öffnet, die Hälfte von 500 W geht direkt hinaus: 250 Wh."),
        ("laibung",
         "Reveal and lintel cool by about 5 kelvin; reheating 160 kilograms of masonry costs about 190 watt-hours.",
         "Laibung und Sturz kühlen um etwa 5 K ab. 160 kg Mauerwerk wieder zu erwärmen kostet rund 190 Wh."),
        ("sum",
         "For the same 50 cubic metres of fresh air: 340 watt-hours purge, about 780 tilted.",
         "Für dieselben 50 m³ Frischluft: 340 Wh beim Stoßlüften, rund 780 Wh bei Kipplüftung."),
        ("rule",
         "So open the window wide for a short time instead of tilting it.",
         "Darum: kurz und weit öffnen statt kippen."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        vol = ROOM_A * ROOM_H
        e_air = C_LUFT * vol * D_THETA
        e_rad = P_RAD * (T_KIPP / 60.0) * RAD_SHARE
        e_lb = M_LAIBUNG * C_MASONRY * DT_LAIBUNG / 3.6
        e_kipp = e_air + e_rad + e_lb

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Stoßlüften oder Kipplüftung", title)
        din = _din_ref("DIN 1946-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "room"))
        self.play(FadeIn(caption), run_time=0.3)

        #region two rooms
        k, fy = 0.85, -0.45
        rs = _vent_room(-5.9, fy, k, r"\text{Stoßlüften}", PASTEL_CYAN)
        rk = _vent_room(1.3, fy, k, r"\text{Kipplüftung}", PASTEL_ORANGE)
        note = math_label(rf"V = 4\,\mathrm{{m}} \cdot {de_num(ROOM_D)}\,\mathrm{{m}} \cdot {de_num(ROOM_H, 1)}\,\mathrm{{m}} = "
                          rf"{de_num(vol)}\,\mathrm{{m^{{3}}}} \quad\quad \Delta\theta = {de_num(D_THETA)}\,\mathrm{{K}}",
                          size=LABEL_FONT_SIZE, color=PASTEL_WHITE).move_to([-0.3, 2.45, 0])
        t_s, t_k = ValueTracker(0.0), ValueTracker(0.0)
        valve_s, valve_k = ValueTracker(0.6), ValueTracker(0.6)
        cool = ValueTracker(0.0)
        clocks = {}
        for room, tr, col in ((rs, t_s, PASTEL_CYAN), (rk, t_k, PASTEL_ORANGE)):
            clocks[id(room)] = _clock([room["ox"] + 0.64, room["ceil"] - 0.5, 0], 0.32, tr, color=col)
        reads = VGroup(*[
            math_readout(lambda tr=tr: rf"t = {de_num(tr.get_value())}\,\mathrm{{min}}",
                         np.array([room["ox"] + 0.64, room["ceil"] - 1.12, 0]), size=LABEL_FONT_SIZE, color=col, edge="center")
            for room, tr, col in ((rs, t_s, PASTEL_CYAN), (rk, t_k, PASTEL_ORANGE))
        ])
        frac_s = lambda: min(1.0, t_s.get_value() / T_STOSS)
        frac_k = lambda: min(1.0, t_k.get_value() / T_KIPP)
        gauges = VGroup(*[_vbar(r["ox"] - 0.5, fy, r["ceil"] - fy, f, color=PASTEL_CYAN) for r, f in ((rs, frac_s), (rk, frac_k))])
        pct = VGroup(*[
            math_readout(lambda f=f: rf"{de_num(100 * f())}\,\%", np.array([room["ox"] - 0.5, room["ceil"] + 0.24, 0]),
                         size=LABEL_FONT_SIZE, color=PASTEL_CYAN, edge="center")
            for room, f in ((rs, frac_s), (rk, frac_k))
        ])
        dials = VGroup(_valve_dial(rs, valve_s.get_value), _valve_dial(rk, valve_k.get_value))
        flows, state = {}, {"air": [], "heat": [rs, rk]}

        def running(rt):
            streams = [st for key in state["air"] for st in flows[key]]
            heat = [ripples([r["rad"][0].get_top() + UP * 0.03 for r in state["heat"]], r_max=0.3, rings=2,
                            cycles=rt / 1.3, color=PASTEL_RED)] if state["heat"] else []
            return ([_streams(streams, rt, speed=0.55, waves=5)] if streams else []) + heat

        self.play(FadeIn(rs["group"]), FadeIn(rk["group"]), FadeIn(rs["head"]), FadeIn(rk["head"]), FadeIn(note),
                  *running(1.4), run_time=1.4)
        self.play(*[FadeIn(c["group"]) for c in clocks.values()], FadeIn(reads), FadeIn(gauges), FadeIn(pct),
                  FadeIn(dials), *running(0.8), run_time=0.8)
        hold_for(self, N, "room", used=2.2, during=running)
        #endregion

        #region open the windows
        caption = swap_caption(self, caption, subtitle_text(N, "open"))
        hinge = rk["sash"].get_bottom()
        state["heat"] = [rk]
        self.play(valve_s.animate.set_value(0.0), rs["rad"].animate.set_stroke(GREY_B), *running(0.8), run_time=0.8)
        self.play(_open_sash(rs["window"]), Rotate(rk["sash"], angle=0.17, about_point=hinge), *running(0.9),
                  run_time=0.9)
        hold_for(self, N, "open", during=running)
        #endregion

        #region first five minutes
        caption = swap_caption(self, caption, subtitle_text(N, "five"))
        unit, bx = 8.0 / 780.0, -4.3
        yr_s, yr_k = -1.2, -1.88
        row_s = _energy_row(bx, yr_s, unit, [(lambda: e_air * frac_s(), PASTEL_CYAN)])
        row_k = _energy_row(bx, yr_k, unit, [
            (lambda: e_air * frac_k(), PASTEL_CYAN),
            (lambda: P_RAD * RAD_SHARE * t_k.get_value() / 60.0, PASTEL_RED),
            (lambda: M_LAIBUNG * C_MASONRY * DT_LAIBUNG * cool.get_value() / 3.6, PASTEL_YELLOW),
        ])
        row_labels = VGroup(
            Text("Stoßlüften", font_size=LABEL_FONT_SIZE, color=PASTEL_CYAN),
            Text("Kipplüftung", font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE),
        )
        for lbl, y in zip(row_labels, (yr_s, yr_k)):
            lbl.move_to([bx - 0.18 - lbl.width / 2, y, 0])

        def total_k():
            return e_air * frac_k() + P_RAD * RAD_SHARE * t_k.get_value() / 60.0 + \
                M_LAIBUNG * C_MASONRY * DT_LAIBUNG * cool.get_value() / 3.6

        tot_s = math_readout(lambda: rf"{de_num(e_air * frac_s())}\,\mathrm{{Wh}}",
                             lambda: np.array([bx + e_air * frac_s() * unit + 0.15, yr_s - 0.08, 0]),
                             size=LABEL_FONT_SIZE, color=PASTEL_CYAN)
        tot_k = math_readout(lambda: rf"{de_num(total_k())}\,\mathrm{{Wh}}",
                             lambda: np.array([bx + total_k() * unit + 0.15, yr_k - 0.08, 0]),
                             size=LABEL_FONT_SIZE, color=PASTEL_ORANGE)
        self.play(FadeIn(row_labels), *running(0.4), run_time=0.4)
        self.add(row_s, row_k, tot_s, tot_k)

        def ext(room, x_off, y):
            return np.array([room["x_out"] + x_off, y, 0])

        def inn(room, x_off, y):
            return np.array([room["x_in"] - x_off, y, 0])

        mid_s = (rs["x_in"] + rs["x_out"]) / 2
        stoss_paths_out = [smooth_path([inn(rs, 1.4, y - 0.15), np.array([mid_s, y, 0]), ext(rs, 0.9, y + 0.3)])
                           for y in (0.98, 1.12, 1.26)]
        stoss_paths_in = [smooth_path([ext(rs, 0.9, y - 0.1), np.array([mid_s, y, 0]), inn(rs, 0.6, y - 0.15),
                                       inn(rs, 1.5, fy + 0.75), inn(rs, 2.4, fy + 0.15)]) for y in (0.52, 0.66, 0.8)]
        mid_k = (rk["x_in"] + rk["x_out"]) / 2
        gap_y = rk["lintel"] - 0.06
        kipp_out = [smooth_path([inn(rk, 0.9, gap_y - 0.25), np.array([mid_k, gap_y, 0]), ext(rk, 0.8, gap_y + 0.3)])]
        kipp_in = [smooth_path([ext(rk, 0.7, gap_y - 0.1), np.array([mid_k + 0.03, gap_y - 0.02, 0]),
                                inn(rk, 0.12, gap_y - 0.25), inn(rk, 0.1, rk["valve"].get_center()[1] + 0.1)])]
        heat_out = [smooth_path([np.array([rk["rad"].get_center()[0], rk["rad"][0].get_top()[1] + 0.05, 0]),
                                 inn(rk, 0.35, rk["sill"] + 0.45), np.array([mid_k, gap_y + 0.01, 0]),
                                 ext(rk, 0.8, gap_y + 0.38)])]
        flows.update({"s": [(stoss_paths_out, PASTEL_ORANGE), (stoss_paths_in, PASTEL_BLUE)],
                      "k": [(kipp_out, PASTEL_ORANGE), (kipp_in, PASTEL_BLUE)], "h": [(heat_out, PASTEL_RED)]})
        state["air"] = ["s", "k"]
        self.play(t_s.animate.set_value(T_STOSS), t_k.animate.set_value(T_STOSS), valve_k.animate.set_value(0.75),
                  cool.animate.set_value(T_STOSS / T_KIPP), *running(4.0), run_time=4.0, rate_func=linear)
        hold_for(self, N, "five", during=running)
        #endregion

        #region purge done
        caption = swap_caption(self, caption, subtitle_text(N, "stoss"))
        air_s_lbl = math_label(rf"\text{{Luft}}\; {de_num(e_air)}\,\mathrm{{Wh}}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        air_s_lbl.move_to([bx + e_air * unit / 2, yr_s, 0])
        state["air"] = ["k"]
        self.play(_open_sash(rs["window"], run_time=1.0, close=True), valve_s.animate.set_value(0.6),
                  rs["rad"].animate.set_stroke(PASTEL_RED), FadeIn(air_s_lbl), *running(1.0), run_time=1.0)
        state["heat"] = [rs, rk]
        hold_for(self, N, "stoss", during=running)
        #endregion

        #region tilted window: the full hour
        caption = swap_caption(self, caption, subtitle_text(N, "hour"))

        def laibung_color():
            return interpolate_color(ManimColor(PASTEL_ORANGE), ManimColor(PASTEL_BLUE), float(cool.get_value()))

        rk["sturz"].add_updater(lambda m: m.set_fill(laibung_color(), opacity=0.7))
        rk["laibung"].add_updater(lambda m: m.set_fill(laibung_color(), opacity=0.3))
        state["air"] = ["k", "h"]
        self.play(t_k.animate.set_value(T_KIPP), valve_k.animate.set_value(1.0), cool.animate.set_value(1.0),
                  *running(6.0), run_time=6.0, rate_func=linear)
        hold_for(self, N, "hour", during=running)
        #endregion

        #region thermostat and reveal
        caption = swap_caption(self, caption, subtitle_text(N, "thermostat"))
        rad_lbl = math_label(rf"\dot{{Q}} = {de_num(P_RAD)}\,\mathrm{{W}} \cdot {de_num(100 * RAD_SHARE)}\,\%",
                             np.array([rk["ox"] + 0.15, fy + 0.2, 0]), size=LABEL_FONT_SIZE, color=PASTEL_RED, edge="left")
        seg_rad = math_label(rf"\text{{Heizkörper}}\; {de_num(e_rad)}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_rad.move_to([bx + (e_air + e_rad / 2) * unit, yr_k, 0])
        seg_air = math_label(rf"\text{{Luft}}\; {de_num(e_air)}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_air.move_to([bx + e_air * unit / 2, yr_k, 0])
        self.play(Indicate(rk["valve"], color=PASTEL_YELLOW, scale_factor=1.8), FadeIn(rad_lbl), FadeIn(seg_air),
                  FadeIn(seg_rad), *running(1.0), run_time=1.0)
        hold_for(self, N, "thermostat", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "laibung"))
        lb_lbl = math_label(rf"\Delta T = {de_num(DT_LAIBUNG)}\,\mathrm{{K}}", np.array([rk["x_out"] + 0.2, rk["sill"] + 0.05, 0]),
                            size=LABEL_FONT_SIZE, color=PASTEL_BLUE, edge="left")
        seg_lb = math_label(rf"\text{{Laibung}}\; {de_num(round(e_lb, -1))}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_lb.move_to([bx + (e_air + e_rad + e_lb / 2) * unit, yr_k, 0])
        self.play(Indicate(VGroup(rk["sturz"], rk["laibung"]), color=PASTEL_BLUE, scale_factor=1.05), FadeIn(lb_lbl),
                  FadeIn(seg_lb), *running(1.0), run_time=1.0)
        hold_for(self, N, "laibung", during=running)
        #endregion

        #region sum and rule
        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        tot_k.clear_updaters()
        tot_k_final = math_label(rf"\approx {de_num(round(e_kipp, -1))}\,\mathrm{{Wh}}", size=LABEL_FONT_SIZE, color=PASTEL_ORANGE)
        tot_k_final.move_to(tot_k.get_left() + RIGHT * tot_k_final.width / 2)
        ratio = math_label(rf"\approx {de_num(e_kipp / e_air, 1)} \times", size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        ratio.next_to(tot_k_final, RIGHT, buff=0.3)
        self.play(ReplacementTransform(tot_k, tot_k_final), *running(0.6), run_time=0.6)
        self.play(TransformFromCopy(row_s, ratio), *running(0.8), run_time=0.8)
        hold_for(self, N, "sum", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "rule"))
        rule = Text("kurz und weit öffnen statt kippen", font_size=BODY_FONT_SIZE, color=PASTEL_GREEN)
        rule_box = SurroundingRectangle(rule, color=PASTEL_GREEN, buff=0.14, corner_radius=0.1, stroke_width=1.8)
        VGroup(rule, rule_box).move_to([3.3, yr_s, 0])
        self.play(FadeOut(rad_lbl), FadeIn(rule), Create(rule_box), *running(0.8), run_time=0.8)
        hold_for(self, N, "rule", during=running)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – Ventilation systems compared
class Beat8_Lueftungssysteme(Scene):
    NARRATION = [
        ("window",
         "Window ventilation: free air change with zero heat recovery — full ventilation loss.",
         "Fensterlüftung: freier Luftwechsel ohne WRG — voller Lüftungsverlust."),
        ("central",
         "A central WRG unit preheats supply air and cuts loss to roughly ten percent.",
         "Eine zentrale WRG-Anlage vorwärmt die Zuluft und senkt den Verlust auf etwa zehn Prozent."),
        ("decentral",
         "Decentralized ceramic push-pull units recover seventy to ninety percent room by room.",
         "Dezentrale Keramik-Pendellüfter rückgewinnen siebzig bis neunzig Prozent raumweise."),
        ("outro",
         "DIN 1946-6: mechanical WRG can cut ventilation heat loss by up to ninety percent.",
         "DIN 1946-6: mechanische WRG kann den Lüftungswärmeverlust um bis zu 90% senken."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Lüftungssysteme im Vergleich", title)
        din = _din_ref("DIN 1946-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "window"))
        self.play(FadeIn(caption), run_time=0.3)

        N = self.NARRATION
        col_w, col_h = 4.3, 4.6
        pivot = np.array([0.0, -0.25, 0.0])

        def place(*mobs):
            for m in mobs:
                m.scale(0.85, about_point=pivot).shift(UP * 0.35)

        def path(*pts):
            return smooth_path([np.array([x, y, 0.0]) for x, y in pts])

        def legend(lines, color, scale, at):
            return VGroup(*[Text(t, font_size=LABEL_FONT_SIZE, color=color) for t in lines]).arrange(DOWN, buff=0.15)\
                .scale(scale).move_to(at)

        def panel(px, color, fill, main, sub, stats):
            card = RoundedRectangle(width=col_w, height=col_h, corner_radius=0.15, color=color, fill_color=fill,
                                    fill_opacity=0.85).move_to([px, -0.25, 0])
            hdr = VGroup(Text(main, font_size=LABEL_FONT_SIZE, color=color),
                         Text(sub, font_size=LABEL_FONT_SIZE, color=GREY_A)).arrange(DOWN, buff=0.05)
            hdr.move_to(card.get_top() + DOWN * 0.45)
            box = VGroup(*stats).arrange(DOWN, buff=0.1).move_to(card.get_bottom() + UP * 0.4)
            room = room_section(np.array([px + 0.35, -0.85, 0.0]), w=2.3, h=1.4, wall=0.13, slab=0.13, window=(0.3, 0.75))
            for part in room["shell"]:
                part.set_stroke(PASTEL_WHITE).set_fill(PASTEL_WHITE, opacity=0.14)
            room["glass"].set_stroke(PASTEL_CYAN).set_fill(PASTEL_CYAN, opacity=0.15)
            person = person_glyph([px + 1.15, room["y_f"] + 0.3, 0], color=PASTEL_ORANGE, scale=0.9)
            return card, hdr, box, room, person

        # Panel 1 — Fensterlüftung
        p1 = -4.4
        card1, hdr1, box1_stat, room1, person1 = panel(
            p1, PASTEL_RED, "#1a1215", "Fensterlüftung", "Freie Lüftung ohne WRG",
            [math_label(r"\eta_{\mathrm{WRG}} = 0\,\%", size=LABEL_FONT_SIZE, color=PASTEL_RED),
             Text("Lüftungsverlust 100 %", font_size=LABEL_FONT_SIZE, color=PASTEL_RED)])
        x_l, y_f, y_c, lo, hi = room1["x_l"], room1["y_f"], room1["y_c"], room1["win_lo"], room1["win_hi"]
        win1 = window_glyph(room1["glass_x"], lo, hi, depth=0.4, color=PASTEL_CYAN)
        in1 = [path((x_l - 1.05, lo - 0.05), (x_l - 0.4, lo + 0.08), (x_l + 0.3, lo + 0.06), (x_l + 0.8, y_f + 0.15),
                    (p1 + 0.7, y_f + 0.2), (p1 + 0.85, y_f + 0.6))]
        out1 = [path((p1 + 0.95, y_c - 0.35), (p1 + 0.4, y_c - 0.15), (x_l + 0.45, hi - 0.06), (x_l - 0.35, hi - 0.04),
                     (x_l - 1.0, hi + 0.35))]
        lbl_in1 = legend(["Kaltluft", "(−5°C)"], PASTEL_BLUE, 0.75, [p1 - 1.4, -1.55, 0])
        lbl_out1 = legend(["Warmluft", "(+21 °C)"], PASTEL_ORANGE, 0.75, [p1 - 1.3, 0.45, 0])
        col1 = VGroup(card1, hdr1, room1["shell"], win1["group"], person1, lbl_in1, lbl_out1, box1_stat)

        # Panel 2 — Zentrale WRG
        p2 = 0.0
        card2, hdr2, box2_stat, room2, person2 = panel(
            p2, PASTEL_BLUE, "#101825", "Zentrale WRG", "Zentrales Lüftungsgerät",
            [math_label(r"\eta_{\mathrm{WRG}} = 85 \ldots 95\,\%", size=LABEL_FONT_SIZE, color=PASTEL_GREEN),
             Text("Lüftungsverlust ≈ 10 %", font_size=LABEL_FONT_SIZE, color=PASTEL_GREEN)])
        y_c2 = room2["y_c"]
        mvhr_box = RoundedRectangle(width=1.5, height=0.5, corner_radius=0.05, color=PASTEL_WHITE, fill_color="#2C3E50",
                                    fill_opacity=0.95, stroke_width=2).move_to([p2 + 0.15, 0.6, 0])
        lbl_mvhr = Text("WRG\nZentral", font_size=LABEL_FONT_SIZE, color=PASTEL_YELLOW).scale(0.65)
        lbl_mvhr.move_to(mvhr_box.get_center() + LEFT * 0.33)
        fan2 = _fan(mvhr_box.get_center() + RIGHT * 0.42, PASTEL_WHITE, radius=0.15)
        x_s, x_e = p2 - 0.35, p2 + 0.6
        ducts2 = VGroup(*[Line([x + dx, mvhr_box.get_bottom()[1], 0], [x + dx, y_c2, 0], color=PASTEL_WHITE, stroke_width=1.4,
                               stroke_opacity=0.7) for x in (x_s, x_e) for dx in (-0.09, 0.09)])
        fresh2 = [path((p2 - 1.95, 1.0), (p2 - 1.0, 0.95), (mvhr_box.get_left()[0] + 0.05, 0.62))]
        fort2 = [path((mvhr_box.get_right()[0] - 0.05, 0.62), (p2 + 1.5, 0.95), (p2 + 1.95, 1.05))]
        sup2 = [path((x_s, 0.33), (x_s, -0.3), (p2 - 0.6, -0.85), (p2 - 0.45, -1.3))]
        ext2 = [path((p2 + 0.45, -1.25), (x_e, -0.6), (x_e, 0.33))]
        lbl_supply = legend(["Vorgewärmte", "Zuluft (+18°C)"], PASTEL_GREEN, 0.62, [p2 - 1.53, -0.75, 0])
        col2 = VGroup(card2, hdr2, room2["shell"], room2["glass"], person2, ducts2, mvhr_box, lbl_mvhr, fan2, lbl_supply,
                      box2_stat)

        # Panel 3 — Dezentrale WRG
        p3 = 4.4
        card3, hdr3, box3_stat, room3, person3 = panel(
            p3, PASTEL_GREEN, "#102018", "Dezentrale WRG", "Pendellüfter mit Keramik",
            [math_label(r"\eta_{\mathrm{WRG}} = 70 \ldots 90\,\%", size=LABEL_FONT_SIZE, color=PASTEL_GREEN),
             Text("Lüftungsverlust ≈ 15 … 30 %", font_size=LABEL_FONT_SIZE, color=PASTEL_GREEN)])
        x_l3, gx3 = room3["x_l"], room3["glass_x"]
        y_u = 0.5 * (room3["win_hi"] + room3["y_c"])
        core = Rectangle(width=0.37, height=0.22, color=PASTEL_YELLOW, stroke_width=1.5, fill_color=PASTEL_ORANGE,
                         fill_opacity=0.9).move_to([gx3, y_u, 0])
        fan3 = _fan([x_l3 + 0.24, y_u, 0], PASTEL_WHITE, radius=0.11)
        ab3 = [path((p3 + 0.6, -0.95), (x_l3 + 0.45, y_u - 0.05), (gx3, y_u), (p3 - 1.4, y_u + 0.2), (p3 - 1.9, 0.12))]
        zu3 = [path((p3 - 1.9, -0.02), (p3 - 1.4, y_u - 0.12), (gx3, y_u), (x_l3 + 0.45, y_u - 0.1), (p3 + 0.6, -1.0))]
        lbl_cyc1 = legend(["70s Abluft", "(Speichern)"], PASTEL_ORANGE, 0.65, [p3 - 1.35, 0.45, 0])
        lbl_cyc2 = legend(["70s Zuluft", "(+17°C)"], PASTEL_GREEN, 0.65, [p3 - 1.35, -0.75, 0])
        col3 = VGroup(card3, hdr3, room3["shell"], room3["glass"], person3, core, fan3, lbl_cyc1, lbl_cyc2, box3_stat)

        paths = VGroup(*in1, *out1, *fresh2, *fort2, *sup2, *ext2, *ab3, *zu3)
        place(VGroup(col1, col2, col3), paths)
        win1["hinge"] = pivot + 0.85 * (win1["hinge"] - pivot) + UP * 0.35
        guides1 = VGroup(flow_guides(in1, PASTEL_BLUE, opacity=0.2), flow_guides(out1, PASTEL_ORANGE, opacity=0.2))
        guides2 = VGroup(flow_guides(fresh2, PASTEL_BLUE, opacity=0.2), flow_guides(fort2, GREY_B, opacity=0.2),
                         flow_guides(sup2, PASTEL_GREEN, opacity=0.2), flow_guides(ext2, PASTEL_ORANGE, opacity=0.2))
        guides3 = VGroup(flow_guides(ab3, PASTEL_ORANGE, opacity=0.2, tips=False))
        active = []
        pendulum = {"out": True}

        def window_air(rt):
            return [_streams([(in1, PASTEL_BLUE, PASTEL_ORANGE), (out1, PASTEL_ORANGE)], rt, speed=0.5, waves=4)]

        def central_air(rt):
            return [_streams([(fresh2, PASTEL_BLUE), (fort2, GREY_B), (sup2, PASTEL_GREEN), (ext2, PASTEL_ORANGE)], rt,
                             speed=0.55, waves=3), *_spin([fan2], rt)]

        def pendulum_air(rt):
            n = max(1, int(round(rt / 2.2)))
            flows, turns = [], []
            for _ in range(n):
                out = pendulum["out"]
                flow = _streams([(ab3, PASTEL_ORANGE)] if out else [(zu3, PASTEL_BLUE, PASTEL_GREEN)], rt / n,
                                speed=0.55, waves=4)
                flow.run_time = rt / n
                turn = _spin([fan3], rt / n, sign=-1.0 if out else 1.0)[0]
                turn.run_time = rt / n
                flows.append(flow)
                turns.append(turn)
                pendulum["out"] = not out
            return [Succession(*flows), Succession(*turns)]

        def systems(rt):
            return [a for fn in active for a in fn(rt)]

        self.play(FadeIn(col1), run_time=1.0)
        self.play(open_window(win1, run_time=0.8), FadeIn(guides1), run_time=0.8)
        active.append(window_air)
        self.play(*systems(1.0), run_time=1.0)
        hold_for(self, N, "window", used=2.8, during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "central"))
        self.play(FadeIn(col2), FadeIn(guides2), *systems(1.3), run_time=1.3)
        active.append(central_air)
        hold_for(self, N, "central", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "decentral"))
        self.play(FadeIn(col3), FadeIn(guides3), *systems(1.3), run_time=1.3)
        active.append(pendulum_air)
        hold_for(self, N, "decentral", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "outro"))
        hold_for(self, N, "outro", during=systems)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion

