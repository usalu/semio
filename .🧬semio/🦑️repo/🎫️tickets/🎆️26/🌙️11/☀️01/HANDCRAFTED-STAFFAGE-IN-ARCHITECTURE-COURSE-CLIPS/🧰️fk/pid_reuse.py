import subprocess
import sys
import time

seen: dict[int, float] = {}
reused = 0
spawned = 0
started = time.monotonic()
while time.monotonic() - started < 20:
    child = subprocess.Popen([sys.executable, "-c", "pass"])
    spawned += 1
    now = time.monotonic()
    if child.pid in seen and now - seen[child.pid] < 10:
        reused += 1
        print(f"pid {child.pid} reused after {now - seen[child.pid]:.2f}s", flush=True)
    child.wait()
    seen[child.pid] = time.monotonic()
print(f"spawned={spawned} reused_within_10s={reused}")
