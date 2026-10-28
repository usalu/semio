"""[gram2-g] read-only process tree: prints "<tree MB> <descendant count> <max descendant MB>" for the pid in argv[1] (descendants by ppid only, no name matching)."""
import subprocess, sys
root = int(sys.argv[1])
rows = {}
for line in subprocess.run(["ps", "-Ao", "pid=,ppid=,rss="], capture_output=True, text=True).stdout.splitlines():
    pid, ppid, rss = map(int, line.split())
    rows[pid] = (ppid, rss)
kids = {}
for pid, (ppid, _) in rows.items():
    kids.setdefault(ppid, []).append(pid)
seen, stack = set(), [root]
while stack:
    pid = stack.pop()
    if pid in seen or pid not in rows:
        continue
    seen.add(pid)
    stack.extend(kids.get(pid, []))
total = sum(rows[p][1] for p in seen) // 1024
big = max((rows[p][1] for p in seen if p != root), default=0) // 1024
print(total, len(seen) - 1, big)
