"""📸 Capture one PNG per narration beat of a Grasshopper or Excel scene, with layout conflict reports.

Usage: python capture.py <series> <topic-folder> <SceneClass> <en|de> [key ...]
Series: 🦗️grasshopper or 📊️excel
Example: python capture.py 🦗️grasshopper 🦴️anatomy Beat1_Anatomy en
Output: 🗑️generated/frames/<series>/<SceneClass>/<language>/<key>.png and layout.md
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(TICKET))
series, topic, name, language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/{name}-{language}")
OUTPUT = TICKET / "🗑️generated/frames" / series / name / language
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
strict = importlib.import_module("layout_strict")
scene_class = getattr(importlib.import_module(f"🎓️teaching.🏛️architecture.{series}.🎬️clip.{topic}.🐍️"), name)
report = []
tally = []


class Capture(scene_class):
    """🖼️ Replace timed beats by instant state changes and one saved frame per clause."""

    def snapshot(self, tag):
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"{tag}.png")

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
        self.renderer.camera.get_image().save(OUTPUT / f"{key}.png")
        conflicts = art.layout_conflicts(self)
        try:
            findings = strict.strict_conflicts(self)
        except Exception as error:
            findings = [f"strict check failed: {type(error).__name__}: {error}"]
            print("[DEBUG] strict check failed", key, repr(error), flush=True)
        tally.append(len(findings))
        report.append(f"## {key}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n" + strict.strict_section(findings))


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    Capture().render()
summary = f"strict findings: {sum(tally)} in {sum(1 for n in tally if n)} of {len(tally)} frames\n"
(OUTPUT / "layout.md").write_text(f"# Layout {series} {name} {language}\n\n{summary}\n" + "\n".join(report), encoding="utf-8")
print("[DEBUG] captured", series, name, language, len(report), "beats", summary.strip(), flush=True)
