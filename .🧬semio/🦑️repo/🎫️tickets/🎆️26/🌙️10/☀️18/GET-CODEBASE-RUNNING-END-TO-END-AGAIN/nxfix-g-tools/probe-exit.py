"""probe-exit.py <name> <repo args...>: runs `nice -n 10 bun repo <args>` in its own session (log: C/🗑️generated/nxfix-g/<name>.raw),
and from the moment the Nx summary ('Successfully ran' / 'Failed tasks') is printed records, every 3 s, the process tree of the run (pid, ppid, state, %cpu, rss, command)
and the open file descriptors of the loader (counts by type, pipes/sockets named), plus a `sample` of the loader at +30 s when it still lives.
It never kills before 15 min after the summary (own pids only). Output: <name>.probe.jsonl; last line = {"exit_after_summary_s":..,"rc":..}. Streams the log (tail reads only)."""
import json, os, re, subprocess, sys, time
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name, args = sys.argv[1], sys.argv[2:]
log, probe = f"{C}/🗑️generated/nxfix-g/{name}.raw", f"{C}/🗑️generated/nxfix-g/{name}.probe.jsonl"
os.chdir("/Users/ueli/Documents/semio")
fh = open(log, "wb", buffering=0)
p = subprocess.Popen(["nice", "-n", "10", "bun", "repo"] + args, stdout=fh, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, start_new_session=True)
open(f"{C}/🗑️generated/nxfix-g/{name}.pid", "w").write(f"pid={p.pid} {time.ctime()}\n")

def rows():
    out = subprocess.run(["ps", "-axo", "pid=,ppid=,stat=,pcpu=,rss=,etime=,command="], capture_output=True, text=True).stdout.split("\n")
    kids, info = {}, {}
    for l in out:
        t = l.split(None, 6)
        if len(t) < 7: continue
        pid, ppid = int(t[0]), int(t[1])
        kids.setdefault(ppid, []).append(pid); info[pid] = t
    order, stack = [], [p.pid]
    while stack:
        x = stack.pop()
        if x in info: order.append(info[x])
        stack.extend(kids.get(x, []))
    return order

def tail(n=65536):
    with open(log, "rb") as f:
        f.seek(0, 2); size = f.tell(); f.seek(max(0, size - n)); return f.read().decode("utf-8", "replace")

def fds(pid):
    r = subprocess.run(["lsof", "-nP", "-p", str(pid)], capture_output=True, text=True, timeout=20).stdout.split("\n")[1:]
    kinds = {}
    for l in r:
        t = l.split(None, 8)
        if len(t) >= 5: kinds[t[4]] = kinds.get(t[4], 0) + 1
    return kinds

t0 = None; sampled = False; started = time.time()
while p.poll() is None:
    time.sleep(3)
    if t0 is None:
        txt = tail()
        if "Successfully ran" in txt or "Failed tasks" in txt: t0 = time.time()
        continue
    el = time.time() - t0
    tree = rows()
    row = {"t": round(el, 1), "tree": [[r[0], r[1], r[2], r[3], r[4], r[5], r[6][:110]] for r in tree[:40]], "n": len(tree)}
    loaders = [int(r[0]) for r in tree if "bun" in r[6] and "repo" in r[6]][:2]
    try: row["fds"] = {str(x): fds(x) for x in loaders}
    except Exception as e: row["fds_error"] = str(e)
    open(probe, "a").write(json.dumps(row, ensure_ascii=False) + "\n")
    if el > 30 and not sampled and loaders:
        sampled = True
        subprocess.run(["sample", str(loaders[0]), "3", "-file", f"{C}/🗑️generated/nxfix-g/{name}.sample.txt"], capture_output=True)
    if el > 900:
        for sig in ("-TERM", "-KILL"):
            for r in reversed(tree): subprocess.run(["kill", sig, r[0]], stderr=subprocess.DEVNULL)
            time.sleep(3)
        break
rc = p.poll()
open(probe, "a").write(json.dumps({"exit_after_summary_s": None if t0 is None else round(time.time() - t0, 1), "rc": rc, "total_s": round(time.time() - started)}) + "\n")
