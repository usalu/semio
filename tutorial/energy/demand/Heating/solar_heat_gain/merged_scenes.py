import os
import numpy as np
import math
from manim import *


class Scene1Transition(Scene):
    def construct(self):
        # 1. Background setup
        self.camera.background_color = "#0f1115"

        # Color definitions
        COLOR_YELLOW = "#FFFF00"
        COLOR_GOLD = "#FFD700"
        COLOR_BLUE_A = "#52B2BF"
        COLOR_GREY = "#888888"

        # 2. Section Title
        title = Text(
            "Von Wärmeverlust zu solarem Wärmegewinn", font_size=32, color=WHITE
        )
        title.to_edge(UP, buff=0.4)

        subtitle = Text(
            "Von passivem Wärmeverlust zu aktiver Umweltenergie (DIN 4108-2)",
            font_size=18,
            color=COLOR_GREY,
        )
        subtitle.next_to(title, DOWN, buff=0.15)

        # 3. Architectural Cross-Section of House
        house_center = DOWN * 0.8

        walls = Rectangle(width=4.0, height=2.2, color=COLOR_GREY, stroke_width=2.5)
        walls.move_to(house_center)

        roof = Polygon(
            house_center + LEFT * 2.3 + UP * 1.1,
            house_center + UP * 2.3,
            house_center + RIGHT * 2.3 + UP * 1.1,
            color=COLOR_GREY,
            stroke_width=2.5,
        )

        floor_line = Line(
            house_center + LEFT * 2.8 + DOWN * 1.1,
            house_center + RIGHT * 2.8 + DOWN * 1.1,
            color=COLOR_GREY,
            stroke_width=2,
        )

        window = Rectangle(width=1.0, height=1.1, color=COLOR_GREY, stroke_width=2)
        window.move_to(house_center + RIGHT * 1.1 + UP * 0.1)

        window_cross_h = Line(
            window.get_left(), window.get_right(), color=COLOR_GREY, stroke_width=1
        )
        window_cross_v = Line(
            window.get_top(), window.get_bottom(), color=COLOR_GREY, stroke_width=1
        )
        window_group = VGroup(window, window_cross_h, window_cross_v)

        door = Rectangle(width=0.7, height=1.2, color=COLOR_GREY, stroke_width=2)
        door.move_to(house_center + LEFT * 1.0 + DOWN * 0.5)

        house = VGroup(walls, roof, floor_line, window_group, door)

        # 4. Outward Blue Heat Loss Arrows
        arrow_left = CurvedArrow(
            house_center + LEFT * 2.1 + DOWN * 0.2,
            house_center + LEFT * 3.2 + UP * 0.2,
            angle=TAU / 12,
            color=COLOR_BLUE_A,
        )
        arrow_right = CurvedArrow(
            house_center + RIGHT * 2.1 + DOWN * 0.2,
            house_center + RIGHT * 3.2 + UP * 0.2,
            angle=-TAU / 12,
            color=COLOR_BLUE_A,
        )
        arrow_roof_left = CurvedArrow(
            house_center + LEFT * 1.2 + UP * 1.8,
            house_center + LEFT * 2.2 + UP * 2.6,
            angle=TAU / 12,
            color=COLOR_BLUE_A,
        )
        arrow_roof_right = CurvedArrow(
            house_center + RIGHT * 1.2 + UP * 1.8,
            house_center + RIGHT * 2.2 + UP * 2.6,
            angle=-TAU / 12,
            color=COLOR_BLUE_A,
        )
        arrow_top = CurvedArrow(
            house_center + UP * 2.4,
            house_center + UP * 3.2 + RIGHT * 0.4,
            angle=-TAU / 12,
            color=COLOR_BLUE_A,
        )

        loss_arrows = VGroup(
            arrow_left, arrow_right, arrow_roof_left, arrow_roof_right, arrow_top
        )

        loss_label = Text("Wärmeverlust (Q_loss)", color=COLOR_BLUE_A, font_size=20)
        loss_label.next_to(house, RIGHT, buff=0.6).shift(UP * 0.5)

        # 5. Sun and Radiation Elements (Top Left)
        sun_pos = LEFT * 4.5 + UP * 2.2
        sun_core = Dot(sun_pos, radius=0.4, color=COLOR_YELLOW)
        sun_glow = Dot(sun_pos, radius=0.65, color=COLOR_GOLD, fill_opacity=0.4)

        # Concentric radiant rings around sun
        ring1 = Circle(
            radius=0.7, color=COLOR_YELLOW, stroke_width=1.5, stroke_opacity=0.8
        ).move_to(sun_pos)
        ring2 = Circle(
            radius=1.0, color=COLOR_GOLD, stroke_width=1.2, stroke_opacity=0.5
        ).move_to(sun_pos)
        ring3 = Circle(
            radius=1.3, color=COLOR_YELLOW, stroke_width=1.0, stroke_opacity=0.25
        ).move_to(sun_pos)
        sun_rings = VGroup(ring1, ring2, ring3)

        # Short radial sunburst rays around the core
        sun_burst_rays = VGroup()
        for angle in np.linspace(0, TAU, 12, endpoint=False):
            start_p = sun_pos + np.array([np.cos(angle) * 0.5, np.sin(angle) * 0.5, 0])
            end_p = sun_pos + np.array([np.cos(angle) * 0.8, np.sin(angle) * 0.8, 0])
            sun_burst_rays.add(Line(start_p, end_p, color=COLOR_GOLD, stroke_width=2))

        # Solar Radiation Lines pointing at key house surfaces (lines, not arrows)
        targets = [
            house_center + LEFT * 1.5 + UP * 1.8,  # roof left slope
            house_center + UP * 2.3,  # roof peak
            house_center + RIGHT * 0.8 + UP * 1.7,  # roof right slope
            window.get_center(),  # window
            house_center + LEFT * 1.8 + UP * 0.3,  # wall left
        ]

        radiation_lines = VGroup()
        for target in targets:
            start_pt = sun_pos + (target - sun_pos) * 0.15
            ray = Line(
                start_pt,
                target,
                color=COLOR_YELLOW,
                stroke_width=2.5,
                stroke_opacity=0.85,
            )
            radiation_lines.add(ray)

        gain_label = Text(
            "Solarer Wärmegewinn (Q_gain)", color=COLOR_YELLOW, font_size=20
        )
        gain_label.next_to(house, RIGHT, buff=0.6).shift(UP * 0.5)

        # --- ANIMATION BEATS ---

        # Beat 1: Draw title, house cross-section, and blue heat loss arrows
        self.play(Write(title), FadeIn(subtitle), run_time=1.2)
        self.play(Create(house), Create(loss_arrows), FadeIn(loss_label), run_time=2.0)
        self.wait(1.0)

        # Beat 2: Outward heat loss arrows fade out
        self.play(FadeOut(loss_arrows), FadeOut(loss_label), run_time=1.2)
        self.wait(0.3)

        # Beat 3: Glowing sun appears with radiant rings and radiating lines
        self.play(
            FadeIn(sun_glow),
            FadeIn(sun_core, scale=0.6),
            Create(sun_rings),
            Create(sun_burst_rays),
            run_time=1.5,
        )

        # Animate radiation lines extending smoothly to the house with pulsing rings
        self.play(
            LaggedStart(*[Create(line) for line in radiation_lines], lag_ratio=0.15),
            FadeIn(gain_label),
            ring1.animate.scale(1.2).set_stroke(opacity=0.3),
            ring2.animate.scale(1.15).set_stroke(opacity=0.2),
            ring3.animate.scale(1.1).set_stroke(opacity=0.1),
            run_time=2.0,
        )

        # Continuous animated radiation pulse effect
        self.play(
            radiation_lines.animate.set_stroke(width=4, opacity=0.1),
            ring1.animate.scale(0.85).set_stroke(opacity=0.2),
            ring2.animate.scale(0.87).set_stroke(opacity=0.2),
            run_time=0.8,
        )
        self.play(
            radiation_lines.animate.set_stroke(width=2.5, opacity=0.1),
            ring1.animate.scale(1.15).set_stroke(opacity=0.1),
            ring2.animate.scale(1.15).set_stroke(opacity=0.1),
            run_time=0.8,
        )

        # Final hold
        self.wait(2)


