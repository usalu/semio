import json
import os
from pathlib import Path
import subprocess
import sys

build, fingerprint, source, output = map(Path, sys.argv[1:])
output.parent.mkdir(parents=True, exist_ok=True)
rustc, = Path.cwd().glob(".cache/b/t/m/r/*/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc")
record = json.loads(fingerprint.read_text())
args = [str(rustc), "--crate-name", "semio_os_store_snapshot_clone", "--test", "--edition=2021", "-Zthreads=1", "-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options", str(source), "-o", str(output)]
for feature in json.loads(record["features"]):
    args.extend(["--cfg", f'feature="{feature}"'])
directories = sorted(build.glob("*/*/out"))
for directory in directories:
    args.extend(["-L", f"dependency={directory}"])
    if any(directory.glob("*.a")):
        args.extend(["-L", f"native={directory}"])
for _, alias, _, digest in record["deps"]:
    marker = digest.to_bytes(8, "little").hex()
    dependency, = (path for path in build.glob(f"*/*/fingerprint/lib-{alias}") if path.read_text() == marker)
    files = sorted((dependency.parent.parent / "out").glob(f"lib{alias}-*"))
    external = [path for path in files if path.suffix in (".rmeta", ".rlib", ".so")]
    assert external, dependency
    for path in external:
        args.extend(["--extern", f"{alias}={path}"])
environment = {**os.environ, "RUSTC_ICE": str(output.parent)}
subprocess.run(args, check=True, env=environment)
raise SystemExit(subprocess.run([output, "--test-threads=1"], env=environment).returncode)
