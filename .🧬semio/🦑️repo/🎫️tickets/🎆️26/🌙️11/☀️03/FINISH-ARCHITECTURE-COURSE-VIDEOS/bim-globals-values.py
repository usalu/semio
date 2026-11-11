import ast
import importlib
import json
import sys
from pathlib import Path
ROOT = Path.cwd()
sys.path.insert(0, str(ROOT))
data = json.loads((Path(__file__).parent / "🗑️generated/bim-dependencies.json").read_text(encoding="utf-8"))
for name in data["literalImportClosure"]:
    tree = ast.parse((ROOT / name).read_text(encoding="utf-8"))
    attrs = [t.id for n in tree.body if isinstance(n, (ast.Assign, ast.AnnAssign)) for t in (n.targets if isinstance(n, ast.Assign) else [n.target]) if isinstance(t, ast.Name)]
    if not attrs:
        continue
    module = importlib.import_module(name[:-3].replace("/", "."))
    print(name)
    for attr in attrs:
        value = getattr(module, attr)
        print(" ", attr, type(value).__name__, repr(value)[:100])
