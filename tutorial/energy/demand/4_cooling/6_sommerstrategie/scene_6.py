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
    beat_subtitle, BEAT_SUBTITLE_FADE, body_text,
    BODY_FONT_SIZE, LABEL_FONT_SIZE,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    fit_band,
    smooth_path, flow_guides,
    meter, bind_meter, chip, cross_mark, dim_chip,
    math_label,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    house_section, person_glyph, sun_glyph, ripples, flow_animation, droplets,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Sommerstrategie"

AIR = "#60A5FA"


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner.

    Exact size, colour, opacity and corner of ``_din_ref`` in the Heating
    series (``3_heating/2_conduction/scene_2.py``): a dim ``P_TEAL`` footnote
    that never competes with the diagram. The formula panel sits on the
    bottom edge, so this corner is clear in every beat.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Physical Fundamentals occupant — the comfort target of the strategy."""
    return person_glyph(pos, color=color, scale=1.09 * scale)


def _streams(streams, rt: float, *, speed: float = 0.85, waves: int = 3):
    """💨 Continuous particle streams for ``rt`` seconds at a steady ``speed`` in passes per second."""
    return flow_animation([(list(p), *c) for p, *c in streams], waves=waves, cycles=speed * rt)


def _through(win, *, inward: bool = False):
    """➡️ Particle track through one opening of the section house — from the room out, or reversed from outside in."""
    x, y = win["x"], win["center"][1]
    pts = [(x + 1.15, y - 0.12), (x + 0.35, y - 0.04), (x - 0.35, y + 0.02), (x - 1.15, y + 0.16)]
    if inward:
        pts = pts[::-1]
    return smooth_path([np.array([px, py, 0.0]) for px, py in pts])


def _park(node, buff=0.28):
    """🅿️ Point just outside a node's left edge — where a travelling token waits without covering the label."""
    return node.get_left() + LEFT * buff


def _back_chip(prefix: str, src: str | None = None):
    """🔗 Back-link chip in the Lüftung blue — names the chapter of the Lüftung video the tool is explained in."""
    parts = [body_text(prefix, font_size=LABEL_FONT_SIZE + 1, color=AIR)]
    if src:
        parts.append(math_label(src, size=LABEL_FONT_SIZE + 1, color=AIR))
    content = VGroup(*parts).arrange(RIGHT, buff=0.12)
    box = SurroundingRectangle(content, color=AIR, corner_radius=0.1, buff=0.1, stroke_width=1.8)
    return VGroup(box, content)

#endregion


