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
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    solar_wave_ray, symbol_token, watt_anchor,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Kühllast mit Sonnenschutz"

# Mid-screen anchor for facade / charts / sections.
CONTENT_CENTER = UP * 0.25


#region Shared visual motifs

def _sun(pos, radius=0.4):
    """🌞 Layered sun disc with corona rings and burst spokes."""
    glow = Dot(pos, radius=radius * 1.6, color=P_YELLOW, fill_opacity=0.3)
    core = Dot(pos, radius=radius, color=P_YELLOW)
    ring1 = Circle(radius=radius * 1.95, color=P_YELLOW, stroke_width=2, stroke_opacity=0.55).move_to(pos)
    ring2 = Circle(radius=radius * 2.5, color=P_YELLOW, stroke_width=1.2, stroke_opacity=0.28).move_to(pos)
    burst = VGroup()
    for angle in np.linspace(0, TAU, 12, endpoint=False):
        d = np.array([np.cos(angle), np.sin(angle), 0.0])
        burst.add(Line(pos + d * radius * 1.3, pos + d * radius * 2.1, color=P_YELLOW, stroke_width=2))
    return VGroup(glow, core, ring1, ring2, burst)


def _build_window(center=ORIGIN, width=3.6, height=2.7, band=0.3, mullion=0.16, opening_pad=0.16):
    """🪟 Face-on window: dashed rough opening, opaque frame bands, transparent panes.

    Frame bands are fully opaque so incoming rays placed at a lower ``z_index`` are
    visually cut off wherever the frame blocks them.
    """
    cx, cy = float(center[0]), float(center[1])
    x0, x1 = cx - width / 2, cx + width / 2
    y0, y1 = cy - height / 2, cy + height / 2
    inner_h = height - 2 * band
    pane_w = (width - 2 * band - mullion) / 2

    opening = DashedVMobject(
        Rectangle(
            width=width + 2 * opening_pad, height=height + 2 * opening_pad,
            color=P_BLUE, stroke_width=3,
        ).move_to(center),
        num_dashes=44,
    )

    def _band(w, h, pos):
        return Rectangle(
            width=w, height=h, color=P_WHITE, stroke_width=2,
            fill_color=P_WHITE, fill_opacity=1.0,
        ).move_to(pos)

    frame = VGroup(
        _band(width, band, np.array([cx, y1 - band / 2, 0.0])),
        _band(width, band, np.array([cx, y0 + band / 2, 0.0])),
        _band(band, inner_h, np.array([x0 + band / 2, cy, 0.0])),
        _band(band, inner_h, np.array([x1 - band / 2, cy, 0.0])),
        _band(mullion, inner_h, np.array([cx, cy, 0.0])),
    )

    def _pane(pos):
        return Rectangle(
            width=pane_w, height=inner_h, color=P_CYAN, stroke_width=2,
            fill_color=P_CYAN, fill_opacity=0.12,
        ).move_to(pos)

    panes = VGroup(
        _pane(np.array([cx - mullion / 2 - pane_w / 2, cy, 0.0])),
        _pane(np.array([cx + mullion / 2 + pane_w / 2, cy, 0.0])),
    )

    return {
        "opening": opening,
        "frame": frame,
        "panes": panes,
        "center": np.array([cx, cy, 0.0]),
        "x0": x0, "x1": x1, "y0": y0, "y1": y1,
        "band": band, "mullion": mullion,
        "width": width, "height": height,
    }


def _section_hatch(rect, spacing=0.2, color=P_WHITE, stroke_width=1.0, opacity=0.35):
    """〽️ 45° hatch clipped to a rectangle — the architectural 'cut through' convention."""
    x0, x1 = float(rect.get_left()[0]), float(rect.get_right()[0])
    y0, y1 = float(rect.get_bottom()[1]), float(rect.get_top()[1])
    lines = VGroup()
    c = x0 - y1
    while c <= x1 - y0:
        xs, xe = max(x0, c + y0), min(x1, c + y1)
        if xe - xs > 1e-3:
            lines.add(Line(
                np.array([xs, xs - c, 0.0]), np.array([xe, xe - c, 0.0]),
                color=color, stroke_width=stroke_width, stroke_opacity=opacity,
            ))
        c += spacing
    return lines


def _heat_wave(start, length=2.4, amp=0.12, cycles=4.0, color=P_RED, stroke_width=2.2):
    """🌡️ Horizontal sine ribbon used for secondary inward heat emission."""
    sx, sy = float(start[0]), float(start[1])
    pts = [
        np.array([sx + t * length, sy + amp * np.sin(cycles * TAU * t), 0.0])
        for t in np.linspace(0, 1, 40)
    ]
    wave = VMobject(color=color, stroke_width=stroke_width, stroke_opacity=0.85)
    wave.set_points_smoothly(pts)
    return wave


def _bell(x, x0, x1, amp):
    """📈 Half-sine daily irradiance profile between sunrise x0 and sunset x1."""
    if x <= x0 or x >= x1:
        return 0.0
    return amp * np.sin(PI * (x - x0) / (x1 - x0))

#endregion


