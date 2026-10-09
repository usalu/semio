"""🔬 [DEBUG] Probe: the layout guard must flag touching labels, a line through a label and a dot under a label."""
from pathlib import Path as _Path
import sys as _sys

_REPO_ROOT = next(p for p in _Path(__file__).resolve().parents if (p / "tutorial" / "manim_fonts.py").is_file())
_sys.path.insert(0, str(_REPO_ROOT / "tutorial"))

from manim import *
from manim_visuals import math_label, P_WHITE, P_RED


class ProbeGuard(Scene):
    def construct(self):
        a = Text("Büroarbeit", font_size=24).move_to(UP * 1.0)
        b = Text("80 W", font_size=24).next_to(a, DOWN, buff=0.0)
        c = Text("Interne Gewinne", font_size=24).move_to(DOWN * 1.0)
        line = Line(LEFT * 2 + DOWN * 1.0, RIGHT * 2 + DOWN * 1.0, color=P_WHITE)
        bulb = Dot(DOWN * 1.0 + LEFT * 0.9, radius=0.08, color=P_RED)
        clean = math_label(r"\dot{Q}_{K}", size=30).move_to(RIGHT * 4 + UP * 2)
        self.play(FadeIn(VGroup(a, b, c, line, bulb, clean)), run_time=0.2)
