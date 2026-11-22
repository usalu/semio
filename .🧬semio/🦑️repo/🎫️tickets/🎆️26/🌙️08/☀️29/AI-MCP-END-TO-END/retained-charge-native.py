"""Link the focused Source witness against the warmed native gateway units."""

import json
import os
from pathlib import Path
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
build = generated / "native-host" / "debug" / "build"
output = generated / "charge-native"
output.mkdir(exist_ok=True)
root = Path.cwd()
job = root / "🧰️framework/🔨️modules/🧵️job"
value = root / "🧰️framework/🔨️modules/🌱️value"
tests = job / "⏱️context/🧪️tests/📊️native-progress"
rustup = next(row["command"] for row in json.loads(subprocess.check_output(["repo", "tools", "list", "--json"])) if row["id"] == "rustup")
channel = re.search(r'^channel\s*=\s*"([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.M)[1]
rustc = [rustup, "run", channel, "rustc"]
index = output / "fingerprints.json"
if index.exists() and os.environ.get("SEMIO_CHARGE_REINDEX") != "1":
    cached = json.loads(index.read_text())
    native = set(map(Path, cached["native"]))
    libraries = {key: tuple(map(Path, paths)) for key, paths in cached["libraries"].items()}
else:
    print("[DEBUG] Indexing warmed native dependencies", flush=True)
    native = set()
    libraries = {}
    for out in build.glob("*/*/out"):
        for artifact in out.iterdir():
            if artifact.suffix in (".a", ".lib"):
                native.add(out)
            match = re.fullmatch(r"lib(.+)-[a-f0-9]+\.(rmeta|rlib|dylib|so|dll)", artifact.name)
            if match:
                fingerprint = out.parent / "fingerprint" / f"lib-{match[1]}"
                if fingerprint.exists():
                    key = fingerprint.read_text().strip()
                    previous = libraries.get(key)
                    if key and (previous is None or previous[0].suffix == ".rmeta"):
                        libraries[key] = (artifact, fingerprint.with_suffix(".json"))
    index.write_text(json.dumps({"native": list(map(str, native)), "libraries": {key: list(map(str, paths)) for key, paths in libraries.items()}}))

def dependencies(fingerprint):
    return {
        name: number.to_bytes(8, "little").hex()
        for _, name, _, number in json.loads(fingerprint.read_text())["deps"]
        if name != "build_script_build"
    }

def manifest(path):
    base = path / "📦️packages/🦀️rust"
    source = re.search(r'^path\s*=\s*"([^"]+)"', (base / "Cargo.toml").read_text(), re.M)[1]
    return base, base / source

search = {output}

def compile_unit(name, path, direct, *, test=False):
    output_file = output / (("witness.exe" if os.name == "nt" else "witness") if test else f"lib{name}.rlib")
    closure, pending = set(), list(direct.values())
    while pending:
        key = pending.pop()
        if isinstance(key, Path) or key in closure:
            continue
        closure.add(key)
        pending.extend(dependencies(libraries[key][1]).values())
    search.update(libraries[key][0].parent for key in closure)
    if not test and output_file.exists() and os.environ.get("SEMIO_CHARGE_WITNESS_ONLY") == "1":
        return output_file
    args = rustc + ["--test" if test else "--crate-type=rlib", "--crate-name", name, "--edition=2021", "-Zthreads=1", "-Cdebuginfo=0", "-Copt-level=3", "-Cmetadata=retained_charge", "-o", str(output_file)]
    for directory in sorted(search):
        args.extend(("-L", f"dependency={directory}"))
    for directory in sorted(native):
        args.extend(("-L", f"native={directory}"))
    for dep, key in direct.items():
        artifact = key if isinstance(key, Path) else libraries[key][0]
        metadata = artifact.with_suffix(".rmeta")
        if artifact.suffix == ".rlib" and metadata.exists():
            args.extend(("--extern", f"{dep}={metadata}"))
        args.extend(("--extern", f"{dep}={artifact}"))
    manifest_dir, source = (path.parent, path) if test else manifest(path)
    args.append(str(source))
    print(f"[DEBUG] Compiling {name}", flush=True)
    result = subprocess.run(args, env={**os.environ, "CARGO_MANIFEST_DIR": str(manifest_dir), "CARGO_PKG_NAME": name.replace("_", "-")})
    if result.returncode:
        sys.exit(result.returncode)
    return output_file

turn_fingerprints = (build / "semio-framework-job-native-turn").glob("*/fingerprint/lib-semio_framework_job_native_turn.json")
turn_fingerprint = next(
    fp for fp in sorted(turn_fingerprints, key=lambda fp: fp.stat().st_mtime, reverse=True)
    if fp.with_suffix("").read_text().strip() in libraries
    and dependencies(fp)["semio_framework_job_context"] in libraries
    and libraries[dependencies(fp)["semio_framework_job_context"]][0].suffix == ".rlib"
)
turn_deps = dependencies(turn_fingerprint)
context_deps = dependencies(libraries[turn_deps["semio_framework_job_context"]][1])
fixture_fingerprint = build / "semio-framework-job-testing-caller/bf8d1a5dc550729b/fingerprint/lib-semio_framework_job_testing_caller.json"
fixture_deps = dependencies(fixture_fingerprint)
serde_key = fixture_deps["serde_json"]
control_key = context_deps["semio_framework_value_control_contract"]
trace_key = context_deps["semio_framework_trace"]
allocator_key = next(
    (key for key, (artifact, fingerprint) in libraries.items()
     if artifact.name.startswith("libsemio_framework_value_testing_allocator-") and artifact.suffix == ".rlib"
     and dependencies(fingerprint).get("semio_framework_trace") == trace_key),
    None,
)
if allocator_key is None:
    trace_testing = compile_unit("semio_framework_trace_memory_testing", root / "🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests", {
        "semio_framework_trace": trace_key,
    })
    allocator_key = compile_unit("semio_framework_value_testing_allocator", value / "🧪️testing/💾️allocator", {
        "semio_framework_trace": trace_key, "semio_framework_trace_memory_testing": trace_testing,
    })
context = compile_unit("semio_framework_job_context", job / "⏱️context", context_deps)
native_turn = compile_unit("semio_framework_job_native_turn", job / "🛬️receiving/🎟️native-turn", {**turn_deps, "semio_framework_job_context": context})
bank = compile_unit("semio_framework_value_testing_effect_bank", value / "🧪️testing/📥️admission/🎟️effect", {
    "serde_json": serde_key, "semio_framework_value_control_contract": control_key,
})
fixture = compile_unit("semio_framework_job_testing_caller", job / "⏱️context/🧪️testing/🎟️caller", {
    **{name: context_deps.get(name, key) for name, key in fixture_deps.items()},
    "semio_framework_job_native_turn": native_turn,
    "semio_framework_job_context": context,
    "semio_framework_value_testing_effect_bank": bank,
    "semio_framework_value_testing_allocator": allocator_key,
})
wrapper = output / "witness.rs"
suite = (tests / "🦀️.rs").read_text()
helpers = "\n".join(line for line in suite.splitlines() if line.startswith(("fn stock()", "fn quantum()")))
wrapper.write_text(f'''use semio_framework_job_context::RetainedCloneGrant;
{helpers}
#[path = "{(tests / "retained-charge.rs").as_posix()}"]
mod retained_charge;
''')
witness = compile_unit("retained_charge_witness", wrapper, {
    **context_deps, "semio_framework_job_context": context,
    "semio_framework_job_testing_caller": fixture, "serde_json": serde_key,
    "semio_framework_value_testing_allocator": allocator_key,
}, test=True)
sys.exit(subprocess.call([str(witness), "--nocapture", *sys.argv[1:]]))
