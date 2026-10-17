"""Test-name multiset: (file relative to the renderer engine folder, inline module path, fn name) of every #[test]-like fn under the engine folder.
usage: testnames.py snapshot|live [--out file]  |  testnames.py compare"""
import collections, json, os, re, sys
from lib import *

TEST = re.compile(r"#\[\s*(?:[\w]+::)*(?:async_)?test(?:_case)?\s*(?:\([^\]]*\))?\s*\]((?:\s*#\[[^\]]*\])*)\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)")


def names_of(text):
    m = blank(text)
    events = [(mm.start(), mm.group(0), None) for mm in re.finditer(r"[{}]", m)]
    events += [(mm.start(), "mod", mm.group(1)) for mm in re.finditer(r"(?<![\w])mod\s+(\w+)\s*\{", m)]
    events += [(mm.start(), "test", mm.group(2)) for mm in TEST.finditer(m)]
    events.sort(key=lambda e: (e[0], 0 if e[1] == "mod" else 1))
    stack, depth, pending, out = [], 0, None, []
    for pos, kind, val in events:
        if kind == "mod":
            pending = val
        elif kind == "{":
            depth += 1
            if pending is not None:
                stack.append((pending, depth))
                pending = None
        elif kind == "}":
            if stack and stack[-1][1] == depth:
                stack.pop()
            depth -= 1
        else:
            out.append(("::".join([s[0] for s in stack] + [val])))
    return out


def scan(root):
    res = collections.Counter()
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated")]
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                try:
                    t = open(p, encoding="utf8").read()
                except Exception:
                    continue
                if "test" not in t:
                    continue
                for n in names_of(t):
                    res[(os.path.relpath(p, root), n)] += 1
    return res


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "compare":
        a = scan(SNAP + "/E")
        b = scan(E)
        only_a, only_b = a - b, b - a
        print("baseline tests", sum(a.values()), "files", len({k[0] for k in a}), "| live tests", sum(b.values()), "files", len({k[0] for k in b}))
        print("only baseline:", sum(only_a.values()), "only live:", sum(only_b.values()))
        for k, v in sorted(only_a.items())[:30]:
            print("  -", k)
        for k, v in sorted(only_b.items())[:30]:
            print("  +", k)
        na, nb = collections.Counter(), collections.Counter()
        for k, v in a.items():
            na[k[1]] += v
        for k, v in b.items():
            nb[k[1]] += v
        print("by name (files ignored): only baseline", sum((na - nb).values()), "only live", sum((nb - na).values()))
        for k in sorted((nb - na)):
            print("  + name", k)
        for k in sorted((na - nb)):
            print("  - name", k)
        print("IDENTICAL" if not only_a and not only_b else "DIFFERENT (files)")
