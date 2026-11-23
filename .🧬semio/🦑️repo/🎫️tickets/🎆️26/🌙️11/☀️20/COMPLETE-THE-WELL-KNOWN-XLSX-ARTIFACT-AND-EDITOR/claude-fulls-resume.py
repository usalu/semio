"""Waits for the pane resume and the Space wasm cargo, then publishes, activates, serves 6070 and runs both journeys."""

import os
import shutil
import subprocess
import time
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
BUN = ROOT / ".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
TARGET = GENERATED / "claude-pane/native/target/wasm32-wasip2/wasm-prototype"
SPACE_LOG = GENERATED / "claude-full-s/space-cargo3.log"


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


def pane_resume_alive() -> bool:
    probe = subprocess.run(["ps", "-ax", "-o", "command="], capture_output=True, text=True)
    return "claude-pane-resume.py" in probe.stdout


def main() -> None:
    print("[DEBUG] waiting for pane resume to release the wasm target", flush=True)
    while pane_resume_alive():
        time.sleep(20)
    print("[DEBUG] rebuilding space wasm with the home config receiving fix", flush=True)
    code = run(["bash", str(TICKET / "claude-fulls-cargo.sh")], TICKET, SPACE_LOG)
    print(f"[DEBUG] space cargo exit {code}", flush=True)
    if code != 0:
        raise SystemExit(f"space cargo exited {code}")
    work = GENERATED / "claude-full-s/work"
    if work.exists():
        shutil.rmtree(work)
    pairs = [
        f"space={wasm('semio_hub_space_component.wasm')}",
        f"stdio={wasm('semio_hub_stdio_component.wasm')}",
        f"stdio-office={wasm('semio_hub_stdio_office_component.wasm')}",
    ]
    code = run([str(BUN), str(TICKET / "claude-fulls-publish.ts"), *pairs], TICKET, GENERATED / "claude-full-s/publish.log")
    if code != 0:
        raise SystemExit(f"fulls publish exited {code}")
    code = run([str(BUN), str(TICKET / "claude-fulls-activate.ts"), "s", "space", "stdio", "stdio-office"], TICKET, GENERATED / "claude-full-s/activate.log")
    if code != 0:
        raise SystemExit(f"fulls activate exited {code}")
    serve_log = GENERATED / "claude-full-s/vite-6070.log"
    serve_log.parent.mkdir(parents=True, exist_ok=True)
    handle = serve_log.open("ab")
    subprocess.Popen(["bash", str(TICKET / "claude-fulls-serve.sh"), "s", "6070"], cwd=ROOT, stdout=handle, stderr=subprocess.STDOUT, start_new_session=True)
    deadline = time.time() + 900
    while time.time() < deadline:
        probe = subprocess.run(["lsof", "-nP", "-iTCP:6070", "-sTCP:LISTEN"], capture_output=True, text=True)
        if probe.returncode == 0 and "LISTEN" in probe.stdout:
            break
        time.sleep(2)
    else:
        raise SystemExit("6070 did not listen")
    print("[DEBUG] 6070 listening", flush=True)
    for locale in ("en", "de"):
        code = run([str(BUN), str(TICKET / "claude-fulls-journey.ts"), locale, "http://127.0.0.1:6070/"], ROOT, GENERATED / f"claude-full-s/journey-{locale}.log")
        print(f"[DEBUG] FULLS {locale} exit {code}", flush=True)
        if code != 0:
            raise SystemExit(code)


if __name__ == "__main__":
    main()
