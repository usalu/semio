"""🎞️ Pre-render one isolated optimization source while BIM publication is blocked."""
from __future__ import annotations
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
    """🚦 Bound one production source job before the main composer can reach optimization."""
    status = json.loads((OUTPUT / "publish.json").read_text(encoding="utf-8"))
    if any(key.startswith("optimization_") for key in status):
        raise RuntimeError("The main publisher already owns optimization")
    package = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
    runtime = importlib.import_module(package + ".🎬️clip.⚙️runtime.🐍️")
    videos = runtime.load_manifest(source_ids=[])
    identity = sys.argv[1] if len(sys.argv) == 2 else "optimization_index"
    if len(sys.argv) > 2 or identity not in {"optimization_index", "optimization_algorithms", "optimization_experiments"}:
        raise ValueError("Select one isolated index, algorithms or experiments video")
    position = next(index for index, video in enumerate(videos, 1) if video["id"] == identity)
    jobs = runtime.source_plan([job for job in runtime.segment_plan(videos) if job["video_index"] == position])
    if len(jobs) != 1:
        raise RuntimeError("Expected one isolated optimization source")
    job = jobs[0]
    settings = dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator")
    session = runtime.ToolSession()
    seconds = 1200 if identity == "optimization_experiments" else 240
    timer = threading.Timer(seconds, session.cancel)
    timer.daemon = True
    finished = threading.Event()
    def handoff():
        while not finished.wait(1):
            try:
                current = json.loads((OUTPUT / "publish.json").read_text(encoding="utf-8"))
                if any(key.startswith("optimization_") for key in current) or "bim_exchange" in current:
                    session.cancel()
                    return
            except (OSError, ValueError):
                continue
    monitor = threading.Thread(target=handoff, daemon=True)
    timer.start()
    monitor.start()
    print(f"[DEBUG] Isolated {identity} source {job['source_ordinal']}; guards, {seconds}s cancellation and main-stage handoff active", flush=True)
    try:
        for attempt in range(1, 4):
            try:
                clip = runtime.render_segment(job["segment"], OUTPUT / "de/spectator/s" / str(job["source_ordinal"]),
                    settings, session=session, output_root=OUTPUT, log_output=True)
                print(f"[DEBUG] Isolated {identity} source certified: {clip}", flush=True)
                break
            except RuntimeError as error:
                if "changed" not in str(error).lower() or attempt == 3 or session.cancelled.is_set():
                    raise
                print(f"[DEBUG] Retrying changed {identity} inputs after attempt {attempt}", flush=True)
    except BaseException:
        session.cancel()
        raise
    finally:
        finished.set()
        timer.cancel()
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
