import numpy as np
from manim import *


# ─── Shared Constants ────────────────────────────────────────────────
P_DEEP_DARK = "#0B0C10"
P_WHITE     = "#E0E6ED"
P_CYAN      = "#66FCF1"
P_TEAL      = "#45A29E"
P_ORANGE    = "#FFAAA5"
P_YELLOW    = "#FFE66D"
P_RED       = "#FF6B6B"
P_BLUE      = "#4D96FF"
P_GREEN     = "#CAFFBF"

MAIN_TITLE = "Kühllast mit Sonnenschutz"


def _main_title():
    """☀️ Persistent chapter header shared by every beat of this topic."""
    return Text(MAIN_TITLE, font_size=30, color=P_CYAN).to_edge(UP, buff=0.35)


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


def _equation_row(parts, font_size=24, buff=0.15):
    """🧮 Build a Text-only equation from labelled fragments so symbols stay addressable."""
    row = VGroup()
    items = {}
    for key, txt, color in parts:
        t = Text(txt, font_size=font_size, color=color)
        row.add(t)
        if key:
            items[key] = t
    row.arrange(RIGHT, buff=buff)
    return row, items


def _boxed(mob, color=P_TEAL, buff=0.22, stroke_width=2.5):
    """🔲 Rounded surrounding frame in the topic's equation-box style."""
    box = SurroundingRectangle(mob, color=color, corner_radius=0.12, buff=buff, stroke_width=stroke_width)
    return VGroup(box, mob), box


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


