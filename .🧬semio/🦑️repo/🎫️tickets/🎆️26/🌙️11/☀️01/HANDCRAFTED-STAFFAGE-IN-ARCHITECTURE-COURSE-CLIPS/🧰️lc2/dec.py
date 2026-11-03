import ast
import glob
import os
import re
import sys
from pathlib import Path

ROOT = "🎓️teaching/🏛️architecture/⚡️energy"
SKIP = ("summer-strategy", "🏗️authoring", "🧪️tests")
MATH = re.compile(r"(?<=\d)\{,\}(?=\d)")
FMATH = re.compile(r"(?<=\d)\{\{,\}\}(?=\d)")
LABELS = {"Text", "body_text", "section_heading", "chip", "_label", "math_label", "math_text", "MathTex", "_handover", "_math_chip", "legend"}


def func_name(node):
    f = node.func
    return f.id if isinstance(f, ast.Name) else f.attr if isinstance(f, ast.Attribute) else ""


def patch(path, only):
    src = Path(path).read_bytes().decode("utf-8")
    tree = ast.parse(src)
    parents = {}
    for p in ast.walk(tree):
        for c in ast.iter_child_nodes(p):
            parents[c] = p
    lines = src.split("\n")
    edits = []
    for n in ast.walk(tree):
        if not isinstance(n, (ast.Constant, ast.JoinedStr)) or n.lineno != n.end_lineno:
            continue
        if isinstance(n, ast.Constant) and not isinstance(n.value, str):
            continue
        p = parents.get(n)
        skip = False
        in_label = False
        while p is not None:
            if isinstance(p, ast.JoinedStr):
                skip = True
                break
            if isinstance(p, ast.Call):
                nm = func_name(p)
                if nm == "tr":
                    skip = True
                    break
                if nm in LABELS:
                    in_label = True
            p = parents.get(p)
        if skip:
            continue
        b = lines[n.lineno - 1].encode("utf-8")
        orig = b[n.col_offset:n.end_col_offset].decode("utf-8")
        if isinstance(n, ast.JoinedStr):
            if not FMATH.search(orig):
                continue
            en = FMATH.sub(".", orig)
        else:
            if not re.match(r"[rRfF]*['\"]", orig) or orig.startswith(('"""', "'''")):
                continue
            if MATH.search(orig):
                en = MATH.sub(".", orig)
            elif in_label and re.search(r"(?<=\d),(?=\d)", orig):
                en = re.sub(r"(?<=\d),(?=\d)", ".", orig)
            else:
                continue
        edits.append((n.lineno - 1, n.col_offset, n.end_col_offset, f"tr({en}, {orig})"))
    for ln, c0, c1, new in sorted(edits, reverse=True):
        b = lines[ln].encode("utf-8")
        lines[ln] = (b[:c0] + new.encode("utf-8") + b[c1:]).decode("utf-8")
    out = "\n".join(lines)
    if edits and not re.search(r"^tr = ", out, re.M):
        out = re.sub(r"^(set_vo_language = .*)$", r"\1\ntr = _manim_module.tr", out, count=1, flags=re.M)
    ast.parse(out)
    if not only:
        tmp = path + ".lc2tmp"
        Path(tmp).write_bytes(out.encode("utf-8"))
        os.replace(tmp, path)
    print(len(edits), path.split("clip")[-1])


def main():
    dry = len(sys.argv) > 1 and sys.argv[1] == "dry"
    targets = [a for a in sys.argv[1:] if a != "dry"]
    for path in sorted(glob.glob(f"{ROOT}/*/🎬️clip/*/🐍️.py")):
        if any(s in path for s in SKIP):
            continue
        if targets and not any(t in path for t in targets):
            continue
        patch(path, dry)


main()
