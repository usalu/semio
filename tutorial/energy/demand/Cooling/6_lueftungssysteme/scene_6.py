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
    meter, bind_meter, chip, cross_mark, dim_chip, dim_arrow,
    equation_row, formula_panel, highlight_param,
    math_label, math_panel, math_readout, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    house_section, person_glyph, sun_glyph, moon_glyph, ripples, flow_animation, droplets,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Lüftungssysteme"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``Heating/2_conduction/scene_2.py``): a dim ``P_TEAL`` footnote
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


def _fan(pos, color, radius=0.24):
    """🌀 Hub plus three blades — the glyph that marks a stream as mechanically driven."""
    ring = Circle(radius=radius, color=color, stroke_width=2.4)
    blades = VGroup(*[
        Line(ORIGIN, RIGHT * radius * 0.78, color=color, stroke_width=2.4).rotate(a, about_point=ORIGIN)
        for a in (0.0, TAU / 3, 2 * TAU / 3)
    ])
    return VGroup(ring, blades).move_to(pos)


def _spin(fans, rt: float, period: float = 0.9):
    """🔄 Turn the blades of every ``_fan`` for ``rt`` seconds."""
    return [Rotate(f[1], angle=-TAU * rt / period, about_point=f[0].get_center(), rate_func=linear) for f in fans]


