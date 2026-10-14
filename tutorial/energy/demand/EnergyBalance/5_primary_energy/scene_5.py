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
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE, body_text,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, SAFE_BOTTOM_FORMULA, CONTENT_TOP,
    note_line, highlight_param, dim_arrow, chip, meter, bind_meter,
    smooth_path, flow_animation, ripples, house_section, person_glyph, radiator, sun_glyph,
    math_label, math_row, math_panel, math_readout, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Primärenergie & das GEG"

NUTZ_BLUE = "#38BDF8"
END_RED = "#DC2626"
PRIM_VIOLET = "#A78BFA"
AMBIENT_GREEN = "#22C55E"
GAS_AMBER = "#F59E0B"
WOOD_BROWN = "#D9A066"

# The schematic worked example carried over from Chapter 4. The plant factors
# mirror ``4_system_losses/scene_4.py`` (E_UEBERGABE · E_VERTEILUNG ·
# E_SPEICHERUNG · E_ERZEUGUNG_*) so final energy here is the value Chapter 4
# puts on screen, not a separately typed number.
Q_NUTZ = 100
E_P_GAS = 1.04 * 1.08 * 1.06 * 1.18   # ≈ 1,40
E_P_WP = 1.04 * 1.08 * 1.06 * 0.26    # ≈ 0,31
Q_END_GAS = round(Q_NUTZ * E_P_GAS)   # 140 kWh/(m²·a)
Q_END_WP = round(Q_NUTZ * E_P_WP)     #  31 kWh/(m²·a)
F_P_GAS, F_P_STROM = 1.1, 1.8          # GEG Anlage 4, nicht erneuerbarer Anteil
F_P_HOLZ, F_P_UMWELT = 0.2, 0.0
# From the *displayed* final energy, so the multiplication on screen checks out
# by hand; half-up because round() is banker's rounding.
Q_PRIM_GAS = int(Q_END_GAS * F_P_GAS + 0.5)    # 154 kWh/(m²·a)
Q_PRIM_WP = int(Q_END_WP * F_P_STROM + 0.5)    #  56 kWh/(m²·a)
Q_LOSS_GAS = Q_END_GAS - Q_NUTZ                #  40 kWh/(m²·a) Anlagenverluste
Q_AMBIENT_WP = Q_NUTZ - Q_END_WP               #  69 kWh/(m²·a) Umweltwärme
Q_CHAIN_GAS = Q_PRIM_GAS - Q_END_GAS           #  14 kWh/(m²·a) Vorkette
Q_CHAIN_WP = Q_PRIM_WP - Q_END_WP              #  25 kWh/(m²·a) Vorkette
# Reference-building value is building-specific; this one is a schematic
# example. The 55 % share is the GEG § 15 requirement for new residential builds.
Q_REF, REF_SHARE = 120, 0.55
Q_LIMIT = int(Q_REF * REF_SHARE + 0.5)          #  66 kWh/(m²·a)
EE_SHARE = 0.65

UNIT = r"\mathrm{kWh/(m^{2}\,a)}"


def _eq(product: float) -> str:
    """🟰 '=' only when the shown product is exact — 140 · 1,1 = 154 but 31 · 1,8 ≈ 56."""
    return "=" if abs(product - round(product)) < 1e-9 else r"\approx"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

#region Buildings and people
def _stand(floor_y, room_h, x, color=P_ORANGE):
    """🧍 Occupant at true scale — 70 % of the inner room height, feet on the floor line at ``x``."""
    height = 0.7 * room_h
    return person_glyph(np.array([x, floor_y + height / 2, 0.0]), color=color, scale=height / 0.66)


def _house(center_x, floor_y, s):
    """🏠 Physical Fundamentals house section standing on ``floor_y``."""
    return house_section(np.array([center_x, floor_y + 1.2 * s, 0.0]), scale=s)


def _furnish(hs, s, person_dx, rad_dx, *, person=True):
    """🛋️ Ground-floor occupant and radiator inside a ``house_section`` — returns both and their warm spot."""
    floor_y, cx = float(hs["bottom_left"][1]), float(hs["center"][0])
    heater = radiator(np.array([cx + rad_dx, floor_y, 0.0]))
    heater.scale(0.75 * s)
    heater.move_to(np.array([cx + rad_dx, floor_y + heater.height / 2 + 0.02, 0.0]))
    figure = _stand(floor_y, 1.2 * s, cx + person_dx) if person else VGroup()
    return {"person": figure, "radiator": heater, "warm": heater.get_top() + UP * 0.03,
            "group": VGroup(figure, heater)}


def _envelope(hs, s, off=0.09):
    """🧥 Thick thermal-envelope outline hugging the house section — the building shell."""
    bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
    return VMobject(color=P_TEAL, stroke_width=6, stroke_opacity=0.85).set_points_as_corners([
        bl + LEFT * off, tl + LEFT * off, tl + LEFT * (0.26 * s + off) + UP * 0.02, peak + UP * off,
        tr + RIGHT * (0.26 * s + off) + UP * 0.02, tr + RIGHT * off, br + RIGHT * off,
    ])


def _ground(x0, x1, y):
    """🟫 Ground line with a few hatches."""
    line = Line(np.array([x0, y, 0.0]), np.array([x1, y, 0.0]), color=P_TEAL, stroke_width=2.2)
    hatches = VGroup(*[
        Line(p, p + (DOWN * 0.09 + LEFT * 0.06), color=P_TEAL, stroke_width=1.0)
        for p in [line.point_from_proportion(u) for u in np.linspace(0.02, 0.98, int((x1 - x0) / 0.33))]
    ])
    return VGroup(line, hatches)


def _ring(x0, x1, y0, y1, color, *, dashed=True):
    """⭕ One balance boundary — the dashed motif Chapter 1 drew around the house."""
    box = RoundedRectangle(corner_radius=0.16, width=x1 - x0, height=y1 - y0, color=color, stroke_width=2.6)
    box.move_to(np.array([(x0 + x1) / 2, (y0 + y1) / 2, 0.0]))
    return DashedVMobject(box, num_dashes=60, dashed_ratio=0.55) if dashed else box
#endregion


#region Plants and sources
def _pt(point):
    """📍 Point as a 3-vector."""
    return np.array([point[0], point[1], 0.0])


def _outline(*mobs, color=P_WHITE, width=1.8):
    """✏️ Shared line style of the plant glyphs."""
    for mob in mobs:
        mob.set_stroke(color=color, width=width)
    return VGroup(*mobs)


def _derrick(base, h=0.85):
    """🛢️ Extraction rig — lattice tower on the ground at ``base``."""
    b = _pt(base)
    apex = b + UP * h
    legs = [Line(b + LEFT * 0.3, apex), Line(b + RIGHT * 0.3, apex)]
    levels = [(0.0, 0.3), (0.3, 0.6), (0.6, 0.85)]
    zig = [Line(b + LEFT * 0.3 * (1 - lo) + UP * h * lo, b + RIGHT * 0.3 * (1 - hi) + UP * h * hi) for lo, hi in levels]
    rungs = [Line(b + LEFT * 0.3 * (1 - f) + UP * h * f, b + RIGHT * 0.3 * (1 - f) + UP * h * f) for f in (0.3, 0.6)]
    return _outline(*legs, *zig, *rungs, width=1.6)


def _power_plant(base, w=0.9, h=0.5):
    """🏭 Conversion plant — hall with two stacks standing on ``base``."""
    b = _pt(base)
    hall = Rectangle(width=w, height=h).move_to(b + UP * h / 2)
    stacks = [Rectangle(width=0.13, height=h * 1.55).move_to(b + RIGHT * (w / 2 - 0.12 - 0.22 * k) + UP * h * 0.775)
              for k in (0, 1)]
    return _outline(hall, *stacks)


