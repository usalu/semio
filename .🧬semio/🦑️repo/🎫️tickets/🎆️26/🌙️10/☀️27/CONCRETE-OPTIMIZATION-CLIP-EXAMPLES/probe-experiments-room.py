"""📋 Measure the footprint of room_section for the four factorial designs at a card scale."""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
manim = importlib.import_module("manim")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")


class Probe(visuals.ClipScene):
    TITLE = {"en": "Probe", "de": "Probe"}
    NARRATION = [["a", "a", "b"]]

    def construct(self):
        fonts.apply_scene_style(self)
        for design in ({"wwr": .3, "overhang": 0}, {"wwr": .3, "overhang": 1}, {"wwr": .7, "overhang": 0}, {"wwr": .7, "overhang": 1}, {"wwr": .5, "overhang": .5}):
            trackers = self.study_trackers(design)
            for labels in (False, True):
                room = self.room_section(trackers, (0, 0, 0), 1.0, "summer", labels=labels, demand=False, person=False)
                left, right, bottom, top = room.get_left()[0], room.get_right()[0], room.get_bottom()[1], room.get_top()[1]
                print("[DEBUG]", design, "labels", labels, "x", round(left, 2), round(right, 2), "y", round(bottom, 2), round(top, 2), flush=True)


with manim.tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(TICKET / "🗑️generated/media"), "verbosity": "ERROR"}):
    Probe().render()
