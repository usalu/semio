import os
import numpy as np
import math
from manim import *

from pathlib import Path as _Path
import sys as _sys
_TUTORIAL_ROOT = next(p for p in _Path(__file__).resolve().parents if (p / "manim_fonts.py").is_file())
if str(_TUTORIAL_ROOT) not in _sys.path:
    _sys.path.insert(0, str(_TUTORIAL_ROOT))
from manim_fonts import apply_scene_style, BODY_FONT


class Scene1(Scene):
    def construct(self):
        # Set dark architectural background
        apply_scene_style(self)

        # Color palette definition
        GREY_LINE = GREY_B
        COLOR_PINK = "#F472B6"
        COLOR_BLUE = "#38BDF8"
        COLOR_YELLOW = "#FBBF24"

        # Vertical alignment offset (shifted DOWN by 0.3 units)
        y_off = -0.3

        # Architectural Cross-Section Geometry
        outer_walls = (
            VGroup(
                Line([-3.2, -2.2 + y_off, 0], [-3.2, 0.8 + y_off, 0]),
                Line([3.2, -2.2 + y_off, 0], [3.2, 0.8 + y_off, 0]),
                Line([-3.5, 0.8 + y_off, 0], [0, 2.6 + y_off, 0]),
                Line([0, 2.6 + y_off, 0], [3.5, 0.8 + y_off, 0]),
                Line([-3.4, -2.2 + y_off, 0], [3.4, -2.2 + y_off, 0]),
            )
            .set_color(GREY_LINE)
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
            .set_color(GREY_LINE)
            .set_stroke(width=1.5, opacity=0.7)
        )

        house = VGroup(outer_walls, interior, windows)

        # Crescent Moon (Upper-left corner)
        moon_center = np.array([-5.2, 2.8, 0])
        moon_base = (
            Circle(radius=0.42, color="#FDE047", fill_opacity=0.85)
            .move_to(moon_center)
            .set_stroke(width=0)
        )
        moon_shadow = (
            Circle(radius=0.38, color="#0f1115", fill_opacity=1.0)
            .move_to(moon_center + np.array([0.16, 0.12, 0]))
            .set_stroke(width=0)
        )
        moon = VGroup(moon_base, moon_shadow)

        # Cold Exterior Ambient Particles
        particle_positions = [
            [-5.5, 1.2, 0],
            [-4.8, -0.5, 0],
            [-5.8, -1.8, 0],
            [-4.2, 2.0, 0],
            [4.5, 1.8, 0],
            [5.2, -0.2, 0],
            [4.1, -1.9, 0],
            [5.8, 1.0, 0],
            [-2.0, 3.1, 0],
            [1.5, 3.0, 0],
            [3.8, 2.7, 0],
            [-4.0, -2.5, 0],
            [4.8, -2.6, 0],
            [-5.2, 0.3, 0],
            [5.5, -1.2, 0],
        ]

        particles = VGroup(
            *[
                Dot(point=pos, radius=0.035, color="#93C5FD", fill_opacity=0.55)
                for pos in particle_positions
            ]
        )

        # Subtitle Text (German & DIN Reference)
        subtitle = Text(
            "Kalte Außenluft vs. Interne Wärmequellen", font_size=20, color=GREY_A
        ).to_edge(DOWN, buff=0.5)

        # Animation Sequence
        self.play(
            Create(outer_walls, run_time=2.0),
            Create(interior, run_time=1.5),
            Create(windows, run_time=1.5),
            FadeIn(moon, shift=DOWN * 0.15, run_time=1.5),
        )

        self.play(FadeIn(particles, run_time=1.0))

        self.play(FadeIn(subtitle, shift=UP * 0.2, run_time=1.2))

        self.play(
            particles.animate.shift(LEFT * 0.45 + DOWN * 0.2),
            rate_func=linear,
            run_time=3.5,
        )

        self.wait(2)


