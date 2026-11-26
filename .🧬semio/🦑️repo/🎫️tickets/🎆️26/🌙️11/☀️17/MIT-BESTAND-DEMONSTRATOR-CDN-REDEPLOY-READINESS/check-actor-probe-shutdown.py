import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
site = Path(sys.argv[1]).resolve()
env = dict(os.environ, PLAYWRIGHT_BROWSERS_PATH=str(ticket / "🗑️generated" / "absent-browser"))
try:
    run = subprocess.run(["node", str(ticket / "probe-release-actors.cjs"), str(site), "--retained"],
                         env=env, capture_output=True, timeout=6)
except subprocess.TimeoutExpired:
    raise SystemExit("Diagnostic server kept the failed browser process alive")
assert run.returncode and b"Executable doesn't exist" in run.stderr, run.stderr
print("Missing Chromium closes the diagnostic server and returns its original launch failure.")
