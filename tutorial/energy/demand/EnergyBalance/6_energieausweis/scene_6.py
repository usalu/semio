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
    note_line, chip, cross_mark,
    ripples,
    house_section, person_glyph, lamp_glyph, thermometer_glyph, radiator,
    math_label, math_panel, math_readout, de_num, highlight_param,
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

CLASSES = (
    ("A+", 30, "#1D7A3E"), ("A", 50, "#3FA34D"), ("B", 75, "#8DC63F"),
    ("C", 100, "#C7D93A"), ("D", 130, "#F9E547"), ("E", 160, "#F5A623"),
    ("F", 200, "#E8622C"), ("G", 250, "#D0342C"), ("H", None, "#A61C1C"),
)

GAS_END, WP_END = 140, 31
Q_NUTZ = 100
UNIT = r"\mathrm{kWh/(m^{2}\cdot a)}"
STAGE_U = 0.0095
MONTHS = 36

_SEASON = np.array([0.55 + 0.45 * np.cos(2 * np.pi * (m % 12) / 12) for m in range(MONTHS)])
_RAW = np.clip(_SEASON * (1.0 + np.random.default_rng(6).uniform(-0.38, 0.38, MONTHS)), 0.06, 1.0)


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

#region Certificates
def _document(center, *, color, width=2.20, height=0.95, lines=3):
    """📄 Certificate card — header band plus ruled body lines."""
    center = np.array(center, dtype=float)
    card = RoundedRectangle(corner_radius=0.09, width=width, height=height,
                            color=color, stroke_width=2.4).move_to(center)
    card.set_fill(P_DEEP_DARK, opacity=0.9)
    header = Rectangle(width=width - 0.28, height=0.20, color=color,
                       fill_color=color, fill_opacity=0.55, stroke_width=0)
    header.move_to(card.get_top() + DOWN * 0.26)
    rules = VGroup(*[
        Line(np.array([card.get_left()[0] + 0.20, y, 0]),
             np.array([card.get_right()[0] - 0.20, y, 0]),
             color=color, stroke_width=1.1, stroke_opacity=0.45)
        for y in np.linspace(center[1] - height * 0.06, center[1] - height * 0.34, lines)
    ])
    return VGroup(card, header, rules)


def _rule_ys(center, height, count):
    """📏 Heights of the ruled lines on a ``_certificate`` sheet."""
    return np.linspace(center[1] + height * 0.22, center[1] - height * 0.34, count)


def _certificate(center, color, name, *, width, height, rules=3):
    """📃 Titled certificate sheet — ``(card with ruled lines, title above the card)``."""
    center = np.append(np.array(center, dtype=float)[:2], 0.0)
    card = _document(center, color=color, width=width, height=height, lines=0)
    lines = VGroup(*[
        Line(np.array([center[0] - width / 2 + 0.3, y, 0.0]), np.array([center[0] + width / 2 - 0.3, y, 0.0]),
             color=color, stroke_width=1.1, stroke_opacity=0.45)
        for y in _rule_ys(center, height, rules)
    ])
    label = body_text(name, font_size=LABEL_FONT_SIZE, color=color).move_to(center + UP * (height / 2 + 0.26))
    return VGroup(VGroup(card, lines), label)
#endregion


#region Efficiency band
def _class_strip(center, *, width=11.0, height=0.62, font=LABEL_FONT_SIZE - 2):
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
        text = body_text(letter, font_size=font, color=P_DEEP_DARK)
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
    """📍 Screen x of a value on the band — linear inside its class, so a pointer glides without jumping."""
    index = _class_index(value)
    lower = 0 if index == 0 else CLASSES[index - 1][1]
    upper = CLASSES[index][1]
    frac = 0.5 if upper is None else (value - lower) / (upper - lower)
    left = strip["center"][0] - strip["width"] / 2 + strip["seg_w"] * index
    return float(left + strip["seg_w"] * min(max(frac, 0.0), 1.0))


def _pointer(strip, value, color):
    """🧭 Gauge pointer riding the band — tip at the value's x, live class letter above, driven by a ``ValueTracker``."""
    top = strip["center"][1] + strip["height"] / 2

    def build():
        v = value.get_value()
        x = _class_x(strip, v)
        tip = Polygon(np.array([x, top + 0.03, 0.0]), np.array([x - 0.11, top + 0.22, 0.0]),
                      np.array([x + 0.11, top + 0.22, 0.0]), color=color, fill_color=color,
                      fill_opacity=1.0, stroke_width=0)
        disc = Circle(radius=0.18, color=color, fill_color=color, fill_opacity=1.0,
                      stroke_width=0).move_to(np.array([x, top + 0.40, 0.0]))
        letter = body_text(_class_letter(v), font_size=LABEL_FONT_SIZE - 2, color=P_DEEP_DARK)
        letter.move_to(disc.get_center())
        return VGroup(tip, disc, letter)

    return always_redraw(build)


def _class_badge(letter, colour, center, *, size=0.5, font=BODY_FONT_SIZE):
    """🏷️ Square efficiency-class badge carrying one letter."""
    box = Rectangle(width=size, height=size, color=colour, fill_color=colour, fill_opacity=0.92,
                    stroke_width=1.0).move_to(center)
    return VGroup(box, body_text(letter, font_size=font, color=P_DEEP_DARK).move_to(box))
#endregion


#region Bars and months
def _end_bar(x, ground, value, parts, *, width=0.8, unit=STAGE_U):
    """📊 Endenergie column that rises with ``value`` — one coloured segment per ``(lo, hi, colour)`` span of kWh/(m²·a)."""

    def build():
        v = value.get_value()
        group = VGroup()
        for lo, hi, colour in parts:
            h = float(np.clip(v - lo, 0.0, hi - lo)) * unit
            if h > 0.004:
                group.add(Rectangle(width=width, height=h, color=colour, fill_color=colour,
                                    fill_opacity=0.82, stroke_width=1.2).move_to(
                    np.array([x, ground + lo * unit + h / 2, 0.0])))
        return group

    return always_redraw(build)


