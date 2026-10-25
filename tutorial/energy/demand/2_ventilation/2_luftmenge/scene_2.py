"""🌬️ Ventilation Chapter 2 — Wie viel Luft? (air volume, air change rate and volume flow).

One example house runs through the chapter: its section gives the gross volume
V_e and, after the construction is subtracted, the net air volume V; V and the
air change rate n give the volume flow V̇ = V · n, which Heating writes as V · n
and Cooling as q_v,R. This chapter owns V, n and V̇ of the running example
(V ≈ 440 m³, n = 0,5 h⁻¹, V̇ = 220 m³/h = 0,061 m³/s); every later chapter reads
them from here. Formulas and units are typeset with the shared ``math_*`` helpers,
every shown quantity counts up from 0 and every partial value is computed from the
drawn geometry.
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
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    highlight_param, math_label, math_readout, math_panel, de_num,
    smooth_path, flow_guides, flow_animation, dim_arrow,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    house_section, room_section, window_glyph, open_window, radiator, person_glyph, seated_person_glyph,
    lamp_glyph, clock_glyph, thermometer_glyph, ripples,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Wie viel Luft?"


#region Beat helpers
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in
    ``1_introduction/scene_1.py`` and ``2_conduction/scene_2.py`` so every module
    footnotes its norm identically. Module 3 is the ventilation-loss chain of
    DIN EN 12831-1 (Φ_V = V · n · c_Luft · Δθ); the systems beat cites DIN 1946-6.
    Beats whose diagram already prints the standard in full get no corner chip.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs
#region Moving air
def _fan(pos, color, radius: float = 0.18):
    """🌀 Ring plus three blades — marks a stream as fan-driven."""
    ring = Circle(radius=radius, color=color, stroke_width=2.2).set_fill(P_DEEP_DARK, 1.0)
    blades = VGroup(*[Line(ORIGIN, RIGHT * radius * 0.78, color=color, stroke_width=2.2).rotate(a, about_point=ORIGIN)
                      for a in (0.0, TAU / 3, 2 * TAU / 3)])
    return VGroup(ring, blades).move_to(pos)


def _streams(streams, rt: float, *, speed: float = 0.8, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


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
    outline = Polygon(*_to_scene(house["outer"], o, s), color=P_WHITE, stroke_width=1.8)
    inner = VGroup(*[Polygon(*_to_scene(r, o, s), color=P_WHITE, stroke_width=1.2, stroke_opacity=0.8)
                     for r in house["rooms"]])
    eaves = VGroup(*[Line(*_to_scene([(x0, house["eave"]), (x0 + d * eave_m, house["eave"] - slope * eave_m)], o, s),
                          color=P_WHITE, stroke_width=1.8) for x0, d in ((0.0, -1.0), (w, 1.0))])
    ground = Line(o + LEFT * 0.4, o + RIGHT * (w * s + 0.2), color=P_TEAL, stroke_width=2.2)
    hatch = VGroup(*[Line(p, p + DOWN * 0.1 + LEFT * 0.07, color=P_TEAL, stroke_width=1.1)
                     for p in [ground.point_from_proportion(u) for u in np.linspace(0.06, 0.94, 11)]])
    windows, masks = [], VGroup()
    for side, x_m in (("l", house["wall"] / 2), ("r", w - house["wall"] / 2)):
        for sill_m, lintel_m in ((1.2, 2.5), (4.1, 5.3)):
            x, sill, lintel = o[0] + x_m * s, o[1] + sill_m * s, o[1] + lintel_m * s
            masks.add(Rectangle(width=house["wall"] * s + 0.04, height=lintel - sill, stroke_width=0,
                                fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to([x, (sill + lintel) / 2, 0]))
            win = window_glyph(x, sill, lintel, depth=0.8 * (lintel - sill), color=P_CYAN)
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


#region People, rooms and hand-over chips
def _room(center, width, height, color=P_WHITE, t: float = 0.16):
    """🏠 Physical Fundamentals room section — ceiling slab and side walls, then the floor slab, centred on the outline."""
    cx, cy = float(center[0]), float(center[1])
    style = dict(color=color, stroke_width=2, fill_color=color, fill_opacity=0.14)
    ceiling = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy + height / 2, 0.0]))
    walls = VGroup(*[
        Rectangle(width=t, height=height - t, **style).move_to(np.array([cx + s * width / 2, cy, 0.0]))
        for s in (-1, 1)
    ])
    floor = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy - height / 2, 0.0]))
    return VGroup(VGroup(ceiling, walls), floor)


def _stand(room_c, room_h, x, color=P_ORANGE, t: float = 0.16):
    """🧍 Occupant at true scale — 1.75 m in a 2.5 m room, feet on the floor slab's top edge at ``x``."""
    height = 0.7 * (room_h - t)
    scale = height / 0.66
    return person_glyph(np.array([x, float(room_c[1]) - room_h / 2 + t / 2 + height / 2, 0.0]), color=color, scale=scale)


def _dim(mob, factor: float, **kwargs):
    """🌘 Animation that fades everything in ``mob`` to ``factor`` of its current opacity — context stays, focus moves on."""
    parts = [(m, m.get_fill_opacity(), m.get_stroke_opacity()) for m in mob.get_family() if not m.submobjects]
    scale = lambda a: 1.0 - a * (1.0 - factor)
    return UpdateFromAlphaFunc(mob, lambda _, a: [(m.set_fill(opacity=fo * scale(a)), m.set_stroke(opacity=so * scale(a)))
                                                 for m, fo, so in parts], **kwargs)


def _handover_chip(src: str, color, size: float = BODY_FONT_SIZE):
    """🔖 Hand-over chip with a typeset quantity — names what the next video takes from this chapter."""
    label = math_label(src, size=size, color=color)
    return VGroup(SurroundingRectangle(label, color=color, corner_radius=0.1, buff=0.14, stroke_width=1.8), label)


def _amount(value: float) -> str:
    """🔢 German number with as many decimals as the magnitude needs to stay readable."""
    return de_num(value, 0 if value >= 10 else 1 if value >= 1 else 3)
#endregion


#region Instrument glyphs
def _clock(center, r: float, minutes, *, color=P_CYAN):
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

#endregion


