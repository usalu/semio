import ast
import importlib
import sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
source=Path(m.__file__).read_text(encoding="utf-8")
assert len(m._nk.slots()) == 140
assert "for capsule in tower.capsules.values()" in source
assert "chosen = tower.capsules[show_key].copy()" in source
ast.parse(source)
print("140 slots; sprite capture and selected vector morph source retained")