#region Beat1 – Maximum Solar Irradiance (I_S,max)
class Beat1_SolarIrradiance(Scene):
    NARRATION = [
        ("intro",
         "Now we tackle the most significant summer heat source: direct solar radiation.",
         "Die größte sommerliche Wärmequelle: direkte Sonnenstrahlung."),
        ("irradiance",
         "Everything starts with the maximum solar irradiance, I S max, measured in watts per square meter.",
         "Alles beginnt mit I_S,max — der maximalen Bestrahlungsstärke in W/m²."),
        ("chart",
         "As the solar chart shows, this maximum value changes drastically depending on the time of day and whether the surface is horizontal, or facing North, South, East, or West.",
         "Die Kurve zeigt: der Höchstwert hängt von Tageszeit und Orientierung ab."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Direkte Sonnenstrahlung", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        # ── Facade with a large window (mid-screen, clear of formula/caption) ──
        fac_c = LEFT * 4.3 + CONTENT_CENTER + DOWN * 0.35
        facade = Rectangle(width=3.4, height=2.6, color=P_WHITE, stroke_width=3).move_to(fac_c)
        win = Rectangle(width=2.0, height=1.35, color=P_CYAN, stroke_width=3).move_to(fac_c + UP * 0.08)
        win_cross = VGroup(
            Line(win.get_top(), win.get_bottom(), color=P_CYAN, stroke_width=2),
            Line(win.get_left(), win.get_right(), color=P_CYAN, stroke_width=2),
        )
        ground = Line(fac_c + LEFT * 2.1 + DOWN * 1.3, fac_c + RIGHT * 2.1 + DOWN * 1.3,
                      color=P_TEAL, stroke_width=4)
        building = VGroup(ground, facade, win, win_cross)

        # Sun sits in the gap between the lowered facade (top y≈1.2) and the
        # content ceiling (2.62): radius 0.26 -> outer ring spans y 1.25-2.55.
        sun_pos = LEFT * 5.9 + UP * 1.9
        sun = _sun(sun_pos, radius=0.26)
        ray_targets = [
            win.get_center() + UP * 0.45 + LEFT * 0.7,
            win.get_center() + UP * 0.12,
            win.get_center() + DOWN * 0.4 + LEFT * 0.4,
            win.get_center() + UP * 0.35 + RIGHT * 0.6,
            win.get_center() + DOWN * 0.25 + RIGHT * 0.7,
        ]
        rays = VGroup(*[
            solar_wave_ray(sun_pos + (t - sun_pos) * 0.22, t, color=P_YELLOW, stroke_width=2.5, amp=0.08)
            for t in ray_targets
        ])

        irr_anchor = watt_anchor(800, compare="vacuum", title="I_S,max ≈ 800 W/m²")
        # Bottom-left: the old upper-left corner placement sat on top of the sun.
        # x≈-4.8 stays clear of the centred formula panel, y≈-2.0 of the caption.
        irr_anchor.scale(0.6).move_to(LEFT * 4.8 + DOWN * 2.0)

        axes = Axes(
            x_range=[6, 18, 3],
            y_range=[0, 900, 300],
            x_length=5.0,
            y_length=3.0,
            axis_config={"color": P_WHITE, "stroke_width": 2},
            tips=False,
        ).move_to(RIGHT * 3.7 + CONTENT_CENTER)

        x_labels = VGroup(*[
            Text(str(h), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(h, 0), DOWN, buff=0.14)
            for h in (6, 9, 12, 15, 18)
        ])
        y_labels = VGroup(*[
            Text(str(v), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(6, v), LEFT, buff=0.14)
            for v in (300, 600, 900)
        ])
        y_axis_name = Text("W/m²", font_size=BODY_FONT_SIZE, color=P_TEAL)
        y_axis_name.next_to(axes.c2p(6, 900), UP, buff=0.16).shift(LEFT * 0.15)
        x_axis_name = Text("Sonnenzeit", font_size=BODY_FONT_SIZE, color=P_TEAL)
        x_axis_name.next_to(axes.c2p(12, 0), DOWN, buff=0.42)

        specs = [
            ("Horiz", 5.0, 19.0, 860, P_YELLOW, (6, 18), (12, 860), UP * 0.2),
            ("O",     4.5, 13.0, 620, P_ORANGE, (6, 13), (8.0, 640), UP * 0.2),
            ("S",     7.0, 17.0, 520, P_CYAN,   (7, 17), (12, 520), UP * 0.24),
            ("W",     11.0, 19.5, 620, P_GREEN, (11, 18), (16.2, 615), UP * 0.2),
            ("N",     4.5, 19.5, 150, P_TEAL,   (6, 18), (16.5, 88), DOWN * 0.25),
        ]
        curves = {}
        curve_labels = {}
        for name, a, b, amp, color, (px0, px1), (lx, ly), off in specs:
            curves[name] = axes.plot(
                lambda x, a=a, b=b, amp=amp: _bell(x, a, b, amp),
                x_range=[px0, px1],
                color=color,
                stroke_width=2.8,
            )
            curve_labels[name] = Text(name, font_size=BODY_FONT_SIZE, color=color).move_to(axes.c2p(lx, ly) + off)

        eq_row, eq_items = equation_row([
            ("i", "I_S,max", P_YELLOW), (None, "=", P_WHITE),
            (None, "≈ 400–800", P_YELLOW), (None, "[W/m²]", P_TEAL),
        ])
        eq_row, eq_box = formula_panel(eq_row, color=P_YELLOW)

        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(building), run_time=1.6)
        self.play(FadeIn(sun, scale=0.7), run_time=1.0)
        self.play(LaggedStart(*[Create(r) for r in rays], lag_ratio=0.12), run_time=1.4)
        self.play(FadeIn(irr_anchor, shift=DOWN * 0.1), run_time=0.9)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "irradiance"))
        hold_for(self, self.NARRATION, "irradiance", used=4.9 + 0.35)

        self.play(Create(axes), run_time=1.2)
        self.play(
            LaggedStart(*[FadeIn(m) for m in (*x_labels, *y_labels, y_axis_name, x_axis_name)], lag_ratio=0.06),
            run_time=1.1,
        )
        self.play(
            LaggedStart(*[Create(curves[n]) for n, *_ in specs], lag_ratio=0.22),
            run_time=3.2,
        )
        self.play(
            LaggedStart(*[FadeIn(curve_labels[n]) for n, *_ in specs], lag_ratio=0.12),
            run_time=1.0,
        )

        self.play(Create(eq_box), FadeIn(eq_row), run_time=1.2)
        i_tok = symbol_token("I_S,max", color=P_YELLOW, font_size=FORMULA_FONT_SIZE)
        i_tok.move_to(sun.get_center())
        self.play(ReplacementTransform(sun.copy(), i_tok), run_time=1.0)
        self.play(i_tok.animate.move_to(eq_items["i"].get_center()), run_time=0.9)
        self.play(FadeOut(i_tok), run_time=0.35)

        for name in ("Horiz", "N", "S", "O", "W"):
            base_color = curves[name].get_stroke_color()
            self.play(
                curves[name].animate.set_stroke(color=P_YELLOW, width=5.5),
                curve_labels[name].animate.set_color(P_YELLOW).scale(1.2),
                run_time=0.55,
            )
            self.play(
                curves[name].animate.set_stroke(color=base_color, width=2.8),
                curve_labels[name].animate.scale(1 / 1.2),
                run_time=0.35,
            )

        ring = highlight_param(eq_items, "i", color=P_YELLOW)
        self.play(Create(ring), run_time=0.5)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "chart"))
        hold_for(self, self.NARRATION, "chart", used=9.55 + 0.5 + 0.35)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion



