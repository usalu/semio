import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[7] / "tutorial"
sys.path.insert(0, str(ROOT))
from manim import *
from manim_fonts import apply_scene_style
from manim_visuals import math_text, math_row, formula_panel, P_CYAN, P_RED, P_YELLOW

class MathProbe(Scene):
    def construct(self):
        apply_scene_style(self)
        rows = VGroup(
            math_text(r"\dot{Q}_{V} = 0{,}34\,\frac{\mathrm{Wh}}{\mathrm{m^{3}\,K}} \cdot \dot{V} \cdot \Delta T", font_size=31),
            math_text(r"g \approx 9{,}81\,\mathrm{\frac{m}{s^{2}}}", font_size=31, color=P_YELLOW),
            math_text(r"1\,\mathrm{kWh} = 1\,000\,\mathrm{W} \cdot 3\,600\,\mathrm{s} = 3{,}6 \cdot 10^{6}\,\mathrm{J}", font_size=31),
            math_text(r"\mathrm{COP} = \frac{Q_{\mathrm{H}}}{W_{\mathrm{el}}} = \frac{4\,\mathrm{kWh}}{1\,\mathrm{kWh}} = 4", font_size=31, color=P_CYAN),
            math_text(r"x_{s}(20\,\mathrm{°C}) \approx 14{,}7\,\mathrm{g/kg} \quad \text{kW}_{\mathrm{el}}", font_size=22, color=P_RED),
        ).arrange(DOWN, buff=0.45, aligned_edge=LEFT).move_to(UP * 0.8)
        row, items = math_row([("e", r"E", P_YELLOW), (None, "=", None), ("p", r"P", P_CYAN), (None, r"\cdot", None), ("t", r"t", P_RED)])
        row, box = formula_panel(row)
        self.add(rows, row, box)
