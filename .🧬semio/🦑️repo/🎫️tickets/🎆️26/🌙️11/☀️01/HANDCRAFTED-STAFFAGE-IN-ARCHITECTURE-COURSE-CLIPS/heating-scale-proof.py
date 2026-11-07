"""🏡 Render the physical-scale heating plant without narration."""
import importlib
import json
import math
import os
import sys
from pathlib import Path

import numpy as np
from manim import Scene, tempconfig

sys.path.insert(0, str(next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "de"
module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️")
output = Path(__file__).parent / "🗑️generated" / "heating-scale"
output.mkdir(parents=True, exist_ok=True)
bridge = module._manim_module._staffage
manifest = json.loads(bridge._MANIFEST.read_text(encoding="utf-8"))
for cast, pose in (("shared-radiator-thermostat", "comfort"), ("heating-calculation-thermostat", "calling"),
                   ("shared-tank-charge-up", "charged"), ("shared-boiler-heating-cycle", "heating"),
                   ("shared-person-winter", "relaxed")):
    entry = manifest["casts"][cast]
    entry.pop("clip", None)
    entry.pop("fps", None)
    entry["pose"] = pose
bridge._MANIFEST = output / "proof-casts.json"
bridge._MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False), encoding="utf-8")

radiators = []

def still(cast, **kwargs):
    kwargs.pop("rate", None)
    kwargs.pop("loop", None)
    figure = module.staffage(cast, **kwargs)
    figure.seek = lambda _: None
    if cast == "shared-radiator-thermostat":
        radiators.append((figure, np.array(kwargs["anchor"]), kwargs["unit"], kwargs.get("mirror", False)))
    return figure

module.staffage_motion = still


class PlantProof(Scene):
    def construct(self):
        plant = module._plant_house()
        metre = plant["room_height"] / 2.5
        root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
        asset = json.loads((root / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/panel-radiator/🔣️.json").read_text(encoding="utf-8"))["asset"]
        bone = next(b for b in asset["rig"]["bones"] if b["id"] == "radiator")
        anchor = next(a for a in asset["anchors"] if a["id"] == "bracket-top")
        angle = math.radians(bone["angleDeg"])
        x = bone["x"] + anchor["x"] * math.cos(angle) - anchor["y"] * math.sin(angle)
        y = bone["y"] + anchor["x"] * math.sin(angle) + anchor["y"] * math.cos(angle)
        assert len(radiators) == 2
        for i, (figure, origin, unit, mirror) in enumerate(radiators):
            assert figure is plant["radiators"][i]
            support = origin + np.array([-x if mirror else x, y, 0]) * unit
            wall = plant["walls"][i]
            assert abs(support[0] - (wall.get_right() if i == 0 else wall.get_left())[0]) < 1e-9
            assert wall.get_bottom()[1] <= support[1] <= wall.get_top()[1]
        assert abs(plant["thermostat"].get_center()[1] - (plant["floor_y"] + 1.5 * metre)) < 0.05 * metre
        self.add(*(plant[name] for name in (
            "room", "cellar", "material", "windows", "radiators", "thermostat",
            "store", "boiler", "chimney", "pipes", "person", "labels", "thermo_label", "pipe_label",
        )))
        self.wait(0.1)


with tempconfig({"media_dir": str(output), "write_to_movie": False, "save_last_frame": True, "pixel_width": 1280, "pixel_height": 720}):
    PlantProof().render()
