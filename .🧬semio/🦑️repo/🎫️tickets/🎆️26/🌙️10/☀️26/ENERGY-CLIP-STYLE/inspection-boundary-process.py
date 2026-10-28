"""🪟 Launch the registered independent boundary process from retained ticket input."""
from pathlib import Path
import runpy
import sys

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
worker = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🧪️tests/🧩️chapters/🐍️process.py"
sys.argv = [str(worker), sys.argv[1], str(TICKET / "🗑️generated/iv/boundary-process")]
runpy.run_path(str(worker), run_name="__main__")
