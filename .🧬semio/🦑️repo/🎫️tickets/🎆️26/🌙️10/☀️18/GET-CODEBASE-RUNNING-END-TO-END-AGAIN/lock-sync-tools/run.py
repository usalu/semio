#!/usr/bin/env python3
"""🔐️ Round-5 run: apply batch -> `dependencies lock --all` -> `dependencies sync --all` twice -> nlink and light verification, with reader probes and timings.
  run.py                       preflight only (read only): guards, baseline, dry run of the batch
  run.py --yes                 all phases (needs the coordinator's ROUND5 GO; start detached: nohup nice -n 10 python3 run.py --yes > log 2>&1 & disown)
  run.py --yes --from lock     resume at a phase: preflight apply lock sync1 sync2 verify;   --only PHASE runs one phase
  run.py --yes --accept-lock-delta   continue after lockdiff reported unexpected rows (review them first)
Cancellation: Ctrl-C / SIGTERM stops the running child (SIGINT first) and leaves the phase marker; resume with --from. Standard library only."""
import argparse, json, os, shutil, signal, subprocess, sys, threading, time, datetime

HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ap = argparse.ArgumentParser()
ap.add_argument("--root", default=os.path.abspath(os.path.join(C, *[".."] * 7)))
ap.add_argument("--yes", action="store_true"); ap.add_argument("--from", dest="start", default="preflight"); ap.add_argument("--only")
ap.add_argument("--accept-lock-delta", action="store_true"); ap.add_argument("--run-dir")
ap.add_argument("--without", action="append", default=[]); ap.add_argument("--ignore-running", action="store_true"); ap.add_argument("--max-load", type=float, default=30)
a = ap.parse_args()
PHASES = ["preflight", "apply", "lock", "sync1", "sync2", "verify"]
BUN = shutil.which("bun") or "bun"; PY = sys.executable
NICE = ["nice", "-n", "10"] if os.name == "posix" else []
ENV = dict(os.environ, SEMIO_LANGUAGE="en")
BASE = os.path.join(C, "🗑️generated", "lock-sync")
RESUME = a.start != "preflight" or a.only
RUN = a.run_dir or (sorted(d for d in os.listdir(BASE) if d.startswith("run-"))[-1] if RESUME and os.path.isdir(BASE) and any(d.startswith("run-") for d in os.listdir(BASE)) else None)
RUN = os.path.join(BASE, RUN) if RUN and not os.path.isabs(RUN) else (RUN or os.path.join(BASE, "run-" + datetime.datetime.now().strftime("%Y%m%dT%H%M%S")))
os.makedirs(RUN, exist_ok=True)
child = None
def say(msg):
    line = datetime.datetime.now().strftime("%H:%M:%S ") + "[lock-sync] " + msg; print(line, flush=True)
    open(os.path.join(RUN, "run.log"), "a", encoding="utf-8").write(line + "\n")
def stop(*_):
    say("cancel requested")
    if child and child.poll() is None: child.send_signal(signal.SIGINT)
    raise SystemExit(130)
signal.signal(signal.SIGINT, stop); signal.signal(signal.SIGTERM, stop)

def run(name, args, cwd=None, heartbeat=60):
    global child
    log = open(os.path.join(RUN, name + ".log"), "w", encoding="utf-8"); began = time.monotonic()
    child = subprocess.Popen(args, cwd=cwd or a.root, env=ENV, stdout=log, stderr=subprocess.STDOUT, text=True)
    while child.poll() is None:
        time.sleep(1)
        if int(time.monotonic() - began) % heartbeat == 0: say(f"{name} running {int(time.monotonic() - began)} s")
    secs = round(time.monotonic() - began, 1); log.close(); text = open(os.path.join(RUN, name + ".log"), encoding="utf-8", errors="replace").read()
    say(f"{name} rc={child.returncode} in {secs} s"); return child.returncode, secs, text
def py(script, *args): return [PY, "-I", os.path.join(HERE, script), *args]
def guard():
    ps = subprocess.run(["ps", "-axo", "pid,args"], capture_output=True, text=True).stdout.splitlines()
    return [l.strip()[:160] for l in ps if ("bun repo" in l or "script.ts repo" in l) and not any(k in l for k in ("run.py", "probe-readers", "selftest", "grep", "dashboard", "mcp stdio"))]
