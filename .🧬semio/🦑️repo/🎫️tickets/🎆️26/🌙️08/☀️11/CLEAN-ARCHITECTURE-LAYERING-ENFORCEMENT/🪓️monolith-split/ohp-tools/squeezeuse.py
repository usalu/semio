"""🧽 Merges runs of adjacent single-name `use a::b::X;` lines with the same parent path into one braced `use a::b::{X, Y};` (top level, no attributes, no `as`) until the file's split length is at most <max-lines>.
usage: squeezeuse.py <repo-rel file> <max-lines>"""
import re, sys, os
R = "/Users/ueli/Documents/semio"
rel, mx = sys.argv[1], int(sys.argv[2])
p = os.path.join(R, rel)
L = open(p, encoding="utf-8").read().split("\n")
n0 = len(L)
def key(l):
    m = re.match(r"^use ([A-Za-z0-9_:]+)::([A-Za-z0-9_]+);$", l)
    return (m.group(1), m.group(2)) if m else None
out = []; i = 0
while i < len(L):
    k = key(L[i])
    if k and len(L) - (i - len(out)) > mx and not (out and out[-1].lstrip().startswith("#[")):
        j = i; names = []
        while j < len(L) and key(L[j]) and key(L[j])[0] == k[0]:
            names.append(key(L[j])[1]); j += 1
        if len(names) > 1:
            total_now = len(out) + (len(L) - i)
            take = min(len(names), total_now - mx + 1)
            if take > 1:
                out.append(f"use {k[0]}::{{{', '.join(names[:take])}}};")
                out += [f"use {k[0]}::{n};" for n in names[take:]]
                i = j
                continue
    out.append(L[i]); i += 1
open(p, "w", encoding="utf-8").write("\n".join(out))
print(rel[-60:], n0, "->", len(out), "target", mx)
