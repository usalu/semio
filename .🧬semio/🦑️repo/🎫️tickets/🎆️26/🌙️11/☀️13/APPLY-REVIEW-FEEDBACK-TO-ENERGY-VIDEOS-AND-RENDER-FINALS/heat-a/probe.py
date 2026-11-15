import importlib
import sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
from manim import *

F = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧲️fundamentals.🐍️")
C = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧱️conduction.🐍️")


class SpecProbe(Scene):
    def construct(self):
        F.apply_scene_style(self)
        stage = F._stage()
        self.add(stage["group"])
        spec = F._spectrum()
        self.add(spec["group"])
        self.add(F.caption_bar(F.subtitle_text(F.Beat4_Strahlung.NARRATION, "short")))


class LawProbe(Scene):
    def construct(self):
        F.apply_scene_style(self)
        stage = F._stage()
        self.add(stage["group"])
        self.add(F.chip(F.tr("x", "Der zweite Hauptsatz der Thermodynamik"), F.PASTEL_YELLOW, font_size=F.LABEL_FONT_SIZE).move_to(F._route_strip()))
        self.add(F.caption_bar(F.subtitle_text(F.Beat1_DreiWegeDerWaerme.NARRATION, "never")))


class ThermProbe(Scene):
    def construct(self):
        C.apply_scene_style(self)
        therms, marks, reads = [], VGroup(), []
        for x, val, color, side in ((-3.2, 40.0, C.PASTEL_RED, -1), (3.2, 0.0, C.PASTEL_BLUE, 1)):
            th = C.thermometer_glyph(np.array([x, -0.7, 0.0]), height=1.75, color=color, level=val / 40.0)
            self.add(th["group"], th["column"])
            for v in (0, 20, 40):
                y = -0.7 + 0.05 + 1.63 * v / 40.0
                marks.add(Line(np.array([x + side * 0.14, y, 0.0]), np.array([x + side * 0.26, y, 0.0]), color=C.P_PRIMARY, stroke_width=1.6),
                          C.math_label(rf"{v}", np.array([x + side * 0.5, y - 0.04, 0.0]), size=C.LABEL_FONT_SIZE - 2, color=C.P_PRIMARY))
            marks.add(C.math_label(r"\theta\text{ in }\mathrm{°C}", np.array([x, 1.35, 0.0]), size=C.LABEL_FONT_SIZE - 2, color=C.P_PRIMARY))
            marks.add(C.math_label(rf"{int(val)}\,\mathrm{{°C}}", np.array([x, -1.12, 0.0]), size=C.BODY_FONT_SIZE, color=color))
        self.add(marks)
