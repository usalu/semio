"""🌬️ Ventilation Chapter 1 — Warum lüften?

Three beats open the Lüftung video: what ventilation has to remove from a room with people (CO₂, moisture,
odours), the path heat takes with the air through a house in winter and in summer, and the roadmap of the
video. Heating and Cooling both build on it, so the need and the air path are told season-neutral.
https://www.umweltbundesamt.de/themen/gesundheit/umwelteinfluesse-auf-den-menschen/innenraumluft
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
    apply_scene_style, scene_title, play_scene_title,
    beat_subtitle, BEAT_SUBTITLE_FADE,
    BODY_FONT_SIZE, LABEL_FONT_SIZE, body_text,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    smooth_path, flow_guides, flow_animation,
    meter, bind_meter, recolor_meter, dim_arrow,
    math_label, math_panel, math_readout, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    house_section, open_window, person_glyph, sun_glyph, cloud_glyph, thermometer_glyph, droplets, ripples,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Warum lüften?"

CO2_COLOR = P_YELLOW
MOISTURE_COLOR = P_BLUE
ODOUR_COLOR = P_GREEN
AIR_COLOR = P_CYAN
WINTER_COLOR = P_BLUE
SUMMER_COLOR = P_RED
PER_PERSON_M3H = 30.0
PEOPLE = 4
LATENT_PER_PERSON_W = 30.0
THETA_WINTER = (20.0, 0.0)
THETA_SUMMER = (26.0, 32.0)


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in every other tutorial chapter. This chapter cites
    DIN EN 16798-1 (indoor air quality, CO₂ categories) and DIN 1946-6 (ventilation of dwellings).
    """
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs
def _room(center, width, height, color=P_WHITE, t: float = 0.16):
    """🏠 Physical Fundamentals room section — ceiling slab and side walls, then the floor slab, centred on the outline."""
    cx, cy = float(center[0]), float(center[1])
    style = dict(color=color, stroke_width=2, fill_color=color, fill_opacity=0.14)
    ceiling = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy + height / 2, 0.0]))
    walls = VGroup(*[Rectangle(width=t, height=height - t, **style).move_to(np.array([cx + s * width / 2, cy, 0.0]))
                     for s in (-1, 1)])
    floor = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy - height / 2, 0.0]))
    return VGroup(VGroup(ceiling, walls), floor)


def _window(pos, height: float = 1.15, width: float = 0.26, color=P_CYAN):
    """🪟 Facade opening in a room wall — a framed gap through the slab line."""
    return Rectangle(width=width, height=height, color=color, stroke_width=2.2, fill_color=P_DEEP_DARK,
                     fill_opacity=1.0).move_to(pos)


def _stand(room_c, room_h, x, color=P_ORANGE, t: float = 0.16):
    """🧍 Occupant at true scale — 1.75 m in a 2.5 m room, feet on the floor slab's top edge at ``x``."""
    height = 0.7 * (room_h - t)
    scale = height / (0.66 * 1.09)
    pos = np.array([x, float(room_c[1]) - room_h / 2 + t / 2 + height / 2, 0.0])
    return person_glyph(pos, color=color, scale=1.09 * scale)


def _fan(pos, color, radius: float = 0.24):
    """🌀 Ring plus three blades — marks a stream as fan-driven."""
    ring = Circle(radius=radius, color=color, stroke_width=2.4)
    blades = VGroup(*[Line(ORIGIN, RIGHT * radius * 0.78, color=color, stroke_width=2.4).rotate(a, about_point=ORIGIN)
                      for a in (0.0, TAU / 3, 2 * TAU / 3)])
    return VGroup(ring, blades).move_to(pos)


def _streams(streams, rt: float, *, speed: float = 0.5, waves: int = 3, radius: float = 0.065):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, radius=radius, cycles=speed * rt)


def _gauge(x: float, y_c: float, color: str, tracker, *, length: float = 2.4, thickness: float = 0.46):
    """📊 Vertical ``meter`` without its text cap, driven by ``tracker`` (0…1) — the caption is typeset by the beat."""
    gauge = meter("·", length=length, thickness=thickness, color=color)
    VGroup(gauge["track"], gauge["fill"]).move_to(np.array([x, y_c, 0.0]))
    gauge["group"] = VGroup(gauge["track"], gauge["fill"])
    gauge["y_of"] = lambda frac: gauge["track"].get_bottom()[1] + 0.04 + frac * gauge["span"]
    return bind_meter(gauge, tracker)


def _handover(src: str, color: str):
    """🔖 Typeset hand-over chip naming the quantity the next video takes over."""
    label = math_label(src, size=BODY_FONT_SIZE, color=color)
    return VGroup(SurroundingRectangle(label, color=color, corner_radius=0.1, buff=0.14, stroke_width=1.8), label)


def _snap(mob):
    """📸 Static copy of a live readout — safe to fly into a formula slot."""
    snap = mob.copy()
    snap.clear_updaters()
    for m in snap.get_family():
        m.clear_updaters()
    return snap


