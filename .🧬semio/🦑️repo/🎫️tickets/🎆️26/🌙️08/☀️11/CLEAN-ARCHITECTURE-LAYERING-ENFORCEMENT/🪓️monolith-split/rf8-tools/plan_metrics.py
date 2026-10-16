"""metrics: lift the vocabulary, counting, numstat, report, time, git source, pipeline, render and benchmark regions of 📊️metrics into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from plan_common import *

SPEC = dict(
    target="metrics",
    gate_members=True,
    imports=(20, 26),
    root_drop=[],
    skip_import=[],
    root_cfg={},
    children=[
        C(kind="cluster", mod="vocabulary", folder="🏷️vocabulary", ranges=[(31, 240)],
          doc=["//! 🏷️ Vocabulary: the default branch, the language table and the counting vocabulary every other part names."]),
        C(kind="cluster", mod="counting", folder="📏️counting", ranges=[(241, 291)],
          doc=["//! 📏️ Counting: the unified line counters."]),
        C(kind="cluster", mod="numstat", folder="🧩️numstat", ranges=[(292, 479)],
          doc=["//! 🧩️ Numstat: the pure `git log --numstat` parser."]),
        C(kind="cluster", mod="report", folder="📈️report", ranges=[(480, 783)],
          doc=["//! 📈️ Report: the `LocReport` aggregation model."]),
        C(kind="cluster", mod="time", folder="🕰️time", ranges=[(784, 903)],
          doc=["//! 🕰️ Time: UTC time bucketing."]),
        C(kind="cluster", mod="git_source", folder="🔌️git-source", ranges=[(904, 1060)],
          doc=["//! 🔌️ Git source: the `GitLogSource` door with the system git and the recorded transcript."]),
        C(kind="cluster", mod="pipeline", folder="🧮️pipeline", ranges=[(1061, 1205)],
          doc=["//! 🧮️ Pipeline: the pure functions that turn a git log into a report."]),
        C(kind="cluster", mod="render", folder="📤️render", ranges=[(1206, 1334)],
          doc=["//! 📤️ Render: the Markdown and text table renderers of a report."]),
        C(kind="cluster", mod="benchmark", folder="⏱️benchmark", ranges=[(1335, 1451)],
          doc=["//! ⏱️ Benchmark: the benchmark timing model."]),
    ],
)

if __name__ == "__main__":
    run(SPEC)
