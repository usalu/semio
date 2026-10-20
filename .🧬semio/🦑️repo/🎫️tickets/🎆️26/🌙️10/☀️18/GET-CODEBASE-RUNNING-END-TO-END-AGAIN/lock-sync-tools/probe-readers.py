#!/usr/bin/env python3
"""📡️ Reader probe: measures what a dependency consumer sees while a lock/sync runs (cheap CLI boot, node_modules continuity per root, image free space, `dependencies ensure` wait).
Usage: probe-readers.py --out FILE.jsonl --stop STOPFILE [--interval 5] [--ensure-every 12] [--root DIR]; writes FILE.summary.json when STOPFILE appears or on SIGINT. Standard library only."""
import argparse, json, os, shutil, signal, subprocess, sys, threading, time

C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ap = argparse.ArgumentParser()
ap.add_argument("--root", default=os.path.abspath(os.path.join(C, *[".."] * 7)))
ap.add_argument("--out", required=True); ap.add_argument("--stop", required=True)
ap.add_argument("--interval", type=float, default=5); ap.add_argument("--ensure-every", type=int, default=12)
a = ap.parse_args()
ROOTS = [".", "♻️mit-bestand", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech"]
BUN = shutil.which("bun") or "bun"
NICE = ["nice", "-n", "10"] if os.name == "posix" else []
ENV = dict(os.environ, SEMIO_LANGUAGE="en")
stopping, lock, rows = threading.Event(), threading.Lock(), []
signal.signal(signal.SIGINT, lambda *_: stopping.set()); signal.signal(signal.SIGTERM, lambda *_: stopping.set())
out = open(a.out, "a", encoding="utf-8")

def emit(row):
    row["t"] = round(time.time(), 3)
    with lock: rows.append(row); out.write(json.dumps(row, ensure_ascii=False) + "\n"); out.flush()

def timed(kind, args, limit):
    began = time.monotonic()
    try:
        done = subprocess.run(NICE + [BUN, "repo"] + args, cwd=a.root, env=ENV, capture_output=True, text=True, timeout=limit)
        tail = [l for l in (done.stderr + done.stdout).splitlines() if "[DATA]" in l][-1:] or (done.stderr + done.stdout).splitlines()[-1:]
        emit({"kind": kind, "ms": round((time.monotonic() - began) * 1000), "rc": done.returncode, "line": (tail[0] if tail else "")[:200]})
    except subprocess.TimeoutExpired: emit({"kind": kind, "ms": round((time.monotonic() - began) * 1000), "rc": "timeout"})

def continuity():
    bad, targets = [], {}
    for r in ROOTS:
        nm = os.path.join(a.root, r, "node_modules")
        try:
            targets[r] = os.readlink(nm)
            os.stat(os.path.join(nm, "typescript", "package.json")); os.stat(os.path.join(nm, "@semio-tech"))
        except OSError as err: bad.append(f"{r}: {err.__class__.__name__}")
    free = shutil.disk_usage(os.path.join(a.root, ".cache", "b", "d", "m")).free
    emit({"kind": "continuity", "broken": bad, "targets": targets, "freeBytes": free})

tick, workers = 0, []
while not stopping.is_set() and not os.path.exists(a.stop):
    continuity(); timed("help", ["--help"], 120)
    if tick % a.ensure_every == 0:
        w = threading.Thread(target=timed, args=("ensure", ["dependencies", "ensure"], 3 * 3600), daemon=True); w.start(); workers.append(w)
    tick += 1; stopping.wait(a.interval)
for w in workers: w.join(timeout=600)
def stats(kind):
    v = sorted(r["ms"] for r in rows if r["kind"] == kind and isinstance(r.get("rc"), int))
    return {"n": len(v), "p50": v[len(v) // 2] if v else None, "p95": v[int(len(v) * .95) - 1] if v else None, "max": v[-1] if v else None, "failures": sum(1 for r in rows if r["kind"] == kind and r.get("rc") != 0)}
cont = [r for r in rows if r["kind"] == "continuity"]
summary = {"help": stats("help"), "ensure": stats("ensure"), "continuityTicks": len(cont), "continuityBroken": sum(1 for r in cont if r["broken"]), "targetChanges": len({json.dumps(r["targets"], sort_keys=True) for r in cont}) - 1 if cont else 0, "minFreeBytes": min((r["freeBytes"] for r in cont), default=None)}
json.dump(summary, open(a.out.replace(".jsonl", "") + ".summary.json", "w"), indent=1); print(json.dumps(summary))
