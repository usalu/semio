import os
import numpy as np
import math
from manim import *


class Scene1(Scene):
    def construct(self):
        # Background setup
        self.camera.background_color = "#0B0C10"
        Text.set_default(font="Serif")

        # --- TITLE & SUBTITLE ---
        title = Text("Makroskopische und Mikroskopische Wärmeleitung", font_size=28, color=WHITE)
        title.to_edge(UP, buff=0.35)
        self.play(Write(title))

      

        # --- BEAT 1: MACROSCOPIC BLOCKS ---
        # Touching hot pastel red and cold pastel blue blocks
        left_block = Rectangle(
            width=2.5, height=2.5, fill_color="#FF6B6B", fill_opacity=0.85,
            stroke_color=WHITE, stroke_width=2
        ).move_to(np.array([-1.25, -0.2, 0]))

        right_block = Rectangle(
            width=2.5, height=2.5, fill_color="#4D96FF", fill_opacity=0.85,
            stroke_color=WHITE, stroke_width=2
        ).move_to(np.array([1.25, -0.2, 0]))

        hot_label = Text("HEISS", font_size=22, color=WHITE).move_to(left_block.get_center())
        cold_label = Text("KALT", font_size=22, color=WHITE).move_to(right_block.get_center())

        self.play(
            FadeIn(left_block, shift=RIGHT * 0.3),
            FadeIn(right_block, shift=LEFT * 0.3),
            Write(hot_label),
            Write(cold_label),
            run_time=1.2
        )
        self.wait(1.2)

        # --- BEAT 2: THERMAL EQUILIBRIUM ---
        eq_label = Text("WARM (GLEICHGEWICHT)", font_size=20, color=WHITE).move_to(np.array([0, -0.2, 0]))

        self.play(
            left_block.animate.set_fill("#9B51E0"),
            right_block.animate.set_fill("#9B51E0"),
            ReplacementTransform(VGroup(hot_label, cold_label), eq_label),
            run_time=1.5
        )
        self.wait(1.5)

        # --- BEAT 3: TRANSITION TO MICROSCOPIC VIEW ---
        new_subtitle = Text("Mikroskopisches Gitter & Wärmebarriere", font_size=20, color=GREY_A)
        new_subtitle.next_to(title, DOWN, buff=0.3)

        self.play(
            FadeOut(left_block),
            FadeOut(right_block),
            FadeOut(eq_label),
            FadeIn(new_subtitle),
            run_time=1.0
        )

        # 8x12 Grid of Atoms Setup
        num_rows = 8
        num_cols = 12
        start_x = -4.2
        start_y = 1.4
        dx = 0.65
        dy = 0.45

        # Position barrier between column 7 (x=0.35) and column 8 (x=1.00)
        insulation_x = 0.68

        dots = []
        dots_group = VGroup()
        phases = {}

        for r in range(num_rows):
            for c in range(num_cols):
                pos = np.array([start_x + c * dx, start_y - r * dy, 0])
                dot = Dot(point=pos, radius=0.09, color="#4D96FF")
                dot.init_pos = pos.copy()
                dot.is_hot = False
                dots.append(dot)
                dots_group.add(dot)
                phases[dot] = (r * 1.3 + c * 2.7)

        # Time tracker for physical wave jitter
        time_tracker = ValueTracker(0)
        time_tracker.add_updater(lambda m, dt: m.increment_value(dt))
        self.add(time_tracker)

        # Dot jitter updaters
        for dot in dots:
            def make_updater(d):
                p = phases[d]
                def updater(mob):
                    t = time_tracker.get_value()
                    if d.is_hot:
                        amp = 0.08
                        freq = 24.0
                    else:
                        amp = 0.015
                        freq = 8.0

                    offset = np.array([
                        amp * np.sin(freq * t + p),
                        amp * np.cos(freq * t * 1.3 + p * 0.7),
                        0
                    ])
                    mob.move_to(d.init_pos + offset)
                return updater

            dot.add_updater(make_updater(dot))

        self.play(FadeIn(dots_group), run_time=1.0)
        self.wait(0.8)

        # --- BEAT 4: INSULATION LAYER ---
        barrier_rect = Rectangle(
            width=0.45,
            height=num_rows * dy + 0.3,
            fill_color="#F2C94C",
            fill_opacity=0.2,
            stroke_color="#F2C94C",
            stroke_width=2
        ).move_to(np.array([insulation_x, start_y - (num_rows - 1) * dy / 2, 0]))

        y_min = barrier_rect.get_bottom()[1]
        y_max = barrier_rect.get_top()[1]

        # Air pockets within the thermal barrier
        air_gaps = VGroup()
        for y_c in np.linspace(y_min + 0.3, y_max - 0.3, 5):
            gap = Circle(
                radius=0.08,
                fill_color="#0B0C10",
                fill_opacity=1.0,
                stroke_color="#F2C94C",
                stroke_width=1.5
            ).move_to(np.array([insulation_x, y_c, 0]))
            air_gaps.add(gap)

        ins_text = Text("Dämmschicht (DIN 4108)", font_size=14, color="#F2C94C")
        ins_text.next_to(barrier_rect, UP, buff=0.15)

        insulation_barrier = VGroup(barrier_rect, air_gaps, ins_text)

        self.play(Create(insulation_barrier), run_time=1.2)
        self.wait(1.0)

        # --- BEAT 5: HEAT SWEEP WAVE ---
        sweep_x = ValueTracker(start_x - 0.5)

        sweep_line = Line(
            start=np.array([start_x - 0.5, start_y + 0.3, 0]),
            end=np.array([start_x - 0.5, start_y - (num_rows - 1) * dy - 0.3, 0]),
            color="#FF9F43",
            stroke_width=5
        )

        def sweep_line_updater(m):
            x_val = sweep_x.get_value()
            m.set_x(x_val)

        sweep_line.add_updater(sweep_line_updater)
        self.add(sweep_line)

        # Updater to heat up atoms as sweep line passes over them
        def heat_propagation_updater(dt):
            x_val = sweep_x.get_value()
            for d in dots:
                if d.init_pos[0] <= x_val and d.init_pos[0] < insulation_x - 0.25:
                    if not d.is_hot:
                        d.is_hot = True
                        d.set_color("#FF9F43")

        self.add_updater(heat_propagation_updater)

        # Move glowing sweep line across lattice smoothly, paced to voiceover track
        target_x = insulation_x - 0.22
        self.play(
            sweep_x.animate.set_value(target_x),
            run_time=4.0,
            rate_func=linear
        )

        # Stop heat propagation at the insulation barrier
        self.remove_updater(heat_propagation_updater)
        sweep_line.clear_updaters()

        # Explanatory callout text
        blocked_text = Text("Lufteinschlüsse stoppen die Wärmeleitung", font_size=16, color="#FF9F43")
        blocked_text.next_to(barrier_rect, DOWN, buff=0.2)

        self.play(Write(blocked_text), run_time=1.0)

        # Final hold showing energetic kinetic atoms on left and quiet blue atoms on right
        self.wait(3)