def _month_bars(left, base_y, months, clean, *, width, height, color):
    """📅 One bar per recorded month — they rise one after another with ``months`` and settle onto the seasonal curve with ``clean``."""

    def build():
        count, mix = months.get_value(), clean.get_value()
        step = width / MONTHS
        tint = interpolate_color(ManimColor(color), ManimColor(BEDARF_CYAN), mix)
        bars = VGroup()
        for i in range(MONTHS):
            grow = float(np.clip(count - i, 0.0, 1.0))
            if grow <= 0.0:
                break
            h = max(0.012, height * grow * ((1.0 - mix) * _RAW[i] + mix * _SEASON[i]))
            bars.add(Rectangle(width=step * 0.72, height=h, stroke_width=0, fill_color=tint,
                               fill_opacity=0.88).move_to(np.array([left + step * (i + 0.5), base_y + h / 2, 0.0])))
        return bars

    return always_redraw(build)
#endregion


#region Scenery
def _stand(hs, x, color=P_ORANGE, storey=0):
    """🧍 Occupant at true scale — 1.75 m in a 2.5 m storey, feet on the floor line at ``x``."""
    storey_h = hs["w_height"] / 2
    height = 0.7 * storey_h
    floor = hs["bottom_left"][1] + storey * storey_h
    return person_glyph(np.array([x, floor + 0.02 + height / 2, 0.0]), color=color, scale=height / 0.66)


def _envelope(hs, color=P_TEAL):
    """🧱 Heavy outline around the thermal envelope of a ``house_section``."""
    off, eave = 0.09, 0.26 * hs["w_width"] / 3.6
    bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
    return VMobject(color=color, stroke_width=6, stroke_opacity=0.85).set_points_as_corners([
        bl + LEFT * off, tl + LEFT * off, tl + LEFT * (eave + off) + UP * 0.02, peak + UP * off,
        tr + RIGHT * (eave + off) + UP * 0.02, tr + RIGHT * off, br + RIGHT * off,
    ])


def _heat(centers, rt, r_max=0.28):
    """♨️ Heat ripples rising from ``centers`` for ``rt`` seconds."""
    return [ripples(list(centers), r_max=r_max, cycles=rt / 1.4, color=P_RED)]
#endregion

#endregion


#region Beat 1 – Bedarf oder Verbrauch

