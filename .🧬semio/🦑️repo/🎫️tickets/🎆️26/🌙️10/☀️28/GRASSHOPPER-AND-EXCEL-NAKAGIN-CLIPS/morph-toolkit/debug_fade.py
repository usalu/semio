"""🔬 Check that a token source fades while morphing inside a rendering scene."""
import importlib
import sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
m = importlib.import_module("manim")
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")


class S(m.Scene):
    def construct(self):
        cell = morph.token("B2 = 12", kind="cell", size=2.0)
        dst = m.VGroup(m.Rectangle(width=1.5, height=0.8), m.Line(m.LEFT, m.RIGHT)).shift(3 * m.RIGHT)
        self.add(cell)
        self.anim = morph.morph(cell, dst)
        self.play(self.anim)

    def update_to_time(self, t):
        super().update_to_time(t)
        a = self.anim
        if getattr(a, "ghost_mobject", None) is not None:
            print("[DEBUG]", round(t, 2), [type(x).__name__ for x in self.mobjects], [round(float(x[0].get_fill_opacity()), 2) for x in a._leaving][:3], flush=True)


with m.tempconfig({"write_to_movie": False, "progress_bar": "none", "verbosity": "ERROR", "pixel_width": 320, "pixel_height": 180, "frame_rate": 5}):
    S().render()
