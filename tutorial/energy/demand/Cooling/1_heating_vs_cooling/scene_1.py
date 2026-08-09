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
    SUBTITLE_FONT_SIZE, BODY_FONT_SIZE, LABEL_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_GREEN,
    convection_stream, solar_wave_ray, watt_anchor,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Heizlast vs. Kühllast"


#region Shared visual motifs

def _build_cross_section_house(center=ORIGIN + DOWN * 0.45):
    """🏠 Two-storey line-art house — mid-screen anchor for winter/summer beats.

    Sits low enough that the roof peak (center + 2.3) stays under the content
    ceiling (~2.62) and never crowds the title / beat subtitle, while leaving
    room above for sun / solar labels.
    """
    w_width, w_height = 3.6, 2.4
    bottom_left = center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
    bottom_right = center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
    top_left = center + LEFT * (w_width / 2) + UP * (w_height / 2)
    top_right = center + RIGHT * (w_width / 2) + UP * (w_height / 2)
    roof_peak = center + UP * (w_height / 2 + 1.1)

    floor_line = Line(bottom_left + LEFT * 0.6, bottom_right + RIGHT * 0.6, color=P_TEAL, stroke_width=4)
    level_1 = Line(bottom_left + UP * (w_height / 2), bottom_right + UP * (w_height / 2), color=P_WHITE, stroke_width=2)

    w_h = 0.5
    wall_left_1 = Line(bottom_left, bottom_left + UP * (w_height / 4 - w_h / 2), color=P_WHITE, stroke_width=3)
    wall_left_2 = Line(
        bottom_left + UP * (w_height / 4 + w_h / 2),
        bottom_left + UP * (3 * w_height / 4 - w_h / 2),
        color=P_WHITE, stroke_width=3,
    )
    wall_left_3 = Line(bottom_left + UP * (3 * w_height / 4 + w_h / 2), top_left, color=P_WHITE, stroke_width=3)
    wall_right = Line(bottom_right, top_right, color=P_WHITE, stroke_width=3)
    walls = VGroup(wall_left_1, wall_left_2, wall_left_3, wall_right, level_1)
    roof = Polygon(top_left, roof_peak, top_right, color=P_WHITE, stroke_width=3)

    win1 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (w_height / 4) + RIGHT * 0.075)
    win2 = Rectangle(width=0.05, height=w_h, color=P_CYAN).move_to(bottom_left + UP * (3 * w_height / 4) + RIGHT * 0.075)
    window_group = VGroup(win1, win2)

    return {
        "center": center,
        "bottom_left": bottom_left, "bottom_right": bottom_right,
        "top_left": top_left, "top_right": top_right, "roof_peak": roof_peak,
        "floor": floor_line, "walls": walls, "roof": roof,
        "win1": win1, "win2": win2, "window_group": window_group,
        "group": VGroup(floor_line, walls, roof, window_group),
        "w_width": w_width, "w_height": w_height,
    }


def _build_sun(pos, color=P_YELLOW, glow_opacity=0.35, burst_width=2):
    """☀️ Compact sun with glow rings and radial burst lines."""
    sun_core = Dot(pos, radius=0.45, color=color)
    sun_glow = Dot(pos, radius=0.7, color=color, fill_opacity=glow_opacity)
    sun_ring1 = Circle(radius=0.85, color=color, stroke_width=2, stroke_opacity=0.6).move_to(pos)
    sun_ring2 = Circle(radius=1.1, color=color, stroke_width=1.2, stroke_opacity=0.3).move_to(pos)
    sun_burst = VGroup()
    for angle in np.linspace(0, TAU, 12, endpoint=False):
        s = pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
        e = pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
        sun_burst.add(Line(s, e, color=color, stroke_width=burst_width))
    return VGroup(sun_glow, sun_core, sun_ring1, sun_ring2, sun_burst)


def _create_wave(x_offset, color, start_pos):
    """〰️ Short rising heat plume above an internal source."""
    pts = [
        start_pos + np.array([x_offset + 0.04 * np.sin(y * 8), y, 0])
        for y in np.linspace(0.08, 0.4, 12)
    ]
    wave = VMobject(color=color, stroke_width=1.8, stroke_opacity=0.7)
    wave.set_points_smoothly(pts)
    return wave


