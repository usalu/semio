import ast
import json
from pathlib import Path
here = Path(__file__).parent
data = json.loads((here / "🗑️generated/bim-dependencies.json").read_text(encoding="utf-8"))
for source in data["literalImportClosure"]:
    tree = ast.parse(Path(source).read_text(encoding="utf-8"))
    classes = [n.name for n in tree.body if isinstance(n, ast.ClassDef)]
    if classes:
        print(source, classes)
