from pathlib import Path
import sys
base = Path(__file__).resolve().with_name("ui-transport-native.py")
code = base.read_text()
a = code.index("scratch = ")
b = code.index("shared = ", a)
code = code[:a] + 'scratch = generated / "current-5d-member-native"\n' + code[b:]
a = code.index("changed_packages =")
b = code.index('historical =', a)
code = code[:a] + 'changed_packages = {"semio-s-artifact-puzzle-5d-testkit","semio-s-artifact-puzzle-5d-puzzle5d"}\n' + code[b:]
code = code.replace('selection = ["history_edit_runtime_tests::"]', 'selection = [a for a in sys.argv[1:] if not a.startswith("--")]')
code = code.replace('env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode', 'cwd=manifest("semio-s-artifact-puzzle-2d-history-edit-runtime-tests").parent, env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode')
code = code.replace("semio_s_artifact_puzzle_2d_history_edit_runtime_tests","semio_s_artifact_puzzle_5d_v1_unit_tests").replace("semio-s-artifact-puzzle-2d-history-edit-runtime-tests","semio-s-artifact-puzzle-5d-v1-unit-tests").replace("ce1cf6c12d6596e3","d78d73d3f1b96766")
code = code.replace('if key not in artifacts:\n            continue', """if key not in artifacts:
            candidates = tuple(shared.glob(f'lib{alias}-*.rlib'))
            if len(candidates) != 1:
                continue
            path = candidates[0].resolve()
            candidate = path.parent.parent / "fingerprint" / f"lib-{alias}"
            if not candidate.with_suffix(".json").exists():
                continue
            artifacts[key] = (path, candidate)""")
exec(compile(code, str(base), "exec"))
