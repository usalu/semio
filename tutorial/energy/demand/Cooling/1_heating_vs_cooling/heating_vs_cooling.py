import numpy as np
from manim import *

class HeatingVsCooling(Scene):
    def construct(self):
        # Color Palette
        P_DEEP_DARK = "#0B0C10"
        P_WHITE = "#E0E6ED"
        P_CYAN = "#66FCF1"
        P_TEAL = "#45A29E"
        P_ORANGE = "#FFAAA5"
        P_YELLOW = "#FFE66D"
        P_RED = "#FF6B6B"
        P_GREEN = "#CAFFBF"

        self.camera.background_color = P_DEEP_DARK

        # ----------------------------------------------------
        # BEAT 1: The Winter Recap (The Helpful Gains)
        # ----------------------------------------------------
        Text.set_default(font="Serif")
        title = Text("Heizlast vs. Kühllast", font_size=36, color=P_WHITE)
        title.to_edge(UP, buff=0.4)

        subtitle = Text("Im Winter: Erwünschte kostenlose Wärme", font_size=20, color=P_GREEN)
        subtitle.next_to(title, DOWN, buff=0.2)

        # Cross-Section House Setup
        house_center = LEFT * 0.8 + DOWN * 0.85
        w_width, w_height = 3.6, 2.4

        bottom_left = house_center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
        bottom_right = house_center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
        top_left = house_center + LEFT * (w_width / 2) + UP * (w_height / 2)
        top_right = house_center + RIGHT * (w_width / 2) + UP * (w_height / 2)
        roof_peak = house_center + UP * (w_height / 2 + 1.1)

        floor_line = Line(bottom_left + LEFT * 0.6, bottom_right + RIGHT * 0.6, color=P_TEAL, stroke_width=4)
        
        # Levels
        level_1 = Line(bottom_left + UP * (w_height / 2), bottom_right + UP * (w_height / 2), color=P_WHITE, stroke_width=2)
        
        # Left wall with 2 windows
        w_h = 0.5
        wall_left_1 = Line(bottom_left, bottom_left + UP * (w_height/4 - w_h/2), color=P_WHITE, stroke_width=3)
        wall_left_2 = Line(bottom_left + UP * (w_height/4 + w_h/2), bottom_left + UP * (3*w_height/4 - w_h/2), color=P_WHITE, stroke_width=3)
        wall_left_3 = Line(bottom_left + UP * (3*w_height/4 + w_h/2), top_left, color=P_WHITE, stroke_width=3)
        
        wall_right = Line(bottom_right, top_right, color=P_WHITE, stroke_width=3)
        walls = VGroup(wall_left_1, wall_left_2, wall_left_3, wall_right, level_1)
        roof = Polygon(top_left, roof_peak, top_right, color=P_WHITE, stroke_width=3)

        # Windows
        win1 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (w_height/4) + RIGHT * 0.075)
        win2 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (3*w_height/4) + RIGHT * 0.075)
        window = win2  # For reference later
        window_group = VGroup(win1, win2)
        door_group = VGroup()

        # Sun
        sun_pos = house_center + LEFT * 4.2 + UP * 2.95
        sun_core = Dot(sun_pos, radius=0.45, color=P_YELLOW)
        sun_glow = Dot(sun_pos, radius=0.7, color=P_YELLOW, fill_opacity=0.35)
        sun_ring1 = Circle(radius=0.85, color=P_YELLOW, stroke_width=2, stroke_opacity=0.6).move_to(sun_pos)
        sun_ring2 = Circle(radius=1.1, color=P_YELLOW, stroke_width=1.2, stroke_opacity=0.3).move_to(sun_pos)
        sun_burst = VGroup()
        for angle in np.linspace(0, TAU, 12, endpoint=False):
            s = sun_pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
            e = sun_pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
            sun_burst.add(Line(s, e, color=P_YELLOW, stroke_width=2))
        sun_group = VGroup(sun_glow, sun_core, sun_ring1, sun_ring2, sun_burst)

        solar_label = Text("Solare Gewinne", font_size=16, color=P_YELLOW)
        solar_label.next_to(sun_core, DOWN, buff=1.0)

        # People & Devices (from internal_heat_gain)
        C_PINK = "#F472B6"
        C_BLUE = "#38BDF8"
        
        # 1st Floor: Device (Laptop & Desk)
        desk_surface = Line(bottom_left + RIGHT*1.8 + UP*(w_height/2 + 0.3), bottom_left + RIGHT*2.4 + UP*(w_height/2 + 0.3), color=P_WHITE, stroke_width=2)
        desk_leg1 = Line(desk_surface.get_start() + RIGHT*0.1, desk_surface.get_start() + RIGHT*0.1 + DOWN*0.3, color=P_WHITE, stroke_width=2)
        desk_leg2 = Line(desk_surface.get_end() + LEFT*0.1, desk_surface.get_end() + LEFT*0.1 + DOWN*0.3, color=P_WHITE, stroke_width=2)
        
        laptop_base = Line(desk_surface.get_center() + LEFT*0.1, desk_surface.get_center() + RIGHT*0.1, color=C_BLUE, stroke_width=2.5)
        laptop_screen = Line(laptop_base.get_right(), laptop_base.get_right() + LEFT*0.05 + UP*0.15, color=C_BLUE, stroke_width=2.5)
        laptop = VGroup(laptop_base, laptop_screen)
        device = VGroup(desk_surface, desk_leg1, desk_leg2, laptop)


        # Ground Floor: Kitchen & Cook (Standing)
        counter_top = Line(bottom_left + RIGHT * 1.5 + UP * 0.3, bottom_left + RIGHT * 2.3 + UP * 0.3, color=P_WHITE, stroke_width=2)
        counter_body = Rectangle(width=0.8, height=0.3, color=P_WHITE, stroke_width=1.5, fill_opacity=0.1).move_to(counter_top.get_center() + DOWN * 0.15)
        stove_pot = VGroup(
            Line(counter_top.get_center() + LEFT*0.1 + UP*0.02, counter_top.get_center() + RIGHT*0.1 + UP*0.02, color=P_WHITE, stroke_width=3),
            RoundedRectangle(corner_radius=0.03, width=0.16, height=0.12, color=C_BLUE, stroke_width=2, fill_opacity=0.3).move_to(counter_top.get_center() + RIGHT*0.1 + UP*0.08)
        )
        kitchen = VGroup(counter_top, counter_body, stove_pot)

        p2_head = Circle(radius=0.08, color=C_PINK, fill_color=C_PINK, fill_opacity=0.3, stroke_width=2)
        p2_head.move_to(bottom_left + RIGHT * 1.1 + UP * 0.62)
        p2_torso = Line(p2_head.get_bottom(), p2_head.get_bottom() + DOWN * 0.22, color=C_PINK, stroke_width=3)
        p2_legs = VGroup(
            Line(p2_torso.get_end(), p2_torso.get_end() + DOWN * 0.28 + LEFT * 0.05, color=C_PINK, stroke_width=3),
            Line(p2_torso.get_end(), p2_torso.get_end() + DOWN * 0.28 + RIGHT * 0.05, color=C_PINK, stroke_width=3)
        )
        p2_arms = Line(p2_torso.get_center() + UP * 0.03, p2_torso.get_center() + RIGHT * 0.2, color=C_PINK, stroke_width=2)
        person2 = VGroup(p2_head, p2_torso, p2_legs, p2_arms)

        # Ceiling Lights (1st Floor & Ground Floor)
        light1_cord = Line(bottom_left + UP * 2.4 + RIGHT * 1.2, bottom_left + UP * 2.15 + RIGHT * 1.2, color=P_WHITE, stroke_width=1.5)
        light1_bulb = Dot(light1_cord.get_end(), radius=0.07, color=P_YELLOW)
        light1_glow = Circle(radius=0.18, color=P_YELLOW, stroke_width=0, fill_opacity=0.1).move_to(light1_bulb.get_center())
        light1 = VGroup(light1_cord, light1_bulb, light1_glow)

        light2_cord = Line(bottom_left + UP * 1.2 + RIGHT * 2.7, bottom_left + UP * 0.95 + RIGHT * 2.7, color=P_WHITE, stroke_width=1.5)
        light2_bulb = Dot(light2_cord.get_end(), radius=0.07, color=P_YELLOW)
        light2_glow = Circle(radius=0.18, color=P_YELLOW, stroke_width=0, fill_opacity=0.1).move_to(light2_bulb.get_center())
        light2 = VGroup(light2_cord, light2_bulb, light2_glow)

        internal_dot = VGroup(device, person2, kitchen, light1, light2)

        # Heat Waves for EVERY device, person, and light
        def create_wave(x_offset, color, start_pos):
            pts = [
                start_pos + np.array([x_offset + 0.04 * np.sin(y * 8), y, 0])
                for y in np.linspace(0.08, 0.4, 12)
            ]
            wave = VMobject(color=color, stroke_width=1.8, stroke_opacity=0.7)
            wave.set_points_smoothly(pts)
            return wave


        d1_w1 = create_wave(-0.06, C_BLUE, laptop_screen.get_center())
        d1_w2 = create_wave(0.06, C_BLUE, laptop_screen.get_center())

        l1_w1 = create_wave(-0.06, P_YELLOW, light1_bulb.get_center())
        l1_w2 = create_wave(0.06, P_YELLOW, light1_bulb.get_center())

        p2_w1 = create_wave(-0.06, C_PINK, p2_head.get_top())
        p2_w2 = create_wave(0.06, C_PINK, p2_head.get_top())

        k_w1 = create_wave(-0.06, C_BLUE, stove_pot.get_top())
        k_w2 = create_wave(0.06, C_BLUE, stove_pot.get_top())

        l2_w1 = create_wave(-0.06, P_YELLOW, light2_bulb.get_center())
        l2_w2 = create_wave(0.06, P_YELLOW, light2_bulb.get_center())

        internal_waves = VGroup(d1_w1, d1_w2, l1_w1, l1_w2, p2_w1, p2_w2, k_w1, k_w2, l2_w1, l2_w2)

        internal_label = Text("Interne Gewinne", font_size=15, color=C_PINK)
        internal_label.next_to(device, UP, buff=0.4)

        # Animate Beat 1
        self.play(Write(title), Write(subtitle), run_time=1.0)
        self.play(
            Create(floor_line),
            Create(walls),
            Create(roof),
            Create(window_group),
            Create(door_group),
            run_time=2.0
        )
        self.play(
            FadeIn(sun_glow), FadeIn(sun_core), Create(sun_ring1), Create(sun_ring2), Create(sun_burst),
            FadeIn(solar_label),
            FadeIn(internal_dot),
            Create(internal_waves),
            FadeIn(internal_label),
            run_time=1.8
        )
        self.wait(2.0)

        # ----------------------------------------------------
        # BEAT 2 & 3: The Seasonal Shift & Greenhouse Trap
        # ----------------------------------------------------
        new_subtitle = Text("Im Sommer: Überhitzung (Treibhauseffekt)", font_size=20, color=P_RED)
        new_subtitle.next_to(title, DOWN, buff=0.2)

        # High Intensity Summer Sun
        sun_core_red = Dot(sun_pos, radius=0.45, color=P_RED)
        sun_glow_red = Dot(sun_pos, radius=0.7, color=P_RED, fill_opacity=0.45)
        sun_ring1_red = Circle(radius=0.85, color=P_RED, stroke_width=2, stroke_opacity=0.8).move_to(sun_pos)
        sun_ring2_red = Circle(radius=1.1, color=P_RED, stroke_width=1.5, stroke_opacity=0.7).move_to(sun_pos)
        sun_burst_red = VGroup()
        for angle in np.linspace(0, TAU, 12, endpoint=False):
            s = sun_pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
            e = sun_pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
            sun_burst_red.add(Line(s, e, color=P_RED, stroke_width=3))
        sun_group_red = VGroup(sun_glow_red, sun_core_red, sun_ring1_red, sun_ring2_red, sun_burst_red)

        solar_label_red = Text("Solare Gewinne (Exzessiv)", font_size=16, color=P_RED)
        solar_label_red.next_to(sun_core_red, DOWN, buff=1.5)

        internal_dot_red = internal_dot.copy().set_color(P_RED)

        d1_w1_r = create_wave(-0.06, P_RED, laptop_screen.get_center())
        d1_w2_r = create_wave(0.06, P_RED, laptop_screen.get_center())
        l1_w1_r = create_wave(-0.06, P_RED, light1_bulb.get_center())
        l1_w2_r = create_wave(0.06, P_RED, light1_bulb.get_center())
        p2_w1_r = create_wave(-0.06, P_RED, p2_head.get_top())
        p2_w2_r = create_wave(0.06, P_RED, p2_head.get_top())
        k_w1_r = create_wave(-0.06, P_RED, stove_pot.get_top())
        k_w2_r = create_wave(0.06, P_RED, stove_pot.get_top())
        l2_w1_r = create_wave(-0.06, P_RED, light2_bulb.get_center())
        l2_w2_r = create_wave(0.06, P_RED, light2_bulb.get_center())

        internal_waves_red = VGroup(d1_w1_r, d1_w2_r, l1_w1_r, l1_w2_r, p2_w1_r, p2_w2_r, k_w1_r, k_w2_r, l2_w1_r, l2_w2_r)
        
        internal_label_red = Text("Interne Gewinne", font_size=15, color=P_RED)
        internal_label_red.next_to(device, UP, buff=0.6)

        # Trapped heat polygon (house interior shape)
        house_interior_points = [
            bottom_left + RIGHT * 0.05 + UP * 0.05,
            bottom_right + LEFT * 0.05 + UP * 0.05,
            top_right + LEFT * 0.05 + DOWN * 0.05,
            roof_peak + DOWN * 0.1,
            top_left + RIGHT * 0.05 + DOWN * 0.05,
        ]
        heat_block = Polygon(*house_interior_points, fill_color=P_RED, fill_opacity=0.45, stroke_width=0)

        # Thermometer Graphic
        def create_thermometer(pos):
            bulb_outer = Circle(radius=0.18, color=P_WHITE, stroke_width=2)
            bulb_outer.move_to(pos + DOWN * 0.3)
            tube_outer = RoundedRectangle(corner_radius=0.06, width=0.14, height=1.0, color=P_WHITE, stroke_width=2)
            tube_outer.move_to(pos + UP * 0.25)
            
            bulb_inner = Circle(radius=0.12, color=P_CYAN, fill_opacity=1, stroke_width=0)
            bulb_inner.move_to(bulb_outer.get_center())
            
            baseline_y = bulb_outer.get_center()[1] + 0.1
            fluid_stem = Rectangle(width=0.06, height=0.2, color=P_CYAN, fill_opacity=1, stroke_width=0)
            fluid_stem.move_to([pos[0], baseline_y + 0.1, 0])

            frame = VGroup(bulb_outer, tube_outer)
            return frame, bulb_inner, fluid_stem, baseline_y
            
        therm_pos = house_center + RIGHT * 3.2 + UP * 0.2
        therm_frame, therm_bulb, therm_fluid, therm_base_y = create_thermometer(therm_pos)
        
        therm_group = VGroup(therm_frame, therm_bulb, therm_fluid)
        
        temp_title = Text("Raumtemperatur", font_size=16, color=P_WHITE)
        temp_title.next_to(therm_frame, UP, buff=0.15)

        temp_text = Text("20°C", font_size=24, color=P_WHITE)
        temp_text.next_to(therm_frame, RIGHT, buff=0.2)
        therm_group.add(temp_text)
        
        temp_val = ValueTracker(20)
        def update_therm(group):
            val = temp_val.get_value()
            color = P_RED if val > 25 else (P_CYAN if val <= 21 else P_WHITE)
            new_text = Text(f"{int(val)}°C", font_size=24, color=color).next_to(therm_frame, RIGHT, buff=0.2)
            h = 0.2 + ((val - 20) / 15) * 0.5
            new_fluid = Rectangle(width=0.06, height=h, color=color, fill_opacity=1, stroke_width=0)
            new_fluid.move_to([therm_pos[0], therm_base_y + h/2, 0])
            new_bulb = Circle(radius=0.12, color=color, fill_opacity=1, stroke_width=0).move_to(therm_frame[0].get_center())
            group[1].become(new_bulb)
            group[2].become(new_fluid)
            group[3].become(new_text)

        therm_group.add_updater(update_therm)
        temp_tracker_group = VGroup(therm_group, temp_title)

        self.play(
            Transform(subtitle, new_subtitle),
            Transform(sun_group, sun_group_red),
            Transform(solar_label, solar_label_red),
            Transform(internal_dot, internal_dot_red),
            Transform(internal_waves, internal_waves_red),
            Transform(internal_label, internal_label_red),
            FadeIn(heat_block),
            FadeIn(temp_tracker_group),
            run_time=1.8
        )

        # Animate heat intensity & temp rise to 35°C
        self.play(
            heat_block.animate.set_fill(opacity=0.55),
            temp_val.animate.set_value(35),
            temp_text.animate.set_color(P_RED),
            run_time=3.5,
            rate_func=linear
        )
        self.wait(2.0)

        # ----------------------------------------------------
        # BEAT 4: The Cooling Goal & System Activation
        # ----------------------------------------------------
        title_beat4 = Text("Kühllast & Systemauslegung", font_size=32, color=P_CYAN)
        title_beat4.to_edge(UP, buff=0.4)

        subtitle_beat4 = Text("Wärme aktiv abführen (Mechanische Lüftung)", font_size=20, color=P_CYAN)
        subtitle_beat4.next_to(title_beat4, DOWN, buff=0.2)

        # Cooling exhaust cyan arrows (bursting from roof and side walls)
        exhaust_arrow_top1 = Arrow(start=roof_peak + LEFT * 0.4 + UP * 0.1, end=roof_peak + LEFT * 1.0 + UP * 1.2, color=P_CYAN, stroke_width=5)
        exhaust_arrow_top2 = Arrow(start=roof_peak + RIGHT * 0.4 + UP * 0.1, end=roof_peak + RIGHT * 1.0 + UP * 1.2, color=P_CYAN, stroke_width=5)
        exhaust_arrow_left = Arrow(start=top_left + DOWN * 0.8, end=top_left + LEFT * 1.2 + DOWN * 0.8, color=P_CYAN, stroke_width=5)
        exhaust_arrow_right = Arrow(start=top_right + DOWN * 0.8, end=top_right + RIGHT * 1.2 + DOWN * 0.8, color=P_CYAN, stroke_width=5)

        exhaust_arrows = VGroup(exhaust_arrow_top1, exhaust_arrow_top2, exhaust_arrow_left, exhaust_arrow_right)
        exhaust_label = Text("Mechanische Lüftung / Kühlung", font_size=15, color=P_CYAN)
        exhaust_label.next_to(roof_peak, LEFT, buff=1.3)

        self.play(
            Transform(title, title_beat4),
            Transform(subtitle, subtitle_beat4),
            FadeOut(sun_group),
            FadeOut(solar_label),
            FadeOut(internal_dot),
            FadeOut(internal_waves),
            FadeOut(internal_label),
            run_time=1.2
        )

        self.play(
            *[GrowArrow(arr) for arr in exhaust_arrows],
            FadeIn(exhaust_label),
            heat_block.animate.set_fill(opacity=0.0),
            temp_val.animate.set_value(21),
            temp_text.animate.set_color(P_CYAN),
            run_time=3.5
        )
        # Hold with VO: mechanical ventilation cool-down
        self.wait(1.8)
