"""Test-name multiset of a crate tree: <module path>::<fn> for every #[test]-like fn reachable from a lib root through #[path] mounts.
usage: testnames.py <lib root abs> [--cfg-test-only]  -> lines on stdout"""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import modtree
from stubgen import mask

TEST = re.compile(r"#\[\s*(?:[\w]+::)*(?:async_)?test(?:_case)?\s*(?:\([^\]]*\))?\s*\]((?:\s*#\[[^\]]*\])*)\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)")
MACRO_TEST = re.compile(r"#\[\s*[\w:]*(?:test)[\w:]*\s*(?:\([^\]]*\))?\s*\]")


def names(lib, strip_prefix=()):
    out = []
    files = [((), lib)] + [(mp, f) for (mp, f, d, parent) in modtree.tree(lib) if f and f != "MISSING-PATH" and os.path.exists(f)]
    for mp, f in files:
        t = open(f, encoding="utf-8").read()
        m = mask(t)
        # inline module stack by brace scan
        stack = []
        depth = 0
        events = []
        for mm in re.finditer(r"[{}]", m):
            events.append((mm.start(), mm.group(0), None))
        for mm in re.finditer(r"(?<![\w])mod\s+(\w+)\s*\{", m):
            events.append((mm.start(), "mod", mm.group(1)))
        for mm in TEST.finditer(m):
            events.append((mm.start(), "test", mm.group(2)))
        events.sort(key=lambda e: (e[0], 0 if e[1] == "mod" else 1))
        pending_mod = None
        for pos, kind, val in events:
            if kind == "mod":
                pending_mod = val
            elif kind == "{":
                depth += 1
                if pending_mod is not None:
                    stack.append((pending_mod, depth))
                    pending_mod = None
            elif kind == "}":
                if stack and stack[-1][1] == depth:
                    stack.pop()
                depth -= 1
            else:
                path = list(mp) + [s[0] for s in stack] + [val]
                if tuple(path[:len(strip_prefix)]) == tuple(strip_prefix):
                    path = path[len(strip_prefix):]
                out.append("::".join(path))
    return out


if __name__ == "__main__":
    lib = os.path.abspath(sys.argv[1])
    strip = tuple(sys.argv[2].split("::")) if len(sys.argv) > 2 and sys.argv[2] else ()
    for n in names(lib, strip):
        print(n)
