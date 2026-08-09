"""🔥 Heating Module 3 — Konvektion (Convection).

Migrated from ``merged_scenes_german.py`` onto the generate-manim-tutorial
template: fixed type scale, ``formula_panel`` with units, German ``caption_bar``
subtitles, and ``hold_for`` timing. Core animations are preserved; content is
shifted up so it never collides with the formula / caption zones.
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
    apply_scene_style, scene_title, play_scene_title,
    beat_subtitle, BEAT_SUBTITLE_FADE,
    SUBTITLE_FONT_SIZE, BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 3: Konvektion"


class Beat1_GebaeudeKonvektion(Scene):
    NARRATION = [
        ("intro",
         "Warm indoor air rises and escapes through gaps — cold outdoor air slips in to replace it.",
         "Warme Innenluft steigt und entweicht durch Fugen — kalte Außenluft strömt nach."),
        ("zones",
         "Inside stays warm; outside stays cold. The exchange through the openings is convection.",
         "Innen bleibt warm, außen bleibt kalt. Der Austausch durch die Öffnungen ist Konvektion."),
        ("flow",
         "Watch the particles: heat leaves with the orange stream while the blue stream cools the room.",
         "Beobachten Sie die Partikel: Wärme geht mit dem orangen Strom, der blaue Strom kühlt den Raum."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Das Gebäude & Konvektion", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # Floor at y≈−1.2 so captions stay clear (was −2.0).
        floor = Line([-3.0, -1.2, 0], [1.0, -1.2, 0], color=GREY_A, stroke_width=4)
        left_wall = Line([-3.0, -1.2, 0], [-3.0, 1.5, 0], color=GREY_A, stroke_width=4)
        roof_left = Line([-3.0, 1.5, 0], [-1.0, 2.5, 0], color=GREY_A, stroke_width=4)
        roof_right = Line([-1.0, 2.5, 0], [1.0, 1.5, 0], color=GREY_A, stroke_width=4)
        right_wall_bot = Line([1.0, -1.2, 0], [1.0, -0.5, 0], color=GREY_A, stroke_width=4)
        right_wall_mid = Line([1.0, 0.5, 0], [1.0, 1.1, 0], color=GREY_A, stroke_width=4)

        gap_bottom = Line([1.0, -0.5, 0], [1.0, 0.5, 0], color=P_BLUE, stroke_width=2).set_opacity(0.4)
        gap_top = Line([1.0, 1.1, 0], [1.0, 1.5, 0], color=P_ORANGE, stroke_width=2).set_opacity(0.4)

        house = VGroup(
            floor, left_wall, roof_left, roof_right,
            right_wall_bot, right_wall_mid, gap_bottom, gap_top,
        )

        txt_innen = Text("Innen", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        txt_warm = Text("Warm", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        label_inside = VGroup(txt_innen, txt_warm).arrange(DOWN, buff=0.12).move_to([-1.2, 0.1, 0])

        txt_aussen = Text("Außen", font_size=BODY_FONT_SIZE, color=P_BLUE)
        txt_kalt = Text("Kalt", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        label_outside = VGroup(txt_aussen, txt_kalt).arrange(DOWN, buff=0.12).move_to([2.6, 0.1, 0])

        self.play(Create(house), run_time=1.5)
        self.play(FadeIn(label_inside), FadeIn(label_outside), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=1.5 + 1.0 + 0.35)

        np.random.seed(42)
        orange_dots = VGroup(*[
            Dot(
                point=[np.random.uniform(-2.7, 0.5), np.random.uniform(-0.9, 1.1), 0],
                radius=0.04, color=P_ORANGE,
            )
            for _ in range(70)
        ])
        blue_dots = VGroup(*[
            Dot(
                point=[np.random.uniform(1.3, 3.3), np.random.uniform(-1.2, 0.6), 0],
                radius=0.04, color=P_BLUE,
            )
            for _ in range(70)
        ])

        self.play(FadeIn(orange_dots), FadeIn(blue_dots), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zones"))
        hold_for(self, self.NARRATION, "zones", used=1.0 + 0.35)

        def update_orange_particles(group, dt):
            for dot in group:
                pos = dot.get_center()
                if pos[0] < 1.0:
                    dx = 0.45 * dt
                    dy = (1.1 - pos[1]) * 0.4 * dt + 0.15 * dt
                    pos += np.array([dx, dy, 0])
                else:
                    pos += np.array([0.5 * dt, 0.8 * dt, 0])
                    if pos[1] > 2.6 or pos[0] > 3.8:
                        pos[0] = np.random.uniform(-2.7, -0.5)
                        pos[1] = np.random.uniform(-0.9, -0.1)
                dot.move_to(pos)

        def update_blue_particles(group, dt):
            for dot in group:
                pos = dot.get_center()
                if pos[0] > 1.0:
                    dx = -0.5 * dt
                    dy = (-0.2 - pos[1]) * 0.4 * dt
                    pos += np.array([dx, dy, 0])
                else:
                    pos += np.array([-0.5 * dt, -0.3 * dt, 0])
                    if pos[0] < -2.7 or pos[1] < -1.0:
                        pos[0] = np.random.uniform(1.5, 3.5)
                        pos[1] = np.random.uniform(-1.0, 0.4)
                dot.move_to(pos)

        orange_dots.add_updater(update_orange_particles)
        blue_dots.add_updater(update_blue_particles)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flow"))
        hold_for(self, self.NARRATION, "flow", used=0.35)

        orange_dots.remove_updater(update_orange_particles)
        blue_dots.remove_updater(update_blue_particles)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat2_Innenvolumen(Scene):
    NARRATION = [
        ("intro",
         "The heated space inside the building envelope is the net volume V.",
         "Der beheizte Raum innerhalb der Hülle ist das Nettovolumen V."),
        ("focus",
         "Outdoor air fades away — we care only about the indoor volume we must heat.",
         "Die Außenluft verblasst — uns interessiert nur das Innenvolumen, das wir heizen."),
        ("fill",
         "That filled volume is V in cubic meters, per DIN EN 12831.",
         "Dieses gefüllte Volumen ist V in Kubikmetern, nach DIN EN 12831."),
        ("label",
         "V is the net heated volume — the starting point for ventilation heat loss.",
         "V ist das nettotemperierte Volumen — Ausgangspunkt für den Lüftungswärmeverlust."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Gebäude-Innenvolumen V", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        v_bottom_left = np.array([-2.5, -1.2, 0])
        v_bottom_right = np.array([2.5, -1.2, 0])
        v_top_right = np.array([2.5, 0.9, 0])
        v_roof_peak = np.array([0.0, 2.3, 0])
        v_top_left = np.array([-2.5, 0.9, 0])
        interior_points = [v_bottom_left, v_bottom_right, v_top_right, v_roof_peak, v_top_left]

        house_outline = Polygon(*interior_points, color="#8A9BA8", stroke_width=3)
        ground = Line(start=[-3.8, -1.2, 0], end=[3.8, -1.2, 0], color="#5A6577", stroke_width=2)

        blue_positions = [
            [-3.2, 1.7, 0], [-3.5, -0.1, 0], [-3.0, -0.9, 0],
            [3.2, 2.0, 0], [3.6, 0.4, 0], [3.3, -0.6, 0],
            [-1.5, 2.9, 0], [1.5, 2.9, 0], [0.0, 3.15, 0],
        ]
        blue_dots = VGroup(*[Dot(point=pos, color=P_BLUE, radius=0.08) for pos in blue_positions])

        orange_positions = [
            [-1.8, -0.5, 0], [-0.6, -0.6, 0], [0.7, -0.5, 0], [1.8, -0.6, 0],
            [-1.6, 0.3, 0], [-0.5, 0.4, 0], [0.6, 0.3, 0], [1.7, 0.4, 0],
            [-1.0, 1.1, 0], [0.0, 1.4, 0], [1.0, 1.1, 0], [0.0, 0.1, 0],
        ]
        orange_dots = VGroup(*[Dot(point=pos, color=P_ORANGE, radius=0.08) for pos in orange_positions])

        self.add(ground, house_outline, blue_dots, orange_dots)
        hold_for(self, self.NARRATION, "intro", used=0.3 + 0.3)

        self.play(
            FadeOut(blue_dots, run_time=1.2),
            orange_dots.animate(run_time=1.2).set_opacity(0.3),
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "focus"))
        hold_for(self, self.NARRATION, "focus", used=1.2 + 0.35)

        interior_fill = Polygon(
            *interior_points, fill_color=P_ORANGE, fill_opacity=0.35, stroke_width=0,
        )
        self.play(FadeIn(interior_fill), run_time=1.5)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fill"))
        hold_for(self, self.NARRATION, "fill", used=1.5 + 0.35)

        row, items = equation_row([
            ("v", "V", P_ORANGE),
            (None, "  [m³]", P_WHITE),
        ])
        row, box = formula_panel(row)
        self.play(FadeIn(row), Create(box), run_time=0.8)

        v_label = Text("V", font_size=FORMULA_FONT_SIZE, color=P_ORANGE)
        v_subtext = Text("Nettovolumen (m³)", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        v_group = VGroup(v_label, v_subtext).arrange(DOWN, buff=0.1).move_to([0.0, 0.35, 0])
        self.play(FadeIn(v_label), FadeIn(v_subtext, shift=UP * 0.15), run_time=1.0)

        dim_line = Line(start=[-2.5, -1.45, 0], end=[2.5, -1.45, 0], color=P_ORANGE, stroke_width=1.5)
        dim_tick_l = Line(start=[-2.5, -1.55, 0], end=[-2.5, -1.35, 0], color=P_ORANGE, stroke_width=1.5)
        dim_tick_r = Line(start=[2.5, -1.55, 0], end=[2.5, -1.35, 0], color=P_ORANGE, stroke_width=1.5)
        self.play(Create(VGroup(dim_line, dim_tick_l, dim_tick_r)), run_time=1.0)

        ring = highlight_param(items, "v", color=P_ORANGE)
        self.play(Create(ring), run_time=0.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "label"))
        hold_for(self, self.NARRATION, "label", used=3.2 + 0.35)
        self.play(FadeOut(ring), run_time=0.2)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat3_Luftwechselrate(Scene):
    NARRATION = [
        ("volume",
         "Start again from the heated volume V inside the house.",
         "Wieder vom beheizten Volumen V im Haus ausgehen."),
        ("rate",
         "The air change rate n tells how often that volume is exchanged per hour.",
         "Die Luftwechselrate n sagt, wie oft dieses Volumen pro Stunde ausgetauscht wird."),
        ("one_change",
         "One air change means the entire volume is replaced — n equals one per hour.",
         "Ein Luftwechsel heißt: das ganze Volumen wird ersetzt — n gleich eins pro Stunde."),
        ("product",
         "Volume times air change rate — V times n — sets the ventilation airflow.",
         "Volumen mal Luftwechselrate — V mal n — bestimmt den Lüftungsvolumenstrom."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Luftwechselrate n", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "volume"))
        self.play(FadeIn(caption), run_time=0.3)

        house_points = [
            np.array([-2.0, -1.0, 0]),
            np.array([2.0, -1.0, 0]),
            np.array([2.0, 0.9, 0]),
            np.array([0.0, 2.1, 0]),
            np.array([-2.0, 0.9, 0]),
        ]
        house_shift = LEFT * 2.6 + UP * 0.15

        house_fill = Polygon(*house_points, fill_color=P_ORANGE, fill_opacity=0.3, stroke_width=0)
        house_outline = Polygon(*house_points, color=P_WHITE, stroke_width=3, fill_opacity=0)
        house_fill.shift(house_shift)
        house_outline.shift(house_shift)

        v_label = Text("V", font_size=FORMULA_FONT_SIZE, color=P_ORANGE).move_to(house_outline.get_center())

        self.play(Create(house_outline), FadeIn(house_fill), FadeIn(v_label), run_time=2.0)
        hold_for(self, self.NARRATION, "volume", used=2.0 + 0.3)

        clock_center = RIGHT * 2.4 + UP * 0.1
        clock_radius = 0.85
        clock_circle = Circle(radius=clock_radius, color=P_WHITE, stroke_width=3).move_to(clock_center)
        ticks = VGroup()
        for i in range(12):
            angle = i * (2 * PI / 12)
            start = clock_center + 0.70 * clock_radius * np.array([np.sin(angle), np.cos(angle), 0])
            end = clock_center + 0.92 * clock_radius * np.array([np.sin(angle), np.cos(angle), 0])
            ticks.add(Line(start, end, color=P_WHITE, stroke_width=2))
        hand = Line(clock_center, clock_center + UP * 0.6, color=P_WHITE, stroke_width=3)
        clock_icon = VGroup(clock_circle, ticks, hand)

        n_symbol = Text("n", font_size=FORMULA_FONT_SIZE, color=P_WHITE).next_to(clock_icon, UP, buff=0.3)
        line1 = Text("Luftwechselrate n (1/h)", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        line2 = Text("(DIN 1946-6)", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        n_subtext = VGroup(line1, line2).arrange(DOWN, buff=0.08).next_to(clock_icon, DOWN, buff=0.25)

        self.play(Create(clock_icon), FadeIn(n_symbol), FadeIn(n_subtext, shift=UP * 0.2), run_time=2.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rate"))
        hold_for(self, self.NARRATION, "rate", used=2.0 + 0.35)

        self.play(
            Rotate(hand, angle=-2 * PI, about_point=clock_center, rate_func=linear),
            house_fill.animate.set_fill(P_CYAN, opacity=0.2),
            run_time=3.5,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "one_change"))
        hold_for(self, self.NARRATION, "one_change", used=3.5 + 0.35)

        row, items = equation_row([
            ("v", "V", P_ORANGE), (None, "·", P_WHITE),
            ("n", "n", P_WHITE),
            (None, "  [m³ · 1/h]", P_WHITE),
        ])
        row, box = formula_panel(row)

        self.play(
            FadeOut(n_subtext),
            FadeIn(row), Create(box),
            run_time=1.5,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "product"))
        hold_for(self, self.NARRATION, "product", used=1.5 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat4_SpezWaermekapazitaet(Scene):
    NARRATION = [
        ("cube",
         "Take one cubic meter of air — the unit volume we must heat.",
         "Nehmen wir einen Kubikmeter Luft — das Einheitsvolumen, das wir heizen."),
        ("coil",
         "A heating element warms that cubic meter from below.",
         "Ein Heizelement erwärmt diesen Kubikmeter von unten."),
        ("heat",
         "Heat waves rise into the cube until the air turns warm.",
         "Wärmewellen steigen in den Würfel, bis die Luft warm wird."),
        ("c_luft",
         "The energy needed per cubic meter and kelvin is c_Luft — about 0.34 watt-hours.",
         "Die Energie pro Kubikmeter und Kelvin ist c_Luft — etwa 0,34 Wattstunden."),
        ("product",
         "So the product grows: V times n times c_Luft.",
         "Das Produkt wächst: V mal n mal c_Luft."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Spezifische Wärmekapazität c_Luft", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "cube"))
        self.play(FadeIn(caption), run_time=0.3)

        eq_v = Text("V", color=P_ORANGE, font_size=BODY_FONT_SIZE)
        eq_times1 = Text(" · ", color=P_WHITE, font_size=BODY_FONT_SIZE)
        eq_n = Text("n", color=P_WHITE, font_size=BODY_FONT_SIZE)
        top_eq_initial = VGroup(eq_v, eq_times1, eq_n).arrange(RIGHT, buff=0.08)
        top_eq_initial.next_to(subtitle, DOWN, buff=0.25)
        self.add(top_eq_initial)

        scale_fac = 1.0
        c_center = UP * 0.55

        top_pt = c_center + UP * 1.0 * scale_fac
        tr_pt = c_center + (RIGHT * 0.866 + UP * 0.5) * scale_fac
        br_pt = c_center + (RIGHT * 0.866 + DOWN * 0.5) * scale_fac
        bot_pt = c_center + DOWN * 1.0 * scale_fac
        bl_pt = c_center + (LEFT * 0.866 + DOWN * 0.5) * scale_fac
        tl_pt = c_center + (LEFT * 0.866 + UP * 0.5) * scale_fac

        face_top = Polygon(
            c_center, tl_pt, top_pt, tr_pt,
            fill_color=P_BLUE, fill_opacity=0.35, stroke_color=P_CYAN, stroke_width=2,
        )
        face_left = Polygon(
            c_center, tl_pt, bl_pt, bot_pt,
            fill_color="#1565C0", fill_opacity=0.45, stroke_color=P_CYAN, stroke_width=2,
        )
        face_right = Polygon(
            c_center, tr_pt, br_pt, bot_pt,
            fill_color="#0D47A1", fill_opacity=0.55, stroke_color=P_CYAN, stroke_width=2,
        )
        cube = VGroup(face_top, face_left, face_right)
        cube_label = Text("1 m³", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(face_top.get_center())

        self.play(FadeIn(cube, shift=UP * 0.3), FadeIn(cube_label), run_time=2.0)
        hold_for(self, self.NARRATION, "cube", used=2.0 + 0.3)

        coil_y = -1.15
        coil = ParametricFunction(
            lambda t: np.array([t, 0.1 * np.sin(10 * t) + coil_y, 0]),
            t_range=[-1.1, 1.1],
            color=P_RED,
        ).set_stroke(width=4)
        coil_label = Text("Heizelement", font_size=LABEL_FONT_SIZE, color=P_RED).next_to(coil, DOWN, buff=0.1)

        self.play(Create(coil), FadeIn(coil_label), run_time=1.5)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "coil"))
        hold_for(self, self.NARRATION, "coil", used=1.5 + 0.35)

        heat_lines = VGroup()
        for x_off in [-0.7, -0.35, 0.0, 0.35, 0.7]:
            line = ParametricFunction(
                lambda t, xo=x_off: np.array([xo + 0.05 * np.sin(8 * t), t + coil_y + 0.15, 0]),
                t_range=[0, 0.85],
                color=P_ORANGE,
            ).set_stroke(width=2.5, opacity=0.8)
            heat_lines.add(line)

        face_top_warm = Polygon(
            c_center, tl_pt, top_pt, tr_pt,
            fill_color=P_ORANGE, fill_opacity=0.45, stroke_color=P_ORANGE, stroke_width=2,
        )
        face_left_warm = Polygon(
            c_center, tl_pt, bl_pt, bot_pt,
            fill_color="#E65100", fill_opacity=0.55, stroke_color=P_ORANGE, stroke_width=2,
        )
        face_right_warm = Polygon(
            c_center, tr_pt, br_pt, bot_pt,
            fill_color="#BF360C", fill_opacity=0.65, stroke_color=P_ORANGE, stroke_width=2,
        )

        self.play(
            Create(heat_lines),
            Transform(face_top, face_top_warm),
            Transform(face_left, face_left_warm),
            Transform(face_right, face_right_warm),
            run_time=2.5,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "heat"))
        hold_for(self, self.NARRATION, "heat", used=2.5 + 0.35)

        c_air_tag = Text("c_Luft", color=P_GREEN, font_size=BODY_FONT_SIZE)
        c_air_desc1 = Text("Spez. Wärmekapazität", color=P_GREEN, font_size=LABEL_FONT_SIZE)
        c_air_desc2 = Text("(0,34 Wh/(m³·K))", color=P_GREEN, font_size=LABEL_FONT_SIZE)
        c_air_group = VGroup(c_air_tag, c_air_desc1, c_air_desc2).arrange(DOWN, buff=0.08)
        c_air_group.next_to(cube, RIGHT, buff=0.45).shift(DOWN * 0.1)

        self.play(FadeIn(c_air_group, shift=RIGHT * 0.2), run_time=1.5)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "c_luft"))
        hold_for(self, self.NARRATION, "c_luft", used=1.5 + 0.35)

        row, items = equation_row([
            ("v", "V", P_ORANGE), (None, "·", P_WHITE),
            ("n", "n", P_WHITE), (None, "·", P_WHITE),
            ("c", "c_Luft", P_GREEN),
            (None, "  [Wh/(m³·K)]", P_WHITE),
        ])
        row, box = formula_panel(row)
        self.play(
            FadeOut(top_eq_initial),
            FadeIn(row), Create(box),
            run_time=1.2,
        )
        ring = highlight_param(items, "c", color=P_GREEN)
        self.play(Create(ring), run_time=0.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "product"))
        hold_for(self, self.NARRATION, "product", used=1.6 + 0.35)
        self.play(FadeOut(ring), run_time=0.2)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat5_Lueftungsverlust(Scene):
    NARRATION = [
        ("intro",
         "Ventilation heat loss closes the equation with the temperature difference delta theta.",
         "Der Lüftungswärmeverlust schließt die Gleichung mit der Temperaturdifferenz Delta-Theta."),
        ("delta",
         "Delta theta is indoor temperature minus outdoor temperature — the driving force.",
         "Delta-Theta ist Innentemperatur minus Außentemperatur — die Triebkraft."),
        ("formula",
         "Phi_V equals V times n times c_Luft times delta theta — in watts, per DIN EN 12831-1.",
         "Phi_V ist V mal n mal c_Luft mal Delta-Theta — in Watt, nach DIN EN 12831-1."),
        ("v",
         "V is the building volume in cubic meters.",
         "V ist das Gebäudevolumen in Kubikmetern."),
        ("n",
         "n is the air change rate in one per hour.",
         "n ist die Luftwechselrate in Eins pro Stunde."),
        ("c",
         "c_Luft is the specific heat capacity of air — 0.34 watt-hours per cubic meter and kelvin.",
         "c_Luft ist die spez. Wärmekapazität der Luft — 0,34 Wh pro Kubikmeter und Kelvin."),
        ("dt",
         "And delta theta is the temperature difference in kelvin.",
         "Und Delta-Theta ist die Temperaturdifferenz in Kelvin."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Lüftungswärmeverlust Φ_V", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        scale_fac = 0.85
        c_center = UP * 0.85
        top_pt = c_center + UP * 1.0 * scale_fac
        tr_pt = c_center + (RIGHT * 0.866 + UP * 0.5) * scale_fac
        br_pt = c_center + (RIGHT * 0.866 + DOWN * 0.5) * scale_fac
        bot_pt = c_center + DOWN * 1.0 * scale_fac
        bl_pt = c_center + (LEFT * 0.866 + DOWN * 0.5) * scale_fac
        tl_pt = c_center + (LEFT * 0.866 + UP * 0.5) * scale_fac

        face_top = Polygon(
            c_center, tl_pt, top_pt, tr_pt,
            fill_color=P_ORANGE, fill_opacity=0.35, stroke_color=P_ORANGE, stroke_width=2,
        )
        face_left = Polygon(
            c_center, tl_pt, bl_pt, bot_pt,
            fill_color="#E65100", fill_opacity=0.45, stroke_color=P_ORANGE, stroke_width=2,
        )
        face_right = Polygon(
            c_center, tr_pt, br_pt, bot_pt,
            fill_color="#BF360C", fill_opacity=0.55, stroke_color=P_ORANGE, stroke_width=2,
        )
        cube = VGroup(face_top, face_left, face_right)
        cube_label = Text("1 m³", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(face_top.get_center())
        loss_label = Text("Lüftungsverlust", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(cube, UP, buff=0.2)
        prev_visuals = VGroup(cube, cube_label, loss_label)

        self.play(FadeIn(prev_visuals), run_time=0.8)
        hold_for(self, self.NARRATION, "intro", used=0.8 + 0.3)

        self.play(FadeOut(prev_visuals), run_time=0.8)

        t_inside = Text("T_innen  (Innentemperatur)", color=P_RED, font_size=LABEL_FONT_SIZE)
        t_inside.move_to(UP * 1.1 + RIGHT * 1.0)
        t_outside = Text("T_außen (Außentemperatur)", color=P_BLUE, font_size=LABEL_FONT_SIZE)
        t_outside.move_to(UP * 0.05 + RIGHT * 1.0)

        dt_brace = BraceBetweenPoints(
            t_outside.get_left() + LEFT * 0.3,
            t_inside.get_left() + LEFT * 0.3,
            direction=LEFT, color=P_YELLOW,
        )
        dt_label = Text("Δθ", color=P_YELLOW, font_size=FORMULA_FONT_SIZE).next_to(dt_brace, LEFT, buff=0.2)
        dt_sub = Text("Temperaturdifferenz", color=P_YELLOW, font_size=LABEL_FONT_SIZE).next_to(dt_label, DOWN, buff=0.12)

        self.play(
            FadeIn(t_inside, shift=LEFT * 0.2),
            FadeIn(t_outside, shift=LEFT * 0.2),
            GrowFromCenter(dt_brace),
            FadeIn(dt_label),
            FadeIn(dt_sub),
            run_time=1.8,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "delta"))
        hold_for(self, self.NARRATION, "delta", used=1.8 + 0.35)

        row, items = equation_row([
            ("phi", "Φ_V", P_WHITE), (None, "=", P_WHITE),
            ("v", "V", P_ORANGE), (None, "·", P_WHITE),
            ("n", "n", P_WHITE), (None, "·", P_WHITE),
            ("c", "c_Luft", P_GREEN), (None, "·", P_WHITE),
            ("dt", "Δθ", P_YELLOW),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row)

        self.play(
            FadeOut(t_inside), FadeOut(t_outside), FadeOut(dt_brace), FadeOut(dt_sub), FadeOut(dt_label),
            FadeIn(row), Create(box),
            run_time=1.5,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        hold_for(self, self.NARRATION, "formula", used=1.5 + 0.35)

        card_v = VGroup(
            Text("V", color=P_ORANGE, font_size=BODY_FONT_SIZE),
            Text("Gebäudevolumen", color=P_WHITE, font_size=LABEL_FONT_SIZE),
            Text("(m³, DIN EN 12831)", color=GREY_A, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.06)
        card_n = VGroup(
            Text("n", color=P_WHITE, font_size=BODY_FONT_SIZE),
            Text("Luftwechselrate", color=P_WHITE, font_size=LABEL_FONT_SIZE),
            Text("(1/h, DIN 1946-6)", color=GREY_A, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.06)
        card_c = VGroup(
            Text("c_Luft", color=P_GREEN, font_size=BODY_FONT_SIZE),
            Text("Spez. Wärmekapazität", color=P_WHITE, font_size=LABEL_FONT_SIZE),
            Text("(0,34 Wh/(m³·K))", color=GREY_A, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.06)
        card_dt = VGroup(
            Text("Δθ", color=P_YELLOW, font_size=BODY_FONT_SIZE),
            Text("Temperaturdifferenz", color=P_WHITE, font_size=LABEL_FONT_SIZE),
            Text("(K)", color=GREY_A, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.06)

        cards = VGroup(card_v, card_n, card_c, card_dt).arrange(RIGHT, buff=0.4)
        cards.move_to(UP * 0.55)

        highlights = [
            ("v", card_v, P_ORANGE, "v"),
            ("n", card_n, P_WHITE, "n"),
            ("c", card_c, P_GREEN, "c"),
            ("dt", card_dt, P_YELLOW, "dt"),
        ]
        for key, card, color, narr_key in highlights:
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), FadeIn(card, shift=UP * 0.1), run_time=0.55)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, narr_key))
            hold_for(self, self.NARRATION, narr_key, used=0.55 + 0.35)
            self.play(FadeOut(ring), run_time=0.2)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat6_Waermerueckgewinnung(Scene):
    NARRATION = [
        ("without",
         "Without heat recovery, the full ventilation loss applies — eta equals zero.",
         "Ohne Wärmerückgewinnung gilt der volle Lüftungsverlust — Eta gleich null."),
        ("with",
         "With WRG, multiply by one minus eta_WRG — only the unrecovered fraction remains.",
         "Mit WRG multiplizieren wir mit eins minus Eta_WRG — nur der Restverlust bleibt."),
        ("bars",
         "At eighty percent recovery, only twenty percent of the ventilation loss remains.",
         "Bei achtzig Prozent Rückgewinnung bleiben nur zwanzig Prozent des Verlusts."),
        ("eta",
         "Eta_WRG is typically seventy to ninety percent for modern systems.",
         "Eta_WRG liegt typisch bei siebzig bis neunzig Prozent bei modernen Anlagen."),
        ("phi",
         "Phi_V is then the reduced ventilation heating load in watts.",
         "Phi_V ist dann die reduzierte Lüftungsheizlast in Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmerückgewinnung (WRG)", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "without"))
        self.play(FadeIn(caption), run_time=0.3)

        row_old, items_old = equation_row([
            ("phi", "Φ_V", P_RED), (None, "=", P_WHITE),
            ("v", "V", P_ORANGE), (None, "·", P_WHITE),
            ("n", "n", P_WHITE), (None, "·", P_WHITE),
            ("c", "c_Luft", P_GREEN), (None, "·", P_WHITE),
            ("dt", "Δθ", P_YELLOW),
            (None, "  [W]", P_WHITE),
        ])
        row_old, box_old = formula_panel(row_old)
        old_note = Text("ohne WRG (η = 0%)", font_size=LABEL_FONT_SIZE, color=GREY_A)
        old_note.next_to(box_old, UP, buff=0.12)

        self.play(FadeIn(row_old), Create(box_old), FadeIn(old_note), run_time=1.3)
        hold_for(self, self.NARRATION, "without", used=1.3 + 0.3)

        self.play(FadeOut(row_old), FadeOut(box_old), FadeOut(old_note), run_time=0.5)

        row, items = equation_row([
            ("phi", "Φ_V", P_RED), (None, "=", P_WHITE),
            ("v", "V", P_ORANGE), (None, "·", P_WHITE),
            ("n", "n", P_WHITE), (None, "·", P_WHITE),
            ("eta", "(1 − η_WRG)", P_GREEN), (None, "·", P_WHITE),
            ("c", "c_Luft", P_GREEN), (None, "·", P_WHITE),
            ("dt", "Δθ", P_YELLOW),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row)
        self.play(FadeIn(row), Create(box), run_time=1.4)
        ring_eta = highlight_param(items, "eta", color=P_GREEN)
        self.play(Create(ring_eta), run_time=0.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "with"))
        hold_for(self, self.NARRATION, "with", used=1.8 + 0.35)
        self.play(FadeOut(ring_eta), run_time=0.2)

        bar_label1 = Text("Ohne WRG (η = 0%)", font_size=LABEL_FONT_SIZE, color=P_RED)
        bar1 = RoundedRectangle(
            width=4.0, height=0.35, corner_radius=0.05,
            color=P_RED, fill_color=P_RED, fill_opacity=0.6,
        )
        bar_text1 = Text("100% Lüftungsverlust", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(bar1)
        bar_group1 = VGroup(bar_label1, VGroup(bar1, bar_text1)).arrange(RIGHT, buff=0.25)

        bar_label2 = Text("Mit WRG (η = 80%)", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        bar2_saved = RoundedRectangle(
            width=3.2, height=0.35, corner_radius=0.05,
            color=P_GREEN, fill_color=P_GREEN, fill_opacity=0.6,
        )
        bar_text2_saved = Text("80% Eingespart", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(bar2_saved)
        bar2_loss = RoundedRectangle(
            width=0.8, height=0.35, corner_radius=0.05,
            color=P_RED, fill_color=P_RED, fill_opacity=0.6,
        ).next_to(bar2_saved, RIGHT, buff=0.0)
        bar_text2_loss = Text("20%", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(bar2_loss)
        bar_group2 = VGroup(
            bar_label2, VGroup(bar2_saved, bar_text2_saved, bar2_loss, bar_text2_loss)
        ).arrange(RIGHT, buff=0.3)

        comparison_visual = (
            VGroup(bar_group1, bar_group2)
            .arrange(DOWN, aligned_edge=LEFT, buff=0.28)
            .move_to(UP * 0.55)
        )

        self.play(FadeIn(comparison_visual, shift=UP * 0.15), run_time=1.3)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bars"))
        hold_for(self, self.NARRATION, "bars", used=1.3 + 0.35)

        self.play(FadeOut(comparison_visual), run_time=0.8)

        card_eta = VGroup(
            Text("η_WRG", color=P_GREEN, font_size=BODY_FONT_SIZE),
            Text("Wärmerückgewinnungsgrad", color=P_WHITE, font_size=LABEL_FONT_SIZE),
            Text("Typisch: 0,70–0,90", color=GREY_A, font_size=LABEL_FONT_SIZE),
            Text("DIN EN 13141-7 / DIN 1946-6", color=P_CYAN, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.06)

        card_phi = VGroup(
            Text("Φ_V", color=P_RED, font_size=BODY_FONT_SIZE),
            Text("Reduzierte Lüftungsheizlast", color=P_WHITE, font_size=LABEL_FONT_SIZE),
            Text("Watt [W]", color=GREY_A, font_size=LABEL_FONT_SIZE),
            Text("DIN EN 12831-1", color=P_CYAN, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.06)

        cards = VGroup(card_eta, card_phi).arrange(RIGHT, buff=0.9).move_to(UP * 0.55)

        ring_eta2 = highlight_param(items, "eta", color=P_GREEN)
        self.play(Create(ring_eta2), FadeIn(card_eta, shift=UP * 0.15), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "eta"))
        hold_for(self, self.NARRATION, "eta", used=1.0 + 0.35)
        self.play(FadeOut(ring_eta2), run_time=0.2)

        ring_phi = highlight_param(items, "phi", color=P_RED)
        self.play(Create(ring_phi), FadeIn(card_phi, shift=UP * 0.15), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "phi"))
        hold_for(self, self.NARRATION, "phi", used=1.0 + 0.35)
        self.play(FadeOut(ring_phi), run_time=0.2)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat7_Lueftungssysteme(Scene):
    NARRATION = [
        ("window",
         "Window ventilation: free air change with zero heat recovery — full ventilation loss.",
         "Fensterlüftung: freier Luftwechsel ohne WRG — voller Lüftungsverlust."),
        ("central",
         "A central WRG unit preheats supply air and cuts loss to roughly ten percent.",
         "Eine zentrale WRG-Anlage vorwärmt die Zuluft und senkt den Verlust auf etwa zehn Prozent."),
        ("decentral",
         "Decentralized ceramic push-pull units recover seventy to ninety percent room by room.",
         "Dezentrale Keramik-Pendellüfter rückgewinnen siebzig bis neunzig Prozent raumweise."),
        ("outro",
         "DIN 1946-6: mechanical WRG can cut ventilation heat loss by up to ninety percent.",
         "DIN 1946-6: mechanische WRG kann den Lüftungswärmeverlust um bis zu 90% senken."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Lüftungssysteme im Vergleich", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "window"))
        self.play(FadeIn(caption), run_time=0.3)

        col_w, col_h = 4.3, 4.6
        pos1 = LEFT * 4.4 + DOWN * 0.25
        pos2 = DOWN * 0.25
        pos3 = RIGHT * 4.4 + DOWN * 0.25

        def create_cube_graphic(center_pos):
            face_top = Polygon(
                center_pos + UP * 0.6,
                center_pos + RIGHT * 0.85 + UP * 0.25,
                center_pos + DOWN * 0.1,
                center_pos + LEFT * 0.85 + UP * 0.25,
                color=P_WHITE, fill_color="#1E293B", fill_opacity=0.85, stroke_width=2,
            )
            face_left = Polygon(
                center_pos + LEFT * 0.85 + UP * 0.25,
                center_pos + DOWN * 0.1,
                center_pos + DOWN * 0.95,
                center_pos + LEFT * 0.85 + DOWN * 0.6,
                color=P_WHITE, fill_color="#0F172A", fill_opacity=0.85, stroke_width=2,
            )
            face_right = Polygon(
                center_pos + DOWN * 0.1,
                center_pos + RIGHT * 0.85 + UP * 0.25,
                center_pos + RIGHT * 0.85 + DOWN * 0.6,
                center_pos + DOWN * 0.95,
                color=P_WHITE, fill_color="#1E293B", fill_opacity=0.75, stroke_width=2,
            )
            return VGroup(face_top, face_left, face_right)

        # Panel 1 — Fensterlüftung
        card1 = RoundedRectangle(
            width=col_w, height=col_h, corner_radius=0.15,
            color=P_RED, fill_color="#1a1215", fill_opacity=0.85,
        ).move_to(pos1)
        t1_main = Text("1. FENSTERLÜFTUNG", font_size=LABEL_FONT_SIZE, color=P_RED)
        t1_sub = Text("Freie Lüftung (Ohne WRG)", font_size=LABEL_FONT_SIZE, color=GREY_A)
        hdr1 = VGroup(t1_main, t1_sub).arrange(DOWN, buff=0.05).move_to(card1.get_top() + DOWN * 0.45)

        cube1 = create_cube_graphic(pos1 + DOWN * 0.1)
        arr_in1 = CurvedArrow(
            pos1 + LEFT * 1.8 + DOWN * 0.7, pos1 + LEFT * 0.7 + DOWN * 0.4,
            radius=-1.0, color=P_BLUE, stroke_width=1.5,
        )
        # Further below the arrow, not pulled back up toward it: at the current
        # label size this used to sit low enough to clear the cube icon, but
        # the UP shift pulls it back up onto the cube's front face.
        lbl_in1 = Text("Kaltluft (−5°C)", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        lbl_in1.scale(0.75).next_to(arr_in1, DOWN, buff=0.10)
        arr_out1 = CurvedArrow(
            pos1 + RIGHT * 0.2 + UP * 0.5, pos1 + RIGHT * 1.5 + UP * 1.3,
            radius=0.9, color=P_ORANGE, stroke_width=3.5,
        )
        # Anchored to the arrow's low start point, not next_to() the whole
        # curved arrow: that bbox reaches up to the curve's high end near the
        # tip, which pushed this label up into the card header above it.
        lbl_out1 = Paragraph(
            "Warmluft (+21°C)", "entweicht!",
            font_size=LABEL_FONT_SIZE, color=P_ORANGE, alignment="center",
        ).scale(0.75)
        lbl_out1.move_to(arr_out1.get_start() + UP * 0.30 + RIGHT * 0.55)
        stat1_wrg = Text("WRG: 0%", font_size=LABEL_FONT_SIZE, color=P_RED)
        stat1_loss = Text("Lüftungsverlust: 100% (Sehr Hoch)", font_size=LABEL_FONT_SIZE, color=P_RED)
        box1_stat = VGroup(stat1_wrg, stat1_loss).arrange(DOWN, buff=0.04).move_to(card1.get_bottom() + UP * 0.4)
        col1 = VGroup(card1, hdr1, cube1, arr_in1, lbl_in1, arr_out1, lbl_out1, box1_stat)

        # Panel 2 — Zentrale WRG
        card2 = RoundedRectangle(
            width=col_w, height=col_h, corner_radius=0.15,
            color=P_BLUE, fill_color="#101825", fill_opacity=0.85,
        ).move_to(pos2)
        t2_main = Text("2. ZENTRALE WRG", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        t2_sub = Text("Zentrales Lüftungsgerät", font_size=LABEL_FONT_SIZE, color=GREY_A)
        hdr2 = VGroup(t2_main, t2_sub).arrange(DOWN, buff=0.05).move_to(card2.get_top() + DOWN * 0.45)

        cube2 = create_cube_graphic(pos2 + DOWN * 0.1)
        mvhr_box = RoundedRectangle(
            width=0.8, height=0.45, corner_radius=0.05,
            color=P_WHITE, fill_color="#2C3E50", fill_opacity=0.95, stroke_width=2,
        ).move_to(pos2 + UP * 1.05)
        lbl_mvhr = Text("WRG\nZentral", font_size=LABEL_FONT_SIZE, color=P_YELLOW).scale(0.65).move_to(mvhr_box)
        arr_fresh = CurvedArrow(
            pos2 + LEFT * 1.7 + UP * 1.4, mvhr_box.get_left() + UP * 0.1,
            radius=1.3, color=P_BLUE, stroke_width=3,
        )
        arr_exhaust = CurvedArrow(
            mvhr_box.get_right() + UP * 0.1, pos2 + RIGHT * 1.7 + UP * 1.4,
            radius=-1.3, color=P_BLUE, stroke_width=3,
        )
        arr_supply = Arrow(
            mvhr_box.get_bottom(), pos2 + UP * 0.3,
            color=P_GREEN, buff=0, stroke_width=3.5,
            max_tip_length_to_length_ratio=0.35,
        )
        lbl_supply = Text("Vorgewärmte Zuluft (+18°C)", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        lbl_supply.scale(0.75).next_to(arr_supply, RIGHT, buff=0.04)
        stat2_wrg = Text("WRG: 85 – 95%", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        stat2_loss = Text("Lüftungsverlust: ~10% (Gering)", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        box2_stat = VGroup(stat2_wrg, stat2_loss).arrange(DOWN, buff=0.04).move_to(card2.get_bottom() + UP * 0.4)
        col2 = VGroup(
            card2, hdr2, cube2, mvhr_box, lbl_mvhr,
            arr_fresh, arr_exhaust, arr_supply, lbl_supply, box2_stat,
        )

        # Panel 3 — Dezentrale WRG
        card3 = RoundedRectangle(
            width=col_w, height=col_h, corner_radius=0.15,
            color=P_GREEN, fill_color="#102018", fill_opacity=0.85,
        ).move_to(pos3)
        t3_main = Text("3. DEZENTRALE WRG", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        t3_sub = Text("Pendellüfter mit Keramik", font_size=LABEL_FONT_SIZE, color=GREY_A)
        hdr3 = VGroup(t3_main, t3_sub).arrange(DOWN, buff=0.05).move_to(card3.get_top() + DOWN * 0.45)

        cube3 = create_cube_graphic(pos3 + DOWN * 0.1)
        center_p3 = pos3 + DOWN * 0.1
        w_top_l = center_p3 + RIGHT * 0.17 + DOWN * 0.3975
        w_top_r = center_p3 + RIGHT * 0.595 + DOWN * 0.2225
        w_bot_r = center_p3 + RIGHT * 0.595 + DOWN * 0.605
        w_bot_l = center_p3 + RIGHT * 0.17 + DOWN * 0.78
        win_poly = Polygon(
            w_top_l, w_top_r, w_bot_r, w_bot_l,
            color=P_CYAN, fill_color="#0284C7", fill_opacity=0.5, stroke_width=1.5,
        )
        win_line_v = Line(
            center_p3 + RIGHT * 0.3825 + DOWN * 0.31,
            center_p3 + RIGHT * 0.3825 + DOWN * 0.6925,
            color=P_WHITE, stroke_width=1,
        )
        win_line_h = Line(
            center_p3 + RIGHT * 0.17 + DOWN * 0.58875,
            center_p3 + RIGHT * 0.595 + DOWN * 0.41375,
            color=P_WHITE, stroke_width=1,
        )
        window3 = VGroup(win_poly, win_line_v, win_line_h)

        c_top_l = center_p3 + RIGHT * 0.17 + DOWN * 0.2275
        c_top_r = center_p3 + RIGHT * 0.595 + DOWN * 0.0525
        c_bot_r = center_p3 + RIGHT * 0.595 + DOWN * 0.18
        c_bot_l = center_p3 + RIGHT * 0.17 + DOWN * 0.355
        unit_face = Polygon(
            c_top_l, c_top_r, c_bot_r, c_bot_l,
            color=P_YELLOW, fill_color=P_ORANGE, fill_opacity=0.9, stroke_width=1.5,
        )

        arr_cyc1 = CurvedArrow(
            center_p3 + RIGHT * 0.3825 + DOWN * 0.2, pos3 + RIGHT * 1.6 + UP * 0.6,
            radius=1.0, color=P_ORANGE, stroke_width=3,
        )
        lbl_cyc1 = Text("70s Abluft\n(Speichern)", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        lbl_cyc1.scale(0.65).next_to(arr_cyc1, UP, buff=0.02)
        arr_cyc2 = CurvedArrow(
            pos3 + RIGHT * 1.6 + DOWN * 0.2, center_p3 + RIGHT * 0.3825 + DOWN * 0.2,
            radius=-1.0, color=P_GREEN, stroke_width=3,
        )
        lbl_cyc2 = Text("70s Zuluft\n(+17°C)", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        lbl_cyc2.scale(0.65).next_to(arr_cyc2, DOWN, buff=0.02).shift(RIGHT * 0.35)

        stat3_wrg = Text("WRG: 70 – 90%", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        stat3_loss = Text("Lüftungsverlust: ~15 – 30%", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        box3_stat = VGroup(stat3_wrg, stat3_loss).arrange(DOWN, buff=0.04).move_to(card3.get_bottom() + UP * 0.4)
        col3 = VGroup(
            card3, hdr3, cube3, window3, unit_face,
            arr_cyc1, lbl_cyc1, arr_cyc2, lbl_cyc2, box3_stat,
        )

        columns = VGroup(col1, col2, col3).scale(0.85).shift(UP * 0.35)

        self.play(FadeIn(col1), run_time=1.3)
        hold_for(self, self.NARRATION, "window", used=1.3 + 0.3)

        self.play(FadeIn(col2), run_time=1.3)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "central"))
        hold_for(self, self.NARRATION, "central", used=1.3 + 0.35)

        self.play(FadeIn(col3), run_time=1.3)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "decentral"))
        hold_for(self, self.NARRATION, "decentral", used=1.3 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
