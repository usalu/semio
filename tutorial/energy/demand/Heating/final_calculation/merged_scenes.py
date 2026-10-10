import os
import numpy as np
import math
from manim import *

from pathlib import Path as _Path
import sys as _sys

_TUTORIAL_ROOT = next(
    p for p in _Path(__file__).resolve().parents
    if (p / "manim_fonts.py").is_file()
)
if str(_TUTORIAL_ROOT) not in _sys.path:
    _sys.path.insert(0, str(_TUTORIAL_ROOT))

from manim_fonts import apply_scene_style, BODY_FONT, LABEL_FONT_SIZE
from manim_visuals import (
    PASTEL_CYAN, PASTEL_PURPLE, PASTEL_PINK,
    PASTEL_WHITE, PASTEL_TEAL, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_RED, PASTEL_BLUE, PASTEL_GREEN, PASTEL_GREY,
    caption_bar, swap_caption, hold_for, subtitle_text, set_vo_language, begin_vo_beat,
    math_label, math_row, math_readout, math_panel, de_num,
    smooth_path, flow_animation, ripples, pulse_flashes, sun_rays, shine,
    house_section, window_glyph, open_window, room_section, radiator,
    sun_glyph, moon_glyph, person_glyph, lamp_glyph, thermometer_glyph, energy_tank,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")


#region Shared numbers
#region Envelope and ventilation (Modul 2 / Modul 3)
_DT = 20.0
_ENVELOPE = [
    ("Dach", 89.0, 0.20, 20.0),
    ("Außenwand", 209.0, 0.24, 20.0),
    ("Fenster", 30.0, 1.10, 20.0),
    ("Haustür", 2.0, 1.30, 20.0),
    ("Boden", 80.0, 0.30, 10.0),
]
_PHI_T = round(sum(a * u * dt for _n, a, u, dt in _ENVELOPE), -2)
_H_T = _PHI_T / _DT
_V_NET, _N_AIR, _C_AIR = 440.0, 0.5, 0.34
_PHI_V = round(_V_NET * _N_AIR * _C_AIR * _DT, -2)
_H_V = _PHI_V / _DT
_PHI_LOSS = _PHI_T + _PHI_V
#endregion

#region Annual balance
_SEASON_DAYS = 212
_GT = 3500.0
_Q_LOSS = (_H_T + _H_V) * _GT * 24.0 / 1000.0
_Q_LOSS_SHOWN = round(_Q_LOSS, -2)
_Q_SOL, _Q_INT = 3000.0, 2000.0
_Q_GAIN = _Q_SOL + _Q_INT
#endregion

#region Utilization day (Scene4)
_DAY_T = np.linspace(0.0, 24.0, 241)
_MASS_SHARE = 0.7


def _loss_kw(t):
    """🥶 Loss power of the example house over a clear winter day — H · (θ_i − θ_e(t))."""
    return (_H_T + _H_V) / 1000.0 * (20.0 - (2.0 + 4.0 * np.cos(2.0 * np.pi * (t - 15.0) / 24.0)))


def _internal_kw(t):
    """🧑‍🍳 Internal gains: base load plus morning and evening occupancy peaks."""
    return 0.35 + 0.25 * np.exp(-((t - 7.5) / 1.2) ** 2) + 0.45 * np.exp(-((t - 19.0) / 2.0) ** 2)


def _solar_kw(t, peak):
    """🌞 Solar gains through the south windows between sunrise and sunset."""
    return peak * np.clip(np.sin(np.pi * (t - 8.0) / 8.5), 0.0, None) ** 1.5 * ((t > 8.0) & (t < 16.5))


def _gain_kw(peak, mass):
    """⚖️ Gain curve; ``mass`` stores part of the surplus and releases it after sunset."""
    gain = _internal_kw(_DAY_T) + _solar_kw(_DAY_T, peak)
    surplus = np.maximum(0.0, gain - _loss_kw(_DAY_T))
    shape = np.exp(-((_DAY_T - 19.5) / 2.0) ** 2)
    shape /= np.trapezoid(shape, _DAY_T)
    stored = _MASS_SHARE * mass
    return gain - stored * surplus + stored * np.trapezoid(surplus, _DAY_T) * shape


def _utilization(peak, mass, upto: float = 24.0):
    """🧮 ``(A_nutz, A_Gewinn, η)`` integrated from the sampled curves up to hour ``upto``; η is over the whole day."""
    gain = _gain_kw(peak, mass)
    used = np.minimum(gain, _loss_kw(_DAY_T))
    cut = _DAY_T <= upto + 1e-9
    if cut.sum() < 2:
        return 0.0, 0.0, 0.0
    a_used = float(np.trapezoid(used[cut], _DAY_T[cut]))
    a_gain = float(np.trapezoid(gain[cut], _DAY_T[cut]))
    return a_used, a_gain, a_used / float(np.trapezoid(gain, _DAY_T))


def _solve_solar_peak(target: float = 0.90) -> float:
    """🎯 Solar peak that makes the light house reach ``target`` utilization."""
    lo, hi = 2.0, 6.0
    for _ in range(60):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if _utilization(mid, 0.0)[2] > target else (lo, mid)
    return (lo + hi) / 2


_SOLAR_PEAK = _solve_solar_peak()
_ETA_LIGHT = _utilization(_SOLAR_PEAK, 0.0)[2]
_ETA_HEAVY = _utilization(_SOLAR_PEAK, 1.0)[2]
_ETA = round(_ETA_LIGHT, 2)
_Q_USE = _ETA * _Q_GAIN
_Q_H = _Q_LOSS_SHOWN - _Q_USE
#endregion

#region System losses (DIN V 18599-5)
_CE_SHARE = 0.05
_PIPE_M, _PIPE_W_PER_M, _PIPE_H = 30.0, 10.0, 3000.0
_STORE_KWH_D, _STORE_D = 1.5, 200.0
_ETA_G = 0.95
_Q_CE = round(_CE_SHARE * _Q_H, -2)
_Q_D = _PIPE_M * _PIPE_W_PER_M * _PIPE_H / 1000.0
_Q_S = _STORE_KWH_D * _STORE_D
_Q_BEFORE_G = _Q_H + _Q_CE + _Q_D + _Q_S
_Q_G = _Q_BEFORE_G * (1.0 / _ETA_G - 1.0)
_Q_E = _Q_BEFORE_G + _Q_G
#endregion
#endregion


#region Shared helpers
def _heading(text: str, *, size: float = 34, buff: float = 0.6):
    """🏷️ Topic heading in the style every scene of this file uses."""
    return Text(text, font_size=size, color=PASTEL_WHITE, font=BODY_FONT, disable_ligatures=True).to_edge(UP, buff=buff)


def _label(text: str, *, size: float = 16, color=GREY_A):
    """🔤 Plain body-font label."""
    return Text(text, font_size=size, color=color, font=BODY_FONT, disable_ligatures=True)


def _slot(mob):
    """📍 Left baseline point of a typeset box — where a live readout replaces it."""
    return np.array([mob.get_left()[0], mob.base.get_center()[1], 0.0])


def _freeze(readout):
    """🧊 Stop a live readout so it can morph like a static label."""
    readout.clear_updaters()
    return readout


def _gt_profile():
    """🌡️ Daily θ_i − θ_e over the heating season; its sum is exactly the degree-day total G_t."""
    rng = np.random.default_rng(18599)
    days = np.arange(_SEASON_DAYS)
    noise = np.convolve(rng.normal(0.0, 2.4, days.size), np.ones(5) / 5, mode="same")
    base = 9.0 * np.sin(np.pi * (days + 0.5) / _SEASON_DAYS) + noise
    return base + (_GT - base.sum()) / days.size
#endregion


#region Physical Fundamentals glyphs
_SLAB = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)


#region Buildings
def _house(center, scale: float = 1.0):
    """🏠 Physical Fundamentals section house in the pastel palette, plus its interior ``air`` for a warm tint."""
    h = house_section(np.array(center, dtype=float), scale=scale)
    VGroup(h["group"][0], h["group"][1]).set_color(PASTEL_TEAL)
    for w in h["windows"]:
        w["sash"].set_color(PASTEL_CYAN)
    t = 0.1 * scale
    air = Polygon(
        h["bottom_left"] + RIGHT * t + UP * 0.02, h["bottom_right"] + LEFT * t + UP * 0.02,
        h["top_right"] + LEFT * t + DOWN * 0.05, h["roof_peak"] + DOWN * 0.14 * scale,
        h["top_left"] + RIGHT * t + DOWN * 0.05,
        stroke_width=0, fill_color=PASTEL_ORANGE, fill_opacity=0.0,
    )
    air.set_z_index(-1)
    h["air"] = air
    return h


def _envelope_paths(h, *, sides=("left", "right", "roof", "floor"), reach: float = 0.5, inset: float = 0.3):
    """🏠 Particle tracks from inside the house out through wall and window, roof slopes and floor slab."""
    s = h["w_width"] / 3.6
    bl, br, tl, tr, peak = h["bottom_left"], h["bottom_right"], h["top_left"], h["top_right"], h["roof_peak"]
    din, dout, hw = inset * s, reach * s, h["w_height"]
    paths = []
    if "left" in sides:
        paths += [Line(bl + UP * f * hw + RIGHT * din, bl + UP * f * hw + LEFT * dout) for f in (0.25, 0.75)]
    if "right" in sides:
        paths += [Line(br + UP * f * hw + LEFT * din, br + UP * f * hw + RIGHT * dout) for f in (0.25, 0.75)]
    if "roof" in sides:
        for a in (tl, tr):
            d = peak - a
            n = np.array([-d[1], d[0], 0.0]) / np.linalg.norm(d)
            n = n if n[1] > 0 else -n
            m = a + d * 0.5
            paths.append(Line(m - n * din, m + n * dout))
    if "floor" in sides:
        paths += [Line(bl + RIGHT * f * h["w_width"] + UP * din, bl + RIGHT * f * h["w_width"] + DOWN * dout * 0.5)
                  for f in (0.3, 0.7)]
    return paths


def _window_wall(x: float, y: float, *, height: float = 1.1, opening: float = 0.56, wall: float = 0.2):
    """🪟 Physical Fundamentals wall slab with a glazed opening — the sash opens into the room on the right."""
    lo, hi = y - opening / 2, y + opening / 2
    y0, y1 = y - height / 2, y + height / 2
    below = Rectangle(width=wall, height=lo - y0, **_SLAB).move_to(np.array([x, (y0 + lo) / 2, 0.0]))
    above = Rectangle(width=wall, height=y1 - hi, **_SLAB).move_to(np.array([x, (hi + y1) / 2, 0.0]))
    win = window_glyph(x, lo, hi, depth=0.42, color=PASTEL_CYAN)
    return {"x": x, "lo": lo, "hi": hi, "window": win, "group": VGroup(below, above, win["group"])}


