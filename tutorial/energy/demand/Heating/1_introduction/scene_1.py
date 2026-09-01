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
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, SAFE_BOTTOM, SAFE_BOTTOM_FORMULA, fit_band,
    radiation_waves, convection_stream, symbol_token, watt_anchor,
    smooth_path, flow_guides, animate_flow, animate_flows, animate_haze,
    meter, bind_meter, chip, cross_mark, dim_chip, dim_arrow,
    equation_row, formula_panel, highlight_param,
    caption_bar, swap_caption, hold_for, subtitle_text,
)

# 🏔️ Persistent module title — written once on Beat1, self.add()'ed on later beats.
TITLE_DE = "Modul 1: Die Grundlagen der Bauphysik"


#region The through-line: one wall section, reused by every beat
# Conduction, convection and radiation are three mechanisms crossing the *same*
# temperature difference. Showing each in its own unrelated cartoon hides that;
# every beat therefore redraws this identical section and only changes the
# mechanism drawn on it.
SECTION_C = np.array([0.0, 0.80, 0.0])
SECTION_W, SECTION_H = 9.2, 2.6
WALL_W = 1.70
THETA_I, THETA_E = 20, 0


def _section(*, cy=SECTION_C[1], warm_opacity=0.24, cold_opacity=0.22):
    """🧱 Warm room | solid wall | cold outside — the one scaffold this module reuses.

    Fill opacities were 0.16/0.14 and the room and outside barely separated from
    the near-black background once the clip was compressed — the first frame of
    every beat looked empty. Raised so the warm/cold split reads immediately,
    and the wall carries a denser fill and stroke so it always reads as *solid
    material*, which is the whole premise of the module (heat has to cross it).

    ``cy`` is the vertical centre. Beats that draw motion *outside* the box
    (Beat 3's buoyancy lanes) drop the whole scaffold so those lanes clear the
    temperature labels below and the beat subtitle above.
    """
    center = np.array([SECTION_C[0], cy, 0.0])
    half_w, half_h = SECTION_W / 2, SECTION_H / 2
    wall_half = WALL_W / 2
    top, bottom = cy + half_h, cy - half_h

    warm = Rectangle(
        width=half_w - wall_half, height=SECTION_H,
        color=P_ORANGE, stroke_width=2,
        fill_color=P_ORANGE, fill_opacity=warm_opacity,
    ).move_to(np.array([-(half_w + wall_half) / 2, cy, 0.0]))
    cold = Rectangle(
        width=half_w - wall_half, height=SECTION_H,
        color=P_BLUE, stroke_width=2,
        fill_color=P_BLUE, fill_opacity=cold_opacity,
    ).move_to(np.array([(half_w + wall_half) / 2, cy, 0.0]))
    wall = Rectangle(
        width=WALL_W, height=SECTION_H,
        color=P_WHITE, stroke_width=3.0,
        fill_color=P_WHITE, fill_opacity=0.11,
    ).move_to(center)

    ti = Text(f"innen {THETA_I} °C", font_size=BODY_FONT_SIZE, color=P_ORANGE)
    ti.move_to(np.array([warm.get_center()[0], bottom - 0.42, 0.0]))
    te = Text(f"außen {THETA_E} °C", font_size=BODY_FONT_SIZE, color=P_BLUE)
    te.move_to(np.array([cold.get_center()[0], bottom - 0.42, 0.0]))
    dth = Text("Δθ = 20 K", font_size=BODY_FONT_SIZE, color=P_YELLOW)
    dth.move_to(np.array([0.0, bottom - 0.42, 0.0]))

    return {
        "warm": warm, "cold": cold, "wall": wall,
        "ti": ti, "te": te, "dth": dth,
        "top": top, "bottom": bottom,
        "wall_l": -wall_half, "wall_r": wall_half,
        "shell": VGroup(warm, cold, wall),
        "group": VGroup(warm, cold, wall, ti, te, dth),
    }


def _route_strip(active: int | None = None):
    """🧭 Leitung · Konvektion · Strahlung — which of the three a beat is on."""
    names = ("Leitung", "Konvektion", "Strahlung")
    colors = (P_RED, P_CYAN, P_YELLOW)
    strip = VGroup(*[
        chip(n, c if active == i else P_TEAL, font_size=LABEL_FONT_SIZE)
        for i, (n, c) in enumerate(zip(names, colors))
    ]).arrange(RIGHT, buff=0.5)
    strip.move_to(np.array([0.0, -2.10, 0.0]))
    if active is not None:
        for i, boxed in enumerate(strip):
            if i != active:
                boxed[0].set_stroke(opacity=0.35)
                boxed[1].set_fill(opacity=0.35)
    return strip


def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Occupant glyph — deliberately unlike a heat packet, so the two never read alike."""
    head = Circle(radius=0.11, color=color, stroke_width=2.2)
    body = RoundedRectangle(
        width=0.30, height=0.40, corner_radius=0.09, color=color, stroke_width=2.2,
    ).next_to(head, DOWN, buff=0.04)
    return VGroup(head, body).scale(scale).move_to(pos)


def _quantum(pos, color=P_YELLOW, scale=1.0):
    """〰️ A packet of heat — a little wave, never a dot, so it cannot be mistaken for a person."""
    pts = []
    for i in range(26):
        t = i / 25
        pts.append(np.array([(t - 0.5) * 0.46, 0.12 * np.sin(t * TAU * 1.5), 0.0]))
    wave = VMobject(color=color, stroke_width=6.0)
    wave.set_points_smoothly(pts)
    return wave.scale(scale).move_to(pos)


def _molecules(bounds, cols=5, rows=4, color=P_RED, radius=0.068):
    """⚛️ Lattice of bound molecules inside the wall — they vibrate, they never travel."""
    x0, x1, y0, y1 = bounds
    xs = np.linspace(x0, x1, cols)
    ys = np.linspace(y0, y1, rows)
    grid = VGroup()
    for y in ys:
        for x in xs:
            grid.add(Dot(np.array([x, y, 0.0]), radius=radius, color=color, fill_opacity=0.9))
    return grid, xs, ys


def _flow_arrow(start, end, color, width=4):
    """➡️ Plain directed heat arrow."""
    return Arrow(
        np.array(start, dtype=float), np.array(end, dtype=float),
        buff=0, color=color, stroke_width=width, max_tip_length_to_length_ratio=0.22,
    )


def _layer(width, height, color, opacity, label, *, center=ORIGIN, font_size=LABEL_FONT_SIZE):
    """🧱 One construction layer — the unit R_ges is summed over. No inline tag:

    columns as thin as 0.26 units (an outer render coat) can never fit a
    readable label under themselves without it overlapping the next column;
    see ``_layer_legend`` for the paired swatch-and-name key instead.
    """
    return Rectangle(
        width=width, height=height, color=color, stroke_width=2,
        fill_color=color, fill_opacity=opacity,
    ).move_to(center)


def _layer_legend(entries):
    """🏷️ Swatch + name per layer, stacked — the readable twin of a too-narrow ``_layer``."""
    from manim import RIGHT, Rectangle

    rows = VGroup(*[
        VGroup(
            Rectangle(width=0.32, height=0.20, color=color, fill_color=color,
                     fill_opacity=0.75, stroke_width=1.5),
            Text(name, font_size=LABEL_FONT_SIZE, color=color),
        ).arrange(RIGHT, buff=0.14)
        for name, color in entries
    ]).arrange(DOWN, aligned_edge=LEFT, buff=0.14)
    return rows


def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the same empty top-right corner.

    Every section of Module 1 names the norm it is built on (mostly
    DIN EN ISO 6946 for building-component heat transfer; DIN EN 12831-1 where
    the beat is really about the heating load). Dim so it reads as a footnote,
    never competing with the diagram.
    """
    ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Beat1 – Heat crosses from warm to cold, and only that way
