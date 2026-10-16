"""Restricted struct fields (private/pub(crate)/pub(super)) of the library closure that a test file touches as `.name` without a call, grouped by owning struct, with the line numbers.
usage: fieldscan.py <test>... -- <lib>..."""
import re, sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftcstub import mask
a = sys.argv[1:]
k = a.index("--")
tests, libs = a[:k], a[k + 1:]
STRUCT = re.compile(r"\bstruct\s+(\w+)\b[^;{(]*\{")
fields = collections.defaultdict(list)
for f in libs:
    t = mask(open(f, encoding="utf-8").read())
    for m in STRUCT.finditer(t):
        i = m.end(); depth = 1; j = i
        while j < len(t) and depth:
            depth += (t[j] == "{") - (t[j] == "}")
            j += 1
        body = t[i:j - 1]
        d = 0; cur = ""; parts = []
        for ch in body:
            if ch in "<([{": d += 1
            if ch in ">)]}" : d -= 1
            if ch == "," and d == 0:
                parts.append(cur); cur = ""
            else:
                cur += ch
        parts.append(cur)
        for p in parts:
            fm = re.match(r"^\s*(?:#\[[^\]]*\]\s*)*(pub(?:\s*\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:", p)
            if fm and (fm.group(1) is None or fm.group(1).strip() != "pub"):
                fields[fm.group(2)].append((m.group(1), os.path.basename(os.path.dirname(f)), fm.group(1).strip() if fm.group(1) else "private"))
for t in tests:
    lines = mask(open(t, encoding="utf-8").read()).split("\n")
    hits = collections.defaultdict(list)
    for i, l in enumerate(lines, 1):
        for name in fields:
            if re.search(r"\." + re.escape(name) + r"\b(?!\s*[(:])", l):
                hits[name].append(i)
    for name, ls in sorted(hits.items()):
        print(f"{os.path.basename(os.path.dirname(t))}\t.{name}\t{','.join(map(str, ls[:6]))}{'…' if len(ls) > 6 else ''}\t" + "; ".join(f"{s}@{d}({v})" for s, d, v in fields[name]))
