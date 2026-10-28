"""📸 Save frames with the segment pointer on: after the matrix beat, the focus row is Punched; after index_2 it is the winner.

Usage: python seg-probe.py <en|de>   (writes 🗑️generated/frames-partial/seg-<language>-<case>.png)
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language = sys.argv[1]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
OUTPUT = TICKET / "🗑️generated/frames-partial"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
scene_class = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.⚖️index.🐍️").Beat4_NormalizedIndex


class Probe(scene_class):
    def present_index(self):
        self.prepare()
        self.screen_rulers()
        board = self.board
        for key, steps in (("index", (0, 1, 3)), ("winner", (0, 2))):
            self.screen_board_until(key)
            for j in steps:
                board.seg.set_value(j)
                board.seg_o.set_value(1.0)
                self.save(f"{key}-{j}")
            board.seg_o.set_value(0.0)

    def screen_board_until(self, key):
        board = self.board
        if key == "index":
            for tracker in (board.mode,):
                tracker.set_value(1.0)
            for c, v in zip(("electricity", "cost", "daylight", "carbon"), self._weights):
                board.w[c].set_value(v)
            board.compact.set_value(1.0)
            board.numbers.set_value(0.0)
            board.scores.set_value(1.0)
        else:
            board.focus.set_value(1)
            self._rt["wwr"].set_value(.5)
            self._rt["overhang"].set_value(.6)

    def step(self, key, *animations, seconds=1.2):
        if self._caption is not None:
            self.remove(self._caption)
        row = next(row for row in self.NARRATION if row[0] == key)
        self._caption = art.caption_bar(row[1] if self.language() == "en" else row[2])
        self.add(self._caption)
        if animations:
            self.play(*animations, run_time=.01)

    def save(self, case):
        self.update_mobjects(0)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"seg-{language}-{case}.png")


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    Probe().render()
print("[DEBUG] seg probe", language, flush=True)
