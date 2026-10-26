"""♻️ Retry a failed isolated full-scene render without touching other jobs."""

import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
ticket = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("parallel_monitor", ticket / "parallel-monitor.py")
monitor = importlib.util.module_from_spec(spec)
spec.loader.exec_module(monitor)
number = int(sys.argv[1])
mapping = json.loads((monitor.PRE / "jobs.json").read_text(encoding="utf-8"))
job = next(job for job in mapping if job["job"] == number)
video = next(video for video in monitor.load_manifest() if video["id"] == job["video"])
monitor.render((number, job["video_index"], video, Path(job["source"]), job["scene"], None))
print(f"[DEBUG] Retried stage job{number} completed successfully", flush=True)
