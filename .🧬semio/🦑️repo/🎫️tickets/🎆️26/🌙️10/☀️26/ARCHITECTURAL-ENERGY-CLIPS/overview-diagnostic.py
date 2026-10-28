"""🔬 Compare overview scene families and rasters with live and frozen tank fills."""

import importlib
import json
import os
from pathlib import Path
import sys

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))

import numpy as np
from PIL import Image
from manim import Camera, Scene, VGroup, LEFT, UP, ORIGIN, tempconfig

VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
HEATING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️")
OUTPUT = Path(__file__).parent / "🗑️generated" / "overview-diagnostic"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
MODE = ""
FROZEN = False
RECORDS = []


class OverviewDiagnostic(Scene):
    def capture(self, stage):
        family = self.get_mobject_family_members()
        tracked = self.house["group"].get_family()
        point_members = [mob for mob in tracked if len(mob.points)]
        camera = Camera(pixel_width=640, pixel_height=360)
        camera.capture_mobjects(self.mobjects)
        fresh = np.asarray(camera.get_image())
        live = self.renderer.get_frame()
        Image.fromarray(live).save(OUTPUT / f"{MODE}-{stage}-live.png")
        Image.fromarray(fresh).save(OUTPUT / f"{MODE}-{stage}-fresh.png")
        right = fresh[:, 340:, :3]
        live_right = live[:, 340:, :3]
        RECORDS.append(dict(mode=MODE, stage=stage, top_level=len(self.mobjects), family=len(family),
                            tracked=len(tracked), tracked_present=sum(mob in family for mob in tracked),
                            drawable=len(point_members), live_bright=int(np.sum(np.min(live_right, axis=2) > 100)),
                            fresh_bright=int(np.sum(np.min(right, axis=2) > 100)),
                            bounds={name: [self.house[name].get_left().tolist(), self.house[name].get_right().tolist()]
                                    for name in ("roof", "walls", "floor")}))
        print("[DEBUG] " + json.dumps(RECORDS[-1]), flush=True)

    def construct(self):
        plant = HEATING._plant_house()
        if FROZEN:
            plant["tank"]["fill"].clear_updaters()
        keys = ("room", "cellar", "walls", "roof", "windows", "radiators", "thermostat", "store", "boiler", "chimney", "pipes", "person")
        group = VGroup(*(plant[key] for key in keys)).scale(0.85).move_to(LEFT * 3.1 + UP * 0.1)
        self.add(group)
        self.house = VISUALS.house_section([3.3, 0.2, 0], 0.9)
        storey = self.house["w_height"] / 2
        metre = storey / 2.5
        figure = VISUALS.person_glyph(ORIGIN).set(height=1.75 * metre)
        figure.move_to([3.3, self.house["y_f"] + figure.height / 2, 0])
        panel = VISUALS.radiator(ORIGIN, height=0.65 * metre, width=1.3 * metre)
        panel.move_to([self.house["x_l"] + 0.08 * metre + panel.width / 2, self.house["y_f"] + 0.12 * metre + panel.height / 2, 0])
        self.add(self.house["group"], figure, panel)
        self.wait(0.3)
        self.capture("before")
        self.play(*(VISUALS.open_window(window) for window in self.house["windows"]), run_time=1.0)
        self.capture("opened")
        self.wait(0.7)
        self.capture("after")


for mode, frozen, uncached in (("dynamic", False, False), ("frozen", True, False), ("uncached", False, True)):
    MODE, FROZEN = mode, frozen
    with tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 6, "write_to_movie": True,
                     "save_last_frame": True, "media_dir": str(OUTPUT / mode), "progress_bar": "none",
                     "verbosity": "ERROR", "disable_caching": uncached}):
        OverviewDiagnostic().render()
(OUTPUT / "families.json").write_text(json.dumps(RECORDS, indent=2) + "\n", encoding="utf-8")
