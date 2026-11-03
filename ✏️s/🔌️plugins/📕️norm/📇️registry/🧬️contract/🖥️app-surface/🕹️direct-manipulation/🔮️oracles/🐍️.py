"""🧲️ Third-party oracle for handle snapping: numpy re-derives the clamp, nearest-target and increment rounding of the Rust implementation.

Reads JSON cases from stdin: {"raw", "min", "max", "targets", "tolerance", "increment"}; prints one snapped value per line.
"""
import json
import sys

import numpy as np


def snap(case):
    s = float(np.clip(case["raw"], case["min"], case["max"]))
    targets = np.asarray(case["targets"], dtype=float)
    if targets.size:
        distance = np.abs(targets - s)
        if distance.min() <= case["tolerance"]:
            s = float(targets[int(np.argmin(distance))])
            return float(np.clip(s, case["min"], case["max"]))
    if case["increment"]:
        s = float(np.round(s / case["increment"]) * case["increment"])
    return float(np.clip(s, case["min"], case["max"]))


for case in json.load(sys.stdin):
    print(repr(snap(case)))
