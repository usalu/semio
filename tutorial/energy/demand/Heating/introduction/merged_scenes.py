import os
import numpy as np
import math
from manim import *


class Scene1(Scene):
    def construct(self):
        # 1. Background and Grid Setup
        self.camera.background_color = "#0B0C10"

        grid = NumberPlane(
            x_range=[-8, 8, 1],
            y_range=[-4.5, 4.5, 1],
            background_line_style={
                "stroke_color": "#1F2833",
                "stroke_width": 1,
                "stroke_opacity": 0.4,
            },
            axis_config={"stroke_opacity": 0},
        )
        baseline = Line(
            start=[-7, -2, 0], end=[7, -2, 0], color="#C5C6C7", stroke_width=2
        )

        title = Text("Thermische Hülle und Wärmeverlust", font="Arial", font_size=32, color=WHITE).to_edge(UP, buff=0.4)

        self.play(
            Create(grid, run_time=1.2),
            Create(baseline, run_time=1.2),
            Write(title),
        )

        # 2. House Structure & Interior Warm Fill
        house_pts = [
            [-1.8, -2.0, 0],
            [1.8, -2.0, 0],
            [1.8, 0.0, 0],
            [0.0, 1.5, 0],
            [-1.8, 0.0, 0],
        ]
        house_outline = Polygon(*house_pts, color=WHITE, stroke_width=4)
        house_interior = Polygon(
            *house_pts, color="#FF6B35", fill_opacity=0.35, stroke_width=0
        )

        self.play(
            Create(house_outline),
            FadeIn(house_interior),
            run_time=1.5,
        )

        # 3. Temperature Labels
        t_in = Text("T_innen = 20 °C", font="Arial", font_size=24, color="#FF6B35").move_to([0, -0.8, 0])
        t_out = Text("T_außen = -5 °C", font="Arial", font_size=24, color="#00B4D8").move_to([3.2, 0.8, 0])

        self.play(
            Write(t_in),
            Write(t_out),
            run_time=1.2,
        )
        self.wait(0.5)

        # 4. Leakage Title
        leak_label = Text("Warme Luft entweicht", font="Arial", font_size=22, color="#FF9F1C").next_to(title, DOWN, buff=0.3)
        self.play(Write(leak_label))

        # 5. Warm Air Leakage Animation (Particles escaping house)
        start_points = [
            [0.0, -0.5, 0],
            [-0.4, -0.8, 0],
            [0.4, -0.8, 0],
            [-0.2, -0.3, 0],
            [0.2, -0.3, 0],
            [0.0, -1.0, 0],
            [-0.6, -0.4, 0],
            [0.6, -0.4, 0],
            [0.0, 0.2, 0],
            [-0.3, 0.0, 0],
        ]

        exit_targets = [
            [-3.0, 0.2, 0],    # Left wall leak
            [-2.5, 1.8, 0],    # Left roof leak
            [0.0, 2.8, 0],     # Roof apex leak
            [2.5, 1.8, 0],     # Right roof leak
            [3.0, 0.2, 0],     # Right wall leak
            [-2.8, -1.2, 0],   # Lower left leak
            [2.8, -1.2, 0],    # Lower right leak
            [-1.2, 2.5, 0],    # High left leak
            [1.2, 2.5, 0],     # High right leak
            [0.0, 2.2, 0],     # Center top leak
        ]

        dots = VGroup(*[
            Dot(point=pt, radius=0.09, color="#FF6B35")
            for pt in start_points
        ])

        self.play(FadeIn(dots), run_time=0.8)

        # First wave of escaping warm air
        wave1_anims = [
            dot.animate.move_to(target).set_color("#00B4D8").set_opacity(0.0)
            for dot, target in zip(dots, exit_targets)
        ]

        self.play(*wave1_anims, run_time=2.5, rate_func=linear)

        # Second wave for continuous leakage visual effect
        dots_wave2 = VGroup(*[
            Dot(point=pt, radius=0.09, color="#FF6B35")
            for pt in start_points
        ])

        self.play(FadeIn(dots_wave2), run_time=0.5)

        wave2_anims = [
            dot.animate.move_to(target).set_color("#00B4D8").set_opacity(0.0)
            for dot, target in zip(dots_wave2, exit_targets)
        ]

        self.play(*wave2_anims, run_time=2.5, rate_func=linear)

        # 6. Transition Zoom - Scale house lines up big and zoom into pure orange screen
        fade_group = VGroup(grid, baseline, t_in, t_out, title, leak_label, dots, dots_wave2)

        zoom_center = np.array([0.0, -0.5, 0.0])

        self.play(
            FadeOut(fade_group),
            house_outline.animate.scale(25, about_point=zoom_center).set_stroke(width=30),
            house_interior.animate.scale(25, about_point=zoom_center).set_fill(opacity=1.0),
            run_time=2.0,
        )

        self.wait(2)



