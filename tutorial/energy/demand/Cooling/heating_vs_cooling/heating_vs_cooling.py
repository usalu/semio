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
        title = Text("Heizlast vs. Kühllast", font_size=32, color=P_WHITE)
        title.to_edge(UP, buff=0.4)

        subtitle = Text("Im Winter: Erwünschte kostenlose Wärme", font_size=20, color=P_GREEN)
        subtitle.next_to(title, DOWN, buff=0.2)

        # Line-Art House Setup
        house_center = LEFT * 0.8 + DOWN * 0.4
        w_width, w_height = 3.6, 2.4

        bottom_left = house_center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
        bottom_right = house_center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
        top_left = house_center + LEFT * (w_width / 2) + UP * (w_height / 2)
        top_right = house_center + RIGHT * (w_width / 2) + UP * (w_height / 2)
        roof_peak = house_center + UP * (w_height / 2 + 1.1)

        floor_line = Line(bottom_left + LEFT * 0.6, bottom_right + RIGHT * 0.6, color=P_TEAL, stroke_width=4)
        walls = VGroup(
            Line(bottom_left, top_left, color=P_WHITE, stroke_width=3),
            Line(bottom_right, top_right, color=P_WHITE, stroke_width=3),
        )
        roof = Polygon(top_left, roof_peak, top_right, color=P_WHITE, stroke_width=3)

        # Window & Door
        window = Square(side_length=0.7, color=P_CYAN, stroke_width=2)
        window.move_to(house_center + LEFT * 0.8 + UP * 0.3)
        window_cross = VGroup(
            Line(window.get_top(), window.get_bottom(), color=P_CYAN, stroke_width=1.5),
            Line(window.get_left(), window.get_right(), color=P_CYAN, stroke_width=1.5),
        )
        window_group = VGroup(window, window_cross)

        door = Rectangle(width=0.6, height=1.0, color=P_CYAN, stroke_width=2)
        door.move_to(house_center + RIGHT * 0.8 + DOWN * 0.7)
        door_knob = Dot(door.get_center() + LEFT * 0.18 + DOWN * 0.05, radius=0.04, color=P_CYAN)
        door_group = VGroup(door, door_knob)

        # Solar gains arrow
        solar_arrow = Arrow(
            start=house_center + LEFT * 3.8 + UP * 2.5,
            end=window.get_center() + LEFT * 0.15 + UP * 0.15,
            color=P_YELLOW,
            stroke_width=4,
            max_tip_length_to_length_ratio=0.2,
        )
        solar_label = Text("Solare Gewinne", font_size=16, color=P_YELLOW)
        solar_label.next_to(solar_arrow.get_start(), DOWN, buff=0.1).shift(RIGHT * 0.4)

        # Internal gains dot + wave rings
        internal_dot = Dot(house_center + RIGHT * 0.7 + UP * 0.3, radius=0.1, color=P_ORANGE)
        ring1 = Circle(radius=0.25, color=P_ORANGE, stroke_width=1.5).move_to(internal_dot.get_center())
        ring2 = Circle(radius=0.45, color=P_ORANGE, stroke_width=1.0, stroke_opacity=0.6).move_to(internal_dot.get_center())
        internal_waves = VGroup(ring1, ring2)
        internal_label = Text("Interne Gewinne", font_size=15, color=P_ORANGE)
        internal_label.next_to(internal_dot, UP, buff=0.2)

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
            GrowArrow(solar_arrow),
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

        # Scale solar arrow by 50% & shift color to Red
        solar_arrow_red = Arrow(
            start=house_center + LEFT * 4.2 + UP * 2.8,
            end=window.get_center() + LEFT * 0.1 + UP * 0.1,
            color=P_RED,
            stroke_width=6,
            max_tip_length_to_length_ratio=0.2,
        )
        solar_label_red = Text("Solare Gewinne (Exzessiv)", font_size=16, color=P_RED)
        solar_label_red.next_to(solar_arrow_red.get_start(), DOWN, buff=0.1).shift(RIGHT * 0.4)

        internal_dot_red = Dot(internal_dot.get_center(), radius=0.14, color=P_RED)
        ring1_red = Circle(radius=0.3, color=P_RED, stroke_width=2.0).move_to(internal_dot.get_center())
        ring2_red = Circle(radius=0.55, color=P_RED, stroke_width=1.5, stroke_opacity=0.7).move_to(internal_dot.get_center())
        internal_waves_red = VGroup(ring1_red, ring2_red)
        internal_label_red = Text("Interne Gewinne", font_size=15, color=P_RED)
        internal_label_red.next_to(internal_dot_red, UP, buff=0.2)

        # Trapped heat polygon (house interior shape)
        house_interior_points = [
            bottom_left + RIGHT * 0.05 + UP * 0.05,
            bottom_right + LEFT * 0.05 + UP * 0.05,
            top_right + LEFT * 0.05 + DOWN * 0.05,
            roof_peak + DOWN * 0.1,
            top_left + RIGHT * 0.05 + DOWN * 0.05,
        ]
        heat_block = Polygon(*house_interior_points, fill_color=P_RED, fill_opacity=0.45, stroke_width=0)

        # Temperature Tracker
        temp_val = ValueTracker(20)
        temp_title = Text("Raumtemperatur", font_size=16, color=P_WHITE)
        temp_box = Rectangle(width=2.4, height=1.4, color=P_WHITE, stroke_width=1.5, fill_color=P_DEEP_DARK, fill_opacity=0.8)
        temp_box.move_to(house_center + RIGHT * 3.2 + UP * 0.2)
        temp_title.next_to(temp_box, UP, buff=0.15)

        temp_text = Text("20°C", font_size=32, color=P_WHITE)
        temp_text.move_to(temp_box.get_center())
        temp_text.add_updater(
            lambda m: m.become(
                Text(f"{int(temp_val.get_value())}°C", font_size=32, color=P_RED if temp_val.get_value() > 25 else (P_CYAN if temp_val.get_value() == 21 else P_WHITE)).move_to(temp_box.get_center())
            )
        )
        temp_tracker_group = VGroup(temp_box, temp_title, temp_text)

        self.play(
            Transform(subtitle, new_subtitle),
            Transform(solar_arrow, solar_arrow_red),
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
            heat_block.animate.set_fill(opacity=0.75),
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
        exhaust_label.next_to(roof_peak, UP, buff=1.3)

        self.play(
            Transform(title, title_beat4),
            Transform(subtitle, subtitle_beat4),
            FadeOut(solar_arrow),
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
        self.wait(3.0)
