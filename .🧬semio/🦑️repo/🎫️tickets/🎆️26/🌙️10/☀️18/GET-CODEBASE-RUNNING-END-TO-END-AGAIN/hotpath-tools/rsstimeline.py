"""📈 RSS timeline of every process of one command: `rsstimeline.py <interval-s> -- cmd...`; prints one line per sample with the RSS (MB) of each live descendant, keyed by pid, plus each pid's command head once. Streams; keeps no samples."""
import subprocess, sys, time
interval = float(sys.argv[1]); cmd = sys.argv[3:]
t0 = time.time(); proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL); seen = set()
while proc.poll() is None:
    rows = {}
    for line in subprocess.run(["ps", "-axo", "pid=,ppid=,rss=,command="], capture_output=True, text=True, errors="replace").stdout.splitlines():
        p = line.split(None, 3)
        if len(p) == 4 and p[0].isdigit():
            rows[int(p[0])] = (int(p[1]), int(p[2]), p[3])
    mine, stack = [], [proc.pid]
    while stack:
        pid = stack.pop(); mine.append(pid); stack.extend(k for k, r in rows.items() if r[0] == pid)
    cells = []
    for pid in mine:
        if pid in rows:
            if pid not in seen:
                seen.add(pid); print(f"# {pid} {rows[pid][2][-90:]}")
            cells.append(f"{pid}:{rows[pid][1]//1024}")
    print(f"{time.time()-t0:6.2f} " + " ".join(cells), flush=True)
    time.sleep(interval)
