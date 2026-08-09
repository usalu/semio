"""🎨 Shared Manim helpers for Cooling/Heating tutorials.

Palette, pedagogy visuals, the dedicated formula panel, and narration timing —
the single source every scene imports from instead of re-declaring its own.
Radiation = waves, convection = air streams, watt values get everyday anchors,
formula symbols appear by morphing physical objects (never as orphan text),
and every formula lives in the one boxed panel with its parameters highlighted
in the order the narration explains them.
"""

from __future__ import annotations

import numpy as np

#region Palette
# The single source of truth — scenes import these, never copy-paste the block.
P_DEEP_DARK = "#0B0C10"
P_WHITE = "#E0E6ED"
P_CYAN = "#66FCF1"
P_TEAL = "#45A29E"
P_ORANGE = "#FFAAA5"
P_YELLOW = "#FFE66D"
P_RED = "#FF6B6B"
P_BLUE = "#4D96FF"
P_GREEN = "#CAFFBF"
#endregion


#region Layout zones
# Three fixed horizontal bands every beat respects, top to bottom: the title
# block, free content, then formula_panel and caption_bar. Beats run their
# scaffold through fit_band() so a label can never drift into a reserved zone —
# which is what stops the "text printed on the room outline" class of bug that
# hand-tuned per-beat coordinates kept reintroducing.
SAFE_TOP = 2.58
# A two-line caption_bar (center-aligned, fixed CAPTION_LINE_BUFF) reaches up to
# y ≈ -2.66, so the floor clears the taller of the two cases rather than
# whichever caption happens to be on screen.
SAFE_BOTTOM = -2.60
SAFE_BOTTOM_FORMULA = -1.66
SAFE_LEFT = -6.80
SAFE_RIGHT = 6.80


def fit_band(group, *, top: float = SAFE_TOP, bottom: float = SAFE_BOTTOM):
    """📏 Pull a scaffold back inside the free band — a no-op when it already fits."""
    from manim import DOWN, UP

    band = top - bottom
    if group.height > band:
        group.scale(band / group.height)
    if group.width > (SAFE_RIGHT - SAFE_LEFT):
        group.scale((SAFE_RIGHT - SAFE_LEFT) / group.width)
    overshoot = group.get_top()[1] - top
    if overshoot > 0:
        group.shift(DOWN * overshoot)
    undershoot = bottom - group.get_bottom()[1]
    if undershoot > 0:
        group.shift(UP * undershoot)
    return group
#endregion


#region Radiation waves
def radiation_wave(
    origin,
    *,
    x_offset: float = 0.0,
    height: float = 0.85,
    color: str = P_ORANGE,
    stroke_width: float = 2.0,
    opacity: float = 0.85,
    cycles: float = 2.2,
):
    """〰️ Sine-shaped radiation plume rising (or falling if ``height`` < 0) from ``origin``."""
    from manim import VMobject

    n = 28
    pts = []
    for i in range(n):
        t = i / (n - 1)
        y = t * height
        x = x_offset + 0.055 * np.sin(t * (2 * np.pi) * cycles)
        pts.append(np.array(origin, dtype=float) + np.array([x, y, 0.0]))
    wave = VMobject(color=color, stroke_width=stroke_width, stroke_opacity=opacity)
    wave.set_points_smoothly(pts)
    return wave


def radiation_waves(
    origin,
    *,
    n: int = 4,
    color: str = P_ORANGE,
    height: float = 0.95,
    x_spread: float = 0.28,
    stroke_width: float = 2.0,
):
    """〰️ Bundle of radiation plumes for Strahlung."""
    from manim import VGroup

    if n <= 1:
        offsets = [0.0]
    else:
        offsets = np.linspace(-x_spread / 2, x_spread / 2, n)
    return VGroup(*[
        radiation_wave(origin, x_offset=float(x), height=height, color=color, stroke_width=stroke_width)
        for x in offsets
    ])


def solar_wave_ray(start, end, *, color: str = P_YELLOW, stroke_width: float = 2.4, amp: float = 0.07, cycles: float = 3.0):
    """☀️ Wavy solar radiation segment from sun toward a surface (not a straight line)."""
    from manim import VMobject

    start = np.array(start, dtype=float)
    end = np.array(end, dtype=float)
    direction = end - start
    length = float(np.linalg.norm(direction[:2]))
    if length < 1e-6:
        length = 1.0
    unit = direction / length
    normal = np.array([-unit[1], unit[0], 0.0])
    n = 36
    pts = []
    for i in range(n):
        t = i / (n - 1)
        pts.append(start + unit * (t * length) + normal * (amp * np.sin(t * (2 * np.pi) * cycles)))
    ray = VMobject(color=color, stroke_width=stroke_width, stroke_opacity=0.9)
    ray.set_points_smoothly(pts)
    return ray