class Scene2IrradianceAndArea(Scene):
    def construct(self):
        # 1. Background & Theme Setup
        self.camera.background_color = "#0f1115"

        COLOR_G = "#FACC15"  # Bright Yellow (Irradiance)
        COLOR_A = "#38BDF8"  # Cyan / Vivid Blue (Area)
        COLOR_WIN = "#00F0FF"  # Glass Cyan
        COLOR_WALL = "#64748B"  # Slate Gray Wall
        COLOR_FRAME = "#94A3B8"  # Window Frame Gray
        COLOR_FF = "#F97316"  # Vivid Orange for Frame Factor (F_f)
        COLOR_TEXT = "#F8FAFC"  # Off-white

        # 2. Initial Text Animation
        intro_title = Text(
            "Bestrahlungsstärke, Fensterfläche & Rahmenfaktor",
            font_size=24,
            color=COLOR_TEXT,
        )
        intro_title.to_edge(UP, buff=0.3)

        self.play(Write(intro_title), run_time=1.0)
        self.play(
            intro_title.animate.scale(0.85).to_edge(RIGHT).shift(DOWN * 0.15),
            run_time=0.8,
        )

        # 3. Base Wall and Full Gross Window Aperture (Positioned slightly right of center to leave room for left labels)
        house_center = LEFT * 0.5 + DOWN * 0.4

        wall_outer = Rectangle(
            width=3.4,
            height=4.2,
            color=COLOR_WALL,
            fill_color="#1E293B",
            fill_opacity=0.9,
            stroke_width=3,
        ).move_to(house_center)

        # Full gross window opening (before adding frame)
        win_gross = Rectangle(
            width=1.6,
            height=2.4,
            color=COLOR_A,
            fill_color=COLOR_A,
            fill_opacity=0.1,
            stroke_width=3,
        ).move_to(house_center)

        gross_label = Text(
            "Gesamte Fensteröffnung", font_size=13, color=COLOR_A
        ).next_to(wall_outer, DOWN, buff=0.25)

        self.play(
            Create(wall_outer),
            DrawBorderThenFill(win_gross),
            FadeIn(gross_label),
            run_time=1.4,
        )
        self.wait(0.6)

        # 4. Step 1: Solar Radiation Shines on the Window (G)
        sun_center = RIGHT * 5.2 + UP * 1.8
        sun_core = Dot(sun_center, radius=0.4, color=COLOR_G)
        sun_ring1 = Circle(
            radius=0.6, color=COLOR_G, stroke_width=2, stroke_opacity=0.6
        ).move_to(sun_center)
        sun_ring2 = Circle(
            radius=0.8, color=COLOR_G, stroke_width=1, stroke_opacity=0.3
        ).move_to(sun_center)
        sun_group = VGroup(sun_core, sun_ring1, sun_ring2)

        self.play(FadeIn(sun_group, scale=0.8), run_time=0.8)

        # Diagonal rays shooting toward the window surface
        top_pt = win_gross.get_top()
        bot_pt = win_gross.get_bottom()
        mid_pt = win_gross.get_center()

        ray_configs = [
            (top_pt + RIGHT * 0.05, 3, 0.4),
            (mid_pt + UP * 0.5, 3, 0.6),
            (mid_pt, 5, 0.85),
            (mid_pt + DOWN * 0.5, 3, 0.6),
            (bot_pt + RIGHT * 0.05, 3, 0.4),
        ]

        rays = VGroup(
            *[
                Line(sun_center, target_pt, color=COLOR_G, stroke_width=sw).set_opacity(
                    op
                )
                for target_pt, sw, op in ray_configs
            ]
        )

        self.play(
            LaggedStart(*[Create(ray) for ray in rays], lag_ratio=0.15, run_time=1.4)
        )

        # Continuous gentle pulsing animation for rays
        for i, ray in enumerate(rays):
            base_op = ray_configs[i][2]
            ray.add_updater(
                lambda m, dt, b=base_op, phase=i * 0.8: m.set_opacity(
                    b + 0.15 * np.sin(3 * self.time + phase)
                )
            )

        # Irradiance Label 'G' placed clearly above rays line
        g_mid_pos = (sun_center + mid_pt) / 2 + UP * 0.9
        label_G = Text("G", font_size=36, color=COLOR_G, weight=BOLD).move_to(g_mid_pos)
        sub_G = Text("Bestrahlungsstärke", font_size=16, color=COLOR_G).next_to(
            label_G, DOWN, buff=0.15
        )

        self.play(
            FadeIn(label_G, shift=UP * 0.2), FadeIn(sub_G, shift=UP * 0.2), run_time=0.8
        )
        self.wait(0.8)

        # Float 'G' to Top-Left Position in Top Bar
        target_G_pos = UP * 2.8 + LEFT * 4.8
        top_bar_G = Text("G", font_size=32, color=COLOR_G, weight=BOLD).move_to(
            target_G_pos
        )

        self.play(FadeOut(sub_G), Transform(label_G, top_bar_G), run_time=1.0)

        # 5. Step 2: Show Full Gross Window Area 'A'
        # Position label & description centered relative to each other, outside wall_outer
        label_A = Text("A", font_size=32, color=COLOR_A, weight=BOLD)
        sub_A = Text(
            "Gesamte Fensterfläche (A)", font_size=15, color=COLOR_A, weight=BOLD
        )
        group_A = (
            VGroup(label_A, sub_A)
            .arrange(DOWN, buff=0.15)
            .next_to(wall_outer, LEFT, buff=0.45)
        )

        self.play(
            win_gross.animate.set_fill(COLOR_A, opacity=0.45),
            FadeIn(label_A, shift=RIGHT * 0.2),
            Transform(gross_label, sub_A),
            run_time=1.2,
        )
        self.wait(1.0)

        # Float 'A' to Top Bar next to 'G' as 'G · A'
        dot1_tex = Text("·", font_size=32, color=COLOR_TEXT).next_to(
            top_bar_G, RIGHT, buff=0.2
        )
        top_bar_A = Text("A", font_size=32, color=COLOR_A, weight=BOLD).next_to(
            dot1_tex, RIGHT, buff=0.2
        )

        self.play(
            FadeOut(gross_label),
            FadeIn(dot1_tex),
            Transform(label_A, top_bar_A),
            run_time=1.0,
        )
        self.wait(0.8)

        # 6. Step 3: Show Window Has Frame & Introduce Frame Factor (F_f)
        win_outer_frame = Rectangle(
            width=1.6,
            height=2.4,
            color=COLOR_FRAME,
            fill_color="#334155",
            fill_opacity=1.0,
            stroke_width=4,
        ).move_to(house_center)

        win_glass = Rectangle(
            width=1.1,
            height=1.8,
            color=COLOR_WIN,
            fill_color=COLOR_WIN,
            fill_opacity=0.35,
            stroke_width=2,
        ).move_to(house_center)

        h_mullion = Line(
            win_glass.get_left(),
            win_glass.get_right(),
            color=COLOR_FRAME,
            stroke_width=2,
        )
        v_mullion = Line(
            win_glass.get_top(),
            win_glass.get_bottom(),
            color=COLOR_FRAME,
            stroke_width=2,
        )
        mullions = VGroup(h_mullion, v_mullion)

        frame_desc = Text(
            "Undurchsichtiger Rahmen blockiert Strahlung",
            font_size=18,
            color=COLOR_FRAME,
        ).next_to(wall_outer, DOWN, buff=0.25)

        self.play(
            FadeOut(win_gross),
            GrowFromCenter(win_outer_frame),
            Create(win_glass),
            Create(mullions),
            FadeIn(frame_desc),
            run_time=1.5,
        )
        self.wait(1.0)

        # Highlight Transparent Glass Fraction (F_f)
        ff_box = win_glass.copy().set_color(COLOR_FF).set_fill(COLOR_FF, opacity=0.45)

        # Position F_f label & description centered relative to each other, outside wall_outer
        label_Ff = Text("F_f", font_size=28, color=COLOR_FF, weight=BOLD)
        sub_Ff = Text(
            "Rahmenfaktor (Glasanteil F_f ≈ 0,7–0,8)",
            font_size=14,
            color=COLOR_FF,
            weight=BOLD,
        )
        group_Ff = (
            VGroup(label_Ff, sub_Ff)
            .arrange(DOWN, buff=0.15)
            .next_to(wall_outer, LEFT, buff=0.45)
        )

        self.play(
            Create(ff_box),
            FadeIn(label_Ff, shift=LEFT * 0.2),
            FadeIn(sub_Ff, shift=LEFT * 0.2),
            run_time=1.2,
        )
        self.play(
            ff_box.animate.set_fill(opacity=0.8), rate_func=there_and_back, run_time=1.2
        )
        self.wait(1.0)

        # 7. Step 4: Add Frame Factor F_f into Formula at Top Bar (G · A · F_f)
        dot2_tex = Text("·", font_size=32, color=COLOR_TEXT).next_to(
            top_bar_A, RIGHT, buff=0.2
        )
        top_bar_Ff = Text("F_f", font_size=32, color=COLOR_FF, weight=BOLD).next_to(
            dot2_tex, RIGHT, buff=0.2
        )

        self.play(
            FadeOut(sub_Ff),
            FadeOut(frame_desc),
            FadeOut(ff_box),
            FadeIn(dot2_tex),
            Transform(label_Ff, top_bar_Ff),
            run_time=1.2,
        )

        # Subtitle banner for equation meaning centered over 'A' in the top bar
        eq_meaning = Text(
            "Wirksame Solarleistung (G · A · F_f)", font_size=16, color=COLOR_TEXT
        ).next_to(top_bar_A, UP, buff=0.25)
        self.play(FadeIn(eq_meaning, shift=UP * 0.1), run_time=0.8)

        self.wait(2.5)


