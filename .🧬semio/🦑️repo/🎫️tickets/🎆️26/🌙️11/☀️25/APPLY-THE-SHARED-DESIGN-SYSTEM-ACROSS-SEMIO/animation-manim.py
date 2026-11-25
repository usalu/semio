"""🎞️ Render the maintained Grasshopper ribbon and Manim window-chip painter."""

from importlib import import_module
from pathlib import Path
import sys

from manim import Scene, Text, config, tempconfig

sys.path.insert(0, str(Path.cwd()))
chapter = import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
visuals = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class Chrome(Scene):
    def construct(self):
        chapter._fonts.register_fonts()
        source = chapter.ClipScene.__new__(chapter.ClipScene)
        self.add(source.gh_ribbon(active="Params", at=(0, 1.15, 0),
                                  groups=[("Primitive", ["Point", "Line"]), ("Geometry", ["Curve", "Surface"])]))
        self.add(visuals.window_card(Text("Meter", font_size=26), Text("4 kWh", font_size=24), width=3).shift([0, -1, 0]))
        self.add(visuals.card_grid([("Heat", "4 kWh"), ("Power", "2 kW"), ("Time", "2 h")], rows=2, cols=2).shift([0, -2.55, 0]))


if __name__ == "__main__":
    destination = Path(sys.argv[1]).resolve()
    destination.mkdir(parents=True, exist_ok=True)
    with tempconfig({"media_dir": str(destination), "text_dir": str(destination / "t"),
                     "images_dir": str(destination), "save_last_frame": True, "write_to_movie": False,
                     "pixel_width": 960, "pixel_height": 540, "output_file": "chrome", "verbosity": "WARNING"}):
        config.background_color = visuals.P_DEEP_DARK
        Chrome().render()