#endregion


#region Convection air stream
def convection_stream(
    start,
    end,
    *,
    color: str = P_CYAN,
    stroke_width: float = 2.4,
    bend: float = 0.35,
    n_ribbons: int = 3,
    spread: float = 0.18,
):
    """🌬️ Curved air-stream ribbons for Konvektion / Zuluft (not particle dots alone)."""
    from manim import VGroup, VMobject

    start = np.array(start, dtype=float)
    end = np.array(end, dtype=float)
    mid = (start + end) / 2 + np.array([0.0, bend, 0.0])
    direction = end - start
    length = float(np.linalg.norm(direction[:2])) or 1.0
    normal = np.array([-direction[1], direction[0], 0.0]) / length

    ribbons = VGroup()
    if n_ribbons <= 1:
        offsets = [0.0]
    else:
        offsets = np.linspace(-spread / 2, spread / 2, n_ribbons)
    for off in offsets:
        shift = normal * float(off)
        pts = [start + shift, mid + shift, end + shift]
        ribbon = VMobject(color=color, stroke_width=stroke_width, stroke_opacity=0.85)
        ribbon.set_points_as_corners(pts)
        ribbon.make_smooth()
        ribbons.add(ribbon)
    return ribbons
#endregion


#region Respiration split
def respiration_parts(mouth_pos, *, scale: float = 1.0):
    """🫁 Atmung as sensible breath plume + latent moisture droplets."""
    from manim import Dot, DOWN, RIGHT, UP, VGroup
    from manim_fonts import LABEL_FONT_SIZE, body_text

    mouth = np.array(mouth_pos, dtype=float)
    sens_waves = radiation_waves(
        mouth + np.array([0.15 * scale, 0.05 * scale, 0.0]),
        n=3,
        color=P_RED,
        height=0.55 * scale,
        x_spread=0.2 * scale,
        stroke_width=1.8,
    )
    sens_label = body_text("fühlbar", font_size=LABEL_FONT_SIZE, color=P_RED)
    sens_label.next_to(sens_waves, UP, buff=0.08)

    droplets = VGroup(*[
        Dot(
            mouth + np.array([0.35 * scale + 0.12 * i, 0.12 * scale - 0.08 * (i % 2), 0.0]),
            radius=0.045 * scale,
            color=P_BLUE,
            fill_opacity=0.9,
        )
        for i in range(4)
    ])
    lat_label = body_text("latent", font_size=LABEL_FONT_SIZE, color=P_BLUE)
    lat_label.next_to(droplets, RIGHT, buff=0.1)

    return {
        "sensible": sens_waves,
        "sensible_label": sens_label,
        "latent": droplets,
        "latent_label": lat_label,
        "group": VGroup(sens_waves, sens_label, droplets, lat_label),
    }
#endregion


#region Watt anchors
_WATT_DEVICES: dict[str, tuple[str, int, str]] = {
    "laptop": ("Laptop", 60, P_CYAN),
    "bulb": ("Glühbirne", 100, P_YELLOW),
    "toaster": ("Toaster", 1000, P_ORANGE),
    "vacuum": ("Staubsauger", 1400, P_TEAL),
    "heater": ("Heizgerät", 2000, P_RED),
}


