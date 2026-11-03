import importlib, sys, os
from pathlib import Path
root = Path(__file__).resolve()
while not (root / "🧰️framework").is_dir():
    root = root.parent
sys.path.insert(0, str(root))
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
from manim import Scene, tempconfig, VGroup, RIGHT, Line
out = Path(__file__).parent / "🗑️generated"
kinds = sys.argv[1:] or ["laptop", "monitor", "bulb", "toaster", "vacuum", "heater"]
with tempconfig({"pixel_width": 1280, "pixel_height": 720, "background_color": "#101418", "media_dir": str(out / "media"), "save_last_frame": True, "write_to_movie": False, "disable_caching": True}):
    class S(Scene):
        def construct(self):
            row = VGroup(*[V.watt_anchor(w, compare=k, title=k) for k, w in zip(kinds, (60, 30, 100, 1000, 1400, 2000))]).arrange(RIGHT, buff=0.2).scale(0.6).to_edge(__import__("manim").UP)
            rad = V.radiator([-4, -1.5, 0], height=1.3, width=2.6)
            floor = Line([-6, -1.5 - 0.0, 0], [6, -1.5, 0])
            box = Line([-5.3, -1.5 + 1.3, 0], [-2.7, -1.5 + 1.3, 0], color="#444444")
            sun = V.sun_glyph([3, -1.2, 0])
            moon = V.moon_glyph([5, -1.2, 0])
            self.add(row, rad, floor, box, sun, moon)
    S().render()
    print(list((out / "media").rglob("*.png")))
