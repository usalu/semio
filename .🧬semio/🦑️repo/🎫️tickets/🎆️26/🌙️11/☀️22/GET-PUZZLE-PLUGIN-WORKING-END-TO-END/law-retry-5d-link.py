import ast
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
output = ticket / "🗑️generated"
manifest = Path(sys.argv[1]).resolve()
workspace = next(root.glob("✏️s/🔌*/🧩*/🗿*/🖐️5d"))
record = next(line for line in (output / "law-history-5d-full-fresh-schema.log").read_text().splitlines()
              if line.startswith("subprocess.CalledProcessError: Command "))
args = ast.literal_eval(record.removeprefix("subprocess.CalledProcessError: Command ").split(" died with ", 1)[0].strip("'"))
args += ["-C", "link-arg=-Wl,--threads=2", "-C", "strip=symbols"]
env = {
    **os.environ,
    "CARGO_MANIFEST_DIR": str(ticket),
    "CARGO_PKG_NAME": "semio-s-artifact-puzzle-5d-v1-unit-tests",
    "CARGO_PKG_VERSION": "0.1.0",
    "OUT_DIR": str(output / "law-5d-option-schema"),
    "RUST_MIN_STACK": "134217728",
}
print("[DEBUG] Linking the exact dependency graph recorded for the complete history law", flush=True)
result = subprocess.run(args, cwd=workspace, env=env)
if result.returncode:
    raise SystemExit(result.returncode if result.returncode > 0 else 128 - result.returncode)
executable = args[args.index("-o") + 1]
raise SystemExit(subprocess.run([executable, "--exact", "history_edits_end_to_end", "--nocapture"],
                                cwd=manifest, env=env).returncode)