def _device_glyph(kind: str, color: str):
    """🔌 Tiny line-art stand-in for an everyday watt reference."""
    from manim import DOWN, LEFT, Line, RIGHT, Circle, Rectangle, RoundedRectangle, UP, VGroup

    if kind == "laptop":
        base = Rectangle(width=0.55, height=0.08, color=color, stroke_width=2)
        screen = Rectangle(width=0.5, height=0.32, color=color, stroke_width=2)
        screen.next_to(base, UP, buff=0)
        return VGroup(base, screen)
    if kind == "bulb":
        glass = Circle(radius=0.16, color=color, stroke_width=2)
        base = Rectangle(width=0.12, height=0.1, color=color, stroke_width=2)
        base.next_to(glass, DOWN, buff=0)
        return VGroup(glass, base)
    if kind == "toaster":
        body = RoundedRectangle(width=0.55, height=0.38, corner_radius=0.06, color=color, stroke_width=2)
        slot1 = Line(UP * 0.08 + LEFT * 0.12, UP * 0.08 + RIGHT * 0.12, color=color, stroke_width=2).move_to(body.get_center() + UP * 0.06 + LEFT * 0.1)
        slot2 = slot1.copy().shift(RIGHT * 0.2)
        return VGroup(body, slot1, slot2)
    if kind == "vacuum":
        body = RoundedRectangle(width=0.42, height=0.32, corner_radius=0.08, color=color, stroke_width=2)
        hose = Line(body.get_right(), body.get_right() + RIGHT * 0.35 + UP * 0.15, color=color, stroke_width=2.5)
        return VGroup(body, hose)
    body = RoundedRectangle(width=0.45, height=0.5, corner_radius=0.06, color=color, stroke_width=2)
    fins = VGroup(*[
        Line(LEFT * 0.12 + UP * (0.12 - 0.1 * i), RIGHT * 0.12 + UP * (0.12 - 0.1 * i), color=color, stroke_width=1.5)
        for i in range(3)
    ]).move_to(body.get_center())
    return VGroup(body, fins)


def watt_anchor(watts: float, *, compare: str = "laptop", title: str | None = None):
    """🔌 Badge that grounds a power number against a familiar device."""
    from manim import DOWN, RoundedRectangle, VGroup
    from manim_fonts import FORMULA_FONT_SIZE, LABEL_FONT_SIZE, body_text

    name, device_w, color = _WATT_DEVICES.get(compare, _WATT_DEVICES["laptop"])
    ratio = watts / device_w if device_w else 0.0
    if ratio >= 0.85 and ratio <= 1.15:
        compare_line = f"≈ 1× {name} ({device_w} W)"
    elif ratio < 1:
        compare_line = f"≈ {ratio:.1f}× {name} ({device_w} W)"
    else:
        compare_line = f"≈ {ratio:.0f}× {name} ({device_w} W)"

    glyph = _device_glyph(compare if compare in _WATT_DEVICES else "laptop", color)
    value = body_text(
        f"{int(watts) if watts == int(watts) else watts} W",
        font_size=FORMULA_FONT_SIZE,
        color=color,
    )
    hint = body_text(compare_line, font_size=LABEL_FONT_SIZE, color=P_WHITE)
    head = body_text(title, font_size=LABEL_FONT_SIZE, color=P_TEAL) if title else None
    body_parts = [p for p in (head, value, glyph, hint) if p is not None]
    body = VGroup(*body_parts).arrange(DOWN, buff=0.12)
    frame = RoundedRectangle(
        width=max(2.8, body.width + 0.45),
        height=body.height + 0.35,
        corner_radius=0.12,
        color=color,
        stroke_width=2,
        fill_color="#0B0C10",
        fill_opacity=0.92,
    )
    body.move_to(frame.get_center())
    return VGroup(frame, body)
#endregion


#region Particle flow
# A drawn streamline says "air could move here"; only moving particles say
# "air is moving now". Anything showing an exchange of air uses these.
def smooth_path(points):
    """〰️ Smooth polyline through ``points`` — the track particles travel along."""
    from manim import TipableVMobject

    path = TipableVMobject()
    path.set_points_smoothly([np.array(p, dtype=float) for p in points])
    return path


def flow_guides(paths, color, *, opacity: float = 0.25, width: float = 2.0, tips: bool = True):
    """🧭 Faint tipped streamlines so the route stays readable between particles."""
    from manim import VGroup

    lines = VGroup()
    for path in paths:
        guide = path.copy().set_stroke(color=color, width=width, opacity=opacity)
        guide.set_fill(opacity=0)
        if tips:
            guide.add_tip(tip_length=0.17, tip_width=0.14)
            guide.tip.set_fill(color=color, opacity=opacity + 0.3)
            guide.tip.set_stroke(width=0)
        lines.add(guide)
    return lines


