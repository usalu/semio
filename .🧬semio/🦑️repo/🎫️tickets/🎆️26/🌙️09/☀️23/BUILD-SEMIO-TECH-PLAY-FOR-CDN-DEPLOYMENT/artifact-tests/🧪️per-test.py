#!/usr/bin/env python3
"""🧪️ Runs every test of each compiled artifact lib test binary in its own process (RUST_MIN_STACK=64 MiB, resumable, per-test timeout, overall budget).

usage: 🧪️per-test.py <sweep-results.jsonl> <out-dir> [--only PACKAGE_SUBSTR] [--jobs N] [--timeout S] [--budget S] [--rerun-failed] [--keep-binary]
A fully recorded package's test unit directory is deleted afterwards (disk), unless --keep-binary.
Reads the latest `ok` row per package from 🧪️sweep.py, writes <out-dir>/<package>.jsonl rows {name, status: pass|ignored|fail|timeout, secs, tail}
and prints one summary line per package.
"""
import argparse, concurrent.futures as cf, json, os, shutil, subprocess, time

p = argparse.ArgumentParser()
p.add_argument("results"); p.add_argument("out")
p.add_argument("--only", default=""); p.add_argument("--jobs", type=int, default=4); p.add_argument("--timeout", type=int, default=300)
p.add_argument("--budget", type=int, default=560); p.add_argument("--rerun-failed", action="store_true"); p.add_argument("--keep-binary", action="store_true")
a = p.parse_args()
start = time.time()
REPO = "/Users/ueli/Documents/semio"
latest = {}
for l in open(a.results):
    try: j = json.loads(l)
    except Exception: continue
    latest[j["package"]] = j
os.makedirs(a.out, exist_ok=True)

def run(binary, cwd, name):
    if time.time() - start > a.budget: return None
    env = dict(os.environ, RUST_MIN_STACK="67108864", RUST_BACKTRACE="0", CARGO_MANIFEST_DIR=cwd)
    t = time.time()
    try:
        r = subprocess.run([binary, "--exact", name, "--test-threads", "1"], cwd=cwd, env=env, capture_output=True, text=True, timeout=a.timeout)
        ok = r.returncode == 0 and "1 passed" in r.stdout
        ignored = r.returncode == 0 and "1 ignored" in r.stdout
        return {"name": name, "status": "pass" if ok else "ignored" if ignored else "fail", "rc": r.returncode, "secs": round(time.time() - t, 2), "tail": "" if ok else (r.stdout + r.stderr)[-2500:]}
    except subprocess.TimeoutExpired:
        return {"name": name, "status": "timeout", "rc": None, "secs": a.timeout, "tail": ""}

for package, row in latest.items():
    if row["status"] != "ok" or a.only not in package or not row.get("binary"): continue
    cwd = os.path.dirname(os.path.join(REPO, row["manifest"]))
    binary = row["binary"]
    if not os.path.exists(binary): print(f"missing-binary {package}", flush=True); continue
    env = dict(os.environ, RUST_MIN_STACK="67108864")
    names = [l[:-len(": test")] for l in subprocess.run([binary, "--list", "--format", "terse"], cwd=cwd, env=env, capture_output=True, text=True).stdout.splitlines() if l.endswith(": test")]
    path = os.path.join(a.out, f"{package}.jsonl")
    recorded = {}
    if os.path.exists(path):
        for l in open(path):
            try: r = json.loads(l); recorded[r["name"]] = r["status"]
            except Exception: pass
    todo = [n for n in names if n not in recorded or (a.rerun_failed and recorded[n] not in ("pass", "ignored"))]
    with open(path, "a") as out, cf.ThreadPoolExecutor(a.jobs) as ex:
        for res in ex.map(lambda n: run(binary, cwd, n), todo):
            if res is None: continue
            recorded[res["name"]] = res["status"]; out.write(json.dumps(res) + "\n"); out.flush()
    rows = {k: v for k, v in recorded.items() if k in set(names)}
    if len(rows) == len(names) and not a.keep_binary: shutil.rmtree(os.path.dirname(os.path.dirname(binary)), ignore_errors=True)
    print(f"{package}: listed={len(names)} recorded={len(rows)} pass={sum(v=='pass' for v in rows.values())} fail={sum(v=='fail' for v in rows.values())} timeout={sum(v=='timeout' for v in rows.values())} ignored={sum(v=='ignored' for v in rows.values())}", flush=True)
    if time.time() - start > a.budget: print("budget", flush=True); break
