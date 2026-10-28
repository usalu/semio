import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "de"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/math-probe")
OUT = TICKET / "🗑️generated/information-r2"
OUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")


class Probe(visuals.ClipScene):
    def construct(self):
        art.set_vo_language("de")
        row, items = art.math_row([("s", "s", art.P_YELLOW), (None, r"\,−\,", None), ("d", "Ø", art.P_ORANGE), (None, "=", None), ("w", "w", art.P_GREEN)], font_size=30)
        row.move_to([0, 1, 0])
        row2, _ = art.math_row([("s", r"4{,}75\,\mathrm{m}", art.P_YELLOW), (None, r"\,−\,", None), ("d", r"0{,}30\,\mathrm{m}", art.P_ORANGE), (None, "=", None), ("w", r"4{,}45\,\mathrm{m}", art.P_GREEN)], font_size=26)
        row2.move_to([0, 0, 0])
        row3, _ = art.math_row([("r", "R_{Glas}", art.P_CYAN), (None, "=", None), ("t", "R_{Wende}", art.P_YELLOW), (None, "−", None), ("b", "b_{Auto}", art.P_ORANGE), (None, "−", None), ("c", "c", art.P_ORANGE)], font_size=26)
        row3.move_to([0, -1, 0])
        self.add(row, row2, row3)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / "math-probe.png")
        print("[DEBUG] widths", row.width, row2.width, row3.width)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media-math"), "verbosity": "ERROR"}):
    Probe().render()
