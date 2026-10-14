"""☀️ Heating Module 5 — Solarer Wärmegewinn.

One south window carries the whole chain Φ_solar = G · A · F_f · g · F_sh:
every factor is read from the drawing (dimension arrows, frame face, counted
ray packets, sunlit glass height) and counts up from zero while it grows.
Formulas and units are typeset with the ``math_*`` helpers from
``manim_visuals``; German ``caption_bar`` / ``hold_for`` keep VO sync. Houses,
rooms, sun, moon, straight rays, light pulses and heat ripples are the Physical
Fundamentals glyphs, so light and heat keep moving while each subtitle is read.
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
    apply_scene_style, scene_title, play_scene_title, TITLE_RUN_TIME,
    beat_subtitle, BEAT_SUBTITLE_FADE,
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    PASTEL_WHITE, PASTEL_CYAN, PASTEL_TEAL, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_RED, PASTEL_BLUE, PASTEL_GREEN,
    highlight_param, caption_bar, swap_caption, hold_for, subtitle_text,
    radiation_ray, set_vo_language, dim_arrow,
    math_text, math_row, math_readout, math_panel, math_label, de_num,
    house_section, room_section, sun_glyph, moon_glyph, sun_rays, shine, ripples, pulse_flashes,
    smooth_path, flow_guides, flow_animation,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 5: Solarer Wärmegewinn"

COLOR_G = PASTEL_YELLOW
COLOR_A = PASTEL_CYAN
COLOR_WIN = PASTEL_CYAN
COLOR_FRAME = "#94A3B8"
COLOR_FF = PASTEL_ORANGE
COLOR_GVAL = PASTEL_GREEN
COLOR_REFL = "#D9DEE6"
COLOR_GLASS_HEAT = PASTEL_RED
COLOR_FSH = "#95A5A6"
COLOR_HEAT = PASTEL_ORANGE
COLOR_SLAB = "#737373"
COLOR_MASS = PASTEL_CYAN
COLOR_C = PASTEL_GREEN

#region Example window
# The one south window of the shared example house (Modul 2/3/5, final):
# 1,2 m × 1,6 m, clear winter day G = 400 W/m², glass share 0,70 from the
# frame face width, g = 0,60 from the energy split at the glass, F_sh from the sunlit
# glass height under an overhang sized for 0,80 at the winter sun.
WIN_W, WIN_H = 1.2, 1.6
WIN_A = WIN_W * WIN_H
G_WINTER = 400.0
FF_GLASS = 0.70
FRAME_FACE = ((WIN_W + WIN_H) - np.sqrt((WIN_W + WIN_H) ** 2 - 4 * WIN_W * WIN_H * (1 - FF_GLASS))) / 4
SHARE_R, SHARE_T, SHARE_IN, SHARE_OUT = 0.20, 0.50, 0.10, 0.20
SHARE_ABS = SHARE_IN + SHARE_OUT
G_VALUE = SHARE_T + SHARE_IN
ROOM_DEPTH, ROOM_H, SILL_H = 5.0, 2.6, 0.6
ELEV_SUMMER, ELEV_WINTER = 60.0, 20.0
ROOF_OVERHANG = 1.07
FSH_WINTER = 0.80
AWNING_DEPTH = (1 - FSH_WINTER) * WIN_H / np.tan(np.radians(ELEV_WINTER))
FSH_SUMMER = max(0.0, 1 - AWNING_DEPTH * np.tan(np.radians(ELEV_SUMMER)) / WIN_H)
PHI_GA = G_WINTER * WIN_A
PHI_FF = PHI_GA * FF_GLASS
PHI_G = PHI_FF * G_VALUE
PHI_SOLAR = PHI_G * FSH_WINTER
SLAB_L, SLAB_B, SLAB_D = 5.0, 4.0, 0.20
RHO_BETON, C_BETON, DT_SLAB = 2400.0, 1000.0, 2.0
#endregion


#region Shared
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in the other Heating
    modules. Module 5's solar-gain chain — Φ_solar = G · A · F_f · g · F_sh —
    is the DIN V 18599-2 method; the thermal-mass beats cite DIN EN ISO 13786.
    Added in absolute frame coordinates; the g-value beat cites DIN EN 410 next to it.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=PASTEL_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Physical Fundamentals glyphs
# Sun, rays, light pulses and heat ripples come from the Physical Fundamentals
# vocabulary in ``manim_visuals``; these wrappers only fix the module's size,
# pastel colours and pulse pacing.
SUN_SCALE = 0.42


def _sun(pos):
    """🌞 Physical Fundamentals sun at the size every solar beat uses."""
    return sun_glyph(np.array(pos, dtype=float), PASTEL_YELLOW).scale(SUN_SCALE)


def _soft_ray(start, end, *, width: float = 2.2, opacity: float = 0.85, fade: bool = True):
    """☀️ Straight sun ray that fades in from transparent at its sun-side end."""
    ray = radiation_ray(start, end, color=COLOR_G, stroke_width=width)
    return ray.set_stroke(opacity=[0.0, opacity, opacity] if fade else opacity)


def _parallel_starts(sun_c, hits, gap: float = 0.5):
    """📐 Shared sun direction onto ``hits`` and each ray's start ``gap`` clear of the sun's centre."""
    sun_c = np.array(sun_c, dtype=float)
    hits = [np.array(h, dtype=float) for h in hits]
    aim = np.mean(hits, axis=0) - sun_c
    aim /= np.linalg.norm(aim)
    return aim, [h - aim * (float(np.dot(h - sun_c, aim)) - gap) for h in hits]


def _pulses(paths, rt: float, *, color=COLOR_G, width: float = 5.0, every: float = 1.6, lead: float = 0.3):
    """⚡ Light pulses along ray polylines, as many rounds as fit into ``rt`` seconds.

    Each pulse starts ``lead`` of the way along the first leg, where a ray that fades in from its sun side is
    already visible, so no pulse is born in empty sky.
    """
    trimmed = [[np.asarray(p[0]) + lead * (np.asarray(p[1]) - np.asarray(p[0])), *p[1:]] for p in paths]
    return pulse_flashes(trimmed, color, repeats=max(1, int(rt / every)), width=width)
#endregion


#region Readouts into the panel
def _freeze(*mobs):
    """🧊 Stop live readouts from redrawing once their value has settled."""
    for mob in mobs:
        mob.clear_updaters()


def _fly_into(scene, panel, sources, *, run_time: float = 1.2):
    """🧲 Fly copies of on-stage readouts into their formula slots, then reveal the rest of the panel.

    Same idea as ``_fill_panel`` in ``1_physical_fundamentals/scene_1.py``.
    https://docs.manim.community/en/stable/reference/manim.animation.transform.FadeTransform.html
    """
    row, box, items = panel
    copies = {}
    for key, src in sources.items():
        copy = src.copy()
        copy.clear_updaters()
        for part in copy.get_family():
            part._layout_zone = "formula"
        copies[key] = copy
    scene.play(*[copy.animate.move_to(items[key]) for key, copy in copies.items()], run_time=run_time * 0.6)
    scene.play(*[FadeTransform(copies[key], items[key]) for key in copies], run_time=run_time * 0.4)
    rest = VGroup(*[m for m in row.submobjects if all(m is not items[key] for key in sources)])
    scene.play(FadeIn(rest), Create(box), run_time=0.6)
#endregion


#region Sun geometry
# The sun is so far away that its rays reach the facade parallel. A room
# section is a dict of screen coordinates (facade plane ``x_w``, back wall
# ``x_b``, floor ``y_f``, sill ``y_s``, window top ``y_t``, ceiling ``y_c``);
# shades are axis-aligned boxes outside the facade whose lower outer edge
# decides where the grazing ray crosses the glass.
def _room_geo(x_w: float, y_f: float, s: float) -> dict:
    """📏 Screen geometry of the example room — facade plane ``x_w``, floor ``y_f``, ``s`` units per metre."""
    return {
        "x_w": x_w, "x_b": x_w - ROOM_DEPTH * s, "y_f": y_f, "y_s": y_f + SILL_H * s,
        "y_t": y_f + (SILL_H + WIN_H) * s, "y_c": y_f + ROOM_H * s, "s": s, "wall": 0.12, "slab": 0.16,
    }


def _block(x0, x1, y0, y1, *, color=PASTEL_WHITE, opacity=0.14):
    """🧱 Filled axis-aligned building element in the Physical Fundamentals slab style."""
    return Polygon([x0, y0, 0], [x1, y0, 0], [x1, y1, 0], [x0, y1, 0],
                   color=PASTEL_WHITE, stroke_width=2, fill_color=color, fill_opacity=opacity)


def _section_room(geo: dict, *, roof_tip=None) -> dict:
    """🏠 Room section: floor and ceiling slab (optionally running out as a roof overhang), back wall, glazed facade."""
    w, sl = geo["wall"], geo["slab"]
    tip = geo["x_w"] if roof_tip is None else roof_tip
    shell = VGroup(
        _block(geo["x_b"] - w, geo["x_w"], geo["y_f"] - sl, geo["y_f"]),
        _block(geo["x_b"] - w, tip, geo["y_c"], geo["y_c"] + sl),
        _block(geo["x_b"] - w, geo["x_b"], geo["y_f"], geo["y_c"]),
        _block(geo["x_w"] - w, geo["x_w"], geo["y_f"], geo["y_s"]),
        _block(geo["x_w"] - w, geo["x_w"], geo["y_t"], geo["y_c"]),
    )
    glass = Rectangle(width=w * 0.8, height=geo["y_t"] - geo["y_s"], color=COLOR_WIN, stroke_width=2.5,
                      fill_color=COLOR_WIN, fill_opacity=0.15).move_to([geo["x_w"] - w / 2, (geo["y_s"] + geo["y_t"]) / 2, 0])
    ground = Line([geo["x_w"], geo["y_f"], 0], [6.6, geo["y_f"], 0], color=PASTEL_TEAL, stroke_width=2.2)
    return {
        "shell": shell, "glass": glass, "ground": ground, "group": VGroup(ground, shell, glass),
        "roof_box": (geo["x_b"] - w, tip, geo["y_c"], geo["y_c"] + sl),
    }


def _sun_dir(elev: float):
    """🧭 Unit vector from the facade toward the sun at ``elev`` degrees (sun on the right)."""
    a = np.radians(elev)
    return np.array([np.cos(a), np.sin(a), 0.0])


def _lit_top(elev: float, geo: dict, edges) -> float:
    """📐 Highest sunlit glass point — window top or where the ray grazing a shade's lower outer edge meets the facade."""
    t = np.tan(np.radians(elev))
    return min([geo["y_t"]] + [y0 - (x1 - geo["x_w"]) * t for x1, y0 in edges])