def animate_flow(
    scene, paths, color, *, run_time: float = 3.2, waves: int = 3, radius: float = 0.075,
    cycles: float = 2.0, color_end=None, extra=None, streak: bool = True,
):
    """💨 Continuous particle stream along ``paths`` — particles fade in and out at the ends.

    ``waves`` particles per path are spread evenly along it and looped ``cycles``
    times, so the stream reads as continuous rather than as one object crossing.
    With ``streak=True`` each particle is a short ellipse aimed along the path —
    that reads as moving air, not as bouncing dots.
    """
    animate_flows(
        scene,
        [(paths, color, color_end)],
        run_time=run_time,
        waves=waves,
        radius=radius,
        cycles=cycles,
        extra=extra,
        streak=streak,
    )


def animate_flows(
    scene,
    streams,
    *,
    run_time: float = 3.2,
    waves: int = 5,
    radius: float = 0.065,
    cycles: float = 2.4,
    extra=None,
    streak: bool = True,
):
    """💨 Several coloured streams at once — e.g. warm exhaust + cold intake exchanging.

    ``streams`` is an iterable of ``(paths, color)`` or ``(paths, color, color_end)``.
    Playing them in one ``scene.play`` is what makes a buoyancy loop look like a
    real exchange instead of two sequential crossings.
    """
    from manim import (
        Dot, Ellipse, ManimColor, UpdateFromAlphaFunc, VGroup,
        interpolate_color, linear,
    )

    dots = VGroup()
    meta = []
    for entry in streams:
        paths, color = entry[0], entry[1]
        color_end = entry[2] if len(entry) > 2 else None
        start_c = ManimColor(color)
        end_c = ManimColor(color_end) if color_end else start_c
        for path in paths:
            for w in range(waves):
                if streak:
                    particle = Ellipse(
                        width=radius * 2.8,
                        height=radius * 1.15,
                        color=color,
                        stroke_width=0,
                        fill_opacity=0.0,
                    )
                else:
                    particle = Dot(radius=radius, color=color, stroke_width=0)
                    particle.set_fill(color, opacity=0.0)
                particle.move_to(path.point_from_proportion(0.0))
                dots.add(particle)
                meta.append((path, w / waves, start_c, end_c, streak))

    def update(group, alpha):
        for particle, (path, offset, start_c, end_c, is_streak) in zip(group, meta):
            t = (alpha * cycles + offset) % 1.0
            pos = path.point_from_proportion(t)
            particle.move_to(pos)
            # Soft head/tail fade so the stream never pops on or off the frame.
            fade = min(1.0, t / 0.08, (1.0 - t) / 0.10)
            fill = interpolate_color(start_c, end_c, t)
            if is_streak:
                eps = 0.02
                t0 = max(0.0, t - eps)
                t1 = min(1.0, t + eps)
                tangent = path.point_from_proportion(t1) - path.point_from_proportion(t0)
                angle = float(np.arctan2(tangent[1], tangent[0]))
                particle.set_fill(fill, opacity=max(0.0, fade * 0.92))
                particle.set_angle(angle)
            else:
                particle.set_fill(fill, opacity=max(0.0, fade))

    scene.add(dots)
    anims = [UpdateFromAlphaFunc(dots, update)]
    if extra:
        anims.extend(extra)
    scene.play(*anims, run_time=run_time, rate_func=linear)
    scene.remove(dots, *dots)


def animate_haze(
    scene,
    *,
    x0: float,
    x1: float,
    y0: float,
    y1: float,
    run_time: float = 3.0,
    n: int = 48,
    color: str = P_ORANGE,
    color_end: str = "#E8D5C4",
    cycles: float = 1.6,
    extra=None,
    seed: int = 7,
):
    """🌫️ Free-floating heat/air haze — soft puffs with buoyancy wobble, no path rails.

    Use this when particles-on-streamlines would read as beads on a wire. Each puff
    drifts ``x0→x1`` with a gentle sine lift and fades at both ends.
    """
    from manim import Dot, ManimColor, TAU, UpdateFromAlphaFunc, VGroup, interpolate_color, linear

    rng = np.random.default_rng(seed)
    puffs = VGroup()
    meta = []
    start_c = ManimColor(color)
    end_c = ManimColor(color_end)
    for _ in range(n):
        radius = float(rng.uniform(0.055, 0.13))
        puff = Dot(radius=radius, color=color, stroke_width=0)
        puff.set_fill(color, opacity=0.0)
        puffs.add(puff)
        meta.append({
            "phase": float(rng.uniform(0.0, 1.0)),
            "y_base": float(rng.uniform(y0, y1)),
            "amp": float(rng.uniform(0.06, 0.22)),
            "speed": float(rng.uniform(0.75, 1.25)),
            "wobble": float(rng.uniform(1.2, 2.4)),
        })

    def update(group, alpha):
        for puff, m in zip(group, meta):
            t = (alpha * cycles * m["speed"] + m["phase"]) % 1.0
            x = x0 + t * (x1 - x0)
            y = m["y_base"] + m["amp"] * np.sin(t * TAU * m["wobble"] + m["phase"] * TAU)
            puff.move_to(np.array([x, y, 0.0]))
            fade = min(1.0, t / 0.10, (1.0 - t) / 0.14) * 0.78
            puff.set_fill(interpolate_color(start_c, end_c, t), opacity=max(0.0, fade))

    scene.add(puffs)
    anims = [UpdateFromAlphaFunc(puffs, update)]
    if extra:
        anims.extend(extra)
    scene.play(*anims, run_time=run_time, rate_func=linear)
    scene.remove(puffs, *puffs)
