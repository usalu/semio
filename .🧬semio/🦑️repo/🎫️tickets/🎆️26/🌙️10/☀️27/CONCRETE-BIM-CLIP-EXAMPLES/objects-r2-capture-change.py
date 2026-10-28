"""🔬 Capture the change beat at its widest diameter (the final undo is skipped): python objects-r2-capture-change.py <en|de>."""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language = sys.argv[1]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/objects-r2/fonts/change-{language}")
OUTPUT = TICKET / "🗑️generated/objects-r2/frames-change"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
scene_class = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧬️objects.🐍️").Beat2_ObjectOrientation


class Probe(scene_class):
    def _ramp(self, tracker, value, seconds):
        if value == 0.3:
            value = 0.4
        return super()._ramp(tracker, value, seconds)

    def step(self, key, *animations, seconds=1.2):
        row = next(row for row in self.NARRATION if row[0] == key)
        if self._caption is not None:
            self.remove(self._caption)
        self._caption = art.caption_bar(row[1] if self.language() == "en" else row[2])
        self.add(self._caption)
        if animations:
            self.play(*animations, run_time=.01)
        if key == "change":
            self.renderer.static_image = None
            self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
            self.renderer.get_frame()
            self.renderer.camera.get_image().save(OUTPUT / f"change-{language}.png")
            raise SystemExit(0)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    Probe().render()
