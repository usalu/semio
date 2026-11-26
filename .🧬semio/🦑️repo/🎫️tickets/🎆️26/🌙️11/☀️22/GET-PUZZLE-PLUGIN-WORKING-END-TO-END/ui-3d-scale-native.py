from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
base = ticket / "ui-transport-native.py"
code = base.read_text().split("\nif is_history:")[0]
dimension = "5d" if "--five" in sys.argv else "3d"
code = code.replace('scratch = generated / "ui-preview-api" if api_only else common', f'scratch = generated / "ui-{dimension}-scale-projection"')
exec(compile(code, str(base), "exec"))
changed_packages = set()
name = "semio_s_artifact_puzzle_5d_document_records" if dimension == "5d" else "semio_s_artifact_puzzle_3d_model"
features = "testkit" if dimension == "5d" else "testing"
fingerprints = (cache / name.replace("_", "-")).glob(f"*/fingerprint/lib-{name}.json")
baseline = max(fingerprints, key=lambda path: (features in json.loads(json.loads(path.read_text())["features"]), path.stat().st_mtime))
_, dependencies = closure(baseline)
if dimension == "5d":
    dependencies["semio_framework_test_fixtures"] = max(shared.glob("libsemio_framework_test_fixtures-*.rlib"), key=lambda path: path.stat().st_mtime)
executable = compile_crate(None, baseline, dependencies)
raise SystemExit(subprocess.run([executable, "bounded_scale_projection_preserves_committed_axes", "--nocapture"]).returncode)
