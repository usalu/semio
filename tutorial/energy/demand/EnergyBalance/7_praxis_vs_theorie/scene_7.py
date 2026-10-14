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
    SAFE_TOP, SAFE_BOTTOM, SAFE_BOTTOM_FORMULA, CONTENT_TOP,
    note_line, stat_card, labeled_axes, fit_band, stacked_bar, side_labels,
    smooth_path, flow_guides, ripples, radiation_ray, color_ramp,
    house_section, radiator, person_glyph, sun_glyph, cloud_glyph, thermometer_glyph,
    lamp_glyph, clock_glyph, meter, bind_meter, energy_tank, cloud_trail, flow_animation, sun_rays, shine, pulse_flashes,
    math_label, math_panel, math_readout, de_num,
    highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Theorie vs. Praxis"

CALC_CYAN = "#38BDF8"
REAL_ORANGE = "#FB923C"
WARM_RED = "#EF4444"
COLD_BLUE = "#4D96FF"
GOOD_GREEN = "#22C55E"
WATER_HOT = "#F97316"

#region Chain constants
CHAIN_E_FACTORS = (1.04, 1.08, 1.06, 1.18)
CHAIN_E_FACTORS_WP = (1.04, 1.08, 1.06, 0.26)
CHAIN_FP_GAS, CHAIN_FP_STROM = 1.1, 1.8
CHAIN_NUTZ = 100
CHAIN_END = round(CHAIN_NUTZ * float(np.prod(CHAIN_E_FACTORS)))
CHAIN_PRIM = int(CHAIN_END * CHAIN_FP_GAS + 0.5)
CHAIN_END_WP = round(CHAIN_NUTZ * float(np.prod(CHAIN_E_FACTORS_WP)))
CHAIN_PRIM_WP = int(CHAIN_END_WP * CHAIN_FP_STROM + 0.5)
CHAIN_CLASSES = (
    ("A+", 30, "#1D7A3E"), ("A", 50, "#3FA34D"), ("B", 75, "#8DC63F"),
    ("C", 100, "#C7D93A"), ("D", 130, "#F9E547"), ("E", 160, "#F5A623"),
    ("F", 200, "#E8622C"), ("G", 250, "#D0342C"), ("H", None, "#A61C1C"),
)


def _class_of(end_energy: float):
    """🔤 Efficiency class (letter, colour) an end-energy value earns — GEG Anlage 10, as Chapter 6 looks it up."""
    for letter, bound, colour in CHAIN_CLASSES:
        if bound is None or end_energy <= bound:
            return letter, colour
    return CHAIN_CLASSES[-1][0], CHAIN_CLASSES[-1][2]


CHAIN_CLASS = _class_of(CHAIN_END)[0]
CHAIN_CLASS_WP = _class_of(CHAIN_END_WP)[0]
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

def _stand(floor_y, room_h, x, color=P_ORANGE):
    """🧍 Occupant at true scale — 70 % of the clear room height, feet on the floor line at ``x``."""
    height = 0.7 * room_h
    return person_glyph(np.array([x, floor_y + 0.02 + height / 2, 0.0]), color=color, scale=height / 0.66)


def _stream(path, rate, *, color, color_end=None, count: int = 5, speed: float = 0.45, radius: float = 0.05,
            streak: bool = False):
    """💧 Particles that keep travelling along ``path`` at a speed set by the ``rate`` tracker.

    Animating the tracker speeds the stream up, slows it down or stops it while the drops keep moving;
    ``streak`` stretches each particle along the path so it reads as moving air instead of bouncing dots.
    """
    start_c, end_c = ManimColor(color), ManimColor(color_end or color)
    drops = VGroup(*[
        Ellipse(width=radius * 3.0, height=radius * 1.2, stroke_width=0) if streak else Dot(radius=radius, stroke_width=0)
        for _ in range(count)
    ])
    state = {"phase": 0.0}

    def advance(group, dt):
        value = float(rate.get_value())
        state["phase"] = (state["phase"] + dt * speed * value) % 1.0
        show = float(np.clip(value * 5.0, 0.0, 1.0))
        for i, drop in enumerate(group):
            t = (state["phase"] + i / count) % 1.0
            drop.move_to(path.point_from_proportion(t))
            if streak:
                tangent = path.point_from_proportion(min(1.0, t + 0.02)) - path.point_from_proportion(max(0.0, t - 0.02))
                drop.set_angle(float(np.arctan2(tangent[1], tangent[0])))
            drop.set_fill(interpolate_color(start_c, end_c, t), opacity=show * min(1.0, t / 0.10, (1.0 - t) / 0.12))

    advance(drops, 0.0)
    drops.add_updater(advance)
    return drops


def _polyline(points):
    """📐 Corner path through ``points`` — the pipe a ``_stream`` follows."""
    line = VMobject()
    line.set_points_as_corners([np.array(p, dtype=float) for p in points])
    return line


def _drive_sash(window, opening):
    """🪟 Let an ``opening`` tracker (0 closed … 1 wide) swing the window sash about its hinge."""
    base = window["sash"].copy()

    def swing(mob):
        mob.become(base.copy().rotate(-float(opening.get_value()) * PI / 2, axis=UP, about_point=window["hinge"]))

    window["sash"].add_updater(swing)
    swing(window["sash"])


def _polyline_hit(origin, direction, a, b):
    """✂️ Point where the ray ``origin + t·direction`` meets the segment ``a→b``."""
    a, b = np.array(a, dtype=float), np.array(b, dtype=float)
    m = np.array([[direction[0], a[0] - b[0]], [direction[1], a[1] - b[1]]])
    t, _u = np.linalg.solve(m, np.array([a[0] - origin[0], a[1] - origin[1]]))
    return np.array(origin, dtype=float) + np.array(direction, dtype=float) * t

#endregion


#region Beat 1 – Warum Rechnung und Abrechnung auseinanderlaufen

SEG_COLOURS = (P_RED, P_CYAN, WATER_HOT, P_YELLOW)
SEG_NORM = (0.26, 0.14, 0.10, 0.12)
BAR_X0, BAR_UNIT, BAR_H = -0.60, 3.80, 0.34
BAR_Y_CALC, BAR_Y_MEAS = 0.92, 0.18


