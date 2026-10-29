"""🖼️ Render every named sub-view of the five Grasshopper canvases as a 1280x720 still into 🗑️generated/gh-toolkit/views and run the strict layout checker.

Usage: python views.py [en|de] [canvas[/view] ...]
"""
import importlib
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
OUT = TICKET / "🗑️generated/gh-toolkit/views"
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(TICKET))
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
engine = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
strict = importlib.import_module("layout_strict")

FRAME = (13.4, 5.1)
STATE = {}
LANGUAGE = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] in ("en", "de") else "en"
ONLY = [a for a in sys.argv[1:] if a not in ("en", "de")]


class ViewScene(visuals.ClipScene):
    """🖼️ One canvas view in its frame."""

    NARRATION = [["still", "View.", "Ansicht."]]

    def construct(self):
        visuals._fonts.apply_scene_style(self)
        self._caption = None
        key, name = STATE["key"], STATE["view"]
        definition = engine.canvas(key)
        canvas = self.gh_canvas(definition, engine.evaluate(definition), view=name, frame=FRAME, at=(0, 0, 0))
        self.add(canvas)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / f"{key}-{name}-{self.language()}.png")
        findings = strict.strict_conflicts(self)
        sheet = canvas.sheet
        inside = all(m.get_left()[0] >= sheet.get_left()[0] - 1e-6 and m.get_right()[0] <= sheet.get_right()[0] + 1e-6 and m.get_bottom()[1] >= sheet.get_bottom()[1] - 1e-6
                     and m.get_top()[1] <= sheet.get_top()[1] + 1e-6 for m in canvas.objects.values())
        STATE["rows"].append((key, name, round(canvas.factor, 2), inside, findings))
        print(f"[DEBUG] {key}/{name} {self.language()}: factor {canvas.factor:.2f} inside {inside} strict {len(findings)}", flush=True)


def render(key, name):
    STATE.update(key=key, view=name)
    with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                           "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
        ViewScene().render()


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    os.environ["SEMIO_TEACHING_LANGUAGE"] = LANGUAGE
    os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
    os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUT / f"fonts-{LANGUAGE}")
    STATE["rows"] = []
    for key in engine.canvas_keys():
        for name in engine.canvas(key).views:
            if not ONLY or key in ONLY or f"{key}/{name}" in ONLY:
                render(key, name)
    lines = [f"# Views {LANGUAGE}", ""] + [f"- {k}/{v}: factor {f}, inside {i}, strict {len(x)}" + "".join(f"\n    - {c}" for c in x) for k, v, f, i, x in STATE["rows"]]
    (OUT / f"report-{LANGUAGE}.md").write_text("\n".join(lines), encoding="utf-8")
