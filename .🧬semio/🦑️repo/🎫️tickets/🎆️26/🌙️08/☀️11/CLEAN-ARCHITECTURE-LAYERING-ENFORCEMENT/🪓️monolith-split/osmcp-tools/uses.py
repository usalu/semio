import re
from tree import *

IDENT = re.compile(r"[A-Za-z_]\w*")
ALIAS = re.compile(r"as\s+([A-Za-z_]\w*)")

def parse_use_tree(s, i=0):
    """Return ([(path_tuple, alias_or_None)], new_index). Globs end with '*'."""
    n = len(s)
    def skip(i):
        while i < n and s[i].isspace(): i += 1
        return i
    def tree(i, prefix):
        res = []
        i = skip(i)
        segs = list(prefix)
        if s.startswith("::", i): i += 2
        while True:
            i = skip(i)
            if i < n and s[i] == "{":
                i += 1
                while True:
                    i = skip(i)
                    if i < n and s[i] == "}": i += 1; break
                    before = i
                    sub, i = tree(i, segs)
                    res += sub
                    i = skip(i)
                    if i == before: return res, n
                    if i < n and s[i] == ",": i += 1; continue
                    if i < n and s[i] == "}": i += 1; break
                    break
                return res, i
            if i < n and s[i] == "*":
                i += 1; res.append((tuple(segs + ["*"]), None)); return res, i
            m = IDENT.match(s, i)
            if not m: return res, i
            segs.append(m.group(0)); i = m.end()
            i = skip(i)
            if s.startswith("::", i):
                i += 2; continue
            m2 = ALIAS.match(s, i)
            alias = None
            if m2: alias = m2.group(1); i = m2.end()
            res.append((tuple(segs), alias)); return res, i
    return tree(i, [])

USE_RE = re.compile(r'(?<![\w])((?:pub(?:\([^)]*\))?\s+)?)use\s+', re.S)

def file_uses_regex(f):
    """(vis, path, alias, offset, cfgtext) for each flattened use entry."""
    res = []
    code = f.code
    for m in USE_RE.finditer(code):
        start = m.end()
        paths, j = parse_use_tree(code, start)
        cfg = ""
        # attributes immediately before
        pre = attrs_before(f.src, m.start())
        for p, alias in paths:
            res.append((m.group(1).strip(), p, alias, m.start(), pre))
    return res

def file_uses(f, items=None):
    """(vis, path, alias, offset, cfgtext) for each flattened TOP-LEVEL use entry of file f."""
    from items import split_items
    res = []
    its = items if items is not None else split_items(f)
    for it in its:
        if it.kind != "use": continue
        code = f.code
        m = USE_RE.match(code, it.start) or USE_RE.search(code, it.start, it.end)
        if not m: continue
        paths, j = parse_use_tree(code, m.end())
        pre = " ".join(it.cfg)
        for p, alias in paths:
            res.append((m.group(1).strip(), p, alias, m.start(), pre))
    return res
