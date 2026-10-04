import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[7] / "tutorial"
sys.path.insert(0, str(ROOT))
from manim import *
from manim_fonts import apply_scene_style, scene_title


class M1TitleProbe(Scene):
    def construct(self):
        apply_scene_style(self)
        self.add(scene_title("Modul 1: Die Grundlagen der Bauphysik"))
