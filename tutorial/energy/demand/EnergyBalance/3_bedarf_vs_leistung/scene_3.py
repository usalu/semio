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
    NOTE_BAND_Y,
    smooth_path, flow_guides,
    meter, bind_meter, cross_mark, dim_arrow,
    math_label, math_panel, math_readout, de_num, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    house_section, radiator, sun_glyph, sun_rays, shine, pulse_flashes, person_glyph, lamp_glyph,
    thermometer_glyph, ripples, energy_tank,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Bedarf vs. Leistung"

POWER = P_RED
ENERGY = P_CYAN
TIME = P_YELLOW
H_T = P_ORANGE
H_V = P_BLUE
DELTA_T = P_YELLOW

PROFILE_N = 8760
PLOT_N = 365
PEAK_KW = 10.0
DESIGN_DT = 32.0
THETA_I = 20.0
THETA_E = -12.0

HOUSE_S = 0.8
BASE_Y = -1.15
CHART_W, CHART_H = 6.1, 2.7
#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs
# The whole chapter is told on one dataset: a synthetic year of heating power.
# Beat 1 reads its height and its area, Beat 3 sorts it into a load-duration
# curve, Beat 4 draws candidate plant capacities across it. The same house
# breathes that year: its particle streams carry the instantaneous power, its
# tank collects the energy, and the chart is the same story drawn as a curve.

#region Data
def _annual_load(n: int = PROFILE_N, seed: int = 5) -> np.ndarray:
    """📉 Synthetic annual heating-power profile, normalised to its own peak.

    Shape only — a cosine season plus multi-day weather driving ``θ_i − θ_e``
    above the heating limit, one sample per hour of the year. It is deliberately
    not measured data; multiplying by ``PEAK_KW`` is what gives the beats real
    kW/kWh.
    """
    rng = np.random.default_rng(seed)
    t = np.linspace(0.0, 1.0, n)
    window = 240
    noise = np.convolve(rng.normal(0.0, 1.0, n + 2 * window), np.ones(window) / window, "same")[window:window + n]
    theta_e = 9.0 - 9.5 * np.cos(2 * np.pi * t) + 3.0 * noise / noise.std()
    load = np.maximum(0.0, 15.0 - theta_e)
    return load / load.max()


def _plot_samples(values: np.ndarray, n: int = PLOT_N) -> np.ndarray:
    """📉 Evenly spaced picks of an hourly series — what the chart actually draws."""
    return values[np.linspace(0, len(values) - 1, n).round().astype(int)]


def _cumulative(load: np.ndarray) -> np.ndarray:
    """∑ Energy in kWh after each hour — the rectangle rule with Δt = 1 h."""
    return np.concatenate([[0.0], np.cumsum(load) * PEAK_KW])


def _energy_at(cum: np.ndarray, frac: float) -> float:
    """∑ Energy collected up to a fraction of the year, in kWh."""
    return float(np.interp(frac * (len(cum) - 1), np.arange(len(cum)), cum))
#endregion


#region Stage
def _stage(base_y: float = BASE_Y, chart_h: float = CHART_H) -> dict:
    """🎬 Positions of the house, the energy tank and the chart on one shared baseline."""
    return {"house": np.array([-4.45, base_y + 1.2 * HOUSE_S, 0.0]), "tank_h": chart_h - 0.2,
            "tank": np.array([-2.0, base_y + (chart_h - 0.2) / 2, 0.0]), "chart": np.array([-0.65, base_y, 0.0]),
            "chart_h": chart_h, "base": base_y}
#endregion


#region Chart
def _chart(origin, x_len: float, y_len: float, x_src: str, y_src: str):
    """📐 Axis cross with arrow tips, typeset axis names and a unit-square mapper."""
    o = np.array(origin, dtype=float)
    x_ax = Arrow(o, o + RIGHT * x_len, buff=0, stroke_width=2.4, color=P_WHITE,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.06)
    y_ax = Arrow(o, o + UP * y_len, buff=0, stroke_width=2.4, color=P_WHITE,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.08)
    xl = math_label(x_src, size=LABEL_FONT_SIZE, color=P_WHITE).next_to(x_ax.get_end(), RIGHT, buff=0.14)
    yl = math_label(y_src, size=LABEL_FONT_SIZE, color=P_WHITE).next_to(y_ax.get_end(), UP, buff=0.10)

    def pt(fx: float, fy: float):
        return o + RIGHT * float(fx) * x_len + UP * float(fy) * y_len

    return {"o": o, "x": x_ax, "y": y_ax, "xl": xl, "yl": yl, "pt": pt, "x_len": x_len, "y_len": y_len,
            "group": VGroup(x_ax, y_ax, xl, yl)}