def _exchange_paths(ww, *, reach: float = 0.75):
    """🌬️ Warm room air rising out through the top of an open window, cold air sinking in at the sill."""
    x, lo, hi = ww["x"], ww["lo"], ww["hi"]
    mid, q = (lo + hi) / 2, (hi - lo) / 4
    out = [smooth_path([np.array([x + reach, mid - q * 0.3, 0.0]), np.array([x + 0.25, hi - q * 0.7, 0.0]),
                        np.array([x - 0.2, hi - q * 0.6, 0.0]), np.array([x - reach, hi + q * 0.2, 0.0])])]
    inflow = [smooth_path([np.array([x - reach, lo + q * 0.2, 0.0]), np.array([x - 0.2, lo + q * 0.6, 0.0]),
                           np.array([x + 0.25, lo + q * 0.5, 0.0]), np.array([x + reach, lo - q * 0.6, 0.0])])]
    return out, inflow


def _gain_room(center, *, w: float = 1.5, h: float = 1.1):
    """🏛️ Small Physical Fundamentals room section in the pastel palette — glazed left wall, tintable ``air``."""
    room = room_section(center, w=w, h=h, wall=0.1, slab=0.1)
    room["shell"].set_stroke(PASTEL_WHITE).set_fill(PASTEL_WHITE)
    room["glass"].set_color(PASTEL_CYAN)
    room["air"].set_z_index(-1)
    return room
#endregion


#region Energy flow
def _dim(mob, k: float):
    """🌗 Scale every fill and stroke opacity of a glyph by ``k`` — for crossfading sun and moon."""
    for m in mob.family_members_with_points():
        m.set_fill(opacity=m.get_fill_opacity() * k)
        m.set_stroke(opacity=m.get_stroke_opacity() * k)
    return mob


def _flow(streams, rt: float, *, speed: float = 0.7, waves: int = 3, radius: float = 0.05):
    """💨 Particle streams that keep moving for ``rt`` seconds at ``speed`` passes per second."""
    return flow_animation(streams, waves=waves, radius=radius, cycles=max(0.6, speed * rt))


def _heat(spots, rt: float, *, r_max: float = 0.35, color=PASTEL_ORANGE, rings: int = 3, down: bool = False,
          facing=None):
    """🌡️ Long-wave heat ripples from warm ``spots`` that keep spreading for ``rt`` seconds."""
    return ripples(spots, r_max=r_max, rings=rings, color=color, down=down, facing=facing,
                   cycles=max(1.0, rt / 1.3))


def _pulses(paths, rt: float, color=PASTEL_YELLOW, *, width: float = 4.0):
    """⚡ Light pulses running along sun rays, repeated to fill ``rt`` seconds."""
    return pulse_flashes(paths, color, repeats=max(1, int(rt / 1.5)), width=width)
#endregion
#endregion


class ReviewingHeatLosses(Scene):
    NARRATION = [
        ("trans",
         "From module 2: heat flows through walls, roof, windows and floor as Phi T — the sum of U times A times Delta theta.",
         "Aus Modul 2: Wärme fließt durch Wand, Dach, Fenster und Boden als Phi-T — Summe aus U mal A mal Delta-Theta, rund 2 300 W."),
        ("vent",
         "From module 3: the air change carries heat out as Phi V — V times n times c Luft times Delta theta.",
         "Aus Modul 3: Der Luftwechsel trägt Wärme als Phi-V hinaus — V mal n mal c-Luft mal Delta-Theta, rund 1 500 W."),
        ("total",
         "Both add up to the total heat-loss power Phi Verlust — after DIN V 18599-2.",
         "Beide zusammen ergeben die Gesamtwärmeverlustleistung Phi-Verlust von rund 3 800 W — nach DIN V 18599-2."),
    ]

    def construct(self):
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "trans"))
        self.play(FadeIn(caption), run_time=0.3)

        title = _heading("Übersicht der Wärmeverluste")

        ICY_BLUE = PASTEL_CYAN
        DEEP_BLUE = PASTEL_BLUE
        WARM = PASTEL_ORANGE
        X0 = -6.3

        #region transmission
        trans_formula = math_label(r"\Phi_{\mathrm{T}} = \Sigma\, U_{i} \cdot A_{i} \cdot \Delta\theta",
                                   np.array([X0, 2.15, 0.0]), size=28, color=ICY_BLUE, edge="left")
        house = _house(np.array([-5.25, 0.8, 0.0]), scale=0.4)
        house_paths = _envelope_paths(house, reach=1.1, inset=0.6)
        phi_t = ValueTracker(0.0)
        t_read = math_readout(lambda: rf"\Phi_{{\mathrm{{T}}}} \approx {de_num(phi_t.get_value())}\,\mathrm{{W}}",
                              np.array([-3.65, 0.85, 0.0]), size=26, color=ICY_BLUE)

        def leak(rt):
            return [_flow([(house_paths, WARM, ICY_BLUE)], rt, speed=0.6, radius=0.045)]

        self.add(house["air"])
        self.play(
            Write(title),
            FadeIn(trans_formula, shift=DOWN * 0.1),
            FadeIn(house["group"]),
            house["air"].animate.set_fill(opacity=0.14),
            run_time=2.0,
        )
        self.add(t_read)
        self.play(*leak(3.0), phi_t.animate.set_value(_PHI_T), run_time=3.0)
        hold_for(self, self.NARRATION, "trans", used=0.3 + 2.0 + 3.0, during=leak)
        #endregion

        #region ventilation
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "vent"))
        vent_formula = math_label(r"\Phi_{\mathrm{V}} = V \cdot n \cdot c_{\mathrm{Luft}} \cdot \Delta\theta",
                                  np.array([X0, -0.6, 0.0]), size=28, color=DEEP_BLUE, edge="left")
        vent = _window_wall(-5.25, -1.72)
        vent_out, vent_in = _exchange_paths(vent)
        phi_v = ValueTracker(0.0)
        v_read = math_readout(lambda: rf"\Phi_{{\mathrm{{V}}}} \approx {de_num(phi_v.get_value())}\,\mathrm{{W}}",
                              np.array([-3.65, -1.72, 0.0]), size=26, color=DEEP_BLUE)

        def losses(rt):
            return [*leak(rt), _flow([(vent_out, WARM, ICY_BLUE), (vent_in, DEEP_BLUE, WARM)], rt, speed=0.55,
                                     waves=4)]

        self.play(FadeIn(vent_formula, shift=DOWN * 0.1), FadeIn(vent["group"]), *leak(1.4), run_time=1.4)
        self.play(open_window(vent["window"]), *leak(0.6), run_time=0.6)
        self.add(v_read)
        self.play(*losses(3.0), phi_v.animate.set_value(_PHI_V), run_time=3.0)
        hold_for(self, self.NARRATION, "vent", during=losses)
        #endregion

        #region total
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "total"))
        left_side = VGroup(trans_formula, house["group"], vent_formula, vent["group"], t_read, v_read)
        brace = Brace(left_side, RIGHT, color=PASTEL_WHITE, buff=0.35)
        arrow = Arrow(brace.get_right(), brace.get_right() + RIGHT * 0.9, color=PASTEL_WHITE, buff=0.1, stroke_width=3)
        loss_desc = _label("Gesamtwärmeverlustleistung (DIN V 18599-2)")
        loss_title = math_label(r"\Phi_{\mathrm{Verlust}}", size=36).next_to(loss_desc, UP, buff=0.15)
        loss_slot = math_label(rf"= {de_num(_PHI_LOSS)}\,\mathrm{{W}}", size=32).next_to(loss_desc, DOWN, buff=0.25)
        loss_box = VGroup(loss_title, loss_desc, loss_slot).next_to(arrow, RIGHT, buff=0.25)
        tot = ValueTracker(0.0)
        tot_read = math_readout(lambda: rf"= {de_num(tot.get_value())}\,\mathrm{{W}}", _slot(loss_slot), size=32)

        self.play(GrowFromCenter(brace), *losses(2.0), run_time=2.0)
        self.play(GrowArrow(arrow), FadeIn(VGroup(loss_title, loss_desc), shift=RIGHT * 0.2), *losses(2.0),
                  run_time=2.0)
        flying = [_freeze(t_read).copy(), _freeze(v_read).copy()]
        self.add(tot_read, *flying)
        self.play(*[f.animate.move_to(loss_slot).set_opacity(0.0) for f in flying],
                  tot.animate.set_value(_PHI_LOSS), *losses(2.0), run_time=2.0)
        self.remove(*flying)
        hold_for(self, self.NARRATION, "total", during=losses)
        self.play(FadeOut(caption), run_time=0.3)
        #endregion