#endregion


#region Gauges and annotation primitives
def meter(label: str, *, length: float = 2.3, thickness: float = 0.52,
          color: str = P_CYAN, vertical: bool = True) -> dict:
    """📊 Track + fill gauge — ``set_meter`` drives the fill from a ``ValueTracker``.

    Turns an invisible quantity (air change rate, driving pressure, remaining
    load, heat flow) into a bar the viewer watches move, instead of a number
    that silently swaps. Returns a dict so the fill stays addressable.
    """
    from manim import Rectangle, UP, VGroup

    from manim_fonts import LABEL_FONT_SIZE, body_text

    w, h = (thickness, length) if vertical else (length, thickness)
    track = Rectangle(
        width=w, height=h, color=P_WHITE, stroke_width=2,
        fill_color=P_DEEP_DARK, fill_opacity=1.0,
    )
    fill = Rectangle(
        width=w - 0.08, height=h - 0.08, stroke_width=0,
        fill_color=color, fill_opacity=0.85,
    ).move_to(track.get_center())
    cap = body_text(label, font_size=LABEL_FONT_SIZE, color=color).next_to(track, UP, buff=0.14)
    return {
        "track": track, "fill": fill, "label": cap, "vertical": vertical,
        "span": length - 0.08, "group": VGroup(track, fill, cap),
    }


def set_meter(gauge: dict, frac: float) -> None:
    """📈 Resize a ``meter`` fill to ``frac`` of its track, anchored at the base."""
    from manim import RIGHT, UP

    span = max(0.004, float(frac) * gauge["span"])
    fill, track = gauge["fill"], gauge["track"]
    if gauge["vertical"]:
        fill.stretch_to_fit_height(span)
        fill.move_to(track.get_bottom() + UP * (span / 2 + 0.04))
    else:
        fill.stretch_to_fit_width(span)
        fill.move_to(track.get_left() + RIGHT * (span / 2 + 0.04))


def bind_meter(gauge: dict, tracker) -> dict:
    """🔗 Let a ``ValueTracker`` drive a ``meter`` fill every frame."""
    set_meter(gauge, tracker.get_value())
    gauge["fill"].add_updater(lambda m: set_meter(gauge, tracker.get_value()))
    return gauge


def chip(text: str, color: str = P_TEAL, *, font_size: int | None = None):
    """🔖 Single-line boxed label — flow-chart node and inline annotation."""
    from manim import SurroundingRectangle, VGroup
    from manim_fonts import BODY_FONT_SIZE, body_text

    if font_size is None:
        font_size = BODY_FONT_SIZE

    label = body_text(text, font_size=font_size, color=color)
    box = SurroundingRectangle(label, color=color, corner_radius=0.1, buff=0.14, stroke_width=1.8)
    return VGroup(box, label)


def dim_chip(boxed, opacity: float) -> list:
    """🌑 Fade a ``chip`` — box stroke and label fill only.

    Never routes through ``set_opacity``: that would give the box a fill it
    never had, so restoring to 1.0 turns the outline into a solid block. Play
    this *before* any ``Indicate`` on the same mobject — ``Indicate`` snapshots
    the starting state and restores it, silently undoing a same-play change.
    """
    return [boxed[0].animate.set_stroke(opacity=opacity),
            boxed[1].animate.set_fill(opacity=opacity)]