class Scene2(Scene):
    def construct(self):
        apply_scene_style(self)

        floor = Line([-4, -2.2, 0], [4, -2.2, 0], color=GREY_B, stroke_width=4)
        house_walls = VMobject(color=GREY_B, stroke_width=3)
        house_walls.set_points_as_corners(
            [
                [-3.5, -2.2, 0],
                [-3.5, 0.8, 0],
                [0, 2.6, 0],
                [3.5, 0.8, 0],
                [3.5, -2.2, 0],
            ]
        )
        house = VGroup(floor, house_walls)

        head = Circle(
            radius=0.22,
            color="#F472B6",
            fill_color="#F472B6",
            fill_opacity=0.3,
            stroke_width=2,
        )
        head.move_to([0, -0.6, 0])

        torso = Line([0, -0.82, 0], [0, -1.5, 0], color="#F472B6", stroke_width=4)
        thighs = Line([0, -1.5, 0], [0.5, -1.5, 0], color="#F472B6", stroke_width=4)
        calves = Line([0.5, -1.5, 0], [0.5, -2.1, 0], color="#F472B6", stroke_width=4)
        arms = Line([0, -1.0, 0], [0.3, -1.3, 0], color="#F472B6", stroke_width=3)
        human = VGroup(head, torso, thighs, calves, arms)

        chair_back = Line(
            [-0.15, -0.8, 0], [-0.15, -1.55, 0], color=GREY_C, stroke_width=2
        )
        chair_seat = Line(
            [-0.15, -1.55, 0], [0.4, -1.55, 0], color=GREY_C, stroke_width=2
        )
        chair_leg = Line([0.1, -1.55, 0], [0.1, -2.2, 0], color=GREY_C, stroke_width=2)
        chair = VGroup(chair_back, chair_seat, chair_leg)

        glow_1 = Circle(
            radius=0.5, color="#F472B6", stroke_width=0, fill_opacity=0.18
        ).move_to([0.1, -1.2, 0])
        glow_2 = Circle(
            radius=0.9, color="#F472B6", stroke_width=0, fill_opacity=0.09
        ).move_to([0.1, -1.2, 0])
        glow_3 = Circle(
            radius=1.4, color="#F472B6", stroke_width=0, fill_opacity=0.04
        ).move_to([0.1, -1.2, 0])
        glow = VGroup(glow_3, glow_2, glow_1)

        def create_wave(x_shift):
            pts = [
                [x_shift + 0.08 * np.sin(y * 5), y, 0]
                for y in np.linspace(-0.5, 1.1, 25)
            ]
            wave = VMobject(color="#F472B6", stroke_width=2, stroke_opacity=0.7)
            wave.set_points_smoothly(pts)
            return wave

        wave1 = create_wave(-0.2)
        wave2 = create_wave(0.1)
        wave3 = create_wave(0.4)
        heat_waves = VGroup(wave1, wave2, wave3)

        eq_zone_title = Text("Interne Wärmegewinne", color=GREY_B, font_size=22)
        eq_zone_title.to_corner(UL).shift(RIGHT * 0.5 + DOWN * 0.3)

        desc_text = Text(
            "Personenabwärme: ~80–100 W", color=WHITE, font_size=16
        ).move_to([1.8, -0.9, 0])
        phi_p = Text("Φ_p", color="#F472B6", font_size=38).next_to(
            desc_text, UP, buff=0.25
        )

        self.play(
            Create(house, run_time=1.5),
            Create(chair, run_time=1.0),
            Create(human, run_time=1.5),
        )
        self.wait(0.5)

        self.play(FadeIn(glow, run_time=1.2), Create(heat_waves, run_time=1.5))
        self.play(
            heat_waves.animate.shift(UP * 0.3).set_opacity(0.4),
            glow_1.animate.scale(1.2),
            glow_2.animate.scale(1.1),
            run_time=1.5,
        )

        self.play(Write(phi_p), FadeIn(desc_text, shift=UP * 0.2), run_time=1.3)
        self.wait(1.0)

        self.play(Write(eq_zone_title), run_time=0.8)

        phi_p_target = Text("Φ_p  (Personen = ~80–100W)", color="#F472B6", font_size=24)
        phi_p_target.next_to(eq_zone_title, DOWN, aligned_edge=LEFT, buff=0.3)

        self.play(Transform(phi_p, phi_p_target), FadeOut(desc_text), run_time=1.7)

        self.wait(2.0)


