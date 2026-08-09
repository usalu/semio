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
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    solar_wave_ray, symbol_token,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent module titles — animated once, self.add()'ed on later beats.
TITLE_OPAQUE_DE = "Transmissionswärme: Opake Bauteile"
TITLE_VENT_DE = "Lüftungswärme & Feuchtigkeit"
TITLE_SPLIT_DE = "Sensible vs. Latente Kühlung"

# Mid-screen anchor for house/diagram content (clear of title + formula/caption).
CONTENT_CENTER = UP * 0.25


#region Shared visual motifs

def _build_house(center=ORIGIN):
    """🏠 Line-art house exterior (shared across opaque-transmission beats)."""
    w_width, w_height = 3.6, 2.4
    bl = center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
    br = center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
    tl = center + LEFT * (w_width / 2) + UP * (w_height / 2)
    tr = center + RIGHT * (w_width / 2) + UP * (w_height / 2)
    roof_peak = center + UP * (w_height / 2 + 1.1)

    floor_line = Line(bl + LEFT * 0.6, br + RIGHT * 0.6, color=P_TEAL, stroke_width=4)
    walls = VGroup(
        Line(bl, tl, color=P_WHITE, stroke_width=3),
        Line(br, tr, color=P_WHITE, stroke_width=3),
    )
    roof = Polygon(tl, roof_peak, tr, color=P_WHITE, stroke_width=3)

    window = Square(side_length=0.7, color=P_CYAN, stroke_width=2)
    window.move_to(center + LEFT * 0.8 + UP * 0.3)
    window_cross = VGroup(
        Line(window.get_top(), window.get_bottom(), color=P_CYAN, stroke_width=1.5),
        Line(window.get_left(), window.get_right(), color=P_CYAN, stroke_width=1.5),
    )
    window_group = VGroup(window, window_cross)

    door = Rectangle(width=0.6, height=1.0, color=P_CYAN, stroke_width=2)
    door.move_to(center + RIGHT * 0.8 + DOWN * 0.7)
    door_knob = Dot(door.get_center() + LEFT * 0.18 + DOWN * 0.05, radius=0.04, color=P_CYAN)
    door_group = VGroup(door, door_knob)

    house = VGroup(floor_line, walls, roof, window_group, door_group)
    return {
        "house": house, "floor": floor_line, "walls": walls, "roof": roof,
        "window": window, "window_group": window_group, "door_group": door_group,
        "bl": bl, "br": br, "tl": tl, "tr": tr, "roof_peak": roof_peak,
        "center": center, "w_width": w_width, "w_height": w_height,
    }


def _build_house_section(center=ORIGIN):
    """🏠 Cross-section house for ventilation / moisture beats."""
    w_width, w_height = 3.6, 2.4
    bl = center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
    br = center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
    tl = center + LEFT * (w_width / 2) + UP * (w_height / 2)
    tr = center + RIGHT * (w_width / 2) + UP * (w_height / 2)
    roof_peak = center + UP * (w_height / 2 + 1.1)
    wall_thickness = 0.25

    floor_line = Line(bl + LEFT * 0.6, br + RIGHT * 0.6, color=P_TEAL, stroke_width=4)
    win_top = tl + DOWN * 0.6
    win_bottom = tl + DOWN * 1.5
    boundary = VGroup(
        Line(bl, win_bottom, color=P_WHITE, stroke_width=6),
        Line(win_top, tl, color=P_WHITE, stroke_width=6),
        Line(tl, roof_peak, color=P_WHITE, stroke_width=6),
        Line(roof_peak, tr, color=P_WHITE, stroke_width=6),
        Line(tr, br, color=P_WHITE, stroke_width=6),
    )
    window_center = (win_top + win_bottom) / 2 + LEFT * (wall_thickness / 2)
    house = VGroup(floor_line, boundary)
    return {
        "house": house, "floor": floor_line, "walls": boundary,
        "window_center": window_center,
        "bl": bl, "br": br, "tl": tl, "tr": tr, "roof_peak": roof_peak,
        "center": center, "w_width": w_width, "w_height": w_height,
        "win_top": win_top, "win_bottom": win_bottom, "wall_thickness": wall_thickness,
    }


