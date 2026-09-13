#!/usr/bin/env python3
"""⏱️ Runs the built `example-geometry` binary repeatedly and reports the MINIMUM wall time per phase.

The repository's build fleet keeps this machine under heavy, bursty load (load averages of 20-60 are
normal while peers compile), and a single wall-clock reading of a single-threaded test inflates by
2-3x under that. The minimum over several runs is the least contended sample and is the only honest
number to compare a before against an after with, so every measurement in
`📓️kernel-performance-2026-09-13.md` is a min-of-N taken this way, with the load average recorded
beside it.

Usage: python3 🐍️example-phase-timings.py <test-binary> [runs]
"""

import os
import re
import subprocess
import sys

BUDGET = re.compile(r"\[BUDGET\] (\S+) evaluateMicros=(\d+) budget=\d+ tessellateMicros=(\d+)")
DELIVERY = re.compile(r"\[DELIVERY\] (\S+) .*totalMicros=(\d+)")


def main() -> None:
    binary = sys.argv[1]
    runs = int(sys.argv[2]) if len(sys.argv) > 2 else 5
    best: dict[str, dict[str, int]] = {}
    for _ in range(runs):
        out = subprocess.run([binary, "--test-threads=1", "--nocapture"], capture_output=True, text=True).stdout
        for line in out.splitlines():
            budget = BUDGET.search(line)
            if budget:
                row = best.setdefault(budget.group(1), {})
                row["evaluate"] = min(row.get("evaluate", 1 << 60), int(budget.group(2)))
                row["tessellate"] = min(row.get("tessellate", 1 << 60), int(budget.group(3)))
            delivery = DELIVERY.search(line)
            if delivery:
                row = best.setdefault(delivery.group(1), {})
                row["preview"] = min(row.get("preview", 1 << 60), int(delivery.group(2)))
    print(f"load={os.getloadavg()} runs={runs}")
    print(f"{'example':<28}{'evaluate':>12}{'tessellate':>12}{'preview':>12}")
    for name in sorted(best, key=lambda key: -best[key].get("tessellate", 0)):
        row = best[name]
        print(f"{name:<28}{row.get('evaluate', 0):>12}{row.get('tessellate', 0):>12}{row.get('preview', 0):>12}")


main()
