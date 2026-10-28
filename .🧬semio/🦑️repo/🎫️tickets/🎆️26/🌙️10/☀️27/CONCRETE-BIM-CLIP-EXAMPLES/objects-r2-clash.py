"""🔎 List scene method names that shadow toolkit mixin names: python objects-r2-clash.py."""
import importlib
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧬️objects.🐍️")
scene = module.Beat2_ObjectOrientation
for name in vars(scene):
    if name.startswith("__"):
        continue
    for base in scene.__mro__[1:]:
        if name in vars(base):
            print("[DEBUG] clash", name, base.__name__)
print("[DEBUG] checked")
