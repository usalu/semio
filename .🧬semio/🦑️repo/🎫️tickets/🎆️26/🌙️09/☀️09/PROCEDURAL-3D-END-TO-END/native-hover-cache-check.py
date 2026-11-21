from pathlib import Path
import sys
import json
import os
import subprocess
import re

mode = sys.argv[1] if len(sys.argv) > 1 else 'metadata'

root = Path.cwd()
generated = Path(__file__).resolve().parent / "🗑️generated" / "native"
deps = generated / "deps"
deps.mkdir(parents=True, exist_ok=True)
cache = root / ".cache/b/r/m/u/l/p9ded5f/prototype/build"
world = next(root.glob("🧰️framework/*/💻️os/*/♾️infinite/🌍️world"))
names = ["model", "components", "cursors", "scene", "bridge", "interaction", "host"]
manifests = {}
for manifest in world.rglob("Cargo.toml"):
    data = manifest.read_text()
    name = re.search(r'^name = "(.*?)"', data, re.M)[1]
    manifests[name] = (manifest, data)

cache_artifacts = {}
for output in cache.glob("*/*/out"):
    for artifact in output.glob("lib*"):
        if artifact.suffix not in {".rlib", ".dylib", ".rmeta"}:
            continue
        key = artifact.stem[3:]
        if key not in cache_artifacts or artifact.suffix != ".rmeta":
            cache_artifacts[key] = artifact

def cached_production(name):
    for f in (cache / name).glob("*/fingerprint/lib-*.json"):
        fingerprint = json.loads(f.read_text())
        if fingerprint["features"] == "[]":
            return f, fingerprint
    raise ValueError(name)

def package_name(alias, data):
    for match in re.finditer(r'^([a-z0-9_-]+) = (.*)$', data, re.M):
        key, spec = match.groups()
        if key.replace("-", "_") == alias:
            renamed = re.search(r'package = "(.*?)"', spec)
            return (renamed[1] if renamed else key).replace("-", "_")
    raise ValueError(alias)

rebuilt_artifacts = {}
cache_host, _ = cached_production("semio-os-infinite-world-host")
metadata = next((cache_host.parent.parent / "out").glob("*.semio-deps"))
for (key, _) in json.loads(metadata.read_text())["dependencies"]:
    if key.rsplit("-", 1)[0].startswith("semio_os_infinite_world_"):
        continue
    artifact = cache_artifacts.get(key)
    if artifact is not None:
        for sibling in [artifact, artifact.with_suffix(".rmeta")]:
            link = deps / sibling.name
            if sibling.exists() and not link.exists():
                link.symlink_to(sibling)
for key, artifact in cache_artifacts.items():
    if artifact.suffix == ".dylib":
        link = deps / artifact.name
        if not link.exists():
            link.symlink_to(artifact)