def _fill_panel(scene, panel, sources, *, run_time: float = 2.0):
    """🧲 Every named formula slot is fed by the drawn quantity it stands for, then the rest of the formula appears.

    ``sources`` maps a slot key to the on-screen mobject it is read from; a copy of it morphs into its slot.
    """
    row, box, items = panel
    targets = [items[key] for key in sources]
    rest = VGroup(*[m for m in row.submobjects if all(m is not t for t in targets)])
    for part in row.get_family():
        part._layout_zone = "formula"
    scene.play(*[TransformFromCopy(_snap(src), items[key]) for key, src in sources.items()], run_time=run_time * 0.75)
    scene.play(FadeIn(rest), Create(box), run_time=1.0)


def _hide(scene, *mobs, run_time: float = 0.8):
    """🫥 Fade out what has been explained so the next step starts on a clean screen; live readouts stop first."""
    for mob in mobs:
        for part in mob.get_family():
            part.clear_updaters()
    scene.play(*[FadeOut(m) for m in mobs], run_time=run_time)
#endregion


#region Beat1 – What ventilation has to remove
class Beat1_Lueftungsbedarf(Scene):
    """🫁 A room with four people: CO₂, moisture and odours build up until fresh air replaces the used air.

    https://de.wikipedia.org/wiki/Pettenkofer-Zahl
    """

    NARRATION = [
        ("intro",
         "Four people are in a room. They breathe, sweat and give off heat.",
         "Vier Personen halten sich im Raum auf. Sie atmen, schwitzen und geben Wärme ab."),
        ("co2",
         "Every breath adds carbon dioxide. Outdoors the air holds about 420 ppm.",
         "Jeder Atemzug erhöht den CO₂-Gehalt. Draußen sind es etwa 420 ppm."),
        ("pettenkofer",
         "At about 1000 ppm the air counts as used up: the Pettenkofer value.",
         "Bei rund 1000 ppm gilt die Luft als verbraucht: der Pettenkoferwert."),
        ("feuchte",
         "Moisture adds up as well: a resting person gives off about 30 watts latent.",
         "Dazu kommt Feuchte: Ein ruhender Mensch gibt rund 30 W latent ab."),
        ("geruch",
         "Odours and pollutants collect in the room too.",
         "Auch Gerüche und Schadstoffe sammeln sich im Raum an."),
        ("frisch",
         "Fresh air replaces the used air, and all three loads fall again.",
         "Frischluft ersetzt die verbrauchte Luft: Alle drei Werte sinken wieder."),
        ("personen",
         "More people need more air: about 30 cubic metres per hour and person is the planning order of magnitude.",
         "Mehr Personen brauchen mehr Luft: etwa 30 m³/h je Person."),
        ("jahreszeit",
         "This need exists in winter as much as in summer, so ventilation comes before heating and cooling.",
         "Diesen Bedarf gibt es im Winter wie im Sommer, darum steht die Lüftung vor Heizung und Kühlung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Was Lüftung leisten muss", title)
        din = _din_ref("DIN EN 16798-1 · DIN 1946-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c, room_w, room_h = np.array([-2.7, 0.45, 0.0]), 5.2, 2.5
        shell = _room(room_c, room_w, room_h)
        xs = [-4.5, -3.3, -2.1, -0.9]
        people = VGroup(*[_stand(room_c, room_h, x) for x in xs])
        win_y = room_c[1] + 0.3
        left_win = _window(np.array([room_c[0] - room_w / 2, win_y, 0.0]))
        right_win = _window(np.array([room_c[0] + room_w / 2, win_y, 0.0]))
        floor_top = room_c[1] - room_h / 2 + 0.08
        heads = [p.get_top() + UP * 0.05 for p in people]

        co2, moist, odor = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        gx = {"co2": 1.5, "moist": 4.9, "odor": 6.0}
        g_co2 = _gauge(gx["co2"], 0.35, CO2_COLOR, co2)
        g_moist = _gauge(gx["moist"], 0.35, MOISTURE_COLOR, moist)
        g_odor = _gauge(gx["odor"], 0.35, ODOUR_COLOR, odor)
        cap_y = g_co2["track"].get_top()[1] + 0.34
        cap_co2 = math_label(r"\mathrm{CO_{2}}", np.array([gx["co2"], cap_y, 0.0]), color=CO2_COLOR)
        cap_moist = math_label(r"\text{Feuchte}", np.array([gx["moist"], cap_y, 0.0]), size=LABEL_FONT_SIZE, color=MOISTURE_COLOR)
        cap_odor = math_label(r"\text{Geruch}", np.array([gx["odor"], cap_y, 0.0]), size=LABEL_FONT_SIZE, color=ODOUR_COLOR)

        def tick(gauge, x, frac):
            y = gauge["y_of"](frac)
            return Line(np.array([x - 0.31, y, 0.0]), np.array([x + 0.31, y, 0.0]), color=P_WHITE, stroke_width=2.2)

        tick_out, tick_pk = tick(g_co2, gx["co2"], 0.42), tick(g_co2, gx["co2"], 1.0)
        def tag(name: str, value: str, y: float):
            lines = VGroup(math_label(rf"\text{{{name}}}", size=LABEL_FONT_SIZE), math_label(value, size=LABEL_FONT_SIZE, color=P_WHITE))
            lines.arrange(DOWN, buff=0.08, aligned_edge=LEFT)
            return lines.move_to(np.array([gx["co2"] + 0.45 + lines.width / 2, y, 0.0]))

        tag_out = tag("Außenluft", r"420\,\mathrm{ppm}", tick_out.get_y())
        tag_pk = tag("Pettenkoferwert", r"1000\,\mathrm{ppm}", tick_pk.get_y())
        read_co2 = math_readout(lambda: rf"c = {de_num(co2.get_value() * 1000)}\,\mathrm{{ppm}}", np.array([gx["co2"], -1.35, 0.0]),
                                color=CO2_COLOR, edge="center")
        read_lat = math_readout(lambda: rf"\dot{{Q}}_{{\mathrm{{lat}}}} = {de_num(moist.get_value() * PEOPLE * LATENT_PER_PERSON_W)}\,\mathrm{{W}}",
                                np.array([gx["moist"], -1.35, 0.0]), color=MOISTURE_COLOR, edge="center")
        co2_col = VGroup(g_co2["group"], cap_co2, tick_out, tick_pk, tag_out, tag_pk, read_co2)
        moist_col = VGroup(g_moist["group"], cap_moist, read_lat)
        odor_col = VGroup(g_odor["group"], cap_odor)

        rng = np.random.default_rng(7)
        n_dots = 16
        co2_dots = VGroup(*[Dot(np.array([rng.uniform(-5.0, -0.4), room_c[1] + rng.uniform(0.62, 1.08), 0.0]), radius=0.05,
                                color=CO2_COLOR).set_fill(CO2_COLOR, opacity=0.0) for _ in range(n_dots)])
        for i, dot in enumerate(co2_dots):
            dot.add_updater(lambda d, i=i: d.set_fill(CO2_COLOR, opacity=0.9 * float(np.clip(co2.get_value() * n_dots - i, 0, 1))))
        drops = VGroup(*[droplets(np.array([x + 0.4, floor_top + 0.85, 0.0]), n=4, spread=(0.15, 0.32), seed=i)
                         for i, x in enumerate(xs)])
        for i, cluster in enumerate(drops):
            cluster.add_updater(lambda m, i=i: m.set_fill(MOISTURE_COLOR, opacity=0.85 * float(np.clip(moist.get_value() * PEOPLE - i, 0, 1))))
            cluster.set_fill(MOISTURE_COLOR, opacity=0.0)
        smells = VGroup(*[VGroup(*[Dot(np.array([cx + rng.uniform(-0.28, 0.28), floor_top + 0.18 + rng.uniform(-0.07, 0.1), 0.0]),
                                       radius=0.04, color=ODOUR_COLOR).set_fill(ODOUR_COLOR, opacity=0.0) for _ in range(5)])
                          for cx in (-4.95, -3.9, -2.7, -1.5, -0.5)])
        for j, cluster in enumerate(smells):
            cluster.add_updater(lambda m, j=j: m.set_fill(ODOUR_COLOR, opacity=0.9 * float(np.clip(odor.get_value() * 5 - j, 0, 1))))

        def warmth(rt):
            return [ripples(heads, r_max=0.32, cycles=rt / 1.4, color=P_ORANGE)]

        self.add(co2_dots, drops, smells)
        self.play(Create(shell[0]), Create(shell[1]), run_time=1.3)
        self.play(FadeIn(people), FadeIn(left_win), FadeIn(right_win), *warmth(1.2), run_time=1.2)
        hold_for(self, N, "intro", used=0.3 + 1.3 + 1.2, during=warmth)

        caption = swap_caption(self, caption, subtitle_text(N, "co2"))
        self.play(FadeIn(co2_col), run_time=0.8)
        self.play(co2.animate.set_value(0.42), run_time=2.2)
        self.play(Indicate(tag_out, color=P_WHITE, scale_factor=1.05), run_time=1.0)
        hold_for(self, N, "co2")

        caption = swap_caption(self, caption, subtitle_text(N, "pettenkofer"))
        self.play(co2.animate.set_value(1.0), recolor_meter(g_co2, P_ORANGE), run_time=4.0)
        self.play(Indicate(tag_pk, color=P_ORANGE, scale_factor=1.05), run_time=1.0)
        hold_for(self, N, "pettenkofer")

        caption = swap_caption(self, caption, subtitle_text(N, "feuchte"))
        self.play(FadeIn(moist_col), run_time=0.8)
        self.play(moist.animate.set_value(1.0), run_time=4.0)
        hold_for(self, N, "feuchte")

        caption = swap_caption(self, caption, subtitle_text(N, "geruch"))
        self.play(FadeIn(odor_col), odor.animate.set_value(1.0), run_time=3.0)
        hold_for(self, N, "geruch")

        caption = swap_caption(self, caption, subtitle_text(N, "frisch"))
        lo, hi = left_win.get_center()[0], right_win.get_center()[0]
        lane = [np.array([lo - 0.9, win_y + 0.28, 0.0]), np.array([lo + 0.3, win_y + 0.28, 0.0]),
                np.array([room_c[0], win_y + 0.46, 0.0]), np.array([hi - 0.3, win_y + 0.3, 0.0]), np.array([hi + 0.9, win_y + 0.38, 0.0])]
        fresh = [smooth_path([p + UP * dy for p in lane]) for dy in (0.0, 0.2)]
        fresh_guides = flow_guides(fresh, AIR_COLOR, opacity=0.3)
        tag_in = body_text("Zuluft", font_size=LABEL_FONT_SIZE, color=AIR_COLOR).move_to(np.array([lo - 0.95, win_y - 0.75, 0.0]))
        tag_exit = body_text("Abluft", font_size=LABEL_FONT_SIZE, color=AIR_COLOR).move_to(np.array([hi + 0.62, win_y - 0.75, 0.0]))

        def breeze(rt):
            return [_streams([(fresh, AIR_COLOR)], rt, speed=0.5, waves=4)]

        self.play(FadeIn(fresh_guides), FadeIn(tag_in), FadeIn(tag_exit), *breeze(1.0), run_time=1.0)
        self.play(co2.animate.set_value(0.42), moist.animate.set_value(0.08), odor.animate.set_value(0.06),
                  recolor_meter(g_co2, CO2_COLOR), *breeze(4.0), run_time=4.0)
        hold_for(self, N, "frisch", during=breeze)
        _hide(self, co2_col, moist_col, odor_col, co2_dots, drops, smells, fresh_guides, tag_in, tag_exit, run_time=0.8)

        caption = swap_caption(self, caption, subtitle_text(N, "personen"))
        blocks, texts = VGroup(), VGroup()
        for x in xs:
            blocks.add(Rectangle(width=1.1, height=0.44, color=AIR_COLOR, stroke_width=2, fill_color=AIR_COLOR,
                                 fill_opacity=0.22).move_to(np.array([x, -1.22, 0.0])))
            texts.add(math_label(rf"{de_num(PER_PERSON_M3H)}\,\mathrm{{m^{{3}}/h}}", np.array([x, -1.29, 0.0]),
                                 size=LABEL_FONT_SIZE, color=AIR_COLOR))
        n_tag = math_label(rf"{PEOPLE}\,\times", np.array([-5.75, -1.29, 0.0]), color=P_ORANGE, edge="center")
        k = ValueTracker(0.0)
        v_total = math_readout(lambda: rf"= {de_num(PER_PERSON_M3H * k.get_value())}\,\mathrm{{m^{{3}}/h}}",
                               np.array([-0.2, -1.29, 0.0]), color=P_WHITE, edge="left")
        self.play(FadeIn(n_tag), run_time=0.6)
        self.add(v_total)
        for i, (b, t, p) in enumerate(zip(blocks, texts, people)):
            self.play(FadeIn(b), FadeIn(t), Indicate(p, color=P_ORANGE, scale_factor=1.1), k.animate.set_value(i + 1.0),
                      run_time=3.2 / PEOPLE)
        panel = math_panel([
            (None, r"\dot{V}_{\mathrm{Pers}}", AIR_COLOR), (None, "=", P_WHITE), ("n", f"{PEOPLE}", P_ORANGE),
            (None, r"\cdot", P_WHITE), ("blk", rf"{de_num(PER_PERSON_M3H)}\,\mathrm{{m^{{3}}/h}}", AIR_COLOR),
            (None, "=", P_WHITE), ("tot", rf"{de_num(PER_PERSON_M3H * PEOPLE)}\,\mathrm{{m^{{3}}/h}}", P_WHITE),
        ], color=AIR_COLOR)
        _fill_panel(self, panel, {"n": n_tag, "blk": texts[0], "tot": v_total})
        hold_for(self, N, "personen")

        caption = swap_caption(self, caption, subtitle_text(N, "jahreszeit"))

        def season_card(name: str, color: str, theta: float, icon, at):
            lines = VGroup(body_text(name, font_size=BODY_FONT_SIZE, color=color),
                           math_label(rf"\theta_{{\mathrm{{e}}}} = {de_num(theta)}\,°\mathrm{{C}}", size=LABEL_FONT_SIZE, color=color),
                           math_label(rf"\dot{{V}}_{{\mathrm{{Pers}}}} = {de_num(PER_PERSON_M3H * PEOPLE)}\,\mathrm{{m^{{3}}/h}}",
                                      size=LABEL_FONT_SIZE, color=P_WHITE)).arrange(DOWN, buff=0.14, aligned_edge=LEFT)
            frame = RoundedRectangle(width=3.7, height=1.4, corner_radius=0.12, color=color, stroke_width=2).move_to(at)
            lines.move_to(at + RIGHT * 0.35)
            return VGroup(frame, icon.move_to(at + LEFT * 1.15), lines)

        winter_icon = cloud_glyph(Rectangle(width=0.22, height=0.06), pad=(0.2, 0.12), color=WINTER_COLOR)
        summer_icon = sun_glyph(ORIGIN, color=P_YELLOW).scale(0.34)
        card_w = season_card("Winter", WINTER_COLOR, THETA_WINTER[1], winter_icon, np.array([4.3, 0.95, 0.0]))
        card_s = season_card("Sommer", SUMMER_COLOR, THETA_SUMMER[1], summer_icon, np.array([4.3, -0.55, 0.0]))
        self.play(FadeOut(VGroup(blocks, texts, n_tag, v_total)), run_time=0.6)
        self.play(FadeIn(card_w, shift=LEFT * 0.2), run_time=1.2)
        self.play(FadeIn(card_s, shift=LEFT * 0.2), run_time=1.2)
        hold_for(self, N, "jahreszeit")
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – The air path through the house in winter and summer
class Beat2_Lueftungsweg(Scene):
    """🔀 One house section, two seasons: the stack-driven air path reverses and heat travels with the air both ways.

    https://de.wikipedia.org/wiki/Kamineffekt
    """

    NARRATION = [
        ("intro",
         "Air passes through the openings of a house: it enters and it leaves again.",
         "Luft strömt durch die Öffnungen eines Hauses: Sie tritt ein und wieder aus."),
        ("winter",
         "In winter, warm room air leaves at the top and cold outdoor air enters at the bottom.",
         "Im Winter verlässt warme Raumluft das Haus oben, kalte Außenluft strömt unten nach."),
        ("waerme_winter",
         "Heat leaves with the air: the temperature difference is 20 kelvin.",
         "Mit der Luft geht Wärme verloren: Der Unterschied beträgt 20 K."),
        ("sommer",
         "In summer the flow reverses: hot outdoor air enters at the top, cooled room air leaves at the bottom.",
         "Im Sommer kehrt sich die Strömung um: Heiße Außenluft tritt oben ein, gekühlte Raumluft unten aus."),
        ("waerme_sommer",
         "Now heat enters with the air. The difference is only 6 kelvin.",
         "Jetzt gelangt Wärme mit der Luft hinein. Der Unterschied beträgt nur 6 K."),
        ("beide",
         "Same openings, opposite direction: heat travels with the air both ways.",
         "Gleiche Öffnungen, umgekehrte Richtung: Wärme wandert in beiden Fällen mit der Luft."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Der Lüftungsweg durchs Haus", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hs = house_section(np.array([-1.0, -0.3, 0.0]), scale=1.05)
        bl, br, tl = hs["bottom_left"], hs["bottom_right"], hs["top_left"]
        x_r, t = br[0], 0.1
        air = Polygon(bl + RIGHT * t + UP * 0.02, br + LEFT * t + UP * 0.02, hs["top_right"] + LEFT * t + DOWN * 0.06,
                      hs["roof_peak"] + DOWN * 0.16, tl + RIGHT * t + DOWN * 0.06,
                      stroke_width=0, fill_color=P_ORANGE, fill_opacity=0.0).set_z_index(-1)
        y_mid = hs["level_1"].get_center()[1]
        in_pts, out_pts = [], []
        for win, (yf, yc) in zip(hs["windows"], ((bl[1], y_mid), (y_mid, tl[1]))):
            x_w, sill, lintel = win["x"], win["sill"], win["lintel"]
            for dy in (0.0, 0.11):
                in_pts.append([(x_w - 1.5, sill - 0.08 + dy), (x_w - 0.5, sill + 0.08 + dy), (x_w + 0.35, sill + 0.02 + dy),
                               (x_w + 1.1, yf + 0.2 + dy), (x_r - 0.9, yf + 0.18 + dy), (x_r - 0.3, yf + 0.6 + dy)])
                out_pts.append([(x_r - 0.3, yf + 0.75 - dy), (x_r - 0.55, yc - 0.18 - dy), (x_w + 1.4, yc - 0.15 - dy),
                                (x_w + 0.4, lintel - 0.06 - dy), (x_w - 0.4, lintel - 0.06 - dy), (x_w - 1.0, lintel + 0.28 - dy)])

        def paths(point_lists, reverse: bool = False):
            return [smooth_path([np.array([x, y, 0.0]) for x, y in (pts[::-1] if reverse else pts)]) for pts in point_lists]

        winter_in, winter_out = paths(in_pts), paths(out_pts)
        summer_in, summer_out = paths(out_pts, True), paths(in_pts, True)
        guides_winter = VGroup(flow_guides(winter_in[::2], WINTER_COLOR, opacity=0.36), flow_guides(winter_out[::2], P_ORANGE, opacity=0.36))
        guides_summer = VGroup(flow_guides(summer_in[::2], SUMMER_COLOR, opacity=0.36), flow_guides(summer_out[::2], P_CYAN, opacity=0.36))
        season = {"winter": [(winter_in, WINTER_COLOR, P_ORANGE), (winter_out, P_ORANGE)],
                  "summer": [(summer_in, SUMMER_COLOR, P_ORANGE), (summer_out, P_ORANGE, P_CYAN)]}
        mode = {"now": "winter"}

        def breeze(rt, waves=3):
            return [_streams(season[mode["now"]], rt, speed=0.45, waves=waves, radius=0.09)]

        def zone_labels(inner: str, outer: str, c_in: str, c_out: str):
            inside = VGroup(body_text("Innen", font_size=BODY_FONT_SIZE, color=c_in),
                            body_text(inner, font_size=LABEL_FONT_SIZE, color=c_in)).arrange(DOWN, buff=0.12)
            inside.move_to(np.array([0.5 * (bl[0] + x_r) + 0.25, 0.5 * (bl[1] + y_mid), 0.0]))
            outside = VGroup(body_text("Außen", font_size=BODY_FONT_SIZE, color=c_out),
                             body_text(outer, font_size=LABEL_FONT_SIZE, color=c_out)).arrange(DOWN, buff=0.12)
            outside.move_to(np.array([-5.7, y_mid, 0.0]))
            return inside, outside

        label_in_w, label_out_w = zone_labels("warm", "kalt", P_ORANGE, WINTER_COLOR)
        label_in_s, label_out_s = zone_labels("kühl", "heiß", P_CYAN, SUMMER_COLOR)

        def lvl(theta: float) -> float:
            return (theta + 10.0) / 50.0

        therm_x = {"e": 5.15, "i": 6.3}
        therm_h = 1.9
        therm_e = thermometer_glyph(np.array([therm_x["e"], -1.1, 0.0]), height=therm_h, color=P_BLUE, level=0.0)
        therm_i = thermometer_glyph(np.array([therm_x["i"], -1.1, 0.0]), height=therm_h, color=P_ORANGE, level=0.0)
        theta_e, theta_i = ValueTracker(0.0), ValueTracker(0.0)

        def top_y(tracker):
            return -1.1 + 0.05 + (therm_h - 0.12) * float(np.clip(lvl(tracker.get_value()), 0, 1))

        cap_y = -1.1 + therm_h + 0.3
        read_e = math_readout(lambda: rf"{de_num(theta_e.get_value())}\,°\mathrm{{C}}", np.array([therm_x["e"], cap_y, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_BLUE, edge="center")
        read_i = math_readout(lambda: rf"{de_num(theta_i.get_value())}\,°\mathrm{{C}}", np.array([therm_x["i"], cap_y, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_ORANGE, edge="center")
        sym_e = math_label(r"\theta_{\mathrm{e}}", np.array([therm_x["e"], -1.62, 0.0]), size=BODY_FONT_SIZE, color=P_BLUE)
        sym_i = math_label(r"\theta_{\mathrm{i}}", np.array([therm_x["i"], -1.62, 0.0]), size=BODY_FONT_SIZE, color=P_ORANGE)
        mid_x = 0.5 * (therm_x["e"] + therm_x["i"])
        gap_arrow = always_redraw(lambda: dim_arrow(np.array([mid_x, top_y(theta_e), 0.0]), np.array([mid_x, top_y(theta_i), 0.0]),
                                                    color=P_YELLOW))
        gap_guides = always_redraw(lambda: VGroup(*[DashedLine(np.array([therm_x[k] + 0.14, top_y(tr), 0.0]), np.array([mid_x, top_y(tr), 0.0]),
                                                               color=P_YELLOW, stroke_width=1.6, dash_length=0.06)
                                                    for k, tr in (("e", theta_e), ("i", theta_i))]))
        therm_group = VGroup(therm_e["group"], therm_i["group"], sym_e, sym_i, read_e, read_i)
        therm_cols = VGroup(therm_e["column"], therm_i["column"])

        badge_pos = np.array([3.0, 1.3, 0.0])
        cloud_label = body_text("Winter", font_size=BODY_FONT_SIZE, color=WINTER_COLOR).move_to(badge_pos)
        badge_winter = VGroup(cloud_glyph(cloud_label, color=WINTER_COLOR), cloud_label)
        sun_label = body_text("Sommer", font_size=BODY_FONT_SIZE, color=SUMMER_COLOR)
        sun = sun_glyph(ORIGIN, color=P_YELLOW).scale(0.4)
        badge_summer = VGroup(sun, sun_label.next_to(sun, DOWN, buff=0.1)).move_to(badge_pos)

        def q_arrow(direction: float, color: str):
            start, end = np.array([-4.0, 1.55, 0.0]), np.array([-5.3, 1.55, 0.0])
            if direction > 0:
                start, end = end, start
            arrow = Arrow(start, end, buff=0, color=color, stroke_width=5, max_tip_length_to_length_ratio=0.3)
            tag = math_label(r"\dot{Q}_{\mathrm{V}}", np.array([-4.65, 1.88, 0.0]), size=BODY_FONT_SIZE, color=color)
            return VGroup(arrow, tag)

        q_winter, q_summer = q_arrow(-1.0, P_ORANGE), q_arrow(1.0, P_RED)

        self.play(Create(hs["group"]), run_time=1.5)
        self.add(air)
        self.play(*[open_window(w) for w in hs["windows"]], FadeIn(guides_winter), run_time=0.9)
        self.play(*breeze(1.2), run_time=1.2)
        hold_for(self, N, "intro", used=0.3 + 1.5 + 0.9 + 1.2, during=breeze)

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        therm_e["level"].add_updater(lambda m: m.set_value(lvl(theta_e.get_value())))
        therm_i["level"].add_updater(lambda m: m.set_value(lvl(theta_i.get_value())))
        self.add(therm_e["level"], therm_i["level"], therm_cols, gap_guides, gap_arrow)
        self.play(FadeIn(label_in_w), FadeIn(label_out_w), FadeIn(badge_winter), FadeIn(therm_group),
                  air.animate.set_fill(P_ORANGE, opacity=0.12), *breeze(1.0), run_time=1.0)
        self.play(theta_e.animate.set_value(THETA_WINTER[1]), theta_i.animate.set_value(THETA_WINTER[0]), *breeze(2.0),
                  run_time=2.0)
        hold_for(self, N, "winter", during=breeze)

        caption = swap_caption(self, caption, subtitle_text(N, "waerme_winter"))
        read_dt = math_readout(lambda: rf"\Delta\theta = {de_num(abs(theta_i.get_value() - theta_e.get_value()))}\,\mathrm{{K}}",
                               np.array([mid_x, -2.1, 0.0]), color=P_YELLOW, edge="center")
        self.play(FadeIn(q_winter), FadeIn(read_dt), *breeze(1.6), run_time=1.6)
        self.play(Circumscribe(read_dt, color=P_YELLOW, fade_out=True), *breeze(1.2), run_time=1.2)
        hold_for(self, N, "waerme_winter", during=breeze)

        caption = swap_caption(self, caption, subtitle_text(N, "sommer"))
        mode["now"] = "summer"
        self.play(FadeOut(guides_winter), FadeOut(q_winter), FadeOut(label_in_w), FadeOut(label_out_w), run_time=0.6)
        self.play(ReplacementTransform(badge_winter, badge_summer), FadeIn(guides_summer), FadeIn(label_in_s), FadeIn(label_out_s),
                  air.animate.set_fill(P_CYAN, opacity=0.1),
                  theta_e.animate.set_value(THETA_SUMMER[1]), theta_i.animate.set_value(THETA_SUMMER[0]),
                  *breeze(2.4), run_time=2.4)
        hold_for(self, N, "sommer", during=breeze)

        caption = swap_caption(self, caption, subtitle_text(N, "waerme_sommer"))
        self.play(FadeIn(q_summer), Circumscribe(read_dt, color=P_YELLOW, fade_out=True), *breeze(1.8), run_time=1.8)
        hold_for(self, N, "waerme_sommer", during=breeze)

        caption = swap_caption(self, caption, subtitle_text(N, "beide"))
        self.play(Indicate(q_summer, color=P_RED, scale_factor=1.12), *breeze(1.6), run_time=1.6)
        hold_for(self, N, "beide", during=lambda rt: breeze(rt, 4))
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Roadmap of the video
class Beat3_Fahrplan(Scene):
    """🗺️ Roadmap of the video as five cards that light up in order, ending on the hand-over chips for Heizung and Kühlung."""

    NARRATION = [
        ("intro",
         "This video follows five steps.",
         "Dieses Video folgt fünf Schritten."),
        ("menge",
         "First, how much air: volume, air change rate and volume flow.",
         "Zuerst die Luftmenge: Volumen, Luftwechselrate und Volumenstrom."),
        ("waerme",
         "Then, what the air carries: temperature difference, heat capacity and humidity.",
         "Dann, was die Luft trägt: Temperaturdifferenz, Wärmekapazität und Feuchte."),
        ("natuerlich",
         "Third, natural ventilation through windows.",
         "Drittens die natürliche Lüftung durch Fenster."),
        ("mechanisch",
         "Fourth, mechanical ventilation with heat recovery.",
         "Viertens die mechanische Lüftung mit Wärmerückgewinnung."),
        ("uebergabe",
         "Last, the hand-over: heating takes the winter value, cooling the summer value.",
         "Zuletzt die Übergabe: Die Heizung übernimmt den Winterwert, die Kühlung den Sommerwert."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Der Fahrplan des Videos", title)
        din = _din_ref("DIN EN 12831-1 · DIN 1946-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        card_w, card_h, gap, y_c = 2.25, 2.7, 0.4, 0.6
        xs = [(i - 2) * (card_w + gap) for i in range(5)]
        colors = [AIR_COLOR, P_YELLOW, P_GREEN, P_TEAL, P_ORANGE]
        names = ["Luftmenge", "Wärmestrom", "natürlich", "mechanisch", "Übergabe"]
        symbols = [r"V,\;n,\;\dot{V}", r"\Delta\theta,\;c_{\mathrm{Luft}},\;\Delta x", r"A_{\mathrm{eff}}",
                   r"\eta_{\mathrm{WRG}}", r"\Phi_{\mathrm{V}},\;\dot{Q}_{\mathrm{L}}"]

        def icon(i: int, color: str, at):
            if i == 1:
                therm = thermometer_glyph(at + DOWN * 0.34, height=0.8, color=color, level=0.65)
                return VGroup(therm["group"], therm["column"])
            if i == 0:
                room = _room(ORIGIN, 1.0, 0.62, color=color, t=0.07)
                path = smooth_path([np.array([-0.62, 0.05, 0.0]), np.array([0.0, 0.12, 0.0]), np.array([0.62, 0.05, 0.0])])
                return VGroup(room, flow_guides([path], color, opacity=0.9, width=2.4))
            if i == 2:
                win = Rectangle(width=0.55, height=0.8, color=color, stroke_width=2.2)
                return VGroup(win, Line(win.get_top(), win.get_bottom(), color=color, stroke_width=1.6),
                              Line(win.get_left(), win.get_right(), color=color, stroke_width=1.6))
            if i == 3:
                return _fan(ORIGIN, color, radius=0.4)
            roof = Polygon(np.array([-0.5, 0.0, 0.0]), np.array([0.0, 0.42, 0.0]), np.array([0.5, 0.0, 0.0]), color=color, stroke_width=2.2)
            body = Rectangle(width=0.8, height=0.4, color=color, stroke_width=2.2).move_to(np.array([0.0, -0.2, 0.0]))
            return VGroup(roof, body)

        cards, shades = [], []
        for i, (x, color) in enumerate(zip(xs, colors)):
            frame = RoundedRectangle(width=card_w, height=card_h, corner_radius=0.14, color=color, stroke_width=2.2)
            frame.move_to(np.array([x, y_c, 0.0]))
            badge = VGroup(Circle(radius=0.19, color=color, stroke_width=2, fill_color=color, fill_opacity=1.0),
                           body_text(str(i + 1), font_size=LABEL_FONT_SIZE, color=P_DEEP_DARK))
            badge[1].move_to(badge[0].get_center())
            name = body_text(names[i], font_size=BODY_FONT_SIZE, color=color)
            sym = math_label(symbols[i], size=BODY_FONT_SIZE, color=P_WHITE)
            at = np.array([x, y_c - 0.12, 0.0])
            pic = icon(i, color, at)
            badge.move_to(np.array([x, y_c + card_h / 2 - 0.4, 0.0]))
            name.move_to(np.array([x, y_c + card_h / 2 - 0.85, 0.0]))
            if i != 1:
                pic.move_to(at)
            sym.move_to(np.array([x, y_c - card_h / 2 + 0.3, 0.0]), coor_mask=np.array([1, 0, 0]))
            sym.shift(UP * (y_c - card_h / 2 + 0.3 - sym.base.get_center()[1]))
            cards.append(VGroup(frame, badge, name, pic, sym))
            shades.append(RoundedRectangle(width=card_w + 0.04, height=card_h + 0.04, corner_radius=0.14, stroke_width=0,
                                           fill_color=P_DEEP_DARK, fill_opacity=0.74).move_to(frame))
        links = [Arrow(np.array([x + card_w / 2 + 0.03, y_c, 0.0]), np.array([x + card_w / 2 + gap - 0.03, y_c, 0.0]), buff=0,
                       color=P_WHITE, stroke_width=3, max_tip_length_to_length_ratio=0.5) for x in xs[:-1]]

        span_y = y_c - card_h / 2 - 0.25
        bracket = VGroup(Line(np.array([xs[0] - card_w / 2, span_y, 0.0]), np.array([xs[3] + card_w / 2, span_y, 0.0]), color=P_TEAL, stroke_width=2),
                         *[Line(np.array([x, span_y, 0.0]), np.array([x, span_y + 0.12, 0.0]), color=P_TEAL, stroke_width=2)
                           for x in (xs[0] - card_w / 2, xs[3] + card_w / 2)])
        bracket_tag = body_text("gilt im Winter wie im Sommer", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([0.5 * (xs[0] + xs[3]) - 0.4, span_y - 0.32, 0.0]))
        chip_h = _handover(r"\to\;\text{Heizung:}\;\Phi_{\mathrm{V}}", P_ORANGE)
        chip_k = _handover(r"\to\;\text{Kühlung:}\;\dot{Q}_{\mathrm{L}}", P_CYAN)
        chips = VGroup(chip_h, chip_k).arrange(DOWN, buff=0.1, aligned_edge=RIGHT)
        chips.move_to(np.array([xs[4] + card_w / 2 - chips.width / 2, span_y - 0.78, 0.0]))
        chip_h.set_x(chips.get_x())
        chip_k.set_x(chips.get_x())

        self.play(LaggedStart(*[FadeIn(c) for c in cards], lag_ratio=0.12), *[FadeIn(s) for s in shades], run_time=1.6)
        hold_for(self, N, "intro", used=0.3 + 1.6)

        def light(i: int, key: str):
            nonlocal caption
            caption = swap_caption(self, caption, subtitle_text(N, key))
            anims = [FadeOut(shades[i])]
            if i > 0:
                anims.append(GrowArrow(links[i - 1]))
            self.play(*anims, run_time=1.0)
            self.play(Indicate(cards[i][0], color=colors[i], scale_factor=1.05), run_time=1.0)
            hold_for(self, N, key)

        for i, key in enumerate(("menge", "waerme", "natuerlich", "mechanisch")):
            light(i, key)
            if i == 3:
                self.play(Create(bracket), FadeIn(bracket_tag), run_time=1.0)
        light(4, "uebergabe")
        self.play(FadeIn(chip_h, shift=UP * 0.15), FadeIn(chip_k, shift=UP * 0.15), run_time=1.0)
        hold_for(self, N, "uebergabe", min_wait=1.0)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