class Scene3(Scene):
    def construct(self):
        apply_scene_style(self)

        pink_color = "#F472B6"
        blue_color = "#38BDF8"
        orange_color = "#F97316"

        eq_title = Text("Interne Wärmegewinne", font_size=22, color=GREY_B)
        eq_title.to_corner(UL).shift(RIGHT * 0.5 + DOWN * 0.2)

        phi_p = Text("Φ_p", color=pink_color, font_size=32)
        phi_p.move_to(UP * 2.8 + LEFT * 0.8)

        plus_sign = Text("+", color=WHITE, font_size=30)
        plus_sign.next_to(phi_p, RIGHT, buff=0.25)

        phi_e_target = Text("Φ_e", color=blue_color, font_size=32)
        phi_e_target.next_to(plus_sign, RIGHT, buff=0.25)

        self.add(eq_title, phi_p)

        floor = Line(
            LEFT * 5 + DOWN * 2, RIGHT * 4.5 + DOWN * 2, color=GREY_B, stroke_width=2
        )
        wall = Line(
            RIGHT * 4 + DOWN * 2, RIGHT * 4 + UP * 2.5, color=GREY_B, stroke_width=2
        )
        ceiling = Line(
            LEFT * 5 + UP * 2.5, RIGHT * 4 + UP * 2.5, color=GREY_B, stroke_width=2
        )
        room = VGroup(floor, wall, ceiling)
        self.add(room)

        head = Circle(radius=0.22, color=pink_color, fill_opacity=0.2).move_to(
            LEFT * 2.5 + DOWN * 0.5
        )
        body = Line(
            LEFT * 2.5 + DOWN * 0.72,
            LEFT * 2.5 + DOWN * 1.4,
            color=pink_color,
            stroke_width=3,
        )
        legs = VGroup(
            Line(
                LEFT * 2.5 + DOWN * 1.4,
                LEFT * 2.0 + DOWN * 1.4,
                color=pink_color,
                stroke_width=3,
            ),
            Line(
                LEFT * 2.0 + DOWN * 1.4,
                LEFT * 2.0 + DOWN * 2.0,
                color=pink_color,
                stroke_width=3,
            ),
        )
        occupant = VGroup(head, body, legs)
        self.add(occupant)

        human_wave1 = ParametricFunction(
            lambda t: np.array([-2.7 + np.sin(t * 4) * 0.08, -0.3 + t * 0.6, 0.0]),
            t_range=[0, 1.5],
            color=pink_color,
            stroke_width=1.5,
        ).set_opacity(0.5)
        human_wave2 = ParametricFunction(
            lambda t: np.array([-2.3 + np.cos(t * 4) * 0.08, -0.3 + t * 0.6, 0.0]),
            t_range=[0, 1.5],
            color=pink_color,
            stroke_width=1.5,
        ).set_opacity(0.5)
        self.add(human_wave1, human_wave2)

        desk_surface = Line(
            LEFT * 0.8 + DOWN * 1.1,
            RIGHT * 1.8 + DOWN * 1.1,
            color=GREY_B,
            stroke_width=2,
        )
        desk_leg1 = Line(
            LEFT * 0.6 + DOWN * 1.1,
            LEFT * 0.6 + DOWN * 2.0,
            color=GREY_B,
            stroke_width=2,
        )
        desk_leg2 = Line(
            RIGHT * 1.6 + DOWN * 1.1,
            RIGHT * 1.6 + DOWN * 2.0,
            color=GREY_B,
            stroke_width=2,
        )
        desk = VGroup(desk_surface, desk_leg1, desk_leg2)

        laptop_base = Line(
            LEFT * 0.4 + DOWN * 1.1,
            RIGHT * 0.2 + DOWN * 1.1,
            color=GREY_B,
            stroke_width=2.5,
        )
        laptop_screen = Line(
            RIGHT * 0.2 + DOWN * 1.1,
            RIGHT * 0.3 + DOWN * 0.6,
            color=GREY_B,
            stroke_width=2.5,
        )
        laptop = VGroup(laptop_base, laptop_screen)

        server_box = Rectangle(
            width=0.8, height=1.3, color=GREY_B, stroke_width=2
        ).move_to(RIGHT * 1.0 + DOWN * 0.45)
        server_lines = VGroup(
            *[
                Line(
                    RIGHT * 0.68 + DOWN * (0.1 + i * 0.2),
                    RIGHT * 1.32 + DOWN * (0.1 + i * 0.2),
                    color=GREY_B,
                    stroke_width=1,
                )
                for i in range(4)
            ]
        )
        server_leds = VGroup(
            *[
                Dot(
                    RIGHT * 0.76 + DOWN * (0.1 + i * 0.2), radius=0.03, color=blue_color
                )
                for i in range(4)
            ]
        )
        server = VGroup(server_box, server_lines, server_leds)

        socket = Square(side_length=0.2, color=GREY_B, fill_opacity=0.3).move_to(
            RIGHT * 4.0 + DOWN * 1.1
        )
        socket_slots = VGroup(
            Line(
                RIGHT * 3.96 + DOWN * 1.05,
                RIGHT * 3.96 + DOWN * 1.15,
                color=GREY_B,
                stroke_width=1,
            ),
            Line(
                RIGHT * 4.04 + DOWN * 1.05,
                RIGHT * 4.04 + DOWN * 1.15,
                color=GREY_B,
                stroke_width=1,
            ),
        )

        self.play(
            Create(desk),
            Create(laptop),
            Create(server),
            FadeIn(socket),
            FadeIn(socket_slots),
            run_time=2.0,
        )

        cord = CubicBezier(
            RIGHT * 1.35 + DOWN * 1.0,
            RIGHT * 2.2 + DOWN * 1.7,
            RIGHT * 3.2 + DOWN * 1.7,
            RIGHT * 3.9 + DOWN * 1.1,
            color=GREY_B,
            stroke_width=2,
        )
        self.play(Create(cord), run_time=1.2)

        spark = Star(
            n=8,
            outer_radius=0.25,
            inner_radius=0.08,
            color=blue_color,
            fill_opacity=0.9,
        )
        spark.move_to(socket.get_center())

        self.play(FadeIn(spark, scale=0.3), run_time=0.3)
        self.play(spark.animate.scale(1.8).set_opacity(0), run_time=0.4)
        self.remove(spark)

        self.play(
            laptop.animate.set_color(orange_color),
            server_box.animate.set_color(blue_color),
            server_lines.animate.set_color(blue_color),
            run_time=1.5,
        )

        equip_wave1 = ParametricFunction(
            lambda t: np.array([-0.1 + np.sin(t * 5) * 0.06, -0.5 + t * 0.7, 0.0]),
            t_range=[0, 1.4],
            color=orange_color,
            stroke_width=1.5,
        ).set_opacity(0.7)

        equip_wave2 = ParametricFunction(
            lambda t: np.array([0.8 + np.cos(t * 5) * 0.06, 0.2 + t * 0.7, 0.0]),
            t_range=[0, 1.4],
            color=blue_color,
            stroke_width=1.5,
        ).set_opacity(0.7)

        equip_wave3 = ParametricFunction(
            lambda t: np.array([1.2 + np.sin(t * 4) * 0.06, 0.2 + t * 0.7, 0.0]),
            t_range=[0, 1.4],
            color=blue_color,
            stroke_width=1.5,
        ).set_opacity(0.7)

        self.play(
            Create(equip_wave1), Create(equip_wave2), Create(equip_wave3), run_time=2.0
        )

        phi_e_spawn = Text("Φ_e", color=blue_color, font_size=28)
        phi_e_spawn.move_to(RIGHT * 0.5 + DOWN * 0.1)

        self.play(FadeIn(phi_e_spawn, shift=UP * 0.2), run_time=1.0)
        self.wait(0.5)

        self.play(Write(plus_sign), Transform(phi_e_spawn, phi_e_target), run_time=1.5)

        self.wait(2)


