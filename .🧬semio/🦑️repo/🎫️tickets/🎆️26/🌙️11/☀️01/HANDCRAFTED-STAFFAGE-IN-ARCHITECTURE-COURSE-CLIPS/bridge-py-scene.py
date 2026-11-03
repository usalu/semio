import importlib
import numpy as np
from manim import Scene, FadeIn, FadeOut, tempconfig, RIGHT
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
from pathlib import Path
SM = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
SM._MANIFEST = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/bridge-py-manifest.json")
class S(Scene):
    def construct(self):
        w = V.staffage_motion("walking", unit=0.5, anchor=np.array([-3, -2, 0]))
        w.add_updater(lambda m, dt: m.shift(RIGHT * dt))
        self.play(FadeIn(w), run_time=0.5)
        self.wait(1)
        print("[DEBUG] after wait frame", w.frame, w.get_center())
        self.play(w.animate.shift(RIGHT), run_time=0.5)
        print("[DEBUG] after animate", w.frame, w.get_center())
        self.play(FadeOut(w), run_time=0.5)
with tempconfig({"dry_run": True, "write_to_movie": False, "pixel_width": 320, "pixel_height": 180, "disable_caching": True, "quality": "low_quality", "verbosity": "WARNING"}):
    S().render()
print("[DEBUG] ok")
