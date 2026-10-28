"""🏃 Preview the four activity representations from the actual chapter helper."""

import importlib
from pathlib import Path
import sys

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))

from manim import Scene, Text, RIGHT, UP
import numpy as np

SOURCE = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🐍️")


class ActivityPreview(Scene):
    def construct(self):
        SOURCE.apply_scene_style(self)
        for x, kind, name, watts in ((-4.8, "sleep", "Schlafen", 80), (-1.7, "sit", "Büroarbeit", 100),
                                     (1.3, "walk", "Gehen", 200), (4.3, "sprint", "Hochleistungssport", 800)):
            figure, heat = SOURCE._activity_figure(kind)
            figure.shift(RIGHT * x + UP * -0.75)
            self.add(figure)
            self.add(Text(name, font_size=22).move_to([x, -1.17, 0]))
            self.add(SOURCE.math_label(rf"{watts}\,\mathrm{{W}}", np.array([x, 1.4, 0]), size=28, edge="center"))
        self.wait(0.2)
