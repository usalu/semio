"""🚦️ Runs a command while holding the ticket's single test-build lane; the lock stays in this process so detached grandchildren never keep it."""
import fcntl, os, signal, subprocess, sys
from pathlib import Path
home = Path(__file__).resolve().parent / "🗑️generated"
home.mkdir(exist_ok=True)
lane = os.open(home / "lane.lock", os.O_RDWR | os.O_CREAT | os.O_CLOEXEC, 0o644)
print("[lane] waiting", file=sys.stderr, flush=True)
fcntl.flock(lane, fcntl.LOCK_EX)
print("[lane] acquired", file=sys.stderr, flush=True)
subprocess.run([str(home.parent / "source-gate.sh")], check=False)
print("[lane] gate open", file=sys.stderr, flush=True)
child = subprocess.Popen(sys.argv[1:])
for name in (signal.SIGTERM, signal.SIGINT):
    signal.signal(name, lambda number, frame: child.send_signal(number))
sys.exit(child.wait() & 0xFF)