class Scene2(Scene):
    NARRATION = [
        ("phi",
         "So the loss power is Phi Verlust equals Phi T plus Phi V — watts at one temperature difference.",
         "Die Verlustleistung ist Phi-Verlust gleich Phi-T plus Phi-V — Watt bei einer Temperaturdifferenz."),
        ("climate",
         "A design-day wattage is not a yearly kilowatt-hour. Per kelvin the house loses H T and H V watts.",
         "Eine Auslegungs-Wattzahl ist keine Jahres-Kilowattstunde. Pro Kelvin verliert das Haus H-T und H-V Watt — Phi geteilt durch Delta-Theta."),
        ("gradtag",
         "The degree-day total G t is the area under the daily indoor-outdoor difference over the heating season.",
         "Die Gradtagzahl G-t ist die Fläche unter der täglichen Differenz innen minus außen über die Heizperiode: rund 3 500 Kelvintage pro Jahr."),
        ("annual",
         "Q Verlust equals H T plus H V, times G t, times 24 hours per day — in kilowatt-hours per year, after DIN 4108-6 and DIN V 18599-2.",
         "Q-Verlust ist H-T plus H-V, mal G-t, mal 24 Stunden pro Tag — rund 16 000 kWh pro Jahr, nach DIN 4108-6 und DIN V 18599-2."),
    ]

    def construct(self):
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "phi"))
        self.play(FadeIn(caption), run_time=0.3)

        title = _heading("Vom Wärmestrom zur Jahresenergie", buff=0.55)
        self.play(Write(title), run_time=0.8)

        ICY_BLUE = PASTEL_CYAN
        DEEP_BLUE = PASTEL_BLUE
        PURPLE = PASTEL_PURPLE
        WARM = PASTEL_ORANGE

        #region power
        initial_eq, ie = math_row([
            ("loss", r"\Phi_{\mathrm{Verlust}}", PASTEL_WHITE), ("eq", "=", PASTEL_WHITE),
            ("t", r"\Phi_{\mathrm{T}}", ICY_BLUE), ("plus", "+", PASTEL_WHITE), ("v", r"\Phi_{\mathrm{V}}", DEEP_BLUE),
        ], font_size=38, buff=0.12)
        initial_eq.move_to(UP * 1.35)

        house = _house(np.array([-2.5, -0.05, 0.0]), scale=0.4)
        house_paths = _envelope_paths(house, reach=1.1, inset=0.6)
        wall_label = _label("Transmission", size=18, color=ICY_BLUE).next_to(house["floor"], DOWN, buff=0.3)
        wall_icon = VGroup(house["group"], wall_label)

        vent = _window_wall(2.5, 0.1)
        vent_out, vent_in = _exchange_paths(vent)
        window_label = _label("Lüftung", size=18, color=DEEP_BLUE).move_to(np.array([2.5, wall_label.get_y(), 0.0]))
        window_icon = VGroup(vent["group"], window_label)

        def losses(rt):
            return [_flow([(house_paths, WARM, ICY_BLUE)], rt, speed=0.6, radius=0.045),
                    _flow([(vent_out, WARM, ICY_BLUE), (vent_in, DEEP_BLUE, WARM)], rt, speed=0.55, waves=4)]

        self.add(house["air"])
        self.play(FadeIn(initial_eq), FadeIn(wall_icon), FadeIn(window_icon), house["air"].animate.set_fill(opacity=0.14),
                  run_time=1.4)
        self.play(open_window(vent["window"]), *losses(0.6), run_time=0.6)
        self.play(*losses(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "phi", used=0.3 + 0.8 + 1.4 + 0.6 + 2.0, during=losses)
        #endregion

        #region heat transfer coefficients
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "climate"))
        multiplier_group, mg = math_row([
            ("arrow", r"\to", PURPLE), ("ht", r"H_{\mathrm{T}}", ICY_BLUE), ("plus", "+", PURPLE),
            ("hv", r"H_{\mathrm{V}}", DEEP_BLUE),
        ], font_size=38, buff=0.18)
        probe = VGroup(initial_eq.copy(), multiplier_group.copy()).arrange(RIGHT, buff=0.3).move_to(UP * 1.35)
        target_initial_pos = probe[0].get_center()
        multiplier_group.move_to(probe[1].get_center())
        multiplier_group.shift(UP * (initial_eq.base.get_center()[1] - multiplier_group.base.get_center()[1]))

        climate_label = math_label(
            r"H = \Phi / \Delta\theta\;[\mathrm{W/K}] \quad \text{Jahresenergie nutzt}\; H \cdot G_{\mathrm{t}}"
            r"\text{, nicht}\; \Phi_{\mathrm{Auslegung}}",
            size=18, color=PURPLE,
        )
        climate_label.next_to(probe, UP, buff=0.45)

        h_t = ValueTracker(0.0)
        h_v = ValueTracker(0.0)
        ht_read = math_readout(
            lambda: rf"H_{{\mathrm{{T}}}} = \frac{{{de_num(_PHI_T)}\,\mathrm{{W}}}}{{20\,\mathrm{{K}}}} = "
                    rf"{de_num(h_t.get_value())}\,\mathrm{{W/K}}",
            np.array([wall_icon.get_center()[0], -1.6, 0.0]), size=20, color=ICY_BLUE, edge="center")
        hv_read = math_readout(
            lambda: rf"H_{{\mathrm{{V}}}} = \frac{{{de_num(_PHI_V)}\,\mathrm{{W}}}}{{20\,\mathrm{{K}}}} = "
                    rf"{de_num(h_v.get_value())}\,\mathrm{{W/K}}",
            np.array([window_icon.get_center()[0], -1.6, 0.0]), size=20, color=DEEP_BLUE, edge="center")

        self.play(
            initial_eq.animate.move_to(target_initial_pos),
            FadeIn(multiplier_group),
            FadeIn(climate_label),
            *losses(1.8),
            run_time=1.8,
        )
        self.add(ht_read, hv_read)
        self.play(h_t.animate.set_value(_H_T), h_v.animate.set_value(_H_V), *losses(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "climate", during=losses)
        #endregion

        #region degree days
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gradtag"))
        profile = _gt_profile()
        O = np.array([-5.7, -1.75, 0.0])
        XL, YL, YMAX = 7.4, 2.4, 25.0

        def cp(day, kelvin):
            return O + RIGHT * (day / _SEASON_DAYS * XL) + UP * (kelvin / YMAX * YL)

        x_axis = Arrow(O, O + RIGHT * (XL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.05)
        y_axis = Arrow(O, O + UP * (YL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.08)
        y_name = math_label(r"\theta_{\mathrm{i}} - \theta_{\mathrm{e}}\;[\mathrm{K}]", size=LABEL_FONT_SIZE)
        y_name.next_to(y_axis.get_end(), RIGHT, buff=0.14)
        y_ticks = VGroup(*[
            VGroup(Line(cp(0, k) + LEFT * 0.06, cp(0, k) + RIGHT * 0.06, color=PASTEL_WHITE, stroke_width=2),
                   math_label(str(k), cp(0, k) + LEFT * 0.14 + DOWN * 0.07, size=15, edge="right"))
            for k in (10, 20)
        ])
        months = ["Okt", "Nov", "Dez", "Jan", "Feb", "Mär", "Apr"]
        starts = [0, 31, 61, 92, 123, 151, 182, _SEASON_DAYS]
        m_marks = VGroup(*[Line(cp(d, 0) + DOWN * 0.06, cp(d, 0) + UP * 0.06, color=PASTEL_WHITE, stroke_width=2)
                           for d in starts[1:-1]])
        m_names = VGroup(*[_label(m, size=15, color=PASTEL_WHITE).move_to(cp((a + b) / 2, 0) + DOWN * 0.22)
                           for m, a, b in zip(months, starts[:-1], starts[1:])])
        steps = [cp(0, 0)]
        for d, v in enumerate(profile):
            steps += [cp(d, v), cp(d + 1, v)]
        steps.append(cp(_SEASON_DAYS, 0))
        curve = VMobject(color=PURPLE, stroke_width=2).set_points_as_corners(steps[1:-1])

        day = ValueTracker(0.0)

        def filled_area():
            x = max(0.001, day.get_value())
            n = int(x)
            pts = [cp(0, 0)]
            for d in range(min(n, _SEASON_DAYS)):
                pts += [cp(d, profile[d]), cp(d + 1, profile[d])]
            if n < _SEASON_DAYS:
                pts += [cp(n, profile[n]), cp(x, profile[n])]
            pts.append(cp(x, 0))
            return Polygon(*pts, stroke_width=0, fill_color=PURPLE, fill_opacity=0.45)

        def gt_now():
            x = day.get_value()
            n = int(x)
            return float(profile[:n].sum() + (profile[n] * (x - n) if n < _SEASON_DAYS else 0.0))

        area = always_redraw(filled_area)
        gt_def = math_label(r"G_{\mathrm{t}} = \Sigma\,(\theta_{\mathrm{i}} - \theta_{\mathrm{e}}) \cdot 1\,\mathrm{d}",
                            np.array([2.35, 0.35, 0.0]), size=22, color=PURPLE, edge="left")
        d_read = math_readout(lambda: rf"\mathrm{{Tag}}\; {int(day.get_value())}\; \text{{von}}\; {_SEASON_DAYS}",
                              np.array([2.35, -0.35, 0.0]), size=20, color=GREY_A)
        gt_read = math_readout(lambda: rf"G_{{\mathrm{{t}}}} \approx {de_num(round(gt_now(), -1))}\,\mathrm{{K\,d/a}}",
                               np.array([2.35, -1.05, 0.0]), size=24, color=PURPLE)

        thermo = thermometer_glyph(np.array([6.15, -1.45, 0.0]), height=2.0, color=PURPLE, level=0.0)
        thermo["column"].add_updater(
            lambda m: thermo["level"].set_value(profile[min(int(day.get_value()), _SEASON_DAYS - 1)] / YMAX))

        self.play(FadeOut(VGroup(wall_icon, window_icon, house["air"], ht_read, hv_read, climate_label)), run_time=0.6)
        self.play(GrowArrow(x_axis), GrowArrow(y_axis), FadeIn(y_name), FadeIn(y_ticks), FadeIn(m_marks),
                  FadeIn(m_names), Create(curve), FadeIn(gt_def), FadeIn(thermo["group"]), FadeIn(thermo["column"]),
                  run_time=1.6)
        self.add(area, d_read, gt_read)
        self.play(day.animate.set_value(_SEASON_DAYS), run_time=4.0, rate_func=linear)
        hold_for(self, self.NARRATION, "gradtag")
        #endregion

        #region annual loss
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "annual"))
        consolidated_eq, ce = math_row([
            ("q", r"Q_{\mathrm{Verlust}}", PASTEL_WHITE), ("eq", "=", PASTEL_WHITE), ("lp", "(", PURPLE),
            ("ht", r"H_{\mathrm{T}}", ICY_BLUE), ("plus", "+", PASTEL_WHITE), ("hv", r"H_{\mathrm{V}}", DEEP_BLUE),
            ("rp", ")", PURPLE), ("times", r"\cdot", PURPLE), ("gt", r"G_{\mathrm{t}}", PURPLE),
            ("times2", r"\cdot", PURPLE), ("hd", r"24\,\mathrm{h/d}", PURPLE),
        ], font_size=36, buff=0.12)
        consolidated_eq.move_to(UP * 1.35)
        values, vi = math_row([
            ("eq", "=", PASTEL_WHITE),
            ("h", rf"({de_num(_H_T)} + {de_num(_H_V)})\,\mathrm{{W/K}}", PASTEL_WHITE),
            ("d1", r"\cdot", PASTEL_WHITE), ("gt", rf"{de_num(_GT)}\,\mathrm{{K\,d/a}}", PURPLE),
            ("d2", r"\cdot", PASTEL_WHITE), ("hd", r"24\,\mathrm{h/d}", PASTEL_WHITE),
        ], font_size=26, buff=0.14)
        values.move_to(UP * 0.35)
        q = ValueTracker(0.0)
        q_read = math_readout(
            lambda: rf"Q_{{\mathrm{{Verlust}}}} \approx {de_num(round(q.get_value(), -2))}\,\mathrm{{kWh/a}}",
            np.array([0.0, -0.6, 0.0]), size=34, color=PASTEL_WHITE, edge="center")

        area_static = filled_area()
        self.remove(area)
        self.add(area_static)
        _freeze(gt_read)
        thermo["column"].clear_updaters()
        self.play(
            ReplacementTransform(VGroup(ie["loss"], ie["eq"]), VGroup(ce["q"], ce["eq"])),
            ReplacementTransform(VGroup(mg["ht"], mg["plus"], mg["hv"]), VGroup(ce["ht"], ce["plus"], ce["hv"])),
            ReplacementTransform(ie["t"], ce["lp"]),
            ReplacementTransform(ie["v"], ce["rp"]),
            ReplacementTransform(mg["arrow"], ce["times"]),
            FadeOut(ie["plus"]),
            ReplacementTransform(area_static, ce["gt"]),
            FadeOut(VGroup(x_axis, y_axis, y_name, y_ticks, m_marks, m_names, curve, gt_def, d_read,
                           thermo["group"], thermo["column"])),
            run_time=2.5,
        )
        self.remove(initial_eq, multiplier_group)
        self.play(FadeIn(VGroup(ce["times2"], ce["hd"]), shift=LEFT * 0.2), run_time=0.8)
        self.play(FadeIn(VGroup(vi["eq"], vi["h"], vi["d1"], vi["d2"], vi["hd"])),
                  ReplacementTransform(gt_read, vi["gt"]), run_time=1.2)
        self.add(q_read)
        self.play(q.animate.set_value(_Q_LOSS), run_time=2.2)

        unit_text = _label("Jahres-Wärmeverlust in kWh/a  —  Gradtagzahl nach DIN 4108-6 / DIN V 18599-2",
                           color=GREY_A).move_to(DOWN * 1.45)
        self.play(FadeIn(unit_text), run_time=1)

        hold_for(self, self.NARRATION, "annual")
        self.play(FadeOut(caption), run_time=0.3)
        #endregion


class ReviewingHeatGains(Scene):
    NARRATION = [
        ("intro",
         "The losses are offset by free heat gains inside the building.",
         "Den Verlusten stehen freie Wärmegewinne im Gebäude gegenüber."),
        ("solar",
         "From module 5: winter sun through the windows is Phi sol; over the heating season it adds up to Q sol.",
         "Aus Modul 5: Wintersonne durch die Fenster ist Phi-sol — über die Heizperiode summiert sich das zu Q-sol, rund 3 000 kWh pro Jahr."),
        ("internal",
         "From module 4: people, devices and lights are Phi int; over the season they become Q int.",
         "Aus Modul 4: Personen, Geräte und Licht sind Phi-int — über die Heizperiode werden daraus Q-int, rund 2 000 kWh pro Jahr."),
        ("total",
         "Together they form the gross heat gain Q Gewinn — free energy we can still use.",
         "Zusammen bilden sie den Brutto-Wärmegewinn Q-Gewinn von 5 000 kWh pro Jahr — freie Energie, die wir noch nutzen können."),
    ]

    def construct(self):
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        SOLAR_YELLOW = PASTEL_YELLOW
        INT_ORANGE = PASTEL_ORANGE
        TEXT_WHITE = "#F3F4F6"
        SUBTEXT_GREY = "#9CA3AF"

        title = Text("Übersicht der Wärmegewinne", font_size=34, color=TEXT_WHITE, font=BODY_FONT, disable_ligatures=True)
        title.to_edge(UP, buff=0.6)

        subtitle = _label("Freie Wärmegewinne (DIN V 18599)", size=18, color=SUBTEXT_GREY)
        subtitle.next_to(title, DOWN, buff=0.15)

        FX, SOL_Y, INT_Y = 1.3, 0.95, -1.15

        #region solar
        sol_room = _gain_room(np.array([0.0, SOL_Y, 0.0]))
        sun_c = np.array([-1.45, SOL_Y + 0.65, 0.0])
        sun = sun_glyph(sun_c, SOLAR_YELLOW).scale(0.3)
        rays = sun_rays(sun_c, sol_room["glass_x"], [SOL_Y + 0.25, SOL_Y], sol_room["y_f"], gap=0.42)
        rays["group"].set_color(SOLAR_YELLOW)
        ray_paths = [[s, h, p] for s, h, p in zip(rays["starts"], rays["hits"], rays["lands"])]
        sun_icon = VGroup(sun, sol_room["group"], rays["group"])

        solar_text = math_label(
            r"\Phi_{\mathrm{sol}} = G \cdot A \cdot F_{\mathrm{f}} \cdot g \cdot F_{\mathrm{sh}}",
            size=24, color=SOLAR_YELLOW)
        solar_label = _label("Solarer Wärmegewinn (DIN V 18599-2)", size=15, color=SOLAR_YELLOW)
        solar_label.next_to(solar_text, DOWN, aligned_edge=LEFT, buff=0.1)
        solar_slot = math_label(rf"Q_{{\mathrm{{sol}}}} \approx {de_num(_Q_SOL)}\,\mathrm{{kWh/a}}", size=24)
        solar_slot.next_to(solar_label, DOWN, aligned_edge=LEFT, buff=0.18)
        solar_eq_group = VGroup(solar_text, solar_label)
        VGroup(solar_eq_group, solar_slot).move_to(np.array([FX, SOL_Y, 0.0]), aligned_edge=LEFT)
        solar_group = VGroup(sun_icon, solar_eq_group, solar_slot)
        #endregion

        #region internal
        int_room = _gain_room(np.array([0.0, INT_Y, 0.0]))
        floor_y, ceil_y = int_room["y_f"], int_room["y_c"]
        person = person_glyph(ORIGIN, INT_ORANGE, scale=0.85)
        person.move_to(np.array([-0.42, floor_y + 0.01 + person.height / 2, 0.0]))
        lamp = lamp_glyph(np.array([0.02, ceil_y, 0.0]), drop=0.1)
        desk_y = floor_y + 0.3
        laptop = VGroup(
            Line(np.array([0.28, desk_y, 0.0]), np.array([0.66, desk_y, 0.0]), color=PASTEL_WHITE, stroke_width=1.5),
            Line(np.array([0.34, desk_y, 0.0]), np.array([0.34, floor_y, 0.0]), color=PASTEL_WHITE, stroke_width=1.5),
            Line(np.array([0.6, desk_y, 0.0]), np.array([0.6, floor_y, 0.0]), color=PASTEL_WHITE, stroke_width=1.5),
            Line(np.array([0.36, desk_y + 0.02, 0.0]), np.array([0.56, desk_y + 0.02, 0.0]), color=PASTEL_CYAN,
                 stroke_width=1.8),
            Line(np.array([0.56, desk_y + 0.02, 0.0]), np.array([0.52, desk_y + 0.2, 0.0]), color=PASTEL_CYAN,
                 stroke_width=1.8),
        )
        sources = VGroup(person, lamp["group"], laptop)
        person_icon = VGroup(int_room["group"], sources)
        warm_spots = [person.get_top() + DOWN * 0.12, laptop[4].get_center() + UP * 0.04]

        int_text = math_label(
            r"\Phi_{\mathrm{int}} = \Phi_{\mathrm{p}} + \Phi_{\mathrm{e}} + \Phi_{\mathrm{l}}",
            size=24, color=INT_ORANGE)
        int_label = _label("Interner Wärmegewinn (DIN V 18599-10)", size=15, color=INT_ORANGE)
        int_label.next_to(int_text, DOWN, aligned_edge=LEFT, buff=0.1)
        int_slot = math_label(rf"Q_{{\mathrm{{int}}}} \approx {de_num(_Q_INT)}\,\mathrm{{kWh/a}}", size=24)
        int_slot.next_to(int_label, DOWN, aligned_edge=LEFT, buff=0.18)
        int_eq_group = VGroup(int_text, int_label)
        VGroup(int_eq_group, int_slot).move_to(np.array([FX, INT_Y, 0.0]), aligned_edge=LEFT)
        internal_group = VGroup(person_icon, int_eq_group, int_slot)
        #endregion

        def sunshine(rt):
            return [_pulses(ray_paths, rt, SOLAR_YELLOW)]

        def body_heat(rt):
            return [_heat(warm_spots, rt, r_max=0.3, color=INT_ORANGE),
                    _heat([lamp["bulb"].get_center()], rt, r_max=0.3, color=PASTEL_RED, down=True)]

        #region total
        gains_vgroup = VGroup(solar_group, internal_group)
        brace = Brace(gains_vgroup, direction=LEFT, color=TEXT_WHITE, buff=0.3)

        q_gain_main = math_label(r"Q_{\mathrm{Gewinn}}", size=38, color=TEXT_WHITE)
        q_gain_sub = _label("Brutto-Gesamtwärmegewinn", color=SUBTEXT_GREY)
        q_gain_slot = math_label(rf"= {de_num(_Q_GAIN)}\,\mathrm{{kWh/a}}", size=28)
        q_gain_box = VGroup(q_gain_main, q_gain_sub, q_gain_slot).arrange(DOWN, buff=0.14)
        q_gain_box.next_to(brace, LEFT, buff=0.3)
        #endregion

        q_sol = ValueTracker(0.0)
        q_int = ValueTracker(0.0)
        q_tot = ValueTracker(0.0)
        sol_read = math_readout(lambda: rf"Q_{{\mathrm{{sol}}}} \approx {de_num(q_sol.get_value())}\,\mathrm{{kWh/a}}",
                                _slot(solar_slot), size=24, color=SOLAR_YELLOW)
        int_read = math_readout(lambda: rf"Q_{{\mathrm{{int}}}} \approx {de_num(q_int.get_value())}\,\mathrm{{kWh/a}}",
                                _slot(int_slot), size=24, color=INT_ORANGE)
        tot_read = math_readout(lambda: rf"= {de_num(q_tot.get_value())}\,\mathrm{{kWh/a}}",
                                _slot(q_gain_slot), size=28, color=TEXT_WHITE)

        self.play(Write(title), FadeIn(subtitle, shift=DOWN * 0.2), FadeIn(sol_room["group"]),
                  FadeIn(int_room["group"]), run_time=1.5)
        hold_for(self, self.NARRATION, "intro", used=0.3 + 1.5)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar"))
        self.play(FadeIn(sun, scale=0.7), FadeIn(solar_eq_group, shift=RIGHT * 0.3), run_time=1.0)
        self.play(shine(rays, lag=0.25), sol_room["air"].animate.set_fill(SOLAR_YELLOW, opacity=0.1), run_time=1.2)
        self.add(sol_read)
        self.play(q_sol.animate.set_value(_Q_SOL), *sunshine(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "solar", during=sunshine)

        def gains(rt):
            return [*sunshine(rt), *body_heat(rt)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "internal"))
        self.play(LaggedStart(*[FadeIn(s) for s in sources], lag_ratio=0.25), FadeIn(int_eq_group, shift=RIGHT * 0.3),
                  int_room["air"].animate.set_fill(INT_ORANGE, opacity=0.1), *sunshine(1.6), run_time=1.6)
        self.add(int_read)
        self.play(q_int.animate.set_value(_Q_INT), *gains(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "internal", during=gains)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "total"))
        self.play(Create(brace), *gains(1.5), run_time=1.5)

        self.play(FadeIn(q_gain_main), FadeIn(q_gain_sub, shift=LEFT * 0.2), *gains(1.5), run_time=1.5)
        flying = [_freeze(sol_read).copy(), _freeze(int_read).copy()]
        self.add(tot_read, *flying)
        self.play(*[f.animate.move_to(q_gain_slot).set_opacity(0.0) for f in flying],
                  q_tot.animate.set_value(_Q_GAIN), *gains(1.8), run_time=1.8)
        self.remove(*flying)

        self.play(
            q_gain_main.animate.set_color(SOLAR_YELLOW),
            brace.animate.set_color(SOLAR_YELLOW),
            *gains(0.5),
            run_time=0.5,
        )
        self.play(
            q_gain_main.animate.set_color(TEXT_WHITE),
            brace.animate.set_color(TEXT_WHITE),
            *gains(0.5),
            run_time=0.5,
        )

        hold_for(self, self.NARRATION, "total", during=gains)
        self.play(FadeOut(caption), run_time=0.3)


