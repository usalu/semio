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

TITLE_DE = "Regelwerk & Klimadaten"

LOSS_BLUE = "#38BDF8"
SOLAR_YELLOW = "#FDE047"
ETA_GREEN = "#22C55E"
DEMAND_RED = "#EF4444"

MONTHS = ("J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D")

# One chart frame for every beat of this chapter: the plot never reaches past
# x ≈ 2.6, so the legend column at TAG_X can never be drawn over a bar.
CHART_ORIGIN = (-6.00, -1.28, 0.0)
CHART_W, CHART_H = 8.60, 2.80
TAG_X = 4.75

# Monthly shape of one heating season, normalised to the January loss. Not
# measured data — a legible stand-in whose *shape* is what the beat teaches:
# losses track the outdoor temperature, gains peak in summer, and the two
# cross twice a year.
LOSS_SHAPE = (1.00, 0.92, 0.76, 0.55, 0.34, 0.18, 0.14, 0.17, 0.33, 0.57, 0.82, 0.97)
GAIN_SHAPE = (0.17, 0.26, 0.40, 0.55, 0.69, 0.78, 0.80, 0.74, 0.58, 0.40, 0.22, 0.14)

# Monthly mean outdoor temperature and global irradiation, DWD test-reference-year
# shape for a central-German site — again the shape, not a certified data row.
THETA_E = (0.5, 1.4, 4.9, 9.4, 13.6, 16.8, 18.4, 18.0, 14.2, 9.6, 4.8, 1.6)
IRRADIATION = (0.10, 0.18, 0.33, 0.50, 0.66, 0.70, 0.68, 0.60, 0.42, 0.25, 0.12, 0.08)


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _month_ticks(axes, *, color=P_WHITE, y_offset=-0.24):
    """🗓️ Twelve single-letter month labels under an axis, evenly spaced."""
    labels = VGroup()
    for i, name in enumerate(MONTHS):
        label = body_text(name, font_size=LABEL_FONT_SIZE - 3, color=color)
        label.set_opacity(0.8)
        label.move_to(axes["pt"]((i + 0.5) / 12, 0) + UP * y_offset)
        labels.add(label)
    return labels


def _month_x(axes, index: int) -> float:
    """📍 Screen x of a month's column centre."""
    return float(axes["pt"]((index + 0.5) / 12, 0)[0])


def _column(axes, index: int, value: float, *, color, width=0.48, opacity=0.8, bottom=0.0,
            fill=True):
    """▮ One month column, drawn in axis fractions so it can never leave the frame."""
    base = axes["pt"]((index + 0.5) / 12, bottom)
    height = max(0.01, (value - bottom) * axes["y_len"])
    rect = Rectangle(width=width, height=height, color=color,
                     fill_color=color, fill_opacity=opacity if fill else 0.10,
                     stroke_width=1.0 if fill else 2.2)
    rect.move_to(base + UP * (height / 2))
    return rect


def _curve(axes, values, *, color, stroke_width=3.0, normalise=None):
    """📈 Smooth polyline through twelve monthly values."""
    if normalise is None:
        lo, hi = min(values), max(values)
        normalise = lambda v: (v - lo) / (hi - lo) if hi > lo else 0.5  # noqa: E731
    points = [axes["pt"]((i + 0.5) / 12, normalise(v)) for i, v in enumerate(values)]
    curve = VMobject(color=color, stroke_width=stroke_width)
    curve.set_points_smoothly(points)
    return curve
def _area_below(axes, values, limit, *, color, opacity=0.0, samples=145):
    """🟧 Filled area between a series and a horizontal limit, one polygon per run.

    Emitting a single polygon that walks out along the series and back along the
    limit self-overlaps wherever the two coincide, and Manim then fills the join
    as if it had area. Splitting at every crossing keeps each region honest.
    """
    xs = np.linspace(0.0, 1.0, samples)
    month_pos = np.linspace(0.5 / 12, 11.5 / 12, 12)
    heights = np.interp(xs, month_pos, list(values))
    patches = VGroup()
    run: list[tuple[float, float]] = []
    for x, h in zip(xs, heights):
        if h < limit:
            run.append((float(x), float(h)))
            continue
        if len(run) > 1:
            patches.add(_run_polygon(axes, run, limit, color, opacity))
        run = []
    if len(run) > 1:
        patches.add(_run_polygon(axes, run, limit, color, opacity))
    return patches


