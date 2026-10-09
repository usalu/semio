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
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE,
    watt_anchor, highlight_param,
    house_section, room_section, person_glyph, seated_person_glyph, lamp_glyph, droplets,
    ripples, smooth_path, flow_guides, flow_animation, pulse_flashes,
    math_label, math_readout, math_panel, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    SAFE_BOTTOM, set_vo_language, load_vo_timing,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Interne Wärmegewinne"


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


#region Shared helpers

def _activity_figure(kind: str, color=P_ORANGE, s: float = 1.0):
    """🏃 Figure in one activity pose — ``sleep``, ``sit``, ``walk`` or ``sprint`` — in the ``person_glyph`` line style.

    Feet sit on ``y = 0`` so a row of poses shares one ground line; metabolic
    rates per pose follow ISO 7730 / VDI 2078 orders of magnitude. Returns the
    figure and the point its body heat leaves from.
    """
    sw = 1.55

    def L(a, b, c=color, w=sw):
        return Line(np.array([*a, 0.0]) * s, np.array([*b, 0.0]) * s, color=c, stroke_width=w)

    def H(cx, cy, r=0.085):
        return Circle(radius=r * s, color=color, stroke_width=sw).move_to(np.array([cx, cy, 0.0]) * s)

    if kind == "sleep":
        bed = VGroup(L([-0.62, 0.0], [0.62, 0.0], P_WHITE, 1.3), L([-0.62, 0.0], [-0.62, 0.22], P_WHITE, 1.3),
                     L([-0.58, -0.0], [-0.58, -0.12], P_WHITE, 1.3), L([0.58, 0.0], [0.58, -0.12], P_WHITE, 1.3))
        body = VGroup(H(-0.4, 0.13), L([-0.31, 0.1], [0.28, 0.1]), L([-0.31, 0.16], [0.24, 0.16]),
                      L([0.28, 0.1], [0.52, 0.08]))
        return VGroup(bed, body), np.array([-0.05, 0.16, 0.0]) * s
    if kind == "sit":
        seated = seated_person_glyph(ORIGIN, color=color, scale=2.2 * s)
        return seated["group"], seated["chest"]
    if kind == "walk":
        return VGroup(
            H(0.0, 1.12), L([-0.06, 1.0], [-0.05, 0.48]), L([0.06, 1.0], [0.05, 0.48]),
            L([0.0, 0.48], [-0.24, 0.02]), L([0.0, 0.48], [0.24, 0.02]),
            L([0.0, 0.92], [-0.22, 0.62]), L([0.0, 0.92], [0.22, 0.62]),
        ), np.array([0.0, 0.8, 0.0]) * s
    if kind == "sprint":
        back_leg = VMobject(color=color, stroke_width=sw).set_points_as_corners(
            [np.array([0.0, 0.5, 0.0]) * s, np.array([-0.26, 0.3, 0.0]) * s, np.array([-0.46, 0.1, 0.0]) * s])
        front_leg = VMobject(color=color, stroke_width=sw).set_points_as_corners(
            [np.array([0.0, 0.5, 0.0]) * s, np.array([0.22, 0.26, 0.0]) * s, np.array([0.14, 0.02, 0.0]) * s])
        dashes = VGroup(*[L([-0.8 + 0.05 * k, y], [-0.55 + 0.05 * k, y], P_WHITE, 1.1).set_stroke(opacity=0.5)
                          for k, y in enumerate((0.78, 0.55, 0.32))])
        return VGroup(
            dashes, H(0.4, 1.12), L([-0.04, 0.5], [0.26, 1.0]), L([0.06, 0.5], [0.34, 0.98]),
            back_leg, front_leg, L([0.3, 0.98], [0.06, 0.78]), L([0.3, 0.98], [0.54, 0.8]),
        ), np.array([0.15, 0.75, 0.0]) * s
    raise ValueError(f"unknown activity pose {kind!r}")


def _office_furniture(room):
    """🖥️ Two workstations — desk, monitor, chair — standing on the room floor in the Physical Fundamentals line style."""
    y_f = room["y_f"]
    group, warm = VGroup(), []
    for x in (room["x_l"] + 0.9, room["center"][0] + 0.4):
        top = y_f + 0.75
        desk = VGroup(
            Line(np.array([x, top, 0.0]), np.array([x + 1.2, top, 0.0]), color=P_WHITE, stroke_width=1.5),
            Line(np.array([x + 0.1, top, 0.0]), np.array([x + 0.1, y_f, 0.0]), color=P_WHITE, stroke_width=1.5),
            Line(np.array([x + 1.1, top, 0.0]), np.array([x + 1.1, y_f, 0.0]), color=P_WHITE, stroke_width=1.5),
        )
        screen = Rectangle(width=0.5, height=0.32, color=P_CYAN, stroke_width=1.6).move_to(
            np.array([x + 0.7, top + 0.26, 0.0]))
        stand = Line(screen.get_bottom(), np.array([x + 0.7, top, 0.0]), color=P_CYAN, stroke_width=1.6)
        chair = VGroup(
            Line(np.array([x - 0.55, y_f + 0.45, 0.0]), np.array([x - 0.15, y_f + 0.45, 0.0]), color=P_WHITE, stroke_width=1.3),
            Line(np.array([x - 0.55, y_f + 0.45, 0.0]), np.array([x - 0.58, y_f + 0.95, 0.0]), color=P_WHITE, stroke_width=1.3),
            Line(np.array([x - 0.35, y_f + 0.45, 0.0]), np.array([x - 0.35, y_f, 0.0]), color=P_WHITE, stroke_width=1.3),
            Line(np.array([x - 0.5, y_f, 0.0]), np.array([x - 0.2, y_f, 0.0]), color=P_WHITE, stroke_width=1.3),
        )
        group.add(VGroup(desk, screen, stand, chair))
        warm.append(screen.get_center())
    return {"group": group, "warm": warm}

#endregion


#region Beat 1 — Office room / enclosed envelope

