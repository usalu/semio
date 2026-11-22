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

for candidate in candidates("serde_json"):
    trial = set()
    try:
        collect(candidate, trial)
    except KeyError:
        continue
    direct["serde_json"] = candidate
    closure.update(trial)
    break
else:
    raise SystemExit("No cached serde_json library")

channel = re.search(r'^channel\s*=\s*"([^"]+)"', (workspace / "rust-toolchain.toml").read_text(), re.M)[1]
compiler = ["repo", "tools", "exec", "--light", f"--cwd={build}", "rustup", "--", "run", channel, "rustc"]
library_args = [arg for directory in sorted({library.parent for library in closure})
                for arg in ("-L", f"dependency={directory.relative_to(build)}")]
effect_bank = "semio_framework_value_testing_effect_bank"
effect_source = workspace / "🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/🎟️effect"
effect_output = generated / f"lib{effect_bank}.rlib"
effect_args = [*compiler, "--crate-type=rlib", "--crate-name", effect_bank, "--edition=2021",
               "-Zthreads=1", "-Cdebuginfo=0", str(effect_source / "🦀️.rs"), "-o", str(effect_output)]
for name in ("semio_framework_value_control_contract", "serde_json"):
    library = direct[name][0]
    metadata = library.with_suffix(".rmeta")
    if metadata.is_file():
        effect_args += ["--extern", f"{name}={metadata.relative_to(build)}"]
    effect_args += ["--extern", f"{name}={library.relative_to(build)}"]
if subprocess.run([*effect_args, *library_args]).returncode:
    raise SystemExit(1)

exe = generated / ("return-budget-observation-native.exe" if os.name == "nt" else "return-budget-observation-native")
args = [*compiler, "--test", "--crate-name", root_crate, "--edition=2021", "-Zthreads=1", "-Cdebuginfo=0",
        str(source / "🦀️.rs"), "-o", str(exe), "--extern", f"{effect_bank}={effect_output}"]
for name, (library, _, _) in direct.items():
    metadata = library.with_suffix(".rmeta")
    if library.suffix == ".rlib" and metadata.is_file():
        args += ["--extern", f"{name}={metadata.relative_to(build)}"]
    args += ["--extern", f"{name}={library.relative_to(build)}"]
args.extend(library_args)
environment = dict(os.environ, CARGO_MANIFEST_DIR=str(source / "📦️packages/🦀️rust"),
                   CARGO_PKG_NAME=root_crate.replace("_", "-"))
print(f"[DEBUG] Relinking return-budget unit tests against {len(closure)} cached libraries", flush=True)
if subprocess.run(args, env=environment).returncode:
    raise SystemExit(1)
raise SystemExit(subprocess.run([str(exe), *sys.argv[1:]], env=environment).returncode)
