#!/usr/bin/env python3
"""🖍️ Runs the S2-CONTROLS REPO-PATH-BUDGET repair (`🧪️s2-controls-path-budget-repair.py`) over the draw and note trees.
Prints the plan; `--apply` performs it. Run from the repository root."""

import importlib.util
import pathlib
import sys

spec = importlib.util.spec_from_file_location("repair", pathlib.Path(__file__).with_name("🧪️s2-controls-path-budget-repair.py"))
repair = importlib.util.module_from_spec(spec)
spec.loader.exec_module(repair)
repair.ROOTS = ["✏️s/🔌️plugins/🖍️draw", "✏️s/🔌️plugins/🗒️note"]

if __name__ == "__main__":
    repair.main("--apply" in sys.argv)
