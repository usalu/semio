"""🎞️ Render a concise architectural overview from actual authored scene helpers."""

import importlib
import os
import sys
from pathlib import Path

workspace = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(workspace))
output = Path(__file__).parent / "🗑️generated" / "overview"
output.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(output)

from manim import Scene, VGroup, LEFT, RIGHT, UP, ORIGIN, tempconfig
visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
heating = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️")

class ArchitecturalOverview(Scene):
    def construct(self):
        plant = heating._plant_house()
        keys = ("room", "cellar", "walls", "roof", "windows", "radiators", "thermostat", "store", "boiler", "chimney", "pipes", "person")
        group = VGroup(*(plant[key] for key in keys)).scale(0.85).move_to(LEFT * 3.1 + UP * 0.1)
        self.add(group)
        house = visuals.house_section([3.3, 0.2, 0], 0.9)
        storey = house["w_height"] / 2
        metre = storey / 2.5
        figure = visuals.person_glyph(ORIGIN).set_height(1.75 * metre)
        figure.move_to([3.3, house["y_f"] + figure.height / 2, 0])
        panel = visuals.radiator(ORIGIN, height=0.65 * metre, width=1.3 * metre)
        panel.move_to([house["x_l"] + 0.08 * metre + panel.width / 2, house["y_f"] + 0.12 * metre + panel.height / 2, 0])
        self.add(house["group"], figure, panel)
        self.wait(0.3)
        self.play(*(visuals.open_window(window) for window in house["windows"]), run_time=1.0)
        self.wait(0.7)

with tempconfig({"pixel_width": 1600, "pixel_height": 900, "frame_rate": 15, "write_to_movie": True,
                 "save_last_frame": True, "media_dir": str(output), "progress_bar": "none", "verbosity": "ERROR"}):
    ArchitecturalOverview().render()
print("[DEBUG] Architectural overview rendered", flush=True)
