import os
import subprocess

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
from manim_fonts import apply_body_font, play_scene_title, scene_title


# ─── LaTeX Bootstrap ─────────────────────────────────────────────────
def _ensure_latex_paths():
    """🔧 Point dvisvgm at the TeX tree so ``MathTex`` works.

    Homebrew's dvisvgm ships its own kpathsea that cannot locate ``texmf.cnf``
    in a Homebrew TeX Live, which surfaces as a misleading "update dvisvgm to at
    least version 2.4" error. Values are derived from ``kpsewhich``, so this is a
    no-op wherever the environment is already wired correctly.
    """
    if os.environ.get("TEXMFROOT"):
        return
    try:
        root = subprocess.run(
            ["kpsewhich", "-var-value=TEXMFROOT"],
            capture_output=True, text=True, timeout=15,
        ).stdout.strip()
    except (OSError, subprocess.SubprocessError):
        return
    if not root or not _Path(root).is_dir():
        return
    dist = _Path(root) / "texmf-dist"
    os.environ.setdefault("TEXMFROOT", root)
    os.environ.setdefault("TEXMFDIST", str(dist))
    os.environ.setdefault("TEXMFCNF", str(dist / "web2c"))


_ensure_latex_paths()


# ─── Shared Constants ────────────────────────────────────────────────
P_DEEP_DARK = "#0B0C10"
P_WHITE     = "#E0E6ED"
P_CYAN      = "#66FCF1"
P_TEAL      = "#45A29E"
P_ORANGE    = "#FFAAA5"
P_YELLOW    = "#FFE66D"
P_RED       = "#FF6B6B"
P_BLUE      = "#4D96FF"
P_GREEN     = "#CAFFBF"
P_MUTED     = "#8B95A5"

MAIN_TITLE = "Lüftungssysteme: Arten und Wirkprinzipien"


def _main_title():
    """🌀 Persistent chapter header shared by every beat of this topic."""
    return scene_title(MAIN_TITLE)


def _subtitle(text, color=P_TEAL):
    """🏷️ Beat headline placed directly under the chapter header."""
    return Text(text, font_size=21, color=color).move_to(UP * 3.0)


def _step_label(text, color=P_WHITE):
    """📌 Bottom-centered instructional caption for pacing a longer beat."""
    return Text(text, font_size=19, color=color).to_edge(DOWN, buff=0.22)


def _equation(*parts, font_size=42, colors=None):
    """🧮 Typeset an equation with ``MathTex`` — each argument stays addressable.

    Passing terms separately keeps every fragment indexable for colouring and as
    an ``Indicate`` / ``Transform`` target, with real mathematical typesetting.

    https://docs.manim.community/en/stable/reference/manim.mobject.text.tex_mobject.MathTex.html
    """
    eq = MathTex(*parts, font_size=font_size)
    eq.set_color(P_WHITE)
    for index, color in (colors or {}).items():
        eq[index].set_color(color)
    return eq


def _node(text, color, width=3.1, height=0.58, font_size=19):
    """🔲 Rounded taxonomy node with centered caption."""
    box = RoundedRectangle(
        width=width, height=height, corner_radius=0.09,
        color=color, stroke_width=2.2,
        fill_color=color, fill_opacity=0.12,
    )
    label = Text(text, font_size=font_size, color=color).move_to(box)
    return VGroup(box, label)


def _elbow(start, end, color, stroke_width=2.0):
    """🪝 Two-segment connector that drops vertically then runs horizontally."""
    knee = np.array([start[0], end[1], 0.0])
    line = VMobject(color=color, stroke_width=stroke_width)
    line.set_points_as_corners([start, knee, end])
    line.set_stroke(opacity=0.55)
    return line


def _room(center, width, height, color=P_WHITE):
    """🏠 Line-art room shell with a teal floor slab."""
    shell = Rectangle(
        width=width, height=height,
        color=color, stroke_width=3.0, fill_opacity=0,
    ).move_to(center)
    floor = Line(shell.get_corner(DL), shell.get_corner(DR), color=P_TEAL, stroke_width=4)
    return VGroup(shell, floor)


def _fan(pos, color, radius=0.26):
    """🌀 Compact fan glyph — hub plus three blades."""
    ring = Circle(radius=radius, color=color, stroke_width=2.4)
    blades = VGroup(*[
        Line(ORIGIN, RIGHT * radius * 0.78, color=color, stroke_width=2.4).rotate(
            a, about_point=ORIGIN
        )
        for a in (0.0, TAU / 3, 2 * TAU / 3)
    ])
    return VGroup(ring, blades).move_to(pos)


def _person(pos, color=P_ORANGE, scale=1.0):
    """🧍 Minimal seated-occupant glyph used as an internal heat source."""
    head = Circle(radius=0.12, color=color, stroke_width=2.2)
    body = RoundedRectangle(
        width=0.32, height=0.44, corner_radius=0.1,
        color=color, stroke_width=2.2,
    ).next_to(head, DOWN, buff=0.04)
    return VGroup(head, body).scale(scale).move_to(pos)


def _plume(base, height, color, width=0.5, layers=4):
    """♨️ Rising convective plume drawn as stacked, widening arcs."""
    plume = VGroup()
    for i in range(layers):
        f = (i + 1) / layers
        arc = Arc(
            radius=width * f, start_angle=PI * 0.15, angle=PI * 0.7,
            color=color, stroke_width=2.2,
        )
        arc.set_stroke(opacity=0.75 - 0.13 * i)
        arc.move_to(base + UP * (height * f))
        plume.add(arc)
    return plume


# region Airflow
def _smooth_path(points):
    """〰️ Smooth polyline path for airflow particles.

    ``TipableVMobject`` rather than a bare ``VMobject`` so ``_guides`` can put an
    arrowhead on a copy of the same curve.
    """
    path = TipableVMobject()
    path.set_points_smoothly([np.array(p, dtype=float) for p in points])
    return path


def _guides(paths, color, opacity=0.25, width=2.0, tips=True):
    """🧭 Faint streamlines so the route of the air stays readable between particles."""
    lines = VGroup()
    for path in paths:
        guide = path.copy().set_stroke(color=color, width=width, opacity=opacity)
        guide.set_fill(opacity=0)
        if tips:
            guide.add_tip(tip_length=0.17, tip_width=0.14)
            guide.tip.set_fill(color=color, opacity=opacity + 0.3)
            guide.tip.set_stroke(width=0)
        lines.add(guide)
    return lines


