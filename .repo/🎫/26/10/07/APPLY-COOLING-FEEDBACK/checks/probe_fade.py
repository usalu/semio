"""🔬 [DEBUG] Probe: does set_stroke-opacity animate dim ParametricFunction waves
that were Created individually and later re-parented into a scaled VGroup?"""
import numpy as np
from manim import *


def waves_at(x):
    out = VGroup()
    for i in range(2):
        x0 = x + (i - 0.5) * 0.2

        def _p(t, x0=x0, ph=i * 0.85):
            return np.array([x0 + np.sin(t * 5 + ph) * 0.08, t * 1.0, 0.0])

        out.add(ParametricFunction(_p, t_range=[0, 1], color=RED, stroke_width=3).set_opacity(0.8))
    return out


class Probe(Scene):
    def construct(self):
        left = waves_at(-2)
        right = waves_at(2)
        both = VGroup(*left, *right)
        self.play(LaggedStart(*[Create(w) for w in both], lag_ratio=0.05), run_time=0.5)
        group = VGroup(both)
        self.play(group.animate.scale(0.82).shift(UP * 0.5), run_time=0.5)
        self.play(*[w.animate.set_stroke(opacity=0.08) for w in both[2:]], run_time=0.5)
        self.wait(0.2)