class Beat1_DreiWegeDerWaerme(Scene):
    NARRATION = [
        ("intro",
         "Before we calculate how much heat a building loses, we need to see how heat moves at all.",
         "Bevor wir berechnen, wie viel Wärme ein Gebäude verliert, sehen wir uns an, wie Wärme sich überhaupt bewegt."),
        ("setup",
         "Here is the whole problem in one picture: a warm room at twenty degrees, a cold night at zero, and a wall in between.",
         "Das ganze Problem in einem Bild: ein warmer Raum bei zwanzig Grad, eine kalte Nacht bei null Grad — und dazwischen eine Wand."),
        ("driver",
         "That twenty kelvin difference is the only driver. No difference, no heat flow.",
         "Diese zwanzig Kelvin Unterschied sind der einzige Antrieb. Kein Unterschied, kein Wärmestrom."),
        ("direction",
         "Energy always crosses from warm toward cold, and the gap closes a little as it does.",
         "Energie wandert immer von warm nach kalt — und der Unterschied wird dabei ein Stück kleiner."),
        ("never",
         "Never the other way. That is the second law, and it is the reason a building has to be heated at all.",
         "Nie umgekehrt. Das ist der zweite Hauptsatz — und der Grund, warum ein Gebäude überhaupt beheizt werden muss."),
        ("three",
         "Heat makes that crossing in exactly three ways, and the whole module is built on them.",
         "Wärme macht diesen Übergang auf genau drei Wegen — auf ihnen baut das ganze Modul auf."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Abschnitt 1.1 — Wie Wärme sich bewegt", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        sec = _section()
        people = VGroup(*[
            _person(np.array([x, SECTION_C[1] - 0.62, 0.0]), scale=0.95)
            for x in (-3.55, -2.62, -1.69)
        ])
        ti_hot = Text(f"innen {THETA_I} °C", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(sec["ti"])
        ti_cool = Text("innen 19 °C", font_size=BODY_FONT_SIZE, color=P_ORANGE).move_to(sec["ti"])
        te_cold = Text(f"außen {THETA_E} °C", font_size=BODY_FONT_SIZE, color=P_BLUE).move_to(sec["te"])
        te_warm = Text("außen 1 °C", font_size=BODY_FONT_SIZE, color=P_BLUE).move_to(sec["te"])
        dth_full = Text("Δθ = 20 K", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(sec["dth"])
        dth_less = Text("Δθ = 18 K", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(sec["dth"])

        scaffold = VGroup(sec["group"], people)
        fit_band(scaffold)

        # Free-floating orange haze drifting warm→cold — not yellow glyphs and not
        # beads on guide rails (those never read as air).
        reverse = _flow_arrow(
            np.array([2.6, SECTION_C[1] - 1.05, 0.0]),
            np.array([-2.6, SECTION_C[1] - 1.05, 0.0]),
            P_RED,
        )
        # A proper "no entry" sign, not a bare tick: at size 0.22 the old cross
        # read as a stray diagonal line over the arrow.
        no_mark = VGroup(
            Circle(radius=0.37, color=P_RED, stroke_width=5),
            Line(UP * 0.26 + LEFT * 0.26, DOWN * 0.26 + RIGHT * 0.26,
                 color=P_RED, stroke_width=5),
        ).move_to(np.array([0.0, SECTION_C[1] - 1.02, 0.0]))

        routes = _route_strip()

        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "setup"))
        self.play(FadeIn(sec["warm"]), FadeIn(sec["cold"]), Create(sec["wall"]), run_time=1.5)
        self.play(FadeIn(people), FadeIn(ti_hot), FadeIn(te_cold), run_time=1.0)
        hold_for(self, self.NARRATION, "setup", used=1.5 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "driver"))
        self.play(FadeIn(dth_full, shift=UP * 0.12), run_time=0.8)
        self.play(Indicate(dth_full, color=P_YELLOW, scale_factor=1.2), run_time=0.9)
        hold_for(self, self.NARRATION, "driver", used=0.8 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "direction"))
        haze_kwargs = dict(
            x0=-3.9,
            x1=3.95,
            y0=SECTION_C[1] - 0.15,
            y1=SECTION_C[1] + 1.15,
            color=P_ORANGE,
            color_end="#C9786E",
            n=56,
        )
        animate_haze(
            self,
            run_time=3.2,
            cycles=1.8,
            extra=[
                ReplacementTransform(ti_hot, ti_cool),
                ReplacementTransform(te_cold, te_warm),
                ReplacementTransform(dth_full, dth_less),
            ],
            **haze_kwargs,
        )
        animate_haze(self, run_time=2.0, cycles=1.4, seed=11, **haze_kwargs)
        hold_for(self, self.NARRATION, "direction", used=3.2 + 2.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "never"))
        self.play(GrowArrow(reverse), run_time=0.8)
        self.play(
            Create(no_mark),
            reverse.animate.set_stroke(color=P_WHITE, opacity=0.22),
            run_time=0.7,
        )
        hold_for(self, self.NARRATION, "never", used=0.8 + 0.7 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "three"))
        self.play(FadeOut(reverse), FadeOut(no_mark), run_time=0.5)
        self.play(LaggedStart(*[FadeIn(c, shift=UP * 0.14) for c in routes], lag_ratio=0.22), run_time=1.4)
        hold_for(self, self.NARRATION, "three", used=0.5 + 1.4 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat2 – Conduction: the lattice hands energy along, and stays put
class Beat2_Waermeleitung(Scene):
    NARRATION = [
        ("label",
         "The first way is conduction, and it happens inside the solid wall itself.",
         "Der erste Weg ist die Wärmeleitung — sie passiert in der festen Wand selbst."),
        ("lattice",
         "The wall is a lattice of molecules, each one bound to its place.",
         "Die Wand ist ein Gitter aus Molekülen, jedes fest an seinen Platz gebunden."),
        ("relay",
         "The warm side makes them vibrate harder, and each one knocks its neighbour into motion. The energy travels along the chain.",
         "Die warme Seite lässt sie stärker schwingen, und jedes stößt seinen Nachbarn an. Die Energie wandert die Kette entlang."),
        ("stayput",
         "But watch one molecule: it never leaves its place. Only the energy moves through the wall, not the material.",
         "Aber sehen Sie ein Molekül an: es verlässt seinen Platz nie. Nur die Energie wandert durch die Wand, nicht das Material."),
        ("symbol",
         "That heat flow through the solid is called Q dot k.",
         "Diesen Wärmestrom durch den Feststoff nennen wir Q-Punkt-k."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmeleitung — Energie ohne Materialtransport", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "label"))
        self.play(FadeIn(caption), run_time=0.3)

        sec = _section()
        routes = _route_strip(active=0)
        scaffold = VGroup(sec["group"])
        fit_band(scaffold)

        grid, xs, ys = _molecules(
            (sec["wall_l"] + 0.22, sec["wall_r"] - 0.22,
             SECTION_C[1] - 0.95, SECTION_C[1] + 0.95),
        )
        cols, rows = len(xs), len(ys)

        def _column(i):
            return VGroup(*[grid[r * cols + i] for r in range(rows)])

        watched = grid[2 * cols + 0]
        watch_ring = Circle(radius=0.20, color=P_GREEN, stroke_width=2.5).move_to(watched)
        watch_tag = Text("bleibt am Platz", font_size=LABEL_FONT_SIZE, color=P_GREEN)
        watch_tag.next_to(watch_ring, LEFT, buff=0.30)

        home = watched.get_center().copy()

        hold_for(self, self.NARRATION, "label", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(FadeIn(sec["warm"]), FadeIn(sec["cold"]), Create(sec["wall"]), run_time=1.2)
        self.play(FadeIn(sec["ti"]), FadeIn(sec["te"]), FadeIn(routes), run_time=0.8)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lattice"))
        self.play(LaggedStart(*[FadeIn(d, scale=0.5) for d in grid], lag_ratio=0.02), run_time=1.4)
        hold_for(self, self.NARRATION, "lattice", used=1.4 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "relay"))
        self.play(
            Succession(*[
                Indicate(_column(i), color=P_YELLOW, scale_factor=1.65)
                for i in range(cols)
            ]),
            run_time=2.6,
        )
        hold_for(self, self.NARRATION, "relay", used=2.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "stayput"))
        self.play(Create(watch_ring), FadeIn(watch_tag), run_time=0.8)
        self.play(watched.animate.move_to(home + RIGHT * 0.20), run_time=0.6)
        self.wait(0.2)
        self.play(watched.animate.move_to(home), run_time=0.6)
        hold_for(self, self.NARRATION, "stayput", used=0.8 + 0.6 + 0.2 + 0.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "symbol"))
        token = symbol_token("Q̇_k", color=P_RED, font_size=FORMULA_FONT_SIZE)
        token.move_to(SECTION_C)
        self.play(
            FadeOut(watch_ring), FadeOut(watch_tag),
            ReplacementTransform(grid, token),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "symbol", used=1.2 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat3 – Convection: this time the carrier itself leaves
class Beat3_Konvektion(Scene):
    NARRATION = [
        ("label",
         "The second way is convection, and here something else happens: the carrier itself leaves.",
         "Der zweite Weg ist die Konvektion — und hier passiert etwas anderes: der Träger selbst verschwindet."),
        ("gap",
         "Give the wall a gap — an open window, a leaky joint — and the air can move through it.",
         "Gibt man der Wand eine Öffnung — ein offenes Fenster, eine undichte Fuge — kann Luft hindurch."),
        ("loop",
         "Warm indoor air is lighter, so it rises, slips out through the top of the gap, and cold outdoor air sinks in below to replace it.",
         "Warme Raumluft ist leichter, steigt auf und entweicht oben durch die Öffnung — kalte Außenluft sinkt unten nach."),
        ("carrier",
         "Follow one parcel of air: unlike the molecules in the wall, this one physically leaves the building, and it takes its energy with it.",
         "Verfolgen Sie ein Luftpaket: anders als die Moleküle in der Wand verlässt es das Gebäude wirklich — und nimmt seine Energie mit."),
        ("symbol",
         "That heat flow carried by moving air is Q dot c.",
         "Diesen von bewegter Luft getragenen Wärmestrom nennen wir Q-Punkt-c."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Konvektion — die Luft nimmt die Wärme mit", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "label"))
        self.play(FadeIn(caption), run_time=0.3)

        # This beat draws its buoyancy lanes *outside* the box on both sides, so
        # the whole scaffold drops: the exit plume then clears the beat subtitle
        # above, and the indoor/outdoor lanes clear the temperature labels below.
        cy = SECTION_C[1] - 0.40
        sec = _section(cy=cy)
        routes = _route_strip(active=1)

        # Tall enough to have an upper and a lower half: with one opening, warm air
        # leaves through the top and cold air enters through the bottom of that same
        # gap. That two-way split is the whole point of the beat.
        gap_lo, gap_hi = cy - 0.35, cy + 0.85
        wall_upper = Rectangle(
            width=WALL_W, height=sec["top"] - gap_hi, color=P_WHITE, stroke_width=3.0,
            fill_color=P_WHITE, fill_opacity=0.11,
        ).move_to(np.array([0.0, (sec["top"] + gap_hi) / 2, 0.0]))
        wall_lower = Rectangle(
            width=WALL_W, height=gap_lo - sec["bottom"], color=P_WHITE, stroke_width=3.0,
            fill_color=P_WHITE, fill_opacity=0.11,
        ).move_to(np.array([0.0, (gap_lo + sec["bottom"]) / 2, 0.0]))
        gap_tag = Text("Öffnung", font_size=LABEL_FONT_SIZE, color=P_CYAN)
        gap_tag.move_to(np.array([0.0, sec["top"] + 0.28, 0.0]))

        # Buoyancy exchange through the one opening, drawn as clean laminar
        # streaklines: each lane keeps a single monotonic vertical trend — a
        # steady rise for warm-out, a steady fall for cold-in — so the curves
        # read as smooth flow rather than a wavy ribbon. The three-lane bundle
        # fans out where the air is free and pinches back together to thread the
        # gap. Both streams cross the wall squarely inside [gap_lo, gap_hi]:
        # warm-out through the upper part of the opening, cold-in through the
        # lower part, the two bands close but never touching.
        LANE = 0.115

        def _stream(spine):
            return VGroup(*[
                smooth_path([
                    np.array([x, cy + yc + k * LANE * taper, 0.0])
                    for x, yc, taper in spine
                ])
                for k in (-1.0, 0.0, 1.0)
            ])

        warm_out = _stream([
            (-4.15, -0.72, 1.30), (-3.05, -0.22, 1.15), (-1.85, 0.22, 1.00),
            (-0.80, 0.50, 0.85), (0.00, 0.60, 0.80), (0.80, 0.62, 0.85),
            (1.95, 0.86, 1.05), (3.05, 1.06, 1.20), (4.15, 1.16, 1.30),
        ])
        cold_in = _stream([
            (4.15, 0.52, 1.30), (3.05, 0.20, 1.20), (1.85, -0.06, 1.05),
            (0.80, -0.16, 0.85), (0.00, -0.20, 0.80), (-0.80, -0.22, 0.85),
            (-1.85, -0.46, 1.05), (-3.05, -0.66, 1.20), (-4.15, -0.78, 1.30),
        ])

        # Lane tags land in genuinely empty bands: "raus" above the section top
        # (freed now the box sits lower), "rein" low on the outside, both clear
        # of every lane and of the temperature labels.
        out_tag = Text("warme Luft raus", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        out_tag.move_to(np.array([3.15, sec["top"] + 0.46, 0.0]))
        in_tag = Text("kalte Luft rein", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        in_tag.move_to(np.array([3.65, cy - 1.00, 0.0]))

        parcel = Dot(radius=0.14, color=P_ORANGE, fill_opacity=1.0)
        parcel_glow = Circle(radius=0.22, color=P_YELLOW, stroke_width=2.0, fill_opacity=0.0)
        parcel_path = warm_out[1]
        parcel.move_to(parcel_path.get_start())
        parcel_glow.move_to(parcel.get_center())
        parcel_tag = Text("ein Luftpaket", font_size=LABEL_FONT_SIZE, color=P_ORANGE)

        hold_for(self, self.NARRATION, "label", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(FadeIn(sec["warm"]), FadeIn(sec["cold"]), Create(sec["wall"]), run_time=1.2)
        self.play(FadeIn(sec["ti"]), FadeIn(sec["te"]), FadeIn(routes), run_time=0.8)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gap"))
        self.play(
            ReplacementTransform(sec["wall"], VGroup(wall_upper, wall_lower)),
            FadeIn(gap_tag),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "gap", used=1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "loop"))
        guides = VGroup(
            flow_guides(warm_out, P_ORANGE, opacity=0.42, width=2.4),
            flow_guides(cold_in, P_BLUE, opacity=0.42, width=2.4),
        )
        self.play(Create(guides), FadeIn(out_tag), FadeIn(in_tag), run_time=1.2)
        # Both directions at once, looping: a real gap exchanges air as a continuous
        # buoyancy circuit — sequential one-way crossings looked like a single bead.
        animate_flows(
            self,
            [
                (warm_out, P_ORANGE, P_YELLOW),
                (cold_in, P_BLUE, P_CYAN),
            ],
            run_time=3.6,
            waves=7,
            cycles=3.2,
            radius=0.048,
        )
        hold_for(self, self.NARRATION, "loop", used=1.2 + 3.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "carrier"))
        # A fixed callout spot, not next_to(parcel): the parcel spawns low at
        # the room's left wall with the warm_out curve threading past, so the
        # tag sits high in the room's open interior, clear of every lane.
        parcel_tag.move_to(np.array([-3.15, cy + 0.78, 0.0]))
        self.play(
            FadeIn(parcel, scale=0.5),
            FadeIn(parcel_glow),
            FadeIn(parcel_tag),
            run_time=0.6,
        )
        self.play(FadeOut(parcel_tag), run_time=0.3)
        # Highlighted parcel rides the warm route once while the exchange keeps
        # circulating behind it — "this one actually leaves" stays legible.
        animate_flows(
            self,
            [
                (warm_out, P_ORANGE, P_YELLOW),
                (cold_in, P_BLUE, P_CYAN),
            ],
            run_time=3.2,
            waves=7,
            cycles=3.0,
            radius=0.045,
            extra=[
                MoveAlongPath(parcel, parcel_path, rate_func=linear),
                MoveAlongPath(parcel_glow, parcel_path, rate_func=linear),
            ],
        )
        self.play(
            parcel.animate.shift(RIGHT * 1.15).set_opacity(0.0),
            parcel_glow.animate.shift(RIGHT * 1.15).set_opacity(0.0),
            run_time=0.8,
        )
        hold_for(self, self.NARRATION, "carrier", used=0.6 + 0.3 + 3.2 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "symbol"))
        token = symbol_token("Q̇_c", color=P_CYAN, font_size=FORMULA_FONT_SIZE)
        token.move_to(np.array([0.0, cy + 0.30, 0.0]))
        self.play(
            ReplacementTransform(guides, token),
            FadeOut(gap_tag), FadeOut(out_tag), FadeOut(in_tag),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "symbol", used=1.2 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat4 – Radiation: the only one that needs nothing in between
class Beat4_Strahlung(Scene):
    NARRATION = [
        ("label",
         "The third way is radiation, and it is the odd one out.",
         "Der dritte Weg ist die Strahlung — und sie fällt aus der Reihe."),
        ("waves",
         "Every warm surface sends out infrared waves. The warm side of the wall radiates straight across to whatever is colder.",
         "Jede warme Oberfläche sendet Infrarotwellen aus. Die warme Wandseite strahlt direkt zu allem, was kälter ist."),
        ("vacuum",
         "Now take the air away completely. Conduction stops, convection stops — there is nothing left to carry anything.",
         "Nehmen wir die Luft ganz weg: Leitung hört auf, Konvektion hört auf — es ist nichts mehr da, das etwas tragen könnte."),
        ("still",
         "And the radiation carries on regardless. It needs no medium at all, which is exactly how the sun reaches us across empty space.",
         "Und die Strahlung läuft weiter. Sie braucht kein Medium — genau so erreicht uns die Sonne durch den leeren Raum."),
        ("symbol",
         "This heat flow is Q dot r.",
         "Diesen Wärmestrom nennen wir Q-Punkt-r."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Strahlung — braucht kein Medium", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "label"))
        self.play(FadeIn(caption), run_time=0.3)

        sec = _section()
        routes = _route_strip(active=2)
        fit_band(VGroup(sec["group"]))

        rng = np.random.default_rng(5)
        air = VGroup(*[
            Dot(
                np.array([
                    float(rng.uniform(sec["wall_r"] + 0.35, 4.35)),
                    float(rng.uniform(SECTION_C[1] - 1.05, SECTION_C[1] + 1.05)),
                    0.0,
                ]),
                radius=0.05, color=P_BLUE, fill_opacity=0.75,
            )
            for _ in range(22)
        ])
        air_tag = Text("Luft", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        air_tag.move_to(np.array([3.6, SECTION_C[1] + 1.42, 0.0]))
        vac_tag = Text("Vakuum — kein Medium", font_size=LABEL_FONT_SIZE, color=P_WHITE)
        vac_tag.move_to(air_tag)

        waves = VGroup(*[
            radiation_waves(
                np.array([sec["wall_r"] + 0.1, SECTION_C[1] + dy, 0.0]),
                n=1, color=P_YELLOW, height=2.1, x_spread=0.0, stroke_width=2.4,
            ).rotate(-PI / 2, about_point=np.array([sec["wall_r"] + 0.1, SECTION_C[1] + dy, 0.0]))
            for dy in (-0.85, -0.28, 0.29, 0.86)
        ])

        stopped = VGroup(
            cross_mark(P_RED, size=0.16),
            Text("Leitung · Konvektion", font_size=LABEL_FONT_SIZE, color=P_RED),
        ).arrange(RIGHT, buff=0.18)
        stopped.move_to(np.array([0.0, SECTION_C[1] - 1.72, 0.0]))

        hold_for(self, self.NARRATION, "label", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(FadeIn(sec["warm"]), FadeIn(sec["cold"]), Create(sec["wall"]), run_time=1.2)
        self.play(FadeIn(sec["ti"]), FadeIn(sec["te"]), FadeIn(routes), run_time=0.8)
        self.play(FadeIn(air), FadeIn(air_tag), run_time=0.9)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "waves"))
        self.play(LaggedStart(*[Create(w) for w in waves], lag_ratio=0.18), run_time=1.6)
        hold_for(self, self.NARRATION, "waves", used=1.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "vacuum"))
        self.play(
            LaggedStart(*[FadeOut(d, scale=0.3) for d in air], lag_ratio=0.04),
            ReplacementTransform(air_tag, vac_tag),
            sec["cold"].animate.set_fill(opacity=0.04),
            run_time=1.8,
        )
        self.play(FadeIn(stopped, shift=UP * 0.1), run_time=0.8)
        hold_for(self, self.NARRATION, "vacuum", used=1.8 + 0.8 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "still"))
        self.play(
            LaggedStart(*[Indicate(w, color=P_YELLOW, scale_factor=1.06) for w in waves], lag_ratio=0.15),
            run_time=1.6,
        )
        self.play(waves.animate.set_stroke(width=3.4), run_time=0.6)
        hold_for(self, self.NARRATION, "still", used=1.6 + 0.6 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "symbol"))
        token = symbol_token("Q̇_r", color=P_YELLOW, font_size=FORMULA_FONT_SIZE)
        token.move_to(np.array([2.3, SECTION_C[1], 0.0]))
        self.play(
            ReplacementTransform(waves, token),
            FadeOut(stopped), FadeOut(vac_tag),
            run_time=1.2,
        )
        hold_for(self, self.NARRATION, "symbol", used=1.2 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat5 – The three add up to one heat flow
class Beat5_Zusammenfassung(Scene):
    NARRATION = [
        ("recap",
         "Conduction, convection, radiation — three mechanisms, all crossing the same temperature difference, all in the same direction.",
         "Leitung, Konvektion, Strahlung — drei Mechanismen über dieselbe Temperaturdifferenz, alle in dieselbe Richtung."),
        ("sum",
         "In a real wall all three run at once, so the total heat flow is simply their sum.",
         "In einer echten Wand laufen alle drei gleichzeitig — der gesamte Wärmestrom ist also einfach ihre Summe."),
        ("k", "Q dot k through the solid material.", "Q-Punkt-k durch das feste Material."),
        ("c", "Q dot c carried by moving air.", "Q-Punkt-c getragen von bewegter Luft."),
        ("r", "Q dot r radiated across the gap.", "Q-Punkt-r gestrahlt über den Zwischenraum."),
        ("standard",
         "Splitting them apart every time would be unusable in practice, which is why DIN EN ISO 6946 bundles them into a single number per building element.",
         "Sie jedes Mal zu trennen wäre unbrauchbar — deshalb fasst die DIN EN ISO 6946 sie zu einer einzigen Kennzahl je Bauteil zusammen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Zusammenfassung — drei Wege, ein Wärmestrom", title)
        # No corner citation here: this beat's punchline *is* the standard, and it
        # gets a full chip in the diagram ("… deshalb fasst die DIN EN ISO 6946 …").
        self.play(FadeIn(subtitle), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "recap"))
        self.play(FadeIn(caption), run_time=0.3)

        sec = _section()
        fit_band(VGroup(sec["group"]), bottom=SAFE_BOTTOM_FORMULA)

        lattice, _, _ = _molecules(
            (sec["wall_l"] + 0.26, sec["wall_r"] - 0.26,
             SECTION_C[1] - 0.55, SECTION_C[1] + 0.55),
            cols=4, rows=3, radius=0.055,
        )
        stream = convection_stream(
            np.array([-2.6, SECTION_C[1] - 0.85, 0.0]),
            np.array([2.9, SECTION_C[1] + 0.45, 0.0]),
            color=P_CYAN, bend=0.6, n_ribbons=2, spread=0.16,
        )
        rays = VGroup(*[
            radiation_waves(
                np.array([sec["wall_r"] + 0.1, SECTION_C[1] + dy, 0.0]),
                n=1, color=P_YELLOW, height=1.7, x_spread=0.0, stroke_width=2.2,
            ).rotate(-PI / 2, about_point=np.array([sec["wall_r"] + 0.1, SECTION_C[1] + dy, 0.0]))
            for dy in (-0.75, 0.55)
        ])

        eq, items = equation_row([
            ("tot", "Q̇_ges", P_WHITE), (None, "=", P_WHITE),
            ("k", "Q̇_k", P_RED), (None, "+", P_WHITE),
            ("c", "Q̇_c", P_CYAN), (None, "+", P_WHITE),
            ("r", "Q̇_r", P_YELLOW),
            (None, "  [W]", P_TEAL),
        ])
        eq, eq_box = formula_panel(eq)

        # The free centre slot of the temperature row. Above the section it collided
        # with the beat subtitle, and below it there is only the formula panel.
        iso = chip("DIN EN ISO 6946", P_TEAL, font_size=LABEL_FONT_SIZE)
        iso.move_to(sec["dth"])

        hold_for(self, self.NARRATION, "recap", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(FadeIn(sec["warm"]), FadeIn(sec["cold"]), Create(sec["wall"]), run_time=1.2)
        self.play(FadeIn(sec["ti"]), FadeIn(sec["te"]), run_time=0.6)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sum"))
        self.play(
            FadeIn(lattice),
            LaggedStart(*[Create(r) for r in stream], lag_ratio=0.2),
            LaggedStart(*[Create(w) for w in rays], lag_ratio=0.2),
            run_time=1.8,
        )
        self.play(FadeIn(eq), Create(eq_box), run_time=1.1)
        hold_for(self, self.NARRATION, "sum", used=1.8 + 1.1 + 0.35)

        for key, visual, color in (
            ("k", lattice, P_RED), ("c", stream, P_CYAN), ("r", rays, P_YELLOW),
        ):
            ring = highlight_param(items, key, color=color)
            self.play(Create(ring), Indicate(visual, color=color, scale_factor=1.08), run_time=0.6)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            hold_for(self, self.NARRATION, key, used=0.6 + 0.35)
            self.play(FadeOut(ring), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "standard"))
        self.play(FadeIn(iso, shift=DOWN * 0.12), run_time=1.0)
        self.play(Indicate(items["tot"], color=P_TEAL, scale_factor=1.15), run_time=0.9)
        hold_for(self, self.NARRATION, "standard", used=1.0 + 0.9 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat6 – Three mechanisms collapse into one building-element number
class Beat6_VonWegenZuZahlen(Scene):
    NARRATION = [
        ("bridge",
         "So far we have three separate mechanisms. No engineer sizes a heating system that way.",
         "Bisher haben wir drei getrennte Mechanismen. So legt kein Ingenieur eine Heizung aus."),
        ("merge",
         "For any real building element the three are measured together and collapsed into one number that describes the whole construction.",
         "Für jedes reale Bauteil werden die drei gemeinsam erfasst und zu einer Zahl zusammengefasst, die den ganzen Aufbau beschreibt."),
        ("layers",
         "And a real wall is never one material. Plaster, masonry, insulation and render each resist the heat flow differently.",
         "Eine echte Wand ist nie ein Material: Putz, Mauerwerk, Dämmung und Außenputz bremsen den Wärmestrom unterschiedlich stark."),
        ("plan",
         "So the plan is two steps: first add up how strongly the layers resist, then turn that resistance into the number we actually use.",
         "Der Plan hat zwei Schritte: erst addieren, wie stark die Schichten bremsen, dann diesen Widerstand in die Kennzahl umrechnen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Abschnitt 1.2 — Von drei Wegen zu einem Kennwert", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "bridge"))
        self.play(FadeIn(caption), run_time=0.3)

        tokens = VGroup(
            symbol_token("Q̇_k", color=P_RED, font_size=FORMULA_FONT_SIZE),
            symbol_token("Q̇_c", color=P_CYAN, font_size=FORMULA_FONT_SIZE),
            symbol_token("Q̇_r", color=P_YELLOW, font_size=FORMULA_FONT_SIZE),
        ).arrange(DOWN, buff=0.62)
        tokens.move_to(np.array([-4.9, 1.05, 0.0]))

        funnel = VGroup(*[
            _flow_arrow(
                tok.get_right() + RIGHT * 0.18,
                np.array([-1.85, 1.05, 0.0]),
                P_WHITE, width=3,
            )
            for tok in tokens
        ])
        one = chip("ein Bauteil-Kennwert", P_TEAL)
        one.move_to(np.array([-0.05, 1.05, 0.0]))

        stack = VGroup(
            _layer(0.34, 1.9, P_WHITE, 0.20, "Putz"),
            _layer(1.05, 1.9, P_ORANGE, 0.22, "Mauerwerk"),
            _layer(0.95, 1.9, P_CYAN, 0.22, "Dämmung"),
            _layer(0.26, 1.9, P_WHITE, 0.20, "Außenputz"),
        ).arrange(RIGHT, buff=0.0, aligned_edge=UP)
        stack.move_to(np.array([2.55, -0.35, 0.0]))
        # A legend beside the stack, not a tag under each column: the thinnest
        # column (0.26 units) can never fit "Außenputz" (≈1.2 units) beneath
        # itself without overlapping its neighbour.
        legend = _layer_legend([
            ("Putz", P_WHITE), ("Mauerwerk", P_ORANGE),
            ("Dämmung", P_CYAN), ("Außenputz", P_WHITE),
        ])
        legend.next_to(stack, DOWN, buff=0.30).set_x(stack.get_x())

        steps = VGroup(
            chip("1 Schichten addieren → R", P_YELLOW, font_size=LABEL_FONT_SIZE),
            chip("2 R umrechnen → U", P_ORANGE, font_size=LABEL_FONT_SIZE),
        ).arrange(DOWN, buff=0.24)
        steps.move_to(np.array([-3.85, -1.60, 0.0]))

        fit_band(VGroup(tokens, funnel, one, stack, legend, steps))

        hold_for(self, self.NARRATION, "bridge", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(LaggedStart(*[FadeIn(t, shift=RIGHT * 0.2) for t in tokens], lag_ratio=0.18), run_time=1.3)
        hold_for(self, self.NARRATION, "bridge", used=1.3 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "merge"))
        self.play(LaggedStart(*[GrowArrow(a) for a in funnel], lag_ratio=0.15), run_time=1.2)
        self.play(FadeIn(one, scale=0.85), run_time=0.9)
        hold_for(self, self.NARRATION, "merge", used=1.2 + 0.9 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "layers"))
        self.play(LaggedStart(*[FadeIn(l, shift=DOWN * 0.15) for l in stack], lag_ratio=0.2), run_time=1.8)
        self.play(FadeIn(legend, shift=DOWN * 0.1), run_time=0.7)
        hold_for(self, self.NARRATION, "layers", used=1.8 + 0.7 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "plan"))
        self.play(LaggedStart(*[FadeIn(s, shift=UP * 0.12) for s in steps], lag_ratio=0.25), run_time=1.4)
        hold_for(self, self.NARRATION, "plan", used=1.4 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat7 – Thermal resistance R = d / lambda
class Beat7_Waermedurchlasswiderstand(Scene):
    NARRATION = [
        ("intro",
         "Step one: how strongly does a single layer resist the heat flow? Watch the temperature fall across it.",
         "Schritt eins: wie stark bremst eine einzelne Schicht den Wärmestrom? Sehen wir, wie die Temperatur über sie abfällt."),
        ("gradient",
         "A poor layer drops the temperature steeply over a short distance, and a lot of heat gets through.",
         "Eine schlechte Schicht lässt die Temperatur steil über eine kurze Strecke fallen — und viel Wärme kommt durch."),
        ("formula",
         "That resisting power is the thermal resistance R: the thickness d divided by the conductivity lambda.",
         "Diese Bremswirkung ist der Wärmedurchlasswiderstand R: die Dicke d geteilt durch die Leitfähigkeit Lambda."),
        ("d",
         "Make the layer thicker and the same temperature drop is spread over more distance. R rises, and the heat flow falls.",
         "Wird die Schicht dicker, verteilt sich derselbe Temperaturabfall auf mehr Strecke. R steigt, der Wärmestrom sinkt."),
        ("lam",
         "Lambda is the material itself. Swapping masonry for insulation cuts lambda by a factor of sixty, so R jumps even at the same thickness.",
         "Lambda ist das Material selbst. Mauerwerk gegen Dämmung senkt Lambda um das Sechzigfache — R springt hoch, bei gleicher Dicke."),
        ("sum",
         "A real construction just adds the layers up: R total is the sum of every layer's own resistance.",
         "Ein realer Aufbau addiert einfach: R-gesamt ist die Summe der Widerstände aller Schichten."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmedurchlasswiderstand R = d / λ", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        y_hi, y_lo = 1.72, 0.12
        layer_c = np.array([-2.60, 0.92, 0.0])
        layer_w, layer_h = 1.60, 2.30

        layer = Rectangle(
            width=layer_w, height=layer_h, color=P_ORANGE, stroke_width=2.5,
            fill_color=P_ORANGE, fill_opacity=0.20,
        ).move_to(layer_c)
        mat = Text("Mauerwerk", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        mat.next_to(layer, UP, buff=0.16)
        # Low in the layer: the temperature gradient runs corner to corner through
        # the centre, so a centred label is always crossed by it.
        lam_tag = Text("λ = 2,1", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        lam_tag.move_to(layer.get_center() + DOWN * 0.72)

        warm_face = Line(
            np.array([layer_c[0] - layer_w / 2 - 1.5, y_hi, 0.0]),
            np.array([layer_c[0] - layer_w / 2, y_hi, 0.0]),
            color=P_ORANGE, stroke_width=3,
        )
        cold_face = Line(
            np.array([layer_c[0] + layer_w / 2, y_lo, 0.0]),
            np.array([layer_c[0] + layer_w / 2 + 1.5, y_lo, 0.0]),
            color=P_BLUE, stroke_width=3,
        )
        grad = Line(
            np.array([layer_c[0] - layer_w / 2, y_hi, 0.0]),
            np.array([layer_c[0] + layer_w / 2, y_lo, 0.0]),
            color=P_YELLOW, stroke_width=4,
        )
        ti = Text("20 °C", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        ti.next_to(warm_face.get_start(), UP, buff=0.12)
        te = Text("0 °C", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        te.next_to(cold_face.get_end(), DOWN, buff=0.12)

        d_brace = dim_arrow(
            np.array([layer_c[0] - layer_w / 2, layer_c[1] - layer_h / 2 - 0.30, 0.0]),
            np.array([layer_c[0] + layer_w / 2, layer_c[1] - layer_h / 2 - 0.30, 0.0]),
            color=P_CYAN,
        )
        d_lbl = Text("d", font_size=BODY_FONT_SIZE, color=P_CYAN)
        d_lbl.next_to(d_brace, DOWN, buff=0.10)

        r_gauge = meter("R", length=2.3, thickness=0.52, color=P_YELLOW)
        r_gauge["group"].move_to(np.array([2.05, 0.75, 0.0]))
        r_val = ValueTracker(0.18)
        bind_meter(r_gauge, r_val)
        r_low = Text("R ≈ 0,3", font_size=BODY_FONT_SIZE, color=P_YELLOW)
        r_low.next_to(r_gauge["track"], DOWN, buff=0.24)
        r_mid = Text("R ≈ 0,6", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(r_low)
        r_high = Text("R ≈ 8,6", font_size=BODY_FONT_SIZE, color=P_YELLOW).move_to(r_low)

        q_gauge = meter("Wärmestrom", length=2.3, thickness=0.52, color=P_RED)
        q_gauge["group"].move_to(np.array([4.35, 0.75, 0.0]))
        q_val = ValueTracker(0.90)
        bind_meter(q_gauge, q_val)

        eq, items = equation_row([
            ("r", "R", P_YELLOW), (None, "=", P_WHITE),
            ("d", "d", P_CYAN), (None, "/", P_WHITE),
            ("lam", "λ", P_ORANGE),
            (None, "  [m²·K/W]", P_TEAL),
        ])
        eq, eq_box = formula_panel(eq, color=P_YELLOW)

        sum_note = Text("R_ges = R₁ + R₂ + R₃ + …", font_size=BODY_FONT_SIZE, color=P_TEAL)
        sum_note.move_to(np.array([0.0, -1.28, 0.0]))

        fit_band(
            VGroup(layer, mat, warm_face, cold_face, ti, te, d_brace, d_lbl),
            bottom=SAFE_BOTTOM_FORMULA,
        )

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(layer), FadeIn(mat), FadeIn(lam_tag), run_time=1.1)
        self.play(Create(warm_face), Create(cold_face), FadeIn(ti), FadeIn(te), run_time=1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gradient"))
        self.play(Create(grad), run_time=1.0)
        self.play(FadeIn(r_gauge["group"]), FadeIn(q_gauge["group"]), FadeIn(r_low), run_time=1.0)
        hold_for(self, self.NARRATION, "gradient", used=1.0 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(Create(d_brace), FadeIn(d_lbl), run_time=0.8)
        self.play(FadeIn(eq), Create(eq_box), run_time=1.1)
        hold_for(self, self.NARRATION, "formula", used=0.8 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "d"))
        ring_d = highlight_param(items, "d", color=P_CYAN)
        wide_w = layer_w * 2.0
        wide_layer = Rectangle(
            width=wide_w, height=layer_h, color=P_ORANGE, stroke_width=2.5,
            fill_color=P_ORANGE, fill_opacity=0.20,
        ).move_to(np.array([layer_c[0] + (wide_w - layer_w) / 2, layer_c[1], 0.0]))
        wide_grad = Line(
            np.array([layer_c[0] - layer_w / 2, y_hi, 0.0]),
            np.array([layer_c[0] - layer_w / 2 + wide_w, y_lo, 0.0]),
            color=P_YELLOW, stroke_width=4,
        )
        wide_brace = dim_arrow(
            np.array([layer_c[0] - layer_w / 2, layer_c[1] - layer_h / 2 - 0.30, 0.0]),
            np.array([layer_c[0] - layer_w / 2 + wide_w, layer_c[1] - layer_h / 2 - 0.30, 0.0]),
            color=P_CYAN,
        )
        self.play(Create(ring_d), run_time=0.4)
        self.play(
            Transform(layer, wide_layer), Transform(grad, wide_grad),
            Transform(d_brace, wide_brace),
            lam_tag.animate.move_to(wide_layer.get_center() + DOWN * 0.72),
            cold_face.animate.shift(RIGHT * (wide_w - layer_w)),
            te.animate.shift(RIGHT * (wide_w - layer_w)),
            d_lbl.animate.shift(RIGHT * (wide_w - layer_w) / 2),
            r_val.animate.set_value(0.36), q_val.animate.set_value(0.62),
            ReplacementTransform(r_low, r_mid),
            run_time=1.8,
        )
        hold_for(self, self.NARRATION, "d", used=0.4 + 1.8 + 0.35)
        self.play(FadeOut(ring_d), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "lam"))
        ring_l = highlight_param(items, "lam", color=P_ORANGE)
        ins_tag = Text("Dämmung", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(mat)
        ins_lam = Text("λ = 0,035", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(lam_tag)
        # A single homogeneous layer always carries the full 20 → 0 °C drop as one
        # straight corner-to-corner line, whatever λ is: the endpoints are fixed by
        # the two surface temperatures. Lower λ does not bend that profile — it
        # raises R and collapses the heat flow. So the gradient keeps its shape
        # (and stays joined to warm_face on the left and cold_face on the right);
        # only the material, the λ label and the two gauges change.
        self.play(Create(ring_l), run_time=0.4)
        self.play(
            layer.animate.set_color(P_CYAN).set_fill(P_CYAN, opacity=0.20),
            ReplacementTransform(mat, ins_tag), ReplacementTransform(lam_tag, ins_lam),
            r_val.animate.set_value(0.95), q_val.animate.set_value(0.10),
            ReplacementTransform(r_mid, r_high),
            run_time=1.9,
        )
        self.play(
            Indicate(grad, color=P_YELLOW, scale_factor=1.0),
            Indicate(q_gauge["track"], color=P_RED, scale_factor=1.05),
            run_time=0.7,
        )
        hold_for(self, self.NARRATION, "lam", used=0.4 + 1.9 + 0.7 + 0.35)
        self.play(FadeOut(ring_l), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "sum"))
        self.play(FadeIn(sum_note, shift=UP * 0.12), run_time=1.0)
        hold_for(self, self.NARRATION, "sum", used=1.0 + 0.35)

        r_gauge["fill"].clear_updaters()
        q_gauge["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat8 – U = 1 / R_ges, the number the standard actually reports
class Beat8_UWert(Scene):
    NARRATION = [
        ("intro",
         "Step two. Resistance is useful, but engineers quote the opposite: how easily heat gets through.",
         "Schritt zwei. Der Widerstand ist nützlich, aber angegeben wird das Gegenteil: wie leicht Wärme hindurchkommt."),
        ("total",
         "First the full resistance of the element: the two thin air films on the surfaces, plus every layer in between.",
         "Zuerst der gesamte Widerstand des Bauteils: die beiden dünnen Luftschichten an den Oberflächen plus alle Schichten dazwischen."),
        ("flip",
         "Invert that total and you get the U value: the watts crossing one square metre for each kelvin of temperature difference.",
         "Kehrt man diese Summe um, erhält man den U-Wert: die Watt, die pro Quadratmeter und pro Kelvin Unterschied hindurchgehen."),
        ("old",
         "An uninsulated solid wall from nineteen-sixty has a U around one point four. It leaks badly.",
         "Eine ungedämmte Massivwand von 1960 hat ein U um 1,4 — sie ist ein Sieb."),
        ("new",
         "The same wall with twenty centimetres of insulation reaches about zero point one five: roughly ten times tighter.",
         "Dieselbe Wand mit zwanzig Zentimetern Dämmung erreicht etwa 0,15 — rund zehnmal dichter."),
        ("meaning",
         "Low U is the goal. Every part of the building envelope is judged by this one number.",
         "Ein kleines U ist das Ziel. Jedes Bauteil der Gebäudehülle wird an dieser einen Zahl gemessen."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("U-Wert — wie leicht Wärme hindurchgeht", title)
        din = _din_ref("DIN EN ISO 6946")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        def _wall(center, layers, tag, tag_color):
            group = VGroup(*[
                Rectangle(
                    width=w, height=1.85, color=c, stroke_width=2,
                    fill_color=c, fill_opacity=0.22,
                )
                for w, c in layers
            ]).arrange(RIGHT, buff=0.0)
            group.move_to(center)
            label = Text(tag, font_size=LABEL_FONT_SIZE, color=tag_color)
            label.next_to(group, UP, buff=0.18)
            return VGroup(group, label), group

        old_card, old_stack = _wall(
            np.array([-3.55, 0.95, 0.0]),
            ((0.22, P_WHITE), (1.55, P_ORANGE), (0.20, P_WHITE)),
            "Altbau 1960 — ungedämmt", P_RED,
        )
        new_card, new_stack = _wall(
            np.array([3.55, 0.95, 0.0]),
            ((0.22, P_WHITE), (1.30, P_ORANGE), (1.05, P_CYAN), (0.20, P_WHITE)),
            "saniert — 20 cm Dämmung", P_CYAN,
        )

        old_u = Text("U ≈ 1,4 W/(m²·K)", font_size=BODY_FONT_SIZE, color=P_RED)
        old_u.next_to(old_stack, DOWN, buff=0.30)
        new_u = Text("U ≈ 0,15 W/(m²·K)", font_size=BODY_FONT_SIZE, color=P_CYAN)
        new_u.next_to(new_stack, DOWN, buff=0.30)

        old_leak = VGroup(*[
            _flow_arrow(
                old_stack.get_right() + RIGHT * 0.06 + UP * dy,
                old_stack.get_right() + RIGHT * 1.05 + UP * dy,
                P_RED, width=3,
            )
            for dy in (-0.62, -0.21, 0.20, 0.61)
        ])
        new_leak = VGroup(
            _flow_arrow(
                new_stack.get_right() + RIGHT * 0.06,
                new_stack.get_right() + RIGHT * 0.55,
                P_CYAN, width=3,
            )
        )

        films = VGroup(
            Text("R_si", font_size=LABEL_FONT_SIZE, color=P_TEAL).next_to(old_stack, LEFT, buff=0.14),
            Text("R_se", font_size=LABEL_FONT_SIZE, color=P_TEAL).next_to(old_stack, RIGHT, buff=0.14),
        )

        fit_band(VGroup(old_card, new_card, old_u, new_u), bottom=SAFE_BOTTOM_FORMULA)

        r_eq, r_items = equation_row([
            ("rges", "R_ges", P_TEAL), (None, "=", P_WHITE),
            ("rsi", "R_si", P_GREEN), (None, "+", P_WHITE),
            (None, "Σ d/λ", P_YELLOW), (None, "+", P_WHITE),
            ("rse", "R_se", P_GREEN),
        ])
        r_eq, r_box = formula_panel(r_eq, color=P_TEAL)

        u_eq, u_items = equation_row([
            ("u", "U", P_ORANGE), (None, "=", P_WHITE),
            (None, "1", P_WHITE), (None, "/", P_WHITE),
            ("rges", "R_ges", P_TEAL),
            (None, "  [W/(m²·K)]", P_TEAL),
        ])

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(FadeIn(old_card), run_time=1.0)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "total"))
        self.play(FadeIn(films), run_time=0.7)
        self.play(FadeIn(r_eq), Create(r_box), run_time=1.1)
        hold_for(self, self.NARRATION, "total", used=0.7 + 1.1 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flip"))
        u_eq, u_box = formula_panel(u_eq, color=P_ORANGE)
        self.play(
            ReplacementTransform(r_eq, u_eq), ReplacementTransform(r_box, u_box),
            FadeOut(films),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "flip", used=1.3 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "old"))
        self.play(FadeIn(old_u), run_time=0.6)
        self.play(LaggedStart(*[GrowArrow(a) for a in old_leak], lag_ratio=0.12), run_time=1.2)
        hold_for(self, self.NARRATION, "old", used=0.6 + 1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "new"))
        self.play(FadeIn(new_card), run_time=1.0)
        self.play(FadeIn(new_u), LaggedStart(*[GrowArrow(a) for a in new_leak], lag_ratio=0.12), run_time=1.0)
        hold_for(self, self.NARRATION, "new", used=1.0 + 1.0 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "meaning"))
        ring = highlight_param(u_items, "u", color=P_ORANGE)
        self.play(Create(ring), Indicate(new_u, color=P_CYAN, scale_factor=1.12), run_time=1.0)
        hold_for(self, self.NARRATION, "meaning", used=1.0 + 0.35)

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion


#region Beat9 – Q = U · A · Δθ, and what each factor actually does
class Beat9_WaermestromFormel(Scene):
    NARRATION = [
        ("intro",
         "Now we can finally count watts. The heat flow through any building element takes three factors.",
         "Jetzt können wir endlich Watt zählen. Der Wärmestrom durch ein Bauteil braucht drei Faktoren."),
        ("formula",
         "Heat flow equals the U value, times the area, times the temperature difference.",
         "Wärmestrom gleich U-Wert mal Fläche mal Temperaturdifferenz."),
        ("u",
         "U is the quality of the construction — the one number we just built out of the layers.",
         "U ist die Qualität des Aufbaus — die Zahl, die wir gerade aus den Schichten gebaut haben."),
        ("a",
         "A is how much of that construction there is. Double the wall area and you double the loss.",
         "A ist, wie viel von diesem Aufbau vorhanden ist. Doppelte Wandfläche, doppelter Verlust."),
        ("dt",
         "Delta theta is the driver from the very first beat. Double the temperature difference and the loss doubles again.",
         "Δθ ist der Antrieb aus dem ersten Beat. Doppelte Temperaturdifferenz, wieder doppelter Verlust."),
        ("power",
         "Only U is a design decision. That is why insulation is the lever: it is the one factor an engineer actually chooses.",
         "Nur U ist eine Entwurfsentscheidung. Deshalb ist Dämmung der Hebel — der einzige Faktor, den man wirklich wählt."),
        ("outro",
         "For this wall the answer is about two thousand watts, the same as a full-power electric heater running non-stop.",
         "Für diese Wand sind das rund zweitausend Watt — so viel wie ein Heizlüfter auf voller Stufe im Dauerbetrieb."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmestrom Q̇ = U · A · Δθ", title)
        din = _din_ref("DIN EN 12831-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        facade_c = np.array([-4.15, 0.85, 0.0])
        facade = Rectangle(
            width=2.9, height=2.1, color=P_WHITE, stroke_width=2.5,
            fill_color=P_TEAL, fill_opacity=0.12,
        ).move_to(facade_c)
        a_lbl = Text("A", font_size=FORMULA_FONT_SIZE, color=P_CYAN).move_to(facade_c)
        ti = Text("innen 20 °C", font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        ti.next_to(facade, UP, buff=0.16)
        te = Text("außen 0 °C", font_size=LABEL_FONT_SIZE, color=P_BLUE)
        te.next_to(facade, DOWN, buff=0.20)

        q_gauge = meter("Wärmestrom", length=2.5, thickness=0.55, color=P_RED)
        # Sits clear of the facade even when the "A" beat stretches it to 1.9×
        # its width and nudges it right — at full stretch the facade's right edge
        # reaches ≈ -0.75, so the gauge (and its label) start well to the right of
        # that.
        q_gauge["group"].move_to(np.array([0.20, 0.72, 0.0]))
        q_val = ValueTracker(0.0)
        bind_meter(q_gauge, q_val)

        w_base = Text("≈ 2000 W", font_size=BODY_FONT_SIZE, color=P_RED)
        w_base.next_to(q_gauge["track"], DOWN, buff=0.24)
        w_double = Text("≈ 4000 W", font_size=BODY_FONT_SIZE, color=P_RED).move_to(w_base)
        w_low = Text("≈ 210 W", font_size=BODY_FONT_SIZE, color=P_GREEN).move_to(w_base)

        eq, items = equation_row([
            ("q", "Q̇", P_WHITE), (None, "=", P_WHITE),
            ("u", "U", P_ORANGE), (None, "·", P_WHITE),
            ("a", "A", P_CYAN), (None, "·", P_WHITE),
            ("dt", "Δθ", P_BLUE),
            (None, "  [W]", P_TEAL),
        ])
        eq, eq_box = formula_panel(eq)

        fit_band(VGroup(facade, a_lbl, ti, te), bottom=SAFE_BOTTOM_FORMULA)

        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3)

        self.play(Create(facade), FadeIn(a_lbl), FadeIn(ti), FadeIn(te), run_time=1.4)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(FadeIn(eq), Create(eq_box), run_time=1.1)
        self.play(FadeIn(q_gauge["group"]), q_val.animate.set_value(0.45), FadeIn(w_base), run_time=1.2)
        hold_for(self, self.NARRATION, "formula", used=1.1 + 1.2 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "u"))
        ring_u = highlight_param(items, "u", color=P_ORANGE)
        self.play(Create(ring_u), Indicate(facade, color=P_ORANGE), run_time=0.9)
        hold_for(self, self.NARRATION, "u", used=0.9 + 0.35)
        self.play(FadeOut(ring_u), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "a"))
        ring_a = highlight_param(items, "a", color=P_CYAN)
        self.play(Create(ring_a), run_time=0.4)
        self.play(
            facade.animate.stretch_to_fit_width(facade.width * 1.9).move_to(facade_c + RIGHT * 0.65),
            a_lbl.animate.move_to(facade_c + RIGHT * 0.65),
            q_val.animate.set_value(0.88),
            ReplacementTransform(w_base, w_double),
            run_time=1.5,
        )
        hold_for(self, self.NARRATION, "a", used=0.4 + 1.5 + 0.35)
        self.play(
            facade.animate.stretch_to_fit_width(2.9).move_to(facade_c),
            a_lbl.animate.move_to(facade_c),
            q_val.animate.set_value(0.45),
            ReplacementTransform(w_double, w_base),
            FadeOut(ring_a),
            run_time=1.0,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "dt"))
        ring_dt = highlight_param(items, "dt", color=P_BLUE)
        te_cold = Text("außen −20 °C", font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(te)
        te_back = Text("außen 0 °C", font_size=LABEL_FONT_SIZE, color=P_BLUE).move_to(te)
        w_double2 = Text("≈ 4000 W", font_size=BODY_FONT_SIZE, color=P_RED).move_to(w_base)
        w_back = Text("≈ 2000 W", font_size=BODY_FONT_SIZE, color=P_RED).move_to(w_base)
        self.play(Create(ring_dt), run_time=0.4)
        self.play(
            ReplacementTransform(te, te_cold),
            q_val.animate.set_value(0.88),
            ReplacementTransform(w_base, w_double2),
            run_time=1.4,
        )
        hold_for(self, self.NARRATION, "dt", used=0.4 + 1.4 + 0.35)
        self.play(
            ReplacementTransform(te_cold, te_back),
            q_val.animate.set_value(0.45),
            ReplacementTransform(w_double2, w_back),
            FadeOut(ring_dt),
            run_time=1.0,
        )

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "power"))
        # Under the facade, not under the gauge — the gauge's watt readout lives there.
        lever = chip("nur U ist wählbar → dämmen", P_GREEN, font_size=LABEL_FONT_SIZE)
        lever.move_to(np.array([facade_c[0], -1.18, 0.0]))
        self.play(
            Indicate(items["u"], color=P_GREEN, scale_factor=1.25),
            FadeIn(lever, shift=UP * 0.12),
            run_time=1.1,
        )
        self.play(
            facade.animate.set_fill(P_CYAN, opacity=0.22),
            q_val.animate.set_value(0.06),
            ReplacementTransform(w_back, w_low),
            run_time=1.5,
        )
        hold_for(self, self.NARRATION, "power", used=1.1 + 1.5 + 0.35)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        anchor = watt_anchor(2000, compare="heater", title="ungedämmt")
        anchor.scale(0.68).move_to(np.array([4.35, 0.60, 0.0]))
        self.play(FadeIn(anchor, shift=LEFT * 0.15), run_time=1.1)
        hold_for(self, self.NARRATION, "outro", used=1.1 + 0.35)

        q_gauge["fill"].clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
