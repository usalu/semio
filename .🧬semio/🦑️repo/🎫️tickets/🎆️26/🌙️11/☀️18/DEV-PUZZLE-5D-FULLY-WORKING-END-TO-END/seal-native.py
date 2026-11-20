import json
import os
from pathlib import Path
import re
import subprocess
import sys

workspace = Path.cwd()
build = Path(sys.argv[1])
output = Path(sys.argv[2])
output.mkdir(parents=True, exist_ok=True)
rustc, = workspace.glob(".cache/b/t/m/r/*/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc")
artifacts = {}
dependencies = set()
for path in build.glob("*/*/out/lib*"):
    if path.suffix in (".rmeta", ".rlib", ".so"):
        identifier = path.stem.removeprefix("lib")
        dependencies.add(path.parent)
        if path.suffix == ".rlib" or identifier not in artifacts:
            artifacts[identifier] = path
protocol = max((path for name, path in artifacts.items() if name.startswith("protocol-")), key=lambda path: path.stat().st_mtime)
metadata = subprocess.check_output([rustc, "-Z", "ls=root", protocol.with_suffix(".rmeta")], text=True)
selected = {
    identifier.split("-")[0]: artifacts[identifier]
    for identifier in re.findall(r"^\d+ ([\w]+-[\da-f]+) hash", metadata, re.M)
    if identifier in artifacts
}
selected["protocol"] = protocol
fallback = {}
for name, path in artifacts.items():
    key = name.split("-")[0]
    if key in selected or name.startswith("semio_s_"):
        continue
    try:
        stamp = path.stat().st_mtime_ns
    except FileNotFoundError:
        continue
    if key not in fallback or stamp > fallback[key][0]:
        fallback[key] = (stamp, path)
selected.update({key: value[1] for key, value in fallback.items()})

def compile_source(name, source, imports, destination, kind, data=None):
    args = [rustc, "--edition=2021", "--crate-name", name, str(source), kind, "-Zthreads=1", "-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options", "-o", str(destination)]
    for directory in sorted(dependencies):
        args.extend(["-L", f"dependency={directory}"])
    args.extend(["-L", f"dependency={output}"])
    for name in imports:
        path = selected[name]
        metadata = path.with_suffix(".rmeta")
        args.extend(["--extern", f"{name}={metadata if metadata.exists() else path}"])
    result = subprocess.run(args, input=data, text=True, env={**os.environ, "RUSTC_ICE": str(output)})
    if result.returncode:
        raise SystemExit(result.returncode)

value = workspace / "🧰️framework/🔨️modules/🌱️value"
replication = workspace / "🧰️framework/🔨️modules/📡️replication"
fixture_sources = [
    ("semio_framework_value_testing_effect_bank", "🧪️testing/📥️admission/🎟️effect/🦀️.rs", ["semio_framework_value_control_contract", "serde_json"]),
    ("semio_framework_value_testing_allocator", "🧪️testing/💾️allocator/🦀️.rs", ["semio_framework_trace", "semio_framework_trace_memory_testing"]),
    ("semio_framework_value_clone_source_testing", "🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs", ["semio_framework_value_testing_allocator", "semio_framework_value_testing_effect_bank", "semio_framework_value_clone_source", "semio_framework_value_control_contract", "semio_framework_value_retirement", "semio_framework_trace"]),
]
used = ["semio_framework_replication_edit", "semio_framework_hash", "semio_framework_pack_json", "semio_framework_value", "semio_framework_value_testing_effect_bank", "semio_framework_value_clone_source_testing", "serde_json"]
for name, relative, imports in fixture_sources:
    destination = output / f"lib{name}.rlib"
    compile_source(name, value / relative, imports, destination, "--crate-type=rlib")
    selected[name] = destination
edit_imports = [
    "semio_framework_hash", "semio_framework_pack_json", "semio_framework_replication_crypto",
    "semio_framework_replication_ids", "semio_framework_replication_mutation",
    "semio_framework_replication_wire_core", "semio_framework_value",
    "semio_framework_value_native_owned", "semio_framework_value_persistent",
]
edit_library = output / "libsemio_framework_replication_edit.rlib"
compile_source("semio_framework_replication_edit", replication / "🎮️mutation/✏️edit/🦀️.rs", edit_imports, edit_library, "--crate-type=rlib")
selected["semio_framework_replication_edit"] = edit_library
test = replication / "🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests/📄️paged-digest/🦀️.rs"
data = test.read_text().replace("use protocol::{io::text::canonical::", "use semio_framework_replication_edit::{canonical::")
data = re.sub(r'include_str!\("([^"]+)"\)', lambda match: 'include_str!(' + json.dumps(str((test.parent / match[1]).resolve()), ensure_ascii=False) + ')', data)
binary = output / "paged-seal"
print("[DEBUG]", json.dumps({name: str(selected[name]) for name in used}, ensure_ascii=False), flush=True)
compile_source("paged_seal_digest", "-", used, binary, "--test", data)
raise SystemExit(subprocess.run([binary, "--nocapture"], env={**os.environ, "RUST_MIN_STACK": "67108864"}).returncode)
