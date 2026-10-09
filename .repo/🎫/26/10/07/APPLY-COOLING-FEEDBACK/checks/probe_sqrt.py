"""🔬 [DEBUG] Probe: \\sqrt and \\pi in the shared math typesetting."""
from pathlib import Path as _Path
import sys as _sys

_REPO_ROOT = next(
    p for p in _Path(__file__).resolve().parents
    if (p / "tutorial" / "manim_fonts.py").is_file()
)
_sys.path.insert(0, str(_REPO_ROOT / "tutorial"))

from manim import *
from manim_visuals import math_label, P_WHITE, P_YELLOW, P_BLUE, P_TEAL, P_CYAN, P_GREEN


class ProbeSqrt(Scene):
    def construct(self):
        self.camera.background_color = "#10131A"
        rows = VGroup(
            math_label(r"r = \sqrt{\frac{A}{\pi}}", size=40, color=P_YELLOW),
            math_label(
                rf"q_{{v,R}} = \frac{{\textcolor{{{P_YELLOW}}}{{\dot{{Q}}_{{S,tr}}}}}}"
                rf"{{\textcolor{{{P_GREEN}}}{{\rho_{{a}} \cdot c_{{p,a}}}} \cdot \textcolor{{{P_BLUE}}}{{Δθ}}}}"
                r"\;[\mathrm{m^{3}/s}]",
                size=40, color=P_WHITE,
            ),
            math_label(r"A = \frac{q_{v,R}}{v_{m}}\;[\mathrm{m^{2}}]", size=40, color=P_CYAN),
        ).arrange(DOWN, buff=0.7)
        self.add(rows)
