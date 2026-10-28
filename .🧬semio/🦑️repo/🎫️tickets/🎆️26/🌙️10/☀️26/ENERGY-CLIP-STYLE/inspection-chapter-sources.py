"""🎞️ Refresh canonical inspection bodies independently of window publication."""
import contextlib
import importlib
import json
import os
from pathlib import Path
import sys
import threading
import time

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/i"
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
os.environ.update(PYTHONUTF8="1", PYTHONIOENCODING="utf-8", SEMIO_TEACHING_OUTPUT_ROOT=str(OUTPUT))

class Tee:
    """📝 Persist bounded native rendering progress."""
    def __init__(self, stream, log):
        self.stream, self.log = stream, log
        self.lock = threading.RLock()
    def write(self, value):
        with self.lock:
            self.stream.write(value)
            self.log.write(value)
            self.log.flush()
        return len(value)
    def flush(self):
        self.stream.flush()
        self.log.flush()

def main():
    """📦 Certify all canonical bodies while leaving navigation publication separate."""
    runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
    videos = runtime.load_manifest(source_ids=[])
    plan = runtime.segment_plan(videos)
    sources = runtime.source_plan(plan)
    partial = {job["source_ordinal"] for job in plan if "clauses" in job["segment"]}
    settings = dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator")
    def render(job, session):
        for attempt in range(1, 4):
            try:
                result = runtime.render_segment(job["segment"], OUTPUT / "de/spectator/s" / str(job["source_ordinal"]), settings, session=session, output_root=OUTPUT, require_trace=job["source_ordinal"] in partial, log_output=True)
                runtime.cursor_intervals(result)
                print(f"[DEBUG] Certified source {job['source_ordinal']} {job['segment']['scene']}", flush=True)
                return result
            except RuntimeError as error:
                if "changed" not in str(error).lower() or attempt == 3:
                    raise
                print(f"[DEBUG] Retry changed source {job['source_ordinal']}: {error}", flush=True)
    results, failures = runtime.execute_jobs(sources, render, max_workers=4)
    record = dict(expected=len(sources), certified=len(results), failures={str(sources[index]["source_ordinal"]): str(error) for index,error in failures.items()}, checked=time.time())
    (OUTPUT / "chapter-source-status.json").write_text(json.dumps(record, indent=2), encoding="utf-8")
    print("[DEBUG] Source refresh complete", record, flush=True)
    return bool(failures)

if __name__ == "__main__":
    OUTPUT.mkdir(parents=True, exist_ok=True)
    with (OUTPUT / "chapter-source-refresh.log").open("a", encoding="utf-8", buffering=1) as log:
        with contextlib.redirect_stdout(Tee(sys.stdout, log)), contextlib.redirect_stderr(Tee(sys.stderr, log)):
            raise SystemExit(main())
