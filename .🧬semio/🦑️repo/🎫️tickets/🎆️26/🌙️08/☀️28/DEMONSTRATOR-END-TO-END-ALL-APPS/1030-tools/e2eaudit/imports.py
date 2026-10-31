import re, os, sys
ROOT = "/Users/ueli/Documents/semio"
def resolve(base, spec):
    p = spec if spec.startswith("/") else os.path.normpath(os.path.join(os.path.dirname(base), spec))
    for c in [p, p + ".ts", p + ".tsx", p + ".mjs", os.path.join(p, "index.ts")]:
        if os.path.isfile(c): return c
    return None
def exports(path, seen=None):
    seen = seen or set()
    if path in seen or not path: return set(), True
    seen.add(path)
    src = open(path, encoding="utf8").read()
    names = set()
    for m in re.finditer(r"export\s+(?:async\s+)?(?:function\*?|const|let|var|class|type|interface|enum)\s+([\w$]+)", src): names.add(m.group(1))
    for m in re.finditer(r"export\s*(?:type\s*)?\{([^}]*)\}\s*(?:from\s*['\"]([^'\"]+)['\"])?", src):
        body, frm = m.group(1), m.group(2)
        for part in body.split(","):
            part = part.strip()
            if not part: continue
            alias = part.split(" as ")[-1].strip()
            names.add(alias)
        if frm and frm.startswith("."):
            tgt = resolve(path, frm)
            if tgt:
                sub, _ = exports(tgt, seen)
                # only trust explicitly listed names; sub used for validation below
    for m in re.finditer(r"export\s*\*\s*from\s*['\"]([^'\"]+)['\"]", src):
        tgt = resolve(path, m.group(1))
        if tgt:
            sub, _ = exports(tgt, seen); names |= sub
        else: names.add("*UNRESOLVED:" + m.group(1))
    return names, True
def check(file):
    src = open(file, encoding="utf8").read()
    bad = []
    for m in re.finditer(r"import\s+(type\s+)?\{([^}]*)\}\s*from\s*['\"]([^'\"]+)['\"]", src, re.S):
        spec = m.group(3)
        if not (spec.startswith(".") or spec.startswith("/")): continue
        tgt = resolve(file, spec)
        if not tgt: bad.append(f"UNRESOLVED FILE {spec}"); continue
        names, _ = exports(tgt)
        for part in m.group(2).split(","):
            part = part.strip().replace("type ", "")
            if not part: continue
            sym = part.split(" as ")[0].strip()
            if sym not in names and not any(n.startswith("*UNRESOLVED") for n in names):
                bad.append(f"MISSING {sym} from {spec} ({os.path.relpath(tgt, ROOT)})")
    return bad
for f in sys.argv[1:]:
    print("==", os.path.relpath(f, ROOT))
    for b in check(f): print("  ", b)