#region Air change scale
def _n_scale(x: float, y_lo: float, y_hi: float, *, lo: float = 0.1, hi: float = 30.0, zero_drop: float = 0.45):
    """📶 Vertical log scale of the air change rate n in 1/h with a broken-off "≈ 0" end below 0,1."""
    span = np.log10(hi) - np.log10(lo)

    def y_of(value: float) -> float:
        if value <= 0:
            return y_lo - zero_drop
        return y_lo + (np.log10(value) - np.log10(lo)) / span * (y_hi - y_lo)

    axis = Line([x, y_lo, 0], [x, y_hi, 0], color=P_TEAL, stroke_width=3)
    stub = Line([x, y_lo - zero_drop, 0], [x, y_lo - 0.55 * zero_drop, 0], color=P_TEAL, stroke_width=3)
    gap = y_lo - 0.36 * zero_drop
    brk = VGroup(*[Line([x - 0.11, gap + dy - 0.05, 0], [x + 0.11, gap + dy + 0.05, 0], color=P_TEAL, stroke_width=2)
                   for dy in (-0.05, 0.05)])
    ticks, labels = VGroup(), VGroup()
    values = [d * 10.0 ** e for e in (-1, 0, 1) for d in range(1, 10)] + [20.0, 30.0]
    for v in sorted(set(v for v in values if lo <= v <= hi)):
        y = y_of(v)
        major = v in (0.1, 1.0, 10.0)
        ticks.add(Line([x - (0.17 if major else 0.08), y, 0], [x, y, 0], color=P_WHITE,
                       stroke_width=2.0 if major else 1.1))
        if major:
            labels.add(math_label(de_num(v, 1 if v < 1 else 0), np.array([x - 0.26, y - 0.07, 0]),
                                  size=LABEL_FONT_SIZE, edge="right"))
    ticks.add(Line([x - 0.17, y_of(0), 0], [x, y_of(0), 0], color=P_WHITE, stroke_width=2.0))
    labels.add(math_label("0", np.array([x - 0.26, y_of(0) - 0.07, 0]), size=LABEL_FONT_SIZE, edge="right"))
    title = math_label(r"n\;[\mathrm{h^{-1}}]", np.array([x + 0.1, y_hi + 0.22, 0]), size=LABEL_FONT_SIZE,
                       color=P_TEAL, edge="right")
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
        unit = Rectangle(width=0.5, height=0.24, color=P_GREEN, stroke_width=2).move_to([-0.2, 0.36, 0])
        parts.add(Line(unit.get_left(), [-0.8, 0.36, 0], color=P_BLUE, stroke_width=2.4),
                  Line(unit.get_right(), [0.8, 0.36, 0], color=P_ORANGE, stroke_width=2.4),
                  unit, _fan(unit.get_center(), P_GREEN, radius=0.08),
                  person_glyph([0.35, fy + 0.3, 0], color=P_ORANGE, scale=0.9))
        air = [path((-0.3, 0.22), (-0.55, -0.05), (-0.35, -0.4)), path((0.65, -0.4), (0.62, 0.02), (0.1, 0.22))]
    elif kind == "buero":
        parts.add(Line([-0.08, -0.12, 0], [0.66, -0.12, 0], color=GREY_B, stroke_width=3),
                  *[Line([x, -0.12, 0], [x, fy, 0], color=GREY_B, stroke_width=2) for x in (-0.02, 0.6)],
                  Rectangle(width=0.36, height=0.26, color=P_CYAN, stroke_width=2).move_to([0.36, 0.06, 0]),
                  seated_person_glyph([-0.42, fy, 0], color=color, scale=1.15, chair_color=GREY_B)["group"])
        air = [path((-0.72, y), (0.0, y - 0.04), (0.72, y)) for y in (0.36, 0.46)]
    elif kind == "klasse":
        parts.add(Rectangle(width=1.0, height=0.3, color=P_GREEN, stroke_width=2).set_fill("#163a2a", 0.9)
                  .move_to([0, 0.32, 0]),
                  *[seated_person_glyph([cx, fy, 0], color=color, scale=0.72, chair_color=GREY_B)["group"]
                    for cx in (-0.6, -0.12, 0.36)])
        air = [path((-0.72, y), (0.0, y + 0.03), (0.72, y)) for y in (0.0, 0.08)]
    elif kind == "kueche":
        stove = Rectangle(width=0.9, height=0.34, color=GREY_B, stroke_width=2).move_to([0, fy + 0.17, 0])
        pot = Rectangle(width=0.32, height=0.2, color=P_WHITE, stroke_width=2).next_to(stove, UP, buff=0)
        hood = Polygon([-0.42, 0.5, 0], [0.42, 0.5, 0], [0.28, 0.3, 0], [-0.28, 0.3, 0], color=GREY_B, stroke_width=2)
        parts.add(stove, pot, hood)
        air = [path((x, -0.0), (x + 0.04, 0.14), (x * 0.6, 0.3)) for x in (-0.09, 0.05)]
    elif kind == "op":
        parts.add(Line([-0.45, -0.2, 0], [0.45, -0.2, 0], color=P_WHITE, stroke_width=4),
                  Line([0, -0.2, 0], [0, fy, 0], color=GREY_B, stroke_width=3),
                  lamp_glyph([0, 0.55, 0], drop=0.08)["group"].set_color(P_YELLOW),
                  person_glyph([0.6, fy + 0.29, 0], color=P_CYAN, scale=0.88))
        air = [path((x, 0.5), (x, -0.12)) for x in (-0.66, -0.42, 0.34)]
    parts.air = air
    return parts
#endregion
#endregion


#region Shared numbers
# 🔢 Source of truth for V, n and V̇ of the running example — later chapters mirror these values from here.
HOUSE = _house_section()
V_NET = round(HOUSE["V"], -1)
N_RATE = 0.5
V_DOT = V_NET * N_RATE
V_DOT_S = V_DOT / 3600.0
PERSONS, PER_PERSON = 4, 30.0
#endregion


