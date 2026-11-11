"""⏱️ Measure the authored analytical box at native preview dimensions."""
import importlib
import os
import sys
import time
from pathlib import Path
sys.path.insert(0, str(next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())))
from manim import tempconfig

module = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️")
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
with tempconfig({"dry_run": True, "pixel_width": 320, "pixel_height": 180, "frame_rate": 15}):
    scene = module.Beat1_Anatomy()
    times = []
    for width in (2., 2.5, 3.):
        start = time.perf_counter()
        drawing = scene.analytical_box_axon(width, 4., 2., dims=True)
        scene.camera.reset()
        scene.camera.capture_mobjects([drawing])
        times.append(time.perf_counter() - start)
        assert set(drawing.dims) == {"width", "length", "height"}
        assert abs(drawing.dims["width"].measurement_value - width) < 1e-9
    print({"widths": [2., 2.5, 3.], "draw_and_capture_seconds": times, "facets": len(drawing.get_family())})
