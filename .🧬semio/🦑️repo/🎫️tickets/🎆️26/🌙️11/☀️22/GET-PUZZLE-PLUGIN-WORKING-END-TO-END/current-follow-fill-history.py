from pathlib import Path
import os
import re
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
build_log = generated / sys.argv[1]
selection = sys.argv[2:]
project = next(root.glob("✏️s/🔌*/🧩*/🗿*/◻️2d/🏅*/🔖*/🪆*/✳*/✏*/🧪*/🧪*history-edit-runtime/📦*packages/🦀*/Cargo.toml")).parent
while True:
    with build_log.open("rb") as log:
        log.seek(max(0, build_log.stat().st_size - 350000))
        tail = log.read().decode(errors="replace")
    for line in tail.splitlines():
        match = re.search(r"Running .*\(([^()]*(?:semio_s_artifact_puzzle_2d_history_edit_runtime_tests|history_edit_runtime)[^()]*)\)\s*$", line)
        if match:
            binary = Path(match[1])
            if not binary.is_absolute():
                binary = (root / binary) if (root / binary).is_file() else (project / binary)
            if binary.is_file():
                args = [str(binary), "--nocapture", "--test-threads=1", *selection]
                result = subprocess.run(args, env={**os.environ, "RUST_MIN_STACK":"134217728"}, cwd=project)
                raise SystemExit(result.returncode)
    if "error: could not compile" in tail or "error: failed to" in tail:
        raise SystemExit("The shared package build stopped before its mounted history executable was ready.")
    time.sleep(15)