def _build_sun(sun_pos):
    """☀️ Layered sun disc with corona rings and burst spokes."""
    sun_core = Dot(sun_pos, radius=0.45, color=P_YELLOW)
    sun_glow = Dot(sun_pos, radius=0.7, color=P_YELLOW, fill_opacity=0.35)
    sun_ring1 = Circle(radius=0.85, color=P_YELLOW, stroke_width=2, stroke_opacity=0.6).move_to(sun_pos)
    sun_ring2 = Circle(radius=1.1, color=P_YELLOW, stroke_width=1.2, stroke_opacity=0.3).move_to(sun_pos)
    sun_burst = VGroup()
    for angle in np.linspace(0, TAU, 12, endpoint=False):
        s = sun_pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
        e = sun_pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
        sun_burst.add(Line(s, e, color=P_YELLOW, stroke_width=2))
    return VGroup(sun_glow, sun_core, sun_ring1, sun_ring2, sun_burst)

#endregion


#region Beat 1 – Transmission through opaque surfaces

class Beat1_TransmissionOpaque(Scene):
    NARRATION = [
        ("intro",
         "Next, we look outside. Just like in winter, heat travels through solid walls and roofs.",
         "Als Nächstes schauen wir nach draußen. Wie im Winter wandert Wärme durch Wände und Dächer."),
        ("sun",
         "In summer a dark roof baking under the midday sun absorbs massive energy.",
         "Im Sommer speichert ein dunkles Dach unter der Mittagssonne enorme Energie."),
        ("formula",
         "We calculate this transmission load Q-dot T as U times A times Delta-T equivalent, in watts.",
         "Die Transmissionslast Q-Punkt-T ist U mal A mal Delta-T-äquivalent — in Watt."),
        ("u",
         "U is the wall's thermal transmittance in watts per square metre kelvin — lower is better.",
         "U ist der Wärmedurchgangskoeffizient in W/(m²·K) — niedriger ist besser."),
        ("a",
         "A is the opaque surface area in square metres.",
         "A ist die opake Bauteilfläche in Quadratmetern."),
        ("dt",
         "Delta-T equivalent accounts for that extreme solar heating on the surface, in kelvin.",
         "Delta-T-äquivalent erfasst die extreme solare Aufheizung der Oberfläche — in Kelvin."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_OPAQUE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Opake Bauteile unter Sommerstrahlung", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # Content raised clear of formula panel + caption bar.
        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house(hc)
        self.play(Create(h["house"]), run_time=1.5)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.5)

        sun_pos = RIGHT * 4.0 + UP * 1.4
        sun_group = _build_sun(sun_pos)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sun"))
        self.play(FadeIn(sun_group, scale=0.7), run_time=1.2)

        opaque_left = Line(h["bl"], h["tl"], color=P_ORANGE, stroke_width=6)
        opaque_right = Line(h["br"], h["tr"], color=P_ORANGE, stroke_width=6)
        opaque_roof_l = Line(h["tl"], h["roof_peak"], color=P_ORANGE, stroke_width=6)
        opaque_roof_r = Line(h["tr"], h["roof_peak"], color=P_ORANGE, stroke_width=6)
        opaque_borders = VGroup(opaque_left, opaque_right, opaque_roof_l, opaque_roof_r)
        self.play(Create(opaque_borders), run_time=1.0)

        targets = [
            h["roof_peak"] + DOWN * 0.3,
            (h["tl"] + h["roof_peak"]) / 2,
            (h["tr"] + h["roof_peak"]) / 2,
            (h["tl"] + h["bl"]) / 2 + RIGHT * 0.1,
            (h["tr"] + h["br"]) / 2 + LEFT * 0.1,
        ]
        rays = VGroup(*[
            solar_wave_ray(sun_pos + (t - sun_pos) * 0.12, t, color=P_YELLOW, stroke_width=2.5)
            for t in targets
        ])
        self.play(LaggedStart(*[Create(r) for r in rays], lag_ratio=0.12), run_time=1.5)
        self.play(
            opaque_borders.animate.set_color(P_RED),
            h["walls"][0].animate.set_color(P_RED),
            h["walls"][1].animate.set_color(P_RED),
            h["roof"].animate.set_color(P_RED),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "sun", used=1.2 + 1.0 + 1.5 + 2.0 + 0.35)

        row, items = equation_row([
            ("qt", "Q̇_T", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, "·", P_WHITE),
            ("a", "A", P_CYAN), (None, "·", P_WHITE),
            ("dt", "ΔT_eq", P_BLUE),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row)

        u_token = symbol_token("U", color=P_ORANGE, font_size=FORMULA_FONT_SIZE)
        u_token.move_to(opaque_borders.get_center())
        a_token = symbol_token("A", color=P_CYAN, font_size=FORMULA_FONT_SIZE)
        area_proxy = Rectangle(
            width=2.2, height=1.6, color=P_ORANGE, stroke_width=3,
            fill_opacity=0.08, fill_color=P_ORANGE,
        ).move_to(h["house"].get_center() + DOWN * 0.15)
        a_token.move_to(area_proxy.get_center())

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(area_proxy), run_time=0.6)
        self.play(
            ReplacementTransform(opaque_borders.copy(), u_token),
            ReplacementTransform(area_proxy, a_token),
            run_time=1.4,
        )
        self.play(
            u_token.animate.move_to(items["u"].get_center()),
            a_token.animate.move_to(items["a"].get_center()),
            Create(box), FadeIn(row),
            run_time=1.4,
        )
        self.play(FadeOut(u_token), FadeOut(a_token), run_time=0.4)
        hold_for(self, self.NARRATION, "formula", used=0.6 + 1.4 + 1.4 + 0.4 + 0.35)

        for key, color in (("u", P_ORANGE), ("a", P_CYAN), ("dt", P_BLUE)):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), run_time=0.5)
            hold_for(self, self.NARRATION, key, used=0.5 + 0.35)
            self.play(FadeOut(ring), run_time=0.3)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 – Thermal mass & time lag

