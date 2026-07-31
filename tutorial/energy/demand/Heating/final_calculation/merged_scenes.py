import os
import numpy as np
import math
from manim import *

# Default Font configuration for sleek Modern Computer style
FONT_NAME = "Computer Modern"


class ReviewingHeatLosses(Scene):
    def construct(self):
        # Set dark architectural background
        self.camera.background_color = "#0f1115"

        # Color definitions
        ICY_BLUE = "#38BDF8"
        DEEP_BLUE = "#1D4ED8"

        # --- TRANSMISSION SECTION (Icy Blue: #38BDF8) ---
        trans_formula = Text("Φ_trans = U  ·  A  ·  ΔT", font_size=28, color=ICY_BLUE)

        wall = Rectangle(
            height=0.8, width=0.2, color=ICY_BLUE, fill_opacity=0.2, stroke_width=2
        )
        wall_lines = VGroup(
            *[
                Line(
                    wall.get_left() + UP * y + RIGHT * 0.02,
                    wall.get_right() + UP * (y + 0.1) + LEFT * 0.02,
                    color=ICY_BLUE,
                    stroke_width=1.5,
                )
                for y in [-0.25, -0.05, 0.15]
            ]
        )
        wall_arrow = Arrow(
            LEFT * 0.5,
            RIGHT * 0.5,
            color=ICY_BLUE,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.3,
        ).move_to(wall.get_center())
        wall_icon = VGroup(wall, wall_lines, wall_arrow)

        trans_section = VGroup(trans_formula, wall_icon).arrange(
            DOWN, buff=0.3, aligned_edge=LEFT
        )

        # --- LÜFTUNGSSECTION (Deep Blue: #1D4ED8) ---
        vent_formula = Text(
            "Φ_vent = V  ·  n  ·  c_Luft  ·  ΔT", font_size=28, color=DEEP_BLUE
        )

        win_frame = Square(side_length=0.8, color=DEEP_BLUE, stroke_width=2)
        win_cross = VGroup(
            Line(
                win_frame.get_top(),
                win_frame.get_bottom(),
                color=DEEP_BLUE,
                stroke_width=1.5,
            ),
            Line(
                win_frame.get_left(),
                win_frame.get_right(),
                color=DEEP_BLUE,
                stroke_width=1.5,
            ),
        )
        win_arrow = CurvedArrow(
            win_frame.get_left() + DOWN * 0.2,
            win_frame.get_right() + UP * 0.2,
            color=DEEP_BLUE,
            angle=-TAU / 6,
            stroke_width=3,
        )
        win_icon = VGroup(win_frame, win_cross, win_arrow)

        vent_section = VGroup(vent_formula, win_icon).arrange(
            DOWN, buff=0.3, aligned_edge=LEFT
        )

        # --- LAYOUT POSITIONING ---
        left_side = VGroup(trans_section, vent_section).arrange(
            DOWN, buff=0.9, aligned_edge=LEFT
        )
        left_side.to_edge(LEFT, buff=1.0).shift(UP * 0.2)

        # --- GROUPING & TOTAL LOSS (White) ---
        brace = Brace(left_side, RIGHT, color=WHITE, buff=0.35)
        arrow = Arrow(
            brace.get_right(),
            brace.get_right() + RIGHT * 0.9,
            color=WHITE,
            buff=0.1,
            stroke_width=3,
        )

        loss_desc = Text(
            "Gesamtwärmeverlustleistung (DIN V 18599-2)", font_size=16, color=GREY_A
        )
        loss_title = Text("Φ_Verlust", font_size=36, color=WHITE).next_to(
            loss_desc, UP, buff=0.15
        )
        loss_box = VGroup(loss_title, loss_desc)
        loss_box.next_to(arrow, RIGHT, buff=0.25)

        # --- ANIMATION BEATS ---
        self.play(
            FadeIn(trans_formula, shift=DOWN * 0.1),
            FadeIn(wall_icon, shift=RIGHT * 0.2),
            run_time=2.0,
        )
        self.wait(0.8)

        self.play(
            FadeIn(vent_formula, shift=DOWN * 0.1),
            FadeIn(win_icon, shift=RIGHT * 0.2),
            run_time=2.0,
        )
        self.wait(0.8)

        self.play(GrowFromCenter(brace), run_time=1.5)
        self.wait(0.5)

        self.play(GrowArrow(arrow), FadeIn(loss_box, shift=RIGHT * 0.2), run_time=2.0)

        self.wait(2.0)


