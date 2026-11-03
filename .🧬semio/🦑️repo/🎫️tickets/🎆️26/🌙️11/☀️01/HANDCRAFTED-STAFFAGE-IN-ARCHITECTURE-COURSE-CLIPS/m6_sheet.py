"""M6: stills of the fundamentals helpers (lamp, vacuum, persons, radiator) and the certificate sleeper at clip scale."""
import importlib
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
import numpy as np
import manim

F = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
C = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🏷️certificate.🐍️")


class Sheet(manim.Scene):
    def construct(self):
        self.camera.background_color = "#101418"
        floor = -2.0
        self.add(manim.Line((-7, floor, 0), (7, floor, 0), color="#555555"))
        self.add(F._person("physics-fundamentals-person", (-5.5, floor, 0), 1.72))
        self.add(F._person("physics-fundamentals-clerk", (-3.8, floor, 0), 1.0, motion=False))
        self.add(F._radiator("physics-fundamentals-radiator", (-2.3, floor, 0), 0.855))
        self.add(F._vacuum(1.8).shift(np.array([0.0, floor + 0.4, 0.0])))
        lamp = F._lamp((3.0, 1.5, 0))
        self.add(lamp["group"], manim.Dot(lamp["bulb"].get_center(), radius=0.03, color="#FF0000"))
        hs = manim.VGroup()
        sleeper = C.staffage_motion("demand-certificate-sleeper", unit=0.6)
        sleeper.move_to((5.2, floor + 0.4, 0))
        self.add(sleeper)
