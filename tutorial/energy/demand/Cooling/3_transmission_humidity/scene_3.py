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
    highlight_param, math_label, math_panel, math_readout, de_num,
    house_section, sun_glyph, radiation_ray, ripples, pulse_flashes, clock_glyph, thermometer_glyph,
    droplets, open_window, smooth_path, flow_guides, flow_animation,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module titles — animated once, self.add()'ed on later beats.
TITLE_OPAQUE_DE = "Transmissionswärme: Opake Bauteile"
TITLE_VENT_DE = "Lüftungswärme & Feuchtigkeit"
TITLE_SPLIT_DE = "Sensible vs. Latente Kühlung"

# Mid-screen anchor for house/diagram content (clear of title + formula/caption).
CONTENT_CENTER = DOWN * 0.1


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``Heating/2_conduction/scene_2.py``): a dim ``P_TEAL`` footnote
    that never competes with the diagram. The formula panel sits on the
    bottom edge, so this corner is clear in every beat.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _build_house(center=ORIGIN):
    """🏠 The Physical Fundamentals section house, keyed for the opaque-envelope beats.

    ``envelope`` holds the outer wall and roof lines that heat up; ``air`` tints the interior.
    """
    h = house_section(np.array(center, dtype=float) + UP * 0.05)
    t = 0.1
    air = Polygon(
        h["bottom_left"] + RIGHT * t + UP * 0.02, h["bottom_right"] + LEFT * t + UP * 0.02,
        h["top_right"] + LEFT * t + DOWN * 0.06, h["roof_peak"] + DOWN * 0.16, h["top_left"] + RIGHT * t + DOWN * 0.06,
        stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
    )
    air.set_z_index(-1)
    return {
        "house": h["group"], "air": air, "windows": h["windows"],
        "envelope": VGroup(*h["walls"][:4], h["roof"][0]), "roof": h["roof"][0],
        "bl": h["bottom_left"], "br": h["bottom_right"], "tl": h["top_left"], "tr": h["top_right"],
        "roof_peak": h["roof_peak"], "center": h["center"],
    }


def _build_sun(sun_pos):
    """☀️ Physical Fundamentals sun glyph."""
    return sun_glyph(sun_pos).scale(0.42)


def _parallel_rays(sun_pos, targets, *, gap: float = 0.45):
    """━ Straight parallel sun rays ending on ``targets`` — one shared direction from the sun."""
    sun_pos = np.array(sun_pos, dtype=float)
    targets = [np.array(t, dtype=float) for t in targets]
    aim = np.mean(targets, axis=0) - sun_pos
    aim /= np.linalg.norm(aim)
    starts = [t - aim * (float(np.dot(t - sun_pos, aim)) - gap) for t in targets]
    return VGroup(*[radiation_ray(a, b) for a, b in zip(starts, targets)]), [[a, b] for a, b in zip(starts, targets)]

#endregion


#region Beat 1 – Transmission through opaque surfaces

