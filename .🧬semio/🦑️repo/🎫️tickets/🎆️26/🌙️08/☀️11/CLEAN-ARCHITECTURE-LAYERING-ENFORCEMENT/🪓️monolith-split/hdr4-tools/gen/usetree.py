import re

def top_statements(rt):
    """All depth-0 `use`/`extern crate` statements of a Rust file as (attrs, text); attributes directly above are attached."""
    out = []
    i = 0
    n = len(rt)
    attrs = []
    depth = 0
    while i < n:
        j = rt.find("\n", i)
        j = n if j < 0 else j
        line = rt[i:j]
        s = line.strip()
        if depth == 0 and (re.match(r"(pub(\([^)]*\))?\s+)?use\b", s) or s.startswith("extern crate ")):
            d2 = 0; k = i
            while k < n:
                c = rt[k]
                if c == "{": d2 += 1
                elif c == "}": d2 -= 1
                elif c == ";" and d2 == 0: break
                k += 1
            out.append((attrs, rt[i:k + 1].strip()))
            attrs = []
            i = k + 1
            while i < n and rt[i] != "\n": i += 1
            i += 1
            continue
        if depth == 0 and s.startswith("#[") and not s.startswith("#!["):
            attrs.append(s)
            i = j + 1
            continue
        stripped = re.sub(r'"(?:[^"\\]|\\.)*"', '""', s)
        stripped = re.sub(r"//.*", "", stripped)
        depth += stripped.count("{") - stripped.count("}")
        if s and not s.startswith("//") and depth == 0 and not s.startswith("#["):
            attrs = []
        i = j + 1
    return out

def _split_top(inner):
    out, depth, cur = [], 0, ""
    for ch in inner:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0:
            out.append(cur.strip()); cur = ""
        else: cur += ch
    if cur.strip(): out.append(cur.strip())
    return out

def _flat(tree, prefix):
    tree = tree.strip()
    m = re.match(r"^(.*?)::\{(.*)\}$", tree, re.S)
    if m and "{" not in m.group(1):
        base = (prefix + "::" if prefix else "") + m.group(1) if m.group(1) else prefix
        res = []
        for it in _split_top(m.group(2)):
            res += _flat(it, base)
        return res
    if tree.startswith("{") and tree.endswith("}"):
        res = []
        for it in _split_top(tree[1:-1]):
            res += _flat(it, prefix)
        return res
    am = re.match(r"^(.*?)\s+as\s+(\w+)$", tree)
    path, alias = (am.group(1).strip(), am.group(2)) if am else (tree, None)
    if path == "self":
        full = prefix
    else:
        full = (prefix + "::" + path) if prefix else path
    return [(full, alias)]

def flatten(stmt):
    """stmt text -> (vis, leaves[(path, alias)]) for `use`, or None for extern crate."""
    m = re.match(r"^((?:pub(?:\([^)]*\))?\s+)?)use\s+(.*);$", stmt.strip(), re.S)
    if not m: return None
    return m.group(1).strip(), _flat(re.sub(r"\s+", " ", m.group(2)), "")

def binding(leaf):
    path, alias = leaf
    if alias: return alias
    if path.endswith("::*"): return "*"
    return path.split("::")[-1]

def prune(stmts, text, keep_globs=True, own=(), skip_first=()):
    """Returns the pruned use lines (grouped by parent path); leaves are kept when their binding appears in `text` (or are `_`/glob)."""
    lines = []
    for attrs, st in stmts:
        if st.startswith("extern crate "): continue
        fl = flatten(st)
        if fl is None: continue
        vis, leaves = fl
        kept = []
        for lf in leaves:
            if lf[0].split("::")[0] in skip_first: continue
            b = binding(lf)
            if b == "*":
                if keep_globs and not lf[0].startswith("super") and lf[0] != "self::*": kept.append(lf)
                continue
            if b == "_" or (re.search(r"\b%s\b" % re.escape(b), text) and b not in own):
                kept.append(lf)
        if not kept: continue
        attr = "".join(a + " " for a in attrs if not a.startswith("#[path"))
        groups = {}
        for p, a in kept:
            parent, _, last = p.rpartition("::")
            groups.setdefault(parent, []).append((last, a))
        for parent, items in groups.items():
            def one(li):
                last, a = li
                return last + (" as " + a if a else "")
            if len(items) == 1 and parent:
                lines.append("%suse %s::%s;" % (attr, parent, one(items[0])))
            elif not parent:
                for it in items: lines.append("%suse %s;" % (attr, one(it)))
            else:
                lines.append("%suse %s::{%s};" % (attr, parent, ", ".join(one(it) for it in items)))
    return lines

def externs(stmts, text):
    out = []
    for attrs, st in stmts:
        m = re.match(r"extern crate (\w+)(?: as (\w+))?;", st)
        if m and m.group(1) != "self":
            nm = m.group(2) or m.group(1)
            if re.search(r"\b%s\b" % re.escape(nm), text):
                out.append("".join(a + "\n" for a in attrs if not a.startswith("#[path")) + st)
    return out
