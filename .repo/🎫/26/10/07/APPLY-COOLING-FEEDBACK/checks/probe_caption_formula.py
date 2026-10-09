"""🔬 [DEBUG] Probe: why a 3-line caption under a formula panel is not flagged by the guard."""
from pathlib import Path as _Path
import sys as _sys

_REPO_ROOT = next(p for p in _Path(__file__).resolve().parents if (p / "tutorial" / "manim_fonts.py").is_file())
_sys.path.insert(0, str(_REPO_ROOT / "tutorial"))

from manim import *
from manim_visuals import caption_bar, math_panel, layout_conflicts, P_YELLOW, P_WHITE, P_TEAL
from manim_fonts import BODY_FONT_SIZE


class ProbeCaptionFormula(Scene):
    def construct(self):
        cap = caption_bar(
            "Die Sonne wandert von Ost über Süd nach West — jede Fassade zeichnet ihre Kurve:\n"
            "Ost am Morgen, Süd am Mittag, West am Nachmittag, Nord bleibt niedrig."
        )
        row, box, items = math_panel([
            ("i", r"I_{S,max}", P_YELLOW), (None, "=", P_WHITE),
            (None, r"\approx\,400–800", P_YELLOW), (None, r"\;[\mathrm{W/m^{2}}]", P_TEAL),
        ], size=BODY_FONT_SIZE, color=P_YELLOW)
        self.add(cap, row, box)
        lines = [m for m in cap.get_family() if isinstance(m, Text)]
        print(f"[DEBUG] caption Text mobjects: {len(lines)}")
        for m in lines:
            print(f"[DEBUG]   {getattr(m, 'text', '?')[:30]!r} y[{m.get_bottom()[1]:.2f},{m.get_top()[1]:.2f}] zone={getattr(m, '_layout_zone', '')!r}")
        print(f"[DEBUG] formula row y[{row.get_bottom()[1]:.2f},{row.get_top()[1]:.2f}] box y[{box.get_bottom()[1]:.2f},{box.get_top()[1]:.2f}]")
        for issue in layout_conflicts(self):
            print(f"[DEBUG] issue: {issue}")
        self.wait(0.1)
