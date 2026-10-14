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
    apply_scene_style, scene_title, play_scene_title, TITLE_RUN_TIME,
    beat_subtitle, BEAT_SUBTITLE_FADE,
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE, body_text,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    chip, note_line, labeled_axes, stacked_bar,
    meter, bind_meter, cross_mark, dim_arrow, smooth_path,
    math_label, math_panel, math_readout, de_num, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    house_section, window_glyph, radiator, sun_glyph, moon_glyph, sun_rays, shine, radiation_ray, ripples,
    person_glyph, lamp_glyph, thermometer_glyph, clock_glyph, cloud_glyph,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Regelwerk & Klimadaten"

LOSS_BLUE = "#38BDF8"
SOLAR_YELLOW = "#FDE047"
ETA_GREEN = "#22C55E"
DEMAND_RED = "#EF4444"

MONTHS = ("J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D")


#region Climate model
# One schematic year drives every beat of this chapter, so the same weather that
# fills the monthly balance in Beat 2 is what Beat 3 draws and Beat 4 integrates.
# Monthly mean outdoor temperature and relative global irradiation, DWD
# test-reference-year shape for a central-German site — the shape, not a
# certified data row.
THETA_E = (0.5, 1.4, 4.9, 9.4, 13.6, 16.8, 18.4, 18.0, 14.2, 9.6, 4.8, 1.6)
IRRADIATION = (0.10, 0.18, 0.33, 0.50, 0.66, 0.70, 0.68, 0.60, 0.42, 0.25, 0.12, 0.08)
IRRADIATION_PEAK = 0.70
IRRADIATION_FULL_KWH = 220.0

THETA_I = 20.0
THETA_HG = 15.0
THETA_LO, THETA_HI = -4.0, 22.0
INTERNAL_GAIN = 0.07
DAYS_PER_MONTH = 365.0 / 12.0

LOSS_SHAPE = tuple((THETA_I - t) / (THETA_I - THETA_E[0]) for t in THETA_E)
GAIN_SHAPE = tuple(INTERNAL_GAIN + s for s in IRRADIATION)


def _eta_h(gain: float, loss: float, a: float = 2.0) -> float:
    """🔋 Monthly utilisation factor η_h = (1 − γᵃ) / (1 − γᵃ⁺¹) with γ = Q_Quelle / Q_Senke (DIN V 18599-2)."""
    gamma = gain / loss
    if abs(gamma - 1.0) < 1e-6:
        return a / (a + 1.0)
    return (1.0 - gamma ** a) / (1.0 - gamma ** (a + 1.0))


ETA_SHAPE = tuple(_eta_h(g, l) for g, l in zip(GAIN_SHAPE, LOSS_SHAPE))
USABLE_SHAPE = tuple(e * g for e, g in zip(ETA_SHAPE, GAIN_SHAPE))
REST_SHAPE = tuple(max(0.0, l - u) for l, u in zip(LOSS_SHAPE, USABLE_SHAPE))
UNUSED_SHAPE = tuple(g - u for g, u in zip(GAIN_SHAPE, USABLE_SHAPE))
REST_MONTHS = tuple(i for i, r in enumerate(REST_SHAPE) if r > 0.004)


def _catmull(series, m: float) -> float:
    """〰️ Smooth periodic value of a monthly series at month position ``m`` (month ``i`` is centred on ``i + 0.5``)."""
    n = len(series)
    u = (m - 0.5) % n
    i = int(np.floor(u))
    f = u - i
    p0, p1, p2, p3 = (series[(i + k) % n] for k in (-1, 0, 1, 2))
    return float(0.5 * (2 * p1 + (p2 - p0) * f + (2 * p0 - 5 * p1 + 4 * p2 - p3) * f ** 2
                        + (3 * p1 - p0 - 3 * p2 + p3) * f ** 3))


def _theta_norm(theta: float) -> float:
    return (theta - THETA_LO) / (THETA_HI - THETA_LO)


def _degree_days(reach: float, shift: float = 0.0) -> float:
    """🌡️ Gradtagszahl G_t in Kd/a: the area between the heating limit and the curve, swept up to ``reach`` months."""
    xs = np.linspace(0.0, reach, max(3, int(reach * 40) + 1))
    deficit = np.array([max(0.0, THETA_HG - (_catmull(THETA_E, x) + shift)) for x in xs])
    return float(np.sum(0.5 * (deficit[1:] + deficit[:-1]) * np.diff(xs)) * DAYS_PER_MONTH)
#endregion


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

CHART_ORIGIN = (-1.70, -1.28, 0.0)
CHART_W, CHART_H = 7.00, 2.80
HOUSE_C = np.array([-4.10, 0.05, 0.0])
HOUSE_S = 0.78
SUN_C = np.array([-6.45, 1.50, 0.0])
THERMO_X = -6.75
CLOCK_C = np.array([-2.70, 1.74, 0.0])
CLOCK_R = 0.30
LEGEND_Y = 1.82
SUB_W = 0.27
SUB_DX = 0.145


#region Climate chart
def _month_ticks(axes, *, color=P_WHITE, y_offset=-0.24):
    """🗓️ Twelve single-letter month labels under an axis, evenly spaced."""
    labels = VGroup()
    for i, name in enumerate(MONTHS):
        label = body_text(name, font_size=LABEL_FONT_SIZE - 3, color=color)
        label.set_opacity(0.8)
        label.move_to(axes["pt"]((i + 0.5) / 12, 0) + UP * y_offset)
        labels.add(label)
    return labels


def _chart(y_latex: str, *, ticks=(), color=P_WHITE):
    """📐 Month axes with a typeset unit label, optional tick numerals and faint level lines.

    ``ticks`` are ``(axis fraction, numeral)``; the unit label sits low on the axis so nothing
    crosses the light path into the house.
    """
    axes = labeled_axes(CHART_ORIGIN, x_len=CHART_W, y_len=CHART_H, color=P_WHITE)
    months = _month_ticks(axes)
    numerals = VGroup()
    levels = VGroup()
    for frac, text in ticks:
        n = math_label(text, size=LABEL_FONT_SIZE, color=color)
        n.next_to(axes["pt"](0.0, frac), LEFT, buff=0.12)
        numerals.add(n)
        levels.add(DashedLine(axes["pt"](0.0, frac), axes["pt"](1.0, frac), color=P_WHITE, stroke_width=1.2,
                              stroke_opacity=0.16, dash_length=0.12))
    unit = math_label(y_latex, size=LABEL_FONT_SIZE, color=color).rotate(PI / 2)
    unit.next_to(numerals if len(numerals) else axes["y_axis"], LEFT, buff=0.12)
    unit.set_y(CHART_ORIGIN[1] + 0.15 + unit.height / 2)
    group = VGroup(axes["group"], levels, numerals, unit)
    return {"axes": axes, "pt": axes["pt"], "months": months, "group": group, "levels": levels,
            "numerals": numerals, "unit": unit}


def _right_axis(chart, y_latex: str, ticks, color):
    """➡️ Second value axis on the right edge of the chart, in its own colour."""
    pt = chart["pt"]
    line = Line(pt(1.0, 0.0), pt(1.0, 1.0), color=color, stroke_width=2.0, stroke_opacity=0.7)
    numerals = VGroup(*[math_label(text, size=LABEL_FONT_SIZE, color=color).next_to(pt(1.0, frac), RIGHT, buff=0.12)
                        for frac, text in ticks])
    unit = math_label(y_latex, size=LABEL_FONT_SIZE, color=color).rotate(-PI / 2)
    unit.next_to(numerals, RIGHT, buff=0.12)
    unit.set_y(CHART_ORIGIN[1] + 0.15 + unit.height / 2)
    return VGroup(line, numerals, unit)


