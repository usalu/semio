import importlib
import numpy as np
from manim import Scene, Line, Rectangle, VGroup, RIGHT, UP, config
m = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
W = (-0.02, 0.5, 0.24, 0.83)
class Probe(Scene):
    def construct(self):
        self.add(Line([-7, 0, 0], [7, 0, 0], color="#555555", stroke_width=1))
        for i, (c, kw) in enumerate([("vacuum", {"loop": True, "window": (-0.59, 0.0, 0.4, 0.84)}), ("kettle", {}), ("shower", {"window": W})]):
            g = m.fitted_staffage("physics-fundamentals-" + c, width=0.8, height=0.6, rate=0.0, **kw)
            g.move_to(np.array([-3 + i * 3, 0.0, 0.0]))
            print(c, g.width, g.height, g.get_center())
            self.add(g, Rectangle(width=0.8, height=0.6, stroke_width=1, color="#ff0000").move_to([-3 + i * 3, 0, 0]))