class Scene4(Scene):
    NARRATION = [
        ("day",
         "A clear winter day: the house loses heat around the clock, while sun and occupants deliver gains mostly at midday.",
         "Ein klarer Wintertag: Das Haus verliert rund um die Uhr Wärme, Sonne und Bewohner liefern Gewinne vor allem mittags."),
        ("overheat",
         "Gains below the loss curve replace heating. The part above overheats the room and escapes unused.",
         "Gewinne unter der Verlustkurve ersetzen Heizwärme. Der Teil darüber überhitzt den Raum und entweicht ungenutzt."),
        ("ratio",
         "Eta h is the green area divided by all gains — here about 0.90.",
         "Eta-h ist die grüne Fläche geteilt durch alle Gewinne — hier rund 0,90."),
        ("mass",
         "Heavier thermal mass stores the surplus and releases it in the evening: the red area shrinks and eta h rises.",
         "Mehr speicherfähige Masse nimmt den Überschuss auf und gibt ihn abends ab: Die rote Fläche schrumpft, Eta-h steigt auf rund 0,97."),
        ("house",
         "Our example house is calculated with eta h of about 0.90.",
         "Unser Beispielhaus rechnen wir mit Eta-h von rund 0,90."),
        ("formula",
         "So we keep only the useful share: Q nutz equals eta h times solar plus internal gains.",
         "Deshalb behalten wir nur den nutzbaren Anteil: Q-nutz ist Eta-h mal solare plus interne Gewinne, also 4 500 kWh pro Jahr."),
        ("eta",
         "Eta h is the utilization factor — how much of those free gains actually cuts the heating demand.",
         "Eta-h ist der Ausnutzungsgrad — wie viel dieser freien Gewinne den Heizwärmebedarf wirklich senkt."),
    ]

    def construct(self):
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "day"))
        self.play(FadeIn(caption), run_time=0.3)

        title = Text("Der Ausnutzungsgrad der Wärmegewinne", font_size=28, color=PASTEL_WHITE, font=BODY_FONT, disable_ligatures=True)
        title.to_edge(UP, buff=0.5)
        self.add(title)

        GREEN = PASTEL_GREEN
        RED = PASTEL_RED
        LOSS_BLUE = PASTEL_CYAN
        GAIN_YELLOW = PASTEL_YELLOW

        #region day chart
        O = np.array([-6.1, -1.75, 0.0])
        XL, YL, PMAX = 7.2, 3.3, 5.0

        def cp(hour, kw):
            return O + RIGHT * (hour / 24.0 * XL) + UP * (kw / PMAX * YL)

        x_axis = Arrow(O, O + RIGHT * (XL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.05)
        y_axis = Arrow(O, O + UP * (YL + 0.3), buff=0, stroke_width=2.4, color=PASTEL_WHITE,
                       tip_length=0.15, max_tip_length_to_length_ratio=0.08)
        x_name = math_label(r"t\;[\mathrm{h}]", size=LABEL_FONT_SIZE).next_to(x_axis.get_end(), DOWN, buff=0.14)
        x_name.align_to(x_axis.get_end(), RIGHT)
        y_name = math_label(r"P\;[\mathrm{kW}]", size=LABEL_FONT_SIZE).next_to(y_axis.get_end(), RIGHT, buff=0.14)
        x_ticks = VGroup(*[
            VGroup(Line(cp(h, 0) + DOWN * 0.06, cp(h, 0) + UP * 0.06, color=PASTEL_WHITE, stroke_width=2),
                   math_label(str(h), cp(h, 0) + DOWN * 0.3, size=15))
            for h in (6, 12, 18)
        ])
        y_ticks = VGroup(*[
            VGroup(Line(cp(0, k) + LEFT * 0.06, cp(0, k) + RIGHT * 0.06, color=PASTEL_WHITE, stroke_width=2),
                   math_label(str(k), cp(0, k) + LEFT * 0.14 + DOWN * 0.07, size=15, edge="right"))
            for k in (2, 4)
        ])
        axes = VGroup(x_axis, y_axis, x_name, y_name, x_ticks, y_ticks)

        loss = _loss_kw(_DAY_T)
        loss_curve = VMobject(color=LOSS_BLUE, stroke_width=3).set_points_smoothly(
            [cp(h, p) for h, p in zip(_DAY_T[::4], loss[::4])])
        mass = ValueTracker(0.0)
        fill_to = ValueTracker(0.0)

        def gain_now():
            return _gain_kw(_SOLAR_PEAK, mass.get_value())

        def gain_curve():
            g = gain_now()
            return VMobject(color=GAIN_YELLOW, stroke_width=3).set_points_as_corners(
                [cp(h, p) for h, p in zip(_DAY_T, g)])

        def band(upper, lower, color):
            cut = _DAY_T <= fill_to.get_value() + 1e-9
            if cut.sum() < 2:
                return VMobject()
            hs = _DAY_T[cut]
            top = [cp(h, p) for h, p in zip(hs, upper[cut])]
            bottom = [cp(h, p) for h, p in zip(hs[::-1], lower[cut][::-1])]
            return Polygon(*top, *bottom, stroke_width=0, fill_color=color, fill_opacity=0.55)

        def green_area():
            return band(np.minimum(gain_now(), loss), np.zeros_like(loss), GREEN)

        def red_area():
            g = gain_now()
            return band(np.maximum(g, loss), loss, RED)

        gains = always_redraw(gain_curve)
        green = always_redraw(green_area)
        red = always_redraw(red_area)
        #endregion

        #region sky
        hour, sky_alpha = ValueTracker(0.0), ValueTracker(0.0)

        def sky():
            h, a = hour.get_value(), sky_alpha.get_value()
            light = float(np.clip((h - 7.6) / 0.8, 0.0, 1.0) * np.clip((16.9 - h) / 0.8, 0.0, 1.0))
            arc = float(np.sin(np.pi * np.clip((h - 8.0) / 8.5, 0.0, 1.0)))
            x = cp(h, 0.0)[0]
            sun = _dim(sun_glyph(np.array([x, 2.05 + 0.35 * arc, 0.0]), GAIN_YELLOW).scale(0.24), light * a)
            moon = _dim(moon_glyph(np.array([x, 2.35, 0.0]), PASTEL_WHITE).scale(0.75), (1.0 - light) * a)
            return VGroup(moon, sun)

        def new_day(run_time: float = 0.5):
            self.play(sky_alpha.animate.set_value(0.0), run_time=run_time / 2)
            hour.set_value(0.0)
            self.play(sky_alpha.animate.set_value(1.0), run_time=run_time / 2)

        def sweep(rt):
            return [hour.animate(rate_func=linear).set_value(24.0)]

        sky_body = always_redraw(sky)
        #endregion

        #region legend and readouts
        LX = 1.75
        legend = VGroup(
            VGroup(Line(ORIGIN, RIGHT * 0.4, color=LOSS_BLUE, stroke_width=3),
                   _label("Wärmeverlust des Hauses", color=LOSS_BLUE)),
            VGroup(Line(ORIGIN, RIGHT * 0.4, color=GAIN_YELLOW, stroke_width=3),
                   _label("Gewinne: Sonne + intern", color=GAIN_YELLOW)),
            VGroup(Square(0.22, stroke_width=0, fill_color=GREEN, fill_opacity=0.7),
                   _label("genutzt, ersetzt Heizwärme", color=GREEN)),
            VGroup(Square(0.22, stroke_width=0, fill_color=RED, fill_opacity=0.7),
                   _label("Überschuss: überhitzt, geht verloren", color=RED)),
        )
        for row in legend:
            row.arrange(RIGHT, buff=0.15)
        legend.arrange(DOWN, aligned_edge=LEFT, buff=0.16).move_to(np.array([LX, 1.45, 0.0]), aligned_edge=LEFT)
        legend.shift(UP * (1.75 - legend.get_top()[1]))

        def util():
            return _utilization(_SOLAR_PEAK, mass.get_value(), fill_to.get_value())

        a_gain_read = math_readout(
            lambda: rf"A_{{\mathrm{{Gewinn}}}} = {de_num(util()[1], 1)}\,\mathrm{{kWh}}",
            np.array([LX, -0.3, 0.0]), size=22, color=GAIN_YELLOW)
        a_used_read = math_readout(
            lambda: rf"A_{{\mathrm{{nutz}}}} = {de_num(util()[0], 1)}\,\mathrm{{kWh}}",
            np.array([LX, -0.82, 0.0]), size=22, color=GREEN)
        eta_read = math_readout(
            lambda: rf"\eta_{{\mathrm{{h}}}} = \frac{{A_{{\mathrm{{nutz}}}}}}{{A_{{\mathrm{{Gewinn}}}}}} = "
                    rf"{de_num(util()[2], 2)}",
            np.array([LX, -1.58, 0.0]), size=26, color=GREEN)
        mass_tag = _label("leichte Bauweise", size=18, color=GREY_A).move_to(np.array([LX, 0.14, 0.0]), aligned_edge=LEFT)
        #endregion

        self.add(sky_body)
        self.play(FadeIn(axes), Create(loss_curve), sky_alpha.animate.set_value(1.0), run_time=1.4)
        gain_static = gain_curve()
        self.play(Create(gain_static), FadeIn(legend[:2]), run_time=1.4)
        self.remove(gain_static)
        self.add(gains)
        hold_for(self, self.NARRATION, "day", used=0.3 + 1.4 + 1.4, during=sweep)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "overheat"))
        self.add(green, red)
        self.bring_to_front(loss_curve, gains)
        self.play(FadeIn(legend[2:]), run_time=0.6)
        new_day()
        self.add(a_gain_read, a_used_read, eta_read)
        self.play(fill_to.animate.set_value(24.0), *sweep(4.5), run_time=4.5, rate_func=linear)
        hold_for(self, self.NARRATION, "overheat")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ratio"))
        self.play(Circumscribe(eta_read, color=GREEN), run_time=1.2)
        hold_for(self, self.NARRATION, "ratio")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "mass"))
        heavy_tag = _label("schwere Bauweise, viel Speichermasse", size=18, color=GREY_A).move_to(mass_tag, aligned_edge=LEFT)
        self.play(FadeIn(mass_tag), run_time=0.4)
        new_day()
        self.play(mass.animate.set_value(1.0), Transform(mass_tag, heavy_tag), *sweep(3.5), run_time=3.5)
        hold_for(self, self.NARRATION, "mass")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "house"))
        light_tag = _label("Beispielhaus, mittlere Bauweise", size=18, color=GREY_A).move_to(mass_tag, aligned_edge=LEFT)
        self.play(mass.animate.set_value(0.0), Transform(mass_tag, light_tag), run_time=2.5)
        hold_for(self, self.NARRATION, "house")

        #region formula
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        green_static = green_area()
        self.remove(green)
        self.add(green_static)
        self.play(FadeOut(VGroup(axes, loss_curve, gains, red, legend, a_gain_read, a_used_read, eta_read, mass_tag)),
                  sky_alpha.animate.set_value(0.0), run_time=0.8)
        self.remove(sky_body)

        initial_eq, ie = math_row([
            ("q", r"Q_{\mathrm{Gewinn}}", PASTEL_WHITE), ("eq", "=", PASTEL_WHITE), ("sol", r"Q_{\mathrm{sol}}", PASTEL_YELLOW),
            ("plus", "+", PASTEL_WHITE), ("int", r"Q_{\mathrm{int}}", PASTEL_ORANGE),
        ], font_size=38, buff=0.15)
        initial_eq.move_to(UP * 1.1)

        self.play(FadeIn(initial_eq), run_time=1.2)
        self.wait(0.6)

        target_group, tg = math_row([
            ("q", r"Q_{\mathrm{nutz}}", PASTEL_WHITE), ("eq", "=", PASTEL_WHITE), ("eta", r"\eta_{\mathrm{h}}", GREEN),
            ("dot", r"\cdot", GREEN), ("lp", "(", GREEN), ("sol", r"Q_{\mathrm{sol}}", PASTEL_YELLOW),
            ("plus", "+", PASTEL_WHITE), ("int", r"Q_{\mathrm{int}}", PASTEL_ORANGE), ("rp", ")", GREEN),
        ], font_size=38, buff=0.12)
        target_group.move_to(UP * 1.1)

        self.play(
            Transform(ie["q"], tg["q"]),
            ie["eq"].animate.move_to(tg["eq"]),
            ie["sol"].animate.move_to(tg["sol"]),
            ie["plus"].animate.move_to(tg["plus"]),
            ie["int"].animate.move_to(tg["int"]),
            run_time=1.2,
        )

        self.play(
            ReplacementTransform(green_static, tg["eta"]),
            FadeIn(tg["dot"]),
            FadeIn(tg["lp"], shift=RIGHT * 0.1),
            FadeIn(tg["rp"], shift=LEFT * 0.1),
            run_time=1.6,
        )
        q_use = ValueTracker(0.0)
        use_read = math_readout(
            lambda: rf"Q_{{\mathrm{{nutz}}}} = {de_num(_ETA, 2)} \cdot {de_num(_Q_GAIN)}\,\mathrm{{kWh/a}} = "
                    rf"{de_num(q_use.get_value())}\,\mathrm{{kWh/a}}",
            np.array([0.0, 0.05, 0.0]), size=28, color=PASTEL_WHITE, edge="center")
        self.add(use_read)
        self.play(q_use.animate.set_value(_Q_USE), run_time=2.0)
        hold_for(self, self.NARRATION, "formula")
        #endregion

        #region explanation
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "eta"))
        eta_box = SurroundingRectangle(tg["eta"], color=GREEN, buff=0.05, corner_radius=0.06)

        eta_title = VGroup(
            math_label(r"\eta_{\mathrm{h}}\text{:}", size=22, color=GREEN),
            _label("Ausnutzungsgrad der Wärmegewinne (DIN V 18599-2)", size=22, color=GREEN),
        ).arrange(RIGHT, buff=0.12, aligned_edge=DOWN)
        eta_title.move_to(DOWN * 0.8)

        eta_line1 = _label("Gibt den Anteil der Wärmegewinne an (0 bis 100 %),", size=18)
        eta_line2 = _label("der tatsächlich zur Deckung des Heizwärmebedarfs beiträgt.", size=18)
        eta_desc = VGroup(eta_line1, eta_line2).arrange(DOWN, buff=0.12).next_to(eta_title, DOWN, buff=0.3)

        self.play(Create(eta_box), FadeIn(eta_title), run_time=1.0)
        self.play(FadeIn(eta_desc, shift=UP * 0.15), run_time=1.0)

        hold_for(self, self.NARRATION, "eta")
        self.play(FadeOut(caption), run_time=0.3)
        #endregion


