import os
from pathlib import Path
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
output = ticket / "🗑️generated"
fixture_root = next(root.glob("✏️s/🔌*/🧩*/🗿*/🖐️5d/🏅*/🔖*/🪆*/✳*"))
executable = output / "law-5d-option-schema/history"
scratch = output / "law-5d-full-corpus"
cwd = scratch / "1/2/3/4/5"
cwd.mkdir(parents=True, exist_ok=True)
selected = scratch / "🧪️tests/🧫️fixtures/🧬️mutations/🌱create-part"
selected.parent.mkdir(parents=True, exist_ok=True)
if not selected.exists():
    selected.symlink_to(fixture_root, target_is_directory=True)
assert selected.resolve() == fixture_root.resolve()
print(f"[DEBUG] Running the freshly embedded 5D schema against the complete authored fixture root {fixture_root}", flush=True)
env = {**os.environ, "RUST_MIN_STACK": "134217728"}
raise SystemExit(subprocess.run([executable, "--exact", "create_part_anchor_option_history", "--nocapture", "--test-threads=1"],
                                cwd=cwd, env=env).returncode)
