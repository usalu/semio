import importlib
import sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
from manim import *

roadmap = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🗺️roadmap.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")


class Probe(Scene):
    def construct(self):
        fonts.apply_scene_style(self)
        names = list(roadmap.PICTOGRAMS)
        cols = 8
        for i, name in enumerate(names):
            x = (i % cols - (cols - 1) / 2) * 1.65
            y = 1.6 - (i // cols) * 1.65
            frame = Rectangle(width=1.6, height=1.6, stroke_width=1).move_to([x, y, 0])
            pic = roadmap.PICTOGRAMS[name]("#44CCDD", 1.2, 0.9, np.array([x, y + 0.1, 0.0]))
            label = fonts.body_text(name, font_size=fonts.LABEL_FONT_SIZE - 4, color="#FFFFFF").move_to([x, y - 0.65, 0])
            self.add(frame, pic, label)
        self.wait(0.1)