def _pump(center, r=0.17):
    """🌀 Compressor or fan — ring with three blades."""
    c = _pt(center)
    ring = Circle(radius=r).move_to(c)
    blades = VGroup(*[Line(c, c + RIGHT * r * 0.8).rotate(a, about_point=c) for a in (0.0, TAU / 3, 2 * TAU / 3)])
    _outline(ring, *blades, width=1.8)
    return VGroup(ring, blades)


def _flame(center, h=0.5, color=P_ORANGE):
    """🔥 Flame drop."""
    c = _pt(center)
    pts = [(0.0, h / 2), (0.3 * h, 0.05 * h), (0.24 * h, -0.34 * h), (0.0, -h / 2), (-0.24 * h, -0.34 * h),
           (-0.3 * h, 0.05 * h), (0.0, h / 2)]
    flame = VMobject(color=color, stroke_width=1.8).set_points_smoothly([c + np.array([x, y, 0.0]) for x, y in pts])
    return flame.set_fill(color, opacity=0.55)


def _tree(base, h=0.5):
    """🌲 Wood carrier — trunk and crown."""
    b = _pt(base)
    trunk = Rectangle(width=0.07, height=0.16).move_to(b + UP * 0.08)
    crown = Polygon(b + UP * 0.14 + LEFT * 0.22, b + UP * 0.14 + RIGHT * 0.22, b + UP * h)
    _outline(trunk, crown, color=WOOD_BROWN)
    return VGroup(trunk, crown)


def _boiler(center):
    """🔥 Gas boiler — casing with its flame."""
    c = _pt(center)
    casing = RoundedRectangle(width=0.5, height=0.62, corner_radius=0.08).move_to(c)
    _outline(casing)
    return VGroup(casing, _flame(c + DOWN * 0.02, 0.36))


def _heat_pump(center):
    """♻️ Air-source heat pump — casing with a fan the outdoor air runs through."""
    c = _pt(center)
    casing = Rectangle(width=0.8, height=0.56).move_to(c)
    _outline(casing)
    fan = _pump(c + LEFT * 0.12, 0.2)
    louvres = VGroup(*[Line(c + RIGHT * 0.22 + UP * dy + LEFT * 0.08, c + RIGHT * 0.22 + UP * dy + RIGHT * 0.08)
                       for dy in (-0.14, 0.0, 0.14)])
    _outline(louvres, width=1.3)
    return {"group": VGroup(casing, fan, louvres), "fan": fan}


def _meter_glyph(center, r=0.17):
    """⏱️ Energy meter — dial with a needle, the point where final energy is counted."""
    c = _pt(center)
    dial = Circle(radius=r, color=P_WHITE, stroke_width=2.0, fill_color=P_DEEP_DARK, fill_opacity=1.0).move_to(c)
    needle = Line(c, c + (RIGHT * 0.8 + UP * 0.6) * r * 0.85, color=P_YELLOW, stroke_width=2.0)
    return VGroup(dial, needle)
#endregion


#region Flows and readouts
def _flow(path, color, rt, *, v=1.5, per_unit=1.5, color_end=None, radius=0.055):
    """💨 Particle stream along ``path`` whose pass count is a whole number, so consecutive plays loop without a jump."""
    length = max(0.4, path.get_arc_length())
    return flow_animation(
        [([path], color, color_end)], waves=max(2, round(length * per_unit)), radius=radius,
        cycles=max(1, round(v * rt / length)), streak=False,
    )


def _heat(spots, rt, r_max=0.45, color=P_ORANGE):
    """🌡️ Heat ripples that keep radiating for ``rt`` seconds."""
    return [ripples(spots, r_max=r_max, cycles=max(1.0, rt / 1.4), color=color)]


def _supply_flow(y, x0, x1, x_loss, kept, lost, counted, color_counted, color_other, rt, *, v=1.5, radius=0.055):
    """⛓️ One carrier's supply chain — ``kept`` particles reach the meter, ``lost`` ones dissipate at ``x_loss``.

    The upstream density is ``(kept + lost) / kept`` times the delivered density — exactly the factor ``f_p``
    for non-renewable carriers; ``counted`` of the ``kept`` particles are the non-renewable share.
    """
    total = kept + lost
    cycles = max(1, round(v * rt / (x1 - x0)))
    dots, meta, seen = VGroup(), [], 0
    for i in range(total):
        is_lost = (i + 1) * lost // total > i * lost // total
        if is_lost:
            colour = PRIM_VIOLET
        else:
            colour = color_counted if (seen + 1) * counted // max(1, kept) > seen * counted // max(1, kept) else color_other
            seen += 1
        dot = Dot(radius=radius, color=colour, stroke_width=0).set_fill(colour, opacity=0.0)
        dots.add(dot)
        meta.append((i, is_lost))

    def update(group, alpha):
        for dot, (i, is_lost) in zip(group, meta):
            t = (alpha * cycles + i / total) % 1.0
            x = x0 + (x1 - x0) * t
            opacity = min(1.0, t / 0.06)
            lift = 0.0
            if is_lost:
                spread = max(0.0, (x - x_loss) / 0.5)
                opacity *= max(0.0, 1.0 - spread)
                lift = spread * 0.3
            else:
                opacity *= min(1.0, (1.0 - t) / 0.06)
            dot.move_to(np.array([x, y + lift, 0.0]))
            dot.set_fill(opacity=0.92 * opacity)

    return UpdateFromAlphaFunc(dots, update, remover=True, rate_func=linear)


def _pipes(*paths):
    """🚰 Pipe bands under the particle streams."""
    return VGroup(*[p.copy().set_stroke(P_WHITE, width=6, opacity=0.2).set_fill(opacity=0) for p in paths])


def _count(tracker, at, color, *, digits=0, size=None, edge="center", prefix=""):
    """🔢 Live number that counts up from its tracker."""
    return math_readout(lambda: prefix + de_num(tracker.get_value(), digits), at, size=size or LABEL_FONT_SIZE,
                        color=color, edge=edge)


def _freeze(scene, readout, src, at, color, *, size=None, edge="center"):
    """🧊 Replace a live readout by the same label typeset once, ready to be morphed."""
    static = math_label(src, np.array(at, dtype=float), size=size or LABEL_FONT_SIZE, color=color, edge=edge)
    scene.remove(readout)
    scene.add(static)
    return static
#endregion


#region Bars
def _bar_rects(value, segments, x, base_y, *, width=0.62, unit=0.01, opacity=0.8):
    """📊 Stacked rectangles for ``value`` — segments fill bottom to top as the value grows."""
    remain, cursor, group = max(0.0, value), 0.0, VGroup()
    for amount, colour in segments:
        take = min(amount, remain)
        if take <= 1e-6:
            break
        h = take * unit
        group.add(Rectangle(width=width, height=h, color=colour, fill_color=colour, fill_opacity=opacity,
                            stroke_width=1.2).move_to(np.array([x, base_y + cursor + h / 2, 0.0])))
        cursor += h
        remain -= take
    return group


def _bar_mob(tracker, segments, x, base_y, *, width=0.62, unit=0.01):
    """📈 Stacked bar whose height follows ``tracker`` every frame."""
    return always_redraw(lambda: _bar_rects(tracker.get_value(), segments, x, base_y, width=width, unit=unit))


def _ghost(x, base_y, value, *, width=0.62, unit=0.01, color=P_WHITE):
    """👻 Dashed outline of a bar height — what the other plant would have reached."""
    box = Rectangle(width=width, height=value * unit, color=color, stroke_width=1.8).move_to(
        np.array([x, base_y + value * unit / 2, 0.0]))
    return DashedVMobject(box, num_dashes=28, dashed_ratio=0.5).set_stroke(opacity=0.7)


def _card(caption, value, color, *, width=None):
    """🪧 Framed caption over a typeset value."""
    cap = body_text(caption, font_size=LABEL_FONT_SIZE, color=P_TEAL)
    body = VGroup(cap, value).arrange(DOWN, buff=0.08)
    frame = RoundedRectangle(
        width=width if width is not None else body.width + 0.36, height=body.height + 0.28, corner_radius=0.1,
        color=color, stroke_width=1.8, fill_color=P_DEEP_DARK, fill_opacity=0.82,
    ).move_to(body)
    return VGroup(frame, body)
