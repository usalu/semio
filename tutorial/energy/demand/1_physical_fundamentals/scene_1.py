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
    SUBTITLE_FONT_SIZE, BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_BOTTOM, SAFE_BOTTOM_FORMULA, fit_band,
    radiation_waves, convection_stream, symbol_token, watt_anchor,
    smooth_path, flow_guides, animate_flow, animate_haze,
    equation_row, formula_panel, highlight_param, chip, cross_mark, dim_arrow,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing, begin_vo_beat,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — animated once on Beat1, self.add()'ed on every later beat.
TITLE_DE = "Physikalische Zusammenhänge: Kraft, Leistung & Energie"

# Mid-screen anchor for diagram content — clear of the title block above and of the
# fixed formula_panel + caption_bar zones below.
CONTENT_CENTER = UP * 0.25


#region Shared visual motifs
# The through-line: one Hannover cross-section house is the "why this matters"
# anchor of every beat, and a pipe→bucket water picture carries the kW-vs-kWh
# distinction from Beat 4 to the closing outlook.

def _build_cross_section_house(center=ORIGIN + DOWN * 0.45):
    """🏠 Two-storey line-art house — the same glyph the Cooling/Heating tutorials use."""
    w_width, w_height = 3.6, 2.4
    bottom_left = center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
    bottom_right = center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
    top_left = center + LEFT * (w_width / 2) + UP * (w_height / 2)
    top_right = center + RIGHT * (w_width / 2) + UP * (w_height / 2)
    roof_peak = center + UP * (w_height / 2 + 1.1)

    floor_line = Line(bottom_left + LEFT * 0.6, bottom_right + RIGHT * 0.6, color=P_TEAL, stroke_width=4)
    level_1 = Line(bottom_left + UP * (w_height / 2), bottom_right + UP * (w_height / 2), color=P_WHITE, stroke_width=2)
    w_h = 0.5
    wall_left_1 = Line(bottom_left, bottom_left + UP * (w_height / 4 - w_h / 2), color=P_WHITE, stroke_width=3)
    wall_left_2 = Line(
        bottom_left + UP * (w_height / 4 + w_h / 2),
        bottom_left + UP * (3 * w_height / 4 - w_h / 2),
        color=P_WHITE, stroke_width=3,
    )
    wall_left_3 = Line(bottom_left + UP * (3 * w_height / 4 + w_h / 2), top_left, color=P_WHITE, stroke_width=3)
    wall_right = Line(bottom_right, top_right, color=P_WHITE, stroke_width=3)
    walls = VGroup(wall_left_1, wall_left_2, wall_left_3, wall_right, level_1)
    roof = Polygon(top_left, roof_peak, top_right, color=P_WHITE, stroke_width=3)
    win1 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (w_height / 4) + RIGHT * 0.075)
    win2 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (3 * w_height / 4) + RIGHT * 0.075)
    window_group = VGroup(win1, win2)
    return {
        "center": center, "bottom_left": bottom_left, "bottom_right": bottom_right,
        "top_left": top_left, "top_right": top_right, "roof_peak": roof_peak,
        "floor": floor_line, "walls": walls, "roof": roof, "window_group": window_group,
        "group": VGroup(floor_line, walls, roof, window_group),
        "w_width": w_width, "w_height": w_height,
    }


def _build_sun(pos, color=P_YELLOW):
    """☀️ Compact sun — reused from the Cooling tutorials."""
    sun_core = Dot(pos, radius=0.45, color=color)
    sun_glow = Dot(pos, radius=0.7, color=color, fill_opacity=0.35)
    sun_ring1 = Circle(radius=0.85, color=color, stroke_width=2, stroke_opacity=0.6).move_to(pos)
    sun_ring2 = Circle(radius=1.1, color=color, stroke_width=1.2, stroke_opacity=0.3).move_to(pos)
    sun_burst = VGroup()
    for angle in np.linspace(0, TAU, 12, endpoint=False):
        s = pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
        e = pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
        sun_burst.add(Line(s, e, color=color, stroke_width=2))
    return VGroup(sun_glow, sun_core, sun_ring1, sun_ring2, sun_burst)