def _shade_factor(elev: float, geo: dict, edges) -> float:
    """☂️ F_sh as the sunlit share of the glass height."""
    return float(np.clip((_lit_top(elev, geo, edges) - geo["y_s"]) / (geo["y_t"] - geo["y_s"]), 0.0, 1.0))


def _room_hit(p, elev: float, geo: dict):
    """🎯 Where a ray entering the facade at ``p`` lands — on the floor or, if it runs past the depth, on the back wall."""
    t = np.tan(np.radians(elev))
    run = (p[1] - geo["y_f"]) / t
    depth = geo["x_w"] - geo["x_b"]
    if run <= depth:
        return np.array([geo["x_w"] - run, geo["y_f"], 0.0])
    return np.array([geo["x_b"], p[1] - depth * t, 0.0])


def _sun_patch(elev: float, geo: dict, edges=()) -> list:
    """☀️ Sunlit section polygon between the ray over the lit glass top and the ray over the sill, clipped by floor and back wall.

    The patch can only fall away from the window, so the upper back corner of
    the room is never reached by direct sun.
    """
    top = _lit_top(elev, geo, edges)
    if top <= geo["y_s"] + 1e-3:
        return []
    a = np.array([geo["x_w"], top, 0.0])
    b = np.array([geo["x_w"], geo["y_s"], 0.0])
    ha, hb = _room_hit(a, elev, geo), _room_hit(b, elev, geo)
    on_wall = ha[0] <= geo["x_b"] + 1e-6
    on_floor = hb[1] <= geo["y_f"] + 1e-6 and hb[0] > geo["x_b"] + 1e-6
    corner = [np.array([geo["x_b"], geo["y_f"], 0.0])] if on_wall and on_floor else []
    return [a, ha, *corner, hb, b]


def _patch_mob(points, color=PASTEL_ORANGE):
    """🟧 Faint sunlit shaft plus the sunlit floor and back-wall stretch drawn bold."""
    if len(points) < 3:
        return VGroup()
    fill = Polygon(*points, stroke_width=0, fill_color=COLOR_G, fill_opacity=0.12)
    lit = VMobject(stroke_color=color, stroke_width=6, stroke_opacity=0.9)
    lit.set_points_as_corners(points[1:-1])
    return VGroup(fill, lit)


def _box_exit(p, u, box):
    """📦 Distance along ``u`` at which the ray from ``p`` leaves an axis-aligned box, or ``None`` if it misses."""
    lo, hi = 0.0, np.inf
    for pc, uc, a, b in ((p[0], u[0], box[0], box[1]), (p[1], u[1], box[2], box[3])):
        if abs(uc) < 1e-9:
            if not a <= pc <= b:
                return None
            continue
        t0, t1 = sorted(((a - pc) / uc, (b - pc) / uc))
        lo, hi = max(lo, t0), min(hi, t1)
    return hi if hi >= lo else None


RAY_REACH = 2.3


def _ray_paths(elev: float, geo: dict, boxes, heights, sun_at) -> list:
    """🧵 Polyline of every parallel ray at one elevation — sun side, then the facade or shade it stops at, then the room.

    ``heights`` are the facade-plane heights the rays aim at; every ray starts at
    most ``RAY_REACH`` out from the facade, so the parallel bundle never reads
    wider than the sun.
    """
    u = _sun_dir(elev)
    paths = []
    for h in heights:
        p = np.array([geo["x_w"], h, 0.0])
        hits = [t for t in (_box_exit(p, u, b) for b in boxes) if t is not None]
        start = p + u * min(RAY_REACH, max(float(np.dot(np.asarray(sun_at) - p, u)) - 0.6, 0.3))
        if hits:
            paths.append([start, p + u * max(hits)])
        elif geo["y_s"] <= h <= geo["y_t"]:
            paths.append([start, p, _room_hit(p, elev, geo)])
        else:
            paths.append([start, p])
    return paths


def _sun_rays(elev: float, geo: dict, boxes, heights, sun_at):
    """☀️ Parallel straight rays at one elevation — bright and fading in outside, dimmer past the glass."""
    return VGroup(*[
        VGroup(_soft_ray(path[0], path[1], width=1.8, opacity=0.75),
               *([_soft_ray(path[1], path[2], width=1.8, opacity=0.5, fade=False)] if len(path) > 2 else []))
        for path in _ray_paths(elev, geo, boxes, heights, sun_at)
    ])


def _patch_spots(points, geo: dict) -> list:
    """♨️ Ripple sources on a sun patch — ``(point, facing)`` on the lit floor and the lit back wall."""
    if len(points) < 3:
        return []
    lit = points[1:-1]
    floor = [q for q in lit if q[1] <= geo["y_f"] + 1e-6]
    wall = [q for q in lit if q[0] <= geo["x_b"] + 1e-6]
    spots = [(interpolate(floor[0], floor[-1], f) + UP * 0.03, None) for f in (0.3, 0.75)] if len(floor) > 1 else []
    if len(wall) > 1:
        spots.append(((wall[0] + wall[-1]) / 2 + RIGHT * 0.03, 0.0))
    return spots
#endregion


#region Beat1 — Verlust zu Gewinn
class Beat1_VerlustZuGewinn(Scene):
    """🔄 Envelope heat loss next to free solar gain."""

    NARRATION = [
        ("loss",
         "Outside heat leaves through the envelope as transmission loss.",
         "Wärme verlässt die Hülle als Transmissionsverlust."),
        ("flip",
         "At the same time the sun supplies free heat.",
         "Gleichzeitig liefert die Sonne freie Wärme."),
        ("gain",
         "Solar heat gain arrives at roof, walls, and windows.",
         "Solarer Wärmegewinn trifft Dach, Wände und Fenster."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Von Wärmeverlust zu solarem Gewinn", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "loss"))

        #region house and transmission loss
        house = house_section(np.array([-0.2, -0.62, 0.0]), scale=1.15)
        x_l, x_r = house["bottom_left"][0], house["bottom_right"][0]
        y_lo, y_hi = house["windows"][0]["center"][1], house["windows"][1]["center"][1]
        eave_l = house["top_left"] + LEFT * 0.26 * 1.15
        peak = house["roof_peak"]

        def roof_y(x):
            return eave_l[1] + (peak[1] - eave_l[1]) * (1 - abs(x - peak[0]) / (peak[0] - eave_l[0]))

        def leak(x_in, y_in, x_out, side):
            return smooth_path([[x_in, y_in, 0], [x_out, y_in + 0.04, 0], [x_out + side * 0.55, y_in + 0.22, 0],
                                [x_out + side * 1.0, y_in + 0.5, 0]])

        def roof_leak(x):
            side = np.sign(x - peak[0])
            y = roof_y(x)
            return smooth_path([[x - side * 0.25, y - 0.45, 0], [x, y, 0], [x + side * 0.3, y + 0.42, 0],
                                [x + side * 0.65, y + 0.72, 0]])

        loss_paths = [leak(x_l + 0.85, y, x_l, -1) for y in (y_lo - 0.18, y_hi + 0.12)]
        loss_paths += [leak(x_r - 0.85, y, x_r, 1) for y in (y_lo - 0.18, y_hi + 0.12)]
        loss_paths += [roof_leak(peak[0] + dx) for dx in (-1.05, 1.05)]
        loss_guides = flow_guides(loss_paths, PASTEL_CYAN, opacity=0.3)
        loss_label = VGroup(
            Text("Wärmeverlust", font_size=BODY_FONT_SIZE, color=PASTEL_CYAN),
            math_text(r"\Phi_{\mathrm{Verlust}}", font_size=BODY_FONT_SIZE, color=PASTEL_CYAN),
        ).arrange(DOWN, buff=0.14).move_to([4.75, -0.35, 0])

        def leaking(rt):
            return [flow_animation([(loss_paths, PASTEL_ORANGE, PASTEL_CYAN)], waves=4, radius=0.06,
                                   cycles=max(1.0, rt / 1.3))]

        self.play(FadeIn(caption), Create(house["group"]), run_time=1.4)
        self.play(FadeIn(loss_guides), FadeIn(loss_label), *leaking(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "loss", used=1.4 + 1.2, during=leaking)
        #endregion

        #region sun
        sun_c = np.array([-5.45, 1.45, 0.0])
        sun = _sun(sun_c)

        def glowing(rt):
            return [Rotate(sun[3], angle=0.22 * rt, about_point=sun_c, rate_func=linear)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flip"))
        self.play(FadeOut(loss_guides), FadeOut(loss_label), FadeIn(sun, scale=0.7), run_time=1.2)
        hold_for(self, self.NARRATION, "flip", during=glowing)
        #endregion

        #region solar gain on roof, wall and windows
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gain"))
        d = np.array([1.0, -0.42, 0.0]) / np.linalg.norm([1.0, -0.42])
        roof_hits = [np.array([x, roof_y(x), 0.0]) for x in (peak[0] - 1.55, peak[0] - 0.8)]
        wall_hit = np.array([x_l, house["level_1"].get_center()[1], 0.0])
        floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
        glass_hits = [np.array([x_l, w["center"][1] + dy, 0.0]) for w in house["windows"] for dy in (0.1, -0.1)]
        lands = [h + d * (fy - h[1]) / d[1] for h, fy in zip(glass_hits, np.repeat(floors, 2))]
        reach = 2.1
        outer = VGroup(*[_soft_ray(h - d * reach, h) for h in (*roof_hits, wall_hit, *glass_hits)])
        inner = VGroup(*[_soft_ray(h, p, width=2.0, opacity=0.5, fade=False) for h, p in zip(glass_hits, lands)])
        patches = VGroup(*[Line(lands[k], lands[k + 1], color=COLOR_G, stroke_width=6, stroke_opacity=0.8)
                           for k in (0, 2)])
        gain_paths = [[h - d * reach, h] for h in (*roof_hits, wall_hit)]
        gain_paths += [[h - d * reach, h, p] for h, p in zip(glass_hits, lands)]
        warm = [(lands[k] + lands[k + 1]) / 2 + UP * 0.03 for k in (0, 2)]
        gain_label = VGroup(
            Text("Solarer Gewinn", font_size=BODY_FONT_SIZE, color=COLOR_G),
            math_text(r"\Phi_{\mathrm{solar}}", font_size=BODY_FONT_SIZE, color=COLOR_G),
        ).arrange(DOWN, buff=0.14).move_to(loss_label)

        def sunshine(rt):
            return [_pulses(gain_paths, rt), ripples(warm, r_max=0.5, color=PASTEL_ORANGE, cycles=max(1.0, rt / 1.4)),
                    *glowing(rt)]

        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in outer], lag_ratio=0.1), FadeIn(gain_label),
                  run_time=1.3)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in inner], lag_ratio=0.1), FadeIn(patches),
                  run_time=0.8)
        hold_for(self, self.NARRATION, "gain", during=sunshine)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 — Bestrahlung und Fläche
