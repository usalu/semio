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
    math_label, math_row, math_panel,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    room_section, smooth_path, flow_guides, flow_animation, ripples,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Systemauslegung"

# Mid-screen anchor for rooms / ducts (clear of title + formula/caption).
CONTENT_CENTER = UP * 0.1


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

def _build_room(center=CONTENT_CENTER, width=7.4, height=3.55):
    """🏠 Physical Fundamentals room section — slabs, glazed left wall, solid right wall."""
    r = room_section(np.array(center, dtype=float), w=width, h=height)
    r["room"] = r["group"]
    return r


def _vent_unit(pos, color=P_TEAL, width=1.55, height=0.3):
    """🔧 Ceiling grille seen in section — a slotted strip hanging from the slab."""
    body = Rectangle(width=width, height=height, color=color, stroke_width=1.8,
                     fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to(pos)
    slots = VGroup(*[
        Line(body.get_bottom() + RIGHT * dx + UP * 0.05, body.get_top() + RIGHT * dx + DOWN * 0.05,
             color=color, stroke_width=1.3)
        for dx in np.linspace(-width / 2 + 0.16, width / 2 - 0.16, 6)
    ])
    return VGroup(body, slots)


def _fan(center, r: float = 0.17, color=P_WHITE):
    """🌀 Fan ring with three blades — the blades turn while air moves."""
    center = np.array(center, dtype=float)
    ring = Circle(radius=r, color=color, stroke_width=1.5).move_to(center)
    blades = VGroup(*[
        Line(center, center + 0.85 * r * np.array([np.cos(a), np.sin(a), 0.0]), color=color, stroke_width=2.0)
        for a in np.linspace(0, TAU, 3, endpoint=False)
    ])
    return {"group": VGroup(ring, blades), "blades": blades, "center": center}


def _spin(fans, rt: float, period: float = 0.9):
    """🔄 Turn every fan's blades for ``rt`` seconds."""
    return [Rotate(f["blades"], angle=-TAU * rt / period, about_point=f["center"], rate_func=linear) for f in fans]


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
        path = smooth_path([start, mid, end])
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
        paths.add(smooth_path([start, mid, end]))
    return paths


#endregion


#region Beat1 – Mechanical Supply/Exhaust System
class Beat1_MechanicalVentilation(Scene):
    NARRATION = [
        ("intro",
         "The room is already saturated with heat. To actively remove that cooling load, we rely on a mechanical supply and exhaust system—a Zu-Abluftsystem.",
         "Der Raum ist wärmegesättigt — ein mechanisches Zu- und Abluftsystem muss die Last abführen."),
        ("flow",
         "Cooled, treated supply air from the air-handling unit is pushed in through a supply grille, while warm indoor air is drawn out at the same time.",
         "Gekühlte, aufbereitete Zuluft aus der RLT-Anlage strömt herein, warme Abluft wird gleichzeitig abgeführt."),
        ("question",
         "Watch the airflow cool the room. The engineering question that follows is precise: what volumetric flow rate do we need to neutralize this heat load?",
         "Welchen Volumenstrom braucht es, um diese Wärmelast auszugleichen?"),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Zu-/Abluftsystem", title)
        din = _din_ref("DIN EN 16798-3")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        built = _build_room(center=np.array([0.3, -0.45, 0.0]), width=6.6, height=2.2)
        y_f, y_c = built["y_f"], built["y_c"]
        air = built["air"]

        sup_x, exh_x = -1.6, 2.4
        supply = _vent_unit(np.array([sup_x, y_c - 0.15, 0.0]), color=P_CYAN, width=1.1)
        exhaust = _vent_unit(np.array([exh_x, y_c - 0.15, 0.0]), color=P_ORANGE, width=1.1)

        unit = RoundedRectangle(width=2.4, height=0.6, corner_radius=0.08, color=P_WHITE, stroke_width=1.8,
                                fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to(np.array([0.3, 1.5, 0.0]))
        duct_y = unit.get_center()[1]
        fans = [_fan(unit.get_center() + LEFT * 0.75), _fan(unit.get_center() + RIGHT * 0.75)]
        coil_c = unit.get_center()
        coil = VMobject(color=P_BLUE, stroke_width=1.8).set_points_as_corners(
            [coil_c + np.array([-0.3 + i * 0.1, 0.1 if i % 2 else -0.1, 0.0]) for i in range(7)])
        unit_tag = Text("RLT-Anlage", font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(unit, UP, buff=0.1)
        rlt = VGroup(unit, coil, *[f["group"] for f in fans])

        def _duct(points, color):
            return VMobject(color=color, stroke_width=11, stroke_opacity=0.3).set_points_as_corners(
                [np.array([x, y, 0.0]) for x, y in points])

        ux_l, ux_r = unit.get_left()[0], unit.get_right()[0]
        duct_sup = _duct([(ux_l, duct_y), (sup_x, duct_y), (sup_x, y_c)], P_CYAN)
        duct_exh = _duct([(exh_x, y_c), (exh_x, duct_y), (ux_r, duct_y)], P_ORANGE)
        sys_lbl = Text("Zu-/Abluftsystem", font_size=BODY_FONT_SIZE, color=P_TEAL)
        sys_lbl.next_to(unit, RIGHT, buff=1.4).shift(UP * 0.1)

        zuluft = VGroup(
            Text("Zuluft", font_size=BODY_FONT_SIZE, color=P_CYAN),
            Text("kühl · aufbereitet", font_size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        zuluft.next_to(built["room"], LEFT, buff=0.35).set_y(0.25)

        abluft = VGroup(
            Text("Abluft", font_size=BODY_FONT_SIZE, color=P_ORANGE),
            Text("warm · abgeführt", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.05, aligned_edge=LEFT)
        abluft.next_to(built["room"], RIGHT, buff=0.35).set_y(0.25)

        def _p(*pts):
            return smooth_path([np.array([x, y, 0.0]) for x, y in pts])

        supply_paths = [
            _p((ux_l, duct_y + dy), (sup_x + dy, duct_y + dy), (sup_x + dy, y_c - 0.35), (sup_x + 0.3 + sx * 0.5, y_c - 0.95),
               (sup_x + 1.4 + sx, y_f + 0.55), (sup_x + 2.6 + sx, y_f + 0.3))
            for dy, sx in ((-0.05, -0.6), (0.0, 0.2), (0.05, 1.0))
        ]
        exhaust_paths = [
            _p((exh_x - 1.9 + sx, y_f + 0.3), (exh_x - 0.9 + sx * 0.5, y_f + 0.65), (exh_x + dy, y_c - 0.7),
               (exh_x + dy, duct_y - dy), (ux_r, duct_y - dy))
            for dy, sx in ((-0.05, -0.5), (0.05, 0.3))
        ]
        guides = VGroup(flow_guides(supply_paths[1:2], P_CYAN, opacity=0.22),
                        flow_guides(exhaust_paths[:1], P_ORANGE, opacity=0.22))
        warm_spots = [np.array([x, y_f + 0.02, 0.0]) for x in (-2.3, -0.7, 0.9, 2.5)]

        def heat(rt, r_max=0.55, color=P_RED):
            return [ripples(warm_spots, r_max=r_max, color=color, cycles=rt / 1.4)]

        def air_flow(rt, r_max=0.55, color=P_RED):
            cyc = rt / 2.4
            return [flow_animation([(supply_paths, P_BLUE, P_CYAN), (exhaust_paths, P_RED, P_ORANGE)],
                                   waves=4, cycles=cyc),
                    *_spin(fans, rt), *heat(rt, r_max, color)]

        self.play(Create(built["group"]), run_time=1.6)
        self.play(air.animate.set_fill(P_RED, opacity=0.18), *heat(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.6 + 1.2,
                 during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flow"))
        self.play(FadeIn(rlt), FadeIn(unit_tag), FadeIn(sys_lbl), *heat(1.2), run_time=1.2)
        self.play(
            Create(duct_sup), Create(duct_exh),
            FadeIn(supply), FadeIn(exhaust),
            FadeIn(zuluft), FadeIn(abluft), *heat(1.4),
            run_time=1.4,
        )
        self.play(Create(guides), *air_flow(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "flow", during=air_flow)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "question"))
        self.play(air.animate.set_fill(P_CYAN, opacity=0.12), *air_flow(2.8, 0.35, P_ORANGE), run_time=2.8)
        hold_for(self, self.NARRATION, "question", during=lambda rt: air_flow(rt, 0.3, P_ORANGE))
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
         "Q Punkt V ist gleich Rho a mal c p a mal Delta Theta mal q v R."),
        ("rho",
         "Density is a material property of air — about 1.2 kilograms per cubic metre at room temperature.",
         "Rho a ist die Luftdichte — etwa eins Komma zwei Kilogramm pro Kubikmeter bei Raumtemperatur."),
        ("cp",
         "Specific heat capacity is likewise a material property — about 1.0 kilojoule per kilogram-kelvin.",
         "c p a ist die spezifische Wärmekapazität — etwa eins Komma null Kilojoule pro Kilogramm und Kelvin."),
        ("dth",
         "Delta theta is the designed temperature lift between room air and cooled supply air — here 25 minus 18 equals 7 kelvin.",
         "Delta Theta ist der Temperaturhub zwischen Raumluft und gekühlter Zuluft — hier fünfundzwanzig minus achtzehn, also sieben Kelvin."),
        ("qvr",
         "And q v R is the free design variable: how much air we must move every second to carry the heat away.",
         "q v R ist die Entwurfsgröße — wie viel Luft pro Sekunde bewegt werden muss."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Konvektive Kühlleistung der Zuluft", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        built = _build_room(center=LEFT * 2.6 + UP * 0.5, width=4.6, height=1.35)
        room, room_c = built["room"], built["center"]
        supply = _vent_unit(np.array([room_c[0] - 1.2, built["y_c"] - 0.13, 0.0]), color=P_CYAN, width=1.1, height=0.26)

        t_supply = VGroup(
            math_label(r"θ_{Zu}", size=LABEL_FONT_SIZE, color=P_CYAN),
            Text("18 °C", font_size=BODY_FONT_SIZE, color=P_CYAN),
        ).arrange(DOWN, buff=0.06)
        t_supply = VGroup(
            SurroundingRectangle(t_supply, color=P_CYAN, corner_radius=0.1, buff=0.12, stroke_width=1.8),
            t_supply,
        ).next_to(room, DOWN, buff=0.15).shift(LEFT * 1.0)

        t_room = VGroup(
            math_label(r"θ_{Raum}", size=LABEL_FONT_SIZE, color=P_ORANGE),
            Text("25 °C", font_size=BODY_FONT_SIZE, color=P_ORANGE),
        ).arrange(DOWN, buff=0.06)
        t_room = VGroup(
            SurroundingRectangle(t_room, color=P_ORANGE, corner_radius=0.1, buff=0.12, stroke_width=1.8),
            t_room,
        ).next_to(room, DOWN, buff=0.15).shift(RIGHT * 1.0)

        delta_card = VGroup(
            Text("Temperaturhub", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            math_label(r"Δθ = 7\,\mathrm{K}", size=BODY_FONT_SIZE, color=P_BLUE),
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

        def cool_air(rt):
            return [flow_animation([(cool_paths, P_CYAN, P_BLUE)], waves=3, radius=0.055, cycles=rt / 2.6)]

        eq, eq_box, items = math_panel([
            ("qv", r"\dot{Q}_{V}", P_CYAN), (None, "=", P_WHITE),
            ("rho", r"\rho_{a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("cp", r"c_{p,a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("dth", r"Δθ", P_BLUE), (None, r"\cdot", P_WHITE),
            ("qvr", r"q_{v,R}", P_YELLOW),
            (None, r"\;[\mathrm{W}]", P_TEAL),
        ])

        card_bodies = [
            VGroup(
                Text("Luftdichte", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                math_label(r"\rho_{a} = 1{,}2\,\mathrm{kg/m^{3}}", size=BODY_FONT_SIZE, color=P_GREEN),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("spez. Wärmekapazität", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                math_label(r"c_{p,a} = 1{,}0\,\mathrm{kJ/(kg\,K)}", size=BODY_FONT_SIZE, color=P_GREEN),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("Temperaturdifferenz", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                math_label(r"Δθ = 7\,\mathrm{K}", size=BODY_FONT_SIZE, color=P_BLUE),
            ).arrange(DOWN, buff=0.06),
            VGroup(
                Text("Volumenstrom", font_size=LABEL_FONT_SIZE, color=P_TEAL),
                math_label(r"q_{v,R}\;[\mathrm{m^{3}/s}]", size=BODY_FONT_SIZE, color=P_YELLOW),
            ).arrange(DOWN, buff=0.06),
        ]
        # Same-size frames so the 2×2 grid lines up. Tight padding keeps the grid
        # narrow enough to clear the room on the left without shrinking the text.
        card_w = max(b.width for b in card_bodies) + 0.32
        card_h = max(b.height for b in card_bodies) + 0.28
        cards = VGroup()
        for body in card_bodies:
            frame = RoundedRectangle(
                width=card_w, height=card_h, corner_radius=0.1,
                color=P_TEAL, stroke_width=1.8,
            )
            body.move_to(frame.get_center())
            cards.add(VGroup(frame, body))
        cards.arrange_in_grid(rows=2, cols=2, buff=(0.20, 0.20))
        # Right of the room, vertically centred on it (room right edge ≈ -0.2).
        cards.move_to(np.array([3.4, room_c[1], 0.0]))

        self.play(Create(room), FadeIn(supply), run_time=1.5)
        self.play(FadeIn(t_supply), FadeIn(t_room), *cool_air(1.0), run_time=1.0)
        self.play(FadeIn(temp_arrow), FadeIn(delta_card), *cool_air(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 1.0 + 1.6, during=cool_air)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq), Create(eq_box), *cool_air(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", during=cool_air)

        for key, card, color in (
            ("rho", cards[0], P_GREEN), ("cp", cards[1], P_GREEN),
            ("dth", cards[2], P_BLUE), ("qvr", cards[3], P_YELLOW),
        ):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), FadeIn(card), *cool_air(0.7), run_time=0.7)
            hold_for(self, self.NARRATION, key, during=cool_air)
            self.play(FadeOut(ring), *cool_air(0.25), run_time=0.25)

        air_blob = RoundedRectangle(
            width=1.4, height=0.9, corner_radius=0.2,
            color=P_CYAN, stroke_width=2.5, fill_color=P_CYAN, fill_opacity=0.18,
        ).move_to(room_c)
        self.play(FadeIn(air_blob), run_time=0.5)
        self.play(FadeOut(items["qvr"]), run_time=0.3)
        self.remove(items["qvr"])
        self.play(ReplacementTransform(air_blob, items["qvr"].set_opacity(1.0)), run_time=1.7)
        self.play(FadeOut(caption), run_time=0.4)
        self.wait(0.5)
#endregion


#region Beat3 – Isolate Required Airflow q_v,R
class Beat3_IsolateAirflow(Scene):
    NARRATION = [
        ("intro",
         "We size the supply airflow so its cooling capacity matches the solar cooling load through the glazing from the last part.",
         "Wir legen den Zuluftstrom so aus, dass seine Kühlleistung der solaren Kühllast durch die Verglasung entspricht."),
        ("substitute",
         "Because Q-dot V equals Q-dot S,tr, we replace the left side and rearrange: q v R moves alone to the left, and the load sits over density, heat capacity and Delta theta.",
         "Weil Q Punkt V gleich Q Punkt S t r ist, stellen wir um: q v R nach links, die Last geteilt durch Rho a mal c p a mal Delta Theta."),
        ("qvr",
         "On the left stands the unknown we are solving for: the required volume flow, in cubic metres per second — the same q v R that sat on the right of the first equation.",
         "Links steht die gesuchte Größe q v R — dieselbe wie rechts in der ersten Formel."),
        ("qstr",
         "In the numerator sits Q-dot S,tr — it replaces Q-dot V because equilibrium made them equal.",
         "Im Zähler ersetzt Q Punkt S t r das Q Punkt V — im Gleichgewicht sind sie gleich."),
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
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

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
        left_txt = VGroup(
            math_label(r"\dot{Q}_{V}", size=BODY_FONT_SIZE, color=P_CYAN),
            Text("Kühlleistung", font_size=LABEL_FONT_SIZE, color=P_CYAN),
        ).arrange(DOWN, buff=0.08).move_to(left_pan)
        right_txt = VGroup(
            math_label(r"\dot{Q}_{S,tr}", size=BODY_FONT_SIZE, color=P_YELLOW),
            Text("Kühllast", font_size=LABEL_FONT_SIZE, color=P_YELLOW),
        ).arrange(DOWN, buff=0.08).move_to(right_pan)
        eq_mark = Text("=", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to(beam.get_center() + DOWN * 0.55)
        balance = VGroup(beam, pivot, left_pan, right_pan, left_txt, right_txt, eq_mark)

        start_eq, start_box, start_items = math_panel([
            ("qv", r"\dot{Q}_{V}", P_CYAN), (None, "=", P_WHITE),
            ("rho", r"\rho_{a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("cp", r"c_{p,a}", P_GREEN), (None, r"\cdot", P_WHITE),
            ("dth", r"Δθ", P_BLUE), (None, r"\cdot", P_WHITE),
            ("qvr", r"q_{v,R}", P_YELLOW),
            (None, r"\;[\mathrm{W}]", P_TEAL),
        ], color=P_CYAN)
        start_panel = VGroup(start_box, start_eq)

        final_parts = [
            ("qvr", r"q_{v,R}", P_CYAN), (None, "=", P_WHITE),
            ("frac",
             rf"\frac{{\textcolor{{{P_YELLOW}}}{{\dot{{Q}}_{{S,tr}}}}}}"
             rf"{{\textcolor{{{P_GREEN}}}{{\rho_{{a}} \cdot c_{{p,a}}}} \cdot \textcolor{{{P_BLUE}}}{{Δθ}}}}",
             P_WHITE),
            (None, r"\;[\mathrm{m^{3}/s}]", P_TEAL),
        ]

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
            math_label(r"\dot{Q}_{V} = \dot{Q}_{S,tr}", size=BODY_FONT_SIZE, color=P_YELLOW),
            math_label(r"↓\;\text{umstellen nach}\;q_{v,R}", size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(DOWN, buff=0.12)
        bridge.next_to(start_panel, DOWN, buff=0.28)

        final_eq, final_box, final_items = math_panel(final_parts, color=P_CYAN)
        # The one typeset fraction box: [numerator, bar, denominator, marker].
        final_frac = final_items["frac"][0]
        final_items["qstr"] = final_frac[0]
        final_items["rho"] = final_frac[2]
        final_items["dth"] = final_frac[2]
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
         "Der Volumenstrom q v R ist gleich der mittleren Geschwindigkeit v m mal der Fläche A."),
        ("qvr",
         "q v R is the required volume flow we just sized — cubic metres of air per second.",
         "q v R ist der benötigte Volumenstrom — Luft in Kubikmeter pro Sekunde."),
        ("vm",
         "v m is the mean duct velocity. To limit noise, engineers often cap it around two point five meters per second.",
         "v m ist die mittlere Kanalgeschwindigkeit — oft begrenzt auf etwa zwei Komma fünf Meter pro Sekunde."),
        ("A_cont",
         "A is the free cross-sectional area of the duct that must carry that flow.",
         "A ist die freie Querschnittsfläche, die diesen Strom tragen muss."),
        ("area",
         "With velocity fixed, rearrange: A equals q v R over v m — that is the required duct area in square metres.",
         "Bei fester Geschwindigkeit ist A gleich q v R geteilt durch v m — die nötige Kanalfläche in Quadratmetern."),
        ("A",
         "A on the left is the duct area we must provide.",
         "A auf der linken Seite ist die bereitzustellende Kanalfläche."),
        ("qvr_num",
         "In the numerator sits the required volume flow again — more air needs more area.",
         "Im Zähler steht wieder der Volumenstrom — mehr Luft braucht mehr Fläche."),
        ("vm_den",
         "In the denominator sits velocity — a lower speed limit forces a larger duct.",
         "Im Nenner steht die Geschwindigkeit — ein niedrigeres v m erzwingt einen größeren Kanal."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Vom Volumenstrom zum Kanalquerschnitt", title)
        din = _din_ref("VDI 2087")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

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
        for y in (-0.72, -0.56, -0.4, 0.4, 0.56, 0.72):
            x0 = -np.sqrt(1.0 - y * y) * float(rng.uniform(0.35, 0.85))
            start = duct_c + RIGHT * x0 + UP * y
            mid = duct_c + RIGHT * 1.3 + UP * y * 0.62
            end = duct_c + RIGHT * 3.1 + UP * y * 0.4
            flow_paths.add(smooth_path([start, mid, end]))

        def duct_air(rt, speed=1.0, color=P_CYAN):
            return [flow_animation([(flow_paths, color)], waves=3, radius=0.06, cycles=speed * rt / 2.2)]

        vm_tag = math_label(
            r"v_{m} \approx 2{,}5\,\mathrm{m/s}\;\text{(lärmarm)}",
            size=BODY_FONT_SIZE, color=P_CYAN,
        )
        vm_tag.next_to(duct_c, DOWN, buff=1.35)

        cont, cont_items = math_row([
            ("qvr", r"q_{v,R}", P_YELLOW), (None, "=", P_WHITE),
            ("vm", r"v_{m}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("A", "A", P_BLUE),
            (None, r"\;[\mathrm{m^{3}/s}]", P_TEAL),
        ])
        cont.move_to(RIGHT * 3.2 + UP * 1.35)

        area_parts = [
            ("A", "A", P_BLUE), (None, "=", P_WHITE),
            ("frac",
             rf"\frac{{\textcolor{{{P_YELLOW}}}{{q_{{v,R}}}}}}{{\textcolor{{{P_CYAN}}}{{v_{{m}}}}}}",
             P_WHITE),
            (None, r"\;[\mathrm{m^{2}}]", P_TEAL),
        ]

        tip_fast = math_label(r"\text{kleines}\;A \;\to\; \text{hohes}\;v_{m}", size=LABEL_FONT_SIZE, color=P_ORANGE)
        tip_slow = math_label(r"\text{großes}\;A \;\to\; \text{niedriges}\;v_{m}", size=LABEL_FONT_SIZE, color=P_CYAN)
        tip_fast.move_to(RIGHT * 3.2 + DOWN * 0.15)
        tip_slow.move_to(RIGHT * 3.2 + DOWN * 0.15)

        self.play(Create(outer), FadeIn(wall), Create(pipe), run_time=1.8)
        self.play(FadeIn(area_fill), FadeIn(area_lbl), run_time=1.0)
        self.play(FadeIn(vm_tag), *duct_air(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.8 + 1.0 + 1.6, during=duct_air)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "continuity"))
        self.play(FadeIn(cont), *duct_air(1.1), run_time=1.1)
        hold_for(self, self.NARRATION, "continuity", during=duct_air)

        for key, item_key, color in (
            ("qvr", "qvr", P_YELLOW),
            ("vm", "vm", P_CYAN),
            ("A_cont", "A", P_BLUE),
        ):
            ring = highlight_param(cont_items, item_key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), *duct_air(0.45), run_time=0.45)
            hold_for(self, self.NARRATION, key, during=duct_air)
            self.play(FadeOut(ring), *duct_air(0.25), run_time=0.25)

        area_eq, area_box, area_items = math_panel(area_parts)
        area_frac = area_items["frac"][0]
        area_items["qvr"] = area_frac[0]
        area_items["vm"] = area_frac[2]
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "area"))
        self.play(FadeOut(cont), *duct_air(0.4), run_time=0.4)
        self.play(FadeIn(area_eq), Create(area_box), *duct_air(1.2), run_time=1.2)
        self.play(FadeIn(tip_fast), *duct_air(0.5), run_time=0.5)
        self.play(
            area_fill.animate.scale(0.62), area_lbl.animate.scale(0.62),
            *duct_air(1.8, speed=2.4, color=P_ORANGE), run_time=1.8,
        )
        self.play(FadeOut(tip_fast), FadeIn(tip_slow), *duct_air(0.5, speed=2.4, color=P_ORANGE), run_time=0.5)
        self.play(
            area_fill.animate.scale(1 / 0.62 * 1.15), area_lbl.animate.scale(1 / 0.62 * 1.15),
            *duct_air(1.8, speed=0.6), run_time=1.8,
        )
        self.play(area_fill.animate.scale(1 / 1.15), area_lbl.animate.scale(1 / 1.15), FadeOut(tip_slow),
                  *duct_air(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "area", during=duct_air)

        for key, item_key, color in (
            ("A", "A", P_BLUE),
            ("qvr_num", "qvr", P_YELLOW),
            ("vm_den", "vm", P_CYAN),
        ):
            ring = highlight_param(area_items, item_key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(Create(ring), *duct_air(0.45), run_time=0.45)
            hold_for(self, self.NARRATION, key, during=duct_air)
            self.play(FadeOut(ring), *duct_air(0.25), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Calculate Duct Radius
class Beat5_CalculateRadius(Scene):
    NARRATION = [
        ("intro",
         "Now we close the sizing chain: every quantity we derived feeds the next — from cooling load to volume flow, to duct area, to radius.",
         "Jetzt die Auslegungskette: von der Kühllast zum Volumenstrom, zur Fläche, zum Radius."),
        ("flow",
         "First, volume flow q v R equals the solar cooling load Q-dot S,tr divided by density, heat capacity and Delta theta.",
         "q v R ist gleich Q Punkt S t r geteilt durch Rho a mal c p a mal Delta Theta."),
        ("qstr",
         "Q-dot S,tr is the cooling load we must remove — it sits in the numerator.",
         "Q Punkt S t r ist die abzuführende Kühllast — sie steht im Zähler."),
        ("qvr",
         "That fixes q v R — the airflow the duct must carry every second.",
         "Daraus folgt q v R — der Luftstrom, den der Kanal tragen muss."),
        ("area",
         "Next, continuity: area A equals that volume flow divided by the mean velocity v m.",
         "Als Nächstes die Kontinuität: A ist gleich q v R geteilt durch v m."),
        ("qvr_a",
         "The same q v R now sits in the numerator of the area formula.",
         "Dasselbe q v R steht jetzt im Zähler der Flächenformel."),
        ("vm",
         "v m is not calculated from the heat load — it is a design limit we choose. To keep the duct quiet, engineers usually cap mean velocity around two point five meters per second.",
         "v m kommt nicht aus der Last — es ist ein Entwurfslimit. Für leise Kanäle oft etwa zwei Komma fünf Meter pro Sekunde."),
        ("A",
         "With v m fixed, A follows — the free cross-section the duct needs.",
         "Mit festem v m folgt A — die nötige Querschnittsfläche."),
        ("radius",
         "Finally the round duct: radius r equals the square root of A over pi.",
         "Zuletzt der runde Kanal: r ist gleich die Wurzel aus A geteilt durch Pi."),
        ("A_r",
         "That same A feeds the root — geometry turns area into a length.",
         "Dieselbe Fläche A steckt unter der Wurzel — Geometrie macht Länge daraus."),
        ("r",
         "r is the duct radius we install. Load to flow to area to radius — the system is fully sized.",
         "r ist der Kanalradius. Von der Last über den Strom und die Fläche zum Radius — die Anlage ist ausgelegt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Auslegungskette: Last → Strom → Fläche → Radius", title)
        din = _din_ref("VDI 2087")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

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
            math_label(r"\dot{Q}_{S,tr}", size=BODY_FONT_SIZE, color=P_YELLOW),
            Text("→", font_size=BODY_FONT_SIZE, color=P_WHITE),
            math_label(r"q_{v,R}", size=BODY_FONT_SIZE, color=P_CYAN),
            Text("→", font_size=BODY_FONT_SIZE, color=P_WHITE),
            math_label("A", size=BODY_FONT_SIZE, color=P_BLUE),
            Text("→", font_size=BODY_FONT_SIZE, color=P_WHITE),
            math_label("r", size=BODY_FONT_SIZE, color=P_YELLOW),
        ).arrange(RIGHT, buff=0.18)
        tokens.move_to(RIGHT * 2.2 + UP * 1.85)

        flow_parts = [
            ("qvr", r"q_{v,R}", P_CYAN), (None, "=", P_WHITE),
            ("frac",
             rf"\frac{{\textcolor{{{P_YELLOW}}}{{\dot{{Q}}_{{S,tr}}}}}}"
             rf"{{\textcolor{{{P_GREEN}}}{{\rho_{{a}} \cdot c_{{p,a}}}} \cdot \textcolor{{{P_BLUE}}}{{Δθ}}}}",
             P_WHITE),
        ]

        area_parts = [
            ("A", "A", P_BLUE), (None, "=", P_WHITE),
            ("frac",
             rf"\frac{{\textcolor{{{P_CYAN}}}{{q_{{v,R}}}}}}{{\textcolor{{{P_TEAL}}}{{v_{{m}}}}}}",
             P_WHITE),
        ]

        rad_parts = [
            ("r", "r", P_YELLOW), (None, "=", P_WHITE),
            ("root",
             rf"\sqrt{{\frac{{\textcolor{{{P_BLUE}}}{{A}}}}{{\textcolor{{{P_TEAL}}}{{\pi}}}}}}",
             P_WHITE),
        ]

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(circle), FadeIn(fill), run_time=1.2)
        self.play(FadeIn(center_dot), Create(radius_line), FadeIn(r_lbl), run_time=0.9)
        self.play(FadeIn(tokens), run_time=0.9)

        flow_eq, flow_box, flow_items = math_panel(flow_parts, color=P_CYAN, size=FORMULA_FONT_SIZE)
        flow_frac = flow_items["frac"][0]
        flow_items["qstr"] = flow_frac[0]
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

        self.play(flow_panel.animate.scale(0.66).move_to(RIGHT * 2.0 + UP * 1.02), run_time=0.7)

        area_eq, area_box, area_items = math_panel(area_parts, color=P_BLUE, size=FORMULA_FONT_SIZE)
        area_frac = area_items["frac"][0]
        area_items["qvr"] = area_frac[0]
        area_items["vm"] = area_frac[2]
        area_panel = VGroup(area_box, area_eq)
        vm_note = VGroup(
            Text("Entwurfslimit (Lärmschutz)", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            math_label(r"v_{m} \approx 2{,}5\,\mathrm{m/s}", size=BODY_FONT_SIZE, color=P_CYAN),
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

        self.play(area_panel.animate.scale(0.66).move_to(RIGHT * 2.0 + DOWN * 0.18), run_time=0.7)

        rad_eq, rad_box, rad_items = math_panel(rad_parts, color=P_YELLOW, size=FORMULA_FONT_SIZE)
        rad_items["A"] = rad_items["root"]
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
