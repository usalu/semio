"""📸 Capture the exchange scene with placeholder stand-ins for the ORTHO widgets that have not landed yet (piloti_detail, slab_strip).

Usage: python exchange-capture.py <en|de> [key ...]
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/Beat5_Interfaces-{language}")
OUTPUT = TICKET / "🗑️generated/frames" / "Beat5_Interfaces"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
scene_class = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔌️exchange.🐍️").Beat5_Interfaces
report = []


def piloti_detail(self, point=(0, 0, 0), scale=0.5, *, model=None, dims=True, grow=1.0, **kw):
    """🧪 Placeholder column elevation and section circle."""
    h = 3.25 * scale * max(grow, 0.001)
    x, y, _ = point
    column = manim.Rectangle(width=0.3 * scale * 2, height=h, color=drafting.STRUCT, fill_opacity=0.4).move_to((x, y - 1.6 * scale + h / 2, 0))
    circle = manim.Circle(radius=0.15 * scale * 3, color=drafting.STRUCT).move_to((x + 1.5, y, 0))
    group = drafting.Drawing()
    group.add(column, circle)
    group.part("elevation", column)
    group.part("section", circle)
    return group


def slab_strip(self, point=(0, 0, 0), scale=0.5, *, collapse=0.0, **kw):
    """🧪 Placeholder slab strip."""
    x, y, _ = point
    t = 0.25 * scale * (1 - collapse) + 0.01
    slab = manim.Rectangle(width=9.5 * scale, height=t, color=drafting.STRUCT, fill_opacity=0.5).move_to((x, y, 0))
    group = drafting.Drawing()
    group.add(slab)
    group.at = lambda *a: [x + (a[0] - 10.75) * scale, y, 0]
    return group


for name, fn in (("piloti_detail", piloti_detail), ("slab_strip", slab_strip)):
    if not hasattr(visuals.ClipScene, name):
        setattr(visuals.ClipScene, name, fn)


class Capture(scene_class):
    """🖼️ Replace timed beats by instant state changes and one saved frame per clause."""

    def step(self, key, *animations, seconds=1.2):
        row = next(row for row in self.NARRATION if row[0] == key)
        if self._caption is not None:
            self.remove(self._caption)
        self._caption = art.caption_bar(row[1] if self.language() == "en" else row[2])
        self.add(self._caption)
        if animations:
            try:
                self.play(*animations, run_time=.01)
            except TypeError:
                bad = sorted({type(m).__name__ + ":" + str(m.__dict__.get("scale")) for top in self.mobjects for m in top.get_family() if not callable(getattr(m, "scale", None))})
                print("[DEBUG] float scale on", bad, flush=True)
                raise
        if only and key not in only:
            return
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"Beat5_Interfaces-{language}-{key}.png")
        conflicts = art.layout_conflicts(self)
        report.append(f"## {key}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n")
        print("[DEBUG] captured", key, flush=True)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                       "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(OUTPUT / f"media-{language}"), "verbosity": "ERROR"}):
    Capture().render()
(OUTPUT / f"Beat5_Interfaces-{language}-layout.md").write_text(f"# Layout Beat5_Interfaces {language}\n\n" + "\n".join(report), encoding="utf-8")
print("[DEBUG] done", len(report), "beats", flush=True)
