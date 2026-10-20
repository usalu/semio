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
    SAFE_TOP, NOTE_BAND_Y, radiation_ray,
    smooth_path, flow_guides, animate_flows, meter, set_meter, side_labels,
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
# 🔠 This video reads from a distance: labels and body text one step above the shared defaults.
LABEL_FONT_SIZE = 21
BODY_FONT_SIZE = 24


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


def _reveal(scene, *mobs, run_time: float = 0.35):
    """✨ Fade readouts and labels in — nothing pops onto the screen unannounced."""
    scene.play(*[FadeIn(m, shift=UP * 0.05) for m in mobs], run_time=run_time)
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

    ``streams`` holds ``(paths, colour, speed, colour_fn, closed)``, optionally followed by a per-stream particle
    count; ``speed``, ``colour_fn`` and ``closed`` may be ``None``/``False``.
    """
    us = np.linspace(0.0, 1.0, 241)
    dots, meta = VGroup(), []
    for paths, color, speed, color_fn, closed, *count in streams:
        n = count[0] if count else waves
        for i, path in enumerate(paths):
            g = np.array([max(1e-3, speed(i, u)) if speed else 1.0 for u in us])
            tau = np.concatenate([[0.0], np.cumsum(0.5 * (1.0 / g[1:] + 1.0 / g[:-1]) * np.diff(us))])
            tau /= tau[-1]
            for w in range(n):
                dot = Dot(radius=radius, color=color, stroke_width=0)
                dot.set_fill(color, opacity=0.0)
                dot.move_to(path.point_from_proportion(0.0))
                dots.add(dot)
                meta.append((path, tau, (w / n + 0.37 * i / n) % 1.0, color, color_fn, closed))

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


WAVES = {"short": (0.14, 0.03, 1.6), "long": (0.55, 0.085, 2.0)}


def _wave_points(corners, kind: str):
    """🌊 Points of a sinusoidal ray along a polyline — ``short`` is a dense ripple, ``long`` a stretched one, both tapering to the exact ends."""
    wavelength, amp, _ = WAVES[kind]
    corners = [np.array(c, dtype=float) for c in corners]
    segs = [(a, b, float(np.linalg.norm(b - a))) for a, b in zip(corners[:-1], corners[1:])]
    total = sum(length for _, _, length in segs)
    pts, done = [], 0.0
    for a, b, length in segs:
        u = (b - a) / length
        n = np.array([-u[1], u[0], 0.0])
        for t in np.linspace(0.0, length, max(4, int(length / wavelength * 12)), endpoint=False):
            s_ = done + t
            taper = min(1.0, s_ / wavelength, (total - s_) / wavelength)
            pts.append(a + u * t + n * amp * taper * np.sin(TAU * s_ / wavelength))
        done += length
    pts.append(corners[-1])
    return pts


def _wave(corners, kind: str, color=P_YELLOW, opacity: float = 0.85):
    """〰️ Wavy ray from ``corners`` — short-wave sunlight is compact, long-wave heat radiation is stretched."""
    mob = VMobject(color=color, stroke_width=WAVES[kind][2], stroke_opacity=opacity)
    mob.set_points_smoothly(_wave_points(corners, kind))
    return mob


def _mirror(direction, surface_angle: float):
    """🪞 Reflect a ray direction off a flat surface tilted by ``surface_angle`` radians."""
    u = np.array([np.cos(surface_angle), np.sin(surface_angle), 0.0])
    n = np.array([-u[1], u[0], 0.0])
    return direction - 2.0 * float(np.dot(direction, n)) * n


def _sun_rays(sun_c, glass_x: float, ys, land_y: float, *, gap: float = 0.5):
    """☀️ Parallel straight sun rays aimed through a glazing at ``glass_x`` — bright outside, dimmer past the glass.

    Every ray shares the direction from the sun to the middle hit, starts ``gap`` clear of the sun's centre and
    lands on the surface at ``land_y``.
    """
    sun_c = np.array(sun_c, dtype=float)
    hits = [np.array([glass_x, y, 0.0]) for y in ys]
    aim = np.mean(hits, axis=0) - sun_c
    aim /= np.linalg.norm(aim)
    starts = [h - aim * (float(np.dot(h - sun_c, aim)) - gap) for h in hits]
    lands = [h + aim * (land_y - h[1]) / aim[1] for h in hits]
    out = VGroup(*[radiation_ray(s, h) for s, h in zip(starts, hits)])
    inside = VGroup(*[radiation_ray(h, p, stroke_width=2.0).set_stroke(opacity=0.5) for h, p in zip(hits, lands)])
    return {"out": out, "in": inside, "group": VGroup(out, inside), "starts": starts, "hits": hits, "lands": lands}


def _shine(rays, *, lag: float = 0.2):
    """🔦 Draw each ray from the sun to the glass, then on through it."""
    return LaggedStart(*[Succession(Create(o, rate_func=linear), Create(i, rate_func=linear))
                         for o, i in zip(rays["out"], rays["in"])], lag_ratio=lag)


def _ripples(centers, *, r_max: float = 0.8, rings: int = 3, cycles: float = 2.0, color=P_RED, down: bool = False):
    """🌡️ Diffuse long-wave emission — half-rings spread from warm surface points in every direction and fade.

    ``down`` emits from a ceiling into the room below instead of from a floor into the room above.
    """
    centers = [np.array(c, dtype=float) for c in centers]
    start = PI if down else 0.0
    arcs = VGroup(*[Arc(radius=0.05, start_angle=start, angle=PI, arc_center=c, color=color, stroke_width=2)
                    for c in centers for _ in range(rings)])

    def update(group, alpha):
        ramp = min(1.0, alpha * 8, (1.0 - alpha) * 8)
        for i, arc in enumerate(group):
            u = (alpha * cycles + (i % rings) / rings) % 1.0
            arc.become(Arc(radius=0.05 + r_max * u, start_angle=start, angle=PI, arc_center=centers[i // rings],
                           color=color, stroke_width=2, stroke_opacity=0.65 * (1.0 - u) * ramp))

    return UpdateFromAlphaFunc(arcs, update, remover=True, rate_func=linear)


def _beam_pulses(scene, paths, color, *, repeats: int = 2, run_time: float = 2.4, width: float = 5.0, extra=None,
                 kind: str | None = None):
    """⚡ Bright pulses running along beam lines — the light itself travels, no extra glyphs."""
    lines = []
    for corners in paths:
        line = VMobject(color=color, stroke_width=width)
        if kind:
            line.set_points_smoothly(_wave_points(corners, kind))
        else:
            line.set_points_as_corners([np.array(c, dtype=float) for c in corners])
        lines.append(line)
    flashes = [ShowPassingFlash(line.copy(), time_width=0.35) for _ in range(repeats) for line in lines]
    scene.play(LaggedStart(*flashes, lag_ratio=0.6 / len(lines)), *(extra or []), run_time=run_time)


#endregion


#region Glyphs — sun shading
def _blind(x: float, y_top: float, y_bot: float, *, pitch: float = 0.3, length: float = 0.5,
           angle: float = 45 * DEGREES, color=P_TEAL):
    """🪟 Venetian blind in section — head rail, slats tilted outer edge low (like shingles) and the two ladder cords.

    ``ys`` are the slat centres, ``tilt`` the slat direction; the slats overlap in projection so a closed blind
    leaves no gap for the sun.
    """
    ys = np.arange(y_top - 0.06, y_bot - 1e-6, -pitch)
    tilt = np.array([np.cos(angle), np.sin(angle), 0.0])
    slats = VGroup(*[Line(np.array([x, y, 0.0]) - tilt * length / 2, np.array([x, y, 0.0]) + tilt * length / 2,
                          color=color, stroke_width=7) for y in ys])
    half = length * np.cos(angle) / 2
    rail = Rectangle(width=2 * half + 0.12, height=0.12, color=P_WHITE, stroke_width=1.6, fill_color=color,
                     fill_opacity=1.0).move_to(np.array([x, y_top + 0.06, 0.0]))
    cords = VGroup(*[Line(np.array([x + dx, y_top, 0.0]), np.array([x + dx, ys[-1] - length * np.sin(angle) / 2, 0.0]), color=P_WHITE,
                          stroke_width=1.2, stroke_opacity=0.6) for dx in (-half, half)])
    return {"ys": ys, "tilt": tilt, "length": length, "angle": angle, "x": x, "slats": slats, "rail": rail,
            "cords": cords, "group": VGroup(rail, cords, slats)}


def _blind_hit(blind, start, direction):
    """🎯 First point where a straight ray meets a slat of ``blind`` — rays end on the slat faces, not on a flat line."""
    start, direction = np.array(start, dtype=float), np.array(direction, dtype=float)
    d, half = blind["tilt"], blind["length"] / 2
    best = None
    for y in blind["ys"]:
        rhs = np.array([blind["x"], y, 0.0]) - start
        det = -direction[0] * d[1] + d[0] * direction[1]
        if abs(det) < 1e-9:
            continue
        t = (-rhs[0] * d[1] + d[0] * rhs[1]) / det
        u = (direction[0] * rhs[1] - direction[1] * rhs[0]) / det
        if t > 0 and abs(u) <= half and (best is None or t < best):
            best = t
    if best is None:
        best = (blind["x"] - start[0]) / direction[0]
    return start + direction * best
#endregion


#region Glyphs — buildings
def _window(x: float, sill: float, lintel: float, *, depth: float = 0.42, color=P_CYAN):
    """🪟 Window set into a line wall — reveal ticks at sill and lintel, a glazed sash seen edge-on in the wall line.

    The sash is built face-on and turned 90° about its vertical hinge, so ``_open_window`` swings it into the
    room in true projection instead of sliding a rectangle around.
    """
    reveal = VGroup(*[Line(np.array([x - 0.1, y, 0.0]), np.array([x + 0.1, y, 0.0]), color=P_WHITE, stroke_width=1.6)
                      for y in (sill, lintel)])
    mid = (sill + lintel) / 2
    frame = Rectangle(width=depth, height=lintel - sill - 0.04, color=color, stroke_width=1.6)
    frame.set_fill(color, opacity=0.14)
    bars = VGroup(Line(frame.get_left(), frame.get_right(), color=color, stroke_width=1.1),
                  Line(frame.get_top(), frame.get_bottom(), color=color, stroke_width=1.1))
    sash = VGroup(frame, bars).move_to(np.array([x + depth / 2, mid, 0.0]))
    hinge = np.array([x, mid, 0.0])
    sash.rotate(PI / 2, axis=UP, about_point=hinge)
    return {"x": x, "sill": sill, "lintel": lintel, "center": hinge, "hinge": hinge, "sash": sash,
            "reveal": reveal, "group": VGroup(reveal, sash)}


def _open_window(window, *, run_time: float = 0.9):
    """🚪 Turn the sash open into the room about its hinge."""
    return Rotate(window["sash"], angle=-PI / 2, axis=UP, about_point=window["hinge"], run_time=run_time)


def _build_cross_section_house(center=ORIGIN):
    """🏠 Two-storey section — wall thickness, eaves, and glazed openings in the left wall."""
    w_width, w_height, t, sw = 3.6, 2.4, 0.08, 1.7
    bottom_left = center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
    bottom_right = center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
    top_left = center + LEFT * (w_width / 2) + UP * (w_height / 2)
    top_right = center + RIGHT * (w_width / 2) + UP * (w_height / 2)
    roof_peak = center + UP * (w_height / 2 + 1.1)
    floor_line = Line(bottom_left + LEFT * 0.55, bottom_right + RIGHT * 0.55, color=P_TEAL, stroke_width=2.2)
    hatches = VGroup(*[
        Line(p, p + DOWN * 0.1 + LEFT * 0.07, color=P_TEAL, stroke_width=1.1)
        for p in [floor_line.point_from_proportion(u) for u in np.linspace(0.08, 0.92, 9)]
    ])
    level_1 = Line(bottom_left + UP * (w_height / 2), bottom_right + UP * (w_height / 2), color=P_WHITE, stroke_width=1.5)
    w_h = 0.5
    cuts = (
        (0.0, w_height / 4 - w_h / 2),
        (w_height / 4 + w_h / 2, 3 * w_height / 4 - w_h / 2),
        (3 * w_height / 4 + w_h / 2, w_height),
    )
    wall_left_1 = Line(bottom_left, bottom_left + UP * cuts[0][1], color=P_WHITE, stroke_width=sw)
    wall_left_2 = Line(bottom_left + UP * cuts[1][0], bottom_left + UP * cuts[1][1], color=P_WHITE, stroke_width=sw)
    wall_left_3 = Line(bottom_left + UP * cuts[2][0], top_left, color=P_WHITE, stroke_width=sw)
    wall_right = Line(bottom_right, top_right, color=P_WHITE, stroke_width=sw)
    inner = VGroup(*[
        Line(bottom_left + RIGHT * t + UP * a, bottom_left + RIGHT * t + UP * b, color=P_WHITE, stroke_width=1.3)
        for a, b in cuts
    ], Line(bottom_right + LEFT * t, top_right + LEFT * t, color=P_WHITE, stroke_width=1.3))
    eave_l, eave_r = top_left + LEFT * 0.26, top_right + RIGHT * 0.26
    roof = VGroup(
        VMobject(color=P_WHITE, stroke_width=sw).set_points_as_corners([eave_l, roof_peak, eave_r]),
        VMobject(color=P_WHITE, stroke_width=1.3).set_points_as_corners([
            top_left + RIGHT * t + DOWN * 0.05, roof_peak + DOWN * 0.12, top_right + LEFT * t + DOWN * 0.05,
        ]),
    )
    walls = VGroup(wall_left_1, wall_left_2, wall_left_3, wall_right, level_1, inner)
    windows = [_window(bottom_left[0], bottom_left[1] + f * w_height - w_h / 2, bottom_left[1] + f * w_height + w_h / 2)
               for f in (0.25, 0.75)]
    window_group = VGroup(*[w["group"] for w in windows])
    return {
        "center": center, "floor": floor_line, "walls": walls, "roof": roof, "windows": windows,
        "window_group": window_group, "level_1": level_1, "wall_right": wall_right,
        "group": VGroup(floor_line, hatches, walls, roof, window_group),
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
    glass = Rectangle(width=0.1, height=hi - lo, color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN,
                      fill_opacity=0.15).move_to(np.array([gx, (lo + hi) / 2, 0.0]))
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
    """♨️ Panel radiator in elevation — casing, fins, feet."""
    sw = 1.5
    body = Rectangle(width=0.92, height=0.5, color=color, stroke_width=sw)
    grille = Line(body.get_corner(UL) + DOWN * 0.08 + RIGHT * 0.06, body.get_corner(UR) + DOWN * 0.08 + LEFT * 0.06,
                  color=color, stroke_width=1.1)
    fins = VGroup(*[
        Line(body.get_top() + DOWN * 0.14, body.get_bottom() + UP * 0.08, color=color, stroke_width=1.1).shift(RIGHT * dx)
        for dx in np.linspace(-0.34, 0.34, 7)
    ])
    feet = VGroup(*[
        Line(body.get_bottom() + RIGHT * dx, body.get_bottom() + RIGHT * dx + DOWN * 0.07, color=color, stroke_width=sw)
        for dx in (-0.32, 0.32)
    ])
    return VGroup(body, grille, fins, feet).move_to(center)




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
    """🧍 Standing figure in elevation — head, torso and limbs, no face."""
    sw = 1.55
    head = Circle(radius=0.05, color=color, stroke_width=sw).shift(UP * 0.29)
    neck = Line(head.get_bottom(), head.get_bottom() + DOWN * 0.025, color=color, stroke_width=sw)
    sy, hy = head.get_bottom()[1] - 0.04, head.get_bottom()[1] - 0.2
    shoulder = Line(LEFT * 0.1 + UP * sy, RIGHT * 0.1 + UP * sy, color=color, stroke_width=sw)
    torso = VGroup(
        Line(LEFT * 0.1 + UP * sy, LEFT * 0.065 + UP * hy, color=color, stroke_width=sw),
        Line(RIGHT * 0.1 + UP * sy, RIGHT * 0.065 + UP * hy, color=color, stroke_width=sw),
    )
    arms = VGroup(
        Line(LEFT * 0.1 + UP * sy, LEFT * 0.145 + UP * (hy + 0.02), color=color, stroke_width=sw),
        Line(RIGHT * 0.1 + UP * sy, RIGHT * 0.145 + UP * (hy + 0.02), color=color, stroke_width=sw),
    )
    foot = hy - 0.3
    legs = VGroup(
        Line(LEFT * 0.035 + UP * hy, LEFT * 0.05 + UP * foot, color=color, stroke_width=sw),
        Line(RIGHT * 0.035 + UP * hy, RIGHT * 0.05 + UP * foot, color=color, stroke_width=sw),
    )
    feet = VGroup(
        Line(LEFT * 0.05 + UP * foot, LEFT * 0.015 + UP * foot, color=color, stroke_width=sw),
        Line(RIGHT * 0.05 + UP * foot, RIGHT * 0.09 + UP * foot, color=color, stroke_width=sw),
    )
    fig = VGroup(head, neck, shoulder, torso, arms, legs, feet)
    fig.scale(0.66 / fig.height)
    return fig.scale(scale).move_to(pos)


def _cloud(content, *, pad=(0.42, 0.36), bump: float = 0.5, color=P_WHITE):
    """☁️ Puffy speech cloud around ``content`` — one closed outline of outward arcs laid on an ellipse."""
    a, b = content.width / 2 + pad[0], content.height / 2 + pad[1]
    c = content.get_center()
    n = max(8, int(round(np.pi * (3 * (a + b) - np.sqrt((3 * a + b) * (a + 3 * b))) / bump)))
    pts = [c + np.array([a * np.cos(t), b * np.sin(t), 0.0]) for t in np.linspace(0, TAU, n, endpoint=False) + PI / n]
    outline = VMobject(color=color, stroke_width=2.2)
    for p, q in zip(pts, pts[1:] + pts[:1]):
        outline.append_points(ArcBetweenPoints(p, q, angle=0.8 * PI).points)
    return outline.set_fill(P_DEEP_DARK, opacity=0.9)


def _cloud_trail(cloud, target, *, n: int = 3, color=P_WHITE):
    """💭 Shrinking puffs that lead from the cloud's rim down to a speaker."""
    start = cloud.get_bottom() + LEFT * cloud.width * 0.22 + UP * 0.04
    end = np.array(target, dtype=float)
    return VGroup(*[
        Circle(radius=0.15 - 0.04 * k, color=color, stroke_width=2.0).set_fill(P_DEEP_DARK, opacity=0.9)
        .move_to(interpolate(start, end, (k + 1) / (n + 1)))
        for k in range(n)
    ])


