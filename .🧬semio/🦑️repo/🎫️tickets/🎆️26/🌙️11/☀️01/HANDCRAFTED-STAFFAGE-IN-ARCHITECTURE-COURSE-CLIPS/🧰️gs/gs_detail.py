import ast
import sys
from pathlib import Path

PRIM = {"Circle", "Rectangle", "RoundedRectangle", "Square", "Polygon", "Ellipse", "Dot", "Line", "Arc", "SurroundingRectangle", "RegularPolygon", "Triangle", "Annulus"}
path, start = Path(sys.argv[1]), int(sys.argv[2])
src = path.read_text(encoding="utf-8")
lines = src.splitlines()
tree = ast.parse(src)
for fn in ast.walk(tree):
    if isinstance(fn, ast.FunctionDef) and fn.lineno == start:
        seen = set()
        for node in ast.walk(fn):
            if isinstance(node, ast.Call) and (getattr(node.func, "id", None) or getattr(node.func, "attr", None)) in PRIM and node.lineno not in seen:
                seen.add(node.lineno)
                print(node.lineno, lines[node.lineno - 1].strip()[:150])
