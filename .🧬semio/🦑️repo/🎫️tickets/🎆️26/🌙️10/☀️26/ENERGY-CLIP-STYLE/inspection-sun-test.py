"""🧪 Retain the independent sun regression output beneath its ticket."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("label", choices=("red", "green"))
args = parser.parse_args()
ticket = Path(__file__).resolve().parent
root = next(path for path in ticket.parents if (path / "🧰️framework").is_dir())
output = ticket / "🗑️generated" / "is"
output.mkdir(parents=True, exist_ok=True)
driver = root / "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🧪️tests/☀️sun/🐍️.py"
environment = dict(os.environ, SEMIO_TEACHING_OUTPUT_ROOT=str(output))
repo = os.environ.get("SEMIO_REPO_EXE") or shutil.which("repo") or "repo"
log = output / f"sun-{args.label}.log"
with log.open("w", encoding="utf-8") as stream:
    result = subprocess.run([repo, "tools", "exec", "python", "--light", "--", "-X", "utf8", str(driver)],
                            cwd=root, env=environment, stdout=stream, stderr=subprocess.STDOUT)
print(log.read_text(encoding="utf-8", errors="replace")[-12000:], flush=True)
raise SystemExit(result.returncode)
