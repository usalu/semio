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
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE, body_text,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, SAFE_BOTTOM_FORMULA, CONTENT_TOP,
    chip, note_line, card_grid, stat_card, side_labels, labeled_axes, fit_band,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Bedarf vs. Leistung"

LOSS_BLUE = "#38BDF8"
DEMAND_RED = "#EF4444"
LOAD_RED = "#EF4444"
ENERGY_CYAN = "#22D3EE"
ETA_GREEN = "#22C55E"

# Sample count of the synthetic annual profile. One point per two days keeps the
# polyline light enough to Transform smoothly while still looking like weather.
PROFILE_N = 183


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs
# The whole chapter is told on one dataset: a synthetic year of heating power.
# Beat 1 reads its height and its area, Beat 3 sorts it into a load-duration
# curve, Beat 4 draws candidate plant capacities across it. Same numbers
# throughout, so the viewer is always looking at the same building.

def _annual_load(n: int = PROFILE_N, seed: int = 5) -> np.ndarray:
    """📉 Synthetic annual heating-power profile, normalised to its own peak.

    Shape only — a cosine season driving ``θ_i − θ_e`` above the heating limit
    plus weather noise. It is deliberately not measured data; what the beats
    teach is the difference between the curve's height and its area.
    """
    rng = np.random.default_rng(seed)
    t = np.linspace(0.0, 1.0, n)
    theta_e = 9.0 - 9.5 * np.cos(2 * np.pi * t) + rng.normal(0.0, 1.9, n)
    load = np.maximum(0.0, 15.0 - theta_e)
    return load / load.max()


def _polyline(axes, values, *, color, stroke_width=2.6, smooth=False):
    """📈 Draw a normalised series across the full axis width."""
    xs = np.linspace(0.0, 1.0, len(values))
    points = [axes["pt"](x, float(v)) for x, v in zip(xs, values)]
    curve = VMobject(color=color, stroke_width=stroke_width)
    if smooth:
        curve.set_points_smoothly(points)
    else:
        curve.set_points_as_corners(points)
    return curve


def _area_under(axes, values, *, color, opacity=0.28):
    """🟦 Filled area between a series and the x-axis — the energy the curve carries."""
    xs = np.linspace(0.0, 1.0, len(values))
    points = [axes["pt"](x, float(v)) for x, v in zip(xs, values)]
    points += [axes["pt"](1.0, 0.0), axes["pt"](0.0, 0.0)]
    return Polygon(*points, fill_color=color, fill_opacity=opacity, stroke_width=0)
#endregion


#region Beat 1 – Leistung ist eine Rate, Energie eine Menge