class Scene2(Scene):
    def construct(self):
        # 1. Background Setup
        self.camera.background_color = "#0B0C10"

        # 2. Title
        title = Text("Wärmedurchlasswiderstand: Der R-Wert", font_size=32, color="#F0F0F0")
        title.to_edge(UP, buff=0.4)

        self.play(
            Write(title),
            run_time=1.5
        )

        # 3. Microscopic Grid Representation
        grid_dots = VGroup(*[
            Dot(
                point=[x * 0.35, y * 0.35, 0],
                radius=0.07,
                color="#80DEEA"
            )
            for x in range(-2, 3) for y in range(-3, 4)
        ])
        grid_dots.move_to(ORIGIN)

        self.play(FadeIn(grid_dots, lag_ratio=0.03), run_time=1.5)
        self.wait(0.5)

        # 4. Transform Atom Grid into Solid Insulation Slab
        rect = Rectangle(
            width=1.5,
            height=2.2,
            color="#FFD54F",
            fill_color="#C59B27",
            fill_opacity=0.85
        )
        rect.move_to(ORIGIN)

        rect_label = Text("Dämmung", font_size=20, color=WHITE)
        rect_label.move_to(rect.get_center())

        self.play(
            ReplacementTransform(grid_dots, rect),
            FadeIn(rect_label),
            run_time=2.0
        )
        self.wait(0.5)

        # 5. Display R-Value Formula above
        formula = Text("R = d / λ", font_size=30, color="#FFE066")
        formula.next_to(title, DOWN, buff=0.25)

        formula_sub = Text(
            "R: Wärmedurchlasswiderstand | d: Dicke | λ: Wärmeleitfähigkeit (DIN 4108)",
            font_size=16,
            color="#B0BEC5"
        )
        formula_sub.next_to(formula, DOWN, buff=0.12)

        self.play(
            Write(formula),
            FadeIn(formula_sub),
            run_time=1.5
        )

        # 6. Attach Initial Brace, Dimension 'd', and R-Value Counter
        r_var = ValueTracker(2.0)

        brace = Brace(rect, DOWN, buff=0.15, color="#E0E0E0")
        d_label = Text("d", font_size=22, color="#E0E0E0")
        d_label.next_to(brace, DOWN, buff=0.1)

        r_label = Text("R = 2.0", font_size=28, color="#FFE066")
        r_label.next_to(rect, UP, buff=0.25)

        self.play(
            GrowFromCenter(brace),
            FadeIn(d_label),
            FadeIn(r_label),
            run_time=1.5
        )
        self.wait(0.5)

        # 7. Dynamically Stretch Insulation Slab and Update R-Value Counter
        brace.add_updater(lambda b: b.become(Brace(rect, DOWN, buff=0.15, color="#E0E0E0")))
        d_label.add_updater(lambda t: t.next_to(brace, DOWN, buff=0.1))
        r_label.add_updater(
            lambda t: t.become(
                Text(f"R = {r_var.get_value():.1f}", font_size=28, color="#FFE066").next_to(rect, UP, buff=0.25)
            )
        )
        rect_label.add_updater(lambda t: t.move_to(rect.get_center()))

        self.play(
            rect.animate.stretch_to_fit_width(3.2),
            r_var.animate.set_value(4.0),
            run_time=4.0,
            rate_func=smooth
        )

        # Clear updaters for static elements
        brace.clear_updaters()
        d_label.clear_updaters()
        r_label.clear_updaters()
        rect_label.clear_updaters()

        # 8. Highlight Linear Scaling Conclusion
        highlight_box = SurroundingRectangle(r_label, color="#FFE066", buff=0.1)
        double_text = Text("2x Dicke → 2x Widerstand", font_size=18, color="#81C784")
        double_text.next_to(rect, RIGHT, buff=0.4)

        self.play(
            Create(highlight_box),
            FadeIn(double_text, shift=LEFT),
            run_time=1.5
        )
        self.wait(1.0)

        # 9. Final Hold
        self.wait(2)



