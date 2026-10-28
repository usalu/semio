"""Find which scene line adds one mobject twice to a group (manim only warns)."""
import importlib
import os
import runpy
import sys
import traceback
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
module = importlib.import_module("manim.mobject.mobject")
seen = set()


def warn(message, *args, **kwargs):
    frames = traceback.extract_stack()[-9:]
    key = tuple((f.lineno, f.name) for f in frames)
    if key not in seen:
        seen.add(key)
        print("[DEBUG] duplicate child", [(Path(f.filename).parent.name, f.name, f.lineno) for f in frames], flush=True)


module.logger.warning = warn
sys.argv = [str(TICKET / "capture.py"), "👁️representations", "Beat3_SameObject", "en", "source"]
runpy.run_path(str(TICKET / "capture.py"), run_name="__main__")
