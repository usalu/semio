import ast
import json
import sys
from pathlib import Path

W = Path("C:/git/semio")
ROOT = W / "🎓️teaching"
CASES = json.loads((ROOT / "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️staffage_only.json").read_text(encoding="utf-8"))
RA = CASES["rule_a"]
PRIM = set(RA["primitives"])
STAFF = set(RA["staffage_calls"])
CONTEXT = {"Axes", "NumberPlane", "NumberLine", "Table", "MobjectTable", "BarChart", "MathTable", "IntegerTable", "DecimalTable", "Graph", "DiGraph", "ThreeDAxes", "plot", "get_graph", "get_axes", "c2p", "coords_to_point", "plot_line_graph", "get_x_axis", "get_y_axis"}
MIN = int(sys.argv[1]) if len(sys.argv) > 1 else 3


def call_name(call):
    return call.func.id if isinstance(call.func, ast.Name) else call.func.attr if isinstance(call.func, ast.Attribute) else None


def key(node):
    if isinstance(node, ast.Name):
        return node.id
    if isinstance(node, ast.Attribute):
        base = key(node.value)
        return f"{base}.{node.attr}" if base else None
    if isinstance(node, ast.Subscript):
        return key(node.value)
    return None


def feed(fn):
    parts = {}
    stack = list(fn.body)
    while stack:
        node = stack.pop()
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            continue
        stack.extend(ast.iter_child_nodes(node))
        if isinstance(node, ast.Assign):
            for t in node.targets:
                for e in (t.elts if isinstance(t, (ast.Tuple, ast.List)) else [t]):
                    if key(e):
                        parts.setdefault(key(e), []).append((node.value, node.lineno))
        elif isinstance(node, ast.AugAssign) and key(node.target):
            parts.setdefault(key(node.target), []).append((node.value, node.lineno))
        elif isinstance(node, ast.AnnAssign) and node.value is not None and key(node.target):
            parts.setdefault(key(node.target), []).append((node.value, node.lineno))
        elif isinstance(node, ast.Expr) and isinstance(node.value, ast.Call):
            c = node.value
            if isinstance(c.func, ast.Attribute) and c.func.attr in ("add", "append", "extend", "add_to_back", "insert") and key(c.func.value) and key(c.func.value) != "self":
                parts.setdefault(key(c.func.value), []).append((ast.Tuple(elts=[*c.args], ctx=ast.Load()), node.lineno))
    return parts


def facts(expr):
    calls = [n for n in ast.walk(expr) if isinstance(n, ast.Call)]
    names = {call_name(c) for c in calls}
    refs = {key(n) for n in ast.walk(expr) if isinstance(n, (ast.Name, ast.Attribute)) and key(n)}
    return {n for n in names if n in PRIM}, names, refs


def analyse(fn):
    parts = feed(fn)
    memo = {}

    def closure(var, seen):
        if var in memo:
            return memo[var]
        if var in seen:
            return set(), set(), 0
        seen = seen | {var}
        prims, names, line = set(), set(), 0
        for expr, ln in parts.get(var, []):
            p, n, refs = facts(expr)
            prims |= p
            names |= n
            line = line or ln
            for r in refs:
                if r in parts and r != var:
                    cp, cn, _ = closure(r, seen)
                    prims |= cp
                    names |= cn
        memo[var] = (prims, names, line)
        return memo[var]

    stack = list(fn.body)
    while stack:
        node = stack.pop()
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            continue
        stack.extend(ast.iter_child_nodes(node))
        if isinstance(node, ast.Call) and call_name(node) in ("VGroup", "Group"):
            parts.setdefault(f"<{call_name(node)}@{node.lineno}:{node.col_offset}>", []).append((node, node.lineno))
        elif isinstance(node, ast.Return) and node.value is not None:
            parts.setdefault(f"<return@{node.lineno}>", []).append((node.value, node.lineno))
        elif isinstance(node, ast.Expr) and isinstance(node.value, ast.Call) and call_name(node.value) in ("add", "add_to_back", "add_foreground_mobject") and key(node.value.func.value if isinstance(node.value.func, ast.Attribute) else node.value.func) in ("self", None):
            parts.setdefault(f"<add@{node.lineno}>", []).append((node.value, node.lineno))
    flagged = {}
    for var in parts:
        prims, names, line = closure(var, frozenset())
        if len(prims) >= MIN and not names & STAFF and not names & CONTEXT:
            flagged[var] = (prims, line)
    minimal = {}
    for var, (prims, line) in flagged.items():
        refs = set()
        for expr, _ in parts[var]:
            refs |= facts(expr)[2]
        if not any(r in flagged and r != var for r in refs):
            minimal[var] = (prims, line)
    return minimal


def main():
    hits = []
    files = [p for p in ROOT.rglob(CASES["file_name"]) if CASES["clip_folder"] in p.parts and not set(CASES["skip_folders"]) & set(p.parts) and "🏛️architecture" in p.parts]
    for p in sorted(files):
        tree = ast.parse(p.read_text(encoding="utf-8"))
        for fn in ast.walk(tree):
            if not isinstance(fn, (ast.FunctionDef, ast.AsyncFunctionDef)):
                continue
            for var, (prims, line) in analyse(fn).items():
                hits.append((p.relative_to(ROOT / "🏛️architecture").as_posix(), fn.name, var, line, sorted(prims)))
    for h in hits:
        print(f"{h[0]}::{h[1]}::{h[2]}:{h[3]} {','.join(h[4])}")
    print(len(hits))


main()