def _bands(x0, x1, y0, y1, t: float, **style):
    """🔲 Four filled bands of thickness ``t`` lining the rectangle ``x0..x1 × y0..y1`` — a wall or frame around a hole."""
    return VGroup(*[
        Rectangle(width=w, height=h, **style).move_to([cx, cy, 0])
        for w, h, cx, cy in (
            (x1 - x0, t, (x0 + x1) / 2, y1 - t / 2), (x1 - x0, t, (x0 + x1) / 2, y0 + t / 2),
            (t, y1 - y0 - 2 * t, x0 + t / 2, (y0 + y1) / 2), (t, y1 - y0 - 2 * t, x1 - t / 2, (y0 + y1) / 2),
        )
    ])


class Beat2_BestrahlungUndFlaeche(Scene):
    """🪟 Irradiance G, area A and glass share F_f — read off the drawn window as symbols, then one formula."""

    NARRATION = [
        ("aperture",
         "Start from the full window opening in the wall.",
         "Zuerst die gesamte Fensteröffnung in der Wand."),
        ("g",
         "Irradiance G is the solar power that reaches each square metre of the facade.",
         "Die Bestrahlungsstärke G ist die Sonnenleistung, die auf jeden Quadratmeter der Fassade trifft."),
        ("a",
         "Area A is the window opening that the radiation can hit.",
         "Die Fläche A ist die Fensteröffnung, auf die die Strahlung trifft."),
        ("ff",
         "The frame blocks; only the glass share F-f lets the radiation through.",
         "Der Rahmen blockiert, nur der Glasanteil F-f lässt die Strahlung durch."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Bestrahlungsstärke G und Fläche A", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "aperture"))
        self.play(FadeIn(caption), run_time=0.3)

        #region opening
        s = 1.1
        oc = np.array([-2.9, 0.35, 0.0])
        ow, oh = WIN_W * s, WIN_H * s
        ox0, ox1, oy0, oy1 = oc[0] - ow / 2, oc[0] + ow / 2, oc[1] - oh / 2, oc[1] + oh / 2
        slab = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)
        wall = VGroup(
            Rectangle(width=3.6, height=oc[1] + 1.65 - oy1, **slab).move_to([oc[0], (oy1 + oc[1] + 1.65) / 2, 0]),
            Rectangle(width=3.6, height=oy0 - (oc[1] - 1.65), **slab).move_to([oc[0], (oy0 + oc[1] - 1.65) / 2, 0]),
            Rectangle(width=1.8 - ow / 2, height=oh, **slab).move_to([(oc[0] - 1.8 + ox0) / 2, oc[1], 0]),
            Rectangle(width=1.8 - ow / 2, height=oh, **slab).move_to([(oc[0] + 1.8 + ox1) / 2, oc[1], 0]),
        )
        opening = Rectangle(width=ow, height=oh, color=COLOR_A, stroke_width=3).move_to(oc)
        self.play(Create(wall), Create(opening), run_time=1.3)
        hold_for(self, N, "aperture", used=1.3 + 0.3)
        #endregion

        #region irradiance
        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        sun_c = np.array([4.4, 1.6, 0.0])
        sun = _sun(sun_c)
        frame_t = FRAME_FACE * s
        frame_hits = [np.array(p) for p in ((ox1 - frame_t / 2, 0.95, 0), (oc[0] + 0.25, oy1 - frame_t / 2, 0),
                                            (oc[0] - 0.35, oy0 + frame_t / 2, 0), (ox0 + frame_t / 2, 0.2, 0))]
        glass_hits = [np.array(p) for p in ((-3.12, 0.92, 0), (-2.6, 0.78, 0), (-3.15, -0.12, 0),
                                            (-2.58, -0.2, 0), (-2.88, -0.28, 0))]
        aim, flat = _parallel_starts(sun_c, frame_hits + glass_hits, gap=0.55)
        starts = [sun_c + aim * 0.55 + 0.22 * (f - sun_c - aim * 0.55) for f in flat]
        rays = VGroup(*[_soft_ray(a, h, width=2.0) for a, h in zip(starts, frame_hits + glass_hits)])
        frame_rays, glass_rays = rays[:len(frame_hits)], rays[len(frame_hits):]
        all_paths = [[a, h] for a, h in zip(starts, frame_hits + glass_hits)]
        glass_paths = all_paths[len(frame_hits):]
        g_lbl = math_label("G", sun_c + DOWN * 1.15, size=FORMULA_FONT_SIZE, color=COLOR_G)

        def sunshine(rt):
            return [_pulses(all_paths, rt, every=1.8, lead=0.12)]

        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in rays], lag_ratio=0.08), FadeIn(g_lbl),
                  run_time=1.6)
        hold_for(self, N, "g", during=sunshine)
        #endregion

        #region area A
        caption = swap_caption(self, caption, subtitle_text(N, "a"))
        area = Rectangle(width=ow, height=oh, stroke_width=0, fill_color=COLOR_A, fill_opacity=0.35).move_to(oc)
        a_lbl = math_label("A", oc, size=FORMULA_FONT_SIZE, color=COLOR_A)
        self.play(GrowFromEdge(area, DOWN), FadeIn(a_lbl), sunshine(1.2)[0], run_time=1.2)
        hold_for(self, N, "a", during=sunshine)
        #endregion

        #region glass share F_f
        caption = swap_caption(self, caption, subtitle_text(N, "ff"))
        frame = _bands(ox0, ox1, oy0, oy1, frame_t, color=PASTEL_WHITE, stroke_width=1.5, fill_color=PASTEL_WHITE,
                       fill_opacity=0.45)
        glass = Rectangle(width=(WIN_W - 2 * FRAME_FACE) * s, height=(WIN_H - 2 * FRAME_FACE) * s,
                          color=COLOR_WIN, fill_color=COLOR_WIN, fill_opacity=0.15, stroke_width=2).move_to(oc)
        ff_lbl = math_label(r"F_{\mathrm{f}}", oc, size=FORMULA_FONT_SIZE, color=COLOR_FF)
        self.play(FadeOut(area), FadeIn(frame), run_time=0.7)
        self.play(TransformFromCopy(opening, glass), ReplacementTransform(a_lbl, ff_lbl),
                  frame_rays.animate.set_stroke(color=GREY_C, opacity=[0.0, 0.35, 0.35]), run_time=1.0)
        self.add(frame_rays, frame, glass, glass_rays)
        self.bring_to_front(ff_lbl)
        a_side = math_label("A", oc + LEFT * (ow / 2 + 0.45), size=FORMULA_FONT_SIZE, color=COLOR_A)
        self.play(FadeIn(a_side), run_time=0.5)

        def through_glass(rt):
            return [_pulses(glass_paths, rt, every=1.8, lead=0.12), glass.animate.set_fill(COLOR_WIN, opacity=0.3)]

        panel = math_panel([
            ("phi", r"\Phi", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g", "G", COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", r"F_{\mathrm{f}}", COLOR_FF), (None, r"\;[\mathrm{W}]", PASTEL_WHITE),
        ])
        _fly_into(self, panel, {"g": g_lbl, "a": a_side, "ff": ff_lbl})
        ring = highlight_param(panel[2], "ff", color=COLOR_FF)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "ff", during=through_glass)
        self.play(FadeOut(ring), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 — g-Wert
def _glazing(x: float, y0: float, y1: float):
    """🪟 Double glazing in section: two panes, spacer and frame blocks."""
    panes = VGroup(*[
        Rectangle(width=0.07, height=y1 - y0, stroke_color=COLOR_WIN, stroke_width=2,
                  fill_color=COLOR_WIN, fill_opacity=0.15).move_to([x + dx, (y0 + y1) / 2, 0])
        for dx in (-0.13, 0.13)
    ])
    frames = VGroup(*[
        Rectangle(width=0.5, height=0.22, stroke_color=PASTEL_WHITE, stroke_width=2,
                  fill_color=PASTEL_WHITE, fill_opacity=0.3).move_to([x, y, 0])
        for y in (y0 - 0.11, y1 + 0.11)
    ])
    return panes, frames


class Beat3_GWert(Scene):
    """🪟 Energy split at the glazing: reflected, absorbed, transmitted — g is what reaches the room."""

    NARRATION = [
        ("section",
         "A section through the glazing: outside on the left, inside on the right.",
         "Ein Schnitt durch die Verglasung: links außen, rechts innen."),
        ("split",
         "Of the solar radiation about 20 percent is reflected, 30 percent taken up by the glass and 50 percent let straight through.",
         "Von der Sonnenstrahlung werden rund 20 % reflektiert, 30 % vom Glas aufgenommen und 50 % direkt durchgelassen."),
        ("gval",
         "The warm glass gives its heat off to both sides. Inside count 50 plus 10 percent — g is about 0.6.",
         "Das warme Glas gibt seine Wärme nach beiden Seiten ab. Innen zählen 50 plus 10 % — g ist rund 0,6."),
        ("phi",
         "The g-value is dimensionless and enters the equation as one more factor.",
         "Der g-Wert ist dimensionslos und kommt als weiterer Faktor in die Gleichung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Gesamtenergiedurchlassgrad g", title)
        din = _din_ref("DIN EN 410 · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "section"))
        self.play(FadeIn(caption), run_time=0.3)

        #region glazing section
        panes, frames = _glazing(0.0, -1.35, 1.75)
        x_block_outer, x_block_inner = -0.165, 0.165
        side_out = Text("außen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([-6.1, 1.9, 0])
        side_in = Text("innen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([6.1, 2.1, 0])
        self.play(Create(panes), FadeIn(frames), FadeIn(side_out), FadeIn(side_in), run_time=1.2)
        hold_for(self, N, "section", used=1.2 + 0.3)
        #endregion

        #region split at the glass — one ray, like the Cooling glazing beat
        caption = swap_caption(self, caption, subtitle_text(N, "split"))
        sun_pos = np.array([-5.6, 1.3, 0.0])
        sun = _sun(sun_pos)
        hit_out = np.array([x_block_outer, 0.9, 0.0])
        d_in = hit_out - sun_pos
        d_in = d_in / np.linalg.norm(d_in)
        d_ref = np.array([-d_in[0], d_in[1], 0.0])
        d_glass = np.array([d_in[0], d_in[1] * 0.6, 0.0])
        d_glass = d_glass / np.linalg.norm(d_glass)
        hit_in = hit_out + d_glass * ((x_block_inner - x_block_outer) / d_glass[0])
        ray_end = hit_in + d_in * 6.0
        refl_end = hit_out + d_ref * 2.8

        incoming = radiation_ray(sun_pos + d_in * 0.45, hit_out, color=COLOR_G, stroke_width=2.6)
        reflected = radiation_ray(hit_out, refl_end, color=COLOR_REFL, stroke_width=2.2).set_stroke(opacity=0.75)
        glass_seg = radiation_ray(hit_out, hit_in, color=COLOR_G, stroke_width=1.8).set_stroke(opacity=0.55)
        transmitted = radiation_ray(hit_in, ray_end, color=COLOR_G, stroke_width=2.6)
        ray_path = [[sun_pos + d_in * 0.45, hit_out, hit_in, ray_end]]
        refl_path = [[sun_pos + d_in * 0.45, hit_out, refl_end]]

        in_lbl = Text("100 %", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
        in_lbl.next_to(sun_pos + d_in * 2.0, UP, buff=0.22)
        refl_lbl = Text("reflektiert 20 %", font_size=LABEL_FONT_SIZE, color=COLOR_REFL)
        refl_lbl.move_to((hit_out + refl_end) / 2 + DOWN * 0.45)
        trans_lbl = Text("durchgelassen 50 %", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
        trans_lbl.next_to(hit_in + d_in * 2.4, UP, buff=0.25)
        abs_lbl = Text("im Glas 30 %", font_size=LABEL_FONT_SIZE, color=COLOR_GLASS_HEAT).move_to([0.0, 2.15, 0])

        def sunshine(rt):
            n = max(1, int(rt / 1.6))
            return [pulse_flashes(ray_path, COLOR_G, repeats=n, width=3.2),
                    pulse_flashes(refl_path, COLOR_REFL, repeats=n, width=2.4)]

        self.play(FadeIn(sun, scale=0.7), run_time=0.5)
        self.play(Create(incoming, rate_func=linear), FadeIn(in_lbl), run_time=1.0)
        self.play(
            Create(reflected, rate_func=linear), Create(glass_seg, rate_func=linear),
            Create(transmitted, rate_func=linear), FadeIn(refl_lbl), FadeIn(trans_lbl),
            panes.animate.set_fill(COLOR_GLASS_HEAT, opacity=0.7), FadeIn(abs_lbl),
            run_time=1.8,
        )
        hold_for(self, N, "split", during=sunshine)
        #endregion

        #region secondary heat and g
        caption = swap_caption(self, caption, subtitle_text(N, "gval"))
        heat_y = -0.55
        warm_in = [np.array([x_block_inner, heat_y, 0.0])]
        warm_out = [np.array([x_block_outer, heat_y, 0.0]), np.array([x_block_outer, heat_y - 0.25, 0.0])]
        in_heat_lbl = Text("Wärme nach innen 10 %", font_size=LABEL_FONT_SIZE, color=COLOR_GLASS_HEAT)
        in_heat_lbl.next_to(warm_in[0], RIGHT, buff=0.35)
        out_heat_lbl = Text("Wärme nach außen 20 %", font_size=LABEL_FONT_SIZE, color=COLOR_GLASS_HEAT)
        out_heat_lbl.next_to(warm_out[0], LEFT, buff=0.35).shift(DOWN * 0.1)

        def glass_heat(rt):
            cycles = max(1.0, rt / 1.4)
            return [*sunshine(rt), ripples(warm_in, r_max=0.5, color=COLOR_GLASS_HEAT, cycles=cycles, facing=0.0),
                    ripples(warm_out, r_max=0.5, color=COLOR_GLASS_HEAT, cycles=cycles, facing=PI)]

        self.play(FadeIn(in_heat_lbl), FadeIn(out_heat_lbl), panes.animate.set_fill(COLOR_GLASS_HEAT, opacity=0.45),
                  *glass_heat(1.6), run_time=1.6)
        g_top, g_bot = 1.35, heat_y - 0.35
        brace = BraceBetweenPoints([6.3, g_top, 0], [6.3, g_bot, 0], direction=RIGHT, color=COLOR_GVAL)
        g_lbl = math_label(rf"g \approx {de_num(G_VALUE, 1)}", size=FORMULA_FONT_SIZE, color=COLOR_GVAL)
        g_lbl.next_to(brace, UP, buff=0.15).shift(LEFT * 0.3)
        self.play(GrowFromCenter(brace), FadeIn(g_lbl, shift=UP * 0.1), *glass_heat(1.0), run_time=1.0)
        hold_for(self, N, "gval", during=glass_heat)
        #endregion

        #region into the formula
        caption = swap_caption(self, caption, subtitle_text(N, "phi"))
        g_sym = math_label("g", size=FORMULA_FONT_SIZE, color=COLOR_GVAL)
        g_sym.move_to(g_lbl.get_left() + RIGHT * g_sym.width / 2)
        panel = math_panel([
            ("phi", r"\Phi", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g_irr", "G", COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", r"F_{\mathrm{f}}", COLOR_FF), (None, r"\cdot", PASTEL_WHITE),
            ("g", "g", COLOR_GVAL), (None, r"\;[\mathrm{W}]", PASTEL_WHITE),
        ])
        _fly_into(self, panel, {"g": g_sym})
        ring = highlight_param(panel[2], "g", color=COLOR_GVAL)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "phi", during=glass_heat)
        self.play(FadeOut(ring), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 — Saisonale Sonnenwinkel
class Beat4_SaisonaleWinkel(Scene):
    """🌞 Parallel sun rays at 60° and 20°: the overhang blocks summer, winter light lands on floor and lower back wall."""

    NARRATION = [
        ("path",
         "The sun path climbs high in summer and stays low in winter.",
         "Die Sonnenbahn steht im Sommer hoch und im Winter flach."),
        ("summer",
         "The sun is so far away that its rays arrive parallel. At 60 degrees the roof overhang holds nearly all of them back.",
         "Die Sonne ist so weit weg, dass ihre Strahlen parallel einfallen. Bei 60° hält der Dachüberstand fast alle ab."),
        ("winter",
         "At 20 degrees the rays fall flat onto the floor and at most the lower back wall — the upper back corner gets no direct sun.",
         "Bei 20° fallen die Strahlen flach auf den Boden und höchstens unten auf die Rückwand — oben hinten kommt keine Sonne an."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Saisonale Sonnenwinkel", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "path"))
        self.play(FadeIn(caption), run_time=0.3)

        #region room and sun path
        geo = _room_geo(1.0, -1.9, 1.05)
        tip = geo["x_w"] + ROOF_OVERHANG * geo["s"]
        room = _section_room(geo, roof_tip=tip)
        boxes = [room["roof_box"]]
        edges = [(tip, geo["y_c"])]
        heights = np.linspace(geo["y_f"] + 0.15, geo["y_c"] + geo["slab"] - 0.03, 10)
        hub = np.array([geo["x_w"], geo["y_f"], 0.0])
        radius = 3.9
        elev = ValueTracker(ELEV_SUMMER)

        def sun_at():
            return hub + radius * _sun_dir(elev.get_value())

        arc = DashedVMobject(Arc(
            radius=radius, start_angle=np.radians(ELEV_WINTER), angle=np.radians(ELEV_SUMMER - ELEV_WINTER),
            arc_center=hub, color=PASTEL_YELLOW, stroke_width=2,
        ), num_dashes=24)
        arc_label = Text("Sonnenbahn", font_size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        arc_label.next_to(hub + radius * _sun_dir(ELEV_WINTER), DR, buff=0.55)
        sun = _sun(sun_at())
        sun.add_updater(lambda m: m.move_to(sun_at()))
        elev_read = math_readout(lambda: rf"\text{{Sonnenhöhe}}\;{de_num(elev.get_value())}°",
                                 [4.3, 1.95, 0], size=BODY_FONT_SIZE, color=COLOR_G)
        tag_summer = Text("Sommer: Überhang hält steile Strahlen ab", font_size=BODY_FONT_SIZE, color=COLOR_G)
        tag_summer.move_to([-6.5 + tag_summer.width / 2, 1.95, 0])
        tag_winter = Text("Winter: flache Strahlen bis zur unteren Rückwand", font_size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        tag_winter.move_to([-6.5 + tag_winter.width / 2, 1.95, 0])
        dark_note = Text("oben hinten: keine direkte Sonne", font_size=LABEL_FONT_SIZE, color=GREY_B)
        dark_note.move_to([geo["x_b"] + 0.18 + dark_note.width / 2, geo["y_c"] - 0.3, 0])

        self.play(Create(room["group"]), Create(arc), FadeIn(arc_label), run_time=1.6)
        self.add(sun)
        self.play(FadeIn(sun, scale=0.6), FadeIn(elev_read), run_time=0.9)
        hold_for(self, N, "path", used=1.6 + 0.9 + 0.3,
                 during=lambda rt: [elev.animate(rate_func=there_and_back, run_time=rt).set_value(ELEV_WINTER)])
        #endregion

        #region summer
        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        rays_now = _sun_rays(ELEV_SUMMER, geo, boxes, heights, sun_at())
        summer_paths = _ray_paths(ELEV_SUMMER, geo, boxes, heights, sun_at())
        patch_now = _patch_mob(_sun_patch(ELEV_SUMMER, geo, edges))
        self.play(LaggedStart(*[Create(r) for r in rays_now], lag_ratio=0.1), FadeIn(patch_now), FadeIn(tag_summer),
                  run_time=1.6)
        hold_for(self, N, "summer", during=lambda rt: [_pulses(summer_paths, rt, every=1.8)])
        #endregion

        #region sweep to winter
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        rays = always_redraw(lambda: _sun_rays(elev.get_value(), geo, boxes, heights, sun_at()))
        patch = always_redraw(lambda: _patch_mob(_sun_patch(elev.get_value(), geo, edges)))
        self.play(FadeOut(rays_now), FadeOut(patch_now), run_time=0.6)
        self.add(patch, rays)
        self.play(
            elev.animate.set_value(ELEV_WINTER),
            FadeTransform(tag_summer, tag_winter),
            run_time=3.0, rate_func=smooth,
        )
        _freeze(rays, patch, sun, elev_read)
        winter_paths = _ray_paths(ELEV_WINTER, geo, boxes, heights, sun_at())
        spots = _patch_spots(_sun_patch(ELEV_WINTER, geo, edges), geo)

        def winter_sun(rt):
            cycles = max(1.0, rt / 1.4)
            return [_pulses(winter_paths, rt, every=1.8),
                    *[ripples([c], r_max=0.6, color=PASTEL_ORANGE, cycles=cycles, facing=f) for c, f in spots]]

        self.play(FadeIn(dark_note), *winter_sun(0.6), run_time=0.6)
        hold_for(self, N, "winter", during=winter_sun)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 — Verschattung
COLOR_BLIND = PASTEL_TEAL
FSH_BLIND = 0.15


def _hatch(rect, spacing: float = 0.2):
    """〽️ 45° hatch clipped to a rectangle — the architectural cut convention."""
    x0, x1 = float(rect.get_left()[0]), float(rect.get_right()[0])
    y0, y1 = float(rect.get_bottom()[1]), float(rect.get_top()[1])
    lines, c = VGroup(), x0 - y1
    while c <= x1 - y0:
        xs, xe = max(x0, c + y0), min(x1, c + y1)
        if xe - xs > 1e-3:
            lines.add(Line([xs, xs - c, 0], [xe, xe - c, 0], color=PASTEL_WHITE, stroke_width=1.0, stroke_opacity=0.35))
        c += spacing
    return lines


class Beat5_Verschattung(Scene):
    """⛱️ Vertical section: unshaded beam, a Raffstore that reflects it, the residue — F_sh on a 0…1 scale."""

    NARRATION = [
        ("intro",
         "A vertical section through window and facade.",
         "Ein Vertikalschnitt durch Fenster und Fassade."),
        ("unshaded",
         "Without shading the full radiation reaches the glass and the room: F-sh is 1.",
         "Ohne Verschattung trifft die volle Strahlung Glas und Raum: F-sh ist 1."),
        ("raffstore",
         "An external Raffstore drops in front. Its slats catch the radiation and send it back outside.",
         "Ein außenliegender Raffstore senkt sich davor. Seine Lamellen fangen die Strahlung ab und werfen sie zurück."),
        ("reduced",
         "Only a small residue gets through — F-sh falls to about 0.15.",
         "Nur ein Restanteil kommt durch — F-sh sinkt auf etwa 0,15."),
        ("winter",
         "In winter the blind stays up; the roof overhang shades the low sun only a little — F-sh about 0.8.",
         "Im Winter bleibt der Raffstore oben. Der Dachüberstand verschattet die flache Sonne nur wenig — F-sh rund 0,8."),
        ("fsh",
         "F-sh is the last factor of the solar equation.",
         "F-sh ist der letzte Faktor der solaren Gleichung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Verschattungsfaktor", title)
        din = _din_ref("DIN V 18599-2 · DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        #region section
        glass_out, glass_in = -3.5, -3.3
        wall_top, wall_bottom = 1.55, -0.85
        slab = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)
        lintel = Rectangle(width=0.5, height=0.6, **slab).move_to([-3.4, wall_top + 0.3, 0])
        sill = Rectangle(width=0.5, height=0.6, **slab).move_to([-3.4, wall_bottom - 0.3, 0])
        masonry = VGroup(lintel, sill, _hatch(lintel), _hatch(sill)).set_z_index(3)
        glazing = Rectangle(width=glass_in - glass_out, height=wall_top - wall_bottom, color=COLOR_WIN, stroke_width=2.5,
                            fill_color=COLOR_WIN, fill_opacity=0.15).move_to([(glass_out + glass_in) / 2,
                                                                              (wall_top + wall_bottom) / 2, 0])
        glazing.set_z_index(3)
        slab_left, room_end_x = lintel.get_right()[0], -0.7
        ceiling_y, floor_y = lintel.get_top()[1], sill.get_bottom()[1]
        t = 0.16
        room = VGroup(
            Rectangle(width=room_end_x + t - slab_left, height=t, **slab).move_to(
                [(slab_left + room_end_x + t) / 2, ceiling_y - t / 2, 0]),
            Rectangle(width=room_end_x + t - slab_left, height=t, **slab).move_to(
                [(slab_left + room_end_x + t) / 2, floor_y - t / 2, 0]),
            Rectangle(width=t, height=ceiling_y - floor_y - t, **slab).move_to(
                [room_end_x + t / 2, (ceiling_y + floor_y - t) / 2, 0]),
        )
        lbl_out = Text("außen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([-6.2, 0.35, 0])
        lbl_in = Text("innen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([-1.55, 1.6, 0])
        self.play(Create(masonry), Create(glazing), Create(room), run_time=1.6)
        self.play(FadeIn(lbl_out), FadeIn(lbl_in), run_time=0.6)
        hold_for(self, N, "intro", used=2.2 + 0.3)
        #endregion

        #region rays
        d = np.array([0.75, -0.661, 0.0])
        sun_c = np.array([-6.25, 1.85, 0.0])
        sun = _sun(sun_c)
        slat_x = -4.4
        slat_ys = [1.35 - i * 0.32 for i in range(8)]
        glass_ys = [y - 0.793 for y in slat_ys[:6]]

        def ray(end, back=3.0, **kw):
            return _soft_ray(end - d * back, end, **kw).set_z_index(1)

        def through(start):
            t_hit = min(t for t in ((floor_y - start[1]) / d[1], (room_end_x - start[0]) / d[0]) if t > 0.05)
            return start + d * t_hit

        def warm_spots(ends):
            return [(e + UP * 0.03, None) if e[1] <= floor_y + 1e-6 else (e + LEFT * 0.03, PI) for e in ends]

        def inside(ys, **kw):
            return VGroup(*[_soft_ray([glass_in, y, 0], through(np.array([glass_in, y, 0.0])), fade=False, **kw)
                            .set_z_index(1) for y in ys])

        direct = VGroup(*[ray(np.array([glass_out, y, 0.0]), width=2.4, opacity=0.8) for y in glass_ys])
        interior = inside(glass_ys[:5], width=2.4, opacity=0.6)
        direct_paths = [[r.get_start(), r.get_end(), s.get_end()] for r, s in zip(direct, interior)]
        direct_spots = warm_spots([s.get_end() for s in interior[::2]])
        #endregion

        #region F_sh scale
        sx0, sx1, sy = 1.3, 5.9, 0.35
        scale = VGroup(
            Line([sx0, sy, 0], [sx1, sy, 0], color=PASTEL_WHITE, stroke_width=2),
            *[Line([x, sy - 0.12, 0], [x, sy + 0.12, 0], color=PASTEL_WHITE, stroke_width=2) for x in (sx0, sx1)],
        )
        end_l = VGroup(math_label("0", size=BODY_FONT_SIZE), Text("ganz verschattet", font_size=LABEL_FONT_SIZE,
                                                                 color=COLOR_FRAME)).arrange(DOWN, buff=0.1)
        end_l.next_to([sx0, sy, 0], DOWN, buff=0.25)
        end_r = VGroup(math_label("1", size=BODY_FONT_SIZE), Text("ohne Verschattung", font_size=LABEL_FONT_SIZE,
                                                                 color=COLOR_FRAME)).arrange(DOWN, buff=0.1)
        end_r.next_to([sx1, sy, 0], DOWN, buff=0.25)
        fsh = ValueTracker(1.0)

        def sx(v):
            return sx0 + v * (sx1 - sx0)

        marker = always_redraw(lambda: Triangle(stroke_width=0, fill_color=COLOR_G, fill_opacity=1.0)
                               .scale(0.15).rotate(PI).move_to([sx(fsh.get_value()), sy + 0.24, 0]))
        marker_lbl = math_readout(lambda: rf"F_{{\mathrm{{sh}}}} = {de_num(fsh.get_value(), 2)}",
                                  lambda: np.array([sx(fsh.get_value()), sy + 0.55, 0]),
                                  size=BODY_FONT_SIZE, color=COLOR_G, edge="center")
        #endregion

        def sunlit(paths, spots):
            def during(rt):
                cycles = max(1.0, rt / 1.4)
                return [_pulses(paths, rt, every=1.8),
                        *[ripples([c], r_max=0.5, color=PASTEL_ORANGE, cycles=cycles, facing=f) for c, f in spots]]
            return during

        caption = swap_caption(self, caption, subtitle_text(N, "unshaded"))
        self.play(FadeIn(sun, scale=0.7), run_time=0.6)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in direct], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in interior], lag_ratio=0.1), run_time=1.2)
        self.play(Create(scale), FadeIn(end_l), FadeIn(end_r), *sunlit(direct_paths, direct_spots)(0.8), run_time=0.8)
        self.add(marker, marker_lbl)
        self.play(FadeIn(marker), run_time=0.4)
        hold_for(self, N, "unshaded", during=sunlit(direct_paths, direct_spots))

        #region Raffstore
        caption = swap_caption(self, caption, subtitle_text(N, "raffstore"))
        u = np.array([np.cos(48 * DEGREES), np.sin(48 * DEGREES), 0.0])
        nrm = np.array([-u[1], u[0], 0.0])
        refl_dir = d - 2 * float(np.dot(d, nrm)) * nrm
        slats = VGroup(*[Rectangle(width=0.5, height=0.08, color=COLOR_BLIND, stroke_width=1.5, fill_color=COLOR_BLIND,
                                   fill_opacity=0.85).move_to([slat_x, y, 0]).rotate(48 * DEGREES) for y in slat_ys])
        rail = Rectangle(width=0.6, height=0.24, color=COLOR_BLIND, stroke_width=2, fill_color=COLOR_BLIND,
                         fill_opacity=0.9).move_to([slat_x, 1.95, 0])
        bracket = Line([slat_x + 0.3, 1.95, 0], [-3.65, 1.95, 0], color=COLOR_BLIND, stroke_width=2.5)
        blind = VGroup(rail, bracket, slats).set_z_index(4)
        lbl_blind = Text("Raffstore", font_size=LABEL_FONT_SIZE, color=COLOR_BLIND).move_to([-4.95, -1.45, 0])
        blocked = VGroup(*[ray(np.array([glass_out, y, 0.0]), back=3.0 - 1.316, width=2.4, opacity=0.8)
                           .shift(-d * 1.316) for y in glass_ys])
        refl_starts = [np.array([-4.487, y + 0.077, 0]) for y in slat_ys[:6]]
        reflected = VGroup(*[radiation_ray(p, p + refl_dir * 1.3, color=COLOR_G, stroke_width=2.2)
                             .set_stroke(opacity=0.55).set_z_index(5) for p in refl_starts])
        bounce_paths = [[b.get_start(), b.get_end(), r.get_end()] for b, r in zip(blocked, reflected)]
        blind.shift(UP * 3.0).set_opacity(0)
        self.add(blind)
        self.play(blind.animate.shift(DOWN * 3.0).set_opacity(1.0), fsh.animate.set_value(0.0), run_time=1.6)
        self.play(FadeIn(lbl_blind, shift=UP * 0.15),
                  *[Transform(r, c) for r, c in zip(direct, blocked)],
                  LaggedStart(*[Create(a, rate_func=linear) for a in reflected], lag_ratio=0.08), FadeOut(interior),
                  run_time=1.6)
        hold_for(self, N, "raffstore", during=sunlit(bounce_paths, []))
        #endregion

        #region residue
        caption = swap_caption(self, caption, subtitle_text(N, "reduced"))
        residual = VGroup(*[radiation_ray([-4.226, y + 0.193, 0], through(np.array([-4.226, y + 0.193, 0.0])),
                                          color=COLOR_G, stroke_width=1.4).set_stroke(opacity=0.32).set_z_index(1)
                            for y in slat_ys[:5]])
        rest_paths = [[r.get_start(), r.get_end()] for r in residual]

        def shaded(rt):
            return [*sunlit(bounce_paths, [])(rt), _pulses(rest_paths, rt, width=2.5, every=1.8, lead=0.0)]

        self.play(LaggedStart(*[Create(r) for r in residual], lag_ratio=0.1), fsh.animate.set_value(FSH_BLIND),
                  run_time=1.8)
        hold_for(self, N, "reduced", during=shaded)
        #endregion

        #region winter: blind up, overhang
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        overhang = Rectangle(width=1.25, height=0.16, **slab).move_to([glass_out - 0.62, wall_top + 0.08, 0]).set_z_index(4)
        lbl_over = Text("Dachüberstand", font_size=LABEL_FONT_SIZE, color=COLOR_FRAME)
        lbl_over.move_to([overhang.get_right()[0] + 0.2 - lbl_over.width / 2, lintel.get_top()[1] + 0.25, 0])
        winter_direct = VGroup(*[ray(np.array([glass_out, y, 0.0]), width=2.4, opacity=0.8) for y in glass_ys[1:]])
        winter_cut = ray(np.array([glass_out, glass_ys[0], 0.0]), back=3.0 - 1.45, width=2.4,
                         opacity=0.8).shift(-d * 1.45)
        winter_in = inside(glass_ys[1:5], width=2.4, opacity=0.6)
        winter_paths = [[r.get_start(), r.get_end(), s.get_end()] for r, s in zip(winter_direct, winter_in)]
        winter_paths.append([winter_cut.get_start(), winter_cut.get_end()])
        winter_spots = warm_spots([s.get_end() for s in winter_in[::2]])
        self.play(blind.animate.shift(UP * 3.0).set_opacity(0), FadeOut(lbl_blind), FadeOut(reflected),
                  FadeOut(residual), FadeOut(direct), run_time=1.2)
        self.play(FadeIn(overhang), FadeIn(lbl_over), Create(winter_cut, rate_func=linear),
                  LaggedStart(*[Create(r, rate_func=linear) for r in winter_direct], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in winter_in], lag_ratio=0.1),
                  fsh.animate.set_value(FSH_WINTER), run_time=1.6)
        winter_sun = sunlit(winter_paths, winter_spots)
        hold_for(self, N, "winter", during=winter_sun)
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "fsh"))
        _freeze(marker, marker_lbl)
        sym = math_label(r"F_{\mathrm{sh}}", size=BODY_FONT_SIZE, color=COLOR_G)
        sym.move_to(marker_lbl.get_left() + RIGHT * sym.width / 2)
        panel = math_panel([
            ("phi", r"\Phi", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g_irr", "G", COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", r"F_{\mathrm{f}}", COLOR_FF), (None, r"\cdot", PASTEL_WHITE),
            ("g", "g", COLOR_GVAL), (None, r"\cdot", PASTEL_WHITE),
            ("fsh", r"F_{\mathrm{sh}}", COLOR_G), (None, r"\;[\mathrm{W}]", PASTEL_WHITE),
        ], edge_buff=1.35)
        _fly_into(self, panel, {"fsh": sym})
        ring = highlight_param(panel[2], "fsh", color=COLOR_G)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "fsh", during=winter_sun)
        self.play(FadeOut(ring), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 — Wärmespeicherung
class Beat6_Waermespeicherung(Scene):
    """🧱 Floor slab stores winter sun by day and radiates at night."""

    NARRATION = [
        ("day",
         "Dense floor slab absorbs winter sun through the window.",
         "Die dichte Bodenplatte absorbiert Wintersonne durchs Fenster."),
        ("night",
         "At night the stored heat radiates back into the room.",
         "Nachts strahlt die gespeicherte Wärme zurück in den Raum."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmespeicherung und Strahlung", title)
        din = _din_ref("DIN EN ISO 13786")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "day"))
        self.play(FadeIn(caption), run_time=0.3)

        #region room, slab and winter sun
        room = room_section(np.array([0.7, -0.3, 0.0]), w=5.8, h=2.0, slab=0.55, window=(0.3, 0.88))
        room["glass"].set_stroke(COLOR_WIN).set_fill(COLOR_WIN)
        floor_slab = room["floor"]
        floor_label = Text("Betonbodenplatte", font_size=BODY_FONT_SIZE, color=PASTEL_WHITE)
        floor_label.move_to([room["x_l"] + 0.1 + floor_label.width / 2, floor_slab.get_center()[1], 0])
        phase_1 = Text("Tag: Absorption", font_size=BODY_FONT_SIZE, color=COLOR_G)
        phase_2 = Text("Nacht: Abstrahlung", font_size=BODY_FONT_SIZE, color=COLOR_HEAT)
        phase_1.next_to(room["ceiling"], UP, buff=0.35)
        phase_2.move_to(phase_1)

        sun_c = np.array([-5.5, 1.0, 0.0])
        sun = _sun(sun_c)
        sun_label = Text("Wintersonne", font_size=BODY_FONT_SIZE, color=COLOR_G).next_to(sun, UP, buff=0.15)
        ys = np.linspace(room["win_hi"] - 0.12, room["win_lo"] + 0.12, 4)
        rays = sun_rays(sun_c, room["glass_x"], ys, room["y_f"], gap=0.5)
        rays["out"].set_stroke(color=COLOR_G)
        rays["in"].set_stroke(color=COLOR_G)
        lands = rays["lands"]
        patch = Line(lands[-1], lands[0], color=COLOR_G, stroke_width=7, stroke_opacity=0.8)
        ray_paths = [[s, h, p] for s, h, p in zip(rays["starts"], rays["hits"], rays["lands"])]
        charge = [interpolate(lands[-1], lands[0], f) + DOWN * 0.03 for f in (0.45, 0.85)]

        def daylight(rt):
            return [_pulses(ray_paths, rt, lead=0.0),
                    ripples(charge, r_max=0.42, color=COLOR_HEAT, cycles=max(1.0, rt / 1.2), down=True)]

        self.play(Create(room["group"]), FadeIn(floor_label), FadeIn(phase_1), FadeIn(sun, scale=0.7),
                  FadeIn(sun_label), run_time=1.2)
        self.play(shine(rays), run_time=1.0)
        self.play(FadeIn(patch, scale=0.6), floor_slab.animate.set_fill(COLOR_HEAT, opacity=0.55),
                  *daylight(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "day", used=1.2 + 1.0 + 1.4 + 0.3, during=lambda rt: [
            *daylight(rt), floor_slab.animate.set_fill(COLOR_HEAT, opacity=0.75)])
        #endregion

        #region night
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "night"))
        moon = moon_glyph(sun_c, color=COLOR_FRAME)
        moon_label = Text("Nachthimmel", font_size=BODY_FONT_SIZE, color=COLOR_FRAME).next_to(moon, UP, buff=0.2)
        emit = [np.array([x, room["y_f"] + 0.03, 0.0]) for x in np.linspace(room["x_l"] + 0.7, room["x_r"] - 0.7, 5)]
        rad_label = Text("Gespeicherte Wärme", font_size=BODY_FONT_SIZE, color=COLOR_HEAT)
        rad_label.move_to([room["center"][0], room["y_c"] - 0.4, 0])

        def night_glow(rt):
            return [ripples(emit, r_max=0.85, color=COLOR_HEAT, cycles=max(1.0, rt / 1.5)),
                    floor_slab.animate.set_fill(COLOR_HEAT, opacity=0.4)]

        self.play(FadeOut(rays["group"]), FadeOut(patch), FadeOut(sun), FadeOut(sun_label), FadeOut(phase_1),
                  run_time=0.6)
        self.play(FadeIn(phase_2), FadeIn(moon), FadeIn(moon_label), run_time=0.8)
        self.play(FadeIn(rad_label), ripples(emit, r_max=0.85, color=COLOR_HEAT, cycles=1.2), run_time=1.8)
        hold_for(self, self.NARRATION, "night", during=night_glow)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 — Speichermasse Formel
class Beat7_SpeichermasseFormel(Scene):
    """🧮 Stored heat of a drawn concrete ceiling — Q_Speicher = m · c · ΔT, every factor counted from its slab."""

    NARRATION = [
        ("slab",
         "A concrete ceiling, 5 by 4 metres and 20 centimetres thick: 4 cubic metres — at 2,400 kilograms per cubic metre that is 9,600 kilograms.",
         "Betondecke 5 m × 4 m × 0,20 m = 4 m³ — bei 2 400 kg/m³ sind das 9 600 kg."),
        ("formula",
         "Stored heat is mass times specific heat capacity times temperature rise: warmed by 2 kelvin, 19.2 megajoules, about 5.3 kilowatt-hours.",
         "Gespeicherte Wärme ist Masse mal Wärmekapazität mal Delta-T: um 2 K erwärmt 19,2 MJ, rund 5,3 kWh."),
        ("why",
         "Thermal mass softens peaks, delays solar gains, and can cut the heating demand.",
         "Thermische Masse dämpft Spitzen, verzögert Solargewinne und kann den Heizwärmebedarf senken."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Thermische Speichermasse", title)
        din = _din_ref("DIN EN ISO 13786")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "slab"))
        self.play(FadeIn(caption), run_time=0.3)

        #region slab
        s = 0.9
        k = np.array([0.369, 0.258, 0.0])
        o = np.array([-5.6, -0.75, 0.0])
        L, B, D, rho, c, dT = (ValueTracker(0.0) for _ in range(6))

        def vol():
            return L.get_value() * B.get_value() * D.get_value()

        def mass():
            return rho.get_value() * vol()

        def heat():
            return mass() * c.get_value() * dT.get_value()

        def slab():
            lx, dy = RIGHT * max(L.get_value(), 1e-3) * s, UP * max(D.get_value(), 1e-3) * s
            depth = k * max(B.get_value(), 1e-3)
            colour = interpolate_color(ManimColor(COLOR_SLAB), ManimColor(COLOR_HEAT), dT.get_value() / DT_SLAB)
            style = dict(color=PASTEL_WHITE, stroke_width=1.5, fill_color=colour,
                         fill_opacity=0.25 + 0.6 * rho.get_value() / RHO_BETON)
            return VGroup(
                Polygon(o + dy, o + lx + dy, o + lx + dy + depth, o + dy + depth, **style),
                Polygon(o + lx, o + lx + depth, o + lx + depth + dy, o + lx + dy, **style),
                Polygon(o, o + lx, o + lx + dy, o + dy, **style),
            )

        b_base = o + RIGHT * (SLAB_L * s + 0.3)
        slab_mob = always_redraw(slab)
        l_dim = always_redraw(lambda: dim_arrow(o + DOWN * 0.28, o + DOWN * 0.28 + RIGHT * max(0.05, L.get_value() * s),
                                                color=COLOR_MASS))
        b_dim = always_redraw(lambda: dim_arrow(b_base, b_base + k * max(0.15, B.get_value()), color=COLOR_MASS))
        d_dim = always_redraw(lambda: VGroup(
            Line(o + LEFT * 0.3, o + LEFT * 0.3 + UP * max(1e-3, D.get_value() * s), color=COLOR_MASS, stroke_width=3),
            Line(o + LEFT * 0.42, o + LEFT * 0.18, color=COLOR_MASS, stroke_width=2),
            Line(o + LEFT * 0.42 + UP * D.get_value() * s, o + LEFT * 0.18 + UP * D.get_value() * s,
                 color=COLOR_MASS, stroke_width=2),
        ))
        l_lbl = math_readout(lambda: rf"{de_num(L.get_value(), 2)}\,\mathrm{{m}}", o + DOWN * 0.68 + RIGHT * SLAB_L * s / 2,
                             size=LABEL_FONT_SIZE, color=COLOR_MASS, edge="center")
        b_lbl = math_readout(lambda: rf"{de_num(B.get_value(), 2)}\,\mathrm{{m}}", b_base + k * SLAB_B + RIGHT * 0.2,
                             size=LABEL_FONT_SIZE, color=COLOR_MASS)
        d_lbl = math_readout(lambda: rf"{de_num(D.get_value(), 2)}\,\mathrm{{m}}", o + LEFT * 0.55 + UP * 0.02,
                             size=LABEL_FONT_SIZE, color=COLOR_MASS, edge="right")
        col = 2.3
        v_read = math_readout(
            lambda: (rf"V = {de_num(L.get_value(), 2)} \cdot {de_num(B.get_value(), 2)} \cdot {de_num(D.get_value(), 2)}"
                     rf" = {de_num(vol(), 2)}\,\mathrm{{m^{{3}}}}"),
            [col, 2.05, 0], size=BODY_FONT_SIZE, color=COLOR_MASS)
        rho_read = math_readout(lambda: rf"\rho = {de_num(rho.get_value())}\,\mathrm{{kg/m^{{3}}}}",
                                [col, 1.45, 0], size=BODY_FONT_SIZE, color=COLOR_MASS)
        m_read = math_readout(lambda: rf"m = \rho \cdot V = {de_num(mass())}\,\mathrm{{kg}}",
                              [col, 0.85, 0], size=BODY_FONT_SIZE, color=COLOR_MASS)
        c_read = math_readout(lambda: rf"c = {de_num(c.get_value())}\,\mathrm{{J/(kg\,K)}}",
                              [col, 0.25, 0], size=BODY_FONT_SIZE, color=COLOR_C)
        dt_read = math_readout(lambda: rf"\Delta T = {de_num(dT.get_value(), 1)}\,\mathrm{{K}}",
                               [col, -0.35, 0], size=BODY_FONT_SIZE, color=COLOR_G)
        q_read = math_readout(
            lambda: (rf"Q_{{\mathrm{{Speicher}}}} = {de_num(heat() / 1e6, 1)}\,\mathrm{{MJ}}"
                     rf" \approx {de_num(heat() / 3.6e6, 1)}\,\mathrm{{kWh}}"),
            [col, -0.95, 0], size=BODY_FONT_SIZE, color=COLOR_HEAT)

        self.add(slab_mob, l_dim, l_lbl, v_read)
        self.play(L.animate.set_value(SLAB_L), run_time=1.2)
        self.add(b_dim, b_lbl)
        self.play(B.animate.set_value(SLAB_B), run_time=1.0)
        self.add(d_dim, d_lbl)
        self.play(D.animate.set_value(SLAB_D), run_time=0.9)
        self.add(rho_read, m_read)
        self.play(rho.animate.set_value(RHO_BETON), run_time=1.4)
        _freeze(l_dim, b_dim, d_dim, l_lbl, b_lbl, d_lbl, v_read, rho_read, m_read)
        hold_for(self, N, "slab", used=1.2 + 1.0 + 0.9 + 1.4 + 0.3)
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        row, box, items = math_panel([
            ("q", r"Q_{\mathrm{Speicher}}", COLOR_HEAT), (None, "=", PASTEL_WHITE),
            ("m", "m", COLOR_MASS), (None, r"\cdot", PASTEL_WHITE),
            ("c", "c", COLOR_C), (None, r"\cdot", PASTEL_WHITE),
            ("dt", r"\Delta T", COLOR_G), (None, r"\;[\mathrm{J}]", PASTEL_WHITE),
        ])
        self.play(FadeIn(row), Create(box), run_time=0.8)
        ring_m = highlight_param(items, "m", color=COLOR_MASS)
        self.play(Create(ring_m), Indicate(m_read, color=COLOR_MASS, scale_factor=1.08), run_time=0.6)
        self.play(FadeOut(ring_m), run_time=0.2)
        self.add(c_read, q_read)
        ring_c = highlight_param(items, "c", color=COLOR_C)
        self.play(Create(ring_c), c.animate.set_value(C_BETON), run_time=1.2)
        self.play(FadeOut(ring_c), run_time=0.2)
        self.add(dt_read)
        ring_dt = highlight_param(items, "dt", color=COLOR_G)
        self.play(Create(ring_dt), dT.animate.set_value(DT_SLAB), run_time=1.6)
        _freeze(slab_mob, c_read, dt_read, q_read)
        top = [o + UP * SLAB_D * s + k * SLAB_B / 2 + RIGHT * SLAB_L * s * f for f in (0.25, 0.5, 0.75)]

        def slab_heat(r_max, down):
            return lambda rt: [ripples(top, r_max=r_max, color=COLOR_GLASS_HEAT, cycles=max(1.0, rt / 1.3), down=down)]

        hold_for(self, N, "formula", during=slab_heat(0.3, True))
        self.play(FadeOut(ring_dt), run_time=0.2)
        #endregion

        #region why
        caption = swap_caption(self, caption, subtitle_text(N, "why"))
        why_body = VGroup(
            Text("Dämpft Spitzen", font_size=BODY_FONT_SIZE, color=GREY_A),
            Text("Speichert solare Gewinne", font_size=BODY_FONT_SIZE, color=GREY_A),
            Text("Senkt den Heizwärmebedarf", font_size=BODY_FONT_SIZE, color=GREY_A),
        ).arrange(DOWN, buff=0.28, aligned_edge=LEFT)
        why_body.move_to([col + why_body.width / 2, 0.9, 0])
        self.play(
            FadeOut(VGroup(v_read, rho_read, m_read, c_read, dt_read)),
            FadeIn(why_body, shift=UP * 0.12),
            run_time=1.0,
        )
        ring_q = highlight_param(items, "q", color=COLOR_HEAT)
        self.play(Create(ring_q), Indicate(q_read, color=COLOR_HEAT, scale_factor=1.06), run_time=0.6)
        hold_for(self, N, "why", during=slab_heat(0.9, False))
        self.play(FadeOut(ring_q), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 — Hauptgleichung
class Beat8_Hauptgleichung(Scene):
    """☀️ Master equation Φ_solar = G · A · F_f · g · F_sh, counted through with this module's window."""

    NARRATION = [
        ("assemble",
         "Put it together: Phi solar equals G times A times F-f times g times F-sh — unit watt.",
         "Zusammen: Phi-solar gleich G mal A mal F-f mal g mal F-sh — Einheit Watt."),
        ("chain",
         "For this window: 400 times 1.92 times 0.70 times 0.60 times 0.80 — about 258 watts.",
         "Für dieses Fenster: 400 · 1,92 · 0,70 · 0,60 · 0,80 — rund 258 Watt."),
        ("ring",
         "Shading decides how much sun counts.",
         "Die Verschattung entscheidet, wie viel Sonne zählt."),
        ("store",
         "Mass only delays that power. Surplus solar heat overheats the room — the utilisation factor eta h follows in the finale.",
         "Masse verzögert diese Leistung nur. Überschüssige Sonnenwärme überhitzt den Raum — den Ausnutzungsgrad Eta-h zeigt das Finale."),
        ("mass",
         "Stored heat Q is mass m times specific heat capacity c times temperature rise delta T — in joule.",
         "Die gespeicherte Wärme Q ist Masse m mal spezifische Wärmekapazität c mal Temperaturanstieg Delta-T — in Joule."),
        ("release",
         "More mass flattens the daily temperature swing and returns the warmth after sunset.",
         "Mehr Masse glättet den Tagesgang der Temperatur und gibt die Wärme nach Sonnenuntergang zurück."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Hauptgleichung der solaren Gewinne", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "assemble"))
        self.play(FadeIn(caption), run_time=0.3)

        #region tokens into the equation
        token_src = [("g", "G", COLOR_G), ("a", "A", COLOR_A), ("ff", r"F_{\mathrm{f}}", COLOR_FF),
                     ("gval", "g", COLOR_GVAL), ("fsh", r"F_{\mathrm{sh}}", COLOR_FSH)]
        unit_src = [r"[\mathrm{W/m^{2}}]", r"[\mathrm{m^{2}}]", "[–]", "[–]", "[–]"]
        tokens = VGroup(*[math_text(src, font_size=FORMULA_FONT_SIZE, color=col) for _k, src, col in token_src])
        tokens.arrange(RIGHT, buff=0.85).move_to(ORIGIN)
        labs = VGroup(*[math_text(src, font_size=BODY_FONT_SIZE, color=col)
                        for src, (_k, _s, col) in zip(unit_src, token_src)])
        for lab, tok in zip(labs, tokens):
            lab.next_to(tok, DOWN, buff=0.2)
        VGroup(tokens, labs).scale(1.35).move_to(DOWN * 0.2)

        self.play(
            LaggedStart(*[FadeIn(t, shift=DOWN * 0.15) for t in tokens], lag_ratio=0.12),
            LaggedStart(*[FadeIn(l) for l in labs], lag_ratio=0.12),
            run_time=1.4,
        )

        row, items = math_row([
            ("phi", r"\Phi_{\mathrm{solar}}", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g", "G", COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", r"F_{\mathrm{f}}", COLOR_FF), (None, r"\cdot", PASTEL_WHITE),
            ("gval", "g", COLOR_GVAL), (None, r"\cdot", PASTEL_WHITE),
            ("fsh", r"F_{\mathrm{sh}}", COLOR_FSH), (None, r"\;[\mathrm{W}]", PASTEL_WHITE),
        ])
        row.next_to(subtitle, DOWN, buff=0.95)
        row.set_x(0)
        box = SurroundingRectangle(row, color=PASTEL_TEAL, buff=0.22, corner_radius=0.1, stroke_width=2)
        slots = [items[key] for key, _s, _c in token_src]
        rest = VGroup(*[m for m in row.submobjects if all(m is not slot for slot in slots)])
        self.play(
            FadeOut(labs),
            *[ReplacementTransform(tok, slot) for tok, slot in zip(tokens, slots)],
            FadeIn(rest), Create(box),
            run_time=1.6,
        )
        self.remove(*slots, rest)
        self.add(row)
        hold_for(self, N, "assemble", used=1.4 + 1.6 + 0.3)
        #endregion

        #region numeric chain
        caption = swap_caption(self, caption, subtitle_text(N, "chain"))
        fac = {"g": ValueTracker(0.0), "a": ValueTracker(0.0), "ff": ValueTracker(1.0),
               "gval": ValueTracker(1.0), "fsh": ValueTracker(1.0)}

        def phi():
            return float(np.prod([t.get_value() for t in fac.values()]))

        def chain_src():
            v = {key: t.get_value() for key, t in fac.items()}
            return (rf"\Phi_{{\mathrm{{solar}}}} = \textcolor{{{COLOR_G}}}{{{de_num(v['g'])}}} \cdot "
                    rf"\textcolor{{{COLOR_A}}}{{{de_num(v['a'], 2)}}} \cdot "
                    rf"\textcolor{{{COLOR_FF}}}{{{de_num(v['ff'], 2)}}} \cdot "
                    rf"\textcolor{{{COLOR_GVAL}}}{{{de_num(v['gval'], 2)}}} \cdot "
                    rf"\textcolor{{{COLOR_FSH}}}{{{de_num(v['fsh'], 2)}}} \approx {de_num(phi())}\,\mathrm{{W}}")

        chain = math_readout(chain_src, lambda: box.get_bottom() + DOWN * 0.72, size=BODY_FONT_SIZE + 4,
                             color=PASTEL_WHITE, edge="center")
        bar_w = 7.0
        track = always_redraw(lambda: Rectangle(width=bar_w, height=0.2, color=GREY_B, stroke_width=1.5).move_to(
            box.get_bottom() + DOWN * 1.25))
        bar = always_redraw(lambda: Rectangle(
            width=max(1e-3, bar_w * phi() / PHI_GA), height=0.2, stroke_width=0,
            fill_color=COLOR_HEAT, fill_opacity=0.9,
        ).move_to(box.get_bottom() + DOWN * 1.25 + LEFT * bar_w / 2, aligned_edge=LEFT))
        full = math_text(rf"{de_num(PHI_GA)}\,\mathrm{{W}}", font_size=LABEL_FONT_SIZE, color=GREY_B)
        full.add_updater(lambda m: m.next_to(track, RIGHT, buff=0.15))
        self.add(chain)
        self.play(FadeIn(track), FadeIn(bar), FadeIn(full), run_time=0.4)
        for key, value in (("g", G_WINTER), ("a", WIN_A), ("ff", FF_GLASS), ("gval", G_VALUE), ("fsh", FSH_WINTER)):
            ring = highlight_param(items, key, color=dict((k, c) for k, _s, c in token_src)[key])
            self.play(Create(ring), fac[key].animate.set_value(value), run_time=1.0)
            self.play(FadeOut(ring), run_time=0.2)
        _freeze(chain)
        hold_for(self, N, "chain")
        #endregion

        caption = swap_caption(self, caption, subtitle_text(N, "ring"))
        ring = highlight_param(items, "fsh", color=COLOR_FSH)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, N, "ring")
        self.play(FadeOut(ring), run_time=0.25)

        #region thermal mass
        # ── Thermal mass: where that solar power is banked (Beat 6/7 payoff) ──
        caption = swap_caption(self, caption, subtitle_text(N, "store"))
        self.play(
            FadeOut(track), FadeOut(bar), FadeOut(full),
            VGroup(row, box).animate.next_to(subtitle, DOWN, buff=0.45).set_x(0),
            chain.animate.shift(UP * ((subtitle.get_bottom()[1] - 0.45) - box.get_top()[1])),
            run_time=0.8,
        )

        store_note = Text(
            "↓ gespeichert in der thermischen Masse", font_size=BODY_FONT_SIZE, color=COLOR_HEAT,
        )
        store_note.next_to(chain, DOWN, buff=0.4).set_x(0)

        row2, items2 = math_row([
            ("q", r"Q_{\mathrm{Speicher}}", COLOR_HEAT), (None, "=", PASTEL_WHITE),
            ("m", "m", COLOR_MASS), (None, r"\cdot", PASTEL_WHITE),
            ("c", "c", COLOR_C), (None, r"\cdot", PASTEL_WHITE),
            ("dt", r"\Delta T", COLOR_G), (None, r"\;[\mathrm{J}]", PASTEL_WHITE),
        ])
        row2.next_to(store_note, DOWN, buff=0.5).set_x(-3.0)
        box2 = SurroundingRectangle(row2, color=COLOR_HEAT, buff=0.22, corner_radius=0.1, stroke_width=2)
        legend = VGroup(
            math_text(r"m\;\;\text{Masse des Bauteils}\;[\mathrm{kg}]", font_size=LABEL_FONT_SIZE, color=COLOR_MASS),
            math_text(r"c\;\;\text{spez. Wärmekapazität}\;[\mathrm{J/(kg\,K)}]", font_size=LABEL_FONT_SIZE,
                      color=COLOR_C),
            math_text(r"\Delta T\;\;\text{Temperaturanstieg}\;[\mathrm{K}]", font_size=LABEL_FONT_SIZE, color=COLOR_G),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.16)
        legend.next_to(box2, RIGHT, buff=0.6)
        legend.set_opacity(0)

        self.play(FadeIn(store_note, shift=DOWN * 0.1), run_time=0.6)
        self.play(FadeIn(row2), Create(box2), run_time=1.0)
        hold_for(self, N, "store")

        caption = swap_caption(self, caption, subtitle_text(N, "mass"))
        legend.set_opacity(1)
        self.play(
            LaggedStart(*[FadeIn(l, shift=RIGHT * 0.1) for l in legend], lag_ratio=0.2),
            run_time=1.1,
        )
        for key, col in (("m", COLOR_MASS), ("c", COLOR_C), ("dt", COLOR_G)):
            r = highlight_param(items2, key, color=col)
            self.play(Create(r), run_time=0.3)
            self.play(FadeOut(r), run_time=0.18)
        hold_for(self, N, "mass")

        caption = swap_caption(self, caption, subtitle_text(N, "release"))
        ring_q = highlight_param(items2, "q", color=COLOR_HEAT)
        self.play(Create(ring_q), run_time=0.4)
        hold_for(self, N, "release")
        self.play(FadeOut(ring_q), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