def cross_mark(color: str = P_RED, size: float = 0.13):
    """✖️ Negation mark drawn from lines — no glyph coverage to depend on."""
    from manim import DOWN, LEFT, Line, RIGHT, UP, VGroup

    return VGroup(
        Line(LEFT * size + DOWN * size, RIGHT * size + UP * size, color=color, stroke_width=3),
        Line(LEFT * size + UP * size, RIGHT * size + DOWN * size, color=color, stroke_width=3),
    )


def dim_arrow(start, end, color: str = P_YELLOW):
    """📐 Dimension line with end ticks — reads as a measured span, not a stray line."""
    from manim import DoubleArrow, LEFT, Line, RIGHT, VGroup

    shaft = DoubleArrow(
        np.array(start, dtype=float), np.array(end, dtype=float),
        buff=0, color=color, stroke_width=3,
        max_tip_length_to_length_ratio=0.07, tip_length=0.16,
    )
    ticks = VGroup(*[
        Line(np.array(p, dtype=float) + LEFT * 0.13, np.array(p, dtype=float) + RIGHT * 0.13,
             color=color, stroke_width=2)
        for p in (start, end)
    ])
    return VGroup(shaft, ticks)
#endregion


#region Formula morph helpers
def symbol_token(text: str, *, color: str = P_WHITE, font_size: int = 28):
    """🔤 Formula variable ready to receive a ``ReplacementTransform`` from a physical object."""
    from manim_fonts import body_text

    return body_text(text, font_size=font_size, color=color)
#endregion


#region Formula panel
# The one dedicated formula slot every academic scene shares — bottom-center,
# boxed, Text-only fragments so a specific parameter can be ringed on cue.
# Never MathTex, never a raw equation string sliced by character index.
# Sits above the caption bar (see below) — the two never overlap by design.
FORMULA_PANEL_EDGE_BUFF = 1.7


def equation_row(parts, *, font_size: int | None = None, color: str = P_WHITE, buff: float = 0.15):
    """🧮 Build a Text-only equation from named fragments.

    ``parts`` is a list of ``(key, text, part_color)`` triples — ``key`` is
    ``None`` for glue tokens (``"="``, spacing) and a short slug (``"u"``,
    ``"area"``, ``"dt"``) for anything ``highlight_param`` needs to ring later.
    Returns ``(row, items)`` where ``items`` maps key → the fragment mobject.
    """
    from manim import VGroup
    from manim_fonts import FORMULA_FONT_SIZE, body_text

    if font_size is None:
        font_size = FORMULA_FONT_SIZE

    items: dict[str, object] = {}
    mobjs = []
    for key, text, part_color in parts:
        mobj = body_text(text, font_size=font_size, color=part_color or color)
        mobjs.append(mobj)
        if key:
            items[key] = mobj
    row = VGroup(*mobjs).arrange(buff=buff)
    return row, items


def formula_panel(row, *, color: str = P_TEAL, edge_buff: float = FORMULA_PANEL_EDGE_BUFF):
    """📐 Place an equation row in the one fixed formula position and box it.

    Same spot, same box, every scene — so viewers learn where to look.
    """
    from manim import DOWN, SurroundingRectangle

    row.to_edge(DOWN, buff=edge_buff)
    row.set_x(0)
    box = SurroundingRectangle(row, color=color, buff=0.22, corner_radius=0.1, stroke_width=2)
    return row, box


def highlight_param(items: dict, key: str, *, color: str = P_ORANGE):
    """🔦 Ring for one named formula fragment — play it while the narration names that parameter."""
    from manim import SurroundingRectangle

    return SurroundingRectangle(items[key], color=color, buff=0.08, stroke_width=3, corner_radius=0.05)
#endregion


#region Caption bar (German subtitles)
# The one dedicated subtitle slot — fixed bottom edge, boxed for legibility,
# sitting below formula_panel so the two fixed zones never collide.
# Always CAPTION_FONT_SIZE, always center-aligned lines — never auto-shrink
# (that made long clauses look tiny next to short ones and undid hand-breaks).
CAPTION_EDGE_BUFF = 0.35
CAPTION_MAX_WIDTH = 11.5
CAPTION_LINE_BUFF = 0.10
# Two lines is what SAFE_BOTTOM was measured against; a third reaches y ≈ -2.1 and
# starts colliding with beat content, so it is a text problem, not a layout one.
CAPTION_MAX_LINES = 2


