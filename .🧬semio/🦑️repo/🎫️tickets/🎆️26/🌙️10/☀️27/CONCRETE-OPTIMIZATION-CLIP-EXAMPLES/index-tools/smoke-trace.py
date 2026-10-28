"""🚬 Real playback of the index scene with the current clause key and scene time printed to a trace file.

Usage: python smoke-trace.py <en|de> [start-method]  (start-method runs only that screen and the ones before are skipped)
"""
import importlib
import os
import sys
import time
import traceback
from pathlib import Path

TICKET = Path(__file__).resolve().parent.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language, *methods = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
OUTPUT = TICKET / "🗑️generated/smoke-trace"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
scene_class = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.⚖️index.🐍️").Beat4_NormalizedIndex
trace = OUTPUT / f"trace-{language}.txt"
trace.write_text("", encoding="utf-8")
_original = manim.Animation.interpolate_mobject


def _wrapped(self, alpha):
    try:
        _original(self, alpha)
    except ValueError:
        mob = self.mobject
        with open(trace, "a", encoding="utf-8") as handle:
            handle.write("FAIL " + type(self).__name__ + " on " + type(mob).__name__ + " family " + str(len(mob.get_family())) + " start " + str(len(self.starting_mobject.get_family())) + " kids " + str([type(m).__name__ + ":" + str(len(m.get_family())) + ":" + str(len(m.updaters)) for m in mob.submobjects][:6]) + chr(10))
        raise


manim.Animation.interpolate_mobject = _wrapped


class Traced(scene_class):
    def present_index(self):
        if not methods:
            return super().present_index()
        self.prepare()
        for method in methods:
            getattr(self, method)()

    def step(self, key, *animations, seconds=1.2):
        with open(trace, "a", encoding="utf-8") as handle:
            handle.write(f"{key} t={self.renderer.time:.1f}\n")
        return super().step(key, *animations, seconds=seconds)


started = time.time()
with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 3, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR", "progress_bar": "none"}):
    scene = Traced()
    try:
        scene.render()
        failed = False
    except Exception:
        traceback.print_exc()
        failed = True
print(f"[DEBUG] trace smoke {'FAILED' if failed else 'ok'} in {time.time() - started:.0f}s, scene time {scene.renderer.time:.1f}s", flush=True)
sys.exit(1 if failed else 0)
