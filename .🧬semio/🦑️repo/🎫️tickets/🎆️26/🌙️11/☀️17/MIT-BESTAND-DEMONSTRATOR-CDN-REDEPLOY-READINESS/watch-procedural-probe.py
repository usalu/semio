"""👁️ Follow the next queued Procedural build through the two-locale raw boot."""
import os
import re
import subprocess
import sys
import time
from pathlib import Path


ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
queue = generated / "browser-procedural.log"
marker = generated / f"generator-probe-ready-{os.getpid()}"
previous = sys.argv[1]

while True:
    try:
        log = queue.read_text(errors="replace")
    except FileNotFoundError:
        log = ""
    runs = re.findall(r"runs/(w[A-Za-z0-9]+):", log)
    if runs and runs[-1] != previous:
        previous = runs[-1]
        print(f"[DEBUG] Found Procedural generation {previous}", flush=True)
        env = {**os.environ, "PROBE_LINKED_READY_MARKER": str(marker)}
        result = subprocess.run(["bash", str(ticket / "probe-linked.sh"), "procedural", previous, "8797", "both"], env=env)
        if result.returncode == 0 or marker.exists():
            raise SystemExit(result.returncode)
    time.sleep(5)