def _streams(streams, rt: float, *, speed: float = 0.85, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


def _port(pos, color=P_WHITE):
    """🕳️ Passive wall opening — the uncontrolled half of a single-fan system."""
    return Rectangle(
        width=0.16, height=0.50, color=color, stroke_width=2.6,
        fill_color=P_DEEP_DARK, fill_opacity=1.0,
    ).move_to(pos)


def _park(node, buff=0.28):
    """🅿️ Point just outside a node's left edge — where a travelling token waits without covering the label."""
    return node.get_left() + LEFT * buff


#endregion


#region Beat1 – Load first: shrink cooling before moving air
class Beat1_PassivhausIdee(Scene):
    NARRATION = [
        ("intro",
         "Summer comfort starts before any fan runs. The first tool is neither a ventilator nor a chiller.",
         "Sommerkomfort beginnt, bevor ein Ventilator läuft —\ndas erste Werkzeug ist weder Lüfter noch Kältemaschine."),
        ("envelope",
         "Start from the cooling load. A heavily insulated, airtight envelope with external shading removes the largest share before any air is moved at all.",
         "Start bei der Kühllast: eine gedämmte, dichte Hülle mit\naußenliegendem Sonnenschutz nimmt den größten Anteil weg."),
        ("natural",
         "The air path cannot be an open window. Without heat recovery the ventilation heat loss is far too high, so a Passivhaus gives that path up.",
         "Der Luftweg darf kein offenes Fenster sein: ohne Rückgewinnung\nsind die Lüftungswärmeverluste viel zu hoch."),
        ("reserve",
         "What remains is supply and exhaust with heat recovery, and only a small cooling reserve. That is the order this chapter follows.",
         "Was bleibt, ist Zu- und Abluft mit Wärmerückgewinnung —\nund nur eine kleine Kühlreserve."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Erst die Last senken, dann lüften", title)
        din = _din_ref("DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hs = house_section(np.array([-3.85, -0.05, 0.0]), scale=0.8)
        bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
        lo_win, up_win = hs["windows"]
        person = _person(np.array([-3.25, bl[1] + 0.02 + 0.66 * 1.09 * 0.8 / 2, 0.0]), color=P_ORANGE, scale=0.8)
        house = VGroup(hs["group"], person)

        off = 0.09
        envelope = VMobject(color=P_TEAL, stroke_width=6, stroke_opacity=0.85).set_points_as_corners([
            bl + LEFT * off, tl + LEFT * off, tl + LEFT * (0.26 * 0.8 + off) + UP * 0.02, peak + UP * off,
            tr + RIGHT * (0.26 * 0.8 + off) + UP * 0.02, tr + RIGHT * off, br + RIGHT * off,
        ])
        louvre = VGroup(*[
            Line(np.array([up_win["x"] - 0.42, y + 0.09, 0.0]), np.array([up_win["x"] - 0.24, y - 0.09, 0.0]),
                 color=P_TEAL, stroke_width=3)
            for y in np.linspace(up_win["sill"] + 0.02, up_win["lintel"] - 0.02, 4)
        ])
        warm_spots = [person.get_top() + UP * 0.05, np.array([-2.75, hs["level_1"].get_center()[1] + 0.02, 0.0])]

        def heat(rt, r_max=0.45):
            return [ripples(warm_spots, r_max=r_max, cycles=rt / 1.4)]

        load_bar = meter("Kühllast", length=4.5, thickness=0.5, color=P_RED, vertical=False)
        load_bar["group"].move_to(np.array([2.35, 1.62, 0.0]))
        load = ValueTracker(1.0)
        bind_meter(load_bar, load)

        def _step(idx, name, share, color):
            chip = Circle(radius=0.17, color=color, stroke_width=2, fill_color=color, fill_opacity=1.0)
            num = Text(idx, font_size=LABEL_FONT_SIZE, color=P_DEEP_DARK).move_to(chip.get_center())
            return VGroup(
                VGroup(chip, num),
                Text(name, font_size=BODY_FONT_SIZE, color=P_WHITE),
                Text(share, font_size=BODY_FONT_SIZE, color=color),
            ).arrange(RIGHT, buff=0.20)

        steps = VGroup(
            _step("1", "Hülle + Sonnenschutz", "Last sinkt", P_TEAL),
            _step("2", "offenes Fenster", "Verlust zu hoch", P_RED),
            _step("3", "Zu-/Abluft mit WRG", "Reserve", P_ORANGE),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.34)
        steps.move_to(np.array([2.55, -0.30, 0.0]))

        self.play(Create(hs["group"]), run_time=1.5)
        self.play(FadeIn(person), *heat(0.8), run_time=0.8)
        self.play(FadeIn(load_bar["group"]), *heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 0.8 + 1.0,
                 during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "envelope"))
        self.play(Create(envelope), Create(louvre), *heat(1.2), run_time=1.2)
        self.play(
            load.animate.set_value(0.55),
            load_bar["fill"].animate.set_fill(P_YELLOW),
            FadeIn(steps[0], shift=RIGHT * 0.2), *heat(1.7, 0.32),
            run_time=1.7,
        )
        hold_for(self, self.NARRATION, "envelope", during=lambda rt: heat(rt, 0.3))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "natural"))

        def _through(w):
            x, y = w["x"], w["center"][1]
            pts = [(x + 1.15, y - 0.12), (x + 0.35, y - 0.04), (x - 0.35, y + 0.02), (x - 1.15, y + 0.16)]
            return smooth_path([np.array([px, py, 0.0]) for px, py in pts])

        loss_paths = VGroup(*[_through(w) for w in (lo_win, up_win)])
        loss_guides = flow_guides(loss_paths, P_RED)

        def loss(rt):
            return [_streams([(loss_paths, P_RED, P_ORANGE)], rt, speed=1.05), *heat(rt, 0.28)]

        self.play(FadeIn(steps[1], shift=RIGHT * 0.2), Create(loss_guides), run_time=0.8)
        self.play(*loss(1.8), run_time=1.8)
        crosses = VGroup(*[cross_mark(size=0.18).move_to(w["center"]) for w in (lo_win, up_win)])
        self.play(FadeIn(crosses), loss_guides.animate.set_stroke(opacity=0.12), run_time=0.6)
        hold_for(self, self.NARRATION, "natural", during=lambda rt: heat(rt, 0.22))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "reserve"))
        self.play(
            load.animate.set_value(0.25),
            load_bar["fill"].animate.set_fill(P_ORANGE),
            FadeIn(steps[2], shift=RIGHT * 0.2), *heat(1.4, 0.18),
            run_time=1.4,
        )
        self.play(Indicate(steps[2], color=P_ORANGE), *heat(1.0, 0.16), run_time=1.0)
        hold_for(self, self.NARRATION, "reserve", during=lambda rt: heat(rt, 0.16))
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – The window is a dial, not a switch
class Beat2_Fensterregeln(Scene):
    NARRATION = [
        ("intro",
         "Natural ventilation is adjustable. A single facade opening already gives you a dial, not a switch.",
         "Natürliche Lüftung ist einstellbar — eine Fassadenöffnung\nist ein Regler, kein Schalter."),
        ("closed",
         "Fully closed, the air change rate drops to almost nothing. Heat and moisture given off by the occupants build up and comfort collapses.",
         "Ganz geschlossen sinkt der Luftwechsel fast auf null —\nWärme und Feuchte stauen sich, der Komfort bricht ein."),
        ("partial",
         "Open a narrow gap and the air change rate climbs into the comfort band: enough fresh air, and still no draught.",
         "Ein schmaler Spalt hebt den Luftwechsel ins Komfortband —\ngenug Frischluft, keine Zugluft."),
        ("cooler",
         "When the outdoor air is cooler than the room, open wide. The very same window now flushes heat out of the space.",
         "Ist die Außenluft kühler als der Raum, weit öffnen —\ndasselbe Fenster spült jetzt Wärme hinaus."),
        ("hotter",
         "When the outdoor air is hotter, throttle back. A wide opening would import heat instead of removing it.",
         "Ist es draußen heißer, drosseln — eine weite Öffnung\nwürde Wärme hereinholen statt abführen."),
        ("rule",
         "So the rule is simple: set the window for the people inside, judged against the outdoor temperature.",
         "Die Regel ist einfach: das Fenster nach den Menschen\ndrinnen und der Außentemperatur einstellen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
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
        person = _person(room_c + LEFT * 1.05 + DOWN * 0.53)

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

        # Right edges share one x left of the window, so the wider readings never reach the frame.
        def _open_lbl(text, color):
            return Text(text, font_size=LABEL_FONT_SIZE, color=color).next_to(
                frame, LEFT, buff=0.22, aligned_edge=DOWN)

        open_pct = _open_lbl("Öffnung 0 %", P_CYAN)
        open_mid = _open_lbl("Öffnung 35 %", P_CYAN)
        open_wide = _open_lbl("Öffnung 85 %", P_CYAN)
        open_thin = _open_lbl("Öffnung 15 %", P_ORANGE)

        outdoor_cool = _badge("Außenluft", "18 °C", P_BLUE)
        outdoor_cool.move_to(np.array([-5.75, 1.30, 0.0]))
        outdoor_hot = _badge("Außenluft", "32 °C", P_RED).move_to(outdoor_cool)

        scaffold = VGroup(room, floor, wash, person, frame, sash, gap, open_pct, outdoor_cool)
        fit_band(scaffold)

        stuffy_spots = [person.get_top() + UP * 0.05, person.get_top() + RIGHT * 1.3 + DOWN * 0.3]

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

        n_low = Text("n ≈ 0,2 1/h", font_size=BODY_FONT_SIZE, color=P_RED)
        n_low.next_to(track, DOWN, buff=0.26)
        n_mid = Text("n ≈ 2 1/h", font_size=BODY_FONT_SIZE, color=P_GREEN).move_to(n_low)
        n_high = Text("n ≈ 6 1/h", font_size=BODY_FONT_SIZE, color=P_BLUE).move_to(n_low)
        n_thin = Text("n ≈ 1 1/h", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(n_low)

        tag_flush = Text("spülen", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        tag_flush.next_to(track, LEFT, buff=0.22).shift(UP * 0.75)
        tag_throttle = Text("drosseln", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to(tag_flush)

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
        rule = Text("Fenster = Regler, kein Schalter", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        rule.move_to(np.array([room_c[0], -1.78, 0.0]))

        def draft(rt, speed=0.8, color=P_CYAN, color_end=None, waves=3):
            return [_streams([(gentle, color, color_end)], rt, speed=speed, waves=waves)]

        self.add(wash)
        self.play(Create(room), Create(floor), FadeIn(person), run_time=1.3)
        self.play(FadeIn(frame), FadeIn(sash), FadeIn(gap), FadeIn(open_pct), run_time=0.8)
        self.play(FadeIn(air["group"]), FadeIn(comfort), FadeIn(comfort_lbl), FadeIn(n_low), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.8 + 1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "closed"))
        self.play(wash.animate.set_fill(P_RED, opacity=0.24), *stuffy(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "closed", during=stuffy)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "partial"))
        self.play(
            opening.animate.set_value(0.35),
            rate.animate.set_value(0.45),
            ReplacementTransform(open_pct, open_mid),
            ReplacementTransform(n_low, n_mid),
            air["fill"].animate.set_fill(P_GREEN),
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
            air["fill"].animate.set_fill(P_BLUE),
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
            air["fill"].animate.set_fill(P_ORANGE),
            wash.animate.set_fill(P_RED, opacity=0.12),
            ReplacementTransform(tag_flush, tag_throttle),
            *draft(1.6, 0.45, P_ORANGE, None, 2),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "hotter", during=lambda rt: draft(rt, 0.45, P_ORANGE, None, 2))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rule"))
        band_ring = SurroundingRectangle(comfort, color=P_GREEN, buff=0.06, stroke_width=3, corner_radius=0.05)
        self.play(FadeIn(rule), Create(band_ring), *draft(1.0, 0.45, P_ORANGE, None, 2), run_time=1.0)
        hold_for(self, self.NARRATION, "rule", during=lambda rt: draft(rt, 0.45, P_ORANGE, None, 2))
        sash.clear_updaters()
        air["fill"].clear_updaters()
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
         "Wind presses on the windward face and pulls on the leeward face. That pressure difference, not a fan, is what drives air through the occupied zone.",
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
        ("passive",
         "So size both sides together. In summer this flush can dump a lot of heat on windy days — it does not replace a design cooling plant by itself.",
         "Also beide Seiten gemeinsam dimensionieren — die Spülung führt an\nwindigen Tagen viel Last ab, ersetzt aber keine Kälteanlage."),
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
        person = _person(np.array([0.45, -0.19, 0.0]))
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
        tip = Text("natürliche Spülung vor der Kältemaschine", font_size=BODY_FONT_SIZE, color=P_TEAL)
        tip.move_to(aeff_wide)

        scaffold = VGroup(room, floor, person, left_win, right_win, a1_lbl, a2_lbl,
                          gusts, wind_lbl, luv, lee, aeff_wide)
        fit_band(scaffold, bottom=SAFE_BOTTOM_FORMULA)

        cross = VGroup(*[
            smooth_path([
                room.get_left() + UP * dy,
                room_c + LEFT * 1.5 + UP * (dy * 0.7),
                room_c + RIGHT * 1.5 + UP * (dy * 0.5 + 0.08),
                room.get_right() + UP * (dy * 0.55 + 0.12),
            ])
            for dy in (-0.30, 0.15, 0.60)
        ])
        # Same three inlet streams, but funnelled through the throttled outlet
        # (≈0.30 m tall, centred on the room axis) instead of running into the wall.
        cross_thin = VGroup(*[
            smooth_path([
                room.get_left() + UP * dy,
                room_c + LEFT * 1.5 + UP * (dy * 0.7),
                room_c + RIGHT * 1.2 + UP * (dy * 0.18),
                room.get_right() + UP * sy,
            ])
            for dy, sy in ((-0.30, -0.09), (0.15, 0.0), (0.60, 0.09))
        ])

        eq, eq_box, items = math_panel([
            ("aeff", r"A_{eff}", P_CYAN), (None, "=", P_WHITE),
            ("root",
             rf"\frac{{1}}{{\sqrt{{\textcolor{{{P_GREEN}}}{{\frac{{1}}{{A_{{1}}^{{2}}}}}} + "
             rf"\textcolor{{{P_ORANGE}}}{{\frac{{1}}{{A_{{2}}^{{2}}}}}}}}}}",
             P_WHITE),
            (None, r"\;[\mathrm{m^{2}}]", P_TEAL),
        ], color=P_CYAN)
        # root → outer frac [num, bar, den, marker]; den wraps the sqrt box
        # [radical, inner, bar, marker]; inner row = [1/A₁², +, 1/A₂², marker].
        _inner = items["root"][0][2][0][1]
        items["a1"], items["a2"] = _inner[0], _inner[2]

        self.play(Create(room), Create(floor), FadeIn(person), run_time=1.3)
        self.play(FadeIn(left_win), FadeIn(right_win), FadeIn(a1_lbl), FadeIn(a2_lbl), run_time=0.9)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.3 + 0.9)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "pressure"))
        self.play(
            LaggedStart(*[GrowArrow(a) for a in gusts], lag_ratio=0.18),
            FadeIn(wind_lbl), FadeIn(luv), FadeIn(lee),
            run_time=1.5,
        )
        def through(rt, paths=cross, color=P_GREEN, speed=0.85, waves=4):
            return [_streams([(paths, color)], rt, speed=speed, waves=waves)]

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

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "passive"))
        self.play(ReplacementTransform(aeff_wide, tip), *through(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "passive", during=through)
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
         "The density difference comes straight out of the temperature difference: the warmer the room air, the lighter it is, and the harder it is pushed up.",
         "Der Dichteunterschied folgt aus der Temperaturdifferenz:\nje wärmer die Raumluft, desto stärker der Auftrieb."),
        ("taller",
         "Raise the outlet and the driving pressure grows with the height. Here the shaft nearly doubles it, and the flow speeds up with it.",
         "Höherer Auslass, größere Höhe — der Druck wächst mit.\nDer Schacht wächst, der Strom wird schneller."),
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
        person = _person(np.array([-3.85, -0.94, 0.0]))

        shaft_walls = VGroup(*[
            Line(
                np.array([self.SHAFT_X + s * self.SHAFT_HW, room_top, 0.0]),
                np.array([self.SHAFT_X + s * self.SHAFT_HW, self.TOP_LOW, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            )
            for s in (-1, 1)
        ])
        outlet = VGroup(
            Line(
                np.array([self.SHAFT_X - self.SHAFT_HW, self.TOP_LOW, 0.0]),
                np.array([self.SHAFT_X + self.SHAFT_HW, self.TOP_LOW, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            ),
            Arrow(
                np.array([self.SHAFT_X + self.SHAFT_HW - 0.05, self.TOP_LOW - 0.12, 0.0]),
                np.array([self.SHAFT_X + self.SHAFT_HW + 0.65, self.TOP_LOW - 0.02, 0.0]),
                buff=0, color=P_ORANGE, stroke_width=4, max_tip_length_to_length_ratio=0.22,
            ),
        )
        outlet_lbl = Text("Abluft", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        # Beside the outlet, not above it: the shaft later rises 1.3 units and a
        # label riding on top would run into the beat subtitle.
        outlet_lbl.next_to(outlet[0], LEFT, buff=0.25)

        legend_rows = VGroup(
            VGroup(
                math_label(r"\rho_{a}", size=BODY_FONT_SIZE, color=P_BLUE),
                Text("außen · kühl · schwer", font_size=LABEL_FONT_SIZE, color=P_WHITE),
            ).arrange(RIGHT, buff=0.18),
            VGroup(
                math_label(r"\rho_{i}", size=BODY_FONT_SIZE, color=P_ORANGE),
                Text("innen · warm · leicht", font_size=LABEL_FONT_SIZE, color=P_WHITE),
            ).arrange(RIGHT, buff=0.18),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.16)
        legend = VGroup(
            SurroundingRectangle(legend_rows, color=P_TEAL, corner_radius=0.1, buff=0.16, stroke_width=1.8),
            legend_rows,
        )
        legend.move_to(np.array([1.55, 1.55, 0.0]))

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
        dp_lo = Text("≈ 1,2 Pa", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        dp_lo.next_to(dp["track"], DOWN, buff=0.26)
        dp_hi = Text("≈ 2,1 Pa", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(dp_lo)

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
        self.play(FadeIn(dp["group"]), press.animate.set_value(0.40), FadeIn(dp_lo), *rising(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "formula", during=rising)

        h_dim = dim_arrow(
            np.array([-0.30, inlet.get_center()[1], 0.0]),
            np.array([-0.30, self.TOP_LOW, 0.0]),
            color=P_CYAN,
        )
        h_tok = symbol_token("h", color=P_CYAN, font_size=FORMULA_FONT_SIZE)
        h_tok.next_to(h_dim, LEFT, buff=0.14).shift(RIGHT * 0.16)
        ring_h = highlight_param(items, "h", color=P_CYAN)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "h"))
        self.play(Create(h_dim), FadeIn(h_tok), Create(ring_h), *rising(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "h", during=rising)
        self.play(FadeOut(ring_h), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rho"))
        ring_rho = highlight_param(items, "rho", color=P_BLUE)
        self.play(
            Create(ring_rho),
            Indicate(legend_rows[0], color=P_BLUE),
            Indicate(legend_rows[1], color=P_ORANGE),
            *rising(1.0),
            run_time=1.0,
        )
        hold_for(self, self.NARRATION, "rho", during=rising)
        self.play(FadeOut(ring_rho), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "taller"))
        rise = self.TOP_HIGH - self.TOP_LOW
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
            h_tok.animate.shift(UP * rise / 2),
            FadeOut(guides_low),
            press.animate.set_value(0.70),
            ReplacementTransform(dp_lo, dp_hi),
            run_time=1.6,
        )
        stack["paths"] = stack_high = self._stack_paths(inlet.get_center(), room_top, self.TOP_HIGH)
        stack["speed"] = 1.1
        self.play(Create(flow_guides(stack_high, P_ORANGE)), run_time=0.5)
        self.play(*rising(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "taller", during=rising)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "comfort"))
        low_ring = SurroundingRectangle(VGroup(inlet, person), color=P_CYAN, buff=0.12,
                                        corner_radius=0.08, stroke_width=2.5)
        high_ring = SurroundingRectangle(outlet[0], color=P_ORANGE, buff=0.14,
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
         "So night ventilation is a schedule, not a setting: wide open at night, closed and shaded by day.",
         "Nachtlüftung ist ein Zeitplan, keine Einstellung:\nnachts weit offen, tagsüber zu und verschattet."),
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
        # Sky object sits outside the room icon, above its top-left corner.
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

        chart = VGroup(axes, x_ticks, x_unit, y_ticks, y_unit, comfort, comfort_lbl,
                       keys, icon, icon_room, mass, mass_lbl, icon_win, schedule)
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
        # Below the bracket: every curve runs above 25 °C there at hours 40–44.
        gain_lbl.next_to(gain, DOWN, buff=0.32).shift(LEFT * 0.38)
        self.play(Create(gain), FadeIn(gain_lbl), run_time=1.0)
        hold_for(self, self.NARRATION, "result", used=2.2 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "schedule"))
        self.play(FadeIn(schedule, shift=UP * 0.12), run_time=1.0)
        hold_for(self, self.NARRATION, "schedule", used=1.0 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Where free ventilation runs out
class Beat6_GrenzenDerFreienLueftung(Scene):
    NARRATION = [
        ("intro",
         "Everything so far rested on one assumption: that the outdoor air is usable. Free ventilation has no answer for the hours when it is not.",
         "Bisher galt eine Annahme: die Außenluft ist brauchbar.\nWenn nicht, hat die freie Lüftung keine Antwort."),
        ("hot",
         "On a hot, humid afternoon the incoming air carries heat and moisture into the room instead of taking them out.",
         "An einem heißen, schwülen Nachmittag trägt die Zuluft\nWärme und Feuchte herein statt heraus."),
        ("filter",
         "An open window also has no filter. Pollen, dust and street noise come in together with the air.",
         "Ein offenes Fenster hat auch keinen Filter — Pollen,\nStaub und Straßenlärm kommen mit."),
        ("control",
         "And the flow follows the weather, not the design. Wind and buoyancy change all day, so this volume flow can never be sized against a cooling load.",
         "Der Volumenstrom folgt dem Wetter, nicht dem Entwurf —\nauf eine Kühllast lässt er sich nicht auslegen."),
        ("recovery",
         "Finally, all the energy in the air leaving the room simply disappears through the opening. Nothing is recovered.",
         "Die gesamte Energie der abströmenden Luft verschwindet\ndurch die Öffnung — nichts wird zurückgewonnen."),
        ("handover",
         "Closing those four gaps is exactly what fan-assisted ventilation is for. This is the reserve the first beat set aside.",
         "Genau diese vier Lücken schließt die ventilatorgestützte\nLüftung — die Reserve aus dem ersten Beat."),
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
        person = _person(np.array([-2.55, -0.19, 0.0]))

        outdoor = _badge("Außenluft", "34 °C · schwül", P_RED)
        outdoor.move_to(np.array([-3.40, 2.15, 0.0]))

        inflow = VGroup(
            smooth_path([
                win.get_right() + RIGHT * 0.05 + UP * 0.20,
                room_c + LEFT * 0.9 + UP * 0.35,
                room_c + RIGHT * 1.4 + DOWN * 0.10,
            ]),
        )
        outflow = VGroup(
            smooth_path([
                room_c + RIGHT * 1.4 + UP * 0.45,
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
                "Außenluft zu heiß oder zu feucht",
                "keine Filterung",
                "Volumenstrom nicht regelbar",
                "keine Wärmerückgewinnung",
            )
        ]).arrange(DOWN, aligned_edge=LEFT, buff=0.34)
        gaps.move_to(np.array([4.05, 0.45, 0.0]))

        handover = chip("→ ventilatorgestützte Lüftung", P_CYAN)
        handover.move_to(np.array([-1.40, -1.78, 0.0]))

        scaffold = VGroup(room, floor, wash, win, person, outdoor, grime_tag, gaps, handover)
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
        self.play(GrowArrow(lost), FadeIn(lost_tag), *exchange(0.9), run_time=0.9)
        self.play(*exchange(1.8), run_time=1.8)
        self.play(FadeIn(gaps[3], shift=RIGHT * 0.2), *exchange(0.7), run_time=0.7)
        hold_for(self, self.NARRATION, "recovery", during=exchange)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "handover"))
        self.play(Indicate(gaps, color=P_RED, scale_factor=1.04), *exchange(1.0), run_time=1.0)
        self.play(FadeIn(handover, shift=UP * 0.14), *exchange(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "handover", during=exchange)
        vol["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 – The three mechanical base types
class Beat7_MechanischeGrundtypen(Scene):
    NARRATION = [
        ("intro",
         "Fan-assisted ventilation comes in three basic types. They differ in one thing only: which of the two air streams gets a fan.",
         "Ventilatorgestützte Lüftung hat drei Grundtypen — sie\nunterscheiden sich nur darin, wer einen Ventilator bekommt."),
        ("exhaust",
         "The exhaust-only system extracts air mechanically. Fresh air seeps in unfiltered through passive wall vents, and the room sits at a slight negative pressure.",
         "Die Abluftanlage saugt mechanisch ab — die Zuluft kommt\nungefiltert nach, im Raum herrscht leichter Unterdruck."),
        ("supply",
         "The supply-only system does the opposite. Treated air is pushed in and leaks back out through joints and gaps, at a slight positive pressure.",
         "Die Zuluftanlage macht es umgekehrt: aufbereitete Luft\nwird eingeblasen und entweicht über Fugen — Überdruck."),
        ("balanced",
         "The balanced supply and exhaust system ducts both streams. The pressures cancel out, and both volume flows are known quantities.",
         "Die Zu-/Abluftanlage führt beide Ströme — die Drücke\ngleichen sich aus, beide Volumenströme sind bekannt."),
        ("verdict",
         "Only that third type has both streams under control, and only then do filtering, humidity control and heat recovery become possible at all.",
         "Nur der dritte Typ hat beide Ströme unter Kontrolle —\nerst dann sind Filter, Feuchte und WRG möglich."),
    ]

    CENTERS = (
        np.array([-4.55, 0.35, 0.0]),
        np.array([0.0, 0.35, 0.0]),
        np.array([4.55, 0.35, 0.0]),
    )
    ROOM_W, ROOM_H = 3.70, 1.90

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Drei mechanische Grundtypen", title)
        din = _din_ref("DIN EN 16798-3")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        c0, c1, c2 = self.CENTERS
        shells = [_room(c, self.ROOM_W, self.ROOM_H) for c in self.CENTERS]
        rooms = VGroup(*[s[0] for s in shells])
        floors = VGroup(*[s[1] for s in shells])
        wall_y = rooms[0].get_top()[1]

        headers = VGroup(*[
            Text(t, font_size=BODY_FONT_SIZE, color=c).move_to(np.array([cx[0], 1.98, 0.0]))
            for t, c, cx in (
                ("Abluftanlage", P_ORANGE, c0),
                ("Zuluftanlage", P_CYAN, c1),
                ("Zu-/Abluftanlage", P_GREEN, c2),
            )
        ])

        def _duct_arrow(start, end, color):
            return Arrow(start, end, buff=0, color=color, stroke_width=4,
                         max_tip_length_to_length_ratio=0.26)

        exh_fan = _fan(np.array([c0[0] + 1.25, wall_y, 0.0]), P_ORANGE)
        exh_out = _duct_arrow(np.array([c0[0] + 1.58, wall_y, 0.0]),
                              np.array([c0[0] + 2.10, wall_y, 0.0]), P_ORANGE)
        ald = _port(np.array([c0[0] - self.ROOM_W / 2, 0.70, 0.0]))
        exh_paths = VGroup(*[
            smooth_path([
                ald.get_right(),
                c0 + LEFT * 0.6 + UP * (dy * 0.5),
                c0 + RIGHT * 0.8 + UP * (dy * 0.35 + 0.25),
                np.array([c0[0] + 1.25, wall_y - 0.08, 0.0]),
            ])
            for dy in (-0.6, 0.0, 0.55)
        ])

        sup_fan = _fan(np.array([c1[0] - 1.25, wall_y, 0.0]), P_CYAN)
        sup_in = _duct_arrow(np.array([c1[0] - 2.10, wall_y, 0.0]),
                             np.array([c1[0] - 1.58, wall_y, 0.0]), P_CYAN)
        leak = _port(np.array([c1[0] + self.ROOM_W / 2, 0.70, 0.0]))
        sup_paths = VGroup(*[
            smooth_path([
                np.array([c1[0] - 1.25, wall_y - 0.08, 0.0]),
                c1 + LEFT * 0.8 + UP * (dy * 0.35 + 0.25),
                c1 + RIGHT * 0.6 + UP * (dy * 0.5),
                leak.get_left(),
            ])
            for dy in (-0.6, 0.0, 0.55)
        ])

        bal_sup = _fan(np.array([c2[0] - 1.25, wall_y, 0.0]), P_CYAN)
        bal_exh = _fan(np.array([c2[0] + 1.25, wall_y, 0.0]), P_ORANGE)
        bal_in = _duct_arrow(np.array([c2[0] - 2.10, wall_y, 0.0]),
                             np.array([c2[0] - 1.58, wall_y, 0.0]), P_CYAN)
        bal_out = _duct_arrow(np.array([c2[0] + 1.58, wall_y, 0.0]),
                              np.array([c2[0] + 2.10, wall_y, 0.0]), P_ORANGE)
        bal_paths = VGroup(*[
            smooth_path([
                np.array([c2[0] - 1.25, wall_y - 0.08, 0.0]),
                c2 + LEFT * 0.7 + UP * (dy * 0.4 + 0.1),
                c2 + RIGHT * 0.7 + UP * (dy * 0.4 - 0.05),
                np.array([c2[0] + 1.25, wall_y - 0.08, 0.0]),
            ])
            for dy in (-0.6, 0.0, 0.55)
        ])

        # Below the room, not at its vertical center: the flow paths inside
        # each panel converge through that centerline, so a sign placed there
        # sits directly on top of the animated stream.
        signs = VGroup(*[
            Text(t, font_size=BODY_FONT_SIZE, color=c).move_to(np.array([cx[0], -0.86, 0.0]))
            for (t, c), cx in zip((("−", P_ORANGE), ("+", P_CYAN), ("=", P_GREEN)), self.CENTERS)
        ])
        sign_tags = VGroup(*[
            Text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([cx[0], -1.18, 0.0]))
            for (t, c), cx in zip(
                (("Unterdruck", P_ORANGE), ("Überdruck", P_CYAN), ("ausgeglichen", P_GREEN)),
                self.CENTERS,
            )
        ])
        port_tags = VGroup(*[
            Text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([cx[0], -1.54, 0.0]))
            for (t, c), cx in zip(
                (("Außenluft ungefiltert", P_WHITE),
                 ("Abluft über Fugen", P_WHITE),
                 ("beide Ströme geführt", P_GREEN)),
                self.CENTERS,
            )
        ])

        verdict = Text(
            "erst beide Ströme geführt → Filter, Feuchte, Wärmerückgewinnung möglich",
            font_size=BODY_FONT_SIZE, color=P_GREEN,
        ).move_to(np.array([0.0, -2.18, 0.0]))

        scaffold = VGroup(rooms, floors, headers, exh_fan, exh_out, ald, sup_fan, sup_in, leak,
                          bal_sup, bal_exh, bal_in, bal_out, signs, sign_tags, port_tags, verdict)
        fit_band(scaffold)

        running = {"streams": [], "fans": []}

        def machines(rt):
            if not running["streams"]:
                return []
            return [_streams(running["streams"], rt, speed=0.8), *_spin(running["fans"], rt)]

        self.play(
            LaggedStart(*[Create(r) for r in rooms], lag_ratio=0.22),
            LaggedStart(*[Create(f) for f in floors], lag_ratio=0.22),
            LaggedStart(*[FadeIn(h, shift=DOWN * 0.1) for h in headers], lag_ratio=0.22),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0)

        for key, new_fans, extras, guide, stream, sign in (
            ("exhaust", [exh_fan], [GrowArrow(exh_out), FadeIn(ald), FadeIn(port_tags[0])],
             (exh_paths, P_ORANGE), (exh_paths, P_WHITE, P_ORANGE), 0),
            ("supply", [sup_fan], [GrowArrow(sup_in), FadeIn(leak), FadeIn(port_tags[1])],
             (sup_paths, P_CYAN), (sup_paths, P_CYAN, P_WHITE), 1),
            ("balanced", [bal_sup, bal_exh], [GrowArrow(bal_in), GrowArrow(bal_out), FadeIn(port_tags[2])],
             (bal_paths, P_GREEN), (bal_paths, P_CYAN, P_ORANGE), 2),
        ):
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(*[FadeIn(f) for f in new_fans], *extras, Create(flow_guides(*guide)), *machines(1.4),
                      run_time=1.4)
            running["streams"].append(stream)
            running["fans"].extend(new_fans)
            self.play(*machines(2.4), run_time=2.4)
            self.play(FadeIn(signs[sign], scale=1.2), FadeIn(sign_tags[sign]), *machines(0.8), run_time=0.8)
            hold_for(self, self.NARRATION, key, during=machines)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verdict"))
        pick = SurroundingRectangle(
            VGroup(rooms[2], headers[2], signs[2], sign_tags[2], port_tags[2], bal_in, bal_out),
            color=P_GREEN, buff=0.14, corner_radius=0.1, stroke_width=2.5,
        )
        self.play(Create(pick), FadeIn(verdict, shift=UP * 0.12), *machines(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "verdict", during=machines)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – Heat recovery, which in summer is cold recovery
class Beat8_Waermerueckgewinnung(Scene):
    NARRATION = [
        ("intro",
         "Once both streams are ducted, they can be run past each other in a plate heat exchanger without ever mixing.",
         "Sobald beide Ströme geführt sind, laufen sie im Platten-\nwärmeübertrager aneinander vorbei, ohne sich zu mischen."),
        ("supply",
         "Hot outdoor air enters the upper channel at thirty-two degrees.",
         "Heiße Außenluft tritt oben mit 32 Grad ein."),
        ("exhaust",
         "The cooler room exhaust runs the other way through the lower channel, at twenty-six degrees.",
         "Die kühlere Abluft läuft mit 26 Grad in Gegenrichtung durch den unteren Kanal."),
        ("transfer",
         "Heat crosses the plates from the hot stream into the cool one. In winter that preheats the supply air; in summer it works in reverse and precools it, so this is cold recovery.",
         "Wärme wandert vom heißen in den kühlen Strom — im Winter\nwärmt das die Zuluft vor, im Sommer kühlt es sie vor."),
        ("formula",
         "The temperature recovery efficiency eta is the temperature change the supply air actually gained, divided by the full difference that was available.",
         "Der Temperaturänderungsgrad η ist die erreichte Temperaturänderung\nder Zuluft, geteilt durch die verfügbare Differenz."),
        ("value",
         "Five kelvin gained out of six available: about eighty percent of the free cooling, taken before the chiller is asked for anything.",
         "Fünf von sechs möglichen Kelvin — rund achtzig Prozent,\nbevor die Kältemaschine überhaupt gefragt wird."),
        ("passivhaus",
         "Natural ventilation and the Passivhaus contradict each other. An open window recovers nothing, so the ventilation heat loss is far too high and window ventilation has to be given up as the main air path.",
         "Natürliche Lüftung und Passivhaus widersprechen sich: ohne Rückgewinnung\nsind die Lüftungswärmeverluste viel zu hoch — das Fenster entfällt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmerückgewinnung — im Sommer Kälterückgewinnung", title)
        din = _din_ref("DIN EN 308")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        y_sup, y_exh = 1.20, 0.00
        x_in, x_out = -5.15, 5.15
        core = Rectangle(width=3.6, height=2.0, color=P_WHITE, stroke_width=3.0)
        core.move_to(np.array([0.0, 0.60, 0.0]))
        plates = VGroup(*[
            Line(
                np.array([x, core.get_bottom()[1] + 0.12, 0.0]),
                np.array([x, core.get_top()[1] - 0.12, 0.0]),
                color=P_TEAL, stroke_width=1.4,
            ).set_stroke(opacity=0.5)
            for x in np.linspace(-1.5, 1.5, 11)
        ])
        core_tag = Text("Plattenwärmeübertrager", font_size=BODY_FONT_SIZE, color=P_WHITE)
        core_tag.move_to(np.array([0.0, -0.72, 0.0]))
        core_note = Text("getrennte Kanäle — die Ströme mischen sich nicht",
                         font_size=LABEL_FONT_SIZE, color=P_TEAL)
        core_note.move_to(np.array([0.0, -1.10, 0.0]))

        supply = VGroup(smooth_path([
            np.array([x_in, y_sup, 0.0]), np.array([-1.9, y_sup, 0.0]),
            np.array([1.9, y_sup, 0.0]), np.array([x_out, y_sup, 0.0]),
        ]))
        exhaust = VGroup(smooth_path([
            np.array([x_out, y_exh, 0.0]), np.array([1.9, y_exh, 0.0]),
            np.array([-1.9, y_exh, 0.0]), np.array([x_in, y_exh, 0.0]),
        ]))

        transfer = VGroup(*[
            Arrow(
                np.array([x, y_sup - 0.20, 0.0]), np.array([x, y_exh + 0.20, 0.0]),
                buff=0, color=P_YELLOW, stroke_width=3.5, max_tip_length_to_length_ratio=0.26,
            )
            for x in (-1.05, 0.0, 1.05)
        ])
        transfer_tag = Text("Wärmestrom", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        transfer_tag.move_to(np.array([2.85, 0.60, 0.0]))

        aul = _badge("Außenluft", "32 °C", P_RED).move_to(np.array([-6.00, y_sup + 0.05, 0.0]))
        zul = _badge("Zuluft", "32 °C", P_CYAN).move_to(np.array([6.00, y_sup + 0.05, 0.0]))
        abl = _badge("Abluft", "26 °C", P_TEAL).move_to(np.array([6.00, y_exh - 0.05, 0.0]))
        fol = _badge("Fortluft", "31 °C", P_ORANGE).move_to(np.array([-6.00, y_exh - 0.05, 0.0]))

        scaffold = VGroup(core, plates, core_tag, core_note, transfer, transfer_tag,
                          aul, zul, abl, fol)
        fit_band(scaffold, bottom=SAFE_BOTTOM_FORMULA)

        eq, eq_box, items = math_panel([
            ("phi", r"\eta", P_YELLOW), (None, "=", P_WHITE),
            ("frac",
             rf"\frac{{\textcolor{{{P_CYAN}}}{{θ_{{ZUL}} - θ_{{AUL}}}}}}"
             rf"{{\textcolor{{{P_TEAL}}}{{θ_{{ABL}} - θ_{{AUL}}}}}}",
             P_WHITE),
            (None, "=", P_WHITE),
            ("val", r"\frac{5\,\mathrm{K}}{6\,\mathrm{K}} \approx 0{,}8", P_YELLOW),
        ], color=P_YELLOW)
        items["num"] = items["frac"][0]

        running = []

        def exchanger(rt):
            return [_streams(running, rt, speed=0.9, waves=4)] if running else []

        self.play(Create(core), Create(plates), FadeIn(core_tag), FadeIn(core_note), run_time=1.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.8)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "supply"))
        self.play(Create(flow_guides(supply, P_RED, opacity=0.28)), FadeIn(aul), run_time=1.0)
        running.append((supply, P_RED, P_CYAN))
        self.play(*exchanger(2.2), run_time=2.2)
        self.play(FadeIn(zul), *exchanger(0.6), run_time=0.6)
        hold_for(self, self.NARRATION, "supply", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "exhaust"))
        self.play(Create(flow_guides(exhaust, P_TEAL, opacity=0.28)), FadeIn(abl), *exchanger(1.0), run_time=1.0)
        running.append((exhaust, P_TEAL, P_ORANGE))
        self.play(*exchanger(2.2), run_time=2.2)
        self.play(FadeIn(fol), *exchanger(0.6), run_time=0.6)
        hold_for(self, self.NARRATION, "exhaust", during=exchanger)

        zul_tr = ValueTracker(32.0)
        eta_read = math_readout(
            lambda: rf"\eta = {de_num((32.0 - zul_tr.get_value()) / 6.0, 2)}",
            np.array([0.0, 2.02, 0.0]),
            size=BODY_FONT_SIZE, color=P_YELLOW, edge="center",
        )
        zul_cool = _badge("Zuluft", "27 °C", P_CYAN).move_to(zul)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "transfer"))
        self.add(eta_read)
        self.play(
            LaggedStart(*[GrowArrow(a) for a in transfer], lag_ratio=0.25),
            FadeIn(transfer_tag),
            ReplacementTransform(zul, zul_cool),
            zul_tr.animate.set_value(27),
            *exchanger(1.6),
            run_time=1.6,
        )
        zul = zul_cool
        self.play(
            LaggedStart(*[Indicate(a, color=P_YELLOW, scale_factor=1.15) for a in transfer], lag_ratio=0.3),
            *exchanger(2.2), run_time=2.2,
        )
        hold_for(self, self.NARRATION, "transfer", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeOut(core_note), FadeOut(eta_read), FadeIn(eq), Create(eq_box), *exchanger(1.2), run_time=1.2)
        ring_num = highlight_param(items, "num", color=P_CYAN)
        self.play(Create(ring_num), Indicate(zul, color=P_CYAN), *exchanger(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "formula", during=exchanger)
        self.play(FadeOut(ring_num), *exchanger(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "value"))
        ring_val = highlight_param(items, "val", color=P_YELLOW)
        self.play(Create(ring_val), Indicate(items["val"], color=P_YELLOW, scale_factor=1.15), *exchanger(1.0),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "value", during=exchanger)

        # 🪟 Der Widerspruch: ein offenes Fenster hat η = 0 — wer WRG braucht,
        # kann die freie Lüftung nicht als Hauptluftweg behalten.
        self.play(FadeOut(ring_val), FadeOut(eq), FadeOut(eq_box), *exchanger(0.5), run_time=0.5)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "passivhaus"))
        open_win = _window(np.array([-3.3, -1.52, 0.0]), height=0.52, width=0.82)
        win_cross = cross_mark(size=0.2).move_to(open_win.get_center())
        win_eta = math_label(r"\text{offenes Fenster:}\;\eta = 0", size=BODY_FONT_SIZE, color=P_RED)
        win_eta.next_to(open_win, RIGHT, buff=0.3)
        ph_line = Text(
            "Passivhaus ⇒ Zu-/Abluft mit WRG — Fensterlüftung entfällt als Hauptweg",
            font_size=LABEL_FONT_SIZE, color=P_ORANGE,
        ).move_to(np.array([0.3, -2.12, 0.0]))
        loss = Arrow(
            open_win.get_center(), open_win.get_center() + LEFT * 1.15 + UP * 0.35,
            buff=0, color=P_RED, stroke_width=4, max_tip_length_to_length_ratio=0.18,
        )
        loss_tag = math_label(r"\dot{Q}_{L}", size=LABEL_FONT_SIZE, color=P_RED)
        loss_tag.next_to(loss, UP, buff=0.06)
        self.play(FadeIn(open_win), Create(win_cross), FadeIn(win_eta), GrowArrow(loss), FadeIn(loss_tag),
                  *exchanger(0.9), run_time=0.9)
        self.play(FadeIn(ph_line, shift=UP * 0.1), *exchanger(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "passivhaus", during=exchanger)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat9 – Sorption-assisted cooling: engineering with material physics
class Beat9_SorptionsKuehlung(Scene):
    """🌞 Sorptionsgestützte Kühlung (DEC): Entfeuchten → Wärmetausch →
    Verdunsten, solar regeneriert — und ihr mikrobieller Preis.

    https://de.wikipedia.org/wiki/Sorptionsgestützte_Klimatisierung
    """

    NARRATION = [
        ("intro",
         "Material physics itself can cool without a chiller: sorption-assisted cooling chains drying, heat exchange and evaporation into one engine.",
         "Mit Stoffeigenschaften lässt sich ohne Kältemaschine kühlen: die sorptions-\ngestützte Kühlung verkettet Trocknen, Wärmetausch und Verdunstung."),
        ("entfeuchten",
         "A desiccant wheel first dries the humid outdoor air — adsorption releases heat, so the dry air leaves warmer than it came.",
         "Das Sorptionsrad entfeuchtet zuerst die schwüle Außenluft —\ndie Sorptionswärme macht die trockene Luft zunächst wärmer."),
        ("tauscher",
         "The plate exchanger hands that heat to the exhaust stream and precools the dry air.",
         "Der Plattentauscher gibt diese Wärme an die Abluft ab\nund kühlt die trockene Luft vor."),
        ("verdunsten",
         "Because the air is now dry, water may evaporate into it: evaporative cooling takes it down to supply temperature.",
         "Weil die Luft jetzt trocken ist, darf Wasser verdunsten:\nVerdunstungskühlung senkt sie auf Zulufttemperatur."),
        ("regeneration",
         "Solar heat regenerates the wheel: it drives the stored moisture out into the exhaust air, so the sun itself powers the cycle.",
         "Solarwärme regeneriert das Rad: sie treibt die gespeicherte\nFeuchte in die Fortluft aus — die Sonne treibt den Kreislauf."),
        ("nachteil",
         "The drawback is microbial. Harsh swings of temperature and humidity put the germs under pressure. They mutate, and the harmful ones are the ones that survive.",
         "Der Nachteil: krasse Temperatur- und Feuchtewechsel setzen Keime unter Druck.\nSie mutieren — es überleben bevorzugt die schädlichen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Sorptionsgestützte Kühlung (DEC)", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        y_sup, y_exh = 1.05, -0.25

        wheel_c = np.array([-2.9, 0.4, 0.0])
        wheel = Circle(radius=0.95, color=P_ORANGE, stroke_width=3).move_to(wheel_c)
        wheel_spokes = VGroup(*[
            Line(wheel_c, wheel_c + 0.95 * np.array([np.cos(a), np.sin(a), 0.0]),
                 color=P_ORANGE, stroke_width=1.4).set_stroke(opacity=0.5)
            for a in np.linspace(0, TAU, 8, endpoint=False)
        ])
        wheel_tag = Text("Sorptionsrad", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        wheel_tag.move_to(np.array([-2.9, -1.05, 0.0]))

        hx = Rectangle(width=1.5, height=1.9, color=P_WHITE, stroke_width=3).move_to(np.array([-0.3, 0.4, 0.0]))
        hx_diag = VGroup(
            Line(hx.get_corner(DL), hx.get_corner(UR), color=P_TEAL, stroke_width=1.6, stroke_opacity=0.7),
            Line(hx.get_corner(UL), hx.get_corner(DR), color=P_TEAL, stroke_width=1.6, stroke_opacity=0.7),
        )
        hx_tag = Text("Plattentauscher", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        hx_tag.move_to(np.array([-0.45, -1.05, 0.0]))

        def _humidifier(pos, color=P_BLUE):
            box = Rectangle(width=0.72, height=0.72, color=color, stroke_width=2.4).move_to(pos)
            return VGroup(box, droplets(pos, n=5, spread=(0.2, 0.2), seed=5, color=color))

        hum_sup = _humidifier(np.array([2.0, y_sup, 0.0]))
        hum_exh = _humidifier(np.array([2.0, y_exh, 0.0]))
        hum_tag = Text("Verdunstungskühlung", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        hum_tag.move_to(np.array([2.25, -1.05, 0.0]))

        heater_c = np.array([-1.5, y_exh, 0.0])
        heater = Rectangle(width=0.72, height=0.6, color=P_RED, stroke_width=2.4).move_to(heater_c)
        coil_pts = [heater_c + np.array([-0.26 + i * 0.087, 0.12 if i % 2 else -0.12, 0.0]) for i in range(7)]
        coil = VMobject(color=P_RED, stroke_width=2.2).set_points_as_corners(coil_pts)

        # Below the regeneration heater: the solar duct stays on the exhaust
        # side and never crosses the supply-air state readouts above.
        sun_c = np.array([-1.5, -1.5, 0.0])
        sun = sun_glyph(sun_c).scale(0.34)
        solar_duct = DashedLine(
            sun_c + UP * 0.36, heater.get_bottom() + DOWN * 0.04,
            color=P_YELLOW, stroke_width=2.4, dash_length=0.08,
        )
        solar_tag = Text("Solarwärme", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        solar_tag.next_to(sun, RIGHT, buff=0.18).shift(DOWN * 0.12)

        # Zustände entlang der Zuluft — jeder Wert ist das Ergebnis des Bauteils davor.
        t_reads = VGroup(
            math_label(r"32\,\mathrm{°C}\;\text{feucht}", at=np.array([-4.1, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_ORANGE),
            math_label(r"48\,\mathrm{°C}\;\text{trocken}", at=np.array([-1.5, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_RED),
            math_label(r"24\,\mathrm{°C}", at=np.array([0.95, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_YELLOW),
            math_label(r"19\,\mathrm{°C}", at=np.array([3.6, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_CYAN),
        )
        end_tags = VGroup(
            Text("Außenluft", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to([-5.9, y_sup + 0.55, 0.0]),
            Text("Zuluft", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to([5.0, y_sup + 0.55, 0.0]),
            Text("Abluft 26 °C", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to([5.0, y_exh - 0.62, 0.0]),
            Text("Fortluft\nfeucht · warm", font_size=LABEL_FONT_SIZE, color=P_ORANGE, line_spacing=0.8).move_to([-5.55, y_exh - 0.62, 0.0]),
        )

        def _lane(x0, x1, y):
            return VGroup(smooth_path([np.array([x0, y, 0.0]), np.array([(x0 + x1) / 2, y, 0.0]),
                                       np.array([x1, y, 0.0])]))

        sup_seg1 = _lane(-5.3, -2.0, y_sup)
        sup_seg2 = _lane(-2.0, 1.45, y_sup)
        sup_seg3 = _lane(1.45, 4.9, y_sup)
        exh_seg1 = _lane(4.9, 0.5, y_exh)
        exh_seg2 = _lane(0.5, -2.1, y_exh)
        exh_seg3 = _lane(-2.1, -5.3, y_exh)

        components = VGroup(
            wheel, wheel_spokes, wheel_tag, hx, hx_diag, hx_tag,
            hum_sup, hum_exh, hum_tag, heater, coil,
        )

        self.play(
            Create(wheel), Create(wheel_spokes), FadeIn(wheel_tag),
            Create(hx), Create(hx_diag), FadeIn(hx_tag),
            FadeIn(hum_sup), FadeIn(hum_exh), FadeIn(hum_tag),
            FadeIn(heater), Create(coil),
            Create(flow_guides(VGroup(*sup_seg1, *sup_seg2, *sup_seg3), P_CYAN, opacity=0.2)),
            Create(flow_guides(VGroup(*exh_seg1, *exh_seg2, *exh_seg3), P_TEAL, opacity=0.2)),
            FadeIn(end_tags[0]), FadeIn(end_tags[2]),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.6)

        def _spin(run_time):
            return Rotate(wheel_spokes, angle=-TAU * run_time / 5.2, about_point=wheel_c,
                          run_time=run_time, rate_func=linear)

        running = []

        def dec(rt):
            return [_streams(running, rt, speed=0.75, waves=5), _spin(rt)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "entfeuchten"))
        self.play(FadeIn(t_reads[0]), run_time=0.4)
        running.append((sup_seg1, P_ORANGE, P_RED))
        self.play(*dec(2.6), run_time=2.6)
        self.play(FadeIn(t_reads[1]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "entfeuchten", during=dec)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "tauscher"))
        heat_down = VGroup(*[
            Arrow(np.array([x, y_sup - 0.25, 0.0]), np.array([x, y_exh + 0.25, 0.0]),
                  buff=0, color=P_YELLOW, stroke_width=3, max_tip_length_to_length_ratio=0.3)
            for x in (-0.8, -0.3, 0.2)
        ])
        running.append((sup_seg2, P_RED, P_YELLOW))
        self.play(LaggedStart(*[GrowArrow(a) for a in heat_down], lag_ratio=0.3), *dec(2.4), run_time=2.4)
        self.play(FadeIn(t_reads[2]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "tauscher", during=dec)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verdunsten"))
        running.append((sup_seg3, P_YELLOW, P_CYAN))
        self.play(Indicate(hum_sup, color=P_BLUE, scale_factor=1.12), *dec(2.4), run_time=2.4)
        self.play(FadeIn(t_reads[3]), FadeIn(end_tags[1]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "verdunsten", during=dec)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "regeneration"))
        self.play(FadeIn(sun, scale=0.7), FadeIn(solar_tag), Create(solar_duct), *dec(1.0), run_time=1.0)
        running.extend([(exh_seg1, P_TEAL, P_BLUE), (exh_seg2, P_BLUE, P_ORANGE), (exh_seg3, P_ORANGE, P_RED)])
        self.play(
            heater.animate.set_fill(P_RED, opacity=0.25),
            Indicate(sun, color=P_YELLOW, scale_factor=1.1), *dec(3.0),
            run_time=3.0,
        )
        self.play(FadeIn(end_tags[3]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "regeneration", during=dec)

        # 🦠 Mikrobieller Preis: Hitze-/Feuchtezyklen selektieren resistente Keime.
        def _microbe(pos, color):
            c = np.array([pos[0], pos[1], 0.0], dtype=float)
            body = Circle(radius=0.11, color=color, stroke_width=2,
                          fill_color=color, fill_opacity=0.25).move_to(c)
            spikes = VGroup(*[
                Line(c + 0.11 * d, c + 0.18 * d, color=color, stroke_width=1.8)
                for a in np.linspace(0, TAU, 6, endpoint=False)
                for d in [np.array([np.cos(a), np.sin(a), 0.0])]
            ])
            return VGroup(body, spikes)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nachteil"))
        # Compact strip between the component labels (bottom ≈ −1.2) and the
        # caption box (top ≈ −2.6): every microbe and the verdict stay inside it.
        strip_y = -1.86
        panel = RoundedRectangle(
            width=10.0, height=1.0, corner_radius=0.12,
            color=P_RED, stroke_width=2.2, fill_color="#12151C", fill_opacity=0.94,
        ).move_to(np.array([0.0, strip_y, 0.0]))
        normal = VGroup(*[
            _microbe([-3.3 + i * 0.55, strip_y - 0.08], P_TEAL) for i in range(4)
        ])
        survivors = (normal[2], normal[3])
        cycle_tag = math_label(r"ΔT\!\uparrow\;\;Δx\!\uparrow", at=np.array([-4.75, strip_y - 0.08, 0.0]),
                               size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        verdict = Text("Mutation:\ndie Schädlichen überleben",
                       font_size=LABEL_FONT_SIZE, color=P_RED, line_spacing=0.8)
        verdict.move_to(np.array([2.9, strip_y, 0.0]))
        self.play(FadeOut(VGroup(sun, solar_tag, solar_duct)), *dec(0.4), run_time=0.4)
        self.play(FadeIn(panel), FadeIn(normal), FadeIn(cycle_tag), *dec(0.9), run_time=0.9)
        for flash in (P_RED, P_CYAN):
            self.play(panel.animate.set_stroke(color=flash), *dec(0.35), run_time=0.35)
        self.play(
            *[m.animate.scale(1.2).set_color(P_RED) for m in normal],
            *dec(1.0),
            run_time=1.0,
        )
        self.play(
            LaggedStart(*[FadeOut(m, scale=0.4) for m in normal[:2]], lag_ratio=0.15),
            *[m[0].animate.set_fill(opacity=0.65) for m in survivors], *dec(1.0),
            run_time=1.0,
        )
        clones = VGroup(_microbe([0.1, strip_y - 0.08], P_RED), _microbe([0.65, strip_y - 0.08], P_RED))
        self.play(FadeIn(clones, scale=0.5), FadeIn(verdict), *dec(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "nachteil", during=dec)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat10 – The operating rule: natural first, machines last
class Beat10_KomfortStrategie(Scene):
    NARRATION = [
        ("intro",
         "Put the levers together and they become one operating rule you can follow every day.",
         "Zusammen ergeben die Stellgrößen eine Betriebsregel, der man täglich folgen kann."),
        ("question",
         "Every decision starts with the same question: is the outdoor air cooler than the room?",
         "Am Anfang steht immer dieselbe Frage: ist die Außenluft kühler als der Raum?"),
        ("yes",
         "If it is, ventilate naturally. Set the opening, pick the cross or the stack path, and let outdoor air carry the heat away for free.",
         "Wenn ja: natürlich lüften — Öffnung einstellen, Quer- oder\nAuftriebsweg wählen, die Außenluft trägt die Wärme ab."),
        ("no",
         "If it is not, close and shade, and let the thermal mass buffer the day until the night window opens again.",
         "Wenn nein: schließen und verschatten — die Speichermasse\npuffert, bis das Nachtfenster wieder aufgeht."),
        ("reserve",
         "Only what these steps cannot cover goes to the balanced supply and exhaust system with heat recovery: the small remainder from the very first beat.",
         "Nur was diese Schritte nicht schaffen, geht an die\nZu-/Abluftanlage mit Wärmerückgewinnung."),
        ("close",
         "Comfort first, outdoor air second, machines last.",
         "Komfort zuerst, Außenluft danach, Technik zuletzt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Strategie: freie Kühlung zuerst", title)
        din = _din_ref("DIN EN 16798-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        start = chip("Sommertag", P_WHITE).move_to(np.array([0.0, 2.28, 0.0]))

        q_text = Text("Außenluft kühler\nals der Raum?", font_size=LABEL_FONT_SIZE,
                      color=P_YELLOW, line_spacing=0.7)
        diamond = Polygon(
            np.array([0.0, 0.66, 0.0]), np.array([2.6, 0.0, 0.0]),
            np.array([0.0, -0.66, 0.0]), np.array([-2.6, 0.0, 0.0]),
            color=P_YELLOW, stroke_width=2.2,
        )
        q_text.move_to(diamond.get_center())
        question = VGroup(diamond, q_text).move_to(np.array([0.0, 1.22, 0.0]))

        yes_box = chip("Natürlich lüften", P_CYAN).move_to(np.array([-3.45, 0.02, 0.0]))
        yes_sub = chip("Öffnung · Luftweg · Zeitplan", P_TEAL, font_size=LABEL_FONT_SIZE)
        yes_sub.move_to(np.array([-3.45, -0.78, 0.0]))
        no_box = chip("Schließen + verschatten", P_ORANGE).move_to(np.array([3.45, 0.02, 0.0]))
        no_sub = chip("Speichermasse puffert", P_TEAL, font_size=LABEL_FONT_SIZE)
        no_sub.move_to(np.array([3.45, -0.78, 0.0]))

        mech = chip("Reserve: Zu-/Abluft mit WRG", P_ORANGE).move_to(np.array([0.0, -1.60, 0.0]))
        goal = chip("Komfort", P_GREEN).move_to(np.array([0.0, -2.46, 0.0]))

        def _link(a, b, color=P_WHITE):
            return Arrow(a, b, buff=0.10, color=color, stroke_width=3,
                         max_tip_length_to_length_ratio=0.10)

        links = VGroup(
            _link(start.get_bottom(), question.get_top()),
            _link(diamond.get_left(), yes_box.get_top(), P_CYAN),
            _link(diamond.get_right(), no_box.get_top(), P_ORANGE),
            _link(yes_box.get_bottom(), yes_sub.get_top(), P_CYAN),
            _link(no_box.get_bottom(), no_sub.get_top(), P_ORANGE),
            _link(yes_sub.get_bottom(), mech.get_left(), P_TEAL),
            _link(no_sub.get_bottom(), mech.get_right(), P_TEAL),
            _link(mech.get_bottom(), goal.get_top(), P_GREEN),
        )
        # Inside the elbow of each link, away from the diamond corner where the token parks.
        yes_lbl = Text("ja", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(links[1].get_center(), RIGHT, buff=0.14)
        no_lbl = Text("nein", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(links[2].get_center(), LEFT, buff=0.14)

        chart = VGroup(start, question, yes_box, yes_sub, no_box, no_sub,
                       mech, goal, links, yes_lbl, no_lbl)
        # Inner band: clear of the beat subtitle above and the two-line caption box below.
        fit_band(chart, top=2.35, bottom=-2.42)

        token = Dot(radius=0.11, color=P_YELLOW, fill_opacity=1.0).move_to(_park(start))

        self.play(FadeIn(start), Create(question), run_time=1.2)
        self.play(
            LaggedStart(
                *[Create(link) for link in links],
                FadeIn(yes_box), FadeIn(no_box), FadeIn(yes_sub), FadeIn(no_sub),
                FadeIn(mech), FadeIn(goal), FadeIn(yes_lbl), FadeIn(no_lbl),
                lag_ratio=0.10,
            ),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 2.6)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "question"))
        self.play(FadeIn(token), run_time=0.4)
        self.play(token.animate.move_to(_park(question)), Indicate(question, color=P_YELLOW), run_time=1.1)
        hold_for(self, self.NARRATION, "question", used=0.4 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "yes"))
        self.play(
            token.animate.set_color(P_CYAN).move_to(_park(yes_box)),
            Indicate(yes_box, color=P_CYAN),
            *dim_chip(no_box, 0.3), *dim_chip(no_sub, 0.3),
            run_time=1.2,
        )
        self.play(token.animate.move_to(_park(yes_sub)), Indicate(yes_sub, color=P_TEAL), run_time=1.1)
        hold_for(self, self.NARRATION, "yes", used=1.2 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "no"))
        # Un-dim before Indicate, never alongside it: Indicate snapshots the starting
        # state and restores it, which silently undoes a same-play opacity change.
        self.play(
            *dim_chip(no_box, 1.0), *dim_chip(no_sub, 1.0),
            token.animate.set_color(P_ORANGE).move_to(_park(no_box)),
            run_time=0.9,
        )
        self.play(Indicate(no_box, color=P_ORANGE), run_time=0.7)
        self.play(token.animate.move_to(_park(no_sub)), Indicate(no_sub, color=P_TEAL), run_time=1.0)
        hold_for(self, self.NARRATION, "no", used=0.9 + 0.7 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "reserve"))
        self.play(token.animate.move_to(_park(mech)), Indicate(mech, color=P_ORANGE), run_time=1.2)
        hold_for(self, self.NARRATION, "reserve", used=1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "close"))
        self.play(
            token.animate.set_color(P_GREEN).move_to(_park(goal)),
            Indicate(goal, color=P_GREEN),
            run_time=1.2,
        )
        self.play(FadeOut(token), run_time=0.3)
        hold_for(self, self.NARRATION, "close", used=1.2 + 0.3 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
