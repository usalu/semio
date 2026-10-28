#!/usr/bin/env python3
"""tree-cpu.py [--interval S] [--out FILE] -- command...: runs the command and polls `ps` for every descendant; prints one row per process (pid, role, wall s, cpu s (user+sys, the last value `ps` showed), peak RSS MB, command head) and the totals. The role is the first recognisable word of the command line. Streams: it keeps one row per pid, never the samples."""
import os, re, subprocess, sys, time

def parse_time(text):
    days = 0
    if "-" in text:
        d, text = text.split("-", 1); days = int(d)
    parts = [float(x) for x in text.split(":")]
    sec = 0.0
    for p in parts:
        sec = sec * 60 + p
    return days * 86400 + sec

def role(command):
    if "--eval" in command and "closed-immutable" not in command and "bun" in command: return "loader"
    for key in ("door/", "repo-watch", "bazel", "java", "cargo", "rustc", "sysctl", "hdiutil", "ps ", "git "):
        if key in command: return key.strip("/ ")
    return "other"

interval = 0.1; out = None; args = sys.argv[1:]
while args and args[0] != "--":
    if args[0] == "--interval": interval = float(args[1]); args = args[2:]
    elif args[0] == "--out": out = args[1]; args = args[2:]
    else: sys.exit("bad arg")
cmd = args[1:]
t0 = time.time()
proc = subprocess.Popen(cmd)
rows = {}
def poll():
    ps = subprocess.run(["ps", "-axo", "pid=,ppid=,time=,rss=,etime=,command="], capture_output=True, text=True).stdout.splitlines()
    table = {}
    for line in ps:
        m = re.match(r"\s*(\d+)\s+(\d+)\s+(\S+)\s+(\d+)\s+(\S+)\s+(.*)$", line)
        if m: table[int(m.group(1))] = (int(m.group(2)), parse_time(m.group(3)), int(m.group(4)), m.group(6))
    mine = {proc.pid}
    changed = True
    while changed:
        changed = False
        for pid, (ppid, *_r) in table.items():
            if ppid in mine and pid not in mine: mine.add(pid); changed = True
    now = time.time()
    for pid in mine:
        if pid not in table: continue
        ppid, cpu, rss, command = table[pid]
        row = rows.setdefault(pid, {"first": now, "last": now, "cpu": 0.0, "rss": 0, "command": command, "ppid": ppid})
        row["last"] = now; row["cpu"] = max(row["cpu"], cpu); row["rss"] = max(row["rss"], rss)
while proc.poll() is None:
    poll(); time.sleep(interval)
poll()
wall = time.time() - t0
total = 0.0
lines = []
for pid, r in sorted(rows.items(), key=lambda kv: kv[1]["first"]):
    total += r["cpu"]
    lines.append(f"{pid:>7} {role(r['command']):<10} start {r['first']-t0:6.1f}s wall {r['last']-r['first']:6.1f}s cpu {r['cpu']:6.2f}s rss {r['rss']/1024:6.0f}MB  {r['command'][:90]}")
text = "\n".join(lines) + f"\n# exit={proc.returncode} wall={wall:.1f}s processes={len(rows)} cpu(sum of last seen)={total:.1f}s\n"
sys.stderr.write(text)
if out: open(out, "w").write(text)
sys.exit(proc.returncode or 0)
