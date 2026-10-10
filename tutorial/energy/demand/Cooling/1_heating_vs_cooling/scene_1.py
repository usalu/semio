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
    SUBTITLE_FONT_SIZE, BODY_FONT_SIZE, LABEL_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_GREEN, P_BLUE,
    watt_anchor,
    smooth_path, flow_guides, flow_animation,
    house_section, sun_glyph, sun_rays, shine, pulse_flashes, ripples,
    person_glyph, lamp_glyph, thermometer_glyph,
    math_label, math_readout, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Heizwärmebedarf vs. Kühllast"

# Shared layout anchors — one house and one thermometer slot for every beat so
# winter / summer / cooling stay visually continuous.
HOUSE_CENTER = ORIGIN + DOWN * 0.75
SUN_C = np.array([-4.2, 1.35, 0.0])
THERM_BOTTOM = np.array([3.3, -1.55, 0.0])


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

def _house():
    """🏠 The Physical Fundamentals section house at the shared anchor, plus its interior air for heat tinting."""
    house = house_section(HOUSE_CENTER)
    t = 0.1
    inner_peak = house["roof_peak"] + DOWN * 0.14
    air = Polygon(
        house["bottom_left"] + RIGHT * t + UP * 0.02, house["bottom_right"] + LEFT * t + UP * 0.02,
        house["top_right"] + LEFT * t + DOWN * 0.06, inner_peak, house["top_left"] + RIGHT * t + DOWN * 0.06,
        stroke_width=0, fill_color=P_RED, fill_opacity=0.0,
    )
    air.set_z_index(-1)
    house["air"] = air
    return house


def _solar(house, *, per_window: int = 2):
    """☀️ Sun plus parallel straight rays through both windows, landing on the floor of each storey."""
    sun = sun_glyph(SUN_C).scale(0.42)
    floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
    spread = np.linspace(0.14, -0.14, per_window)
    sets = [sun_rays(SUN_C, w["x"], [w["center"][1] + dy for dy in spread], floor_y, gap=0.5)
            for w, floor_y in zip(house["windows"], floors)]
    paths = [[s, h, p] for r in sets for s, h, p in zip(r["starts"], r["hits"], r["lands"])]
    return {"sun": sun, "rays": sets, "paths": paths, "group": VGroup(sun, *[r["group"] for r in sets])}


def _internal_gains(house):
    """💡 Occupant, kitchen, desk laptop and two pendant lamps — the Physical Fundamentals glyphs inside the house."""
    bl = house["bottom_left"]
    floor_y = bl[1]
    mid_y = house["level_1"].get_center()[1]

    person = person_glyph(ORIGIN, scale=1.1)
    person.move_to(np.array([-0.75, floor_y + 0.02 + person.height / 2, 0.0]))

    counter = Rectangle(width=0.8, height=0.34, color=P_WHITE, stroke_width=1.5)
    counter.move_to(np.array([1.1, floor_y + 0.17, 0.0]))
    pot = Rectangle(width=0.26, height=0.13, color=P_CYAN, stroke_width=1.5).next_to(counter, UP, buff=0.0)
    handle = Line(pot.get_corner(UR) + DOWN * 0.04, pot.get_corner(UR) + DOWN * 0.04 + RIGHT * 0.1,
                  color=P_CYAN, stroke_width=1.5)
    kitchen = VGroup(counter, pot, handle)

    desk_y = mid_y + 0.42
    desk = VGroup(
        Line(np.array([0.2, desk_y, 0.0]), np.array([1.1, desk_y, 0.0]), color=P_WHITE, stroke_width=1.5),
        Line(np.array([0.3, desk_y, 0.0]), np.array([0.3, mid_y, 0.0]), color=P_WHITE, stroke_width=1.5),
        Line(np.array([1.0, desk_y, 0.0]), np.array([1.0, mid_y, 0.0]), color=P_WHITE, stroke_width=1.5),
    )
    laptop = VGroup(
        Line(np.array([0.45, desk_y + 0.02, 0.0]), np.array([0.75, desk_y + 0.02, 0.0]), color=P_CYAN, stroke_width=1.8),
        Line(np.array([0.75, desk_y + 0.02, 0.0]), np.array([0.69, desk_y + 0.26, 0.0]), color=P_CYAN, stroke_width=1.8),
    )
    device = VGroup(desk, laptop)

    lamp_low = lamp_glyph(np.array([0.15, mid_y, 0.0]), drop=0.18)
    roof_x = -0.75
    roof_y = house["top_left"][1] - 0.05 + (house["roof_peak"][1] - 0.12 - house["top_left"][1] + 0.05) * (
        (roof_x - house["top_left"][0]) / (house["roof_peak"][0] - house["top_left"][0]))
    lamp_up = lamp_glyph(np.array([roof_x, roof_y, 0.0]), drop=0.5)

    warm = [person.get_center() + UP * 0.12, laptop[1].get_center(), pot.get_top()]
    bulbs = [lamp_low["bulb"].get_center(), lamp_up["bulb"].get_center()]
    return {
        "person": person, "device": device, "kitchen": kitchen, "lamps": VGroup(lamp_low["group"], lamp_up["group"]),
        "sources": VGroup(device, person, kitchen, lamp_low["group"], lamp_up["group"]),
        "warm": warm, "bulbs": bulbs,
    }


