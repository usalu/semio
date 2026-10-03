import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[7] / "tutorial"
sys.path.insert(0, str(ROOT))
from manim import *
from manim_fonts import apply_scene_style
from manim_visuals import (math_label, math_readout, math_panel, de_num,
                           P_CYAN, P_RED, P_YELLOW, P_GREEN, P_ORANGE, P_WHITE)


class HeatingMathProbe(Scene):
    def construct(self):
        apply_scene_style(self)
        rows = VGroup(
            math_label(r"\Phi_{\mathrm{V}} = V \cdot n \cdot (1 - \eta_{\mathrm{WRG}}) \cdot c_{\mathrm{Luft}} \cdot \Delta\theta", size=31),
            math_label(r"R_{\mathrm{ges}} = R_{\mathrm{si}} + \Sigma\,\frac{d}{\lambda} + R_{\mathrm{se}} \quad U = \frac{1}{R_{\mathrm{ges}}}\,\left[\frac{\mathrm{W}}{\mathrm{m^{2}\,K}}\right]".replace(r"\left[", "[").replace(r"\right]", "]"), size=31, color=P_CYAN),
            math_label(r"H_{\mathrm{T}} = \Sigma\,U_{i} A_{i} + \Sigma\,\Psi_{k}\, l_{k} + \Sigma\,\chi_{j}", size=31, color=P_YELLOW),
            math_label(r"\dot{Q} = U \cdot A \cdot \Delta\theta = 1{,}43 \cdot 25\,\mathrm{m^{2}} \cdot 20\,\mathrm{K} = 715\,\mathrm{W}", size=28, color=P_ORANGE),
        ).arrange(DOWN, buff=0.4, aligned_edge=LEFT).move_to(UP * 1.2)
        t = ValueTracker(0.0)
        live = math_readout(lambda: rf"Q_{{\mathrm{{h}}}} = {de_num(t.get_value())}\,\mathrm{{kWh/a}}", np.array([-2.0, -1.0, 0.0]), size=26, color=P_RED)
        row, box, items = math_panel([("phi", r"\Phi_{\mathrm{solar}}", P_WHITE), (None, "=", None), ("g", "G", P_YELLOW), (None, r"\cdot", None), ("ff", r"F_{\mathrm{f}}", P_ORANGE), (None, r"\cdot", None), ("gv", "g", P_GREEN), (None, r"\cdot", None), ("fsh", r"F_{\mathrm{sh}}", P_CYAN), (None, r"\;[\mathrm{W}]", P_WHITE)])
        self.add(rows, live, row, box)
        self.play(t.animate.set_value(10812), run_time=1)
