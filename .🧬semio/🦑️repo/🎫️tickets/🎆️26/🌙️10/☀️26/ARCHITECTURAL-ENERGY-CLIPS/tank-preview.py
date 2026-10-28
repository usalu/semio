"""🔋 Preview live fills after their tank frames move and scale."""

import importlib
import sys
from pathlib import Path

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))

from manim import Scene, Text, UP, config
import numpy as np

VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class TankTransformPreview(Scene):
    def construct(self):
        config.background_color = VISUALS.P_DEEP_DARK
        for x, level in ((-3, 1), (0, 0.5), (3, 0)):
            tank = VISUALS.energy_tank([-2.3, -1.45, 0], height=2.2, width=0.9, level=level)
            tank["group"].scale(0.85).move_to(np.array([x, -0.2, 0]))
            tank["fill"].update(0)
            self.add(tank["group"], tank["fill"])
            self.add(Text(f"{level:.0%}", font_size=28).move_to(np.array([x, 1.5, 0])))
        self.wait(0.2)