class Scene2(Scene):
    def construct(self):
        # Set dark background color
        self.camera.background_color = "#0B0C10"

        # ----------------------------------------------------
        # Phase 1: Water + Thermometer (20°C -> 21°C) + Heat
        # ----------------------------------------------------

        # Create Water Square (1 Liter) with lower initial blue fill opacity
        water_square = Square(
            side_length=2.2,
            fill_color="#00E5FF",
            fill_opacity=0.25,
            stroke_color="#00E5FF",
            stroke_width=3
        )
        water_label = Text("1 Liter\nWasser", font="Arial", font_size=20, color=WHITE, line_spacing=1.1).move_to(water_square.get_center())
        water_group = VGroup(water_square, water_label).move_to(LEFT * 2.5 + UP * 0.6)

        # Single Thermometer setup
        thermo_frame = RoundedRectangle(
            corner_radius=0.15,
            height=3.2,
            width=0.5,
            stroke_color=WHITE,
            stroke_width=2,
            fill_color="#1E222A",
            fill_opacity=0.8
        )
        bulb = Circle(
            radius=0.4,
            stroke_color=WHITE,
            stroke_width=2,
            fill_color="#1E222A",
            fill_opacity=0.8
        ).next_to(thermo_frame, DOWN, buff=-0.2)

        fluid_bulb = Circle(
            radius=0.32,
            stroke_color=RED,
            stroke_width=0,
            fill_color=RED,
            fill_opacity=0.9
        ).move_to(bulb)

        fluid_stem = Rectangle(
            width=0.22,
            height=1.2,
            stroke_width=0,
            fill_color=RED,
            fill_opacity=0.9
        ).next_to(bulb.get_center(), UP, buff=0)

        temp_display = Text("20.0 °C", font="Arial", font_size=22, color=WHITE, weight=BOLD).next_to(thermo_frame, UP, buff=0.3)
        single_thermo = VGroup(thermo_frame, bulb, fluid_bulb, fluid_stem, temp_display).move_to(RIGHT * 2 + UP * 0.4)

        self.play(
            FadeIn(water_group, shift=RIGHT),
            FadeIn(single_thermo, shift=LEFT),
            run_time=1.5
        )
        self.wait(0.8)

        # Wavy orange bezier heat curves beneath water square
        heat_waves = VGroup()
        for i in range(4):
            x_pos = water_square.get_left()[0] + 0.4 + i * 0.48
            y_bottom = water_square.get_bottom()[1] - 0.7
            path = CubicBezier(
                [x_pos, y_bottom, 0],
                [x_pos + 0.12, y_bottom + 0.2, 0],
                [x_pos - 0.12, y_bottom + 0.4, 0],
                [x_pos, y_bottom + 0.6, 0]
            )
            path.set_color(ORANGE).set_stroke(width=3)
            heat_waves.add(path)

        self.play(
            Create(heat_waves),
            run_time=1.5
        )
        self.wait(0.5)

        # Rise temperature from 20 to 21 degrees C & reduce water square fill opacity
        new_fluid_stem = Rectangle(
            width=0.22,
            height=1.5,
            stroke_width=0,
            fill_color=RED,
            fill_opacity=0.9
        ).next_to(bulb.get_center(), UP, buff=0)
        
        new_temp_display = Text("21.0 °C", font="Arial", font_size=22, color=WHITE, weight=BOLD).next_to(thermo_frame, UP, buff=0.3)

        self.play(
            heat_waves.animate.set_opacity(0.3),
            water_square.animate.set_fill(color=ORANGE, opacity=0.1),
            Transform(fluid_stem, new_fluid_stem),
            Transform(temp_display, new_temp_display),
            run_time=2.0
        )
        # Extended pause to align with voiceover explaining 1-degree warmth increment
        self.wait(1.8)

        # ----------------------------------------------------
        # Phase 2: Split Screen into Dual Scales
        # ----------------------------------------------------
        self.play(
            FadeOut(water_group),
            FadeOut(heat_waves),
            FadeOut(single_thermo),
            run_time=1.2
        )
        self.wait(0.4)

        c_x = -2.8
        k_x = 2.8

        c_line = Line(DOWN * 1.8, UP * 2.2, stroke_width=4, color="#00E5FF").move_to(RIGHT * c_x + UP * 0.2)
        k_line = Line(DOWN * 1.8, UP * 2.2, stroke_width=4, color=ORANGE).move_to(RIGHT * k_x + UP * 0.2)

        c_header = Text("Celsius (°C)", font="Arial", font_size=20, color="#00E5FF", weight=BOLD).next_to(c_line, UP, buff=0.3)
        k_header = Text("Kelvin (K)", font="Arial", font_size=20, color=ORANGE, weight=BOLD).next_to(k_line, UP, buff=0.3)

        # Vertical positions
        y_abs = -1.4
        y_frz = 0.3
        y_boil = 1.9

        def make_tick(scale_x, y_val, text_str, align_left=True):
            pt = np.array([scale_x, y_val, 0])
            tick_line = Line(pt + LEFT * 0.1, pt + RIGHT * 0.1, stroke_width=2, color=WHITE)
            txt = Text(text_str, font_size=15, color=WHITE)
            if align_left:
                txt.next_to(tick_line, LEFT, buff=0.15)
            else:
                txt.next_to(tick_line, RIGHT, buff=0.15)
            return VGroup(tick_line, txt)

        c_t1 = make_tick(c_x, y_abs, "-273.15 °C", True)
        c_t2 = make_tick(c_x, y_frz, "0 °C", True)
        c_t3 = make_tick(c_x, y_boil, "100 °C", True)

        k_t1 = make_tick(k_x, y_abs, "0 K", False)
        k_t2 = make_tick(k_x, y_frz, "273.15 K", False)
        k_t3 = make_tick(k_x, y_boil, "373.15 K", False)

        c_ticks = VGroup(c_t1, c_t2, c_t3)
        k_ticks = VGroup(k_t1, k_t2, k_t3)

        dual_scales = VGroup(c_line, k_line, c_header, k_header, c_ticks, k_ticks)

        self.play(
            FadeIn(dual_scales),
            run_time=1.8
        )
        self.wait(1.4)

        # Dashed horizontal alignment lines
        dash1 = DashedLine(np.array([c_x, y_abs, 0]), np.array([k_x, y_abs, 0]), color=GREY, stroke_width=1.5)
        dash2 = DashedLine(np.array([c_x, y_frz, 0]), np.array([k_x, y_frz, 0]), color=GREY, stroke_width=1.5)
        dash3 = DashedLine(np.array([c_x, y_boil, 0]), np.array([k_x, y_boil, 0]), color=GREY, stroke_width=1.5)

        dashed_connectors = VGroup(dash1, dash2, dash3)

        self.play(
            Create(dashed_connectors),
            run_time=1.8
        )
        self.wait(1.5)

        # ----------------------------------------------------
        # Phase 3: Step Size Comparison & Pulsing Brackets
        # ----------------------------------------------------
        y_step_top = y_frz + 0.75

        # Brackets highlighting identical intervals
        c_b_start = np.array([c_x + 0.15, y_frz, 0])
        c_b_end = np.array([c_x + 0.15, y_step_top, 0])
        c_bracket = Brace(Line(c_b_start, c_b_end), RIGHT, color=WHITE)
        c_bracket_label = Text("Δ 1°C", font="Arial", font_size=15, color=WHITE).next_to(c_bracket, RIGHT, buff=0.1)

        k_b_start = np.array([k_x - 0.15, y_frz, 0])
        k_b_end = np.array([k_x - 0.15, y_step_top, 0])
        k_bracket = Brace(Line(k_b_start, k_b_end), LEFT, color=WHITE)
        k_bracket_label = Text("Δ 1 K", font="Arial", font_size=15, color=WHITE).next_to(k_bracket, LEFT, buff=0.1)

        step_dash = DashedLine(np.array([c_x, y_step_top, 0]), np.array([k_x, y_step_top, 0]), color=WHITE, stroke_width=1.5)

        self.play(
            Create(step_dash),
            GrowFromCenter(c_bracket),
            Write(c_bracket_label),
            GrowFromCenter(k_bracket),
            Write(k_bracket_label),
            run_time=1.5
        )
        self.wait(1.0)

        # Pulsing effect on brackets
        bracket_highlight = VGroup(c_bracket, c_bracket_label, k_bracket, k_bracket_label, step_dash)
        self.play(
            bracket_highlight.animate.scale(1.12),
            run_time=0.6
        )
        self.play(
            bracket_highlight.animate.scale(1 / 1.12),
            run_time=0.6
        )
        self.wait(0.8)

        # Center Callout Box
        eq_box = RoundedRectangle(
            corner_radius=0.15,
            height=0.7,
            width=3.2,
            fill_color="#000000",
            fill_opacity=0.95,
            stroke_color=WHITE,
            stroke_width=2
        ).move_to(UP * 1.1)
        eq_text = Text("1 °C Schritt = 1 K Schritt", font="Arial", font_size=16, color=WHITE, weight=BOLD).move_to(eq_box)
        eq_group = VGroup(eq_box, eq_text)

        self.play(
            FadeIn(eq_group, shift=DOWN * 0.15),
            run_time=1.2
        )

        # Final hold for narration conclusion
        self.wait(2.5)



