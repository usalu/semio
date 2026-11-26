#!/usr/bin/env python3
"""🏗️ Build current Math sources against one fingerprint-coherent private WASI dependency closure."""
from collections import deque
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys

root = Path.cwd().resolve()
lane = Path(sys.argv[1]).resolve()
out = Path(sys.argv[2]).resolve()
toolchain = sys.argv[3]
sdk_pin = next((arg.split("=", 1)[1] for arg in sys.argv[4:] if arg.startswith("--sdk-unit=")), None)
math = root / "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math"
out.mkdir(parents=True, exist_ok=True)
links = out / "deps"
links.mkdir(exist_ok=True)
watched = (
    math / "🦀️.rs",
    math / "⏱️retained/🦀️.rs",
    math / "🧩️component/🦀️.rs",
    math / "📦️packages/🦀️rust/Cargo.toml",
)

def live_source(path):
    for snapshot in (root / ".cache/g3d-native/b", root / ".cache/g3d-native/s", root / ".cache/g3d-native/p"):
        try:
            return root / path.relative_to(snapshot)
        except ValueError:
            pass
    return path

def sum_file(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()

def record_inputs():
    return {str(path.relative_to(root)): sum_file(path) for path in watched}

host = lane / "debug/build"
target = lane / "wasm32-wasip2/debug/build"
fingerprints = {}
records = {}
for directory in [host, target]:
    for unit in directory.glob("*/*"):
        for file in (unit / "fingerprint").glob("lib-*.json"):
            stem = file.stem.removeprefix("lib-")
            possible = list((unit / "out").glob(f"lib{stem}-*.rlib")) + list((unit / "out").glob(f"lib{stem}-*.dylib"))
            if len(possible) != 1:
                continue
            value = possible[0]
            fingerprint = file.with_suffix("").read_text().strip()
            fingerprints[fingerprint] = value
            records[value] = (unit.parent.name, json.loads(file.read_text()))
            link = links / value.name
            if not link.exists():
                link.symlink_to(value)
            meta = value.with_suffix(".rmeta")
            if meta.is_file():
                link = links / meta.name
                if not link.exists():
                    link.symlink_to(meta)

flow_unit = max(
    (file for file, (package, _) in records.items()
     if package == "semio-os-flow-extensions" and target in file.parents
     and (sdk_pin is None or file.parent.parent.name == sdk_pin)),
    key=lambda file: file.stat().st_mtime, default=None,
)
if flow_unit is None:
    sys.exit("the requested lane has no completed WASI Flow dependency")
queue = deque([flow_unit])
closure = {}
edges = {}
while queue:
    file = queue.popleft()
    if file in closure:
        continue
    package, record = records[file]
    closure[file] = package
    deps = {}
    for _, alias, _, raw in record["deps"]:
        fingerprint = raw.to_bytes(8, byteorder="little").hex()
        dependency = fingerprints.get(fingerprint)
        if dependency is not None:
            deps[alias] = dependency
            queue.append(dependency)
    edges[file] = deps

def one(package, first=None):
    if first is not None and package in edges[first]:
        return edges[first][package]
    candidates = [
        file for file, name in closure.items()
        if name == package and target in file.parents
    ]
    expected = {file.name: file for file in candidates}
    if len(expected) != 1:
        sys.exit(f"{package} dependency is ambiguous or absent: {list(expected)}")
    return next(iter(expected.values()))

def compile(crate, source, crate_type, output, dependencies, package, features=(), rebuilt=False):
    opts = [
        "rustup", "run", toolchain, "rustc",
        "--crate-name", crate, "--edition=2021", str(source),
        "--crate-type", crate_type, "--target", "wasm32-wasip2",
        "-Zthreads=1", "-Cdebuginfo=0", "-Cstrip=debuginfo",
        "-Copt-level=0", "-Clink-arg=--max-memory=536870912", "-Clink-arg=-zstack-size=8388608",
        "-L", f"dependency={links}",
    ]
    if crate_type == "rlib":
        opts.extend(["--emit=metadata,link", "--out-dir", str(output.parent)])
    else:
        opts.extend(["-o", str(output)])
    if rebuilt:
        opts.extend(["-Cmetadata=math-wasi-live", "-Cextra-filename=-math-wasi-live"])
    for feature in features:
        opts.extend(["--cfg", 'feature="' + feature + '"'])
    for alias, path in dependencies.items():
        meta = path.with_suffix(".rmeta")
        selected = (links / meta.name) if (links / meta.name).is_file() else (links / path.name)
        print(f"[DEBUG] {crate} --extern {alias}={selected.name}", flush=True)
        opts.extend(["--extern", f"{alias}={selected}"])
        if crate_type == "cdylib" and selected.suffix == ".rmeta":
            opts.extend(["--extern", f"{alias}={links / path.name}"])
    env = dict(os.environ, CARGO_PKG_VERSION="0.1.0", CARGO_MANIFEST_DIR=str(source.parent), CARGO_PKG_NAME=package)
    subprocess.run(opts, env=env, cwd=root, check=True)

def transitive_rebuild(changed_packages):
    rebuild = {}

    def visit(file):
        if file in rebuild:
            return rebuild[file]
        dependencies = {alias: visit(dependency) for alias, dependency in edges[file].items()}
        package, record = records[file]
        if package not in changed_packages and dependencies == edges[file]:
            rebuild[file] = file
            return file
        missing = {alias for _, alias, _, _ in record["deps"]} - dependencies.keys()
        if missing:
            sys.exit(f"cannot replay {package}; unresolved dependency aliases {sorted(missing)}")
        dep_info, = file.parent.glob("*.d")
        source_paths = [live_source(Path(path)) for path in shlex.split(dep_info.read_text().splitlines()[0].split(": ", 1)[1])]
        source = source_paths[0]
        if package == "semio-os-kernel-neural-engine-registry":
            source_paths.extend((source.resolve().parent / "⏱️schema-plan").glob("*.rs"))
        source_paths = {path.resolve() for path in source_paths if path.is_file()}
        for path in source_paths:
            watched_sources[path] = sum_file(path)
        crate = file.stem.removeprefix("lib").rsplit("-", 1)[0]
        output = links / f"lib{crate}-math-wasi-live.rlib"
        present = output.is_file() and output.with_suffix(".rmeta").is_file()
        inputs = source_paths | set(dependencies.values())
        stamp = output.with_suffix(".sdk-pin")
        compatible = stamp.is_file() and stamp.read_text() == flow_unit.name
        if present and compatible and all(path.stat().st_mtime_ns < output.stat().st_mtime_ns for path in inputs):
            print(f"[DEBUG] validated warm replay {package}", flush=True)
            rebuild[file] = output
            return output
        print(f"[DEBUG] replay {package} ({len(dependencies)} direct dependencies)", flush=True)
        compile(crate, source, "rlib", output, dependencies, package, json.loads(record["features"]), True)
        stamp.write_text(flow_unit.name)
        rebuild[file] = output
        return output

    visit(flow_unit)
    return rebuild

before = record_inputs()
watched_sources = {}
refreshed = "--refresh-schema" in sys.argv[4:]
changed = {"semio-os-kernel-neural-engine-registry", "semio-os-kernel-neural-engine-model"} if refreshed else set()
if "--refresh-runtime" in sys.argv[4:]:
    changed.update(("semio-os-kernel-neural-engine-registry", "semio-os-kernel-neural-engine", "semio-os-flow-extensions"))
replacements = transitive_rebuild(changed) if changed else {}
neural = one("neural_engine", flow_unit)
semio_value = one("semio_framework_value", flow_unit)
core_deps = {
    "flow_extension_sdk": flow_unit,
    "neural_engine": neural,
    "semio_framework_value": semio_value,
    "semio_framework_job": one("semio-framework-job"),
    "semio_framework": one("semio-framework"),
    "semio_framework_pack_json": one("semio-framework-pack-json"),
    "pack": one("semio-framework-pack"),
    "semio_plugin": one("semio-plugin"),
}
core_deps = {alias: replacements.get(path, path) for alias, path in core_deps.items()}
lib = links / "libsemio_s_plugin_flow_math.rlib"
component = out / "semio_s_plugin_flow_math_component.wasm"
compile("semio_s_plugin_flow_math", math / "📦️packages/🦀️rust/🦀️.rs", "rlib", lib, core_deps, "semio-s-plugin-flow-math")
compile("semio_s_plugin_flow_math_component", math / "🧩️component/🦀️.rs", "cdylib", component, {
    "semio_s_plugin_flow_math": lib,
    "semio_framework_pack": core_deps["pack"],
    "semio_plugin": core_deps["semio_plugin"],
}, "semio-s-plugin-flow-math-component")
if record_inputs() != before or any(sum_file(path) != digest for path, digest in watched_sources.items()):
    sys.exit("Math changed during compilation; this result is ineligible for publication")
receipt = {
    "inputs": before,
    "refreshedSources": {str(path): digest for path, digest in watched_sources.items()},
    "directDependencies": {name: str(path) for name, path in core_deps.items()},
    "component": {"sha256": sum_file(component), "bytes": component.stat().st_size},
    "warmSdkUnit": str(flow_unit),
}
(out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(f"[DEBUG] completed Math component {receipt['component']}", flush=True)
