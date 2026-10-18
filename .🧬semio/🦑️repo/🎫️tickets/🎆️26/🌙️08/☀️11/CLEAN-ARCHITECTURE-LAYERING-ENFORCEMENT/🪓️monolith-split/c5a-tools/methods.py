"""📐 C5A method boundary lister: for an `impl` block of a snapshot, prints each depth-1 method with the line where its doc/attribute block starts and where it ends. usage: methods.py <tag> <first-line> <last-line>"""
import re, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
from rslex import lex
tag, a, b = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
t = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read()
ls = t.split("\n")
d, lit = lex(t)
heads = []
for i in range(a, b):
    l = ls[i]
    if d[i] == 1 and not lit[i] and re.match(r"^    (pub(\([a-z]+\)\s)?\s*)?(async\s+)?(unsafe\s+)?(const\s+)?fn\s+\w+", l):
        s = i
        while s - 1 >= a and (ls[s - 1].strip().startswith(("///", "#[", "//")) and d[s - 1] == 1):
            s -= 1
        heads.append((s + 1, i + 1, re.search(r"fn\s+(\w+)", l).group(1)))
for k, (s, i, n) in enumerate(heads):
    e = (heads[k + 1][0] - 1) if k + 1 < len(heads) else b - 1
    print(f"{s:5}-{e:<5} {e - s + 1:4} {n}")