class LightingHeat(Scene):
    def construct(self):
        apply_scene_style(self)

        COLOR_PEOPLE = "#F472B6"
        COLOR_EQUIPMENT = "#38BDF8"
        COLOR_LIGHTING = "#FBBF24"
        COLOR_HEAT = "#F97316"
        COLOR_STRUCTURE = GREY_B

        ceiling = Line(
            LEFT * 6 + UP * 2.5,
            RIGHT * 6 + UP * 2.5,
            color=COLOR_STRUCTURE,
            stroke_width=4,
        )
        floor = Line(
            LEFT * 6 + DOWN * 2.2,
            RIGHT * 6 + DOWN * 2.2,
            color=COLOR_STRUCTURE,
            stroke_width=4,
        )

        cord = Line(UP * 2.5, UP * 1.3, color=COLOR_STRUCTURE, stroke_width=2)
        shade = Polygon(
            UP * 1.3 + LEFT * 0.3,
            UP * 1.3 + RIGHT * 0.3,
            UP * 0.9 + RIGHT * 0.8,
            UP * 0.9 + LEFT * 0.8,
            color=COLOR_STRUCTURE,
            fill_color="#1b1e24",
            fill_opacity=1.0,
            stroke_width=2,
        )
        bulb = Dot(point=UP * 0.85, color=COLOR_LIGHTING, radius=0.16)

        light_cone = Polygon(
            UP * 0.85,
            DOWN * 2.2 + LEFT * 2.8,
            DOWN * 2.2 + RIGHT * 2.8,
            color=COLOR_LIGHTING,
            fill_color=COLOR_LIGHTING,
            fill_opacity=0.22,
            stroke_width=0,
        )
        floor_patch = Ellipse(
            width=5.6,
            height=0.3,
            color=COLOR_LIGHTING,
            fill_color=COLOR_LIGHTING,
            fill_opacity=0.45,
            stroke_width=0,
        ).move_to(DOWN * 2.2)

        self.play(Create(ceiling), Create(floor), run_time=1.2)
        self.play(Create(cord), Create(shade), run_time=1.0)
        self.wait(0.3)

        self.play(
            FadeIn(bulb),
            GrowFromPoint(light_cone, point=UP * 0.85),
            FadeIn(floor_patch),
            run_time=1.5,
        )

        photon_end_x = [-1.8, -0.9, 0.0, 0.9, 1.8]

        heat_waves = VGroup()
        for ex in photon_end_x:
            wave = VMobject(color=COLOR_HEAT, stroke_width=3)
            wave.set_points_smoothly(
                [
                    np.array([ex, -2.2, 0]),
                    np.array([ex + 0.12, -1.7, 0]),
                    np.array([ex - 0.12, -1.2, 0]),
                    np.array([ex, -0.7, 0]),
                ]
            )
            heat_waves.add(wave)

        self.play(Create(heat_waves), run_time=1.4)
        self.play(heat_waves.animate.shift(UP * 0.3).set_opacity(0.6), run_time=1.2)

        phi_l_label = Text("Φ_l", color=COLOR_LIGHTING, font_size=36)
        phi_l_label.next_to(shade, RIGHT, buff=0.6)

        eq_p = Text("Φ_p", color=COLOR_PEOPLE, font_size=36)
        eq_plus1 = Text(" + ", color=WHITE, font_size=36)
        eq_e = Text("Φ_e", color=COLOR_EQUIPMENT, font_size=36)
        eq_plus2 = Text(" + ", color=WHITE, font_size=36)
        eq_l_target = Text("Φ_l", color=COLOR_LIGHTING, font_size=36)

        full_equation = (
            VGroup(eq_p, eq_plus1, eq_e, eq_plus2, eq_l_target)
            .arrange(RIGHT, buff=0.12)
            .to_edge(UP, buff=0.5)
        )
        partial_equation = VGroup(eq_p, eq_plus1, eq_e, eq_plus2)

        self.play(Write(phi_l_label), FadeIn(partial_equation), run_time=1.2)
        self.wait(0.4)

        self.play(Transform(phi_l_label, eq_l_target), run_time=1.5)

        bounding_box = SurroundingRectangle(
            full_equation, color=COLOR_LIGHTING, buff=0.15, corner_radius=0.1
        )
        bounding_box.set_stroke(width=1.5)

        total_title = Text(
            "Interne Wärmegewinne (DIN V 18599-10)", color=WHITE, font_size=18
        ).next_to(bounding_box, DOWN, buff=0.18)

        self.play(Create(bounding_box), Write(total_title), run_time=1.0)

        self.wait(2.0)


