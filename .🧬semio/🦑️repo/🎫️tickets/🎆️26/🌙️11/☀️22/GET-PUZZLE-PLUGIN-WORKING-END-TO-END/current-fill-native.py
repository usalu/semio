from pathlib import Path
import sys
base = Path(__file__).resolve().with_name("ui-transport-native.py")
code = base.read_text()
a = code.index("scratch = ")
b = code.index("shared = ", a)
code = code[:a] + 'scratch = generated / "current-fill-native"\n' + code[b:]
a = code.index("changed_packages =")
b = code.index('historical =', a)
code = code[:a] + 'changed_packages = {"semio-s-artifact-puzzle-2d-tests"}\n' + code[b:]
code = code.replace('selection = ["history_edit_runtime_tests::"]', 'selection = [a for a in sys.argv[1:] if not a.startswith("--")]')
code = code.replace('env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode', 'cwd=manifest("semio-s-artifact-puzzle-2d-history-edit-runtime-tests").parent, env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode')
exec(compile(code, str(base), "exec"))