def _build_internal_gains(house: dict, color_device=P_CYAN, color_person=P_ORANGE):
    """💡 Occupant, kitchen, desk laptop and ceiling lights inside the house."""
    bl = house["bottom_left"]
    w_height = house["w_height"]

    desk_surface = Line(
        bl + RIGHT * 1.8 + UP * (w_height / 2 + 0.3),
        bl + RIGHT * 2.4 + UP * (w_height / 2 + 0.3),
        color=P_WHITE, stroke_width=2,
    )
    desk_leg1 = Line(desk_surface.get_start() + RIGHT * 0.1, desk_surface.get_start() + RIGHT * 0.1 + DOWN * 0.3, color=P_WHITE, stroke_width=2)
    desk_leg2 = Line(desk_surface.get_end() + LEFT * 0.1, desk_surface.get_end() + LEFT * 0.1 + DOWN * 0.3, color=P_WHITE, stroke_width=2)
    laptop_base = Line(desk_surface.get_center() + LEFT * 0.1, desk_surface.get_center() + RIGHT * 0.1, color=color_device, stroke_width=2.5)
    laptop_screen = Line(laptop_base.get_right(), laptop_base.get_right() + LEFT * 0.05 + UP * 0.15, color=color_device, stroke_width=2.5)
    device = VGroup(desk_surface, desk_leg1, desk_leg2, laptop_base, laptop_screen)

    counter_top = Line(bl + RIGHT * 1.5 + UP * 0.3, bl + RIGHT * 2.3 + UP * 0.3, color=P_WHITE, stroke_width=2)
    counter_body = Rectangle(width=0.8, height=0.3, color=P_WHITE, stroke_width=1.5, fill_opacity=0.1).move_to(counter_top.get_center() + DOWN * 0.15)
    stove_pot = VGroup(
        Line(counter_top.get_center() + LEFT * 0.1 + UP * 0.02, counter_top.get_center() + RIGHT * 0.1 + UP * 0.02, color=P_WHITE, stroke_width=3),
        RoundedRectangle(
            corner_radius=0.03, width=0.16, height=0.12, color=color_device, stroke_width=2, fill_opacity=0.3,
        ).move_to(counter_top.get_center() + RIGHT * 0.1 + UP * 0.08),
    )
    kitchen = VGroup(counter_top, counter_body, stove_pot)

    p2_head = Circle(radius=0.08, color=color_person, fill_color=color_person, fill_opacity=0.3, stroke_width=2)
    p2_head.move_to(bl + RIGHT * 1.1 + UP * 0.62)
    p2_torso = Line(p2_head.get_bottom(), p2_head.get_bottom() + DOWN * 0.22, color=color_person, stroke_width=3)
    p2_legs = VGroup(
        Line(p2_torso.get_end(), p2_torso.get_end() + DOWN * 0.28 + LEFT * 0.05, color=color_person, stroke_width=3),
        Line(p2_torso.get_end(), p2_torso.get_end() + DOWN * 0.28 + RIGHT * 0.05, color=color_person, stroke_width=3),
    )
    p2_arms = Line(p2_torso.get_center() + UP * 0.03, p2_torso.get_center() + RIGHT * 0.2, color=color_person, stroke_width=2)
    person2 = VGroup(p2_head, p2_torso, p2_legs, p2_arms)

    light1_cord = Line(bl + UP * 2.4 + RIGHT * 1.2, bl + UP * 2.15 + RIGHT * 1.2, color=P_WHITE, stroke_width=1.5)
    light1_bulb = Dot(light1_cord.get_end(), radius=0.07, color=P_YELLOW)
    light1_glow = Circle(radius=0.18, color=P_YELLOW, stroke_width=0, fill_opacity=0.1).move_to(light1_bulb.get_center())
    light1 = VGroup(light1_cord, light1_bulb, light1_glow)

    light2_cord = Line(bl + UP * 1.2 + RIGHT * 2.7, bl + UP * 0.95 + RIGHT * 2.7, color=P_WHITE, stroke_width=1.5)
    light2_bulb = Dot(light2_cord.get_end(), radius=0.07, color=P_YELLOW)
    light2_glow = Circle(radius=0.18, color=P_YELLOW, stroke_width=0, fill_opacity=0.1).move_to(light2_bulb.get_center())
    light2 = VGroup(light2_cord, light2_bulb, light2_glow)

    sources = VGroup(device, person2, kitchen, light1, light2)
    waves = VGroup(
        _create_wave(-0.06, color_device, laptop_screen.get_center()),
        _create_wave(0.06, color_device, laptop_screen.get_center()),
        _create_wave(-0.06, P_YELLOW, light1_bulb.get_center()),
        _create_wave(0.06, P_YELLOW, light1_bulb.get_center()),
        _create_wave(-0.06, color_person, p2_head.get_top()),
        _create_wave(0.06, color_person, p2_head.get_top()),
        _create_wave(-0.06, color_device, stove_pot.get_top()),
        _create_wave(0.06, color_device, stove_pot.get_top()),
        _create_wave(-0.06, P_YELLOW, light2_bulb.get_center()),
        _create_wave(0.06, P_YELLOW, light2_bulb.get_center()),
    )
    return {
        "sources": sources, "waves": waves, "device": device, "person": person2,
        "laptop_screen": laptop_screen, "p2_head": p2_head,
    }


