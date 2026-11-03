"""🎬 Count the `staffage_motion` calls each Scene class makes, following same-module helpers with their call-site arguments, and flag a cast id used more than once in one scene.

@see ../../🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py — the `staffage_motion` API
"""

import ast
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve()
W = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
ENERGY = W / "🎓️teaching/🏛️architecture/⚡️energy"
GENERATED = HERE.parent.parent / "🗑️generated" / "hp"
DEPTH = 8


def callee(node):
    """🔤 Name of a called function or method, or None."""
    f = node.func
    if isinstance(f, ast.Name):
        return f.id
    if isinstance(f, ast.Attribute):
        return f.attr
    return None


def params_of(fn, call):
    """📥 Parameter names a call binds, without `self` for a method call."""
    names = [a.arg for a in fn.args.args]
    if names[:1] == ["self"] and isinstance(call.func, ast.Attribute):
        names = names[1:]
    return names


def own_calls(fn):
    """🪜 Calls in a function body, not descending into nested functions, which are reached only when called."""
    stack = list(fn.body)
    while stack:
        node = stack.pop()
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef, ast.Lambda)):
            continue
        if isinstance(node, ast.Call):
            yield node
        stack.extend(ast.iter_child_nodes(node))


def resolve(source, fn, env, defs, depth, out, trail):
    """🧭 Collect the cast expressions of every motion reached from `fn`, substituting parameters by the caller's argument text."""
    if depth > DEPTH:
        return
    for sub in own_calls(fn):
        name = callee(sub)
        if name == "staffage_motion" and sub.args:
            arg = sub.args[0]
            text = env.get(arg.id, ast.get_source_segment(source, arg)) if isinstance(arg, ast.Name) else ast.get_source_segment(source, arg)
            out.append({"line": sub.lineno, "cast": text, "via": trail})
        elif name in defs:
            for target in defs[name]:
                names = params_of(target, sub)
                bound = {}
                for param, value in zip(names, sub.args):
                    bound[param] = env.get(value.id, ast.get_source_segment(source, value)) if isinstance(value, ast.Name) else ast.get_source_segment(source, value)
                for kw in sub.keywords:
                    if kw.arg:
                        bound[kw.arg] = env.get(kw.value.id, ast.get_source_segment(source, kw.value)) if isinstance(kw.value, ast.Name) else ast.get_source_segment(source, kw.value)
                resolve(source, target, bound, defs, depth + 1, out, trail + [name])


def analyse(path):
    """🧭 Per Scene class: the motion calls reached from its construct and helper methods."""
    source = path.read_text(encoding="utf-8")
    tree = ast.parse(source)
    defs = {}
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            defs.setdefault(node.name, []).append(node)
    scenes = [n for n in tree.body if isinstance(n, ast.ClassDef) and any((isinstance(b, ast.Name) and b.id.endswith("Scene")) or (isinstance(b, ast.Attribute) and b.attr.endswith("Scene")) for b in n.bases)]
    rows = []
    for scene in scenes:
        calls = []
        for method in [n for n in scene.body if isinstance(n, ast.FunctionDef)]:
            resolve(source, method, {}, defs, 0, calls, [method.name])
        if calls:
            casts = [c["cast"] for c in calls]
            dupes = sorted({c for c in casts if casts.count(c) > 1})
            rows.append({"file": path.relative_to(ENERGY).as_posix(), "scene": scene.name, "motions": len(calls), "casts": calls, "duplicate_cast_expr": dupes})
    return rows


def main():
    """📋 Scan every clip, visuals and staffage module and write the result."""
    files = sorted(p for p in ENERGY.rglob("*.py") if "node_modules" not in p.parts and ("🎬️clip" in p.parts or "🎨️visuals" in p.parts or "🧍️staffage" in p.parts))
    rows = []
    for path in files:
        if "staffage_motion" in path.read_text(encoding="utf-8"):
            rows.extend(analyse(path))
    (GENERATED / "motion-scan.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2), encoding="utf-8")
    for r in rows:
        flag = "  DUP " + " | ".join(r["duplicate_cast_expr"]) if r["duplicate_cast_expr"] else ""
        print(f"{r['file']} :: {r['scene']} motions={r['motions']}{flag}")
        for c in r["casts"]:
            print(f"    L{c['line']} {' > '.join(c['via'])}: {' '.join(c['cast'].split())}")
    print(f"scenes with motions: {len(rows)}, scenes with duplicate cast expressions: {sum(1 for r in rows if r['duplicate_cast_expr'])}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
