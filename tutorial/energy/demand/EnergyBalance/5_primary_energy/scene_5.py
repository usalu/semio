<import numpy as np
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
    chip, note_line, card_grid, stat_card, side_labels, stacked_bar, fit_band,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE  = "Primärenergie & das GEG"

NUTZ_BLUE = "#38BDF8"
END_RED = "#DC2626"
PRIM_VIOLET = "#A78BFA"
AMBIENT_GREEN = "#22C55E"
GAS_AMBER = "#F59E0B"

# The worked example carried over from Chapter 4, continued to primary energy.
Q_NUTZ = 100
Q_END_GAS, Q_END_WP = 135, 29
F_P_GAS, F_P_STROM = 1.1, 1.8
# Round half up — Python's round() is banker's rounding and would turn the
# gas case's exact 148,5 into 148, contradicting every other slide.
Q_PRIM_GAS = int(Q_END_GAS * F_P_GAS + 0.5)    # 149 kWh/(m²·a)
Q_PRIM_WP = int(Q_END_WP * F_P_STROM + 0.5)    #  52 kWh/(m²·a)
Q_REF, REF_SHARE = 100, 0.55                    # Referenzgebäude and the new-build share
Q_LIMIT = int(round(Q_REF * REF_SHARE))         #  55 kWh/(m²·a)


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _small_house(center, *, width=1.5, height=0.85, roof=0.45, color=P_WHITE):
    """🏠 The through-line house, reduced to a badge that fits inside a boundary ring."""
    center = np.array(center, dtype=float)
    bl = center + LEFT * width / 2 + DOWN * height / 2
    br = center + RIGHT * width / 2 + DOWN * height / 2
    tl = center + LEFT * width / 2 + UP * height / 2
    tr = center + RIGHT * width / 2 + UP * height / 2
    peak = center + UP * (height / 2 + roof)
    return VGroup(
        Line(bl, br, color=P_TEAL, stroke_width=3),
        Line(bl, tl, color=color, stroke_width=2.5),
        Line(tr, br, color=color, stroke_width=2.5),
        Polygon(tl, peak, tr, color=color, stroke_width=2.5),
    )


def _ring(center, *, width, height, color, dashed=True):
    """⭕ One balance boundary — the same dashed motif Chapter 1 drew around the house."""
    box = RoundedRectangle(corner_radius=0.16, width=width, height=height,
                           color=color, stroke_width=2.4)
    box.move_to(np.array(center, dtype=float))
    return DashedVMobject(box, num_dashes=44, dashed_ratio=0.55) if dashed else box


def _factor_bar(y, name, value, colour, *, x0=-1.35, scale=2.15):
    """▬ One horizontal primary-energy-factor bar with its carrier name and value."""
    length = max(0.04, value * scale)
    bar = Rectangle(width=length, height=0.42, color=colour,
                    fill_color=colour, fill_opacity=0.78, stroke_width=1.2)
    bar.move_to(np.array([x0 + length / 2, y, 0]))
    label = body_text(name, font_size=LABEL_FONT_SIZE, color=colour)
    label.move_to(np.array([x0 - 0.22, y, 0])).align_to(np.array([x0 - 0.22, y, 0]), RIGHT)
    value_text = body_text(f"f_p = {value:.1f}".replace(".", ","),
                           font_size=LABEL_FONT_SIZE, color=colour)
    value_text.next_to(bar, RIGHT, buff=0.20)
    return {"bar": bar, "label": label, "value": value_text,
            "group": VGroup(bar, label, value_text)}
#endregion


#region Beat 1 – Drei Bilanzgrenzen

