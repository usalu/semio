#!/usr/bin/env python3
"""🔬️ Scan: `pub(super)`, `pub(in super::..)` and `super::..` paths that climb above the root of the crate they now live in (invalid in a real crate; the union oracle maps them into the facade where they still resolve).
usage: python3 supercheck.py [crate ...]"""
import re, sys, os
from lib import *
from rs import blank
import uo, tree as T

VIS = re.compile(r"pub\s*\(\s*(?:in\s+)?((?:super)(?:\s*::\s*super)*)")
PATH = re.compile(r"(?<![A-Za-z0-9_:])((?:super\s*::\s*)+)")
MODOPEN = re.compile(r"\bmod\s+[A-Za-z_]\w*\s*\{")


def depths(root):
    d = {}
    for f, parent, name in T.tree(root):
        d[f] = 0 if parent is None else d[parent] + (0 if name == "<include>" else 1)
    return d


def inline_depth(clean, pos):
    stack, depth = [], 0
    opens = {m.end() - 1 for m in MODOPEN.finditer(clean)}
    d = 0
    for i, c in enumerate(clean[:pos]):
        if c == "{":
            stack.append(i in opens)
        elif c == "}" and stack:
            stack.pop()
    return sum(1 for s in stack if s)


def main(only):
    reals = uo.real_crates()
    for K in sorted(set(uo.flipped_by_me()) & set(reals)):
        if only and K not in only:
            continue
        dep = depths(reals[K])
        for f, fd in dep.items():
            t = read(f)
            c = blank(t)
            for rx in (VIS, PATH):
                for m in rx.finditer(c):
                    n = len(re.findall(r"super", m.group(1)))
                    eff = fd + inline_depth(c, m.start())
                    if n > eff:
                        print(K, rel(f), c.count("\n", 0, m.start()) + 1, m.group(0).strip()[:40], "needs", n, "has", eff)


if __name__ == "__main__":
    main(set(sys.argv[1:]))
