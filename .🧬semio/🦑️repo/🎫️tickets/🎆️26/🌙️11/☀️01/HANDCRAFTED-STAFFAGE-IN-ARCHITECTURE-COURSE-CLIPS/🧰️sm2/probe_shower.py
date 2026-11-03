"""sm2 probe: the shower cast running at 3 units per metre (metre grid), for the head and spray geometry: manim -s probe_shower.py ShowerStill."""
import importlib

import numpy as np
from manim import Rectangle, NumberPlane, Scene

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class ShowerStill(Scene):
    def construct(self):
        self.add(NumberPlane(x_range=(-4, 4), y_range=(-4, 4), background_line_style={"stroke_opacity": 0.35}))
        for i, t in enumerate((60.0, 61.3)):
            m = V.staffage_motion("physics-fundamentals-shower", unit=3.0, anchor=np.array([-1.5 + 3.0 * i, -3.0, 0.0]), rate=1.0)
            m.seek(t)
            self.add(m)
        self.add(Rectangle(width=0.8 * 3, height=0.65 * 3, color="#ff0000").move_to(np.array([0.0, -3.0 + 0.6 * 3 / 2 * 1.0, 0.0])))
