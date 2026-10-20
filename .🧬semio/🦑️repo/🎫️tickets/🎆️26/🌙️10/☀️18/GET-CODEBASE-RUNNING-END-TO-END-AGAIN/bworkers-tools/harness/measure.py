#!/usr/bin/env python3
# 📏 Measures the same Bazel build with and without persistent workers, alternating the two modes, and prints one JSON row per run.
# usage: measure.py <label> <graph.json> <tasks> <mode on|off> <repeat> [jobs] [extra BW_OPTIONS]
import json, os, re, subprocess, sys, time

TOOLS = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GO = os.path.join(TOOLS, "harness", "go.sh")
label, graph, tasks, only_mode, only_repeat = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], int(sys.argv[5])
jobs = sys.argv[6] if len(sys.argv) > 6 else "8"
extra = sys.argv[7] if len(sys.argv) > 7 else ""
OUT = os.environ.get("BW_MEASURE_OUT", f"/private/tmp/bworkers/m-{label}.jsonl")
BAZEL = "/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"


def rusage(path, offset):
    rows = []
    if not os.path.exists(path):
        return {"launches": 0, "cpu": 0.0, "maxrss_mb": 0, "size": 0}
    text = open(path, encoding="utf8", errors="replace").read()
    chunk = text[offset:]
    cpu = 0.0
    maxrss = 0
    launches = 0
    for match in re.finditer(r"\s*([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys", chunk):
        launches += 1
        cpu += float(match.group(2)) + float(match.group(3))
    for match in re.finditer(r"\s*(\d+)\s+maximum resident set size", chunk):
        maxrss = max(maxrss, int(match.group(1)) // 1048576)
    return {"launches": launches, "cpu": round(cpu, 2), "maxrss_mb": maxrss, "size": len(text)}


def build(base, mode, tag):
    for attempt in range(3):
        row = build_once(base, mode, tag)
        if row["ok"] or not row.get("source_changed"):
            row["attempts"] = attempt + 1
            return row
    return row


def build_once(base, mode, tag):
    log = os.path.join(base, "time.log")
    offset = os.path.getsize(log) if os.path.exists(log) else 0
    profile = os.path.join(base, f"profile-{tag}.json.gz")
    env = ["BW_GRAPH=" + graph, "BW_WORKERS=" + mode, "BW_SKIP_CACHE=1", "BW_BUN=/private/tmp/bworkers/bunw", f"BW_JOBS={jobs}", f"BW_OPTIONS=--profile={profile} {extra} {'--worker_quit_after_build' if tag == 'warm2' else ''}".strip()]
    started = time.time()
    run = subprocess.run([GO, base, tasks] + env, capture_output=True, text=True)
    wall = time.time() - started
    text = run.stdout + run.stderr
    with open(os.path.join(base, f"build-{tag}.log"), "w", encoding="utf8") as handle:
        handle.write(text)
    leaf = [int(x) for x in re.findall(r"\[DEBUG\] bw leaf=(\d+)ms", text)]
    request = [int(x) for x in re.findall(r"\[DEBUG\] bw request recheck=\d+ms total=(\d+)ms", text)]
    elapsed = re.search(r"Elapsed time: ([\d.]+)s, Critical Path: ([\d.]+)s", text)
    processes = re.search(r"INFO: (\d+) processes?: (.*)", text)
    row = {"label": label, "mode": mode, "tag": tag, "ok": "Build completed successfully" in text, "harness_s": round(wall, 1),
           "bazel_s": float(elapsed.group(1)) if elapsed else None, "critical_s": float(elapsed.group(2)) if elapsed else None,
           "processes": processes.group(2).strip() if processes else None}
    row.update(rusage(log, offset))
    row["profile"] = profile
    row["source_changed"] = "Captured implementation Source changed" in text
    marks = {}
    for name, value in re.findall(r"\[DEBUG\] bw (?:mark |birth )?(\w+)=(\d+)", text):
        if name != "leaf":
            marks.setdefault(name, []).append(int(value))
    row["marks_ms_median"] = {k: sorted(v)[len(v) // 2] for k, v in marks.items()}
    if leaf:
        row["leaf_ms_median"] = sorted(leaf)[len(leaf) // 2]
    if request:
        row["request_ms_median"] = sorted(request)[len(request) // 2]
    return row


def main():
    rows = []
    for repeat in [only_repeat]:
        for mode in [only_mode]:
            base = f"/private/tmp/bworkers/m-{label}-{mode}-{repeat}"
            subprocess.run(["rm", "-rf", base])
            subprocess.run([BAZEL, f"--output_base={base}/ob", "--install_base=/private/tmp/bworkers/install", "shutdown"], capture_output=True)
            cold = build(base, mode, "cold")
            warm = build(base, mode, "warm")
            warm2 = build(base, mode, "warm2")
            subprocess.run([BAZEL, f"--output_base={base}/ob", "--install_base=/private/tmp/bworkers/install", "shutdown"], capture_output=True)
            time.sleep(4)
            total = {"label": label, "mode": mode, "tag": "total", "repeat": repeat}
            total.update(rusage(os.path.join(base, "time.log"), 0))
            for tag, row in (("cold", cold), ("warm", warm), ("warm2", warm2), ("total", total)):
                row["repeat"] = repeat
                print(json.dumps(row), flush=True)
                with open(OUT, "a") as handle:
                    handle.write(json.dumps(row) + "\n")


main()
