"""🪟 Run the authored-window checks when Cargo emits each selected editor."""
import json
from pathlib import Path
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
out = ticket / "🗑️generated"
run = workspace / ".cache/b/r/m/components/runs" / sys.argv[1]
pending = set(sys.argv[2:])
rustc = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
positions = {}
failed = []
while pending:
    for path in (run / "l").glob("cargo*.jsonl"):
        with path.open("rb") as stream:
            stream.seek(positions.get(path, 0))
            for line in stream:
                if not line.endswith(b"\n"):
                    break
                positions[path] = positions.get(path, 0) + len(line)
                msg = json.loads(line)
                if msg.get("reason") != "compiler-artifact":
                    continue
                for family in list(pending):
                    if msg["target"]["name"] != f"semio_s_artifact_norm_{family}_editor":
                        continue
                    library = next((Path(p) for p in msg["filenames"] if p.endswith(".rlib")), None)
                    if not library:
                        continue
                    lane = library.relative_to(workspace / ".cache/b/r/m/u/l").parts[0]
                    pending.remove(family)
                    files = sorted(workspace.glob(f"✏️s/*plugins/*norm/*artifacts/*{family}/*standards/*1/*subsets/*any/*assets/*/*dsl.semio"))
                    with (out / f"authored-{family}.log").open("w") as log:
                        compile_result = subprocess.run([sys.executable, ticket / "link-eurocode-windows.py", family, "--lane", lane, "--rustc", rustc], stdout=log, stderr=log)
                        if compile_result.returncode or not files:
                            failed.append(family)
                            continue
                        result = subprocess.run([workspace / "node_modules/.bin/jco", "run", "--jco-dir",out / f"authored-{family}-jco", out / f"windows-{family}.wasm", *files], stdout=log, stderr=log)
                        if result.returncode:
                            failed.append(family)
    if pending and (run / "r/report.md").exists():
        raise RuntimeError(f"Build ended before {pending}")
    if pending:
        time.sleep(8)
if failed:
    raise SystemExit("Failed authored-window checks for " + ", ".join(failed))
