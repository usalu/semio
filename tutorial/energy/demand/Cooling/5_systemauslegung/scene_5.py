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
    convection_stream, symbol_token, watt_anchor,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Mechanische Wohnungslüftung: Auslegung"

# Mid-screen anchor for rooms / ducts (clear of title + formula/caption).
CONTENT_CENTER = UP * 0.1


#region Shared visual motifs

def _build_room(center=CONTENT_CENTER, width=7.4, height=3.55):
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


def _supply_paths(supply_bottom, room_c, n=16, seed=3, half_w=2.2, half_h=0.9, margin=0.22):
    """➡️ Cool air enters from the supply grille and stays inside the room bounds."""
    rng = np.random.default_rng(seed)
    paths = VGroup()
    x_lo, x_hi = -half_w + margin, half_w - margin
    y_lo, y_hi = -half_h + margin, half_h - margin

    def _inside(pt):
        return np.array([
            float(np.clip(pt[0], room_c[0] + x_lo, room_c[0] + x_hi)),
            float(np.clip(pt[1], room_c[1] + y_lo, room_c[1] + y_hi)),
            0.0,
        ])

    for _ in range(n):
        start = _inside(np.array(supply_bottom, dtype=float) + np.array([
            float(rng.uniform(-0.32, 0.32)), -0.06, 0.0,
        ]))
        mid = _inside(room_c + np.array([
            float(rng.uniform(x_lo * 0.65, x_hi * 0.35)),
            float(rng.uniform(y_lo * 0.15, y_hi * 0.55)),
            0.0,
        ]))
        end = _inside(room_c + np.array([
            float(rng.uniform(x_lo * 0.8, x_hi * 0.8)),
            float(rng.uniform(y_lo * 0.85, y_hi * 0.05)),
            0.0,
        ]))
        path = _smooth_path([start, mid, end])
        # Smooth beziers can bulge past control points — clamp every sample.
        for i in range(len(path.points)):
            path.points[i, 0] = np.clip(path.points[i, 0], room_c[0] + x_lo, room_c[0] + x_hi)
            path.points[i, 1] = np.clip(path.points[i, 1], room_c[1] + y_lo, room_c[1] + y_hi)
        paths.add(path)
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
    """📌 Mid-lower instructional callout — kept clear of caption_bar / formula_panel."""
    return Text(text, font_size=BODY_FONT_SIZE, color=color).move_to(DOWN * 1.5)


#endregion