class Scene3GValue(Scene):
    def construct(self):
        self.camera.background_color = "#0f1115"

        # --- Top Equation Bar Initial (Continuing from Scene 2: G · A · F_f) ---
        g_inc_text = Text("G", color=YELLOW, font_size=32)
        dot1_text = Text(" · ", color=WHITE, font_size=32)
        a_area_text = Text("A", color=BLUE, font_size=32)
        dot2_text = Text(" · ", color=WHITE, font_size=32)
        ff_text = Text("F_f", color="#F97316", font_size=32, weight=BOLD)

        top_eq_1 = VGroup(
            g_inc_text, dot1_text, a_area_text, dot2_text, ff_text
        ).arrange(RIGHT, buff=0.15)
        top_eq_1.to_edge(UP, buff=0.4)

        self.add(top_eq_1)

        # --- Beat 1: Architectural Window Wall Cross-Section ---
        # Upper and lower house wall cuts
        wall_upper = Rectangle(
            width=1.0,
            height=0.9,
            stroke_color=GREY_B,
            stroke_width=2,
            fill_opacity=0.05,
        ).shift(UP * 1.8)
        wall_lower = Rectangle(
            width=1.0,
            height=0.9,
            stroke_color=GREY_B,
            stroke_width=2,
            fill_opacity=0.05,
        ).shift(DOWN * 2.0)

        # Wall hatching / section detail lines
        wall_lines = VGroup(
            Line(
                wall_upper.get_corner(DL),
                wall_upper.get_corner(UR),
                stroke_color=GREY_C,
                stroke_width=1,
            ),
            Line(
                wall_lower.get_corner(DL),
                wall_lower.get_corner(UR),
                stroke_color=GREY_C,
                stroke_width=1,
            ),
        )

        # Frame profiles holding the glass (centered horizontally with the walls)
        frame_top = (
            Rectangle(
                width=0.6,
                height=0.3,
                stroke_color=WHITE,
                stroke_width=2,
                fill_opacity=0.1,
            )
            .next_to(wall_upper, DOWN, buff=0)
            .match_x(wall_upper)
        )

        frame_bot = (
            Rectangle(
                width=0.6,
                height=0.3,
                stroke_color=WHITE,
                stroke_width=2,
                fill_opacity=0.1,
            )
            .next_to(wall_lower, UP, buff=0)
            .match_x(wall_lower)
        )

        # Cyan Glass pane section (spans strictly between the inner edges of frame_top and frame_bot)
        glass_height = frame_top.get_bottom()[1] - frame_bot.get_top()[1]
        glass_pane = Rectangle(
            width=0.25,
            height=glass_height,
            stroke_color="#00FFFF",
            stroke_width=2,
            fill_opacity=0.12,
            fill_color="#00FFFF",
        )
        glass_pane.move_to(VGroup(frame_top, frame_bot).get_center())

        # Context Labels (Exterior / Interior) - positioned clearly away from top equation
        ext_label = Text("Außen", color=GREY_B, font_size=16).shift(
            LEFT * 4.5 + UP * 2.6
        )
        int_label = Text("Innen", color=GREY_B, font_size=16).shift(
            RIGHT * 4.5 + UP * 2.6
        )
        glass_title = Text("Glasquerschnitt", color="#00FFFF", font_size=15).next_to(
            wall_upper, RIGHT, buff=0.4
        )

        self.play(
            Create(wall_upper),
            Create(wall_lower),
            Create(wall_lines),
            Create(frame_top),
            Create(frame_bot),
            Create(glass_pane),
            Write(ext_label),
            Write(int_label),
            Write(glass_title),
            run_time=1.5,
        )
        self.wait(0.4)

        # --- Beat 2: Incoming Yellow Ray ---
        hit_pt = glass_pane.get_left() + DOWN * 0.1
        inc_start = LEFT * 5.2 + UP * 0.8

        inc_ray = Line(inc_start, hit_pt, color=YELLOW, stroke_width=5)
        inc_angle = inc_ray.get_angle()
        inc_label = Text("Sonnenstrahlung (G · A)", color=YELLOW, font_size=15)
        inc_label.rotate(inc_angle)
        inc_label.next_to(inc_ray.get_center(), UP)
        inc_label.shift(LEFT * 1.0)

        self.play(
            Create(inc_ray),
            FadeIn(inc_label, shift=inc_ray.get_vector() * 0.2),
            run_time=1.2,
        )
        self.wait(0.4)

        # --- Beat 3: Split Rays (Reflected, Absorbed, Transmitted) ---
        # 1. Reflected ray (bouncing back outdoors, angled upward clearly without crossing incoming ray label)
        refl_end = LEFT * 5.0 + UP * 1.8
        refl_ray = Line(hit_pt, refl_end, color=WHITE, stroke_width=4)
        # Ensure angle is right-reading (readable left-to-right, non-mirrored)
        refl_angle = refl_ray.get_angle()
        if refl_angle > PI / 2 or refl_angle < -PI / 2:
            refl_angle += PI
        refl_label = Text("Reflektiert", color=WHITE, font_size=14)
        refl_label.rotate(refl_angle)
        refl_label.next_to(refl_ray.get_center(), UP, buff=0.15)

        # 2. Absorbed ray (inside glass, extending down to the bottom corner of the glass pane)
        abs_end = glass_pane.get_corner(DR)
        abs_ray = Line(hit_pt, abs_end, color=RED, stroke_width=4)
        abs_label = Text("Absorbiert", color=RED, font_size=14).next_to(
            abs_ray.get_center(), RIGHT, buff=0.2
        )

        # 3. Transmitted ray (passing through to interior)
        trans_pt_right = hit_pt + RIGHT * glass_pane.width
        trans_ray_inside = Line(hit_pt, trans_pt_right, color=YELLOW, stroke_width=5)
        trans_end = RIGHT * 5.0 + DOWN * 0.9
        trans_ray_out = Line(trans_pt_right, trans_end, color=YELLOW, stroke_width=5)
        trans_ray = VGroup(trans_ray_inside, trans_ray_out)

        trans_angle = trans_ray_out.get_angle()
        if trans_angle > PI / 2 or trans_angle < -PI / 2:
            trans_angle += PI
        trans_label = Text("Transmittiert", color=YELLOW, font_size=15)
        trans_label.rotate(trans_angle)
        trans_label.next_to(trans_ray_out.get_center(), UP, buff=0.15)

        self.play(
            LaggedStart(
                AnimationGroup(
                    Create(refl_ray),
                    FadeIn(refl_label, shift=refl_ray.get_vector() * 0.2),
                ),
                AnimationGroup(Create(abs_ray), FadeIn(abs_label, shift=DOWN * 0.1)),
                AnimationGroup(
                    Create(trans_ray),
                    FadeIn(trans_label, shift=trans_ray_out.get_vector() * 0.2),
                ),
                lag_ratio=0.35,
                run_time=2.2,
            )
        )
        self.wait(0.5)

        # --- Beat 4: Green 'g' Label and Equation Assembly ---
        g_label = Text("g", color=GREEN, font_size=32, weight="BOLD")
        g_label.next_to(trans_ray_out, DOWN, buff=0.75)
        g_sub = Text(
            "g-Wert (Transmissionsgrad nach DIN V 18599)", color=GREEN, font_size=14
        ).next_to(g_label, DOWN, buff=0.15)

        self.play(FadeIn(g_label), Write(g_sub))
        self.wait(0.4)

        # --- Beat 5: Dynamic Solar Energy Flow Animation (Demonstrating g-value visually) ---
        # Flowing energy pulse on incoming ray
        pulse_inc = Dot(color=YELLOW, radius=0.12)
        # Flowing pulse on transmitted ray (passing into room)
        pulse_trans = Dot(color=YELLOW, radius=0.12)
        # Flowing pulse on reflected ray (bouncing back outside)
        pulse_refl = Dot(color=WHITE, radius=0.09)

        glow_label = Text(
            "Transmission (~60-80% Heat Gain)", color=GREEN, font_size=13
        ).next_to(trans_ray_out, DOWN, buff=0.15)
        self.play(MoveAlongPath(pulse_inc, inc_ray, rate_func=linear, run_time=1.0))
        self.play(
            FadeOut(pulse_inc),
            MoveAlongPath(pulse_trans, trans_ray_out, rate_func=linear, run_time=1.2),
            MoveAlongPath(pulse_refl, refl_ray, rate_func=linear, run_time=1.2),
            FadeIn(glow_label),
            run_time=1.2,
        )
        self.play(Indicate(g_label, color=GREEN, scale_factor=1.4), run_time=0.8)
        self.wait(0.6)

        # Target layout for top equation bar: G · A · F_f · g
        dot3_text = Text(" · ", color=WHITE, font_size=32)

        full_eq_target = (
            VGroup(
                Text("G", color=YELLOW, font_size=32),
                Text(" · ", color=WHITE, font_size=32),
                Text("A", color=BLUE, font_size=32),
                Text(" · ", color=WHITE, font_size=32),
                Text("F_f", color="#F97316", font_size=32, weight=BOLD),
                dot3_text,
                Text("g", color=GREEN, font_size=32, weight="BOLD"),
            )
            .arrange(RIGHT, buff=0.15)
            .to_edge(UP, buff=0.4)
        )

        self.play(
            FadeOut(g_sub),
            FadeOut(pulse_trans),
            FadeOut(pulse_refl),
            FadeOut(glow_label),
            top_eq_1.animate.move_to(full_eq_target[:5].get_center()),
            FadeIn(dot3_text.move_to(full_eq_target[5].get_center())),
            g_label.animate.move_to(full_eq_target[6].get_center()),
            run_time=1.4,
        )

        self.wait(2)


