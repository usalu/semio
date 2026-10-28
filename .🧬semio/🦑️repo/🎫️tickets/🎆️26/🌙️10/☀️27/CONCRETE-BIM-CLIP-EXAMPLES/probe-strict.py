"""🔬 Run the strict layout check on chosen keyframes and write the findings to 🗑️generated/probe/<Scene>-<lang>-strict.md.

Usage: python probe-strict.py <topic-folder> <SceneClass> <en|de> <key> [key ...]
"""
import importlib
import os
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(TICKET))
topic, name, language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/probe-{name}-{language}")
OUTPUT = TICKET / "🗑️generated/probe"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
strict = importlib.import_module("layout_strict")
domain = "🎯️optimization" if "OPTIMIZATION" in str(TICKET) else "🏗️bim"
scene_class = getattr(importlib.import_module(f"🎓️teaching.🏛️architecture.{domain}.🎬️clip." + topic + ".🐍️"), name)
report = []


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
        self.renderer.get_frame()
        (OUTPUT / "png").mkdir(exist_ok=True)
        self.renderer.camera.get_image().save(OUTPUT / "png" / f"{name}-{language}-{key}.png")
        started = time.time()
        found = strict.strict_conflicts(self)
        old = art.layout_conflicts(self)
        report.append(f"## {key} ({time.time() - started:.2f}s, texts shared/strict={len(art._visible_texts(self))}/{len(strict._collect(self)[0])}, shared={len(old)}, strict={len(found)})\n\n" + "\n".join("- " + c for c in found) + "\n")
        if os.environ.get("STRICT_DUMP"):
            labels, shapes = strict._collect(self)
            frame = strict.Frame()
            report.append("labels:\n" + "\n".join(f"- {type(l.mob).__name__} {l.text!r} {frame.span(l.box)} zone={l.zone!r}" for l in labels) + "\n")
            report.append(f"shapes: {len(shapes)}\n")
            import numpy as np
            focus = os.environ.get("STRICT_FOCUS")
            for label in [l for l in labels if focus and focus in l.text]:
                boxes = np.array([s.box for s in shapes])
                chip = strict._chip_of(label, shapes, boxes)
                report.append(f"focus {label.text!r} order={label.order} chip={None if chip is None else (type(chip.mob).__name__, frame.span(chip.box), chip.order, chip.fill, chip.stroke)}\n")
                for k in strict._nearby(boxes, strict._grown(chip.box if chip else label.box, .05)):
                    s = shapes[k]
                    p = s.samples
                    report.append(f"  - {type(s.mob).__name__} {frame.span(s.box)} order={s.order} fill={s.fill:.2f} stroke={s.stroke:.2f} n={len(p)} first=({p[0][0]:.2f},{p[0][1]:.2f}) last=({p[-1][0]:.2f},{p[-1][1]:.2f}) thin={s.thin}\n")


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / f"media-{name}-{language}"), "verbosity": "ERROR"}):
    Probe().render()
(OUTPUT / f"{name}-{language}-strict.md").write_text("\n".join(report), encoding="utf-8")
print("[DEBUG] probed", name, language, len(report), flush=True)