class Scene2(Scene):
    def construct(self):
        self.camera.background_color = "#0f1115"

        ICY_BLUE = "#38BDF8"
        DEEP_BLUE = "#0284C7"
        PURPLE = "#C084FC"

        # Step 1: Initial Heat Loss Equation and Icons (German)
        phi_loss = Text("Φ_Verlust", font_size=38, color=WHITE)
        eq_1 = Text(" = ", font_size=38, color=WHITE)
        phi_trans = Text("Φ_trans", font_size=38, color=ICY_BLUE)
        plus_1 = Text(" + ", font_size=38, color=WHITE)
        phi_vent = Text("Φ_vent", font_size=38, color=DEEP_BLUE)

        initial_eq = VGroup(phi_loss, eq_1, phi_trans, plus_1, phi_vent).arrange(
            RIGHT, buff=0.12
        )
        initial_eq.move_to(UP * 1.8)

        wall_box = Rectangle(width=1.6, height=1.8, color=ICY_BLUE, stroke_width=2)
        wall_label = Text("Transmission", font_size=18, color=ICY_BLUE).next_to(
            wall_box, DOWN, buff=0.2
        )
        wall_arrow = Arrow(
            LEFT * 0.6,
            RIGHT * 0.6,
            color=ICY_BLUE,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.25,
        ).move_to(wall_box.get_center())
        wall_icon = VGroup(wall_box, wall_label, wall_arrow).move_to(
            LEFT * 2.5 + DOWN * 0.8
        )

        window_box = Rectangle(width=1.6, height=1.8, color=DEEP_BLUE, stroke_width=2)
        window_line_h = Line(
            window_box.get_left(),
            window_box.get_right(),
            color=DEEP_BLUE,
            stroke_width=1,
        )
        window_line_v = Line(
            window_box.get_top(),
            window_box.get_bottom(),
            color=DEEP_BLUE,
            stroke_width=1,
        )
        window_label = Text("Lüftung", font_size=18, color=DEEP_BLUE).next_to(
            window_box, DOWN, buff=0.2
        )
        window_arrow = Arrow(
            LEFT * 0.6,
            RIGHT * 0.6,
            color=DEEP_BLUE,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.25,
        ).move_to(window_box.get_center())
        window_icon = VGroup(
            window_box, window_line_h, window_line_v, window_label, window_arrow
        ).move_to(RIGHT * 2.5 + DOWN * 0.8)

        self.play(Write(initial_eq), Create(wall_icon), Create(window_icon), run_time=2)
        self.wait(1)

        # Step 2: Climate Factor (Gradtagzahlen Gt nach DIN V 18599-10)
        times_symbol = Text("×", font_size=38, color=PURPLE)
        f_climate = Text("F_Klima", font_size=38, color=PURPLE)
        multiplier_group = VGroup(times_symbol, f_climate).arrange(RIGHT, buff=0.18)

        step2_full_eq = (
            VGroup(initial_eq.copy(), multiplier_group.copy())
            .arrange(RIGHT, buff=0.2)
            .move_to(UP * 1.8)
        )
        target_initial_pos = step2_full_eq[0].get_center()
        target_multiplier_pos = step2_full_eq[1].get_center()

        multiplier_group.move_to(target_multiplier_pos)

        climate_label = Text(
            "Klimafaktor (Gradtagstunden nach DIN V 18599-10)",
            font_size=18,
            color=PURPLE,
        )
        climate_label.next_to(multiplier_group, UP, buff=0.45)

        self.play(
            initial_eq.animate.move_to(target_initial_pos),
            FadeIn(multiplier_group),
            FadeIn(climate_label),
            run_time=1.8,
        )
        self.wait(1.2)

        # Step 3: Transformation into Q_Verlust consolidated equation
        q_loss = Text("Q_Verlust", font_size=40, color=WHITE)
        eq_2 = Text(" = ", font_size=40, color=WHITE)
        l_paren = Text("(", font_size=40, color=WHITE)
        phi_trans_c = Text("Φ_trans", font_size=40, color=ICY_BLUE)
        plus_c = Text(" + ", font_size=40, color=WHITE)
        phi_vent_c = Text("Φ_vent", font_size=40, color=DEEP_BLUE)
        r_paren = Text(")", font_size=40, color=WHITE)
        times_c = Text("×", font_size=40, color=PURPLE)
        f_climate_c = Text("F_Klima", font_size=40, color=PURPLE)

        consolidated_eq = (
            VGroup(
                q_loss,
                eq_2,
                l_paren,
                phi_trans_c,
                plus_c,
                phi_vent_c,
                r_paren,
                times_c,
                f_climate_c,
            )
            .arrange(RIGHT, buff=0.12)
            .move_to(ORIGIN)
        )

        paren_terms = VGroup(
            consolidated_eq[2],
            consolidated_eq[3],
            consolidated_eq[4],
            consolidated_eq[5],
            consolidated_eq[6],
        )

        mult_terms = VGroup(consolidated_eq[7], consolidated_eq[8])

        self.play(
            FadeOut(wall_icon),
            FadeOut(window_icon),
            FadeOut(climate_label),
            Transform(initial_eq, paren_terms),
            Transform(multiplier_group, mult_terms),
            FadeIn(consolidated_eq[0]),
            FadeIn(consolidated_eq[1]),
            run_time=2.5,
        )
        self.wait(1)

        final_eq_group = VGroup(
            consolidated_eq[0], consolidated_eq[1], initial_eq, multiplier_group
        )

        unit_text = Text(
            "Jahres-Wärmeverlust [kWh/a] (DIN V 18599-2)", font_size=18, color=GREY_A
        )

        self.play(
            final_eq_group.animate.scale(0.85).to_corner(UL, buff=0.8), run_time=2
        )

        unit_text.next_to(final_eq_group, DOWN, aligned_edge=LEFT, buff=0.25)
        self.play(FadeIn(unit_text), run_time=1)

        self.wait(2)


