#!/usr/bin/env python3
"""🛡️ Runs a command and kills only that child when its RSS passes the limit: python3 -I guarded.py <limit MB> <timeout s> -- cmd..."""
import subprocess, sys, time
limit, timeout = int(sys.argv[1]), int(sys.argv[2])
cmd = sys.argv[sys.argv.index("--") + 1:]
p = subprocess.Popen(cmd)
start = time.time()
peak = 0
while p.poll() is None:
    time.sleep(0.5)
    r = subprocess.run(["ps", "-o", "rss=", "-p", str(p.pid)], capture_output=True, text=True).stdout.strip()
    mb = int(r) // 1024 if r else 0
    peak = max(peak, mb)
    if mb > limit or time.time() - start > timeout:
        p.terminate()
        time.sleep(1)
        if p.poll() is None: p.kill()
        print(f"[DEBUG] guard killed pid {p.pid}: rss {mb} MB, {time.time()-start:.0f}s", file=sys.stderr)
        sys.exit(124)
print(f"[DEBUG] exit {p.returncode} peak rss {peak} MB {time.time()-start:.0f}s", file=sys.stderr)
sys.exit(p.returncode)
