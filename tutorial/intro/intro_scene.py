"""🏛️ Reusable NGS intro card for Manim tutorial videos.

Institute: Nachhaltige Gebäudesysteme (IEK), Leibniz Universität Hannover
https://www.iek.uni-hannover.de/ngs

Subclass ``NGSIntro`` and set ``topic_de`` / ``topic_explain_de`` / ``series_de``,
or render ``Demo_Intro_Kuehllast`` as a Sideview smoke test.
"""

from __future__ import annotations

from pathlib import Path

import sys as _sys

import numpy as np
from manim import *
from manim.utils.rate_functions import ease_in_out_sine, ease_out_cubic, smootherstep

_TUTORIAL_ROOT = next(
    p for p in Path(__file__).resolve().parents
    if (p / "manim_fonts.py").is_file()
)
if str(_TUTORIAL_ROOT) not in _sys.path:
    _sys.path.insert(0, str(_TUTORIAL_ROOT))
from manim_fonts import apply_body_font

# region Palette
P_DEEP_DARK = "#0B0C10"
P_WHITE = "#E6ECF3"
P_CYAN = "#E6ECF3"
P_TEAL = "#E6ECF3"
P_MUTED = "#8B95A5"
# endregion

# region Institute Copy
INSTITUTE_SHORT = "Nachhaltige Gebäudesysteme"
INSTITUTE_PARENT = "Institut für Entwerfen und Konstruieren"
UNIVERSITY = "Leibniz Universität Hannover"
INSTITUTE_URL = "Prof. Dr.-Ing. Philipp Geyer"
# endregion

# region Geometry
FRAME_W = 13.30
FRAME_H = 7.10
# endregion

WELFENSCHLOSS_PDF = Path(__file__).resolve().parent / "welfenschloss (1).pdf"


# region Vector Art
def _pdf_subpaths(path: Path) -> list[np.ndarray]:
    """🏛️ Read a line-art PDF's stroked and filled paths as cubic Bézier anchor/handle arrays in PDF points.

    Every content stream that draws paths is tokenised; ``cm`` transforms are composed on a ``q``/``Q`` stack,
    lines become straight cubics and ``h`` closes a subpath.
    https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf (§8.5 Path Construction)
    """
    import re
    import zlib

    data = path.read_bytes()
    subpaths: list[np.ndarray] = []
    for raw in re.findall(rb"stream\r?\n(.*?)\r?\nendstream", data, re.S):
        try:
            content = zlib.decompress(raw).decode("latin-1")
        except (zlib.error, UnicodeDecodeError):
            continue
        if " Do" in content or " m" not in content:
            continue
        ctm, stack, args = np.eye(3), [], []
        current: list[np.ndarray] = []

        def to_page(x: float, y: float) -> np.ndarray:
            return (np.array([x, y, 1.0]) @ ctm)[:2]

        def flush():
            if len(current) >= 5:
                subpaths.append(np.array(current))
            current.clear()

        for token in content.split():
            if re.fullmatch(r"[-+]?(\d+\.?\d*|\.\d+)", token):
                args.append(float(token))
                continue
            if token == "q":
                stack.append(ctm.copy())
            elif token == "Q":
                ctm = stack.pop() if stack else np.eye(3)
            elif token == "cm" and len(args) >= 6:
                a, b, c, d, e, f = args[-6:]
                ctm = np.array([[a, b, 0.0], [c, d, 0.0], [e, f, 1.0]]) @ ctm
            elif token == "m" and len(args) >= 2:
                flush()
                pen = to_page(*args[-2:])
                current.append(pen)
            elif token == "l" and len(args) >= 2 and current:
                a, b = current[-1], to_page(*args[-2:])
                current.extend([a, a + (b - a) / 3, a + 2 * (b - a) / 3, b])
            elif token == "c" and len(args) >= 6 and current:
                current.extend([current[-1], to_page(*args[-6:-4]), to_page(*args[-4:-2]), to_page(*args[-2:])])
            elif token == "h" and current:
                a, b = current[-1], current[0]
                current.extend([a, a + (b - a) / 3, a + 2 * (b - a) / 3, b])
            elif token in ("S", "s", "f", "F", "f*", "B", "b", "n"):
                flush()
            args = []
        flush()
    return subpaths


