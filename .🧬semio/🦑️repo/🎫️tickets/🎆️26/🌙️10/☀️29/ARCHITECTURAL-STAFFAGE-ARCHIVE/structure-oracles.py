"""Runs every structural oracle case in one interpreter: `repo tools exec uv -- run --with anastruct python <this> --write|--check [case ...]`."""
import runpy
import sys
from pathlib import Path

CASES = ["🏗️cantilever", "🌉️simply-supported", "🪢️tied-canopy", "🧍️two-foot-stance", "⚖️equilibrium", "🏃️inertial-consistency", "🦶️centre-of-pressure"]
mode = sys.argv[1] if len(sys.argv) > 1 else "--check"
chosen = sys.argv[2:] or CASES
tests = Path(__file__).resolve().parents[7] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🧪️tests"
for case in chosen:
    sys.argv = ["oracle", mode]
    runpy.run_path(str(tests / case / "🐍️.py"), run_name="__main__")
