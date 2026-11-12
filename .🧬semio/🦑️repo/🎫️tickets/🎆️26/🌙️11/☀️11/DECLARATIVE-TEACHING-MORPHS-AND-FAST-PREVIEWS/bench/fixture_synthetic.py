"""🧪 Synthetic morph benchmark scene: only plain Manim shapes and the public morph toolkit API, so it runs on any revision.

Content: 4 multi-part single morphs (capsule-like drawing to cell chip), a 24-part scatter to item tokens, a 24-to-1 gather
to a number badge, and a retext. Every morph is a real teaching category (concretize, scatter, gather, relabel).
"""
import importlib

import numpy as np
from manim import DOWN, LEFT, RIGHT, UP, Line, RoundedRectangle, Scene, Square, Text, VGroup

_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")


def _drawing(centre, colour):
    body = RoundedRectangle(width=1.6, height=0.7, corner_radius=0.12, stroke_color=colour, fill_color=colour, fill_opacity=0.35).move_to(centre)
    windows = [Square(0.18, stroke_color=colour, fill_color="#ffffff", fill_opacity=0.5).move_to(centre + np.array([dx, 0.05, 0])) for dx in (-0.5, 0.0, 0.5)]
    door = Line(centre + np.array([-0.7, -0.35, 0]), centre + np.array([0.7, -0.35, 0]), stroke_color=colour)
    return VGroup(body, *windows, door)


def _small(centre, colour):
    return VGroup(Square(0.22, stroke_color=colour, fill_color=colour, fill_opacity=0.5), Square(0.08, stroke_color="#ffffff", fill_color="#ffffff", fill_opacity=0.8)).move_to(centre)


class MorphBench(Scene):
    def construct(self):
        colours = ["#4C9AFF", "#36B37E", "#FFAB00", "#FF5630"]
        for i, colour in enumerate(colours):
            centre = np.array([-4.5 + 3.0 * i, 2.4, 0.0])
            source = _drawing(centre, colour)
            self.add(source)
            target = _morph.token("ABCD"[i], kind="cell", color=colour).move_to(centre + DOWN * 0.0)
            self.play(_morph.morph(source, target, run_time=1.4))
            self.wait(0.3)
        sources = [_small(np.array([-4.2 + 0.8 * (i % 12), 0.6 - 0.8 * (i // 12), 0.0]), colours[i % 4]) for i in range(24)]
        self.add(*sources)
        targets = [_morph.token(str(i), kind="item", color=colours[i % 4]).move_to(np.array([-4.2 + 0.8 * (i % 12), -1.4 - 0.8 * (i // 12), 0.0])) for i in range(24)]
        self.wait(0.3)
        self.play(_morph.scatter(sources, targets, lag=0.04, run_time=1.6))
        self.wait(0.3)
        total = _morph.token("24", kind="number", color="#4C9AFF").move_to(np.array([0.0, -3.0, 0.0]))
        self.play(_morph.gather(targets, total, lag=0.02, run_time=1.4))
        self.wait(0.3)
        a = Text("123", font_size=36).move_to(np.array([3.0, -3.0, 0.0]))
        b = Text("456", font_size=36).move_to(np.array([3.0, -3.0, 0.0]))
        self.add(a)
        self.play(_morph.retext(a, b, run_time=1.2))
        self.wait(0.5)
