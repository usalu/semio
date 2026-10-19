"""📐 C6UI nested method boundary lister: like methods.py for an `impl` inside an inline module. usage: methods_n.py <tag> <first-line> <last-line> <impl-indent> (line numbers of the ORIGINAL snapshot)"""
import re, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
tag, a, b, ind = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
ls = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read().split("\n")
pad = " " * (ind + 4)
heads = []
for i in range(a, b):
    l = ls[i]
    if re.match("^" + pad + r"(pub(\([a-z]+\)\s)?\s*)?(async\s+)?(unsafe\s+)?(const\s+)?fn\s+\w+", l):
        s = i
        while s - 1 >= a and ls[s - 1].startswith(pad) and ls[s - 1].strip().startswith(("///", "#[", "//")):
            s -= 1
        heads.append((s + 1, i + 1, re.search(r"fn\s+(\w+)", l).group(1)))
for k, (s, i, n) in enumerate(heads):
    e = (heads[k + 1][0] - 1) if k + 1 < len(heads) else b - 1
    print(f"{s:5}-{e:<5} {e - s + 1:4} {n}")
