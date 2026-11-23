"""Starts the base mutation parity run after the suite, assembly, and claims processes exit."""

import os
import subprocess
import time
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = Path(__file__).resolve().parent
LOG = TICKET / "🗑️generated/claude-oracles2/feature-parity.log"
REPO = Path("/Users/ueli/.local/bin/repo")
WAIT = (70955, 67639, 8826)
OWNER = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"
CASE = "🔀️mutate-xlsx-ecma-376"


def alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def parity_running() -> bool:
    listed = subprocess.check_output(["ps", "-ax", "-o", "command="], text=True, errors="replace")
    return "test-parity" in listed and "mutate-xlsx-ecma-376" in listed


def main() -> None:
    if parity_running():
        print("[DEBUG] exhaustive parity is already running; not starting another", flush=True)
        return
    print("[DEBUG] waiting for suite cargos", WAIT, flush=True)
    while any(alive(pid) for pid in WAIT):
        time.sleep(20)
    LOG.parent.mkdir(parents=True, exist_ok=True)
    args = [str(REPO), "run", "@semio-tech/repo-test-domain:test-parity", "--", "exhaustive", "--owner-root", OWNER, "--case", CASE, "--skip-cache"]
    env = os.environ.copy()
    env["SEMIO_LANGUAGE"] = "en"
    print("[DEBUG] starting parity", flush=True)
    with LOG.open("ab") as handle:
        handle.write(("[DEBUG] " + " ".join(args) + "\n").encode())
        handle.flush()
        code = subprocess.run(args, cwd=ROOT, env=env, stdout=handle, stderr=subprocess.STDOUT).returncode
    print(f"[DEBUG] PARITY exit {code}", flush=True)
    raise SystemExit(code)


if __name__ == "__main__":
    main()
