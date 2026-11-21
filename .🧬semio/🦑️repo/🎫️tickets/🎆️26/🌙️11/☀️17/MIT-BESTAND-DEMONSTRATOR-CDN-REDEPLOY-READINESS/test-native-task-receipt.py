"""🧾️ Exercise real issue, publication, failure cleanup and errors through the native task helper."""
import json
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
helper = ticket / "native-receipt.mjs"

def run(*args):
    return subprocess.run(["node", str(helper), *args], cwd=root, capture_output=True, text=True)

with TemporaryDirectory(dir=ticket / "🗑️generated") as scratch:
    project = str(Path(scratch).relative_to(root))
    assert run("issue", project, "invalid target").returncode != 0
    assert run("settle", project, "build", "{", "1", "0").returncode != 0
    first = run("issue", project, "build")
    first.check_returncode()
    issued = json.loads(first.stdout)
    pending = Path(next(iter(issued.values())))
    pending.parent.mkdir()
    final = pending.parent / "build.json"
    pending.write_text("first")
    run("settle", project, "build", first.stdout, "1", "0").check_returncode()
    assert final.read_text() == "first" and not pending.exists()
    second = run("issue", project, "build")
    second.check_returncode()
    pending = Path(next(iter(json.loads(second.stdout).values())))
    pending.write_text("second")
    run("settle", project, "build", second.stdout, "0", "0").check_returncode()
    assert final.read_text() == "first" and not pending.exists()
print("Native task receipt errors, successful publication and failed-run cleanup passed.")
