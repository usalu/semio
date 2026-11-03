import importlib, sys
from pathlib import Path
import numpy as np
from manim import Camera, VGroup, RIGHT, LEFT, Scene, FadeIn, config
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
out = Path(sys.argv[1]); out.mkdir(parents=True, exist_ok=True)
unit = 1.2
row = VGroup(
    V.staffage("standing", unit=unit, anchor=np.array([-4.5, -2.5, 0]), color="#FFFFFF"),
    V.staffage("seated-reader", unit=unit, anchor=np.array([-2.5, -2.5, 0]), color="#FFFFFF"),
    V.staffage("radiator", unit=unit, anchor=np.array([-0.5, -2.5, 0])),
)
walk = V.staffage_motion("walking", unit=unit, anchor=np.array([2.5, -2.5, 0]), color="#FFD166")
walk.add_updater(lambda m, dt: m.shift(RIGHT * 0.5 * dt))
def shot(name, items):
    cam = Camera(pixel_width=1280, pixel_height=720, background_color="#101418")
    cam.capture_mobjects(items)
    from PIL import Image
    Image.fromarray(np.asarray(cam.get_image()).copy()).save(out / name)
shot("a.png", [row, walk])
for _ in range(6): walk.update(1/15)
shot("b.png", [row, walk])
print("frame", walk.frame)
