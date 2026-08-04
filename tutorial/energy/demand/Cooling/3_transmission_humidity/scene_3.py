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


def _build_house(center=ORIGIN):
    """Line-art house exterior (shared across beats)."""
    w_width, w_height = 3.6, 2.4
    bl = center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
    br = center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
    tl = center + LEFT * (w_width / 2) + UP * (w_height / 2)
    tr = center + RIGHT * (w_width / 2) + UP * (w_height / 2)
    roof_peak = center + UP * (w_height / 2 + 1.1)

    floor_line = Line(bl + LEFT * 0.6, br + RIGHT * 0.6, color=P_TEAL, stroke_width=4)
    walls = VGroup(
        Line(bl, tl, color=P_WHITE, stroke_width=3),
        Line(br, tr, color=P_WHITE, stroke_width=3),
    )
    roof = Polygon(tl, roof_peak, tr, color=P_WHITE, stroke_width=3)

    # Window
    window = Square(side_length=0.7, color=P_CYAN, stroke_width=2)
    window.move_to(center + LEFT * 0.8 + UP * 0.3)
    window_cross = VGroup(
        Line(window.get_top(), window.get_bottom(), color=P_CYAN, stroke_width=1.5),
        Line(window.get_left(), window.get_right(), color=P_CYAN, stroke_width=1.5),
    )
    window_group = VGroup(window, window_cross)

    # Door
    door = Rectangle(width=0.6, height=1.0, color=P_CYAN, stroke_width=2)
    door.move_to(center + RIGHT * 0.8 + DOWN * 0.7)
    door_knob = Dot(door.get_center() + LEFT * 0.18 + DOWN * 0.05, radius=0.04, color=P_CYAN)
    door_group = VGroup(door, door_knob)

    house = VGroup(floor_line, walls, roof, window_group, door_group)

    return {
        "house": house,
        "floor": floor_line,
        "walls": walls,
        "roof": roof,
        "window": window,
        "window_group": window_group,
        "door_group": door_group,
        "bl": bl, "br": br, "tl": tl, "tr": tr,
        "roof_peak": roof_peak,
        "center": center,
        "w_width": w_width,
        "w_height": w_height,
    }

def _build_house_section(center=ORIGIN):
    """Cross-section view of the house for ventilation scenes."""
    w_width, w_height = 3.6, 2.4
    bl = center + LEFT * (w_width / 2) + DOWN * (w_height / 2)
    br = center + RIGHT * (w_width / 2) + DOWN * (w_height / 2)
    tl = center + LEFT * (w_width / 2) + UP * (w_height / 2)
    tr = center + RIGHT * (w_width / 2) + UP * (w_height / 2)
    roof_peak = center + UP * (w_height / 2 + 1.1)

    wall_thickness = 0.25

    floor_line = Line(bl + LEFT * 0.6, br + RIGHT * 0.6, color=P_TEAL, stroke_width=4)

    # Cut window on left wall
    win_top = tl + DOWN * 0.6
    win_bottom = tl + DOWN * 1.5

    boundary = VGroup(
        Line(bl, win_bottom, color=P_WHITE, stroke_width=6),
        Line(win_top, tl, color=P_WHITE, stroke_width=6),
        Line(tl, roof_peak, color=P_WHITE, stroke_width=6),
        Line(roof_peak, tr, color=P_WHITE, stroke_width=6),
        Line(tr, br, color=P_WHITE, stroke_width=6),
    )

    window_center = (win_top + win_bottom) / 2 + LEFT * (wall_thickness/2)

    house = VGroup(floor_line, boundary)

    return {
        "house": house,
        "floor": floor_line,
        "walls": boundary,
        "window_center": window_center,
        "bl": bl, "br": br, "tl": tl, "tr": tr,
        "roof_peak": roof_peak,
        "center": center,
        "w_width": w_width,
        "w_height": w_height,
        "win_top": win_top,
        "win_bottom": win_bottom,
        "wall_thickness": wall_thickness
    }


