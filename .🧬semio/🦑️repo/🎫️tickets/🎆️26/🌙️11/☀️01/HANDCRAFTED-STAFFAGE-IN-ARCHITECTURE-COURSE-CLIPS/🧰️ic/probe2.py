import importlib
import numpy as np
from manim import Scene, Line
m = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
class Probe2(Scene):
    def construct(self):
        self.add(Line([-7, 0, 0], [7, 0, 0], color="#555555", stroke_width=1))
        g = m.fitted_staffage("physics-fundamentals-vacuum", height=3.0, rate=60.0, loop=True, window=(-0.8, -0.1, 1.4, 0.9))
        g.move_to([0, 0, 0]); self.add(g)
        self.wait(1.0)
