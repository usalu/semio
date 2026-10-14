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
    smooth_path, flow_guides, flow_animation, ripples,
    house_section, open_window, person_glyph, lamp_glyph, thermometer_glyph, radiator,
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


def _footprint(hs, color=P_TEAL):
    """📐 Translucent cross-section area of a ``house_section`` — the geometry the calculation starts from."""
    return Rectangle(width=hs["w_width"], height=hs["w_height"], stroke_width=0,
                     fill_color=color, fill_opacity=0.16).move_to(hs["center"])


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


def _stream(paths, color, rt, *, color_end=None, speed=0.9, waves=3):
    """💨 Particle air or energy stream along ``paths`` for ``rt`` seconds."""
    return flow_animation([(list(paths), color, color_end)], waves=waves, cycles=max(1.0, speed * rt))
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

        hx, hy, sx, sy, s = 4.9, -0.45, 1.25, 0.1, 0.8
        left = house_section(np.array([-hx, hy, 0.0]), scale=s)
        right = house_section(np.array([hx, hy, 0.0]), scale=s)
        floor_y, level_y = left["bottom_left"][1], left["level_1"].get_center()[1]

        left_sheet = _document([-sx, sy, 0], color=P_WHITE, width=2.2, height=2.5, lines=0)
        right_sheet = _document([sx, sy, 0], color=VERBRAUCH_ORANGE, width=2.2, height=2.5, lines=0)
        sheet_top = sy + 1.25 + 0.24
        sheet_title = body_text("Energieausweis", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(
            np.array([-sx, sheet_top, 0.0]))
        left_title = body_text("Bedarfsausweis", font_size=LABEL_FONT_SIZE, color=BEDARF_CYAN).move_to(sheet_title)
        right_title = body_text("Verbrauchsausweis", font_size=LABEL_FONT_SIZE, color=VERBRAUCH_ORANGE).move_to(
            np.array([sx, sheet_top, 0.0]))

        rule_ys = (0.62, 0.08, -0.46)
        rules = VGroup(*[
            Line(np.array([-sx - 0.9, y, 0.0]), np.array([-sx + 0.9, y, 0.0]), color=P_WHITE,
                 stroke_width=1.1, stroke_opacity=0.4)
            for y in rule_ys
        ])
        input_rows = VGroup(*[
            body_text(name, font_size=LABEL_FONT_SIZE - 1, color=BEDARF_CYAN).move_to(
                np.array([-sx - 0.9, y + 0.2, 0.0])).align_to(np.array([-sx - 0.9, y, 0.0]), LEFT)
            for name, y in zip(("Geometrie", "U-Werte", "Anlage"), rule_ys)
        ])
        computed = body_text("berechnet", font_size=LABEL_FONT_SIZE - 1, color=BEDARF_CYAN).move_to(
            np.array([-sx, sy - 0.92, 0.0]))

        norm_person = _stand(left, -hx - 0.55, P_CYAN)
        norm_lamp = lamp_glyph(np.array([-hx - 0.05, level_y, 0.0]), drop=0.1)
        norm_therm = thermometer_glyph(np.array([-hx + 0.45, floor_y + 0.2, 0.0]), height=0.55,
                                       color=P_CYAN, level=0.5)
        norm_temp = math_label(r"20\,\mathrm{°C}", at=np.array([-hx + 0.7, floor_y + 0.42, 0.0]),
                               size=LABEL_FONT_SIZE - 2, color=P_CYAN, edge="left")
        norm_radiator = radiator(np.array([-hx - 1.08, floor_y + 0.17, 0.0]), color=P_RED).scale(0.45)
        norm_label = body_text("Norm-Randbedingungen", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            np.array([-hx, 1.78, 0.0]))
        area = _footprint(left)
        envelope = _envelope(left)

        real_label = body_text("real, witterungsbereinigt", font_size=LABEL_FONT_SIZE, color=VERBRAUCH_ORANGE).move_to(
            np.array([hx, 1.78, 0.0]))
        walker = _stand(right, hx - 0.65, P_ORANGE)
        sleeper = _stand(right, hx + 0.55, P_ORANGE, storey=1)
        real_lamp = lamp_glyph(np.array([hx + 0.5, level_y, 0.0]), drop=0.1)
        real_therm = thermometer_glyph(np.array([hx + 1.05, floor_y + 0.2, 0.0]), height=0.55,
                                       color=P_ORANGE, level=0.35)
        window = right["windows"][0]

        left_rail = smooth_path([np.array([-hx + 1.44, -0.45, 0.0]), np.array([-2.95, -0.40, 0.0]),
                                 np.array([-sx - 1.15, -0.45, 0.0])])
        right_rail = smooth_path([np.array([hx - 1.44, -0.45, 0.0]), np.array([2.95, -0.40, 0.0]),
                                  np.array([sx + 1.15, -0.45, 0.0])])
        vent_rail = smooth_path([np.array([hx - 1.0, -0.95, 0.0]), np.array([3.2, -0.9, 0.0]),
                                 np.array([2.7, -0.78, 0.0])])
        left_guide = flow_guides([left_rail], P_CYAN)
        right_guide = flow_guides([right_rail], VERBRAUCH_ORANGE)

        months, clean = ValueTracker(0.0), ValueTracker(0.0)
        bars = _month_bars(sx - 0.9, sy - 0.45, months, clean, width=1.8, height=0.95,
                           color=VERBRAUCH_ORANGE)
        bars_floor = Line(np.array([sx - 0.9, sy - 0.45, 0.0]), np.array([sx + 0.9, sy - 0.45, 0.0]),
                          color=P_WHITE, stroke_width=1.4)
        month_read = math_readout(lambda: rf"{de_num(months.get_value())}\,\text{{Monate}}",
                                  np.array([sx, sy - 0.78, 0.0]), size=LABEL_FONT_SIZE, color=VERBRAUCH_ORANGE,
                                  edge="center")
        corrected = body_text("witterungsbereinigt", font_size=LABEL_FONT_SIZE - 3, color=BEDARF_CYAN).move_to(
            np.array([sx, sy + 0.68, 0.0]))

        def left_heat(rt, r_max=0.3):
            return [ripples([norm_person.get_top() + UP * 0.05, norm_radiator.get_top() + UP * 0.02],
                            r_max=r_max, cycles=rt / 1.4, color=P_RED),
                    _stream([left_rail], P_CYAN, rt)]

        def both_heat(rt, r_max=0.3):
            return [
                ripples([walker.get_top() + UP * 0.05, sleeper.get_top() + UP * 0.05],
                        r_max=r_max, cycles=rt / 1.4, color=P_RED),
                _stream([left_rail], P_CYAN, rt),
                _stream([right_rail], VERBRAUCH_ORANGE, rt),
                _stream([vent_rail], P_RED, rt, color_end=P_BLUE, speed=1.0),
            ]

        self.play(Create(left["group"]), run_time=1.2)
        self.play(Create(left_sheet), FadeIn(sheet_title), Create(rules), Create(left_guide), run_time=0.8)
        hold_for(self, self.NARRATION, "intro",
                 used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 0.8,
                 during=lambda rt: [_stream([left_rail], P_TEAL, rt)])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bedarf"))
        self.play(
            left_sheet[0].animate.set_stroke(BEDARF_CYAN), left_sheet[1].animate.set_fill(BEDARF_CYAN, opacity=0.55),
            ReplacementTransform(sheet_title, left_title), FadeIn(computed),
            FadeIn(norm_person), FadeIn(norm_lamp["group"]), FadeIn(norm_label),
            FadeIn(norm_therm["group"]), FadeIn(norm_therm["column"]), FadeIn(norm_temp),
            *left_heat(1.1), run_time=1.1,
        )
        self.play(FadeIn(input_rows[0], shift=RIGHT * 0.15), FadeIn(area), *left_heat(0.9), run_time=0.9)
        self.play(FadeIn(input_rows[1], shift=RIGHT * 0.15), Create(envelope), *left_heat(0.9), run_time=0.9)
        self.play(FadeIn(input_rows[2], shift=RIGHT * 0.15), FadeIn(norm_radiator), *left_heat(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "bedarf", during=left_heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verbrauch"))
        self.play(
            Create(right["group"]), Create(right_sheet), FadeIn(right_title), Create(bars_floor),
            Create(right_guide), run_time=1.1,
        )
        self.add(real_therm["column"], month_read, bars)
        self.play(
            FadeIn(walker), FadeIn(sleeper), FadeIn(real_lamp["group"]), FadeIn(real_therm["group"]),
            FadeIn(real_label), *both_heat(0.9), run_time=0.9,
        )
        self.play(
            months.animate.set_value(MONTHS), real_therm["level"].animate.set_value(0.8),
            open_window(window, run_time=1.0), *both_heat(2.6), run_time=2.6, rate_func=linear,
        )
        self.play(clean.animate.set_value(1.0), FadeIn(corrected), real_therm["level"].animate.set_value(0.5),
                  *both_heat(1.3), run_time=1.3)
        levels = iter([0.75, 0.4, 0.65, 0.45])
        hold_for(self, self.NARRATION, "verbrauch",
                 during=lambda rt: [*both_heat(rt), real_therm["level"].animate.set_value(next(levels, 0.5))])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unterschied"))
        row_a, box_a, items_a = math_panel([
            ("bed", r"\text{Bedarf}", BEDARF_CYAN), (None, "=", P_WHITE),
            ("geb", r"\text{Gebäude}\;|_{\,\text{Norm}}", P_TEAL),
            (None, r"\quad\quad", P_WHITE),
            ("ver", r"\text{Verbrauch}", VERBRAUCH_ORANGE), (None, "=", P_WHITE),
            ("geb2", r"\text{Gebäude}", P_TEAL), (None, "+", P_WHITE),
            ("use", r"\text{Nutzung}", VERBRAUCH_ORANGE),
        ], size=BODY_FONT_SIZE + 2)
        envelope_r = _envelope(right)
        self.play(Create(envelope_r), Create(box_a), FadeIn(row_a), *both_heat(1.3), run_time=1.3)
        use_ring = highlight_param(items_a, "use", color=VERBRAUCH_ORANGE)
        self.play(
            Indicate(walker, color=VERBRAUCH_ORANGE), Indicate(sleeper, color=VERBRAUCH_ORANGE),
            Indicate(real_lamp["group"], color=VERBRAUCH_ORANGE),
            Create(use_ring), *both_heat(1.2), run_time=1.2,
        )
        hold_for(self, self.NARRATION, "unterschied", during=both_heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wann"))
        row_b, box_b, items_b = math_panel([
            ("neu", r"\text{Neubau}", P_TEAL), (None, r"\Rightarrow", P_WHITE),
            ("bed", r"\text{Bedarfsausweis}", BEDARF_CYAN),
            (None, r"\quad\quad", P_WHITE),
            ("alt", r"\text{Bestand}", P_TEAL), (None, r"\Rightarrow", P_WHITE),
            ("bed2", r"\text{Bedarf}", BEDARF_CYAN), (None, r"\text{oder}", P_WHITE),
            ("ver", r"\text{Verbrauch}", VERBRAUCH_ORANGE),
        ], size=BODY_FONT_SIZE + 2)
        self.play(ReplacementTransform(row_a, row_b), ReplacementTransform(box_a, box_b), FadeOut(use_ring),
                  left_sheet.animate.set_stroke(BEDARF_CYAN, width=5), *both_heat(1.3), run_time=1.3)
        self.play(Indicate(left_sheet, color=BEDARF_CYAN, scale_factor=1.04), *both_heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "wann", during=both_heat)

        self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)
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
         "Same walls, same windows, same useful heating demand. The plant moves the letter too, not the envelope alone.",
         "Gleiche Wände, gleiche Fenster, gleicher Heizwärmebedarf. Den Buchstaben bestimmt die Anlage mit, nicht die Hülle allein."),
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

        sheet_l = _document([-2.3, 0.85, 0], color=BEDARF_CYAN, width=1.7, height=0.95, lines=2)
        sheet_r = _document([2.3, 0.85, 0], color=VERBRAUCH_ORANGE, width=1.7, height=0.95, lines=2)
        name_l = body_text("Bedarf", font_size=LABEL_FONT_SIZE, color=BEDARF_CYAN).next_to(sheet_l, UP, buff=0.12)
        name_r = body_text("Verbrauch", font_size=LABEL_FONT_SIZE, color=VERBRAUCH_ORANGE).next_to(sheet_r, UP, buff=0.12)
        feed_l = Arrow(sheet_l.get_bottom() + DOWN * 0.04, np.array([-0.3, band_top + 0.04, 0.0]), buff=0.05,
                       color=BEDARF_CYAN, stroke_width=3, max_tip_length_to_length_ratio=0.12)
        feed_r = Arrow(sheet_r.get_bottom() + DOWN * 0.04, np.array([0.3, band_top + 0.04, 0.0]), buff=0.05,
                       color=VERBRAUCH_ORANGE, stroke_width=3, max_tip_length_to_length_ratio=0.12)

        self.play(
            LaggedStart(*[FadeIn(b, shift=UP * 0.1) for b in strip["bands"]], lag_ratio=0.07), run_time=1.7)
        self.play(FadeIn(bounds), FadeIn(unit_label), FadeIn(schematic_note), Create(sheet_l), Create(sheet_r),
                  FadeIn(name_l), FadeIn(name_r), run_time=0.8)
        self.play(GrowArrow(feed_l), GrowArrow(feed_r), run_time=0.9)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.7 + 0.8 + 0.9)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "werte"))
        house = house_section(np.array([-1.6, 0.82, 0.0]), scale=0.52)
        ground = float(house["bottom_left"][1])
        warm = [house["center"] + UP * 0.1]

        def heat(rt, r_max=0.28):
            return [ripples(warm, r_max=r_max, cycles=rt / 1.4, color=P_RED)]

        chip_end = chip("Endenergie", END_RED).move_to(np.array([1.2, 0.9, 0.0]))
        chip_prim = chip("Primärenergie", PRIM_VIOLET).move_to(np.array([4.4, 0.9, 0.0]))
        arrow_end = Arrow(chip_end.get_bottom(), np.array([1.2, band_top + 0.04, 0.0]), buff=0.06,
                          color=END_RED, stroke_width=3, max_tip_length_to_length_ratio=0.12)
        arrow_prim = Arrow(chip_prim.get_bottom(), np.array([4.4, band_top + 0.55, 0.0]), buff=0.06,
                           color=PRIM_VIOLET, stroke_width=3, max_tip_length_to_length_ratio=0.2)
        arrow_prim.set_stroke(opacity=0.55)
        blocked = cross_mark(PRIM_VIOLET, 0.13).move_to(np.array([4.4, band_top + 0.3, 0.0]))

        self.play(
            FadeOut(sheet_l), FadeOut(sheet_r), FadeOut(name_l), FadeOut(name_r), FadeOut(feed_l), FadeOut(feed_r),
            FadeOut(schematic_note), Create(house["group"]), *heat(1.2), run_time=1.2,
        )
        self.play(FadeIn(chip_end), FadeIn(chip_prim), *heat(0.6), run_time=0.6)
        self.play(GrowArrow(arrow_end), GrowArrow(arrow_prim), FadeIn(blocked), *heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "werte", used=1.2 + 0.6 + 1.0 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gas"))
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
        qh_right = DashedLine(np.array([house["bottom_right"][0] + 0.01, qh_y, 0.0]),
                              np.array([gx - 0.4, qh_y, 0.0]), color=BEDARF_CYAN, stroke_width=2.2, dash_length=0.1)
        qh_label = math_label(r"Q_{h}=100", at=np.array([-0.1, qh_y - 0.3, 0.0]), size=LABEL_FONT_SIZE - 2,
                              color=BEDARF_CYAN, edge="center")
        loss_label = body_text("Anlagenverluste", font_size=LABEL_FONT_SIZE - 3, color=END_RED)
        loss_label.move_to(np.array([gx + 0.55, ground + 1.2 * Q_NUTZ * STAGE_U, 0.0])).align_to(
            np.array([gx + 0.55, 0.0, 0.0]), LEFT)

        self.play(FadeOut(chip_end), FadeOut(chip_prim), FadeOut(arrow_end), FadeOut(arrow_prim), FadeOut(blocked),
                  FadeIn(gas_name), run_time=0.5)
        self.add(gas_bar, gas_read, gas_ptr)
        self.play(Create(qh_right), FadeIn(qh_label), v_gas.animate.set_value(GAS_END),
                  FadeIn(loss_label, run_time=0.4), *heat(2.6), run_time=2.6, rate_func=smooth)
        gas_ring = SurroundingRectangle(strip["bands"][_class_index(GAS_END)], color=P_ORANGE, buff=0.0,
                                        stroke_width=4)
        self.play(Create(gas_ring), *heat(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "gas", used=0.5 + 2.6 + 0.7 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "wp"))
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
        qh_left = DashedLine(np.array([house["bottom_left"][0] - 0.01, qh_y, 0.0]),
                             np.array([wx + 0.4, qh_y, 0.0]), color=BEDARF_CYAN, stroke_width=2.2, dash_length=0.1)
        ambient = Rectangle(width=0.8, height=(Q_NUTZ - WP_END) * STAGE_U, color=AMBIENT_GREEN, stroke_width=1.2,
                            fill_color=AMBIENT_GREEN, fill_opacity=0.16).move_to(
            np.array([wx, ground + (WP_END + Q_NUTZ) / 2 * STAGE_U, 0.0]))
        ambient_label = body_text("Umweltwärme", font_size=LABEL_FONT_SIZE - 3, color=AMBIENT_GREEN)
        ambient_label.move_to(np.array([wx - 0.55, ground + 0.65 * Q_NUTZ * STAGE_U, 0.0])).align_to(
            np.array([wx - 0.55, 0.0, 0.0]), RIGHT)

        self.play(FadeIn(wp_name), run_time=0.4)
        self.add(wp_bar, wp_read, wp_ptr)
        self.play(Create(qh_left), v_wp.animate.set_value(WP_END), *heat(1.8), run_time=1.8, rate_func=smooth)
        wp_ring = SurroundingRectangle(strip["bands"][_class_index(WP_END)], color=AMBIENT_GREEN, buff=0.0,
                                       stroke_width=4)
        self.play(Create(wp_ring), FadeIn(ambient), FadeIn(ambient_label), *heat(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "wp", used=0.4 + 1.8 + 0.8 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lesen"))
        ep_gas, ep_wp = de_num(GAS_END / Q_NUTZ, 2), de_num(WP_END / Q_NUTZ, 2)
        row, box, items = math_panel([
            ("qh", r"Q_{h}=100", BEDARF_CYAN), (None, r"\Rightarrow", P_WHITE),
            ("gas", rf"100\cdot{ep_gas}={GAS_END}\to\mathrm{{{_class_letter(GAS_END)}}}", P_ORANGE),
            (None, r"\quad|\quad", P_WHITE),
            ("wp", rf"100\cdot{ep_wp}={WP_END}\to\mathrm{{{_class_letter(WP_END)}}}", AMBIENT_GREEN),
        ], size=BODY_FONT_SIZE + 2)
        envelope = _envelope(house)
        self.play(Create(box), FadeIn(row), Create(envelope), *heat(1.2), run_time=1.2)
        self.play(Create(highlight_param(items, "qh", color=BEDARF_CYAN)), Indicate(qh_right, color=BEDARF_CYAN),
                  Indicate(qh_left, color=BEDARF_CYAN), *heat(1.0), run_time=1.0)
        self.play(Create(highlight_param(items, "gas", color=P_ORANGE)), Indicate(gas_bar, color=P_ORANGE),
                  *heat(0.9), run_time=0.9)
        self.play(Create(highlight_param(items, "wp", color=AMBIENT_GREEN)), Indicate(wp_bar, color=AMBIENT_GREEN),
                  *heat(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "lesen", used=1.2 + 1.0 + 0.9 + 0.9 + 0.35, during=heat)

        self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)
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

        class_color = CLASSES[_class_index(GAS_END)][2]
        class_letter = CLASSES[_class_index(GAS_END)][0]
        listing = RoundedRectangle(corner_radius=0.10, width=6.4, height=3.8, color=P_TEAL, stroke_width=2.4)
        listing.move_to(np.array([-3.35, 0.0, 0.0]))
        listing.set_fill(P_DEEP_DARK, opacity=0.9)
        listing_head = body_text("Pflichtangaben nach GEG § 87", font_size=LABEL_FONT_SIZE - 2, color=P_TEAL)
        listing_head.next_to(listing.get_top(), DOWN, buff=0.22)

        house = house_section(np.array([-5.4, -0.2, 0.0]), scale=0.5)
        ground = float(house["bottom_left"][1])
        warm = [house["center"] + UP * 0.1]
        v_end = ValueTracker(0.0)
        bar_x, bar_unit = -3.95, 0.0058
        end_bar = _end_bar(bar_x, ground, v_end, [(0, Q_NUTZ, BEDARF_CYAN), (Q_NUTZ, GAS_END, END_RED)],
                           width=0.34, unit=bar_unit)
        gas_rail = smooth_path([np.array([bar_x - 0.17, ground + 0.45, 0.0]), np.array([-4.3, ground + 0.5, 0.0]),
                                np.array([house["bottom_right"][0], ground + 0.5, 0.0])])
        gas_guide = flow_guides([gas_rail], P_ORANGE)

        def heat(rt, r_max=0.28):
            return [ripples(warm, r_max=r_max, cycles=rt / 1.4, color=P_RED)]

        keys = ("Ausweisart", "Endenergiekennwert", "wesentlicher Energieträger", "Baujahr (Wohngebäude)")
        row_y = (1.1, 0.5, -0.1, -0.7)
        key_texts = VGroup(*[
            body_text(k, font_size=LABEL_FONT_SIZE - 4, color=P_WHITE).set_opacity(0.75).move_to(
                np.array([-3.5, y, 0.0])).align_to(np.array([-3.5, y, 0.0]), LEFT)
            for k, y in zip(keys, row_y)
        ])
        value_at = [np.array([-3.5, y - 0.29, 0.0]) for y in row_y]
        v_type = body_text("Bedarfsausweis", font_size=LABEL_FONT_SIZE, color=BEDARF_CYAN)
        v_type.move_to(value_at[0]).align_to(value_at[0], LEFT)
        v_end_read = math_readout(lambda: rf"{de_num(v_end.get_value())}\,{UNIT}", value_at[1] + DOWN * 0.1,
                                  size=LABEL_FONT_SIZE, color=END_RED, edge="left")
        v_carrier = body_text("Erdgas", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        v_carrier.move_to(value_at[2]).align_to(value_at[2], LEFT)
        v_year = body_text("1996", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        v_year.move_to(value_at[3]).align_to(value_at[3], LEFT)
        mini_doc = _document([-1.05, row_y[0] - 0.15, 0], color=BEDARF_CYAN, width=0.7, height=0.85, lines=3)

        self.play(Create(listing), FadeIn(listing_head), Create(house["group"]), *heat(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "angaben"))
        self.play(FadeIn(key_texts[0]), FadeIn(v_type, shift=RIGHT * 0.1), Create(mini_doc), *heat(0.9), run_time=0.9)
        self.add(end_bar, v_end_read)
        self.play(FadeIn(key_texts[1]), v_end.animate.set_value(GAS_END), *heat(1.4), run_time=1.4, rate_func=smooth)
        self.play(FadeIn(key_texts[2]), FadeIn(v_carrier, shift=RIGHT * 0.1), Create(gas_guide), *heat(0.9),
                  run_time=0.9)
        self.play(FadeIn(key_texts[3]), FadeIn(v_year, shift=RIGHT * 0.1), Indicate(house["group"], color=P_WHITE,
                                                                                    scale_factor=1.03),
                  *heat(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "angaben", used=0.9 + 1.4 + 0.9 + 0.9 + 0.35,
                 during=lambda rt: [*heat(rt), _stream([gas_rail], P_ORANGE, rt)])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "klasse"))
        strip = _class_strip([3.35, 1.2, 0.0], width=5.4, height=0.5, font=LABEL_FONT_SIZE - 4)
        v_scale = ValueTracker(0.0)
        scale_ptr = _pointer(strip, v_scale, P_ORANGE)
        fly = math_label(rf"{de_num(GAS_END)}\,{UNIT}", at=value_at[1] + DOWN * 0.1,
                         size=LABEL_FONT_SIZE, color=END_RED, edge="left")
        self.play(LaggedStart(*[FadeIn(b, shift=UP * 0.08) for b in strip["bands"]], lag_ratio=0.06), run_time=1.0)
        self.add(fly, scale_ptr)
        self.play(
            fly.animate.move_to(np.array([3.35, 0.55, 0.0])),
            v_scale.animate.set_value(GAS_END),
            *heat(2.0), run_time=2.0, rate_func=smooth,
        )
        cell = strip["bands"][_class_index(GAS_END)]
        badge = Rectangle(width=0.62, height=0.62, color=class_color, fill_color=class_color, fill_opacity=0.92,
                          stroke_width=1.2).move_to(np.array([-1.0, -1.42, 0.0]))
        badge_letter = body_text(class_letter, font_size=BODY_FONT_SIZE + 4, color=P_DEEP_DARK).move_to(badge)
        badge_tag = body_text("Effizienzklasse", font_size=LABEL_FONT_SIZE - 4, color=P_WHITE)
        badge_tag.set_opacity(0.75).next_to(badge, LEFT, buff=0.25)
        flight = cell.copy()
        self.play(FadeOut(fly), Indicate(cell, color=P_WHITE, scale_factor=1.15), *heat(0.7), run_time=0.7)
        self.add(flight)
        self.play(ReplacementTransform(flight, VGroup(badge, badge_letter)), FadeIn(badge_tag), *heat(1.3),
                  run_time=1.3)
        hold_for(self, self.NARRATION, "klasse", used=1.0 + 2.0 + 0.7 + 1.3 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "folgen"))
        ground_r = -1.15
        h_wp = house_section(np.array([1.7, ground_r + 0.5 * 0.8, 0.0]), scale=0.4)
        h_gas = house_section(np.array([5.0, ground_r + 0.5 * 0.8, 0.0]), scale=0.4)
        buyer = _stand(h_gas, 2.8, P_ORANGE)
        tenant = _stand(h_gas, 3.6, P_YELLOW)
        v_wp = ValueTracker(0.0)
        wp_ptr = _pointer(strip, v_wp, AMBIENT_GREEN)
        wp_cell = strip["bands"][_class_index(WP_END)]
        wp_color, wp_letter = CLASSES[_class_index(WP_END)][2], CLASSES[_class_index(WP_END)][0]
        wp_badge = VGroup(
            Rectangle(width=0.5, height=0.5, color=wp_color, fill_color=wp_color, fill_opacity=0.92,
                      stroke_width=1.0).move_to(np.array([1.7, -1.55, 0.0])),
        )
        wp_badge.add(body_text(wp_letter, font_size=BODY_FONT_SIZE, color=P_DEEP_DARK).move_to(wp_badge[0]))
        gas_badge = VGroup(
            Rectangle(width=0.5, height=0.5, color=class_color, fill_color=class_color, fill_opacity=0.92,
                      stroke_width=1.0).move_to(np.array([5.0, -1.55, 0.0])),
        )
        gas_badge.add(body_text(class_letter, font_size=BODY_FONT_SIZE, color=P_DEEP_DARK).move_to(gas_badge[0]))
        who = VGroup(
            body_text("Käufer", font_size=LABEL_FONT_SIZE - 3, color=P_ORANGE).next_to(buyer, UP, buff=0.12),
            body_text("Mieter", font_size=LABEL_FONT_SIZE - 3, color=P_YELLOW).next_to(tenant, UP, buff=0.12),
        )
        compare = DoubleArrow(np.array([2.3, -1.55, 0.0]), np.array([4.4, -1.55, 0.0]), buff=0.05, color=P_WHITE,
                              stroke_width=2.5, max_tip_length_to_length_ratio=0.1)
        costs = chip("genaue Kosten · Sanierungsaufwand", P_TEAL, font_size=LABEL_FONT_SIZE - 1)
        costs.move_to(np.array([3.35, -2.15, 0.0]))
        no_costs = cross_mark(P_RED, 0.12).next_to(costs, LEFT, buff=0.15)

        self.play(
            Create(h_wp["group"]), Create(h_gas["group"]), FadeIn(buyer), FadeIn(tenant), FadeIn(who),
            *heat(1.2), run_time=1.2,
        )
        self.add(wp_ptr)
        self.play(v_wp.animate.set_value(WP_END), FadeIn(wp_badge), FadeIn(gas_badge), *heat(1.4), run_time=1.4)
        self.play(GrowArrow(compare), *heat(0.8), run_time=0.8)
        self.play(FadeIn(costs), FadeIn(no_costs), *heat(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "folgen", used=1.2 + 1.4 + 0.8 + 0.8 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "empfehlung"))
        recommend = chip("Modernisierungsempfehlungen", AMBIENT_GREEN, font_size=LABEL_FONT_SIZE - 3)
        recommend.move_to(np.array([4.95, 0.52, 0.0]))
        better = Arrow(recommend.get_left() + LEFT * 0.05, np.array([1.9, 0.52, 0.0]), buff=0.0,
                       color=AMBIENT_GREEN, stroke_width=3, max_tip_length_to_length_ratio=0.15)
        envelope_gas = _envelope(h_gas, AMBIENT_GREEN)
        self.play(
            FadeOut(costs), FadeOut(no_costs), Create(envelope_gas), FadeIn(recommend, shift=UP * 0.1),
            *heat(1.4), run_time=1.4,
        )
        self.play(GrowArrow(better), *heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "empfehlung", used=1.4 + 1.0 + 0.35, during=heat)

        self.play(*[FadeOut(m) for m in list(self.mobjects) if m not in (title, subtitle, din)], run_time=0.6)
        self.wait(0.4)

#endregion
