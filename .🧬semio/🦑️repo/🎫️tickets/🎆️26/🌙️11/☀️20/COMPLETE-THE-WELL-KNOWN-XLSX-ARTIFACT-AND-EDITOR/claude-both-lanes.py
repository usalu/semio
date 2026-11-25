"""[DEBUG] Sequential driver for both browser lanes: waits for the Space cargo, rebuilds stdio and stdio-office wasm (retrying others' transient breakage), then publishes, activates, serves and runs the pane journey (6272) and the full S journeys (6070)."""

import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = Path(__file__).resolve().parent
G = TICKET / "🗑️generated"
BUN = ROOT / ".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
TARGET = G / "claude-pane/native/target/wasm32-wasip2/wasm-prototype"
STATUS = G / "claude-pane/lanes.status"
os.environ.setdefault("SEMIO_PLUGIN_CATALOG", str(ROOT / "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"))


def say(text: str) -> None:
    print(f"[DEBUG] {text}", flush=True)
    with STATUS.open("a") as handle:
        handle.write(f"{time.strftime('%H:%M:%S')} {text}\n")


def alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def run(args, cwd, log) -> int:
    Path(log).parent.mkdir(parents=True, exist_ok=True)
    with open(log, "ab") as handle:
        handle.write(f"\n[DEBUG] {' '.join(map(str, args))}\n".encode())
        handle.flush()
        return subprocess.run(list(map(str, args)), cwd=cwd, stdout=handle, stderr=subprocess.STDOUT).returncode


def wasm(name: str) -> Path:
    matches = [p for p in TARGET.rglob(name) if "deps" not in p.parts and p.is_file()]
    if len(matches) != 1:
        raise SystemExit(f"{name} not uniquely present: {matches}")
    return matches[0]


def cargo(script: str, tag: str, needed: list[str]) -> None:
    for attempt in range(1, 9):
        code = run(["bash", TICKET / script], TICKET, G / f"claude-pane/{tag}-{attempt}.log")
        say(f"{tag} cargo attempt {attempt} exit {code}")
        if code == 0:
            return
        time.sleep(120)
    raise SystemExit(f"{tag} cargo never succeeded")


def serve(variant: str, port: int, log: Path) -> None:
    subprocess.Popen(["bash", TICKET / "claude-fulls-serve.sh", variant, str(port)], cwd=ROOT, stdout=log.open("ab"), stderr=subprocess.STDOUT, start_new_session=True)
    deadline = time.time() + 900
    while time.time() < deadline:
        probe = subprocess.run(["lsof", "-nP", f"-iTCP:{port}", "-sTCP:LISTEN"], capture_output=True, text=True)
        if "LISTEN" in probe.stdout:
            say(f"{port} listening")
            return
        time.sleep(2)
    raise SystemExit(f"{port} did not listen")


def main() -> None:
    STATUS.write_text("")
    say("space cargo process gone")
    if os.environ.get("SKIP_OFFICE_CARGO") != "1":
        cargo("claude-pane-cargo.sh", "office", ["semio_hub_stdio_component.wasm", "semio_hub_stdio_office_component.wasm"])
    stdio, office = wasm("semio_hub_stdio_component.wasm"), wasm("semio_hub_stdio_office_component.wasm")
    code = run([BUN, TICKET / "claude-fulls-session.ts", "stdio-xlsx", "stdio,stdio-office"], TICKET, G / "claude-pane/session.log")
    say(f"pane session exit {code}")
    if os.environ.get("SKIP_PANE_PUBLISH") != "1":
        code = run([BUN, TICKET / "claude-pane-publish.ts", "stdio", stdio, "stdio-office", office], TICKET, G / "claude-pane/publish.log")
        say(f"pane publish exit {code}")
        if code:
            raise SystemExit(code)
    code = run([BUN, TICKET / "claude-pane-support.ts"], ROOT, G / "claude-pane/support.log")
    say(f"pane support exit {code}")
    if code:
        raise SystemExit(code)
    code = run([BUN, TICKET / "claude-pane-activate.ts"], TICKET, G / "claude-pane/activate.log")
    say(f"pane activate exit {code}")
    if code:
        raise SystemExit(code)
    serve("stdio-xlsx", 6272, G / "claude-pane/vite-6272.log")
    code = run([BUN, TICKET / "standalone-pane-journey.ts"], ROOT, G / "standalone-journey/run.log")
    say(f"PANE JOURNEY exit {code}")
    if not [p for p in TARGET.rglob("semio_hub_space_component.wasm") if "deps" not in p.parts]:
        cargo("claude-fulls-cargo.sh", "space", ["semio_hub_space_component.wasm"])
    space = wasm("semio_hub_space_component.wasm")
    work = G / "claude-full-s/work"
    shutil.rmtree(work, ignore_errors=True)
    code = run([BUN, TICKET / "claude-fulls-session.ts"], TICKET, G / "claude-full-s/session.log")
    say(f"fulls session exit {code}")
    code = run([BUN, TICKET / "claude-fulls-publish.ts", f"space={space}", f"stdio={stdio}", f"stdio-office={office}"], TICKET, G / "claude-full-s/publish.log")
    say(f"fulls publish exit {code}")
    if code:
        raise SystemExit(code)
    code = run([BUN, TICKET / "claude-fulls-activate.ts", "s", "space", "stdio", "stdio-office"], TICKET, G / "claude-full-s/activate.log")
    say(f"fulls activate exit {code}")
    if code:
        raise SystemExit(code)
    serve("s", 6070, G / "claude-full-s/vite-6070.log")
    for locale in ("en", "de"):
        code = run([BUN, TICKET / "claude-fulls-journey.ts", locale, "http://127.0.0.1:6070/"], ROOT, G / f"claude-full-s/journey-{locale}.log")
        say(f"FULLS {locale} exit {code}")
    say("DONE")


main()