def compile_world(name):
    manifest, data = manifests[name]
    fingerprint_path, fingerprint = cached_production(name)
    name_us = name.replace("-", "_")
    key = name_us + "-" + fingerprint_path.parent.parent.name
    metadata = Path(str(cache_artifacts[key]).replace(".rlib", ".rmeta.semio-deps"))
    dependencies = [id for (id, svh) in json.loads(metadata.read_text())["dependencies"]]
    externs = {}
    for (_, alias, _, _) in fingerprint["deps"]:
        dep = package_name(alias, data)
        if dep in rebuilt_artifacts:
            externs[alias] = rebuilt_artifacts[dep]
        else:
            keys = [key for key in dependencies if key.rsplit("-", 1)[0] == dep and key in cache_artifacts]
            if len(keys) != 1:
                raise ValueError((name, alias, keys))
            artifact = cache_artifacts[keys[0]]
            externs[alias] = artifact.with_suffix('.rmeta') if artifact.with_suffix('.rmeta').exists() else artifact
    if mode == "test" and name.endswith("-host"):
        externs["semio_framework_test_fixtures"] = cache_artifacts["semio_framework_test_fixtures-db60097bcef07695"].with_suffix(".rmeta")
    output = deps / ("lib" + name_us + ("-hover.rlib" if mode == "test" else "-hover.rmeta"))
    if mode == "test":
        saved_externs.update(externs)
    rebuilt_artifacts[name_us] = output
    args = ["--edition=2021", "--crate-name", name_us, "--crate-type=rlib", "-C", "metadata=hover", "-C", "debuginfo=0", "-L", f"dependency={deps}"]
    if mode == "test":
        args.extend(["--cfg", 'feature="testkit"'])
    else:
        args.append("--emit=metadata")
    for directory in {artifact.parent for key, artifact in cache_artifacts.items() if not key.startswith("semio_os_infinite_world_")}:
        args.extend(["-L", f"dependency={directory}"])
    for alias, artifact in externs.items():
        if artifact.suffix == ".rmeta" and artifact.with_suffix(".rlib").exists():
            args.extend(["--extern", f"{alias}={artifact.with_suffix('.rlib')}"])
        args.extend(["--extern", f"{alias}={artifact}"])
    args.extend([str((manifest.parent / "../../🦀️.rs").resolve()), "-o", str(output)])
    env = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest.parent), "CARGO_PKG_VERSION": "0.1.0", "CARGO_PKG_NAME": name}
    print(f"[DEBUG] Compiling {name}", flush=True)
    if mode == "test" and output.exists():
        return
    result = subprocess.run(compiler + args, env=env)
    if result.returncode:
        sys.exit(result.returncode)

compiler = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
saved_externs = {}
for name in names:
    compile_world("semio-os-infinite-world-" + name)

if mode == "test":
    import resource
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    manifest, data = manifests["semio-os-infinite-world-host"]
    env = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest.parent), "CARGO_PKG_VERSION": "0.1.0", "CARGO_PKG_NAME": "semio-os-infinite-world-host"}
    saved_externs.update(rebuilt_artifacts)
    extra = {"base64_codec": "semio_framework_io_base64", "image": "image",
        "semio_framework_trace": "semio_framework_trace", "semio_framework_hash": "semio_framework_hash"}
    for alias, original in extra.items():
        candidates = [artifact for key, artifact in cache_artifacts.items() if key.rsplit("-", 1)[0] == original]
        if len(candidates) == 1:
            saved_externs[alias] = candidates[0]
    cases = {
        "world_host_pointer_gestures": ["an_authored_instance_hover_group", "authored_hover_group", "authored_hover_falls_back"],
        "world_host_unit_2": ["scene_bridge_replaces_and_retires_authored_hover_targets"],
    }
    for name, filters in cases.items():
        if len(sys.argv) > 2 and name != sys.argv[2]:
            continue
        test = re.search('name = "' + name + r'".*?path = "(.*?)"', data, re.S)[1]
        binary = generated / name
        args = ["--edition=2021", "--crate-name", name, "--test", "-C", "debuginfo=0", "-L", f"dependency={deps}"]
        for directory in {artifact.parent for artifact in cache_artifacts.values()}:
            args.extend(["-L", f"dependency={directory}"])
        for alias, artifact in saved_externs.items():
            if artifact.suffix == ".rmeta" and artifact.with_suffix(".rlib").exists():
                args.extend(["--extern", f"{alias}={artifact.with_suffix('.rlib')}"])
            if artifact.suffix == ".rlib" and artifact.with_suffix(".rmeta").exists():
                args.extend(["--extern", f"{alias}={artifact.with_suffix('.rmeta')}"])
            args.extend(["--extern", f"{alias}={artifact}"])
        print(f"[DEBUG] Compiling {name}", flush=True)
        result = subprocess.run(compiler + args + [str((manifest.parent / test).resolve()), "-o", str(binary)], env=env)
        if result.returncode:
            sys.exit(result.returncode)
        run = ["repo", "tools", "exec", "--light", "python", "--", "-c", "import os, sys; os.execv(sys.argv[1], sys.argv[1:])"]
        for test_filter in filters:
            result = subprocess.run(run + [str(binary), test_filter], env=env)
            if result.returncode:
                sys.exit(result.returncode)
