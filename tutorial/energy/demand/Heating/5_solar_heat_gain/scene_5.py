"""☀️ Heating Module 5 — Solarer Wärmegewinn.

Migrated from ``merged_scenes.py`` onto the generate-manim-tutorial template:
fixed type scale from ``manim_fonts``, ``formula_panel`` with SI units,
German ``caption_bar`` / ``hold_for`` sync, and ``_fit_stage`` screen
management — same polish pattern as Modul 2–4.
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
    solar_wave_ray,
)

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 5: Solarer Wärmegewinn"

COLOR_G = "#FACC15"
COLOR_A = "#38BDF8"
COLOR_WIN = "#00F0FF"
COLOR_WALL = "#64748B"
COLOR_FRAME = "#94A3B8"
COLOR_FF = "#F97316"
COLOR_GVAL = "#2ECC71"
COLOR_FSH = "#95A5A6"
COLOR_GOLD = "#FFD700"
COLOR_HEAT = "#F59E0B"
COLOR_SLAB = "#737373"
COLOR_MASS = "#38BDF8"
COLOR_C = "#2ECC71"

# Stage fills the band under the beat subtitle and above the formula/caption zone.
# Prefer native type-scale labels — do not crush the stage to ~0.6.
CONTENT_GAP_BELOW_TITLE = 0.35
CONTENT_TOP_MAX = 2.15
CONTENT_BOTTOM_MIN = -2.25
CONTENT_MAX_WIDTH = 12.8
CONTENT_SCALE_MAX = 1.55
CONTENT_SHRINK_FLOOR = 1.0
# 1.0 sat the formula box's own bottom edge inside the caption_bar zone below
# it (measured: box bottom ≈ -3.27 vs caption top ≈ -3.06) — every beat whose
# caption ran the width of the frame had its formula panel crossing the
# caption text. 1.35 clears it with margin while shifting the panel as little
# as possible from its original position.
FORMULA_EDGE_BUFF = 1.35


#region Shared
def _is_drawn(m):
    """True if the mobject currently contributes visible fill or stroke."""
    fo = m.get_fill_opacity() if hasattr(m, "get_fill_opacity") else 0
    so = m.get_stroke_opacity() if hasattr(m, "get_stroke_opacity") else 0
    sw = m.get_stroke_width() if hasattr(m, "get_stroke_width") else 0
    return fo > 0.02 or (so > 0.02 and sw > 0.4)


def _fit_stage(mob, *, below, focus=None, scale_max=None, shrink_floor=None):
    """↘️ Fill the free band under the beat subtitle.

    ``focus`` (optional) is the dense diagram core used for sizing — sparse
    arrows/labels are left out of the probe so the house/window actually grows.
    Fully hidden children are ignored so later-reveal pieces do not crush size.
    ``scale_max`` / ``shrink_floor`` override module defaults for dense beats.
    """
    from manim import Arrow, DashedLine, Text, VGroup

    top = min(below.get_bottom()[1] - CONTENT_GAP_BELOW_TITLE, CONTENT_TOP_MAX)
    avail_h = max(top - CONTENT_BOTTOM_MIN, 0.5)
    if focus is not None:
        probe_src = focus
    else:
        probe_src = VGroup(*[
            m for m in mob.family_members_with_points()
            if _is_drawn(m) and not isinstance(m, (Text, Arrow, DashedLine))
        ])
        if len(probe_src) == 0:
            probe_src = VGroup(*[m for m in mob.family_members_with_points() if _is_drawn(m)])
    probe = probe_src if len(probe_src) > 0 else mob
    max_s = CONTENT_SCALE_MAX if scale_max is None else scale_max
    min_s = CONTENT_SHRINK_FLOOR if shrink_floor is None else shrink_floor
    scale = min(
        avail_h / max(probe.height, 1e-6),
        CONTENT_MAX_WIDTH / max(probe.width, 1e-6),
        max_s,
    )
    if scale < min_s:
        scale = min_s
    mob.scale(scale)
    # Re-measure after scale; park using focus/geometry, not label outliers.
    if focus is not None:
        anchor = focus
    else:
        anchor = VGroup(*[
            m for m in mob.family_members_with_points()
            if _is_drawn(m) and not isinstance(m, (Text, Arrow, DashedLine))
        ]) or mob
    mob.shift(DOWN * (anchor.get_top()[1] - top))
    if anchor.get_bottom()[1] < CONTENT_BOTTOM_MIN:
        mob.shift(UP * (CONTENT_BOTTOM_MIN - anchor.get_bottom()[1]))
    return mob


def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in the other Heating
    modules. Module 5's solar-gain chain — Φ_solar = G · A · F_f · g · F_sh —
    is the DIN V 18599-2 method; the thermal-mass beats cite DIN EN ISO 13786.
    Added in absolute frame coordinates, so ``_fit_stage`` never scales it, and
    skipped on the g-value beat, which already prints its norm in the diagram.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Beat1 — Verlust zu Gewinn
class Beat1_VerlustZuGewinn(Scene):
    """🔄 Flip from envelope heat loss to free solar gain."""

    NARRATION = [
        ("loss",
         "Outside heat leaves through the envelope as transmission loss.",
         "Wärme verlässt die Hülle als Transmissionsverlust."),
        ("flip",
         "Now flip the view — the sun becomes a free heat source.",
         "Jetzt drehen wir um — die Sonne wird zur freien Wärmequelle."),
        ("gain",
         "Solar heat gain arrives at roof, walls, and windows.",
         "Solarer Wärmegewinn trifft Dach, Wände und Fenster."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Von Wärmeverlust zu solarem Gewinn", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "loss"))
        self.play(FadeIn(caption), run_time=0.3)

        house_center = DOWN * 0.2
        walls = Rectangle(width=4.0, height=2.2, color=GREY_B, stroke_width=2.5)
        walls.move_to(house_center)
        roof = Polygon(
            house_center + LEFT * 2.3 + UP * 1.1,
            house_center + UP * 2.3,
            house_center + RIGHT * 2.3 + UP * 1.1,
            color=GREY_B, stroke_width=2.5,
        )
        floor_line = Line(
            house_center + LEFT * 2.8 + DOWN * 1.1,
            house_center + RIGHT * 2.8 + DOWN * 1.1,
            color=GREY_B, stroke_width=2,
        )
        window = Rectangle(width=1.0, height=1.1, color=GREY_B, stroke_width=2)
        window.move_to(house_center + RIGHT * 1.1 + UP * 0.1)
        window_cross_h = Line(window.get_left(), window.get_right(), color=GREY_B, stroke_width=1)
        window_cross_v = Line(window.get_top(), window.get_bottom(), color=GREY_B, stroke_width=1)
        window_group = VGroup(window, window_cross_h, window_cross_v)
        door = Rectangle(width=0.7, height=1.2, color=GREY_B, stroke_width=2)
        door.move_to(house_center + LEFT * 1.0 + DOWN * 0.5)
        house = VGroup(walls, roof, floor_line, window_group, door)

        arrow_left = CurvedArrow(
            house_center + LEFT * 2.1 + DOWN * 0.2,
            house_center + LEFT * 3.2 + UP * 0.2,
            angle=TAU / 12, color=P_CYAN,
        )
        arrow_right = CurvedArrow(
            house_center + RIGHT * 2.1 + DOWN * 0.2,
            house_center + RIGHT * 3.2 + UP * 0.2,
            angle=-TAU / 12, color=P_CYAN,
        )
        arrow_roof_left = CurvedArrow(
            house_center + LEFT * 1.2 + UP * 1.8,
            house_center + LEFT * 2.2 + UP * 2.6,
            angle=TAU / 12, color=P_CYAN,
        )
        arrow_roof_right = CurvedArrow(
            house_center + RIGHT * 1.2 + UP * 1.8,
            house_center + RIGHT * 2.2 + UP * 2.6,
            angle=-TAU / 12, color=P_CYAN,
        )
        arrow_top = CurvedArrow(
            house_center + UP * 2.4,
            house_center + UP * 3.2 + RIGHT * 0.4,
            angle=-TAU / 12, color=P_CYAN,
        )
        loss_arrows = VGroup(
            arrow_left, arrow_right, arrow_roof_left, arrow_roof_right, arrow_top,
        )
        loss_label = Text("Wärmeverlust Q_loss", font_size=BODY_FONT_SIZE, color=P_CYAN)
        loss_label.next_to(house, RIGHT, buff=0.45).shift(UP * 0.4)

        sun_pos = LEFT * 4.2 + UP * 1.8
        # Soft sun like merged_scenes — glow stays translucent (never set_opacity(1)).
        sun_core = Dot(sun_pos, radius=0.35, color=COLOR_G, fill_opacity=1.0)
        sun_glow = Dot(sun_pos, radius=0.55, color=COLOR_GOLD, fill_opacity=0.4)
        ring1 = Circle(radius=0.65, color=COLOR_G, stroke_width=1.5, stroke_opacity=0.8).move_to(sun_pos)
        ring2 = Circle(radius=0.9, color=COLOR_GOLD, stroke_width=1.2, stroke_opacity=0.5).move_to(sun_pos)
        ring3 = Circle(radius=1.15, color=COLOR_G, stroke_width=1.0, stroke_opacity=0.25).move_to(sun_pos)
        sun_rings = VGroup(ring1, ring2, ring3)
        sun_burst_rays = VGroup()
        for angle in np.linspace(0, TAU, 10, endpoint=False):
            start_p = sun_pos + np.array([np.cos(angle) * 0.45, np.sin(angle) * 0.45, 0])
            end_p = sun_pos + np.array([np.cos(angle) * 0.72, np.sin(angle) * 0.72, 0])
            sun_burst_rays.add(Line(start_p, end_p, color=COLOR_GOLD, stroke_width=2, stroke_opacity=0.85))

        targets = [
            house_center + LEFT * 1.5 + UP * 1.8,
            house_center + UP * 2.3,
            house_center + RIGHT * 0.8 + UP * 1.7,
            window.get_center(),
            house_center + LEFT * 1.8 + UP * 0.3,
        ]
        # Wavy solar rays (``solar_wave_ray``), matching the Strahlung style used
        # in the other tutorial scenes — a straight Line read as a laser, not
        # radiation.
        radiation_lines = VGroup()
        for target in targets:
            start_pt = sun_pos + (target - sun_pos) * 0.18
            radiation_lines.add(solar_wave_ray(
                start_pt, target, color=COLOR_G, stroke_width=2.5, amp=0.08, cycles=3.5,
            ))
        gain_label = Text("Solarer Gewinn Q_gain", font_size=BODY_FONT_SIZE, color=COLOR_G)
        gain_label.next_to(house, RIGHT, buff=0.45).shift(UP * 0.4)

        stage = VGroup(
            house, loss_arrows, loss_label,
            sun_glow, sun_core, sun_rings, sun_burst_rays, radiation_lines, gain_label,
        )
        sun_glow.set_fill(opacity=0)
        sun_core.set_fill(opacity=0)
        sun_rings.set_stroke(opacity=0)
        sun_burst_rays.set_stroke(opacity=0)
        radiation_lines.set_stroke(opacity=0)
        gain_label.set_opacity(0)
        _fit_stage(
            stage, below=subtitle,
            focus=VGroup(house, sun_glow, sun_core, sun_rings, sun_burst_rays),
        )

        self.play(Create(house), Create(loss_arrows), FadeIn(loss_label), run_time=1.8)
        hold_for(self, self.NARRATION, "loss", used=TITLE_RUN_TIME + 0.35 + 1.8)

        self.play(FadeOut(loss_arrows), FadeOut(loss_label), run_time=0.9)
        # Set final soft opacities, then FadeIn only the opaque core.
        sun_core.set_fill(COLOR_G, opacity=1.0)
        sun_glow.set_fill(COLOR_GOLD, opacity=0)
        ring1.set_stroke(COLOR_G, width=1.5, opacity=0)
        ring2.set_stroke(COLOR_GOLD, width=1.2, opacity=0)
        ring3.set_stroke(COLOR_G, width=1.0, opacity=0)
        sun_burst_rays.set_stroke(COLOR_GOLD, width=2, opacity=0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flip"))
        self.play(
            FadeIn(sun_core, scale=0.7),
            sun_glow.animate.set_fill(COLOR_GOLD, opacity=0.4),
            ring1.animate.set_stroke(opacity=0.8),
            ring2.animate.set_stroke(opacity=0.5),
            ring3.animate.set_stroke(opacity=0.25),
            sun_burst_rays.animate.set_stroke(COLOR_GOLD, width=2, opacity=0.85),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "flip", used=0.9 + 1.3 + 0.35)

        radiation_lines.set_stroke(COLOR_G, width=2.5, opacity=0.85)
        gain_label.set_opacity(1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gain"))
        self.play(
            LaggedStart(*[Create(line) for line in radiation_lines], lag_ratio=0.12),
            FadeIn(gain_label),
            ring1.animate.scale(1.15).set_stroke(opacity=0.3),
            ring2.animate.scale(1.1).set_stroke(opacity=0.2),
            ring3.animate.scale(1.08).set_stroke(opacity=0.1),
            run_time=1.8,
        )
        hold_for(self, self.NARRATION, "gain", used=1.8 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 — Bestrahlung und Fläche
class Beat2_BestrahlungUndFlaeche(Scene):
    """🪟 Irradiance G, area A, and frame factor F_f."""

    NARRATION = [
        ("aperture",
         "Start from the full window opening in the wall.",
         "Zuerst die gesamte Fensteröffnung in der Wand."),
        ("g",
         "Irradiance G is the solar power density on the glass — in watts per square meter.",
         "Bestrahlungsstärke G — solare Leistungsdichte in Watt pro Quadratmeter."),
        ("a",
         "Area A is the gross window area in square meters.",
         "Die Fläche A ist die Brutto-Fensterfläche in Quadratmetern."),
        ("ff",
         "Only the glass fraction F_f transmits — the opaque frame blocks — product in watts.",
         "Nur der Glasanteil F_f lässt durch — der Rahmen blockiert — Produkt in Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Bestrahlungsstärke G und Fläche A", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "aperture"))
        self.play(FadeIn(caption), run_time=0.3)

        house_center = LEFT * 0.2 + DOWN * 0.05
        wall_outer = Rectangle(
            width=2.9, height=3.2, color=COLOR_WALL,
            fill_color="#1E293B", fill_opacity=0.9, stroke_width=3,
        ).move_to(house_center)
        win_gross = Rectangle(
            width=1.35, height=1.9, color=COLOR_A,
            fill_color=COLOR_A, fill_opacity=0.12, stroke_width=3,
        ).move_to(house_center)
        gross_label = Text("Gesamte Fensteröffnung", font_size=BODY_FONT_SIZE, color=COLOR_A)
        gross_label.next_to(wall_outer, DOWN, buff=0.18)

        sun_center = RIGHT * 4.2 + UP * 1.15
        sun_core = Dot(sun_center, radius=0.28, color=COLOR_G, fill_opacity=1.0)
        sun_ring1 = Circle(radius=0.45, color=COLOR_G, stroke_width=2, stroke_opacity=0.6).move_to(sun_center)
        sun_ring2 = Circle(radius=0.62, color=COLOR_G, stroke_width=1, stroke_opacity=0.3).move_to(sun_center)
        sun_group = VGroup(sun_core, sun_ring1, sun_ring2)

        mid_pt = win_gross.get_center()
        ray_targets = [
            win_gross.get_top() + RIGHT * 0.05,
            mid_pt + UP * 0.4,
            mid_pt,
            mid_pt + DOWN * 0.4,
            win_gross.get_bottom() + RIGHT * 0.05,
        ]
        ray_ops = [0.4, 0.55, 0.85, 0.55, 0.4]
        # Wavy Strahlung (``solar_wave_ray``), matching Beat 1 and the other
        # tutorial scenes — a straight Line reads as a laser, not radiation.
        rays = VGroup(*[
            solar_wave_ray(sun_center, t, color=COLOR_G, stroke_width=sw, amp=0.07, cycles=3.5).set_opacity(op)
            for t, sw, op in zip(
                ray_targets,
                [2.5, 2.5, 4.0, 2.5, 2.5],
                ray_ops,
            )
        ])
        # Lifted clear of the ray fan converging on the window: at the previous
        # UP*0.65 offset this cluster sat inside the bundle and "G [W/m²]" was
        # visibly crossed by one of the rays.
        label_G = Text("G", font_size=FORMULA_FONT_SIZE, color=COLOR_G)
        sub_G = Text("G [W/m²]", font_size=BODY_FONT_SIZE, color=COLOR_G)
        g_mid = (sun_center + mid_pt) / 2 + UP * 1.15
        label_G.move_to(g_mid)
        sub_G.next_to(label_G, DOWN, buff=0.1)

        label_A = Text("A", font_size=FORMULA_FONT_SIZE, color=COLOR_A)
        sub_A = Text("A [m²]", font_size=BODY_FONT_SIZE, color=COLOR_A)
        group_A = VGroup(label_A, sub_A).arrange(DOWN, buff=0.1)
        group_A.next_to(wall_outer, LEFT, buff=0.35)

        win_outer_frame = Rectangle(
            width=1.35, height=1.9, color=COLOR_FRAME,
            fill_color="#334155", fill_opacity=1.0, stroke_width=4,
        ).move_to(house_center)
        win_glass = Rectangle(
            width=0.95, height=1.45, color=COLOR_WIN,
            fill_color=COLOR_WIN, fill_opacity=0.35, stroke_width=2,
        ).move_to(house_center)
        h_mullion = Line(win_glass.get_left(), win_glass.get_right(), color=COLOR_FRAME, stroke_width=2)
        v_mullion = Line(win_glass.get_top(), win_glass.get_bottom(), color=COLOR_FRAME, stroke_width=2)
        mullions = VGroup(h_mullion, v_mullion)
        frame_desc = Text(
            "Rahmen blockiert Strahlung", font_size=BODY_FONT_SIZE, color=COLOR_FRAME,
        ).next_to(wall_outer, DOWN, buff=0.18)
        ff_box = win_glass.copy().set_color(COLOR_FF).set_fill(COLOR_FF, opacity=0.4)
        label_Ff = Text("F_f", font_size=BODY_FONT_SIZE, color=COLOR_FF)
        sub_Ff = Text("F_f ≈ 0,7–0,8 [-]", font_size=BODY_FONT_SIZE, color=COLOR_FF)
        group_Ff = VGroup(label_Ff, sub_Ff).arrange(DOWN, buff=0.1)
        group_Ff.next_to(wall_outer, LEFT, buff=0.35)

        stage = VGroup(
            wall_outer, win_gross, gross_label, sun_group, rays,
            label_G, sub_G, group_A, win_outer_frame, win_glass, mullions,
            frame_desc, ff_box, group_Ff,
        )
        sun_core.set_fill(opacity=0)
        sun_ring1.set_stroke(opacity=0)
        sun_ring2.set_stroke(opacity=0)
        for ray in rays:
            ray.set_stroke(opacity=0)
        for m in (
            label_G, sub_G, group_A,
            win_outer_frame, win_glass, mullions, frame_desc, ff_box, group_Ff,
        ):
            m.set_opacity(0)
        # Hold the diagram smaller (scale_max) so the wall/window assembly does
        # not reach down into the formula panel — at the module default the
        # wall's bottom edge sat on the "G · A · F_f [W]" box at the end.
        _fit_stage(
            stage, below=subtitle,
            focus=VGroup(wall_outer, win_gross, sun_group), scale_max=1.0,
        )

        self.play(Create(wall_outer), DrawBorderThenFill(win_gross), FadeIn(gross_label), run_time=1.3)
        hold_for(self, self.NARRATION, "aperture", used=1.3 + 0.3)

        sun_core.set_fill(COLOR_G, opacity=1.0)
        sun_ring1.set_stroke(COLOR_G, width=2, opacity=0)
        sun_ring2.set_stroke(COLOR_G, width=1, opacity=0)
        for ray, op in zip(rays, ray_ops):
            ray.set_stroke(COLOR_G, opacity=0)
        label_G.set_opacity(1)
        sub_G.set_opacity(1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "g"))
        self.play(
            FadeIn(sun_core, scale=0.85),
            sun_ring1.animate.set_stroke(opacity=0.6),
            sun_ring2.animate.set_stroke(opacity=0.3),
            run_time=0.7,
        )
        for ray, op in zip(rays, ray_ops):
            ray.set_stroke(COLOR_G, opacity=op)
        self.play(LaggedStart(*[Create(ray) for ray in rays], lag_ratio=0.12), run_time=1.2)
        self.play(FadeIn(label_G, shift=UP * 0.15), FadeIn(sub_G), run_time=0.7)
        hold_for(self, self.NARRATION, "g", used=0.7 + 1.2 + 0.7 + 0.35)

        group_A.set_opacity(1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "a"))
        self.play(
            win_gross.animate.set_fill(COLOR_A, opacity=0.4),
            FadeIn(group_A, shift=RIGHT * 0.15),
            FadeOut(gross_label),
            FadeOut(sub_G),
            run_time=1.1,
        )
        hold_for(self, self.NARRATION, "a", used=1.1 + 0.35)

        win_outer_frame.set_opacity(1)
        win_glass.set_opacity(1)
        mullions.set_opacity(1)
        frame_desc.set_opacity(1)
        ff_box.set_opacity(1)
        group_Ff.set_opacity(1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ff"))
        self.play(
            FadeOut(win_gross), FadeOut(group_A),
            GrowFromCenter(win_outer_frame), Create(win_glass), Create(mullions),
            FadeIn(frame_desc),
            run_time=1.3,
        )
        self.play(Create(ff_box), FadeIn(group_Ff, shift=LEFT * 0.1), run_time=0.9)
        self.play(ff_box.animate.set_fill(opacity=0.75), rate_func=there_and_back, run_time=0.9)

        row, items = equation_row([
            ("g", "G", COLOR_G),
            (None, "·", P_WHITE),
            ("a", "A", COLOR_A),
            (None, "·", P_WHITE),
            ("ff", "F_f", COLOR_FF),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row, edge_buff=FORMULA_EDGE_BUFF)
        self.play(
            FadeOut(frame_desc), FadeOut(ff_box),
            FadeOut(label_G), FadeOut(group_Ff),
            FadeIn(row), Create(box),
            run_time=1.1,
        )
        ring = highlight_param(items, "ff", color=COLOR_FF)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, self.NARRATION, "ff", used=1.3 + 0.9 + 0.9 + 1.1 + 0.4 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 — g-Wert
class Beat3_GWert(Scene):
    """🪟 Glass section: reflect, absorb, transmit — the g-value."""

    NARRATION = [
        ("section",
         "Look at a glass cross-section: outside left, inside right.",
         "Glasquerschnitt: links außen, rechts innen."),
        ("split",
         "Radiation reflects, absorbs, and transmits.",
         "Strahlung reflektiert, absorbiert und transmittiert."),
        ("gval",
         "The g-value is the transmitted heat fraction into the room — dimensionless, times power still in watts.",
         "Der g-Wert ist der transmittierte Wärmeanteil — dimensionslos, Leistung bleibt in Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Gesamtenergiedurchlassgrad g", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "section"))
        self.play(FadeIn(caption), run_time=0.3)

        wall_upper = Rectangle(
            width=1.0, height=0.85, stroke_color=GREY_B, stroke_width=2, fill_opacity=0.05,
        ).shift(UP * 1.5)
        wall_lower = Rectangle(
            width=1.0, height=0.85, stroke_color=GREY_B, stroke_width=2, fill_opacity=0.05,
        ).shift(DOWN * 1.7)
        wall_lines = VGroup(
            Line(wall_upper.get_corner(DL), wall_upper.get_corner(UR), stroke_color=GREY_C, stroke_width=1),
            Line(wall_lower.get_corner(DL), wall_lower.get_corner(UR), stroke_color=GREY_C, stroke_width=1),
        )
        frame_top = (
            Rectangle(width=0.55, height=0.28, stroke_color=P_WHITE, stroke_width=2, fill_opacity=0.1)
            .next_to(wall_upper, DOWN, buff=0).match_x(wall_upper)
        )
        frame_bot = (
            Rectangle(width=0.55, height=0.28, stroke_color=P_WHITE, stroke_width=2, fill_opacity=0.1)
            .next_to(wall_lower, UP, buff=0).match_x(wall_lower)
        )
        glass_height = frame_top.get_bottom()[1] - frame_bot.get_top()[1]
        glass_pane = Rectangle(
            width=0.22, height=glass_height,
            stroke_color=COLOR_WIN, stroke_width=2,
            fill_opacity=0.12, fill_color=COLOR_WIN,
        ).move_to(VGroup(frame_top, frame_bot).get_center())

        ext_label = Text("Außen", color=GREY_B, font_size=BODY_FONT_SIZE).shift(LEFT * 3.8 + UP * 1.6)
        int_label = Text("Innen", color=GREY_B, font_size=BODY_FONT_SIZE).shift(RIGHT * 3.8 + UP * 1.6)
        glass_title = Text("Glasquerschnitt", color=COLOR_WIN, font_size=BODY_FONT_SIZE)
        glass_title.next_to(wall_upper, RIGHT, buff=0.35)

        hit_pt = glass_pane.get_left() + DOWN * 0.08
        inc_start = LEFT * 4.6 + UP * 0.6
        inc_ray = Line(inc_start, hit_pt, color=COLOR_G, stroke_width=4)
        # next_to(ray.get_center(), UP, buff) only guarantees clearance directly
        # above that one point — for a diagonal ray the label's own width still
        # reaches back down across the line elsewhere along its span. Needs
        # more buff than a horizontal label next to a horizontal line would.
        # Anchored near each ray's own far endpoint, not its center: both rays
        # converge on hit_pt near the glass, so labels placed near the center
        # of either one crowd both that convergence point and each other.
        # Away from the glass, near where each ray starts/ends, is open space.
        inc_label = Text("Sonnenstrahlung", color=COLOR_G, font_size=BODY_FONT_SIZE)
        inc_label.next_to(inc_start, UP, buff=0.30)

        refl_end = LEFT * 4.4 + UP * 1.5
        refl_ray = Line(hit_pt, refl_end, color=P_WHITE, stroke_width=3.5)
        refl_label = Text("Reflektiert", color=P_WHITE, font_size=BODY_FONT_SIZE)
        refl_label.next_to(refl_end, UP, buff=0.30)

        abs_end = glass_pane.get_corner(DR)
        abs_ray = Line(hit_pt, abs_end, color=P_RED, stroke_width=3.5)
        abs_label = Text("Absorbiert", color=P_RED, font_size=BODY_FONT_SIZE)
        abs_label.next_to(abs_ray.get_center(), RIGHT, buff=0.18)

        trans_pt_right = hit_pt + RIGHT * glass_pane.width
        trans_ray_inside = Line(hit_pt, trans_pt_right, color=COLOR_G, stroke_width=4)
        trans_end = RIGHT * 4.4 + DOWN * 0.7
        trans_ray_out = Line(trans_pt_right, trans_end, color=COLOR_G, stroke_width=4)
        trans_ray = VGroup(trans_ray_inside, trans_ray_out)
        trans_label = Text("Transmittiert", color=COLOR_G, font_size=BODY_FONT_SIZE)
        trans_label.next_to(trans_ray_out.get_center(), UP, buff=0.12)

        g_label = Text("g", color=COLOR_GVAL, font_size=FORMULA_FONT_SIZE)
        g_label.next_to(trans_ray_out, DOWN, buff=0.45)
        g_sub = Text("g [-] nach DIN V 18599", color=COLOR_GVAL, font_size=BODY_FONT_SIZE)
        g_sub.next_to(g_label, DOWN, buff=0.12)

        stage = VGroup(
            wall_upper, wall_lower, wall_lines, frame_top, frame_bot, glass_pane,
            ext_label, int_label, glass_title,
            inc_ray, inc_label, refl_ray, refl_label, abs_ray, abs_label,
            trans_ray, trans_label, g_label, g_sub,
        )
        for m in (
            inc_ray, inc_label, refl_ray, refl_label, abs_ray, abs_label,
            trans_ray, trans_label, g_label, g_sub,
        ):
            m.set_opacity(0)
        # The glass section runs the full frame height (wall_upper at UP*1.5 to
        # wall_lower at DOWN*1.7); at the module default it was scaled up to fill
        # the band and the lower wall block, its "g" caption and the ray labels
        # all collided with the formula panel and the subtitle text. Cap the
        # scale well below 1 so the whole section sits inside the free band, and
        # lift the formula panel (edge_buff below) to clear the two-line caption.
        _fit_stage(
            stage, below=subtitle,
            focus=VGroup(wall_upper, wall_lower, frame_top, frame_bot, glass_pane),
            scale_max=0.83, shrink_floor=0.8,
        )

        self.play(
            Create(wall_upper), Create(wall_lower), Create(wall_lines),
            Create(frame_top), Create(frame_bot), Create(glass_pane),
            FadeIn(ext_label), FadeIn(int_label), FadeIn(glass_title),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "section", used=1.4 + 0.3)

        for m in (inc_ray, inc_label):
            m.set_opacity(1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "split"))
        self.play(Create(inc_ray), FadeIn(inc_label), run_time=1.0)
        for m in (refl_ray, refl_label, abs_ray, abs_label, trans_ray, trans_label):
            m.set_opacity(1)
        self.play(
            LaggedStart(
                AnimationGroup(Create(refl_ray), FadeIn(refl_label)),
                AnimationGroup(Create(abs_ray), FadeIn(abs_label)),
                AnimationGroup(Create(trans_ray), FadeIn(trans_label)),
                lag_ratio=0.3,
            ),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "split", used=1.0 + 2.0 + 0.35)

        g_label.set_opacity(1)
        g_sub.set_opacity(1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gval"))
        self.play(FadeIn(g_label), FadeIn(g_sub), run_time=0.8)
        pulse_inc = Dot(color=COLOR_G, radius=0.1)
        pulse_trans = Dot(color=COLOR_G, radius=0.1)
        self.play(MoveAlongPath(pulse_inc, inc_ray, rate_func=linear), run_time=0.8)
        self.play(
            FadeOut(pulse_inc),
            MoveAlongPath(pulse_trans, trans_ray_out, rate_func=linear),
            run_time=0.9,
        )
        self.play(FadeOut(pulse_trans), Indicate(g_label, color=COLOR_GVAL, scale_factor=1.3), run_time=0.7)

        row, items = equation_row([
            ("g_irr", "G", COLOR_G),
            (None, "·", P_WHITE),
            ("a", "A", COLOR_A),
            (None, "·", P_WHITE),
            ("ff", "F_f", COLOR_FF),
            (None, "·", P_WHITE),
            ("g", "g", COLOR_GVAL),
            (None, "  [W]", P_WHITE),
        ])
        # Lifted above the module default so the box clears this beat's two-line
        # caption; the shrunk glass section (scale_max above) leaves the headroom.
        row, box = formula_panel(row, edge_buff=1.6)
        self.play(
            FadeOut(g_sub), FadeOut(g_label),
            FadeIn(row), Create(box),
            run_time=1.0,
        )
        ring = highlight_param(items, "g", color=COLOR_GVAL)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, self.NARRATION, "gval", used=0.8 + 0.8 + 0.9 + 0.7 + 1.0 + 0.4 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 — Saisonale Sonnenwinkel
class Beat4_SaisonaleWinkel(Scene):
    """🌞 High summer sun blocked; low winter sun warms the room."""

    NARRATION = [
        ("path",
         "The sun path climbs high in summer and stays low in winter.",
         "Die Sonnenbahn steht im Sommer hoch und im Winter flach."),
        ("summer",
         "A roof overhang blocks steep summer rays.",
         "Ein Dachüberstand blockiert steile Sommerstrahlen."),
        ("winter",
         "Low winter rays reach deep into the room and warm it.",
         "Flache Winterstrahlen dringen tief ein und wärmen den Raum."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Saisonale Sonnenwinkel", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "path"))
        self.play(FadeIn(caption), run_time=0.3)

        # Slightly compact room so winter beam clears formula / caption zones.
        floor_y = -1.35
        back_wall_x = -1.8
        front_wall_x = 1.0
        roof_y = 0.55
        overhang_x = 1.45
        sill_y = -0.4
        win_top_y = 0.4

        ground = Line([-4.6, floor_y, 0], [3.0, floor_y, 0], color=GREY_C, stroke_width=3)
        back_wall = Line([back_wall_x, floor_y, 0], [back_wall_x, roof_y, 0], color=P_WHITE, stroke_width=4)
        roof = Line([back_wall_x - 0.25, roof_y, 0], [overhang_x, roof_y, 0], color=P_WHITE, stroke_width=5)
        front_lower = Line([front_wall_x, floor_y, 0], [front_wall_x, sill_y, 0], color=P_WHITE, stroke_width=4)
        window_glass = Line(
            [front_wall_x, sill_y, 0], [front_wall_x, win_top_y, 0],
            color=P_CYAN, stroke_width=3,
        ).set_opacity(0.85)

        arc_center = np.array([0.15, -1.0, 0])
        arc_radius = 3.8
        summer_angle = np.radians(65)
        winter_angle = np.radians(25)
        solar_arc = Arc(
            radius=arc_radius, start_angle=np.radians(15), angle=np.radians(60),
            arc_center=arc_center, color=YELLOW_E, stroke_width=2,
        )
        dashed_arc = DashedVMobject(solar_arc, num_dashes=28)
        arc_label = Text("Sonnenbahn", font_size=BODY_FONT_SIZE, color=YELLOW_B)
        arc_label.move_to([3.2, 1.9, 0])

        def get_sun_pos(angle):
            return arc_center + arc_radius * np.array([np.cos(angle), np.sin(angle), 0])

        sun_pos_summer = get_sun_pos(summer_angle)
        sun_core = Dot(sun_pos_summer, radius=0.2, color=COLOR_G, fill_opacity=1.0)
        sun_glow = Dot(sun_pos_summer, radius=0.34, color=YELLOW_A, fill_opacity=0.3)
        sun = VGroup(sun_glow, sun_core)

        tag_summer = Text("Sommer: steiler Winkel", font_size=BODY_FONT_SIZE, color=COLOR_G)
        tag_summer.move_to([-3.4, 1.4, 0])
        tag_winter = Text("Winter: flacher Winkel", font_size=BODY_FONT_SIZE, color=P_BLUE)
        tag_winter.move_to([-3.4, 1.4, 0])

        summer_beam = Polygon(
            [overhang_x, roof_y, 0], [1.15, floor_y, 0],
            [front_wall_x, floor_y, 0], [front_wall_x, sill_y, 0],
            color=COLOR_G, fill_color=COLOR_G, fill_opacity=0.32, stroke_width=1,
        )
        summer_rays = VGroup(
            Line(sun_pos_summer, [overhang_x, roof_y, 0], color=YELLOW_A, stroke_width=1.5),
            Line(sun_pos_summer, [overhang_x, win_top_y, 0], color=YELLOW_A, stroke_width=1.5),
        )
        summer_note = Text(
            "Überhang blockiert\nSommerhitze",
            font_size=BODY_FONT_SIZE, color=YELLOW_B, line_spacing=1.1,
        ).move_to([-3.6, 0.1, 0])

        sun_pos_winter = get_sun_pos(winter_angle)
        winter_anchor = Dot(sun_pos_winter, radius=0.01).set_opacity(0)
        winter_beam = Polygon(
            [front_wall_x, win_top_y, 0], [back_wall_x, roof_y - 0.15, 0],
            [back_wall_x, floor_y, 0], [-1.5, floor_y, 0], [front_wall_x, sill_y, 0],
            color=P_ORANGE, fill_color=P_ORANGE, fill_opacity=0.32, stroke_width=1,
        )
        winter_rays = VGroup(
            Line(sun_pos_winter, [front_wall_x, win_top_y, 0], color=YELLOW_A, stroke_width=1.5),
            Line(sun_pos_winter, [front_wall_x, sill_y, 0], color=YELLOW_A, stroke_width=1.5),
        )
        winter_note = Text(
            "Tiefe Einstrahlung\nwärmt den Raum",
            font_size=BODY_FONT_SIZE, color=P_ORANGE, line_spacing=1.1,
        ).move_to([-3.6, 0.1, 0])

        # Path must live in the stage so scale/shift match the dashed Sonnenbahn.
        path_arc = Arc(
            radius=arc_radius, start_angle=summer_angle,
            angle=winter_angle - summer_angle, arc_center=arc_center,
        )
        path_arc.set_stroke(opacity=0)

        stage = VGroup(
            ground, back_wall, roof, front_lower, window_glass, dashed_arc, arc_label,
            sun, winter_anchor, tag_summer, tag_winter, summer_beam, summer_rays, summer_note,
            winter_beam, winter_rays, winter_note, path_arc,
        )
        sun_glow.set_fill(opacity=0)
        sun_core.set_fill(opacity=0)
        for m in (
            tag_summer, tag_winter, summer_beam, summer_rays, summer_note,
            winter_beam, winter_rays, winter_note,
        ):
            m.set_opacity(0)
        # Include sun path + summer sun height so the arc is not clipped under the title.
        _fit_stage(
            stage, below=subtitle,
            focus=VGroup(
                ground, back_wall, roof, front_lower, window_glass,
                dashed_arc, sun, arc_label, tag_summer,
            ),
        )
        # Park the full stage (house + sun path) fully inside the title/caption band.
        clear_top = subtitle.get_bottom()[1] - 0.9
        band_bottom = CONTENT_BOTTOM_MIN - 0.15
        band_h = clear_top - band_bottom
        if stage.height > band_h:
            stage.scale(band_h / stage.height)
        stage.shift(DOWN * (stage.get_top()[1] - clear_top))
        arc_label.next_to(dashed_arc, RIGHT, buff=0.12)
        high = max(
            dashed_arc.get_top()[1],
            sun.get_top()[1],
            arc_label.get_top()[1],
            tag_summer.get_top()[1],
        )
        if high > clear_top:
            stage.shift(DOWN * (high - clear_top))
        stage.shift(DOWN * 0.55)

        def _rebuild_summer_rays():
            """Rays from the live sun into the overhang / window after stage fit."""
            src = sun.get_center()
            tip = roof.get_end()
            win_hi = window_glass.get_top()
            return VGroup(
                Line(src, tip, color=YELLOW_A, stroke_width=1.5),
                Line(src, win_hi, color=YELLOW_A, stroke_width=1.5),
            )

        def _rebuild_winter_rays():
            """Rays from the live sun through the window after the sun has moved."""
            src = sun.get_center()
            return VGroup(
                Line(src, window_glass.get_top(), color=YELLOW_A, stroke_width=1.5),
                Line(src, window_glass.get_bottom(), color=YELLOW_A, stroke_width=1.5),
            )

        def _rebuild_summer_beam():
            src = sun.get_center()
            tip = roof.get_end()
            fx = window_glass.get_center()[0]
            gy = ground.get_center()[1]
            return Polygon(
                src,
                tip,
                [fx + 0.15, gy, 0],
                [fx, gy, 0],
                window_glass.get_bottom(),
                color=COLOR_G, fill_color=COLOR_G, fill_opacity=0.32, stroke_width=1,
            )

        def _rebuild_winter_beam():
            src = sun.get_center()
            fx = window_glass.get_center()[0]
            bx = back_wall.get_center()[0]
            gy = ground.get_center()[1]
            ry = roof.get_center()[1]
            return Polygon(
                src,
                window_glass.get_top(),
                [bx, ry - 0.1, 0],
                [bx, gy, 0],
                [bx + 0.35, gy, 0],
                window_glass.get_bottom(),
                color=P_ORANGE, fill_color=P_ORANGE, fill_opacity=0.32, stroke_width=1,
            )

        self.play(
            Create(ground), Create(back_wall), Create(roof), Create(front_lower),
            Create(window_glass), Create(dashed_arc), FadeIn(arc_label),
            run_time=1.6,
        )
        # Snap sun onto the fitted summer point, then reveal.
        sun.move_to(sun.get_center())
        sun_core.set_fill(COLOR_G, opacity=1.0)
        sun_glow.set_fill(YELLOW_A, opacity=0)
        tag_summer.set_opacity(1)
        self.play(
            FadeIn(sun_core, scale=0.6),
            sun_glow.animate.set_fill(YELLOW_A, opacity=0.3),
            FadeIn(tag_summer),
            run_time=0.9,
        )
        hold_for(self, self.NARRATION, "path", used=1.6 + 0.9 + 0.3)

        # Rebuild summer optics from the fitted sun / house — never pre-fit coords.
        self.remove(summer_rays, summer_beam)
        summer_rays = _rebuild_summer_rays()
        summer_beam = _rebuild_summer_beam()
        summer_note.set_opacity(1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "summer"))
        self.play(
            LaggedStart(*[Create(r) for r in summer_rays], lag_ratio=0.15),
            FadeIn(summer_beam),
            FadeIn(summer_note),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "summer", used=1.3 + 0.35)

        # Clear summer first — caption must not flip to winter while summer is still up.
        self.play(
            FadeOut(summer_rays), FadeOut(summer_beam), FadeOut(summer_note),
            ReplacementTransform(tag_summer, tag_winter),
            run_time=0.9,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        # Path from the fitted sun onto the fitted winter anchor (same curve family).
        move_path = ArcBetweenPoints(
            sun.get_center(),
            winter_anchor.get_center(),
            angle=winter_angle - summer_angle,
        )
        move_path.set_stroke(width=0, opacity=0)
        self.play(MoveAlongPath(sun, move_path), run_time=2.0, rate_func=smooth)
        sun.move_to(winter_anchor.get_center())

        self.remove(winter_rays, winter_beam)
        winter_rays = _rebuild_winter_rays()
        winter_beam = _rebuild_winter_beam()
        winter_note.set_opacity(1)
        self.play(
            LaggedStart(*[Create(r) for r in winter_rays], lag_ratio=0.12),
            GrowFromPoint(winter_beam, point=sun.get_center()),
            FadeIn(winter_note),
            run_time=1.5,
        )
        hold_for(self, self.NARRATION, "winter", used=0.9 + 2.0 + 1.5 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 — Verschattung
class Beat5_Verschattung(Scene):
    """⛱️ Awning shades summer sun; winter sun slips under — F_sh."""

    NARRATION = [
        ("awning",
         "An awning shades the window in summer.",
         "Ein Überhang verschattet das Fenster im Sommer."),
        ("winter",
         "In winter the low sun slips under the awning into the room.",
         "Im Winter gleitet die flache Sonne unter den Überhang."),
        ("fsh",
         "That seasonal blocking is the shading factor F_sh.",
         "Diese saisonale Abschattung ist der Verschattungsfaktor F_sh."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Verschattungsfaktor F_sh", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "awning"))
        self.play(FadeIn(caption), run_time=0.3)

        top_wall = Rectangle(
            width=0.28, height=0.6, fill_color="#22252a", fill_opacity=1,
            stroke_color=P_WHITE, stroke_width=2,
        ).move_to([0, 1.0, 0])
        bottom_wall = Rectangle(
            width=0.28, height=0.8, fill_color="#22252a", fill_opacity=1,
            stroke_color=P_WHITE, stroke_width=2,
        ).move_to([0, -1.35, 0])
        glass_window = Line([0, 0.7, 0], [0, -0.95, 0], color=COLOR_WIN, stroke_width=5)
        floor = Line([0, -1.75, 0], [3.2, -1.75, 0], color=GREY, stroke_width=2)
        ceiling = Line([0, 1.3, 0], [3.2, 1.3, 0], color=GREY, stroke_width=2)
        interior_label = Text("Wohnraum", font_size=BODY_FONT_SIZE, color=GREY).move_to([1.9, 0.95, 0])
        awning = Polygon(
            [0.12, 0.75, 0], [-1.3, 0.75, 0], [-1.3, 0.64, 0], [0.12, 0.64, 0],
            color=COLOR_FSH, fill_color=GREY, fill_opacity=0.9, stroke_width=2,
        )

        sun_glow = Dot(radius=0.38, color=COLOR_G, fill_opacity=0.2)
        sun_core = Dot(radius=0.2, color=COLOR_G)
        sun_halo = Circle(radius=0.3, color=P_ORANGE, stroke_width=2, stroke_opacity=0.75)
        sun_rays_grp = VGroup()
        for angle in np.linspace(0, TAU, 8, endpoint=False):
            p1 = np.array([0.24 * np.cos(angle), 0.24 * np.sin(angle), 0])
            p2 = np.array([0.36 * np.cos(angle), 0.36 * np.sin(angle), 0])
            sun_rays_grp.add(Line(p1, p2, color=COLOR_G, stroke_width=2, stroke_opacity=0.9))
        sun = VGroup(sun_glow, sun_core, sun_halo, sun_rays_grp)
        summer_pos = np.array([-2.9, 1.35, 0])
        winter_pos = np.array([-2.9, 0.05, 0])
        sun.move_to(summer_pos)
        winter_anchor = Dot(winter_pos, radius=0.01).set_opacity(0)
        sun_label = Text("Sommersonne", font_size=BODY_FONT_SIZE, color=P_RED)
        winter_sun_label = Text("Wintersonne", font_size=BODY_FONT_SIZE, color=P_ORANGE)

        summer_beam = Polygon(
            [-2.9, 1.35, 0], [0.12, 0.64, 0], [-1.3, 0.64, 0],
            fill_color=COLOR_G, fill_opacity=0.22,
            stroke_color=COLOR_G, stroke_opacity=0.4, stroke_width=1,
        )
        blocked_text = Text("Durch Überhang blockiert", font_size=BODY_FONT_SIZE, color=P_RED)
        blocked_text.move_to([-1.35, -0.05, 0])

        winter_beam = Polygon(
            [-2.9, 0.05, 0], [0, 0.6, 0], [2.7, -0.45, 0], [2.7, -1.75, 0], [0, -0.95, 0],
            fill_color=COLOR_G, fill_opacity=0.22,
            stroke_color=COLOR_G, stroke_opacity=0.4, stroke_width=1,
        )
        # Shortened, not just repositioned: at this label size "Dringt in
        # Wohnraum ein" is wider than the beam wedge itself (the wedge's own
        # widest cross-section is 2.7 units; the full sentence render past
        # 3.7), so no placement inside the wedge could avoid crossing one of
        # its slanted edges or the vertical right edge. "Dringt ein" fits.
        enters_text = Text("Dringt ein", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        enters_text.move_to([1.7, -1.05, 0])
        # Placed after _fit_stage (below) — stacked above the scaled top_wall but
        # clamped clear of the subtitle. Pre-fit placement pushed them into the
        # heading once the room was scaled up.
        fsh_label = Text("F_sh", font_size=BODY_FONT_SIZE, color=COLOR_FSH)
        fsh_desc = Text("F_sh [-] DIN 4108-2", font_size=BODY_FONT_SIZE, color=COLOR_FSH)

        stage = VGroup(
            top_wall, bottom_wall, glass_window, floor, ceiling, interior_label, awning,
            sun, winter_anchor, summer_beam, blocked_text,
            winter_beam, enters_text,
        )
        for m in (
            summer_beam, blocked_text, winter_beam,
            enters_text, fsh_label, fsh_desc,
        ):
            m.set_opacity(0)
        # Soft sun: glow 0.2 — never FadeIn the whole group (that flattens opacities).
        sun_core.set_fill(COLOR_G, opacity=1.0)
        sun_glow.set_fill(COLOR_G, opacity=0)
        sun_halo.set_stroke(opacity=0)
        sun_rays_grp.set_stroke(opacity=0)
        # Cap the scale so the room (bottom_wall at DOWN*1.35, floor at DOWN*1.75)
        # and the F_sh labels above top_wall stay inside the free band instead of
        # crossing the formula panel below and the subtitle above.
        _fit_stage(
            stage, below=subtitle,
            focus=VGroup(top_wall, bottom_wall, glass_window, floor, ceiling, awning, sun),
            scale_max=1.0,
        )
        # Room on the left for type-scale sun labels (not stage-scaled).
        label_margin = max(sun_label.width, winter_sun_label.width) + 2.2
        stage_left_need = -6.55 + label_margin
        if sun.get_left()[0] < stage_left_need:
            stage.shift(RIGHT * (stage_left_need - sun.get_left()[0]))

        # Keep callout clear of awning / glass after stage scale.
        blocked_text.next_to(awning, DOWN, buff=0.7)
        blocked_text.shift(LEFT * 0.75)
        if blocked_text.get_right()[0] > glass_window.get_center()[0] - 0.45:
            blocked_text.set_x(glass_window.get_center()[0] - 0.5 - blocked_text.width / 2)

        # Labels stay at type-scale size and clear of the sun disc+glow.
        def _place_sun_label(label, at_mob):
            # Inflated clearance: soft glow reads larger than the Dot bbox.
            cx, cy, _ = at_mob.get_center()
            clear = max(sun.width, sun.height) * 1.4 + 0.55
            # Prefer above-left of the disc.
            label.move_to([
                cx - clear - label.width / 2,
                cy + clear * 0.7 + label.height / 2,
                0,
            ])
            if label.get_top()[1] > subtitle.get_bottom()[1] - 0.18:
                # Not enough headroom — hard left of the disc.
                label.move_to([cx - clear - label.width / 2, cy, 0])
            if label.get_left()[0] < -6.85:
                label.set_x(-6.55 + label.width / 2)
                # Keep clear even if the frame edge is tight.
                if label.get_right()[0] > cx - clear:
                    label.shift(UP * (clear * 0.7 + label.height * 0.35))
                    if label.get_top()[1] > subtitle.get_bottom()[1] - 0.12:
                        label.set_y(subtitle.get_bottom()[1] - 0.14 - label.height / 2)
                    if label.get_right()[0] > cx - clear * 0.9:
                        label.set_x(cx - clear * 0.9 - label.width / 2)
            # Final hard rule: label box must not enter the inflated sun circle.
            while (
                np.linalg.norm(label.get_right() - at_mob.get_center()) < clear
                or np.linalg.norm(label.get_center() - at_mob.get_center()) < clear + label.width * 0.25
            ):
                label.shift(LEFT * 0.2 + UP * 0.05)
                if label.get_left()[0] < -6.9:
                    break

        _place_sun_label(sun_label, sun)
        winter_sun_label.set_opacity(0)
        self.add(sun_label)

        # F_sh callout (type scale — not stage-scaled): above the scaled top_wall
        # when it fits under the subtitle there, otherwise in the open band above
        # the ceiling on the room side. The old spot ran into the heading once
        # the room grew.
        fsh_group = VGroup(fsh_desc, fsh_label).arrange(DOWN, buff=0.08)
        fsh_group.next_to(top_wall, UP, buff=0.16)
        if fsh_group.get_top()[1] > subtitle.get_bottom()[1] - 0.22:
            fsh_group.next_to(ceiling, UP, buff=0.24).align_to(ceiling, RIGHT).shift(LEFT * 0.1)

        self.play(
            Create(top_wall), Create(bottom_wall), Create(glass_window),
            Create(floor), Create(ceiling), FadeIn(interior_label),
            Create(awning), FadeIn(sun_label),
            FadeIn(sun_core),
            sun_glow.animate.set_fill(COLOR_G, opacity=0.2),
            sun_halo.animate.set_stroke(opacity=0.75),
            sun_rays_grp.animate.set_stroke(opacity=0.9),
            run_time=1.4,
        )
        summer_beam.set_fill(COLOR_G, opacity=0)
        summer_beam.set_stroke(COLOR_G, width=1, opacity=0)
        blocked_text.set_opacity(1)
        self.play(
            sun_rays_grp.animate.rotate(0.35),
            summer_beam.animate.set_fill(COLOR_G, opacity=0.22).set_stroke(COLOR_G, width=1, opacity=0.4),
            run_time=0.9,
        )
        self.play(FadeIn(blocked_text), run_time=0.5)
        hold_for(self, self.NARRATION, "awning", used=1.4 + 0.9 + 0.5 + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "winter"))
        # Move sun first, then park Wintersonne — avoids Transform morphing into the disc.
        self.play(
            FadeOut(summer_beam), FadeOut(blocked_text), FadeOut(sun_label),
            sun.animate.move_to(winter_anchor.get_center()),
            run_time=1.1,
        )
        self.remove(summer_beam, blocked_text, sun_label)
        _place_sun_label(winter_sun_label, sun)
        winter_sun_label.set_opacity(0)
        winter_beam.set_fill(COLOR_G, opacity=0.22)
        winter_beam.set_stroke(COLOR_G, width=1, opacity=0.4)
        enters_text.set_opacity(1)
        self.add(winter_sun_label)
        self.play(
            winter_sun_label.animate.set_opacity(1),
            sun_rays_grp.animate.rotate(-0.35),
            FadeIn(winter_beam),
            run_time=0.9,
        )
        self.play(FadeIn(enters_text), run_time=0.5)
        hold_for(self, self.NARRATION, "winter", used=1.1 + 0.9 + 0.5 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fsh"))
        self.play(FadeOut(winter_beam), FadeOut(enters_text), FadeOut(winter_sun_label), run_time=0.7)
        fsh_label.set_opacity(1)
        fsh_desc.set_opacity(1)
        self.play(FadeIn(fsh_label), FadeIn(fsh_desc), run_time=0.6)

        row, items = equation_row([
            ("g", "G", COLOR_G),
            (None, "·", P_WHITE),
            ("a", "A", COLOR_A),
            (None, "·", P_WHITE),
            ("ff", "F_f", COLOR_FF),
            (None, "·", P_WHITE),
            ("gval", "g", COLOR_GVAL),
            (None, "·", P_WHITE),
            ("fsh", "F_sh", COLOR_FSH),
            (None, "  [W]", P_WHITE),
        ])
        row, box = formula_panel(row, edge_buff=FORMULA_EDGE_BUFF)
        self.play(
            FadeOut(fsh_desc), FadeOut(fsh_label),
            FadeIn(row), Create(box),
            run_time=1.0,
        )
        ring = highlight_param(items, "fsh", color=COLOR_FSH)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, self.NARRATION, "fsh", used=0.7 + 0.6 + 1.0 + 0.4 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 — Wärmespeicherung
class Beat6_Waermespeicherung(Scene):
    """🧱 Floor slab stores winter sun by day and radiates at night."""

    NARRATION = [
        ("day",
         "Dense floor slab absorbs winter sun through the window.",
         "Die dichte Bodenplatte absorbiert Wintersonne durchs Fenster."),
        ("night",
         "At night the stored heat radiates back into the room.",
         "Nachts strahlt die gespeicherte Wärme zurück in den Raum."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmespeicherung und Strahlung", title)
        din = _din_ref("DIN EN ISO 13786")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "day"))
        self.play(FadeIn(caption), run_time=0.3)

        phase_1 = Text("Tag: Absorption", font_size=BODY_FONT_SIZE, color=COLOR_G)
        phase_2 = Text("Nacht: Abstrahlung", font_size=BODY_FONT_SIZE, color=COLOR_HEAT)
        # Type-scale phase titles under the subtitle — not scaled with the room.
        phase_1.next_to(subtitle, DOWN, buff=0.4)
        phase_2.move_to(phase_1.get_center())

        floor_slab = Rectangle(
            width=5.8, height=0.55, color=COLOR_SLAB,
            fill_color=COLOR_SLAB, fill_opacity=0.85, stroke_width=2,
        ).move_to([0, -1.2, 0])
        floor_label = Text("Betonbodenplatte", font_size=BODY_FONT_SIZE, color=P_WHITE)

        left_wall = Line([-2.9, 0.85, 0], [-2.9, -0.95, 0], color="#E2E8F0", stroke_width=3)
        ceiling = Line([-2.9, 0.85, 0], [2.9, 0.85, 0], color="#E2E8F0", stroke_width=3)
        right_wall_top = Line([2.9, 0.85, 0], [2.9, 0.35, 0], color="#E2E8F0", stroke_width=3)
        right_wall_bot = Line([2.9, -0.35, 0], [2.9, -0.95, 0], color="#E2E8F0", stroke_width=3)
        window = Rectangle(
            width=0.1, height=0.7, color=COLOR_A,
            fill_color=COLOR_A, fill_opacity=0.5, stroke_width=1.5,
        ).move_to([2.9, 0.0, 0])
        room_group = VGroup(left_wall, ceiling, right_wall_top, right_wall_bot, window)

        sun_center = np.array([4.1, 0.15, 0])
        sun = Dot(point=sun_center, radius=0.3, color=COLOR_G)
        sun_glow = Circle(radius=0.42, color=COLOR_G, stroke_width=1, stroke_opacity=0.4).move_to(sun_center)
        sun_label = Text("Wintersonne", font_size=BODY_FONT_SIZE, color=COLOR_G)
        sun_group = VGroup(sun, sun_glow)
        # One translucent beam wedge from the sun to the slab — the same
        # low-opacity solar fill the other scenes use for Strahlung (Beat 5's
        # summer/winter beams), not a fan of stroke lines.
        sun_beam = Polygon(
            sun_center, [-1.7, -0.95, 0], [1.2, -0.95, 0],
            fill_color=COLOR_G, fill_opacity=0.2,
            stroke_color=COLOR_G, stroke_opacity=0.35, stroke_width=1,
        )

        moon = Text("☾", font_size=FORMULA_FONT_SIZE, color="#94A3B8").move_to([4.1, 1.0, 0])
        moon_label = Text("Nachthimmel", font_size=BODY_FONT_SIZE, color="#94A3B8")
        night_group = VGroup(moon, moon_label)

        def make_wavy_line(x_pos):
            # Shortened from 18 steps (post-scale top kept reaching rad_label
            # regardless of how far up the label got pushed) to 11 — reads the
            # same as a heat-radiation squiggle, with real headroom above it.
            points = []
            for step in range(11):
                y = -0.95 + step * 0.09
                x = x_pos + 0.07 * np.sin(step * 0.6)
                points.append([x, y, 0])
            curve = VMobject()
            curve.set_points_smoothly([np.array(p) for p in points])
            curve.set_color(COLOR_HEAT)
            curve.set_stroke(width=2, opacity=0.8)
            return curve

        wavy_lines = VGroup(*[make_wavy_line(x) for x in [-2.1, -1.2, -0.3, 0.6, 1.5, 2.3]])
        rad_label = Text("Gespeicherte Wärme", font_size=BODY_FONT_SIZE, color=COLOR_HEAT)

        stage = VGroup(
            floor_slab, room_group,
            sun_group, sun_beam, night_group, wavy_lines,
        )
        sun_beam.set_fill(opacity=0)
        sun_beam.set_stroke(opacity=0)
        for m in (phase_2, night_group, wavy_lines):
            m.set_opacity(0)
        # Core opaque; glow ring soft — set targets before FadeIn.
        sun.set_fill(COLOR_G, opacity=1.0)
        sun_glow.set_stroke(opacity=0)
        # Normal scale; park building + related objects lower above the caption.
        stage.scale(1.2)
        core = VGroup(floor_slab, room_group)
        stage.shift([-core.get_center()[0], 0, 0])
        band_bot = max(CONTENT_BOTTOM_MIN, caption.get_top()[1] + 0.4)
        stage.shift(DOWN * (core.get_bottom()[1] - band_bot))
        # Phase titles sit just above the lowered room (type scale).
        phase_1.next_to(core, UP, buff=0.4)
        if phase_1.get_top()[1] > subtitle.get_bottom()[1] - 0.2:
            stage.shift(DOWN * (phase_1.get_top()[1] - (subtitle.get_bottom()[1] - 0.2)))
            phase_1.next_to(core, UP, buff=0.35)
            if core.get_bottom()[1] < band_bot:
                stage.shift(UP * (band_bot - core.get_bottom()[1]))
                phase_1.next_to(core, UP, buff=0.3)
        phase_2.move_to(phase_1.get_center())
        # Type-scale callouts (do not scale Text with the room).
        floor_label.move_to(floor_slab.get_center())
        sun_label.next_to(sun, UR, buff=0.18)
        moon_label.next_to(moon, UP, buff=0.12)
        rad_label.next_to(floor_slab, UP, buff=1.3)
        rad_label.set_opacity(0)
        sun_label.set_opacity(0)

        self.play(
            Create(room_group), Create(floor_slab), FadeIn(floor_label),
            FadeIn(sun), FadeIn(sun_label), FadeIn(phase_1),
            sun_glow.animate.set_stroke(opacity=0.4),
            run_time=1.6,
        )
        sun_beam.set_fill(COLOR_G, opacity=0.2)
        sun_beam.set_stroke(COLOR_G, width=1, opacity=0.35)
        self.play(GrowFromPoint(sun_beam, sun_center), run_time=1.2)
        self.play(
            floor_slab.animate.set_color(COLOR_HEAT).set_fill(COLOR_HEAT, opacity=0.9),
            run_time=1.8,
        )
        hold_for(self, self.NARRATION, "day", used=1.6 + 1.2 + 1.8 + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "night"))
        phase_2.set_opacity(1)
        night_group.set_opacity(1)
        self.play(
            FadeOut(sun_beam), FadeOut(sun_group), FadeOut(sun_label),
            ReplacementTransform(phase_1, phase_2),
            FadeIn(night_group),
            run_time=1.4,
        )
        wavy_lines.set_opacity(1)
        rad_label.set_opacity(1)
        self.play(
            LaggedStart(*[Create(wl) for wl in wavy_lines], lag_ratio=0.12),
            FadeIn(rad_label),
            run_time=1.8,
        )
        self.play(
            wavy_lines.animate.shift(UP * 0.12),
            floor_slab.animate.set_fill("#D97706", opacity=0.75),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "night", used=1.4 + 1.8 + 1.2 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 — Speichermasse Formel
class Beat7_SpeichermasseFormel(Scene):
    """🧮 Stored heat Q_speicher = m · c · ΔT [J]."""

    NARRATION = [
        ("formula",
         "Stored heat Q is mass times specific heat capacity times temperature rise — unit joule.",
         "Gespeicherte Wärme Q ist Masse mal Wärmekapazität mal Delta-T — Einheit Joule."),
        ("why",
         "Thermal mass softens peaks, banks solar gains, and cuts the heating load.",
         "Thermische Masse dämpft Spitzen, speichert Solargewinne und senkt die Heizlast."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Thermische Speichermasse Q_speicher", title)
        din = _din_ref("DIN EN ISO 13786")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(caption), run_time=0.3)

        desc_m = Text("m = Masse des Bauteils [kg]", font_size=BODY_FONT_SIZE, color=COLOR_MASS)
        desc_c = Text("c = spez. Wärmekapazität [J/(kg·K)]", font_size=BODY_FONT_SIZE, color=COLOR_C)
        desc_dt = Text("ΔT = Temperaturdifferenz [K]", font_size=BODY_FONT_SIZE, color=COLOR_G)
        desc_group = VGroup(desc_m, desc_c, desc_dt).arrange(DOWN, buff=0.34)

        why_body = VGroup(
            Text("Dämpft Spitzen", font_size=BODY_FONT_SIZE, color=GREY_A),
            Text("Speichert solare Gewinne", font_size=BODY_FONT_SIZE, color=GREY_A),
            Text("Senkt die Heizlast Q_h", font_size=BODY_FONT_SIZE, color=GREY_A),
        ).arrange(DOWN, buff=0.28)

        row, items = equation_row([
            ("q", "Q_speicher", COLOR_HEAT),
            (None, "=", P_WHITE),
            ("m", "m", COLOR_MASS),
            (None, "·", P_WHITE),
            ("c", "c", COLOR_C),
            (None, "·", P_WHITE),
            ("dt", "ΔT", COLOR_G),
            (None, "  [J]", P_WHITE),
        ])
        # Compact middle stack: three lines + equation, under the subtitle.
        stack = VGroup(desc_group, row).arrange(DOWN, buff=0.55)
        stack.next_to(subtitle, DOWN, buff=0.6)
        stack.set_x(0)
        # Drift a bit toward the vertical mid-band without dropping into the caption.
        mid_y = (subtitle.get_bottom()[1] + caption.get_top()[1]) / 2
        stack.shift(DOWN * np.clip(stack.get_center()[1] - mid_y, 0, 0.55))
        if stack.get_bottom()[1] < caption.get_top()[1] + 0.45:
            stack.shift(UP * (caption.get_top()[1] + 0.45 - stack.get_bottom()[1]))
        box = SurroundingRectangle(
            row, color=P_TEAL, buff=0.22, corner_radius=0.1, stroke_width=2,
        )
        why_body.move_to(desc_group.get_center())
        why_body.set_opacity(0)

        self.play(
            LaggedStart(*[FadeIn(d, shift=UP * 0.1) for d in desc_group], lag_ratio=0.18),
            run_time=1.2,
        )

        self.play(FadeIn(row), Create(box), run_time=1.0)
        ring_m = highlight_param(items, "m", color=COLOR_MASS)
        self.play(Create(ring_m), run_time=0.35)
        self.play(FadeOut(ring_m), run_time=0.2)
        ring_c = highlight_param(items, "c", color=COLOR_C)
        self.play(Create(ring_c), run_time=0.35)
        self.play(FadeOut(ring_c), run_time=0.2)
        ring_dt = highlight_param(items, "dt", color=COLOR_G)
        self.play(Create(ring_dt), run_time=0.35)
        hold_for(self, self.NARRATION, "formula", used=1.2 + 1.0 + 1.45 + 0.35)
        self.play(FadeOut(ring_dt), run_time=0.2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "why"))
        why_body.set_opacity(1)
        self.play(FadeOut(desc_group), FadeIn(why_body, shift=UP * 0.12), run_time=1.0)
        ring_q = highlight_param(items, "q", color=COLOR_HEAT)
        self.play(Create(ring_q), run_time=0.4)
        hold_for(self, self.NARRATION, "why", used=1.0 + 0.4 + 0.35)
        self.play(FadeOut(ring_q), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 — Hauptgleichung
class Beat8_Hauptgleichung(Scene):
    """☀️ Master equation Φ_solar = G · A · F_f · g · F_sh."""

    NARRATION = [
        ("assemble",
         "Put it together: Phi solar equals G times A times F_f times g times F_sh — unit watt.",
         "Zusammen: Phi-solar gleich G mal A mal F_f mal g mal F_sh — Einheit Watt."),
        ("ring",
         "Ring F_sh — shading decides how much sun actually counts.",
         "Markiere F_sh — Verschattung entscheidet, wie viel Sonne zählt."),
        ("store",
         "That solar power is not lost — the building's thermal mass banks it as stored heat.",
         "Diese solare Leistung geht nicht verloren — die thermische Masse speichert sie als Wärme."),
        ("mass",
         "Stored heat Q is mass m times specific heat capacity c times temperature rise delta T — in joule.",
         "Die gespeicherte Wärme Q ist Masse m mal spezifische Wärmekapazität c mal Temperaturanstieg Delta-T — in Joule."),
        ("release",
         "More mass flattens the daily temperature swing and returns the warmth after sunset.",
         "Mehr Masse glättet den Tagesgang der Temperatur und gibt die Wärme nach Sonnenuntergang zurück."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Hauptgleichung Φ_solar", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "assemble"))
        self.play(FadeIn(caption), run_time=0.3)

        tokens = VGroup(
            Text("G", font_size=FORMULA_FONT_SIZE, color=COLOR_G),
            Text("A", font_size=FORMULA_FONT_SIZE, color=COLOR_A),
            Text("F_f", font_size=FORMULA_FONT_SIZE, color=COLOR_FF),
            Text("g", font_size=FORMULA_FONT_SIZE, color=COLOR_GVAL),
            Text("F_sh", font_size=FORMULA_FONT_SIZE, color=COLOR_FSH),
        ).arrange(RIGHT, buff=0.85).move_to(ORIGIN)
        labs = VGroup(
            Text("G [W/m²]", font_size=BODY_FONT_SIZE, color=COLOR_G),
            Text("A [m²]", font_size=BODY_FONT_SIZE, color=COLOR_A),
            Text("F_f [-]", font_size=BODY_FONT_SIZE, color=COLOR_FF),
            Text("g [-]", font_size=BODY_FONT_SIZE, color=COLOR_GVAL),
            Text("F_sh [-]", font_size=BODY_FONT_SIZE, color=COLOR_FSH),
        )
        for lab, tok in zip(labs, tokens):
            lab.next_to(tok, DOWN, buff=0.2)

        stage = VGroup(tokens, labs)
        _fit_stage(stage, below=subtitle)

        self.play(
            LaggedStart(*[FadeIn(t, shift=DOWN * 0.15) for t in tokens], lag_ratio=0.12),
            LaggedStart(*[FadeIn(l) for l in labs], lag_ratio=0.12),
            run_time=1.4,
        )

        row, items = equation_row([
            ("phi", "Φ_solar", P_WHITE),
            (None, "=", P_WHITE),
            ("g", "G", COLOR_G),
            (None, "·", P_WHITE),
            ("a", "A", COLOR_A),
            (None, "·", P_WHITE),
            ("ff", "F_f", COLOR_FF),
            (None, "·", P_WHITE),
            ("gval", "g", COLOR_GVAL),
            (None, "·", P_WHITE),
            ("fsh", "F_sh", COLOR_FSH),
            (None, "  [W]", P_WHITE),
        ])
        # Beat7-style: formula in the mid-upper band, not the bottom panel.
        row.next_to(subtitle, DOWN, buff=0.95)
        row.set_x(0)
        if row.get_bottom()[1] < caption.get_top()[1] + 0.5:
            row.shift(UP * (caption.get_top()[1] + 0.5 - row.get_bottom()[1]))
        box = SurroundingRectangle(
            row, color=P_TEAL, buff=0.22, corner_radius=0.1, stroke_width=2,
        )
        # The tokens were built at FORMULA_FONT_SIZE but ``_fit_stage`` rescaled
        # the whole stage, so a bare ``move_to`` dropped them at the wrong size
        # over the equation glyphs — a visible jump when they were removed. Scale
        # each one to its target glyph's height as it travels so it lands exactly.
        self.play(
            FadeOut(labs),
            tokens[0].animate.scale(items["g"].height / tokens[0].height).move_to(items["g"].get_center()),
            tokens[1].animate.scale(items["a"].height / tokens[1].height).move_to(items["a"].get_center()),
            tokens[2].animate.scale(items["ff"].height / tokens[2].height).move_to(items["ff"].get_center()),
            tokens[3].animate.scale(items["gval"].height / tokens[3].height).move_to(items["gval"].get_center()),
            tokens[4].animate.scale(items["fsh"].height / tokens[4].height).move_to(items["fsh"].get_center()),
            FadeIn(row), Create(box),
            run_time=1.6,
        )
        self.remove(*tokens)
        hold_for(self, self.NARRATION, "assemble", used=1.4 + 1.6 + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ring"))
        ring = highlight_param(items, "fsh", color=COLOR_FSH)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "ring", used=0.5 + 0.35)
        self.play(FadeOut(ring), run_time=0.25)

        # ── Thermal mass: where that solar power is banked (Beat 6/7 payoff) ──
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "store"))
        self.play(
            VGroup(row, box).animate.next_to(subtitle, DOWN, buff=0.5).set_x(0),
            run_time=0.8,
        )

        store_note = Text(
            "↓ gespeichert in der thermischen Masse", font_size=BODY_FONT_SIZE, color=COLOR_HEAT,
        )
        store_note.next_to(box, DOWN, buff=0.28).set_x(0)

        row2, items2 = equation_row([
            ("q", "Q_speicher", COLOR_HEAT),
            (None, "=", P_WHITE),
            ("m", "m", COLOR_MASS),
            (None, "·", P_WHITE),
            ("c", "c", COLOR_C),
            (None, "·", P_WHITE),
            ("dt", "ΔT", COLOR_G),
            (None, "  [J]", P_WHITE),
        ])
        row2.next_to(store_note, DOWN, buff=0.3).set_x(0)
        box2 = SurroundingRectangle(
            row2, color=COLOR_HEAT, buff=0.22, corner_radius=0.1, stroke_width=2,
        )
        legend = VGroup(
            Text("m  Masse des Bauteils [kg]", font_size=LABEL_FONT_SIZE, color=COLOR_MASS),
            Text("c  spez. Wärmekapazität [J/(kg·K)]", font_size=LABEL_FONT_SIZE, color=COLOR_C),
            Text("ΔT  Temperaturanstieg [K]", font_size=LABEL_FONT_SIZE, color=COLOR_G),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.16)
        legend.next_to(row2, DOWN, buff=0.3).set_x(0)
        _mass_stack = VGroup(store_note, row2, box2, legend)
        if _mass_stack.get_bottom()[1] < caption.get_top()[1] + 0.3:
            _mass_stack.shift(UP * (caption.get_top()[1] + 0.3 - _mass_stack.get_bottom()[1]))
        legend.set_opacity(0)

        self.play(FadeIn(store_note, shift=DOWN * 0.1), run_time=0.6)
        self.play(FadeIn(row2), Create(box2), run_time=1.0)
        hold_for(self, self.NARRATION, "store", used=0.8 + 0.6 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "mass"))
        legend.set_opacity(1)
        self.play(
            LaggedStart(*[FadeIn(l, shift=RIGHT * 0.1) for l in legend], lag_ratio=0.2),
            run_time=1.1,
        )
        for key, col in (("m", COLOR_MASS), ("c", COLOR_C), ("dt", COLOR_G)):
            r = highlight_param(items2, key, color=col)
            self.play(Create(r), run_time=0.3)
            self.play(FadeOut(r), run_time=0.18)
        hold_for(self, self.NARRATION, "mass", used=1.1 + 3 * 0.48 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "release"))
        ring_q = highlight_param(items2, "q", color=COLOR_HEAT)
        self.play(Create(ring_q), run_time=0.4)
        hold_for(self, self.NARRATION, "release", used=0.4 + 0.35)
        self.play(FadeOut(ring_q), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