class Beat1_TransmissionOpaque(Scene):
    NARRATION = [
        ("intro",
         "Next we look at the opaque envelope. In summer the heat flow reverses: hot surfaces drive heat inward.",
         "Als Nächstes die opake Hülle. Im Sommer kehrt sich der Wärmestrom um: heiße Oberflächen treiben Wärme nach innen."),
        ("sun",
         "A dark roof under midday sun absorbs solar radiation and heats the outer surface.",
         "Ein dunkles Dach absorbiert unter der Mittagssonne Strahlung und heizt die Außenfläche auf."),
        ("formula",
         "The opaque transmission load Q-dot T is U times A times the equivalent temperature difference Delta-theta eq, in watts.",
         "Die opake Transmissionslast Q-Punkt-T ist U mal A mal Delta-Theta-äquivalent — in Watt."),
        ("u",
         "U is the thermal transmittance in watts per square metre kelvin. On the design day a lower U cuts the inward heat flow.",
         "U ist der Wärmedurchgangskoeffizient in W/(m²·K). Am Auslegungstag senkt ein niedrigeres U den Wärmestrom nach innen."),
        ("a",
         "A is the opaque surface area in square metres.",
         "A ist die opake Bauteilfläche in Quadratmetern."),
        ("dt",
         "Delta-theta eq is the sol-air equivalent temperature difference — surface heating from radiation, not indoor-to-outdoor air Delta-theta.",
         "Delta-Theta-eq ist die äquivalente Temperaturdifferenz nach Sol-Air — solare Oberflächenerwärmung, nicht das Luft-Delta-Theta."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_OPAQUE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Opake Bauteile unter Sommerstrahlung", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

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

        # Only the sun-facing surfaces: right roof slope and right wall.
        targets = [
            h["roof_peak"] + (h["tr"] - h["roof_peak"]) * 0.3,
            h["roof_peak"] + (h["tr"] - h["roof_peak"]) * 0.7,
            h["tr"] + (h["br"] - h["tr"]) * 0.3,
            h["tr"] + (h["br"] - h["tr"]) * 0.65,
        ]
        rays, ray_paths = _parallel_rays(sun_pos, targets)
        slope = h["tr"] - h["roof_peak"]
        roof_in = float(np.arctan2(-slope[0], slope[1]))
        hot_spots = [(t + np.array([0.0, -0.06, 0.0]), roof_in) for t in targets[:2]] + \
                    [(t + LEFT * 0.08, PI) for t in targets[2:]]

        def surface_heat(rt):
            cyc = max(1.0, rt / 1.3)
            return [pulse_flashes(ray_paths, P_YELLOW, repeats=max(1, int(rt / 1.5)), width=4.0),
                    *[ripples([c], r_max=0.45, rings=2, color=P_RED, facing=f, cycles=cyc) for c, f in hot_spots]]

        self.add(h["air"])
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in rays], lag_ratio=0.15), run_time=1.5)
        self.play(
            opaque_borders.animate.set_color(P_RED),
            h["envelope"].animate.set_color(P_RED),
            h["air"].animate.set_fill(P_RED, opacity=0.12),
            *surface_heat(2.0),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "sun", during=surface_heat)

        row, box, items = math_panel([
            ("qt", r"\dot{Q}_{T}", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta_{eq}", P_BLUE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ])

        # U belongs to the sunlit right wall, A to the envelope band on the left —
        # two separate spots so the tokens never sit on each other.
        u_token = math_label("U", size=FORMULA_FONT_SIZE, color=P_ORANGE)
        u_token.move_to((h["tr"] + h["br"]) / 2 + LEFT * 0.42 + UP * 0.3)
        a_token = math_label("A", size=FORMULA_FONT_SIZE, color=P_CYAN)

        # A is the opaque envelope itself — shade the two walls and both roof
        # slopes as bands, not a floating box in the room.
        area_band = 0.18
        area_fill = VGroup(
            Polygon(
                h["bl"], h["tl"], h["tl"] + RIGHT * area_band, h["bl"] + RIGHT * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
            Polygon(
                h["br"], h["tr"], h["tr"] + LEFT * area_band, h["br"] + LEFT * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
            Polygon(
                h["tl"], h["roof_peak"], h["roof_peak"] + DOWN * area_band, h["tl"] + DOWN * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
            Polygon(
                h["tr"], h["roof_peak"], h["roof_peak"] + DOWN * area_band, h["tr"] + DOWN * area_band,
                fill_color=P_CYAN, fill_opacity=0.3, stroke_width=0,
            ),
        )
        a_token.move_to((h["tl"] + h["bl"]) / 2 + RIGHT * 0.45 + DOWN * 0.55)
        rest = VGroup(*[m for m in row.submobjects if m is not items["u"] and m is not items["a"]])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(LaggedStart(*[FadeIn(p) for p in area_fill], lag_ratio=0.12), run_time=0.8)
        self.play(
            ReplacementTransform(opaque_borders.copy(), u_token),
            ReplacementTransform(area_fill, a_token),
            run_time=1.4,
        )
        self.play(Create(box), FadeIn(rest), run_time=0.6)
        self.play(
            ReplacementTransform(u_token, items["u"]),
            ReplacementTransform(a_token, items["a"]),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "formula", during=surface_heat)

        for key, color in (("u", P_ORANGE), ("a", P_CYAN), ("dt", P_BLUE)):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), *surface_heat(0.5), run_time=0.5)
            hold_for(self, self.NARRATION, key, during=surface_heat)
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
         "So the thermal mass shifts the peak cooling load later — typically into the late afternoon, rarely as late as the evening.",
         "Deshalb verschiebt die Speichermasse die Spitzenkühllast nach hinten — typisch in den späten Nachmittag, selten erst in den Abend."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_OPAQUE_DE)
        self.add(title)
        subtitle = beat_subtitle("Phasenverschiebung (Time Lag)", title)
        din = _din_ref("DIN EN ISO 13786")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house(hc)
        h["envelope"].set_color(P_RED)

        row, box, items = math_panel([
            ("qt", r"\dot{Q}_{T}", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, r"\cdot", P_WHITE),
            ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE),
            ("dt", r"\Delta\theta_{eq}", P_BLUE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ])

        sun_pos = RIGHT * 4.0 + UP * 1.4
        sun_group = _build_sun(sun_pos)
        self.add(h["air"], h["house"], row, box, sun_group)
        wall_spots = [h["tl"] + (h["bl"] - h["tl"]) * f + RIGHT * 0.12 for f in (0.3, 0.7)] + \
                     [h["tr"] + (h["br"] - h["tr"]) * f + LEFT * 0.12 for f in (0.3, 0.7)]
        wall_dirs = [0.0, 0.0, PI, PI]

        def stored_heat(rt, r_max=0.35):
            cyc = max(1.0, rt / 1.4)
            return [ripples([c], r_max=r_max, rings=2, color=P_ORANGE, facing=f, cycles=cyc)
                    for c, f in zip(wall_spots, wall_dirs)]

        hold_for(self, self.NARRATION, "intro", during=stored_heat)

        clock = clock_glyph(np.array([-4.6, 1.3, 0.0]), r=0.48, color=P_ORANGE)
        clock_center = clock["center"]
        clock_group = clock["group"]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "clock"))
        self.play(Create(clock_group), run_time=1.0)

        dt_ring = highlight_param(items, "dt", color=P_ORANGE)
        self.play(Create(dt_ring), run_time=0.5)

        # Five hours on the dial: one hand turn per hour of time lag.
        self.play(
            Rotate(clock["hand"], angle=-TAU * 5, about_point=clock_center),
            sun_group.animate.shift(DOWN * 2.0 + RIGHT * 1.1).set_opacity(0.35),
            *stored_heat(3.0, r_max=0.45),
            run_time=3.0,
            rate_func=linear,
        )
        # 🕔 Five hours from noon lands at 17:00 — late afternoon, not evening.
        peak_tag = VGroup(
            Text("17 Uhr", font_size=BODY_FONT_SIZE, color=P_ORANGE),
            Text("später Nachmittag", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.06).next_to(clock_group, DOWN, buff=0.14)
        late_tag = Text("21 Uhr Abend", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        late_tag.set_opacity(0.38).next_to(peak_tag, DOWN, buff=0.08)
        self.play(FadeIn(peak_tag), FadeIn(late_tag), run_time=0.5)
        hold_for(self, self.NARRATION, "clock", during=lambda rt: stored_heat(rt, r_max=0.5))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "peak"))
        self.play(
            dt_ring.animate.set_stroke(width=4),
            h["air"].animate.set_fill(opacity=0.25), *stored_heat(1.4, r_max=0.7),
            run_time=1.4,
        )
        self.play(h["air"].animate.set_fill(opacity=0.4), *stored_heat(1.3, r_max=0.85), run_time=1.3)
        self.play(
            h["air"].animate.set_fill(opacity=0.5),
            dt_ring.animate.set_stroke(width=5, color=P_RED), *stored_heat(1.3, r_max=0.95),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "peak", during=lambda rt: stored_heat(rt, r_max=0.95))

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
         "Q-Punkt-lat ist die latente Feuchtelast — Feuchte entfernen kostet Phasenwechselenergie."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_VENT_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Luftwechsel und Feuchtigkeit", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house(hc)
        win = h["windows"][1]
        sun_pos = RIGHT * 4.3 + UP * 1.5
        sun_group = _build_sun(sun_pos)
        self.add(h["air"], h["house"])
        self.play(FadeIn(sun_group, scale=0.7), run_time=1.0)

        win_center = win["center"]
        sq_box = Square(side_length=1.3, color=P_CYAN, stroke_width=2).move_to(win_center)
        zoom_box = DashedVMobject(sq_box, num_dashes=16)
        self.play(Create(zoom_box), run_time=0.8)
        self.play(FadeOut(zoom_box), open_window(win, run_time=1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 0.8 + 1.0)

        # Warm humid outdoor air streams in through the upper half of the open sash,
        # cool room air leaves through the lower half — particles, not wavy lines.
        wx, wy = win["x"], win_center[1]
        y_in, y_out = wy + 0.12, wy - 0.12
        air_start_x = wx - 2.4
        inflow = [smooth_path([np.array([air_start_x, y_in + dy + 0.1, 0.0]), np.array([wx - 0.5, y_in + dy, 0.0]),
                               np.array([wx + 0.6, y_in + dy - 0.05, 0.0]), np.array([wx + 2.0, y_in + dy - 0.45, 0.0])])
                  for dy in (0.0, 0.08)]
        outflow = [smooth_path([np.array([wx + 2.0, y_out - 0.7 + dy, 0.0]), np.array([wx + 0.6, y_out + dy, 0.0]),
                                np.array([wx - 0.5, y_out + dy, 0.0]), np.array([air_start_x, y_out + dy - 0.1, 0.0])])
                   for dy in (0.0, -0.08)]
        moist = [smooth_path([np.array([air_start_x, y_in + 0.3, 0.0]), np.array([wx - 0.5, y_in + 0.2, 0.0]),
                              np.array([wx + 0.6, y_in + 0.14, 0.0]), np.array([wx + 2.0, y_in - 0.25, 0.0])])]
        heat_waves_in = flow_guides(inflow, P_RED, opacity=0.3)
        cold_waves_out = flow_guides(outflow, P_BLUE, opacity=0.3)
        drops = droplets(np.array([air_start_x + 0.8, y_in + 0.32, 0.0]), n=6, spread=(0.6, 0.06), seed=11)
        air_y_base_in = y_in

        def window_air(rt):
            cyc = max(1.0, rt / 1.5)
            return [flow_animation([(inflow, P_RED, P_ORANGE), (outflow, P_CYAN, P_BLUE)], waves=4, cycles=cyc),
                    flow_animation([(moist, P_BLUE)], waves=5, radius=0.05, cycles=cyc, streak=False)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flow"))
        self.play(Create(heat_waves_in), Create(cold_waves_out), FadeIn(drops, lag_ratio=0.15), run_time=1.2)
        self.play(*window_air(3.6), h["air"].animate.set_fill(P_RED, opacity=0.12), run_time=3.6)
        hold_for(self, self.NARRATION, "flow", during=window_air)

        row, box, items = math_panel([
            ("ql", r"\dot{Q}_{L}", P_WHITE), (None, "=", P_WHITE),
            ("sens", r"\dot{Q}_{sens}", P_RED), (None, "+", P_WHITE),
            ("lat", r"\dot{Q}_{lat}", P_BLUE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ], size=BODY_FONT_SIZE)
        rest = VGroup(*[
            m for m in row.submobjects
            if m is not items["sens"] and m is not items["lat"]
        ])

        # Tokens rise into the free sky left of the house, apart from each other
        # and from the streams, then morph straight into their formula slots.
        sens_tok = math_label(r"\dot{Q}_{sens}", size=BODY_FONT_SIZE, color=P_RED)
        sens_tok.move_to(np.array([air_start_x + 0.35, air_y_base_in + 0.95, 0.0]))
        lat_tok = math_label(r"\dot{Q}_{lat}", size=BODY_FONT_SIZE, color=P_BLUE)
        lat_tok.move_to(np.array([air_start_x + 1.55, air_y_base_in + 0.95, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(
            ReplacementTransform(heat_waves_in.copy(), sens_tok),
            ReplacementTransform(drops.copy(), lat_tok),
            Create(box), FadeIn(rest), *window_air(1.2),
            run_time=1.2,
        )
        self.play(
            ReplacementTransform(sens_tok, items["sens"]),
            ReplacementTransform(lat_tok, items["lat"]), *window_air(1.3),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "formula", during=window_air)

        for key, color in (("sens", P_RED), ("lat", P_BLUE)):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            stress = (flow_animation([(inflow, P_RED, P_ORANGE)], waves=8, radius=0.075, cycles=1.0) if key == "sens"
                      else flow_animation([(moist, P_BLUE)], waves=10, radius=0.06, cycles=1.0, streak=False))
            self.play(Create(ring), stress, run_time=1.4)
            hold_for(self, self.NARRATION, key, during=window_air)
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
         "Q-Punkt-sens ist Dichte mal Wärmekapazität mal Delta-Theta mal Volumenstrom — Energie zum Absenken der Temperatur."),
        ("delta_theta",
         "Delta-Theta is the temperature drop in kelvin — here from 30 to 20 degrees Celsius. Watch the thermometer fall.",
         "Delta-Theta ist die Temperaturdifferenz — hier von 30 auf 20 °C. Das Thermometer sinkt."),
        ("lat_eq",
         "Latent cooling Q-dot lat equals density times latent heat of vaporization r times Delta-x times volume flow.",
         "Q-Punkt-lat ist Dichte mal Verdampfungswärme mal Delta-x mal Volumenstrom — Energie zum Entfernen von Feuchte."),
        ("delta_x",
         "Delta-x is the absolute humidity difference. Moisture accumulates in the gauge — removing it needs massive phase-change energy.",
         "Delta-x ist die Feuchtedifferenz. Der Feuchtezeiger steigt — Feuchte entfernen kostet viel Energie."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_SPLIT_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Zwei Anteile der Lüftungslast", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        mid_y = 0.25
        lx, rx = -3.2, 3.2
        divider = Line(UP * (mid_y + 1.15), DOWN * (1.15 - mid_y), color=P_TEAL, stroke_width=2)

        left_header = Text("Sensible Last", font_size=SUBTITLE_FONT_SIZE, color=P_RED)
        left_header.move_to(np.array([lx, mid_y + 1.92, 0]))
        left_sub = Text("Temperaturabsenkung", font_size=BODY_FONT_SIZE, color=P_WHITE)
        left_sub.next_to(left_header, DOWN, buff=0.1)

        right_header = Text("Latente Feuchtigkeit", font_size=SUBTITLE_FONT_SIZE, color=P_CYAN)
        right_header.move_to(np.array([rx, mid_y + 1.92, 0]))
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
        therm = thermometer_glyph(np.array([lx, mid_y - 1.0, 0.0]), height=2.15, color=P_RED, level=1.0)
        tube = therm["group"][0]
        temp_ticks = VGroup(*[
            Line([lx - 0.28, y, 0], [lx - 0.14, y, 0], color=P_TEAL, stroke_width=2)
            for y in np.linspace(mid_y - 0.55, mid_y + 0.95, 6)
        ])
        sensible_tag = Text("Misst Lufttemperatur", font_size=LABEL_FONT_SIZE - 4, color=P_ORANGE)
        sensible_tag.move_to(np.array([lx, mid_y - 1.58, 0]))

        # Start hot (≈30 °C), then fall to 20 °C for ΔΘ.
        temp_tracker = ValueTracker(1.7)
        therm["level"].set_value(0.12 + 0.86 * temp_tracker.get_value() / 1.7)
        column = therm["column"]
        column.add_updater(lambda m: therm["level"].set_value(0.12 + 0.86 * temp_tracker.get_value() / 1.7))
        def _room_c():
            return 20 + temp_tracker.get_value() * (10 / 1.7)

        temp_label = math_readout(
            lambda: rf"{de_num(_room_c())}\,\mathrm{{°C}}",
            lambda: np.array([lx + 1.05, mid_y - 0.7 + temp_tracker.get_value(), 0.0]),
            size=BODY_FONT_SIZE, color=P_ORANGE, edge="left",
        )

        sens_row, sens_box, sens_items = math_panel([
            ("qs", r"\dot{Q}_{sens}", P_RED), (None, "=", P_WHITE),
            ("rho", r"\rho_{a}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("cp", r"c_{p,a}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("dth", "ΔΘ", P_RED), (None, r"\cdot", P_WHITE),
            ("qv", r"q_{v,R}", P_WHITE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ], size=BODY_FONT_SIZE)
        unit_sens = math_label(
            r"\rho_{a}\,[\mathrm{kg/m^{3}}] \cdot c_{p,a}\,[\mathrm{kJ/(kg\,K)}]"
            r" \cdot ΔΘ\,[\mathrm{K}] \cdot q_{v,R}\,[\mathrm{m^{3}/s}]",
            size=LABEL_FONT_SIZE - 4, color=P_TEAL,
        )
        unit_sens.next_to(sens_box, UP, buff=0.05)
        unit_sens.set_x(0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sens_eq"))
        self.play(
            FadeIn(therm["group"]), Create(temp_ticks),
            run_time=1.4,
        )
        self.play(
            FadeIn(sensible_tag), FadeIn(column), FadeIn(temp_label),
            Create(sens_box), FadeIn(sens_row), FadeIn(unit_sens),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "sens_eq", used=1.4 + 1.4 + 0.35)

        delta_read = math_readout(
            lambda: rf"\Delta Θ = {de_num(30 - _room_c())}\,\mathrm{{K}}",
            lambda: tube.get_left() + LEFT * 0.2,
            size=LABEL_FONT_SIZE, color=P_RED, edge="right",
        )
        ring_dth = highlight_param(sens_items, "dth", color=P_RED)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "delta_theta"))
        self.add(delta_read)
        self.play(Create(ring_dth), run_time=0.6)
        self.play(temp_tracker.animate.set_value(0.0), run_time=2.4)
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
        latent_tag = Text("Misst Wasserdampf", font_size=LABEL_FONT_SIZE - 4, color=P_CYAN)
        latent_tag.move_to(np.array([rx, mid_y - 1.58, 0]))
        droplet_group = droplets(np.array([rx, mid_y + 0.9, 0.0]), n=6, spread=(0.36, 0.1), seed=7)
        moist_tracker = ValueTracker(0.25)
        water_fill = always_redraw(lambda: Rectangle(
            width=1.02,
            height=max(0.05, moist_tracker.get_value() * 1.7),
            color=P_BLUE,
            fill_color=P_CYAN,
            fill_opacity=0.75,
            stroke_width=0,
        ).move_to(np.array([rx, mid_y - 0.95 + (moist_tracker.get_value() * 1.7) / 2, 0])))
        rh_label = math_readout(
            lambda: rf"{de_num(30 + moist_tracker.get_value() * 60)}\%\,\text{{r.F.}}",
            lambda: np.array([rx + 1.25, mid_y - 0.95 + moist_tracker.get_value() * 1.7, 0.0]),
            size=BODY_FONT_SIZE, color=P_CYAN, edge="left",
        )

        lat_row, lat_box, lat_items = math_panel([
            ("ql", r"\dot{Q}_{lat}", P_BLUE), (None, "=", P_WHITE),
            ("rho", r"\rho_{a}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("r", "r", P_WHITE), (None, r"\cdot", P_WHITE),
            ("dx", r"\Delta x", P_BLUE), (None, r"\cdot", P_WHITE),
            ("qv", r"q_{v,R}", P_WHITE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ], size=BODY_FONT_SIZE)
        unit_lat = math_label(
            r"\rho_{a}\,[\mathrm{kg/m^{3}}] \cdot r\,[\mathrm{kJ/kg}]"
            r" \cdot \Delta x\,[\mathrm{kg/kg}] \cdot q_{v,R}\,[\mathrm{m^{3}/s}]",
            size=LABEL_FONT_SIZE - 4, color=P_TEAL,
        )
        unit_lat.next_to(lat_box, UP, buff=0.05)
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

        delta_x = math_label(r"\Delta x", size=FORMULA_FONT_SIZE, color=P_BLUE)
        delta_x.next_to(moist_ticks, LEFT, buff=0.3)
        ring_dx = highlight_param(lat_items, "dx", color=P_BLUE)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "delta_x"))
        self.play(FadeIn(delta_x), Create(ring_dx), run_time=0.6)
        self.play(
            moist_tracker.animate.set_value(0.95),
            droplet_group.animate.shift(DOWN * 0.9).set_opacity(0.25),
            run_time=3.2,
        )
        hold_for(self, self.NARRATION, "delta_x", used=0.6 + 3.2 + 0.35)

        column.clear_updaters()
        self.play(FadeOut(ring_dx), FadeOut(caption), run_time=0.4)
        self.wait(0.5)

#endregion
