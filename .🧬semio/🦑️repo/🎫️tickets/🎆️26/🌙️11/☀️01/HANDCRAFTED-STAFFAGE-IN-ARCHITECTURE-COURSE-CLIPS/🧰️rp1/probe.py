"""rp1 probe: still of staffage casts on a unit grid: probe.py <Scene> (render with manim -s)."""
import importlib

import numpy as np
from manim import DOWN, UP, Dot, Line, NumberPlane, Scene, Text

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class Radiator(Scene):
    def construct(self):
        self.add(NumberPlane(x_range=(-7, 7), y_range=(-4, 4), background_line_style={"stroke_opacity": 0.25}))
        g = V.staffage("physics-fundamentals-radiator-front", unit=1.2, anchor=np.array([0.0, -1.0, 0.0]))
        self.add(g)
        print("RAD", g.get_left(), g.get_right(), g.get_bottom(), g.get_top(), V.staffage_height("physics-fundamentals-radiator-front"))


class Kettle(Scene):
    def construct(self):
        self.add(NumberPlane(x_range=(-7, 7), y_range=(-4, 4), background_line_style={"stroke_opacity": 0.25}))
        for i, (cast, unit) in enumerate((("physics-fundamentals-kettle", 4.0), ("physics-fundamentals-pot", 6.0),
                                          ("physics-fundamentals-hob", 6.0))):
            m = V.staffage_motion(cast, unit=unit, anchor=np.array([-4.0 + 4.0 * i, -2.0, 0.0]), rate=60.0)
            self.add(m)
            m.update(2.0)
            print(cast, m.get_left(), m.get_right(), m.get_bottom(), m.get_top())