class ReviewingHeatGains(Scene):
    def construct(self):
        self.camera.background_color = "#0f1115"

        SOLAR_YELLOW = "#FDE047"
        INT_ORANGE = "#F59E0B"
        TEXT_WHITE = "#F3F4F6"
        SUBTEXT_GREY = "#9CA3AF"

        # Title & Subtitle in German
        title = Text("Übersicht der Wärmegewinne", font_size=34, color=TEXT_WHITE)
        title.to_edge(UP, buff=0.6)

        subtitle = Text(
            "Kostenlose Energieeinträge ins Gebäude (DIN V 18599)",
            font_size=18,
            color=SUBTEXT_GREY,
        )
        subtitle.next_to(title, DOWN, buff=0.15)

        # 1. Solar Gain Component
        sun_center = Circle(
            radius=0.22, color=SOLAR_YELLOW, fill_opacity=0.3, stroke_width=2
        )
        rays = VGroup(
            *[
                Line(
                    start=np.array([np.cos(a) * 0.3, np.sin(a) * 0.3, 0]),
                    end=np.array([np.cos(a) * 0.45, np.sin(a) * 0.45, 0]),
                    color=SOLAR_YELLOW,
                    stroke_width=2,
                )
                for a in np.linspace(0, 2 * PI, 8, endpoint=False)
            ]
        )
        sun_icon = VGroup(sun_center, rays)

        solar_text = Text(
            "Q_sol = G · A · F_f · g · F_sh", font_size=24, color=SOLAR_YELLOW
        )
        solar_label = Text(
            "Solarer Wärmegewinn (DIN V 18599-2)", font_size=15, color=SOLAR_YELLOW
        )
        solar_label.next_to(solar_text, DOWN, aligned_edge=LEFT, buff=0.1)
        solar_eq_group = VGroup(solar_text, solar_label)

        solar_group = VGroup(sun_icon, solar_eq_group).arrange(RIGHT, buff=0.4)
        solar_group.move_to(RIGHT * 1.5 + UP * 1.2)

        # 2. Internal Gain Component
        head = Circle(
            radius=0.12, color=INT_ORANGE, fill_opacity=0.4, stroke_width=2
        ).shift(UP * 0.15)
        torso = Arc(
            radius=0.28,
            start_angle=PI * 0.15,
            angle=PI * 0.7,
            color=INT_ORANGE,
            stroke_width=2,
        )
        torso.rotate(PI)
        person_icon = VGroup(head, torso)

        int_text = Text(
            "Q_int = Φ_p + Φ_e + Φ_l", font_size=24, color=INT_ORANGE
        )
        int_label = Text(
            "Interner Wärmegewinn (DIN V 18599-10)", font_size=15, color=INT_ORANGE
        )
        int_label.next_to(int_text, DOWN, aligned_edge=LEFT, buff=0.1)
        int_eq_group = VGroup(int_text, int_label)

        internal_group = VGroup(person_icon, int_eq_group).arrange(RIGHT, buff=0.4)
        internal_group.move_to(RIGHT * 1.5 + DOWN * 1.2)

        # 3. Combined Total Variable Setup
        gains_vgroup = VGroup(solar_group, internal_group)
        brace = Brace(gains_vgroup, direction=LEFT, color=TEXT_WHITE, buff=0.3)

        q_gain_main = Text("Q_Gewinn", font_size=38, color=TEXT_WHITE)
        q_gain_sub = Text("Brutto-Gesamtwärmegewinn", font_size=16, color=SUBTEXT_GREY)
        q_gain_box = VGroup(q_gain_main, q_gain_sub).arrange(
            DOWN, aligned_edge=RIGHT, buff=0.12
        )
        q_gain_box.next_to(brace, LEFT, buff=0.3)

        self.play(Write(title), FadeIn(subtitle, shift=DOWN * 0.2), run_time=1.5)
        self.wait(0.5)

        self.play(
            GrowFromCenter(sun_icon),
            FadeIn(solar_eq_group, shift=RIGHT * 0.3),
            run_time=2.0,
        )
        self.wait(1.0)

        self.play(
            GrowFromCenter(person_icon),
            FadeIn(int_eq_group, shift=RIGHT * 0.3),
            run_time=2.0,
        )
        self.wait(1.0)

        self.play(Create(brace), run_time=1.5)

        self.play(
            Write(q_gain_main), FadeIn(q_gain_sub, shift=LEFT * 0.2), run_time=1.5
        )

        self.play(
            q_gain_main.animate.set_color(SOLAR_YELLOW),
            brace.animate.set_color(SOLAR_YELLOW),
            run_time=0.5,
        )
        self.play(
            q_gain_main.animate.set_color(TEXT_WHITE),
            brace.animate.set_color(TEXT_WHITE),
            run_time=0.5,
        )

        self.wait(2.0)