class SeasonalSunAngles(Scene):
    def construct(self):
        # Background configuration
        self.camera.background_color = "#0f1115"

        # Title and Subtitles positioned safely at top
        title = Text(
            "Saisonale Sonnenwinkel & passives Solardesign", font_size=24, color=WHITE
        )
        title.to_edge(UP, buff=0.25)
        self.add(title)

        tag_summer = Text(
            "Sommer: Steiler hoher Sonnenwinkel", font_size=18, color=YELLOW
        )
        tag_summer.next_to(title, DOWN, buff=0.15)

        tag_winter = Text(
            "Winter: Flacher niedriger Sonnenwinkel", font_size=18, color=BLUE_B
        )
        tag_winter.next_to(title, DOWN, buff=0.15)

        # Geometry coordinates for House
        floor_y = -1.8
        back_wall_x = -2.0
        front_wall_x = 1.2
        roof_y = 0.8
        overhang_x = 1.7
        sill_y = -0.6
        win_top_y = 0.6

        # Build House Elements
        ground = Line(
            [-5.5, floor_y, 0], [3.5, floor_y, 0], color=GREY_C, stroke_width=3
        )
        back_wall = Line(
            [back_wall_x, floor_y, 0],
            [back_wall_x, roof_y, 0],
            color=WHITE,
            stroke_width=4,
        )
        roof = Line(
            [back_wall_x - 0.3, roof_y, 0],
            [overhang_x, roof_y, 0],
            color=WHITE,
            stroke_width=5,
        )
        front_lower = Line(
            [front_wall_x, floor_y, 0],
            [front_wall_x, sill_y, 0],
            color=WHITE,
            stroke_width=4,
        )

        # Window line
        window_glass = Line(
            [front_wall_x, sill_y, 0],
            [front_wall_x, win_top_y, 0],
            color=BLUE_A,
            stroke_width=3,
        )
        window_glass.set_opacity(0.8)

        # Solar Arc setup
        arc_center = np.array([0.2, -1.2, 0])
        arc_radius = 4.2

        summer_angle = np.radians(65)
        winter_angle = np.radians(25)

        solar_arc = Arc(
            radius=arc_radius,
            start_angle=np.radians(15),
            angle=np.radians(60),
            arc_center=arc_center,
            color=YELLOW_E,
            stroke_width=2,
        )
        dashed_arc = DashedVMobject(solar_arc, num_dashes=30)

        # Arc Label positioned clearly outside the arc
        arc_label = Text("Sonnenbahn", font_size=14, color=YELLOW_B)
        arc_label.move_to([3.6, 2.2, 0])

        # Render House and Solar Path
        self.play(
            Create(ground),
            Create(back_wall),
            Create(roof),
            Create(front_lower),
            Create(window_glass),
            Create(dashed_arc),
            Write(arc_label),
            run_time=2,
        )

        # Helper function for sun positioning
        def get_sun_pos(angle):
            return arc_center + arc_radius * np.array([np.cos(angle), np.sin(angle), 0])

        sun_pos_summer = get_sun_pos(summer_angle)
        sun_core = Dot(sun_pos_summer, radius=0.22, color=YELLOW)
        sun_glow = Dot(sun_pos_summer, radius=0.38, color=YELLOW_A).set_opacity(0.3)
        sun = VGroup(sun_glow, sun_core)

        self.play(FadeIn(sun, scale=0.5), Write(tag_summer), run_time=1)

        # Summer Rays & Beam Shading
        summer_beam = Polygon(
            [overhang_x, roof_y, 0],
            [1.31, floor_y, 0],
            [front_wall_x, floor_y, 0],
            [front_wall_x, sill_y, 0],
            color=YELLOW,
            fill_color=YELLOW,
            fill_opacity=0.35,
            stroke_width=1,
        )

        # Summer Rays & Beam Shading (rays stop at roof overhang and window edge without clipping through roof)
        ray_line1 = Line(
            sun_pos_summer, [overhang_x, roof_y, 0], color=YELLOW_A, stroke_width=1.5
        )
        ray_line2 = Line(
            sun_pos_summer, [overhang_x, win_top_y, 0], color=YELLOW_A, stroke_width=1.5
        )
        summer_rays = VGroup(ray_line1, ray_line2)

        # Descriptive Note placed safely on the left side of the house
        summer_note = Text(
            "Überhang blockiert\nstarke Sommerhitze",
            font_size=16,
            color=YELLOW_B,
            line_spacing=1.2,
        )
        summer_note.move_to([-4.2, 0.2, 0])

        self.play(
            Create(summer_rays), FadeIn(summer_beam), Write(summer_note), run_time=1.5
        )
        self.wait(1.5)

        # Transition to Winter
        self.play(
            FadeOut(summer_rays),
            FadeOut(summer_beam),
            FadeOut(summer_note),
            ReplacementTransform(tag_summer, tag_winter),
            run_time=1,
        )

        # Animate Sun moving down the arc to Winter position
        path_arc = Arc(
            radius=arc_radius,
            start_angle=summer_angle,
            angle=winter_angle - summer_angle,
            arc_center=arc_center,
        )

        self.play(MoveAlongPath(sun, path_arc), run_time=2.5, rate_func=smooth)

        # Winter Rays & Deep Penetration Beam (Synchronized with sun rays shooting from sun into room)
        sun_pos_winter = get_sun_pos(winter_angle)

        winter_beam = Polygon(
            [front_wall_x, win_top_y, 0],
            [back_wall_x, 0.63, 0],
            [back_wall_x, floor_y, 0],
            [-1.68, floor_y, 0],
            [front_wall_x, sill_y, 0],
            color=ORANGE,
            fill_color=ORANGE,
            fill_opacity=0.35,
            stroke_width=1,
        )

        w_ray1 = Line(
            sun_pos_winter,
            [front_wall_x, win_top_y, 0],
            color=YELLOW_A,
            stroke_width=1.5,
        )
        w_ray2 = Line(
            sun_pos_winter, [front_wall_x, sill_y, 0], color=YELLOW_A, stroke_width=1.5
        )
        winter_rays = VGroup(w_ray1, w_ray2)

        # Descriptive Note placed safely on the left side of the house
        winter_note = Paragraph(
            "Tiefe Sonneneinstrahlung",
            "erwärmt den Raum",
            font_size=16,
            color=ORANGE,
            line_spacing=1.2,
            alignment="center",
        )
        winter_note.move_to([-4.2, 0.2, 0])

        self.play(
            Create(winter_rays),
            GrowFromPoint(winter_beam, point=sun_pos_winter),
            Write(winter_note),
            run_time=1.8,
        )

        self.wait(2)


