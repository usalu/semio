"""📸 Capture chosen beats of Beat4_PartialModels (state carries over only through the beats listed).

Usage: python models-r2-capture.py <en|de> <beat,beat,...>
Output: 🗑️generated/models-r2/<language>-<key>.png and <language>-layout.md
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language, chosen = sys.argv[1], sys.argv[2].split(",")
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/models-r2-{language}")
OUTPUT = TICKET / "🗑️generated/models-r2"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
scene_class = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️").Beat4_PartialModels
report = []


class Capture(scene_class):
    def present_models(self):
        self._prepare()
        for key in chosen:
            getattr(self, "_" + key)()

    def step(self, key, *animations, seconds=1.2):
        row = next(row for row in self.NARRATION if row[0] == key)
        if self._caption is not None:
            self.remove(self._caption)
        self._caption = art.caption_bar(row[1] if self.language() == "en" else row[2])
        self.add(self._caption)
        if animations:
            self.play(*animations, run_time=.01)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"{language}-{key}.png")
        conflicts = art.layout_conflicts(self)
        report.append(f"## {key}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n")
        print(f"[DEBUG] {key}: {len(conflicts)} conflicts", flush=True)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / f"media-{language}"), "verbosity": "ERROR"}):
    Capture().render()
(OUTPUT / f"{language}-layout.md").write_text(f"# Layout {language}\n\n" + "\n".join(report), encoding="utf-8")
