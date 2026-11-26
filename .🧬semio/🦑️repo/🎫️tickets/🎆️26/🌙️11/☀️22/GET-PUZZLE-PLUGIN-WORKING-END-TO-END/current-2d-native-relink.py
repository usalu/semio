from pathlib import Path
from sys import argv
ticket = Path(__file__).resolve().parent
base = ticket / 'current-5d-relink.py'
source = base.read_text().replace("current-5d-relink","current-2d-hist-relink").replace("semio_s_artifact_puzzle_5d_v1_unit_tests","semio_s_artifact_puzzle_2d_history_edit_runtime_tests").replace("semio-s-artifact-puzzle-5d-v1-unit-tests","semio-s-artifact-puzzle-2d-history-edit-runtime-tests").replace("🖐*/🏅*/🔖*/🪆*/✳*/✏️*/🧪*/🔬️unit-2","◻️2d/🏅*/🔖*/🪆*/✳*/✏️*/🧪*/🧪️history-edit-runtime").replace("subprocess.run(args, env=env,check=True)","subprocess.run(args + ['-Z', 'threads=1'], env=env,check=True)")
source = source.replace("else 'tests')", "else os.environ.get('PUZZLE_NATIVE_TEST_OUTPUT', 'tests'))")
exec(compile(source, str(base), 'exec'))