def snapshot(tag):
    d = os.path.join(RUN, tag); os.makedirs(d, exist_ok=True)
    for r in [".", "♻️mit-bestand", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech"]:
        shutil.copyfile(os.path.join(a.root, r, "bun.lock"), os.path.join(d, ("root" if r == "." else r) + ".bun.lock"))
    return d
def targets(): return {r: os.path.realpath(os.path.join(a.root, r, "node_modules")) for r in [".", "♻️mit-bestand", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech"]}
def state(): return json.load(open(os.path.join(RUN, "state.json"))) if os.path.exists(os.path.join(RUN, "state.json")) else {}
def save(**kw): s = state(); s.update(kw); json.dump(s, open(os.path.join(RUN, "state.json"), "w"), indent=1, ensure_ascii=False)
def fail(msg): say("STOP: " + msg); save(failed=msg); raise SystemExit(1)

def preflight():
    busy = [] if a.ignore_running else guard()
    if busy: fail("repo commands are running; announce the freeze (any build-module or manifest edit aborts them): " + "; ".join(busy[:3]))
    load = os.getloadavg()[0] if hasattr(os, "getloadavg") else 0
    free = shutil.disk_usage(os.path.join(a.root, ".cache", "b", "d", "m")).free
    inst = os.path.isdir(os.path.join(a.root, ".nx", "installation"))
    cache = os.path.join(a.root, ".cache", "b", "d", "m", "bun-cache")
    if not (os.path.isdir(cache) and os.listdir(cache)): fail("warm bun cache missing in the dependencies image; lock needs registry metadata (offline lock cannot resolve)")
    say(f"load {load:.1f}, dependencies image free {free / 2**30:.2f} GiB, .nx/installation resolves: {inst}")
    if not inst and "nx" not in a.without: fail(".nx/installation does not resolve (run `bun repo tools setup nx`, or pass --without nx)")
    if load > a.max_load: fail(f"load average above {a.max_load}")
    rc, _, text = run("nlink-before", py("../sync-incr-tools/nlink.py", a.root))
    if rc or "0 with nlink" not in text: fail("hard-linked manifests: " + text[-300:])
    rc, _, text = run("lockdelta-before", py("lockdelta.py", "--root", a.root, "--json", os.path.join(RUN, "lockdelta-before.json")))
    rep = json.load(open(os.path.join(RUN, "lockdelta-before.json")))
    if any(r["added"] or r["removed"] or r["changed"] or r["missing"] for r in rep.values()): fail("manifests and locks already differ before the batch: " + text[-400:])
    grp = sum([["--without", g] for g in a.without], [])
    rc, _, text = run("apply-dry", py("apply.py", "--root", a.root, *grp))
    if rc: fail("batch drift: " + text[-600:])
    save(targetsBefore=targets(), imageFreeBefore=free, without=a.without)
    say("preflight ok")

def apply():
    snapshot("lock-before")
    grp = sum([["--without", g] for g in a.without], [])
    rc, _, text = run("apply", py("apply.py", "--root", a.root, "--write", *grp, *(["--ignore-running"] if a.ignore_running else [])))
    if rc: fail("apply refused: " + text[-600:])
    rc, _, text = run("lockdelta-predicted", py("lockdelta.py", "--root", a.root, "--json", os.path.join(RUN, "lockdelta-after-apply.json")))
    rep = json.load(open(os.path.join(RUN, "lockdelta-after-apply.json")))
    want = {r: (len(v["added"]), len(v["changed"])) for r, v in rep.items()}
    say("predicted lock delta (added members, changed rows) per root: " + json.dumps(want, ensure_ascii=False)); save(predicted=want)

def lock():
    rc, secs, text = run("lock", [*NICE, BUN, "repo", "dependencies", "lock", "--all"])
    save(lockSeconds=secs, lockRc=rc)
    if rc: fail("dependencies lock --all failed; manifests stay edited (undo: python3 apply.py --undo --write)")
    rc, _, text = run("lockdelta-after-lock", py("lockdelta.py", "--root", a.root, "--json", os.path.join(RUN, "lockdelta-after-lock.json")))
    rep = json.load(open(os.path.join(RUN, "lockdelta-after-lock.json")))
    if any(r["added"] or r["removed"] or r["changed"] or r["missing"] for r in rep.values()): fail("locks do not match manifests after lock --all")
    after = snapshot("lock-after")
    rc, _, text = run("lockdiff", py("lockdiff.py", "--before", os.path.join(RUN, "lock-before"), "--after", after))
    if rc and not a.accept_lock_delta: fail("lockdiff reports unexpected lock rows (see lockdiff.log); review, then --from sync1 --accept-lock-delta")

def sync(name):
    stopfile = os.path.join(RUN, name + ".stop"); 
    if os.path.exists(stopfile): os.remove(stopfile)
    probe = subprocess.Popen(py("probe-readers.py", "--root", a.root, "--out", os.path.join(RUN, name + "-probe.jsonl"), "--stop", stopfile), cwd=a.root, env=ENV, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try: rc, secs, text = run(name, [*NICE, BUN, "repo", "dependencies", "sync", "--all"])
    finally:
        open(stopfile, "w").write("stop"); probe.wait(timeout=1800)
    save(**{name + "Seconds": secs, name + "Rc": rc, name + "Current": "Store dependency sync current" in text, name + "Stale": [l[:200] for l in text.splitlines() if "sync stale" in l][:2]})
    if rc: fail(f"{name} failed")
    return text

def sync1(): sync("sync1")
def sync2():
    text = sync("sync2")
    if "Store dependency sync current" not in text: fail("second sync did not report current")

def verify():
    res = {}
    rc, _, text = run("nlink-after", py("../sync-incr-tools/nlink.py", a.root)); res["nlink0"] = rc == 0 and "0 with nlink" in text
    rc, _, text = run("lockdelta-final", py("lockdelta.py", "--root", a.root, "--json", os.path.join(RUN, "lockdelta-final.json")))
    rep = json.load(open(os.path.join(RUN, "lockdelta-final.json"))); res["locksMatchManifests"] = not any(r["added"] or r["removed"] or r["changed"] for r in rep.values())
    res["helpSeconds"] = [run("help%d" % i, [*NICE, BUN, "repo", "--help"])[1] for i in range(3)]
    res["ensureSeconds"] = []; res["ensureRc"] = []
    for i in range(3):
        rc, secs, text = run("ensure%d" % i, [*NICE, BUN, "repo", "dependencies", "ensure"]); res["ensureSeconds"].append(secs); res["ensureRc"].append(rc)
    now, before = targets(), state().get("targetsBefore", {})
    res["nodeModulesSwitched"] = {r: (now[r] != before.get(r)) and os.path.isdir(now[r]) for r in now}
    root = os.path.join(a.root, "node_modules", "@semio-tech", "build-parsers")
    res["rootHasBuildParsers"] = os.path.exists(root)
    pointers = os.path.join(a.root, ".cache", "b", "d", "m", "pointer"); res["pointerEpochs"] = {}
    if os.path.isdir(pointers):
        for f in sorted(os.listdir(pointers)):
            try: p = json.load(open(os.path.join(pointers, f))); res["pointerEpochs"][p["scope"]] = p["epoch"]
            except Exception: pass
    res["imageFreeAfterGiB"] = round(shutil.disk_usage(os.path.join(a.root, ".cache", "b", "d", "m")).free / 2**30, 2)
    for name in ("sync1", "sync2"):
        f = os.path.join(RUN, name + "-probe.summary.json")
        if os.path.exists(f): res[name + "Probe"] = json.load(open(f))
    save(verify=res); say("verify: " + json.dumps(res, ensure_ascii=False))
    ok = res["nlink0"] and res["locksMatchManifests"] and all(v == 0 for v in res["ensureRc"]) and all(res["nodeModulesSwitched"].values())
    if not ok: fail("verification found problems (see state.json)")
    say("ALL GREEN")

TABLE = dict(preflight=preflight, apply=apply, lock=lock, sync1=sync1, sync2=sync2, verify=verify)
todo = [a.only] if a.only else PHASES[PHASES.index(a.start):]
if not a.yes: todo = ["preflight"]
say(f"run dir {RUN}; phases {todo}")
for ph in todo: say("== " + ph); TABLE[ph]()
