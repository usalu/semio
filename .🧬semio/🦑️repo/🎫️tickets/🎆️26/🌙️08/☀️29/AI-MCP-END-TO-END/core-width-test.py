import json
import os
from pathlib import Path
import subprocess
import sys
import re

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
build = generated / "native-host" / "debug" / "build"
wake = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake").resolve()
manifest = wake / "📦️packages" / "🦀️rust" / "Cargo.toml"
source = wake / "📃️policy" / "📥️operation" / "🪙️guest" / "🧪️tests" / "core-width.rs"
def setting(source, key):
    return re.search(r"^" + key + "\s*=\s*\"([^\"]+)\"", source, re.MULTILINE).group(1)

crate = manifest.read_text()
channel = setting(Path("rust-toolchain.toml").read_text(), "channel")

by_digest = {}
native = set()
for unit in build.glob("*/*"):
    files = unit / "out"
    if not files.is_dir():
        continue
    for artifact in files.iterdir():
        if artifact.suffix in (".a", ".lib"):
            native.add(files)
        if not artifact.name.startswith("lib") or artifact.suffix not in (".rlib", ".dylib", ".so", ".dll", ".rmeta"):
            continue
        name = artifact.stem[3:].rsplit("-", 1)[0]
        fingerprint = unit / "fingerprint" / ("lib-" + name)
        if fingerprint.is_file():
            digest = fingerprint.read_text().strip()
            if digest and (digest not in by_digest or artifact.suffix != ".rmeta"):
                by_digest[digest] = (artifact, fingerprint.with_suffix(".json"))

wake_candidates = sorted(
    (entry for entry in by_digest.values() if entry[0].name.startswith("libsemio_plugin_host_wake-") and entry[0].suffix == ".rlib"),
    key=lambda item: item[1].stat().st_mtime,
    reverse=True,
)
visited = set()
paths = set()
def dependencies(descriptor):
    externs = {}
    for _, alias, _, digest in json.loads(descriptor.read_text())["deps"]:
        if alias == "build_script_build":
            continue
        fingerprint = digest.to_bytes(8, "little").hex()
        try:
            artifact, dependency = by_digest[fingerprint]
        except KeyError:
            raise ValueError(f"{descriptor.name} lacks ready {alias} ({fingerprint})") from None
        externs[alias] = artifact
        paths.add(artifact.parent)
        if dependency not in visited:
            visited.add(dependency)
            dependencies(dependency)
    return externs

for wake_library, descriptor in wake_candidates:
    paths.clear()
    visited.clear()
    try:
        wake_deps = dependencies(descriptor)
        guest_descriptor = wake_deps["semio_plugin_guest_contract"].parent.parent / "fingerprint" / "lib-semio_plugin_guest_contract.json"
        guest_deps = dependencies(guest_descriptor)
        break
    except ValueError as failure:
        print(f"[DEBUG] Skipping incomplete wake fingerprint: {failure}")
else:
    sys.exit("No coherent production Wake dependency closure")
paths.add(wake_library.parent)
externs = {
    "semio_plugin_host_wake": wake_library,
    "semio_framework_actor": wake_deps["semio_framework_actor"],
    "semio_framework_job": wake_deps["semio_framework_job"],
    "semio_framework_value": wake_deps["semio_framework_value"],
    "serde_json": guest_deps["serde_json"],
}
output = generated / "core-width"
output.mkdir(exist_ok=True)
binary = output / ("test.exe" if sys.platform == "win32" else "test")
registry = json.loads(subprocess.check_output(["repo", "tools", "list", "--json"]))
rustup = next(row["command"] for row in registry if row["id"] == "rustup")
command = [
    rustup, "run", channel, "rustc", "--test", "--edition=" + setting(crate, "edition"),
    "--crate-name", "core_width_test", "-Zthreads=1", "-Cdebuginfo=0", "-Cmetadata=core_width_test",
    str(source), "-o", str(binary),
]
for alias, artifact in externs.items():
    metadata = artifact.with_suffix(".rmeta")
    if metadata != artifact and metadata.is_file():
        command.extend(("--extern", f"{alias}={metadata}"))
    command.extend(("--extern", f"{alias}={artifact}"))
for directory in sorted(paths):
    command.extend(("-L", f"dependency={directory}"))
for directory in sorted(native):
    command.extend(("-L", f"native={directory}"))
env = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest.parent), "CARGO_PKG_NAME": setting(crate, "name")}
print("[DEBUG] Linking the wake test with existing pinned native dependencies", flush=True)
completed = subprocess.run(command, env=env)
if completed.returncode:
    sys.exit(completed.returncode)
subprocess.run([str(binary), "fixed_core_guest_grants_fit_wasm32_with_larger_host_source_stock"], check=True)
