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
    beat_subtitle, BEAT_SUBTITLE_FADE, centered_body_text,
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, radiation_waves, solar_wave_ray, convection_stream, respiration_parts,
    smooth_path, animate_flows, animate_haze, meter, set_meter, side_labels,
    math_text, math_row, formula_panel, highlight_param, dim_arrow,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing, begin_vo_beat,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Physikalische Grundlagen"
PANEL_MAX_WIDTH = 11.6


#region Shared helpers
#region Numbers and typeset labels
def _de(value: float, digits: int = 0) -> str:
    """🔢 German number for ``math_text``: decimal comma, thin-space thousands."""
    return f"{value:,.{digits}f}".replace(",", r"\,").replace(".", "{,}")


def _place(mob, at, edge: str = "center"):
    """📌 Sit a typeset box on a baseline point; ``edge`` picks which side lands on ``at``."""
    at = np.array(at, dtype=float)
    x = {"left": mob.get_left()[0], "right": mob.get_right()[0], "center": mob.get_center()[0]}[edge]
    mob.shift(np.array([at[0] - x, at[1] - mob.base.get_center()[1], 0.0]))
    return mob


def _m(src: str, at=None, *, size: float = BODY_FONT_SIZE, color: str = P_WHITE, edge: str = "center"):
    """∑ Typeset label, optionally placed on a baseline point."""
    mob = math_text(src, font_size=size, color=color)
    return _place(mob, at, edge) if at is not None else mob


def _readout(fn, at, *, size: float = BODY_FONT_SIZE, color: str = P_WHITE, edge: str = "left"):
    """📟 Live label redrawn every frame from ``fn()`` — the number is read off the geometry."""
    return always_redraw(lambda: _m(fn(), at, size=size, color=color, edge=edge))


def _panel(parts, *, color: str = P_TEAL, size: float = FORMULA_FONT_SIZE):
    """🧮 Typeset formula in the fixed bottom panel, shrunk only when it would overrun the frame."""
    row, items = math_row(parts, font_size=size, buff=0.24)
    if row.width > PANEL_MAX_WIDTH:
        row.scale(PANEL_MAX_WIDTH / row.width)
    row, box = formula_panel(row, color=color)
    return row, box, items


def _fill_panel(scene, panel, sources, *, run_time: float = 1.2, fly: bool = True):
    """🧲 Bring source values into their empty panel slots first, then reveal the rest of the formula and its box.

    ``fly`` morphs a copy along a clear path; without it the source pulses while its slot appears in the
    same colour — used whenever a straight flight would cross the drawing.
    """
    row, box, items = panel
    targets = [items[key] for key in sources]
    rest = VGroup(*[m for m in row.submobjects if all(m is not t for t in targets)])
    if fly:
        copies = {key: src.copy() for key, src in sources.items()}
        for copy in copies.values():
            for part in copy.get_family():
                part._layout_zone = "formula"
        scene.play(*[copies[key].animate.move_to(items[key]) for key in sources], run_time=run_time * 0.7)
        scene.play(*[FadeTransform(copies[key], items[key]) for key in sources], run_time=run_time * 0.4)
    else:
        scene.play(*[Indicate(src, scale_factor=1.15) for src in sources.values()],
                   *[FadeIn(items[key], scale=1.3) for key in sources], run_time=run_time)
    scene.play(FadeIn(rest), Create(box), run_time=0.6)


def _swap_panel(scene, old, new, *, run_time: float = 0.8):
    """🔁 Replace one formula panel with the next."""
    scene.play(FadeOut(old[0]), FadeOut(old[1]), run_time=run_time / 2)
    scene.play(FadeIn(new[0]), Create(new[1]), run_time=run_time / 2)
    return new
#endregion


#region Charts
def _axes(origin, x_len: float, y_len: float, x_label: str, y_label: str, *, color: str = P_WHITE):
    """📐 Axis cross with arrow tips and typeset axis names."""
    o = np.array(origin, dtype=float)
    x_ax = Arrow(o, o + RIGHT * x_len, buff=0, stroke_width=2.4, color=color,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.06)
    y_ax = Arrow(o, o + UP * y_len, buff=0, stroke_width=2.4, color=color,
                 tip_length=0.15, max_tip_length_to_length_ratio=0.08)
    xl = _m(x_label, size=LABEL_FONT_SIZE, color=color)
    xl.next_to(x_ax.get_end(), DOWN, buff=0.14).align_to(x_ax.get_end(), RIGHT)
    yl = _m(y_label, size=LABEL_FONT_SIZE, color=color)
    yl.next_to(y_ax.get_end(), RIGHT, buff=0.14)
    return {"o": o, "x": x_ax, "y": y_ax, "xl": xl, "yl": yl, "group": VGroup(x_ax, y_ax, xl, yl)}


def _log_ladder(lo: float, hi: float, *, x: float, y_lo: float, y_hi: float, unit: str, step: int = 3):
    """📶 Vertical decade scale — every factor ten is one equal step, so a tealight and the world share one axis."""
    axis = Line(np.array([x, y_lo, 0.0]), np.array([x, y_hi, 0.0]), color=P_TEAL, stroke_width=3)

    def y_of(value: float) -> float:
        return y_lo + (np.log10(value) - lo) / (hi - lo) * (y_hi - y_lo)

    ticks, labels = VGroup(), VGroup()
    for e in range(int(np.ceil(lo)), int(np.floor(hi)) + 1):
        y = y_of(10.0 ** e)
        major = e % step == 0
        ticks.add(Line(np.array([x - (0.16 if major else 0.08), y, 0.0]), np.array([x, y, 0.0]),
                       color=P_WHITE, stroke_width=2.0 if major else 1.2))
        if major:
            labels.add(_m(rf"10^{{{e}}}\,\mathrm{{{unit}}}", np.array([x - 0.24, y - 0.07, 0.0]),
                          size=LABEL_FONT_SIZE, color=P_WHITE, edge="right"))
    return {"axis": axis, "ticks": ticks, "labels": labels, "group": VGroup(axis, ticks, labels),
            "y_of": y_of, "x": x}


def _ladder_marks(ladder, entries, *, x_text: float, gap: float = 0.07, bottom: float = -1.7, top: float = 2.3):
    """🏷️ Rung dots with labels dodged apart so neighbouring values stay readable."""
    x_axis = ladder["x"]
    dots, labels, ys = [], [], []
    for value, src, color in entries:
        y = ladder["y_of"](value)
        dots.append(Dot(np.array([x_axis, y, 0.0]), radius=0.065, color=color))
        labels.append(_m(src, size=LABEL_FONT_SIZE, color=color))
        ys.append(y)
    placed = list(ys)
    order = sorted(range(len(placed)), key=lambda i: -placed[i])
    for a, b in zip(order, order[1:]):
        need = (labels[a].height + labels[b].height) / 2 + gap
        placed[b] = min(placed[b], placed[a] - need)
    under = bottom - (placed[order[-1]] - labels[order[-1]].height / 2)
    if under > 0:
        placed = [p + under for p in placed]
    over = placed[order[0]] + labels[order[0]].height / 2 - top
    if over > 0:
        placed = [p - over for p in placed]
    leaders = []
    for i, label in enumerate(labels):
        label.move_to(np.array([x_text + label.width / 2, placed[i], 0.0]))
        leaders.append(Line(np.array([x_axis + 0.09, ys[i], 0.0]), np.array([x_text - 0.08, placed[i], 0.0]),
                            color=entries[i][2], stroke_width=1.3, stroke_opacity=0.6))
    return dots, labels, leaders
#endregion


#region Particle flow with variable speed
def _gradient(stops):
    """🎨 Piecewise colour ramp over ``u`` in [0, 1] — ``stops`` are ``(u, colour)``."""
    us = [s[0] for s in stops]
    cols = [ManimColor(s[1]) for s in stops]

    def color_at(u: float):
        u = float(np.clip(u, 0.0, 1.0))
        for i in range(len(us) - 1):
            if us[i] <= u <= us[i + 1]:
                f = (u - us[i]) / max(1e-9, us[i + 1] - us[i])
                return interpolate_color(cols[i], cols[i + 1], f)
        return cols[-1]

    return color_at


def _paced_flow(scene, streams, *, run_time: float = 3.0, waves: int = 6, cycles: float = 2.0,
                radius: float = 0.055, extra=None):
    """🌀 Particles whose speed follows ``speed(i, u)`` along each path — continuity made visible.

    ``streams`` holds ``(paths, colour, speed, colour_fn, closed)``; the last three may be ``None``/``False``.
    """
    us = np.linspace(0.0, 1.0, 241)
    dots, meta = VGroup(), []
    for paths, color, speed, color_fn, closed in streams:
        for i, path in enumerate(paths):
            g = np.array([max(1e-3, speed(i, u)) if speed else 1.0 for u in us])
            tau = np.concatenate([[0.0], np.cumsum(0.5 * (1.0 / g[1:] + 1.0 / g[:-1]) * np.diff(us))])
            tau /= tau[-1]
            for w in range(waves):
                dot = Dot(radius=radius, color=color, stroke_width=0)
                dot.set_fill(color, opacity=0.0)
                dot.move_to(path.point_from_proportion(0.0))
                dots.add(dot)
                meta.append((path, tau, (w / waves + 0.37 * i / waves) % 1.0, color, color_fn, closed))

    def update(group, alpha):
        for dot, (path, tau, offset, color, color_fn, closed) in zip(group, meta):
            t = (alpha * cycles + offset) % 1.0
            u = float(np.interp(t, tau, us))
            dot.move_to(path.point_from_proportion(u))
            fade = 1.0 if closed else min(1.0, t / 0.06, (1.0 - t) / 0.08)
            dot.set_fill(color_fn(u) if color_fn else color, opacity=max(0.0, fade))

    scene.add(dots)
    scene.play(UpdateFromAlphaFunc(dots, update), *(extra or []), run_time=run_time, rate_func=linear)
    scene.remove(dots)
#endregion


#region Light beams and wave packets
def _beam(start, end, color=P_YELLOW, opacity: float = 0.85, width: float = 2.4):
    """━ Straight light beam — sunlight arrives as parallel rays (same look as Cooling Teil 4)."""
    return Line(np.array(start, dtype=float), np.array(end, dtype=float), color=color, stroke_width=width,
                stroke_opacity=opacity)


def _mirror(direction, surface_angle: float):
    """🪞 Reflect a ray direction off a flat surface tilted by ``surface_angle`` radians."""
    u = np.array([np.cos(surface_angle), np.sin(surface_angle), 0.0])
    n = np.array([-u[1], u[0], 0.0])
    return direction - 2.0 * float(np.dot(direction, n)) * n


def _beam_pulses(scene, paths, color, *, repeats: int = 2, run_time: float = 2.4, width: float = 5.0, extra=None):
    """⚡ Bright pulses running along beam lines — the light itself travels, no extra glyphs."""
    lines = []
    for corners in paths:
        line = VMobject(color=color, stroke_width=width)
        line.set_points_as_corners([np.array(c, dtype=float) for c in corners])
        lines.append(line)
    flashes = [ShowPassingFlash(line.copy(), time_width=0.35) for _ in range(repeats) for line in lines]
    scene.play(LaggedStart(*flashes, lag_ratio=0.6 / len(lines)), *(extra or []), run_time=run_time)


#endregion


#region Glyphs — buildings
def _build_cross_section_house(center=ORIGIN):
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
    wall_left_2 = Line(bottom_left + UP * (w_height / 4 + w_h / 2), bottom_left + UP * (3 * w_height / 4 - w_h / 2),
                       color=P_WHITE, stroke_width=3)
    wall_left_3 = Line(bottom_left + UP * (3 * w_height / 4 + w_h / 2), top_left, color=P_WHITE, stroke_width=3)
    wall_right = Line(bottom_right, top_right, color=P_WHITE, stroke_width=3)
    walls = VGroup(wall_left_1, wall_left_2, wall_left_3, wall_right, level_1)
    roof = Polygon(top_left, roof_peak, top_right, color=P_WHITE, stroke_width=3)
    win1 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (w_height / 4) + RIGHT * 0.075)
    win2 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (3 * w_height / 4) + RIGHT * 0.075)
    window_group = VGroup(win1, win2)
    return {
        "center": center, "floor": floor_line, "walls": walls, "roof": roof, "window_group": window_group,
        "level_1": level_1, "wall_right": wall_right,
        "group": VGroup(floor_line, walls, roof, window_group),
    }


def _room(center, *, w: float = 5.0, h: float = 2.5, wall: float = 0.2, slab: float = 0.26,
          window=(0.30, 0.88)):
    """🏛️ One-room section — floor and ceiling slabs, a glazed left wall, a solid right wall."""
    cx, cy = float(center[0]), float(center[1])
    x_l, x_r = cx - w / 2, cx + w / 2
    y_f, y_c = cy - h / 2, cy + h / 2
    style = dict(color=P_WHITE, stroke_width=2, fill_color=P_WHITE, fill_opacity=0.14)
    floor = Rectangle(width=w + 2 * wall, height=slab, **style).move_to(np.array([cx, y_f - slab / 2, 0.0]))
    ceiling = Rectangle(width=w + 2 * wall, height=slab, **style).move_to(np.array([cx, y_c + slab / 2, 0.0]))
    lo, hi = y_f + window[0] * h, y_f + window[1] * h
    gx = x_l - wall / 2
    wall_lo = Rectangle(width=wall, height=lo - y_f, **style).move_to(np.array([gx, (y_f + lo) / 2, 0.0]))
    wall_hi = Rectangle(width=wall, height=y_c - hi, **style).move_to(np.array([gx, (hi + y_c) / 2, 0.0]))
    glass = VGroup(*[
        Line(np.array([gx + dx, lo, 0.0]), np.array([gx + dx, hi, 0.0]), color=P_CYAN, stroke_width=2.5)
        for dx in (-0.05, 0.05)
    ])
    wall_r = Rectangle(width=wall, height=h, **style).move_to(np.array([x_r + wall / 2, cy, 0.0]))
    air = Rectangle(width=w, height=h, stroke_width=0, fill_color=P_RED, fill_opacity=0.0)
    air.move_to(np.array([cx, cy, 0.0]))
    shell = VGroup(floor, ceiling, wall_lo, wall_hi, wall_r)
    return {
        "floor": floor, "ceiling": ceiling, "wall_lo": wall_lo, "wall_hi": wall_hi, "wall_r": wall_r,
        "glass": glass, "air": air, "shell": shell, "group": VGroup(air, shell, glass),
        "x_l": x_l, "x_r": x_r, "y_f": y_f, "y_c": y_c, "win_lo": lo, "win_hi": hi, "glass_x": gx,
        "center": np.array([cx, cy, 0.0]), "w": w, "h": h, "slab": slab,
    }


def _radiator(center, color=P_RED):
    """♨️ Panel radiator glyph."""
    body = RoundedRectangle(width=0.9, height=0.62, corner_radius=0.06, color=color, stroke_width=2.2)
    fins = VGroup(*[
        Line(UP * 0.22, DOWN * 0.22, color=color, stroke_width=1.6).shift(RIGHT * dx)
        for dx in np.linspace(-0.3, 0.3, 5)
    ])
    return VGroup(body, fins).move_to(center)
#endregion


#region Glyphs — everyday objects
def _build_sun(pos, color=P_YELLOW):
    """☀️ Compact sun — reused from the Cooling tutorials."""
    core = Dot(pos, radius=0.45, color=color)
    glow = Dot(pos, radius=0.7, color=color, fill_opacity=0.35)
    ring = Circle(radius=0.85, color=color, stroke_width=2, stroke_opacity=0.6).move_to(pos)
    burst = VGroup(*[
        Line(pos + 0.55 * np.array([np.cos(a), np.sin(a), 0.0]), pos + 0.9 * np.array([np.cos(a), np.sin(a), 0.0]),
             color=color, stroke_width=2)
        for a in np.linspace(0, TAU, 12, endpoint=False)
    ])
    return VGroup(glow, core, ring, burst)


def _moon(pos, color=P_WHITE):
    """🌙 Crescent for night-time beats."""
    disc = Circle(radius=0.24, color=color, fill_color=color, fill_opacity=0.85, stroke_width=0).move_to(pos)
    bite = Circle(radius=0.22, color=P_DEEP_DARK, fill_color=P_DEEP_DARK, fill_opacity=1.0, stroke_width=0)
    bite.move_to(np.array(pos) + np.array([0.12, 0.08, 0.0]))
    return VGroup(disc, bite)


