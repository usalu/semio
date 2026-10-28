"""⏱️ Measure the real math readout path and a defensive-copy prototype without production edits."""
from __future__ import annotations

import importlib
import json
import os
import statistics
import sys
import time
from functools import lru_cache
from pathlib import Path
from unittest.mock import patch

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/iv/math-bench"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "cache")
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
CASES = json.loads((TICKET / "inspection-math-bench.json").read_text(encoding="utf-8"))


def timed(call, count):
    """📊 Retain bounded per-call wall and CPU samples under current renderer contention."""
    values, result = [], None
    for _ in range(count):
        wall, cpu = time.perf_counter(), time.process_time()
        result = call()
        values.append({"wall": time.perf_counter() - wall, "cpu": time.process_time() - cpu})
    return {"wall_median": statistics.median(row["wall"] for row in values),
            "cpu_median": statistics.median(row["cpu"] for row in values), "samples": values}, result


def main():
    """🔬 Compare real math geometry, actual updater work, and independent defensive copies."""
    import numpy as np
    from manim import Camera, tempconfig

    visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
    original = visuals.math_text

    @lru_cache(maxsize=CASES["cacheCapacity"])
    def template(source, size, color):
        """🗃️ Own an immutable prototype for this experiment's fixed font environment."""
        return original(source, font_size=size, color=color)

    def cached(source, *, font_size=None, color=visuals.P_WHITE):
        """🧬 Return a fresh deep Manim copy before any placement, style or animation mutation."""
        size = float(font_size or importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️").BODY_FONT_SIZE)
        return template(source, size, str(color)).copy()

    def geometry(actual, expected):
        """📐 Compare every source point, style and baseline through NumPy and Manim."""
        actual_family, expected_family = actual.get_family(), expected.get_family()
        assert len(actual_family) == len(expected_family)
        for left, right in zip(actual_family, expected_family, strict=True):
            np.testing.assert_array_equal(left.points, right.points)
            assert left.get_color() == right.get_color()
            assert not np.shares_memory(left.points, right.points)
        np.testing.assert_array_equal(actual.base.get_center(), expected.base.get_center())
        np.testing.assert_array_equal([actual.get_left(), actual.get_right(), actual.get_top(), actual.get_bottom()],
                                      [expected.get_left(), expected.get_right(), expected.get_top(), expected.get_bottom()])

    rows = []
    with tempconfig({"media_dir": str(OUTPUT / "media"), "text_dir": str(OUTPUT / "text")}):
        for case in CASES["cases"]:
            print("[DEBUG] Benchmark", case["id"], "current", flush=True)
            kwargs = {"size": CASES["fontSize"], "color": CASES["color"], "edge": "left"}
            call = lambda: visuals.math_label(case["source"], CASES["position"], **kwargs)
            initial, reference = timed(call, 1)
            current, reference = timed(call, CASES["repetitions"])
            baseline = visuals.math_readout(lambda: case["source"], CASES["position"], **kwargs)
            updates, _ = timed(lambda: baseline.update(0.25), CASES["readoutUpdates"])
            print("[DEBUG] Benchmark", case["id"], "prototype", flush=True)
            with patch.object(visuals, "math_text", cached):
                miss, clone = timed(call, 1)
                hits, clone = timed(call, CASES["repetitions"])
                geometry(clone, reference)
                clone.shift([3, 1, 0]).set_color("#FF344F").set_opacity(0.2)
                geometry(call(), reference)
                moving = [0.0, 0.0, 0.0]
                readout = visuals.math_readout(lambda: case["source"], lambda: moving, **kwargs)
                cached_updates, _ = timed(lambda: readout.update(0.25), CASES["readoutUpdates"])
                moving[:] = [1.0, 0.5, 0.0]
                readout.update(0.25)
                expected = visuals.math_label(case["source"], moving, **kwargs)
                geometry(readout, expected)
                camera_a = Camera(pixel_width=480, pixel_height=270)
                camera_b = Camera(pixel_width=480, pixel_height=270)
                camera_a.capture_mobjects([reference])
                camera_b.capture_mobjects([call()])
                np.testing.assert_array_equal(np.asarray(camera_a.get_image()), np.asarray(camera_b.get_image()))
            rows.append({"id": case["id"], "source": case["source"], "first_call": initial,
                         "current_label": current, "cached_miss": miss, "cached_label": hits,
                         "current_readout_update": updates, "cached_readout_update": cached_updates,
                         "label_speedup": current["wall_median"] / hits["wall_median"],
                         "readout_speedup": updates["wall_median"] / cached_updates["wall_median"],
                         "geometry_style_baseline_raster_and_defensive_copy_pass": True})
            (OUTPUT / "measurements.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")
        lines = ["# Inspection Math Readout Benchmark", "", "Status: bounded existing-library benchmark completed while the canonical catalog render continued. No shared production helper was edited. This compares warmed current math_label and real always_redraw updater calls with a ticket-local 32-entry immutable-template LRU that returns Manim deep copies. The key is source, resolved font size and normalized color; the experiment holds font environment constant. Counts and source formulas are retained in inspection-math-bench.json.", "", "| Formula | Current Label | Cached Copy | Label Speedup | Current Readout Update | Cached Update | Update Speedup |", "| --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
        for row in rows:
            lines.append(f"| {row['id']} | {row['current_label']['wall_median']:.6f}s | {row['cached_label']['wall_median']:.6f}s | {row['label_speedup']:.1f}x | {row['current_readout_update']['wall_median']:.6f}s | {row['cached_readout_update']['wall_median']:.6f}s | {row['readout_speedup']:.1f}x |")
        lines += ["", "All point arrays, family sizes, colors, baselines and bounds matched the current path exactly through independent NumPy comparisons. Cairo camera rasters matched pixel for pixel. Mutating a returned copy's position, color and opacity left later requests unchanged. Callable readout positions continued to move even when formula text was unchanged.", "", "These are bounded per-call wall-time measurements under concurrent rendering, not an end-to-end render forecast. CPU medians and raw samples are retained transiently with the measurements. Template cache misses still pay the original construction cost; continuously changing numeric strings may remain misses. A production cache must additionally bind templates to every mutable font/rendering environment input, normalize colors consistently, bound storage, preserve baseline metadata and defensive copies, and retain existing cancellation/provenance behavior. A shared helper edit would invalidate existing canonical source certificates and requires a deliberate render restart before claiming current outputs.", "", "## Detailed Samples", "", "```json", json.dumps(rows, indent=2), "```"]
        (TICKET / "inspection-math-bench.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
        print("[DEBUG] Math benchmark complete", json.dumps([{key: row[key] for key in ("id", "label_speedup", "readout_speedup")} for row in rows]), flush=True)


if __name__ == "__main__":
    main()