class Scene5ShadingFactor(Scene):
    def construct(self):
        # Set dark background
        self.camera.background_color = "#0f1115"

        # Color Palette
        COLOR_G = YELLOW
        COLOR_A = BLUE_B
        COLOR_G_VAL = GREEN
        COLOR_FSH = LIGHT_GREY
        COLOR_WINDOW = "#00FFFF"
        COLOR_SUMMER = RED_B
        COLOR_WINTER = ORANGE
        COLOR_RADIATION = YELLOW

        # --- Top Equation Bar (Continuing: G · A · F_f · g) ---
        eq_G = Text("G", font_size=28, color=COLOR_G)
        eq_d1 = Text(" · ", font_size=28, color=WHITE)
        eq_A = Text("A", font_size=28, color=COLOR_A)
        eq_d2 = Text(" · ", font_size=28, color=WHITE)
        eq_Ff = Text("F_f", font_size=28, color="#F97316", weight=BOLD)
        eq_d3 = Text(" · ", font_size=28, color=WHITE)
        eq_g = Text("g", font_size=28, color=COLOR_G_VAL)

        top_eq = VGroup(eq_G, eq_d1, eq_A, eq_d2, eq_Ff, eq_d3, eq_g)
        top_eq.arrange(RIGHT, buff=0.1)
        top_eq.to_edge(UP, buff=0.4)

        # --- Architectural Diagram Setup ---
        top_wall = Rectangle(
            width=0.3,
            height=0.8,
            fill_color="#22252a",
            fill_opacity=1,
            stroke_color=WHITE,
            stroke_width=2,
        ).move_to([0, 1.4, 0])

        bottom_wall = Rectangle(
            width=0.3,
            height=1.0,
            fill_color="#22252a",
            fill_opacity=1,
            stroke_color=WHITE,
            stroke_width=2,
        ).move_to([0, -1.7, 0])

        glass_window = Line(
            [0, 1.0, 0], [0, -1.2, 0], color=COLOR_WINDOW, stroke_width=5
        )

        floor = Line([0, -2.2, 0], [3.8, -2.2, 0], color=GREY, stroke_width=2)
        ceiling = Line([0, 1.8, 0], [3.8, 1.8, 0], color=GREY, stroke_width=2)
        interior_label = Text("Wohnraum", font_size=16, color=GREY).move_to(
            [2.2, 1.3, 0]
        )

        # Solid Grey Overhang Awning above window aperture
        awning = Polygon(
            [0.15, 1.05, 0],
            [-1.6, 1.05, 0],
            [-1.6, 0.92, 0],
            [0.15, 0.92, 0],
            color=COLOR_FSH,
            fill_color=GREY,
            fill_opacity=0.9,
            stroke_width=2,
        )

        # --- Animated Sun Mobject Setup ---
        sun_glow = Dot(radius=0.5, color=YELLOW, fill_opacity=0.2)
        sun_core = Dot(radius=0.25, color=YELLOW)
        sun_halo = Circle(radius=0.38, color=ORANGE, stroke_width=2, stroke_opacity=0.8)

        sun_rays_grp = VGroup()
        for angle in np.linspace(0, 2 * np.pi, 8, endpoint=False):
            p1 = np.array([0.3 * np.cos(angle), 0.3 * np.sin(angle), 0])
            p2 = np.array([0.45 * np.cos(angle), 0.45 * np.sin(angle), 0])
            sun_rays_grp.add(
                Line(p1, p2, color=YELLOW, stroke_width=2, stroke_opacity=0.9)
            )

        sun = VGroup(sun_glow, sun_core, sun_halo, sun_rays_grp)

        summer_pos = np.array([-3.5, 2.4, 0])
        winter_pos = np.array([-3.5, 0.2, 0])
        sun.move_to(summer_pos)

        sun_label = Text(
            "Sommersonne (Hoher Winkel)", font_size=16, color=COLOR_SUMMER
        ).move_to([-3.3, 3.2, 0])

        # --- Beat 1: Render Architecture, Top Equation & Sun ---
        self.play(Write(top_eq), run_time=0.8)
        self.play(
            Create(top_wall),
            Create(bottom_wall),
            Create(glass_window),
            Create(floor),
            Create(ceiling),
            FadeIn(interior_label),
            run_time=0.8,
        )
        self.play(Create(awning), FadeIn(sun), FadeIn(sun_label), run_time=0.8)
        self.wait(0.4)

        # --- Beat 2: Summer Solar Radiation Beam (Blocked by Awning) ---
        summer_beam = Polygon(
            [-3.5, 2.4, 0],
            [0.15, 0.92, 0],
            [-1.6, 0.92, 0],
            fill_color=COLOR_RADIATION,
            fill_opacity=0.2,
            stroke_color=COLOR_RADIATION,
            stroke_opacity=0.4,
            stroke_width=1,
        )

        blocked_text = Text(
            "Durch Überhang blockiert", font_size=14, color=COLOR_SUMMER
        ).move_to([-1.4, 0.4, 0])

        self.play(sun_rays_grp.animate.rotate(0.4), FadeIn(summer_beam), run_time=1.0)
        self.play(FadeIn(blocked_text), run_time=0.6)
        self.wait(1.0)

        # --- Beat 3: Transition to Winter Sun & Low Angle Radiation ---
        winter_sun_label = Text(
            "Wintersonne (Niedriger Winkel)", font_size=16, color=COLOR_WINTER
        ).move_to([-3.3, 0.9, 0])

        self.play(
            FadeOut(summer_beam),
            FadeOut(blocked_text),
            Transform(sun_label, winter_sun_label),
            sun.animate.move_to(winter_pos),
            run_time=1.2,
        )

        winter_beam = Polygon(
            [-3.5, 0.2, 0],
            [0, 0.9, 0],
            [3.2, -0.6, 0],
            [3.2, -2.2, 0],
            [0, -1.2, 0],
            fill_color=COLOR_RADIATION,
            fill_opacity=0.2,
            stroke_color=COLOR_RADIATION,
            stroke_opacity=0.4,
            stroke_width=1,
        )

        enters_text = Text(
            "Dringt in Wohnraum ein", font_size=14, color=COLOR_WINTER
        ).move_to([2.0, -1.2, 0])

        self.play(sun_rays_grp.animate.rotate(-0.4), FadeIn(winter_beam), run_time=1.0)
        self.play(FadeIn(enters_text), run_time=0.6)
        self.wait(1.0)

        # Fade radiation overlay before equation update
        self.play(
            FadeOut(winter_beam), FadeOut(enters_text), FadeOut(sun_label), run_time=0.8
        )

        # --- Beat 4: Display 'F_sh' Label and Float to Equation Bar ---
        # Positioned above top_wall to avoid collision
        fsh_label = Text("F_sh", font_size=24, color=COLOR_FSH).next_to(
            top_wall, UP, buff=0.15
        )
        fsh_desc = Text(
            "Verschattungsfaktor (F_sh nach DIN 4108-2)", font_size=14, color=COLOR_FSH
        ).next_to(fsh_label, UP, buff=0.1)

        self.play(FadeIn(fsh_label), FadeIn(fsh_desc), run_time=0.6)
        self.wait(0.5)

        # Assemble updated full equation at top: G · A · F_f · g · F_sh
        dot4_text = Text(" · ", font_size=28, color=WHITE)
        full_eq = (
            VGroup(
                Text("G", font_size=28, color=COLOR_G),
                Text(" · ", font_size=28, color=WHITE),
                Text("A", font_size=28, color=COLOR_A),
                Text(" · ", font_size=28, color=WHITE),
                Text("F_f", font_size=28, color="#F97316", weight=BOLD),
                Text(" · ", font_size=28, color=WHITE),
                Text("g", font_size=28, color=COLOR_G_VAL),
                dot4_text,
                Text("F_sh", font_size=28, color=COLOR_FSH),
            )
            .arrange(RIGHT, buff=0.1)
            .to_edge(UP, buff=0.4)
        )

        self.play(
            Transform(top_eq, full_eq[:7]),
            FadeIn(dot4_text.move_to(full_eq[7].get_center())),
            Transform(fsh_label, full_eq[8]),
            FadeOut(fsh_desc),
            run_time=1.2,
        )

        self.wait(2.0)


