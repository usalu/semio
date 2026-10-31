"""🎞️ Capture the midpoint frame of every animation of a Grasshopper or Excel scene, so morphs can be judged in motion.

Usage: python capture_morph.py <series> <topic-folder> <SceneClass> <en|de> [key ...]
Output: 🗑️generated/morph/<series>/<SceneClass>/<language>/<key>-<n>-mid.png (n counts the animations within a clause)
and <key>-end.png after the clause; index.md lists every frame with its clause and animation types.
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
series, topic, name, language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/{name}-{language}")
OUTPUT = TICKET / "🗑️generated/morph" / series / name / language
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
scene_class = getattr(importlib.import_module(f"🎓️teaching.🏛️architecture.{series}.🎬️clip.{topic}.🐍️"), name)
index = []


class MorphCapture(scene_class):
    """🖼️ Play every animation at a low frame rate and save its midpoint, plus the end state of each clause."""

    _mc_key, _mc_count, _mc_mid, _mc_saved = "intro", 0, None, True

    def _mc_save(self, tag):
        self.renderer.update_frame(self, ignore_skipping=True)
        self.renderer.camera.get_image().save(OUTPUT / f"{tag}.png")

    def play(self, *animations, **kwargs):
        self._mc_count += 1
        self._mc_saved = not (not only or self._mc_key in only)
        kinds = ", ".join(type(animation).__name__ for animation in animations)
        self._mc_mid = f"{self._mc_key}-{self._mc_count:02d}-mid"
        if not self._mc_saved:
            index.append(f"| {self._mc_key} | {self._mc_count} | {kinds} | {self._mc_mid}.png |")
        self.renderer._original_skipping_status = self._mc_saved
        return super().play(*animations, **kwargs)

    def update_to_time(self, t):
        super().update_to_time(t)
        if not self._mc_saved and t >= 0.5 * getattr(self, "duration", 0):
            self._mc_saved = True
            self._mc_save(self._mc_mid)

    def wait(self, *args, **kwargs):
        return None

    def step(self, key, *animations, **kwargs):
        if self._mc_key != key and (not only or self._mc_key in only) and self._mc_key != "intro":
            self._mc_save(f"{self._mc_key}-end")
        self._mc_key, self._mc_count = key, 0
        return super().step(key, *animations, **kwargs)


art.hold_for = lambda scene, localized, key: None


def _begin(scene, text):
    row = next((r for r in scene.NARRATION if text in (r[1], r[2])), None)
    if row is not None and isinstance(scene, MorphCapture) and scene._mc_key != row[0]:
        if (not only or scene._mc_key in only) and scene._mc_key != "intro":
            scene._mc_save(f"{scene._mc_key}-end")
        scene._mc_key, scene._mc_count = row[0], 0


_caption_bar, _swap_caption = art.caption_bar, art.swap_caption
art.swap_caption = lambda scene, bar, text, *a, **k: (_begin(scene, text), _swap_caption(scene, bar, text, *a, **k))[1]
with manim.tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 2, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": False,
                       "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    MorphCapture().render()
(OUTPUT / "index.md").write_text(f"# Morph frames {series} {name} {language}\n\n| clause | n | animations | frame |\n| --- | --- | --- | --- |\n" + "\n".join(index) + "\n", encoding="utf-8")
print("[DEBUG] morph frames", series, name, language, len(index), flush=True)
