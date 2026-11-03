"""G1 mini scene: the new pump and thermostatic valves of demand practice Beat 2 in their real layout (ticket-local, not part of the course)."""
import importlib.util
import sys
from pathlib import Path

import numpy as np
from manim import *

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
SOURCE = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py"
spec = importlib.util.spec_from_file_location("g1_practice", SOURCE)
practice = importlib.util.module_from_spec(spec)
sys.modules["g1_practice"] = practice
spec.loader.exec_module(practice)


class G1Practice(Scene):
    def construct(self):
        practice.apply_scene_style(self)
        xs = [-4.1, -2.4, -0.7, 1.0]
        floor_top = -0.35
        rads = [practice._heater(floor_top, 1.6, x - 0.62) for x in xs]
        x_in = [float(r.get_left()[0]) for r in rads]
        pump_c = np.array([-5.5, -0.55, 0.0])
        pump = practice.fitted_staffage("demand-practice-pump", width=0.6, height=0.6, rate=1.0, loop=True).move_to(pump_c)
        supply = Line(pump_c + RIGHT * 0.3, np.array([x_in[-1], -0.55, 0.0]), color=practice.WATER_HOT, stroke_width=3.2)
        risers = VGroup(*[Line(np.array([xi, -0.55, 0.0]), np.array([xi, float(r.get_bottom()[1]), 0.0]), color=practice.WATER_HOT, stroke_width=3.2) for xi, r in zip(x_in, rads)])
        tag = practice.body_text("Pumpe", font_size=practice.LABEL_FONT_SIZE - 2, color=practice.P_ORANGE).move_to(pump_c + UP * 0.58)
        opens = [ValueTracker(1.0) for _ in xs]
        valves = [practice._valve(np.array([xi, -0.28, 0.0]), op) for xi, op in zip(x_in, opens)]
        self.add(supply, risers, *rads, pump, tag, *opens, *valves)
        self.wait(1.5)
        self.play(*[op.animate.set_value(v) for op, v in zip(opens, (0.30, 0.45, 0.80, 1.0))], run_time=2.5)
        self.wait(1.0)