#region Beat1 – Gross volume V_e and net air volume V
class Beat1_Innenvolumen(Scene):
    """📦 Section of the example house: gross volume from outer dimensions, then construction subtracted to the net air volume."""

    NARRATION = [
        ("intro",
         "We draw the house in section: ten metres wide, six metres to the eaves, eight and a half to the ridge.",
         "Wir zeichnen das Haus im Schnitt: 10 m breit, 6 m bis zur Traufe, 8,5 m bis zum First."),
        ("gross",
         "With outer dimensions the section is 72.5 square metres; times 8 metres depth gives the gross volume V e of 580 cubic metres.",
         "Mit Außenmaßen hat der Schnitt 72,5 m², mal 8 m Tiefe: das Bruttovolumen V-e mit 580 m³."),
        ("focus",
         "Outdoor air fades away — only the air inside is exchanged. But walls, slabs and roof are not air.",
         "Die Außenluft verblasst — getauscht wird nur die Luft innen. Wände, Decken und Dach sind aber keine Luft."),
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
         "V is the net volume — the starting point of every ventilation heat flow, in winter and in summer.",
         "V ist das Nettovolumen — Ausgangspunkt jedes Lüftungswärmestroms, im Winter wie im Sommer."),
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
        w_dim = dim_arrow([x_l, O[1] - 0.24, 0], [x_r, O[1] - 0.24, 0], color=P_YELLOW)
        w_lbl = math_label(rf"{de_num(H['w'])}\,\mathrm{{m}}", np.array([(x_l + x_r) / 2, O[1] - 0.52, 0]),
                           size=LABEL_FONT_SIZE, color=P_YELLOW)
        e_dim = dim_arrow([x_l - 0.3, O[1], 0], [x_l - 0.3, outer_pts[4][1], 0], color=P_YELLOW)
        e_lbl = math_label(rf"{de_num(H['eave'])}\,\mathrm{{m}}", size=LABEL_FONT_SIZE, color=P_YELLOW)
        e_lbl.next_to(e_dim, LEFT, buff=0.1)
        r_x = x_r + depth_vec[0] + 0.3
        r_dim = dim_arrow([r_x, O[1], 0], [r_x, outer_pts[3][1], 0], color=P_YELLOW)
        r_lbl = math_label(rf"{de_num(H['ridge'], 1)}\,\mathrm{{m}}", size=LABEL_FONT_SIZE, color=P_YELLOW)
        r_lbl.next_to(r_dim, RIGHT, buff=0.1)
        out_paths = [smooth_path([np.array(p) + d for d in ([-0.16, -0.03, 0], [0.06, 0.03, 0], [0.28, -0.01, 0])])
                     for p in ([-6.6, 1.75, 0], [-5.2, 2.2, 0], [-6.75, 0.2, 0], [-2.35, 2.25, 0], [-1.05, 1.95, 0],
                               [-0.75, -0.35, 0], [-1.15, -0.95, 0], [-6.75, -0.75, 0])]

        def outdoor(rt):
            return [_streams([(out_paths, P_BLUE)], rt, speed=0.55, waves=2)]

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
                                                 fill_color=P_TEAL, fill_opacity=0.28))
        a_read = math_readout(lambda: rf"A_{{\mathrm{{e}}}} = {de_num(area_e(), 1)}\,\mathrm{{m^{{2}}}}",
                              np.array([x0, 1.85, 0]), size=BODY_FONT_SIZE, color=P_TEAL)
        self.add(gross_fill, a_read)
        self.play(level.animate.set_value(H["ridge"]), *outdoor(2.0), run_time=2.0, rate_func=linear)

        def back():
            off = depth_vec * depth.get_value() / H["depth"]
            pts = [p + off for p in outer_pts]
            edges = [Line(outer_pts[i], pts[i], color=P_WHITE, stroke_width=1.4, stroke_opacity=0.45) for i in (1, 2, 3, 4)]
            return VGroup(Polygon(*pts, color=P_WHITE, stroke_width=1.4, stroke_opacity=0.45), *edges)

        back_mob = always_redraw(back)
        ve_read = math_readout(
            lambda: rf"V_{{\mathrm{{e}}}} = {de_num(H['A_e'], 1)} \cdot {de_num(depth.get_value(), 1)} = "
                    rf"{de_num(H['A_e'] * depth.get_value())}\,\mathrm{{m^{{3}}}}",
            np.array([x0, 1.28, 0]), size=BODY_FONT_SIZE, color=P_WHITE)
        self.add(back_mob, ve_read)
        self.play(depth.animate.set_value(H["depth"]), *outdoor(1.8), run_time=1.8, rate_func=linear)
        ve_read.clear_updaters()
        d_start = outer_pts[1] + RIGHT * 0.12 + DOWN * 0.1
        d_dim = dim_arrow(d_start, d_start + depth_vec, color=P_YELLOW)
        d_lbl = math_label(rf"{de_num(H['depth'])}\,\mathrm{{m}}", size=LABEL_FONT_SIZE, color=P_YELLOW)
        d_lbl.next_to(d_dim.get_center(), DR, buff=0.04)
        self.play(GrowFromCenter(d_dim), FadeIn(d_lbl), *outdoor(0.6), run_time=0.6)
        hold_for(self, N, "gross", during=outdoor)
        #endregion

        #region construction lights up
        caption = swap_caption(self, caption, subtitle_text(N, "focus"))
        pieces = {name: _poly(pts, O, S, stroke_width=1, color=P_YELLOW, fill_color=P_YELLOW, fill_opacity=0.0)
                  for name, pts in H["pieces"].items()}
        self.add(*pieces.values())
        last = outdoor(1.2)[0]
        self.play(last, _fade_flow(last), gross_fill.animate.set_opacity(0.0), run_time=1.2)
        self.remove(gross_fill)
        self.play(*[p.animate.set_fill(P_YELLOW, opacity=0.75) for p in pieces.values()], run_time=1.0)
        hold_for(self, N, "focus")
        #endregion

        #region subtract construction
        caption = swap_caption(self, caption, subtitle_text(N, "subtract"))
        removed = ValueTracker(0.0)
        a_net_read = math_readout(
            lambda: rf"A = {de_num(H['A_e'], 1)} - {de_num(removed.get_value(), 1)} = "
                    rf"{de_num(H['A_e'] - removed.get_value(), 1)}\,\mathrm{{m^{{2}}}}",
            np.array([x0, 0.62, 0]), size=BODY_FONT_SIZE, color=P_ORANGE)
        self.add(a_net_read)
        for group in (("wall_l", "wall_r"), ("slab", "decke"), ("roof_l", "roof_r")):
            share = sum(_shoelace(H["pieces"][g]) for g in group)
            self.play(*[Indicate(pieces[g], color=P_WHITE, scale_factor=1.0) for g in group], run_time=0.5)
            self.play(*[FadeOut(pieces[g], scale=0.8) for g in group],
                      removed.animate.increment_value(share), run_time=0.9)
        inner_d = ValueTracker(H["depth"])
        di_read = math_readout(
            lambda: rf"d_{{\mathrm{{i}}}} = {de_num(H['depth'])} - 2 \cdot {de_num(H['wall'], 2)} = "
                    rf"{de_num(inner_d.get_value(), 2)}\,\mathrm{{m}}",
            np.array([x0, 0.05, 0]), size=BODY_FONT_SIZE, color=P_WHITE)
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
            faces = VGroup(Polygon(*back, stroke_color=P_ORANGE, stroke_width=stroke, stroke_opacity=0.55,
                                   fill_color=P_ORANGE, fill_opacity=fill * 0.45))
            for a, b in zip(range(len(front)), list(range(1, len(front))) + [0]):
                faces.add(Polygon(front[a], front[b], back[b], back[a], stroke_width=0,
                                  fill_color=P_ORANGE, fill_opacity=fill * 0.35))
            faces.add(*[Line(f, k, color=P_ORANGE, stroke_width=stroke, stroke_opacity=0.7) for f, k in zip(front, back)])
            faces.add(Polygon(*front, stroke_color=P_ORANGE, stroke_width=stroke, fill_color=P_ORANGE, fill_opacity=fill))
            return faces

        room_fill = always_redraw(lambda: VGroup(*[
            prism(_clip_below(r, lvl.get_value()), fill=0.5, stroke=0.0) for r in rooms
        ]))
        room_edges = VGroup(*[prism(r, fill=0.0, stroke=1.6) for r in rooms])
        v_read = math_readout(
            lambda: rf"V = {de_num(net_area(), 1)} \cdot {de_num(H['d_i'], 2)} = {de_num(net_area() * H['d_i'])}\,\mathrm{{m^{{3}}}}",
            np.array([x0, -0.52, 0]), size=BODY_FONT_SIZE, color=P_ORANGE)
        self.add(room_fill, v_read)
        self.play(Create(room_edges), pf["group"].animate.set_stroke(opacity=0.5), run_time=0.6)
        self.play(lvl.animate.set_value(H["top_in"]), run_time=2.4, rate_func=linear)
        og_c = np.mean(_to_scene(rooms[1], O, S), axis=0)
        v_sym = math_label("V", np.array([og_c[0], og_c[1] - 0.12, 0]) + off_front, size=FORMULA_FONT_SIZE, color=P_WHITE)
        loops = []
        for r in rooms:
            pts = [p + off_front for p in _to_scene(r, O, S)]
            mid = np.mean(pts, axis=0)
            ring = [mid + 0.72 * (p - mid) for p in pts]
            loops.append(smooth_path(ring + ring[:1]))

        def warm_air(rt):
            return [_streams([(loops, P_ORANGE)], rt, speed=0.22, waves=5)]

        self.play(FadeIn(v_sym, scale=1.3), *warm_air(0.5), run_time=0.5)
        hold_for(self, N, "fill", during=warm_air)
        #endregion

        #region ratio bars and formula
        caption = swap_caption(self, caption, subtitle_text(N, "ratio"))
        ratio = H["V"] / H["V_e"]
        bx, blen = 1.05, 3.9
        bar_e = Rectangle(width=blen, height=0.22, color="#8A9BA8", stroke_width=1.6).set_fill("#8A9BA8", 0.25)
        bar_e.move_to([bx + blen / 2, -1.0, 0])
        bar_v = Rectangle(width=blen * ratio, height=0.22, color=P_ORANGE, stroke_width=1.6).set_fill(P_ORANGE, 0.6)
        bar_v.move_to([bx + blen * ratio / 2, -1.36, 0])
        lbl_e = math_label(r"V_{\mathrm{e}}", np.array([bx - 0.12, -1.07, 0]), size=LABEL_FONT_SIZE, edge="right")
        lbl_v = math_label("V", np.array([bx - 0.12, -1.43, 0]), size=LABEL_FONT_SIZE, color=P_ORANGE, edge="right")
        val_e = math_label(rf"{de_num(H['V_e'])}\,\mathrm{{m^{{3}}}}", np.array([bx + blen + 0.14, -1.07, 0]),
                           size=LABEL_FONT_SIZE, edge="left")
        val_v = math_label(rf"{de_num(H['V'])}\,\mathrm{{m^{{3}}}}", np.array([bx + blen * ratio + 0.14, -1.43, 0]),
                           size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        self.play(TransformFromCopy(outline, bar_e), FadeIn(lbl_e), FadeIn(val_e), *warm_air(1.0), run_time=1.0)
        self.play(TransformFromCopy(room_edges, bar_v), FadeIn(lbl_v), FadeIn(val_v), *warm_air(1.0), run_time=1.0)
        row, box, items = math_panel([
            ("v", "V", P_ORANGE), (None, r"\approx", None),
            ("f", de_num(ratio, 2), P_YELLOW), (None, r"\cdot", None),
            ("ve", r"V_{\mathrm{e}}", P_WHITE), (None, r"=", None),
            ("num", rf"{de_num(ratio, 2)} \cdot {de_num(H['V_e'])} \approx {de_num(V_NET)}\,\mathrm{{m^{{3}}}}", P_WHITE),
        ])
        self.play(TransformFromCopy(v_sym, items["v"]), TransformFromCopy(lbl_e, items["ve"]), *warm_air(0.8), run_time=0.8)
        self.play(FadeIn(VGroup(*[m for m in row if m is not items["v"] and m is not items["ve"]])),
                  Create(box), *warm_air(0.7), run_time=0.7)
        ring = highlight_param(items, "f", color=P_YELLOW)
        self.play(Create(ring), *warm_air(0.4), run_time=0.4)
        hold_for(self, N, "ratio", during=warm_air)
        self.play(FadeOut(ring), *warm_air(0.2), run_time=0.2)
        #endregion

        caption = swap_caption(self, caption, subtitle_text(N, "label"))
        ring = highlight_param(items, "v", color=P_ORANGE)
        self.play(Create(ring), Indicate(v_sym, color=P_ORANGE), *warm_air(0.6), run_time=0.6)
        hold_for(self, N, "label", during=warm_air)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Air change rate n: animated exchange and a scale of uses
AIR_CHANGE_USES = [
    ("lager", 0.0, r"\text{Lagerraum ohne Nutzung}", GREY_B),
    ("passiv", 0.3, r"\text{Passivhaus mit Lüftungsanlage}", P_GREEN),
    ("buero", 2.0, r"\text{Büro}", P_CYAN),
    ("klasse", 4.0, r"\text{Klassenzimmer}", P_YELLOW),
    ("kueche", 15.0, r"\text{Gewerbeküche}", P_ORANGE),
    ("op", 20.0, r"\text{Operationssaal}", P_RED),
]


class Beat2_Luftwechselrate(Scene):
    """🔄 Fresh air replaces room air at the rate n; typical uses morph onto a logarithmic n scale."""

    NARRATION = [
        ("volume",
         "Start again from the air volume V inside the house: 440 cubic metres.",
         "Wieder vom Luftvolumen V im Haus ausgehen: 440 m³."),
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
         "Volume times air change rate — V times n — sets the ventilation airflow, in both seasons.",
         "Volumen mal Luftwechselrate — V mal n — bestimmt den Lüftungsvolumenstrom, in beiden Jahreszeiten."),
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

        warm = always_redraw(lambda: _poly(_clip_above(outer, fill_y()), O, S, stroke_width=0, fill_color=P_ORANGE,
                                           fill_opacity=0.32))
        fresh = always_redraw(lambda: _poly(_clip_below(outer, fill_y()), O, S, stroke_width=0, fill_color=P_BLUE,
                                            fill_opacity=0.45))
        v_lbl = math_label(rf"V = {de_num(V_NET)}\,\mathrm{{m^{{3}}}}", np.array([O[0] + 5 * S, O[1] + 6.1 * S, 0]),
                           size=BODY_FONT_SIZE, color=P_WHITE)
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
                              size=BODY_FONT_SIZE, color=P_CYAN, edge="center")
        x0 = 0.55
        n_lbl = math_label(rf"n = {de_num(N_RATE, 1)}\,\mathrm{{h^{{-1}}}}", np.array([x0, 1.75, 0]),
                           size=BODY_FONT_SIZE, color=P_CYAN, edge="left")
        x_i, y_i = win_in["x"], win_in["center"][1]
        x_o, y_o = win_out["x"], win_out["center"][1]
        in_paths = [smooth_path([np.array([x_i + dx, y_i + dy, 0]) for dx, dy in ((-0.8, -0.08), (0.0, -0.04), (0.9, -0.28))]
                                + [np.array([x_i + 2.2, O[1] + 0.25, 0])])]
        out_paths = [smooth_path([np.array([x_o + dx, y_o + dy, 0]) for dx, dy in ((-1.4, -0.22), (0.0, 0.02), (0.8, 0.37))])]
        streams = [(in_paths, P_BLUE), (out_paths, P_ORANGE)]
        guides = VGroup(flow_guides(in_paths, P_BLUE, opacity=0.22), flow_guides(out_paths, P_ORANGE, opacity=0.22))

        def exchange(rt):
            return [_streams(streams, rt, speed=0.6, waves=4)]

        self.play(FadeIn(clock["group"]), FadeIn(t_read), FadeIn(n_lbl), _open_sash(win_in, run_time=1.0),
                  _open_sash(win_out, run_time=1.0), run_time=1.0)
        self.play(FadeIn(guides), *exchange(0.8), run_time=0.8)
        hold_for(self, N, "rate", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(N, "hour"))
        fresh_read = math_readout(
            lambda: rf"V_{{\mathrm{{frisch}}}} = V \cdot n \cdot t = {de_num(V_NET * N_RATE * hours.get_value())}\,\mathrm{{m^{{3}}}}",
            np.array([x0, 1.05, 0]), size=BODY_FONT_SIZE, color=P_BLUE)
        pct_read = math_readout(lambda: rf"{de_num(100 * frac.get_value())}\,\%\;\text{{der Raumluft getauscht}}",
                                np.array([x0, 0.45, 0]), size=LABEL_FONT_SIZE, color=P_BLUE)
        self.add(fresh_read, pct_read)

        def run_hour(h_to: float):
            rt = 3.2 * (h_to - hours.get_value())
            self.play(_streams(streams, rt, speed=0.75, waves=6), hours.animate.set_value(h_to),
                      minutes.animate.set_value(60.0 * h_to), frac.animate.set_value(min(1.0, N_RATE * h_to)),
                      run_time=rt, rate_func=linear)

        run_hour(1.0)
        flow_read = math_label(rf"\dot{{V}} = \frac{{{de_num(V_NET * N_RATE)}\,\mathrm{{m^{{3}}}}}}{{1\,\mathrm{{h}}}} = "
                               rf"{de_num(V_NET * N_RATE)}\,\mathrm{{m^{{3}}/h}}", np.array([x0, -0.35, 0]),
                               size=BODY_FONT_SIZE, color=P_TEAL, edge="left")
        snap = math_label(rf"V_{{\mathrm{{frisch}}}} = V \cdot n \cdot t = {de_num(V_NET * N_RATE)}\,\mathrm{{m^{{3}}}}",
                          np.array([x0, 1.05, 0]), size=BODY_FONT_SIZE, color=P_BLUE, edge="left")
        self.play(TransformFromCopy(snap, flow_read), *exchange(0.9), run_time=0.9)
        hold_for(self, N, "hour", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(N, "one_change"))
        run_hour(2.0)
        full = math_label(rf"t = \frac{{1}}{{n}} = {de_num(1 / N_RATE)}\,\mathrm{{h}}", np.array([-1.0, -0.65, 0]),
                          size=BODY_FONT_SIZE, color=P_CYAN)
        self.play(FadeIn(full, shift=UP * 0.1), Indicate(fresh, color=P_CYAN, scale_factor=1.0), *exchange(0.8),
                  run_time=0.8)
        hold_for(self, N, "one_change", during=exchange)
        #endregion

        #region formula panel
        caption = swap_caption(self, caption, subtitle_text(N, "product"))
        row, box, items = math_panel([
            ("v", "V", P_ORANGE), (None, r"\cdot", None), ("n", "n", P_CYAN), (None, "=", None),
            ("vd", r"\dot{V}", P_TEAL), (None, r"\;[\mathrm{m^{3}/h}]", None),
        ])
        self.play(TransformFromCopy(v_lbl, items["v"]), TransformFromCopy(n_lbl, items["n"]),
                  TransformFromCopy(flow_read, items["vd"]), *exchange(1.0), run_time=1.0)
        self.play(FadeIn(row), Create(box), *exchange(0.5), run_time=0.5)
        hold_for(self, N, "product", during=exchange)
        #endregion

        #region scale of uses
        caption = swap_caption(self, caption, subtitle_text(N, "home"))
        self.remove(fresh)
        fresh_static = _poly(outer, O, S, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.45)
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

        entries = [(N_RATE, r"\text{Wohnung} \approx 0{,}5", P_ORANGE)] + [
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

        show_use(house, r"\text{Wohnung}", N_RATE, P_ORANGE, 0)
        hold_for(self, N, "home")
        for key, idxs in (("low", (0, 1)), ("mid", (2, 3)), ("high", (4, 5))):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            for i in idxs:
                kind, value, name, color = AIR_CHANGE_USES[i]
                show_use(_use_glyph(kind, color), name, value, color, i + 1)
            if key == "high":
                renew = math_label(rf"\frac{{60\,\mathrm{{min}}}}{{{de_num(20.0)}}} = {de_num(60 / 20.0)}\,\mathrm{{min}}",
                                   stage + DOWN * 0.1, size=BODY_FONT_SIZE, color=P_RED)
                self.wait(1.0)
                self.play(TransformFromCopy(labels[6], renew), run_time=1.5)
                self.play(Indicate(renew, color=P_RED, scale_factor=1.12), run_time=0.8)
            hold_for(self, N, key)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Volume flow V̇ in two spellings
class Beat3_Volumenstrom(Scene):
    """💨 V · n as an area, per hour and per second — one airflow that Heating writes V · n and Cooling writes q_v,R."""

    NARRATION = [
        ("area",
         "Volume times air change rate: 440 cubic metres high, 0.5 per hour wide — the area is the volume flow, 220 cubic metres per hour.",
         "Volumen mal Luftwechselrate: 440 m³ hoch, 0,5 pro Stunde breit — die Fläche ist der Volumenstrom, 220 m³ pro Stunde."),
        ("hour",
         "Those 220 cubic metres flow through the house within one hour — 3600 seconds.",
         "Diese 220 m³ strömen in einer Stunde durch das Haus — das sind 3600 Sekunden."),
        ("second",
         "Divided by 3600, 0.061 cubic metres remain per second — about 61 litres.",
         "Geteilt durch 3600 bleiben pro Sekunde 0,061 m³ — rund 61 Liter."),
        ("spell",
         "One airflow, two spellings: Heating writes V times n in cubic metres per hour, Cooling writes q v R in cubic metres per second.",
         "Ein Luftstrom, zwei Schreibweisen: Die Heizung schreibt V mal n in m³/h, die Kühlung q-v-R in m³/s."),
        ("people",
         "Cross-check by hygiene: four people need about 30 cubic metres per hour each — 120 in total.",
         "Gegenprobe Hygiene: Vier Personen brauchen je etwa 30 m³/h, zusammen 120 m³/h."),
        ("cover",
         "That is below 220 — the minimum air change n of 0.5 covers the hygiene demand, in winter and in summer.",
         "Das liegt unter 220 m³/h — der Mindestluftwechsel n = 0,5 deckt den Hygienebedarf ab, im Winter wie im Sommer."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Volumenstrom: ein Luftstrom, zwei Schreibweisen", title)
        din = _din_ref("DIN EN 12831-1 · DIN EN 16798-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "area"))
        self.play(FadeIn(caption), run_time=0.3)

        #region area V times n
        ox, oy, ux, uy = -5.0, -0.6, 3.4, 0.0040
        v_tr, n_tr = ValueTracker(0.0), ValueTracker(0.0)
        x_end, y_end = ox + 1.08 * ux, oy + 640 * uy
        axes = VGroup(
            Arrow([ox, oy, 0], [x_end, oy, 0], buff=0, color=P_WHITE, stroke_width=2.4, tip_length=0.16),
            Arrow([ox, oy, 0], [ox, y_end, 0], buff=0, color=P_WHITE, stroke_width=2.4, tip_length=0.16),
            *[Line([ox + n * ux, oy, 0], [ox + n * ux, oy - 0.1, 0], color=P_WHITE, stroke_width=2) for n in (0.5, 1.0)],
            math_label(de_num(0.5, 1), np.array([ox + 0.5 * ux, oy - 0.3, 0]), size=LABEL_FONT_SIZE),
            math_label("1", np.array([ox + ux, oy - 0.3, 0]), size=LABEL_FONT_SIZE),
            math_label(r"n\;[\mathrm{h^{-1}}]", np.array([x_end + 0.2, oy - 0.07, 0]), size=LABEL_FONT_SIZE, edge="left"),
            math_label(r"V\;[\mathrm{m^{3}}]", np.array([ox, y_end + 0.3, 0]), size=LABEL_FONT_SIZE),
        )

        def area_rect():
            w, h = max(0.12, n_tr.get_value() * ux), max(0.004, v_tr.get_value() * uy)
            return Rectangle(width=w, height=h, color=P_TEAL, stroke_width=2, fill_color=P_TEAL,
                             fill_opacity=0.4).move_to([ox + w / 2, oy + h / 2, 0])

        v_slide = always_redraw(lambda: Line([ox, oy, 0], [ox, oy + max(0.004, v_tr.get_value() * uy), 0],
                                             color=P_ORANGE, stroke_width=7))
        n_slide = always_redraw(lambda: Line([ox, oy, 0], [ox + max(0.004, n_tr.get_value() * ux), oy, 0],
                                             color=P_CYAN, stroke_width=7))
        area = always_redraw(area_rect)
        v_read = math_readout(lambda: rf"V = {de_num(v_tr.get_value())}\,\mathrm{{m^{{3}}}}",
                              lambda: [ox - 0.2, oy + max(0.15, v_tr.get_value() * uy), 0],
                              size=BODY_FONT_SIZE, color=P_ORANGE, edge="right")
        n_read = math_readout(lambda: rf"n = {de_num(n_tr.get_value(), 1)}\,\mathrm{{h^{{-1}}}}",
                              [ox + 0.5 * ux, oy - 0.68, 0], size=BODY_FONT_SIZE, color=P_CYAN, edge="center")
        a_read = math_readout(lambda: rf"\dot{{V}} = {de_num(v_tr.get_value() * n_tr.get_value())}\,\mathrm{{m^{{3}}/h}}",
                              [ox + 0.5 * ux + 0.4, oy + 440 * uy / 2, 0], size=BODY_FONT_SIZE, color=P_TEAL)
        self.play(FadeIn(axes), run_time=0.8)
        self.add(area, v_slide, n_slide, v_read, n_read, a_read)
        self.play(v_tr.animate.set_value(V_NET), run_time=1.8, rate_func=linear)
        self.play(n_tr.animate.set_value(N_RATE), run_time=1.8, rate_func=linear)
        row, box, items = math_panel([
            ("v", "V", P_ORANGE), (None, r"\cdot", None), ("n", "n", P_CYAN), (None, "=", None),
            ("vd", r"\dot{V}", P_TEAL), (None, "=", None),
            ("num", rf"{de_num(V_NET)} \cdot {de_num(N_RATE, 1)} = {de_num(V_DOT)}\,\mathrm{{m^{{3}}/h}}", P_WHITE),
        ])
        self.play(TransformFromCopy(v_read, items["v"]), TransformFromCopy(n_read, items["n"]),
                  TransformFromCopy(a_read, items["vd"]), run_time=1.0)
        self.play(FadeIn(VGroup(*[m for m in row if m not in (items["v"], items["n"], items["vd"])])), Create(box),
                  run_time=0.6)
        hold_for(self, N, "area")
        #endregion

        #region per hour and per second
        caption = swap_caption(self, caption, subtitle_text(N, "hour"))
        x0, width, sy = 1.3, 5.0, 0.95
        t_tr = ValueTracker(0.0)
        track = Rectangle(width=width, height=0.5, color=P_WHITE, stroke_width=2).set_fill(P_DEEP_DARK, 1.0)
        track.move_to([x0 + width / 2, sy, 0])
        minute_ticks = VGroup(*[Line([x0 + width * i / 12, sy - 0.25, 0], [x0 + width * i / 12, sy - 0.15, 0],
                                     color=P_WHITE, stroke_width=1.4) for i in range(1, 12)])
        flow_fill = always_redraw(lambda: Rectangle(
            width=max(0.025, width * t_tr.get_value() / 3600.0), height=0.42, stroke_width=0, fill_color=P_BLUE,
            fill_opacity=0.85).move_to([x0 + max(0.025, width * t_tr.get_value() / 3600.0) / 2, sy, 0]))
        hour_dim = dim_arrow([x0, sy + 0.55, 0], [x0 + width, sy + 0.55, 0], color=P_YELLOW)
        hour_lbl = math_label(r"1\,\mathrm{h} = 3600\,\mathrm{s}", np.array([x0 + width / 2, sy + 0.9, 0]),
                              size=BODY_FONT_SIZE, color=P_YELLOW)
        t_read = math_readout(lambda: rf"t = {de_num(t_tr.get_value())}\,\mathrm{{s}}", [x0, sy - 0.65, 0],
                              size=BODY_FONT_SIZE, color=P_YELLOW)
        dv_read = math_readout(
            lambda: rf"\Delta V = \dot{{V}} \cdot t = {_amount(V_DOT_S * t_tr.get_value())}\,\mathrm{{m^{{3}}}}",
            [x0, sy - 1.15, 0], size=BODY_FONT_SIZE, color=P_BLUE)
        self.play(FadeIn(track), FadeIn(minute_ticks), GrowFromCenter(hour_dim), FadeIn(hour_lbl), run_time=0.8)
        self.add(flow_fill, t_read, dv_read)
        self.play(t_tr.animate.set_value(3600.0), run_time=2.4, rate_func=linear)
        hold_for(self, N, "hour")

        caption = swap_caption(self, caption, subtitle_text(N, "second"))
        self.play(t_tr.animate.set_value(1.0), run_time=3.0)
        per_s = math_label(
            rf"\frac{{{de_num(V_DOT)}\,\mathrm{{m^{{3}}/h}}}}{{3600\,\mathrm{{s/h}}}} = {de_num(V_DOT_S, 3)}\,\mathrm{{m^{{3}}/s}}"
            rf" \approx {de_num(V_DOT_S * 1000)}\,\mathrm{{l/s}}",
            np.array([x0 + width / 2, -1.0, 0]), size=BODY_FONT_SIZE, color=P_TEAL)
        self.play(TransformFromCopy(a_read, per_s), run_time=1.0)
        hold_for(self, N, "second")
        #endregion

        #region two spellings
        caption = swap_caption(self, caption, subtitle_text(N, "spell"))
        live = [area, v_slide, n_slide, v_read, n_read, a_read, flow_fill, t_read, dv_read]
        for m in live:
            m.clear_updaters()
        old_panel = VGroup(row, box)
        self.play(*[FadeOut(m) for m in (axes, track, minute_ticks, hour_dim, hour_lbl, area, v_slide, n_slide, v_read,
                                         n_read, flow_fill, t_read, dv_read, old_panel)], run_time=0.8)

        def card(head: str, symbol: str, value: str, color, cx: float):
            frame = RoundedRectangle(width=4.4, height=2.5, corner_radius=0.12, color=color, stroke_width=2)
            lines = VGroup(body_text(head, font_size=LABEL_FONT_SIZE, color=color),
                           math_label(symbol, size=FORMULA_FONT_SIZE + 6, color=P_WHITE),
                           math_label(value, size=BODY_FONT_SIZE, color=color)).arrange(DOWN, buff=0.3)
            return VGroup(frame, lines).move_to([cx, 0.35, 0])

        card_h = card("Heizung · Winter", r"V \cdot n", rf"{de_num(V_DOT)}\,\mathrm{{m^{{3}}/h}}", P_ORANGE, -3.4)
        card_k = card("Kühlung · Sommer", r"q_{v,\mathrm{R}}", rf"{de_num(V_DOT_S, 3)}\,\mathrm{{m^{{3}}/s}}", P_CYAN, 3.4)
        to_s = Arrow([-1.05, 0.65, 0], [1.05, 0.65, 0], buff=0, color=P_WHITE, stroke_width=2.4, tip_length=0.16)
        to_h = Arrow([1.05, 0.0, 0], [-1.05, 0.0, 0], buff=0, color=P_WHITE, stroke_width=2.4, tip_length=0.16)
        to_s_lbl = math_label(r"÷\,3600", np.array([0, 1.0, 0]), size=BODY_FONT_SIZE)
        to_h_lbl = math_label(r"\cdot\,3600", np.array([0, -0.35, 0]), size=BODY_FONT_SIZE)
        self.play(FadeIn(card_h[0]), FadeIn(card_h[1][0]), FadeIn(card_h[1][1]), ReplacementTransform(a_read, card_h[1][2]),
                  run_time=0.9)
        self.play(FadeIn(card_k[0]), FadeIn(card_k[1][0]), FadeIn(card_k[1][1]), ReplacementTransform(per_s, card_k[1][2]),
                  GrowArrow(to_s), FadeIn(to_s_lbl), GrowArrow(to_h), FadeIn(to_h_lbl), run_time=1.2)
        row2, box2, items2 = math_panel([
            ("vd", r"\dot{V}", P_TEAL), (None, "=", None), ("vn", r"V \cdot n", P_ORANGE), (None, "=", None),
            ("h", rf"{de_num(V_DOT)}\,\mathrm{{m^{{3}}/h}}", P_ORANGE), (None, "=", None),
            ("s", rf"{de_num(V_DOT_S, 3)}\,\mathrm{{m^{{3}}/s}}", P_CYAN), (None, "=", None),
            ("q", r"q_{v,\mathrm{R}}", P_CYAN),
        ])
        self.play(TransformFromCopy(card_h[1][1], items2["vn"]), TransformFromCopy(card_h[1][2], items2["h"]),
                  TransformFromCopy(card_k[1][1], items2["q"]), TransformFromCopy(card_k[1][2], items2["s"]),
                  FadeIn(items2["vd"]), Create(box2), FadeIn(VGroup(*[m for m in row2 if m not in items2.values()])),
                  run_time=1.2)
        hold_for(self, N, "spell")
        #endregion

        #region hygiene per person
        caption = swap_caption(self, caption, subtitle_text(N, "people"))
        room_c, room_w, room_h = np.array([-4.1, 0.58, 0.0]), 4.0, 2.4
        room = _room(room_c, room_w, room_h)
        persons = [_stand(room_c, room_h, room_c[0] + dx) for dx in (-1.35, -0.45, 0.45, 1.35)]
        room_lbl = math_label(rf"{PERSONS}\;\text{{Personen}}", np.array([room_c[0], room_c[1] + room_h / 2 + 0.38, 0]),
                              size=BODY_FONT_SIZE, color=P_ORANGE)
        bx, unit, row_h = -0.4, 5.6 / V_DOT, 0.45
        bar_n = Rectangle(width=V_DOT * unit, height=row_h, color=P_TEAL, stroke_width=1.6, fill_color=P_TEAL,
                          fill_opacity=0.5).move_to([bx + V_DOT * unit / 2, 1.2, 0])
        lbl_n = math_label(rf"V \cdot n = {de_num(V_DOT)}\,\mathrm{{m^{{3}}/h}}", np.array([bx, 1.75, 0]),
                           size=BODY_FONT_SIZE, color=P_TEAL, edge="left")
        count = ValueTracker(0.0)
        lbl_p = math_readout(lambda: rf"{PERSONS} \cdot {de_num(PER_PERSON)} = {de_num(count.get_value())}\,\mathrm{{m^{{3}}/h}}",
                             [bx, 0.55, 0], size=BODY_FONT_SIZE, color=P_BLUE)
        value_h, value_k = card_h[1][2], card_k[1][2]
        card_h[1].remove(value_h)
        card_k[1].remove(value_k)
        self.play(FadeOut(VGroup(card_h, card_k, to_s, to_h, to_s_lbl, to_h_lbl, value_k, VGroup(row2, box2))),
                  ReplacementTransform(value_h, bar_n), run_time=0.9)
        self.play(Create(room), FadeIn(room_lbl), FadeIn(lbl_n), *[FadeIn(p) for p in persons], run_time=1.0)
        self.add(lbl_p)
        for i, person in enumerate(persons):
            seg = Rectangle(width=PER_PERSON * unit, height=row_h, color=P_BLUE, stroke_width=1.6,
                            fill_color=P_BLUE, fill_opacity=0.6)
            seg.move_to([bx + (i + 0.5) * PER_PERSON * unit, 0.1, 0])
            seg_lbl = math_label(de_num(PER_PERSON), seg.get_center() + DOWN * 0.07, size=LABEL_FONT_SIZE, color=P_WHITE)
            self.play(TransformFromCopy(person, seg), FadeIn(seg_lbl, scale=0.8, rate_func=rush_from),
                      count.animate.set_value(PER_PERSON * (i + 1)), run_time=0.8)
        count.set_value(PER_PERSON * PERSONS)
        lbl_p.clear_updaters()
        hold_for(self, N, "people")
        #endregion

        #region covered by the air change rate
        caption = swap_caption(self, caption, subtitle_text(N, "cover"))
        x_a, x_b = bx + PERSONS * PER_PERSON * unit, bx + V_DOT * unit
        reserve = dim_arrow([x_a, -0.35, 0], [x_b, -0.35, 0], color=P_YELLOW)
        reserve_lbl = math_label(rf"\mathrm{{Reserve}}\;{de_num(V_DOT - PERSONS * PER_PERSON)}\,\mathrm{{m^{{3}}/h}}",
                                 np.array([(x_a + x_b) / 2, -0.72, 0]), size=LABEL_FONT_SIZE, color=P_YELLOW)
        row3, box3, items3 = math_panel([
            ("p", rf"{PERSONS} \cdot {de_num(PER_PERSON)}\,\mathrm{{m^{{3}}/h}} = {de_num(PERSONS * PER_PERSON)}\,\mathrm{{m^{{3}}/h}}", P_BLUE),
            (None, "<", None),
            ("n", rf"V \cdot n = {de_num(V_DOT)}\,\mathrm{{m^{{3}}/h}}", P_TEAL),
        ])
        self.play(TransformFromCopy(lbl_p, items3["p"]), TransformFromCopy(lbl_n, items3["n"]),
                  FadeIn(VGroup(*[m for m in row3 if m not in items3.values()])), Create(box3), run_time=1.0)
        self.play(GrowFromCenter(reserve), FadeIn(reserve_lbl), FadeOut(lbl_n), FadeOut(lbl_p),
                  _dim(VGroup(room, room_lbl, *persons), 0.25), run_time=0.8)
        chips = VGroup(_handover_chip(r"\to\;\text{Heizung:}\;V \cdot n", P_ORANGE, LABEL_FONT_SIZE),
                       _handover_chip(r"\to\;\text{Kühlung:}\;q_{v,\mathrm{R}}", P_CYAN, LABEL_FONT_SIZE)).arrange(RIGHT, buff=0.5)
        chips.move_to([0.0, -1.25, 0])
        self.play(FadeIn(chips, shift=UP * 0.1), run_time=0.6)
        hold_for(self, N, "cover")
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
        #endregion
#endregion