#endregion

#endregion


#region Beat 1 – Drei Bilanzgrenzen

class Beat1_DreiBilanzgrenzen(Scene):
    NARRATION = [
        ("intro",
         "Everything in this series has depended on one decision: where we draw the balance boundary.",
         "Alles in dieser Serie hing an einer Entscheidung: wo wir die Bilanzgrenze ziehen."),
        ("raum",
         "Draw it around the room and you count useful energy — the heat the balance said the room needs.",
         "Zieht man sie um den Raum, zählt man Nutzenergie — die Wärme, die die Bilanz für den Raum verlangt."),
        ("gebaeude",
         "Draw it around the building at the meter and you count final energy, plant losses included.",
         "Zieht man sie am Zähler um das Gebäude, zählt man Endenergie — samt Anlagenverlusten."),
        ("quelle",
         "Draw it all the way back to the source and you count primary energy: extraction, conversion and transport as well.",
         "Zieht man sie bis zur Quelle, zählt man Primärenergie — Gewinnung, Umwandlung und Transport inklusive."),
        ("geg",
         "That is why the GEG caps primary energy demand: it makes different energy carriers comparable.",
         "Deshalb begrenzt das GEG den Primärenergiebedarf: Er macht verschiedene Energieträger vergleichbar."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Drei Bilanzgrenzen", title)
        din = _din_ref("DIN V 18599-1:2018-09 · GEG § 20")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        revision_note = note_line(
            "Seit 2025 als DIN/TS 18599-1:2025-10 fortgeschrieben — GEG § 20 nutzt weiterhin 2018",
            color=P_TEAL,
        )
        self.play(FadeIn(caption), FadeIn(revision_note), run_time=0.3)

        G, S, HX = -0.95, 0.75, 0.45
        hs = _house(HX, G, S)
        inside = _furnish(hs, S, -0.65, 0.45)
        peak = float(hs["roof_peak"][1])
        wall_r = HX + 1.8 * S
        annex_x = wall_r + 0.5
        y_pipe, y_ground = G + 0.2, G - 0.22
        meter_x = -1.5

        ground = _ground(-6.8, 3.7, G)
        annex = _outline(
            Line(np.array([wall_r, G + 0.95, 0.0]), np.array([wall_r + 1.05, G + 0.95, 0.0])),
            Line(np.array([wall_r + 1.05, G + 0.95, 0.0]), np.array([wall_r + 1.05, G, 0.0])),
        )
        boiler = _boiler(np.array([annex_x, G + 0.4, 0.0]))
        meter_glyph = _meter_glyph(np.array([meter_x, y_ground, 0.0]))
        derrick = _derrick(np.array([-5.9, G]))
        plant = _power_plant(np.array([-4.3, G]), w=0.95, h=0.5)
        pump = _pump(np.array([-2.75, y_pipe]))
        station_names = VGroup(*[
            body_text(text, font_size=LABEL_FONT_SIZE, color=PRIM_VIOLET).move_to(np.array([x, G + 1.28, 0.0]))
            for text, x in (("Gewinnung", -5.9), ("Umwandlung", -4.3), ("Transport", -2.75))
        ])

        path_up = smooth_path([(-5.9, y_pipe, 0.0), (-4.3, y_pipe, 0.0), (-2.75, y_pipe, 0.0), (-2.3, y_pipe, 0.0),
                               (-1.95, y_ground, 0.0), (meter_x, y_ground, 0.0)])
        path_end = smooth_path([(meter_x, y_ground, 0.0), (HX + 0.4, y_ground, 0.0), (annex_x, y_ground, 0.0),
                                (annex_x, G + 0.1, 0.0)])
        path_use = smooth_path([(annex_x - 0.3, G + 0.18, 0.0), (wall_r, G + 0.18, 0.0),
                                (inside["radiator"].get_right()[0] + 0.05, G + 0.18, 0.0)])
        pipes = _pipes(path_up, path_end, path_use)

        ring_room = _ring(HX - 1.8 * S + 0.12, wall_r - 0.12, G + 0.05, G + 2.4 * S - 0.06, NUTZ_BLUE)
        ring_house = _ring(meter_x - 0.15, annex_x + 0.95, G - 0.36, peak + 0.12, END_RED)
        ring_source = _ring(-6.85, 3.8, G - 0.52, peak + 0.25, PRIM_VIOLET)

        def stage_card(caption_text, name, symbol, color, y):
            name_mob = body_text(name, font_size=LABEL_FONT_SIZE + 1, color=color)
            sym = math_label(symbol, size=BODY_FONT_SIZE, color=color)
            value = VGroup(name_mob, sym).arrange(RIGHT, buff=0.16)
            card = _card(caption_text, value, color, width=2.75)
            card.move_to(np.array([5.45, y, 0.0]))
            return {"card": card, "sym": sym}

        cards = [
            stage_card("Raum", "Nutzenergie", r"Q_{h}", NUTZ_BLUE, 1.2),
            stage_card("Zähler", "Endenergie", r"Q_{E}", END_RED, 0.1),
            stage_card("Quelle", "Primärenergie", r"Q_{P}", PRIM_VIOLET, -1.0),
        ]

        active = []

        def during(rt):
            anims = []
            for make in active:
                anims.extend(make(rt))
            return anims

        active.append(lambda rt: _heat([inside["warm"]], rt, 0.4))

        self.play(FadeOut(revision_note), Create(hs["group"]), Create(ground), FadeIn(inside["group"]),
                  run_time=1.4)
        self.play(Create(annex), FadeIn(boiler), FadeIn(meter_glyph), FadeIn(pipes), run_time=0.9)
        self.play(FadeIn(derrick), FadeIn(plant), FadeIn(pump), *_heat([inside["warm"]], 0.9, 0.4), run_time=0.9)
        hold_for(self, self.NARRATION, "intro",
                 used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 0.9 + 0.9, during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "raum"))
        active.append(lambda rt: [_flow(path_use, NUTZ_BLUE, rt, v=0.9, per_unit=3.0)])
        self.play(Create(ring_room), FadeIn(cards[0]["card"], shift=LEFT * 0.2), *during(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "raum", during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gebaeude"))
        active.append(lambda rt: [_flow(path_end, END_RED, rt, v=1.4, per_unit=1.5)])
        active.append(lambda rt: _heat([np.array([annex_x, G + 0.75, 0.0])], rt, 0.35))
        self.play(Create(ring_house), FadeIn(cards[1]["card"], shift=LEFT * 0.2), *during(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "gebaeude", during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "quelle"))
        active.append(lambda rt: [_flow(path_up, PRIM_VIOLET, rt, v=1.5, per_unit=1.8)])
        active.append(lambda rt: _heat([np.array([-4.3, G + 0.62, 0.0])], rt, 0.4))
        self.play(Create(ring_source), FadeIn(cards[2]["card"], shift=LEFT * 0.2), FadeIn(station_names),
                  *during(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "quelle", during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "geg"))
        row, box, items = math_panel([
            ("qp", r"Q_{P}", PRIM_VIOLET), ("eq", "=", P_WHITE),
            ("qe", r"Q_{E}", END_RED), ("dot", r"\cdot", P_WHITE),
            ("fp", r"f_{p}", P_YELLOW), ("cap", r"\;\text{← vom GEG begrenzt}", P_TEAL),
        ])
        self.play(
            ring_source.animate.set_stroke(width=4.5),
            Create(box), FadeIn(items["eq"]), FadeIn(items["dot"]), FadeIn(items["cap"]),
            TransformFromCopy(cards[2]["sym"], items["qp"]),
            TransformFromCopy(cards[1]["sym"], items["qe"]),
            FadeIn(items["fp"], shift=UP * 0.2),
            Indicate(VGroup(derrick, plant, pump, station_names), color=P_YELLOW, scale_factor=1.04),
            *during(1.4), run_time=1.4,
        )
        ring_p = highlight_param(items, "qp", color=P_YELLOW)
        self.play(Create(ring_p), *during(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "geg", used=1.4 + 0.5, during=during)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(ring_p),
            *[FadeOut(m) for m in (hs["group"], ground, inside["group"], annex, boiler, meter_glyph, derrick,
                                   plant, pump, station_names, pipes, ring_room, ring_house, ring_source)],
            *[FadeOut(c["card"]) for c in cards], run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 2 – Der Primärenergiefaktor

class Beat2_Primaerenergiefaktor(Scene):
    NARRATION = [
        ("intro",
         "The step from final to primary energy is one multiplication, with one number per energy carrier.",
         "Der Schritt von End- zu Primärenergie ist eine Multiplikation — mit einer Zahl je Energieträger."),
        ("strom",
         "Grid electricity carries a high factor among household carriers — its generation mix still has a large fossil share. The value is fixed in the GEG.",
         "Netzstrom trägt unter den Haushalts-Energieträgern einen hohen Faktor — sein Erzeugungsmix hat noch einen großen fossilen Anteil. Der Wert ist im GEG festgelegt."),
        ("gas",
         "Gas and oil sit just above one: the fuel itself is fossil, plus a small share for the supply chain.",
         "Gas und Öl liegen knapp über eins: Der Brennstoff selbst ist fossil, dazu kommt ein kleiner Anteil für die Vorkette."),
        ("holz",
         "Wood is counted at a fifth, and ambient heat a heat pump draws from air or ground counts as zero.",
         "Holz wird mit einem Fünftel angesetzt, und Umweltwärme aus Luft oder Erdreich zählt als null."),
        ("erneuerbar",
         "These are the non-renewable shares — fixed calculation values, not measurements of any individual delivery.",
         "Das sind die nicht erneuerbaren Anteile — festgelegte Rechenwerte, keine Messgrößen einzelner Lieferungen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Der nicht erneuerbare Primärenergiefaktor", title)
        din = _din_ref("GEG Anlage 4")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        X_NAME, X_SRC, X_START, X_METER, X_BAR, UNIT_LEN = -3.75, -3.2, -2.75, 0.35, 1.2, 1.8
        X_LOSS, V = -1.9, 1.5
        rows = [
            ("strom", "Netzstrom", F_P_STROM, P_YELLOW, 1.45),
            ("gas", "Erdgas / Heizöl", F_P_GAS, GAS_AMBER, 0.62),
            ("holz", "Holz (Biomasse)", F_P_HOLZ, WOOD_BROWN, -0.21),
            ("umwelt", "Umweltwärme, Solar", F_P_UMWELT, AMBIENT_GREEN, -1.04),
        ]

        def source_glyph(key, y):
            if key == "strom":
                return _power_plant(np.array([X_SRC, y - 0.24, 0.0]), w=0.5, h=0.3)
            if key == "gas":
                return _flame(np.array([X_SRC, y, 0.0]), 0.5, color=GAS_AMBER)
            if key == "holz":
                return _tree(np.array([X_SRC, y - 0.25, 0.0]), 0.5)
            return sun_glyph(np.array([X_SRC, y, 0.0])).scale(0.3)

        names = {k: body_text(n, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([X_NAME - 1.4, y, 0.0]))
                 .align_to(np.array([X_NAME, y, 0.0]), RIGHT) for k, n, f, c, y in rows}
        glyphs = {k: source_glyph(k, y) for k, n, f, c, y in rows}
        meters = {k: _meter_glyph(np.array([X_METER + 0.2, y, 0.0])) for k, n, f, c, y in rows}
        rails = VGroup(*[
            Line(np.array([X_START, y, 0.0]), np.array([X_METER, y, 0.0]), color=P_WHITE, stroke_width=6,
                 stroke_opacity=0.2) for k, n, f, c, y in rows
        ])
        ref_line = DashedLine(np.array([X_BAR + UNIT_LEN, -1.4, 0.0]), np.array([X_BAR + UNIT_LEN, 1.8, 0.0]),
                              color=P_WHITE, stroke_width=1.8, stroke_opacity=0.6)
        ref_label = math_label(r"Q_{E}=1\,\mathrm{kWh}", np.array([X_BAR + UNIT_LEN - 0.1, 1.84, 0.0]),
                               size=LABEL_FONT_SIZE, color=P_TEAL, edge="right")
        chain_tag = body_text("Vorkette", font_size=LABEL_FONT_SIZE, color=PRIM_VIOLET)
        chain_tag.move_to(np.array([X_BAR + UNIT_LEN + 0.1, 1.84, 0.0])).align_to(
            np.array([X_BAR + UNIT_LEN + 0.1, 1.84, 0.0]), LEFT)
        bar_axis = Line(np.array([X_BAR, -1.4, 0.0]), np.array([X_BAR, 1.8, 0.0]), color=P_WHITE, stroke_width=2,
                        stroke_opacity=0.5)

        trackers = {k: ValueTracker(0.0) for k, *_ in rows}

        def factor_bar(key, y, colour):
            tracker = trackers[key]

            def build():
                v = max(0.0, tracker.get_value())
                group = VGroup()
                for lo, hi, fill in ((0.0, min(v, 1.0), colour), (1.0, v, PRIM_VIOLET)):
                    if hi - lo > 0.004:
                        group.add(Rectangle(width=(hi - lo) * UNIT_LEN, height=0.36, color=fill, fill_color=fill,
                                            fill_opacity=0.78, stroke_width=1.2).move_to(
                            np.array([X_BAR + (lo + hi) / 2 * UNIT_LEN, y, 0.0])))
                return group
            return always_redraw(build)

        bars = {k: factor_bar(k, y, c) for k, n, f, c, y in rows}
        reads = {
            k: math_readout(
                lambda t=trackers[k]: rf"f_{{p}}={de_num(t.get_value(), 1)}",
                lambda t=trackers[k], y=y: np.array([X_BAR + max(t.get_value(), 1.0) * UNIT_LEN + 0.22, y - 0.1, 0.0]),
                size=BODY_FONT_SIZE, color=c, edge="left")
            for k, n, f, c, y in rows
        }
        renewable = {}
        for k, n, f, c, y in rows:
            if f < 1.0:
                rest = Rectangle(width=(1.0 - f) * UNIT_LEN, height=0.36, color=AMBIENT_GREEN, stroke_width=1.8)
                rest.move_to(np.array([X_BAR + (f + 1.0) / 2 * UNIT_LEN, y, 0.0]))
                rest = DashedVMobject(rest, num_dashes=22, dashed_ratio=0.55)
                tag = body_text("erneuerbar", font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN)
                tag.move_to(np.array([X_BAR + (f + 1.0) / 2 * UNIT_LEN, y, 0.0]))
                renewable[k] = VGroup(rest, tag)

        def supply(key, y, rt):
            f = dict((k, ff) for k, n, ff, c, yy in rows)[key]
            kept, lost = 20, int(round(20 * max(0.0, f - 1.0)))
            counted = int(round(20 * min(f, 1.0)))
            colour_counted = dict((k, c) for k, n, ff, c, yy in rows)[key]
            return _supply_flow(y, X_START, X_METER, X_LOSS, kept, lost, counted, colour_counted,
                                AMBIENT_GREEN, rt, v=V)

        active = []

        def during(rt):
            return [make(rt) for make in active]

        row, items = math_row([
            ("qp", r"Q_{P}", PRIM_VIOLET), ("eq", "=", P_WHITE),
            ("qe", r"Q_{E}", END_RED), ("dot", r"\cdot", P_WHITE),
            ("fp", r"f_{p}", P_YELLOW),
        ], font_size=BODY_FONT_SIZE + 4)
        row.move_to(np.array([0.0, 2.32, 0.0]))

        self.play(
            LaggedStart(*[FadeIn(names[k]) for k, *_ in rows], lag_ratio=0.15),
            LaggedStart(*[FadeIn(glyphs[k]) for k, *_ in rows], lag_ratio=0.15),
            FadeIn(rails), FadeIn(VGroup(*meters.values())), Create(bar_axis), Create(ref_line), FadeIn(ref_label),
            run_time=1.4,
        )
        self.play(FadeIn(row, shift=DOWN * 0.15), run_time=0.9)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 0.9)

        for clause, keys in (("strom", ("strom",)), ("gas", ("gas",)), ("holz", ("holz", "umwelt"))):
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, clause))
            for key in keys:
                y = dict((k, yy) for k, n, ff, c, yy in rows)[key]
                active.append(lambda rt, key=key, y=y: supply(key, y, rt))
                self.add(bars[key], reads[key])
            self.play(
                *[trackers[k].animate.set_value(dict((kk, ff) for kk, n, ff, c, yy in rows)[k]) for k in keys],
                *[FadeIn(renewable[k]) for k in keys if k in renewable],
                *([FadeIn(chain_tag)] if clause == "strom" else []),
                *during(1.8), run_time=1.8,
            )
            hold_for(self, self.NARRATION, clause, used=1.8, during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "erneuerbar"))
        note = note_line("f_p nach GEG Anlage 4: nicht erneuerbarer Anteil — festgelegter Rechenwert", color=P_TEAL)
        row2, box2, items2 = math_panel([
            ("qp", r"Q_{P}", PRIM_VIOLET), ("eq", "=", P_WHITE),
            ("qe", r"Q_{E}", END_RED), ("dot", r"\cdot", P_WHITE),
            ("fp", r"f_{p,\mathrm{n.\,ern.}}", P_YELLOW), ("unit", rf"\;[{UNIT}]", P_TEAL),
        ])
        self.play(
            FadeIn(note), Create(box2),
            *[ReplacementTransform(items[k], items2[k]) for k in ("qp", "eq", "qe", "dot", "fp")],
            FadeIn(items2["unit"]),
            Indicate(VGroup(*reads.values()), color=P_YELLOW, scale_factor=1.06),
            *during(1.2), run_time=1.2,
        )
        ring = highlight_param(items2, "fp", color=P_YELLOW)
        self.play(Create(ring), *during(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "erneuerbar", used=1.2 + 0.5, during=during)

        self.play(
            FadeOut(caption), FadeOut(ring), FadeOut(box2), FadeOut(row2), FadeOut(note),
            FadeOut(VGroup(*names.values())), FadeOut(VGroup(*glyphs.values())), FadeOut(rails),
            FadeOut(VGroup(*meters.values())), FadeOut(bar_axis), FadeOut(ref_line), FadeOut(ref_label),
            FadeOut(chain_tag), *[FadeOut(m) for m in list(bars.values()) + list(reads.values()) + list(renewable.values())],
            run_time=0.6,
        )
        self.wait(0.4)

#endregion


#region Beat 3 – Ein Gebäude, zwei Anlagen

G3, S3, U3 = -0.75, 0.55, 0.0100


def _plant_panel(px, kind):
    """🏠 One house on one plant: bars at the three boundaries, the plant between them, and the ``f_p`` gauge.

    Everything sits on the same ground line, so the two panels read as columns that can be compared by eye.
    """
    gas = kind == "gas"
    xs = {"h": px + 2.85, "plant": px + 3.95, "e": px + 5.0, "q": px + 6.1}
    y_b = G3 - 0.3
    hs = _house(px + 1.1, G3, S3)
    inside = _furnish(hs, S3, -0.55, 0.45)
    rad_x = float(inside["radiator"].get_center()[0])
    colour = GAS_AMBER if gas else AMBIENT_GREEN
    head = body_text("Gaskessel" if gas else "Wärmepumpe", font_size=BODY_FONT_SIZE, color=colour)
    head.move_to(np.array([px + 1.1, 1.5, 0.0]))
    fan = None
    if gas:
        plant = _boiler(np.array([xs["plant"], G3 + 0.36, 0.0]))
    else:
        heat_pump = _heat_pump(np.array([xs["plant"], G3 + 0.31, 0.0]))
        plant, fan = heat_pump["group"], heat_pump["fan"]
    meter_glyph = _meter_glyph(np.array([xs["e"], y_b, 0.0]), 0.14)
    p_up = Line(np.array([xs["q"], y_b, 0.0]), np.array([xs["e"] + 0.14, y_b, 0.0]))
    p_end = smooth_path([(xs["e"] - 0.14, y_b, 0.0), (xs["plant"] + 0.2, y_b, 0.0), (xs["plant"] + 0.2, G3 + 0.04, 0.0)])
    p_use = smooth_path([(xs["plant"] - 0.2, G3 + 0.04, 0.0), (xs["plant"] - 0.2, y_b, 0.0), (rad_x, y_b, 0.0),
                         (rad_x, G3 + 0.1, 0.0)])
    p_air = Line(np.array([xs["plant"], 0.72, 0.0]), np.array([xs["plant"], G3 + 0.62, 0.0]))
    sun = None if gas else sun_glyph(np.array([xs["plant"], 1.0, 0.0])).scale(0.3)
    trackers = {k: ValueTracker(0.0) for k in ("h", "e", "q")}
    plain = [(Q_NUTZ, NUTZ_BLUE)]
    split = [(Q_END_WP, NUTZ_BLUE), (Q_AMBIENT_WP, AMBIENT_GREEN)]
    stacks = {
        "e": [(Q_NUTZ, NUTZ_BLUE), (Q_LOSS_GAS, P_ORANGE)] if gas else [(Q_END_WP, NUTZ_BLUE)],
        "q": [(Q_NUTZ, NUTZ_BLUE), (Q_LOSS_GAS, P_ORANGE), (Q_CHAIN_GAS, PRIM_VIOLET)] if gas
        else [(Q_END_WP, NUTZ_BLUE), (Q_CHAIN_WP, PRIM_VIOLET)],
    }
    bars = {"h": _bar_mob(trackers["h"], plain, xs["h"], G3, unit=U3),
            "e": _bar_mob(trackers["e"], stacks["e"], xs["e"], G3, unit=U3),
            "q": _bar_mob(trackers["q"], stacks["q"], xs["q"], G3, unit=U3)}
    stage = {"h": (r"Q_{h}", NUTZ_BLUE), "e": (r"Q_{E}", END_RED), "q": (r"Q_{P}", PRIM_VIOLET)}

    def top(key):
        return G3 + max(0.0, trackers[key].get_value()) * U3

    reads = {}
    for key, (symbol, tint) in stage.items():
        reads[key] = {
            "num": math_readout(lambda t=trackers[key]: de_num(t.get_value()),
                                lambda k=key: np.array([xs[k], top(k) + 0.2, 0.0]),
                                size=LABEL_FONT_SIZE, color=tint, edge="center"),
            "sym": math_readout(lambda s=symbol: s, lambda k=key: np.array([xs[k], top(k) + 0.58, 0.0]),
                                size=LABEL_FONT_SIZE, color=tint, edge="center"),
        }

    gauge = meter("f", length=1.6, thickness=0.26, color=P_YELLOW, vertical=False)
    gauge["group"].remove(gauge["label"])
    gauge["group"].move_to(np.array([px + 5.65, 1.85, 0.0]))
    share = ValueTracker(0.0)
    bind_meter(gauge, share)
    tick = Line(gauge["track"].get_top() + DOWN * 0.04, gauge["track"].get_bottom() + UP * 0.04,
                color=P_WHITE, stroke_width=1.6, stroke_opacity=0.7)
    gauge_read = math_readout(lambda: rf"f_{{p}}={de_num(2.0 * share.get_value(), 1)}",
                              np.array([px + 4.75, 1.78, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW, edge="right")
    return {
        "kind": kind, "xs": xs, "y_b": y_b, "hs": hs, "inside": inside, "ground": _ground(px, px + 6.6, G3),
        "head": head, "plant": plant, "fan": fan, "meter": meter_glyph, "sun": sun,
        "paths": {"up": p_up, "end": p_end, "use": p_use, "air": p_air}, "pipes": _pipes(p_up, p_end, p_use),
        "trackers": trackers, "bars": bars, "reads": reads, "split": split, "plain": plain, "stacks": stacks,
        "gauge": gauge, "tick": tick, "share": share, "gauge_read": gauge_read, "colour": colour,
        "static": {},
    }


class Beat3_Rechenbeispiel(Scene):
    NARRATION = [
        ("intro",
         "Now the whole chain on one building — the same hundred kilowatt-hours of useful heat as in the last chapter.",
         "Jetzt die ganze Kette an einem Gebäude — dieselben hundert Kilowattstunden Nutzwärme wie im letzten Kapitel."),
        ("end",
         "The plant turns that into final energy: a hundred and forty of gas, or thirty-one of electricity.",
         "Die Anlage macht daraus Endenergie: hundertvierzig Gas oder einunddreißig Strom."),
        ("prim",
         "Now apply the factors. Gas is multiplied by one point one, electricity by one point eight.",
         "Jetzt die Faktoren: Gas mal eins Komma eins, Strom mal eins Komma acht."),
        ("ergebnis",
         "A hundred and fifty-four against fifty-six. The electricity factor is higher, yet the heat pump ends up far lower.",
         "Hundertvierundfünfzig gegen sechsundfünfzig. Der Stromfaktor ist höher — und die Wärmepumpe liegt trotzdem weit darunter."),
        ("warum",
         "Because the factor is applied to a number that was already more than three quarters smaller. Efficiency beats the factor.",
         "Weil der Faktor auf eine Zahl trifft, die schon gut drei Viertel kleiner war. Effizienz schlägt den Faktor."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Ein Gebäude, zwei Anlagen", title)
        din = _din_ref("GEG § 20 · DIN V 18599:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        schematic_note = note_line("Vereinfachtes Rechenbeispiel — keine vollständige DIN/GEG-Berechnung", color=P_TEAL)
        self.play(FadeIn(caption), FadeIn(schematic_note), run_time=0.3)

        gas, wp = _plant_panel(-6.85, "gas"), _plant_panel(0.1, "wp")
        panels = (gas, wp)

        legend_unit = math_label(rf"[{UNIT}]", size=LABEL_FONT_SIZE, color=P_TEAL)
        words = {key: body_text(text, font_size=LABEL_FONT_SIZE, color=colour)
                 for key, text, colour in (("nutz", "Nutzwärme", NUTZ_BLUE), ("loss", "Anlagenverlust", P_ORANGE),
                                           ("env", "Umweltwärme", AMBIENT_GREEN), ("chain", "Vorkette", PRIM_VIOLET))}
        legend = VGroup(legend_unit, *words.values()).arrange(RIGHT, buff=0.45).move_to(np.array([0.0, 2.3, 0.0]))

        streams = []

        def during(rt):
            anims = []
            for p in panels:
                anims.extend(_heat([p["inside"]["warm"]], rt, 0.35))
            for make in streams:
                anims.extend(make(rt))
            return anims

        def freeze(p, key, *, base_src, color):
            x = p["xs"][key]
            value = p["trackers"][key].get_value()
            top_y = G3 + value * U3
            num = _freeze(self, p["reads"][key]["num"], de_num(value), (x, top_y + 0.2), color)
            sym = _freeze(self, p["reads"][key]["sym"], base_src, (x, top_y + 0.58), color)
            p["static"][key] = {"num": num, "sym": sym}

        self.play(
            *[Create(p["hs"]["group"]) for p in panels], *[Create(p["ground"]) for p in panels],
            *[FadeIn(p["inside"]["group"]) for p in panels], *[FadeIn(p["head"]) for p in panels], run_time=1.5,
        )
        for p in panels:
            self.add(p["bars"]["h"], p["reads"]["h"]["num"], p["reads"]["h"]["sym"])
        self.play(*[p["trackers"]["h"].animate.set_value(Q_NUTZ) for p in panels], *during(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 1.6, during=during)
        for p in panels:
            freeze(p, "h", base_src=r"Q_{h}", color=NUTZ_BLUE)
            rect = _bar_rects(Q_NUTZ, p["plain"], p["xs"]["h"], G3, unit=U3)
            self.remove(p["bars"]["h"])
            self.add(rect)
            p["rect_h"] = rect

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "end"))
        for p in panels:
            xs = p["xs"]
            streams.append(lambda rt, p=p: [_flow(p["paths"]["end"], END_RED, rt, v=1.3, per_unit=2.2)])
            if p["kind"] == "gas":
                streams.append(lambda rt, p=p: [_flow(p["paths"]["use"], NUTZ_BLUE, rt, v=1.3, per_unit=2.2)])
                streams.append(lambda rt, p=p: _heat([np.array([xs["plant"], G3 + 0.75, 0.0])], rt, 0.3))
            else:
                streams.append(lambda rt, p=p: [_flow(p["paths"]["use"], NUTZ_BLUE, rt, v=1.3, per_unit=1.0),
                                                _flow(p["paths"]["use"], AMBIENT_GREEN, rt, v=1.3, per_unit=2.3)])
                streams.append(lambda rt, p=p: [_flow(p["paths"]["air"], AMBIENT_GREEN, rt, v=0.9, per_unit=3.0)])
                streams.append(lambda rt, p=p: [Rotate(p["fan"][1], angle=-TAU * rt / 0.9,
                                                      about_point=p["fan"][0].get_center(), rate_func=linear)])
            self.add(p["bars"]["e"], p["reads"]["e"]["num"], p["reads"]["e"]["sym"])
        split_bar = _bar_rects(Q_NUTZ, wp["split"], wp["xs"]["h"], G3, unit=U3)
        self.play(
            ReplacementTransform(schematic_note, legend_unit),
            *[FadeIn(p["plant"]) for p in panels], *[FadeIn(p["pipes"]) for p in panels],
            *[FadeIn(p["meter"]) for p in panels], FadeIn(wp["sun"]),
            FadeIn(words["nutz"]), FadeIn(words["loss"]), FadeIn(words["env"]),
            *[p["trackers"]["e"].animate.set_value(v) for p, v in ((gas, Q_END_GAS), (wp, Q_END_WP))],
            ReplacementTransform(wp["rect_h"], split_bar),
            *during(2.2), run_time=2.2,
        )
        wp["rect_h"] = split_bar
        hold_for(self, self.NARRATION, "end", used=2.2, during=during)
        for p in panels:
            freeze(p, "e", base_src=r"Q_{E}", color=END_RED)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "prim"))
        for p in panels:
            streams.append(lambda rt, p=p: [_flow(p["paths"]["up"], PRIM_VIOLET, rt, v=1.0, per_unit=2.4)])
            p["trackers"]["q"].set_value(Q_END_GAS if p is gas else Q_END_WP)
            self.add(p["bars"]["q"], p["reads"]["q"]["num"], p["reads"]["q"]["sym"])
            self.add(p["tick"], p["gauge"]["track"], p["gauge"]["fill"], p["gauge_read"])
        self.play(
            *[FadeIn(m) for p in panels for m in (p["gauge"]["track"], p["tick"], p["gauge_read"])],
            FadeIn(words["chain"]),
            *during(0.6), run_time=0.6,
        )
        self.play(
            gas["share"].animate.set_value(F_P_GAS / 2.0), wp["share"].animate.set_value(F_P_STROM / 2.0),
            gas["trackers"]["q"].animate.set_value(Q_PRIM_GAS), wp["trackers"]["q"].animate.set_value(Q_PRIM_WP),
            *during(2.0), run_time=2.0,
        )
        hold_for(self, self.NARRATION, "prim", used=2.6, during=during)
        for p in panels:
            freeze(p, "q", base_src=r"Q_{P}", color=PRIM_VIOLET)
            factor = F_P_GAS if p is gas else F_P_STROM
            p["static"]["f"] = _freeze(self, p["gauge_read"], rf"f_{{p}}={de_num(factor, 1)}",
                                       (p["xs"]["e"] - 0.25 + 0.0, 1.78), P_YELLOW, edge="right")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ergebnis"))
        rings = VGroup(
            SurroundingRectangle(VGroup(gas["static"]["q"]["num"]), color=P_RED, buff=0.08, corner_radius=0.05,
                                 stroke_width=2.6),
            SurroundingRectangle(VGroup(wp["static"]["q"]["num"]), color=AMBIENT_GREEN, buff=0.08, corner_radius=0.05,
                                 stroke_width=2.6),
        )
        ghost_q = _ghost(wp["xs"]["q"], G3, Q_PRIM_GAS, unit=U3)
        self.play(
            Create(rings), FadeIn(ghost_q),
            Indicate(VGroup(wp["gauge"]["track"], wp["gauge"]["fill"], wp["static"]["f"]), color=P_YELLOW,
                     scale_factor=1.12),
            *during(1.4), run_time=1.4,
        )
        hold_for(self, self.NARRATION, "ergebnis", used=1.4, during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "warum"))
        ghost_e = _ghost(wp["xs"]["e"], G3, Q_END_GAS, unit=U3)
        parts = [
            ("g_qe", de_num(Q_END_GAS), END_RED), ("g_dot", r"\cdot", P_WHITE),
            ("g_fp", de_num(F_P_GAS, 1), P_YELLOW), ("g_eq", _eq(Q_END_GAS * F_P_GAS), P_WHITE),
            ("g_qp", de_num(Q_PRIM_GAS), PRIM_VIOLET), (None, r"\quad", P_WHITE),
            ("w_qe", de_num(Q_END_WP), END_RED), ("w_dot", r"\cdot", P_WHITE),
            ("w_fp", de_num(F_P_STROM, 1), P_YELLOW), ("w_eq", _eq(Q_END_WP * F_P_STROM), P_WHITE),
            ("w_qp", de_num(Q_PRIM_WP), PRIM_VIOLET), ("unit", rf"\;{UNIT}", P_TEAL),
        ]
        row, box, items = math_panel(parts)
        copies = []
        for p, tag in ((gas, "g"), (wp, "w")):
            copies += [
                TransformFromCopy(p["static"]["e"]["num"], items[f"{tag}_qe"]),
                TransformFromCopy(p["static"]["f"], items[f"{tag}_fp"]),
                TransformFromCopy(p["static"]["q"]["num"], items[f"{tag}_qp"]),
                FadeIn(items[f"{tag}_dot"]), FadeIn(items[f"{tag}_eq"]),
            ]
        self.play(FadeOut(rings), FadeIn(ghost_e), Create(box), FadeIn(items["unit"]), *copies,
                  *during(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "warum", used=1.6, during=during)

        everything = [
            caption, box, row, ghost_q, ghost_e, legend_unit, *words.values(), wp["sun"],
            *[m for p in panels for m in (
                p["hs"]["group"], p["ground"], p["inside"]["group"], p["head"], p["plant"], p["pipes"], p["meter"],
                p["rect_h"], p["bars"]["e"], p["bars"]["q"], p["gauge"]["track"], p["gauge"]["fill"], p["tick"],
                *[m for tag in p["static"].values() for m in (tag.values() if isinstance(tag, dict) else [tag])],
            )],
        ]
        self.play(*[FadeOut(m) for m in everything], run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 4 – Der Grenzwert: das Referenzgebäude

G4, S4, U4 = -0.45, 0.62, 0.0125


class Beat4_Referenzgebaeude(Scene):
    NARRATION = [
        ("intro",
         "So what does the law compare those numbers against? Not a flat limit — against a reference building with your own geometry.",
         "Womit vergleicht das Gesetz diese Zahlen? Nicht mit einem pauschalen Grenzwert — sondern mit einem Referenzgebäude Ihrer Geometrie."),
        ("referenz",
         "It takes the identical geometry and fills it with the technical reference specification from Annex 1: the reference building.",
         "Es nimmt dieselbe Geometrie und füllt sie mit der technischen Referenzausführung nach Anlage 1: das Referenzgebäude."),
        ("grenze",
         "Its calculated primary energy sets the benchmark — a new residential building may reach at most fifty-five percent of it.",
         "Dessen Primärenergiebedarf setzt den Maßstab — ein neues Wohngebäude darf höchstens fünfundfünfzig Prozent davon erreichen."),
        ("ergebnis",
         "In this example the heat pump clears the line; the gas boiler misses it by more than a factor of two.",
         "Im Beispiel unterschreitet die Wärmepumpe die Linie — der Gaskessel verfehlt sie um mehr als das Doppelte."),
        ("zusatz",
         "The GEG also demands thermal protection of the envelope, and new heating systems — with transition periods — 65 percent renewable energy.",
         "Daneben verlangt das GEG baulichen Wärmeschutz und für neue Heizungen, mit Übergangsfristen, 65 Prozent erneuerbare Energie."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Der Grenzwert: das Referenzgebäude", title)
        din = _din_ref("GEG §§ 15–17 · § 71 · Anlage 1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        design = _house(-5.25, G4, S4)
        reference = _house(-1.95, G4, S4)
        design_in = _furnish(design, S4, -0.55, 0.45)
        reference_in = _furnish(reference, S4, -0.55, 0.45)
        ground = _ground(-6.9, 6.7, G4)

        def dims(hs):
            tl, bl, tr = hs["top_left"], hs["bottom_left"], hs["top_right"]
            vertical = dim_arrow(np.array([bl[0] - 0.4, bl[1], 0.0]), np.array([bl[0] - 0.4, tl[1], 0.0]), P_YELLOW)
            horizontal = dim_arrow(np.array([0.0, 0.0, 0.0]), np.array([0.0, tr[0] - tl[0], 0.0]), P_YELLOW)
            horizontal.rotate(-PI / 2).move_to(np.array([(tl[0] + tr[0]) / 2, hs["roof_peak"][1] + 0.28, 0.0]))
            return VGroup(vertical, horizontal)

        dims_d, dims_r = dims(design), dims(reference)
        envelope_d, envelope_r = _envelope(design, S4), _envelope(reference, S4)
        tag_d = body_text("Ihr Entwurf", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(np.array([-5.25, -0.95, 0.0]))
        tag_r = body_text("Referenzgebäude", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(
            np.array([-1.95, -0.95, 0.0]))
        spec = chip("Anlage 1", P_TEAL, font_size=LABEL_FONT_SIZE).move_to(np.array([-1.95, G4 + 1.1, 0.0]))

        x_ref, x_gas, x_wp = 0.45, 2.55, 4.85
        t_ref, t_share, t_gas, t_wp = (ValueTracker(0.0) for _ in range(4))
        bar_ref = _bar_mob(t_ref, [(Q_REF, PRIM_VIOLET)], x_ref, G4, width=1.0, unit=U4)
        fill = always_redraw(lambda: Rectangle(
            width=0.72, height=max(0.002, t_share.get_value() * t_ref.get_value() * U4), stroke_width=0,
            fill_color=P_RED, fill_opacity=0.85).move_to(np.array(
                [x_ref, G4 + t_share.get_value() * t_ref.get_value() * U4 / 2, 0.0])))
        limit_line = always_redraw(lambda: DashedLine(
            np.array([x_ref + 0.5, G4 + t_share.get_value() * t_ref.get_value() * U4, 0.0]),
            np.array([6.6, G4 + t_share.get_value() * t_ref.get_value() * U4, 0.0]),
            color=P_RED, stroke_width=2.6))
        share_read = math_readout(
            lambda: rf"{de_num(t_share.get_value(), 2)}\cdot {Q_REF}", np.array([x_ref, G4 + Q_REF * U4 + 0.28, 0.0]),
            size=LABEL_FONT_SIZE, color=P_RED, edge="center")
        y_limit = G4 + Q_LIMIT * U4
        limit_card = _card("Grenzwert im Beispiel", math_label(rf"{Q_LIMIT}\,{UNIT}", size=BODY_FONT_SIZE, color=P_RED),
                           P_RED)
        limit_card.move_to(np.array([5.55, 1.15, 0.0]))
        limit_leader = DashedLine(limit_card.get_bottom(), np.array([5.55, y_limit, 0.0]), color=P_RED,
                                  stroke_width=1.6, stroke_opacity=0.6)
        share_note = note_line(
            f"Neubau (Wohngebäude): max. {int(REF_SHARE * 100)} % des Referenzwerts — hier schematisch", color=P_RED)
        bar_gas = _bar_mob(t_gas, [(Q_LIMIT, GAS_AMBER), (Q_PRIM_GAS - Q_LIMIT, P_RED)], x_gas, G4, width=1.0, unit=U4)
        bar_wp = _bar_mob(t_wp, [(Q_PRIM_WP, AMBIENT_GREEN)], x_wp, G4, width=1.0, unit=U4)

        def caption_under(name, colour, tracker, x):
            label = body_text(name, font_size=LABEL_FONT_SIZE, color=colour).move_to(np.array([x - 0.3, -0.95, 0.0]))
            number = _count(tracker, lambda: np.array([label.get_right()[0] + 0.1, -1.01, 0.0]), colour, edge="left")
            return label, number

        name_ref, num_ref = caption_under("Referenz", PRIM_VIOLET, t_ref, x_ref)
        name_gas, num_gas = caption_under("Gaskessel", GAS_AMBER, t_gas, x_gas)
        name_wp, num_wp = caption_under("Wärmepumpe", AMBIENT_GREEN, t_wp, x_wp)
        double_line = DashedLine(np.array([x_gas - 0.7, G4 + 2 * Q_LIMIT * U4, 0.0]),
                                 np.array([x_gas + 0.75, G4 + 2 * Q_LIMIT * U4, 0.0]), color=P_RED, stroke_width=1.8,
                                 stroke_opacity=0.55)
        double_tag = math_label(r"2\times", np.array([x_gas + 0.88, G4 + 2 * Q_LIMIT * U4 - 0.06, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_RED, edge="left")
        path_ref = smooth_path([(reference["bottom_right"][0] + 0.1, G4 - 0.2, 0.0), (x_ref - 0.5, G4 - 0.2, 0.0)])
        pipe_ref = _pipes(path_ref)

        share_gauge = meter("e", length=1.0, thickness=0.2, color=AMBIENT_GREEN, vertical=False)
        share_gauge["group"].remove(share_gauge["label"])
        share_gauge["group"].move_to(np.array([-5.55, G4 + 0.95, 0.0]))
        t_ee = ValueTracker(0.0)
        bind_meter(share_gauge, t_ee)
        ee_name = body_text("erneuerbar", font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN).move_to(
            np.array([-5.55, G4 + 1.24, 0.0]))
        ee_read = math_readout(lambda: rf"{de_num(100.0 * t_ee.get_value())}\,\%", np.array([-4.95, G4 + 0.88, 0.0]),
                               size=LABEL_FONT_SIZE, color=AMBIENT_GREEN, edge="left")

        warm = [design_in["warm"]]
        active = [lambda rt: _heat(warm, rt, 0.35)]

        def during(rt):
            anims = []
            for make in active:
                anims.extend(make(rt))
            return anims

        self.play(Create(design["group"]), Create(ground), FadeIn(design_in["group"]), run_time=1.4)
        self.play(Create(dims_d), FadeIn(tag_d), *during(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 1.0, during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "referenz"))
        warm.append(reference_in["warm"])
        self.play(
            TransformFromCopy(design["group"], reference["group"]),
            TransformFromCopy(design_in["group"], reference_in["group"]),
            TransformFromCopy(dims_d, dims_r),
            FadeIn(tag_r), *during(1.6), run_time=1.6,
        )
        self.play(Create(envelope_r), FadeIn(spec), *during(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "referenz", used=2.6, during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "grenze"))
        active.append(lambda rt: [_flow(path_ref, PRIM_VIOLET, rt, v=1.0, per_unit=3.0)])
        self.add(bar_ref, num_ref, fill, limit_line)
        self.play(FadeIn(pipe_ref), FadeIn(name_ref), FadeOut(dims_d), FadeOut(dims_r), FadeIn(share_note),
                  t_ref.animate.set_value(Q_REF), *during(1.6), run_time=1.6)
        self.add(share_read)
        self.play(t_share.animate.set_value(REF_SHARE), FadeIn(limit_card), Create(limit_leader), *during(1.6),
                  run_time=1.6)
        hold_for(self, self.NARRATION, "grenze", used=3.2, during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ergebnis"))
        verdict = note_line("im Beispiel unterschreitet nur die Wärmepumpe den Grenzwert", color=AMBIENT_GREEN)
        self.add(bar_gas, bar_wp, num_gas, num_wp)
        self.play(FadeOut(share_note), FadeIn(name_gas), run_time=0.3)
        self.play(t_gas.animate.set_value(Q_PRIM_GAS), *during(1.2), run_time=1.2)
        self.play(Create(double_line), FadeIn(double_tag), FadeIn(name_wp), *during(0.6), run_time=0.6)
        self.play(t_wp.animate.set_value(Q_PRIM_WP), FadeIn(verdict), *during(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "ergebnis", used=0.3 + 1.2 + 0.6 + 1.2, during=during)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zusatz"))
        row, box, items = math_panel([
            ("ht", r"H′_{T}", P_CYAN), ("ht_ref", r"\text{(§ 16)}", P_TEAL), (None, r"\quad", P_WHITE),
            ("ee", r"65\,\%", AMBIENT_GREEN), ("ee_text", r"\text{erneuerbar für neue Heizungen (§ 71)}", P_TEAL),
        ])
        self.play(FadeOut(verdict), Create(envelope_d), *during(1.0), run_time=1.0)
        ring_ht = highlight_param(items, "ht", color=P_CYAN)
        self.play(Create(box), FadeIn(items["ht_ref"]), TransformFromCopy(envelope_d, items["ht"]),
                  Indicate(VGroup(envelope_d, envelope_r), color=P_CYAN, scale_factor=1.02),
                  *during(1.2), run_time=1.2)
        self.play(Create(ring_ht), *during(0.5), run_time=0.5)
        self.add(ee_read)
        self.play(FadeIn(share_gauge["track"]), FadeIn(share_gauge["fill"]), FadeIn(ee_name), FadeOut(ring_ht),
                  t_ee.animate.set_value(EE_SHARE), *during(1.6), run_time=1.6)
        ee_static = _freeze(self, ee_read, rf"{de_num(100.0 * EE_SHARE)}\,\%", (-4.95, G4 + 0.88), AMBIENT_GREEN,
                            edge="left")
        self.play(TransformFromCopy(ee_static, items["ee"]), FadeIn(items["ee_text"]), *during(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "zusatz", used=1.0 + 1.2 + 0.5 + 1.6 + 1.0, during=during)

        self.play(
            FadeOut(caption), FadeOut(box), FadeOut(row), FadeOut(ring_ht),
            *[FadeOut(m) for m in (design["group"], reference["group"], design_in["group"], reference_in["group"],
                                   ground, tag_d, tag_r, spec, envelope_d, envelope_r, bar_ref, fill, limit_line,
                                   share_read, limit_card, limit_leader, bar_gas, bar_wp, name_ref, name_gas, name_wp,
                                   num_ref, num_gas, num_wp, double_line, double_tag, pipe_ref,
                                   share_gauge["track"], share_gauge["fill"], ee_name, ee_static)],
            run_time=0.6,
        )
        self.wait(0.4)

#endregion
