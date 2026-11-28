#!/usr/bin/env python3
"""H44 live proof: python3 proof.py always|never FILE FILE FILE. Runs `repo test-file --json` and prints per file the machine, the status, the counts and the wall seconds, then the total wall seconds."""
import json, os, subprocess, sys, time
mode = sys.argv[1]
files = sys.argv[2:]
env = dict(os.environ, SEMIO_TEST_FILE_OFFLOAD=mode, SEMIO_LANGUAGE="en")
started = time.time()
run = subprocess.run(["repo", "test-file", "--json", "--", *files], cwd="/Users/ueli/Documents/semio", env=env, capture_output=True, text=True)
wall = time.time() - started
sys.stderr.write(run.stderr[-6000:])
document = json.loads(run.stdout)
for row in document["files"]:
    print(row["file"][-60:], row["status"], row.get("pass"), row.get("fail"), row.get("ranOn", "here"), row["seconds"])
print("exit", run.returncode, "wall", round(wall, 1), "total", document["seconds"], "status", document["status"])
