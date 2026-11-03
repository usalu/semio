import ast
import os
import re
import runpy
import sys
from pathlib import Path


def func_name(node):
    f = node.func
    return f.id if isinstance(f, ast.Name) else f.attr if isinstance(f, ast.Attribute) else ""


def main(path, mapfile):
    mapping = runpy.run_path(mapfile)["MAP"]
    raw = Path(path).read_bytes().decode("utf-8")
    src = raw
    tree = ast.parse(src)
    parents = {}
    for p in ast.walk(tree):
        for c in ast.iter_child_nodes(p):
            parents[c] = p
    lines = src.split("\n")
    edits = []
    used = set()
    for n in ast.walk(tree):
        if not (isinstance(n, ast.Constant) and isinstance(n.value, str)) or n.lineno != n.end_lineno:
            continue
        key = f"{n.lineno}:{n.value}" if f"{n.lineno}:{n.value}" in mapping else n.value
        par = parents.get(n)
        if isinstance(par, ast.Subscript) or (isinstance(par, ast.Dict) and n in par.keys) or isinstance(par, ast.Compare):
            continue
        subc = {k[1:]: v for k, v in mapping.items() if k.startswith("^") and k[1:] in n.value}
        if key not in mapping and not subc:
            continue
        p = parents.get(n)
        skip = False
        in_fn = False
        while p is not None:
            if isinstance(p, (ast.FunctionDef, ast.AsyncFunctionDef, ast.Lambda)):
                in_fn = True
            if isinstance(p, ast.JoinedStr):
                skip = True
                break
            if isinstance(p, ast.Call) and func_name(p) == "tr":
                skip = True
                break
            p = parents.get(p)
        if not in_fn and not skip:
            print("MODULE-LEVEL skipped:", n.lineno, repr(n.value))
            continue
        if skip:
            continue
        line = lines[n.lineno - 1]
        b = line.encode("utf-8")
        orig = b[n.col_offset:n.end_col_offset].decode("utf-8")
        m = re.match(r"([rRbBuU]*)(['\"])", orig)
        prefix, quote = m.group(1), m.group(2)
        en = mapping[key] if key in mapping else None
        if en is None:
            body = orig[len(prefix) + 1:-1]
            for k, v in subc.items():
                body = body.replace(k, v)
                used.add("^" + k)
            en = body
        en = en.replace(chr(10), chr(92) + "n")
        edits.append((n.lineno - 1, n.col_offset, n.end_col_offset, f"tr({prefix}{quote}{en}{quote}, {orig})"))
        used.add(key) if key in mapping else None
    subs = {k[1:]: v for k, v in mapping.items() if k.startswith("~")}
    for n in ast.walk(tree):
        if not isinstance(n, ast.JoinedStr) or n.lineno != n.end_lineno or not subs:
            continue
        p = parents.get(n)
        skip = False
        while p is not None:
            if isinstance(p, (ast.JoinedStr,)) or (isinstance(p, ast.Call) and func_name(p) == "tr"):
                skip = True
                break
            p = parents.get(p)
        if skip:
            continue
        orig = lines[n.lineno - 1].encode("utf-8")[n.col_offset:n.end_col_offset].decode("utf-8")
        en = orig
        for k, v in subs.items():
            if k in en:
                en = en.replace(k, v)
                used.add("~" + k)
        if en != orig:
            edits.append((n.lineno - 1, n.col_offset, n.end_col_offset, f"tr({en}, {orig})"))
    for ln, c0, c1, new in sorted(edits, reverse=True):
        b = lines[ln].encode("utf-8")
        lines[ln] = (b[:c0] + new.encode("utf-8") + b[c1:]).decode("utf-8")
    out = "\n".join(lines)
    if edits and not re.search(r"^tr = ", out, re.M):
        out = re.sub(r"^(set_vo_language = .*)$", r"\1\ntr = _manim_module.tr", out, count=1, flags=re.M)
    ast.parse(out)
    tmp = path + ".lc2tmp"
    Path(tmp).write_bytes(out.encode("utf-8"))
    os.replace(tmp, path)
    print(f"applied {len(edits)}; unused keys: {[k for k in mapping if k not in used]}")


main(sys.argv[1], sys.argv[2])
