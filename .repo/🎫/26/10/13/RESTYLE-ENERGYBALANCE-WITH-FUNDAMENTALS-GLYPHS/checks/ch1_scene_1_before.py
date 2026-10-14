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
    chip, note_line, card_grid, side_labels, stacked_bar, fit_band,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ One persistent chapter title — written once on Beat 1, re-added on later beats.
TITLE_DE = "Die Energiebilanz des Gebäudes"

# Carried over unchanged from the Heating and Cooling series so a viewer who
# watched those recognises each flow by its colour before reading its label.
LOSS_BLUE = "#38BDF8"     # Transmission — Heating/2_conduction
VENT_BLUE = "#1D4ED8"     # Lüftung — Heating/3_convection
SOLAR_YELLOW = "#FDE047"  # Solare Gewinne — Heating/5_solar_heat_gain
INT_ORANGE = "#F59E0B"    # Interne Gewinne — Heating/4_internal_heat_gain
ETA_GREEN = "#22C55E"     # Ausnutzungsgrad
DEMAND_RED = "#EF4444"    # Was die Anlage liefern muss

# Mid-point of the free band between the note line and the formula panel.
CONTENT_CENTER = UP * 0.16


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as every Heating/Cooling beat
    (``Cooling/3_transmission_humidity/scene_3.py``) — a dim footnote that
    never competes with the diagram.
    """
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs
# The through-line house: the same cross-section the Cooling series draws, so
# the building under discussion never changes shape between videos.

def _build_house(center=ORIGIN, *, width=3.4, height=2.3, roof=1.0):
    """🏠 Line-art house cross-section with a teal floor slab."""
    center = np.array(center, dtype=float)
    bl = center + LEFT * (width / 2) + DOWN * (height / 2)
    br = center + RIGHT * (width / 2) + DOWN * (height / 2)
    tl = center + LEFT * (width / 2) + UP * (height / 2)
    tr = center + RIGHT * (width / 2) + UP * (height / 2)
    peak = center + UP * (height / 2 + roof)

    floor = Line(bl + LEFT * 0.45, br + RIGHT * 0.45, color=P_TEAL, stroke_width=5)
    shell = VGroup(
        Line(bl, tl, color=P_WHITE, stroke_width=4),
        Line(tl, peak, color=P_WHITE, stroke_width=4),
        Line(peak, tr, color=P_WHITE, stroke_width=4),
        Line(tr, br, color=P_WHITE, stroke_width=4),
    )
    window = VGroup(
        Square(side_length=0.62, color=P_CYAN, stroke_width=2),
    )
    window.move_to(center + RIGHT * (width / 2) + LEFT * 0.42)
    window.add(Line(window[0].get_top(), window[0].get_bottom(), color=P_CYAN, stroke_width=1.4))
    return {
        "group": VGroup(floor, shell, window), "floor": floor, "shell": shell, "window": window,
        "center": center, "width": width, "height": height,
        "left": center + LEFT * (width / 2), "right": center + RIGHT * (width / 2),
        "top": peak, "bottom": center + DOWN * (height / 2),
    }


def _boundary(house, *, pad=0.34, color=P_TEAL):
    """⭕ Dashed thermal envelope — the balance boundary every flow has to cross."""
    box = RoundedRectangle(
        corner_radius=0.16,
        width=house["width"] + 2 * pad,
        height=(house["top"][1] - house["bottom"][1]) + 2 * pad,
        color=color, stroke_width=2.5,
    )
    box.move_to((house["top"] + house["bottom"]) / 2)
    return DashedVMobject(box, num_dashes=54, dashed_ratio=0.55)


def _flow_arrow(start, end, color, *, width=5.0):
    """➡️ One heat flow crossing the envelope."""
    return Arrow(
        np.array(start, dtype=float), np.array(end, dtype=float),
        buff=0, color=color, stroke_width=width,
        max_tip_length_to_length_ratio=0.28, tip_length=0.22,
    )


def _person(color=INT_ORANGE, scale=1.0):
    """🧍 The same reduced person glyph the Heating series uses for internal gains."""
    head = Circle(radius=0.11, color=color, fill_opacity=0.45, stroke_width=2).shift(UP * 0.16)
    torso = Arc(radius=0.26, start_angle=PI * 0.15, angle=PI * 0.7, color=color, stroke_width=2).rotate(PI)
    return VGroup(head, torso).scale(scale)


def _sun(pos, *, scale=1.0):
    """☀️ Layered sun disc — identical to the Heating/Cooling solar beats."""
    pos = np.array(pos, dtype=float)
    core = Dot(pos, radius=0.30, color=P_YELLOW)
    glow = Dot(pos, radius=0.46, color=P_YELLOW, fill_opacity=0.30)
    burst = VGroup(*[
        Line(pos + np.array([np.cos(a) * 0.38, np.sin(a) * 0.38, 0]),
             pos + np.array([np.cos(a) * 0.60, np.sin(a) * 0.60, 0]),
             color=P_YELLOW, stroke_width=2)
        for a in np.linspace(0, TAU, 10, endpoint=False)
    ])
    return VGroup(glow, core, burst).scale(scale, about_point=pos)
#endregion


#region Beat 1 – Bilanzgrenze & Energieerhaltung

class Beat1_Bilanzgrenze(Scene):
    NARRATION = [
        ("intro",
         "In three videos we calculated single heat flows. Now we assemble them into one balance.",
         "In drei Videos haben wir einzelne Wärmeströme berechnet. Jetzt fügen wir sie zu einer Bilanz zusammen."),
        ("huelle",
         "A balance needs a boundary: the thermal envelope. Everything that crosses it over the year is an energy flow we can add up.",
         "Eine Bilanz braucht eine Grenze: die thermische Hülle. Alles, was sie übers Jahr kreuzt, ist ein Energiefluss, den wir aufsummieren."),
        ("erhaltung",
         "From the fundamentals we know energy is conserved — so at a constant indoor temperature, everything leaving must be replaced.",
         "Aus den Grundlagen wissen wir: Energie bleibt erhalten. Bei konstanter Innentemperatur muss alles Abfließende ersetzt werden."),
        ("bedarf",
         "What the free gains don't cover, the heating has to deliver. That remainder is the useful heating demand — Heizwärmebedarf.",
         "Was die freien Gewinne nicht decken, muss die Heizung liefern. Dieser Rest ist der Heizwärmebedarf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Bilanzgrenze & Energieerhaltung", title)
        din = _din_ref("DIN V 18599-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # Sized so the dashed envelope tops out at CONTENT_TOP and still leaves
        # room under the house for the supply arrow above the formula panel.
        house = _build_house(UP * 0.11, width=2.8, height=1.5, roof=0.70)
        self.play(Create(house["group"]), run_time=1.4)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.4)

        # —— The envelope ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "huelle"))
        envelope = _boundary(house)
        env_note = note_line("Alles, was diese Grenze übers Jahr kreuzt, ist Energie.", color=P_WHITE)
        self.play(Create(envelope), FadeIn(env_note), run_time=1.3)
        hold_for(self, self.NARRATION, "huelle", used=1.3 + 0.35)

        # —— Flows out and in ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "erhaltung"))
        x_out = envelope.get_left()[0]
        x_in = envelope.get_right()[0]
        rows = (house["center"][1] + 0.45, house["center"][1] - 0.45)
        out_arrows = VGroup(*[
            _flow_arrow([x_out, y, 0], [x_out - 1.05, y, 0], LOSS_BLUE) for y in rows
        ])
        # Solar arrives from outside; internal gains are released inside the
        # envelope — drawing both as inward arrows would misstate where they
        # come from, so the occupants sit in the room and only their label
        # reaches out to the same column.
        in_arrows = VGroup(_flow_arrow([x_in + 1.05, rows[0], 0], [x_in, rows[0], 0], SOLAR_YELLOW))
        occupants = _person(INT_ORANGE, 1.0).move_to(
            np.array([house["center"][0] + 0.45, rows[1] - 0.05, 0]))
        out_labels, out_leaders = side_labels(
            [(a.get_end(), t, LOSS_BLUE) for a, t in zip(out_arrows, ("Transmission", "Lüftung"))],
            x=x_out - 1.25, align="right",
        )
        in_labels, in_leaders = side_labels(
            [(in_arrows[0].get_start(), "Solare Gewinne", SOLAR_YELLOW),
             (occupants.get_right(), "Interne Gewinne", INT_ORANGE)],
            x=x_in + 1.25, align="left",
        )
        self.play(
            LaggedStart(*[GrowArrow(a) for a in out_arrows], lag_ratio=0.2),
            GrowArrow(in_arrows[0]), FadeIn(occupants, scale=0.7),
            run_time=1.4,
        )
        self.play(
            FadeIn(out_labels), Create(out_leaders),
            FadeIn(in_labels), Create(in_leaders),
            run_time=0.9,
        )
        hold_for(self, self.NARRATION, "erhaltung", used=1.4 + 0.9 + 0.35)

        # —— The remainder the plant has to supply ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf"))
        supply = _flow_arrow(
            [house["center"][0], envelope.get_bottom()[1] - 0.78, 0],
            [house["center"][0], envelope.get_bottom()[1], 0],
            DEMAND_RED, width=6.0,
        )
        supply_label = body_text("Heizwärmebedarf Q_h", font_size=BODY_FONT_SIZE, color=DEMAND_RED)
        supply_label.next_to(supply, LEFT, buff=0.30)

        row, items = equation_row([
            ("qh", "Q_h", DEMAND_RED), (None, "+", P_WHITE),
            ("quelle", "Q_Quelle", SOLAR_YELLOW), (None, "=", P_WHITE),
            ("senke", "Q_Senke", LOSS_BLUE), (None, "  [kWh/a]", P_TEAL),
        ])
        row, box = formula_panel(row)
        self.play(GrowArrow(supply), FadeIn(supply_label), run_time=0.9)
        self.play(Create(box), FadeIn(row), run_time=1.0)
        ring = highlight_param(items, "qh", color=DEMAND_RED)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "bedarf", used=0.9 + 1.0 + 0.5 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(ring), FadeOut(box), FadeOut(row),
            FadeOut(VGroup(out_labels, out_leaders, in_labels, in_leaders, out_arrows, in_arrows)),
            FadeOut(occupants),
            FadeOut(supply), FadeOut(supply_label), FadeOut(envelope), FadeOut(env_note),
            FadeOut(house["group"]),
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Die Verlustseite (Q_T + Q_V)

class Beat2_Verlustseite(Scene):
    NARRATION = [
        ("intro",
         "The loss side first — and it is exactly the two mechanisms the Heating video derived.",
         "Zuerst die Verlustseite — genau die beiden Mechanismen aus dem Heizen-Video."),
        ("transmission",
         "Transmission through every envelope surface: U times area times the mean temperature difference, summed over the hours of the month — a schematic form of H times degree-hours.",
         "Transmission durch jede Hüllfläche: U mal Fläche mal mittlere Temperaturdifferenz, über die Stunden des Monats — schematisch wie H mal Gradstunden."),
        ("bruecken",
         "Thermal bridges are added as a lump surcharge on the U-values — 0.05 watts per square metre kelvin when a thermal-bridge check is available, more without one.",
         "Wärmebrücken kommen als Pauschalzuschlag auf die U-Werte — 0,05 W/(m²·K) mit Wärmebrückennachweis, sonst höher."),
        ("lueftung",
         "Ventilation carries the second stream out: the air change rate replaces warm indoor air with cold outdoor air.",
         "Die Lüftung trägt den zweiten Strom hinaus: Der Luftwechsel ersetzt warme Innenluft durch kalte Außenluft."),
        ("summe",
         "Together they form the sink side of the balance — every kilowatt-hour the building loses over the year.",
         "Zusammen bilden sie die Senkenseite der Bilanz — jede Kilowattstunde, die das Gebäude im Jahr verliert."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Verlustseite: Transmission & Lüftung", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        house = _build_house(LEFT * 3.8 + CONTENT_CENTER + DOWN * 0.1, width=2.9, height=2.0, roof=0.85)
        self.play(Create(house["group"]), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2)

        # —— The stacked loss column is built one contribution at a time ——
        segments = [
            ("Außenwand", 2.9, LOSS_BLUE),
            ("Dach", 1.7, LOSS_BLUE),
            ("Fenster", 2.2, P_CYAN),
            ("Boden", 1.1, P_TEAL),
            ("Wärmebrücken", 0.8, P_ORANGE),
            ("Lüftung", 2.6, VENT_BLUE),
        ]
        stack = stacked_bar(segments, base=RIGHT * 1.15 + DOWN * 1.55, width=1.15, unit=0.30)
        # Boden and Wärmebrücken sit only 0.285 apart — the default 0.40 min_gap
        # cascades from Lüftung above and bends the Boden leader; a tighter gap
        # tuned to this stack's own label heights keeps every leader horizontal.
        labels, leaders = side_labels(
            [(a, f"{name}", col) for a, (name, _v, col) in zip(stack["anchors"], segments)],
            x=2.60, align="left", min_gap=0.30, pad=0.08,
        )

        # —— Transmission ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "transmission"))
        self.play(house["shell"].animate.set_color(LOSS_BLUE), run_time=0.7)
        self.play(
            LaggedStart(*[GrowFromEdge(stack["bars"][i], DOWN) for i in (0, 1, 2, 3)], lag_ratio=0.25),
            LaggedStart(*[FadeIn(labels[i]) for i in (0, 1, 2, 3)], lag_ratio=0.25),
            LaggedStart(*[Create(leaders[i]) for i in (0, 1, 2, 3)], lag_ratio=0.25),
            run_time=2.0,
        )
        row, items = equation_row([
            ("qt", "Q_T", LOSS_BLUE), (None, "=", P_WHITE),
            ("ua", "Σ (U · A)", P_ORANGE), (None, "·", P_WHITE),
            ("dt", "Δθ", P_BLUE), (None, "·", P_WHITE),
            ("t", "t", P_TEAL), (None, "  [kWh]", P_TEAL),
        ])
        row, box = formula_panel(row)
        self.play(Create(box), FadeIn(row), run_time=0.9)
        hold_for(self, self.NARRATION, "transmission", used=0.7 + 2.0 + 0.9 + 0.35)

        # —— Wärmebrücken ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bruecken"))
        bridge_ring = highlight_param(items, "ua", color=P_ORANGE)
        self.play(
            GrowFromEdge(stack["bars"][4], DOWN), FadeIn(labels[4]), Create(leaders[4]),
            Create(bridge_ring), run_time=1.1,
        )
        bridge_note = note_line("Wärmebrückenzuschlag ΔU_WB ≈ 0,05 W/(m²·K)", color=P_ORANGE)
        self.play(FadeIn(bridge_note), run_time=0.6)
        hold_for(self, self.NARRATION, "bruecken", used=1.1 + 0.6 + 0.35)

        # —— Lüftung ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lueftung"))
        self.play(FadeOut(bridge_ring), FadeOut(bridge_note), run_time=0.3)
        air = VGroup(*[
            CurvedArrow(
                house["window"].get_center() + RIGHT * 0.1 + UP * dy,
                house["window"].get_center() + RIGHT * 1.15 + UP * (dy + 0.25),
                angle=-TAU / 8, color=VENT_BLUE, stroke_width=4,
            )
            for dy in (0.25, -0.25)
        ])
        self.play(
            LaggedStart(*[Create(a) for a in air], lag_ratio=0.25),
            GrowFromEdge(stack["bars"][5], DOWN), FadeIn(labels[5]), Create(leaders[5]),
            run_time=1.5,
        )
        vent_row, vent_items = equation_row([
            ("qv", "Q_V", VENT_BLUE), (None, "=", P_WHITE),
            ("rho", "ρ · c", P_GREEN), (None, "·", P_WHITE),
            ("n", "n · V", P_YELLOW), (None, "·", P_WHITE),
            ("dt", "Δθ", P_BLUE), (None, "·", P_WHITE),
            ("t", "t", P_TEAL), (None, "  [kWh]", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        vent_row, vent_box = formula_panel(vent_row)
        self.play(
            ReplacementTransform(box, vent_box), ReplacementTransform(row, vent_row),
            run_time=0.9,
        )
        hold_for(self, self.NARRATION, "lueftung", used=0.3 + 1.5 + 0.9 + 0.35)

        # —— The sink side ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summe"))
        brace = Brace(stack["bars"], LEFT, color=LOSS_BLUE, buff=0.18)
        sink_label = body_text("Q_Senke", font_size=BODY_FONT_SIZE, color=LOSS_BLUE)
        sink_label.next_to(brace, LEFT, buff=0.16)
        sum_row, _sum_items = equation_row([
            ("senke", "Q_Senke", LOSS_BLUE), (None, "=", P_WHITE),
            ("qt", "Q_T", LOSS_BLUE), (None, "+", P_WHITE),
            ("qv", "Q_V", VENT_BLUE), (None, "  [kWh/a]", P_TEAL),
        ])
        sum_row, sum_box = formula_panel(sum_row)
        self.play(
            GrowFromCenter(brace), FadeIn(sink_label),
            ReplacementTransform(vent_box, sum_box), ReplacementTransform(vent_row, sum_row),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "summe", used=1.2 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(sum_box), FadeOut(sum_row), FadeOut(brace), FadeOut(sink_label),
            FadeOut(stack["bars"]), FadeOut(labels), FadeOut(leaders), FadeOut(air),
            FadeOut(house["group"]),
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Die Gewinnseite & der Ausnutzungsgrad

class Beat3_Gewinnseite(Scene):
    NARRATION = [
        ("intro",
         "Now the other pan of the scale: the heat the building gets for free.",
         "Jetzt die andere Waagschale: die Wärme, die das Gebäude kostenlos bekommt."),
        ("solar",
         "Solar gains through the glazing — window area times the g-value times the irradiation on that facade.",
         "Solare Gewinne durch die Verglasung — Fensterfläche mal g-Wert mal Einstrahlung auf diese Fassade."),
        ("intern",
         "Internal gains from people, appliances and lighting: DIN V 18599-10 tables them by use, here as a watt per square metre.",
         "Interne Gewinne von Personen, Geräten und Licht: DIN V 18599-10 tabelliert sie nutzungsabhängig, hier als Wert je Quadratmeter."),
        ("eta",
         "But not every free kilowatt-hour helps. The utilization factor eta_h depends on the gain-to-loss ratio — here a schematic 0.8. Gains that arrive when no heat is needed only overheat the room.",
         "Doch nicht jede freie Kilowattstunde hilft. Der Ausnutzungsgrad η_h hängt vom Gewinn-Verlust-Verhältnis ab — hier schematisch 0,8. Gewinne ohne Wärmebedarf überhitzen nur den Raum."),
        ("bilanz",
         "So the losses are covered by the usable gains plus whatever the heating adds. That remainder is the heating demand Q_h.",
         "Die Verluste werden also von den nutzbaren Gewinnen plus der Heizung gedeckt. Dieser Rest ist der Heizwärmebedarf Q_h."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Gewinnseite & der Ausnutzungsgrad", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # Column geometry — one base line and one scale for every bar in the beat,
        # sized so the tallest stack stops below CONTENT_TOP and every caption
        # stays above the formula panel.
        base_y, unit = -1.18, 0.27
        x_sink, x_gain, x_mix = -3.60, -1.15, -1.15
        v_sink, v_int, v_sol, eta = 11.3, 3.0, 4.4, 0.82

        sink = stacked_bar([("Senke", v_sink, LOSS_BLUE)], base=np.array([x_sink, base_y, 0]),
                           width=1.25, unit=unit, opacity=0.55)
        sink_cap = body_text("Q_Senke", font_size=BODY_FONT_SIZE, color=LOSS_BLUE)
        sink_cap.move_to(np.array([x_sink, base_y - 0.32, 0]))
        ground = Line(np.array([x_sink - 1.0, base_y, 0]), np.array([x_mix + 1.0, base_y, 0]),
                      color=P_WHITE, stroke_width=2, stroke_opacity=0.5)
        self.play(Create(ground), GrowFromEdge(sink["bars"][0], DOWN), FadeIn(sink_cap), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2)

        gains = stacked_bar([("intern", v_int, INT_ORANGE), ("solar", v_sol, SOLAR_YELLOW)],
                            base=np.array([x_gain, base_y, 0]), width=1.25, unit=unit)
        gain_cap = body_text("Q_Quelle", font_size=BODY_FONT_SIZE, color=SOLAR_YELLOW)
        gain_cap.move_to(np.array([x_gain, base_y - 0.32, 0]))

        # —— Solar gains ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar"))
        sun = _sun([5.2, 1.35, 0], scale=0.9)
        window = Square(side_length=0.75, color=P_CYAN, stroke_width=2.5).move_to([3.3, -0.05, 0])
        window.add(Line(window.get_top(), window.get_bottom(), color=P_CYAN, stroke_width=1.5))
        self.play(FadeIn(sun, scale=0.7), Create(window), run_time=0.9)
        self.play(FadeIn(gain_cap), run_time=0.6)
        solar_only = Rectangle(width=1.25, height=v_sol * unit, color=SOLAR_YELLOW,
                               fill_color=SOLAR_YELLOW, fill_opacity=0.78, stroke_width=1.2)
        solar_only.move_to(np.array([x_gain, base_y + v_sol * unit / 2, 0]))
        self.play(GrowFromEdge(solar_only, DOWN), run_time=1.0)
        solar_row, _solar_items = equation_row([
            ("qs", "Q_S", SOLAR_YELLOW), (None, "=", P_WHITE),
            ("a", "A_W", P_CYAN), (None, "·", P_WHITE),
            ("g", "g", P_ORANGE), (None, "·", P_WHITE),
            ("f", "F_S", P_TEAL), (None, "·", P_WHITE),
            ("i", "I_S", P_YELLOW), (None, "  [kWh]", P_TEAL),
        ])
        solar_row, solar_box = formula_panel(solar_row)
        self.play(Create(solar_box), FadeIn(solar_row), run_time=0.9)
        hold_for(self, self.NARRATION, "solar", used=0.9 + 0.6 + 1.0 + 0.9 + 0.35)

        # —— Internal gains stack underneath ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "intern"))
        people = VGroup(_person(INT_ORANGE), _person(INT_ORANGE))
        people.arrange(RIGHT, buff=0.38).move_to([3.3, -1.25, 0])
        self.play(
            FadeIn(people, scale=0.7),
            GrowFromEdge(gains["bars"][0], DOWN),
            solar_only.animate.move_to(np.array([x_gain, base_y + (v_int + v_sol / 2) * unit, 0])),
            run_time=1.4,
        )
        int_row, _int_items = equation_row([
            ("qi", "Q_I", INT_ORANGE), (None, "=", P_WHITE),
            ("q", "q_I", P_ORANGE), (None, "·", P_WHITE),
            ("a", "A_N", P_CYAN), (None, "·", P_WHITE),
            ("t", "t", P_TEAL), (None, "   q_I: normierter Pauschalwert je m²", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        int_row, int_box = formula_panel(int_row)
        self.play(ReplacementTransform(solar_box, int_box), ReplacementTransform(solar_row, int_row), run_time=0.9)
        hold_for(self, self.NARRATION, "intern", used=1.4 + 0.9 + 0.35)

        # —— Utilisation factor cuts the gain column down ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "eta"))
        self.play(
            FadeOut(sun), FadeOut(window), FadeOut(people),
            FadeOut(int_box), FadeOut(int_row), run_time=0.5,
        )
        v_use = (v_int + v_sol) * eta
        usable = Rectangle(width=1.25, height=v_use * unit, color=ETA_GREEN,
                           fill_color=ETA_GREEN, fill_opacity=0.85, stroke_width=1.2)
        usable.move_to(np.array([x_gain, base_y + v_use * unit / 2, 0]))
        spill = DashedLine(
            np.array([x_gain - 0.95, base_y + (v_int + v_sol) * unit, 0]),
            np.array([x_gain + 0.95, base_y + (v_int + v_sol) * unit, 0]),
            color=P_RED, stroke_width=2,
        )
        spill_note = note_line("nicht nutzbare Gewinne → Überhitzung", color=P_RED)
        self.play(
            FadeOut(solar_only), FadeOut(gains["bars"][0]),
            FadeIn(usable), Create(spill), FadeIn(spill_note), run_time=1.3,
        )
        eta_row, eta_items = equation_row([
            ("eta", "η_h", ETA_GREEN), (None, "· Q_Quelle = nutzbarer Anteil", P_WHITE),
            (None, "   η_h schematisch ≈ 0,8", P_TEAL),
        ], font_size=BODY_FONT_SIZE)
        eta_row, eta_box = formula_panel(eta_row)
        self.play(Create(eta_box), FadeIn(eta_row), run_time=0.9)
        hold_for(self, self.NARRATION, "eta", used=0.5 + 1.3 + 0.9 + 0.35)

        # —— The balance closes: usable gains + heating = losses ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bilanz"))
        self.play(FadeOut(spill), FadeOut(spill_note), FadeOut(gain_cap), run_time=0.4)
        v_dem = v_sink - v_use
        demand = Rectangle(width=1.25, height=v_dem * unit, color=DEMAND_RED,
                           fill_color=DEMAND_RED, fill_opacity=0.85, stroke_width=1.2)
        demand.move_to(np.array([x_mix, base_y + (v_use + v_dem / 2) * unit, 0]))
        level = DashedLine(
            np.array([x_sink + 0.70, base_y + v_sink * unit, 0]),
            np.array([x_mix + 0.75, base_y + v_sink * unit, 0]),
            color=P_WHITE, stroke_width=1.8, stroke_opacity=0.55,
        )
        mix_labels, mix_leaders = side_labels(
            [(usable.get_right(), "η_h · Q_Quelle — freie Wärme", ETA_GREEN),
             (demand.get_center(), "Q_h — Heizwärmebedarf", DEMAND_RED)],
            x=x_mix + 0.85, align="left",
        )
        self.play(GrowFromEdge(demand, DOWN), Create(level), run_time=1.3)
        self.play(FadeIn(mix_labels), Create(mix_leaders), run_time=0.8)

        bal_row, bal_items = equation_row([
            ("qh", "Q_h", DEMAND_RED), (None, "=", P_WHITE),
            ("senke", "Q_Senke", LOSS_BLUE), (None, "−", P_WHITE),
            ("eta", "η_h", ETA_GREEN), (None, "·", P_WHITE),
            ("quelle", "Q_Quelle", SOLAR_YELLOW),
        ])
        bal_row, bal_box = formula_panel(bal_row)
        self.play(ReplacementTransform(eta_box, bal_box), ReplacementTransform(eta_row, bal_row), run_time=1.0)
        ring = highlight_param(bal_items, "qh", color=DEMAND_RED)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "bilanz", used=0.4 + 1.3 + 0.8 + 1.0 + 0.5 + 0.35)

        self.play(
            FadeOut(caption), FadeOut(ring), FadeOut(bal_box), FadeOut(bal_row),
            FadeOut(sink["bars"]), FadeOut(sink_cap), FadeOut(usable), FadeOut(demand),
            FadeOut(mix_labels), FadeOut(mix_leaders), FadeOut(level), FadeOut(ground),
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 4 – Dieselbe Bilanz im Sommer

class Beat4_WinterUndSommer(Scene):
    NARRATION = [
        ("intro",
         "The same balance runs all year — only the two pans change height.",
         "Dieselbe Bilanz läuft das ganze Jahr — nur die beiden Waagschalen ändern ihre Höhe."),
        ("winter",
         "In January losses dominate: the gain pan is small, and the gap is heating demand.",
         "Im Januar dominieren die Verluste: Die Gewinnschale ist klein, und die Lücke ist Heizwärmebedarf."),
        ("sommer",
         "In July it flips. Sun and internal loads exceed the losses, and the surplus is cooling energy demand that has to be removed.",
         "Im Juli kippt es. Sonne und interne Lasten übersteigen die Verluste — der Überschuss ist Kühlenergiebedarf, den wir abführen müssen."),
        ("beide",
         "Same building, same physics, opposite sign. Heating counted the annual demand, cooling the design load — two quantities, one envelope.",
         "Gleiches Gebäude, gleiche Physik, umgekehrtes Vorzeichen. Heizen zählte den Jahresbedarf, Kühlen die Auslegungslast — zwei Größen, eine Hülle."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Dieselbe Bilanz, zwei Jahreszeiten", title)
        din = _din_ref("DIN V 18599-2 · VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        base_y = -1.15
        unit = 0.25
        loss_x, gain_x = -2.5, 0.4
        loss_t = ValueTracker(11.5)
        gain_t = ValueTracker(6.0)

        def _column(tracker, x, colour):
            return always_redraw(lambda: Rectangle(
                width=1.35, height=max(0.05, tracker.get_value() * unit),
                color=colour, fill_color=colour, fill_opacity=0.8, stroke_width=1.2,
            ).move_to(np.array([x, base_y + max(0.05, tracker.get_value() * unit) / 2, 0])))

        loss_col = _column(loss_t, loss_x, LOSS_BLUE)
        gain_col = _column(gain_t, gain_x, SOLAR_YELLOW)
        loss_cap = body_text("Q_Senke", font_size=BODY_FONT_SIZE, color=LOSS_BLUE)
        loss_cap.move_to(np.array([loss_x, base_y - 0.32, 0]))
        gain_cap = body_text("Q_Quelle", font_size=BODY_FONT_SIZE, color=SOLAR_YELLOW)
        gain_cap.move_to(np.array([gain_x, base_y - 0.32, 0]))
        ground = Line(np.array([loss_x - 1.2, base_y, 0]), np.array([gain_x + 1.2, base_y, 0]),
                      color=P_WHITE, stroke_width=2)

        season = card_grid([("Jahreszeit", "Januar", LOSS_BLUE)], rows=1)
        season.move_to(np.array([3.9, base_y + 2.55, 0]))
        result = card_grid([("Bilanzlücke", "Heizwärmebedarf", DEMAND_RED)], rows=1)
        result.move_to(np.array([3.9, base_y + 1.35, 0]))

        self.add(loss_col, gain_col)
        self.play(Create(ground), FadeIn(loss_cap), FadeIn(gain_cap), run_time=0.9)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 0.9)

        # —— Winter ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        self.play(FadeIn(season), FadeIn(result), run_time=0.8)
        gap = always_redraw(lambda: DashedLine(
            np.array([loss_x - 0.85, base_y + max(loss_t.get_value(), gain_t.get_value()) * unit, 0]),
            np.array([gain_x + 0.85, base_y + max(loss_t.get_value(), gain_t.get_value()) * unit, 0]),
            color=P_WHITE, stroke_width=1.8, stroke_opacity=0.6,
        ))
        self.add(gap)
        win_row, win_items = equation_row([
            ("qh", "Q_h", DEMAND_RED), (None, "=", P_WHITE),
            ("senke", "Q_Senke", LOSS_BLUE), (None, "−", P_WHITE),
            ("eta", "η_h", ETA_GREEN), (None, "·", P_WHITE),
            ("quelle", "Q_Quelle", SOLAR_YELLOW),
        ])
        win_row, win_box = formula_panel(win_row)
        self.play(Create(win_box), FadeIn(win_row), run_time=0.9)
        hold_for(self, self.NARRATION, "winter", used=0.8 + 0.9 + 0.35)

        # —— Summer: the same two columns swap magnitude ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sommer"))
        summer_season = card_grid([("Jahreszeit", "Juli", P_RED)], rows=1).move_to(season)
        summer_result = card_grid([("Bilanzlücke", "Kühlbedarf", P_RED)], rows=1).move_to(result)
        sum_row, sum_items = equation_row([
            ("qk", "Q_K", P_RED), (None, "=", P_WHITE),
            ("quelle", "Q_Quelle", SOLAR_YELLOW), (None, "−", P_WHITE),
            ("eta", "η_k", ETA_GREEN), (None, "·", P_WHITE),
            ("senke", "Q_Senke", LOSS_BLUE),
        ])
        sum_row, sum_box = formula_panel(sum_row)
        self.play(
            loss_t.animate.set_value(3.4), gain_t.animate.set_value(12.2),
            ReplacementTransform(season, summer_season), ReplacementTransform(result, summer_result),
            ReplacementTransform(win_box, sum_box), ReplacementTransform(win_row, sum_row),
            run_time=2.4, rate_func=smooth,
        )
        hold_for(self, self.NARRATION, "sommer", used=2.4 + 0.35)

        # —— Both halves of one calculation ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "beide"))
        bridge = note_line("Heizen & Kühlen: eine Bilanz, zwei Vorzeichen", color=P_TEAL)
        self.play(FadeIn(bridge), run_time=0.8)
        hold_for(self, self.NARRATION, "beide", used=0.8 + 0.35)

        self.remove(loss_col, gain_col, gap)
        self.play(
            FadeOut(caption), FadeOut(sum_box), FadeOut(sum_row), FadeOut(bridge),
            FadeOut(summer_season), FadeOut(summer_result),
            FadeOut(ground), FadeOut(loss_cap), FadeOut(gain_cap),
            run_time=0.6,
        )
        self.wait(0.4)

#endregion