def _flow(
    scene, paths, color, run_time=3.2, waves=3, radius=0.075,
    cycles=2.0, color_end=None, extra=None,
):
    """💨 Continuous particle stream — several waves per path, looping and fading at both ends.

    ``color_end`` lets a particle change colour along its route, which is how the
    beats show air picking up or giving off heat while it crosses a room.
    """
    dots = VGroup()
    meta = []
    for path in paths:
        for w in range(waves):
            dot = Dot(radius=radius, color=color, stroke_width=0)
            dot.set_fill(color, opacity=0.0)
            dot.move_to(path.point_from_proportion(0.0))
            dots.add(dot)
            meta.append((path, w / waves))

    start_c = ManimColor(color)
    end_c = ManimColor(color_end) if color_end else start_c

    def update(group, alpha):
        for dot, (path, offset) in zip(group, meta):
            t = (alpha * cycles + offset) % 1.0
            dot.move_to(path.point_from_proportion(t))
            fade = min(1.0, t / 0.10, (1.0 - t) / 0.10)
            dot.set_fill(interpolate_color(start_c, end_c, t), opacity=max(0.0, fade))

    scene.add(dots)
    anims = [UpdateFromAlphaFunc(dots, update)]
    if extra:
        anims.extend(extra)
    scene.play(*anims, run_time=run_time, rate_func=linear)
    # play() registers each dot individually, so the group handle alone leaves them on screen.
    scene.remove(dots, *dots)


def _pad(scene, budget: float, used: float) -> None:
    """⏱️ Hold the frame so a visual phase fills its narration budget."""
    rem = float(budget) - float(used)
    if rem > 0.05:
        scene.wait(rem)
# endregion


