"""Unit-level proof for profile-driven effect budgets in the shared visuals.

Run with the teaching venv: final frames must hash identical to the authored (unrouted) code path,
preview must draw fewer particles and be cheaper per frame. Output: JSON on stdout.
"""

import dataclasses
import hashlib
import importlib
import json
import os
import statistics
import sys
import time
from pathlib import Path
from unittest.mock import patch

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_PROFILE"] = "final"

import numpy as np
from manim import Scene, config

visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
profile = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")

config.pixel_width, config.pixel_height, config.frame_rate = 320, 180, 15
config.write_to_movie = False
config.disable_caching = True
config.verbosity = "ERROR"

PATHS = [visuals.smooth_path([(-6, -2 + 0.5 * i, 0), (-2, 0.3 * i, 0), (2, -0.5 + 0.4 * i, 0), (6, 1 + 0.3 * i, 0)]) for i in range(8)]
CENTERS = [(-5 + i, -3, 0) for i in range(10)]


def build(kind: str):
    if kind == "flow":
        return visuals.flow_animation([(PATHS[:4], "#ff6a00", "#0099ff"), (PATHS[4:], "#00cc66")], waves=5)
    return visuals.ripples(CENTERS, rings=3)


def frames(kind: str, active: str, alphas):
    with patch.dict(os.environ, {"SEMIO_TEACHING_PROFILE": active}):
        scene = Scene()
        animation = build(kind)
        visuals._FLOW_PHASE.clear()
        scene.add(animation.mobject)
        animation.begin()
        hashes, times = [], []
        for alpha in alphas:
            started = time.perf_counter()
            animation.interpolate(alpha)
            scene.renderer.update_frame(scene)
            times.append((time.perf_counter() - started) * 1000)
            hashes.append(hashlib.sha256(scene.renderer.get_frame().tobytes()).hexdigest()[:16])
        count = len(animation.mobject)
        return hashes, times, count


def reference(kind: str, alphas):
    with patch.object(visuals, "effect_density", lambda budget, authored, units: authored):
        return frames(kind, "final", alphas)


alphas = [i / 30 for i in range(1, 30)]
report = {}
for kind in ("flow", "ripples"):
    ref_hashes, ref_times, ref_count = reference(kind, alphas)
    final_hashes, final_times, final_count = frames(kind, "final", alphas)
    preview_hashes, preview_times, preview_count = frames(kind, "preview", alphas)
    report[kind] = {
        "finalIdenticalToAuthored": ref_hashes == final_hashes,
        "previewDiffers": preview_hashes != final_hashes,
        "authoredMobjects": ref_count, "finalMobjects": final_count, "previewMobjects": preview_count,
        "finalMedianMs": round(statistics.median(final_times), 3), "previewMedianMs": round(statistics.median(preview_times), 3),
        "speedup": round(statistics.median(final_times) / statistics.median(preview_times), 2),
    }
print(json.dumps(report, indent=2))