class Scene4(Scene):
    def construct(self):
        self.camera.background_color = "#0f1115"

        title = Text("Der Ausnutzungsgrad der Wärmegewinne", font_size=28, color=WHITE)
        title.to_edge(UP, buff=0.5)
        self.add(title)

        bulb_center = DOWN * 0.6
        bulb_outer = Circle(radius=0.55, color=WHITE, stroke_width=3).move_to(
            bulb_center
        )
        stem_outer = Rectangle(width=0.45, height=2.6, color=WHITE, stroke_width=3)
        stem_outer.next_to(bulb_outer, UP, buff=-0.15)

        mercury_bulb = Circle(radius=0.45, color="#FDE047", fill_opacity=1.0).move_to(
            bulb_center
        )

        mercury_start = Line(
            start=bulb_center,
            end=bulb_center + UP * 0.8,
            stroke_width=16,
            color="#FDE047",
        )
        mercury_top = Line(
            start=bulb_center,
            end=stem_outer.get_top() + DOWN * 0.15,
            stroke_width=16,
            color="#EF4444",
        )

        red_zone = Rectangle(
            width=0.65, height=0.7, color="#EF4444", fill_opacity=0.35, stroke_width=1.5
        )
        red_zone.move_to(stem_outer.get_top() + DOWN * 0.45)

        red_zone_label = Text("Überhitzungsbereich", font_size=18, color="#EF4444")
        red_zone_label.next_to(red_zone, RIGHT, buff=0.35)

        warning_text = Text(
            "Nicht nutzbare / überschüssige Wärme!", font_size=20, color="#EF4444"
        )
        warning_text.next_to(bulb_outer, DOWN, buff=0.4)

        self.play(
            Create(bulb_outer),
            Create(stem_outer),
            FadeIn(mercury_bulb),
            Create(mercury_start),
            run_time=1.2,
        )
        self.play(FadeIn(red_zone), Write(red_zone_label), run_time=0.8)

        self.play(
            Transform(mercury_start, mercury_top),
            mercury_bulb.animate.set_color("#EF4444"),
            run_time=1.8,
        )
        self.play(Write(warning_text), run_time=0.8)
        self.wait(0.8)

        thermo_group = VGroup(
            bulb_outer,
            stem_outer,
            mercury_bulb,
            mercury_start,
            red_zone,
            red_zone_label,
            warning_text,
        )
        self.play(FadeOut(thermo_group), run_time=0.8)

        # Part 2: Algebraic Insertion of eta_h
        q_gain_lbl = Text("Q_Gewinn", font_size=38, color=WHITE)
        eq_sign = Text(" = ", font_size=38, color=WHITE)
        q_solar = Text("Q_sol", font_size=38, color="#FDE047")
        plus_sign = Text(" + ", font_size=38, color=WHITE)
        q_int = Text("Q_int", font_size=38, color="#F97316")

        initial_eq = VGroup(q_gain_lbl, eq_sign, q_solar, plus_sign, q_int)
        initial_eq.arrange(RIGHT, buff=0.15).move_to(UP * 0.8)

        self.play(Write(initial_eq), run_time=1.2)
        self.wait(0.6)

        q_useful_lbl = Text("Q_nutz", font_size=38, color=WHITE)
        eta_ht = Text("η_h", font_size=42, color="#10B981")
        dot_sym = Text(" · ", font_size=38, color="#10B981")
        l_paren = Text("(", font_size=44, color="#10B981")
        r_paren = Text(")", font_size=44, color="#10B981")

        eq_sign_target = Text(" = ", font_size=38, color=WHITE)
        q_solar_target = Text("Q_sol", font_size=38, color="#FDE047")
        plus_sign_target = Text(" + ", font_size=38, color=WHITE)
        q_int_target = Text("Q_int", font_size=38, color="#F97316")

        target_group = (
            VGroup(
                q_useful_lbl,
                eq_sign_target,
                eta_ht,
                dot_sym,
                l_paren,
                q_solar_target,
                plus_sign_target,
                q_int_target,
                r_paren,
            )
            .arrange(RIGHT, buff=0.12)
            .move_to(UP * 0.8)
        )

        self.play(
            Transform(q_gain_lbl, q_useful_lbl),
            eq_sign.animate.move_to(eq_sign_target),
            q_solar.animate.move_to(q_solar_target),
            plus_sign.animate.move_to(plus_sign_target),
            q_int.animate.move_to(q_int_target),
            run_time=1.2,
        )

        self.play(
            FadeIn(eta_ht, shift=DOWN * 0.2),
            FadeIn(dot_sym),
            FadeIn(l_paren, shift=RIGHT * 0.1),
            FadeIn(r_paren, shift=LEFT * 0.1),
            run_time=1.2,
        )
        self.wait(0.6)

        # Part 3: Explanation & Focus on Ausnutzungsgrad (DIN V 18599-2)
        eta_box = SurroundingRectangle(
            eta_ht, color="#10B981", buff=0.12, corner_radius=0.1
        )

        eta_title = Text(
            "η_h : Ausnutzungsgrad der Wärmegewinne (DIN V 18599-2)",
            font_size=22,
            color="#10B981",
        )
        eta_title.move_to(DOWN * 0.6)

        eta_line1 = Text(
            "Gibt den Anteil der Wärmegewinne an (0 bis 100%),",
            font_size=18,
            color=GREY_A,
        )
        eta_line2 = Text(
            "der tatsächlich zur Deckung des Heizwärmebedarfs beiträgt.",
            font_size=18,
            color=GREY_A,
        )
        eta_desc = (
            VGroup(eta_line1, eta_line2)
            .arrange(DOWN, buff=0.12)
            .next_to(eta_title, DOWN, buff=0.3)
        )

        self.play(Create(eta_box), Write(eta_title), run_time=1.0)
        self.play(FadeIn(eta_desc, shift=UP * 0.15), run_time=1.0)

        self.wait(2.0)


