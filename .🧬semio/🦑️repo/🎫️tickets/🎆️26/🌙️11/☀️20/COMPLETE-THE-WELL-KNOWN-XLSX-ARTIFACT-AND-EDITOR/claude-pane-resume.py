"""Waits for the pane wasm cargo, then publishes, activates, serves 6272 and runs the journey."""

import os
import subprocess
import time
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"
GENERATED = TICKET / "🗑️generated"
BUN = ROOT / ".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
SPACE_PID = 96426
TARGET = GENERATED / "claude-pane/native/target/wasm32-wasip2/wasm-prototype"
LOG = GENERATED / "claude-pane/cargo8.log"


def alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def wasm(name: str) -> Path:
    direct = TARGET / name
    if direct.is_file() and direct.stat().st_size > 0:
        return direct
    matches = [path for path in TARGET.rglob(name) if "deps" not in path.parts and path.is_file()]
    if len(matches) != 1:
        raise SystemExit(f"{name} not uniquely present: {matches}")
    return matches[0]


def run(args: list[str], cwd: Path, log: Path) -> int:
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("ab") as handle:
        handle.write(f"\n[DEBUG] {' '.join(args)}\n".encode())
        handle.flush()
        return subprocess.run(args, cwd=cwd, stdout=handle, stderr=subprocess.STDOUT).returncode


def retry_alive() -> bool:
    probe = subprocess.run(["ps", "-ax", "-o", "command="], capture_output=True, text=True)
    return "claude-pane-cargo-retry.sh" in probe.stdout


def main() -> None:
    print("[DEBUG] waiting for the pane wasm retry", flush=True)
    while retry_alive():
        time.sleep(15)
    status_path = GENERATED / "claude-pane/retry.status"
    status = status_path.read_text(errors="replace") if status_path.exists() else ""
    print("[DEBUG] pane retry status", status.strip(), flush=True)
    if "RETRY-OK" not in status:
        raise SystemExit(f"pane wasm retry did not succeed: {status.strip()}")
    stdio = wasm("semio_hub_stdio_component.wasm")
    office = wasm("semio_hub_stdio_office_component.wasm")
    code = run([str(BUN), str(TICKET / "claude-pane-publish.ts"), "stdio", str(stdio), "stdio-office", str(office)], TICKET, GENERATED / "claude-pane/publish.log")
    if code != 0:
        raise SystemExit(f"publish exited {code}")
    code = run([str(BUN), str(TICKET / "claude-pane-activate.ts")], TICKET, GENERATED / "claude-pane/activate.log")
    if code != 0:
        raise SystemExit(f"activate exited {code}")
    serve_log = GENERATED / "claude-pane/vite-6272.log"
    serve_log.parent.mkdir(parents=True, exist_ok=True)
    handle = serve_log.open("ab")
    subprocess.Popen(["bash", str(TICKET / "claude-fulls-serve.sh"), "stdio-xlsx", "6272"], cwd=ROOT, stdout=handle, stderr=subprocess.STDOUT, start_new_session=True)
    deadline = time.time() + 900
    while time.time() < deadline:
        probe = subprocess.run(["lsof", "-nP", "-iTCP:6272", "-sTCP:LISTEN"], capture_output=True, text=True)
        if probe.returncode == 0 and "LISTEN" in probe.stdout:
            break
        time.sleep(2)
    else:
        raise SystemExit("6272 did not listen")
    print("[DEBUG] 6272 listening", flush=True)
    code = run([str(BUN), str(TICKET / "standalone-pane-journey.ts")], ROOT, GENERATED / "standalone-journey/run.log")
    print(f"[DEBUG] JOURNEY exit {code}", flush=True)
    raise SystemExit(code)


if __name__ == "__main__":
    main()