#region Beat2 – Gross Area & Frame Factor (A · F_F)
class Beat2_FrameFactor(Scene):
    NARRATION = [
        ("intro",
         "To calculate the cooling load, we start with the gross area of the window opening, A.",
         "Die Kühllast beginnt mit der Rohbauöffnung A."),
        ("frame",
         "However, glass doesn't cover the entire opening. We must multiply by F F, the dimensionless frame factor.",
         "Glas füllt die Öffnung nicht ganz — multipliziert mit dem Rahmenfaktor F_F."),
        ("aeff",
         "This mathematically isolates the effective transparent area by subtracting the opaque window frames that physically block the sun.",
         "So bleibt die transparente Restfläche A_eff — ohne den undurchsichtigen Rahmen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Fensterfläche und Rahmenfaktor", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        w = _build_window(center=LEFT * 3.4 + CONTENT_CENTER, width=3.6, height=2.5)
        w["panes"].set_z_index(2)
        w["frame"].set_z_index(3)

        area_label = Text("A: Rohbauöffnung", font_size=BODY_FONT_SIZE, color=P_BLUE)
        area_label.next_to(w["opening"], DOWN, buff=0.18)

        direction = np.array([1.0, -0.55, 0.0])
        direction = direction / np.linalg.norm(direction)
        frame_hits = [
            np.array([w["x0"] + 0.15, 0.55, 0.0]),
            np.array([-3.4, 0.65, 0.0]),
            np.array([-4.2, w["y1"] - 0.15, 0.0]),
            np.array([-2.6, w["y0"] + 0.15, 0.0]),
        ]
        glass_hits = [
            np.array([-4.5, 0.75, 0.0]),
            np.array([-4.0, 0.05, 0.0]),
            np.array([-4.6, -0.65, 0.0]),
            np.array([-2.9, 0.6, 0.0]),
            np.array([-2.3, -0.1, 0.0]),
            np.array([-2.6, 0.92, 0.0]),
        ]

        def _incoming(hit):
            return Line(
                hit - direction * 2.15, hit,
                color=P_YELLOW, stroke_width=2.5, stroke_opacity=0.85,
            ).set_z_index(1)

        frame_rays = VGroup(*[_incoming(h) for h in frame_hits])
        glass_rays = VGroup(*[_incoming(h) for h in glass_hits])
        through_rays = VGroup(*[
            Line(
                h, h + direction * 1.35,
                color=P_YELLOW, stroke_width=2.5, stroke_opacity=0.85,
            ).set_z_index(4)
            for h in glass_hits
        ])

        eq_row, eq_items = equation_row([
            ("aeff", "A_eff", P_CYAN), (None, "=", P_WHITE), ("a", "A", P_BLUE),
            (None, "·", P_WHITE), ("ff", "F_F", P_WHITE),
            (None, "  [m²]", P_TEAL),
        ])
        eq_row, panel_box = formula_panel(eq_row, color=P_TEAL)

        unit_a = Text("[m²]", font_size=LABEL_FONT_SIZE, color=P_BLUE).next_to(eq_items["a"], UP, buff=0.12)
        unit_ff = Text("[-]", font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(eq_items["ff"], UP, buff=0.12)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(w["opening"]), FadeIn(area_label, shift=UP * 0.15), run_time=1.4)
        self.play(w["opening"].animate.set_stroke(color=P_BLUE, width=5), rate_func=there_and_back, run_time=0.9)

        self.play(FadeIn(w["frame"], scale=0.9), run_time=1.3)
        self.play(FadeIn(w["panes"]), run_time=0.7)

        self.play(FadeOut(area_label), Create(panel_box), FadeIn(eq_row), FadeIn(unit_a), FadeIn(unit_ff), run_time=1.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "frame"))

        self.play(
            LaggedStart(*[Create(r) for r in (*glass_rays, *frame_rays)], lag_ratio=0.08),
            run_time=1.8,
        )
        self.play(
            LaggedStart(*[Create(r) for r in through_rays], lag_ratio=0.08),
            frame_rays.animate.set_stroke(color="#5A6472", opacity=0.35),
            run_time=1.6,
        )

        ring = highlight_param(eq_items, "ff", color=P_ORANGE)
        self.play(Create(ring), eq_items["ff"].animate.set_color(P_ORANGE), run_time=0.7)
        hold_for(self, self.NARRATION, "frame", used=6.8 + 0.35)
        self.play(FadeOut(ring), run_time=0.3)

        ring = highlight_param(eq_items, "aeff", color=P_CYAN)
        self.play(
            Create(ring),
            w["panes"].animate.set_fill(opacity=0.34),
            run_time=0.8,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "aeff"))
        hold_for(self, self.NARRATION, "aeff", used=0.8 + 0.35)
        self.play(FadeOut(ring), FadeOut(caption), FadeOut(unit_a), FadeOut(unit_ff), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Shading Factor (F_V)
class Beat3_ShadingFactor(Scene):
    """🪟 Vertical section: without shading the full beam reaches the glass; Raffstore reflects heat outside."""

    NARRATION = [
        ("intro",
         "Next, we deploy our sun protection. This is a vertical section through the facade.",
         "Als Nächstes der Sonnenschutz — Vertikalschnitt durch die Fassade."),
        ("ismax",
         "I S max is the maximum solar irradiance on the facade — the starting intensity in watts per square meter, before any shading.",
         "I_S,max ist die maximale Bestrahlungsstärke auf die Fassade — der Ausgangswert in W/m² vor Verschattung."),
        ("unshaded",
         "With no shading, the full beam strikes the glass and passes straight into the room: a shading factor, F V, of one point zero.",
         "Ohne Schutz trifft die volle Strahlung auf die Scheibe: F_V = 1,0."),
        ("raffstore",
         "Now an external Raffstore drops in front. Its slats intercept the beam and reflect the heat back outside, before it ever reaches the glass.",
         "Ein außenliegender Raffstore fängt den Strahl ab und reflektiert die Wärme."),
        ("reduced",
         "Only a small residual gets through, so F V falls to about zero point one five.",
         "Nur ein Restanteil kommt durch — F_V sinkt auf etwa 0,15."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Verschattungsfaktor", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        glass_out, glass_in = -3.5, -3.3
        wall_top, wall_bottom = 1.65, -0.95
        lintel = Rectangle(
            width=0.5, height=0.7, color=P_WHITE, stroke_width=2.5,
            fill_color="#161A21", fill_opacity=1.0,
        ).move_to(np.array([-3.4, wall_top + 0.35, 0.0]))
        sill = Rectangle(
            width=0.5, height=0.7, color=P_WHITE, stroke_width=2.5,
            fill_color="#161A21", fill_opacity=1.0,
        ).move_to(np.array([-3.4, wall_bottom - 0.35, 0.0]))
        masonry = VGroup(
            lintel, sill,
            _section_hatch(lintel), _section_hatch(sill),
        ).set_z_index(3)

        glazing = VGroup(*[
            Line(np.array([x, wall_bottom, 0.0]), np.array([x, wall_top, 0.0]),
                 color=P_CYAN, stroke_width=3)
            for x in (glass_out, glass_in)
        ]).set_z_index(3)

        ceiling = Line(np.array([-3.15, 2.45, 0.0]), np.array([-0.7, 2.45, 0.0]),
                       color=P_WHITE, stroke_width=2.5)
        floor = Line(np.array([-3.15, -1.45, 0.0]), np.array([-0.7, -1.45, 0.0]),
                     color=P_TEAL, stroke_width=4)
        inner_wall = Line(np.array([-0.7, -1.45, 0.0]), np.array([-0.7, 2.45, 0.0]),
                          color=P_WHITE, stroke_width=2.5)
        section = VGroup(masonry, glazing, ceiling, floor, inner_wall)

        lbl_out = Text("Außen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(np.array([-6.2, -0.7, 0.0]))
        lbl_in = Text("Innen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(np.array([-1.5, 1.85, 0.0]))
        lbl_glass = Text("Verglasung", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([-2.6, -1.65, 0.0]))

        d = np.array([0.75, -0.661, 0.0])
        sun = _sun(np.array([-6.3, 1.85, 0.0]), radius=0.26)

        slat_x = -4.4
        slat_ys = [1.45 - i * 0.35 for i in range(8)]
        aimed_ys = slat_ys[:6]
        glass_ys = [y - 0.793 for y in aimed_ys]

        def _ray(end, back=3.0, **kwargs):
            return Line(end - d * back, end, color=P_YELLOW, **kwargs)

        direct = VGroup(*[
            _ray(np.array([glass_out, y, 0.0]), stroke_width=2.6, stroke_opacity=0.9).set_z_index(1)
            for y in glass_ys
        ])
        blocked = VGroup(*[
            _ray(np.array([glass_out, y, 0.0]), back=3.0 - 1.316,
                 stroke_width=2.6, stroke_opacity=0.9).shift(-d * 1.316).set_z_index(1)
            for y in glass_ys
        ])

        room_end_x, floor_y = -0.7, -1.45

        def _through_end(start):
            t_floor = (floor_y - start[1]) / d[1]
            t_wall = (room_end_x - start[0]) / d[0]
            t = min(t for t in (t_floor, t_wall) if t > 0.05)
            return start + d * t

        interior = VGroup(*[
            Line(
                np.array([glass_in, y, 0.0]),
                _through_end(np.array([glass_in, y, 0.0])),
                color=P_YELLOW, stroke_width=2.6, stroke_opacity=0.85,
            ).set_z_index(1)
            for y in glass_ys[:5]
        ])

        u = np.array([np.cos(48 * DEGREES), np.sin(48 * DEGREES), 0.0])
        n = np.array([-u[1], u[0], 0.0])
        r = d - 2 * float(np.dot(d, n)) * n

        slats = VGroup(*[
            Rectangle(
                width=0.52, height=0.08, color=P_TEAL, stroke_width=1.5,
                fill_color=P_TEAL, fill_opacity=0.8,
            ).move_to(np.array([slat_x, y, 0.0])).rotate(48 * DEGREES)
            for y in slat_ys
        ])
        rail = Rectangle(
            width=0.62, height=0.26, color=P_TEAL, stroke_width=2,
            fill_color=P_TEAL, fill_opacity=0.9,
        ).move_to(np.array([slat_x, 1.85, 0.0]))
        bracket = Line(np.array([slat_x + 0.31, 1.85, 0.0]), np.array([-3.65, 1.85, 0.0]),
                       color=P_TEAL, stroke_width=2.5)
        blind = VGroup(rail, bracket, slats).set_z_index(4)
        lbl_blind = Text("Raffstore", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([-4.95, -1.55, 0.0]))

        reflected = VGroup(*[
            Arrow(
                np.array([-4.487, y + 0.077, 0.0]),
                np.array([-4.487, y + 0.077, 0.0]) + r * 1.45,
                buff=0, color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.55,
                max_tip_length_to_length_ratio=0.16,
            ).set_z_index(5)
            for y in aimed_ys
        ])
        def _floor_end(start):
            return start + d * ((floor_y - start[1]) / d[1])

        residual = VGroup(*[
            Line(
                np.array([-4.226, y + 0.193, 0.0]),
                _floor_end(np.array([-4.226, y + 0.193, 0.0])),
                color=P_YELLOW, stroke_width=1.4, stroke_opacity=0.32,
            ).set_z_index(1)
            for y in aimed_ys[:5]
        ])
        lbl_rest = Text("Restanteil", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([-1.25, -0.15, 0.0]))

        eq_row, eq_items = equation_row([
            ("ired", "I_reduziert", P_TEAL), (None, "=", P_WHITE), ("i", "I_S,max", P_YELLOW),
            (None, "·", P_WHITE), ("fv", "F_V", P_TEAL),
            (None, "  [W/m²]", P_TEAL),
        ])
        eq_row, eq_box = formula_panel(eq_row, color=P_TEAL, edge_buff=1.15)

        sx0, sx1, sy = 1.5, 6.0, 0.45
        scale_line = Line(np.array([sx0, sy, 0.0]), np.array([sx1, sy, 0.0]), color=P_WHITE, stroke_width=2)
        tick_l = Line(np.array([sx0, sy - 0.12, 0.0]), np.array([sx0, sy + 0.12, 0.0]), color=P_WHITE, stroke_width=2)
        tick_r = Line(np.array([sx1, sy - 0.12, 0.0]), np.array([sx1, sy + 0.12, 0.0]), color=P_WHITE, stroke_width=2)
        end_l = Text("0,1\naußenliegend", font_size=LABEL_FONT_SIZE, color=P_TEAL, line_spacing=0.8)
        end_l.next_to(tick_l, DOWN, buff=0.12)
        end_r = Text("1,0\nohne Schutz", font_size=LABEL_FONT_SIZE, color=P_YELLOW, line_spacing=0.8)
        end_r.next_to(tick_r, DOWN, buff=0.12)
        scale_title = Text("Bandbreite F_V", font_size=BODY_FONT_SIZE, color=P_WHITE)
        scale_title.next_to(scale_line, UP, buff=0.85)

        def _sx(fv):
            return sx0 + (fv - 0.1) / 0.9 * (sx1 - sx0)

        marker = Triangle(color=P_YELLOW, fill_color=P_YELLOW, fill_opacity=1.0, stroke_width=0)
        marker.scale(0.16).rotate(PI).move_to(np.array([_sx(1.0), sy + 0.24, 0.0]))
        marker_val = Text("F_V = 1,0", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        marker_val.next_to(marker, UP, buff=0.1)
        marker_val_new = Text("F_V = 0,15", font_size=BODY_FONT_SIZE, color=P_TEAL)
        marker_val_new.next_to(np.array([_sx(0.15), sy + 0.4, 0.0]), UP, buff=0.1)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(section), run_time=1.8)
        self.play(
            LaggedStart(FadeIn(lbl_out), FadeIn(lbl_in), FadeIn(lbl_glass), lag_ratio=0.2),
            run_time=1.0,
        )
        self.play(Create(eq_box), FadeIn(eq_row), run_time=1.2)
        ring_i = highlight_param(eq_items, "i", color=P_YELLOW)
        self.play(Create(ring_i), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ismax"))
        hold_for(self, self.NARRATION, "ismax", used=1.2 + 0.45 + 0.35)
        self.play(FadeOut(ring_i), run_time=0.25)

        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(LaggedStart(*[Create(ray) for ray in direct], lag_ratio=0.08), run_time=1.6)
        self.play(LaggedStart(*[Create(ray) for ray in interior], lag_ratio=0.1), run_time=1.4)
        self.play(
            Create(scale_line), Create(tick_l), Create(tick_r),
            FadeIn(end_l), FadeIn(end_r), FadeIn(scale_title),
            FadeIn(marker, shift=DOWN * 0.2), FadeIn(marker_val),
            run_time=1.4,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unshaded"))
        hold_for(self, self.NARRATION, "unshaded", used=9.1 + 0.35)

        blind.shift(UP * 3.0).set_opacity(0)
        self.add(blind)
        self.play(blind.animate.shift(DOWN * 3.0).set_opacity(1.0), run_time=1.5)
        self.play(FadeIn(lbl_blind, shift=UP * 0.15), run_time=0.5)

        self.play(
            *[Transform(ray, cut) for ray, cut in zip(direct, blocked)],
            LaggedStart(*[GrowArrow(a) for a in reflected], lag_ratio=0.08),
            run_time=1.8,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "raffstore"))
        hold_for(self, self.NARRATION, "raffstore", used=3.8 + 0.35)

        self.play(
            FadeOut(interior),
            LaggedStart(*[Create(ray) for ray in residual], lag_ratio=0.1),
            FadeIn(lbl_rest),
            run_time=1.6,
        )
        self.play(
            marker.animate.move_to(np.array([_sx(0.15), sy + 0.24, 0.0])).set_color(P_TEAL),
            ReplacementTransform(marker_val, marker_val_new),
            run_time=1.8,
        )
        ring = highlight_param(eq_items, "fv", color=P_TEAL)
        self.play(Create(ring), run_time=0.7)
        self.play(Indicate(eq_items["ired"], color=P_TEAL, scale_factor=1.12), run_time=0.8)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "reduced"))
        hold_for(self, self.NARRATION, "reduced", used=4.9 + 0.35)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Total Solar Energy Transmittance (g_tot)
class Beat4_GlassTransmittance(Scene):
    NARRATION = [
        ("intro",
         "Finally, the remaining light hits the glass pane itself.",
         "Zuletzt trifft das Restlicht auf die Glasscheibe selbst."),
        ("gtot",
         "We multiply by g tot, the total solar energy transmittance.",
         "Wir multiplizieren mit g_tot — dem Gesamtenergiedurchlassgrad."),
        ("parts",
         "Academically, this is the sum of direct solar transmission, tau e, and the secondary inward heat emission, q i, from the glass absorbing the radiation.",
         "Das ist die Summe aus direkter Transmission τ_e und sekundärer Wärmeabgabe q_i."),
        ("meaning",
         "It tells us exactly what fraction of that heat successfully penetrates into the room.",
         "Er sagt, welcher Anteil der Wärme tatsächlich in den Raum gelangt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Gesamtenergiedurchlassgrad", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        pane_x, pane_top, pane_bottom = -1.5, 2.15, -0.95
        pane_w = 0.36
        pane = Rectangle(
            width=pane_w, height=pane_top - pane_bottom, color=P_CYAN, stroke_width=3,
            fill_color=P_CYAN, fill_opacity=0.18,
        ).move_to(np.array([pane_x, (pane_top + pane_bottom) / 2, 0.0]))
        pane_label = Text("Verglasung im Schnitt", font_size=BODY_FONT_SIZE, color=P_CYAN)
        pane_label.move_to(np.array([pane_x, 2.35, 0.0]))

        outside = Text("Außen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(LEFT * 5.6 + UP * 1.6)
        inside = Text("Innen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(RIGHT * 4.8 + UP * 1.6)

        hit_out = np.array([pane_x - pane_w / 2, 1.45, 0.0])
        hit_in = np.array([pane_x + pane_w / 2, 1.45, 0.0])
        d_in = np.array([3.52, -1.5, 0.0])
        d_in = d_in / np.linalg.norm(d_in)
        d_ref = np.array([-d_in[0], d_in[1], 0.0])

        incoming = Line(hit_out - d_in * 3.83, hit_out, color=P_YELLOW, stroke_width=3)
        reflected = Line(hit_out, hit_out + d_ref * 2.8, color=P_WHITE, stroke_width=2.5, stroke_opacity=0.7)
        transmitted = Line(hit_in, hit_in + d_in * 4.0, color=P_YELLOW, stroke_width=3)

        lbl_refl = Text("Reflexion", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(LEFT * 4.55 + UP * 0.15)
        lbl_tau = Text("τ_e", font_size=FORMULA_FONT_SIZE, color=P_YELLOW).move_to(np.array([0.99, 1.05, 0.0]))
        lbl_qi = Text("q_i", font_size=FORMULA_FONT_SIZE, color=P_RED).move_to(np.array([3.05, -0.45, 0.0]))

        waves = VGroup(*[
            _heat_wave(np.array([pane_x + pane_w / 2 + 0.05, y, 0.0]), length=3.2, color=P_RED)
            for y in (0.05, -0.45, -0.85)
        ])

        merge_point = np.array([3.4, 0.25, 0.0])
        merge_glow = VGroup(
            Dot(merge_point, radius=0.5, color=P_ORANGE, fill_opacity=0.12),
            Dot(merge_point, radius=0.3, color=P_ORANGE, fill_opacity=0.25),
            Dot(merge_point, radius=0.15, color=P_ORANGE, fill_opacity=0.5),
        )

        eq_row, eq_items = equation_row([
            ("g", "g_tot", P_RED), (None, "=", P_WHITE), ("tau", "τ_e", P_YELLOW),
            (None, "+", P_WHITE), ("qi", "q_i", P_RED),
            (None, "  [-]", P_TEAL),
        ])
        eq_row, eq_box = formula_panel(eq_row, color=P_RED)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(pane), FadeIn(pane_label), run_time=1.2)
        self.play(FadeIn(outside), FadeIn(inside), run_time=0.6)

        self.play(Create(incoming), run_time=1.0)
        self.play(Create(reflected), FadeIn(lbl_refl), run_time=1.0)
        self.play(Create(transmitted), FadeIn(lbl_tau), run_time=1.2)

        self.play(
            pane.animate.set_fill(color=P_RED, opacity=0.45).set_stroke(color=P_RED),
            run_time=1.4,
        )
        self.play(
            LaggedStart(*[Create(wv) for wv in waves], lag_ratio=0.15),
            FadeIn(lbl_qi),
            run_time=1.8,
        )

        self.play(Create(eq_box), FadeIn(eq_row), run_time=1.2)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gtot"))
        hold_for(self, self.NARRATION, "gtot", used=9.4 + 0.35)

        tau_copy = lbl_tau.copy()
        qi_copy = lbl_qi.copy()
        self.add(tau_copy, qi_copy)
        # Converge side by side, not onto the same point — stacking both tokens
        # on merge_point left them superimposed and unreadable for the whole
        # fade that follows.
        self.play(
            tau_copy.animate.scale(0.7).move_to(merge_point + LEFT * 0.34),
            qi_copy.animate.scale(0.7).move_to(merge_point + RIGHT * 0.34),
            FadeIn(merge_glow),
            run_time=1.6,
        )
        self.play(
            FadeOut(tau_copy), FadeOut(qi_copy),
            merge_glow.animate.move_to(eq_items["g"].get_center()).scale(0.8),
            run_time=1.2,
        )
        ring = highlight_param(eq_items, "g", color=P_ORANGE)
        self.play(Create(ring), run_time=0.5)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "parts"))
        hold_for(self, self.NARRATION, "parts", used=3.3 + 0.35)
        self.play(FadeOut(merge_glow), FadeOut(ring), run_time=0.5)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meaning"))
        hold_for(self, self.NARRATION, "meaning", used=0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Final Solar Cooling Load (Q̇_S,tr)
class Beat5_SolarCoolingLoad(Scene):
    NARRATION = [
        ("intro",
         "By multiplying the raw solar irradiance by our building's gross area, and then applying our three dimensionless reduction filters, the frame factor, the shading factor, and the glass transmittance, we arrive at our answer.",
         "Bestrahlungsstärke mal Fläche, gefiltert durch F_F, F_V und g_tot."),
        ("result",
         "This is Q dot S t r, the final transmission cooling load.",
         "Das ist Q-Punkt-S,tr — die solare Transmissionskühllast."),
        ("meaning",
         "It is the precise thermal wattage our mechanical system must actively remove to prevent the room from overheating.",
         "Genau diese Leistung muss die Anlage aktiv abführen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Solare Kühllast", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        floor_y, ceil_y = -1.1, 1.6
        wall_x = 5.5
        win_lo, win_hi = 0.3, 1.1

        floor = Line(np.array([-6.0, floor_y, 0.0]), np.array([6.0, floor_y, 0.0]), color=P_TEAL, stroke_width=5)
        ceiling = Line(np.array([-wall_x, ceil_y, 0.0]), np.array([wall_x, ceil_y, 0.0]), color=P_WHITE, stroke_width=3)
        left_wall = VGroup(
            Line(np.array([-wall_x, floor_y, 0.0]), np.array([-wall_x, win_lo, 0.0]), color=P_WHITE, stroke_width=3),
            Line(np.array([-wall_x, win_hi, 0.0]), np.array([-wall_x, ceil_y, 0.0]), color=P_WHITE, stroke_width=3),
        )
        right_wall = Line(np.array([wall_x, floor_y, 0.0]), np.array([wall_x, ceil_y, 0.0]), color=P_WHITE, stroke_width=3)
        window = Line(np.array([-wall_x, win_lo, 0.0]), np.array([-wall_x, win_hi, 0.0]), color=P_CYAN, stroke_width=6)
        room = VGroup(floor, ceiling, left_wall, right_wall, window)

        beam = Polygon(
            np.array([-wall_x, win_hi, 0.0]),
            np.array([-wall_x, win_lo, 0.0]),
            np.array([-2.9, floor_y, 0.0]),
            np.array([-0.6, floor_y, 0.0]),
            color=P_YELLOW, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0,
        )

        pool_center = np.array([-1.75, floor_y + 0.08, 0.0])
        heat_wash = Polygon(
            np.array([-3.05, floor_y + 0.02, 0.0]),
            np.array([-0.45, floor_y + 0.02, 0.0]),
            np.array([-0.85, floor_y + 0.55, 0.0]),
            np.array([-2.65, floor_y + 0.55, 0.0]),
            color=P_ORANGE, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.0,
        )

        def _rising_heat_waves(origin, n=4, color=P_ORANGE, height=1.35, x_spread=0.38, stroke_width=2.2):
            ox, oy = float(origin[0]), float(origin[1])
            waves = VGroup()
            for i in range(n):
                x0 = ox + (i - (n - 1) / 2) * x_spread
                phase = i * 0.55
                pts = [
                    np.array([
                        x0 + 0.11 * np.sin(phase + t * 3.2),
                        oy + t * height,
                        0.0,
                    ])
                    for t in np.linspace(0.0, 1.0, 18)
                ]
                wave = VMobject(color=color, stroke_width=stroke_width, stroke_opacity=0.0)
                wave.set_points_smoothly(pts)
                waves.add(wave)
            return waves

        heat_waves = _rising_heat_waves(pool_center, n=5, color=P_ORANGE, height=1.35, x_spread=0.42)
        heat_waves_hot = _rising_heat_waves(
            pool_center + UP * 0.12, n=4, color=P_RED, height=1.1, x_spread=0.34, stroke_width=1.8
        )
        heat_load = VGroup(heat_wash, heat_waves, heat_waves_hot)

        eq_row, eq_items = equation_row([
            ("q", "Q̇_S,tr", P_YELLOW), (None, "=", P_WHITE),
            ("A", "A", P_BLUE), (None, "·", P_WHITE),
            ("ff", "F_F", P_WHITE), (None, "·", P_WHITE),
            ("fv", "F_V", P_TEAL), (None, "·", P_WHITE),
            ("g", "g_tot", P_RED), (None, "·", P_WHITE),
            ("i", "I_S,max", P_YELLOW),
            (None, "  [W]", P_TEAL),
        ], buff=0.12)
        eq_row, eq_box = formula_panel(eq_row, color=P_YELLOW)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(room), run_time=1.8)
        self.play(beam.animate.set_fill(opacity=0.22), run_time=1.2)

        self.add(heat_load)
        self.play(
            heat_wash.animate.set_fill(opacity=0.30),
            LaggedStart(*[Create(w) for w in heat_waves], lag_ratio=0.1),
            LaggedStart(*[Create(w) for w in heat_waves_hot], lag_ratio=0.08),
            run_time=1.1,
        )
        self.play(
            *[w.animate.set_stroke(opacity=0.85) for w in heat_waves],
            *[w.animate.set_stroke(opacity=0.7) for w in heat_waves_hot],
            heat_waves.animate.shift(UP * 0.35),
            heat_waves_hot.animate.shift(UP * 0.22),
            heat_wash.animate.set_fill(opacity=0.40),
            run_time=0.9,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "result"))
        self.play(Create(eq_box), FadeIn(eq_row), run_time=1.2)
        ring = highlight_param(eq_items, "q", color=P_YELLOW)
        self.play(
            Create(ring),
            heat_wash.animate.set_fill(opacity=0.48),
            heat_waves.animate.shift(UP * 0.15).set_stroke(opacity=1.0),
            heat_waves_hot.animate.shift(UP * 0.1).set_stroke(opacity=0.9),
            run_time=1.0,
        )
        hold_for(self, self.NARRATION, "result", used=1.2 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meaning"))
        hold_for(self, self.NARRATION, "meaning", used=0.35)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
