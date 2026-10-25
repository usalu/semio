"""🌬️ Lüftung Kapitel 4 — natürliche Lüftung, im Winter wie im Sommer.

Fenster, Querlüftung, Auftrieb und Nachtlüftung bewegen Luft ohne Ventilator. Dieselbe Öffnung
dient im Winter dem kurzen Stoßlüften und im Sommer dem Spülen; die Grenzen der freien Lüftung
(keine Filterung, nicht regelbar, keine Wärmerückgewinnung) führen zur mechanischen Lüftung.
Laufendes Beispiel: 20 °C innen, 0 °C außen im Winter (Δθ = 20 K), 26 °C innen, 32 °C außen im Sommer.
"""

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
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, SAFE_BOTTOM, SAFE_BOTTOM_FORMULA, fit_band,
    symbol_token,
    smooth_path, flow_guides,
    meter, bind_meter, recolor_meter, chip, cross_mark, dim_arrow,
    highlight_param,
    math_label, math_panel, math_readout, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    sun_glyph, moon_glyph, ripples, flow_animation,
    window_glyph, radiator, clock_glyph, person_glyph,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Natürliche Lüftung"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``3_heating/2_conduction/scene_2.py``): a dim ``P_TEAL`` footnote
    that never competes with the diagram. The formula panel sits on the
    bottom edge, so this corner is clear in every beat.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _room(center, width, height, color=P_WHITE, t: float = 0.16):
    """🏠 Physical Fundamentals room section — ceiling slab and side walls, then the floor slab, centred on the outline."""
    cx, cy = float(center[0]), float(center[1])
    style = dict(color=color, stroke_width=2, fill_color=color, fill_opacity=0.14)
    ceiling = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy + height / 2, 0.0]))
    walls = VGroup(*[
        Rectangle(width=t, height=height - t, **style).move_to(np.array([cx + s * width / 2, cy, 0.0]))
        for s in (-1, 1)
    ])
    floor = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy - height / 2, 0.0]))
    return VGroup(VGroup(ceiling, walls), floor)


def _window(pos, height=1.15, width=0.26):
    """🪟 Facade opening — adjustable natural-ventilation aperture."""
    return Rectangle(
        width=width, height=height, color=P_CYAN, stroke_width=2.2,
        fill_color=P_DEEP_DARK, fill_opacity=1.0,
    ).move_to(pos)


def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Physical Fundamentals occupant — the comfort target of the strategy."""
    return person_glyph(pos, color=color, scale=1.09 * scale)


def _stand(room_c, room_h, x, color=P_ORANGE, t: float = 0.16):
    """🧍 Occupant at true scale — 1.75 m in a 2.5 m room, feet on the floor slab's top edge at ``x``."""
    height = 0.7 * (room_h - t)
    scale = height / (0.66 * 1.09)
    return _person(np.array([x, float(room_c[1]) - room_h / 2 + t / 2 + height / 2, 0.0]), color=color, scale=scale)


def _badge(title, value, color=P_TEAL):
    """🏷️ Compact callout card for design levers / comfort states."""
    import re

    temp = re.match(r"^(\d+)\s*°C(?:\s*·\s*(.+))?$", value)
    if temp:
        value_mob = math_label(rf"{temp.group(1)}\,\mathrm{{°C}}", size=BODY_FONT_SIZE, color=P_WHITE)
        if temp.group(2):
            value_mob = VGroup(
                value_mob,
                Text(temp.group(2), font_size=LABEL_FONT_SIZE, color=P_WHITE),
            ).arrange(RIGHT, buff=0.1)
    else:
        value_mob = Text(value, font_size=BODY_FONT_SIZE, color=P_WHITE)
    body = VGroup(
        Text(title, font_size=LABEL_FONT_SIZE, color=color),
        value_mob,
    ).arrange(DOWN, buff=0.06)
    box = SurroundingRectangle(body, color=color, corner_radius=0.1, buff=0.12, stroke_width=1.8)
    return VGroup(box, body)


def _hand_chip(text: str, src: str, color: str):
    """🔖 Hand-over chip — german lead-in plus a typeset quantity, boxed in the later video's colour."""
    label = VGroup(Text(text, font_size=LABEL_FONT_SIZE, color=color),
                   math_label(src, size=BODY_FONT_SIZE, color=color)).arrange(RIGHT, buff=0.14)
    return VGroup(SurroundingRectangle(label, color=color, corner_radius=0.1, buff=0.14, stroke_width=1.8), label)


def _streams(streams, rt: float, *, speed: float = 0.85, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)



def _open_sash(window, *, run_time: float = 0.9, close: bool = False):
    """🚪 Swing a ``window_glyph`` sash into the room — rightwards in a left wall, leftwards in a right wall."""
    angle = (-PI / 2 if window.get("side", "l") == "l" else PI / 2) * (-1 if close else 1)
    return Rotate(window["sash"], angle=angle, axis=UP, about_point=window["hinge"], run_time=run_time)


def _clock(center, r: float, minutes, *, color=P_CYAN):
    """🕐 Physical Fundamentals ``clock_glyph`` whose minute hand follows ``minutes`` (a ``ValueTracker``) — one turn is one hour."""
    dial = clock_glyph(center, r=r, color=color)
    face = dial["face"].set_fill(P_DEEP_DARK, opacity=0.9)
    ticks = dial["group"][1]

    def hand():
        mid, rad = face.get_center(), face.width / 2
        a = TAU * minutes.get_value() / 60.0
        return Line(mid, mid + 0.8 * rad * np.array([np.sin(a), np.cos(a), 0.0]), color=color, stroke_width=3)

    hand_mob = always_redraw(hand)
    pin = always_redraw(lambda: Dot(face.get_center(), radius=0.035, color=color))
    return {"face": face, "group": VGroup(face, ticks, hand_mob, pin), "static": VGroup(face, ticks)}


def _vbar(x: float, y0: float, height: float, frac, *, width: float = 0.26, color=P_CYAN):
    """📊 Vertical fill gauge from ``y0`` upward; ``frac()`` returns 0…1 every frame."""
    track = Rectangle(width=width, height=height, color=P_WHITE, stroke_width=2)
    track.set_fill(P_DEEP_DARK, opacity=1.0).move_to([x, y0 + height / 2, 0])

    def fill():
        h = max(0.004, float(np.clip(frac(), 0, 1)) * (height - 0.06))
        return Rectangle(width=width - 0.06, height=h, stroke_width=0, fill_color=color,
                         fill_opacity=0.85).move_to([x, y0 + 0.03 + h / 2, 0])

    return VGroup(track, always_redraw(fill))


def _vent_room(ox: float, floor_y: float, k: float, title: str, color):
    """🪟 Room section with exterior wall, window opening, radiator, thermostat and the reveal (Laibung).

    ``ox`` is the inner face of the back wall, ``k`` scene units per metre. The
    room is 4 m wide and 2,5 m high; the window sits 0,9 m … 2,2 m above the floor
    in a 0,35 m exterior wall with the radiator below it.
    """
    w, h, t = 4.0 * k, 2.5 * k, 0.35 * k
    x_in, x_out = ox + w, ox + w + t
    sill, lintel, ceil = floor_y + 0.9 * k, floor_y + 2.2 * k, floor_y + h
    style = dict(color=P_WHITE, stroke_width=2, fill_color=P_WHITE, fill_opacity=0.14)
    floor = Rectangle(width=w + t + 0.14, height=0.16, **style)
    floor.move_to([ox - 0.14 + (w + t + 0.14) / 2, floor_y - 0.08, 0])
    ceiling = floor.copy().move_to([floor.get_center()[0], ceil + 0.08, 0])
    back = Rectangle(width=0.14, height=h, **style).move_to([ox - 0.07, floor_y + h / 2, 0])
    brust = Rectangle(width=t, height=sill - floor_y, **style).move_to([(x_in + x_out) / 2, (floor_y + sill) / 2, 0])
    sturz = Rectangle(width=t, height=ceil - lintel, color=P_WHITE, stroke_width=2, fill_color=P_ORANGE,
                      fill_opacity=0.7).move_to([(x_in + x_out) / 2, (lintel + ceil) / 2, 0])
    laibung = Rectangle(width=t, height=lintel - sill, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.14)
    laibung.move_to([(x_in + x_out) / 2, (sill + lintel) / 2, 0])
    window = window_glyph((x_in + x_out) / 2, sill, lintel, depth=0.42, color=P_CYAN)
    window["side"] = "r"
    rad = radiator([x_in - 0.62, floor_y + 0.33, 0], color=P_RED)
    valve = Circle(radius=0.065, color=P_WHITE, stroke_width=2).set_fill(P_DEEP_DARK, 1.0)
    valve.move_to([x_in - 0.08, rad[0].get_top()[1] - 0.06, 0])
    head = math_label(title, size=BODY_FONT_SIZE, color=color)
    head.move_to([ox + w / 2, ceil + 0.36, 0])
    return {
        "x_in": x_in, "x_out": x_out, "sill": sill, "lintel": lintel, "ceil": ceil, "floor": floor_y,
        "ox": ox, "w": w, "k": k, "rad": rad, "valve": valve, "window": window, "sash": window["sash"],
        "sturz": sturz, "laibung": laibung, "head": head,
        "shell": VGroup(floor, ceiling, back, brust, sturz, laibung),
        "group": VGroup(floor, ceiling, back, brust, laibung, sturz, window["group"], rad, valve),
    }


def _valve_dial(room, opening):
    """🌡️ Thermostat dial mark — turns with ``opening()`` from closed (0) to fully open (1)."""
    def mark():
        c = room["valve"].get_center()
        a = PI / 2 - 0.9 * PI * float(np.clip(opening(), 0, 1))
        return Line(c, c + 0.06 * np.array([np.cos(a), np.sin(a), 0]), color=P_YELLOW, stroke_width=2)
    return always_redraw(mark)


def _energy_row(x0: float, y: float, unit: float, parts, *, height: float = 0.5):
    """🧱 Horizontal stacked bar redrawn every frame from ``(value_fn, colour)`` parts in Wh."""
    def build():
        row, cursor = VGroup(), x0
        for fn, colour in parts:
            w = max(0.0, float(fn())) * unit
            if w > 0.004:
                row.add(Rectangle(width=w, height=height, stroke_width=1.2, color=colour, fill_color=colour,
                                  fill_opacity=0.72).move_to([cursor + w / 2, y, 0]))
            cursor += w
        return row
    return always_redraw(build)


#endregion