class UltimateEnergyBalance(Scene):
    def construct(self):
        title = Text(
            "Die Hauptgleichung des Heizwärmebedarfs (DIN V 18599)",
            font_size=26,
            color=WHITE,
        )
        title.to_edge(UP, buff=0.5)
        self.play(Write(title))
        self.wait(0.5)

        P = DOWN * 0.5

        fulcrum = Polygon(
            P,
            P + DOWN * 1.2 + LEFT * 0.6,
            P + DOWN * 1.2 + RIGHT * 0.6,
            color=GREY,
            fill_opacity=0.5,
        )
        base = Line(
            P + DOWN * 1.2 + LEFT * 1.2,
            P + DOWN * 1.2 + RIGHT * 1.2,
            color=GREY,
            stroke_width=4,
        )

        beam = Line(P + LEFT * 2.2, P + RIGHT * 2.2, color=WHITE, stroke_width=5)

        left_string = Line(
            P + LEFT * 2.2, P + LEFT * 2.2 + DOWN * 1.2, color=GREY_B, stroke_width=2
        )
        left_plate = Line(
            P + LEFT * 2.8 + DOWN * 1.2,
            P + LEFT * 1.6 + DOWN * 1.2,
            color=WHITE,
            stroke_width=4,
        )
        left_pan = VGroup(left_string, left_plate)

        right_string = Line(
            P + RIGHT * 2.2, P + RIGHT * 2.2 + DOWN * 1.2, color=GREY_B, stroke_width=2
        )
        right_plate = Line(
            P + RIGHT * 2.8 + DOWN * 1.2,
            P + RIGHT * 1.6 + DOWN * 1.2,
            color=WHITE,
            stroke_width=4,
        )
        right_pan = VGroup(right_string, right_plate)

        beam_assembly = VGroup(beam, left_pan, right_pan)

        self.play(Create(fulcrum), Create(base), Create(beam_assembly))
        self.wait(0.5)

        q_loss_tag = Text("Q_Verlust", color="#3B82F6", font_size=26)
        q_loss_tag.move_to(P + LEFT * 2.2 + UP * 2.0)

        self.play(FadeIn(q_loss_tag, shift=DOWN))
        self.play(q_loss_tag.animate.move_to(left_plate.get_center() + UP * 0.35))

        scale_with_loss = VGroup(beam_assembly, q_loss_tag)
        self.play(
            Rotate(scale_with_loss, angle=16 * DEGREES, about_point=P, run_time=1.2)
        )
        self.wait(0.5)

        eta_text = Text("η_h", color="#22C55E", font_size=26)
        times_text = Text(" · ", color=WHITE, font_size=26)
        q_g_text = Text("Q_Gewinn", color="#EAB308", font_size=26)
        q_gain_tag = VGroup(eta_text, times_text, q_g_text).arrange(RIGHT, buff=0.08)
        q_gain_tag.move_to(P + RIGHT * 2.2 + UP * 2.0)

        self.play(FadeIn(q_gain_tag, shift=DOWN))
        self.play(q_gain_tag.animate.move_to(right_plate.get_center() + UP * 0.35))

        scale_all = VGroup(scale_with_loss, q_gain_tag)
        self.play(Rotate(scale_all, angle=-10 * DEGREES, about_point=P, run_time=1.2))
        self.wait(1)

        # Master Equation (German standard notation: Q_h = Q_b = Q_Verlust - eta_h * Q_Gewinn)
        q_heat_text = Text("Q_h", color="#EF4444", font_size=38, weight=BOLD)
        eq_sign = Text(" = ", color=WHITE, font_size=36)
        q_loss_eq = Text("Q_Verlust", color="#3B82F6", font_size=34)
        minus_sign = Text(" - ", color=WHITE, font_size=36)
        eta_eq = Text("η_h", color="#22C55E", font_size=36)
        dot_sign = Text(" · ", color=WHITE, font_size=36)
        q_gain_eq = Text("Q_Gewinn", color="#EAB308", font_size=34)

        master_eq = VGroup(
            q_heat_text, eq_sign, q_loss_eq, minus_sign, eta_eq, dot_sign, q_gain_eq
        ).arrange(RIGHT, buff=0.12)
        master_eq.move_to(UP * 0.8)

        scale_everything = VGroup(fulcrum, base, scale_all)
        self.play(ReplacementTransform(scale_everything, master_eq), run_time=1.5)
        self.wait(1)

        # Expanded German DIN V 18599 heating demand equation
        q_heat_exp = Text("Q_h", color="#EF4444", font_size=28, weight=BOLD)
        eq_exp = Text(" = ", color=WHITE, font_size=28)
        loss_exp = Text("(Q_trans + Q_vent)", color="#3B82F6", font_size=28)
        minus_exp = Text(" - ", color=WHITE, font_size=28)
        eta_exp = Text("η_h", color="#22C55E", font_size=28)
        dot_exp = Text(" · ", color=WHITE, font_size=28)
        gain_exp = Text("(Q_sol + Q_int)", color="#EAB308", font_size=28)

        expanded_eq = VGroup(
            q_heat_exp, eq_exp, loss_exp, minus_exp, eta_exp, dot_exp, gain_exp
        ).arrange(RIGHT, buff=0.1)
        expanded_eq.move_to(DOWN * 1.0)

        self.play(FadeIn(VGroup(q_heat_exp, eq_exp), shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(FadeIn(loss_exp, shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(
            FadeIn(VGroup(minus_exp, eta_exp, dot_exp), shift=UP * 0.3), run_time=0.8
        )
        self.wait(0.3)
        self.play(FadeIn(gain_exp, shift=UP * 0.3), run_time=0.8)

        self.wait(5)

        self.play(FadeOut(VGroup(title, master_eq, expanded_eq)), run_time=1.5)
        self.wait(0.5)


class FullFinalCalculationVideo(Scene):
    def construct(self):
        scenes = [
            ReviewingHeatLosses,
            Scene2,
            ReviewingHeatGains,
            Scene4,
            UltimateEnergyBalance,
        ]
        base_dir = os.path.dirname(os.path.abspath(__file__))
        audio_files = [
            os.path.join(base_dir, f"scene_{i}_audio.mp3") for i in range(1, 6)
        ]

        for scene_cls, audio_path in zip(scenes, audio_files):
            if os.path.exists(audio_path):
                self.add_sound(audio_path)
            scene_cls.construct(self)
            self.clear()
