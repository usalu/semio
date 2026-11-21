"""🕰️ Run the Shooting browser probes as soon as their dev host starts."""

import socket
import subprocess
import sys
import time
from pathlib import Path


LANES = {
    "react": (6019, ("browser-export-probe.mjs", "browser-cancel-probe.mjs")),
    "wgpu": (6119, ("browser-wgpu-probe.mjs",)),
}


def main(lane):
    port, probes = LANES[lane]
    ticket = Path(__file__).resolve().parent
    print(f"[DEBUG] Waiting for the Shooting {lane} dev host on port {port}", flush=True)
    while True:
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=1):
                break
        except OSError:
            time.sleep(2)

    failed = False
    for probe in probes:
        print(f"[DEBUG] Starting {probe}", flush=True)
        status = subprocess.run(
            ["repo", "tools", "exec", "bun", "--", str(ticket / probe)]
        ).returncode
        print(f"[DEBUG] {probe} exited {status}", flush=True)
        failed |= status != 0
    return int(failed)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
