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
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE,
    convection_stream, radiation_waves, respiration_parts, symbol_token, watt_anchor,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Interne Wärmegewinne"


#region Shared helpers

def _rising_heat_waves(origin, n=3, color=P_ORANGE, height=1.35, x_spread=0.32, stroke_width=2.0):
    """〰️ Sine-wavy radiation plumes rising from a heat source."""
    ox, oy = float(origin[0]), float(origin[1])
    waves = VGroup()
    for i in range(n):
        x0 = ox + (i - (n - 1) / 2) * x_spread
        phase = i * 0.85

        def _path(t, x0=x0, oy=oy, phase=phase, height=height):
            return np.array([
                x0 + np.sin(t * 5.0 + phase) * 0.08,
                oy + t * height,
                0.0,
            ])

        waves.add(
            ParametricFunction(_path, t_range=[0, 1.0], color=color, stroke_width=stroke_width).set_opacity(0.8)
        )
    return waves


def _device_glow(center, color=P_ORANGE):
    """🌟 Soft layered glow under a heat-emitting device."""
    return VGroup(
        Circle(radius=1.05, color=color, stroke_width=0, fill_opacity=0.05).move_to(center),
        Circle(radius=0.65, color=color, stroke_width=0, fill_opacity=0.10).move_to(center),
        Circle(radius=0.32, color=color, stroke_width=0, fill_opacity=0.18).move_to(center),
    )


def _occupant_icon(color=P_CYAN, scale=1.0):
    """👤 Compact ISO-style occupant pictogram for hall density grids."""
    head = Circle(
        radius=0.07 * scale, color=color, fill_color=color, fill_opacity=0.25, stroke_width=1.5 * scale,
    )
    shoulders = Arc(radius=0.13 * scale, start_angle=0, angle=PI, color=color, stroke_width=1.5 * scale)
    shoulders.next_to(head, DOWN, buff=0.02 * scale)
    return VGroup(head, shoulders)

#endregion


#region Beat 1 — Office room / enclosed envelope