class ThermalMassScene(Scene):
    def construct(self):
        # Set dark architectural background
        self.camera.background_color = "#0f1115"

        # -----------------------------------------------------------------
        # Titles & Headers (Top section reserved for text)
        # -----------------------------------------------------------------
        title = Text("Wärmespeicherung und Strahlung", font_size=28, color=WHITE)
        title.to_edge(UP, buff=0.35)

        subtitle = Text(
            "Dichte Materialien absorbieren tagsüber Solarwärme und strahlen sie nachts ab",
            font_size=16,
            color=GREY_A,
        )
        subtitle.next_to(title, DOWN, buff=0.12)

        phase_1_text = Text(
            "Phase 1: Solarwärmeabsorption", font_size=18, color="#FACC15"
        )
        phase_1_text.move_to([0, 2.1, 0])

        phase_2_text = Text(
            "Phase 2: Nächtliche Strahlungsheizung", font_size=18, color="#F59E0B"
        )
        phase_2_text.move_to([0, 2.1, 0])

        self.play(Write(title), FadeIn(subtitle), FadeIn(phase_1_text), run_time=1.5)

        # -----------------------------------------------------------------
        # Architectural Room Setup (Lower center shift)
        # -----------------------------------------------------------------
        # Concrete floor slab (starts in dense grey #737373)
        floor_slab = Rectangle(
            width=6.4,
            height=0.6,
            color="#737373",
            fill_color="#737373",
            fill_opacity=0.85,
            stroke_width=2,
        )
        floor_slab.move_to([0, -1.5, 0])

        floor_label = Text(
            "Betonbodenplatte (Thermische Masse)", font_size=15, color=WHITE
        )
        floor_label.move_to(floor_slab.get_center())

        # Interior structural frame lines
        left_wall = Line(
            [-3.2, 1.0, 0], [-3.2, -1.2, 0], color="#E2E8F0", stroke_width=3
        )
        ceiling = Line([-3.2, 1.0, 0], [3.2, 1.0, 0], color="#E2E8F0", stroke_width=3)
        right_wall_top = Line(
            [3.2, 1.0, 0], [3.2, 0.4, 0], color="#E2E8F0", stroke_width=3
        )
        right_wall_bot = Line(
            [3.2, -0.4, 0], [3.2, -1.2, 0], color="#E2E8F0", stroke_width=3
        )

        # Glazing / Window frame shifted lower (y=0.0)
        window = Rectangle(
            width=0.1,
            height=0.8,
            color="#38BDF8",
            fill_color="#38BDF8",
            fill_opacity=0.5,
            stroke_width=1.5,
        ).move_to([3.2, 0.0, 0])

        room_group = VGroup(left_wall, ceiling, right_wall_top, right_wall_bot, window)

        # Sun element (Daytime, positioned lower to align radiation rays through the window aperture at y=0.0)
        sun_center = np.array([4.6, 0.2, 0])
        sun = Dot(point=sun_center, radius=0.35, color="#FACC15")
        sun_glow = Circle(
            radius=0.5, color="#FACC15", stroke_width=1, stroke_opacity=0.4
        ).move_to(sun_center)
        sun_label = Text("Wintersonne", font_size=13, color="#FACC15").next_to(
            sun, UP, buff=0.15
        )
        sun_group = VGroup(sun, sun_glow, sun_label)

        sun_rays = VGroup(
            Line(
                sun_center,
                [-1.8, -1.2, 0],
                color="#FACC15",
                stroke_width=2.5,
                stroke_opacity=0.8,
            ),
            Line(
                sun_center,
                [-0.4, -1.2, 0],
                color="#FACC15",
                stroke_width=2.5,
                stroke_opacity=0.8,
            ),
            Line(
                sun_center,
                [1.0, -1.2, 0],
                color="#FACC15",
                stroke_width=2.5,
                stroke_opacity=0.8,
            ),
        )

        self.play(
            Create(room_group),
            Create(floor_slab),
            Write(floor_label),
            FadeIn(sun_group),
            run_time=2.0,
        )

        # -----------------------------------------------------------------
        # Daytime Heating & Color Transition
        # -----------------------------------------------------------------
        self.play(Create(sun_rays), run_time=1.5)

        # Floor absorbs heat: transitions from dense grey (#737373) to warm orange (#F59E0B)
        self.play(
            floor_slab.animate.set_color("#F59E0B").set_fill("#F59E0B", opacity=0.9),
            run_time=2.5,
        )
        self.wait(1.0)

        # -----------------------------------------------------------------
        # Transition to Night Sky
        # -----------------------------------------------------------------
        moon_center = np.array([4.6, 1.2, 0])
        moon = Text("☾", font_size=32, color="#94A3B8").move_to(moon_center)
        moon_label = Text("Nachthimmel", font_size=13, color="#94A3B8").next_to(
            moon, UP, buff=0.15
        )
        night_group = VGroup(moon, moon_label)

        self.play(
            FadeOut(sun_rays),
            FadeOut(sun_group),
            ReplacementTransform(phase_1_text, phase_2_text),
            FadeIn(night_group),
            run_time=1.8,
        )

        # -----------------------------------------------------------------
        # Nighttime Thermal Radiation Lines
        # -----------------------------------------------------------------
        def make_wavy_line(x_pos):
            points = []
            for step in range(21):
                y = -1.2 + step * 0.1
                x = x_pos + 0.08 * np.sin(step * 0.6)
                points.append([x, y, 0])
            curve = VMobject()
            curve.set_points_smoothly([np.array(p) for p in points])
            curve.set_color("#F59E0B")
            curve.set_stroke(width=2, opacity=0.8)
            return curve

        wavy_x_positions = [-2.4, -1.5, -0.6, 0.3, 1.2, 2.1]
        wavy_lines = VGroup(*[make_wavy_line(x) for x in wavy_x_positions])

        rad_label = Text(
            "Gespeicherte Wärme strahlt nach oben", font_size=15, color="#F59E0B"
        )
        rad_label.move_to([0, 0.2, 0])

        self.play(
            LaggedStart(*[Create(wl) for wl in wavy_lines], lag_ratio=0.15),
            FadeIn(rad_label),
            run_time=2.5,
        )

        # Subtle upward pulse/shift of thermal radiation lines
        self.play(
            wavy_lines.animate.shift(UP * 0.15),
            floor_slab.animate.set_fill("#D97706", opacity=0.75),
            run_time=1.7,
        )

        # -----------------------------------------------------------------
        # Final Hold
        # -----------------------------------------------------------------
        self.wait(2.0)


