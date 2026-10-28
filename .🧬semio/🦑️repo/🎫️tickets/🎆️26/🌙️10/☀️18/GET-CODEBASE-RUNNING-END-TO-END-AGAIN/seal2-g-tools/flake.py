#!/usr/bin/env python3
"""🎲️ Hunts the sealed light command that exits 2 without output (run INSIDE a mirror: mm run <n> -- python3 -I flake.py <runs> <scenario...> [--out FILE]).
Scenarios (combine): plain | edit (a peer edits a closure file every 0.4 s) | promote (another process promotes continuously) | freeze (SIGSTOP the group at a random moment, SIGCONT 0.5 s later) | emfile=<n> (RLIMIT_NOFILE n) | slow (renice the child group).
Every run is `bun repo --help` sealed; a run is BAD when its exit is not 0 or its output lacks `Usage:`. Streams one line per bad run (and a summary) to FILE; keeps no output beyond 4 KB per bad run."""
import json, os, random, resource, signal, subprocess, sys, threading, time
ARGS = sys.argv[1:]
RUNS = int(ARGS.pop(0)); OUT = None
if "--out" in ARGS:
    i = ARGS.index("--out"); OUT = ARGS[i + 1]; del ARGS[i:i + 2]
SCEN = set(ARGS)
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
EDIT = B + "/🏛️sealed/🟦️.ts"
EMFILE = next((int(s.split("=")[1]) for s in SCEN if s.startswith("emfile=")), None)
stop = threading.Event(); bad = []; lock = threading.Lock()
def log(line):
    print(line, flush=True)
    if OUT:
        try:
            with open(OUT, "a", encoding="utf-8") as f: f.write(line + "\n")
        except OSError as error: print("[DEBUG] log write failed: %s" % error, flush=True)
def editor():
    original = open(EDIT, encoding="utf-8").read()
    n = 0
    try:
        while not stop.is_set():
            n += 1
            open(EDIT, "w", encoding="utf-8").write(original + "\n// [DEBUG] flake edit %d\n" % n)
            stop.wait(0.4)
    finally:
        open(EDIT, "w", encoding="utf-8").write(original)
def promoter():
    while not stop.is_set():
        p = subprocess.Popen(["bun", "repo", "--help"], env=dict(os.environ, SEMIO_REPO_IMPLEMENTATION="promote"), stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
        while p.poll() is None and not stop.is_set(): time.sleep(0.2)
        if p.poll() is None: os.killpg(p.pid, signal.SIGTERM); p.wait()
def limit():
    if EMFILE: resource.setrlimit(resource.RLIMIT_NOFILE, (EMFILE, EMFILE))
    os.setsid()
if "edit" in SCEN: threading.Thread(target=editor, daemon=True).start()
if "promote" in SCEN: threading.Thread(target=promoter, daemon=True).start()
counts = {"ok": 0, "bad": 0}; started = time.time()
for n in range(1, RUNS + 1):
    env = dict(os.environ, SEMIO_REPO_IMPLEMENTATION="sealed", SEMIO_LANGUAGE="en")
    t0 = time.perf_counter()
    p = subprocess.Popen(["bun", "repo", "--help"], env=env, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, preexec_fn=limit)
    frozen = None
    if "freeze" in SCEN:
        time.sleep(random.uniform(0, 0.8))
        if p.poll() is None:
            try: os.killpg(p.pid, signal.SIGSTOP); frozen = time.perf_counter() - t0; time.sleep(0.5); os.killpg(p.pid, signal.SIGCONT)
            except ProcessLookupError: pass
    try: out, err = p.communicate(timeout=300)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid, signal.SIGKILL); out, err = p.communicate(); err += b"\n[DEBUG] TIMEOUT"
    wall = time.perf_counter() - t0
    good = p.returncode == 0 and b"Usage:" in out
    counts["ok" if good else "bad"] += 1
    if not good:
        log(json.dumps({"run": n, "exit": p.returncode, "wall": round(wall, 2), "frozenAt": frozen, "stdoutBytes": len(out), "stderr": err[-1500:].decode("utf-8", "replace"), "stdoutHead": out[:200].decode("utf-8", "replace"), "load": os.getloadavg()[0]}, ensure_ascii=False))
    elif n % 25 == 0: log("[DEBUG] run %d ok=%d bad=%d wall %.1f s" % (n, counts["ok"], counts["bad"], time.time() - started))
stop.set(); time.sleep(1)
log("SUMMARY scenario=%s runs=%d ok=%d bad=%d" % (",".join(sorted(SCEN)), RUNS, counts["ok"], counts["bad"]))
