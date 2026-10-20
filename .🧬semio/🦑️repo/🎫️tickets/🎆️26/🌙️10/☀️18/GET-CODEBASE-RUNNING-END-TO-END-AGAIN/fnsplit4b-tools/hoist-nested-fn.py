#!/usr/bin/env python3
"""📤 Moves the depth-1 nested `fn` items of a Rust function out to module level (right after the function, dedented by four spaces).
usage: hoist-nested-fn.py <file> <fn> [nth] [APPLY]   Refuses when the outer body has depth-1 `use` or other items the nested fns could depend on."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn
file, fname = sys.argv[1], sys.argv[2]
nth = int(sys.argv[3]) if len(sys.argv) > 3 and sys.argv[3].isdigit() else 0
APPLY = "APPLY" in sys.argv
text = open(file, encoding="utf-8").read()
toks = rsfn.lex(text)
k, op, cl = rsfn.find_fn(toks, fname, nth)
sts = rsfn.statements(toks, op, cl)
heads = [toks[s][1] for s, e in sts]
ITEMS = ("const", "static", "struct", "enum", "type")
bad = [h for h in heads if h in ("use", "impl", "trait", "mod", "macro_rules") or (h in ITEMS and "ITEMS" not in sys.argv)]
nested = [(s, e) for (s, e) in sts if (toks[s][1] in ("fn", "pub", "async", "unsafe") and any(toks[q][1] == "fn" for q in range(s, min(s + 4, e)))) or ("ITEMS" in sys.argv and toks[s][1] in ITEMS)]
print("nested fns", len(nested), "other items", bad)
if bad: sys.exit(1)
lines = text.split("\n")
moved = []
kill = set()
for s, e in nested:
    a = toks[s][2]
    while a - 2 >= 0 and lines[a - 2].lstrip().startswith(("///", "#[")): a -= 1
    b = toks[e][2]
    moved.append("\n".join((l[4:] if l.startswith("    ") else l) for l in lines[a - 1:b]))
    kill.update(range(a, b + 1))
print("lines", sum(m.count("\n") + 1 for m in moved))
if APPLY:
    out = []
    end_line = toks[cl][2]
    for i, l in enumerate(lines, 1):
        if i in kill: continue
        out.append(l)
        if i == end_line:
            for m in moved:
                out.append("")
                out.extend(m.split("\n"))
    open(file, "w", encoding="utf-8").write("\n".join(out))
    print("APPLIED")