def _cell(chart, i: int, side: int, lo: float, hi: float, color, *, opacity=0.85, fill=True, stroke=1.0):
    """▮ One month column between two axis fractions; ``side`` −1 / +1 picks the sink / source half, 0 the full slot."""
    width = SUB_W if side else 0.42
    base = chart["pt"]((i + 0.5) / 12, lo)
    height = max(0.01, (hi - lo) * CHART_H)
    rect = Rectangle(width=width, height=height, color=color, fill_color=color,
                     fill_opacity=opacity if fill else 0.10, stroke_width=stroke)
    return rect.move_to(base + RIGHT * side * SUB_DX + UP * height / 2)


def _layer(chart, month, base: float, cells, *, side: int, color, opacity=0.85, fill=True, stroke=1.0):
    """📊 Month columns that rise as the year cursor passes them — a layer of ``(lo, hi)`` cells, one per month or ``None``."""
    def build():
        reach = month.get_value() - base
        group = VGroup()
        for i, cell in enumerate(cells):
            f = float(np.clip(reach - i, 0.0, 1.0))
            if cell is None or f <= 0.0:
                continue
            lo, hi = cell
            group.add(_cell(chart, i, side, lo, lo + (hi - lo) * f, color, opacity=opacity, fill=fill, stroke=stroke))
        return group

    return always_redraw(build)


def _trace(chart, month, base: float, fn, color, *, width=3.0, dots=False):
    """✏️ Curve ``fn(month position) → axis fraction`` drawn up to the year cursor, with a dot per month once passed."""
    def build():
        reach = float(np.clip(month.get_value() - base, 0.0, 12.0))
        group = VGroup()
        if reach > 0.02:
            xs = np.linspace(0.0, reach, max(3, int(reach * 10) + 1))
            line = VMobject(color=color, stroke_width=width)
            line.set_points_as_corners([chart["pt"](x / 12, fn(x)) for x in xs])
            group.add(line)
        if dots:
            for i in range(12):
                if reach >= i + 0.5:
                    group.add(Dot(chart["pt"]((i + 0.5) / 12, fn(i + 0.5)), radius=0.055, color=color))
        return group

    return always_redraw(build)


def _curve(chart, fn, color, *, width=3.0, opacity=1.0):
    """〰️ Finished smooth curve ``fn(month position) → axis fraction`` over the whole year."""
    xs = np.linspace(0.0, 12.0, 121)
    line = VMobject(color=color, stroke_width=width, stroke_opacity=opacity)
    line.set_points_as_corners([chart["pt"](x / 12, fn(x)) for x in xs])
    return line


def _cursor(chart, month, base: float):
    """📍 Dashed year cursor on the chart, following the month tracker."""
    def build():
        x = float(np.clip(month.get_value() - base, 0.0, 12.0)) / 12
        return DashedLine(chart["pt"](x, 0.0), chart["pt"](x, 1.0), color=P_WHITE, stroke_width=1.6,
                          stroke_opacity=0.5, dash_length=0.1)

    return always_redraw(build)


def _legend(entries):
    """🗝️ Colour swatch plus typeset symbol per entry, laid out on one row above the chart; shown entry by entry."""
    items = {}
    for key, color, latex, filled in entries:
        swatch = Square(side_length=0.17, color=color, stroke_width=2.0, fill_color=color,
                        fill_opacity=0.85 if filled else 0.10)
        label = body_text(latex[1:], font_size=LABEL_FONT_SIZE, color=color) if latex.startswith("=") \
            else math_label(latex, size=LABEL_FONT_SIZE, color=color)
        items[key] = VGroup(swatch, label.next_to(swatch, RIGHT, buff=0.10))
    row = VGroup(*items.values()).arrange(RIGHT, buff=0.32, center=False)
    row.move_to(np.array([CHART_ORIGIN[0], LEGEND_Y, 0.0]), aligned_edge=LEFT)
    return items


def _gauge(symbol: str, *, color, length=1.9, thickness=0.32):
    """📏 0–1 gauge driven by a tracker, with a typeset symbol instead of plain text."""
    g = meter("x", length=length, thickness=thickness, color=color, vertical=True)
    g["group"].remove(g["label"])
    tracker = ValueTracker(0.0)
    bind_meter(g, tracker)
    tag = math_label(symbol, size=BODY_FONT_SIZE, color=color)
    g["tracker"] = tracker
    g["tag"] = tag
    g["group"] = VGroup(g["track"], g["fill"], tag)
    return g
#endregion


#region House stage
def _stand(floor_y: float, x: float, color=P_ORANGE):
    """🧍 Occupant at true scale — 1.75 m in a 2.5 m storey is 70 % of the clear height, feet on the slab's top edge."""
    height = 0.7 * HOUSE_S * 1.2
    return person_glyph(np.array([x, floor_y + 0.02 + height / 2, 0.0]), color=color, scale=height / 0.66)


def _level_flow(paths, color, level_fn, *, waves=7, speed=0.5, radius=0.06):
    """💧 Particle stream whose density follows ``level_fn() ∈ [0, 1]`` — more flow, more particles."""
    dots = VGroup(*[Dot(radius=radius, color=color, stroke_width=0).set_fill(color, opacity=0.0)
                    for _ in range(waves * len(paths))])
    clock = {"t": 0.0}

    def update(group, dt):
        clock["t"] += dt
        level = float(np.clip(level_fn(), 0.0, 1.0))
        for k, dot in enumerate(group):
            p, w = divmod(k, waves)
            t = (clock["t"] * speed + w / waves + 0.31 * p) % 1.0
            dot.move_to(paths[p].point_from_proportion(t))
            on = float(np.clip(level * waves - w, 0.0, 1.0))
            dot.set_fill(color, opacity=max(0.0, min(1.0, t / 0.08, (1.0 - t) / 0.12) * on * 0.9))

    dots.add_updater(update)
    return dots


def _north_window(hs):
    """🧭 Second glazed opening in the right wall of the section — the north façade."""
    br, tr = hs["bottom_right"], hs["top_right"]
    up = hs["windows"][1]
    sill, lintel = up["sill"], up["lintel"]
    wall, inner = hs["wall_right"], hs["walls"][5][3]
    t = 0.08 * HOUSE_S
    lower = Line(br, np.array([br[0], sill, 0.0]), color=P_WHITE, stroke_width=wall.get_stroke_width())
    lower_in = Line(br + LEFT * t, np.array([br[0] - t, sill, 0.0]), color=P_WHITE,
                    stroke_width=inner.get_stroke_width())
    wall.put_start_and_end_on(np.array([br[0], lintel, 0.0]), tr)
    inner.put_start_and_end_on(np.array([br[0] - t, lintel, 0.0]), tr + LEFT * t)
    glass = window_glyph(float(br[0]), sill, lintel, depth=0.42 * HOUSE_S)
    hs["walls"].add(lower, lower_in)
    hs["window_group"].add(glass["group"])
    return glass


