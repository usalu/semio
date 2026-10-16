"""Module tree of a Rust crate via #[path] mounts. Read-only. Usage: modtree.py <lib root abs path> [--tests]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
from stubgen import mask

ATTR = re.compile(r'#\s*!?\[((?:[^\[\]]|\[[^\]]*\])*)\]')
MOD = re.compile(r'(?:^|[;{}\s])((?:pub(?:\([^)]*\))?\s+)?)mod\s+([A-Za-z_]\w*)\s*([;{])')


def mounts(path, text=None):
    """Yield dict(name, vis, attrs, path_attr, inline, line, pos) of every `mod` declaration in a file, with inline nesting stack."""
    if text is None:
        text = open(path, encoding="utf-8").read()
    m = mask(text)
    ATTR_SPANS = [(x.start(), x.end()) for x in ATTR.finditer(m)]
    out = []
    stack = []
    depth = 0
    i = 0
    n = len(m)
    # walk: track braces; at each brace depth, detect mod decls
    for mm in MOD.finditer(m):
        pass
    # simple approach: iterate tokens of interest in order
    events = []
    for mm in re.finditer(r'[{}]', m):
        events.append((mm.start(), mm.group(0), None))
    for mm in MOD.finditer(m):
        events.append((mm.start(2), "mod", mm))
    events.sort(key=lambda e: e[0])
    depth = 0
    inline_stack = []  # (name, depth_of_open)
    for pos, kind, mm in events:
        if kind == "{":
            depth += 1
        elif kind == "}":
            depth -= 1
            while inline_stack and depth <= inline_stack[-1][1]:
                inline_stack.pop()
        else:
            name = mm.group(2)
            term = mm.group(3)
            head = mm.start(1)
            attrs = []
            cur = head
            while True:
                prev = [sp for sp in ATTR_SPANS if sp[1] <= cur and not text[sp[1]:cur].strip()]
                if not prev:
                    break
                sp = max(prev, key=lambda x: x[1])
                attrs.append(text[sp[0]:sp[1]])
                cur = sp[0]
            attrs.reverse()
            pa = None
            for a in attrs:
                x = re.search(r'path\s*=\s*"([^"]*)"', a)
                if x and a.replace(" ", "").startswith("#[path"):
                    pa = x.group(1)
            line = text.count("\n", 0, pos) + 1
            out.append(dict(name=name, vis=mm.group(1).strip(), attrs=attrs, path_attr=pa, inline=(term == "{"), line=line,
                            parent_inline=[s[0] for s in inline_stack], parent_dirs=[s[2] for s in inline_stack]))
            if term == "{":
                inline_stack.append((name, depth, pa if pa is not None else name))
    return out


def tree(lib, test=False, seen=None, prefix=(), cfgs=None):
    """Return list of (modpath tuple, file, decl) in DFS order for out-of-line modules (inline ones are listed with file=None)."""
    res = []
    seen = seen if seen is not None else set()
    if lib in seen:
        return res
    seen.add(lib)
    base = os.path.dirname(lib)
    for d in mounts(lib):
        mp = prefix + tuple(d["parent_inline"]) + (d["name"],)
        if d["inline"]:
            res.append((mp, None, d, lib))
            continue
        if d["path_attr"] is None:
            res.append((mp, "MISSING-PATH", d, lib))
            continue
        sub = os.path.normpath(os.path.join(base, *d["parent_dirs"], d["path_attr"]))
        res.append((mp, sub, d, lib))
        if os.path.exists(sub):
            res.extend(tree(sub, test, seen, mp))
    return res


if __name__ == "__main__":
    lib = os.path.abspath(sys.argv[1])
    root = "/Users/ueli/Documents/semio/"
    for mp, f, d, parent in tree(lib):
        cfg = [a for a in d["attrs"] if "cfg" in a]
        print("::".join(mp), "|", (f or "-").replace(root, "").replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "M/")[-120:], "|", d["vis"], "|", " ".join(c.replace(" ", "") for c in cfg), "|", os.path.basename(os.path.dirname(parent)))