# ═══════════════════════════════════════════════════════════════════════
# BEAT 1 – Maximum Solar Irradiance (I_S,max)
# ═══════════════════════════════════════════════════════════════════════
class Beat1_SolarIrradiance(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Direkte Sonnenstrahlung", font_size=20, color=P_YELLOW)
        subtitle.next_to(title, DOWN, buff=0.18)

        # ── Facade with a large window ──
        fac_c = LEFT * 4.3 + UP * 0.15
        facade = Rectangle(width=3.4, height=2.9, color=P_WHITE, stroke_width=3).move_to(fac_c)
        win = Rectangle(width=2.0, height=1.5, color=P_CYAN, stroke_width=3).move_to(fac_c + UP * 0.1)
        win_cross = VGroup(
            Line(win.get_top(), win.get_bottom(), color=P_CYAN, stroke_width=2),
            Line(win.get_left(), win.get_right(), color=P_CYAN, stroke_width=2),
        )
        ground = Line(fac_c + LEFT * 2.1 + DOWN * 1.45, fac_c + RIGHT * 2.1 + DOWN * 1.45,
                      color=P_TEAL, stroke_width=4)
        building = VGroup(ground, facade, win, win_cross)

        # ── Sun + rays onto the glazing ──
        sun_pos = LEFT * 5.9 + UP * 2.55
        sun = _sun(sun_pos, radius=0.34)
        ray_targets = [
            win.get_center() + UP * 0.5 + LEFT * 0.7,
            win.get_center() + UP * 0.15,
            win.get_center() + DOWN * 0.45 + LEFT * 0.4,
            win.get_center() + UP * 0.4 + RIGHT * 0.6,
            win.get_center() + DOWN * 0.3 + RIGHT * 0.7,
        ]
        rays = VGroup(*[
            Line(sun_pos + (t - sun_pos) * 0.22, t, color=P_YELLOW, stroke_width=2.5, stroke_opacity=0.85)
            for t in ray_targets
        ])

        # ── Irradiance chart ──
        axes = Axes(
            x_range=[6, 18, 3],
            y_range=[0, 900, 300],
            x_length=5.2,
            y_length=3.7,
            axis_config={"color": P_WHITE, "stroke_width": 2},
            tips=False,
        ).move_to(RIGHT * 3.95 + DOWN * 0.25)

        x_labels = VGroup(*[
            Text(str(h), font_size=15, color=P_WHITE).next_to(axes.c2p(h, 0), DOWN, buff=0.16)
            for h in (6, 9, 12, 15, 18)
        ])
        y_labels = VGroup(*[
            Text(str(v), font_size=15, color=P_WHITE).next_to(axes.c2p(6, v), LEFT, buff=0.16)
            for v in (300, 600, 900)
        ])
        y_axis_name = Text("W/m²", font_size=16, color=P_TEAL)
        y_axis_name.next_to(axes.c2p(6, 900), UP, buff=0.2).shift(LEFT * 0.15)
        x_axis_name = Text("Sonnenzeit", font_size=16, color=P_TEAL)
        x_axis_name.next_to(axes.c2p(12, 0), DOWN, buff=0.5)

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
            curve_labels[name] = Text(name, font_size=16, color=color).move_to(axes.c2p(lx, ly) + off)

        # ── Equation box ──
        eq_row, eq_items = _equation_row(
            [("i", "I_S,max", P_YELLOW), (None, "=", P_WHITE), (None, "ca. 400 bis 800 W/m²", P_YELLOW)],
            font_size=21,
        )
        eq_group, eq_box = _boxed(eq_row, color=P_YELLOW)
        eq_group.to_corner(DL, buff=0.4)

        # ── ANIMATION ──
        self.play(FadeIn(title, shift=DOWN * 0.2), run_time=0.8)
        self.play(FadeIn(subtitle, shift=DOWN * 0.15), run_time=0.5)
        self.play(Create(building), run_time=1.6)
        self.play(FadeIn(sun, scale=0.7), run_time=1.0)
        self.play(LaggedStart(*[Create(r) for r in rays], lag_ratio=0.12), run_time=1.4)

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

        self.play(eq_items["i"].animate.scale(1.15), rate_func=there_and_back, run_time=0.9)
        # Hold with VO: irradiance chart / I_S,max
        self.wait(3.1)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 2 – Gross Area & Frame Factor (A · F_F)
# ═══════════════════════════════════════════════════════════════════════
class Beat2_FrameFactor(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Fensterfläche und Rahmenfaktor", font_size=20, color=P_WHITE)
        subtitle.next_to(title, DOWN, buff=0.18)

        w = _build_window(center=LEFT * 3.4 + DOWN * 0.1, width=3.6, height=2.7)
        w["panes"].set_z_index(2)
        w["frame"].set_z_index(3)

        area_label = Text("A: Rohbauöffnung", font_size=16, color=P_BLUE)
        area_label.next_to(w["opening"], DOWN, buff=0.22)

        # ── Rays: some blocked by the opaque frame, some through the glass ──
        direction = np.array([1.0, -0.55, 0.0])
        direction = direction / np.linalg.norm(direction)
        frame_hits = [
            np.array([w["x0"] + 0.15, 0.25, 0.0]),
            np.array([-3.4, 0.35, 0.0]),
            np.array([-4.2, w["y1"] - 0.15, 0.0]),
            np.array([-2.6, w["y0"] + 0.15, 0.0]),
        ]
        glass_hits = [
            np.array([-4.5, 0.45, 0.0]),
            np.array([-4.0, -0.25, 0.0]),
            np.array([-4.6, -0.95, 0.0]),
            np.array([-2.9, 0.3, 0.0]),
            np.array([-2.3, -0.4, 0.0]),
            np.array([-2.6, 0.62, 0.0]),
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

        # ── Formula panel ──
        eq_row, eq_items = _equation_row(
            [("aeff", "A_eff", P_CYAN), (None, "=", P_WHITE), ("a", "A", P_BLUE),
             (None, "·", P_WHITE), ("ff", "F_F", P_WHITE)],
            font_size=28,
        )
        def_a = Text("A: Rohbauöffnung des Fensters", font_size=16, color=P_BLUE)
        def_ff = Text("F_F: Rahmenfaktor, F_F < 1,0", font_size=16, color=P_WHITE)
        def_aeff = Text("A_eff: transparente Restfläche", font_size=16, color=P_CYAN)
        defs = VGroup(def_a, def_ff, def_aeff).arrange(DOWN, aligned_edge=LEFT, buff=0.16)
        panel_body = VGroup(eq_row, defs).arrange(DOWN, buff=0.34)
        panel, panel_box = _boxed(panel_body, color=P_TEAL)
        panel.move_to(RIGHT * 3.6 + UP * 0.1)

        ff_highlight = SurroundingRectangle(
            eq_items["ff"], color=P_ORANGE, buff=0.1, stroke_width=3, corner_radius=0.06
        )

        # ── ANIMATION ──
        self.add(title, subtitle)
        self.play(Create(w["opening"]), FadeIn(area_label, shift=UP * 0.15), run_time=1.4)
        self.play(w["opening"].animate.set_stroke(color=P_BLUE, width=5), rate_func=there_and_back, run_time=0.9)

        self.play(FadeIn(w["frame"], scale=0.9), run_time=1.3)
        self.play(FadeIn(w["panes"]), run_time=0.7)

        self.play(FadeOut(area_label), Create(panel_box), FadeIn(panel_body), run_time=1.4)

        self.play(
            LaggedStart(*[Create(r) for r in (*glass_rays, *frame_rays)], lag_ratio=0.08),
            run_time=1.8,
        )
        self.play(
            LaggedStart(*[Create(r) for r in through_rays], lag_ratio=0.08),
            frame_rays.animate.set_stroke(color="#5A6472", opacity=0.35),
            run_time=1.6,
        )

        self.play(Create(ff_highlight), run_time=0.7)
        self.play(
            ff_highlight.animate.set_stroke(width=5),
            eq_items["ff"].animate.set_color(P_ORANGE),
            rate_func=there_and_back,
            run_time=1.1,
        )
        self.play(
            w["panes"].animate.set_fill(opacity=0.34),
            Indicate(eq_items["aeff"], color=P_CYAN, scale_factor=1.18),
            run_time=1.4,
        )
        self.play(FadeOut(ff_highlight), run_time=0.4)
        # Hold with VO: frame factor / A_eff
        self.wait(3.0)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 3 – Shading Factor (F_V)
# ═══════════════════════════════════════════════════════════════════════
class Beat3_ShadingFactor(Scene):
    """🪟 Vertical section through the facade: without shading the full beam reaches the
    glass; the external Raffstore intercepts it and reflects the heat back outside."""

    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Verschattungsfaktor", font_size=20, color=P_TEAL)
        subtitle.next_to(title, DOWN, buff=0.18)

        # ── Facade in vertical section ──
        glass_out, glass_in = -3.5, -3.3
        wall_top, wall_bottom = 1.35, -1.65
        lintel = Rectangle(
            width=0.5, height=0.85, color=P_WHITE, stroke_width=2.5,
            fill_color="#161A21", fill_opacity=1.0,
        ).move_to(np.array([-3.4, wall_top + 0.425, 0.0]))
        sill = Rectangle(
            width=0.5, height=0.85, color=P_WHITE, stroke_width=2.5,
            fill_color="#161A21", fill_opacity=1.0,
        ).move_to(np.array([-3.4, wall_bottom - 0.425, 0.0]))
        masonry = VGroup(
            lintel, sill,
            _section_hatch(lintel), _section_hatch(sill),
        ).set_z_index(3)

        glazing = VGroup(*[
            Line(np.array([x, wall_bottom, 0.0]), np.array([x, wall_top, 0.0]),
                 color=P_CYAN, stroke_width=3)
            for x in (glass_out, glass_in)
        ]).set_z_index(3)

        ceiling = Line(np.array([-3.15, 2.2, 0.0]), np.array([-0.7, 2.2, 0.0]),
                       color=P_WHITE, stroke_width=2.5)
        floor = Line(np.array([-3.15, -2.5, 0.0]), np.array([-0.7, -2.5, 0.0]),
                     color=P_TEAL, stroke_width=4)
        section = VGroup(masonry, glazing, ceiling, floor)

        lbl_out = Text("Außen", font_size=16, color=P_TEAL).move_to(np.array([-6.4, -1.5, 0.0]))
        lbl_in = Text("Innen", font_size=16, color=P_TEAL).move_to(np.array([-1.5, 1.5, 0.0]))
        lbl_glass = Text("Verglasung", font_size=15, color=P_CYAN).move_to(np.array([-2.6, -3.0, 0.0]))

        # ── Solar geometry: parallel beam at a high summer altitude ──
        d = np.array([0.75, -0.661, 0.0])
        sun = _sun(np.array([-6.3, 2.85, 0.0]), radius=0.26)

        slat_x = -4.4
        slat_ys = [1.2 - i * 0.4 for i in range(8)]
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

        interior = VGroup(*[
            Line(np.array([glass_in, y, 0.0]), np.array([glass_in, y, 0.0]) + d * 2.55,
                 color=P_YELLOW, stroke_width=2.6, stroke_opacity=0.85).set_z_index(1)
            for y in glass_ys[:3]
        ])

        # ── External Raffstore: slats normal to the beam, reflecting it back outside ──
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
        ).move_to(np.array([slat_x, 1.62, 0.0]))
        bracket = Line(np.array([slat_x + 0.31, 1.62, 0.0]), np.array([-3.65, 1.62, 0.0]),
                       color=P_TEAL, stroke_width=2.5)
        blind = VGroup(rail, bracket, slats).set_z_index(4)
        lbl_blind = Text("Raffstore", font_size=15, color=P_TEAL).move_to(np.array([-4.95, -2.35, 0.0]))

        reflected = VGroup(*[
            Arrow(
                np.array([-4.487, y + 0.077, 0.0]),
                np.array([-4.487, y + 0.077, 0.0]) + r * 1.45,
                buff=0, color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.55,
                max_tip_length_to_length_ratio=0.16,
            ).set_z_index(5)
            for y in aimed_ys
        ])
        # Residual: leaves the slat's inner tip, crosses the glass, trickles into the room
        residual = VGroup(*[
            Line(
                np.array([-4.226, y + 0.193, 0.0]),
                np.array([-4.226, y + 0.193, 0.0]) + d * 3.6,
                color=P_YELLOW, stroke_width=1.4, stroke_opacity=0.32,
            ).set_z_index(1)
            for y in aimed_ys[:3]
        ])
        lbl_rest = Text("Restanteil", font_size=15, color=P_YELLOW).move_to(np.array([-1.25, -0.55, 0.0]))

        # ── Equation ──
        eq_row, eq_items = _equation_row(
            [("ired", "I_reduziert", P_TEAL), (None, "=", P_WHITE), ("i", "I_S,max", P_YELLOW),
             (None, "·", P_WHITE), ("fv", "F_V", P_TEAL)],
            font_size=25,
        )
        eq_group, eq_box = _boxed(eq_row, color=P_TEAL)
        eq_group.move_to(RIGHT * 3.7 + UP * 2.0)

        # ── F_V scale ──
        sx0, sx1, sy = 1.5, 6.3, -1.55
        scale_line = Line(np.array([sx0, sy, 0.0]), np.array([sx1, sy, 0.0]), color=P_WHITE, stroke_width=2)
        tick_l = Line(np.array([sx0, sy - 0.12, 0.0]), np.array([sx0, sy + 0.12, 0.0]), color=P_WHITE, stroke_width=2)
        tick_r = Line(np.array([sx1, sy - 0.12, 0.0]), np.array([sx1, sy + 0.12, 0.0]), color=P_WHITE, stroke_width=2)
        end_l = Text("0,1\naußenliegend", font_size=13, color=P_TEAL, line_spacing=0.8)
        end_l.next_to(tick_l, DOWN, buff=0.14)
        end_r = Text("1,0\nohne Schutz", font_size=13, color=P_YELLOW, line_spacing=0.8)
        end_r.next_to(tick_r, DOWN, buff=0.14)
        scale_title = Text("Bandbreite F_V", font_size=16, color=P_WHITE)
        scale_title.next_to(scale_line, UP, buff=1.05)

        def _sx(fv):
            return sx0 + (fv - 0.1) / 0.9 * (sx1 - sx0)

        marker = Triangle(color=P_YELLOW, fill_color=P_YELLOW, fill_opacity=1.0, stroke_width=0)
        marker.scale(0.16).rotate(PI).move_to(np.array([_sx(1.0), sy + 0.24, 0.0]))
        marker_val = Text("F_V = 1,0", font_size=17, color=P_YELLOW)
        marker_val.next_to(marker, UP, buff=0.12)
        marker_val_new = Text("F_V = 0,15", font_size=17, color=P_TEAL)
        marker_val_new.next_to(np.array([_sx(0.15), sy + 0.4, 0.0]), UP, buff=0.12)

        cap_before = Text("Ohne Schutz trifft die volle Strahlung auf die Scheibe",
                          font_size=17, color=P_YELLOW)
        cap_before.to_edge(DOWN, buff=0.28)
        cap_after = Text("Außenliegend reflektiert: Die Wärme bleibt vor der Fassade",
                         font_size=17, color=P_TEAL)
        cap_after.to_edge(DOWN, buff=0.28)

        # ── ANIMATION ──
        self.add(title, subtitle)
        self.play(Create(section), run_time=1.8)
        self.play(
            LaggedStart(FadeIn(lbl_out), FadeIn(lbl_in), FadeIn(lbl_glass), lag_ratio=0.2),
            run_time=1.0,
        )
        self.play(Create(eq_box), FadeIn(eq_row), run_time=1.2)

        # Situation A — unshaded: the whole beam reaches the glass and enters the room
        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(LaggedStart(*[Create(ray) for ray in direct], lag_ratio=0.08), run_time=1.6)
        self.play(
            LaggedStart(*[Create(ray) for ray in interior], lag_ratio=0.1),
            FadeIn(cap_before),
            run_time=1.4,
        )
        self.play(
            Create(scale_line), Create(tick_l), Create(tick_r),
            FadeIn(end_l), FadeIn(end_r), FadeIn(scale_title),
            FadeIn(marker, shift=DOWN * 0.2), FadeIn(marker_val),
            run_time=1.4,
        )

        # Situation B — the Raffstore drops in front of the facade
        blind.shift(UP * 3.0).set_opacity(0)
        self.add(blind)
        self.play(blind.animate.shift(DOWN * 3.0).set_opacity(1.0), run_time=1.5)
        self.play(FadeIn(lbl_blind, shift=UP * 0.15), run_time=0.5)

        self.play(
            *[Transform(ray, cut) for ray, cut in zip(direct, blocked)],
            LaggedStart(*[GrowArrow(a) for a in reflected], lag_ratio=0.08),
            run_time=1.8,
        )
        self.play(
            FadeOut(interior),
            LaggedStart(*[Create(ray) for ray in residual], lag_ratio=0.1),
            FadeIn(lbl_rest),
            ReplacementTransform(cap_before, cap_after),
            run_time=1.6,
        )
        self.play(
            marker.animate.move_to(np.array([_sx(0.15), sy + 0.24, 0.0])).set_color(P_TEAL),
            ReplacementTransform(marker_val, marker_val_new),
            run_time=1.8,
        )

        fv_highlight = SurroundingRectangle(
            eq_items["fv"], color=P_TEAL, buff=0.1, stroke_width=3, corner_radius=0.06
        )
        self.play(Create(fv_highlight), run_time=0.7)
        self.play(
            fv_highlight.animate.set_stroke(width=5),
            Indicate(eq_items["ired"], color=P_TEAL, scale_factor=1.12),
            run_time=1.2,
        )
        self.play(FadeOut(fv_highlight), run_time=0.4)
        # Hold with VO: shading factor / Raffstore
        self.wait(3.0)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 4 – Total Solar Energy Transmittance (g_tot)
# ═══════════════════════════════════════════════════════════════════════
class Beat4_GlassTransmittance(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Gesamtenergiedurchlassgrad", font_size=20, color=P_RED)
        subtitle.next_to(title, DOWN, buff=0.18)

        # ── Glass pane cross-section ──
        pane_x, pane_top, pane_bottom = -1.5, 1.75, -2.2
        pane_w = 0.36
        pane = Rectangle(
            width=pane_w, height=pane_top - pane_bottom, color=P_CYAN, stroke_width=3,
            fill_color=P_CYAN, fill_opacity=0.18,
        ).move_to(np.array([pane_x, (pane_top + pane_bottom) / 2, 0.0]))
        pane_label = Text("Verglasung im Schnitt", font_size=16, color=P_CYAN)
        pane_label.move_to(np.array([pane_x, 2.32, 0.0]))

        outside = Text("Außen", font_size=17, color=P_TEAL).move_to(LEFT * 5.6 + UP * 1.95)
        inside = Text("Innen", font_size=17, color=P_TEAL).move_to(RIGHT * 4.8 + UP * 1.95)

        # ── Three light paths (reflection mirrors x on the vertical pane) ──
        hit_out = np.array([pane_x - pane_w / 2, 1.1, 0.0])
        hit_in = np.array([pane_x + pane_w / 2, 1.1, 0.0])
        d_in = np.array([3.52, -1.5, 0.0])
        d_in = d_in / np.linalg.norm(d_in)
        d_ref = np.array([-d_in[0], d_in[1], 0.0])

        incoming = Line(hit_out - d_in * 3.83, hit_out, color=P_YELLOW, stroke_width=3)
        reflected = Line(hit_out, hit_out + d_ref * 2.8, color=P_WHITE, stroke_width=2.5, stroke_opacity=0.7)
        transmitted = Line(hit_in, hit_in + d_in * 4.6, color=P_YELLOW, stroke_width=3)

        lbl_refl = Text("Reflexion", font_size=16, color=P_WHITE).move_to(LEFT * 4.55 + DOWN * 0.5)
        lbl_tau = Text("τ_e", font_size=22, color=P_YELLOW).move_to(np.array([0.99, 0.68, 0.0]))
        lbl_qi = Text("q_i", font_size=22, color=P_RED).move_to(np.array([3.05, -1.45, 0.0]))

        waves = VGroup(*[
            _heat_wave(np.array([pane_x + pane_w / 2 + 0.05, y, 0.0]), length=3.6, color=P_RED)
            for y in (-0.9, -1.45, -2.0)
        ])

        merge_point = np.array([3.6, -0.55, 0.0])
        merge_glow = VGroup(
            Dot(merge_point, radius=0.5, color=P_ORANGE, fill_opacity=0.12),
            Dot(merge_point, radius=0.3, color=P_ORANGE, fill_opacity=0.25),
            Dot(merge_point, radius=0.15, color=P_ORANGE, fill_opacity=0.5),
        )

        # ── Equation ──
        eq_row, eq_items = _equation_row(
            [("g", "g_tot", P_RED), (None, "=", P_WHITE), ("tau", "τ_e", P_YELLOW),
             (None, "+", P_WHITE), ("qi", "q_i", P_RED)],
            font_size=27,
        )
        eq_group, eq_box = _boxed(eq_row, color=P_RED)
        eq_group.move_to(DOWN * 3.2)

        # ── ANIMATION ──
        self.add(title, subtitle)
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

        tau_copy = lbl_tau.copy()
        qi_copy = lbl_qi.copy()
        self.add(tau_copy, qi_copy)
        self.play(
            tau_copy.animate.move_to(merge_point),
            qi_copy.animate.move_to(merge_point),
            FadeIn(merge_glow),
            run_time=1.6,
        )
        self.play(
            FadeOut(tau_copy), FadeOut(qi_copy),
            merge_glow.animate.move_to(eq_items["g"].get_center()).scale(0.8),
            Indicate(eq_items["g"], color=P_ORANGE, scale_factor=1.2),
            run_time=1.6,
        )
        self.play(FadeOut(merge_glow), run_time=0.6)
        # Hold with VO: g_tot transmittance
        self.wait(3.2)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 5 – Final Solar Cooling Load (Q̇_S,tr)
# ═══════════════════════════════════════════════════════════════════════
class Beat5_SolarCoolingLoad(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Solare Kühllast", font_size=20, color=P_YELLOW)
        subtitle.next_to(title, DOWN, buff=0.18)

        # ── Room interior (wide shot) ──
        floor_y, ceil_y = -3.0, 0.5
        wall_x = 5.5
        win_lo, win_hi = -0.8, 0.1

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

        # Soft floor heat wash where the beam lands (no blob circles)
        pool_center = np.array([-1.75, floor_y + 0.08, 0.0])
        heat_wash = Polygon(
            np.array([-3.05, floor_y + 0.02, 0.0]),
            np.array([-0.45, floor_y + 0.02, 0.0]),
            np.array([-0.85, floor_y + 0.55, 0.0]),
            np.array([-2.65, floor_y + 0.55, 0.0]),
            color=P_ORANGE, stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.0,
        )

        def _rising_heat_waves(origin, n=4, color=P_ORANGE, height=1.55, x_spread=0.38, stroke_width=2.2):
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

        heat_waves = _rising_heat_waves(pool_center, n=5, color=P_ORANGE, height=1.7, x_spread=0.42)
        heat_waves_hot = _rising_heat_waves(
            pool_center + UP * 0.15, n=4, color=P_RED, height=1.35, x_spread=0.34, stroke_width=1.8
        )
        heat_load = VGroup(heat_wash, heat_waves, heat_waves_hot)

        # ── Master equation ──
        eq_row, eq_items = _equation_row(
            [("q", "Q̇_S,tr", P_YELLOW), (None, "=", P_WHITE),
             ("A", "A", P_BLUE), (None, "·", P_WHITE),
             ("ff", "F_F", P_WHITE), (None, "·", P_WHITE),
             ("fv", "F_V", P_TEAL), (None, "·", P_WHITE),
             ("g", "g_tot", P_RED), (None, "·", P_WHITE),
             ("i", "I_S,max", P_YELLOW)],
            font_size=25,
            buff=0.14,
        )
        eq_row.move_to(UP * 2.15)
        eq_box = SurroundingRectangle(eq_row, color=P_YELLOW, corner_radius=0.14, buff=0.26, stroke_width=3)

        head = VGroup(eq_items["q"], eq_row[1])
        fly_keys = ["A", "ff", "fv", "g", "i"]
        fly_targets = {k: eq_items[k].get_center() for k in fly_keys}
        start_xs = [-5.0, -2.5, 0.0, 2.5, 5.0]
        separators = [m for m in eq_row if m not in list(eq_items.values())][1:]

        for k, sx in zip(fly_keys, start_xs):
            eq_items[k].move_to(np.array([sx, 1.25, 0.0]))
        for s in separators:
            s.set_opacity(0.0)

        caption = Text("Der Anteil, den die Anlage aktiv abführen muss",
                       font_size=17, color=P_WHITE)
        caption.to_edge(DOWN, buff=0.28)

        # ── ANIMATION ──
        self.add(title, subtitle)
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
            heat_waves.animate.shift(UP * 0.4),
            heat_waves_hot.animate.shift(UP * 0.28),
            heat_wash.animate.set_fill(opacity=0.40),
            run_time=0.9,
        )

        self.play(FadeIn(head, shift=DOWN * 0.2), run_time=0.9)
        self.play(
            LaggedStart(*[FadeIn(eq_items[k], scale=0.8) for k in fly_keys], lag_ratio=0.12),
            run_time=1.4,
        )
        self.play(
            LaggedStart(*[eq_items[k].animate.move_to(fly_targets[k]) for k in fly_keys], lag_ratio=0.1),
            run_time=1.8,
        )
        self.play(
            *[s.animate.set_opacity(1.0) for s in separators],
            Create(eq_box),
            run_time=1.0,
        )

        self.play(
            eq_box.animate.set_stroke(width=6),
            heat_wash.animate.set_fill(opacity=0.48),
            heat_waves.animate.shift(UP * 0.2).set_stroke(opacity=1.0),
            heat_waves_hot.animate.shift(UP * 0.15).set_stroke(opacity=0.9),
            FadeIn(caption),
            run_time=1.2,
        )
        self.play(
            eq_box.animate.set_stroke(width=3),
            heat_wash.animate.set_fill(opacity=0.32),
            heat_waves.animate.set_stroke(opacity=0.7),
            heat_waves_hot.animate.set_stroke(opacity=0.55),
            run_time=0.8,
        )
        # Hold with VO: Q̇_S,tr cooling load
        self.wait(1.8)
