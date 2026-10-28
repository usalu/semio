"""🧪 Capture real decision-scene keyframes in both authored languages."""
import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-OPTIMIZATION-VIDEOS/🗑️generated/decision-fonts")
OUTPUT = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-OPTIMIZATION-VIDEOS/🗑️generated/decision-final"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
topics = [("⚖️index", "Beat4_NormalizedIndex"), ("🏆️pareto", "Beat5_ParetoFront"), ("🧬️algorithms", "Beat6_EvolutionarySearch")]
for language in ("en", "de"):
    os.environ["SEMIO_TEACHING_LANGUAGE"] = language
    for topic, name in topics:
        module = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip." + topic + ".🐍️")
        scene_class = getattr(module, name)
        class Capture(scene_class):
            def step(self, key, *animations, seconds=1.2):
                if key.rsplit("_", 1)[-1].isdigit():
                    return
                row = next(row for row in self.NARRATION if row[0] == key)
                art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
                if self._caption is not None:
                    self.remove(self._caption)
                self._caption = art.caption_bar(row[1] if self.language() == "en" else row[2])
                self.add(self._caption)
                if animations:
                    self.play(*animations, run_time=.01)
                self.renderer.static_image = None
                self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
                self.renderer.get_frame()
                self.renderer.camera.get_image().save(OUTPUT / (name + "-" + language + "-" + key + ".png"))
        with manim.tempconfig({"pixel_width":1280, "pixel_height":720, "frame_rate":1, "write_to_movie":False,
                               "save_last_frame":False, "disable_caching":True, "skip_animations":True,
                               "media_dir":str(OUTPUT / "media"), "verbosity":"ERROR"}):
            Capture().render()
        print("[DEBUG] Captured",name,language,flush=True)