def _run_polygon(axes, run, limit, color, opacity):
    """🔻 One contiguous below-the-limit patch."""
    pts = [axes["pt"](x, h) for x, h in run]
    pts += [axes["pt"](x, limit) for x, _h in reversed(run)]
    return Polygon(*pts, fill_color=color, fill_opacity=opacity, stroke_width=0)

#endregion


#region Beat 1 – Vom Gesetz zur Rechenmethode

class Beat1_GEGundNormen(Scene):
    NARRATION = [
        ("intro",
         "Everything we just balanced is not a private calculation — in Germany it is prescribed by law.",
         "Alles, was wir gerade bilanziert haben, ist keine private Rechnung — in Deutschland ist sie gesetzlich vorgeschrieben."),
        ("geg",
         "The Gebäudeenergiegesetz sets the target. It caps a building's primary energy demand, but never says how to compute it.",
         "Das Gebäudeenergiegesetz setzt das Ziel: Es begrenzt den Primärenergiebedarf, sagt aber nicht, wie man ihn berechnet."),
        ("din",
         "For that it points to DIN V 18599:2018-09 — the calculation method the GEG references.",
         "Dafür verweist es auf DIN V 18599:2018-09 – die vom GEG referenzierte Berechnungsmethodik."),
        ("nachbarn",
         "Beside it sit the standards from the earlier videos: peak heating load, cooling load, and summer overheating.",
         "Daneben stehen die Normen der früheren Videos: Heizlast, Kühllast und sommerlicher Wärmeschutz."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Vom Gesetz zur Rechenmethode", title)
        din = _din_ref("GEG § 20 ff.")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        # —— Tier 1: the law ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "geg"))
        law = card_grid([("Gesetz", "GEG — Gebäudeenergiegesetz", P_TEAL)], rows=1,
                        value_size=BODY_FONT_SIZE)
        law.move_to(UP * 1.55)
        law_goal = note_line("Ziel: den Primärenergiebedarf begrenzen", color=P_TEAL)
        self.play(FadeIn(law, shift=DOWN * 0.15), FadeIn(law_goal), run_time=1.1)
        hold_for(self, self.NARRATION, "geg", used=1.1 + 0.35)

        # —— Tier 2: the method ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        method = card_grid([("Rechenverfahren", "DIN V 18599:2018-09 — Energetische Bewertung", P_CYAN)], rows=1,
                           value_size=BODY_FONT_SIZE)
        method.move_to(UP * 0.30)
        link = Arrow(law.get_bottom(), method.get_top(), buff=0.10, color=P_WHITE,
                     stroke_width=3, max_tip_length_to_length_ratio=0.22)
        # On the note band, replacing the goal line: directly under the method card
        # it sat in the gap the three spoke arrows cross and was struck through.
        revision_note = note_line("Die Normenreihe wurde 2025 als DIN/TS 18599 überarbeitet.", color=P_TEAL)
        self.play(GrowArrow(link), FadeIn(method, shift=DOWN * 0.15), run_time=1.2)
        self.play(FadeOut(law_goal), FadeIn(revision_note), run_time=0.5)
        hold_for(self, self.NARRATION, "din", used=1.2 + 0.5 + 0.35)

        # —— Tier 3: the neighbouring standards from the earlier videos ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nachbarn"))
        neighbours = card_grid([
            ("Heizlast [kW]", "DIN EN 12831-1", LOSS_BLUE),
            ("Kühllast [kW]", "VDI 2078", P_RED),
            ("Sommerlicher Wärmeschutz", "DIN 4108-2", SOLAR_YELLOW),
        ], rows=1, buff=0.30)
        neighbours.move_to(DOWN * 1.05)
        spokes = VGroup(*[
            Arrow(method.get_bottom(), card.get_top(), buff=0.10, color=P_WHITE,
                  stroke_width=2, stroke_opacity=0.6, max_tip_length_to_length_ratio=0.18)
            for card in neighbours
        ])
        self.play(
            LaggedStart(*[GrowArrow(a) for a in spokes], lag_ratio=0.18),
            LaggedStart(*[FadeIn(c, shift=DOWN * 0.12) for c in neighbours], lag_ratio=0.18),
            run_time=1.8,
        )
        hold_for(self, self.NARRATION, "nachbarn", used=1.8 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(law), FadeOut(method),
            FadeOut(revision_note), FadeOut(link), FadeOut(neighbours), FadeOut(spokes), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Warum Monat für Monat bilanziert wird

class Beat2_Monatsbilanz(Scene):
    NARRATION = [
        ("intro",
         "The standard balances month by month. Here is why an annual average would be wrong.",
         "Die Norm bilanziert Monat für Monat. Hier ist der Grund, warum ein Jahresmittel falsch wäre."),
        ("verluste",
         "These are the monthly losses — high in winter, small in summer, following the outdoor temperature.",
         "Das sind die monatlichen Verluste — im Winter hoch, im Sommer klein, entlang der Außentemperatur."),
        ("gewinne",
         "And these are the usable gains. Where green covers blue, sun and internal loads already pay the bill.",
         "Und das sind die nutzbaren Gewinne. Wo Grün das Blau deckt, zahlen Sonne und interne Lasten die Rechnung."),
        ("rest",
         "Only the red remainder has to be heated. In summer there is no heating remainder — those unused gains sit in the cooling balance instead of paying January.",
         "Nur der rote Rest muss beheizt werden. Im Sommer bleibt kein Heizrest — ungenutzte Gewinne gehören in die Kühlbilanz, nicht in den Januar."),
        ("jahr",
         "An annual average would credit that summer surplus against January, and make the demand look far too small.",
         "Ein Jahresmittel würde diesen Sommerüberschuss dem Januar gutschreiben — der Bedarf erschiene viel zu klein."),
        ("summe",
         "So the annual heating demand is the sum of twelve separate monthly balances, never one averaged one.",
         "Der Jahresheizwärmebedarf ist deshalb die Summe von zwölf getrennten Monatsbilanzen, nie eine gemittelte."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Warum Monat für Monat bilanziert wird", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = labeled_axes(CHART_ORIGIN, x_len=CHART_W, y_len=CHART_H,
                            y_label="kWh / Monat", color=P_WHITE)
        months = _month_ticks(axes)
        self.play(Create(axes["group"]), FadeIn(months), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2)

        # —— Losses ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verluste"))
        # Outline only: the fills that follow (gains, then the remainder) have to
        # read *inside* this envelope, and a summer surplus has to read as green
        # standing above it.
        loss_cols = VGroup(*[
            _column(axes, i, v, color=LOSS_BLUE, fill=False, width=0.54)
            for i, v in enumerate(LOSS_SHAPE)
        ])
        loss_tag = stat_card("Verluste", "Q_Senke", color=LOSS_BLUE)
        loss_tag.move_to(np.array([TAG_X, 1.55, 0]))
        self.play(
            LaggedStart(*[GrowFromEdge(c, DOWN) for c in loss_cols], lag_ratio=0.06),
            FadeIn(loss_tag), run_time=1.9,
        )
        hold_for(self, self.NARRATION, "verluste", used=1.9 + 0.35)

        # —— Usable gains ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gewinne"))
        gain_cols = VGroup(*[
            _column(axes, i, v, color=ETA_GREEN, opacity=0.8, width=0.42)
            for i, v in enumerate(GAIN_SHAPE)
        ])
        gain_tag = stat_card("nutzbare Gewinne", "η · Q_Quelle", color=ETA_GREEN)
        gain_tag.next_to(loss_tag, DOWN, buff=0.22).align_to(loss_tag, LEFT)
        self.play(
            LaggedStart(*[GrowFromEdge(c, DOWN) for c in gain_cols], lag_ratio=0.06),
            FadeIn(gain_tag), run_time=1.9,
        )
        hold_for(self, self.NARRATION, "gewinne", used=1.9 + 0.35)

        # —— What is left over is the demand ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rest"))
        rest_cols = VGroup()
        for i, (loss, gain) in enumerate(zip(LOSS_SHAPE, GAIN_SHAPE)):
            if loss <= gain:
                continue
            rest_cols.add(_column(axes, i, loss, color=DEMAND_RED, opacity=0.9, bottom=gain,
                                  width=0.42))
        # Where green already stands above the blue outline there is nothing left to
        # heat. That excess is marked as its own area — a diagonal tick read as a
        # stray stroke across the column.
        surplus_marks = VGroup(*[
            _column(axes, i, gain, color=P_TEAL, opacity=0.55, bottom=loss, width=0.42)
            for i, (loss, gain) in enumerate(zip(LOSS_SHAPE, GAIN_SHAPE)) if loss < gain
        ])
        rest_tag = stat_card("Rest = Bedarf", "Q_h", color=DEMAND_RED)
        rest_tag.next_to(gain_tag, DOWN, buff=0.22).align_to(loss_tag, LEFT)
        self.play(
            LaggedStart(*[FadeIn(c, shift=UP * 0.1) for c in rest_cols], lag_ratio=0.08),
            FadeIn(rest_tag), run_time=1.7,
        )
        self.play(LaggedStart(*[FadeIn(m) for m in surplus_marks], lag_ratio=0.15), run_time=0.9)
        hold_for(self, self.NARRATION, "rest", used=1.7 + 0.9 + 0.35)

        # —— Why the annual average fails ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "jahr"))
        warn = note_line("Sommerüberschuss zählt nicht für die Heizbilanz.", color=P_RED)
        self.play(
            FadeIn(warn),
            *[c.animate.set_fill(opacity=0.85) for c in surplus_marks],
            run_time=1.0,
        )
        hold_for(self, self.NARRATION, "jahr", used=1.0 + 0.35)

        # —— The annual sum ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summe"))
        self.play(FadeOut(warn), FadeOut(surplus_marks), run_time=0.4)
        row, items = equation_row([
            ("qh", "Q_h,a", DEMAND_RED), (None, "=", P_WHITE),
            ("sum", "Σ", P_WHITE), (None, "über 12 Monate", P_TEAL), (None, "  von", P_WHITE),
            ("mon", "( Q_Senke − η_h · Q_Quelle )", P_WHITE),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(
            Create(box), FadeIn(row),
            LaggedStart(*[Indicate(c, color=DEMAND_RED, scale_factor=1.05) for c in rest_cols],
                        lag_ratio=0.08),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "summe", used=2.0 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(loss_cols), FadeOut(gain_cols),
            FadeOut(rest_cols), FadeOut(loss_tag), FadeOut(gain_tag), FadeOut(rest_tag),
            FadeOut(axes["group"]), FadeOut(months), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Klimadaten: das Testreferenzjahr

class Beat3_Klimadaten(Scene):
    NARRATION = [
        ("intro",
         "Every one of those twelve balances needs two inputs from outside the building: temperature and sun.",
         "Jede dieser zwölf Bilanzen braucht zwei Eingangsgrößen von außen: Temperatur und Sonne."),
        ("temperatur",
         "The monthly mean outdoor temperature drives the losses — it is the delta theta in every transmission term.",
         "Die mittlere monatliche Außentemperatur treibt die Verluste — sie ist das Delta Theta in jedem Transmissionsterm."),
        ("strahlung",
         "The monthly irradiation per orientation drives the solar gains, which is why a south window counts more than a north one.",
         "Die monatliche Einstrahlung je Orientierung treibt die solaren Gewinne — deshalb zählt ein Südfenster mehr als ein Nordfenster."),
        ("try",
         "Both come from the German weather service as a test reference year, a synthetic but representative twelve months.",
         "Beides liefert der Deutsche Wetterdienst als Testreferenzjahr — zwölf synthetische, aber repräsentative Monate."),
        ("potsdam",
         "For certain GEG proofs a standardised reference climate is fixed, and the Potsdam reference climate zone plays the central role in it.",
         "Für bestimmte GEG-Nachweise ist ein standardisiertes Referenzklima festgelegt; die Referenzklimazone Potsdam spielt dabei eine zentrale Rolle."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Klimadaten: das Testreferenzjahr", title)
        din = _din_ref("DIN V 18599-10 · DWD TRY")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = labeled_axes(CHART_ORIGIN, x_len=CHART_W, y_len=CHART_H,
                            y_label="θ_e [°C]", color=P_WHITE)
        months = _month_ticks(axes)
        self.play(Create(axes["group"]), FadeIn(months), run_time=1.1)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.1)

        # —— Outdoor temperature ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "temperatur"))
        theta_norm = lambda v: (v - (-4.0)) / 26.0  # noqa: E731  −4 … 22 °C on the axis
        theta_curve = _curve(axes, THETA_E, color=P_BLUE, normalise=theta_norm)
        theta_dots = VGroup(*[
            Dot(axes["pt"]((i + 0.5) / 12, theta_norm(v)), radius=0.055, color=P_BLUE)
            for i, v in enumerate(THETA_E)
        ])
        theta_tag = stat_card("Außentemperatur", "θ_e [°C]", color=P_BLUE)
        theta_tag.move_to(np.array([TAG_X, 1.55, 0]))
        self.play(Create(theta_curve), run_time=1.5)
        self.play(LaggedStart(*[FadeIn(d, scale=0.6) for d in theta_dots], lag_ratio=0.05),
                  FadeIn(theta_tag), run_time=1.0)
        hold_for(self, self.NARRATION, "temperatur", used=1.5 + 1.0 + 0.35)

        # —— Solar irradiation ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "strahlung"))
        sun_cols = VGroup(*[
            _column(axes, i, v, color=SOLAR_YELLOW, opacity=0.55, width=0.42)
            for i, v in enumerate(IRRADIATION)
        ])
        sun_tag = stat_card("relative Einstrahlung", "schematische Darstellung", color=SOLAR_YELLOW)
        sun_tag.next_to(theta_tag, DOWN, buff=0.22).align_to(theta_tag, RIGHT)
        self.play(
            LaggedStart(*[GrowFromEdge(c, DOWN) for c in sun_cols], lag_ratio=0.06),
            FadeIn(sun_tag), run_time=1.8,
        )
        hold_for(self, self.NARRATION, "strahlung", used=1.8 + 0.35)

        # —— Where the numbers come from ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "try"))
        source = card_grid([("Datenquelle", "DWD · Testreferenzjahr (TRY)", P_TEAL)], rows=1,
                           value_size=BODY_FONT_SIZE)
        source.move_to(np.array([-2.3, 1.72, 0]))
        self.play(FadeIn(source, shift=DOWN * 0.12), run_time=1.0)
        hold_for(self, self.NARRATION, "try", used=1.0 + 0.35)

        # —— One reference climate for the legal proof ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "potsdam"))
        ref_note = note_line("GEG-Nachweis: standardisiertes Referenzklima — Referenzklimazone Potsdam zentral", color=P_CYAN)
        self.play(FadeIn(ref_note), run_time=0.9)
        hold_for(self, self.NARRATION, "potsdam", used=0.9 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(ref_note), FadeOut(source), FadeOut(theta_curve),
            FadeOut(theta_dots), FadeOut(sun_cols), FadeOut(theta_tag), FadeOut(sun_tag),
            FadeOut(axes["group"]), FadeOut(months), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 4 – Heizgrenze & Gradtagszahl

class Beat4_Gradtagszahl(Scene):
    NARRATION = [
        ("intro",
         "One number compresses that whole weather year into a single figure engineers actually quote.",
         "Eine Kennzahl verdichtet dieses ganze Wetterjahr zu einer Zahl, die Planer wirklich nennen."),
        ("heizgrenze",
         "Above a defined reference temperature the free gains cover the losses — a calculation convention, not a physical limit.",
         "Oberhalb einer festgelegten Referenztemperatur decken die freien Gewinne die Verluste — eine Rechenkonvention, keine physikalische Grenze."),
        ("flaeche",
         "Below it, every degree and every day counts. The area shows the principle — the real degree-day figure comes from daily values on the heating days.",
         "Darunter zählt jedes Grad und jeder Tag. Die Fläche zeigt das Prinzip — real wird die Gradtagszahl aus Tageswerten der Heiztage berechnet."),
        ("nutzen",
         "As a simplified indicator of temperature-dependent losses it makes a cold winter comparable with a mild one.",
         "Als vereinfachter Indikator temperaturabhängiger Verluste macht sie einen kalten mit einem milden Winter vergleichbar."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Heizgrenze & Gradtagszahl", title)
        din = _din_ref("VDI 3807 · DIN 4108-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = labeled_axes(CHART_ORIGIN, x_len=CHART_W, y_len=CHART_H,
                            y_label="θ_e [°C]", color=P_WHITE)
        months = _month_ticks(axes)
        self.play(Create(axes["group"]), FadeIn(months), run_time=1.1)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.1)

        theta_norm = lambda v: (v - (-4.0)) / 26.0  # noqa: E731
        theta_curve = _curve(axes, THETA_E, color=P_BLUE, normalise=theta_norm)

        # —— Heating limit ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "heizgrenze"))
        limit_f = theta_norm(15.0)
        limit = DashedLine(axes["pt"](0.0, limit_f), axes["pt"](1.0, limit_f),
                           color=P_ORANGE, stroke_width=2.5)
        limit_tag = stat_card("Referenz-Heizgrenze", "θ_HG = 15 °C", color=P_ORANGE)
        limit_tag.move_to(np.array([TAG_X, 1.55, 0]))
        self.play(Create(theta_curve), run_time=1.4)
        self.play(Create(limit), FadeIn(limit_tag), run_time=1.0)
        hold_for(self, self.NARRATION, "heizgrenze", used=1.4 + 1.0 + 0.35)

        # —— Degree-day area between curve and limit ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flaeche"))
        area = _area_below(axes, [theta_norm(v) for v in THETA_E], limit_f,
                           color=P_ORANGE, opacity=0.0)
        gt_tag = stat_card("Gradtagszahl", "G_t [Kd/a]", color=P_ORANGE)
        gt_tag.next_to(limit_tag, DOWN, buff=0.22).align_to(limit_tag, RIGHT)
        self.add(area)
        self.play(*[patch.animate.set_fill(opacity=0.32) for patch in area],
                  FadeIn(gt_tag), run_time=1.4)
        heiz_note = note_line("Prinzipdarstellung — reale Gradtagszahlen aus Tageswerten", color=P_ORANGE)
        self.play(FadeIn(heiz_note), run_time=0.7)
        hold_for(self, self.NARRATION, "flaeche", used=1.4 + 0.7 + 0.35)

        # —— What it is used for ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nutzen"))
        row, items = equation_row([
            ("q", "Q_temp.-abh.", LOSS_BLUE), (None, "~", P_WHITE),
            ("gt", "G_t", P_ORANGE), (None, "   →  vereinfachtes Prinzip der Witterungsbereinigung", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(FadeOut(heiz_note), Create(box), FadeIn(row), run_time=1.0)
        ring = highlight_param(items, "gt", color=P_ORANGE)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "nutzen", used=1.0 + 0.5 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(ring), FadeOut(box), FadeOut(row), FadeOut(area),
            FadeOut(theta_curve), FadeOut(limit), FadeOut(limit_tag), FadeOut(gt_tag),
            FadeOut(axes["group"]), FadeOut(months), run_time=0.6,
        )
        self.wait(0.4)

#endregion