class Beat1_OfficeRoom(Scene):
    NARRATION = [
        ("intro",
         "Before the sun even touches the building, heat is already produced inside — by the internal sources.",
         "Schon bevor die Sonne das Gebäude trifft, entsteht innen Wärme — aus den internen Quellen."),
        ("facade",
         "Start from the commercial facade, then zoom into the insulated office envelope.",
         "Beginnen wir bei der Gewerbefassade und zoomen in die gedämmte Bürohülle."),
        ("interior",
         "Inside sits a commercial workplace — desks, screens, chairs — sealed by insulation.",
         "Drinnen liegt ein gewerblicher Arbeitsplatz — Tische, Bildschirme, Stühle — dicht gedämmt."),
        ("outro",
         "In offices and lecture halls, internal heat is often the primary cooling load.",
         "In Büros und Hörsälen ist die interne Wärme oft die dominante Kühllast."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Die umschlossene Umgebung", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        facade = house_section(np.array([0.0, -0.55, 0.0]))["group"]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "facade"))
        self.play(FadeIn(facade), run_time=1.4)
        self.play(facade.animate.scale(1.35).move_to(DOWN * 0.15), run_time=2.0)
        hold_for(self, self.NARRATION, "facade", used=3.4 + 0.35)

        room = room_section(np.array([0.0, -0.2, 0.0]), w=6.6, h=2.5)
        insulation_box = SurroundingRectangle(room["shell"], buff=0.12, corner_radius=0.12, color=P_ORANGE,
                                              stroke_width=2.2)
        office = _office_furniture(room)
        interior_label = Text("Gewerblicher Arbeitsbereich", font_size=BODY_FONT_SIZE, color=P_CYAN)
        interior_label.move_to(np.array([1.3, room["y_c"] - 0.45, 0.0]))
        office_env_label = Text("Bürohülle", font_size=SUBTITLE_FONT_SIZE, color=P_WHITE)
        office_env_label.next_to(insulation_box, UP, buff=0.12)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "interior"))
        self.play(ReplacementTransform(facade, room["shell"]), FadeIn(room["glass"]), Create(insulation_box),
                  run_time=1.8)
        self.add(room["air"])
        self.play(LaggedStart(*[FadeIn(m) for m in office["group"]], lag_ratio=0.15), FadeIn(interior_label),
                  FadeIn(office_env_label), run_time=1.6)
        hold_for(self, self.NARRATION, "interior", used=3.4 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        self.play(room["air"].animate.set_fill(P_RED, opacity=0.1), run_time=0.8)
        hold_for(self, self.NARRATION, "outro", during=lambda rt: [
            ripples(office["warm"], r_max=0.5, color=P_ORANGE, cycles=max(1.0, rt / 1.3))])

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 — Human factor

class Beat2_HumanFactor(Scene):
    NARRATION = [
        ("intro",
         "Think of a human body as a biological heater sitting at a desk.",
         "Stellen Sie sich den menschlichen Körper als biologischen Heizkörper am Schreibtisch vor."),
        ("modes",
         "Heat leaves as radiation, convection, and respiration — both sensible and latent.",
         "Wärme geht als Strahlung, Konvektion und Atmung ab — fühlbar und latent."),
        ("anchor",
         "A single seated adult emits about one hundred watts — like a bright light bulb.",
         "Eine sitzende Person gibt etwa einhundert Watt ab — wie eine helle Glühbirne."),
        ("activities",
         "How much heat the body emits depends on activity — it rises from sleeping to desk work, walking and high-performance sport.",
         "Wie viel Wärme der Körper abgibt, hängt von der Aktivität ab — sie steigt vom Schlafen über Büroarbeit und Gehen bis zum Hochleistungssport."),
        ("activity_scale",
         "On one watt scale: about eighty watts asleep, one hundred at the desk, two hundred walking, up to eight hundred in sport.",
         "Auf einer Watt-Skala: rund achtzig Watt im Schlaf, hundert am Schreibtisch, zweihundert beim Gehen, bis achthundert beim Sport."),
        ("formula",
         "We calculate Q-dot Personen as n times the specific emission q-dot p, in watts.",
         "Wir berechnen Q-Punkt Personen als n mal die spezifische Abgabe q-Punkt p, in Watt."),
        ("n",
         "n is the number of occupants in the room.",
         "n ist die Anzahl der Personen im Raum."),
        ("qp",
         "q-dot p is roughly one hundred watts per person at desk work.",
         "q-Punkt p liegt bei etwa einhundert Watt pro Person bei Büroarbeit."),
        ("scale",
         "Pack fifty students into a lecture hall and you run a five-thousand-watt heater continuously.",
         "Fünfzig Studierende im Hörsaal bedeuten dauerhaft fünftausend Watt — wie fünf Toaster."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Menschliche Stoffwechselwärme", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        room = room_section(np.array([-0.9, 0.0, 0.0]), w=6.6, h=3.4)
        seated = seated_person_glyph(np.array([-1.55, room["y_f"], 0.0]), color=P_ORANGE, scale=3.6)
        desk_top_y = seated["figure"][7].get_end()[1] - 0.12
        dx0 = seated["figure"][7].get_end()[0] + 0.1
        desk = VGroup(
            Line(np.array([dx0, desk_top_y, 0.0]), np.array([dx0 + 2.1, desk_top_y, 0.0]), color=P_WHITE, stroke_width=1.5),
            Line(np.array([dx0 + 2.0, desk_top_y, 0.0]), np.array([dx0 + 2.0, room["y_f"], 0.0]), color=P_WHITE,
                 stroke_width=1.5),
            Line(np.array([dx0 + 0.95, desk_top_y + 0.02, 0.0]), np.array([dx0 + 1.45, desk_top_y + 0.02, 0.0]),
                 color=P_CYAN, stroke_width=1.8),
            Line(np.array([dx0 + 1.45, desk_top_y + 0.02, 0.0]), np.array([dx0 + 1.52, desk_top_y + 0.42, 0.0]),
                 color=P_CYAN, stroke_width=1.8),
        )
        office_setup = VGroup(seated["group"], desk)
        self.add(room["air"])
        self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), FadeIn(office_setup), run_time=1.6)

        # 🌡️ Three heat paths leave the body without crossing: Strahlung spreads
        # from the chest, Konvektion rises off the head to the ceiling, Atmung
        # leaves the mouth to the right — warm air above, moisture below.
        chest = seated["chest"]
        head = seated["head"]
        mouth = seated["mouth"]
        lbl_rad = Text("Strahlung", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        lbl_rad.next_to(np.array([chest[0] - 0.85, chest[1] + 0.05, 0.0]), LEFT, buff=0.0)
        plume = [smooth_path([head.get_top() + UP * 0.08 + RIGHT * dx, head.get_top() + UP * 0.45 + RIGHT * (0.12 + dx),
                              head.get_top() + UP * 0.85 + LEFT * (0.05 - dx),
                              np.array([head.get_center()[0] + 0.1 + dx, room["y_c"] - 0.08, 0.0])])
                 for dx in (-0.08, 0.08)]
        lbl_conv = Text("Konvektion", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        lbl_conv.next_to(np.array([head.get_center()[0] - 0.3, head.get_top()[1] + 0.65, 0.0]), LEFT, buff=0.0)
        breath_warm = [smooth_path([mouth + RIGHT * 0.06, mouth + RIGHT * 0.6 + UP * 0.12, mouth + RIGHT * 1.25 + UP * 0.32])]
        breath_wet = [smooth_path([mouth + RIGHT * 0.06 + DOWN * 0.04, mouth + RIGHT * 0.65 + DOWN * 0.1,
                                   mouth + RIGHT * 1.25 + DOWN * 0.18])]
        lbl_sens = Text("fühlbar", font_size=LABEL_FONT_SIZE, color=P_RED)
        lbl_sens.next_to(breath_warm[0].get_end(), RIGHT, buff=0.15)
        lbl_lat = Text("latent", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        lbl_lat.next_to(breath_wet[0].get_end(), RIGHT, buff=0.15)
        lbl_resp = Text("Atmung", font_size=BODY_FONT_SIZE, color=P_RED)
        lbl_resp.next_to(lbl_sens, UP, buff=0.18).align_to(lbl_sens, LEFT)
        drops = droplets(breath_wet[0].get_end() + LEFT * 0.35 + DOWN * 0.04, n=5, spread=(0.25, 0.05), seed=5)

        def body_heat(rt):
            cyc = max(1.0, rt / 1.4)
            return [ripples([chest], r_max=0.7, color=P_ORANGE, cycles=cyc),
                    flow_animation([(plume, P_ORANGE, P_RED)], waves=4, radius=0.055, cycles=cyc),
                    flow_animation([(breath_warm, P_RED, P_ORANGE)], waves=3, radius=0.05, cycles=cyc, streak=False),
                    flow_animation([(breath_wet, P_BLUE)], waves=3, radius=0.045, cycles=cyc, streak=False)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "modes"))
        self.play(ripples([chest], r_max=0.7, color=P_ORANGE, cycles=1.2), FadeIn(lbl_rad), run_time=1.4)
        self.play(flow_animation([(plume, P_ORANGE, P_RED)], waves=4, radius=0.055, cycles=1.0), FadeIn(lbl_conv),
                  run_time=1.4)
        self.play(*body_heat(1.4)[2:], FadeIn(lbl_sens), FadeIn(lbl_resp), FadeIn(lbl_lat),
                  FadeIn(drops, lag_ratio=0.2), run_time=1.4)
        hold_for(self, self.NARRATION, "modes", during=body_heat)

        badge = watt_anchor(100, compare="bulb", title="Körperwärme")
        badge.scale(0.85).next_to(room["wall_r"], RIGHT, buff=0.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "anchor"))
        self.play(FadeIn(badge, shift=LEFT * 0.25), room["air"].animate.set_fill(P_RED, opacity=0.08), *body_heat(1.0),
                  run_time=1.0)
        hold_for(self, self.NARRATION, "anchor", during=body_heat)

        desk_group = VGroup(room["group"], office_setup, lbl_rad, lbl_conv, lbl_sens, lbl_lat, lbl_resp, drops, badge)
        self.play(FadeOut(desk_group), run_time=0.7)

        # 🏃 Activity ladder — every pose animated, its watt output counted from
        # zero, then all four morph onto one watt scale (Schlafen → Sport).
        acts = [
            ("Schlafen", 80, "sleep", P_TEAL),
            ("Büroarbeit", 100, "sit", P_CYAN),
            ("Gehen", 200, "walk", P_YELLOW),
            ("Hochleistungssport", 800, "sprint", P_RED),
        ]
        base_y = -0.75
        xs = [-4.8, -1.7, 1.3, 4.3]
        figures, trackers, readouts, names, heat_spots = [], [], [], [], []
        for (name, watts, pose, color), x in zip(acts, xs):
            fig, heat_at = _activity_figure(pose, color=P_ORANGE, s=1.0)
            fig.shift(RIGHT * x + UP * base_y)
            head_y = base_y + (0.25 if pose == "sleep" else 1.25)
            heat_spots.append((np.array([x, base_y, 0.0]) + heat_at, 0.3 + watts / 1300))
            tr = ValueTracker(0.0)
            read = math_readout(
                lambda tr=tr: rf"{de_num(tr.get_value())}\,\mathrm{{W}}",
                np.array([x, head_y + 0.75 + watts / 1600, 0.0]),
                size=BODY_FONT_SIZE, color=color, edge="center",
            )
            label = Text(name, font_size=LABEL_FONT_SIZE, color=color)
            label.move_to([x, base_y - 0.42, 0.0])
            figures.append(fig); trackers.append(tr); readouts.append(read)
            names.append(label)

        def activity_heat(rt, upto=4):
            cyc = max(1.0, rt / 1.2)
            return [ripples([spot], r_max=r, color=P_ORANGE, cycles=cyc) for spot, r in heat_spots[:upto]]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "activities"))
        for k, (fig, tr, read, label, (name, watts, pose, color)) in enumerate(
            zip(figures, trackers, readouts, names, acts),
        ):
            self.add(read)
            self.play(FadeIn(fig), FadeIn(label), tr.animate.set_value(watts), *activity_heat(1.1, k + 1),
                      run_time=1.1)
        hold_for(self, self.NARRATION, "activities", during=activity_heat)

        # Freeze the live counters so the morph below transforms plain labels.
        static_reads = VGroup(*[
            math_label(rf"{de_num(watts)}\,\mathrm{{W}}",
                       at=np.array([x, base_y + (0.25 if pose == "sleep" else 1.25) + 0.75 + watts / 1600, 0.0]),
                       size=BODY_FONT_SIZE, color=color, edge="center")
            for (name, watts, pose, color), x in zip(acts, xs)
        ])
        self.add(static_reads)
        for read in readouts:
            self.remove(read)

        scale_y = -0.85
        def w_to_x(w):
            return -5.2 + (w / 800.0) * 10.4
        scale_line = Line([w_to_x(0), scale_y, 0], [w_to_x(800), scale_y, 0], color=P_WHITE, stroke_width=2.5)
        ticks = VGroup()
        tick_labels = VGroup()
        for w in (0, 200, 400, 600, 800):
            x = w_to_x(w)
            ticks.add(Line([x, scale_y - 0.09, 0], [x, scale_y + 0.09, 0], color=P_WHITE, stroke_width=2))
            tick_labels.add(math_label(de_num(w), at=np.array([x, scale_y - 0.42, 0.0]),
                                       size=LABEL_FONT_SIZE, color=P_TEAL, edge="center"))
        unit_label = math_label(r"\mathrm{W}", at=np.array([w_to_x(800) + 0.45, scale_y - 0.42, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_TEAL, edge="center")
        dots = VGroup(*[
            Dot([w_to_x(watts), scale_y, 0], radius=0.09, color=color)
            for (name, watts, pose, color) in acts
        ])
        # Schlafen (80 W) hangs below the axis under the tick row, the others sit
        # above it — 80 W and 100 W are too close on the scale to share a side.
        name_targets = []
        read_targets = []
        for i, ((name, watts, pose, color), label) in enumerate(zip(acts, names)):
            x = w_to_x(watts)
            if i == 0:
                read_targets.append(np.array([x, scale_y - 0.85, 0.0]))
                name_targets.append(np.array([x, scale_y - 1.18, 0.0]))
            else:
                read_targets.append(np.array([x, scale_y + 0.82, 0.0]))
                name_targets.append(np.array([x, scale_y + 0.46, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "activity_scale"))
        self.play(
            Create(scale_line), Create(ticks), FadeIn(tick_labels), FadeIn(unit_label),
            *[ReplacementTransform(fig, dot) for fig, dot in zip(figures, dots)],
            *[label.animate.move_to(t) for label, t in zip(names, name_targets)],
            *[read.animate.move_to(t) for read, t in zip(static_reads, read_targets)],
            run_time=2.2,
        )
        buero_ring = Circle(radius=0.17, color=P_ORANGE, stroke_width=3).move_to(dots[1].get_center())
        self.play(Create(buero_ring), run_time=0.5)
        hold_for(self, self.NARRATION, "activity_scale", used=2.7 + 0.35)

        scale_assembly = VGroup(
            scale_line, ticks, tick_labels, unit_label, dots, buero_ring,
            *names, *static_reads,
        )
        self.play(scale_assembly.animate.scale(0.82).shift(UP * 1.75), run_time=0.8)

        row, box, items = math_panel([
            ("qp_tot", r"\dot{Q}_{\text{Pers}}", P_ORANGE), (None, "=", P_WHITE),
            ("n", "n", P_CYAN), (None, r"\cdot", P_WHITE),
            ("qp", r"\dot{q}_{p}", P_ORANGE), (None, r"\;[\mathrm{W}]", P_TEAL),
        ])
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(row), Create(box), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

        for key, color in (("n", P_CYAN), ("qp", P_ORANGE)):
            ring = highlight_param(items, key, color=color)
            anims = [Create(ring)]
            if key == "qp":
                # 💡 q̇_p is the ringed Büroarbeit point on the scale — fly its
                # 100 W value into the formula slot.
                qp_fly = static_reads[1].copy()
                anims.append(qp_fly.animate.next_to(box, UP, buff=0.14).set_x(items["qp"].get_center()[0]))
            self.play(*anims, run_time=0.6)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.6 + 0.35)
            fades = [FadeOut(ring)]
            if key == "qp":
                fades.append(FadeOut(qp_fly))
            self.play(*fades, run_time=0.25)

        self.play(FadeOut(scale_assembly), FadeOut(row), FadeOut(box), run_time=0.7)

        single = person_glyph(DOWN * 0.05, color=P_ORANGE, scale=1.8)
        single_label = Text("Einzelperson (100 W)", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        single_label.next_to(single, DOWN, buff=0.28)
        self.play(FadeIn(single), FadeIn(single_label), run_time=0.9)

        # 📐 Shift the whole hall block up (keeping its internal spacing
        # untouched) so the anchor badge below it has real room before the
        # caption band, instead of the badge crowding into the subtitle.
        hall_shift = UP * 0.4
        hall_outline = Rectangle(
            width=9.6, height=3.0, stroke_color="#2C3545", stroke_width=2,
            fill_color=P_DEEP_DARK, fill_opacity=0.85,
        ).move_to(UP * 0.15 + hall_shift)
        tier_y = np.linspace(-1.15, 0.95, 5) + hall_shift[1]
        tier_lines = VGroup(*[
            Line(start=[-4.5, y, 0], end=[4.5, y, 0], stroke_color="#2C3545", stroke_width=1, stroke_opacity=0.6)
            for y in tier_y
        ])
        grid_icons = VGroup()
        for y_pos in tier_y:
            for c in range(10):
                x_pos = -4.05 + c * 0.9
                icon = person_glyph(ORIGIN, color=P_ORANGE, scale=0.5)
                grid_icons.add(icon.move_to([x_pos, y_pos + 0.03 + icon.height / 2, 0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "scale"))
        self.play(FadeOut(single_label), Create(hall_outline), Create(tier_lines), run_time=0.8)
        self.play(
            ReplacementTransform(single, grid_icons[0]),
            LaggedStart(*[TransformFromCopy(grid_icons[0], icon) for icon in grid_icons[1:]], lag_ratio=0.03),
            run_time=2.0,
        )
        hall_anchor = watt_anchor(5000, compare="toaster", title="Gesamtwärme", row=True)
        hall_anchor.next_to(hall_outline, DOWN, buff=0.3)
        # Clamp above the caption band explicitly — measured against the
        # actual rendered badge, never a hand-tuned shift that silently goes
        # stale (that gap is exactly what let this badge overlap the caption).
        clearance = (SAFE_BOTTOM + 0.15) - hall_anchor.get_bottom()[1]
        if clearance > 0:
            hall_anchor.shift(UP * clearance)
        hall_heat_at = [icon.get_center() + UP * 0.05 for icon in grid_icons]

        def hall_heat(rt):
            return [ripples(hall_heat_at, r_max=0.32, rings=2, color=P_ORANGE, cycles=max(1.0, rt / 1.3))]

        self.play(*hall_heat(2.4), hall_outline.animate.set_fill(P_RED, opacity=0.18), FadeIn(hall_anchor),
                  run_time=2.4)
        hold_for(self, self.NARRATION, "scale", during=hall_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 3 — Devices and lighting

class Beat3_DevicesLighting(Scene):
    NARRATION = [
        ("intro",
         "Every laptop, server rack and light fixture converts electrical power straight into heat.",
         "Jedes Notebook, jedes Serverrack und jede Leuchte wandelt elektrische Leistung direkt in Wärme um."),
        ("devices",
         "Plug loads follow Q-dot Geräte equals the sum of P el times the usage factor f N, in watts.",
         "Steckerlasten: Q-Punkt Geräte gleich Summe aus P el mal Nutzungsfaktor f N, in Watt."),
        ("pel",
         "P el is the electrical power drawn by each device.",
         "P el ist die elektrische Leistung jedes Geräts."),
        ("fn",
         "f N accounts for the fact that not every device runs at once.",
         "f N berücksichtigt, dass nicht jedes Gerät gleichzeitig läuft."),
        ("fn_usage",
         "The usage sets f N: in an office about seven of ten devices run, in a server room all of them, in a meeting room only three.",
         "Die Nutzung bestimmt f N: im Büro laufen etwa sieben von zehn Geräten, im Serverraum alle, im Besprechungsraum nur drei."),
        ("lights",
         "Ceiling lighting adds Q-dot Licht equals the sum of P Licht times the coincidence factor f g, in watts.",
         "Deckenlicht: Q-Punkt Licht gleich Summe aus P Licht mal Gleichzeitigkeit f g, in Watt."),
        ("pl",
         "P Licht is the installed electrical power of the luminaires.",
         "P Licht ist die installierte elektrische Leistung der Leuchten."),
        ("fg",
         "f g is the coincidence factor — the share of luminaires that actually run at the same time.",
         "f g ist der Gleichzeitigkeitsfaktor — der Anteil der gleichzeitig betriebenen Leuchten."),
        ("outro",
         "Together, devices and lights bake the room from the inside out.",
         "Zusammen heizen Geräte und Licht den Raum von innen heraus auf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Geräte, Steckerlasten und Beleuchtung", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        room = room_section(np.array([0.0, 0.25, 0.0]), w=8.6, h=2.6)
        desk_y = room["y_f"] + 0.8
        desk = VGroup(
            Line(np.array([-3.6, desk_y, 0.0]), np.array([-1.2, desk_y, 0.0]), color=P_WHITE, stroke_width=1.5),
            Line(np.array([-3.5, desk_y, 0.0]), np.array([-3.5, room["y_f"], 0.0]), color=P_WHITE, stroke_width=1.5),
            Line(np.array([-1.3, desk_y, 0.0]), np.array([-1.3, room["y_f"], 0.0]), color=P_WHITE, stroke_width=1.5),
        )
        laptop = VGroup(
            Line(np.array([-2.75, desk_y + 0.02, 0.0]), np.array([-2.15, desk_y + 0.02, 0.0]), color=P_CYAN, stroke_width=1.8),
            Line(np.array([-2.15, desk_y + 0.02, 0.0]), np.array([-2.05, desk_y + 0.5, 0.0]), color=P_CYAN, stroke_width=1.8),
        )
        rack_frame = Rectangle(width=1.0, height=1.9, color=P_CYAN, stroke_width=1.6)
        rack_frame.move_to(np.array([2.6, room["y_f"] + 0.95, 0.0]))
        slots = VGroup(*[
            VGroup(Rectangle(width=0.78, height=0.2, color=P_WHITE, stroke_width=1.1),
                   Dot(radius=0.03, color=P_CYAN)).move_to(rack_frame.get_top() + DOWN * (0.25 + 0.34 * i))
            for i in range(5)
        ])
        for slot in slots:
            slot[1].move_to(slot[0].get_left() + RIGHT * 0.12)
        server_rack = VGroup(rack_frame, slots)
        devices = VGroup(desk, laptop, server_rack)
        self.add(room["air"])
        self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), run_time=0.8)
        self.play(LaggedStart(FadeIn(desk), FadeIn(laptop, shift=DOWN * 0.15), FadeIn(server_rack, shift=DOWN * 0.15),
                              lag_ratio=0.35), run_time=1.4)

        # ⚡ Electricity flows into each device along its cable and leaves as heat.
        plug_y = room["y_f"] + 0.12
        cables = [smooth_path([np.array([room["x_r"] - 0.05, plug_y, 0.0]), np.array([0.0, plug_y, 0.0]),
                               np.array([-1.9, plug_y, 0.0]), np.array([-2.15, desk_y + 0.02, 0.0])]),
                  smooth_path([np.array([room["x_r"] - 0.05, plug_y, 0.0]), np.array([3.4, plug_y, 0.0]),
                               rack_frame.get_bottom() + RIGHT * 0.3])]
        cable_lines = VGroup(*[c.copy().set_stroke(P_CYAN, width=1.3, opacity=0.4) for c in cables])
        warm = [laptop.get_center() + UP * 0.1, rack_frame.get_center() + UP * 0.3]

        def device_heat(rt):
            cyc = max(1.0, rt / 1.3)
            return [flow_animation([(cables, P_CYAN, P_ORANGE)], waves=4, radius=0.045, cycles=cyc, streak=False),
                    ripples(warm, r_max=0.75, color=P_ORANGE, cycles=cyc)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "devices"))
        self.play(Create(cable_lines), run_time=0.6)
        self.play(*device_heat(2.0), laptop.animate.set_color(P_ORANGE), rack_frame.animate.set_color(P_ORANGE),
                  room["air"].animate.set_fill(P_RED, opacity=0.08), run_time=2.0)
        devices_group = VGroup(room["group"], devices, cable_lines)
        self.play(devices_group.animate.scale(0.78).shift(UP * 0.45), run_time=0.8)
        warm[:] = [laptop.get_center() + UP * 0.08, rack_frame.get_center() + UP * 0.25]
        cables[:] = [cl.copy() for cl in cable_lines]

        row_d, box_d, items_d = math_panel([
            ("qg", r"\dot{Q}_{\text{Geräte}}", P_CYAN), (None, "=", P_WHITE),
            (None, r"\Sigma", P_WHITE),
            ("pel", r"P_{el}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("fn", r"f_{N}", P_YELLOW), (None, r"\;[\mathrm{W}]", P_TEAL),
        ])
        self.play(FadeIn(row_d), Create(box_d), *device_heat(1.1), run_time=1.1)
        hold_for(self, self.NARRATION, "devices", during=device_heat)

        for key, color in (("pel", P_CYAN), ("fn", P_YELLOW)):
            ring = highlight_param(items_d, key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, during=device_heat)
            self.play(FadeOut(ring), run_time=0.25)

        # 📊 f_N aus der Nutzung — in each usage a share of ten devices lights
        # up, and f_N is read live off that lit share (jeweils animiert).
        self.play(FadeOut(devices_group), run_time=0.5)
        fn_ring = highlight_param(items_d, "fn", color=P_YELLOW)
        usages = [("Büro", 7, P_CYAN, -3.9), ("Serverraum", 10, P_ORANGE, 0.0), ("Besprechung", 3, P_TEAL, 3.9)]
        usage_groups = VGroup()
        usage_anim_specs = []
        for name, lit, color, cx in usages:
            frame = RoundedRectangle(
                width=3.4, height=2.4, corner_radius=0.12,
                color=color, stroke_width=2.2, fill_color="#12151C", fill_opacity=0.92,
            ).move_to([cx, 0.5, 0])
            label = Text(name, font_size=BODY_FONT_SIZE, color=color)
            label.next_to(frame.get_top(), DOWN, buff=0.12)
            icons = VGroup()
            for i in range(10):
                r, c = divmod(i, 5)
                icon = VGroup(
                    Rectangle(width=0.34, height=0.22, color="#2C3545", stroke_width=1.8, fill_opacity=0.0),
                    Line(ORIGIN, RIGHT * 0.2, color="#2C3545", stroke_width=1.8).move_to([0, -0.16, 0]),
                ).move_to([cx - 1.1 + c * 0.55, 0.7 - r * 0.55, 0])
                icons.add(icon)
            tr = ValueTracker(0.0)
            read = math_readout(
                lambda tr=tr: rf"f_{{N}} = {de_num(tr.get_value(), 1)}",
                np.array([cx, -0.30, 0.0]), size=BODY_FONT_SIZE, color=color, edge="center",
            )
            usage_groups.add(VGroup(frame, label, icons))
            usage_anim_specs.append((icons, lit, color, tr, read))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fn_usage"))
        self.play(Create(fn_ring), FadeIn(usage_groups), run_time=0.9)
        for icons, lit, color, tr, read in usage_anim_specs:
            self.add(read)
            self.play(
                LaggedStart(*[
                    icons[i].animate.set_color(color).set_fill(color, opacity=0.45)
                    for i in range(lit)
                ], lag_ratio=0.12),
                tr.animate.set_value(lit / 10),
                run_time=1.1,
            )
        hold_for(self, self.NARRATION, "fn_usage", used=0.9 + 3 * 1.1 + 0.35)

        usage_reads = [read for *_rest, read in usage_anim_specs]
        self.play(
            FadeOut(usage_groups), *[FadeOut(r) for r in usage_reads],
            FadeOut(fn_ring), FadeOut(row_d), FadeOut(box_d),
            run_time=0.7,
        )

        # 💡 Lighting: two Physical Fundamentals pendant lamps — light rays to the
        # floor, heat ripples spreading down from each bulb.
        lroom = room_section(np.array([0.0, 0.3, 0.0]), w=7.4, h=2.7)
        lamps = [lamp_glyph(np.array([x, lroom["y_c"], 0.0]), drop=0.5) for x in (-1.8, 1.8)]
        bulbs = [lp["bulb"].get_center() for lp in lamps]
        fixture_l, fixture_r = lamps[0]["bulb"], lamps[1]["bulb"]
        fixtures = VGroup(*[lp["group"] for lp in lamps])

        def light_rays(bulb):
            return VGroup(*[Line(bulb, np.array([bulb[0] + dx, lroom["y_f"] + 0.02, 0.0]), color=P_YELLOW,
                                 stroke_width=1.8, stroke_opacity=0.6) for dx in (-1.1, -0.4, 0.4, 1.1)])

        rays_l, rays_r = light_rays(bulbs[0]), light_rays(bulbs[1])
        lit = {"l": True, "r": True}

        def light_heat(rt):
            on = [b for b, k in zip(bulbs, ("l", "r")) if lit[k]]
            paths = [[b, r.get_end()] for b, r_set, k in zip(bulbs, (rays_l, rays_r), ("l", "r")) if lit[k]
                     for r in r_set]
            return [ripples(on, r_max=0.8, color=P_RED, down=True, cycles=max(1.0, rt / 1.3)),
                    pulse_flashes(paths, P_YELLOW, repeats=max(1, int(rt / 1.6)), width=3.5)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lights"))
        self.add(lroom["air"])
        self.play(FadeIn(lroom["shell"]), FadeIn(lroom["glass"]), FadeIn(fixtures), run_time=1.2)
        self.play(*[b.animate.set_fill(P_YELLOW, opacity=0.8) for b in (fixture_l, fixture_r)],
                  LaggedStart(*[Create(r) for r in (*rays_l, *rays_r)], lag_ratio=0.08), run_time=1.2)
        self.play(*light_heat(1.4), lroom["air"].animate.set_fill(P_RED, opacity=0.08), run_time=1.4)

        lighting_group = VGroup(lroom["group"], fixtures, rays_l, rays_r)
        self.play(lighting_group.animate.scale(0.82).shift(UP * 0.4), run_time=0.7)
        bulbs[:] = [fixture_l.get_center(), fixture_r.get_center()]

        row_l, box_l, items_l = math_panel([
            ("ql", r"\dot{Q}_{\text{Licht}}", P_YELLOW), (None, "=", P_WHITE),
            (None, r"\Sigma", P_WHITE),
            ("pl", r"P_{\text{Licht}}", P_CYAN), (None, r"\cdot", P_WHITE),
            ("fg", r"f_{g}", P_YELLOW), (None, r"\;[\mathrm{W}]", P_TEAL),
        ])
        self.play(FadeIn(row_l), Create(box_l), *light_heat(1.1), run_time=1.1)
        hold_for(self, self.NARRATION, "lights", during=light_heat)

        for key, color in (("pl", P_CYAN), ("fg", P_YELLOW)):
            ring = highlight_param(items_l, key, color=color)
            self.play(Create(ring), run_time=0.45)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            if key == "fg":
                # 💡 f_g live aus der Zeichnung: eine der zwei Leuchten schaltet
                # ab, der Anteil der laufenden Leuchten fällt von 1,0 auf 0,5.
                fg_tr = ValueTracker(1.0)
                fg_read = math_readout(
                    lambda: rf"f_{{g}} = {de_num(fg_tr.get_value(), 1)}",
                    lambda: fixtures.get_center() + DOWN * 0.95,
                    size=BODY_FONT_SIZE, color=P_YELLOW, edge="center",
                )
                self.add(fg_read)
                lit["r"] = False
                self.play(
                    fixture_r.animate.set_fill(P_YELLOW, opacity=0.0).set_stroke(opacity=0.4),
                    rays_r.animate.set_stroke(opacity=0.0),
                    fg_tr.animate.set_value(0.5), *light_heat(1.4),
                    run_time=1.4,
                )
                hold_for(self, self.NARRATION, key, during=light_heat)
                self.remove(fg_read)
                self.play(FadeOut(ring), run_time=0.25)
            else:
                hold_for(self, self.NARRATION, key, during=light_heat)
                self.play(FadeOut(ring), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=light_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 4 — Cumulative internal load

class Beat4_CumulativeLoad(Scene):
    NARRATION = [
        ("intro",
         "In building physics we sum every internal source into one cooling load, Q-dot internal.",
         "In der Bauphysik summieren wir alle internen Quellen zur Kühllast Q-Punkt intern."),
        ("sources",
         "People, plug loads and lighting are the three terms on the right-hand side.",
         "Personen, Steckerlasten und Beleuchtung sind die drei Terme auf der rechten Seite."),
        ("formula",
         "Q-dot i equals Q-dot Personen plus Q-dot Geräte plus Q-dot Licht, all in watts.",
         "Q-Punkt i gleich Q-Punkt Personen plus Q-Punkt Geräte plus Q-Punkt Licht, alles in Watt."),
        ("qi",
         "Q-dot i is the total internal cooling load the system must remove.",
         "Q-Punkt i ist die gesamte interne Kühllast, die das System abführen muss."),
        ("outro",
         "On a hot summer day the cooling system must remove this internal heat on top — even with blinds drawn.",
         "An einem heißen Sommertag muss die Kühlung diese interne Wärme zusätzlich abführen — selbst bei geschlossenen Jalousien."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Summe der internen Gewinne", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        def mini_person(center=ORIGIN):
            return person_glyph(center, color=P_ORANGE, scale=1.0)

        def mini_laptop(center=ORIGIN):
            c = np.array(center, dtype=float)
            return VGroup(
                Line(c + np.array([-0.32, -0.14, 0.0]), c + np.array([0.24, -0.14, 0.0]), color=P_CYAN, stroke_width=1.8),
                Line(c + np.array([0.24, -0.14, 0.0]), c + np.array([0.34, 0.22, 0.0]), color=P_CYAN, stroke_width=1.8),
            )

        def mini_lamp(center=ORIGIN):
            lamp = lamp_glyph(np.array(center, dtype=float) + UP * 0.45, drop=0.3)
            lamp["bulb"].set_fill(P_YELLOW, opacity=0.6)
            return lamp["group"]

        def source_card(icon, title_de, term, color, center):
            frame = RoundedRectangle(
                width=2.15, height=2.1, corner_radius=0.12,
                color=color, stroke_width=2.2, fill_color="#12151C", fill_opacity=0.92,
            ).move_to(center)
            label = Text(title_de, font_size=BODY_FONT_SIZE, color=color)
            label.next_to(frame.get_top(), DOWN, buff=0.12)
            term_t = math_label(term, size=BODY_FONT_SIZE, color=P_WHITE)
            term_t.next_to(frame.get_bottom(), UP, buff=0.14)
            icon.move_to((label.get_bottom() + term_t.get_top()) / 2)
            return VGroup(frame, label, icon, term_t)

        c_p = LEFT * 3.8 + UP * 0.85
        c_e = ORIGIN + UP * 0.85
        c_l = RIGHT * 3.8 + UP * 0.85
        card_p = source_card(mini_person(), "Personen", r"\dot{Q}_{\text{Pers}}", P_ORANGE, c_p)
        card_e = source_card(mini_laptop(), "Geräte", r"\dot{Q}_{\text{Geräte}}", P_CYAN, c_e)
        card_l = source_card(mini_lamp(), "Beleuchtung", r"\dot{Q}_{\text{Licht}}", P_YELLOW, c_l)
        plus_1 = Text("+", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to((c_p + c_e) / 2)
        plus_2 = Text("+", font_size=FORMULA_FONT_SIZE, color=P_WHITE).move_to((c_e + c_l) / 2)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sources"))
        self.play(
            LaggedStart(FadeIn(card_p), FadeIn(card_e), FadeIn(card_l), lag_ratio=0.2),
            run_time=1.3,
        )
        self.play(FadeIn(plus_1), FadeIn(plus_2), run_time=0.5)

        def card_heat(rt):
            spots = [card_p[2].get_center() + UP * 0.08, card_e[2].get_center() + UP * 0.05]
            cyc = max(1.0, rt / 1.3)
            return [ripples(spots, r_max=0.38, color=P_ORANGE, cycles=cyc),
                    ripples([card_l[2][2].get_center()], r_max=0.38, color=P_RED, down=True, cycles=cyc)]

        hold_for(self, self.NARRATION, "sources", during=card_heat)

        self.play(
            VGroup(card_p, card_e, card_l, plus_1, plus_2).animate.shift(UP * 0.35).scale(0.92),
            run_time=0.7,
        )

        row, box, items = math_panel([
            ("qi", r"\dot{Q}_{i}", P_CYAN), (None, "=", P_WHITE),
            ("pers", r"\dot{Q}_{\text{Pers}}", P_ORANGE), (None, "+", P_WHITE),
            ("ger", r"\dot{Q}_{\text{Geräte}}", P_CYAN), (None, "+", P_WHITE),
            ("licht", r"\dot{Q}_{\text{Licht}}", P_YELLOW), (None, r"\;[\mathrm{W}]", P_TEAL),
        ])
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(row), Create(box), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", during=card_heat)

        card_of = {"pers": card_p, "ger": card_e, "licht": card_l}
        for key, color in (("pers", P_ORANGE), ("ger", P_CYAN), ("licht", P_YELLOW), ("qi", P_CYAN)):
            ring = highlight_param(items, key, color=color)
            pulse = ([card_of[key][0].animate(rate_func=there_and_back).set_stroke(width=5.5)]
                     if key in card_of else [])
            self.play(Create(ring), *pulse, run_time=0.5)
            if key == "qi":
                caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "qi"))
                hold_for(self, self.NARRATION, "qi", during=card_heat)
            else:
                self.play(*card_heat(0.45), run_time=0.45)
            self.play(FadeOut(ring), run_time=0.22)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=card_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Disabled beats 5–7
# Sensible/latent visuals moved to Cooling/3 Beat4_SensibleVsLatent.
# HeatTrap + HVAC deferred — keep Beat8 Mitigation active.
if False:
    #region Beat 5 — Sensible vs latent heat

    class Beat5_SensibleVsLatent(Scene):
        NARRATION = [
            ("intro",
             "Internal gains split into two loads — sensible heat that raises air temperature, and latent moisture that changes humidity.",
             "Interne Gewinne teilen sich in zwei Lasten: fühlbar (Temperatur) und latent (Feuchte)."),
            ("sensible",
             "A thermometer tracks the sensible side — the air itself gets warmer.",
             "Ein Thermometer zeigt die fühlbare Seite — die Luft wird wärmer."),
            ("latent",
             "A moisture gauge tracks the latent side — water vapor accumulates in the room.",
             "Ein Feuchtezeiger zeigt die latente Seite — Wasserdampf sammelt sich im Raum."),
            ("rise",
             "Both gauges climb together as people and processes keep adding heat and moisture.",
             "Beide Anzeigen steigen, während Personen und Prozesse Wärme und Feuchte zuführen."),
            ("formula",
             "Total heat load Q-dot ges equals sensible load plus latent load, in watts.",
             "Gesamtwärmelast Q-Punkt-ges gleich sensible plus latente Last, in Watt."),
            ("qsens",
             "The sensible term is the heat you feel directly as a rising air temperature.",
             "Der fühlbare Term ist die Wärme, die man direkt als steigende Lufttemperatur spürt."),
            ("qlat",
             "The latent term is the energy hidden in the moisture that people and processes release.",
             "Der latente Term ist die Energie, die in der abgegebenen Feuchte steckt."),
            ("qges",
             "Added together they give the total load the cooling system has to remove.",
             "Zusammen ergeben sie die Gesamtlast, die die Kühlung abführen muss."),
        ]

        def construct(self):
            apply_scene_style(self)

            title = scene_title(TITLE_DE)
            self.add(title)
            subtitle = beat_subtitle("Sensible versus latente Wärme", title)
            self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

            caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
            self.play(FadeIn(caption), run_time=0.3)

            # Mid-screen split: panel center ~ y=+0.25 (clear of title / formula / caption).
            mid_y = 0.25
            lx, rx = -3.2, 3.2

            divider = Line(UP * (mid_y + 1.15), DOWN * (1.15 - mid_y), color=P_TEAL, stroke_width=2)

            # Headers sit high enough that the rising mercury column and its °C
            # readout (which climb to y≈1.25 with the tracker) never reach them.
            left_header = Text("Sensible Last", font_size=SUBTITLE_FONT_SIZE, color=P_RED)
            left_header.move_to(np.array([lx, mid_y + 1.75, 0]))
            left_sub = Text("Temperaturanstieg", font_size=BODY_FONT_SIZE, color=P_WHITE)
            left_sub.next_to(left_header, DOWN, buff=0.1)

            right_header = Text("Latente Feuchtigkeit", font_size=SUBTITLE_FONT_SIZE, color=P_CYAN)
            right_header.move_to(np.array([rx, mid_y + 1.75, 0]))
            right_sub = Text("Phasen- / Feuchtigkeitswechsel", font_size=BODY_FONT_SIZE, color=P_WHITE)
            right_sub.next_to(right_header, DOWN, buff=0.1)

            self.play(Create(divider), run_time=1.0)
            self.play(
                FadeIn(left_header), FadeIn(left_sub),
                FadeIn(right_header), FadeIn(right_sub),
                run_time=1.2,
            )
            hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 1.2)

            bulb = Circle(radius=0.38, color=P_RED, fill_color=P_DEEP_DARK, fill_opacity=1.0, stroke_width=3)
            bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
            tube = RoundedRectangle(corner_radius=0.12, height=2.1, width=0.34, color=P_RED, stroke_width=3)
            tube.move_to(np.array([lx, mid_y + 0.15, 0]))
            mercury_bulb = Circle(radius=0.35, color=P_RED, fill_color=P_RED, fill_opacity=0.9, stroke_width=0)
            mercury_bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
            temp_ticks = VGroup(*[
                Line([lx - 0.28, y, 0], [lx - 0.12, y, 0], color=P_TEAL, stroke_width=2)
                for y in np.linspace(mid_y - 0.55, mid_y + 0.95, 6)
            ])
            sensible_tag = Text("Misst Lufttemperatur", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
            sensible_tag.move_to(np.array([lx, mid_y - 1.55, 0]))

            temp_tracker = ValueTracker(0.2)
            column = always_redraw(lambda: Rectangle(
                width=0.22,
                height=max(0.05, temp_tracker.get_value()),
                color=P_RED,
                fill_color=P_RED,
                fill_opacity=0.9,
                stroke_width=0,
            ).move_to(np.array([lx, mid_y - 0.75 + temp_tracker.get_value() / 2, 0])))
            temp_label = always_redraw(lambda: Text(
                f"{int(21 + temp_tracker.get_value() * 5)}°C",
                font_size=BODY_FONT_SIZE,
                color=P_ORANGE,
            ).move_to(np.array([lx + 1.05, mid_y - 0.7 + temp_tracker.get_value(), 0])))

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sensible"))
            self.play(
                Create(bulb), Create(tube), Create(temp_ticks), FadeIn(mercury_bulb),
                run_time=1.6,
            )
            self.play(FadeIn(sensible_tag), FadeIn(column), FadeIn(temp_label), run_time=1.2)
            hold_for(self, self.NARRATION, "sensible", used=1.6 + 1.2 + 0.35)

            container = RoundedRectangle(corner_radius=0.1, height=2.1, width=1.15, color=P_CYAN, stroke_width=3)
            container.move_to(np.array([rx, mid_y + 0.05, 0]))
            moist_ticks = VGroup(*[
                Line([rx - 0.72, y, 0], [rx - 0.58, y, 0], color=P_TEAL, stroke_width=2)
                for y in np.linspace(mid_y - 0.85, mid_y + 0.85, 5)
            ])
            latent_tag = Text("Misst Wasserdampf", font_size=LABEL_FONT_SIZE, color=P_CYAN)
            latent_tag.move_to(np.array([rx, mid_y - 1.55, 0]))
            droplet_group = VGroup(*[
                Circle(radius=0.07, color=P_CYAN, fill_color=P_CYAN, fill_opacity=0.85, stroke_width=1)
                .move_to(np.array([rx + dx, mid_y + 1.05 + dy, 0]))
                for dx, dy in [(-0.3, 0.08), (-0.08, 0.35), (0.18, 0.15), (0.35, -0.08)]
            ])
            moist_tracker = ValueTracker(0.25)
            water_fill = always_redraw(lambda: Rectangle(
                width=1.02,
                height=max(0.05, moist_tracker.get_value() * 1.7),
                color=P_BLUE,
                fill_color=P_CYAN,
                fill_opacity=0.75,
                stroke_width=0,
            ).move_to(np.array([rx, mid_y - 0.95 + (moist_tracker.get_value() * 1.7) / 2, 0])))
            rh_label = always_redraw(lambda: Text(
                f"{int(30 + moist_tracker.get_value() * 60)}% r.F.",
                font_size=BODY_FONT_SIZE,
                color=P_CYAN,
            ).move_to(np.array([rx + 1.25, mid_y - 0.95 + moist_tracker.get_value() * 1.7, 0])))

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "latent"))
            self.play(Create(container), Create(moist_ticks), run_time=1.5)
            self.play(
                FadeIn(latent_tag), FadeIn(water_fill), FadeIn(rh_label), FadeIn(droplet_group),
                run_time=1.2,
            )
            hold_for(self, self.NARRATION, "latent", used=1.5 + 1.2 + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rise"))
            self.play(
                temp_tracker.animate.set_value(1.7),
                moist_tracker.animate.set_value(0.95),
                droplet_group.animate.shift(DOWN * 0.9).set_opacity(0.2),
                run_time=4.0,
            )
            hold_for(self, self.NARRATION, "rise", used=4.0 + 0.35)

            row, items = equation_row([
                ("qges", "Q̇_ges", P_WHITE), (None, "=", P_WHITE),
                ("qsens", "Q̇_sens", P_RED), (None, "+", P_WHITE),
                ("qlat", "Q̇_lat", P_CYAN), (None, "  [W]", P_TEAL),
            ])
            row, box = formula_panel(row)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
            self.play(Create(row), Create(box), run_time=1.2)
            hold_for(self, self.NARRATION, "formula", used=1.2 + 0.35)

            for key, color in (("qsens", P_RED), ("qlat", P_CYAN), ("qges", P_WHITE)):
                ring = highlight_param(items, key, color=color)
                self.play(Create(ring), run_time=0.35)
                caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
                hold_for(self, self.NARRATION, key, used=0.35 + 0.35)
                self.play(FadeOut(ring), run_time=0.2)

            self.play(FadeOut(caption), run_time=0.3)
            self.wait(0.5)

    #endregion


    #region Beat 6 — Insulated heat trap

    class Beat6_HeatTrap(Scene):
        NARRATION = [
            ("intro",
             "In a well-insulated room, internal gains have nowhere to escape.",
             "In einem gut gedämmten Raum haben interne Gewinne keinen Ausweg."),
            ("particles",
             "Heat particles drift toward the walls.",
             "Wärmeteilchen wandern zur Wand."),
            ("bounce",
             "Insulation throws them back — and each bounce leaves them hotter.",
             "Dämmung wirft sie zurück — und sie werden heißer."),
            ("trapped",
             "The room fills with trapped heat, so the cooling load rises.",
             "Der Raum füllt sich mit eingeschlossener Wärme — die Kühllast steigt."),
        ]

        def construct(self):
            apply_scene_style(self)

            title = scene_title(TITLE_DE)
            self.add(title)
            subtitle = beat_subtitle("Die isolierte Wärmefalle", title)
            self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

            caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
            self.play(FadeIn(caption), run_time=0.3)

            C_LIGHT = P_ORANGE
            C_SHARP = P_YELLOW
            C_HOT = P_RED
            C_VERY_HOT = P_RED

            # Mid-screen room (center ~ ORIGIN), shortened so floor stays above formula_panel.
            room_c = UP * 0.1
            inner_w, inner_h = 4.4, 2.2
            ins_t = 0.45
            r_dot = 0.085

            inner = Rectangle(
                width=inner_w, height=inner_h,
                color=P_WHITE, stroke_width=3.5, fill_opacity=0,
            ).move_to(room_c)
            outer = Rectangle(
                width=inner_w + 2 * ins_t, height=inner_h + 2 * ins_t,
                color=P_CYAN, stroke_width=2.5, fill_opacity=0,
            ).move_to(room_c)

            top_ins = Rectangle(
                width=inner_w + 2 * ins_t, height=ins_t,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + UP * ((inner_h + ins_t) / 2))
            bot_ins = Rectangle(
                width=inner_w + 2 * ins_t, height=ins_t,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + DOWN * ((inner_h + ins_t) / 2))
            left_ins = Rectangle(
                width=ins_t, height=inner_h,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + LEFT * ((inner_w + ins_t) / 2))
            right_ins = Rectangle(
                width=ins_t, height=inner_h,
                stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.25,
            ).move_to(room_c + RIGHT * ((inner_w + ins_t) / 2))
            insulation = VGroup(top_ins, bot_ins, left_ins, right_ins)

            floor_line = Line(
                room_c + LEFT * (inner_w / 2) + DOWN * (inner_h / 2),
                room_c + RIGHT * (inner_w / 2) + DOWN * (inner_h / 2),
                color=P_TEAL, stroke_width=4,
            )
            warm_fill = Rectangle(
                width=inner_w - 0.06, height=inner_h - 0.06,
                stroke_width=0, fill_color=C_LIGHT, fill_opacity=0.0,
            ).move_to(room_c)

            room_lbl = Text("Innenraum", font_size=LABEL_FONT_SIZE, color=P_TEAL)
            room_lbl.next_to(floor_line, UP, buff=0.1)

            ins_lbl = VGroup(
                Text("Dämmung", font_size=BODY_FONT_SIZE, color=P_ORANGE),
                Text("Isolationsschicht", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            ).arrange(DOWN, buff=0.08, aligned_edge=LEFT)
            ins_lbl.next_to(outer, RIGHT, buff=0.3)
            ins_lbl.set_y(float(room_c[1]) + 0.85)

            explain_out = Text("Wärmeteilchen wandern zur Wand", font_size=LABEL_FONT_SIZE, color=C_LIGHT)
            explain_back = Text(
                "Dämmung wirft sie zurück — sie werden heißer",
                font_size=LABEL_FONT_SIZE,
                color=C_SHARP,
            )
            explain_out.move_to(room_c + DOWN * 0.75)
            explain_back.move_to(room_c + DOWN * 0.75)

            trapped_lbl = Text("Eingeschlossene Wärme", font_size=SUBTITLE_FONT_SIZE, color=WHITE)
            trapped_lbl.move_to(room_c)

            rng = np.random.default_rng(7)
            n_particles = 18
            wall_x = inner_w / 2 - r_dot
            wall_y = inner_h / 2 - r_dot
            spawn_w, spawn_h = wall_x * 0.5, wall_y * 0.5

            starts = [
                room_c + np.array([
                    float(rng.uniform(-spawn_w, spawn_w)),
                    float(rng.uniform(-spawn_h, spawn_h)),
                    0.0,
                ])
                for _ in range(n_particles)
            ]

            def make_wall_hits(seed_shift=0):
                hits = []
                for i in range(n_particles):
                    side = (i + seed_shift) % 4
                    if side == 0:
                        hits.append(np.array([
                            float(room_c[0]) + wall_x,
                            float(room_c[1]) + float(rng.uniform(-wall_y, wall_y)),
                            0.0,
                        ]))
                    elif side == 1:
                        hits.append(np.array([
                            float(room_c[0]) - wall_x,
                            float(room_c[1]) + float(rng.uniform(-wall_y, wall_y)),
                            0.0,
                        ]))
                    elif side == 2:
                        hits.append(np.array([
                            float(room_c[0]) + float(rng.uniform(-wall_x, wall_x)),
                            float(room_c[1]) + wall_y,
                            0.0,
                        ]))
                    else:
                        hits.append(np.array([
                            float(room_c[0]) + float(rng.uniform(-wall_x, wall_x)),
                            float(room_c[1]) - wall_y,
                            0.0,
                        ]))
                return hits

            def make_interior(scale=0.45):
                return [
                    room_c + np.array([
                        float(rng.uniform(-wall_x * scale, wall_x * scale)),
                        float(rng.uniform(-wall_y * scale, wall_y * scale)),
                        0.0,
                    ])
                    for _ in range(n_particles)
                ]

            wall_1 = make_wall_hits(0)
            back_1 = make_interior(0.45)
            wall_2 = make_wall_hits(1)
            back_2 = make_interior(0.4)
            wall_3 = make_wall_hits(2)
            back_3 = make_interior(0.35)

            particles = VGroup(*[
                Dot(point=s, radius=r_dot, color=C_LIGHT, fill_opacity=1.0, stroke_width=0)
                for s in starts
            ])

            paths = VGroup()
            for i in range(n_particles):
                path = VMobject()
                path.set_points_as_corners([
                    starts[i], wall_1[i], back_1[i],
                    wall_2[i], back_2[i],
                    wall_3[i], back_3[i],
                ])
                paths.add(path)

            a_wall_1, a_wall_2, a_wall_3 = 1 / 6, 3 / 6, 5 / 6

            def heat_progress(mob, alpha):
                if alpha < a_wall_1:
                    color, fill_c, fill_op = C_LIGHT, C_LIGHT, 0.0
                elif alpha < a_wall_2:
                    t = (alpha - a_wall_1) / (a_wall_2 - a_wall_1)
                    color, fill_c, fill_op = C_SHARP, C_LIGHT, 0.12 + 0.16 * t
                elif alpha < a_wall_3:
                    t = (alpha - a_wall_2) / (a_wall_3 - a_wall_2)
                    color, fill_c, fill_op = C_HOT, C_SHARP, 0.28 + 0.17 * t
                else:
                    t = (alpha - a_wall_3) / max(1e-6, 1.0 - a_wall_3)
                    color, fill_c, fill_op = C_VERY_HOT, C_HOT, 0.45 + 0.2 * t
                for p in particles:
                    p.set_color(color)
                warm_fill.set_fill(fill_c, opacity=float(fill_op))

            self.add(warm_fill)
            self.play(Create(inner), Create(floor_line), FadeIn(room_lbl), run_time=1.1)
            self.play(Create(outer), FadeIn(insulation), FadeIn(ins_lbl), run_time=1.1)
            hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.1 + 1.1)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "particles"))
            self.play(
                LaggedStart(*[FadeIn(p, scale=0.5) for p in particles], lag_ratio=0.03),
                FadeIn(explain_out),
                run_time=1.0,
            )
            hold_for(self, self.NARRATION, "particles", used=1.0 + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bounce"))
            motion_rt = 8.5
            self.play(
                AnimationGroup(*[
                    MoveAlongPath(p, path, rate_func=linear)
                    for p, path in zip(particles, paths)
                ]),
                UpdateFromAlphaFunc(particles, heat_progress),
                Succession(
                    Wait(motion_rt * a_wall_1),
                    AnimationGroup(FadeOut(explain_out), FadeIn(explain_back), run_time=0.45),
                    Wait(motion_rt * (1.0 - a_wall_1) - 0.45),
                ),
                run_time=motion_rt,
            )
            hold_for(self, self.NARRATION, "bounce", used=motion_rt + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "trapped"))
            self.play(
                FadeOut(explain_back),
                FadeOut(room_lbl),
                FadeOut(particles),
                warm_fill.animate.set_fill(C_VERY_HOT, opacity=0.72),
                insulation.animate.set_fill(opacity=0.38),
                FadeIn(trapped_lbl),
                run_time=1.4,
            )
            hold_for(self, self.NARRATION, "trapped", used=1.4 + 0.35)

            self.play(FadeOut(caption), run_time=0.3)
            self.wait(0.5)

    #endregion


    #region Beat 7 — HVAC cooling demand

    class Beat7_HvacCooling(Scene):
        NARRATION = [
            ("intro",
             "Once heat is trapped, the HVAC system has to remove it mechanically.",
             "Ist die Wärme eingeschlossen, muss die HLK sie aktiv abführen."),
            ("return",
             "Return air extracts the warm load through the exhaust grille.",
             "Abluft saugt die warme Last über den Abluftauslass ab."),
            ("supply",
             "Supply air displaces that load with cool conditioned air.",
             "Zuluft verdrängt die Last mit kühler aufbereiteter Luft."),
            ("outro",
             "That active displacement is the HVAC cooling load driven by internal gains.",
             "Diese aktive Verdrängung ist die HLK-Kühllast durch interne Gewinne."),
        ]

        def construct(self):
            apply_scene_style(self)

            title = scene_title(TITLE_DE)
            self.add(title)
            subtitle = beat_subtitle("HLK-Kühllast", title)
            self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

            caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
            self.play(FadeIn(caption), run_time=0.3)

            C_HEAT = P_ORANGE
            C_HEAT_MID = P_YELLOW
            C_HEAT_HOT = P_RED
            C_COOL = P_CYAN
            C_COOL_DIM = P_TEAL
            C_WALL = P_WHITE

            # Mid-screen room (~ ORIGIN); shortened vs legacy to clear formula/caption.
            room_c = UP * 0.05
            room_w, room_h = 6.6, 2.55
            r_dot = 0.08
            room = Rectangle(
                width=room_w, height=room_h,
                color=C_WALL, stroke_width=3.5, fill_opacity=0,
            ).move_to(room_c)
            floor = Line(
                room.get_corner(DL), room.get_corner(DR),
                color=C_COOL_DIM, stroke_width=4,
            )

            warm_fill = Rectangle(
                width=room_w - 0.08, height=room_h - 0.08,
                stroke_width=0, fill_color=C_HEAT_HOT, fill_opacity=0.0,
            ).move_to(room_c)
            cool_fill = Rectangle(
                width=room_w - 0.08, height=room_h - 0.08,
                stroke_width=0, fill_color=C_COOL, fill_opacity=0.0,
            ).move_to(room_c)

            vent_w, vent_h = 1.2, 0.24
            supply = RoundedRectangle(
                width=vent_w, height=vent_h, corner_radius=0.06,
                color=C_COOL, stroke_width=2.5,
                fill_color=C_COOL, fill_opacity=0.2,
            ).move_to(room.get_top() + DOWN * 0.2 + LEFT * 2.0)
            ret = RoundedRectangle(
                width=vent_w, height=vent_h, corner_radius=0.06,
                color=C_HEAT_HOT, stroke_width=2.5,
                fill_color=C_HEAT_HOT, fill_opacity=0.2,
            ).move_to(room.get_top() + DOWN * 0.2 + RIGHT * 2.0)

            def vent_grille(vent, color):
                lines = VGroup()
                for t in (-0.35, 0.0, 0.35):
                    lines.add(Line(
                        vent.get_left() + RIGHT * 0.18 + UP * t * 0.07,
                        vent.get_right() + LEFT * 0.18 + UP * t * 0.07,
                        color=color, stroke_width=1.5, stroke_opacity=0.85,
                    ))
                return lines

            supply_grille = vent_grille(supply, C_COOL)
            ret_grille = vent_grille(ret, C_HEAT_MID)

            lbl_supply = VGroup(
                Text("Zuluft", font_size=BODY_FONT_SIZE, color=C_COOL),
                Text("kühl", font_size=LABEL_FONT_SIZE, color=C_COOL_DIM),
            ).arrange(DOWN, buff=0.04)
            lbl_supply.next_to(supply, UP, buff=0.12)

            lbl_return = VGroup(
                Text("Abluft", font_size=BODY_FONT_SIZE, color=C_HEAT_MID),
                Text("warm", font_size=LABEL_FONT_SIZE, color=C_HEAT),
            ).arrange(DOWN, buff=0.04)
            lbl_return.next_to(ret, UP, buff=0.12)

            step1 = Text("1  Interne Wärme staut sich im Raum", font_size=LABEL_FONT_SIZE, color=C_HEAT)
            step2 = Text("2  Abluft saugt warme Luft ab", font_size=LABEL_FONT_SIZE, color=C_HEAT_MID)
            step3 = Text("3  Zuluft bringt Kühlluft nach", font_size=LABEL_FONT_SIZE, color=C_COOL)
            for s in (step1, step2, step3):
                s.move_to([float(room_c[0]), float(room.get_bottom()[1]) + 0.28, 0])

            rng = np.random.default_rng(42)
            wall_x = room_w / 2 - r_dot
            wall_y = room_h / 2 - r_dot
            bounce_top = wall_y - 0.4

            def wall_point(side):
                if side == 0:
                    return room_c + np.array([wall_x, float(rng.uniform(-wall_y, bounce_top)), 0.0])
                if side == 1:
                    return room_c + np.array([-wall_x, float(rng.uniform(-wall_y, bounce_top)), 0.0])
                if side == 2:
                    return room_c + np.array([float(rng.uniform(-wall_x, wall_x)), bounce_top, 0.0])
                return room_c + np.array([float(rng.uniform(-wall_x, wall_x)), -wall_y, 0.0])

            def interior_point(scale=0.55):
                return room_c + np.array([
                    float(rng.uniform(-wall_x * scale, wall_x * scale)),
                    float(rng.uniform(-wall_y * scale, bounce_top * scale)),
                    0.0,
                ])

            def bounce_waypoints(start, n_bounces, seed_shift=0):
                pts = [np.array(start, dtype=float)]
                for k in range(n_bounces):
                    pts.append(wall_point((k + seed_shift) % 4))
                    pts.append(interior_point(0.4 + 0.08 * (k % 3)))
                return pts

            def path_from_points(pts):
                path = VMobject()
                path.set_points_as_corners(pts)
                return path

            def vent_into_room_points(vent, from_left, n_bounces, seed_shift):
                start = vent.get_bottom() + DOWN * 0.08 + np.array([
                    float(rng.uniform(-0.35, 0.35)), 0.0, 0.0,
                ])
                if from_left:
                    first = room_c + np.array([
                        float(rng.uniform(-wall_x * 0.5, wall_x * 0.2)),
                        float(rng.uniform(-0.15, bounce_top * 0.4)),
                        0.0,
                    ])
                else:
                    first = room_c + np.array([
                        float(rng.uniform(-wall_x * 0.2, wall_x * 0.5)),
                        float(rng.uniform(-0.15, bounce_top * 0.4)),
                        0.0,
                    ])
                bounce = bounce_waypoints(first, n_bounces=n_bounces, seed_shift=seed_shift)
                return [start, first] + bounce[1:]

            n_flow = 16
            n_bounces = 4
            flow_rt = 10.0

            cool_paths = VGroup()
            cool_point_lists = []
            for i in range(n_flow):
                pts = vent_into_room_points(supply, from_left=True, n_bounces=n_bounces, seed_shift=i + 1)
                cool_point_lists.append(pts)
                cool_paths.add(path_from_points(pts))

            cool_dots = VGroup(*[
                Dot(
                    point=pts[0],
                    radius=float(rng.uniform(0.055, 0.09)),
                    color=C_COOL,
                    fill_opacity=1.0,
                    stroke_width=0,
                )
                for pts in cool_point_lists
            ])

            heat_paths = VGroup()
            heat_point_lists = []
            for i in range(n_flow):
                forward = vent_into_room_points(ret, from_left=False, n_bounces=n_bounces, seed_shift=i)
                pts = [np.array(p, dtype=float) for p in reversed(forward)]
                heat_point_lists.append(pts)
                heat_paths.add(path_from_points(pts))

            heat_dots = VGroup(*[
                Dot(
                    point=pts[0],
                    radius=r_dot,
                    color=C_HEAT,
                    fill_opacity=1.0,
                    stroke_width=0,
                )
                for pts in heat_point_lists
            ])

            heat_lbl = Text("Thermische Last", font_size=BODY_FONT_SIZE, color=C_HEAT)
            heat_lbl.move_to(room_c + UP * 0.35)

            self.add(warm_fill, cool_fill)
            self.play(
                Create(room), Create(floor),
                FadeIn(supply), FadeIn(ret),
                FadeIn(supply_grille), FadeIn(ret_grille),
                run_time=1.1,
            )
            self.play(FadeIn(lbl_supply), FadeIn(lbl_return), run_time=0.5)
            self.play(
                FadeIn(step1),
                warm_fill.animate.set_fill(C_HEAT_HOT, opacity=0.22),
                LaggedStart(*[FadeIn(d, scale=0.5) for d in heat_dots], lag_ratio=0.03),
                FadeIn(heat_lbl),
                run_time=1.2,
            )
            hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.1 + 0.5 + 1.2)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "return"))
            self.play(
                FadeOut(step1),
                FadeIn(step2),
                FadeOut(heat_lbl),
                ret.animate.set_fill(C_HEAT_HOT, opacity=0.55),
                run_time=0.7,
            )

            def warm_progress(mob, alpha):
                warm_fill.set_fill(C_HEAT_HOT, opacity=0.22 - 0.18 * alpha)

            self.play(
                AnimationGroup(*[
                    MoveAlongPath(d, path, rate_func=linear)
                    for d, path in zip(heat_dots, heat_paths)
                ], lag_ratio=0.04),
                UpdateFromAlphaFunc(warm_fill, warm_progress),
                run_time=flow_rt,
            )
            self.play(FadeOut(heat_dots), run_time=0.4)
            hold_for(self, self.NARRATION, "return", used=0.7 + flow_rt + 0.4 + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "supply"))
            self.play(
                FadeOut(step2),
                FadeIn(step3),
                supply.animate.set_fill(C_COOL, opacity=0.6),
                run_time=0.7,
            )
            for d, path in zip(cool_dots, cool_paths):
                d.move_to(path.get_start())
            self.add(cool_dots)

            def cool_progress(mob, alpha):
                cool_fill.set_fill(C_COOL, opacity=0.08 + 0.32 * alpha)

            self.play(
                AnimationGroup(*[
                    MoveAlongPath(d, path, rate_func=linear)
                    for d, path in zip(cool_dots, cool_paths)
                ], lag_ratio=0.04),
                UpdateFromAlphaFunc(cool_fill, cool_progress),
                run_time=flow_rt,
            )
            hold_for(self, self.NARRATION, "supply", used=0.7 + flow_rt + 0.35)

            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
            self.play(
                FadeOut(step3),
                cool_dots.animate.set_opacity(0.5),
                run_time=0.8,
            )
            hold_for(self, self.NARRATION, "outro", used=0.8 + 0.35)

            self.play(FadeOut(caption), run_time=0.3)
            self.wait(0.5)

    #endregion


#endregion  # Disabled beats 5–7


#region Beat 8 — Mitigation & smart design

class Beat8_Mitigation(Scene):
    NARRATION = [
        ("intro",
         "Smart design cuts the load before the chiller must remove it.",
         "Intelligentes Design senkt die Last, bevor die Kälteanlage sie abführen muss."),
        ("high",
         "High plug and lighting loads first heat the office interior.",
         "Hohe Stecker- und Lichtlasten heizen zuerst den Büroraum."),
        ("dim",
         "Controls dim lights to thirty percent and plug loads to forty percent.",
         "Steuerung dimmt Licht auf dreißig und Steckerlasten auf vierzig Prozent."),
        ("outro",
         "Less internal heat gain means a smaller cooling load.",
         "Weniger interne Wärmegewinne bedeuten eine kleinere Kühllast."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Minderung & intelligentes Design", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        C_HEAT = P_ORANGE
        C_HEAT_HOT = P_RED
        C_COOL = P_CYAN
        C_COOL_DIM = P_TEAL
        C_YELLOW = P_YELLOW
        C_WALL = "#1A1E28"

        # Room + control strip as one mid-screen group — Physical Fundamentals room,
        # seated person at a desk, laptop and pendant lamp.
        room_w, room_h = 7.0, 2.25
        rs = room_section(np.array([0.0, 0.35, 0.0]), w=room_w, h=room_h, slab=0.2)
        room = rs["shell"]
        warm_fill = rs["air"]
        y_f = rs["y_f"]
        seated = seated_person_glyph(np.array([-0.95, y_f, 0.0]), color=P_ORANGE, scale=2.6)
        knee = seated["figure"][7].get_end()
        desk_y = knee[1] - 0.06
        desk = VGroup(
            Line(np.array([knee[0] + 0.05, desk_y, 0.0]), np.array([knee[0] + 2.3, desk_y, 0.0]), color=P_WHITE, stroke_width=1.5),
            Line(np.array([knee[0] + 2.2, desk_y, 0.0]), np.array([knee[0] + 2.2, y_f, 0.0]), color=P_WHITE, stroke_width=1.5),
        )
        laptop_screen = Line(np.array([knee[0] + 1.35, desk_y + 0.02, 0.0]), np.array([knee[0] + 1.45, desk_y + 0.45, 0.0]),
                             color=C_HEAT_HOT, stroke_width=2.0)
        laptop = VGroup(Line(np.array([knee[0] + 0.8, desk_y + 0.02, 0.0]), np.array([knee[0] + 1.35, desk_y + 0.02, 0.0]),
                             color=C_HEAT_HOT, stroke_width=2.0), laptop_screen)
        laptop_lbl = Text("Gerät", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)
        laptop_lbl.next_to(laptop_screen, UP, buff=0.2)
        lamp = lamp_glyph(np.array([2.55, rs["y_c"], 0.0]), drop=0.42)
        fixture = lamp["bulb"].set_fill(C_YELLOW, opacity=0.8)
        bulb_c = fixture.get_center()
        beam = VGroup(*[Line(bulb_c, np.array([bulb_c[0] + dx, y_f + 0.02, 0.0]), color=C_YELLOW, stroke_width=1.8,
                             stroke_opacity=0.6) for dx in (-0.65, -0.22, 0.22, 0.65)])
        light_lbl = Text("Beleuchtung", font_size=LABEL_FONT_SIZE, color=C_YELLOW)
        light_lbl.next_to(lamp["group"], LEFT, buff=0.3).shift(UP * 0.12)
        window_group = rs["glass"]
        chair = seated["group"]
        cord = lamp["group"]

        _slider_half = 0.52
        light_label = Text("Licht", font_size=LABEL_FONT_SIZE, color=C_YELLOW)
        light_track = Line(LEFT * _slider_half, RIGHT * _slider_half, color=P_WHITE, stroke_width=5)
        light_fill = Line(LEFT * _slider_half, RIGHT * _slider_half, color=C_YELLOW, stroke_width=5)
        light_knob = Dot(color=C_YELLOW, radius=0.1)
        light_pct = Text("100%", font_size=LABEL_FONT_SIZE, color=C_YELLOW)

        plug_label = Text("Stecker", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)
        plug_track = Line(LEFT * _slider_half, RIGHT * _slider_half, color=P_WHITE, stroke_width=5)
        plug_fill = Line(LEFT * _slider_half, RIGHT * _slider_half, color=C_HEAT_HOT, stroke_width=5)
        plug_knob = Dot(color=C_HEAT_HOT, radius=0.1)
        plug_pct = Text("100%", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT)

        # Control strip sits just under the room, still above formula_panel (~ y=-1.2).
        ctrl_panel = RoundedRectangle(
            width=room_w, height=0.9, corner_radius=0.1,
            color=C_COOL_DIM, stroke_width=2,
            fill_color="#12151C", fill_opacity=0.95,
        ).next_to(room, DOWN, buff=0.18)

        ctrl_title = Text("Smarte Laststeuerung", font_size=LABEL_FONT_SIZE, color=C_COOL)
        ctrl_title.move_to(ctrl_panel.get_top() + DOWN * 0.18)

        light_track.move_to(ctrl_panel.get_center() + LEFT * 2.0 + DOWN * 0.12)
        light_fill.put_start_and_end_on(light_track.get_start(), light_track.get_end())
        light_knob.move_to(light_track.get_end())
        light_label.next_to(light_track, LEFT, buff=0.2)
        light_pct.next_to(light_track, RIGHT, buff=0.26)

        plug_track.move_to(ctrl_panel.get_center() + RIGHT * 1.9 + DOWN * 0.12)
        plug_fill.put_start_and_end_on(plug_track.get_start(), plug_track.get_end())
        plug_knob.move_to(plug_track.get_end())
        plug_label.next_to(plug_track, LEFT, buff=0.2)
        plug_pct.next_to(plug_track, RIGHT, buff=0.26)

        self.add(warm_fill)
        self.play(FadeIn(room), run_time=1.0)
        self.play(
            FadeIn(window_group),
            FadeIn(chair),
            FadeIn(desk),
            run_time=1.0,
        )
        self.play(
            FadeIn(laptop),
            FadeIn(laptop_lbl),
            FadeIn(cord),
            FadeIn(fixture),
            FadeIn(beam),
            FadeIn(light_lbl),
            run_time=0.9,
        )

        light_on = ValueTracker(1.0)
        plug_on = ValueTracker(1.0)
        heat_spots = {"laptop": laptop_screen.get_center(), "person": seated["chest"]}

        def room_heat(rt):
            cyc = max(1.0, rt / 1.3)
            anims = [ripples([heat_spots["person"]], r_max=0.55, color=P_ORANGE, cycles=cyc),
                     ripples([heat_spots["laptop"]], r_max=0.3 + 0.5 * plug_on.get_value(), color=C_HEAT_HOT, cycles=cyc),
                     ripples([bulb_c], r_max=0.25 + 0.6 * light_on.get_value(), color=C_HEAT_HOT, down=True, cycles=cyc)]
            if light_on.get_value() > 0.5:
                anims.append(pulse_flashes([[bulb_c, r.get_end()] for r in beam], C_YELLOW,
                                           repeats=max(1, int(rt / 1.6)), width=3.5))
            return anims

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "high"))
        self.play(warm_fill.animate.set_fill(C_HEAT_HOT, opacity=0.2), *room_heat(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "high", during=room_heat)

        self.play(
            FadeIn(ctrl_panel),
            FadeIn(ctrl_title),
            FadeIn(light_label), Create(light_track), Create(light_fill),
            FadeIn(light_knob), FadeIn(light_pct),
            FadeIn(plug_label), Create(plug_track), Create(plug_fill),
            FadeIn(plug_knob), FadeIn(plug_pct),
            run_time=1.1,
        )

        light_t, plug_t = 0.30, 0.40
        light_target = light_track.point_from_proportion(light_t)
        plug_target = plug_track.point_from_proportion(plug_t)
        light_share = ValueTracker(100.0)
        plug_share = ValueTracker(100.0)
        light_pct_anchor = light_pct.get_left()
        plug_pct_anchor = plug_pct.get_left()
        light_pct_live = always_redraw(lambda: Text(
            f"{light_share.get_value():.0f}%", font_size=LABEL_FONT_SIZE, color=C_YELLOW,
        ).move_to(light_pct_anchor, aligned_edge=LEFT))
        plug_pct_live = always_redraw(lambda: Text(
            f"{plug_share.get_value():.0f}%", font_size=LABEL_FONT_SIZE, color=C_HEAT_HOT,
        ).move_to(plug_pct_anchor, aligned_edge=LEFT))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "dim"))
        self.remove(light_pct, plug_pct)
        self.add(light_pct_live, plug_pct_live)
        self.play(
            light_knob.animate.move_to(light_target),
            plug_knob.animate.move_to(plug_target),
            UpdateFromAlphaFunc(
                light_fill,
                lambda m, a: m.put_start_and_end_on(
                    light_track.get_start(),
                    light_track.point_from_proportion(1.0 - a * (1.0 - light_t)),
                ),
            ),
            UpdateFromAlphaFunc(
                plug_fill,
                lambda m, a: m.put_start_and_end_on(
                    plug_track.get_start(),
                    plug_track.point_from_proportion(1.0 - a * (1.0 - plug_t)),
                ),
            ),
            light_share.animate.set_value(light_t * 100),
            plug_share.animate.set_value(plug_t * 100),
            beam.animate.set_stroke(opacity=0.18),
            fixture.animate.set_fill(opacity=0.24),
            laptop.animate.set_color(C_COOL_DIM),
            warm_fill.animate.set_fill(C_COOL, opacity=0.08),
            light_on.animate.set_value(light_t), plug_on.animate.set_value(plug_t),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "dim", during=room_heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=room_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion
