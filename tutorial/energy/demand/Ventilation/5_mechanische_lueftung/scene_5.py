"""⚙️ Ventilation Chapter 5 — Mechanische Lüftung und Wärmerückgewinnung.

Fan-driven ventilation in four beats: the three base types, the plate heat
exchanger with ONE definition of the temperature change ratio η for winter and
summer, the system comparison, and the duct sized from the hygiene airflow.
This chapter owns η_WRG = 0,8 (winter 1500 W → 300 W, summer sensible
450 W → 90 W, latent 810 W untouched) and the duct numbers of the running
example (q_v,R = 0,061 m³/s, v_m = 2,5 m/s, A ≈ 0,0244 m², r ≈ 0,088 m).
The airflow itself (V̇ = 220 m³/h) and the heat flows 1500 W / 450 W / 810 W are
owned by Chapters 2 and 3 and mirrored here as module constants.
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
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE,
)
from types import SimpleNamespace

from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, SAFE_BOTTOM, SAFE_BOTTOM_FORMULA, fit_band,
    smooth_path, flow_guides, flow_animation,
    meter, bind_meter, cross_mark, note_line,
    highlight_param, math_label, math_panel, math_readout, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    room_section, window_glyph, open_window, person_glyph, color_ramp,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent chapter title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Mechanische Lüftung und Wärmerückgewinnung"


#region Shared numbers
Q_V_WINTER, Q_V_SENS_SUMMER, Q_V_LAT_SUMMER = 1500.0, 450.0, 810.0
ETA_WRG = 0.8
V_DOT_M3H = 220.0
Q_VR = V_DOT_M3H / 3600.0
V_M = 2.5
THETA_I_WINTER, THETA_E_WINTER = 20.0, 0.0
THETA_I_SUMMER, THETA_E_SUMMER = 26.0, 32.0
#endregion


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``Heating/2_conduction/scene_2.py``): a dim ``P_TEAL`` footnote
    that never competes with the diagram. The chapter title is wide, so the
    footnote sits one line lower, level with the beat subtitle.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30).shift(DOWN * 0.6)
    return ref
#endregion


#region Shared visual motifs

def _room(center, width, height, color=P_WHITE, t: float = 0.16):
    """🏠 Physical Fundamentals room section — ceiling slab and side walls, then the floor slab, centred on the outline."""
    cx, cy = float(center[0]), float(center[1])
    style = dict(color=color, stroke_width=2, fill_color=color, fill_opacity=0.14)
    ceiling = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy + height / 2, 0.0]))
    walls = VGroup(*[
        Rectangle(width=t, height=height - t, **style).move_to(np.array([cx + s * width / 2, cy, 0.0]))
        for s in (-1, 1)
    ])
    floor = Rectangle(width=width + t, height=t, **style).move_to(np.array([cx, cy - height / 2, 0.0]))
    return VGroup(VGroup(ceiling, walls), floor)


def _fan(pos, color, radius=0.24):
    """🌀 Hub plus three blades — the glyph that marks a stream as mechanically driven."""
    ring = Circle(radius=radius, color=color, stroke_width=2.4).set_fill(P_DEEP_DARK, 1.0)
    blades = VGroup(*[
        Line(ORIGIN, RIGHT * radius * 0.78, color=color, stroke_width=2.4).rotate(a, about_point=ORIGIN)
        for a in (0.0, TAU / 3, 2 * TAU / 3)
    ])
    return VGroup(ring, blades).move_to(pos)


def _spin(fans, rt: float, period: float = 0.9, sign: float = -1.0):
    """🔄 Turn the blades of every ``_fan`` for ``rt`` seconds."""
    return [Rotate(f[1], angle=sign * TAU * rt / period, about_point=f[0].get_center(), rate_func=linear) for f in fans]


def _streams(streams, rt: float, *, speed: float = 0.85, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


def _port(pos, color=P_WHITE):
    """🕳️ Passive wall opening — the uncontrolled half of a single-fan system."""
    return Rectangle(
        width=0.16, height=0.50, color=color, stroke_width=2.6,
        fill_color=P_DEEP_DARK, fill_opacity=1.0,
    ).move_to(pos)


def _stand(room_c, room_h, x, color=P_ORANGE, t: float = 0.16):
    """🧍 Occupant at true scale — 70 % of the inner room height, feet on the floor slab's top edge at ``x``."""
    height = 0.7 * (room_h - t)
    return person_glyph(np.array([x, float(room_c[1]) - room_h / 2 + t / 2 + height / 2, 0.0]),
                        color=color, scale=height / 0.66)


def _math_chip(src: str, color: str):
    """🔖 Boxed typeset label — hand-over chips and inline annotations with real subscripts."""
    label = math_label(src, size=BODY_FONT_SIZE, color=color)
    return VGroup(SurroundingRectangle(label, color=color, corner_radius=0.1, buff=0.14, stroke_width=1.8), label)


_AIR_TEMP = color_ramp([(0.0, P_BLUE), (0.4, P_CYAN), (0.7, P_ORANGE), (1.0, P_RED)])


def _air_color(theta: float):
    """🌡️ Particle colour for an air temperature in °C — cold blue to hot red over 0 … 32 °C."""
    return _AIR_TEMP(theta / 32.0)


def _live_flow(streams, rt: float, *, speed: float = 0.8, waves: int = 4, radius: float = 0.065):
    """🎨 Particle streams coloured by ``color_fn(t)`` along the path every frame — supply air changes colour in the core."""
    dots, meta = VGroup(), []
    for paths, color_fn in streams:
        for path in paths:
            for w in range(waves):
                dot = Ellipse(width=radius * 2.8, height=radius * 1.15, stroke_width=0, fill_opacity=0.0)
                dots.add(dot.move_to(path.point_from_proportion(0.0)))
                meta.append([path, w / waves, color_fn, 0.0])

    def update(group, alpha):
        for dot, m in zip(group, meta):
            path, offset, color_fn, angle = m
            t = (alpha * speed * rt + offset) % 1.0
            a, b = path.point_from_proportion(max(0.0, t - 0.02)), path.point_from_proportion(min(1.0, t + 0.02))
            m[3] = float(np.arctan2(b[1] - a[1], b[0] - a[0]))
            dot.rotate(m[3] - angle).move_to(path.point_from_proportion(t))
            dot.set_fill(color_fn(t), opacity=0.92 * max(0.0, min(1.0, t / 0.08, (1.0 - t) / 0.10)))

    return UpdateFromAlphaFunc(dots, update, remover=True, rate_func=linear)


def _live_value(fn):
    """🔗 Read-only stand-in for a ``ValueTracker`` whose value is computed from the drawn picture — drives ``bind_meter``."""
    return SimpleNamespace(get_value=fn)


def _temp_port(title: str, value_fn, at, color):
    """🏷️ Port tag of a duct — name over a live temperature read from the drawn exchanger."""
    box = RoundedRectangle(width=1.3, height=0.84, corner_radius=0.1, color=color, stroke_width=1.8,
                           fill_color=P_DEEP_DARK, fill_opacity=0.82).move_to(at)
    name = Text(title, font_size=LABEL_FONT_SIZE, color=color).move_to(box.get_center() + UP * 0.24)
    read = math_readout(lambda: rf"{de_num(value_fn())}\,°\mathrm{{C}}", box.get_center() + DOWN * 0.17,
                        size=BODY_FONT_SIZE, color=P_WHITE, edge="center")
    return VGroup(box, name), read


#endregion


#region Beat1 – The three mechanical base types
class Beat1_Grundtypen(Scene):
    NARRATION = [
        ("intro",
         "Fan-assisted ventilation comes in three basic types. They differ in one thing only: which of the two air streams gets a fan.",
         "Ventilatorgestützte Lüftung hat drei Grundtypen — sie\nunterscheiden sich nur darin, wer einen Ventilator bekommt."),
        ("exhaust",
         "The exhaust-only system extracts air mechanically. Fresh air seeps in unfiltered through passive wall vents, and the room sits at a slight negative pressure.",
         "Die Abluftanlage saugt mechanisch ab — die Zuluft kommt\nungefiltert nach, im Raum herrscht leichter Unterdruck."),
        ("supply",
         "The supply-only system does the opposite. Treated air is pushed in and leaks back out through joints and gaps, at a slight positive pressure.",
         "Die Zuluftanlage macht es umgekehrt: aufbereitete Luft\nwird eingeblasen und entweicht über Fugen — Überdruck."),
        ("balanced",
         "The balanced supply and exhaust system ducts both streams. The pressures cancel out, and both volume flows are known quantities.",
         "Die Zu-/Abluftanlage führt beide Ströme — die Drücke\ngleichen sich aus, beide Volumenströme sind bekannt."),
        ("verdict",
         "Only that third type has both streams under control, and only then do filtering, humidity control and heat recovery become possible, in winter as in summer.",
         "Nur der dritte Typ hat beide Ströme unter Kontrolle — erst dann\nsind Filter, Feuchte und WRG möglich, im Winter wie im Sommer."),
    ]

    CENTERS = (
        np.array([-4.55, 0.35, 0.0]),
        np.array([0.0, 0.35, 0.0]),
        np.array([4.55, 0.35, 0.0]),
    )
    ROOM_W, ROOM_H = 3.70, 1.90

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Drei mechanische Grundtypen", title)
        din = _din_ref("DIN EN 16798-3")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        c0, c1, c2 = self.CENTERS
        shells = [_room(c, self.ROOM_W, self.ROOM_H) for c in self.CENTERS]
        rooms = VGroup(*[s[0] for s in shells])
        floors = VGroup(*[s[1] for s in shells])
        wall_y = rooms[0].get_top()[1]

        headers = VGroup(*[
            Text(t, font_size=BODY_FONT_SIZE, color=c).move_to(np.array([cx[0], 1.98, 0.0]))
            for t, c, cx in (
                ("Abluftanlage", P_ORANGE, c0),
                ("Zuluftanlage", P_CYAN, c1),
                ("Zu-/Abluftanlage", P_GREEN, c2),
            )
        ])

        def _duct_arrow(start, end, color):
            return Arrow(start, end, buff=0, color=color, stroke_width=4,
                         max_tip_length_to_length_ratio=0.26)

        exh_fan = _fan(np.array([c0[0] + 1.25, wall_y, 0.0]), P_ORANGE)
        exh_out = _duct_arrow(np.array([c0[0] + 1.58, wall_y, 0.0]),
                              np.array([c0[0] + 2.10, wall_y, 0.0]), P_ORANGE)
        ald = _port(np.array([c0[0] - self.ROOM_W / 2, 0.70, 0.0]))
        exh_paths = VGroup(*[
            smooth_path([
                ald.get_right(),
                c0 + LEFT * 0.6 + UP * (dy * 0.5),
                c0 + RIGHT * 0.8 + UP * (dy * 0.35 + 0.25),
                np.array([c0[0] + 1.25, wall_y - 0.08, 0.0]),
            ])
            for dy in (-0.6, 0.0, 0.55)
        ])

        sup_fan = _fan(np.array([c1[0] - 1.25, wall_y, 0.0]), P_CYAN)
        sup_in = _duct_arrow(np.array([c1[0] - 2.10, wall_y, 0.0]),
                             np.array([c1[0] - 1.58, wall_y, 0.0]), P_CYAN)
        leak = _port(np.array([c1[0] + self.ROOM_W / 2, 0.70, 0.0]))
        sup_paths = VGroup(*[
            smooth_path([
                np.array([c1[0] - 1.25, wall_y - 0.08, 0.0]),
                c1 + LEFT * 0.8 + UP * (dy * 0.35 + 0.25),
                c1 + RIGHT * 0.6 + UP * (dy * 0.5),
                leak.get_left(),
            ])
            for dy in (-0.6, 0.0, 0.55)
        ])

        bal_sup = _fan(np.array([c2[0] - 1.25, wall_y, 0.0]), P_CYAN)
        bal_exh = _fan(np.array([c2[0] + 1.25, wall_y, 0.0]), P_ORANGE)
        bal_in = _duct_arrow(np.array([c2[0] - 2.10, wall_y, 0.0]),
                             np.array([c2[0] - 1.58, wall_y, 0.0]), P_CYAN)
        bal_out = _duct_arrow(np.array([c2[0] + 1.58, wall_y, 0.0]),
                              np.array([c2[0] + 2.10, wall_y, 0.0]), P_ORANGE)
        bal_paths = VGroup(*[
            smooth_path([
                np.array([c2[0] - 1.25, wall_y - 0.08, 0.0]),
                c2 + LEFT * 0.7 + UP * (dy * 0.4 + 0.1),
                c2 + RIGHT * 0.7 + UP * (dy * 0.4 - 0.05),
                np.array([c2[0] + 1.25, wall_y - 0.08, 0.0]),
            ])
            for dy in (-0.6, 0.0, 0.55)
        ])

        signs = VGroup(*[
            Text(t, font_size=BODY_FONT_SIZE, color=c).move_to(np.array([cx[0], -0.86, 0.0]))
            for (t, c), cx in zip((("−", P_ORANGE), ("+", P_CYAN), ("=", P_GREEN)), self.CENTERS)
        ])
        sign_tags = VGroup(*[
            Text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([cx[0], -1.18, 0.0]))
            for (t, c), cx in zip(
                (("Unterdruck", P_ORANGE), ("Überdruck", P_CYAN), ("ausgeglichen", P_GREEN)),
                self.CENTERS,
            )
        ])
        port_tags = VGroup(*[
            Text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([cx[0], -1.54, 0.0]))
            for (t, c), cx in zip(
                (("Außenluft ungefiltert", P_WHITE),
                 ("Abluft über Fugen", P_WHITE),
                 ("beide Ströme geführt", P_GREEN)),
                self.CENTERS,
            )
        ])

        verdict = Text(
            "erst beide Ströme geführt → Filter, Feuchte, Wärmerückgewinnung möglich",
            font_size=BODY_FONT_SIZE, color=P_GREEN,
        ).move_to(np.array([0.0, -2.18, 0.0]))

        scaffold = VGroup(rooms, floors, headers, exh_fan, exh_out, ald, sup_fan, sup_in, leak,
                          bal_sup, bal_exh, bal_in, bal_out, signs, sign_tags, port_tags, verdict)
        fit_band(scaffold)

        running = {"streams": [], "fans": []}

        def machines(rt):
            if not running["streams"]:
                return []
            return [_streams(running["streams"], rt, speed=0.8), *_spin(running["fans"], rt)]

        self.play(
            LaggedStart(*[Create(r) for r in rooms], lag_ratio=0.22),
            LaggedStart(*[Create(f) for f in floors], lag_ratio=0.22),
            LaggedStart(*[FadeIn(h, shift=DOWN * 0.1) for h in headers], lag_ratio=0.22),
            run_time=2.0,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.0)

        for key, new_fans, extras, guide, stream, sign in (
            ("exhaust", [exh_fan], [GrowArrow(exh_out), FadeIn(ald), FadeIn(port_tags[0])],
             (exh_paths, P_ORANGE), (exh_paths, P_WHITE, P_ORANGE), 0),
            ("supply", [sup_fan], [GrowArrow(sup_in), FadeIn(leak), FadeIn(port_tags[1])],
             (sup_paths, P_CYAN), (sup_paths, P_CYAN, P_WHITE), 1),
            ("balanced", [bal_sup, bal_exh], [GrowArrow(bal_in), GrowArrow(bal_out), FadeIn(port_tags[2])],
             (bal_paths, P_GREEN), (bal_paths, P_CYAN, P_ORANGE), 2),
        ):
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            self.play(*[FadeIn(f) for f in new_fans], *extras, Create(flow_guides(*guide)), *machines(1.4),
                      run_time=1.4)
            running["streams"].append(stream)
            running["fans"].extend(new_fans)
            self.play(*machines(2.4), run_time=2.4)
            self.play(FadeIn(signs[sign], scale=1.2), FadeIn(sign_tags[sign]), *machines(0.8), run_time=0.8)
            hold_for(self, self.NARRATION, key, during=machines)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verdict"))
        pick = SurroundingRectangle(
            VGroup(rooms[2], headers[2], signs[2], sign_tags[2], port_tags[2], bal_in, bal_out),
            color=P_GREEN, buff=0.14, corner_radius=0.1, stroke_width=2.5,
        )
        self.play(Create(pick), FadeIn(verdict, shift=UP * 0.12), *machines(1.3), run_time=1.3)
        hold_for(self, self.NARRATION, "verdict", during=machines)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Heat recovery: one definition of η for winter and summer
class Beat2_Waermerueckgewinnung(Scene):
    NARRATION = [
        ("intro",
         "Once both streams are ducted, they can be run past each other in a plate heat exchanger without ever mixing.",
         "Sobald beide Ströme geführt sind, laufen sie im Platten-\nwärmeübertrager aneinander vorbei, ohne sich zu mischen."),
        ("winter",
         "In winter the twenty-degree exhaust air gives heat to the zero-degree outdoor air, and the supply air rises to sixteen degrees.",
         "Im Winter gibt die 20 °C warme Abluft Wärme an die 0 °C\nkalte Außenluft ab — die Zuluft steigt auf 16 °C."),
        ("formula",
         "The temperature change ratio eta is the warming the supply air actually gained, divided by the difference that was available between exhaust and outdoor air.",
         "Der Temperaturänderungsgrad η: erreichte Erwärmung der Zuluft,\ngeteilt durch die verfügbare Differenz Abluft − Außenluft."),
        ("winter_bars",
         "Only the share one minus eta stays as a loss: at eta equal to zero point eight, fifteen hundred watts shrink to three hundred.",
         "Nur der Anteil 1 − η bleibt als Verlust: bei η = 0,8\nsinken 1500 W auf 300 W."),
        ("summer",
         "In summer the same definition holds: outdoor air at thirty-two degrees, exhaust at twenty-six. About five of six kelvin are gained, so heat recovery becomes cold recovery.",
         "Im Sommer gilt dieselbe Definition: Außenluft 32 °C, Abluft 26 °C —\nfast fünf von sechs Kelvin, die WRG wird zur Kälterückgewinnung."),
        ("summer_bars",
         "The sensible cooling load drops from four hundred fifty to about ninety watts.",
         "Die fühlbare Last sinkt von 450 W auf rund 90 W."),
        ("latent",
         "A plate exchanger transfers sensible heat only. The latent load of 810 watts stays, which is why cooling will later have to dehumidify.",
         "Der Plattenübertrager überträgt nur fühlbare Wärme: die latente\nLast von 810 W bleibt — die Kühlung muss später entfeuchten."),
        ("close",
         "One definition, two seasons: heating will calculate with 300 watts, cooling with 90 watts sensible plus 810 watts latent.",
         "Eine Definition, zwei Jahreszeiten: die Heizung rechnet mit 300 W,\ndie Kühlung mit 90 W fühlbar plus 810 W latent."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmerückgewinnung im Plattenübertrager", title)
        din = _din_ref("DIN EN 308")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        season = ValueTracker(0.0)
        gain = ValueTracker(0.0)

        def t_aul():
            return THETA_E_WINTER + (THETA_E_SUMMER - THETA_E_WINTER) * season.get_value()

        def t_abl():
            return THETA_I_WINTER + (THETA_I_SUMMER - THETA_I_WINTER) * season.get_value()

        def t_zul():
            return t_aul() + gain.get_value() * (t_abl() - t_aul())

        def t_fol():
            return t_abl() - gain.get_value() * (t_abl() - t_aul())

        core_c = np.array([-2.55, 0.95, 0.0])
        y_sup, y_exh = 1.45, 0.45
        x_in, x_out = -5.0, -0.1
        core = Rectangle(width=2.8, height=1.8, color=P_WHITE, stroke_width=3.0).move_to(core_c)
        plate = Line(core.get_left(), core.get_right(), color=P_TEAL, stroke_width=3.0)
        fins = VGroup(*[
            Line(np.array([x, core_c[1] - 0.09, 0.0]), np.array([x, core_c[1] + 0.09, 0.0]), color=P_TEAL, stroke_width=1.6)
            for x in np.linspace(core_c[0] - 1.1, core_c[0] + 1.1, 9)
        ])
        stubs = VGroup(*[
            Line(np.array([xa, y + dy, 0.0]), np.array([xb, y + dy, 0.0]), color=P_WHITE, stroke_width=1.6).set_stroke(opacity=0.7)
            for y in (y_sup, y_exh) for dy in (-0.22, 0.22)
            for xa, xb in ((x_in, core.get_left()[0]), (core.get_right()[0], x_out))
        ])
        fans = [_fan(np.array([-4.45, y_sup, 0.0]), P_WHITE, radius=0.17),
                _fan(np.array([-0.62, y_exh, 0.0]), P_WHITE, radius=0.17)]
        core_tag = Text("Plattenwärmeübertrager", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(np.array([core_c[0], -0.22, 0.0]))

        supply = VGroup(smooth_path([np.array([x_in, y_sup, 0.0]), np.array([-3.0, y_sup, 0.0]),
                                     np.array([-2.0, y_sup, 0.0]), np.array([x_out, y_sup, 0.0])]))
        exhaust = VGroup(smooth_path([np.array([x_out, y_exh, 0.0]), np.array([-2.0, y_exh, 0.0]),
                                      np.array([-3.0, y_exh, 0.0]), np.array([x_in, y_exh, 0.0])]))

        def heat_arrows(up: bool):
            ya, yb = (y_exh + 0.14, y_sup - 0.14) if up else (y_sup - 0.14, y_exh + 0.14)
            return VGroup(*[
                Arrow(np.array([x, ya, 0.0]), np.array([x, yb, 0.0]), buff=0, color=P_YELLOW, stroke_width=3.5,
                      max_tip_length_to_length_ratio=0.3)
                for x in (core_c[0] - 0.8, core_c[0], core_c[0] + 0.8)
            ])

        arrows_up, arrows_down = heat_arrows(True), heat_arrows(False)

        aul_box, aul_read = _temp_port("Außenluft", t_aul, np.array([-5.95, y_sup, 0.0]), P_WHITE)
        zul_box, zul_read = _temp_port("Zuluft", t_zul, np.array([0.75, y_sup, 0.0]), P_CYAN)
        abl_box, abl_read = _temp_port("Abluft", t_abl, np.array([0.75, y_exh, 0.0]), P_TEAL)
        fol_box, fol_read = _temp_port("Fortluft", t_fol, np.array([-5.95, y_exh, 0.0]), P_ORANGE)

        def stream_color(t_from, t_to):
            def fn(t):
                f = float(np.clip((t - 0.21) / 0.58, 0.0, 1.0))
                f = f * f * (3.0 - 2.0 * f)
                return _air_color(t_from() + f * (t_to() - t_from()))
            return fn

        air = [([supply[0]], stream_color(t_aul, t_zul)), ([exhaust[0]], stream_color(t_abl, t_fol))]
        running = []

        def exchanger(rt):
            if not running:
                return []
            return [_live_flow(air, rt, speed=0.35, waves=7), *_spin(fans, rt)]

        chip_winter = _math_chip(r"\text{Winter}", P_BLUE).move_to(np.array([core_c[0], 2.2, 0.0]))
        chip_summer = _math_chip(r"\text{Sommer}", P_ORANGE).move_to(chip_winter)

        eta_gauge = meter("η", length=3.0, thickness=0.3, color=P_GREEN, vertical=False)
        eta_gauge["group"] = VGroup(eta_gauge["track"], eta_gauge["fill"])
        eta_gauge["group"].move_to(np.array([core_c[0] - 0.4, -0.72, 0.0]))
        bind_meter(eta_gauge, gain)
        eta_read = math_readout(lambda: rf"\eta = {de_num(gain.get_value(), 2)}",
                                eta_gauge["track"].get_right() + RIGHT * 0.25, size=BODY_FONT_SIZE, color=P_GREEN)

        def eta_panel(value_src):
            row, box, items = math_panel([
                ("eta", r"\eta", P_YELLOW), (None, "=", P_WHITE),
                ("frac",
                 rf"\frac{{\textcolor{{{P_CYAN}}}{{\theta_{{\mathrm{{ZUL}}}} - \theta_{{\mathrm{{AUL}}}}}}}}"
                 rf"{{\textcolor{{{P_TEAL}}}{{\theta_{{\mathrm{{ABL}}}} - \theta_{{\mathrm{{AUL}}}}}}}}",
                 P_WHITE),
                (None, "=", P_WHITE),
                ("val", value_src, P_YELLOW),
            ], color=P_YELLOW)
            items["num"], items["den"] = items["frac"][0][0], items["frac"][0][2]
            return row, box, items

        def loss_panel(value_src):
            return math_panel([
                ("q", r"\dot{Q}_{\mathrm{V,WRG}}", P_RED), (None, "=", P_WHITE),
                ("sens", r"\dot{Q}_{\mathrm{V,sens}}", P_RED), (None, r"\cdot", P_WHITE),
                ("eta", r"(1-\eta)", P_GREEN), (None, "=", P_WHITE),
                ("val", value_src, P_RED),
            ], color=P_GREEN)

        self.play(Create(core), Create(plate), FadeIn(fins), Create(stubs), FadeIn(core_tag), FadeIn(chip_winter),
                  run_time=1.6)
        self.play(FadeIn(fans[0]), FadeIn(fans[1]), Create(flow_guides(supply, P_WHITE, opacity=0.2)),
                  Create(flow_guides(exhaust, P_WHITE, opacity=0.2)),
                  FadeIn(aul_box), FadeIn(zul_box), FadeIn(abl_box), FadeIn(fol_box), run_time=1.0)
        self.add(aul_read, zul_read, abl_read, fol_read)
        running.append(True)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.6, during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        self.play(LaggedStart(*[GrowArrow(a) for a in arrows_up], lag_ratio=0.25),
                  gain.animate(rate_func=smooth).set_value(ETA_WRG), *exchanger(3.2), run_time=3.2)
        hold_for(self, N, "winter", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "formula"))
        eq, eq_box, items = eta_panel(r"\frac{16\,\mathrm{K}}{20\,\mathrm{K}} = 0{,}8")
        self.add(eta_read)
        self.play(FadeIn(eta_gauge["group"]), FadeIn(eq), Create(eq_box), *exchanger(1.2), run_time=1.2)
        ring_num = highlight_param(items, "num", color=P_CYAN)
        self.play(Create(ring_num), Indicate(zul_box, color=P_CYAN), Indicate(aul_box, color=P_WHITE),
                  gain.animate(rate_func=smooth).set_value(0.4), *exchanger(1.4), run_time=1.4)
        self.play(FadeOut(ring_num), gain.animate(rate_func=smooth).set_value(ETA_WRG), *exchanger(1.2), run_time=1.2)
        ring_den = highlight_param(items, "den", color=P_TEAL)
        self.play(Create(ring_den), Indicate(abl_box, color=P_TEAL), Indicate(aul_box, color=P_WHITE),
                  *exchanger(1.0), run_time=1.0)
        hold_for(self, N, "formula", during=exchanger)
        self.play(FadeOut(ring_den), *exchanger(0.25), run_time=0.25)

        bx0, unit = 2.35, 0.0029
        y_w, y_s = 1.0, -0.75

        def seg(x_from, width, color, y, opacity=0.78):
            return Rectangle(width=max(width, 0.002), height=0.40, color=color, fill_color=color, fill_opacity=opacity,
                             stroke_width=1.2).move_to(np.array([x_from + width / 2, y, 0.0]))

        def head(text, color, y):
            label = Text(text, font_size=LABEL_FONT_SIZE, color=color)
            return label.move_to(np.array([bx0 + label.width / 2, y + 0.95, 0.0]))

        def name(src, x, y, color):
            return math_label(src, np.array([x, y + 0.42, 0.0]), size=LABEL_FONT_SIZE, color=color)

        def value(src, x, y, color):
            return math_label(src, np.array([x, y - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=color)

        caption = swap_caption(self, caption, subtitle_text(N, "winter_bars"))
        full_w = Q_V_WINTER * unit
        w_head = head("Winter · Heizung", P_ORANGE, y_w)
        w_name = name(r"\dot{Q}_{\mathrm{V,sens}}", bx0 + full_w / 2, y_w, P_RED)
        w_full = seg(bx0, full_w, P_RED, y_w)
        wf = ValueTracker(0.0)
        w_read = math_readout(lambda: rf"{de_num(Q_V_WINTER * wf.get_value())}\,\mathrm{{W}}",
                              np.array([bx0 + full_w / 2, y_w - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=P_RED,
                              edge="center")
        self.add(w_read)
        self.play(FadeIn(w_head), FadeIn(w_name), GrowFromEdge(w_full, LEFT), wf.animate.set_value(1.0),
                  *exchanger(1.4), run_time=1.4)
        self.remove(w_read)
        w_val = value(rf"{de_num(Q_V_WINTER)}\,\mathrm{{W}}", bx0 + full_w / 2, y_w, P_RED)
        self.add(w_val)
        self.play(*exchanger(1.2), run_time=1.2)
        w_green = seg(bx0, full_w * ETA_WRG, P_GREEN, y_w)
        w_rest = seg(bx0 + full_w * ETA_WRG, full_w * (1 - ETA_WRG), P_RED, y_w)
        w_green_val = value(rf"−{de_num(Q_V_WINTER * ETA_WRG)}\,\mathrm{{W}}", bx0 + full_w * ETA_WRG / 2, y_w, P_GREEN)
        w_rest_val = value(rf"{de_num(Q_V_WINTER * (1 - ETA_WRG))}\,\mathrm{{W}}",
                           bx0 + full_w * (ETA_WRG + (1 - ETA_WRG) / 2), y_w, P_RED)
        lp = loss_panel(r"300\,\mathrm{W}")
        self.play(ReplacementTransform(w_full, w_rest), GrowFromEdge(w_green, LEFT),
                  ReplacementTransform(w_val, w_rest_val), FadeIn(w_green_val),
                  ReplacementTransform(eq, lp[0]), ReplacementTransform(eq_box, lp[1]),
                  *exchanger(2.0), run_time=2.0)
        hold_for(self, N, "winter_bars", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        eq2, eq2_box, items2 = eta_panel(r"\frac{5\,\mathrm{K}}{6\,\mathrm{K}} \approx 0{,}8")
        self.play(ReplacementTransform(chip_winter, chip_summer),
                  ReplacementTransform(arrows_up, arrows_down),
                  season.animate(rate_func=smooth).set_value(1.0),
                  ReplacementTransform(lp[0], eq2), ReplacementTransform(lp[1], eq2_box),
                  *exchanger(3.0), run_time=3.0)
        ring_val = highlight_param(items2, "val", color=P_YELLOW)
        self.play(Create(ring_val), Indicate(zul_box, color=P_CYAN), Indicate(abl_box, color=P_TEAL), *exchanger(1.2),
                  run_time=1.2)
        hold_for(self, N, "summer", during=exchanger)
        self.play(FadeOut(ring_val), *exchanger(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "summer_bars"))
        sens_w, lat_w = Q_V_SENS_SUMMER * unit, Q_V_LAT_SUMMER * unit
        s_head = head("Sommer · Kühlung", P_CYAN, y_s)
        s_name = name(r"\dot{Q}_{\mathrm{V,sens}}", bx0 + sens_w / 2, y_s, P_RED)
        l_name = name(r"\dot{Q}_{\mathrm{V,lat}}", bx0 + sens_w + lat_w / 2, y_s, P_CYAN)
        s_full = seg(bx0, sens_w, P_RED, y_s)
        l_full = seg(bx0 + sens_w, lat_w, P_CYAN, y_s)
        sf = ValueTracker(0.0)
        s_read = math_readout(lambda: rf"{de_num(Q_V_SENS_SUMMER * sf.get_value())}\,\mathrm{{W}}",
                              np.array([bx0 + sens_w / 2, y_s - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=P_RED,
                              edge="center")
        l_read = math_readout(lambda: rf"{de_num(Q_V_LAT_SUMMER * sf.get_value())}\,\mathrm{{W}}",
                              np.array([bx0 + sens_w + lat_w / 2, y_s - 0.42, 0.0]), size=LABEL_FONT_SIZE, color=P_CYAN,
                              edge="center")
        self.add(s_read, l_read)
        self.play(FadeIn(s_head), FadeIn(s_name), FadeIn(l_name), GrowFromEdge(s_full, LEFT), GrowFromEdge(l_full, LEFT),
                  sf.animate.set_value(1.0), *exchanger(1.4), run_time=1.4)
        self.remove(s_read, l_read)
        s_val = value(rf"{de_num(Q_V_SENS_SUMMER)}\,\mathrm{{W}}", bx0 + sens_w / 2, y_s, P_RED)
        l_val = value(rf"{de_num(Q_V_LAT_SUMMER)}\,\mathrm{{W}}", bx0 + sens_w + lat_w / 2, y_s, P_CYAN)
        self.add(s_val, l_val)
        self.play(*exchanger(1.0), run_time=1.0)
        s_green = seg(bx0, sens_w * ETA_WRG, P_GREEN, y_s)
        s_rest = seg(bx0 + sens_w * ETA_WRG, sens_w * (1 - ETA_WRG), P_RED, y_s)
        s_green_val = value(rf"−{de_num(Q_V_SENS_SUMMER * ETA_WRG)}\,\mathrm{{W}}", bx0 + 0.02, y_s, P_GREEN)
        s_rest_val = value(rf"{de_num(Q_V_SENS_SUMMER * (1 - ETA_WRG))}\,\mathrm{{W}}", bx0 + sens_w + 0.02, y_s, P_RED)
        sp = math_panel([
            ("q", r"\dot{Q}_{\mathrm{V,sens,WRG}}", P_RED), (None, "=", P_WHITE),
            ("sens", r"\dot{Q}_{\mathrm{V,sens}}", P_RED), (None, r"\cdot", P_WHITE),
            ("eta", r"(1-\eta)", P_GREEN), (None, r"\approx", P_WHITE),
            ("val", r"90\,\mathrm{W}", P_RED),
        ], color=P_GREEN)
        self.play(ReplacementTransform(s_full, s_rest), GrowFromEdge(s_green, LEFT),
                  ReplacementTransform(s_val, s_rest_val), FadeIn(s_green_val),
                  ReplacementTransform(eq2, sp[0]), ReplacementTransform(eq2_box, sp[1]),
                  *exchanger(2.0), run_time=2.0)
        hold_for(self, N, "summer_bars", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "latent"))
        keep = Text("unverändert", font_size=LABEL_FONT_SIZE, color=P_DEEP_DARK).move_to(
            np.array([bx0 + sens_w + lat_w / 2, y_s, 0.0]))
        lp2 = math_panel([
            ("sens", r"\dot{Q}_{\mathrm{V,sens}}\cdot(1-\eta)", P_RED), (None, "+", P_WHITE),
            ("lat", r"\dot{Q}_{\mathrm{V,lat}}", P_CYAN), (None, "=", P_WHITE),
            ("val", r"90\,\mathrm{W} + 810\,\mathrm{W}", P_WHITE),
        ], color=P_CYAN)
        self.play(ReplacementTransform(sp[0], lp2[0]), ReplacementTransform(sp[1], lp2[1]), FadeIn(keep),
                  Indicate(l_full, color=P_CYAN, scale_factor=1.06), *exchanger(1.6), run_time=1.6)
        hold_for(self, N, "latent", during=exchanger)

        caption = swap_caption(self, caption, subtitle_text(N, "close"))
        self.play(Indicate(w_rest, color=P_ORANGE, scale_factor=1.15), Indicate(s_rest, color=P_CYAN, scale_factor=1.3),
                  *exchanger(1.6), run_time=1.6)
        hold_for(self, N, "close", during=exchanger)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Ventilation systems compared
class Beat3_Anlagenvergleich(Scene):
    NARRATION = [
        ("window",
         "Window ventilation is a free air change without any recovery: the full ventilation loss of fifteen hundred watts remains, eta equals zero.",
         "Fensterlüftung ist freier Luftwechsel ohne Rückgewinnung — der\nvolle Lüftungsverlust von 1500 W bleibt, η = 0."),
        ("central",
         "A central unit with heat recovery reaches an eta of about zero point eight to zero point nine and cuts the loss to between 300 and 150 watts.",
         "Eine zentrale Anlage mit WRG erreicht η von etwa 0,8 bis 0,9 —\nder Verlust sinkt auf 300 bis 150 W."),
        ("decentral",
         "Decentralised ceramic push-pull units store the heat alternately and recover seventy to ninety percent, room by room.",
         "Dezentrale Keramik-Pendellüfter speichern die Wärme abwechselnd\nund gewinnen raumweise 70 bis 90 % zurück."),
        ("outro",
         "DIN 1946-6: mechanical recovery can cut the ventilation heat loss by up to ninety percent in winter, and in summer the same technology recovers cold.",
         "DIN 1946-6: mechanische WRG senkt den Lüftungswärmeverlust im Winter\num bis zu 90 % — im Sommer gewinnt sie dieselbe Kälte zurück."),
        ("passivhaus",
         "An open window recovers nothing. That is why the Passivhaus ducts its air through a heat recovery unit and gives up window ventilation as the main air path.",
         "Ein offenes Fenster gewinnt nichts zurück — das Passivhaus führt\ndie Luft deshalb über die WRG, nicht über das Fenster."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Lüftungssysteme im Vergleich", title)
        din = _din_ref("DIN 1946-6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "window"))
        self.play(FadeIn(caption), run_time=0.3)

        N = self.NARRATION
        col_w, col_h = 4.3, 4.6
        pivot = np.array([0.0, -0.25, 0.0])
        room_w, room_h, slab = 2.3, 1.4, 0.13

        def place(*mobs):
            for m in mobs:
                m.scale(0.85, about_point=pivot).shift(UP * 0.35)

        def path(*pts):
            return smooth_path([np.array([x, y, 0.0]) for x, y in pts])

        def legend(lines, color, scale, at):
            return VGroup(*[Text(t, font_size=LABEL_FONT_SIZE, color=color) for t in lines]).arrange(DOWN, buff=0.15)\
                .scale(scale).move_to(at)

        def loss_bar(lo, hi, color):
            width = 1.6
            track = Rectangle(width=width, height=0.2, color=P_WHITE, stroke_width=1.6, fill_color=P_DEEP_DARK, fill_opacity=1.0)
            upper = Rectangle(width=width * hi, height=0.2, stroke_width=0, fill_color=color, fill_opacity=0.35)
            lower = Rectangle(width=width * lo, height=0.2, stroke_width=0, fill_color=color, fill_opacity=0.9)
            watts = (rf"\dot{{Q}}_{{\mathrm{{V}}}} = {de_num(Q_V_WINTER * lo)}\,\mathrm{{W}}" if lo == hi
                     else rf"\dot{{Q}}_{{\mathrm{{V}}}} = {de_num(Q_V_WINTER * lo)}\ldots{de_num(Q_V_WINTER * hi)}\,\mathrm{{W}}")
            label = math_label(watts, size=LABEL_FONT_SIZE, color=color)
            label.next_to(track, RIGHT, buff=0.15)
            return {"track": track, "upper": upper, "lower": lower, "group": VGroup(track, label)}

        def panel(px, color, fill, main, sub, eta_src, bar):
            card = RoundedRectangle(width=col_w, height=col_h, corner_radius=0.15, color=color, fill_color=fill,
                                    fill_opacity=0.85).move_to([px, -0.25, 0])
            hdr = VGroup(Text(main, font_size=LABEL_FONT_SIZE, color=color),
                         Text(sub, font_size=LABEL_FONT_SIZE, color=GREY_A)).arrange(DOWN, buff=0.05)
            hdr.move_to(card.get_top() + DOWN * 0.45)
            eta_lbl = math_label(eta_src, size=LABEL_FONT_SIZE, color=color)
            box = VGroup(eta_lbl, bar["group"]).arrange(DOWN, buff=0.12).move_to(card.get_bottom() + UP * 0.45)
            for fill in (bar["upper"], bar["lower"]):
                fill.move_to(bar["track"].get_center()).align_to(bar["track"], LEFT)
            room = room_section(np.array([px + 0.35, -0.85, 0.0]), w=room_w, h=room_h, wall=0.13, slab=slab,
                                window=(0.3, 0.75))
            for part in room["shell"]:
                part.set_stroke(P_WHITE).set_fill(P_WHITE, opacity=0.14)
            room["glass"].set_stroke(P_CYAN).set_fill(P_CYAN, opacity=0.15)
            height = 0.7 * room_h
            person = person_glyph([px + 1.17, room["y_f"] + height / 2, 0], color=P_ORANGE, scale=height / 0.66)
            return card, hdr, box, room, person

        p1 = -4.4
        bar1 = loss_bar(1.0, 1.0, P_RED)
        card1, hdr1, box1_stat, room1, person1 = panel(
            p1, P_RED, "#1a1215", "Fensterlüftung", "freie Lüftung ohne WRG", r"\eta_{\mathrm{WRG}} = 0", bar1)
        x_l, y_f, y_c, lo, hi = room1["x_l"], room1["y_f"], room1["y_c"], room1["win_lo"], room1["win_hi"]
        win1 = window_glyph(room1["glass_x"], lo, hi, depth=0.4, color=P_CYAN)
        in1 = [path((x_l - 1.05, lo - 0.05), (x_l - 0.4, lo + 0.08), (x_l + 0.3, lo + 0.06), (x_l + 0.8, y_f + 0.15),
                    (p1 + 0.5, y_f + 0.2))]
        out1 = [path((p1 + 0.55, y_c - 0.35), (p1 + 0.3, y_c - 0.15), (x_l + 0.45, hi - 0.06), (x_l - 0.35, hi - 0.04),
                     (x_l - 1.0, hi + 0.35))]
        lbl_in1 = legend(["Kaltluft", "(0 °C)"], P_BLUE, 0.75, [p1 - 1.4, -1.55, 0])
        lbl_out1 = legend(["Warmluft", "(20 °C)"], P_ORANGE, 0.75, [p1 - 1.3, 0.45, 0])
        col1 = VGroup(card1, hdr1, room1["shell"], win1["group"], person1, lbl_in1, lbl_out1, box1_stat)

        p2 = 0.0
        bar2 = loss_bar(0.1, 0.2, P_GREEN)
        card2, hdr2, box2_stat, room2, person2 = panel(
            p2, P_BLUE, "#101825", "Zentrale WRG", "zentrales Lüftungsgerät", r"\eta_{\mathrm{WRG}} \approx 0{,}8 \ldots 0{,}9", bar2)
        y_c2 = room2["y_c"]
        mvhr_box = RoundedRectangle(width=1.5, height=0.5, corner_radius=0.05, color=P_WHITE, fill_color="#2C3E50",
                                    fill_opacity=0.95, stroke_width=2).move_to([p2 + 0.15, 0.6, 0])
        lbl_mvhr = Text("WRG\nZentral", font_size=LABEL_FONT_SIZE, color=P_YELLOW).scale(0.65)
        lbl_mvhr.move_to(mvhr_box.get_center() + LEFT * 0.33)
        fan2 = _fan(mvhr_box.get_center() + RIGHT * 0.42, P_WHITE, radius=0.15)
        x_s, x_e = p2 - 0.35, p2 + 0.6
        ducts2 = VGroup(*[Line([x + dx, mvhr_box.get_bottom()[1], 0], [x + dx, y_c2, 0], color=P_WHITE, stroke_width=1.4,
                               stroke_opacity=0.7) for x in (x_s, x_e) for dx in (-0.09, 0.09)])
        fresh2 = [path((p2 - 1.95, 1.0), (p2 - 1.0, 0.95), (mvhr_box.get_left()[0] + 0.05, 0.62))]
        fort2 = [path((mvhr_box.get_right()[0] - 0.05, 0.62), (p2 + 1.5, 0.95), (p2 + 1.95, 1.05))]
        sup2 = [path((x_s, 0.33), (x_s, -0.3), (p2 - 0.6, -0.85), (p2 - 0.45, -1.3))]
        ext2 = [path((p2 + 0.45, -1.25), (x_e, -0.6), (x_e, 0.33))]
        lbl_supply = legend(["vorgewärmte", "Zuluft (16 °C)"], P_GREEN, 0.62, [p2 - 1.53, -0.75, 0])
        col2 = VGroup(card2, hdr2, room2["shell"], room2["glass"], person2, ducts2, mvhr_box, lbl_mvhr, fan2, lbl_supply,
                      box2_stat)

        p3 = 4.4
        bar3 = loss_bar(0.1, 0.3, P_GREEN)
        card3, hdr3, box3_stat, room3, person3 = panel(
            p3, P_GREEN, "#102018", "Dezentrale WRG", "Pendellüfter mit Keramik", r"\eta_{\mathrm{WRG}} \approx 0{,}7 \ldots 0{,}9", bar3)
        x_l3, gx3 = room3["x_l"], room3["glass_x"]
        y_u = 0.5 * (room3["win_hi"] + room3["y_c"])
        core = Rectangle(width=0.37, height=0.22, color=P_YELLOW, stroke_width=1.5, fill_color=P_ORANGE,
                         fill_opacity=0.9).move_to([gx3, y_u, 0])
        fan3 = _fan([x_l3 + 0.24, y_u, 0], P_WHITE, radius=0.11)
        ab3 = [path((p3 + 0.6, -0.95), (x_l3 + 0.45, y_u - 0.05), (gx3, y_u), (p3 - 1.4, y_u + 0.2), (p3 - 1.9, 0.12))]
        zu3 = [path((p3 - 1.9, -0.02), (p3 - 1.4, y_u - 0.12), (gx3, y_u), (x_l3 + 0.45, y_u - 0.1), (p3 + 0.6, -1.0))]
        lbl_cyc1 = legend(["70 s Abluft", "(Speichern)"], P_ORANGE, 0.65, [p3 - 1.35, 0.45, 0])
        lbl_cyc2 = legend(["70 s Zuluft", "(16 °C)"], P_GREEN, 0.65, [p3 - 1.35, -0.75, 0])
        col3 = VGroup(card3, hdr3, room3["shell"], room3["glass"], person3, core, fan3, lbl_cyc1, lbl_cyc2, box3_stat)

        paths = VGroup(*in1, *out1, *fresh2, *fort2, *sup2, *ext2, *ab3, *zu3)
        fills = VGroup(*[b[k] for b in (bar1, bar2, bar3) for k in ("upper", "lower")])
        place(VGroup(col1, col2, col3), paths, fills)
        win1["hinge"] = pivot + 0.85 * (win1["hinge"] - pivot) + UP * 0.35
        guides1 = VGroup(flow_guides(in1, P_BLUE, opacity=0.2), flow_guides(out1, P_ORANGE, opacity=0.2))
        guides2 = VGroup(flow_guides(fresh2, P_BLUE, opacity=0.2), flow_guides(fort2, GREY_B, opacity=0.2),
                         flow_guides(sup2, P_GREEN, opacity=0.2), flow_guides(ext2, P_ORANGE, opacity=0.2))
        guides3 = VGroup(flow_guides(ab3, P_ORANGE, opacity=0.2, tips=False))
        active = []
        pendulum = {"out": True}

        def window_air(rt):
            return [_streams([(in1, P_BLUE, P_ORANGE), (out1, P_ORANGE)], rt, speed=0.5, waves=4)]

        def central_air(rt):
            return [_streams([(fresh2, P_BLUE), (fort2, GREY_B), (sup2, P_GREEN), (ext2, P_ORANGE)], rt,
                             speed=0.55, waves=3), *_spin([fan2], rt)]

        def pendulum_air(rt):
            n = max(1, int(round(rt / 2.2)))
            flows, turns = [], []
            for _ in range(n):
                out = pendulum["out"]
                flow = _streams([(ab3, P_ORANGE)] if out else [(zu3, P_BLUE, P_GREEN)], rt / n, speed=0.55, waves=4)
                flow.run_time = rt / n
                turn = _spin([fan3], rt / n, sign=-1.0 if out else 1.0)[0]
                turn.run_time = rt / n
                flows.append(flow)
                turns.append(turn)
                pendulum["out"] = not out
            return [Succession(*flows), Succession(*turns)]

        def systems(rt):
            return [a for fn in active for a in fn(rt)]

        def grow(bar):
            return [GrowFromEdge(bar["upper"], LEFT), GrowFromEdge(bar["lower"], LEFT)]

        def reveal(col, bar, *extra, rt):
            self.play(FadeIn(col), *extra, *systems(rt), run_time=rt)
            self.play(*grow(bar), *systems(1.0), run_time=1.0)

        reveal(col1, bar1, rt=1.0)
        self.play(open_window(win1, run_time=0.8), FadeIn(guides1), run_time=0.8)
        active.append(window_air)
        self.play(*systems(1.0), run_time=1.0)
        hold_for(self, N, "window", used=4.6, during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "central"))
        reveal(col2, bar2, FadeIn(guides2), rt=1.3)
        active.append(central_air)
        hold_for(self, N, "central", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "decentral"))
        reveal(col3, bar3, FadeIn(guides3), rt=1.3)
        active.append(pendulum_air)
        hold_for(self, N, "decentral", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "outro"))
        hold_for(self, N, "outro", during=systems)

        caption = swap_caption(self, caption, subtitle_text(N, "passivhaus"))
        verdict = note_line("Passivhaus: Zu-/Abluft mit WRG — Fensterlüftung entfällt als Hauptweg", color=P_ORANGE)
        mark = cross_mark(size=0.2).move_to(win1["hinge"])
        ring_bad = SurroundingRectangle(col1, color=P_RED, buff=0.06, corner_radius=0.15, stroke_width=2.5)
        ring_good = SurroundingRectangle(VGroup(col2, col3), color=P_GREEN, buff=0.06, corner_radius=0.15, stroke_width=2.5)
        self.play(FadeIn(verdict, shift=DOWN * 0.1), Create(ring_bad), Create(ring_good), Create(mark), *systems(1.2), run_time=1.2)
        hold_for(self, N, "passivhaus", during=systems)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Duct cross-section from the hygiene airflow
class Beat4_Kanalquerschnitt(Scene):
    NARRATION = [
        ("intro",
         "The system has to deliver the hygiene airflow of zero point zero six one cubic metres per second. Which duct cross-section does that take?",
         "Die Anlage muss den Hygieneluftstrom von 0,061 m³/s fördern —\nwelchen Kanalquerschnitt braucht sie dafür?"),
        ("continuity",
         "Continuity links the three quantities: the volume flow is the mean velocity times the cross-sectional area.",
         "Die Kontinuität verknüpft sie: Der Volumenstrom ist die mittlere\nGeschwindigkeit mal die Querschnittsfläche."),
        ("vm",
         "To keep the duct quiet, the mean velocity is limited to about two point five metres per second. This small duct is far too fast.",
         "Für einen leisen Kanal wird die Geschwindigkeit auf etwa 2,5 m/s\nbegrenzt — der kleine Kanal hier ist viel zu schnell."),
        ("area",
         "A larger area slows the same flow down. At two point five metres per second the area is the flow divided by the velocity: about 0.0244 square metres.",
         "Mehr Fläche bremst denselben Strom: bei 2,5 m/s ist A gleich\nVolumenstrom durch Geschwindigkeit — rund 0,0244 m²."),
        ("radius",
         "For a round duct the radius is the square root of the area over pi: about nine centimetres, a diameter of roughly eighteen centimetres.",
         "Beim runden Kanal ist r die Wurzel aus A durch π: rund 9 cm,\nalso ein Durchmesser von etwa 18 cm."),
        ("cooling",
         "Cooling will need several times more air for its load. The method stays the same, only the duct gets bigger.",
         "Die Kühlung braucht für ihre Last ein Vielfaches an Luft —\ndieselbe Methode, nur ein größerer Kanal."),
        ("handover",
         "Heating takes over eta for its ventilation loss, and cooling takes the airflow for its duct sizing.",
         "Die Heizung übernimmt η für ihren Lüftungsverlust, die Kühlung\nden Luftstrom für ihre Kanalauslegung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Vom Luftstrom zum Kanalquerschnitt", title)
        din = _din_ref("VDI 2087")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        units_per_m, r_start = 12.0, 0.62
        r_final = float(np.sqrt(Q_VR / V_M / np.pi)) * units_per_m
        cy, disc_x, tube_x0 = 0.55, -2.6, -6.5
        disc_c = np.array([disc_x, cy, 0.0])
        r = ValueTracker(r_start)
        shown = ValueTracker(0.0)

        def area():
            return np.pi * (r.get_value() / units_per_m) ** 2

        def vel():
            return Q_VR / area()

        def tube():
            rr = r.get_value()
            return VGroup(
                Rectangle(width=disc_x - rr - tube_x0, height=2 * rr, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.12)
                .move_to(np.array([(disc_x - rr + tube_x0) / 2, cy, 0.0])),
                *[Line(np.array([tube_x0, cy + s * rr, 0.0]), np.array([disc_x, cy + s * rr, 0.0]), color=P_TEAL,
                       stroke_width=4) for s in (-1, 1)],
            )

        def disc():
            return Circle(radius=r.get_value(), color=P_TEAL, stroke_width=4, fill_color=P_BLUE, fill_opacity=0.3).move_to(disc_c)

        duct_wall = always_redraw(tube)
        duct_disc = always_redraw(disc)
        a_tag = always_redraw(lambda: math_label("A", disc_c, size=FORMULA_FONT_SIZE, color=P_BLUE))

        def lanes(rr):
            return [smooth_path([np.array([tube_x0 + 0.15, cy + k * rr, 0.0]), np.array([(tube_x0 + disc_x) / 2, cy + k * rr, 0.0]),
                                 np.array([disc_x - 1.1, cy + k * rr, 0.0])]) for k in (-0.62, -0.2, 0.2, 0.62)]

        def duct_air(rt, speed=None):
            pace = (0.18 + 0.12 * min(vel() / V_M, 4.0)) if speed is None else speed
            return [flow_animation([(lanes(r.get_value()), P_CYAN)], waves=3, radius=0.06, cycles=pace * rt)]

        rx = 1.25

        def line_at(y):
            return np.array([rx, y, 0.0])

        q_read = math_readout(lambda: rf"q_{{v,\mathrm{{R}}}} = {de_num(Q_VR * shown.get_value(), 3)}\,\mathrm{{m^{{3}}/s}}",
                              line_at(1.95), size=BODY_FONT_SIZE, color=P_YELLOW)
        v_read = math_readout(lambda: rf"v_{{m}} = {de_num(vel(), 1)}\,\mathrm{{m/s}}", line_at(1.25),
                              size=BODY_FONT_SIZE, color=P_CYAN)
        a_read = math_readout(lambda: rf"A = {de_num(area(), 4)}\,\mathrm{{m^{{2}}}}", line_at(0.55),
                              size=BODY_FONT_SIZE, color=P_BLUE)
        r_read = math_readout(lambda: rf"r = {de_num(r.get_value() / units_per_m, 3)}\,\mathrm{{m}}", line_at(-0.15),
                              size=BODY_FONT_SIZE, color=P_YELLOW)
        d_read = math_readout(lambda: rf"d = 2\,r = {de_num(200 * r.get_value() / units_per_m)}\,\mathrm{{cm}}",
                              line_at(-0.85), size=BODY_FONT_SIZE, color=P_WHITE)

        v_gauge = meter("m/s", length=2.4, thickness=0.3, color=P_GREEN, vertical=False)
        v_gauge["group"] = VGroup(v_gauge["track"], v_gauge["fill"])
        v_gauge["group"].move_to(np.array([5.45, 1.25, 0.0]))
        v_scale = 8.0
        bind_meter(v_gauge, _live_value(lambda: min(vel() / v_scale, 1.0)))
        v_gauge["fill"].add_updater(lambda m: m.set_fill(P_RED if vel() > V_M * 1.001 else P_GREEN))
        limit_x = v_gauge["track"].get_left()[0] + 0.04 + (V_M / v_scale) * v_gauge["span"]
        limit = VGroup(
            Line(np.array([limit_x, 1.25 - 0.26, 0.0]), np.array([limit_x, 1.25 + 0.26, 0.0]), color=P_YELLOW, stroke_width=3),
            math_label(r"2{,}5", np.array([limit_x, 1.25 - 0.48, 0.0]), size=LABEL_FONT_SIZE, color=P_YELLOW),
        )

        def cont_panel():
            return math_panel([
                ("qvr", r"q_{v,\mathrm{R}}", P_YELLOW), (None, "=", P_WHITE),
                ("vm", r"v_{m}", P_CYAN), (None, r"\cdot", P_WHITE),
                ("A", "A", P_BLUE),
            ], color=P_YELLOW)

        def area_panel():
            return math_panel([
                ("A", "A", P_BLUE), (None, "=", P_WHITE),
                ("frac", rf"\frac{{\textcolor{{{P_YELLOW}}}{{q_{{v,\mathrm{{R}}}}}}}}{{\textcolor{{{P_CYAN}}}{{v_{{m}}}}}}", P_WHITE),
                (None, "=", P_WHITE),
                ("val", r"\frac{0{,}061\,\mathrm{m^{3}/s}}{2{,}5\,\mathrm{m/s}} \approx 0{,}0244\,\mathrm{m^{2}}", P_BLUE),
            ], color=P_BLUE)

        def radius_panel():
            return math_panel([
                ("r", "r", P_YELLOW), (None, "=", P_WHITE),
                ("root", rf"\sqrt{{\frac{{\textcolor{{{P_BLUE}}}{{A}}}}{{\pi}}}}", P_WHITE),
                (None, "=", P_WHITE),
                ("val", r"\sqrt{\frac{0{,}0244\,\mathrm{m^{2}}}{\pi}} \approx 0{,}088\,\mathrm{m}", P_YELLOW),
            ], color=P_YELLOW)

        self.add(duct_wall, duct_disc, a_tag)
        self.add(q_read)
        self.play(shown.animate(rate_func=smooth).set_value(1.0), *duct_air(2.6, 0.5), run_time=2.6)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.6, during=duct_air)

        caption = swap_caption(self, caption, subtitle_text(N, "continuity"))
        cp, cp_box, cp_items = cont_panel()
        self.play(FadeIn(cp), Create(cp_box), *duct_air(1.0), run_time=1.0)
        rings = [highlight_param(cp_items, key, color=color) for key, color in
                 (("qvr", P_YELLOW), ("vm", P_CYAN), ("A", P_BLUE))]
        self.add(v_read, a_read)
        self.play(*[Create(g) for g in rings], *duct_air(1.0), run_time=1.0)
        hold_for(self, N, "continuity", during=duct_air)
        self.play(*[FadeOut(g) for g in rings], *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "vm"))
        self.play(FadeIn(v_gauge["group"]), FadeIn(limit), *duct_air(1.2), run_time=1.2)
        ring_v = SurroundingRectangle(v_read, color=P_RED, buff=0.1, corner_radius=0.05, stroke_width=3)
        self.play(Create(ring_v), *duct_air(1.2), run_time=1.2)
        hold_for(self, N, "vm", during=duct_air)
        self.play(FadeOut(ring_v), *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "area"))
        ap, ap_box, ap_items = area_panel()
        self.play(ReplacementTransform(cp, ap), ReplacementTransform(cp_box, ap_box), *duct_air(1.2), run_time=1.2)
        self.play(r.animate(rate_func=smooth).set_value(r_final), *duct_air(3.4), run_time=3.4)
        ring_a = SurroundingRectangle(a_read, color=P_BLUE, buff=0.1, corner_radius=0.05, stroke_width=3)
        ring_ok = SurroundingRectangle(v_read, color=P_GREEN, buff=0.1, corner_radius=0.05, stroke_width=3)
        self.play(Create(ring_a), Create(ring_ok), *duct_air(1.0), run_time=1.0)
        hold_for(self, N, "area", during=duct_air)
        self.play(FadeOut(ring_a), FadeOut(ring_ok), *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "radius"))
        rp, rp_box, rp_items = radius_panel()
        radius_line = always_redraw(lambda: Line(disc_c, disc_c + RIGHT * r.get_value(), color=P_YELLOW, stroke_width=4))
        radius_dot = Dot(disc_c, radius=0.06, color=P_YELLOW)
        r_tag = always_redraw(lambda: math_label("r", disc_c + RIGHT * r.get_value() / 2 + UP * 0.2, size=BODY_FONT_SIZE,
                                                 color=P_YELLOW))
        self.remove(a_tag)
        self.add(r_read, d_read)
        self.play(ReplacementTransform(ap, rp), ReplacementTransform(ap_box, rp_box), FadeIn(radius_dot),
                  Create(radius_line), FadeIn(r_tag), *duct_air(1.6), run_time=1.6)
        ring_root = highlight_param(rp_items, "root", color=P_BLUE)
        self.play(Create(ring_root), Indicate(duct_disc, color=P_BLUE), *duct_air(1.0), run_time=1.0)
        hold_for(self, N, "radius", during=duct_air)
        self.play(FadeOut(ring_root), *duct_air(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "cooling"))
        r_big = 1.6 * r_final
        ghost = VGroup(
            DashedVMobject(Circle(radius=r_big, color=P_CYAN, stroke_width=3).move_to(disc_c), num_dashes=40),
            *[DashedLine(np.array([tube_x0, cy + s * r_big, 0.0]), np.array([disc_x, cy + s * r_big, 0.0]), color=P_CYAN,
                         stroke_width=3) for s in (-1, 1)],
        )
        ghost_tag = Text("Kühlung: mehr Luft, größerer Kanal", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        ghost_tag.move_to(np.array([-2.6, cy - r_big - 0.3, 0.0]))
        self.play(FadeOut(rp), FadeOut(rp_box), Create(ghost), FadeIn(ghost_tag), *duct_air(1.8), run_time=1.8)
        hold_for(self, N, "cooling", during=duct_air)

        caption = swap_caption(self, caption, subtitle_text(N, "handover"))
        heat_chip = _math_chip(r"\text{→ Heizung · Modul 3:}\;\eta_{\mathrm{WRG}}\;\to\;\Phi_{\mathrm{V}}", P_ORANGE)
        cool_chip = _math_chip(r"\text{→ Kühlung · Teil 3:}\;q_{v,\mathrm{R}}\;\to\;A,\,r", P_CYAN)
        chips = VGroup(heat_chip, cool_chip).arrange(RIGHT, buff=0.5).move_to(np.array([0.9, -1.95, 0.0]))
        self.play(FadeIn(heat_chip, shift=UP * 0.1), FadeIn(cool_chip, shift=UP * 0.1), *duct_air(1.2), run_time=1.2)
        hold_for(self, N, "handover", during=duct_air)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
