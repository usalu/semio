import sys
sys.path.insert(0, "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python")
from importlib import import_module
import importlib.util, pathlib
p = pathlib.Path("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py")
spec = importlib.util.spec_from_file_location("stf", p); m = importlib.util.module_from_spec(spec); sys.modules["stf"]=m; spec.loader.exec_module(m)
for c in sys.argv[1:]:
    print(c, m.ensure_casts([c])[c])
    try:
        for pre in ("box", "bottom-rail", "slat-", "block-sill", "block-head"):
            d = m.staffage_parts(c, pre); print(" ", pre, len(d), list(d)[:3])
    except Exception as e: print("  ERR", e)
