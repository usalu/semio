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
WATER_HOT = "#F97316"

# Droplet speed in scene units per second per unit of volume flow, and pump
# impeller spin in radians per second.
WATER_SPEED = 0.065
PUMP_SPIN = 4.0

# Riser volume flows in the same arbitrary units: unbalanced water follows the
# lowest resistance, balanced water follows each room's calculated heat load.
FLOW_UNBALANCED = (11.0, 7.5, 4.0, 2.0)
FLOW_BALANCED = (5.5, 5.5, 5.5, 5.5)

# The schematic gas-boiler example as Chapters 4–6 display it: useful heat,
# round(Q_E_GAS) from ``4_system_losses``, Q_PRIM_GAS from ``5_primary_energy``,
# and the class ``6_energieausweis`` looks up for that final energy.
CHAIN_NUTZ, CHAIN_END, CHAIN_PRIM, CHAIN_CLASS = 100, 140, 154, "E"


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


def _water_stream(start, end, *, flow, colour=WATER_HOT, spacing=0.30, radius=0.055):
    """💧 Droplets travelling along one straight pipe section at a speed set by its volume flow.

    Returns the droplets and the flow tracker — animating the tracker changes the
    speed smoothly while the droplets keep moving.
    """
    start, end = np.array(start, dtype=float), np.array(end, dtype=float)
    length = float(np.linalg.norm(end - start))
    direction = (end - start) / length
    count = max(2, round(length / spacing))
    rate = ValueTracker(flow)
    state = {"phase": 0.0}
    drops = VGroup(*[Dot(start, radius=radius, color=colour) for _ in range(count)])

    def advance(group, dt):
        state["phase"] = (state["phase"] + dt * rate.get_value() * WATER_SPEED) % length
        for i, drop in enumerate(group):
            drop.move_to(start + direction * ((state["phase"] + i * length / count) % length))

    advance(drops, 0.0)
    drops.add_updater(advance)
    return drops, rate


def _section_flows(riser_flows):
    """➗ Flow in each main-pipe section: everything still to be delivered downstream."""
    return [float(sum(riser_flows[i:])) for i in range(len(riser_flows))]
#endregion


#region Beat 1 – Warum Rechnung und Abrechnung auseinanderlaufen