class Beat1_BedarfVsVerbrauch(Scene):
    NARRATION = [
        ("intro",
         "Everything we calculated ends up on one sheet of paper that buyers and tenants actually get to see.",
         "Alles, was wir berechnet haben, landet auf einem Blatt Papier, das Käufer und Mieter wirklich zu sehen bekommen."),
        ("bedarf_berechnung",
         "The demand certificate is exactly this series: a standardised calculation.",
         "Der Bedarfsausweis ist genau diese Serie: eine standardisierte Berechnung."),
        ("bedarf_rand",
         "It rests on fixed boundary conditions.",
         "Sie beruht auf festgelegten Randbedingungen."),
        ("bedarf_norm",
         "A set room temperature and a standard use, whatever the real residents do.",
         "Eine feste Raumtemperatur und eine Norm-Nutzung — unabhängig vom Verhalten der echten Bewohner."),
        ("verbrauch_daten",
         "The consumption certificate instead uses recorded consumption data.",
         "Der Verbrauchsausweis nutzt stattdessen erfasste Verbrauchsdaten."),
        ("verbrauch_monate",
         "At least thirty-six months of it are required.",
         "Es werden mindestens sechsunddreißig Monate benötigt."),
        ("verbrauch_witterung",
         "These values are corrected for the weather.",
         "Diese Werte werden witterungsbereinigt."),
        ("unterschied_bedarf",
         "So the demand certificate describes the building under those fixed conditions.",
         "Der Bedarfsausweis beschreibt also das Gebäude unter festen Randbedingungen."),
        ("unterschied_verbrauch",
         "The consumption certificate describes the building and its actual use together.",
         "Der Verbrauchsausweis beschreibt Gebäude und tatsächliche Nutzung zusammen."),
        ("wann_neubau",
         "New buildings always need the calculated certificate.",
         "Für Neubauten ist der berechnete Ausweis Pflicht."),
        ("wann_bestand",
         "For existing buildings there is usually a choice, with exceptions.",
         "Im Bestand besteht meist die Wahl — mit Ausnahmen."),
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

        def clear():
            return [FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din, caption)]

        sheet = _certificate([0.0, -0.05], P_WHITE, "Energieausweis", width=3.2, height=3.4)
        self.play(Create(sheet[0]), FadeIn(sheet[1]), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.6)

        sx = -3.6
        left_c = np.array([sx, -0.05, 0.0])
        bedarf_sheet = _certificate(left_c, BEDARF_CYAN, "Bedarfsausweis", width=3.2, height=3.4)
        row_x = sx - 1.3
        input_rows = VGroup(*[
            body_text(name, font_size=LABEL_FONT_SIZE - 1, color=BEDARF_CYAN).move_to(
                np.array([row_x, y + 0.22, 0.0])).align_to(np.array([row_x, y, 0.0]), LEFT)
            for name, y in zip(("Geometrie", "U-Werte", "Anlage"), _rule_ys(left_c, 3.4, 3))
        ])
        computed = body_text("berechnet", font_size=LABEL_FONT_SIZE - 1, color=BEDARF_CYAN).move_to(
            np.array([sx, -1.58, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf_berechnung"))
        self.play(ReplacementTransform(sheet, bedarf_sheet), run_time=1.4)
        for row in input_rows:
            self.play(FadeIn(row, shift=RIGHT * 0.15), run_time=1.2)
        self.play(FadeIn(computed), run_time=1.0)
        hold_for(self, self.NARRATION, "bedarf_berechnung")

        hx, hy = 3.6, -0.45
        house = house_section(np.array([hx, hy, 0.0]), scale=0.8)
        floor_y, level_y = house["bottom_left"][1], house["level_1"].get_center()[1]
        norm_label = body_text("Norm-Randbedingungen", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([hx, 1.78, 0.0]))
        norm_person = _stand(house, hx - 0.55, P_CYAN)
        norm_lamp = lamp_glyph(np.array([hx - 0.05, level_y, 0.0]), drop=0.1)
        norm_therm = thermometer_glyph(np.array([hx + 0.45, floor_y + 0.2, 0.0]), height=0.55,
                                       color=P_CYAN, level=0.5)
        norm_temp = math_label(r"20\,\mathrm{°C}", at=np.array([hx + 0.7, floor_y + 0.42, 0.0]),
                               size=LABEL_FONT_SIZE - 2, color=P_CYAN, edge="left")
        norm_radiator = radiator(np.array([hx - 1.08, floor_y + 0.17, 0.0]), color=P_RED).scale(0.45)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf_rand"))
        self.play(Create(house["group"]), run_time=1.5)
        self.play(FadeIn(norm_label), run_time=1.2)
        hold_for(self, self.NARRATION, "bedarf_rand")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf_norm"))
        self.play(FadeIn(norm_therm["group"]), FadeIn(norm_therm["column"]), FadeIn(norm_temp),
                  FadeIn(norm_radiator), *_heat([norm_radiator.get_top() + UP * 0.02], 1.4), run_time=1.4)
        self.play(FadeIn(norm_person), FadeIn(norm_lamp["group"]),
                  *_heat([norm_person.get_top() + UP * 0.05, norm_radiator.get_top() + UP * 0.02], 1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "bedarf_norm",
                 during=lambda rt: _heat([norm_person.get_top() + UP * 0.05, norm_radiator.get_top() + UP * 0.02], rt))

        orange_sheet = _certificate([0.0, -0.05], VERBRAUCH_ORANGE, "Verbrauchsausweis", width=8.0, height=3.4,
                                    rules=0)
        base_y = -1.35
        months, clean = ValueTracker(0.0), ValueTracker(0.0)
        bars = _month_bars(-3.4, base_y, months, clean, width=6.8, height=2.0, color=VERBRAUCH_ORANGE)
        bars_floor = Line(np.array([-3.4, base_y, 0.0]), np.array([3.4, base_y, 0.0]), color=P_WHITE, stroke_width=1.4)
        month_read = math_readout(lambda: rf"{de_num(months.get_value())}\,\text{{Monate}}",
                                  np.array([-3.4, 0.98, 0.0]), size=LABEL_FONT_SIZE, color=VERBRAUCH_ORANGE,
                                  edge="left")
        corrected = body_text("witterungsbereinigt", font_size=LABEL_FONT_SIZE, color=BEDARF_CYAN).move_to(
            np.array([2.4, 0.98, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verbrauch_daten"))
        self.play(*clear(), run_time=0.7)
        self.play(Create(orange_sheet[0]), FadeIn(orange_sheet[1]), Create(bars_floor), run_time=1.4)
        self.add(month_read, bars)
        self.play(months.animate.set_value(6), run_time=2.0, rate_func=linear)
        hold_for(self, self.NARRATION, "verbrauch_daten")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verbrauch_monate"))
        self.play(months.animate.set_value(MONTHS), run_time=4.0, rate_func=linear)
        hold_for(self, self.NARRATION, "verbrauch_monate")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verbrauch_witterung"))
        self.play(clean.animate.set_value(1.0), FadeIn(corrected), run_time=2.0)
        hold_for(self, self.NARRATION, "verbrauch_witterung")

        hx2 = 0.0
        house2 = house_section(np.array([hx2, -0.3, 0.0]), scale=0.8)
        envelope = _envelope(house2)
        walker = _stand(house2, hx2 - 0.65, P_ORANGE)
        sleeper = _stand(house2, hx2 + 0.55, P_ORANGE, storey=1)
        real_lamp = lamp_glyph(np.array([hx2 + 0.5, house2["level_1"].get_center()[1], 0.0]), drop=0.1)
        row_a, box_a, items_a = math_panel([
            ("bed", r"\text{Bedarf}", BEDARF_CYAN), (None, "=", P_WHITE),
            ("geb", r"\text{Gebäude}\;|_{\,\text{Norm}}", P_TEAL),
        ], size=BODY_FONT_SIZE + 2)
        row_b, box_b, items_b = math_panel([
            ("ver", r"\text{Verbrauch}", VERBRAUCH_ORANGE), (None, "=", P_WHITE),
            ("geb2", r"\text{Gebäude}", P_TEAL), (None, "+", P_WHITE),
            ("use", r"\text{Nutzung}", VERBRAUCH_ORANGE),
        ], size=BODY_FONT_SIZE + 2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unterschied_bedarf"))
        self.play(*clear(), run_time=0.7)
        self.play(Create(house2["group"]), run_time=1.4)
        self.play(Create(envelope), Create(box_a), FadeIn(row_a), *_heat([house2["center"] + UP * 0.1], 1.4),
                  run_time=1.4)
        hold_for(self, self.NARRATION, "unterschied_bedarf", during=lambda rt: _heat([house2["center"] + UP * 0.1], rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unterschied_verbrauch"))
        self.play(FadeOut(row_a), FadeOut(box_a), Create(box_b), FadeIn(row_b), run_time=1.2)
        self.play(FadeIn(walker), FadeIn(sleeper), *_heat([walker.get_top(), sleeper.get_top()], 1.2), run_time=1.2)
        use_ring = highlight_param(items_b, "use", color=VERBRAUCH_ORANGE)
        self.play(FadeIn(real_lamp["group"]), Create(use_ring), Indicate(walker, color=VERBRAUCH_ORANGE),
                  Indicate(sleeper, color=VERBRAUCH_ORANGE),
                  *_heat([walker.get_top(), sleeper.get_top()], 1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "unterschied_verbrauch",
                 during=lambda rt: _heat([walker.get_top(), sleeper.get_top()], rt))

        cyan_sheet = _certificate([0.0, 0.2], BEDARF_CYAN, "Bedarfsausweis", width=2.6, height=2.8)
        cyan_left = _certificate([-2.6, 0.2], BEDARF_CYAN, "Bedarfsausweis", width=2.6, height=2.8)
        orange_right = _certificate([2.6, 0.2], VERBRAUCH_ORANGE, "Verbrauchsausweis", width=2.6, height=2.8)
        oder = body_text("oder", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(np.array([0.0, 0.2, 0.0]))
        row_c, box_c, items_c = math_panel([
            ("neu", r"\text{Neubau}", P_TEAL), (None, r"\Rightarrow", P_WHITE),
            ("bed", r"\text{Bedarfsausweis}", BEDARF_CYAN),
        ], size=BODY_FONT_SIZE + 2)
        row_d, box_d, items_d = math_panel([
            ("alt", r"\text{Bestand}", P_TEAL), (None, r"\Rightarrow", P_WHITE),
            ("bed2", r"\text{Bedarf}", BEDARF_CYAN), (None, r"\text{oder}", P_WHITE),
            ("ver", r"\text{Verbrauch}", VERBRAUCH_ORANGE),
        ], size=BODY_FONT_SIZE + 2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wann_neubau"))
        self.play(*clear(), run_time=0.7)
        self.play(Create(cyan_sheet[0]), FadeIn(cyan_sheet[1]), run_time=1.4)
        self.play(Create(box_c), FadeIn(row_c), Indicate(cyan_sheet, color=BEDARF_CYAN, scale_factor=1.04),
                  run_time=1.4)
        hold_for(self, self.NARRATION, "wann_neubau")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wann_bestand"))
        self.play(ReplacementTransform(cyan_sheet, cyan_left), FadeOut(row_c), FadeOut(box_c), run_time=1.4)
        self.play(Create(orange_right[0]), FadeIn(orange_right[1]), FadeIn(oder), Create(box_d), FadeIn(row_d),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "wann_bestand")

        self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 2 – Die Skala lesen

class Beat2_DieSkala(Scene):
    NARRATION = [
        ("skala",
         "For residential buildings, the result is printed on one colour band, from A plus down to H.",
         "Bei Wohngebäuden wird das Ergebnis auf einem Farbband ausgewiesen — von A-plus bis H."),
        ("skala_beide",
         "Both certificate types print on that same band.",
         "Beide Ausweistypen nutzen dasselbe Farbband."),
        ("werte",
         "Two numbers stand beside it: final energy and primary energy.",
         "Daneben stehen zwei Zahlen: Endenergie und Primärenergie."),
        ("werte_klasse",
         "But for residential buildings the class follows final energy.",
         "Die Effizienzklasse richtet sich bei Wohngebäuden aber nach der Endenergie."),
        ("gas",
         "Our gas-heated design needs a hundred and forty kilowatt-hours per square metre.",
         "Unser gasbeheizter Entwurf braucht hundertvierzig Kilowattstunden je Quadratmeter."),
        ("gas_nutz",
         "Of that, a hundred is useful heat; the other forty are plant losses.",
         "Davon sind hundert Nutzwärme — die übrigen vierzig sind Anlagenverluste."),
        ("gas_klasse",
         "That lands in class E.",
         "Das ergibt Klasse E."),
        ("wp",
         "The identical building with a heat pump needs only thirty-one.",
         "Dasselbe Gebäude mit Wärmepumpe braucht nur einunddreißig."),
        ("wp_klasse",
         "That moves it up to class A.",
         "Damit rückt es auf Klasse A."),
        ("wp_umwelt",
         "The rest of the useful heat comes from ambient heat.",
         "Den Rest der Nutzwärme liefert die Umweltwärme."),
        ("lesen",
         "Same walls, same windows, same useful heating demand.",
         "Gleiche Wände, gleiche Fenster, gleicher Heizwärmebedarf."),
        ("lesen_anlage",
         "The plant moves the letter too, not the envelope alone.",
         "Den Buchstaben bestimmt die Anlage mit, nicht die Hülle allein."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Skala lesen", title)
        din = _din_ref("GEG § 86 · Anlage 10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "skala"))
        self.play(FadeIn(caption), run_time=0.3)

        band_y = -1.1
        strip = _class_strip([0.0, band_y, 0.0], width=11.0, height=0.42)
        band_top, band_bottom = band_y + 0.21, band_y - 0.21
        bounds = VGroup(*[
            math_label(str(bound), at=np.array([strip["center"][0] - strip["width"] / 2 + strip["seg_w"] * (i + 1),
                                                band_bottom - 0.3, 0.0]),
                       size=LABEL_FONT_SIZE - 4, color=P_WHITE, edge="center").set_opacity(0.6)
            for i, (_l, bound, _c) in enumerate(CLASSES[:-1])
        ])
        unit_label = math_label(UNIT, at=np.array([strip["center"][0] + strip["width"] / 2 + 0.2, band_y - 0.07, 0.0]),
                                size=LABEL_FONT_SIZE - 4, color=P_TEAL, edge="left")
        schematic_note = note_line("Klassenbreiten schematisch, nicht maßstäblich", color=P_TEAL)

        def clear(*keep):
            return [FadeOut(m) for m in list(self.mobjects)
                    if m not in (title, subtitle, din, caption, *strip["bands"], *keep)]

        sheet_l = _certificate([-2.6, 0.75], BEDARF_CYAN, "Bedarf", width=2.2, height=1.3, rules=2)
        sheet_r = _certificate([2.6, 0.75], VERBRAUCH_ORANGE, "Verbrauch", width=2.2, height=1.3, rules=2)
        feed_l = Arrow(np.array([-2.6, 0.06, 0.0]), np.array([-0.3, band_top + 0.04, 0.0]), buff=0.05,
                       color=BEDARF_CYAN, stroke_width=3, max_tip_length_to_length_ratio=0.12)
        feed_r = Arrow(np.array([2.6, 0.06, 0.0]), np.array([0.3, band_top + 0.04, 0.0]), buff=0.05,
                       color=VERBRAUCH_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.12)

        self.play(
            LaggedStart(*[FadeIn(b, shift=UP * 0.1) for b in strip["bands"]], lag_ratio=0.12), run_time=2.2)
        self.play(FadeIn(bounds), FadeIn(unit_label), FadeIn(schematic_note), run_time=1.2)
        hold_for(self, self.NARRATION, "skala", used=BEAT_SUBTITLE_FADE + 0.3 + 2.2 + 1.2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "skala_beide"))
        self.play(Create(sheet_l[0]), FadeIn(sheet_l[1]), Create(sheet_r[0]), FadeIn(sheet_r[1]), run_time=1.4)
        self.play(GrowArrow(feed_l), GrowArrow(feed_r), run_time=1.4)
        hold_for(self, self.NARRATION, "skala_beide")

        chip_end = chip("Endenergie", END_RED).move_to(np.array([-2.0, 0.7, 0.0]))
        chip_prim = chip("Primärenergie", PRIM_VIOLET).move_to(np.array([2.0, 0.7, 0.0]))
        arrow_end = Arrow(chip_end.get_bottom(), np.array([-2.0, band_top + 0.04, 0.0]), buff=0.06,
                          color=END_RED, stroke_width=3, max_tip_length_to_length_ratio=0.12)
        arrow_prim = Arrow(chip_prim.get_bottom(), np.array([2.0, band_top + 0.04, 0.0]), buff=0.06,
                           color=PRIM_VIOLET, stroke_width=3, max_tip_length_to_length_ratio=0.12)
        arrow_prim.set_stroke(opacity=0.55)
        blocked = cross_mark(PRIM_VIOLET, 0.15).move_to(np.array([2.0, -0.2, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "werte"))
        self.play(*clear(bounds, unit_label), run_time=0.7)
        self.play(FadeIn(chip_end), run_time=1.4)
        self.play(FadeIn(chip_prim), run_time=1.4)
        hold_for(self, self.NARRATION, "werte")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "werte_klasse"))
        self.play(GrowArrow(arrow_end), Indicate(chip_end, color=END_RED), run_time=1.4)
        self.play(GrowArrow(arrow_prim), FadeIn(blocked), run_time=1.4)
        hold_for(self, self.NARRATION, "werte_klasse")

        ground = 0.3
        gx, wx = 1.02, -4.23
        v_gas = ValueTracker(0.0)
        gas_bar = _end_bar(gx, ground, v_gas, [(0, Q_NUTZ, BEDARF_CYAN), (Q_NUTZ, GAS_END, END_RED)])
        gas_ptr = _pointer(strip, v_gas, P_ORANGE)
        gas_read = math_readout(
            lambda: rf"{de_num(v_gas.get_value())}\,{UNIT}",
            np.array([gx, ground + GAS_END * STAGE_U + 0.3, 0.0]),
            size=LABEL_FONT_SIZE - 1, color=P_ORANGE, edge="center",
        )
        gas_name = body_text("Gaskessel", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(
            np.array([gx, ground - 0.25, 0.0]))
        qh_y = ground + Q_NUTZ * STAGE_U
        nutz_label = math_label(r"\text{Nutzwärme}\;Q_{h}=100", at=np.array([gx + 0.55, ground + 0.5 * Q_NUTZ * STAGE_U, 0.0]),
                                size=LABEL_FONT_SIZE - 2, color=BEDARF_CYAN, edge="left")
        loss_label = body_text("Anlagenverluste", font_size=LABEL_FONT_SIZE - 3, color=END_RED)
        loss_label.move_to(np.array([gx + 0.55, ground + 1.2 * Q_NUTZ * STAGE_U, 0.0])).align_to(
            np.array([gx + 0.55, 0.0, 0.0]), LEFT)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gas"))
        self.play(*clear(), run_time=0.7)
        self.play(FadeIn(gas_name), run_time=1.0)
        self.add(gas_bar, gas_read, gas_ptr)
        self.play(v_gas.animate.set_value(GAS_END), run_time=3.0, rate_func=smooth)
        hold_for(self, self.NARRATION, "gas")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gas_nutz"))
        self.play(FadeIn(nutz_label, shift=LEFT * 0.1), run_time=1.4)
        self.play(FadeIn(loss_label, shift=LEFT * 0.1), run_time=1.4)
        hold_for(self, self.NARRATION, "gas_nutz")

        gas_ring = SurroundingRectangle(strip["bands"][_class_index(GAS_END)], color=P_ORANGE, buff=0.0,
                                        stroke_width=4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gas_klasse"))
        self.play(Create(gas_ring), Indicate(gas_ptr, color=P_ORANGE), run_time=1.4)
        hold_for(self, self.NARRATION, "gas_klasse")

        v_wp = ValueTracker(0.0)
        wp_bar = _end_bar(wx, ground, v_wp, [(0, WP_END, END_RED)])
        wp_ptr = _pointer(strip, v_wp, AMBIENT_GREEN)
        wp_read = math_readout(
            lambda: rf"{de_num(v_wp.get_value())}\,{UNIT}",
            np.array([wx, ground + Q_NUTZ * STAGE_U + 0.3, 0.0]),
            size=LABEL_FONT_SIZE - 1, color=AMBIENT_GREEN, edge="center",
        )
        wp_name = body_text("Wärmepumpe", font_size=LABEL_FONT_SIZE, color=AMBIENT_GREEN).move_to(
            np.array([wx, ground - 0.25, 0.0]))
        ambient = Rectangle(width=0.8, height=(Q_NUTZ - WP_END) * STAGE_U, color=AMBIENT_GREEN, stroke_width=1.2,
                            fill_color=AMBIENT_GREEN, fill_opacity=0.16).move_to(
            np.array([wx, ground + (WP_END + Q_NUTZ) / 2 * STAGE_U, 0.0]))
        ambient_label = body_text("Umweltwärme", font_size=LABEL_FONT_SIZE - 3, color=AMBIENT_GREEN)
        ambient_label.move_to(np.array([wx - 0.55, ground + 0.65 * Q_NUTZ * STAGE_U, 0.0])).align_to(
            np.array([wx - 0.55, 0.0, 0.0]), RIGHT)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wp"))
        gas_bar.clear_updaters()
        self.play(*clear(gas_bar, gas_name), gas_bar.animate.set_opacity(0.3), gas_name.animate.set_opacity(0.4),
                  FadeIn(wp_name), run_time=1.2)
        self.add(wp_bar, wp_read, wp_ptr)
        self.play(v_wp.animate.set_value(WP_END), run_time=2.4, rate_func=smooth)
        hold_for(self, self.NARRATION, "wp")

        wp_ring = SurroundingRectangle(strip["bands"][_class_index(WP_END)], color=AMBIENT_GREEN, buff=0.0,
                                       stroke_width=4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wp_klasse"))
        self.play(Create(wp_ring), Indicate(wp_ptr, color=AMBIENT_GREEN), run_time=1.4)
        hold_for(self, self.NARRATION, "wp_klasse")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wp_umwelt"))
        self.play(FadeIn(ambient), run_time=1.4)
        self.play(FadeIn(ambient_label), run_time=1.2)
        hold_for(self, self.NARRATION, "wp_umwelt")

        house = house_section(np.array([-1.6, ground + 0.624, 0.0]), scale=0.52)
        warm = [house["center"] + UP * 0.1]
        qh_left = DashedLine(np.array([wx + 0.4, qh_y, 0.0]), np.array([house["bottom_left"][0] - 0.01, qh_y, 0.0]),
                             color=BEDARF_CYAN, stroke_width=2.2, dash_length=0.1)
        qh_right = DashedLine(np.array([house["bottom_right"][0] + 0.01, qh_y, 0.0]),
                              np.array([gx - 0.4, qh_y, 0.0]), color=BEDARF_CYAN, stroke_width=2.2, dash_length=0.1)
        qh_label = math_label(r"Q_{h}=100", at=np.array([wx + 0.75, qh_y + 0.3, 0.0]),
                              size=LABEL_FONT_SIZE - 2, color=BEDARF_CYAN, edge="center")
        envelope = _envelope(house)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lesen"))
        wp_bar.clear_updaters()
        self.play(*clear(gas_bar, gas_name, wp_bar, wp_name, ambient), gas_bar.animate.set_opacity(0.82),
                  gas_name.animate.set_opacity(1.0), run_time=0.8)
        self.play(Create(house["group"]), run_time=1.6)
        self.play(Create(envelope), Create(qh_left), Create(qh_right), FadeIn(qh_label),
                  *_heat(warm, 1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "lesen", during=lambda rt: _heat(warm, rt))

        ep_gas, ep_wp = de_num(GAS_END / Q_NUTZ, 2), de_num(WP_END / Q_NUTZ, 2)
        row, box, items = math_panel([
            ("qh", r"Q_{h}=100", BEDARF_CYAN), (None, r"\Rightarrow", P_WHITE),
            ("gas", rf"100\cdot{ep_gas}={GAS_END}\to\mathrm{{{_class_letter(GAS_END)}}}", P_ORANGE),
            (None, r"\quad|\quad", P_WHITE),
            ("wp", rf"100\cdot{ep_wp}={WP_END}\to\mathrm{{{_class_letter(WP_END)}}}", AMBIENT_GREEN),
        ], size=BODY_FONT_SIZE + 2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lesen_anlage"))
        self.play(Create(box), FadeIn(row), *_heat(warm, 1.2), run_time=1.2)
        self.play(Create(highlight_param(items, "qh", color=BEDARF_CYAN)), Indicate(qh_left, color=BEDARF_CYAN),
                  Indicate(qh_right, color=BEDARF_CYAN), *_heat(warm, 1.2), run_time=1.2)
        self.play(Create(highlight_param(items, "gas", color=P_ORANGE)), Indicate(gas_bar, color=P_ORANGE),
                  *_heat(warm, 1.2), run_time=1.2)
        self.play(Create(highlight_param(items, "wp", color=AMBIENT_GREEN)), Indicate(wp_bar, color=AMBIENT_GREEN),
                  *_heat(warm, 1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "lesen_anlage", during=lambda rt: _heat(warm, rt))

        self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 3 – Was der Ausweis auslöst

class Beat3_PflichtangabenUndFolgen(Scene):
    NARRATION = [
        ("intro",
         "The certificate is not a formality.",
         "Der Ausweis ist keine Formalie."),
        ("anzeige",
         "Once one exists, its key entries have to appear in commercial property ads.",
         "Liegt einer vor, gehören seine Kernangaben in kommerzielle Immobilienanzeigen."),
        ("art_wert",
         "These are the type of certificate and the final energy value.",
         "Das sind die Ausweisart und der Endenergiekennwert."),
        ("traeger",
         "Then the main energy carrier.",
         "Dazu kommt der wesentliche Energieträger."),
        ("baujahr",
         "And for residential buildings the year of construction.",
         "Und bei Wohngebäuden das Baujahr."),
        ("klasse",
         "For residential buildings the efficiency class is required as well.",
         "Bei Wohngebäuden zusätzlich die Effizienzklasse."),
        ("buchstabe",
         "That single letter travels furthest.",
         "Dieser eine Buchstabe wirkt am weitesten."),
        ("vergleich",
         "It lets buyers and tenants roughly compare buildings.",
         "Er lässt Käufer und Mieter Gebäude überschlägig vergleichen."),
        ("grenzen",
         "For exact costs or renovation effort it is not enough on its own.",
         "Für genaue Kosten oder Sanierungsaufwand reicht er allein nicht."),
        ("empfehlung",
         "For existing buildings it also contains modernisation recommendations.",
         "Bei Bestandsgebäuden enthält er zudem Modernisierungsempfehlungen."),
        ("keine",
         "Or the statement that none are reasonably possible.",
         "Oder den Hinweis, dass keine sinnvoll möglich sind."),
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

        def clear(*keep):
            return [FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din, caption, *keep)]

        class_color = CLASSES[_class_index(GAS_END)][2]
        class_letter = CLASSES[_class_index(GAS_END)][0]

        big_sheet = _certificate([0.0, 0.0], BEDARF_CYAN, "Energieausweis", width=3.2, height=3.4)
        self.play(Create(big_sheet[0]), FadeIn(big_sheet[1]), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.6)

        card_x, text_x = -4.3, -6.2
        listing = RoundedRectangle(corner_radius=0.10, width=4.6, height=3.9, color=P_TEAL, stroke_width=2.4)
        listing.move_to(np.array([card_x, 0.0, 0.0]))
        listing.set_fill(P_DEEP_DARK, opacity=0.9)
        listing_head = body_text("Pflichtangaben nach GEG § 87", font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
        listing_head.next_to(listing.get_top(), DOWN, buff=0.22)
        small_sheet = _certificate([2.0, 0.1], BEDARF_CYAN, "Energieausweis", width=2.4, height=2.7)
        into_ad = Arrow(np.array([0.7, 0.1, 0.0]), np.array([listing.get_right()[0] + 0.12, 0.1, 0.0]), buff=0.0,
                        color=BEDARF_CYAN, stroke_width=3, max_tip_length_to_length_ratio=0.12)

        keys = ("Ausweisart", "Endenergiekennwert", "wesentlicher Energieträger", "Baujahr (Wohngebäude)")
        row_y = (1.05, 0.45, -0.15, -0.75)
        key_texts = VGroup(*[
            body_text(k, font_size=LABEL_FONT_SIZE - 4, color=P_WHITE).set_opacity(0.75).move_to(
                np.array([text_x, y, 0.0])).align_to(np.array([text_x, y, 0.0]), LEFT)
            for k, y in zip(keys, row_y)
        ])
        value_at = [np.array([text_x, y - 0.29, 0.0]) for y in row_y]
        v_type = body_text("Bedarfsausweis", font_size=LABEL_FONT_SIZE, color=BEDARF_CYAN)
        v_type.move_to(value_at[0]).align_to(value_at[0], LEFT)
        v_end = ValueTracker(0.0)
        v_end_read = math_readout(lambda: rf"{de_num(v_end.get_value())}\,{UNIT}", value_at[1] + DOWN * 0.1,
                                  size=LABEL_FONT_SIZE, color=END_RED, edge="left")
        v_carrier = body_text("Erdgas", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        v_carrier.move_to(value_at[2]).align_to(value_at[2], LEFT)
        v_year = body_text("1996", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        v_year.move_to(value_at[3]).align_to(value_at[3], LEFT)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "anzeige"))
        self.play(ReplacementTransform(big_sheet, small_sheet), run_time=1.4)
        self.play(Create(listing), FadeIn(listing_head), GrowArrow(into_ad), run_time=1.4)
        hold_for(self, self.NARRATION, "anzeige")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "art_wert"))
        self.play(FadeIn(key_texts[0]), FadeIn(v_type, shift=RIGHT * 0.1), run_time=1.2)
        self.add(v_end_read)
        self.play(FadeIn(key_texts[1]), v_end.animate.set_value(GAS_END), run_time=2.0, rate_func=smooth)
        hold_for(self, self.NARRATION, "art_wert")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "traeger"))
        self.play(FadeIn(key_texts[2]), FadeIn(v_carrier, shift=RIGHT * 0.1), run_time=1.4)
        hold_for(self, self.NARRATION, "traeger")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "baujahr"))
        self.play(FadeIn(key_texts[3]), FadeIn(v_year, shift=RIGHT * 0.1), run_time=1.4)
        hold_for(self, self.NARRATION, "baujahr")

        strip = _class_strip([2.2, 0.9, 0.0], width=6.4, height=0.5, font=LABEL_FONT_SIZE - 4)
        v_scale = ValueTracker(0.0)
        scale_ptr = _pointer(strip, v_scale, P_ORANGE)
        tag_y = -1.55
        badge_tag = body_text("Effizienzklasse", font_size=LABEL_FONT_SIZE - 4, color=P_WHITE).set_opacity(0.75)
        badge_tag.move_to(np.array([text_x, tag_y, 0.0])).align_to(np.array([text_x, tag_y, 0.0]), LEFT)
        badge_spot = np.array([text_x + 3.0, tag_y, 0.0])
        fly = math_label(rf"{de_num(GAS_END)}\,{UNIT}", at=value_at[1] + DOWN * 0.1,
                         size=LABEL_FONT_SIZE, color=END_RED, edge="left")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "klasse"))
        self.play(*clear(listing, listing_head, *key_texts, v_type, v_end_read, v_carrier, v_year), run_time=0.8)
        self.play(LaggedStart(*[FadeIn(b, shift=UP * 0.08) for b in strip["bands"]], lag_ratio=0.1), run_time=1.6)
        self.play(FadeIn(badge_tag), run_time=1.2)
        hold_for(self, self.NARRATION, "klasse")

        cell = strip["bands"][_class_index(GAS_END)]
        badge = _class_badge(class_letter, class_color, badge_spot, size=0.55, font=BODY_FONT_SIZE + 2)
        flight = cell.copy()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "buchstabe"))
        self.add(fly, scale_ptr)
        self.play(fly.animate.move_to(np.array([2.2, 0.1, 0.0])), v_scale.animate.set_value(GAS_END),
                  run_time=2.2, rate_func=smooth)
        self.play(FadeOut(fly), Indicate(cell, color=P_WHITE, scale_factor=1.15), run_time=0.8)
        self.add(flight)
        self.play(ReplacementTransform(flight, badge), run_time=1.6)
        hold_for(self, self.NARRATION, "buchstabe")

        h_wp = house_section(np.array([-3.2, -0.24, 0.0]), scale=0.55)
        h_gas = house_section(np.array([3.2, -0.24, 0.0]), scale=0.55)
        ground = float(h_wp["bottom_left"][1])
        buyer = person_glyph(np.array([-0.8, ground + 0.52, 0.0]), color=P_ORANGE, scale=1.5)
        tenant = person_glyph(np.array([0.8, ground + 0.52, 0.0]), color=P_YELLOW, scale=1.5)
        who = VGroup(
            body_text("Käufer", font_size=LABEL_FONT_SIZE - 3, color=P_ORANGE).next_to(buyer, UP, buff=0.12),
            body_text("Mieter", font_size=LABEL_FONT_SIZE - 3, color=P_YELLOW).next_to(tenant, UP, buff=0.12),
        )
        wp_color, wp_letter = CLASSES[_class_index(WP_END)][2], CLASSES[_class_index(WP_END)][0]
        wp_badge = _class_badge(wp_letter, wp_color, np.array([-3.2, -1.5, 0.0]), size=0.55, font=BODY_FONT_SIZE)
        gas_badge = _class_badge(class_letter, class_color, np.array([3.2, -1.5, 0.0]), size=0.55, font=BODY_FONT_SIZE)
        compare = DoubleArrow(np.array([-2.5, -1.5, 0.0]), np.array([2.5, -1.5, 0.0]), buff=0.05, color=P_WHITE,
                              stroke_width=2.5, max_tip_length_to_length_ratio=0.1)
        warm_both = [h_wp["center"] + UP * 0.1, h_gas["center"] + UP * 0.1]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "vergleich"))
        self.play(*clear(), run_time=0.7)
        self.play(Create(h_wp["group"]), Create(h_gas["group"]), run_time=1.6)
        self.play(FadeIn(wp_badge), FadeIn(gas_badge), *_heat(warm_both, 1.2), run_time=1.2)
        self.play(FadeIn(buyer), FadeIn(tenant), FadeIn(who), *_heat(warm_both, 1.2), run_time=1.2)
        self.play(GrowArrow(compare), *_heat(warm_both, 1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "vergleich", during=lambda rt: _heat(warm_both, rt))

        doc = _certificate([-3.4, 0.0], BEDARF_CYAN, "Energieausweis", width=2.0, height=2.4)
        costs = chip("genaue Kosten", P_TEAL, font_size=LABEL_FONT_SIZE).move_to(np.array([2.6, 0.8, 0.0]))
        renovation = chip("Sanierungsaufwand", P_TEAL, font_size=LABEL_FONT_SIZE).move_to(np.array([2.6, -0.8, 0.0]))
        to_costs = Arrow(np.array([-2.3, 0.1, 0.0]), costs.get_left() + LEFT * 0.1, buff=0.05, color=P_WHITE,
                         stroke_width=2.5, max_tip_length_to_length_ratio=0.1)
        to_renovation = Arrow(np.array([-2.3, -0.1, 0.0]), renovation.get_left() + LEFT * 0.1, buff=0.05,
                              color=P_WHITE, stroke_width=2.5, max_tip_length_to_length_ratio=0.1)
        no_costs = cross_mark(P_RED, 0.15).move_to(to_costs.point_from_proportion(0.55))
        no_renovation = cross_mark(P_RED, 0.15).move_to(to_renovation.point_from_proportion(0.55))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "grenzen"))
        self.play(*clear(), run_time=0.7)
        self.play(Create(doc[0]), FadeIn(doc[1]), run_time=1.4)
        self.play(FadeIn(costs), GrowArrow(to_costs), FadeIn(no_costs), run_time=1.4)
        self.play(FadeIn(renovation), GrowArrow(to_renovation), FadeIn(no_renovation), run_time=1.4)
        hold_for(self, self.NARRATION, "grenzen")

        h_old = house_section(np.array([-3.8, -0.24, 0.0]), scale=0.55)
        h_new = house_section(np.array([3.8, -0.24, 0.0]), scale=0.55)
        old_badge = _class_badge(class_letter, class_color, np.array([-3.8, -1.5, 0.0]), size=0.55, font=BODY_FONT_SIZE)
        new_badge = _class_badge(wp_letter, wp_color, np.array([3.8, -1.5, 0.0]), size=0.55, font=BODY_FONT_SIZE)
        recommend = chip("Modernisierungsempfehlungen", AMBIENT_GREEN, font_size=LABEL_FONT_SIZE - 2)
        recommend.move_to(np.array([0.0, 0.5, 0.0]))
        better = Arrow(np.array([-2.0, -0.1, 0.0]), np.array([2.0, -0.1, 0.0]), buff=0.0,
                       color=AMBIENT_GREEN, stroke_width=3, max_tip_length_to_length_ratio=0.1)
        envelope_old = _envelope(h_old, AMBIENT_GREEN)
        none_chip = chip("keine sinnvolle Maßnahme möglich", P_ORANGE, font_size=LABEL_FONT_SIZE - 2)
        none_chip.move_to(np.array([0.0, 0.5, 0.0]))
        no_measure = cross_mark(P_RED, 0.2).move_to(np.array([0.0, -0.1, 0.0]))
        warm_old = [h_old["center"] + UP * 0.1]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "empfehlung"))
        self.play(*clear(), run_time=0.7)
        self.play(Create(h_old["group"]), FadeIn(old_badge), *_heat(warm_old, 1.4), run_time=1.4)
        self.play(Create(envelope_old), FadeIn(recommend, shift=UP * 0.1), *_heat(warm_old, 1.4), run_time=1.4)
        self.play(GrowArrow(better), Create(h_new["group"]), FadeIn(new_badge), *_heat(warm_old, 1.4), run_time=1.6)
        hold_for(self, self.NARRATION, "empfehlung", during=lambda rt: _heat(warm_old, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "keine"))
        self.play(FadeOut(h_new["group"]), FadeOut(new_badge), FadeOut(better),
                  ReplacementTransform(recommend, none_chip), *_heat(warm_old, 1.4), run_time=1.4)
        self.play(FadeIn(no_measure), *_heat(warm_old, 1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "keine", during=lambda rt: _heat(warm_old, rt))

        self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)
        self.wait(0.4)

#endregion