def _stage(*, vent_color=LOSS_BLUE, north=False, with_radiator=False):
    """🏠 The Physical Fundamentals house with its weather: sun and straight rays, outdoor thermometer, loss streams,
    occupant, lamp and — for the heating limit — a radiator.

    Every effect follows one of the ``lvl`` trackers, so a beat drives the whole picture from the month or sets a
    state by hand.
    """
    hs = house_section(HOUSE_C, HOUSE_S)
    bl, tl = hs["bottom_left"], hs["top_left"]
    lo, up = hs["windows"]
    floor_y, level_y = float(bl[1]), float(hs["level_1"].get_center()[1])
    x0 = float(bl[0])
    glass_n = _north_window(hs) if north else None

    person = _stand(floor_y, x0 + 1.75)
    lamp = lamp_glyph(np.array([x0 + 1.55, tl[1] - 0.05, 0.0]), drop=0.12)
    sun = sun_glyph(SUN_C).scale(0.42)
    therm = thermometer_glyph(np.array([THERMO_X, -0.80, 0.0]), height=1.30, color=P_BLUE, level=0.4)
    rays = [sun_rays(SUN_C, w["x"], [w["center"][1] + dy for dy in (0.10, -0.10)], fy, gap=0.5)
            for w, fy in zip((lo, up), (floor_y, level_y))]

    def route(y, dy):
        return smooth_path([np.array([x0 + 1.0, y - 0.03, 0.0]), np.array([x0 + 0.35, y, 0.0]),
                            np.array([x0 - 0.35, y + dy / 2, 0.0]), np.array([x0 - 0.85, y + dy, 0.0])])

    p_lo, p_up, p_wall = route(lo["center"][1], 0.08), route(up["center"][1], 0.08), route(0.22, 0.06)
    lvl = {k: ValueTracker(v) for k, v in (("loss", 1.0), ("sun", 1.0), ("heat", 0.0), ("eta", 1.0), ("k", 1.0))}
    flows = [
        _level_flow([p_lo], vent_color, lambda: lvl["loss"].get_value()),
        _level_flow([p_up, p_wall], LOSS_BLUE, lambda: lvl["loss"].get_value()),
    ]
    warm = [person.get_top() + UP * 0.05, lamp["bulb"].get_center() + UP * 0.1]

    def sun_tick(g):
        k = lvl["sun"].get_value()
        g[0].set_fill(opacity=0.10 + 0.30 * k)
        g[2].set_stroke(opacity=0.15 + 0.45 * k)
        g[3].set_stroke(opacity=0.20 + 0.80 * k)

    sun.add_updater(sun_tick)
    for r in rays:
        r["out"].add_updater(lambda g: [ln.set_stroke(opacity=0.95 * lvl["sun"].get_value()) for ln in g])
        r["in"].add_updater(lambda g: [ln.set_stroke(opacity=0.5 * lvl["sun"].get_value()) for ln in g])

    stage = {"hs": hs, "person": person, "lamp": lamp, "sun": sun, "therm": therm, "rays": rays, "flows": flows,
             "lvl": lvl, "warm": warm, "glass_n": glass_n, "x0": x0, "floor_y": floor_y, "level_y": level_y,
             "lo": lo, "up": up}
    if with_radiator:
        rad = radiator(np.array([x0 + 0.62, floor_y + 0.02 + 0.14, 0.0])).scale(0.55)
        rad.move_to(np.array([x0 + 0.62, floor_y + 0.03 + rad.height / 2, 0.0]))
        rise = smooth_path([np.array([x0 + 0.62, rad.get_top()[1] + 0.02, 0.0]), np.array([x0 + 0.78, -0.10, 0.0]),
                            np.array([x0 + 1.0, 0.08, 0.0])])
        flows.append(_level_flow([rise], P_RED, lambda: lvl["heat"].get_value(), waves=5, speed=0.4))

        def rad_tick(g):
            k = lvl["heat"].get_value()
            g.set_stroke(color=interpolate_color(ManimColor(P_WHITE), ManimColor(P_RED), k), opacity=0.45 + 0.55 * k)

        rad.add_updater(rad_tick)
        stage["radiator"] = rad
    return stage


def _stage_glyphs(st):
    """🧩 Everything of the stage that is simply drawn (flows and live effects are added separately)."""
    parts = [st["person"], st["lamp"]["group"], st["sun"], st["therm"]["group"]]
    if "radiator" in st:
        parts.append(st["radiator"])
    return VGroup(*parts)


def _enter_stage(scene, st, *, shine_rays=True, run_time=1.4):
    """🎬 Draw the house, then bring in its inhabitants and weather and start the streams."""
    scene.play(Create(st["hs"]["group"]), run_time=run_time)
    scene.play(FadeIn(_stage_glyphs(st)), run_time=0.8)
    scene.add(st["therm"]["column"], *st["flows"])
    if shine_rays:
        scene.play(*[shine(r) for r in st["rays"]], run_time=1.4)
    else:
        scene.add(*[r["group"] for r in st["rays"]])


def _warm(st, rt: float, r_max: float = 0.34, color=P_RED):
    """♨️ Internal gains: heat rippling off the occupant and the lamp for ``rt`` seconds."""
    return [ripples(st["warm"], r_max=r_max, cycles=max(1.0, rt / 1.5), color=color)]


def _year_clock(month):
    """🕐 Year dial — one turn of the hand is the twelve ticks of the twelve months."""
    clock = clock_glyph(CLOCK_C, r=CLOCK_R)
    dial = VGroup(clock["face"], clock["group"][1])

    def hand():
        a = TAU * (month.get_value() % 12) / 12
        return Line(CLOCK_C, CLOCK_C + CLOCK_R * 0.78 * np.array([np.sin(a), np.cos(a), 0.0]), color=P_CYAN,
                    stroke_width=3)

    return dial, always_redraw(hand)


def _drive(st, month):
    """⚙️ Let the month tracker set every weather effect of the stage: loss, sun, heating, η_h and the thermometer."""
    lvl, therm = st["lvl"], st["therm"]

    def tick(_m):
        m = month.get_value()
        theta = _catmull(THETA_E, m)
        lvl["loss"].set_value(float(np.clip(lvl["k"].get_value() * (THETA_I - theta) / (THETA_I - THETA_E[0]), 0, 1)))
        lvl["sun"].set_value(float(np.clip(_catmull(IRRADIATION, m) / IRRADIATION_PEAK, 0, 1)))
        lvl["heat"].set_value(float(np.clip((THETA_HG - theta) / 3.0, 0, 1)))
        lvl["eta"].set_value(float(np.clip(_catmull(ETA_SHAPE, m), 0, 1)))
        therm["level"].set_value(float(np.clip(_theta_norm(theta), 0, 1)))

    driver = Mobject()
    driver.add_updater(tick)
    return driver


def _wrap_up(scene, st, keep, run_time=0.6):
    """🧹 Stop every live effect, then fade the whole beat out except the chapter header."""
    scene.remove(*st["flows"])
    for mob in scene.mobjects:
        mob.clear_updaters()
    scene.play(*[FadeOut(m) for m in scene.mobjects if m not in keep], run_time=run_time)
    scene.wait(0.4)
#endregion

#endregion


#region Beat 1 – Vom Gesetz zur Rechenmethode