class UltimateEnergyBalance(Scene):
    NARRATION = [
        ("balance",
         "Balance: losses leave the house, useful gains stay — the difference is the heating demand.",
         "Bilanz: Verluste gehen hinaus, nutzbare Gewinne bleiben — die Differenz ist der Heizwärmebedarf."),
        ("master",
         "Heating demand Q h equals Q Verlust minus eta h times Q Gewinn: 16,000 minus 4,500 is about 11,500 kilowatt-hours per year.",
         "Heizwärmebedarf Q-h ist Q-Verlust minus Eta-h mal Q-Gewinn: 16 000 minus 4 500 ergibt rund 11 500 kWh pro Jahr."),
        ("expand",
         "Expanded: transmission energy plus ventilation energy, minus eta h times solar plus internal energy.",
         "Ausgeschrieben: Transmissionsenergie plus Lüftungsenergie, minus Eta-h mal Solar plus intern."),
    ]

    def construct(self):
        apply_scene_style(self)

        caption = caption_bar(subtitle_text(self.NARRATION, "balance"))
        self.play(FadeIn(caption), run_time=0.3)

        title = Text(
            "Die Hauptgleichung des Heizwärmebedarfs (DIN V 18599)",
            font_size=26,
            color=PASTEL_WHITE,
            font=BODY_FONT, disable_ligatures=True)
        title.to_edge(UP, buff=0.5)

        LOSS_C, ETA_C, GAIN_C, QH_C = PASTEL_BLUE, PASTEL_GREEN, PASTEL_YELLOW, PASTEL_RED

        #region house balance
        house = _house(np.array([-0.3, -0.5, 0.0]), scale=0.85)
        loss_paths = _envelope_paths(house, sides=("right", "roof", "floor"), reach=1.1, inset=0.45)
        sun_c = np.array([-4.4, 1.55, 0.0])
        sun = sun_glyph(sun_c, GAIN_C).scale(0.42)
        floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
        ray_sets = [sun_rays(sun_c, w["x"], [w["center"][1] + dy for dy in (0.1, -0.1)], fy, gap=0.55)
                    for w, fy in zip(house["windows"], floors)]
        for r in ray_sets:
            r["group"].set_color(GAIN_C)
        ray_paths = [[s, h, p] for r in ray_sets for s, h, p in zip(r["starts"], r["hits"], r["lands"])]
        floor_y = house["bottom_left"][1]
        person = person_glyph(ORIGIN, PASTEL_ORANGE, scale=0.9)
        person.move_to(np.array([0.0, floor_y + 0.02 + person.height / 2, 0.0]))
        lamp = lamp_glyph(np.array([0.4, 0.95, 0.0]), drop=0.45)
        heater = radiator(np.array([0.82, floor_y + 0.2, 0.0]), QH_C).scale(0.45)
        heater.align_to(np.array([0.0, floor_y + 0.05, 0.0]), DOWN)
        scene_house = VGroup(house["group"], house["air"], sun, *[r["group"] for r in ray_sets], person,
                             lamp["group"], heater)

        def losses(rt):
            return [_flow([(loss_paths, PASTEL_ORANGE, LOSS_C)], rt, speed=0.55, radius=0.055)]

        def gains(rt):
            return [_pulses(ray_paths, rt, GAIN_C),
                    _heat([person.get_top() + DOWN * 0.12], rt, r_max=0.4),
                    _heat([lamp["bulb"].get_center()], rt, r_max=0.4, color=QH_C, down=True)]

        def heating(rt):
            return [_heat([heater.get_top() + UP * 0.03], rt, r_max=0.45, color=QH_C)]

        def balance(rt):
            return [*losses(rt), *gains(rt), *heating(rt)]

        self.add(house["air"])
        self.play(Write(title), FadeIn(house["group"]), house["air"].animate.set_fill(opacity=0.14), run_time=1.2)
        #endregion

        #region tags
        q_loss_tag = math_label(r"Q_{\mathrm{Verlust}}", size=26, color=LOSS_C).move_to(np.array([3.4, -0.3, 0.0]))
        loss_v = ValueTracker(0.0)
        loss_read = math_readout(lambda: rf"{de_num(round(loss_v.get_value(), -1))}\,\mathrm{{kWh/a}}",
                                 lambda: q_loss_tag.get_top() + UP * 0.2, size=22, color=LOSS_C, edge="center")

        self.play(FadeIn(q_loss_tag, shift=RIGHT * 0.3), *losses(0.6), run_time=0.6)
        self.add(loss_read)
        self.play(loss_v.animate.set_value(_Q_LOSS_SHOWN), *losses(1.4), run_time=1.4)

        q_gain_tag, gt = math_row([
            ("eta", r"\eta_{\mathrm{h}}", ETA_C), ("dot", r"\cdot", PASTEL_WHITE), ("g", r"Q_{\mathrm{Gewinn}}", GAIN_C),
        ], font_size=26, buff=0.08)
        q_gain_tag.move_to(np.array([-4.6, -0.6, 0.0]))
        gain_v = ValueTracker(0.0)
        gain_read = math_readout(lambda: rf"{de_num(round(gain_v.get_value(), -1))}\,\mathrm{{kWh/a}}",
                                 lambda: q_gain_tag.get_top() + UP * 0.2, size=22, color=GAIN_C, edge="center")

        self.play(FadeIn(sun, scale=0.7), FadeIn(person), FadeIn(lamp["group"]), *losses(0.6), run_time=0.6)
        self.play(*[shine(r, lag=0.25) for r in ray_sets], *losses(0.8), run_time=0.8)
        self.play(FadeIn(q_gain_tag, shift=RIGHT * 0.3), *losses(0.6), *gains(0.6), run_time=0.6)
        self.add(gain_read)
        self.play(gain_v.animate.set_value(_Q_USE), *losses(1.4), *gains(1.4), run_time=1.4)
        self.play(FadeIn(heater), *balance(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "balance", used=0.3 + 1.2 + 0.6 + 1.4 + 0.6 + 0.8 + 0.6 + 1.4 + 1.0,
                 during=balance)
        #endregion

        #region master equation
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "master"))
        master_eq, me = math_row([
            ("qh", r"Q_{\mathrm{h}}", QH_C), ("eq", "=", PASTEL_WHITE), ("loss", r"Q_{\mathrm{Verlust}}", LOSS_C),
            ("minus", "-", PASTEL_WHITE), ("eta", r"\eta_{\mathrm{h}}", ETA_C), ("dot", r"\cdot", PASTEL_WHITE),
            ("gain", r"Q_{\mathrm{Gewinn}}", GAIN_C),
        ], font_size=36, buff=0.12)
        master_eq.move_to(UP * 1.2)

        numbers, nb = math_row([
            ("qh", r"Q_{\mathrm{h}}", QH_C), ("eq", "=", PASTEL_WHITE), ("loss", de_num(_Q_LOSS_SHOWN), LOSS_C),
            ("minus", "-", PASTEL_WHITE), ("gain", de_num(_Q_USE), GAIN_C), ("approx", r"\approx", PASTEL_WHITE),
            ("slot", rf"{de_num(_Q_H)}\,\mathrm{{kWh/a}}", QH_C),
        ], font_size=32, buff=0.14)
        numbers.move_to(UP * 0.1)
        q_h = ValueTracker(0.0)
        qh_read = math_readout(lambda: rf"{de_num(round(q_h.get_value(), -1))}\,\mathrm{{kWh/a}}",
                               _slot(nb["slot"]), size=32, color=QH_C)

        self.play(
            FadeOut(scene_house),
            ReplacementTransform(q_loss_tag, me["loss"]),
            ReplacementTransform(q_gain_tag, VGroup(me["eta"], me["dot"], me["gain"])),
            FadeIn(VGroup(me["qh"], me["eq"], me["minus"])),
            ReplacementTransform(_freeze(loss_read), nb["loss"]),
            ReplacementTransform(_freeze(gain_read), nb["gain"]),
            FadeIn(VGroup(nb["qh"], nb["eq"], nb["minus"], nb["approx"])),
            run_time=1.5,
        )
        self.add(qh_read)
        self.play(q_h.animate.set_value(_Q_H), run_time=2.0)
        hold_for(self, self.NARRATION, "master")
        #endregion

        #region expanded equation
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "expand"))
        expanded_eq, ex = math_row([
            ("qh", r"Q_{\mathrm{h}}", QH_C), ("eq", "=", PASTEL_WHITE),
            ("loss", r"(Q_{\mathrm{T}} + Q_{\mathrm{V}})", LOSS_C), ("minus", "-", PASTEL_WHITE),
            ("eta", r"\eta_{\mathrm{h}}", ETA_C), ("dot", r"\cdot", PASTEL_WHITE),
            ("gain", r"(Q_{\mathrm{sol}} + Q_{\mathrm{int}})", GAIN_C),
        ], font_size=30, buff=0.12)
        expanded_eq.move_to(DOWN * 1.1)

        self.play(FadeIn(VGroup(ex["qh"], ex["eq"]), shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(FadeIn(ex["loss"], shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(FadeIn(VGroup(ex["minus"], ex["eta"], ex["dot"]), shift=UP * 0.3), run_time=0.8)
        self.wait(0.3)
        self.play(FadeIn(ex["gain"], shift=UP * 0.3), run_time=0.8)

        hold_for(self, self.NARRATION, "expand")

        shown = VGroup(*[nb[k] for k in ("qh", "eq", "loss", "minus", "gain", "approx")])
        self.play(FadeOut(VGroup(title, master_eq, shown, qh_read, expanded_eq, caption)), run_time=1.5)
        self.wait(0.5)
        #endregion


#region System losses scene
P_DEEP_GREY = "#1F2937"
_HOUSE_DY = 0.22


def _plant_house():
    """🏠 Physical Fundamentals section through the example house: heated storey, roof, unheated cellar with boiler,
    energy-tank store and pipes."""
    PIPE = PASTEL_RED
    P = lambda x, y: np.array([x, y + _HOUSE_DY, 0.0])
    slab = lambda x0, x1, y0, y1: Rectangle(width=x1 - x0, height=y1 - y0, **_SLAB).move_to(P((x0 + x1) / 2, (y0 + y1) / 2))
    WIN = (0.65, 1.25)
    RAD_X, RAD_Y, PIPE_Y, TANK_X = (-6.12, -1.88), 0.04, -0.48, -4.9
    parts = {}
    parts["room"] = Rectangle(width=4.95, height=1.6, stroke_width=0, fill_color=PASTEL_ORANGE, fill_opacity=0.10
                              ).move_to(P(-4.0, 0.65))
    parts["cellar"] = Rectangle(width=4.95, height=1.3, stroke_width=0, fill_color=PASTEL_BLUE, fill_opacity=0.10
                                ).move_to(P(-4.0, -0.95))
    parts["walls"] = VGroup(
        *[slab(x - 0.15, x + 0.15, -0.15, WIN[0]) for x in (-6.65, -1.35)],
        *[slab(x - 0.15, x + 0.15, WIN[1], 1.45) for x in (-6.65, -1.35)],
        *[slab(x - 0.15, x + 0.15, -1.6, -0.3) for x in (-6.65, -1.35)],
        slab(-6.8, -1.2, -0.3, -0.15), slab(-6.8, -1.2, -1.72, -1.6),
    )
    parts["back_wall"] = parts["walls"][0]
    parts["roof"] = VGroup(
        VMobject(color=PASTEL_WHITE, stroke_width=1.7).set_points_as_corners([P(-7.0, 1.45), P(-4.0, 2.4), P(-1.0, 1.45)]),
        VMobject(color=PASTEL_WHITE, stroke_width=1.3).set_points_as_corners([P(-6.5, 1.42), P(-4.0, 2.26), P(-1.5, 1.42)]),
    )
    windows = [window_glyph(x, WIN[0] + _HOUSE_DY, WIN[1] + _HOUSE_DY, depth=0.3, color=PASTEL_CYAN)
               for x in (-6.65, -1.35)]
    parts["windows"] = VGroup(*[w["group"] for w in windows])
    parts["radiators"] = VGroup(*[radiator(P(x, RAD_Y), PIPE).scale(0.42) for x in RAD_X])
    parts["thermostat"] = Circle(radius=0.065, color=PASTEL_WHITE, stroke_width=2, fill_color=P_DEEP_GREY, fill_opacity=1
                                 ).move_to(P(-5.83, RAD_Y + 0.08))
    tank = energy_tank(P(TANK_X, -1.13), height=0.72, width=0.5, color=PASTEL_YELLOW, level=0.9)
    parts["tank"] = tank
    parts["store"] = VGroup(tank["group"], tank["fill"])
    parts["boiler"] = Rectangle(width=1.6, height=0.7, color=PASTEL_ORANGE, stroke_width=2, fill_color=PASTEL_ORANGE,
                                fill_opacity=0.12).move_to(P(-2.85, -1.2))
    parts["chimney"] = VGroup(
        Line(P(-2.46, -0.85), P(-2.46, 2.3), color=PASTEL_WHITE, stroke_width=1.5),
        Line(P(-2.24, -0.85), P(-2.24, 2.3), color=PASTEL_WHITE, stroke_width=1.5),
    )
    rad_in = [P(x, RAD_Y - 0.1) for x in RAD_X]
    tank_top = tank["cap"].get_top()
    run = [rad_in[0], P(RAD_X[0], PIPE_Y), P(RAD_X[1], PIPE_Y), rad_in[1]]
    parts["pipes"] = VGroup(
        VMobject(color=PIPE, stroke_width=4).set_points_as_corners(run),
        Line(tank_top, P(TANK_X, PIPE_Y), color=PIPE, stroke_width=4),
        Line(P(-3.65, -1.2), P(TANK_X + 0.25, -1.2), color=PIPE, stroke_width=4),
    )
    parts["flow_paths"] = [
        VMobject().set_points_as_corners([P(-3.65, -1.2), P(TANK_X + 0.25, -1.2)]),
        VMobject().set_points_as_corners([tank_top, P(TANK_X, PIPE_Y), P(RAD_X[0], PIPE_Y), rad_in[0]]),
        VMobject().set_points_as_corners([P(TANK_X, PIPE_Y), P(RAD_X[1], PIPE_Y), rad_in[1]]),
    ]
    parts["pipe_spots"] = [P(x, PIPE_Y - 0.04) for x in (-5.55, -4.2, -3.75, -2.05)]
    parts["sleeves"] = VGroup(
        VMobject(color=PASTEL_WHITE, stroke_width=11, stroke_opacity=0.55).set_points_as_corners(
            [P(RAD_X[0], -0.32), P(RAD_X[0], PIPE_Y), P(RAD_X[1], PIPE_Y), P(RAD_X[1], -0.32)]),
        Line(tank_top, P(TANK_X, PIPE_Y), color=PASTEL_WHITE, stroke_width=11, stroke_opacity=0.55),
    )
    parts["labels"] = VGroup(
        _label("Speicher", size=15, color=PASTEL_YELLOW).move_to(P(-4.2, -0.92)),
        _label("Wärmeerzeuger", size=15, color=PASTEL_ORANGE).move_to(parts["boiler"]),
        VGroup(_label("unbeheizter", size=15, color=GREY_A), _label("Keller", size=15, color=GREY_A)
               ).arrange(DOWN, buff=0.06).move_to(P(-5.9, -1.15)),
        _label("beheizt, 20 °C", size=15, color=PASTEL_ORANGE).move_to(P(-4.0, 0.95)),
    )
    parts["thermo_label"] = _label("Thermostat", size=15, color=PASTEL_WHITE).move_to(P(-5.05, RAD_Y + 0.08))
    parts["pipe_label"] = _label("30 m Rohr", size=15, color=PIPE).move_to(P(-3.05, -0.68))
    return parts



class AnlagenVerluste(Scene):
    NARRATION = [
        ("need",
         "The heating demand Q h of about 11,500 kilowatt-hours per year is what the rooms need.",
         "Der Heizwärmebedarf Q-h von rund 11 500 kWh pro Jahr ist das, was die Räume brauchen."),
        ("chain",
         "The system must deliver more: on the way from the boiler to the room, generator, store, pipes and radiators lose heat.",
         "Die Anlage muss mehr liefern: Auf dem Weg vom Kessel in den Raum verlieren Erzeuger, Speicher, Rohre und Heizkörper Wärme."),
        ("ce",
         "Emission: the radiator also heats the outer wall behind it and the thermostat regulates sluggishly — about five percent.",
         "Übergabe: Der Heizkörper erwärmt auch die Außenwand dahinter, das Thermostat regelt träge — rund 5 Prozent, also etwa 600 kWh."),
        ("d",
         "Distribution: 30 metres of pipe in the cold cellar lose 10 watts per metre over 3,000 hours.",
         "Verteilung: 30 Meter Rohr im kalten Keller verlieren 10 Watt pro Meter über 3 000 Stunden — 900 kWh."),
        ("s",
         "Storage: the buffer store loses 1.5 kilowatt-hours per day on standby, over 200 heating days.",
         "Speicherung: Der Pufferspeicher verliert im Bereitschaftsbetrieb 1,5 kWh pro Tag, über 200 Heiztage 300 kWh."),
        ("g",
         "Generation: with an efficiency of 0.95, part of the fuel energy leaves through the flue.",
         "Erzeugung: Bei einem Wirkungsgrad von 0,95 geht ein Teil als Abgas durch den Schornstein — rund 700 kWh."),
        ("sum",
         "Final energy Q E is Q h plus all system losses: about 14,000 kilowatt-hours per year, after DIN V 18599-5.",
         "Die Endenergie Q-E ist Q-h plus alle Anlagenverluste: rund 14 000 kWh pro Jahr, nach DIN V 18599-5."),
        ("tip",
         "So keep pipes and stores inside the heated envelope and insulate the pipes.",
         "Darum gehören Rohre und Speicher in die beheizte Hülle, und die Leitungen werden gedämmt."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        caption = caption_bar(subtitle_text(N, "need"))
        self.play(FadeIn(caption), run_time=0.3)

        title = _heading("Anlagenverluste: vom Heizwärmebedarf zur Endenergie", size=28, buff=0.5)
        self.play(Write(title), run_time=0.8)

        QH_C, CE_C, D_C, S_C, G_C, E_C = PASTEL_RED, PASTEL_PINK, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_PURPLE, PASTEL_WHITE
        h = _plant_house()
        floor_y = h["walls"][6].get_top()[1]
        person = person_glyph(ORIGIN, PASTEL_ORANGE, scale=0.85)
        person.move_to(np.array([-3.35, floor_y + 0.01 + person.height / 2, 0.0]))
        rad_l = h["radiators"][0]
        glow = h["back_wall"].copy().set_stroke(width=0).set_fill(QH_C, opacity=0.0)
        wall_paths = [Line(np.array([rad_l.get_left()[0] - 0.02, y, 0.0]), np.array([-7.0, y, 0.0]))
                      for y in np.linspace(rad_l.get_bottom()[1] + 0.06, rad_l.get_top()[1] - 0.04, 3)]
        sc = h["tank"]["frame"].get_center()
        flue = [smooth_path([np.array([-2.35, -0.85 + _HOUSE_DY, 0.0]), np.array([-2.35, 1.0 + _HOUSE_DY, 0.0]),
                             np.array([-2.35, 2.3 + _HOUSE_DY, 0.0]), np.array([-2.15, 2.6 + _HOUSE_DY, 0.0])])]
        on = set()

        def running(rt):
            anims = [_heat([person.get_top() + DOWN * 0.12], rt, r_max=0.3)]
            if "water" in on:
                anims.append(_flow([(h["flow_paths"], PASTEL_RED)], rt, speed=0.5, waves=6, radius=0.05))
            if "ce" in on:
                anims += [_flow([(wall_paths, PASTEL_RED, PASTEL_BLUE)], rt, speed=0.8, radius=0.045),
                          _heat([rad_l.get_left() + RIGHT * 0.02], rt, r_max=0.28, color=QH_C, facing=PI)]
            if "d" in on:
                anims.append(_heat(h["pipe_spots"], rt, r_max=0.24, color=D_C, rings=2, down=True))
            if "s" in on:
                anims += [_heat([sc + LEFT * 0.27], rt, r_max=0.24, color=S_C, rings=2, facing=PI),
                          _heat([sc + RIGHT * 0.27], rt, r_max=0.24, color=S_C, rings=2, facing=0.0)]
            if "g" in on:
                anims.append(_flow([(flue, PASTEL_ORANGE, PASTEL_GREY)], rt, speed=0.6, waves=6, radius=0.075))
            return anims

        #region stacked bar
        BASE = np.array([0.35, -1.5, 0.0])
        BAR_W = 0.7
        UNIT = 3.5 / _Q_E
        vals = [ValueTracker(0.0) for _ in range(5)]
        colors = [QH_C, CE_C, D_C, S_C, G_C]

        def bar():
            group = VGroup()
            cursor = 0.0
            for v, c in zip(vals, colors):
                height = v.get_value() * UNIT
                if height > 1e-4:
                    group.add(Rectangle(width=BAR_W, height=height, stroke_width=1, stroke_color=c, fill_color=c,
                                        fill_opacity=0.8).move_to(BASE + UP * (cursor + height / 2)))
                cursor += height
            return group

        finals = [_Q_H, _Q_CE, _Q_D, _Q_S, _Q_G]
        mids = []
        cursor = 0.0
        for f in finals:
            mids.append(BASE + UP * (cursor + f * UNIT / 2) + RIGHT * BAR_W / 2)
            cursor += f * UNIT
        bar_live = always_redraw(bar)
        bar_floor = Line(BASE + LEFT * 0.55, BASE + RIGHT * 0.55, color=PASTEL_WHITE, stroke_width=2)
        #endregion

        #region readout column
        CX = 1.55
        rows_y = [-0.95, -0.3, 0.32, 0.94, 1.56, 2.18]
        names = ["Heizwärmebedarf", "Übergabe: Heizkörper, Thermostat", "Verteilung: Rohre im Keller",
                 "Speicherung: Bereitschaft", "Erzeugung: Abgas, Wirkungsgrad 0,95", "Endenergie"]
        row_colors = colors + [E_C]
        name_mobs = [_label(n, size=15, color=c).move_to(np.array([CX, y + 0.2, 0.0]), aligned_edge=LEFT)
                     for n, y, c in zip(names, rows_y, row_colors)]
        e_val = ValueTracker(0.0)
        formulas = [
            lambda: rf"Q_{{\mathrm{{h}}}} \approx {de_num(vals[0].get_value())}\,\mathrm{{kWh/a}}",
            lambda: rf"Q_{{\mathrm{{h,ce}}}} = 5\,\% \cdot {de_num(_Q_H)} \approx {de_num(vals[1].get_value())}\,\mathrm{{kWh/a}}",
            lambda: rf"Q_{{\mathrm{{h,d}}}} = {de_num(_PIPE_M)}\,\mathrm{{m}} \cdot {de_num(_PIPE_W_PER_M)}\,\mathrm{{W/m}} \cdot {de_num(_PIPE_H)}\,\mathrm{{h}} = "
                    rf"{de_num(vals[2].get_value())}\,\mathrm{{kWh/a}}",
            lambda: rf"Q_{{\mathrm{{h,s}}}} = 1{{,}}5\,\mathrm{{kWh/d}} \cdot 200\,\mathrm{{d}} = "
                    rf"{de_num(vals[3].get_value())}\,\mathrm{{kWh/a}}",
            lambda: rf"Q_{{\mathrm{{h,g}}}} = {de_num(_Q_BEFORE_G)} \cdot (1/0{{,}}95 - 1) \approx "
                    rf"{de_num(vals[4].get_value())}\,\mathrm{{kWh/a}}",
            lambda: rf"Q_{{\mathrm{{E}}}} \approx {de_num(round(e_val.get_value(), -2))}\,\mathrm{{kWh/a}}",
        ]
        readouts = [math_readout(fn, np.array([CX, y - 0.16, 0.0]), size=19, color=c)
                    for fn, y, c in zip(formulas, rows_y, row_colors)]
        leaders = [Line(np.array([CX - 0.1, y + 0.05, 0.0]), mid, color=c, stroke_width=1.4, stroke_opacity=0.6)
                   for y, mid, c in zip(rows_y[:5], mids, colors)]
        #endregion

        #region need
        self.play(FadeIn(h["room"]), FadeIn(h["cellar"]), FadeIn(h["walls"]), FadeIn(h["roof"]),
                  FadeIn(h["windows"]), FadeIn(h["labels"][3]), FadeIn(person), Create(bar_floor), run_time=1.2)
        self.add(bar_live, readouts[0])
        self.play(FadeIn(name_mobs[0]), Create(leaders[0]), vals[0].animate.set_value(_Q_H),
                  h["room"].animate.set_fill(opacity=0.22), *running(2.0), run_time=2.0)
        hold_for(self, N, "need", used=0.3 + 0.8 + 1.2 + 2.0, during=running)
        #endregion

        #region chain
        caption = swap_caption(self, caption, subtitle_text(N, "chain"))
        self.play(FadeIn(h["store"]), FadeIn(h["boiler"]), FadeIn(h["chimney"]), Create(h["pipes"]),
                  FadeIn(h["radiators"]), FadeIn(h["thermostat"]), FadeIn(h["labels"][:3]), *running(1.5),
                  run_time=1.5)
        on.add("water")
        self.play(*running(3.0), run_time=3.0)
        hold_for(self, N, "chain", during=running)
        #endregion

        #region emission
        caption = swap_caption(self, caption, subtitle_text(N, "ce"))
        self.add(glow)
        self.add(readouts[1])
        self.play(FadeIn(name_mobs[1]), FadeIn(h["thermo_label"]), Create(leaders[1]), *running(0.5), run_time=0.5)
        on.add("ce")
        self.play(vals[1].animate.set_value(_Q_CE), glow.animate.set_fill(opacity=0.55),
                  Wiggle(h["thermostat"], scale_value=1.4, n_wiggles=4), *running(3.0), run_time=3.0)
        hold_for(self, N, "ce", during=running)
        #endregion

        #region distribution
        caption = swap_caption(self, caption, subtitle_text(N, "d"))
        self.add(readouts[2])
        self.play(FadeIn(name_mobs[2]), FadeIn(h["pipe_label"]), Create(leaders[2]), *running(0.5), run_time=0.5)
        on.add("d")
        self.play(vals[2].animate.set_value(_Q_D), *running(3.0), run_time=3.0)
        hold_for(self, N, "d", during=running)
        #endregion

        #region storage
        caption = swap_caption(self, caption, subtitle_text(N, "s"))
        self.add(readouts[3])
        self.play(FadeIn(name_mobs[3]), Create(leaders[3]), *running(0.5), run_time=0.5)
        on.add("s")
        self.play(vals[3].animate.set_value(_Q_S), h["tank"]["level"].animate.set_value(0.75), *running(3.0),
                  run_time=3.0)
        hold_for(self, N, "s", during=running)
        #endregion

        #region generation
        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        self.add(readouts[4])
        self.play(FadeIn(name_mobs[4]), Create(leaders[4]), *running(0.5), run_time=0.5)
        on.add("g")
        self.play(vals[4].animate.set_value(_Q_G), *running(3.0),
                  run_time=3.0)
        hold_for(self, N, "g", during=running)
        #endregion

        #region sum
        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        row, box, items = math_panel([
            ("e", r"Q_{\mathrm{E}}", E_C), (None, "=", PASTEL_WHITE), ("h", r"Q_{\mathrm{h}}", QH_C), (None, "+", PASTEL_WHITE),
            ("ce", r"Q_{\mathrm{h,ce}}", CE_C), (None, "+", PASTEL_WHITE), ("d", r"Q_{\mathrm{h,d}}", D_C),
            (None, "+", PASTEL_WHITE), ("s", r"Q_{\mathrm{h,s}}", S_C), (None, "+", PASTEL_WHITE),
            ("g", r"Q_{\mathrm{h,g}}", G_C), (None, r"\approx", PASTEL_WHITE),
            ("v", rf"{de_num(round(_Q_E, -2))}\,\mathrm{{kWh/a}}", E_C),
        ], color=PASTEL_TEAL)
        sources = {
            key: math_label(src, _slot(r), size=21, color=c, edge="left")
            for key, src, r, c in zip(
                ("h", "ce", "d", "s", "g"),
                (r"Q_{\mathrm{h}}", r"Q_{\mathrm{h,ce}}", r"Q_{\mathrm{h,d}}", r"Q_{\mathrm{h,s}}", r"Q_{\mathrm{h,g}}"),
                readouts[:5], colors)
        }
        self.add(readouts[5])
        self.play(FadeIn(name_mobs[5]), e_val.animate.set_value(_Q_E), *running(1.8), run_time=1.8)
        copies = {k: s.copy() for k, s in sources.items()}
        for c in copies.values():
            for part in c.get_family():
                part._layout_zone = "formula"
        self.play(*[ReplacementTransform(copies[k], items[k]) for k in copies], *running(1.2), run_time=1.2)
        rest = VGroup(*[m for m in row.submobjects if all(m is not items[k] for k in copies)])
        self.play(FadeIn(rest), Create(box), *running(0.6), run_time=0.6)
        hold_for(self, N, "sum", during=running)
        #endregion

        #region tip
        caption = swap_caption(self, caption, subtitle_text(N, "tip"))
        on.discard("d")
        self.play(Create(h["sleeves"]), FadeOut(h["pipe_label"]), *running(1.2), run_time=1.2)
        hold_for(self, N, "tip", during=running)
        self.play(FadeOut(caption), run_time=0.3)
        #endregion
#endregion


class FullFinalCalculationVideo(Scene):
    def construct(self):
        scenes = [
            ReviewingHeatLosses,
            Scene2,
            ReviewingHeatGains,
            Scene4,
            UltimateEnergyBalance,
            AnlagenVerluste,
        ]
        base_dir = os.path.dirname(os.path.abspath(__file__))
        audio_files = [
            os.path.join(base_dir, f"scene_{i}_audio.mp3") for i in range(1, len(scenes) + 1)
        ]

        for scene_cls, audio_path in zip(scenes, audio_files):
            if os.path.exists(audio_path):
                self.add_sound(audio_path)
            self.NARRATION = scene_cls.NARRATION
            begin_vo_beat(self, scene_cls.__name__)
            scene_cls.construct(self)
            self.clear()
