"""🔥 Heating Module 4 — Interne Wärmegewinne.

Migrated from ``merged_scenes.py`` onto the generate-manim-tutorial template:
fixed type scale, typeset ``math_panel`` formulas with units, German ``caption_bar``
subtitles, and ``hold_for`` timing.

Relative layouts stay readable (Beat3: person at the desk with Geräte).
The whole animated stage is then ``scale`` + ``shift`` once so it clears
title / formula / caption zones — no per-point hand tweaks.
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
    SUBTITLE_FONT_SIZE, BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    highlight_param, dim_arrow, math_label, math_panel, de_num, place_math,
    caption_bar, swap_caption, hold_for, subtitle_text,
    watt_anchor, set_vo_language,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 4: Interne Wärmegewinne"

COLOR_PEOPLE = "#F472B6"
COLOR_EQUIP = "#38BDF8"
COLOR_LIGHT = "#FBBF24"
COLOR_HEAT = "#F97316"

# Whole-stage fit (applied once per beat after building the stage).
CONTENT_SCALE = 0.62
CONTENT_GAP_BELOW_TITLE = 1.15
CONTENT_TOP_MAX = 1.15
# Default formula_panel edge_buff is 1.7 — sit a bit lower above the caption.
FORMULA_EDGE_BUFF = 1.2

# Example office occupancy (DIN V 18599-10 style point loads) — one source per beat.
PHI_P_W = 80.0
PHI_E_LAPTOP_W, PHI_E_DEVICE_W = 60.0, 90.0
PHI_E_W = PHI_E_LAPTOP_W + PHI_E_DEVICE_W
PHI_L_W = 60.0
PHI_INT_W = PHI_P_W + PHI_E_W + PHI_L_W
ROOM_W_M, ROOM_D_M = 5.0, 4.0


#region Shared

def _fit_stage(mob, *, below):
    """↘️ Scale the whole stage, then park it under the topic subtitle."""
    mob.scale(CONTENT_SCALE)
    target_top = min(below.get_bottom()[1] - CONTENT_GAP_BELOW_TITLE, CONTENT_TOP_MAX)
    mob.shift(DOWN * (mob.get_top()[1] - target_top))
    return mob


def _seated_person_with_chair():
    """🧍 Seated person + chair (same figure as Beat2 / Φ_p)."""
    head = Circle(
        radius=0.22, color=COLOR_PEOPLE, fill_color=COLOR_PEOPLE,
        fill_opacity=0.3, stroke_width=2,
    ).move_to([0, -0.6, 0])
    torso = Line([0, -0.82, 0], [0, -1.5, 0], color=COLOR_PEOPLE, stroke_width=4)
    thighs = Line([0, -1.5, 0], [0.5, -1.5, 0], color=COLOR_PEOPLE, stroke_width=4)
    calves = Line([0.5, -1.5, 0], [0.5, -2.1, 0], color=COLOR_PEOPLE, stroke_width=4)
    arms = Line([0, -1.0, 0], [0.3, -1.3, 0], color=COLOR_PEOPLE, stroke_width=3)
    human = VGroup(head, torso, thighs, calves, arms)
    chair = VGroup(
        Line([-0.15, -0.8, 0], [-0.15, -1.55, 0], color=GREY_C, stroke_width=2),
        Line([-0.15, -1.55, 0], [0.4, -1.55, 0], color=GREY_C, stroke_width=2),
        Line([0.1, -1.55, 0], [0.1, -2.2, 0], color=GREY_C, stroke_width=2),
    )
    return human, chair


def _person_heat_waves():
    """♨️ Soft rising heat lines above the seated person."""
    def create_wave(x_shift):
        pts = [[x_shift + 0.08 * np.sin(y * 5), y, 0] for y in np.linspace(-0.5, 1.1, 25)]
        wave = VMobject(color=COLOR_PEOPLE, stroke_width=2, stroke_opacity=0.45)
        wave.set_points_smoothly(pts)
        return wave

    return VGroup(create_wave(-0.2), create_wave(0.1), create_wave(0.4))


#region Typeset symbols
def _phi(sub: str) -> str:
    """🔣 LaTeX source of a heat flux Φ with an upright index (p, e, l, int)."""
    return rf"\Phi_{{\mathrm{{{sub}}}}}"


def _watts(sub: str, tracker) -> str:
    """⚡ Live ``Φ_x = … W`` source read from a tracker."""
    return rf"{_phi(sub)} = {de_num(tracker.get_value())}\,\mathrm{{W}}"


def _sum_parts(keys, *, lead=None):
    """🧮 Panel parts ``[lead =] Φ_p + Φ_e + … [W]`` for ``math_panel``."""
    colors = {"p": COLOR_PEOPLE, "e": COLOR_EQUIP, "l": COLOR_LIGHT, "int": P_WHITE}
    parts = [(f"phi_{lead}", _phi(lead), colors[lead]), (None, "=", P_WHITE)] if lead else []
    for i, k in enumerate(keys):
        if i:
            parts.append((None, "+", P_WHITE))
        parts.append((f"phi_{k}", _phi(k), colors[k]))
    parts.append((None, r"\;[\mathrm{W}]", P_WHITE))
    return parts
#endregion

#region Live readouts
_READOUT_CACHE: dict = {}


def _live(fn, at, *, size=None, color=P_WHITE, edge="left"):
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



def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in the other Heating
    modules. Internal gains are tabulated in DIN V 18599-10, so every beat that
    does not already print that standard in its diagram footnotes it here. The
    chip is added after ``_fit_stage`` and sits in absolute frame coordinates,
    so ``CONTENT_SCALE`` never touches it.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Beat1 — Winter house & free internal gains
class Beat1_WinterInterneGewinne(Scene):
    NARRATION = [
        ("winter",
         "Outside it is cold — the envelope is losing heat.",
         "Draußen ist es kalt — die Hülle verliert Wärme."),
        ("inside",
         "Inside, people, devices, and lights already make free heat.",
         "Drinnen erzeugen Personen, Geräte und Licht schon freie Wärme."),
        ("din",
         "Those internal gains cut the annual heating demand in DIN V 18599. A design heating load after DIN EN 12831 usually ignores them.",
         "Diese internen Gewinne senken den Jahres-Heizwärmebedarf nach DIN V 18599. Eine Auslegungsheizlast nach DIN EN 12831 lässt sie meist weg."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Kalte Außenluft vs. interne Quellen", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "winter"))
        self.play(FadeIn(caption), run_time=0.3)

        y_off = -0.3
        grey_line = GREY_B

        outer_walls = (
            VGroup(
                Line([-3.2, -2.2 + y_off, 0], [-3.2, 0.8 + y_off, 0]),
                Line([3.2, -2.2 + y_off, 0], [3.2, 0.8 + y_off, 0]),
                Line([-3.5, 0.8 + y_off, 0], [0, 2.6 + y_off, 0]),
                Line([0, 2.6 + y_off, 0], [3.5, 0.8 + y_off, 0]),
                Line([-3.4, -2.2 + y_off, 0], [3.4, -2.2 + y_off, 0]),
            )
            .set_color(grey_line)
            .set_stroke(width=3)
        )
        interior = (
            VGroup(
                Line([-3.2, -0.7 + y_off, 0], [3.2, -0.7 + y_off, 0]),
                Line([0, -2.2 + y_off, 0], [0, -0.7 + y_off, 0]),
                Line([-0.8, -0.7 + y_off, 0], [-0.8, 0.8 + y_off, 0]),
            )
            .set_color(GREY_C)
            .set_stroke(width=1.5, opacity=0.6)
        )
        windows = (
            VGroup(
                Square(side_length=0.8).move_to([-1.8, 0.1 + y_off, 0]),
                Square(side_length=0.8).move_to([1.8, 0.1 + y_off, 0]),
                Rectangle(width=1.2, height=0.8).move_to([-1.8, -1.45 + y_off, 0]),
                Rectangle(width=1.2, height=0.8).move_to([1.8, -1.45 + y_off, 0]),
            )
            .set_color(grey_line)
            .set_stroke(width=1.5, opacity=0.7)
        )

        moon_center = np.array([-5.2, 2.8, 0])
        moon = VGroup(
            Circle(radius=0.42, color=P_YELLOW, fill_opacity=0.85)
            .move_to(moon_center).set_stroke(width=0),
            Circle(radius=0.38, color="#0B0C10", fill_opacity=1.0)
            .move_to(moon_center + np.array([0.16, 0.12, 0])).set_stroke(width=0),
        )

        particle_positions = [
            [-5.5, 1.2, 0], [-4.8, -0.5, 0], [-5.8, -1.8, 0], [-4.2, 2.0, 0],
            [4.5, 1.8, 0], [5.2, -0.2, 0], [4.1, -1.9, 0], [5.8, 1.0, 0],
            [-2.0, 3.1, 0], [1.5, 3.0, 0], [3.8, 2.7, 0], [-4.0, -2.5, 0],
            [4.8, -2.6, 0], [-5.2, 0.3, 0], [5.5, -1.2, 0],
        ]
        particles = VGroup(*[
            Dot(point=pos, radius=0.035, color=P_BLUE, fill_opacity=0.55)
            for pos in particle_positions
        ])

        # Centered in each upper room (not on window frames / partition).
        source_dots = VGroup(
            Dot([-2.0, -0.05 + y_off, 0], radius=0.09, color=COLOR_PEOPLE, fill_opacity=0.85),
            Dot([0.5, -0.05 + y_off, 0], radius=0.09, color=COLOR_EQUIP, fill_opacity=0.85),
            Dot([2.0, -0.05 + y_off, 0], radius=0.09, color=COLOR_LIGHT, fill_opacity=0.85),
        )
        source_labels = VGroup(
            Text("Personen", font_size=LABEL_FONT_SIZE, color=COLOR_PEOPLE),
            Text("Geräte", font_size=LABEL_FONT_SIZE, color=COLOR_EQUIP),
            Text("Licht", font_size=LABEL_FONT_SIZE, color=COLOR_LIGHT),
        )
        # Personen/Licht's dots sit inside the small 0.8-unit window squares;
        # this whole stage is later uniformly scaled down by CONTENT_SCALE in
        # _fit_stage, which shrinks the label toward the group's centroid
        # faster than it shrinks the gap to the window edge — buff=0.12 (and
        # even 0.30) still lands the label on the window's top edge post-scale,
        # confirmed by replicating the exact scale in isolation. Geräte's dot
        # sits in open wall space between windows, so it was never affected.
        for lab, dot in zip(source_labels, source_dots):
            lab.next_to(dot, UP, buff=0.65)

        din_tag = Text("DIN V 18599", font_size=BODY_FONT_SIZE, color=P_TEAL)
        din_tag.next_to(outer_walls, RIGHT, buff=0.40, aligned_edge=DOWN)

        _fit_stage(VGroup(
            outer_walls, interior, windows, moon, particles,
            source_dots, source_labels, din_tag,
        ), below=subtitle)
        source_dots.set_opacity(0)
        source_labels.set_opacity(0)
        din_tag.set_opacity(0)

        self.play(
            Create(outer_walls, run_time=1.6),
            Create(interior, run_time=1.2),
            Create(windows, run_time=1.2),
            FadeIn(moon, shift=DOWN * 0.15, run_time=1.2),
        )
        self.play(FadeIn(particles, run_time=0.8))
        hold_for(self, self.NARRATION, "winter", used=TITLE_RUN_TIME + 0.35 + 1.6 + 0.8)

        source_dots.set_opacity(1)
        source_labels.set_opacity(1)
        self.play(FadeIn(source_dots), FadeIn(source_labels), run_time=0.9)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "inside"))
        hold_for(self, self.NARRATION, "inside", used=0.9 + 0.35)

        din_tag.set_opacity(1)
        self.play(
            FadeIn(din_tag),
            particles.animate.shift(LEFT * 0.35 * CONTENT_SCALE + DOWN * 0.12 * CONTENT_SCALE),
            rate_func=linear,
            run_time=2.2,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        hold_for(self, self.NARRATION, "din", used=2.2 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 — Persons Φ_p
class Beat2_PersonenPhiP(Scene):
    NARRATION = [
        ("person",
         "One seated person already sheds sensible heat into the room.",
         "Eine sitzende Person gibt schon fühlbare Wärme in den Raum ab."),
        ("power",
         "That load is about eighty watts — roughly a bright bulb.",
         "Diese Last liegt bei etwa achtzig Watt — etwa wie eine helle Birne."),
        ("phi_p",
         "We call the person heat flux Phi p — unit watt.",
         "Wir nennen den Personenwärmestrom Phi-p — Einheit Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Personenabwärme", title)
        din = _din_ref("DIN V 18599-10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "person"))
        self.play(FadeIn(caption), run_time=0.3)

        floor = Line([-4, -2.2, 0], [4, -2.2, 0], color=GREY_B, stroke_width=4)
        house_walls = VMobject(color=GREY_B, stroke_width=3)
        house_walls.set_points_as_corners([
            [-3.5, -2.2, 0], [-3.5, 0.8, 0], [0, 2.6, 0], [3.5, 0.8, 0], [3.5, -2.2, 0],
        ])
        house = VGroup(floor, house_walls)

        human, chair = _seated_person_with_chair()

        glow = VGroup(
            Circle(radius=1.4, color=COLOR_PEOPLE, stroke_width=0, fill_opacity=0.03).move_to([0.1, -1.2, 0]),
            Circle(radius=0.9, color=COLOR_PEOPLE, stroke_width=0, fill_opacity=0.06).move_to([0.1, -1.2, 0]),
            Circle(radius=0.5, color=COLOR_PEOPLE, stroke_width=0, fill_opacity=0.12).move_to([0.1, -1.2, 0]),
        )

        heat_waves = _person_heat_waves()

        # Fit house + figure only — badge is placed afterward at full size.
        _fit_stage(VGroup(house, chair, human, glow, heat_waves), below=subtitle)

        # Hide radiation without destroying soft fill opacities (never set_opacity(1)).
        for ring in glow:
            ring.set_fill(COLOR_PEOPLE, opacity=0)
        heat_waves.set_stroke(opacity=0)

        self.play(
            Create(house, run_time=1.3),
            Create(chair, run_time=0.9),
            Create(human, run_time=1.3),
        )
        hold_for(self, self.NARRATION, "person", used=1.3 + 0.3)

        phi_p = ValueTracker(0.0)
        phi_p_live = _live(
            lambda: _watts("p", phi_p),
            np.array([glow[1].get_left()[0] - 0.05, human.get_center()[1] - 0.05, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_PEOPLE, edge="right",
        )

        self.add(glow, phi_p_live)
        heat_waves.set_stroke(opacity=0.45)
        self.play(
            glow[0].animate.set_fill(opacity=0.03),
            glow[1].animate.set_fill(opacity=0.06),
            glow[2].animate.set_fill(opacity=0.12),
            Create(heat_waves),
            phi_p.animate.set_value(0.6 * PHI_P_W),
            run_time=1.2,
        )
        self.play(
            heat_waves.animate.shift(UP * 0.3 * CONTENT_SCALE).set_stroke(opacity=0.28),
            glow[2].animate.scale(1.12),
            glow[1].animate.scale(1.06),
            phi_p.animate.set_value(PHI_P_W),
            run_time=1.2,
        )

        # Watt badge outside the house (not stage-scaled) — normal readable size.
        anchor = watt_anchor(PHI_P_W, compare="bulb", title="eine sitzende Person")
        anchor.scale(1.0)
        anchor.next_to(house, RIGHT, buff=0.3)
        anchor.set_y(human.get_center()[1] + 0.1)
        if anchor.get_top()[1] > subtitle.get_bottom()[1] - 0.25:
            anchor.set_y(subtitle.get_bottom()[1] - 0.25 - anchor.height / 2)
        if anchor.get_bottom()[1] < -1.1:
            anchor.set_y(-1.1 + anchor.height / 2)
        if anchor.get_right()[0] > 6.5:
            anchor.scale(6.5 / anchor.get_right()[0] * 0.97)
            anchor.next_to(house, RIGHT, buff=0.28)
            anchor.set_y(human.get_center()[1] + 0.05)
        self.play(FadeIn(anchor), run_time=0.7)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "power"))
        hold_for(self, self.NARRATION, "power", used=1.2 + 0.7 + 0.35)

        row, box, items = math_panel(_sum_parts(["p"]), edge_buff=FORMULA_EDGE_BUFF)
        self.play(Create(row), Create(box), run_time=1.0)
        ring = highlight_param(items, "phi_p", color=COLOR_PEOPLE)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "phi_p"))
        hold_for(self, self.NARRATION, "phi_p", used=1.0 + 0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 — Equipment Φ_e
class Beat3_GeraetePhiE(Scene):
    NARRATION = [
        ("desk",
         "Plug loads join in — laptop and rack equipment warm the room.",
         "Steckdosenlasten kommen dazu — Laptop und Geräte wärmen den Raum."),
        ("spark",
         "Electricity becomes heat the moment it leaves the wall socket.",
         "Strom wird Wärme, sobald er die Steckdose verlässt."),
        ("phi_e",
         "That equipment heat flux is Phi e — again in watts.",
         "Dieser Gerätewärmestrom heißt Phi-e — wieder in Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Geräteabwärme", title)
        din = _din_ref("DIN V 18599-10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "desk"))
        self.play(FadeIn(caption), run_time=0.3)

        # Room + Beat2 seated person/chair at the desk.
        floor_y, ceil_y, desk_y = -2.0, 2.5, -1.1

        floor = Line(np.array([-5.0, floor_y, 0]), np.array([4.5, floor_y, 0]), color=GREY_B, stroke_width=2)
        wall = Line(np.array([4.0, floor_y, 0]), np.array([4.0, ceil_y, 0]), color=GREY_B, stroke_width=2)
        ceiling = Line(np.array([-5.0, ceil_y, 0]), np.array([4.0, ceil_y, 0]), color=GREY_B, stroke_width=2)
        room = VGroup(floor, wall, ceiling)

        human, chair = _seated_person_with_chair()
        # Same Beat2 figure; sit at the left edge of the desk (feet on this floor).
        person = VGroup(chair, human)
        person.shift(LEFT * 0.85 + UP * 0.1)

        human_waves = _person_heat_waves()
        human_waves.shift(LEFT * 0.85 + UP * 0.1)
        human_waves.set_stroke(opacity=0.4)

        desk = VGroup(
            Line(np.array([-0.45, desk_y, 0]), np.array([2.05, desk_y, 0]), color=GREY_B, stroke_width=2),
            Line(np.array([-0.25, desk_y, 0]), np.array([-0.25, floor_y, 0]), color=GREY_B, stroke_width=2),
            Line(np.array([1.85, desk_y, 0]), np.array([1.85, floor_y, 0]), color=GREY_B, stroke_width=2),
        )
        laptop = VGroup(
            Line(np.array([-0.05, desk_y, 0]), np.array([0.55, desk_y, 0]), color=GREY_B, stroke_width=2.5),
            Line(np.array([0.55, desk_y, 0]), np.array([0.65, desk_y + 0.5, 0]), color=GREY_B, stroke_width=2.5),
        )
        server_cy = desk_y + 0.65
        server_box = Rectangle(width=0.75, height=1.25, color=GREY_B, stroke_width=2).move_to(np.array([1.3, server_cy, 0]))
        server_lines = VGroup(*[
            Line(np.array([1.0, server_cy - 0.35 + i * 0.2, 0]), np.array([1.6, server_cy - 0.35 + i * 0.2, 0]),
                 color=GREY_B, stroke_width=1)
            for i in range(4)
        ])
        server_leds = VGroup(*[
            Dot(np.array([1.08, server_cy - 0.35 + i * 0.2, 0]), radius=0.03, color=COLOR_EQUIP)
            for i in range(4)
        ])
        server = VGroup(server_box, server_lines, server_leds)

        socket = Square(side_length=0.2, color=GREY_B, fill_opacity=0.3).move_to(np.array([4.0, desk_y, 0]))
        socket_slots = VGroup(
            Line(np.array([3.96, desk_y - 0.05, 0]), np.array([3.96, desk_y + 0.05, 0]), color=GREY_B, stroke_width=1),
            Line(np.array([4.04, desk_y - 0.05, 0]), np.array([4.04, desk_y + 0.05, 0]), color=GREY_B, stroke_width=1),
        )
        # Thin stroke cable — never set_opacity (that fills the arc sector).
        cord = ArcBetweenPoints(
            np.array([1.6, desk_y + 0.1, 0.0]),
            np.array([3.9, desk_y, 0.0]),
            angle=-PI * 0.45,
            color=GREY_B,
            stroke_width=2,
        )
        cord.set_fill(opacity=0)
        cord.set_stroke(GREY_B, width=2, opacity=0)

        equip_wave1 = ParametricFunction(
            lambda t: np.array([0.25 + np.sin(t * 5) * 0.05, desk_y + 0.25 + t * 0.65, 0.0]),
            t_range=[0, 1.3], color=COLOR_HEAT, stroke_width=1.5,
        ).set_opacity(0.7)
        equip_wave2 = ParametricFunction(
            lambda t: np.array([1.15 + np.cos(t * 5) * 0.05, server_cy + 0.2 + t * 0.55, 0.0]),
            t_range=[0, 1.3], color=COLOR_EQUIP, stroke_width=1.5,
        ).set_opacity(0.7)
        equip_wave3 = ParametricFunction(
            lambda t: np.array([1.45 + np.sin(t * 4) * 0.05, server_cy + 0.2 + t * 0.55, 0.0]),
            t_range=[0, 1.3], color=COLOR_EQUIP, stroke_width=1.5,
        ).set_opacity(0.7)

        _fit_stage(VGroup(
            room, person, human_waves,
            desk, laptop, server, socket, socket_slots, cord,
            equip_wave1, equip_wave2, equip_wave3,
        ), below=subtitle)
        for w in (equip_wave1, equip_wave2, equip_wave3):
            w.set_stroke(opacity=0)

        self.play(
            Create(room),
            Create(chair), Create(human),
            Create(human_waves),
            run_time=1.2,
        )
        self.play(
            Create(desk), Create(laptop), Create(server),
            FadeIn(socket), FadeIn(socket_slots),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "desk", used=1.2 + 1.4 + 0.3)

        cord.set_stroke(opacity=1)
        self.play(Create(cord), run_time=1.0)
        spark = Star(
            n=8, outer_radius=0.25 * CONTENT_SCALE, inner_radius=0.08 * CONTENT_SCALE,
            color=COLOR_EQUIP, fill_opacity=0.9,
        ).move_to(socket.get_center())
        self.play(FadeIn(spark, scale=0.3), run_time=0.3)
        self.play(spark.animate.scale(1.8).set_opacity(0), run_time=0.4)
        self.remove(spark)

        lap_w, dev_w = ValueTracker(0.0), ValueTracker(0.0)
        col_x = room.get_right()[0] + 0.35
        lap_live = _live(
            lambda: rf"\text{{Laptop}}\;{de_num(lap_w.get_value())}\,\mathrm{{W}}",
            np.array([col_x, server_box.get_top()[1] + 0.30, 0.0]),
            size=LABEL_FONT_SIZE, color=COLOR_HEAT,
        )
        dev_live = _live(
            lambda: rf"\text{{Gerät}}\;{de_num(dev_w.get_value())}\,\mathrm{{W}}",
            np.array([col_x, server_box.get_top()[1] - 0.12, 0.0]),
            size=LABEL_FONT_SIZE, color=COLOR_EQUIP,
        )
        phi_e = ValueTracker(0.0)
        phi_e.add_updater(lambda t: t.set_value(lap_w.get_value() + dev_w.get_value()))
        phi_e_live = _live(
            lambda: _watts("e", phi_e),
            np.array([col_x, server_box.get_top()[1] - 0.62, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_EQUIP,
        )
        self.add(phi_e, lap_live, dev_live, phi_e_live)
        self.play(
            laptop.animate.set_color(COLOR_HEAT),
            lap_w.animate.set_value(PHI_E_LAPTOP_W),
            run_time=0.6,
        )
        self.play(
            server_box.animate.set_color(COLOR_EQUIP),
            server_lines.animate.set_color(COLOR_EQUIP),
            dev_w.animate.set_value(PHI_E_DEVICE_W),
            run_time=0.6,
        )
        for w in (equip_wave1, equip_wave2, equip_wave3):
            w.set_stroke(opacity=0.7)
        self.play(Create(equip_wave1), Create(equip_wave2), Create(equip_wave3), run_time=1.6)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "spark"))
        hold_for(self, self.NARRATION, "spark", used=1.0 + 0.7 + 1.2 + 1.6 + 0.35)

        row, box, items = math_panel(_sum_parts(["p", "e"]), edge_buff=FORMULA_EDGE_BUFF)
        self.play(Create(row), Create(box), run_time=1.0)
        ring = highlight_param(items, "phi_e", color=COLOR_EQUIP)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "phi_e"))
        hold_for(self, self.NARRATION, "phi_e", used=1.0 + 0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 — Lighting Φ_l
class Beat4_BeleuchtungPhiL(Scene):
    NARRATION = [
        ("lamp",
         "Lighting shines onto the floor — and most of it becomes heat.",
         "Beleuchtung strahlt auf den Boden — und der Großteil wird Wärme."),
        ("waves",
         "Warm plumes rise from the lit patch back into the room air.",
         "Warme Schwaden steigen vom beleuchteten Fleck zurück in die Raumluft."),
        ("phi_l",
         "Lighting heat is Phi l. Together: Phi p plus Phi e plus Phi l.",
         "Lichtwärme ist Phi-l. Zusammen: Phi-p plus Phi-e plus Phi-l."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Beleuchtungswärme", title)
        din = _din_ref("DIN V 18599-10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "lamp"))
        self.play(FadeIn(caption), run_time=0.3)

        ceiling = Line(LEFT * 6 + UP * 2.5, RIGHT * 6 + UP * 2.5, color=GREY_B, stroke_width=4)
        floor = Line(LEFT * 6 + DOWN * 2.2, RIGHT * 6 + DOWN * 2.2, color=GREY_B, stroke_width=4)
        cord = Line(UP * 2.5, UP * 1.3, color=GREY_B, stroke_width=2)
        shade = Polygon(
            UP * 1.3 + LEFT * 0.3, UP * 1.3 + RIGHT * 0.3,
            UP * 0.95 + RIGHT * 0.8, UP * 0.95 + LEFT * 0.8,
            color=GREY_B, fill_color="#1b1e24", fill_opacity=1.0, stroke_width=2,
        )
        bulb = Dot(point=UP * 0.9, color=COLOR_LIGHT, radius=0.16)
        light_cone = Polygon(
            UP * 0.9, DOWN * 2.2 + LEFT * 2.8, DOWN * 2.2 + RIGHT * 2.8,
            color=COLOR_LIGHT, fill_color=COLOR_LIGHT, fill_opacity=0.22, stroke_width=0,
        )
        floor_patch = Ellipse(
            width=5.6, height=0.3, color=COLOR_LIGHT,
            fill_color=COLOR_LIGHT, fill_opacity=0.45, stroke_width=0,
        ).move_to(DOWN * 2.2)

        heat_waves = VGroup()
        for ex in [-1.8, -0.9, 0.0, 0.9, 1.8]:
            wave = VMobject(color=COLOR_HEAT, stroke_width=3)
            wave.set_points_smoothly([
                np.array([ex, -2.2, 0]),
                np.array([ex + 0.12, -1.7, 0]),
                np.array([ex - 0.12, -1.2, 0]),
                np.array([ex, -0.7, 0]),
            ])
            heat_waves.add(wave)

        _fit_stage(VGroup(ceiling, floor, cord, shade, bulb, light_cone, floor_patch, heat_waves), below=subtitle)
        heat_waves.set_opacity(0)

        self.play(Create(ceiling), Create(floor), run_time=1.0)
        self.play(Create(cord), Create(shade), run_time=0.9)
        phi_l = ValueTracker(0.0)
        phi_l_live = _live(
            lambda: _watts("l", phi_l),
            np.array([shade.get_right()[0] + 0.30, shade.get_bottom()[1], 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_LIGHT,
        )
        self.add(phi_l_live)
        self.play(
            FadeIn(bulb),
            GrowFromPoint(light_cone, point=bulb.get_center()),
            FadeIn(floor_patch),
            phi_l.animate.set_value(PHI_L_W),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "lamp", used=1.0 + 0.9 + 1.3 + 0.3)

        heat_waves.set_opacity(1)
        self.play(Create(heat_waves), run_time=1.2)
        self.play(heat_waves.animate.shift(UP * 0.25 * CONTENT_SCALE).set_opacity(0.6), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "waves"))
        hold_for(self, self.NARRATION, "waves", used=1.2 + 1.0 + 0.35)

        row, box, items = math_panel(_sum_parts(["p", "e", "l"]), edge_buff=FORMULA_EDGE_BUFF)
        self.play(Create(row), Create(box), run_time=1.1)
        ring = highlight_param(items, "phi_l", color=COLOR_LIGHT)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "phi_l"))
        hold_for(self, self.NARRATION, "phi_l", used=1.1 + 0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 — Sum Φ_int and specific density
#region Beat5 layout
B5_BAR_UNIT = 3.0 / PHI_INT_W
B5_BAR_W = 0.62
B5_BASE_Y = -1.05
B5_BAR_XS = (-5.30, -3.85, -2.40)
B5_STACK_X = -0.60
B5_PLAN_C = np.array([3.40, 0.35, 0.0])
B5_PLAN_UNIT = 0.5
#endregion


class Beat5_SummeUndDichte(Scene):
    NARRATION = [
        ("sources",
         "Persons, equipment, and lighting are the three internal sources.",
         "Personen, Geräte und Beleuchtung sind die drei internen Quellen."),
        ("sum",
         "Phi int equals Phi p plus Phi e plus Phi l — total free heat in watts.",
         "Phi-int ist Phi-p plus Phi-e plus Phi-l — die gesamte freie Wärme in Watt."),
        ("density",
         "Divide by the net floor area A N to get watts per square meter.",
         "Geteilt durch die Nettogrundfläche A-N ergibt Watt pro Quadratmeter."),
        ("din",
         "That specific flux q int is tabulated in DIN V 18599-10.",
         "Diese spezifische Dichte q-int steht tabellarisch in DIN V 18599-10."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Summe und Wärmestromdichte", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "sources"))
        self.play(FadeIn(caption), run_time=0.3)

        sources = (
            ("p", PHI_P_W, COLOR_PEOPLE, "Personen"),
            ("e", PHI_E_W, COLOR_EQUIP, "Geräte"),
            ("l", PHI_L_W, COLOR_LIGHT, "Beleuchtung"),
        )
        grow = [ValueTracker(0.0) for _ in sources]

        def bar_h(t):
            return max(0.002, t.get_value() * B5_BAR_UNIT)

        bars, tokens, names, values = VGroup(), VGroup(), VGroup(), VGroup()
        for (key, _w, color, name), x, t in zip(sources, B5_BAR_XS, grow):
            bar = Rectangle(
                width=B5_BAR_W, height=0.002, color=color, stroke_width=1.2,
                fill_color=color, fill_opacity=0.78,
            )
            bar.add_updater(lambda m, x=x, t=t: m.stretch_to_fit_height(bar_h(t)).move_to(
                np.array([x, B5_BASE_Y + bar_h(t) / 2, 0.0])))
            bars.add(bar)
            tokens.add(math_label(_phi(key), size=BODY_FONT_SIZE, color=color).move_to(
                np.array([x, B5_BASE_Y - 0.30, 0.0])))
            names.add(Text(name, font_size=LABEL_FONT_SIZE, color=color).move_to(
                np.array([x, B5_BASE_Y - 0.66, 0.0])))
            values.add(_live(
                lambda t=t: rf"{de_num(t.get_value())}\,\mathrm{{W}}",
                lambda bar=bar: bar.get_top() + UP * 0.14,
                size=LABEL_FONT_SIZE, color=color, edge="center",
            ))

        total = ValueTracker(0.0)
        total_live = _live(
            lambda: _watts("int", total),
            np.array([B5_STACK_X, B5_BASE_Y + PHI_INT_W * B5_BAR_UNIT + 0.22, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_HEAT, edge="center",
        )

        self.add(*bars, *values)
        self.play(
            *[FadeIn(m, shift=DOWN * 0.2) for m in (*tokens, *names)],
            *[t.animate.set_value(w) for t, (_k, w, _c, _n) in zip(grow, sources)],
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "sources", used=1.2 + 0.3)

        for bar in bars:
            bar.clear_updaters()
        heights = [w * B5_BAR_UNIT for _k, w, _c, _n in sources]
        lows = np.cumsum([0.0, *heights[:-1]])
        stack_moves, token_moves = [], []
        for bar, tok, h, lo in zip(bars, tokens, heights, lows):
            mid = np.array([B5_STACK_X, B5_BASE_Y + lo + h / 2, 0.0])
            stack_moves.append(bar.animate.move_to(mid))
            token_moves.append(tok.animate.move_to(mid + LEFT * (B5_BAR_W / 2 + 0.40)))
        self.add(total_live)
        self.play(
            FadeOut(values), FadeOut(names),
            *stack_moves, *token_moves,
            total.animate.set_value(PHI_INT_W),
            run_time=1.4,
        )

        row, box, items = math_panel(_sum_parts(["p", "e", "l"], lead="int"), edge_buff=FORMULA_EDGE_BUFF)
        p_tok, e_tok, l_tok = tokens
        self.play(
            p_tok.animate.scale(items["phi_p"].height / p_tok.height).move_to(items["phi_p"].get_center()),
            e_tok.animate.scale(items["phi_e"].height / e_tok.height).move_to(items["phi_e"].get_center()),
            l_tok.animate.scale(items["phi_l"].height / l_tok.height).move_to(items["phi_l"].get_center()),
            FadeIn(row), Create(box),
            run_time=1.6,
        )
        self.remove(p_tok, e_tok, l_tok)
        ring = highlight_param(items, "phi_int", color=P_ORANGE)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sum"))
        hold_for(self, self.NARRATION, "sum", used=1.4 + 1.6 + 0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        density_sub = beat_subtitle("Spezifische Wärmestromdichte (DIN V 18599-10)", title)
        self.play(ReplacementTransform(subtitle, density_sub), run_time=0.7)
        subtitle = density_sub

        row2, box2, items2 = math_panel([
            ("q_int", r"q_{\mathrm{int}}", COLOR_HEAT),
            (None, "=", P_WHITE),
            ("frac", rf"\frac{{{_phi('int')}}}{{\textcolor{{{P_CYAN}}}{{A_{{\mathrm{{N}}}}}}}}", P_WHITE),
            (None, r"\;[\mathrm{W/m^{2}}]", P_WHITE),
        ], edge_buff=FORMULA_EDGE_BUFF)
        self.play(ReplacementTransform(row, row2), ReplacementTransform(box, box2), run_time=1.4)
        row, box, items = row2, box2, {**items2, "a_n": items2["frac"][0][2]}

        pw, pd = ROOM_W_M * B5_PLAN_UNIT, ROOM_D_M * B5_PLAN_UNIT
        plan = Rectangle(
            width=pw, height=pd, color=GREY_B, stroke_width=4,
            fill_color=P_CYAN, fill_opacity=0.10,
        ).move_to(B5_PLAN_C)
        plan_tag = Text("Grundriss Büro", font_size=LABEL_FONT_SIZE, color=GREY_B)
        plan_tag.next_to(plan, UP, buff=0.14)
        w_dim = dim_arrow(plan.get_corner(DL) + DOWN * 0.25, plan.get_corner(DR) + DOWN * 0.25, color=P_CYAN)
        d_dim = dim_arrow(plan.get_corner(DR) + RIGHT * 0.25, plan.get_corner(UR) + RIGHT * 0.25, color=P_CYAN)
        w_on, d_on = ValueTracker(0.0), ValueTracker(0.0)

        def area():
            return w_on.get_value() * ROOM_W_M * d_on.get_value() * ROOM_D_M

        w_live = _live(
            lambda: rf"{de_num(w_on.get_value() * ROOM_W_M)}\,\mathrm{{m}}",
            np.array([plan.get_center()[0], plan.get_bottom()[1] - 0.62, 0.0]),
            size=LABEL_FONT_SIZE, color=P_CYAN, edge="center",
        )
        d_live = _live(
            lambda: rf"{de_num(d_on.get_value() * ROOM_D_M)}\,\mathrm{{m}}",
            np.array([plan.get_right()[0] + 0.45, plan.get_center()[1] - 0.08, 0.0]),
            size=LABEL_FONT_SIZE, color=P_CYAN,
        )
        a_live = _live(
            lambda: rf"A_{{\mathrm{{N}}}} = {de_num(area())}\,\mathrm{{m^{{2}}}}",
            np.array([plan.get_center()[0], plan.get_center()[1] - 0.10, 0.0]),
            size=BODY_FONT_SIZE, color=P_CYAN, edge="center",
        )
        q_on = ValueTracker(0.0)
        q_live = _live(
            lambda: (rf"q_{{\mathrm{{int}}}} = \frac{{{de_num(total.get_value())}\,\mathrm{{W}}}}"
                     rf"{{{de_num(area())}\,\mathrm{{m^{{2}}}}}}"
                     rf" = {de_num(q_on.get_value() * total.get_value() / max(area(), 1e-6), 1)}\,\mathrm{{W/m^{{2}}}}"),
            np.array([plan.get_center()[0], plan_tag.get_top()[1] + 0.42, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_HEAT, edge="center",
        )

        self.play(Create(plan), FadeIn(plan_tag), run_time=0.8)
        self.add(w_live, d_live, a_live)
        self.play(
            Create(w_dim), Create(d_dim),
            w_on.animate.set_value(1.0), d_on.animate.set_value(1.0),
            run_time=1.2,
        )
        ring = highlight_param(items, "a_n", color=P_CYAN)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "density"))
        hold_for(self, self.NARRATION, "density", used=1.4 + 0.8 + 1.2 + 0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        ring = highlight_param(items, "q_int", color=COLOR_HEAT)
        self.add(q_live)
        self.play(Create(ring), q_on.animate.set_value(1.0), run_time=1.2)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        hold_for(self, self.NARRATION, "din", used=1.2 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
