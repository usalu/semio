"""✂️ Language-agnostic proof that the matrix generator survives every kind of cut and that a resumed run equals an uninterrupted one: `python3 resume-proof.py <bun> <T> <work folder> <quick id>,<slow id>`.

Four runs of `📊️matrix.ts --jobs 1` on two assets: (A) straight through; (B1) cut with SIGINT as soon as the first record exists (exit code 130, one record, no merge), (B2) restarted and killed with SIGKILL while the second asset runs (the orphaned asset process must end by itself and leave no record), (B3) restarted to its end (exit 0, the finished asset skipped). The records of B must equal those of A cell by cell, byte for byte, and the cells of `matrix.json` must have the same SHA-256. Every progress line of an ended asset must carry its peak resident memory (`peak <n> MB`).
Prints one `ok`/`FAIL` line per claim; the exit code is the number of failed claims."""

import hashlib
import json
import os
import signal
import subprocess
import sys
import time

bun, suite, work, ids = sys.argv[1:5]
quick, slow = ids.split(",")
failures = 0


def claim(ok, text):
    global failures
    failures += 0 if ok else 1
    print(("ok   " if ok else "FAIL ") + text, flush=True)


def command(out, language="en"):
    return [bun, os.path.join(suite, "📊️matrix.ts"), "--language", language, "--out", out, "--assets", ids, "--jobs", "1"]


def records(out):
    folder = os.path.join(out, "cells")
    return sorted(name for name in os.listdir(folder) if name.endswith(".json")) if os.path.isdir(folder) else []


def cells(out, asset):
    raw = open(os.path.join(out, "cells", asset + ".json"), "rb").read()
    return raw[raw.index(b'"cells":') :]


def wait(test, limit):
    started = time.time()
    while time.time() - started < limit and not test():
        time.sleep(0.2)
    return test()


def running(out):
    try:
        return json.load(open(os.path.join(out, "progress.json")))["running"]
    except Exception:
        return []


def orphans(asset):
    found = subprocess.run(["pgrep", "-f", "--asset " + asset], capture_output=True, text=True).stdout.split()
    return [pid for pid in found if int(pid) != os.getpid()]


straight, cut = os.path.join(work, "A"), os.path.join(work, "B")
subprocess.run(["rm", "-rf", work])
os.makedirs(work)

a = subprocess.run(command(straight), cwd=suite, stderr=subprocess.PIPE, text=True)
claim(a.returncode == 0, f"A straight run ends with exit code 0 (got {a.returncode})")
peaks = [line for line in a.stderr.splitlines() if " done " in line]
claim(len(peaks) == 2 and all(" peak " in line and line.split(" peak ")[1].split(" MB")[0].isdigit() for line in peaks), "A every progress line of an ended asset carries `peak <n> MB`")
print("\n".join(peaks))

first = subprocess.Popen(command(cut), cwd=suite, stderr=subprocess.PIPE, text=True)
wait(lambda: first.poll() is not None or records(cut), 3600)
time.sleep(1.0)
first.send_signal(signal.SIGINT)
log = first.communicate()[1]
claim(first.returncode == 130, f"B1 SIGINT ends with exit code 130 (got {first.returncode})")
claim(records(cut) == [quick + ".json"], f"B1 exactly the record of {quick} stays (got {records(cut)})")
claim(not os.path.exists(os.path.join(cut, "matrix.json")), "B1 no merge after a cut")
claim(not orphans(slow), "B1 no asset process is left")

second = subprocess.Popen(command(cut), cwd=suite, stderr=subprocess.PIPE, text=True)
claim(wait(lambda: any(item["asset"] == slow and item["entry"] for item in running(cut)), 600), f"B2 {slow} is running after the restart")
second.kill()
second.communicate()
claim(wait(lambda: not orphans(slow), 300), f"B2 the orphaned asset process of {slow} ends by itself after SIGKILL of the generator")
claim(records(cut) == [quick + ".json"], f"B2 no record of the cut asset (got {records(cut)})")

third = subprocess.run(command(cut), cwd=suite, stderr=subprocess.PIPE, text=True)
claim(third.returncode == 0, f"B3 the restart ends with exit code 0 (got {third.returncode})")
claim("1 assets to run, 1 finished ones skipped" in third.stderr, "B3 the finished asset is skipped")
for asset in (quick, slow):
    claim(cells(cut, asset) == cells(straight, asset), f"B3 the cells of {asset} equal those of the straight run (byte for byte)")
def sha(out):
    raw = open(os.path.join(out, "matrix.json"), "rb").read()
    return hashlib.sha256(raw[raw.index(b'"cells":[') :]).hexdigest()[:16]


claim(sha(cut) == sha(straight), f"B3 the cells of matrix.json have the same SHA-256 ({sha(cut)} and {sha(straight)}; the seconds of `run` differ by nature)")
sys.exit(failures)