class Beat1_DreiBilanzgrenzen(Scene):
    NARRATION = [
        ("intro",
         "Everything in this series has depended on one decision: where we draw the balance boundary.",
         "Alles in dieser Serie hing an einer Entscheidung: wo wir die Bilanzgrenze ziehen."),
        ("raum",
         "Draw it around the room and you count useful energy — the heat the balance said the room needs.",
         "Zieht man sie um den Raum, zählt man Nutzenergie — die Wärme, die die Bilanz für den Raum verlangt."),
        ("gebaeude",
         "Draw it around the building at the meter and you count final energy, plant losses included.",
         "Zieht man sie am Zähler um das Gebäude, zählt man Endenergie — samt Anlagenverlusten."),
        ("quelle",
         "Draw it all the way back to the source and you count primary energy: extraction, conversion and transport as well.",
         "Zieht man sie bis zur Quelle, zählt man Primärenergie — Gewinnung, Umwandlung und Transport inklusive."),
        ("geg",
         "And primary energy is the one the law actually regulates, because only it compares different fuels fairly.",
         "Und genau die Primärenergie reguliert das Gesetz — nur sie vergleicht verschiedene Energieträger fair."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Drei Bilanzgrenzen", title)
        din = _din_ref("DIN V 18599-1 · GEG § 20")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # The outermost ring fills the free band; the house fills the innermost.
        hub = np.array([-3.60, 0.16, 0])
        house = _small_house(hub + DOWN * 0.08, width=1.55, height=0.90, roof=0.46)
        self.play(Create(house), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.0)

        sizes = ((2.10, 1.42), (3.30, 2.46), (4.50, 3.50))
        colours = (NUTZ_BLUE, END_RED, PRIM_VIOLET)
        rings = [_ring(hub, width=w, height=h, color=c) for (w, h), c in zip(sizes, colours)]
        anchors = [
            (np.array([hub[0] + w / 2, hub[1] + h / 4, 0]), text, colour)
            for (w, h), text, colour in zip(
                sizes,
                ("Raum  →  Nutzenergie Q_h", "Zähler  →  Endenergie Q_E",
                 "Quelle  →  Primärenergie Q_P"),
                colours,
            )
        ]
        labels, leaders = side_labels(anchors, x=-0.75, align="left", font_size=BODY_FONT_SIZE)

        for key, index in (("raum", 0), ("gebaeude", 1), ("quelle", 2)):
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(rings[index]), run_time=0.9)
            self.play(FadeIn(labels[index]), Create(leaders[index]), run_time=0.6)
            if key == "quelle":
                # The upstream chain belongs on the reserved note band — stacking
                # it under the ring label would land it on the ring label below.
                chain = note_line("Vorkette: Gewinnung · Umwandlung · Transport", color=PRIM_VIOLET)
                self.play(FadeIn(chain), run_time=0.5)
            hold_for(self, self.NARRATION, key, used=0.9 + 0.6 + 0.35)

        # —— What the law regulates ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "geg"))
        row, items = equation_row([
            ("qp", "Q_P", PRIM_VIOLET), (None, "=", P_WHITE),
            ("qe", "Q_E", END_RED), (None, "·", P_WHITE),
            ("fp", "f_p", P_YELLOW), (None, "   ← das GEG begrenzt Q_P", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(rings[2].animate.set_stroke(width=4), Create(box), FadeIn(row), run_time=1.2)
        hold_for(self, self.NARRATION, "geg", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(house), FadeOut(chain),
            *[FadeOut(r) for r in rings], FadeOut(labels), FadeOut(leaders), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Der Primärenergiefaktor

class Beat2_Primaerenergiefaktor(Scene):
    NARRATION = [
        ("intro",
         "The step from final to primary energy is one multiplication, with one number per energy carrier.",
         "Der Schritt von End- zu Primärenergie ist eine Multiplikation — mit einer Zahl je Energieträger."),
        ("strom",
         "Grid electricity carries the largest factor, because power stations and the grid lose energy before it arrives.",
         "Netzstrom trägt den größten Faktor — Kraftwerke und Netz verlieren Energie, bevor er ankommt."),
        ("gas",
         "Gas and oil sit just above one: burning them at home wastes comparatively little upstream.",
         "Gas und Öl liegen knapp über eins: Ihre Vorkette verschwendet vergleichsweise wenig."),
        ("holz",
         "Wood is counted at a fifth, and ambient heat a heat pump draws from air or ground counts as zero.",
         "Holz wird mit einem Fünftel angesetzt, und Umweltwärme aus Luft oder Erdreich zählt als null."),
        ("erneuerbar",
         "These are the non-renewable shares — the law only counts the part of the chain that burns something finite.",
         "Das sind die nicht erneuerbaren Anteile — das Gesetz zählt nur den Teil der Kette, der Endliches verbrennt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Der Primärenergiefaktor f_p", title)
        din = _din_ref("GEG Anlage 4")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        rows = [
            ("strom", "Netzstrom", 1.8, P_YELLOW),
            ("gas", "Erdgas / Heizöl", 1.1, GAS_AMBER),
            ("holz", "Holz (Biomasse)", 0.2, P_GREEN),
            ("umwelt", "Umweltwärme, Solar", 0.0, AMBIENT_GREEN),
        ]
        ys = (1.45, 0.62, -0.21, -1.04)
        bars = {key: _factor_bar(y, name, value, colour)
                for (key, name, value, colour), y in zip(rows, ys)}
        axis = Line(np.array([-1.35, -1.40, 0]), np.array([-1.35, 1.80, 0]),
                    color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        self.play(Create(axis), run_time=0.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 0.6)

        # The last clause introduces wood and ambient heat together — they are the
        # same point, so they grow in one step rather than stalling the narration.
        for clause, keys in (("strom", ("strom",)), ("gas", ("gas",)), ("holz", ("holz", "umwelt"))):
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, clause))
            self.play(
                LaggedStart(*[GrowFromEdge(bars[k]["bar"], LEFT) for k in keys], lag_ratio=0.3),
                LaggedStart(*[FadeIn(bars[k]["label"]) for k in keys], lag_ratio=0.3),
                LaggedStart(*[FadeIn(bars[k]["value"]) for k in keys], lag_ratio=0.3),
                run_time=0.9 + 0.4 * (len(keys) - 1),
            )
            hold_for(self, self.NARRATION, clause, used=1.3 + 0.35)

        # —— The non-renewable share ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "erneuerbar"))
        note = note_line("f_p im GEG = nicht erneuerbarer Anteil der Vorkette", color=P_TEAL)
        row, items = equation_row([
            ("qp", "Q_P", PRIM_VIOLET), (None, "=", P_WHITE),
            ("qe", "Q_E", END_RED), (None, "·", P_WHITE),
            ("fp", "f_p,n.ern.", P_YELLOW), (None, "  [kWh/(m²·a)]", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(FadeIn(note), Create(box), FadeIn(row), run_time=1.2)
        ring = highlight_param(items, "fp", color=P_YELLOW)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "erneuerbar", used=1.2 + 0.5 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(ring), FadeOut(box), FadeOut(row), FadeOut(note),
            FadeOut(axis), *[FadeOut(e["group"]) for e in bars.values()],
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Ein Gebäude, zwei Anlagen

class Beat3_Rechenbeispiel(Scene):
    NARRATION = [
        ("intro",
         "Now the whole chain on one building — the same hundred kilowatt-hours of useful heat as in the last chapter.",
         "Jetzt die ganze Kette an einem Gebäude — dieselben hundert Kilowattstunden Nutzwärme wie im letzten Kapitel."),
        ("end",
         "The plant turns that into final energy: a hundred and thirty-five of gas, or twenty-nine of electricity.",
         "Die Anlage macht daraus Endenergie: hundertfünfunddreißig Gas oder neunundzwanzig Strom."),
        ("prim",
         "Now apply the factors. Gas is multiplied by one point one, electricity by one point eight.",
         "Jetzt die Faktoren: Gas mal eins Komma eins, Strom mal eins Komma acht."),
        ("ergebnis",
         "A hundred and forty-nine against fifty-two. The electricity factor is higher, yet the heat pump ends up far lower.",
         "Hundertneunundvierzig gegen zweiundfünfzig. Der Stromfaktor ist höher — und die Wärmepumpe liegt trotzdem weit darunter."),
        ("warum",
         "Because the factor is applied to a number that was already three quarters smaller. Efficiency beats the factor.",
         "Weil der Faktor auf eine Zahl trifft, die schon drei Viertel kleiner war. Effizienz schlägt den Faktor."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Ein Gebäude, zwei Anlagen", title)
        din = _din_ref("GEG § 20 · DIN V 18599")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        x_gas, x_wp = -3.55, 3.55
        y_rows = (1.52, 0.42, -0.68)
        step_labels = VGroup(*[
            body_text(text, font_size=LABEL_FONT_SIZE - 2, color=P_TEAL).move_to(np.array([0.0, y, 0]))
            for text, y in zip(("Nutzenergie Q_h", "× e_p  →  Endenergie Q_E",
                                "× f_p  →  Primärenergie Q_P"), y_rows)
        ])
        heads = VGroup(
            stat_card("Anlage", "Gaskessel", color=GAS_AMBER).move_to(np.array([x_gas, 2.12, 0])),
            stat_card("Anlage", "Wärmepumpe", color=AMBIENT_GREEN).move_to(np.array([x_wp, 2.12, 0])),
        )

        nutz = VGroup(*[
            stat_card("", f"{Q_NUTZ} kWh/(m²·a)", color=NUTZ_BLUE).move_to(np.array([x, y_rows[0], 0]))
            for x in (x_gas, x_wp)
        ])
        self.play(FadeIn(heads), FadeIn(step_labels[0]), FadeIn(nutz), run_time=1.5)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5)

        # —— Final energy ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "end"))
        end_cards = VGroup(
            stat_card("e_p = 1,35", f"{Q_END_GAS} kWh/(m²·a)", color=END_RED).move_to(
                np.array([x_gas, y_rows[1], 0])),
            stat_card("e_p = 0,29", f"{Q_END_WP} kWh/(m²·a)", color=AMBIENT_GREEN).move_to(
                np.array([x_wp, y_rows[1], 0])),
        )
        drops_1 = VGroup(*[
            Arrow(nutz[i].get_bottom(), end_cards[i].get_top(), buff=0.08, color=P_WHITE,
                  stroke_width=2.4, max_tip_length_to_length_ratio=0.28)
            for i in (0, 1)
        ])
        self.play(LaggedStart(*[GrowArrow(a) for a in drops_1], lag_ratio=0.2),
                  FadeIn(step_labels[1]), FadeIn(end_cards), run_time=1.6)
        hold_for(self, self.NARRATION, "end", used=1.6 + 0.35)

        # —— Primary energy ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "prim"))
        prim_cards = VGroup(
            stat_card(f"f_p = {F_P_GAS:.1f}".replace(".", ","), f"{Q_PRIM_GAS} kWh/(m²·a)",
                      color=PRIM_VIOLET).move_to(np.array([x_gas, y_rows[2], 0])),
            stat_card(f"f_p = {F_P_STROM:.1f}".replace(".", ","), f"{Q_PRIM_WP} kWh/(m²·a)",
                      color=PRIM_VIOLET).move_to(np.array([x_wp, y_rows[2], 0])),
        )
        drops_2 = VGroup(*[
            Arrow(end_cards[i].get_bottom(), prim_cards[i].get_top(), buff=0.08, color=P_WHITE,
                  stroke_width=2.4, max_tip_length_to_length_ratio=0.28)
            for i in (0, 1)
        ])
        self.play(LaggedStart(*[GrowArrow(a) for a in drops_2], lag_ratio=0.2),
                  FadeIn(step_labels[2]), FadeIn(prim_cards), run_time=1.6)
        hold_for(self, self.NARRATION, "prim", used=1.6 + 0.35)

        # —— The comparison ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ergebnis"))
        self.play(
            prim_cards[0].animate.set_stroke(color=P_RED, width=3),
            prim_cards[1].animate.set_stroke(color=AMBIENT_GREEN, width=3),
            run_time=1.0,
        )
        hold_for(self, self.NARRATION, "ergebnis", used=1.0 + 0.35)

        # —— Why ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "warum"))
        row, items = equation_row([
            ("gas", f"{Q_END_GAS} · {F_P_GAS:.1f} = {Q_PRIM_GAS}".replace(".", ","), GAS_AMBER),
            (None, "     ", P_WHITE),
            ("wp", f"{Q_END_WP} · {F_P_STROM:.1f} = {Q_PRIM_WP}".replace(".", ","), AMBIENT_GREEN),
            (None, "  kWh/(m²·a)", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(Create(box), FadeIn(row), run_time=1.2)
        hold_for(self, self.NARRATION, "warum", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(heads), FadeOut(nutz),
            FadeOut(end_cards), FadeOut(prim_cards), FadeOut(drops_1), FadeOut(drops_2),
            FadeOut(step_labels), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 4 – Der Grenzwert: das Referenzgebäude

class Beat4_Referenzgebaeude(Scene):
    NARRATION = [
        ("intro",
         "So what does the law compare those numbers against? Not a fixed value — against your own building.",
         "Womit vergleicht das Gesetz diese Zahlen? Nicht mit einem festen Wert — sondern mit Ihrem eigenen Gebäude."),
        ("referenz",
         "It takes the identical geometry and fills it with standardised components and a standardised plant: the reference building.",
         "Es nimmt dieselbe Geometrie und füllt sie mit genormten Bauteilen und einer genormten Anlage: das Referenzgebäude."),
        ("grenze",
         "Its calculated primary energy sets the benchmark, and a new building has to stay clearly below a fixed share of it.",
         "Dessen Primärenergiebedarf setzt den Maßstab — ein Neubau muss deutlich unter einem festen Anteil davon bleiben."),
        ("ergebnis",
         "Our heat pump clears that line. The gas boiler misses it by a factor of three — which is why new builds look the way they do.",
         "Unsere Wärmepumpe unterschreitet die Linie. Der Gaskessel verfehlt sie um das Dreifache — deshalb sehen Neubauten heute so aus."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Der Grenzwert: das Referenzgebäude", title)
        din = _din_ref("GEG § 15 f. · Anlage 1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        # —— Same geometry, standardised build-up ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "referenz"))
        design = _small_house([-4.85, 1.30, 0], width=1.9, height=1.05, roof=0.55, color=P_CYAN)
        reference = _small_house([-2.15, 1.30, 0], width=1.9, height=1.05, roof=0.55, color=P_TEAL)
        design_tag = body_text("Ihr Entwurf", font_size=LABEL_FONT_SIZE - 2, color=P_CYAN)
        design_tag.next_to(design, DOWN, buff=0.18)
        ref_tag = body_text("Referenzgebäude", font_size=LABEL_FONT_SIZE - 2, color=P_TEAL)
        ref_tag.next_to(reference, DOWN, buff=0.18)
        same = body_text("gleiche Geometrie · genormte Bauteile & Anlage",
                         font_size=LABEL_FONT_SIZE - 3, color=P_WHITE)
        same.set_opacity(0.75)
        same.move_to(np.array([-3.50, 0.25, 0]))
        self.play(Create(design), Create(reference), FadeIn(design_tag), FadeIn(ref_tag), run_time=1.5)
        self.play(FadeIn(same), run_time=0.6)
        hold_for(self, self.NARRATION, "referenz", used=1.5 + 0.6 + 0.35)

        # —— The limit line ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "grenze"))
        base_y, unit = -1.35, 0.0145
        ground = Line(np.array([0.55, base_y, 0]), np.array([6.55, base_y, 0]),
                      color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        limit_y = base_y + Q_LIMIT * unit
        limit = DashedLine(np.array([0.55, limit_y, 0]), np.array([6.55, limit_y, 0]),
                           color=P_RED, stroke_width=2.6)
        limit_tag = body_text(f"GEG-Grenze {Q_LIMIT} kWh/(m²·a)", font_size=LABEL_FONT_SIZE - 2, color=P_RED)
        limit_tag.next_to(limit, UP, buff=0.20).align_to(limit, LEFT)
        share_note = note_line(f"Neubau: höchstens {int(REF_SHARE * 100)} % des Referenzgebäude-Werts", color=P_RED)
        self.play(Create(ground), Create(limit), FadeIn(limit_tag), FadeIn(share_note), run_time=1.4)
        hold_for(self, self.NARRATION, "grenze", used=1.4 + 0.35)

        # —— The two designs against it ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ergebnis"))
        gas_bar = stacked_bar([("Gas", Q_PRIM_GAS, GAS_AMBER)], base=np.array([2.10, base_y, 0]),
                              width=1.05, unit=unit)
        wp_bar = stacked_bar([("WP", Q_PRIM_WP, AMBIENT_GREEN)], base=np.array([4.95, base_y, 0]),
                             width=1.05, unit=unit)
        gas_cap = body_text(f"Gaskessel {Q_PRIM_GAS}", font_size=LABEL_FONT_SIZE - 2, color=GAS_AMBER)
        gas_cap.move_to(np.array([2.10, base_y - 0.30, 0]))
        wp_cap = body_text(f"Wärmepumpe {Q_PRIM_WP}", font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN)
        wp_cap.move_to(np.array([4.95, base_y - 0.30, 0]))
        self.play(FadeOut(share_note), run_time=0.3)
        self.play(GrowFromEdge(gas_bar["bars"][0], DOWN), FadeIn(gas_cap), run_time=1.1)
        self.play(GrowFromEdge(wp_bar["bars"][0], DOWN), FadeIn(wp_cap), run_time=1.1)
        verdict = note_line("nur die Wärmepumpe unterschreitet den Grenzwert", color=AMBIENT_GREEN)
        self.play(FadeIn(verdict), run_time=0.7)
        hold_for(self, self.NARRATION, "ergebnis", used=0.3 + 1.1 + 1.1 + 0.7 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(verdict), FadeOut(design), FadeOut(reference),
            FadeOut(design_tag), FadeOut(ref_tag), FadeOut(same), FadeOut(ground),
            FadeOut(limit), FadeOut(limit_tag), FadeOut(gas_bar["bars"]), FadeOut(wp_bar["bars"]),
            FadeOut(gas_cap), FadeOut(wp_cap), run_time=0.6,
        )
        self.wait(0.4)

#endregion