#region Beat1 – Load first: shrink cooling before moving air
class Beat1_LastZuerstSenken(Scene):
    NARRATION = [
        ("intro",
         "Summer comfort starts before any fan runs: first reduce the load, then ventilate.",
         "Sommerkomfort beginnt, bevor ein Ventilator läuft:\nerst die Last senken, dann lüften."),
        ("envelope",
         "Start from the cooling load. A heavily insulated, airtight envelope with external shading removes the largest share before any air is moved at all.",
         "Start bei der Kühllast: eine gedämmte, dichte Hülle mit\naußenliegendem Sonnenschutz nimmt den größten Anteil weg."),
        ("natural",
         "What remains is ventilated with the tools of the ventilation video: window, cross and stack ventilation and the night purge from chapter 4.",
         "Den Rest lüften die Werkzeuge aus dem Lüftungsvideo: Fenster-,\nQuer-, Auftriebs- und Nachtlüftung, Kapitel 4."),
        ("reserve",
         "Only the remainder goes to supply and exhaust with heat recovery from chapter 5. An open window recovers nothing, so the Passivhaus does not use it as the main air path.",
         "Nur der Rest geht an Zu-/Abluft mit WRG (Kapitel 5): ein offenes\nFenster gewinnt nichts zurück, im Passivhaus ist es nicht der Hauptweg."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Erst die Last senken, dann lüften", title)
        din = _din_ref("DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        hs = house_section(np.array([-3.85, -0.05, 0.0]), scale=0.8)
        bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
        lo_win, up_win = hs["windows"]
        person = _person(np.array([-3.25, bl[1] + 0.02 + 0.66 * 1.09 * 0.8 / 2, 0.0]), color=P_ORANGE, scale=0.8)

        off = 0.09
        envelope = VMobject(color=P_TEAL, stroke_width=6, stroke_opacity=0.85).set_points_as_corners([
            bl + LEFT * off, tl + LEFT * off, tl + LEFT * (0.26 * 0.8 + off) + UP * 0.02, peak + UP * off,
            tr + RIGHT * (0.26 * 0.8 + off) + UP * 0.02, tr + RIGHT * off, br + RIGHT * off,
        ])
        louvre = VGroup(*[
            Line(np.array([w["x"] - 0.40, y - 0.07, 0.0]), np.array([w["x"] - 0.14, y + 0.07, 0.0]),
                 color=P_TEAL, stroke_width=3)
            for w in (lo_win, up_win)
            for y in np.linspace(w["sill"] + 0.07, w["lintel"] - 0.07, 4)
        ])
        warm_spots = [person.get_top() + UP * 0.05, np.array([-2.75, hs["level_1"].get_center()[1] + 0.02, 0.0])]

        def heat(rt, r_max=0.45):
            return [ripples(warm_spots, r_max=r_max, cycles=rt / 1.4)]

        load_bar = meter("Kühllast", length=4.5, thickness=0.5, color=P_RED, vertical=False)
        load_bar["group"].move_to(np.array([2.35, 1.62, 0.0]))
        load = ValueTracker(1.0)
        bind_meter(load_bar, load)

        def _step(idx, name, share, color):
            badge = Circle(radius=0.17, color=color, stroke_width=2, fill_color=color, fill_opacity=1.0)
            num = Text(idx, font_size=LABEL_FONT_SIZE, color=P_DEEP_DARK).move_to(badge.get_center())
            return VGroup(
                VGroup(badge, num),
                Text(name, font_size=BODY_FONT_SIZE, color=P_WHITE),
                Text(share, font_size=BODY_FONT_SIZE, color=color),
            ).arrange(RIGHT, buff=0.20)

        steps = VGroup(
            _step("1", "Hülle + Sonnenschutz", "Last sinkt", P_TEAL),
            _step("2", "Fenster · Quer · Nacht", "freie Kühlung", P_CYAN),
            _step("3", "Zu-/Abluft mit WRG", "Hauptweg", P_ORANGE),
        ).arrange(DOWN, aligned_edge=LEFT, buff=0.34)
        steps.move_to(np.array([2.55, -0.30, 0.0]))

        chip_nat = _back_chip("← Lüftung · Kapitel 4: Fenster · Quer · Auftrieb · Nacht")
        chip_mech = _back_chip("← Lüftung · Kapitel 5: Zu-/Abluft · WRG")
        chip_nat.move_to(np.array([2.55, -1.8, 0.0]))
        chip_mech.move_to(chip_nat)

        self.play(Create(hs["group"]), run_time=1.5)
        self.play(FadeIn(person), *heat(0.8), run_time=0.8)
        self.play(FadeIn(load_bar["group"]), *heat(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 0.8 + 1.0,
                 during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "envelope"))
        self.play(Create(envelope), Create(louvre), *heat(1.2), run_time=1.2)
        self.play(
            load.animate.set_value(0.55),
            load_bar["fill"].animate.set_fill(P_YELLOW),
            FadeIn(steps[0], shift=RIGHT * 0.2), *heat(1.7, 0.32),
            run_time=1.7,
        )
        hold_for(self, self.NARRATION, "envelope", during=lambda rt: heat(rt, 0.3))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "natural"))
        cool_paths = VGroup(*[_through(w, inward=True) for w in (lo_win, up_win)])
        cool_guides = flow_guides(cool_paths, P_CYAN)

        def night(rt):
            return [_streams([(cool_paths, P_CYAN, P_BLUE)], rt, speed=1.05), *heat(rt, 0.28)]

        self.play(FadeIn(steps[1], shift=RIGHT * 0.2), FadeIn(chip_nat), Create(cool_guides),
                  load.animate.set_value(0.4), load_bar["fill"].animate.set_fill(P_CYAN), run_time=0.8)
        self.play(*night(1.8), run_time=1.8)
        hold_for(self, self.NARRATION, "natural", during=lambda rt: night(rt))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "reserve"))
        loss_paths = VGroup(*[_through(w) for w in (lo_win, up_win)])
        loss_guides = flow_guides(loss_paths, P_RED)

        def loss(rt):
            return [_streams([(loss_paths, P_RED, P_ORANGE)], rt, speed=1.05), *heat(rt, 0.28)]

        self.play(FadeOut(cool_guides), Create(loss_guides), run_time=0.6)
        self.play(*loss(1.6), run_time=1.6)
        crosses = VGroup(*[cross_mark(size=0.18).move_to(w["center"]) for w in (lo_win, up_win)])
        self.play(FadeIn(crosses), loss_guides.animate.set_stroke(opacity=0.12), run_time=0.6)
        self.play(
            load.animate.set_value(0.25),
            load_bar["fill"].animate.set_fill(P_ORANGE),
            FadeIn(steps[2], shift=RIGHT * 0.2), ReplacementTransform(chip_nat, chip_mech), *heat(1.4, 0.18),
            run_time=1.4,
        )
        self.play(Indicate(steps[2], color=P_ORANGE), *heat(1.0, 0.16), run_time=1.0)
        hold_for(self, self.NARRATION, "reserve", during=lambda rt: heat(rt, 0.16))
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Sorption-assisted cooling: engineering with material physics
class Beat2_SorptionsKuehlung(Scene):
    """🌞 Sorptionsgestützte Kühlung (DEC): Entfeuchten → Wärmetausch →
    Verdunsten, solar regeneriert — und ihr mikrobieller Preis.

    https://de.wikipedia.org/wiki/Sorptionsgestützte_Klimatisierung
    """

    NARRATION = [
        ("intro",
         "Material physics itself can cool without a chiller: sorption-assisted cooling chains drying, heat exchange and evaporation into one engine.",
         "Mit Stoffeigenschaften lässt sich ohne Kältemaschine kühlen: die sorptions-\ngestützte Kühlung verkettet Trocknen, Wärmetausch und Verdunstung."),
        ("entfeuchten",
         "A desiccant wheel first dries the humid outdoor air — adsorption releases heat, so the dry air leaves warmer than it came.",
         "Das Sorptionsrad entfeuchtet zuerst die schwüle Außenluft —\ndie Sorptionswärme macht die trockene Luft zunächst wärmer."),
        ("tauscher",
         "The plate exchanger hands that heat to the exhaust stream and precools the dry air.",
         "Der Plattentauscher gibt diese Wärme an die Abluft ab\nund kühlt die trockene Luft vor."),
        ("verdunsten",
         "Because the air is now dry, water may evaporate into it: evaporative cooling takes it down to supply temperature.",
         "Weil die Luft jetzt trocken ist, darf Wasser verdunsten:\nVerdunstungskühlung senkt sie auf Zulufttemperatur."),
        ("regeneration",
         "Solar heat regenerates the wheel: it drives the stored moisture out into the exhaust air, so the sun itself powers the cycle.",
         "Solarwärme regeneriert das Rad: sie treibt die gespeicherte\nFeuchte in die Fortluft aus — die Sonne treibt den Kreislauf."),
        ("nachteil",
         "The drawback is microbial. Harsh swings of temperature and humidity put the germs under pressure. They mutate, and the harmful ones are the ones that survive.",
         "Nachteil: krasse Temperatur- und Feuchtewechsel setzen Keime unter Druck.\nSie mutieren — es überleben bevorzugt die schädlichen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Sorptionsgestützte Kühlung (DEC)", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        y_sup, y_exh = 1.05, -0.25

        wheel_c = np.array([-2.9, 0.4, 0.0])
        wheel = Circle(radius=0.95, color=P_ORANGE, stroke_width=3).move_to(wheel_c)
        wheel_spokes = VGroup(*[
            Line(wheel_c, wheel_c + 0.95 * np.array([np.cos(a), np.sin(a), 0.0]),
                 color=P_ORANGE, stroke_width=1.4).set_stroke(opacity=0.5)
            for a in np.linspace(0, TAU, 8, endpoint=False)
        ])
        wheel_tag = Text("Sorptionsrad", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        wheel_tag.move_to(np.array([-2.9, -1.05, 0.0]))

        hx = Rectangle(width=1.5, height=1.9, color=P_WHITE, stroke_width=3).move_to(np.array([-0.3, 0.4, 0.0]))
        hx_diag = VGroup(
            Line(hx.get_corner(DL), hx.get_corner(UR), color=P_TEAL, stroke_width=1.6, stroke_opacity=0.7),
            Line(hx.get_corner(UL), hx.get_corner(DR), color=P_TEAL, stroke_width=1.6, stroke_opacity=0.7),
        )
        hx_tag = Text("Plattentauscher", font_size=LABEL_FONT_SIZE, color=P_TEAL)
        hx_tag.move_to(np.array([-0.45, -1.05, 0.0]))

        def _humidifier(pos, color=P_BLUE):
            box = Rectangle(width=0.72, height=0.72, color=color, stroke_width=2.4).move_to(pos)
            return VGroup(box, droplets(pos, n=5, spread=(0.2, 0.2), seed=5, color=color))

        hum_sup = _humidifier(np.array([2.0, y_sup, 0.0]))
        hum_exh = _humidifier(np.array([2.0, y_exh, 0.0]))
        hum_tag = Text("Verdunstungskühlung", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        hum_tag.move_to(np.array([2.25, -1.05, 0.0]))

        heater_c = np.array([-1.5, y_exh, 0.0])
        heater = Rectangle(width=0.72, height=0.6, color=P_RED, stroke_width=2.4).move_to(heater_c)
        coil_pts = [heater_c + np.array([-0.26 + i * 0.087, 0.12 if i % 2 else -0.12, 0.0]) for i in range(7)]
        coil = VMobject(color=P_RED, stroke_width=2.2).set_points_as_corners(coil_pts)

        # Below the regeneration heater: the solar duct stays on the exhaust
        # side and never crosses the supply-air state readouts above.
        sun_c = np.array([-1.5, -1.5, 0.0])
        sun = sun_glyph(sun_c).scale(0.34)
        solar_duct = DashedLine(
            sun_c + UP * 0.36, heater.get_bottom() + DOWN * 0.04,
            color=P_YELLOW, stroke_width=2.4, dash_length=0.08,
        )
        solar_tag = Text("Solarwärme", font_size=LABEL_FONT_SIZE, color=P_YELLOW)
        solar_tag.next_to(sun, RIGHT, buff=0.18).shift(DOWN * 0.12)

        # Zustände entlang der Zuluft — jeder Wert ist das Ergebnis des Bauteils davor.
        t_reads = VGroup(
            math_label(r"32\,\mathrm{°C}\;\text{feucht}", at=np.array([-4.1, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_ORANGE),
            math_label(r"48\,\mathrm{°C}\;\text{trocken}", at=np.array([-1.5, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_RED),
            math_label(r"24\,\mathrm{°C}", at=np.array([0.95, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_YELLOW),
            math_label(r"19\,\mathrm{°C}", at=np.array([3.6, y_sup + 0.55, 0.0]),
                       size=LABEL_FONT_SIZE, color=P_CYAN),
        )
        end_tags = VGroup(
            Text("Außenluft", font_size=LABEL_FONT_SIZE, color=P_ORANGE).move_to([-5.9, y_sup + 0.55, 0.0]),
            Text("Zuluft", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to([5.0, y_sup + 0.55, 0.0]),
            Text("Abluft 26 °C", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to([5.0, y_exh - 0.62, 0.0]),
            Text("Fortluft\nfeucht · warm", font_size=LABEL_FONT_SIZE, color=P_ORANGE, line_spacing=0.8).move_to([-5.55, y_exh - 0.62, 0.0]),
        )

        def _lane(x0, x1, y):
            return VGroup(smooth_path([np.array([x0, y, 0.0]), np.array([(x0 + x1) / 2, y, 0.0]),
                                       np.array([x1, y, 0.0])]))

        sup_seg1 = _lane(-5.3, -2.0, y_sup)
        sup_seg2 = _lane(-2.0, 1.45, y_sup)
        sup_seg3 = _lane(1.45, 4.9, y_sup)
        exh_seg1 = _lane(4.9, 0.5, y_exh)
        exh_seg2 = _lane(0.5, -2.1, y_exh)
        exh_seg3 = _lane(-2.1, -5.3, y_exh)

        components = VGroup(
            wheel, wheel_spokes, wheel_tag, hx, hx_diag, hx_tag,
            hum_sup, hum_exh, hum_tag, heater, coil,
        )

        self.play(
            Create(wheel), Create(wheel_spokes), FadeIn(wheel_tag),
            Create(hx), Create(hx_diag), FadeIn(hx_tag),
            FadeIn(hum_sup), FadeIn(hum_exh), FadeIn(hum_tag),
            FadeIn(heater), Create(coil),
            Create(flow_guides(VGroup(*sup_seg1, *sup_seg2, *sup_seg3), P_CYAN, opacity=0.2)),
            Create(flow_guides(VGroup(*exh_seg1, *exh_seg2, *exh_seg3), P_TEAL, opacity=0.2)),
            FadeIn(end_tags[0]), FadeIn(end_tags[2]),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 2.6)

        def _spin(run_time):
            return Rotate(wheel_spokes, angle=-TAU * run_time / 5.2, about_point=wheel_c,
                          run_time=run_time, rate_func=linear)

        running = []

        def dec(rt):
            return [_streams(running, rt, speed=0.75, waves=5), _spin(rt)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "entfeuchten"))
        self.play(FadeIn(t_reads[0]), run_time=0.4)
        running.append((sup_seg1, P_ORANGE, P_RED))
        self.play(*dec(2.6), run_time=2.6)
        self.play(FadeIn(t_reads[1]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "entfeuchten", during=dec)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "tauscher"))
        heat_down = VGroup(*[
            Arrow(np.array([x, y_sup - 0.25, 0.0]), np.array([x, y_exh + 0.25, 0.0]),
                  buff=0, color=P_YELLOW, stroke_width=3, max_tip_length_to_length_ratio=0.3)
            for x in (-0.8, -0.3, 0.2)
        ])
        running.append((sup_seg2, P_RED, P_YELLOW))
        self.play(LaggedStart(*[GrowArrow(a) for a in heat_down], lag_ratio=0.3), *dec(2.4), run_time=2.4)
        self.play(FadeIn(t_reads[2]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "tauscher", during=dec)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verdunsten"))
        running.append((sup_seg3, P_YELLOW, P_CYAN))
        self.play(Indicate(hum_sup, color=P_BLUE, scale_factor=1.12), *dec(2.4), run_time=2.4)
        self.play(FadeIn(t_reads[3]), FadeIn(end_tags[1]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "verdunsten", during=dec)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "regeneration"))
        self.play(FadeIn(sun, scale=0.7), FadeIn(solar_tag), Create(solar_duct), *dec(1.0), run_time=1.0)
        running.extend([(exh_seg1, P_TEAL, P_BLUE), (exh_seg2, P_BLUE, P_ORANGE), (exh_seg3, P_ORANGE, P_RED)])
        self.play(
            heater.animate.set_fill(P_RED, opacity=0.25),
            Indicate(sun, color=P_YELLOW, scale_factor=1.1), *dec(3.0),
            run_time=3.0,
        )
        self.play(FadeIn(end_tags[3]), *dec(0.5), run_time=0.5)
        hold_for(self, self.NARRATION, "regeneration", during=dec)

        # 🦠 Mikrobieller Preis: Hitze-/Feuchtezyklen selektieren resistente Keime.
        def _microbe(pos, color):
            c = np.array([pos[0], pos[1], 0.0], dtype=float)
            body = Circle(radius=0.11, color=color, stroke_width=2,
                          fill_color=color, fill_opacity=0.25).move_to(c)
            spikes = VGroup(*[
                Line(c + 0.11 * d, c + 0.18 * d, color=color, stroke_width=1.8)
                for a in np.linspace(0, TAU, 6, endpoint=False)
                for d in [np.array([np.cos(a), np.sin(a), 0.0])]
            ])
            return VGroup(body, spikes)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nachteil"))
        # Compact strip between the component labels (bottom ≈ −1.2) and the
        # caption box (top ≈ −2.6): every microbe and the verdict stay inside it.
        strip_y = -1.86
        panel = RoundedRectangle(
            width=10.0, height=1.0, corner_radius=0.12,
            color=P_RED, stroke_width=2.2, fill_color="#12151C", fill_opacity=0.94,
        ).move_to(np.array([0.0, strip_y, 0.0]))
        normal = VGroup(*[
            _microbe([-3.3 + i * 0.55, strip_y - 0.08], P_TEAL) for i in range(4)
        ])
        survivors = (normal[2], normal[3])
        cycle_tag = math_label(r"ΔT\!\uparrow\;\;Δx\!\uparrow", at=np.array([-4.75, strip_y - 0.08, 0.0]),
                               size=LABEL_FONT_SIZE, color=P_ORANGE, edge="left")
        verdict = Text("Mutation:\ndie Schädlichen überleben",
                       font_size=LABEL_FONT_SIZE, color=P_RED, line_spacing=0.8)
        verdict.move_to(np.array([2.9, strip_y, 0.0]))
        self.play(FadeOut(VGroup(sun, solar_tag, solar_duct)), *dec(0.4), run_time=0.4)
        self.play(FadeIn(panel), FadeIn(normal), FadeIn(cycle_tag), *dec(0.9), run_time=0.9)
        for flash in (P_RED, P_CYAN):
            self.play(panel.animate.set_stroke(color=flash), *dec(0.35), run_time=0.35)
        self.play(
            *[m.animate.scale(1.2).set_color(P_RED) for m in normal],
            *dec(1.0),
            run_time=1.0,
        )
        self.play(
            LaggedStart(*[FadeOut(m, scale=0.4) for m in normal[:2]], lag_ratio=0.15),
            *[m[0].animate.set_fill(opacity=0.65) for m in survivors], *dec(1.0),
            run_time=1.0,
        )
        clones = VGroup(_microbe([0.1, strip_y - 0.08], P_RED), _microbe([0.65, strip_y - 0.08], P_RED))
        self.play(FadeIn(clones, scale=0.5), FadeIn(verdict), *dec(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "nachteil", during=dec)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – The operating rule: natural first, machines last
class Beat3_KomfortStrategie(Scene):
    NARRATION = [
        ("intro",
         "Put the levers together and they become one operating rule you can follow every day.",
         "Zusammen ergeben die Stellgrößen eine Betriebsregel, der man täglich folgen kann."),
        ("question",
         "Every decision starts with the same question: is the outdoor air cooler than the room?",
         "Am Anfang steht immer dieselbe Frage: ist die Außenluft kühler als der Raum?"),
        ("yes",
         "If it is, ventilate naturally as in chapter 4 of the ventilation video. Set the opening, pick the cross or the stack path, and let outdoor air carry the heat away for free.",
         "Wenn ja: natürlich lüften (Lüftung Kapitel 4) — Öffnung einstellen,\nQuer- oder Auftriebsweg wählen, die Außenluft trägt die Wärme ab."),
        ("no",
         "If it is not, close and shade, and let the thermal mass buffer the day until the night window opens again.",
         "Wenn nein: schließen und verschatten — die Speichermasse\npuffert, bis das Nachtfenster wieder aufgeht."),
        ("reserve",
         "Only what these steps cannot cover goes to the balanced supply and exhaust system with heat recovery from chapter 5: the small remainder from the very first beat.",
         "Nur was diese Schritte nicht schaffen, geht an die Zu-/Abluftanlage\nmit Wärmerückgewinnung (Lüftung Kapitel 5)."),
        ("close",
         "Comfort first, outdoor air second, machines last.",
         "Komfort zuerst, Außenluft danach, Technik zuletzt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Strategie: freie Kühlung zuerst", title)
        din = _din_ref("DIN EN 16798-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        start = chip("Sommertag", P_WHITE).move_to(np.array([0.0, 2.28, 0.0]))

        q_text = Text("Außenluft kühler\nals der Raum?", font_size=LABEL_FONT_SIZE,
                      color=P_YELLOW, line_spacing=0.7)
        diamond = Polygon(
            np.array([0.0, 0.66, 0.0]), np.array([2.6, 0.0, 0.0]),
            np.array([0.0, -0.66, 0.0]), np.array([-2.6, 0.0, 0.0]),
            color=P_YELLOW, stroke_width=2.2,
        )
        q_text.move_to(diamond.get_center())
        question = VGroup(diamond, q_text).move_to(np.array([0.0, 1.22, 0.0]))

        yes_box = chip("Natürlich lüften", P_CYAN).move_to(np.array([-3.45, 0.02, 0.0]))
        yes_sub = chip("Öffnung · Luftweg · Zeitplan", P_TEAL, font_size=LABEL_FONT_SIZE)
        yes_sub.move_to(np.array([-3.45, -0.78, 0.0]))
        no_box = chip("Schließen + verschatten", P_ORANGE).move_to(np.array([3.45, 0.02, 0.0]))
        no_sub = chip("Speichermasse puffert", P_TEAL, font_size=LABEL_FONT_SIZE)
        no_sub.move_to(np.array([3.45, -0.78, 0.0]))

        mech = chip("Reserve: Zu-/Abluft mit WRG", P_ORANGE).move_to(np.array([0.0, -1.60, 0.0]))
        goal = chip("Komfort", P_GREEN).move_to(np.array([0.0, -2.46, 0.0]))

        def _link(a, b, color=P_WHITE):
            return Arrow(a, b, buff=0.10, color=color, stroke_width=3,
                         max_tip_length_to_length_ratio=0.10)

        links = VGroup(
            _link(start.get_bottom(), question.get_top()),
            _link(diamond.get_left(), yes_box.get_top(), P_CYAN),
            _link(diamond.get_right(), no_box.get_top(), P_ORANGE),
            _link(yes_box.get_bottom(), yes_sub.get_top(), P_CYAN),
            _link(no_box.get_bottom(), no_sub.get_top(), P_ORANGE),
            _link(yes_sub.get_bottom(), mech.get_left(), P_TEAL),
            _link(no_sub.get_bottom(), mech.get_right(), P_TEAL),
            _link(mech.get_bottom(), goal.get_top(), P_GREEN),
        )
        # Inside the elbow of each link, away from the diamond corner where the token parks.
        yes_lbl = Text("ja", font_size=LABEL_FONT_SIZE, color=P_CYAN).next_to(links[1].get_center(), RIGHT, buff=0.14)
        no_lbl = Text("nein", font_size=LABEL_FONT_SIZE, color=P_ORANGE).next_to(links[2].get_center(), LEFT, buff=0.14)

        chart = VGroup(start, question, yes_box, yes_sub, no_box, no_sub,
                       mech, goal, links, yes_lbl, no_lbl)
        # Inner band: clear of the beat subtitle above and the two-line caption box below.
        fit_band(chart, top=2.35, bottom=-2.42)

        token = Dot(radius=0.11, color=P_YELLOW, fill_opacity=1.0).move_to(_park(start))
        chip_nat = _back_chip("← Lüftung · Kapitel 4").move_to(np.array([-4.9, -1.6, 0.0]))
        chip_mech = _back_chip("← Lüftung · Kapitel 5").move_to(np.array([4.9, -1.6, 0.0]))

        self.play(FadeIn(start), Create(question), run_time=1.2)
        self.play(
            LaggedStart(
                *[Create(link) for link in links],
                FadeIn(yes_box), FadeIn(no_box), FadeIn(yes_sub), FadeIn(no_sub),
                FadeIn(mech), FadeIn(goal), FadeIn(yes_lbl), FadeIn(no_lbl),
                lag_ratio=0.10,
            ),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 2.6)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "question"))
        self.play(FadeIn(token), run_time=0.4)
        self.play(token.animate.move_to(_park(question)), Indicate(question, color=P_YELLOW), run_time=1.1)
        hold_for(self, self.NARRATION, "question", used=0.4 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "yes"))
        self.play(
            token.animate.set_color(P_CYAN).move_to(_park(yes_box)),
            Indicate(yes_box, color=P_CYAN),
            *dim_chip(no_box, 0.3), *dim_chip(no_sub, 0.3),
            FadeIn(chip_nat),
            run_time=1.2,
        )
        self.play(token.animate.move_to(_park(yes_sub)), Indicate(yes_sub, color=P_TEAL), run_time=1.1)
        hold_for(self, self.NARRATION, "yes", used=1.2 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "no"))
        # Un-dim before Indicate, never alongside it: Indicate snapshots the starting
        # state and restores it, which silently undoes a same-play opacity change.
        self.play(
            *dim_chip(no_box, 1.0), *dim_chip(no_sub, 1.0),
            token.animate.set_color(P_ORANGE).move_to(_park(no_box)),
            run_time=0.9,
        )
        self.play(Indicate(no_box, color=P_ORANGE), run_time=0.7)
        self.play(token.animate.move_to(_park(no_sub)), Indicate(no_sub, color=P_TEAL), run_time=1.0)
        hold_for(self, self.NARRATION, "no", used=0.9 + 0.7 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "reserve"))
        self.play(token.animate.move_to(_park(mech)), Indicate(mech, color=P_ORANGE), FadeIn(chip_mech), run_time=1.2)
        hold_for(self, self.NARRATION, "reserve", used=1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "close"))
        self.play(
            token.animate.set_color(P_GREEN).move_to(_park(goal)),
            Indicate(goal, color=P_GREEN),
            run_time=1.2,
        )
        self.play(FadeOut(token), run_time=0.3)
        hold_for(self, self.NARRATION, "close", used=1.2 + 0.3 + 0.35)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