# ═══════════════════════════════════════════════════════════════════════
# BEAT 1 – Taxonomy of ventilation systems
# ═══════════════════════════════════════════════════════════════════════
class Beat1_Systemuebersicht(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = _main_title()
        sub = _subtitle("Wie wird Luft überhaupt bewegt?")

        root = _node("Lüftung", P_WHITE, width=2.8).move_to(UP * 2.0)
        free = _node("Freie Lüftung", P_GREEN, width=3.9).move_to(LEFT * 3.7 + UP * 0.8)
        mech = _node("Ventilatorgestützte Lüftung", P_CYAN, width=5.3).move_to(RIGHT * 3.7 + UP * 0.8)

        free_items = ["Fugenlüftung", "Fensterlüftung", "Querlüftung", "Auftriebs-/Schachtlüftung"]
        mech_items = ["Abluftanlage", "Zuluftanlage", "Zu-/Abluftanlage", "RLT-/Klimaanlage"]

        def _branch(items, color, anchor):
            """🌿 Indented leaf column hanging off a spine dropped from the branch node."""
            spine_x = anchor[0].get_left()[0] + 0.4
            leaves = VGroup(*[Text(t, font_size=18, color=color) for t in items])
            leaves.arrange(DOWN, buff=0.34, aligned_edge=LEFT)
            leaves.next_to(anchor, DOWN, buff=0.55)
            leaves.shift(RIGHT * (spine_x + 0.52 - leaves.get_left()[0]))

            spine = Line(
                np.array([spine_x, anchor[0].get_bottom()[1], 0.0]),
                np.array([spine_x, leaves[-1].get_y(), 0.0]),
                color=color, stroke_width=2.0,
            ).set_stroke(opacity=0.45)
            ticks = VGroup(*[
                Line(
                    np.array([spine_x, leaf.get_y(), 0.0]),
                    leaf.get_left() + LEFT * 0.14,
                    color=color, stroke_width=2.0,
                ).set_stroke(opacity=0.45)
                for leaf in leaves
            ])
            return leaves, spine, ticks

        free_leaves, free_spine, free_ticks = _branch(free_items, P_GREEN, free)
        mech_leaves, mech_spine, mech_ticks = _branch(mech_items, P_CYAN, mech)

        trunk = VGroup(
            _elbow(root[0].get_bottom(), free[0].get_top(), P_WHITE),
            _elbow(root[0].get_bottom(), mech[0].get_top(), P_WHITE),
        )

        driver_free = Text("Antrieb: Wind und Thermik", font_size=16, color=P_MUTED)
        driver_free.next_to(free_leaves, DOWN, buff=0.36).set_x(free.get_x())
        driver_mech = Text("Antrieb: Ventilator", font_size=16, color=P_MUTED)
        driver_mech.next_to(mech_leaves, DOWN, buff=0.36).set_x(mech.get_x())

        focus = SurroundingRectangle(
            VGroup(mech, mech_leaves), color=P_YELLOW, buff=0.28, corner_radius=0.12,
        )
        focus.set_stroke(width=2.4, opacity=0.9)
        step = _step_label("Diese Folge: die mechanischen Systeme im Detail", P_YELLOW)

        play_scene_title(self, title)
        self.play(FadeIn(sub, shift=DOWN * 0.12), run_time=0.8)
        self.play(FadeIn(root, scale=1.06), run_time=0.9)
        _pad(self, 8.96, 1.4 + 0.8 + 0.9)

        self.play(
            LaggedStart(
                Create(trunk[0]), Create(trunk[1]),
                FadeIn(free, shift=UP * 0.12), FadeIn(mech, shift=UP * 0.12),
                lag_ratio=0.35,
            ),
            run_time=2.2,
        )
        self.play(Create(free_spine), Create(mech_spine), run_time=1.0)
        _pad(self, 9.18, 2.2 + 1.0)

        self.play(
            LaggedStart(
                *[
                    AnimationGroup(Create(t), FadeIn(leaf, shift=RIGHT * 0.14))
                    for t, leaf in zip(
                        list(free_ticks) + list(mech_ticks),
                        list(free_leaves) + list(mech_leaves),
                    )
                ],
                lag_ratio=0.22,
            ),
            run_time=3.1,
        )
        _pad(self, 3.37, 3.1)

        self.play(FadeIn(driver_free), FadeIn(driver_mech), run_time=1.0)
        self.play(Create(focus), FadeIn(step), run_time=1.2)
        _pad(self, 7.43, 1.0 + 1.2)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 2 – Free ventilation and its limits
# ═══════════════════════════════════════════════════════════════════════
class Beat2_FreieLueftung(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = _main_title()
        sub = _subtitle("Freie Lüftung: Wind und Auftrieb als Antrieb", P_GREEN)

        center = DOWN * 0.6
        shell = _room(center, 8.2, 3.4)
        room, floor = shell[0], shell[1]

        left_win = Rectangle(
            width=0.24, height=1.15, color=P_CYAN, stroke_width=2.2,
            fill_color=P_DEEP_DARK, fill_opacity=1.0,
        ).move_to(room.get_left() + UP * 0.3)
        right_win = left_win.copy().move_to(room.get_right() + UP * 0.3)

        heat = Rectangle(
            width=8.1, height=3.3, stroke_width=0,
            fill_color=P_RED, fill_opacity=0.0,
        ).move_to(center)

        # region Step 1 — single-sided window ventilation
        step1 = _step_label("1  Fensterlüftung: Austausch nur über eine Öffnung", P_CYAN)
        single = VGroup(
            _smooth_path([
                room.get_left() + DOWN * 0.15,
                center + LEFT * 2.7 + DOWN * 0.55,
                center + LEFT * 1.6 + DOWN * 0.4,
            ]),
            _smooth_path([
                center + LEFT * 1.6 + UP * 0.2,
                center + LEFT * 2.7 + UP * 0.55,
                room.get_left() + UP * 0.75,
            ]),
        )
        weak = Text("geringer, unkontrollierter Luftwechsel", font_size=17, color=P_MUTED)
        weak.next_to(room, DOWN, buff=0.3)
        # endregion

        # region Step 2 — cross ventilation
        step2 = _step_label("2  Querlüftung: Durchströmung von Fassade zu Fassade", P_GREEN)
        cross = VGroup(*[
            _smooth_path([
                room.get_left() + UP * dy,
                center + LEFT * 1.4 + UP * (dy * 0.7 + 0.1),
                center + RIGHT * 1.4 + UP * (dy * 0.5 + 0.18),
                room.get_right() + UP * (dy * 0.6 + 0.3),
            ])
            for dy in (-0.2, 0.3, 0.75)
        ])
        strong = Text("hoher Luftwechsel, aber wind- und wetterabhängig", font_size=17, color=P_MUTED)
        strong.next_to(room, DOWN, buff=0.3)
        # endregion

        # region Step 3 — stack effect
        step3 = _step_label("3  Auftriebslüftung: warme Luft steigt im Schacht", P_ORANGE)
        shaft_w, shaft_h = 0.9, 1.15
        shaft_top = room.get_top()[1] + shaft_h
        shaft = VGroup(*[
            Line(
                np.array([room.get_corner(UR)[0] - dx, room.get_top()[1] - 0.3, 0.0]),
                np.array([room.get_corner(UR)[0] - dx, shaft_top, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            )
            for dx in (0.35 + shaft_w, 0.35)
        ])
        shaft_x = room.get_corner(UR)[0] - 0.35 - shaft_w / 2
        stack = VGroup(*[
            _smooth_path([
                center + LEFT * dx + DOWN * 0.9,
                center + RIGHT * (shaft_x - center[0]) * 0.55 + UP * 0.2,
                np.array([shaft_x, room.get_top()[1] + 0.2, 0.0]),
                np.array([shaft_x, shaft_top - 0.08, 0.0]),
            ])
            for dx in (2.0, 0.5, -0.9)
        ])
        stack_eq = _equation(
            r"\Delta p", "=", r"h \cdot g \cdot (\rho_a - \rho_i)",
            font_size=40, colors={0: P_ORANGE},
        )
        stack_eq.next_to(room, DOWN, buff=0.26)
        # endregion

        # region Step 4 — night purge, told as a heat balance
        step4 = _step_label("4  Nachtlüftung: die Speichermasse gibt ihre Wärme ab", P_BLUE)
        mass_tag = Text("Speichermasse", font_size=18, color=P_RED)
        mass_tag.move_to(center + DOWN * 1.15)
        mass_temp = Text("26 °C", font_size=28, color=P_RED)
        mass_temp.next_to(mass_tag, DOWN, buff=0.14)
        mass_tag_cold = Text("Speichermasse", font_size=18, color=P_BLUE).move_to(mass_tag)
        mass_temp_cold = Text("21 °C", font_size=28, color=P_BLUE).move_to(mass_temp)

        night = VGroup(*[
            _smooth_path([
                room.get_left() + UP * dy,
                center + LEFT * 2.0 + UP * (dy * 0.6 - 0.15),
                center + RIGHT * 2.0 + UP * (dy * 0.5 + 0.15),
                room.get_right() + UP * (dy * 0.55 + 0.3),
            ])
            for dy in (-0.75, -0.1, 0.55)
        ])
        in_tag = VGroup(
            Text("kühle Nachtluft", font_size=16, color=P_BLUE),
            Text("16 °C", font_size=20, color=P_BLUE),
        ).arrange(DOWN, buff=0.06)
        in_tag.next_to(room, LEFT, buff=0.26).shift(DOWN * 0.15)
        out_tag = VGroup(
            Text("erwärmte Abluft", font_size=16, color=P_ORANGE),
            Text("trägt die Wärme fort", font_size=15, color=P_MUTED),
        ).arrange(DOWN, buff=0.06)
        out_tag.next_to(room, RIGHT, buff=0.26).shift(UP * 0.25)
        # endregion

        limits = VGroup(
            Text("Grenzen der freien Lüftung", font_size=24, color=P_RED),
            Text("keine Filterung · keine Entfeuchtung · keine Wärmerückgewinnung",
                 font_size=19, color=P_MUTED),
            Text("Volumenstrom nicht regelbar — für die Kühllast nicht auslegbar",
                 font_size=19, color=P_MUTED),
        ).arrange(DOWN, buff=0.22)
        limits.move_to(center + UP * 0.1)

        self.add(heat)
        play_scene_title(self, title)
        self.play(FadeIn(sub, shift=DOWN * 0.12), run_time=0.7)
        _pad(self, 2.71, 1.4 + 0.7)

        self.play(Create(room), Create(floor), run_time=1.2)
        self.play(heat.animate.set_fill(opacity=0.12), FadeIn(left_win), run_time=1.0)
        g1 = _guides(single, P_CYAN)
        self.play(FadeIn(step1), Create(g1), run_time=0.9)
        _flow(self, single, P_CYAN, run_time=3.2, waves=3, cycles=2.0)
        self.play(FadeIn(weak), run_time=0.7)
        _pad(self, 12.13, 1.2 + 1.0 + 0.9 + 3.2 + 0.7)

        g2 = _guides(cross, P_GREEN)
        self.play(
            LaggedStart(
                AnimationGroup(FadeOut(step1, shift=DOWN * 0.25), FadeOut(weak, shift=DOWN * 0.25),
                               FadeOut(g1)),
                AnimationGroup(FadeIn(step2, shift=UP * 0.25), FadeIn(right_win), Create(g2)),
                lag_ratio=0.7,
            ),
            run_time=1.4,
        )
        _flow(self, cross, P_GREEN, run_time=3.2, waves=4, cycles=2.6)
        self.play(FadeIn(strong), run_time=0.7)
        _pad(self, 11.71, 1.4 + 3.2 + 0.7)

        g3 = _guides(stack, P_ORANGE)
        self.play(
            LaggedStart(
                AnimationGroup(FadeOut(step2, shift=DOWN * 0.25), FadeOut(strong, shift=DOWN * 0.25),
                               FadeOut(g2)),
                AnimationGroup(FadeIn(step3, shift=UP * 0.25), Create(shaft), Create(g3)),
                lag_ratio=0.7,
            ),
            run_time=1.6,
        )
        _flow(self, stack, P_ORANGE, run_time=3.2, waves=3, cycles=2.2)
        self.play(FadeIn(stack_eq), run_time=0.9)
        _pad(self, 13.47, 1.6 + 3.2 + 0.9)

        g4 = _guides(night, P_BLUE)
        self.play(
            LaggedStart(
                AnimationGroup(
                    FadeOut(step3, shift=DOWN * 0.25), FadeOut(stack_eq),
                    FadeOut(shaft), FadeOut(g3),
                ),
                AnimationGroup(FadeIn(step4, shift=UP * 0.25), FadeIn(mass_tag), FadeIn(mass_temp)),
                lag_ratio=0.7,
            ),
            run_time=1.4,
        )
        self.play(Create(g4), FadeIn(in_tag), run_time=0.9)
        _flow(self, night, P_BLUE, run_time=2.8, waves=4, cycles=2.4, color_end=P_ORANGE)
        self.play(FadeIn(out_tag), run_time=0.7)
        _flow(
            self, night, P_BLUE, run_time=3.0, waves=4, cycles=2.6, color_end=P_ORANGE,
            extra=[
                heat.animate.set_fill(P_BLUE, opacity=0.10),
                room.animate.set_stroke(color=P_BLUE),
                ReplacementTransform(mass_temp, mass_temp_cold),
                ReplacementTransform(mass_tag, mass_tag_cold),
            ],
        )
        _pad(self, 10.30, 1.4 + 0.9 + 2.8 + 0.7 + 3.0)

        self.play(
            FadeOut(VGroup(room, floor, left_win, right_win, step4, g4,
                           in_tag, out_tag, mass_tag_cold, mass_temp_cold)),
            FadeOut(heat),
            run_time=1.0,
        )
        self.play(
            LaggedStart(*[FadeIn(line, shift=UP * 0.14) for line in limits], lag_ratio=0.35),
            run_time=1.8,
        )
        _pad(self, 13.32, 1.0 + 1.8)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 3 – The three mechanical base types
# ═══════════════════════════════════════════════════════════════════════
class Beat3_MechanischeGrundtypen(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = _main_title()
        sub = _subtitle("Drei mechanische Grundtypen", P_CYAN)

        centers = [LEFT * 4.66 + DOWN * 0.45, DOWN * 0.45, RIGHT * 4.66 + DOWN * 0.45]
        rooms = VGroup(*[_room(c, 3.85, 2.15) for c in centers])

        headers = VGroup(*[
            Text(t, font_size=21, color=c).move_to(np.array([centers[i][0], 2.42, 0.0]))
            for i, (t, c) in enumerate(
                [("Abluftanlage", P_ORANGE), ("Zuluftanlage", P_CYAN), ("Zu-/Abluftanlage", P_GREEN)]
            )
        ])

        def _port(center, side, color):
            """🕳️ Passive wall opening — the uncontrolled half of a one-fan system."""
            slot = Rectangle(
                width=0.16, height=0.5, color=color, stroke_width=2.6,
                fill_color=P_DEEP_DARK, fill_opacity=1.0,
            )
            return slot.move_to(center + side * 1.925 + UP * 0.45)

        # region Panel 1 — exhaust only: untreated air seeps in, warm air is ducted out
        exh_fan = _fan(centers[0] + RIGHT * 1.35 + UP * 1.525, P_ORANGE)
        exh_duct = Line(centers[0] + RIGHT * 1.35 + UP * 1.075, exh_fan.get_bottom(),
                        color=P_ORANGE, stroke_width=5)
        exh_out = Arrow(exh_fan.get_top(), exh_fan.get_top() + UP * 0.55, color=P_ORANGE,
                        stroke_width=4, buff=0.04, max_tip_length_to_length_ratio=0.35)
        ald = _port(centers[0], LEFT, P_MUTED)
        exh_paths = VGroup(*[
            _smooth_path([
                ald.get_right(),
                centers[0] + LEFT * 0.9 + UP * (dy * 0.6),
                centers[0] + RIGHT * 0.5 + UP * (dy * 0.4 + 0.2),
                centers[0] + RIGHT * 1.35 + UP * 1.0,
            ])
            for dy in (-0.7, 0.0, 0.6)
        ])
        ald_tag = Text("Außenluft ungefiltert", font_size=15, color=P_MUTED)
        ald_tag.next_to(rooms[0], DOWN, buff=0.18)
        # endregion

        # region Panel 2 — supply only: treated air in, uncontrolled leakage out
        sup_fan = _fan(centers[1] + LEFT * 1.35 + UP * 1.525, P_CYAN)
        sup_duct = Line(centers[1] + LEFT * 1.35 + UP * 1.075, sup_fan.get_bottom(),
                        color=P_CYAN, stroke_width=5)
        sup_in = Arrow(sup_fan.get_top() + UP * 0.55, sup_fan.get_top(), color=P_CYAN,
                       stroke_width=4, buff=0.04, max_tip_length_to_length_ratio=0.35)
        leak = _port(centers[1], RIGHT, P_MUTED)
        sup_paths = VGroup(*[
            _smooth_path([
                centers[1] + LEFT * 1.35 + UP * 1.0,
                centers[1] + LEFT * 0.5 + UP * (dy * 0.5 + 0.2),
                centers[1] + RIGHT * 0.9 + UP * (dy * 0.6),
                leak.get_left(),
            ])
            for dy in (-0.7, 0.0, 0.6)
        ])
        leak_tag = Text("Abluft über Fugen", font_size=15, color=P_MUTED)
        leak_tag.next_to(rooms[1], DOWN, buff=0.18)
        # endregion

        # region Panel 3 — balanced: both streams ducted
        bal_sup = _fan(centers[2] + LEFT * 1.35 + UP * 1.525, P_CYAN)
        bal_exh = _fan(centers[2] + RIGHT * 1.35 + UP * 1.525, P_ORANGE)
        bal_duct_s = Line(centers[2] + LEFT * 1.35 + UP * 1.075, bal_sup.get_bottom(),
                          color=P_CYAN, stroke_width=5)
        bal_duct_e = Line(centers[2] + RIGHT * 1.35 + UP * 1.075, bal_exh.get_bottom(),
                          color=P_ORANGE, stroke_width=5)
        bal_in = Arrow(bal_sup.get_top() + UP * 0.55, bal_sup.get_top(), color=P_CYAN,
                       stroke_width=4, buff=0.04, max_tip_length_to_length_ratio=0.35)
        bal_out = Arrow(bal_exh.get_top(), bal_exh.get_top() + UP * 0.55, color=P_ORANGE,
                        stroke_width=4, buff=0.04, max_tip_length_to_length_ratio=0.35)
        bal_paths = VGroup(*[
            _smooth_path([
                centers[2] + LEFT * 1.35 + UP * 1.0,
                centers[2] + LEFT * 0.6 + UP * (dy * 0.6 - 0.1),
                centers[2] + RIGHT * 0.6 + UP * (dy * 0.5),
                centers[2] + RIGHT * 1.35 + UP * 1.0,
            ])
            for dy in (-0.7, 0.0, 0.6)
        ])
        bal_tag = Text("beide Ströme geführt", font_size=15, color=P_GREEN)
        bal_tag.next_to(rooms[2], DOWN, buff=0.18)
        # endregion

        pressures = VGroup(*[
            Text(t, font_size=30, color=c).move_to(centers[i] + DOWN * 0.42)
            for i, (t, c) in enumerate([("−", P_ORANGE), ("+", P_CYAN), ("=", P_GREEN)])
        ])
        press_tags = VGroup(*[
            Text(t, font_size=16, color=P_MUTED).next_to(pressures[i], DOWN, buff=0.1)
            for i, t in enumerate(["Unterdruck", "Überdruck", "ausgeglichen"])
        ])

        legend = VGroup(*[
            VGroup(Dot(radius=0.08, color=c), Text(t, font_size=15, color=c)).arrange(RIGHT, buff=0.14)
            for t, c in (
                ("ungefilterte Außenluft", P_MUTED),
                ("aufbereitete Zuluft", P_CYAN),
                ("erwärmte Abluft", P_ORANGE),
            )
        ]).arrange(RIGHT, buff=0.7)
        legend.move_to(DOWN * 2.42)

        verdict = VGroup(
            Text("Nur die Zu-/Abluftanlage führt beide Luftströme kontrolliert —",
                 font_size=19, color=P_GREEN),
            Text("erst dadurch werden Filterung, Wärmerückgewinnung und Feuchteregelung möglich.",
                 font_size=19, color=P_GREEN),
        ).arrange(DOWN, buff=0.12)
        verdict.to_edge(DOWN, buff=0.26)

        play_scene_title(self, title)
        self.play(FadeIn(sub, shift=DOWN * 0.12), run_time=0.7)
        _pad(self, 3.31, 1.4 + 0.7)

        self.play(
            LaggedStart(*[Create(r) for r in rooms], lag_ratio=0.25),
            LaggedStart(*[FadeIn(h, shift=DOWN * 0.1) for h in headers], lag_ratio=0.25),
            run_time=2.0,
        )
        self.play(FadeIn(legend), run_time=0.8)
        g1 = _guides(exh_paths, P_ORANGE)
        self.play(FadeIn(exh_fan), Create(exh_duct), GrowArrow(exh_out),
                  FadeIn(ald), FadeIn(ald_tag), Create(g1), run_time=1.5)
        _flow(self, exh_paths, P_MUTED, run_time=2.8, waves=3, cycles=2.0, color_end=P_ORANGE)
        self.play(FadeIn(pressures[0], scale=1.2), FadeIn(press_tags[0]), run_time=0.9)
        _pad(self, 11.07, 2.0 + 0.8 + 1.5 + 2.8 + 0.9)

        g2 = _guides(sup_paths, P_CYAN)
        self.play(FadeIn(sup_fan), Create(sup_duct), GrowArrow(sup_in),
                  FadeIn(leak), FadeIn(leak_tag), Create(g2), run_time=1.5)
        _flow(self, sup_paths, P_CYAN, run_time=2.8, waves=3, cycles=2.0, color_end=P_MUTED)
        self.play(FadeIn(pressures[1], scale=1.2), FadeIn(press_tags[1]), run_time=0.9)
        _pad(self, 9.59, 1.5 + 2.8 + 0.9)

        g3 = _guides(bal_paths, P_GREEN)
        self.play(FadeIn(bal_sup), FadeIn(bal_exh), Create(bal_duct_s), Create(bal_duct_e),
                  GrowArrow(bal_in), GrowArrow(bal_out), FadeIn(bal_tag), Create(g3),
                  run_time=1.5)
        _flow(self, bal_paths, P_CYAN, run_time=2.6, waves=3, cycles=2.0, color_end=P_ORANGE)
        self.play(FadeIn(pressures[2], scale=1.2), FadeIn(press_tags[2]), run_time=0.8)
        _pad(self, 6.21, 1.5 + 2.6 + 0.8)

        self.play(FadeOut(legend), run_time=0.4)
        _flow(
            self, VGroup(*exh_paths, *sup_paths, *bal_paths),
            P_CYAN, run_time=2.8, waves=3, cycles=2.2, color_end=P_ORANGE,
            extra=[rooms[2].animate.set_stroke(color=P_GREEN)],
        )
        self.play(FadeIn(verdict, shift=UP * 0.14), run_time=1.3)
        _pad(self, 7.65, 0.4 + 2.8 + 1.3)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 4 – Heat and cold recovery
# ═══════════════════════════════════════════════════════════════════════
class Beat4_Waermerueckgewinnung(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = _main_title()
        sub = _subtitle("Wärmerückgewinnung: im Sommer Kälterückgewinnung", P_BLUE)

        # region Exchanger body — two parallel channels in counterflow
        y_sup, y_exh = 1.15, -0.35
        x_in, x_out = -5.35, 5.35
        core = Rectangle(width=4.4, height=2.6, color=P_WHITE, stroke_width=3.0)
        core.move_to(np.array([0.0, (y_sup + y_exh) / 2, 0.0]))
        plates = VGroup(*[
            Line(
                np.array([x, core.get_bottom()[1] + 0.14, 0.0]),
                np.array([x, core.get_top()[1] - 0.14, 0.0]),
                color=P_MUTED, stroke_width=1.4,
            ).set_stroke(opacity=0.45)
            for x in np.linspace(-1.85, 1.85, 11)
        ])
        core_tag = Text("Plattenwärmeübertrager", font_size=18, color=P_WHITE)
        core_tag.next_to(core, DOWN, buff=0.2)
        core_note = Text("getrennte Kanäle — die Luftströme mischen sich nicht",
                         font_size=16, color=P_MUTED)
        core_note.next_to(core_tag, DOWN, buff=0.12)
        # endregion

        supply = VGroup(_smooth_path([
            np.array([x_in, y_sup, 0.0]), np.array([-2.0, y_sup, 0.0]),
            np.array([2.0, y_sup, 0.0]), np.array([x_out, y_sup, 0.0]),
        ]))
        exhaust = VGroup(_smooth_path([
            np.array([x_out, y_exh, 0.0]), np.array([2.0, y_exh, 0.0]),
            np.array([-2.0, y_exh, 0.0]), np.array([x_in, y_exh, 0.0]),
        ]))

        # Heat leaves the hot outdoor air and enters the cooler exhaust: arrows point down.
        transfer = VGroup(*[
            Arrow(
                np.array([x, y_sup - 0.24, 0.0]), np.array([x, y_exh + 0.24, 0.0]),
                color=P_YELLOW, stroke_width=3.5, buff=0.0,
                max_tip_length_to_length_ratio=0.24,
            ).set_opacity(0.85)
            for x in (-1.3, 0.0, 1.3)
        ])
        transfer_tag = Text("Wärmestrom", font_size=16, color=P_YELLOW)
        transfer_tag.next_to(transfer[2], RIGHT, buff=0.14)

        def _tag(name, temp, color, point, direction):
            group = VGroup(
                Text(name, font_size=17, color=color),
                Text(temp, font_size=23, color=color),
            ).arrange(DOWN, buff=0.06)
            return group.move_to(point).shift(direction)

        aul = _tag("Außenluft", "32 °C", P_RED, np.array([x_in, y_sup, 0.0]), LEFT * 0.72 + UP * 0.1)
        zul = _tag("Zuluft", "27 °C", P_CYAN, np.array([x_out, y_sup, 0.0]), RIGHT * 0.72 + UP * 0.1)
        abl = _tag("Abluft", "26 °C", P_TEAL, np.array([x_out, y_exh, 0.0]), RIGHT * 0.72 + DOWN * 0.1)
        fol = _tag("Fortluft", "31 °C", P_ORANGE, np.array([x_in, y_exh, 0.0]), LEFT * 0.72 + DOWN * 0.1)

        message = Text("Die kühlere Abluft kühlt die heiße Außenluft vor — ohne Kältemaschine.",
                       font_size=21, color=P_YELLOW)
        message.move_to(DOWN * 2.28)

        eq = _equation(
            r"\Phi", "=",
            r"\frac{\theta_{ZUL} - \theta_{AUL}}{\theta_{ABL} - \theta_{AUL}}", "=",
            r"\frac{5\,\mathrm{K}}{6\,\mathrm{K}}", r"\approx 0{,}8",
            font_size=40, colors={0: P_YELLOW, 5: P_YELLOW},
        )
        eq.move_to(DOWN * 3.12)

        play_scene_title(self, title)
        self.play(FadeIn(sub, shift=DOWN * 0.12), run_time=0.7)
        _pad(self, 4.51, 1.4 + 0.7)

        self.play(Create(core), Create(plates), FadeIn(core_tag), FadeIn(core_note), run_time=2.0)
        _pad(self, 8.92, 2.0)

        gs = _guides(supply, P_RED, opacity=0.28)
        ge = _guides(exhaust, P_TEAL, opacity=0.28)
        self.play(Create(gs), FadeIn(aul), run_time=1.0)
        _flow(self, supply, P_RED, run_time=2.5, waves=4, cycles=2.0, color_end=P_CYAN)
        self.play(FadeIn(zul), run_time=0.6)
        self.play(Create(ge), FadeIn(abl), run_time=1.0)
        _flow(self, exhaust, P_TEAL, run_time=2.5, waves=4, cycles=2.0, color_end=P_ORANGE)
        self.play(FadeIn(fol), run_time=0.6)
        self.play(
            LaggedStart(*[GrowArrow(a) for a in transfer], lag_ratio=0.25),
            FadeIn(transfer_tag), run_time=1.2,
        )
        _flow(
            self, VGroup(*supply, *exhaust), P_RED, run_time=1.9, waves=4, cycles=2.2,
            color_end=P_ORANGE,
            extra=[LaggedStart(*[Indicate(a, color=P_YELLOW, scale_factor=1.15) for a in transfer],
                               lag_ratio=0.3, run_time=1.9)],
        )
        self.play(FadeIn(message, shift=UP * 0.14), run_time=0.8)
        _pad(self, 12.07, 1.0 + 2.5 + 0.6 + 1.0 + 2.5 + 0.6 + 1.2 + 1.9 + 0.8)

        self.play(FadeIn(eq, shift=UP * 0.12), run_time=1.2)
        self.play(Indicate(eq[5], color=P_YELLOW, scale_factor=1.3), run_time=0.9)
        _pad(self, 15.45, 1.2 + 0.9)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 5 – Air distribution: mixing vs. displacement
# ═══════════════════════════════════════════════════════════════════════
class Beat5_Luftfuehrung(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = _main_title()
        sub = _subtitle("Luftführung im Raum: Mischen oder Verdrängen?", P_YELLOW)

        left_c = LEFT * 3.55 + DOWN * 0.45
        right_c = RIGHT * 3.55 + DOWN * 0.45
        left_room = _room(left_c, 5.8, 3.3)
        right_room = _room(right_c, 5.8, 3.3)

        head_l = Text("Mischlüftung", font_size=23, color=P_CYAN).next_to(left_room, UP, buff=0.28)
        head_r = Text("Quelllüftung", font_size=23, color=P_GREEN).next_to(right_room, UP, buff=0.28)

        # region Mixing — one high-momentum jet stirs the whole volume
        diffuser = RoundedRectangle(
            width=1.1, height=0.2, corner_radius=0.05,
            color=P_CYAN, stroke_width=2.4, fill_color=P_CYAN, fill_opacity=0.3,
        ).move_to(left_c + LEFT * 1.2 + UP * 1.45)
        mix_return = RoundedRectangle(
            width=0.9, height=0.2, corner_radius=0.05,
            color=P_ORANGE, stroke_width=2.4, fill_color=P_ORANGE, fill_opacity=0.3,
        ).move_to(left_c + RIGHT * 1.95 + UP * 1.45)
        mix_wash = Rectangle(
            width=5.7, height=3.2, stroke_width=0, fill_color=P_CYAN, fill_opacity=0.0,
        ).move_to(left_c)
        mix_person = _person(left_c + RIGHT * 1.3 + DOWN * 1.05)
        mix_paths = VGroup(
            _smooth_path([
                diffuser.get_bottom(),
                left_c + LEFT * 2.3 + UP * 0.55,
                left_c + LEFT * 2.0 + DOWN * 1.15,
                left_c + RIGHT * 0.2 + DOWN * 0.75,
                mix_return.get_bottom(),
            ]),
            _smooth_path([
                diffuser.get_bottom(),
                left_c + LEFT * 0.6 + UP * 0.5,
                left_c + LEFT * 0.9 + DOWN * 1.2,
                left_c + RIGHT * 1.0 + DOWN * 0.35,
                mix_return.get_bottom() + LEFT * 0.1,
            ]),
            _smooth_path([
                diffuser.get_bottom(),
                left_c + RIGHT * 0.9 + UP * 0.85,
                left_c + RIGHT * 2.2 + DOWN * 0.2,
                left_c + RIGHT * 1.4 + DOWN * 1.15,
                mix_return.get_bottom() + LEFT * 0.2,
            ]),
        )
        jet_tag = Text("hohe Austrittsgeschwindigkeit", font_size=15, color=P_CYAN)
        jet_tag.next_to(diffuser, DOWN, buff=0.1).shift(RIGHT * 0.5)
        mix_note = VGroup(
            Text("Induktion — der Strahl reißt Raumluft mit", font_size=16, color=P_MUTED),
            Text("gleichmäßige Temperatur · hohe Kühlleistung", font_size=16, color=P_CYAN),
            Text("Schadstoffe werden mitverteilt", font_size=16, color=P_ORANGE),
        ).arrange(DOWN, buff=0.14)
        mix_note.next_to(left_room, DOWN, buff=0.26)
        # endregion

        # region Displacement — a cool lake at the floor, plumes lift the heat out
        source = RoundedRectangle(
            width=0.34, height=0.95, corner_radius=0.07,
            color=P_GREEN, stroke_width=2.4, fill_color=P_GREEN, fill_opacity=0.25,
        ).move_to(right_c + LEFT * 2.55 + DOWN * 1.15)
        strata = VGroup(*[
            Rectangle(width=5.7, height=h, stroke_width=0, fill_color=c, fill_opacity=0.0)
            .move_to(right_c + UP * y)
            for h, c, y in ((0.75, P_GREEN, -1.22), (1.2, P_TEAL, -0.25), (1.25, P_ORANGE, 1.02))
        ])
        scale_marks = VGroup(*[
            Text(t, font_size=15, color=c).move_to(right_c + RIGHT * 2.35 + UP * y)
            for t, c, y in (("20 °C", P_GREEN, -1.22), ("23 °C", P_TEAL, -0.25), ("26 °C", P_ORANGE, 1.02))
        ])
        src_person = _person(right_c + RIGHT * 0.2 + DOWN * 1.05)
        plume = _plume(right_c + RIGHT * 0.2 + DOWN * 0.35, 1.1, P_ORANGE, width=0.4)
        exit_grille = RoundedRectangle(
            width=1.0, height=0.2, corner_radius=0.05,
            color=P_ORANGE, stroke_width=2.4, fill_color=P_ORANGE, fill_opacity=0.3,
        ).move_to(right_c + RIGHT * 1.3 + UP * 1.45)
        disp_paths = VGroup(*[
            _smooth_path([
                source.get_right() + UP * dy,
                right_c + LEFT * 1.4 + DOWN * 1.3,
                right_c + LEFT * 0.2 + DOWN * 1.28,
                right_c + RIGHT * (0.05 + 0.12 * i) + DOWN * 1.15,
            ])
            for i, dy in enumerate((-0.3, 0.0, 0.3))
        ])
        rise_paths = VGroup(*[
            _smooth_path([
                right_c + RIGHT * (0.2 + 0.14 * s) + DOWN * 1.0,
                right_c + RIGHT * (0.3 + 0.28 * s) + DOWN * 0.25,
                right_c + RIGHT * (0.8 + 0.2 * s) + UP * 0.7,
                exit_grille.get_bottom(),
            ])
            for s in (-1, 0, 1)
        ])
        disp_note = VGroup(
            Text("impulsarm am Boden · der Auftrieb trägt die Last nach oben",
                 font_size=16, color=P_MUTED),
            Text("Temperaturschichtung · beste Luftqualität in der Aufenthaltszone",
                 font_size=16, color=P_GREEN),
            Text("begrenzte Kühlleistung (ca. 40 W/m²) · Zuggefahr am Boden",
                 font_size=16, color=P_ORANGE),
        ).arrange(DOWN, buff=0.14)
        disp_note.next_to(right_room, DOWN, buff=0.26)
        # endregion

        play_scene_title(self, title)
        self.play(FadeIn(sub, shift=DOWN * 0.12), run_time=0.7)
        self.play(Create(left_room), Create(right_room), FadeIn(head_l), FadeIn(head_r),
                  run_time=1.6)
        self.add(mix_wash, *strata)
        _pad(self, 4.52, 1.4 + 0.7 + 1.6)

        gm = _guides(mix_paths, P_CYAN)
        self.play(FadeIn(diffuser), FadeIn(mix_return), FadeIn(mix_person),
                  Create(gm), FadeIn(jet_tag), run_time=1.4)
        _flow(
            self, mix_paths, P_CYAN, run_time=3.4, waves=5, cycles=2.8,
            extra=[mix_wash.animate.set_fill(opacity=0.13)],
        )
        self.play(
            LaggedStart(*[FadeIn(n, shift=UP * 0.1) for n in mix_note], lag_ratio=0.3),
            run_time=1.7,
        )
        _pad(self, 13.07, 1.4 + 3.4 + 1.7)

        gd = _guides(disp_paths, P_GREEN)
        self.play(FadeIn(source), FadeIn(src_person), FadeIn(exit_grille), Create(gd),
                  run_time=1.2)
        _flow(
            self, disp_paths, P_GREEN, run_time=2.8, waves=4, cycles=2.0,
            extra=[strata[0].animate.set_fill(opacity=0.2)],
        )
        gr = _guides(rise_paths, P_ORANGE)
        self.play(Create(plume), Create(gr), run_time=1.2)
        _flow(
            self, rise_paths, P_GREEN, run_time=3.2, waves=4, cycles=2.2, color_end=P_ORANGE,
            extra=[
                strata[1].animate.set_fill(opacity=0.13),
                strata[2].animate.set_fill(opacity=0.13),
                FadeIn(scale_marks),
            ],
        )
        _pad(self, 15.43, 1.2 + 2.8 + 1.2 + 3.2)

        self.play(
            LaggedStart(*[FadeIn(n, shift=UP * 0.1) for n in disp_note], lag_ratio=0.3),
            run_time=1.7,
        )
        _pad(self, 9.72, 1.7)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 6 – Air handling functions and air change rate
# ═══════════════════════════════════════════════════════════════════════
class Beat6_RLTFunktionen(Scene):
    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = _main_title()
        sub = _subtitle("Vier thermodynamische Funktionen der RLT-Anlage", P_TEAL)

        stages = [
            ("Filtern", P_MUTED),
            ("Heizen", P_RED),
            ("Kühlen", P_BLUE),
            ("Befeuchten", P_CYAN),
            ("Entfeuchten", P_YELLOW),
        ]
        blocks = VGroup()
        for name, color in stages:
            box = Rectangle(
                width=2.05, height=1.3, color=color, stroke_width=2.4,
                fill_color=color, fill_opacity=0.12,
            )
            label = Text(name, font_size=18, color=color).move_to(box)
            blocks.add(VGroup(box, label))
        blocks.arrange(RIGHT, buff=0.2)
        blocks.move_to(UP * 1.3)

        casing = SurroundingRectangle(blocks, color=P_WHITE, buff=0.22, corner_radius=0.08)
        casing.set_stroke(width=2.0, opacity=0.6)
        in_arrow = Arrow(casing.get_left() + LEFT * 0.85, casing.get_left(), color=P_RED,
                         stroke_width=4, buff=0.05, max_tip_length_to_length_ratio=0.3)
        out_arrow = Arrow(casing.get_right(), casing.get_right() + RIGHT * 0.85, color=P_CYAN,
                          stroke_width=4, buff=0.05, max_tip_length_to_length_ratio=0.3)
        in_tag = Text("Außenluft", font_size=16, color=P_RED).next_to(in_arrow, UP, buff=0.14)
        out_tag = Text("Zuluft", font_size=16, color=P_CYAN).next_to(out_arrow, UP, buff=0.14)

        classes = VGroup(
            Text("Lüftungsanlage — nur Luftförderung und Filterung", font_size=18, color=P_MUTED),
            Text("Teilklimaanlage — nicht alle vier Funktionen", font_size=18, color=P_TEAL),
            Text("Vollklimaanlage — Heizen, Kühlen, Be- und Entfeuchten", font_size=18, color=P_GREEN),
        ).arrange(DOWN, buff=0.2, aligned_edge=LEFT)
        classes.move_to(DOWN * 0.75)

        eq = _equation(
            "n", "=", r"\frac{\dot q_{V,R}}{V_{\mathrm{Raum}}}",
            r"\quad\left[\tfrac{1}{\mathrm{h}}\right]",
            font_size=42, colors={0: P_YELLOW, 2: P_CYAN, 3: P_MUTED},
        )
        eq.move_to(DOWN * 2.4)
        eq_note = Text(
            "Der Luftwechsel verknüpft den ausgelegten Volumenstrom mit dem Raumvolumen",
            font_size=17, color=P_MUTED,
        )
        eq_note.next_to(eq, DOWN, buff=0.26)

        play_scene_title(self, title)
        self.play(FadeIn(sub, shift=DOWN * 0.12), run_time=0.7)
        _pad(self, 3.32, 1.4 + 0.7)

        self.play(Create(casing), GrowArrow(in_arrow), GrowArrow(out_arrow),
                  FadeIn(in_tag), FadeIn(out_tag), run_time=1.5)
        self.play(
            LaggedStart(*[FadeIn(b, shift=UP * 0.12) for b in blocks], lag_ratio=0.28),
            run_time=2.4,
        )
        _pad(self, 6.43, 1.5 + 2.4)

        self.play(
            LaggedStart(*[FadeIn(c, shift=RIGHT * 0.14) for c in classes], lag_ratio=0.35),
            run_time=2.0,
        )
        self.play(
            Indicate(VGroup(blocks[1], blocks[2], blocks[3], blocks[4]), color=P_GREEN,
                     scale_factor=1.05),
            run_time=1.4,
        )
        _pad(self, 15.29, 2.0 + 1.4)

        self.play(FadeIn(eq, shift=UP * 0.14), run_time=1.2)
        self.play(FadeIn(eq_note), run_time=0.9)
        _pad(self, 13.48, 1.2 + 0.9)


# ═══════════════════════════════════════════════════════════════════════
# BEAT 7 – Recap and normative references
# ═══════════════════════════════════════════════════════════════════════
class Beat7_Zusammenfassung(Scene):
    """📚 Closing card: what the video established, and where the rules live."""

    LEARNED = [
        ("Freie Lüftung: Wind und Auftrieb, nicht regelbar", P_GREEN),
        ("Abluft, Zuluft, Zu-/Abluft: die Druckverhältnisse", P_CYAN),
        ("Nur die Zu-/Abluftanlage erlaubt WRG und Filterung", P_CYAN),
        ("Wärmerückgewinnung: Φ ≈ 0,8 kühlt die Zuluft vor", P_BLUE),
        ("Mischlüftung mischt, Quelllüftung schichtet", P_YELLOW),
        ("Luftwechsel n verknüpft Volumenstrom und Raum", P_ORANGE),
    ]

    NORMS = [
        ("DIN 1946-6", "Wohnungslüftung: Systemarten"),
        ("DIN EN 16798-3", "Nichtwohngebäude: RLT-Anlagen"),
        ("DIN EN 16798-1", "Auslegungs-Volumenströme"),
        ("DIN EN 308", "Rückwärmzahl Φ der WRG"),
        ("DIN EN ISO 7730", "Behaglichkeit, Zugluftrisiko"),
        ("VDI 2078", "Kühllastberechnung"),
    ]

    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        title = _main_title()
        sub = _subtitle("Zusammenfassung und Normenbezug", P_TEAL)

        left_x, right_x, top_y = -3.72, 3.72, 2.18

        def _column_head(text, color, x):
            head = Text(text, font_size=21, color=color)
            head.move_to(np.array([x, top_y, 0.0]))
            rule = Line(LEFT * 3.15, RIGHT * 3.15, color=color, stroke_width=1.6)
            rule.set_stroke(opacity=0.5)
            rule.next_to(head, DOWN, buff=0.2).set_x(x)
            return head, rule

        head_l, rule_l = _column_head("Was wir gelernt haben", P_WHITE, left_x)
        # "Normen und Richtlinien": VDI 2078 / 6022 are guidelines, not DIN standards.
        head_r, rule_r = _column_head("Normen und Richtlinien", P_WHITE, right_x)

        bullets = VGroup(*[
            VGroup(
                Dot(radius=0.07, color=color),
                Text(text, font_size=17, color=P_WHITE),
            ).arrange(RIGHT, buff=0.2)
            for text, color in self.LEARNED
        ])
        bullets.arrange(DOWN, buff=0.34, aligned_edge=LEFT)
        bullets.next_to(rule_l, DOWN, buff=0.42).align_to(rule_l, LEFT)

        def _norm_row(norm, desc, gutter=2.25):
            """📖 Standard number in a fixed left column, subject aligned beside it."""
            tag = Text(norm, font_size=17, color=P_CYAN)
            body = Text(desc, font_size=15, color=P_MUTED)
            body.move_to(tag.get_left() + RIGHT * (gutter + body.width / 2))
            body.set_y(tag.get_y())
            return VGroup(tag, body)

        norms = VGroup(*[_norm_row(n, d) for n, d in self.NORMS])
        norms.arrange(DOWN, buff=0.34, aligned_edge=LEFT)
        norms.next_to(rule_r, DOWN, buff=0.42).align_to(rule_r, LEFT)

        footnote = VGroup(
            Text("Ergänzend: VDI 6022 (Hygiene) und DIN EN ISO 16890 (Filterklassen)",
                 font_size=15, color=P_MUTED),
            Text("Normen ohne Jahresangabe zitiert — bitte den aktuellen Ausgabestand prüfen.",
                 font_size=15, color=P_MUTED),
        ).arrange(DOWN, buff=0.12)
        footnote.to_edge(DOWN, buff=0.34)

        play_scene_title(self, title)
        self.play(FadeIn(sub, shift=DOWN * 0.12), run_time=0.8)
        self.play(
            LaggedStart(
                AnimationGroup(FadeIn(head_l, shift=DOWN * 0.1), Create(rule_l)),
                AnimationGroup(FadeIn(head_r, shift=DOWN * 0.1), Create(rule_r)),
                lag_ratio=0.45,
            ),
            run_time=1.5,
        )
        self.play(
            LaggedStart(*[FadeIn(b, shift=RIGHT * 0.16) for b in bullets], lag_ratio=0.3),
            run_time=2.8,
        )
        self.play(
            LaggedStart(*[FadeIn(n, shift=LEFT * 0.16) for n in norms], lag_ratio=0.3),
            run_time=2.8,
        )
        self.play(FadeIn(footnote), run_time=1.0)
        self.wait(3.4)
