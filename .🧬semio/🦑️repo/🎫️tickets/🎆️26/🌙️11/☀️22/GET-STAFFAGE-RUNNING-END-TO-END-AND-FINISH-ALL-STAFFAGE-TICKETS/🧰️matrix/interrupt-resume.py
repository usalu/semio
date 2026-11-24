"""✂️ Proves that the matrix generator survives a cut: `python3 interrupt-resume.py <bun> <T> <out> <ids,…>` starts `📊️matrix.ts` on the assets with one job, sends SIGINT as soon as the first record exists, prints the exit code and the records left, then starts it again and prints what it skipped and the merged summary."""

import json
import os
import signal
import subprocess
import sys
import time

bun, suite, out, ids = sys.argv[1:5]
command = [bun, os.path.join(suite, "📊️matrix.ts"), "--language", "en", "--out", out, "--assets", ids, "--jobs", "1"]
cells = os.path.join(out, "cells")
first = subprocess.Popen(command, cwd=suite, stderr=subprocess.PIPE, text=True)
started = time.time()
while first.poll() is None and not (os.path.isdir(cells) and any(name.endswith(".json") for name in os.listdir(cells))):
    time.sleep(0.2)
time.sleep(1.0)
first.send_signal(signal.SIGINT)
log = first.communicate()[1]
print(f"first run: cut after {time.time() - started:.0f} s, exit code {first.returncode}")
print(log.strip())
print("records after the cut:", sorted(os.listdir(cells)))
print("progress.json:", json.dumps(json.load(open(os.path.join(out, "progress.json")))))
print("matrix.json exists:", os.path.exists(os.path.join(out, "matrix.json")))
second = subprocess.run(command, cwd=suite, stderr=subprocess.PIPE, text=True)
print(f"second run: exit code {second.returncode}")
print(second.stderr.strip())
print("records after the resume:", sorted(os.listdir(cells)))
summary = json.load(open(os.path.join(out, "summary.json")))
print("run:", json.dumps({key: summary["run"][key] for key in ("done", "failed", "missing")}))
print("total:", json.dumps(summary["summary"]["total"]), "checks:", json.dumps(summary["summary"]["checks"]))
