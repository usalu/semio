"""🧪️ Link the worker admission witness from already admitted WASI libraries."""

import json
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
root = Path(".cache/b/r/m/components/runs") / os.environ.get("SEMIO_WORKER_RECEIPT_RUN", "wGACoW7")
suffix = "-retry" if os.environ.get("SEMIO_WORKER_RECEIPT_RETRY") == "1" else ""
deps = output / ("worker-deps-" + root.name + suffix)
deps.mkdir(exist_ok=True)
libs = {}
for line in (root / "l" / ("cargo-release-00" + suffix + ".jsonl")).open():
    if '"compiler-artifact"' not in line:
        continue
    try:
        receipt = json.loads(line)
    except ValueError:
        continue
    if receipt.get("reason") != "compiler-artifact":
        continue
    for name in receipt["filenames"]:
        artifact = Path(name)
        if not artifact.is_file() or artifact.suffix not in (".rlib", ".rmeta", ".dylib", ".so", ".dll"):
            continue
        link = deps / artifact.name
        if link.is_symlink() and link.resolve() != artifact.resolve():
            link.unlink()
        if not link.exists():
            link.symlink_to(artifact)
        if artifact.suffix == ".rlib" and "wasm32-wasip2" in artifact.parts:
            libs[receipt["target"]["name"]] = link

names = [name for name in sys.argv[2:] if name != "--link"] or ["semio_framework_actor", "semio_framework_job", "semio_framework_value", "semio_plugin_reactor_context"]
request = Path(sys.argv[1])
source = request if request.suffix == ".rs" else ticket / (request.name + ".rs")
stem = source.stem
command = [
    "repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
    "--edition=2021", "--target=wasm32-wasip2", "-Cdebuginfo=0", "-Copt-level=1",
    "-Clink-arg=--threads=1", "-Clink-arg=-zstack-size=8388608", "-Clink-arg=--max-memory=536870912", "-L", f"dependency={deps}", "--crate-name", stem.replace("-", "_"),
]
linked = "--link" in sys.argv[2:]
for dependency in names:
    command.extend(["--extern", f"{dependency}={libs[dependency].with_suffix('.rmeta')}"])
    if linked:
        command.extend(["--extern", f"{dependency}={libs[dependency]}"])
step = ["-o", output / (stem + ".wasm")] if linked else ["--emit=metadata", "-o", output / (stem + ".rmeta")]
subprocess.run([*command, *step, source], check=True)
