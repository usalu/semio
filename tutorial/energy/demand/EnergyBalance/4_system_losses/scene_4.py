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
    chip, note_line, card_grid, stat_card, side_labels, stacked_bar, fit_band,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Von der Nutzenergie zur Endenergie"

NUTZ_BLUE = "#38BDF8"
UEBER_RED = "#EF4444"
VERT_ORANGE = "#FB923C"
SPEI_YELLOW = "#FACC15"
ERZ_AMBER = "#F59E0B"
END_RED = "#DC2626"
AMBIENT_GREEN = "#22C55E"

# The one worked example the rest of the series keeps quoting: 100 kWh/(m²·a)
# of useful heat, delivered once by a gas boiler and once by a heat pump. Every
# number below is a schematic teaching example, not a measured or normed value.
Q_NUTZ = 100.0

# —— The Anlagenaufwandszahl chain (Beats 2–4) — one set of factors used
# everywhere, so the multiplication a viewer reproduces by hand always matches
# what is on screen. Erzeugung is kept separate for the plain and the
# condensing boiler, since Beat 3 shows both and they must not share a label.
E_UEBERGABE = 1.04
E_VERTEILUNG = 1.08
E_SPEICHERUNG = 1.06
E_ERZEUGUNG_GAS = 1.18            # Gaskessel: 118 kWh Erdgas → 100 kWh Wärme
E_ERZEUGUNG_GAS_BRENNWERT = 1.07  # nach Kondensation: 107 kWh → 100 kWh
E_ERZEUGUNG_WP = 0.26             # Wärmepumpe: 26 kWh Strom → 100 kWh Wärme

E_P_GAS = E_UEBERGABE * E_VERTEILUNG * E_SPEICHERUNG * E_ERZEUGUNG_GAS  # ≈ 1.40
E_P_WP = E_UEBERGABE * E_VERTEILUNG * E_SPEICHERUNG * E_ERZEUGUNG_WP    # ≈ 0.31
Q_E_GAS = Q_NUTZ * E_P_GAS  # ≈ 140 kWh/(m²·a)
Q_E_WP = Q_NUTZ * E_P_WP    # ≈ 31 kWh/(m²·a)


def _cascade(factors) -> tuple[float, ...]:
    """🪜 Energy each stage adds when the useful heat is multiplied up the chain."""
    added, level = [], Q_NUTZ
    for factor in factors:
        added.append(level * factor - level)
        level *= factor
    return tuple(added)


# Beat 1's staircase is the same multiplication Beat 4 shows as e_p, so its
# segments are derived from the factors rather than typed — typed values once
# summed to 135 while Beat 4 printed 140 for the very same boiler.
STAGES = tuple(zip(
    ("Übergabe", "Verteilung", "Speicherung", "Erzeugung"),
    _cascade((E_UEBERGABE, E_VERTEILUNG, E_SPEICHERUNG, E_ERZEUGUNG_GAS)),
    (UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER),
))  # ≈ +4, +8, +7, +21 kWh → Q_E_GAS


def _de_num(value: float, decimals: int = 2) -> str:
    """🔢 Format a factor with a German decimal comma, e.g. 1.40 → '1,40'."""
    return f"{value:.{decimals}f}".replace(".", ",")


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _tank(center, color=SPEI_YELLOW):
    """🛢️ Buffer / hot-water tank, sized to sit wholly inside the basement band."""
    center = np.array(center, dtype=float)
    body = RoundedRectangle(corner_radius=0.18, width=0.80, height=0.85, color=color, stroke_width=3)
    body.move_to(center)
    lid = Ellipse(width=0.80, height=0.18, color=color, stroke_width=2).move_to(body.get_top())
    return VGroup(body, lid)


def _radiator(center, color=UEBER_RED, width=0.85, height=0.62):
    """🔥 Wall radiator seen from the front."""
    center = np.array(center, dtype=float)
    body = Rectangle(width=width, height=height, color=color, stroke_width=2).move_to(center)
    fins = VGroup(*[
        Line(body.get_top() + RIGHT * x, body.get_bottom() + RIGHT * x, color=color, stroke_width=1.4)
        for x in np.linspace(-width * 0.36, width * 0.36, 5)
    ])
    return VGroup(body, fins)


