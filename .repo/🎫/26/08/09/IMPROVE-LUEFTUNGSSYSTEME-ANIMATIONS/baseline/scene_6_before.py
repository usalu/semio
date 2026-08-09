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
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Natürliche Lüftung im Passivhaus"

CONTENT_CENTER = ORIGIN


#region Shared visual motifs

def _step_label(text, color=P_WHITE):
    """📌 Mid-lower instructional callout — kept clear of caption_bar."""
    return Text(text, font_size=BODY_FONT_SIZE, color=color).move_to(DOWN * 1.55)


def _room(center, width, height, color=P_WHITE):
    """🏠 Line-art room shell with a teal floor slab."""
    shell = Rectangle(
        width=width, height=height,
        color=color, stroke_width=3.0, fill_opacity=0,
    ).move_to(center)
    floor = Line(shell.get_corner(DL), shell.get_corner(DR), color=P_TEAL, stroke_width=4)
    return VGroup(shell, floor)


def _window(pos, height=1.15, width=0.24):
    """🪟 Facade opening — adjustable natural-ventilation aperture."""
    return Rectangle(
        width=width, height=height, color=P_CYAN, stroke_width=2.2,
        fill_color=P_DEEP_DARK, fill_opacity=1.0,
    ).move_to(pos)


def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Occupant glyph — the comfort target of the strategy."""
    head = Circle(radius=0.12, color=color, stroke_width=2.2)
    body = RoundedRectangle(
        width=0.32, height=0.44, corner_radius=0.1,
        color=color, stroke_width=2.2,
    ).next_to(head, DOWN, buff=0.04)
    return VGroup(head, body).scale(scale).move_to(pos)


def _badge(title, value, color=P_TEAL):
    """🏷️ Compact callout card for design levers / comfort states."""
    body = VGroup(
        Text(title, font_size=LABEL_FONT_SIZE, color=color),
        Text(value, font_size=BODY_FONT_SIZE, color=P_WHITE),
    ).arrange(DOWN, buff=0.06)
    box = SurroundingRectangle(body, color=color, corner_radius=0.1, buff=0.12, stroke_width=1.8)
    return VGroup(box, body)


def _smooth_path(points):
    """〰️ Smooth polyline path for airflow particles."""
    path = TipableVMobject()
    path.set_points_smoothly([np.array(p, dtype=float) for p in points])
    return path


def _guides(paths, color, opacity=0.25, width=2.0, tips=True):
    """🧭 Faint streamlines so the route of the air stays readable."""
    lines = VGroup()
    for path in paths:
        guide = path.copy().set_stroke(color=color, width=width, opacity=opacity)
        guide.set_fill(opacity=0)
        if tips:
            guide.add_tip(tip_length=0.17, tip_width=0.14)
            guide.tip.set_fill(color=color, opacity=opacity + 0.3)
            guide.tip.set_stroke(width=0)
        lines.add(guide)
    return lines


def _flow(
    scene, paths, color, run_time=3.2, waves=3, radius=0.075,
    cycles=2.0, color_end=None, extra=None,
):
    """💨 Continuous particle stream along natural-ventilation paths."""
    dots = VGroup()
    meta = []
    for path in paths:
        for w in range(waves):
            dot = Dot(radius=radius, color=color, stroke_width=0)
            dot.set_fill(color, opacity=0.0)
            dot.move_to(path.point_from_proportion(0.0))
            dots.add(dot)
            meta.append((path, w / waves))

    start_c = ManimColor(color)
    end_c = ManimColor(color_end) if color_end else start_c

    def update(group, alpha):
        for dot, (path, offset) in zip(group, meta):
            t = (alpha * cycles + offset) % 1.0
            dot.move_to(path.point_from_proportion(t))
            fade = min(1.0, t / 0.10, (1.0 - t) / 0.10)
            dot.set_fill(interpolate_color(start_c, end_c, t), opacity=max(0.0, fade))

    scene.add(dots)
    anims = [UpdateFromAlphaFunc(dots, update)]
    if extra:
        anims.extend(extra)
    scene.play(*anims, run_time=run_time, rate_func=linear)
    scene.remove(dots, *dots)

#endregion


#region Beat1 – Passive house: comfort without mechanical first
class Beat1_PassivhausIdee(Scene):
    NARRATION = [
        ("intro",
         "In a passive house, comfort is the goal — and the first tool is not a fan or a chiller.",
         "Im Passivhaus zählt Komfort — und das erste Werkzeug ist kein Ventilator."),
        ("envelope",
         "A highly insulated, airtight envelope already cuts heat gains and losses.",
         "Die gedämmte, dichte Hülle senkt Gewinne und Verluste bereits stark."),
        ("natural",
         "Then we adjust natural ventilation — openings, timing, and air paths — to keep people comfortable.",
         "Dann steuern wir natürliche Lüftung — Öffnungen, Timing, Luftwege — für Komfort."),
        ("priority",
         "Mechanical systems stay in reserve. Prefer free, adjustable natural flow whenever the climate allows.",
         "Technik bleibt Reserve — freie, einstellbare Lüftung hat Vorrang."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Komfort zuerst — ohne Mechanik wo möglich", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        house_c = LEFT * 2.6 + UP * 0.15
        facade = Rectangle(width=3.4, height=2.8, color=P_WHITE, stroke_width=3).move_to(house_c)
        roof = Polygon(
            facade.get_corner(UL) + LEFT * 0.15,
            facade.get_top() + UP * 0.85,
            facade.get_corner(UR) + RIGHT * 0.15,
            color=P_WHITE, stroke_width=3,
        )
        win = _window(house_c + UP * 0.15, height=1.0, width=1.2)
        ground = Line(house_c + LEFT * 2.1 + DOWN * 1.4, house_c + RIGHT * 2.1 + DOWN * 1.4,
                      color=P_TEAL, stroke_width=4)
        person = _person(house_c + DOWN * 0.55 + RIGHT * 0.15, color=P_ORANGE, scale=0.95)
        comfort = Text("Komfort", font_size=BODY_FONT_SIZE, color=P_GREEN)
        comfort.next_to(person, UP, buff=0.25)

        envelope = SurroundingRectangle(
            VGroup(facade, roof), color=P_TEAL, buff=0.18, corner_radius=0.08, stroke_width=3,
        )
        env_lbl = Text("Passivhaus-Hülle", font_size=BODY_FONT_SIZE, color=P_TEAL)
        env_lbl.next_to(envelope, UP, buff=0.18)

        levers = VGroup(
            _badge("Öffnungsweite", "einstellbar", P_CYAN),
            _badge("Zeitfenster", "Tag / Nacht", P_BLUE),
            _badge("Luftweg", "Quer / Auftrieb", P_GREEN),
        ).arrange(DOWN, buff=0.22)
        levers.move_to(RIGHT * 3.2 + UP * 0.35)

        mech = VGroup(
            Text("Mechanik", font_size=BODY_FONT_SIZE, color=P_ORANGE),
            Text("nur wenn nötig", font_size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(DOWN, buff=0.06)
        mech_box = SurroundingRectangle(mech, color=P_ORANGE, corner_radius=0.1, buff=0.14, stroke_width=1.8)
        mech_card = VGroup(mech_box, mech).next_to(levers, DOWN, buff=0.35)

        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(ground), Create(facade), Create(roof), run_time=1.4)
        self.play(FadeIn(win), FadeIn(person), FadeIn(comfort), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "envelope"))
        self.play(Create(envelope), FadeIn(env_lbl), run_time=1.2)
        hold_for(self, self.NARRATION, "envelope", used=1.4 + 1.0 + 1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "natural"))
        self.play(LaggedStart(*[FadeIn(card, shift=LEFT * 0.2) for card in levers], lag_ratio=0.2), run_time=1.6)
        hold_for(self, self.NARRATION, "natural", used=1.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "priority"))
        self.play(FadeIn(mech_card), run_time=0.9)
        self.play(Indicate(levers, color=P_CYAN), run_time=1.0)
        hold_for(self, self.NARRATION, "priority", used=0.9 + 1.0 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Adjust window openings for comfort
class Beat2_Fensterregeln(Scene):
    NARRATION = [
        ("intro",
         "Natural ventilation is adjustable. Start with a single facade opening and tune it for comfort.",
         "Natürliche Lüftung ist einstellbar — beginne mit einer Öffnung und justiere."),
        ("closed",
         "Fully closed traps heat and stale air — comfort collapses.",
         "Geschlossen: Wärme und Abluft stauen sich — Komfort bricht ein."),
        ("partial",
         "A partial opening lets a gentle exchange: enough fresh air without a draft.",
         "Teilweise geöffnet: sanfter Austausch ohne Zugluft."),
        ("timed",
         "Open wider when outdoor air is cooler than the room, and throttle back when it is hotter.",
         "Weiter öffnen, wenn es draußen kühler ist — drosseln, wenn es heißer ist."),
        ("rule",
         "The design rule: treat the window as a valve you set for people, not as an all-or-nothing switch.",
         "Regel: das Fenster ist ein Ventil für Menschen — kein Alles-oder-nichts."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Fenster als Ventil für Komfort", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        center = CONTENT_CENTER + LEFT * 0.4
        shell = _room(center, 7.6, 3.2)
        room, floor = shell[0], shell[1]
        wash = Rectangle(
            width=7.5, height=3.1, stroke_width=0,
            fill_color=P_RED, fill_opacity=0.0,
        ).move_to(center)
        person = _person(center + DOWN * 0.35, color=P_ORANGE)

        win_full = _window(room.get_left() + UP * 0.25, height=1.25, width=0.28)
        win_partial = Rectangle(
            width=0.28, height=0.55, color=P_CYAN, stroke_width=2.2,
            fill_color=P_DEEP_DARK, fill_opacity=1.0,
        ).move_to(room.get_left() + UP * 0.05)
        shutter = Rectangle(
            width=0.28, height=0.7, color=P_TEAL, stroke_width=2.0,
            fill_color=P_TEAL, fill_opacity=0.35,
        ).move_to(room.get_left() + UP * 0.72)

        closed_tag = Text("zu", font_size=BODY_FONT_SIZE, color=P_RED).next_to(win_full, LEFT, buff=0.3)
        open_tag = Text("teilweise", font_size=BODY_FONT_SIZE, color=P_CYAN)
        open_tag.next_to(win_partial, LEFT, buff=0.25)

        gentle = VGroup(
            _smooth_path([
                room.get_left() + DOWN * 0.2,
                center + LEFT * 2.2 + DOWN * 0.5,
                center + LEFT * 1.0 + DOWN * 0.25,
            ]),
            _smooth_path([
                center + LEFT * 1.0 + UP * 0.15,
                center + LEFT * 2.2 + UP * 0.55,
                room.get_left() + UP * 0.55,
            ]),
        )

        outdoor = _badge("Außenluft", "kühler als Raum", P_BLUE)
        outdoor.move_to(LEFT * 5.3 + UP * 1.2)
        indoor = _badge("Raum", "Komfortzone", P_GREEN)
        indoor.next_to(person, RIGHT, buff=0.55)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(room), Create(floor), FadeIn(person), run_time=1.3)
        self.add(wash)
        self.play(FadeIn(win_full), FadeIn(closed_tag), wash.animate.set_fill(opacity=0.22), run_time=1.1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "closed"))
        hold_for(self, self.NARRATION, "closed", used=1.3 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "partial"))
        self.play(
            ReplacementTransform(win_full, win_partial),
            FadeIn(shutter),
            ReplacementTransform(closed_tag, open_tag),
            wash.animate.set_fill(P_CYAN, opacity=0.08),
            run_time=1.4,
        )
        g = _guides(gentle, P_CYAN)
        self.play(Create(g), run_time=0.6)
        _flow(self, gentle, P_CYAN, run_time=3.0, waves=3, cycles=2.0)
        hold_for(self, self.NARRATION, "partial", used=1.4 + 0.6 + 3.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "timed"))
        self.play(FadeIn(outdoor), FadeIn(indoor), run_time=1.0)
        self.play(win_partial.animate.stretch_to_fit_height(0.95).shift(UP * 0.2),
                  shutter.animate.shift(UP * 0.35).stretch_to_fit_height(0.35), run_time=1.2)
        _flow(self, gentle, P_BLUE, run_time=2.6, waves=4, cycles=2.2)
        hold_for(self, self.NARRATION, "timed", used=1.0 + 1.2 + 2.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rule"))
        rule = Text("Fenster = einstellbares Ventil", font_size=FORMULA_FONT_SIZE, color=P_YELLOW)
        rule.move_to(DOWN * 1.55)
        self.play(FadeIn(rule), run_time=0.8)
        hold_for(self, self.NARRATION, "rule", used=0.8 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Cross ventilation as a comfort strategy
class Beat3_Querlueftung(Scene):
    NARRATION = [
        ("intro",
         "For stronger, still-natural cooling, open opposite facades and create cross ventilation.",
         "Für stärkeren, natürlichen Kühlzug: gegenüberliegende Fassaden öffnen."),
        ("path",
         "Wind pressure drives outdoor air straight through the occupied zone.",
         "Winddruck treibt Außenluft durch die Aufenthaltszone."),
        ("adjust",
         "Comfort control means sizing both openings — not only one side — so the through-flow feels pleasant, not drafty.",
         "Komfort heißt: beide Öffnungen dosieren — angenehm, nicht zugig."),
        ("passive",
         "In passive houses this is a summer staple: flush heat with outdoor air instead of starting mechanical cooling.",
         "Im Passivhaus Standard im Sommer: Wärme mit Außenluft spülen statt Kälteanlage."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Querlüftung dosieren", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        center = CONTENT_CENTER
        shell = _room(center, 8.0, 3.2)
        room, floor = shell[0], shell[1]
        left_win = _window(room.get_left() + UP * 0.2, height=1.2)
        right_win = _window(room.get_right() + UP * 0.2, height=1.2)
        person = _person(center + DOWN * 0.25)

        cross = VGroup(*[
            _smooth_path([
                room.get_left() + UP * dy,
                center + LEFT * 1.5 + UP * (dy * 0.7),
                center + RIGHT * 1.5 + UP * (dy * 0.5 + 0.1),
                room.get_right() + UP * (dy * 0.55 + 0.15),
            ])
            for dy in (-0.25, 0.25, 0.7)
        ])

        inlet = Text("Zuluftöffnung", font_size=BODY_FONT_SIZE, color=P_CYAN)
        inlet.next_to(left_win, LEFT, buff=0.25)
        outlet = Text("Abluftöffnung", font_size=BODY_FONT_SIZE, color=P_GREEN)
        outlet.next_to(right_win, RIGHT, buff=0.25)
        dial = _badge("Öffnungsanteil", "beide Seiten abstimmen", P_YELLOW)
        dial.move_to(DOWN * 1.55)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(room), Create(floor), FadeIn(person), run_time=1.3)
        self.play(FadeIn(left_win), FadeIn(right_win), FadeIn(inlet), FadeIn(outlet), run_time=1.1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "path"))
        g = _guides(cross, P_GREEN)
        self.play(Create(g), run_time=0.7)
        _flow(self, cross, P_GREEN, run_time=3.2, waves=4, cycles=2.4)
        hold_for(self, self.NARRATION, "path", used=1.3 + 1.1 + 0.7 + 3.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "adjust"))
        self.play(FadeIn(dial), run_time=0.8)
        self.play(
            left_win.animate.stretch_to_fit_height(0.75),
            right_win.animate.stretch_to_fit_height(1.35),
            run_time=1.3,
        )
        _flow(self, cross, P_CYAN, run_time=2.8, waves=3, cycles=2.0)
        hold_for(self, self.NARRATION, "adjust", used=0.8 + 1.3 + 2.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "passive"))
        tip = Text("Passivhaus: natürliche Spülung vor mechanischer Kühlung",
                   font_size=BODY_FONT_SIZE, color=P_TEAL)
        tip.move_to(UP * 1.85)
        self.play(FadeIn(tip), run_time=0.9)
        hold_for(self, self.NARRATION, "passive", used=0.9 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Stack effect for passive houses
class Beat4_Auftrieb(Scene):
    NARRATION = [
        ("intro",
         "When wind is weak, buoyancy still works: warm indoor air rises and leaves high, cool air enters low.",
         "Bei wenig Wind hilft Auftrieb: warme Luft steigt ab, kühle Luft nach."),
        ("shaft",
         "A stairwell or dedicated shaft amplifies the height difference that drives the flow.",
         "Treppenhaus oder Schacht vergrößert die treibende Höhendifferenz."),
        ("formula",
         "The driving pressure equals shaft height times gravity times the density difference between outdoor and indoor air.",
         "Δp = h · g · (ρ_a − ρ_i)."),
        ("h",
         "h is the vertical distance between inlet and outlet — taller means stronger natural draft.",
         "h ist die Höhe zwischen Ein- und Auslass — größer heißt stärkerer Zug."),
        ("comfort",
         "Designers place low inlets near people and high outlets at the roof — comfort without a fan.",
         "Tiefe Zulüfte bei Menschen, hohe Abluft am Dach — Komfort ohne Ventilator."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Auftrieb nutzen", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        center = CONTENT_CENTER + LEFT * 0.6
        shell = _room(center, 6.4, 3.0)
        room, floor = shell[0], shell[1]
        low_win = _window(room.get_left() + DOWN * 0.35, height=0.85)
        person = _person(center + DOWN * 0.45 + LEFT * 0.8)

        shaft_w, shaft_h = 0.95, 1.25
        shaft_x = room.get_corner(UR)[0] - 0.55
        shaft_top = room.get_top()[1] + shaft_h
        shaft = VGroup(*[
            Line(
                np.array([shaft_x - shaft_w / 2, room.get_top()[1] - 0.2, 0.0]),
                np.array([shaft_x - shaft_w / 2, shaft_top, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            ),
            Line(
                np.array([shaft_x + shaft_w / 2, room.get_top()[1] - 0.2, 0.0]),
                np.array([shaft_x + shaft_w / 2, shaft_top, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            ),
        ])
        stack = VGroup(*[
            _smooth_path([
                room.get_left() + DOWN * 0.4 + RIGHT * 0.05,
                center + LEFT * 0.2 + DOWN * 0.2,
                np.array([shaft_x, room.get_top()[1] + 0.15, 0.0]),
                np.array([shaft_x, shaft_top - 0.05, 0.0]),
            ])
            for _ in range(3)
        ])

        eq, items = equation_row([
            ("dp", "Δp", P_ORANGE), (None, "=", P_WHITE),
            ("h", "h", P_YELLOW), (None, "·", P_WHITE),
            (None, "g", P_WHITE), (None, "·", P_WHITE),
            (None, "(ρ_a − ρ_i)", P_WHITE),
            (None, "  [Pa]", P_TEAL),
        ])

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(room), Create(floor), FadeIn(low_win), FadeIn(person), run_time=1.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "shaft"))
        self.play(Create(shaft), run_time=1.1)
        g = _guides(stack, P_ORANGE)
        self.play(Create(g), run_time=0.6)
        _flow(self, stack, P_ORANGE, run_time=3.0, waves=3, cycles=2.1)
        hold_for(self, self.NARRATION, "shaft", used=1.4 + 1.1 + 0.6 + 3.0 + 0.35)

        eq, box = formula_panel(eq, color=P_ORANGE)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq), Create(box), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

        ring = highlight_param(items, "h", color=P_YELLOW)
        h_line = Line(
            np.array([shaft_x + 0.7, room.get_bottom()[1] + 0.5, 0.0]),
            np.array([shaft_x + 0.7, shaft_top, 0.0]),
            color=P_YELLOW, stroke_width=3,
        )
        h_lbl = Text("h", font_size=FORMULA_FONT_SIZE, color=P_YELLOW).next_to(h_line, RIGHT, buff=0.12)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "h"))
        self.play(Create(ring), Create(h_line), FadeIn(h_lbl), run_time=0.9)
        hold_for(self, self.NARRATION, "h", used=0.9 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "comfort"))
        tip = Text("tiefe Zuluft · hohe Abluft — ohne Ventilator", font_size=BODY_FONT_SIZE, color=P_TEAL)
        tip.move_to(DOWN * 1.55)
        self.play(FadeIn(tip), Indicate(person, color=P_ORANGE), run_time=1.1)
        hold_for(self, self.NARRATION, "comfort", used=1.1 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Night purge for summer comfort
class Beat5_Nachtlueftung(Scene):
    NARRATION = [
        ("intro",
         "Summer comfort in passive houses leans on night ventilation: open when the outdoor air is cool.",
         "Sommerkomfort im Passivhaus: Nachtlüftung — öffnen, wenn die Außenluft kühl ist."),
        ("store",
         "By day the structure stores heat. By night, cool air flushes that heat out of the thermal mass.",
         "Tags speichert die Masse Wärme — nachts spült kühle Luft sie wieder aus."),
        ("cool",
         "Watch the stored temperature fall. The building starts the next morning cooler — without mechanical cooling.",
         "Die Speichertemperatur sinkt — der Morgen beginnt kühler, ohne Kältemaschine."),
        ("schedule",
         "The adjustment is a schedule: open wide at night, close or shade by day when outdoor air is hotter than the room.",
         "Die Einstellung ist ein Zeitplan: nachts weit, tags zu oder schattieren."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Nachtlüftung für Sommerkomfort", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        center = CONTENT_CENTER
        shell = _room(center, 8.0, 3.2)
        room, floor = shell[0], shell[1]
        left_win = _window(room.get_left() + UP * 0.15)
        right_win = _window(room.get_right() + UP * 0.15)
        heat = Rectangle(
            width=7.9, height=3.1, stroke_width=0,
            fill_color=P_RED, fill_opacity=0.18,
        ).move_to(center)

        mass_tag = Text("Speichermasse", font_size=BODY_FONT_SIZE, color=P_RED)
        mass_tag.move_to(center + DOWN * 1.05)
        mass_temp = Text("26 °C", font_size=FORMULA_FONT_SIZE, color=P_RED)
        mass_temp.next_to(mass_tag, DOWN, buff=0.12)
        mass_tag_c = Text("Speichermasse", font_size=BODY_FONT_SIZE, color=P_BLUE).move_to(mass_tag)
        mass_temp_c = Text("21 °C", font_size=FORMULA_FONT_SIZE, color=P_BLUE).move_to(mass_temp)

        night = VGroup(*[
            _smooth_path([
                room.get_left() + UP * dy,
                center + LEFT * 2.0 + UP * (dy * 0.55 - 0.1),
                center + RIGHT * 2.0 + UP * (dy * 0.45 + 0.1),
                room.get_right() + UP * (dy * 0.5 + 0.2),
            ])
            for dy in (-0.7, -0.05, 0.55)
        ])
        in_tag = Text("Nachtluft 16 °C", font_size=BODY_FONT_SIZE, color=P_BLUE)
        in_tag.next_to(room, LEFT, buff=0.22)
        out_tag = Text("wärmere Abluft", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        out_tag.next_to(room, RIGHT, buff=0.22)

        schedule = VGroup(
            _badge("Nacht", "weit öffnen", P_BLUE),
            _badge("Tag", "schließen / schatten", P_ORANGE),
        ).arrange(RIGHT, buff=0.35)
        schedule.move_to(DOWN * 1.55)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(room), Create(floor), FadeIn(left_win), FadeIn(right_win), run_time=1.4)
        self.add(heat)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "store"))
        self.play(FadeIn(mass_tag), FadeIn(mass_temp), run_time=0.9)
        hold_for(self, self.NARRATION, "store", used=1.4 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cool"))
        g = _guides(night, P_BLUE)
        self.play(Create(g), FadeIn(in_tag), FadeIn(out_tag), run_time=1.0)
        _flow(
            self, night, P_BLUE, run_time=3.2, waves=4, cycles=2.4, color_end=P_ORANGE,
            extra=[
                heat.animate.set_fill(P_BLUE, opacity=0.10),
                ReplacementTransform(mass_temp, mass_temp_c),
                ReplacementTransform(mass_tag, mass_tag_c),
                room.animate.set_stroke(color=P_BLUE),
            ],
        )
        hold_for(self, self.NARRATION, "cool", used=1.0 + 3.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "schedule"))
        self.play(FadeIn(schedule), run_time=1.0)
        hold_for(self, self.NARRATION, "schedule", used=1.0 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Strategy summary: natural first
class Beat6_KomfortStrategie(Scene):
    NARRATION = [
        ("intro",
         "Put the levers together: passive-house comfort is mostly an operations and openings problem.",
         "Zusammengefasst: Passivhaus-Komfort ist vor allem Öffnen und Betreiben."),
        ("levers",
         "Adjust opening size, choose cross or stack paths, and schedule night purge against outdoor temperature.",
         "Öffnungsweite, Quer- oder Auftriebspfad, Nachtlüftung nach Außentemperatur."),
        ("goal",
         "Keep occupants in the comfort band with outdoor air — free and quiet.",
         "Menschen in der Komfortzone halten — mit Außenluft, gratis und leise."),
        ("reserve",
         "Only if natural strategies cannot meet the load do we escalate to mechanical ventilation or cooling.",
         "Nur wenn natürliche Strategien nicht reichen, greifen wir zur Mechanik."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Strategie: natürlich zuerst", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        cards = VGroup(
            _badge("1  Öffnung", "weiten / drosseln", P_CYAN),
            _badge("2  Luftweg", "Quer · Auftrieb", P_GREEN),
            _badge("3  Zeitplan", "Nacht vor Tag", P_BLUE),
            _badge("4  Komfort", "Zielgröße Mensch", P_ORANGE),
        ).arrange_in_grid(rows=2, cols=2, buff=(0.45, 0.35))
        cards.move_to(UP * 0.55)

        ladder = VGroup(
            Text("1  Natürliche Lüftung einstellen", font_size=BODY_FONT_SIZE, color=P_CYAN),
            Text("2  Speichermasse nachts entladen", font_size=BODY_FONT_SIZE, color=P_BLUE),
            Text("3  Mechanik nur als Reserve", font_size=BODY_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.22)
        ladder_box = SurroundingRectangle(ladder, color=P_TEAL, corner_radius=0.12, buff=0.22, stroke_width=2)
        ladder_group = VGroup(ladder_box, ladder).move_to(DOWN * 0.15)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(LaggedStart(*[FadeIn(c, shift=UP * 0.15) for c in cards], lag_ratio=0.18), run_time=1.8)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "levers"))
        hold_for(self, self.NARRATION, "levers", used=1.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "goal"))
        self.play(Indicate(cards[3], color=P_ORANGE), run_time=1.0)
        hold_for(self, self.NARRATION, "goal", used=1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "reserve"))
        self.play(FadeOut(cards), run_time=0.6)
        self.play(FadeIn(ladder_group), run_time=1.1)
        self.play(Indicate(ladder[0], color=P_CYAN), Indicate(ladder[2], color=P_ORANGE), run_time=1.2)
        hold_for(self, self.NARRATION, "reserve", used=0.6 + 1.1 + 1.2 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