class Beat1_LeistungUndEnergie(Scene):
    NARRATION = [
        ("intro",
         "Two numbers describe every heating system, and mixing them up is the classic beginner's mistake.",
         "Zwei Zahlen beschreiben jede Heizung — sie zu verwechseln ist der klassische Anfängerfehler."),
        ("kurve",
         "This is one year of heating power for our building: high on cold days, zero all summer.",
         "Das ist ein Jahr Heizleistung unseres Gebäudes: hoch an kalten Tagen, den ganzen Sommer null."),
        ("leistung",
         "Power is the height of the curve at one moment — kilowatts, how fast energy flows right now.",
         "Leistung ist die Höhe der Kurve in einem Moment — Kilowatt, wie schnell Energie gerade fließt."),
        ("energie",
         "Energy is the area under it — kilowatt-hours, how much flowed in total over the whole year.",
         "Energie ist die Fläche darunter — Kilowattstunden, wie viel insgesamt über das Jahr geflossen ist."),
        ("formel",
         "Power times time gives energy. That single relation from the fundamentals separates the load from the demand.",
         "Leistung mal Zeit ergibt Energie. Diese eine Beziehung aus den Grundlagen trennt Last von Bedarf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Leistung ist eine Rate, Energie eine Menge", title)
        din = _din_ref("Grundlagen · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = labeled_axes([-5.4, -1.25, 0], x_len=10.1, y_len=2.80,
                            x_label="ein Jahr  →", y_label="Leistung [kW]", color=P_WHITE)
        self.play(Create(axes["group"]), run_time=1.1)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.1)

        # —— The profile ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kurve"))
        load = _annual_load()
        curve = _polyline(axes, load, color=LOAD_RED)
        self.play(Create(curve), run_time=2.2)
        hold_for(self, self.NARRATION, "kurve", used=2.2 + 0.35)

        # —— Height = power ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "leistung"))
        probe_i = 18
        probe_x = probe_i / (len(load) - 1)
        probe_y = float(load[probe_i])
        stem = DashedLine(axes["pt"](probe_x, 0.0), axes["pt"](probe_x, probe_y),
                          color=P_YELLOW, stroke_width=2.4)
        knob = Dot(axes["pt"](probe_x, probe_y), radius=0.075, color=P_YELLOW)
        power_card = stat_card("Leistung — Höhe", "P(t)  [kW]", color=P_YELLOW)
        power_card.move_to(np.array([3.5, 1.52, 0]))
        power_leader = Line(power_card.get_bottom(), knob.get_center(),
                            color=P_YELLOW, stroke_width=1.4, stroke_opacity=0.5)
        self.play(Create(stem), FadeIn(knob, scale=0.6), run_time=0.9)
        self.play(FadeIn(power_card), Create(power_leader), run_time=0.8)
        hold_for(self, self.NARRATION, "leistung", used=0.9 + 0.8 + 0.35)

        # —— Area = energy ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "energie"))
        area = _area_under(axes, load, color=ENERGY_CYAN, opacity=0.0)
        energy_card = stat_card("Energie — Fläche", "Q  [kWh/a]", color=ENERGY_CYAN)
        energy_card.next_to(power_card, DOWN, buff=0.24).align_to(power_card, LEFT)
        self.add(area)
        self.play(
            FadeOut(stem), FadeOut(knob), FadeOut(power_leader),
            area.animate.set_fill(opacity=0.30), FadeIn(energy_card), run_time=1.6,
        )
        hold_for(self, self.NARRATION, "energie", used=1.6 + 0.35)

        # —— The relation ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formel"))
        row, items = equation_row([
            ("q", "Q", ENERGY_CYAN), (None, "=", P_WHITE),
            ("sum", "Σ", P_WHITE),
            ("p", "P", P_YELLOW), (None, "·", P_WHITE),
            ("dt", "Δt", P_TEAL), (None, "      1 kW · 1 h = 1 kWh", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(Create(box), FadeIn(row), run_time=1.0)
        hold_for(self, self.NARRATION, "formel", used=1.0 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(area), FadeOut(curve),
            FadeOut(power_card), FadeOut(energy_card), FadeOut(axes["group"]), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Die Normheizlast nach DIN EN 12831-1

class Beat2_Normheizlast(Scene):
    NARRATION = [
        ("intro",
         "The peak of that curve has its own standard, its own symbol and its own rules: the design heat load.",
         "Die Spitze dieser Kurve hat eine eigene Norm, ein eigenes Formelzeichen und eigene Regeln: die Norm-Heizlast."),
        ("bedingungen",
         "It is computed for one artificial worst case: the design outdoor temperature of the site, room by room.",
         "Sie wird für einen künstlichen Extremfall berechnet: die Norm-Außentemperatur des Standorts, Raum für Raum."),
        ("keine",
         "And crucially, no solar and no internal gains are credited — a cloudy night with an empty house.",
         "Und entscheidend: Weder solare noch interne Gewinne werden angerechnet — eine bewölkte Nacht im leeren Haus."),
        ("formel",
         "So the load is simply the transmission and ventilation heat transfer coefficients times the design difference.",
         "Die Heizlast ist damit schlicht der Transmissions- und Lüftungs-Wärmetransferkoeffizient mal der Auslegungsdifferenz."),
        ("einheit",
         "The result is kilowatts, not kilowatt-hours. It sizes the boiler or the heat pump — nothing else.",
         "Das Ergebnis sind Kilowatt, nicht Kilowattstunden. Es dimensioniert Kessel oder Wärmepumpe — sonst nichts."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Norm-Heizlast nach DIN EN 12831-1", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        # —— The design conditions, as one aligned card row ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedingungen"))
        conditions = card_grid([
            ("Innentemperatur", "θ_int = 20 °C", DEMAND_RED),
            ("Norm-Außentemperatur", "θ_e = −12 °C", P_BLUE),
            ("Auslegungsdifferenz", "Δθ = 32 K", P_YELLOW),
        ], rows=1, buff=0.32)
        conditions.move_to(UP * 1.15)
        site_note = note_line("θ_e ist standortabhängig — in Deutschland etwa −10 bis −16 °C", color=P_TEAL)
        self.play(
            LaggedStart(*[FadeIn(c, shift=DOWN * 0.12) for c in conditions], lag_ratio=0.2),
            run_time=1.6,
        )
        self.play(FadeIn(site_note), run_time=0.7)
        hold_for(self, self.NARRATION, "bedingungen", used=1.6 + 0.7 + 0.35)

        # —— Gains are deliberately not credited ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "keine"))
        excluded = card_grid([
            ("solare Gewinne", "nicht angerechnet", P_ORANGE),
            ("interne Gewinne", "nicht angerechnet", P_ORANGE),
        ], rows=1, buff=0.32)
        excluded.move_to(DOWN * 0.30)
        strikes = VGroup(*[
            Line(card.get_corner(DL) + RIGHT * 0.12, card.get_corner(UR) + LEFT * 0.12,
                 color=P_RED, stroke_width=2.4, stroke_opacity=0.75)
            for card in excluded
        ])
        contrast = note_line("Gegensatz zur Jahresbilanz: dort senken dieselben Gewinne den Bedarf.",
                             color=P_ORANGE)
        self.play(
            FadeOut(site_note),
            LaggedStart(*[FadeIn(c, shift=UP * 0.12) for c in excluded], lag_ratio=0.2),
            run_time=1.3,
        )
        self.play(LaggedStart(*[Create(s) for s in strikes], lag_ratio=0.2),
                  FadeIn(contrast), run_time=1.0)
        hold_for(self, self.NARRATION, "keine", used=1.3 + 1.0 + 0.35)

        # —— The formula ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formel"))
        row, items = equation_row([
            ("phi", "Φ_HL", LOAD_RED), (None, "=", P_WHITE),
            ("h", "( H_T + H_V )", LOSS_BLUE), (None, "·", P_WHITE),
            ("dt", "Δθ", P_YELLOW), (None, "  [kW]", P_TEAL),
        ])
        row, box = formula_panel(row)
        self.play(FadeOut(contrast), Create(box), FadeIn(row), run_time=1.1)
        ring = highlight_param(items, "h", color=LOSS_BLUE)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "formel", used=1.1 + 0.5 + 0.35)

        # —— Unit reminder ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "einheit"))
        self.play(FadeOut(ring), run_time=0.3)
        unit_note = note_line("Φ_HL in kW → Anlagengröße   ·   Q_h in kWh/a → Betriebskosten", color=P_CYAN)
        self.play(FadeIn(unit_note), run_time=0.9)
        hold_for(self, self.NARRATION, "einheit", used=0.3 + 0.9 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(unit_note),
            FadeOut(conditions), FadeOut(excluded), FadeOut(strikes), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Die Jahresdauerlinie

class Beat3_Jahresdauerlinie(Scene):
    NARRATION = [
        ("intro",
         "Load and demand live on the same curve. One picture shows both at once — sort the year by size.",
         "Last und Bedarf stecken in derselben Kurve. Ein Bild zeigt beides — man sortiert das Jahr nach Größe."),
        ("sortieren",
         "Take every hour of the year and line them up from the coldest to the mildest. That is the load duration curve.",
         "Man reiht jede Stunde des Jahres von der kältesten bis zur mildesten auf. Das ist die Jahresdauerlinie."),
        ("spitze",
         "Its starting height is the design heat load — the few hours the plant must still be able to cover.",
         "Ihre Anfangshöhe ist die Norm-Heizlast — die wenigen Stunden, die die Anlage noch abdecken muss."),
        ("flaeche",
         "Its area is the annual heating demand, the kilowatt-hours the building actually consumes.",
         "Ihre Fläche ist der Jahresheizwärmebedarf — die Kilowattstunden, die das Gebäude wirklich verbraucht."),
        ("volllast",
         "Area divided by peak gives the full-load hours: a building runs at full load only a fraction of the year.",
         "Fläche geteilt durch Spitze ergibt die Vollbenutzungsstunden: Volllast läuft ein Gebäude nur einen Bruchteil des Jahres."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Jahresdauerlinie", title)
        din = _din_ref("DIN EN 12831-1 · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = labeled_axes([-5.4, -1.25, 0], x_len=10.1, y_len=2.80,
                            x_label="Stunden des Jahres, nach Größe sortiert  →",
                            y_label="Leistung [kW]", color=P_WHITE)
        load = _annual_load()
        curve = _polyline(axes, load, color=LOAD_RED)
        self.play(Create(axes["group"]), Create(curve), run_time=1.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.8)

        # —— Sort the year ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sortieren"))
        ordered = np.sort(load)[::-1]
        duration = _polyline(axes, ordered, color=LOAD_RED, stroke_width=3.0)
        self.play(Transform(curve, duration), run_time=2.6, rate_func=smooth)
        hold_for(self, self.NARRATION, "sortieren", used=2.6 + 0.35)

        # —— The peak ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "spitze"))
        peak_line = DashedLine(axes["pt"](0.0, 1.0), axes["pt"](1.0, 1.0),
                               color=LOAD_RED, stroke_width=2.2, stroke_opacity=0.8)
        peak_card = stat_card("Heizlast — Höhe", "Φ_HL  [kW]", color=LOAD_RED)
        peak_card.move_to(np.array([3.7, 1.52, 0]))
        self.play(Create(peak_line), FadeIn(peak_card), run_time=1.1)
        hold_for(self, self.NARRATION, "spitze", used=1.1 + 0.35)

        # —— The area ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flaeche"))
        area = _area_under(axes, ordered, color=ENERGY_CYAN, opacity=0.0)
        demand_card = stat_card("Heizwärmebedarf — Fläche", "Q_h  [kWh/a]", color=ENERGY_CYAN)
        demand_card.next_to(peak_card, DOWN, buff=0.24).align_to(peak_card, RIGHT)
        self.add(area)
        self.play(area.animate.set_fill(opacity=0.30), FadeIn(demand_card), run_time=1.5)
        hold_for(self, self.NARRATION, "flaeche", used=1.5 + 0.35)

        # —— Full load hours ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "volllast"))
        equiv_f = float(np.mean(ordered))
        equiv = Rectangle(
            width=equiv_f * axes["x_len"], height=axes["y_len"],
            color=P_YELLOW, stroke_width=2.2, fill_opacity=0.0,
        )
        equiv.move_to(axes["pt"](equiv_f / 2, 0.5))
        row, items = equation_row([
            ("t", "t_VL", P_YELLOW), (None, "=", P_WHITE),
            ("q", "Q_h", ENERGY_CYAN), (None, "/", P_WHITE),
            ("p", "Φ_HL", LOAD_RED), (None, "   ≈ 1 500 – 2 200 h/a", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(Create(equiv), run_time=1.2)
        self.play(Create(box), FadeIn(row), run_time=1.0)
        hold_for(self, self.NARRATION, "volllast", used=1.2 + 1.0 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(equiv), FadeOut(area),
            FadeOut(curve), FadeOut(peak_line), FadeOut(peak_card), FadeOut(demand_card),
            FadeOut(axes["group"]), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 4 – Auslegen auf die Spitze, nicht auf den Mittelwert

class Beat4_RichtigAuslegen(Scene):
    NARRATION = [
        ("intro",
         "On that same curve you can see, directly, what happens when a plant is sized wrongly.",
         "An derselben Kurve sieht man unmittelbar, was passiert, wenn eine Anlage falsch ausgelegt wird."),
        ("zuklein",
         "Size it on the average and the shaded hours at the top are simply not covered — the house cools down exactly in the cold snap.",
         "Legt man auf den Mittelwert aus, bleiben die Stunden oben ungedeckt — das Haus kühlt genau in der Kältewelle aus."),
        ("zugross",
         "Size it far above the peak and the opposite happens: for most of the year the plant is far too large and starts cycling.",
         "Legt man weit über die Spitze aus, passiert das Gegenteil: Fast das ganze Jahr ist die Anlage zu groß und beginnt zu takten."),
        ("richtig",
         "The design heat load is the sizing point — it covers the coldest hours without oversizing the rest of the year.",
         "Die Norm-Heizlast ist der Auslegungspunkt — sie deckt die kältesten Stunden, ohne den Rest des Jahres zu überdimensionieren."),
        ("brueck",
         "And the demand under the curve is what turns into a fuel bill — which is where the next chapter starts.",
         "Und die Fläche darunter wird zur Energierechnung — genau dort beginnt das nächste Kapitel."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Auslegen auf die Spitze, nicht auf den Mittelwert", title)
        din = _din_ref("DIN EN 12831-1 · VDI 4645")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = labeled_axes([-5.4, -1.25, 0], x_len=10.1, y_len=2.80,
                            x_label="Stunden des Jahres, nach Größe sortiert  →",
                            y_label="Leistung [kW]", color=P_WHITE)
        # The curve is plotted at PEAK_F of the axis height, not 1.0: this beat
        # draws a deliberately oversized plant *above* the peak, and without that
        # headroom the capacity line leaves the plot and crosses the subtitle.
        PEAK_F = 0.68
        ordered = np.sort(_annual_load())[::-1] * PEAK_F
        curve = _polyline(axes, ordered, color=LOAD_RED, stroke_width=3.0)
        self.play(Create(axes["group"]), Create(curve), run_time=1.7)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.7)

        capacity = ValueTracker(float(np.mean(ordered)))
        cap_line = always_redraw(lambda: DashedLine(
            axes["pt"](0.0, capacity.get_value()), axes["pt"](1.0, capacity.get_value()),
            color=P_GREEN, stroke_width=2.6,
        ))
        cap_tag = always_redraw(lambda: stat_card(
            "gewählte Anlagenleistung", "Φ_Anlage", color=P_GREEN,
        ).move_to(np.array([3.7, 1.52, 0])))
        self.add(cap_line, cap_tag)

        # —— Undersized ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zuklein"))
        gap_pts = [axes["pt"](x, float(v)) for x, v in
                   zip(np.linspace(0, 1, len(ordered)), ordered) if v > capacity.get_value()]
        if gap_pts:
            level = capacity.get_value()
            xs = [x for x, v in zip(np.linspace(0, 1, len(ordered)), ordered) if v > level]
            gap = Polygon(
                *[axes["pt"](x, float(v)) for x, v in zip(xs, [v for v in ordered if v > level])],
                *[axes["pt"](x, level) for x in reversed(xs)],
                fill_color=P_RED, fill_opacity=0.35, stroke_width=0,
            )
        else:
            gap = VGroup()
        under_note = note_line("ungedeckte Stunden → Komfortverlust an den kältesten Tagen", color=P_RED)
        self.play(FadeIn(gap), FadeIn(under_note), run_time=1.3)
        hold_for(self, self.NARRATION, "zuklein", used=1.3 + 0.35)

        # —— Oversized ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zugross"))
        over_note = note_line("dauerhafte Teillast → Takten, schlechtere Jahresarbeitszahl", color=P_ORANGE)
        self.play(FadeOut(gap), FadeOut(under_note), run_time=0.35)
        self.play(capacity.animate.set_value(PEAK_F * 1.42), run_time=1.6, rate_func=smooth)
        self.play(FadeIn(over_note), run_time=0.7)
        hold_for(self, self.NARRATION, "zugross", used=0.35 + 1.6 + 0.7 + 0.35)

        # —— Correct ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "richtig"))
        right_note = note_line("Auslegungspunkt: Φ_Anlage ≈ Φ_HL", color=ETA_GREEN)
        self.play(FadeOut(over_note), run_time=0.3)
        self.play(capacity.animate.set_value(PEAK_F), run_time=1.5, rate_func=smooth)
        self.play(FadeIn(right_note), run_time=0.7)
        hold_for(self, self.NARRATION, "richtig", used=0.3 + 1.5 + 0.7 + 0.35)

        # —— Bridge to the loss chain ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "brueck"))
        area = _area_under(axes, ordered, color=ENERGY_CYAN, opacity=0.0)
        self.add(area)
        row, items = equation_row([
            ("q", "Q_h", ENERGY_CYAN), (None, "  →  Endenergie  →  Primärenergie", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(FadeOut(right_note), area.animate.set_fill(opacity=0.30),
                  Create(box), FadeIn(row), run_time=1.5)
        hold_for(self, self.NARRATION, "brueck", used=1.5 + 0.35)

        self.remove(cap_line, cap_tag)
        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(area), FadeOut(curve),
            FadeOut(axes["group"]), run_time=0.6,
        )
        self.wait(0.4)

#endregion