def _flame(base, *, h: float, w: float, sway: float = 0.0, color=P_ORANGE, core=P_YELLOW):
    """🔥 Teardrop flame standing on ``base`` — an outer tongue around a hot inner core; ``sway`` bends the tip."""
    base = np.array(base, dtype=float)

    def tongue(hh, ww, fill, opacity):
        pts = [base + np.array([ww / 2 * np.sin(a) * np.sin(a / 2) + sway * (1 + np.cos(a)) / 2,
                                hh / 2 * (1 + np.cos(a)), 0.0]) for a in np.linspace(0, TAU, 36, endpoint=False)]
        return VMobject(stroke_width=0).set_points_smoothly(pts + pts[:1]).set_fill(fill, opacity=opacity)

    return VGroup(tongue(h, w, color, 0.85), tongue(h * 0.55, w * 0.5, core, 0.95))


def _hob(x: float, base_y: float, *, width: float = 1.4, flame_h: float = 0.36, n: int = 4, seed: int = 4):
    """🍳 Gas hob — burner head, pan supports and flames that flicker on their own clock, even during waits."""
    top = base_y + 0.12
    head = RoundedRectangle(width=width, height=0.12, corner_radius=0.05, color=P_WHITE, stroke_width=2,
                            fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to(np.array([x, base_y + 0.06, 0.0]))
    supports = VGroup(*[Line(np.array([x + s * width / 2, top, 0.0]), np.array([x + s * width / 2, top + flame_h + 0.06, 0.0]),
                             color=P_WHITE, stroke_width=2.5, stroke_opacity=0.7) for s in (-1, 1)])
    xs = np.linspace(x - width * 0.32, x + width * 0.32, n)
    phases = np.random.default_rng(seed).uniform(0, TAU, (n, 2))
    clock = {"t": 0.0}

    def draw():
        t = clock["t"]
        return VGroup(*[_flame(np.array([fx, top, 0.0]), w=0.3, sway=0.035 * np.sin(6.0 * t + p[1]),
                               h=flame_h * (1 + 0.14 * np.sin(9.0 * t + p[0]) + 0.07 * np.sin(17.0 * t + p[1])))
                        for fx, p in zip(xs, phases)])

    flames = draw()

    def flicker(m, dt):
        clock["t"] += dt
        m.become(draw())

    flames.add_updater(flicker)
    return {"head": head, "supports": supports, "flames": flames, "group": VGroup(head, supports, flames),
            "pan_y": top + flame_h + 0.06}


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
    """🧹 Canister vacuum in side elevation — body, hose, wand, floor nozzle."""
    sw = 1.5
    body = RoundedRectangle(width=0.74, height=0.32, corner_radius=0.06, color=color, stroke_width=sw)
    lid = Line(body.get_left() + RIGHT * 0.16 + UP * 0.1, body.get_left() + RIGHT * 0.16 + DOWN * 0.1,
               color=color, stroke_width=1.1)
    wheels = VGroup(*[
        VGroup(Circle(radius=0.045, color=color, stroke_width=1.3), Dot(radius=0.012, color=color)).move_to(
            body.get_bottom() + RIGHT * dx + DOWN * 0.01)
        for dx in (-0.2, 0.18)
    ])
    hose = ArcBetweenPoints(body.get_right() + UP * 0.04, body.get_right() + RIGHT * 0.62 + UP * 0.42,
                            angle=-PI / 2.6, color=color, stroke_width=sw)
    cuff = Circle(radius=0.035, color=color, stroke_width=1.2).move_to(hose.get_end())
    tube = Line(hose.get_end(), hose.get_end() + RIGHT * 0.28 + DOWN * 0.62, color=color, stroke_width=sw)
    foot = tube.get_end()
    nozzle = VMobject(color=color, stroke_width=sw).set_points_as_corners([
        foot + LEFT * 0.16 + UP * 0.06, foot + RIGHT * 0.2 + UP * 0.06, foot + RIGHT * 0.2, foot + LEFT * 0.16, foot + LEFT * 0.16 + UP * 0.06,
    ])
    return VGroup(body, lid, wheels, hose, cuff, tube, nozzle)


def _shower_head(anchor, color=P_WHITE):
    """🚿 Wall arm and rose — drops leave the underside of the rose."""
    a = np.array(anchor, dtype=float)
    sw = 1.5
    plate = Line(a + LEFT * 0.72 + UP * 0.1, a + LEFT * 0.72 + DOWN * 0.1, color=color, stroke_width=sw)
    arm = Line(a + LEFT * 0.72, a + DOWN * 0.02, color=color, stroke_width=sw)
    stem = Line(a + DOWN * 0.02, a + DOWN * 0.1, color=color, stroke_width=sw)
    rose = Circle(radius=0.14, color=color, stroke_width=sw).move_to(a + DOWN * 0.22)
    nozzles = VGroup(*[
        Dot(rose.get_center() + 0.07 * np.array([np.cos(t), np.sin(t), 0.0]), radius=0.012, color=color)
        for t in np.linspace(0, TAU, 8, endpoint=False)
    ])
    return VGroup(plate, arm, stem, rose, nozzles)


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




def _lamp(anchor, drop: float = 0.55):
    """💡 Pendant lamp — cable, conical shade, bulb."""
    a = np.array(anchor, dtype=float)
    cable = Line(a, a + DOWN * drop, color=P_WHITE, stroke_width=1.4)
    top = a + DOWN * drop
    shade = VGroup(
        Line(top + LEFT * 0.05, top + DOWN * 0.2 + LEFT * 0.22, color=P_WHITE, stroke_width=1.5),
        Line(top + RIGHT * 0.05, top + DOWN * 0.2 + RIGHT * 0.22, color=P_WHITE, stroke_width=1.5),
        Line(top + DOWN * 0.2 + LEFT * 0.22, top + DOWN * 0.2 + RIGHT * 0.22, color=P_WHITE, stroke_width=1.5),
    )
    bulb = Circle(radius=0.09, color=P_YELLOW, stroke_width=1.5).move_to(a + DOWN * (drop + 0.32))
    filament = Line(bulb.get_center() + UP * 0.03 + LEFT * 0.025, bulb.get_center() + DOWN * 0.03 + RIGHT * 0.025,
                    color=P_YELLOW, stroke_width=1.1)
    return {"cable": cable, "bulb": bulb, "group": VGroup(cable, shade, bulb, filament)}




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
         "Mass times specific heat times temperature rise gives about 3,520 kilojoules.",
         "Masse mal spezifische Wärme mal Temperaturanstieg: rund 3 520 kJ."),
        ("convert1",
         "To convert: one kilowatt is one kilojoule per second, so one kilowatt-hour is 3,600 seconds of that — 3,600 kilojoules.",
         "Umrechnung: 1 kW sind 1 kJ pro Sekunde, also sind 1 kWh genau 3 600 s davon — 3 600 kJ."),
        ("convert2",
         "3,520 kilojoules divided by 3,600 kilojoules per kilowatt-hour: about one kilowatt-hour.",
         "3 520 kJ geteilt durch 3 600 kJ pro kWh: rund 1 kWh."),
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
        ("envelope",
         "Through walls and roof heat also flows outward — transmission.",
         "Durch Wände und Dach fließt Wärme außerdem nach außen — die Transmission."),
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
        grid_tag = Text("Stromnetz", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(wire_in.get_start(), RIGHT,
                                                                                     buff=0.12)
        self.play(FadeIn(hg), FadeIn(meter_box), FadeIn(meter_tag), Create(wire_in), Create(wire_out), FadeIn(grid_tag),
                  run_time=1.0)
        _reveal(self, meter_read)
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
        cloud = _cloud(said)
        trail = _cloud_trail(cloud, person.get_top() + RIGHT * 0.12 + UP * 0.08)
        self.play(FadeIn(person, shift=UP * 0.15), FadeOut(grid_tag), run_time=0.5)
        self.play(LaggedStart(*[GrowFromCenter(p) for p in trail[::-1]], lag_ratio=0.45), run_time=0.6)
        self.play(GrowFromPoint(cloud, trail[0].get_center(), rate_func=rate_functions.ease_out_back), run_time=0.6)
        self.play(LaggedStart(*[FadeIn(said_items[k], shift=UP * 0.12) for k in ("n", "kw", "v")], lag_ratio=0.35),
                  run_time=0.8)
        self.play(cloud.animate(rate_func=there_and_back).scale(1.04), run_time=0.6)
        hold_for(self, N, "slip")

        caption = swap_caption(self, caption, subtitle_text(N, "hour"))
        clock = _clock(np.array([5.7, 0.95, 0.0]))
        clock_lbl = Text("1 Stunde", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(clock["face"], DOWN, buff=0.14)
        h_glyph = _m(r"\mathrm{h}", clock["face"].get_top() + UP * 0.3, color=P_ORANGE)
        self.play(FadeIn(clock["group"]), FadeIn(clock_lbl), run_time=0.6)
        self.play(Rotate(clock["hand"], angle=-TAU, about_point=clock["center"]), FadeIn(h_glyph, scale=0.5),
                  run_time=1.2)
        kw = said_items["kw"]
        h_target = h_glyph.copy()
        _place(h_target, np.array([kw.get_right()[0] + 0.07, kw.base.get_center()[1], 0.0]), "left")
        shift = h_target.width + 0.09
        wide = _cloud(Rectangle(width=said.width + shift, height=said.height).move_to(said).shift(RIGHT * shift / 2))
        self.play(said_items["v"].animate.shift(RIGHT * shift), kw.animate.set_color(P_CYAN), Transform(cloud, wide),
                  run_time=0.7)
        self.play(h_glyph.animate.move_to(np.array([h_target.get_center()[0],
                                                    cloud.get_top()[1] + h_glyph.height / 2 + 0.15, 0.0])),
                  run_time=0.7)
        self.play(h_glyph.animate(rate_func=rate_functions.ease_in_quad).move_to(h_target.get_center()), run_time=0.4)
        self.play(cloud.animate(rate_func=there_and_back).scale(1.05), Flash(h_glyph, color=P_ORANGE, line_length=0.16,
                                                                             flash_radius=0.28), run_time=0.6)
        bubble = VGroup(cloud, trail)
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
        clock2 = _clock(np.array([5.5, 1.35, 0.0]))
        minutes = ValueTracker(0.0)
        min_read = _readout(lambda: rf"t = {_de(minutes.get_value())}\,\mathrm{{min}}",
                            np.array([5.5, 0.6, 0.0]), color=P_CYAN, edge="center")
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
        _reveal(self, tank_read, min_read)
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
        _reveal(self, litre_read, temp_read)
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
            ("dt", r"28\,\mathrm{K}", P_RED), (None, "=", P_WHITE), ("res", r"3\,520\,\mathrm{kJ}", P_ORANGE),
        ])
        self.play(FadeOut(row), FadeOut(box), FadeOut(floor), run_time=0.4)
        h_row, h_box, h_items = heat_panel
        _fill_panel(self, heat_panel, {"m": litre_read, "dt": temp_read})
        self.play(Indicate(h_items["res"], color=P_ORANGE), run_time=0.9)
        hold_for(self, N, "heat")
        #endregion

        #region convert
        caption = swap_caption(self, caption, subtitle_text(N, "convert1"))
        conv_panel = _panel([
            (None, r"1\,\mathrm{kWh}", P_YELLOW), (None, "=", P_WHITE), ("kw", r"1\,\mathrm{kW}", P_CYAN),
            (None, r"\cdot", P_WHITE), ("s", r"3\,600\,\mathrm{s}", P_ORANGE), (None, "=", P_WHITE),
            ("kj", r"1\,\frac{\mathrm{kJ}}{\mathrm{s}}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("s2", r"3\,600\,\mathrm{s}", P_ORANGE), (None, "=", P_WHITE), ("res", r"3\,600\,\mathrm{kJ}", P_ORANGE),
        ])
        conv_row, conv_box, conv_items = conv_panel
        _swap_panel(self, (h_row, h_box), (conv_row, conv_box))
        self.play(Indicate(conv_items["kw"], color=P_CYAN), Indicate(conv_items["kj"], color=P_CYAN), run_time=0.8)
        self.play(Indicate(conv_items["s"], color=P_ORANGE), Indicate(conv_items["s2"], color=P_ORANGE),
                  Indicate(conv_items["res"], color=P_ORANGE), run_time=0.9)
        hold_for(self, N, "convert1")

        caption = swap_caption(self, caption, subtitle_text(N, "convert2"))
        fin_panel = _panel([
            (None, "Q", P_WHITE), (None, "=", P_WHITE),
            ("a", r"\frac{3\,520\,\mathrm{kJ}}{3\,600\,\mathrm{kJ}}", P_ORANGE), (None, r"\mathrm{kWh}", P_YELLOW),
            (None, r"\approx", P_WHITE), ("res", r"0{,}98\,\mathrm{kWh}", P_YELLOW), (None, r"\approx", P_WHITE),
            ("one", r"1\,\mathrm{kWh}", P_YELLOW),
        ])
        fin_row, fin_box, fin_items = fin_panel
        _swap_panel(self, (conv_row, conv_box), (fin_row, fin_box))
        self.play(Indicate(fin_items["a"], color=P_ORANGE), run_time=0.8)
        self.play(Indicate(fin_items["one"], color=P_YELLOW), Indicate(tank_lbl, color=P_YELLOW), run_time=0.9)
        hold_for(self, N, "convert2")
        #endregion

        #region ladder
        caption = swap_caption(self, caption, subtitle_text(N, "ladder"))
        ladder = _log_ladder(-1.5, 14.6, x=-1.4, y_lo=-1.6, y_hi=2.25, unit="kWh")
        entries = [
            (0.13, r"\text{Teelicht} \approx 0{,}1\,\mathrm{kWh}", P_ORANGE),
            (1.0, r"\text{Staubsauger, 1 Stunde} \approx 1\,\mathrm{kWh}", P_YELLOW),
            (3000, r"\text{Haushaltsstrom pro Jahr} \approx 3\,000\,\mathrm{kWh}", P_CYAN),
            (15000, r"\text{Heizwärme Einfamilienhaus pro Jahr} \approx 15\,000\,\mathrm{kWh}", P_RED),
            (2.4e12, r"\text{Deutschland pro Jahr, Endenergie} \approx 2{,}4 \cdot 10^{12}\,\mathrm{kWh}", P_GREEN),
            (1.7e14, r"\text{Welt pro Jahr, Primärenergie} \approx 1{,}7 \cdot 10^{14}\,\mathrm{kWh}", P_TEAL),
        ]
        dots, labels, leaders = _ladder_marks(ladder, entries, x_text=-0.75)
        thermo["level"].clear_updaters()
        self.play(
            FadeOut(fin_row), FadeOut(fin_box), FadeOut(head), FadeOut(thermo["group"]), FadeOut(thermo["column"]),
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
        span = dim_arrow(np.array([-4.0, ladder["y_of"](0.13), 0.0]), np.array([-4.0, ladder["y_of"](1.7e14), 0.0]),
                         color=P_YELLOW)
        span_lbl = _m(r"\times\,10^{15}", color=P_YELLOW).next_to(span, LEFT, buff=0.14)
        self.play(GrowFromCenter(span), FadeIn(span_lbl), run_time=0.9)
        hold_for(self, N, "world")
        #endregion

        #region design
        caption = swap_caption(self, caption, subtitle_text(N, "design"))
        self.play(FadeOut(VGroup(ladder["group"], *dots, *labels, *leaders, span, span_lbl)), run_time=0.6)
        house2 = _build_cross_section_house(center=np.array([-0.3, -0.45, 0.0]))
        win_lo, win_hi = house2["windows"]
        sun_pos = np.array([-5.6, 1.85, 0.0])
        sun = _build_sun(sun_pos).scale(0.4)
        slab = Rectangle(width=house2["level_1"].width, height=0.18, color=P_ORANGE, stroke_width=2,
                         fill_color=P_ORANGE, fill_opacity=0.5).move_to(house2["level_1"].get_center())
        rays = _sun_rays(sun_pos, win_hi["x"], [win_hi["center"][1] + dy for dy in (0.15, 0.0, -0.15)],
                         slab.get_top()[1], gap=0.55)
        hits, lands = rays["hits"], rays["lands"]
        sun_patch = Line(lands[-1], lands[0], color=P_YELLOW, stroke_width=7, stroke_opacity=0.75)
        beams = [VMobject(color=P_YELLOW, stroke_width=5).set_points_as_corners([s, h, p])
                 for s, h, p in zip(rays["starts"], hits, lands)]

        x_w, sill, lintel = win_lo["x"], win_lo["sill"], win_lo["lintel"]
        x_r = house2["wall_right"].get_center()[0]
        floor_y, ceil_y = house2["walls"][0].get_bottom()[1], slab.get_bottom()[1]
        intake = [smooth_path([np.array(p) + UP * dy for p in (
            [x_w - 1.5, sill - 0.08, 0.0], [x_w - 0.5, sill + 0.08, 0.0], [x_w + 0.35, sill + 0.02, 0.0],
            [x_w + 1.1, floor_y + 0.2, 0.0], [x_r - 0.9, floor_y + 0.18, 0.0], [x_r - 0.3, floor_y + 0.6, 0.0],
        )]) for dy in (0.0, 0.11)]
        exhaust = [smooth_path([np.array(p) + DOWN * dy for p in (
            [x_r - 0.3, floor_y + 0.75, 0.0], [x_r - 0.55, ceil_y - 0.18, 0.0], [x_w + 1.4, ceil_y - 0.15, 0.0],
            [x_w + 0.4, lintel - 0.06, 0.0], [x_w - 0.4, lintel - 0.06, 0.0], [x_w - 1.0, lintel + 0.28, 0.0],
        )]) for dy in (0.0, 0.11)]
        guides = VGroup(flow_guides(intake[:1], P_CYAN, opacity=0.22), flow_guides(exhaust[:1], P_ORANGE, opacity=0.22))
        air = [(intake, P_CYAN, P_ORANGE), (exhaust, P_ORANGE)]

        left_tags = VGroup(
            Text("Glas — Strahlung", font_size=LABEL_FONT_SIZE, color=P_YELLOW),
            Text("Öffnungen — Luftströmung", font_size=LABEL_FONT_SIZE, color=P_CYAN),
        )
        left_tags[0].move_to(np.array([x_w - 0.35 - left_tags[0].width / 2, win_hi["center"][1] - 0.3, 0.0]))
        left_tags[1].move_to(np.array([x_w - 0.55 - left_tags[1].width / 2, floor_y - 0.38, 0.0]))
        left_leaders = VGroup(
            Line(left_tags[0].get_right() + RIGHT * 0.1, hits[-1] + LEFT * 0.08, color=P_YELLOW, stroke_width=1.4,
                 stroke_opacity=0.55),
            Line(left_tags[1].get_right() + RIGHT * 0.1, np.array([x_w - 0.08, sill - 0.04, 0.0]), color=P_CYAN,
                 stroke_width=1.4, stroke_opacity=0.55),
        )
        right_tags, right_leaders = side_labels([
            (slab.get_right(), "Decke — Speichermasse", P_ORANGE),
        ], x=x_r + 0.65, align="left")

        hc = house2["center"]
        top_l, top_r = hc + LEFT * 1.8 + UP * 1.2, hc + RIGHT * 1.8 + UP * 1.2
        peak = hc + UP * 2.3
        slopes = [(top_l + LEFT * 0.26, peak), (peak, top_r + RIGHT * 0.26)]

        def outward(a, b):
            u = (b - a) / np.linalg.norm(b - a)
            n = np.array([-u[1], u[0], 0.0])
            return n if n[1] > 0 else -n

        tr_lines = []
        for a, b in slopes:
            n = outward(a, b)
            m = (a + b) / 2
            tr_lines.append((m - n * 0.05, m + n * 0.65))
        wall_up, wall_dn = hc[1] + 0.7, hc[1] - 0.7
        for y in (wall_up, wall_dn):
            tr_lines.append((np.array([x_r - 0.05, y, 0.0]), np.array([x_r + 0.7, y, 0.0])))
        tr_paths = [smooth_path([a, (a + b) / 2, b]) for a, b in tr_lines]
        tr_guides = flow_guides(tr_paths, P_RED, opacity=0.3)
        tr_tag = Text("Wände, Dach — Transmission", font_size=LABEL_FONT_SIZE, color=P_RED)
        tr_tag.move_to(np.array([x_r + 0.9 + tr_tag.width / 2, wall_up, 0.0]))

        self.play(FadeIn(house2["group"]), FadeIn(sun), run_time=0.9)
        self.play(LaggedStart(*[Create(r) for r in rays["out"]], lag_ratio=0.25), FadeIn(left_tags[0]),
                  Create(left_leaders[0]), run_time=1.1)
        self.play(Indicate(win_hi["sash"], color=P_YELLOW, scale_factor=1.0),
                  LaggedStart(*[Create(r) for r in rays["in"]], lag_ratio=0.25), run_time=0.8)
        self.play(FadeIn(sun_patch, scale=0.6), run_time=0.4)
        self.play(ReplacementTransform(house2["level_1"].copy(), slab), FadeIn(right_tags[0]),
                  Create(right_leaders[0]), sun_patch.animate.set_stroke(color=P_ORANGE, opacity=0.9), run_time=1.0)
        self.play(_open_window(win_lo), FadeIn(left_tags[1]), Create(left_leaders[1]), run_time=0.9)
        self.play(FadeIn(guides), run_time=0.4)
        animate_flows(self, air, run_time=4.0, waves=5, cycles=2.0, radius=0.06,
                      extra=[LaggedStart(*[ShowPassingFlash(b.copy(), time_width=0.35) for b in beams * 3],
                                         lag_ratio=0.3)])
        hold_for(self, N, "design")

        caption = swap_caption(self, caption, subtitle_text(N, "envelope"))
        self.play(FadeIn(tr_guides), FadeIn(tr_tag), run_time=0.8)
        animate_flows(self, [(tr_paths, P_RED, P_BLUE)], run_time=3.6, waves=4, cycles=2.0, radius=0.06)
        hold_for(self, N, "envelope")

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
         "The design heating load is determined for defined design conditions — DIN EN 12831 — and is not the annual energy use.",
         "Die Auslegungsheizlast wird für definierte Auslegungsbedingungen bestimmt (DIN EN 12831) und ist nicht der Jahresenergieverbrauch."),
        ("demand",
         "The annual heating demand in kilowatt-hours is the area under the power curve over the year — DIN V 18599.",
         "Der Jahres-Heizwärmebedarf in kWh ist die Fläche unter der Leistungskurve übers Jahr (DIN V 18599)."),
        ("summer",
         "From May to September, sun and internal heat cover the losses — the heating only runs on cool days.",
         "Von Mai bis September decken Sonne und innere Wärme die Verluste — geheizt wird nur an kühlen Tagen."),
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
        _reveal(self, p_read, t_read, e_read)
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
        ladder = _log_ladder(1.0, 13.3, x=-1.4, y_lo=-1.6, y_hi=2.25, unit="W")
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
        dots, labels, leaders = _ladder_marks(ladder, entries, x_text=-0.75)
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
                span = dim_arrow(np.array([-4.0, ladder["y_of"](30), 0.0]),
                                 np.array([-4.0, ladder["y_of"](9e12), 0.0]), color=P_YELLOW)
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
        weather = np.convolve(np.random.default_rng(3).normal(0.0, 1.0, 408), np.ones(21) / 21, "same")[21:387]
        drive = 0.26 + np.cos(TAU * (days - 15) / 365.0) + 0.25 * weather / weather.std()
        raw = 0.07 * np.log1p(np.exp(drive / 0.07))
        kw_day = raw * 15000.0 / (24.0 * raw[:365].sum())
        cum = np.concatenate([[0.0], np.cumsum(kw_day[:365] * 24.0)])

        def ypt(d, p):
            return YO + RIGHT * d * DX + UP * p * DY

        yax = _axes(YO, 10.7, 3.0, "t", r"P\;[\mathrm{kW}]")
        chart_title = _m(r"\text{Heizleistung}\; P \;\text{über das Jahr (schematisch)}", size=LABEL_FONT_SIZE,
                         color=P_TEAL).move_to(UP * NOTE_BAND_Y)
        months = VGroup(*[
            Text(m, font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(ypt(15 + 30.4 * i, 0) + DOWN * 0.24)
            for i, m in enumerate(("Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"))
        ])
        curve = VMobject(color=P_CYAN, stroke_width=3)
        curve.set_points_smoothly([ypt(d, kw_day[d]) for d in range(0, 366, 2)])
        design = DashedLine(ypt(0, 8), ypt(365, 8), color=P_RED, stroke_width=2.5, dash_length=0.12)
        design_lbl = _m(r"\text{Auslegungsheizlast}\; P_{\mathrm{HL}} \approx 8\,\mathrm{kW}", ypt(200, 8) + UP * 0.2,
                        color=P_RED, size=LABEL_FONT_SIZE, edge="left")
        p_ticks2 = VGroup(*[_m(str(p), ypt(0, p) + LEFT * 0.22 + DOWN * 0.07, size=LABEL_FONT_SIZE, edge="right")
                            for p in (2, 4, 6, 8)])
        self.play(FadeIn(yax["group"]), FadeIn(months), FadeIn(p_ticks2), FadeIn(chart_title), run_time=0.8)
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
        self.add(fill)
        _reveal(self, q_read)
        self.play(sweep.animate.set_value(365), run_time=4.0, rate_func=linear)
        year_lbl = _m(r"\text{Heizwärmebedarf} \approx 15\,000\,\mathrm{kWh/a}", ypt(200, 6.2), color=P_CYAN,
                      edge="left")
        self.play(ReplacementTransform(q_read, year_lbl), run_time=0.6)
        hold_for(self, N, "demand")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        summer_tag = centered_body_text("Sommer: keine Heizung\nnotwendig bzw. Gewinne\ndecken die Verluste", font_size=LABEL_FONT_SIZE,
                                        color=P_YELLOW).move_to(ypt(197, 3.0))
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
        ("losses",
         "But not everything gets in: the glass reflects 20 joules. And what is stored gives 10 joules back later.",
         "Nicht alles kommt hinein: Das Glas reflektiert 20 J. Und Gespeichertes gibt später 10 J wieder ab."),
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
        form_x0, form_w, form_y = 1.6, 4.2, 0.75

        def form_bar(parts):
            bar, x = VGroup(), form_x0
            for frac, color in parts:
                bar.add(Rectangle(width=form_w * frac, height=0.5, stroke_width=0, fill_color=color, fill_opacity=0.7)
                        .move_to(np.array([x + form_w * frac / 2, form_y, 0.0])))
                x += form_w * frac
            return bar

        forms = [form_bar([(1.0, P_CYAN)]), form_bar([(0.05, P_YELLOW), (0.95, P_RED)]), form_bar([(1.0, P_RED)])]
        bar_base = form_y - 0.09

        def in_bar_lbl(src, frac_from, frac_to):
            cx = form_x0 + form_w * (frac_from + frac_to) / 2
            return _m(src, np.array([cx, bar_base, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE)

        form_lbls = [
            VGroup(in_bar_lbl(r"\text{elektrisch}\;100\,\mathrm{J}\;(100\,\%)", 0.0, 1.0)),
            VGroup(in_bar_lbl(r"\text{Wärme}\;95\,\mathrm{J}\;(95\,\%)", 0.05, 1.0),
                   _m(r"\text{Licht}\;5\,\mathrm{J}\;(5\,\%)", np.array([form_x0, form_y + 0.45, 0.0]),
                      size=LABEL_FONT_SIZE, color=P_YELLOW, edge="left")),
            VGroup(in_bar_lbl(r"\text{Wärme}\;100\,\mathrm{J}\;(100\,\%)", 0.0, 1.0)),
        ]
        form_brace = Brace(forms[0], DOWN, color=P_WHITE)
        form_sum = _m(r"E = \text{konstant}", form_brace.get_bottom() + DOWN * 0.38, color=P_WHITE)
        law_lamp = _lamp(np.array([room["center"][0], room["y_c"], 0.0]))
        law_bulb = law_lamp["bulb"].get_center()
        law_cable = Line(law_lamp["cable"].get_start(), law_lamp["cable"].get_end())
        law_rays = VGroup(*[
            Line(law_bulb, end, color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.85)
            for end in (np.array([room["x_l"] + 0.05, law_bulb[1] - 0.3, 0.0]),
                        np.array([room["center"][0] + 0.4, room["y_f"] + 0.02, 0.0]),
                        np.array([room["x_r"] - 0.05, law_bulb[1] - 0.6, 0.0]))
        ])
        law_glow = Circle(radius=0.3, stroke_width=0, fill_color=P_RED, fill_opacity=0.25).move_to(law_bulb)

        def law_heat(rt: float):
            return _ripples([law_bulb], r_max=0.8, color=P_RED, down=True, cycles=max(1.0, rt / 1.4))

        _paced_flow(self, [([law_cable], P_CYAN, None, None, False)], run_time=4.0, waves=3, cycles=2.5, radius=0.045,
                    extra=[FadeIn(room["group"]), FadeIn(law_lamp["group"]),
                           GrowFromEdge(forms[0], LEFT, rate_func=linear)])
        self.play(FadeIn(form_lbls[0]), run_time=0.7)
        self.play(GrowFromCenter(form_brace), FadeIn(form_sum), run_time=1.0)
        _paced_flow(self, [([law_cable], P_CYAN, None, None, False)], run_time=2.0, waves=3, cycles=1.5, radius=0.045)
        self.play(FadeOut(form_lbls[0]), run_time=0.5)
        self.play(ReplacementTransform(forms[0], forms[1], rate_func=linear), FadeIn(law_glow, scale=0.4),
                  LaggedStart(*[Create(r) for r in law_rays], lag_ratio=0.2), law_heat(4.0), run_time=4.0)
        self.play(FadeIn(form_lbls[1]), run_time=0.7)
        self.play(law_heat(2.5), run_time=2.5)
        self.play(FadeOut(form_lbls[1]), run_time=0.5)
        self.play(ReplacementTransform(forms[1], forms[2], rate_func=linear), law_rays.animate.set_color(P_ORANGE),
                  LaggedStart(*[Flash(r.get_end(), color=P_ORANGE, flash_radius=0.18, line_length=0.12)
                                for r in law_rays], lag_ratio=0.3),
                  room["air"].animate.set_fill(P_RED, opacity=0.1), law_heat(4.0), run_time=4.0)
        self.play(FadeIn(form_lbls[2]), FadeOut(law_rays), run_time=0.7)
        hold_for(self, N, "law", used=BEAT_SUBTITLE_FADE + 0.3 + 24.5, during=lambda rt: [law_heat(rt)])

        caption = swap_caption(self, caption, subtitle_text(N, "balance"))
        mid_y = room["center"][1]
        e_in = Arrow(np.array([-6.9, mid_y + 0.3, 0.0]), np.array([room["glass_x"] - 0.05, mid_y + 0.3, 0.0]),
                     buff=0, color=P_CYAN, stroke_width=5)
        e_out = Arrow(np.array([room["x_r"] + 0.25, mid_y - 0.3, 0.0]), np.array([1.6, mid_y - 0.3, 0.0]),
                      buff=0, color=P_RED, stroke_width=5)
        bx, bw = 2.0, 3.8

        def bar(y, x0, frac, width, color):
            return Rectangle(width=max(0.002, width * frac), height=0.42, stroke_width=0, fill_color=color,
                             fill_opacity=0.7).move_to(np.array([x0 + width * frac / 2, y, 0.0]))

        def lost_bar(y, x0, frac, width, color, ghost_opacity: float = 0.15):
            ghost = Rectangle(width=width * frac, height=0.42, stroke_width=0, fill_color=color,
                              fill_opacity=ghost_opacity)
            outline = DashedVMobject(Rectangle(width=width * frac, height=0.42, color=P_WHITE, stroke_width=2.5),
                                     num_dashes=14)
            return VGroup(ghost, outline).move_to(np.array([x0 + width * frac / 2, y, 0.0]))

        in_bar = bar(1.15, bx, 0.8, bw, P_CYAN)
        out_bar = bar(0.0, bx, 0.5, bw, P_RED)
        st_bar = bar(0.0, bx + bw * 0.5, 0.3, bw, P_ORANGE)
        refl_bar = lost_bar(1.15, bx + bw * 0.8, 0.2, bw, P_BLUE)
        st_lost = lost_bar(0.0, bx + bw * 0.7, 0.1, bw, P_ORANGE, ghost_opacity=0.0)
        in_t = _m(r"\text{hinein}\;80\,\mathrm{J}", np.array([bx, 1.63, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN,
                  edge="left")
        out_t = _m(r"\text{hinaus}\;50\,\mathrm{J}", np.array([bx, -0.48, 0.0]), size=LABEL_FONT_SIZE,
                   color=P_RED, edge="left")
        st_t = _m(r"\text{gespeichert}\;30\,\mathrm{J}", np.array([bx + bw * 0.5, -0.48, 0.0]),
                  size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        refl_t = _m(r"\text{Verlust}\;20\,\mathrm{J}", np.array([bx + bw, 1.63, 0.0]), size=LABEL_FONT_SIZE,
                    color=P_BLUE, edge="right")
        lost_t = _m(r"10\,\mathrm{J}\;\text{später wieder hinaus}", np.array([bx + bw * 0.8, 0.5, 0.0]),
                    size=LABEL_FONT_SIZE, color=P_ORANGE, edge="right")
        lost_leader = Line(np.array([bx + bw * 0.75, 0.38, 0.0]), np.array([bx + bw * 0.75, 0.24, 0.0]),
                           color=P_ORANGE, stroke_width=1.6, stroke_opacity=0.8)
        refl_arrow = Arrow(np.array([room["glass_x"] - 0.05, mid_y + 0.3, 0.0]),
                           np.array([room["glass_x"] - 0.72, mid_y + 0.97, 0.0]), buff=0, color=P_BLUE,
                           stroke_width=4, stroke_opacity=0.9, max_tip_length_to_length_ratio=0.2)
        refl_tag = Text("Reflexion", font_size=LABEL_FONT_SIZE - 2, color=P_BLUE)
        refl_tag.move_to(refl_arrow.get_end() + UP * 0.33)
        row, box, items = _panel([
            ("in", r"E_{\mathrm{ein}}", P_CYAN), (None, "=", P_WHITE), ("out", r"E_{\mathrm{aus}}", P_RED),
            (None, "+", P_WHITE), ("st", r"\Delta E_{\mathrm{Speicher}}", P_ORANGE),
        ])
        self.play(FadeOut(VGroup(forms[2], form_lbls[2], form_brace, form_sum, law_lamp["group"], law_glow)),
                  room["air"].animate.set_fill(P_RED, opacity=0.0), run_time=0.6)
        self.play(FadeIn(e_in), FadeIn(in_bar), FadeIn(in_t), run_time=1.4)
        self.play(FadeIn(e_out), FadeIn(out_bar), FadeIn(st_bar), FadeIn(out_t), FadeIn(st_t),
                  room["ceiling"].animate.set_fill(P_ORANGE, opacity=0.55), run_time=1.4)
        self.play(FadeIn(row), Create(box), run_time=0.8)
        hold_for(self, N, "balance")

        caption = swap_caption(self, caption, subtitle_text(N, "losses"))
        self.play(GrowArrow(refl_arrow), FadeIn(refl_tag), FadeIn(refl_bar), FadeIn(refl_t), run_time=1.6)
        self.play(FadeIn(st_lost), FadeIn(lost_t), Create(lost_leader), run_time=1.6)
        hold_for(self, N, "losses", during=lambda rt: [
            ShowPassingFlash(refl_arrow.copy().set_stroke(width=8), time_width=0.6, run_time=rt),
            Indicate(st_lost, color=P_ORANGE, scale_factor=1.0, run_time=rt)])
        #endregion

        #region lamp and split
        self.play(FadeOut(VGroup(e_in, e_out, in_t, out_t, st_t, row, box, refl_arrow, refl_tag, refl_bar, refl_t, st_lost,
                                 lost_t, lost_leader)), FadeOut(in_bar), FadeOut(out_bar), FadeOut(st_bar), room["ceiling"].animate.set_fill(P_WHITE, opacity=0.14), run_time=0.6)
        caption = swap_caption(self, caption, subtitle_text(N, "lamp"))
        lamp = _lamp(np.array([room["center"][0], room["y_c"], 0.0]))
        p_el = ValueTracker(0.0)
        p_read = _readout(lambda: rf"P_{{\mathrm{{el}}}} = {_de(p_el.get_value())}\,\mathrm{{W}}",
                          lamp["cable"].get_center() + RIGHT * 0.3, color=P_CYAN, edge="left")
        self.play(FadeIn(lamp["group"]), run_time=0.6)
        _reveal(self, p_read)
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
        heat_waves = Circle(radius=0.3, stroke_width=0, fill_color=P_RED, fill_opacity=0.25).move_to(bulb_c)
        self.play(FadeIn(src), FadeIn(src_lbl), run_time=0.6)
        self.add(light_lane, heat_lane)
        _reveal(self, light_read, heat_read)
        self.play(f_l.animate.set_value(1.0), f_h.animate.set_value(1.0), FadeIn(heat_waves, scale=0.4),
                  _ripples([bulb_c], r_max=0.7, color=P_RED, down=True),
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
        self.play(room["air"].animate.set_fill(P_RED, opacity=0.1), _ripples([bulb_c], r_max=1.1, color=P_RED, down=True),
                  run_time=1.6)
        hold_for(self, N, "total")

        caption = swap_caption(self, caption, subtitle_text(N, "person"))
        person = _person(np.array([room["x_r"] - 0.9, room["y_f"] + 0.55, 0.0]), scale=1.9)
        person.shift(UP * (room["y_f"] + 0.08 - person.get_bottom()[1]))
        head = person.get_top()
        plume = [smooth_path([head + UP * 0.08, head + UP * 0.45 + RIGHT * 0.1, head + UP * 0.8 + LEFT * 0.05,
                              np.array([head[0] + 0.1, room["y_c"] - 0.08, 0.0])])]
        person_lbl = _m(r"\approx 100\,\mathrm{W}", person.get_left() + LEFT * 0.2 + UP * 0.05, size=LABEL_FONT_SIZE,
                        color=P_ORANGE, edge="right")
        self.play(FadeIn(person), FadeIn(person_lbl), run_time=0.6)
        animate_flows(self, [(plume, P_ORANGE, P_RED)], run_time=1.8, waves=4, cycles=1.5, radius=0.055,
                      extra=[_ripples([person.get_center() + UP * 0.2], r_max=0.6, color=P_ORANGE)])
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
        wx = 2.2

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
        self.add(w_bars)
        _reveal(self, heat_read2, int_read)
        self.play(f_int.animate.set_value(1.0), Indicate(person, color=P_ORANGE), Indicate(lamp["bulb"]),
                  run_time=2.0)
        hold_for(self, N, "winter")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        sx = 5.1
        f_sun = ValueTracker(0.0)
        sun_c = np.array([-5.8, 2.05, 0.0])
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
        rays = _sun_rays(sun_c, room["glass_x"], [room["win_hi"] - 0.2 - 0.35 * k for k in range(3)], room["y_f"],
                         gap=0.42)
        self.play(FadeOut(loss_arrows), FadeIn(sun), FadeIn(s_title), run_time=0.7)
        self.add(s_bars)
        _reveal(self, cool_read)
        self.play(_shine(rays), f_sun.animate.set_value(1.0), room["air"].animate.set_fill(P_RED, opacity=0.12),
                  run_time=2.0)
        hold_for(self, N, "summer")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Heat pump: lifting energy to a higher temperature level
class Beat4_Waermepumpe(Scene):
    NARRATION = [
        ("move",
         "A heat pump does not create heat. It transfers energy from the cold outdoor air to a higher temperature level, into the warm house.",
         "Eine Wärmepumpe erzeugt keine Wärme. Sie überträgt Energie aus der kalten Außenluft auf ein höheres Temperaturniveau — ins warme Haus."),
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
         "Heat divided by electricity is the coefficient of performance, COP — here four.",
         "Heizwärme durch Strom ist die Leistungszahl COP — hier 4."),
        ("temp",
         "The colder the outdoor air, the more the compressor has to lift — the COP drops.",
         "Je kälter die Außenluft, desto mehr muss der Verdichter leisten — der COP sinkt."),
        ("jaz",
         "Averaged over a year this is the seasonal performance factor JAZ — for air-source units often around three.",
         "Übers Jahr gemittelt ergibt sich die Jahresarbeitszahl JAZ — bei Luft-Wärmepumpen oft rund 3."),
        ("final",
         "Final energy is what is delivered to the building and paid for: fuel in the tank, electricity at the meter.",
         "Endenergie ist, was ans Haus geliefert und bezahlt wird: Brennstoff im Tank, Strom am Zähler."),
        ("compare1",
         "Per kilowatt-hour of final energy: wood chips give about 0.85 kilowatt-hours of heat, oil about 0.94, gas about 0.96.",
         "Je 1 kWh Endenergie liefern Holzhackschnitzel rund 0,85 kWh Wärme, Heizöl rund 0,94, Erdgas rund 0,96."),
        ("compare2",
         "At the house an electric heater gives exactly one. With fuels the rest leaves with the flue gas at the house; electricity may cause flue gas elsewhere.",
         "Am Haus liefert ein Elektroheizstab genau 1. Bei Brennstoffen geht der Rest mit dem Abgas verloren — Strom verursacht gegebenenfalls anderswo Abgas."),
        ("power",
         "But electricity is often made from fuels too: then the flue gas arises at the power station, not in the house.",
         "Auch Strom entsteht oft aus Brennstoffen: Dann entsteht das Abgas im Kraftwerk, nicht im Haus."),
        ("compare3",
         "The air-source heat pump gives about three, the ground-source about four — because it adds ambient heat.",
         "Die Luft-Wärmepumpe liefert rund 3, die Erd-Wärmepumpe rund 4 — weil sie Umweltwärme dazuholt."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmepumpe — Energie auf höheres Temperaturniveau übertragen", title)
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
        air_paths = [smooth_path([np.array([-6.6, y, 0.0]), np.array([-2.75, y, 0.0])]) for y in (-0.15, 0.4, 0.95)]
        air_rays = flow_guides(air_paths, P_GREEN, opacity=0.3)
        air_lbl = _m(r"\text{Außenluft}\;0\,\mathrm{°C}", np.array([-5.1, -0.6, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_GREEN)
        power = Arrow(np.array([-1.3, 2.3, 0.0]), comp.get_top(), buff=0.02, color=P_CYAN, stroke_width=5,
                      max_tip_length_to_length_ratio=0.3)
        power_lbl = Text("Strom", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(power, LEFT, buff=0.12)
        radiator = _radiator(np.array([5.2, 0.38, 0.0]))
        pipe_pts = [np.array([3.2, 0.75, 0.0]), np.array([4.75, 0.75, 0.0]), np.array([4.75, 0.0, 0.0]),
                    np.array([3.2, 0.0, 0.0])]
        hot_rays = VGroup(Line(pipe_pts[0], pipe_pts[1], color=P_RED, stroke_width=3),
                          Line(pipe_pts[2], pipe_pts[3], color=P_ORANGE, stroke_width=3))
        water_path = VMobject().set_points_as_corners(pipe_pts + pipe_pts[:1])
        flows = {"air": False, "water": False}
        rad_lbl = _m(r"\text{Heizwasser}\;35\,\mathrm{°C}", np.array([4.9, -0.3, 0.0]), size=LABEL_FONT_SIZE,
                     color=P_RED)
        refrigerant = _gradient([(0.0, P_BLUE), (0.13, P_CYAN), (0.24, P_CYAN), (0.27, P_RED), (0.5, P_RED),
                                 (0.63, P_ORANGE), (0.86, P_TEAL), (0.9, P_BLUE), (1.0, P_BLUE)])

        def cycle(run_time=3.0, extra=None):
            streams = [([loop_path], P_CYAN, None, refrigerant, True)]
            if flows["air"]:
                streams.append((air_paths, P_GREEN, None, _gradient([(0.0, P_GREEN), (0.8, P_GREEN), (1.0, P_BLUE)]),
                                False, 4))
            if flows["water"]:
                streams.append(([water_path], P_RED, None, _gradient([(0.0, P_RED), (0.3, P_RED), (0.5, P_ORANGE),
                                                                       (1.0, P_ORANGE)]), True, 8))
            _paced_flow(self, streams, run_time=run_time, waves=22, cycles=run_time / 3.0, radius=0.07, extra=extra)

        self.play(FadeIn(cold_bg), Create(wall), FadeIn(out_tag), FadeIn(in_tag), Create(loop), FadeIn(evap),
                  FadeIn(cond), FadeIn(comp), FadeIn(valve), FadeIn(radiator), run_time=1.4)
        cycle(2.4)
        hold_for(self, N, "move", used=BEAT_SUBTITLE_FADE + 0.3 + 3.8)

        caption = swap_caption(self, caption, subtitle_text(N, "evap"))
        self.play(FadeIn(lbl_evap), FadeIn(air_lbl), FadeIn(air_rays), Indicate(evap, color=P_CYAN), run_time=1.2)
        flows["air"] = True
        cycle()
        hold_for(self, N, "evap")

        caption = swap_caption(self, caption, subtitle_text(N, "comp"))
        self.play(FadeIn(lbl_comp), GrowArrow(power), FadeIn(power_lbl), run_time=0.9)
        cycle(extra=[Rotate(comp[1:], angle=TAU * 2, about_point=comp_c)])
        hold_for(self, N, "comp")

        caption = swap_caption(self, caption, subtitle_text(N, "cond"))
        self.play(FadeIn(lbl_cond), LaggedStart(*[Create(r) for r in hot_rays], lag_ratio=0.2), FadeIn(rad_lbl),
                  Indicate(cond, color=P_RED), run_time=1.2)
        flows["water"] = True
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
        flows["water"] = False

        def block(x, h, color, width=0.55, y0=base):
            return Rectangle(width=width, height=h, stroke_width=1.5, color=color, fill_color=color,
                             fill_opacity=0.65).move_to(np.array([x, y0 + h / 2, 0.0]))

        stack_x = 5.3
        w_bar = block(stack_x, unit, P_CYAN, width=0.62)
        u_bar = block(stack_x, 3 * unit, P_GREEN, width=0.62, y0=base + unit)
        w_lbl = _m(r"W_{\mathrm{el}} = 1\,\mathrm{kWh}", np.array([stack_x - 0.42, base + unit / 2 - 0.07, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")
        u_lbl = _m(r"Q_{\mathrm{U}} = 3\,\mathrm{kWh}", np.array([stack_x - 0.42, base + 2.5 * unit - 0.07, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_GREEN, edge="right")
        self.play(Indicate(power, color=P_CYAN), GrowFromEdge(w_bar, DOWN), FadeIn(w_lbl), run_time=0.9)
        self.play(Indicate(air_rays, color=P_GREEN), GrowFromEdge(u_bar, DOWN), FadeIn(u_lbl), run_time=0.9)
        h_frame = Rectangle(width=0.72, height=4 * unit + 0.1, color=P_RED, stroke_width=3).move_to(
            np.array([stack_x, base + 2 * unit, 0.0]))
        h_lbl = _m(r"Q_{\mathrm{H}} = 4\,\mathrm{kWh}", np.array([stack_x - 0.42, h_frame.get_top()[1] + 0.05, 0.0]),
                   size=LABEL_FONT_SIZE, color=P_RED, edge="right")
        self.play(Create(h_frame), FadeIn(h_lbl), run_time=0.7)
        hold_for(self, N, "balance")

        caption = swap_caption(self, caption, subtitle_text(N, "cop"))
        tower = VGroup(*[block(6.1, unit, P_CYAN, width=0.4, y0=base + k * unit) for k in range(4)])
        count = ValueTracker(0)
        count_read = _readout(lambda: rf"\times\,{int(count.get_value())}", np.array([6.1, base + 4 * unit + 0.25, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
        _reveal(self, count_read)
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
                          np.array([stack_x - 0.42, base + 0.2, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="right")
        self.play(FadeOut(air_rays), FadeIn(thermo["group"]), FadeIn(thermo["column"]), run_time=0.6)
        self.add(stack)
        _reveal(self, cop_read, t_read, w_live)
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

        #region final energy
        caption = swap_caption(self, caption, subtitle_text(N, "final"))
        thermo["level"].clear_updaters()
        self.play(*[FadeOut(m) for m in self.mobjects if m not in (title, subtitle, caption)], run_time=0.7)
        fe_house = _build_cross_section_house(center=np.array([1.8, -0.3, 0.0]))
        fe_limit_x = -0.9
        fe_limit = DashedLine(np.array([fe_limit_x, -1.7, 0.0]), np.array([fe_limit_x, 1.9, 0.0]), color=P_WHITE,
                              stroke_width=2, dash_length=0.12)
        fe_limit_tag = Text("Gebäudegrenze", font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(fe_limit, UP, buff=0.1)
        fe_fuel = Arrow(np.array([-5.6, 0.6, 0.0]), np.array([fe_limit_x - 0.05, 0.6, 0.0]), buff=0, color=P_ORANGE,
                        stroke_width=5)
        fe_power = Arrow(np.array([-5.6, -0.6, 0.0]), np.array([fe_limit_x - 0.05, -0.6, 0.0]), buff=0, color=P_CYAN,
                         stroke_width=5)
        fe_fuel_lbl = Text("Brennstoff im Tank", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(fe_fuel, UP, buff=0.12,
                                                                                                   aligned_edge=LEFT)
        fe_power_lbl = Text("Strom am Zähler", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(fe_power, UP, buff=0.12,
                                                                                               aligned_edge=LEFT)
        fe_brace = Brace(VGroup(fe_fuel, fe_fuel_lbl), UP, color=P_YELLOW).stretch_to_fit_width(4.6, about_edge=RIGHT)
        fe_brace.move_to(np.array([(-5.6 + fe_limit_x) / 2, fe_fuel_lbl.get_top()[1] + 0.3, 0.0]))
        fe_lbl = Text("Endenergie", font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(fe_brace, UP, buff=0.12)
        fe_heater = _radiator(np.array([2.9, -1.2, 0.0]))
        fe_heat_lbl = Text("Nutzwärme", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([1.8, 0.45, 0.0]))
        self.play(FadeIn(fe_house["group"]), Create(fe_limit), FadeIn(fe_limit_tag), run_time=1.0)
        self.play(GrowArrow(fe_fuel), FadeIn(fe_fuel_lbl), GrowArrow(fe_power), FadeIn(fe_power_lbl), run_time=1.4)
        self.play(GrowFromCenter(fe_brace), FadeIn(fe_lbl), run_time=0.8)
        self.play(FadeIn(fe_heater), FadeIn(fe_heat_lbl),
                  _ripples([fe_heater.get_top() + UP * 0.04], r_max=0.7, color=P_RED), run_time=2.0)
        hold_for(self, N, "final", during=lambda rt: [
            _ripples([fe_heater.get_top() + UP * 0.04], r_max=0.7, color=P_RED, cycles=max(1.0, rt / 1.4))])
        #endregion

        #region compare
        self.play(*[FadeOut(m) for m in self.mobjects if m not in (title, subtitle, caption)], run_time=0.7)
        caption = swap_caption(self, caption, subtitle_text(N, "compare1"))
        cb, cu = -1.3, 0.74
        xs = [-4.8, -2.9, -1.0, 0.9, 2.8, 4.7]
        specs = [
            ("Holz-\nhackschnitzel", 0.85, "fuel"), ("Heizöl\nBrennwert", 0.94, "fuel"),
            ("Erdgas\nBrennwert", 0.96, "fuel"), ("Elektro-\nheizstab", 1.0, "el"),
            ("Luft-WP\n(JAZ)", 3.0, "hp"), ("Erd-WP\n(JAZ)", 4.0, "hp"),
        ]
        base_line = Line(np.array([-5.3, cb, 0.0]), np.array([5.9, cb, 0.0]), color=P_WHITE, stroke_width=2)
        one_line = DashedLine(np.array([-5.3, cb + cu, 0.0]), np.array([5.9, cb + cu, 0.0]), color=P_WHITE,
                              stroke_width=1.6, stroke_opacity=0.6, dash_length=0.1)
        one_lbl = _m(r"1\,\mathrm{kWh}", np.array([-6.2, cb + cu - 0.06, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE,
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
        loss_lbl = Text("Abgas am Haus", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(np.array([-3.4, cb + cu + 0.55, 0.0]))
        self.play(grow[3].animate.set_value(1.0), run_time=1.0)
        self.play(FadeIn(values[3]), run_time=0.4)
        self.play(Create(losses), FadeIn(loss_lbl), run_time=0.8)
        hold_for(self, N, "compare2")

        caption = swap_caption(self, caption, subtitle_text(N, "power"))
        plant_start, plant_end = np.array([xs[3], cb + cu + 0.08, 0.0]), np.array([xs[3], cb + cu + 1.05, 0.0])
        plant_arrow = Arrow(plant_start, plant_end, buff=0, color=P_ORANGE, stroke_width=4, stroke_opacity=0.75,
                            max_tip_length_to_length_ratio=0.25)
        plant_lbl = Text("Abgas im Kraftwerk", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            plant_end + UP * 0.3)
        self.play(GrowArrow(plant_arrow), FadeIn(plant_lbl, shift=UP * 0.1), run_time=1.0)
        self.play(ShowPassingFlash(plant_arrow.copy().set_stroke(width=7), time_width=0.5), run_time=1.0)
        hold_for(self, N, "power")

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
         "Solar gain is g times window area times irradiance.",
         "Solarer Gewinn: g mal Fensterfläche mal Einstrahlung."),
        ("absorb",
         "Inside, the radiation hits floor and walls. The surfaces warm up.",
         "Drinnen trifft die Strahlung auf Boden und Wände. Die Oberflächen erwärmen sich."),
        ("long",
         "Warm surfaces radiate too — but long-wave, in the infrared.",
         "Warme Oberflächen strahlen selbst — aber langwellig, im Infrarot."),
        ("trap",
         "Glass hardly lets long-wave radiation through: plain glass absorbs it, a low-e coating reflects it back into the room.",
         "Langwellige Strahlung lässt Glas kaum durch: Vollglas nimmt sie auf, eine Low-E-Beschichtung wirft sie in den Raum zurück."),
        ("warm",
         "The energy stays trapped and the room warms up — the greenhouse effect.",
         "Die Energie bleibt gefangen, der Raum erwärmt sich — der Treibhauseffekt."),
        ("winter",
         "In winter this gain saves heating energy.",
         "Im Winter spart dieser Gewinn Heizwärme."),
        ("summer",
         "In summer it overheats the room — so shade outside, before the radiation reaches the glass (DIN 4108-2).",
         "Im Sommer überhitzt er den Raum — darum Sonnenschutz außen, bevor die Strahlung das Glas erreicht."),
        ("inner",
         "With shading on the inside the radiation is already in the room: it warms the slats, and they radiate long-wave into the room.",
         "Innen liegender Sonnenschutz: Die Strahlung ist schon im Raum, erwärmt die Lamellen — sie strahlen langwellig in den Raum."),
        ("compare",
         "The comparison: with outside shading the room stays at 22 degrees, with inside shading it rises to 30 — outside works better.",
         "Der Vergleich: Außen bleibt der Raum bei 22 °C, innen steigt er auf 30 °C — außen verschattet wirkt besser."),
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
        gx = room["glass_x"]
        sun_dir = np.array([1.0, -0.5, 0.0]) / np.linalg.norm([1.0, -0.5])
        sun_c = np.array([gx, (room["win_lo"] + room["win_hi"]) / 2, 0.0]) - sun_dir * 3.25
        sun = _build_sun(sun_c).scale(0.42)
        bounce_dir = np.array([-sun_dir[0], sun_dir[1], 0.0])

        def hit_point(g_pt):
            t_floor = (room["y_f"] + 0.02 - g_pt[1]) / sun_dir[1]
            t_wall = (room["x_r"] - 0.02 - g_pt[0]) / sun_dir[0]
            return g_pt + sun_dir * min(t_floor, t_wall)

        entries = [np.array([gx, y, 0.0]) for y in np.linspace(room["win_lo"] + 0.12, room["win_hi"] - 0.12, 8)]
        starts = [e - sun_dir * 2.3 for e in entries]
        hits = [hit_point(e) for e in entries]
        beams_out = VGroup(*[_wave([s, e], "short") for s, e in zip(starts, entries)])
        reflected = VGroup(*[_wave([e, e + bounce_dir * 0.55], "short", opacity=0.4) for e in entries])
        corner = [np.array([room["x_r"] - 0.02, room["y_f"] + 0.02, 0.0])] if hits[-1][0] > room["x_r"] - 0.05 else []
        shaft = Polygon(entries[0], hits[0], *corner, hits[-1], entries[-1], stroke_width=0, fill_color=P_YELLOW,
                        fill_opacity=0.0)
        patch = VMobject(color=P_YELLOW, stroke_width=6, stroke_opacity=0.85)
        patch.set_points_as_corners([hits[0], *corner, hits[-1]])
        pass_paths = [[starts[k], entries[k], hits[k]] for k in (1, 3, 5, 7)]
        split_paths = [[starts[k], entries[k], entries[k] + bounce_dir * 0.55] for k in (0, 2, 4, 6)]
        i_lbl = _m(r"I = 500\,\mathrm{W/m^{2}}", sun_c + LEFT * 0.4 + DOWN * 0.8, size=LABEL_FONT_SIZE,
                   color=P_YELLOW)
        g_lbl = _m(r"g = 0{,}50", np.array([4.6, 1.95, 0.0]), color=P_YELLOW)
        gi_lbl = _m(r"g \cdot I \approx 250\,\mathrm{W/m^{2}}", np.array([gx + 0.3, room["y_c"] - 0.32, 0.0]),
                    size=LABEL_FONT_SIZE, color=P_YELLOW, edge="left")
        self.play(FadeIn(room["group"]), FadeIn(sun), run_time=1.0)
        self.play(LaggedStart(*[Create(b) for b in beams_out], lag_ratio=0.08), run_time=1.0)
        self.play(shaft.animate.set_fill(opacity=0.16), Create(patch), run_time=0.8)
        _beam_pulses(self, pass_paths, P_YELLOW, repeats=2, run_time=3.0, kind="short")
        hold_for(self, N, "sun", used=BEAT_SUBTITLE_FADE + 0.3 + 5.8)

        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        self.play(LaggedStart(*[Create(r) for r in reflected], lag_ratio=0.08),
                  Indicate(room["glass"], color=P_ORANGE, scale_factor=1.0), FadeIn(i_lbl), run_time=0.9)
        _beam_pulses(self, pass_paths + split_paths, P_YELLOW, repeats=1, run_time=2.6, kind="short",
                     extra=[FadeIn(g_lbl), FadeIn(gi_lbl, shift=RIGHT * 0.1)])
        norm_tag = Text("DIN EN 410", font_size=LABEL_FONT_SIZE, color=P_TEAL).next_to(g_lbl, DOWN, buff=0.12)
        self.play(FadeIn(norm_tag), run_time=0.4)
        hold_for(self, N, "g")
        #endregion

        #region gain
        caption = swap_caption(self, caption, subtitle_text(N, "gain"))
        pane_c = np.array([5.6, 0.1, 0.0])
        pane_w, pane_h = 1.0, 0.96
        pane = Rectangle(width=pane_w, height=pane_h, color=P_CYAN, stroke_width=2.5).move_to(pane_c)
        a_frac = ValueTracker(0.0)
        pane_fill = always_redraw(lambda: Rectangle(
            width=pane_w, height=max(0.002, pane_h * a_frac.get_value()), stroke_width=0, fill_color=P_CYAN,
            fill_opacity=0.35).move_to(pane.get_bottom() + UP * pane_h * a_frac.get_value() / 2))
        w_dim = dim_arrow(pane.get_corner(DL) + DOWN * 0.18, pane.get_corner(DR) + DOWN * 0.18, color=P_WHITE)
        h_dim = dim_arrow(pane.get_corner(DL) + LEFT * 0.18, pane.get_corner(UL) + LEFT * 0.18, color=P_WHITE)
        w_lbl = _m(r"1{,}25\,\mathrm{m}", w_dim.get_center() + DOWN * 0.32, size=LABEL_FONT_SIZE)
        h_lbl = _m(r"1{,}2\,\mathrm{m}", size=LABEL_FONT_SIZE).next_to(h_dim, LEFT, buff=0.08)
        a_read = _readout(lambda: rf"A = {_de(1.5 * a_frac.get_value(), 1)}\,\mathrm{{m^{{2}}}}",
                          pane.get_top() + UP * 0.3, size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
        self.play(FadeIn(pane), GrowFromCenter(w_dim), GrowFromCenter(h_dim), FadeIn(w_lbl), FadeIn(h_lbl),
                  FadeOut(reflected), FadeOut(gi_lbl), FadeOut(norm_tag), run_time=0.9)
        self.add(pane_fill)
        _reveal(self, a_read)
        self.play(a_frac.animate.set_value(1.0), run_time=1.4)
        row, box, items = _panel([
            (None, r"\dot{Q}_{\mathrm{S}}", P_ORANGE), (None, "=", P_WHITE), ("g", "g", P_YELLOW),
            (None, r"\cdot", P_WHITE), ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE), ("i", "I", P_YELLOW),
            (None, "=", P_WHITE), ("gv", r"0{,}5", P_YELLOW), (None, r"\cdot", P_WHITE),
            ("av", r"1{,}5\,\mathrm{m^{2}}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("iv", r"500\,\mathrm{W/m^{2}}", P_YELLOW), (None, "=", P_WHITE), ("r", r"375\,\mathrm{W}", P_ORANGE),
        ])
        _fill_panel(self, (row, box, items), {"gv": g_lbl, "av": a_read, "iv": i_lbl}, fly=False)
        hold_for(self, N, "gain")
        #endregion

        #region absorb, long-wave and trap
        caption = swap_caption(self, caption, subtitle_text(N, "absorb"))
        self.play(FadeOut(VGroup(pane, w_dim, h_dim, w_lbl, h_lbl)), FadeOut(pane_fill), FadeOut(a_read),
                  run_time=0.5)
        _beam_pulses(self, pass_paths, P_YELLOW, repeats=1, run_time=2.0, kind="short", extra=[
            LaggedStart(*[Flash(hits[k], color=P_ORANGE, flash_radius=0.12, line_length=0.08) for k in (1, 3, 5, 7)],
                        lag_ratio=0.2),
            patch.animate.set_stroke(color=P_ORANGE),
            room["floor"].animate.set_fill(P_ORANGE, opacity=0.4),
            room["wall_lo"].animate.set_fill(P_ORANGE, opacity=0.4),
            room["wall_hi"].animate.set_fill(P_ORANGE, opacity=0.4),
        ])
        hold_for(self, N, "absorb")

        caption = swap_caption(self, caption, subtitle_text(N, "long"))
        emitters = [interpolate(hits[0], hits[-1], f) + UP * 0.03 for f in (0.2, 0.5, 0.8)]
        ir_up, ir_back, up_paths, trap_paths = VGroup(), VGroup(), [], []
        for k, src in enumerate(emitters[:2]):
            g_in = np.array([gx + 0.1, room["win_lo"] + 0.3 + 0.9 * k, 0.0])
            bounce = _mirror((g_in - src) / np.linalg.norm(g_in - src), PI / 2)
            back_end = g_in + bounce * min(1.6, (room["y_c"] - 0.12 - g_in[1]) / max(0.05, bounce[1]))
            ir_up.add(_wave([src, g_in], "long", color=P_RED, opacity=0.45))
            ir_back.add(_wave([g_in, back_end], "long", color=P_RED, opacity=0.45))
            up_paths.append([src, g_in])
            trap_paths.append([src, g_in, back_end])
        lw_lbl = Text("langwellig", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(
            np.array([room["center"][0] + 0.6, room["y_c"] + room["slab"] + 0.22, 0.0]))
        # Below the incoming fan: at x = −3.3 the lowest sun ray runs through y ≈ 0.3.
        sw_lbl = Text("kurzwellig", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([-3.6, -0.4, 0.0]))
        self.play(shaft.animate.set_fill(opacity=0.07), LaggedStart(*[Create(b) for b in ir_up], lag_ratio=0.15),
                  FadeIn(lw_lbl), FadeIn(sw_lbl), run_time=1.0)
        _beam_pulses(self, up_paths, P_RED, repeats=2, run_time=2.6, kind="long", extra=[_ripples(emitters, r_max=0.85)])
        hold_for(self, N, "long")

        caption = swap_caption(self, caption, subtitle_text(N, "trap"))
        win_mid = (room["win_lo"] + room["win_hi"]) / 2
        lowe = Line(np.array([gx + 0.1, room["win_lo"], 0.0]), np.array([gx + 0.1, win_mid, 0.0]),
                    color=P_ORANGE, stroke_width=4)
        lowe_lbl = Text("Low-E", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            np.array([gx - 0.55, room["win_lo"] - 0.3, 0.0]))
        plain_lbl = Text("Vollglas", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([gx - 0.2, room["y_c"] + room["slab"] + 0.22, 0.0]))
        plain_leader = Line(plain_lbl.get_bottom() + DOWN * 0.04, np.array([gx, room["win_hi"] - 0.08, 0.0]),
                            color=P_CYAN, stroke_width=1.4, stroke_opacity=0.55)
        lowe_leader = Line(lowe_lbl.get_right() + RIGHT * 0.06, np.array([gx + 0.08, room["win_lo"] + 0.12, 0.0]),
                           color=P_ORANGE, stroke_width=1.4, stroke_opacity=0.55)
        self.play(Create(lowe), FadeIn(lowe_lbl), Create(lowe_leader), FadeIn(plain_lbl), Create(plain_leader),
                  Create(ir_back[0]), run_time=1.0)
        _beam_pulses(self, trap_paths[:1] + up_paths[1:], P_RED, repeats=2, run_time=3.0, kind="long",
                     extra=[_ripples(emitters, r_max=0.85)])
        hold_for(self, N, "trap")

        caption = swap_caption(self, caption, subtitle_text(N, "warm"))
        t_room = ValueTracker(20.0)
        t_read = _readout(lambda: rf"\vartheta_{{\mathrm{{Raum}}}} = {_de(t_room.get_value())}\,\mathrm{{°C}}",
                          np.array([room["x_r"] - 0.15, room["y_c"] - 0.4, 0.0]), size=LABEL_FONT_SIZE,
                          color=P_RED, edge="right")
        _reveal(self, t_read)
        self.play(FadeOut(ir_up), FadeOut(ir_back[0]), FadeOut(lw_lbl), shaft.animate.set_fill(opacity=0.16),
                  run_time=0.8)
        plumes = [smooth_path([np.array([x, room["y_f"] + 0.06, 0.0]), np.array([x + 0.15, room["center"][1], 0.0]),
                               np.array([x - 0.1, room["y_c"] - 0.1, 0.0])])
                  for x in np.linspace(hits[0][0] + 0.4, hits[-1][0] - 0.4, 3)]
        animate_flows(self, [(plumes, P_ORANGE, P_RED)], run_time=2.4, waves=4, cycles=1.5, radius=0.06,
                      extra=[t_room.animate.set_value(35.0), room["air"].animate.set_fill(P_RED, opacity=0.14)])
        hold_for(self, N, "warm")
        #endregion

        #region winter and summer
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        radiator = _radiator(np.array([room["x_l"] + 0.55, room["y_f"] + 0.3, 0.0])).scale(0.75)
        rad_top = radiator.get_top()
        rad_plume = [smooth_path([rad_top + UP * 0.05 + RIGHT * dx, rad_top + UP * 0.6 + RIGHT * (dx + 0.15),
                                  rad_top + UP * 1.2 + RIGHT * (dx + 0.05)]) for dx in (-0.15, 0.15)]
        heat_need = ValueTracker(1000.0)
        need_read = _readout(lambda: rf"\text{{Heizung}}\;{_de(heat_need.get_value())}\,\mathrm{{W}}",
                             np.array([4.9, -0.2, 0.0]), size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        self.play(FadeIn(radiator), FadeIn(need_read), run_time=0.7)
        animate_flows(self, [(rad_plume, P_RED, P_ORANGE)], run_time=1.6, waves=3, cycles=1.2, radius=0.055,
                      extra=[heat_need.animate.set_value(625.0)])
        self.play(Indicate(items["r"], color=P_ORANGE), run_time=0.6)
        hold_for(self, N, "winter")

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        out_blind = _blind(gx - 0.3, room["win_hi"] + 0.06, room["win_lo"] + 0.18)
        rail, cords, slats = out_blind["rail"], out_blind["cords"], out_blind["slats"]
        blind_lbl = Text("Raffstore", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([gx - 0.85, room["win_lo"] - 0.35, 0.0]))
        stopped, shade_paths = VGroup(), []
        for start in starts:
            stop = _blind_hit(out_blind, start, sun_dir)
            stopped.add(_wave([start, stop], "short"))
            bounce = _mirror(sun_dir, out_blind["angle"])
            back = stop + bounce * min(0.4, max(0.05, (room["win_hi"] + 0.04 - stop[1]) / max(bounce[1], 1e-6)))
            stopped.add(_beam(stop, back, opacity=0.4, width=1.6))
            shade_paths.append([start, stop])
        norm2 = Text("DIN 4108-2", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([-3.7, -1.0, 0.0]))
        self.play(FadeOut(radiator), FadeOut(need_read), FadeOut(lowe_lbl), FadeOut(lowe_leader), FadeOut(plain_lbl), FadeOut(plain_leader),
                  run_time=0.4)
        self.play(FadeIn(rail), Create(cords), LaggedStart(*[FadeIn(s, shift=DOWN * 0.25) for s in slats],
                                                            lag_ratio=0.12),
                  FadeIn(blind_lbl), run_time=1.2)
        self.play(ReplacementTransform(beams_out, stopped), shaft.animate.set_fill(opacity=0.03), FadeOut(patch),
                  slats.animate.set_stroke(color=interpolate_color(ManimColor(P_TEAL), ManimColor(P_ORANGE), 0.45)),
                  t_room.animate.set_value(22.0), room["air"].animate.set_fill(P_RED, opacity=0.04),
                  room["floor"].animate.set_fill(P_WHITE, opacity=0.14),
                  room["wall_lo"].animate.set_fill(P_WHITE, opacity=0.14),
                  room["wall_hi"].animate.set_fill(P_WHITE, opacity=0.14), FadeIn(norm2), run_time=1.4)
        _beam_pulses(self, shade_paths, P_YELLOW, repeats=2, run_time=2.6, kind="short")
        hold_for(self, N, "summer")
        #endregion

        #region inside shading and comparison
        caption = swap_caption(self, caption, subtitle_text(N, "inner"))
        bar_base, bar_unit = -0.55, 0.075
        bar_x = {"out": 4.55, "in": 5.55}
        out_bar = Rectangle(width=0.62, height=(22 - 15) * bar_unit, stroke_width=0, fill_color=P_TEAL, fill_opacity=0.7)
        out_bar.move_to(np.array([bar_x["out"], bar_base + out_bar.height / 2, 0.0]))
        in_frac = ValueTracker(0.0)
        in_bar = always_redraw(lambda: Rectangle(
            width=0.62, height=max(0.002, (22 + 8 * in_frac.get_value() - 15) * bar_unit), stroke_width=0,
            fill_color=P_RED, fill_opacity=0.7).move_to(np.array(
                [bar_x["in"], bar_base + max(0.002, (22 + 8 * in_frac.get_value() - 15) * bar_unit) / 2, 0.0])))
        out_name = Text("außen", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([bar_x["out"], bar_base - 0.28, 0.0]))
        in_name = Text("innen", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([bar_x["in"], bar_base - 0.28, 0.0]))
        out_val = _m(r"22\,\mathrm{°C}", np.array([bar_x["out"], bar_base + out_bar.height + 0.3, 0.0]),
                     size=LABEL_FONT_SIZE, color=P_TEAL)
        in_val = always_redraw(lambda: _m(rf"{_de(22 + 8 * in_frac.get_value())}\,\mathrm{{°C}}", np.array(
            [bar_x["in"], bar_base + max(0.002, (22 + 8 * in_frac.get_value() - 15) * bar_unit) + 0.3, 0.0]),
            size=LABEL_FONT_SIZE, color=P_RED))
        in_blind = _blind(gx + 0.3, room["win_hi"] + 0.06, room["win_lo"] + 0.18)
        in_rail, in_cords, in_slats = in_blind["rail"], in_blind["cords"], in_blind["slats"]
        in_lbl = Text("Raffstore innen", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([gx + 1.9, room["y_f"] + 0.3, 0.0]))
        in_beams, in_paths = VGroup(), []
        for start in starts:
            stop = _blind_hit(in_blind, start, sun_dir)
            in_beams.add(_wave([start, stop], "short"))
            in_paths.append([start, stop])
        slat_pts = [np.array([gx + 0.55, y, 0.0]) for y in in_blind["ys"][::3]]
        self.play(FadeOut(VGroup(rail, cords, slats, blind_lbl, norm2)), FadeOut(stopped), run_time=0.6)
        self.play(FadeIn(in_rail), Create(in_cords), LaggedStart(*[FadeIn(s_, shift=DOWN * 0.2) for s_ in in_slats],
                                                                  lag_ratio=0.1), FadeIn(in_lbl),
                  LaggedStart(*[Create(b) for b in in_beams], lag_ratio=0.08), run_time=1.4)
        warm_slats = interpolate_color(ManimColor(P_TEAL), ManimColor(P_RED), 0.7)
        self.play(in_slats.animate.set_stroke(color=warm_slats), shaft.animate.set_fill(opacity=0.0), LaggedStart(
            *[Flash(np.array(pth[1]), color=P_ORANGE, flash_radius=0.12, line_length=0.08) for pth in in_paths],
            lag_ratio=0.1), run_time=1.0)
        _beam_pulses(self, in_paths, P_YELLOW, repeats=1, run_time=2.0, kind="short", extra=[
                         _ripples(slat_pts, r_max=0.6), t_room.animate.set_value(30.0),
                         in_frac.animate.set_value(1.0), room["air"].animate.set_fill(P_RED, opacity=0.1)])
        hold_for(self, N, "inner")

        caption = swap_caption(self, caption, subtitle_text(N, "compare"))
        self.add(in_bar, in_val)
        self.play(FadeIn(out_bar), FadeIn(out_val), FadeIn(out_name), FadeIn(in_name), run_time=0.9)
        better = Text("besser", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([bar_x["out"], bar_base + 1.3, 0.0]))
        self.play(Indicate(out_bar, color=P_TEAL), FadeIn(better, shift=UP * 0.1), run_time=1.0)
        hold_for(self, N, "compare")
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
         "The ceiling has the greatest effect: it is free of cladding, warm air rises to it and cold air sinks.",
         "Die Decke wirkt am stärksten: Sie ist frei von Verkleidung, warme Luft steigt zu ihr auf und kalte Luft fällt."),
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
        fl = np.array([-5.0, -0.75, 0.0])
        W, D, depth = 3.8, 0.42, np.array([1.1, 0.7, 0.0])
        front = Polygon(fl, fl + RIGHT * W, fl + RIGHT * W + UP * D, fl + UP * D, color=P_WHITE, stroke_width=2.5,
                        fill_color=P_WHITE, fill_opacity=0.18)
        top = Polygon(fl + UP * D, fl + RIGHT * W + UP * D, fl + RIGHT * W + UP * D + depth, fl + UP * D + depth,
                      color=P_WHITE, stroke_width=2.5, fill_color=P_WHITE, fill_opacity=0.28)
        side = Polygon(fl + RIGHT * W, fl + RIGHT * W + depth, fl + RIGHT * W + UP * D + depth, fl + RIGHT * W + UP * D,
                       color=P_WHITE, stroke_width=2.5, fill_color=P_WHITE, fill_opacity=0.1)
        slab = VGroup(front, top, side)
        concrete = Text("Beton", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(fl + RIGHT * W / 2 + UP * D / 2)
        d_w = dim_arrow(fl + DOWN * 0.24, fl + RIGHT * W + DOWN * 0.24, color=P_WHITE)
        d_w_lbl = _m(r"100\,\mathrm{cm}", d_w.get_center() + DOWN * 0.36, size=LABEL_FONT_SIZE)
        d_d = dim_arrow(fl + RIGHT * W + RIGHT * 0.25, fl + RIGHT * W + RIGHT * 0.25 + depth, color=P_WHITE)
        d_d_lbl = _m(r"100\,\mathrm{cm}", size=LABEL_FONT_SIZE).next_to(d_d, RIGHT, buff=0.06)
        d_t = dim_arrow(fl + LEFT * 0.22, fl + LEFT * 0.22 + UP * D, color=P_ORANGE)
        d_t_lbl = _m(r"10\,\mathrm{cm}", size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(d_t, LEFT, buff=0.08)
        col_x, rows = 0.7, [1.75, 1.15, 0.55, -0.05, -0.65]
        heat_rows = [1.45, 0.8, 0.15]
        dT = ValueTracker(0.0)
        v_line = _m(r"V = 100\,\mathrm{cm} \cdot 100\,\mathrm{cm} \cdot 10\,\mathrm{cm} = 0{,}1\,\mathrm{m^{3}}",
                    np.array([col_x, rows[0], 0.0]), size=LABEL_FONT_SIZE, edge="left")
        m_line = _m(r"m = \rho \cdot V = 2\,400\,\mathrm{kg/m^{3}} \cdot 0{,}1\,\mathrm{m^{3}} = 240\,\mathrm{kg}",
                    np.array([col_x, rows[1], 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="left")
        dt_read = _readout(lambda: rf"\Delta T = {_de(dT.get_value(), 1)}\,\mathrm{{K}}\quad"
                                   rf"(20 \to {_de(20 + dT.get_value(), 1)}\,\mathrm{{°C}})",
                           np.array([col_x, heat_rows[0], 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        q_read = _readout(
            lambda: (rf"Q = 240\,\mathrm{{kg}} \cdot 1\,000\,\mathrm{{J/(kg\,K)}} \cdot {_de(dT.get_value(), 1)}\,"
                     rf"\mathrm{{K}}"),
            np.array([col_x, heat_rows[1], 0.0]), size=LABEL_FONT_SIZE, color=P_ORANGE)
        kwh_read = _readout(lambda: rf"=\;{_de(240 * dT.get_value())}\,\mathrm{{kJ}} \approx "
                                    rf"{_de(240 * dT.get_value() / 3600, 2)}\,\mathrm{{kWh}}",
                            np.array([col_x, heat_rows[2], 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW)
        heat_in = VGroup(*[Arrow(fl + depth * 0.5 + UP * (D + 1.0) + RIGHT * x, fl + depth * 0.5 + UP * (D + 0.08) + RIGHT * x,
                                 buff=0, color=P_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.25)
                           for x in np.linspace(0.5, W - 0.5, 4)])
        self.play(FadeIn(slab), FadeIn(concrete), run_time=1.0)
        hold_for(self, N, "store", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0)

        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        self.play(GrowFromCenter(d_w), FadeIn(d_w_lbl), GrowFromCenter(d_d), FadeIn(d_d_lbl), GrowFromCenter(d_t),
                  FadeIn(d_t_lbl), run_time=1.0)
        self.play(FadeIn(v_line, shift=RIGHT * 0.1), run_time=0.8)
        self.play(FadeIn(m_line, shift=RIGHT * 0.1), slab.animate.set_fill(P_CYAN, opacity=0.25), run_time=0.8)
        self.wait(2.5)
        self.play(FadeOut(v_line), run_time=1.6)
        self.play(m_line.animate.move_to(np.array([m_line.get_center()[0], rows[0], 0.0])), run_time=1.2)
        hold_for(self, N, "mass")

        caption = swap_caption(self, caption, subtitle_text(N, "heat"))
        row, box, items = _panel([
            (None, "Q", P_ORANGE), (None, "=", P_WHITE), ("m", "m", P_CYAN), (None, r"\cdot", P_WHITE),
            ("c", "c", P_WHITE), (None, r"\cdot", P_WHITE), ("dt", r"\Delta T", P_ORANGE),
        ])
        self.wait(1.5)
        self.play(FadeOut(VGroup(d_w, d_w_lbl, d_d, d_d_lbl, d_t, d_t_lbl)), FadeOut(m_line), run_time=1.8)
        self.play(FadeIn(row), Create(box), LaggedStart(*[GrowArrow(a) for a in heat_in], lag_ratio=0.15), run_time=0.8)
        _reveal(self, dt_read, q_read, kwh_read)
        self.play(dT.animate.set_value(2.0), slab.animate.set_fill(P_ORANGE, opacity=0.45),
                  LaggedStart(*[ShowPassingFlash(a.copy().set_stroke(width=6), time_width=0.4) for a in list(heat_in) * 3],
                              lag_ratio=0.12), run_time=2.4)
        hold_for(self, N, "heat")

        caption = swap_caption(self, caption, subtitle_text(N, "room"))
        self.play(FadeOut(VGroup(heat_in, concrete)), FadeOut(dt_read), FadeOut(q_read), run_time=0.5)
        tile = 0.5
        grid_origin = np.array([-5.4, -0.9, 0.0])
        tiles = VGroup(*[
            Square(side_length=tile * 0.94, color=P_ORANGE, stroke_width=1.6, fill_color=P_ORANGE, fill_opacity=0.45)
            .move_to(grid_origin + RIGHT * tile * (c + 0.5) + UP * tile * (r + 0.5))
            for r in range(4) for c in range(5)
        ])
        g_w = dim_arrow(grid_origin + DOWN * 0.2, grid_origin + RIGHT * 5 * tile + DOWN * 0.2, color=P_WHITE)
        g_w_lbl = _m(r"5\,\mathrm{m}", g_w.get_center() + DOWN * 0.36, size=LABEL_FONT_SIZE)
        g_h = dim_arrow(grid_origin + LEFT * 0.2, grid_origin + UP * 4 * tile + LEFT * 0.2, color=P_WHITE)
        g_h_lbl = _m(r"4\,\mathrm{m}", size=LABEL_FONT_SIZE).next_to(g_h, LEFT, buff=0.08)
        n_tiles = ValueTracker(1)
        total_read = _readout(
            lambda: rf"{int(n_tiles.get_value())}\,\mathrm{{m^{{2}}}} \cdot 0{{,}}133\,\mathrm{{kWh}} = "
                    rf"{_de(0.1333 * n_tiles.get_value(), 1)}\,\mathrm{{kWh}}",
            np.array([col_x, rows[1], 0.0]), color=P_YELLOW)
        self.play(ReplacementTransform(slab, tiles[0]), FadeOut(kwh_read), run_time=1.0)
        _reveal(self, total_read)
        self.play(LaggedStart(*[TransformFromCopy(tiles[0], t) for t in tiles[1:]], lag_ratio=0.08),
                  n_tiles.animate.set_value(20), GrowFromCenter(g_w), FadeIn(g_w_lbl), GrowFromCenter(g_h),
                  FadeIn(g_h_lbl), run_time=2.4)
        vac = _vacuum().scale(0.8)
        vac.move_to(np.array([col_x + vac.width / 2, rows[3], 0.0]))
        vclock = _clock(np.array([col_x + vac.width + 0.75, rows[3], 0.0]), r=0.36)
        hours = ValueTracker(0.0)
        vac_lbl = _readout(lambda: rf"\approx {_de(hours.get_value(), 1)}\,\mathrm{{h}}\;\text{{Staubsaugen}}",
                           np.array([col_x + vac.width + 1.35, rows[3] - 0.08, 0.0]), color=P_TEAL)
        self.play(FadeIn(vac), FadeIn(vclock["group"]), run_time=0.6)
        _reveal(self, vac_lbl)
        self.play(hours.animate.set_value(2.7), Rotate(vclock["hand"], angle=-TAU * 2.7, about_point=vclock["center"]),
                  run_time=1.8, rate_func=linear)
        hold_for(self, N, "room")
        #endregion

        #region day and lag
        caption = swap_caption(self, caption, subtitle_text(N, "day"))
        self.play(FadeOut(VGroup(tiles, vac, vclock["group"], row, box, g_w, g_w_lbl, g_h, g_h_lbl)),
                  FadeOut(total_read), FadeOut(vac_lbl), run_time=0.6)
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
        ]).arrange(RIGHT, buff=0.35)
        legend.move_to(np.array([CO[0] + legend.width / 2, 2.3, 0.0]))
        cursor = always_redraw(lambda: DashedLine(cpt(clock_h.get_value(), 14), cpt(clock_h.get_value(), 34.5),
                                                  color=P_WHITE, stroke_width=1.5, stroke_opacity=0.5))
        small = _room(np.array([4.3, CO[1] + 1.5, 0.0]), w=3.4, h=1.9, slab=0.36, window=(0.3, 0.85))

        def ceiling_heat():
            f = float(np.clip((t_heavy(clock_h.get_value()) - 23) / 4, 0, 1))
            return small["ceiling"].copy().set_fill(interpolate_color(ManimColor(P_WHITE), ManimColor(P_ORANGE), f),
                                                    opacity=0.2 + 0.55 * f)

        def ceiling_flux():
            h = clock_h.get_value()
            rate = np.cos(TAU * (h - 15) / 24)
            y_c = small["y_c"]
            arrows = VGroup()
            for x in np.linspace(small["x_l"] + 0.6, small["x_r"] - 0.6, 3):
                tail, tip = (np.array([x, y_c - 0.65, 0.0]), np.array([x, y_c - 0.08, 0.0]))
                if rate < 0:
                    tail, tip = tip, tail
                arrows.add(Arrow(tail, tip, buff=0, color=P_ORANGE, stroke_width=3,
                                 max_tip_length_to_length_ratio=0.3).set_opacity(0.15 + 0.75 * abs(rate)))
            return arrows

        ceil_live = always_redraw(ceiling_heat)
        flux_live = always_redraw(ceiling_flux)
        sky_c = np.array([small["center"][0], small["y_c"] + small["slab"] + 0.45, 0.0])
        sky_sun = _build_sun(sky_c).scale(0.22)
        sky_moon = _moon(sky_c)

        def sky():
            h = clock_h.get_value() % 24
            return (sky_sun if 6 <= h <= 19 else sky_moon).copy()

        sky_live = always_redraw(sky)
        hour_read = _readout(lambda: rf"{int(clock_h.get_value()) % 24:02d}{{:}}00\,\mathrm{{Uhr}}",
                             sky_c + RIGHT * 0.45 + DOWN * 0.08, size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
        self.play(FadeIn(cax["group"]), FadeIn(h_ticks), FadeIn(legend), FadeIn(small["group"]), run_time=0.9)
        self.add(c_out, c_light, c_heavy, cursor, ceil_live, flux_live, sky_live)
        _reveal(self, hour_read)
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
        amp_out = dim_arrow(cpt(14.5, 16), cpt(14.5, 32), color=P_YELLOW)
        amp_heavy = dim_arrow(cpt(20.5, 23), cpt(20.5, 27), color=P_CYAN)
        amp_lbl = VGroup(_m(r"16\,\mathrm{K}", cpt(14.2, 19.0), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="right"),
                         _m(r"4\,\mathrm{K}", cpt(20.2, 20.5), size=LABEL_FONT_SIZE, color=P_CYAN, edge="right"))
        self.remove(cursor)
        self.play(Create(v1), Create(v2), run_time=0.7)
        _reveal(self, phi_read)
        self.play(GrowFromCenter(shift_arrow), phi.animate.set_value(6.0), run_time=1.4)
        self.play(GrowFromCenter(amp_out), GrowFromCenter(amp_heavy), FadeIn(amp_lbl), run_time=1.0)
        hold_for(self, N, "lag")
        #endregion

        #region ceiling and view
        caption = swap_caption(self, caption, subtitle_text(N, "ceiling"))
        self.play(FadeOut(VGroup(cax["group"], h_ticks, legend, small["group"], v1, v2, shift_arrow, amp_out, amp_heavy,
                                 amp_lbl)),
                  *[FadeOut(m) for m in (c_out, c_light, c_heavy, ceil_live, flux_live, sky_live, hour_read, phi_read)],
                  run_time=0.6)
        big = _room(np.array([-1.3, 0.05, 0.0]), w=5.6, h=2.4, slab=0.34, window=(0.3, 0.85))
        rise_paths = [smooth_path([start, start + np.array([0.18, 0.6, 0.0]), start + np.array([0.05, 1.3, 0.0]),
                                   np.array([start[0] + 0.3, big["y_c"] - 0.08, 0.0]),
                                   np.array([start[0] + 1.1, big["y_c"] - 0.12, 0.0])])
                      for start in (np.array([big["center"][0] - 1.2, big["y_f"] + 0.1, 0.0]),
                                    np.array([big["center"][0] + 0.6, big["y_f"] + 0.1, 0.0]))]
        rise_guides = flow_guides(rise_paths, P_RED, opacity=0.2)
        fall_paths = [smooth_path([np.array([big["glass_x"] + 0.28 + dx, big["y_c"] - 0.15, 0.0]),
                                   np.array([big["glass_x"] + 0.34 + dx, big["center"][1], 0.0]),
                                   np.array([big["glass_x"] + 0.4 + dx, big["y_f"] + 0.25, 0.0]),
                                   np.array([big["glass_x"] + 1.3 + dx, big["y_f"] + 0.14, 0.0])]) for dx in (0.0, 0.14)]
        fall_guides = flow_guides(fall_paths[:1], P_BLUE, opacity=0.2)
        cold_tag = Text("kalte Luft fällt", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        cold_tag.move_to(np.array([big["glass_x"] - 0.3 - cold_tag.width / 2, big["center"][1] - 0.1, 0.0]))
        cold_leader = Line(cold_tag.get_right() + RIGHT * 0.08, np.array([big["glass_x"] + 0.28, big["center"][1] - 0.1, 0.0]),
                           color=P_BLUE, stroke_width=1.4, stroke_opacity=0.55)
        ceil_fill = ValueTracker(0.2)
        big["ceiling"].add_updater(lambda m: m.set_fill(P_ORANGE, opacity=ceil_fill.get_value()))
        tags, tag_leaders = side_labels([
            (big["ceiling"].get_right() + LEFT * 0.1, "frei von Verkleidung", P_ORANGE),
            (rise_paths[0].point_from_proportion(0.5), "warme Luft steigt auf", P_RED),
            (big["wall_r"].get_center(), "Strahlung mit allen Flächen", P_YELLOW),
        ], x=1.95, align="left", min_gap=0.55)
        norm = Text("DIN 4108-2 · DIN EN ISO 13786", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([3.9, -1.35, 0.0]))
        self.play(FadeIn(big["group"]), run_time=1.0)
        self.play(FadeIn(tags[0]), Create(tag_leaders[0]), Indicate(big["ceiling"], color=P_ORANGE, scale_factor=1.0),
                  run_time=1.0)
        self.play(FadeIn(tags[1]), Create(tag_leaders[1]), FadeIn(rise_guides), FadeIn(cold_tag), Create(cold_leader),
                  FadeIn(fall_guides), run_time=0.7)
        animate_flows(self, [(rise_paths, P_RED, P_ORANGE), (fall_paths, P_BLUE, P_CYAN)], run_time=2.4, waves=5,
                      cycles=2.0, radius=0.06, extra=[ceil_fill.animate.set_value(0.45)])
        hold_for(self, N, "ceiling")

        caption = swap_caption(self, caption, subtitle_text(N, "view"))
        ceil_y = big["y_c"] - 0.02
        targets = (np.array([big["glass_x"] + 0.1, big["center"][1] - 0.2, 0.0]),
                   np.array([big["x_l"] + 1.2, big["y_f"] + 0.02, 0.0]),
                   np.array([big["center"][0], big["y_f"] + 0.02, 0.0]),
                   np.array([big["x_r"] - 1.2, big["y_f"] + 0.02, 0.0]),
                   np.array([big["x_r"] - 0.02, big["center"][1] - 0.5, 0.0]))
        exchange = VGroup(*[
            DoubleArrow(np.array([0.6 * t[0] + 0.4 * big["center"][0], ceil_y, 0.0]), t, buff=0.04, color=P_YELLOW,
                        stroke_width=1.8, tip_length=0.12, max_tip_length_to_length_ratio=0.08).set_opacity(0.55)
            for t in targets
        ])
        emit = [np.array([x, ceil_y, 0.0]) for x in np.linspace(big["x_l"] + 1.0, big["x_r"] - 1.0, 3)]
        self.play(FadeIn(tags[2]), Create(tag_leaders[2]), FadeOut(rise_guides), FadeOut(fall_guides), FadeOut(cold_tag),
                  FadeOut(cold_leader),
                  LaggedStart(*[GrowFromCenter(r) for r in exchange], lag_ratio=0.12),
                  ceil_fill.animate.set_value(0.6), FadeIn(norm), run_time=1.8)
        self.play(_ripples(emit, r_max=0.9, color=P_ORANGE, down=True), run_time=2.4)
        hold_for(self, N, "view")
        #endregion

        #region summer and winter
        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        self.play(FadeOut(VGroup(tags, tag_leaders, exchange)), run_time=0.5)
        sky_pos = np.array([-5.6, 1.9, 0.0])
        sun = _build_sun(sky_pos).scale(0.35)
        moon = _moon(sky_pos)
        t_ceil = ValueTracker(24.0)
        t_tag = _readout(lambda: rf"\vartheta_{{\mathrm{{Decke}}}} = {_de(t_ceil.get_value())}\,\mathrm{{°C}}",
                         np.array([big["x_r"] + 0.45, big["y_c"] + big["slab"] / 2 - 0.08, 0.0]),
                         size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        day_flux = VGroup(*[Arrow(np.array([x, big["center"][1] - 0.35, 0.0]), np.array([x, big["y_c"] - 0.06, 0.0]),
                                  buff=0, color=P_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.2)
                            for x in np.linspace(big["x_l"] + 1.0, big["x_r"] - 1.0, 4)])
        self.play(FadeIn(sun), LaggedStart(*[GrowArrow(a) for a in day_flux], lag_ratio=0.12), run_time=0.8)
        _reveal(self, t_tag)
        self.play(LaggedStart(*[ShowPassingFlash(a.copy().set_stroke(width=6), time_width=0.4) for a in list(day_flux) * 3],
                              lag_ratio=0.1),
                  ceil_fill.animate.set_value(0.85), t_ceil.animate.set_value(27.0), run_time=2.0)
        tilt = 28 * DEGREES
        open_glass = big["glass"].copy().rotate(-tilt, about_point=np.array([big["glass_x"], big["win_lo"], 0.0]))
        gx, lo, hi = big["glass_x"], big["win_lo"], big["win_hi"]
        purge = [smooth_path([np.array([gx - 1.4, lo + 0.05 + dy, 0.0]), np.array([gx + 0.3, lo + 0.12 + dy, 0.0]),
                              np.array([big["center"][0], big["y_f"] + 0.3 + dy, 0.0]),
                              np.array([big["x_r"] - 0.35, big["center"][1] - dy, 0.0]),
                              np.array([big["center"][0] + 0.5, big["y_c"] - 0.15 - dy, 0.0]),
                              np.array([gx + 0.25, hi - 0.1 - dy, 0.0]), np.array([gx - 1.2, hi + 0.25 - dy, 0.0])])
                 for dy in (0.0, 0.12)]
        self.play(ReplacementTransform(sun, moon), Transform(big["glass"], open_glass), FadeOut(day_flux), run_time=0.9)
        purge_guide = flow_guides(purge[:1], P_CYAN, opacity=0.18)
        self.play(FadeIn(purge_guide), run_time=0.3)
        animate_flows(self, [(purge, P_BLUE, P_ORANGE)], run_time=3.0, waves=6, cycles=2.0, radius=0.06,
                      extra=[ceil_fill.animate.set_value(0.2), t_ceil.animate.set_value(22.0)])
        hold_for(self, N, "summer")

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        closed_glass = _room(np.array([-1.3, 0.05, 0.0]), w=5.6, h=2.4, slab=0.34, window=(0.3, 0.85))["glass"]
        sun2 = _build_sun(sky_pos).scale(0.35)
        winter_rays = _sun_rays(sky_pos, big["glass_x"], [big["win_hi"] - 0.15 - 0.3 * k for k in range(3)],
                                big["y_f"] + 0.05, gap=0.45)
        lands = winter_rays["lands"]
        patch = Line(lands[0], lands[-1], color=P_RED, stroke_width=6, stroke_opacity=0.4)
        floor_lbl = Text("Boden erwärmt sich auch", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(
            np.array([(lands[0][0] + lands[-1][0]) / 2, big["y_f"] - big["slab"] - 0.3, 0.0]))
        self.play(ReplacementTransform(moon, sun2), Transform(big["glass"], closed_glass), FadeOut(purge_guide),
                  run_time=0.8)
        self.play(_shine(winter_rays), run_time=1.2)
        self.play(FadeIn(patch, scale=0.6), run_time=0.3)
        self.play(_ripples([interpolate(lands[0], lands[-1], f) + UP * 0.03 for f in (0.25, 0.75)], r_max=1.2),
                  ceil_fill.animate.set_value(0.75), t_ceil.animate.set_value(24.0),
                  big["floor"].animate.set_fill(P_ORANGE, opacity=0.5), FadeIn(floor_lbl), run_time=2.0)
        evening = _moon(sky_pos)
        self.play(ReplacementTransform(sun2, evening), FadeOut(winter_rays["group"]), FadeOut(patch), FadeOut(floor_lbl),
                  big["floor"].animate.set_fill(P_WHITE, opacity=0.14), run_time=0.7)
        self.play(_ripples(emit, r_max=1.1, color=P_ORANGE, down=True), ceil_fill.animate.set_value(0.3),
                  t_ceil.animate.set_value(22.0), run_time=2.4)
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
         "Darunter kondensiert Wasser — zum Beispiel an einer kalten Fensterscheibe."),
        ("drops",
         "On the cold surface the vapour turns into liquid droplets — that is condensation. They run down and collect as condensate.",
         "An der kalten Oberfläche wird der Dampf zu flüssigen Tropfen — das ist die Kondensation. Sie laufen ab und sammeln sich als Kondensat."),
        ("person",
         "A resting person gives off about seventy watts sensibly and thirty latently — about 45 grams of water per hour.",
         "Ein ruhender Mensch gibt rund 70 W sensibel und 30 W latent ab — etwa 45 g Wasser pro Stunde."),
        ("rel_winter",
         "In winter, condensation threatens on cold surfaces: the inside surface is colder than the dew point.",
         "Im Winter droht an kalten Flächen Tauwasser (DIN 4108-3): Die Oberfläche innen ist kälter als der Taupunkt."),
        ("rel_summer",
         "In summer, cooling must also dehumidify: the cold coil takes water out of the air.",
         "Im Sommer muss die Kühlung auch entfeuchten: Das kalte Register nimmt der Luft Wasser ab."),
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
        QO = np.array([-5.5, -0.95, 0.0])
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
        hob = _hob(4.6, QO[1], width=1.4)
        pot_w, pot_h = 1.6, 1.0
        pot_c = np.array([4.6, hob["pan_y"] + pot_h / 2, 0.0])
        bl, br = pot_c + np.array([-pot_w / 2, -pot_h / 2, 0.0]), pot_c + np.array([pot_w / 2, -pot_h / 2, 0.0])
        pot = VGroup(
            VMobject(color=P_WHITE, stroke_width=3).set_points_as_corners(
                [bl + UP * pot_h, bl, br, br + UP * pot_h]),
            *[Line(c + UP * (pot_h - 0.12), c + UP * (pot_h - 0.12) + RIGHT * s * 0.2, color=P_WHITE, stroke_width=3)
              for c, s in ((bl, -1), (br, 1))],
        )
        water = Rectangle(width=pot_w - 0.1, height=0.7, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.5)
        water.move_to(pot_c).align_to(bl + UP * 0.03, DOWN)
        col_x = bl[0] - 0.35
        q_read = _readout(lambda: rf"Q = {_de(Q.get_value())}\,\mathrm{{kJ}}", np.array([col_x, pot_c[1] + 0.45, 0.0]),
                          color=P_ORANGE, edge="right")
        t_read = _readout(lambda: rf"T = {_de(temp_of(Q.get_value()))}\,\mathrm{{°C}}",
                          np.array([col_x, pot_c[1] - 0.1, 0.0]), color=P_RED, edge="right")
        mass_tag = _m(r"m = 1\,\mathrm{kg}", np.array([col_x, pot_c[1] - 0.65, 0.0]), color=P_BLUE, edge="right")
        steam_paths = [smooth_path([np.array([pot_c[0] + dx, bl[1] + pot_h + 0.05, 0.0]),
                                    np.array([pot_c[0] + dx + 0.12, bl[1] + pot_h + 0.45, 0.0]),
                                    np.array([pot_c[0] + dx - 0.1, bl[1] + pot_h + 0.85, 0.0]),
                                    np.array([pot_c[0] + dx + 0.08, bl[1] + pot_h + 1.2, 0.0])])
                       for dx in (-0.45, -0.15, 0.15, 0.45)]
        bubble_paths = [Line(bl + np.array([0.3 + 0.33 * k, 0.08, 0.0]), bl + np.array([0.3 + 0.33 * k, 0.6, 0.0]))
                        for k in range(4)]
        self.play(FadeIn(qax["group"]), FadeIn(t_ticks), FadeIn(pot), FadeIn(water), FadeIn(hob["head"]),
                  FadeIn(hob["supports"]), FadeIn(mass_tag), run_time=1.0)
        self.play(FadeIn(hob["flames"], scale=0.3, shift=DOWN * 0.1), run_time=0.5)
        self.add(heat_path, head)
        _reveal(self, q_read, t_read)
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
        _paced_flow(self, [(bubble_paths, P_WHITE, None, None, False), (steam_paths, "#D9DEE3", None, None, False)],
                    run_time=4.0, waves=3, cycles=4.0, radius=0.05,
                    extra=[Q.animate.set_value(2676.0),
                           water.animate.stretch_to_fit_height(0.03, about_edge=DOWN).set_fill(opacity=0.3)])
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
        _reveal(self, ratio_read)
        self.play(ratio.animate.set_value(2257 / 419), run_time=1.4)
        _swap_panel(self, (row, box), lat_panel[:2])
        hold_for(self, N, "ratio")
        #endregion

        #region air and dew point
        caption = swap_caption(self, caption, subtitle_text(N, "air"))
        hob["flames"].clear_updaters()
        self.play(FadeOut(VGroup(qax["group"], t_ticks, pot, water, hob["group"], mass_tag, sens, lat, sens_lbl, lat_lbl,
                                 lat_panel[0], lat_panel[1])),
                  *[FadeOut(m) for m in (heat_path, head, q_read, t_read, ratio_read)], run_time=0.6)
        AO = np.array([-5.4, -1.2, 0.0])
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
        box_c = np.array([3.6, AO[1] + 3.25 / 2, 0.0])
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
        phi_gauge = meter("φ", length=air_box.height, thickness=0.36, color=P_BLUE)
        phi_gauge["group"].shift(box_c + RIGHT * (air_box.width / 2 + 0.55) - phi_gauge["track"].get_center())
        phi_gauge["label"].set_y(air_tag.get_center()[1])
        phi_gauge["fill"].add_updater(lambda m: set_meter(phi_gauge, min(1.0, p_v / _p_sat(T.get_value()))))
        phi_read = _readout(lambda: rf"\varphi = {_de(min(100.0, 100 * p_v / _p_sat(T.get_value())))}\,\%",
                            box_c + DOWN * 1.35, color=P_BLUE, edge="center")
        st_read = _readout(lambda: rf"{_de(T.get_value(), 1)}\,\mathrm{{°C}},\; x = {_de(x_now(), 1)}\,\mathrm{{g/kg}}",
                           apt(-10, 0) + np.array([6.2, 3.25, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW,
                           edge="right")
        v20 = DashedLine(apt(20, 0), apt(20, _x_sat(20)), color=P_WHITE, stroke_width=1.8, stroke_opacity=0.7,
                         dash_length=0.1)
        tag100 = _m(r"100\,\%", apt(20, _x_sat(20)) + RIGHT * 0.2 + DOWN * 0.28, size=LABEL_FONT_SIZE, color=P_CYAN,
                    edge="left")
        tag50 = _m(r"50\,\%", apt(20, x_room) + RIGHT * 0.25 + DOWN * 0.07, size=LABEL_FONT_SIZE, color=P_YELLOW,
                   edge="left")
        self.play(FadeOut(marks), FadeOut(vapour[7:]), FadeIn(phi_gauge["group"]), run_time=0.7)
        _reveal(self, state, phi_read, st_read)
        self.play(Create(v20), FadeIn(tag100), FadeIn(tag50), run_time=1.0)
        hold_for(self, N, "room")

        caption = swap_caption(self, caption, subtitle_text(N, "dew"))
        cool_line = DashedLine(apt(20, x_room), apt(9.3, x_room), color=P_YELLOW, stroke_width=1.8, stroke_opacity=0.7,
                               dash_length=0.1)
        self.play(T.animate.set_value(9.3), Create(cool_line, rate_func=linear), run_time=2.6, rate_func=linear)
        dew_lbl = Text("Taupunkt", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(apt(9.3, x_room), UP + LEFT,
                                                                                      buff=0.3)
        self.play(FadeIn(dew_lbl), Flash(apt(9.3, x_room), color=P_YELLOW, flash_radius=0.14, line_length=0.08),
                  run_time=0.8)
        hold_for(self, N, "dew")

        caption = swap_caption(self, caption, subtitle_text(N, "condense"))
        surf_x = box_c[0] - air_box.width / 2 - 0.1
        surf = Rectangle(width=0.14, height=air_box.height, color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN,
                         fill_opacity=0.25).move_to(np.array([surf_x, box_c[1], 0.0]))
        surf_lbl = Text("kalte Oberfläche", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([surf_x - 0.25 - 0.5 * Text("kalte Oberfläche", font_size=LABEL_FONT_SIZE).width, box_c[1], 0.0]))
        trough = Rectangle(width=1.1, height=0.35, color=P_CYAN, stroke_width=2.5).move_to(
            np.array([surf_x + 0.5, box_c[1] - 1.9, 0.0]))
        trough_lvl = ValueTracker(0.0)
        trough_fill = always_redraw(lambda: Rectangle(
            width=1.04, height=max(0.002, 0.29 * trough_lvl.get_value()), stroke_width=0,
            fill_color=P_CYAN, fill_opacity=0.55).move_to(
                trough.get_bottom() + UP * (0.03 + 0.29 * trough_lvl.get_value() / 2)))
        x_cond = x_room - _x_sat(0.0)
        cond_read = _readout(lambda: rf"\text{{Kondensat}}\;{_de(x_cond * trough_lvl.get_value(), 1)}\,\mathrm{{g}}",
                             trough.get_right() + RIGHT * 0.2 + DOWN * 0.07, color=P_CYAN, edge="left")
        movers = [vapour[1], vapour[3], vapour[5]]
        rest = [vapour[0], vapour[2], vapour[4], vapour[6]]
        spots = [np.array([surf_x + 0.17, box_c[1] + dy, 0.0]) for dy in (0.55, 0.05, -0.4)]
        self.play(FadeOut(phi_read), FadeIn(surf), FadeIn(surf_lbl), run_time=0.6)
        self.play(T.animate.set_value(0.0), *[m.animate.move_to(sp).set_color(P_CYAN) for m, sp in zip(movers, spots)],
                  run_time=2.4, rate_func=linear)
        hold_for(self, N, "condense")

        caption = swap_caption(self, caption, subtitle_text(N, "drops"))
        def droplet(at):
            return VGroup(Circle(radius=0.075, stroke_width=1.2, color=P_WHITE, stroke_opacity=0.6, fill_color=P_CYAN,
                                 fill_opacity=0.85),
                          Dot(radius=0.02, color=P_WHITE).shift(UP * 0.03 + LEFT * 0.03)).move_to(at)

        drops = VGroup(*[droplet(sp) for sp in spots])
        drop_tag = Text("Tropfen", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([surf_x - 0.75, box_c[1] + air_box.height / 2 + 0.3, 0.0]))
        self.play(*[ReplacementTransform(m, d) for m, d in zip(movers, drops)], FadeIn(drop_tag), FadeIn(trough),
                  FadeIn(cond_read), run_time=0.8)
        self.add(trough_fill)
        self.play(LaggedStart(*[d.animate.scale(1.9) for d in drops], lag_ratio=0.3), run_time=2.0)
        bottom_y = box_c[1] - air_box.height / 2 + 0.12
        self.play(LaggedStart(*[d.animate(rate_func=rate_functions.ease_in_quad).move_to(
            np.array([surf_x + 0.17, bottom_y, 0.0])) for d in drops], lag_ratio=0.25), run_time=3.0)
        self.play(LaggedStart(*[d.animate(rate_func=rate_functions.ease_in_quad).move_to(
            np.array([surf_x + 0.3 + 0.18 * k, trough.get_top()[1] + 0.02, 0.0])) for k, d in enumerate(drops)],
                              lag_ratio=0.25), trough_lvl.animate.set_value(1.0), run_time=3.2)
        self.play(*[FadeOut(d) for d in drops], run_time=0.4)
        hold_for(self, N, "drops")
        #endregion

        #region person and relevance
        caption = swap_caption(self, caption, subtitle_text(N, "person"))
        phi_gauge["fill"].clear_updaters()
        self.play(FadeOut(VGroup(aax["group"], t_lbls, x_lbls, sat, sat_lbl, air_box, air_tag, *rest, dew_lbl,
                                 surf, surf_lbl, drop_tag, trough, cond_read, v20, tag100, tag50, cool_line,
                                 phi_gauge["group"])), FadeOut(trough_fill), FadeOut(state), FadeOut(st_read),
                  run_time=0.6)
        f = ValueTracker(0.0)
        bb, bu = -1.0, 0.03
        person = _person(np.array([-4.2, -0.35, 0.0]), scale=2.6)
        person.shift(UP * (bb - person.get_bottom()[1]))
        head = person[0].get_center()
        plume = [smooth_path([head + UP * 0.35, head + UP * 0.8 + RIGHT * 0.12, head + UP * 1.3 + LEFT * 0.05,
                              head + UP * 1.8 + RIGHT * 0.1])]
        mouth = person[0].get_right() + DOWN * 0.04
        exhale = [smooth_path([mouth, mouth + np.array([0.45, 0.08 + dy, 0.0]), mouth + np.array([0.95, 0.3 + dy, 0.0]),
                               mouth + np.array([1.4, 0.6 + dy, 0.0])]) for dy in (-0.12, 0.08)]
        sens_tag = Text("sensibel: Strahlung, Konvektion", font_size=LABEL_FONT_SIZE, color=P_RED)
        sens_tag.next_to(plume[0].get_end(), LEFT, buff=0.2).align_to(np.array([-6.2, 0.0, 0.0]), LEFT)
        lat_tag = Text("latent: Wasserdampf", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        lat_tag.move_to(exhale[1].get_end() + UP * 0.32)

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
        self.play(FadeIn(person), Create(base), FadeIn(sens_tag), FadeIn(lat_tag), run_time=0.9)
        self.add(bars)
        _reveal(self, s_read, l_read, g_read)
        _paced_flow(self, [(plume, P_RED, None, _gradient([(0.0, P_RED), (1.0, P_ORANGE)]), False, 4),
                           (exhale, P_BLUE, None, None, False, 3)], run_time=2.4, waves=4, cycles=1.6, radius=0.05,
                    extra=[f.animate.set_value(1.0), _ripples([person.get_center() + UP * 0.3], r_max=0.7, color=P_RED)])
        hold_for(self, N, "person")

        caption = swap_caption(self, caption, subtitle_text(N, "rel_winter"))
        self.play(FadeOut(VGroup(person, sens_tag, lat_tag, base)),
                  *[FadeOut(m) for m in (bars, s_read, l_read, g_read)], run_time=0.5)
        head_y, foot_y = 2.3, -1.55
        wx, wt, w_lo, w_hi, w_bot, w_top = -4.2, 0.4, -0.45, 0.95, -1.2, 1.45
        wall_style = dict(color=P_WHITE, stroke_width=2, fill_color=P_WHITE, fill_opacity=0.16)
        wall_parts = VGroup(
            Rectangle(width=wt, height=w_lo - w_bot, **wall_style).move_to(np.array([wx, (w_bot + w_lo) / 2, 0.0])),
            Rectangle(width=wt, height=w_top - w_hi, **wall_style).move_to(np.array([wx, (w_hi + w_top) / 2, 0.0])),
        )
        pane = Line(np.array([wx, w_lo, 0.0]), np.array([wx, w_hi, 0.0]), color=P_CYAN, stroke_width=4)
        sill = Line(np.array([wx + 0.02, w_lo, 0.0]), np.array([wx + 0.38, w_lo, 0.0]), color=P_WHITE, stroke_width=3)
        room_lines = VGroup(Line(np.array([wx + wt / 2, w_bot, 0.0]), np.array([-1.6, w_bot, 0.0]), color=P_WHITE,
                                 stroke_width=2.5),
                            Line(np.array([wx + wt / 2, w_top, 0.0]), np.array([-1.6, w_top, 0.0]), color=P_WHITE,
                                 stroke_width=2.5))
        w_head = Text("Winter: Tauwasser an kalten Flächen", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([-3.1, head_y, 0.0]))
        t_e = _m(r"\vartheta_{\mathrm{e}} = \text{−5}\,\mathrm{°C}", np.array([wx - 0.3, 0.3, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_BLUE, edge="right")
        t_i = _m(r"\vartheta_{\mathrm{i}} = 20\,\mathrm{°C},\; \varphi = 50\,\%", np.array([wx + 0.6, 1.1, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        w_foot = _m(r"\vartheta_{\mathrm{si}} \approx 1\,\mathrm{°C} < \vartheta_{\mathrm{Tau}} \approx 9\,\mathrm{°C}",
                    np.array([-3.6, foot_y, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN)
        rng_w = np.random.default_rng(21)
        stick_ys = np.sort(rng_w.uniform(w_lo + 0.2, w_hi - 0.1, 7))
        vapour_in = [smooth_path([np.array([rng_w.uniform(-2.5, -1.8), rng_w.uniform(-0.8, 0.8), 0.0]),
                                  np.array([rng_w.uniform(-3.6, -3.0), y + rng_w.uniform(-0.2, 0.2), 0.0]),
                                  np.array([wx + 0.06, y, 0.0])]) for y in stick_ys]
        drops = VGroup(*[Ellipse(width=0.07, height=0.1, stroke_width=0, fill_color=P_CYAN, fill_opacity=0.9)
                         .move_to(np.array([wx + 0.06, y, 0.0])) for y in stick_ys])

        dx0, dx1, d_lo, d_hi, cx = 0.9, 6.3, -0.6, 1.15, 3.6
        duct = VGroup(Line(np.array([dx0, d_hi, 0.0]), np.array([dx1, d_hi, 0.0]), color=P_WHITE, stroke_width=3),
                      Line(np.array([dx0, d_lo, 0.0]), np.array([dx1, d_lo, 0.0]), color=P_WHITE, stroke_width=3))
        c_lo, c_hi = 0.0, d_hi - 0.08
        fins = VGroup(*[Line(np.array([cx + dx, c_lo, 0.0]), np.array([cx + dx, c_hi, 0.0]), color=P_BLUE, stroke_width=1.6)
                        for dx in np.linspace(-0.25, 0.25, 7)])
        coil = VGroup(Rectangle(width=0.7, height=c_hi - c_lo, color=P_BLUE, stroke_width=2).move_to(
            np.array([cx, (c_lo + c_hi) / 2, 0.0])), fins)
        tray = VMobject(color=P_WHITE, stroke_width=2.5).set_points_as_corners(
            [np.array([cx - 0.45, d_lo + 0.25, 0.0]), np.array([cx - 0.45, d_lo + 0.05, 0.0]),
             np.array([cx + 0.45, d_lo + 0.05, 0.0]), np.array([cx + 0.45, d_lo + 0.25, 0.0])])
        drain = Line(np.array([cx + 0.3, d_lo + 0.05, 0.0]), np.array([cx + 0.3, d_lo - 0.4, 0.0]), color=P_WHITE,
                     stroke_width=2.5)
        coil_tag = Text("Kühlregister", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        coil_tag.move_to(np.array([cx - 0.6 - coil_tag.width / 2, d_lo - 0.32, 0.0]))
        cond_tag = Text("Kondensat", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        cond_tag.move_to(np.array([cx + 0.5 + cond_tag.width / 2, d_lo - 0.55, 0.0]))
        s_head = Text("Sommer: Kühlung entfeuchtet", font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(
            np.array([(dx0 + dx1) / 2, head_y, 0.0]))
        a_in = VGroup(
            _m(r"26\,\mathrm{°C} \cdot \varphi = 56\,\%", np.array([dx0 + 0.1, d_hi + 0.7, 0.0]),
               size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left"),
            _m(r"x = 11{,}7\,\mathrm{g/kg}", np.array([dx0 + 0.1, d_hi + 0.3, 0.0]),
               size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left"))
        a_out = VGroup(
            _m(r"14\,\mathrm{°C} \cdot \varphi = 95\,\%", np.array([dx1 - 0.1, d_hi + 0.7, 0.0]),
               size=LABEL_FONT_SIZE, color=P_BLUE, edge="right"),
            _m(r"x = 9{,}5\,\mathrm{g/kg}", np.array([dx1 - 0.1, d_hi + 0.3, 0.0]),
               size=LABEL_FONT_SIZE, color=P_BLUE, edge="right"))
        s_foot = _m(r"\vartheta_{\mathrm{Reg}} \approx 8\,\mathrm{°C} < \vartheta_{\mathrm{Tau}} \approx 16\,\mathrm{°C}",
                    np.array([(dx0 + dx1) / 2, foot_y, 0.0]), size=LABEL_FONT_SIZE, color=P_BLUE)
        air = [Line(np.array([dx0 - 0.3, y, 0.0]), np.array([dx1 + 0.3, y, 0.0])) for y in (0.2, 0.55, 0.9)]
        moist = [Line(np.array([dx0 - 0.3, y, 0.0]), np.array([cx - 0.36, y, 0.0])) for y in (0.08, 0.38, 0.72, 1.0)]
        drips = [Line(np.array([cx + dx, c_lo - 0.02, 0.0]), np.array([cx + dx, d_lo + 0.1, 0.0])) for dx in (-0.2, 0.05, 0.25)]
        drain_drop = [Line(np.array([cx + 0.3, d_lo - 0.4, 0.0]), np.array([cx + 0.3, d_lo - 0.75, 0.0]))]
        cool = _gradient([(0.0, P_ORANGE), (0.47, P_ORANGE), (0.53, P_BLUE), (1.0, P_BLUE)])

        self.play(FadeIn(w_head), FadeIn(wall_parts), Create(room_lines), Create(pane), Create(sill), run_time=1.6)
        self.play(FadeIn(t_e), FadeIn(t_i), run_time=1.0)
        _paced_flow(self, [(vapour_in, P_BLUE, None, None, False)], run_time=4.5, waves=1, cycles=1.0, radius=0.05,
                    extra=[LaggedStart(*[FadeIn(d, scale=0.3) for d in drops], lag_ratio=0.4)])
        self.play(LaggedStart(*[d.animate(rate_func=rate_functions.ease_in_quad).move_to(
            np.array([wx + 0.06 + 0.04 * k, w_lo + 0.05, 0.0])) for k, d in enumerate(drops)], lag_ratio=0.3),
                  run_time=3.0)
        self.play(FadeIn(w_foot), run_time=1.0)
        hold_for(self, N, "rel_winter")

        caption = swap_caption(self, caption, subtitle_text(N, "rel_summer"))
        self.play(FadeIn(s_head), run_time=1.2)
        self.play(Create(duct), run_time=1.6)
        self.play(FadeIn(coil), FadeIn(coil_tag), run_time=1.4)
        self.play(Create(tray), Create(drain), run_time=1.2)
        self.play(FadeIn(a_in), run_time=1.2)
        self.play(FadeIn(a_out), run_time=1.2)
        _paced_flow(self, [(air, P_ORANGE, None, cool, False), (moist, P_BLUE, None, None, False)],
                    run_time=6.0, waves=4, cycles=2.0, radius=0.045)
        self.play(FadeIn(cond_tag), FadeIn(s_foot), run_time=1.0)
        _paced_flow(self, [(air, P_ORANGE, None, cool, False), (moist, P_BLUE, None, None, False),
                           (drips, P_CYAN, None, None, False), (drain_drop, P_CYAN, None, None, False)],
                    run_time=6.0, waves=4, cycles=2.0, radius=0.045)
        hold_for(self, N, "rel_summer")
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – Summary: seven building blocks of the video
class Beat8_Zusammenfassung(Scene):
    NARRATION = [
        ("energy",
         "Energy is what we use: one kilowatt for one hour is one kilowatt-hour.",
         "Energie ist, was wir verbrauchen: 1 kW über 1 Stunde sind 1 Kilowattstunde."),
        ("power",
         "Power says how fast the energy flows — energy per time.",
         "Leistung sagt, wie schnell die Energie fließt — Energie pro Zeit."),
        ("keep",
         "Energy is never lost: it leaves as heat or it is stored.",
         "Energie geht nie verloren: Sie kommt als Wärme heraus oder wird gespeichert."),
        ("pump",
         "The heat pump transfers energy to a higher temperature level — the COP counts the heat per kilowatt-hour of electricity.",
         "Die Wärmepumpe überträgt Energie auf ein höheres Temperaturniveau — der COP nennt die Wärme je kWh Strom."),
        ("sun",
         "Radiation comes through the glass short-wave and stays trapped long-wave — the g-value sets the gain.",
         "Strahlung kommt kurzwellig durchs Glas und bleibt langwellig gefangen — der g-Wert bestimmt den Gewinn."),
        ("mass",
         "Thermal mass stores heat and gives it back later.",
         "Thermische Masse speichert Wärme und gibt sie zeitversetzt wieder ab."),
        ("latent",
         "Heat sits in the temperature and in the water vapour — evaporating takes over five times as much as warming.",
         "Wärme steckt in der Temperatur und im Wasserdampf — Verdampfen braucht über fünfmal so viel wie Erwärmen."),
        ("next",
         "With these seven building blocks we now calculate heating and cooling.",
         "Mit diesen sieben Bausteinen rechnen wir nun Heizung und Kühlung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Zusammenfassung — was wir gelernt haben", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "energy"))
        self.play(FadeIn(caption), run_time=0.3)

        #region rows
        specs = [
            ("energy", "Energie", P_YELLOW, r"1\,\mathrm{kW} \cdot 1\,\mathrm{h} = 1\,\mathrm{kWh}"),
            ("power", "Leistung", P_CYAN, r"P = E \,/\, t"),
            ("keep", "Energieerhaltung", P_RED, r"E_{\mathrm{ein}} = E_{\mathrm{aus}} + \Delta E_{\mathrm{Speicher}}"),
            ("pump", "Wärmepumpe", P_ORANGE, r"\mathrm{COP} = Q_{\mathrm{H}} \,/\, W_{\mathrm{el}}"),
            ("sun", "Strahlung", P_YELLOW, r"\dot{Q}_{\mathrm{S}} = g \cdot A \cdot I"),
            ("mass", "Thermische Masse", P_ORANGE, r"Q = m \cdot c \cdot \Delta T"),
            ("latent", "Sensibel und latent", P_BLUE, r"Q_{\mathrm{lat}} = m \cdot r"),
        ]
        top, pitch, x_bullet, x_topic, x_formula = 1.65, 0.62, -3.8, -3.5, 0.8
        rows = {}
        for k, (key, topic, color, formula) in enumerate(specs):
            y = top - k * pitch
            bullet = Square(side_length=0.2, stroke_width=0, fill_color=color, fill_opacity=0.85).move_to(
                np.array([x_bullet, y, 0.0]))
            label = Text(topic, font_size=LABEL_FONT_SIZE, color=color)
            label.move_to(np.array([x_topic + label.width / 2, y, 0.0]))
            math = _m(formula, np.array([x_formula, y - 0.08, 0.0]), size=BODY_FONT_SIZE, color=P_WHITE, edge="left")
            rows[key] = VGroup(bullet, label, math)
        #endregion

        for k, (key, *_rest) in enumerate(specs):
            if k:
                caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(FadeIn(rows[key], shift=RIGHT * 0.2), run_time=0.8)
            self.play(Indicate(rows[key][2], color=specs[k][2], scale_factor=1.08), run_time=0.7)
            hold_for(self, N, key, used=0.0 if k else BEAT_SUBTITLE_FADE + 0.3 + 1.5)

        caption = swap_caption(self, caption, subtitle_text(N, "next"))
        frame = SurroundingRectangle(VGroup(*rows.values()), color=P_TEAL, buff=0.3, stroke_width=2.5,
                                     corner_radius=0.15)
        self.play(Create(frame), run_time=1.0)
        hold_for(self, N, "next")

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
    Beat8_Zusammenfassung,
]
#endregion
