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
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    symbol_token, watt_anchor,
    equation_row, formula_panel, highlight_param,
    math_label, math_readout, math_panel, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    house_section, room_section, sun_glyph, sun_rays, shine, radiation_ray, ripples, pulse_flashes,
)

# 🗣️ VO reads the German subtitles; measured clause durations live in vo_timing.json.
set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Solare Einstrahlung"

# Mid-screen anchor for facade / charts / sections.
CONTENT_CENTER = UP * 0.25


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

def _sun(pos):
    """🌞 Physical Fundamentals sun at the size every Cooling part uses."""
    return sun_glyph(np.array(pos, dtype=float)).scale(0.42)


def _parallel_rays(sun_c, hits, gap: float = 0.5):
    """☀️ Straight parallel rays from the sun onto ``hits``, each starting ``gap`` clear of the sun's centre."""
    sun_c = np.array(sun_c, dtype=float)
    hits = [np.array(h, dtype=float) for h in hits]
    aim = np.mean(hits, axis=0) - sun_c
    aim /= np.linalg.norm(aim)
    starts = [h - aim * (float(np.dot(h - sun_c, aim)) - gap) for h in hits]
    return aim, starts


def _house_sun_rays(house, sun_c):
    """🔆 Two parallel rays through each window of ``house``, landing on that storey's floor."""
    floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
    sets = [sun_rays(sun_c, w["x"], [w["center"][1] + 0.09, w["center"][1] - 0.09], fy, gap=0.5)
            for w, fy in zip(house["windows"], floors)]
    paths = [[s, h, p] for r in sets for s, h, p in zip(r["starts"], r["hits"], r["lands"])]
    return sets, paths


def _build_window(center=ORIGIN, width=3.6, height=2.7, band=0.3, mullion=0.16, opening_pad=0.16):
    """🪟 Face-on window: dashed rough opening, opaque frame bands, transparent panes.

    Frame bands are fully opaque so incoming rays placed at a lower ``z_index`` are
    visually cut off wherever the frame blocks them.
    """
    cx, cy = float(center[0]), float(center[1])
    x0, x1 = cx - width / 2, cx + width / 2
    y0, y1 = cy - height / 2, cy + height / 2
    inner_h = height - 2 * band
    pane_w = (width - 2 * band - mullion) / 2

    opening = DashedVMobject(
        Rectangle(
            width=width + 2 * opening_pad, height=height + 2 * opening_pad,
            color=P_BLUE, stroke_width=3,
        ).move_to(center),
        num_dashes=44,
    )

    def _band(w, h, pos):
        return Rectangle(
            width=w, height=h, color=P_WHITE, stroke_width=2,
            fill_color=P_WHITE, fill_opacity=1.0,
        ).move_to(pos)

    frame = VGroup(
        _band(width, band, np.array([cx, y1 - band / 2, 0.0])),
        _band(width, band, np.array([cx, y0 + band / 2, 0.0])),
        _band(band, inner_h, np.array([x0 + band / 2, cy, 0.0])),
        _band(band, inner_h, np.array([x1 - band / 2, cy, 0.0])),
        _band(mullion, inner_h, np.array([cx, cy, 0.0])),
    )

    def _pane(pos):
        return Rectangle(
            width=pane_w, height=inner_h, color=P_CYAN, stroke_width=2,
            fill_color=P_CYAN, fill_opacity=0.12,
        ).move_to(pos)

    panes = VGroup(
        _pane(np.array([cx - mullion / 2 - pane_w / 2, cy, 0.0])),
        _pane(np.array([cx + mullion / 2 + pane_w / 2, cy, 0.0])),
    )

    return {
        "opening": opening,
        "frame": frame,
        "panes": panes,
        "center": np.array([cx, cy, 0.0]),
        "x0": x0, "x1": x1, "y0": y0, "y1": y1,
        "band": band, "mullion": mullion,
        "width": width, "height": height,
    }


def _measured_frame_factor(width, height, band=0.3, mullion=0.16, pad=0.16) -> float:
    """🪟 Glass area over the rough opening — the frame factor of this drawn window."""
    glass = (width - 2 * band - mullion) * (height - 2 * band)
    opening = (width + 2 * pad) * (height + 2 * pad)
    return glass / opening


def _section_hatch(rect, spacing=0.2, color=P_WHITE, stroke_width=1.0, opacity=0.35):
    """〽️ 45° hatch clipped to a rectangle — the architectural 'cut through' convention."""
    x0, x1 = float(rect.get_left()[0]), float(rect.get_right()[0])
    y0, y1 = float(rect.get_bottom()[1]), float(rect.get_top()[1])
    lines = VGroup()
    c = x0 - y1
    while c <= x1 - y0:
        xs, xe = max(x0, c + y0), min(x1, c + y1)
        if xe - xs > 1e-3:
            lines.add(Line(
                np.array([xs, xs - c, 0.0]), np.array([xe, xe - c, 0.0]),
                color=color, stroke_width=stroke_width, stroke_opacity=opacity,
            ))
        c += spacing
    return lines


def _bell(x, x0, x1, amp):
    """📈 Half-sine daily irradiance profile between sunrise x0 and sunset x1."""
    if x <= x0 or x >= x1:
        return 0.0
    return amp * np.sin(PI * (x - x0) / (x1 - x0))


#region 3D sun path
# One solar model drives both halves of Beat 1: the sun's 3D position over the
# cube house AND the orientation power curves on the chart — the curves are
# literally drawn by the moving sun, never hand-tuned bells.
def _cav_proj(x, y, z):
    """📐 Cavalier projection — south face true, north depth recedes up-right."""
    return np.array([x + 0.42 * y, z + 0.26 * y, 0.0])


def _sun_angles(h: float):
    """🌞 Solar elevation α and azimuth φ (0 = Süd, −Ost/+West) for hour ``h``."""
    t = np.clip((h - 6.0) / 12.0, 0.0, 1.0)
    alpha = np.radians(62.0) * np.sin(np.pi * t)
    phi = np.radians((h - 12.0) / 6.0 * 90.0)
    return alpha, phi


def _facade_direct(name: str, h: float) -> float:
    """☀️ Direct-beam share (0–1) on one orientation at hour ``h``."""
    if h <= 6.0 or h >= 18.0:
        return 0.0
    a, p = _sun_angles(h)
    direct = {
        "Horiz": np.sin(a),
        "S": np.cos(a) * np.cos(p),
        "O": np.cos(a) * -np.sin(p),
        "W": np.cos(a) * np.sin(p),
        "N": 0.0,
    }[name]
    return max(0.0, float(direct))


def _sun_atten(h: float) -> float:
    """🌫️ Air-mass attenuation of the direct beam — kills the sunrise spike a
    pure cosine model would paint onto the east facade."""
    a, _ = _sun_angles(h)
    return float(np.exp(-0.18 / max(np.sin(a), 0.02)))


