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
    chip, note_line, card_grid, stat_card, side_labels, fit_band,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Der Energieausweis"

BEDARF_CYAN = "#38BDF8"
VERBRAUCH_ORANGE = "#FB923C"
PRIM_VIOLET = "#A78BFA"
END_RED = "#DC2626"
AMBIENT_GREEN = "#22C55E"

# The A+ … H efficiency classes with their upper Endenergie bound in kWh/(m²·a),
# per GEG Anlage 10 — each class is "up to and including" its bound, and H has
# no upper bound at all (not a printed 320). Same nine colours on every beat.
CLASSES = (
    ("A+", 30, "#1D7A3E"), ("A", 50, "#3FA34D"), ("B", 75, "#8DC63F"),
    ("C", 100, "#C7D93A"), ("D", 130, "#F9E547"), ("E", 160, "#F5A623"),
    ("F", 200, "#E8622C"), ("G", 250, "#D0342C"), ("H", None, "#A61C1C"),
)

# The two designs Chapters 4 and 5 calculated, landing on this scale.
# Final energy exactly as displayed in ``4_system_losses/scene_4.py`` (round(Q_E_GAS),
# round(Q_E_WP)); the letters below are looked up from these, never typed.
GAS_END, WP_END = 140, 31


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _document(center, *, color, width=2.20, height=0.95, lines=3):
    """📄 Certificate card — header band plus ruled body lines."""
    center = np.array(center, dtype=float)
    card = RoundedRectangle(corner_radius=0.09, width=width, height=height,
                            color=color, stroke_width=2.4).move_to(center)
    header = Rectangle(width=width - 0.28, height=0.20, color=color,
                       fill_color=color, fill_opacity=0.55, stroke_width=0)
    header.move_to(card.get_top() + DOWN * 0.26)
    # Absolute y, not an offset added onto card.get_left() — that double-counted
    # the card's own height and threw the rules out of the card entirely.
    rules = VGroup(*[
        Line(np.array([card.get_left()[0] + 0.20, y, 0]),
             np.array([card.get_right()[0] - 0.20, y, 0]),
             color=color, stroke_width=1.1, stroke_opacity=0.45)
        for y in np.linspace(center[1] - height * 0.06, center[1] - height * 0.34, lines)
    ])
    return VGroup(card, header, rules)


def _class_strip(center, *, width=11.0, height=0.62):
    """🌈 Horizontal A+ → H efficiency band with one letter per class.

    Equal-width classes: the printed certificate's bands are not linear in
    kWh either, and equal widths are the only way nine letters stay readable.
    """
    center = np.array(center, dtype=float)
    seg_w = width / len(CLASSES)
    bands = VGroup()
    for i, (letter, _bound, colour) in enumerate(CLASSES):
        rect = Rectangle(width=seg_w, height=height, color=colour,
                         fill_color=colour, fill_opacity=0.92, stroke_width=1.0)
        rect.move_to(center + RIGHT * (-width / 2 + seg_w * (i + 0.5)))
        text = body_text(letter, font_size=LABEL_FONT_SIZE - 2, color=P_DEEP_DARK)
        text.move_to(rect.get_center())
        bands.add(VGroup(rect, text))
    return {"bands": bands, "center": center, "width": width, "height": height, "seg_w": seg_w}


def _class_index(value: float) -> int:
    """🔢 Which efficiency class an Endenergie value falls into.

    Each class is "up to and including" its bound (GEG Anlage 10) — a value
    sitting exactly on 30, 50, 75 … belongs to the lower-lettered class, not
    the next one up, so this must be ``<=``, not ``<``.
    """
    for i, (_letter, bound, _colour) in enumerate(CLASSES):
        if bound is None or value <= bound:
            return i
    return len(CLASSES) - 1


def _class_letter(value: float) -> str:
    """🔤 The efficiency-class letter an Endenergie value earns."""
    return CLASSES[_class_index(value)][0]


