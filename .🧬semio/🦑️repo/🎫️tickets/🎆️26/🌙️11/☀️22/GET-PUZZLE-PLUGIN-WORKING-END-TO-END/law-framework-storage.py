from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
base = ticket / "ui-history-frozen.py"
shared = base.read_text().split("test_deps = dependencies(test_fingerprint)", 1)[0]
shared = shared.replace("mounted = '--refresh-mounted' in sys.argv", "mounted = True")
shared = shared.replace("output = ticket / f'🗑️generated/ui-history-{variant}'", "output = ticket / '🗑️generated/law-storage'")
shared = shared.replace("cached_fixture = ticket / '🗑️generated/ui-history-frozen'", "cached_fixture = ticket / '🗑️generated/law-preparation'")
shared = shared.replace("'-C', 'opt-level=1'", "'-C', 'opt-level=0'")
exec(compile(shared, str(base), "exec"))
for manifest in list(manifests.values()):
    library_name = tomllib.loads(manifest.read_text()).get("lib", {}).get("name")
    if library_name:
        manifests.setdefault(library_name, manifest)

refresh_names = {
    "semio_framework_value_native_clone_engine",
    "semio_framework_value_paged_clone",
    "semio_framework_value_retained_clone",
    "semio_framework_value_paged",
}

def test_library(name):
    filename = f"test-lib-{name}.json"
    directory = name.replace("_", "-")
    candidates = sorted(path for cache in (upper, lower) for path in cache.glob(f"{directory}/*/fingerprint/{filename}"))
    manifest, source = source_of(name)
    if candidates:
        fingerprint = candidates[-1]
        selected = dependencies(fingerprint)
    else:
        key = next(key for key, value in artifacts.items() if value[0] == name)
        fingerprint = fingerprints[key]
        selected = dependencies(fingerprint)
        dev = tomllib.loads((manifest / "Cargo.toml").read_text()).get("dev-dependencies", {})
        for alias, config in sorted(dev.items(), key=lambda entry: not entry[0].startswith("semio-")):
            package = config.get("package", alias) if isinstance(config, dict) else alias
            alias = alias.replace("-", "_")
            package = package.replace("-", "_")
            candidates = [key for key, value in artifacts.items() if value[0] == package]
            if candidates:
                def compatibility(key):
                    edges = dependencies(fingerprints[key])
                    outgoing = sum(1 if edges[dep] == selected[dep] else -1 for dep in edges.keys() & selected.keys())
                    incoming = sum(1 if parent[alias] == key else -1
                                   for parent in (dependencies(fingerprints[dep]) for dep in selected.values())
                                   if alias in parent)
                    return outgoing + incoming
                selected[alias] = max(candidates, key=compatibility)
            else:
                print(f"[DEBUG] No frozen development dependency for {alias}", flush=True)
                return 2
    replacements = {alias: refresh(key) for alias, key in selected.items() if needs_refresh(key)}
    executable = compile_source(name, manifest, source, fingerprint, replacements, test=True, selected_deps=selected)
    env = {**os.environ, "RUST_MIN_STACK": "134217728"}
    return subprocess.run([str(executable), "--nocapture", "--test-threads=1"], cwd=manifest, env=env, check=False).returncode

if "--libraries-only" in sys.argv:
    for name in sorted(refresh_names):
        key = next(key for key, value in artifacts.items() if value[0] == name)
        refresh(key)
    raise SystemExit(0)

if "--puzzle" in sys.argv:
    name = "semio_s_artifact_puzzle_2d_connect_handles_preparation"
    fingerprint = upper / f"semio-s-artifact-puzzle-2d-connect-handles-preparation/d6a5df77d421813d/fingerprint/test-lib-{name}.json"
    manifest, source = source_of(name)
    replacements = {alias: refresh(key) for alias, key in dependencies(fingerprint).items() if needs_refresh(key)}
    executable = compile_source(name, manifest, source, fingerprint, replacements, test=True)
    env = {**os.environ, "RUST_MIN_STACK": "134217728"}
    filters = [arg for arg in sys.argv[1:] if not arg.startswith("--")]
    raise SystemExit(subprocess.run([str(executable), "--nocapture", "--test-threads=1", *filters], cwd=manifest, env=env, check=False).returncode)

status = 0
for name in sys.argv[1:]:
    if name.startswith("semio_"):
        result = test_library(name)
        status = status or result
raise SystemExit(status)
