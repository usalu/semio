"""🔥 Heating Module 3 — Lüftung im Winter (bridge to the Lüftung video).

Ventilation is explained once, season-neutral, in ``tutorial/energy/demand/2_ventilation``. This module plugs the
winter into it with one short reminder: the airflow V · n (Lüftung Kapitel 2), c_Luft and Δθ = 20 K (Kapitel 3) give
the ventilation loss Φ_V = V · n · c_Luft · Δθ ≈ 1500 W, which stands next to the transmission loss Φ_T ≈ 2300 W of Modul 2; a heat
exchanger (Kapitel 5) leaves Φ_V · (1 − η_WRG) ≈ 300 W. Every shown quantity counts up from 0 and every result is
read from the drawn bar.
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
    beat_subtitle, BEAT_SUBTITLE_FADE, body_text,
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, PASTEL_WHITE, PASTEL_CYAN, PASTEL_TEAL, PASTEL_ORANGE, PASTEL_YELLOW, PASTEL_RED, PASTEL_BLUE, PASTEL_GREEN,
    highlight_param, math_label, math_readout, math_panel, de_num,
    smooth_path, flow_guides, flow_animation, meter, set_meter,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language,
    house_section, open_window,
)

# 🗣️ Timing follows German captions (reading floor in hold_for).
set_vo_language("de")

# 🏔️ Persistent topic title — written once on Beat1.
TITLE_DE = "Modul 3: Lüftung im Winter"


#region Beat helpers
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Same size, colour, opacity and corner as ``_din_ref`` in ``1_introduction/scene_1.py`` and
    ``2_conduction/scene_2.py`` so every module footnotes its norm identically. The winter ventilation loss is the
    chain of DIN EN 12831-1 (Φ_V = V · n · c_Luft · Δθ).
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=PASTEL_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Moving air
def _streams(streams, rt: float, *, speed: float = 0.8, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)
#endregion


#region Running example
# 🧮 One house, one set of numbers. V, n and V̇ come from the Lüftung video (Kapitel 2), c_Luft and Δθ from Kapitel 3,
# η_WRG from Kapitel 5; Φ_T is the transmission loss of Modul 2 (``6_final_calculation/scene_6.py``).
RHO_AIR, CP_AIR = 1.2, 1005.0
C_LUFT = round(RHO_AIR * CP_AIR / 3600.0, 2)
V_NET, N_RATE, D_THETA = 440.0, 0.5, 20.0
V_DOT = V_NET * N_RATE
PHI_V = round(V_DOT * C_LUFT * D_THETA, -2)
PHI_T = 2300.0
ETA_WRG = 0.8
VENT_BLUE = "#60A5FA"
#endregion


#region Back-link chip
def _backlink(origin: str, src: str | None = None, *, color: str = VENT_BLUE):
    """🔗 Chip that names where a reused value comes from — a Lüftung chapter or Modul 2; a typeset quantity is optional."""
    parts = VGroup(body_text(f"← {origin}" + (":" if src else ""), font_size=LABEL_FONT_SIZE, color=color))
    if src:
        parts.add(math_label(src, size=LABEL_FONT_SIZE + 1, color=color))
    parts.arrange(RIGHT, buff=0.1)
    box = SurroundingRectangle(parts, color=color, corner_radius=0.1, buff=0.09, stroke_width=1.6)
    return VGroup(box, parts)


def _outlook_chip(text: str, *, color: str = VENT_BLUE):
    """➡️ Chip that points forward to a chapter of the Lüftung video."""
    label = body_text(text, font_size=LABEL_FONT_SIZE, color=color)
    return VGroup(SurroundingRectangle(label, color=color, corner_radius=0.1, buff=0.1, stroke_width=1.6), label)
#endregion


#region Bar segments
def _segment(x: float, bottom, height, color: str, *, width: float = 1.3, opacity: float = 0.78):
    """📊 Bar segment redrawn every frame from ``bottom()`` and ``height()`` so it grows and shrinks with its tracker."""
    def build():
        h = max(0.004, float(height()))
        return Rectangle(width=width, height=h, color=color, fill_color=color, fill_opacity=opacity,
                         stroke_width=1.2).move_to([x, float(bottom()) + h / 2, 0.0])
    return always_redraw(build)
#endregion


#region Beat1 – Ventilation in winter
class Beat1_LueftungImWinter(Scene):
    """🌬️ Φ_V = V · n · c_Luft · Δθ ≈ 1500 W next to Φ_T ≈ 2300 W, then ·(1 − η_WRG) ≈ 300 W."""

    NARRATION = [
        ("intro",
         "In winter the house loses heat on two paths: through the envelope and with the air that ventilation exchanges.",
         "Im Winter verliert das Haus Wärme auf zwei Wegen: durch die Hülle und mit der Luft, die die Lüftung austauscht."),
        ("formula",
         "From the ventilation video we know the ventilation loss: Phi V equals V times n times c Luft times delta theta.",
         "Aus dem Lüftungsvideo kennen wir den Lüftungsverlust: Phi-V gleich V mal n mal c-Luft mal Delta-Theta."),
        ("recap",
         "A short reminder: 220 cubic metres per hour, 0.34 watt-hours per cubic metre and kelvin, and 20 kelvin in winter — about 1500 watts.",
         "Kurz zur Erinnerung: 220 Kubikmeter pro Stunde, 0,34 Wh pro Kubikmeter und Kelvin und im Winter 20 Kelvin — rund 1500 Watt."),
        ("bars",
         "Next to the transmission loss of about 2300 watts from module 2, ventilation adds 1500 watts: the total loss is 3800 watts.",
         "Neben dem Transmissionsverlust von rund 2300 Watt aus Modul 2 kommen 1500 Watt Lüftung hinzu: Der Gesamtverlust beträgt 3800 Watt."),
        ("wrg",
         "With heat recovery of 0.8 from chapter 5 only 300 watts remain.",
         "Mit Wärmerückgewinnung von 0,8 aus Kapitel 5 bleiben nur 300 Watt."),
        ("link",
         "Purge or tilted window and the system comparison stay in the ventilation video.",
         "Stoß- oder Kipplüftung und der Anlagenvergleich bleiben im Lüftungsvideo."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Lüftungsverlust im Winter", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        #region winter house
        hs = house_section(np.array([-3.9, -0.05, 0.0]), scale=0.9)
        bl, br, tl = hs["bottom_left"], hs["bottom_right"], hs["top_left"]
        x_r, t = br[0], 0.08
        air = Polygon(bl + RIGHT * t + UP * 0.02, br + LEFT * t + UP * 0.02, hs["top_right"] + LEFT * t + DOWN * 0.06,
                      hs["roof_peak"] + DOWN * 0.14, tl + RIGHT * t + DOWN * 0.06,
                      stroke_width=0, fill_color=PASTEL_ORANGE, fill_opacity=0.0).set_z_index(-1)
        y_mid = hs["level_1"].get_center()[1]
        intake, exhaust = [], []
        for win, (yf, yc) in zip(hs["windows"], ((bl[1], y_mid), (y_mid, tl[1]))):
            x_w, sill, lintel = win["x"], win["sill"], win["lintel"]
            for dy in (0.0, 0.1):
                intake.append(smooth_path([np.array([x, y + dy, 0.0]) for x, y in (
                    (x_w - 1.0, sill - 0.06), (x_w - 0.4, sill + 0.06), (x_w + 0.3, sill + 0.02),
                    (x_w + 0.9, yf + 0.2), (x_r - 0.8, yf + 0.18), (x_r - 0.3, yf + 0.55))]))
                exhaust.append(smooth_path([np.array([x, y - dy, 0.0]) for x, y in (
                    (x_r - 0.3, yf + 0.68), (x_r - 0.5, yc - 0.16), (x_w + 1.2, yc - 0.14),
                    (x_w + 0.35, lintel - 0.05), (x_w - 0.35, lintel - 0.05), (x_w - 0.8, lintel + 0.22))]))
        guides = VGroup(flow_guides(intake[::2], PASTEL_BLUE, opacity=0.22),
                        flow_guides(exhaust[::2], PASTEL_ORANGE, opacity=0.22))

        def breeze(rt, waves=3):
            return [_streams([(intake, PASTEL_BLUE, PASTEL_ORANGE), (exhaust, PASTEL_ORANGE)], rt, speed=0.45, waves=waves)]

        th_i = math_label(r"\theta_{\mathrm{i}} = 20\,°\mathrm{C}", np.array([-3.9, 1.27, 0.0]), size=LABEL_FONT_SIZE + 1,
                          color=PASTEL_RED)
        th_e = math_label(r"\theta_{\mathrm{e}} = 0\,°\mathrm{C}", np.array([-6.05, 1.85, 0.0]), size=LABEL_FONT_SIZE + 3,
                          color=PASTEL_BLUE)

        self.play(Create(hs["group"]), run_time=1.4)
        self.add(air)
        self.play(*[open_window(w) for w in hs["windows"]], FadeIn(guides), FadeIn(th_i), FadeIn(th_e),
                  air.animate.set_fill(PASTEL_ORANGE, opacity=0.12), *breeze(1.0), run_time=1.0)
        hold_for(self, N, "intro", used=0.3 + 1.4 + 1.0, during=breeze)
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        parts = [
            ("phi", r"\Phi_{\mathrm{V}}", PASTEL_WHITE), (None, "=", None),
            ("v", "V", PASTEL_ORANGE), (None, r"\cdot", None), ("n", "n", PASTEL_CYAN), (None, r"\cdot", None),
            ("c", r"c_{\mathrm{Luft}}", PASTEL_GREEN), (None, r"\cdot", None), ("dt", r"\Delta\theta", PASTEL_YELLOW),
        ]
        row, box, items = math_panel(parts)
        self.play(FadeIn(row), Create(box), *breeze(0.8), run_time=0.8)
        hold_for(self, N, "formula", during=breeze)
        #endregion

        #region factor cards
        specs = [
            ("vn", r"V \cdot n", "Volumenstrom", V_DOT, 0, r"\mathrm{m^{3}/h}", PASTEL_ORANGE, "Lüftung · Kap. 2", ("v", "n")),
            ("c", r"c_{\mathrm{Luft}}", "Wärmekapazität", C_LUFT, 2, r"\mathrm{Wh/(m^{3}K)}", PASTEL_GREEN, "Lüftung · Kap. 3", ("c",)),
            ("dt", r"\Delta\theta", "Innen − außen", D_THETA, 0, r"\mathrm{K}", PASTEL_YELLOW, "Lüftung · Kap. 3", ("dt",)),
        ]
        trackers = {spec[0]: ValueTracker(0.0) for spec in specs}
        card_w, gap, cy = 2.45, 0.15, 0.95
        xs = [0.025 + i * (card_w + gap) for i in range(3)]
        cards = {}
        for (key, sym, name, value, digits, unit, color, chapter, _keys), x in zip(specs, xs):
            frame = RoundedRectangle(width=card_w, height=1.62, corner_radius=0.1, color=color, stroke_width=1.8)
            frame.set_fill(P_DEEP_DARK, 0.82).move_to([x, cy, 0])
            head = math_label(sym, np.array([x, cy + 0.32, 0]), size=FORMULA_FONT_SIZE, color=color)
            cap = body_text(name, font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE).move_to([x, cy - 0.12, 0])
            val = math_readout(lambda key=key, digits=digits, unit=unit: rf"{de_num(trackers[key].get_value(), digits)}\,{unit}",
                               np.array([x, cy - 0.55, 0]), size=BODY_FONT_SIZE, color=color, edge="center")
            link = _backlink(chapter).move_to([x, cy - 1.2, 0])
            cards[key] = (VGroup(frame, head, cap), val, link, value)
        #endregion

        #region recap — all factors at once, counted from 0
        caption = swap_caption(self, caption, subtitle_text(N, "recap"))
        statics, vals, links, rings, rises = [], [], [], [], []
        for key, _sym, _name, _value, _digits, _unit, color, _chapter, keys in specs:
            static, val, link, value = cards[key]
            statics.append(static); vals.append(val); links.append(link)
            rings += [highlight_param(items, k, color=color) for k in keys]
            rises.append(trackers[key].animate.set_value(value))
        self.play(*[Create(r) for r in rings], *[FadeIn(m, shift=UP * 0.1) for m in statics + links], *breeze(0.5),
                  run_time=0.5)
        self.add(*vals)
        self.play(*rises, *breeze(1.0), run_time=1.0)
        self.play(*[FadeOut(r) for r in rings], *breeze(0.2), run_time=0.2)
        k = ValueTracker(0.0)
        phi_read = math_readout(
            lambda: rf"\Phi_{{\mathrm{{V}}}} = {de_num(V_NET)} \cdot {de_num(N_RATE, 1)} \cdot {de_num(C_LUFT, 2)} \cdot "
                    rf"{de_num(D_THETA * k.get_value())} \approx {de_num(round(PHI_V * k.get_value(), -1))}\,\mathrm{{W}}",
            np.array([2.6, -0.98, 0.0]), size=BODY_FONT_SIZE + 2, color=PASTEL_BLUE, edge="center")
        self.add(phi_read)
        ring = highlight_param(items, "phi", color=PASTEL_BLUE)
        self.play(k.animate.set_value(1.0), Create(ring), *breeze(1.2), run_time=1.2)
        hold_for(self, N, "recap", used=0.35 + 0.5 + 1.0 + 0.2 + 1.2, during=breeze)
        self.play(FadeOut(ring), *breeze(0.2), run_time=0.2)
        #endregion

        #region next to transmission
        caption = swap_caption(self, caption, subtitle_text(N, "bars"))
        for mob in (phi_read, *[c[1] for c in cards.values()]):
            mob.clear_updaters()
        self.play(FadeOut(phi_read), *[FadeOut(VGroup(c[0], c[1], c[2])) for c in cards.values()], *breeze(0.7), run_time=0.7)

        unit, base_y, bar_x, col_x = 3.0 / (PHI_T + PHI_V), -1.2, -0.5, 0.35
        k_t, k_v, eta = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        cut = lambda: PHI_V * (1.0 - ETA_WRG * eta.get_value())
        seg_t = _segment(bar_x, lambda: base_y, lambda: PHI_T * unit * k_t.get_value(), PASTEL_CYAN)
        seg_v = _segment(bar_x, lambda: base_y + PHI_T * unit * k_t.get_value(),
                         lambda: cut() * unit * k_v.get_value(), PASTEL_BLUE)
        t_mid = lambda: [col_x, base_y + 0.5 * PHI_T * unit * k_t.get_value(), 0.0]
        v_mid = lambda: [col_x, base_y + PHI_T * unit * k_t.get_value() + 0.5 * cut() * unit * k_v.get_value(), 0.0]
        t_read = math_readout(lambda: rf"\Phi_{{\mathrm{{T}}}} \approx {de_num(round(PHI_T * k_t.get_value(), -1))}\,\mathrm{{W}}",
                              t_mid, size=BODY_FONT_SIZE, color=PASTEL_CYAN, edge="left")
        v_read = math_readout(lambda: rf"\Phi_{{\mathrm{{V}}}} \approx {de_num(round(cut() * k_v.get_value(), -1))}\,\mathrm{{W}}",
                              v_mid, size=BODY_FONT_SIZE, color=PASTEL_BLUE, edge="left")
        total = math_readout(
            lambda: rf"\Phi_{{\mathrm{{T}}}} + \Phi_{{\mathrm{{V}}}} \approx "
                    rf"{de_num(round(PHI_T * k_t.get_value() + cut() * k_v.get_value(), -1))}\,\mathrm{{W}}",
            [bar_x, 2.12, 0.0],
            size=BODY_FONT_SIZE, color=PASTEL_WHITE, edge="center")
        t_link = _backlink("Modul 2", color=PASTEL_CYAN)
        v_link = _backlink("Lüftung · Kap. 3", r"\dot{Q}_{\mathrm{V,sens}}")
        sheet = Line(np.array([bar_x - 0.9, base_y, 0]), np.array([bar_x + 0.9, base_y, 0]), color=PASTEL_WHITE, stroke_width=2)
        wall_arrows = VGroup(*[Arrow(np.array([hs["bottom_right"][0] + 0.02, y, 0.0]), np.array([hs["bottom_right"][0] + 0.55, y, 0.0]),
                                     color=PASTEL_CYAN, buff=0.0, stroke_width=3, max_tip_length_to_length_ratio=0.35)
                               for y in (-0.7, 0.35)])
        self.add(seg_t, seg_v, t_read, total)
        self.play(Create(sheet), FadeIn(wall_arrows), k_t.animate.set_value(1.0), *breeze(1.2), run_time=1.2)
        self.add(v_read)
        self.play(FadeOut(wall_arrows), k_v.animate.set_value(1.0), *breeze(1.2), run_time=1.2)
        t_link.move_to([col_x + t_link.width / 2, base_y + PHI_T * unit / 2 - 0.45, 0.0])
        v_link.move_to([col_x + v_link.width / 2, v_mid()[1] - 0.45, 0.0])
        self.play(FadeIn(t_link), FadeIn(v_link), *breeze(0.4), run_time=0.4)
        hold_for(self, N, "bars", used=0.35 + 0.7 + 1.2 + 1.2 + 0.4, during=breeze)
        #endregion

        #region heat recovery
        caption = swap_caption(self, caption, subtitle_text(N, "wrg"))
        gauge = meter("Rückgewinnung", length=2.3, thickness=0.34, color=PASTEL_GREEN, vertical=False)
        gauge["group"].move_to([5.3, 1.85, 0.0])
        gauge["fill"].add_updater(lambda m: set_meter(gauge, ETA_WRG * eta.get_value()))
        eta_read = math_readout(lambda: rf"\eta_{{\mathrm{{WRG}}}} = {de_num(ETA_WRG * eta.get_value(), 2)}",
                                np.array([5.3, 1.25, 0.0]), size=BODY_FONT_SIZE, color=PASTEL_GREEN, edge="center")
        eta_link = _backlink("Lüftung · Kap. 5").move_to([5.3, 0.75, 0])
        ghost = always_redraw(lambda: Rectangle(
            width=1.3, height=max(0.004, PHI_V * ETA_WRG * eta.get_value() * unit), color=PASTEL_GREEN, stroke_width=1.6,
            fill_opacity=0.0).move_to([bar_x, base_y + (PHI_T + PHI_V) * unit - PHI_V * ETA_WRG * eta.get_value() * unit / 2, 0.0])
            .set_stroke(opacity=1.0 if eta.get_value() > 0.001 else 0.0))
        saved = math_readout(lambda: rf"-{de_num(round(PHI_V * ETA_WRG * eta.get_value(), -1))}\,\mathrm{{W}}",
                             lambda: [col_x, base_y + (PHI_T + PHI_V) * unit - PHI_V * ETA_WRG * eta.get_value() * unit / 2, 0.0],
                             size=BODY_FONT_SIZE, color=PASTEL_GREEN, edge="left")
        row2, box2, items2 = math_panel(parts + [
            ("dot2", r"\cdot", None), ("eta", r"(1 - \eta_{\mathrm{WRG}})", PASTEL_GREEN)])
        dx = items2["phi"].get_left()[0] - items["phi"].get_left()[0]
        tail = VGroup(items2["dot2"], items2["eta"])
        self.play(FadeIn(VGroup(gauge["track"], gauge["fill"], gauge["label"])), FadeIn(eta_link), FadeOut(v_link),
                  *breeze(0.5), run_time=0.5)
        self.add(eta_read, ghost, saved)
        self.play(row.animate.shift(RIGHT * dx), ReplacementTransform(box, box2), *breeze(0.6), run_time=0.6)
        self.play(FadeIn(tail), *breeze(0.3), run_time=0.3)
        ring = highlight_param(items2, "eta", color=PASTEL_GREEN)
        self.play(eta.animate.set_value(1.0), Create(ring), *breeze(2.0), run_time=2.0)
        hold_for(self, N, "wrg", used=0.35 + 0.5 + 0.6 + 0.3 + 2.0, during=breeze)
        self.play(FadeOut(ring), *breeze(0.2), run_time=0.2)
        #endregion

        #region hand back to the Lüftung video
        caption = swap_caption(self, caption, subtitle_text(N, "link"))
        to_window = _outlook_chip("→ Lüftung · Kap. 4: Stoß- oder Kipplüftung").move_to([3.9, 1.95, 0.0])
        to_systems = _outlook_chip("→ Lüftung · Kap. 5: Anlagenvergleich").move_to([4.3, 1.4, 0.0])
        eta_column = VGroup(gauge["track"], gauge["fill"], gauge["label"], eta_read, eta_link)
        for mob in (eta_read, gauge["fill"]):
            mob.clear_updaters()
        self.play(FadeOut(eta_column), FadeIn(to_window, shift=LEFT * 0.2), FadeIn(to_systems, shift=LEFT * 0.2),
                  *breeze(0.6), run_time=0.6)
        hold_for(self, N, "link", used=0.35 + 0.6, during=breeze)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
