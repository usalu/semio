"""🔥 Heating Module 4 — Interne Wärmegewinne.

Migrated from ``merged_scenes.py`` onto the generate-manim-tutorial template:
fixed type scale, ``formula_panel`` with units, German ``caption_bar``
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
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    watt_anchor,
)

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
#endregion


#region Beat1 — Winter house & free internal gains
class Beat1_WinterInterneGewinne(Scene):
    NARRATION = [
        ("winter",
         "Outside it is winter — cold air wraps the house under a quiet moon.",
         "Draußen ist Winter — kalte Luft umschließt das Haus unter dem Mond."),
        ("inside",
         "Inside, people, devices, and lights already make free heat.",
         "Drinnen erzeugen Personen, Geräte und Licht schon freie Wärme."),
        ("din",
         "Those internal gains cut the heating demand — the idea behind DIN V 18599.",
         "Diese internen Gewinne senken den Heizbedarf — so meint es DIN V 18599."),
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
        din_tag.next_to(outer_walls, DOWN, buff=0.22)

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
         "That load is about eighty to one hundred watts — roughly a bright bulb.",
         "Diese Last liegt bei etwa achtzig bis hundert Watt — etwa wie eine helle Birne."),
        ("phi_p",
         "We call the person heat flux Phi p — unit watt.",
         "Wir nennen den Personenwärmestrom Phi-p — Einheit Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Personenabwärme Φ_p", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

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

        self.add(glow)
        heat_waves.set_stroke(opacity=0.45)
        self.play(
            glow[0].animate.set_fill(opacity=0.03),
            glow[1].animate.set_fill(opacity=0.06),
            glow[2].animate.set_fill(opacity=0.12),
            Create(heat_waves),
            run_time=1.2,
        )
        self.play(
            heat_waves.animate.shift(UP * 0.3 * CONTENT_SCALE).set_stroke(opacity=0.28),
            glow[2].animate.scale(1.12),
            glow[1].animate.scale(1.06),
            run_time=1.2,
        )

        # Watt badge outside the house (not stage-scaled) — normal readable size.
        anchor = watt_anchor(90, compare="bulb", title="Personen ~80–100 W")
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

        # Φ_p panel well below the house floor (clear of the stage).
        row, items = equation_row([
            ("phi_p", "Φ_p", COLOR_PEOPLE),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row, edge_buff=FORMULA_EDGE_BUFF)
        clearance = house.get_bottom()[1] - 0.55
        delta = box.get_top()[1] - clearance
        if delta > 0:
            row.shift(DOWN * delta)
            box.shift(DOWN * delta)
        caption_top = -3.4
        if box.get_bottom()[1] < caption_top:
            up = caption_top - box.get_bottom()[1]
            row.shift(UP * up)
            box.shift(UP * up)
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
        subtitle = beat_subtitle("Geräteabwärme Φ_e", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

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

        self.play(
            laptop.animate.set_color(COLOR_HEAT),
            server_box.animate.set_color(COLOR_EQUIP),
            server_lines.animate.set_color(COLOR_EQUIP),
            run_time=1.2,
        )
        for w in (equip_wave1, equip_wave2, equip_wave3):
            w.set_stroke(opacity=0.7)
        self.play(Create(equip_wave1), Create(equip_wave2), Create(equip_wave3), run_time=1.6)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "spark"))
        hold_for(self, self.NARRATION, "spark", used=1.0 + 0.7 + 1.2 + 1.6 + 0.35)

        row, items = equation_row([
            ("phi_p", "Φ_p", COLOR_PEOPLE),
            (None, "+", P_WHITE),
            ("phi_e", "Φ_e", COLOR_EQUIP),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row, edge_buff=FORMULA_EDGE_BUFF)
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
         "Lighting pours luminous energy onto the floor — and most of it becomes heat.",
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
        subtitle = beat_subtitle("Beleuchtungswärme Φ_l", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

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
        self.play(
            FadeIn(bulb),
            GrowFromPoint(light_cone, point=bulb.get_center()),
            FadeIn(floor_patch),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "lamp", used=1.0 + 0.9 + 1.3 + 0.3)

        heat_waves.set_opacity(1)
        self.play(Create(heat_waves), run_time=1.2)
        self.play(heat_waves.animate.shift(UP * 0.25 * CONTENT_SCALE).set_opacity(0.6), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "waves"))
        hold_for(self, self.NARRATION, "waves", used=1.2 + 1.0 + 0.35)

        row, items = equation_row([
            ("phi_p", "Φ_p", COLOR_PEOPLE),
            (None, "+", P_WHITE),
            ("phi_e", "Φ_e", COLOR_EQUIP),
            (None, "+", P_WHITE),
            ("phi_l", "Φ_l", COLOR_LIGHT),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row, edge_buff=FORMULA_EDGE_BUFF)
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

        p_tok = Text("Φ_p", font_size=FORMULA_FONT_SIZE, color=COLOR_PEOPLE)
        e_tok = Text("Φ_e", font_size=FORMULA_FONT_SIZE, color=COLOR_EQUIP)
        l_tok = Text("Φ_l", font_size=FORMULA_FONT_SIZE, color=COLOR_LIGHT)
        p_lab = Text("Personen", font_size=LABEL_FONT_SIZE, color=COLOR_PEOPLE)
        e_lab = Text("Geräte", font_size=LABEL_FONT_SIZE, color=COLOR_EQUIP)
        l_lab = Text("Beleuchtung", font_size=LABEL_FONT_SIZE, color=COLOR_LIGHT)
        p_lab.next_to(p_tok, UP, buff=0.18)
        e_lab.next_to(e_tok, UP, buff=0.18)
        l_lab.next_to(l_tok, UP, buff=0.18)
        sources = VGroup(
            VGroup(p_tok, p_lab), VGroup(e_tok, e_lab), VGroup(l_tok, l_lab),
        ).arrange(RIGHT, buff=1.1).move_to(ORIGIN)
        _fit_stage(sources, below=subtitle)
        p_group, e_group, l_group = sources

        self.play(
            FadeIn(p_group, shift=DOWN * 0.2),
            FadeIn(e_group, shift=DOWN * 0.2),
            FadeIn(l_group, shift=DOWN * 0.2),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "sources", used=1.2 + 0.3)

        row, items = equation_row([
            ("phi_int", "Φ_int", P_WHITE),
            (None, "=", P_WHITE),
            ("phi_p", "Φ_p", COLOR_PEOPLE),
            (None, "+", P_WHITE),
            ("phi_e", "Φ_e", COLOR_EQUIP),
            (None, "+", P_WHITE),
            ("phi_l", "Φ_l", COLOR_LIGHT),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row, edge_buff=FORMULA_EDGE_BUFF)
        self.play(
            FadeOut(p_lab), FadeOut(e_lab), FadeOut(l_lab),
            p_tok.animate.move_to(items["phi_p"].get_center()),
            e_tok.animate.move_to(items["phi_e"].get_center()),
            l_tok.animate.move_to(items["phi_l"].get_center()),
            FadeIn(row), Create(box),
            run_time=1.6,
        )
        self.remove(p_tok, e_tok, l_tok)
        ring = highlight_param(items, "phi_int", color=P_ORANGE)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sum"))
        hold_for(self, self.NARRATION, "sum", used=1.6 + 0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        density_sub = beat_subtitle("Spezifische Dichte q_int (DIN V 18599-10)", title)
        self.play(ReplacementTransform(subtitle, density_sub), run_time=0.7)
        subtitle = density_sub

        row2, items2 = equation_row([
            ("q_int", "q_int", COLOR_HEAT),
            (None, "=", P_WHITE),
            ("phi_int", "Φ_int", P_WHITE),
            (None, "/", P_WHITE),
            ("a_n", "A_N", P_CYAN),
            (None, "  [W/m²]", P_WHITE),
        ])
        row2, box2 = formula_panel(row2, edge_buff=FORMULA_EDGE_BUFF)
        self.play(ReplacementTransform(row, row2), ReplacementTransform(box, box2), run_time=1.4)
        row, box, items = row2, box2, items2
        ring = highlight_param(items, "a_n", color=P_CYAN)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "density"))
        hold_for(self, self.NARRATION, "density", used=1.4 + 0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        ring = highlight_param(items, "q_int", color=COLOR_HEAT)
        self.play(Create(ring), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        hold_for(self, self.NARRATION, "din", used=0.45 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