def _vector_art(path: Path, color: str, *, width: float, opacity: float = 1.0, stroke: float = 1.2,
                bands: int = 36) -> VGroup:
    """🖋️ PDF line art as ``bands`` vertical VMobject strips, scaled to ``width`` and centred on the origin.

    Strips sorted left to right let ``Create`` draw the drawing across the frame like a pen sweep.
    """
    subpaths = _pdf_subpaths(path)
    pts = np.concatenate(subpaths)
    lo, hi = pts.min(axis=0), pts.max(axis=0)
    k = width / (hi[0] - lo[0])
    mid = (lo + hi) / 2
    edges = np.linspace(lo[0], hi[0], bands + 1)
    strips = [VMobject(stroke_color=color, stroke_width=stroke, stroke_opacity=opacity, fill_opacity=0.0)
              for _ in range(bands)]
    for sub in subpaths:
        quads = sub[1:].reshape(-1, 4, 2)
        band = min(bands - 1, int(np.searchsorted(edges, quads[:, :, 0].mean(), side="right")) - 1)
        scene_pts = np.column_stack([(quads.reshape(-1, 2) - mid) * k, np.zeros(len(quads) * 4)])
        strips[max(0, band)].append_points(scene_pts)
    return VGroup(*[s for s in strips if s.has_points()])
# endregion


# region Chrome
def _screen_frame() -> VGroup:
    """🖼️ Double hairline border with cyan corner ticks."""
    outer = Rectangle(width=FRAME_W, height=FRAME_H, color=P_TEAL, stroke_width=2.2)
    inner = Rectangle(width=FRAME_W - 0.3, height=FRAME_H - 0.3, color=P_TEAL, stroke_width=0.9)
    inner.set_stroke(opacity=0.45)
    return VGroup(outer, inner)


def _corner_ticks(rect: Rectangle, arm: float = 0.62) -> VGroup:
    """📐 L-shaped accents anchored to the border corners."""
    ticks = VGroup()
    for corner, h_dir, v_dir in (
        (rect.get_corner(UL), RIGHT, DOWN),
        (rect.get_corner(UR), LEFT, DOWN),
        (rect.get_corner(DL), RIGHT, UP),
        (rect.get_corner(DR), LEFT, UP),
    ):
        ticks.add(
            VGroup(
                Line(corner, corner + h_dir * arm, color=P_CYAN, stroke_width=3.4),
                Line(corner, corner + v_dir * arm, color=P_CYAN, stroke_width=3.4),
            )
        )
    return ticks


def _diamond_rule(half: float = 2.55) -> VGroup:
    """💠 Centered divider with a diamond node."""
    node = Square(side_length=0.13, color=P_CYAN, fill_color=P_CYAN, fill_opacity=1.0, stroke_width=0)
    node.rotate(PI / 4)
    left = Line(LEFT * half, LEFT * 0.22, color=P_TEAL, stroke_width=1.6)
    right = Line(RIGHT * 0.22, RIGHT * half, color=P_TEAL, stroke_width=1.6)
    left.set_stroke(opacity=0.75)
    right.set_stroke(opacity=0.75)
    return VGroup(left, node, right)
# endregion


