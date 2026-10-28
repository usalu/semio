import importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
s = m.surface()
print(s["wwr"][16], [round(s["electricity"][16][j], 3) for j in range(5)], [round(s["electricity"][i][0], 2) for i in range(14, 19)], [round(s["electricity"][i][1], 2) for i in range(14, 19)])
