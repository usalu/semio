"""🔬 Diagnose why layout_conflicts reports nothing: census of mobjects per kept keyframe.

Usage: python probe-layout.py <topic-folder> <SceneClass> <en|de> <key> [key ...]
"""
import importlib
import os
import sys
from collections import Counter
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
topic, name, language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/probe-{name}-{language}")
OUTPUT = TICKET / "🗑️generated/probe" / name
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
domain = sys.argv[0] and ("🎯️optimization" if "OPTIMIZATION" in str(TICKET) else "🏗️bim")
scene_class = getattr(importlib.import_module(f"🎓️teaching.🏛️architecture.{domain}.🎬️clip." + topic + ".🐍️"), name)


def census(scene, tag):
    from manim import Text, VMobject, ImageMobject, Group, VGroup
    top = Counter(type(m).__name__ for m in scene.mobjects)
    fam = [s for m in scene.mobjects for s in m.get_family()]
    texts = [s for s in fam if isinstance(s, Text)]
    visible = [t for t in texts if t.width > 0.01 and t.get_fill_opacity() >= 0.05]
    zero_op = [t for t in texts if t.get_fill_opacity() < 0.05]
    texty = [s for s in fam if type(s).__name__ not in ("VMobject", "VGroup", "Group") and hasattr(s, "text")]
    kinds = Counter(type(s).__name__ for s in fam)
    imgs = [s for s in fam if isinstance(s, ImageMobject)]
    updaters = [s for s in fam if s.updaters]
    print(f"[DEBUG] {tag} top={dict(top)}")
    print(f"[DEBUG] {tag} family={len(fam)} texts={len(texts)} visible={len(visible)} zero_opacity={len(zero_op)} images={len(imgs)} updaters={len(updaters)}")
    print(f"[DEBUG] {tag} kinds={dict(kinds)}")
    roots, glyphs = art._label_roots(scene)
    from collections import defaultdict
    members = defaultdict(int)
    for i in roots.values():
        members[i] += 1
    print(f"[DEBUG] {tag} label roots={len(members)} biggest={sorted(members.values())[-5:]}")
    zones = Counter(art._zone(s) for s in fam)
    print(f"[DEBUG] {tag} zones={dict(zones)}")
    gfx = art._visible_graphics(scene, roots, glyphs)
    print(f"[DEBUG] {tag} graphics={len(gfx)}")
    print(f"[DEBUG] {tag} layout_conflicts={art.layout_conflicts(scene)}")
    return visible


class Probe(scene_class):
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
        census(self, f"{key}/before-update")
        self.update_mobjects(0)
        census(self, f"{key}/after-update")
        for t in census(self, f"{key}/final"):
            print(f"[DEBUG] {key} text {t.text!r} box=({t.get_left()[0]:.2f},{t.get_right()[0]:.2f},{t.get_bottom()[1]:.2f},{t.get_top()[1]:.2f}) h={t.height:.3f}")


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / f"media-{language}"), "verbosity": "ERROR"}):
    Probe().render()