def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Occupant glyph — same as Heating Modul 1."""
    head = Circle(radius=0.11, color=color, stroke_width=2.2)
    body = RoundedRectangle(width=0.30, height=0.40, corner_radius=0.09, color=color, stroke_width=2.2)
    body.next_to(head, DOWN, buff=0.04)
    return VGroup(head, body).scale(scale).move_to(pos)


def _clock(center, r: float = 0.42, color=P_CYAN):
    """🕐 Analogue dial — a full turn of the hand is one hour."""
    c = np.array(center, dtype=float)
    face = Circle(radius=r, color=P_WHITE, stroke_width=2.5).move_to(c)
    ticks = VGroup(*[
        Line(c + (r - 0.08) * np.array([np.cos(a), np.sin(a), 0.0]), c + r * np.array([np.cos(a), np.sin(a), 0.0]),
             color=P_WHITE, stroke_width=1.6)
        for a in np.linspace(0, TAU, 12, endpoint=False)
    ])
    hand = Line(c, c + UP * r * 0.78, color=color, stroke_width=3)
    return {"face": face, "hand": hand, "center": c, "group": VGroup(face, ticks, hand)}


def _vacuum(color=P_TEAL):
    """🧹 Canister vacuum cleaner."""
    body = RoundedRectangle(width=0.72, height=0.42, corner_radius=0.14, color=color, stroke_width=2.5,
                            fill_color=color, fill_opacity=0.2)
    wheels = VGroup(*[Circle(radius=0.08, color=color, stroke_width=2).move_to(body.get_bottom() + RIGHT * dx)
                      for dx in (-0.2, 0.2)])
    hose = ArcBetweenPoints(body.get_right() + UP * 0.08, body.get_right() + RIGHT * 0.7 + UP * 0.5,
                            angle=-PI / 2.5, color=color, stroke_width=2.5)
    tube = Line(hose.get_end(), hose.get_end() + RIGHT * 0.35 + DOWN * 0.72, color=color, stroke_width=2.5)
    nozzle = Line(tube.get_end() + LEFT * 0.14, tube.get_end() + RIGHT * 0.14, color=color, stroke_width=4)
    return VGroup(body, wheels, hose, tube, nozzle)


def _shower_head(anchor, color=P_WHITE):
    """🚿 Wall arm with a rose — drops fall from its underside."""
    a = np.array(anchor, dtype=float)
    arm = VMobject(color=color, stroke_width=3)
    arm.set_points_as_corners([a + LEFT * 0.9, a + LEFT * 0.9 + UP * 0.35, a + UP * 0.35, a + UP * 0.12])
    rose = Polygon(a + UP * 0.12 + LEFT * 0.12, a + UP * 0.12 + RIGHT * 0.12, a + RIGHT * 0.38, a + LEFT * 0.38,
                   color=color, stroke_width=2.5, fill_color=color, fill_opacity=0.3)
    return VGroup(arm, rose)


def _thermometer(bottom, *, height: float = 1.6, color=P_RED, level: float = 0.0):
    """🌡️ Bulb thermometer whose column follows a 0–1 ``level`` tracker."""
    b = np.array(bottom, dtype=float)
    tube = RoundedRectangle(width=0.2, height=height, corner_radius=0.1, color=P_WHITE, stroke_width=2)
    tube.move_to(b + UP * height / 2)
    bulb = Circle(radius=0.15, color=P_WHITE, stroke_width=2, fill_color=color, fill_opacity=0.9).move_to(b)
    tracker = ValueTracker(level)

    def column():
        h = max(0.02, (height - 0.12) * float(np.clip(tracker.get_value(), 0.0, 1.0)))
        return Rectangle(width=0.1, height=h, stroke_width=0, fill_color=color, fill_opacity=0.9).move_to(
            b + UP * (h / 2 + 0.05))

    return {"group": VGroup(tube, bulb), "column": always_redraw(column), "level": tracker,
            "top": b + UP * height}


def _energy_tank(center, *, height: float = 2.2, width: float = 0.9, color=P_YELLOW, level: float = 1.0):
    """🔋 One kilowatt-hour as a tank — the fill is the energy still in it."""
    c = np.array(center, dtype=float)
    frame = RoundedRectangle(width=width, height=height, corner_radius=0.1, color=P_WHITE, stroke_width=2.5)
    frame.move_to(c)
    cap = RoundedRectangle(width=width * 0.4, height=0.12, corner_radius=0.04, color=P_WHITE, stroke_width=2.5,
                           fill_color=P_WHITE, fill_opacity=0.6)
    cap.next_to(frame, UP, buff=0)
    tracker = ValueTracker(level)
    inner = height - 0.12
    base = frame.get_bottom() + UP * 0.06

    def fill():
        h = max(0.002, inner * float(np.clip(tracker.get_value(), 0.0, 1.0)))
        return Rectangle(width=width - 0.12, height=h, stroke_width=0, fill_color=color, fill_opacity=0.7).move_to(
            base + UP * h / 2)

    return {"frame": frame, "cap": cap, "level": tracker, "fill": always_redraw(fill),
            "group": VGroup(frame, cap), "center": c, "height": height, "width": width}


def _heart(center, *, size: float = 1.0, color=P_RED):
    """❤️ Heart outline from the classic parametric curve."""
    t = np.linspace(0, TAU, 90, endpoint=False)
    xs = 16 * np.sin(t) ** 3
    ys = 13 * np.cos(t) - 5 * np.cos(2 * t) - 2 * np.cos(3 * t) - np.cos(4 * t)
    pts = [np.array([x, y, 0.0]) * size / 34.0 for x, y in zip(xs, ys)]
    heart = VMobject(color=color, stroke_width=3, fill_color=color, fill_opacity=0.55)
    heart.set_points_smoothly(pts + [pts[0]])
    return heart.move_to(center)


def _lamp(anchor, drop: float = 0.55):
    """💡 Pendant lamp hanging from a ceiling point."""
    a = np.array(anchor, dtype=float)
    cable = Line(a, a + DOWN * drop, color=P_WHITE, stroke_width=2)
    shade = Polygon(a + DOWN * drop + LEFT * 0.1, a + DOWN * drop + RIGHT * 0.1,
                    a + DOWN * (drop + 0.2) + RIGHT * 0.3, a + DOWN * (drop + 0.2) + LEFT * 0.3,
                    color=P_WHITE, stroke_width=2)
    bulb = Circle(radius=0.12, color=P_YELLOW, fill_color=P_YELLOW, fill_opacity=0.85).move_to(a + DOWN * (drop + 0.3))
    return {"cable": cable, "bulb": bulb, "group": VGroup(cable, shade, bulb)}


def _tree_and_apple(ground_y=-1.15):
    """🌳 Line-art tree and the 100 g apple of the newton beat."""
    ground = Line(LEFT * 1.75, RIGHT * 1.75, color=P_TEAL, stroke_width=3)
    trunk_base = np.array([0.0, ground_y, 0.0])
    trunk = Line(trunk_base, trunk_base + UP * 1.42, color=P_WHITE, stroke_width=5)
    crown = Circle(radius=0.52, color=P_WHITE, stroke_width=3).move_to(trunk_base + UP * 1.72)
    ground.move_to(trunk_base)
    stem = Line(ORIGIN, DOWN * 0.07, color=P_WHITE, stroke_width=2.2)
    body = Circle(radius=0.13, color=P_RED, fill_color=P_RED, fill_opacity=0.92, stroke_width=2)
    body.next_to(stem, DOWN, buff=0)
    return VGroup(trunk, crown), VGroup(stem, body), ground


def _droplets(center, n: int = 6, spread=(0.5, 0.6), seed: int = 3, color=P_BLUE):
    """💧 Scattered condensate droplets."""
    rng = np.random.default_rng(seed)
    c = np.array(center, dtype=float)
    return VGroup(*[
        Ellipse(width=0.07, height=0.1, color=color, fill_color=color, fill_opacity=0.85, stroke_width=0).move_to(
            c + np.array([rng.uniform(-spread[0], spread[0]), rng.uniform(-spread[1], spread[1]), 0.0]))
        for _ in range(n)
    ])
#endregion
#endregion


#region Beat1 – Energy in everyday life: the kilowatt-hour
class Beat1_EnergieImAlltag(Scene):
    NARRATION = [
        ("meter",
         "The meter counts a house's energy in kilowatt-hours — physically, work that has been done.",
         "Der Zähler misst die Energie eines Hauses in Kilowattstunden — physikalisch: verrichtete Arbeit."),
        ("slip",
         "In everyday speech the h often gets dropped: we used four thousand kilowatts.",
         "Im Alltag fällt dabei oft das h weg: „4 000 kW verbraucht“."),
        ("hour",
         "But the h is the hour. Without it the number describes a power, not an amount.",
         "Doch das h ist die Stunde. Ohne sie beschreibt die Zahl eine Leistung, keine Menge."),
        ("product",
         "Kilowatt-hour means kilowatt times hour: one kilowatt, for one hour.",
         "Kilowattstunde heißt Kilowatt mal Stunde: 1 kW, eine Stunde lang."),
        ("vacuum",
         "What is one kilowatt-hour? A vacuum cleaner of about one kilowatt runs on it for one hour.",
         "Was ist 1 kWh? Ein Staubsauger mit rund 1 kW läuft damit eine Stunde."),
        ("shower",
         "Or about three minutes of warm shower: thirty litres of water heated from ten to thirty-eight degrees.",
         "Oder rund drei Minuten warm duschen: 30 Liter Wasser, erwärmt von 10 auf 38 °C."),
        ("heat",
         "Mass times specific heat times temperature rise: about one kilowatt-hour.",
         "Masse mal spezifische Wärme mal Temperaturanstieg: rund 1 kWh."),
        ("ladder",
         "On a scale: a tealight holds about a tenth of a kilowatt-hour; a household uses about three thousand a year in electricity.",
         "Auf einer Skala: Ein Teelicht enthält rund 0,1 kWh, ein Haushalt braucht rund 3 000 kWh Strom im Jahr."),
        ("house",
         "A detached house needs around fifteen thousand kilowatt-hours of heat per year.",
         "Ein Einfamilienhaus braucht rund 15 000 kWh Heizwärme im Jahr."),
        ("world",
         "Germany uses about 2.4 trillion kilowatt-hours of final energy a year, the world about 170 trillion of primary energy.",
         "Deutschland: rund 2 400 Mrd. kWh Endenergie im Jahr. Die Welt: rund 170 000 Mrd. kWh Primärenergie."),
        ("design",
         "How many of them a house needs is decided in the design: glass areas, ceilings, openings.",
         "Wie viele kWh ein Haus braucht, entscheidet der Entwurf: Glasflächen, Decken, Öffnungen."),
        ("chapters",
         "Each of these elements gets its own chapter: radiation, storage, airflow.",
         "Jedes dieser Bauteile bekommt ein eigenes Kapitel: Strahlung, Speicherung, Luftströmung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Energie im Alltag — die Kilowattstunde", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "meter"))
        self.play(FadeIn(caption), run_time=0.3)

        #region meter
        house = _build_cross_section_house(center=ORIGIN)
        hg = house["group"].scale(0.72).move_to(LEFT * 4.3 + DOWN * 0.35)
        wall_x = house["wall_right"].get_center()[0]
        meter_box = RoundedRectangle(width=2.5, height=0.66, corner_radius=0.08, color=P_TEAL, stroke_width=2.2,
                                     fill_color=P_DEEP_DARK, fill_opacity=0.92).move_to(np.array([-1.45, 0.3, 0.0]))
        meter_tag = Text("Zähler", font_size=LABEL_FONT_SIZE, color=P_TEAL).next_to(meter_box, UP, buff=0.12)
        kwh = ValueTracker(0.0)
        meter_read = _readout(lambda: rf"{_de(kwh.get_value())}\,\mathrm{{kWh}}",
                              meter_box.get_center() + DOWN * 0.09, color=P_TEAL, edge="center")
        wire_in = Line(np.array([0.6, 0.3, 0.0]), meter_box.get_right(), color=P_YELLOW, stroke_width=2, stroke_opacity=0.5)
        wire_out = Line(meter_box.get_left(), np.array([wall_x, 0.3, 0.0]), color=P_YELLOW, stroke_width=2,
                        stroke_opacity=0.5)
        self.play(FadeIn(hg), FadeIn(meter_box), FadeIn(meter_tag), Create(wire_in), Create(wire_out), run_time=1.0)
        self.add(meter_read)
        animate_flows(self, [(VGroup(Line(wire_in.get_start(), wire_in.get_end()),
                                     Line(wire_out.get_start(), wire_out.get_end())), P_YELLOW)],
                      run_time=3.2, waves=4, cycles=3.0, radius=0.06, streak=False,
                      extra=[kwh.animate.set_value(4000)])
        hold_for(self, N, "meter", used=BEAT_SUBTITLE_FADE + 0.3 + 4.2)
        #endregion

        #region slip and hour
        caption = swap_caption(self, caption, subtitle_text(N, "slip"))
        person = _person(np.array([0.7, -0.75, 0.0]), scale=1.5)
        said, said_items = math_row([
            ("n", r"\text{4 000}", P_WHITE), ("kw", r"\mathrm{kW}", P_WHITE), ("v", r"\text{verbraucht}", P_WHITE),
        ], font_size=BODY_FONT_SIZE, buff=0.14)
        said.move_to(np.array([2.75, 0.95, 0.0]))

        def bubble_for(content):
            frame = RoundedRectangle(width=content.width + 0.55, height=content.height + 0.45, corner_radius=0.18,
                                     color=P_WHITE, stroke_width=2).move_to(content)
            tail = Polygon(frame.get_bottom() + LEFT * 0.9 + UP * 0.02, frame.get_bottom() + LEFT * 0.45 + UP * 0.02,
                           person.get_top() + RIGHT * 0.2 + UP * 0.12, color=P_WHITE, stroke_width=2)
            return VGroup(frame, tail)

        bubble = bubble_for(said)
        self.play(FadeIn(person), Create(bubble), FadeIn(said), run_time=1.0)
        hold_for(self, N, "slip")

        caption = swap_caption(self, caption, subtitle_text(N, "hour"))
        clock = _clock(np.array([5.75, 0.95, 0.0]))
        clock_lbl = Text("1 Stunde", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(clock["face"], DOWN, buff=0.14)
        h_glyph = _m(r"\mathrm{h}", clock["face"].get_top() + UP * 0.3, color=P_ORANGE)
        self.play(FadeIn(clock["group"]), FadeIn(clock_lbl), run_time=0.6)
        self.play(Rotate(clock["hand"], angle=-TAU, about_point=clock["center"]), FadeIn(h_glyph, scale=0.5),
                  run_time=1.2)
        kw = said_items["kw"]
        h_target = h_glyph.copy()
        _place(h_target, np.array([kw.get_right()[0] + 0.03, kw.base.get_center()[1], 0.0]), "left")
        shift = h_target.width + 0.05
        frame, tail = bubble
        self.remove(bubble)
        self.add(frame, tail)
        self.play(FadeOut(frame), said_items["v"].animate.shift(RIGHT * shift), kw.animate.set_color(P_CYAN),
                  run_time=0.6)
        self.play(h_glyph.animate.move_to(np.array([h_target.get_center()[0], said.get_top()[1] + 0.45, 0.0])),
                  run_time=0.7)
        self.play(h_glyph.animate.move_to(h_target.get_center()), run_time=0.5)
        frame = bubble_for(VGroup(said, h_glyph))[0]
        self.play(Create(frame), run_time=0.5)
        bubble = VGroup(frame, tail)
        hold_for(self, N, "hour")
        #endregion

        #region product
        caption = swap_caption(self, caption, subtitle_text(N, "product"))
        panel = _panel([
            (None, "1", P_WHITE), ("kw", r"\mathrm{kW}", P_CYAN), (None, r"\cdot", P_WHITE),
            (None, "1", P_WHITE), ("h", r"\mathrm{h}", P_ORANGE), (None, "=", P_WHITE),
            (None, "1", P_WHITE), ("kwh", r"\mathrm{kWh}", P_YELLOW),
        ])
        row, box, items = panel
        self.play(FadeOut(person), FadeOut(bubble), run_time=0.5)
        _fill_panel(self, panel, {"kw": kw, "h": h_glyph})
        hold_for(self, N, "product")
        #endregion

        #region vacuum
        caption = swap_caption(self, caption, subtitle_text(N, "vacuum"))
        tank = _energy_tank(np.array([-1.3, 0.35, 0.0]), height=2.3)
        tank_lbl = _m(r"1\,\mathrm{kWh}", color=P_YELLOW).next_to(tank["cap"], UP, buff=0.14)
        tank_read = _readout(lambda: rf"{_de(tank['level'].get_value(), 2)}\,\mathrm{{kWh}}",
                             np.array([-1.95, 0.25, 0.0]), color=P_YELLOW, edge="right")
        floor_y = -0.98
        floor = Line(np.array([0.5, floor_y, 0.0]), np.array([4.9, floor_y, 0.0]), color=P_TEAL, stroke_width=3)
        vac = _vacuum()
        vac.shift(np.array([2.7 - vac.get_center()[0], floor_y + 0.02 - vac.get_bottom()[1], 0.0]))
        vac_tag = always_redraw(lambda: _m(r"P \approx 1\,\mathrm{kW}",
                                           np.array([vac[0].get_center()[0] + 0.45, vac.get_top()[1] + 0.3, 0.0]),
                                           color=P_TEAL))
        clock2 = _clock(np.array([5.85, 1.35, 0.0]))
        minutes = ValueTracker(0.0)
        min_read = _readout(lambda: rf"t = {_de(minutes.get_value())}\,\mathrm{{min}}",
                            np.array([5.85, 0.6, 0.0]), color=P_CYAN, edge="center")
        outlet = tank["frame"].get_right() + DOWN * 0.3
        phase = ValueTracker(0.0)

        def plugged_cable():
            plug = vac[0].get_left() + RIGHT * 0.02
            sag = max(floor_y + 0.04, min(outlet[1], plug[1]) - 0.25)
            wire = VMobject(color=P_YELLOW, stroke_width=2.5, stroke_opacity=0.6)
            wire.set_points_smoothly([outlet, outlet + RIGHT * 0.3,
                                      np.array([0.5 * (outlet[0] + plug[0]), sag, 0.0]), plug + LEFT * 0.3, plug])
            pulses = VGroup()
            for k in range(6):
                u = (k / 6 + phase.get_value()) % 1.0
                pulses.add(Dot(wire.point_from_proportion(u), radius=0.055, color=P_YELLOW).set_opacity(
                    min(1.0, u / 0.08, (1.0 - u) / 0.08)))
            return VGroup(wire, pulses)

        cable = always_redraw(plugged_cable)
        self.play(
            FadeOut(said), FadeOut(h_glyph), FadeOut(clock["group"]),
            FadeOut(clock_lbl), FadeOut(hg), FadeOut(meter_box), FadeOut(meter_tag), FadeOut(wire_in),
            FadeOut(wire_out), FadeOut(meter_read), run_time=0.7,
        )
        self.play(FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(tank_lbl), FadeIn(floor), FadeIn(vac),
                  FadeIn(vac_tag), FadeIn(clock2["group"]), FadeIn(cable), run_time=1.0)
        self.add(tank_read, min_read)
        x0 = vac.get_center()[0]
        self.play(
            phase.animate.set_value(5.0),
            tank["level"].animate.set_value(0.0),
            minutes.animate.set_value(60),
            Rotate(clock2["hand"], angle=-TAU, about_point=clock2["center"]),
            UpdateFromAlphaFunc(vac, lambda m, a: m.set_x(x0 + 0.85 * np.sin(a * TAU * 3))),
            run_time=5.5, rate_func=linear,
        )
        hold_for(self, N, "vacuum")
        #endregion

        #region shower
        caption = swap_caption(self, caption, subtitle_text(N, "shower"))
        head = _shower_head(np.array([3.0, 1.35, 0.0]))
        drop_paths = [Line(np.array([3.0 + dx, 1.2, 0.0]), np.array([3.0 + dx * 1.6, -0.95, 0.0])) for dx in
                      np.linspace(-0.3, 0.3, 5)]
        litres = ValueTracker(0.0)
        litre_read = _readout(lambda: rf"V = {_de(litres.get_value())}\,\mathrm{{l}}",
                              np.array([2.25, -0.75, 0.0]), color=P_CYAN, edge="right")
        thermo = _thermometer(np.array([4.55, -0.85, 0.0]), height=1.7, level=10 / 50)
        temp = ValueTracker(10.0)
        thermo["level"].add_updater(lambda m: m.set_value(temp.get_value() / 50))
        temp_read = _readout(lambda: rf"{_de(temp.get_value())}\,\mathrm{{°C}}",
                             np.array([4.3, 0.05, 0.0]), color=P_RED, edge="right")
        shower_cable = smooth_path([tank["frame"].get_right() + UP * 0.6, np.array([0.8, 1.45, 0.0]),
                                    np.array([2.1, 1.7, 0.0])])
        self.play(FadeOut(vac), FadeOut(vac_tag), FadeOut(cable), FadeIn(head), FadeIn(thermo["group"]),
                  FadeIn(thermo["column"]), tank["level"].animate.set_value(1.0), minutes.animate.set_value(0),
                  run_time=1.0)
        self.add(litre_read, temp_read)
        _paced_flow(self, [
            (drop_paths, P_CYAN, None, None, False),
            ([shower_cable], P_YELLOW, None, None, False),
        ], run_time=5.0, waves=4, cycles=5.0, radius=0.045, extra=[
            tank["level"].animate.set_value(0.0),
            minutes.animate.set_value(3),
            Rotate(clock2["hand"], angle=-TAU * 3 / 60, about_point=clock2["center"]),
            litres.animate.set_value(30),
            temp.animate.set_value(38),
        ])
        hold_for(self, N, "shower")
        #endregion

        #region heat
        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        heat_panel = _panel([
            (None, "Q", P_WHITE), (None, "=", P_WHITE), ("m", r"30\,\mathrm{kg}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("c", r"4{,}19\,\frac{\mathrm{kJ}}{\mathrm{kg\,K}}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("dt", r"28\,\mathrm{K}", P_RED), (None, r"\approx", P_WHITE), ("res", r"1\,\mathrm{kWh}", P_YELLOW),
        ])
        self.play(FadeOut(row), FadeOut(box), FadeOut(floor), run_time=0.4)
        h_row, h_box, h_items = heat_panel
        _fill_panel(self, heat_panel, {"m": litre_read, "dt": temp_read})
        self.play(Indicate(h_items["res"], color=P_YELLOW), Indicate(tank_lbl, color=P_YELLOW), run_time=0.9)
        hold_for(self, N, "heat")
        #endregion

        #region ladder
        caption = swap_caption(self, caption, subtitle_text(N, "ladder"))
        ladder = _log_ladder(-1.5, 14.6, x=-2.4, y_lo=-1.6, y_hi=2.25, unit="kWh")
        entries = [
            (0.13, r"\text{Teelicht} \approx 0{,}1\,\mathrm{kWh}", P_ORANGE),
            (1.0, r"\text{Staubsauger, 1 Stunde} \approx 1\,\mathrm{kWh}", P_YELLOW),
            (3000, r"\text{Haushaltsstrom pro Jahr} \approx 3\,000\,\mathrm{kWh}", P_CYAN),
            (15000, r"\text{Heizwärme Einfamilienhaus pro Jahr} \approx 15\,000\,\mathrm{kWh}", P_RED),
            (2.4e12, r"\text{Deutschland pro Jahr, Endenergie} \approx 2{,}4 \cdot 10^{12}\,\mathrm{kWh}", P_GREEN),
            (1.7e14, r"\text{Welt pro Jahr, Primärenergie} \approx 1{,}7 \cdot 10^{14}\,\mathrm{kWh}", P_TEAL),
        ]
        dots, labels, leaders = _ladder_marks(ladder, entries, x_text=-1.75)
        thermo["level"].clear_updaters()
        self.play(
            FadeOut(h_row), FadeOut(h_box), FadeOut(head), FadeOut(thermo["group"]), FadeOut(thermo["column"]),
            FadeOut(litre_read), FadeOut(temp_read), FadeOut(min_read), FadeOut(clock2["group"]),
            FadeOut(tank_read), FadeOut(tank_lbl), run_time=0.6,
        )
        tank_static = VGroup(tank["frame"].copy(), tank["cap"].copy())
        self.remove(tank["group"], tank["fill"])
        self.add(tank_static)
        self.play(Create(ladder["axis"]), FadeIn(ladder["ticks"]), FadeIn(ladder["labels"]),
                  ReplacementTransform(tank_static, dots[1]), run_time=1.2)
        for i in (0, 1, 2):
            self.play(FadeIn(dots[i]) if i != 1 else Indicate(dots[i], color=P_YELLOW),
                      Create(leaders[i]), FadeIn(labels[i], shift=RIGHT * 0.1), run_time=0.7)
        hold_for(self, N, "ladder")

        caption = swap_caption(self, caption, subtitle_text(N, "house"))
        self.play(FadeIn(dots[3]), Create(leaders[3]), FadeIn(labels[3], shift=RIGHT * 0.1), run_time=0.8)
        hold_for(self, N, "house")

        caption = swap_caption(self, caption, subtitle_text(N, "world"))
        for i in (4, 5):
            self.play(FadeIn(dots[i]), Create(leaders[i]), FadeIn(labels[i], shift=RIGHT * 0.1), run_time=0.8)
        span = dim_arrow(np.array([-5.0, ladder["y_of"](0.13), 0.0]), np.array([-5.0, ladder["y_of"](1.7e14), 0.0]),
                         color=P_YELLOW)
        span_lbl = _m(r"\times\,10^{15}", color=P_YELLOW).next_to(span, LEFT, buff=0.14)
        self.play(GrowFromCenter(span), FadeIn(span_lbl), run_time=0.9)
        hold_for(self, N, "world")
        #endregion

        #region design
        caption = swap_caption(self, caption, subtitle_text(N, "design"))
        self.play(FadeOut(VGroup(ladder["group"], *dots, *labels, *leaders, span, span_lbl)), run_time=0.6)
        house2 = _build_cross_section_house(center=np.array([-0.9, -0.45, 0.0]))
        sun = _build_sun(np.array([-5.9, 1.85, 0.0])).scale(0.4)
        win_hi = house2["window_group"][1].get_center()
        win_lo = house2["window_group"][0].get_center()
        rays = VGroup(*[
            solar_wave_ray(np.array([-5.6, 1.6, 0.0]) + DOWN * 0.15 * k, win_hi + UP * (0.12 - 0.12 * k),
                           amp=0.045, cycles=12)
            for k in range(3)
        ])
        slab = Rectangle(width=3.6, height=0.18, color=P_ORANGE, stroke_width=2, fill_color=P_ORANGE, fill_opacity=0.5)
        slab.move_to(house2["level_1"].get_center())
        air_path = smooth_path([np.array([-6.0, -1.15, 0.0]), win_lo, np.array([-1.2, -1.05, 0.0]),
                                np.array([0.4, -0.75, 0.0])])
        left_tags = VGroup(
            Text("Glas — Strahlung", font_size=LABEL_FONT_SIZE, color=P_YELLOW),
            Text("Öffnungen — Luftströmung", font_size=LABEL_FONT_SIZE, color=P_CYAN),
        )
        for tag, y in zip(left_tags, (win_hi[1] - 0.42, win_lo[1] + 0.32)):
            tag.move_to(np.array([-3.0 - tag.width / 2, y, 0.0]))
        left_leaders = VGroup(*[
            Line(tag.get_right() + RIGHT * 0.12, anchor + LEFT * 0.08, color=color, stroke_width=1.4,
                 stroke_opacity=0.55)
            for tag, anchor, color in zip(left_tags, (win_hi, win_lo), (P_YELLOW, P_CYAN))
        ])
        right_tags, right_leaders = side_labels([
            (slab.get_right(), "Decke — Speichermasse", P_ORANGE),
        ], x=1.55, align="left")
        self.play(FadeIn(house2["group"]), FadeIn(sun), run_time=0.9)
        self.play(LaggedStart(*[Create(r) for r in rays], lag_ratio=0.25), FadeIn(left_tags[0]),
                  Create(left_leaders[0]), Indicate(house2["window_group"][1], color=P_YELLOW), run_time=1.2)
        self.play(ReplacementTransform(house2["level_1"].copy(), slab), FadeIn(right_tags[0]),
                  Create(right_leaders[0]), run_time=1.0)
        self.play(FadeIn(left_tags[1]), Create(left_leaders[1]), run_time=0.5)
        _paced_flow(self, [([air_path], P_CYAN, None, None, False)], run_time=2.0, waves=7, cycles=1.6)
        hold_for(self, N, "design")

        caption = swap_caption(self, caption, subtitle_text(N, "chapters"))
        self.play(LaggedStart(
            Indicate(left_tags[0], color=P_WHITE, scale_factor=1.0),
            Indicate(right_tags[0], color=P_WHITE, scale_factor=1.0),
            Indicate(left_tags[1], color=P_WHITE, scale_factor=1.0), lag_ratio=0.5), run_time=2.0)
        hold_for(self, N, "chapters")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Power: how fast energy flows
class Beat2_Leistung(Scene):
    NARRATION = [
        ("rate",
         "The same kilowatt-hour can flow slowly or quickly. How fast is the power.",
         "Dieselbe Kilowattstunde kann langsam oder schnell fließen. Wie schnell, sagt die Leistung."),
        ("area",
         "In a power-time diagram energy is an area: the height is the power, the width the time.",
         "Im Leistung-Zeit-Diagramm ist Energie eine Fläche: Höhe = Leistung, Breite = Zeit."),
        ("vacuum",
         "The vacuum cleaner: one kilowatt high, one hour wide — one kilowatt-hour.",
         "Der Staubsauger: 1 kW hoch, 1 h breit — Fläche 1 kWh."),
        ("kettle",
         "A two-kilowatt kettle needs only half an hour for it: twice as high, half as wide.",
         "Ein Wasserkocher mit 2 kW braucht dafür nur eine halbe Stunde: doppelt so hoch, halb so breit."),
        ("shower",
         "The shower: one kilowatt-hour in three minutes is about twenty kilowatts — so instantaneous heaters have 18 to 24.",
         "Die Dusche: 1 kWh in drei Minuten sind rund 20 kW — darum haben Durchlauferhitzer 18 bis 24 kW."),
        ("formula",
         "So energy is power times time, and power is energy divided by time.",
         "Also: Energie ist Leistung mal Zeit, Leistung ist Energie durch Zeit."),
        ("watt",
         "The base unit: one watt is one joule per second.",
         "Die Grundeinheit: 1 Watt ist 1 Joule pro Sekunde."),
        ("joule",
         "One kilowatt-hour is a thousand watts times 3,600 seconds — 3.6 million joules.",
         "1 kWh sind 1 000 W mal 3 600 s — 3,6 Millionen Joule."),
        ("scale1",
         "A scale of power: a tealight about thirty watts, a resting person about a hundred, a vacuum cleaner about one kilowatt.",
         "Eine Skala der Leistung: Teelicht rund 30 W, ruhender Mensch rund 100 W, Staubsauger rund 1 kW."),
        ("scale2",
         "An insulated detached house on the coldest day about eight kilowatts, a car about a hundred.",
         "Gedämmtes Einfamilienhaus am kältesten Tag rund 8 kW, ein Auto rund 100 kW."),
        ("scale3",
         "A wind turbine about five megawatts, a large power-station unit about 1.4 gigawatts.",
         "Ein Windrad rund 5 MW, ein großer Kraftwerksblock rund 1,4 GW."),
        ("scale4",
         "All power stations in the world together: about nine terawatts of installed capacity.",
         "Alle Kraftwerke der Welt zusammen: rund 9 TW installierte Leistung."),
        ("load",
         "For a house: the heating load in kilowatts, the peak of the coldest hour, sizes the heating — DIN EN 12831.",
         "Für ein Haus: Die Heizlast in kW, der Spitzenwert der kältesten Stunde, bemisst die Heizung (DIN EN 12831)."),
        ("demand",
         "The annual heating demand in kilowatt-hours is the area under the power curve over the year — DIN V 18599.",
         "Der Jahres-Heizwärmebedarf in kWh ist die Fläche unter der Leistungskurve übers Jahr (DIN V 18599)."),
        ("summer",
         "From May to September, sun and internal heat cover the losses — the heating stays off.",
         "Von Mai bis September decken Sonne und innere Wärme die Verluste — die Heizung bleibt aus."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Leistung — wie schnell Energie fließt", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "rate"))
        self.play(FadeIn(caption), run_time=0.3)

        #region rate and area
        O = np.array([-4.9, -0.55, 0.0])
        SX, SY = 4.0, 1.18

        def pt(t, p):
            return O + RIGHT * t * SX + UP * p * SY

        tank = _energy_tank(np.array([-1.8, 0.4, 0.0]), height=2.2)
        tank_lbl = _m(r"1\,\mathrm{kWh}", color=P_YELLOW).next_to(tank["cap"], UP, buff=0.14)
        self.play(FadeIn(tank["group"]), FadeIn(tank["fill"]), FadeIn(tank_lbl), run_time=0.8)
        hold_for(self, N, "rate", used=BEAT_SUBTITLE_FADE + 0.3 + 0.8)

        caption = swap_caption(self, caption, subtitle_text(N, "area"))
        ax = _axes(O, 5.1, 2.95, r"t\;[\mathrm{h}]", r"P\;[\mathrm{kW}]")
        t_ticks = VGroup(*[Line(pt(t, 0) + DOWN * 0.07, pt(t, 0) + UP * 0.07, color=P_WHITE, stroke_width=2)
                           for t in (0.5, 1.0)])
        p_ticks = VGroup(*[Line(pt(0, p) + LEFT * 0.07, pt(0, p) + RIGHT * 0.07, color=P_WHITE, stroke_width=2)
                           for p in (1.0, 2.0)])
        t_lbls = VGroup(_m(r"0{,}5", pt(0.5, 0) + DOWN * 0.38, size=LABEL_FONT_SIZE),
                        _m(r"1", pt(1.0, 0) + DOWN * 0.38, size=LABEL_FONT_SIZE))
        p_lbls = VGroup(_m(r"1", pt(0, 1) + LEFT * 0.22 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right"),
                        _m(r"2", pt(0, 2) + LEFT * 0.22 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right"))
        P = ValueTracker(1.0)
        T = ValueTracker(1.0)
        locked = {"on": False}

        def t_now():
            return 1.0 / P.get_value() if locked["on"] else T.get_value()

        def area():
            p = min(P.get_value(), 2.2)
            t = max(0.002, t_now())
            return Polygon(pt(0, 0), pt(t, 0), pt(t, p), pt(0, p), stroke_color=P_YELLOW, stroke_width=2,
                           fill_color=P_YELLOW, fill_opacity=0.35)

        rect = always_redraw(area)
        target = area()
        tank_static = VGroup(tank["frame"].copy(), tank["fill"].copy())
        self.remove(tank["group"], tank["fill"])
        self.add(tank_static)
        self.play(FadeIn(ax["group"]), FadeIn(t_ticks), FadeIn(p_ticks), FadeIn(t_lbls), FadeIn(p_lbls),
                  ReplacementTransform(tank_static, target), tank_lbl.animate.move_to(pt(0.5, 0.5)), run_time=1.4)
        self.remove(target)
        self.add(rect)
        self.bring_to_front(tank_lbl)
        hold_for(self, N, "area")
        #endregion

        #region vacuum, kettle, shower
        caption = swap_caption(self, caption, subtitle_text(N, "vacuum"))
        name = Text("Staubsauger", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(np.array([2.6, 1.95, 0.0]))
        p_read = _readout(lambda: rf"P = {_de(P.get_value(), 1)}\,\mathrm{{kW}}", np.array([1.2, 1.3, 0.0]),
                          color=P_CYAN)
        t_read = _readout(
            lambda: rf"t = {_de(t_now(), 2)}\,\mathrm{{h}} = {_de(t_now() * 60)}\,\mathrm{{min}}",
            np.array([1.2, 0.75, 0.0]), color=P_ORANGE)
        e_read = _readout(lambda: rf"E = P \cdot t = {_de(P.get_value() * t_now(), 2)}\,\mathrm{{kWh}}",
                          np.array([1.2, 0.2, 0.0]), color=P_YELLOW)
        self.play(FadeOut(tank_lbl), FadeIn(name), run_time=0.5)
        T.set_value(0.0)
        self.add(p_read, t_read, e_read)
        self.play(T.animate.set_value(1.0), run_time=3.0, rate_func=linear)
        hold_for(self, N, "vacuum")

        caption = swap_caption(self, caption, subtitle_text(N, "kettle"))
        locked["on"] = True
        self.play(Transform(name, Text("Wasserkocher", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(name)),
                  run_time=0.5)
        self.play(P.animate.set_value(2.0), run_time=2.2)
        hold_for(self, N, "kettle")

        caption = swap_caption(self, caption, subtitle_text(N, "shower"))
        self.play(Transform(name, Text("Dusche (Durchlauferhitzer)", font_size=BODY_FONT_SIZE,
                                       color=P_CYAN).move_to(name)), run_time=0.5)
        self.play(P.animate.set_value(20.0), run_time=2.6, rate_func=rate_functions.ease_in_out_sine)
        zigzag = VMobject(color=P_CYAN, stroke_width=2.5)
        zigzag.set_points_as_corners([pt(0.0, 2.2), pt(0.017, 2.28), pt(0.033, 2.2), pt(0.05, 2.28)])
        clip = VGroup(zigzag, _m(r"\uparrow 20\,\mathrm{kW}", pt(0.12, 1.85), size=LABEL_FONT_SIZE, color=P_CYAN,
                                 edge="left"))
        self.play(FadeIn(clip), run_time=0.5)
        hold_for(self, N, "shower")
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        self.play(FadeOut(clip), FadeOut(name), run_time=0.4)
        self.play(P.animate.set_value(1.0), run_time=1.0)
        h_brace = Brace(Line(pt(0, 1), pt(1, 1)), UP, color=P_ORANGE)
        h_lbl = _m("t", color=P_ORANGE).next_to(h_brace, UP, buff=0.08)
        v_brace = Brace(Line(pt(1, 0), pt(1, 1)), RIGHT, color=P_CYAN)
        v_lbl = _m("P", color=P_CYAN).next_to(v_brace, RIGHT, buff=0.1)
        row, box, items = _panel([
            ("e", "E", P_YELLOW), (None, "=", P_WHITE), ("p", "P", P_CYAN), (None, r"\cdot", P_WHITE),
            ("t", "t", P_ORANGE), (None, r"\quad\quad", P_WHITE),
            ("p2", "P", P_CYAN), (None, "=", P_WHITE),
            ("frac", r"\frac{\textcolor{#FFE66D}{E}}{\textcolor{#FFAAA5}{t}}", P_WHITE),
        ])
        ring_p = highlight_param(items, "p", color=P_CYAN)
        ring_t = highlight_param(items, "t", color=P_ORANGE)
        self.play(FadeIn(row), Create(box), run_time=0.9)
        self.play(GrowFromCenter(v_brace), FadeIn(v_lbl), Create(ring_p), run_time=0.8)
        self.play(GrowFromCenter(h_brace), FadeIn(h_lbl), Create(ring_t), run_time=0.8)
        hold_for(self, N, "formula")
        #endregion

        #region watt and joule
        caption = swap_caption(self, caption, subtitle_text(N, "watt"))
        self.play(FadeOut(ring_p), FadeOut(ring_t), FadeOut(e_read), FadeOut(t_read), FadeOut(p_read), run_time=0.4)
        new_xl = _m(r"t\;[\mathrm{s}]", size=LABEL_FONT_SIZE).move_to(ax["xl"])
        new_yl = _m(r"P\;[\mathrm{W}]", size=LABEL_FONT_SIZE).move_to(ax["yl"], aligned_edge=LEFT)
        new_t = VGroup(_m(r"1\,800", t_lbls[0].get_center(), size=LABEL_FONT_SIZE),
                       _m(r"3\,600", t_lbls[1].get_center(), size=LABEL_FONT_SIZE))
        new_p = VGroup(_m(r"1\,000", size=LABEL_FONT_SIZE).move_to(p_lbls[0], aligned_edge=RIGHT),
                       _m(r"2\,000", size=LABEL_FONT_SIZE).move_to(p_lbls[1], aligned_edge=RIGHT))
        new_h = _m(r"3\,600\,\mathrm{s}", color=P_ORANGE).next_to(h_brace, UP, buff=0.08)
        new_v = _m(r"1\,000\,\mathrm{W}", color=P_CYAN).next_to(v_brace, RIGHT, buff=0.1)
        w_panel = _panel([
            (None, r"1\,\mathrm{W}", P_CYAN), (None, "=", P_WHITE), (None, r"1\,\frac{\mathrm{J}}{\mathrm{s}}", P_YELLOW),
        ])
        self.play(ReplacementTransform(ax["xl"], new_xl), ReplacementTransform(ax["yl"], new_yl),
                  ReplacementTransform(t_lbls, new_t), ReplacementTransform(p_lbls, new_p),
                  ReplacementTransform(h_lbl, new_h), ReplacementTransform(v_lbl, new_v), run_time=1.2)
        _swap_panel(self, (row, box), w_panel[:2])
        hold_for(self, N, "watt")

        caption = swap_caption(self, caption, subtitle_text(N, "joule"))
        j_lbl = _m(r"3{,}6 \cdot 10^{6}\,\mathrm{J}", pt(0.5, 0.45), color=P_YELLOW)
        j_panel = _panel([
            (None, r"1\,\mathrm{kWh}", P_YELLOW), (None, "=", P_WHITE), (None, r"1\,000\,\mathrm{W}", P_CYAN),
            (None, r"\cdot", P_WHITE), (None, r"3\,600\,\mathrm{s}", P_ORANGE), (None, "=", P_WHITE),
            (None, r"3{,}6 \cdot 10^{6}\,\mathrm{J}", P_YELLOW),
        ])
        self.play(FadeIn(j_lbl, scale=0.7), run_time=0.7)
        _swap_panel(self, w_panel[:2], j_panel[:2])
        hold_for(self, N, "joule")
        #endregion

        #region scale
        caption = swap_caption(self, caption, subtitle_text(N, "scale1"))
        self.play(FadeOut(VGroup(ax["x"], ax["y"], new_xl, new_yl, t_ticks, p_ticks, new_t, new_p, h_brace, v_brace,
                                 new_h, new_v, j_lbl, j_panel[0], j_panel[1])), FadeOut(rect), run_time=0.6)
        ladder = _log_ladder(1.0, 13.3, x=-2.6, y_lo=-1.6, y_hi=2.25, unit="W")
        entries = [
            (30, r"\text{Teelicht} \approx 30\,\mathrm{W}", P_ORANGE),
            (100, r"\text{Mensch in Ruhe} \approx 100\,\mathrm{W}", P_RED),
            (1000, r"\text{Staubsauger} \approx 1\,\mathrm{kW}", P_YELLOW),
            (8000, r"\text{Einfamilienhaus, kältester Tag} \approx 8\,\mathrm{kW}", P_RED),
            (1e5, r"\text{Auto} \approx 100\,\mathrm{kW}", P_CYAN),
            (5e6, r"\text{Windrad} \approx 5\,\mathrm{MW}", P_GREEN),
            (1.4e9, r"\text{Kraftwerksblock} \approx 1{,}4\,\mathrm{GW}", P_ORANGE),
            (9e12, r"\text{alle Kraftwerke der Welt} \approx 9\,\mathrm{TW}", P_TEAL),
        ]
        dots, labels, leaders = _ladder_marks(ladder, entries, x_text=-1.95)
        marker = Dot(np.array([ladder["x"], ladder["y_of"](30), 0.0]), radius=0.1, color=P_YELLOW)
        self.play(Create(ladder["axis"]), FadeIn(ladder["ticks"]), FadeIn(ladder["labels"]), FadeIn(marker),
                  run_time=1.0)
        groups = {"scale1": (0, 1, 2), "scale2": (3, 4), "scale3": (5, 6), "scale4": (7,)}
        for key, idxs in groups.items():
            if key != "scale1":
                caption = swap_caption(self, caption, subtitle_text(N, key))
            for i in idxs:
                self.play(marker.animate.move_to(dots[i]), FadeIn(dots[i]), Create(leaders[i]),
                          FadeIn(labels[i], shift=RIGHT * 0.1), run_time=0.75)
            if key == "scale4":
                span = dim_arrow(np.array([-5.0, ladder["y_of"](30), 0.0]),
                                 np.array([-5.0, ladder["y_of"](9e12), 0.0]), color=P_YELLOW)
                span_lbl = _m(r"\times\,3 \cdot 10^{11}", color=P_YELLOW).next_to(span, LEFT, buff=0.12)
                self.play(GrowFromCenter(span), FadeIn(span_lbl), run_time=0.9)
            hold_for(self, N, key)
        #endregion

        #region load and demand
        caption = swap_caption(self, caption, subtitle_text(N, "load"))
        self.play(FadeOut(VGroup(ladder["group"], marker, *dots, *labels, *leaders, span, span_lbl)), run_time=0.6)
        YO = np.array([-5.7, -1.2, 0.0])
        DX, DY = 9.6 / 365.0, 0.31
        days = np.arange(366)
        raw = np.maximum(0.0, 0.26 + np.cos(TAU * (days - 15) / 365.0))
        kw_day = raw * 15000.0 / (24.0 * raw[:365].sum())
        cum = np.concatenate([[0.0], np.cumsum(kw_day[:365] * 24.0)])

        def ypt(d, p):
            return YO + RIGHT * d * DX + UP * p * DY

        yax = _axes(YO, 10.7, 3.25, "t", r"P\;[\mathrm{kW}]")
        months = VGroup(*[
            Text(m, font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(ypt(15 + 30.4 * i, 0) + DOWN * 0.24)
            for i, m in enumerate(("Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"))
        ])
        curve = VMobject(color=P_CYAN, stroke_width=3)
        curve.set_points_smoothly([ypt(d, kw_day[d]) for d in range(0, 366, 3)])
        design = DashedLine(ypt(0, 8), ypt(365, 8), color=P_RED, stroke_width=2.5, dash_length=0.12)
        design_lbl = _m(r"\text{Heizlast} \approx 8\,\mathrm{kW}", ypt(200, 8) + UP * 0.2, color=P_RED,
                        size=LABEL_FONT_SIZE, edge="left")
        p_ticks2 = VGroup(*[_m(str(p), ypt(0, p) + LEFT * 0.22 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
                            for p in (2, 4, 6, 8)])
        self.play(FadeIn(yax["group"]), FadeIn(months), FadeIn(p_ticks2), run_time=0.8)
        self.play(Create(curve), run_time=1.6)
        self.play(Create(design), FadeIn(design_lbl), run_time=1.0)
        hold_for(self, N, "load")

        caption = swap_caption(self, caption, subtitle_text(N, "demand"))
        sweep = ValueTracker(0.0)

        def year_area():
            d_end = max(1, int(sweep.get_value()))
            pts = [ypt(0, 0)] + [ypt(d, kw_day[d]) for d in range(0, d_end + 1, 2)] + [ypt(d_end, 0)]
            return Polygon(*pts, stroke_width=0, fill_color=P_CYAN, fill_opacity=0.3)

        fill = always_redraw(year_area)
        q_read = _readout(lambda: rf"Q = {_de(cum[int(np.clip(sweep.get_value(), 0, 365))])}\,\mathrm{{kWh}}",
                          ypt(200, 6.2), color=P_CYAN, edge="left")
        self.add(fill, q_read)
        self.play(sweep.animate.set_value(365), run_time=4.0, rate_func=linear)
        year_lbl = _m(r"\text{Heizwärmebedarf} \approx 15\,000\,\mathrm{kWh/a}", ypt(200, 6.2), color=P_CYAN,
                      edge="left")
        self.play(ReplacementTransform(q_read, year_lbl), run_time=0.6)
        hold_for(self, N, "demand")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        summer_tag = Text("Sommer: Gewinne decken die Verluste", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        summer_tag.move_to(ypt(197, 2.2))
        gap = Line(ypt(121, 0) + UP * 0.08, ypt(273, 0) + UP * 0.08, color=P_YELLOW, stroke_width=5)
        self.play(Create(gap), FadeIn(summer_tag, shift=UP * 0.1), run_time=1.0)
        hold_for(self, N, "summer")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Energy is conserved: every watt ends up as heat
class Beat3_Energieerhaltung(Scene):
    NARRATION = [
        ("law",
         "Energy is not lost; it only changes form.",
         "Energie geht nicht verloren, sie wechselt nur die Form."),
        ("balance",
         "What goes into a room either leaves it again or stays stored in it.",
         "Was in einen Raum hineingeht, verlässt ihn wieder oder bleibt in ihm gespeichert."),
        ("lamp",
         "A lamp draws a hundred watts of electrical power.",
         "Eine Lampe nimmt 100 W elektrische Leistung auf."),
        ("split",
         "With an incandescent bulb about five percent becomes light and ninety-five percent heat straight away.",
         "Bei einer Glühlampe werden etwa 5 % Licht und 95 % sofort Wärme."),
        ("absorb",
         "The light hits walls, floor and furniture, is absorbed there — and becomes heat as well.",
         "Das Licht trifft Wände, Boden und Möbel, wird dort absorbiert — und wird ebenfalls Wärme."),
        ("total",
         "In the end all hundred watts are room heat — with LEDs too, and with every appliance.",
         "Am Ende sind alle 100 W Raumwärme. Das gilt auch für LEDs und für jedes Gerät."),
        ("person",
         "A resting person also gives off about a hundred watts.",
         "Auch ein ruhender Mensch gibt rund 100 W Wärme ab."),
        ("winter",
         "In winter this internal heat covers part of the losses and lowers the heating demand.",
         "Im Winter deckt diese innere Wärme einen Teil der Verluste und senkt den Heizwärmebedarf."),
        ("summer",
         "In summer it adds to the sun's heat and has to be removed by cooling.",
         "Im Sommer kommt sie zur Sonnenwärme hinzu und muss weggekühlt werden."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Energie bleibt erhalten — jedes Watt wird Wärme", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "law"))
        self.play(FadeIn(caption), run_time=0.3)

        #region law and balance
        room = _room(np.array([-2.3, 0.15, 0.0]), w=5.2, h=2.5)
        self.play(FadeIn(room["group"]), run_time=1.0)
        hold_for(self, N, "law", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0)

        caption = swap_caption(self, caption, subtitle_text(N, "balance"))
        mid_y = room["center"][1]
        e_in = Arrow(np.array([-6.9, mid_y + 0.3, 0.0]), np.array([room["glass_x"] - 0.05, mid_y + 0.3, 0.0]),
                     buff=0, color=P_CYAN, stroke_width=5)
        e_out = Arrow(np.array([room["x_r"] + 0.25, mid_y - 0.3, 0.0]), np.array([1.6, mid_y - 0.3, 0.0]),
                      buff=0, color=P_RED, stroke_width=5)
        fin, fout = ValueTracker(0.0), ValueTracker(0.0)
        bx, bw = 2.2, 4.2

        def bar(y, x0, frac, width, color):
            return Rectangle(width=max(0.002, width * frac), height=0.42, stroke_width=0, fill_color=color,
                             fill_opacity=0.7).move_to(np.array([x0 + width * frac / 2, y, 0.0]))

        in_bar = always_redraw(lambda: bar(1.0, bx, fin.get_value(), bw, P_CYAN))
        out_bar = always_redraw(lambda: bar(0.25, bx, fout.get_value(), bw * 0.7, P_RED))
        st_bar = always_redraw(lambda: bar(0.25, bx + bw * 0.7, fout.get_value(), bw * 0.3, P_ORANGE))
        in_t = Text("hinein", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([bx + 0.5, 1.48, 0.0]))
        out_t = Text("hinaus", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([bx + 0.5, -0.23, 0.0]))
        st_t = Text("gespeichert", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            np.array([bx + bw * 0.85, -0.23, 0.0]))
        row, box, items = _panel([
            ("in", r"E_{\mathrm{ein}}", P_CYAN), (None, "=", P_WHITE), ("out", r"E_{\mathrm{aus}}", P_RED),
            (None, "+", P_WHITE), ("st", r"\Delta E_{\mathrm{Speicher}}", P_ORANGE),
        ])
        self.add(in_bar, out_bar, st_bar)
        self.play(GrowArrow(e_in), fin.animate.set_value(1.0), FadeIn(in_t), run_time=1.2)
        self.play(GrowArrow(e_out), fout.animate.set_value(1.0), FadeIn(out_t), FadeIn(st_t),
                  room["ceiling"].animate.set_fill(P_ORANGE, opacity=0.55), run_time=1.4)
        self.play(FadeIn(row), Create(box), run_time=0.8)
        hold_for(self, N, "balance")
        #endregion

        #region lamp and split
        caption = swap_caption(self, caption, subtitle_text(N, "lamp"))
        self.play(FadeOut(VGroup(e_in, e_out, in_t, out_t, st_t, row, box)), FadeOut(in_bar), FadeOut(out_bar),
                  FadeOut(st_bar), room["ceiling"].animate.set_fill(P_WHITE, opacity=0.14), run_time=0.6)
        lamp = _lamp(np.array([room["center"][0], room["y_c"], 0.0]))
        p_el = ValueTracker(0.0)
        p_read = _readout(lambda: rf"P_{{\mathrm{{el}}}} = {_de(p_el.get_value())}\,\mathrm{{W}}",
                          lamp["cable"].get_center() + RIGHT * 0.3, color=P_CYAN, edge="left")
        self.play(FadeIn(lamp["group"]), run_time=0.6)
        self.add(p_read)
        _paced_flow(self, [([Line(lamp["cable"].get_start(), lamp["cable"].get_end())], P_CYAN, None, None, False)],
                    run_time=1.6, waves=3, cycles=2.0, radius=0.045, extra=[p_el.animate.set_value(100)])
        hold_for(self, N, "lamp")

        caption = swap_caption(self, caption, subtitle_text(N, "split"))
        src_x, lane_x0, lane_len, top_y, total_h = 1.5, 1.85, 2.5, 1.25, 1.9
        f_l, f_h = ValueTracker(0.0), ValueTracker(0.0)
        src = Rectangle(width=0.45, height=total_h, stroke_width=0, fill_color=P_CYAN, fill_opacity=0.6)
        src.move_to(np.array([src_x, top_y - total_h / 2, 0.0]))
        light_h, heat_h = total_h * 0.05, total_h * 0.95
        light_lane = always_redraw(lambda: Rectangle(
            width=max(0.002, lane_len * f_l.get_value()), height=light_h, stroke_width=0, fill_color=P_YELLOW,
            fill_opacity=0.8).move_to(np.array([lane_x0 + lane_len * f_l.get_value() / 2, top_y - light_h / 2, 0.0])))
        heat_lane = always_redraw(lambda: Rectangle(
            width=max(0.002, lane_len * f_h.get_value()), height=heat_h, stroke_width=0, fill_color=P_RED,
            fill_opacity=0.45).move_to(np.array([lane_x0 + lane_len * f_h.get_value() / 2,
                                                 top_y - light_h - 0.08 - heat_h / 2, 0.0])))
        light_read = _readout(lambda: rf"\text{{Licht}}\;{_de(5 * f_l.get_value())}\,\mathrm{{W}}",
                              np.array([lane_x0, top_y + 0.22, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW)
        heat_mid_y = top_y - light_h - 0.08 - heat_h / 2

        def riding_heat_label():
            edge = lane_x0 + lane_len * f_h.get_value()
            label = _m(rf"\text{{Wärme}}\;{_de(95 * f_h.get_value())}\,\mathrm{{W}}", size=LABEL_FONT_SIZE,
                       color=P_WHITE)
            label.move_to(np.array([edge - 0.15 - label.width / 2, heat_mid_y, 0.0]))
            if edge - lane_x0 < label.width + 0.35:
                label.set_opacity(0.0)
            return label

        heat_read = always_redraw(riding_heat_label)
        src_lbl = _m(r"100\,\mathrm{W}", np.array([src_x, top_y - total_h - 0.35, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_CYAN)
        bulb_c = lamp["bulb"].get_center()
        light_rays = VGroup(*[
            Line(bulb_c, end, color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.85)
            for end in (np.array([room["x_l"] + 0.05, bulb_c[1] - 0.3, 0.0]),
                        np.array([room["x_l"] + 1.0, room["y_f"] + 0.02, 0.0]),
                        np.array([room["center"][0] + 0.4, room["y_f"] + 0.02, 0.0]),
                        np.array([room["x_r"] - 0.6, room["y_f"] + 0.02, 0.0]),
                        np.array([room["x_r"] - 0.05, bulb_c[1] - 0.6, 0.0]))
        ])
        heat_waves = radiation_waves(bulb_c + DOWN * 0.15, n=3, color=P_RED, height=-0.6, x_spread=0.4)
        self.play(FadeIn(src), FadeIn(src_lbl), run_time=0.6)
        self.add(light_lane, heat_lane, light_read, heat_read)
        self.play(f_l.animate.set_value(1.0), f_h.animate.set_value(1.0), Create(heat_waves),
                  LaggedStart(*[Create(r) for r in light_rays], lag_ratio=0.1), run_time=2.0)
        p_row, p_box, p_items = _panel([
            ("el", r"P_{\mathrm{el}}", P_CYAN), (None, "=", P_WHITE), ("l", r"P_{\mathrm{Licht}}", P_YELLOW),
            (None, "+", P_WHITE), ("w", r"P_{\mathrm{Wärme}}", P_RED), (None, "=", P_WHITE),
            ("v", r"5\,\mathrm{W} + 95\,\mathrm{W}", P_WHITE),
        ])
        self.play(FadeIn(p_row), Create(p_box), run_time=0.8)
        hold_for(self, N, "split")
        #endregion

        #region absorb and total
        caption = swap_caption(self, caption, subtitle_text(N, "absorb"))
        flashes = [Flash(r.get_end(), color=P_ORANGE, flash_radius=0.18, line_length=0.12) for r in light_rays]
        self.play(LaggedStart(*flashes, lag_ratio=0.15), light_rays.animate.set_color(P_ORANGE), run_time=1.6)
        self.remove(light_lane)
        merged_light = Rectangle(width=lane_len, height=light_h, stroke_width=0, fill_color=P_YELLOW,
                                 fill_opacity=0.8).move_to(np.array([lane_x0 + lane_len / 2, top_y - light_h / 2, 0.0]))
        self.add(merged_light)
        light_heat = _m(r"\text{Licht} \to \text{Wärme}\;5\,\mathrm{W}", np.array([lane_x0, top_y + 0.22, 0.0]),
                        size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        self.play(merged_light.animate.set_fill(P_RED, opacity=0.45), FadeOut(light_rays),
                  ReplacementTransform(light_read, light_heat), run_time=1.0)
        hold_for(self, N, "absorb")

        caption = swap_caption(self, caption, subtitle_text(N, "total"))
        sink = Rectangle(width=0.45, height=total_h, stroke_width=0, fill_color=P_RED, fill_opacity=0.65)
        sink.move_to(np.array([lane_x0 + lane_len + 0.3, top_y - total_h / 2, 0.0]))
        sink_lbl = _m(r"100\,\mathrm{W}\;\text{Wärme}", np.array([sink.get_right()[0], top_y - total_h - 0.35, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_RED, edge="right")
        tot_row, tot_box, _ = _panel([
            (None, r"P_{\mathrm{el}}", P_CYAN), (None, "=", P_WHITE), (None, r"P_{\mathrm{Wärme}}", P_RED),
            (None, "=", P_WHITE), (None, r"100\,\mathrm{W}", P_RED),
        ])
        self.play(GrowFromEdge(sink, LEFT), FadeIn(sink_lbl), run_time=0.9)
        _swap_panel(self, (p_row, p_box), (tot_row, tot_box))
        animate_haze(self, run_time=1.6, cycles=1.2, x0=room["x_l"] + 0.3, x1=room["x_r"] - 0.3,
                     y0=room["y_f"] + 0.3, y1=room["y_c"] - 0.4, color=P_RED, color_end="#C9786E", n=26, seed=5)
        hold_for(self, N, "total")

        caption = swap_caption(self, caption, subtitle_text(N, "person"))
        person = _person(np.array([room["x_r"] - 0.9, room["y_f"] + 0.55, 0.0]), scale=1.9)
        person.shift(UP * (room["y_f"] + 0.08 - person.get_bottom()[1]))
        body_waves = radiation_waves(person.get_top() + UP * 0.05, n=3, color=P_ORANGE, height=0.55, x_spread=0.35)
        person_lbl = _m(r"\approx 100\,\mathrm{W}", person.get_left() + LEFT * 0.2 + UP * 0.05, size=LABEL_FONT_SIZE,
                        color=P_ORANGE, edge="right")
        self.play(FadeIn(person), Create(body_waves), FadeIn(person_lbl), run_time=1.0)
        hold_for(self, N, "person")
        #endregion

        #region winter and summer
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        self.play(FadeOut(VGroup(src, src_lbl, merged_light, sink, sink_lbl, tot_row, tot_box, heat_waves, light_heat)),
                  FadeOut(heat_lane), FadeOut(heat_read), run_time=0.6)
        loss_arrows = VGroup(*[
            Arrow(np.array([room["x_l"] + 0.3, y, 0.0]), np.array([room["x_l"] - 1.3, y, 0.0]), buff=0,
                  color=P_BLUE, stroke_width=3, max_tip_length_to_length_ratio=0.18)
            for y in (room["y_f"] + 0.25, room["y_c"] - 0.2)
        ] + [
            Arrow(np.array([room["x_r"] - 0.2, room["center"][1], 0.0]),
                  np.array([room["x_r"] + 1.0, room["center"][1], 0.0]), buff=0, color=P_BLUE, stroke_width=3,
                  max_tip_length_to_length_ratio=0.18)
        ])
        base_y, unit = -1.15, 0.0021
        f_int = ValueTracker(0.0)
        wx = 3.2

        def winter_bars():
            gi = 200 * f_int.get_value()
            heat = 1200 - gi
            low = Rectangle(width=0.8, height=max(0.002, gi * unit), stroke_width=0, fill_color=P_ORANGE,
                            fill_opacity=0.75).move_to(np.array([wx, base_y + gi * unit / 2, 0.0]))
            high = Rectangle(width=0.8, height=heat * unit, stroke_width=0, fill_color=P_RED, fill_opacity=0.6).move_to(
                np.array([wx, base_y + gi * unit + heat * unit / 2, 0.0]))
            return VGroup(low, high)

        w_bars = always_redraw(winter_bars)
        w_frame = Rectangle(width=0.8, height=1200 * unit, color=P_BLUE, stroke_width=2).move_to(
            np.array([wx, base_y + 1200 * unit / 2, 0.0]))
        w_title = Text("Winter", font_size=BODY_FONT_SIZE, color=P_BLUE).next_to(w_frame, UP, buff=0.15)
        heat_read2 = _readout(lambda: rf"\text{{Heizung}}\;{_de(1200 - 200 * f_int.get_value())}\,\mathrm{{W}}",
                              np.array([wx + 0.55, base_y + 1.6, 0.0]), size=LABEL_FONT_SIZE, color=P_RED)
        int_read = _readout(lambda: rf"\text{{intern}}\;{_de(200 * f_int.get_value())}\,\mathrm{{W}}",
                            np.array([wx + 0.55, base_y + 0.15, 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        loss_lbl = _m(r"\text{Verluste}\;1\,200\,\mathrm{W}", np.array([wx, base_y - 0.35, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_BLUE)
        self.play(LaggedStart(*[GrowArrow(a) for a in loss_arrows], lag_ratio=0.2), FadeIn(w_frame), FadeIn(w_title),
                  FadeIn(loss_lbl), run_time=1.0)
        self.add(w_bars, heat_read2, int_read)
        self.play(f_int.animate.set_value(1.0), Indicate(person, color=P_ORANGE), Indicate(lamp["bulb"]),
                  run_time=2.0)
        hold_for(self, N, "winter")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        sx = 6.05
        f_sun = ValueTracker(0.0)
        sun_c = np.array([-6.4, 2.05, 0.0])
        sun = _build_sun(sun_c).scale(0.32)

        def summer_bars():
            s = 500 * f_sun.get_value()
            low = Rectangle(width=0.8, height=200 * unit, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.75)
            low.move_to(np.array([sx, base_y + 200 * unit / 2, 0.0]))
            high = Rectangle(width=0.8, height=max(0.002, s * unit), stroke_width=0, fill_color=P_YELLOW,
                             fill_opacity=0.7).move_to(np.array([sx, base_y + 200 * unit + s * unit / 2, 0.0]))
            return VGroup(low, high)

        s_bars = always_redraw(summer_bars)
        s_title = Text("Sommer", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(
            np.array([sx, w_title.get_center()[1], 0.0]))
        cool_read = _readout(lambda: rf"\text{{Kühllast}}\;{_de(200 + 500 * f_sun.get_value())}\,\mathrm{{W}}",
                             np.array([sx, base_y - 0.35, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="center")
        rays = VGroup()
        for k in range(3):
            g_pt = np.array([room["glass_x"], room["win_hi"] - 0.2 - 0.35 * k, 0.0])
            start = sun_c + (g_pt - sun_c) / np.linalg.norm(g_pt - sun_c) * 0.42
            d = g_pt - start
            end = g_pt + d * ((room["y_f"] - g_pt[1]) / d[1])
            rays.add(solar_wave_ray(start, end, amp=0.045, cycles=np.linalg.norm(end - start) / 0.16))
        self.play(FadeOut(loss_arrows), FadeIn(sun), FadeIn(s_title), run_time=0.7)
        self.add(s_bars, cool_read)
        self.play(LaggedStart(*[Create(r) for r in rays], lag_ratio=0.2), f_sun.animate.set_value(1.0),
                  room["air"].animate.set_fill(P_RED, opacity=0.12), run_time=2.0)
        hold_for(self, N, "summer")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Heat pump: moving heat instead of making it
class Beat4_Waermepumpe(Scene):
    NARRATION = [
        ("move",
         "A heat pump does not create heat. It moves heat from the cold outdoor air into the warm house.",
         "Eine Wärmepumpe erzeugt keine Wärme. Sie verschiebt Wärme aus der kalten Außenluft ins warme Haus."),
        ("evap",
         "In the evaporator a very cold refrigerant takes up heat from the outdoor air and evaporates.",
         "Im Verdampfer nimmt ein sehr kaltes Kältemittel Wärme aus der Außenluft auf und verdampft."),
        ("comp",
         "The compressor squeezes the vapour — this needs electricity. Pressure and temperature rise.",
         "Der Verdichter presst den Dampf zusammen — dafür braucht er Strom. Druck und Temperatur steigen."),
        ("cond",
         "In the condenser the hot refrigerant gives its heat to the heating water and turns liquid again.",
         "Im Verflüssiger gibt das heiße Kältemittel seine Wärme ans Heizwasser ab und wird wieder flüssig."),
        ("valve",
         "The expansion valve lowers the pressure, the refrigerant becomes very cold again, and the cycle restarts.",
         "Das Expansionsventil senkt den Druck, das Kältemittel wird wieder sehr kalt — der Kreislauf beginnt neu."),
        ("balance",
         "The balance: one kilowatt-hour of electricity plus three of ambient heat gives four of heating heat.",
         "Die Bilanz: 1 kWh Strom plus 3 kWh Umweltwärme ergeben 4 kWh Heizwärme."),
        ("cop",
         "Heat divided by electricity is the coefficient of performance, COP — here four, measured to DIN EN 14511.",
         "Heizwärme durch Strom ist die Leistungszahl COP — hier 4. Gemessen wird sie nach DIN EN 14511."),
        ("temp",
         "The colder the outdoor air, the more the compressor has to lift — the COP drops.",
         "Je kälter die Außenluft, desto mehr muss der Verdichter leisten — der COP sinkt."),
        ("jaz",
         "Averaged over a year this is the seasonal performance factor JAZ — for air-source units often around three.",
         "Übers Jahr gemittelt ergibt sich die Jahresarbeitszahl JAZ — bei Luft-Wärmepumpen oft rund 3."),
        ("compare1",
         "Per kilowatt-hour of final energy: wood chips give about 0.85 kilowatt-hours of heat, oil about 0.94, gas about 0.96.",
         "Je 1 kWh Endenergie liefern Holzhackschnitzel rund 0,85 kWh Wärme, Heizöl rund 0,94, Erdgas rund 0,96."),
        ("compare2",
         "An electric heater gives exactly one. With fuels the rest leaves with the flue gas.",
         "Ein Elektroheizstab liefert genau 1. Bei Brennstoffen geht der Rest mit dem Abgas verloren."),
        ("compare3",
         "The air-source heat pump gives about three, the ground-source about four — because it adds ambient heat.",
         "Die Luft-Wärmepumpe liefert rund 3, die Erd-Wärmepumpe rund 4 — weil sie Umweltwärme dazuholt."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmepumpe — Wärme verschieben statt erzeugen", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "move"))
        self.play(FadeIn(caption), run_time=0.3)

        #region schema
        bl, tl, tr, br = (np.array([-3.2, -0.55, 0.0]), np.array([-3.2, 1.3, 0.0]),
                          np.array([3.0, 1.3, 0.0]), np.array([3.0, -0.55, 0.0]))
        comp_c, valve_c = np.array([-1.3, 1.3, 0.0]), np.array([-1.3, -0.55, 0.0])
        loop = VMobject(stroke_width=3, color=P_WHITE, stroke_opacity=0.55)
        loop.set_points_as_corners([bl, tl, comp_c, tr, br, valve_c, bl])
        loop_path = loop.copy()
        wall = DashedLine(np.array([0.6, -0.85, 0.0]), np.array([0.6, 2.1, 0.0]), color=P_WHITE, stroke_width=2,
                          dash_length=0.12)
        out_tag = Text("außen", font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(np.array([0.0, 2.15, 0.0]))
        in_tag = Text("innen", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(np.array([1.25, 2.15, 0.0]))
        cold_bg = Rectangle(width=7.5, height=2.95, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.06).move_to(
            np.array([-3.15, 0.62, 0.0]))

        def coil(x, y0, y1, color):
            pts = [np.array([x + (0.2 if k % 2 else -0.2), y, 0.0]) for k, y in enumerate(np.linspace(y0, y1, 9))]
            c = VMobject(color=color, stroke_width=3)
            c.set_points_as_corners(pts)
            return c

        evap = coil(-3.2, -0.3, 1.05, P_BLUE)
        cond = coil(3.0, 1.05, -0.3, P_RED)
        comp = VGroup(Circle(radius=0.34, color=P_WHITE, stroke_width=3, fill_color=P_DEEP_DARK, fill_opacity=1),
                      Line(LEFT * 0.24 + UP * 0.22, RIGHT * 0.3, color=P_WHITE, stroke_width=2.5),
                      Line(LEFT * 0.24 + DOWN * 0.22, RIGHT * 0.3, color=P_WHITE, stroke_width=2.5)).move_to(comp_c)
        valve = VGroup(Polygon(LEFT * 0.3 + UP * 0.2, LEFT * 0.3 + DOWN * 0.2, ORIGIN, color=P_WHITE, stroke_width=2.5,
                               fill_color=P_DEEP_DARK, fill_opacity=1),
                       Polygon(RIGHT * 0.3 + UP * 0.2, RIGHT * 0.3 + DOWN * 0.2, ORIGIN, color=P_WHITE,
                               stroke_width=2.5, fill_color=P_DEEP_DARK, fill_opacity=1)).move_to(valve_c)
        lbl_evap = Text("Verdampfer", font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(np.array([-4.5, 1.6, 0.0]))
        lbl_comp = Text("Verdichter", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(np.array([-0.15, 1.68, 0.0]))
        lbl_cond = Text("Verflüssiger", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([1.85, -0.1, 0.0]))
        lbl_valve = Text("Expansionsventil", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(
            np.array([-1.3, -0.1, 0.0]))
        air_rays = VGroup(*[solar_wave_ray(np.array([-6.5, y, 0.0]), np.array([-3.5, y, 0.0]), color=P_GREEN,
                                           amp=0.08, cycles=4) for y in (-0.15, 0.4, 0.95)])
        air_lbl = _m(r"\text{Außenluft}\;0\,\mathrm{°C}", np.array([-5.1, -0.6, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_GREEN)
        power = Arrow(np.array([-1.3, 2.3, 0.0]), comp.get_top(), buff=0.02, color=P_CYAN, stroke_width=5,
                      max_tip_length_to_length_ratio=0.3)
        power_lbl = Text("Strom", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(power, LEFT, buff=0.12)
        radiator = _radiator(np.array([5.2, 0.38, 0.0]))
        hot_rays = VGroup(*[solar_wave_ray(np.array([3.3, y, 0.0]), np.array([4.65, y, 0.0]), color=P_RED, amp=0.08,
                                           cycles=2.5) for y in (0.1, 0.65)])
        rad_lbl = _m(r"\text{Heizwasser}\;35\,\mathrm{°C}", np.array([5.2, -0.3, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_RED)
        refrigerant = _gradient([(0.0, P_BLUE), (0.13, P_CYAN), (0.24, P_CYAN), (0.27, P_RED), (0.5, P_RED),
                                 (0.63, P_ORANGE), (0.86, P_TEAL), (0.9, P_BLUE), (1.0, P_BLUE)])

        def cycle(run_time=3.0, extra=None):
            _paced_flow(self, [([loop_path], P_CYAN, None, refrigerant, True)], run_time=run_time, waves=22,
                        cycles=run_time / 3.0, radius=0.07, extra=extra)

        self.play(FadeIn(cold_bg), Create(wall), FadeIn(out_tag), FadeIn(in_tag), Create(loop), FadeIn(evap),
                  FadeIn(cond), FadeIn(comp), FadeIn(valve), FadeIn(radiator), run_time=1.4)
        cycle(2.4)
        hold_for(self, N, "move", used=BEAT_SUBTITLE_FADE + 0.3 + 3.8)

        caption = swap_caption(self, caption, subtitle_text(N, "evap"))
        self.play(FadeIn(lbl_evap), FadeIn(air_lbl), LaggedStart(*[Create(r) for r in air_rays], lag_ratio=0.2),
                  Indicate(evap, color=P_CYAN), run_time=1.2)
        cycle()
        hold_for(self, N, "evap")

        caption = swap_caption(self, caption, subtitle_text(N, "comp"))
        self.play(FadeIn(lbl_comp), GrowArrow(power), FadeIn(power_lbl), run_time=0.9)
        cycle(extra=[Rotate(comp[1:], angle=TAU * 2, about_point=comp_c)])
        hold_for(self, N, "comp")

        caption = swap_caption(self, caption, subtitle_text(N, "cond"))
        self.play(FadeIn(lbl_cond), LaggedStart(*[Create(r) for r in hot_rays], lag_ratio=0.2), FadeIn(rad_lbl),
                  Indicate(cond, color=P_RED), run_time=1.2)
        cycle()
        hold_for(self, N, "cond")

        caption = swap_caption(self, caption, subtitle_text(N, "valve"))
        self.play(FadeIn(lbl_valve), Indicate(valve, color=P_BLUE), run_time=0.9)
        cycle()
        hold_for(self, N, "valve")
        #endregion

        #region balance and cop
        caption = swap_caption(self, caption, subtitle_text(N, "balance"))
        unit, base = 0.55, -1.0
        self.play(FadeOut(radiator), FadeOut(rad_lbl), FadeOut(hot_rays), run_time=0.5)

        def block(x, h, color, width=0.55, y0=base):
            return Rectangle(width=width, height=h, stroke_width=1.5, color=color, fill_color=color,
                             fill_opacity=0.65).move_to(np.array([x, y0 + h / 2, 0.0]))

        stack_x = 5.75
        w_bar = block(stack_x, unit, P_CYAN, width=0.62)
        u_bar = block(stack_x, 3 * unit, P_GREEN, width=0.62, y0=base + unit)
        w_lbl = _m(r"W_{\mathrm{el}} = 1", np.array([stack_x - 0.5, base + unit / 2 - 0.07, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")
        u_lbl = _m(r"Q_{\mathrm{U}} = 3", np.array([stack_x - 0.5, base + 2.5 * unit - 0.07, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_GREEN, edge="right")
        self.play(Indicate(power, color=P_CYAN), GrowFromEdge(w_bar, DOWN), FadeIn(w_lbl), run_time=0.9)
        self.play(Indicate(air_rays, color=P_GREEN), GrowFromEdge(u_bar, DOWN), FadeIn(u_lbl), run_time=0.9)
        h_frame = Rectangle(width=0.72, height=4 * unit + 0.1, color=P_RED, stroke_width=3).move_to(
            np.array([stack_x, base + 2 * unit, 0.0]))
        h_lbl = _m(r"Q_{\mathrm{H}} = 4", h_frame.get_top() + UP * 0.2, size=LABEL_FONT_SIZE, color=P_RED)
        self.play(Create(h_frame), FadeIn(h_lbl), run_time=0.7)
        hold_for(self, N, "balance")

        caption = swap_caption(self, caption, subtitle_text(N, "cop"))
        tower = VGroup(*[block(6.65, unit, P_CYAN, width=0.4, y0=base + k * unit) for k in range(4)])
        count = ValueTracker(0)
        count_read = _readout(lambda: rf"\times\,{int(count.get_value())}", np.array([6.65, base + 4 * unit + 0.25, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
        self.add(count_read)
        for k in range(4):
            self.play(TransformFromCopy(w_bar, tower[k]), count.animate.set_value(k + 1), run_time=0.45)
        row, box, items = _panel([
            (None, r"\mathrm{COP}", P_TEAL), (None, "=", P_WHITE),
            ("f", r"\frac{\textcolor{#FF6B6B}{Q_{\mathrm{H}}}}{\textcolor{#66FCF1}{W_{\mathrm{el}}}}", P_WHITE),
            (None, "=", P_WHITE), (None, r"\frac{4\,\mathrm{kWh}}{1\,\mathrm{kWh}}", P_WHITE), (None, "=", P_WHITE),
            ("v", "4", P_YELLOW),
        ])
        self.play(FadeIn(row), Create(box), run_time=0.9)
        hold_for(self, N, "cop")
        #endregion

        #region temperature and jaz
        caption = swap_caption(self, caption, subtitle_text(N, "temp"))
        self.play(FadeOut(tower), FadeOut(count_read), FadeOut(w_bar), FadeOut(u_bar),
                  FadeOut(w_lbl), FadeOut(u_lbl), FadeOut(h_lbl), FadeOut(air_lbl), FadeOut(row), FadeOut(box),
                  run_time=0.6)
        t_out = ValueTracker(10.0)

        def cop_of(t):
            return 3.0 + 0.08 * t

        thermo = _thermometer(np.array([-5.9, -0.4, 0.0]), height=1.6, color=P_BLUE, level=0.5)
        thermo["level"].add_updater(lambda m: m.set_value((t_out.get_value() + 15) / 30))
        t_read = _readout(lambda: rf"{_de(t_out.get_value())}\,\mathrm{{°C}}", np.array([-5.55, 0.75, 0.0]),
                          size=LABEL_FONT_SIZE, color=P_BLUE)

        def live_stack():
            w = 4.0 / cop_of(t_out.get_value())
            low = block(stack_x, w * unit, P_CYAN, width=0.62)
            high = block(stack_x, (4.0 - w) * unit, P_GREEN, width=0.62, y0=base + w * unit)
            return VGroup(low, high)

        stack = always_redraw(live_stack)
        cop_read = _readout(lambda: rf"\mathrm{{COP}} \approx {_de(cop_of(t_out.get_value()), 1)}",
                            np.array([stack_x, base + 4 * unit + 0.3, 0.0]), color=P_YELLOW, edge="center")
        w_live = _readout(lambda: rf"W_{{\mathrm{{el}}}} = {_de(4.0 / cop_of(t_out.get_value()), 2)}\,\mathrm{{kWh}}",
                          np.array([5.3, base + 0.2, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")
        self.play(FadeOut(air_rays), FadeIn(thermo["group"]), FadeIn(thermo["column"]), run_time=0.6)
        self.add(stack, cop_read, t_read, w_live)
        self.bring_to_front(h_frame)
        self.play(t_out.animate.set_value(-10.0), run_time=3.0, rate_func=linear)
        hold_for(self, N, "temp")

        caption = swap_caption(self, caption, subtitle_text(N, "jaz"))
        self.play(t_out.animate.set_value(0.0), run_time=1.5)
        jaz = _m(r"\mathrm{JAZ} \approx 3 \quad \text{(VDI 4650)}", np.array([stack_x - 0.9, 1.95, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_TEAL)
        self.play(FadeIn(jaz, shift=DOWN * 0.1), run_time=0.7)
        hold_for(self, N, "jaz")
        #endregion

        #region compare
        caption = swap_caption(self, caption, subtitle_text(N, "compare1"))
        thermo["level"].clear_updaters()
        self.play(*[FadeOut(m) for m in self.mobjects if m not in (title, subtitle, caption)], run_time=0.7)
        cb, cu = -0.95, 0.66
        xs = [-5.0, -3.0, -1.0, 1.0, 3.0, 5.0]
        specs = [
            ("Holz-\nhackschnitzel", 0.85, "fuel"), ("Heizöl\nBrennwert", 0.94, "fuel"),
            ("Erdgas\nBrennwert", 0.96, "fuel"), ("Elektro-\nheizstab", 1.0, "el"),
            ("Luft-WP\n(JAZ)", 3.0, "hp"), ("Erd-WP\n(JAZ)", 4.0, "hp"),
        ]
        base_line = Line(np.array([-6.0, cb, 0.0]), np.array([6.2, cb, 0.0]), color=P_WHITE, stroke_width=2)
        one_line = DashedLine(np.array([-6.0, cb + cu, 0.0]), np.array([6.2, cb + cu, 0.0]), color=P_WHITE,
                              stroke_width=1.6, stroke_opacity=0.6, dash_length=0.1)
        one_lbl = _m(r"1\,\mathrm{kWh}", np.array([-6.9, cb + cu - 0.06, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE,
                     edge="left")
        names = VGroup(*[
            centered_body_text(name, font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(
                np.array([x, cb, 0.0]), DOWN, buff=0.14)
            for x, (name, _v, _k) in zip(xs, specs)
        ])
        axis_lbl = Text("Nutzwärme aus 1 kWh Endenergie", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([-3.0, 2.15, 0.0]))
        legend = VGroup(*[
            VGroup(Square(side_length=0.2, stroke_width=0, fill_color=c, fill_opacity=0.75),
                   Text(t, font_size=LABEL_FONT_SIZE, color=c)).arrange(RIGHT, buff=0.12)
            for t, c in (("Brennstoff", P_ORANGE), ("Strom", P_CYAN), ("Umweltwärme", P_GREEN))
        ]).arrange(RIGHT, buff=0.4).move_to(np.array([3.0, 2.15, 0.0]))
        grow = [ValueTracker(0.0) for _ in specs]

        def bar_for(i):
            def build():
                x, (_n, value, kind) = xs[i], specs[i]
                v = value * grow[i].get_value()
                if kind == "hp":
                    el = min(v, 1.0)
                    parts = [block(x, max(0.002, el * cu), P_CYAN, width=0.9, y0=cb)]
                    if v > 1.0:
                        parts.append(block(x, (v - 1.0) * cu, P_GREEN, width=0.9, y0=cb + cu))
                    return VGroup(*parts)
                return VGroup(block(x, max(0.002, v * cu), P_ORANGE if kind == "fuel" else P_CYAN, width=0.9, y0=cb))
            return always_redraw(build)

        def value_for(i):
            x, (_n, value, _k) = xs[i], specs[i]
            return _m(_de(value, 2 if value < 2 else 0), np.array([x, cb + value * cu - 0.26, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_WHITE)

        bars = [bar_for(i) for i in range(len(specs))]
        values = [value_for(i) for i in range(len(specs))]
        self.play(Create(base_line), Create(one_line), FadeIn(one_lbl), FadeIn(names), FadeIn(axis_lbl),
                  FadeIn(legend), run_time=0.9)
        self.add(*bars)
        self.play(*[grow[i].animate.set_value(1.0) for i in range(3)], run_time=1.6)
        self.play(*[FadeIn(values[i]) for i in range(3)], run_time=0.5)
        hold_for(self, N, "compare1")

        caption = swap_caption(self, caption, subtitle_text(N, "compare2"))
        losses = VGroup(*[
            DashedVMobject(Rectangle(width=0.9, height=(1 - specs[i][1]) * cu, color=P_ORANGE, stroke_width=1.6).move_to(
                np.array([xs[i], cb + (1 + specs[i][1]) / 2 * cu, 0.0])), num_dashes=14)
            for i in range(3)
        ])
        loss_lbl = Text("Abgas", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(np.array([-4.0, cb + cu + 0.55, 0.0]))
        self.play(grow[3].animate.set_value(1.0), run_time=1.0)
        self.play(FadeIn(values[3]), run_time=0.4)
        self.play(Create(losses), FadeIn(loss_lbl), run_time=0.8)
        hold_for(self, N, "compare2")

        caption = swap_caption(self, caption, subtitle_text(N, "compare3"))
        self.play(grow[4].animate.set_value(1.0), run_time=1.4)
        self.play(FadeIn(values[4]), run_time=0.4)
        self.play(grow[5].animate.set_value(1.0), run_time=1.4)
        self.play(FadeIn(values[5]), run_time=0.4)
        hold_for(self, N, "compare3")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Radiation: short-wave in, long-wave trapped
class Beat5_Strahlung(Scene):
    NARRATION = [
        ("sun",
         "The sun radiates short-wave: visible light and near infrared. Glass lets most of it through.",
         "Die Sonne strahlt kurzwellig: sichtbares Licht und nahes Infrarot. Glas lässt das größtenteils durch."),
        ("g",
         "The share that passes is the g-value — for triple glazing about half (DIN EN 410).",
         "Welcher Anteil hindurchkommt, beschreibt der g-Wert — bei Dreifachverglasung rund die Hälfte."),
        ("gain",
         "Solar gain is g times window area times irradiance: half of two square metres at 500 watts per square metre.",
         "Solarer Gewinn: g mal Fensterfläche mal Einstrahlung — 0,5 · 2 m² · 500 W/m² = 500 W."),
        ("absorb",
         "Inside, the radiation hits floor and walls. The surfaces warm up.",
         "Drinnen trifft die Strahlung auf Boden und Wände. Die Oberflächen erwärmen sich."),
        ("long",
         "Warm surfaces radiate too — but long-wave, in the far infrared.",
         "Warme Oberflächen strahlen selbst — aber langwellig, im fernen Infrarot."),
        ("trap",
         "Glass hardly lets long-wave radiation through; a low-e coating reflects it back into the room.",
         "Langwellige Strahlung lässt Glas kaum durch; eine Low-E-Beschichtung wirft sie in den Raum zurück."),
        ("warm",
         "The energy stays trapped and the room warms up — the greenhouse effect.",
         "Die Energie bleibt gefangen, der Raum erwärmt sich — der Treibhauseffekt."),
        ("winter",
         "In winter this gain saves heating energy.",
         "Im Winter spart dieser Gewinn Heizwärme."),
        ("summer",
         "In summer it overheats the room — so shade outside, before the radiation reaches the glass (DIN 4108-2).",
         "Im Sommer überhitzt er den Raum — darum Sonnenschutz außen, bevor die Strahlung das Glas erreicht."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Strahlung — kurzwellig hinein, langwellig gefangen", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "sun"))
        self.play(FadeIn(caption), run_time=0.3)

        #region sun and g
        room = _room(np.array([1.0, 0.25, 0.0]), w=4.6, h=2.6, window=(0.22, 0.92))
        sun_c = np.array([-4.5, 1.6, 0.0])
        sun = _build_sun(sun_c).scale(0.42)
        gx = room["glass_x"]
        glass_ys = np.linspace(room["win_lo"] + 0.15, room["win_hi"] - 0.1, 8)
        passing = [1, 3, 4, 6]
        sun_dir = np.array([1.0, -0.5, 0.0]) / np.linalg.norm([1.0, -0.5])
        bounce_dir = np.array([-sun_dir[0], sun_dir[1], 0.0])

        def hit_point(g_pt):
            t_floor = (room["y_f"] + 0.02 - g_pt[1]) / sun_dir[1]
            t_wall = (room["x_r"] - 0.02 - g_pt[0]) / sun_dir[0]
            return g_pt + sun_dir * min(t_floor, t_wall)

        beams_out, beams_in, reflected, hits = VGroup(), VGroup(), VGroup(), []
        pass_paths, block_paths, starts = [], [], []
        for k, y in enumerate(glass_ys):
            g_pt = np.array([gx - 0.06, y, 0.0])
            start = g_pt - sun_dir * ((g_pt[0] - (sun_c[0] + 0.6)) / sun_dir[0])
            starts.append(start)
            beams_out.add(_beam(start, g_pt))
            if k in passing:
                end = hit_point(g_pt)
                hits.append(end)
                beams_in.add(_beam(g_pt, end))
                pass_paths.append([start, end])
            else:
                back = g_pt + bounce_dir * 0.7
                reflected.add(_beam(g_pt, back, opacity=0.4, width=1.8))
                block_paths.append([start, g_pt, back])
        n_in, n_pass = ValueTracker(0), ValueTracker(0)
        g_read = _readout(
            lambda: (rf"g = \frac{{{int(n_pass.get_value())}}}{{{max(1, int(n_in.get_value()))}}}"
                     rf" = {_de(int(n_pass.get_value()) / max(1, int(n_in.get_value())), 2)}"),
            np.array([4.6, 1.95, 0.0]), color=P_YELLOW, edge="center")
        self.play(FadeIn(room["group"]), FadeIn(sun), run_time=1.0)
        self.play(LaggedStart(*[Create(b) for b in beams_out], lag_ratio=0.08), run_time=1.0)
        self.play(LaggedStart(*[Create(b) for b in beams_in], lag_ratio=0.1), run_time=0.8)
        _beam_pulses(self, pass_paths, P_YELLOW, repeats=2, run_time=3.0)
        hold_for(self, N, "sun", used=BEAT_SUBTITLE_FADE + 0.3 + 5.8)

        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        self.add(g_read)
        self.play(Create(reflected), run_time=0.6)
        _beam_pulses(self, pass_paths + block_paths, P_YELLOW, repeats=1, run_time=2.6,
                     extra=[n_in.animate.set_value(8), n_pass.animate.set_value(4)])
        norm_tag = Text("DIN EN 410", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([4.6, 1.3, 0.0]))
        self.play(FadeIn(norm_tag), run_time=0.4)
        hold_for(self, N, "g")
        #endregion

        #region gain
        caption = swap_caption(self, caption, subtitle_text(N, "gain"))
        pane_c = np.array([5.6, 0.1, 0.0])
        pane = Rectangle(width=0.9, height=1.15, color=P_CYAN, stroke_width=2.5).move_to(pane_c)
        a_frac = ValueTracker(0.0)
        pane_fill = always_redraw(lambda: Rectangle(
            width=0.9, height=max(0.002, 1.15 * a_frac.get_value()), stroke_width=0, fill_color=P_CYAN,
            fill_opacity=0.35).move_to(pane.get_bottom() + UP * 1.15 * a_frac.get_value() / 2))
        w_dim = dim_arrow(pane.get_corner(DL) + DOWN * 0.18, pane.get_corner(DR) + DOWN * 0.18, color=P_WHITE)
        h_dim = dim_arrow(pane.get_corner(DL) + LEFT * 0.18, pane.get_corner(UL) + LEFT * 0.18, color=P_WHITE)
        w_lbl = _m(r"1{,}25\,\mathrm{m}", w_dim.get_center() + DOWN * 0.32, size=LABEL_FONT_SIZE)
        h_lbl = _m(r"1{,}6\,\mathrm{m}", size=LABEL_FONT_SIZE).next_to(h_dim, LEFT, buff=0.08)
        a_read = _readout(lambda: rf"A = {_de(2.0 * a_frac.get_value(), 1)}\,\mathrm{{m^{{2}}}}",
                          pane.get_top() + UP * 0.3, size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
        i_lbl = _m(r"I = 500\,\mathrm{W/m^{2}}", sun_c + LEFT * 1.55 + DOWN * 0.08, size=LABEL_FONT_SIZE,
                   color=P_YELLOW)
        self.play(FadeIn(pane), GrowFromCenter(w_dim), GrowFromCenter(h_dim), FadeIn(w_lbl), FadeIn(h_lbl),
                  FadeIn(i_lbl), run_time=0.9)
        self.add(pane_fill, a_read)
        self.play(a_frac.animate.set_value(1.0), run_time=1.4)
        row, box, items = _panel([
            (None, r"\dot{Q}_{\mathrm{S}}", P_ORANGE), (None, "=", P_WHITE), ("g", "g", P_YELLOW),
            (None, r"\cdot", P_WHITE), ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE), ("i", "I", P_YELLOW),
            (None, "=", P_WHITE), ("gv", r"0{,}5", P_YELLOW), (None, r"\cdot", P_WHITE),
            ("av", r"2{,}0\,\mathrm{m^{2}}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("iv", r"500\,\mathrm{W/m^{2}}", P_YELLOW), (None, "=", P_WHITE), ("r", r"500\,\mathrm{W}", P_ORANGE),
        ])
        _fill_panel(self, (row, box, items), {"gv": g_read, "av": a_read, "iv": i_lbl}, fly=False)
        hold_for(self, N, "gain")
        #endregion

        #region absorb, long-wave and trap
        caption = swap_caption(self, caption, subtitle_text(N, "absorb"))
        self.play(FadeOut(VGroup(pane, w_dim, h_dim, w_lbl, h_lbl, norm_tag)), FadeOut(pane_fill), FadeOut(a_read),
                  run_time=0.5)
        _beam_pulses(self, pass_paths, P_YELLOW, repeats=1, run_time=2.0, extra=[
            LaggedStart(*[Flash(p, color=P_ORANGE, flash_radius=0.12, line_length=0.08) for p in hits],
                        lag_ratio=0.2),
            room["floor"].animate.set_fill(P_ORANGE, opacity=0.55),
            room["wall_r"].animate.set_fill(P_ORANGE, opacity=0.35),
        ])
        hold_for(self, N, "absorb")

        caption = swap_caption(self, caption, subtitle_text(N, "long"))
        ir_up, ir_back, up_paths, trap_paths = VGroup(), VGroup(), [], []
        for k, p in enumerate(hits):
            src = p + UP * 0.04
            g_in = np.array([gx + 0.1, room["win_lo"] + 0.3 + 0.42 * k, 0.0])
            bounce = _mirror((g_in - src) / np.linalg.norm(g_in - src), PI / 2)
            back_end = g_in + bounce * min(1.6, (room["y_c"] - 0.12 - g_in[1]) / max(0.05, bounce[1]))
            ir_up.add(_beam(src, g_in, color=P_RED, opacity=0.35, width=1.8))
            ir_back.add(_beam(g_in, back_end, color=P_RED, opacity=0.35, width=1.8))
            up_paths.append([src, g_in])
            trap_paths.append([src, g_in, back_end])
        lw_lbl = Text("langwellig", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(
            np.array([room["center"][0] + 0.6, room["y_c"] + room["slab"] + 0.22, 0.0]))
        sw_lbl = Text("kurzwellig", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([-3.3, 0.2, 0.0]))
        self.play(beams_in.animate.set_stroke(opacity=0.3), LaggedStart(*[Create(b) for b in ir_up], lag_ratio=0.15),
                  FadeIn(lw_lbl), FadeIn(sw_lbl), run_time=1.0)
        _beam_pulses(self, up_paths, P_RED, repeats=2, run_time=2.6)
        hold_for(self, N, "long")

        caption = swap_caption(self, caption, subtitle_text(N, "trap"))
        lowe = Line(np.array([gx + 0.1, room["win_lo"], 0.0]), np.array([gx + 0.1, room["win_hi"], 0.0]),
                    color=P_ORANGE, stroke_width=4)
        lowe_lbl = Text("Low-E", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            np.array([gx - 0.55, room["win_lo"] - 0.3, 0.0]))
        self.play(Create(lowe), FadeIn(lowe_lbl), LaggedStart(*[Create(b) for b in ir_back], lag_ratio=0.15),
                  run_time=1.0)
        _beam_pulses(self, trap_paths, P_RED, repeats=2, run_time=3.0)
        hold_for(self, N, "trap")

        caption = swap_caption(self, caption, subtitle_text(N, "warm"))
        t_room = ValueTracker(20.0)
        t_read = _readout(lambda: rf"\vartheta_{{\mathrm{{Raum}}}} = {_de(t_room.get_value())}\,\mathrm{{°C}}",
                          np.array([room["x_r"] - 0.15, room["y_c"] - 0.4, 0.0]), size=LABEL_FONT_SIZE,
                          color=P_RED, edge="right")
        self.add(t_read)
        self.play(t_room.animate.set_value(26.0), room["air"].animate.set_fill(P_RED, opacity=0.14), run_time=2.0)
        hold_for(self, N, "warm")
        #endregion

        #region winter and summer
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        radiator = _radiator(np.array([room["x_r"] - 0.65, room["y_f"] + 0.45, 0.0]))
        rad_waves = radiation_waves(radiator.get_top() + UP * 0.05, n=3, color=P_RED, height=0.5, x_spread=0.5)
        heat_need = ValueTracker(1000.0)
        need_read = _readout(lambda: rf"\text{{Heizung}}\;{_de(heat_need.get_value())}\,\mathrm{{W}}",
                             np.array([4.6, -0.2, 0.0]), size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        self.play(FadeIn(radiator), Create(rad_waves), run_time=0.7)
        self.add(need_read)
        self.play(heat_need.animate.set_value(500.0), rad_waves.animate.set_stroke(opacity=0.35),
                  Indicate(items["r"], color=P_ORANGE), run_time=1.6)
        hold_for(self, N, "winter")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        slat_angle = 50 * DEGREES
        slat_x = gx - 0.62
        slat_ys = np.arange(room["win_hi"] + 0.05, room["win_lo"] - 0.1, -0.3)
        slats = VGroup(*[
            Rectangle(width=0.52, height=0.11, color=P_WHITE, stroke_width=2, fill_color=P_TEAL,
                      fill_opacity=1.0).move_to(np.array([slat_x, y, 0.0])).rotate(slat_angle)
            for y in slat_ys
        ])
        rail = Rectangle(width=0.62, height=0.22, color=P_WHITE, stroke_width=2, fill_color=P_TEAL,
                         fill_opacity=1.0).move_to(np.array([slat_x, room["win_hi"] + 0.42, 0.0]))
        cords = VGroup(*[Line(rail.get_bottom() + RIGHT * dx, np.array([slat_x + dx, slat_ys[-1], 0.0]),
                              color=P_WHITE, stroke_width=1.2, stroke_opacity=0.6) for dx in (-0.12, 0.12)])
        blind = VGroup(rail, cords, slats)
        blind_lbl = Text("Raffstore", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([slat_x - 0.1, room["win_lo"] - 0.35, 0.0]))
        bounce_up = _mirror(sun_dir, slat_angle)
        stopped, shade_paths = VGroup(), []
        for start in starts:
            stop = start + sun_dir * ((slat_x - 0.05 - start[0]) / sun_dir[0])
            stopped.add(_beam(start, stop))
            back = stop + bounce_up * 0.6
            stopped.add(_beam(stop, back, opacity=0.4, width=1.8))
            shade_paths.append([start, stop, back])
        norm2 = Text("DIN 4108-2", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([-3.7, -1.0, 0.0]))
        self.play(FadeOut(radiator), FadeOut(rad_waves), FadeOut(need_read), FadeOut(lowe_lbl), run_time=0.4)
        self.play(FadeIn(rail), Create(cords), LaggedStart(*[FadeIn(s, shift=DOWN * 0.25) for s in slats],
                                                            lag_ratio=0.12),
                  FadeIn(blind_lbl), run_time=1.2)
        self.play(ReplacementTransform(beams_out, stopped), FadeOut(beams_in), FadeOut(ir_up), FadeOut(ir_back),
                  FadeOut(reflected), t_room.animate.set_value(22.0), room["air"].animate.set_fill(P_RED, opacity=0.04),
                  room["floor"].animate.set_fill(P_WHITE, opacity=0.14), FadeIn(norm2), run_time=1.4)
        _beam_pulses(self, shade_paths, P_YELLOW, repeats=2, run_time=2.6)
        hold_for(self, N, "summer")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Thermal mass: storing heat and releasing it later
class Beat6_ThermischeMasse(Scene):
    NARRATION = [
        ("store",
         "Building elements store heat. How much depends on mass, material and temperature change.",
         "Bauteile speichern Wärme. Wie viel, hängt von Masse, Material und Temperaturänderung ab."),
        ("mass",
         "One square metre of concrete ceiling, ten centimetres active depth: a tenth of a cubic metre — 240 kilograms.",
         "1 m² Betondecke mit 10 cm wirksamer Schicht: 0,1 m³ — das sind 240 kg."),
        ("heat",
         "Warmed by two kelvin it stores 480 kilojoules, about 0.13 kilowatt-hours.",
         "Um 2 K erwärmt speichert sie 480 kJ — rund 0,13 kWh."),
        ("room",
         "A twenty-square-metre ceiling holds about 2.7 kilowatt-hours — almost three hours of vacuuming.",
         "Eine Decke mit 20 m² nimmt so rund 2,7 kWh auf — fast drei Stunden Staubsaugen."),
        ("day",
         "During the day a massive ceiling absorbs heat that would otherwise warm the room air.",
         "Tagsüber nimmt eine massive Decke Wärme auf, die sonst die Raumluft aufheizen würde."),
        ("lag",
         "It releases it hours later — phase-shifted, and with a smaller swing.",
         "Sie gibt die Wärme Stunden später wieder ab — phasenverschoben und mit kleinerer Schwankung."),
        ("ceiling",
         "The ceiling has the greatest effect: it is free of furniture and carpets, and warm air rises to it.",
         "Die Decke wirkt am stärksten: Sie ist frei von Möbeln und Teppichen, und warme Luft steigt zu ihr auf."),
        ("view",
         "And it exchanges radiation with every surface in the room.",
         "Außerdem tauscht sie mit jeder Fläche im Raum Strahlung aus."),
        ("summer",
         "In summer it takes the peaks and is discharged at night by ventilation.",
         "Im Sommer fängt sie die Spitzen ab und wird nachts durch Lüften entladen."),
        ("winter",
         "In winter it stores the day's solar gains into the evening.",
         "Im Winter speichert sie die Sonnengewinne des Tages bis in den Abend."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Thermische Masse — speichern und zeitversetzt abgeben", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "store"))
        self.play(FadeIn(caption), run_time=0.3)

        #region store, mass, heat
        fl = np.array([-4.9, -0.15, 0.0])
        W, D, depth = 3.0, 0.3, np.array([0.95, 0.6, 0.0])
        front = Polygon(fl, fl + RIGHT * W, fl + RIGHT * W + UP * D, fl + UP * D, color=P_WHITE, stroke_width=2.5,
                        fill_color=P_WHITE, fill_opacity=0.18)
        top = Polygon(fl + UP * D, fl + RIGHT * W + UP * D, fl + RIGHT * W + UP * D + depth, fl + UP * D + depth,
                      color=P_WHITE, stroke_width=2.5, fill_color=P_WHITE, fill_opacity=0.28)
        side = Polygon(fl + RIGHT * W, fl + RIGHT * W + depth, fl + RIGHT * W + UP * D + depth, fl + RIGHT * W + UP * D,
                       color=P_WHITE, stroke_width=2.5, fill_color=P_WHITE, fill_opacity=0.1)
        slab = VGroup(front, top, side)
        d_w = dim_arrow(fl + DOWN * 0.22, fl + RIGHT * W + DOWN * 0.22, color=P_WHITE)
        d_w_lbl = _m(r"1\,\mathrm{m}", d_w.get_center() + DOWN * 0.34, size=LABEL_FONT_SIZE)
        d_d = dim_arrow(fl + RIGHT * W + RIGHT * 0.25, fl + RIGHT * W + RIGHT * 0.25 + depth, color=P_WHITE)
        d_d_lbl = _m(r"1\,\mathrm{m}", size=LABEL_FONT_SIZE).next_to(d_d, RIGHT, buff=0.06)
        d_t = dim_arrow(fl + LEFT * 0.22, fl + LEFT * 0.22 + UP * D, color=P_ORANGE)
        d_t_lbl = _m(r"0{,}10\,\mathrm{m}", size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(d_t, LEFT, buff=0.08)
        self.play(FadeIn(slab), run_time=1.0)
        hold_for(self, N, "store", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0)

        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        v_line = _m(r"V = A \cdot d = 1\,\mathrm{m^{2}} \cdot 0{,}10\,\mathrm{m} = 0{,}1\,\mathrm{m^{3}}",
                    np.array([0.3, 1.55, 0.0]), size=BODY_FONT_SIZE, edge="left")
        m_line = _m(r"m = \rho \cdot V = 2\,400\,\mathrm{kg/m^{3}} \cdot 0{,}1\,\mathrm{m^{3}} = 240\,\mathrm{kg}",
                    np.array([0.3, 0.95, 0.0]), size=BODY_FONT_SIZE, color=P_CYAN, edge="left")
        self.play(GrowFromCenter(d_w), FadeIn(d_w_lbl), GrowFromCenter(d_d), FadeIn(d_d_lbl), GrowFromCenter(d_t),
                  FadeIn(d_t_lbl), run_time=1.0)
        self.play(FadeIn(v_line, shift=RIGHT * 0.1), run_time=0.8)
        self.play(FadeIn(m_line, shift=RIGHT * 0.1), slab.animate.set_fill(P_CYAN, opacity=0.25), run_time=0.8)
        hold_for(self, N, "mass")

        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        dT = ValueTracker(0.0)
        thermo = _thermometer(np.array([-6.3, -0.75, 0.0]), height=1.6, color=P_ORANGE)
        thermo["level"].add_updater(lambda m: m.set_value(dT.get_value() / 2.4))
        dt_read = _readout(lambda: rf"\Delta T = {_de(dT.get_value(), 1)}\,\mathrm{{K}}", np.array([-6.0, 1.15, 0.0]),
                           size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        q_read = _readout(
            lambda: (rf"Q = 240\,\mathrm{{kg}} \cdot 1\,000\,\mathrm{{J/(kg\,K)}} \cdot {_de(dT.get_value(), 1)}\,"
                     rf"\mathrm{{K}} = {_de(240 * dT.get_value())}\,\mathrm{{kJ}}"),
            np.array([0.3, 0.35, 0.0]), size=BODY_FONT_SIZE, color=P_ORANGE, edge="left")
        kwh_read = _readout(lambda: rf"\approx {_de(240 * dT.get_value() / 3600, 2)}\,\mathrm{{kWh}}",
                            np.array([0.3, -0.25, 0.0]), size=BODY_FONT_SIZE, color=P_YELLOW, edge="left")
        row, box, items = _panel([
            (None, "Q", P_ORANGE), (None, "=", P_WHITE), ("m", "m", P_CYAN), (None, r"\cdot", P_WHITE),
            ("c", "c", P_WHITE), (None, r"\cdot", P_WHITE), ("dt", r"\Delta T", P_ORANGE),
        ])
        self.play(FadeIn(thermo["group"]), FadeIn(thermo["column"]), FadeIn(row), Create(box), run_time=0.8)
        self.add(dt_read, q_read, kwh_read)
        self.play(dT.animate.set_value(2.0), slab.animate.set_fill(P_ORANGE, opacity=0.45), run_time=2.4)
        hold_for(self, N, "heat")

        caption = swap_caption(self, caption, subtitle_text(N, "room"))
        thermo["level"].clear_updaters()
        self.play(FadeOut(VGroup(d_w, d_w_lbl, d_d, d_d_lbl, d_t, d_t_lbl, v_line, m_line, thermo["group"],
                                 thermo["column"], dt_read)), run_time=0.5)
        tile = 0.36
        grid_origin = np.array([-5.6, -0.95, 0.0])
        tiles = VGroup(*[
            Square(side_length=tile * 0.92, color=P_ORANGE, stroke_width=1.6, fill_color=P_ORANGE, fill_opacity=0.45)
            .move_to(grid_origin + RIGHT * tile * (c + 0.5) + UP * tile * (r + 0.5))
            for r in range(4) for c in range(5)
        ])
        n_tiles = ValueTracker(1)
        total_read = _readout(
            lambda: rf"{int(n_tiles.get_value())}\,\mathrm{{m^{{2}}}} \cdot 0{{,}}133\,\mathrm{{kWh}} = "
                    rf"{_de(0.1333 * n_tiles.get_value(), 1)}\,\mathrm{{kWh}}",
            np.array([-4.7, 0.85, 0.0]), size=BODY_FONT_SIZE, color=P_YELLOW, edge="center")
        self.play(ReplacementTransform(slab, tiles[0]), FadeOut(q_read), FadeOut(kwh_read), run_time=1.0)
        self.add(total_read)
        self.play(LaggedStart(*[TransformFromCopy(tiles[0], t) for t in tiles[1:]], lag_ratio=0.08),
                  n_tiles.animate.set_value(20), run_time=2.4)
        vac = _vacuum().scale(0.8).move_to(np.array([0.8, -0.6, 0.0]))
        vac_lbl = _m(r"\approx 2{,}7\,\mathrm{h}\;\text{Staubsaugen}", np.array([2.7, -0.7, 0.0]), color=P_TEAL,
                     edge="left")
        self.play(FadeIn(vac), FadeIn(vac_lbl), run_time=0.7)
        hold_for(self, N, "room")
        #endregion

        #region day and lag
        caption = swap_caption(self, caption, subtitle_text(N, "day"))
        self.play(FadeOut(VGroup(tiles, total_read, vac, vac_lbl, row, box)), run_time=0.6)
        CO = np.array([-6.3, -1.25, 0.0])
        CX, CY = 6.4 / 24.0, 0.145

        def cpt(h, temp):
            return CO + RIGHT * h * CX + UP * (temp - 14) * CY

        def t_out(h):
            return 24 + 8 * np.sin(TAU * (h - 9) / 24)

        def t_light(h):
            return 25 + 5 * np.sin(TAU * (h - 10.5) / 24)

        def t_heavy(h):
            return 25 + 2 * np.sin(TAU * (h - 15) / 24)

        cax = _axes(CO, 7.15, 3.05, r"t\;[\mathrm{h}]", r"\vartheta\;[\mathrm{°C}]")
        h_ticks = VGroup(*[_m(str(h), cpt(h, 14) + DOWN * 0.32, size=LABEL_FONT_SIZE) for h in (0, 6, 12, 18, 24)])
        clock_h = ValueTracker(0.0)

        def curve(fn, color):
            def build():
                hs = np.linspace(0, max(0.05, clock_h.get_value()), 80)
                c = VMobject(color=color, stroke_width=3)
                c.set_points_smoothly([cpt(h, fn(h)) for h in hs])
                return c
            return always_redraw(build)

        c_out, c_light, c_heavy = curve(t_out, P_YELLOW), curve(t_light, P_ORANGE), curve(t_heavy, P_CYAN)
        legend = VGroup(*[
            VGroup(Line(LEFT * 0.2, RIGHT * 0.2, color=c, stroke_width=3), Text(t, font_size=LABEL_FONT_SIZE, color=c))
            .arrange(RIGHT, buff=0.12)
            for t, c in (("außen", P_YELLOW), ("Raum, leicht", P_ORANGE), ("Raum, massive Decke", P_CYAN))
        ]).arrange(RIGHT, buff=0.35).move_to(np.array([-2.7, 2.3, 0.0]))
        cursor = always_redraw(lambda: DashedLine(cpt(clock_h.get_value(), 14), cpt(clock_h.get_value(), 34.5),
                                                  color=P_WHITE, stroke_width=1.5, stroke_opacity=0.5))
        small = _room(np.array([4.3, 0.25, 0.0]), w=3.4, h=1.9, slab=0.36, window=(0.3, 0.85))

        def ceiling_heat():
            f = float(np.clip((t_heavy(clock_h.get_value()) - 23) / 4, 0, 1))
            return small["ceiling"].copy().set_fill(interpolate_color(ManimColor(P_WHITE), ManimColor(P_ORANGE), f),
                                                    opacity=0.2 + 0.55 * f)

        ceil_live = always_redraw(ceiling_heat)
        sky_sun = _build_sun(np.array([4.3, 2.05, 0.0])).scale(0.22)
        sky_moon = _moon(np.array([4.3, 2.05, 0.0]))

        def sky():
            h = clock_h.get_value() % 24
            day = 6 <= h <= 19
            return (sky_sun if day else sky_moon).copy()

        sky_live = always_redraw(sky)
        hour_read = _readout(lambda: rf"{int(clock_h.get_value()) % 24:02d}{{:}}00\,\mathrm{{Uhr}}",
                             np.array([6.2, 2.05, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE, edge="right")
        self.play(FadeIn(cax["group"]), FadeIn(h_ticks), FadeIn(legend), FadeIn(small["group"]), run_time=0.9)
        self.add(c_out, c_light, c_heavy, cursor, ceil_live, sky_live, hour_read)
        self.play(clock_h.animate.set_value(24.0), run_time=7.0, rate_func=linear)
        hold_for(self, N, "day")

        caption = swap_caption(self, caption, subtitle_text(N, "lag"))
        p_out, p_heavy = cpt(15, 32), cpt(21, 27)
        v1 = DashedLine(p_out, cpt(15, 14), color=P_YELLOW, stroke_width=1.6, dash_length=0.08)
        v2 = DashedLine(p_heavy, cpt(21, 14), color=P_CYAN, stroke_width=1.6, dash_length=0.08)
        shift_arrow = dim_arrow(cpt(15, 33.3), cpt(21, 33.3), color=P_TEAL)
        phi = ValueTracker(0.0)
        phi_read = _readout(lambda: rf"\varphi \approx {_de(phi.get_value())}\,\mathrm{{h}}", cpt(18, 33.9),
                            size=LABEL_FONT_SIZE, color=P_TEAL, edge="center")
        amp_out = dim_arrow(cpt(23.2, 16), cpt(23.2, 32), color=P_YELLOW)
        amp_heavy = dim_arrow(cpt(23.9, 23), cpt(23.9, 27), color=P_CYAN)
        amp_lbl = _m(r"16\,\mathrm{K} \to 4\,\mathrm{K}", cpt(24.3, 24.0), size=LABEL_FONT_SIZE, color=P_WHITE,
                     edge="left")
        self.remove(cursor)
        self.play(Create(v1), Create(v2), run_time=0.7)
        self.add(phi_read)
        self.play(GrowFromCenter(shift_arrow), phi.animate.set_value(6.0), run_time=1.4)
        self.play(GrowFromCenter(amp_out), GrowFromCenter(amp_heavy), FadeIn(amp_lbl), run_time=1.0)
        hold_for(self, N, "lag")
        #endregion

        #region ceiling and view
        caption = swap_caption(self, caption, subtitle_text(N, "ceiling"))
        self.play(FadeOut(VGroup(cax["group"], h_ticks, legend, small["group"], v1, v2, shift_arrow, amp_out, amp_heavy,
                                 amp_lbl)),
                  *[FadeOut(m) for m in (c_out, c_light, c_heavy, ceil_live, sky_live, hour_read, phi_read)],
                  run_time=0.6)
        big = _room(np.array([-1.3, 0.05, 0.0]), w=5.6, h=2.4, slab=0.34, window=(0.3, 0.85))
        sofa = RoundedRectangle(width=1.5, height=0.5, corner_radius=0.1, color=P_TEAL, stroke_width=2,
                                fill_color=P_TEAL, fill_opacity=0.3).move_to(
            np.array([big["x_l"] + 1.3, big["y_f"] + 0.25, 0.0]))
        rug = Rectangle(width=1.6, height=0.06, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.6).move_to(
            np.array([big["center"][0] + 0.6, big["y_f"] + 0.03, 0.0]))
        cabinet = Rectangle(width=0.55, height=1.3, color=P_TEAL, stroke_width=2, fill_color=P_TEAL,
                            fill_opacity=0.3).move_to(np.array([big["x_r"] - 0.3, big["y_f"] + 0.65, 0.0]))
        person = _person(np.array([big["center"][0] + 0.4, big["y_f"] + 0.5, 0.0]), scale=1.8)
        person.shift(UP * (big["y_f"] + 0.08 - person.get_bottom()[1]))
        rise_starts = (np.array([person.get_center()[0], person.get_top()[1] + 0.12, 0.0]),
                       np.array([big["center"][0] - 1.0, big["y_f"] + 0.95, 0.0]))
        rising = VGroup(*[
            convection_stream(start, np.array([start[0] + 0.3, big["y_c"] - 0.08, 0.0]), color=P_RED, n_ribbons=2,
                              bend=0.15)
            for start in rise_starts
        ])
        rise_paths = [smooth_path([start, np.array([start[0] + 0.15, (start[1] + big["y_c"]) / 2, 0.0]),
                                   np.array([start[0] + 0.3, big["y_c"] - 0.1, 0.0])])
                      for start in rise_starts]
        ceil_fill = ValueTracker(0.2)
        big["ceiling"].add_updater(lambda m: m.set_fill(P_ORANGE, opacity=ceil_fill.get_value()))
        tags, tag_leaders = side_labels([
            (big["ceiling"].get_right() + LEFT * 0.1, "frei von Möbeln und Teppich", P_ORANGE),
            (rising[0].get_top(), "warme Luft steigt auf", P_RED),
            (big["wall_r"].get_center(), "Strahlungsaustausch mit allen Flächen", P_YELLOW),
        ], x=2.05, align="left", min_gap=0.55)
        norm = Text("DIN 4108-2 · DIN EN ISO 13786", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([4.4, -1.35, 0.0]))
        self.play(FadeIn(big["group"]), FadeIn(sofa), FadeIn(rug), FadeIn(cabinet), FadeIn(person), run_time=1.0)
        self.play(FadeIn(tags[0]), Create(tag_leaders[0]), Indicate(sofa, color=P_TEAL), Indicate(rug),
                  run_time=1.0)
        self.play(FadeIn(tags[1]), Create(tag_leaders[1]), Create(rising), run_time=0.9)
        _paced_flow(self, [(rise_paths, P_RED, None, None, False)], run_time=2.2, waves=5, cycles=2.0,
                    extra=[ceil_fill.animate.set_value(0.45)])
        hold_for(self, N, "ceiling")

        caption = swap_caption(self, caption, subtitle_text(N, "view"))
        ceil_mid_y = big["y_c"] - 0.05
        exchange = VGroup(*[
            solar_wave_ray(start, np.array([start[0] * 0.4 + big["center"][0] * 0.6, ceil_mid_y, 0.0]), color=P_YELLOW,
                           amp=0.07, cycles=3, stroke_width=2)
            for start in (np.array([big["x_l"] + 0.1, big["center"][1] - 0.3, 0.0]),
                          np.array([big["center"][0] - 0.4, big["y_f"] + 0.08, 0.0]),
                          np.array([big["x_r"] - 0.7, big["y_f"] + 0.08, 0.0]),
                          np.array([big["x_r"] - 0.05, big["center"][1] + 0.4, 0.0]))
        ])
        self.play(FadeIn(tags[2]), Create(tag_leaders[2]), LaggedStart(*[Create(r) for r in exchange],
                                                                     lag_ratio=0.15),
                  ceil_fill.animate.set_value(0.6), FadeIn(norm), run_time=1.8)
        hold_for(self, N, "view")
        #endregion

        #region summer and winter
        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        self.play(FadeOut(VGroup(tags, tag_leaders, rising, exchange, sofa, rug, cabinet)), run_time=0.5)
        sky_pos = np.array([-5.6, 1.9, 0.0])
        sun = _build_sun(sky_pos).scale(0.35)
        moon = _moon(sky_pos)
        heat_in = radiation_waves(np.array([big["center"][0], big["y_c"] - 0.95, 0.0]), n=4, color=P_RED, height=0.75,
                                  x_spread=1.6)
        self.play(FadeIn(sun), Create(heat_in), ceil_fill.animate.set_value(0.85), run_time=1.6)
        tilt = 28 * DEGREES
        open_glass = big["glass"].copy().rotate(-tilt, about_point=np.array([big["glass_x"], big["win_lo"], 0.0]))
        sash_top = big["win_lo"] + (big["win_hi"] - big["win_lo"]) * np.cos(tilt)
        gap_ys = np.linspace(sash_top + 0.05, big["win_hi"] - 0.04, 2)
        night_paths = [smooth_path([np.array([-6.6, y - 0.25, 0.0]), np.array([big["glass_x"] - 0.4, y, 0.0]),
                                    np.array([big["glass_x"] + 0.5, y + 0.02, 0.0]),
                                    np.array([big["center"][0], big["y_c"] - 0.15, 0.0]),
                                    np.array([big["x_r"] - 0.3, big["y_c"] - 0.3, 0.0])])
                       for y in gap_ys]
        self.play(ReplacementTransform(sun, moon), FadeOut(heat_in), Transform(big["glass"], open_glass), run_time=0.9)
        _paced_flow(self, [(night_paths, P_BLUE, None, None, False)], run_time=2.6, waves=6, cycles=2.0,
                    extra=[ceil_fill.animate.set_value(0.2)])
        hold_for(self, N, "summer")

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        closed_glass = _room(np.array([-1.3, 0.05, 0.0]), w=5.6, h=2.4, slab=0.34, window=(0.3, 0.85))["glass"]
        sun2 = _build_sun(sky_pos).scale(0.35)
        winter_rays = VGroup(*[
            solar_wave_ray(sky_pos + np.array([0.4, -0.3 - 0.12 * k, 0.0]),
                           np.array([big["center"][0] - 0.6 + 0.5 * k, big["y_f"] + 0.05, 0.0]), amp=0.04,
                           cycles=16, stroke_width=2)
            for k in range(3)
        ])
        self.play(ReplacementTransform(moon, sun2), Transform(big["glass"], closed_glass), run_time=0.8)
        self.play(LaggedStart(*[Create(r) for r in winter_rays], lag_ratio=0.2), ceil_fill.animate.set_value(0.75),
                  run_time=1.6)
        evening = _moon(sky_pos)
        down_waves = radiation_waves(np.array([big["center"][0], big["y_c"] - 0.1, 0.0]), n=4, color=P_ORANGE,
                                     height=-0.8, x_spread=1.8)
        self.play(ReplacementTransform(sun2, evening), FadeOut(winter_rays), Create(down_waves),
                  ceil_fill.animate.set_value(0.3), run_time=2.0)
        big["ceiling"].clear_updaters()
        hold_for(self, N, "winter")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 – Sensible and latent heat
def _x_sat(t_c: float) -> float:
    """💧 Saturation humidity ratio in g/kg (Magnus formula, 1013 hPa)."""
    p_s = 611.2 * np.exp(17.62 * t_c / (243.12 + t_c))
    return 622.0 * p_s / (101325.0 - p_s)


def _p_sat(t_c: float) -> float:
    return 611.2 * np.exp(17.62 * t_c / (243.12 + t_c))


class Beat7_SensibelLatent(Scene):
    NARRATION = [
        ("sensible",
         "Sensible heat changes the temperature — you can measure it with a thermometer.",
         "Sensible Wärme ändert die Temperatur — man kann sie mit dem Thermometer messen."),
        ("water",
         "Heating one kilogram of water from zero to a hundred degrees takes about 419 kilojoules.",
         "1 kg Wasser von 0 auf 100 °C zu erwärmen kostet rund 419 kJ."),
        ("latent",
         "At a hundred degrees the temperature stops, although energy keeps flowing in — it goes into evaporation.",
         "Bei 100 °C bleibt die Temperatur stehen, obwohl weiter Energie hineingeht — sie steckt im Verdampfen."),
        ("ratio",
         "This latent heat, about 2,257 kilojoules per kilogram, is more than five times the heating.",
         "Diese latente Wärme, rund 2 257 kJ pro kg, ist mehr als fünfmal so groß wie das Erwärmen."),
        ("air",
         "Air carries water vapour. How much it can hold at most depends strongly on temperature.",
         "Luft trägt Wasserdampf. Wie viel sie höchstens aufnehmen kann, hängt stark von der Temperatur ab."),
        ("curve",
         "At zero degrees about four grams per kilogram of air, at twenty about fifteen, at thirty about twenty-seven.",
         "Bei 0 °C sind es rund 4 g pro kg Luft, bei 20 °C rund 15 g, bei 30 °C rund 27 g."),
        ("room",
         "Room air at twenty degrees and fifty percent relative humidity holds about seven grams.",
         "Raumluft mit 20 °C und 50 % relativer Feuchte enthält rund 7 g Wasser pro kg."),
        ("dew",
         "If it cools, the relative humidity rises — at about nine degrees it reaches one hundred percent: the dew point.",
         "Kühlt sie ab, steigt die relative Feuchte — bei rund 9 °C sind es 100 %: der Taupunkt."),
        ("condense",
         "Below that, water condenses — on a cold window pane, for example.",
         "Darunter fällt Wasser aus — zum Beispiel an einer kalten Fensterscheibe."),
        ("person",
         "A resting person gives off about seventy watts sensibly and thirty latently — about 45 grams of water per hour.",
         "Ein ruhender Mensch gibt rund 70 W sensibel und 30 W latent ab — etwa 45 g Wasser pro Stunde."),
        ("relevance",
         "In winter, condensation threatens on cold surfaces; in summer, cooling must also dehumidify.",
         "Im Winter droht an kalten Flächen Tauwasser (DIN 4108-3); im Sommer muss die Kühlung auch entfeuchten."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Sensible und latente Wärme", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "sensible"))
        self.play(FadeIn(caption), run_time=0.3)

        #region sensible and latent
        QO = np.array([-6.3, -0.95, 0.0])
        QX, QY = 7.0 / 2800.0, 2.9 / 125.0

        def qpt(q, t):
            return QO + RIGHT * q * QX + UP * t * QY

        def temp_of(q):
            return min(100.0, q / 4.19)

        qax = _axes(QO, 7.4, 3.15, r"Q\;[\mathrm{kJ}]", r"T\;[\mathrm{°C}]")
        t_ticks = VGroup(*[_m(str(t), qpt(0, t) + LEFT * 0.2 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
                           for t in (50, 100)])
        Q = ValueTracker(0.0)

        def path_upto():
            q_end = max(1.0, Q.get_value())
            qs = [q for q in (0.0, 419.0) if q < q_end] + [q_end]
            c = VMobject(color=P_RED, stroke_width=3.5)
            c.set_points_as_corners([qpt(q, temp_of(q)) for q in qs])
            return c

        heat_path = always_redraw(path_upto)
        head = always_redraw(lambda: Dot(qpt(Q.get_value(), temp_of(Q.get_value())), radius=0.08, color=P_YELLOW))
        pot_c = np.array([3.6, -0.35, 0.0])
        pot = VGroup(
            Line(pot_c + LEFT * 0.8 + UP * 0.55, pot_c + LEFT * 0.8 + DOWN * 0.55, color=P_WHITE, stroke_width=3),
            Line(pot_c + LEFT * 0.8 + DOWN * 0.55, pot_c + RIGHT * 0.8 + DOWN * 0.55, color=P_WHITE, stroke_width=3),
            Line(pot_c + RIGHT * 0.8 + DOWN * 0.55, pot_c + RIGHT * 0.8 + UP * 0.55, color=P_WHITE, stroke_width=3),
        )
        water = Rectangle(width=1.5, height=0.7, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.5).move_to(
            pot_c + DOWN * 0.18)
        flames = VGroup(*[
            Polygon(pot_c + DOWN * 0.72 + RIGHT * (dx - 0.1), pot_c + DOWN * 0.72 + RIGHT * (dx + 0.1),
                    pot_c + DOWN * 0.95 + RIGHT * dx, color=P_ORANGE, fill_color=P_ORANGE, fill_opacity=0.7,
                    stroke_width=1.5).rotate(PI, about_point=pot_c + DOWN * 0.835 + RIGHT * dx)
            for dx in (-0.45, 0.0, 0.45)
        ])
        mass_tag = _m(r"m = 1\,\mathrm{kg}", pot_c + UP * 0.85, size=LABEL_FONT_SIZE, color=P_BLUE)
        q_read = _readout(lambda: rf"Q = {_de(Q.get_value())}\,\mathrm{{kJ}}", np.array([2.2, 1.85, 0.0]),
                          color=P_ORANGE)
        t_read = _readout(lambda: rf"T = {_de(temp_of(Q.get_value()))}\,\mathrm{{°C}}", np.array([2.2, 1.3, 0.0]),
                          color=P_RED)
        self.play(FadeIn(qax["group"]), FadeIn(t_ticks), FadeIn(pot), FadeIn(water), FadeIn(flames),
                  FadeIn(mass_tag), run_time=1.0)
        self.add(heat_path, head, q_read, t_read)
        self.play(Q.animate.set_value(210.0), water.animate.set_fill(interpolate_color(
            ManimColor(P_BLUE), ManimColor(P_RED), 0.5)), run_time=2.0, rate_func=linear)
        hold_for(self, N, "sensible", used=BEAT_SUBTITLE_FADE + 0.3 + 3.0)

        caption = swap_caption(self, caption, subtitle_text(N, "water"))
        row, box, _ = _panel([
            (None, "Q", P_ORANGE), (None, "=", P_WHITE), (None, "m", P_BLUE), (None, r"\cdot", P_WHITE),
            (None, "c", P_WHITE), (None, r"\cdot", P_WHITE), (None, r"\Delta T", P_RED), (None, "=", P_WHITE),
            (None, r"1\,\mathrm{kg} \cdot 4{,}19\,\mathrm{kJ/(kg\,K)} \cdot 100\,\mathrm{K}", P_WHITE),
            (None, r"\approx", P_WHITE), (None, r"419\,\mathrm{kJ}", P_ORANGE),
        ])
        self.play(Q.animate.set_value(419.0), water.animate.set_fill(P_RED, opacity=0.5), run_time=1.6,
                  rate_func=linear)
        self.play(FadeIn(row), Create(box), run_time=0.8)
        hold_for(self, N, "water")

        caption = swap_caption(self, caption, subtitle_text(N, "latent"))
        bubble_paths = [Line(pot_c + DOWN * 0.45 + RIGHT * dx, pot_c + UP * 0.6 + RIGHT * dx * 1.2) for dx in
                        (-0.5, -0.2, 0.15, 0.45)]
        _paced_flow(self, [(bubble_paths, P_WHITE, None, None, False)], run_time=4.0, waves=3, cycles=4.0,
                    radius=0.05, extra=[Q.animate.set_value(2676.0), water.animate.stretch_to_fit_height(0.3).shift(
                        DOWN * 0.2)])
        hold_for(self, N, "latent")

        caption = swap_caption(self, caption, subtitle_text(N, "ratio"))
        sens = dim_arrow(qpt(0, 108), qpt(419, 108), color=P_ORANGE)
        lat = dim_arrow(qpt(419, 108), qpt(2676, 108), color=P_BLUE)
        sens_lbl = _m(r"419\,\mathrm{kJ}", qpt(210, 113), size=LABEL_FONT_SIZE, color=P_ORANGE)
        lat_lbl = _m(r"2\,257\,\mathrm{kJ}", qpt(1550, 113), size=LABEL_FONT_SIZE, color=P_BLUE)
        ratio = ValueTracker(0.0)
        ratio_read = _readout(lambda: rf"\frac{{2\,257}}{{419}} \approx {_de(ratio.get_value(), 1)}",
                              qpt(1550, 70), color=P_YELLOW, edge="center")
        lat_panel = _panel([
            (None, r"Q_{\mathrm{lat}}", P_BLUE), (None, "=", P_WHITE), (None, "m", P_BLUE), (None, r"\cdot", P_WHITE),
            (None, "r", P_WHITE), (None, "=", P_WHITE),
            (None, r"1\,\mathrm{kg} \cdot 2\,257\,\mathrm{kJ/kg}", P_WHITE),
        ])
        self.play(GrowFromCenter(sens), FadeIn(sens_lbl), GrowFromCenter(lat), FadeIn(lat_lbl), run_time=1.0)
        self.add(ratio_read)
        self.play(ratio.animate.set_value(2257 / 419), run_time=1.4)
        _swap_panel(self, (row, box), lat_panel[:2])
        hold_for(self, N, "ratio")
        #endregion

        #region air and dew point
        caption = swap_caption(self, caption, subtitle_text(N, "air"))
        self.play(FadeOut(VGroup(qax["group"], t_ticks, pot, water, flames, mass_tag, sens, lat, sens_lbl, lat_lbl,
                                 lat_panel[0], lat_panel[1])),
                  *[FadeOut(m) for m in (heat_path, head, q_read, t_read, ratio_read)], run_time=0.6)
        AO = np.array([-6.0, -1.2, 0.0])
        AX, AY = 5.8 / 45.0, 0.1

        def apt(t, x):
            return AO + RIGHT * (t + 10) * AX + UP * x * AY

        aax = _axes(AO, 6.2, 3.25, r"\vartheta\;[\mathrm{°C}]", r"x\;[\mathrm{g/kg}]")
        t_lbls = VGroup(*[_m(str(t).replace("-", "−"), apt(t, 0) + DOWN * 0.32, size=LABEL_FONT_SIZE)
                          for t in (-10, 0, 10, 20, 30)])
        x_lbls = VGroup(*[_m(str(x), apt(-10, x) + LEFT * 0.2 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
                          for x in (10, 20, 30)])
        sat = VMobject(color=P_CYAN, stroke_width=3.5)
        sat.set_points_smoothly([apt(t, _x_sat(t)) for t in np.linspace(-10, 31.5, 60)])
        sat_lbl = Text("Sättigung (100 %)", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(apt(17, 24))
        box_c = np.array([3.6, 0.35, 0.0])
        air_box = Square(side_length=1.9, color=P_WHITE, stroke_width=2.5).move_to(box_c)
        air_tag = Text("1 kg Luft", font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(air_box, UP, buff=0.12)
        rng = np.random.default_rng(11)
        vapour = VGroup(*[Dot(box_c + np.array([rng.uniform(-0.82, 0.82), rng.uniform(-0.82, 0.82), 0.0]), radius=0.055,
                              color=P_BLUE) for _ in range(27)])
        self.play(FadeIn(aax["group"]), FadeIn(t_lbls), FadeIn(x_lbls), FadeIn(air_box), FadeIn(air_tag), run_time=0.9)
        self.play(Create(sat), FadeIn(sat_lbl), run_time=1.8)
        hold_for(self, N, "air")

        caption = swap_caption(self, caption, subtitle_text(N, "curve"))
        marks = VGroup()
        for t, txt in ((0, r"\approx 4\,\mathrm{g}"), (20, r"\approx 15\,\mathrm{g}"), (30, r"\approx 27\,\mathrm{g}")):
            p = apt(t, _x_sat(t))
            marks.add(VGroup(Dot(p, radius=0.07, color=P_CYAN),
                             _m(txt, p + LEFT * 0.15 + UP * 0.12, size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")))
        for k, mark in enumerate(marks):
            n_show = (4, 15, 27)[k]
            self.play(FadeIn(mark), FadeIn(vapour[:n_show]), FadeOut(vapour[n_show:]), run_time=0.7)
        hold_for(self, N, "curve")

        caption = swap_caption(self, caption, subtitle_text(N, "room"))
        x_room = _x_sat(20) * 0.5 * (101325 - _p_sat(20)) / (101325 - 0.5 * _p_sat(20))
        T = ValueTracker(20.0)
        p_v = 0.5 * _p_sat(20)

        def x_now():
            t = T.get_value()
            return min(x_room, _x_sat(t))

        state = always_redraw(lambda: Dot(apt(T.get_value(), x_now()), radius=0.1, color=P_YELLOW))
        phi_gauge = meter("φ", length=1.9, thickness=0.36, color=P_BLUE)
        phi_gauge["group"].move_to(box_c + RIGHT * 1.6 + DOWN * 0.12)
        phi_gauge["fill"].add_updater(lambda m: set_meter(phi_gauge, min(1.0, p_v / _p_sat(T.get_value()))))
        phi_read = _readout(lambda: rf"\varphi = {_de(min(100.0, 100 * p_v / _p_sat(T.get_value())))}\,\%",
                            box_c + DOWN * 1.35, color=P_BLUE, edge="center")
        st_read = _readout(lambda: rf"{_de(T.get_value(), 1)}\,\mathrm{{°C}},\; x = {_de(x_now(), 1)}\,\mathrm{{g/kg}}",
                           np.array([-0.4, 2.05, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="center")
        self.play(FadeOut(marks), FadeOut(vapour[7:]), FadeIn(phi_gauge["group"]), run_time=0.7)
        self.add(state, phi_read, st_read)
        hold_for(self, N, "room")

        caption = swap_caption(self, caption, subtitle_text(N, "dew"))
        self.play(T.animate.set_value(9.3), run_time=2.6, rate_func=linear)
        dew_lbl = Text("Taupunkt", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(apt(9.3, x_room), UP + LEFT,
                                                                                      buff=0.3)
        self.play(FadeIn(dew_lbl), Flash(apt(9.3, x_room), color=P_YELLOW, flash_radius=0.14, line_length=0.08),
                  run_time=0.8)
        hold_for(self, N, "dew")

        caption = swap_caption(self, caption, subtitle_text(N, "condense"))
        pane = Rectangle(width=1.9, height=0.35, color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN,
                         fill_opacity=0.15).move_to(box_c + DOWN * 1.9)
        pane_lbl = Text("kalte Scheibe", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(pane, LEFT, buff=0.15)
        self.play(FadeOut(phi_read), FadeIn(pane), FadeIn(pane_lbl), run_time=0.5)
        falling = vapour[4:7]
        self.play(T.animate.set_value(0.0), falling.animate.move_to(pane.get_center()).set_color(P_CYAN),
                  run_time=2.4, rate_func=linear)
        drops = _droplets(pane.get_center(), n=7, spread=(0.8, 0.1), color=P_CYAN)
        self.play(ReplacementTransform(falling, drops), run_time=0.6)
        hold_for(self, N, "condense")
        #endregion

        #region person and relevance
        caption = swap_caption(self, caption, subtitle_text(N, "person"))
        phi_gauge["fill"].clear_updaters()
        self.play(FadeOut(VGroup(aax["group"], t_lbls, x_lbls, sat, sat_lbl, air_box, air_tag, vapour[:4], dew_lbl,
                                 pane, pane_lbl, drops, phi_gauge["group"])), FadeOut(state), FadeOut(st_read),
                  run_time=0.6)
        person = _person(np.array([-4.2, -0.35, 0.0]), scale=2.6)
        breath = respiration_parts(person[0].get_right() + RIGHT * 0.05, scale=1.6)
        f = ValueTracker(0.0)
        bb, bu = -1.0, 0.03

        def person_bars():
            s, l_ = 70 * f.get_value(), 30 * f.get_value()
            return VGroup(
                Rectangle(width=0.9, height=max(0.002, s * bu), stroke_width=0, fill_color=P_RED,
                          fill_opacity=0.7).move_to(np.array([0.2, bb + s * bu / 2, 0.0])),
                Rectangle(width=0.9, height=max(0.002, l_ * bu), stroke_width=0, fill_color=P_BLUE,
                          fill_opacity=0.7).move_to(np.array([1.6, bb + l_ * bu / 2, 0.0])),
            )

        bars = always_redraw(person_bars)
        s_read = _readout(lambda: rf"{_de(70 * f.get_value())}\,\mathrm{{W}}\;\text{{sensibel}}",
                          np.array([0.2, bb + 2.1 + 0.25, 0.0]), size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        l_read = _readout(lambda: rf"{_de(30 * f.get_value())}\,\mathrm{{W}}\;\text{{latent}}",
                          np.array([1.6, bb + 0.9 + 0.25, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE, edge="center")
        g_read = _readout(lambda: rf"\approx {_de(45 * f.get_value())}\,\mathrm{{g/h}}\;\text{{Wasser}}",
                          np.array([3.2, bb + 0.45, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE, edge="left")
        base = Line(np.array([-0.6, bb, 0.0]), np.array([2.4, bb, 0.0]), color=P_WHITE, stroke_width=2)
        self.play(FadeIn(person), FadeIn(breath["group"]), Create(base), run_time=0.9)
        self.add(bars, s_read, l_read, g_read)
        self.play(f.animate.set_value(1.0), run_time=2.0)
        hold_for(self, N, "person")

        caption = swap_caption(self, caption, subtitle_text(N, "relevance"))
        self.play(FadeOut(VGroup(person, breath["group"], base)),
                  *[FadeOut(m) for m in (bars, s_read, l_read, g_read)], run_time=0.5)
        w_pane = Rectangle(width=0.35, height=2.0, color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN,
                           fill_opacity=0.12).move_to(np.array([-3.6, 0.35, 0.0]))
        w_drops = _droplets(w_pane.get_center(), n=10, spread=(0.12, 0.85), color=P_CYAN, seed=5)
        w_lbl = Text("Winter: Tauwasser an kalten Flächen", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(
            w_pane, DOWN, buff=0.25)
        coil_pts = [np.array([2.6 + 0.3 * k, 1.0 if k % 2 else 0.0, 0.0]) for k in range(8)]
        cool_coil = VMobject(color=P_BLUE, stroke_width=3)
        cool_coil.set_points_as_corners(coil_pts)
        tray = Line(np.array([2.5, -0.85, 0.0]), np.array([4.8, -0.85, 0.0]), color=P_WHITE, stroke_width=3)
        drip_paths = [Line(np.array([2.9 + 0.6 * k, -0.05, 0.0]), np.array([2.9 + 0.6 * k, -0.8, 0.0])) for k in range(3)]
        s_lbl = Text("Sommer: Kühlung entfeuchtet", font_size=LABEL_FONT_SIZE, color=P_BLUE).next_to(tray, DOWN,
                                                                                                    buff=0.3)
        self.play(FadeIn(w_pane), FadeIn(w_drops, lag_ratio=0.2), FadeIn(w_lbl), Create(cool_coil), Create(tray),
                  FadeIn(s_lbl), run_time=1.2)
        _paced_flow(self, [(drip_paths, P_CYAN, None, None, False)], run_time=2.4, waves=2, cycles=3.0, radius=0.05)
        hold_for(self, N, "relevance")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – Venturi effect: air in motion
class Beat8_Venturi(Scene):
    NARRATION = [
        ("narrow",
         "Air flowing through a narrowing must pass the same amount in the same time — so it speeds up.",
         "Strömt Luft durch eine Engstelle, muss in gleicher Zeit gleich viel hindurch — sie wird schneller."),
        ("continuity",
         "Half the cross-section, twice the speed.",
         "Halber Querschnitt, doppelte Geschwindigkeit."),
        ("pressure",
         "Where the air is faster, the pressure drops. That is the Venturi effect.",
         "Wo die Luft schneller ist, sinkt der Druck. Das ist der Venturi-Effekt."),
        ("suction",
         "The narrowing sucks: air is drawn in through a side opening.",
         "Die Engstelle saugt: Durch eine seitliche Öffnung wird Luft angesaugt."),
        ("roof",
         "On a building the same happens: wind speeds up over the ridge and creates suction on the roof and leeward side.",
         "Am Gebäude passiert dasselbe: Über dem First beschleunigt der Wind und erzeugt Sog an Dach und Leeseite."),
        ("openings",
         "An inlet low on the windward side and an outlet high in the suction zone draw air across the house.",
         "Eine Zuluftöffnung unten auf der Windseite und eine Abluftöffnung oben im Sog ziehen Luft durchs Haus."),
        ("size",
         "The smaller opening limits the flow: a larger outlet raises the air volume.",
         "Die kleinere Öffnung begrenzt den Strom: Eine größere Abluftöffnung erhöht den Volumenstrom."),
        ("loss",
         "Each cubic metre takes heat with it: 0.34 watt-hours per cubic metre and kelvin.",
         "Jeder Kubikmeter nimmt Wärme mit: 0,34 Wh pro m³ und Kelvin."),
        ("winter",
         "In winter, a hundred cubic metres per hour at twenty kelvin difference is about 680 watts — seven people's worth.",
         "Im Winter sind 100 m³/h bei 20 K Unterschied rund 680 W — so viel wie sieben Menschen."),
        ("night",
         "So ventilate small and controlled in winter, and open wide at summer nights — that discharges the ceiling.",
         "Darum im Winter klein und kontrolliert lüften, im Sommer nachts weit öffnen — das entlädt die Decke."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Venturi-Effekt — Luft in Bewegung", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "narrow"))
        self.play(FadeIn(caption), run_time=0.3)

        #region duct
        yc, x0, x1, xc, L, hw, ht = 0.0, -6.3, 1.5, -2.4, 1.6, 0.85, 0.425

        def half(x):
            d = abs(x - xc)
            return hw if d >= L else ht + (hw - ht) * (1 - np.cos(PI * d / L)) / 2

        xs = np.linspace(x0, x1, 120)
        top_wall = VMobject(color=P_WHITE, stroke_width=3.5)
        top_wall.set_points_smoothly([np.array([x, yc + half(x), 0.0]) for x in xs])
        bot_wall = VMobject(color=P_WHITE, stroke_width=3.5)
        bot_wall.set_points_smoothly([np.array([x, yc - half(x), 0.0]) for x in xs])
        fractions = (-0.72, -0.36, 0.0, 0.36, 0.72)
        lanes = []
        for f in fractions:
            lane = VMobject()
            lane.set_points_smoothly([np.array([x, yc + f * half(x), 0.0]) for x in xs])
            lanes.append(lane)

        def duct_speed(_i, u):
            return hw / half(x0 + u * (x1 - x0))

        v1 = Arrow(np.array([-5.6, yc, 0.0]), np.array([-4.9, yc, 0.0]), buff=0, color=P_YELLOW, stroke_width=5,
                   max_tip_length_to_length_ratio=0.3)
        v2 = Arrow(np.array([xc - 0.7, yc, 0.0]), np.array([xc + 0.7, yc, 0.0]), buff=0, color=P_YELLOW,
                   stroke_width=5, max_tip_length_to_length_ratio=0.18)
        v1_lbl = _m(r"v_{1}", np.array([-5.25, yc - hw - 0.32, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW)
        v2_lbl = _m(r"v_{2}", np.array([xc + 0.95, yc - half(xc + 0.95) - 0.32, 0.0]), size=LABEL_FONT_SIZE,
                    color=P_YELLOW)
        self.play(Create(top_wall), Create(bot_wall), run_time=1.0)
        _paced_flow(self, [(lanes, P_CYAN, duct_speed, None, False)], run_time=3.2, waves=7, cycles=1.6)
        hold_for(self, N, "narrow", used=BEAT_SUBTITLE_FADE + 0.3 + 4.2)

        caption = swap_caption(self, caption, subtitle_text(N, "continuity"))
        a1 = dim_arrow(np.array([x0 - 0.18, yc - hw, 0.0]), np.array([x0 - 0.18, yc + hw, 0.0]), color=P_CYAN)
        a2 = dim_arrow(np.array([xc - 0.95, yc - ht, 0.0]), np.array([xc - 0.95, yc + ht, 0.0]), color=P_CYAN)
        a1_lbl = _m(r"A_{1}", size=LABEL_FONT_SIZE, color=P_CYAN).next_to(a1, LEFT, buff=0.06)
        a2_lbl = _m(r"A_{2}", np.array([xc - 0.95, yc - half(xc - 0.95) - 0.32, 0.0]), size=LABEL_FONT_SIZE,
                    color=P_CYAN)
        row, box, items = _panel([
            ("a1", r"A_{1}", P_CYAN), (None, r"\cdot", P_WHITE), ("v1", r"v_{1}", P_YELLOW), (None, "=", P_WHITE),
            ("a2", r"A_{2}", P_CYAN), (None, r"\cdot", P_WHITE), ("v2", r"v_{2}", P_YELLOW), (None, r"\quad", P_WHITE),
            (None, r"A_{2} = \frac{A_{1}}{2} \Rightarrow v_{2} = 2\,v_{1}", P_WHITE),
        ])
        self.play(GrowFromCenter(a1), GrowFromCenter(a2), FadeIn(a1_lbl), FadeIn(a2_lbl), GrowArrow(v1), GrowArrow(v2),
                  FadeIn(v1_lbl), FadeIn(v2_lbl), run_time=1.0)
        self.play(FadeIn(row), Create(box), run_time=0.8)
        _paced_flow(self, [(lanes, P_CYAN, duct_speed, None, False)], run_time=2.4, waves=7, cycles=1.2)
        hold_for(self, N, "continuity")

        caption = swap_caption(self, caption, subtitle_text(N, "pressure"))
        tube_xs = (-5.0, xc)
        flow_on = ValueTracker(0.0)

        def column(i):
            def build():
                x = tube_xs[i]
                bottom = yc + half(x)
                level = 1.55 - (0.6 * flow_on.get_value() if i == 1 else 0.0)
                return Rectangle(width=0.16, height=level, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.75).move_to(
                    np.array([x, bottom + level / 2, 0.0]))
            return always_redraw(build)

        tubes = VGroup(*[
            RoundedRectangle(width=0.26, height=1.85, corner_radius=0.06, color=P_WHITE, stroke_width=2).move_to(
                np.array([x, yc + half(x) + 0.92, 0.0])) for x in tube_xs
        ])
        cols = [column(0), column(1)]
        p_lbls = VGroup(_m(r"p_{1}", tubes[0].get_right() + RIGHT * 0.15, size=LABEL_FONT_SIZE, color=P_BLUE,
                           edge="left"),
                        _m(r"p_{2}", tubes[1].get_right() + RIGHT * 0.15, size=LABEL_FONT_SIZE, color=P_BLUE,
                           edge="left"))
        b_panel = _panel([
            (None, "p", P_BLUE), (None, "+", P_WHITE), (None, r"\frac{1}{2}\,\rho\,v^{2}", P_YELLOW),
            (None, "=", P_WHITE), (None, r"\text{konstant}", P_WHITE),
        ])
        self.play(FadeOut(VGroup(a1, a2, a1_lbl, a2_lbl)), FadeIn(tubes), FadeIn(p_lbls), run_time=0.7)
        self.add(*cols)
        _paced_flow(self, [(lanes, P_CYAN, duct_speed, None, False)], run_time=2.6, waves=7, cycles=1.3,
                    extra=[flow_on.animate.set_value(1.0)])
        _swap_panel(self, (row, box), b_panel[:2])
        hold_for(self, N, "pressure")

        caption = swap_caption(self, caption, subtitle_text(N, "suction"))
        side = VGroup(Line(np.array([xc - 0.12, yc - ht, 0.0]), np.array([xc - 0.12, -1.2, 0.0]), color=P_WHITE,
                           stroke_width=3),
                      Line(np.array([xc + 0.12, yc - ht, 0.0]), np.array([xc + 0.12, -1.2, 0.0]), color=P_WHITE,
                           stroke_width=3))
        gap_cover = Line(np.array([xc - 0.1, yc - ht, 0.0]), np.array([xc + 0.1, yc - ht, 0.0]),
                         color=P_DEEP_DARK, stroke_width=6)
        side_path = [smooth_path([np.array([xc, -1.25, 0.0]), np.array([xc, yc - ht, 0.0]),
                                  np.array([xc + 0.6, yc - 0.15, 0.0]), np.array([x1, yc - 0.2, 0.0])])]
        self.play(Create(side), FadeIn(gap_cover), run_time=0.7)
        _paced_flow(self, [(lanes, P_CYAN, duct_speed, None, False), (side_path, P_GREEN, None, None, False)],
                    run_time=3.0, waves=6, cycles=1.5)
        hold_for(self, N, "suction")
        #endregion

        #region building
        caption = swap_caption(self, caption, subtitle_text(N, "roof"))
        self.play(FadeOut(VGroup(top_wall, bot_wall, v1, v2, v1_lbl, v2_lbl, tubes, p_lbls, side, gap_cover,
                                 b_panel[0], b_panel[1])), *[FadeOut(c) for c in cols], run_time=0.6)
        g_y, eave, peak, hx_l, hx_r = -1.3, 0.35, 1.25, -3.0, 1.0
        hxc = (hx_l + hx_r) / 2
        inlet = (g_y + 0.3, g_y + 0.8)
        out_lo = ValueTracker(-0.15)
        out_hi = 0.25
        wall_l = VGroup(Line(np.array([hx_l, g_y, 0.0]), np.array([hx_l, inlet[0], 0.0]), color=P_WHITE, stroke_width=3),
                        Line(np.array([hx_l, inlet[1], 0.0]), np.array([hx_l, eave, 0.0]), color=P_WHITE, stroke_width=3))
        wall_r_top = Line(np.array([hx_r, out_hi, 0.0]), np.array([hx_r, eave, 0.0]), color=P_WHITE, stroke_width=3)
        wall_r_low = always_redraw(lambda: Line(np.array([hx_r, g_y, 0.0]), np.array([hx_r, out_lo.get_value(), 0.0]),
                                                color=P_WHITE, stroke_width=3))
        roof = VMobject(color=P_WHITE, stroke_width=3)
        roof.set_points_as_corners([np.array([hx_l - 0.3, eave - 0.15, 0.0]), np.array([hxc, peak, 0.0]),
                                    np.array([hx_r + 0.3, eave - 0.15, 0.0])])
        ceiling = Rectangle(width=hx_r - hx_l, height=0.18, color=P_WHITE, stroke_width=2, fill_color=P_WHITE,
                            fill_opacity=0.15).move_to(np.array([hxc, eave - 0.09, 0.0]))
        ground = Line(np.array([-6.8, g_y, 0.0]), np.array([6.8, g_y, 0.0]), color=P_TEAL, stroke_width=3)
        far = (0.75, 1.3, 1.85)
        ridge = (1.55, 1.95, 2.3)
        sig = 1.7

        def stream_y(k, x):
            return far[k] + (ridge[k] - far[k]) * np.exp(-((x - hxc) / sig) ** 2)

        wind = []
        for k in range(3):
            lane = VMobject()
            lane.set_points_smoothly([np.array([x, stream_y(k, x), 0.0]) for x in np.linspace(-6.8, 6.8, 90)])
            wind.append(lane)

        def wind_speed(_i, u):
            x = -6.8 + 13.6 * u
            return 1.0 + 0.6 * np.exp(-((x - hxc) / sig) ** 2)

        plus = VGroup(*[_m("+", np.array([hx_l - 0.35, y, 0.0]), color=P_RED) for y in (-0.35, 0.1)])
        minus = VGroup(*[_m(r"−", p, color=P_BLUE) for p in (np.array([hxc + 0.95, peak + 0.05, 0.0]),
                                                              np.array([hx_r + 0.4, -0.9, 0.0]))])
        wind_lbl = Text("Wind", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([-6.2, 2.15, 0.0]))
        self.play(Create(ground), Create(wall_l), Create(wall_r_top), FadeIn(wall_r_low), Create(roof),
                  FadeIn(ceiling), FadeIn(wind_lbl), run_time=1.0)
        _paced_flow(self, [(wind, P_CYAN, wind_speed, None, False)], run_time=3.0, waves=8, cycles=1.4,
                    extra=[FadeIn(plus), FadeIn(minus)])
        hold_for(self, N, "roof")

        caption = swap_caption(self, caption, subtitle_text(N, "openings"))

        def cross_paths():
            lo = out_lo.get_value()
            return [smooth_path([np.array([-6.6, inlet[0] + 0.25 + dy, 0.0]),
                                 np.array([hx_l, (inlet[0] + inlet[1]) / 2 + dy, 0.0]),
                                 np.array([hxc, -0.6 + dy, 0.0]),
                                 np.array([hx_r, (lo + out_hi) / 2 + dy * 0.5, 0.0]),
                                 np.array([hx_r + 1.6, (lo + out_hi) / 2 + 0.3 + dy, 0.0]),
                                 np.array([4.6, 0.9 + dy, 0.0])])
                    for dy in (-0.12, 0.0, 0.12)]

        in_lbl = Text("Zuluft", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([hx_l - 0.95, inlet[1] + 0.3, 0.0]))
        out_lbl = Text("Abluft", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([hx_r + 0.8, out_hi + 0.35, 0.0]))
        self.play(FadeIn(in_lbl), FadeIn(out_lbl), run_time=0.5)
        _paced_flow(self, [(cross_paths(), P_CYAN, None, None, False), (wind, P_CYAN, wind_speed, None, False)],
                    run_time=3.0, waves=6, cycles=1.5)
        hold_for(self, N, "openings")

        caption = swap_caption(self, caption, subtitle_text(N, "size"))
        a_out = ValueTracker(0.5)

        def vdot():
            a_in = 0.5
            return 3944.0 / np.sqrt(1 / a_in ** 2 + 1 / a_out.get_value() ** 2)

        a_read = _readout(lambda: rf"A_{{\mathrm{{Zu}}}} = 0{{,}}5\,\mathrm{{m^{{2}}}},\;A_{{\mathrm{{Ab}}}} = "
                                  rf"{_de(a_out.get_value(), 1)}\,\mathrm{{m^{{2}}}}",
                          np.array([4.4, -0.55, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
        v_read = _readout(lambda: rf"\dot{{V}} \approx {_de(round(vdot(), -1))}\,\mathrm{{m^{{3}}/h}}",
                          np.array([4.4, -0.95, 0.0]), color=P_YELLOW, edge="center")
        self.add(a_read, v_read)
        _paced_flow(self, [(cross_paths(), P_CYAN, None, None, False)], run_time=1.6, waves=6, cycles=0.8)
        self.play(a_out.animate.set_value(1.0), out_lo.animate.set_value(-0.75), run_time=1.4)
        _paced_flow(self, [(cross_paths(), P_CYAN, None, None, False)], run_time=2.0, waves=8, cycles=1.3)
        hold_for(self, N, "size")
        #endregion

        #region loss
        caption = swap_caption(self, caption, subtitle_text(N, "loss"))
        self.play(FadeOut(VGroup(plus, minus, wind_lbl, in_lbl, out_lbl)), FadeOut(a_read), FadeOut(v_read),
                  out_lo.animate.set_value(0.05), run_time=0.7)
        row, box, items = _panel([
            (None, r"\dot{Q}_{\mathrm{V}}", P_RED), (None, "=", P_WHITE),
            (None, r"0{,}34\,\mathrm{Wh/(m^{3}\,K)}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("v", r"\dot{V}", P_CYAN), (None, r"\cdot", P_WHITE), ("dt", r"\Delta T", P_ORANGE),
        ])
        t_in = _m(r"20\,\mathrm{°C}", np.array([hxc, -1.05, 0.0]), color=P_RED)
        t_out = _m(r"0\,\mathrm{°C}", np.array([-5.4, -0.25, 0.0]), color=P_BLUE)
        self.play(FadeIn(row), Create(box), FadeIn(t_in), FadeIn(t_out), run_time=0.9)
        hold_for(self, N, "loss")

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        frac = ValueTracker(0.0)
        q_read = _readout(lambda: (rf"\dot{{V}} = 100\,\mathrm{{m^{{3}}/h}},\;\Delta T = 20\,\mathrm{{K}}\;\to\;"
                                   rf"\dot{{Q}}_{{\mathrm{{V}}}} = {_de(680 * frac.get_value())}\,\mathrm{{W}}"),
                          np.array([4.4, -0.3, 0.0]), size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        people = VGroup(*[_person(ORIGIN, scale=0.9) for _ in range(7)]).arrange(RIGHT, buff=0.18).move_to(
            np.array([4.4, -1.0, 0.0]))
        self.add(q_read)
        _paced_flow(self, [(cross_paths(), P_RED, None, None, False)], run_time=3.0, waves=3, cycles=1.0,
                    extra=[frac.animate.set_value(1.0), LaggedStart(*[FadeIn(p) for p in people], lag_ratio=0.3)])
        hold_for(self, N, "winter")

        caption = swap_caption(self, caption, subtitle_text(N, "night"))
        self.play(FadeOut(VGroup(people, t_in, t_out, row, box)), FadeOut(q_read), run_time=0.5)
        moon = _moon(np.array([5.6, 1.9, 0.0]))
        ceil_heat = ValueTracker(0.75)
        ceiling.add_updater(lambda m: m.set_fill(P_ORANGE, opacity=ceil_heat.get_value()))
        norm = Text("DIN 1946-6", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([4.6, -1.2, 0.0]))
        self.play(FadeIn(moon), out_lo.animate.set_value(-0.75), FadeIn(norm), run_time=0.8)
        night = [smooth_path([np.array([-6.6, inlet[0] + 0.25 + dy, 0.0]),
                              np.array([hx_l, (inlet[0] + inlet[1]) / 2 + dy, 0.0]),
                              np.array([hxc - 0.6, eave - 0.35 + dy * 0.5, 0.0]),
                              np.array([hx_r, -0.25 + dy, 0.0]), np.array([4.6, 0.6 + dy, 0.0])])
                 for dy in (-0.15, 0.0, 0.15)]
        _paced_flow(self, [(night, P_BLUE, None, None, False)], run_time=3.2, waves=7, cycles=1.6,
                    extra=[ceil_heat.animate.set_value(0.15)])
        ceiling.clear_updaters()
        hold_for(self, N, "night")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat9 – Force: what is inside a joule
class Beat9_Kraft(Scene):
    NARRATION = [
        ("question",
         "Finally: what is inside one joule? For that we need force, measured in newtons.",
         "Zum Schluss: Was steckt in einem Joule? Dafür brauchen wir die Kraft, gemessen in Newton."),
        ("apple",
         "The Earth pulls on a hundred-gram apple with about one newton: mass times gravitational acceleration.",
         "Die Erde zieht an einem 100-g-Apfel mit rund 1 N: Masse mal Erdbeschleunigung."),
        ("lift",
         "Lifting it one metre is work: force times distance — again an area, now in the force-distance diagram.",
         "Hebt man ihn 1 m hoch, ist das Arbeit: Kraft mal Weg — wieder eine Fläche, jetzt im Kraft-Weg-Diagramm."),
        ("joule",
         "One newton over one metre is one joule.",
         "1 N über 1 m ergibt 1 J."),
        ("heart",
         "The heart does about one joule with every beat — as much as moving one newton over one metre.",
         "Das Herz leistet pro Schlag rund 1 J — so viel, wie 1 N einen Meter weit zu bewegen."),
        ("watt",
         "About one beat per second: one joule per second — one watt.",
         "Etwa ein Schlag pro Sekunde: 1 J pro Sekunde — 1 W."),
        ("kilowatt",
         "A thousand hearts beating together: one kilowatt.",
         "Tausend Herzen zusammen: 1 kW."),
        ("kwh",
         "For one hour: one kilowatt-hour — 3.6 million joules. The kilowatt-hour from the start.",
         "Eine Stunde lang: 1 kWh — 3,6 Millionen Joule. Die Kilowattstunde vom Anfang."),
        ("next",
         "The next videos calculate with these: heating load in kilowatts, heating demand in kilowatt-hours per square metre and year.",
         "Damit rechnen die nächsten Videos: Heizlast in kW, Heizwärmebedarf in kWh pro m² und Jahr."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Kraft — was in einem Joule steckt", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "question"))
        self.play(FadeIn(caption), run_time=0.3)

        #region apple
        token = _m(r"1\,\mathrm{J}", np.array([0.0, 0.4, 0.0]), size=FORMULA_FONT_SIZE * 1.6, color=P_YELLOW)
        self.play(FadeIn(token, scale=0.6), run_time=0.8)
        hold_for(self, N, "question", used=BEAT_SUBTITLE_FADE + 0.3 + 0.8)

        caption = swap_caption(self, caption, subtitle_text(N, "apple"))
        ground_y = -0.75
        tree, apple, ground = _tree_and_apple(ground_y=ground_y)
        VGroup(tree, ground).shift(LEFT * 5.3)
        scale_body = RoundedRectangle(width=0.9, height=0.22, corner_radius=0.05, color=P_WHITE, stroke_width=2.2)
        scale_body.move_to(np.array([-4.0, ground_y + 0.11, 0.0]))
        apple.move_to(scale_body.get_top() + UP * (apple.height / 2))
        mass = ValueTracker(0.0)
        mass_read = _readout(lambda: rf"{_de(mass.get_value())}\,\mathrm{{g}}", scale_body.get_bottom() + DOWN * 0.3,
                             size=LABEL_FONT_SIZE, color=P_ORANGE, edge="center")
        force = ValueTracker(0.0)
        f_arrow = always_redraw(lambda: Arrow(
            apple.get_bottom() + RIGHT * 0.35, apple.get_bottom() + RIGHT * 0.35 + DOWN * max(0.01, 0.75 * force.get_value()),
            buff=0, color=P_RED, stroke_width=4, max_tip_length_to_length_ratio=0.25))
        f_read = _readout(lambda: rf"F_{{\mathrm{{G}}}} = {_de(force.get_value(), 2)}\,\mathrm{{N}}",
                          apple.get_right() + RIGHT * 0.55 + UP * 0.25, size=LABEL_FONT_SIZE, color=P_RED)
        row, box, items = _panel([
            (None, r"F_{\mathrm{G}}", P_RED), (None, "=", P_WHITE), (None, "m", P_ORANGE), (None, r"\cdot", P_WHITE),
            (None, "g", P_WHITE), (None, "=", P_WHITE), (None, r"0{,}1\,\mathrm{kg}", P_ORANGE),
            (None, r"\cdot", P_WHITE), (None, r"9{,}81\,\frac{\mathrm{m}}{\mathrm{s^{2}}}", P_WHITE),
            (None, r"\approx", P_WHITE), (None, r"1\,\mathrm{N}", P_RED),
        ])
        self.play(token.animate.scale(0.6).move_to(np.array([5.6, 1.9, 0.0])), FadeIn(tree), FadeIn(ground),
                  FadeIn(scale_body), FadeIn(apple), run_time=1.0)
        self.add(mass_read, f_arrow, f_read)
        self.play(mass.animate.set_value(100), force.animate.set_value(0.98), run_time=1.6)
        self.play(FadeIn(row), Create(box), run_time=0.8)
        hold_for(self, N, "apple")
        #endregion

        #region lift and joule
        caption = swap_caption(self, caption, subtitle_text(N, "lift"))
        FO = np.array([-1.6, ground_y, 0.0])
        FS, FF = 2.6, 1.8

        def fpt(s, f):
            return FO + RIGHT * s * FS + UP * f * FF

        fax = _axes(FO, 3.3, 2.5, r"s\;[\mathrm{m}]", r"F\;[\mathrm{N}]")
        f_line = Line(fpt(0, 1), fpt(1.2, 1), color=P_RED, stroke_width=2.6)
        s_tick = _m("1", fpt(1, 0) + DOWN * 0.3, size=LABEL_FONT_SIZE)
        f_tick = _m("1", fpt(0, 1) + LEFT * 0.2 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
        s = ValueTracker(0.0)
        lift = 1.3
        rest = apple.get_center().copy()
        work = always_redraw(lambda: Polygon(fpt(0, 0), fpt(max(0.01, s.get_value()), 0), fpt(max(0.01, s.get_value()), 1),
                                             fpt(0, 1), stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.35))
        w_read = _readout(lambda: rf"W = F \cdot s = {_de(s.get_value(), 2)}\,\mathrm{{J}}", fpt(0.6, 1.25),
                          size=LABEL_FONT_SIZE, color=P_YELLOW, edge="center")
        ruler = dim_arrow(rest + LEFT * 0.55, rest + LEFT * 0.55 + UP * lift, color=P_CYAN)
        ruler_lbl = _m(r"1\,\mathrm{m}", size=LABEL_FONT_SIZE, color=P_CYAN).next_to(ruler, LEFT, buff=0.06)
        self.play(FadeOut(mass_read), FadeIn(fax["group"]), Create(f_line), FadeIn(s_tick), FadeIn(f_tick),
                  GrowFromCenter(ruler), FadeIn(ruler_lbl), run_time=1.0)
        self.add(work, w_read)
        self.play(s.animate.set_value(1.0), apple.animate.shift(UP * lift), run_time=2.4)
        hold_for(self, N, "lift")

        caption = swap_caption(self, caption, subtitle_text(N, "joule"))
        j_panel = _panel([
            (None, "W", P_YELLOW), (None, "=", P_WHITE), (None, r"1\,\mathrm{N}", P_RED), (None, r"\cdot", P_WHITE),
            (None, r"1\,\mathrm{m}", P_CYAN), (None, "=", P_WHITE), ("j", r"1\,\mathrm{J}", P_YELLOW),
        ])
        _swap_panel(self, (row, box), j_panel[:2])
        self.play(Indicate(j_panel[2]["j"], color=P_YELLOW), Indicate(token, color=P_YELLOW), run_time=0.9)
        hold_for(self, N, "joule")
        #endregion

        #region heart and watt
        caption = swap_caption(self, caption, subtitle_text(N, "heart"))
        self.play(FadeOut(VGroup(fax["group"], f_line, s_tick, f_tick, tree, ground, scale_body, apple, ruler,
                                 ruler_lbl, j_panel[0], j_panel[1])),
                  *[FadeOut(m) for m in (work, w_read, f_arrow, f_read)], run_time=0.6)
        heart = _heart(np.array([-3.4, -0.35, 0.0]), size=1.5)
        vessel_x = -1.6
        vessel = VGroup(Line(np.array([vessel_x - 0.15, -0.6, 0.0]), np.array([vessel_x - 0.15, 1.55, 0.0]),
                             color=P_WHITE, stroke_width=2.5),
                        Line(np.array([vessel_x + 0.15, -0.6, 0.0]), np.array([vessel_x + 0.15, 1.55, 0.0]),
                             color=P_WHITE, stroke_width=2.5))
        link = ArcBetweenPoints(heart.get_top() + RIGHT * 0.25, np.array([vessel_x, -0.6, 0.0]), angle=-PI / 3,
                                color=P_WHITE, stroke_width=2.5)
        meter_1m = dim_arrow(np.array([vessel_x + 0.5, -0.45, 0.0]), np.array([vessel_x + 0.5, 0.85, 0.0]), color=P_CYAN)
        meter_lbl = _m(r"1\,\mathrm{m}", size=LABEL_FONT_SIZE, color=P_CYAN).next_to(meter_1m, RIGHT, buff=0.08)
        f_tag = _m(r"F \approx 1\,\mathrm{N}", np.array([vessel_x - 0.35, 1.0, 0.0]), size=LABEL_FONT_SIZE,
                   color=P_RED, edge="right")
        beats, secs = ValueTracker(0), ValueTracker(0)
        j_read = _readout(lambda: rf"W = {int(beats.get_value())}\,\mathrm{{J}}", np.array([1.6, 1.4, 0.0]),
                          color=P_YELLOW)
        t_read = _readout(lambda: rf"t = {int(secs.get_value())}\,\mathrm{{s}}", np.array([1.6, 0.8, 0.0]),
                          color=P_ORANGE)
        p_read = _readout(lambda: (rf"P = \frac{{{int(beats.get_value())}\,\mathrm{{J}}}}"
                                   rf"{{{max(1, int(secs.get_value()))}\,\mathrm{{s}}}} = "
                                   rf"{_de(beats.get_value() / max(1, secs.get_value()), 1)}\,\mathrm{{W}}"),
                          np.array([1.6, -0.15, 0.0]), color=P_CYAN)
        self.play(FadeIn(heart), Create(vessel), Create(link), GrowFromCenter(meter_1m), FadeIn(meter_lbl),
                  FadeIn(f_tag), run_time=1.0)
        self.add(j_read, t_read)

        def one_beat(k):
            slug = Rectangle(width=0.24, height=0.3, stroke_width=0, fill_color=P_RED, fill_opacity=0.85).move_to(
                np.array([vessel_x, -0.45, 0.0]))
            self.add(slug)
            beats.set_value(k)
            secs.set_value(k)
            self.play(heart.animate.scale(0.82), slug.animate.shift(UP * 1.3), run_time=0.35, rate_func=rush_from)
            self.play(heart.animate.scale(1 / 0.82), FadeOut(slug), run_time=0.65)

        for k in range(1, 4):
            one_beat(k)
        hold_for(self, N, "heart")

        caption = swap_caption(self, caption, subtitle_text(N, "watt"))
        self.add(p_read)
        for k in range(4, 7):
            one_beat(k)
        w_panel = _panel([
            (None, r"1\,\mathrm{W}", P_CYAN), (None, "=", P_WHITE), (None, r"\frac{1\,\mathrm{J}}{1\,\mathrm{s}}", P_YELLOW),
        ])
        self.play(FadeIn(w_panel[0]), Create(w_panel[1]), run_time=0.8)
        hold_for(self, N, "watt")
        #endregion

        #region kilowatt and kilowatt-hour
        caption = swap_caption(self, caption, subtitle_text(N, "kilowatt"))
        self.play(FadeOut(VGroup(vessel, link, meter_1m, meter_lbl, f_tag)),
                  *[FadeOut(m) for m in (j_read, t_read, p_read)], run_time=0.5)
        swarm = VGroup(*[Dot(radius=0.03, color=P_RED) for _ in range(1000)]).arrange_in_grid(rows=20, cols=50,
                                                                                            buff=0.075)
        swarm.move_to(np.array([-1.4, 0.35, 0.0]))
        kw_lbl = _m(r"1\,000 \times 1\,\mathrm{W} = 1\,\mathrm{kW}", np.array([4.4, 0.35, 0.0]), color=P_CYAN)
        self.play(ReplacementTransform(heart, swarm[0]), run_time=0.6)
        self.play(LaggedStart(*[TransformFromCopy(swarm[0], d) for d in swarm[1:]], lag_ratio=0.002), run_time=2.2)
        self.play(FadeIn(kw_lbl), run_time=0.6)
        hold_for(self, N, "kilowatt")

        caption = swap_caption(self, caption, subtitle_text(N, "kwh"))
        clock = _clock(np.array([4.4, 1.25, 0.0]), r=0.45)
        tank = _energy_tank(np.array([4.4, -0.75, 0.0]), height=1.3, width=0.7, level=0.0)
        kwh_panel = _panel([
            (None, r"1\,\mathrm{kW}", P_CYAN), (None, r"\cdot", P_WHITE), (None, r"1\,\mathrm{h}", P_ORANGE),
            (None, "=", P_WHITE), (None, r"1\,\mathrm{kWh}", P_YELLOW), (None, "=", P_WHITE),
            (None, r"3{,}6 \cdot 10^{6}\,\mathrm{J}", P_YELLOW),
        ])
        self.play(kw_lbl.animate.scale(0.8).move_to(np.array([4.4, 2.15, 0.0])), FadeOut(w_panel[0]),
                  FadeOut(w_panel[1]), run_time=0.6)
        self.play(FadeIn(clock["group"]), FadeIn(tank["group"]), FadeIn(tank["fill"]), run_time=0.5)
        self.play(Rotate(clock["hand"], angle=-TAU, about_point=clock["center"]), tank["level"].animate.set_value(1.0),
                  swarm.animate.set_color(P_YELLOW), run_time=2.6, rate_func=linear)
        self.play(FadeIn(kwh_panel[0]), Create(kwh_panel[1]), run_time=0.8)
        hold_for(self, N, "kwh")
        #endregion

        #region next
        caption = swap_caption(self, caption, subtitle_text(N, "next"))
        self.play(FadeOut(VGroup(swarm, kw_lbl, clock["group"], token, kwh_panel[0], kwh_panel[1])), run_time=0.6)
        house = _build_cross_section_house(center=np.array([-2.2, -0.4, 0.0]))
        load_tag = _m(r"\text{Heizlast}\;[\mathrm{kW}]", np.array([-5.5, 1.2, 0.0]), color=P_RED)
        demand_tag = _m(r"\text{Heizwärmebedarf}\;[\mathrm{kWh/(m^{2}\,a)}]", np.array([4.4, 0.6, 0.0]), color=P_CYAN)
        self.play(FadeIn(house["group"]), run_time=1.0)
        self.play(FadeIn(load_tag, shift=RIGHT * 0.1), FadeIn(demand_tag, shift=DOWN * 0.1), run_time=0.9)
        hold_for(self, N, "next")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Curriculum order
# 📚 The one beat order every compose, audio and build script imports.
BEATS = [
    Beat1_EnergieImAlltag,
    Beat2_Leistung,
    Beat3_Energieerhaltung,
    Beat4_Waermepumpe,
    Beat5_Strahlung,
    Beat6_ThermischeMasse,
    Beat7_SensibelLatent,
    Beat8_Venturi,
    Beat9_Kraft,
]
#endregion
