"""🖼️ Render a representative physical scale proof without narration."""
import importlib.util
import json
import os
from pathlib import Path

import numpy as np
from manim import Scene, RoundedRectangle, Text, VGroup, tempconfig

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/primary-energy"
os.environ["SEMIO_TEACHING_CACHE_ROOT"] = str(OUTPUT.parent)
SOURCE = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py"
SPEC = importlib.util.spec_from_file_location("primary_energy_proof", SOURCE)
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)
OUTPUT.mkdir(parents=True, exist_ok=True)
bridge = C._manim_module._staffage
manifest = json.loads(bridge._MANIFEST.read_text(encoding="utf-8"))
for cast, pose in (("shared-person-winter", "relaxed"), ("shared-boiler-heating-cycle", "heating"),
                   ("demand-primary-energy-pumpjack", "mid-stroke")):
    entry = manifest["casts"][cast]
    entry.pop("clip")
    entry.pop("fps")
    entry["pose"] = pose
bridge._MANIFEST = OUTPUT / "proof-casts.json"
bridge._MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False), encoding="utf-8")
def still(cast, **kwargs):
    kwargs.pop("rate", None)
    kwargs.pop("loop", None)
    return C.staffage(cast, **kwargs)
C.staffage_motion = still



class Proof(Scene):
    def construct(self):
        g, s, hx = -0.95, 0.75, 0.45
        house = C._house(hx, g, s)
        inside = C._furnish(house, s, -0.65, 0.45)
        metre = C.room_metre(house["w_height"] / 2)
        inset = RoundedRectangle(corner_radius=0.06, width=3.35, height=1.15, color=C.P_PRIMARY, stroke_width=1.2)
        inset.move_to(np.array([-5.1, g + 0.45, 0.0]))
        boiler = C._boiler(np.array([2.3, g, 0.0]), metre)
        path = C.smooth_path([(-1.5, g - 0.22, 0.0), (0.85, g - 0.22, 0.0), (2.3, g - 0.22, 0.0), tuple(boiler.get_bottom())])
        self.add(house["group"], inside["group"], C._ground(-2.2, 3.7, g),
                 C._pipes(path), boiler,
                 inset, C._derrick(np.array([-6.1, g]), 0.14),
                 C._power_plant(np.array([-4.45, g]), 0.14))


OUTPUT.mkdir(parents=True, exist_ok=True)
with tempconfig({"media_dir": str(OUTPUT), "save_last_frame": True, "write_to_movie": False,
                 "pixel_width": 1280, "pixel_height": 720, "frame_rate": 15}):
    Proof().render()
