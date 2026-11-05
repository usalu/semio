"""🧪 Sequential shared-store checks, rereading authored pending jobs between commands."""
from pathlib import Path
import json
import os
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
root = Path.cwd()
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
waiting = int(sys.argv[1]) if len(sys.argv) > 1 else 0
while waiting:
    try:
        os.kill(waiting, 0)
    except ProcessLookupError:
        break
    time.sleep(2)
result_file = generated / "[DEBUG]verification-results.json"
results = json.loads(result_file.read_text()) if result_file.exists() else {}
completed = set(results)
while True:
    jobs = json.loads((ticket / "repair-checks.json").read_text())
    job = next((row for row in jobs if row["label"] not in completed and all(results.get(dependency) == 0 for dependency in row.get("requires_success", []))), None)
    if job is None:
        break
    label = job["label"]
    command = ["repo", "cargo", "--store=shared", "--configuration=prototype", "--", *job["args"]]
    print("[DEBUG] starting " + label, flush=True)
    with (generated / ("[DEBUG]repair-" + label + ".log")).open("w") as log:
        log.write("[DEBUG] cwd=" + job["cwd"] + "\n[DEBUG] argv=" + repr(command) + "\n")
        log.flush()
        outcome = subprocess.run(command, cwd=root / job["cwd"], stdout=log, stderr=subprocess.STDOUT, check=False)
        log.write("\n[DEBUG] rc=" + str(outcome.returncode) + "\n")
    results[label] = outcome.returncode
    (generated / "[DEBUG]verification-results.json").write_text(json.dumps(results, indent=2) + "\n")
    completed.add(label)
    print("[DEBUG] " + label + " rc=" + str(outcome.returncode), flush=True)
    if outcome.returncode and job.get("stop_on_failure", False):
        break
