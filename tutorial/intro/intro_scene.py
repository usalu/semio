"""🏛️ Reusable NGS intro card for Manim tutorial videos.

Institute: Nachhaltige Gebäudesysteme (IEK), Leibniz Universität Hannover
https://www.iek.uni-hannover.de/ngs

Subclass ``NGSIntro`` and set ``topic_de`` / ``topic_explain_de`` / ``series_de``,
or render ``Demo_Intro_Kuehllast`` as a Sideview smoke test.
"""

from __future__ import annotations

from pathlib import Path

import numpy as np
from manim import *
from manim.utils.rate_functions import ease_in_out_sine, ease_out_cubic, smootherstep
from PIL import Image

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

ASSETS_DIR = Path(__file__).resolve().parent / "assets"
WELFENSCHLOSS_PNG = ASSETS_DIR / "welfenschloss (1) (1).png"
LEIBNIZ_MARK_PNG = ASSETS_DIR / "csm_leibniz-binaerzahlen_13f738b2c9 (1).png"


# region Image Stencils
def _stencil(path: Path, color: str, opacity: float = 1.0) -> np.ndarray:
    """🖌️ Recolor a transparent line-art PNG into a single-tone RGBA stencil.

    https://docs.manim.community/en/stable/reference/manim.mobject.types.image_mobject.ImageMobject.html
    """
    arr = np.array(Image.open(str(path)).convert("RGBA"), dtype=np.uint8)
    arr[..., 0:3] = np.array(ManimColor(color).to_int_rgb(), dtype=np.uint8)
    arr[..., 3] = (arr[..., 3].astype(np.float64) * opacity).round().astype(np.uint8)
    return arr


def _stencil_image(path: Path, color: str, opacity: float = 1.0, width: float = 1.0) -> ImageMobject:
    """🪪 Single tinted stencil sized to ``width``."""
    image = ImageMobject(_stencil(path, color, opacity))
    image.width = width
    return image


def _sliced_stencil(path: Path, color: str, opacity: float, width: float, slices: int = 30) -> Group:
    """🧩 Vertical stencil slices laid back edge-to-edge for a wipe-in reveal."""
    arr = _stencil(path, color, opacity)
    pixel_h, pixel_w = arr.shape[:2]
    height = width * pixel_h / pixel_w
    cuts = np.linspace(0, pixel_w, slices + 1).round().astype(int)

    group = Group()
    for left, right in zip(cuts[:-1], cuts[1:]):
        if right <= left:
            continue
        # 1px bleed removes hairline seams between neighbouring slices.
        stop = min(right + 1, pixel_w)
        piece = ImageMobject(arr[:, left:stop])
        piece.height = height
        piece.move_to(RIGHT * width * ((left + stop) / 2 / pixel_w - 0.5))
        group.add(piece)
    return group
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
        Text.set_default(font="Serif")

        # region Chrome
        frame = _screen_frame()
        ticks = _corner_ticks(frame[0])
        # endregion

        # region Watermark
        building = _sliced_stencil(WELFENSCHLOSS_PNG, P_WHITE, opacity=0.13, width=12.6)
        building.align_to(frame[0], DOWN).shift(UP * 0.3)
        sweep = Line(UP * (FRAME_H / 2 - 0.2), DOWN * (FRAME_H / 2 - 0.2), color=P_CYAN, stroke_width=2.6)
        sweep.move_to(LEFT * (FRAME_W / 2 - 0.35))
        # endregion

        # region Corner Mark
        mark = _stencil_image(LEIBNIZ_MARK_PNG, P_CYAN, opacity=0.95, width=0.98)
        mark.move_to(frame[0].get_corner(UL) + RIGHT * (0.8 + mark.width / 2) + DOWN * (0.7 + mark.height / 2))
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
                *[FadeIn(piece, rate_func=smootherstep) for piece in building],
                lag_ratio=0.028,
                run_time=2.6,
            ),
            sweep.animate(rate_func=ease_in_out_sine).shift(RIGHT * (FRAME_W - 0.7)),
            run_time=2.6,
        )

        self.play(
            AnimationGroup(
                FadeOut(sweep, shift=RIGHT * 0.5, run_time=0.7, rate_func=ease_out_cubic),
                FadeIn(mark, shift=DOWN * 0.22, run_time=1.1, rate_func=ease_out_cubic),
                lag_ratio=0.35,
            )
        )

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
    """❄️ Example intro for the Cooling demand series."""

    topic_de = "Kühllast"
    topic_explain_de = "Wärmequellen, Feuchte und Systemauslegung im Sommer."
    series_de = "Gebäudeenergie · Kühllast"


class Demo_Intro_Heizlast(NGSIntro):
    """🔥 Example intro for the Heating demand series."""

    topic_de = "Heizlast"
    topic_explain_de = "Transmission, Lüftung und Gewinne im Winterbetrieb."
    series_de = "Gebäudeenergie · Heizlast"


class Intro_HeatingVsCooling(NGSIntro):
    """⚖️ Intro for Cooling part 1 — Heizlast vs. Kühllast."""

    topic_de = "Heizlast vs. Kühllast"
    topic_explain_de = "Warum dieselben Gewinne im Sommer zur Kühllast werden."
    series_de = "Kühllast · Teil 1"


class Intro_InternalGains(NGSIntro):
    """💡 Intro for Cooling part 2 — Interne Lasten."""

    topic_de = "Interne Lasten"
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
# endregion