def _floor_loop(center, *, width=1.9, color=P_CYAN):
    """➰ Underfloor heating loop embedded in a screed line."""
    center = np.array(center, dtype=float)
    slab = Line(center + LEFT * width / 2, center + RIGHT * width / 2, color=P_WHITE, stroke_width=4)
    xs = np.linspace(-width / 2 + 0.12, width / 2 - 0.12, 40)
    loop = VMobject(color=color, stroke_width=4)
    loop.set_points_smoothly([center + RIGHT * x + UP * (0.16 + 0.07 * np.sin(x * 7)) for x in xs])
    return VGroup(slab, loop)


def _loss_waves(center, color=ERZ_AMBER, n=3, spread=0.13):
    """〰️ Radiating arcs — heat escaping at this point in the chain."""
    center = np.array(center, dtype=float)
    return VGroup(*[
        Arc(radius=0.17 + i * spread, start_angle=PI * 0.18, angle=PI * 0.64,
            color=color, stroke_width=2, stroke_opacity=0.85 - i * 0.2).move_arc_center_to(center)
        for i in range(n)
    ])


def _cascade_column(x, values, *, base_y, unit, width=1.15):
    """🏗️ One step of the loss cascade: the useful heat plus every loss added so far."""
    segments = [("Nutzenergie", Q_NUTZ, NUTZ_BLUE)]
    segments += [(name, value, colour) for (name, value, colour), keep
                 in zip(STAGES, values) if keep]
    return stacked_bar(segments, base=np.array([x, base_y, 0]), width=width, unit=unit)
#endregion


#region Beat 1 – Die Verlustkette vom Raum zum Zähler