class Scene3(Scene):
    def construct(self):
        # Set dark background
        self.camera.background_color = "#0B0C10"

        # Define custom cyan color
        CYAN_COLOR = "#00E5FF"

        # Section Title
        title = Text("Das Rennen um die spezifische Wärmekapazität", font="Arial", font_size=28, color=WHITE)
        title.to_edge(UP, buff=0.3)
        self.add(title)

        # Positions for 3 columns
        x_water, x_concrete, x_air = -4.0, 0, 4.0
        y_mat = -1.6
        y_therm = 0.7

        # --- Materials ---
        # Water (Cyan medium square)
        water_box = Square(side_length=1.5, color=CYAN_COLOR, fill_opacity=0.4, stroke_width=2)
        water_box.move_to([x_water, y_mat, 0])
        water_label = Text("Wasser (1 kg)", font="Arial", font_size=16, color=CYAN_COLOR)
        water_label.next_to(water_box, DOWN, buff=0.2)

        # Concrete (Gray dense block)
        concrete_box = Square(side_length=1.1, color=GREY_B, fill_opacity=0.75, stroke_color=LIGHT_GREY, stroke_width=2)
        concrete_box.move_to([x_concrete, y_mat, 0])
        concrete_label = Text("Beton (1 kg)", font="Arial", font_size=16, color=LIGHT_GREY)
        concrete_label.next_to(concrete_box, DOWN, buff=0.2)

        # Air (Large diffuse faint-blue box)
        air_box = Rectangle(width=2.2, height=1.9, color=BLUE_B, fill_opacity=0.15, stroke_width=2)
        air_box.move_to([x_air, y_mat, 0])
        air_label = Text("Luft (1 kg)", font="Arial", font_size=16, color=BLUE_B)
        air_label.next_to(air_box, DOWN, buff=0.2)

        materials_group = VGroup(water_box, water_label, concrete_box, concrete_label, air_box, air_label)

        # --- Thermometers ---
        def create_thermometer(x_pos):
            bulb_outer = Circle(radius=0.24, color=WHITE, stroke_width=2)
            bulb_outer.move_to([x_pos, y_therm - 0.45, 0])
            tube_outer = RoundedRectangle(corner_radius=0.06, width=0.18, height=1.4, color=WHITE, stroke_width=2)
            tube_outer.move_to([x_pos, y_therm + 0.3, 0])
            
            bulb_inner = Circle(radius=0.18, color=RED, fill_opacity=1, stroke_width=0)
            bulb_inner.move_to(bulb_outer.get_center())
            
            # Initial low fluid height
            baseline_y = y_therm - 0.35
            fluid_stem = Rectangle(width=0.08, height=0.08, color=RED, fill_opacity=1, stroke_width=0)
            fluid_stem.move_to([x_pos, baseline_y + 0.04, 0])

            frame = VGroup(bulb_outer, tube_outer)
            return frame, bulb_inner, fluid_stem, baseline_y

        therm_w_frame, therm_w_bulb, fluid_w, base_w = create_thermometer(x_water)
        therm_c_frame, therm_c_bulb, fluid_c, base_c = create_thermometer(x_concrete)
        therm_a_frame, therm_a_bulb, fluid_a, base_a = create_thermometer(x_air)

        thermometers = VGroup(
            therm_w_frame, therm_w_bulb, fluid_w,
            therm_c_frame, therm_c_bulb, fluid_c,
            therm_a_frame, therm_a_bulb, fluid_a
        )

        # Fade in elements
        self.play(
            FadeIn(materials_group),
            FadeIn(thermometers),
            run_time=1.8
        )
        self.wait(0.8)

        # --- Energy Orbs ---
        def create_orb(x_pos):
            core = Dot(point=[x_pos, 2.9, 0], radius=0.14, color=ORANGE)
            glow = Dot(point=[x_pos, 2.9, 0], radius=0.28, color=YELLOW, fill_opacity=0.35)
            lbl = Text("1 kJ", font="Arial", font_size=12, color=WHITE)
            lbl.move_to(core.get_center())
            return VGroup(glow, core, lbl)

        orb_w = create_orb(x_water)
        orb_c = create_orb(x_concrete)
        orb_a = create_orb(x_air)

        self.play(
            FadeIn(orb_w, shift=DOWN * 0.2),
            FadeIn(orb_c, shift=DOWN * 0.2),
            FadeIn(orb_a, shift=DOWN * 0.2),
            run_time=1.2
        )
        self.wait(0.4)

        # Drop Orbs into Materials simultaneously
        self.play(
            orb_w.animate.move_to([x_water, y_mat, 0]),
            orb_c.animate.move_to([x_concrete, y_mat, 0]),
            orb_a.animate.move_to([x_air, y_mat, 0]),
            run_time=1.3,
            rate_func=smooth
        )

        # Impact Ripple Effect
        ripple_w = Circle(radius=0.1, color=ORANGE, stroke_width=3).move_to([x_water, y_mat, 0])
        ripple_c = Circle(radius=0.1, color=ORANGE, stroke_width=3).move_to([x_concrete, y_mat, 0])
        ripple_a = Circle(radius=0.1, color=ORANGE, stroke_width=3).move_to([x_air, y_mat, 0])

        self.add(ripple_w, ripple_c, ripple_a)
        self.play(
            FadeOut(orb_w), FadeOut(orb_c), FadeOut(orb_a),
            ripple_w.animate.scale(6).set_stroke(opacity=0),
            ripple_c.animate.scale(6).set_stroke(opacity=0),
            ripple_a.animate.scale(6).set_stroke(opacity=0),
            run_time=0.7
        )
        self.remove(ripple_w, ripple_c, ripple_a)

        # --- Temperature Response Animations ---
        # Water: Tiny rise
        # Concrete: Moderate rise
        # Air: Shoots up high
        new_fluid_w = Rectangle(width=0.08, height=0.15, color=RED, fill_opacity=1, stroke_width=0)
        new_fluid_w.move_to([x_water, base_w + 0.075, 0])

        new_fluid_c = Rectangle(width=0.08, height=0.55, color=RED, fill_opacity=1, stroke_width=0)
        new_fluid_c.move_to([x_concrete, base_c + 0.275, 0])

        new_fluid_a = Rectangle(width=0.08, height=1.1, color=RED, fill_opacity=1, stroke_width=0)
        new_fluid_a.move_to([x_air, base_a + 0.55, 0])

        self.play(
            Transform(fluid_w, new_fluid_w),
            Transform(fluid_c, new_fluid_c),
            Transform(fluid_a, new_fluid_a),
            run_time=2.5,
            rate_func=smooth
        )
        self.wait(0.8)

        # Add Capacity Labels above columns
        cap_w = Text("c = 4.18 kJ/(kg K)\n(Hohe Kapazität)", font="Arial", font_size=13, color=CYAN_COLOR)
        cap_w.next_to(therm_w_frame, UP, buff=0.15)

        cap_c = Text("c = 0.88 kJ/(kg K)\n(Moderat)", font="Arial", font_size=13, color=LIGHT_GREY)
        cap_c.next_to(therm_c_frame, UP, buff=0.15)

        cap_a = Text("c = 1.00 kJ/(kg K)\n(Schneller Anstieg)", font="Arial", font_size=13, color=BLUE_B)
        cap_a.next_to(therm_a_frame, UP, buff=0.15)

        self.play(
            FadeIn(cap_w),
            FadeIn(cap_c),
            FadeIn(cap_a),
            run_time=1.2
        )

        self.wait(2.0)



