"""🔎 EN 1991 oracle-depth scan: token rule over non-test oracle sources."""
import json, pathlib, re, sys
B = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any")
STD = set(sys.stdlib_module_names)
files = [p for p in B.rglob("*.py") if "oracle" in str(p.relative_to(B)) and "/tests/" not in str(p) and "🧪️tests" not in str(p)]
def third(p):
    mods = set(re.findall(r"^\s*(?:import|from)\s+([A-Za-z_][\w]*)", p.read_text(encoding="utf8"), re.M))
    return sorted(m for m in mods if m not in STD)
info = {p: (p.read_text(encoding="utf8"), third(p)) for p in files}
items = json.load(open(B / "🧭️spec-trace/🔣️.json"))["items"]
rows = []
for it in items:
    toks = [it["id"]] + [c for c in it["realisation"].get("checks", [])]
    hits = []
    for p, (txt, tp) in info.items():
        for t in toks:
            if re.search(r"(?<![\w.-])" + re.escape(t.rstrip(".")) + r"(?![\w-])", txt) or (t.endswith(".") and t in txt):
                hits.append((p, tp)); break
    cls = "none" if not hits else ("third-party" if any(tp for _, tp in hits) else "stdlib")
    libs = sorted({m for _, tp in hits for m in tp})
    rows.append((it["id"], it["clause"], ",".join(it["realisation"].get("checks", [])) or it["realisation"]["kind"], cls, ",".join(libs), len(hits)))
if __name__ == "__main__":
    print("| id | clause | check | class | libs |\n|---|---|---|---|---|")
    for r in rows: print("| %s | %s | %s | %s | %s |" % r[:5])
    from collections import Counter
    print(Counter(r[3] for r in rows), file=sys.stderr)


import ast
def deep():
    out = {}
    for p, (txt, tp) in info.items():
        if not tp: continue
        tree = ast.parse(txt)
        aliases = set()
        for n in ast.walk(tree):
            if isinstance(n, ast.Import):
                for a in n.names:
                    if a.name.split(".")[0] not in STD: aliases.add((a.asname or a.name).split(".")[0])
            elif isinstance(n, ast.ImportFrom) and n.module and n.module.split(".")[0] not in STD:
                for a in n.names: aliases.add(a.asname or a.name)
        funcs = {n.name: n for n in tree.body if isinstance(n, ast.FunctionDef)}
        memo = {}
        def uses(node, seen=()):
            for m in ast.walk(node):
                if isinstance(m, ast.Name) and m.id in aliases: return True
                if isinstance(m, ast.Name) and m.id in funcs and m.id not in seen:
                    if m.id not in memo: memo[m.id] = uses(funcs[m.id], seen + (m.id,))
                    if memo[m.id]: return True
            return False
        for n in ast.walk(tree):
            if isinstance(n, ast.If):
                seg = ast.get_source_segment(txt, n.test) or ""
                body_third = None
                for t in re.findall(r"E\d+\.\d+", seg):
                    if body_third is None: body_third = any(uses(s) for s in n.body)
                    out.setdefault(t, set()).add(body_third)
    return out
if __name__ == "__main__":
    d = deep()
    shallow = sorted(k for k, v in d.items() if not any(v))
    print("DEEP-STDLIB-ONLY", shallow, file=sys.stderr)
    print("DEEP-THIRD", sorted(k for k, v in d.items() if any(v)).__len__(), file=sys.stderr)