def _gain_heat(gains, run_time: float, *, color=P_ORANGE):
    """🌡️ Long-wave heat from every internal source — ripples from people and devices, down from the lamps."""
    cycles = max(1.0, run_time / 1.3)
    return [ripples(gains["warm"], r_max=0.42, color=color, cycles=cycles),
            ripples(gains["bulbs"], r_max=0.4, color=P_RED, down=True, cycles=cycles)]


def _room_thermometer(temp: ValueTracker):
    """🌡️ Physical Fundamentals thermometer whose column and reading follow the room temperature."""
    th = thermometer_glyph(THERM_BOTTOM, height=1.9, level=(temp.get_value() - 15) / 25)
    th["column"].add_updater(lambda m: th["level"].set_value((temp.get_value() - 15) / 25))
    title = Text("Raumtemperatur", font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(th["group"], UP, buff=0.18)

    def reading():
        t = temp.get_value()
        color = P_RED if t > 25 else (P_CYAN if t <= 21.5 else P_WHITE)
        return math_label(rf"{de_num(t)}\,\mathrm{{°C}}", size=SUBTITLE_FONT_SIZE, color=color)

    read = always_redraw(lambda: reading().next_to(th["group"][0], RIGHT, buff=0.22))
    return {"static": VGroup(th["group"], title), "column": th["column"], "read": read}


def _build_ahu(house: dict):
    """🌀 Lüftungsgerät standing beside the house — fan, cooling coil and the four air paths of DIN EN 16798-3.

    Außenluft → Kühlregister → Zuluft through the lower window, Abluft through
    the upper window → fan → Fortluft above. Paths feed ``flow_guides`` +
    ``animate_flows`` so the air visibly moves and changes colour at the coil.
    """
    floor_y = house["bottom_left"][1]
    win_lo, win_hi = house["windows"]
    wx = win_lo["x"]
    unit = RoundedRectangle(corner_radius=0.08, width=0.78, height=1.5, color=P_WHITE, stroke_width=1.8,
                            fill_color=P_DEEP_DARK, fill_opacity=1.0)
    unit.move_to(np.array([-3.15, floor_y + 0.75, 0.0]))
    fan_center = unit.get_top() + DOWN * 0.38
    fan_ring = Circle(radius=0.2, color=P_CYAN, stroke_width=1.6).move_to(fan_center)
    fan_blades = VGroup(*[
        Line(fan_center, fan_center + 0.17 * np.array([np.cos(a), np.sin(a), 0.0]), color=P_CYAN, stroke_width=2.0)
        for a in np.linspace(0, TAU, 3, endpoint=False)
    ])
    coil_c = unit.get_bottom() + UP * 0.42
    coil = VMobject(color=P_BLUE, stroke_width=1.6).set_points_as_corners(
        [coil_c + np.array([-0.27 + i * 0.09, 0.08 if i % 2 else -0.08, 0.0]) for i in range(7)])

    lo_y, hi_y = win_lo["center"][1], win_hi["center"][1]
    supply_pts = [(-4.75, floor_y + 0.32), (-3.75, floor_y + 0.36), (coil_c[0], coil_c[1]),
                  (-2.85, lo_y - 0.05), (wx - 0.4, lo_y), (wx + 0.5, lo_y - 0.05), (wx + 1.3, floor_y + 0.25)]
    extract_pts = [(wx + 1.4, hi_y - 0.3), (wx + 0.5, hi_y), (wx - 0.45, hi_y), (fan_center[0] + 0.25, fan_center[1] - 0.05),
                   (fan_center[0], fan_center[1] + 0.3), (fan_center[0] - 0.05, fan_center[1] + 1.2),
                   (fan_center[0] + 0.35, fan_center[1] + 1.95)]
    supply_paths = VGroup(*[smooth_path([np.array([x, y + dy, 0.0]) for x, y in supply_pts]) for dy in (0.0, -0.1)])
    extract_paths = VGroup(*[smooth_path([np.array([x + dx, y, 0.0]) for x, y in extract_pts]) for dx in (0.0, 0.1)])

    labels = VGroup(
        Text("Außenluft", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(
            np.array([supply_pts[0][0], supply_pts[0][1], 0.0]), DOWN, buff=0.22),
        Text("Zuluft", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([wx + 0.75, lo_y + 0.32, 0.0])),
        Text("Abluft", font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([wx + 0.85, hi_y + 0.3, 0.0])),
        Text("Fortluft", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(
            np.array([extract_pts[-1][0], extract_pts[-1][1], 0.0]), LEFT, buff=0.2),
    )
    unit_label = Text("Lüftungsgerät\nmit Kühlregister", font_size=LABEL_FONT_SIZE, color=P_TEAL, line_spacing=0.8)
    unit_label.move_to(np.array([-5.3, unit.get_top()[1] + 0.35, 0.0]))
    unit_leader = Line(unit_label.get_right() + RIGHT * 0.1, unit.get_corner(UL) + DOWN * 0.12,
                       color=P_TEAL, stroke_width=1.4, stroke_opacity=0.7)
    return {
        "unit": VGroup(unit, fan_ring, fan_blades, coil), "fan_blades": fan_blades, "fan_center": fan_center,
        "supply_paths": supply_paths, "extract_paths": extract_paths,
        "labels": labels, "unit_label": VGroup(unit_label, unit_leader), "top": extract_pts[-1],
    }

#endregion


#region Beat 1 — Winter helpful gains

class Beat1_WinterGains(Scene):
    NARRATION = [
        ("intro",
         "In winter, free heat gains are welcome — they cut the annual heating demand we must cover.",
         "Im Winter sind kostenlose Wärmegewinne willkommen — sie senken den Heizwärmebedarf."),
        ("solar",
         "Solar gains enter through the windows and warm the rooms for free.",
         "Solare Gewinne treten durch die Fenster ein und heizen die Räume kostenlos."),
        ("internal",
         "People, devices and lights add internal gains — roughly a laptop and a person as everyday watt anchors.",
         "Personen, Geräte und Licht erzeugen interne Gewinne — greifbar wie Laptop und Person."),
        ("outro",
         "Together they cover part of the winter heating demand in DIN V 18599. A design heating load after DIN EN 12831 usually ignores them.",
         "Zusammen decken sie einen Teil des Heizwärmebedarfs nach DIN V 18599. Eine Auslegungsheizlast nach DIN EN 12831 lässt sie meist weg."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Im Winter: Erwünschte kostenlose Wärme", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        house = _house()
        self.add(house["air"])
        self.play(FadeIn(house["group"]), run_time=1.4)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.4)

        solar = _solar(house)
        solar_label = Text("Solare Gewinne", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        solar_label.move_to(SUN_C + np.array([-0.8, -0.85, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar"))
        self.play(FadeIn(solar["sun"], scale=0.7), FadeIn(solar_label), run_time=0.7)
        self.play(*[shine(r, lag=0.25) for r in solar["rays"]], run_time=1.6)
        self.play(house["air"].animate.set_fill(P_ORANGE, opacity=0.08), run_time=0.6)
        hold_for(self, self.NARRATION, "solar",
                 during=lambda rt: [pulse_flashes(solar["paths"], P_YELLOW, repeats=max(1, int(rt / 1.6)))])

        gains = _internal_gains(house)
        internal_label = Text("Interne Gewinne", font_size=BODY_FONT_SIZE, color=P_ORANGE)
        internal_label.next_to(house["floor"], DOWN, buff=0.2)
        laptop_anchor = watt_anchor(60, compare="laptop", title="Gerät").scale(0.55)
        person_anchor = watt_anchor(100, compare="bulb", title="Person").scale(0.55)
        anchor_row = VGroup(person_anchor, laptop_anchor).arrange(DOWN, buff=0.4)
        anchor_row.next_to(house["walls"], RIGHT, buff=0.55)
        anchor_row.set_y(house["center"][1])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "internal"))
        self.play(LaggedStart(*[FadeIn(s) for s in gains["sources"]], lag_ratio=0.2), FadeIn(internal_label),
                  run_time=1.2)
        self.play(*_gain_heat(gains, 1.4), FadeIn(person_anchor), FadeIn(laptop_anchor), run_time=1.4)
        hold_for(self, self.NARRATION, "internal", during=lambda rt: _gain_heat(gains, rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=lambda rt: [
            *_gain_heat(gains, rt), pulse_flashes(solar["paths"], P_YELLOW, repeats=max(1, int(rt / 1.6)))])

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 2 — Summer overheating

class Beat2_SummerOverheat(Scene):
    NARRATION = [
        ("intro",
         "In summer the same gains become a problem — the building overheats.",
         "Im Sommer werden dieselben Gewinne zum Problem — das Gebäude überhitzt."),
        ("excess",
         "Solar gains turn excessive under a harsh red sun, and internal gains keep stacking heat inside.",
         "Solare Gewinne werden übermäßig, und interne Gewinne stapeln weiter Wärme im Inneren."),
        ("trap",
         "Heat is trapped in the insulated envelope — watch the room temperature climb toward thirty-five degrees.",
         "Wärme staut sich in der gedämmten Hülle — die Raumtemperatur steigt Richtung fünfunddreißig Grad."),
        ("outro",
         "Without active cooling the room stays too warm on a hot summer afternoon.",
         "Ohne aktive Kühlung bleibt der Raum am heißen Sommernachmittag zu warm."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Im Sommer: Überhitzung hinter Glas", title)
        din = _din_ref("DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        house = _house()
        self.add(house["air"], house["group"])
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        solar = _solar(house, per_window=3)
        solar_label = Text("Übermäßige\nsolare Gewinne", font_size=LABEL_FONT_SIZE, color=P_RED, line_spacing=0.8)
        solar_label.move_to(SUN_C + np.array([-0.8, -0.95, 0.0]))
        gains = _internal_gains(house)
        internal_label = Text("Interne Gewinne", font_size=BODY_FONT_SIZE, color=P_RED)
        internal_label.next_to(house["floor"], DOWN, buff=0.2)

        def summer_heat(rt):
            return [*_gain_heat(gains, rt, color=P_RED),
                    pulse_flashes(solar["paths"], P_YELLOW, repeats=max(1, int(rt / 1.4)))]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "excess"))
        self.play(FadeIn(solar["sun"], scale=0.7), FadeIn(solar_label), run_time=0.6)
        self.play(*[shine(r, lag=0.2) for r in solar["rays"]],
                  LaggedStart(*[FadeIn(s) for s in gains["sources"]], lag_ratio=0.2), FadeIn(internal_label),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "excess", during=summer_heat)

        temp = ValueTracker(20)
        therm = _room_thermometer(temp)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "trap"))
        self.play(FadeIn(therm["static"]), FadeIn(therm["column"]), FadeIn(therm["read"]), run_time=0.8)
        self.play(temp.animate.set_value(35), house["air"].animate.set_fill(P_RED, opacity=0.3), *summer_heat(3.4),
                  run_time=3.4, rate_func=linear)
        hold_for(self, self.NARRATION, "trap", during=summer_heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        hold_for(self, self.NARRATION, "outro", during=summer_heat)

        therm["column"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 3 — Cooling system activation

class Beat3_CoolingSystem(Scene):
    NARRATION = [
        ("intro",
         "The cooling load is the heat-removal rate we must provide so the room returns to comfort.",
         "Die Kühllast ist die Wärmeleistung, die wir aktiv abführen müssen, damit der Raum wieder komfortabel wird."),
        ("vent",
         "An air-handling unit extracts the warm room air, cools outdoor air at its cooling coil and supplies it back to the room.",
         "Ein Lüftungsgerät saugt die warme Raumluft als Abluft ab, kühlt Außenluft am Kühlregister und bläst sie als Zuluft ein."),
        ("coil",
         "At the coil the outdoor air falls from thirty-two to eighteen degrees. That drop is the heat the unit removes.",
         "Am Kühlregister fällt die Außenluft von 32 auf 18 Grad.\nGenau diese Differenz nimmt das Gerät auf."),
        ("cool_down",
         "As heat leaves, the interior cools and the thermometer settles near twenty-one degrees again.",
         "Wenn Wärme abfließt, kühlt der Innenraum — das Thermometer sinkt wieder Richtung einundzwanzig Grad."),
        ("outro",
         "That required heat removal rate is the cooling load we size the system for.",
         "Genau diese abzuführende Leistung ist die Kühllast, nach der wir das System auslegen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Kühllast als abzuführende Leistung", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)
        house = _house()
        house["air"].set_fill(P_RED, opacity=0.3)
        temp = ValueTracker(35)
        therm = _room_thermometer(temp)
        self.add(house["air"], house["group"], therm["static"], therm["column"], therm["read"])
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        ahu = _build_ahu(house)
        supply_guides = flow_guides(ahu["supply_paths"], P_CYAN, opacity=0.22)
        extract_guides = flow_guides(ahu["extract_paths"], P_RED, opacity=0.22)
        streams = [(ahu["extract_paths"], P_RED, P_ORANGE), (ahu["supply_paths"], P_ORANGE, P_CYAN)]

        def fan_spin(turns: float, run_time: float):
            return Rotate(ahu["fan_blades"], angle=turns * TAU, about_point=ahu["fan_center"],
                          run_time=run_time, rate_func=linear)

        def running(rt):
            return [fan_spin(rt / 1.3, rt), flow_animation(streams, waves=5, cycles=rt / 1.6)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "vent"))
        self.play(FadeIn(ahu["unit"]), Create(supply_guides), Create(extract_guides),
                  FadeIn(ahu["labels"]), FadeIn(ahu["unit_label"]), run_time=1.6)
        self.play(*running(2.2), run_time=2.2)
        hold_for(self, self.NARRATION, "vent", during=running)

        # 🌡️ The coil is the machine: outdoor air arrives at 32 °C and leaves at 18 °C.
        coil = ahu["unit"][-1]
        coil_temp = ValueTracker(32.0)
        coil_read = math_readout(
            lambda: rf"{de_num(coil_temp.get_value())}\,\mathrm{{°C}}",
            lambda: coil.get_center() + LEFT * 1.45,
            size=BODY_FONT_SIZE, color=P_WHITE, edge="right",
        )
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "coil"))
        self.add(coil_read)
        self.play(
            coil.animate.set_color(P_BLUE).set_stroke(width=3.2),
            coil_temp.animate.set_value(18),
            *running(2.4),
            run_time=2.4,
        )
        hold_for(self, self.NARRATION, "coil", during=running)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "cool_down"))
        self.play(*running(4.0), house["air"].animate.set_fill(P_CYAN, opacity=0.06), temp.animate.set_value(21),
                  run_time=4.0)
        hold_for(self, self.NARRATION, "cool_down", during=running)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        top = np.array([ahu["top"][0], ahu["top"][1], 0.0])
        qk_symbol = math_label(r"\dot{Q}_{K}", at=top + np.array([-0.15, 0.42, 0.0]), size=SUBTITLE_FONT_SIZE,
                               color=P_CYAN, edge="left")
        qk_word = Text("Kühllast", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        qk_word.next_to(qk_symbol, RIGHT, buff=0.18).align_to(qk_symbol, DOWN)
        self.play(FadeIn(qk_symbol), FadeIn(qk_word), *running(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "outro", during=running)

        therm["column"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion
