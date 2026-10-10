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
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, PASTEL_WHITE, PASTEL_CYAN, PASTEL_TEAL, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_RED, PASTEL_BLUE, PASTEL_GREEN,
    PASTEL_PURPLE,
    fit_band, watt_anchor, radiation_ray,
    smooth_path, flow_guides, flow_animation, ripples, pulse_flashes,
    person_glyph, radiator, moon_glyph, window_glyph,
    meter, set_meter, chip, cross_mark, dim_arrow,
    highlight_param, math_label, math_panel, de_num, place_math,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Modul 1: Die Grundlagen der Bauphysik"


#region The through-line: one exterior wall between a heated room and a winter night
# Conduction, convection and radiation are three ways the *same* heat leaves the
# same room. Beats 1–5 redraw this one section — the Physical Fundamentals room
# slabs, radiator and occupant, a plastered brick wall, a snowy night outside —
# and only change the mechanism drawn on it. The occupant stands between the
# side wall and the radiator so the radiator's rays and the room-air loop never
# cross the figure. Coordinates are design units; ``_stage(scale=…)``
# shrinks the whole section about ``STAGE_PIVOT`` when a beat needs the
# formula band, and ``stage["p"]`` / ``stage["fit"]`` map later drawings onto it.
THETA_I, THETA_E = 20, 0
FLOOR_Y, CEIL_Y = -1.25, 1.75
SLAB = 0.22
SIDE_WALL = 0.2
ROOM_X0, WALL_X0, WALL_X1, OUT_X1 = -6.3, -0.7, 0.7, 6.3
PLASTER = 0.10
LABEL_Y = -1.70
STAGE_PIVOT = np.array([0.0, 2.0, 0.0])
RADIATOR_X, RADIATOR_SCALE = -4.6, 1.1
RADIATOR = (RADIATOR_X - 0.51, FLOOR_Y, RADIATOR_X + 0.51, FLOOR_Y + 0.63)
RAD_EMIT = (RADIATOR[2] + 0.04, FLOOR_Y + 0.35)
OCCUPANT_X, OCCUPANT_SCALE = -5.72, 2.2
BRICK_FILL = "#2A2E35"
MORTAR = "#A9B0BA"
HOT = PASTEL_RED
SHELL_STYLE = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)
STARS = ((2.1, 1.62), (3.0, 1.08), (3.9, 1.68), (4.4, 0.25), (5.9, 0.62), (2.6, 0.12), (5.2, -0.15), (6.0, 1.62))


def _box(x0, y0, x1, y1, **style):
    """▭ Axis-aligned polygon from two design corners."""
    return Polygon(
        np.array([x0, y0, 0.0]), np.array([x1, y0, 0.0]),
        np.array([x1, y1, 0.0]), np.array([x0, y1, 0.0]), **style,
    )


def _brick_wall():
    """🧱 Plastered masonry: brick courses with offset joints between two render coats."""
    bx0, bx1 = WALL_X0 + PLASTER, WALL_X1 - PLASTER
    y0, y1 = FLOOR_Y - SLAB, CEIL_Y + SLAB
    core = _box(bx0, y0, bx1, y1, stroke_width=0, fill_color=BRICK_FILL, fill_opacity=1.0)
    joints = VGroup()
    course, brick = 0.26, 0.62
    rows = int(np.ceil((y1 - y0) / course))
    for i in range(1, rows):
        y = y0 + i * course
        joints.add(Line(np.array([bx0, y, 0.0]), np.array([bx1, y, 0.0]), color=MORTAR, stroke_width=1.3))
    for i in range(rows):
        ya, yb = y0 + i * course, min(y1, y0 + (i + 1) * course)
        x = bx0 + (brick / 2 if i % 2 else 0.0) + brick
        while x < bx1 - 0.05:
            joints.add(Line(np.array([x, ya, 0.0]), np.array([x, yb, 0.0]), color=MORTAR, stroke_width=1.3))
            x += brick
    inner = _box(WALL_X0, y0, bx0, y1, stroke_width=0, fill_color="#E8E2D6", fill_opacity=0.95)
    outer = _box(bx1, y0, WALL_X1, y1, stroke_width=0, fill_color="#B9C2CC", fill_opacity=0.95)
    outline = _box(WALL_X0, y0, WALL_X1, y1, stroke_color=PASTEL_WHITE, stroke_width=2.4, fill_opacity=0.0)
    tint = VGroup(*[
        _box(bx0 + k * (bx1 - bx0) / 16, y0, bx0 + (k + 1) * (bx1 - bx0) / 16, y1,
             stroke_width=0, fill_color=interpolate_color(ManimColor(HOT), ManimColor(PASTEL_BLUE), (k + 0.5) / 16),
             fill_opacity=0.0)
        for k in range(16)
    ])
    return {"core": core, "joints": joints, "inner": inner, "outer": outer, "outline": outline,
            "tint": tint, "group": VGroup(core, tint, joints, inner, outer, outline)}


def _radiator():
    """♨️ Physical Fundamentals panel radiator standing on the floor slab."""
    rad = radiator(ORIGIN, PASTEL_RED).scale(RADIATOR_SCALE)
    return rad.move_to(np.array([RADIATOR_X, FLOOR_Y + rad.height / 2, 0.0]))


def _occupant(x=OCCUPANT_X, color=PASTEL_ORANGE, scale=OCCUPANT_SCALE):
    """🧍 Physical Fundamentals occupant standing on the floor."""
    person = person_glyph(ORIGIN, color=color, scale=scale)
    return person.move_to(np.array([x, FLOOR_Y + person.height / 2, 0.0]))


def _night_sky():
    """🌙 Physical Fundamentals crescent and a few stars over the snow."""
    moon = moon_glyph(np.array([5.25, 1.35, 0.0]), color="#F2E8C9")
    stars = VGroup(*[
        Dot(np.array([x, y, 0.0]), radius=0.025, color=PASTEL_WHITE, fill_opacity=0.8) for x, y in STARS
    ])
    return VGroup(moon, stars)


def _ground():
    """❄️ Ground outside — the Physical Fundamentals ground line with hatches."""
    line = Line(np.array([WALL_X1, FLOOR_Y, 0.0]), np.array([OUT_X1, FLOOR_Y, 0.0]), color=PASTEL_TEAL, stroke_width=2.2)
    hatches = VGroup(*[
        Line(q, q + np.array([-0.07, -0.1, 0.0]), color=PASTEL_TEAL, stroke_width=1.1)
        for q in [line.point_from_proportion(u) for u in np.linspace(0.04, 0.96, 15)]
    ])
    return VGroup(line, hatches)


