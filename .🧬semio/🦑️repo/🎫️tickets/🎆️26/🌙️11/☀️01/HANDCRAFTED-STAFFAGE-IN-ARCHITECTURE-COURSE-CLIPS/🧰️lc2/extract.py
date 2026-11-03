import ast
import re
import sys
from pathlib import Path

LABEL_FUNCS = {"Text", "body_text", "section_heading", "chip", "_label", "math_label", "math_text", "MathTex", "Tex", "_din_ref", "math_readout", "_handover", "_tag", "_chip"}
GERMAN = re.compile(r"[äöüßÄÖÜ]|\b(und|der|die|das|mit|ohne|von|für|im|bei|nach|zu|ist|nicht|Wärme\w*|Luft\w*|Heiz\w*|Kühl\w*|Strom|Energie|Haus|Raum|Fenster|Wand|Dach|innen|außen|Innen|Außen|Person\w*|Start|Gerät\w*|Licht|Winter|Sommer|Sonne|Wasser|Zuluft|Abluft)\b")


def func_name(node):
    f = node.func
    return f.id if isinstance(f, ast.Name) else f.attr if isinstance(f, ast.Attribute) else ""


def main(path):
    src = Path(path).read_text(encoding="utf-8")
    tree = ast.parse(src)
    parents = {}
    for p in ast.walk(tree):
        for c in ast.iter_child_nodes(p):
            parents[c] = p
    doc = set()
    for n in ast.walk(tree):
        if isinstance(n, (ast.Module, ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)) and n.body and isinstance(n.body[0], ast.Expr) and isinstance(n.body[0].value, ast.Constant):
            doc.add(n.body[0].value)
    rows = []
    for n in ast.walk(tree):
        if not (isinstance(n, ast.Constant) and isinstance(n.value, str)) or n in doc:
            continue
        chain = []
        p = parents.get(n)
        skip = False
        label = ""
        while p is not None:
            if isinstance(p, ast.Call):
                nm = func_name(p)
                if nm == "tr":
                    skip = True
                    break
                if nm in LABEL_FUNCS and not label:
                    label = nm
            if isinstance(p, ast.Assign) and any(isinstance(t, ast.Name) and t.id in {"NARRATION", "TITLE_DE"} for t in p.targets):
                skip = True
                break
            if isinstance(p, ast.AnnAssign) and isinstance(p.target, ast.Name) and p.target.id in {"NARRATION"}:
                skip = True
                break
            p = parents.get(p)
        if skip:
            continue
        v = n.value
        if not re.search(r"[A-Za-zäöüÄÖÜ]{3,}", re.sub(r"\\[A-Za-z]+", "", v)):
            continue
        keep = GERMAN.search(v) or (label in {"Text","body_text","section_heading","chip","_label","_tag","_chip"} and not re.fullmatch(r"center|left|right", v)) or r"\text{" in v
        if not keep and len(sys.argv) > 2:
            par = parents.get(n)
            bad = isinstance(par, (ast.Subscript, ast.Compare)) or re.fullmatch(r"[a-z_0-9.\-:/#]+|[A-Z_0-9]+|#[0-9a-fA-F]+", v) or isinstance(par, ast.Index if hasattr(ast, "Index") else ast.Subscript)
            keep = not bad and not "🎓️" in v and not "🧰️" in v and not v.endswith((".json", ".py", ".svg", ".png"))
        if keep:
            rows.append((n.lineno, n.col_offset, label, v))
    lines = src.splitlines()
    for r in sorted(rows):
        print(f"{r[0]}\t{r[2] or '-'}\t{r[3]!r}\t| {lines[r[0] - 1].strip()[:150]}")


import glob
if sys.argv[1] == "lower":
    for f in sorted(glob.glob(sys.argv[2])):
        if "summer" in f or "🧪️" in f or "🏗️" in f:
            continue
        import io, contextlib
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            main(f)
        for ln in buf.getvalue().splitlines():
            c = ln.split("	")
            if c[1] in {"Text", "body_text", "chip", "section_heading", "_label", "_tag", "_chip"} and re.fullmatch(r"[a-zäöüß ·-]+", eval(c[2])):
                print(f.split("clip/")[-1][:14], ln[:170])
else:
    main(sys.argv[1])
