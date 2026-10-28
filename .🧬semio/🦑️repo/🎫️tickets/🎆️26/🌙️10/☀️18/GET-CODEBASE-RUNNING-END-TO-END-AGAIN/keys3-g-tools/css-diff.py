#!/usr/bin/env python3
"""keys3-g: compares two Tailwind outputs by top-level rule (at-rule prelude + selector): prints rules only in A, only in B, and rules whose body differs. Streams nothing large: both files are ~300 KB. usage: css-diff.py A.css B.css"""
import re, sys
def rules(path):
    text = open(path, encoding="utf8").read()
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    out, stack, buf, i = {}, [], "", 0
    def key(stack, selector): return " >> ".join(stack + [selector.strip()])
    depth = 0; cur = ""; stack = []
    for ch in text:
        if ch == "{":
            stack.append(cur.strip()); cur = ""; body_start = True
        elif ch == "}":
            if cur.strip():
                out.setdefault(" >> ".join(stack), []).append(cur.strip())
            cur = ""; stack.pop() if stack else None
        elif ch == ";":
            if stack:
                out.setdefault(" >> ".join(stack), []).append(cur.strip())
            cur = ""
        else:
            cur += ch
    return {k: "\n".join(v) for k, v in out.items()}
a, b = rules(sys.argv[1]), rules(sys.argv[2])
only_a = sorted(set(a) - set(b)); only_b = sorted(set(b) - set(a)); changed = sorted(k for k in set(a) & set(b) if a[k] != b[k])
print(f"rules A {len(a)} B {len(b)}; only in A {len(only_a)}; only in B {len(only_b)}; body differs {len(changed)}")
for title, rows in (("ONLY-A", only_a), ("ONLY-B", only_b), ("DIFF", changed)):
    for row in rows: print(title, row[:300])
