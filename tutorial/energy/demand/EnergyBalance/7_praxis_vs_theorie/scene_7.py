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

TITLE_DE = "Theorie vs. Praxis"

CALC_CYAN = "#38BDF8"
REAL_ORANGE = "#FB923C"
WARM_RED = "#EF4444"
COLD_BLUE = "#4D96FF"
GOOD_GREEN = "#22C55E"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _room(center, *, width=1.45, height=1.05, color=P_WHITE):
    """🚪 One room box with a radiator on its left wall."""
    center = np.array(center, dtype=float)
    box = Rectangle(width=width, height=height, color=color, stroke_width=2.4).move_to(center)
    radiator = Rectangle(width=width * 0.34, height=height * 0.38, color=WARM_RED, stroke_width=1.6)
    radiator.move_to(box.get_left() + RIGHT * width * 0.26)
    return VGroup(box, radiator)


def _flow_arrow(start, end, colour, *, width):
    """💧 Volume-flow arrow whose thickness is the flow it carries."""
    return Arrow(np.array(start, dtype=float), np.array(end, dtype=float), buff=0,
                 color=colour, stroke_width=width, max_tip_length_to_length_ratio=0.35,
                 tip_length=0.16)
#endregion


#region Beat 1 – Warum Rechnung und Abrechnung auseinanderlaufen

