import importlib
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
from manim import *

F = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")


class ParenStill(Scene):
    def construct(self):
        F.apply_scene_style(self)
        lines = [
            F.body_text("Gebäudemodernisierungsgesetz (seit 7/2026 GModG)", font_size=F.BODY_FONT_SIZE, color=WHITE),
            F.body_text("Innentemperatur (20 °C) und Außentemperatur (0 °C) [Vergleich]", font_size=F.BODY_FONT_SIZE, color=WHITE),
            F.body_text("Zuluft (16 °C) · (Speichern) · Gedämmt (12 cm) ( ) [ ]", font_size=F.LABEL_FONT_SIZE, color=WHITE),
            F.body_text("Willems, Schild, Stricker (2024) · Bohne, Bohne (2026) [2024]", font_size=F.LABEL_FONT_SIZE, color=WHITE),
        ]
        group = VGroup(*lines).arrange(DOWN, buff=0.5).move_to(ORIGIN)
        self.add(group)
        self.wait(0.1)
