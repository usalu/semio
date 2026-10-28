"""🎞️ Pre-render disjoint optimization sources while the shared publisher repairs BIM."""
from __future__ import annotations
from concurrent.futures import ThreadPoolExecutor, as_completed
import importlib
import json
import os
from pathlib import Path
import sys
import threading

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated" / "i"
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
os.environ.update(PYTHONUTF8="1", PYTHONIOENCODING="utf-8", SEMIO_TEACHING_OUTPUT_ROOT=str(OUTPUT))

def main():
    """🚦 Bound five distinct production source owners and preserve every valid completion."""
    status = json.loads((OUTPUT / "publish.json").read_text(encoding="utf-8"))
    if any(key.startswith("optimization_") for key in status):
        raise RuntimeError("The main publisher already owns optimization")
    runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
    videos = runtime.load_manifest(source_ids=[])
    positions = {index for index, video in enumerate(videos, 1) if video["id"].startswith("optimization_") and video["id"] != "optimization_index"}
    jobs = runtime.source_plan([job for job in runtime.segment_plan(videos) if job["video_index"] in positions])
    if len(jobs) != 5:
        raise RuntimeError("Expected five disjoint non-index optimization sources")
    settings = dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator")
    session = runtime.ToolSession()
    timer = threading.Timer(240, session.cancel)
    timer.daemon = True
    timer.start()
    failures = []
    def render(job):
        return runtime.render_segment(job["segment"], OUTPUT / "de/spectator/s" / str(job["source_ordinal"]),
            settings, session=session, output_root=OUTPUT, log_output=True)
    print("[DEBUG] Five disjoint optimization sources; four workers, production guards, 240s cancellation", flush=True)
    try:
        with ThreadPoolExecutor(max_workers=4) as pool:
            futures = {pool.submit(render, job): job for job in jobs}
            for future in as_completed(futures):
                job = futures[future]
                try:
                    clip = future.result()
                    print(f"[DEBUG] Source {job['source_ordinal']} certified: {clip}", flush=True)
                except Exception as error:
                    failures.append(f"{job['source_ordinal']}: {error}")
                    print(f"[DEBUG] Source {job['source_ordinal']} incomplete: {error}", flush=True)
    except BaseException:
        session.cancel()
        raise
    finally:
        timer.cancel()
    return int(bool(failures))

if __name__ == "__main__":
    raise SystemExit(main())