class Beat2_TimeLag(Scene):
    NARRATION = [
        ("intro",
         "Because materials like concrete and brick have high thermal mass, they store this heat.",
         "Beton und Ziegel speichern Wärme — sie haben eine hohe thermische Masse."),
        ("clock",
         "They soak it up during the day and slowly release it into the room hours later.",
         "Tagsüber nehmen sie Wärme auf und geben sie erst Stunden später an den Raum ab."),
        ("peak",
         "So your equivalent temperature difference and peak cooling demand may hit in the late evening, long after sunset.",
         "Deshalb kann die Spitzenkühllast erst am späten Abend auftreten — lange nach Sonnenuntergang."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_OPAQUE_DE)
        self.add(title)
        subtitle = beat_subtitle("Phasenverschiebung (Time Lag)", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house(hc)
        h["walls"][0].set_color(P_RED)
        h["walls"][1].set_color(P_RED)
        h["roof"].set_color(P_RED)

        row, items = equation_row([
            ("qt", "Q̇_T", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, "·", P_WHITE),
            ("a", "A", P_CYAN), (None, "·", P_WHITE),
            ("dt", "ΔT_eq", P_BLUE),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row)

        sun_pos = RIGHT * 4.0 + UP * 1.4
        sun_group = _build_sun(sun_pos)
        self.add(h["house"], row, box, sun_group)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        clock_center = LEFT * 3.2 + UP * 1.85
        clock_face = Circle(radius=0.45, color=P_WHITE, stroke_width=2).move_to(clock_center)
        hour_hand = Line(clock_center, clock_center + UP * 0.25, color=P_WHITE, stroke_width=3)
        minute_hand = Line(clock_center, clock_center + UP * 0.35, color=P_WHITE, stroke_width=2)
        ticks = VGroup()
        for i in range(12):
            angle = i * TAU / 12
            inner = clock_center + np.array([np.cos(angle) * 0.35, np.sin(angle) * 0.35, 0])
            outer = clock_center + np.array([np.cos(angle) * 0.42, np.sin(angle) * 0.42, 0])
            ticks.add(Line(inner, outer, color=P_WHITE, stroke_width=1.5))
        clock_group = VGroup(clock_face, ticks, hour_hand, minute_hand)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "clock"))
        self.play(Create(clock_group), run_time=1.0)

        dt_ring = highlight_param(items, "dt", color=P_ORANGE)
        self.play(Create(dt_ring), run_time=0.5)

        hour_tracker = ValueTracker(0)
        minute_tracker = ValueTracker(0)

        def update_hour(m):
            a = hour_tracker.get_value()
            end = clock_center + np.array([np.sin(a) * 0.25, np.cos(a) * 0.25, 0])
            m.put_start_and_end_on(clock_center, end)

        def update_minute(m):
            a = minute_tracker.get_value()
            end = clock_center + np.array([np.sin(a) * 0.35, np.cos(a) * 0.35, 0])
            m.put_start_and_end_on(clock_center, end)

        hour_hand.add_updater(update_hour)
        minute_hand.add_updater(update_minute)
        self.play(
            hour_tracker.animate.set_value(TAU * 2),
            minute_tracker.animate.set_value(TAU * 12),
            sun_group.animate.shift(DOWN * 4 + RIGHT * 1.5).set_opacity(0),
            run_time=3.0,
            rate_func=linear,
        )
        hour_hand.remove_updater(update_hour)
        minute_hand.remove_updater(update_minute)
        hold_for(self, self.NARRATION, "clock", used=1.0 + 0.5 + 3.0 + 0.35)

        house_interior_points = [
            h["bl"] + RIGHT * 0.05 + UP * 0.05,
            h["br"] + LEFT * 0.05 + UP * 0.05,
            h["tr"] + LEFT * 0.05 + DOWN * 0.05,
            h["roof_peak"] + DOWN * 0.1,
            h["tl"] + RIGHT * 0.05 + DOWN * 0.05,
        ]
        interior_heat_glow = Polygon(
            *house_interior_points, fill_color=P_RED, fill_opacity=0.0, stroke_width=0,
        )
        heat_wave_1 = Polygon(*house_interior_points, color=P_ORANGE, stroke_width=2, stroke_opacity=0.0)
        heat_wave_2 = Polygon(*house_interior_points, color=P_RED, stroke_width=1.5, stroke_opacity=0.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "peak"))
        self.add(interior_heat_glow, heat_wave_1, heat_wave_2)
        self.play(
            dt_ring.animate.set_stroke(width=4),
            interior_heat_glow.animate.set_fill(opacity=0.35),
            heat_wave_1.animate.set_stroke(opacity=0.7).scale(0.92),
            run_time=2.0,
        )
        self.play(
            interior_heat_glow.animate.set_fill(opacity=0.55, color=P_RED),
            heat_wave_1.animate.scale(0.9).set_stroke(color=P_RED, opacity=0.9),
            heat_wave_2.animate.set_stroke(opacity=0.8).scale(0.85),
            dt_ring.animate.set_stroke(width=5, color=P_RED),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "peak", used=2.0 + 2.0 + 0.35)

        self.play(FadeOut(dt_ring), FadeOut(caption), run_time=0.4)
        self.wait(0.5)