# ═══════════════════════════════════════════════════════════════════════
# BEAT 1 – Transmission Through Opaque Surfaces
# ═══════════════════════════════════════════════════════════════════════
# ═══════════════════════════════════════════════════════════════════════
# BEAT 1 – Transmission Through Opaque Surfaces
# ═══════════════════════════════════════════════════════════════════════
class Beat1_TransmissionOpaque(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        # ── Title ──
        title = Text("Transmissionswärme: Opake Bauteile", font_size=30, color=P_WHITE)
        title.to_edge(UP, buff=0.4)

        # ── House ──
        hc = LEFT * 0.5 + DOWN * 0.7
        h = _build_house(hc)

        # ── Sun ──
        sun_pos = RIGHT * 4.0 + UP * 1.8
        sun_core = Dot(sun_pos, radius=0.45, color=P_YELLOW)
        sun_glow = Dot(sun_pos, radius=0.7, color=P_YELLOW, fill_opacity=0.35)
        sun_ring1 = Circle(radius=0.85, color=P_YELLOW, stroke_width=2, stroke_opacity=0.6).move_to(sun_pos)
        sun_ring2 = Circle(radius=1.1, color=P_YELLOW, stroke_width=1.2, stroke_opacity=0.3).move_to(sun_pos)
        sun_burst = VGroup()
        for angle in np.linspace(0, TAU, 12, endpoint=False):
            s = sun_pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
            e = sun_pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
            sun_burst.add(Line(s, e, color=P_YELLOW, stroke_width=2))
        sun_group = VGroup(sun_glow, sun_core, sun_ring1, sun_ring2, sun_burst)

        # ── Opaque Highlights (walls + roof borders) ──
        opaque_left = Line(h["bl"], h["tl"], color=P_ORANGE, stroke_width=6)
        opaque_right = Line(h["br"], h["tr"], color=P_ORANGE, stroke_width=6)
        opaque_roof_l = Line(h["tl"], h["roof_peak"], color=P_ORANGE, stroke_width=6)
        opaque_roof_r = Line(h["tr"], h["roof_peak"], color=P_ORANGE, stroke_width=6)
        opaque_borders = VGroup(opaque_left, opaque_right, opaque_roof_l, opaque_roof_r)

        # ── Sun Rays hitting surfaces ──
        targets = [
            h["roof_peak"] + DOWN * 0.3,
            (h["tl"] + h["roof_peak"]) / 2,
            (h["tr"] + h["roof_peak"]) / 2,
            (h["tl"] + h["bl"]) / 2 + RIGHT * 0.1,
            (h["tr"] + h["br"]) / 2 + LEFT * 0.1,
        ]
        rays = VGroup(*[
            Line(sun_pos + (t - sun_pos) * 0.12, t, color=P_YELLOW, stroke_width=2.5, stroke_opacity=0.8)
            for t in targets
        ])

        # ── Equation Box ──
        eq_text = Text("Q̇_T = U · A · ΔT_eq", font_size=22, color=P_WHITE)
        eq_box = SurroundingRectangle(eq_text, color=P_TEAL, corner_radius=0.1, buff=0.2)
        eq_group = VGroup(eq_box, eq_text)
        eq_group.to_corner(DL, buff=0.5)

        # ── Variable highlight helpers (exact Mobject target elements) ──
        # eq_text character indices in Text("Q̇_T = U · A · ΔT_eq"):
        # [0:4] -> Q̇_T (Q, combining dot, _, T)
        # [5]   -> U
        # [7]   -> A
        # [9:]  -> ΔT_eq (Δ, T, _, e, q)
        u_mobj = eq_text[5]
        a_mobj = eq_text[7]
        dt_mobj = eq_text[9:]

        u_highlight = SurroundingRectangle(
            u_mobj, color=P_ORANGE, buff=0.08, stroke_width=3, corner_radius=0.05
        )
        a_highlight = SurroundingRectangle(
            a_mobj, color=P_ORANGE, buff=0.08, stroke_width=3, corner_radius=0.05
        )
        dt_highlight = SurroundingRectangle(
            dt_mobj, color=P_ORANGE, buff=0.08, stroke_width=3, corner_radius=0.05
        )

        # ── ANIMATION ──
        # Beat 1a: Title + house draw
        self.play(FadeIn(title, shift=DOWN * 0.2), run_time=0.8)
        self.play(Create(h["house"]), run_time=1.5)

        # Beat 1b: Sun appears
        self.play(
            FadeIn(sun_group, scale=0.7),
            run_time=1.2,
        )

        # Beat 1c: Opaque borders light up
        self.play(
            Create(opaque_borders),
            run_time=1.0,
        )

        # Beat 1d: Sun rays + walls absorb heat (White → Red)
        self.play(
            LaggedStart(*[Create(r) for r in rays], lag_ratio=0.12),
            run_time=1.5,
        )
        self.play(
            opaque_borders.animate.set_color(P_RED),
            h["walls"][0].animate.set_color(P_RED),
            h["walls"][1].animate.set_color(P_RED),
            h["roof"].animate.set_color(P_RED),
            run_time=2.5,
        )

        # Beat 1e: Equation appears
        self.play(
            Create(eq_box), FadeIn(eq_text),
            run_time=1.2,
        )

        # Beat 1f: Highlight U and A
        self.play(Create(u_highlight), Create(a_highlight), run_time=1.0)
        self.play(FadeOut(u_highlight), FadeOut(a_highlight), run_time=0.6)

        # Beat 1g: Highlight ΔT_eq
        self.play(Create(dt_highlight), run_time=0.8)
        self.play(
            dt_highlight.animate.set_color(P_YELLOW),
            run_time=0.8,
            rate_func=there_and_back,
        )
        self.play(FadeOut(dt_highlight), run_time=0.5)

        # Hold with VO: ΔT_eq / opaque transmission
        self.wait(4.0)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 2 – Thermal Mass & Time Lag (Phasenverschiebung)
# ═══════════════════════════════════════════════════════════════════════
class Beat2_TimeLag(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        # ── Title ──
        title = Text("Transmissionswärme: Opake Bauteile", font_size=30, color=P_WHITE)
        title.to_edge(UP, buff=0.4)

        # ── House (walls/roof already red from Beat 1) ──
        hc = LEFT * 0.5 + DOWN * 0.7
        h = _build_house(hc)
        # Set walls/roof to red (continuing from beat 1)
        h["walls"][0].set_color(P_RED)
        h["walls"][1].set_color(P_RED)
        h["roof"].set_color(P_RED)

        # ── Equation (persists from Beat 1) ──
        eq_text = Text("Q̇_T = U · A · ΔT_eq", font_size=22, color=P_WHITE)
        eq_box = SurroundingRectangle(eq_text, color=P_TEAL, corner_radius=0.1, buff=0.2)
        eq_group = VGroup(eq_box, eq_text)
        eq_group.to_corner(DL, buff=0.5)

        # ── Sun (will be faded out) ──
        sun_pos = RIGHT * 4.0 + UP * 1.8
        sun_core = Dot(sun_pos, radius=0.45, color=P_YELLOW)
        sun_glow = Dot(sun_pos, radius=0.7, color=P_YELLOW, fill_opacity=0.35)
        sun_ring1 = Circle(radius=0.85, color=P_YELLOW, stroke_width=2, stroke_opacity=0.6).move_to(sun_pos)
        sun_ring2 = Circle(radius=1.1, color=P_YELLOW, stroke_width=1.2, stroke_opacity=0.3).move_to(sun_pos)
        sun_burst = VGroup()
        for angle in np.linspace(0, TAU, 12, endpoint=False):
            s = sun_pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
            e = sun_pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
            sun_burst.add(Line(s, e, color=P_YELLOW, stroke_width=2))
        sun_group = VGroup(sun_glow, sun_core, sun_ring1, sun_ring2, sun_burst)

        # ── Label (Subtitle placement under main title) ──
        label = Text("Phasenverschiebung (Time Lag)", font_size=20, color=P_ORANGE)
        label.next_to(title, DOWN, buff=0.2)

        # ── Clock ──
        clock_center = LEFT * 3.0 + UP * 2.5
        clock_face = Circle(radius=0.45, color=P_WHITE, stroke_width=2).move_to(clock_center)
        # Hour hand
        hour_hand = Line(
            clock_center, clock_center + UP * 0.25,
            color=P_WHITE, stroke_width=3,
        )
        # Minute hand
        minute_hand = Line(
            clock_center, clock_center + UP * 0.35,
            color=P_WHITE, stroke_width=2,
        )
        # Tick marks
        ticks = VGroup()
        for i in range(12):
            angle = i * TAU / 12
            inner = clock_center + np.array([np.cos(angle) * 0.35, np.sin(angle) * 0.35, 0])
            outer = clock_center + np.array([np.cos(angle) * 0.42, np.sin(angle) * 0.42, 0])
            ticks.add(Line(inner, outer, color=P_WHITE, stroke_width=1.5))
        clock_group = VGroup(clock_face, ticks, hour_hand, minute_hand)

        # ── Interior Trapped Heat Glow (Polygon matching interior shape) ──
        house_interior_points = [
            h["bl"] + RIGHT * 0.05 + UP * 0.05,
            h["br"] + LEFT * 0.05 + UP * 0.05,
            h["tr"] + LEFT * 0.05 + DOWN * 0.05,
            h["roof_peak"] + DOWN * 0.1,
            h["tl"] + RIGHT * 0.05 + DOWN * 0.05,
        ]
        interior_heat_glow = Polygon(
            *house_interior_points,
            fill_color=P_RED, fill_opacity=0.0, stroke_width=0
        )

        # ── Radiant heat wave rings expanding from walls inward ──
        heat_wave_1 = Polygon(
            *house_interior_points,
            color=P_ORANGE, stroke_width=2, stroke_opacity=0.0
        )
        heat_wave_2 = Polygon(
            *house_interior_points,
            color=P_RED, stroke_width=1.5, stroke_opacity=0.0
        )

        # ── ΔT_eq highlight (pulses) ──
        dt_highlight = SurroundingRectangle(
            eq_text[9:], color=P_ORANGE, buff=0.08, stroke_width=3, corner_radius=0.05,
        )

        # ── ANIMATION ──
        # Show persisted state
        self.add(title, h["house"], eq_group, sun_group)

        # Beat 2a: Label appears + clock draws
        self.play(
            FadeIn(label, shift=LEFT * 0.3),
            Create(clock_group),
            run_time=1.2,
        )

        # Beat 2b: ΔT_eq highlight pulses
        self.play(Create(dt_highlight), run_time=0.6)

        # Beat 2c: Clock hands spin (time passing) + sun sets
        hour_tracker = ValueTracker(0)
        minute_tracker = ValueTracker(0)

        def update_hour(m):
            a = hour_tracker.get_value()
            end = clock_center + np.array([np.sin(a) * 0.25, np.cos(a) * 0.25, 0])
            m.put_start_and_end_on(clock_center, end)

        def update_minute(m):
            a = minute_tracker.get_value()
            end = clock_center + np.array([np.sin(a) * 0.35, np.cos(a) * 0.35, 0])
            m.put_start_and_end_on(clock_center, end)

        hour_hand.add_updater(update_hour)
        minute_hand.add_updater(update_minute)

        self.play(
            hour_tracker.animate.set_value(TAU * 2),
            minute_tracker.animate.set_value(TAU * 12),
            sun_group.animate.shift(DOWN * 4 + RIGHT * 1.5).set_opacity(0),
            run_time=3.0,
            rate_func=linear,
        )
        hour_hand.remove_updater(update_hour)
        minute_hand.remove_updater(update_minute)

        # Beat 2d: Sun is gone, stored thermal heat releases into the room (soft glow fill + pulsing waves)
        self.add(interior_heat_glow, heat_wave_1, heat_wave_2)
        self.play(
            dt_highlight.animate.set_stroke(width=4),
            interior_heat_glow.animate.set_fill(opacity=0.35),
            heat_wave_1.animate.set_stroke(opacity=0.7).scale(0.92),
            run_time=2.0,
        )

        # Beat 2e: Continuous thermal lag peak demand pulse (Glow intensifies to deep red)
        self.play(
            interior_heat_glow.animate.set_fill(opacity=0.55, color=P_RED),
            heat_wave_1.animate.scale(0.9).set_stroke(color=P_RED, opacity=0.9),
            heat_wave_2.animate.set_stroke(opacity=0.8).scale(0.85),
            dt_highlight.animate.set_stroke(width=5, color=P_RED),
            run_time=2.0,
        )
        self.play(
            dt_highlight.animate.set_stroke(width=3, color=P_ORANGE),
            interior_heat_glow.animate.set_fill(opacity=0.4),
            run_time=0.5,
            rate_func=there_and_back,
        )

        # Final hold
        self.play(FadeOut(dt_highlight), run_time=0.4)
        # Hold with VO: thermal mass / evening peak load
        self.wait(1.6)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 3 – Ventilation Heat (The Muggy Air)
# ═══════════════════════════════════════════════════════════════════════
# ═══════════════════════════════════════════════════════════════════════
# BEAT 3 – Ventilation Heat (The Muggy Air)
# ═══════════════════════════════════════════════════════════════════════
class Beat3_VentilationHeat(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        # ── Title ──
        title = Text("Lüftungswärme & Feuchtigkeit", font_size=30, color=P_CYAN)
        title.to_edge(UP, buff=0.4)

        # ── House (Section View) ──
        hc = LEFT * 0.5 + DOWN * 0.7
        h = _build_house_section(hc)

        # ── Sun returns ──
        sun_pos = RIGHT * 4.0 + UP * 1.8
        sun_core = Dot(sun_pos, radius=0.45, color=P_YELLOW)
        sun_glow = Dot(sun_pos, radius=0.7, color=P_YELLOW, fill_opacity=0.35)
        sun_ring1 = Circle(radius=0.85, color=P_YELLOW, stroke_width=2, stroke_opacity=0.6).move_to(sun_pos)
        sun_ring2 = Circle(radius=1.1, color=P_YELLOW, stroke_width=1.2, stroke_opacity=0.3).move_to(sun_pos)
        sun_burst = VGroup()
        for angle in np.linspace(0, TAU, 12, endpoint=False):
            s = sun_pos + np.array([np.cos(angle) * 0.55, np.sin(angle) * 0.55, 0])
            e = sun_pos + np.array([np.cos(angle) * 0.9, np.sin(angle) * 0.9, 0])
            sun_burst.add(Line(s, e, color=P_YELLOW, stroke_width=2))
        sun_group = VGroup(sun_glow, sun_core, sun_ring1, sun_ring2, sun_burst)

        # ── Master Equation Box ──
        eq_text = Text("Q̇_L = Q̇_sens + Q̇_lat", font_size=22, color=P_WHITE)
        eq_box = SurroundingRectangle(eq_text, color=P_TEAL, corner_radius=0.1, buff=0.2)
        eq_group = VGroup(eq_box, eq_text)
        eq_group.to_corner(DL, buff=0.5)

        # ── Window section zoom highlight box ──
        win_center = h["window_center"]
        # Zoom box
        sq_box = Square(side_length=1.6, color=P_CYAN, stroke_width=2).move_to(win_center)
        zoom_box = DashedVMobject(sq_box, num_dashes=16)

        # ── Air Current Streamlines ──
        # Hot air entering:
        air_start_x = h["win_top"][0] - 2.5
        air_end_x = hc[0] + 0.5
        air_y_base_in = h["win_top"][1] - 0.2

        heat_waves_in = VGroup()
        for i in range(3):
            y_off = (i - 1) * 0.15
            points = []
            for x in np.linspace(air_start_x, air_end_x, 35):
                y = air_y_base_in + y_off + 0.05 * np.sin(6 * (x - air_start_x))
                points.append(np.array([x, y, 0]))
            wave = VMobject(color=P_RED, stroke_width=2.5, stroke_opacity=0.85)
            wave.set_points_smoothly(points)
            heat_waves_in.add(wave)

        # Cold air exiting:
        air_start_x_out = hc[0] + 0.5
        air_end_x_out = h["win_bottom"][0] - 2.5
        air_y_base_out = h["win_bottom"][1] + 0.2
        
        cold_waves_out = VGroup()
        for i in range(3):
            y_off = (i - 1) * 0.15
            points = []
            for x in np.linspace(air_start_x_out, air_end_x_out, 35):
                y = air_y_base_out + y_off + 0.05 * np.sin(6 * (x - air_start_x_out))
                points.append(np.array([x, y, 0]))
            wave = VMobject(color=P_BLUE, stroke_width=2.5, stroke_opacity=0.85)
            wave.set_points_smoothly(points)
            cold_waves_out.add(wave)

        # Blue water droplets entering (Moisture)
        droplets = VGroup()
        np.random.seed(42)
        for _ in range(15):
            x = np.random.uniform(air_start_x + 0.2, air_end_x - 0.2)
            y = air_y_base_in + np.random.uniform(-0.25, 0.25)
            drop = Circle(radius=0.065, color=P_BLUE, fill_color=P_BLUE, fill_opacity=0.85, stroke_width=1)
            drop.move_to(np.array([x, y, 0]))
            droplets.add(drop)

        # Airflow arrows
        flow_arrow_in = Arrow(start=LEFT*2.0 + UP*0.1, end=RIGHT*0.2 + UP*0.1, color=P_RED, stroke_width=3, max_tip_length_to_length_ratio=0.2).move_to(win_center + UP*0.2)
        flow_arrow_out = Arrow(start=RIGHT*0.2 + DOWN*0.1, end=LEFT*2.0 + DOWN*0.1, color=P_BLUE, stroke_width=3, max_tip_length_to_length_ratio=0.2).move_to(win_center + DOWN*0.2)

        # ── Interior fill (mixed warm + humid) ──
        interior_fill = Polygon(
            h["bl"] + RIGHT * h["wall_thickness"],
            h["br"] + LEFT * h["wall_thickness"],
            h["tr"] + LEFT * h["wall_thickness"],
            h["roof_peak"] + DOWN * 0.2,
            h["tl"] + RIGHT * h["wall_thickness"],
            fill_color=P_RED, fill_opacity=0.0, stroke_width=0,
        )

        # ── Detailed Parameter Callouts ──
        sens_label = Text("Q̇_sens (Fühlbare Wärme)", font_size=16, color=P_RED)
        sens_label.next_to(eq_group, RIGHT, buff=0.4).shift(UP * 0.2)
        lat_label = Text("Q̇_lat (Latente Feuchte)", font_size=16, color=P_BLUE)
        lat_label.next_to(sens_label, RIGHT, buff=0.4)

        # ── ANIMATION ──
        self.add(h["house"])

        # Beat 3a: Title + Sun returns
        self.play(
            FadeIn(title, shift=DOWN * 0.2),
            FadeIn(sun_group, scale=0.7),
            run_time=1.2,
        )

        # Beat 3b: Zoom focus on window section
        self.play(
            Create(zoom_box),
            run_time=1.5,
        )
        self.play(
            FadeOut(zoom_box),
            run_time=1.5,
        )

        # Beat 3c: Air flow animation through window (Heat waves + Moisture droplets + Flow arrows)
        self.add(interior_fill)
        self.play(
            LaggedStart(*[Create(w) for w in heat_waves_in], lag_ratio=0.1),
            LaggedStart(*[Create(w) for w in cold_waves_out], lag_ratio=0.1),
            LaggedStart(*[FadeIn(d, scale=0.5) for d in droplets], lag_ratio=0.05),
            GrowArrow(flow_arrow_in), GrowArrow(flow_arrow_out),
            run_time=3.5,
        )

        # Beat 3d: Interior fills with mixed thermal & humidity load
        self.play(
            interior_fill.animate.set_fill(opacity=0.35),
            run_time=2.0,
        )

        # Beat 3e: Master Equation appears
        self.play(
            Create(eq_box), FadeIn(eq_text),
            run_time=1.4,
        )

        # Beat 3f: Parameter Callouts (Sensible vs Latent distinction)
        self.play(
            FadeIn(sens_label, shift=UP * 0.15),
            FadeIn(lat_label, shift=UP * 0.15),
            run_time=1.5,
        )

        # Highlight pulse of airflow parameters
        self.play(
            heat_waves_in.animate.set_stroke(opacity=1.0, width=3.5),
            cold_waves_out.animate.set_stroke(opacity=1.0, width=3.5),
            droplets.animate.set_fill(opacity=1.0),
            run_time=1.2,
        )
        self.play(
            heat_waves_in.animate.set_stroke(opacity=0.7, width=2.5),
            cold_waves_out.animate.set_stroke(opacity=0.7, width=2.5),
            droplets.animate.set_fill(opacity=0.75),
            run_time=1.0,
        )

        # Final hold
        self.wait(1.0)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 4 – Sensible vs. Latent Heat (Split Screen)
# ═══════════════════════════════════════════════════════════════════════
class Beat4_SensibleVsLatent(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        # ── Title ──
        title = Text("Sensible vs. Latente Kühlung", font_size=30, color=P_WHITE)
        title.to_edge(UP, buff=0.4)

        # ── Dividing Line ──
        divider = DashedLine(UP * 2.2, DOWN * 2.5, color=P_WHITE, stroke_width=1.5, dash_length=0.15)

        # ── LEFT SIDE: Sensible Heat ──
        sens_title = Text("Sensible", font_size=24, color=P_RED)
        sens_title.move_to(LEFT * 3.2 + UP * 1.8)

        sens_eq = Text("Q̇_sens = ρ_a · c_p,a · ΔΘ · q_v,R", font_size=17, color=P_RED)
        sens_eq_box = SurroundingRectangle(sens_eq, color=P_RED, corner_radius=0.08, buff=0.15, stroke_width=2)
        sens_eq_group = VGroup(sens_eq_box, sens_eq)
        sens_eq_group.move_to(LEFT * 3.2 + DOWN * 2.0)

        # Thermometer
        thermo_x = LEFT * 3.2
        thermo_bulb = Circle(radius=0.2, color=P_WHITE, stroke_width=2, fill_color=P_RED, fill_opacity=0.6).move_to(thermo_x + DOWN * 0.8)
        thermo_tube = Rectangle(width=0.12, height=1.8, color=P_WHITE, stroke_width=2).move_to(thermo_x + UP * 0.2)
        thermo_fill_full = Rectangle(
            width=0.08, height=1.5,
            fill_color=P_RED, fill_opacity=0.8, stroke_width=0,
        ).move_to(thermo_x + UP * 0.15)

        # Temperature labels
        temp_30 = Text("30°C", font_size=16, color=P_RED)
        temp_30.next_to(thermo_tube, RIGHT, buff=0.2).align_to(thermo_tube, UP)
        temp_20 = Text("20°C", font_size=16, color=P_WHITE)
        temp_20.next_to(thermo_tube, RIGHT, buff=0.2).align_to(thermo_tube, DOWN).shift(UP * 0.3)

        thermo_group = VGroup(thermo_tube, thermo_fill_full, thermo_bulb, temp_30, temp_20)

        # ΔΘ label
        delta_theta = Text("ΔΘ", font_size=28, color=P_RED, weight=BOLD)
        delta_theta.next_to(thermo_group, LEFT, buff=0.4)

        # ── RIGHT SIDE: Latent Heat ──
        lat_title = Text("Latent", font_size=24, color=P_BLUE)
        lat_title.move_to(RIGHT * 3.2 + UP * 1.8)

        lat_eq = Text("Q̇_lat = ρ_a · r · Δx · q_v,R", font_size=17, color=P_BLUE)
        lat_eq_box = SurroundingRectangle(lat_eq, color=P_BLUE, corner_radius=0.08, buff=0.15, stroke_width=2)
        lat_eq_group = VGroup(lat_eq_box, lat_eq)
        lat_eq_group.move_to(RIGHT * 3.2 + DOWN * 2.0)

        # Water droplets (will be removed one by one)
        drop_positions = [
            RIGHT * 2.7 + UP * 0.6,
            RIGHT * 2.9 + UP * 0.3,
            RIGHT * 3.4 + UP * 0.7,
            RIGHT * 3.7 + UP * 0.4,
            RIGHT * 2.8 + DOWN * 0.1,
            RIGHT * 3.2 + DOWN * 0.2,
            RIGHT * 3.5 + UP * 0.0,
            RIGHT * 3.1 + UP * 0.9,
        ]
        drops = VGroup(*[
            VGroup(
                Circle(radius=0.1, color=P_BLUE, fill_color=P_BLUE, fill_opacity=0.7, stroke_width=1.5),
                Polygon(
                    UP * 0.15, LEFT * 0.07 + DOWN * 0.02, RIGHT * 0.07 + DOWN * 0.02,
                    fill_color=P_BLUE, fill_opacity=0.7, stroke_width=0,
                ).shift(UP * 0.08),
            ).move_to(pos)
            for pos in drop_positions
        ])

        # Δx label
        delta_x = Text("Δx", font_size=28, color=P_BLUE, weight=BOLD)
        delta_x.move_to(RIGHT * 3.2 + DOWN * 0.6)

        # "Sponge" squeeze effect — bracket
        squeeze_l = Line(RIGHT * 2.2 + UP * 1.1, RIGHT * 2.2 + DOWN * 0.4, color=P_BLUE, stroke_width=2)
        squeeze_r = Line(RIGHT * 4.2 + UP * 1.1, RIGHT * 4.2 + DOWN * 0.4, color=P_BLUE, stroke_width=2)

        # ── Parameter Breakdown Callouts ──
        sens_param_desc = Text("ρ_a: Dichte | c_p,a: W-Kapazität\nΔΘ: Temp-Diff | q_v,R: Luftvolumenstrom", font_size=14, color=P_WHITE, line_spacing=1)
        sens_param_desc.next_to(sens_eq_group, DOWN, buff=0.2)

        lat_param_desc = Text("ρ_a: Dichte | r: Verdampfungswärme\nΔx: Abs. Feuchte | q_v,R: Luftvolumenstrom", font_size=14, color=P_WHITE, line_spacing=1)
        lat_param_desc.next_to(lat_eq_group, DOWN, buff=0.2)

        # ── ANIMATION ──
        # Beat 4a: Title + divider
        self.play(FadeIn(title, shift=DOWN * 0.2), run_time=0.8)
        self.play(Create(divider), run_time=0.6)

        # Beat 4b: Both subtitles
        self.play(
            FadeIn(sens_title, shift=RIGHT * 0.2),
            FadeIn(lat_title, shift=LEFT * 0.2),
            run_time=0.8,
        )

        # Beat 4c: Left — Thermometer appears
        self.play(
            Create(thermo_tube), FadeIn(thermo_fill_full), FadeIn(thermo_bulb),
            FadeIn(temp_30), FadeIn(temp_20),
            run_time=1.2,
        )

        # Beat 4d: Right — Droplets appear
        self.play(
            LaggedStart(*[FadeIn(d, scale=0.5) for d in drops], lag_ratio=0.08),
            run_time=1.2,
        )

        # Beat 4e: Equations + detailed parameter definitions appear
        self.play(
            Create(sens_eq_box), FadeIn(sens_eq), FadeIn(sens_param_desc),
            Create(lat_eq_box), FadeIn(lat_eq), FadeIn(lat_param_desc),
            run_time=1.5,
        )

        # Beat 4f: Left — temperature drops (30→20), ΔΘ pulses
        thermo_fill_low = Rectangle(
            width=0.08, height=0.5,
            fill_color=P_WHITE, fill_opacity=0.6, stroke_width=0,
        ).move_to(thermo_x + DOWN * 0.35)

        self.play(
            Transform(thermo_fill_full, thermo_fill_low),
            FadeIn(delta_theta),
            run_time=2.5,
        )

        # ΔΘ pulse
        dt_pulse = SurroundingRectangle(delta_theta, color=P_RED, buff=0.1, stroke_width=3, corner_radius=0.08)
        self.play(Create(dt_pulse), run_time=0.6)
        self.play(
            dt_pulse.animate.set_stroke(width=5),
            run_time=0.8,
            rate_func=there_and_back,
        )
        self.play(FadeOut(dt_pulse), run_time=0.4)

        # Beat 4g: Right — droplets disappear one by one, Δx pulses
        self.play(FadeIn(squeeze_l), FadeIn(squeeze_r), run_time=0.5)
        self.play(
            squeeze_l.animate.shift(RIGHT * 0.3),
            squeeze_r.animate.shift(LEFT * 0.3),
            run_time=1.0,
        )
        self.play(
            LaggedStart(*[FadeOut(d, scale=0.3, shift=DOWN * 0.3) for d in drops], lag_ratio=0.15),
            FadeIn(delta_x),
            run_time=3.5,
        )

        # Δx pulse
        dx_pulse = SurroundingRectangle(delta_x, color=P_BLUE, buff=0.1, stroke_width=3, corner_radius=0.08)
        self.play(Create(dx_pulse), run_time=0.6)
        self.play(
            dx_pulse.animate.set_stroke(width=5),
            run_time=1.0,
            rate_func=there_and_back,
        )
        self.play(FadeOut(dx_pulse), run_time=0.4)

        # Extra hold for formula VO (sensible vs latent)
        self.wait(5.0)

        # Final hold
        self.play(
            FadeOut(squeeze_l), FadeOut(squeeze_r),
            run_time=0.4,
        )
        self.wait(1.4)
