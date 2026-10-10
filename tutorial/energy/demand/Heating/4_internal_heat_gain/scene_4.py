"""🔥 Heating Module 4 — Interne Wärmegewinne.

Migrated from ``merged_scenes.py`` onto the generate-manim-tutorial template:
fixed type scale, typeset ``math_panel`` formulas with units, German ``caption_bar``
subtitles, and ``hold_for`` timing.

Every stage is drawn with the Physical Fundamentals glyphs (house and room
sections, person, lamp, moon, thermometer) and its heat keeps moving while a
subtitle is read — ripples for long-wave heat, particles for air, electricity
and envelope losses, pulses for light.
https://docs.manim.community/en/stable/reference/manim.animation.updaters.update.UpdateFromAlphaFunc.html
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
    PASTEL_PINK,
    PASTEL_WHITE, PASTEL_CYAN, PASTEL_TEAL, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_RED, PASTEL_BLUE, PASTEL_GREEN,
    highlight_param, dim_arrow, math_label, math_panel, de_num, place_math,
    caption_bar, swap_caption, hold_for, subtitle_text,
    watt_anchor, set_vo_language,
    house_section, room_section, person_glyph, seated_person_glyph, lamp_glyph, moon_glyph, thermometer_glyph,
    ripples, smooth_path, flow_animation, pulse_flashes,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")

# 🏔️ Persistent topic title — Write once on Beat1, self.add() on later beats.
TITLE_DE = "Modul 4: Interne Wärmegewinne"

COLOR_PEOPLE = PASTEL_PINK
COLOR_EQUIP = PASTEL_CYAN
COLOR_LIGHT = PASTEL_YELLOW
COLOR_HEAT = PASTEL_ORANGE

# Default formula_panel edge_buff is 1.7 — sit a bit lower above the caption.
FORMULA_EDGE_BUFF = 1.2

# Example office occupancy (DIN V 18599-10 style point loads) — one source per beat.
PHI_P_W = 80.0
PHI_E_LAPTOP_W, PHI_E_DEVICE_W = 60.0, 90.0
PHI_E_W = PHI_E_LAPTOP_W + PHI_E_DEVICE_W
PHI_L_W = 60.0
PHI_INT_W = PHI_P_W + PHI_E_W + PHI_L_W
ROOM_W_M, ROOM_D_M = 5.0, 4.0


#region Shared

#region Physical Fundamentals stage
def _cycles(run_time: float, period: float = 1.3) -> int:
    """🔁 Whole loops for a looping animation, so particles continue seamlessly into the next play."""
    return max(1, round(run_time / period))


def _laptop(hinge, *, color=PASTEL_WHITE, w: float = 0.5, h: float = 0.42):
    """💻 Laptop in side elevation — base on the desk, screen tilted back at ``hinge``."""
    hinge = np.array(hinge, dtype=float)
    return VGroup(
        Line(hinge + LEFT * w, hinge, color=color, stroke_width=1.8),
        Line(hinge, hinge + np.array([0.09, h, 0.0]), color=color, stroke_width=1.8),
    )


def _desk(x0: float, x1: float, top: float, floor: float, legs):
    """🪑 Desk in side elevation — top board from ``x0`` to ``x1`` standing on legs at the ``legs`` x positions."""
    return VGroup(
        Line(np.array([x0, top, 0.0]), np.array([x1, top, 0.0]), color=PASTEL_WHITE, stroke_width=1.5),
        *[Line(np.array([x, top, 0.0]), np.array([x, floor, 0.0]), color=PASTEL_WHITE, stroke_width=1.5)
          for x in legs],
    )


def _plume(top, ceiling_y: float, *, spread: float = 0.08):
    """♨️ Two warm-air tracks rising off a head or a warm patch up toward the ceiling."""
    top = np.array(top, dtype=float)
    return [smooth_path([top + UP * 0.06 + RIGHT * dx, top + UP * 0.4 + RIGHT * (0.12 + dx),
                         top + UP * 0.8 + LEFT * (0.05 - dx), np.array([top[0] + 0.1 + dx, ceiling_y, 0.0])])
            for dx in (-spread, spread)]


def _light_rays(bulb, floor_y: float, dxs, *, color=COLOR_LIGHT):
    """💡 Straight light rays from a bulb down to the lit floor."""
    bulb = np.array(bulb, dtype=float)
    return VGroup(*[Line(bulb, np.array([bulb[0] + dx, floor_y, 0.0]), color=color, stroke_width=1.8,
                         stroke_opacity=0.6) for dx in dxs])


def _ray_paths(rays):
    """⚡ Corner lists of each ray for ``pulse_flashes``."""
    return [[r.get_start(), r.get_end()] for r in rays]


def _twinkle(stars, run_time: float):
    """✨ Stars dim and brighten at their own pace, starting and ending fully lit."""
    rates = [1 + (i * 2) % 3 for i in range(len(stars))]
    turns = max(1, round(run_time / 2.4))

    def update(group, alpha):
        for star, rate in zip(group, rates):
            star.set_fill(opacity=0.9 - 0.6 * np.sin(PI * alpha * rate * turns) ** 2)

    return UpdateFromAlphaFunc(stars, update)
#endregion


#region Typeset symbols
def _phi(sub: str) -> str:
    """🔣 LaTeX source of a heat flux Φ with an upright index (p, e, l, int)."""
    return rf"\Phi_{{\mathrm{{{sub}}}}}"


def _watts(sub: str, tracker) -> str:
    """⚡ Live ``Φ_x = … W`` source read from a tracker."""
    return rf"{_phi(sub)} = {de_num(tracker.get_value())}\,\mathrm{{W}}"


def _sum_parts(keys, *, lead=None):
    """🧮 Panel parts ``[lead =] Φ_p + Φ_e + … [W]`` for ``math_panel``."""
    colors = {"p": COLOR_PEOPLE, "e": COLOR_EQUIP, "l": COLOR_LIGHT, "int": PASTEL_WHITE}
    parts = [(f"phi_{lead}", _phi(lead), colors[lead]), (None, "=", PASTEL_WHITE)] if lead else []
    for i, k in enumerate(keys):
        if i:
            parts.append((None, "+", PASTEL_WHITE))
        parts.append((f"phi_{k}", _phi(k), colors[k]))
    parts.append((None, r"\;[\mathrm{W}]", PASTEL_WHITE))
    return parts
#endregion

#region Live readouts
_READOUT_CACHE: dict = {}


def _live(fn, at, *, size=None, color=PASTEL_WHITE, edge="left"):
    """📟 ``math_readout`` that reuses the typeset box while ``fn()`` returns the same source.

    https://docs.manim.community/en/stable/reference/manim.animation.updaters.mobject_update_utils.html
    """
    def build():
        key = (fn(), size, color)
        if key not in _READOUT_CACHE:
            _READOUT_CACHE[key] = math_label(key[0], size=size, color=color)
        return place_math(_READOUT_CACHE[key].copy(), np.array(at() if callable(at) else at, dtype=float), edge)

    return always_redraw(build)
#endregion



def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in the other Heating
    modules. Internal gains are tabulated in DIN V 18599-10, so every beat that
    does not already print that standard in its diagram footnotes it here.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=PASTEL_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Beat1 — Winter house & free internal gains
#region Beat1 stage
B1_HOUSE_C = np.array([0.0, -0.79, 0.0])
B1_HOUSE_SCALE = 1.3
B1_MOON = np.array([-5.3, 1.75, 0.0])
B1_THERMO = np.array([-4.6, -2.15, 0.0])
B1_STARS = ((-6.3, 2.25), (-4.3, 2.3), (-3.4, 1.45), (-6.2, 0.8), (-4.4, 0.75),
            (3.5, 2.3), (5.0, 1.9), (6.2, 2.3), (4.2, 1.15), (5.9, 0.55))
B1_DIN_AT = np.array([4.6, -2.1, 0.0])


def _house_air(house):
    """🌫️ Interior air of the section house, tinted as the rooms warm up."""
    t = 0.1
    air = Polygon(
        house["bottom_left"] + RIGHT * t + UP * 0.02, house["bottom_right"] + LEFT * t + UP * 0.02,
        house["top_right"] + LEFT * t + DOWN * 0.06, house["roof_peak"] + DOWN * 0.18,
        house["top_left"] + RIGHT * t + DOWN * 0.06,
        stroke_width=0, fill_color=COLOR_HEAT, fill_opacity=0.0,
    )
    air.set_z_index(-1)
    return air


def _envelope_losses(house):
    """🧱 Heat tracks leaving through windows, walls and both roof slopes — warm inside, cold outside."""
    x_l, x_r = house["bottom_left"][0], house["bottom_right"][0]
    tracks = [(np.array([x_l + 0.45, w["center"][1], 0.0]), LEFT) for w in house["windows"]]
    tracks += [(np.array([x_r - 0.45, y, 0.0]), RIGHT) for y in (-1.75, -0.45)]
    peak = house["roof_peak"]
    for eave, side in ((house["top_left"], 1.0), (house["top_right"], -1.0)):
        along = peak - eave
        normal = np.array([-along[1], along[0], 0.0]) * side
        tracks.append((eave + 0.45 * along - normal / np.linalg.norm(normal) * 0.3, normal / np.linalg.norm(normal)))
    return [smooth_path([a, a + d * 0.55 + UP * 0.03, a + d * 1.4]) for a, d in tracks]


def _house_gains(house):
    """💡 Person, desk laptop and pendant lamp in the house — the Physical Fundamentals glyphs, labelled."""
    floor_y, mid_y = house["bottom_left"][1], house["level_1"].get_center()[1]
    person = person_glyph(ORIGIN, color=COLOR_PEOPLE, scale=1.6)
    person.move_to(np.array([-1.3, floor_y + 0.02 + person.height / 2, 0.0]))
    desk_y = mid_y + 0.5
    desk = _desk(-0.6, 0.7, desk_y, mid_y, legs=(-0.5, 0.6))
    laptop = _laptop(np.array([0.25, desk_y + 0.02, 0.0]), color=COLOR_EQUIP)
    lamp = lamp_glyph(np.array([1.0, mid_y, 0.0]), drop=0.12)
    bulb = lamp["bulb"].get_center()
    rays = _light_rays(bulb, floor_y + 0.02, (-0.5, -0.17, 0.17, 0.5))
    labels = VGroup(
        Text("Personen", font_size=LABEL_FONT_SIZE, color=COLOR_PEOPLE).next_to(person, RIGHT, buff=0.45),
        Text("Geräte", font_size=LABEL_FONT_SIZE, color=COLOR_EQUIP).next_to(desk, RIGHT, buff=0.3).set_y(desk_y + 0.2),
        Text("Licht", font_size=LABEL_FONT_SIZE, color=COLOR_LIGHT).next_to(lamp["group"], RIGHT, buff=0.22),
    )
    return {
        "sources": VGroup(person, VGroup(desk, laptop), lamp["group"]), "labels": labels, "lamp": lamp, "rays": rays,
        "warm": [person.get_center() + UP * 0.12, laptop[1].get_center()], "bulb": bulb,
    }


def _house_heat(gains, run_time: float):
    """🌡️ Heat from the gains — ripples off person and laptop, down from the lamp, light pulsing to the floor."""
    cyc = _cycles(run_time)
    return [ripples(gains["warm"], r_max=0.35, color=COLOR_HEAT, cycles=cyc),
            ripples([gains["bulb"]], r_max=0.42, color=PASTEL_RED, down=True, cycles=cyc),
            pulse_flashes(_ray_paths(gains["rays"]), COLOR_LIGHT, repeats=max(1, round(run_time / 1.6)), width=3.5)]
#endregion


class Beat1_WinterInterneGewinne(Scene):
    NARRATION = [
        ("winter",
         "Outside it is cold — the envelope is losing heat.",
         "Draußen ist es kalt — die Hülle verliert Wärme."),
        ("inside",
         "Inside, people, devices, and lights already make free heat.",
         "Drinnen erzeugen Personen, Geräte und Licht schon freie Wärme."),
        ("din",
         "Those internal gains cut the annual heating demand in DIN V 18599. A design heating load after DIN EN 12831 usually ignores them.",
         "Diese internen Gewinne senken den Jahres-Heizwärmebedarf nach DIN V 18599. Eine Auslegungsheizlast nach DIN EN 12831 lässt sie meist weg."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Kalte Außenluft vs. interne Quellen", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "winter"))
        self.play(FadeIn(caption), run_time=0.3)

        house = house_section(B1_HOUSE_C, scale=B1_HOUSE_SCALE)
        air = _house_air(house)
        moon = moon_glyph(B1_MOON, color=PASTEL_WHITE).scale(1.8)
        stars = VGroup(*[Dot(np.array([x, y, 0.0]), radius=0.03, color=PASTEL_WHITE, fill_opacity=0.9)
                         for x, y in B1_STARS])
        therm = thermometer_glyph(B1_THERMO, height=1.4, color=PASTEL_BLUE, level=0.6)
        losses = _envelope_losses(house)

        def loss_flow(rt):
            return [flow_animation([(losses, PASTEL_RED, PASTEL_BLUE)], waves=4, radius=0.065,
                                   cycles=_cycles(rt, 1.6)), _twinkle(stars, rt)]

        self.add(air)
        self.play(FadeIn(house["group"]), FadeIn(moon, shift=DOWN * 0.15), FadeIn(stars, lag_ratio=0.1),
                  FadeIn(therm["group"]), FadeIn(therm["column"]), run_time=1.4)
        self.play(therm["level"].animate.set_value(0.15), air.animate.set_fill(COLOR_HEAT, opacity=0.08),
                  *loss_flow(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "winter", used=1.4 + 2.0, during=loss_flow)

        gains = _house_gains(house)

        def all_heat(rt):
            return [*loss_flow(rt), *_house_heat(gains, rt)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "inside"))
        self.play(LaggedStart(*[FadeIn(m) for m in gains["sources"]], lag_ratio=0.3),
                  LaggedStart(*[FadeIn(m) for m in gains["labels"]], lag_ratio=0.3), *loss_flow(1.6), run_time=1.6)
        self.play(gains["lamp"]["bulb"].animate.set_fill(COLOR_LIGHT, opacity=0.8),
                  LaggedStart(*[Create(r) for r in gains["rays"]], lag_ratio=0.1),
                  air.animate.set_fill(COLOR_HEAT, opacity=0.14), *loss_flow(1.6),
                  ripples(gains["warm"], r_max=0.35, color=COLOR_HEAT, cycles=1), run_time=1.6)
        hold_for(self, self.NARRATION, "inside", during=all_heat)

        din_tag = Text("DIN V 18599", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to(B1_DIN_AT)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        self.play(FadeIn(din_tag, shift=LEFT * 0.2), *all_heat(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "din", during=all_heat)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 — Persons Φ_p
B2_ROOM_C = np.array([-1.25, -0.05, 0.0])


class Beat2_PersonenPhiP(Scene):
    NARRATION = [
        ("person",
         "One seated person already sheds sensible heat into the room.",
         "Eine sitzende Person gibt schon fühlbare Wärme in den Raum ab."),
        ("power",
         "That load is about eighty watts — roughly a bright bulb.",
         "Diese Last liegt bei etwa achtzig Watt — etwa wie eine helle Birne."),
        ("phi_p",
         "We call the person heat flux Phi p — unit watt.",
         "Wir nennen den Personenwärmestrom Phi-p — Einheit Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Personenabwärme", title)
        din = _din_ref("DIN V 18599-10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "person"))
        self.play(FadeIn(caption), run_time=0.3)

        room = room_section(B2_ROOM_C, w=5.6, h=3.3)
        seated = seated_person_glyph(np.array([room["x_l"] + 1.9, room["y_f"], 0.0]), color=COLOR_PEOPLE, scale=3.6,
                                     chair_color=PASTEL_WHITE)
        chest = seated["chest"]
        plume = _plume(seated["head"].get_top(), room["y_c"] - 0.08)

        def body_heat(rt):
            cyc = _cycles(rt, 1.4)
            return [ripples([chest], r_max=0.7, color=COLOR_HEAT, cycles=cyc),
                    flow_animation([(plume, COLOR_HEAT, PASTEL_RED)], waves=4, radius=0.055, cycles=cyc)]

        self.add(room["air"])
        self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), FadeIn(seated["group"]), run_time=1.3)
        self.play(*body_heat(1.4), room["air"].animate.set_fill(COLOR_HEAT, opacity=0.05), run_time=1.4)
        hold_for(self, self.NARRATION, "person", used=1.3 + 1.4, during=body_heat)

        phi_p = ValueTracker(0.0)
        phi_p_live = _live(
            lambda: _watts("p", phi_p),
            np.array([room["x_l"] + 3.3, room["y_c"] - 0.85, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_PEOPLE,
        )
        anchor = watt_anchor(PHI_P_W, compare="bulb", title="eine sitzende Person", color=PASTEL_YELLOW)
        anchor.next_to(room["wall_r"], RIGHT, buff=0.45).set_y(room["center"][1])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "power"))
        self.add(phi_p_live)
        self.play(phi_p.animate.set_value(PHI_P_W), room["air"].animate.set_fill(COLOR_HEAT, opacity=0.1),
                  *body_heat(1.4), run_time=1.4)
        self.play(FadeIn(anchor, shift=LEFT * 0.25), *body_heat(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "power", during=body_heat)

        row, box, items = math_panel(_sum_parts(["p"]), edge_buff=FORMULA_EDGE_BUFF)
        ring = highlight_param(items, "phi_p", color=COLOR_PEOPLE)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "phi_p"))
        self.play(Create(row), Create(box), *body_heat(1.0), run_time=1.0)
        self.play(Create(ring), *body_heat(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "phi_p", during=body_heat)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 — Equipment Φ_e
B3_ROOM_C = np.array([-0.9, 0.0, 0.0])


def _rack(center, *, w: float = 0.7, h: float = 1.05):
    """🗄️ Small equipment tower — casing, four drive bays and their status LEDs."""
    box = Rectangle(width=w, height=h, color=PASTEL_WHITE, stroke_width=1.6).move_to(center)
    bays = VGroup(*[Line(box.get_top() + DOWN * (0.25 + 0.2 * i) + LEFT * (w / 2 - 0.1),
                         box.get_top() + DOWN * (0.25 + 0.2 * i) + RIGHT * (w / 2 - 0.1),
                         color=PASTEL_WHITE, stroke_width=1.0) for i in range(4)])
    leds = VGroup(*[Dot(bay.get_start() + RIGHT * 0.06 + UP * 0.07, radius=0.025, color=COLOR_EQUIP) for bay in bays])
    return VGroup(box, bays, leds)


class Beat3_GeraetePhiE(Scene):
    NARRATION = [
        ("desk",
         "Plug loads join in — laptop and rack equipment warm the room.",
         "Steckdosenlasten kommen dazu — Laptop und Geräte wärmen den Raum."),
        ("spark",
         "Electricity becomes heat the moment it leaves the wall socket.",
         "Strom wird Wärme, sobald er die Steckdose verlässt."),
        ("phi_e",
         "That equipment heat flux is Phi e — again in watts.",
         "Dieser Gerätewärmestrom heißt Phi-e — wieder in Watt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Geräteabwärme", title)
        din = _din_ref("DIN V 18599-10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "desk"))
        self.play(FadeIn(caption), run_time=0.3)

        room = room_section(B3_ROOM_C, w=7.0, h=3.2)
        y_f = room["y_f"]
        seated = seated_person_glyph(np.array([room["x_l"] + 1.4, y_f, 0.0]), color=COLOR_PEOPLE, scale=3.2,
                                     chair_color=PASTEL_WHITE)
        elbow = seated["figure"][7].get_end()
        desk_y, x0 = elbow[1] - 0.1, elbow[0] + 0.1
        desk = _desk(x0, x0 + 2.9, desk_y, y_f, legs=(x0 + 1.25, x0 + 2.8))
        hinge = np.array([x0 + 1.0, desk_y + 0.02, 0.0])
        laptop = _laptop(hinge)
        server = _rack(np.array([x0 + 2.3, desk_y + 0.525 + 0.01, 0.0]))
        server_box = server[0]
        sock = np.array([room["x_r"] - 0.1, y_f + 0.4, 0.0])
        socket = VGroup(
            Square(side_length=0.2, color=PASTEL_WHITE, stroke_width=1.6).move_to(sock),
            Line(sock + np.array([-0.04, -0.05, 0.0]), sock + np.array([-0.04, 0.05, 0.0]), color=PASTEL_WHITE,
                 stroke_width=1.2),
            Line(sock + np.array([0.04, -0.05, 0.0]), sock + np.array([0.04, 0.05, 0.0]), color=PASTEL_WHITE,
                 stroke_width=1.2),
        )
        out = sock + LEFT * 0.1
        cables = [
            smooth_path([out, out + np.array([-0.6, -0.27, 0.0]), np.array([x0 + 3.5, y_f + 0.1, 0.0]),
                         np.array([x0 + 2.65, y_f + 0.45, 0.0]), server_box.get_bottom() + RIGHT * 0.18]),
            smooth_path([out, out + np.array([-0.6, -0.33, 0.0]), np.array([x0 + 1.6, y_f + 0.05, 0.0]),
                         np.array([x0 + 1.1, y_f + 0.45, 0.0]), hinge + DOWN * 0.02]),
        ]
        cable_lines = VGroup(*[c.copy().set_stroke(COLOR_EQUIP, width=1.3, opacity=0.45) for c in cables])
        warm = [laptop[1].get_center(), server_box.get_top() + UP * 0.04]

        def warm_heat(rt):
            cyc = _cycles(rt)
            return [ripples([seated["chest"]], r_max=0.55, color=COLOR_HEAT, cycles=cyc),
                    ripples(warm, r_max=0.6, color=COLOR_HEAT, cycles=cyc)]

        def device_heat(rt):
            return [*warm_heat(rt), flow_animation([(cables, COLOR_EQUIP, COLOR_HEAT)], waves=4, radius=0.045,
                                                   cycles=_cycles(rt), streak=False)]

        self.add(room["air"])
        self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), FadeIn(seated["group"]), Create(desk), run_time=1.2)
        self.play(LaggedStart(FadeIn(laptop, shift=DOWN * 0.15), FadeIn(server, shift=DOWN * 0.15), FadeIn(socket),
                              lag_ratio=0.3), *warm_heat(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "desk", used=1.2 + 1.4, during=warm_heat)

        lap_w, dev_w = ValueTracker(0.0), ValueTracker(0.0)
        col_x = room["wall_r"].get_right()[0] + 0.35
        lap_live = _live(
            lambda: rf"\text{{Laptop}}\;{de_num(lap_w.get_value())}\,\mathrm{{W}}",
            np.array([col_x, 0.8, 0.0]),
            size=LABEL_FONT_SIZE, color=COLOR_HEAT,
        )
        dev_live = _live(
            lambda: rf"\text{{Gerät}}\;{de_num(dev_w.get_value())}\,\mathrm{{W}}",
            np.array([col_x, 0.38, 0.0]),
            size=LABEL_FONT_SIZE, color=COLOR_EQUIP,
        )
        phi_e = ValueTracker(0.0)
        phi_e.add_updater(lambda t: t.set_value(lap_w.get_value() + dev_w.get_value()))
        phi_e_live = _live(
            lambda: _watts("e", phi_e),
            np.array([col_x, -0.15, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_EQUIP,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "spark"))
        self.play(Create(cable_lines), Flash(sock, color=COLOR_EQUIP, flash_radius=0.28, line_length=0.16),
                  *warm_heat(0.9), run_time=0.9)
        self.add(phi_e, lap_live, dev_live, phi_e_live)
        self.play(*device_heat(1.3), laptop.animate.set_color(COLOR_HEAT), lap_w.animate.set_value(PHI_E_LAPTOP_W),
                  run_time=1.3)
        self.play(*device_heat(1.3), server_box.animate.set_color(COLOR_EQUIP), server[1].animate.set_color(COLOR_EQUIP),
                  dev_w.animate.set_value(PHI_E_DEVICE_W), room["air"].animate.set_fill(COLOR_HEAT, opacity=0.08),
                  run_time=1.3)
        hold_for(self, self.NARRATION, "spark", during=device_heat)

        row, box, items = math_panel(_sum_parts(["p", "e"]), edge_buff=FORMULA_EDGE_BUFF)
        ring = highlight_param(items, "phi_e", color=COLOR_EQUIP)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "phi_e"))
        self.play(Create(row), Create(box), *device_heat(1.3), run_time=1.3)
        self.play(Create(ring), *device_heat(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "phi_e", during=device_heat)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 — Lighting Φ_l
B4_ROOM_C = np.array([0.0, 0.05, 0.0])
B4_LAMP_SCALE = 1.8


class Beat4_BeleuchtungPhiL(Scene):
    NARRATION = [
        ("lamp",
         "Lighting shines onto the floor — and most of it becomes heat.",
         "Beleuchtung strahlt auf den Boden — und der Großteil wird Wärme."),
        ("waves",
         "Warm plumes rise from the lit patch back into the room air.",
         "Warme Schwaden steigen vom beleuchteten Fleck zurück in die Raumluft."),
        ("phi_l",
         "Lighting heat is Phi l. Together: Phi p plus Phi e plus Phi l.",
         "Lichtwärme ist Phi-l. Zusammen: Phi-p plus Phi-e plus Phi-l."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Beleuchtungswärme", title)
        din = _din_ref("DIN V 18599-10")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "lamp"))
        self.play(FadeIn(caption), run_time=0.3)

        room = room_section(B4_ROOM_C, w=9.0, h=3.3)
        y_f = room["y_f"]
        anchor = np.array([room["center"][0], room["y_c"], 0.0])
        lamp = lamp_glyph(anchor, drop=0.45)
        lamp["group"].scale(B4_LAMP_SCALE, about_point=anchor)
        bulb = lamp["bulb"].get_center()
        rays = _light_rays(bulb, y_f + 0.02, (-2.6, -1.3, 0.0, 1.3, 2.6))
        floor_patch = Ellipse(width=5.4, height=0.14, stroke_width=0, fill_color=COLOR_LIGHT, fill_opacity=0.35)
        floor_patch.move_to(np.array([bulb[0], y_f + 0.07, 0.0]))
        spots = [np.array([bulb[0] + dx, y_f + 0.1, 0.0]) for dx in (-1.9, -0.65, 0.65, 1.9)]
        plumes = [smooth_path([p, p + np.array([0.12, 0.4, 0.0]), p + np.array([-0.1, 0.8, 0.0]),
                               p + np.array([0.05, 1.25, 0.0])]) for p in spots]

        def light(rt):
            return [ripples([bulb], r_max=0.8, color=PASTEL_RED, down=True, cycles=_cycles(rt)),
                    pulse_flashes(_ray_paths(rays), COLOR_LIGHT, repeats=max(1, round(rt / 1.6)), width=3.5)]

        def light_heat(rt):
            cyc = _cycles(rt, 1.5)
            return [*light(rt), ripples(spots, r_max=0.35, color=COLOR_HEAT, cycles=cyc),
                    flow_animation([(plumes, COLOR_HEAT, PASTEL_RED)], waves=3, radius=0.055, cycles=cyc)]

        self.add(room["air"])
        self.play(FadeIn(room["shell"]), FadeIn(room["glass"]), run_time=1.0)
        self.play(FadeIn(lamp["group"], shift=DOWN * 0.15), run_time=0.7)
        phi_l = ValueTracker(0.0)
        phi_l_live = _live(
            lambda: _watts("l", phi_l),
            np.array([lamp["group"].get_right()[0] + 0.35, lamp["group"].get_bottom()[1] + 0.35, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_LIGHT,
        )
        self.add(phi_l_live)
        self.play(
            lamp["bulb"].animate.set_fill(COLOR_LIGHT, opacity=0.85),
            LaggedStart(*[Create(r) for r in rays], lag_ratio=0.08),
            FadeIn(floor_patch),
            phi_l.animate.set_value(PHI_L_W),
            run_time=1.3,
        )
        self.play(*light(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "lamp", used=1.0 + 0.7 + 1.3 + 1.4, during=light)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "waves"))
        self.play(*light_heat(1.5), room["air"].animate.set_fill(COLOR_HEAT, opacity=0.08), run_time=1.5)
        hold_for(self, self.NARRATION, "waves", during=light_heat)

        row, box, items = math_panel(_sum_parts(["p", "e", "l"]), edge_buff=FORMULA_EDGE_BUFF)
        ring = highlight_param(items, "phi_l", color=COLOR_LIGHT)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "phi_l"))
        self.play(Create(row), Create(box), *light_heat(1.5), run_time=1.5)
        self.play(Create(ring), *light_heat(1.5), run_time=1.5)
        hold_for(self, self.NARRATION, "phi_l", during=light_heat)
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 — Sum Φ_int and specific density
#region Beat5 layout
B5_BAR_UNIT = 3.0 / PHI_INT_W
B5_BAR_W = 0.62
B5_BASE_Y = -1.05
B5_BAR_XS = (-5.30, -3.85, -2.40)
B5_STACK_X = -0.60
B5_PLAN_C = np.array([3.40, 0.35, 0.0])
B5_PLAN_UNIT = 0.5
B5_ICON_Y = 1.3


def _b5_icon(key: str, x: float):
    """🧩 Source glyph above its bar — person, laptop or lamp — with the point its heat leaves from."""
    at = np.array([x, B5_ICON_Y, 0.0])
    if key == "p":
        fig = person_glyph(at, color=COLOR_PEOPLE, scale=0.8)
        return fig, fig.get_center() + UP * 0.08, False
    if key == "e":
        lap = _laptop(at + np.array([0.22, -0.2, 0.0]), color=COLOR_EQUIP)
        return lap, lap[1].get_center(), False
    lamp = lamp_glyph(at + UP * 0.3, drop=0.1)
    lamp["bulb"].set_fill(COLOR_LIGHT, opacity=0.85)
    return lamp["group"], lamp["bulb"].get_center(), True
#endregion


class Beat5_SummeUndDichte(Scene):
    NARRATION = [
        ("sources",
         "Persons, equipment, and lighting are the three internal sources.",
         "Personen, Geräte und Beleuchtung sind die drei internen Quellen."),
        ("sum",
         "Phi int equals Phi p plus Phi e plus Phi l — total free heat in watts.",
         "Phi-int ist Phi-p plus Phi-e plus Phi-l — die gesamte freie Wärme in Watt."),
        ("density",
         "Divide by the net floor area A N to get watts per square meter.",
         "Geteilt durch die Nettogrundfläche A-N ergibt Watt pro Quadratmeter."),
        ("din",
         "That specific flux q int is tabulated in DIN V 18599-10.",
         "Diese spezifische Dichte q-int steht tabellarisch in DIN V 18599-10."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Summe und Wärmestromdichte", title)
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "sources"))
        self.play(FadeIn(caption), run_time=0.3)

        sources = (
            ("p", PHI_P_W, COLOR_PEOPLE, "Personen"),
            ("e", PHI_E_W, COLOR_EQUIP, "Geräte"),
            ("l", PHI_L_W, COLOR_LIGHT, "Beleuchtung"),
        )
        grow = [ValueTracker(0.0) for _ in sources]

        def bar_h(t):
            return max(0.002, t.get_value() * B5_BAR_UNIT)

        bars, tokens, names, values, icons, heat_at = VGroup(), VGroup(), VGroup(), VGroup(), VGroup(), []
        for (key, _w, color, name), x, t in zip(sources, B5_BAR_XS, grow):
            icon, spot, down = _b5_icon(key, x)
            icons.add(icon)
            heat_at.append((spot, down))
            bar = Rectangle(
                width=B5_BAR_W, height=0.002, color=color, stroke_width=1.2,
                fill_color=color, fill_opacity=0.78,
            )
            bar.add_updater(lambda m, x=x, t=t: m.stretch_to_fit_height(bar_h(t)).move_to(
                np.array([x, B5_BASE_Y + bar_h(t) / 2, 0.0])))
            bars.add(bar)
            tokens.add(math_label(_phi(key), size=BODY_FONT_SIZE, color=color).move_to(
                np.array([x, B5_BASE_Y - 0.30, 0.0])))
            names.add(Text(name, font_size=LABEL_FONT_SIZE, color=color).move_to(
                np.array([x, B5_BASE_Y - 0.66, 0.0])))
            values.add(_live(
                lambda t=t: rf"{de_num(t.get_value())}\,\mathrm{{W}}",
                lambda bar=bar: bar.get_top() + UP * 0.14,
                size=LABEL_FONT_SIZE, color=color, edge="center",
            ))

        total = ValueTracker(0.0)
        total_live = _live(
            lambda: _watts("int", total),
            np.array([B5_STACK_X, B5_BASE_Y + PHI_INT_W * B5_BAR_UNIT + 0.22, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_HEAT, edge="center",
        )

        def icon_heat(rt):
            cyc = _cycles(rt)
            return [ripples([p for p, d in heat_at if not d], r_max=0.35, color=COLOR_HEAT, cycles=cyc),
                    ripples([p for p, d in heat_at if d], r_max=0.3, color=PASTEL_RED, down=True, cycles=cyc)]

        self.add(*bars, *values)
        self.play(
            *[FadeIn(m, shift=DOWN * 0.2) for m in (*icons, *tokens, *names)],
            *[t.animate.set_value(w) for t, (_k, w, _c, _n) in zip(grow, sources)],
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "sources", used=1.2, during=icon_heat)

        for bar in bars:
            bar.clear_updaters()
        heights = [w * B5_BAR_UNIT for _k, w, _c, _n in sources]
        lows = np.cumsum([0.0, *heights[:-1]])
        stack_moves, token_moves = [], []
        for bar, tok, h, lo in zip(bars, tokens, heights, lows):
            mid = np.array([B5_STACK_X, B5_BASE_Y + lo + h / 2, 0.0])
            stack_moves.append(bar.animate.move_to(mid))
            token_moves.append(tok.animate.move_to(mid + LEFT * (B5_BAR_W / 2 + 0.40)))
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sum"))
        self.add(total_live)
        self.play(
            FadeOut(values), FadeOut(names), FadeOut(icons),
            *stack_moves, *token_moves,
            total.animate.set_value(PHI_INT_W),
            run_time=1.4,
        )

        row, box, items = math_panel(_sum_parts(["p", "e", "l"], lead="int"), edge_buff=FORMULA_EDGE_BUFF)
        targets = [items[f"phi_{k}"] for k in ("p", "e", "l")]
        rest = VGroup(*[m for m in row if all(m is not t for t in targets)])
        self.play(
            *[ReplacementTransform(tok, target) for tok, target in zip(tokens, targets)],
            FadeIn(rest), Create(box),
            run_time=1.6,
        )
        self.remove(rest, *targets)
        self.add(row)
        ring = highlight_param(items, "phi_int", color=PASTEL_ORANGE)
        self.play(Create(ring), run_time=0.45)
        hold_for(self, self.NARRATION, "sum")
        self.play(FadeOut(ring), run_time=0.25)

        density_sub = beat_subtitle("Spezifische Wärmestromdichte (DIN V 18599-10)", title)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "density"))
        self.play(ReplacementTransform(subtitle, density_sub), run_time=0.7)
        subtitle = density_sub

        row2, box2, items2 = math_panel([
            ("q_int", r"q_{\mathrm{int}}", COLOR_HEAT),
            (None, "=", PASTEL_WHITE),
            ("frac", rf"\frac{{{_phi('int')}}}{{\textcolor{{{PASTEL_CYAN}}}{{A_{{\mathrm{{N}}}}}}}}", PASTEL_WHITE),
            (None, r"\;[\mathrm{W/m^{2}}]", PASTEL_WHITE),
        ], edge_buff=FORMULA_EDGE_BUFF)
        self.play(ReplacementTransform(row, row2), ReplacementTransform(box, box2), run_time=1.4)
        row, box, items = row2, box2, {**items2, "a_n": items2["frac"][0][2]}

        pw, pd = ROOM_W_M * B5_PLAN_UNIT, ROOM_D_M * B5_PLAN_UNIT
        plan = Rectangle(
            width=pw, height=pd, color=GREY_B, stroke_width=4,
            fill_color=PASTEL_CYAN, fill_opacity=0.10,
        ).move_to(B5_PLAN_C)
        plan_tag = Text("Grundriss Büro", font_size=LABEL_FONT_SIZE, color=GREY_B)
        plan_tag.next_to(plan, UP, buff=0.14)
        w_dim = dim_arrow(plan.get_corner(DL) + DOWN * 0.25, plan.get_corner(DR) + DOWN * 0.25, color=PASTEL_CYAN)
        d_dim = dim_arrow(plan.get_corner(DR) + RIGHT * 0.25, plan.get_corner(UR) + RIGHT * 0.25, color=PASTEL_CYAN)
        w_on, d_on = ValueTracker(0.0), ValueTracker(0.0)

        def area():
            return w_on.get_value() * ROOM_W_M * d_on.get_value() * ROOM_D_M

        w_live = _live(
            lambda: rf"{de_num(w_on.get_value() * ROOM_W_M)}\,\mathrm{{m}}",
            np.array([plan.get_center()[0], plan.get_bottom()[1] - 0.62, 0.0]),
            size=LABEL_FONT_SIZE, color=PASTEL_CYAN, edge="center",
        )
        d_live = _live(
            lambda: rf"{de_num(d_on.get_value() * ROOM_D_M)}\,\mathrm{{m}}",
            np.array([plan.get_right()[0] + 0.45, plan.get_center()[1] - 0.08, 0.0]),
            size=LABEL_FONT_SIZE, color=PASTEL_CYAN,
        )
        a_live = _live(
            lambda: rf"A_{{\mathrm{{N}}}} = {de_num(area())}\,\mathrm{{m^{{2}}}}",
            np.array([plan.get_center()[0], plan.get_center()[1] - 0.10, 0.0]),
            size=BODY_FONT_SIZE, color=PASTEL_CYAN, edge="center",
        )
        q_on = ValueTracker(0.0)
        q_live = _live(
            lambda: (rf"q_{{\mathrm{{int}}}} = \frac{{{de_num(total.get_value())}\,\mathrm{{W}}}}"
                     rf"{{{de_num(area())}\,\mathrm{{m^{{2}}}}}}"
                     rf" = {de_num(q_on.get_value() * total.get_value() / max(area(), 1e-6), 1)}\,\mathrm{{W/m^{{2}}}}"),
            np.array([plan.get_center()[0], plan_tag.get_top()[1] + 0.42, 0.0]),
            size=BODY_FONT_SIZE, color=COLOR_HEAT, edge="center",
        )

        self.play(Create(plan), FadeIn(plan_tag), run_time=0.8)
        self.add(w_live, d_live, a_live)
        self.play(
            Create(w_dim), Create(d_dim),
            w_on.animate.set_value(1.0), d_on.animate.set_value(1.0),
            run_time=1.2,
        )
        ring = highlight_param(items, "a_n", color=PASTEL_CYAN)
        self.play(Create(ring), run_time=0.45)
        hold_for(self, self.NARRATION, "density")
        self.play(FadeOut(ring), run_time=0.25)

        ring = highlight_param(items, "q_int", color=COLOR_HEAT)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "din"))
        self.add(q_live)
        self.play(Create(ring), q_on.animate.set_value(1.0), run_time=1.2)
        hold_for(self, self.NARRATION, "din")
        self.play(FadeOut(ring), run_time=0.25)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
