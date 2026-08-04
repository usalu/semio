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

MAIN_TITLE = "Mechanische Wohnungslüftung: Auslegung"


def _main_title():
    """🌬️ Persistent chapter header shared by every beat of this topic."""
    return Text(MAIN_TITLE, font_size=26, color=P_CYAN).to_edge(UP, buff=0.28)


def _equation_row(parts, font_size=24, buff=0.14):
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


def _build_room(center=DOWN * 0.35, width=7.4, height=3.55):
    """🏠 Line-art interior room with teal floor — matches Cooling HVAC scenes."""
    room = Rectangle(
        width=width, height=height,
        color=P_WHITE, stroke_width=3.5, fill_opacity=0,
    ).move_to(center)
    floor = Line(
        room.get_corner(DL), room.get_corner(DR),
        color=P_TEAL, stroke_width=4,
    )
    return {"room": room, "floor": floor, "center": np.array(center), "w": width, "h": height}


def _heat_cloud(center, scale=1.0):
    """🌡️ Layered pastel-red heat cloud filling a saturated room."""
    return VGroup(*[
        Ellipse(
            width=w * scale, height=h * scale,
            color=P_RED, stroke_width=0,
            fill_color=P_RED, fill_opacity=op,
        ).move_to(center + UP * (h * scale * 0.04))
        for w, h, op in ((5.9, 2.5, 0.14), (4.5, 1.8, 0.20), (3.1, 1.15, 0.30))
    ])


def _vent_unit(pos, color=P_TEAL, width=1.55, height=0.34):
    """🔧 Ceiling RLT grille block with hatch lines."""
    body = RoundedRectangle(
        width=width, height=height, corner_radius=0.06,
        color=color, stroke_width=2.5,
        fill_color=color, fill_opacity=0.3,
    ).move_to(pos)
    grille = VGroup()
    for t in (-0.38, -0.12, 0.12, 0.38):
        grille.add(Line(
            body.get_left() + RIGHT * 0.18 + UP * t * 0.07,
            body.get_right() + LEFT * 0.18 + UP * t * 0.07,
            color=color, stroke_width=1.5, stroke_opacity=0.95,
        ))
    return VGroup(body, grille)


def _air_particles(paths, color, radius_range=(0.055, 0.09), seed=7):
    """💨 Opaque airflow dots that travel along precomputed paths."""
    rng = np.random.default_rng(seed)
    dots = VGroup()
    for path in paths:
        dots.add(Dot(
            point=path.get_start(),
            radius=float(rng.uniform(*radius_range)),
            color=color, fill_opacity=1.0, stroke_width=0,
        ))
    return dots


def _smooth_path(points):
    """〰️ Smooth polyline path for airflow particles."""
    path = VMobject()
    path.set_points_smoothly([np.array(p, dtype=float) for p in points])
    return path


def _supply_paths(supply_bottom, room_c, n=16, seed=3):
    """➡️ Cool air enters from the supply grille and spreads through the room."""
    rng = np.random.default_rng(seed)
    paths = VGroup()
    for i in range(n):
        start = supply_bottom + np.array([float(rng.uniform(-0.45, 0.45)), 0.02, 0.0])
        mid = room_c + np.array([
            float(rng.uniform(-2.2, 0.6)),
            float(rng.uniform(-0.9, 0.55)),
            0.0,
        ])
        end = room_c + np.array([
            float(rng.uniform(-2.4, 2.0)),
            float(rng.uniform(-1.2, 0.35)),
            0.0,
        ])
        paths.add(_smooth_path([start, mid, end]))
    return paths


def _exhaust_paths(room_c, exhaust_bottom, n=16, seed=11):
    """⬅️ Warm air drifts through the room then exits into the return grille."""
    rng = np.random.default_rng(seed)
    paths = VGroup()
    for i in range(n):
        start = room_c + np.array([
            float(rng.uniform(-2.3, 1.8)),
            float(rng.uniform(-1.15, 0.45)),
            0.0,
        ])
        mid = room_c + np.array([
            float(rng.uniform(0.2, 2.4)),
            float(rng.uniform(-0.4, 0.9)),
            0.0,
        ])
        end = exhaust_bottom + np.array([float(rng.uniform(-0.35, 0.35)), 0.02, 0.0])
        paths.add(_smooth_path([start, mid, end]))
    return paths


def _step_label(text, color=P_WHITE):
    """📌 Bottom-centered instructional caption for pacing a longer beat."""
    return Text(text, font_size=16, color=color).to_edge(DOWN, buff=0.24)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 1 – Mechanical Supply/Exhaust System
# ═══════════════════════════════════════════════════════════════════════
class Beat1_MechanicalVentilation(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        built = _build_room()
        room, floor, room_c = built["room"], built["floor"], built["center"]
        warm_wash = Rectangle(
            width=built["w"] - 0.1, height=built["h"] - 0.1,
            stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
        ).move_to(room_c)

        supply = _vent_unit(room.get_top() + DOWN * 0.3 + LEFT * 2.25, color=P_CYAN)
        exhaust = _vent_unit(room.get_top() + DOWN * 0.3 + RIGHT * 2.25, color=P_ORANGE)

        unit = RoundedRectangle(
            width=2.1, height=0.52, corner_radius=0.08,
            color=P_TEAL, stroke_width=2.5,
            fill_color=P_TEAL, fill_opacity=0.4,
        ).move_to(room.get_top() + UP * 0.58)
        fans = VGroup(*[
            Circle(radius=0.11, color=P_WHITE, stroke_width=1.5).move_to(unit.get_center() + RIGHT * dx)
            for dx in (-0.45, 0.0, 0.45)
        ])
        unit_tag = Text("RLT-Anlage", font_size=15, color=P_WHITE).next_to(unit, UP, buff=0.1)
        rlt = VGroup(unit, fans)

        duct_sup = Line(unit.get_bottom() + LEFT * 0.4, supply.get_top(), color=P_CYAN, stroke_width=7)
        duct_exh = Line(unit.get_bottom() + RIGHT * 0.4, exhaust.get_top(), color=P_ORANGE, stroke_width=7)
        sys_lbl = Text("Zu-/Abluftsystem", font_size=17, color=P_TEAL)
        sys_lbl.next_to(rlt, RIGHT, buff=0.3)

        zuluft = VGroup(
            Text("Zuluft", font_size=17, color=P_CYAN),
            Text("kühl · aufbereitet", font_size=13, color=P_TEAL),
        ).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        zuluft.next_to(room, LEFT, buff=0.42)
        zuluft.shift(UP * 1.05)

        abluft = VGroup(
            Text("Abluft", font_size=17, color=P_ORANGE),
            Text("warm · abgeführt", font_size=13, color=P_ORANGE),
        ).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        abluft.next_to(room, RIGHT, buff=0.42)
        abluft.shift(UP * 1.05)

        step1 = _step_label("1  Raum erwärmt sich und speichert Wärmelast", P_RED)
        step2 = _step_label("2  RLT-Anlage bringt Zuluft und führt Abluft ab", P_TEAL)
        step3 = _step_label("3  Die Raumfarbe zeigt die Abkühlung", P_CYAN)
        step4 = _step_label("4  Welche Volumenstromrate ist nötig?", P_WHITE)

        supply_flow = VGroup(
            CurvedArrow(
                supply.get_bottom() + DOWN * 0.05,
                room_c + LEFT * 1.8 + DOWN * 0.15,
                angle=0.35 * PI,
                color=P_CYAN,
                stroke_width=5,
            ),
            CurvedArrow(
                supply.get_bottom() + DOWN * 0.08 + RIGHT * 0.1,
                room_c + LEFT * 0.15,
                angle=0.22 * PI,
                color=P_CYAN,
                stroke_width=5,
            ),
            CurvedArrow(
                supply.get_bottom() + DOWN * 0.05 + RIGHT * 0.18,
                room_c + RIGHT * 1.25 + UP * 0.18,
                angle=0.08 * PI,
                color=P_CYAN,
                stroke_width=5,
            ),
        )
        exhaust_flow = VGroup(
            CurvedArrow(
                room_c + RIGHT * 0.8 + DOWN * 0.3,
                exhaust.get_bottom() + DOWN * 0.04 + LEFT * 0.1,
                angle=-0.14 * PI,
                color=P_ORANGE,
                stroke_width=5,
            ),
            CurvedArrow(
                room_c + RIGHT * 1.75 + UP * 0.05,
                exhaust.get_bottom() + DOWN * 0.02,
                angle=-0.24 * PI,
                color=P_ORANGE,
                stroke_width=5,
            ),
        )

        self.add(warm_wash)
        self.play(Write(title), run_time=1.6)
        self.play(Create(room), Create(floor), run_time=1.6)
        self.play(
            FadeIn(step1),
            warm_wash.animate.set_fill(opacity=0.18),
            run_time=1.8,
        )
        self.wait(0.6)

        self.play(FadeOut(step1), FadeIn(step2), run_time=0.7)
        self.play(
            FadeIn(rlt), FadeIn(unit_tag), FadeIn(sys_lbl),
            run_time=1.4,
        )
        self.play(
            Create(duct_sup), Create(duct_exh),
            FadeIn(supply), FadeIn(exhaust),
            FadeIn(zuluft), FadeIn(abluft),
            run_time=2.0,
        )
        self.wait(0.5)

        self.play(FadeOut(step2), FadeIn(step3), run_time=0.7)
        self.play(
            LaggedStart(*[Create(a) for a in supply_flow], lag_ratio=0.18),
            warm_wash.animate.set_fill(P_CYAN, opacity=0.10),
            room.animate.set_stroke(color=P_CYAN),
            floor.animate.set_color(P_CYAN),
            run_time=2.8,
        )
        self.play(
            LaggedStart(*[Create(a) for a in exhaust_flow], lag_ratio=0.2),
            run_time=1.8,
        )
        self.play(FadeOut(step3), FadeIn(step4), run_time=0.8)
        self.wait(1.0)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 2 – Thermodynamic Volume-Flow Equation
# ═══════════════════════════════════════════════════════════════════════
class Beat2_VolumeFlowEquation(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Konvektive Kühlleistung der Zuluft", font_size=18, color=P_TEAL)
        subtitle.next_to(title, DOWN, buff=0.12)

        built = _build_room(center=LEFT * 2.4 + DOWN * 0.2, width=5.0, height=2.3)
        room, floor, room_c = built["room"], built["floor"], built["center"]
        supply = _vent_unit(room.get_top() + DOWN * 0.22 + LEFT * 1.25, color=P_CYAN, width=1.25)

        t_supply = VGroup(
            Text("θ_Zu", font_size=15, color=P_CYAN),
            Text("18 °C", font_size=18, color=P_CYAN),
        ).arrange(DOWN, buff=0.06)
        t_supply_frame = SurroundingRectangle(t_supply, color=P_CYAN, corner_radius=0.1, buff=0.14, stroke_width=1.8)
        t_supply = VGroup(t_supply_frame, t_supply)
        t_supply.next_to(room, DOWN, buff=0.28).shift(LEFT * 1.1)

        t_room = VGroup(
            Text("θ_Raum", font_size=15, color=P_ORANGE),
            Text("25 °C", font_size=18, color=P_ORANGE),
        ).arrange(DOWN, buff=0.06)
        t_room_frame = SurroundingRectangle(t_room, color=P_ORANGE, corner_radius=0.1, buff=0.14, stroke_width=1.8)
        t_room = VGroup(t_room_frame, t_room)
        t_room.next_to(room, DOWN, buff=0.28).shift(RIGHT * 1.1)

        delta_card = VGroup(
            Text("Temperaturhub", font_size=13, color=P_TEAL),
            Text("Δθ = 25 − 18 = 7 K", font_size=18, color=P_BLUE),
        ).arrange(DOWN, buff=0.06)
        delta_frame = SurroundingRectangle(delta_card, color=P_BLUE, corner_radius=0.1, buff=0.14, stroke_width=1.8)
        delta_card = VGroup(delta_frame, delta_card)
        delta_card.next_to(t_supply, DOWN, buff=0.22).set_x(room_c[0])

        temp_arrow = Arrow(
            t_supply.get_right() + RIGHT * 0.08,
            t_room.get_left() + LEFT * 0.08,
            buff=0.04,
            stroke_width=4,
            max_tip_length_to_length_ratio=0.12,
            color=P_BLUE,
        )

        cool_paths = _supply_paths(supply.get_bottom(), room_c + DOWN * 0.15, n=14, seed=21)
        cool_dots = _air_particles(cool_paths, P_CYAN, seed=21)

        eq, items = _equation_row(
            [
                ("qv", "Q̇_V", P_CYAN),
                (None, "=", P_WHITE),
                ("rho", "ρ_a", P_GREEN),
                (None, "·", P_WHITE),
                ("cp", "c_p,a", P_GREEN),
                (None, "·", P_WHITE),
                ("dth", "Δθ", P_BLUE),
                (None, "·", P_WHITE),
                ("qvr", "q_v,R", P_YELLOW),
            ],
            font_size=27,
            buff=0.15,
        )
        eq.move_to(RIGHT * 2.95 + UP * 1.15)
        eq_box = SurroundingRectangle(eq, color=P_WHITE, corner_radius=0.12, buff=0.24, stroke_width=2.5)

        cards = VGroup(
            VGroup(
                Text("Luftdichte", font_size=14, color=P_TEAL),
                Text("ρ_a = 1,29 kg/m³", font_size=17, color=P_GREEN),
            ).arrange(DOWN, buff=0.08),
            VGroup(
                Text("spez. Wärmekapazität", font_size=14, color=P_TEAL),
                Text("c_p,a = 1,0 kJ/kgK", font_size=17, color=P_GREEN),
            ).arrange(DOWN, buff=0.08),
            VGroup(
                Text("Temperaturdifferenz", font_size=14, color=P_TEAL),
                Text("Δθ = 25 − 18 = 7 K", font_size=17, color=P_BLUE),
            ).arrange(DOWN, buff=0.08),
            VGroup(
                Text("gesuchter Volumenstrom", font_size=14, color=P_TEAL),
                Text("q_v,R  [m³/s]", font_size=17, color=P_YELLOW),
            ).arrange(DOWN, buff=0.08),
        ).arrange_in_grid(rows=2, cols=2, buff=(0.28, 0.3))
        for card in cards:
            frame = SurroundingRectangle(card, color=P_TEAL, corner_radius=0.1, buff=0.14, stroke_width=1.8)
            card.add_to_back(frame)
        cards.next_to(eq, DOWN, buff=0.38)
        cards.set_x(eq.get_x())

        step1 = _step_label("1  Kühle Zuluft strömt in den warmen Raum", P_CYAN)
        step2 = _step_label("2  Stoffwerte und Temperaturhub bestimmen die Kühlleistung", P_GREEN)
        step3 = _step_label("3  q_v,R bestimmt, wie viel Luft wir brauchen", P_YELLOW)

        self.play(Write(title), run_time=1.5)
        self.play(FadeIn(subtitle), run_time=0.6)
        self.play(Create(room), Create(floor), FadeIn(supply), run_time=1.5)
        self.play(FadeIn(step1), FadeIn(t_supply), FadeIn(t_room), run_time=1.2)
        self.add(cool_dots)
        self.play(
            AnimationGroup(*[
                MoveAlongPath(d, path, rate_func=linear)
                for d, path in zip(cool_dots, cool_paths)
            ], lag_ratio=0.05),
            FadeIn(temp_arrow),
            FadeIn(delta_card),
            run_time=4.5,
        )

        self.play(FadeOut(step1), FadeIn(step2), run_time=0.7)
        self.play(FadeIn(eq), Create(eq_box), run_time=1.5)
        self.play(FadeIn(cards[0]), items["rho"].animate.scale(1.2), run_time=1.2)
        self.play(items["rho"].animate.scale(1 / 1.2), run_time=0.45)
        self.play(FadeIn(cards[1]), items["cp"].animate.scale(1.2), run_time=1.2)
        self.play(items["cp"].animate.scale(1 / 1.2), run_time=0.45)
        self.play(FadeIn(cards[2]), items["dth"].animate.scale(1.25).set_color(P_BLUE), run_time=1.3)
        self.play(items["dth"].animate.scale(1 / 1.25), run_time=0.45)

        self.play(FadeOut(step2), FadeIn(step3), run_time=0.7)
        self.play(FadeIn(cards[3]), items["qvr"].animate.scale(1.25).set_color(P_YELLOW), run_time=1.4)
        self.play(items["qvr"].animate.scale(1 / 1.25), eq_box.animate.set_stroke(width=4), run_time=0.8)
        self.wait(1.0)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 3 – Isolate Required Airflow q_v,R
# ═══════════════════════════════════════════════════════════════════════
class Beat3_IsolateAirflow(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Gleichgewicht: Kühlleistung = Kühllast", font_size=18, color=P_CYAN)
        subtitle.next_to(title, DOWN, buff=0.12)

        # Balance scale metaphor
        beam = Line(LEFT * 2.8, RIGHT * 2.8, color=P_WHITE, stroke_width=4).move_to(UP * 0.85)
        pivot = Triangle(color=P_TEAL, fill_opacity=1).scale(0.18).rotate(PI).next_to(beam, DOWN, buff=0)
        left_pan = RoundedRectangle(
            width=2.2, height=0.9, corner_radius=0.1,
            color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN, fill_opacity=0.12,
        ).next_to(beam.get_left(), DOWN, buff=0.35).shift(RIGHT * 0.9)
        right_pan = RoundedRectangle(
            width=2.2, height=0.9, corner_radius=0.1,
            color=P_YELLOW, stroke_width=2.5, fill_color=P_YELLOW, fill_opacity=0.12,
        ).next_to(beam.get_right(), DOWN, buff=0.35).shift(LEFT * 0.9)
        left_txt = Text("Q̇_V\nKühlleistung", font_size=16, color=P_CYAN).move_to(left_pan)
        right_txt = Text("Q̇_S,tr\nKühllast", font_size=16, color=P_YELLOW).move_to(right_pan)
        eq_mark = Text("=", font_size=36, color=P_WHITE).move_to(beam.get_center() + DOWN * 0.55)

        start_eq, start_items = _equation_row(
            [
                ("qv", "Q̇_V", P_CYAN),
                (None, "=", P_WHITE),
                ("rho", "ρ_a", P_GREEN),
                (None, "·", P_WHITE),
                ("cp", "c_p,a", P_GREEN),
                (None, "·", P_WHITE),
                ("dth", "Δθ", P_BLUE),
                (None, "·", P_WHITE),
                ("qvr", "q_v,R", P_YELLOW),
            ],
            font_size=24,
            buff=0.12,
        )
        start_eq.move_to(DOWN * 2.1)

        sub_eq, sub_items = _equation_row(
            [
                ("qstr", "Q̇_S,tr", P_YELLOW),
                (None, "=", P_WHITE),
                ("rho", "ρ_a", P_GREEN),
                (None, "·", P_WHITE),
                ("cp", "c_p,a", P_GREEN),
                (None, "·", P_WHITE),
                ("dth", "Δθ", P_BLUE),
                (None, "·", P_WHITE),
                ("qvr", "q_v,R", P_CYAN),
            ],
            font_size=26,
            buff=0.13,
        )
        sub_eq.move_to(DOWN * 1.15)

        lhs, lhs_items = _equation_row(
            [
                ("qvr", "q_v,R", P_CYAN),
                (None, "=", P_WHITE),
            ],
            font_size=28,
            buff=0.13,
        )
        num = Text("Q̇_S,tr", font_size=26, color=P_YELLOW)
        den, den_items = _equation_row(
            [
                ("rho", "ρ_a", P_GREEN),
                (None, "·", P_WHITE),
                ("cp", "c_p,a", P_GREEN),
                (None, "·", P_WHITE),
                ("dth", "Δθ", P_BLUE),
            ],
            font_size=24,
            buff=0.12,
        )
        frac_line = Line(LEFT, RIGHT, color=P_WHITE, stroke_width=2.5)
        frac_line.scale_to_fit_width(max(num.width, den.width) + 0.2)
        frac = VGroup(num, frac_line, den).arrange(DOWN, buff=0.12)
        final_eq = VGroup(lhs, frac).arrange(RIGHT, buff=0.22)
        final_eq.move_to(DOWN * 0.62 + RIGHT * 0.15)
        final_box = SurroundingRectangle(
            final_eq, color=P_CYAN, corner_radius=0.14, buff=0.28, stroke_width=3,
        )

        load_token = Text("Q̇_S,tr", font_size=22, color=P_YELLOW).move_to(right_pan.get_center())
        load_path = ArcBetweenPoints(
            right_pan.get_bottom() + DOWN * 0.12,
            sub_eq.get_center() + LEFT * 2.95 + UP * 0.04,
            angle=-0.28 * PI,
        )
        load_hint = Text("Kühllast wird übernommen", font_size=14, color=P_YELLOW)
        load_hint.next_to(load_path.point_from_proportion(0.45), RIGHT, buff=0.18).shift(DOWN * 0.08)

        step1 = _step_label("1  Kühlleistung muss der solaren Kühllast gleichen", P_ORANGE)
        step2 = _step_label("2  Ersetze Q̇_V durch Q̇_S,tr", P_YELLOW)
        step3 = _step_label("3  Isoliere q_v,R — das ist der Auslegungswert", P_CYAN)

        self.play(Write(title), run_time=1.5)
        self.play(FadeIn(subtitle), run_time=0.6)
        self.play(FadeIn(step1), run_time=0.7)
        self.play(
            Create(beam), FadeIn(pivot),
            FadeIn(left_pan), FadeIn(right_pan),
            FadeIn(left_txt), FadeIn(right_txt),
            run_time=2.2,
        )
        self.play(FadeIn(eq_mark), run_time=0.8)
        self.wait(0.7)

        self.play(FadeOut(step1), FadeIn(step2), run_time=0.7)
        self.play(FadeIn(start_eq), run_time=1.3)
        self.play(Indicate(left_txt, color=P_CYAN), Indicate(right_txt, color=P_YELLOW), run_time=1.4)
        self.play(
            FadeIn(load_token),
            Create(load_path),
            FadeIn(load_hint),
            run_time=0.9,
        )
        self.play(
            MoveAlongPath(load_token, load_path, rate_func=smooth),
            TransformMatchingShapes(start_eq.copy(), sub_eq),
            run_time=1.5,
        )
        self.play(FadeOut(load_token), FadeOut(load_path), FadeOut(load_hint), run_time=0.5)
        self.wait(0.5)

        self.play(FadeOut(step2), FadeIn(step3), run_time=0.7)
        self.play(
            FadeOut(beam), FadeOut(pivot), FadeOut(left_pan), FadeOut(right_pan),
            FadeOut(left_txt), FadeOut(right_txt), FadeOut(eq_mark),
            FadeOut(start_eq), FadeOut(sub_eq),
            run_time=1.2,
        )
        self.play(FadeIn(final_eq), run_time=1.4)
        self.play(Create(final_box), run_time=1.0)

        # Walk through each piece of the isolated formula
        self.play(lhs_items["qvr"].animate.scale(1.3).set_color(P_CYAN), run_time=1.0)
        self.play(lhs_items["qvr"].animate.scale(1 / 1.3), run_time=0.45)
        self.play(num.animate.scale(1.2).set_color(P_YELLOW), run_time=1.0)
        self.play(num.animate.scale(1 / 1.2), run_time=0.45)
        self.play(
            den_items["rho"].animate.scale(1.15),
            den_items["cp"].animate.scale(1.15),
            den_items["dth"].animate.scale(1.15),
            run_time=1.1,
        )
        self.play(
            den_items["rho"].animate.scale(1 / 1.15),
            den_items["cp"].animate.scale(1 / 1.15),
            den_items["dth"].animate.scale(1 / 1.15),
            final_box.animate.set_stroke(width=5),
            run_time=1.0,
        )
        self.play(final_box.animate.set_stroke(width=3), run_time=0.5)
        self.wait(1.0)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 4 – Duct Cross-Section & Continuity
# ═══════════════════════════════════════════════════════════════════════
class Beat4_DuctCrossSection(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Vom Volumenstrom zum Kanalquerschnitt", font_size=18, color=P_TEAL)
        subtitle.next_to(title, DOWN, buff=0.12)

        duct_c = LEFT * 2.8 + DOWN * 0.25
        outer = Circle(radius=1.4, color=P_TEAL, stroke_width=4).move_to(duct_c)
        wall = Annulus(
            inner_radius=1.18, outer_radius=1.4,
            color=P_TEAL, fill_opacity=0.35, stroke_width=0,
        ).move_to(duct_c)
        area_fill = Circle(
            radius=1.18, color=P_BLUE, stroke_width=0,
            fill_color=P_BLUE, fill_opacity=0.30,
        ).move_to(duct_c)
        area_lbl = Text("A", font_size=44, color=P_BLUE).move_to(duct_c)

        # Perspective pipe body
        pipe_top = Line(duct_c + RIGHT * 1.4 + UP * 0.95, duct_c + RIGHT * 3.6 + UP * 0.5, color=P_TEAL, stroke_width=3)
        pipe_bot = Line(duct_c + RIGHT * 1.4 + DOWN * 0.95, duct_c + RIGHT * 3.6 + DOWN * 0.5, color=P_TEAL, stroke_width=3)
        pipe_end = Ellipse(width=0.55, height=1.0, color=P_TEAL, stroke_width=3).move_to(duct_c + RIGHT * 3.6)
        pipe = VGroup(pipe_top, pipe_bot, pipe_end)

        # Continuous particles through the duct
        rng = np.random.default_rng(42)
        flow_paths = VGroup()
        for i in range(12):
            y = float(rng.uniform(-0.7, 0.7))
            start = duct_c + LEFT * 1.0 + UP * y * 0.85
            end = duct_c + RIGHT * 3.4 + UP * y * 0.35
            mid = duct_c + RIGHT * 1.2 + UP * y * 0.55
            flow_paths.add(_smooth_path([start, mid, end]))
        flow_dots = _air_particles(flow_paths, P_CYAN, radius_range=(0.05, 0.08), seed=42)

        vm_tag = Text("v_m ≈ 2,5 m/s  (lärmarm)", font_size=18, color=P_CYAN)
        vm_tag.next_to(duct_c, DOWN, buff=1.65)

        cont, cont_items = _equation_row(
            [
                ("qvr", "q_v,R", P_YELLOW),
                (None, "=", P_WHITE),
                ("vm", "v_m", P_CYAN),
                (None, "·", P_WHITE),
                ("A", "A", P_BLUE),
            ],
            font_size=26,
            buff=0.15,
        )
        cont.move_to(RIGHT * 3.35 + UP * 1.1)

        area_eq, area_items = _equation_row(
            [
                ("A", "A", P_BLUE),
                (None, "=", P_WHITE),
                ("qvr", "q_v,R", P_YELLOW),
                (None, "/", P_WHITE),
                ("vm", "v_m", P_CYAN),
            ],
            font_size=26,
            buff=0.15,
        )
        area_eq.move_to(RIGHT * 3.35 + DOWN * 0.45)
        area_box = SurroundingRectangle(
            area_eq, color=P_WHITE, corner_radius=0.12, buff=0.22, stroke_width=2.5,
        )

        tip_fast = Text("kleiner A → höhere v_m", font_size=15, color=P_ORANGE)
        tip_slow = Text("größerer A → niedrigere v_m", font_size=15, color=P_CYAN)
        tip_fast.move_to(RIGHT * 3.35 + DOWN * 1.45)
        tip_slow.move_to(RIGHT * 3.35 + DOWN * 1.45)

        step1 = _step_label("1  Volumenstrom fließt durch den Kanalquerschnitt", P_CYAN)
        step2 = _step_label("2  Kontinuität: q_v,R = v_m · A", P_WHITE)
        step3 = _step_label("3  Bei begrenzter Geschwindigkeit folgt die nötige Fläche A", P_BLUE)

        self.play(Write(title), run_time=1.5)
        self.play(FadeIn(subtitle), run_time=0.55)
        self.play(FadeIn(step1), run_time=0.6)
        self.play(Create(outer), FadeIn(wall), Create(pipe), run_time=2.0)
        self.play(FadeIn(area_fill), FadeIn(area_lbl), run_time=1.1)
        self.add(flow_dots)
        self.play(
            AnimationGroup(*[
                MoveAlongPath(d, path, rate_func=linear)
                for d, path in zip(flow_dots, flow_paths)
            ], lag_ratio=0.06),
            FadeIn(vm_tag),
            run_time=4.0,
        )

        self.play(FadeOut(step1), FadeIn(step2), run_time=0.7)
        self.play(FadeIn(cont), run_time=1.3)
        self.play(
            cont_items["qvr"].animate.scale(1.2),
            cont_items["vm"].animate.scale(1.2),
            cont_items["A"].animate.scale(1.2),
            run_time=1.2,
        )
        self.play(
            cont_items["qvr"].animate.scale(1 / 1.2),
            cont_items["vm"].animate.scale(1 / 1.2),
            cont_items["A"].animate.scale(1 / 1.2),
            run_time=0.5,
        )

        self.play(FadeOut(step2), FadeIn(step3), run_time=0.7)
        self.play(FadeOut(cont), FadeIn(area_eq), Create(area_box), run_time=1.5)

        # Demonstrate inverse relationship twice, slowly
        self.play(FadeIn(tip_fast), run_time=0.6)
        self.play(
            area_fill.animate.scale(0.62),
            area_lbl.animate.scale(0.62),
            flow_dots.animate.set_color(P_ORANGE),
            run_time=2.0,
        )
        self.play(FadeOut(tip_fast), FadeIn(tip_slow), run_time=0.6)
        self.play(
            area_fill.animate.scale(1 / 0.62 * 1.2),
            area_lbl.animate.scale(1 / 0.62 * 1.2),
            flow_dots.animate.set_color(P_CYAN),
            run_time=2.2,
        )
        self.play(
            area_fill.animate.scale(1 / 1.2),
            area_lbl.animate.scale(1 / 1.2),
            FadeOut(tip_slow),
            run_time=1.2,
        )
        self.wait(0.9)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 5 – Calculate Duct Radius
# ═══════════════════════════════════════════════════════════════════════
class Beat5_CalculateRadius(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        Text.set_default(font="Serif")

        title = _main_title()
        subtitle = Text("Runder Kanal: von der Fläche zum Radius", font_size=18, color=P_YELLOW)
        subtitle.next_to(title, DOWN, buff=0.12)

        duct_c = LEFT * 2.9 + DOWN * 0.2
        circle = Circle(radius=1.6, color=P_TEAL, stroke_width=4).move_to(duct_c)
        fill = Circle(
            radius=1.6, color=P_BLUE, stroke_width=0,
            fill_color=P_BLUE, fill_opacity=0.2,
        ).move_to(duct_c)
        center_dot = Dot(duct_c, radius=0.07, color=P_YELLOW)
        radius_line = Line(duct_c, duct_c + RIGHT * 1.6, color=P_YELLOW, stroke_width=4)
        r_lbl = Text("r", font_size=30, color=P_YELLOW).next_to(radius_line, UP, buff=0.12)

        # Diameter helper for intuition
        diam = DashedLine(
            duct_c + LEFT * 1.6, duct_c + RIGHT * 1.6,
            color=P_WHITE, stroke_width=2, stroke_opacity=0.55,
        )
        d_lbl = Text("d = 2r", font_size=16, color=P_WHITE).next_to(diam, DOWN, buff=0.18)

        area_eq, area_items = _equation_row(
            [
                ("A", "A", P_BLUE),
                (None, "=", P_WHITE),
                (None, "π", P_WHITE),
                (None, "·", P_WHITE),
                ("r2", "r²", P_YELLOW),
            ],
            font_size=28,
            buff=0.16,
        )
        area_eq.move_to(RIGHT * 3.35 + UP * 1.0)

        master, master_items = _equation_row(
            [
                ("r", "r", P_YELLOW),
                (None, "=", P_WHITE),
                (None, "√", P_WHITE),
                (None, "(", P_WHITE),
                ("A", "A", P_BLUE),
                (None, "/", P_WHITE),
                (None, "π", P_WHITE),
                (None, ")", P_WHITE),
            ],
            font_size=30,
            buff=0.12,
        )
        master.move_to(RIGHT * 3.35 + DOWN * 0.55)
        master_box = SurroundingRectangle(
            master, color=P_YELLOW, corner_radius=0.14, buff=0.3, stroke_width=3,
        )

        chain = VGroup(
            Text("Q̇_S,tr", font_size=16, color=P_YELLOW),
            Text("→", font_size=16, color=P_WHITE),
            Text("q_v,R", font_size=16, color=P_CYAN),
            Text("→", font_size=16, color=P_WHITE),
            Text("A", font_size=16, color=P_BLUE),
            Text("→", font_size=16, color=P_WHITE),
            Text("r", font_size=16, color=P_YELLOW),
        ).arrange(RIGHT, buff=0.18)
        chain.to_edge(DOWN, buff=0.85)

        step1 = _step_label("1  Der Querschnitt A ist eine Kreisfläche", P_BLUE)
        step2 = _step_label("2  A = π · r²  — Geometrie des Rohrs", P_WHITE)
        step3 = _step_label("3  Auflösen nach r liefert die Kanaldimension", P_YELLOW)

        self.play(Write(title), run_time=1.5)
        self.play(FadeIn(subtitle), run_time=0.55)
        self.play(FadeIn(step1), run_time=0.6)
        self.play(Create(circle), FadeIn(fill), run_time=1.8)
        self.play(Create(diam), FadeIn(d_lbl), run_time=1.1)
        self.play(
            FadeIn(center_dot),
            Create(radius_line),
            FadeIn(r_lbl),
            run_time=1.5,
        )
        self.wait(0.5)

        self.play(FadeOut(step1), FadeIn(step2), run_time=0.7)
        self.play(FadeIn(area_eq), run_time=1.3)
        self.play(area_items["A"].animate.scale(1.2), run_time=0.9)
        self.play(area_items["A"].animate.scale(1 / 1.2), run_time=0.4)
        self.play(area_items["r2"].animate.scale(1.25).set_color(P_YELLOW), run_time=1.0)
        self.play(area_items["r2"].animate.scale(1 / 1.25), run_time=0.45)
        self.wait(0.4)

        self.play(FadeOut(step2), FadeIn(step3), run_time=0.7)
        self.play(ReplacementTransform(area_eq, master), run_time=1.8)
        self.play(Create(master_box), run_time=1.0)
        self.play(FadeIn(chain), run_time=1.2)
        self.play(
            master_box.animate.set_stroke(width=6),
            master_items["r"].animate.scale(1.25),
            run_time=1.1,
        )
        self.play(
            master_box.animate.set_stroke(width=3),
            master_items["r"].animate.scale(1 / 1.25),
            run_time=0.8,
        )
        self.play(master_box.animate.set_stroke(width=5), run_time=0.7)
        self.play(master_box.animate.set_stroke(width=3), run_time=0.55)
        self.wait(1.0)
