import ast
import os
import sys
from pathlib import Path

ROOT = "🎓️teaching/🏛️architecture/⚡️energy"
import glob
for path in sorted(glob.glob(f"{ROOT}/*/🎬️clip/*/🐍️.py")):
    src = Path(path).read_bytes().decode("utf-8")
    tree = ast.parse(src)
    parents = {}
    for p in ast.walk(tree):
        for c in ast.iter_child_nodes(p):
            parents[c] = p
    lines = src.split("\n")
    edits = []
    for n in ast.walk(tree):
        if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id == "tr" and len(n.args) == 2 and n.lineno == n.end_lineno:
            p = parents.get(n)
            inside = False
            while p is not None:
                if isinstance(p, (ast.FunctionDef, ast.AsyncFunctionDef, ast.Lambda)):
                    inside = True
                    break
                p = parents.get(p)
            if inside:
                continue
            a, b = n.args
            ln = lines[n.lineno - 1].encode("utf-8")
            sa = ln[a.col_offset:a.end_col_offset].decode("utf-8")
            sb = ln[b.col_offset:b.end_col_offset].decode("utf-8")
            print(path.split("clip")[-1], n.lineno, sa, "=>", sb)
            edits.append((n.lineno - 1, n.col_offset, n.end_col_offset, sb))
    if edits and "apply" in sys.argv:
        for l, c0, c1, new in sorted(edits, reverse=True):
            b = lines[l].encode("utf-8")
            lines[l] = (b[:c0] + new.encode("utf-8") + b[c1:]).decode("utf-8")
        out = "\n".join(lines)
        ast.parse(out)
        tmp = path + ".lc2tmp"
        Path(tmp).write_bytes(out.encode("utf-8"))
        os.replace(tmp, path)
