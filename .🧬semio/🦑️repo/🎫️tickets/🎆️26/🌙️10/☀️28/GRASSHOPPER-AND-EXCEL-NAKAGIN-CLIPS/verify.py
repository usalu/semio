"""🧪 Run a validation input with all temporary render/cache output owned by its ticket."""
import argparse
import os
from pathlib import Path
import runpy
import sys
sys.stdout.reconfigure(errors="backslashreplace", line_buffering=True)
sys.stderr.reconfigure(errors="backslashreplace", line_buffering=True)

parser = argparse.ArgumentParser()
parser.add_argument("target")
parser.add_argument("--label", required=True)
args, selected = parser.parse_known_args()
ticket = Path(__file__).resolve().parent
root = next(p for p in ticket.parents if (p / "🧰️framework").is_dir())
target = (root / args.target).resolve()
if not target.is_relative_to(root) or not target.is_file():
    raise ValueError("Validation input must be a file inside this repository")
output = (ticket / "🗑️generated" / args.label).resolve()
if not output.is_relative_to((ticket / "🗑️generated").resolve()):
    raise ValueError("Validation output must remain inside the ticket generated folder")
output.mkdir(parents=True, exist_ok=True)
for key in ("TMPDIR", "TEMP", "TMP", "SEMIO_TEACHING_OUTPUT_ROOT"):
    os.environ[key] = str(output)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(output / "cache")
sys.dont_write_bytecode = True
sys.path.insert(0, str(root))
from manim import config
config.media_dir, config.text_dir = str(output / "media"), str(output / "text")
sys.argv = [str(target), *selected]
runpy.run_path(str(target), run_name="__main__")