# region Intro Template
class NGSIntro(Scene):
    """🎬 Institute intro card — override class attrs per video topic.

    Attributes:
        topic_de: Main on-screen topic title (German).
        topic_explain_de: One short sentence explaining the video.
        series_de: Optional series / part label (e.g. ``Kühllast · Teil 1``).
        hold_seconds: Extra hold after all elements are on screen.
    """

    topic_de = "Thema der Lektion"
    topic_explain_de = "Kurze Erklärung des Videothemas."
    series_de = ""
    hold_seconds = 1.8

    def construct(self):
        self.camera.background_color = P_DEEP_DARK
        apply_body_font()

        # region Chrome
        frame = _screen_frame()
        ticks = _corner_ticks(frame[0])
        # endregion

        # region Watermark
        building = _vector_art(WELFENSCHLOSS_PDF, P_WHITE, width=12.6, opacity=0.22, stroke=1.1)
        building.align_to(frame[0], DOWN).shift(UP * 0.3)
        sweep = Line(UP * (FRAME_H / 2 - 0.2), DOWN * (FRAME_H / 2 - 0.2), color=P_CYAN, stroke_width=2.6)
        sweep.move_to(LEFT * (FRAME_W / 2 - 0.35))
        # endregion

        # region Identity Block
        university = Text(UNIVERSITY, font_size=42, color=P_WHITE)
        rule = _diamond_rule()
        parent = Text(INSTITUTE_PARENT, font_size=19, color=P_MUTED)
        institute = Text(INSTITUTE_SHORT, font_size=31, color=P_CYAN)
        url = Text(INSTITUTE_URL, font_size=14, color=P_TEAL)

        identity = VGroup(university, rule, parent, institute, url)
        identity.arrange(DOWN, buff=0.3)
        # endregion

        # region Topic Block
        topic_block = VGroup()
        if self.series_de:
            topic_block.add(Text(self.series_de, font_size=16, color=P_TEAL))

        topic = Text(self.topic_de, font_size=36, color=P_WHITE)
        topic_block.add(topic)

        explain = Text(self.topic_explain_de, font_size=19, color=P_MUTED)
        if explain.width > FRAME_W - 1.6:
            explain = Text(self.topic_explain_de, font_size=16, color=P_MUTED)
        topic_block.add(explain)
        topic_block.arrange(DOWN, buff=0.24)
        # endregion

        # region Layout Guard
        card = VGroup(identity, topic_block).arrange(DOWN, buff=0.72)
        if card.height > FRAME_H - 1.6:
            card.scale_to_fit_height(FRAME_H - 1.6)
        card.move_to(UP * 0.25)

        plate = RoundedRectangle(
            width=topic_block.width + 1.5,
            height=topic_block.height + 0.75,
            corner_radius=0.14,
            color=P_TEAL,
            stroke_width=1.1,
            fill_color=P_DEEP_DARK,
            fill_opacity=0.8,
        )
        plate.set_stroke(opacity=0.35)
        plate.move_to(topic_block)
        # endregion

        # region Animation
        self.play(
            AnimationGroup(
                Create(frame[0], run_time=2.0, rate_func=ease_in_out_sine),
                FadeIn(frame[1], run_time=1.6, rate_func=smootherstep),
                LaggedStart(
                    *[GrowFromCenter(tick, rate_func=ease_out_cubic) for tick in ticks],
                    lag_ratio=0.18,
                    run_time=1.4,
                ),
                lag_ratio=0.4,
            )
        )

        self.add(sweep)
        self.play(
            LaggedStart(
                *[Create(strip, rate_func=smootherstep) for strip in building],
                lag_ratio=0.12,
                run_time=2.6,
            ),
            sweep.animate(rate_func=ease_in_out_sine).shift(RIGHT * (FRAME_W - 0.7)),
            run_time=2.6,
        )

        self.play(FadeOut(sweep, shift=RIGHT * 0.5, rate_func=ease_out_cubic), run_time=0.7)

        self.play(Write(university, run_time=1.8))
        self.play(
            LaggedStart(
                GrowFromCenter(rule, run_time=0.9, rate_func=ease_out_cubic),
                FadeIn(parent, shift=UP * 0.16, run_time=1.0, rate_func=ease_out_cubic),
                FadeIn(institute, shift=UP * 0.16, run_time=1.0, rate_func=ease_out_cubic),
                FadeIn(url, shift=UP * 0.16, run_time=1.0, rate_func=ease_out_cubic),
                lag_ratio=0.42,
            )
        )
        self.play(
            LaggedStart(
                FadeIn(plate, scale=1.06, run_time=1.0, rate_func=ease_out_cubic),
                *[
                    FadeIn(part, shift=UP * 0.18, run_time=1.0, rate_func=ease_out_cubic)
                    for part in topic_block
                ],
                lag_ratio=0.38,
            )
        )
        self.wait(self.hold_seconds)
        # endregion
