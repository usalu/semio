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
    lamp_glyph, clock_glyph,
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


def _stream(path, rate, *, color, color_end=None, count: int = 5, speed: float = 0.45, radius: float = 0.05):
    """💧 Particles that keep travelling along ``path`` at a speed set by the ``rate`` tracker.

    Animating the tracker speeds the stream up, slows it down or stops it while the drops keep moving.
    """
    start_c, end_c = ManimColor(color), ManimColor(color_end or color)
    drops = VGroup(*[Dot(radius=radius, stroke_width=0) for _ in range(count)])
    state = {"phase": 0.0}

    def advance(group, dt):
        value = float(rate.get_value())
        state["phase"] = (state["phase"] + dt * speed * value) % 1.0
        show = float(np.clip(value * 5.0, 0.0, 1.0))
        for i, drop in enumerate(group):
            t = (state["phase"] + i / count) % 1.0
            drop.move_to(path.point_from_proportion(t))
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


