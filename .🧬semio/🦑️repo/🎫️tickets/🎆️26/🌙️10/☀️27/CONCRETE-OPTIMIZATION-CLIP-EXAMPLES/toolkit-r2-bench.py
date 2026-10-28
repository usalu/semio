"""⏱️ Measure the redraw cost of the round-2 widgets (section with daylight, formula, bars) when a design tracker moves.

Usage: python toolkit-r2-bench.py   (prints [DEBUG] lines: ms per rebuild, including the moving-tracker rebuild)
"""
import importlib
import os
import sys
import time
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/toolkit-r2/fonts")
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")


class Bench(visuals.ClipScene):
    def construct(self):
        trackers = self.study_trackers({})
        widgets = {
            "section daylight off": self.room_section(trackers, (0, 0, 0), .45, "summer", daylight=False),
            "section daylight on": self.room_section(trackers, (0, 0, 0), .45, "summer"),
            "section daylight + light_label": self.room_section(trackers, (0, 0, 0), .45, "summer", light_label=True),
            "electricity_formula": self.electricity_formula(trackers, (0, 0, 0)),
            "derivation lighting": self.derivation(trackers, "lighting", (0, 0, 0)),
            "energy_bars": self.energy_bars(trackers, (0, 0, 0), 3.0, 800),
        }
        for name, widget in widgets.items():
            spent = 0.0
            for step in range(12):
                trackers["wwr"].set_value(.2 + .04 * step)
                started = time.perf_counter()
                widget.refresh()
                spent += time.perf_counter() - started
            print(f"[DEBUG] {name}: {spent / 12 * 1000:.0f} ms per rebuild", flush=True)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True,
                       "skip_animations": True, "media_dir": str(TICKET / "🗑️generated/toolkit-r2/media"), "verbosity": "ERROR"}):
    Bench().render()
