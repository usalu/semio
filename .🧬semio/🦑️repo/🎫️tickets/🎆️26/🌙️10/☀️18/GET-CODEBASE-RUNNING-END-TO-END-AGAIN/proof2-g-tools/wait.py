"""Samples the RSS of a pid's process tree every 5 s until it ends: wait.py <pid> <out.json> [max-seconds]; prints wall and peak MB."""
import json, subprocess, sys, time

def tree(root):
    rows = subprocess.run(["ps", "-axo", "pid=,ppid=,rss="], capture_output=True, text=True).stdout.split("\n")
    kids, rss = {}, {}
    for line in rows:
        parts = line.split()
        if len(parts) == 3:
            pid, ppid, kb = map(int, parts)
            kids.setdefault(ppid, []).append(pid)
            rss[pid] = kb
    seen, stack = [], [root]
    while stack:
        pid = stack.pop()
        if pid in rss:
            seen.append(pid)
        stack.extend(kids.get(pid, []))
    return len(seen), sum(rss[p] for p in seen) / 1024

def main() -> None:
    pid, out = int(sys.argv[1]), sys.argv[2]
    limit = float(sys.argv[3]) if len(sys.argv) > 3 else 520
    start, peak, procs, samples = time.time(), 0.0, 0, []
    while time.time() - start < limit:
        count, mb = tree(pid)
        if count == 0:
            break
        peak, procs = max(peak, mb), max(procs, count)
        samples.append([round(time.time() - start), count, round(mb)])
        time.sleep(5)
    done = tree(pid)[0] == 0
    json.dump({"pid": pid, "done": done, "wall": round(time.time() - start), "peakMB": round(peak), "maxProcs": procs, "samples": samples[-200:]}, open(out, "w"))
    print(json.dumps({"done": done, "wall": round(time.time() - start), "peakMB": round(peak), "maxProcs": procs}))

main()
