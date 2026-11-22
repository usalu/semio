"""🪶️ Uses the current recorder build's exact mutation dependency fingerprints."""

import argparse
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

arguments = argparse.ArgumentParser(description=__doc__)
arguments.add_argument("--execute", action="store_true")
options = arguments.parse_args()
ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
build = generated / "cargo-puzzle-clipboard-build/debug/build"
mutation = "semio_s_artifact_puzzle_2d_mutations"
codec = root / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs"
extension = build / mutation.replace("_", "-")
anchors = [
    path for path in extension.glob(f"*/out/lib{mutation}-*.rmeta")
    if path.with_suffix(".rlib").is_file() and path.stat().st_mtime_ns >= codec.stat().st_mtime_ns
]
if not anchors:
    raise SystemExit("[DEBUG] Awaiting 2D mutation code generation from the current recorder build.")
anchor = max(anchors, key=lambda path: path.stat().st_mtime_ns)
metadata = subprocess.check_output(["rustc", "-Z", "ls=all", anchor], cwd=root, text=True)
known = {
    f"lib{name}-{fingerprint}"
    for name, fingerprint in re.findall(r"^\d+ (\w+)-([0-9a-f]{16}) hash ", metadata, re.MULTILINE)
}
known.add(anchor.stem)
outputs = list(build.glob("*/*/out"))
artifacts = {
    path.name: path
    for output in outputs
    for path in output.iterdir()
    if path.stem in known and path.suffix in {".rlib", ".rmeta", ".so", ".dylib", ".dll"}
}
external = {}
for name in (mutation, "semio_os_kernel", "semio_framework_dsl_record", "semio_framework_value", "serde_json"):
    candidates = [
        path for filename, path in artifacts.items()
        if re.fullmatch(rf"lib{name}-[0-9a-f]{{16}}\.rlib", filename)
        and path.with_suffix(".rmeta").is_file()
    ]
    if len(candidates) != 1:
        raise SystemExit(f"[DEBUG] Expected one warm {name} rlib/rmeta pair, found {len(candidates)}.")
    external[name] = candidates[0]
    print(f"[DEBUG] {name} uses {candidates[0].name}")

if not options.execute:
    raise SystemExit(0)
if shutil.disk_usage(generated).free < 8 * 1024**3:
    raise SystemExit("[DEBUG] Warm wire law needs 8 GiB workspace reserve.")
workspace = generated / "2d-history-wire-warm"
dependencies = workspace / "deps"
dependencies.mkdir(parents=True, exist_ok=True)
for filename, artifact in artifacts.items():
    link = dependencies / filename
    if not link.exists():
        try:
            link.symlink_to(artifact)
        except OSError:
            os.link(artifact, link)

executable = workspace / ("wire-format.exe" if sys.platform == "win32" else "wire-format")
command = [
    "rustc", "--edition=2021", "--test", ticket / "verify-2d-history-wire-warm.rs",
    "-o", executable, "-Cdebuginfo=0", "-Zthreads=1", "-L", f"dependency={dependencies}",
]
if sys.platform.startswith("linux"):
    command.extend(("-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options"))
for name, library in external.items():
    for suffix in (".rlib", ".rmeta"):
        artifact = dependencies / library.with_suffix(suffix).name
        command.extend(("--extern", f"{name}={artifact}"))
for output in outputs:
    if any(path.suffix in {".a", ".lib"} for path in output.iterdir()):
        command.extend(("-L", f"native={output}"))

environment = {
    **os.environ,
    "RUST_MIN_STACK": "134217728",
    "ZIG_GLOBAL_CACHE_DIR": str(generated / "zig-clipboard" / "global"),
    "ZIG_LOCAL_CACHE_DIR": str(generated / "zig-clipboard" / "local"),
}
subprocess.run(command, cwd=root, env=environment, check=True)
subprocess.run([executable, "wire_format_guard::", "--test-threads=1"], cwd=root, env=environment, check=True)