class Scene6ThermalMassFormula(Scene):
    def construct(self):
        self.camera.background_color = "#0f1115"

        # Title
        title = Text(
            "Berechnung der thermischen Speichermasse", font_size=26, color=WHITE
        )
        title.to_edge(UP, buff=0.35)

        subtitle = Text(
            "Warum Speichermasse im solaren Wärmegewinn entscheidend ist",
            font_size=15,
            color=GREY_A,
        ).next_to(title, DOWN, buff=0.12)

        self.play(Write(title), FadeIn(subtitle), run_time=1.2)

        # Main Heat Storage Formula: Q_speicher = m · c · ΔT
        q_main = Text("Q", color="#F59E0B", font_size=32, weight=BOLD)
        q_sub = (
            Text("speicher", color="#F59E0B", font_size=16)
            .next_to(q_main, RIGHT, buff=0.03)
            .align_to(q_main, DOWN)
            .shift(DOWN * 0.04)
        )
        q_label = VGroup(q_main, q_sub)

        eq_sign = Text("=", color=WHITE, font_size=32)
        m_txt = Text("m", color="#38BDF8", font_size=32, weight=BOLD)
        dot1 = Text("·", color=WHITE, font_size=32)
        c_txt = Text("c", color="#2ECC71", font_size=32, weight=BOLD)
        dot2 = Text("·", color=WHITE, font_size=32)
        dt_txt = Text("ΔT", color="#FACC15", font_size=32, weight=BOLD)

        formula_group = (
            VGroup(q_label, eq_sign, m_txt, dot1, c_txt, dot2, dt_txt)
            .arrange(RIGHT, buff=0.15)
            .move_to(UP * 0.9)
        )

        rect_box = SurroundingRectangle(
            formula_group, color="#F59E0B", buff=0.2, stroke_width=2
        )

        self.play(
            Write(q_label),
            Write(eq_sign),
            Write(m_txt),
            Write(dot1),
            Write(c_txt),
            Write(dot2),
            Write(dt_txt),
            Create(rect_box),
            run_time=1.6,
        )
        self.wait(0.6)

        # Variable explanations
        desc_m = Text(
            "m = Masse des Bauteils [kg] (z. B. Beton)", font_size=14, color="#38BDF8"
        )
        desc_c = Text(
            "c = Spezifische Wärmekapazität [J/(kg·K)]", font_size=14, color="#2ECC71"
        )
        desc_dt = Text(
            "ΔT = Temperaturdifferenz [K] (Tag zu Nacht)", font_size=14, color="#FACC15"
        )

        desc_group = (
            VGroup(desc_m, desc_c, desc_dt)
            .arrange(DOWN, buff=0.2, aligned_edge=LEFT)
            .move_to(DOWN * 0.3)
        )

        self.play(
            LaggedStart(
                *[FadeIn(d, shift=RIGHT * 0.2) for d in desc_group], lag_ratio=0.2
            ),
            run_time=1.5,
        )
        self.wait(0.8)

        # Why we consider it in Solar Heat Gain
        why_title = Text(
            "Bedeutung für die Heizlast:", font_size=15, color=WHITE, weight=BOLD
        )
        why_body = Paragraph(
            "• Dämpft Überhitzung im Sommer (Dämpfungsfaktor)",
            "• Speichert solare Gewinne für kühle Abendstunden",
            "• Reduziert die aktive Heizlast Q_h deutlich",
            font_size=13,
            color=GREY_A,
            line_spacing=1.2,
        )
        why_box = (
            VGroup(why_title, why_body)
            .arrange(DOWN, buff=0.15, aligned_edge=LEFT)
            .move_to(DOWN * 1.8)
        )

        self.play(FadeIn(why_box, shift=UP * 0.2), run_time=1.2)
        self.wait(2.5)