class Beat1_BerechnungVsAbrechnung(Scene):
    NARRATION = [
        ("intro",
         "Does the calculated demand match the bill? Rarely exactly — because the calculation deliberately uses standardised conditions.",
         "Stimmt der berechnete Bedarf mit der Rechnung überein? Selten genau — denn die Berechnung nutzt bewusst Normbedingungen."),
        ("normen",
         "The calculation fixes the use: a twenty-degree setpoint, a standard air change, standard hot water, a reference climate.",
         "Die Berechnung legt die Nutzung fest: zwanzig Grad als Sollwert, genormter Luftwechsel, genormtes Warmwasser, ein Referenzklima."),
        ("real",
         "Real households heat unevenly, air unevenly and live unevenly. So the measured points scatter around the diagonal.",
         "Echte Haushalte heizen, lüften und leben ungleichmäßig. Die Messpunkte streuen deshalb um die Diagonale."),
        ("prebound",
         "The scatter is not symmetric: poorly insulated buildings consume on average less than calculated — people heat more sparingly.",
         "Die Streuung ist nicht symmetrisch: Schlecht gedämmte Gebäude verbrauchen im Mittel weniger als berechnet — man heizt sparsamer."),
        ("rebound",
         "In efficient buildings it is on average above — for instance through higher comfort temperatures.",
         "In effizienten Gebäuden liegt er im Mittel darüber — etwa durch höhere Komforttemperaturen."),
        ("fazit",
         "The certificate therefore compares buildings, not households. That is exactly what it was built to do.",
         "Der Ausweis vergleicht deshalb Gebäude, nicht Haushalte. Genau dafür ist er gemacht."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Warum Rechnung und Abrechnung auseinanderlaufen", title)
        din = _din_ref("DIN V 18599-10:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        # —— The standardised boundary conditions ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "normen"))
        assumptions = card_grid([
            ("Innentemperatur", "20 °C Sollwert", CALC_CYAN),
            ("Luftwechsel", "genormt", CALC_CYAN),
            ("Warmwasser", "pauschal je m²", CALC_CYAN),
            ("Klima", "Referenzklima", CALC_CYAN),
        ], rows=2, cols=2, buff=0.20)
        # The whole left column is stacked from measured heights: grid, then the two
        # verdict cards, with fixed gaps — hand-set y values let the cards touch.
        assumptions.move_to(np.array([-3.95, 0.0, 0])).align_to(np.array([0, 2.02, 0]), UP)
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
        schematic_tag = body_text("schematische Darstellung", font_size=LABEL_FONT_SIZE - 4, color=P_TEAL)
        schematic_tag.set_opacity(0.8)
        diag_tag.set_opacity(0.6).next_to(axes["pt"](0.88, 0.88), UL, buff=0.06)
        schematic_tag.next_to(axes["x_label"], DOWN, buff=0.12)
        self.play(Create(axes["group"]), FadeIn(y_cap), run_time=1.1)
        self.play(Create(diagonal), FadeIn(diag_tag), FadeIn(schematic_tag), run_time=0.8)

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
        pre_card = stat_card("schlecht gedämmt · im Mittel", "gemessen < berechnet", color=GOOD_GREEN)
        pre_card.next_to(assumptions, DOWN, buff=0.30).set_x(-3.95)
        self.play(Create(trend), FadeIn(pre_card), run_time=1.3)
        hold_for(self, self.NARRATION, "prebound", used=1.3 + 0.35)

        # —— Rebound ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rebound"))
        re_card = stat_card("effizient · im Mittel", "gemessen > berechnet", color=WARM_RED)
        re_card.next_to(pre_card, DOWN, buff=0.16).set_x(-3.95)
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
            FadeOut(trend), FadeOut(diagonal), FadeOut(diag_tag), FadeOut(schematic_tag),
            FadeOut(axes["group"]),
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
         "So the near rooms overheat while the far ones stay cold — and often the flow temperature simply gets turned up.",
         "Die nahen Räume überhitzen, die fernen bleiben kalt — und oft wird einfach die Vorlauftemperatur hochgedreht."),
        ("abgleich",
         "Hydraulic balancing presets every valve to the flow that room's calculated heat load actually needs.",
         "Der hydraulische Abgleich stellt jedes Ventil auf den Durchfluss ein, den die berechnete Raumheizlast wirklich braucht."),
        ("gewinn",
         "The flow temperature can then come down — an important condition for a heat pump to reach a good seasonal performance factor.",
         "Danach kann die Vorlauftemperatur sinken — eine wichtige Voraussetzung für eine gute Jahresarbeitszahl der Wärmepumpe."),
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
        pump_center = np.array([-5.65, main_y, 0])
        pump = Circle(radius=0.32, color=P_ORANGE, fill_color=P_ORANGE, fill_opacity=0.5,
                      stroke_width=2).move_to(pump_center)
        impeller = VGroup(
            Line(LEFT * 0.20, RIGHT * 0.20), Line(DOWN * 0.20, UP * 0.20),
        ).set_stroke(P_WHITE, width=2.6).move_to(pump_center)
        pump_tag = body_text("Pumpe", font_size=LABEL_FONT_SIZE - 3, color=P_ORANGE)
        pump_tag.move_to(np.array([-5.65, main_y - 0.58, 0]))
        rooms = VGroup(*[_room([x, room_y, 0], width=1.70, height=1.25) for x in xs])
        # Each riser climbs straight into its radiator, so the water visibly arrives
        # where the heat is given off instead of stopping at the floor.
        riser_x = [float(room[1].get_center()[0]) for room in rooms]
        riser_top = [room[1].get_bottom() for room in rooms]
        outlet = pump_center + RIGHT * 0.32
        main = Line(outlet, np.array([riser_x[-1], main_y, 0]), color=P_TEAL, stroke_width=4)
        risers = VGroup(*[
            Line(np.array([rx, main_y, 0]), top, color=P_TEAL, stroke_width=4)
            for rx, top in zip(riser_x, riser_top)
        ])
        self.play(Create(pump), Create(impeller), FadeIn(pump_tag), Create(main),
                  Create(risers), Create(rooms), run_time=1.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.8)

        # —— Unbalanced: flow follows resistance, not demand ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unabgeglichen"))
        section_starts = [outlet] + [np.array([rx, main_y, 0]) for rx in riser_x[:-1]]
        section_ends = [np.array([rx, main_y, 0]) for rx in riser_x]
        main_streams = [
            _water_stream(a, b, flow=f)
            for a, b, f in zip(section_starts, section_ends, _section_flows(FLOW_UNBALANCED))
        ]
        riser_streams = [
            _water_stream(np.array([rx, main_y, 0]), top, flow=f)
            for rx, top, f in zip(riser_x, riser_top, FLOW_UNBALANCED)
        ]
        streams = VGroup(*[drops for drops, _rate in main_streams + riser_streams])
        main_rates = [rate for _drops, rate in main_streams]
        riser_rates = [rate for _drops, rate in riser_streams]
        spin = ValueTracker(PUMP_SPIN)
        impeller.add_updater(lambda m, dt: m.rotate(-dt * spin.get_value(), about_point=pump_center))
        self.play(
            FadeIn(streams),
            *[room[1].animate.set_fill(WARM_RED, opacity=0.85 * f / max(FLOW_UNBALANCED))
              for room, f in zip(rooms, FLOW_UNBALANCED)],
            run_time=1.5,
        )
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
        # Built at the origin and then moved: a VGroup whose parts are already at
        # absolute coordinates recentres its *bounding box*, which is what left
        # the valve glyphs scattered across the pipe run.
        valves = VGroup(*[
            VGroup(
                Circle(radius=0.12, color=P_TEAL, stroke_width=2),
                Line(LEFT * 0.20, RIGHT * 0.20, color=P_TEAL, stroke_width=2),
            ).move_to(np.array([rx, main_y + 0.30, 0]))
            for rx in riser_x
        ])
        self.play(
            *[rate.animate.set_value(f) for rate, f in zip(main_rates, _section_flows(FLOW_BALANCED))],
            *[rate.animate.set_value(f) for rate, f in zip(riser_rates, FLOW_BALANCED)],
            *[r[0].animate.set_color(GOOD_GREEN) for r in rooms],
            *[r[1].animate.set_fill(WARM_RED, opacity=0.85 * f / max(FLOW_UNBALANCED))
              for r, f in zip(rooms, FLOW_BALANCED)],
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
            ("jaz", "höhere JAZ", GOOD_GREEN),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        duty = note_line("für viele Heizungsanlagen verpflichtend — GEG § 60c", color=P_TEAL)
        self.play(FadeOut(preset), Create(box), FadeIn(row), FadeIn(duty), run_time=1.2)
        hold_for(self, self.NARRATION, "gewinn", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(duty), FadeOut(streams), FadeOut(valves),
            FadeOut(rooms), FadeOut(risers), FadeOut(main), FadeOut(pump), FadeOut(impeller),
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
         "A hundred kilowatt-hours of useful heat became a hundred and forty at the meter, a hundred and fifty-four at the source — and one letter.",
         "Aus hundert Kilowattstunden Nutzwärme wurden hundertvierzig am Zähler, hundertvierundfünfzig an der Quelle — und ein Buchstabe."),
        ("outro",
         "That is the complete path from a cold wall in January to the number a landlord shows a new tenant.",
         "Das ist der vollständige Weg von einer kalten Wand im Januar bis zur Zahl, die ein Vermieter einem neuen Mieter zeigt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Serie im Rückblick", title)
        din = _din_ref("GEG · DIN V 18599:2018-09")
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
            ("Nutzenergie", str(CHAIN_NUTZ), "#38BDF8"),
            ("Endenergie", str(CHAIN_END), "#DC2626"),
            ("Primärenergie", str(CHAIN_PRIM), "#A78BFA"),
            ("Ausweis", f"Klasse {CHAIN_CLASS}", "#F5A623"),
        ], rows=1, buff=0.40)
        chain.move_to(DOWN * 0.35)
        unit_note = note_line("kWh/(m²·a) — schematisches Beispiel, Gaskessel-Variante", color=P_TEAL)
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
