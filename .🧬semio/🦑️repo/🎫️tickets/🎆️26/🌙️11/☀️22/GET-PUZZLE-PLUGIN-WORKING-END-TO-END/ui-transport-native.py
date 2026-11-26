import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tomllib

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
cache = generated / "current-native/b/debug/build"
api_only = "--preview-api" in sys.argv
common = generated / ("ui-preview-native" if "--preview" in sys.argv else "ui-transport-native")
scratch = generated / "ui-preview-api" if api_only else common
shared = generated / "current-2d-hist-relink/deps"
scratch.mkdir(parents=True, exist_ok=True)
rustc = root / ".cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc"
native = set(path.parent for path in cache.glob("*/*/out/*.a"))
artifacts = {}
for path in shared.iterdir():
    if path.suffix not in (".rlib", ".so") or not path.name.startswith("lib"):
        continue
    source = path.resolve()
    stem = source.name.rsplit("-", 1)[0][3:]
    fingerprint = source.parent.parent / "fingerprint" / f"lib-{stem}"
    if fingerprint.exists():
        artifacts[fingerprint.read_text().strip()] = (source, fingerprint)
changed_packages = {"semio-os-store-engine-core", "semio-os-store-engine-live", "semio-os-store-engine"}
if "--preview" in sys.argv:
    changed_packages.update({"semio-os-store-read", "semio-plugin-time-travel-turns"})
historical = "semio_s_artifact_puzzle_2d_history_edit_runtime_tests"
baseline = cache / "semio-s-artifact-puzzle-2d-history-edit-runtime-tests/ce1cf6c12d6596e3/fingerprint" / f"test-lib-{historical}.json"
manifest_map = {}
is_history = "--history" in sys.argv
is_focused = "--focused" in sys.argv
compiled = {}


def manifest(package):
    if package not in manifest_map:
        expression = f'^name\\s*=\\s*"{re.escape(package)}"'
        prefixes = ["🧰️framework", "✏️s"]
        candidates = subprocess.check_output(["rg", "-l", "-g", "Cargo.toml", expression, *prefixes], cwd=root, text=True).splitlines()
        if len(candidates) != 1:
            raise RuntimeError((package, candidates))
        manifest_map[package] = root / candidates[0]
    return manifest_map[package]


def closure(fingerprint):
    data = json.loads(fingerprint.read_text())
    paths = {}
    changed = False
    for _, alias, _, checksum in data["deps"]:
        key = checksum.to_bytes(8, "little").hex()
        if key not in artifacts:
            continue
        path, library_fingerprint = artifacts[key]
        if key not in compiled:
            rebuilt, externs = closure(library_fingerprint.with_suffix(".json"))
            package = path.parents[2].name
            if rebuilt or package in changed_packages:
                path = compile_crate(path, library_fingerprint.with_suffix(".json"), externs)
                rebuilt = True
            compiled[key] = (rebuilt, path)
        rebuilt, path = compiled[key]
        changed |= rebuilt
        paths[alias] = path
    return changed, paths


def compile_crate(path, fingerprint, externs):
    package = fingerprint.parents[2].name
    if path and "--reuse" in sys.argv and (common / path.name).is_file():
        if not api_only or package not in {"semio-os-store-engine-derived", "semio-os-store-engine"}:
            return common / path.name
    crate = fingerprint.name.removeprefix("test-").removeprefix("lib-").removesuffix(".json")
    config = manifest(package)
    specification = tomllib.loads(config.read_text())
    source = (config.parent / specification["lib"]["path"]).resolve()
    output = scratch / (path.name if path else ("history-focused" if is_focused else "history-tests"))
    if path is None and is_focused:
        history_source = config.parent.parent.parent / "🦀️.rs"
        source = scratch / "focused.rs"
        template = (ticket / "ui-history-focused.rs").read_text()
        source.write_text(template.replace("@HISTORY@", history_source.as_posix()))
    edition = specification["package"].get("edition", "2021")
    if isinstance(edition, dict):
        workspace = config.parent / specification["package"]["workspace"] / "Cargo.toml"
        edition = tomllib.loads(workspace.read_text())["workspace"]["package"]["edition"]
    args = [str(rustc), str(source), "--edition", edition, "--crate-name", crate, "-Z", "threads=1", "-C", "debuginfo=0", "-L", f"dependency={scratch}", "-L", f"dependency={shared}", "-o", str(output)]
    if api_only:
        args.extend(["-L", f"dependency={common}"])
    if path:
        args.extend(["--crate-type", "rlib", "-C", f"metadata={path.stem.rsplit('-', 1)[-1]}"])
        if package in {"semio-plugin-actor-input", "semio-s-artifact-stdio-pdf-model"}:
            args.extend(["-C", "codegen-units=64"])
    else:
        args.extend(["--test", "-Z", "unstable-options", "-C", "linker-features=+lld", "-C", "link-self-contained=+linker"])
        if is_focused:
            args.extend(["-C", "codegen-units=64"])
    for feature in json.loads(json.loads(fingerprint.read_text())["features"]):
        args.extend(["--cfg", f'feature="{feature}"'])
    for alias, dependency in externs.items():
        args.extend(["--extern", f"{alias}={dependency}"])
        if dependency.with_suffix(".rmeta").is_file():
            args.extend(["--extern", f"{alias}={dependency.with_suffix('.rmeta')}"])
    for directory in native:
        args.extend(["-L", f"native={directory}"])
    env = {**os.environ, "CARGO_MANIFEST_DIR": str(config.parent), "CARGO_PKG_NAME": package, "CARGO_PKG_VERSION": "0.1.0", "OUT_DIR": str(fingerprint.parent.parent / "out"), "RUST_MIN_STACK": "134217728", "ZIG_GLOBAL_CACHE_DIR": str(generated / "zig"), "ZIG_LOCAL_CACHE_DIR": str(generated / "zig-local")}
    print(f"[DEBUG] Building {package}", flush=True)
    subprocess.run(args, env=env, check=True, cwd=config.parent)
    return output


if is_history:
    _, dependencies = closure(baseline)
    executable = compile_crate(None, baseline, dependencies)
    selection = ["history_edit_runtime_tests::"]
elif "--preview" in sys.argv:
    target = "lib-semio_os_store_engine" if api_only else "lib-semio_os_store_engine_derived"
    candidates = [pair for pair in artifacts.values() if pair[1].name == target]
    engine, engine_fingerprint = max(candidates, key=lambda pair: ((common / pair[0].name).is_file(), pair[1].stat().st_mtime))
    engine_fingerprint = engine_fingerprint.with_suffix(".json")
    _, dependencies = closure(engine_fingerprint)
    compile_crate(engine, engine_fingerprint, dependencies)
    raise SystemExit(0)
else:
    raise SystemExit(2)
raise SystemExit(subprocess.run([str(executable), "--test-threads=1", "--nocapture", *selection], env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode)