class Scene5(Scene):
    def construct(self):
        apply_scene_style(self)

        COLOR_PEOPLE = "#F472B6"  # Pink
        COLOR_EQUIP = "#38BDF8"  # Blue
        COLOR_LIGHT = "#FBBF24"  # Yellow
        COLOR_FLUX = "#F97316"  # Orange

        bg_line = Line(LEFT * 6, RIGHT * 6, color=GREY_E, stroke_width=1.5).to_edge(
            DOWN, buff=1.2
        )
        self.add(bg_line)

        def make_var(main_str, sub_str, color, main_size=42, sub_size=26):
            m = Text(main_str, color=color, font_size=main_size)
            s = Text(sub_str, color=color, font_size=sub_size)
            s.next_to(m, RIGHT, buff=0.04).align_to(m, DOWN)
            return VGroup(m, s)

        title = Text("Gesamte interne Wärmegewinne", font_size=30, color=WHITE)
        title.to_edge(UP, buff=0.8)
        self.play(Write(title), run_time=1.0)

        p_var = make_var("Φ", "p", COLOR_PEOPLE)
        e_var = make_var("Φ", "e", COLOR_EQUIP)
        l_var = make_var("Φ", "l", COLOR_LIGHT)

        p_label = Text("Personen", font_size=16, color=COLOR_PEOPLE).next_to(
            p_var, UP, buff=0.2
        )
        e_label = Text("Geräte", font_size=16, color=COLOR_EQUIP).next_to(
            e_var, UP, buff=0.2
        )
        l_label = Text("Beleuchtung", font_size=16, color=COLOR_LIGHT).next_to(
            l_var, UP, buff=0.2
        )

        p_group = VGroup(p_var, p_label)
        e_group = VGroup(e_var, e_label)
        l_group = VGroup(l_var, l_label)

        sources = (
            VGroup(p_group, e_group, l_group).arrange(RIGHT, buff=1.2).shift(UP * 1.5)
        )

        self.play(
            FadeIn(p_group, shift=DOWN * 0.3),
            FadeIn(e_group, shift=DOWN * 0.3),
            FadeIn(l_group, shift=DOWN * 0.3),
            run_time=1.5,
        )
        self.wait(0.5)

        phi_int = make_var("Φ", "int", WHITE)
        eq_sign1 = Text("=", font_size=40, color=WHITE)
        plus1 = Text("+", font_size=32, color=WHITE)
        plus2 = Text("+", font_size=32, color=WHITE)

        p_var_eq = make_var("Φ", "p", COLOR_PEOPLE)
        e_var_eq = make_var("Φ", "e", COLOR_EQUIP)
        l_var_eq = make_var("Φ", "l", COLOR_LIGHT)

        master_eq = (
            VGroup(phi_int, eq_sign1, p_var_eq, plus1, e_var_eq, plus2, l_var_eq)
            .arrange(RIGHT, buff=0.2)
            .move_to(ORIGIN)
        )

        self.play(
            FadeOut(p_label),
            FadeOut(e_label),
            FadeOut(l_label),
            FadeIn(phi_int, shift=DOWN * 0.3),
            FadeIn(eq_sign1),
            Transform(p_var, p_var_eq),
            FadeIn(plus1),
            Transform(e_var, e_var_eq),
            FadeIn(plus2),
            Transform(l_var, l_var_eq),
            run_time=2.0,
        )
        self.wait(1.5)

        density_title = Text(
            "Spezifische interne Wärmestromdichte (DIN V 18599-10)",
            font_size=24,
            color=WHITE,
        )
        density_title.to_edge(UP, buff=0.8)

        l_paren = Text("(", font_size=38, color=WHITE)
        r_paren = Text(")", font_size=38, color=WHITE)

        p_var_num = make_var("Φ", "p", COLOR_PEOPLE, main_size=36, sub_size=22)
        plus1_num = Text("+", font_size=28, color=WHITE)
        e_var_num = make_var("Φ", "e", COLOR_EQUIP, main_size=36, sub_size=22)
        plus2_num = Text("+", font_size=28, color=WHITE)
        l_var_num = make_var("Φ", "l", COLOR_LIGHT, main_size=36, sub_size=22)

        numerator_terms = VGroup(
            l_paren, p_var_num, plus1_num, e_var_num, plus2_num, l_var_num, r_paren
        ).arrange(RIGHT, buff=0.12)

        div_bar = Line(LEFT, RIGHT, color=WHITE, stroke_width=2)
        div_bar.match_width(numerator_terms).scale(1.05)
        div_bar.next_to(numerator_terms, DOWN, buff=0.15)

        area_var = Text("A_N", font_size=34, color=WHITE)
        area_var.next_to(div_bar, DOWN, buff=0.15)

        fraction = VGroup(numerator_terms, div_bar, area_var)

        q_int_var = make_var("q", "int", COLOR_FLUX, main_size=42, sub_size=26)
        eq_sign2 = Text("=", font_size=38, color=WHITE)
        lhs = VGroup(q_int_var, eq_sign2).arrange(RIGHT, buff=0.2)

        full_density_eq = VGroup(lhs, fraction).arrange(RIGHT, buff=0.25).center()

        self.play(
            Transform(title, density_title),
            ReplacementTransform(phi_int, q_int_var),
            ReplacementTransform(eq_sign1, eq_sign2),
            ReplacementTransform(p_var, p_var_num),
            ReplacementTransform(plus1, plus1_num),
            ReplacementTransform(e_var, e_var_num),
            ReplacementTransform(plus2, plus2_num),
            ReplacementTransform(l_var, l_var_num),
            FadeIn(l_paren),
            FadeIn(r_paren),
            Create(div_bar),
            FadeIn(area_var, shift=UP * 0.2),
            run_time=2.5,
        )

        units = Text(
            "Einheit: Watt pro Quadratmeter [W/m²]", font_size=18, color=GREY_B
        )
        units.next_to(full_density_eq, DOWN, buff=0.7)
        self.play(FadeIn(units, shift=UP * 0.2), run_time=1.0)
        self.wait(1.0)

        q_box = SurroundingRectangle(
            q_int_var, color=COLOR_FLUX, buff=0.12, corner_radius=0.08, stroke_width=2
        )
        q_box_glow = SurroundingRectangle(
            q_int_var, color=COLOR_FLUX, buff=0.18, corner_radius=0.12, stroke_width=4
        )
        q_box_glow.set_opacity(0.35)

        self.play(Create(q_box), FadeIn(q_box_glow), run_time=1.2)
        self.wait(2.5)

        self.play(*[FadeOut(mob) for mob in self.mobjects], run_time=1.5)
        self.wait(2)


class FullInternalHeatGainVideo(Scene):
    def construct(self):
        scenes = [Scene1, Scene2, Scene3, LightingHeat, Scene5]
        base_dir = os.path.dirname(os.path.abspath(__file__))
        audio_files = [
            os.path.join(base_dir, f"scene_{i}_audio.mp3") for i in range(1, 6)
        ]

        for scene_cls, audio_path in zip(scenes, audio_files):
            if os.path.exists(audio_path):
                self.add_sound(audio_path)
            scene_cls.construct(self)
            self.clear()
