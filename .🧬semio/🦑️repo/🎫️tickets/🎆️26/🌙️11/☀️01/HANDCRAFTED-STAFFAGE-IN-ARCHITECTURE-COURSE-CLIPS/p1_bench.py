"""Benchmark scene: one thermal staffage_motion occupant plus three animated thermal devices for 10 s.

`manim -ql --fps 15 p1_bench.py Bench` from the workspace root with PYTHONPATH=. (quality l, 15 fps).
"""

import importlib

import numpy as np
from manim import DOWN, Scene

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")

CASTS = {
    "cooling-heating-comparison-occupant": (1.0, -4.5),
    "heating-calculation-radiator": (4.0, -1.5),
    "cooling-heating-comparison-toaster": (30.0, 1.5),
    "cooling-heating-comparison-laptop": (60.0, 4.5),
}


class Bench(Scene):
    def construct(self):
        V.ensure_casts(list(CASTS))
        for cast, (rate, x) in CASTS.items():
            self.add(V.staffage_motion(cast, unit=1.2, anchor=np.array([x, -2.5, 0.0]), rate=rate))
        self.wait(10)