class Scene7MasterEquation(Scene):
    def construct(self):
        # Set dark architectural background
        self.camera.background_color = "#0f1115"

        # -------------------------------------------------------------
        # 1. Title and Header Setup
        # -------------------------------------------------------------
        title = Text(
            "Die Hauptgleichung für solaren Wärmegewinn", color=WHITE, font_size=28
        )
        title.to_edge(UP, buff=0.4)

        # Variables in the top bar with subscript formatting
        top_g = Text("G", color="#F1C40F", font_size=26)
        top_a = Text("A", color="#3498DB", font_size=26)
        top_ff = Text("F_f", color="#F97316", font_size=26, weight=BOLD)
        top_gv = Text("g", color="#2ECC71", font_size=26)

        top_f_main = Text("F", color="#95A5A6", font_size=26)
        top_f_sub = Text("sh", color="#95A5A6", font_size=15)
        top_f_sub.next_to(top_f_main, RIGHT, buff=0.02).align_to(
            top_f_main, DOWN
        ).shift(DOWN * 0.03)
        top_fsh = VGroup(top_f_main, top_f_sub)

        top_vars = VGroup(top_g, top_a, top_ff, top_gv, top_fsh).arrange(
            RIGHT, buff=0.9
        )
        top_vars.next_to(title, DOWN, buff=0.25)

        # Initial Scene Fade In
        self.play(
            FadeIn(title),
            FadeIn(top_vars),
            run_time=1.5,
        )
        self.wait(0.5)

        # -------------------------------------------------------------
        # 3. Assemble the Master Equation in Center Stage
        # -------------------------------------------------------------
        phi_main = Text("Φ", color=WHITE, font_size=36)
        phi_sub = Text("solar", color=WHITE, font_size=20)
        phi_sub.next_to(phi_main, RIGHT, buff=0.04).align_to(phi_main, DOWN).shift(
            DOWN * 0.05
        )
        phi_txt = VGroup(phi_main, phi_sub)

        eq_txt = Text("=", color=WHITE, font_size=36)
        g_txt = Text("G", color="#F1C40F", font_size=36)
        dot1 = Text("·", color=WHITE, font_size=36)
        a_txt = Text("A", color="#3498DB", font_size=36)
        dot2 = Text("·", color=WHITE, font_size=36)
        ff_txt = Text("F_f", color="#F97316", font_size=36, weight=BOLD)
        dot3 = Text("·", color=WHITE, font_size=36)
        gv_txt = Text("g", color="#2ECC71", font_size=36)
        dot4 = Text("·", color=WHITE, font_size=36)

        f_main = Text("F", color="#95A5A6", font_size=36)
        f_sub = Text("sh", color="#95A5A6", font_size=20)
        f_sub.next_to(f_main, RIGHT, buff=0.03).align_to(f_main, DOWN).shift(
            DOWN * 0.04
        )
        fsh_txt = VGroup(f_main, f_sub)

        eq_group = (
            VGroup(
                phi_txt,
                eq_txt,
                g_txt,
                dot1,
                a_txt,
                dot2,
                ff_txt,
                dot3,
                gv_txt,
                dot4,
                fsh_txt,
            )
            .arrange(RIGHT, buff=0.12)
            .move_to(ORIGIN)
        )

        # Transition top variables down into position
        self.play(
            Write(phi_txt),
            Write(eq_txt),
            ReplacementTransform(top_g, g_txt),
            Write(dot1),
            ReplacementTransform(top_a, a_txt),
            Write(dot2),
            ReplacementTransform(top_ff, ff_txt),
            Write(dot3),
            ReplacementTransform(top_gv, gv_txt),
            Write(dot4),
            ReplacementTransform(top_fsh, fsh_txt),
            run_time=2.0,
        )

        # Frame around the assembled master equation
        rect = SurroundingRectangle(eq_group, color=WHITE, buff=0.3, stroke_width=2)

        self.play(Create(rect), run_time=1.5)
        self.wait(1.5)

        # Highlight Shading Factor
        fsh_box = SurroundingRectangle(
            fsh_txt, color="#95A5A6", buff=0.1, stroke_width=2
        )

        self.play(Create(fsh_box), run_time=1.5)
        self.wait(1.5)

        # -------------------------------------------------------------
        # 4. Clean Fade Out
        # -------------------------------------------------------------
        self.play(FadeOut(*self.mobjects), run_time=1.5)

        self.wait(2)


class FullSolarHeatGainVideo(Scene):
    def construct(self):
        scenes = [
            Scene1Transition,
            Scene2IrradianceAndArea,
            Scene3GValue,
            SeasonalSunAngles,
            Scene5ShadingFactor,
            ThermalMassScene,
            Scene6ThermalMassFormula,
            Scene7MasterEquation,
        ]
        base_dir = os.path.dirname(os.path.abspath(__file__))
        audio_files = [
            os.path.join(base_dir, f"scene_{i}_audio.mp3") for i in range(1, 8)
        ]

        for scene_cls, audio_path in zip(scenes, audio_files):
            if os.path.exists(audio_path):
                self.add_sound(audio_path)
            scene_cls.construct(self)
            self.clear()
