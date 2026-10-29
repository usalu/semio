"""🖼️ Render the Grasshopper canvas toolkit as 1280x720 stills (en, de) into 🗑️generated/gh-toolkit and run the strict layout checker on each.

Usage: python showcase.py [canvas|widgets ...]
"""
import importlib
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
OUT = TICKET / "🗑️generated/gh-toolkit"
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(TICKET))
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
strict = importlib.import_module("layout_strict")
np = importlib.import_module("numpy")

PAGES = sys.argv[1:] or ["canvas", "widgets"]
engine = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
OUTPUT = {"slug": ""}


class Showcase(visuals.ClipScene):
    """🖼️ One still: ribbon, canvas, viewport and the widget zoo."""

    NARRATION = [["still", "Canvas toolkit showcase.", "Canvas-Werkzeugkasten."]]

    def construct(self):
        visuals._fonts.apply_scene_style(self)
        self._caption = None
        page = OUTPUT["page"]
        getattr(self, "page_" + page)()
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / f"{page}-{self.language()}.png")
        findings = strict.strict_conflicts(self)
        conflicts = visuals._art.layout_conflicts(self)
        report = OUT / f"layout-{page}-{self.language()}.md"
        report.write_text(f"# {page} {self.language()}\n\n- shared: {conflicts or 'none'}\n" + strict.strict_section(findings), encoding="utf-8")
        print(f"[DEBUG] {page} {self.language()}: strict {len(findings)}, shared {len(conflicts)}", flush=True)

    def preview_box(self, view, width, depth, height):
        """🟩 A green preview box in the axonometric view of the viewport."""
        hw, hd = width / 2, depth / 2
        corners = {(i, j, k): view(x, y, z) for i, x in enumerate((-hw, hw)) for j, y in enumerate((-hd, hd)) for k, z in enumerate((0, height))}
        faces = [((0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1), 1.0), ((1, 0, 0), (1, 1, 0), (1, 1, 1), (1, 0, 1), 0.62), ((0, 0, 0), (1, 0, 0), (1, 0, 1), (0, 0, 1), 0.8)]
        group = manim.VGroup()
        for *ids, shade in faces:
            group.add(visuals._drafting.polygon([corners[i] for i in ids], visuals._canvas.mix(visuals._canvas.PREVIEW_FILL, "#000000", 1 - shade), 1.0, visuals._canvas.PREVIEW_EDGE, 2.0))
        return group

    def page_canvas(self):
        ribbon = self.gh_ribbon(tabs=visuals._canvas.RIBBON_TABS + ("Ladybug", "Kangaroo2"), active="Params", at=(0, 1.83), width=13.4, highlight="Number Slider",
                                groups=[("Input", ["Panel", "Number Slider", "Boolean Toggle", "Value List"]), ("Geometry", ["Point", "Curve", "Brep", "Box"]),
                                        ("Util", ["Param Viewer", "Data Dam"])])
        definition = engine.canvas("anatomy")
        canvas = self.gh_canvas(definition, engine.evaluate(definition), view="box", frame=(9.7, 3.64), at=(-1.95, -0.76))
        viewport = self.gh_viewport(at=(5.0, -0.76), width=3.6, height=3.64, label="Perspective")
        nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
        preview = self.capsule_axon(nakagin.capsule_types()[0], at=viewport.get_center() + np.array([0, 0.1, 0]), scale=0.62, base=visuals._canvas.PREVIEW_FILL, window=True)
        viewport.fit(preview, 0.55)
        self.add(canvas, ribbon, viewport, preview)

    def page_widgets(self):
        tracker = manim.ValueTracker(2.4)
        sheet = self.gh_background(at=(0, 0), width=13.2, height=5.0)
        slider = self.gh_slider("Height", 0, 1.0, 4.0, at=(-4.4, 1.8), tracker=tracker, width=3.1)
        toggle = self.gh_toggle(True, at=(-1.6, 1.8), name="Window")
        values = self.gh_value_list(["Type A", "Type B", "Type C"], 1, at=(1.0, 1.4), is_open=True)
        parameter = self.gh_param("Number", at=(4.1, 1.8), label="Num")
        error = self.gh_component({"name": "Number", "nickname": "N", "inputs": [{"name": "Text", "nickname": "T"}], "outputs": [{"name": "Number", "nickname": "N"}]},
                                  at=(-4.6, -0.55), state="error")
        selected = self.gh_component({"name": "Orient", "nickname": "Orient", "inputs": [{"nickname": "G"}, {"nickname": "A"}, {"nickname": "B"}],
                                      "outputs": [{"nickname": "G"}, {"nickname": "X"}]}, at=(-1.9, -0.55), selected=True)
        disabled = self.gh_component({"name": "Move", "nickname": "Move", "inputs": [{"nickname": "G"}, {"nickname": "T"}], "outputs": [{"nickname": "G"}]},
                                     at=(1.0, -1.45), state="disabled")
        viewer = self.gh_param_viewer({(0, 0): [1, 2], (0, 1): [3, 4, 5], (1, 0): [6]}, at=(4.0, -0.7))
        bubble = self.gh_bubble(self.tr("Cannot convert Type A to Number", "Type A lässt sich nicht in Number umwandeln"), level="error", target=error, width=3.4)
        wire = self.gh_wire(error.grip_out(), selected.grip_in("A"), kind="item", selected=True)
        long = self.gh_wire(selected.grip_out("G"), viewer.grip_in(), kind="tree")
        group = self.gh_group([error, selected], self.tr("Casting", "Umwandlung"), index=1)
        remark = self.gh_bubble(self.tr("Component disabled", "Deaktiviert"), level="remark", target=disabled, side="below", width=3.4)
        self.add(sheet, group, slider, toggle, values, parameter, error, selected, disabled, viewer, bubble, wire, long, remark)


def render(page: str, language: str):
    os.environ["SEMIO_TEACHING_LANGUAGE"] = language
    os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
    os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUT / f"fonts-{page}-{language}")
    OUTPUT["page"] = page
    with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                           "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
        Showcase().render()


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for page in PAGES:
        for language in ("en", "de"):
            render(page, language)