class Beat1_BerechnungVsAbrechnung(Scene):
    NARRATION = [
        ("intro",
         "One honest closing question: does the calculated demand match the real bill? Almost never — and that is by design.",
         "Eine ehrliche Schlussfrage: Stimmt der berechnete Bedarf mit der echten Rechnung überein? Fast nie — und das mit Absicht."),
        ("normen",
         "The calculation fixes the occupants: twenty degrees everywhere, a standard air change, standard hot water, reference weather.",
         "Die Berechnung legt die Nutzung fest: überall zwanzig Grad, genormter Luftwechsel, genormtes Warmwasser, Referenzwetter."),
        ("real",
         "Real households heat unevenly, air unevenly and live unevenly. So the measured points scatter around the diagonal.",
         "Echte Haushalte heizen, lüften und leben ungleichmäßig. Die Messpunkte streuen deshalb um die Diagonale."),
        ("prebound",
         "And the scatter is not symmetric. In poor buildings people heat less than the standard assumes, so they use less.",
         "Und die Streuung ist nicht symmetrisch. In schlechten Gebäuden heizen Menschen weniger als angenommen — sie verbrauchen weniger."),
        ("rebound",
         "In efficient buildings the opposite happens: comfort rises, and measured consumption overshoots the calculation.",
         "In effizienten Gebäuden ist es umgekehrt: Der Komfort steigt, und der gemessene Verbrauch übertrifft die Rechnung."),
        ("fazit",
         "The certificate therefore compares buildings, not households. That is exactly what it was built to do.",
         "Der Ausweis vergleicht deshalb Gebäude, nicht Haushalte. Genau dafür ist er gemacht."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Warum Rechnung und Abrechnung auseinanderlaufen", title)
        din = _din_ref("DIN V 18599-10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        # —— The standardised boundary conditions ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "normen"))
        assumptions = card_grid([
            ("Innentemperatur", "20 °C, alle Räume", CALC_CYAN),
            ("Luftwechsel", "genormt", CALC_CYAN),
            ("Warmwasser", "pauschal je m²", CALC_CYAN),
            ("Wetter", "Referenzjahr", CALC_CYAN),
        ], rows=2, cols=2, buff=0.20)
        assumptions.move_to(np.array([-3.95, 0.62, 0]))
        self.play(LaggedStart(*[FadeIn(c, shift=UP * 0.1) for c in assumptions], lag_ratio=0.15),
                  run_time=1.7)
        hold_for(self, self.NARRATION, "normen", used=1.7 + 0.35)

        # —— Calculated vs. measured ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "real"))
        axes = labeled_axes([1.05, -1.30, 0], x_len=4.60, y_len=3.05,
                            x_label="berechnet  →", color=P_WHITE)
        y_cap = body_text("gemessen", font_size=LABEL_FONT_SIZE - 2, color=P_WHITE)
        y_cap.set_opacity(0.85).rotate(PI / 2).next_to(axes["y_axis"], LEFT, buff=0.14)
        diagonal = DashedLine(axes["pt"](0.0, 0.0), axes["pt"](1.0, 1.0),
                              color=P_WHITE, stroke_width=2, stroke_opacity=0.55)
        diag_tag = body_text("Rechnung = Messung", font_size=LABEL_FONT_SIZE - 4, color=P_WHITE)
        diag_tag.set_opacity(0.6).next_to(axes["pt"](0.88, 0.88), UL, buff=0.06)
        self.play(Create(axes["group"]), FadeIn(y_cap), run_time=1.1)
        self.play(Create(diagonal), FadeIn(diag_tag), run_time=0.8)

        rng = np.random.default_rng(11)
        xs = np.linspace(0.12, 0.94, 22)
        # Measured consumption is a flatter line than the diagonal: the well
        # documented prebound / rebound split, not random noise.
        ys = 0.30 + 0.52 * xs + rng.normal(0.0, 0.055, len(xs))
        points = VGroup(*[
            Dot(axes["pt"](float(x), float(np.clip(y, 0.05, 0.97))), radius=0.058,
                color=REAL_ORANGE, fill_opacity=0.9)
            for x, y in zip(xs, ys)
        ])
        self.play(LaggedStart(*[FadeIn(d, scale=0.5) for d in points], lag_ratio=0.05), run_time=1.6)
        hold_for(self, self.NARRATION, "real", used=1.1 + 0.8 + 1.6 + 0.35)

        # —— Prebound ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "prebound"))
        trend = Line(axes["pt"](0.10, 0.352), axes["pt"](0.96, 0.799),
                     color=REAL_ORANGE, stroke_width=3)
        pre_card = stat_card("schlechte Gebäude", "gemessen < berechnet", color=GOOD_GREEN)
        pre_card.move_to(np.array([-3.95, -1.05, 0]))
        self.play(Create(trend), FadeIn(pre_card), run_time=1.3)
        hold_for(self, self.NARRATION, "prebound", used=1.3 + 0.35)

        # —— Rebound ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rebound"))
        re_card = stat_card("effiziente Gebäude", "gemessen > berechnet", color=WARM_RED)
        re_card.move_to(np.array([-3.95, -1.88, 0]))
        self.play(FadeIn(re_card), run_time=1.1)
        hold_for(self, self.NARRATION, "rebound", used=1.1 + 0.35)

        # —— What the certificate is for ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fazit"))
        verdict = note_line("genormte Randbedingungen machen Gebäude vergleichbar — nicht Haushalte",
                            color=P_TEAL)
        self.play(FadeIn(verdict), run_time=0.9)
        hold_for(self, self.NARRATION, "fazit", used=0.9 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(verdict), FadeOut(assumptions), FadeOut(points),
            FadeOut(trend), FadeOut(diagonal), FadeOut(diag_tag), FadeOut(axes["group"]),
            FadeOut(y_cap), FadeOut(pre_card), FadeOut(re_card), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Hydraulischer Abgleich

class Beat2_HydraulischerAbgleich(Scene):
    NARRATION = [
        ("intro",
         "One more gap between paper and practice, and this one is pure hydraulics.",
         "Noch eine Lücke zwischen Papier und Praxis — und diese ist reine Hydraulik."),
        ("unabgeglichen",
         "Water takes the path of least resistance. The rooms nearest the pump get far more flow than they were calculated for.",
         "Wasser nimmt den Weg des geringsten Widerstands. Die Räume nahe der Pumpe bekommen weit mehr Durchfluss als berechnet."),
        ("folge",
         "So the near rooms overheat while the far ones stay cold — and the occupants answer by raising the flow temperature.",
         "Die nahen Räume überhitzen, die fernen bleiben kalt — und die Bewohner drehen die Vorlauftemperatur hoch."),
        ("abgleich",
         "Hydraulic balancing presets every valve to the flow that room's calculated load actually needs.",
         "Der hydraulische Abgleich stellt jedes Ventil auf den Durchfluss ein, den die berechnete Raumlast wirklich braucht."),
        ("gewinn",
         "The flow temperature can then come down, which is exactly the condition a heat pump needs to reach a good performance factor.",
         "Danach kann die Vorlauftemperatur sinken — genau die Bedingung, unter der eine Wärmepumpe eine gute Arbeitszahl erreicht."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Hydraulischer Abgleich", title)
        din = _din_ref("GEG § 60c · VOB/C DIN 18380")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        main_y, room_y = -0.72, 0.95
        xs = (-3.30, -1.10, 1.10, 3.30)
        pump = Circle(radius=0.32, color=P_ORANGE, fill_color=P_ORANGE, fill_opacity=0.5,
                      stroke_width=2).move_to(np.array([-5.65, main_y, 0]))
        pump_tag = body_text("Pumpe", font_size=LABEL_FONT_SIZE - 3, color=P_ORANGE)
        pump_tag.move_to(np.array([-5.65, main_y - 0.58, 0]))
        rooms = VGroup(*[_room([x, room_y, 0], width=1.70, height=1.25) for x in xs])
        main = Line(np.array([-5.33, main_y, 0]), np.array([4.40, main_y, 0]),
                    color=P_TEAL, stroke_width=3)
        risers = VGroup(*[
            Line(np.array([x, main_y, 0]), np.array([x, room_y - 0.625, 0]),
                 color=P_TEAL, stroke_width=3)
            for x in xs
        ])
        self.play(Create(pump), FadeIn(pump_tag), Create(main),
                  Create(risers), Create(rooms), run_time=1.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.8)

        # —— Unbalanced: flow follows resistance, not demand ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unabgeglichen"))
        wide = (11.0, 7.5, 4.0, 2.0)
        bad_flows = VGroup(*[
            _flow_arrow([x, main_y + 0.30, 0], [x, room_y - 0.70, 0],
                        COLD_BLUE if w < 5 else WARM_RED, width=w)
            for x, w in zip(xs, wide)
        ])
        self.play(LaggedStart(*[GrowArrow(a) for a in bad_flows], lag_ratio=0.15), run_time=1.5)
        hold_for(self, self.NARRATION, "unabgeglichen", used=1.5 + 0.35)

        # —— Consequence ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "folge"))
        verdicts = VGroup(*[
            body_text(text, font_size=LABEL_FONT_SIZE - 3, color=colour).move_to(
                np.array([x, room_y + 0.85, 0]))
            for x, text, colour in zip(xs, ("zu heiß", "zu heiß", "zu kalt", "zu kalt"),
                                       (WARM_RED, WARM_RED, COLD_BLUE, COLD_BLUE))
        ])
        self.play(
            rooms[0][0].animate.set_color(WARM_RED), rooms[1][0].animate.set_color(WARM_RED),
            rooms[2][0].animate.set_color(COLD_BLUE), rooms[3][0].animate.set_color(COLD_BLUE),
            LaggedStart(*[FadeIn(v) for v in verdicts], lag_ratio=0.15), run_time=1.5,
        )
        reaction = note_line("Reaktion im Alltag: Vorlauftemperatur hochdrehen", color=WARM_RED)
        self.play(FadeIn(reaction), run_time=0.7)
        hold_for(self, self.NARRATION, "folge", used=1.5 + 0.7 + 0.35)

        # —— Balanced ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "abgleich"))
        self.play(FadeOut(reaction), FadeOut(verdicts), run_time=0.35)
        good_flows = VGroup(*[
            _flow_arrow([x, main_y + 0.30, 0], [x, room_y - 0.70, 0], GOOD_GREEN, width=5.5)
            for x in xs
        ])
        # Built at the origin and then moved: a VGroup whose parts are already at
        # absolute coordinates recentres its *bounding box*, which is what left
        # the valve glyphs scattered across the pipe run.
        valves = VGroup(*[
            VGroup(
                Circle(radius=0.12, color=P_TEAL, stroke_width=2),
                Line(LEFT * 0.20, RIGHT * 0.20, color=P_TEAL, stroke_width=2),
            ).move_to(np.array([x, main_y + 0.12, 0]))
            for x in xs
        ])
        self.play(
            Transform(bad_flows, good_flows),
            *[r[0].animate.set_color(GOOD_GREEN) for r in rooms],
            run_time=1.6,
        )
        self.play(LaggedStart(*[Create(v) for v in valves], lag_ratio=0.15), run_time=1.0)
        preset = note_line("Voreinstellung je Ventil = berechneter Volumenstrom des Raums", color=GOOD_GREEN)
        self.play(FadeIn(preset), run_time=0.7)
        hold_for(self, self.NARRATION, "abgleich", used=0.35 + 1.6 + 1.0 + 0.7 + 0.35)

        # —— Why it pays ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gewinn"))
        row, items = equation_row([
            ("ab", "Abgleich", GOOD_GREEN), (None, "→", P_WHITE),
            ("vl", "niedrigerer Vorlauf", P_CYAN), (None, "→", P_WHITE),
            ("jaz", "höhere JAZ", GOOD_GREEN), (None, "  ·  Pflicht & förderfähig", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(FadeOut(preset), Create(box), FadeIn(row), run_time=1.2)
        hold_for(self, self.NARRATION, "gewinn", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(bad_flows), FadeOut(valves),
            FadeOut(rooms), FadeOut(risers), FadeOut(main), FadeOut(pump),
            FadeOut(pump_tag), run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Die Serie im Rückblick

class Beat3_SerienAbschluss(Scene):
    NARRATION = [
        ("intro",
         "Four videos, one continuous calculation — worth seeing end to end one last time.",
         "Vier Videos, eine durchgehende Rechnung — es lohnt sich, sie einmal am Stück zu sehen."),
        ("kette",
         "The fundamentals gave us the units. Heating and cooling gave us the flows across the envelope.",
         "Die Grundlagen gaben uns die Einheiten. Heizen und Kühlen gaben uns die Ströme über die Hülle."),
        ("bilanz",
         "This video closed the balance, walked it out to the meter, out to the source, and onto the certificate.",
         "Dieses Video schloss die Bilanz, führte sie zum Zähler, zur Quelle und auf den Ausweis."),
        ("zahlen",
         "A hundred kilowatt-hours of useful heat became a hundred and thirty-five at the meter, a hundred and forty-nine at the source — and one letter.",
         "Aus hundert Kilowattstunden Nutzwärme wurden hundertfünfunddreißig am Zähler, hundertneunundvierzig an der Quelle — und ein Buchstabe."),
        ("outro",
         "That is the complete path from a cold wall in January to the number a landlord shows a new tenant.",
         "Das ist der vollständige Weg von einer kalten Wand im Januar bis zur Zahl, die ein Vermieter einem neuen Mieter zeigt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Serie im Rückblick", title)
        din = _din_ref("GEG · DIN V 18599")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        # —— The four videos ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kette"))
        videos = card_grid([
            ("1 · Grundlagen", "Watt, Joule, ΔT", P_YELLOW),
            ("2 · Heizen", "Verluste & Gewinne", P_ORANGE),
            ("3 · Kühlen", "Sommerlasten", P_CYAN),
            ("4 · Energiebilanz", "Gesetz & Ausweis", P_TEAL),
        ], rows=1, buff=0.40)
        videos.move_to(UP * 1.35)
        links = VGroup(*[
            Arrow(videos[i].get_right(), videos[i + 1].get_left(), buff=0.06, color=P_WHITE,
                  stroke_width=2, stroke_opacity=0.6, max_tip_length_to_length_ratio=0.3)
            for i in range(3)
        ])
        self.play(LaggedStart(*[FadeIn(c, shift=DOWN * 0.1) for c in videos], lag_ratio=0.18),
                  run_time=1.7)
        self.play(LaggedStart(*[GrowArrow(a) for a in links], lag_ratio=0.2), run_time=1.0)
        hold_for(self, self.NARRATION, "kette", used=1.7 + 1.0 + 0.35)

        # —— What this video added ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bilanz"))
        self.play(videos[3][0].animate.set_stroke(width=3.4), run_time=0.9)
        hold_for(self, self.NARRATION, "bilanz", used=0.9 + 0.35)

        # —— The number chain ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zahlen"))
        chain = card_grid([
            ("Nutzenergie", "100", "#38BDF8"),
            ("Endenergie", "135", "#DC2626"),
            ("Primärenergie", "149", "#A78BFA"),
            ("Ausweis", "Klasse E", "#F5A623"),
        ], rows=1, buff=0.40)
        chain.move_to(DOWN * 0.35)
        unit_note = note_line("alle Werte in kWh/(m²·a) — Gaskessel-Variante", color=P_TEAL)
        chain_links = VGroup(*[
            Arrow(chain[i].get_right(), chain[i + 1].get_left(), buff=0.06, color=P_WHITE,
                  stroke_width=2, stroke_opacity=0.6, max_tip_length_to_length_ratio=0.3)
            for i in range(3)
        ])
        self.play(LaggedStart(*[FadeIn(c, shift=UP * 0.1) for c in chain], lag_ratio=0.2),
                  FadeIn(unit_note), run_time=1.8)
        self.play(LaggedStart(*[GrowArrow(a) for a in chain_links], lag_ratio=0.2), run_time=1.0)
        hold_for(self, self.NARRATION, "zahlen", used=1.8 + 1.0 + 0.35)

        # —— Close ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        outro = body_text("Vom Watt zum Energieausweis.", font_size=FORMULA_FONT_SIZE, color=P_WHITE)
        outro.move_to(DOWN * 1.55)
        self.play(FadeIn(outro, shift=UP * 0.12), run_time=1.2)
        hold_for(self, self.NARRATION, "outro", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(outro), FadeOut(videos), FadeOut(links),
            FadeOut(chain), FadeOut(chain_links), FadeOut(unit_note), run_time=0.7,
        )
        self.wait(0.6)

#endregion