class Beat1_Verlustkette(Scene):
    NARRATION = [
        ("intro",
         "So far every kilowatt-hour has been heat inside the room. Nobody sells that — it has to be produced.",
         "Bisher war jede Kilowattstunde Wärme im Raum. Die verkauft niemand — sie muss erst erzeugt werden."),
        ("nutz",
         "Start from the demand we calculated: a hundred kilowatt-hours per square metre and year of useful heat.",
         "Wir starten beim berechneten Bedarf: hundert Kilowattstunden je Quadratmeter und Jahr Nutzenergie."),
        ("stufen",
         "Between that room and the gas meter sit four stages, each needing extra energy — how it is counted depends on where it happens.",
         "Zwischen Raum und Gaszähler liegen vier Stufen, jede braucht zusätzliche Energie — wie sie zählt, hängt vom Ort des Verlusts ab."),
        ("ende",
         "The energy billed at the building boundary is counted as final energy — here roughly forty percent more than the room ever received.",
         "Die am Gebäude bezogene Energie wird als Endenergie bilanziert — hier rund vierzig Prozent mehr, als der Raum je bekommen hat."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Die Verlustkette vom Raum zum Zähler", title)
        din = _din_ref("DIN V 18599-5:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        revision_note = note_line(
            "Seit 2025 als DIN/TS 18599-5:2025-10 fortgeschrieben — GEG nutzt weiterhin 2018",
            color=P_TEAL,
        )
        self.play(FadeIn(caption), FadeIn(revision_note), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        # Columns occupy the left two thirds; the stage legend and the two summary
        # cards get their own non-overlapping columns to the right of them.
        base_y, unit = -1.55, 0.0250
        xs = np.linspace(-6.05, -0.05, 5)
        ground = Line(np.array([xs[0] - 0.95, base_y, 0]), np.array([xs[-1] + 0.95, base_y, 0]),
                      color=P_WHITE, stroke_width=2, stroke_opacity=0.5)

        # —— The starting quantity ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nutz"))
        columns = [
            _cascade_column(x, [i > s for s in range(4)], base_y=base_y, unit=unit, width=1.15)
            for i, x in enumerate(xs)
        ]
        captions = VGroup()
        for x, text in zip(xs, ("Nutzenergie", "+ Übergabe", "+ Verteilung", "+ Speicherung", "+ Erzeugung")):
            label = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE)
            label.set_opacity(0.85)
            label.move_to(np.array([x, base_y - 0.32, 0]))
            captions.add(label)

        start_card = stat_card("Nutzenergie Q_h", "100 kWh/(m²·a)", color=NUTZ_BLUE)
        start_card.move_to(np.array([5.25, 1.55, 0]))
        self.play(
            FadeOut(revision_note),
            Create(ground), GrowFromEdge(columns[0]["bars"][0], DOWN),
            FadeIn(captions[0]), FadeIn(start_card), run_time=1.4,
        )
        hold_for(self, self.NARRATION, "nutz", used=1.4 + 0.35)

        # —— The four stages stack up ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "stufen"))
        for i in range(1, 5):
            self.play(
                TransformFromCopy(columns[i - 1]["bars"], columns[i]["bars"][:-1]),
                FadeIn(captions[i]),
                run_time=0.55,
            )
            self.play(GrowFromEdge(columns[i]["bars"][-1], DOWN), run_time=0.45)
        legend, legend_leaders = side_labels(
            [(columns[4]["anchors"][k + 1], f"{name}  + {round(value)} kWh", colour)
             for k, (name, value, colour) in enumerate(STAGES)],
            x=xs[-1] + 0.78, align="left", font_size=LABEL_FONT_SIZE - 2,
        )
        self.play(FadeIn(legend), Create(legend_leaders), run_time=0.9)
        hold_for(self, self.NARRATION, "stufen", used=5 * 1.0 + 0.9 + 0.35)

        # —— Final energy ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ende"))
        end_card = stat_card("Endenergie Q_E", f"{round(Q_E_GAS)} kWh/(m²·a)", color=END_RED)
        end_card.move_to(np.array([5.25, 0.42, 0]))
        arrow = Arrow(start_card.get_bottom(), end_card.get_top(), buff=0.08, color=P_WHITE,
                      stroke_width=3, max_tip_length_to_length_ratio=0.25)
        example_note = note_line("Schematische Beispielrechnung — keine Normwerte", color=P_TEAL)
        self.play(GrowArrow(arrow), FadeIn(end_card, shift=DOWN * 0.12), FadeIn(example_note), run_time=1.2)
        hold_for(self, self.NARRATION, "ende", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(start_card), FadeOut(end_card), FadeOut(arrow),
            FadeOut(legend), FadeOut(legend_leaders), FadeOut(captions), FadeOut(ground),
            FadeOut(example_note), *[FadeOut(c["bars"]) for c in columns], run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Übergabe, Verteilung, Speicherung

class Beat2_UebergabeVerteilungSpeicherung(Scene):
    NARRATION = [
        ("intro",
         "Walk the chain backwards through a real building, starting in the room itself.",
         "Gehen wir die Kette rückwärts durch ein echtes Gebäude — beginnend im Raum selbst."),
        ("uebergabe",
         "Emission: how the surface hands heat to the air. A radiator needs hot water, underfloor heating needs far less.",
         "Übergabe: wie die Fläche Wärme an die Luft abgibt. Ein Heizkörper braucht heißes Wasser, eine Fußbodenheizung viel weniger."),
        ("verteilung",
         "Distribution: every pipe metre loses heat. Outside the envelope it is lost for good — inside it partly comes back as a gain.",
         "Verteilung: Jeder Rohrmeter verliert Wärme. Außerhalb der Hülle ist sie weg — innerhalb kommt sie teilweise als Gewinn zurück."),
        ("speicher",
         "Storage: a tank kept warm around the clock loses heat even without any draw-off, for as long as a whole week.",
         "Speicherung: Ein warm gehaltener Speicher verliert auch ohne Zapfung Wärme — selbst eine ganze Woche lang."),
        ("kennzahl",
         "Each stage gets one expense figure: how much has to go in for one unit to come out.",
         "Jede Stufe bekommt eine Aufwandszahl: wie viel hinein muss, damit eine Einheit herauskommt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Übergabe, Verteilung, Speicherung", title)
        din = _din_ref("DIN V 18599-5:2018-09 · -6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # —— Building shell: heated storey over an unheated basement ——
        # One coordinate system for the whole beat: the slab at SLAB_Y divides
        # heated from unheated, and every component is placed against it so
        # nothing straddles the boundary it is supposed to be on one side of.
        SLAB_Y, MAIN_Y = -0.40, -0.98
        shell = Rectangle(width=9.4, height=2.05, color=P_WHITE, stroke_width=3)
        shell.move_to(np.array([-0.45, SLAB_Y + 1.025, 0]))
        slab = Line(shell.get_corner(DL), shell.get_corner(DR), color=P_TEAL, stroke_width=4)
        basement = Rectangle(width=9.4, height=1.30, color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        basement.move_to(np.array([-0.45, SLAB_Y - 0.65, 0]))
        shell_tag = body_text("beheizte Hülle", font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
        # Upper-right corner: the upper-left is where the radiator's flow-temperature
        # label sits, and the right half of the storey stays empty all beat.
        shell_tag.set_opacity(0.85).next_to(shell.get_corner(UR), DL, buff=0.14)
        cellar_tag = body_text("unbeheizter Keller", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE)
        cellar_tag.set_opacity(0.6).next_to(basement.get_corner(DL), UR, buff=0.12)
        self.play(Create(shell), Create(basement), Create(slab),
                  FadeIn(shell_tag), FadeIn(cellar_tag), run_time=1.5)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5)

        # —— Emission ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "uebergabe"))
        radiator = _radiator(np.array([-3.55, 0.78, 0]), width=0.95, height=0.68)
        rad_tag = body_text("Heizkörper · z. B. 55 °C", font_size=LABEL_FONT_SIZE - 3, color=UEBER_RED)
        rad_tag.next_to(radiator, UP, buff=0.16)
        loop = _floor_loop(np.array([1.05, SLAB_Y + 0.06, 0]), width=2.1)
        loop_tag = body_text("Fußbodenheizung · z. B. 35 °C", font_size=LABEL_FONT_SIZE - 3, color=P_CYAN)
        loop_tag.next_to(loop, UP, buff=0.18)
        self.play(Create(radiator), FadeIn(rad_tag), run_time=1.0)
        self.play(Create(loop), FadeIn(loop_tag), run_time=1.0)
        flow_note = note_line("niedrige Vorlauftemperatur → bessere Erzeugereffizienz", color=P_CYAN)
        self.play(FadeIn(flow_note), run_time=0.6)
        hold_for(self, self.NARRATION, "uebergabe", used=1.0 + 1.0 + 0.6 + 0.35)

        # —— Distribution ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verteilung"))
        self.play(FadeOut(flow_note), run_time=0.3)
        riser = VGroup(
            Line(np.array([-3.55, MAIN_Y, 0]), np.array([2.05, MAIN_Y, 0]),
                 color=VERT_ORANGE, stroke_width=7),
            Line(np.array([-3.55, MAIN_Y, 0]), np.array([-3.55, 0.45, 0]),
                 color=VERT_ORANGE, stroke_width=7),
            Line(np.array([1.05, MAIN_Y, 0]), np.array([1.05, SLAB_Y - 0.04, 0]),
                 color=VERT_ORANGE, stroke_width=7),
        )
        pipe_waves = VGroup(*[
            _loss_waves(np.array([x, MAIN_Y + 0.18, 0]), color=VERT_ORANGE, n=2)
            for x in (-1.85, -0.35)
        ])
        inside_note = note_line("Verluste innerhalb der Hülle werden als Gewinn gutgeschrieben", color=VERT_ORANGE)
        self.play(Create(riser), run_time=1.0)
        self.play(LaggedStart(*[Create(w) for w in pipe_waves], lag_ratio=0.25),
                  FadeIn(inside_note), run_time=1.1)
        hold_for(self, self.NARRATION, "verteilung", used=0.3 + 1.0 + 1.1 + 0.35)

        # —— Storage ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "speicher"))
        self.play(FadeOut(inside_note), run_time=0.3)
        tank = _tank(np.array([2.55, SLAB_Y - 0.53, 0]))
        tank_tag = body_text("Speicher", font_size=LABEL_FONT_SIZE - 3, color=SPEI_YELLOW)
        tank_tag.move_to(np.array([2.55, SLAB_Y - 1.15, 0]))
        # _loss_waves opens upward by construction (it reads right on a horizontal
        # pipe) — rotated -90° here so it reads as heat radiating sideways off the
        # tank's flank instead of diagonally off its corner.
        tank_wave_anchor = tank.get_right() + RIGHT * 0.12
        tank_waves = _loss_waves(tank_wave_anchor, color=SPEI_YELLOW, n=3)
        tank_waves.rotate(-PI / 2, about_point=tank_wave_anchor)
        self.play(Create(tank), FadeIn(tank_tag), run_time=1.0)
        self.play(LaggedStart(*[Create(w) for w in tank_waves], lag_ratio=0.2), run_time=0.9)
        hold_for(self, self.NARRATION, "speicher", used=0.3 + 1.0 + 0.9 + 0.35)

        # —— The three expense figures ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kennzahl"))
        row, items = equation_row([
            ("eu", "e_ü ≈ 1,04", UEBER_RED), (None, "·", P_WHITE),
            ("ed", "e_d ≈ 1,08", VERT_ORANGE), (None, "·", P_WHITE),
            ("es", "e_s ≈ 1,06", SPEI_YELLOW), (None, "   alle Aufwandszahlen > 1", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(Create(box), FadeIn(row), run_time=1.1)
        hold_for(self, self.NARRATION, "kennzahl", used=1.1 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(radiator), FadeOut(rad_tag),
            FadeOut(loop), FadeOut(loop_tag), FadeOut(riser), FadeOut(pipe_waves),
            FadeOut(tank), FadeOut(tank_tag), FadeOut(tank_waves),
            FadeOut(shell), FadeOut(basement), FadeOut(slab), FadeOut(shell_tag), FadeOut(cellar_tag),
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Erzeugung: Wirkungsgrad oder Arbeitszahl

class Beat3_Erzeugung(Scene):
    NARRATION = [
        ("intro",
         "The last stage is the biggest — and this is where a combustion boiler and a heat pump differ fundamentally.",
         "Die letzte Stufe ist die größte — hier unterscheiden sich Verbrennungskessel und Wärmepumpe grundlegend."),
        ("kessel",
         "A boiler burns fuel. Part of the heat leaves through the flue, so more always goes in than comes out.",
         "Ein Kessel verbrennt Brennstoff. Ein Teil der Wärme geht durch den Schornstein — es muss immer mehr hinein als heraus."),
        ("brennwert",
         "A condensing boiler cools the flue gas until its water vapour condenses — the latent heat from the fundamentals video becomes usable.",
         "Ein Brennwertkessel kühlt das Abgas, bis Wasserdampf darin kondensiert — die latente Wärme aus dem Grundlagen-Video wird nutzbar."),
        ("waermepumpe",
         "A heat pump burns nothing. It lifts ambient heat, so three quarters of what it delivers is not purchased final energy.",
         "Eine Wärmepumpe verbrennt nichts. Sie hebt Umweltwärme an — drei Viertel der Lieferung sind keine zugekaufte Endenergie."),
        ("jaz",
         "Its seasonal performance factor is the ratio of heat delivered over the year to the electricity that drove it: here a hundred to twenty-six.",
         "Ihre Jahresarbeitszahl ist das Verhältnis von Wärme zu eingesetztem Strom im Jahr: hier hundert zu sechsundzwanzig."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Erzeugung: Wirkungsgrad oder Arbeitszahl", title)
        din = _din_ref("DIN V 18599-5 · VDI 4650-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        # Every quantity in this beat is a segment of an input or an output column,
        # on one shared baseline and one shared scale, so the two technologies can
        # be compared by eye alone.
        base_y, unit, bar_w = -1.28, 0.0195, 1.10
        divider = DashedLine(np.array([0.0, base_y - 0.22, 0]), np.array([0.0, 1.85, 0]),
                             color=P_WHITE, stroke_width=1.6, stroke_opacity=0.3)

        def _caption(text, x, colour):
            label = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=colour)
            label.set_opacity(0.9)
            label.move_to(np.array([x, base_y - 0.32, 0]))
            return label

        # —— Boiler ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kessel"))
        gas_in = stacked_bar([("nutzbar", 100.0, NUTZ_BLUE), ("Abgasverlust", 18.0, P_RED)],
                             base=np.array([-4.95, base_y, 0]), width=bar_w, unit=unit)
        gas_out = stacked_bar([("Wärme", 100.0, NUTZ_BLUE)],
                              base=np.array([-2.35, base_y, 0]), width=bar_w, unit=unit)
        gas_arrow = Arrow(np.array([-4.30, base_y + 0.95, 0]), np.array([-3.00, base_y + 0.95, 0]),
                          buff=0, color=P_WHITE, stroke_width=3.5,
                          max_tip_length_to_length_ratio=0.22)
        gas_caps = VGroup(_caption("Erdgas 118", -4.95, ERZ_AMBER),
                          _caption("Wärme 100", -2.35, NUTZ_BLUE))
        gas_tag = stat_card("Gaskessel", f"e_g ≈ {_de_num(E_ERZEUGUNG_GAS)}", color=ERZ_AMBER)
        gas_tag.move_to(np.array([-3.65, 1.62, 0]))
        flue_tag = body_text("Abgasverlust 18", font_size=LABEL_FONT_SIZE - 3, color=P_RED)
        flue_tag.next_to(gas_in["bars"][1], RIGHT, buff=0.18)
        self.play(Create(divider), run_time=0.4)
        self.play(
            LaggedStart(GrowFromEdge(gas_in["bars"][0], DOWN), GrowFromEdge(gas_in["bars"][1], DOWN),
                        lag_ratio=0.45),
            FadeIn(gas_caps), FadeIn(gas_tag), run_time=1.5,
        )
        self.play(GrowArrow(gas_arrow), GrowFromEdge(gas_out["bars"][0], DOWN),
                  FadeIn(flue_tag), run_time=1.1)
        hold_for(self, self.NARRATION, "kessel", used=0.4 + 1.5 + 1.1 + 0.35)

        # —— Condensing recovers part of the flue loss ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "brennwert"))
        recovered = Rectangle(width=bar_w, height=7.0 * unit, color=P_CYAN,
                              fill_color=P_CYAN, fill_opacity=0.8, stroke_width=1.2)
        recovered.move_to(np.array([-4.95, base_y + (100.0 + 3.5) * unit, 0]))
        cond_note = note_line("Brennwert: Wasserdampf im Abgas kondensiert → latente Wärme wird nutzbar",
                              color=P_CYAN)
        self.play(
            Transform(gas_in["bars"][1], recovered),
            flue_tag.animate.become(
                body_text("Restverlust 7", font_size=LABEL_FONT_SIZE - 3, color=P_CYAN)
                .next_to(recovered, RIGHT, buff=0.18)),
            gas_caps[0].animate.become(_caption("Erdgas 107", -4.95, ERZ_AMBER)),
            gas_tag.animate.become(
                stat_card("Brennwertkessel", f"e_g ≈ {_de_num(E_ERZEUGUNG_GAS_BRENNWERT)}", color=P_CYAN)
                .move_to(gas_tag)),
            FadeIn(cond_note), run_time=1.5,
        )
        hold_for(self, self.NARRATION, "brennwert", used=1.5 + 0.35)

        # —— Heat pump ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "waermepumpe"))
        self.play(FadeOut(cond_note), run_time=0.3)
        hp_in = stacked_bar([("Strom", 26.0, P_YELLOW), ("Umweltwärme", 74.0, AMBIENT_GREEN)],
                            base=np.array([2.35, base_y, 0]), width=bar_w, unit=unit)
        hp_out = stacked_bar([("Wärme", 100.0, NUTZ_BLUE)],
                             base=np.array([4.95, base_y, 0]), width=bar_w, unit=unit)
        hp_arrow = Arrow(np.array([3.00, base_y + 0.95, 0]), np.array([4.30, base_y + 0.95, 0]),
                         buff=0, color=P_WHITE, stroke_width=3.5,
                         max_tip_length_to_length_ratio=0.22)
        hp_caps = VGroup(_caption("Strom 26 + Umwelt 74", 2.35, P_WHITE),
                         _caption("Wärme 100", 4.95, NUTZ_BLUE))
        hp_tag = stat_card("Wärmepumpe", f"e_g ≈ {_de_num(E_ERZEUGUNG_WP)}", color=AMBIENT_GREEN)
        hp_tag.move_to(np.array([3.65, 1.62, 0]))
        hp_labels, hp_leaders = side_labels(
            [(hp_in["anchors"][0], "zugekauft", P_YELLOW),
             (hp_in["anchors"][1], "nicht zugekauft", AMBIENT_GREEN)],
            x=1.75, align="right", font_size=LABEL_FONT_SIZE - 2,
        )
        self.play(
            LaggedStart(GrowFromEdge(hp_in["bars"][0], DOWN), GrowFromEdge(hp_in["bars"][1], DOWN),
                        lag_ratio=0.45),
            FadeIn(hp_caps), FadeIn(hp_tag), run_time=1.6,
        )
        self.play(GrowArrow(hp_arrow), GrowFromEdge(hp_out["bars"][0], DOWN),
                  FadeIn(hp_labels), Create(hp_leaders), run_time=1.2)
        hold_for(self, self.NARRATION, "waermepumpe", used=0.3 + 1.6 + 1.2 + 0.35)

        # —— The performance factor ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "jaz"))
        row, items = equation_row([
            ("jaz", "JAZ", AMBIENT_GREEN), (None, "=", P_WHITE),
            ("out", "Wärme ab", NUTZ_BLUE), (None, "/", P_WHITE),
            ("in", "Strom zu", P_YELLOW), (None, "  ≈ 3,8   →   e_g = 1 / JAZ", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        row, box = formula_panel(row)
        self.play(Create(box), FadeIn(row), run_time=1.1)
        hold_for(self, self.NARRATION, "jaz", used=1.1 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(divider), FadeOut(flue_tag),
            FadeOut(gas_in["bars"]), FadeOut(gas_out["bars"]), FadeOut(gas_caps), FadeOut(gas_tag),
            FadeOut(gas_arrow), FadeOut(hp_in["bars"]), FadeOut(hp_out["bars"]), FadeOut(hp_caps),
            FadeOut(hp_tag), FadeOut(hp_arrow), FadeOut(hp_labels), FadeOut(hp_leaders),
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 4 – Die Anlagenaufwandszahl e_p

class Beat4_Anlagenaufwandszahl(Scene):
    NARRATION = [
        ("intro",
         "Multiply the four stages together and the whole plant collapses into a single number.",
         "Multipliziert man die vier Stufen, schrumpft die ganze Anlage auf eine einzige Zahl."),
        ("kette",
         "That is the plant expense figure: how much final energy is required per unit of useful heat.",
         "Das ist die Anlagenaufwandszahl: wie viel Endenergie je Einheit Nutzenergie erforderlich ist."),
        ("gas",
         "For the gas boiler it lands at one point four — a hundred becomes about a hundred and forty.",
         "Beim Gaskessel liegt sie bei eins Komma vier — aus hundert werden etwa hundertvierzig."),
        ("wp",
         "For the heat pump it drops below one, because three quarters of the delivered heat came from the environment.",
         "Bei der Wärmepumpe fällt sie unter eins — drei Viertel der gelieferten Wärme kamen aus der Umwelt."),
        ("brueck",
         "But thirty-one kilowatt-hours of electricity and a hundred and forty of gas are still not comparable. That is the next chapter.",
         "Doch einunddreißig Kilowattstunden Strom und hundertvierzig Gas sind noch nicht vergleichbar. Das ist das nächste Kapitel."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Anlagenaufwandszahl e_p", title)
        din = _din_ref("DIN V 18599-5:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        # —— The product ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kette"))
        factors = card_grid([
            ("Übergabe", "e_ü", UEBER_RED),
            ("Verteilung", "e_d", VERT_ORANGE),
            ("Speicherung", "e_s", SPEI_YELLOW),
            ("Erzeugung", "e_g", ERZ_AMBER),
        ], rows=1, buff=0.30)
        factors.move_to(UP * 1.30)
        dots = VGroup(*[
            body_text("·", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to(
                (factors[i].get_right() + factors[i + 1].get_left()) / 2)
            for i in range(3)
        ])
        row, items = equation_row([
            ("ep", "e_p", P_WHITE), (None, "=", P_WHITE),
            ("eu", "e_ü", UEBER_RED), (None, "·", P_WHITE),
            ("ed", "e_d", VERT_ORANGE), (None, "·", P_WHITE),
            ("es", "e_s", SPEI_YELLOW), (None, "·", P_WHITE),
            ("eg", "e_g", ERZ_AMBER),
        ])
        row, box = formula_panel(row)
        schematic_note = note_line(
            "Schematische Darstellung — DIN/TS 18599-5 rechnet deutlich detaillierter", color=P_TEAL,
        )
        self.play(
            LaggedStart(*[FadeIn(c, shift=DOWN * 0.1) for c in factors], lag_ratio=0.15),
            FadeIn(dots), FadeIn(schematic_note), run_time=1.5,
        )
        self.play(Create(box), FadeIn(row), run_time=1.0)
        hold_for(self, self.NARRATION, "kette", used=1.5 + 1.0 + 0.35)

        # —— Gas result ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gas"))
        gas_card = card_grid([
            ("Gaskessel", f"e_p ≈ {_de_num(E_P_GAS)}", ERZ_AMBER),
            ("Endenergie", f"{round(Q_E_GAS)} kWh/(m²·a)", END_RED),
        ], rows=1, buff=0.30)
        gas_card.move_to(np.array([0.0, -0.20, 0]))
        self.play(FadeOut(schematic_note), FadeIn(gas_card, shift=UP * 0.12), run_time=1.2)
        hold_for(self, self.NARRATION, "gas", used=1.2 + 0.35)

        # —— Heat pump result ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wp"))
        hp_card = card_grid([
            ("Wärmepumpe", f"e_p ≈ {_de_num(E_P_WP)}", AMBIENT_GREEN),
            ("Endenergie", f"{round(Q_E_WP)} kWh/(m²·a)", AMBIENT_GREEN),
        ], rows=1, buff=0.30)
        hp_card.move_to(np.array([0.0, -1.20, 0]))
        self.play(FadeIn(hp_card, shift=UP * 0.12), run_time=1.2)
        why = note_line("e_p < 1 nur, weil Umweltwärme ohne zugekaufte Endenergie dazukommt", color=AMBIENT_GREEN)
        self.play(FadeIn(why), run_time=0.7)
        hold_for(self, self.NARRATION, "wp", used=1.2 + 0.7 + 0.35)

        # —— Bridge ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "brueck"))
        bridge_row, _bridge_items = equation_row([
            ("qe", "Q_E", END_RED), (None, "=", P_WHITE),
            ("qh", "Q_h", NUTZ_BLUE), (None, "·", P_WHITE),
            ("ep", "e_p", P_WHITE), (None, "   →  aber 1 kWh Strom ≠ 1 kWh Gas", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        bridge_row, bridge_box = formula_panel(bridge_row)
        self.play(
            FadeOut(why),
            ReplacementTransform(box, bridge_box), ReplacementTransform(row, bridge_row),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "brueck", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(bridge_box), FadeOut(bridge_row), FadeOut(factors),
            FadeOut(dots), FadeOut(gas_card), FadeOut(hp_card), run_time=0.6,
        )
        self.wait(0.4)

#endregion
