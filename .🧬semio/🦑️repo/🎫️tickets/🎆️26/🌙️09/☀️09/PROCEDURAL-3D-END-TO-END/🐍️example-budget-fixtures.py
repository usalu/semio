#!/usr/bin/env python3
"""⏱️ Writes the `budget` block of every bundled generation3d example fixture.

Reads the `[BUDGET]` / `[DELIVERY]` lines the `example-geometry` Rust lane prints (pass the captured
run log as the first argument) and rewrites each example's `🧫️fixtures/🧩️example/🔣️.json` with a
ceiling derived from the measured wall time. Ceilings are `measured * HEADROOM`, floored at
`FLOOR_MICROS`, so a sub-millisecond example never gets a ceiling that machine jitter alone can
cross; the point of the number is to convict an ALGORITHMIC regression, never a few percent of
machine-to-machine variance.

Usage: python3 🐍️example-budget-fixtures.py <run-log>... [--seed]
       `--seed` writes a permissive placeholder instead, for the first measuring run. Several logs may be
passed at once; the MINIMUM across all of them is taken, because this machine's build fleet inflates
any single reading (see `🐍️example-phase-timings.py`'s own note).
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[7]
EXAMPLES = REPO / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples"
HEADROOM = 4.0
FLOOR_MICROS = 20_000
SEED_MICROS = 2_000_000

DIRS = {
    "rectangle-wire-preview": "🪢️rectangle-wire-preview",
    "rectangle-extrude-volume": "📦️rectangle-extrude-volume",
    "face-sweep-extrude": "🧹️face-sweep-extrude",
    "hexagonal-mushroom-column": "🍄️hexagonal-mushroom-column",
    "box-shell-preview": "🐚️box-shell-preview",
    "box-fillet-preview": "📐️box-fillet-preview",
    "sphere-box-fuse": "🧲️sphere-box-fuse",
    "sphere-cut-with-torus": "🍩️sphere-cut-with-torus",
}


def ceiling(measured: int) -> int:
    return max(FLOOR_MICROS, int(round(measured * HEADROOM / 1000.0)) * 1000)


def measured(log: str, out: dict[str, dict[str, int]] | None = None) -> dict[str, dict[str, int]]:
    out = out if out is not None else {name: {} for name in DIRS}
    for line in log.splitlines():
        budget = re.search(r"\[BUDGET\] (\S+) evaluateMicros=(\d+) budget=\d+ tessellateMicros=(\d+)", line)
        if budget:
            row = out[budget.group(1)]
            row["evaluate"] = min(row.get("evaluate", 1 << 60), int(budget.group(2)))
            row["tessellate"] = min(row.get("tessellate", 1 << 60), int(budget.group(3)))
        delivery = re.search(r"\[DELIVERY\] (\S+) .*totalMicros=(\d+)", line)
        if delivery:
            row = out[delivery.group(1)]
            row["preview"] = min(row.get("preview", 1 << 60), int(delivery.group(2)))
    return out


def write(name: str, block: dict[str, int]) -> None:
    path = EXAMPLES / DIRS[name] / "🧫️fixtures/🧩️example/🔣️.json"
    fixture = json.loads(path.read_text(encoding="utf-8"))
    ordered: dict[str, object] = {}
    for key, value in fixture.items():
        if key != "budget":
            ordered[key] = value
        if key == "delivery":
            ordered["budget"] = block
    if "budget" not in ordered:
        ordered["budget"] = block
    path.write_text(json.dumps(ordered, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"{name}: {block}")


def main() -> None:
    if "--seed" in sys.argv:
        block = {"maxEvaluateMicros": SEED_MICROS, "maxTessellateMicros": SEED_MICROS, "maxPreviewTessellateMicros": SEED_MICROS}
        for name in DIRS:
            write(name, dict(block))
        return
    best: dict[str, dict[str, int]] = {name: {} for name in DIRS}
    for path in sys.argv[1:]:
        measured(Path(path).read_text(encoding="utf-8", errors="replace"), best)
    for name, values in best.items():
        missing = {"evaluate", "tessellate", "preview"} - set(values)
        if missing:
            raise SystemExit(f"{name}: run log is missing {sorted(missing)}")
        write(
            name,
            {
                "maxEvaluateMicros": ceiling(values["evaluate"]),
                "maxTessellateMicros": ceiling(values["tessellate"]),
                "maxPreviewTessellateMicros": ceiling(values["preview"]),
            },
        )


main()
