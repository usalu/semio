from pathlib import Path
import os, subprocess
ticket = Path(__file__).resolve().parent
base = ticket / 'ui-transport-native.py'
code = base.read_text().split('\nif is_history:')[0]
code = code.replace('scratch = generated / ("ui-preview-native" if "--preview" in sys.argv else "ui-transport-native")','scratch = generated / "current-3d-source-native"')
code = code.replace('crate = fingerprint.name.removeprefix', 'crate = "editor_surface" if not path else fingerprint.name.removeprefix')
code = code.replace('source = (config.parent / specification["lib"]["path"]).resolve()','source = generated / "current-fill-paid-probe.rs" if not path else (config.parent / specification["lib"]["path"]).resolve()')
code = code.replace('for feature in json.loads(json.loads(fingerprint.read_text())["features"]):', 'for feature in set(json.loads(json.loads(fingerprint.read_text())["features"])) | ({"artifact-app-testing"} if package == "semio-plugin-tool-run-operations" else set()):')
exec(compile(code, str(base), 'exec'))
changed_packages = {'semio-plugin-tool-run-operations','semio-plugin-tool-run-turns'}
baseline = next((cache / 'semio-s-artifact-puzzle-3d-editor').glob('*/fingerprint/test-integration-test-editor-surface.json'))
_, dependencies = closure(baseline)
executable = compile_crate(None, baseline, dependencies)
raise SystemExit(subprocess.run([str(executable),'--exact','publish_fill_count_from_original_window','--nocapture'],cwd=manifest('semio-s-artifact-puzzle-3d-editor').parent,env={**os.environ,'RUST_MIN_STACK':'134217728'}).returncode)