class Beat1_BerechnungVsAbrechnung(Scene):
    NARRATION = [
        ("intro",
         "Does the calculated demand match the bill? Rarely exactly — because the calculation deliberately uses standardised conditions.",
         "Stimmt der berechnete Bedarf mit der Rechnung überein? Selten genau — denn die Berechnung nutzt bewusst Normbedingungen."),
        ("normen",
         "The calculation fixes the use: a twenty-degree setpoint, a standard air change, standard hot water, a reference climate.",
         "Die Berechnung legt die Nutzung fest: zwanzig Grad als Sollwert, genormter Luftwechsel, genormtes Warmwasser, ein Referenzklima."),
        ("real",
         "Real households heat unevenly, air unevenly and live unevenly. So the measured points scatter around the diagonal.",
         "Echte Haushalte heizen, lüften und leben ungleichmäßig. Die Messpunkte streuen deshalb um die Diagonale."),
        ("prebound",
         "The scatter is not symmetric: poorly insulated buildings consume on average less than calculated — people heat more sparingly.",
         "Die Streuung ist nicht symmetrisch: Schlecht gedämmte Gebäude verbrauchen im Mittel weniger als berechnet — man heizt sparsamer."),
        ("rebound",
         "In efficient buildings it is on average above — for instance through higher comfort temperatures.",
         "In effizienten Gebäuden liegt er im Mittel darüber — etwa durch höhere Komforttemperaturen."),
        ("fazit",
         "The certificate therefore compares buildings, not households. That is exactly what it was built to do.",
         "Der Ausweis vergleicht deshalb Gebäude, nicht Haushalte. Genau dafür ist er gemacht."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Warum Rechnung und Abrechnung auseinanderlaufen", title)
        din = _din_ref("DIN V 18599-10:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        scale = 1.05
        hs = house_section(np.array([-4.5, -0.95, 0.0]), scale=scale)
        bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
        lo_win, up_win = hs["windows"]
        y_floor, y_mid = float(bl[1]), float(hs["level_1"].get_center()[1])
        room_h = hs["w_height"] / 2
        wall_in = float(br[0]) - 0.08 * scale
        person = _stand(y_floor, room_h, -4.00)
        rad = radiator(np.array([float(bl[0]) + 1.0, y_floor + 0.27, 0.0])).scale(0.85)
        th_h = 0.85
        th = thermometer_glyph(np.array([-4.30, y_mid + 0.28, 0.0]), height=th_h, color=P_RED, level=0.5)
        tick_y = y_mid + 0.28 + 0.05 + (th_h - 0.12) * 0.5
        th_tick = Line(np.array([-4.17, tick_y, 0.0]), np.array([-4.07, tick_y, 0.0]), color=CALC_CYAN, stroke_width=2.4)
        temp = th["level"]
        temp_read = math_readout(
            lambda: rf"{de_num(10 + 20 * temp.get_value())}\,\mathrm{{°C}}",
            np.array([-3.98, tick_y - 0.07, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
        spout_y, spout_x = y_mid + 0.80, -3.10
        spout = VGroup(
            Line(np.array([wall_in, spout_y, 0.0]), np.array([spout_x, spout_y, 0.0]), color=P_WHITE, stroke_width=1.8),
            Line(np.array([spout_x, spout_y, 0.0]), np.array([spout_x, spout_y - 0.08, 0.0]), color=P_WHITE, stroke_width=1.8),
        )
        basin = VGroup(
            Line(np.array([spout_x - 0.26, y_mid + 0.22, 0.0]), np.array([spout_x - 0.18, y_mid + 0.08, 0.0]), color=P_WHITE, stroke_width=1.6),
            Line(np.array([spout_x - 0.18, y_mid + 0.08, 0.0]), np.array([spout_x + 0.18, y_mid + 0.08, 0.0]), color=P_WHITE, stroke_width=1.6),
            Line(np.array([spout_x + 0.18, y_mid + 0.08, 0.0]), np.array([spout_x + 0.26, y_mid + 0.22, 0.0]), color=P_WHITE, stroke_width=1.6),
        )
        hot = ValueTracker(0.5)
        tap_stream = _stream(_polyline([(spout_x, spout_y - 0.12, 0), (spout_x, y_mid + 0.12, 0)]),
                             hot, color=WATER_HOT, count=4, speed=0.9, radius=0.04)
        opening = ValueTracker(0.3)
        air_rate = ValueTracker(0.0)
        air_streams = VGroup()
        for win in (lo_win, up_win):
            x, y = win["x"], float(win["center"][1])
            air_in = smooth_path([(x - 0.50, y - 0.10, 0), (x + 0.15, y - 0.10, 0), (x + 0.85, y - 0.06, 0), (x + 1.45, y + 0.02, 0)])
            air_out = smooth_path([(x + 1.45, y + 0.14, 0), (x + 0.85, y + 0.17, 0), (x + 0.15, y + 0.16, 0), (x - 0.50, y + 0.20, 0)])
            air_streams.add(
                _stream(air_in, air_rate, color=COLD_BLUE, color_end=P_ORANGE, count=4, speed=0.30, streak=True, radius=0.06),
                _stream(air_out, air_rate, color=P_ORANGE, count=4, speed=0.30, streak=True, radius=0.06),
            )
        for win in (lo_win, up_win):
            _drive_sash(win, opening)

        sun_c = np.array([-1.70, 1.10, 0.0])
        sun = sun_glyph(sun_c).scale(0.55)
        slope = (peak, tr + RIGHT * 0.26 * scale)
        aim = np.array([-0.80, -0.60, 0.0])
        perp = np.array([0.60, -0.80, 0.0])
        rays = VGroup(*[
            radiation_ray(sun_c + aim * 0.62 + perp * s, _polyline_hit(sun_c + aim * 0.62 + perp * s, aim, *slope))
            for s in (-0.20, 0.0, 0.20)
        ])
        cover = ValueTracker(0.0)
        cloud_core = Rectangle(width=0.75, height=0.10, stroke_width=0, fill_opacity=0).move_to(ORIGIN)
        cloud_base = cloud_glyph(cloud_core, pad=(0.42, 0.26), bump=0.40)
        cloud_base.set_fill(P_DEEP_DARK, opacity=0.95)
        cloud = cloud_base.copy()
        cloud_far, cloud_near = np.array([5.00, 1.70, 0.0]), np.array([-1.66, 1.14, 0.0])

        def _place_cloud(mob):
            k = float(cover.get_value())
            drop = k * k * (3 - 2 * k)
            where = cloud_far + (cloud_near - cloud_far) * np.array([k, drop, 0.0])
            mob.become(cloud_base.copy().move_to(where))

        cloud.add_updater(_place_cloud)
        rays.add_updater(lambda mob: mob.set_stroke(opacity=0.9 * (1.0 - 0.85 * float(cover.get_value()))))
        _place_cloud(cloud)
        warm_spots = [rad.get_top() + UP * 0.02, person.get_top() + UP * 0.04]

        def heat(rt, r_max=0.36):
            return [ripples(warm_spots, r_max=r_max, cycles=max(1.0, rt / 1.4))]

        def _bar_label(text, colour, y):
            label = body_text(text, font_size=LABEL_FONT_SIZE, color=colour)
            label.move_to(np.array([BAR_X0, y, 0.0]), aligned_edge=LEFT)
            return label

        def _bar(total, colour, y):
            return Rectangle(width=total * BAR_UNIT, height=BAR_H, color=colour, stroke_width=1.6,
                             fill_color=colour, fill_opacity=0.55).move_to(
                np.array([BAR_X0 + total * BAR_UNIT / 2, y, 0.0]))

        def _segments(lengths, y, opacity=0.78):
            cursor, out = 0.0, VGroup()
            for length, colour in zip(lengths, SEG_COLOURS):
                width = max(0.004, length * BAR_UNIT)
                out.add(Rectangle(width=width, height=BAR_H, color=colour, stroke_width=1.2,
                                  fill_color=colour, fill_opacity=opacity).move_to(
                    np.array([BAR_X0 + cursor * BAR_UNIT + width / 2, y, 0.0])))
                cursor += length
            return out

        def _measured_lengths():
            factors = ((10 + 20 * temp.get_value()) / 20.0, (0.4 + opening.get_value()) / 0.7,
                       hot.get_value() / 0.5, 1.0 + 0.3 * cover.get_value())
            return [c * f for c, f in zip(SEG_NORM, factors)]

        calc_total = sum(SEG_NORM)
        calc_lbl = _bar_label("berechnet", CALC_CYAN, BAR_Y_CALC + 0.36)
        meas_lbl = _bar_label("gemessen", REAL_ORANGE, BAR_Y_MEAS + 0.36)
        calc_bar = _bar(calc_total, CALC_CYAN, BAR_Y_CALC)
        meas_bar = _bar(0.68, REAL_ORANGE, BAR_Y_MEAS)
        calc_segs = _segments(SEG_NORM, BAR_Y_CALC)
        meas_segs = always_redraw(lambda: _segments(_measured_lengths(), BAR_Y_MEAS))

        def _legend(src, colour, at):
            row = VGroup(Dot(radius=0.07, color=colour), math_label(src, size=LABEL_FONT_SIZE, color=P_WHITE))
            return row.arrange(RIGHT, buff=0.14).move_to(at, aligned_edge=LEFT)

        legend = VGroup(
            _legend(r"\text{Innentemperatur}\;20\,\mathrm{°C}\;\text{Sollwert}", SEG_COLOURS[0], np.array([-2.15, -0.45, 0.0])),
            _legend(r"\text{Luftwechsel genormt}", SEG_COLOURS[1], np.array([2.15, -0.45, 0.0])),
            _legend(r"\text{Warmwasser pauschal je}\;\mathrm{m^{2}}", SEG_COLOURS[2], np.array([-2.15, -0.90, 0.0])),
            _legend(r"\text{Referenzklima}", SEG_COLOURS[3], np.array([2.15, -0.90, 0.0])),
        )

        self.play(Create(hs["group"]), run_time=1.5)
        self.play(FadeIn(rad), FadeIn(person), *heat(1.0), run_time=1.0)
        self.play(FadeIn(calc_lbl), FadeIn(meas_lbl), GrowFromEdge(calc_bar, LEFT), GrowFromEdge(meas_bar, LEFT),
                  *heat(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 1.0 + 1.2,
                 during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "normen"))
        self.add(th["column"], temp_read)
        self.play(FadeOut(calc_bar), FadeIn(th["group"]), FadeIn(th_tick), FadeIn(legend[0], shift=UP * 0.1),
                  GrowFromEdge(calc_segs[0], LEFT), *heat(1.4), run_time=1.4)
        self.add(air_streams)
        self.play(air_rate.animate.set_value(0.5), FadeIn(legend[1], shift=UP * 0.1),
                  GrowFromEdge(calc_segs[1], LEFT), *heat(1.4), run_time=1.4)
        self.add(tap_stream)
        self.play(FadeIn(spout), FadeIn(basin), FadeIn(legend[2], shift=UP * 0.1), GrowFromEdge(calc_segs[2], LEFT),
                  *heat(1.4), run_time=1.4)
        self.add(rays)
        self.play(FadeIn(sun), Create(rays), FadeIn(legend[3], shift=UP * 0.1), GrowFromEdge(calc_segs[3], LEFT),
                  *heat(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "normen", used=1.4 * 4 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "real"))
        self.add(meas_segs, cloud)
        self.play(FadeOut(meas_bar), *heat(0.6), run_time=0.6)
        self.play(temp.animate.set_value(0.6), *heat(1.0), run_time=1.0)
        self.play(opening.animate.set_value(0.55), air_rate.animate.set_value(0.9), *heat(1.0), run_time=1.0)
        self.play(hot.animate.set_value(0.35), person.animate.shift(RIGHT * 0.5), *heat(1.0), run_time=1.0)
        self.play(cover.animate.set_value(1.0), *heat(1.4), run_time=1.4)

        axes = labeled_axes([BAR_X0, -1.42, 0], x_len=BAR_UNIT, y_len=2.95, x_label="berechnet  →", color=P_WHITE)
        y_cap = body_text("gemessen", font_size=LABEL_FONT_SIZE - 2, color=P_WHITE)
        y_cap.set_opacity(0.85).rotate(PI / 2).next_to(axes["y_axis"], LEFT, buff=0.14)
        diagonal = DashedLine(axes["pt"](0.0, 0.0), axes["pt"](1.0, 1.0), color=P_WHITE, stroke_width=2, stroke_opacity=0.55)
        diag_tag = body_text("Rechnung = Messung", font_size=LABEL_FONT_SIZE - 4, color=P_WHITE)
        diag_tag.set_opacity(0.6).next_to(axes["pt"](0.88, 0.88), UL, buff=0.06)
        schematic_tag = body_text("schematische Darstellung", font_size=LABEL_FONT_SIZE - 4, color=P_TEAL)
        schematic_tag.set_opacity(0.8).next_to(axes["x_label"], DOWN, buff=0.12)
        meas_total = sum(_measured_lengths())
        x_seg = Line(axes["pt"](0, 0), axes["pt"](calc_total, 0), color=CALC_CYAN, stroke_width=8)
        y_seg = Line(axes["pt"](0, 0), axes["pt"](0, meas_total), color=REAL_ORANGE, stroke_width=8)
        me_dot = Dot(axes["pt"](calc_total, meas_total), radius=0.09, color=REAL_ORANGE)
        proj = VGroup(
            DashedLine(axes["pt"](calc_total, 0), me_dot.get_center(), color=CALC_CYAN, stroke_width=1.8),
            DashedLine(axes["pt"](0, meas_total), me_dot.get_center(), color=REAL_ORANGE, stroke_width=1.8),
        )
        meas_frozen = _segments(_measured_lengths(), BAR_Y_MEAS)
        self.remove(meas_segs)
        self.add(meas_frozen)
        self.play(
            FadeOut(legend), FadeOut(calc_lbl), FadeOut(meas_lbl),
            Create(axes["group"]), FadeIn(y_cap),
            ReplacementTransform(calc_segs, x_seg),
            ReplacementTransform(meas_frozen, y_seg),
            run_time=1.6,
        )
        self.play(Create(proj), FadeIn(me_dot, scale=0.4), Create(diagonal), FadeIn(diag_tag), FadeIn(schematic_tag),
                  run_time=1.0)
        rng = np.random.default_rng(11)
        xs = np.linspace(0.12, 0.94, 22)
        ys = 0.30 + 0.52 * xs + rng.normal(0.0, 0.055, len(xs))
        points = VGroup(*[
            Dot(axes["pt"](float(x), float(np.clip(y, 0.05, 0.97))), radius=0.058, color=REAL_ORANGE, fill_opacity=0.9)
            for x, y in zip(xs, ys)
        ])
        self.play(LaggedStart(*[FadeIn(d, scale=0.5) for d in points], lag_ratio=0.05), *heat(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "real", during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "prebound"))
        trend = Line(axes["pt"](0.10, 0.352), axes["pt"](0.96, 0.799), color=REAL_ORANGE, stroke_width=3)
        pre_card = stat_card("schlecht gedämmt · im Mittel", "gemessen < berechnet", color=GOOD_GREEN,
                             caption_size=LABEL_FONT_SIZE - 2, value_size=LABEL_FONT_SIZE)
        pre_card.move_to(np.array([5.45, 0.55, 0.0]))
        leak_l = np.array([[float(bl[0]), y, 0.0] for y in (float(lo_win["center"][1]) - 0.45, float(up_win["center"][1]) + 0.45)])
        leak_r = np.array([[float(br[0]), y, 0.0] for y in (y_floor + 0.5, y_mid + 0.6)])

        def leak(rt, r_max=0.55):
            return [ripples(list(leak_l), r_max=r_max, color=P_RED, facing=PI, cycles=max(1.0, rt / 1.4)),
                    ripples(list(leak_r), r_max=r_max, color=P_RED, facing=0.0, cycles=max(1.0, rt / 1.4))]

        pre_dot = axes["pt"](0.85, 0.70)
        self.play(
            FadeOut(x_seg), FadeOut(y_seg), FadeOut(proj), Create(trend), FadeIn(pre_card),
            temp.animate.set_value(0.4), me_dot.animate.move_to(pre_dot), person.animate.shift(LEFT * 0.5),
            cover.animate.set_value(0.0), *leak(1.6),
            run_time=1.6,
        )
        ring = Circle(radius=0.17, color=P_WHITE, stroke_width=2.5).move_to(me_dot.get_center())
        self.play(Create(ring), *leak(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "prebound", during=lambda rt: leak(rt) + heat(rt, 0.28))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rebound"))
        off = 0.09
        envelope = VMobject(color=P_TEAL, stroke_width=6, stroke_opacity=0.85).set_points_as_corners([
            bl + LEFT * off, tl + LEFT * off, tl + LEFT * (0.26 * scale + off) + UP * 0.02, peak + UP * off,
            tr + RIGHT * (0.26 * scale + off) + UP * 0.02, tr + RIGHT * off, br + RIGHT * off,
        ])
        re_card = stat_card("effizient · im Mittel", "gemessen > berechnet", color=WARM_RED,
                            caption_size=LABEL_FONT_SIZE - 2, value_size=LABEL_FONT_SIZE)
        re_card.move_to(np.array([5.45, -0.45, 0.0]))
        re_dot = axes["pt"](0.20, 0.47)
        self.play(
            Create(envelope), FadeIn(re_card),
            temp.animate.set_value(0.65), me_dot.animate.move_to(re_dot), ring.animate.move_to(re_dot),
            *leak(1.6, 0.25),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "rebound", during=lambda rt: heat(rt, 0.36))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fazit"))
        verdict = note_line("genormte Randbedingungen machen Gebäude vergleichbar — nicht Haushalte", color=P_TEAL)
        self.play(
            FadeIn(verdict), FadeOut(trend), FadeOut(pre_card), FadeOut(re_card), FadeOut(ring),
            temp.animate.set_value(0.5), opening.animate.set_value(0.3), air_rate.animate.set_value(0.5),
            hot.animate.set_value(0.5), cover.animate.set_value(0.0), person.animate.shift(RIGHT * 0.0),
            *[d.animate.move_to(axes["pt"](float((d.get_center()[0] - BAR_X0) / BAR_UNIT),
                                           float((d.get_center()[0] - BAR_X0) / BAR_UNIT))) for d in (*points, me_dot)],
            *heat(1.6),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "fazit", during=heat)

        for mob in (*air_streams, tap_stream, rays, cloud, temp_read, th["column"]):
            mob.clear_updaters()
        self.play(FadeOut(caption), FadeOut(verdict), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 – Hydraulischer Abgleich

PUMP_SPIN = 4.0
UNIT_SPEED = 0.36
FLOW_UNBALANCED = (2.0, 1.36, 0.73, 0.36)
FLOW_BALANCED = (1.0, 1.0, 1.0, 1.0)


def _pipe_stream(points, rate, *, color, color_end=None):
    """🚰 Water drops along a corner pipe — speed in units per second follows the volume flow ``rate`` (1 = design flow)."""
    pts = [np.array(p, dtype=float) for p in points]
    length = float(sum(np.linalg.norm(b - a) for a, b in zip(pts, pts[1:])))
    return _stream(_polyline(pts), rate, color=color, color_end=color_end, count=max(2, round(length / 0.30)),
                   speed=UNIT_SPEED / length, radius=0.045)


def _room_row(x_left, width, count, cy, height, t=0.14):
    """🏠 Row of rooms in section — shared ceiling and floor slabs, partition walls between the rooms."""
    style = dict(color=P_WHITE, stroke_width=2, fill_color=P_WHITE, fill_opacity=0.14)
    total = width * count
    ceiling = Rectangle(width=total + t, height=t, **style).move_to(np.array([x_left + total / 2, cy + height / 2, 0.0]))
    floor = Rectangle(width=total + t, height=t, **style).move_to(np.array([x_left + total / 2, cy - height / 2, 0.0]))
    walls = VGroup(*[Rectangle(width=t, height=height - t, **style).move_to(np.array([x_left + k * width, cy, 0.0]))
                     for k in range(count + 1)])
    return {"shell": VGroup(ceiling, walls, floor), "floor_top": cy - height / 2 + t / 2,
            "ceil_bottom": cy + height / 2 - t / 2, "x_left": x_left, "width": width, "t": t}


def _valve(center, opening):
    """🎚️ Radiator valve on the riser — the handle turns with the ``opening`` tracker (1 wide open … 0 shut)."""
    c = np.array(center, dtype=float)
    body = VGroup(Circle(radius=0.10, color=P_TEAL, stroke_width=2.2).set_fill(P_DEEP_DARK, opacity=1.0).move_to(c),
                  Line(c + LEFT * 0.10, c + RIGHT * 0.10, color=P_TEAL, stroke_width=2.2))
    handle = always_redraw(lambda: VGroup(
        Line(c, c + 0.26 * np.array([np.sin((1 - opening.get_value()) * PI / 2), np.cos((1 - opening.get_value()) * PI / 2), 0.0]),
             color=P_TEAL, stroke_width=3),
    ))
    return body, handle


def _thermo(bottom, height, level):
    """🌡️ Room thermometer whose column colour runs cold → comfortable → hot with the ``level`` tracker."""
    b = np.array(bottom, dtype=float)
    ramp = color_ramp([(0.0, COLD_BLUE), (0.5, GOOD_GREEN), (1.0, WARM_RED)])
    tube = RoundedRectangle(width=0.2, height=height, corner_radius=0.1, color=P_WHITE, stroke_width=2).move_to(b + UP * height / 2)
    bulb = Circle(radius=0.15, color=P_WHITE, stroke_width=2).set_fill(P_DEEP_DARK, opacity=1.0).move_to(b)
    tick_y = b[1] + 0.05 + (height - 0.12) * 0.5
    tick = Line(np.array([b[0] + 0.12, tick_y, 0.0]), np.array([b[0] + 0.22, tick_y, 0.0]), color=CALC_CYAN, stroke_width=2.4)

    def column():
        v = float(np.clip(level.get_value(), 0.0, 1.0))
        h = max(0.02, (height - 0.12) * v)
        col = Rectangle(width=0.1, height=h, stroke_width=0, fill_color=ramp(v), fill_opacity=0.95).move_to(b + UP * (h / 2 + 0.05))
        dot = Circle(radius=0.115, stroke_width=0, fill_color=ramp(v), fill_opacity=0.95).move_to(b)
        return VGroup(dot, col)

    return {"frame": VGroup(tube, bulb), "tick": tick, "column": always_redraw(column), "top": b + UP * height}


class Beat2_HydraulischerAbgleich(Scene):
    NARRATION = [
        ("intro",
         "One more gap between paper and practice, and this one is pure hydraulics.",
         "Noch eine Lücke zwischen Papier und Praxis — und diese ist reine Hydraulik."),
        ("unabgeglichen",
         "Water takes the path of least resistance. The rooms nearest the pump get far more flow than they were calculated for.",
         "Wasser nimmt den Weg des geringsten Widerstands. Die Räume nahe der Pumpe bekommen weit mehr Durchfluss als berechnet."),
        ("folge",
         "So the near rooms overheat while the far ones stay cold — and often the flow temperature simply gets turned up.",
         "Die nahen Räume überhitzen, die fernen bleiben kalt — und oft wird einfach die Vorlauftemperatur hochgedreht."),
        ("abgleich",
         "Hydraulic balancing presets every valve to the flow that room's calculated heat load actually needs.",
         "Der hydraulische Abgleich stellt jedes Ventil auf den Durchfluss ein, den die berechnete Raumheizlast wirklich braucht."),
        ("gewinn",
         "The flow temperature can then come down — an important condition for a heat pump to reach a good seasonal performance factor.",
         "Danach kann die Vorlauftemperatur sinken — eine wichtige Voraussetzung für eine gute Jahresarbeitszahl der Wärmepumpe."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Hydraulischer Abgleich", title)
        din = _din_ref("GEG § 60c · VOB/C DIN 18380")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        width, count, t = 2.3, 4, 0.14
        row = _room_row(-4.0, width, count, 0.85, 1.7, t)
        floor_top, ceil_bottom = row["floor_top"], row["ceil_bottom"]
        clear_h = ceil_bottom - floor_top
        xs = [-4.0 + width * (i + 0.5) for i in range(count)]
        y_sup, y_ret = -0.55, -0.95
        pump_c = np.array([-5.5, y_sup, 0.0])

        washes = VGroup(*[
            Rectangle(width=width - t, height=clear_h, stroke_width=0, fill_color=GOOD_GREEN, fill_opacity=0.0).move_to(
                np.array([x, (floor_top + ceil_bottom) / 2, 0.0]))
            for x in xs
        ])
        rads = VGroup(*[radiator(np.array([x - 0.62, floor_top + 0.27, 0.0])).scale(0.85) for x in xs])
        rad_x = [float(r.get_center()[0]) for r in rads]
        rad_bottom = float(rads[0].get_bottom()[1])
        rad_mid = float(rads[0].get_center()[1])
        people = VGroup(*[_stand(floor_top - 0.02, clear_h, x + 0.30) for x in xs])
        levels = [ValueTracker(0.5) for _ in xs]
        thermos = [_thermo(np.array([x + 0.84, floor_top + 0.30, 0.0]), 1.0, lv) for x, lv in zip(xs, levels)]

        x_in = [rx - 0.26 for rx in rad_x]
        x_out = [rx + 0.26 for rx in rad_x]
        supply = VGroup(
            Line(pump_c + RIGHT * 0.30, np.array([x_in[-1], y_sup, 0.0]), color=WATER_HOT, stroke_width=3.2, stroke_opacity=0.55),
            *[Line(np.array([xi, y_sup, 0.0]), np.array([xi, rad_bottom, 0.0]), color=WATER_HOT, stroke_width=3.2, stroke_opacity=0.55)
              for xi in x_in],
        )
        back = VGroup(
            Line(np.array([x_out[-1], y_ret, 0.0]), np.array([pump_c[0], y_ret, 0.0]), color=COLD_BLUE, stroke_width=3.2, stroke_opacity=0.55),
            Line(np.array([pump_c[0], y_ret, 0.0]), pump_c + DOWN * 0.30, color=COLD_BLUE, stroke_width=3.2, stroke_opacity=0.55),
            *[Line(np.array([xo, rad_bottom, 0.0]), np.array([xo, y_ret, 0.0]), color=COLD_BLUE, stroke_width=3.2, stroke_opacity=0.55)
              for xo in x_out],
        )
        pump = VGroup(Circle(radius=0.30, color=P_ORANGE, stroke_width=2.2).set_fill(P_ORANGE, opacity=0.35).move_to(pump_c))
        impeller = VGroup(Line(LEFT * 0.20, RIGHT * 0.20), Line(DOWN * 0.20, UP * 0.20)).set_stroke(P_WHITE, width=2.6).move_to(pump_c)
        impeller.add_updater(lambda m, dt: m.rotate(-dt * PUMP_SPIN, about_point=pump_c))
        pump_tag = body_text("Pumpe", font_size=LABEL_FONT_SIZE - 2, color=P_ORANGE).move_to(pump_c + UP * 0.58)

        vl = ValueTracker(0.55)
        vl_th = _thermo(np.array([-6.45, -0.92, 0.0]), 1.0, vl)
        vl_tag = math_label(r"\vartheta_{\mathrm{VL}}", at=np.array([-6.45, 0.28, 0.0]), size=BODY_FONT_SIZE, color=WATER_HOT)

        flows = [ValueTracker(0.0) for _ in xs]
        sections = [ValueTracker(0.0) for _ in xs]
        for k, sec in enumerate(sections):
            sec.add_updater(lambda m, k=k: m.set_value(sum(f.get_value() for f in flows[k:])))
        opens = [ValueTracker(1.0) for _ in xs]
        valves = [_valve(np.array([xi, -0.28, 0.0]), op) for xi, op in zip(x_in, opens)]

        streams = VGroup()
        sup_starts = [pump_c + RIGHT * 0.30] + [np.array([xi, y_sup, 0.0]) for xi in x_in[:-1]]
        for k in range(count):
            a = sup_starts[k]
            b = np.array([x_in[k], y_sup, 0.0])
            if np.linalg.norm(b - a) > 0.05:
                streams.add(_pipe_stream([a, b], sections[k], color=WATER_HOT))
        for k in range(count):
            streams.add(_pipe_stream(
                [(x_in[k], y_sup, 0), (x_in[k], rad_mid, 0), (x_out[k], rad_mid, 0), (x_out[k], y_ret, 0)],
                flows[k], color=WATER_HOT, color_end=COLD_BLUE))
        ret_ends = [np.array([pump_c[0], y_ret, 0.0])] + [np.array([xo, y_ret, 0.0]) for xo in x_out[:-1]]
        for k in range(count):
            streams.add(_pipe_stream([np.array([x_out[k], y_ret, 0.0]), ret_ends[k]], sections[k], color=COLD_BLUE))

        gauge_y = -1.38
        gauges, readouts, halves = [], [], []
        for k, x in enumerate(xs):
            g = meter("x", length=1.3, thickness=0.20, color=GOOD_GREEN, vertical=False)
            g["group"].remove(g["label"])
            VGroup(g["track"], g["fill"]).move_to(np.array([x - 0.30, gauge_y, 0.0]))
            g["fill"].move_to(g["track"].get_center())
            half = ValueTracker(0.0)
            half.add_updater(lambda m, f=flows[k]: m.set_value(f.get_value() / 2))
            bind_meter(g, half)
            mid_tick = Line(g["track"].get_center() + UP * 0.10, g["track"].get_center() + UP * 0.20, color=CALC_CYAN, stroke_width=2.4)
            read = math_readout(lambda f=flows[k]: de_num(f.get_value(), 1), np.array([x + 0.50, gauge_y - 0.07, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
            gauges.append(VGroup(g["track"], g["fill"], mid_tick))
            readouts.append(read)
            halves.append(half)
        gauge_title = math_label(r"\dot{V}/\dot{V}_{\mathrm{soll}}", at=np.array([-5.30, gauge_y - 0.09, 0.0]),
                                 size=LABEL_FONT_SIZE, color=CALC_CYAN)

        def heat(rt):
            return [ripples([rads[k].get_top() + UP * 0.02], r_max=0.14 + 0.36 * float(np.clip(levels[k].get_value(), 0.0, 1.0)),
                            color=P_RED, cycles=max(1.0, rt / 1.4)) for k in range(count)]

        self.play(Create(row["shell"]), FadeIn(washes), run_time=1.0)
        self.play(FadeIn(rads), FadeIn(people), *[FadeIn(th["frame"]) for th in thermos],
                  *[FadeIn(th["tick"]) for th in thermos], run_time=0.8)
        self.add(*[th["column"] for th in thermos], vl_th["column"], *flows, *sections, *halves, *opens)
        self.play(Create(supply), Create(back), FadeIn(pump), FadeIn(impeller), FadeIn(pump_tag),
                  FadeIn(vl_th["frame"]), FadeIn(vl_th["tick"]), FadeIn(vl_tag), run_time=1.2)
        self.play(*[FadeIn(v[0]) for v in valves], *[FadeIn(v[1]) for v in valves], run_time=0.5)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 0.8 + 1.2 + 0.5)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unabgeglichen"))
        self.add(streams, *readouts, *[g for g in gauges], gauge_title)
        self.play(FadeIn(gauge_title), *[FadeIn(g) for g in gauges], run_time=0.6)
        self.play(*[fl.animate.set_value(f) for fl, f in zip(flows, FLOW_UNBALANCED)], run_time=1.8)
        hold_for(self, self.NARRATION, "unabgeglichen", used=0.6 + 1.8 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "folge"))
        verdicts = VGroup(*[
            body_text(text, font_size=LABEL_FONT_SIZE, color=colour).move_to(np.array([x, ceil_bottom + 0.36, 0.0]))
            for x, text, colour in zip(xs, ("zu heiß", "zu heiß", "zu kalt", "zu kalt"), (WARM_RED, WARM_RED, COLD_BLUE, COLD_BLUE))
        ])
        tints = [(WARM_RED, 0.16), (WARM_RED, 0.10), (COLD_BLUE, 0.10), (COLD_BLUE, 0.16)]
        self.play(
            *[lv.animate.set_value(v) for lv, v in zip(levels, (0.78, 0.66, 0.36, 0.28))],
            *[w.animate.set_fill(c, opacity=o) for w, (c, o) in zip(washes, tints)],
            LaggedStart(*[FadeIn(v) for v in verdicts], lag_ratio=0.2), *heat(2.0), run_time=2.0)
        reaction = note_line("Reaktion im Alltag: Vorlauftemperatur hochdrehen", color=WARM_RED)
        self.play(FadeIn(reaction), vl.animate.set_value(0.85),
                  *[lv.animate.set_value(v) for lv, v in zip(levels, (0.90, 0.76, 0.46, 0.36))], *heat(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "folge", used=2.0 + 1.6 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "abgleich"))
        self.play(FadeOut(reaction), FadeOut(verdicts), run_time=0.35)
        preset = note_line("Voreinstellung je Ventil = berechneter Volumenstrom des Raums", color=GOOD_GREEN)
        self.play(
            *[op.animate.set_value(v) for op, v in zip(opens, (0.30, 0.45, 0.80, 1.0))],
            *[fl.animate.set_value(f) for fl, f in zip(flows, FLOW_BALANCED)],
            *[lv.animate.set_value(0.5) for lv in levels],
            *[w.animate.set_fill(GOOD_GREEN, opacity=0.07) for w in washes],
            FadeIn(preset), *heat(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "abgleich", used=0.35 + 2.4, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gewinn"))
        jaz = ValueTracker(0.25)
        jaz_meter = meter("JAZ", length=1.8, thickness=0.40, color=GOOD_GREEN)
        jaz_meter["group"].move_to(np.array([6.25, 0.10, 0.0]))
        bind_meter(jaz_meter, jaz)
        panel, box, items = math_panel([
            ("ab", r"\text{Abgleich}", GOOD_GREEN), (None, r"\to", P_WHITE),
            ("vl", r"\vartheta_{\mathrm{VL}}\;\text{sinkt}", WATER_HOT), (None, r"\to", P_WHITE),
            ("jaz", r"\mathrm{JAZ}\uparrow", GOOD_GREEN),
        ])
        duty = note_line("für viele Heizungsanlagen verpflichtend — GEG § 60c", color=P_TEAL)
        self.play(FadeOut(preset), FadeIn(duty), Create(box), FadeIn(panel), FadeIn(jaz_meter["group"]), run_time=1.0)
        ring_vl = highlight_param(items, "vl", color=WATER_HOT)
        self.play(Create(ring_vl), vl.animate.set_value(0.40), *heat(1.6), run_time=1.6)
        ring_jaz = highlight_param(items, "jaz", color=GOOD_GREEN)
        self.play(Create(ring_jaz), jaz.animate.set_value(0.85), *heat(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "gewinn", used=1.0 + 1.6 + 1.6 + 0.35, during=heat)

        for mob in (*streams, *sections, *halves, *readouts, *[th["column"] for th in thermos], vl_th["column"]):
            mob.clear_updaters()
        self.play(FadeOut(caption), FadeOut(duty), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 3 – Die Serie im Rückblick

COL_NUTZ, COL_END, COL_PRIM = "#38BDF8", "#DC2626", "#A78BFA"
LOSS_COLOURS = ("#FBBF24", "#F97316", "#EF4444", "#B91C1C")
LOSS_VALUES = tuple(int(v) for v in np.diff(np.concatenate([[CHAIN_NUTZ], np.round(CHAIN_NUTZ * np.cumprod(CHAIN_E_FACTORS))])))
STAGE_Y, STAGE_UNIT = -0.55, 0.0115
X_A, X_B, X_C = -3.05, -0.75, 1.95
X_B_WP, X_C_WP = -0.22, 2.45
CUT_X = (-3.55, -1.45, 1.30)
METER_X, TANK_X = -1.90, 0.55
CLASS_X, CHIP_X = 6.25, 5.10
PIPE_Y = STAGE_Y - 0.16


def _stack(x, colours, *, width=0.5):
    """📊 Vertical stack of segments whose heights follow value trackers — a quantity as a drawn length."""
    heights = [ValueTracker(0.0) for _ in colours]

    def build():
        out, cursor = VGroup(), STAGE_Y
        for h, colour in zip(heights, colours):
            v = h.get_value() * STAGE_UNIT
            if v > 0.004:
                out.add(Rectangle(width=width, height=v, color=colour, stroke_width=1.2, fill_color=colour,
                                  fill_opacity=0.8).move_to(np.array([x, cursor + v / 2, 0.0])))
            cursor += v
        return out

    return {"mob": always_redraw(build), "heights": heights, "x": x,
            "total": lambda: float(sum(h.get_value() for h in heights)),
            "top": lambda: STAGE_Y + sum(h.get_value() for h in heights) * STAGE_UNIT}


def _class_scale(x, top, bottom):
    """🌈 Vertical A+ … H certificate scale — one band per class, A+ on top."""
    band_h = (top - bottom) / len(CHAIN_CLASSES)
    bands, centres = VGroup(), {}
    for i, (letter, _bound, colour) in enumerate(CHAIN_CLASSES):
        rect = Rectangle(width=0.62, height=band_h, color=colour, fill_color=colour, fill_opacity=0.9,
                         stroke_width=1.0).move_to(np.array([x, top - band_h * (i + 0.5), 0.0]))
        text = body_text(letter, font_size=LABEL_FONT_SIZE - 5, color=P_DEEP_DARK).move_to(rect.get_center())
        bands.add(VGroup(rect, text))
        centres[letter] = rect.get_center()
    return {"bands": bands, "centres": centres}


def _class_chip(letter, colour, at):
    """🔤 Efficiency-class letter on a coloured tile."""
    tile = RoundedRectangle(width=0.54, height=0.54, corner_radius=0.08, color=colour, stroke_width=1.6,
                            fill_color=colour, fill_opacity=0.95)
    mark = body_text(letter, font_size=BODY_FONT_SIZE + 6, color=P_DEEP_DARK).move_to(tile.get_center())
    return VGroup(tile, mark).move_to(at)


def _gas_meter(center, rate):
    """⏱️ Gas meter box on the house line — the needle turns with the gas ``rate``."""
    c = np.array(center, dtype=float)
    box = RoundedRectangle(width=0.50, height=0.66, corner_radius=0.06, color=P_WHITE, stroke_width=2.0).move_to(c)
    dial_c = c + UP * 0.12
    dial = Circle(radius=0.16, color=P_WHITE, stroke_width=1.6).move_to(dial_c)
    needle = Line(dial_c, dial_c + UP * 0.13, color=COL_END, stroke_width=2.6)
    needle.add_updater(lambda m, dt: m.rotate(-dt * 3.0 * float(rate.get_value()), about_point=dial_c))
    counter = VGroup(*[Rectangle(width=0.08, height=0.12, color=P_WHITE, stroke_width=1.2).move_to(c + DOWN * 0.20 + RIGHT * dx)
                       for dx in (-0.12, -0.04, 0.04, 0.12)])
    return VGroup(box, dial, counter, needle)


def _boiler(center):
    """🔥 Wall boiler with a flame — where the gas becomes heat in the house."""
    c = np.array(center, dtype=float)
    casing = Rectangle(width=0.42, height=0.64, color=P_WHITE, stroke_width=2.0).move_to(c)
    flame = Polygon(c + DOWN * 0.18 + LEFT * 0.09, c + DOWN * 0.18 + RIGHT * 0.09, c + UP * 0.02, color=P_ORANGE,
                    stroke_width=1.4, fill_color=P_ORANGE, fill_opacity=0.8)
    core = Polygon(c + DOWN * 0.18 + LEFT * 0.04, c + DOWN * 0.18 + RIGHT * 0.04, c + DOWN * 0.06, color=P_YELLOW,
                   stroke_width=1.0, fill_color=P_YELLOW, fill_opacity=0.9)
    return VGroup(casing, flame, core)


def _tile(cx, title, detail, colour):
    """🗂️ One video of the series — framed slot with its title and what it contributed."""
    frame = RoundedRectangle(width=2.95, height=2.70, corner_radius=0.12, color=colour, stroke_width=2.0).move_to(
        np.array([cx, 0.45, 0.0]))
    name = body_text(title, font_size=BODY_FONT_SIZE, color=colour).move_to(np.array([cx, -0.28, 0.0]))
    detail.move_to(np.array([cx, -0.68, 0.0]))
    return VGroup(frame, name, detail)


class Beat3_SerienAbschluss(Scene):
    NARRATION = [
        ("intro",
         "Four videos, one continuous calculation — worth seeing end to end one last time.",
         "Vier Videos, eine durchgehende Rechnung — es lohnt sich, sie einmal am Stück zu sehen."),
        ("kette",
         "The fundamentals gave us the units. Heating and cooling gave us the flows across the envelope.",
         "Die Grundlagen gaben uns die Einheiten. Heizen und Kühlen gaben uns die Ströme über die Hülle."),
        ("bilanz",
         "This video closed the balance, walked it out to the meter, out to the source, and onto the certificate.",
         "Dieses Video schloss die Bilanz, führte sie zum Zähler, zur Quelle und auf den Ausweis."),
        ("zahlen",
         "A hundred kilowatt-hours of useful heat became, on the gas path, a hundred and forty at the meter, a hundred and fifty-four at the source — and one letter. The heat pump writes a different letter.",
         "Aus hundert Kilowattstunden Nutzwärme wurden auf dem Gasweg hundertvierzig am Zähler, hundertvierundfünfzig an der Quelle — und ein Buchstabe. Die Wärmepumpe schreibt einen anderen."),
        ("outro",
         "That is the complete path from a cold wall in January to the number a landlord shows a new tenant.",
         "Das ist der vollständige Weg von einer kalten Wand im Januar bis zur Zahl, die ein Vermieter einem neuen Mieter zeigt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Serie im Rückblick", title)
        din = _din_ref("GEG · DIN V 18599:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        xs = (-4.95, -1.65, 1.65, 4.95)
        small = 0.42
        anchor = np.array([xs[0] - 0.55, 1.62, 0.0])
        lamp = lamp_glyph(anchor, drop=0.25)
        lamp["group"].scale(1.5, about_point=anchor)
        gauge = thermometer_glyph(np.array([xs[0] + 0.62, 0.22, 0.0]), height=1.15, color=P_RED, level=0.0)
        glow = [ripples([lamp["bulb"].get_center()], r_max=0.55, color=P_YELLOW, down=True, cycles=2.0)]
        hs_heat = house_section(np.array([xs[1], 0.78, 0.0]), scale=small)
        hs_cool = house_section(np.array([xs[2] + 0.30, 0.78, 0.0]), scale=small)
        hs_small = house_section(np.array([xs[3], 0.78, 0.0]), scale=small)
        sun_small_c = np.array([xs[2] - 1.05, 1.45, 0.0])
        sun_small = sun_glyph(sun_small_c).scale(0.34)
        cool_win = hs_cool["windows"][1]
        sun_beams = sun_rays(sun_small_c, cool_win["x"], [cool_win["sill"] + 0.04, cool_win["center"][1], cool_win["lintel"] - 0.04],
                             hs_cool["bottom_left"][1] + 0.02, gap=0.36)
        beam_paths = [[s_, h_, l_] for s_, h_, l_ in zip(sun_beams["starts"], sun_beams["hits"], sun_beams["lands"])]
        heat_paths = [smooth_path([(xs[1] + 0.30, y, 0), (xs[1] + 0.80, y + 0.02, 0), (xs[1] + 1.30, y + 0.04, 0)])
                      for y in (0.50, 0.78, 1.06)]
        tiles = VGroup(
            _tile(xs[0], "1 · Grundlagen", math_label(r"\mathrm{W},\;\mathrm{J},\;\Delta T", size=LABEL_FONT_SIZE, color=P_WHITE), P_YELLOW),
            _tile(xs[1], "2 · Heizen", body_text("Verluste & Gewinne", font_size=LABEL_FONT_SIZE, color=P_WHITE), P_ORANGE),
            _tile(xs[2], "3 · Kühlen", body_text("Sommerlasten", font_size=LABEL_FONT_SIZE, color=P_WHITE), P_CYAN),
            _tile(xs[3], "4 · Energiebilanz", body_text("Gesetz & Ausweis", font_size=LABEL_FONT_SIZE, color=P_WHITE), P_TEAL),
        )
        glyphs = VGroup(lamp["group"], gauge["group"], hs_heat["group"], hs_cool["group"], sun_small, hs_small["group"])
        line_y = -1.18
        thread = smooth_path([(-6.0, line_y, 0), (0.0, line_y, 0), (6.0, line_y, 0)])
        thread_guide = flow_guides(VGroup(thread), P_WHITE, opacity=0.35)
        pace = ValueTracker(1.0)
        thread_flow = _stream(thread, pace, color=P_WHITE, count=12, speed=0.10, radius=0.05)

        self.play(LaggedStart(*[FadeIn(tile, shift=DOWN * 0.1) for tile in tiles], lag_ratio=0.18),
                  LaggedStart(*[Create(g) for g in glyphs], lag_ratio=0.18), run_time=1.9)
        self.add(gauge["column"])
        self.play(Create(thread_guide), run_time=0.6)
        self.add(thread_flow)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.9 + 0.6)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kette"))
        self.play(
            gauge["level"].animate.set_value(0.8), lamp["bulb"].animate.set_fill(P_YELLOW, opacity=0.9),
            tiles[0][0].animate.set_stroke(width=3.6), *glow, run_time=1.6)
        leaving = [(heat_paths, P_RED, P_ORANGE)]
        self.play(flow_animation(leaving, waves=3, radius=0.05, cycles=2.0), tiles[1][0].animate.set_stroke(width=3.6),
                  run_time=1.8)
        self.play(shine(sun_beams), tiles[2][0].animate.set_stroke(width=3.6), run_time=1.8)
        hold_for(self, self.NARRATION, "kette", used=1.6 + 1.8 + 1.8 + 0.35,
                 during=lambda rt: [flow_animation(leaving, waves=3, radius=0.05, cycles=max(1.0, rt / 0.9)),
                                    pulse_flashes(beam_paths, P_YELLOW, repeats=max(1, int(rt / 1.6)), width=3.5)])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bilanz"))
        hs = house_section(np.array([-5.15, STAGE_Y + 0.768, 0.0]), scale=0.64)
        bl, br = hs["bottom_left"], hs["bottom_right"]
        room_h = hs["w_height"] / 2
        lo_win = hs["windows"][0]
        rad = radiator(np.array([float(bl[0]) + 0.62, STAGE_Y + 0.17, 0.0])).scale(0.5)
        boiler = _boiler(np.array([float(br[0]) - 0.42, STAGE_Y + 0.32 + 0.02, 0.0]))
        person = _stand(STAGE_Y, room_h, float(bl[0]) + 1.25)
        gas = ValueTracker(0.0)
        gas_path = _polyline([(TANK_X, STAGE_Y + 0.10, 0), (TANK_X, PIPE_Y, 0), (float(br[0]) - 0.42, PIPE_Y, 0),
                              (float(br[0]) - 0.42, STAGE_Y + 0.05, 0)])
        pipe = VGroup(
            Line(np.array([TANK_X, STAGE_Y, 0.0]), np.array([TANK_X, PIPE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
            Line(np.array([TANK_X, PIPE_Y, 0.0]), np.array([float(br[0]) - 0.42, PIPE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
            Line(np.array([float(br[0]) - 0.42, PIPE_Y, 0.0]), np.array([float(br[0]) - 0.42, STAGE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
            Line(np.array([METER_X, PIPE_Y, 0.0]), np.array([METER_X, STAGE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
        )
        gas_flow = _stream(gas_path, gas, color=COL_PRIM, color_end=COL_END, count=14, speed=0.09, radius=0.05)
        meter_glyph = _gas_meter(np.array([METER_X, STAGE_Y + 0.37, 0.0]), gas)
        tank = energy_tank(np.array([TANK_X, STAGE_Y + 0.75, 0.0]), height=1.30, width=0.80, color=COL_PRIM, level=1.0)
        scale = _class_scale(CLASS_X, STAGE_Y + 1.90, STAGE_Y)
        cuts = VGroup(*[DashedLine(np.array([x, STAGE_Y + 0.02, 0.0]), np.array([x, 1.70, 0.0]), color=c, stroke_width=2.2,
                                   dash_length=0.12)
                        for x, c in zip(CUT_X, (COL_NUTZ, COL_END, COL_PRIM))])
        names = VGroup(*[body_text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([x, 1.92, 0.0]))
                         for x, t, c in zip(CUT_X, ("Raum", "Zähler", "Quelle"), (COL_NUTZ, COL_END, COL_PRIM))])
        ausweis = body_text("Ausweis", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([CLASS_X, STAGE_Y + 2.20, 0.0]))
        warm_spots = [rad.get_top() + UP * 0.02, person.get_top() + UP * 0.04]

        def heat(rt, r_max=0.30):
            return [ripples(warm_spots, r_max=r_max, cycles=max(1.0, rt / 1.4))]

        self.play(FadeOut(tiles[:3]), FadeOut(tiles[3][0]), FadeOut(tiles[3][1]), FadeOut(tiles[3][2]),
                  FadeOut(VGroup(lamp["group"], gauge["group"], gauge["column"], hs_heat["group"], hs_cool["group"], sun_small,
                                 sun_beams["group"])),
                  FadeOut(thread_guide), FadeOut(thread_flow),
                  ReplacementTransform(hs_small["group"], hs["group"]), run_time=1.5)
        self.remove(thread_flow)
        self.play(FadeIn(rad), FadeIn(boiler), FadeIn(person), Create(cuts[0]), FadeIn(names[0]), *heat(1.0), run_time=1.0)
        self.add(gas_flow, tank["fill"])
        self.play(Create(pipe), FadeIn(meter_glyph), gas.animate.set_value(1.0), Create(cuts[1]), FadeIn(names[1]),
                  *heat(1.4), run_time=1.4)
        self.play(FadeIn(tank["group"]), Create(cuts[2]), FadeIn(names[2]), tank["level"].animate.set_value(0.8),
                  *heat(1.4), run_time=1.4)
        self.play(FadeIn(scale["bands"], shift=LEFT * 0.2), FadeIn(ausweis), *heat(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "bilanz", used=1.5 + 1.0 + 1.4 + 1.4 + 1.2 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zahlen"))
        stack_a = _stack(X_A, [COL_NUTZ])
        stack_b = _stack(X_B, [COL_NUTZ, *LOSS_COLOURS])
        stack_c = _stack(X_C, [COL_NUTZ, *LOSS_COLOURS, COL_PRIM])
        stack_bw = _stack(X_B_WP, [COL_END], width=0.28)
        stack_cw = _stack(X_C_WP, [COL_END, COL_PRIM], width=0.28)
        self.add(stack_a["mob"], stack_b["mob"], stack_c["mob"], stack_bw["mob"], stack_cw["mob"])
        guide = DashedLine(np.array([X_A - 0.25, STAGE_Y + CHAIN_NUTZ * STAGE_UNIT, 0.0]),
                           np.array([X_C + 0.25, STAGE_Y + CHAIN_NUTZ * STAGE_UNIT, 0.0]), color=P_WHITE, stroke_width=1.6,
                           dash_length=0.10, stroke_opacity=0.5)

        def ratio(num, den):
            return num() / den() if den() > 1e-6 else 0.0

        row, box, items = math_panel([
            ("qh", r"Q_{h}", COL_NUTZ), (None, r"\cdot", P_WHITE), ("ep", r"e_{p}", COL_END), (None, r"\cdot", P_WHITE),
            ("fp", r"f_{p}", COL_PRIM), (None, "=", P_WHITE), ("qp", r"Q_{P}", COL_PRIM),
            (None, r"\;[\mathrm{kWh/(m^{2}\cdot a)}]", P_TEAL),
        ], size=24, edge_buff=2.32)

        def e_p():
            return ratio(stack_b["total"], stack_a["total"])

        def f_p():
            return ratio(stack_c["total"], stack_b["total"])

        def top_read(expr, stack, x_left, colour, y_min=-9.0):
            return math_readout(expr, lambda: np.array([x_left, max(y_min, stack["top"]() + 0.30), 0.0]), size=LABEL_FONT_SIZE,
                                color=colour, edge="left")

        qh_read = top_read(lambda: rf"Q_{{h}}={de_num(stack_a['total']())}", stack_a, X_A - 0.25, COL_NUTZ)
        qe_read = top_read(lambda: rf"Q_{{E}}={de_num(stack_a['total']())}\cdot {de_num(e_p(), 2)}={de_num(stack_b['total']())}",
                           stack_b, X_B - 0.25, COL_END, y_min=1.40)
        qp_read = top_read(lambda: rf"Q_{{P}}={de_num(stack_b['total']())}\cdot {de_num(f_p(), 1)}={de_num(stack_c['total']())}",
                           stack_c, X_C - 0.25, COL_PRIM, y_min=1.40)
        wp_e_read = math_readout(lambda: de_num(stack_bw["total"]()), lambda: np.array([X_B_WP, stack_bw["top"]() + 0.28, 0.0]),
                                 size=LABEL_FONT_SIZE, color=COL_END, edge="center")
        wp_p_read = math_readout(lambda: de_num(stack_cw["total"]()), lambda: np.array([X_C_WP, stack_cw["top"]() + 0.28, 0.0]),
                                 size=LABEL_FONT_SIZE, color=COL_PRIM, edge="center")
        unit_note = VGroup(
            body_text("Gaskessel-Variante, schematisches Beispiel, in", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            math_label(r"\mathrm{kWh/(m^{2}\cdot a)}", size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(RIGHT, buff=0.14, aligned_edge=DOWN).move_to(np.array([0.0, 2.30, 0.0]))
        gas_class, gas_colour = _class_of(CHAIN_END)
        wp_class, wp_colour = _class_of(CHAIN_END_WP)
        y_gas, y_wp = scale["centres"][gas_class][1], scale["centres"][wp_class][1]
        chip_gas = _class_chip(gas_class, gas_colour, np.array([CHIP_X, y_gas, 0.0]))
        chip_wp = _class_chip(wp_class, wp_colour, np.array([CHIP_X, y_wp, 0.0]))
        link_gas = Line(chip_gas.get_right(), np.array([CLASS_X - 0.31, y_gas, 0.0]), color=gas_colour, stroke_width=2.4)
        link_wp = Line(chip_wp.get_right(), np.array([CLASS_X - 0.31, y_wp, 0.0]), color=wp_colour, stroke_width=2.4)
        tag_gas = body_text("Gaskessel", font_size=LABEL_FONT_SIZE, color=gas_colour).next_to(chip_gas, LEFT, buff=0.16)
        tag_wp = body_text("Wärmepumpe", font_size=LABEL_FONT_SIZE, color=wp_colour).next_to(chip_wp, LEFT, buff=0.16)

        self.add(qh_read)
        self.play(FadeIn(row), Create(box), FadeIn(unit_note), stack_a["heights"][0].animate.set_value(CHAIN_NUTZ),
                  *heat(1.4), run_time=1.4)
        ring_h = highlight_param(items, "qh", color=COL_NUTZ)
        self.play(Create(ring_h), Create(guide), *heat(0.5), run_time=0.5)
        self.add(qe_read)
        ring_e = highlight_param(items, "ep", color=COL_END)
        self.play(FadeOut(ring_h), Create(ring_e), stack_b["heights"][0].animate.set_value(CHAIN_NUTZ), run_time=0.8)
        for h, value in zip(stack_b["heights"][1:], LOSS_VALUES):
            self.play(h.animate.set_value(value), run_time=0.6)
        self.add(qp_read)
        ring_f = highlight_param(items, "fp", color=COL_PRIM)
        self.play(FadeOut(ring_e), Create(ring_f),
                  *[h.animate.set_value(v) for h, v in zip(stack_c["heights"][:-1], (CHAIN_NUTZ, *LOSS_VALUES))], run_time=1.0)
        self.play(stack_c["heights"][-1].animate.set_value(CHAIN_PRIM - CHAIN_END), run_time=0.9)
        self.play(FadeOut(ring_f), FadeIn(link_gas), FadeIn(chip_gas, shift=LEFT * 0.2), FadeIn(tag_gas), run_time=1.0)
        self.add(wp_e_read, wp_p_read)
        self.play(stack_bw["heights"][0].animate.set_value(CHAIN_END_WP), run_time=0.9)
        self.play(stack_cw["heights"][0].animate.set_value(CHAIN_END_WP),
                  stack_cw["heights"][1].animate.set_value(CHAIN_PRIM_WP - CHAIN_END_WP),
                  run_time=1.0)
        self.play(TransformFromCopy(chip_gas, chip_wp), FadeIn(link_wp), FadeIn(tag_wp), run_time=1.2)
        hold_for(self, self.NARRATION, "zahlen", used=1.4 + 0.5 + 0.8 + 0.6 * 4 + 1.0 + 0.9 + 1.0 + 0.9 + 1.0 + 1.2 + 0.35,
                 during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        tops = [np.array([X_A, stack_a["top"](), 0.0]), np.array([X_B, stack_b["top"](), 0.0]),
                np.array([X_C, stack_c["top"](), 0.0])]
        cold_wall = Line(bl + RIGHT * 0.01, bl + UP * room_h + RIGHT * 0.01, color=COLD_BLUE, stroke_width=6)
        cold_spots = [bl + UP * 0.30 + RIGHT * 0.03, bl + UP * 0.62 + RIGHT * 0.03]
        path = [bl + UP * 0.45, tops[0], tops[1], tops[2], chip_gas.get_left() + LEFT * 0.05]
        trace = VMobject(color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.55).set_points_as_corners(path)
        landlord = _stand(STAGE_Y, room_h, 3.85)
        tenant = _stand(STAGE_Y, room_h, 4.55, color=P_CYAN)
        speech = _class_chip(gas_class, gas_colour, np.array([3.85, 0.62, 0.0]))
        cloud = cloud_glyph(speech, pad=(0.34, 0.26), bump=0.40)
        trail = cloud_trail(cloud, landlord.get_top() + UP * 0.12)

        def cold_air(rt):
            return [ripples(cold_spots, r_max=0.5, color=COLD_BLUE, facing=0.0, cycles=max(1.0, rt / 1.4))]

        self.play(FadeOut(VGroup(qh_read, qe_read, qp_read, wp_e_read, wp_p_read, unit_note, tag_gas, tag_wp,
                                 link_wp, chip_wp, chip_gas, link_gas)), Create(cold_wall), *cold_air(1.4), run_time=1.4)
        self.play(Create(trace), *cold_air(1.4), run_time=1.4)
        self.play(pulse_flashes([path], P_YELLOW, repeats=2, width=5.0), *cold_air(2.6), run_time=2.6)
        self.play(FadeIn(landlord), FadeIn(tenant), FadeOut(trace), run_time=0.7)
        self.play(FadeIn(cloud), FadeIn(speech), FadeIn(trail), run_time=0.9)
        hold_for(self, self.NARRATION, "outro", used=1.4 + 1.4 + 2.6 + 0.7 + 0.9 + 0.35, during=lambda rt: heat(rt) + cold_air(rt))

        for mob in (gas_flow, meter_glyph[3], qh_read, qe_read, qp_read, wp_e_read, wp_p_read):
            mob.clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.6)

#endregion