def _create_thermometer(pos):
    """🌡️ Thermometer frame + fluid stem for room-temperature tracking."""
    bulb_outer = Circle(radius=0.18, color=P_WHITE, stroke_width=2).move_to(pos + DOWN * 0.3)
    tube_outer = RoundedRectangle(corner_radius=0.06, width=0.14, height=1.0, color=P_WHITE, stroke_width=2)
    tube_outer.move_to(pos + UP * 0.25)
    bulb_inner = Circle(radius=0.12, color=P_CYAN, fill_opacity=1, stroke_width=0).move_to(bulb_outer.get_center())
    baseline_y = bulb_outer.get_center()[1] + 0.1
    fluid_stem = Rectangle(width=0.06, height=0.2, color=P_CYAN, fill_opacity=1, stroke_width=0)
    fluid_stem.move_to([pos[0], baseline_y + 0.1, 0])
    frame = VGroup(bulb_outer, tube_outer)
    return frame, bulb_inner, fluid_stem, baseline_y

#endregion


#region Beat 1 — Winter helpful gains

class Beat1_WinterGains(Scene):
    NARRATION = [
        ("intro",
         "In winter, free heat gains are welcome — they reduce the heating load we must supply.",
         "Im Winter sind kostenlose Wärmegewinne willkommen — sie senken die Heizlast."),
        ("solar",
         "Solar gains enter through the windows and warm the rooms for free.",
         "Solare Gewinne treten durch die Fenster ein und heizen die Räume kostenlos."),
        ("internal",
         "People, devices and lights add internal gains — roughly a laptop and a person as everyday watt anchors.",
         "Personen, Geräte und Licht erzeugen interne Gewinne — greifbar wie Laptop und Person."),
        ("outro",
         "Together, solar and internal gains help cover part of the winter heating demand.",
         "Zusammen decken solare und interne Gewinne einen Teil der Heizlast."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Im Winter: Erwünschte kostenlose Wärme", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        house = _build_cross_section_house()
        self.play(Create(house["group"]), run_time=2.0)

        sun_pos = house["center"] + LEFT * 3.6 + UP * 1.35
        sun_group = _build_sun(sun_pos, color=P_YELLOW)
        solar_label = Text("Solare Gewinne", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        solar_label.next_to(sun_group, DOWN, buff=0.55)
        solar_rays = VGroup(
            solar_wave_ray(sun_pos, house["win1"].get_center(), color=P_YELLOW, stroke_width=2.0, amp=0.06),
            solar_wave_ray(sun_pos, house["win2"].get_center(), color=P_YELLOW, stroke_width=2.0, amp=0.06),
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar"))
        self.play(
            FadeIn(sun_group), FadeIn(solar_label),
            LaggedStart(*[Create(r) for r in solar_rays], lag_ratio=0.15),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "solar", used=1.6 + 0.35)

        gains = _build_internal_gains(house)
        internal_label = Text("Interne Gewinne", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        internal_label.next_to(gains["device"], UP, buff=0.35)
        laptop_anchor = watt_anchor(60, compare="laptop", title="Gerät").scale(0.55)
        person_anchor = watt_anchor(100, compare="bulb", title="Person").scale(0.55)
        anchor_row = VGroup(person_anchor, laptop_anchor).arrange(DOWN, buff=0.4)
        anchor_row.next_to(house["walls"], RIGHT, buff=0.4)
        anchor_row.set_y(house["center"][1])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "internal"))
        self.play(
            FadeIn(gains["sources"]), Create(gains["waves"]), FadeIn(internal_label),
            run_time=1.4,
        )
        self.play(FadeIn(laptop_anchor), FadeIn(person_anchor), run_time=0.8)
        hold_for(self, self.NARRATION, "internal", used=2.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 — Summer overheating

class Beat2_SummerOverheat(Scene):
    NARRATION = [
        ("intro",
         "In summer the same gains become a problem — the building overheats like a greenhouse.",
         "Im Sommer werden dieselben Gewinne zum Problem — das Gebäude überhitzt wie ein Treibhaus."),
        ("excess",
         "Solar gains turn excessive under a harsh red sun, and internal gains keep stacking heat inside.",
         "Solare Gewinne werden übermäßig, und interne Gewinne stapeln weiter Wärme im Inneren."),
        ("trap",
         "Heat is trapped in the insulated envelope — watch the room temperature climb toward thirty-five degrees.",
         "Wärme staut sich in der gedämmten Hülle — die Raumtemperatur steigt Richtung fünfunddreißig Grad."),
        ("outro",
         "Without active cooling, comfort collapses on a hot summer afternoon.",
         "Ohne aktive Kühlung bricht der Komfort an einem heißen Sommernachmittag zusammen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Im Sommer: Überhitzung (Treibhauseffekt)", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        house = _build_cross_section_house()
        self.add(house["group"])

        sun_pos = house["center"] + LEFT * 3.6 + UP * 1.35
        sun_group = _build_sun(sun_pos, color=P_RED, glow_opacity=0.45, burst_width=3)
        solar_label = Text("Solare Gewinne (Exzessiv)", font_size=BODY_FONT_SIZE, color=P_RED)
        solar_label.next_to(sun_group, DOWN, buff=0.7)
        gains = _build_internal_gains(house, color_device=P_RED, color_person=P_RED)
        gains["sources"].set_color(P_RED)
        gains["waves"].set_color(P_RED)
        internal_label = Text("Interne Gewinne", font_size=LABEL_FONT_SIZE, color=P_RED)
        internal_label.next_to(gains["device"], UP, buff=0.45)

        heat_block = Polygon(
            house["bottom_left"] + RIGHT * 0.05 + UP * 0.05,
            house["bottom_right"] + LEFT * 0.05 + UP * 0.05,
            house["top_right"] + LEFT * 0.05 + DOWN * 0.05,
            house["roof_peak"] + DOWN * 0.1,
            house["top_left"] + RIGHT * 0.05 + DOWN * 0.05,
            fill_color=P_RED, fill_opacity=0.45, stroke_width=0,
        )

        therm_pos = house["center"] + RIGHT * 2.9 + UP * 0.1
        therm_frame, therm_bulb, therm_fluid, therm_base_y = _create_thermometer(therm_pos)
        temp_title = Text("Raumtemperatur", font_size=BODY_FONT_SIZE, color=P_WHITE)
        temp_title.next_to(therm_frame, UP, buff=0.15)
        temp_text = Text("20°C", font_size=SUBTITLE_FONT_SIZE, color=P_WHITE)
        temp_text.next_to(therm_frame, RIGHT, buff=0.2)
        therm_group = VGroup(therm_frame, therm_bulb, therm_fluid, temp_text)
        temp_val = ValueTracker(20)

        def update_therm(group):
            val = temp_val.get_value()
            color = P_RED if val > 25 else (P_CYAN if val <= 21 else P_WHITE)
            new_text = Text(f"{int(val)}°C", font_size=SUBTITLE_FONT_SIZE, color=color).next_to(therm_frame, RIGHT, buff=0.2)
            h = 0.2 + ((val - 20) / 15) * 0.5
            new_fluid = Rectangle(width=0.06, height=h, color=color, fill_opacity=1, stroke_width=0)
            new_fluid.move_to([therm_pos[0], therm_base_y + h / 2, 0])
            new_bulb = Circle(radius=0.12, color=color, fill_opacity=1, stroke_width=0).move_to(therm_frame[0].get_center())
            group[1].become(new_bulb)
            group[2].become(new_fluid)
            group[3].become(new_text)

        therm_group.add_updater(update_therm)
        temp_tracker_group = VGroup(therm_group, temp_title)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "excess"))
        self.play(
            FadeIn(sun_group), FadeIn(solar_label),
            FadeIn(gains["sources"]), Create(gains["waves"]), FadeIn(internal_label),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "excess", used=1.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "trap"))
        self.play(FadeIn(heat_block), FadeIn(temp_tracker_group), run_time=1.0)
        self.play(
            heat_block.animate.set_fill(opacity=0.55),
            temp_val.animate.set_value(35),
            run_time=3.2,
            rate_func=linear,
        )
        hold_for(self, self.NARRATION, "trap", used=4.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        therm_group.clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 3 — Cooling system activation

class Beat3_CoolingSystem(Scene):
    NARRATION = [
        ("intro",
         "Cooling design means actively removing heat so the room returns to comfort.",
         "Kühllast bedeutet: Wärme aktiv abführen, damit der Raum wieder komfortabel wird."),
        ("vent",
         "Mechanical ventilation and cooling exhaust the trapped heat through supply and extract streams.",
         "Mechanische Lüftung und Kühlung führen die gestaute Wärme über Zu- und Abluft ab."),
        ("cool_down",
         "As heat leaves, the interior cools and the thermometer settles near twenty-one degrees again.",
         "Wenn Wärme abfließt, kühlt der Innenraum — das Thermometer sinkt wieder Richtung einundzwanzig Grad."),
        ("outro",
         "That required heat removal rate is the cooling load we size the system for.",
         "Genau diese abzuführende Leistung ist die Kühllast, nach der wir das System auslegen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärme aktiv abführen (Mechanische Lüftung)", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        house = _build_cross_section_house()
        self.add(house["group"])

        heat_block = Polygon(
            house["bottom_left"] + RIGHT * 0.05 + UP * 0.05,
            house["bottom_right"] + LEFT * 0.05 + UP * 0.05,
            house["top_right"] + LEFT * 0.05 + DOWN * 0.05,
            house["roof_peak"] + DOWN * 0.1,
            house["top_left"] + RIGHT * 0.05 + DOWN * 0.05,
            fill_color=P_RED, fill_opacity=0.55, stroke_width=0,
        )
        self.add(heat_block)

        therm_pos = house["center"] + RIGHT * 2.9 + UP * 0.1
        therm_frame, therm_bulb, therm_fluid, therm_base_y = _create_thermometer(therm_pos)
        temp_title = Text("Raumtemperatur", font_size=BODY_FONT_SIZE, color=P_WHITE)
        temp_title.next_to(therm_frame, UP, buff=0.15)
        temp_text = Text("35°C", font_size=SUBTITLE_FONT_SIZE, color=P_RED)
        temp_text.next_to(therm_frame, RIGHT, buff=0.2)
        therm_group = VGroup(therm_frame, therm_bulb, therm_fluid, temp_text)
        # Start already hot
        therm_bulb.set_color(P_RED)
        therm_fluid.become(
            Rectangle(width=0.06, height=0.7, color=P_RED, fill_opacity=1, stroke_width=0).move_to(
                [therm_pos[0], therm_base_y + 0.35, 0]
            )
        )
        temp_val = ValueTracker(35)

        def update_therm(group):
            val = temp_val.get_value()
            color = P_RED if val > 25 else (P_CYAN if val <= 21 else P_WHITE)
            new_text = Text(f"{int(val)}°C", font_size=SUBTITLE_FONT_SIZE, color=color).next_to(therm_frame, RIGHT, buff=0.2)
            h = 0.2 + ((val - 20) / 15) * 0.5
            new_fluid = Rectangle(width=0.06, height=h, color=color, fill_opacity=1, stroke_width=0)
            new_fluid.move_to([therm_pos[0], therm_base_y + h / 2, 0])
            new_bulb = Circle(radius=0.12, color=color, fill_opacity=1, stroke_width=0).move_to(therm_frame[0].get_center())
            group[1].become(new_bulb)
            group[2].become(new_fluid)
            group[3].become(new_text)

        therm_group.add_updater(update_therm)
        self.add(therm_group, temp_title)

        exhaust_streams = VGroup(
            convection_stream(house["roof_peak"] + LEFT * 0.3, house["roof_peak"] + LEFT * 1.1 + UP * 1.1, color=P_CYAN, bend=0.25, n_ribbons=2),
            convection_stream(house["roof_peak"] + RIGHT * 0.3, house["roof_peak"] + RIGHT * 1.1 + UP * 1.1, color=P_CYAN, bend=0.25, n_ribbons=2),
            convection_stream(house["top_left"] + DOWN * 0.8, house["top_left"] + LEFT * 1.2 + DOWN * 0.8, color=P_CYAN, bend=0.15, n_ribbons=2),
            convection_stream(house["top_right"] + DOWN * 0.8, house["top_right"] + RIGHT * 1.2 + DOWN * 0.8, color=P_CYAN, bend=-0.15, n_ribbons=2),
        )
        exhaust_label = Text("Mechanische Lüftung / Kühlung", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        exhaust_label.next_to(house["roof_peak"], LEFT, buff=1.1)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "vent"))
        self.play(
            LaggedStart(*[Create(s) for s in exhaust_streams], lag_ratio=0.12),
            FadeIn(exhaust_label),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "vent", used=2.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cool_down"))
        self.play(
            heat_block.animate.set_fill(opacity=0.0),
            temp_val.animate.set_value(21),
            run_time=3.2,
        )
        hold_for(self, self.NARRATION, "cool_down", used=3.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        therm_group.clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion
