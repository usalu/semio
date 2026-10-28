"""vmmap-probe.py <out.txt> -- <cmd...> : runs cmd, and when a descendant passes 2.2 GB RSS writes `vmmap -summary` of the biggest one to out.txt (once)."""
import subprocess, sys, time, os
out = sys.argv[1]; cmd = sys.argv[sys.argv.index("--") + 1:]
proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
done = False
while proc.poll() is None:
    rows = {}
    for line in subprocess.run(["ps", "-axo", "pid=,ppid=,rss="], capture_output=True, text=True).stdout.splitlines():
        a = line.split()
        if len(a) == 3: rows[int(a[0])] = (int(a[1]), int(a[2]))
    mine, stack = set(), [proc.pid]
    while stack:
        p = stack.pop()
        if p in mine: continue
        mine.add(p); stack += [q for q, (pp, _) in rows.items() if pp == p]
    big = max(mine, key=lambda p: rows.get(p, (0, 0))[1])
    if not done and rows.get(big, (0, 0))[1] > 2200000:
        done = True
        text = subprocess.run(["vmmap", "-summary", str(big)], capture_output=True, text=True).stdout
        open(out, "w").write("pid %d rss %d KB\n" % (big, rows[big][1]) + text)
    time.sleep(1)
print("rc", proc.returncode, "captured", done)
