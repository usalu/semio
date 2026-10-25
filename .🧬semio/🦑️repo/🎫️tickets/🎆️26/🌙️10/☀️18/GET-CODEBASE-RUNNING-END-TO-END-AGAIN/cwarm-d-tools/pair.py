"""Warm-proof driver: runs one target twice back to back (cold-ish A, warm B) with Bazel execution logs and repeats until B executes nothing.

python3 pair.py <outDir> <attempts> <target> [--tool=bazel]
Writes <outDir>/pair<N>-{A,B}.log + evidence dirs and <outDir>/pairs.jsonl (one summary row per attempt).
"""
import json, os, re, subprocess, sys, time

out, attempts, target = sys.argv[1], int(sys.argv[2]), sys.argv[3]
tool = sys.argv[4] if len(sys.argv) > 4 else "--tool=bazel"
os.makedirs(out, exist_ok=True)
root = "/Users/ueli/Documents/semio"

def run(tag):
    evidence = os.path.join(out, tag + "-ev")
    os.makedirs(evidence, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if k != "SEMIO_SCRIPT_STALL_MS"}
    env["SEMIO_BUILD_EVIDENCE_DIR"] = evidence
    started = time.time()
    with open(os.path.join(out, tag + ".log"), "wb") as log:
        code = subprocess.call(["nice", "-n", "10", "bun", "repo", tool, "run", target], cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
    text = open(os.path.join(out, tag + ".log"), encoding="utf-8", errors="replace").read()
    rows = re.findall(r"INFO: (\d+) processes?: ([^\n]*)", text)
    return {"tag": tag, "exit": code, "seconds": round(time.time() - started), "processes": rows[-1][1] if rows else None}

for index in range(1, attempts + 1):
    first, second = run(f"pair{index}-A"), run(f"pair{index}-B")
    row = {"attempt": index, "A": first, "B": second, "warm": second["exit"] == 0 and second["processes"] is not None and "local" not in second["processes"] and "sandbox" not in second["processes"]}
    with open(os.path.join(out, "pairs.jsonl"), "a") as handle:
        handle.write(json.dumps(row) + "\n")
    if row["warm"]:
        break