def caption_bar(text_de: str, *, font_size: int | None = None, color: str = P_WHITE):
    """💬 German subtitle line, fixed at the very bottom, boxed for legibility against any backdrop.

    Always ``CAPTION_FONT_SIZE`` — never shrink. Word-wraps to ``CAPTION_MAX_WIDTH``
    (and honours hand ``\\n``) so long clauses stay on-screen, center-aligned, with
    a fixed vertical gap between lines.
    """
    from manim import DOWN, RoundedRectangle, VGroup
    from manim_fonts import CAPTION_FONT_SIZE, centered_body_text

    if font_size is None:
        font_size = CAPTION_FONT_SIZE

    label = centered_body_text(
        text_de,
        font_size=font_size,
        color=color,
        line_buff=CAPTION_LINE_BUFF,
        max_width=CAPTION_MAX_WIDTH,
    )
    # centered_body_text returns a bare Text for one line and a VGroup of Texts for
    # several — counting submobjects on the single-line case would count glyphs.
    from manim import VGroup as _VGroup

    n_lines = len(label.submobjects) if isinstance(label, _VGroup) else 1
    if n_lines > CAPTION_MAX_LINES:
        import warnings

        warnings.warn(
            f"caption_bar wrapped to {n_lines} lines (max {CAPTION_MAX_LINES}); "
            f"shorten the German clause: {text_de!r}",
            stacklevel=2,
        )
    bg = RoundedRectangle(
        width=label.width + 0.5,
        height=label.height + 0.3,
        corner_radius=0.08,
        fill_color=P_DEEP_DARK,
        fill_opacity=0.78,
        stroke_width=0,
    )
    bg.move_to(label.get_center())
    group = VGroup(bg, label)
    group.to_edge(DOWN, buff=CAPTION_EDGE_BUFF)
    group.set_x(0)
    return group


def swap_caption(scene, old, text_de: str, *, run_time: float = 0.35, **kwargs):
    """💬 Swap the on-screen caption to the next clause's German subtitle; returns the new one.

    Fades out then in — not a simultaneous cross-fade. Two different
    sentences blended mid-transition render as doubled, overlapping text
    (reproduced while verifying ``Heating/1_introduction/scene_1.py``); a
    sequential fade never overlaps two different strings on screen at once.
    """
    from manim import FadeIn, FadeOut

    new = caption_bar(text_de, **kwargs)
    half = run_time / 2
    scene.play(FadeOut(old), run_time=half)
    scene.play(FadeIn(new), run_time=half)
    return new
#endregion


#region Narration timing
# Every Beat embeds its own narration as an ordered list of
# (section_key, narration_en, subtitle_de) clauses — one list drives the TTS
# text, the on-screen German subtitle, AND exactly how many seconds each
# section needs. No hand-typed "hold this many seconds" guesses, no separate
# subtitle file that can drift out of sync with what's spoken.
NARRATION_WPS: float = 2.5  # spoken words per second, English VO

Clause = tuple[str, str, str]  # (section_key, narration_en, subtitle_de)


def narration_seconds(narration: list[Clause], key: str | None = None) -> float:
    """⏱️ Estimated spoken seconds for one clause (``key``) or the whole narration."""
    words = sum(len(text_en.split()) for section, text_en, _ in narration if key is None or section == key)
    return round(words / NARRATION_WPS, 2)


def narration_text(narration: list[Clause], key: str | None = None) -> str:
    """📝 Join clauses' English text back into one string — what ``generate_audio.py`` sends to TTS."""
    return " ".join(text_en for section, text_en, _ in narration if key is None or section == key)


def subtitle_text(narration: list[Clause], key: str) -> str:
    """💬 The German subtitle for one clause — what ``caption_bar()`` displays while it plays."""
    for section, _, text_de in narration:
        if section == key:
            return text_de
    raise KeyError(key)


def hold_for(scene, narration: list[Clause], key: str, *, used: float = 0.0, min_wait: float = 0.3) -> float:
    """⏸️ Wait exactly as long as this clause's narration needs, minus animation time already spent.

    ``used`` is the sum of ``run_time`` already spent animating this clause's
    visuals (e.g. ``Create``ing the highlight ring) — only the remainder is
    idle wait, so the beat never runs shorter or longer than its own VO.
    """
    remaining = max(min_wait, narration_seconds(narration, key) - used)
    scene.wait(remaining)
    return remaining
#endregion