# endregion


# region Demo Scenes
class Demo_Intro_Kuehllast(NGSIntro):
    """❄️ Series intro for Cooling demand — design cooling load, not annual energy."""

    topic_de = "Kühllast"
    topic_explain_de = "Welche Leistung im Sommer abgeführt werden muss — Last, nicht Jahresbedarf."
    series_de = "Gebäudeenergie · Kühllast"


class Demo_Intro_Heizlast(NGSIntro):
    """🔥 Series intro for Heating demand — losses, gains, annual balance."""

    topic_de = "Heizwärmebedarf"
    topic_explain_de = "Verluste, freie Gewinne und die Jahresbilanz im Winterbetrieb."
    series_de = "Gebäudeenergie · Heizwärmebedarf"


class Demo_Intro_PhysikalischeGrundlagen(NGSIntro):
    """⚛️ Series intro for Physical Fundamentals — energy, power, heat and air in the building."""

    topic_de = "Physikalische Grundlagen"
    topic_explain_de = "Energie, Leistung, Wärme und Luft im Gebäude."
    series_de = "Gebäudeenergie · Grundlagen"


class Demo_Intro_Energiebilanz(NGSIntro):
    """📜 Series intro for EnergyBalance — regulation, losses, and the Energieausweis."""

    topic_de = "Energiebilanz"
    topic_explain_de = "Vom Heizwärmebedarf über Last, Anlagen und GEG zum Energieausweis."
    series_de = "Gebäudeenergie · Energiebilanz"


class Intro_HeatingVsCooling(NGSIntro):
    """⚖️ Intro for Cooling part 1 — Heizwärmebedarf vs. Kühllast."""

    topic_de = "Heizwärmebedarf vs. Kühllast"
    topic_explain_de = "Gewinne senken im Winter den Bedarf — im Sommer werden sie zur Kühllast."
    series_de = "Kühllast · Teil 1"


class Intro_InternalGains(NGSIntro):
    """💡 Intro for Cooling part 2 — Interne Wärmegewinne."""

    topic_de = "Interne Wärmegewinne"
    topic_explain_de = "Personen, Geräte und Beleuchtung als innere Wärmequellen."
    series_de = "Kühllast · Teil 2"


class Intro_TransmissionHumidity(NGSIntro):
    """💧 Intro for Cooling part 3 — Transmission & Feuchte."""

    topic_de = "Transmission und Feuchte"
    topic_explain_de = "Wärmeleitung durch die Hülle und latente Lasten durch Feuchte."
    series_de = "Kühllast · Teil 3"


class Intro_SolarRadiation(NGSIntro):
    """☀️ Intro for Cooling part 4 — Solare Einstrahlung."""

    topic_de = "Solare Einstrahlung"
    topic_explain_de = "Direkte und diffuse Strahlung als dominante Sommerlast."
    series_de = "Kühllast · Teil 4"


class Intro_Systemauslegung(NGSIntro):
    """🌬️ Intro for Cooling part 5 — Systemauslegung."""

    topic_de = "Systemauslegung"
    topic_explain_de = "Von der Kühllast zum Luftvolumenstrom und Kanalquerschnitt."
    series_de = "Kühllast · Teil 5"


class Intro_Lueftungssysteme(NGSIntro):
    """🌀 Intro for Cooling part 6 — Lüftungssysteme."""

    topic_de = "Lüftungssysteme"
    topic_explain_de = "Last senken, frei lüften wenn die Außenluft kühler ist, RLT als Reserve."
    series_de = "Kühllast · Teil 6"
# endregion