class Scene4(Scene):
    def construct(self):
        # Set dark space background
        self.camera.background_color = "#0B0C10"

        # --- 1. Water Square & Base Heat Equation Intro ---
        water_box = Square(side_length=2.0, color=TEAL, fill_opacity=0.3)
        water_box.set_stroke(TEAL, width=3)
        water_label = Text("1 kg Wasser", font="Arial", font_size=22, color=TEAL).move_to(water_box)
        water_group = VGroup(water_box, water_label).shift(UP * 1.3)

        eq1 = Text("E = m * c * ΔT", font="Arial", font_size=40, color=WHITE)
        eq1.next_to(water_group, DOWN, buff=0.5)

        self.play(
            Create(water_box),
            Write(water_label),
            run_time=1.8
        )
        self.wait(1.0)

        self.play(Write(eq1), run_time=1.8)
        self.wait(1.5)

        # --- 2. Variable Substitution ---
        eq2 = Text("E = (1 kg) * (4.184 kJ/kg·K) * (1 K)", font="Arial", font_size=23, color=WHITE)
        eq2.move_to(eq1)

        self.play(
            Transform(eq1, eq2),
            run_time=2.0
        )
        self.wait(1.5)

        # Result calculation
        eq3 = Text("E = 4.184 kJ", font="Arial", font_size=38, color=YELLOW)
        eq3.move_to(eq1)

        self.play(Transform(eq1, eq3), run_time=1.8)
        self.wait(2.0)

        # --- 3. Transition to Upper Corner & Power Concept ---
        top_left_target = VGroup(water_group, eq1)
        
        self.play(
            top_left_target.animate.scale(0.65).to_corner(UL, buff=0.5),
            run_time=2.2
        )
        self.wait(1.2)

        # Power Concept Box
        power_box = Rectangle(height=2.8, width=7.2, color=BLUE_D, fill_color="#121824", fill_opacity=0.9)
        power_box.shift(RIGHT * 1.5 + UP * 0.9)

        p_title = Text("Kontinuierliche Leistung", font="Arial", font_size=26, color=TEAL).move_to(power_box.get_top() + DOWN * 0.4)
        p_def = Text("1 Watt = 1 Joule / Sekunde", font="Arial", font_size=24, color=BLUE_A).next_to(p_title, DOWN, buff=0.25)
        p_rate = Text("Um 1 K pro Sekunde zu erhitzen:", font="Arial", font_size=22, color=WHITE).next_to(p_def, DOWN, buff=0.25)
        p_watts = Text("4.184 kJ/s = 4.184 Watt", font="Arial", font_size=30, color=ORANGE).next_to(p_rate, DOWN, buff=0.25)

        self.play(
            FadeIn(power_box),
            Write(p_title),
            Write(p_def),
            run_time=2.0
        )
        self.wait(1.5)

        self.play(
            Write(p_rate),
            FadeIn(p_watts, shift=UP * 0.2),
            run_time=2.0
        )
        self.wait(2.0)

        # --- 4. Toaster Visual Comparison ---
        def make_toaster():
            body = RoundedRectangle(corner_radius=0.12, height=0.85, width=1.1, color=GREY_B, fill_color=GREY_D, fill_opacity=0.8)
            slot = Line(LEFT * 0.28, RIGHT * 0.28, color=DARK_GREY, stroke_width=3.5).shift(UP * 0.28)
            knob = Circle(radius=0.06, color=RED, fill_opacity=1).shift(DOWN * 0.18 + RIGHT * 0.28)
            label = Text("1.000 W", font="Arial", font_size=12, color=YELLOW).shift(DOWN * 0.18 + LEFT * 0.12)
            return VGroup(body, slot, knob, label)

        toasters = VGroup(*[make_toaster() for _ in range(4)]).arrange(RIGHT, buff=0.4)
        toasters.shift(DOWN * 1.7 + RIGHT * 0.2)

        toast_title = Text("4 Toaster = 4.000 W", font="Arial", font_size=20, color=GREY_A).next_to(toasters, UP, buff=0.2)

        self.play(
            FadeIn(toast_title),
            LaggedStart(*[GrowFromCenter(t) for t in toasters], lag_ratio=0.3),
            run_time=2.5
        )
        self.wait(1.5)

        # Energetic red comparison text
        comp_text = Text("4.184 W > 4 Toaster!", font="Arial", font_size=32, color=RED)
        comp_text.next_to(toasters, DOWN, buff=0.3)

        self.play(Write(comp_text), run_time=1.8)
        self.wait(1.0)

        # Slow, deliberate pulsing animation for emphasis
        self.play(
            comp_text.animate.scale(1.2).set_color(RED_A),
            rate_func=there_and_back,
            run_time=1.8
        )
        self.wait(0.5)

        self.play(
            comp_text.animate.scale(1.2).set_color(RED_A),
            rate_func=there_and_back,
            run_time=1.8
        )

        self.wait(2.5)



class FullIntroductionVideo(Scene):
    def construct(self):
        scenes = [Scene1, Scene2, Scene3, Scene4]
        base_dir = os.path.dirname(os.path.abspath(__file__))
        audio_files = [
            os.path.join(base_dir, f"scene_{i}_audio.mp3")
            for i in range(1, 5)
        ]
        
        for scene_cls, audio_path in zip(scenes, audio_files):
            if os.path.exists(audio_path):
                self.add_sound(audio_path)
            scene_cls.construct(self)
            self.clear()
