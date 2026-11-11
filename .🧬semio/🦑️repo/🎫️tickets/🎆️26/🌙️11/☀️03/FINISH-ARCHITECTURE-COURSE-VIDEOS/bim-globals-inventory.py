import ast
import json
from pathlib import Path
HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
data = json.loads((HERE / "🗑️generated/bim-dependencies.json").read_text(encoding="utf-8"))
for name in data["literalImportClosure"]:
    tree = ast.parse((ROOT / name).read_text(encoding="utf-8"))
    assignments = []
    for statement in tree.body:
        if isinstance(statement, (ast.Assign, ast.AnnAssign)):
            targets = statement.targets if isinstance(statement, ast.Assign) else [statement.target]
            for target in targets:
                if isinstance(target, ast.Name):
                    assignments.append(target.id)
    print(name, assignments)