def _class_x(strip, value: float) -> float:
    """📍 Screen x of a value inside its class band."""
    index = _class_index(value)
    lower = 0 if index == 0 else CLASSES[index - 1][1]
    upper = CLASSES[index][1]
    if upper is None:
        frac = 0.5  # H has no upper bound — no ratio to place it by, so centre it.
    else:
        frac = 0.5 if upper <= lower else (value - lower) / (upper - lower)
    left = strip["center"][0] - strip["width"] / 2 + strip["seg_w"] * index
    return float(left + strip["seg_w"] * min(max(frac, 0.12), 0.88))
#endregion


#region Beat 1 – Bedarf oder Verbrauch

class Beat1_BedarfVsVerbrauch(Scene):
    NARRATION = [
        ("intro",
         "Everything we calculated ends up on one sheet of paper that buyers and tenants actually get to see.",
         "Alles, was wir berechnet haben, landet auf einem Blatt Papier, das Käufer und Mieter wirklich zu sehen bekommen."),
        ("bedarf",
         "The demand certificate is exactly this series: a standardised calculation with fixed boundary conditions.",
         "Der Bedarfsausweis ist genau diese Serie: eine standardisierte Berechnung mit festgelegten Randbedingungen."),
        ("verbrauch",
         "The consumption certificate instead uses at least thirty-six months of recorded data, corrected for the weather.",
         "Der Verbrauchsausweis nutzt stattdessen mindestens sechsunddreißig Monate erfasste Verbrauchsdaten, witterungsbereinigt."),
        ("unterschied",
         "So one describes the building under those fixed conditions, the other the building and its actual use together.",
         "Der eine beschreibt also das Gebäude unter festen Randbedingungen, der andere Gebäude und tatsächliche Nutzung zusammen."),
        ("wann",
         "New buildings always need the calculated certificate; for existing buildings there is usually a choice, with exceptions.",
         "Für Neubauten ist der berechnete Ausweis Pflicht — im Bestand besteht meist die Wahl, mit Ausnahmen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Bedarf oder Verbrauch", title)
        din = _din_ref("GEG § 79 ff. · § 80")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        left_doc = _document([-3.55, 1.52, 0], color=BEDARF_CYAN)
        right_doc = _document([3.55, 1.52, 0], color=VERBRAUCH_ORANGE)
        self.play(Create(left_doc), Create(right_doc), run_time=1.3)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.3)

        # —— Demand certificate ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf"))
        left_cards = card_grid([
            ("Bedarfsausweis", "berechnet", BEDARF_CYAN),
            ("Eingang", "Geometrie · U-Werte · Anlage", BEDARF_CYAN),
            ("Randbedingungen", "festgelegt", BEDARF_CYAN),
        ], rows=3, buff=0.12)
        left_cards.move_to(np.array([-3.55, -0.92, 0]))
        self.play(FadeIn(left_cards, shift=UP * 0.12), run_time=1.3)
        hold_for(self, self.NARRATION, "bedarf", used=1.3 + 0.35)

        # —— Consumption certificate ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verbrauch"))
        right_cards = card_grid([
            ("Verbrauchsausweis", "gemessen", VERBRAUCH_ORANGE),
            ("Eingang", "Verbrauchsdaten · ≥ 36 Monate", VERBRAUCH_ORANGE),
            ("Randbedingungen", "real, witterungsbereinigt", VERBRAUCH_ORANGE),
        ], rows=3, buff=0.12)
        right_cards.move_to(np.array([3.55, -0.92, 0]))
        self.play(FadeIn(right_cards, shift=UP * 0.12), run_time=1.3)
        hold_for(self, self.NARRATION, "verbrauch", used=1.3 + 0.35)

        # —— What they each describe ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unterschied"))
        who = VGroup(
            body_text("beschreibt das Gebäude (feste Randbedingungen)",
                     font_size=LABEL_FONT_SIZE - 2, color=BEDARF_CYAN)
            .move_to(np.array([-3.55, 0.78, 0])),
            body_text("beschreibt Gebäude + tatsächliche Nutzung", font_size=LABEL_FONT_SIZE - 2, color=VERBRAUCH_ORANGE)
            .move_to(np.array([3.55, 0.78, 0])),
        )
        self.play(FadeIn(who), run_time=1.0)
        hold_for(self, self.NARRATION, "unterschied", used=1.0 + 0.35)

        # —— When the calculated one is mandatory ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wann"))
        rule = note_line("Neubau: stets Bedarfsausweis · Bestand: meist Wahlrecht, mit Ausnahmen (GEG § 80)", color=P_TEAL)
        self.play(FadeIn(rule), left_doc.animate.set_stroke(width=4), run_time=1.0)
        hold_for(self, self.NARRATION, "wann", used=1.0 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(rule), FadeOut(who), FadeOut(left_doc), FadeOut(right_doc),
            FadeOut(left_cards), FadeOut(right_cards), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Die Skala lesen

class Beat2_DieSkala(Scene):
    NARRATION = [
        ("intro",
         "For residential buildings, both certificate types print their result on the same colour band, from A plus down to H.",
         "Bei Wohngebäuden landet das Ergebnis beider Ausweistypen auf demselben Farbband — von A-plus bis H."),
        ("werte",
         "Two numbers stand beside it: final energy and primary energy — but for residential buildings the class follows final energy.",
         "Zwei Zahlen stehen daneben: Endenergie und Primärenergie — die Effizienzklasse richtet sich bei Wohngebäuden nach der Endenergie."),
        ("gas",
         "Our gas-heated design needs a hundred and forty kilowatt-hours per square metre — that lands in class E.",
         "Unser gasbeheizter Entwurf braucht hundertvierzig Kilowattstunden je Quadratmeter — das landet in Klasse E."),
        ("wp",
         "The identical building with a heat pump needs thirty-one, and moves up to class A.",
         "Dasselbe Gebäude mit Wärmepumpe braucht einunddreißig — und rückt auf Klasse A."),
        ("lesen",
         "Same walls, same windows, same useful-heat demand. The plant moves the letter too, not the envelope alone.",
         "Gleiche Wände, gleiche Fenster, gleicher Nutzwärmebedarf. Den Buchstaben bestimmt die Anlage mit, nicht die Hülle allein."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Skala lesen", title)
        din = _din_ref("GEG § 86 · Anlage 10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        strip = _class_strip([0.0, -0.65, 0], width=11.0, height=0.62)
        axis_note = body_text("Endenergie  kWh/(m²·a)", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE)
        axis_note.set_opacity(0.7)
        axis_note.next_to(strip["bands"], DOWN, buff=0.54)
        bounds = VGroup(*[
            body_text(str(bound), font_size=LABEL_FONT_SIZE - 5, color=P_WHITE).set_opacity(0.55)
            .move_to(np.array([strip["center"][0] - strip["width"] / 2 + strip["seg_w"] * (i + 1),
                               strip["center"][1] - strip["height"] / 2 - 0.22, 0]))
            for i, (_l, bound, _c) in enumerate(CLASSES[:-1])
        ])
        self.play(
            LaggedStart(*[FadeIn(b, shift=UP * 0.1) for b in strip["bands"]], lag_ratio=0.07),
            run_time=1.7,
        )
        schematic_note = note_line("Klassenbreiten schematisch, nicht maßstäblich", color=P_TEAL)
        self.play(FadeIn(bounds), FadeIn(axis_note), FadeIn(schematic_note), run_time=0.7)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.7 + 0.7)

        # —— The two printed numbers ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "werte"))
        legend = card_grid([
            ("Endenergie", "bestimmt die Effizienzklasse", END_RED),
            ("Primärenergie", "zusätzlich im Ausweis", PRIM_VIOLET),
        ], rows=1, buff=0.36)
        legend.move_to(np.array([0.0, 1.85, 0]))
        self.play(FadeOut(schematic_note), FadeIn(legend, shift=DOWN * 0.1), run_time=1.2)
        hold_for(self, self.NARRATION, "werte", used=1.2 + 0.35)

        # —— The gas design ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gas"))
        gas_x = _class_x(strip, GAS_END)
        gas_card = stat_card("Gaskessel", f"{GAS_END} kWh/(m²·a)  →  {_class_letter(GAS_END)}", color=P_ORANGE)
        gas_card.move_to(np.array([2.55, 0.85, 0]))
        gas_pointer = Arrow(gas_card.get_bottom(), np.array([gas_x, strip["center"][1] + 0.34, 0]),
                            buff=0.06, color=P_ORANGE, stroke_width=3,
                            max_tip_length_to_length_ratio=0.22)
        self.play(FadeIn(gas_card), GrowArrow(gas_pointer), run_time=1.3)
        hold_for(self, self.NARRATION, "gas", used=1.3 + 0.35)

        # —— The heat pump design ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wp"))
        wp_x = _class_x(strip, WP_END)
        wp_card = stat_card("Wärmepumpe", f"{WP_END} kWh/(m²·a)  →  {_class_letter(WP_END)}", color=AMBIENT_GREEN)
        wp_card.move_to(np.array([-3.30, 0.85, 0]))
        wp_pointer = Arrow(wp_card.get_bottom(), np.array([wp_x, strip["center"][1] + 0.34, 0]),
                           buff=0.06, color=AMBIENT_GREEN, stroke_width=3,
                           max_tip_length_to_length_ratio=0.22)
        self.play(FadeIn(wp_card), GrowArrow(wp_pointer), run_time=1.3)
        hold_for(self, self.NARRATION, "wp", used=1.3 + 0.35)

        # —— What that means ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lesen"))
        row, items = equation_row([
            ("same", "gleiches Gebäude, gleicher Q_h", P_WHITE), (None, "→", P_WHITE),
            ("classes", f"{_class_letter(GAS_END)} oder {_class_letter(WP_END)}", P_YELLOW),
            (None, "  bestimmt die Anlagentechnik mit", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(Create(box), FadeIn(row), run_time=1.1)
        hold_for(self, self.NARRATION, "lesen", used=1.1 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(legend), FadeOut(gas_card),
            FadeOut(wp_card), FadeOut(gas_pointer), FadeOut(wp_pointer), FadeOut(bounds),
            FadeOut(axis_note), FadeOut(strip["bands"]), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Was der Ausweis auslöst

class Beat3_PflichtangabenUndFolgen(Scene):
    NARRATION = [
        ("intro",
         "The certificate is not a formality. Once one exists, its key entries have to appear in commercial property ads.",
         "Der Ausweis ist keine Formalie. Liegt einer vor, gehören seine Kernangaben in kommerzielle Immobilienanzeigen."),
        ("angaben",
         "The type of certificate, the final energy value, the main energy carrier — and for residential buildings the year of construction.",
         "Die Ausweisart, der Endenergiekennwert, der wesentliche Energieträger — und bei Wohngebäuden das Baujahr."),
        ("klasse",
         "For residential buildings the efficiency class is required as well — that single letter travels furthest.",
         "Bei Wohngebäuden zusätzlich die Effizienzklasse — dieser eine Buchstabe wirkt am weitesten."),
        ("folgen",
         "It lets buyers and tenants roughly compare buildings — on its own it is not enough for exact costs or renovation effort.",
         "Er lässt Käufer und Mieter Gebäude überschlägig vergleichen — für genaue Kosten oder Sanierungsaufwand reicht er allein nicht."),
        ("empfehlung",
         "For existing buildings it also contains modernisation recommendations — or the statement that none are reasonably possible.",
         "Bei Bestandsgebäuden enthält er zudem Modernisierungsempfehlungen — oder den Hinweis, dass keine sinnvoll möglich sind."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Was der Ausweis auslöst", title)
        din = _din_ref("GEG § 84 · § 87")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # —— The advertisement ——
        listing = RoundedRectangle(corner_radius=0.10, width=5.6, height=3.15,
                                   color=P_TEAL, stroke_width=2.4)
        listing.move_to(np.array([-3.10, 0.20, 0]))
        listing_head = body_text("Pflichtangaben nach GEG § 87",
                                 font_size=LABEL_FONT_SIZE - 2, color=P_TEAL)
        listing_head.next_to(listing.get_top(), DOWN, buff=0.22)
        self.play(Create(listing), FadeIn(listing_head), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2)

        # —— The mandatory entries ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "angaben"))
        entries = (
            ("Ausweisart", "Bedarfsausweis", P_CYAN),
            ("Endenergiebedarf/-verbrauch", f"{GAS_END} kWh/(m²·a)", END_RED),
            ("wesentlicher Energieträger", "Erdgas", P_ORANGE),
            ("Baujahr (Wohngebäude)", "1996", P_WHITE),
        )
        rows = VGroup()
        for i, (key, value, colour) in enumerate(entries):
            y = 0.92 - i * 0.55
            key_text = body_text(key, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE)
            key_text.set_opacity(0.75)
            key_text.move_to(np.array([-5.55, y, 0])).align_to(np.array([-5.55, y, 0]), LEFT)
            value_text = body_text(value, font_size=LABEL_FONT_SIZE - 2, color=colour)
            value_text.move_to(np.array([-0.85, y, 0])).align_to(np.array([-0.85, y, 0]), RIGHT)
            rows.add(VGroup(key_text, value_text))
        self.play(LaggedStart(*[FadeIn(r, shift=RIGHT * 0.1) for r in rows], lag_ratio=0.18),
                  run_time=1.8)
        hold_for(self, self.NARRATION, "angaben", used=1.8 + 0.35)

        # —— The class ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "klasse"))
        badge = Rectangle(width=0.8, height=0.8, color=CLASSES[_class_index(GAS_END)][2],
                          fill_color=CLASSES[_class_index(GAS_END)][2], fill_opacity=0.92,
                          stroke_width=1.2)
        badge.move_to(np.array([-3.10, -2.00, 0]))
        badge_letter = body_text(CLASSES[_class_index(GAS_END)][0],
                                 font_size=FORMULA_FONT_SIZE, color=P_DEEP_DARK)
        badge_letter.move_to(badge.get_center())
        badge_tag = body_text("Effizienzklasse", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE)
        badge_tag.set_opacity(0.75).next_to(badge, LEFT, buff=0.25)
        self.play(FadeIn(badge, scale=0.7), FadeIn(badge_letter), FadeIn(badge_tag), run_time=1.1)
        hold_for(self, self.NARRATION, "klasse", used=1.1 + 0.35)

        # —— Who reads it ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "folgen"))
        readers = card_grid([
            ("Käufer", "energetische Qualität vergleichen", P_ORANGE),
            ("Mieter", "energetische Qualität vergleichen", P_YELLOW),
        ], rows=2, buff=0.22)
        readers.move_to(np.array([3.75, 0.70, 0]))
        self.play(LaggedStart(*[FadeIn(c, shift=LEFT * 0.12) for c in readers], lag_ratio=0.18),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "folgen", used=1.6 + 0.35)

        # —— Modernisation recommendations ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "empfehlung"))
        recommend = stat_card("bei Bestandsgebäuden", "Modernisierungsempfehlungen", color=AMBIENT_GREEN)
        recommend.move_to(np.array([3.75, -1.35, 0]))
        self.play(FadeIn(recommend, shift=UP * 0.12), run_time=1.1)
        hold_for(self, self.NARRATION, "empfehlung", used=1.1 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(listing), FadeOut(listing_head), FadeOut(rows),
            FadeOut(badge), FadeOut(badge_letter), FadeOut(badge_tag), FadeOut(readers),
            FadeOut(recommend), run_time=0.6,
        )
        self.wait(0.4)

#endregion
