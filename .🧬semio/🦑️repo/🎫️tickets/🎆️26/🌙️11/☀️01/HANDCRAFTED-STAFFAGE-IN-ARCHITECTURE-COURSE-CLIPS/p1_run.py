"""Run a manim render in-process and report wall time and peak working set: `p1_run.py <manim args...>`."""

import atexit
import sys
import time
from pathlib import Path

sys.path.insert(0, ".")
sys.path.insert(0, str(Path(__file__).parent))
from p1_rss import peak_mb

started = time.time()
atexit.register(lambda: print(f"[DEBUG] render wall={time.time() - started:.1f}s peak={peak_mb():.0f}MB", flush=True))
from manim.__main__ import main

sys.argv = ["manim", *sys.argv[1:]]
main()
