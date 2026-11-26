from pathlib import Path
ticket = Path(__file__).resolve().parent
base = ticket/'ui-history-frozen.py'
shared = base.read_text().split('test_deps = dependencies(test_fingerprint)', 1)[0]
shared = shared.replace("mounted = '--refresh-mounted' in sys.argv","mounted = True")
shared = shared.replace("variant = 'mounted' if mounted else 'frozen'","variant = '5d-media'")
shared = shared.replace("output = ticket / f'🗑️generated/ui-history-{variant}'","output = ticket/'🗑️generated/current-frozen-media'")
exec(compile(shared,str(base),'exec'))
editor = puzzle/'🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
manifest = editor/'🧪️tests/🔬️unit-2/📦️packages/🦀️rust'
name = 'semio_s_artifact_puzzle_5d_v1_unit_tests'
fp = max((lower/'semio-s-artifact-puzzle-5d-v1-unit-tests').glob('*/fingerprint/test-lib-*.json'),key=lambda path:path.stat().st_mtime)
refresh_names = {'semio_s_artifact_puzzle_5d_puzzle5d', 'semio_s_artifact_puzzle_5d_testkit'}
replacements = {alias:refresh(key) for alias,key in dependencies(fp).items() if needs_refresh(key)}
executable = compile_source(name,manifest,manifest/'🦀️.rs',fp,replacements,test=True)
raise SystemExit(subprocess.run([str(executable),'--test-threads=1','kit_in_','rewrite_in_'],cwd=manifest,env={**os.environ,'RUST_MIN_STACK':'134217728'}).returncode)
