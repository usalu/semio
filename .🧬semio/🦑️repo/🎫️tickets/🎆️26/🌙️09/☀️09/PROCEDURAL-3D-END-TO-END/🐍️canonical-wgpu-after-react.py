"""🪟 Reuses the shared browser component publication when starting the WGPU playground."""
from pathlib import Path
import csv
import subprocess
import sys
import time
import urllib.error
import urllib.request

ticket = Path(__file__).resolve().parent
react = int(sys.argv[1])
out = ticket / "🗑️generated" / "canonical-wgpu-after-planning-watchdog.log"
def react_is_running():
    if sys.platform == "win32":
        listing = subprocess.run(["tasklist", "/FI", f"PID eq {react}", "/FO", "CSV", "/NH"],
                                 capture_output=True, text=True, check=True)
        return any(len(row) >= 2 and row[1] == str(react) for row in csv.reader(listing.stdout.splitlines()))
    listing = subprocess.run(["ps", "-p", str(react), "-o", "command="], capture_output=True, text=True)
    return listing.returncode == 0 and "dev playground generation3d react prototype" in listing.stdout

with out.open("w") as log:
    print("[DEBUG] Waiting for the React boot component publication or the React launcher to exit.", file=log, flush=True)
    while True:
        try:
            with urllib.request.urlopen("http://127.0.0.1:6018/?plugin=generation3d", timeout=2) as response:
                if response.status == 200:
                    print("[DEBUG] The shared React playground route is ready.", file=log, flush=True)
                    break
        except (OSError, urllib.error.URLError):
            pass
        if not react_is_running():
            print("[DEBUG] The previous React launcher exited, starting WGPU with current inputs.", file=log, flush=True)
            break
        time.sleep(5)
    launch = ["repo", "./📜️script.ts", "dev", "playground", "generation3d", "wgpu", "prototype"]
    raise SystemExit(subprocess.run(launch, stdout=log, stderr=subprocess.STDOUT).returncode)
