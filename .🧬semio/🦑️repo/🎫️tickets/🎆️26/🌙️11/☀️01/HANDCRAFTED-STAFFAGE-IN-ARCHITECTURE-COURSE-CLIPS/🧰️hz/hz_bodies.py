"""🔎 Print function and method bodies with file:line for a list of ``file::name`` targets."""

import ast
import sys
from pathlib import Path

ARCH = Path("C:/git/semio/🎓️teaching/🏛️architecture")
MAX = 60
for target in sys.argv[1:]:
    rel, _, name = target.partition("::")
    path = ARCH / rel
    tree = ast.parse(path.read_text(encoding="utf-8"))
    hits = [n for n in ast.walk(tree) if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef)) and n.name == name]
    if not hits:
        print(f"## {target}: NOT FOUND")
        continue
    for node in hits:
        lines = path.read_text(encoding="utf-8").splitlines()
        body = lines[node.lineno - 1: node.end_lineno]
        print(f"## {rel}:{node.lineno}-{node.end_lineno} {name}")
        for i, line in enumerate(body[:MAX], node.lineno):
            print(f"{i}: {line}")
        if len(body) > MAX:
            print(f"... ({len(body) - MAX} more lines)")
