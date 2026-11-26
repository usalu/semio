from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
base = ticket / "ui-history-frozen.py"
shared = base.read_text().split("test_deps = dependencies(test_fingerprint)", 1)[0]
shared = shared.replace("variant = 'mounted' if mounted else 'frozen'", "variant = '3d-law'")
shared = shared.replace("mounted = '--refresh-mounted' in sys.argv", "mounted = True")
shared = shared.replace("output = ticket / f'🗑️generated/ui-history-{variant}'", "output = ticket / '🗑️generated/law-relink-3d'")
exec(compile(shared, str(base), "exec"))

editor = puzzle / "🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
test_manifest = editor / "📦️packages/🦀️rust"
suite, key, test_dir = (
    ("editor-action-producer", "f60ebcd6fe29ffac", "🔬️action-producer") if "--action-producer" in sys.argv
    else ("editor-gestures", "54b5221ba28c3487", "🔬️gestures") if "--gestures" in sys.argv
    else ("editor-surface", "894730afeea35cb2", "🔬️unit")
)
test_name = suite.replace("-", "_")
test_fingerprint = upper / f"semio-s-artifact-puzzle-3d-editor/{key}/fingerprint/test-integration-test-{suite}.json"
env = {**os.environ, "RUST_MIN_STACK": "134217728"}
if "--jobs-law" in sys.argv:
    jobs_key = dependencies(test_fingerprint)[jobs_name]
    fingerprint = fingerprints[jobs_key]
    keys = dependencies(fingerprint)
    keys["serde_json"] = dependencies(test_fingerprint)["serde_json"]
    executable = compile_source(jobs_name, jobs_manifest, jobs_manifest.parent.parent / "🦀️.rs", fingerprint, {}, test=True, selected_deps=keys)
    raise SystemExit(subprocess.run([str(executable), "--nocapture", sys.argv[1]], cwd=jobs_manifest, env=env).returncode)
refresh_names = set(arg for arg in sys.argv[2:] if arg.startswith("semio_")) or {jobs_name}
live_source = source_of
builder = ticket / "🗑️generated/builder"

def frozen(path):
    prefix, remainder = next((prefix, path.relative_to(prefix)) for prefix in (puzzle.parent, root / "🧰️framework") if path.is_relative_to(prefix))
    layer = 2 if prefix == puzzle.parent else 1
    return next((candidate for candidate in (builder / "ovs" / str(layer) / "u" / remainder, builder / "snap" / str(layer) / remainder) if candidate.exists()), path)

def source_of(name):
    manifest, source = live_source(name)
    if name in refresh_names:
        return manifest, source
    return frozen(manifest / "Cargo.toml").parent, frozen(source.resolve())

replacements = {alias: refresh(key) for alias, key in dependencies(test_fingerprint).items() if needs_refresh(key)}
test_source = editor / "🧪️tests" / test_dir / "🦀️.rs"
if "--live-tests" not in sys.argv:
    test_source = frozen(test_source)
executable = compile_source(test_name, test_manifest, test_source, test_fingerprint, replacements, test=True)
raise SystemExit(subprocess.run([str(executable), "--nocapture", "--test-threads=1", sys.argv[1]], cwd=test_manifest, env=env).returncode)
