"""📸 Capture one PNG per narration beat of an optimization scene, with layout conflict reports.

Usage: python capture.py <topic-folder> <SceneClass> <en|de> [key ...]
Example: python capture.py 📉️extrema Beat1_LocalGlobalExtrema en
Output: 🗑️generated/frames/<SceneClass>-<language>-<key>.png and <SceneClass>-<language>-layout.md
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
topic, name, language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/toolkit-labels/fonts-capture/{name}-{language}")
OUTPUT = TICKET / "🗑️generated/toolkit-labels/frames" / name
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
scene_class = getattr(importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip." + topic + ".🐍️"), name)
report = []


class Capture(scene_class):
    """🖼️ Replace timed beats by instant state changes and one saved frame per clause."""

    def step(self, key, *animations, seconds=1.2):
        row = next(row for row in self.NARRATION if row[0] == key)
        if self._caption is not None:
            self.remove(self._caption)
        self._caption = art.caption_bar(row[1] if self.language() == "en" else row[2])
        self.add(self._caption)
        if animations:
            self.play(*animations, run_time=.01)
        if only and key not in only:
            return
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"{name}-{language}-{key}.png")
        conflicts = art.layout_conflicts(self)
        report.append(f"## {key}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n")


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / f"media-{language}"), "verbosity": "ERROR"}):
    Capture().render()
(OUTPUT / f"{name}-{language}-layout.md").write_text(f"# Layout {name} {language}\n\n" + "\n".join(report), encoding="utf-8")
print("[DEBUG] captured", name, language, len(report), "beats", flush=True)