class Beat1_GEGundNormen(Scene):
    NARRATION = [
        ("intro",
         "Everything we just balanced is not a private calculation — in Germany it is prescribed by law.",
         "Alles, was wir gerade bilanziert haben, ist keine private Rechnung — in Deutschland ist sie gesetzlich vorgeschrieben."),
        ("geg",
         "The Gebäudeenergiegesetz sets the target. It caps a building's primary energy demand, but never says how to compute it.",
         "Das Gebäudeenergiegesetz setzt das Ziel: Es begrenzt den Primärenergiebedarf, sagt aber nicht, wie man ihn berechnet."),
        ("din",
         "For that it points to DIN V 18599:2018-09 — the calculation method the GEG references.",
         "Dafür verweist es auf DIN V 18599:2018-09 – die vom GEG referenzierte Berechnungsmethodik."),
        ("nachbarn",
         "Beside it sit the standards from the earlier videos: peak heating load, cooling load, and summer overheating.",
         "Daneben stehen die Normen der früheren Videos: Heizlast, Kühllast und sommerlicher Wärmeschutz."),
    ]

    @staticmethod
    def _norm_card(lines, norm, color, size):
        """🪧 Standard card — topic on one or two lines, the DIN number below, in the shared card frame."""
        topic = VGroup(*[line if isinstance(line, Mobject) else body_text(line, font_size=LABEL_FONT_SIZE, color=P_TEAL)
                         for line in lines]).arrange(DOWN, buff=0.04)
        body = VGroup(topic, body_text(norm, font_size=LABEL_FONT_SIZE, color=color)).arrange(DOWN, buff=0.07)
        frame = RoundedRectangle(width=size[0], height=size[1], corner_radius=0.1, color=color, stroke_width=1.8,
                                 fill_color=P_DEEP_DARK, fill_opacity=0.82)
        body.move_to(frame.get_center())
        return VGroup(frame, body)

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Vom Gesetz zur Rechenmethode", title)
        din = _din_ref("GEG § 20 ff.")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        st = _stage(vent_color=P_CYAN)
        st["lvl"]["loss"].set_value(0.85)
        st["therm"]["level"].set_value(0.35)
        legend = _legend([
            ("t", LOSS_BLUE, r"Q_T", True), ("v", P_CYAN, r"Q_V", True),
            ("s", SOLAR_YELLOW, r"Q_S", True), ("i", P_RED, r"Q_I", True),
        ])
        legend_row = VGroup(*legend.values())
        legend_row.move_to(np.array([HOUSE_C[0], -1.52, 0.0]))
        _enter_stage(self, st, run_time=1.4)
        self.play(LaggedStart(*[FadeIn(e, shift=UP * 0.1) for e in legend_row], lag_ratio=0.25),
                  *_warm(st, 1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 0.8 + 1.4 + 1.4,
                 during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "geg"))
        law = self._law_card()
        law.move_to(np.array([3.6, 1.52, 0.0]))
        goal = note_line("Ziel: den Primärenergiebedarf begrenzen", color=P_TEAL)
        gauge = _gauge(r"Q_P", color=P_TEAL, length=2.2, thickness=0.5)
        gauge["group"][0].move_to(np.array([-1.2, 0.05, 0.0]))
        gauge["fill"].move_to(gauge["track"].get_center())
        gauge["tag"].next_to(gauge["track"], UP, buff=0.16)
        cap_y = gauge["track"].get_bottom()[1] + 0.04 + 0.8 * gauge["span"]
        cap_line = DashedLine(np.array([-1.2 - 0.45, cap_y, 0.0]), np.array([-1.2 + 0.45, cap_y, 0.0]),
                              color=P_RED, stroke_width=3)
        self.play(FadeIn(law, shift=DOWN * 0.15), FadeIn(goal), Create(gauge["track"]), FadeIn(gauge["tag"]),
                  FadeIn(gauge["fill"]), run_time=1.1)
        self.play(Create(cap_line), run_time=0.6)
        self.play(Indicate(cap_line, color=P_RED, scale_factor=1.15), *_warm(st, 1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "geg", during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        method = self._method_card()
        method.move_to(np.array([3.6, 0.32, 0.0]))
        link = Arrow(law.get_bottom(), method.get_top(), buff=0.10, color=P_WHITE, stroke_width=3,
                     max_tip_length_to_length_ratio=0.22)
        revision = note_line("Die Normenreihe wurde 2025 als DIN/TS 18599 überarbeitet.", color=P_TEAL)
        compute = Arrow(method.get_left(), gauge["track"].get_right() + RIGHT * 0.05, buff=0.08, color=P_CYAN,
                        stroke_width=3, max_tip_length_to_length_ratio=0.12)
        self.play(GrowArrow(link), FadeIn(method, shift=DOWN * 0.15), run_time=1.0)
        self.play(FadeOut(goal), FadeIn(revision), GrowArrow(compute), run_time=0.6)
        self.play(gauge["tracker"].animate.set_value(0.62), *_warm(st, 2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "din", during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nachbarn"))
        size = (2.62, 1.18)
        cards = VGroup(
            self._norm_card([VGroup(body_text("Heizlast", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                                    math_label(r"[\mathrm{kW}]", size=LABEL_FONT_SIZE, color=P_TEAL)).arrange(RIGHT, buff=0.1)],
                            "DIN EN 12831-1", LOSS_BLUE, size),
            self._norm_card([VGroup(body_text("Kühllast", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                                    math_label(r"[\mathrm{kW}]", size=LABEL_FONT_SIZE, color=P_TEAL)).arrange(RIGHT, buff=0.1)],
                            "VDI 2078", P_RED, size),
            self._norm_card(["Sommerlicher", "Wärmeschutz"], "DIN 4108-2", SOLAR_YELLOW, size),
        ).arrange(RIGHT, buff=0.2)
        cards.move_to(np.array([2.95, -1.42, 0.0]))
        spokes = VGroup(*[
            Arrow(method.get_bottom(), c.get_top(), buff=0.10, color=P_WHITE, stroke_width=2, stroke_opacity=0.6,
                  max_tip_length_to_length_ratio=0.18)
            for c in cards
        ])
        self.play(FadeOut(compute), FadeOut(gauge["track"]), FadeOut(gauge["fill"]), FadeOut(gauge["tag"]),
                  FadeOut(cap_line), FadeOut(legend_row), run_time=0.5)
        self.play(LaggedStart(*[GrowArrow(a) for a in spokes], lag_ratio=0.18),
                  LaggedStart(*[FadeIn(c, shift=DOWN * 0.12) for c in cards], lag_ratio=0.18), run_time=1.4)
        self._weather_cycle(st, cards, spokes)

        self.play(FadeOut(caption), FadeOut(law), FadeOut(method), FadeOut(revision), FadeOut(link),
                  FadeOut(cards), FadeOut(spokes), run_time=0.6)
        _wrap_up(self, st, keep=[title, subtitle, din], run_time=0.5)

    @staticmethod
    def _law_card():
        from manim_visuals import card_grid
        return card_grid([("Gesetz", "GEG — Gebäudeenergiegesetz", P_TEAL)], rows=1, value_size=LABEL_FONT_SIZE + 2)

    @staticmethod
    def _method_card():
        from manim_visuals import card_grid
        return card_grid([("Rechenverfahren", "DIN V 18599:2018-09 — Energetische Bewertung", P_CYAN)], rows=1,
                         value_size=LABEL_FONT_SIZE + 1)

    def _weather_cycle(self, st, cards, spokes):
        """🌦️ The three neighbouring standards are three weather states of the one house, in the order they are named."""
        lvl, therm = st["lvl"], st["therm"]
        sun = st["sun"]
        moon = moon_glyph(SUN_C).scale(1.5).move_to(SUN_C)
        air = Polygon(st["hs"]["bottom_left"] + RIGHT * 0.08 + UP * 0.02, st["hs"]["bottom_right"] + LEFT * 0.08 + UP * 0.02,
                      st["hs"]["top_right"] + LEFT * 0.08 + DOWN * 0.06, st["hs"]["roof_peak"] + DOWN * 0.14,
                      st["hs"]["top_left"] + RIGHT * 0.08 + DOWN * 0.06, stroke_width=0, fill_color=P_RED,
                      fill_opacity=0.0).set_z_index(-1)
        slats = VGroup(*[
            Line(np.array([w["x"] - 0.40, y - 0.07, 0.0]), np.array([w["x"] - 0.14, y + 0.07, 0.0]),
                 color=P_TEAL, stroke_width=3)
            for w in (st["lo"], st["up"]) for y in np.linspace(w["sill"] + 0.07, w["lintel"] - 0.07, 4)
        ])
        self.add(air)
        rays = VGroup(*[r["group"] for r in st["rays"]])

        def mark(i):
            return [Indicate(cards[i], color=cards[i][0].get_color(), scale_factor=1.06),
                    spokes[i].animate.set_stroke(opacity=1.0, width=3.5)]

        def settle(i):
            return [spokes[i].animate.set_stroke(opacity=0.6, width=2)]

        sun.suspend_updating()
        self.play(Transform(sun, moon), FadeOut(rays), lvl["sun"].animate.set_value(0.0),
                  therm["level"].animate.set_value(0.1), lvl["loss"].animate.set_value(1.0), *mark(0), *_warm(st, 2.0, 0.2),
                  run_time=2.0)
        self.play(*settle(0), run_time=0.1)
        self.play(Transform(sun, sun_glyph(SUN_C).scale(0.42)), FadeIn(rays), lvl["sun"].animate.set_value(1.0),
                  therm["level"].animate.set_value(0.95), lvl["loss"].animate.set_value(0.12),
                  air.animate.set_fill(P_RED, opacity=0.22), *mark(1), *_warm(st, 2.2, 0.45), run_time=2.2)
        sun.resume_updating()
        self.play(*settle(1), run_time=0.1)
        self.play(Create(slats), lvl["sun"].animate.set_value(0.35), air.animate.set_fill(P_RED, opacity=0.06),
                  *mark(2), *_warm(st, 2.0, 0.25), run_time=2.0)
        hold_for(self, self.NARRATION, "nachbarn", during=lambda rt: _warm(st, rt, 0.25))

#endregion


#region Beat 2 – Warum Monat für Monat bilanziert wird

class Beat2_Monatsbilanz(Scene):
    NARRATION = [
        ("intro",
         "The standard balances month by month. Here is why an annual average would be wrong.",
         "Die Norm bilanziert Monat für Monat. Hier ist der Grund, warum ein Jahresmittel falsch wäre."),
        ("verluste",
         "These are the monthly losses — high in winter, small in summer, following the outdoor temperature.",
         "Das sind die monatlichen Verluste — im Winter hoch, im Sommer klein, entlang der Außentemperatur."),
        ("gewinne",
         "And these are the usable gains. Where green covers blue, sun and internal loads already pay the bill.",
         "Und das sind die nutzbaren Gewinne. Wo Grün das Blau deckt, zahlen Sonne und interne Lasten die Rechnung."),
        ("rest",
         "Only the red remainder has to be heated. In summer there is no heating remainder — those unused gains sit in the cooling balance instead of paying January.",
         "Nur der rote Rest muss beheizt werden. Im Sommer bleibt kein Heizrest — ungenutzte Gewinne gehören in die Kühlbilanz, nicht in den Januar."),
        ("jahr",
         "An annual average would credit that summer surplus against January, and make the demand look far too small.",
         "Ein Jahresmittel würde diesen Sommerüberschuss dem Januar gutschreiben — der Bedarf erschiene viel zu klein."),
        ("summe",
         "So the annual heating demand is the sum of twelve separate monthly balances, never one averaged one.",
         "Der Jahresheizwärmebedarf ist deshalb die Summe von zwölf getrennten Monatsbilanzen, nie eine gemittelte."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Warum Monat für Monat bilanziert wird", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        month = ValueTracker(0.0)
        st = _stage()
        lvl = st["lvl"]
        chart = _chart(r"\mathrm{kWh/Monat}")
        dial, hand = _year_clock(month)
        driver = _drive(st, month)
        legend = _legend([
            ("sink", LOSS_BLUE, r"Q_{\mathrm{Senke}}", False),
            ("src", SOLAR_YELLOW, r"Q_{\mathrm{Quelle}}", True),
            ("use", ETA_GREEN, r"\eta_h\cdot Q_{\mathrm{Quelle}}", True),
            ("rest", DEMAND_RED, r"Q_h", True),
            ("unused", P_TEAL, r"=ungenutzt", True),
        ])
        self.play(Create(chart["group"]), FadeIn(chart["months"]), FadeIn(dial), run_time=1.0)
        _enter_stage(self, st, run_time=1.2)
        self.add(driver, hand)
        self.play(month.animate.set_value(12.0), *_warm(st, 3.6), run_time=3.6, rate_func=linear)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 1.2 + 0.8 + 1.4 + 3.6,
                 during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verluste"))
        sink = _layer(chart, month, base, [(0.0, v) for v in LOSS_SHAPE], side=-1, color=LOSS_BLUE, fill=False, stroke=2.0)
        cursor = _cursor(chart, month, base)
        self.add(sink, cursor)
        self.play(month.animate.set_value(base + 12), FadeIn(legend["sink"]), *_warm(st, 6.5),
                  run_time=6.5, rate_func=linear)
        sink.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "verluste", during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gewinne"))
        src = _layer(chart, month, base, [(0.0, v) for v in GAIN_SHAPE], side=1, color=SOLAR_YELLOW, opacity=0.6)
        use_src = _layer(chart, month, base, [(0.0, v) for v in USABLE_SHAPE], side=1, color=ETA_GREEN, opacity=0.95)
        use_snk = _layer(chart, month, base, [(0.0, v) for v in USABLE_SHAPE], side=-1, color=ETA_GREEN, opacity=0.95)
        cursor = _cursor(chart, month, base)
        eta = _gauge(r"\eta_h", color=ETA_GREEN, length=1.9, thickness=0.30)
        eta["group"][0].move_to(np.array([6.1, -0.30, 0.0]))
        eta["fill"].move_to(eta["track"].get_center())
        eta["tag"].next_to(eta["track"], UP, buff=0.14)
        eta["fill"].add_updater(lambda _f: eta["tracker"].set_value(lvl["eta"].get_value()))
        self.add(src, use_src, use_snk, cursor)
        self.play(FadeIn(eta["track"]), FadeIn(eta["fill"]), FadeIn(eta["tag"]), FadeIn(legend["src"]),
                  FadeIn(legend["use"]), run_time=0.5)
        self.play(month.animate.set_value(base + 12), *_warm(st, 7.0), run_time=7.0, rate_func=linear)
        for layer in (src, use_src, use_snk):
            layer.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "gewinne", during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rest"))
        rest = _layer(chart, month, base, [(USABLE_SHAPE[i], LOSS_SHAPE[i]) if i in REST_MONTHS else None
                                           for i in range(12)], side=-1, color=DEMAND_RED, opacity=0.92)
        cursor = _cursor(chart, month, base)
        self.add(rest, cursor)
        self.play(month.animate.set_value(base + 12), FadeIn(legend["rest"]), *_warm(st, 7.0),
                  run_time=7.0, rate_func=linear)
        rest.clear_updaters()
        self.remove(cursor)
        self.play(src.animate.set_fill(P_TEAL, opacity=0.7), FadeIn(legend["unused"]), run_time=1.0)
        hold_for(self, self.NARRATION, "rest", during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "jahr"))
        warn = note_line("Sommerüberschuss zählt nicht für die Heizbilanz.", color=P_RED)
        rest_cells = list(rest)
        credit = 1.04 / sum(REST_SHAPE)
        shrunk = VGroup(*[_cell(chart, i, -1, USABLE_SHAPE[i], USABLE_SHAPE[i] + REST_SHAPE[i] * credit, DEMAND_RED,
                                opacity=0.92) for i in REST_MONTHS])
        surplus = [i for i in range(12) if UNUSED_SHAPE[i] > 0.1]
        winter = [0, 11, 1, 10, 2, 9]
        chunks = VGroup(*[_cell(chart, i, 1, USABLE_SHAPE[i], GAIN_SHAPE[i], P_TEAL, opacity=0.9) for i in surplus])
        targets = [rest_cells[REST_MONTHS.index(j)].get_top() for j in winter[:len(surplus)]]
        self.play(FadeIn(warn), FadeIn(chunks), run_time=0.6)
        self.play(*[c.animate.move_to(t).scale(0.25).set_opacity(0.0) for c, t in zip(chunks, targets)],
                  Transform(rest, shrunk), *_warm(st, 1.8), run_time=1.8)
        wrong = cross_mark(P_RED, 0.28).move_to(chart["pt"](0.42, 0.78))
        self.play(FadeIn(wrong, scale=0.6), run_time=0.4)
        hold_for(self, self.NARRATION, "jahr", during=lambda rt: _warm(st, rt))
        full = VGroup(*[_cell(chart, i, -1, USABLE_SHAPE[i], LOSS_SHAPE[i], DEMAND_RED, opacity=0.92)
                        for i in REST_MONTHS])
        self.play(FadeOut(wrong), FadeOut(warn), Transform(rest, full), run_time=0.8)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summe"))
        unit = 0.5
        bar = stacked_bar([(None, REST_SHAPE[i], DEMAND_RED) for i in REST_MONTHS], base=np.array([6.1, -1.28, 0.0]),
                          width=0.5, unit=unit)
        total = math_label(r"Q_{h,a}", size=BODY_FONT_SIZE, color=DEMAND_RED).next_to(bar["top"], UP, buff=0.16)
        row, box, items = math_panel([
            ("qh", r"Q_{h,a}", DEMAND_RED), (None, "=", P_WHITE),
            ("sum", r"\Sigma_{\text{12 Monate}}", P_WHITE),
            (None, "(", P_WHITE), ("sink", r"Q_{\mathrm{Senke}}", LOSS_BLUE), (None, "-", P_WHITE),
            ("eta", r"\eta_h", ETA_GREEN), (None, r"\cdot", P_WHITE), ("src", r"Q_{\mathrm{Quelle}}", SOLAR_YELLOW),
            (None, ")", P_WHITE),
        ], size=FORMULA_FONT_SIZE - 3, buff=0.12)
        self.play(FadeOut(eta["track"]), FadeOut(eta["fill"]), FadeOut(eta["tag"]), run_time=0.4)
        self.play(LaggedStart(*[ReplacementTransform(c, b) for c, b in zip(rest_cells, bar["bars"])], lag_ratio=0.1),
                  FadeIn(total), run_time=1.6)
        self.play(Create(box), FadeIn(row), run_time=0.9)
        ring = highlight_param(items, "qh", color=P_ORANGE)
        self.play(Create(ring), Indicate(bar["bars"], color=DEMAND_RED, scale_factor=1.04), run_time=0.8)
        ring2 = highlight_param(items, "sum", color=P_ORANGE)
        self.play(ReplacementTransform(ring, ring2), Indicate(bar["bars"], color=P_ORANGE),
                  run_time=0.8)
        ring3 = highlight_param(items, "sink", color=P_ORANGE)
        self.play(ReplacementTransform(ring2, ring3), Indicate(sink, color=LOSS_BLUE), run_time=0.8)
        hold_for(self, self.NARRATION, "summe", during=lambda rt: _warm(st, rt))

        self.remove(driver, hand)
        self.play(FadeOut(box), FadeOut(row), FadeOut(ring3), run_time=0.4)
        _wrap_up(self, st, keep=[title, subtitle, din])

#endregion


#region Beat 3 – Klimadaten: das Testreferenzjahr

class Beat3_Klimadaten(Scene):
    NARRATION = [
        ("intro",
         "Every one of those twelve balances needs two inputs from outside the building: temperature and sun.",
         "Jede dieser zwölf Bilanzen braucht zwei Eingangsgrößen von außen: Temperatur und Sonne."),
        ("temperatur",
         "The monthly mean outdoor temperature drives the losses — it is the delta theta in every transmission term.",
         "Die mittlere monatliche Außentemperatur treibt die Verluste — sie ist das Delta Theta in jedem Transmissionsterm."),
        ("strahlung",
         "The monthly irradiation per orientation drives the solar gains, which is why a south window counts more than a north one.",
         "Die monatliche Einstrahlung je Orientierung treibt die solaren Gewinne — deshalb zählt ein Südfenster mehr als ein Nordfenster."),
        ("try",
         "Both come from the German weather service as a test reference year, a synthetic but representative twelve months.",
         "Beides liefert der Deutsche Wetterdienst als Testreferenzjahr — zwölf synthetische, aber repräsentative Monate."),
        ("potsdam",
         "For certain GEG proofs a standardised reference climate is fixed, and the Potsdam reference climate zone plays the central role in it.",
         "Für bestimmte GEG-Nachweise ist ein standardisiertes Referenzklima festgelegt; die Referenzklimazone Potsdam spielt dabei eine zentrale Rolle."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Klimadaten: das Testreferenzjahr", title)
        din = _din_ref("DIN V 18599-10 · DWD TRY")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        month = ValueTracker(0.0)
        st = _stage(north=True)
        lvl = st["lvl"]
        theta_fn = lambda x: _theta_norm(_catmull(THETA_E, x))  # noqa: E731
        chart = _chart(r"\theta_e\,[\mathrm{°C}]", ticks=[(_theta_norm(v), str(v)) for v in (0, 10, 20)], color=P_BLUE)
        right = _right_axis(chart, r"I_S\,[\mathrm{kWh/m^2}]",
                            [(v / IRRADIATION_FULL_KWH, str(v)) for v in (0, 100, 200)], SOLAR_YELLOW)
        dial, hand = _year_clock(month)
        driver = _drive(st, month)
        theta_tag = math_label(r"\theta_e", size=BODY_FONT_SIZE, color=P_BLUE).move_to(np.array([THERMO_X, 0.82, 0.0]))
        sun_tag = math_label(r"I_S", size=BODY_FONT_SIZE, color=SOLAR_YELLOW).move_to(np.array([-5.85, 1.82, 0.0]))
        lvl["loss"].set_value(0.8)
        lvl["sun"].set_value(0.85)

        self.play(Create(chart["group"]), FadeIn(chart["months"]), FadeIn(dial), run_time=1.0)
        _enter_stage(self, st, run_time=1.2)
        self.play(FadeIn(theta_tag, shift=DOWN * 0.1), FadeIn(sun_tag, shift=DOWN * 0.1), *_warm(st, 1.0), run_time=1.0)
        self.play(Indicate(theta_tag, color=P_BLUE), Indicate(sun_tag, color=SOLAR_YELLOW), *_warm(st, 1.2),
                  run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 1.2 + 0.8 + 1.4 + 1.0 + 1.2,
                 during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "temperatur"))
        theta = _trace(chart, month, base, theta_fn, P_BLUE, dots=True)
        indoor = DashedLine(chart["pt"](0.0, _theta_norm(THETA_I)), chart["pt"](1.0, _theta_norm(THETA_I)),
                            color=P_ORANGE, stroke_width=2.0, stroke_opacity=0.8)
        indoor_tag = math_label(r"\theta_i", size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(indoor, UP, buff=0.04).align_to(
            indoor, RIGHT)

        def delta():
            x = float(np.clip(month.get_value() - base, 0.0, 12.0)) / 12
            lo_pt = chart["pt"](x, theta_fn(x * 12))
            hi_pt = chart["pt"](x, _theta_norm(THETA_I))
            arrow = dim_arrow(lo_pt, hi_pt, P_ORANGE) if hi_pt[1] - lo_pt[1] > 0.3 else VGroup()
            tag = math_label(r"\Delta\theta", size=LABEL_FONT_SIZE, color=P_ORANGE)
            tag.next_to(hi_pt, RIGHT, buff=0.12).shift(UP * 0.0)
            tag.set_y(hi_pt[1] + 0.22)
            return VGroup(arrow, tag)

        spread = always_redraw(delta)
        cursor = _cursor(chart, month, base)
        self.add(theta, cursor, driver, hand)
        self.play(Create(indoor), FadeIn(indoor_tag), run_time=0.7)
        self.add(spread)
        self.play(month.animate.set_value(base + 12), *_warm(st, 6.5), run_time=6.5, rate_func=linear)
        theta.clear_updaters()
        self.remove(cursor, spread)
        hold_for(self, self.NARRATION, "temperatur", during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "strahlung"))
        irr_fn = lambda x: _catmull(IRRADIATION, x)  # noqa: E731
        sun_cols = _layer(chart, month, base, [(0.0, v) for v in IRRADIATION], side=0, color=SOLAR_YELLOW, opacity=0.55)
        cursor = _cursor(chart, month, base)
        self.add(sun_cols, cursor)
        self.play(FadeOut(indoor), FadeOut(indoor_tag), FadeIn(right), run_time=0.5)
        self.play(month.animate.set_value(base + 12), *_warm(st, 6.5), run_time=6.5, rate_func=linear)
        sun_cols.clear_updaters()
        self.remove(cursor)
        self._orientation(st, month, base + 12 + 6.5)
        hold_for(self, self.NARRATION, "strahlung", during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "try"))
        source = self._source_card().move_to(np.array([2.0, 1.72, 0.0]))
        years = VGroup(*[
            _curve(chart, lambda x, k=k: _theta_norm(_catmull(THETA_E, x) + 1.2 * np.sin(1.7 * x + 1.9 * k)),
                   P_BLUE, width=2.0, opacity=0.4)
            for k in range(4)
        ])
        mean = VGroup(*[_curve(chart, theta_fn, P_BLUE, width=3.0) for _ in range(4)])
        self.play(FadeIn(years), FadeIn(source, shift=DOWN * 0.12), run_time=1.0)
        self.play(month.animate.set_value(base + 12), Transform(years, mean), *_warm(st, 6.0), run_time=6.0,
                  rate_func=linear)
        self.remove(years)
        hold_for(self, self.NARRATION, "try", during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "potsdam"))
        ref_note = note_line("GEG-Nachweis: standardisiertes Referenzklima — Referenzklimazone Potsdam zentral",
                             color=P_CYAN)
        pin = chip("Potsdam", P_CYAN, font_size=LABEL_FONT_SIZE).move_to(np.array([2.3, 1.62, 0.0]))
        leader = Line(pin.get_bottom(), chart["pt"](6.5 / 12, theta_fn(6.5)), color=P_CYAN, stroke_width=1.6,
                      stroke_opacity=0.8)
        self.play(FadeOut(source), FadeIn(ref_note), FadeIn(pin), Create(leader), run_time=0.9)
        self.play(Indicate(theta, color=P_CYAN, scale_factor=1.03), Indicate(sun_cols, color=P_CYAN, scale_factor=1.03),
                  *_warm(st, 1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "potsdam", during=lambda rt: _warm(st, rt))

        self.remove(driver, hand)
        self.play(FadeOut(ref_note), FadeOut(pin), FadeOut(leader), run_time=0.4)
        _wrap_up(self, st, keep=[title, subtitle, din])

    @staticmethod
    def _source_card():
        from manim_visuals import card_grid
        return card_grid([("Datenquelle", "DWD · Testreferenzjahr (TRY)", P_TEAL)], rows=1, value_size=LABEL_FONT_SIZE + 1)

    def _orientation(self, st, month, target):
        """🧭 Same summer sun, two façades: the south window takes the rays, the north one only the diffuse sky."""
        hs = st["hs"]
        br = hs["bottom_right"]
        up = hs["windows"][1]
        sky_c = np.array([-0.95, 1.18, 0.0])
        marker = Rectangle(width=0.62, height=0.10, stroke_width=0, fill_opacity=0).move_to(sky_c)
        sky = cloud_glyph(marker, pad=(0.30, 0.22), bump=0.4)
        diffuse = sun_rays(sky_c, float(br[0]), [up["center"][1] + 0.08, up["center"][1] - 0.08], st["level_y"], gap=0.5)
        diffuse["out"].set_stroke(opacity=0.38)
        diffuse["in"].set_stroke(opacity=0.2)
        south = body_text("Süd", font_size=LABEL_FONT_SIZE, color=SOLAR_YELLOW).move_to(
            np.array([float(hs["bottom_left"][0]) + 0.1, -1.38, 0.0]))
        north = body_text("Nord", font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(np.array([float(br[0]) - 0.1, -1.38, 0.0]))
        self.play(month.animate.set_value(target), run_time=1.4, rate_func=linear)
        self.play(FadeIn(sky), FadeIn(south), FadeIn(north), run_time=0.7)
        self.play(Create(diffuse["out"]), run_time=0.6)
        self.play(Create(diffuse["in"]), *[shine(r) for r in st["rays"]], run_time=1.0)

#endregion


#region Beat 4 – Heizgrenze & Gradtagszahl

class Beat4_Gradtagszahl(Scene):
    NARRATION = [
        ("intro",
         "One number compresses that whole weather year into a single figure engineers actually quote.",
         "Eine Kennzahl verdichtet dieses ganze Wetterjahr zu einer Zahl, die Planer wirklich nennen."),
        ("heizgrenze",
         "Above a defined reference temperature the free gains cover the losses — a calculation convention, not a physical limit.",
         "Oberhalb einer festgelegten Referenztemperatur decken die freien Gewinne die Verluste — eine Rechenkonvention, keine physikalische Grenze."),
        ("flaeche",
         "Below it, every degree and every day counts. The area shows the principle — the real degree-day figure comes from daily values on the heating days.",
         "Darunter zählt jedes Grad und jeder Tag. Die Fläche zeigt das Prinzip — real wird die Gradtagszahl aus Tageswerten der Heiztage berechnet."),
        ("nutzen",
         "As a simplified indicator of temperature-dependent losses it makes a cold winter comparable with a mild one.",
         "Als vereinfachter Indikator temperaturabhängiger Verluste macht sie einen kalten mit einem milden Winter vergleichbar."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Heizgrenze & Gradtagszahl", title)
        din = _din_ref("VDI 3807 · DIN 4108-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        month = ValueTracker(0.0)
        st = _stage(with_radiator=True)
        lvl = st["lvl"]
        theta_fn = lambda x: _theta_norm(_catmull(THETA_E, x))  # noqa: E731
        limit_f = _theta_norm(THETA_HG)
        chart = _chart(r"\theta_e\,[\mathrm{°C}]", ticks=[(_theta_norm(v), str(v)) for v in (0, 10, 20)], color=P_BLUE)
        dial, hand = _year_clock(month)
        driver = _drive(st, month)

        self.play(Create(chart["group"]), FadeIn(chart["months"]), FadeIn(dial), run_time=1.0)
        _enter_stage(self, st, run_time=1.2)
        self.add(driver, hand)
        base = month.get_value()
        theta = _trace(chart, month, base, theta_fn, P_BLUE, dots=True)
        cursor = _cursor(chart, month, base)
        self.add(theta, cursor)
        self.play(month.animate.set_value(base + 12), *_warm(st, 6.5), run_time=6.5, rate_func=linear)
        theta.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 1.2 + 0.8 + 1.4 + 6.5,
                 during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "heizgrenze"))
        limit = DashedLine(chart["pt"](0.0, limit_f), chart["pt"](1.0, limit_f), color=P_ORANGE, stroke_width=2.5)
        limit_tag = math_label(r"\theta_{HG}=15\,\mathrm{°C}", size=BODY_FONT_SIZE, color=P_ORANGE)
        limit_tag.next_to(chart["pt"](1.0, limit_f), UP, buff=0.10).align_to(chart["pt"](1.0, limit_f), RIGHT)
        mark_y = st["therm"]["top"][1] - 1.30 + 0.05 + (1.30 - 0.12) * limit_f
        therm_mark = Line(np.array([THERMO_X - 0.2, mark_y, 0.0]), np.array([THERMO_X + 0.2, mark_y, 0.0]),
                          color=P_ORANGE, stroke_width=3)
        self.play(Create(limit), FadeIn(limit_tag), Create(therm_mark), run_time=1.0)
        base = month.get_value()
        strips = always_redraw(lambda: self._heating_strips(chart, month, base))
        cursor = _cursor(chart, month, base)
        self.add(strips, cursor)
        self.play(month.animate.set_value(base + 12), *_warm(st, 7.0), run_time=7.0, rate_func=linear)
        strips.clear_updaters()
        self.remove(cursor)
        hold_for(self, self.NARRATION, "heizgrenze", during=lambda rt: _warm(st, rt))

        base = month.get_value()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flaeche"))
        area = self._degree_area(chart, month, base, shift=0.0, color=P_ORANGE, opacity=0.34)
        gt = math_readout(lambda: rf"G_t={de_num(_degree_days(float(np.clip(month.get_value() - base, 0, 12))), 0)}"
                                  r"\,\mathrm{Kd/a}",
                          np.array([0.85, LEGEND_Y - 0.1, 0.0]), size=BODY_FONT_SIZE, color=P_ORANGE, edge="left")
        cursor = _cursor(chart, month, base)
        self.add(area, cursor, gt)
        self.play(month.animate.set_value(base + 12), *_warm(st, 7.0), run_time=7.0, rate_func=linear)
        area.clear_updaters()
        self.remove(cursor)
        daily = self._daily_values(chart)
        daily_note = note_line("Prinzipdarstellung — reale Gradtagszahlen aus Tageswerten", color=P_ORANGE)
        self.play(Create(daily, run_time=1.4, rate_func=linear), FadeIn(daily_note), run_time=1.4)
        hold_for(self, self.NARRATION, "flaeche", during=lambda rt: _warm(st, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nutzen"))
        self.remove(driver)
        g_cold, g_mild, g_ref = _degree_days(12.0, -2.0), _degree_days(12.0, 2.0), _degree_days(12.0)
        cold = _curve(chart, lambda x: _theta_norm(_catmull(THETA_E, x) - 2.0), P_BLUE, width=3.0)
        mild = _curve(chart, lambda x: _theta_norm(_catmull(THETA_E, x) + 2.0), P_CYAN, width=3.0)
        cold_area = self._degree_area(chart, ValueTracker(12.0), 0.0, shift=-2.0, color=P_BLUE, opacity=0.34)
        mild_area = self._degree_area(chart, ValueTracker(12.0), 0.0, shift=2.0, color=P_CYAN, opacity=0.34)
        cold_area.clear_updaters()
        mild_area.clear_updaters()
        cold_gt = math_label(rf"G_{{t,\mathrm{{kalt}}}}={de_num(g_cold, 0)}\,\mathrm{{Kd/a}}", size=BODY_FONT_SIZE,
                             color=P_BLUE).move_to(np.array([-0.2, LEGEND_Y - 0.1, 0.0]))
        mild_gt = math_label(rf"G_{{t,\mathrm{{mild}}}}={de_num(g_mild, 0)}\,\mathrm{{Kd/a}}", size=BODY_FONT_SIZE,
                             color=P_CYAN).move_to(np.array([3.6, LEGEND_Y - 0.1, 0.0]))
        row, box, items = math_panel([
            ("q", r"Q_{\text{temp.-abh.}}", LOSS_BLUE), (None, "~", P_WHITE), ("gt", r"G_t", P_ORANGE),
            (None, r"\to", P_WHITE), (None, r"\text{vereinfachtes Prinzip der Witterungsbereinigung}", P_TEAL),
        ], size=FORMULA_FONT_SIZE - 4)
        self.remove(gt)
        self.play(FadeOut(daily), FadeOut(daily_note), FadeOut(area), theta.animate.set_stroke(opacity=0.35),
                  FadeOut(limit_tag), run_time=0.6)
        self.play(Create(cold), FadeIn(cold_area), FadeIn(cold_gt), lvl["loss"].animate.set_value(1.0),
                  st["therm"]["level"].animate.set_value(_theta_norm(THETA_E[0] - 2.0)), *_warm(st, 2.2, 0.4),
                  run_time=2.2)
        self.play(Create(mild), FadeIn(mild_area), FadeIn(mild_gt), lvl["loss"].animate.set_value(g_mild / g_cold),
                  st["therm"]["level"].animate.set_value(_theta_norm(THETA_E[0] + 2.0)), *_warm(st, 2.2, 0.25),
                  run_time=2.2)
        self.play(Create(box), FadeIn(row), run_time=0.9)
        ring = highlight_param(items, "gt", color=P_ORANGE)
        self.play(Create(ring), Indicate(VGroup(cold_area, mild_area), color=P_ORANGE, scale_factor=1.02), run_time=0.8)
        hold_for(self, self.NARRATION, "nutzen", during=lambda rt: _warm(st, rt))

        self.remove(hand)
        self.play(FadeOut(box), FadeOut(row), FadeOut(ring), run_time=0.4)
        _wrap_up(self, st, keep=[title, subtitle, din])

    @staticmethod
    def _degree_area(chart, month, base, *, shift, color, opacity):
        """🟧 Area between the heating limit and the outdoor curve, filled up to the year cursor."""
        limit_f = _theta_norm(THETA_HG)

        def build():
            reach = float(np.clip(month.get_value() - base, 0.0, 12.0))
            group = VGroup()
            if reach <= 0.02:
                return group
            run = []

            def flush():
                if len(run) > 1:
                    pts = [chart["pt"](x / 12, _theta_norm(t)) for x, t in run]
                    pts += [chart["pt"](x / 12, limit_f) for x, _t in reversed(run)]
                    group.add(Polygon(*pts, fill_color=color, fill_opacity=opacity, stroke_width=0))
                run.clear()

            for x in np.linspace(0.0, reach, max(3, int(reach * 24) + 1)):
                t = _catmull(THETA_E, x) + shift
                if t < THETA_HG:
                    run.append((float(x), t))
                else:
                    flush()
            flush()
            return group

        return always_redraw(build)

    @staticmethod
    def _heating_strips(chart, month, base):
        """🔥 The heating season marked on the time axis as far as the year cursor has come."""
        reach = float(np.clip(month.get_value() - base, 0.0, 12.0))
        group = VGroup()
        run = []

        def flush():
            if len(run) > 1:
                line = Line(chart["pt"](run[0] / 12, 0.0), chart["pt"](run[-1] / 12, 0.0), color=P_ORANGE,
                            stroke_width=7, stroke_opacity=0.85)
                group.add(line.set_cap_style(CapStyleType.ROUND))
            run.clear()

        for x in np.linspace(0.0, reach, max(3, int(reach * 12) + 1)):
            if _catmull(THETA_E, x) < THETA_HG:
                run.append(float(x))
            else:
                flush()
        flush()
        return group

    @staticmethod
    def _daily_values(chart):
        """📆 Daily outdoor temperatures scatter around the monthly mean — the basis of the real G_t."""
        rng = np.random.default_rng(11)
        xs = np.linspace(0.0, 12.0, 183)
        walk = np.cumsum(rng.normal(0.0, 1.0, xs.size))
        walk -= np.linspace(walk[0], walk[-1], xs.size)
        walk = 2.2 * walk / max(1e-6, float(np.abs(walk).max())) + rng.normal(0.0, 0.9, xs.size)
        line = VMobject(color=P_BLUE, stroke_width=1.3, stroke_opacity=0.5)
        line.set_points_as_corners([chart["pt"](x / 12, float(np.clip(_theta_norm(_catmull(THETA_E, x) + w), 0.0, 1.0)))
                                    for x, w in zip(xs, walk)])
        return line

#endregion
