import importlib.util, sys
from pathlib import Path
root = Path("🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py").resolve()
spec = importlib.util.spec_from_file_location("conduction_clip", root)
m = importlib.util.module_from_spec(spec); sys.modules["conduction_clip"] = m; spec.loader.exec_module(m)
for c in ("toaster", "tealight", "person"):
    n = "heating-conduction-" + c
    s = m.staffage(n, unit=1.0)
    print(c, m.staffage_height(n), s.width, s.height)
