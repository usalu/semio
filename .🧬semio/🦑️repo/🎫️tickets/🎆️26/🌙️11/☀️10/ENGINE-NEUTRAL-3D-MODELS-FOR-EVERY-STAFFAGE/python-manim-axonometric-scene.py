"""One real manim scene drawing an elevation and an axonometric cast of the same asset side by side through the Python consumer (the course manifest has no axonometric cast, so the manifest is replaced in memory)."""
import importlib
import numpy as np
from manim import Scene, FadeIn, Wait
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
base = {"asset": "standing-adult", "pose": "relaxed", "view": "dark-line", "theme": "dark", "scale": 50}
S._manifest = lambda: {"version": 1, "casts": {"scene-elevation": {**base, "viewpoint": "elevation"}, "scene-axonometric": {**base, "viewpoint": "axonometric"}}}

class ElevationAndAxonometric(Scene):
    def construct(self):
        left = S.staffage("scene-elevation", unit=1.0, anchor=np.array([-2.0, -1.5, 0.0]))
        right = S.staffage("scene-axonometric", unit=1.0, anchor=np.array([2.0, -1.5, 0.0]))
        self.play(FadeIn(left), FadeIn(right), run_time=1)
        self.wait(1)