class Scene3(Scene):
    def construct(self):
        self.camera.background_color = "#0B0C10"
        
        pastel_yellow = "#FFE66D"
        pastel_blue = "#A0C4FF"
        pastel_red = "#FF6B6B"
        pastel_green = "#CAFFBF"
        
        # --- Title ---
        main_title = Text("Wärmedurchgangskoeffizient & Temperaturgradient", font_size=28, color=WHITE)
        main_title.to_edge(UP, buff=0.4)
        self.play(Write(main_title), run_time=0.8)
        self.wait(0.3)
        
        # --- Beat 1: R to U = 1 / R ---
        r_text = Text("Wärmedurchlasswiderstand: R", font_size=30, color=pastel_yellow)
        r_text.move_to(UP * 1.2)
        self.play(FadeIn(r_text, shift=UP * 0.2), run_time=0.8)
        self.wait(0.5)
        
        u_eq = Text("Wärmedurchgangskoeffizient: U = 1 / R", font_size=30, color=WHITE)
        u_eq.move_to(UP * 1.2)
        
        u_subtext = Text("U-Wert misst den Wärmestrom pro Fläche und Temperaturdifferenz", font_size=16, color=pastel_blue)
        u_subtext.next_to(u_eq, DOWN, buff=0.35)
        
        self.play(
            ReplacementTransform(r_text, u_eq),
            run_time=1.0
        )
        self.play(FadeIn(u_subtext, shift=UP * 0.1), run_time=0.8)
        self.wait(1.0)
        
        # --- Beat 2: Morph 1x1 unit square into 5x5 grid (Area A) ---
        u_badge = Text("U = 1 / R", font_size=20, color=pastel_yellow)
        u_badge.to_corner(UL, buff=0.5)
        
        area_title = Text("Wärmeverlust skaliert mit der Oberfläche (A)", font_size=26, color=WHITE)
        area_title.to_edge(UP, buff=0.4)
        
        self.play(
            FadeOut(main_title),
            FadeOut(u_subtext),
            ReplacementTransform(u_eq, u_badge),
            Write(area_title),
            run_time=0.8
        )
        
        unit_square = Square(side_length=1.2, color=pastel_red, fill_opacity=0.3, stroke_width=2)
        unit_square.move_to(DOWN * 0.3)
        unit_label = Text("Flächeneinheit = 1 m²", font_size=18, color=pastel_red)
        unit_label.next_to(unit_square, DOWN, buff=0.3)
        
        self.play(
            Create(unit_square),
            FadeIn(unit_label),
            run_time=0.8
        )
        self.wait(0.5)
        
        grid = VGroup()
        for i in range(5):
            for j in range(5):
                sq = Square(side_length=0.48, color=pastel_green, fill_opacity=0.25, stroke_width=1.5)
                sq.move_to(np.array([(j - 2) * 0.5, (2 - i) * 0.5 - 0.3, 0]))
                grid.add(sq)
        
        grid_label = Text("Gesamtoberfläche (A) = 25 m²", font_size=18, color=pastel_green)
        grid_label.next_to(grid, DOWN, buff=0.3)
        
        self.play(
            ReplacementTransform(unit_square, grid),
            ReplacementTransform(unit_label, grid_label),
            run_time=1.2
        )
        self.wait(1.0)
        
        # Clear Beat 2 elements
        self.play(
            FadeOut(u_badge),
            FadeOut(area_title),
            FadeOut(grid),
            FadeOut(grid_label),
            run_time=0.6
        )
        
        # --- Beat 3 & 4: 2D Temperature vs Position Axes & Delta T ---
        graph_title = Text("Temperaturprofil durch die Wand", font_size=26, color=WHITE)
        graph_title.to_edge(UP, buff=0.4)
        self.play(Write(graph_title), run_time=0.8)
        
        axes = Axes(
            x_range=[0, 3, 1],
            y_range=[-25, 30, 10],
            x_length=5.5,
            y_length=4.2,
            axis_config={"include_numbers": False, "color": GREY}
        ).move_to(DOWN * 0.3 + LEFT * 0.6)
        
        x_label_in = Text("Innen", font_size=15, color=WHITE).move_to(axes.c2p(0.5, -25) + DOWN * 0.3)
        x_label_wall = Text("Wand", font_size=15, color=WHITE).move_to(axes.c2p(1.5, -25) + DOWN * 0.3)
        x_label_out = Text("Außen", font_size=15, color=WHITE).move_to(axes.c2p(2.5, -25) + DOWN * 0.3)
        
        y_label_20 = Text("20°C", font_size=15, color=pastel_red).next_to(axes.c2p(0, 20), LEFT, buff=0.15)
        y_label_m5 = Text("-5°C", font_size=15, color=pastel_blue).next_to(axes.c2p(0, -5), LEFT, buff=0.15)
        
        self.play(
            Create(axes),
            FadeIn(x_label_in),
            FadeIn(x_label_wall),
            FadeIn(x_label_out),
            FadeIn(y_label_20),
            FadeIn(y_label_m5),
            run_time=1.0
        )
        
        p_in_20 = axes.c2p(0.5, 20)
        p_w1_20 = axes.c2p(1.0, 20)
        p_w2_m5 = axes.c2p(2.0, -5)
        p_out_m5 = axes.c2p(2.5, -5)
        
        temp_line1 = Line(p_in_20, p_w1_20, color=pastel_red, stroke_width=3)
        temp_line_wall = Line(p_w1_20, p_w2_m5, color=pastel_yellow, stroke_width=3)
        temp_line_out = Line(p_w2_m5, p_out_m5, color=pastel_blue, stroke_width=3)
        
        temp_profile = VGroup(temp_line1, temp_line_wall, temp_line_out)
        
        dot_in = Dot(p_in_20, color=pastel_red, radius=0.08)
        dot_out = Dot(p_out_m5, color=pastel_blue, radius=0.08)
        
        self.play(
            Create(temp_profile),
            FadeIn(dot_in),
            FadeIn(dot_out),
            run_time=1.0
        )
        
        b_top = axes.c2p(2.7, 20)
        b_bot = axes.c2p(2.7, -5)
        
        brace = BraceBetweenPoints(b_top, b_bot, direction=RIGHT, color=pastel_yellow)
        brace_label = Text("ΔT = 25°C", font_size=16, color=pastel_yellow)
        brace_label.next_to(brace, RIGHT, buff=1.2)
        
        self.play(
            Create(brace),
            FadeIn(brace_label),
            run_time=0.8
        )
        self.wait(1.0)
        
        # Drop outdoor temperature to -15°C
        p_w2_m15 = axes.c2p(2.0, -15)
        p_out_m15 = axes.c2p(2.5, -15)
        
        y_label_m15 = Text("-15°C", font_size=15, color=pastel_blue).next_to(axes.c2p(0, -15), LEFT, buff=0.15)
        
        temp_line_wall_new = Line(p_w1_20, p_w2_m15, color=pastel_yellow, stroke_width=3)
        temp_line_out_new = Line(p_w2_m15, p_out_m15, color=pastel_blue, stroke_width=3)
        
        b_bot_new = axes.c2p(2.7, -15)
        brace_new = BraceBetweenPoints(b_top, b_bot_new, direction=RIGHT, color=pastel_yellow)
        brace_label_new = Text("ΔT = 35°C", font_size=16, color=pastel_yellow)
        brace_label_new.next_to(brace_new, RIGHT, buff=0.2)
        
        steep_note = Text("Steilerer Gradient = Höherer Wärmestrom", font_size=18, color=pastel_yellow)
        steep_note.to_edge(DOWN, buff=0.4)
        
        self.play(
            Transform(y_label_m5, y_label_m15),
            Transform(temp_line_wall, temp_line_wall_new),
            Transform(temp_line_out, temp_line_out_new),
            dot_out.animate.move_to(p_out_m15),
            ReplacementTransform(brace, brace_new),
            ReplacementTransform(brace_label, brace_label_new),
            Write(steep_note),
            run_time=1.5
        )
        
        self.wait(2)



