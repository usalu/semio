"""Marks the pane wasm retry successful once both hub components exist.

The in-memory retry loop still assigns zsh's read-only `status` and would
treat a finished link as a failure. This watcher only writes RETRY-OK after
cargo has exited and both wasm files are non-empty, then stops that loop so
it does not start another build.
"""

import os
import signal
import subprocess
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated" / "claude-pane"
TARGET = GENERATED / "native/target/wasm32-wasip2/wasm-prototype"
NAMES = ("semio_hub_stdio_component.wasm", "semio_hub_stdio_office_component.wasm")


def ready() -> bool:
    return all((TARGET / name).is_file() and (TARGET / name).stat().st_size > 0 for name in NAMES)


def cargo_alive() -> bool:
    probe = subprocess.run(["ps", "-ax", "-o", "command="], capture_output=True, text=True)
    return "claude-pane-cargo.sh" in probe.stdout or "semio-hub-stdio-component" in probe.stdout


def retry_pids() -> list[int]:
    probe = subprocess.run(["ps", "-ax", "-o", "pid=,command="], capture_output=True, text=True)
    found = []
    for line in probe.stdout.splitlines():
        if "claude-pane-cargo-retry.sh" not in line:
            continue
        found.append(int(line.split(None, 1)[0]))
    return found


def main() -> None:
    status = GENERATED / "retry.status"
    for _ in range(240):
        if status.exists() and "RETRY-OK" in status.read_text(errors="replace"):
            return
        if ready() and not cargo_alive():
            status.write_text("RETRY-OK rescued\n")
            for pid in retry_pids():
                os.kill(pid, signal.SIGTERM)
            print("[DEBUG] pane wasm rescued", flush=True)
            return
        time.sleep(15)
    raise SystemExit("pane wasm was not linked")


if __name__ == "__main__":
    main()
