from pathlib import Path
base = Path(__file__).resolve().with_name('ui-transport-native.py')
text = base.read_text()
text = text.replace('scratch = generated / ("ui-preview-native" if "--preview" in sys.argv else "ui-transport-native")','scratch = generated / "current-fill-source-native"')
a = text.index('changed_packages =')
b = text.index('historical =', a)
text = text[:a] + 'changed_packages = {"semio-plugin-tool-run-operations", "semio-plugin-tool-run-turns","semio-s-artifact-puzzle-2d-tests"}\n' + text[b:]
text = text.replace('return output\n    edition =', 'return output\n    common = generated / "current-3d-source-native" / output.name\n    if path and common.is_file() and path.name.startswith("libsemio_plugin") and common.stat().st_size:\n        return common\n    edition =')
text = text.replace('selection = ["history_edit_runtime_tests::"]','selection = [a for a in sys.argv[1:] if not a.startswith("--")]')
text = text.replace('env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode','cwd=manifest("semio-s-artifact-puzzle-2d-history-edit-runtime-tests").parent, env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode')
text = text.replace('f"dependency={shared}", "-o"', '''f"dependency={shared}", "-L", f"dependency={generated / 'current-3d-source-native'}", "-o"''')
exec(compile(text, str(base), 'exec'))