#endregion


#region Beat 3 – Ventilation heat & moisture

class Beat3_VentilationHeat(Scene):
    NARRATION = [
        ("intro",
         "Now let's examine ventilation heat and moisture through an open window.",
         "Jetzt betrachten wir Lüftungswärme und Feuchtigkeit am offenen Fenster."),
        ("flow",
         "Cool conditioned air escapes outward, while warm humid outdoor air streams inside.",
         "Kühle Zuluft entweicht nach draußen — warme, feuchte Außenluft strömt hinein."),
        ("formula",
         "The total ventilation load Q-dot L is the sum of sensible and latent loads, in watts.",
         "Die gesamte Lüftungslast Q-Punkt-L ist die Summe aus fühlbarer und latenter Last — in Watt."),
        ("sens",
         "Q-dot sens is the sensible heat that cools or heats the air temperature.",
         "Q-Punkt-sens ist die fühlbare Wärme — sie ändert die Lufttemperatur."),
        ("lat",
         "Q-dot lat is the latent humidity load — removing moisture costs phase-change energy.",
         "Q-Punkt-lat ist die latente Feuchtelasten — Feuchte entfernen kostet Phasenwechselenergie."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_VENT_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Luftwechsel und Feuchtigkeit", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house_section(hc)
        sun_pos = RIGHT * 4.0 + UP * 1.4
        sun_group = _build_sun(sun_pos)
        self.add(h["house"])
        self.play(FadeIn(sun_group, scale=0.7), run_time=1.0)

        win_center = h["window_center"]
        sq_box = Square(side_length=1.6, color=P_CYAN, stroke_width=2).move_to(win_center)
        zoom_box = DashedVMobject(sq_box, num_dashes=16)
        self.play(Create(zoom_box), run_time=1.0)
        self.play(FadeOut(zoom_box), run_time=0.8)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 1.0 + 0.8)

        air_start_x = h["win_top"][0] - 2.5
        air_end_x = hc[0] + 0.5
        air_y_base_in = h["win_top"][1] - 0.2
        heat_waves_in = VGroup()
        for i in range(3):
            y_off = (i - 1) * 0.15
            points = []
            for x in np.linspace(air_start_x, air_end_x, 35):
                y = air_y_base_in + y_off + 0.05 * np.sin(6 * (x - air_start_x))
                points.append(np.array([x, y, 0]))
            wave = VMobject(color=P_RED, stroke_width=2.5, stroke_opacity=0.85)
            wave.set_points_smoothly(points)
            heat_waves_in.add(wave)

        air_start_x_out = hc[0] + 0.5
        air_end_x_out = h["win_bottom"][0] - 2.5
        air_y_base_out = h["win_bottom"][1] + 0.2
        cold_waves_out = VGroup()
        for i in range(3):
            y_off = (i - 1) * 0.15
            points = []
            for x in np.linspace(air_start_x_out, air_end_x_out, 35):
                y = air_y_base_out + y_off + 0.05 * np.sin(6 * (x - air_start_x_out))
                points.append(np.array([x, y, 0]))
            wave = VMobject(color=P_BLUE, stroke_width=2.5, stroke_opacity=0.85)
            wave.set_points_smoothly(points)
            cold_waves_out.add(wave)

        droplets = VGroup()
        rng = np.random.default_rng(42)
        for _ in range(15):
            x = rng.uniform(air_start_x + 0.2, air_end_x - 0.2)
            y = air_y_base_in + rng.uniform(-0.25, 0.25)
            drop = Circle(radius=0.065, color=P_BLUE, fill_color=P_BLUE, fill_opacity=0.85, stroke_width=1)
            drop.move_to(np.array([x, y, 0]))
            droplets.add(drop)

        flow_arrow_in = Arrow(
            start=LEFT * 2.0 + UP * 0.1, end=RIGHT * 0.2 + UP * 0.1,
            color=P_RED, stroke_width=3, max_tip_length_to_length_ratio=0.2,
        ).move_to(win_center + UP * 0.2)
        flow_arrow_out = Arrow(
            start=RIGHT * 0.2 + DOWN * 0.1, end=LEFT * 2.0 + DOWN * 0.1,
            color=P_BLUE, stroke_width=3, max_tip_length_to_length_ratio=0.2,
        ).move_to(win_center + DOWN * 0.2)

        interior_fill = Polygon(
            h["bl"] + RIGHT * h["wall_thickness"],
            h["br"] + LEFT * h["wall_thickness"],
            h["tr"] + LEFT * h["wall_thickness"],
            h["roof_peak"] + DOWN * 0.2,
            h["tl"] + RIGHT * h["wall_thickness"],
            fill_color=P_RED, fill_opacity=0.0, stroke_width=0,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flow"))
        self.add(interior_fill)
        self.play(
            LaggedStart(*[Create(w) for w in heat_waves_in], lag_ratio=0.1),
            LaggedStart(*[Create(w) for w in cold_waves_out], lag_ratio=0.1),
            LaggedStart(*[FadeIn(d, scale=0.5) for d in droplets], lag_ratio=0.05),
            GrowArrow(flow_arrow_in), GrowArrow(flow_arrow_out),
            run_time=3.0,
        )
        self.play(interior_fill.animate.set_fill(opacity=0.35), run_time=1.5)
        hold_for(self, self.NARRATION, "flow", used=3.0 + 1.5 + 0.35)

        row, items = equation_row([
            ("ql", "Q̇_L", P_WHITE), (None, "=", P_WHITE),
            ("sens", "Q̇_sens", P_RED), (None, "+", P_WHITE),
            ("lat", "Q̇_lat", P_BLUE),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row)

        sens_tok = symbol_token("Q̇_sens", color=P_RED, font_size=BODY_FONT_SIZE)
        sens_tok.move_to(heat_waves_in.get_center())
        lat_tok = symbol_token("Q̇_lat", color=P_BLUE, font_size=BODY_FONT_SIZE)
        lat_tok.move_to(droplets.get_center())

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(
            ReplacementTransform(heat_waves_in.copy(), sens_tok),
            ReplacementTransform(droplets.copy(), lat_tok),
            run_time=1.2,
        )
        self.play(
            sens_tok.animate.move_to(items["sens"].get_center()),
            lat_tok.animate.move_to(items["lat"].get_center()),
            Create(box), FadeIn(row),
            run_time=1.2,
        )
        self.play(FadeOut(sens_tok), FadeOut(lat_tok), run_time=0.35)
        hold_for(self, self.NARRATION, "formula", used=1.2 + 1.2 + 0.35 + 0.35)

        for key, color, group in (
            ("sens", P_RED, heat_waves_in),
            ("lat", P_BLUE, droplets),
        ):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), run_time=0.45)
            if key == "sens":
                self.play(group.animate.set_stroke(opacity=1.0, width=3.5), run_time=0.6)
                self.play(group.animate.set_stroke(opacity=0.7, width=2.5), run_time=0.4)
            else:
                self.play(group.animate.set_fill(opacity=1.0), run_time=0.6)
                self.play(group.animate.set_fill(opacity=0.75), run_time=0.4)
            hold_for(self, self.NARRATION, key, used=0.45 + 1.0 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 4 – Sensible vs latent split
# Visual language from Cooling/2 Beat5_SensibleVsLatent (thermometer + water
# column) — formulas stay the ventilation-load pair from this module.

class Beat4_SensibleVsLatent(Scene):
    NARRATION = [
        ("intro",
         "Let's break the two formulas apart — sensible on the left, latent on the right.",
         "Wir trennen die Formeln: links fühlbar, rechts latent."),
        ("sens_eq",
         "Sensible cooling Q-dot sens equals air density times specific heat times Delta-Theta times volume flow.",
         "Q-Punkt-sens = ρ_a · c_p,a · ΔΘ · q_v,R — Energie zum Absenken der Temperatur."),
        ("delta_theta",
         "Delta-Theta is the temperature drop in kelvin — here from 30 to 20 degrees Celsius. Watch the thermometer fall.",
         "Delta-Theta ist die Temperaturdifferenz — hier von 30 auf 20 °C. Das Thermometer sinkt."),
        ("lat_eq",
         "Latent cooling Q-dot lat equals density times latent heat of vaporization r times Delta-x times volume flow.",
         "Q-Punkt-lat = ρ_a · r · Δx · q_v,R — Energie zum Entfernen von Feuchte."),
        ("delta_x",
         "Delta-x is the absolute humidity difference. Moisture accumulates in the gauge — removing it needs massive phase-change energy.",
         "Delta-x ist die Feuchtedifferenz. Der Feuchtezeiger steigt — Feuchte entfernen kostet viel Energie."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_SPLIT_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Zwei Anteile der Lüftungslast", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        mid_y = 0.25
        lx, rx = -3.2, 3.2
        divider = Line(UP * (mid_y + 1.15), DOWN * (1.15 - mid_y), color=P_TEAL, stroke_width=2)

        left_header = Text("Sensible Last", font_size=SUBTITLE_FONT_SIZE, color=P_RED)
        left_header.move_to(np.array([lx, mid_y + 1.75, 0]))
        left_sub = Text("Temperaturabsenkung", font_size=BODY_FONT_SIZE, color=P_WHITE)
        left_sub.next_to(left_header, DOWN, buff=0.1)

        right_header = Text("Latente Feuchtigkeit", font_size=SUBTITLE_FONT_SIZE, color=P_CYAN)
        right_header.move_to(np.array([rx, mid_y + 1.75, 0]))
        right_sub = Text("Feuchte entfernen", font_size=BODY_FONT_SIZE, color=P_WHITE)
        right_sub.next_to(right_header, DOWN, buff=0.1)

        self.play(Create(divider), run_time=0.8)
        self.play(
            FadeIn(left_header), FadeIn(left_sub),
            FadeIn(right_header), FadeIn(right_sub),
            run_time=1.0,
        )
        hold_for(
            self, self.NARRATION, "intro",
            used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 0.8 + 1.0,
        )

        # —— Sensible: thermometer (same motif as internal-gains Beat5) ——
        bulb = Circle(
            radius=0.38, color=P_RED, fill_color=P_DEEP_DARK, fill_opacity=1.0, stroke_width=3,
        )
        bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
        tube = RoundedRectangle(
            corner_radius=0.12, height=2.1, width=0.34, color=P_RED, stroke_width=3,
        )
        tube.move_to(np.array([lx, mid_y + 0.15, 0]))
        mercury_bulb = Circle(
            radius=0.35, color=P_RED, fill_color=P_RED, fill_opacity=0.9, stroke_width=0,
        )
        mercury_bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
        temp_ticks = VGroup(*[
            Line([lx - 0.28, y, 0], [lx - 0.12, y, 0], color=P_TEAL, stroke_width=2)
            for y in np.linspace(mid_y - 0.55, mid_y + 0.95, 6)
        ])
        sensible_tag = Text("Misst Lufttemperatur", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        sensible_tag.move_to(np.array([lx, mid_y - 1.55, 0]))

        # Start hot (≈30 °C), then fall to 20 °C for ΔΘ.
        temp_tracker = ValueTracker(1.7)
        column = always_redraw(lambda: Rectangle(
            width=0.22,
            height=max(0.05, temp_tracker.get_value()),
            color=P_RED,
            fill_color=P_RED,
            fill_opacity=0.9,
            stroke_width=0,
        ).move_to(np.array([lx, mid_y - 0.75 + temp_tracker.get_value() / 2, 0])))
        temp_label = always_redraw(lambda: Text(
            f"{int(20 + temp_tracker.get_value() * (10 / 1.7))}°C",
            font_size=BODY_FONT_SIZE,
            color=P_ORANGE,
        ).move_to(np.array([lx + 1.05, mid_y - 0.7 + temp_tracker.get_value(), 0])))

        sens_row, sens_items = equation_row([
            ("qs", "Q̇_sens", P_RED), (None, "=", P_WHITE),
            ("rho", "ρ_a", P_WHITE), (None, "·", P_WHITE),
            ("cp", "c_p,a", P_WHITE), (None, "·", P_WHITE),
            ("dth", "ΔΘ", P_RED), (None, "·", P_WHITE),
            ("qv", "q_v,R", P_WHITE),
            (None, "  [W]", P_WHITE),
        ], font_size=BODY_FONT_SIZE)
        sens_row, sens_box = formula_panel(sens_row)
        unit_sens = Text(
            "ρ_a [kg/m³] · c_p,a [kJ/(kg·K)] · ΔΘ [K] · q_v,R [m³/s]",
            font_size=LABEL_FONT_SIZE, color=P_TEAL,
        )
        unit_sens.next_to(sens_box, UP, buff=0.12)
        unit_sens.set_x(0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sens_eq"))
        self.play(
            Create(bulb), Create(tube), Create(temp_ticks), FadeIn(mercury_bulb),
            run_time=1.4,
        )
        self.play(
            FadeIn(sensible_tag), FadeIn(column), FadeIn(temp_label),
            Create(sens_box), FadeIn(sens_row), FadeIn(unit_sens),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "sens_eq", used=1.4 + 1.4 + 0.35)

        delta_theta = Text("ΔΘ", font_size=FORMULA_FONT_SIZE, color=P_RED)
        delta_theta.next_to(tube, LEFT, buff=0.35)
        ring_dth = highlight_param(sens_items, "dth", color=P_RED)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "delta_theta"))
        self.play(FadeIn(delta_theta), Create(ring_dth), run_time=0.6)
        self.play(temp_tracker.animate.set_value(0.25), run_time=2.4)
        hold_for(self, self.NARRATION, "delta_theta", used=0.6 + 2.4 + 0.35)
        self.play(FadeOut(ring_dth), run_time=0.25)

        # —— Latent: water / moisture column (same motif as internal-gains Beat5) ——
        container = RoundedRectangle(
            corner_radius=0.1, height=2.1, width=1.15, color=P_CYAN, stroke_width=3,
        )
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

        lat_row, lat_items = equation_row([
            ("ql", "Q̇_lat", P_BLUE), (None, "=", P_WHITE),
            ("rho", "ρ_a", P_WHITE), (None, "·", P_WHITE),
            ("r", "r", P_WHITE), (None, "·", P_WHITE),
            ("dx", "Δx", P_BLUE), (None, "·", P_WHITE),
            ("qv", "q_v,R", P_WHITE),
            (None, "  [W]", P_WHITE),
        ], font_size=BODY_FONT_SIZE)
        lat_row, lat_box = formula_panel(lat_row)
        unit_lat = Text(
            "ρ_a [kg/m³] · r [kJ/kg] · Δx [kg/kg] · q_v,R [m³/s]",
            font_size=LABEL_FONT_SIZE, color=P_TEAL,
        )
        unit_lat.next_to(lat_box, UP, buff=0.12)
        unit_lat.set_x(0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lat_eq"))
        self.play(
            FadeOut(sens_box), FadeOut(sens_row), FadeOut(unit_sens),
            Create(container), Create(moist_ticks),
            run_time=1.2,
        )
        self.play(
            FadeIn(latent_tag), FadeIn(water_fill), FadeIn(rh_label), FadeIn(droplet_group),
            FadeIn(lat_box), FadeIn(lat_row), FadeIn(unit_lat),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "lat_eq", used=1.2 + 1.4 + 0.35)

        delta_x = Text("Δx", font_size=FORMULA_FONT_SIZE, color=P_BLUE)
        delta_x.next_to(container, RIGHT, buff=0.55)
        ring_dx = highlight_param(lat_items, "dx", color=P_BLUE)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "delta_x"))
        self.play(FadeIn(delta_x), Create(ring_dx), run_time=0.6)
        self.play(
            moist_tracker.animate.set_value(0.95),
            droplet_group.animate.shift(DOWN * 0.9).set_opacity(0.25),
            run_time=3.2,
        )
        hold_for(self, self.NARRATION, "delta_x", used=0.6 + 3.2 + 0.35)

        self.play(FadeOut(ring_dx), FadeOut(caption), run_time=0.4)
        self.wait(0.5)

#endregion
