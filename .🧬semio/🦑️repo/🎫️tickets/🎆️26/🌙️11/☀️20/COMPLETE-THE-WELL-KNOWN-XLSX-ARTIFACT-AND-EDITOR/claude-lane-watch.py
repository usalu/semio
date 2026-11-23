"""👀 Prints one line when an in-flight xlsx lane exits or its log gains a compile or test result."""

import os
import time
from pathlib import Path

BASE = Path(__file__).resolve().parent / "🗑️generated"
LOGS = {
    "rust-five-4": BASE / "claude-suites" / "rust-five-4.log",
    "assembly-3": BASE / "claude-suites" / "assembly-3.log",
    "claims-2": BASE / "claude-suites" / "claims-2.log",
    "cargo7": BASE / "claude-pane" / "cargo7.log",
    "space": BASE / "claude-full-s" / "space-cargo.log",
}
PIDS = {"xlsx-cargo": 70955, "assembly-cargo": 67639, "space-cargo": 62180, "pane-cargo": 62591}
seen = {name: path.stat().st_size if path.exists() else 0 for name, path in LOGS.items()}
dead: set[str] = set()

while True:
    for name, pid in PIDS.items():
        if name in dead:
            continue
        try:
            os.kill(pid, 0)
        except OSError:
            dead.add(name)
            print(f"LANE-EXIT {name} pid={pid}", flush=True)
    for name, path in LOGS.items():
        if not path.exists():
            continue
        size = path.stat().st_size
        if size <= seen[name]:
            continue
        with path.open("rb") as handle:
            handle.seek(seen[name])
            chunk = handle.read().decode("utf-8", "replace")
        seen[name] = size
        for line in chunk.splitlines():
            if "error[E" in line or "could not compile" in line or "test result:" in line or line.startswith("exit ") or "repository:command failed" in line:
                print(f"LANE-MARK {name}: {line[:240]}", flush=True)
    if len(dead) == len(PIDS):
        print("LANE-ALL-CARGO-EXIT", flush=True)
        break
    time.sleep(15)