class Scene4(Scene):
    def construct(self):
        # Set dark theme background
        self.camera.background_color = "#0B0C10"

        # Color definitions
        pastel_white = "#E0E6ED"
        pastel_cyan = "#66FCF1"
        pastel_teal = "#45A29E"
        pastel_orange = "#FFAAA5"
        pastel_yellow = "#DCEDC1"

        # --- Title & Master Equation ---
        title = Text("Die thermische Gebäudehülle", font_size=32, color=pastel_cyan)
        title.to_edge(UP, buff=0.3)

        eq_text = Text("Transmissionswärmeverlust = Σ ( U_i × A_i × ΔT_i ) (DIN EN ISO 13789)", font_size=20, color=pastel_white)
        eq_text.next_to(title, DOWN, buff=0.4)

        self.play(Write(title), run_time=1.2)
        self.play(FadeIn(eq_text, shift=DOWN * 0.15), run_time=1.2)
        self.wait(0.5)

        # --- House Line-Art Setup ---
        house_center = DOWN * 0.5
        w_width = 3.0
        w_height = 1.8

        bottom_left = house_center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
        bottom_right = house_center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
        top_left = house_center + LEFT * (w_width / 2) + UP * (w_height / 2)
        top_right = house_center + RIGHT * (w_width / 2) + UP * (w_height / 2)
        roof_peak = house_center + UP * (w_height / 2 + 1.0)

        floor_line = Line(bottom_left + LEFT * 0.6, bottom_right + RIGHT * 0.6, color=pastel_teal, stroke_width=3)
        walls = VGroup(
            Line(bottom_left, top_left, color=pastel_white, stroke_width=3),
            Line(bottom_right, top_right, color=pastel_white, stroke_width=3)
        )
        roof = Polygon(top_left, roof_peak, top_right, color=pastel_white, stroke_width=3)

        # Window - positioned in upper-left quadrant of house
        window = Square(side_length=0.6, color=pastel_cyan, stroke_width=2)
        window.move_to(house_center + LEFT * 0.75 + UP * 0.25)
        window_cross = VGroup(
            Line(window.get_top(), window.get_bottom(), color=pastel_cyan, stroke_width=1.5),
            Line(window.get_left(), window.get_right(), color=pastel_cyan, stroke_width=1.5)
        )
        window_group = VGroup(window, window_cross)

        # Door - positioned in lower-right quadrant of house
        door = Rectangle(width=0.55, height=0.9, color=pastel_cyan, stroke_width=2)
        door.move_to(house_center + RIGHT * 0.65 + DOWN * 0.45)
        door_knob = Dot(door.get_center() + LEFT * 0.15 + DOWN * 0.05, radius=0.035, color=pastel_cyan)
        door_group = VGroup(door, door_knob)

        # Draw line-art house
        self.play(
            Create(floor_line),
            Create(walls),
            Create(roof),
            run_time=1.8
        )
        self.play(
            Create(window_group),
            Create(door_group),
            run_time=1.2
        )
        self.wait(0.5)

        # --- Heat Loss Pathways (Arrows & Labels aligned with elements) ---

        # 1. Roof pathway (Top)
        roof_arrow = Arrow(
            roof_peak + UP * 0.05,
            roof_peak + UP * 0.65,
            color=pastel_orange,
            buff=0,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.25
        )
        roof_label = Text("Dach", font_size=16, color=pastel_orange).next_to(roof_arrow, UP, buff=0.1)

        # 2. Window pathway (Horizontal from window left edge)
        win_start = window.get_left()
        win_end = house_center + LEFT * 2.25 + UP * 0.25
        win_arrow = Arrow(
            win_start,
            win_end,
            color=pastel_orange,
            buff=0.05,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.25
        )
        win_label = Text("Fenster", font_size=16, color=pastel_orange).next_to(win_arrow, LEFT, buff=0.1)

        # 3. Wall pathway (Horizontal from left wall line below window)
        wall_start = house_center + LEFT * 1.5 + DOWN * 0.25
        wall_end = house_center + LEFT * 2.25 + DOWN * 0.25
        wall_l_arrow = Arrow(
            wall_start,
            wall_end,
            color=pastel_orange,
            buff=0.05,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.25
        )
        wall_l_label = Text("Wände", font_size=16, color=pastel_orange).next_to(wall_l_arrow, LEFT, buff=0.1)

        # 4. Door pathway (Horizontal from door right edge)
        door_start = door.get_right()
        door_end = house_center + RIGHT * 2.25 + DOWN * 0.45
        door_arrow = Arrow(
            door_start,
            door_end,
            color=pastel_orange,
            buff=0.05,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.25
        )
        door_label = Text("Türen", font_size=16, color=pastel_orange).next_to(door_arrow, RIGHT, buff=0.1)

        # 5. Foundation pathway (Bottom)
        floor_arrow = Arrow(
            house_center + DOWN * 0.9,
            house_center + DOWN * 1.5,
            color=pastel_orange,
            buff=0,
            stroke_width=3,
            max_tip_length_to_length_ratio=0.25
        )
        floor_label = Text("Fundament", font_size=16, color=pastel_orange).next_to(floor_arrow, DOWN, buff=0.1)

        pathways = [
            (roof_arrow, roof_label),
            (win_arrow, win_label),
            (wall_l_arrow, wall_l_label),
            (door_arrow, door_label),
            (floor_arrow, floor_label)
        ]

        # Fade in radiating heat loss arrows and labels
        self.play(
            LaggedStart(
                *[GrowArrow(arrow) for arrow, _ in pathways],
                lag_ratio=0.2
            ),
            LaggedStart(
                *[FadeIn(label, shift=label.get_center() * 0.05) for _, label in pathways],
                lag_ratio=0.2
            ),
            run_time=2.5
        )
        self.wait(0.8)

        # --- Glowing & Sync Animation ---
        box = SurroundingRectangle(eq_text, color=pastel_yellow, buff=0.15, corner_radius=0.08)
        self.play(Create(box), run_time=0.8)

        # Sequential glow highlighting each envelope component
        for arrow, label in pathways:
            self.play(
                arrow.animate.set_color(pastel_yellow),
                label.animate.set_color(pastel_yellow),
                run_time=0.3
            )
            self.play(
                arrow.animate.set_color(pastel_orange),
                label.animate.set_color(pastel_orange),
                run_time=0.3
            )

        # Global sync glow
        self.play(
            eq_text.animate.set_color(pastel_yellow),
            *[arrow.animate.set_color(pastel_yellow) for arrow, _ in pathways],
            *[label.animate.set_color(pastel_yellow) for _, label in pathways],
            run_time=1.0
        )
        self.wait(0.5)

        # Reset equation color
        self.play(
            eq_text.animate.set_color(pastel_white),
            *[arrow.animate.set_color(pastel_orange) for arrow, _ in pathways],
            *[label.animate.set_color(pastel_orange) for _, label in pathways],
            run_time=1.0
        )

        # Subtitle summary note at bottom
        note = Text("Summe der Hüllverluste: Dach + Fenster + Wände + Türen + Fundament", font_size=18, color=pastel_teal)
        note.to_edge(DOWN, buff=0.3)
        self.play(FadeIn(note, shift=UP * 0.15), run_time=1.2)

        self.wait(2.0)


class FullConductionVideo(Scene):
    def construct(self):
        scenes = [Scene1, Scene2, Scene3, Scene4]
        base_dir = os.path.dirname(os.path.abspath(__file__))
        audio_files = [
            os.path.join(base_dir, f"scene_{i}_german_audio.mp3")
            for i in range(1, 5)
        ]
        
        for scene_cls, audio_path in zip(scenes, audio_files):
            if os.path.exists(audio_path):
                self.add_sound(audio_path)
            scene_cls.construct(self)
            self.clear()