def _kw_ticks(chart, y_max_kw: float, values=(5, 10)):
    """📏 Power ticks on the y axis — the height of the curve read off in kW."""
    group = VGroup()
    for v in values:
        f = v / y_max_kw
        group.add(Line(chart["pt"](0, f) + LEFT * 0.07, chart["pt"](0, f) + RIGHT * 0.07, color=P_WHITE, stroke_width=2))
        group.add(math_label(rf"{v}", chart["pt"](0, f) + LEFT * 0.20 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right"))
    return group


def _month_labels(chart):
    """🗓️ Twelve month names under the year axis."""
    names = ("Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez")
    return VGroup(*[
        body_text(m, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).move_to(chart["pt"]((i + 0.5) / 12, 0) + DOWN * 0.24)
        for i, m in enumerate(names)
    ])


def _hour_labels(chart, values=(0, 2000, 4000, 6000, 8760)):
    """⏱️ Hour ticks under the sorted axis."""
    group = VGroup()
    for h in values:
        f = h / PROFILE_N
        group.add(Line(chart["pt"](f, 0) + DOWN * 0.06, chart["pt"](f, 0) + UP * 0.06, color=P_WHITE, stroke_width=2))
        group.add(math_label(de_num(h), chart["pt"](f, 0) + DOWN * 0.38, size=LABEL_FONT_SIZE - 2))
    return group


def _partial(chart, ys, frac: float, *, color=POWER, width: float = 3.0):
    """📈 The series drawn up to a fraction of the axis."""
    xs = np.linspace(0.0, 1.0, len(ys))
    k = int(np.searchsorted(xs, frac, side="right"))
    pts = [chart["pt"](x, float(y)) for x, y in zip(xs[:k], ys[:k])]
    if k < len(ys):
        pts.append(chart["pt"](frac, float(np.interp(frac, xs, ys))))
    if len(pts) < 2:
        pts = [chart["pt"](0, float(ys[0])), chart["pt"](0, float(ys[0])) + RIGHT * 0.001]
    return VMobject(color=color, stroke_width=width).set_points_as_corners(pts)


def _partial_area(chart, ys, frac: float, *, color=ENERGY, opacity: float = 0.24):
    """🟦 The area under the series up to a fraction of the axis."""
    xs = np.linspace(0.0, 1.0, len(ys))
    k = int(np.searchsorted(xs, frac, side="right"))
    pts = [chart["pt"](x, float(y)) for x, y in zip(xs[:k], ys[:k])]
    if k < len(ys):
        pts.append(chart["pt"](frac, float(np.interp(frac, xs, ys))))
    pts += [chart["pt"](max(frac, 0.0005), 0.0), chart["pt"](0.0, 0.0)]
    return Polygon(*pts, fill_color=color, fill_opacity=opacity, stroke_width=0)
#endregion


#region House and flow
def _heated_house(center, scale: float):
    """🏠 Physical-Fundamentals house with a radiator under each window and five heat-loss paths.

    Two window paths start at the radiators, two wall paths and one roof path start in the room air.
    ``spots`` are the warm points that radiate long-wave heat.
    """
    hs = house_section(center, scale)
    bl, br = hs["bottom_left"], hs["bottom_right"]
    top_y = float(hs["top_left"][1])
    storey = hs["w_height"] / 2
    floors = (float(bl[1]), float(hs["level_1"].get_center()[1]))
    rad_scale = 0.26 * storey / 0.5
    rads = VGroup()
    for fy in floors:
        r = radiator(ORIGIN, color=POWER).scale(rad_scale)
        r.move_to(np.array([bl[0] + 0.15 * scale + 0.1 + r.width / 2, fy + r.height / 2 + 0.03, 0.0]))
        rads.add(r)
    win_paths = []
    for w, r in zip(hs["windows"], rads):
        x, y = float(w["x"]), float(w["center"][1])
        start = r.get_top() + UP * 0.03
        win_paths.append(smooth_path([start, np.array([x + 0.10, y - 0.10, 0.0]), np.array([x - 0.30, y - 0.02, 0.0]),
                                      np.array([x - 0.95, y + 0.12, 0.0])]))
    wall_paths = [
        smooth_path([np.array([br[0] - 0.85 * scale - 0.2, fy + 0.4 * storey, 0.0]),
                     np.array([br[0] - 0.1, fy + 0.4 * storey + 0.02, 0.0]),
                     np.array([br[0] + 0.2, fy + 0.4 * storey + 0.12, 0.0]),
                     np.array([br[0] + 0.42, fy + 0.4 * storey + 0.52, 0.0])])
        for fy in floors
    ]
    cx = float(hs["center"][0])
    roof_x = cx + 0.8 * scale
    roof_y = top_y + 1.1 * scale * (1 - (0.8 * scale) / (1.8 * scale))
    roof_path = smooth_path([np.array([roof_x, top_y - 0.28 * scale, 0.0]), np.array([roof_x, top_y - 0.02, 0.0]),
                             np.array([roof_x + 0.03, roof_y + 0.2, 0.0]), np.array([roof_x + 0.07, roof_y + 0.5, 0.0])])
    return {
        "hs": hs, "rads": rads, "floors": floors, "storey": storey,
        "win_paths": win_paths, "wall_paths": wall_paths, "roof_path": roof_path,
        "paths": [*win_paths, *wall_paths, roof_path],
        "guides": flow_guides([*win_paths, *wall_paths, roof_path], POWER, opacity=0.22),
        "spots": [r.get_top() + UP * 0.05 for r in rads],
    }


def _stand(floor_y: float, x: float, storey: float, color=P_ORANGE):
    """🧍 Occupant at true scale — 70 % of the room height, feet on the floor line at ``x``."""
    height = 0.7 * storey
    return person_glyph(np.array([x, floor_y + 0.015 + height / 2, 0.0]), color=color, scale=height / 0.66)


def _rate_stream(paths, level, rt: float, *, speed: float = 0.8, per_path: int = 8, color=POWER, color_end=P_ORANGE,
                 radius: float = 0.075):
    """💨 Particle stream whose density is the rate — ``level()`` in 0–1 decides how many particles run.

    Every particle moves at one speed, so a thicker stream means more energy per second, never a faster one.
    """
    start_c, end_c = ManimColor(color), ManimColor(color_end)
    parts, meta = VGroup(), []
    for i, path in enumerate(paths):
        for k in range(per_path):
            p = Ellipse(width=radius * 2.8, height=radius * 1.15, color=color, stroke_width=0, fill_opacity=0.0)
            p.move_to(path.point_from_proportion(0.0))
            parts.add(p)
            meta.append((path, k, (0.618 * k + 0.31 * i) % 1.0))
    cycles = speed * rt

    def update(group, alpha):
        lv = float(np.clip(level(), 0.0, 1.0)) * per_path
        for p, (path, k, off) in zip(group, meta):
            t = (alpha * cycles + off) % 1.0
            p.move_to(path.point_from_proportion(t))
            t0, t1 = max(0.0, t - 0.02), min(1.0, t + 0.02)
            tangent = path.point_from_proportion(t1) - path.point_from_proportion(t0)
            p.set_angle(float(np.arctan2(tangent[1], tangent[0])))
            fade = min(1.0, t / 0.08, (1.0 - t) / 0.10) * float(np.clip(lv - k, 0.0, 1.0))
            p.set_fill(interpolate_color(start_c, end_c, t), opacity=max(0.0, 0.92 * fade))

    return UpdateFromAlphaFunc(parts, update, remover=True, rate_func=linear)


def _heat(spots, rt: float, *, r_max: float = 0.35, color=POWER):
    """🌡️ Long-wave heat spreading from the warm radiators for ``rt`` seconds."""
    return ripples(spots, r_max=r_max, cycles=rt / 1.4, color=color)


def _life(house, level, rt: float, *, r_max: float = 0.35, paths=None, speed: float = 0.8):
    """🔥 The house at work for ``rt`` seconds — a stream per loss path plus heat ripples."""
    return [_rate_stream(house["paths"] if paths is None else paths, level, rt, speed=speed), _heat(house["spots"], rt, r_max=r_max)]


def _ramp(tracker, to: float, *, rate_func=smooth):
    """⏩ Move a tracker to ``to`` — pass ``linear`` where the movement itself must stay proportional to time."""
    start = tracker.get_value()
    return UpdateFromAlphaFunc(tracker, lambda m, a: m.set_value(start + (to - start) * a), rate_func=rate_func)
#endregion


#region Formula panel
def _snap(mob):
    """📸 Static copy of a live readout — safe to fly into a formula slot."""
    snap = mob.copy()
    snap.clear_updaters()
    for m in snap.get_family():
        m.clear_updaters()
    return snap


def _fill_panel(scene, panel, sources, *, run_time: float = 1.3):
    """🧲 Every named formula slot is fed by the drawn quantity it stands for, then the rest of the formula appears.

    ``sources`` maps a slot key to the on-screen readout it is read from; a copy of the readout morphs into its slot.
    """
    row, box, items = panel
    targets = [items[key] for key in sources]
    rest = VGroup(*[m for m in row.submobjects if all(m is not t for t in targets)])
    for part in row.get_family():
        part._layout_zone = "formula"
    scene.play(*[TransformFromCopy(VGroup(*[_snap(m) for m in src]) if isinstance(src, (list, tuple)) else _snap(src),
                                   items[key]) for key, src in sources.items()], run_time=run_time * 0.75)
    scene.play(FadeIn(rest), Create(box), run_time=0.6)


def _clear_stage(scene, keep, run_time: float = 0.6):
    """🧹 Fade out everything but the persistent title block, stopping the live updaters first."""
    leaving = [m for m in scene.mobjects if all(m is not k for k in keep)]
    for m in leaving:
        m.clear_updaters()
    scene.play(*[FadeOut(m) for m in leaving], run_time=run_time)
#endregion
#endregion


#region Beat 1 – Leistung ist eine Rate, Energie eine Menge

class Beat1_LeistungUndEnergie(Scene):
    NARRATION = [
        ("intro",
         "Two numbers describe every heating system, and mixing them up is the classic beginner's mistake.",
         "Zwei Zahlen beschreiben jede Heizung — sie zu verwechseln ist der klassische Anfängerfehler."),
        ("kurve",
         "This is one year of heating power for our building: high on cold days, zero all summer.",
         "Das ist ein Jahr Heizleistung unseres Gebäudes: hoch an kalten Tagen, den ganzen Sommer null."),
        ("leistung",
         "Power is the height of the curve at one moment — kilowatts, how fast energy flows right now.",
         "Leistung ist die Höhe der Kurve in einem Moment — Kilowatt, wie schnell Energie gerade fließt."),
        ("energie",
         "Energy is the running sum of power over time — the area under the curve. Its unit is the kilowatt-hour.",
         "Energie ist die zeitliche Summe der Leistung — die Fläche unter der Kurve. Einheit: Kilowattstunden."),
        ("formel",
         "Power times time gives energy. That single relation from the fundamentals separates the load from the demand.",
         "Leistung mal Zeit ergibt Energie. Diese eine Beziehung aus den Grundlagen trennt Last von Bedarf."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Leistung ist eine Rate, Energie eine Menge", title)
        din = _din_ref("Grundlagen · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ys = _plot_samples(load)
        xs = np.linspace(0.0, 1.0, len(ys))
        cum = _cumulative(load)
        q_year = round(float(cum[-1]), -2)
        year = ValueTracker(1.0)

        def p_frac():
            return float(np.interp(year.get_value(), xs, ys))

        stage = _stage()
        house = _heated_house(stage["house"], HOUSE_S)
        hs = house["hs"]
        tank = energy_tank(stage["tank"], height=stage["tank_h"], width=0.7, color=ENERGY, level=0.0)
        chart = _chart(stage["chart"], CHART_W, stage["chart_h"], r"t", r"P\;[\mathrm{kW}]")
        kw_ticks = _kw_ticks(chart, PEAK_KW)
        months = _month_labels(chart)
        p_tag = math_label(r"P\;[\mathrm{kW}]", house["win_paths"][1].get_end() + UP * 0.5 + RIGHT * 0.35,
                           size=LABEL_FONT_SIZE, color=POWER)
        q_tag = math_label(r"Q\;[\mathrm{kWh}]", tank["cap"].get_top() + UP * 0.28, size=LABEL_FONT_SIZE, color=ENERGY)

        self.play(Create(hs["group"]), run_time=1.2)
        self.play(FadeIn(house["guides"]), run_time=0.4)
        self.play(FadeIn(house["rads"]), FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(p_tag), FadeIn(q_tag),
                  *_life(house, lambda: 0.7, 0.9), run_time=0.9)
        self.play(Create(chart["group"]), FadeIn(kw_ticks), FadeIn(months),
                  *_life(house, lambda: 0.7, 1.0), run_time=1.0)
        hold_for(self, N, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 0.9 + 1.0,
                 during=lambda rt: [*_life(house, lambda: 0.7, rt), _ramp(tank["level"], 0.3, rate_func=linear)])

        caption = swap_caption(self, caption, subtitle_text(N, "kurve"))
        year.set_value(0.0)
        curve = always_redraw(lambda: _partial(chart, ys, year.get_value()))
        knob = always_redraw(lambda: Dot(chart["pt"](year.get_value(), p_frac()), radius=0.07, color=POWER))
        p_read = math_readout(lambda: rf"P = {de_num(p_frac() * PEAK_KW, 1)}\,\mathrm{{kW}}",
                              np.array([0.7, 1.30, 0.0]), color=POWER)
        self.add(curve, knob)
        self.play(_ramp(tank["level"], 0.0), FadeIn(p_read), *_life(house, p_frac, 0.8), run_time=0.8)
        self.play(_ramp(year, 1.0, rate_func=linear), *_life(house, p_frac, 5.0), run_time=5.0)
        hold_for(self, N, "kurve", during=lambda rt: _life(house, p_frac, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "leistung"))
        stem = always_redraw(lambda: DashedLine(chart["pt"](year.get_value(), 0), chart["pt"](year.get_value(), p_frac()),
                                                color=P_YELLOW, stroke_width=2.4))
        guide = always_redraw(lambda: DashedLine(chart["pt"](0, p_frac()), chart["pt"](year.get_value(), p_frac()),
                                                 color=P_YELLOW, stroke_width=1.6, stroke_opacity=0.6))
        self.add(stem, guide)
        self.play(_ramp(year, 0.06), *_life(house, p_frac, 1.2), run_time=1.2)
        self.play(_ramp(year, 0.55), *_life(house, p_frac, 2.4), run_time=2.4)
        self.play(_ramp(year, 0.94), *_life(house, p_frac, 2.4), run_time=2.4)
        hold_for(self, N, "leistung", during=lambda rt: _life(house, p_frac, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "energie"))
        area = always_redraw(lambda: _partial_area(chart, ys, year.get_value()))
        q_read = math_readout(lambda: rf"Q = {de_num(round(_energy_at(cum, year.get_value()), -2))}\,\mathrm{{kWh}}",
                              np.array([0.7, 0.80, 0.0]), color=ENERGY)
        tank["level"].add_updater(lambda m: m.set_value(_energy_at(cum, year.get_value()) / cum[-1]))
        self.add(area, q_read)
        self.play(FadeOut(stem), FadeOut(guide), _ramp(year, 0.0), *_life(house, p_frac, 0.8), run_time=0.8)
        self.play(_ramp(year, 1.0, rate_func=linear), *_life(house, p_frac, 4.5), run_time=4.5)
        q_final = math_label(rf"Q \approx {de_num(q_year)}\,\mathrm{{kWh/a}}", np.array([0.7, 0.80, 0.0]),
                             size=BODY_FONT_SIZE, color=ENERGY, edge="left")
        self.play(ReplacementTransform(q_read, q_final), *_life(house, p_frac, 0.8), run_time=0.8)
        hold_for(self, N, "energie", during=lambda rt: _life(house, p_frac, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "formel"))
        cell_c = np.array([4.15, 1.25, 0.0])
        cell = Rectangle(width=0.95, height=0.62, color=ENERGY, stroke_width=2, fill_color=ENERGY, fill_opacity=0.30)
        cell.move_to(cell_c)
        cell_h = math_label(r"1\,\mathrm{h}", cell.get_top() + UP * 0.30, size=LABEL_FONT_SIZE, color=TIME)
        cell_p = math_label(r"1\,\mathrm{kW}", cell.get_left() + LEFT * 0.12 + DOWN * 0.07, size=LABEL_FONT_SIZE, color=POWER,
                            edge="right")
        cell_q = math_label(r"1\,\mathrm{kWh}", cell.get_center() + DOWN * 0.07, size=LABEL_FONT_SIZE, color=ENERGY)
        self.play(FadeIn(cell), FadeIn(cell_h), FadeIn(cell_p), FadeIn(cell_q),
                  *_life(house, p_frac, 0.9), run_time=0.9)
        panel = math_panel([
            ("q", "Q", ENERGY), (None, "=", P_WHITE), (None, r"\Sigma", P_WHITE), ("p", "P_{i}", POWER),
            (None, r"\cdot", P_WHITE), ("dt", r"\Delta t", TIME), (None, r"\quad", P_WHITE),
            ("u1", r"1\,\mathrm{kW}", POWER), (None, r"\cdot", P_WHITE), ("u2", r"1\,\mathrm{h}", TIME),
            (None, "=", P_WHITE), ("u3", r"1\,\mathrm{kWh}", ENERGY),
        ])
        _fill_panel(self, panel, {"q": q_final, "p": p_read, "dt": cell_h, "u1": cell_p, "u2": cell_h, "u3": cell_q})
        hold_for(self, N, "formel", during=lambda rt: _life(house, p_frac, rt))

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion


#region Beat 2 – Die Normheizlast nach DIN EN 12831-1

def _lvl(temp: float) -> float:
    """🌡️ Thermometer fill for a temperature on the −15…25 °C scale shared by both glyphs."""
    return float(np.clip((temp + 15.0) / 40.0, 0.0, 1.0))


def _signed(value: float, digits: int = 0) -> str:
    """➖ German signed number for ``math_text`` — a real minus sign for negatives."""
    return ("−" if value < 0 else "") + de_num(abs(value), digits)


class Beat2_Normheizlast(Scene):
    NARRATION = [
        ("intro",
         "The peak of that curve has its own standard, its own symbol and its own rules: the design heat load.",
         "Die Spitze dieser Kurve hat eine eigene Norm, ein eigenes Formelzeichen und eigene Regeln: die Norm-Heizlast."),
        ("bedingungen",
         "It is computed for one artificial worst case: the design outdoor temperature of the site, room by room.",
         "Sie wird für einen künstlichen Extremfall berechnet: die Norm-Außentemperatur des Standorts, Raum für Raum."),
        ("keine",
         "And crucially: only the losses under these design conditions count — solar or internal gains are not credited as reliable heating power.",
         "Entscheidend: Nur die Verluste unter diesen Normbedingungen zählen — solare oder interne Gewinne gelten nicht als verlässliche Heizleistung."),
        ("formel",
         "So, as a schematic simplification, the load is the transmission and ventilation heat transfer coefficients times the design difference.",
         "Als schematische Vereinfachung ist die Heizlast der Transmissions- und Lüftungs-Wärmetransferkoeffizient mal der Auslegungsdifferenz."),
        ("einheit",
         "The result is kilowatts, not kilowatt-hours — an important basis for sizing the whole heating system.",
         "Das Ergebnis sind Kilowatt, nicht Kilowattstunden — eine wichtige Grundlage für die Auslegung der Heizungsanlage."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        H_T_VAL, H_V_VAL = 225.0, 87.5

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Norm-Heizlast nach DIN EN 12831-1", title)
        din = _din_ref("DIN EN 12831-1 · DIN/TS 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ys = _plot_samples(load)
        scale = 0.9
        house = _heated_house(np.array([-1.9, BASE_Y + 1.2 * scale, 0.0]), scale)
        hs, storey = house["hs"], house["storey"]
        floor0, floor1 = house["floors"]
        walls_t = [house["wall_paths"][1], house["roof_path"]]
        walls_v = house["win_paths"]
        every = [*walls_v, *walls_t]
        guides = flow_guides(every, POWER, opacity=0.22)

        theta_e = ValueTracker(8.0)
        d_theta = lambda: THETA_I - theta_e.get_value()
        phi_kw = lambda: d_theta() / DESIGN_DT * PEAK_KW
        level = lambda: d_theta() / DESIGN_DT

        bulb_y = floor0 + 0.17
        th_h = 0.9
        th_i = thermometer_glyph(np.array([hs["bottom_right"][0] - 0.38, bulb_y, 0.0]), height=th_h, color=P_RED,
                                 level=_lvl(THETA_I))
        th_e = thermometer_glyph(np.array([hs["bottom_right"][0] + 0.38, bulb_y, 0.0]), height=th_h, color=P_BLUE,
                                 level=_lvl(theta_e.get_value()))
        th_e["level"].add_updater(lambda m: m.set_value(_lvl(theta_e.get_value())))
        top = lambda temp: bulb_y + 0.05 + (th_h - 0.12) * _lvl(temp)
        arrow_x = hs["bottom_right"][0] + 0.95
        row_x = arrow_x + 0.3
        guide_i = always_redraw(lambda: DashedLine(np.array([hs["bottom_right"][0] - 0.28, top(THETA_I), 0.0]),
                                                   np.array([arrow_x, top(THETA_I), 0.0]), color=P_YELLOW,
                                                   stroke_width=1.4, stroke_opacity=0.6))
        guide_e = always_redraw(lambda: DashedLine(np.array([hs["bottom_right"][0] + 0.5, top(theta_e.get_value()), 0.0]),
                                                   np.array([arrow_x, top(theta_e.get_value()), 0.0]), color=P_YELLOW,
                                                   stroke_width=1.4, stroke_opacity=0.6))
        span = always_redraw(lambda: dim_arrow(np.array([arrow_x, top(theta_e.get_value()), 0.0]),
                                               np.array([arrow_x, top(THETA_I), 0.0]), color=DELTA_T))
        row_i = math_readout(lambda: rf"\theta_{{\mathrm{{int}}}} = {de_num(THETA_I)}\,\mathrm{{°C}}",
                             np.array([row_x, -0.12, 0.0]), size=LABEL_FONT_SIZE, color=P_RED)
        row_e = math_readout(lambda: rf"\theta_{{\mathrm{{e}}}} = {_signed(theta_e.get_value())}\,\mathrm{{°C}}",
                             np.array([row_x, -0.52, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE)
        row_d = math_readout(lambda: rf"\Delta\theta = {de_num(d_theta())}\,\mathrm{{K}}",
                             np.array([row_x, -0.97, 0.0]), size=BODY_FONT_SIZE, color=DELTA_T)
        phi_read = math_readout(lambda: rf"\Phi_{{\mathrm{{HL}}}} = {de_num(phi_kw(), 1)}\,\mathrm{{kW}}",
                                np.array([row_x, 0.55, 0.0]), size=BODY_FONT_SIZE, color=POWER)

        mini = _chart(np.array([4.0, BASE_Y, 0.0]), 2.7, 2.2, r"t", r"P\;[\mathrm{kW}]")
        peak_i = int(np.argmax(ys))
        peak_x = peak_i / (len(ys) - 1)
        mini_curve = _partial(mini, ys, 1.0)
        peak_dot = Dot(mini["pt"](peak_x, 1.0), radius=0.08, color=POWER)
        peak_tag = math_label(r"\Phi_{\mathrm{HL}}", mini["pt"](peak_x, 1.0) + LEFT * (0.5 if peak_x > 0.5 else -0.5) + UP * 0.13,
                              size=BODY_FONT_SIZE, color=POWER)
        peak_line = always_redraw(lambda: DashedLine(mini["pt"](0, min(level(), 1.0)), mini["pt"](1, min(level(), 1.0)),
                                                     color=POWER, stroke_width=2.0, stroke_opacity=0.85))

        self.play(Create(hs["group"]), run_time=1.2)
        self.play(FadeIn(house["rads"]), FadeIn(guides), run_time=0.5)
        self.play(Create(mini["group"]), run_time=0.8, *_life(house, level, 0.8, paths=every))
        self.play(Create(mini_curve), run_time=1.2, *_life(house, level, 1.2, paths=every))
        self.play(FadeIn(peak_dot, scale=2.0), FadeIn(peak_tag), Indicate(peak_dot, color=POWER, scale_factor=1.8),
                  *_life(house, level, 1.0, paths=every), run_time=1.0)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 0.5 + 0.8 + 1.2 + 1.0,
                 during=lambda rt: _life(house, level, rt, paths=every))

        caption = swap_caption(self, caption, subtitle_text(N, "bedingungen"))
        self.add(th_i["column"], th_e["column"], guide_i, guide_e, span)
        self.play(FadeIn(th_i["group"]), FadeIn(th_e["group"]), FadeIn(row_i), FadeIn(row_e), FadeIn(row_d),
                  FadeIn(phi_read), FadeIn(peak_line), *_life(house, level, 1.0, paths=every), run_time=1.0)
        self.play(_ramp(theta_e, THETA_E), *_life(house, level, 3.6, paths=every, r_max=0.45), run_time=3.6)
        rooms = [Rectangle(width=hs["w_width"] - 0.2, height=storey - 0.06, color=P_TEAL, stroke_width=2.6)
                 .move_to(np.array([hs["center"][0], fy + storey / 2, 0.0])) for fy in (floor0, floor1)]
        self.play(Create(rooms[0]), *_life(house, level, 0.9, paths=every), run_time=0.9)
        self.play(ReplacementTransform(rooms[0], rooms[1]), *_life(house, level, 0.9, paths=every), run_time=0.9)
        self.play(FadeOut(rooms[1]), *_life(house, level, 0.4, paths=every), run_time=0.4)
        hold_for(self, N, "bedingungen", during=lambda rt: _life(house, level, rt, paths=every))

        caption = swap_caption(self, caption, subtitle_text(N, "keine"))
        sun = sun_glyph(np.array([-5.7, 1.3, 0.0])).scale(0.6)
        up_win = hs["windows"][1]
        rays = sun_rays(sun.get_center(), float(up_win["x"]),
                        [up_win["lintel"] - 0.08, (up_win["sill"] + up_win["lintel"]) / 2, up_win["sill"] + 0.08],
                        floor1 + 0.02, gap=0.55)
        ray_paths = [[s, h, l] for s, h, l in zip(rays["starts"], rays["hits"], rays["lands"])]
        person = _stand(floor1, hs["bottom_right"][0] - 0.65, storey)
        lamp = lamp_glyph(np.array([hs["center"][0] - 0.1, float(hs["top_left"][1]), 0.0]), drop=0.1)
        lamp["group"].scale(0.6, about_point=np.array([hs["center"][0] - 0.1, float(hs["top_left"][1]), 0.0]))
        gain_spots = [person.get_top() + UP * 0.04]
        lamp_spot = [lamp["bulb"].get_center() + DOWN * 0.1]
        sun_lbl = VGroup(body_text("solare Gewinne", font_size=LABEL_FONT_SIZE, color=P_YELLOW),
                         cross_mark(size=0.11)).arrange(RIGHT, buff=0.14)
        sun_lbl.move_to(np.array([-5.6, 0.2, 0.0]))
        int_lbl = VGroup(body_text("interne Gewinne", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
                         cross_mark(size=0.11)).arrange(RIGHT, buff=0.14)
        int_lbl.move_to(np.array([-5.6, -0.25, 0.0]))

        def gains(rt):
            return [*_life(house, level, rt, paths=every, r_max=0.45),
                    ripples(gain_spots, r_max=0.4, cycles=rt / 1.4, color=P_ORANGE),
                    ripples(lamp_spot, r_max=0.35, cycles=rt / 1.4, color=P_ORANGE, down=True),
                    pulse_flashes(ray_paths, P_YELLOW, repeats=2, width=4.0)]

        self.play(FadeIn(sun), FadeIn(person), FadeIn(lamp["group"]), *_life(house, level, 0.8, paths=every),
                  run_time=0.8)
        self.play(shine(rays), *_life(house, level, 1.6, paths=every), run_time=1.6)
        self.play(FadeIn(sun_lbl), FadeIn(int_lbl), *gains(1.0), run_time=1.0)
        hold_for(self, N, "keine", during=gains)

        caption = swap_caption(self, caption, subtitle_text(N, "formel"))
        t_lbl = math_label(rf"H_{{T}} = {de_num(H_T_VAL)}\,\mathrm{{W/K}}", np.array([-0.85, 1.72, 0.0]),
                           size=LABEL_FONT_SIZE, color=H_T, edge="left")
        v_lbl = math_label(rf"H_{{V}} = {de_num(H_V_VAL, 1)}\,\mathrm{{W/K}}", np.array([-6.2, -0.05, 0.0]),
                           size=LABEL_FONT_SIZE, color=H_V, edge="left")
        note = math_label(r"\text{schematisch vereinfacht — die Norm enthält weitere Terme}",
                          np.array([0.0, NOTE_BAND_Y - 0.07, 0.0]), size=LABEL_FONT_SIZE, color=P_TEAL)
        split_guides = VGroup(flow_guides(walls_t, H_T, opacity=0.3), flow_guides(walls_v, H_V, opacity=0.3))

        def split(rt):
            return [_rate_stream(walls_t, level, rt, color=H_T, color_end=H_T),
                    _rate_stream(walls_v, level, rt, color=H_V, color_end=H_V),
                    _heat(house["spots"], rt, r_max=0.4)]

        self.play(FadeOut(sun), FadeOut(person), FadeOut(lamp["group"]), FadeOut(rays["group"]), FadeOut(sun_lbl),
                  FadeOut(int_lbl), FadeOut(guides), FadeIn(split_guides), FadeIn(t_lbl), FadeIn(v_lbl), FadeIn(note),
                  *split(1.2), run_time=1.2)
        panel = math_panel([
            (None, r"\Phi_{\mathrm{HL}}", POWER), (None, "=", P_WHITE), (None, "(", P_WHITE),
            (None, "H_{T}", H_T), (None, "+", P_WHITE), (None, "H_{V}", H_V), (None, ")", P_WHITE),
            (None, r"\cdot", P_WHITE), (None, r"\Delta\theta", DELTA_T), (None, "=", P_WHITE),
            (None, "(", P_WHITE), ("t", de_num(H_T_VAL), H_T), (None, "+", P_WHITE), ("v", de_num(H_V_VAL, 1), H_V),
            (None, ")", P_WHITE), (None, r"\mathrm{W/K}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("d", rf"{de_num(DESIGN_DT)}\,\mathrm{{K}}", DELTA_T), (None, "=", P_WHITE),
            ("phi", rf"{de_num(PEAK_KW)}\,\mathrm{{kW}}", POWER),
        ], size=FORMULA_FONT_SIZE - 5)
        _fill_panel(self, panel, {"t": t_lbl, "v": v_lbl, "d": row_d, "phi": phi_read})
        hold_for(self, N, "formel", during=split)

        caption = swap_caption(self, caption, subtitle_text(N, "einheit"))
        area = _partial_area(mini, ys, 1.0)
        kw_tag = math_label(r"\text{Anlagengröße}", mini["pt"](1.0, 1.0) + UP * 0.2, size=LABEL_FONT_SIZE, color=POWER,
                            edge="right")
        kwh_tag = math_label(r"Q_h\;[\mathrm{kWh/a}]\quad\text{Betriebskosten}", mini["pt"](0.5, 0.0) + DOWN * 0.3,
                             size=LABEL_FONT_SIZE, color=ENERGY)
        ring = highlight_param(panel[2], "phi", color=POWER)
        self.play(FadeOut(note), FadeIn(area), Create(ring), FadeIn(kw_tag), FadeIn(kwh_tag),
                  *split(1.4), run_time=1.4)
        hold_for(self, N, "einheit", during=split)

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion


#region Beat 3 – Die Jahresdauerlinie

class Beat3_Jahresdauerlinie(Scene):
    NARRATION = [
        ("intro",
         "Load and demand live on the same curve. One picture shows both at once — sort the year by size.",
         "Last und Bedarf stecken in derselben Kurve. Ein Bild zeigt beides — man sortiert das Jahr nach Größe."),
        ("sortieren",
         "Take every hour of the year and line them up from the coldest to the mildest. That is the load duration curve.",
         "Man reiht jede Stunde des Jahres von der kältesten bis zur mildesten auf. Das ist die Jahresdauerlinie."),
        ("spitze",
         "Its starting height is the design heat load — the few hours the plant must still be able to cover.",
         "Ihre Anfangshöhe ist die Norm-Heizlast — die wenigen Stunden, die die Anlage noch abdecken muss."),
        ("flaeche",
         "Its area is the annual heating demand — useful energy, not the billed consumption.",
         "Ihre Fläche ist der Jahres-Heizwärmebedarf — Nutzenergie, nicht der abgerechnete Verbrauch."),
        ("volllast",
         "Area divided by peak gives the full-load hours: a building runs at full load only a fraction of the year.",
         "Fläche geteilt durch Spitze ergibt die Vollbenutzungsstunden: Volllast läuft ein Gebäude nur einen Bruchteil des Jahres."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Jahresdauerlinie", title)
        din = _din_ref("DIN EN 12831-1 · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ordered = np.sort(load)[::-1]
        ys_year, ys_sorted = _plot_samples(load), _plot_samples(ordered)
        xs = np.linspace(0.0, 1.0, PLOT_N)
        cum = _cumulative(ordered)
        q_year = round(float(cum[-1]), -2)
        t_vl = q_year / PEAK_KW
        full = t_vl / PROFILE_N
        series = {"ys": ys_year}
        x = ValueTracker(0.0)

        def p_frac():
            return float(np.interp(x.get_value(), xs, series["ys"]))

        stage = _stage(-0.85, 2.4)
        house = _heated_house(stage["house"], HOUSE_S)
        hs = house["hs"]
        tank = energy_tank(stage["tank"], height=stage["tank_h"], width=0.7, color=ENERGY, level=0.0)
        chart = _chart(stage["chart"], CHART_W, stage["chart_h"], r"t", r"P\;[\mathrm{kW}]")
        kw_ticks = _kw_ticks(chart, PEAK_KW)
        months = _month_labels(chart)
        hours = _hour_labels(chart)
        q_tag = math_label(r"Q\;[\mathrm{kWh}]", tank["cap"].get_top() + UP * 0.28, size=LABEL_FONT_SIZE, color=ENERGY)

        curve = always_redraw(lambda: _partial(chart, ys_year, x.get_value()))
        knob = always_redraw(lambda: Dot(chart["pt"](x.get_value(), p_frac()), radius=0.07, color=POWER))

        self.play(Create(hs["group"]), run_time=1.2)
        self.play(FadeIn(house["rads"]), FadeIn(house["guides"]), FadeIn(tank["group"]), FadeIn(tank["fill"]),
                  FadeIn(q_tag), Create(chart["group"]), FadeIn(kw_ticks), FadeIn(months), run_time=1.0)
        self.add(curve, knob)
        self.play(_ramp(x, 1.0, rate_func=linear), *_life(house, p_frac, 3.0), run_time=3.0)
        curve.clear_updaters()
        area = _partial_area(chart, ys_year, 1.0, opacity=0.14)
        peak_i = int(np.argmax(ys_year))
        tag_load = math_label(r"\text{Last}\;=\;\text{Höhe}", chart["pt"](peak_i / (PLOT_N - 1), 1.0) + LEFT * 1.0 + UP * 0.2,
                              size=LABEL_FONT_SIZE, color=POWER)
        tag_demand = math_label(r"\text{Bedarf}\;=\;\text{Fläche}", chart["pt"](0.02, 0.05), size=LABEL_FONT_SIZE, color=ENERGY,
                                edge="left")
        self.play(FadeIn(area), FadeIn(tag_load), FadeIn(tag_demand), *_life(house, p_frac, 1.0), run_time=1.0)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 1.0 + 3.0 + 1.0,
                 during=lambda rt: _life(house, p_frac, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "sortieren"))
        sorted_curve = _partial(chart, ys_sorted, 1.0, width=3.0)
        sorted_area = _partial_area(chart, ys_sorted, 1.0, opacity=0.14)
        self.play(FadeOut(tag_load), FadeOut(tag_demand), FadeOut(knob), *_life(house, p_frac, 0.5), run_time=0.5)
        self.play(Transform(curve, sorted_curve), Transform(area, sorted_area), FadeOut(months), FadeIn(hours),
                  *_life(house, lambda: 0.5, 2.6), run_time=2.6)
        series["ys"] = ys_sorted
        x.set_value(0.0)
        knob = always_redraw(lambda: Dot(chart["pt"](x.get_value(), p_frac()), radius=0.07, color=POWER))
        self.add(knob)
        self.play(_ramp(x, 1.0, rate_func=linear), *_life(house, p_frac, 3.2), run_time=3.2)
        hold_for(self, N, "sortieren", during=lambda rt: _life(house, p_frac, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "spitze"))
        peak_line = DashedLine(chart["pt"](0, 1.0), chart["pt"](1, 1.0), color=POWER, stroke_width=2.2, stroke_opacity=0.8)
        phi_read = math_readout(lambda: rf"\Phi_{{\mathrm{{HL}}}} = {de_num(p_frac() * PEAK_KW, 1)}\,\mathrm{{kW}}",
                                np.array([2.2, 1.18, 0.0]), size=BODY_FONT_SIZE, color=POWER)
        self.play(_ramp(x, 0.0), Create(peak_line), FadeIn(phi_read), *_life(house, p_frac, 1.6, r_max=0.5), run_time=1.6)
        hold_for(self, N, "spitze", during=lambda rt: _life(house, p_frac, rt, r_max=0.5))
        self.remove(phi_read)
        phi_read = math_label(rf"\Phi_{{\mathrm{{HL}}}} = {de_num(PEAK_KW, 1)}\,\mathrm{{kW}}", np.array([2.2, 1.18, 0.0]),
                              size=BODY_FONT_SIZE, color=POWER, edge="left")
        self.add(phi_read)

        caption = swap_caption(self, caption, subtitle_text(N, "flaeche"))
        sweep = always_redraw(lambda: _partial_area(chart, ys_sorted, x.get_value()))
        q_read = math_readout(lambda: rf"Q_{{h}} = {de_num(round(_energy_at(cum, x.get_value()), -2))}\,\mathrm{{kWh}}",
                              np.array([2.2, 0.72, 0.0]), size=BODY_FONT_SIZE, color=ENERGY)
        tank["level"].add_updater(lambda m: m.set_value(_energy_at(cum, x.get_value()) / cum[-1]))
        self.add(sweep, q_read, tank["level"])
        self.play(FadeOut(area), FadeIn(q_read), *_life(house, p_frac, 0.6), run_time=0.6)
        self.play(_ramp(x, 1.0, rate_func=linear), *_life(house, p_frac, 4.6), run_time=4.6)
        q_final = math_label(rf"Q_{{h}} \approx {de_num(q_year)}\,\mathrm{{kWh/a}}", np.array([2.2, 0.72, 0.0]),
                             size=BODY_FONT_SIZE, color=ENERGY, edge="left")
        self.play(ReplacementTransform(q_read, q_final), *_life(house, p_frac, 0.8), run_time=0.8)
        hold_for(self, N, "flaeche", during=lambda rt: _life(house, p_frac, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "volllast"))
        rect = Polygon(*[chart["pt"](full * i / (PLOT_N - 1), 1.0) for i in range(PLOT_N)], chart["pt"](full, 0), chart["pt"](0, 0),
                       fill_color=P_YELLOW, fill_opacity=0.24, stroke_color=P_YELLOW, stroke_width=2.2)
        sweep.clear_updaters()
        knob.clear_updaters()
        run = ValueTracker(0.0)
        t_read = math_readout(lambda: rf"t_{{VL}} = {de_num(min(run.get_value(), full) * PROFILE_N, 0)}\,\mathrm{{h}}",
                              chart["pt"](0.12, 1.0) + UP * 0.18, size=BODY_FONT_SIZE, color=TIME)
        tank["level"].clear_updaters()
        self.add(run, tank["level"])
        self.play(ReplacementTransform(sweep, rect), FadeOut(peak_line), FadeOut(knob), _ramp(tank["level"], 0.0),
                  FadeIn(t_read), *_life(house, lambda: 1.0, 1.6, r_max=0.5), run_time=1.6)
        tank["level"].add_updater(lambda m: m.set_value(min(run.get_value(), full) / full))

        def full_load():
            return 1.0 if run.get_value() < full else 0.0

        self.play(_ramp(run, 1.0, rate_func=linear), *_life(house, full_load, 4.0, r_max=0.5), run_time=4.0)
        panel = math_panel([
            ("t", r"t_{VL}", TIME), (None, "=", P_WHITE),
            (None, r"\frac{Q_{h}}{\Phi_{\mathrm{HL}}}", P_WHITE), (None, "=", P_WHITE),
            ("f", rf"\frac{{\textcolor{{{ENERGY}}}{{{de_num(q_year)}\,\mathrm{{kWh/a}}}}}}"
                  rf"{{\textcolor{{{POWER}}}{{{de_num(PEAK_KW)}\,\mathrm{{kW}}}}}}", P_WHITE),
            (None, "=", P_WHITE), ("res", rf"{de_num(t_vl)}\,\mathrm{{h/a}}", TIME),
        ], size=FORMULA_FONT_SIZE - 5)
        _fill_panel(self, panel, {"f": [q_final, phi_read], "res": t_read, "t": t_read})
        hold_for(self, N, "volllast", during=lambda rt: _life(house, full_load, rt, r_max=0.5))

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion


#region Beat 4 – Auslegen auf die Spitze, nicht auf den Mittelwert

class Beat4_RichtigAuslegen(Scene):
    NARRATION = [
        ("intro",
         "On that same curve you can see, directly, what happens when a plant is sized wrongly.",
         "An derselben Kurve sieht man unmittelbar, was passiert, wenn eine Anlage falsch ausgelegt wird."),
        ("zuklein",
         "Size it on the average and a power deficit remains at high load — without reserve capacity the indoor temperature can no longer be held.",
         "Legt man auf den Mittelwert aus, bleibt bei hoher Last ein Leistungsdefizit — ohne Reserve lässt sich die Raumtemperatur dann nicht halten."),
        ("zugross",
         "Size it far above the peak and the plant is oversized almost all year — at too high a minimum output that means more cycling.",
         "Legt man weit über die Spitze aus, ist die Anlage fast das ganze Jahr überdimensioniert — das begünstigt häufigeres Takten."),
        ("richtig",
         "The design heat load gives the central sizing point — the actual plant also depends on the heat generator, controls and storage.",
         "Die Norm-Heizlast liefert den zentralen Auslegungspunkt — die reale Anlage hängt zusätzlich von Erzeuger, Regelung und Speicher ab."),
        ("brueck",
         "The area under the curve is the useful heating demand — the plant turns that into final energy, and only that into cost.",
         "Die Fläche darunter ist der Heizwärmebedarf — das Heizsystem macht daraus den Endenergiebedarf, und erst daraus die Kosten."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        PEAK_F = 0.68

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Auslegen auf die Spitze, nicht auf den Mittelwert", title)
        din = _din_ref("DIN EN 12831-1 · DIN/TS 12831-1 · VDI 4645")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        load = _annual_load()
        ordered = np.sort(load)[::-1]
        ys = _plot_samples(ordered) * PEAK_F
        xs = np.linspace(0.0, 1.0, PLOT_N)
        cum = _cumulative(ordered)
        q_year = round(float(cum[-1]), -2)
        mean_f = float(np.mean(ordered)) * PEAK_F
        kw_of = lambda f: f / PEAK_F * PEAK_KW
        cap = ValueTracker(1.0)
        x = ValueTracker(0.0)

        def demand():
            return float(np.interp(x.get_value(), xs, ys))

        def supply():
            return min(demand(), cap.get_value())

        def level():
            return supply() / PEAK_F

        stage = _stage()
        house = _heated_house(stage["house"], HOUSE_S)
        hs = house["hs"]
        chart = _chart(stage["chart"], CHART_W, stage["chart_h"], r"t", r"P\;[\mathrm{kW}]")
        kw_ticks = _kw_ticks(chart, PEAK_KW / PEAK_F)
        hours = _hour_labels(chart)
        curve = _partial(chart, ys, 1.0, width=3.0)
        knob = always_redraw(lambda: Dot(chart["pt"](x.get_value(), demand()), radius=0.07, color=POWER))

        bulb = np.array([hs["bottom_left"][0] + 1.0, house["floors"][0] + 0.17, 0.0])
        th = thermometer_glyph(bulb, height=0.78, color=P_RED, level=0.8)
        th["level"].add_updater(lambda m: m.set_value(0.8 - 0.4 * float(np.clip((demand() - cap.get_value()) / PEAK_F, 0.0, 1.0))))
        soll = Line(bulb + UP * (0.05 + 0.66 * 0.8) + LEFT * 0.16, bulb + UP * (0.05 + 0.66 * 0.8) + RIGHT * 0.16,
                    color=P_GREEN, stroke_width=1.8)
        soll_lbl = math_label(rf"{de_num(THETA_I)}\,\mathrm{{°C}}", bulb + UP * (0.05 + 0.66 * 0.8) + RIGHT * 0.22 + DOWN * 0.07,
                              size=LABEL_FONT_SIZE - 2, color=P_GREEN, edge="left")

        self.play(Create(hs["group"]), run_time=1.2)
        self.play(FadeIn(house["rads"]), FadeIn(house["guides"]), FadeIn(th["group"]), FadeIn(soll), FadeIn(soll_lbl),
                  Create(chart["group"]), FadeIn(kw_ticks), FadeIn(hours), run_time=1.0)
        self.add(th["column"], th["level"], knob)
        self.play(Create(curve), *_life(house, level, 1.4), run_time=1.4)
        self.play(_ramp(x, 1.0, rate_func=linear), *_life(house, level, 2.6), run_time=2.6)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 1.0 + 1.4 + 2.6,
                 during=lambda rt: _life(house, level, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "zuklein"))
        cap.set_value(mean_f)
        cap_line = always_redraw(lambda: DashedLine(chart["pt"](0.0, cap.get_value()), chart["pt"](1.0, cap.get_value()),
                                                    color=P_GREEN, stroke_width=2.6))
        cap_read = math_readout(lambda: rf"\Phi_{{\mathrm{{Anlage}}}} = {de_num(kw_of(cap.get_value()), 1)}\,\mathrm{{kW}}",
                                lambda: chart["pt"](1.0, cap.get_value()) + UP * 0.2, size=LABEL_FONT_SIZE, color=P_GREEN,
                                edge="right")
        above = [i for i, v in enumerate(ys) if v > mean_f]
        gap = Polygon(*[chart["pt"](xs[i], float(ys[i])) for i in above], *[chart["pt"](xs[i], mean_f) for i in reversed(above)],
                      fill_color=P_RED, fill_opacity=0.22, stroke_width=0)
        self.add(cap_line)
        self.play(_ramp(x, 0.0), FadeIn(cap_read), FadeIn(gap), *_life(house, level, 1.0), run_time=1.0)
        self.play(_ramp(x, 0.55, rate_func=linear), *_life(house, level, 4.0, r_max=0.5), run_time=4.0)
        hold_for(self, N, "zuklein", during=lambda rt: _life(house, level, rt, r_max=0.5))

        caption = swap_caption(self, caption, subtitle_text(N, "zugross"))
        big = PEAK_F * 1.42
        reserve = always_redraw(lambda: Polygon(
            *[chart["pt"](xs[i], float(ys[i])) for i in range(PLOT_N)],
            *[chart["pt"](xs[i], cap.get_value()) for i in reversed(range(PLOT_N))],
            fill_color=P_ORANGE, fill_opacity=0.2, stroke_width=0))
        taktet = math_label(r"\text{taktet}", np.array([-6.3, 1.25, 0.0]), size=BODY_FONT_SIZE, color=P_ORANGE)

        def cycled():
            d, c = demand(), cap.get_value()
            floor_out = 0.3 * c
            if d >= floor_out:
                return d / PEAK_F
            on = ((self.time % 1.2) / 1.2) < d / floor_out
            return floor_out / PEAK_F if on else 0.0

        self.add(reserve)
        self.play(FadeOut(gap), _ramp(cap, big), _ramp(x, 0.45), *_life(house, level, 1.8), run_time=1.8)
        self.play(FadeIn(taktet), _ramp(x, 0.9, rate_func=linear), *_life(house, cycled, 4.0), run_time=4.0)
        hold_for(self, N, "zugross", during=lambda rt: _life(house, cycled, rt))

        caption = swap_caption(self, caption, subtitle_text(N, "richtig"))
        self.play(FadeOut(reserve), FadeOut(taktet), _ramp(cap, PEAK_F), _ramp(x, 0.0), *_life(house, level, 1.8), run_time=1.8)
        reserve.clear_updaters()
        self.play(_ramp(x, 1.0, rate_func=linear), *_life(house, level, 4.0, r_max=0.5), run_time=4.0)
        hold_for(self, N, "richtig", during=lambda rt: _life(house, level, rt, r_max=0.5))

        caption = swap_caption(self, caption, subtitle_text(N, "brueck"))
        tank = energy_tank(stage["tank"], height=stage["tank_h"], width=0.7, color=ENERGY, level=0.0)
        q_tag = math_label(r"Q_{h}\;[\mathrm{kWh}]", tank["cap"].get_top() + UP * 0.28, size=LABEL_FONT_SIZE, color=ENERGY)
        sweep = always_redraw(lambda: _partial_area(chart, ys, x.get_value()))
        q_read = math_readout(lambda: rf"Q_{{h}} = {de_num(round(_energy_at(cum, x.get_value()), -2))}\,\mathrm{{kWh}}",
                              np.array([2.2, 0.72, 0.0]), size=BODY_FONT_SIZE, color=ENERGY)
        tank["level"].add_updater(lambda m: m.set_value(_energy_at(cum, x.get_value()) / cum[-1]))
        self.add(sweep, tank["level"])
        cap_line.clear_updaters()
        for m in cap_read.get_family():
            m.clear_updaters()
        self.play(FadeOut(cap_line), FadeOut(cap_read), _ramp(x, 0.0), *_life(house, level, 0.8), run_time=0.8)
        self.play(FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(q_tag), FadeIn(q_read), *_life(house, level, 0.6),
                  run_time=0.6)
        self.play(_ramp(x, 1.0, rate_func=linear), *_life(house, level, 4.0), run_time=4.0)
        q_final = math_label(rf"Q_{{h}} \approx {de_num(q_year)}\,\mathrm{{kWh/a}}", np.array([2.2, 0.72, 0.0]),
                             size=BODY_FONT_SIZE, color=ENERGY, edge="left")
        self.play(ReplacementTransform(q_read, q_final), *_life(house, level, 0.6), run_time=0.6)
        panel = math_panel([
            ("q", r"Q_{h}", ENERGY), (None, r"\to", P_WHITE), (None, r"\text{Endenergie}", P_WHITE),
            (None, r"\to", P_WHITE), (None, r"\text{Kosten}", P_WHITE),
        ])
        _fill_panel(self, panel, {"q": q_final})
        hold_for(self, N, "brueck", during=lambda rt: _life(house, level, rt))

        _clear_stage(self, (title, subtitle, din))
        self.wait(0.4)

#endregion
