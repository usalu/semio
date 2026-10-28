"""Find scene methods that shadow toolkit mixin methods."""
import importlib, os, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
v = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
s = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.👁️representations.🐍️").Beat3_SameObject
for name in sorted(vars(s)):
    if name.startswith("__"): continue
    for base in v.ClipScene.__mro__:
        if base is not s and name in vars(base):
            print("[DEBUG] collision", name, base.__name__)
