"""measure.py <label> <out.jsonl> -- <cmd...> : runs cmd, samples the RSS of its whole process tree every 0.5 s, appends one JSON line (wall, peak tree RSS, peak process count, biggest single process, load, swap)."""
import json, os, resource, subprocess, sys, time

def table():
    out = subprocess.run(["ps", "-axo", "pid=,ppid=,rss=,command="], capture_output=True, text=True).stdout
    rows = {}
    for line in out.splitlines():
        parts = line.split(None, 3)
        if len(parts) == 4 and parts[0].isdigit():
            rows[int(parts[0])] = (int(parts[1]), int(parts[2]), parts[3])
    return rows

def tree(rows, root):
    kids = {}
    for pid, (ppid, _, _) in rows.items():
        kids.setdefault(ppid, []).append(pid)
    seen, stack = set(), [root]
    while stack:
        pid = stack.pop()
        if pid in seen or pid not in rows:
            continue
        seen.add(pid)
        stack.extend(kids.get(pid, []))
    return seen

def swap():
    try:
        return subprocess.run(["sysctl", "-n", "vm.swapusage"], capture_output=True, text=True).stdout.strip()
    except Exception:
        return None

label, out = sys.argv[1], sys.argv[2]
cmd = sys.argv[sys.argv.index("--") + 1:]
load0, swap0, t0, ru0 = os.getloadavg(), swap(), time.time(), resource.getrusage(resource.RUSAGE_CHILDREN)
proc = subprocess.Popen(cmd)
peak, count, big, bigname, samples, series = 0, 0, 0, "", 0, []
while proc.poll() is None:
    rows = table()
    members = tree(rows, proc.pid)
    total = sum(rows[p][1] for p in members)
    samples += 1
    if total > peak:
        peak = total
    count = max(count, len(members))
    for p in members:
        if rows[p][1] > big:
            big, bigname = rows[p][1], rows[p][2]
    if samples % 10 == 1:
        top = sorted(members, key=lambda p: -rows[p][1])[:3]
        series.append([round(time.time() - t0), round(total / 1024)] + [[round(rows[p][1] / 1024), rows[p][2][:120].replace(chr(10), " ")[-90:]] for p in top])
    time.sleep(0.5)
wall = time.time() - t0
ru1 = resource.getrusage(resource.RUSAGE_CHILDREN)
record = {"label": label, "rc": proc.returncode, "wallS": round(wall, 1), "cpuUserS": round(ru1.ru_utime - ru0.ru_utime, 1), "cpuSysS": round(ru1.ru_stime - ru0.ru_stime, 1), "peakTreeRssMB": round(peak / 1024), "peakProcs": count, "biggestProcMB": round(big / 1024), "biggestProc": bigname[-200:], "series": series, "samples": samples, "load0": [round(x, 1) for x in load0], "load1": [round(x, 1) for x in os.getloadavg()], "swap0": swap0, "swap1": swap(), "start": time.strftime("%H:%M:%S", time.localtime(t0)), "cmd": " ".join(cmd)[:300]}
with open(out, "a") as handle:
    handle.write(json.dumps(record) + "\n")
print(json.dumps(record))
sys.exit(proc.returncode or 0)