def _facade_irradiance(name: str, h: float) -> float:
    """📈 I in W/m² on one orientation: attenuated direct beam plus diffuse share."""
    if h <= 6.0 or h >= 18.0:
        return 0.0
    a, _ = _sun_angles(h)
    return 950.0 * _sun_atten(h) * _facade_direct(name, h) + 110.0 * float(np.sin(a))
#endregion

#endregion


#region Beat1 – Maximum Solar Irradiance (I_S,max)
class Beat1_SolarIrradiance(Scene):
    NARRATION = [
        ("intro",
         "Now we tackle the most significant summer heat source: direct solar radiation.",
         "Die größte sommerliche Wärmequelle: direkte Sonnenstrahlung."),
        ("irradiance",
         "Everything starts with the maximum solar irradiance, I S max, measured in watts per square meter.",
         "Alles beginnt mit I S max — der maximalen Bestrahlungsstärke in Watt pro Quadratmeter."),
        ("chart",
         "Watch the sun travel from east over south to west above the house: each facade lights up in turn and draws its own power curve — east in the morning, south at noon, west in the afternoon, north stays low.",
         "Die Sonne wandert von Ost über Süd nach West und zeichnet die Kurven:\nOst am Morgen, Süd am Mittag, West am Nachmittag, Nord bleibt niedrig."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Direkte Sonnenstrahlung", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        house = house_section(np.array([-3.75, -0.3, 0.0]), scale=0.78)
        building = house["group"]
        sun_pos = np.array([-6.45, 0.75, 0.0])
        sun = _sun(sun_pos)
        ray_sets, ray_paths = _house_sun_rays(house, sun_pos)
        light_beam = VGroup(*[r["group"] for r in ray_sets])

        def sunshine(rt):
            return [pulse_flashes(ray_paths, P_YELLOW, repeats=max(1, int(rt / 1.6)))]

        irr_anchor = watt_anchor(800, compare="vacuum", title="Bestrahlungsstärke je m²")
        irr_anchor.scale(0.6).move_to(np.array([-0.55, -0.1, 0.0]))

        axes = Axes(
            x_range=[6, 18, 3],
            y_range=[0, 900, 300],
            x_length=5.0,
            y_length=3.0,
            axis_config={"color": P_WHITE, "stroke_width": 2},
            tips=False,
        ).move_to(RIGHT * 3.7 + CONTENT_CENTER + UP * 0.2)

        x_labels = VGroup(*[
            Text(str(h), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(h, 0), DOWN, buff=0.14)
            for h in (6, 9, 12, 15, 18)
        ])
        y_labels = VGroup(*[
            Text(str(v), font_size=LABEL_FONT_SIZE, color=P_WHITE).next_to(axes.c2p(6, v), LEFT, buff=0.14)
            for v in (300, 600, 900)
        ])
        y_axis_name = math_label(r"\mathrm{W/m^{2}}", size=BODY_FONT_SIZE, color=P_TEAL)
        y_axis_name.next_to(axes.c2p(6, 900), UP, buff=0.16).shift(LEFT * 0.15)
        x_axis_name = Text("Sonnenzeit", font_size=BODY_FONT_SIZE, color=P_TEAL)
        x_axis_name.next_to(axes.c2p(12, 0), DOWN, buff=0.36)

        # ── 3D-Sonnenbahn über dem Würfelhaus (Kavalierprojektion) ──
        cube_c = np.array([-4.2, -1.3, 0.0])
        cs, ch = 0.95, 1.15

        def CV(x, y, z):
            return cube_c + _cav_proj(x, y, z)

        a0, b0, c0, d0 = CV(-cs, -cs, 0), CV(cs, -cs, 0), CV(cs, cs, 0), CV(-cs, cs, 0)
        a1, b1, c1, d1 = CV(-cs, -cs, ch), CV(cs, -cs, ch), CV(cs, cs, ch), CV(-cs, cs, ch)
        faces = {
            "S": Polygon(a0, b0, b1, a1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "O": Polygon(b0, c0, c1, b1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "W": Polygon(d0, a0, a1, d1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "N": Polygon(c0, d0, d1, c1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
            "Horiz": Polygon(a1, b1, c1, d1, stroke_width=0, fill_color=P_YELLOW, fill_opacity=0.0),
        }
        solid_edges = VGroup(*[
            Line(p, q, color=P_WHITE, stroke_width=2.4)
            for p, q in ((a0, b0), (b0, c0), (a0, a1), (b0, b1), (c0, c1),
                         (a1, b1), (b1, c1), (c1, d1), (d1, a1))
        ])
        hidden_edges = VGroup(*[
            DashedLine(p, q, color=P_WHITE, stroke_width=1.4, dash_length=0.1, stroke_opacity=0.45)
            for p, q in ((c0, d0), (d0, a0), (d0, d1))
        ])
        # Only the two visible facades carry a tag; W and N face away from the
        # viewer and are named by the compass instead.
        face_tags = VGroup(
            Text("S", font_size=BODY_FONT_SIZE, color=P_CYAN).move_to((a0 + b0) / 2 + RIGHT * 0.45 + UP * 0.24),
            Text("O", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to((b0 + c0 + c1 + b1) / 4),
        )
        north_dir = _cav_proj(0, 1, 0)
        north_dir = north_dir / np.linalg.norm(north_dir)
        compass_base = np.array([-6.55, 1.05, 0.0])
        compass = VGroup(
            Arrow(compass_base, compass_base + north_dir * 0.75, buff=0,
                  color=P_TEAL, stroke_width=2.5, max_tip_length_to_length_ratio=0.22),
            Text("N", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(compass_base + north_dir * 1.0),
        )
        cube = VGroup(*faces.values(), solid_edges, hidden_edges, face_tags)

        def sun_screen(h):
            a, p = _sun_angles(h)
            r = 2.5
            pos3 = (r * np.cos(a) * -np.sin(p), r * np.cos(a) * -np.cos(p), r * np.sin(a))
            return cube_c + _cav_proj(*pos3)

        sun_path = DashedVMobject(
            VMobject(color=P_YELLOW, stroke_width=1.6, stroke_opacity=0.5).set_points_smoothly(
                [sun_screen(h) for h in np.linspace(6.0, 18.0, 25)]
            ),
            num_dashes=40,
        )
        # Tags sit clear of the sun disc even when the sun parks on an end point.
        path_tags = VGroup(
            Text("Ost", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(sun_screen(6.0), DOWN, buff=0.48),
            Text("Süd", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(sun_screen(12.0), UP, buff=0.48),
            Text("West", font_size=LABEL_FONT_SIZE, color=P_YELLOW).next_to(sun_screen(18.0), DOWN, buff=0.48),
        )

        h_tr = ValueTracker(6.0)
        for name, face in faces.items():
            face.add_updater(lambda m, name=name: m.set_fill(
                P_YELLOW,
                opacity=0.75 * _sun_atten(h_tr.get_value()) * _facade_direct(name, h_tr.get_value()),
            ))

        colors = {"Horiz": P_YELLOW, "O": P_ORANGE, "S": P_CYAN, "W": P_GREEN, "N": P_TEAL}
        live_curves = VGroup(*[
            always_redraw(lambda n=n: axes.plot(
                lambda x: _facade_irradiance(n, x),
                x_range=[6.0, max(6.06, min(18.0, h_tr.get_value()))],
                color=colors[n], stroke_width=2.8,
            ))
            for n in colors
        ])
        time_cursor = always_redraw(lambda: DashedLine(
            axes.c2p(np.clip(h_tr.get_value(), 6.0, 18.0), 0),
            axes.c2p(np.clip(h_tr.get_value(), 6.0, 18.0), 900),
            color=P_WHITE, stroke_width=1.2, dash_length=0.08, stroke_opacity=0.35,
        ))
        label_spots = {"Horiz": (12, 860), "O": (6.6, 650), "S": (12, 540), "W": (17.4, 650), "N": (9.0, 210)}
        curve_labels = VGroup(*[
            Text(n, font_size=BODY_FONT_SIZE, color=colors[n]).move_to(axes.c2p(*label_spots[n]))
            for n in colors
        ])

        south_peak = int(round(max(float(_facade_irradiance("S", h)) for h in np.linspace(6.0, 18.0, 90))))
        peak_tr = ValueTracker(0.0)
        eq_row, eq_box, eq_items = math_panel([
            ("i", r"I_{S,max}", P_YELLOW), (None, "=", P_WHITE),
            (None, rf"{de_num(south_peak)}", P_YELLOW), (None, r"\;[\mathrm{W/m^{2}}]", P_TEAL),
        ], size=BODY_FONT_SIZE, color=P_YELLOW)

        self.play(Create(building), run_time=1.6)
        self.play(FadeIn(sun, scale=0.7), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.6 + 1.0)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "irradiance"))
        self.play(*[shine(r) for r in ray_sets], run_time=1.4)
        self.play(FadeIn(irr_anchor, shift=DOWN * 0.1), *sunshine(0.9), run_time=0.9)
        hold_for(self, self.NARRATION, "irradiance", during=sunshine)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "chart"))
        self.play(Create(axes), run_time=1.0)
        self.play(
            LaggedStart(*[FadeIn(m) for m in (*x_labels, *y_labels, y_axis_name, x_axis_name)], lag_ratio=0.06),
            run_time=0.9,
        )
        # Facade morphs into the cube house; the sun shrinks onto its 3D path.
        self.play(FadeOut(light_beam), FadeOut(irr_anchor), run_time=0.5)
        self.play(
            ReplacementTransform(building, cube), Create(sun_path),
            sun.animate.scale(0.55).move_to(sun_screen(6.0)),
            run_time=1.5,
        )
        self.play(FadeIn(path_tags), FadeIn(compass), run_time=0.5)
        sun.add_updater(lambda m: m.move_to(sun_screen(h_tr.get_value())))
        i_read = math_readout(
            lambda: rf"I_{{S,max}} = {de_num(peak_tr.get_value())}",
            np.array([2.15, 2.28, 0.0]),
            size=LABEL_FONT_SIZE, color=P_CYAN, edge="left",
        )

        def _raise_peak(_mob):
            peak_tr.set_value(max(peak_tr.get_value(), _facade_irradiance("S", h_tr.get_value())))

        i_read.add_updater(_raise_peak)
        self.add(live_curves, time_cursor, i_read)
        self.play(h_tr.animate.set_value(18.0), run_time=8.0, rate_func=linear)
        i_read.remove_updater(_raise_peak)
        peak_tr.set_value(south_peak)
        sun.clear_updaters()
        for face in faces.values():
            face.clear_updaters()

        static_curves = VGroup(*[
            axes.plot(lambda x, n=n: _facade_irradiance(n, x), x_range=[6.0, 18.0],
                      color=colors[n], stroke_width=2.8)
            for n in colors
        ])
        self.remove(live_curves, time_cursor, i_read)
        self.add(static_curves)
        self.play(
            LaggedStart(*[FadeIn(lbl) for lbl in curve_labels], lag_ratio=0.12),
            FadeOut(path_tags),
            run_time=0.9,
        )

        eq_rest = VGroup(*[m for m in eq_row.submobjects if m is not eq_items["i"]])
        self.play(Create(eq_box), FadeIn(eq_rest), run_time=1.0)
        self.play(ReplacementTransform(sun.copy(), eq_items["i"]), run_time=1.4)

        ring = highlight_param(eq_items, "i", color=P_YELLOW)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "chart", used=16.75 + 0.5 + 0.35)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion



#region Beat2 – Gross Area & Frame Factor (A · F_F)
class Beat2_FrameFactor(Scene):
    NARRATION = [
        ("intro",
         "To calculate the cooling load, we start with the gross area of the window opening, A.",
         "Die Kühllast beginnt mit der Rohbauöffnung A."),
        ("frame",
         "However, glass doesn't cover the entire opening. We must multiply by F F, the dimensionless frame factor.",
         "Glas füllt die Öffnung nicht ganz — multipliziert mit dem Rahmenfaktor F F."),
        ("aeff",
         "This mathematically isolates the effective transparent area by subtracting the opaque window frames that physically block the sun.",
         "So bleibt die transparente Restfläche A eff — ohne den undurchsichtigen Rahmen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Fensterfläche und Rahmenfaktor", title)
        din = _din_ref("DIN EN ISO 52016-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        w = _build_window(center=LEFT * 3.4 + CONTENT_CENTER, width=3.6, height=2.5)
        w["panes"].set_z_index(2)
        w["frame"].set_z_index(3)

        area_label = Text("A: Rohbauöffnung", font_size=BODY_FONT_SIZE, color=P_BLUE)
        area_label.next_to(w["opening"], DOWN, buff=0.18)

        sun_pos = np.array([-6.35, 1.95, 0.0])
        sun = _sun(sun_pos)
        frame_hits = [
            np.array([w["x0"] + 0.15, 0.55, 0.0]),
            np.array([-3.4, 0.65, 0.0]),
            np.array([-4.2, w["y1"] - 0.15, 0.0]),
            np.array([-2.6, w["y0"] + 0.15, 0.0]),
        ]
        glass_hits = [
            np.array([-4.5, 0.75, 0.0]),
            np.array([-4.0, 0.05, 0.0]),
            np.array([-4.6, -0.65, 0.0]),
            np.array([-2.9, 0.6, 0.0]),
            np.array([-2.3, -0.1, 0.0]),
            np.array([-2.6, 0.92, 0.0]),
        ]

        direction, starts = _parallel_rays(sun_pos, frame_hits + glass_hits)
        frame_starts, glass_starts = starts[:len(frame_hits)], starts[len(frame_hits):]
        frame_rays = VGroup(*[radiation_ray(s, h).set_z_index(1) for s, h in zip(frame_starts, frame_hits)])
        glass_rays = VGroup(*[radiation_ray(s, h).set_z_index(1) for s, h in zip(glass_starts, glass_hits)])
        through_rays = VGroup(*[
            radiation_ray(h, h + direction * 1.35, stroke_width=2.0).set_stroke(opacity=0.6).set_z_index(4)
            for h in glass_hits
        ])
        glass_paths = [[s, h, h + direction * 1.35] for s, h in zip(glass_starts, glass_hits)]

        def through_glass(rt):
            return [pulse_flashes(glass_paths, P_YELLOW, repeats=max(1, int(rt / 1.8)))]

        eq_row, panel_box, eq_items = math_panel([
            ("aeff", r"A_{eff}", P_CYAN), (None, "=", P_WHITE), ("a", "A", P_BLUE),
            (None, r"\cdot", P_WHITE), ("ff", r"F_{F}", P_WHITE),
            (None, r"\;[\mathrm{m^{2}}]", P_TEAL),
        ])

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(w["opening"]), FadeIn(area_label, shift=UP * 0.15), run_time=1.4)
        self.play(w["opening"].animate.set_stroke(color=P_BLUE, width=5), rate_func=there_and_back, run_time=0.9)

        self.play(FadeIn(w["frame"], scale=0.9), run_time=1.3)
        self.play(FadeIn(w["panes"]), run_time=0.7)

        self.play(FadeOut(area_label), Create(panel_box), FadeIn(eq_row), run_time=1.4)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "frame"))

        self.play(FadeIn(sun, scale=0.7), run_time=0.6)
        self.play(
            LaggedStart(*[Create(r, rate_func=linear) for r in (*glass_rays, *frame_rays)], lag_ratio=0.08),
            run_time=1.8,
        )
        self.play(
            LaggedStart(*[Create(r) for r in through_rays], lag_ratio=0.08),
            frame_rays.animate.set_stroke(color="#5A6472", opacity=0.35),
            run_time=1.6,
        )

        ring = highlight_param(eq_items, "ff", color=P_ORANGE)
        ff_note = VGroup(
            Text("Rahmenfaktor", font_size=LABEL_FONT_SIZE, color=P_ORANGE),
            math_label(r"A_{\mathrm{Glas}}/A", size=LABEL_FONT_SIZE, color=P_WHITE),
        ).arrange(DOWN, buff=0.12, aligned_edge=LEFT)
        ff_note.move_to(RIGHT * 2.2 + CONTENT_CENTER + UP * 0.3)
        # 🔢 F_F is the drawn glass area divided by the drawn rough opening.
        ff_target = _measured_frame_factor(w["width"], w["height"])
        ff_tr = ValueTracker(1.0)
        ff_read = math_readout(
            lambda: rf"F_{{F}} = {de_num(ff_tr.get_value(), 2)}",
            np.array([-0.55, 1.35, 0.0]), size=BODY_FONT_SIZE, color=P_ORANGE, edge="center",
        )
        self.add(ff_read)
        self.play(
            Create(ring),
            eq_items["ff"].animate.set_color(P_ORANGE),
            w["frame"].animate.set_fill(color=P_ORANGE, opacity=1.0).set_stroke(color=P_ORANGE),
            FadeIn(ff_note, shift=UP * 0.1),
            ff_tr.animate.set_value(ff_target),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "frame", during=through_glass)
        self.play(
            FadeOut(ring),
            w["frame"].animate.set_fill(color=P_WHITE, opacity=1.0).set_stroke(color=P_WHITE),
            run_time=0.3,
        )

        ring = highlight_param(eq_items, "aeff", color=P_CYAN)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "aeff"))
        self.play(
            Create(ring),
            w["panes"].animate.set_fill(opacity=0.34),
            *through_glass(0.8),
            run_time=0.8,
        )
        hold_for(self, self.NARRATION, "aeff", during=through_glass)
        self.play(FadeOut(ring), FadeOut(caption), FadeOut(ff_note), FadeOut(ff_read), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Shading Factor (F_V)
class Beat3_ShadingFactor(Scene):
    """🪟 Vertical section: without shading the full beam reaches the glass; Raffstore reflects heat outside."""

    NARRATION = [
        ("intro",
         "Next, we deploy our sun protection. This is a vertical section through the facade.",
         "Als Nächstes der Sonnenschutz — Vertikalschnitt durch die Fassade."),
        ("ismax",
         "I S max is the maximum solar irradiance on the facade — the starting intensity in watts per square meter, before any shading.",
         "I S max ist die maximale Bestrahlungsstärke auf die Fassade — der Ausgangswert vor Verschattung."),
        ("unshaded",
         "With no shading, the full beam strikes the glass and passes straight into the room: a shading factor, F V, of one point zero.",
         "Ohne Schutz trifft die volle Strahlung auf die Scheibe: F V gleich eins Komma null."),
        ("raffstore",
         "Now an external Raffstore drops in front. Its slats intercept the beam and reflect the heat back outside, before it ever reaches the glass.",
         "Ein außenliegender Raffstore fängt den Strahl ab und reflektiert die Wärme."),
        ("reduced",
         "Only a small residual gets through, so F V falls to about zero point one five.",
         "Nur ein Restanteil kommt durch — F V sinkt auf etwa null Komma fünfzehn."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Verschattungsfaktor", title)
        din = _din_ref("DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        glass_out, glass_in = -3.5, -3.3
        wall_top, wall_bottom = 1.65, -0.95
        lintel = Rectangle(
            width=0.5, height=0.7, color=P_WHITE, stroke_width=2.5,
            fill_color="#161A21", fill_opacity=1.0,
        ).move_to(np.array([-3.4, wall_top + 0.35, 0.0]))
        sill = Rectangle(
            width=0.5, height=0.7, color=P_WHITE, stroke_width=2.5,
            fill_color="#161A21", fill_opacity=1.0,
        ).move_to(np.array([-3.4, wall_bottom - 0.35, 0.0]))
        masonry = VGroup(
            lintel, sill,
            _section_hatch(lintel), _section_hatch(sill),
        ).set_z_index(3)

        glazing = VGroup(*[
            Line(np.array([x, wall_bottom, 0.0]), np.array([x, wall_top, 0.0]),
                 color=P_CYAN, stroke_width=3)
            for x in (glass_out, glass_in)
        ]).set_z_index(3)

        slab_left, room_end_x = lintel.get_right()[0], -0.7
        ceiling_y, floor_y = lintel.get_top()[1], sill.get_bottom()[1]

        ceiling = Line(np.array([slab_left, ceiling_y, 0.0]), np.array([room_end_x, ceiling_y, 0.0]),
                       color=P_WHITE, stroke_width=2.5)
        floor = Line(np.array([slab_left, floor_y, 0.0]), np.array([room_end_x, floor_y, 0.0]),
                     color=P_TEAL, stroke_width=4)
        inner_wall = Line(np.array([room_end_x, floor_y, 0.0]), np.array([room_end_x, ceiling_y, 0.0]),
                          color=P_WHITE, stroke_width=2.5)
        section = VGroup(masonry, glazing, ceiling, floor, inner_wall)

        lbl_out = Text("Außen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(
            np.array([-6.2, (wall_top + wall_bottom) / 2, 0.0]))
        lbl_in = Text("Innen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(np.array([-1.5, 1.85, 0.0]))

        d = np.array([0.75, -0.661, 0.0])
        sun = _sun(np.array([-6.3, 1.85, 0.0]))

        slat_x = -4.4
        slat_ys = [1.45 - i * 0.35 for i in range(8)]
        aimed_ys = slat_ys[:6]
        glass_ys = [y - 0.793 for y in aimed_ys]

        def _ray(end, back=3.0, **kwargs):
            return Line(end - d * back, end, color=P_YELLOW, **kwargs)

        direct = VGroup(*[
            _ray(np.array([glass_out, y, 0.0]), stroke_width=2.6, stroke_opacity=0.9).set_z_index(1)
            for y in glass_ys
        ])
        blocked = VGroup(*[
            _ray(np.array([glass_out, y, 0.0]), back=3.0 - 1.316,
                 stroke_width=2.6, stroke_opacity=0.9).shift(-d * 1.316).set_z_index(1)
            for y in glass_ys
        ])

        def _through_end(start):
            t_floor = (floor_y - start[1]) / d[1]
            t_wall = (room_end_x - start[0]) / d[0]
            t = min(t for t in (t_floor, t_wall) if t > 0.05)
            return start + d * t

        interior = VGroup(*[
            Line(
                np.array([glass_in, y, 0.0]),
                _through_end(np.array([glass_in, y, 0.0])),
                color=P_YELLOW, stroke_width=2.6, stroke_opacity=0.85,
            ).set_z_index(1)
            for y in glass_ys[:5]
        ])

        u = np.array([np.cos(48 * DEGREES), np.sin(48 * DEGREES), 0.0])
        n = np.array([-u[1], u[0], 0.0])
        r = d - 2 * float(np.dot(d, n)) * n

        slats = VGroup(*[
            Rectangle(
                width=0.52, height=0.08, color=P_TEAL, stroke_width=1.5,
                fill_color=P_TEAL, fill_opacity=0.8,
            ).move_to(np.array([slat_x, y, 0.0])).rotate(48 * DEGREES)
            for y in slat_ys
        ])
        rail = Rectangle(
            width=0.62, height=0.26, color=P_TEAL, stroke_width=2,
            fill_color=P_TEAL, fill_opacity=0.9,
        ).move_to(np.array([slat_x, 1.85, 0.0]))
        bracket = Line(np.array([slat_x + 0.31, 1.85, 0.0]), np.array([-3.65, 1.85, 0.0]),
                       color=P_TEAL, stroke_width=2.5)
        blind = VGroup(rail, bracket, slats).set_z_index(4)
        lbl_blind = Text("Raffstore", font_size=LABEL_FONT_SIZE, color=P_TEAL).move_to(np.array([-4.95, -1.55, 0.0]))

        reflected = VGroup(*[
            Arrow(
                np.array([-4.487, y + 0.077, 0.0]),
                np.array([-4.487, y + 0.077, 0.0]) + r * 1.45,
                buff=0, color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.55,
                max_tip_length_to_length_ratio=0.16,
            ).set_z_index(5)
            for y in aimed_ys
        ])
        residual = VGroup(*[
            Line(
                np.array([-4.226, y + 0.193, 0.0]),
                _through_end(np.array([-4.226, y + 0.193, 0.0])),
                color=P_YELLOW, stroke_width=1.4, stroke_opacity=0.32,
            ).set_z_index(1)
            for y in aimed_ys[:5]
        ])
        direct_paths = [[ray.get_start(), ray.get_end(), seg.get_end()] for ray, seg in zip(direct, interior)]
        bounce_paths = [[b.get_start(), b.get_end(), a.get_end()] for b, a in zip(blocked, reflected)]
        rest_paths = [[r.get_start(), r.get_end()] for r in residual]

        def unshaded(rt):
            return [pulse_flashes(direct_paths, P_YELLOW, repeats=max(1, int(rt / 1.8)))]

        def shaded(rt):
            n = max(1, int(rt / 1.8))
            return [pulse_flashes(bounce_paths, P_YELLOW, repeats=n),
                    pulse_flashes(rest_paths, P_YELLOW, repeats=n, width=2.5)]

        # Upper right of the room: the residual rays all run below this corner.
        lbl_rest = Text("Restanteil", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(
            np.array([room_end_x - 0.75, 1.2, 0.0]))

        eq_row, eq_box, eq_items = math_panel([
            ("ired", r"I_{red}", P_TEAL), (None, "=", P_WHITE), ("i", r"I_{S,max}", P_YELLOW),
            (None, r"\cdot", P_WHITE), ("fv", r"F_{V}", P_TEAL),
            (None, r"\;[\mathrm{W/m^{2}}]", P_TEAL),
        ], color=P_TEAL)
        # The section reaches the panel band on the left, so this panel sits
        # right of it, under the F_V scale.
        VGroup(eq_row, eq_box).set_x(3.0)

        sx0, sx1, sy = 1.5, 6.0, 0.45
        scale_line = Line(np.array([sx0, sy, 0.0]), np.array([sx1, sy, 0.0]), color=P_WHITE, stroke_width=2)
        tick_l = Line(np.array([sx0, sy - 0.12, 0.0]), np.array([sx0, sy + 0.12, 0.0]), color=P_WHITE, stroke_width=2)
        tick_r = Line(np.array([sx1, sy - 0.12, 0.0]), np.array([sx1, sy + 0.12, 0.0]), color=P_WHITE, stroke_width=2)
        end_l = Text("0,1\naußenliegend", font_size=LABEL_FONT_SIZE, color=P_TEAL, line_spacing=0.8)
        end_l.next_to(tick_l, DOWN, buff=0.12)
        end_r = Text("1,0\nohne Schutz", font_size=LABEL_FONT_SIZE, color=P_YELLOW, line_spacing=0.8)
        end_r.next_to(tick_r, DOWN, buff=0.12)
        scale_title = math_label(r"\text{Bandbreite}\;F_{V}", size=BODY_FONT_SIZE, color=P_WHITE)
        scale_title.next_to(scale_line, UP, buff=0.85)

        def _sx(fv):
            return sx0 + (fv - 0.1) / 0.9 * (sx1 - sx0)

        fv_tr = ValueTracker(1.0)

        def _marker_at():
            return np.array([_sx(fv_tr.get_value()), sy + 0.24, 0.0])

        marker = Triangle(color=P_YELLOW, fill_color=P_YELLOW, fill_opacity=1.0, stroke_width=0)
        marker.scale(0.16).rotate(PI).move_to(_marker_at())

        def _pin_marker(mob):
            mob.move_to(_marker_at())
            mob.set_color(P_TEAL if fv_tr.get_value() < 0.5 else P_YELLOW)

        marker.add_updater(_pin_marker)
        fv_read = math_readout(
            lambda: rf"F_{{V}} = {de_num(fv_tr.get_value(), 2)}",
            lambda: _marker_at() + UP * 0.42,
            size=BODY_FONT_SIZE, color=P_TEAL, edge="center",
        )

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(section), run_time=1.8)
        self.play(
            LaggedStart(FadeIn(lbl_out), FadeIn(lbl_in), lag_ratio=0.2),
            run_time=1.0,
        )
        self.play(Create(eq_box), FadeIn(eq_row), run_time=1.2)
        ring_i = highlight_param(eq_items, "i", color=P_YELLOW)
        self.play(Create(ring_i), run_time=0.45)
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ismax"))
        hold_for(self, self.NARRATION, "ismax", used=1.2 + 0.45 + 0.35)
        self.play(FadeOut(ring_i), run_time=0.25)

        # Unshaded / Raffstore / reduced visuals belong to those captions —
        # swap first, then animate, so ``used=`` never backdates into the
        # previous clause (that caused overlapping VO in the muxed series).
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unshaded"))
        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(LaggedStart(*[Create(ray) for ray in direct], lag_ratio=0.08), run_time=1.6)
        self.play(LaggedStart(*[Create(ray) for ray in interior], lag_ratio=0.1), run_time=1.4)
        self.add(fv_read)
        self.play(
            Create(scale_line), Create(tick_l), Create(tick_r),
            FadeIn(end_l), FadeIn(end_r), FadeIn(scale_title),
            FadeIn(marker),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "unshaded", during=unshaded)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "raffstore"))
        blind.shift(UP * 3.0).set_opacity(0)
        self.add(blind)
        self.play(blind.animate.shift(DOWN * 3.0).set_opacity(1.0), run_time=1.5)
        self.play(FadeIn(lbl_blind, shift=UP * 0.15), run_time=0.5)
        self.play(
            *[Transform(ray, cut) for ray, cut in zip(direct, blocked)],
            LaggedStart(*[GrowArrow(a) for a in reflected], lag_ratio=0.08),
            FadeOut(interior),
            run_time=1.8,
        )
        hold_for(self, self.NARRATION, "raffstore", during=lambda rt: shaded(rt)[:1])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "reduced"))
        self.play(
            LaggedStart(*[Create(ray) for ray in residual], lag_ratio=0.1),
            FadeIn(lbl_rest),
            run_time=1.6,
        )
        self.play(fv_tr.animate.set_value(0.15), run_time=1.8)
        ring = highlight_param(eq_items, "fv", color=P_TEAL)
        self.play(Create(ring), run_time=0.7)
        self.play(Indicate(eq_items["ired"], color=P_TEAL, scale_factor=1.12), run_time=0.8)
        hold_for(self, self.NARRATION, "reduced", during=shaded)
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Total Solar Energy Transmittance (g_tot)
class Beat4_GlassTransmittance(Scene):
    NARRATION = [
        ("intro",
         "Finally, the remaining light hits the glass pane itself.",
         "Zuletzt trifft das Restlicht auf die Glasscheibe selbst."),
        ("gtot",
         "We multiply by g tot, the total solar energy transmittance.",
         "Wir multiplizieren mit g tot — dem Gesamtenergiedurchlassgrad."),
        ("parts",
         "Academically, this is the sum of direct solar transmission, tau e, and the secondary inward heat emission, q i, from the glass absorbing the radiation.",
         "Das ist die Summe aus direkter Transmission tau e und sekundärer Wärmeabgabe q i."),
        ("meaning",
         "It tells us exactly what fraction of that heat successfully penetrates into the room.",
         "Er sagt, welcher Anteil der Wärme tatsächlich in den Raum gelangt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Gesamtenergiedurchlassgrad", title)
        din = _din_ref("DIN EN 410")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        pane_x, pane_top, pane_bottom = -1.5, 2.15, -0.95
        pane_w = 0.54
        leaf_w = 0.11
        cav_w = pane_w - 2 * leaf_w
        pane_cy = (pane_top + pane_bottom) / 2
        pane_h = pane_top - pane_bottom

        def _leaf(cx):
            return Rectangle(
                width=leaf_w, height=pane_h, color=P_CYAN, stroke_width=2.5,
                fill_color=P_CYAN, fill_opacity=0.20,
            ).move_to(np.array([cx, pane_cy, 0.0]))

        outer_leaf = _leaf(pane_x - pane_w / 2 + leaf_w / 2)
        inner_leaf = _leaf(pane_x + pane_w / 2 - leaf_w / 2)
        leaves = VGroup(outer_leaf, inner_leaf)
        cavity = Rectangle(
            width=cav_w, height=pane_h, color=P_TEAL, stroke_width=1,
            fill_color=P_TEAL, fill_opacity=0.06,
        ).move_to(np.array([pane_x, pane_cy, 0.0]))
        spacer_h = 0.16
        spacer_top = Rectangle(
            width=cav_w, height=spacer_h, color=P_WHITE, stroke_width=1,
            fill_color="#8892A0", fill_opacity=1.0,
        ).move_to(np.array([pane_x, pane_top - spacer_h / 2, 0.0]))
        spacer_bot = Rectangle(
            width=cav_w, height=spacer_h, color=P_WHITE, stroke_width=1,
            fill_color="#8892A0", fill_opacity=1.0,
        ).move_to(np.array([pane_x, pane_bottom + spacer_h / 2, 0.0]))
        lowe_x = pane_x + pane_w / 2 - leaf_w
        lowe = Line(
            np.array([lowe_x, pane_bottom + spacer_h + 0.05, 0.0]),
            np.array([lowe_x, pane_top - spacer_h - 0.05, 0.0]),
            color=P_ORANGE, stroke_width=3, stroke_opacity=0.55,
        )
        glazing = VGroup(cavity, spacer_top, spacer_bot, outer_leaf, inner_leaf, lowe)

        pane_label = Text("2-fach Isolierglas im Schnitt", font_size=BODY_FONT_SIZE, color=P_CYAN)
        # Right of the pane top: the incoming ray occupies the sky to the left.
        pane_label.next_to(np.array([pane_x - 0.1, 2.4, 0.0]), RIGHT, buff=0.0)

        outside = Text("Außen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(LEFT * 6.0 + UP * 0.9)
        inside = Text("Innen", font_size=BODY_FONT_SIZE, color=P_TEAL).move_to(RIGHT * 4.8 + UP * 1.6)

        hit_out = np.array([pane_x - pane_w / 2, 1.45, 0.0])
        sun_pos = np.array([-5.25, 2.2, 0.0])
        sun = _sun(sun_pos)
        d_in = hit_out - sun_pos
        d_in = d_in / np.linalg.norm(d_in)
        d_ref = np.array([-d_in[0], d_in[1], 0.0])
        d_glass = np.array([d_in[0], d_in[1] * 0.6, 0.0])
        d_glass = d_glass / np.linalg.norm(d_glass)
        hit_in = hit_out + d_glass * (pane_w / d_glass[0])
        ray_end = hit_in + d_in * 3.4

        normal_line = DashedLine(
            hit_out + LEFT * 0.95, hit_out + RIGHT * 0.95,
            color=P_WHITE, stroke_width=1.2, dash_length=0.08, stroke_opacity=0.4,
        )
        incoming = radiation_ray(sun_pos + d_in * 0.5, hit_out, stroke_width=3)
        reflected = radiation_ray(hit_out, hit_out + d_ref * 2.8, color=P_WHITE, stroke_width=2.5).set_stroke(opacity=0.7)
        glass_seg = radiation_ray(hit_out, hit_in, stroke_width=2).set_stroke(opacity=0.5)
        transmitted = radiation_ray(hit_in, ray_end, stroke_width=3)
        ray_path = [[sun_pos + d_in * 0.5, hit_out, hit_in, ray_end]]
        refl_path = [[sun_pos + d_in * 0.5, hit_out, hit_out + d_ref * 2.8]]

        lbl_refl = Text("Reflexion", font_size=BODY_FONT_SIZE, color=P_WHITE).move_to(LEFT * 4.55 + UP * 0.15)
        lbl_tau = math_label(r"τ_{e}", size=FORMULA_FONT_SIZE, color=P_YELLOW)
        lbl_tau.next_to(hit_in + d_in * 2.2, UP, buff=0.22)
        lbl_qi = math_label(r"q_{i}", size=FORMULA_FONT_SIZE, color=P_RED)
        lbl_qi.move_to(np.array([1.8, -0.45, 0.0]))

        glass_warm = [np.array([pane_x + pane_w / 2 + 0.03, y, 0.0]) for y in (0.55, 0.05, -0.4)]

        def glass_heat(rt):
            n = max(1, int(rt / 1.6))
            return [ripples(glass_warm, r_max=0.6, cycles=max(1.0, rt / 1.4), facing=0.0),
                    pulse_flashes(ray_path, P_YELLOW, repeats=n),
                    pulse_flashes(refl_path, P_WHITE, repeats=n, width=3.0)]

        merge_point = np.array([3.4, 0.25, 0.0])
        merge_glow = VGroup(
            Dot(merge_point, radius=0.5, color=P_ORANGE, fill_opacity=0.12),
            Dot(merge_point, radius=0.3, color=P_ORANGE, fill_opacity=0.25),
            Dot(merge_point, radius=0.15, color=P_ORANGE, fill_opacity=0.5),
        )

        eq_row, eq_box, eq_items = math_panel([
            ("g", r"g_{tot}", P_RED), (None, "=", P_WHITE), ("tau", r"τ_{e}", P_YELLOW),
            (None, "+", P_WHITE), ("qi", r"q_{i}", P_RED),
            (None, r"\;[-]", P_TEAL),
        ], color=P_RED)

        g_tr = ValueTracker(0.0)
        g_read = math_readout(
            lambda: rf"g_{{tot}} = {de_num(g_tr.get_value(), 2)}",
            np.array([4.6, 0.15, 0.0]),
            size=BODY_FONT_SIZE, color=P_RED, edge="left",
        )
        self.play(
            Create(glazing), FadeIn(pane_label),
            FadeIn(outside), FadeIn(inside),
            run_time=1.5,
        )

        self.play(FadeIn(sun, scale=0.7), run_time=0.6)
        self.play(
            Create(incoming, rate_func=linear),
            ShowPassingFlash(incoming.copy().set_stroke(P_WHITE, 6), time_width=0.5),
            Create(normal_line),
            run_time=1.1,
        )
        # Impact: reflection and the refracted path split at the same instant, the
        # glass starts to absorb, and both ray labels appear together.
        self.add(g_read)
        self.play(
            Flash(hit_out, color=P_YELLOW, line_length=0.16, num_lines=12, flash_radius=0.34),
            Create(reflected, rate_func=linear),
            Create(glass_seg, rate_func=linear),
            Create(transmitted, rate_func=linear),
            leaves.animate.set_fill(color=P_ORANGE, opacity=0.28),
            FadeIn(lbl_refl), FadeIn(lbl_tau),
            g_tr.animate.set_value(0.50),
            run_time=1.3,
        )
        # Absorbed share re-radiates inward — q_i waves emit in sync with the
        # glass deepening to red and a second pulse leaving the inner face.
        self.play(
            leaves.animate.set_fill(color=P_RED, opacity=0.42).set_stroke(color=P_RED),
            *glass_heat(1.7),
            FadeIn(lbl_qi),
            g_tr.animate.set_value(0.60),
            run_time=1.7,
        )
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 0.6 + 1.1 + 1.3 + 1.7,
                 during=glass_heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gtot"))
        self.play(Create(eq_box), FadeIn(eq_row), *glass_heat(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "gtot", during=glass_heat)

        tau_copy = lbl_tau.copy()
        qi_copy = lbl_qi.copy()
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "parts"))
        self.add(tau_copy, qi_copy)
        self.play(
            tau_copy.animate.scale(0.7).move_to(merge_point + LEFT * 0.62),
            qi_copy.animate.scale(0.7).move_to(merge_point + RIGHT * 0.62),
            FadeIn(merge_glow), *glass_heat(1.6),
            run_time=1.6,
        )
        g_target = eq_items["g"]
        self.play(
            ReplacementTransform(tau_copy, g_target.copy().set_opacity(0.0)),
            ReplacementTransform(qi_copy, g_target.copy().set_opacity(0.0)),
            FadeOut(merge_glow),
            Indicate(g_target, color=P_ORANGE, scale_factor=1.15),
            run_time=1.2,
        )
        ring = highlight_param(eq_items, "g", color=P_ORANGE)
        self.play(Create(ring), run_time=0.5)
        hold_for(self, self.NARRATION, "parts", during=glass_heat)
        self.play(FadeOut(ring), run_time=0.5)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meaning"))
        hold_for(self, self.NARRATION, "meaning", during=lambda rt: glass_heat(rt)[:2])
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – Final Solar Cooling Load (Q̇_S,tr)
class Beat5_SolarCoolingLoad(Scene):
    NARRATION = [
        ("intro",
         "By multiplying the raw solar irradiance by our building's gross area, and then applying our three dimensionless reduction filters, the frame factor, the shading factor, and the glass transmittance, we arrive at our answer.",
         "Bestrahlungsstärke mal Fläche, gefiltert durch Rahmenfaktor, Verschattungsfaktor und g tot."),
        ("result",
         "This is Q-dot S,tr — the solar cooling load through the glazing.",
         "Das ist Q-Punkt-S,tr — die solare Kühllast durch die Verglasung."),
        ("meaning",
         "It is the precise thermal wattage our mechanical system must actively remove to prevent the room from overheating.",
         "Genau diese Leistung muss die Anlage aktiv abführen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Solare Kühllast", title)
        din = _din_ref("VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room = room_section(np.array([0.0, 0.25, 0.0]), w=11.0, h=2.7, window=(0.3, 0.88))
        sun_pos = np.array([-6.45, 1.45, 0.0])
        sun = _sun(sun_pos)
        ys = np.linspace(room["win_hi"] - 0.18, room["win_lo"] + 0.18, 4)
        rays = sun_rays(sun_pos, room["glass_x"], ys, room["y_f"], gap=0.5)
        ray_paths = [[s, h, p] for s, h, p in zip(rays["starts"], rays["hits"], rays["lands"])]
        warm_spots = [p + UP * 0.03 for p in rays["lands"]]

        def solar_load(rt, r_max=0.75):
            return [ripples(warm_spots, r_max=r_max, color=P_ORANGE, cycles=max(1.0, rt / 1.3)),
                    pulse_flashes(ray_paths, P_YELLOW, repeats=max(1, int(rt / 1.6)))]

        eq_row, eq_box, eq_items = math_panel([
            ("q", r"\dot{Q}_{S,tr}", P_YELLOW), (None, "=", P_WHITE),
            ("A", "A", P_BLUE), (None, r"\cdot", P_WHITE),
            ("ff", r"F_{F}", P_WHITE), (None, r"\cdot", P_WHITE),
            ("fv", r"F_{V}", P_TEAL), (None, r"\cdot", P_WHITE),
            ("g", r"g_{tot}", P_RED), (None, r"\cdot", P_WHITE),
            ("i", r"I_{S,max}", P_YELLOW),
            (None, r"\;[\mathrm{W}]", P_TEAL),
        ], color=P_YELLOW, buff=0.12)

        self.play(Create(room["group"]), run_time=1.8)
        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(shine(rays), run_time=1.6)
        self.play(room["air"].animate.set_fill(P_RED, opacity=0.08), *solar_load(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.8 + 0.7 + 1.6 + 1.6,
                 during=solar_load)

        ff_tr = ValueTracker(1.0)
        fv_tr = ValueTracker(1.0)
        g_tr = ValueTracker(0.0)
        i_tr = ValueTracker(0.0)
        read_at = np.array([3.9, 1.2, 0.0])
        gx = room["glass_x"]
        win_mid_y = (room["win_hi"] + room["win_lo"]) / 2
        frame = Rectangle(
            width=0.28, height=(room["win_hi"] - room["win_lo"]) + 0.16,
            color=P_WHITE, stroke_width=2.4,
        ).move_to(np.array([gx, win_mid_y, 0.0]))
        slats = VGroup(*[
            Line(
                np.array([gx - 0.46, y, 0.0]),
                np.array([gx - 0.14, y - 0.12, 0.0]),
                color=P_TEAL, stroke_width=3,
            )
            for y in np.linspace(room["win_lo"] + 0.16, room["win_hi"] - 0.16, 5)
        ])
        ff_target = _measured_frame_factor(3.6, 2.5)

        def _factor_read(src, color):
            return math_readout(src, read_at, size=BODY_FONT_SIZE, color=color, edge="left")

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "result"))
        self.play(Create(eq_box), FadeIn(eq_row), Indicate(room["glass"], color=P_BLUE), *solar_load(0.8), run_time=0.8)
        ff_read = _factor_read(lambda: rf"F_{{F}} = {de_num(ff_tr.get_value(), 2)}", P_WHITE)
        self.add(ff_read)
        self.play(Create(frame), ff_tr.animate.set_value(ff_target), *solar_load(1.0), run_time=1.0)
        self.play(FadeOut(ff_read), run_time=0.2)
        fv_read = _factor_read(lambda: rf"F_{{V}} = {de_num(fv_tr.get_value(), 2)}", P_TEAL)
        self.add(fv_read)
        slats.shift(UP * 1.1).set_opacity(0)
        self.add(slats)
        self.play(
            slats.animate.shift(DOWN * 1.1).set_opacity(1),
            fv_tr.animate.set_value(0.15),
            rays["group"].animate.set_stroke(opacity=0.22),
            run_time=1.2,
        )
        self.play(FadeOut(fv_read), run_time=0.2)
        g_read = _factor_read(lambda: rf"g_{{tot}} = {de_num(g_tr.get_value(), 2)}", P_RED)
        self.add(g_read)
        self.play(
            room["glass"].animate.set_fill(P_ORANGE, opacity=0.45),
            g_tr.animate.set_value(0.60),
            *solar_load(1.0),
            run_time=1.0,
        )
        self.play(FadeOut(g_read), run_time=0.2)
        i_read = _factor_read(lambda: rf"I_{{S,max}} = {de_num(i_tr.get_value())}", P_YELLOW)
        self.add(i_read)
        self.play(Indicate(sun, color=P_YELLOW), i_tr.animate.set_value(800), *solar_load(1.0), run_time=1.0)
        self.play(FadeOut(i_read), run_time=0.2)
        ring = highlight_param(eq_items, "q", color=P_YELLOW)
        self.play(Create(ring), room["air"].animate.set_fill(P_RED, opacity=0.16), *solar_load(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "result", used=0.8 + 1.0 + 0.2 + 1.2 + 0.2 + 1.0 + 0.2 + 1.0 + 0.2 + 0.8 + 0.35,
                 during=solar_load)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meaning"))
        hold_for(self, self.NARRATION, "meaning", during=lambda rt: solar_load(rt, r_max=0.95))
        self.play(FadeOut(ring), FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
