"""proof.py <target> [probe-file]: Nx cache proof of the engine digest split on the default route (`bun repo run <target>`).
A (cold or warm) -> append a one-line comment to a scheduling-domain file of the engine closure -> B -> remove exactly that suffix again.
Expectation: B hits the cache for every cacheable task with the SAME task hashes as A (the engine digest is unchanged).
Per run: wall seconds, peak RSS of the whole process tree (ps sample every 3 s), engine digest before/after, run.json copy.
Streaming only; aborts if its own RSS exceeds 200 MB. Logs in C/🗑️generated/nxperf-g/proof/."""
import os, resource, shutil, subprocess, sys, threading, time, hashlib
ROOT = "/Users/ueli/Documents/semio"
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OUT = f"{ROOT}/{C}/🗑️generated/nxperf-g/proof"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
ENTRY = f"{LIB}/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟨️.mjs"
RUNJSON = f"{ROOT}/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts/run.json"
target = sys.argv[1]
probe = sys.argv[2] if len(sys.argv) > 2 else "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧾️refusals/🟦️.ts"
SUFFIX = b"\n// nxperf-g probe (temporary)\n"
os.makedirs(OUT, exist_ok=True)
OUT = OUT + "/" + (sys.argv[3] if len(sys.argv) > 3 else "r2"); os.makedirs(OUT, exist_ok=True)
os.chdir(ROOT)

def say(text):
    line = f"{time.strftime('%H:%M:%S')} {text}"
    print(line, flush=True)
    open(f"{OUT}/proof.log", "a").write(line + "\n")

def guard():
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 200 * 1024 * 1024:
        raise SystemExit("own RSS above 200 MB")

def digest(family="generic"):
    return subprocess.run(["bun", ENTRY] + ([] if family == "generic" else [family]), capture_output=True, text=True).stdout.strip()

def tree_rss(root):
    rows = subprocess.run(["ps", "-axo", "pid=,ppid=,rss="], capture_output=True, text=True).stdout.splitlines()
    kids, rss = {}, {}
    for row in rows:
        parts = row.split()
        if len(parts) == 3:
            kids.setdefault(int(parts[1]), []).append(int(parts[0])); rss[int(parts[0])] = int(parts[2])
    total, stack = 0, [root]
    while stack:
        pid = stack.pop(); total += rss.get(pid, 0); stack += kids.get(pid, [])
    return total / 1024

def retrying(call, attempts=30):
    for attempt in range(attempts):
        try: return call()
        except OSError as error:
            if error.errno not in (23, 24) or attempt == attempts - 1: raise
            time.sleep(10)

def run(name):
    log = f"{OUT}/{name}.raw"
    retrying(lambda: open(log, "wb").close())
    fh = retrying(lambda: open(log, "ab", buffering=0))
    started = time.time()
    p = subprocess.Popen(["nice", "-n", "10", "bun", "repo", "run", target], stdout=fh, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
    open(f"{OUT}/{name}.pid", "w").write(f"pid={p.pid} {time.ctime()}")
    peak = [0.0]; stop = threading.Event(); drift = []
    def sample():
        last, tick = digest(), 0
        while not stop.is_set():
            peak[0] = max(peak[0], tree_rss(p.pid)); guard(); stop.wait(3); tick += 1
            if tick % 40 == 0:
                now = digest()
                if now != last: drift.append((time.strftime("%H:%M:%S"), now[:16])); last = now
    thread = threading.Thread(target=sample, daemon=True); thread.start()
    seen = None
    while p.poll() is None:
        time.sleep(2)
        text = open(log, "rb").read().decode("utf-8", "replace")
        if seen is None and ("Run duration:" in text or "Pass --verbose to see the stacktrace" in text): seen = time.time()
        if seen and time.time() - seen > 20: break
    if p.poll() is None:
        for sig in ("-TERM", "-KILL"):
            order, stack = [], [p.pid]
            rows = subprocess.run(["ps", "-axo", "pid=,ppid="], capture_output=True, text=True).stdout.splitlines()
            kids = {}
            for row in rows:
                t = row.split()
                if len(t) == 2: kids.setdefault(int(t[1]), []).append(int(t[0]))
            def walk(x):
                for k in kids.get(x, []): walk(k)
                order.append(x)
            walk(p.pid)
            for q in order: subprocess.run(["kill", sig, str(q)], stderr=subprocess.DEVNULL)
            time.sleep(3)
        rc = "nx-reaped"
    else:
        rc = p.returncode
    stop.set()
    try: shutil.copy(RUNJSON, f"{OUT}/{name}.run.json")
    except OSError: pass
    text = open(log, "rb").read().decode("utf-8", "replace")
    cache = [l.strip() for l in text.splitlines() if "Cache:" in l or "Successfully ran" in l or "Nx exit code" in l][-3:]
    say(f"{name}: rc={rc} wall={time.time()-started:.0f}s peak_tree_rss_mb={peak[0]:.0f} engine_digest_changes_during_run={drift} {cache}")

say(f"start target={target} probe={probe}")
d0 = digest(); n0 = digest("native"); say(f"engine digest generic={d0[:16]} native={n0[:16]}")
run("A")
d1 = digest(); n1_ref = digest("native"); say(f"engine digest after A generic={d1[:16]} (same as before: {d1 == d0})")
path = f"{ROOT}/{probe}"
original = open(path, "rb").read(); retrying(lambda: open(path, "ab").write(SUFFIX))
try:
    n1 = n1_ref; d2 = digest(); n2 = digest("native"); say(f"probe applied to {probe}: engine digest generic={d2[:16]} (unchanged: {d2 == d1}) native={n2[:16]} (unchanged: {n2 == n1})")
    run("B")
finally:
    def restore():
        data = open(path, "rb").read()
        if data.endswith(SUFFIX): open(path, "wb").write(data[:-len(SUFFIX)])
    retrying(restore, 200)
    say(f"probe removed: sha256 {hashlib.sha256(open(path,'rb').read()).hexdigest()[:16]} (original {hashlib.sha256(original).hexdigest()[:16]})")
d3 = digest(); say(f"engine digest after revert generic={d3[:16]} (same as before: {d3 == d0})")
say("done")