#region Shared numbers
"""🔢 Laufendes Beispiel — Zahlenquellen: Kapitel 2 (V, n), Kapitel 3 (c_Luft, Δθ_Winter, Δθ_Sommer).

c_Luft = ρ·c_p/3600 = 1,2 · 1005 / 3600 ≈ 0,34 Wh/(m³·K); Winter 20 °C innen gegen 0 °C außen (Δθ = 20 K),
Sommer 26 °C innen gegen 32 °C außen am Tag und 18 °C in der Nacht.
"""
RHO_AIR, CP_AIR = 1.2, 1005.0
C_LUFT = round(RHO_AIR * CP_AIR / 3600.0, 2)
D_THETA = 20.0
V_NET, N_RATE = 440.0, 0.5
T_IN_WINTER, T_OUT_WINTER = 20.0, 0.0
T_IN_SUMMER, T_OUT_NIGHT, T_OUT_DAY = 26.0, 18.0, 32.0
G_ACC = 9.81


def _rho_air(theta: float) -> float:
    """🌡️ Dichte trockener Luft bei ``theta`` °C aus dem idealen Gasgesetz, ρ = p/(R·T) mit p = 101325 Pa.

    https://de.wikipedia.org/wiki/Dichte_(Luft)
    """
    return 101325.0 / (287.05 * (273.15 + theta))


def _stack_dp(height_m: float, t_out: float, t_in: float) -> float:
    """⬆️ Thermischer Auftrieb Δp = h · g · (ρ_a − ρ_i) in Pa.

    https://de.wikipedia.org/wiki/Kamineffekt
    """
    return height_m * G_ACC * (_rho_air(t_out) - _rho_air(t_in))
#endregion