def _stage(*, scale: float = 1.0):
    """🏠 Heated room | brick exterior wall | winter night — the section Beats 1–5 share."""
    room = _box(ROOM_X0, FLOOR_Y, WALL_X0, CEIL_Y, stroke_width=0, fill_opacity=0.0)
    night = _box(WALL_X1, FLOOR_Y, OUT_X1, CEIL_Y + SLAB, stroke_width=0, fill_opacity=0.0)
    floor = _box(ROOM_X0 - SIDE_WALL, FLOOR_Y - SLAB, WALL_X0, FLOOR_Y, **SHELL_STYLE)
    ceiling = _box(ROOM_X0 - SIDE_WALL, CEIL_Y, WALL_X0, CEIL_Y + SLAB, **SHELL_STYLE)
    side = _box(ROOM_X0 - SIDE_WALL, FLOOR_Y, ROOM_X0, CEIL_Y, **SHELL_STYLE)
    snow = _ground()
    wall = _brick_wall()
    radiator = _radiator()
    occupant = _occupant()
    sky = _night_sky()

    ti = Text(f"innen {THETA_I} °C", font_size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
    ti.move_to(np.array([(ROOM_X0 + WALL_X0) / 2, LABEL_Y, 0.0]))
    te = Text(f"außen {THETA_E} °C", font_size=BODY_FONT_SIZE, color=PASTEL_BLUE)
    te.move_to(np.array([(WALL_X1 + OUT_X1) / 2, LABEL_Y, 0.0]))
    dth = place_math(_dtheta_label(THETA_I - THETA_E), np.array([0.0, ti.get_bottom()[1], 0.0]), "center")

    shell = VGroup(room, night, floor, ceiling, side, snow)
    group = VGroup(shell, wall["group"], radiator, occupant, sky, ti, te, dth)
    group.scale(scale, about_point=STAGE_PIVOT)

    def p(x, y):
        return STAGE_PIVOT + scale * np.array([x, y - STAGE_PIVOT[1], 0.0])

    def fit(mob):
        return mob.scale(scale, about_point=STAGE_PIVOT)

    return {
        "room": room, "night": night, "shell": shell, "wall": wall, "radiator": radiator,
        "occupant": occupant, "sky": sky, "ti": ti, "te": te, "dth": dth,
        "furniture": VGroup(radiator, occupant, sky),
        "group": group, "p": p, "fit": fit, "s": scale,
    }


def _show_stage(scene, stage, *, labels=True, run_time: float = 1.4):
    """🎬 Draw the section in a fixed order: spaces, slabs and wall, then furniture and labels."""
    scene.play(FadeIn(stage["shell"]), FadeIn(stage["wall"]["group"]), run_time=run_time)
    extra = [FadeIn(stage["ti"]), FadeIn(stage["te"])] if labels else []
    scene.play(FadeIn(stage["furniture"]), *extra, run_time=0.8)
    return run_time + 0.8


def _warmth(stage, run_time: float):
    """♨️ The heated radiator — long-wave ripples rising off its top while a clause is read."""
    top = stage["radiator"].get_top()
    w = stage["radiator"].width
    centers = [top + RIGHT * dx * w + UP * 0.03 for dx in (-0.28, 0.0, 0.28)]
    return [ripples(centers, r_max=0.42 * stage["s"], color=HOT, cycles=max(1.0, run_time / 1.3))]
#endregion


#region Particles with a colour along the path
def _path(points):
    """〰️ Smooth track through design points."""
    return smooth_path([(x, y, 0.0) for x, y in points])


def _knots_color(knots):
    """🎨 Piecewise colour over ``t`` in [0, 1] from ``(t, colour)`` knots."""
    ts = [k[0] for k in knots]
    cs = [ManimColor(k[1]) for k in knots]

    def color(t):
        i = int(np.clip(np.searchsorted(ts, t) - 1, 0, len(ts) - 2))
        u = float(np.clip((t - ts[i]) / max(1e-6, ts[i + 1] - ts[i]), 0.0, 1.0))
        return interpolate_color(cs[i], cs[i + 1], u)

    return color


def _chord_t(points, index):
    """📏 Arc-length fraction of ``points[index]`` along the polyline — knots for ``_knots_color``."""
    pts = np.array(points, dtype=float)
    seg = np.linalg.norm(np.diff(pts, axis=0), axis=1)
    return float(np.sum(seg[:index]) / np.sum(seg))


def _loop_flow(path, color_fn, *, cycles: float, n: int = 12, radius: float = 0.06):
    """🔁 ``flow_animation`` particles on a closed loop — coloured by ``color_fn(t)``, no fade at the seam."""
    parts = VGroup(*[Ellipse(width=radius * 2.8, height=radius * 1.15, stroke_width=0, fill_opacity=0.0)
                     for _ in range(n)])

    def update(group, alpha):
        for k, dot in enumerate(group):
            t = (alpha * cycles + k / n) % 1.0
            pos, ahead = path.point_from_proportion(t), path.point_from_proportion((t + 0.01) % 1.0)
            dot.move_to(pos)
            dot.set_angle(float(np.arctan2(ahead[1] - pos[1], ahead[0] - pos[0])))
            dot.set_fill(color_fn(t), opacity=0.92)

    return UpdateFromAlphaFunc(parts, update, remover=True, rate_func=linear)
#endregion


#region Mechanism drawings on the section
RISE_PTS = [(RADIATOR_X, -0.48), (RADIATOR_X, 0.55), (RADIATOR_X + 0.25, 1.27), (RADIATOR_X + 0.9, 1.50),
            (-2.5, 1.52)]
RETURN_PTS = [(-1.65, -1.08), (-3.2, -1.08), (-3.75, -1.0), (-3.95, -0.72), (-4.25, -0.5), (RADIATOR_X, -0.48)]


def _loop_points():
    """🔁 Room air loop: rise at the radiator, ceiling run, sink at the cold wall, return on the floor."""
    return [*RISE_PTS, (-1.45, 1.47), (-1.02, 1.12), (-0.96, 0.40), (-0.96, -0.45), (-1.08, -0.92), *RETURN_PTS]


def _loop(stage):
    """🌀 Closed room-air loop and its temperature colouring (warm ➜ cooled at the wall ➜ rewarmed)."""
    pts = _loop_points()
    path = stage["fit"](_path(pts))
    color = _knots_color([
        (0.0, HOT), (_chord_t(pts, 5), PASTEL_ORANGE), (_chord_t(pts, 9), PASTEL_BLUE),
        (_chord_t(pts, 12), PASTEL_BLUE), (1.0, HOT),
    ])
    return path, color


def _wind(stage):
    """🌬️ Wind lanes: in from the right, up along the facade, away into the night."""
    lanes = []
    for k, off in enumerate((0.0, 0.24, 0.48)):
        pts = [(6.1, -0.95 + 0.35 * k), (3.0, -0.95 + 0.35 * k), (1.5 + off, -0.65 + 0.25 * k),
               (0.98 + off, 0.05), (0.98 + off, 0.85), (1.45 + off, 1.35), (2.35 + off, 1.62)]
        lanes.append(stage["fit"](_path(pts)))
    return lanes


def _through(stage, ys=(-0.45, 0.25, 0.95)):
    """🧱 Conduction lanes across the masonry, inner face to outer face."""
    return [stage["fit"](_path([(WALL_X0 + 0.15, y), (WALL_X1 - 0.15, y)])) for y in ys]


def _ir_rays(stage, pairs):
    """📏 Infrared rays between surfaces — straight, evenly placed lines."""
    p = stage["p"]
    return VGroup(*[radiation_ray(p(*a), p(*b), color=PASTEL_YELLOW, stroke_width=2.2) for a, b in pairs])


def _ir_paths(stage, pairs):
    """🛤️ Ray endpoints on the stage — the tracks ``pulse_flashes`` runs along."""
    return [[stage["p"](*a), stage["p"](*b)] for a, b in pairs]


def _ir_glow(stage, pairs, run_time: float, *, pulses: bool = True):
    """✨ Long-wave emission: ripples off every emitting surface, bright packets running along the rays."""
    p, cycles = stage["p"], max(1.0, run_time / 1.3)
    anims = []
    if any(a[0] < 0 for a, _b in pairs):
        anims.append(ripples([p(RAD_EMIT[0], y) for y in (FLOOR_Y + 0.2, FLOOR_Y + 0.48)], r_max=0.5 * stage["s"],
                             color=HOT, cycles=cycles, facing=0.0))
    if any(a[0] > 0 for a, _b in pairs):
        anims.append(ripples([p(WALL_X1 + 0.03, y) for y in (-0.45, 0.45, 1.35)], r_max=0.45 * stage["s"],
                             color=PASTEL_BLUE, cycles=cycles, facing=0.0))
    if pulses:
        anims.append(pulse_flashes(_ir_paths(stage, pairs), PASTEL_YELLOW, repeats=max(1, int(run_time / 1.3)),
                                   width=6.0))
    return anims


INNER_RAYS = [(RAD_EMIT, (WALL_X0 - 0.02, y)) for y in (-0.90, -0.15, 0.60, 1.35)]
OUTER_RAYS = [((WALL_X1 + 0.02, y), (WALL_X1 + 2.02, y + 0.80)) for y in (-0.75, -0.15, 0.45, 1.05)]
#endregion


#region Wall temperature
def _tint_wall(stage, opacity: float = 0.22):
    """🌡️ Animation that colours the masonry warm inside ➜ cold outside — the gradient conduction runs down."""
    return stage["wall"]["tint"].animate.set_fill(opacity=opacity)


def _dim(mobs, opacity: float):
    """🌫️ Dim strokes, and only fills that are already visible — curved guides never fill in."""
    target = mobs.copy()
    for m in target.get_family():
        m.set_stroke(opacity=min(opacity, m.get_stroke_opacity()))
        if m.get_fill_opacity() > 0:
            m.set_fill(opacity=min(opacity, m.get_fill_opacity()))
    return Transform(mobs, target)
#endregion


#region Route strip
def _route_strip(active: int | None = None):
    """🧭 Leitung · Konvektion · Strahlung — which of the three a beat is on."""
    names = ("Leitung", "Konvektion", "Strahlung")
    colors = (PASTEL_RED, PASTEL_CYAN, PASTEL_YELLOW)
    strip = VGroup(*[
        chip(n, c if active == i else PASTEL_TEAL, font_size=LABEL_FONT_SIZE)
        for i, (n, c) in enumerate(zip(names, colors))
    ]).arrange(RIGHT, buff=0.5)
    strip.move_to(np.array([0.0, -2.16, 0.0]))
    if active is not None:
        for i, boxed in enumerate(strip):
            if i != active:
                boxed[0].set_stroke(opacity=0.35)
                boxed[1].set_fill(opacity=0.35)
    return strip
#endregion


#region Typeset symbols
def _dtheta_src(kelvin: float) -> str:
    """🔺 LaTeX source of the driving temperature difference."""
    return rf"\Delta\theta = {de_num(kelvin)}\,\mathrm{{K}}"


def _dtheta_label(kelvin: float):
    """📏 Typeset Δθ label for the section's centre slot."""
    return math_label(_dtheta_src(kelvin), size=BODY_FONT_SIZE, color=PASTEL_YELLOW)


def _q_token(sub: str, color: str):
    """🔣 Typeset heat-flow symbol Q̇ with an upright mechanism index (k, c, r)."""
    return math_label(rf"\dot{{Q}}_{{\mathrm{{{sub}}}}}", size=FORMULA_FONT_SIZE, color=color)
#endregion

#region Live readouts
_READOUT_CACHE: dict = {}


def _live(fn, at, *, size=None, color=PASTEL_WHITE, edge="left"):
    """📟 ``math_readout`` that reuses the typeset box while ``fn()`` returns the same source.

    https://docs.manim.community/en/stable/reference/manim.animation.updaters.mobject_update_utils.html
    """
    def build():
        key = (fn(), size, color)
        if key not in _READOUT_CACHE:
            _READOUT_CACHE[key] = math_label(key[0], size=size, color=color)
        return place_math(_READOUT_CACHE[key].copy(), np.array(at() if callable(at) else at, dtype=float), edge)

    return always_redraw(build)
#endregion


#region Shared glyphs
def _flow_arrow(start, end, color, width=4):
    """➡️ Plain directed heat arrow."""
    return Arrow(
        np.array(start, dtype=float), np.array(end, dtype=float),
        buff=0, color=color, stroke_width=width, max_tip_length_to_length_ratio=0.22,
    )


def _layer(width, height, color, opacity, label, *, center=ORIGIN, font_size=LABEL_FONT_SIZE):
    """🧱 One construction layer — the unit R_ges is summed over. No inline tag:

    columns as thin as 0.26 units (an outer render coat) can never fit a
    readable label under themselves without it overlapping the next column;
    see ``_layer_legend`` for the paired swatch-and-name key instead.
    """
    return Rectangle(
        width=width, height=height, color=color, stroke_width=2,
        fill_color=color, fill_opacity=opacity,
    ).move_to(center)


def _layer_legend(entries):
    """🏷️ Swatch + name per layer, stacked — the readable twin of a too-narrow ``_layer``."""
    from manim import RIGHT, Rectangle

    rows = VGroup(*[
        VGroup(
            Rectangle(width=0.32, height=0.20, color=color, fill_color=color,
                     fill_opacity=0.75, stroke_width=1.5),
            Text(name, font_size=LABEL_FONT_SIZE, color=color),
        ).arrange(RIGHT, buff=0.14)
        for name, color in entries
    ]).arrange(DOWN, aligned_edge=LEFT, buff=0.14)
    return rows


def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the same empty top-right corner.

    Every section of Module 1 names the norm it is built on (mostly
    DIN EN ISO 6946 for building-component heat transfer; DIN EN 12831-1 where
    the beat is really about the heating load). Dim so it reads as a footnote,
    never competing with the diagram.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=PASTEL_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Beat1 – Heat crosses from warm to cold, and only that way
class Beat1_DreiWegeDerWaerme(Scene):
    NARRATION = [
        ("intro",
         "Before we calculate how much heat a building loses, we need to see how heat moves at all.",
         "Bevor wir berechnen, wie viel Wärme ein Gebäude verliert, sehen wir uns an, wie Wärme sich überhaupt bewegt."),
        ("setup",
         "Here is the whole problem in one picture: a heated room at twenty degrees, a winter night at zero, and the exterior wall in between.",
         "Das ganze Problem in einem Bild: ein beheizter Raum bei zwanzig Grad, eine Winternacht bei null Grad — dazwischen die Außenwand."),
        ("driver",
         "That twenty kelvin difference is the only driver. No difference, no heat flow.",
         "Diese zwanzig Kelvin Unterschied sind der einzige Antrieb. Kein Unterschied, kein Wärmestrom."),
        ("direction",
         "Energy always crosses from warm toward cold. The wall warms on the inside and stays cold on the outside, and the gap closes a little.",
         "Energie wandert immer von warm nach kalt. Die Wand wird innen warm und bleibt außen kalt — der Unterschied wird etwas kleiner."),
        ("never",
         "Spontaneously, never the other way. That is the second law, and it is why a building has to be heated at all.",
         "Von allein nie umgekehrt. Das ist der zweite Hauptsatz — und der Grund, warum ein Gebäude überhaupt beheizt werden muss."),
        ("three",
         "Heat makes that crossing in exactly three ways, and the whole module is built on them.",
         "Wärme macht diesen Übergang auf genau drei Wegen — auf ihnen baut das ganze Modul auf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Abschnitt 1.1 — Wie Wärme sich bewegt", title)
        din = _din_ref("2. Hauptsatz")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        stage = _stage()
        p = stage["p"]
        ti_cool = Text("innen 19 °C", font_size=BODY_FONT_SIZE, color=PASTEL_ORANGE).move_to(stage["ti"])
        te_warm = Text("außen 1 °C", font_size=BODY_FONT_SIZE, color=PASTEL_BLUE).move_to(stage["te"])
        dt = ValueTracker(0.0)
        dth_live = _live(
            lambda: _dtheta_src(dt.get_value()),
            np.array([stage["wall"]["outline"].get_center()[0], stage["ti"].get_bottom()[1], 0.0]),
            size=BODY_FONT_SIZE, color=PASTEL_YELLOW, edge="center",
        )
        heat_lanes = [stage["fit"](_path([(-3.75, y), (4.6, y)])) for y in (-0.85, -0.3, 0.8, 1.35)]
        heat_guides = flow_guides(heat_lanes, HOT, opacity=0.18)
        reverse = _flow_arrow(p(2.8, 0.25), p(-2.4, 0.25), PASTEL_BLUE)
        no_mark = VGroup(
            Circle(radius=0.37, color=PASTEL_RED, stroke_width=5),
            Line(UP * 0.26 + LEFT * 0.26, DOWN * 0.26 + RIGHT * 0.26, color=PASTEL_RED, stroke_width=5),
        ).move_to(p(0.0, 0.25))
        routes = _route_strip()

        def heat(rt):
            return [flow_animation([(heat_lanes, HOT, PASTEL_BLUE)], waves=4, radius=0.07, cycles=rt / 2.6),
                    *_warmth(stage, rt)]

        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "setup"))
        _show_stage(self, stage)
        hold_for(self, self.NARRATION, "setup", during=lambda rt: _warmth(stage, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "driver"))
        self.add(dth_live)
        self.play(dt.animate.set_value(THETA_I - THETA_E), *_warmth(stage, 0.8), run_time=0.8)
        self.play(Indicate(dth_live, color=PASTEL_YELLOW, scale_factor=1.2), *_warmth(stage, 0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "driver", during=lambda rt: _warmth(stage, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "direction"))
        self.play(Create(heat_guides), *_warmth(stage, 0.6), run_time=0.6)
        self.play(*heat(3.2), _tint_wall(stage), run_time=3.2)
        self.play(*heat(2.4), ReplacementTransform(stage["ti"], ti_cool), ReplacementTransform(stage["te"], te_warm),
                  dt.animate.set_value(18), run_time=2.4)
        hold_for(self, self.NARRATION, "direction", during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "never"))
        self.play(GrowArrow(reverse), *heat(0.8), run_time=0.8)
        self.play(Create(no_mark), reverse.animate.set_stroke(color=PASTEL_WHITE, opacity=0.22), *heat(0.7),
                  run_time=0.7)
        hold_for(self, self.NARRATION, "never", during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "three"))
        self.play(FadeOut(reverse), FadeOut(no_mark), *heat(0.5), run_time=0.5)
        self.play(LaggedStart(*[FadeIn(c, shift=UP * 0.14) for c in routes], lag_ratio=0.22), *heat(1.4),
                  run_time=1.4)
        hold_for(self, self.NARRATION, "three", during=heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Conduction: a magnifier on the brick, particles pass the vibration on
LENS_C = (0.0, 0.25)
LENS_R = 1.5
LENS_WALL = 0.95


def _smoothstep(u):
    """〽️ Smooth 0→1 ramp."""
    u = float(np.clip(u, 0.0, 1.0))
    return u * u * (3.0 - 2.0 * u)


def _lens(stage):
    """🔍 Magnifier on the wall: air | brick lattice | air, clipped to the lens circle."""
    c = stage["p"](*LENS_C)
    r = LENS_R * stage["s"]
    w = LENS_WALL * stage["s"]
    disc = Circle(radius=r, stroke_width=0, fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to(c)

    def band(x0, x1, color, opacity):
        rect = Rectangle(width=x1 - x0, height=2 * r + 0.2).move_to(c + np.array([(x0 + x1) / 2, 0.0, 0.0]))
        return Intersection(disc.copy(), rect, stroke_width=0, fill_color=color, fill_opacity=opacity)

    bands = VGroup(band(-r, -w, P_DEEP_DARK, 0.0), band(-w, w, BRICK_FILL, 1.0), band(w, r, P_DEEP_DARK, 0.0))
    rim = Circle(radius=r, color="#D5D9DE", stroke_width=7).move_to(c)
    tip = c + r * np.array([np.cos(-0.87), np.sin(-0.87), 0.0])
    handle = Line(tip, tip + 0.75 * np.array([np.cos(-0.87), np.sin(-0.87), 0.0]), color="#D5D9DE", stroke_width=12)
    return {"disc": disc, "bands": bands, "rim": rim, "handle": handle, "c": c, "r": r, "w": w,
            "group": VGroup(disc, bands, rim, handle)}


def _lattice(lens, *, seed: int = 4):
    """⚛️ Bonded particles inside the brick plus free air particles on both sides of the lens."""
    c, r, w = lens["c"], lens["r"], lens["w"]
    xs = np.linspace(-0.80 * w / LENS_WALL, 0.80 * w / LENS_WALL, 6)
    ys = np.arange(-1.2, 1.21, 0.3) * r / LENS_R
    homes, index = [], {}
    for j, y in enumerate(ys):
        for i, x in enumerate(xs):
            if np.hypot(x, y) < r - 0.2:
                index[(i, j)] = len(homes)
                homes.append((c + np.array([x, y, 0.0]), (x - xs[0]) / (xs[-1] - xs[0])))
    pairs = [(index[a], index[b]) for a in index for b in ((a[0] + 1, a[1]), (a[0], a[1] + 1)) if b in index]
    dots = VGroup(*[Dot(h, radius=0.065, color=PASTEL_BLUE) for h, _xn in homes])
    bonds = VGroup(*[Line(homes[a][0], homes[b][0], color=MORTAR, stroke_width=1.6, stroke_opacity=0.55)
                     for a, b in pairs])
    rng = np.random.default_rng(seed)
    air = []
    for side in (-1, 1):
        while sum(1 for a in air if a[1] == side) < 7:
            x = side * float(rng.uniform(w + 0.12, r - 0.12))
            y = float(rng.uniform(-r + 0.2, r - 0.2))
            if np.hypot(x, y) < r - 0.15:
                air.append((c + np.array([x, y, 0.0]), side, float(rng.uniform(0, TAU))))
    air_dots = VGroup(*[Dot(h, radius=0.05, color=HOT if side < 0 else PASTEL_BLUE) for h, side, _ph in air])
    phases = rng.uniform(0, TAU, size=len(homes))
    return {"homes": homes, "pairs": pairs, "dots": dots, "bonds": bonds, "air": air, "air_dots": air_dots,
            "phases": phases, "index": index, "group": VGroup(bonds, dots, air_dots)}


class Beat2_Waermeleitung(Scene):
    NARRATION = [
        ("label",
         "The first way is conduction, and it happens inside the solid masonry itself.",
         "Der erste Weg ist die Wärmeleitung — sie passiert im festen Mauerwerk selbst."),
        ("lattice",
         "Under the magnifier the brick is a lattice of particles, each one bound to its place.",
         "Unter der Lupe ist der Ziegel ein Gitter aus Teilchen, jedes fest an seinen Platz gebunden."),
        ("relay",
         "The warm side makes them vibrate harder, and each one knocks its neighbour into motion. The energy travels along the chain.",
         "Die warme Seite lässt sie stärker schwingen, und jedes stößt seinen Nachbarn an. Die Energie wandert die Kette entlang."),
        ("stayput",
         "But watch one particle: it never leaves its place. Only the energy moves through the wall, not the material.",
         "Aber sehen Sie ein Teilchen an: es verlässt seinen Platz nie. Nur die Energie wandert durch die Wand, nicht das Material."),
        ("symbol",
         "That heat flow through the solid is called Q dot k.",
         "Diesen Wärmestrom durch den Feststoff nennen wir Q-Punkt-k."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmeleitung — Energie ohne Materialtransport", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "label"))
        self.play(FadeIn(caption), run_time=0.3)

        stage = _stage()
        p = stage["p"]
        routes = _route_strip(active=0)
        lens = _lens(stage)
        lat = _lattice(lens)
        marker = DashedVMobject(Circle(radius=0.32, color="#D5D9DE", stroke_width=3).move_to(lens["c"]), num_dashes=16)
        lane = _path([(-1.25, 2.12), (1.25, 2.12)])
        energy = flow_guides([lane], HOT, opacity=0.45, width=3.0)
        energy_tag = Text("Energie", font_size=LABEL_FONT_SIZE, color=PASTEL_RED).next_to(energy, LEFT, buff=0.2)
        lane_k = _path([(-0.55, 0.25), (0.55, 0.25)])
        energy_k = flow_guides([lane_k], HOT, opacity=0.45, width=3.0)

        def flow(rt, track=lane):
            return [flow_animation([([track], HOT, PASTEL_BLUE)], waves=3, radius=0.07, cycles=rt / 1.4)]

        clock = ValueTracker(0.0)
        clock.add_updater(lambda m, dt: m.increment_value(dt))
        front = ValueTracker(0.0)
        cold, hot = ManimColor(PASTEL_BLUE), ManimColor(HOT)

        def temp(xn):
            return (1.0 - 0.85 * xn) * _smoothstep((front.get_value() - xn) / 0.3)

        def jiggle(group):
            t = clock.get_value()
            for dot, (home, xn), ph in zip(lat["dots"], lat["homes"], lat["phases"]):
                tk = temp(xn)
                amp, om = 0.012 + 0.06 * tk, 8.0 + 16.0 * tk
                dot.move_to(home + amp * np.array([np.sin(om * t + ph), np.cos(1.3 * om * t + 0.7 * ph), 0.0]))
                dot.set_fill(interpolate_color(cold, hot, tk), opacity=1.0)
            for line, (a, b) in zip(lat["bonds"], lat["pairs"]):
                line.put_start_and_end_on(lat["dots"][a].get_center(), lat["dots"][b].get_center())
            for dot, (home, side, ph) in zip(lat["air_dots"], lat["air"]):
                amp, om = (0.12, 11.0) if side < 0 else (0.05, 5.0)
                dot.move_to(home + amp * np.array([np.sin(om * t + ph), np.cos(0.8 * om * t + 2 * ph), 0.0]))

        watched = lat["index"][(1, len({k[1] for k in lat["index"]}) // 2)]
        home_w = lat["homes"][watched][0]
        watch_ring = Circle(radius=0.15, color=PASTEL_GREEN, stroke_width=2.5).move_to(home_w)
        watch_tag = Text("bleibt am Platz", font_size=LABEL_FONT_SIZE, color=PASTEL_GREEN).move_to(p(-2.75, 1.35))
        watch_lead = Line(watch_tag.get_right() + RIGHT * 0.08, watch_ring.get_left(), color=PASTEL_GREEN,
                          stroke_width=1.4, stroke_opacity=0.7)

        _show_stage(self, stage, run_time=1.2)
        self.play(FadeIn(routes), Create(marker), *_warmth(stage, 0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "label", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0 + 0.8,
                 during=lambda rt: _warmth(stage, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lattice"))
        self.add(clock)
        self.play(ReplacementTransform(marker, lens["rim"]), FadeIn(lens["disc"]), FadeIn(lens["bands"]),
                  FadeIn(lens["handle"]), run_time=1.0)
        lat["group"].add_updater(jiggle)
        self.play(FadeIn(lat["group"]), run_time=0.8)
        hold_for(self, self.NARRATION, "lattice")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "relay"))
        self.play(Create(energy), FadeIn(energy_tag), run_time=0.6)
        self.play(front.animate.set_value(1.35), _tint_wall(stage), *flow(4.0), run_time=4.0, rate_func=linear)
        hold_for(self, self.NARRATION, "relay", during=flow)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "stayput"))
        self.play(Create(watch_ring), FadeIn(watch_tag), Create(watch_lead), *flow(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "stayput", during=flow)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "symbol"))
        lat["group"].clear_updaters()
        token = _q_token("k", PASTEL_RED).move_to(p(-1.5, 0.25))
        self.play(FadeOut(watch_ring), FadeOut(watch_tag), FadeOut(watch_lead), FadeOut(energy_tag),
                  FadeOut(lens["group"]), ReplacementTransform(lat["group"], token),
                  ReplacementTransform(energy, energy_k), run_time=1.4)
        self.play(*flow(0.8, lane_k), run_time=0.8)
        hold_for(self, self.NARRATION, "symbol", during=lambda rt: flow(rt, lane_k))

        clock.clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Convection: an open window lets the warm room air out and cold air in
WIN_LO, WIN_HI = 0.05, 1.35


def _window_opening(stage):
    """🪟 Opening cut through the masonry with a Physical Fundamentals casement at the inner face, swinging into the room."""
    p = stage["p"]
    cut = Polygon(p(WALL_X0, WIN_LO), p(WALL_X1, WIN_LO), p(WALL_X1, WIN_HI), p(WALL_X0, WIN_HI),
                  stroke_width=0, fill_color=P_DEEP_DARK, fill_opacity=1.0)
    frame = VGroup(*[Line(p(WALL_X0, y), p(WALL_X1, y), color=PASTEL_WHITE, stroke_width=2.4) for y in (WIN_LO, WIN_HI)])
    win = window_glyph(p(WALL_X0 + 0.3, 0)[0], p(0, WIN_LO)[1], p(0, WIN_HI)[1], depth=0.7 * stage["s"], color=PASTEL_CYAN)
    return {"cut": cut, "frame": frame, "window": win, "group": VGroup(cut, frame, win["group"])}


def _open_inward(window, *, run_time: float = 1.0):
    """🚪 ``open_window`` mirrored — the sash turns about its hinge into a room that lies left of the wall."""
    return Rotate(window["sash"], angle=PI / 2, axis=UP, about_point=window["hinge"], run_time=run_time)


def _window_flows(stage):
    """🌬️ Rising air at the radiator, warm room air out through the top of the opening, cold air in at the sill."""
    rise = stage["fit"](_path([*RISE_PTS, (-1.2, 1.4)]))
    warm = stage["fit"](_path([*RISE_PTS, (-1.3, 1.30), (-0.6, 1.05), (0.6, 1.0), (1.4, 1.2), (2.6, 1.65)]))
    cold = stage["fit"](_path([(3.6, 0.05), (2.0, 0.22), (0.6, 0.30), (-0.6, 0.30), (-1.1, 0.0), (-1.25, -0.6),
                               *RETURN_PTS]))
    return rise, warm, cold


class Beat3_Konvektion(Scene):
    NARRATION = [
        ("label",
         "The second way is convection: moving air carries the heat with it.",
         "Der zweite Weg ist die Konvektion: bewegte Luft trägt die Wärme mit sich."),
        ("heater",
         "At the radiator the air warms, becomes lighter and rises. Under the ceiling it flows toward the window.",
         "Am Heizkörper erwärmt sich die Luft, wird leichter und steigt auf. Unter der Decke strömt sie zum Fenster."),
        ("window",
         "When the window is open, the warm air escapes at the top — and takes its heat outside.",
         "Ist das Fenster offen, entweicht die warme Luft oben ins Freie — und nimmt ihre Wärme mit."),
        ("cold",
         "At the bottom cold outdoor air flows in, sinks to the floor and is warmed again at the radiator. The room cools down.",
         "Unten strömt kalte Außenluft herein, fällt zu Boden und wird am Heizkörper wieder erwärmt. Der Raum kühlt ab."),
        ("symbol",
         "Heat carried by moving air is Q dot c. How much air is exchanged is the topic of Module 3.",
         "Wärme, die bewegte Luft trägt, nennen wir Q-Punkt-c. Wie viel Luft getauscht wird, zeigt Modul 3."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Konvektion — bewegte Luft trägt die Wärme", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "label"))
        self.play(FadeIn(caption), run_time=0.3)

        stage = _stage()
        p = stage["p"]
        routes = _route_strip(active=1)
        window = _window_opening(stage)
        rise, warm, cold = _window_flows(stage)
        rise_guide = flow_guides([rise], HOT, opacity=0.25)
        warm_guide = flow_guides([warm], HOT, opacity=0.25)
        cold_guide = flow_guides([cold], PASTEL_BLUE, opacity=0.25)
        glow = stage["radiator"][0].copy().set_fill(PASTEL_RED, opacity=0.55)
        warm_tag = Text("warm, leicht", font_size=LABEL_FONT_SIZE, color=HOT).move_to(p(-3.0, 0.55))
        cold_tag = Text("kalt, schwer", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE).move_to(p(2.6, 0.6))
        open_tag = Text("Fenster geöffnet", font_size=LABEL_FONT_SIZE, color=PASTEL_CYAN).move_to(p(2.3, -0.75))
        ti_cool = Text("innen 17 °C", font_size=BODY_FONT_SIZE, color=PASTEL_ORANGE).move_to(stage["ti"])
        streams = {"rise": (rise, HOT, PASTEL_ORANGE), "warm": (warm, HOT, PASTEL_PURPLE),
                   "cold": (cold, PASTEL_BLUE, HOT)}

        def air(*keys):
            def during(rt):
                return [flow_animation([([streams[k][0]], *streams[k][1:]) for k in keys], waves=10, radius=0.07,
                                       cycles=rt / 6.0)]
            return during

        _show_stage(self, stage, run_time=1.2)
        self.play(FadeIn(window["group"]), FadeIn(routes), *_warmth(stage, 0.6), run_time=0.6)
        hold_for(self, self.NARRATION, "label", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0 + 0.6,
                 during=lambda rt: _warmth(stage, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "heater"))
        self.play(FadeIn(glow), FadeIn(warm_tag), Create(rise_guide), run_time=0.6)
        self.play(*air("rise")(4.0), run_time=4.0)
        hold_for(self, self.NARRATION, "heater", during=air("rise"))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "window"))
        self.play(_open_inward(window["window"]), FadeIn(open_tag), *air("rise")(1.0), run_time=1.0)
        self.play(ReplacementTransform(rise_guide, warm_guide), *air("rise")(0.6), run_time=0.6)
        self.play(*air("warm")(4.5), run_time=4.5)
        hold_for(self, self.NARRATION, "window", during=air("warm"))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cold"))
        self.play(Create(cold_guide), FadeIn(cold_tag), *air("warm")(0.6), run_time=0.6)
        self.play(*air("warm", "cold")(6.0), FadeTransform(stage["ti"], ti_cool), run_time=6.0)
        hold_for(self, self.NARRATION, "cold", during=air("warm", "cold"))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "symbol"))
        token = _q_token("c", PASTEL_CYAN).move_to(p(-3.0, 0.95))
        self.play(ReplacementTransform(VGroup(warm_guide, cold_guide), token),
                  FadeOut(warm_tag), FadeOut(cold_tag), FadeOut(glow), *air("warm", "cold")(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "symbol", during=air("warm", "cold"))

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Radiation: surface to surface, with or without air
def _room_air(stage, n: int = 30, seed: int = 9):
    """💨 Free air particles drifting in the room, kept clear of the radiator, the occupant and the rays' labels."""
    rng = np.random.default_rng(seed)
    x0, y0, x1, y1 = RADIATOR
    pts = []
    while len(pts) < n:
        x, y = float(rng.uniform(ROOM_X0 + 0.2, WALL_X0 - 0.15)), float(rng.uniform(FLOOR_Y + 0.15, CEIL_Y - 0.15))
        if x0 - 0.15 < x < x1 + 0.15 and y < y1 + 0.15:
            continue
        if abs(x - OCCUPANT_X) < 0.5 and y < 0.45:
            continue
        if abs(x - (-3.0)) < 1.05 and abs(y - 0.95) < 0.4:
            continue
        pts.append((x, y))
    dots = VGroup(*[Dot(stage["p"](x, y), radius=0.035, color="#9AA4B1", fill_opacity=0.85) for x, y in pts])
    homes = [d.get_center() for d in dots]
    phases = rng.uniform(0, TAU, size=(n, 2))
    clock = ValueTracker(0.0)
    clock.add_updater(lambda m, dt: m.increment_value(dt))

    def drift(group):
        t = clock.get_value()
        for d, h, (a, b) in zip(group, homes, phases):
            d.move_to(h + 0.07 * np.array([np.sin(2.3 * t + a), np.cos(1.9 * t + b), 0.0]))

    dots.add_updater(drift)
    return dots, clock


class Beat4_Strahlung(Scene):
    NARRATION = [
        ("label",
         "The third way is radiation, and it is the odd one out.",
         "Der dritte Weg ist die Strahlung — und sie fällt aus der Reihe."),
        ("waves",
         "Every warm surface emits invisible infrared radiation: the radiator to the wall, the outer wall to the cold night sky.",
         "Jede warme Oberfläche sendet unsichtbare Infrarotstrahlung aus: der Heizkörper an die Wand, die Außenwand an den Nachthimmel."),
        ("vacuum",
         "Now take the air away in thought. Without air there is no convection.",
         "Nehmen wir gedanklich die Luft weg: ohne Luft gibt es keine Konvektion mehr."),
        ("still",
         "The radiation still arrives — it needs no medium. That is how the sun's heat crosses empty space to reach us.",
         "Die Strahlung kommt trotzdem an — sie braucht kein Medium. So erreicht uns auch die Sonnenwärme durch das leere Weltall."),
        ("symbol",
         "This heat flow is Q dot r.",
         "Diesen Wärmestrom nennen wir Q-Punkt-r."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Strahlung — braucht kein Medium", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "label"))
        self.play(FadeIn(caption), run_time=0.3)

        stage = _stage()
        p = stage["p"]
        routes = _route_strip(active=2)
        air, air_clock = _room_air(stage)
        air_tag = Text("Raumluft", font_size=LABEL_FONT_SIZE, color="#9AA4B1").move_to(p(-3.0, 0.95))
        inner = _ir_rays(stage, INNER_RAYS)
        outer = _ir_rays(stage, OUTER_RAYS)
        sky_tag = Text("kalter Nachthimmel", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE).move_to(p(4.5, 0.55))
        loop, _ = _loop(stage)
        loop_guide = DashedVMobject(loop.copy().set_stroke(color=PASTEL_CYAN, width=2.0, opacity=0.5), num_dashes=46)
        no_air = Text("ohne Luft\nkeine Konvektion", font_size=LABEL_FONT_SIZE, color=PASTEL_RED,
                      line_spacing=0.8).move_to(air_tag)
        no_mark = cross_mark(PASTEL_RED, size=0.16).move_to(p(-2.45, 1.52))
        both = INNER_RAYS + OUTER_RAYS

        def glow(rt, pairs=both, pulses=True):
            return _ir_glow(stage, pairs, rt, pulses=pulses)

        self.add(air_clock)
        _show_stage(self, stage, run_time=1.2)
        self.play(FadeIn(routes), FadeIn(air), FadeIn(air_tag), run_time=0.8)
        hold_for(self, self.NARRATION, "label", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0 + 0.8,
                 during=lambda rt: glow(rt, INNER_RAYS, False))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "waves"))
        self.play(LaggedStart(*[Create(r) for r in inner], lag_ratio=0.15), *glow(1.2, INNER_RAYS, False), run_time=1.2)
        self.play(*glow(1.3, INNER_RAYS), _tint_wall(stage, 0.14), run_time=1.3)
        self.play(LaggedStart(*[Create(r) for r in outer], lag_ratio=0.15), FadeIn(sky_tag), *glow(1.2, INNER_RAYS),
                  run_time=1.2)
        self.play(*glow(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "waves", during=glow)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "vacuum"))
        self.play(Create(loop_guide), *glow(0.8, pulses=False), run_time=0.8)
        air.clear_updaters()
        self.play(LaggedStart(*[FadeOut(d, scale=0.3) for d in air], lag_ratio=0.03),
                  ReplacementTransform(air_tag, no_air), *glow(1.6, pulses=False), run_time=1.6)
        self.play(Create(no_mark), loop_guide.animate.set_stroke(opacity=0.15), *glow(0.6, pulses=False), run_time=0.6)
        hold_for(self, self.NARRATION, "vacuum", during=lambda rt: glow(rt, pulses=False))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "still"))
        self.play(*glow(2.6), run_time=2.6)
        hold_for(self, self.NARRATION, "still", during=glow)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "symbol"))
        token = _q_token("r", PASTEL_YELLOW).move_to(p(-3.0, 0.95))
        self.play(ReplacementTransform(VGroup(inner, outer), token),
                  FadeOut(loop_guide), FadeOut(no_mark), FadeOut(no_air), FadeOut(sky_tag),
                  *glow(1.2, pulses=False), run_time=1.2)
        hold_for(self, self.NARRATION, "symbol", during=lambda rt: glow(rt, pulses=False))
        air_clock.clear_updaters()

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – The three ways in series through one wall
SUMMARY_SCALE = 0.86
PROFILE_U = 1.43
PROFILE_RSI, PROFILE_RSE = 0.13, 0.04


