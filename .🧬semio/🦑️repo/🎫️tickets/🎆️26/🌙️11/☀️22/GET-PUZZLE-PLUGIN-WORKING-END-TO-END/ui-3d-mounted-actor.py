from pathlib import Path
import os

ticket = Path(__file__).resolve().parent
base = ticket / "ui-transport-native.py"
code = base.read_text().split("\nif is_history:")[0]
target_volume = os.environ.get("PUZZLE_NATIVE_ACTOR_TARGET_VOLUME")
run_directory = "ui-3d-mounted-actor-scale" if target_volume else "ui-3d-mounted-actor"
code = code.replace('scratch = generated / "ui-preview-api" if api_only else common', f'scratch = generated / "{run_directory}"')
code = code.replace('crate = fingerprint.name.removeprefix', 'crate = "editor_action_producer" if not path else fingerprint.name.removeprefix')
code = code.replace('source = (config.parent / specification["lib"]["path"]).resolve()', 'source = ticket / "ui-3d-mounted-actor.rs" if not path else (config.parent / specification["lib"]["path"]).resolve()')
exec(compile(code, str(base), "exec"))
changed_packages = {"semio-s-artifact-puzzle-3d-model"} if target_volume else set()
baseline = max((cache / "semio-s-artifact-puzzle-3d-editor").glob("*/fingerprint/test-integration-test-editor-action-producer.json"), key=lambda path: path.stat().st_mtime)
_, dependencies = closure(baseline)
executable = compile_crate(None, baseline, dependencies)
raise SystemExit(subprocess.run([executable, "--nocapture"], env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode)
