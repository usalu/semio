from pathlib import Path
import json
import os
import re
import subprocess
import sys

workspace = Path.cwd()
generated = Path(__file__).resolve().parent / "🗑️generated"
build = generated / "native-host/debug/build"
source = workspace / "🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget"
root_crate = "semio_framework_job_return_budget"
rows = {}
for fingerprint in build.glob("*/*/fingerprint/lib-*.json"):
    library = fingerprint.parent.parent
    name = fingerprint.stem.removeprefix("lib-")
    for extension in ("rlib", "dylib", "so", "dll"):
        artifact = library / "out" / f"lib{name}-{library.name}.{extension}"
        if artifact.is_file() and fingerprint.with_suffix("").is_file():
            key = fingerprint.with_suffix("").read_text().strip()
            rows[key] = (artifact, fingerprint, json.loads(fingerprint.read_text()))
            break


def dependencies(row):
    return {
        name: rows[digest.to_bytes(8, "little").hex()]
        for _, name, _, digest in row[2]["deps"]
        if name != "build_script_build"
    }


def collect(row, closure):
    library = row[0]
    if library not in closure:
        closure.add(library)
        for dependency in dependencies(row).values():
            collect(dependency, closure)


def candidates(name):
    matches = (row for row in rows.values() if row[1].stem == f"lib-{name}")
    return sorted(matches, key=lambda row: row[1].stat().st_mtime, reverse=True)


for root in candidates(root_crate):
    closure = set()
    try:
        direct = dependencies(root)
        for row in direct.values():
            collect(row, closure)
    except KeyError:
        continue
    break
else:
    raise SystemExit("No complete cached dependency closure for return-budget")

for name in ["semio_framework_value_testing_effect_bank", "serde_json"]:
    for candidate in candidates(name):
        trial = set()
        try:
            collect(candidate, trial)
        except KeyError:
            continue
        if name != "serde_json" and dependencies(candidate)["semio_framework_value_control_contract"][0] != direct["semio_framework_value_control_contract"][0]:
            continue
        direct[name] = candidate
        closure.update(trial)
        break
    else:
        raise SystemExit(f"No compatible cached library for {name}")

channel = re.search(r'^channel\s*=\s*"([^"]+)"', (workspace / "rust-toolchain.toml").read_text(), re.M)[1]
exe = generated / ("return-budget-observation-native.exe" if os.name == "nt" else "return-budget-observation-native")
args = ["repo", "tools", "exec", "--light", f"--cwd={build}", "rustup", "--", "run", channel, "rustc",
        "--test", "--crate-name", root_crate, "--edition=2021", "-Zthreads=1", "-Cdebuginfo=0",
        str(source / "🦀️.rs"), "-o", str(exe)]
for name, (library, _, _) in direct.items():
    metadata = library.with_suffix(".rmeta")
    if library.suffix == ".rlib" and metadata.is_file():
        args += ["--extern", f"{name}={metadata.relative_to(build)}"]
    args += ["--extern", f"{name}={library.relative_to(build)}"]
for directory in sorted({library.parent for library in closure}):
    args += ["-L", f"dependency={directory.relative_to(build)}"]
environment = dict(os.environ, CARGO_MANIFEST_DIR=str(source / "📦️packages/🦀️rust"),
                   CARGO_PKG_NAME=root_crate.replace("_", "-"))
print(f"[DEBUG] Relinking return-budget unit tests against {len(closure)} cached libraries", flush=True)
subprocess.run(args, env=environment, check=True)
raise SystemExit(subprocess.run([str(exe), *sys.argv[1:]], env=environment).returncode)
