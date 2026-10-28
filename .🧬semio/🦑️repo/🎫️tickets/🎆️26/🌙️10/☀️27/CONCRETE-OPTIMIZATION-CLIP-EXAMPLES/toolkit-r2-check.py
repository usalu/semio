"""🧪 Check the largest-remainder helper (``apportion`` / ``ClipScene.parts``) against an independent NumPy implementation and the model's own bars.

Usage: python toolkit-r2-check.py   (exit code 1 on any failure)
"""
import importlib
import os
import sys
from pathlib import Path

import numpy as np

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/toolkit-r2/fonts")
room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
failures = []


def check(name, condition):
    print(("ok   " if condition else "FAIL ") + name, flush=True)
    if not condition:
        failures.append(name)


def reference(values, digits):
    """Hamilton method with NumPy: floor, then the largest fractional parts get the missing units."""
    scale = 10 ** digits
    exact = np.asarray(values, dtype=float) * scale
    base = np.floor(exact + 1e-9).astype(int)
    goal = int(round(round(float(np.sum(values)), digits) * scale))
    order = np.lexsort((np.arange(len(exact)), -np.abs(exact), -(exact - base)))
    missing = goal - int(base.sum())
    for step in range(abs(missing)):
        base[order[step % len(order)] if missing > 0 else order[-1 - step % len(order)]] += 1 if missing > 0 else -1
    return (base / scale).tolist()


check("audit case 34.41 + 424.24 + 139.89 -> 35 + 424 + 140 = 599", room.apportion([34.41, 424.24, 139.89], 0) == [35.0, 424.0, 140.0])
check("audit case 38.58 + 280.60 + 176.68 -> 38 + 281 + 177 = 496", room.apportion([38.58, 280.60, 176.68], 0) == [38.0, 281.0, 177.0])
check("digits=1 keeps one decimal and sums to the total", room.apportion([34.41, 424.24, 139.89], 1) == [34.4, 424.2, 139.9])
check("explicit total shifts the parts", sum(room.apportion([1.4, 1.4, 1.4], 0, total=5)) == 5)
check("negative parts", sum(room.apportion([-1.4, 2.7, -0.3], 0)) == round(-1.4 + 2.7 - 0.3))
check("empty", room.apportion([], 0) == [])
rng = np.random.default_rng(7)
bad = 0
for _ in range(4000):
    digits = int(rng.integers(0, 3))
    values = (rng.random(int(rng.integers(1, 7))) * rng.choice([1, 10, 1000])).tolist()
    mine, other = room.apportion(values, digits), reference(values, digits)
    total = round(sum(values), digits)
    bad += not (abs(sum(mine) - total) < 1e-9 and all(abs(a - b) < 1e-9 for a, b in zip(mine, other)) and all(abs(m - v) < 1.0 / 10 ** digits + 1e-9 for m, v in zip(mine, values)))
check("4000 random cases: sum equals rounded total, equals NumPy Hamilton reference, each part within one unit of its value", bad == 0)
broken = 0
for orientation in range(90, 451, 15):
    for wwr in (.1, .3, .45, .6, .8):
        for overhang in (0, .6, 1.2):
            found = model.evaluate({"orientation": orientation, "wwr": wwr, "overhang": overhang})
            shown = room.apportion([found["heating"], found["cooling"], found["lighting"]], 0)
            broken += abs(sum(shown) - round(found["electricity"])) > 1e-9
check("every design of a 25 x 5 x 3 grid: printed heating + cooling + lighting equals the printed electricity", broken == 0)
naive = sum(1 for orientation in range(90, 451, 15) for wwr in (.1, .3, .45, .6, .8) for overhang in (0, .6, 1.2)
            if (lambda f: sum(round(f[k]) for k in ("heating", "cooling", "lighting")) != round(f["electricity"]))(model.evaluate({"orientation": orientation, "wwr": wwr, "overhang": overhang})))
print(f"[DEBUG] designs where plain rounding of each part misses the total: {naive} of 375", flush=True)
sys.exit(1 if failures else 0)
