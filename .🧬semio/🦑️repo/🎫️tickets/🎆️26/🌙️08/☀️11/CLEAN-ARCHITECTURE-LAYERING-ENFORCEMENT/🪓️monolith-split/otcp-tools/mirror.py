"""Pruned host imports for a mirror module. usage: mirror.py <host.rs> <test.rs>..."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
from defs import scan
from splitter import tokenize
from stubgen import mask


def split_top(s):
    parts, cur, d = [], "", 0
    for ch in s:
        if ch == "{": d += 1
        if ch == "}": d -= 1
        if ch == "," and d == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def flatten(tree, prefix=()):
    tree = tree.strip()
    if tree.startswith("{") and tree.endswith("}"):
        out = []
        for p in split_top(tree[1:-1]):
            out += flatten(p, prefix)
        return out
    m = re.match(r"^(.*?)::\s*(\{.*\})$", tree, re.S)
    if m:
        return flatten(m.group(2), prefix + tuple(s.strip() for s in m.group(1).split("::") if s.strip()))
    alias = None
    am = re.match(r"^(.*?)\s+as\s+(\w+)$", tree, re.S)
    if am:
        tree, alias = am.group(1).strip(), am.group(2)
    segs = tuple(s.strip() for s in tree.split("::") if s.strip())
    if segs and segs[-1] == "self":
        segs = segs[:-1]
    full = prefix + segs
    if not full:
        return []
    return [(full, alias or full[-1])]


def host_imports(host_text):
    s = scan(host_text)
    out = []
    for imp in s["imports"]:
        if imp["vis"]:
            continue
        txt = imp["text"]
        for full, name in flatten(txt):
            out.append((full, name, imp["attrs"]))
    return out


def group(items):
    """items [(full, name)] -> use lines grouped by parent path."""
    by = {}
    for full, name in items:
        by.setdefault(full[:-1], []).append((full[-1], name))
    lines = []
    for parent, leaves in by.items():
        def leaf(l, n):
            return l if l == n else f"{l} as {n}"
        if len(leaves) == 1:
            lines.append("use " + "::".join(parent + (leaf(*leaves[0]),)) + ";")
        else:
            lines.append("use " + "::".join(parent) + "::{" + ", ".join(leaf(*l) for l in leaves) + "};")
    return lines


if __name__ == "__main__":
    host = open(sys.argv[1], encoding="utf-8").read()
    used = set()
    for t in sys.argv[2:]:
        first, allt = tokenize(open(t, encoding="utf-8").read())
        used |= allt
    imps = host_imports(host)
    keep = [(f, n) for (f, n, a) in imps if n in used]
    print("\n".join(group(keep)))
