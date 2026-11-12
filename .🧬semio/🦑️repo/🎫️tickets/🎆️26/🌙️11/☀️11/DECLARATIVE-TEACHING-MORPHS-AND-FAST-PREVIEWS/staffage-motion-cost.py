"""Time the warm-cache bridge calls for a few casts: ensure_casts, staffage(), staffage_motion() construction, one frame swap, fitted_staffage probe.
Run: python -I staffage-motion-cost.py (cwd = repo root, PYTHONPATH = repo root, PYTHONUTF8=1)"""
import importlib
import json
import sys
import time
from pathlib import Path

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
CASTS = ["cooling-internal-gains-walker", "cooling-internal-gains-laptop", "cooling-summer-strategy-wheel", "physics-fundamentals-vacuum", "ventilation-volume-cook"]


def ms(fn, repeat=1):
    start = time.perf_counter()
    for _ in range(repeat):
        result = fn()
    return round((time.perf_counter() - start) * 1000 / repeat, 2), result


rows = []
for cast in CASTS:
    if cast not in S._manifest()["casts"]:
        continue
    row = {"cast": cast}
    row["ensure_ms_warm"], _ = ms(lambda: S.ensure_casts([cast]), 20)
    folder = S.ensure_casts([cast])[cast]
    index = S._index(folder)
    row["frames"] = len(index["frames"])
    S._SLOT_COUNTS.pop(folder, None)
    row["slot_count_ms"], _ = ms(lambda: S._slot_count(folder))
    row["staffage_first_frame_ms"], _ = ms(lambda: S.staffage(cast, unit=1.0))
    S._SLOT_COUNTS.pop(folder, None)
    row["motion_ctor_ms_cold_slots"], motion = ms(lambda: S.staffage_motion(cast, unit=1.0, rate=1.0))
    row["motion_ctor_ms_warm_slots"], _ = ms(lambda: S.staffage_motion(cast, unit=1.0, rate=1.0))
    row["probe_fitted_ms"], _ = ms(lambda: V.fitted_staffage(cast, height=1.0, rate=1.0))
    S._FRAMES.clear()
    row["swap_cold_ms"], _ = ms(lambda: motion._show(min(5, len(motion.frames) - 1)))
    row["swap_warm_ms"], _ = ms(lambda: motion._show(min(5, len(motion.frames) - 1)), 20)
    row["slots"] = len(motion.slots)
    rows.append(row)
    print(json.dumps(row, ensure_ascii=False))
Path(sys.argv[1] if len(sys.argv) > 1 else "staffage-motion-cost.json").write_text(json.dumps(rows, ensure_ascii=False, indent=1), encoding="utf-8")
