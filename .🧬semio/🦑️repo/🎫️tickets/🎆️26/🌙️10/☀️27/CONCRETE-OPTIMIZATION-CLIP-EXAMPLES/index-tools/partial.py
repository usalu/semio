"""📸 Capture only some screens of the index scene (after prepare); optional DEBUG_KEY bisects a failing step.

Usage: python partial.py <topic-folder> <SceneClass> <en|de> <method[,method]> [key ...]
"""
import importlib
import os
import sys
import traceback
from pathlib import Path

TICKET = Path(__file__).resolve().parent.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
topic, name, language, methods, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
OUTPUT = TICKET / "🗑️generated/frames-partial"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
scene_class = getattr(importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip." + topic + ".🐍️"), name)
report = []
_original_cleanup = manim.FadeOut.clean_up_from_scene


def _cleanup(self, scene):
    try:
        _original_cleanup(self, scene)
    except ValueError:
        mob = self.mobject
        with open(OUTPUT / "dbg.txt", "a", encoding="utf-8") as handle:
            handle.write("FadeOut failed: " + type(mob).__name__ + " family " + str(len(mob.get_family())) + " start " + str(len(self.starting_mobject.get_family())) + " updaters " + str(len(mob.updaters)) + " kids " + str([type(m).__name__ + ":" + str(len(m.get_family())) for m in mob.submobjects][:8]) + chr(10))
        raise


manim.FadeOut.clean_up_from_scene = _cleanup


def leaves(item):
    for sub in getattr(item, "animations", []):
        yield sub
        yield from leaves(sub)


class Capture(scene_class):
    """🖼️ Replace timed beats by instant state changes and one saved frame per clause."""

    def present_index(self):
        self.prepare()
        for method in methods.split(","):
            getattr(self, method)()

    def step(self, key, *animations, seconds=1.2):
        row = next(row for row in self.NARRATION if row[0] == key)
        if self._caption is not None:
            self.remove(self._caption)
        self._caption = art.caption_bar(row[1] if self.language() == "en" else row[2])
        self.add(self._caption)
        if animations:
            if key == os.environ.get("DEBUG_KEY"):
                for item in animations:
                    for sub in [item, *leaves(item)]:
                        try:
                            sub._setup_scene(self)
                            sub.begin()
                            sub.interpolate(.5)
                        except Exception:
                            with open(OUTPUT / "dbg.txt", "a", encoding="utf-8") as handle:
                                handle.write("failing " + type(sub).__name__ + " " + type(getattr(sub, "mobject", None)).__name__ + "\n" + traceback.format_exc()[-300:] + "\n")
                raise SystemExit
            self.play(*animations, run_time=.01)
        if key == os.environ.get("DUMP_KEY"):
            with open(OUTPUT / "dump.txt", "a", encoding="utf-8") as handle:
                for m in self.mobjects:
                    names = [getattr(sub, "text", "") for sub in m.get_family() if hasattr(sub, "text")]
                    handle.write(type(m).__name__ + " " + str(len(m.get_family())) + " " + str(names[:3]) + " id=" + str(getattr(m, "identifier", "")) + chr(10))
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
                       "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    Capture().render()
(OUTPUT / f"{name}-{language}-layout.md").write_text(f"# Layout {name} {language}\n\n" + "\n".join(report), encoding="utf-8")
print("[DEBUG] captured", name, language, len(report), "beats", flush=True)