class Beat1_OfficeRoom(Scene):
    NARRATION = [
        ("intro",
         "Before the sun even touches our building, we already have a heat problem brewing inside.",
         "Noch bevor die Sonne das Gebäude berührt, brodelt innen bereits ein Wärmeproblem."),
        ("facade",
         "Start from the commercial facade, then zoom into the insulated office envelope.",
         "Beginnen wir bei der Gewerbefassade und zoomen in die gedämmte Bürohülle."),
        ("interior",
         "Inside sits a commercial workplace — desks, screens, chairs — sealed by insulation.",
         "Drinnen liegt ein gewerblicher Arbeitsplatz — Tische, Bildschirme, Stühle — dicht gedämmt."),
        ("outro",
         "In offices and lecture halls, internal heat is often the primary cooling load.",
         "In Büros und Hörsälen ist die interne Wärme oft die dominante Kühllast."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Die umschlossene Umgebung", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        ground = Line(LEFT * 4, RIGHT * 4, color=P_CYAN, stroke_width=3).move_to(DOWN * 0.85)
        roof = Polygon([-2.0, 1.15, 0], [2.0, 1.15, 0], [0, 2.15, 0], color=P_WHITE, stroke_width=3)
        body = Polygon([-1.8, -0.85, 0], [1.8, -0.85, 0], [1.8, 1.15, 0], [-1.8, 1.15, 0], color=P_WHITE, stroke_width=3)
        window1 = Square(side_length=0.5, color=P_CYAN, stroke_width=2).move_to(LEFT * 0.8 + UP * 0.35)
        window2 = Square(side_length=0.5, color=P_CYAN, stroke_width=2).move_to(RIGHT * 0.8 + UP * 0.35)
        door = Rectangle(width=0.6, height=0.9, color=P_WHITE, stroke_width=2).move_to(DOWN * 0.4)
        facade = VGroup(ground, roof, body, window1, window2, door)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "facade"))
        self.play(Create(facade), run_time=1.8)
        # 1.6x / slightly low keeps the roof peak clear of the beat subtitle and
        # the base clear of the caption bar; 1.85x filled the band exactly and
        # drove the apex through the subtitle text.
        self.play(facade.animate.scale(1.6).move_to(DOWN * 0.1), run_time=2.0)
        hold_for(self, self.NARRATION, "facade", used=3.8 + 0.35)

        floor = Line(LEFT * 3.6 + DOWN * 1.5, RIGHT * 3.6 + DOWN * 1.5, color=P_CYAN, stroke_width=4)
        ceiling = Line(LEFT * 3.6 + UP * 1.5, RIGHT * 3.6 + UP * 1.5, color=P_WHITE, stroke_width=3)
        left_wall = Line(LEFT * 3.6 + DOWN * 1.5, LEFT * 3.6 + UP * 1.5, color=P_WHITE, stroke_width=3)
        right_wall = Line(RIGHT * 3.6 + DOWN * 1.5, RIGHT * 3.6 + UP * 1.5, color=P_WHITE, stroke_width=3)
        room_box = VGroup(floor, ceiling, left_wall, right_wall)
        insulation_box = RoundedRectangle(
            width=7.5, height=3.4, corner_radius=0.15, color=P_ORANGE, stroke_width=2.5,
        ).move_to(ORIGIN + DOWN * 0.0)

        desk_top = Line(LEFT * 1.2 + DOWN * 0.55, RIGHT * 1.2 + DOWN * 0.55, color=P_CYAN, stroke_width=3)
        leg_left = Line(LEFT * 1.1 + DOWN * 0.55, LEFT * 1.1 + DOWN * 1.5, color=P_CYAN, stroke_width=3)
        leg_right = Line(RIGHT * 1.1 + DOWN * 0.55, RIGHT * 1.1 + DOWN * 1.5, color=P_CYAN, stroke_width=3)
        monitor_screen = Rectangle(width=0.9, height=0.55, color=P_WHITE, stroke_width=2.5).move_to(LEFT * 0.3 + DOWN * 0.2)
        monitor_base = Line(LEFT * 0.4 + DOWN * 0.52, LEFT * 0.2 + DOWN * 0.52, color=P_WHITE, stroke_width=2.5)
        monitor_stand = Line(LEFT * 0.3 + DOWN * 0.45, LEFT * 0.3 + DOWN * 0.52, color=P_WHITE, stroke_width=2.5)
        chair_seat = Line(RIGHT * 0.3 + DOWN * 1.1, RIGHT * 1.0 + DOWN * 1.1, color=P_WHITE, stroke_width=2.5)
        chair_back = Line(RIGHT * 1.0 + DOWN * 0.55, RIGHT * 1.0 + DOWN * 1.1, color=P_WHITE, stroke_width=2.5)
        chair_stem = Line(RIGHT * 0.65 + DOWN * 1.1, RIGHT * 0.65 + DOWN * 1.5, color=P_WHITE, stroke_width=2.5)
        chair_base = Line(RIGHT * 0.4 + DOWN * 1.5, RIGHT * 0.9 + DOWN * 1.5, color=P_WHITE, stroke_width=2.5)
        furniture = VGroup(
            desk_top, leg_left, leg_right,
            monitor_screen, monitor_base, monitor_stand,
            chair_seat, chair_back, chair_stem, chair_base,
        )
        interior_label = Text("Gewerblicher Arbeitsbereich", font_size=BODY_FONT_SIZE, color=P_CYAN)
        interior_label.move_to(UP * 0.55)
        office_env_label = Text("Bürohülle", font_size=SUBTITLE_FONT_SIZE, color=P_WHITE)
        office_env_label.move_to(UP * 1.15)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "interior"))
        self.play(ReplacementTransform(facade, room_box), Create(insulation_box), run_time=1.8)
        self.play(Create(furniture), FadeIn(interior_label), FadeIn(office_env_label), run_time=1.6)
        hold_for(self, self.NARRATION, "interior", used=3.4 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 — Human factor

class Beat2_HumanFactor(Scene):
    NARRATION = [
        ("intro",
         "Think of a human body as a biological heater sitting at a desk.",
         "Stellen Sie sich den menschlichen Körper als biologischen Heizkörper am Schreibtisch vor."),
        ("modes",
         "Heat leaves as radiation, convection, and respiration — both sensible and latent.",
         "Wärme geht als Strahlung, Konvektion und Atmung ab — fühlbar und latent."),
        ("anchor",
         "A single seated adult emits about one hundred watts — like a bright light bulb.",
         "Eine sitzende Person gibt etwa einhundert Watt ab — wie eine helle Glühbirne."),
        ("formula",
         "We calculate Q-dot Personen as n times the specific emission q-dot p, in watts.",
         "Wir berechnen Q-Punkt Personen als n mal die spezifische Abgabe q-Punkt p, in Watt."),
        ("n",
         "n is the number of occupants in the room.",
         "n ist die Anzahl der Personen im Raum."),
        ("qp",
         "q-dot p is roughly one hundred watts per person at desk work.",
         "q-Punkt p liegt bei etwa einhundert Watt pro Person bei Büroarbeit."),
        ("scale",
         "Pack fifty students into a lecture hall and you run a five-thousand-watt heater continuously.",
         "Fünfzig Studierende im Hörsaal bedeuten dauerhaft fünftausend Watt — wie fünf Toaster."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Menschliche Stoffwechselwärme", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        ceiling_line = Line(LEFT * 5.0 + UP * 2.0, RIGHT * 5.0 + UP * 2.0, color="#2C3545", stroke_width=2)
        floor_line = Line(LEFT * 5.0 + DOWN * 1.7, RIGHT * 5.0 + DOWN * 1.7, color="#2C3545", stroke_width=2)
        torso_center = LEFT * 1.2 + DOWN * 0.15

        chair_back = Line(torso_center + DOWN * 0.4 + LEFT * 0.35, torso_center + UP * 0.6 + LEFT * 0.35, color=P_CYAN, stroke_width=3)
        chair_seat = Line(torso_center + DOWN * 0.45 + LEFT * 0.4, torso_center + DOWN * 0.45 + RIGHT * 0.4, color=P_CYAN, stroke_width=3)
        chair_stem = Line(torso_center + DOWN * 0.45, torso_center + DOWN * 1.15, color=P_CYAN, stroke_width=2.5)
        chair_base = Line(torso_center + DOWN * 1.15 + LEFT * 0.4, torso_center + DOWN * 1.15 + RIGHT * 0.4, color=P_CYAN, stroke_width=2.5)
        chair = VGroup(chair_back, chair_seat, chair_stem, chair_base)

        head = Circle(radius=0.28, stroke_color=P_WHITE, stroke_width=3).move_to(torso_center + UP * 0.9)
        torso = RoundedRectangle(width=0.55, height=0.9, corner_radius=0.15, stroke_color=P_WHITE, stroke_width=3).move_to(torso_center)
        arm = Line(torso_center + UP * 0.2 + RIGHT * 0.1, torso_center + DOWN * 0.2 + RIGHT * 0.6, stroke_color=P_WHITE, stroke_width=3)
        person = VGroup(head, torso, arm)

        desk_top = Line(torso_center + RIGHT * 0.3 + DOWN * 0.2, torso_center + RIGHT * 1.9 + DOWN * 0.2, color=P_CYAN, stroke_width=3)
        desk_leg = Line(torso_center + RIGHT * 1.8 + DOWN * 0.2, torso_center + RIGHT * 1.8 + DOWN * 1.15, color=P_CYAN, stroke_width=2.5)
        laptop_base = Line(torso_center + RIGHT * 0.8 + DOWN * 0.2, torso_center + RIGHT * 1.4 + DOWN * 0.2, color=P_WHITE, stroke_width=2)
        laptop_screen = Line(torso_center + RIGHT * 1.4 + DOWN * 0.2, torso_center + RIGHT * 1.5 + UP * 0.3, color=P_WHITE, stroke_width=2)
        desk = VGroup(desk_top, desk_leg, laptop_base, laptop_screen)
        office_setup = VGroup(chair, person, desk)

        self.play(Create(ceiling_line), Create(floor_line), Create(office_setup), run_time=1.8)

        rad_origin = torso.get_center() + UP * 0.15
        rad_waves = radiation_waves(rad_origin, n=4, color=P_ORANGE, height=1.0, x_spread=0.42)
        lbl_rad = Text("Strahlung", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(torso_center + UP * 1.55 + LEFT * 1.55)
        conv_stream = convection_stream(
            torso.get_center() + RIGHT * 0.35,
            torso.get_center() + UP * 1.35 + RIGHT * 0.9,
            color=P_CYAN, bend=0.45, n_ribbons=3, spread=0.22,
        )
        lbl_conv = Text("Konvektion", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to(torso_center + UP * 1.85 + RIGHT * 0.55)
        mouth = head.get_center() + RIGHT * 0.22 + DOWN * 0.05
        breath = respiration_parts(mouth, scale=1.0)
        lbl_resp = Text("Atmung", font_size=BODY_FONT_SIZE, color=P_RED).move_to(torso_center + UP * 1.25 + RIGHT * 2.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "modes"))
        self.play(
            LaggedStart(*[Create(w) for w in rad_waves], lag_ratio=0.12),
            FadeIn(lbl_rad),
            run_time=1.6,
        )
        self.play(
            LaggedStart(*[Create(r) for r in conv_stream], lag_ratio=0.15),
            FadeIn(lbl_conv),
            run_time=1.4,
        )
        self.play(
            LaggedStart(*[Create(w) for w in breath["sensible"]], lag_ratio=0.1),
            FadeIn(breath["sensible_label"]), FadeIn(lbl_resp),
            run_time=1.2,
        )
        self.play(
            LaggedStart(*[FadeIn(d, scale=0.6) for d in breath["latent"]], lag_ratio=0.12),
            FadeIn(breath["latent_label"]),
            run_time=1.0,
        )
        hold_for(self, self.NARRATION, "modes", used=5.2 + 0.35)

        badge = watt_anchor(100, compare="bulb", title="Körperwärme")
        badge.move_to(RIGHT * 3.2 + DOWN * 0.05)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "anchor"))
        self.play(FadeIn(badge, shift=LEFT * 0.25), run_time=1.0)
        hold_for(self, self.NARRATION, "anchor", used=1.0 + 0.35)

        # Clear desk scene for formula + hall scale
        desk_group = VGroup(
            ceiling_line, floor_line, office_setup, rad_waves, lbl_rad, conv_stream, lbl_conv,
            breath["sensible"], breath["sensible_label"], breath["latent"], breath["latent_label"], lbl_resp, badge,
        )
        self.play(desk_group.animate.scale(0.72).shift(UP * 0.85 + LEFT * 0.4), run_time=0.9)

        row, items = equation_row([
            ("qp_tot", "Q̇_Pers", P_ORANGE), (None, "=", P_WHITE),
            ("n", "n", P_CYAN), (None, "·", P_WHITE),
            ("qp", "q̇_p", P_ORANGE), (None, "  [W]", P_TEAL),
        ])
        row, box = formula_panel(row)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(Create(row), Create(box), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

        for key, color in (("n", P_CYAN), ("qp", P_ORANGE)):
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.45 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(desk_group), FadeOut(row), FadeOut(box), run_time=0.7)

        single = _occupant_icon(color=P_CYAN, scale=1.8).move_to(DOWN * 0.05)
        single_label = Text("Einzelperson (100 W)", font_size=BODY_FONT_SIZE, color=P_CYAN)
        single_label.next_to(single, DOWN, buff=0.28)
        self.play(FadeIn(single), FadeIn(single_label), run_time=0.9)

        hall_outline = Rectangle(
            width=9.6, height=3.0, stroke_color="#2C3545", stroke_width=2,
            fill_color=P_DEEP_DARK, fill_opacity=0.85,
        ).move_to(UP * 0.15)
        tier_y = np.linspace(-1.15, 0.95, 5)
        tier_lines = VGroup(*[
            Line(start=[-4.5, y, 0], end=[4.5, y, 0], stroke_color="#2C3545", stroke_width=1, stroke_opacity=0.6)
            for y in tier_y
        ])
        grid_icons = VGroup()
        for y_pos in tier_y:
            for c in range(10):
                x_pos = -4.05 + c * 0.9
                grid_icons.add(_occupant_icon(color=P_CYAN, scale=0.8).move_to([x_pos, y_pos + 0.06, 0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "scale"))
        self.play(
            ReplacementTransform(single, grid_icons),
            FadeOut(single_label),
            Create(hall_outline), Create(tier_lines),
            run_time=2.6,
        )
        hall_anchor = watt_anchor(5000, compare="toaster", title="Gesamtwärme")
        hall_anchor.scale(0.7).next_to(hall_outline, DOWN, buff=0.55)
        # Keep clear of caption bar
        hall_anchor.shift(UP * 0.15)
        thermal_waves = VGroup(*[
            Circle(radius=0.18, stroke_color=P_ORANGE, stroke_width=1.1, stroke_opacity=0.75).move_to(icon.get_center())
            for icon in grid_icons
        ])
        self.play(
            *[icon.animate.set_color(P_ORANGE) for icon in grid_icons],
            LaggedStart(*[GrowFromCenter(w) for w in thermal_waves], lag_ratio=0.01),
            FadeIn(hall_anchor),
            run_time=2.4,
        )
        hold_for(self, self.NARRATION, "scale", used=5.0 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 3 — Devices and lighting

class Beat3_DevicesLighting(Scene):
    NARRATION = [
        ("intro",
         "Every laptop, server rack and light fixture converts electrical power straight into heat.",
         "Jedes Notebook, jedes Serverrack und jede Leuchte wandelt elektrische Leistung direkt in Wärme um."),
        ("devices",
         "Plug loads follow Q-dot Geräte equals the sum of P el times the usage factor f N, in watts.",
         "Steckerlasten: Q-Punkt Geräte gleich Summe aus P el mal Nutzungsfaktor f N, in Watt."),
        ("pel",
         "P el is the electrical power drawn by each device.",
         "P el ist die elektrische Leistung jedes Geräts."),
        ("fn",
         "f N accounts for the fact that not every device runs at once.",
         "f N berücksichtigt, dass nicht jedes Gerät gleichzeitig läuft."),
        ("lights",
         "Ceiling lighting adds Q-dot Licht equals the sum of P Licht times the coincidence factor f g, in watts.",
         "Deckenlicht: Q-Punkt Licht gleich Summe aus P Licht mal Gleichzeitigkeit f g, in Watt."),
        ("pl",
         "P Licht is the installed electrical power of the luminaires.",
         "P Licht ist die installierte elektrische Leistung der Leuchten."),
        ("fg",
         "f g is the coincidence factor — the share of luminaires that actually run at the same time.",
         "f g ist der Gleichzeitigkeitsfaktor — der Anteil der gleichzeitig betriebenen Leuchten."),
        ("outro",
         "Together, devices and lights bake the room from the inside out.",
         "Zusammen heizen Geräte und Licht den Raum von innen heraus auf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Geräte, Steckerlasten und Beleuchtung", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        floor = Line(LEFT * 5.5 + DOWN * 1.85, RIGHT * 5.5 + DOWN * 1.85, color="#2C3545", stroke_width=2)
        desk_surface = Line(LEFT * 4.2 + DOWN * 0.55, LEFT * 0.4 + DOWN * 0.55, color=P_CYAN, stroke_width=3)
        leg_left = Line(LEFT * 4.0 + DOWN * 0.55, LEFT * 4.0 + DOWN * 1.85, color=P_CYAN, stroke_width=2)
        leg_right = Line(LEFT * 0.7 + DOWN * 0.55, LEFT * 0.7 + DOWN * 1.85, color=P_CYAN, stroke_width=2)
        desk = VGroup(desk_surface, leg_left, leg_right)

        laptop_base = Rectangle(width=1.3, height=0.12, color=P_CYAN, fill_color="#1F2833", fill_opacity=0.9, stroke_width=2)
        laptop_screen = Rectangle(width=1.2, height=0.8, color=P_CYAN, fill_color=P_DEEP_DARK, fill_opacity=0.9, stroke_width=2)
        laptop_screen.next_to(laptop_base, UP, buff=0)
        laptop = VGroup(laptop_base, laptop_screen)
        target_laptop = LEFT * 2.3 + DOWN * 0.1
        laptop.move_to(LEFT * 2.3 + UP * 2.2)

        rack_frame = Rectangle(width=1.6, height=2.6, color=P_CYAN, fill_color="#1F2833", fill_opacity=0.9, stroke_width=2)
        slots = VGroup()
        for i in range(5):
            slot_y = 0.85 - i * 0.42
            slot_rect = Rectangle(width=1.25, height=0.3, color="#2C3545", fill_color=P_DEEP_DARK, fill_opacity=0.8, stroke_width=1.5)
            slot_rect.move_to(UP * slot_y)
            slots.add(VGroup(slot_rect, Dot(slot_rect.get_left() + RIGHT * 0.18, radius=0.035, color=P_CYAN)))
        server_rack = VGroup(rack_frame, slots)
        target_rack = RIGHT * 2.8 + DOWN * 0.45
        server_rack.move_to(RIGHT * 2.8 + UP * 2.8)

        self.play(Create(floor), Create(desk), run_time=1.0)
        self.play(laptop.animate.move_to(target_laptop), rate_func=rate_functions.ease_out_bounce, run_time=1.1)
        self.play(server_rack.animate.move_to(target_rack), rate_func=rate_functions.ease_out_bounce, run_time=1.1)

        laptop_origin = target_laptop + UP * 0.15
        laptop_glow = _device_glow(laptop_origin)
        laptop_waves = _rising_heat_waves(laptop_origin + DOWN * 0.05, n=4, color=P_ORANGE, height=1.15, x_spread=0.26)
        server_origin = target_rack + UP * 0.35
        server_glow = _device_glow(server_origin)
        server_waves = _rising_heat_waves(server_origin + UP * 0.55, n=3, color=P_ORANGE, height=0.95, x_spread=0.32)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "devices"))
        self.play(
            FadeIn(laptop_glow), laptop.animate.set_color(P_ORANGE),
            FadeIn(server_glow), rack_frame.animate.set_color(P_ORANGE),
            run_time=0.8,
        )
        self.play(
            LaggedStart(*[Create(w) for w in laptop_waves], *[Create(w) for w in server_waves], lag_ratio=0.06),
            run_time=1.6,
        )

        devices_group = VGroup(floor, desk, laptop, server_rack, laptop_glow, laptop_waves, server_glow, server_waves)
        self.play(devices_group.animate.scale(0.78).shift(UP * 0.95), run_time=0.8)

        row_d, items_d = equation_row([
            ("qg", "Q̇_Geräte", P_CYAN), (None, "=", P_WHITE),
            (None, "Σ", P_WHITE),
            ("pel", "P_el", P_CYAN), (None, "·", P_WHITE),
            ("fn", "f_N", P_YELLOW), (None, "  [W]", P_TEAL),
        ])
        row_d, box_d = formula_panel(row_d)
        self.play(Create(row_d), Create(box_d), run_time=1.1)
        hold_for(self, self.NARRATION, "devices", used=4.3 + 0.35)

        for key, color in (("pel", P_CYAN), ("fn", P_YELLOW)):
            ring = highlight_param(items_d, key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.45 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(devices_group), FadeOut(row_d), FadeOut(box_d), run_time=0.7)

        # Lighting section — keep clear of formula/caption zones
        room_w, room_h = 7.6, 3.4
        bl = LEFT * (room_w / 2) + DOWN * 1.15
        br = RIGHT * (room_w / 2) + DOWN * 1.15
        tl = LEFT * (room_w / 2) + UP * 1.55
        tr = RIGHT * (room_w / 2) + UP * 1.55
        room = VGroup(
            Line(bl, br, color=P_TEAL, stroke_width=5),
            Line(bl, tl, color=P_WHITE, stroke_width=3),
            Line(br, tr, color=P_WHITE, stroke_width=3),
            Line(tl, tr, color=P_WHITE, stroke_width=3),
        )
        drop_y = 0.75
        drop_ceiling = DashedLine(
            LEFT * (room_w / 2 - 0.08) + UP * drop_y,
            RIGHT * (room_w / 2 - 0.08) + UP * drop_y,
            color=P_CYAN, stroke_width=2, dash_length=0.18,
        )
        fixture_l = RoundedRectangle(
            width=1.1, height=0.22, corner_radius=0.05,
            color=P_YELLOW, stroke_width=2, fill_color=P_YELLOW, fill_opacity=0.15,
        ).move_to(LEFT * 1.8 + UP * drop_y)
        fixture_r = fixture_l.copy().move_to(RIGHT * 1.8 + UP * drop_y)
        fixtures = VGroup(fixture_l, fixture_r)
        heat_up = VGroup(
            *_rising_heat_waves(fixture_l.get_center() + UP * 0.05, n=2, color=P_RED, height=0.45, x_spread=0.18, stroke_width=1.8),
            *_rising_heat_waves(fixture_r.get_center() + UP * 0.05, n=2, color=P_RED, height=0.45, x_spread=0.18, stroke_width=1.8),
        )
        heat_dn = VGroup(
            *_rising_heat_waves(fixture_l.get_center() + DOWN * 0.9, n=2, color=P_ORANGE, height=-0.7, x_spread=0.2, stroke_width=1.8),
            *_rising_heat_waves(fixture_r.get_center() + DOWN * 0.9, n=2, color=P_ORANGE, height=-0.7, x_spread=0.2, stroke_width=1.8),
        )
        # Fix downward waves: ParametricFunction with negative height still goes up via t*height — rebuild as downward
        heat_dn = VGroup()
        for origin in (fixture_l.get_center(), fixture_r.get_center()):
            ox, oy = float(origin[0]), float(origin[1]) - 0.12
            for i, dx in enumerate([-0.12, 0.12]):
                phase = i * 0.7

                def _down(t, x0=ox + dx, oy=oy, phase=phase):
                    return np.array([x0 + np.sin(t * 5 + phase) * 0.05, oy - t * 0.85, 0.0])

                heat_dn.add(ParametricFunction(_down, t_range=[0, 1], color=P_ORANGE, stroke_width=1.8).set_opacity(0.8))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lights"))
        self.play(Create(room), Create(drop_ceiling), FadeIn(fixtures), run_time=1.4)
        self.play(
            fixture_l.animate.set_fill(P_YELLOW, opacity=0.55),
            fixture_r.animate.set_fill(P_YELLOW, opacity=0.55),
            LaggedStart(*[Create(w) for w in heat_up], *[Create(w) for w in heat_dn], lag_ratio=0.05),
            run_time=1.8,
        )

        lighting_group = VGroup(room, drop_ceiling, fixtures, heat_up, heat_dn)
        self.play(lighting_group.animate.scale(0.82).shift(UP * 0.55), run_time=0.7)

        row_l, items_l = equation_row([
            ("ql", "Q̇_Licht", P_YELLOW), (None, "=", P_WHITE),
            (None, "Σ", P_WHITE),
            ("pl", "P_Licht", P_CYAN), (None, "·", P_WHITE),
            ("fg", "f_g", P_YELLOW), (None, "  [W]", P_TEAL),
        ])
        row_l, box_l = formula_panel(row_l)
        self.play(Create(row_l), Create(box_l), run_time=1.1)
        hold_for(self, self.NARRATION, "lights", used=5.0 + 1.1 + 0.35)

        for key, color in (("pl", P_CYAN), ("fg", P_YELLOW)):
            ring = highlight_param(items_l, key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.45 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 4 — Cumulative internal load

class Beat4_CumulativeLoad(Scene):
    NARRATION = [
        ("intro",
         "In building physics we sum every internal source into one cooling load, Q-dot internal.",
         "In der Bauphysik summieren wir alle internen Quellen zur Kühllast Q-Punkt intern."),
        ("sources",
         "People, plug loads and lighting are the three terms on the right-hand side.",
         "Personen, Steckerlasten und Beleuchtung sind die drei Terme auf der rechten Seite."),
        ("formula",
         "Q-dot i equals Q-dot Personen plus Q-dot Geräte plus Q-dot Licht, all in watts.",
         "Q-Punkt i gleich Q-Punkt Personen plus Q-Punkt Geräte plus Q-Punkt Licht, alles in Watt."),
        ("qi",
         "Q-dot i is the total internal cooling load the system must remove.",
         "Q-Punkt i ist die gesamte interne Kühllast, die das System abführen muss."),
        ("outro",
         "On a hot summer day this internal heat trap forces cooling overtime — even with blinds drawn.",
         "An einem heißen Sommertag zwingt diese interne Wärmefalle die Kühlung zu Überstunden — selbst bei geschlossenen Jalousien."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Summe der internen Gewinne", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        def mini_person(center=ORIGIN):
            head = Circle(radius=0.13, color=P_ORANGE, stroke_width=2.3, fill_color=P_ORANGE, fill_opacity=0.18).move_to(center + UP * 0.26)
            shoulders = Arc(radius=0.24, start_angle=0, angle=PI, color=P_ORANGE, stroke_width=2.5)
            shoulders.next_to(head, DOWN, buff=0.03)
            return VGroup(head, shoulders)

        def mini_laptop(center=ORIGIN):
            base = Rectangle(width=0.62, height=0.07, color=P_CYAN, stroke_width=2).move_to(center + DOWN * 0.04)
            screen = Rectangle(width=0.48, height=0.3, color=P_CYAN, stroke_width=2).move_to(center + UP * 0.18)
            return VGroup(base, screen)

        def mini_lamp(center=ORIGIN):
            cord = Line(center + UP * 0.38, center + UP * 0.12, color=P_WHITE, stroke_width=1.5)
            bulb = Circle(radius=0.14, color=P_YELLOW, stroke_width=2, fill_opacity=0.35).move_to(center)
            return VGroup(cord, bulb)

        def source_card(icon, title_de, term, color, center):
            frame = RoundedRectangle(
                width=2.05, height=1.7, corner_radius=0.12,
                color=color, stroke_width=2.2, fill_color="#12151C", fill_opacity=0.92,
            ).move_to(center)
            label = Text(title_de, font_size=BODY_FONT_SIZE, color=color)
            label.next_to(frame.get_top(), DOWN, buff=0.1)
            icon.move_to(center + DOWN * 0.02)
            term_t = Text(term, font_size=BODY_FONT_SIZE, color=P_WHITE)
            term_t.next_to(frame.get_bottom(), UP, buff=0.12)
            return VGroup(frame, label, icon, term_t)

        c_p = LEFT * 3.8 + UP * 0.85
        c_e = ORIGIN + UP * 0.85
        c_l = RIGHT * 3.8 + UP * 0.85
        card_p = source_card(mini_person(), "Personen", "Q̇_Pers", P_ORANGE, c_p)
        card_e = source_card(mini_laptop(), "Geräte", "Q̇_Geräte", P_CYAN, c_e)
        card_l = source_card(mini_lamp(), "Beleuchtung", "Q̇_Licht", P_YELLOW, c_l)
        plus_1 = Text("+", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to((c_p + c_e) / 2)
        plus_2 = Text("+", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to((c_e + c_l) / 2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sources"))
        self.play(
            LaggedStart(FadeIn(card_p), FadeIn(card_e), FadeIn(card_l), lag_ratio=0.2),
            run_time=1.3,
        )
        self.play(FadeIn(plus_1), FadeIn(plus_2), run_time=0.5)
        hold_for(self, self.NARRATION, "sources", used=1.8 + 0.35)

        self.play(
            VGroup(card_p, card_e, card_l, plus_1, plus_2).animate.shift(UP * 0.35).scale(0.92),
            run_time=0.7,
        )

        row, items = equation_row([
            ("qi", "Q̇_i", P_CYAN), (None, "=", P_WHITE),
            ("pers", "Q̇_Pers", P_ORANGE), (None, "+", P_WHITE),
            ("ger", "Q̇_Geräte", P_CYAN), (None, "+", P_WHITE),
            ("licht", "Q̇_Licht", P_YELLOW), (None, "  [W]", P_TEAL),
        ])
        row, box = formula_panel(row)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(Create(row), Create(box), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

        for key, color in (("pers", P_ORANGE), ("ger", P_CYAN), ("licht", P_YELLOW), ("qi", P_CYAN)):
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), run_time=0.4)
            if key == "qi":
                caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "qi"))
                hold_for(self, self.NARRATION, "qi", used=0.4 + 0.35)
            else:
                self.wait(0.35)
            self.play(FadeOut(ring), run_time=0.22)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Disabled beats 5–7
# Sensible/latent visuals moved to Cooling/3 Beat4_SensibleVsLatent.
# HeatTrap + HVAC deferred — keep Beat8 Mitigation active.
if False:
    #region Beat 5 — Sensible vs latent heat

    class Beat5_SensibleVsLatent(Scene):
        NARRATION = [
            ("intro",
             "Internal gains split into two loads — sensible heat that raises air temperature, and latent moisture that changes humidity.",
             "Interne Gewinne teilen sich in zwei Lasten: fühlbar (Temperatur) und latent (Feuchte)."),
            ("sensible",
             "A thermometer tracks the sensible side — the air itself gets warmer.",
             "Ein Thermometer zeigt die fühlbare Seite — die Luft wird wärmer."),
            ("latent",
             "A moisture gauge tracks the latent side — water vapor accumulates in the room.",
             "Ein Feuchtezeiger zeigt die latente Seite — Wasserdampf sammelt sich im Raum."),
            ("rise",
             "Both gauges climb together as people and processes keep adding heat and moisture.",
             "Beide Anzeigen steigen, während Personen und Prozesse Wärme und Feuchte zuführen."),
            ("formula",
             "Total heat load Q-dot ges equals sensible load plus latent load, in watts.",
             "Gesamtwärmelast Q-Punkt-ges gleich sensible plus latente Last, in Watt."),
            ("qsens",
             "The sensible term is the heat you feel directly as a rising air temperature.",
             "Der fühlbare Term ist die Wärme, die man direkt als steigende Lufttemperatur spürt."),
            ("qlat",
             "The latent term is the energy hidden in the moisture that people and processes release.",
             "Der latente Term ist die Energie, die in der abgegebenen Feuchte steckt."),
            ("qges",
             "Added together they give the total load the cooling system has to remove.",
             "Zusammen ergeben sie die Gesamtlast, die die Kühlung abführen muss."),
        ]

        def construct(self):
            apply_scene_style(self)

            title = scene_title(TITLE_DE)
            self.add(title)
            subtitle = beat_subtitle("Sensible versus latente Wärme", title)
            self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

            caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
            self.play(FadeIn(caption), run_time=0.3)

            # Mid-screen split: panel center ~ y=+0.25 (clear of title / formula / caption).
            mid_y = 0.25
            lx, rx = -3.2, 3.2

            divider = Line(UP * (mid_y + 1.15), DOWN * (1.15 - mid_y), color=P_TEAL, stroke_width=2)

            # Headers sit high enough that the rising mercury column and its °C
            # readout (which climb to y≈1.25 with the tracker) never reach them.
            left_header = Text("Sensible Last", font_size=SUBTITLE_FONT_SIZE, color=P_RED)
            left_header.move_to(np.array([lx, mid_y + 1.75, 0]))
            left_sub = Text("Temperaturanstieg", font_size=BODY_FONT_SIZE, color=P_WHITE)
            left_sub.next_to(left_header, DOWN, buff=0.1)

            right_header = Text("Latente Feuchtigkeit", font_size=SUBTITLE_FONT_SIZE, color=P_CYAN)
            right_header.move_to(np.array([rx, mid_y + 1.75, 0]))
            right_sub = Text("Phasen- / Feuchtigkeitswechsel", font_size=BODY_FONT_SIZE, color=P_WHITE)
            right_sub.next_to(right_header, DOWN, buff=0.1)

            self.play(Create(divider), run_time=1.0)
            self.play(
                FadeIn(left_header), FadeIn(left_sub),
                FadeIn(right_header), FadeIn(right_sub),
                run_time=1.2,
            )
            hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 1.2)

            bulb = Circle(radius=0.38, color=P_RED, fill_color=P_DEEP_DARK, fill_opacity=1.0, stroke_width=3)
            bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
            tube = RoundedRectangle(corner_radius=0.12, height=2.1, width=0.34, color=P_RED, stroke_width=3)
            tube.move_to(np.array([lx, mid_y + 0.15, 0]))
            mercury_bulb = Circle(radius=0.35, color=P_RED, fill_color=P_RED, fill_opacity=0.9, stroke_width=0)
            mercury_bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
            temp_ticks = VGroup(*[
                Line([lx - 0.28, y, 0], [lx - 0.12, y, 0], color=P_TEAL, stroke_width=2)
                for y in np.linspace(mid_y - 0.55, mid_y + 0.95, 6)
            ])
            sensible_tag = Text("Misst Lufttemperatur", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
            sensible_tag.move_to(np.array([lx, mid_y - 1.55, 0]))

            temp_tracker = ValueTracker(0.2)
            column = always_redraw(lambda: Rectangle(
                width=0.22,
                height=max(0.05, temp_tracker.get_value()),
                color=P_RED,
                fill_color=P_RED,
                fill_opacity=0.9,
                stroke_width=0,
            ).move_to(np.array([lx, mid_y - 0.75 + temp_tracker.get_value() / 2, 0])))
            temp_label = always_redraw(lambda: Text(
                f"{int(21 + temp_tracker.get_value() * 5)}°C",
                font_size=BODY_FONT_SIZE,
                color=P_ORANGE,
            ).move_to(np.array([lx + 1.05, mid_y - 0.7 + temp_tracker.get_value(), 0])))

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sensible"))
            self.play(
                Create(bulb), Create(tube), Create(temp_ticks), FadeIn(mercury_bulb),
                run_time=1.6,
            )
            self.play(FadeIn(sensible_tag), FadeIn(column), FadeIn(temp_label), run_time=1.2)
            hold_for(self, self.NARRATION, "sensible", used=1.6 + 1.2 + 0.35)

            container = RoundedRectangle(corner_radius=0.1, height=2.1, width=1.15, color=P_CYAN, stroke_width=3)
            container.move_to(np.array([rx, mid_y + 0.05, 0]))
            moist_ticks = VGroup(*[
                Line([rx - 0.72, y, 0], [rx - 0.58, y, 0], color=P_TEAL, stroke_width=2)
                for y in np.linspace(mid_y - 0.85, mid_y + 0.85, 5)
            ])
            latent_tag = Text("Misst Wasserdampf", font_size=LABEL_FONT_SIZE, color=P_CYAN)
            latent_tag.move_to(np.array([rx, mid_y - 1.55, 0]))
            droplet_group = VGroup(*[
                Circle(radius=0.07, color=P_CYAN, fill_color=P_CYAN, fill_opacity=0.85, stroke_width=1)
                .move_to(np.array([rx + dx, mid_y + 1.05 + dy, 0]))
                for dx, dy in [(-0.3, 0.08), (-0.08, 0.35), (0.18, 0.15), (0.35, -0.08)]
            ])
            moist_tracker = ValueTracker(0.25)
            water_fill = always_redraw(lambda: Rectangle(
                width=1.02,
                height=max(0.05, moist_tracker.get_value() * 1.7),
                color=P_BLUE,
                fill_color=P_CYAN,
                fill_opacity=0.75,
                stroke_width=0,
            ).move_to(np.array([rx, mid_y - 0.95 + (moist_tracker.get_value() * 1.7) / 2, 0])))
            rh_label = always_redraw(lambda: Text(
                f"{int(30 + moist_tracker.get_value() * 60)}% r.F.",
                font_size=BODY_FONT_SIZE,
                color=P_CYAN,
            ).move_to(np.array([rx + 1.25, mid_y - 0.95 + moist_tracker.get_value() * 1.7, 0])))

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "latent"))
            self.play(Create(container), Create(moist_ticks), run_time=1.5)
            self.play(
                FadeIn(latent_tag), FadeIn(water_fill), FadeIn(rh_label), FadeIn(droplet_group),
                run_time=1.2,
            )
            hold_for(self, self.NARRATION, "latent", used=1.5 + 1.2 + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rise"))
            self.play(
                temp_tracker.animate.set_value(1.7),
                moist_tracker.animate.set_value(0.95),
                droplet_group.animate.shift(DOWN * 0.9).set_opacity(0.2),
                run_time=4.0,
            )
            hold_for(self, self.NARRATION, "rise", used=4.0 + 0.35)

            row, items = equation_row([
                ("qges", "Q̇_ges", P_WHITE), (None, "=", P_WHITE),
                ("qsens", "Q̇_sens", P_RED), (None, "+", P_WHITE),
                ("qlat", "Q̇_lat", P_CYAN), (None, "  [W]", P_TEAL),
            ])
            row, box = formula_panel(row)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
            self.play(Create(row), Create(box), run_time=1.2)
            hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

            for key, color in (("qsens", P_RED), ("qlat", P_CYAN), ("qges", P_WHITE)):
                ring = highlight_param(items, key, color=color)
                self.play(Create(ring), run_time=0.35)
                caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
                hold_for(self, self.NARRATION, key, used=0.35 + 0.35)
                self.play(FadeOut(ring), run_time=0.2)

            self.play(FadeOut(caption), run_time=0.3)
            self.wait(0.5)

    #endregion


    #region Beat 6 — Insulated heat trap

    class Beat6_HeatTrap(Scene):
        NARRATION = [
            ("intro",
             "In a well-insulated room, internal gains have nowhere to escape.",
             "In einem gut gedämmten Raum haben interne Gewinne keinen Ausweg."),
            ("particles",
             "Heat particles drift toward the walls.",
             "Wärmeteilchen wandern zur Wand."),
            ("bounce",
             "Insulation throws them back — and each bounce leaves them hotter.",
             "Dämmung wirft sie zurück — und sie werden heißer."),
            ("trapped",
             "The room fills with trapped heat, so the cooling demand rises.",
             "Der Raum füllt sich mit eingeschlossener Wärme — der Kühlbedarf steigt."),
        ]

        def construct(self):
            apply_scene_style(self)

            title = scene_title(TITLE_DE)
            self.add(title)
            subtitle = beat_subtitle("Die isolierte Wärmefalle", title)
            self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

            caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
            self.play(FadeIn(caption), run_time=0.3)

            C_LIGHT = P_ORANGE
            C_SHARP = P_YELLOW
            C_HOT = P_RED
            C_VERY_HOT = P_RED

            # Mid-screen room (center ~ ORIGIN), shortened so floor stays above formula_panel.
            room_c = UP * 0.1
            inner_w, inner_h = 4.4, 2.2
            ins_t = 0.45
            r_dot = 0.085

            inner = Rectangle(
                width=inner_w, height=inner_h,
                color=P_WHITE, stroke_width=3.5, fill_opacity=0,
            ).move_to(room_c)
            outer = Rectangle(
                width=inner_w + 2 * ins_t, height=inner_h + 2 * ins_t,
                color=P_CYAN, stroke_width=2.5, fill_opacity=0,
            ).move_to(room_c)

            top_ins = Rectangle(
                width=inner_w + 2 * ins_t, height=ins_t,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + UP * ((inner_h + ins_t) / 2))
            bot_ins = Rectangle(
                width=inner_w + 2 * ins_t, height=ins_t,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + DOWN * ((inner_h + ins_t) / 2))
            left_ins = Rectangle(
                width=ins_t, height=inner_h,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + LEFT * ((inner_w + ins_t) / 2))
            right_ins = Rectangle(
                width=ins_t, height=inner_h,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + RIGHT * ((inner_w + ins_t) / 2))
            insulation = VGroup(top_ins, bot_ins, left_ins, right_ins)

            floor_line = Line(
                room_c + LEFT * (inner_w / 2) + DOWN * (inner_h / 2),
                room_c + RIGHT * (inner_w / 2) + DOWN * (inner_h / 2),
                color=P_TEAL, stroke_width=4,
            )
            warm_fill = Rectangle(
                width=inner_w - 0.06, height=inner_h - 0.06,
                stroke_width=0, fill_color=C_LIGHT, fill_opacity=0.0,
            ).move_to(room_c)

            room_lbl = Text("Innenraum", font_size=LABEL_FONT_SIZE, color=P_TEAL)
            room_lbl.next_to(floor_line, UP, buff=0.1)

            ins_lbl = VGroup(
                Text("Dämmung", font_size=BODY_FONT_SIZE, color=P_ORANGE),
                Text("Isolationsschicht", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            ).arrange(DOWN, buff=0.08, aligned_edge=LEFT)
            ins_lbl.next_to(outer, RIGHT, buff=0.3)
            ins_lbl.set_y(float(room_c[1]) + 0.85)

            explain_out = Text("Wärmeteilchen wandern zur Wand", font_size=LABEL_FONT_SIZE, color=C_LIGHT)
            explain_back = Text(
                "Dämmung wirft sie zurück — sie werden heißer",
                font_size=LABEL_FONT_SIZE,
                color=C_SHARP,
            )
            explain_out.move_to(room_c + DOWN * 0.75)
            explain_back.move_to(room_c + DOWN * 0.75)

            trapped_lbl = Text("Eingeschlossene Wärme", font_size=SUBTITLE_FONT_SIZE, color=WHITE)
            trapped_lbl.move_to(room_c)

            rng = np.random.default_rng(7)
            n_particles = 18
            wall_x = inner_w / 2 - r_dot
            wall_y = inner_h / 2 - r_dot
            spawn_w, spawn_h = wall_x * 0.5, wall_y * 0.5

            starts = [
                room_c + np.array([
                    float(rng.uniform(-spawn_w, spawn_w)),
                    float(rng.uniform(-spawn_h, spawn_h)),
                    0.0,
                ])
                for _ in range(n_particles)
            ]

            def make_wall_hits(seed_shift=0):
                hits = []
                for i in range(n_particles):
                    side = (i + seed_shift) % 4
                    if side == 0:
                        hits.append(np.array([
                            float(room_c[0]) + wall_x,
                            float(room_c[1]) + float(rng.uniform(-wall_y, wall_y)),
                            0.0,
                        ]))
                    elif side == 1:
                        hits.append(np.array([
                            float(room_c[0]) - wall_x,
                            float(room_c[1]) + float(rng.uniform(-wall_y, wall_y)),
                            0.0,
                        ]))
                    elif side == 2:
                        hits.append(np.array([
                            float(room_c[0]) + float(rng.uniform(-wall_x, wall_x)),
                            float(room_c[1]) + wall_y,
                            0.0,
                        ]))
                    else:
                        hits.append(np.array([
                            float(room_c[0]) + float(rng.uniform(-wall_x, wall_x)),
                            float(room_c[1]) - wall_y,
                            0.0,
                        ]))
                return hits

            def make_interior(scale=0.45):
                return [
                    room_c + np.array([
                        float(rng.uniform(-wall_x * scale, wall_x * scale)),
                        float(rng.uniform(-wall_y * scale, wall_y * scale)),
                        0.0,
                    ])
                    for _ in range(n_particles)
                ]

            wall_1 = make_wall_hits(0)
            back_1 = make_interior(0.45)
            wall_2 = make_wall_hits(1)
            back_2 = make_interior(0.4)
            wall_3 = make_wall_hits(2)
            back_3 = make_interior(0.35)

            particles = VGroup(*[
                Dot(point=s, radius=r_dot, color=C_LIGHT, fill_opacity=1.0, stroke_width=0)
                for s in starts
            ])

            paths = VGroup()
            for i in range(n_particles):
                path = VMobject()
                path.set_points_as_corners([
                    starts[i], wall_1[i], back_1[i],
                    wall_2[i], back_2[i],
                    wall_3[i], back_3[i],
                ])
                paths.add(path)

            a_wall_1, a_wall_2, a_wall_3 = 1 / 6, 3 / 6, 5 / 6

            def heat_progress(mob, alpha):
                if alpha < a_wall_1:
                    color, fill_c, fill_op = C_LIGHT, C_LIGHT, 0.0
                elif alpha < a_wall_2:
                    t = (alpha - a_wall_1) / (a_wall_2 - a_wall_1)
                    color, fill_c, fill_op = C_SHARP, C_LIGHT, 0.12 + 0.16 * t
                elif alpha < a_wall_3:
                    t = (alpha - a_wall_2) / (a_wall_3 - a_wall_2)
                    color, fill_c, fill_op = C_HOT, C_SHARP, 0.28 + 0.17 * t
                else:
                    t = (alpha - a_wall_3) / max(1e-6, 1.0 - a_wall_3)
                    color, fill_c, fill_op = C_VERY_HOT, C_HOT, 0.45 + 0.2 * t
                for p in particles:
                    p.set_color(color)
                warm_fill.set_fill(fill_c, opacity=float(fill_op))

            self.add(warm_fill)
            self.play(Create(inner), Create(floor_line), FadeIn(room_lbl), run_time=1.1)
            self.play(Create(outer), FadeIn(insulation), FadeIn(ins_lbl), run_time=1.1)
            hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.1 + 1.1)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "particles"))
            self.play(
                LaggedStart(*[FadeIn(p, scale=0.5) for p in particles], lag_ratio=0.03),
                FadeIn(explain_out),
                run_time=1.0,
            )
            hold_for(self, self.NARRATION, "particles", used=1.0 + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bounce"))
            motion_rt = 8.5
            self.play(
                AnimationGroup(*[
                    MoveAlongPath(p, path, rate_func=linear)
                    for p, path in zip(particles, paths)
                ]),
                UpdateFromAlphaFunc(particles, heat_progress),
                Succession(
                    Wait(motion_rt * a_wall_1),
                    AnimationGroup(FadeOut(explain_out), FadeIn(explain_back), run_time=0.45),
                    Wait(motion_rt * (1.0 - a_wall_1) - 0.45),
                ),
                run_time=motion_rt,
            )
            hold_for(self, self.NARRATION, "bounce", used=motion_rt + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "trapped"))
            self.play(
                FadeOut(explain_back),
                FadeOut(room_lbl),
                FadeOut(particles),
                warm_fill.animate.set_fill(C_VERY_HOT, opacity=0.72),
                insulation.animate.set_fill(opacity=0.38),
                FadeIn(trapped_lbl),
                run_time=1.4,
            )
            hold_for(self, self.NARRATION, "trapped", used=1.4 + 0.35)

            self.play(FadeOut(caption), run_time=0.3)
            self.wait(0.5)

    #endregion


    #region Beat 7 — HVAC cooling demand

    class Beat7_HvacCooling(Scene):
        NARRATION = [
            ("intro",
             "Once heat is trapped, the HVAC system has to remove it mechanically.",
             "Ist die Wärme eingeschlossen, muss die HLK sie aktiv abführen."),
            ("return",
             "Return air extracts the warm load through the exhaust grille.",
             "Abluft saugt die warme Last über den Abluftauslass ab."),
            ("supply",
             "Supply air displaces that load with cool conditioned air.",
             "Zuluft verdrängt die Last mit kühler aufbereiteter Luft."),
            ("outro",
             "That active displacement is the HVAC cooling demand driven by internal gains.",
             "Diese aktive Verdrängung ist der HLK-Kühlbedarf durch interne Gewinne."),
        ]

        def construct(self):
            apply_scene_style(self)

            title = scene_title(TITLE_DE)
            self.add(title)
            subtitle = beat_subtitle("HLK-Kühlbedarf", title)
            self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

            caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
            self.play(FadeIn(caption), run_time=0.3)

            C_HEAT = P_ORANGE
            C_HEAT_MID = P_YELLOW
            C_HEAT_HOT = P_RED
            C_COOL = P_CYAN
            C_COOL_DIM = P_TEAL
            C_WALL = P_WHITE

            # Mid-screen room (~ ORIGIN); shortened vs legacy to clear formula/caption.
            room_c = UP * 0.05
            room_w, room_h = 6.6, 2.55
            r_dot = 0.08
            room = Rectangle(
                width=room_w, height=room_h,
                color=C_WALL, stroke_width=3.5, fill_opacity=0,
            ).move_to(room_c)
            floor = Line(
                room.get_corner(DL), room.get_corner(DR),
                color=C_COOL_DIM, stroke_width=4,
            )

            warm_fill = Rectangle(
                width=room_w - 0.08, height=room_h - 0.08,
                stroke_width=0, fill_color=C_HEAT_HOT, fill_opacity=0.0,
            ).move_to(room_c)
            cool_fill = Rectangle(
                width=room_w - 0.08, height=room_h - 0.08,
                stroke_width=0, fill_color=C_COOL, fill_opacity=0.0,
            ).move_to(room_c)

            vent_w, vent_h = 1.2, 0.24
            supply = RoundedRectangle(
                width=vent_w, height=vent_h, corner_radius=0.06,
                color=C_COOL, stroke_width=2.5,
                fill_color=C_COOL, fill_opacity=0.2,
            ).move_to(room.get_top() + DOWN * 0.2 + LEFT * 2.0)
            ret = RoundedRectangle(
                width=vent_w, height=vent_h, corner_radius=0.06,
                color=C_HEAT_HOT, stroke_width=2.5,
                fill_color=C_HEAT_HOT, fill_opacity=0.2,
            ).move_to(room.get_top() + DOWN * 0.2 + RIGHT * 2.0)

            def vent_grille(vent, color):
                lines = VGroup()
                for t in (-0.35, 0.0, 0.35):
                    lines.add(Line(
                        vent.get_left() + RIGHT * 0.18 + UP * t * 0.07,
                        vent.get_right() + LEFT * 0.18 + UP * t * 0.07,
                        color=color, stroke_width=1.5, stroke_opacity=0.85,
                    ))
                return lines

            supply_grille = vent_grille(supply, C_COOL)
            ret_grille = vent_grille(ret, C_HEAT_MID)

            lbl_supply = VGroup(
                Text("Zuluft", font_size=BODY_FONT_SIZE, color=C_COOL),
                Text("kühl", font_size=LABEL_FONT_SIZE, color=C_COOL_DIM),
            ).arrange(DOWN, buff=0.04)
            lbl_supply.next_to(supply, UP, buff=0.12)

            lbl_return = VGroup(
                Text("Abluft", font_size=BODY_FONT_SIZE, color=C_HEAT_MID),
                Text("warm", font_size=LABEL_FONT_SIZE, color=C_HEAT),
            ).arrange(DOWN, buff=0.04)
            lbl_return.next_to(ret, UP, buff=0.12)

            step1 = Text("1  Interne Wärme staut sich im Raum", font_size=LABEL_FONT_SIZE, color=C_HEAT)
            step2 = Text("2  Abluft saugt warme Luft ab", font_size=LABEL_FONT_SIZE, color=C_HEAT_MID)
            step3 = Text("3  Zuluft bringt Kühlluft nach", font_size=LABEL_FONT_SIZE, color=C_COOL)
            for s in (step1, step2, step3):
                s.move_to([float(room_c[0]), float(room.get_bottom()[1]) + 0.28, 0])

            rng = np.random.default_rng(42)
            wall_x = room_w / 2 - r_dot
            wall_y = room_h / 2 - r_dot
            bounce_top = wall_y - 0.4

            def wall_point(side):
                if side == 0:
                    return room_c + np.array([wall_x, float(rng.uniform(-wall_y, bounce_top)), 0.0])
                if side == 1:
                    return room_c + np.array([-wall_x, float(rng.uniform(-wall_y, bounce_top)), 0.0])
                if side == 2:
                    return room_c + np.array([float(rng.uniform(-wall_x, wall_x)), bounce_top, 0.0])
                return room_c + np.array([float(rng.uniform(-wall_x, wall_x)), -wall_y, 0.0])

            def interior_point(scale=0.55):
                return room_c + np.array([
                    float(rng.uniform(-wall_x * scale, wall_x * scale)),
                    float(rng.uniform(-wall_y * scale, bounce_top * scale)),
                    0.0,
                ])

            def bounce_waypoints(start, n_bounces, seed_shift=0):
                pts = [np.array(start, dtype=float)]
                for k in range(n_bounces):
                    pts.append(wall_point((k + seed_shift) % 4))
                    pts.append(interior_point(0.4 + 0.08 * (k % 3)))
                return pts

            def path_from_points(pts):
                path = VMobject()
                path.set_points_as_corners(pts)
                return path

            def vent_into_room_points(vent, from_left, n_bounces, seed_shift):
                start = vent.get_bottom() + DOWN * 0.08 + np.array([
                    float(rng.uniform(-0.35, 0.35)), 0.0, 0.0,
                ])
                if from_left:
                    first = room_c + np.array([
                        float(rng.uniform(-wall_x * 0.5, wall_x * 0.2)),
                        float(rng.uniform(-0.15, bounce_top * 0.4)),
                        0.0,
                    ])
                else:
                    first = room_c + np.array([
                        float(rng.uniform(-wall_x * 0.2, wall_x * 0.5)),
                        float(rng.uniform(-0.15, bounce_top * 0.4)),
                        0.0,
                    ])
                bounce = bounce_waypoints(first, n_bounces=n_bounces, seed_shift=seed_shift)
                return [start, first] + bounce[1:]

            n_flow = 16
            n_bounces = 4
            flow_rt = 10.0

            cool_paths = VGroup()
            cool_point_lists = []
            for i in range(n_flow):
                pts = vent_into_room_points(supply, from_left=True, n_bounces=n_bounces, seed_shift=i + 1)
                cool_point_lists.append(pts)
                cool_paths.add(path_from_points(pts))

            cool_dots = VGroup(*[
                Dot(
                    point=pts[0],
                    radius=float(rng.uniform(0.055, 0.09)),
                    color=C_COOL,
                    fill_opacity=1.0,
                    stroke_width=0,
                )
                for pts in cool_point_lists
            ])

            heat_paths = VGroup()
            heat_point_lists = []
            for i in range(n_flow):
                forward = vent_into_room_points(ret, from_left=False, n_bounces=n_bounces, seed_shift=i)
                pts = [np.array(p, dtype=float) for p in reversed(forward)]
                heat_point_lists.append(pts)
                heat_paths.add(path_from_points(pts))

            heat_dots = VGroup(*[
                Dot(
                    point=pts[0],
                    radius=r_dot,
                    color=C_HEAT,
                    fill_opacity=1.0,
                    stroke_width=0,
                )
                for pts in heat_point_lists
            ])

            heat_lbl = Text("Thermische Last", font_size=BODY_FONT_SIZE, color=C_HEAT)
            heat_lbl.move_to(room_c + UP * 0.35)

            self.add(warm_fill, cool_fill)
            self.play(
                Create(room), Create(floor),
                FadeIn(supply), FadeIn(ret),
                FadeIn(supply_grille), FadeIn(ret_grille),
                run_time=1.1,
            )
            self.play(FadeIn(lbl_supply), FadeIn(lbl_return), run_time=0.5)
            self.play(
                FadeIn(step1),
                warm_fill.animate.set_fill(C_HEAT_HOT, opacity=0.22),
                LaggedStart(*[FadeIn(d, scale=0.5) for d in heat_dots], lag_ratio=0.03),
                FadeIn(heat_lbl),
                run_time=1.2,
            )
            hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.1 + 0.5 + 1.2)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "return"))
            self.play(
                FadeOut(step1),
                FadeIn(step2),
                FadeOut(heat_lbl),
                ret.animate.set_fill(C_HEAT_HOT, opacity=0.55),
                run_time=0.7,
            )

            def warm_progress(mob, alpha):
                warm_fill.set_fill(C_HEAT_HOT, opacity=0.22 - 0.18 * alpha)

            self.play(
                AnimationGroup(*[
                    MoveAlongPath(d, path, rate_func=linear)
                    for d, path in zip(heat_dots, heat_paths)
                ], lag_ratio=0.04),
                UpdateFromAlphaFunc(warm_fill, warm_progress),
                run_time=flow_rt,
            )
            self.play(FadeOut(heat_dots), run_time=0.4)
            hold_for(self, self.NARRATION, "return", used=0.7 + flow_rt + 0.4 + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "supply"))
            self.play(
                FadeOut(step2),
                FadeIn(step3),
                supply.animate.set_fill(C_COOL, opacity=0.6),
                run_time=0.7,
            )
            for d, path in zip(cool_dots, cool_paths):
                d.move_to(path.get_start())
            self.add(cool_dots)

            def cool_progress(mob, alpha):
                cool_fill.set_fill(C_COOL, opacity=0.08 + 0.32 * alpha)

            self.play(
                AnimationGroup(*[
                    MoveAlongPath(d, path, rate_func=linear)
                    for d, path in zip(cool_dots, cool_paths)
                ], lag_ratio=0.04),
                UpdateFromAlphaFunc(cool_fill, cool_progress),
                run_time=flow_rt,
            )
            hold_for(self, self.NARRATION, "supply", used=0.7 + flow_rt + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
            self.play(
                FadeOut(step3),
                cool_dots.animate.set_opacity(0.5),
                run_time=0.8,
            )
            hold_for(self, self.NARRATION, "outro", used=0.8 + 0.35)

            self.play(FadeOut(caption), run_time=0.3)
            self.wait(0.5)

    #endregion


#endregion  # Disabled beats 5–7


#region Beat 8 — Mitigation & smart design

class Beat8_Mitigation(Scene):
    NARRATION = [
        ("intro",
         "Smart design cuts the load before the chiller has to fight it.",
         "Intelligentes Design senkt die Last, bevor die Kälteanlage kämpfen muss."),
        ("high",
         "High plug and lighting loads first heat the office interior.",
         "Hohe Stecker- und Lichtlasten heizen zuerst den Büroraum."),
        ("dim",
         "Controls dim lights to thirty percent and plug loads to forty percent.",
         "Steuerung dimmt Licht auf dreißig und Steckerlasten auf vierzig Prozent."),
        ("outro",
         "Less internal heat gain means less cooling demand.",
         "Weniger interne Wärmegewinne bedeuten weniger Kühlbedarf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Minderung & intelligentes Design", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        C_HEAT = P_ORANGE
        C_HEAT_HOT = P_RED
        C_COOL = P_CYAN
        C_COOL_DIM = P_TEAL
        C_YELLOW = P_YELLOW
        C_WALL = "#1A1E28"

        # Room + control strip as one mid-screen group (combined center ~ ORIGIN).
        room_c = UP * 0.2
        room_w, room_h = 7.2, 2.35
        room_fill = Rectangle(
            width=room_w, height=room_h,
            stroke_width=0, fill_color=C_WALL, fill_opacity=0.55,
        ).move_to(room_c)
        room = Rectangle(
            width=room_w, height=room_h,
            color=P_WHITE, stroke_width=3, fill_opacity=0,
        ).move_to(room_c)
        floor_edge = Line(
            room.get_corner(DL), room.get_corner(DR),
            color=P_WHITE, stroke_width=3.5,
        )

        warm_fill = Rectangle(
            width=room_w - 0.1, height=room_h - 0.1,
            stroke_width=0, fill_color=C_HEAT_HOT, fill_opacity=0.0,
        ).move_to(room_c)

        window = RoundedRectangle(
            width=1.35, height=1.35, corner_radius=0.06,
            color=P_WHITE, stroke_width=2,
            fill_color="#243040", fill_opacity=0.9,
        ).move_to(room_c + LEFT * 2.7 + UP * 0.15)
        pane_v = Line(window.get_top(), window.get_bottom(), color=P_WHITE, stroke_width=1.5)
        pane_h = Line(window.get_left(), window.get_right(), color=P_WHITE, stroke_width=1.5)
        window_group = VGroup(window, pane_v, pane_h)

        desk_top = RoundedRectangle(
            width=2.9, height=0.12, corner_radius=0.03,
            color=P_WHITE, stroke_width=2.5, fill_opacity=0,
        ).move_to(room_c + DOWN * 0.35 + RIGHT * 0.35)
        leg_l = Line(
            desk_top.get_corner(DL) + RIGHT * 0.18,
            desk_top.get_corner(DL) + RIGHT * 0.18 + DOWN * 0.7,
            color=P_WHITE, stroke_width=2.5,
        )
        leg_r = Line(
            desk_top.get_corner(DR) + LEFT * 0.18,
            desk_top.get_corner(DR) + LEFT * 0.18 + DOWN * 0.7,
            color=P_WHITE, stroke_width=2.5,
        )
        desk = VGroup(desk_top, leg_l, leg_r)

        seat = RoundedRectangle(
            width=0.65, height=0.09, corner_radius=0.03,
            color=P_WHITE, stroke_width=1.5,
            fill_color="#3A4050", fill_opacity=1,
        ).move_to(desk_top.get_center() + DOWN * 0.42 + LEFT * 0.15)
        backrest = RoundedRectangle(
            width=0.11, height=0.55, corner_radius=0.03,
            color=P_WHITE, stroke_width=1.5,
            fill_color="#3A4050", fill_opacity=1,
        ).move_to(seat.get_center() + LEFT * 0.32 + UP * 0.28)
        chair_leg = Line(
            seat.get_bottom(), seat.get_bottom() + DOWN * 0.35,
            color=P_WHITE, stroke_width=2,
        )
        chair = VGroup(backrest, seat, chair_leg)

        laptop_base = RoundedRectangle(
            width=0.85, height=0.07, corner_radius=0.02,
            color="#9AA3B2", stroke_width=1,
            fill_color="#1C1F27", fill_opacity=1,
        ).move_to(desk_top.get_top() + UP * 0.03 + RIGHT * 0.12)
        laptop_screen = RoundedRectangle(
            width=0.78, height=0.45, corner_radius=0.04,
            color="#9AA3B2", stroke_width=1.5,
            fill_color=C_HEAT_HOT, fill_opacity=0.65,
        ).move_to(laptop_base.get_top() + UP * 0.24)
        screen_glow = Rectangle(
            width=0.6, height=0.3,
            stroke_width=0, fill_color=C_YELLOW, fill_opacity=0.2,
        ).move_to(laptop_screen.get_center())
        laptop = VGroup(laptop_base, laptop_screen, screen_glow)
        laptop_lbl = Text("Gerät", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)
        laptop_lbl.next_to(laptop_screen, RIGHT, buff=0.15)

        lamp_c = room.get_top() + DOWN * 0.42 + RIGHT * 1.2
        cord = Line(
            room.get_top() + RIGHT * 1.2 + DOWN * 0.02,
            lamp_c + UP * 0.18,
            color=P_WHITE, stroke_width=2,
        )
        fixture = Circle(
            radius=0.18,
            color=C_YELLOW,
            stroke_width=2.5,
            fill_color=C_YELLOW,
            fill_opacity=0.55,
        ).move_to(lamp_c)
        beam = Polygon(
            fixture.get_bottom() + LEFT * 0.07,
            fixture.get_bottom() + RIGHT * 0.07,
            fixture.get_bottom() + DOWN * 1.15 + RIGHT * 0.9,
            fixture.get_bottom() + DOWN * 1.15 + LEFT * 0.9,
            stroke_width=0,
            fill_color=C_YELLOW,
            fill_opacity=0.22,
        )
        light_lbl = Text("Beleuchtung", font_size=LABEL_FONT_SIZE, color=C_YELLOW)
        light_lbl.next_to(fixture, LEFT, buff=0.18)

        _slider_half = 0.52
        light_label = Text("Licht", font_size=LABEL_FONT_SIZE, color=C_YELLOW)
        light_track = Line(LEFT * _slider_half, RIGHT * _slider_half, color=P_WHITE, stroke_width=5)
        light_fill = Line(LEFT * _slider_half, RIGHT * _slider_half, color=C_YELLOW, stroke_width=5)
        light_knob = Dot(color=C_YELLOW, radius=0.1)
        light_pct = Text("100%", font_size=LABEL_FONT_SIZE, color=C_YELLOW)

        plug_label = Text("Stecker", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)
        plug_track = Line(LEFT * _slider_half, RIGHT * _slider_half, color=P_WHITE, stroke_width=5)
        plug_fill = Line(LEFT * _slider_half, RIGHT * _slider_half, color=C_HEAT_HOT, stroke_width=5)
        plug_knob = Dot(color=C_HEAT_HOT, radius=0.1)
        plug_pct = Text("100%", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)

        # Control strip sits just under the room, still above formula_panel (~ y=-1.2).
        ctrl_panel = RoundedRectangle(
            width=room_w, height=0.9, corner_radius=0.1,
            color=C_COOL_DIM, stroke_width=2,
            fill_color="#12151C", fill_opacity=0.95,
        ).next_to(room, DOWN, buff=0.18)

        ctrl_title = Text("Smarte Laststeuerung", font_size=LABEL_FONT_SIZE, color=C_COOL)
        ctrl_title.move_to(ctrl_panel.get_top() + DOWN * 0.18)

        light_track.move_to(ctrl_panel.get_center() + LEFT * 1.55 + DOWN * 0.12)
        light_fill.put_start_and_end_on(light_track.get_start(), light_track.get_end())
        light_knob.move_to(light_track.get_end())
        light_label.next_to(light_track, LEFT, buff=0.16)
        light_pct.next_to(light_track, RIGHT, buff=0.14)

        plug_track.move_to(ctrl_panel.get_center() + RIGHT * 1.55 + DOWN * 0.12)
        plug_fill.put_start_and_end_on(plug_track.get_start(), plug_track.get_end())
        plug_knob.move_to(plug_track.get_end())
        plug_label.next_to(plug_track, LEFT, buff=0.16)
        plug_pct.next_to(plug_track, RIGHT, buff=0.14)

        self.add(room_fill, warm_fill)
        self.play(
            FadeIn(room_fill),
            Create(room),
            Create(floor_edge),
            run_time=1.0,
        )
        self.play(
            FadeIn(window_group),
            FadeIn(chair),
            FadeIn(desk),
            run_time=1.0,
        )
        self.play(
            FadeIn(laptop),
            FadeIn(laptop_lbl),
            FadeIn(cord),
            FadeIn(fixture),
            FadeIn(beam),
            FadeIn(light_lbl),
            run_time=0.9,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "high"))
        self.play(
            warm_fill.animate.set_fill(C_HEAT_HOT, opacity=0.22),
            run_time=0.9,
        )
        hold_for(self, self.NARRATION, "high", used=1.0 + 1.0 + 0.9 + 0.9 + 0.35)

        self.play(
            FadeIn(ctrl_panel),
            FadeIn(ctrl_title),
            FadeIn(light_label), Create(light_track), Create(light_fill),
            FadeIn(light_knob), FadeIn(light_pct),
            FadeIn(plug_label), Create(plug_track), Create(plug_fill),
            FadeIn(plug_knob), FadeIn(plug_pct),
            run_time=1.1,
        )

        light_t, plug_t = 0.30, 0.40
        light_target = light_track.point_from_proportion(light_t)
        plug_target = plug_track.point_from_proportion(plug_t)
        new_light_pct = Text("30%", font_size=LABEL_FONT_SIZE, color=C_YELLOW).move_to(light_pct)
        new_plug_pct = Text("40%", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT).move_to(plug_pct)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "dim"))
        self.play(
            light_knob.animate.move_to(light_target),
            plug_knob.animate.move_to(plug_target),
            UpdateFromAlphaFunc(
                light_fill,
                lambda m, a: m.put_start_and_end_on(
                    light_track.get_start(),
                    light_track.point_from_proportion(1.0 - a * (1.0 - light_t)),
                ),
            ),
            UpdateFromAlphaFunc(
                plug_fill,
                lambda m, a: m.put_start_and_end_on(
                    plug_track.get_start(),
                    plug_track.point_from_proportion(1.0 - a * (1.0 - plug_t)),
                ),
            ),
            Transform(light_pct, new_light_pct),
            Transform(plug_pct, new_plug_pct),
            beam.animate.set_fill(opacity=0.05),
            fixture.animate.set_fill(opacity=0.18),
            laptop_screen.animate.set_fill(C_COOL_DIM, opacity=0.35),
            screen_glow.animate.set_opacity(0.05),
            warm_fill.animate.set_fill(C_COOL, opacity=0.12),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "dim", used=1.1 + 2.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion
