#!/usr/bin/env python3
"""🧪️ Runs every test of one libtest binary in its own process (resumable JSONL, per-test timeout, overall deadline).

usage: 🧪️per-test.py <binary> <cwd> <results.jsonl> [--jobs N] [--timeout S] [--deadline S] [--filter SUBSTR]
"""
import argparse, concurrent.futures as cf, json, os, subprocess, sys, time

p = argparse.ArgumentParser()
p.add_argument("binary"); p.add_argument("cwd"); p.add_argument("results")
p.add_argument("--jobs", type=int, default=6); p.add_argument("--timeout", type=int, default=300)
p.add_argument("--deadline", type=int, default=540); p.add_argument("--filter", default="")
a = p.parse_args(); a.binary = os.path.abspath(a.binary)
start = time.time()
env = dict(os.environ, RUST_MIN_STACK="268435456", RUST_BACKTRACE="0", CARGO_MANIFEST_DIR=os.path.abspath(a.cwd))
names = [l[:-len(": test")] for l in subprocess.run([a.binary, "--list", "--format", "terse"], cwd=a.cwd, env=env, capture_output=True, text=True).stdout.splitlines() if l.endswith(": test")]
names = [n for n in names if a.filter in n]
done = set()
if os.path.exists(a.results):
    for l in open(a.results):
        try: done.add(json.loads(l)["name"])
        except Exception: pass
todo = [n for n in names if n not in done]
print(f"total={len(names)} done={len(done & set(names))} todo={len(todo)}", flush=True)

def run(name):
    if time.time() - start > a.deadline: return None
    t = time.time()
    try:
        r = subprocess.run([a.binary, "--exact", name, "--test-threads", "1", "--nocapture"], cwd=a.cwd, env=env, capture_output=True, text=True, timeout=a.timeout)
        ok = r.returncode == 0 and "test result: ok. 1 passed" in r.stdout
        tail = "" if ok else (r.stdout + r.stderr)[-1500:]
        return {"name": name, "status": "pass" if ok else "fail", "rc": r.returncode, "secs": round(time.time() - t, 2), "tail": tail}
    except subprocess.TimeoutExpired:
        return {"name": name, "status": "timeout", "rc": None, "secs": a.timeout, "tail": ""}

with open(a.results, "a") as out, cf.ThreadPoolExecutor(a.jobs) as ex:
    for res in ex.map(run, todo):
        if res is None: continue
        out.write(json.dumps(res) + "\n"); out.flush()
rows = {}
for l in open(a.results):
    try: r = json.loads(l); rows[r["name"]] = r["status"]
    except Exception: pass
rows = {k: v for k, v in rows.items() if k in set(names)}
print(f"recorded={len(rows)}/{len(names)} pass={sum(v=='pass' for v in rows.values())} fail={sum(v=='fail' for v in rows.values())} timeout={sum(v=='timeout' for v in rows.values())}", flush=True)
