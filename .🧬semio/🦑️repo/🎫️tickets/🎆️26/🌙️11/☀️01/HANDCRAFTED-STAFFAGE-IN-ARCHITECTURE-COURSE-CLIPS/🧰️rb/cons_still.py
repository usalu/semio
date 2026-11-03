"""🧑 rb: consistency person call sites at their authored coordinates, without the model geometry."""
import importlib
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = sys.argv[1] if len(sys.argv) > 1 else "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(HERE.parent / "🗑️generated/fonts/rb")
import manim

c = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔗️consistency.🐍️")
info = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.📊️information.🐍️")
d = c._d
E = c._engine


class Still(c.Beat6_Consistency):
    def construct(self):
        c._art.set_vo_language(self.language())
        c._visuals._fonts.apply_scene_style(self)
        self.camera.background_color = "#101418"
        for x, title, color in ((-3.8, "edit A · person 1", c.ARCH), (3.8, "edit B · person 2", c.STRUCT)):
            y = -0.7
            card = self.panel(5.0, 2.65, (x, y, 0), color)
            self.add(card, self.person((x - 2.1, y + 0.74, 0), color, 0.4), self.text(title, (x - 1.75, y + 0.98, 0), size=c.S_LABEL, color=color, anchor="l"),
                     self.text("r2 · paving 0.12 m *", (x - 2.3, y + 0.5, 0), size=c.S_LABEL, color=c.INK, anchor="l"))
        self.add(self.person((0.0, -1.0, 0), c.GOOD, 0.6, "bim-consistency-decider"), self.label("a person decides", "eine Person entscheidet", (0.0, -1.5, 0), size=c.S_LABEL, color=c.GOOD, width=2.5))
        card = self.panel(6.4, 1.45, (3.7, 2.2), c.STRUCT)
        self.add(card, self.label("review · checked by a person", "Prüfung · von einer Person geprüft", (1.05, 0.86 + 1.9, 0), size=c.S_LABEL, color=c.GOOD, anchor="l"),
                 self.person((0.55, 0.83 + 1.9, 0), c.GOOD, 0.4))
        probe = info.Beat1_DataToInformation()
        probe._c = {"car_length": 4.6, "car_width": 1.75}
        k = info.BAY_K
        for xa, xb in ((-4.0, -2.0),):
            for x in (xa, xb):
                self.add(E.Circle(radius=0.2 * k * 2, stroke_color=c.STRUCT).move_to((x, -3.2, 0)))
            self.add(probe._car(k, 270).move_to(((xa + xb) / 2, -3.2, 0)))


out = HERE.parent / "🗑️generated" / "rb" / "cons"
with manim.tempconfig({"pixel_width": 1600, "pixel_height": 900, "media_dir": str(out), "verbosity": "WARNING", "disable_caching": True, "save_last_frame": True, "write_to_movie": False}):
    Still().render()
print("[DEBUG] done")