#region Beat1 – The window is a dial, not a switch
class Beat1_Fensterregeln(Scene):
    NARRATION = [
        ("intro",
         "Natural ventilation is adjustable, in winter as in summer. A single facade opening already gives you a dial, not a switch.",
         "Natürliche Lüftung ist einstellbar, Winter wie Sommer —\neine Fassadenöffnung ist ein Regler, kein Schalter."),
        ("closed",
         "Fully closed, the air change rate drops to almost nothing. Heat and moisture given off by the occupants build up and comfort collapses.",
         "Ganz geschlossen sinkt der Luftwechsel fast auf null —\nWärme und Feuchte stauen sich, der Komfort bricht ein."),
        ("partial",
         "Open a narrow gap and the air change rate climbs into the comfort band: enough fresh air, and still no draught.",
         "Ein schmaler Spalt hebt den Luftwechsel ins Komfortband —\ngenug Frischluft, keine Zugluft."),
        ("cooler",
         "On a summer night the outdoor air is cooler than the room. Open wide: the very same window now flushes heat out of the space.",
         "In der Sommernacht ist die Außenluft kühler als der Raum —\nweit öffnen, das Fenster spült jetzt Wärme hinaus."),
        ("hotter",
         "On a summer afternoon the outdoor air is hotter. Throttle back: a wide opening would import heat instead of removing it.",
         "Am Sommernachmittag ist es draußen heißer — drosseln,\neine weite Öffnung würde Wärme hereinholen."),
        ("winter",
         "In winter the outdoor air is always colder. Open wide, but only briefly: every minute carries heat out, so the duration counts as much as the width.",
         "Im Winter ist die Außenluft immer kälter — weit, aber nur kurz\nöffnen: jede Minute trägt Wärme hinaus."),
        ("rule",
         "So the rule is the same in both seasons: set the window for the people inside, judged against the outdoor temperature and the time it stays open.",
         "Die Regel gilt in beiden Jahreszeiten: das Fenster nach den Menschen,\nder Außentemperatur und der Öffnungsdauer einstellen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Das Fenster ist ein Regler", title)
        din = _din_ref("DIN 1946-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([-1.55, 0.15, 0.0])
        shell = _room(room_c, 5.6, 2.5)
        room, floor = shell[0], shell[1]
        wash = Rectangle(
            width=5.5, height=2.4, stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
        ).move_to(room_c)
        person = _stand(room_c, 2.5, room_c[0] + 1.3)

        win_h = 1.42
        frame = Rectangle(
            width=0.30, height=win_h, color=P_CYAN, stroke_width=2.4,
            fill_color=P_DEEP_DARK, fill_opacity=1.0,
        ).move_to(np.array([room.get_left()[0], 0.30, 0.0]))
        sash = Rectangle(width=0.24, height=win_h - 0.06, stroke_width=1.6,
                         color=P_TEAL, fill_color=P_TEAL, fill_opacity=0.45)
        gap = Rectangle(width=0.24, height=0.006, stroke_width=0,
                        fill_color=P_CYAN, fill_opacity=0.40)
        opening = ValueTracker(0.0)

        def _shape_window(_m=None):
            span = win_h - 0.06
            g = max(0.006, opening.get_value() * span)
            gap.stretch_to_fit_height(g)
            gap.move_to(frame.get_bottom() + UP * (g / 2 + 0.03))
            s = max(0.006, span - g)
            sash.stretch_to_fit_height(s)
            sash.move_to(frame.get_top() + DOWN * (s / 2 + 0.03))

        _shape_window()
        sash.add_updater(_shape_window)

        def _open_lbl(text, color):
            return Text(text, font_size=LABEL_FONT_SIZE, color=color).next_to(
                frame, LEFT, buff=0.22, aligned_edge=DOWN)

        open_pct = _open_lbl("Öffnung 0 %", P_CYAN)
        open_mid = _open_lbl("Öffnung 35 %", P_CYAN)
        open_wide = _open_lbl("Öffnung 85 %", P_CYAN)
        open_thin = _open_lbl("Öffnung 15 %", P_ORANGE)

        outdoor_cool = _badge("Sommernacht", "18 °C", P_BLUE)
        outdoor_cool.move_to(np.array([-5.60, 1.55, 0.0]))
        outdoor_hot = _badge("Sommertag", "32 °C", P_RED).move_to(outdoor_cool)
        outdoor_winter = _badge("Winter", "0 °C", P_CYAN).move_to(outdoor_cool)

        def _inside(deg, color):
            tag = math_label(rf"\text{{innen}}\;{deg}\,\mathrm{{°C}}", size=LABEL_FONT_SIZE, color=color)
            return tag.move_to(room_c + np.array([1.85, 0.88, 0.0]))

        inside_summer, inside_winter = _inside(26, P_ORANGE), _inside(20, P_ORANGE)

        scaffold = VGroup(room, floor, wash, person, frame, sash, gap, open_pct, outdoor_cool, inside_summer)
        fit_band(scaffold)

        stuffy_spots = [person.get_top() + UP * 0.05, person.get_top() + LEFT * 1.6 + DOWN * 0.6]

        def stuffy(rt, r_max=0.6, color=P_RED):
            return [ripples(stuffy_spots, r_max=r_max, color=color, cycles=rt / 1.4)]

        air = meter("Luftwechsel n", length=2.5, thickness=0.55, color=P_CYAN)
        air["group"].move_to(np.array([3.45, 0.15, 0.0]))
        rate = ValueTracker(0.03)
        bind_meter(air, rate)

        track = air["track"]
        band_lo = track.get_bottom()[1] + 0.04 + 0.30 * air["span"]
        band_hi = track.get_bottom()[1] + 0.04 + 0.60 * air["span"]
        comfort = Rectangle(
            width=0.55, height=band_hi - band_lo, stroke_width=0,
            fill_color=P_GREEN, fill_opacity=0.22,
        ).move_to(np.array([track.get_center()[0], (band_lo + band_hi) / 2, 0.0]))
        comfort_lbl = Text("Komfort", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        comfort_lbl.next_to(comfort, RIGHT, buff=0.18)

        n_low = math_label(r"n \approx 0{,}2\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_RED)
        n_low.next_to(track, DOWN, buff=0.26)
        n_mid = math_label(r"n \approx 2\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_GREEN).move_to(n_low)
        n_high = math_label(r"n \approx 6\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_BLUE).move_to(n_low)
        n_thin = math_label(r"n \approx 1\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_ORANGE).move_to(n_low)
        open_short = _open_lbl("Öffnung 85 %", P_CYAN)
        n_short = math_label(r"n \approx 6\,\mathrm{h^{-1}}", size=BODY_FONT_SIZE, color=P_BLUE).move_to(n_low)

        tag_flush = Text("spülen", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        tag_flush.next_to(track, LEFT, buff=0.22).shift(UP * 0.75)
        tag_throttle = Text("drosseln", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(tag_flush)
        tag_short = Text("kurz", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(tag_flush)

        gentle = VGroup(
            smooth_path([
                frame.get_right() + RIGHT * 0.05 + DOWN * 0.25,
                room_c + LEFT * 1.4 + DOWN * 0.55,
                room_c + RIGHT * 0.6 + DOWN * 0.15,
            ]),
            smooth_path([
                room_c + RIGHT * 0.6 + UP * 0.30,
                room_c + LEFT * 1.4 + UP * 0.62,
                frame.get_right() + RIGHT * 0.05 + UP * 0.48,
            ]),
        )
        rule = Text("Fenster = Regler: Temperatur und Dauer", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        rule.move_to(np.array([room_c[0], -1.78, 0.0]))

        def draft(rt, speed=0.8, color=P_CYAN, color_end=None, waves=3):
            return [_streams([(gentle, color, color_end)], rt, speed=speed, waves=waves)]

        self.add(wash)
        self.play(Create(room), Create(floor), FadeIn(person), run_time=1.3)
        self.play(FadeIn(frame), FadeIn(sash), FadeIn(gap), FadeIn(open_pct), FadeIn(inside_summer), run_time=0.8)
        self.play(FadeIn(air["group"]), FadeIn(comfort), FadeIn(comfort_lbl), FadeIn(n_low), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.8 + 1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "closed"))
        self.play(wash.animate.set_fill(P_RED, opacity=0.24), *stuffy(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "closed", during=stuffy)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "partial"))
        self.play(
            opening.animate.set_value(0.35),
            rate.animate.set_value(0.45),
            ReplacementTransform(open_pct, open_mid),
            ReplacementTransform(n_low, n_mid),
            recolor_meter(air, P_GREEN),
            wash.animate.set_fill(P_CYAN, opacity=0.07),
            *stuffy(1.6, 0.35),
            run_time=1.6,
        )
        self.play(Create(flow_guides(gentle, P_CYAN)), run_time=0.6)
        self.play(*draft(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "partial", during=draft)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cooler"))
        self.play(FadeIn(outdoor_cool), *draft(0.6), run_time=0.6)
        self.play(
            opening.animate.set_value(0.85),
            rate.animate.set_value(0.85),
            ReplacementTransform(open_mid, open_wide),
            ReplacementTransform(n_mid, n_high),
            recolor_meter(air, P_BLUE),
            wash.animate.set_fill(P_BLUE, opacity=0.14),
            FadeIn(tag_flush), *draft(1.5, 1.2, P_BLUE, P_ORANGE, 4),
            run_time=1.5,
        )
        self.play(*draft(2.2, 1.2, P_BLUE, P_ORANGE, 4), run_time=2.2)
        hold_for(self, self.NARRATION, "cooler", during=lambda rt: draft(rt, 1.2, P_BLUE, P_ORANGE, 4))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "hotter"))
        self.play(
            ReplacementTransform(outdoor_cool, outdoor_hot),
            opening.animate.set_value(0.15),
            rate.animate.set_value(0.18),
            ReplacementTransform(open_wide, open_thin),
            ReplacementTransform(n_high, n_thin),
            recolor_meter(air, P_ORANGE),
            wash.animate.set_fill(P_RED, opacity=0.12),
            ReplacementTransform(tag_flush, tag_throttle),
            *draft(1.6, 0.45, P_ORANGE, None, 2),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "hotter", during=lambda rt: draft(rt, 0.45, P_ORANGE, None, 2))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        minutes = ValueTracker(0.0)
        dial = _clock(np.array([2.15, -0.45, 0.0]), 0.36, minutes, color=P_CYAN)
        dial_read = math_readout(
            lambda: rf"t \approx {de_num(minutes.get_value())}\,\mathrm{{min}}",
            np.array([2.15, -1.02, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN, edge="center",
        )
        self.play(
            ReplacementTransform(outdoor_hot, outdoor_winter),
            ReplacementTransform(inside_summer, inside_winter),
            opening.animate.set_value(0.85),
            rate.animate.set_value(0.85),
            ReplacementTransform(open_thin, open_short),
            ReplacementTransform(n_thin, n_short),
            recolor_meter(air, P_BLUE),
            wash.animate.set_fill(P_BLUE, opacity=0.14),
            ReplacementTransform(tag_throttle, tag_short),
            FadeIn(dial["group"]), FadeIn(dial_read),
            *draft(1.5, 1.2, P_BLUE, P_ORANGE, 4),
            run_time=1.5,
        )
        self.play(minutes.animate.set_value(5.0), *draft(2.4, 1.2, P_BLUE, P_ORANGE, 4), run_time=2.4, rate_func=linear)
        hold_for(self, self.NARRATION, "winter", during=lambda rt: draft(rt, 1.2, P_BLUE, P_ORANGE, 4))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rule"))
        band_ring = SurroundingRectangle(comfort, color=P_GREEN, buff=0.06, stroke_width=3, corner_radius=0.05)
        self.play(FadeIn(rule), Create(band_ring), *draft(1.0, 1.2, P_BLUE, P_ORANGE, 4), run_time=1.0)
        hold_for(self, self.NARRATION, "rule", during=lambda rt: draft(rt, 1.2, P_BLUE, P_ORANGE, 4))
        sash.clear_updaters()
        air["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Purge ventilation versus tilted window
ROOM_A, ROOM_H, ROOM_D = 4.0 * 5.0, 2.5, 5.0
T_STOSS, T_KIPP = 5.0, 60.0
P_RAD, RAD_SHARE = 500.0, 0.5
M_LAIBUNG, C_MASONRY, DT_LAIBUNG = 160.0, 0.84, 5.0


class Beat2_StossOderKipp(Scene):
    """⏱️ Same 50 m³ of fresh air: wide-open window in 5 min versus tilted window in 60 min — and what each costs."""

    NARRATION = [
        ("room",
         "Winter case: a room of 20 square metres and 2.5 metres height holds 50 cubic metres of air; inside 20, outside 0 degrees.",
         "Winterfall: Ein Raum mit 20 m² und 2,5 m Höhe enthält 50 m³ Luft. Innen 20 °C, außen 0 °C."),
        ("open",
         "Two ways to air it: the thermostat turned down and the window wide open — or the window tilted.",
         "Zwei Arten zu lüften: Thermostat zu und Fenster weit auf — oder Fenster gekippt."),
        ("five",
         "After five minutes the wide-open window has exchanged the whole air once; the tilted one only about eight percent.",
         "Nach fünf Minuten ist beim weit offenen Fenster die ganze Luft getauscht, beim gekippten erst etwa 8 %."),
        ("stoss",
         "Purge ventilation only reheats the air: 0.34 times 50 times 20 is 340 watt-hours. The walls stay warm.",
         "Beim Stoßlüften wird nur die Luft nachgeheizt: 0,34 · 50 · 20 = 340 Wh. Die Wände bleiben warm."),
        ("hour",
         "The tilted window needs about an hour for one full air change.",
         "Das gekippte Fenster braucht für einen vollen Luftwechsel etwa eine Stunde."),
        ("thermostat",
         "Cold air falls onto the thermostat below the window; the valve opens and half of 500 watts flows straight out: 250 watt-hours.",
         "Kalte Luft fällt auf das Thermostat unter dem Fenster. Das Ventil öffnet, die Hälfte von 500 W geht direkt hinaus: 250 Wh."),
        ("laibung",
         "Reveal and lintel cool by about 5 kelvin; reheating 160 kilograms of masonry costs about 190 watt-hours.",
         "Laibung und Sturz kühlen um etwa 5 K ab. 160 kg Mauerwerk wieder zu erwärmen kostet rund 190 Wh."),
        ("sum",
         "For the same 50 cubic metres of fresh air: 340 watt-hours purge, about 780 tilted.",
         "Für dieselben 50 m³ Frischluft: 340 Wh beim Stoßlüften, rund 780 Wh bei Kipplüftung."),
        ("rule",
         "So in winter, open the window wide for a short time instead of tilting it.",
         "Darum im Winter: kurz und weit öffnen statt kippen."),
        ("summer",
         "In summer radiator and reveal drop out: at six kelvin the air alone is only about 100 watt-hours, yet a tilted window still exchanges little air. The rule stays: wide and short, in the cool hours.",
         "Im Sommer entfallen Heizkörper und Laibung: nur etwa 100 Wh bei 6 K.\nGekippt tauscht kaum Luft — weit und kurz, in den kühlen Stunden."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION
        vol = ROOM_A * ROOM_H
        e_air = C_LUFT * vol * D_THETA
        e_rad = P_RAD * (T_KIPP / 60.0) * RAD_SHARE
        e_lb = M_LAIBUNG * C_MASONRY * DT_LAIBUNG / 3.6
        e_kipp = e_air + e_rad + e_lb

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Stoßlüften oder Kipplüftung", title)
        din = _din_ref("DIN 1946-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "room"))
        self.play(FadeIn(caption), run_time=0.3)

        #region two rooms
        k, fy = 0.85, -0.45
        rs = _vent_room(-5.9, fy, k, r"\text{Stoßlüften}", P_CYAN)
        rk = _vent_room(1.3, fy, k, r"\text{Kipplüftung}", P_ORANGE)
        note = math_label(rf"\text{{Winter}}\quad V = 4\,\mathrm{{m}} \cdot {de_num(ROOM_D)}\,\mathrm{{m}} \cdot {de_num(ROOM_H, 1)}\,\mathrm{{m}} = "
                          rf"{de_num(vol)}\,\mathrm{{m^{{3}}}} \quad\quad \Delta\theta = {de_num(D_THETA)}\,\mathrm{{K}}",
                          size=LABEL_FONT_SIZE, color=P_WHITE).move_to([-0.3, 2.45, 0])
        t_s, t_k = ValueTracker(0.0), ValueTracker(0.0)
        valve_s, valve_k = ValueTracker(0.6), ValueTracker(0.6)
        cool = ValueTracker(0.0)
        clocks = {}
        for room, tr, col in ((rs, t_s, P_CYAN), (rk, t_k, P_ORANGE)):
            clocks[id(room)] = _clock([room["ox"] + 0.64, room["ceil"] - 0.5, 0], 0.32, tr, color=col)
        reads = VGroup(*[
            math_readout(lambda tr=tr: rf"t = {de_num(tr.get_value())}\,\mathrm{{min}}",
                         np.array([room["ox"] + 0.64, room["ceil"] - 1.12, 0]), size=LABEL_FONT_SIZE, color=col, edge="center")
            for room, tr, col in ((rs, t_s, P_CYAN), (rk, t_k, P_ORANGE))
        ])
        frac_s = lambda: min(1.0, t_s.get_value() / T_STOSS)
        frac_k = lambda: min(1.0, t_k.get_value() / T_KIPP)
        gauges = VGroup(*[_vbar(r["ox"] - 0.5, fy, r["ceil"] - fy, f, color=P_CYAN) for r, f in ((rs, frac_s), (rk, frac_k))])
        pct = VGroup(*[
            math_readout(lambda f=f: rf"{de_num(100 * f())}\,\%", np.array([room["ox"] - 0.5, room["ceil"] + 0.24, 0]),
                         size=LABEL_FONT_SIZE, color=P_CYAN, edge="center")
            for room, f in ((rs, frac_s), (rk, frac_k))
        ])
        dials = VGroup(_valve_dial(rs, valve_s.get_value), _valve_dial(rk, valve_k.get_value))
        flows, state = {}, {"air": [], "heat": [rs, rk]}

        def running(rt):
            streams = [st for key in state["air"] for st in flows[key]]
            heat = [ripples([r["rad"][0].get_top() + UP * 0.03 for r in state["heat"]], r_max=0.3, rings=2,
                            cycles=rt / 1.3, color=P_RED)] if state["heat"] else []
            return ([_streams(streams, rt, speed=0.55, waves=5)] if streams else []) + heat

        self.play(FadeIn(rs["group"]), FadeIn(rk["group"]), FadeIn(rs["head"]), FadeIn(rk["head"]), FadeIn(note),
                  *running(1.4), run_time=1.4)
        self.play(*[FadeIn(c["group"]) for c in clocks.values()], FadeIn(reads), FadeIn(gauges), FadeIn(pct),
                  FadeIn(dials), *running(0.8), run_time=0.8)
        hold_for(self, N, "room", used=2.2, during=running)
        #endregion

        #region open the windows
        caption = swap_caption(self, caption, subtitle_text(N, "open"))
        hinge = rk["sash"].get_bottom()
        state["heat"] = [rk]
        self.play(valve_s.animate.set_value(0.0), rs["rad"].animate.set_stroke(GREY_B), *running(0.8), run_time=0.8)
        self.play(_open_sash(rs["window"]), Rotate(rk["sash"], angle=0.17, about_point=hinge), *running(0.9),
                  run_time=0.9)
        hold_for(self, N, "open", during=running)
        #endregion

        #region first five minutes
        caption = swap_caption(self, caption, subtitle_text(N, "five"))
        unit, bx = 8.0 / 780.0, -4.3
        yr_s, yr_k = -1.2, -1.88
        row_s = _energy_row(bx, yr_s, unit, [(lambda: e_air * frac_s(), P_CYAN)])
        row_k = _energy_row(bx, yr_k, unit, [
            (lambda: e_air * frac_k(), P_CYAN),
            (lambda: P_RAD * RAD_SHARE * t_k.get_value() / 60.0, P_RED),
            (lambda: M_LAIBUNG * C_MASONRY * DT_LAIBUNG * cool.get_value() / 3.6, P_YELLOW),
        ])
        row_labels = VGroup(
            Text("Stoßlüften", font_size=LABEL_FONT_SIZE, color=P_CYAN),
            Text("Kipplüftung", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
        )
        for lbl, y in zip(row_labels, (yr_s, yr_k)):
            lbl.move_to([bx - 0.18 - lbl.width / 2, y, 0])

        def total_k():
            return e_air * frac_k() + P_RAD * RAD_SHARE * t_k.get_value() / 60.0 + \
                M_LAIBUNG * C_MASONRY * DT_LAIBUNG * cool.get_value() / 3.6

        tot_s = math_readout(lambda: rf"{de_num(e_air * frac_s())}\,\mathrm{{Wh}}",
                             lambda: np.array([bx + e_air * frac_s() * unit + 0.15, yr_s - 0.08, 0]),
                             size=LABEL_FONT_SIZE, color=P_CYAN)
        tot_k = math_readout(lambda: rf"{de_num(total_k())}\,\mathrm{{Wh}}",
                             lambda: np.array([bx + total_k() * unit + 0.15, yr_k - 0.08, 0]),
                             size=LABEL_FONT_SIZE, color=P_ORANGE)
        self.play(FadeIn(row_labels), *running(0.4), run_time=0.4)
        self.add(row_s, row_k, tot_s, tot_k)

        def ext(room, x_off, y):
            return np.array([room["x_out"] + x_off, y, 0])

        def inn(room, x_off, y):
            return np.array([room["x_in"] - x_off, y, 0])

        mid_s = (rs["x_in"] + rs["x_out"]) / 2
        stoss_paths_out = [smooth_path([inn(rs, 1.4, y - 0.15), np.array([mid_s, y, 0]), ext(rs, 0.9, y + 0.3)])
                           for y in (0.98, 1.12, 1.26)]
        stoss_paths_in = [smooth_path([ext(rs, 0.9, y - 0.1), np.array([mid_s, y, 0]), inn(rs, 0.6, y - 0.15),
                                       inn(rs, 1.5, fy + 0.75), inn(rs, 2.4, fy + 0.15)]) for y in (0.52, 0.66, 0.8)]
        mid_k = (rk["x_in"] + rk["x_out"]) / 2
        gap_y = rk["lintel"] - 0.06
        kipp_out = [smooth_path([inn(rk, 0.9, gap_y - 0.25), np.array([mid_k, gap_y, 0]), ext(rk, 0.8, gap_y + 0.3)])]
        kipp_in = [smooth_path([ext(rk, 0.7, gap_y - 0.1), np.array([mid_k + 0.03, gap_y - 0.02, 0]),
                                inn(rk, 0.12, gap_y - 0.25), inn(rk, 0.1, rk["valve"].get_center()[1] + 0.1)])]
        heat_out = [smooth_path([np.array([rk["rad"].get_center()[0], rk["rad"][0].get_top()[1] + 0.05, 0]),
                                 inn(rk, 0.35, rk["sill"] + 0.45), np.array([mid_k, gap_y + 0.01, 0]),
                                 ext(rk, 0.8, gap_y + 0.38)])]
        flows.update({"s": [(stoss_paths_out, P_ORANGE), (stoss_paths_in, P_BLUE)],
                      "k": [(kipp_out, P_ORANGE), (kipp_in, P_BLUE)], "h": [(heat_out, P_RED)]})
        state["air"] = ["s", "k"]
        self.play(t_s.animate.set_value(T_STOSS), t_k.animate.set_value(T_STOSS), valve_k.animate.set_value(0.75),
                  cool.animate.set_value(T_STOSS / T_KIPP), *running(4.0), run_time=4.0, rate_func=linear)
        hold_for(self, N, "five", during=running)
        #endregion

        #region purge done
        caption = swap_caption(self, caption, subtitle_text(N, "stoss"))
        air_s_lbl = math_label(rf"\text{{Luft}}\; {de_num(e_air)}\,\mathrm{{Wh}}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        air_s_lbl.move_to([bx + e_air * unit / 2, yr_s, 0])
        state["air"] = ["k"]
        self.play(_open_sash(rs["window"], run_time=1.0, close=True), valve_s.animate.set_value(0.6),
                  rs["rad"].animate.set_stroke(P_RED), FadeIn(air_s_lbl), *running(1.0), run_time=1.0)
        state["heat"] = [rs, rk]
        hold_for(self, N, "stoss", during=running)
        #endregion

        #region tilted window: the full hour
        caption = swap_caption(self, caption, subtitle_text(N, "hour"))

        def laibung_color():
            return interpolate_color(ManimColor(P_ORANGE), ManimColor(P_BLUE), float(cool.get_value()))

        rk["sturz"].add_updater(lambda m: m.set_fill(laibung_color(), opacity=0.7))
        rk["laibung"].add_updater(lambda m: m.set_fill(laibung_color(), opacity=0.3))
        state["air"] = ["k", "h"]
        self.play(t_k.animate.set_value(T_KIPP), valve_k.animate.set_value(1.0), cool.animate.set_value(1.0),
                  *running(6.0), run_time=6.0, rate_func=linear)
        hold_for(self, N, "hour", during=running)
        #endregion

        #region thermostat and reveal
        caption = swap_caption(self, caption, subtitle_text(N, "thermostat"))
        rad_lbl = math_label(rf"\dot{{Q}} = {de_num(P_RAD)}\,\mathrm{{W}} \cdot {de_num(100 * RAD_SHARE)}\,\%",
                             np.array([rk["ox"] + 0.15, fy + 0.2, 0]), size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        seg_rad = math_label(rf"\text{{Heizkörper}}\; {de_num(e_rad)}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_rad.move_to([bx + (e_air + e_rad / 2) * unit, yr_k, 0])
        seg_air = math_label(rf"\text{{Luft}}\; {de_num(e_air)}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_air.move_to([bx + e_air * unit / 2, yr_k, 0])
        self.play(Indicate(rk["valve"], color=P_YELLOW, scale_factor=1.8), FadeIn(rad_lbl), FadeIn(seg_air),
                  FadeIn(seg_rad), *running(1.0), run_time=1.0)
        hold_for(self, N, "thermostat", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "laibung"))
        lb_lbl = math_label(rf"\Delta T = {de_num(DT_LAIBUNG)}\,\mathrm{{K}}", np.array([rk["x_out"] + 0.2, rk["sill"] + 0.05, 0]),
                            size=LABEL_FONT_SIZE, color=P_BLUE, edge="left")
        seg_lb = math_label(rf"\text{{Laibung}}\; {de_num(round(e_lb, -1))}", size=LABEL_FONT_SIZE, color=P_DEEP_DARK)
        seg_lb.move_to([bx + (e_air + e_rad + e_lb / 2) * unit, yr_k, 0])
        self.play(Indicate(VGroup(rk["sturz"], rk["laibung"]), color=P_BLUE, scale_factor=1.05), FadeIn(lb_lbl),
                  FadeIn(seg_lb), *running(1.0), run_time=1.0)
        hold_for(self, N, "laibung", during=running)
        #endregion

        #region sum and rule
        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        tot_k.clear_updaters()
        tot_k_final = math_label(rf"\approx {de_num(round(e_kipp, -1))}\,\mathrm{{Wh}}", size=LABEL_FONT_SIZE, color=P_ORANGE)
        tot_k_final.move_to(tot_k.get_left() + RIGHT * tot_k_final.width / 2)
        ratio = math_label(rf"\approx {de_num(e_kipp / e_air, 1)} \times", size=BODY_FONT_SIZE, color=P_ORANGE)
        ratio.next_to(tot_k_final, RIGHT, buff=0.3)
        self.play(ReplacementTransform(tot_k, tot_k_final), *running(0.6), run_time=0.6)
        self.play(TransformFromCopy(row_s, ratio), *running(0.8), run_time=0.8)
        hold_for(self, N, "sum", during=running)

        caption = swap_caption(self, caption, subtitle_text(N, "rule"))
        rule = Text("kurz und weit öffnen statt kippen", font_size=BODY_FONT_SIZE, color=P_GREEN)
        rule_box = SurroundingRectangle(rule, color=P_GREEN, buff=0.14, corner_radius=0.1, stroke_width=1.8)
        VGroup(rule, rule_box).move_to([3.3, yr_s, 0])
        self.play(FadeOut(rad_lbl), FadeIn(rule), Create(rule_box), *running(0.8), run_time=0.8)
        hold_for(self, N, "rule", during=running)
        #endregion

        #region summer side
        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        e_sum = C_LUFT * vol * (T_OUT_DAY - T_IN_SUMMER)
        winter_rows = VGroup(row_s, row_k, tot_s, tot_k_final, ratio, air_s_lbl, seg_air, seg_rad, seg_lb,
                             row_labels, lb_lbl)
        sun = sun_glyph(np.array([bx - 0.9, yr_s - 0.3, 0.0])).scale(0.32)
        sum_bar = Rectangle(width=e_sum * unit, height=0.5, stroke_width=1.2, color=P_YELLOW, fill_color=P_YELLOW,
                            fill_opacity=0.72).move_to([bx + e_sum * unit / 2, yr_s - 0.3, 0])
        sum_head = math_label(r"\text{Sommer}\quad \Delta\theta = 6\,\mathrm{K}", size=LABEL_FONT_SIZE, color=P_YELLOW)
        sum_head.move_to([bx + sum_head.width / 2, yr_s + 0.28, 0])
        sum_val = math_label(rf"{de_num(C_LUFT, 2)} \cdot {de_num(vol)} \cdot 6 \approx {de_num(round(e_sum, -1))}\,\mathrm{{Wh}}",
                             size=LABEL_FONT_SIZE, color=P_YELLOW)
        sum_val.next_to(sum_bar, RIGHT, buff=0.2)
        self.play(FadeOut(winter_rows), *running(0.6), run_time=0.6)
        self.play(FadeIn(sun), GrowFromEdge(sum_bar, LEFT), FadeIn(sum_head), FadeIn(sum_val), *running(1.2), run_time=1.2)
        hold_for(self, N, "summer", during=running)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Cross ventilation: pressure difference and both openings
class Beat3_Querlueftung(Scene):
    NARRATION = [
        ("intro",
         "For a stronger flow that is still entirely natural, open two opposite facades.",
         "Für stärkeren, weiterhin natürlichen Luftstrom:\nzwei gegenüberliegende Fassaden öffnen."),
        ("pressure",
         "Wind presses on the windward face and pulls on the leeward face. That pressure difference, not a fan, is what drives air through the room.",
         "Wind drückt auf der Luvseite und saugt auf der Leeseite —\ndiese Druckdifferenz treibt die Luft, kein Ventilator."),
        ("formula",
         "How much air gets through is set by an effective area that combines both openings.",
         "Wie viel Luft durchkommt, bestimmt eine wirksame Fläche aus beiden Öffnungen."),
        ("a1",
         "A one is the inlet on the windward facade.",
         "A eins ist die Zuluftöffnung auf der Luvseite."),
        ("a2",
         "A two is the outlet on the leeward facade.",
         "A zwei ist die Abluftöffnung auf der Leeseite."),
        ("limit",
         "Because the two act in series, the smaller one dominates. Shrink the outlet and the whole cross flow collapses, no matter how wide the inlet stays.",
         "Weil beide in Reihe wirken, bestimmt die kleinere Öffnung:\nverkleinert man den Auslass, bricht der Durchzug ein."),
        ("summer",
         "So size both sides together. In summer this flush dumps a lot of heat on windy nights, but it does not replace a design cooling plant by itself.",
         "Also beide Seiten gemeinsam dimensionieren. Im Sommer führt die\nSpülung nachts viel Wärme ab, ersetzt aber keine Kälteanlage."),
        ("winter",
         "In winter the same wind pressure pulls cold air through the room. The cross flow costs heat quickly, so use it briefly and close the openings afterwards.",
         "Im Winter zieht derselbe Winddruck kalte Luft durch den Raum —\nDurchzug kostet schnell Wärme: kurz lüften, danach schließen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Querlüftung: beide Öffnungen zählen", title)
        din = _din_ref("DIN EN 16798-7")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([0.0, 0.55, 0.0])
        shell = _room(room_c, 6.4, 2.2)
        room, floor = shell[0], shell[1]
        left_win = _window(np.array([room.get_left()[0], 0.55, 0.0]), height=1.15)
        right_win = _window(np.array([room.get_right()[0], 0.55, 0.0]), height=1.15)

        a1_lbl = math_label(r"A_{1}", size=BODY_FONT_SIZE, color=P_GREEN).next_to(left_win, LEFT, buff=0.18)
        a2_lbl = math_label(r"A_{2}", size=BODY_FONT_SIZE, color=P_ORANGE).next_to(right_win, RIGHT, buff=0.18)

        gusts = VGroup(*[
            Arrow(
                np.array([-5.75, y, 0.0]), np.array([-4.45, y, 0.0]),
                buff=0, color=P_CYAN, stroke_width=4,
                max_tip_length_to_length_ratio=0.16,
            )
            for y in (0.55, 1.05, 1.55)
        ])
        wind_lbl = Text("Wind", font_size=BODY_FONT_SIZE, color=P_CYAN).next_to(gusts, UP, buff=0.16)

        luv = _badge("Luv", "+ Überdruck", P_CYAN)
        luv.move_to(np.array([-5.05, -0.05, 0.0]))
        lee = _badge("Lee", "− Unterdruck", P_ORANGE)
        lee.move_to(np.array([5.05, -0.05, 0.0]))

        aeff_wide = math_label(r"A_{eff} \approx 0{,}35\,\mathrm{m^{2}}", size=BODY_FONT_SIZE, color=P_CYAN)
        aeff_wide.move_to(np.array([0.0, 2.12, 0.0]))
        aeff_thin = math_label(r"A_{eff} \approx 0{,}10\,\mathrm{m^{2}}", size=BODY_FONT_SIZE, color=P_RED).move_to(aeff_wide)
        tip = Text("Sommer: Spülung vor der Kältemaschine", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        tip.move_to(aeff_wide)
        tip_winter = Text("Winter: kurz lüften, danach schließen", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to(aeff_wide)
        wash = Rectangle(
            width=6.3, height=2.1, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.0,
        ).move_to(room_c)

        scaffold = VGroup(room, floor, left_win, right_win, a1_lbl, a2_lbl,
                          gusts, wind_lbl, luv, lee, aeff_wide)
        fit_band(scaffold, bottom=SAFE_BOTTOM_FORMULA)

        left_x, right_x = room.get_left()[0], room.get_right()[0]

        def _cross(rows):
            return VGroup(*[
                smooth_path([
                    np.array([left_x, y0, 0.0]),
                    np.array([-1.5, y1, 0.0]),
                    np.array([1.5, y2, 0.0]),
                    np.array([2.5, y3, 0.0]),
                    np.array([right_x, y4, 0.0]),
                ])
                for y0, y1, y2, y3, y4 in rows
            ])

        cross = _cross([(0.92, 0.92, 0.92, 0.92, 0.92), (0.56, 0.56, 0.56, 0.56, 0.56), (0.20, 0.20, 0.20, 0.20, 0.20)])
        cross_thin = _cross([(0.92, 0.92, 0.88, 0.76, 0.64), (0.56, 0.56, 0.56, 0.56, 0.56), (0.20, 0.20, 0.24, 0.36, 0.46)])

        eq, eq_box, items = math_panel([
            ("aeff", r"A_{eff}", P_CYAN), (None, "=", P_WHITE),
            ("root",
             rf"\frac{{1}}{{\sqrt{{\textcolor{{{P_GREEN}}}{{\frac{{1}}{{A_{{1}}^{{2}}}}}} + "
             rf"\textcolor{{{P_ORANGE}}}{{\frac{{1}}{{A_{{2}}^{{2}}}}}}}}}}",
             P_WHITE),
            (None, r"\;[\mathrm{m^{2}}]", P_TEAL),
        ], color=P_CYAN)
        _inner = items["root"][0][2][0][1]
        items["a1"], items["a2"] = _inner[0], _inner[2]

        self.play(Create(room), Create(floor), run_time=1.3)
        self.play(FadeIn(left_win), FadeIn(right_win), FadeIn(a1_lbl), FadeIn(a2_lbl), run_time=0.9)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.9)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "pressure"))
        self.play(
            LaggedStart(*[GrowArrow(a) for a in gusts], lag_ratio=0.18),
            FadeIn(wind_lbl), FadeIn(luv), FadeIn(lee),
            run_time=1.5,
        )
        def through(rt, paths=cross, color=P_GREEN, speed=0.85, waves=4, end=None):
            return [_streams([(paths, color, end)], rt, speed=speed, waves=waves)]

        guides = flow_guides(cross, P_GREEN)
        self.play(Create(guides), run_time=0.7)
        self.play(*through(2.8), run_time=2.8)
        hold_for(self, self.NARRATION, "pressure", during=through)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        rest = VGroup(*[
            m for m in eq.submobjects
            if m is not items["a1"] and m is not items["a2"]
        ])
        self.play(Create(eq_box), FadeIn(rest), FadeIn(aeff_wide), *through(0.9), run_time=0.9)
        self.play(
            ReplacementTransform(a1_lbl.copy(), items["a1"]),
            ReplacementTransform(a2_lbl.copy(), items["a2"]),
            *through(1.2),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "formula", during=through)

        for key, win, color in (("a1", left_win, P_GREEN), ("a2", right_win, P_ORANGE)):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), Indicate(win, color=color, scale_factor=1.2), *through(0.55), run_time=0.55)
            hold_for(self, self.NARRATION, key, during=through)
            self.play(FadeOut(ring), *through(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "limit"))
        a2_ring = highlight_param(items, "a2", color=P_RED)
        guides_thin = flow_guides(cross_thin, P_RED)
        self.play(
            right_win.animate.stretch_to_fit_height(0.30),
            ReplacementTransform(aeff_wide, aeff_thin),
            ReplacementTransform(guides, guides_thin),
            Create(a2_ring),
            run_time=1.3,
        )
        self.play(*through(2.6, cross_thin, P_RED, 0.3, 2), run_time=2.6)
        guides = flow_guides(cross, P_GREEN)
        self.play(
            right_win.animate.stretch_to_fit_height(1.15),
            ReplacementTransform(aeff_thin, aeff_wide),
            ReplacementTransform(guides_thin, guides),
            FadeOut(a2_ring),
            run_time=1.1,
        )
        self.play(*through(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "limit", during=through)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summer"))
        self.play(ReplacementTransform(aeff_wide, tip), *through(0.9, color=P_BLUE, end=P_ORANGE), run_time=0.9)
        hold_for(self, self.NARRATION, "summer", during=lambda rt: through(rt, color=P_BLUE, end=P_ORANGE))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        self.add(wash)
        wash.set_z_index(-1)
        self.play(ReplacementTransform(tip, tip_winter), wash.animate.set_fill(P_BLUE, opacity=0.14),
                  *through(1.0, speed=1.2, color=P_BLUE, end=P_ORANGE), run_time=1.0)
        self.play(*through(2.0, speed=1.2, color=P_BLUE, end=P_ORANGE), run_time=2.0)
        hold_for(self, self.NARRATION, "winter", during=lambda rt: through(rt, speed=1.2, color=P_BLUE, end=P_ORANGE))
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Stack effect: height makes the pressure
class Beat4_Auftrieb(Scene):
    NARRATION = [
        ("intro",
         "When the wind drops, buoyancy still moves air. Warm room air is lighter, rises, and leaves high, while cool air is drawn in low.",
         "Fällt der Wind aus, bleibt der Auftrieb: warme Raumluft\nist leichter und entweicht oben, kühle Luft strömt unten nach."),
        ("shaft",
         "A stairwell or a dedicated shaft carries that warm air up to a high outlet, so the flow keeps running even on a still day.",
         "Treppenhaus oder Schacht führt die warme Luft zu einem\nhohen Auslass — auch an windstillen Tagen."),
        ("formula",
         "The driving pressure is the height difference, times gravity, times the density difference between outdoor and indoor air.",
         "Der treibende Druck ist Höhendifferenz mal Erdbeschleunigung\nmal Dichteunterschied zwischen außen und innen."),
        ("h",
         "h is the vertical distance between the low inlet and the high outlet.",
         "h ist der senkrechte Abstand zwischen tiefem Einlass und hohem Auslass."),
        ("rho",
         "The density difference comes straight out of the temperature difference: on a summer night 18 outside against 26 inside. The warmer the room air, the lighter it is, and the harder it is pushed up.",
         "Der Dichteunterschied folgt aus der Temperaturdifferenz: 18 °C außen,\n26 °C innen — je wärmer die Raumluft, desto stärker der Auftrieb."),
        ("taller",
         "Raise the outlet and the driving pressure grows with the height. Here the shaft lifts it from 2.5 to 4.4 metres, and the flow speeds up with it.",
         "Höherer Auslass, größere Höhe — der Druck wächst mit.\nDer Schacht hebt h von 2,5 auf 4,4 m, der Strom wird schneller."),
        ("winter",
         "In winter the temperature difference is 20 instead of 8 kelvin. The same shaft now builds almost three times the pressure: the stack effect is strongest when it is cold.",
         "Im Winter beträgt die Temperaturdifferenz 20 statt 8 K — derselbe Schacht\nerzeugt fast dreimal so viel Druck: im Kalten wirkt der Auftrieb am stärksten."),
        ("comfort",
         "That is why inlets sit low, near the people, and outlets sit high at the roof: comfort without a fan.",
         "Deshalb sitzen Zulüfte tief bei den Menschen und die\nAbluft hoch am Dach — Komfort ohne Ventilator."),
    ]

    SHAFT_X = -1.60
    SHAFT_HW = 0.42
    TOP_LOW = 0.75
    TOP_HIGH = 2.05

    def _stack_paths(self, inlet, room_top, shaft_top):
        """〰️ Inlet → occupied zone → up the shaft → out at the top."""
        return VGroup(*[
            smooth_path([
                inlet + RIGHT * 0.06 + UP * (0.06 * i),
                np.array([-3.20, -0.62 + 0.10 * i, 0.0]),
                np.array([self.SHAFT_X, room_top + 0.10, 0.0]),
                np.array([self.SHAFT_X, shaft_top - 0.08, 0.0]),
            ])
            for i in range(3)
        ])

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Auftrieb: Höhe erzeugt Druck", title)
        din = _din_ref("DIN EN 16798-7")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([-2.90, -0.45, 0.0])
        shell = _room(room_c, 4.2, 1.7)
        room, floor = shell[0], shell[1]
        room_top = room.get_top()[1]
        inlet = _window(np.array([room.get_left()[0], -0.95, 0.0]), height=0.60)
        inlet_lbl = Text("Zuluft", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        inlet_lbl.next_to(inlet, LEFT, buff=0.34)
        person = _stand(room_c, 1.7, -1.75)

        shaft_walls = VGroup(*[
            Line(
                np.array([self.SHAFT_X + s * self.SHAFT_HW, room_top, 0.0]),
                np.array([self.SHAFT_X + s * self.SHAFT_HW, self.TOP_LOW, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            )
            for s in (-1, 1)
        ])
        outlet = Line(
            np.array([self.SHAFT_X - self.SHAFT_HW, self.TOP_LOW, 0.0]),
            np.array([self.SHAFT_X + self.SHAFT_HW, self.TOP_LOW, 0.0]),
            color=P_ORANGE, stroke_width=3.0,
        )
        outlet_lbl = Text("Abluft", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        outlet_lbl.next_to(outlet, LEFT, buff=0.25)

        def _legend(season, season_color, t_out, t_in):
            rows = VGroup(
                Text(season, font_size=LABEL_FONT_SIZE, color=season_color),
                VGroup(
                    math_label(r"\rho_{a}", size=BODY_FONT_SIZE, color=P_BLUE),
                    math_label(rf"{de_num(t_out)}\,\mathrm{{°C}}", size=LABEL_FONT_SIZE, color=P_WHITE),
                    Text("außen · kühl · schwer", font_size=LABEL_FONT_SIZE, color=P_WHITE),
                ).arrange(RIGHT, buff=0.16),
                VGroup(
                    math_label(r"\rho_{i}", size=BODY_FONT_SIZE, color=P_ORANGE),
                    math_label(rf"{de_num(t_in)}\,\mathrm{{°C}}", size=LABEL_FONT_SIZE, color=P_WHITE),
                    Text("innen · warm · leicht", font_size=LABEL_FONT_SIZE, color=P_WHITE),
                ).arrange(RIGHT, buff=0.16),
            ).arrange(DOWN, aligned_edge=LEFT, buff=0.12)
            box = SurroundingRectangle(rows, color=P_TEAL, corner_radius=0.1, buff=0.14, stroke_width=1.8)
            return VGroup(box, rows).move_to(np.array([1.85, 1.30, 0.0]))

        legend = _legend("Sommernacht", P_BLUE, T_OUT_NIGHT, T_IN_SUMMER)
        legend_winter = _legend("Winter", P_CYAN, T_OUT_WINTER, T_IN_WINTER)
        legend_rows = legend[1]

        scaffold = VGroup(room, floor, inlet, inlet_lbl, person,
                          shaft_walls, outlet, outlet_lbl, legend)
        fit_band(scaffold, bottom=SAFE_BOTTOM_FORMULA)

        warm_spots = [person.get_top() + UP * 0.05, np.array([-2.30, floor.get_top()[1] + 0.02, 0.0])]
        stack = {"paths": None, "speed": 0.65}

        def warmth(rt, r_max=0.55):
            return [ripples(warm_spots, r_max=r_max, color=P_ORANGE, cycles=rt / 1.4)]

        def rising(rt):
            return [_streams([(stack["paths"], P_CYAN, P_ORANGE)], rt, speed=stack["speed"]), *warmth(rt, 0.4)]

        dp = meter("Δp", length=2.2, thickness=0.52, color=P_YELLOW)
        dp["group"].move_to(np.array([4.65, 0.35, 0.0]))
        press = ValueTracker(0.0)
        bind_meter(dp, press)
        dp_max = 5.0
        dp_low, dp_high = _stack_dp(2.5, T_OUT_NIGHT, T_IN_SUMMER), _stack_dp(4.4, T_OUT_NIGHT, T_IN_SUMMER)
        dp_winter = _stack_dp(4.4, T_OUT_WINTER, T_IN_WINTER)
        dp_read = math_readout(
            lambda: rf"{de_num(press.get_value() * dp_max, 1)}\,\mathrm{{Pa}}",
            dp["track"].get_bottom() + DOWN * 0.34, size=BODY_FONT_SIZE, color=P_YELLOW, edge="center",
        )

        eq, eq_box, items = math_panel([
            ("dp", r"Δp", P_YELLOW), (None, "=", P_WHITE),
            ("h", "h", P_CYAN), (None, r"\cdot", P_WHITE),
            (None, "g", P_WHITE), (None, r"\cdot", P_WHITE),
            ("rho", r"(\rho_{a} - \rho_{i})", P_BLUE),
            (None, r"\;[\mathrm{Pa}]", P_TEAL),
        ], color=P_YELLOW)

        self.play(Create(room), Create(floor), FadeIn(inlet), FadeIn(inlet_lbl), FadeIn(person), run_time=1.4)
        self.play(*warmth(1.1), run_time=1.1)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 1.1, during=warmth)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "shaft"))
        self.play(Create(shaft_walls), Create(outlet), FadeIn(outlet_lbl), *warmth(1.2), run_time=1.2)
        self.play(FadeIn(legend), *warmth(0.8), run_time=0.8)
        stack["paths"] = stack_low = self._stack_paths(inlet.get_center(), room_top, self.TOP_LOW)
        guides_low = flow_guides(stack_low, P_ORANGE)
        self.play(Create(guides_low), *warmth(0.6), run_time=0.6)
        self.play(*rising(2.6), run_time=2.6)
        hold_for(self, self.NARRATION, "shaft", during=rising)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq), Create(eq_box), *rising(1.0), run_time=1.0)
        self.play(FadeIn(dp["group"]), FadeIn(dp_read), press.animate.set_value(dp_low / dp_max), *rising(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "formula", during=rising)

        h_dim = dim_arrow(
            np.array([-0.30, inlet.get_center()[1], 0.0]),
            np.array([-0.30, self.TOP_LOW, 0.0]),
            color=P_CYAN,
        )
        h_tok = math_label(r"h \approx 2{,}5\,\mathrm{m}", size=LABEL_FONT_SIZE, color=P_CYAN)
        h_tok.next_to(h_dim, RIGHT, buff=0.14)
        h_tok_tall = math_label(r"h \approx 4{,}4\,\mathrm{m}", size=LABEL_FONT_SIZE, color=P_CYAN)
        ring_h = highlight_param(items, "h", color=P_CYAN)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "h"))
        self.play(Create(h_dim), FadeIn(h_tok), Create(ring_h), *rising(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "h", during=rising)
        self.play(FadeOut(ring_h), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rho"))
        ring_rho = highlight_param(items, "rho", color=P_BLUE)
        self.play(
            Create(ring_rho),
            Indicate(legend_rows[1], color=P_BLUE),
            Indicate(legend_rows[2], color=P_ORANGE),
            *rising(1.0),
            run_time=1.0,
        )
        hold_for(self, self.NARRATION, "rho", during=rising)
        self.play(FadeOut(ring_rho), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "taller"))
        rise = self.TOP_HIGH - self.TOP_LOW
        h_tok_tall.move_to(h_tok.get_center() + UP * rise / 2)
        tall_dim = dim_arrow(
            np.array([-0.30, inlet.get_center()[1], 0.0]),
            np.array([-0.30, self.TOP_HIGH, 0.0]),
            color=P_CYAN,
        )
        self.play(
            *[
                wall.animate.put_start_and_end_on(
                    wall.get_start(), wall.get_end() + UP * rise,
                )
                for wall in shaft_walls
            ],
            outlet.animate.shift(UP * rise),
            outlet_lbl.animate.shift(UP * rise),
            Transform(h_dim, tall_dim),
            ReplacementTransform(h_tok, h_tok_tall),
            FadeOut(guides_low),
            press.animate.set_value(dp_high / dp_max),
            run_time=1.6,
        )
        stack["paths"] = stack_high = self._stack_paths(inlet.get_center(), room_top, self.TOP_HIGH)
        stack["speed"] = 1.1
        self.play(Create(flow_guides(stack_high, P_ORANGE)), run_time=0.5)
        self.play(*rising(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "taller", during=rising)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        stack["speed"] = 1.8
        self.play(
            ReplacementTransform(legend, legend_winter),
            press.animate.set_value(dp_winter / dp_max),
            recolor_meter(dp, P_CYAN),
            *rising(1.6),
            run_time=1.6,
        )
        self.play(*rising(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "winter", during=rising)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "comfort"))
        low_ring = VGroup(
            SurroundingRectangle(inlet, color=P_CYAN, buff=0.12, corner_radius=0.08, stroke_width=2.5),
            SurroundingRectangle(person, color=P_CYAN, buff=0.12, corner_radius=0.08, stroke_width=2.5),
        )
        high_ring = SurroundingRectangle(outlet, color=P_ORANGE, buff=0.14,
                                         corner_radius=0.08, stroke_width=2.5)
        self.play(Create(low_ring), Create(high_ring), *rising(1.0), run_time=1.0)
        self.play(Indicate(person, color=P_ORANGE), *rising(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "comfort", during=rising)
        dp["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Night purge over a two-day summer profile
class Beat5_Nachtlueftung(Scene):
    NARRATION = [
        ("intro",
         "Summer comfort lives on night ventilation once the envelope is shaded. Here are two summer days, with the outdoor temperature in yellow.",
         "Sommerkomfort lebt von der Nachtlüftung, sobald die Hülle\nverschattet ist — hier zwei Sommertage, Außentemperatur in Gelb."),
        ("day",
         "By day the outdoor air is hotter than the room, so the windows stay shut and the structure soaks up the heat it cannot reject.",
         "Tagsüber ist es draußen heißer als drinnen — die Fenster\nbleiben zu, und die Bauteile nehmen die Wärme auf."),
        ("without",
         "Without night ventilation that stored heat is never released. Each day starts warmer than the last, and the room drifts out of the comfort band.",
         "Ohne Nachtlüftung wird die gespeicherte Wärme nie\nabgegeben — der Raum verlässt das Komfortband."),
        ("night",
         "After sunset the outdoor air falls below the room temperature. That is the window of opportunity: open wide and let cool air sweep the thermal mass.",
         "Nach Sonnenuntergang fällt die Außenluft unter die\nRaumtemperatur — jetzt weit öffnen und kühl spülen."),
        ("result",
         "The mass is discharged overnight, so the second day peaks about four kelvin lower, back inside the comfort band, with no chiller at all.",
         "Über Nacht entlädt sich die Masse — der zweite Tag liegt\nrund vier Kelvin tiefer, ganz ohne Kältemaschine."),
        ("schedule",
         "So in summer night ventilation is a schedule, not a setting: wide open at night, closed and shaded by day.",
         "Im Sommer ist Nachtlüftung ein Zeitplan, keine Einstellung:\nnachts weit offen, tagsüber zu und verschattet."),
        ("winter",
         "In winter the schedule reverses. The mass holds the warmth of the day, so the windows stay shut at night and the room is aired only briefly by day.",
         "Im Winter kehrt sich der Zeitplan um: die Masse hält die Tageswärme —\nnachts bleiben die Fenster zu, tagsüber wird kurz gelüftet."),
    ]

    @staticmethod
    def _outdoor(t):
        return 27.5 + 6.0 * np.sin(2 * np.pi * (t - 8.0) / 24.0)

    @staticmethod
    def _indoor_sealed(t):
        return 26.8 + 1.3 * np.sin(2 * np.pi * (t - 12.0) / 24.0) + 0.035 * t

    @staticmethod
    def _indoor_purged(t):
        return 24.6 + 1.2 * np.sin(2 * np.pi * (t - 12.0) / 24.0)

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Nachtlüftung: den Speicher entladen", title)
        din = _din_ref("DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        axes = Axes(
            x_range=[0, 48, 12], y_range=[18, 34, 4],
            x_length=7.2, y_length=2.8,
            axis_config={"color": P_WHITE, "stroke_width": 2.2, "include_tip": False},
            tips=False,
        )
        axes.move_to(np.array([-1.35, 0.60, 0.0]))

        x_ticks = VGroup(*[
            Text(s, font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(v, 18), DOWN, buff=0.14)
            for v, s in ((0, "0"), (12, "12"), (24, "24"), (36, "36"), (48, "48"))
        ])
        x_unit = Text("Stunden", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        x_unit.next_to(axes.c2p(48, 18), DOWN, buff=0.14).shift(RIGHT * 0.78)
        y_ticks = VGroup(*[
            Text(s, font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(0, v), LEFT, buff=0.14)
            for v, s in ((22, "22"), (26, "26"), (30, "30"), (34, "34"))
        ])
        y_unit = Text("°C", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        y_unit.next_to(axes.c2p(0, 34), UP, buff=0.14)

        def _zone(t0, t1, v0, v1, color, opacity):
            p0, p1 = axes.c2p(t0, v0), axes.c2p(t1, v1)
            return Rectangle(
                width=abs(p1[0] - p0[0]), height=abs(p1[1] - p0[1]),
                stroke_width=0, fill_color=color, fill_opacity=opacity,
            ).move_to((p0 + p1) / 2)

        comfort = _zone(0, 48, 22, 26, P_GREEN, 0.12)
        comfort_lbl = Text("Komfortband", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        comfort_lbl.next_to(axes.c2p(0, 22), LEFT, buff=0.14).shift(DOWN * 0.30)
        nights = VGroup(_zone(20, 30, 18, 34, P_BLUE, 0.16), _zone(44, 48, 18, 34, P_BLUE, 0.16))
        night_lbl = Text("Nacht", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        night_lbl.next_to(nights[0], UP, buff=0.10)
        warn = Text("verlässt das Komfortband", font_size=LABEL_FONT_SIZE, color=P_RED)
        warn.next_to(axes, UP, buff=0.22).set_x(axes.get_center()[0])

        outdoor = axes.plot(self._outdoor, x_range=[0, 48], color=P_YELLOW, stroke_width=3)
        sealed = axes.plot(self._indoor_sealed, x_range=[0, 48], color=P_RED, stroke_width=3)
        purged = axes.plot(self._indoor_purged, x_range=[0, 48], color=P_CYAN, stroke_width=3)

        def _key(color, text):
            swatch = Line(LEFT * 0.22, RIGHT * 0.22, color=color, stroke_width=4)
            return VGroup(swatch, Text(text, font_size=LABEL_FONT_SIZE, color=P_WHITE)).arrange(RIGHT, buff=0.16)

        keys = VGroup(
            _key(P_YELLOW, "Außenluft"),
            _key(P_RED, "innen — ohne"),
            _key(P_CYAN, "innen — mit"),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.18)
        keys.move_to(np.array([4.85, -0.55, 0.0]))

        room_c = np.array([4.85, 1.55, 0.0])
        icon = Rectangle(width=2.1, height=1.25, stroke_width=0).move_to(room_c)
        icon_room = _room(room_c, 2.1, 1.25, t=0.12)
        mass = Rectangle(
            width=1.85, height=0.42, stroke_width=0,
            fill_color=P_RED, fill_opacity=0.0,
        ).move_to(icon.get_bottom() + UP * 0.24)
        mass_lbl = Text("Speichermasse", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        mass_lbl.next_to(icon, UP, buff=0.12)
        mass_hot = Text("27 °C", font_size=BODY_FONT_SIZE, color=P_RED).move_to(mass)
        mass_cool = Text("22 °C", font_size=BODY_FONT_SIZE, color=P_BLUE).move_to(mass)
        icon_win = VGroup(
            _window(icon.get_left() + UP * 0.28, height=0.45, width=0.16),
            _window(icon.get_right() + UP * 0.28, height=0.45, width=0.16),
        )
        sky_pos = icon.get_corner(UL) + LEFT * 0.42 + UP * 0.44
        sun = sun_glyph(sky_pos).scale(0.32)
        moon = VGroup(
            moon_glyph(sky_pos).scale(0.85),
            *[
                Dot(sky_pos + RIGHT * dx + UP * dy, radius=0.03, color=P_WHITE, fill_opacity=0.85)
                for dx, dy in ((0.52, 0.16), (-0.34, 0.42), (0.28, -0.40))
            ],
        )

        schedule = VGroup(
            _badge("Nacht", "weit öffnen", P_BLUE),
            _badge("Tag", "schließen + verschatten", P_ORANGE),
        ).arrange(RIGHT, buff=0.45)
        schedule.move_to(np.array([-1.35, -2.05, 0.0]))
        schedule_winter = VGroup(
            _badge("Nacht", "geschlossen", P_BLUE),
            _badge("Tag", "kurz stoßlüften", P_ORANGE),
        ).arrange(RIGHT, buff=0.45).move_to(schedule)
        season_lbl = Text("Sommer", font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(schedule, LEFT, buff=0.3)
        season_lbl_winter = Text("Winter", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to(season_lbl)
        mass_warm = Text("21 °C", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(mass)

        chart = VGroup(axes, x_ticks, x_unit, y_ticks, y_unit, comfort, comfort_lbl,
                       keys, icon, icon_room, mass, mass_lbl, icon_win, schedule, season_lbl)
        fit_band(chart)

        mass_spots = [mass.get_top() + LEFT * 0.6, mass.get_top() + RIGHT * 0.6]

        def stored(rt, color=P_RED):
            return [ripples(mass_spots, r_max=0.32, color=color, cycles=rt / 1.3)]

        self.play(Create(axes), FadeIn(x_ticks), FadeIn(y_ticks), FadeIn(x_unit), FadeIn(y_unit), run_time=1.6)
        self.play(FadeIn(comfort), FadeIn(comfort_lbl), run_time=0.7)
        self.play(Create(outdoor), FadeIn(keys[0]), run_time=2.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.6 + 0.7 + 2.2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "day"))
        self.play(Create(icon_room), FadeIn(icon_win), FadeIn(mass_lbl), FadeIn(sun), run_time=1.1)
        self.add(mass)
        self.play(mass.animate.set_fill(P_RED, opacity=0.55), FadeIn(mass_hot), *stored(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "day", during=stored)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "without"))
        self.play(Create(sealed), FadeIn(keys[1]), *stored(2.2), run_time=2.2)
        self.play(FadeIn(warn, shift=DOWN * 0.1), Indicate(sealed, color=P_RED, scale_factor=1.0), *stored(1.0),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "without", during=stored)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "night"))
        self.play(
            FadeIn(nights), FadeIn(night_lbl), FadeOut(warn),
            FadeOut(sun, shift=UP * 0.3), FadeIn(moon, shift=DOWN * 0.3),
            run_time=1.0,
        )
        sweep = VGroup(
            smooth_path([
                icon.get_left() + UP * 0.28,
                icon.get_center() + DOWN * 0.02,
                icon.get_right() + UP * 0.28,
            ]),
        )
        def purge(rt):
            return [_streams([(sweep, P_BLUE, P_ORANGE)], rt, speed=1.0, waves=4), *stored(rt, P_ORANGE)]

        self.play(Create(flow_guides(sweep, P_BLUE)), run_time=0.5)
        self.play(*purge(2.2), run_time=2.2)
        hold_for(self, self.NARRATION, "night", during=purge)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "result"))
        self.play(
            Create(purged), FadeIn(keys[2]),
            mass.animate.set_fill(P_BLUE, opacity=0.45),
            ReplacementTransform(mass_hot, mass_cool),
            _streams([(sweep, P_BLUE, P_ORANGE)], 2.2, speed=1.0, waves=4),
            run_time=2.2,
        )
        peak = 42.0
        gain = dim_arrow(
            axes.c2p(peak, self._indoor_purged(peak)),
            axes.c2p(peak, self._indoor_sealed(peak)),
            color=P_YELLOW,
        )
        gain_lbl = Text("≈ 4 K", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        gain_lbl.next_to(gain, DOWN, buff=0.32).shift(LEFT * 0.38)
        self.play(Create(gain), FadeIn(gain_lbl), run_time=1.0)
        hold_for(self, self.NARRATION, "result", used=2.2 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "schedule"))
        self.play(FadeIn(schedule, shift=UP * 0.12), FadeIn(season_lbl), run_time=1.0)
        hold_for(self, self.NARRATION, "schedule", used=1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        self.play(
            ReplacementTransform(schedule, schedule_winter),
            ReplacementTransform(season_lbl, season_lbl_winter),
            mass.animate.set_fill(P_ORANGE, opacity=0.5),
            ReplacementTransform(mass_cool, mass_warm),
            *stored(1.4, P_ORANGE),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "winter", used=1.4 + 0.35, during=lambda rt: stored(rt, P_ORANGE))
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Where free ventilation runs out
class Beat6_GrenzenDerFreienLueftung(Scene):
    NARRATION = [
        ("intro",
         "Everything so far rested on one assumption: that the outdoor air is usable. Free ventilation has no answer for the hours when it is not, in summer or in winter.",
         "Bisher galt eine Annahme: die Außenluft ist brauchbar. Wenn nicht,\nhat die freie Lüftung in Sommer wie Winter keine Antwort."),
        ("hot",
         "On a hot, humid summer afternoon the incoming air carries heat and moisture into the room instead of taking them out.",
         "An einem heißen, schwülen Sommernachmittag trägt die Zuluft\nWärme und Feuchte herein statt heraus."),
        ("cold",
         "In winter it is the other way round: air at zero degrees streams in and the room loses heat.",
         "Im Winter ist es umgekehrt: Luft mit 0 °C strömt herein,\nder Raum verliert Wärme."),
        ("filter",
         "An open window also has no filter. Pollen, dust and street noise come in together with the air.",
         "Ein offenes Fenster hat auch keinen Filter — Pollen,\nStaub und Straßenlärm kommen mit."),
        ("control",
         "And the flow follows the weather, not the design. Wind and buoyancy change all day, so this volume flow can never be sized against a heating or cooling load.",
         "Der Volumenstrom folgt dem Wetter, nicht dem Entwurf — auf eine\nHeiz- oder Kühllast lässt er sich nicht auslegen."),
        ("recovery",
         "Finally, all the energy in the air leaving the room simply disappears through the opening. With η equal to zero nothing is recovered: in winter the full 1500 watts of ventilation heat loss.",
         "Die gesamte Energie der abströmenden Luft verschwindet durch die Öffnung:\nη = 0, im Winter gehen die vollen 1500 W Lüftungswärmeverlust verloren."),
        ("handover",
         "Closing those four gaps is exactly what fan-assisted ventilation is for, with heat recovery: the next chapter. Heating and Cooling will build on both.",
         "Diese vier Lücken schließt die mechanische Lüftung mit\nWärmerückgewinnung — nächstes Kapitel, Basis für Heizung und Kühlung."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Grenzen der freien Lüftung", title)
        din = _din_ref("DIN EN 16798-7")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([-3.40, 0.55, 0.0])
        shell = _room(room_c, 4.4, 2.2)
        room, floor = shell[0], shell[1]
        wash = Rectangle(
            width=4.3, height=2.1, stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
        ).move_to(room_c)
        win = _window(np.array([room.get_left()[0], 0.55, 0.0]), height=1.0)
        person = _stand(room_c, 2.2, -1.65)

        outdoor = _badge("Sommernachmittag", "32 °C · schwül", P_RED)
        outdoor.move_to(np.array([-3.40, 2.15, 0.0]))
        outdoor_cold = _badge("Winter", "0 °C · kalt", P_CYAN).move_to(outdoor)

        inflow = VGroup(
            smooth_path([
                win.get_right() + RIGHT * 0.05 + UP * 0.20,
                room_c + LEFT * 0.9 + UP * 0.35,
                room_c + RIGHT * 0.9 + DOWN * 0.10,
            ]),
        )
        outflow = VGroup(
            smooth_path([
                room_c + RIGHT * 0.9 + UP * 0.45,
                room_c + LEFT * 0.9 + UP * 0.72,
                win.get_right() + RIGHT * 0.05 + UP * 0.62,
            ]),
        )

        rng = np.random.default_rng(9)
        grime = VGroup(*[
            Dot(
                room_c + np.array([float(rng.uniform(-1.7, 1.7)), float(rng.uniform(-0.7, 0.8)), 0.0]),
                radius=0.045, color=P_WHITE, fill_opacity=0.55, stroke_width=0,
            )
            for _ in range(14)
        ])
        grime_tag = Text("Pollen · Staub", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        grime_tag.next_to(room, DOWN, buff=0.16)

        lost = Arrow(
            win.get_left() + LEFT * 0.10 + UP * 0.62,
            win.get_left() + LEFT * 0.95 + UP * 0.62,
            buff=0, color=P_ORANGE, stroke_width=4, max_tip_length_to_length_ratio=0.24,
        )
        lost_tag = Text("ungenutzt", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        lost_tag.next_to(lost, UP, buff=0.12).set_x(room.get_left()[0] - 0.12 - lost_tag.width / 2)

        gaps = VGroup(*[
            VGroup(cross_mark(), Text(t, font_size=BODY_FONT_SIZE, color=P_WHITE)).arrange(RIGHT, buff=0.22)
            for t in (
                "Außenluft zu heiß, schwül oder kalt",
                "keine Filterung",
                "Volumenstrom nicht regelbar",
                "keine Rückgewinnung (η = 0)",
            )
        ]).arrange(DOWN, aligned_edge=LEFT, buff=0.34)
        gaps.move_to(np.array([4.05, 0.45, 0.0]))

        lost_eta = math_label(r"\eta = 0", size=BODY_FONT_SIZE, color=P_ORANGE)
        lost_eta.next_to(lost, DOWN, buff=0.14).set_x(lost_tag.get_x())
        q_loss = V_NET * N_RATE * C_LUFT * D_THETA
        loss_read = math_label(
            rf"\text{{Winter:}}\;\dot{{Q}}_{{\mathrm{{V,sens}}}} \approx {de_num(round(q_loss, -2))}\,\mathrm{{W}}",
            size=LABEL_FONT_SIZE, color=P_CYAN,
        ).move_to(np.array([4.05, -1.05, 0.0]))

        handover = VGroup(
            chip("→ mechanische Lüftung · nächstes Kapitel", P_TEAL, font_size=LABEL_FONT_SIZE),
            _hand_chip("→ Heizung:", r"\Phi_{\mathrm{V}}", P_ORANGE),
            _hand_chip("→ Kühlung:", r"\dot{Q}_{\mathrm{L}}", P_CYAN),
        ).arrange(RIGHT, buff=0.3)
        handover.move_to(np.array([0.0, -1.95, 0.0]))

        scaffold = VGroup(room, floor, wash, win, person, outdoor, grime_tag, gaps, handover, loss_read)
        fit_band(scaffold)

        vol = meter("Volumenstrom", length=2.1, thickness=0.52, color=P_CYAN)
        vol["group"].move_to(np.array([0.3, 0.45, 0.0]))
        rate = ValueTracker(0.50)
        bind_meter(vol, rate)

        air = {"in": (P_CYAN,), "out": (P_CYAN,), "speed": 0.9}

        def exchange(rt):
            return [_streams([(inflow, *air["in"]), (outflow, *air["out"])], rt, speed=air["speed"])]

        self.add(wash)
        self.play(Create(room), Create(floor), FadeIn(win), FadeIn(person), run_time=1.3)
        self.play(Create(flow_guides(inflow, P_CYAN)), Create(flow_guides(outflow, P_CYAN)), run_time=0.7)
        self.play(*exchange(1.8), run_time=1.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.7 + 1.8, during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "hot"))
        air["in"] = (P_RED, P_ORANGE)
        self.play(FadeIn(outdoor), wash.animate.set_fill(P_RED, opacity=0.22), *exchange(1.1), run_time=1.1)
        self.play(*exchange(2.0), run_time=2.0)
        self.play(FadeIn(gaps[0], shift=RIGHT * 0.2), *exchange(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "hot", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cold"))
        air["in"] = (P_BLUE, P_ORANGE)
        self.play(ReplacementTransform(outdoor, outdoor_cold), wash.animate.set_fill(P_BLUE, opacity=0.16),
                  *exchange(1.2), run_time=1.2)
        self.play(*exchange(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "cold", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "filter"))
        self.play(
            LaggedStart(*[FadeIn(d, scale=0.4) for d in grime], lag_ratio=0.06),
            FadeIn(grime_tag), FadeIn(gaps[1], shift=RIGHT * 0.2), *exchange(1.8),
            run_time=1.8,
        )
        hold_for(self, self.NARRATION, "filter", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "control"))
        self.play(FadeIn(vol["group"]), *exchange(0.7), run_time=0.7)
        for level in (0.82, 0.22, 0.95, 0.38):
            air["speed"] = 0.4 + 1.2 * level
            self.play(rate.animate.set_value(level), *exchange(0.5), run_time=0.5)
        self.play(FadeIn(gaps[2], shift=RIGHT * 0.2), *exchange(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "control", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "recovery"))
        air["out"] = (P_ORANGE,)
        self.play(GrowArrow(lost), FadeIn(lost_tag), FadeIn(lost_eta), *exchange(0.9), run_time=0.9)
        self.play(*exchange(1.8), run_time=1.8)
        self.play(FadeIn(gaps[3], shift=RIGHT * 0.2), FadeIn(loss_read, shift=UP * 0.1), *exchange(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "recovery", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "handover"))
        self.play(Indicate(gaps, color=P_RED, scale_factor=1.04), *exchange(1.0), run_time=1.0)
        self.play(LaggedStart(*[FadeIn(h, shift=UP * 0.14) for h in handover], lag_ratio=0.25),
                  *exchange(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "handover", during=exchange)
        vol["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