def _profile_y(theta: float) -> float:
    """🌡️ Design height of a temperature on the profile overlay (0 °C at the floor, 20 °C under the ceiling)."""
    return -0.95 + theta / THETA_I * 2.55


class Beat5_Zusammenfassung(Scene):
    NARRATION = [
        ("recap",
         "At a real exterior wall all three ways work together — one after another.",
         "An einer echten Außenwand arbeiten alle drei Wege zusammen — hintereinander."),
        ("sum",
         "Inside, convection and radiation bring the heat to the wall, the wall conducts it through, and outside wind and radiation carry it off.",
         "Innen bringen Konvektion und Strahlung die Wärme an die Wand, die Wand leitet sie durch, außen tragen Wind und Strahlung sie fort."),
        ("k", "Inside the wall only conduction works: Q dot k.", "In der Wand wirkt nur die Leitung: Q-Punkt-k."),
        ("c", "At both surfaces moving air carries the heat: Q dot c.", "An beiden Oberflächen trägt bewegte Luft die Wärme: Q-Punkt-c."),
        ("r", "Next to it the surfaces radiate: Q dot r.", "Daneben strahlen die Oberflächen: Q-Punkt-r."),
        ("profile",
         "The same heat flow passes every station. The temperature drops in steps: twenty, sixteen point three, one point one, zero degrees.",
         "Derselbe Wärmestrom läuft durch alle Stationen. Die Temperatur fällt in Stufen: 20, 16,3, 1,1 und 0 Grad."),
        ("standard",
         "DIN EN ISO 6946 merges convection and radiation at the surfaces into R si and R se; with Sigma d over lambda that gives the U-value.",
         "DIN EN ISO 6946 fasst beide Oberflächen zu R-si und R-se zusammen — mit Summe d durch Lambda ergibt das den U-Wert."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Zusammenfassung — drei Wege, ein Wärmestrom", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "recap"))
        self.play(FadeIn(caption), run_time=0.3)

        stage = _stage(scale=SUMMARY_SCALE)
        p = stage["p"]
        zone_tags = VGroup(
            Text("Konvektion + Strahlung", font_size=LABEL_FONT_SIZE, color=PASTEL_CYAN).move_to(p(-3.5, 2.20)),
            Text("Leitung", font_size=LABEL_FONT_SIZE, color=PASTEL_RED).move_to(p(0.0, 2.20)),
            Text("Konvektion + Strahlung", font_size=LABEL_FONT_SIZE, color=PASTEL_CYAN).move_to(p(3.5, 2.20)),
        )
        loop, loop_color = _loop(stage)
        loop_guide = loop.copy().set_stroke(color=PASTEL_CYAN, width=2.0, opacity=0.35).set_fill(opacity=0)
        lanes = _wind(stage)
        wind_guides = flow_guides(lanes, PASTEL_CYAN, opacity=0.28, width=1.8)
        inner = _ir_rays(stage, INNER_RAYS)
        outer = _ir_rays(stage, OUTER_RAYS)
        wall_lanes = _through(stage)
        through = flow_guides(wall_lanes, HOT, opacity=0.5, width=2.6)

        def heat(rt):
            return [_loop_flow(loop, loop_color, cycles=rt / 7.0, n=14),
                    flow_animation([(lanes, PASTEL_BLUE, PASTEL_PURPLE)], waves=6, radius=0.06, cycles=rt / 4.0),
                    flow_animation([(wall_lanes, HOT, PASTEL_BLUE)], waves=2, radius=0.06, cycles=rt / 1.6),
                    *_ir_glow(stage, INNER_RAYS + OUTER_RAYS, rt)]

        eq, eq_box, items = math_panel([
            ("ci", r"\dot{Q}_{\mathrm{c,i}}", PASTEL_CYAN), (None, "+", PASTEL_WHITE), ("ri", r"\dot{Q}_{\mathrm{r,i}}", PASTEL_YELLOW),
            (None, "=", PASTEL_WHITE), ("k", r"\dot{Q}_{\mathrm{k}}", PASTEL_RED), (None, "=", PASTEL_WHITE),
            ("ce", r"\dot{Q}_{\mathrm{c,e}}", PASTEL_CYAN), (None, "+", PASTEL_WHITE), ("re", r"\dot{Q}_{\mathrm{r,e}}", PASTEL_YELLOW),
            (None, r"\;[\mathrm{W}]", PASTEL_TEAL),
        ])

        q = PROFILE_U * (THETA_I - THETA_E)
        theta_si = THETA_I - q * PROFILE_RSI
        theta_se = THETA_E + q * PROFILE_RSE
        prof_pts = [(-6.0, THETA_I), (-1.25, THETA_I), (WALL_X0, theta_si), (WALL_X1, theta_se),
                    (1.25, THETA_E), (6.0, THETA_E)]
        profile = VMobject(color=PASTEL_WHITE, stroke_width=4)
        profile.set_points_as_corners([p(x, _profile_y(t)) for x, t in prof_pts])
        prof_dots = VGroup(*[Dot(p(x, _profile_y(t)), radius=0.06, color=PASTEL_YELLOW) for x, t in prof_pts[1:5]])
        prof_tags = VGroup(
            Text(f"{THETA_I} °C", font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE).move_to(p(-3.5, _profile_y(THETA_I) - 0.28)),
            Text(f"{theta_si:.1f} °C".replace(".", ","), font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE)
            .move_to(p(-1.75, _profile_y(theta_si) - 0.32)),
            Text(f"{theta_se:.1f} °C".replace(".", ","), font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE)
            .move_to(p(1.75, _profile_y(theta_se) + 0.32)),
            Text(f"{THETA_E} °C", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE).move_to(p(3.5, _profile_y(THETA_E) + 0.28)),
        )
        iso = _din_ref("DIN EN ISO 6946")

        prof_path = [p(x, _profile_y(t)) for x, t in prof_pts]

        def passing(rt):
            return [pulse_flashes([prof_path], HOT, repeats=max(1, int(rt / 1.8)), width=6.0)]

        _show_stage(self, stage, run_time=1.2)
        self.play(FadeIn(zone_tags, shift=DOWN * 0.1), *_warmth(stage, 0.6), run_time=0.6)
        hold_for(self, self.NARRATION, "recap", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0 + 0.6,
                 during=lambda rt: _warmth(stage, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sum"))
        self.play(Create(loop_guide), LaggedStart(*[Create(r) for r in inner], lag_ratio=0.15), run_time=1.0)
        self.play(Create(through), _tint_wall(stage), run_time=0.8)
        self.play(Create(wind_guides), LaggedStart(*[Create(r) for r in outer], lag_ratio=0.15), run_time=1.0)
        self.play(*heat(3.0), run_time=3.0)
        self.play(FadeIn(eq), Create(eq_box), *heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "sum", during=heat)

        for key, keys, visual, color in (
            ("k", ("k",), through, PASTEL_RED),
            ("c", ("ci", "ce"), VGroup(loop_guide, wind_guides), PASTEL_CYAN),
            ("r", ("ri", "re"), VGroup(inner, outer), PASTEL_YELLOW),
        ):
            rings = VGroup(*[highlight_param(items, k, color=color) for k in keys])
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(rings), Indicate(visual, color=color, scale_factor=1.04), *heat(0.7), run_time=0.7)
            hold_for(self, self.NARRATION, key, during=heat)
            self.play(FadeOut(rings), *heat(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "profile"))
        self.play(_dim(VGroup(loop_guide, wind_guides, inner, outer, through), 0.1),
                  _dim(VGroup(stage["radiator"], stage["occupant"]), 0.3), run_time=0.6)
        self.play(Create(profile), run_time=1.6)
        self.play(LaggedStart(*[AnimationGroup(FadeIn(d, scale=0.5), FadeIn(t)) for d, t in zip(prof_dots, prof_tags)],
                              lag_ratio=0.35), run_time=1.6)
        hold_for(self, self.NARRATION, "profile", during=passing)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "standard"))
        self.play(FadeIn(iso), *passing(0.5), run_time=0.5)
        self.play(Indicate(iso, color=PASTEL_TEAL, scale_factor=1.25), run_time=0.5)
        self.play(Indicate(eq, color=PASTEL_TEAL, scale_factor=1.06), *passing(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "standard", during=passing)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Three mechanisms collapse into one building-element number
class Beat6_VonWegenZuZahlen(Scene):
    NARRATION = [
        ("bridge",
         "So far we have three separate mechanisms. No engineer sizes a heating system that way.",
         "Bisher haben wir drei getrennte Mechanismen. So legt kein Ingenieur eine Heizung aus."),
        ("merge",
         "For any real building element the three are measured together and collapsed into one number that describes the whole construction.",
         "Für jedes reale Bauteil werden die drei gemeinsam erfasst und zu einer Zahl zusammengefasst, die den ganzen Aufbau beschreibt."),
        ("layers",
         "And a real wall is never one material. Plaster, masonry, insulation and render each resist the heat flow differently.",
         "Eine echte Wand ist nie ein Material: Putz, Mauerwerk, Dämmung und Außenputz bremsen den Wärmestrom unterschiedlich stark."),
        ("plan",
         "So the plan is two steps: first add up how strongly the layers resist, then turn that resistance into the number we actually use.",
         "Der Plan hat zwei Schritte: erst addieren, wie stark die Schichten bremsen, dann diesen Widerstand in die Kennzahl umrechnen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Abschnitt 1.2 — Von drei Wegen zu einem Kennwert", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "bridge"))
        self.play(FadeIn(caption), run_time=0.3)

        tokens = VGroup(
            _q_token("k", PASTEL_RED),
            _q_token("c", PASTEL_CYAN),
            _q_token("r", PASTEL_YELLOW),
        ).arrange(DOWN, buff=0.62)
        tokens.move_to(np.array([-4.9, 1.05, 0.0]))

        funnel = VGroup(*[
            _flow_arrow(
                tok.get_right() + RIGHT * 0.18,
                np.array([-1.85, 1.05, 0.0]),
                PASTEL_WHITE, width=3,
            )
            for tok in tokens
        ])
        one = chip("ein Bauteil-Kennwert", PASTEL_TEAL)
        one.move_to(np.array([-0.05, 1.05, 0.0]))

        stack = VGroup(
            _layer(0.34, 1.9, PASTEL_WHITE, 0.20, "Putz"),
            _layer(1.05, 1.9, PASTEL_ORANGE, 0.22, "Mauerwerk"),
            _layer(0.95, 1.9, PASTEL_CYAN, 0.22, "Dämmung"),
            _layer(0.26, 1.9, PASTEL_WHITE, 0.20, "Außenputz"),
        ).arrange(RIGHT, buff=0.0, aligned_edge=UP)
        stack.move_to(np.array([2.55, -0.35, 0.0]))
        # A legend beside the stack, not a tag under each column: the thinnest
        # column (0.26 units) can never fit "Außenputz" (≈1.2 units) beneath
        # itself without overlapping its neighbour.
        legend = _layer_legend([
            ("Putz", PASTEL_WHITE), ("Mauerwerk", PASTEL_ORANGE),
            ("Dämmung", PASTEL_CYAN), ("Außenputz", PASTEL_WHITE),
        ])
        legend.next_to(stack, DOWN, buff=0.30).set_x(stack.get_x())

        steps = VGroup(
            chip("1 Schichten addieren → R", PASTEL_YELLOW, font_size=LABEL_FONT_SIZE),
            chip("2 R umrechnen → U", PASTEL_ORANGE, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.24)
        steps.move_to(np.array([-3.85, -1.60, 0.0]))

        fit_band(VGroup(tokens, funnel, one, stack, legend, steps))

        hold_for(self, self.NARRATION, "bridge", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(LaggedStart(*[FadeIn(t, shift=RIGHT * 0.2) for t in tokens], lag_ratio=0.18), run_time=1.3)
        hold_for(self, self.NARRATION, "bridge", used=1.3 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "merge"))
        self.play(LaggedStart(*[GrowArrow(a) for a in funnel], lag_ratio=0.15), run_time=1.2)
        self.play(FadeIn(one, scale=0.85), run_time=0.9)
        hold_for(self, self.NARRATION, "merge", used=1.2 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "layers"))
        self.play(LaggedStart(*[FadeIn(l, shift=DOWN * 0.15) for l in stack], lag_ratio=0.2), run_time=1.8)
        self.play(FadeIn(legend, shift=DOWN * 0.1), run_time=0.7)
        hold_for(self, self.NARRATION, "layers", used=1.8 + 0.7 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "plan"))
        self.play(LaggedStart(*[FadeIn(s, shift=UP * 0.12) for s in steps], lag_ratio=0.25), run_time=1.4)
        hold_for(self, self.NARRATION, "plan", used=1.4 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 – Thermal resistance R = d / lambda
#region Beat7 material data
B7_UNIT_M = 0.15
B7_D0 = 0.24
B7_LAM_KS = 1.0
B7_LAM_INS = 0.035


def _lam_digits(lam: float) -> int:
    """🔢 Decimal places for λ — three in the insulation range, one for masonry."""
    return 3 if lam < 0.1 else (2 if lam < 0.95 else 1)


def _r_digits(r: float) -> int:
    """🎚️ Decimal places for a thermal resistance — two below ten, one above."""
    return 2 if r < 9.95 else 1
#endregion


class Beat7_Waermedurchlasswiderstand(Scene):
    NARRATION = [
        ("intro",
         "Step one: how strongly does a single layer resist the heat flow? Watch the temperature fall across it.",
         "Schritt eins: wie stark bremst eine einzelne Schicht den Wärmestrom? Sehen wir, wie die Temperatur über sie abfällt."),
        ("gradient",
         "For a single layer the drop from twenty to zero is always the same distance. A poorly insulating material lets more heat through that same drop.",
         "Über eine Schicht fällt die Temperatur von zwanzig auf null. Ein schlecht dämmendes Material lässt dabei mehr Wärme durch."),
        ("formula",
         "That resisting power is the thermal resistance R: the thickness d divided by the conductivity lambda.",
         "Diese Bremswirkung ist der Wärmedurchlasswiderstand R: die Dicke d geteilt durch die Leitfähigkeit Lambda."),
        ("d",
         "Make the layer thicker and the same temperature drop is spread over more distance. R rises, and the heat flow falls.",
         "Wird die Schicht dicker, verteilt sich derselbe Temperaturabfall auf mehr Strecke. R steigt, der Wärmestrom sinkt."),
        ("lam",
         "Lambda is the material itself. Swapping masonry for insulation cuts lambda roughly thirtyfold, so R jumps even at the same thickness.",
         "Lambda ist das Material selbst. Mauerwerk gegen Dämmung senkt Lambda rund dreißigmal — R springt hoch, bei gleicher Dicke."),
        ("sum",
         "A real construction just adds the layers up: R total is the sum of every layer's own resistance.",
         "Ein realer Aufbau addiert einfach: R-gesamt ist die Summe der Widerstände aller Schichten."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmedurchlasswiderstand R", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        y_hi, y_lo = 1.70, 0.40
        cy, layer_h = 1.05, 1.90
        x0 = -2.60 - B7_D0 / B7_UNIT_M / 2
        y_dim = cy - layer_h / 2 - 0.28

        d_m, lam, ins = ValueTracker(B7_D0), ValueTracker(B7_LAM_KS), ValueTracker(0.0)
        lam_on, d_on, r_on, g_on = (ValueTracker(0.0) for _ in range(4))

        def x1():
            return x0 + d_m.get_value() / B7_UNIT_M

        def r_val():
            return d_m.get_value() / lam.get_value()

        def mat_color():
            return interpolate_color(ManimColor(PASTEL_ORANGE), ManimColor(PASTEL_CYAN), ins.get_value())

        def fit_layer(m):
            m.stretch_to_fit_width(x1() - x0)
            m.move_to(np.array([(x0 + x1()) / 2, cy, 0.0]))
            m.set_stroke(mat_color())
            m.set_fill(mat_color(), opacity=0.20)

        layer = Rectangle(width=x1() - x0, height=layer_h, stroke_width=2.5)
        fit_layer(layer)
        layer.add_updater(fit_layer)

        mat = Text("Kalksandstein", font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE)
        mat.next_to(np.array([x0, cy + layer_h / 2, 0.0]), UP, buff=0.16, aligned_edge=LEFT)
        ins_tag = Text("Dämmung", font_size=LABEL_FONT_SIZE, color=PASTEL_CYAN)
        ins_tag.next_to(np.array([x0, cy + layer_h / 2, 0.0]), UP, buff=0.16, aligned_edge=LEFT)
        lam_live = _live(
            lambda: (rf"\lambda = {de_num(lam_on.get_value() * lam.get_value(), _lam_digits(lam.get_value()))}"
                     r"\,\mathrm{W/(m\,K)}"),
            np.array([mat.get_right()[0] + 0.32, mat.get_bottom()[1], 0.0]),
            size=LABEL_FONT_SIZE, color=PASTEL_ORANGE,
        )

        warm_face = Line(
            np.array([x0 - 1.5, y_hi, 0.0]), np.array([x0, y_hi, 0.0]),
            color=PASTEL_ORANGE, stroke_width=3,
        )
        cold_face = Line(np.array([x1(), y_lo, 0.0]), np.array([x1() + 1.5, y_lo, 0.0]), color=PASTEL_BLUE, stroke_width=3)
        cold_face.add_updater(lambda m: m.put_start_and_end_on(
            np.array([x1(), y_lo, 0.0]), np.array([x1() + 1.5, y_lo, 0.0])))
        grad = Line(np.array([x0, y_hi, 0.0]), np.array([x1(), y_lo, 0.0]), color=PASTEL_YELLOW, stroke_width=4)
        grad.add_updater(lambda m: m.put_start_and_end_on(
            np.array([x0, y_hi, 0.0]), np.array([x1(), y_lo, 0.0])))
        ti = Text("20 °C", font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE)
        ti.next_to(warm_face.get_start(), UP, buff=0.12)
        te = Text("0 °C", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE)
        te.add_updater(lambda m: m.next_to(np.array([x1() + 1.5, y_lo, 0.0]), DOWN, buff=0.12))

        d_brace = always_redraw(lambda: dim_arrow(
            np.array([x0, y_dim, 0.0]), np.array([x1(), y_dim, 0.0]), color=PASTEL_CYAN,
        ))
        d_live = _live(
            lambda: rf"d = {de_num(d_on.get_value() * d_m.get_value(), 2)}\,\mathrm{{m}}",
            lambda: np.array([(x0 + x1()) / 2, y_dim - 0.42, 0.0]),
            size=BODY_FONT_SIZE, color=PASTEL_CYAN, edge="center",
        )

        r_gauge = meter("R", length=2.0, thickness=0.52, color=PASTEL_YELLOW)
        r_gauge["group"].move_to(np.array([2.05, 1.05, 0.0]))
        q_gauge = meter("Wärmestrom", length=2.0, thickness=0.52, color=PASTEL_RED)
        q_gauge["group"].move_to(np.array([4.35, 1.05, 0.0]))
        set_meter(r_gauge, 0.0)
        set_meter(q_gauge, 0.0)
        r_gauge["fill"].add_updater(lambda m: set_meter(r_gauge, g_on.get_value() * r_val() / (r_val() + 1.0)))
        q_gauge["fill"].add_updater(lambda m: set_meter(q_gauge, g_on.get_value() * min(1.0, 0.9 * B7_D0 / r_val())))
        r_live = _live(
            lambda: (rf"R = \frac{{d}}{{\lambda}} = {de_num(r_on.get_value() * r_val(), _r_digits(r_val()))}"
                     r"\,\mathrm{m^{2}K/W}"),
            np.array([3.20, -0.62, 0.0]),
            size=BODY_FONT_SIZE, color=PASTEL_YELLOW, edge="center",
        )

        eq, eq_box, items = math_panel([
            ("r", "R", PASTEL_YELLOW), (None, "=", PASTEL_WHITE),
            ("frac", r"\frac{\textcolor{#66FCF1}{d}}{\textcolor{#FFAAA5}{\lambda}}", PASTEL_WHITE),
            (None, r"\;[\mathrm{m^{2}K/W}]", PASTEL_TEAL),
        ], color=PASTEL_YELLOW)
        frac = items["frac"][0]
        parts = {"d": frac[0], "lam": frac[2]}

        sum_note = math_label(
            r"R_{\mathrm{ges}} = R_{1} + R_{2} + R_{3} + \ldots",
            np.array([x0, -1.10, 0.0]), size=BODY_FONT_SIZE, color=PASTEL_TEAL, edge="left",
        )
        heat_lanes = [_path([(x0 - 1.2, y), (x0 + 2 * B7_D0 / B7_UNIT_M + 1.0, y)]) for y in (0.78, 1.05, 1.32)]

        def heat(rt):
            q = B7_D0 / B7_LAM_KS / r_val()
            lanes = heat_lanes if q > 0.3 else heat_lanes[1:2]
            return [flow_animation([(lanes, HOT, PASTEL_BLUE)], waves=max(1, round(4 * q)), radius=0.06,
                                   cycles=rt / 2.8 * max(q, 0.35))]

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.add(lam_live)
        self.play(Create(layer), FadeIn(mat), lam_on.animate.set_value(1.0), run_time=1.1)
        self.play(Create(warm_face), Create(cold_face), FadeIn(ti), FadeIn(te), run_time=1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gradient"))
        self.play(Create(grad), run_time=1.0)
        self.play(
            *[FadeIn(g[k]) for g in (r_gauge, q_gauge) for k in ("track", "label")], *heat(0.5),
            run_time=0.5,
        )
        self.add(r_gauge["fill"], q_gauge["fill"])
        self.play(g_on.animate.set_value(1.0), *heat(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "gradient", during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.add(d_live)
        self.play(Create(d_brace), d_on.animate.set_value(1.0), *heat(0.9), run_time=0.9)
        self.play(FadeIn(eq), Create(eq_box), *heat(1.0), run_time=1.0)
        self.add(r_live)
        self.play(r_on.animate.set_value(1.0), *heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "formula", during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "d"))
        ring_d = highlight_param(parts, "d", color=PASTEL_CYAN)
        self.play(Create(ring_d), *heat(0.4), run_time=0.4)
        self.play(d_m.animate.set_value(2 * B7_D0), *heat(1.8), run_time=1.8)
        hold_for(self, self.NARRATION, "d", during=heat)
        self.play(FadeOut(ring_d), *heat(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lam"))
        ring_l = highlight_param(parts, "lam", color=PASTEL_ORANGE)
        self.play(Create(ring_l), *heat(0.4), run_time=0.4)
        self.play(
            ReplacementTransform(mat, ins_tag),
            lam.animate.set_value(B7_LAM_INS), ins.animate.set_value(1.0), *heat(1.9),
            run_time=1.9,
        )
        self.play(
            Indicate(grad, color=PASTEL_YELLOW, scale_factor=1.0),
            Indicate(q_gauge["track"], color=PASTEL_RED, scale_factor=1.05), *heat(0.7),
            run_time=0.7,
        )
        hold_for(self, self.NARRATION, "lam", during=heat)
        self.play(FadeOut(ring_l), *heat(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sum"))
        self.play(FadeIn(sum_note, shift=UP * 0.12), *heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "sum", during=heat)

        for mob in (layer, cold_face, grad, te, r_gauge["fill"], q_gauge["fill"]):
            mob.clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – U = 1 / R_ges, the number the standard actually reports
#region Beat8 wall build-ups
R_SI, R_SE = 0.13, 0.04
WALL_1960 = (
    ("Innenputz", 0.015, 0.70, PASTEL_WHITE),
    ("Vollziegel", 0.365, 0.75, PASTEL_ORANGE),
    ("Außenputz", 0.020, 0.87, PASTEL_WHITE),
)
WALL_SANIERT = WALL_1960[:2] + (("Dämmung", 0.20, 0.035, PASTEL_CYAN),) + WALL_1960[2:]
B8_UNIT_PER_M = 3.6
B8_MIN_W = 0.06
B8_FILM_W = 0.05
B8_WALL_H = 1.85
B8_LEAK_MAX = 1.10


def _wall_terms(layers) -> list[float]:
    """🧮 Resistances in heat-flow order, R_si, every d/λ, R_se — rounded to the shown two decimals (DIN EN ISO 6946)."""
    return [round(r, 2) for r in (R_SI, *[d / lam for _n, d, lam, _c in layers], R_SE)]


def _wall_section(center, layers):
    """🏚️ Wall section drawn to scale — films, then one column per layer at its real thickness."""
    film = lambda: Rectangle(
        width=B8_FILM_W, height=B8_WALL_H, stroke_width=0, fill_color=PASTEL_GREEN, fill_opacity=0.55,
    )
    cols = [
        Rectangle(
            width=max(B8_MIN_W, d * B8_UNIT_PER_M), height=B8_WALL_H, color=c, stroke_width=2,
            fill_color=c, fill_opacity=0.22,
        )
        for _n, d, _lam, c in layers
    ]
    return VGroup(film(), *cols, film()).arrange(RIGHT, buff=0.0).move_to(center)
#endregion


class Beat8_UWert(Scene):
    NARRATION = [
        ("intro",
         "Step two. Resistance is useful, but engineers quote the opposite: how easily heat gets through.",
         "Schritt zwei. Der Widerstand ist nützlich, aber angegeben wird das Gegenteil: wie leicht Wärme hindurchkommt."),
        ("total",
         "First the full resistance of the element: the two thin air films on the surfaces, plus every layer in between.",
         "Zuerst der gesamte Widerstand des Bauteils: die beiden dünnen Luftschichten an den Oberflächen plus alle Schichten dazwischen."),
        ("flip",
         "Invert that total and you get the U value: the watts crossing one square metre for each kelvin of temperature difference.",
         "Kehrt man diese Summe um, erhält man den U-Wert: die Watt, die pro Quadratmeter und pro Kelvin Unterschied hindurchgehen."),
        ("old",
         "An uninsulated solid wall from nineteen-sixty has a U of about one point four three. It loses a lot of heat.",
         "Eine ungedämmte Massivwand von 1960 hat ein U von etwa 1,43 — sie verliert viel Wärme."),
        ("new",
         "The same wall with twenty centimetres of insulation reaches about zero point one six: roughly nine times less heat flow.",
         "Dieselbe Wand mit zwanzig Zentimetern Dämmung erreicht etwa 0,16 — rund neunmal weniger Wärmestrom."),
        ("meaning",
         "Low U is the goal. Every part of the building envelope is judged by this one number.",
         "Ein kleines U ist das Ziel. Jedes Bauteil der Gebäudehülle wird an dieser einen Zahl gemessen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("U-Wert — wie leicht Wärme hindurchgeht", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        def _card(cx, layers, tag, tag_color):
            stack = _wall_section(np.array([cx, 1.05, 0.0]), layers)
            label = Text(tag, font_size=LABEL_FONT_SIZE, color=tag_color)
            label.next_to(stack, UP, buff=0.18)
            terms = _wall_terms(layers)
            on = [ValueTracker(0.0) for _ in terms]
            u_on = ValueTracker(0.0)

            def r_now():
                return sum(t.get_value() * r for t, r in zip(on, terms))

            def u_now():
                return u_on.get_value() / sum(terms)

            r_src = lambda: (
                r"R_{\mathrm{ges}} = "
                + " + ".join(de_num(t.get_value() * r, 2) for t, r in zip(on, terms))
                + rf" = {de_num(r_now(), 2)}\,\mathrm{{m^{{2}}K/W}}"
            )
            u_src = lambda: rf"U = \frac{{1}}{{R_{{\mathrm{{ges}}}}}} = {de_num(u_now(), 2)}\,\mathrm{{W/(m^{{2}}K)}}"
            r_live = _live(r_src, np.array([cx, -0.22, 0.0]), size=LABEL_FONT_SIZE, color=PASTEL_TEAL, edge="center")
            u_live = _live(u_src, np.array([cx, -0.82, 0.0]), size=BODY_FONT_SIZE, color=tag_color, edge="center")
            right = stack.get_right()[0] + 0.06
            share = 1.0 / sum(terms) / U_1960
            lanes = [_path([(right, 1.05 + dy), (right + B8_LEAK_MAX, 1.05 + dy)]) for dy in (-0.62, -0.21, 0.20, 0.61)]
            lanes = lanes if share > 0.5 else lanes[1:3]

            def leak(rt):
                return [flow_animation([(lanes, tag_color, PASTEL_BLUE)], waves=max(1, round(4 * share)), radius=0.06,
                                       cycles=rt / 1.4 * max(share, 0.4))]

            return {"stack": stack, "label": label, "on": on, "u_on": u_on, "r_live": r_live, "u_live": u_live,
                    "guides": flow_guides(lanes, tag_color, opacity=0.3), "leak": leak, "card": VGroup(stack, label)}

        U_1960 = 1.0 / sum(_wall_terms(WALL_1960))
        old = _card(-3.45, WALL_1960, "Altbau 1960 — ungedämmt", PASTEL_RED)
        new = _card(3.45, WALL_SANIERT, "saniert — 20 cm Dämmung", PASTEL_CYAN)

        films = VGroup(
            math_label(r"R_{\mathrm{si}}", size=LABEL_FONT_SIZE, color=PASTEL_GREEN).next_to(old["stack"], LEFT, buff=0.14),
            math_label(r"R_{\mathrm{se}}", size=LABEL_FONT_SIZE, color=PASTEL_GREEN).next_to(old["stack"], RIGHT, buff=0.14),
        )

        r_eq, r_box, r_items = math_panel([
            ("rges", r"R_{\mathrm{ges}}", PASTEL_TEAL), (None, "=", PASTEL_WHITE),
            ("rsi", r"R_{\mathrm{si}}", PASTEL_GREEN), (None, "+", PASTEL_WHITE),
            (None, r"\Sigma\,\frac{d}{\lambda}", PASTEL_YELLOW), (None, "+", PASTEL_WHITE),
            ("rse", r"R_{\mathrm{se}}", PASTEL_GREEN),
            (None, r"\;[\mathrm{m^{2}K/W}]", PASTEL_TEAL),
        ], color=PASTEL_TEAL)
        u_eq, u_box, u_items = math_panel([
            ("u", "U", PASTEL_ORANGE), (None, "=", PASTEL_WHITE),
            (None, r"\frac{1}{R_{\mathrm{ges}}}", PASTEL_TEAL),
            (None, r"\;[\mathrm{W/(m^{2}K)}]", PASTEL_TEAL),
        ], color=PASTEL_ORANGE)

        def count_layers(card, step, during=lambda rt: []):
            for col, tracker in zip(card["stack"], card["on"]):
                self.play(
                    Indicate(col, color=PASTEL_YELLOW, scale_factor=1.0),
                    tracker.animate.set_value(1.0), *during(step),
                    run_time=step,
                )

        def both(rt):
            return [*old["leak"](rt), *new["leak"](rt)]

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(FadeIn(old["card"]), run_time=1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "total"))
        self.play(FadeIn(films), FadeIn(r_eq), Create(r_box), run_time=1.0)
        self.add(old["r_live"])
        count_layers(old, 0.5)
        hold_for(self, self.NARRATION, "total", used=1.0 + 5 * 0.5 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flip"))
        self.play(
            ReplacementTransform(r_eq, u_eq), ReplacementTransform(r_box, u_box),
            FadeOut(films),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "flip", used=1.3 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "old"))
        self.add(old["u_live"])
        self.play(old["u_on"].animate.set_value(1.0), Create(old["guides"]), run_time=1.8)
        hold_for(self, self.NARRATION, "old", during=old["leak"])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "new"))
        self.play(FadeIn(new["card"]), *old["leak"](0.8), run_time=0.8)
        self.add(new["r_live"])
        count_layers(new, 0.35, old["leak"])
        self.add(new["u_live"])
        self.play(new["u_on"].animate.set_value(1.0), Create(new["guides"]), *old["leak"](1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "new", during=both)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meaning"))
        ring = highlight_param(u_items, "u", color=PASTEL_ORANGE)
        self.play(Create(ring), Indicate(new["u_live"], color=PASTEL_CYAN, scale_factor=1.12), *both(1.0),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "meaning", during=both)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat9 – Q = U · A · Δθ, and what each factor actually does
#region Beat9 example facade
B9_UNIT_PER_M = 0.25
B9_W, B9_H = 12.0, 6.0
B9_U_OLD, B9_U_NEW = 1.43, 0.16
B9_Q_FULL = 4500.0
#endregion


class Beat9_WaermestromFormel(Scene):
    NARRATION = [
        ("intro",
         "Now we can finally count watts. The heat flow through any building element takes three factors.",
         "Jetzt können wir endlich Watt zählen. Der Wärmestrom durch ein Bauteil braucht drei Faktoren."),
        ("formula",
         "Heat flow equals the U value, times the area, times the temperature difference.",
         "Wärmestrom gleich U-Wert mal Fläche mal Temperaturdifferenz."),
        ("u",
         "U is the quality of the construction — the one number we just built out of the layers.",
         "U ist die Qualität des Aufbaus — die Zahl, die wir gerade aus den Schichten gebaut haben."),
        ("a",
         "A is how much of that construction there is. Double the wall area and you double the loss.",
         "A ist, wie viel von diesem Aufbau vorhanden ist. Doppelte Wandfläche, doppelter Verlust."),
        ("dt",
         "Delta theta is the driver from the very first beat. Double the temperature difference and the loss doubles again.",
         "Δθ ist der Antrieb aus dem ersten Beat. Doppelte Temperaturdifferenz, wieder doppelter Verlust."),
        ("power",
         "U is the main construction lever. Compactness and the indoor setpoint also matter — but insulation is the factor you choose in the wall itself.",
         "U ist der Haupt-Hebel am Aufbau. Kompaktheit und Innentemperatur zählen auch — aber Dämmung ist der Faktor, den man in der Wand selbst wählt."),
        ("outro",
         "For this wall the answer is about two thousand watts, the same as a full-power electric heater running non-stop.",
         "Für diese Wand sind das rund zweitausend Watt — so viel wie ein Heizlüfter auf voller Stufe im Dauerbetrieb."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmestrom durch ein Bauteil", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        fl, fcy = -5.80, 1.00
        fh = B9_H * B9_UNIT_PER_M
        w_geo, w_on, h_on = ValueTracker(B9_W), ValueTracker(0.0), ValueTracker(0.0)
        u_t, dt_t = ValueTracker(0.0), ValueTracker(0.0)

        def fr():
            return fl + w_geo.get_value() * B9_UNIT_PER_M

        def area():
            return w_on.get_value() * w_geo.get_value() * h_on.get_value() * B9_H

        def q_now():
            return u_t.get_value() * area() * dt_t.get_value()

        def fit_facade(m):
            m.stretch_to_fit_width(fr() - fl)
            m.move_to(np.array([(fl + fr()) / 2, fcy, 0.0]))

        facade = Rectangle(
            width=fr() - fl, height=fh, color=PASTEL_WHITE, stroke_width=2.5,
            fill_color=PASTEL_TEAL, fill_opacity=0.12,
        )
        fit_facade(facade)
        facade.add_updater(fit_facade)

        w_dim = always_redraw(lambda: dim_arrow(
            np.array([fl, fcy - fh / 2 - 0.25, 0.0]), np.array([fr(), fcy - fh / 2 - 0.25, 0.0]), color=PASTEL_CYAN,
        ))
        h_dim = dim_arrow(
            np.array([fl - 0.25, fcy - fh / 2, 0.0]), np.array([fl - 0.25, fcy + fh / 2, 0.0]), color=PASTEL_CYAN,
        )
        w_live = _live(
            lambda: rf"{de_num(w_on.get_value() * w_geo.get_value())}\,\mathrm{{m}}",
            lambda: np.array([(fl + fr()) / 2, fcy - fh / 2 - 0.62, 0.0]),
            size=LABEL_FONT_SIZE, color=PASTEL_CYAN, edge="center",
        )
        h_live = _live(
            lambda: rf"{de_num(h_on.get_value() * B9_H)}\,\mathrm{{m}}",
            np.array([fl - 0.42, fcy - 0.08, 0.0]),
            size=LABEL_FONT_SIZE, color=PASTEL_CYAN, edge="right",
        )
        a_live = _live(
            lambda: rf"A = {de_num(area())}\,\mathrm{{m^{{2}}}}",
            lambda: np.array([(fl + fr()) / 2, fcy + 0.22, 0.0]),
            size=BODY_FONT_SIZE, color=PASTEL_CYAN, edge="center",
        )
        u_live = _live(
            lambda: rf"U = {de_num(u_t.get_value(), 2)}\,\mathrm{{W/(m^{{2}}K)}}",
            lambda: np.array([(fl + fr()) / 2, fcy - 0.40, 0.0]),
            size=LABEL_FONT_SIZE, color=PASTEL_ORANGE, edge="center",
        )

        ti = Text("innen 20 °C", font_size=LABEL_FONT_SIZE, color=PASTEL_ORANGE)
        ti.next_to(np.array([fl, fcy + fh / 2, 0.0]), UP, buff=0.16, aligned_edge=LEFT)
        te = Text("außen 0 °C", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE)
        te.move_to(np.array([fl, fcy - fh / 2 - 1.10, 0.0]), aligned_edge=LEFT)
        te_cold = Text("außen −20 °C", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE).move_to(te, aligned_edge=LEFT)
        te_back = Text("außen 0 °C", font_size=LABEL_FONT_SIZE, color=PASTEL_BLUE).move_to(te, aligned_edge=LEFT)
        dt_live = _live(
            lambda: _dtheta_src(dt_t.get_value()),
            np.array([te_cold.get_right()[0] + 0.40, te.get_bottom()[1], 0.0]),
            size=LABEL_FONT_SIZE, color=PASTEL_BLUE,
        )

        q_gauge = meter("Wärmestrom", length=2.2, thickness=0.55, color=PASTEL_RED)
        q_gauge["group"].move_to(np.array([1.60, 0.95, 0.0]))
        set_meter(q_gauge, 0.0)
        q_gauge["fill"].add_updater(lambda m: set_meter(q_gauge, min(1.0, q_now() / B9_Q_FULL)))
        q_live = _live(
            lambda: rf"\dot{{Q}} = U \cdot A \cdot \Delta\theta = {de_num(q_now())}\,\mathrm{{W}}",
            np.array([1.60, -0.62, 0.0]),
            size=BODY_FONT_SIZE, color=PASTEL_RED, edge="center",
        )

        eq, eq_box, items = math_panel([
            ("q", r"\dot{Q}", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("u", "U", PASTEL_ORANGE), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", PASTEL_CYAN), (None, r"\cdot", PASTEL_WHITE),
            ("dt", r"\Delta\theta", PASTEL_BLUE),
            (None, r"\;[\mathrm{W}]", PASTEL_TEAL),
        ])

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(facade), FadeIn(ti), FadeIn(te), run_time=1.0)
        self.add(w_live, h_live, a_live)
        self.play(
            Create(w_dim), Create(h_dim),
            w_on.animate.set_value(1.0), h_on.animate.set_value(1.0),
            run_time=1.4,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq), Create(eq_box), FadeIn(q_gauge["track"]), FadeIn(q_gauge["label"]), run_time=1.0)
        self.add(q_gauge["fill"], u_live, dt_live, q_live)
        self.play(u_t.animate.set_value(B9_U_OLD), run_time=0.9)
        self.play(dt_t.animate.set_value(THETA_I - THETA_E), run_time=0.9)
        hold_for(self, self.NARRATION, "formula", used=1.0 + 0.9 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "u"))
        ring_u = highlight_param(items, "u", color=PASTEL_ORANGE)
        self.play(Create(ring_u), Indicate(facade, color=PASTEL_ORANGE), run_time=0.9)
        hold_for(self, self.NARRATION, "u", used=0.9 + 0.35)
        self.play(FadeOut(ring_u), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "a"))
        ring_a = highlight_param(items, "a", color=PASTEL_CYAN)
        self.play(Create(ring_a), run_time=0.4)
        self.play(w_geo.animate.set_value(2 * B9_W), run_time=1.5)
        hold_for(self, self.NARRATION, "a", used=0.4 + 1.5 + 0.35)
        self.play(w_geo.animate.set_value(B9_W), FadeOut(ring_a), run_time=1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "dt"))
        ring_dt = highlight_param(items, "dt", color=PASTEL_BLUE)
        self.play(Create(ring_dt), run_time=0.4)
        self.play(
            ReplacementTransform(te, te_cold),
            dt_t.animate.set_value(2 * (THETA_I - THETA_E)),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "dt", used=0.4 + 1.4 + 0.35)
        self.play(
            ReplacementTransform(te_cold, te_back),
            dt_t.animate.set_value(THETA_I - THETA_E),
            FadeOut(ring_dt),
            run_time=1.0,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "power"))
        lever = chip("Hebel in der Wand: U dämmen", PASTEL_GREEN, font_size=LABEL_FONT_SIZE)
        lever.move_to(np.array([fl + B9_W * B9_UNIT_PER_M / 2, -1.30, 0.0]))
        self.play(
            Indicate(items["u"], color=PASTEL_GREEN, scale_factor=1.25),
            FadeIn(lever, shift=UP * 0.12),
            run_time=1.1,
        )
        self.play(
            facade.animate.set_fill(PASTEL_CYAN, opacity=0.22),
            u_t.animate.set_value(B9_U_NEW),
            run_time=1.5,
        )
        hold_for(self, self.NARRATION, "power", used=1.1 + 1.5 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        anchor = watt_anchor(2000, compare="heater", title="ungedämmt", color=PASTEL_RED)
        anchor.scale(0.68).move_to(np.array([5.20, 0.60, 0.0]))
        self.play(FadeIn(anchor, shift=LEFT * 0.15), run_time=1.1)
        hold_for(self, self.NARRATION, "outro", used=1.1 + 0.35)

        for mob in (facade, q_gauge["fill"]):
            mob.clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
