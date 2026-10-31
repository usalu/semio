"""🧪 Construct shared Nakagin drawing assemblies and inspect physical depth."""
import importlib
import numpy as np
from pathlib import Path
import sys
sys.path.insert(0, str(Path.cwd()))
v = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
s = v.NakaginDrawing()
a_study = [part.role for part, _ in v._cap_parts(v._cap(v._model.capsule_type("A")), "full", "study")]
c_study = [part.role for part, _ in v._cap_parts(v._cap(v._model.capsule_type("C")), "full", "study")]
assert "bed" in a_study and "sofa" not in a_study
assert "sofa" in c_study and "bed" not in c_study
print("[DEBUG] distinct program-study geometry retained")
c = s.capsule_axon(v._model.capsule_types()[0],cutaway=True,bolts=True)
assert c.semantic_parts and c.window.submobjects
print("[DEBUG] capsule", len(c), len(c.semantic_parts))
from manim import config, Camera, VGroup
config.pixel_width, config.pixel_height = 1920, 1080
config.frame_width, config.frame_height = 16, 9
camera = Camera(background_color=v.BACKGROUND)
c.scale(.9).move_to((-5.0, 1.9, 0))
core = s.core_axon(0, scale=.105, at=(-5.2, -2.0, 0))
podium = s.podium_axon(scale=.065, at=(-2.5, -2.0, 0))
capital = s.capital_axon(0, scale=.25, at=(-2.3, .0, 0))
t = s.tower_axon(scale=.12, at=(2.8, 0, 0), ground=True)
assert len(t.capsules) == 140
for shape in t:
    if hasattr(shape,'world_depth'):
        assert np.isfinite(shape.world_depth)
        assert abs(shape.world_depth-v._order(t.view,np.asarray(shape.world_vertices).mean(axis=0)))<1e-8
camera.capture_mobjects([VGroup(c, core, capital, podium, t)])
output = Path(__file__).parent / "🗑️generated"
output.mkdir(exist_ok=True)
camera.get_image().save(output / "nakagin-detail-gallery.png")
print("[DEBUG] gallery saved")
class Sheet(v.NakaginDrawing, v._d.Drafting):
    language = "en"
    def number(self, value, digits=2):
        return f"{value:.{digits}f}"
sheet = Sheet()
views = sheet.capsule_views(v._model.capsule_types()[0], at=(0,0,0), scale=.65, dims=True)
assert views.parts["plan"].semantic_parts
camera.reset()
camera.capture_mobjects([views])
camera.get_image().save(output / "nakagin-orthographic-gallery.png")
print("[DEBUG] orthographic saved")