#region Beat1 – Mechanical Supply/Exhaust System
class Beat1_MechanicalVentilation(Scene):
    NARRATION = [
        ("intro",
         "The room is already saturated with heat. To actively remove that cooling load, we rely on a mechanical supply and exhaust system—a Zu-Abluftsystem.",
         "Der Raum ist wärmegesättigt — ein mechanisches Zu-/Abluftsystem muss die Last abführen."),
        ("flow",
         "Cool outdoor air is pushed in through a supply grille, while warm indoor air is drawn out through an exhaust grille at the same time.",
         "Kühle Zuluft kommt rein, warme Abluft wird gleichzeitig abgeführt."),
        ("question",
         "Watch the airflow cool the room. The engineering question that follows is precise: what volumetric flow rate do we need to neutralize this heat load?",
         "Welche Volumenstromrate braucht es, um diese Wärmelast zu neutralisieren?"),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Zu-/Abluftsystem", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        built = _build_room(center=DOWN * 0.1, width=7.4, height=2.9)
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
        ).move_to(room.get_top() + UP * 0.42)
        fans = VGroup(*[
            Circle(radius=0.11, color=P_WHITE, stroke_width=1.5).move_to(unit.get_center() + RIGHT * dx)
            for dx in (-0.45, 0.0, 0.45)
        ])
        unit_tag = Text("RLT-Anlage", font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(unit, UP, buff=0.08)
        rlt = VGroup(unit, fans)

        duct_sup = Line(unit.get_bottom() + LEFT * 0.4, supply.get_top(), color=P_CYAN, stroke_width=7)
        duct_exh = Line(unit.get_bottom() + RIGHT * 0.4, exhaust.get_top(), color=P_ORANGE, stroke_width=7)
        sys_lbl = Text("Zu-/Abluftsystem", font_size=BODY_FONT_SIZE, color=P_TEAL)
        sys_lbl.next_to(rlt, RIGHT, buff=0.3)

        zuluft = VGroup(
            Text("Zuluft", font_size=BODY_FONT_SIZE, color=P_CYAN),
            Text("kühl · aufbereitet", font_size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        zuluft.next_to(room, LEFT, buff=0.42).shift(UP * 1.05)

        abluft = VGroup(
            Text("Abluft", font_size=BODY_FONT_SIZE, color=P_ORANGE),
            Text("warm · abgeführt", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        abluft.next_to(room, RIGHT, buff=0.42).shift(UP * 1.05)

        supply_flow = VGroup(
            CurvedArrow(supply.get_bottom() + DOWN * 0.05, room_c + LEFT * 1.8 + DOWN * 0.15,
                        angle=0.35 * PI, color=P_CYAN, stroke_width=5),
            CurvedArrow(supply.get_bottom() + DOWN * 0.08 + RIGHT * 0.1, room_c + LEFT * 0.15,
                        angle=0.22 * PI, color=P_CYAN, stroke_width=5),
            CurvedArrow(supply.get_bottom() + DOWN * 0.05 + RIGHT * 0.18, room_c + RIGHT * 1.25 + UP * 0.18,
                        angle=0.08 * PI, color=P_CYAN, stroke_width=5),
        )
        exhaust_flow = VGroup(
            CurvedArrow(room_c + RIGHT * 0.8 + DOWN * 0.3, exhaust.get_bottom() + DOWN * 0.04 + LEFT * 0.1,
                        angle=-0.14 * PI, color=P_ORANGE, stroke_width=5),
            CurvedArrow(room_c + RIGHT * 1.75 + UP * 0.05, exhaust.get_bottom() + DOWN * 0.02,
                        angle=-0.24 * PI, color=P_ORANGE, stroke_width=5),
        )

        self.add(warm_wash)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(room), Create(floor), run_time=1.6)
        self.play(warm_wash.animate.set_fill(opacity=0.18), run_time=1.2)

        self.play(FadeIn(rlt), FadeIn(unit_tag), FadeIn(sys_lbl), run_time=1.4)
        self.play(
            Create(duct_sup), Create(duct_exh),
            FadeIn(supply), FadeIn(exhaust),
            FadeIn(zuluft), FadeIn(abluft),
            run_time=2.0,
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flow"))
        hold_for(self, self.NARRATION, "flow", used=6.2 + 0.35)

        self.play(
            LaggedStart(*[Create(a) for a in supply_flow], lag_ratio=0.18),
            warm_wash.animate.set_fill(P_CYAN, opacity=0.10),
            room.animate.set_stroke(color=P_CYAN),
            floor.animate.set_color(P_CYAN),
            run_time=2.8,
        )
        self.play(LaggedStart(*[Create(a) for a in exhaust_flow], lag_ratio=0.2), run_time=1.8)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "question"))
        hold_for(self, self.NARRATION, "question", used=4.6 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Thermodynamic Volume-Flow Equation
class Beat2_VolumeFlowEquation(Scene):
    NARRATION = [
        ("intro",
         "The convective cooling capacity of that airflow is defined by a fundamental thermodynamic product.",
         "Die konvektive Kühlleistung der Zuluft folgt einem thermodynamischen Produkt."),
        ("formula",
         "Q-dot V equals air density times specific heat capacity times the temperature difference between cool supply air and the warm room, times q v R—the required room airflow volume.",
         "Q-Punkt-V = ρ_a · c_p,a · Δθ · q_v,R."),
        ("rho",
         "Density is a material property of air — about 1.29 kilograms per cubic metre.",
         "ρ_a ist die Luftdichte — etwa 1,29 kg/m³."),
        ("cp",
         "Specific heat capacity is likewise a material property — about 1.0 kilojoule per kilogram-kelvin.",
         "c_p,a ist die spez. Wärmekapazität — etwa 1,0 kJ/(kg·K)."),
        ("dth",
         "Delta theta is the designed temperature lift we allow — here 25 minus 18 equals 7 kelvin.",
         "Δθ ist der Temperaturhub — hier 25 − 18 = 7 K."),
        ("qvr",
         "And q v R is the free design variable: how much air we must move every second to carry the heat away.",
         "q_v,R ist die Entwurfsgröße — wie viel Luft pro Sekunde bewegt werden muss."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Konvektive Kühlleistung der Zuluft", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        built = _build_room(center=LEFT * 2.6 + UP * 0.45, width=4.8, height=2.1)
        room, floor, room_c = built["room"], built["floor"], built["center"]
        supply = _vent_unit(room.get_top() + DOWN * 0.22 + LEFT * 1.25, color=P_CYAN, width=1.25)

        t_supply = VGroup(
            Text("θ_Zu", font_size=LABEL_FONT_SIZE, color=P_CYAN),
            Text("18 °C", font_size=BODY_FONT_SIZE, color=P_CYAN),
        ).arrange(DOWN, buff=0.06)
        t_supply = VGroup(
            SurroundingRectangle(t_supply, color=P_CYAN, corner_radius=0.1, buff=0.12, stroke_width=1.8),
            t_supply,
        ).next_to(room, DOWN, buff=0.15).shift(LEFT * 1.0)

        t_room = VGroup(
            Text("θ_Raum", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
            Text("25 °C", font_size=BODY_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.06)
        t_room = VGroup(
            SurroundingRectangle(t_room, color=P_ORANGE, corner_radius=0.1, buff=0.12, stroke_width=1.8),
            t_room,
        ).next_to(room, DOWN, buff=0.15).shift(RIGHT * 1.0)

        delta_card = VGroup(
            Text("Temperaturhub", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            Text("Δθ = 7 K", font_size=BODY_FONT_SIZE, color=P_BLUE),
        ).arrange(DOWN, buff=0.06)
        delta_card = VGroup(
            SurroundingRectangle(delta_card, color=P_BLUE, corner_radius=0.1, buff=0.12, stroke_width=1.8),
            delta_card,
        ).next_to(room, UP, buff=0.20).set_x(room_c[0])

        temp_arrow = Arrow(
            t_supply.get_right() + RIGHT * 0.08, t_room.get_left() + LEFT * 0.08,
            buff=0.04, stroke_width=4, max_tip_length_to_length_ratio=0.12, color=P_BLUE,
        )

        cool_paths = _supply_paths(
            supply.get_bottom(),
            room_c,
            n=14,
            seed=21,
            half_w=built["w"] / 2,
            half_h=built["h"] / 2,
            margin=0.25,
        )
        cool_dots = _air_particles(cool_paths, P_CYAN, seed=21, radius_range=(0.045, 0.07))

        eq, items = equation_row([
            ("qv", "Q̇_V", P_CYAN), (None, "=", P_WHITE),
            ("rho", "ρ_a", P_GREEN), (None, "·", P_WHITE),
            ("cp", "c_p,a", P_GREEN), (None, "·", P_WHITE),
            ("dth", "Δθ", P_BLUE), (None, "·", P_WHITE),
            ("qvr", "q_v,R", P_YELLOW),
            (None, "  [W]", P_TEAL),
        ])
        eq, eq_box = formula_panel(eq)

        card_bodies = [
            VGroup(
                Text("Luftdichte", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                Text("ρ_a = 1,29 kg/m³", font_size=BODY_FONT_SIZE, color=P_GREEN),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("spez. Wärmekapazität", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                Text("c_p,a = 1,0 kJ/(kg·K)", font_size=BODY_FONT_SIZE, color=P_GREEN),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("Temperaturdifferenz", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                Text("Δθ = 7 K", font_size=BODY_FONT_SIZE, color=P_BLUE),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("Volumenstrom", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                Text("q_v,R  [m³/s]", font_size=BODY_FONT_SIZE, color=P_YELLOW),
            ).arrange(DOWN, buff=0.06),
        ]
        # Same-size frames so the 2×2 grid lines up (a bit larger than the widest label).
        card_w = max(b.width for b in card_bodies) + 0.55
        card_h = max(b.height for b in card_bodies) + 0.42
        cards = VGroup()
        for body in card_bodies:
            frame = RoundedRectangle(
                width=card_w, height=card_h, corner_radius=0.1,
                color=P_TEAL, stroke_width=1.8,
            )
            body.move_to(frame.get_center())
            cards.add(VGroup(frame, body))
        cards.arrange_in_grid(rows=2, cols=2, buff=(0.28, 0.28))
        cards.move_to(RIGHT * 3.15 + UP * 0.55)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(room), Create(floor), FadeIn(supply), run_time=1.5)
        self.play(FadeIn(t_supply), FadeIn(t_room), run_time=1.0)
        self.add(cool_dots)
        self.play(
            AnimationGroup(*[
                MoveAlongPath(d, path, rate_func=linear)
                for d, path in zip(cool_dots, cool_paths)
            ], lag_ratio=0.05),
            FadeIn(temp_arrow), FadeIn(delta_card),
            run_time=3.5,
        )

        self.play(FadeIn(eq), Create(eq_box), run_time=1.2)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        hold_for(self, self.NARRATION, "formula", used=7.2 + 0.35)

        for key, card, color in (
            ("rho", cards[0], P_GREEN), ("cp", cards[1], P_GREEN),
            ("dth", cards[2], P_BLUE), ("qvr", cards[3], P_YELLOW),
        ):
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), FadeIn(card), run_time=0.7)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.7 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        air_blob = RoundedRectangle(
            width=1.4, height=0.9, corner_radius=0.2,
            color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN, fill_opacity=0.18,
        ).move_to(room_c)
        qvr_tok = symbol_token("q_v,R", color=P_YELLOW, font_size=FORMULA_FONT_SIZE)
        qvr_tok.move_to(air_blob.get_center())
        self.play(FadeIn(air_blob), run_time=0.5)
        self.play(ReplacementTransform(air_blob, qvr_tok), run_time=1.0)
        self.play(qvr_tok.animate.move_to(items["qvr"].get_center()), run_time=1.0)
        self.play(FadeOut(qvr_tok), FadeOut(caption), run_time=0.4)
        self.wait(0.5)
#endregion


#region Beat3 – Isolate Required Airflow q_v,R
class Beat3_IsolateAirflow(Scene):
    NARRATION = [
        ("intro",
         "Thermal equilibrium demands that cooling capacity equals the solar transmission load we calculated earlier.",
         "Im Gleichgewicht muss die Kühlleistung der solaren Kühllast gleichen."),
        ("substitute",
         "Because Q-dot V equals Q-dot S,tr, we replace the left side and rearrange: q v R moves alone to the left, and the load sits over density, heat capacity and Delta theta.",
         "Wegen Q-Punkt-V = Q-Punkt-S,tr stellen wir um: q_v,R links, die Last über ρ_a · c_p,a · Δθ."),
        ("qvr",
         "On the left stands the unknown we are solving for: the required volume flow, in cubic metres per second — the same q v R that sat on the right of the first equation.",
         "Links die gesuchte Größe q_v,R — dieselbe wie rechts in der ersten Formel."),
        ("qstr",
         "In the numerator sits Q-dot S,tr — it replaces Q-dot V because equilibrium made them equal.",
         "Im Zähler: Q-Punkt-S,tr ersetzt Q-Punkt-V — im Gleichgewicht sind sie gleich."),
        ("rho",
         "The denominator keeps the air properties from the first formula — density and specific heat capacity.",
         "Im Nenner bleiben die Luftkennwerte der ersten Formel: Dichte und Wärmekapazität."),
        ("dth",
         "And the same temperature lift, seven Kelvin, closes the denominator.",
         "Und derselbe Temperaturhub von sieben Kelvin schließt den Nenner ab."),
        ("result",
         "That flow rate is the sizing target for the ventilation system.",
         "Dieser Volumenstrom ist der Auslegungswert der Lüftung."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Gleichgewicht: Kühlleistung = Kühllast", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        beam = Line(LEFT * 2.8, RIGHT * 2.8, color=P_WHITE, stroke_width=4).move_to(UP * 1.15)
        pivot = Triangle(color=P_TEAL, fill_opacity=1).scale(0.18).rotate(PI).next_to(beam, DOWN, buff=0)
        left_pan = RoundedRectangle(
            width=2.2, height=0.9, corner_radius=0.1,
            color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN, fill_opacity=0.12,
        ).next_to(beam.get_left(), DOWN, buff=0.35).shift(RIGHT * 0.9)
        right_pan = RoundedRectangle(
            width=2.2, height=0.9, corner_radius=0.1,
            color=P_YELLOW, stroke_width=2.5, fill_color=P_YELLOW, fill_opacity=0.12,
        ).next_to(beam.get_right(), DOWN, buff=0.35).shift(LEFT * 0.9)
        left_txt = Text("Q̇_V\nKühlleistung", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to(left_pan)
        right_txt = Text("Q̇_S,tr\nKühllast", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(right_pan)
        eq_mark = Text("=", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to(beam.get_center() + DOWN * 0.55)
        balance = VGroup(beam, pivot, left_pan, right_pan, left_txt, right_txt, eq_mark)

        start_eq, start_items = equation_row([
            ("qv", "Q̇_V", P_CYAN), (None, "=", P_WHITE),
            ("rho", "ρ_a", P_GREEN), (None, "·", P_WHITE),
            ("cp", "c_p,a", P_GREEN), (None, "·", P_WHITE),
            ("dth", "Δθ", P_BLUE), (None, "·", P_WHITE),
            ("qvr", "q_v,R", P_YELLOW),
            (None, "  [W]", P_TEAL),
        ])
        start_eq, start_box = formula_panel(start_eq, color=P_CYAN)
        start_panel = VGroup(start_box, start_eq)

        final_eq, final_items = equation_row([
            ("qvr", "q_v,R", P_CYAN), (None, "=", P_WHITE),
            ("qstr", "Q̇_S,tr", P_YELLOW), (None, "/", P_WHITE),
            (None, "(", P_WHITE),
            ("rho", "ρ_a", P_GREEN), (None, "·", P_WHITE),
            ("cp", "c_p,a", P_GREEN), (None, "·", P_WHITE),
            ("dth", "Δθ", P_BLUE),
            (None, ")", P_WHITE),
            (None, "  [m³/s]", P_TEAL),
        ])

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(
            Create(beam), FadeIn(pivot),
            FadeIn(left_pan), FadeIn(right_pan),
            FadeIn(left_txt), FadeIn(right_txt),
            run_time=2.0,
        )
        self.play(FadeIn(eq_mark), run_time=0.6)
        self.play(FadeIn(start_eq), Create(start_box), run_time=1.2)
        self.play(Indicate(left_txt, color=P_CYAN), Indicate(right_txt, color=P_YELLOW), run_time=1.2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "substitute"))
        self.play(FadeOut(balance), run_time=0.8)
        self.play(start_panel.animate.move_to(UP * 1.05), run_time=0.8)

        bridge = VGroup(
            Text("Q̇_V = Q̇_S,tr", font_size=BODY_FONT_SIZE, color=P_YELLOW),
            Text("↓  umstellen nach q_v,R", font_size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(DOWN, buff=0.12)
        bridge.next_to(start_panel, DOWN, buff=0.28)

        final_eq, final_box = formula_panel(final_eq, color=P_CYAN)
        self.play(FadeIn(bridge), run_time=0.7)
        self.play(FadeIn(final_eq), Create(final_box), run_time=1.2)
        hold_for(self, self.NARRATION, "substitute", used=0.8 + 0.8 + 0.7 + 1.2 + 0.35)

        start_links = {
            "qvr": ("qvr",),
            "qstr": ("qv",),
            "rho": ("rho", "cp"),
            "dth": ("dth",),
        }
        for key, color in (("qvr", P_CYAN), ("qstr", P_YELLOW), ("rho", P_GREEN), ("dth", P_BLUE)):
            ring = highlight_param(final_items, key, color=color)
            self.play(
                Create(ring),
                *[Indicate(start_items[sk], color=color, scale_factor=1.12) for sk in start_links[key]],
                run_time=0.55,
            )
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.55 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "result"))
        hold_for(self, self.NARRATION, "result", used=0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Duct Cross-Section & Continuity
class Beat4_DuctCrossSection(Scene):
    NARRATION = [
        ("intro",
         "With the required air volume known, we size the physical duct using continuity.",
         "Mit bekanntem Volumenstrom dimensionieren wir den Kanal über die Kontinuität."),
        ("continuity",
         "Volumetric flow equals mean air velocity times cross-sectional area: q v R equals v m times A.",
         "Volumenstrom: q_v,R = v_m · A."),
        ("qvr",
         "q v R is the required volume flow we just sized — cubic metres of air per second.",
         "q_v,R ist der benötigte Volumenstrom — Luft in m³/s."),
        ("vm",
         "v m is the mean duct velocity. To limit noise, engineers often cap it around two point five meters per second.",
         "v_m ist die mittlere Kanalgeschwindigkeit — oft begrenzt auf etwa 2,5 m/s."),
        ("A_cont",
         "A is the free cross-sectional area of the duct that must carry that flow.",
         "A ist die freie Querschnittsfläche, die diesen Strom tragen muss."),
        ("area",
         "With velocity fixed, rearrange: A equals q v R over v m — that is the required duct area in square metres.",
         "Bei fester Geschwindigkeit: A = q_v,R / v_m — die nötige Kanalfläche in m²."),
        ("A",
         "A on the left is the duct area we must provide.",
         "A links ist die bereitzustellende Kanalfläche."),
        ("qvr_num",
         "In the numerator sits the required volume flow again — more air needs more area.",
         "Im Zähler steht wieder der Volumenstrom — mehr Luft braucht mehr Fläche."),
        ("vm_den",
         "In the denominator sits velocity — a lower speed limit forces a larger duct.",
         "Im Nenner steht die Geschwindigkeit — niedrigere v_m erzwingt einen größeren Kanal."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Vom Volumenstrom zum Kanalquerschnitt", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        duct_c = LEFT * 2.8 + UP * 0.35
        outer = Circle(radius=1.25, color=P_TEAL, stroke_width=4).move_to(duct_c)
        wall = Annulus(inner_radius=1.05, outer_radius=1.25, color=P_TEAL, fill_opacity=0.35, stroke_width=0).move_to(duct_c)
        area_fill = Circle(radius=1.05, color=P_BLUE, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.30).move_to(duct_c)
        area_lbl = Text("A", font_size=FORMULA_FONT_SIZE, color=P_BLUE).move_to(duct_c)

        pipe_top = Line(duct_c + RIGHT * 1.25 + UP * 0.85, duct_c + RIGHT * 3.3 + UP * 0.45, color=P_TEAL, stroke_width=3)
        pipe_bot = Line(duct_c + RIGHT * 1.25 + DOWN * 0.85, duct_c + RIGHT * 3.3 + DOWN * 0.45, color=P_TEAL, stroke_width=3)
        pipe_end = Ellipse(width=0.5, height=0.9, color=P_TEAL, stroke_width=3).move_to(duct_c + RIGHT * 3.3)
        pipe = VGroup(pipe_top, pipe_bot, pipe_end)

        rng = np.random.default_rng(42)
        flow_paths = VGroup()
        for i in range(12):
            y = float(rng.uniform(-0.6, 0.6))
            start = duct_c + LEFT * 0.9 + UP * y * 0.85
            end = duct_c + RIGHT * 3.1 + UP * y * 0.35
            mid = duct_c + RIGHT * 1.1 + UP * y * 0.55
            flow_paths.add(_smooth_path([start, mid, end]))
        flow_dots = _air_particles(flow_paths, P_CYAN, radius_range=(0.05, 0.08), seed=42)

        vm_tag = Text("v_m ≈ 2,5 m/s  (lärmarm)", font_size=BODY_FONT_SIZE, color=P_CYAN)
        vm_tag.next_to(duct_c, DOWN, buff=1.35)

        cont, cont_items = equation_row([
            ("qvr", "q_v,R", P_YELLOW), (None, "=", P_WHITE),
            ("vm", "v_m", P_CYAN), (None, "·", P_WHITE),
            ("A", "A", P_BLUE),
            (None, "  [m³/s]", P_TEAL),
        ])
        cont.move_to(RIGHT * 3.2 + UP * 1.35)

        area_eq, area_items = equation_row([
            ("A", "A", P_BLUE), (None, "=", P_WHITE),
            ("qvr", "q_v,R", P_YELLOW), (None, "/", P_WHITE),
            ("vm", "v_m", P_CYAN),
            (None, "  [m²]", P_TEAL),
        ])

        tip_fast = Text("kleiner A → höhere v_m", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        tip_slow = Text("größerer A → niedrigere v_m", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        tip_fast.move_to(RIGHT * 3.2 + DOWN * 0.15)
        tip_slow.move_to(RIGHT * 3.2 + DOWN * 0.15)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(outer), FadeIn(wall), Create(pipe), run_time=1.8)
        self.play(FadeIn(area_fill), FadeIn(area_lbl), run_time=1.0)
        self.add(flow_dots)
        self.play(
            AnimationGroup(*[
                MoveAlongPath(d, path, rate_func=linear)
                for d, path in zip(flow_dots, flow_paths)
            ], lag_ratio=0.06),
            FadeIn(vm_tag),
            run_time=3.5,
        )

        self.play(FadeIn(cont), run_time=1.1)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "continuity"))
        hold_for(self, self.NARRATION, "continuity", used=1.1 + 0.35)

        for key, item_key, color in (
            ("qvr", "qvr", P_YELLOW),
            ("vm", "vm", P_CYAN),
            ("A_cont", "A", P_BLUE),
        ):
            ring = highlight_param(cont_items, item_key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.45 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(cont), run_time=0.4)
        area_eq, area_box = formula_panel(area_eq)
        self.play(FadeIn(area_eq), Create(area_box), run_time=1.2)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "area"))
        hold_for(self, self.NARRATION, "area", used=1.2 + 0.35)

        self.play(FadeIn(tip_fast), run_time=0.5)
        self.play(
            area_fill.animate.scale(0.62), area_lbl.animate.scale(0.62),
            flow_dots.animate.set_color(P_ORANGE), run_time=1.8,
        )
        self.play(FadeOut(tip_fast), FadeIn(tip_slow), run_time=0.5)
        self.play(
            area_fill.animate.scale(1 / 0.62 * 1.15), area_lbl.animate.scale(1 / 0.62 * 1.15),
            flow_dots.animate.set_color(P_CYAN), run_time=1.8,
        )
        self.play(area_fill.animate.scale(1 / 1.15), area_lbl.animate.scale(1 / 1.15), FadeOut(tip_slow), run_time=0.9)

        for key, item_key, color in (
            ("A", "A", P_BLUE),
            ("qvr_num", "qvr", P_YELLOW),
            ("vm_den", "vm", P_CYAN),
        ):
            ring = highlight_param(area_items, item_key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.45 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Calculate Duct Radius
class Beat5_CalculateRadius(Scene):
    NARRATION = [
        ("intro",
         "Now we close the sizing chain: every quantity we derived feeds the next — from cooling load to volume flow, to duct area, to radius.",
         "Jetzt die Auslegungskette: Kühllast → Volumenstrom → Fläche → Radius."),
        ("flow",
         "First, volume flow q v R equals the solar cooling load Q-dot S,tr divided by density, heat capacity and Delta theta.",
         "q_v,R = Q-Punkt-S,tr / (ρ_a · c_p,a · Δθ)."),
        ("qstr",
         "Q-dot S,tr is the heat we must remove — it sits in the numerator.",
         "Q-Punkt-S,tr ist die abzuführende Wärme — sie steht im Zähler."),
        ("qvr",
         "That fixes q v R — the airflow the duct must carry every second.",
         "Daraus folgt q_v,R — der Luftstrom, den der Kanal tragen muss."),
        ("area",
         "Next, continuity: area A equals that volume flow divided by the mean velocity v m.",
         "Als Nächstes Kontinuität: A = q_v,R / v_m."),
        ("qvr_a",
         "The same q v R now sits in the numerator of the area formula.",
         "Dasselbe q_v,R steht jetzt im Zähler der Flächenformel."),
        ("vm",
         "v m is not calculated from the heat load — it is a design limit we choose. To keep the duct quiet, engineers usually cap mean velocity around two point five meters per second.",
         "v_m kommt nicht aus der Last — es ist ein Entwurfslimit. Für leise Kanäle oft etwa 2,5 m/s."),
        ("A",
         "With v m fixed, A follows — the free cross-section the duct needs.",
         "Mit festem v_m folgt A — die nötige Querschnittsfläche."),
        ("radius",
         "Finally the round duct: radius r equals the square root of A over pi.",
         "Zuletzt der runde Kanal: r = √(A / π)."),
        ("A_r",
         "That same A feeds the root — geometry turns area into a length.",
         "Dieselbe Fläche A steckt unter der Wurzel — Geometrie macht Länge daraus."),
        ("r",
         "r is the duct radius we install. Load to flow to area to radius — the system is fully sized.",
         "r ist der Kanalradius. Last → Strom → Fläche → Radius — die Anlage ist ausgelegt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Auslegungskette: Last → Strom → Fläche → Radius", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        duct_c = LEFT * 4.0 + UP * 0.35
        circle = Circle(radius=1.05, color=P_TEAL, stroke_width=4).move_to(duct_c)
        fill = Circle(radius=1.05, color=P_BLUE, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.2).move_to(duct_c)
        center_dot = Dot(duct_c, radius=0.06, color=P_YELLOW)
        radius_line = Line(duct_c, duct_c + RIGHT * 1.05, color=P_YELLOW, stroke_width=4)
        r_lbl = Text("r", font_size=FORMULA_FONT_SIZE, color=P_YELLOW)
        r_lbl.next_to(radius_line, UP, buff=0.08)

        tokens = VGroup(
            Text("Q̇_S,tr", font_size=BODY_FONT_SIZE, color=P_YELLOW),
            Text("→", font_size=BODY_FONT_SIZE, color=P_WHITE),
            Text("q_v,R", font_size=BODY_FONT_SIZE, color=P_CYAN),
            Text("→", font_size=BODY_FONT_SIZE, color=P_WHITE),
            Text("A", font_size=BODY_FONT_SIZE, color=P_BLUE),
            Text("→", font_size=BODY_FONT_SIZE, color=P_WHITE),
            Text("r", font_size=BODY_FONT_SIZE, color=P_YELLOW),
        ).arrange(RIGHT, buff=0.18)
        tokens.move_to(RIGHT * 2.2 + UP * 1.85)

        flow_eq, flow_items = equation_row([
            ("qvr", "q_v,R", P_CYAN), (None, "=", P_WHITE),
            ("qstr", "Q̇_S,tr", P_YELLOW), (None, "/", P_WHITE),
            (None, "(", P_WHITE),
            ("rho", "ρ_a", P_GREEN), (None, "·", P_WHITE),
            ("cp", "c_p,a", P_GREEN), (None, "·", P_WHITE),
            ("dth", "Δθ", P_BLUE),
            (None, ")", P_WHITE),
        ], font_size=FORMULA_FONT_SIZE, buff=0.12)

        area_eq, area_items = equation_row([
            ("A", "A", P_BLUE), (None, "=", P_WHITE),
            ("qvr", "q_v,R", P_CYAN), (None, "/", P_WHITE),
            ("vm", "v_m", P_TEAL),
        ], font_size=FORMULA_FONT_SIZE, buff=0.12)

        rad_eq, rad_items = equation_row([
            ("r", "r", P_YELLOW), (None, "=", P_WHITE),
            (None, "√", P_WHITE), (None, "(", P_WHITE),
            ("A", "A", P_BLUE), (None, "/", P_WHITE),
            ("pi", "π", P_TEAL), (None, ")", P_WHITE),
        ], font_size=FORMULA_FONT_SIZE, buff=0.12)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(circle), FadeIn(fill), run_time=1.2)
        self.play(FadeIn(center_dot), Create(radius_line), FadeIn(r_lbl), run_time=0.9)
        self.play(FadeIn(tokens), run_time=0.9)

        flow_eq, flow_box = formula_panel(flow_eq, color=P_CYAN)
        flow_panel = VGroup(flow_box, flow_eq)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flow"))
        self.play(FadeIn(flow_eq), Create(flow_box), run_time=1.2)
        hold_for(self, self.NARRATION, "flow", used=1.2 + 0.9 + 0.9 + 0.35)

        for key, item_key, color, tok_idx in (
            ("qstr", "qstr", P_YELLOW, 0),
            ("qvr", "qvr", P_CYAN, 2),
        ):
            ring = highlight_param(flow_items, item_key, color=color)
            self.play(Create(ring), Indicate(tokens[tok_idx], color=color, scale_factor=1.15), run_time=0.55)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.55 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(flow_panel.animate.scale(0.88).move_to(RIGHT * 2.0 + UP * 1.20), run_time=0.7)

        area_eq, area_box = formula_panel(area_eq, color=P_BLUE)
        area_panel = VGroup(area_box, area_eq)
        vm_note = VGroup(
            Text("Entwurfslimit (Lärmschutz)", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            Text("v_m ≈ 2,5 m/s", font_size=BODY_FONT_SIZE, color=P_CYAN),
        ).arrange(DOWN, buff=0.06)
        vm_note = VGroup(
            SurroundingRectangle(vm_note, color=P_TEAL, corner_radius=0.1, buff=0.12, stroke_width=1.8),
            vm_note,
        )
        vm_note.next_to(duct_c, DOWN, buff=1.25).set_x(duct_c[0])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "area"))
        self.play(FadeIn(area_eq), Create(area_box), run_time=1.2)
        hold_for(self, self.NARRATION, "area", used=0.7 + 1.2 + 0.35)

        for key, item_key, color, tok_idx in (
            ("qvr_a", "qvr", P_CYAN, 2),
            ("vm", "vm", P_TEAL, None),
            ("A", "A", P_BLUE, 4),
        ):
            ring = highlight_param(area_items, item_key, color=color)
            extras = [Create(ring)]
            if tok_idx is not None:
                extras.append(Indicate(tokens[tok_idx], color=color, scale_factor=1.15))
            if key == "qvr_a":
                extras.append(Indicate(flow_items["qvr"], color=P_CYAN, scale_factor=1.12))
            elif key == "vm":
                extras.append(FadeIn(vm_note, shift=UP * 0.1))
            self.play(*extras, run_time=0.55)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.55 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(area_panel.animate.scale(0.88).move_to(RIGHT * 2.0 + UP * 0.40), run_time=0.7)

        rad_eq, rad_box = formula_panel(rad_eq, color=P_YELLOW)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "radius"))
        self.play(FadeIn(rad_eq), Create(rad_box), run_time=1.2)
        hold_for(self, self.NARRATION, "radius", used=0.7 + 1.2 + 0.35)

        for key, item_key, color, tok_idx in (
            ("A_r", "A", P_BLUE, 4),
            ("r", "r", P_YELLOW, 6),
        ):
            ring = highlight_param(rad_items, item_key, color=color)
            extras = [Create(ring), Indicate(tokens[tok_idx], color=color, scale_factor=1.15)]
            if key == "A_r":
                extras.append(Indicate(area_items["A"], color=P_BLUE, scale_factor=1.12))
                extras.append(Indicate(fill, color=P_BLUE))
            else:
                extras.append(Indicate(r_lbl, color=P_YELLOW, scale_factor=1.2))
                extras.append(Indicate(radius_line, color=P_YELLOW))
            self.play(*extras, run_time=0.55)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.55 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        self.play(
            Indicate(tokens[0], color=P_YELLOW),
            Indicate(tokens[2], color=P_CYAN),
            Indicate(tokens[4], color=P_BLUE),
            Indicate(tokens[6], color=P_YELLOW),
            run_time=1.2,
        )
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
