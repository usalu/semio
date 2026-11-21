"""🌡️ Replays the captured DIN V 18599 document against the completed current energy compiler cohort."""
from pathlib import Path
import json
import os
import re
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
run = workspace / ".cache/b/r/m/components/runs" / sys.argv[1]
crate = "semio_s_artifact_norm_din18599_editor"
editor_target = ('"name":"' + crate + '"').encode()
offsets = {}
candidates = {}

def compiler_lane():
    for log in run.glob("l/cargo-prototype-*.jsonl"):
        with log.open("rb") as stream:
            stream.seek(offsets.get(log, 0) if log.stat().st_size >= offsets.get(log, 0) else 0)
            while True:
                position = stream.tell()
                row = stream.readline()
                if not row.endswith(b"\n"):
                    offsets[log] = position
                    break
                if editor_target not in row:
                    continue
                try:
                    event = json.loads(row)
                except ValueError:
                    continue
                if event.get("reason") != "compiler-artifact" or event.get("target", {}).get("name") != crate:
                    continue
                for file in event["filenames"]:
                    match = re.search(r"/u/l/([^/]+)/(?:[^/]+/)?wasm32-wasip2/", file)
                    if match and file.endswith(".rlib"):
                        candidates[Path(file)] = match[1]
    for file, lane in reversed(tuple(candidates.items())):
        if file.is_file():
            return lane
    return None

deadline = time.monotonic() + 3 * 3600
while time.monotonic() < deadline:
    lane = compiler_lane()
    if lane:
        print(f"[DEBUG] Captured climate child test using current energy compiler cohort {lane}", flush=True)
        if os.environ.get("WATCH_MEMBER_ARCHIVE_WAIT_ONLY") == "1":
            sys.exit(0)
        rustc = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
        sys.exit(subprocess.call(
            [sys.executable, str(ticket / "check-original-archive-context.py"), lane,
             "wire", "replacement", "runtime-cleanup", "member-maintenance", "runtime-diagnostics",
             *sys.argv[2:]],
            env=dict(os.environ, RUSTC=rustc, NORM_ARCHIVE_FAMILY="din18599"),
        ))
    if (run / "r" / "report.md").is_file():
        sys.exit("[DEBUG] Build finished without a current wasm32 DIN V 18599 editor cohort")
    time.sleep(5)

sys.exit("[DEBUG] Watcher reached its three-hour limit")
