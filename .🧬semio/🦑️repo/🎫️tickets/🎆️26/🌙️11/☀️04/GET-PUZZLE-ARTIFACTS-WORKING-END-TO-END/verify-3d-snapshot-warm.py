"""🧊️ Runs the focused snapshot law against the exact already-compiled dependency graph."""

import json
import os
from pathlib import Path
import re
import subprocess


repository = Path.cwd()
generated = Path(__file__).resolve().parent / "🗑️generated"
build = generated / "cargo-puzzle-clipboard-build" / "debug" / "build"
snapshot_name = "semio_s_artifact_puzzle_3d_editor_snapshot"
snapshot_metadata = max(
    (build / "semio-s-artifact-puzzle-3d-editor-snapshot").glob("*/out/lib*.rmeta"),
    key=lambda path: path.stat().st_mtime_ns,
)
metadata = subprocess.check_output(["rustc", "-Z", "ls=all", snapshot_metadata], text=True, cwd=generated)
fingerprints = dict(re.findall(r"^\d+ ([a-z][a-z0-9_]+)-([0-9a-f]{16}) hash ", metadata, re.MULTILINE))
fingerprints[snapshot_name] = snapshot_metadata.stem.rsplit("-", 1)[1]
direct = {
    snapshot_name: snapshot_name,
    "protocol": "semio_os_kernel",
    "pack": "pack",
    "semio_framework_value": "semio_framework_value",
    "semio_framework_pack_json": "semio_framework_pack_json",
    "semio_s_artifact_puzzle_3d_model": "semio_s_artifact_puzzle_3d_model",
    "semio_s_artifact_puzzle_3d_diff": "semio_s_artifact_puzzle_3d_diff",
    "serde_json": "serde_json",
}
workspace = generated / "3d-snapshot-warm"
workspace.mkdir(exist_ok=True)
dependencies = workspace / "deps"
dependencies.mkdir(exist_ok=True)
known_artifacts = {f"lib{name}-{fingerprint}" for name, fingerprint in fingerprints.items()}
native_dirs = set()
for output in build.glob("*/*/out"):
    for artifact in output.iterdir():
        if artifact.stem in known_artifacts and artifact.suffix in (".rmeta", ".rlib", ".so", ".dll", ".dylib"):
            link = dependencies / artifact.name
            if not link.exists():
                link.symlink_to(artifact)
        if artifact.suffix == ".a":
            native_dirs.add(output)

law_source = repository / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📸️snapshot/🧪️tests/🔬️play-pack/🦀️.rs"
wrapper = workspace / "snapshot_law.rs"
wrapper.write_text(
    f"pub use {snapshot_name}::*;\n"
    f"#[path = {json.dumps(str(law_source), ensure_ascii=False)}]\n"
    "mod play_pack_tests;\n",
    encoding="utf-8",
)
executable = workspace / "snapshot-law"
command = ["rustc", "--edition=2021", "--test", wrapper, "-o", executable,
           "-Cdebuginfo=0", "-Zthreads=1", "-Clinker-features=+lld", "-Clink-self-contained=+linker",
           "-Zunstable-options", "-L", f"dependency={dependencies}"]
for alias, name in direct.items():
    filename = f"lib{name}-{fingerprints[name]}.rmeta"
    artifact = dependencies / filename
    if not artifact.is_file():
        raise SystemExit(f"[DEBUG] Warm snapshot dependency absent: {filename}")
    command.extend(["--extern", f"{alias}={artifact}", "--extern", f"{alias}={artifact.with_suffix('.rlib')}"])
for native in sorted(native_dirs):
    command.extend(["-L", f"native={native}"])

environment = dict(os.environ, ZIG_GLOBAL_CACHE_DIR=str(generated / "zig-clipboard" / "global"),
                   ZIG_LOCAL_CACHE_DIR=str(generated / "zig-clipboard" / "local"), RUST_MIN_STACK="134217728")
subprocess.run(command, check=True, cwd=workspace, env=environment)
subprocess.run([executable, "play_pack_tests::play_snapshot_diff_keeps_shared_source_and_cache_until_success", "--exact"],
               check=True, cwd=repository, env=environment)