def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Occupant glyph — same as Heating Modul 1."""
    head = Circle(radius=0.11, color=color, stroke_width=2.2)
    body = RoundedRectangle(
        width=0.30, height=0.40, corner_radius=0.09, color=color, stroke_width=2.2,
    ).next_to(head, DOWN, buff=0.04)
    return VGroup(head, body).scale(scale).move_to(pos)


def _tree_and_apple(ground_y=-1.15):
    """🌳 Line-art tree with a falling apple for the Newton beat.

    Trunk base sits on ``ground_y``; crown and trunk are stroke-only (no fill).
    """
    ground = Line(LEFT * 1.75, RIGHT * 1.75, color=P_TEAL, stroke_width=3)
    trunk_base = np.array([0.0, ground_y, 0.0])
    trunk_top = trunk_base + UP * 1.42
    trunk = Line(trunk_base, trunk_top, color=P_WHITE, stroke_width=5)
    crown = Circle(radius=0.52, color=P_WHITE, stroke_width=3, fill_opacity=0.0)
    crown.move_to(trunk_top + UP * 0.30)
    tree = VGroup(trunk, crown)
    ground.move_to(trunk_base)

    stem = Line(ORIGIN, DOWN * 0.07, color=P_WHITE, stroke_width=2.2)
    apple_body = Circle(
        radius=0.13, color=P_RED, fill_color=P_RED,
        fill_opacity=0.92, stroke_width=2,
    )
    apple_body.next_to(stem, DOWN, buff=0)
    apple = VGroup(stem, apple_body)
    return tree, apple, ground


def _stopwatch(reading="", color=P_CYAN):
    """⏱️ Minimal stopwatch with an optional reading tag underneath."""
    face = Circle(radius=0.42, color=P_WHITE, stroke_width=2.5)
    hand = Line(ORIGIN, UP * 0.30, color=color, stroke_width=3)
    crown = Line(UP * 0.42, UP * 0.52, color=P_WHITE, stroke_width=3)
    grp = VGroup(face, hand, crown)
    tag = None
    if reading:
        tag = Text(reading, font_size=LABEL_FONT_SIZE, color=color)
        tag.next_to(face, DOWN, buff=0.16)
        grp = VGroup(face, hand, crown, tag)
    return {"face": face, "hand": hand, "tag": tag, "group": grp}


def _meter_dial():
    """⚡ Spinning electricity-meter face."""
    body = RoundedRectangle(width=1.6, height=1.1, corner_radius=0.08, color=P_TEAL, stroke_width=2.5)
    dial = Circle(radius=0.32, color=P_WHITE, stroke_width=2).move_to(body.get_center() + UP * 0.05)
    needle = Line(dial.get_center(), dial.get_center() + UP * 0.24, color=P_YELLOW, stroke_width=3)
    label = Text("kWh", font_size=LABEL_FONT_SIZE, color=P_CYAN)
    label.next_to(body, DOWN, buff=0.12)
    return {"body": body, "dial": dial, "needle": needle, "label": label, "group": VGroup(body, dial, needle, label)}


def _water_bucket(*, width=1.12, height=0.96, fill_frac=0.36):
    """🪣 Open-top bucket with tapered walls and a visible water fill."""
    hw_top = width / 2
    hw_bot = width * 0.36
    hh = height / 2
    rim_l = np.array([-hw_top, hh, 0.0])
    rim_r = np.array([hw_top, hh, 0.0])
    bot_l = np.array([-hw_bot, -hh, 0.0])
    bot_r = np.array([hw_bot, -hh, 0.0])
    shell = VGroup(
        Line(rim_l, bot_l, color=P_WHITE, stroke_width=3),
        Line(rim_r, bot_r, color=P_WHITE, stroke_width=3),
        Line(bot_l, bot_r, color=P_WHITE, stroke_width=3),
        Line(rim_l, rim_r, color=P_TEAL, stroke_width=3.5),
    )
    fill_h = max(0.10, height * fill_frac)
    t = fill_h / height
    surf_l = bot_l * (1 - t) + rim_l * t
    surf_r = bot_r * (1 - t) + rim_r * t
    inset = 0.05
    water = Polygon(
        bot_l + UP * inset + RIGHT * inset,
        bot_r + UP * inset + LEFT * inset,
        surf_r + DOWN * inset,
        surf_l + DOWN * inset,
        stroke_width=0, fill_color=P_CYAN, fill_opacity=0.44,
    )
    surface = Line(surf_l, surf_r, color=P_CYAN, stroke_width=2.4)
    handle = Arc(radius=0.26, start_angle=PI * 0.12, angle=PI * 0.76, color=P_WHITE, stroke_width=2.5)
    handle.move_to(rim_r + RIGHT * 0.14 + UP * 0.04)
    return VGroup(water, surface, shell, handle)


def _pipe_and_bucket():
    """🚰 Pipe (rate) with spout feeding an open bucket (amount)."""
    pipe_body = Rectangle(
        width=3.0, height=0.26, color=P_WHITE, stroke_width=2.5,
        fill_color=P_CYAN, fill_opacity=0.30,
    )
    spout = Line(ORIGIN, DOWN * 0.44, color=P_WHITE, stroke_width=3.2)
    spout.move_to(pipe_body.get_bottom() + LEFT * 0.42)
    nozzle = Dot(spout.get_end(), radius=0.055, color=P_CYAN)
    pipe = VGroup(pipe_body, spout, nozzle)
    bucket = _water_bucket()
    bucket.next_to(spout, DOWN, buff=0.10).shift(RIGHT * 0.06)
    stream = smooth_path([
        spout.get_end(),
        spout.get_end() + DOWN * 0.20 + RIGHT * 0.04,
        bucket.get_top() + LEFT * 0.04,
    ])
    return pipe, bucket, stream


def _pour_rig(origin=np.array([1.2, 0.15, 0.0])):
    """🚰 Legible pipe → bucket rig: a bent pipe, an open bucket that starts **empty**,
    and ``frac`` (0..1) driving an ``always_redraw`` rising water body, water surface
    and live kWh readout — so "kWh = the amount that collects over time" is watchable."""
    ox, oy = float(origin[0]), float(origin[1])
    pipe_y = oy + 1.70
    x_in = ox - 3.0
    spout_x = ox + 0.15

    rail_hi = VMobject(color=P_CYAN, stroke_width=3)
    rail_hi.set_points_as_corners([
        np.array([x_in, pipe_y + 0.15, 0.0]),
        np.array([spout_x + 0.15, pipe_y + 0.15, 0.0]),
        np.array([spout_x + 0.15, pipe_y - 0.55, 0.0]),
    ])
    rail_lo = VMobject(color=P_CYAN, stroke_width=3)
    rail_lo.set_points_as_corners([
        np.array([x_in, pipe_y - 0.15, 0.0]),
        np.array([spout_x - 0.15, pipe_y - 0.15, 0.0]),
        np.array([spout_x - 0.15, pipe_y - 0.55, 0.0]),
    ])
    nozzle = Line(np.array([spout_x - 0.15, pipe_y - 0.55, 0.0]),
                  np.array([spout_x + 0.15, pipe_y - 0.55, 0.0]),
                  color=P_CYAN, stroke_width=3)
    pipe = VGroup(rail_hi, rail_lo, nozzle)

    bw_top, bw_bot, bh = 1.95, 1.6, 1.6
    floor_y = oy - 1.05
    rim_l = np.array([spout_x - bw_top / 2, floor_y + bh, 0.0])
    rim_r = np.array([spout_x + bw_top / 2, floor_y + bh, 0.0])
    bot_l = np.array([spout_x - bw_bot / 2, floor_y, 0.0])
    bot_r = np.array([spout_x + bw_bot / 2, floor_y, 0.0])
    bucket = VGroup(
        Line(rim_l, bot_l, color=P_TEAL, stroke_width=4),
        Line(bot_l, bot_r, color=P_TEAL, stroke_width=4),
        Line(bot_r, rim_r, color=P_TEAL, stroke_width=4),
    )

    frac = ValueTracker(0.0)

    def _lerp(t):
        t = float(np.clip(t, 0.0, 1.0))
        return bot_l * (1 - t) + rim_l * t, bot_r * (1 - t) + rim_r * t

    def _vis():
        # 5 % baseline puddle → 88 % full at frac 1, so the rim keeps some freeboard.
        return 0.05 + 0.83 * float(np.clip(frac.get_value(), 0.0, 1.0))

    def _water():
        sl, sr = _lerp(_vis())
        return Polygon(
            bot_l + np.array([0.05, 0.04, 0.0]), bot_r + np.array([-0.05, 0.04, 0.0]),
            sr + np.array([-0.05, 0.0, 0.0]), sl + np.array([0.05, 0.0, 0.0]),
            stroke_width=0, fill_color=P_CYAN, fill_opacity=0.5,
        )

    def _surface():
        sl, sr = _lerp(_vis())
        return Line(sl, sr, color=P_CYAN, stroke_width=2.6)

    def _readout():
        v = float(np.clip(frac.get_value(), 0.0, 1.0))
        return Text(
            f"{v:.1f}".replace(".", ",") + " kWh",
            font_size=BODY_FONT_SIZE, color=P_TEAL,
        ).move_to(np.array([rim_r[0] + 1.2, floor_y + 0.42, 0.0]))

    return {
        "pipe": pipe, "bucket": bucket,
        "water": always_redraw(_water), "surface": always_redraw(_surface),
        "readout": always_redraw(_readout), "frac": frac,
        "spout": np.array([spout_x, pipe_y - 0.58, 0.0]),
        "rim_l": rim_l, "rim_r": rim_r, "floor_y": floor_y, "bh": bh,
        "pipe_y": pipe_y, "spout_x": spout_x,
    }


def _heat_pump_box():
    """♨️ Schematic heat-pump block."""
    box = RoundedRectangle(width=2.8, height=1.5, corner_radius=0.12, color=P_TEAL, stroke_width=2.5)
    label = Text("Wärmepumpe", font_size=LABEL_FONT_SIZE, color=P_WHITE)
    label.move_to(box.get_center())
    return VGroup(box, label), box


def _resistance_heater():
    """🔥 Resistance heater block — the COP = 1 counter-example."""
    box = RoundedRectangle(width=2.2, height=1.4, corner_radius=0.12, color=P_RED, stroke_width=2.5)
    coil = VGroup(*[
        Arc(radius=0.13, start_angle=PI, angle=-PI, stroke_width=2.5, color=P_ORANGE,
            arc_center=box.get_center() + LEFT * 0.55 + RIGHT * i * 0.28 + DOWN * 0.18)
        for i in range(5)
    ])
    label = Text("Heizstab", font_size=LABEL_FONT_SIZE, color=P_WHITE)
    label.move_to(box.get_center() + UP * 0.36)
    return VGroup(box, coil, label), box


def _spec_card(heading, value, color, *, width_min=2.6):
    """🪪 Nameplate-style card; returns the group and its editable value Text."""
    head = Text(heading, font_size=LABEL_FONT_SIZE, color=P_WHITE)
    val = Text(value, font_size=FORMULA_FONT_SIZE, color=color)
    body = VGroup(head, val).arrange(DOWN, buff=0.16)
    frame = RoundedRectangle(
        width=max(width_min, body.width + 0.5), height=body.height + 0.44, corner_radius=0.12,
        color=color, stroke_width=2, fill_color=P_DEEP_DARK, fill_opacity=0.92,
    )
    body.move_to(frame.get_center())
    return VGroup(frame, body), val


def _speedometer(center, radius=0.72):
    """🚗 Small analogue dial — the "instantaneous reading" picture for power."""
    face = Arc(radius=radius, start_angle=PI * 0.92, angle=-PI * 0.84, color=P_WHITE,
               stroke_width=3, arc_center=center)
    ticks = VGroup()
    for f in np.linspace(0.0, 1.0, 6):
        ang = PI * 0.92 - f * PI * 0.84
        inner = center + np.array([np.cos(ang) * (radius - 0.12), np.sin(ang) * (radius - 0.12), 0.0])
        outer = center + np.array([np.cos(ang) * radius, np.sin(ang) * radius, 0.0])
        ticks.add(Line(inner, outer, color=P_WHITE, stroke_width=2))
    hub = Dot(center, radius=0.05, color=P_CYAN)
    start_ang = PI * 0.92
    needle = Line(
        center,
        center + np.array([np.cos(start_ang) * (radius - 0.16), np.sin(start_ang) * (radius - 0.16), 0.0]),
        color=P_CYAN, stroke_width=4,
    )
    return {
        "face": face, "ticks": ticks, "hub": hub, "needle": needle,
        "group": VGroup(face, ticks, needle, hub), "center": center, "radius": radius,
        "_ang": start_ang,
    }


def _speedo_rotate(spd, frac):
    """↻ Rotate animation that swings the needle to ``frac`` of full scale."""
    target = PI * 0.92 - float(np.clip(frac, 0.0, 1.0)) * PI * 0.84
    delta = target - spd["_ang"]
    spd["_ang"] = target
    return Rotate(spd["needle"], angle=delta, about_point=spd["center"])


def _energy_split(left_x, y, *, total_w=3.1, height=0.95):
    """🔀 A 100 % input bar that forks into a thin "light" lane and a wide "heat" lane."""
    src = Rectangle(width=0.55, height=height, color=P_CYAN, fill_color=P_CYAN,
                    fill_opacity=0.45, stroke_width=2).move_to(np.array([left_x, y, 0.0]))
    light = Rectangle(width=total_w, height=height * 0.14, color=P_YELLOW, fill_color=P_YELLOW,
                      fill_opacity=0.65, stroke_width=1.5)
    heat = Rectangle(width=total_w, height=height * 0.82, color=P_RED, fill_color=P_RED,
                     fill_opacity=0.30, stroke_width=1.5)
    branch = VGroup(light, heat).arrange(DOWN, buff=0.12)
    branch.next_to(src, RIGHT, buff=0.12)
    sink = Rectangle(width=0.55, height=height, color=P_RED, fill_color=P_RED,
                     fill_opacity=0.42, stroke_width=2)
    sink.next_to(branch, RIGHT, buff=0.12)
    return {"src": src, "light": light, "heat": heat, "branch": branch, "sink": sink,
            "group": VGroup(src, branch, sink)}


def _unit_cheatsheet():
    """📇 Compact key of the four units the module introduces."""
    rows = [
        ("N", "Kraft", P_RED),
        ("J", "Arbeit · Energie", P_YELLOW),
        ("W = J/s", "Leistung", P_CYAN),
        ("kWh", "Energie im Alltag", P_TEAL),
    ]
    built = VGroup()
    for sym, meaning, color in rows:
        s = Text(sym, font_size=LABEL_FONT_SIZE, color=color)
        dash = Text("—", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        m = Text(meaning, font_size=LABEL_FONT_SIZE, color=P_WHITE)
        built.add(VGroup(s, dash, m).arrange(RIGHT, buff=0.16))
    built.arrange(DOWN, aligned_edge=LEFT, buff=0.14)
    frame = SurroundingRectangle(built, color=P_TEAL, corner_radius=0.1, buff=0.2, stroke_width=1.6)
    return VGroup(frame, built)

#endregion


#region Beat1 – The invisible dimension of architecture
class Beat1_UnsichtbareDimension(Scene):
    NARRATION = [
        ("intro",
         "Architecture is usually described as the art of shaping space. But the moment that space is enclosed by walls, a roof and windows, a second discipline takes over — building physics — and it never switches off.",
         "Architektur formt Raum. Doch sobald der Raum geschlossen ist, übernimmt die Bauphysik — und sie hört nie auf."),
        ("seasons",
         "The same house has to answer two very different questions. How much heat does it lose on a cold January night in Hannover? And how badly does it overheat under the July sun?",
         "Dasselbe Haus muss zwei Fragen beantworten: Wie viel Wärme verliert es im Winter? Wie stark überhitzt es im Sommer?"),
        ("language",
         "Before we can calculate a heating load or an annual demand, we have to speak the language those calculations are written in — and that language is built from just four physical quantities.",
         "Bevor wir Heizlast oder Jahresbedarf berechnen, müssen wir ihre Sprache sprechen — aufgebaut aus vier Größen."),
        ("quantities",
         "Force, measured in newtons. Work and energy, measured in joules. Power, measured in watts. And, for everyday building numbers, the kilowatt-hour. Four words — we will earn each one.",
         "Kraft in Newton. Arbeit und Energie in Joule. Leistung in Watt. Und im Alltag die Kilowattstunde."),
        ("confusion",
         "In practice these get mixed up constantly. A boiler is quoted in kilowatt-hours instead of kilowatts. An annual bill is written in kilowatts per year instead of kilowatt-hours. Both look almost right and are completely wrong.",
         "In der Praxis werden sie verwechselt: Kessel in kWh statt kW, Jahresverbrauch in kW/a statt kWh/a."),
        ("plan",
         "Over the next minutes we take the units apart, tie every formula to something you can picture, and build a feel for the numbers using Hannover as our example city.",
         "Wir zerlegen die Einheiten, knüpfen jede Formel an ein Bild und entwickeln ein Gefühl für die Zahlen — Beispielstadt Hannover."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Die unsichtbare Dimension der Architektur", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        house = _build_cross_section_house(center=CONTENT_CENTER + DOWN * 0.2)

        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "seasons"))
        self.play(FadeIn(house["group"], shift=UP * 0.2), run_time=1.2)
        haze_warm = dict(
            x0=house["center"][0] - 2.0, x1=house["center"][0] + 2.2,
            y0=house["center"][1] - 1.0, y1=house["center"][1] + 1.4,
            color=P_RED, color_end="#C9786E", n=34,
        )
        haze_cool = dict(
            x0=house["center"][0] + 2.2, x1=house["center"][0] - 2.0,
            y0=house["center"][1] - 1.0, y1=house["center"][1] + 1.4,
            color=P_BLUE, color_end="#6A9FD4", n=30, seed=7,
        )
        animate_haze(self, run_time=2.0, cycles=1.5, **haze_warm)
        animate_haze(self, run_time=1.8, cycles=1.3, **haze_cool)
        hold_for(self, self.NARRATION, "seasons", used=1.2 + 2.0 + 1.8)

        # Four unit chips in a row, high in the free band (subtitle sits far above at y≈2.9).
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "language"))
        chips = VGroup(
            chip("Kraft — N", P_RED, font_size=LABEL_FONT_SIZE),
            chip("Arbeit — J", P_YELLOW, font_size=LABEL_FONT_SIZE),
            chip("Leistung — W", P_CYAN, font_size=LABEL_FONT_SIZE),
            chip("Alltag — kWh", P_TEAL, font_size=LABEL_FONT_SIZE),
        ).arrange(RIGHT, buff=0.28)
        if chips.width > 12.0:
            chips.scale(12.0 / chips.width)
        chips.move_to(UP * 1.75)
        self.play(house["group"].animate.scale(0.82).move_to(CONTENT_CENTER + DOWN * 0.55), run_time=0.7)
        self.play(LaggedStart(*[FadeIn(c, shift=DOWN * 0.15) for c in chips], lag_ratio=0.18), run_time=1.4)
        hold_for(self, self.NARRATION, "language", used=0.7 + 1.4 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "quantities"))
        self.play(
            LaggedStart(*[Indicate(c, color=c[1].get_color(), scale_factor=1.12) for c in chips], lag_ratio=0.25),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "quantities", used=2.0 + 0.35)

        # Two wrong-unit nameplates, one per side; the house is only a faint backdrop now.
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "confusion"))
        boiler_card, boiler_val = _spec_card("Kessel · Leistung", "24 kWh", P_RED)
        boiler_card.move_to(LEFT * 3.45 + DOWN * 0.15)
        bill_card, bill_val = _spec_card("Rechnung · Verbrauch", "12 000 kW/a", P_RED, width_min=3.0)
        bill_card.move_to(RIGHT * 3.45 + DOWN * 0.15)
        self.play(house["group"].animate.set_stroke(opacity=0.18), FadeOut(chips), run_time=0.6)
        self.play(FadeIn(boiler_card, shift=RIGHT * 0.2), FadeIn(bill_card, shift=LEFT * 0.2), run_time=1.0)
        x_boiler = cross_mark(P_RED, size=0.24).move_to(boiler_val)
        x_bill = cross_mark(P_RED, size=0.24).move_to(bill_val)
        self.play(Create(x_boiler), Create(x_bill), run_time=0.6)
        boiler_fix = Text("24 kW", font_size=FORMULA_FONT_SIZE, color=P_GREEN).move_to(boiler_val)
        bill_fix = Text("12 000 kWh/a", font_size=FORMULA_FONT_SIZE, color=P_GREEN).move_to(bill_val)
        self.play(
            FadeOut(x_boiler), FadeOut(x_bill),
            ReplacementTransform(boiler_val, boiler_fix),
            ReplacementTransform(bill_val, bill_fix),
            run_time=1.1,
        )
        hold_for(self, self.NARRATION, "confusion", used=0.6 + 1.0 + 0.6 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "plan"))
        topic_card = chip("Kraft · Leistung · Energie", P_YELLOW, font_size=BODY_FONT_SIZE)
        topic_card.scale(1.12).move_to(UP * 1.75)
        city_tag = Text("Beispielstadt: Hannover", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        city_tag.next_to(topic_card, DOWN, buff=0.18)
        self.play(
            FadeOut(boiler_card), FadeOut(bill_card), FadeOut(boiler_fix), FadeOut(bill_fix),
            house["group"].animate.set_stroke(opacity=0.45).move_to(CONTENT_CENTER + DOWN * 0.95),
            FadeIn(topic_card, shift=DOWN * 0.2), FadeIn(city_tag),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "plan", used=1.3 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – From force to work (newton and joule)
class Beat2_KraftUndArbeit(Scene):
    NARRATION = [
        ("start",
         "We start at the very bottom of the ladder, with force. Force is any push or pull. Its unit is the newton, and it is defined through gravity.",
         "Wir beginnen ganz unten: bei der Kraft. Ihre Einheit ist das Newton — definiert über die Schwerkraft."),
        ("newton",
         "Hold a bar of chocolate — about one hundred grams — in your open hand. The downward pull of the Earth on it is very close to one newton. That is the reference you can feel.",
         "Ein Riegel Schokolade, rund 100 Gramm: Die Erde zieht mit etwa einem Newton daran. Das ist das spürbare Maß."),
        ("gravity",
         "The pull comes from the gravitational acceleration g, about nine point eight one metres per second squared. Mass times g gives the weight force — one hundred grams times g is roughly one newton.",
         "Die Erdbeschleunigung g beträgt etwa 9,81 m/s². Masse mal g ergibt die Gewichtskraft — 100 g ergeben rund 1 N."),
        ("work",
         "Now move that force through a distance. Lifting the apple against gravity is work. Work W equals force F times the distance s along which the force acts.",
         "Bewegt man die Kraft über eine Strecke, verrichtet man Arbeit: W gleich Kraft F mal Weg s."),
        ("area",
         "You can read the work straight off a picture: the force is the width, the lifting height is the height, and the work is the area of that rectangle. A bigger lift sweeps out more area.",
         "Die Arbeit ist die Fläche: Kraft als Breite, Höhe als Höhe. Mehr Hub — mehr Fläche."),
        ("joule",
         "One newton acting over one metre is one joule. The joule is the international unit of energy, and every other energy unit is just a repackaged joule.",
         "Ein Newton mal ein Meter ist ein Joule — die internationale Einheit der Energie."),
        ("feel",
         "A joule is tiny. Lifting that chocolate bar a single metre costs about one joule. One human heartbeat releases roughly one joule. Keep that smallness in mind — it is why buildings need a bigger unit.",
         "Ein Joule ist winzig: der Schokoriegel einen Meter hoch, ein Herzschlag — je etwa ein Joule."),
        ("stored",
         "And the work does not vanish. The lifted apple now holds that joule as stored energy. Let it fall and the energy comes back out, doing work again on whatever it hits.",
         "Die Arbeit ist nicht weg: Der angehobene Apfel speichert sie. Fällt er, wird sie wieder zu Arbeit."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Von der Kraft zur Arbeit", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "start"))
        self.play(FadeIn(caption), run_time=0.3)

        # —— Left: the newton reference — an apple's weight against gravity ——
        tree, apple, ground = _tree_and_apple(ground_y=-1.62)
        tree_anchor = LEFT * 4.72 + DOWN * 0.18
        tree_grp = VGroup(tree, ground).move_to(tree_anchor)
        apple.move_to(ground.get_center() + RIGHT * 0.52 + UP * (apple[1].height / 2))
        apple_x = apple.get_center()[0]
        rest_y = apple.get_center()[1]
        ground_y_world = ground.get_center()[1]
        mass_tag = Text("100 g", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        fg_arrow = Arrow(ORIGIN, DOWN * 0.82, color=P_RED, buff=0, stroke_width=4,
                         max_tip_length_to_length_ratio=0.18)
        fg_lbl = Text("Gewicht ≈ 1 N", font_size=LABEL_FONT_SIZE, color=P_RED)
        g_lbl = Text("g ≈ 9,81 m/s²", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        g_lbl.move_to(LEFT * 4.72 + DOWN * 2.35)

        # —— Center: F–s diagram — area under constant F is work W = F·s ——
        s_max, f_const, lift_scene = 1.0, 1.0, 1.34
        axes = Axes(
            x_range=[0, 1.3, 0.5], y_range=[0, 1.3, 0.5],
            x_length=3.65, y_length=2.12,
            axis_config={
                "color": P_WHITE, "stroke_width": 2.2,
                "include_tip": True, "tip_length": 0.08, "tip_width": 0.08,
            },
            tips=True,
        )
        plot_origin = np.array([apple_x + 2.35, ground_y_world, 0.0])
        axes.shift(plot_origin - axes.c2p(0, 0))
        s_lbl = Text("s", font_size=BODY_FONT_SIZE, color=P_CYAN)
        s_lbl.next_to(axes.x_axis, RIGHT, buff=0.10)
        s_unit = Text("[m]", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        s_unit.next_to(s_lbl, RIGHT, buff=0.08)
        f_lbl = Text("F", font_size=BODY_FONT_SIZE, color=P_RED)
        f_lbl.next_to(axes.y_axis, UP, buff=0.10)
        f_unit = Text("[N]", font_size=LABEL_FONT_SIZE, color=P_RED)
        f_unit.next_to(f_lbl, UP, buff=0.08)
        f_tick = Text("1 N", font_size=LABEL_FONT_SIZE, color=P_RED)
        f_tick.next_to(axes.c2p(0, f_const), LEFT, buff=0.12)
        f_line = axes.plot(lambda _x: f_const, x_range=[0, 1.25], color=P_RED, stroke_width=2.6)
        axis_grp = VGroup(axes, s_lbl, s_unit, f_lbl, f_unit, f_tick, f_line)

        s_val = ValueTracker(0.0)
        work_area = always_redraw(lambda: Polygon(
            axes.c2p(0, 0), axes.c2p(max(0.03, s_val.get_value()), 0),
            axes.c2p(max(0.03, s_val.get_value()), f_const), axes.c2p(0, f_const),
            stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.28,
        ))
        s_marker = always_redraw(lambda: DashedLine(
            axes.c2p(max(0.02, s_val.get_value()), 0),
            axes.c2p(max(0.02, s_val.get_value()), f_const),
            color=P_CYAN, stroke_width=2, dash_length=0.08,
        ))
        s_dot = always_redraw(lambda: Dot(
            axes.c2p(s_val.get_value(), f_const),
            radius=0.055, color=P_CYAN,
        ))
        sync_line = always_redraw(lambda: DashedLine(
            np.array([
                apple_x + 0.42,
                rest_y + (s_val.get_value() / s_max) * lift_scene,
                0.0,
            ]),
            axes.c2p(max(0.02, s_val.get_value()), 0),
            color=P_WHITE, stroke_width=1.6, stroke_opacity=0.35, dash_length=0.07,
        ))
        w_note = Text("W = F · s", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        j_lbl = Text("1 J", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        area_lbls = VGroup(w_note, j_lbl).arrange(RIGHT, buff=0.34, aligned_edge=DOWN)
        area_lbls.move_to(axes.c2p(0.52, f_const * 0.52))

        eq_w, items_w = equation_row([
            ("w", "W", P_YELLOW), (None, "=", P_WHITE),
            ("f", "F", P_RED), (None, "·", P_WHITE), ("s", "s", P_CYAN),
            (None, "  [J]", P_TEAL),
        ])
        eq_w, box_w = formula_panel(eq_w, color=P_YELLOW)

        hold_for(self, self.NARRATION, "start", used=BEAT_SUBTITLE_FADE + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "newton"))
        mass_tag.next_to(apple, RIGHT, buff=0.16)
        self.play(FadeIn(tree_grp), FadeIn(apple), FadeIn(mass_tag), run_time=0.9)
        fg_arrow.next_to(apple, UP, buff=0.04)
        fg_lbl.next_to(fg_arrow, RIGHT, buff=0.12)
        self.play(GrowArrow(fg_arrow), FadeIn(fg_lbl), run_time=0.7)
        hold_for(self, self.NARRATION, "newton", used=0.9 + 0.7 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gravity"))
        self.play(FadeIn(g_lbl, shift=UP * 0.1), run_time=0.7)
        self.play(Indicate(fg_lbl, color=P_RED, scale_factor=1.15), run_time=0.9)
        hold_for(self, self.NARRATION, "gravity", used=0.7 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "work"))
        self.play(FadeIn(axis_grp), FadeIn(eq_w), Create(box_w), run_time=1.0)
        self.add(work_area, s_marker, s_dot, sync_line)
        lifted_y = rest_y + lift_scene
        self.play(
            s_val.animate.set_value(s_max),
            apple.animate.move_to(np.array([apple_x, lifted_y, 0.0])),
            mass_tag.animate.move_to(np.array([apple_x + 0.55, lifted_y, 0.0])),
            fg_arrow.animate.shift(UP * lift_scene),
            fg_lbl.animate.shift(UP * lift_scene),
            run_time=2.0, rate_func=smooth,
        )
        ring_f = highlight_param(items_w, "f", color=P_RED)
        ring_s = highlight_param(items_w, "s", color=P_CYAN)
        self.play(Create(ring_f), Indicate(f_line, color=P_RED), run_time=0.7)
        self.play(FadeOut(ring_f), Create(ring_s), Indicate(s_marker, color=P_CYAN), run_time=0.7)
        self.play(FadeOut(ring_s), run_time=0.25)
        hold_for(self, self.NARRATION, "work", used=1.0 + 2.0 + 0.7 + 0.7 + 0.25 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "area"))
        self.play(FadeIn(w_note), Indicate(work_area, color=P_YELLOW, scale_factor=1.03), run_time=0.9)
        hold_for(self, self.NARRATION, "area", used=0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "joule"))
        self.bring_to_front(j_lbl)
        self.play(FadeIn(j_lbl, scale=0.6), run_time=0.7)
        self.play(Indicate(j_lbl, color=P_YELLOW, scale_factor=1.2), run_time=0.8)
        hold_for(self, self.NARRATION, "joule", used=0.7 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "feel"))
        feel = VGroup(
            Text("Schokoriegel 1 m hoch ≈ 1 J", font_size=LABEL_FONT_SIZE, color=P_WHITE),
            Text("ein Herzschlag ≈ 1 J", font_size=LABEL_FONT_SIZE, color=P_WHITE),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.2).move_to(RIGHT * 4.15 + UP * 1.25)
        self.play(LaggedStart(*[FadeIn(t, shift=DOWN * 0.1) for t in feel], lag_ratio=0.4), run_time=1.2)
        hold_for(self, self.NARRATION, "feel", used=1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "stored"))
        store_tag = Text("gespeicherte Energie", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        store_tag.next_to(apple, UP, buff=0.92)
        self.play(FadeIn(store_tag), Indicate(apple, color=P_YELLOW, scale_factor=1.18), run_time=0.9)
        self.play(
            s_val.animate.set_value(0.0),
            apple.animate.move_to(np.array([apple_x, rest_y, 0.0])),
            mass_tag.animate.move_to(np.array([apple_x + 0.55, rest_y, 0.0])),
            fg_arrow.animate.shift(DOWN * lift_scene),
            fg_lbl.animate.shift(DOWN * lift_scene),
            FadeOut(store_tag), FadeOut(j_lbl), FadeOut(w_note),
            run_time=1.2, rate_func=rush_into,
        )
        self.remove(work_area, s_marker, s_dot, sync_line)
        self.play(Flash(apple, color=P_YELLOW, flash_radius=0.42), run_time=0.6)
        hold_for(self, self.NARRATION, "stored", used=0.9 + 1.2 + 0.6 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – From work to power (the watt)
class Beat3_ArbeitZuLeistung(Scene):
    NARRATION = [
        ("intro",
         "We know how much work it takes to lift a load. Power asks a different question — not how much, but how fast the work is done.",
         "Wir wissen, wie viel Arbeit ein Hub kostet. Die Leistung fragt: Wie schnell wird sie verrichtet?"),
        ("compare",
         "Two identical crates, the same weight, lifted to exactly the same shelf height. The work is identical for both. Only the time differs.",
         "Zwei gleiche Lasten, dieselbe Höhe: Die Arbeit ist identisch. Nur die Zeit ist verschieden."),
        ("fast",
         "The left crate goes up in two seconds. Same work, packed into a short time — a high rate of energy delivery.",
         "Die linke Last steigt in zwei Sekunden — dieselbe Arbeit in kurzer Zeit, hohe Leistung."),
        ("slow",
         "The right crate takes ten seconds for the very same lift. Same joules, spread thin over a long time — a low rate.",
         "Die rechte Last braucht zehn Sekunden — dieselben Joule, dünn verteilt, geringe Leistung."),
        ("formula",
         "That rate is power. Power P equals the work W divided by the time t it took. Halve the time and you double the power.",
         "Diese Rate ist die Leistung: P gleich Arbeit W geteilt durch Zeit t. Halbe Zeit, doppelte Leistung."),
        ("watt",
         "One joule delivered in exactly one second is one watt. A kilowatt is one thousand joules every second — a steady, continuous flow of energy.",
         "Ein Joule in einer Sekunde ist ein Watt. Ein Kilowatt sind 1 000 Joule pro Sekunde."),
        ("rate",
         "Power is always an instantaneous value — a snapshot. It is the speedometer of energy: it tells you the rate right now, not how far you have travelled.",
         "Leistung ist ein Momentanwert — der Tacho der Energie. Sie zeigt die Rate jetzt, nicht die zurückgelegte Strecke."),
        ("meter",
         "The distance travelled — the joules that actually added up over time — is a separate quantity. That is energy, and it is what the next beat is about.",
         "Die zurückgelegte Strecke — die aufsummierten Joule — ist eine eigene Größe: die Energie. Darum geht es als Nächstes."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Von der Arbeit zur Leistung", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        rail_bottom, rail_top = -1.35, 1.05
        lx, rx = -2.7, 2.7

        def _lift_column(x, tag_text, tag_color):
            rail = Line(np.array([x, rail_bottom, 0.0]), np.array([x, rail_top, 0.0]),
                        color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
            shelf = Line(np.array([x - 0.45, rail_top, 0.0]), np.array([x + 0.45, rail_top, 0.0]),
                         color=P_TEAL, stroke_width=3)
            floor = Line(np.array([x - 0.45, rail_bottom, 0.0]), np.array([x + 0.45, rail_bottom, 0.0]),
                         color=P_TEAL, stroke_width=3)
            crate = Square(side_length=0.5, color=tag_color, stroke_width=3,
                           fill_color=tag_color, fill_opacity=0.15)
            crate.move_to(np.array([x, rail_bottom + 0.25, 0.0]))
            tag = Text(tag_text, font_size=LABEL_FONT_SIZE, color=tag_color)
            tag.move_to(np.array([x, rail_bottom - 0.4, 0.0]))
            return {"rail": VGroup(rail, shelf, floor), "crate": crate, "tag": tag}

        left = _lift_column(lx, "schnell", P_CYAN)
        right = _lift_column(rx, "langsam", P_ORANGE)

        sw_left = _stopwatch("t = 2 s", color=P_CYAN)
        sw_left["group"].scale(0.8).move_to(np.array([lx, rail_top + 0.95, 0.0]))
        sw_right = _stopwatch("t = 10 s", color=P_ORANGE)
        sw_right["group"].scale(0.8).move_to(np.array([rx, rail_top + 0.95, 0.0]))

        same_work = chip("gleiche Arbeit W", P_YELLOW, font_size=LABEL_FONT_SIZE)
        same_work.move_to(UP * 0.15)

        eq_p, items_p = equation_row([
            ("p", "P", P_CYAN), (None, "=", P_WHITE),
            ("w", "W", P_YELLOW), (None, "/", P_WHITE), ("t", "t", P_ORANGE),
            (None, "  [W]", P_TEAL),
        ])
        eq_p, box_p = formula_panel(eq_p, color=P_CYAN)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "compare"))
        self.play(
            Create(left["rail"]), Create(right["rail"]),
            FadeIn(left["crate"]), FadeIn(right["crate"]),
            FadeIn(left["tag"]), FadeIn(right["tag"]),
            run_time=1.2,
        )
        self.play(FadeIn(sw_left["group"]), FadeIn(sw_right["group"]), FadeIn(same_work), run_time=0.8)
        hold_for(self, self.NARRATION, "compare", used=1.2 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fast"))
        self.play(
            left["crate"].animate.move_to(np.array([lx, rail_top - 0.25, 0.0])),
            Rotate(sw_left["hand"], angle=-TAU * 0.6, about_point=sw_left["face"].get_center()),
            run_time=0.9, rate_func=rush_from,
        )
        self.play(Indicate(sw_left["tag"], color=P_CYAN, scale_factor=1.2), run_time=0.6)
        hold_for(self, self.NARRATION, "fast", used=0.9 + 0.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "slow"))
        self.play(
            right["crate"].animate.move_to(np.array([rx, rail_top - 0.25, 0.0])),
            Rotate(sw_right["hand"], angle=-TAU * 1.6, about_point=sw_right["face"].get_center()),
            run_time=3.0, rate_func=linear,
        )
        hold_for(self, self.NARRATION, "slow", used=3.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq_p), Create(box_p), run_time=1.0)
        ring_t = highlight_param(items_p, "t", color=P_ORANGE)
        self.play(Create(ring_t), run_time=0.5)
        self.play(FadeOut(ring_t), run_time=0.25)
        hold_for(self, self.NARRATION, "formula", used=1.0 + 0.5 + 0.25 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "watt"))
        kw_note = Text("1 W = 1 J/s     1 kW = 1 000 J/s", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        kw_note.move_to(UP * 1.95)
        drop_y, tray_y = -0.08, -0.62
        drop = Dot(radius=0.08, color=P_YELLOW).move_to(np.array([0.0, drop_y, 0.0]))
        tray = Line(LEFT * 0.4, RIGHT * 0.4, color=P_WHITE, stroke_width=3).move_to(np.array([0.0, tray_y, 0.0]))
        self.play(FadeIn(kw_note), FadeIn(tray), run_time=0.7)
        for _ in range(3):
            d = drop.copy().move_to(np.array([0.0, drop_y, 0.0]))
            self.play(d.animate.move_to(tray.get_center() + UP * 0.1), run_time=0.35, rate_func=rush_into)
            self.play(FadeOut(d), run_time=0.12)
        hold_for(self, self.NARRATION, "watt", used=0.7 + 3 * 0.47 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rate"))
        self.play(
            FadeOut(left["rail"]), FadeOut(right["rail"]),
            FadeOut(left["crate"]), FadeOut(right["crate"]),
            FadeOut(left["tag"]), FadeOut(right["tag"]),
            FadeOut(sw_left["group"]), FadeOut(sw_right["group"]),
            FadeOut(same_work), FadeOut(tray), FadeOut(kw_note),
            run_time=0.7,
        )
        spd = _speedometer(np.array([0.0, 0.55, 0.0]), radius=0.9)
        now_lbl = Text("Momentanwert — kW jetzt", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        now_lbl.next_to(spd["face"], DOWN, buff=0.35)
        self.play(Create(spd["face"]), Create(spd["ticks"]), FadeIn(spd["needle"]), FadeIn(spd["hub"]), run_time=1.0)
        self.play(FadeIn(now_lbl), run_time=0.4)
        self.play(_speedo_rotate(spd, 0.8), run_time=0.7, rate_func=rush_from)
        self.play(_speedo_rotate(spd, 0.45), run_time=0.9)
        hold_for(self, self.NARRATION, "rate", used=1.0 + 0.4 + 0.7 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meter"))
        odo = chip("Zählerstand (kWh) = Summe über die Zeit", P_TEAL, font_size=LABEL_FONT_SIZE)
        odo.move_to(DOWN * 0.95)
        self.play(FadeIn(odo, shift=UP * 0.12), run_time=0.8)
        hold_for(self, self.NARRATION, "meter", used=0.8 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Energy = power x time, and why kilowatt-hours
class Beat4_Kilowattstunde(Scene):
    NARRATION = [
        ("question",
         "If the joule is the true unit of energy, why does every electricity bill and every building standard use the kilowatt-hour instead?",
         "Wenn das Joule die Energieeinheit ist — warum rechnen Rechnung und Norm in Kilowattstunden?"),
        ("small",
         "Because the joule is hopelessly small for a building. Filling a year of heating energy one joule at a time is like filling a reservoir with a pipette — the numbers run to twelve or thirteen digits.",
         "Weil das Joule für ein Gebäude viel zu klein ist — ein Jahresbedarf hätte zwölf, dreizehn Stellen."),
        ("formula",
         "Energy is simply power multiplied by the time it runs. E equals P times t. Hold a power for a while and you have accumulated energy.",
         "Energie ist Leistung mal Zeit: E gleich P mal t. Eine Leistung über eine Dauer ergibt Energie."),
        ("convert",
         "Take one kilowatt and let it run for one hour. That is one thousand watts times three thousand six hundred seconds — three point six million joules.",
         "Ein Kilowatt eine Stunde lang: 1 000 W mal 3 600 s — 3,6 Millionen Joule."),
        ("pack",
         "We bundle those three point six million joules into one clean package and call it one kilowatt-hour — three point six megajoules, written as a single small number.",
         "Diese 3,6 Millionen Joule bündeln wir zu einer Kilowattstunde — 3,6 Megajoule, eine handliche Zahl."),
        ("analogy",
         "Here is the picture that makes it stick. Water runs out of a pipe into an empty bucket. How far the tap is opened — how fast the water flows right now — that, and only that, is the kilowatt. It is a rate.",
         "Das Bild dazu: Wasser läuft aus einem Rohr in einen leeren Eimer. Wie weit der Hahn offen ist — wie schnell es gerade fließt — genau das ist das Kilowatt. Eine Rate."),
        ("fill",
         "Now let it run, and watch the clock. After one full hour the bucket holds a certain amount of water. That collected amount — not the speed, the total in the bucket — is the kilowatt-hour. One kilowatt flowing for one hour leaves exactly one kilowatt-hour behind.",
         "Jetzt laufen lassen und auf die Uhr schauen: Nach einer Stunde steht eine Menge Wasser im Eimer. Diese gesammelte Menge — nicht das Tempo — ist die Kilowattstunde. 1 kW · 1 h = 1 kWh."),
        ("bill",
         "This is exactly the confusion from the start. A connection rated fifteen kilowatts is the pipe. An annual consumption of eighteen thousand kilowatt-hours is the bucket. Different questions, different units.",
         "Genau die Verwechslung vom Anfang: 15 kW Anschlussleistung ist das Rohr, 18 000 kWh Jahresverbrauch der Eimer."),
        ("ladder",
         "And the bucket comes in sizes. A watt-hour charges a phone. A kilowatt-hour runs a household for a few hours. A megawatt-hour supplies a street, a gigawatt-hour a whole district.",
         "Der Eimer hat Größen: Wh lädt ein Handy, kWh trägt einen Haushalt, MWh einen Straßenzug, GWh ein Quartier."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Warum Kilowattstunden?", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "question"))
        self.play(FadeIn(caption), run_time=0.3)

        meter = _meter_dial()
        meter["group"].move_to(LEFT * 3.6 + CONTENT_CENTER)
        big_j = Text("3 600 000 J", font_size=FORMULA_FONT_SIZE, color=P_ORANGE)
        big_j.move_to(RIGHT * 0.4 + CONTENT_CENTER)
        kwh = Text("1 kWh = 3,6 MJ", font_size=FORMULA_FONT_SIZE, color=P_CYAN)
        kwh.move_to(big_j)

        eq, items = equation_row([
            ("e", "E", P_WHITE), (None, "=", P_WHITE),
            ("p", "P", P_CYAN), (None, "·", P_WHITE), ("t", "t", P_ORANGE),
            (None, "  [kWh]", P_TEAL),
        ])
        eq, box = formula_panel(eq)

        rig = _pour_rig(origin=np.array([1.35, 0.15, 0.0]))
        pipe_lbl = Text("kW = wie schnell es fließt", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        pipe_lbl.next_to(rig["pipe"], UP, buff=0.16)
        amount_lbl = Text("kWh = Menge im Eimer", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        amount_lbl.move_to(np.array([rig["rim_r"][0] + 1.7, rig["floor_y"] + 1.15, 0.0]))
        hour_note = Text("1 kW · 1 h = 1 kWh", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        hour_note.move_to(np.array([rig["spout_x"], rig["floor_y"] - 0.4, 0.0]))

        clock_c = np.array([rig["rim_l"][0] - 1.4, rig["floor_y"] + 0.80, 0.0])
        clock_face = Circle(radius=0.44, color=P_WHITE, stroke_width=2.5).move_to(clock_c)
        clock_ticks = VGroup(*[
            Line(clock_c + 0.34 * np.array([np.cos(a), np.sin(a), 0.0]),
                 clock_c + 0.42 * np.array([np.cos(a), np.sin(a), 0.0]),
                 color=P_WHITE, stroke_width=1.6)
            for a in np.linspace(0, TAU, 12, endpoint=False)
        ])
        clock_hand = Line(clock_c, clock_c + UP * 0.30, color=P_CYAN, stroke_width=3)
        clock = VGroup(clock_face, clock_ticks, clock_hand)
        clock_lbl = Text("1 Stunde", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        clock_lbl.next_to(clock_face, DOWN, buff=0.16)

        drop_path = smooth_path([
            rig["spout"],
            rig["spout"] + DOWN * 0.5,
            np.array([rig["spout_x"], rig["floor_y"] + 0.4, 0.0]),
        ])

        hold_for(self, self.NARRATION, "question", used=BEAT_SUBTITLE_FADE + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "small"))
        self.play(FadeIn(meter["group"]), run_time=0.8)
        self.play(
            Rotate(meter["needle"], angle=-TAU * 3, about_point=meter["dial"].get_center()),
            run_time=1.8, rate_func=linear,
        )
        hold_for(self, self.NARRATION, "small", used=0.8 + 1.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq), Create(box), run_time=1.0)
        for key, color in (("p", P_CYAN), ("t", P_ORANGE)):
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), run_time=0.4)
            self.play(FadeOut(ring), run_time=0.2)
        hold_for(self, self.NARRATION, "formula", used=1.0 + 2 * 0.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "convert"))
        self.play(FadeIn(big_j, shift=UP * 0.1), run_time=0.9)
        self.play(Indicate(big_j, color=P_ORANGE, scale_factor=1.12), run_time=1.0)
        hold_for(self, self.NARRATION, "convert", used=0.9 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "pack"))
        self.play(ReplacementTransform(big_j, kwh), run_time=1.2)
        self.play(Indicate(kwh, color=P_CYAN, scale_factor=1.1), run_time=0.8)
        hold_for(self, self.NARRATION, "pack", used=1.2 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "analogy"))
        self.play(
            FadeOut(meter["group"]), FadeOut(kwh),
            FadeIn(rig["pipe"]), FadeIn(rig["bucket"]), FadeIn(pipe_lbl),
            run_time=1.1,
        )
        self.add(rig["water"], rig["surface"], rig["readout"])
        # A short "tap just opened" trickle — so "kW is a rate" lands before the hour runs.
        animate_flow(self, VGroup(drop_path), P_CYAN, run_time=1.6, waves=4, cycles=1.8, streak=False)
        hold_for(self, self.NARRATION, "analogy", used=1.1 + 1.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fill"))
        self.play(FadeIn(clock), FadeIn(clock_lbl), run_time=0.7)
        # One hour of steady pour: droplets fall, the water level climbs, the hand
        # sweeps once and the kWh readout counts up together.
        animate_flow(
            self, VGroup(drop_path), P_CYAN,
            run_time=5.0, waves=8, cycles=5.0, radius=0.05, streak=False,
            extra=[
                rig["frac"].animate.set_value(1.0),
                Rotate(clock_hand, angle=-TAU, about_point=clock_c),
            ],
        )
        self.play(FadeIn(amount_lbl), FadeIn(hour_note), run_time=0.8)
        self.play(Indicate(hour_note, color=P_YELLOW, scale_factor=1.12), run_time=0.8)
        hold_for(self, self.NARRATION, "fill", used=0.7 + 5.0 + 0.8 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bill"))
        self.remove(rig["water"], rig["surface"], rig["readout"])
        self.play(
            FadeOut(rig["pipe"]), FadeOut(rig["bucket"]), FadeOut(pipe_lbl),
            FadeOut(amount_lbl), FadeOut(hour_note), FadeOut(clock), FadeOut(clock_lbl),
            run_time=0.6,
        )
        bill = VGroup(
            Text("Anschlussleistung  15 kW   → Rohr", font_size=LABEL_FONT_SIZE, color=P_CYAN),
            Text("Jahresverbrauch  18 000 kWh  → Eimer", font_size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.16)
        bill_frame = SurroundingRectangle(bill, color=P_WHITE, corner_radius=0.1, buff=0.2, stroke_width=1.6)
        bill_card = VGroup(bill_frame, bill).move_to(LEFT * 3.15 + CONTENT_CENTER)
        self.play(FadeIn(bill_card, shift=RIGHT * 0.15), run_time=1.0)
        hold_for(self, self.NARRATION, "bill", used=0.6 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ladder"))
        rungs = VGroup()
        for unit, use, color in (
            ("Wh", "Handy laden", P_WHITE),
            ("kWh", "Haushalt · Stunden", P_CYAN),
            ("MWh", "Straßenzug", P_ORANGE),
            ("GWh", "Stadtquartier", P_RED),
        ):
            u = Text(unit, font_size=BODY_FONT_SIZE, color=color)
            v = Text(use, font_size=LABEL_FONT_SIZE, color=P_WHITE)
            rungs.add(VGroup(u, v).arrange(DOWN, buff=0.1))
        rungs.arrange(RIGHT, buff=0.55)
        if rungs.width > 12.2:
            rungs.scale(12.2 / rungs.width)
        rungs.move_to(UP * 1.85)
        arrows = VGroup(*[
            Arrow(rungs[i].get_right(), rungs[i + 1].get_left(), buff=0.12, stroke_width=3, color=P_TEAL,
                  max_tip_length_to_length_ratio=0.4)
            for i in range(len(rungs) - 1)
        ])
        self.play(FadeOut(bill_card), run_time=0.4)
        self.play(LaggedStart(*[FadeIn(r, shift=UP * 0.1) for r in rungs], lag_ratio=0.2), run_time=1.4)
        self.play(LaggedStart(*[GrowArrow(a) for a in arrows], lag_ratio=0.2), run_time=1.0)
        hold_for(self, self.NARRATION, "ladder", used=0.4 + 1.4 + 1.0 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – A feel for orders of magnitude
class Beat5_Groessenordnungen(Scene):
    NARRATION = [
        ("intro",
         "A number like ten kilowatts means nothing until you can compare it to something. Architecture needs a built-in feel for scale, so let us climb a ladder of everyday power.",
         "Eine Zahl wie 10 kW sagt nichts ohne Vergleich. Steigen wir eine Leiter alltäglicher Leistungen hoch."),
        ("candle",
         "At the bottom, twenty-five watts — a single candle flame. Small, but a real, measurable heat output.",
         "Ganz unten: 25 W — eine Kerzenflamme. Klein, aber messbar."),
        ("person",
         "One hundred watts — an old incandescent bulb, and almost exactly the heat a resting adult body gives off. You are a hundred-watt heater.",
         "100 W — eine Glühbirne, und fast genau die Wärme eines ruhenden Menschen. Sie sind ein 100-Watt-Heizkörper."),
        ("fridge",
         "One hundred watts again — a fridge, averaged over its on-off cycling. Same order of magnitude, running quietly in every kitchen.",
         "Wieder 100 W — ein Kühlschrank im Mittel seiner Taktung. Dieselbe Größenordnung, in jeder Küche."),
        ("workstation",
         "Around one hundred and fifty watts — a workstation with two monitors. Fill an office floor with these and the cooling engineer starts paying attention.",
         "Rund 150 W — ein Arbeitsplatz mit zwei Monitoren. Ein ganzes Bürogeschoss davon fällt auf."),
        ("kettle",
         "Two kilowatts — an electric kettle. This is the ceiling of a normal wall socket, and twenty times a resting person.",
         "2 kW — ein Wasserkocher. Das Limit einer normalen Steckdose — das Zwanzigfache eines Menschen."),
        ("heating",
         "Around nine kilowatts — the design heating load of a well-insulated detached house in Hannover on the coldest night, near minus twelve degrees.",
         "Rund 9 kW — die Heizlast eines gut gedämmten Einfamilienhauses in Hannover bei etwa −12 °C."),
        ("hall",
         "Roughly eight kilowatts — one hundred students sitting in a lecture hall. Their bodies alone are a serious summer cooling load before a single lamp is switched on.",
         "Rund 8 kW — 100 Studierende im Hörsaal. Allein ihre Körper sind eine ernste sommerliche Kühllast."),
        ("city",
         "At the top, thirty megawatts — the urban scale. Enercity's large heat pump at Herrenhausen delivers about that, enough for roughly three thousand homes.",
         "Ganz oben: 30 MW — städtischer Maßstab. Die Großwärmepumpe Herrenhausen versorgt rund 3 000 Häuser."),
        ("close",
         "From a candle to a power station is a factor of more than a million. Keep this ladder in your head — every load in the coming videos lands somewhere on it.",
         "Von der Kerze zum Kraftwerk: Faktor über eine Million. Jede Last der nächsten Videos liegt auf dieser Leiter."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Ein Gefühl für Größenordnungen", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        rungs = [
            ("candle", "25 W", "Kerze", 25, "bulb"),
            ("person", "100 W", "Mensch (Ruhe)", 100, "bulb"),
            ("fridge", "100 W", "Kühlschrank", 100, "bulb"),
            ("workstation", "150 W", "Arbeitsplatz", 150, "laptop"),
            ("kettle", "2 kW", "Wasserkocher", 2000, "toaster"),
            ("heating", "9 kW", "Heizlast EFH Hannover", 9000, "heater"),
            ("hall", "8 kW", "Hörsaal · 100 Pers.", 8000, "toaster"),
            ("city", "30 MW", "Enercity Herrenhausen", 30000000, "heater"),
        ]

        scale_x = -5.0
        y_hi, y_lo = 2.0, -1.7
        ys = list(np.linspace(y_hi, y_lo, len(rungs)))
        scale_line = Line(np.array([scale_x, y_lo, 0.0]), np.array([scale_x, y_hi, 0.0]),
                          color=P_TEAL, stroke_width=3)
        ticks = VGroup()
        for (key, val, _n, _w, _c), y in zip(rungs, ys):
            tick = Line(np.array([scale_x, y, 0.0]), np.array([scale_x + 0.22, y, 0.0]),
                        color=P_WHITE, stroke_width=2)
            tag = Text(val, font_size=BODY_FONT_SIZE, color=P_WHITE)
            tag.next_to(tick, RIGHT, buff=0.14)
            ticks.add(VGroup(tick, tag))

        anchor_pos = RIGHT * 1.7 + UP * 0.15
        marker = Dot(np.array([scale_x, ys[0], 0.0]), radius=0.09, color=P_YELLOW)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)
        self.play(
            Create(scale_line),
            LaggedStart(*[FadeIn(t) for t in ticks], lag_ratio=0.12),
            FadeIn(marker),
            run_time=1.6,
        )

        prev = None
        for (key, val, name, watts, comp), y in zip(rungs, ys):
            badge = watt_anchor(watts, compare=comp, title=name).scale(1.12)
            anchor = badge[1]
            anchor.move_to(anchor_pos)
            extras = []
            if key == "heating":
                house = _build_cross_section_house(center=anchor_pos + LEFT * 2.55)
                extras = [house["group"].scale(0.42).move_to(anchor_pos + LEFT * 2.55)]

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            anims = [
                marker.animate.move_to(np.array([scale_x, y, 0.0])),
                FadeIn(anchor, shift=LEFT * 0.15),
            ]
            anims += [FadeIn(e) for e in extras]
            if prev is not None:
                anims.insert(0, FadeOut(prev))
            self.play(*anims, run_time=1.1)
            hold_for(self, self.NARRATION, key, used=1.1 + 0.35)
            prev = VGroup(anchor, *extras) if extras else anchor

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "close"))
        span = dim_arrow(np.array([scale_x - 0.35, y_lo, 0.0]), np.array([scale_x - 0.35, y_hi, 0.0]), color=P_YELLOW)
        span_lbl = Text("× 1 000 000+", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        span_lbl.rotate(PI / 2).next_to(span, LEFT, buff=0.1)
        self.play(FadeIn(span), FadeIn(span_lbl), run_time=0.9)
        hold_for(self, self.NARRATION, "close", used=0.9 + 0.35)

        self.play(FadeOut(caption), FadeOut(prev), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – First law of thermodynamics
class Beat6_Energieerhaltung(Scene):
    NARRATION = [
        ("law",
         "Every heating and cooling calculation rests on one law. Energy is never created and never destroyed. It only changes form. The sum of all energy in a closed system stays constant.",
         "Jede Heiz- und Kühlrechnung ruht auf einem Satz: Energie entsteht nicht und vergeht nicht — sie wandelt nur ihre Form. Die Summe bleibt konstant."),
        ("input",
         "Feed one hundred watts of electricity into a lamp. One hundred percent goes in as electrical power. The law says one hundred percent must come out again, in some form.",
         "100 W Strom in eine Lampe: 100 % gehen hinein — 100 % müssen wieder heraus, in irgendeiner Form."),
        ("split",
         "A little of it leaves as visible light — maybe five watts for an old bulb. The remaining ninety-five watts leave immediately as heat: warm glass, warm air around it.",
         "Ein kleiner Teil wird sichtbares Licht — bei der Glühbirne etwa 5 W. Die übrigen 95 W sind sofort Wärme."),
        ("sankey",
         "Follow the flow. The input bar forks: a thin lane of light, a wide lane of heat. Nothing is lost at the fork — the two lanes still add back to one hundred watts.",
         "Der Eingangsbalken teilt sich: eine schmale Spur Licht, eine breite Spur Wärme. Zusammen wieder 100 W."),
        ("walls",
         "And the light does not escape either. It travels a few metres, strikes the walls, the floor, the furniture — and is absorbed. At each surface it becomes heat as well.",
         "Auch das Licht entkommt nicht: Es trifft Wände, Boden, Möbel — wird absorbiert und ebenfalls zu Wärme."),
        ("merge",
         "So the two lanes rejoin. A short time later, essentially all one hundred watts of electricity have become heat inside the room. Light was just a brief detour.",
         "Die Spuren vereinen sich wieder: Kurz darauf sind praktisch alle 100 W Strom zu Raumwärme geworden."),
        ("building",
         "This is the rule an architect carries everywhere. Almost every watt of electricity used inside a building — lights, computers, appliances, people — ends up as a thermal load on that building.",
         "Die Regel für Architekten: Fast jedes Watt Strom im Gebäude — Licht, Rechner, Geräte, Menschen — endet als thermische Last."),
        ("seasons",
         "In January that internal heat is a welcome gift that lowers the heating demand. In July it is an unwelcome load the cooling system has to fight. Same watts, opposite sign.",
         "Im Januar ist diese innere Wärme ein Gewinn, im Juli eine Last. Dieselben Watt, umgekehrtes Vorzeichen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Erster Hauptsatz — Energie geht nie verloren", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "law"))
        self.play(FadeIn(caption), run_time=0.3)

        eq, items = equation_row([
            (None, "Σ E", P_WHITE), (None, "=", P_WHITE),
            (None, "konstant", P_TEAL),
        ])
        eq, box = formula_panel(eq, color=P_TEAL)
        law_note = Text("1. Hauptsatz der Thermodynamik", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        law_note.next_to(box, UP, buff=0.16)

        hold_for(self, self.NARRATION, "law", used=BEAT_SUBTITLE_FADE + 0.3)
        self.play(FadeIn(eq), Create(box), FadeIn(law_note), run_time=1.0)
        hold_for(self, self.NARRATION, "law", used=1.0)

        split = _energy_split(-3.9, 0.35, total_w=3.0, height=1.0)
        in_lbl = Text("100 % Strom", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        in_lbl.next_to(split["src"], DOWN, buff=0.16)
        light_lbl = Text("≈ 5 % Licht", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        light_lbl.next_to(split["light"], UP, buff=0.12)
        heat_lbl = Text("≈ 95 % Wärme", font_size=LABEL_FONT_SIZE, color=P_RED)
        heat_lbl.next_to(split["heat"], DOWN, buff=0.12)
        out_lbl = Text("100 % Wärme", font_size=LABEL_FONT_SIZE, color=P_RED)
        out_lbl.next_to(split["sink"], UP, buff=0.24)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "input"))
        self.play(FadeIn(split["src"]), FadeIn(in_lbl), run_time=0.9)
        hold_for(self, self.NARRATION, "input", used=0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "split"))
        self.play(
            GrowFromEdge(split["light"], LEFT), GrowFromEdge(split["heat"], LEFT),
            FadeIn(light_lbl), FadeIn(heat_lbl),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "split", used=1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sankey"))
        self.play(FadeIn(split["sink"]), FadeIn(out_lbl), run_time=0.9)
        self.play(Indicate(split["group"], color=P_TEAL, scale_factor=1.03), run_time=0.9)
        hold_for(self, self.NARRATION, "sankey", used=0.9 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "walls"))
        wall = Line(split["sink"].get_right() + RIGHT * 1.15 + UP * 0.45,
                    split["sink"].get_right() + RIGHT * 1.15 + DOWN * 0.75,
                    color=P_WHITE, stroke_width=4)
        ray_origin = split["sink"].get_right() + RIGHT * 0.12 + DOWN * 0.15
        ray = radiation_waves(ray_origin, n=2, color=P_YELLOW, height=0.85)
        ray.rotate(-PI / 2, about_point=ray_origin)
        self.play(Create(wall), LaggedStart(*[Create(r) for r in ray], lag_ratio=0.2), run_time=1.0)
        self.play(Indicate(wall, color=P_ORANGE, scale_factor=1.05), ray.animate.set_color(P_RED), run_time=1.0)
        hold_for(self, self.NARRATION, "walls", used=1.0 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "merge"))
        self.play(
            split["light"].animate.set_fill(P_RED, opacity=0.30).set_stroke(P_RED),
            FadeOut(light_lbl), FadeOut(ray),
            heat_lbl.animate.set_color(P_RED),
            run_time=1.2,
        )
        self.play(Indicate(out_lbl, color=P_RED, scale_factor=1.12), run_time=0.8)
        hold_for(self, self.NARRATION, "merge", used=1.2 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "building"))
        # House sits low and centred so the header clears the subtitle above and the
        # side labels sit in open margin left and right — nothing stacked on anything.
        house = _build_cross_section_house(center=ORIGIN)
        house["group"].scale(0.60).move_to(DOWN * 0.85)
        hc = house["group"].get_center()
        h_top = house["group"].get_top()
        h_left_x = house["group"].get_left()[0]
        h_right_x = house["group"].get_right()[0]

        internal_chip = chip("interne Wärme", P_ORANGE, font_size=LABEL_FONT_SIZE)
        internal_note = Text("Licht · Geräte · Menschen", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        internal_hdr = VGroup(internal_chip, internal_note).arrange(DOWN, buff=0.16)
        internal_hdr.next_to(h_top, UP, buff=0.55).set_x(0)
        internal_marks = VGroup(*[
            Dot(hc + np.array([ox, oy, 0.0]), radius=0.055, color=P_ORANGE)
            for ox, oy in ((-0.34, 0.06), (0.10, -0.16), (0.42, 0.14))
        ])
        self.play(
            FadeOut(split["group"]), FadeOut(in_lbl), FadeOut(heat_lbl), FadeOut(out_lbl), FadeOut(wall),
            FadeOut(eq), FadeOut(box), FadeOut(law_note),
            FadeIn(house["group"]), FadeIn(internal_hdr), FadeIn(internal_marks),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "building", used=1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "seasons"))
        # —— Winter: the internal heat is a welcome gift (left margin) ——
        season_chip_pos = np.array([h_left_x - 2.5, 1.4, 0.0])
        winter_chip = chip("Januar · −12 °C", P_BLUE, font_size=LABEL_FONT_SIZE)
        winter_chip.move_to(season_chip_pos)
        loss_arrows = VGroup(*[
            Arrow(
                np.array([h_left_x - 0.10, hc[1] + dy, 0.0]),
                np.array([h_left_x - 1.55, hc[1] + dy, 0.0]),
                color=P_BLUE, buff=0.0, stroke_width=3,
                max_tip_length_to_length_ratio=0.16,
            )
            for dy in (-0.5, 0.0, 0.5)
        ])
        gift_lbl = Text("Gewinn — spart Heizung", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        gift_lbl.move_to(np.array([h_left_x - 2.3, hc[1] - 1.0, 0.0]))
        self.play(
            FadeIn(winter_chip),
            LaggedStart(*[GrowArrow(a) for a in loss_arrows], lag_ratio=0.2),
            run_time=1.0,
        )
        animate_haze(
            self, run_time=1.6, cycles=1.2,
            x0=h_left_x - 0.15, x1=h_left_x - 1.9,
            y0=hc[1] - 0.75, y1=hc[1] + 0.75,
            color=P_BLUE, color_end="#6A9FD4", n=22, seed=3,
        )
        self.play(FadeIn(gift_lbl), Indicate(internal_marks, color=P_GREEN, scale_factor=1.2), run_time=0.9)

        # —— Summer: the same internal heat is now an extra load (right margin) ——
        summer_chip = chip("Juli · Sonne", P_YELLOW, font_size=LABEL_FONT_SIZE)
        summer_chip.move_to(season_chip_pos)
        sun = _build_sun(ORIGIN, color=P_YELLOW).scale(0.5)
        sun.move_to(np.array([h_right_x + 2.35, 1.45, 0.0]))
        load_lbl = Text("Last — mehr Kühlung", font_size=LABEL_FONT_SIZE, color=P_RED)
        load_lbl.move_to(np.array([h_right_x + 2.3, hc[1] - 0.1, 0.0]))
        self.play(
            ReplacementTransform(winter_chip, summer_chip),
            FadeOut(gift_lbl), FadeOut(loss_arrows),
            FadeIn(sun),
            run_time=1.0,
        )
        animate_haze(
            self, run_time=1.8, cycles=1.3,
            x0=hc[0] - 0.7, x1=hc[0] + 0.7,
            y0=hc[1] - 0.45, y1=hc[1] + 0.9,
            color=P_RED, color_end="#C9786E", n=24, seed=11,
        )
        self.play(
            FadeIn(load_lbl),
            Indicate(internal_marks, color=P_RED, scale_factor=1.2),
            Indicate(house["group"], color=P_RED, scale_factor=1.04),
            run_time=0.9,
        )
        hold_for(self, self.NARRATION, "seasons", used=1.0 + 1.6 + 0.9 + 1.0 + 1.8 + 0.9 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 – The heat-pump trick (COP)
class Beat7_Waermepumpe(Scene):
    NARRATION = [
        ("shift",
         "The first law seems to cap every heater at one hundred percent. Heat pumps look like they break that rule — and understanding why they do not is the most useful idea in this whole module.",
         "Der erste Hauptsatz scheint jede Heizung bei 100 % zu deckeln. Wärmepumpen wirken wie ein Bruch dieser Regel — zu Unrecht."),
        ("conserve",
         "The trick is that a heat pump does not create heat. It moves heat that already exists in the cold outdoor air into the warm building, and it spends some electricity to do the moving.",
         "Der Trick: Eine Wärmepumpe erzeugt keine Wärme. Sie verschiebt vorhandene Wärme aus der kalten Außenluft ins Haus — mit etwas Stromaufwand."),
        ("inputs",
         "One kilowatt-hour of electricity drives the compressor. That work pulls roughly three kilowatt-hours of ambient heat out of the outside air, even at a few degrees Celsius.",
         "1 kWh Strom treibt den Verdichter. Diese Arbeit holt rund 3 kWh Umgebungswärme aus der Außenluft."),
        ("output",
         "Add them: one kilowatt-hour of electricity plus three kilowatt-hours of ambient heat leave the machine as four kilowatt-hours of useful heat delivered to the rooms.",
         "Zusammen: 1 kWh Strom plus 3 kWh Umweltwärme ergeben 4 kWh nutzbare Heizwärme im Gebäude."),
        ("ledger",
         "The books balance perfectly. One plus three equals four. No energy was invented — the electricity only paid for the transport of heat that was already out there.",
         "Die Bilanz stimmt: 1 plus 3 gleich 4. Nichts wurde erfunden — der Strom bezahlt nur den Transport."),
        ("cop",
         "The ratio of what you get to what you pay is the coefficient of performance, the COP. It is the useful heat output divided by the electrical input.",
         "Das Verhältnis von Nutzen zu Aufwand ist der COP: nutzbare Heizwärme geteilt durch Stromaufwand."),
        ("value",
         "Four kilowatt-hours out for one kilowatt-hour of electricity in is a COP of four. Every unit of electricity is multiplied fourfold — the best translation of power into heat we have.",
         "4 kWh Nutzen je 1 kWh Strom ist ein COP von 4 — jede Einheit Strom vervierfacht sich."),
        ("subs",
         "This is why heat-pump data sheets are full of little subscripts. kW with a small e-l is the electrical drive power you pay for at the meter. kW with a small t-h is the thermal heating power that actually warms the house. A data sheet might list two point five kilowatts electrical and ten kilowatts thermal — the same factor of four.",
         "kW_el = elektrische Antriebsleistung (bezahlt), kW_th = thermische Heizleistung fürs Haus."),
        ("contrast",
         "Compare a plain resistance heater: one kilowatt-hour of electricity gives exactly one kilowatt-hour of heat. Its COP is one. The heat pump does the same job with a quarter of the electricity.",
         "Zum Vergleich der Heizstab: 1 kWh Strom ergibt genau 1 kWh Wärme — COP gleich 1. Die Wärmepumpe braucht ein Viertel."),
        ("jaz",
         "Averaged over a whole Hannover heating season, with cold spells and defrost cycles, a real air-source unit lands nearer a COP of three — reported as the seasonal performance factor, the JAZ.",
         "Übers Jahr gemittelt — mit Kälteperioden und Abtauen — liegt eine Luft-Wärmepumpe eher bei COP 3: die Jahresarbeitszahl JAZ."),
        ("primary",
         "One last distinction to keep in your pocket. The kilowatt-hour you pull from the socket is called final energy. How much coal, gas or wind the power station had to spend to deliver it is the primary energy — but that belongs in the next video.",
         "Strom aus der Steckdose = Endenergie. Der Aufwand im Kraftwerk = Primärenergie — mehr im nächsten Video."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Der Wärmepumpen-Trick (COP)", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "shift"))
        self.play(FadeIn(caption), run_time=0.3)

        pump, box = _heat_pump_box()
        pump.move_to(CONTENT_CENTER + UP * 0.1)

        plug = Text("1 kWh Strom", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        plug.next_to(box, LEFT, buff=0.7)
        ambient = Text("3 kWh Umweltwärme", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        ambient.next_to(box, UP, buff=0.55)
        heat_out = Text("4 kWh Heizwärme", font_size=LABEL_FONT_SIZE, color=P_RED)
        heat_out.next_to(box, RIGHT, buff=0.7)

        in_e = Arrow(plug.get_right(), box.get_left(), color=P_CYAN, buff=0.1, stroke_width=3)
        in_a = Arrow(ambient.get_bottom(), box.get_top(), color=P_GREEN, buff=0.1, stroke_width=3)
        out_h = Arrow(box.get_right(), heat_out.get_left(), color=P_RED, buff=0.1, stroke_width=4)

        eq, items = equation_row([
            (None, "COP", P_TEAL), (None, "=", P_WHITE),
            ("q", "Q_ab", P_RED), (None, "/", P_WHITE),
            ("w", "W_zu", P_CYAN),
            (None, "  = 4,0", P_YELLOW),
        ])
        eq, eq_box = formula_panel(eq, color=P_TEAL)

        hold_for(self, self.NARRATION, "shift", used=BEAT_SUBTITLE_FADE + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "conserve"))
        self.play(FadeIn(pump), run_time=0.9)
        hold_for(self, self.NARRATION, "conserve", used=0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "inputs"))
        self.play(GrowArrow(in_e), FadeIn(plug), run_time=0.8)
        self.play(GrowArrow(in_a), FadeIn(ambient), run_time=0.8)
        hold_for(self, self.NARRATION, "inputs", used=0.8 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "output"))
        self.play(GrowArrow(out_h), FadeIn(heat_out), run_time=1.0)
        hold_for(self, self.NARRATION, "output", used=1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ledger"))
        ledger = VGroup(
            Text("Strom      1", font_size=LABEL_FONT_SIZE, color=P_CYAN),
            Text("Umwelt   + 3", font_size=LABEL_FONT_SIZE, color=P_GREEN),
            Line(LEFT * 0.6, RIGHT * 0.6, color=P_WHITE, stroke_width=2),
            Text("Wärme      4", font_size=LABEL_FONT_SIZE, color=P_RED),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.12)
        ledger_frame = SurroundingRectangle(ledger, color=P_WHITE, corner_radius=0.1, buff=0.2, stroke_width=1.6)
        ledger_card = VGroup(ledger_frame, ledger).move_to(LEFT * 4.3 + DOWN * 0.7)
        self.play(FadeIn(ledger_card, shift=RIGHT * 0.15), run_time=1.0)
        hold_for(self, self.NARRATION, "ledger", used=1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cop"))
        self.play(FadeIn(eq), Create(eq_box), run_time=1.1)
        for key, color in (("q", P_RED), ("w", P_CYAN)):
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), run_time=0.4)
            self.play(FadeOut(ring), run_time=0.2)
        hold_for(self, self.NARRATION, "cop", used=1.1 + 2 * 0.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "value"))
        self.play(Indicate(items["q"], color=P_RED, scale_factor=1.15),
                  Indicate(heat_out, color=P_RED), run_time=1.0)
        hold_for(self, self.NARRATION, "value", used=1.0 + 0.35)

        # —— el vs. th: what the data sheet's subscripts mean ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "subs"))
        el_tag = Text("kW_el", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(plug, DOWN, buff=0.14)
        th_tag = Text("kW_th", font_size=LABEL_FONT_SIZE, color=P_RED).next_to(heat_out, DOWN, buff=0.14)
        datasheet = Text("Datenblatt: 2,5 kW_el → 10 kW_th", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        datasheet.move_to(CONTENT_CENTER + DOWN * 1.15)
        subs_extra = VGroup(el_tag, th_tag, datasheet)
        self.play(FadeIn(el_tag), Indicate(in_e, color=P_CYAN, scale_factor=1.12), run_time=0.7)
        self.play(FadeIn(th_tag), Indicate(out_h, color=P_RED, scale_factor=1.12), run_time=0.7)
        self.play(FadeIn(datasheet, shift=UP * 0.1), run_time=0.7)
        hold_for(self, self.NARRATION, "subs", used=0.7 + 0.7 + 0.7 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "contrast"))
        self.play(
            FadeOut(ledger_card), FadeOut(subs_extra),
            FadeOut(plug), FadeOut(ambient), FadeOut(heat_out),
            FadeOut(in_e), FadeOut(in_a), FadeOut(out_h),
            pump.animate.scale(0.72).move_to(LEFT * 3.25 + CONTENT_CENTER + UP * 0.1),
            run_time=1.0,
        )
        wp_in = Text("1 kWh", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(pump, LEFT, buff=0.3)
        wp_out = Text("4 kWh", font_size=LABEL_FONT_SIZE, color=P_RED).next_to(pump, RIGHT, buff=0.3)
        wp_cop = Text("COP = 4", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(pump, DOWN, buff=0.3)
        heater, hbox = _resistance_heater()
        heater.scale(0.85).move_to(RIGHT * 3.25 + CONTENT_CENTER + UP * 0.1)
        h_in = Text("1 kWh", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(hbox, LEFT, buff=0.3)
        h_out = Text("1 kWh", font_size=LABEL_FONT_SIZE, color=P_RED).next_to(hbox, RIGHT, buff=0.3)
        h_cop = Text("COP = 1", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(heater, DOWN, buff=0.3)
        self.play(
            FadeIn(wp_in), FadeIn(wp_out), FadeIn(wp_cop),
            FadeIn(heater), FadeIn(h_in), FadeIn(h_out), FadeIn(h_cop),
            run_time=1.1,
        )
        hold_for(self, self.NARRATION, "contrast", used=1.0 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "jaz"))
        jaz = chip("übers Jahr: JAZ ≈ 3 (Hannover)", P_TEAL, font_size=LABEL_FONT_SIZE)
        jaz.move_to(UP * 1.95)
        self.play(FadeIn(jaz, shift=DOWN * 0.12), run_time=0.9)
        hold_for(self, self.NARRATION, "jaz", used=0.9 + 0.35)

        # —— Closing teaser: Endenergie vs. Primärenergie (a seed for the DIN V 18599 video) ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "primary"))
        self.play(
            FadeOut(pump), FadeOut(wp_in), FadeOut(wp_out), FadeOut(wp_cop),
            FadeOut(heater), FadeOut(h_in), FadeOut(h_out), FadeOut(h_cop),
            FadeOut(eq), FadeOut(eq_box), FadeOut(jaz),
            run_time=0.7,
        )
        plant = chip("Kraftwerk", P_ORANGE, font_size=LABEL_FONT_SIZE)
        socket = chip("Steckdose", P_CYAN, font_size=LABEL_FONT_SIZE)
        consumer = chip("Gebäude", P_RED, font_size=LABEL_FONT_SIZE)
        chain = VGroup(plant, socket, consumer).arrange(RIGHT, buff=1.15).move_to(CONTENT_CENTER)
        a1 = Arrow(plant.get_right(), socket.get_left(), buff=0.12, color=P_ORANGE, stroke_width=3)
        a2 = Arrow(socket.get_right(), consumer.get_left(), buff=0.12, color=P_CYAN, stroke_width=3)
        prim_lbl = Text("Primärenergie", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(a1, UP, buff=0.16)
        end_lbl = Text("Endenergie", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(a2, UP, buff=0.16)
        next_lbl = Text("→ nächstes Video", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(chain, DOWN, buff=0.65)
        self.play(
            LaggedStart(
                FadeIn(plant), GrowArrow(a1), FadeIn(socket), GrowArrow(a2), FadeIn(consumer),
                lag_ratio=0.3,
            ),
            run_time=1.6,
        )
        self.play(FadeIn(prim_lbl), FadeIn(end_lbl), run_time=0.6)
        self.play(FadeIn(next_lbl), run_time=0.5)
        hold_for(self, self.NARRATION, "primary", used=0.7 + 1.6 + 0.6 + 0.5 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – Outlook to the heating and cooling videos
class Beat8_Ausblick(Scene):
    NARRATION = [
        ("recap",
         "One picture holds the whole module together. The kilowatt is the pipe — the rate energy flows at a given instant. The kilowatt-hour is the bucket — the amount that has collected over time.",
         "Ein Bild trägt das Modul: Das Kilowatt ist das Rohr — die Rate. Die Kilowattstunde ist der Eimer — die Menge."),
        ("cheat",
         "And the four units line up. Newtons for force. Joules for work and energy. Watts, which are joules per second, for power. Kilowatt-hours for the energy amounts a building actually deals in.",
         "Die vier Einheiten: Newton für Kraft, Joule für Energie, Watt gleich Joule pro Sekunde für Leistung, Kilowattstunde für den Alltag."),
        ("left",
         "The heating videos first size the pipe. The heating load, in kilowatts, is the worst-case rate the system must deliver on the coldest hour — computed after DIN EN 12831.",
         "Die Heiz-Videos dimensionieren zuerst das Rohr: die Heizlast in kW für die kälteste Stunde — nach DIN EN 12831."),
        ("right",
         "Then they size the bucket. The annual heating demand, in kilowatt-hours per square metre and year, is the total energy the building consumes across a whole heating season — after DIN V 18599.",
         "Dann den Eimer: den Jahres-Heizwärmebedarf in kWh/m²a über die ganze Heizperiode — nach DIN V 18599."),
        ("perarea",
         "And why divide the whole thing by the square metres of floor area at the end? So that any two buildings can be compared. Only per square metre can we tell whether a small villa or a large apartment block is the more efficient one.",
         "Warum am Ende durch die Quadratmeter? So lassen sich Gebäude vergleichen — kleine Villa oder großer Wohnblock."),
        ("both",
         "Pipe and bucket, rate and amount, power and energy — you will need both numbers for every project, and now you know exactly what each one means.",
         "Rohr und Eimer, Rate und Menge, Leistung und Energie — beide Zahlen braucht jedes Projekt, und ihr wisst jetzt, was sie bedeuten."),
        ("cta",
         "In the next video we step into the mathematics of the building envelope and start turning walls, windows and roofs into watts.",
         "Im nächsten Video geht es in die Mathematik der Gebäudehülle — Wände, Fenster, Dächer werden zu Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Ausblick — die nächsten Videos", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "recap"))
        self.play(FadeIn(caption), run_time=0.3)

        divider = Line(UP * 1.35, DOWN * 1.7, color=P_TEAL, stroke_width=2)

        rig = _pour_rig(origin=np.array([0.0, 0.30, 0.0]))
        pipe_icon = Text("kW — Rate", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        pipe_icon.next_to(rig["pipe"], UP, buff=0.16)
        bucket_icon = Text("kWh — Menge", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        bucket_icon.next_to(
            np.array([rig["rim_r"][0] + 1.15, rig["floor_y"] + rig["bh"] * 0.45, 0.0]),
            RIGHT, buff=0.18,
        )
        pb_group = VGroup(rig["pipe"], rig["bucket"])
        pb_all = VGroup(pb_group, pipe_icon, bucket_icon)
        drop_path = smooth_path([
            rig["spout"],
            rig["spout"] + DOWN * 0.48,
            np.array([rig["spout_x"], rig["floor_y"] + 0.38, 0.0]),
        ])

        winter = _build_cross_section_house(center=ORIGIN)
        winter["group"].scale(0.58).move_to(LEFT * 3.4 + DOWN * 0.05)
        w_ctr = winter["group"].get_center()
        winter_tag = Text("kälteste Stunde · −12 °C", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        winter_tag.next_to(winter["group"], UP, buff=0.22).set_x(w_ctr[0])
        left_lbl = Text("Heizlast (kW)\nDIN EN 12831", font_size=LABEL_FONT_SIZE,
                        color=P_RED, line_spacing=0.9)
        left_lbl.next_to(winter["group"], DOWN, buff=0.3).set_x(w_ctr[0])
        winter_all = VGroup(winter["group"], winter_tag, left_lbl)

        year = _build_cross_section_house(center=ORIGIN)
        year["group"].scale(0.58).move_to(RIGHT * 3.4 + DOWN * 0.05)
        y_ctr = year["group"].get_center()
        seasons = VGroup(
            chip("Frühling", P_GREEN, font_size=LABEL_FONT_SIZE),
            chip("Sommer", P_YELLOW, font_size=LABEL_FONT_SIZE),
            chip("Herbst", P_ORANGE, font_size=LABEL_FONT_SIZE),
            chip("Winter", P_BLUE, font_size=LABEL_FONT_SIZE),
        ).arrange(RIGHT, buff=0.1).scale(0.7)
        seasons.next_to(year["group"], UP, buff=0.22).set_x(y_ctr[0])
        right_lbl = Text("Heizwärmebedarf (kWh/m²a)\nDIN V 18599", font_size=LABEL_FONT_SIZE,
                         color=P_CYAN, line_spacing=0.9)
        right_lbl.next_to(year["group"], DOWN, buff=0.3).set_x(y_ctr[0])
        year_all = VGroup(year["group"], seasons, right_lbl)

        hold_for(self, self.NARRATION, "recap", used=BEAT_SUBTITLE_FADE + 0.3)
        self.play(FadeIn(pb_group), FadeIn(pipe_icon), FadeIn(bucket_icon), run_time=1.0)
        self.add(rig["water"], rig["surface"], rig["readout"])
        animate_flow(
            self, VGroup(drop_path), P_CYAN,
            run_time=2.8, waves=6, cycles=2.8, radius=0.05, streak=False,
            extra=[rig["frac"].animate.set_value(0.82)],
        )
        hold_for(self, self.NARRATION, "recap", used=1.0 + 2.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cheat"))
        cheat = _unit_cheatsheet()
        cheat.next_to(pb_group, DOWN, buff=0.55).set_x(0)
        self.play(FadeIn(cheat, shift=DOWN * 0.12), run_time=1.2)
        hold_for(self, self.NARRATION, "cheat", used=1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "left"))
        self.remove(rig["water"], rig["surface"], rig["readout"])
        self.play(
            FadeOut(cheat), FadeOut(pb_all),
            FadeOut(rig["pipe"]), FadeOut(rig["bucket"]),
            Create(divider), run_time=0.7,
        )
        self.play(FadeIn(winter_all), run_time=1.3)
        hold_for(self, self.NARRATION, "left", used=0.7 + 1.3 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "right"))
        self.play(FadeIn(year_all), run_time=1.3)
        hold_for(self, self.NARRATION, "right", used=1.3 + 0.35)

        # —— Why per m²: normalise a small and a large building to the same yardstick ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "perarea"))
        area_chip = chip("÷ Fläche → Gebäude vergleichbar", P_CYAN, font_size=LABEL_FONT_SIZE)
        area_chip.move_to(RIGHT * 3.4 + UP * 2.05)
        col_x = 6.05
        villa = Square(side_length=0.42, color=P_GREEN, stroke_width=2.5).move_to(np.array([col_x, 0.62, 0.0]))
        villa_lbl = Text("Villa", font_size=LABEL_FONT_SIZE, color=P_GREEN).next_to(villa, UP, buff=0.12)
        block = Square(side_length=0.9, color=P_ORANGE, stroke_width=2.5).move_to(np.array([col_x, -0.78, 0.0]))
        block_lbl = Text("Block", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(block, DOWN, buff=0.12)
        eqmark = Text("=", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to(np.array([col_x, -0.08, 0.0]))
        per_extra = VGroup(area_chip, villa, villa_lbl, block, block_lbl, eqmark)
        self.play(
            FadeIn(area_chip),
            FadeIn(villa), FadeIn(villa_lbl), FadeIn(block), FadeIn(block_lbl),
            run_time=1.0,
        )
        self.play(
            block.animate.scale(0.42 / 0.9),
            Indicate(right_lbl, color=P_CYAN, scale_factor=1.06),
            run_time=0.9,
        )
        self.play(FadeIn(eqmark), run_time=0.4)
        hold_for(self, self.NARRATION, "perarea", used=1.0 + 0.9 + 0.4 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "both"))
        self.play(
            FadeOut(per_extra),
            Indicate(winter_all, color=P_CYAN, scale_factor=1.06),
            Indicate(year_all, color=P_TEAL, scale_factor=1.06),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "both", used=1.4 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cta"))
        cta = chip("Nächstes Video ▶", P_YELLOW, font_size=BODY_FONT_SIZE)
        cta.move_to(UP * 1.9)
        self.play(FadeIn(cta, scale=0.9), Indicate(cta, color=P_YELLOW), run_time=1.2)
        hold_for(self, self.NARRATION, "cta", used=1.2 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
