"""🔥 Heating Module 2 — Wärmeleitung (Conduction).

Migrated from ``merged_scenes_german.py`` onto the generate-manim-tutorial
template: fixed type scale, ``formula_panel`` with units, German ``caption_bar``
subtitles, and ``hold_for`` timing. Core animations are preserved; content is
shifted up so it never collides with the formula / caption zones.
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
    SUBTITLE_FONT_SIZE, BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 2: Wärmeleitung"


#region Beat helpers
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Part 2 rests on the same norms Module 1's conduction beats cite, so this
    mirrors ``_din_ref`` in ``1_introduction/scene_1.py`` exactly (same size,
    colour, opacity, corner): DIN EN ISO 6946 for the R- and U-value of a
    building component, DIN EN ISO 13789 for the envelope-wide transmission
    coefficient, DIN 4108-2 for the envelope insulation requirement the
    air-pocket beat is really about. Dim so it reads as a footnote, never
    competing with the diagram. The formula panel sits on the bottom edge, so
    this corner is clear in every beat.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


class Beat1_MakroUndMikro(Scene):
    NARRATION = [
        ("macro",
         "Macroscopically, heat flows from a hot block into a cold one until both feel equally warm.",
         "Makroskopisch fließt Wärme vom heißen Block in den kalten — bis beide gleich warm wirken."),
        ("equilibrium",
         "That shared warmth is thermal equilibrium — the net flow has stopped.",
         "Diese gemeinsame Wärme ist das thermische Gleichgewicht — der Nettostrom ist null."),
        ("micro",
         "Zoom in: a lattice of molecules. Heat is just kinetic vibration hopping neighbor to neighbor.",
         "Hereinzoomen: ein Molekülgitter. Wärme ist nur kinetische Schwingung von Nachbar zu Nachbar."),
        ("insulation",
         "An insulation layer full of air pockets breaks that chain — conduction nearly stops.",
         "Eine Dämmschicht mit Lufteinschlüssen unterbricht die Kette — die Leitung stoppt fast."),
        ("blocked",
         "Air pockets stop conduction, exactly as DIN 4108 intends for building envelopes.",
         "Lufteinschlüsse stoppen die Wärmeleitung — genau so meint es DIN 4108 für die Gebäudehülle."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Makroskopisch und mikroskopisch", title)
        din = _din_ref("DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "macro"))
        self.play(FadeIn(caption), run_time=0.3)

        left_block = Rectangle(
            width=2.5, height=2.2, fill_color=P_RED, fill_opacity=0.85,
            stroke_color=P_WHITE, stroke_width=2,
        ).move_to(LEFT * 1.25 + UP * 0.15)
        right_block = Rectangle(
            width=2.5, height=2.2, fill_color=P_BLUE, fill_opacity=0.85,
            stroke_color=P_WHITE, stroke_width=2,
        ).move_to(RIGHT * 1.25 + UP * 0.15)
        hot_label = Text("HEISS", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(left_block.get_center())
        cold_label = Text("KALT", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(right_block.get_center())

        self.play(
            FadeIn(left_block, shift=RIGHT * 0.3),
            FadeIn(right_block, shift=LEFT * 0.3),
            FadeIn(hot_label), FadeIn(cold_label),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "macro", used=1.2 + 0.35)

        eq_label = Text("WARM (GLEICHGEWICHT)", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(UP * 0.15)
        # A single outer border replaces the two blocks' own strokes: left and
        # right otherwise keep their shared inner edge, which at equilibrium
        # sits exactly where the merged label is centred and cuts through it.
        merged_outline = Rectangle(
            width=5.0, height=2.2, stroke_color=P_WHITE, stroke_width=2, fill_opacity=0,
        ).move_to(UP * 0.15)
        self.play(
            left_block.animate.set_fill("#9B51E0").set_stroke(width=0),
            right_block.animate.set_fill("#9B51E0").set_stroke(width=0),
            FadeIn(merged_outline),
            ReplacementTransform(VGroup(hot_label, cold_label), eq_label),
            run_time=1.5,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "equilibrium"))
        hold_for(self, self.NARRATION, "equilibrium", used=1.5 + 0.35)

        micro_sub = beat_subtitle("Mikroskopisches Gitter & Wärmebarriere", title)
        self.play(
            FadeOut(left_block), FadeOut(right_block), FadeOut(eq_label), FadeOut(merged_outline),
            ReplacementTransform(subtitle, micro_sub),
            run_time=1.0,
        )
        subtitle = micro_sub

        num_rows, num_cols = 7, 12
        start_x, start_y = -4.2, 1.15
        dx, dy = 0.65, 0.38
        insulation_x = 0.68

        dots = []
        dots_group = VGroup()
        phases = {}
        for r in range(num_rows):
            for c in range(num_cols):
                pos = np.array([start_x + c * dx, start_y - r * dy, 0.0])
                dot = Dot(point=pos, radius=0.08, color=P_BLUE)
                dot.init_pos = pos.copy()
                dot.is_hot = False
                dots.append(dot)
                dots_group.add(dot)
                phases[dot] = (r * 1.3 + c * 2.7)

        time_tracker = ValueTracker(0)
        time_tracker.add_updater(lambda m, dt: m.increment_value(dt))
        self.add(time_tracker)

        for dot in dots:
            def make_updater(d):
                p = phases[d]

                def updater(mob):
                    t = time_tracker.get_value()
                    amp, freq = (0.07, 24.0) if d.is_hot else (0.012, 8.0)
                    offset = np.array([
                        amp * np.sin(freq * t + p),
                        amp * np.cos(freq * t * 1.3 + p * 0.7),
                        0.0,
                    ])
                    mob.move_to(d.init_pos + offset)
                return updater

            dot.add_updater(make_updater(dot))

        self.play(FadeIn(dots_group), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "micro"))
        hold_for(self, self.NARRATION, "micro", used=1.0 + 0.35)

        barrier_rect = Rectangle(
            width=0.45, height=num_rows * dy + 0.25,
            fill_color=P_YELLOW, fill_opacity=0.2,
            stroke_color=P_YELLOW, stroke_width=2,
        ).move_to(np.array([insulation_x, start_y - (num_rows - 1) * dy / 2, 0.0]))
        air_gaps = VGroup(*[
            Circle(radius=0.07, fill_color="#0B0C10", fill_opacity=1.0, stroke_color=P_YELLOW, stroke_width=1.5)
            .move_to(np.array([insulation_x, y_c, 0.0]))
            for y_c in np.linspace(barrier_rect.get_bottom()[1] + 0.25, barrier_rect.get_top()[1] - 0.25, 5)
        ])
        ins_text = Text("Dämmschicht (DIN 4108)", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        ins_text.next_to(barrier_rect, UP, buff=0.12)
        insulation_barrier = VGroup(barrier_rect, air_gaps, ins_text)
        self.play(Create(insulation_barrier), run_time=1.1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "insulation"))
        hold_for(self, self.NARRATION, "insulation", used=1.1 + 0.35)

        sweep_x = ValueTracker(start_x - 0.5)
        sweep_line = Line(
            np.array([start_x - 0.5, start_y + 0.25, 0.0]),
            np.array([start_x - 0.5, start_y - (num_rows - 1) * dy - 0.2, 0.0]),
            color=P_ORANGE, stroke_width=5,
        )
        sweep_line.add_updater(lambda m: m.set_x(sweep_x.get_value()))
        self.add(sweep_line)

        def heat_propagation_updater(dt):
            x_val = sweep_x.get_value()
            for d in dots:
                if d.init_pos[0] <= x_val and d.init_pos[0] < insulation_x - 0.25 and not d.is_hot:
                    d.is_hot = True
                    d.set_color(P_ORANGE)

        self.add_updater(heat_propagation_updater)
        self.play(sweep_x.animate.set_value(insulation_x - 0.22), run_time=3.6, rate_func=linear)
        self.remove_updater(heat_propagation_updater)
        sweep_line.clear_updaters()

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "blocked"))
        hold_for(self, self.NARRATION, "blocked", used=3.6 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat2_RWert(Scene):
    NARRATION = [
        ("intro",
         "Zoom back out: that insulating slab has a thermal resistance R.",
         "Herauszoomen: diese Dämmplatte hat einen Wärmedurchlasswiderstand R."),
        ("formula",
         "R equals thickness d over conductivity lambda — unit square-meter kelvin per watt.",
         "R ist Dicke d geteilt durch Leitfähigkeit Lambda — Einheit Quadratmeter Kelvin pro Watt."),
        ("d",
         "d is the layer thickness in meters.",
         "d ist die Schichtdicke in Metern."),
        ("lam",
         "Lambda is thermal conductivity in watts per meter and kelvin, from DIN 4108.",
         "Lambda ist die Wärmeleitfähigkeit in Watt pro Meter und Kelvin, nach DIN 4108."),
        ("stretch",
         "Double the thickness and R doubles — resistance scales linearly with d.",
         "Verdoppelt man die Dicke, verdoppelt sich R — der Widerstand skaliert linear mit d."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmedurchlasswiderstand R", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        grid_dots = VGroup(*[
            Dot(point=[x * 0.35, y * 0.35, 0], radius=0.07, color=P_CYAN)
            for x in range(-2, 3) for y in range(-2, 3)
        ]).move_to(UP * 0.55)

        self.play(FadeIn(grid_dots, lag_ratio=0.03), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=1.2 + 0.3)

        rect = Rectangle(
            width=1.5, height=1.9, color=P_YELLOW,
            fill_color="#C59B27", fill_opacity=0.85,
        ).move_to(UP * 0.55)
        rect_label = Text("Dämmung", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(rect.get_center())
        self.play(ReplacementTransform(grid_dots, rect), FadeIn(rect_label), run_time=1.6)

        row, items = equation_row([
            ("r", "R", P_TEAL), (None, "=", P_WHITE),
            ("d", "d", P_ORANGE), (None, "/", P_WHITE),
            ("lam", "λ", P_CYAN),
            (None, "  [m²·K/W]", P_WHITE),
        ])
        row, box = formula_panel(row)
        self.play(FadeIn(row), Create(box), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        hold_for(self, self.NARRATION, "formula", used=2.6 + 0.35)

        r_var = ValueTracker(2.0)
        brace = Brace(rect, DOWN, buff=0.12, color=P_WHITE)
        d_label = Text("d [m]", font_size=BODY_FONT_SIZE, color=P_ORANGE).next_to(brace, DOWN, buff=0.08)
        r_label = Text("R = 2.0 m²·K/W", font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(rect, UP, buff=0.18)

        ring_d = highlight_param(items, "d", color=P_ORANGE)
        self.play(GrowFromCenter(brace), FadeIn(d_label), FadeIn(r_label), Create(ring_d), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "d"))
        hold_for(self, self.NARRATION, "d", used=1.0 + 0.35)
        self.play(FadeOut(ring_d), run_time=0.2)

        ring_l = highlight_param(items, "lam", color=P_CYAN)
        self.play(Create(ring_l), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lam"))
        hold_for(self, self.NARRATION, "lam", used=0.45 + 0.35)
        self.play(FadeOut(ring_l), run_time=0.2)

        brace.add_updater(lambda b: b.become(Brace(rect, DOWN, buff=0.12, color=P_WHITE)))
        d_label.add_updater(lambda t: t.next_to(brace, DOWN, buff=0.08))
        r_label.add_updater(
            lambda t: t.become(
                Text(f"R = {r_var.get_value():.1f} m²·K/W", font_size=BODY_FONT_SIZE, color=P_YELLOW)
                .next_to(rect, UP, buff=0.18)
            )
        )
        rect_label.add_updater(lambda t: t.move_to(rect.get_center()))

        self.play(
            rect.animate.stretch_to_fit_width(3.0),
            r_var.animate.set_value(4.0),
            run_time=3.2,
            rate_func=smooth,
        )
        brace.clear_updaters()
        d_label.clear_updaters()
        r_label.clear_updaters()
        rect_label.clear_updaters()

        highlight_box = SurroundingRectangle(r_label, color=P_YELLOW, buff=0.08)
        double_text = Text("2× Dicke → 2× R", font_size=BODY_FONT_SIZE, color=P_GREEN)
        double_text.next_to(rect, RIGHT, buff=0.3)
        self.play(Create(highlight_box), FadeIn(double_text), run_time=1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "stretch"))
        hold_for(self, self.NARRATION, "stretch", used=4.2 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat3_UWertUndGradient(Scene):
    NARRATION = [
        ("u_intro",
         "Flip resistance and you get the U-value — heat flow per area and kelvin.",
         "Den Widerstand umdrehen ergibt den U-Wert — Wärmestrom pro Fläche und Kelvin."),
        ("u_formula",
         "U equals one over R, in watts per square meter and kelvin.",
         "U ist eins durch R, in Watt pro Quadratmeter und Kelvin."),
        ("area",
         "Heat loss also scales with surface area A — more facade, more watts.",
         "Der Wärmeverlust skaliert auch mit der Fläche A — mehr Fassade, mehr Watt."),
        ("profile",
         "Through the wall, temperature drops from indoors to outdoors — that gap is delta theta.",
         "Durch die Wand fällt die Temperatur von innen nach außen — diese Lücke ist Delta-Theta."),
        ("steeper",
         "A steeper gradient means a larger driving force and a higher heat flow.",
         "Ein steilerer Gradient heißt größere Triebkraft und höherer Wärmestrom."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("U-Wert und Temperaturgradient", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "u_intro"))
        self.play(FadeIn(caption), run_time=0.3)

        r_text = Text("R  [m²·K/W]", font_size=FORMULA_FONT_SIZE, color=P_YELLOW).move_to(UP * 0.7)
        self.play(FadeIn(r_text), run_time=0.7)
        hold_for(self, self.NARRATION, "u_intro", used=0.7 + 0.3)

        row, items = equation_row([
            ("u", "U", P_ORANGE), (None, "=", P_WHITE),
            (None, "1", P_WHITE), (None, "/", P_WHITE),
            ("r", "R", P_TEAL),
            (None, "  [W/(m²·K)]", P_WHITE),
        ])
        row, box = formula_panel(row)
        self.play(FadeOut(r_text), FadeIn(row), Create(box), run_time=1.0)
        ring = highlight_param(items, "u", color=P_ORANGE)
        self.play(Create(ring), run_time=0.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "u_formula"))
        hold_for(self, self.NARRATION, "u_formula", used=1.4 + 0.35)
        self.play(FadeOut(ring), run_time=0.2)

        self.play(FadeOut(row), FadeOut(box), run_time=0.4)
        unit_square = Square(side_length=1.1, color=P_RED, fill_opacity=0.3, stroke_width=2).move_to(UP * 0.35)
        unit_label = Text("1 m²", font_size=BODY_FONT_SIZE, color=P_RED).next_to(unit_square, DOWN, buff=0.2)
        self.play(Create(unit_square), FadeIn(unit_label), run_time=0.8)

        grid = VGroup()
        for i in range(5):
            for j in range(5):
                sq = Square(side_length=0.42, color=P_GREEN, fill_opacity=0.25, stroke_width=1.5)
                sq.move_to(np.array([(j - 2) * 0.45, (2 - i) * 0.45 + 0.35, 0.0]))
                grid.add(sq)
        grid_label = Text("A = 25 m²", font_size=BODY_FONT_SIZE, color=P_GREEN).next_to(grid, DOWN, buff=0.2)
        self.play(ReplacementTransform(unit_square, grid), ReplacementTransform(unit_label, grid_label), run_time=1.2)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "area"))
        hold_for(self, self.NARRATION, "area", used=2.0 + 0.35)

        self.play(FadeOut(grid), FadeOut(grid_label), run_time=0.5)

        axes = Axes(
            x_range=[0, 3, 1], y_range=[-25, 30, 10],
            x_length=5.2, y_length=3.4,
            axis_config={"include_numbers": False, "color": GREY},
        ).move_to(UP * 0.25 + LEFT * 0.4)
        x_label_in = Text("Innen", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(axes.c2p(0.5, -25) + DOWN * 0.25)
        x_label_wall = Text("Wand", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(axes.c2p(1.5, -25) + DOWN * 0.25)
        x_label_out = Text("Außen", font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(axes.c2p(2.5, -25) + DOWN * 0.25)
        y_label_20 = Text("20 °C", font_size=LABEL_FONT_SIZE, color=P_RED).next_to(axes.c2p(0, 20), LEFT, buff=0.12)
        y_label_m5 = Text("−5 °C", font_size=LABEL_FONT_SIZE, color=P_BLUE).next_to(axes.c2p(0, -5), LEFT, buff=0.12)

        self.play(
            Create(axes),
            FadeIn(x_label_in), FadeIn(x_label_wall), FadeIn(x_label_out),
            FadeIn(y_label_20), FadeIn(y_label_m5),
            run_time=1.0,
        )

        p_in_20, p_w1_20 = axes.c2p(0.5, 20), axes.c2p(1.0, 20)
        p_w2_m5, p_out_m5 = axes.c2p(2.0, -5), axes.c2p(2.5, -5)
        temp_line1 = Line(p_in_20, p_w1_20, color=P_RED, stroke_width=3)
        temp_line_wall = Line(p_w1_20, p_w2_m5, color=P_YELLOW, stroke_width=3)
        temp_line_out = Line(p_w2_m5, p_out_m5, color=P_BLUE, stroke_width=3)
        dot_in = Dot(p_in_20, color=P_RED, radius=0.07)
        dot_out = Dot(p_out_m5, color=P_BLUE, radius=0.07)
        self.play(Create(VGroup(temp_line1, temp_line_wall, temp_line_out)), FadeIn(dot_in), FadeIn(dot_out), run_time=1.0)

        brace = BraceBetweenPoints(
            axes.c2p(2.7, 20), axes.c2p(2.7, -5), direction=RIGHT, color=P_YELLOW
        ).shift(RIGHT * 0.3)
        brace_label = Text("Δθ = 25 K", font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(brace, RIGHT, buff=0.15)
        self.play(Create(brace), FadeIn(brace_label), run_time=0.8)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "profile"))
        hold_for(self, self.NARRATION, "profile", used=2.8 + 0.35)

        p_w2_m15, p_out_m15 = axes.c2p(2.0, -15), axes.c2p(2.5, -15)
        y_label_m15 = Text("−15 °C", font_size=LABEL_FONT_SIZE, color=P_BLUE).next_to(axes.c2p(0, -15), LEFT, buff=0.12)
        brace_new = BraceBetweenPoints(
            axes.c2p(2.7, 20), axes.c2p(2.7, -15), direction=RIGHT, color=P_YELLOW
        ).shift(RIGHT * 0.3)
        brace_label_new = Text("Δθ = 35 K", font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(brace_new, RIGHT, buff=0.15)
        self.play(
            Transform(y_label_m5, y_label_m15),
            Transform(temp_line_wall, Line(p_w1_20, p_w2_m15, color=P_YELLOW, stroke_width=3)),
            Transform(temp_line_out, Line(p_w2_m15, p_out_m15, color=P_BLUE, stroke_width=3)),
            dot_out.animate.move_to(p_out_m15),
            ReplacementTransform(brace, brace_new),
            ReplacementTransform(brace_label, brace_label_new),
            run_time=1.5,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "steeper"))
        hold_for(self, self.NARRATION, "steeper", used=1.5 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)


class Beat4_Gebaeudehuelle(Scene):
    NARRATION = [
        ("intro",
         "For the whole building, add every envelope piece: roof, windows, walls, doors, floor.",
         "Für das ganze Gebäude addieren wir jedes Hüllteil: Dach, Fenster, Wände, Türen, Boden."),
        ("formula",
         "Transmission heat loss is the sum of U_i times A_i times delta theta_i — in watts, per DIN EN ISO 13789.",
         "Der Transmissionswärmeverlust ist die Summe aus U_i mal A_i mal Delta-Theta_i — in Watt, nach DIN EN ISO 13789."),
        ("paths",
         "Each pathway leaks on its own — the total is simply their sum.",
         "Jeder Pfad leckt für sich — die Summe ergibt den Gesamtverlust."),
        ("outro",
         "That sum is the conduction story of the thermal envelope.",
         "Diese Summe ist die Wärmeleitungs-Geschichte der thermischen Gebäudehülle."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die thermische Gebäudehülle", title)
        din = _din_ref("DIN EN ISO 13789")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        row, items = equation_row([
            ("q", "Q̇_T", P_WHITE), (None, "=", P_WHITE),
            (None, "Σ", P_WHITE),
            ("u", "(U_i", P_ORANGE), (None, "·", P_WHITE),
            ("a", "A_i", P_CYAN), (None, "·", P_WHITE),
            ("dt", "Δθ_i)", P_BLUE),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row)
        self.play(FadeIn(row), Create(box), run_time=1.1)
        hold_for(self, self.NARRATION, "intro", used=1.1 + 0.3)

        ring = highlight_param(items, "q", color=P_TEAL)
        self.play(Create(ring), run_time=0.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        hold_for(self, self.NARRATION, "formula", used=0.4 + 0.35)
        self.play(FadeOut(ring), run_time=0.2)

        # Dropped from UP*0.35: the roof arrow and its "Dach" label reached y ≈ 2.9
        # and ran into the "Die thermische Gebäudehülle" beat subtitle. Every house
        # point and pathway arrow/label below is placed relative to house_center,
        # so lowering it shifts the whole group down together; the shorter roof
        # arrow keeps "Dach" clear of the subtitle at the top without pushing
        # "Boden" onto the formula panel (SAFE_BOTTOM_FORMULA) at the bottom.
        house_center = DOWN * 0.1
        w_width, w_height = 3.0, 1.7
        bl = house_center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
        br = house_center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
        tl = house_center + LEFT * (w_width / 2) + UP * (w_height / 2)
        tr = house_center + RIGHT * (w_width / 2) + UP * (w_height / 2)
        roof_peak = house_center + UP * (w_height / 2 + 0.85)

        floor_line = Line(bl + LEFT * 0.5, br + RIGHT * 0.5, color=P_TEAL, stroke_width=3)
        walls = VGroup(Line(bl, tl, color=P_WHITE, stroke_width=3), Line(br, tr, color=P_WHITE, stroke_width=3))
        roof = Polygon(tl, roof_peak, tr, color=P_WHITE, stroke_width=3)
        window = Square(side_length=0.55, color=P_CYAN, stroke_width=2).move_to(house_center + LEFT * 0.7 + UP * 0.2)
        window_cross = VGroup(
            Line(window.get_top(), window.get_bottom(), color=P_CYAN, stroke_width=1.5),
            Line(window.get_left(), window.get_right(), color=P_CYAN, stroke_width=1.5),
        )
        door = Rectangle(width=0.5, height=0.8, color=P_CYAN, stroke_width=2).move_to(house_center + RIGHT * 0.6 + DOWN * 0.4)
        door_knob = Dot(door.get_center() + LEFT * 0.14 + DOWN * 0.05, radius=0.03, color=P_CYAN)

        self.play(Create(floor_line), Create(walls), Create(roof), run_time=1.4)
        self.play(Create(VGroup(window, window_cross)), Create(VGroup(door, door_knob)), run_time=1.0)

        roof_arrow = Arrow(roof_peak + UP * 0.05, roof_peak + UP * 0.34, color=P_ORANGE, buff=0, stroke_width=3)
        roof_label = Text("Dach", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(roof_arrow, UP, buff=0.08)
        win_arrow = Arrow(window.get_left(), house_center + LEFT * 2.1 + UP * 0.2, color=P_ORANGE, buff=0.05, stroke_width=3)
        win_label = Text("Fenster", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(win_arrow, LEFT, buff=0.08)
        wall_arrow = Arrow(house_center + LEFT * 1.5 + DOWN * 0.2, house_center + LEFT * 2.1 + DOWN * 0.2, color=P_ORANGE, buff=0.05, stroke_width=3)
        wall_label = Text("Wände", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(wall_arrow, LEFT, buff=0.08)
        door_arrow = Arrow(door.get_right(), house_center + RIGHT * 2.1 + DOWN * 0.4, color=P_ORANGE, buff=0.05, stroke_width=3)
        door_label = Text("Türen", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(door_arrow, RIGHT, buff=0.08)
        floor_arrow = Arrow(house_center + DOWN * 0.9, house_center + DOWN * 1.1, color=P_ORANGE, buff=0, stroke_width=3)
        floor_label = Text("Boden", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(floor_arrow, DOWN, buff=0.06)

        pathways = [
            (roof_arrow, roof_label), (win_arrow, win_label), (wall_arrow, wall_label),
            (door_arrow, door_label), (floor_arrow, floor_label),
        ]
        self.play(
            LaggedStart(*[GrowArrow(a) for a, _ in pathways], lag_ratio=0.18),
            LaggedStart(*[FadeIn(l) for _, l in pathways], lag_ratio=0.18),
            run_time=2.2,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "paths"))
        hold_for(self, self.NARRATION, "paths", used=4.6 + 0.35)

        for arrow, label in pathways:
            self.play(arrow.animate.set_color(P_YELLOW), label.animate.set_color(P_YELLOW), run_time=0.25)
            self.play(arrow.animate.set_color(P_ORANGE), label.animate.set_color(P_ORANGE), run_time=0.25)

        self.play(
            *[arrow.animate.set_color(P_YELLOW) for arrow, _ in pathways],
            *[label.animate.set_color(P_YELLOW) for _, label in pathways],
            run_time=0.8,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", used=3.3 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
