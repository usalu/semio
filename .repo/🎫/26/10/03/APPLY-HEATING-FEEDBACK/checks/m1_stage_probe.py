import sys
from pathlib import Path
M1 = Path(__file__).resolve().parents[7] / "tutorial" / "energy" / "demand" / "Heating" / "1_introduction"
sys.path.insert(0, str(M1))
from manim import Scene
import scene_1 as m1


class StageProbe(Scene):
    def construct(self):
        m1.apply_scene_style(self)
        stage = m1._stage()
        stage["wall"]["tint"].set_fill(opacity=0.55)
        self.add(stage["group"])


class OccupantProbe(Scene):
    def construct(self):
        m1.apply_scene_style(self)
        self.add(m1._occupant(x=0.0).scale(2.6, about_point=[0, -1.25, 0]))
