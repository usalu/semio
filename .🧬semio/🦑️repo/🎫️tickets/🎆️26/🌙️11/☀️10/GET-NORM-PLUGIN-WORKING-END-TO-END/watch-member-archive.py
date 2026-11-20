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
crate = "semio_hub_norm_energy_component"

def compiler_lane():
    for log in run.glob("l/cargo-prototype-*.jsonl"):
        for row in reversed(log.read_text().splitlines()):
            try:
                event = json.loads(row)
            except ValueError:
                continue
            if event.get("reason") != "compiler-artifact" or event.get("target", {}).get("name") != crate:
                continue
            for file in event["filenames"]:
                match = re.search(r"/u/l/([^/]+)/(?:[^/]+/)?wasm32-wasip2/", file)
                if match and Path(file).is_file():
                    return match[1]
    return None

deadline = time.monotonic() + 3 * 3600
while time.monotonic() < deadline:
    lane = compiler_lane()
    if lane:
        print(f"[DEBUG] Captured climate child test using current energy compiler cohort {lane}", flush=True)
        rustc = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
        sys.exit(subprocess.call(
            [sys.executable, str(ticket / "check-original-archive-context.py"), lane,
             "wire", "replacement", "runtime-cleanup", "member-maintenance"],
            env=dict(os.environ, RUSTC=rustc, NORM_ARCHIVE_FAMILY="din18599"),
        ))
    if (run / "r" / "report.md").is_file():
        sys.exit("[DEBUG] Build finished without a current wasm32 energy component cohort")
    time.sleep(5)

sys.exit("[DEBUG] Watcher reached its three-hour limit")
