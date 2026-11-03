import ast
import json
import os
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio")
ARCH = ROOT / "🎓️teaching" / "🏛️architecture"
MANIFEST = ARCH / "⚡️energy" / "🏗️authoring" / "🎨️visuals" / "🧍️staffage" / "🔣️.json"
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
BASE = {"staffage", "staffage_motion", "fitted_staffage", "pendant_lamp"}
HELPERS = {"_figure", "_cast", "_person", "_use_cast", "_on_surface", "_stand", "_clerk"}
NAMES = BASE | {"_" + n for n in BASE} | HELPERS

casts = json.loads(MANIFEST.read_text(encoding="utf-8"))["casts"]


def py_files():
    for base, dirs, files in os.walk(ARCH):
        dirs[:] = [d for d in dirs if d not in {"node_modules", "__pycache__", ".git"} and "🗑️" not in d]
        for f in files:
            if f.endswith(".py"):
                yield Path(base) / f


def consts(tree):
    env = {}
    for node in tree.body:
        if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
            try:
                env[node.targets[0].id] = eval(compile(ast.Expression(node.value), "x", "eval"), {"__builtins__": {}}, dict(env))
            except Exception:
                pass
    return env


rows = []
for path in py_files():
    try:
        text = path.read_text(encoding="utf-8")
        tree = ast.parse(text)
    except Exception:
        continue
    if not any(n in text for n in NAMES):
        continue
    env = consts(tree)
    rel = path.relative_to(ARCH).as_posix()
    for node in ast.walk(tree):
        if isinstance(node, ast.Call):
            fn = node.func
            name = fn.id if isinstance(fn, ast.Name) else fn.attr if isinstance(fn, ast.Attribute) else None
            if name not in NAMES:
                continue
            if "staffage/" in rel and "🧪" not in rel and rel.endswith("🧍️staffage/🐍️.py"):
                continue
            cast = ast.unparse(node.args[0]) if node.args else "?"
            kw = {k.arg: ast.unparse(k.value) for k in node.keywords if k.arg}
            def ev(expr):
                if expr is None:
                    return None
                try:
                    return round(float(eval(expr, {"__builtins__": {}}, dict(env))), 4)
                except Exception:
                    return None
            lit = cast.strip("'\"") if cast[:1] in "'\"" else None
            if lit is None and "_CAST +" in cast:
                lit = None
            if lit is None:
                for a in node.args[:2]:
                    if isinstance(a, ast.Constant) and isinstance(a.value, str) and a.value in casts:
                        lit, cast = a.value, a.value
                        break
            entry = casts.get(lit) if lit else None
            rows.append({
                "file": rel, "line": node.lineno, "fn": name, "cast": cast, "lit": lit,
                "asset": entry and entry.get("asset"), "pose": entry and (entry.get("pose") or entry.get("clip")),
                "view": entry and entry.get("view"), "viewpoint": entry and entry.get("viewpoint", "elevation"),
                "env": entry and (entry.get("environment") or ("timeline" if entry.get("timeline") else None)),
                "mirror": entry and entry.get("mirror"),
                "unit": kw.get("unit"), "unit_v": ev(kw.get("unit")),
                "width": kw.get("width"), "height": kw.get("height"), "kw": kw,
                "test": "🧪" in rel, "pos": [ast.unparse(a) for a in node.args[1:]],
            })
rows.sort(key=lambda r: (r["file"], r["line"]))
(OUT / "xc_calls.json").write_text(json.dumps(rows, ensure_ascii=False, indent=1), encoding="utf-8")
used = {r["lit"] for r in rows if r["lit"]}
lines = ["file:line | fn | cast | asset/pose | view | vp | env | unit | unit_v | w | h"]
for r in rows:
    lines.append(f"{r['file'].replace('/🎬️clip','').replace('🐍️.py','')}:{r['line']} | {r['fn']} | {r['cast']} | {r['asset']}/{r['pose']} | {r['view']} | {r['viewpoint']} | {r['env']} | {r['unit']} | {r['unit_v']} | {r['width']} | {r["height"]} | pos={r["pos"]}{' | TEST' if r['test'] else ''}")
(OUT / "xc_table.txt").write_text("\n".join(lines), encoding="utf-8")
print(len(rows), "calls;", len(used), "distinct literal casts;", len(casts), "manifest casts")
print("unused manifest casts:", sorted(set(casts) - used))
