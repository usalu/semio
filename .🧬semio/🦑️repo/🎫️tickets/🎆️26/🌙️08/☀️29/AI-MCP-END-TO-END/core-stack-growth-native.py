from pathlib import Path
import json
import os
import re
import subprocess
import sys

workspace = Path.cwd()
generated = Path(__file__).resolve().parent / "🗑️generated"
build = generated / "native-host/debug/build"
source = workspace / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter"
manifest = source / "📦️packages/🦀️rust"
root_crate = "semio_plugin_interpreter"
rows = {}
for fingerprint in build.glob("*/*/fingerprint/lib-*.json"):
    name = fingerprint.stem.removeprefix("lib-")
    output = fingerprint.parent.parent / "out"
    identity = fingerprint.parent.parent.name
    for extension in ("rlib", "dylib", "so", "dll"):
        library = output / f"lib{name}-{identity}.{extension}"
        if library.exists() and fingerprint.with_suffix("").exists():
            key = fingerprint.with_suffix("").read_text().strip()
            rows[key] = (name, library, fingerprint, json.loads(fingerprint.read_text()))
            break


def dependencies(row):
    for _, alias, _, number in row[3]["deps"]:
        if alias != "build_script_build":
            yield alias, rows[number.to_bytes(8, "little").hex()]


def candidates(name):
    return sorted((row for row in rows.values() if row[0] == name), key=lambda row: row[2].stat().st_mtime, reverse=True)


def collect(row, found):
    library = row[1]
    if library not in found:
        found.add(library)
        for _, dependency in dependencies(row):
            collect(dependency, found)


for actor in candidates("semio_framework_actor"):
    try:
        job = dict(dependencies(actor))["semio_framework_job"]
        if "semio_framework_job_testing_caller" not in dict(dependencies(job)):
            continue
        collect(actor, set())
    except KeyError:
        continue
    else:
        break
else:
    raise SystemExit("No compatible cached actor and job testkit")
root = candidates(root_crate)[0]
direct = dict(dependencies(root))
direct.update({"semio_framework_actor": actor, "semio_framework_job": job,
               "semio_framework_value": dict(dependencies(actor))["semio_framework_value"]})
trace = dict(dependencies(direct["semio_framework_job"]))["semio_framework_trace"]
direct[trace[0]] = trace
for name in ["wasmtime", "serde", "serde_json", "semio_framework_test_fixtures"]:
    for row in candidates(name):
        try:
            collect(row, set())
        except KeyError:
            continue
        direct[name] = row
        break
    else:
        raise SystemExit(f"No complete cached dependency for {name}")
closure = set()
for row in direct.values():
    collect(row, closure)
for row in candidates("semio_framework_value_testing_allocator"):
    if dict(dependencies(row))[trace[0]][1] == trace[1]:
        collect(row, closure)
        break
registered = json.loads(subprocess.check_output(["repo", "tools", "list", "--json"]))
rustup = next(row["command"] for row in registered if row["id"] == "rustup")
channel = re.search(r'^channel\s*=\s*"([^"]+)"', (workspace / "rust-toolchain.toml").read_text(), re.M)[1]
exe = generated / ("core-stack-growth-native.exe" if os.name == "nt" else "core-stack-growth-native")
args = [rustup, "run", channel, "rustc", "--test", "--crate-name", root_crate, "--edition=2021", "-Zthreads=1", "-Cdebuginfo=0", str(source / "🦀️.rs"), "-o", str(exe)]
for name, library, *_ in direct.values():
    metadata = library.with_suffix(".rmeta")
    if library.suffix == ".rlib" and metadata.exists():
        args += ["--extern", f"{name}={metadata.relative_to(build)}"]
    args += ["--extern", f"{name}={library.relative_to(build)}"]
for directory in sorted({library.parent for library in closure} | {entry for entry in build.glob("*/*/out") if entry.is_dir()}):
    args += ["-L", f"dependency={directory.relative_to(build)}"]
for archive in sorted({archive.parent for archive in build.glob("*/*/out/*.a")}):
    args += ["-L", f"native={archive.relative_to(build)}"]
environment = dict(os.environ, CARGO_MANIFEST_DIR=str(manifest), CARGO_PKG_NAME=root_crate.replace("_", "-"))
print(f"[DEBUG] Relinking Core unit harness against {len(closure)} cached Rust dependencies", flush=True)
if subprocess.run(args, env=environment, cwd=build).returncode:
    raise SystemExit(1)
raise SystemExit(subprocess.run([str(exe), *sys.argv[1:]], env=environment).returncode)
