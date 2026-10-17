"""🔓 Widens listed struct fields and inherent methods of one type to `pub` in one file (the type's own `struct T {..}` block and every `impl T {..}` block; trait impls untouched).
usage: widen.py <file> <Type> <comma fields or -> <comma methods or -> [--dry]"""
import re, sys
p, ty, fields, methods = sys.argv[1:5]
dry = "--dry" in sys.argv
fields = [] if fields == "-" else fields.split(",")
methods = [] if methods == "-" else methods.split(",")
t = open(p, encoding="utf-8").read()
changed = []
def block_end(s, i):
    d = 0
    for k in range(i, len(s)):
        if s[k] == "{": d += 1
        elif s[k] == "}":
            d -= 1
            if d == 0: return k
    return len(s)
m = re.search(r"^(?:pub(?:\([^)]*\))? )?struct " + re.escape(ty) + r"\b[^;{]*\{", t, re.M)
if m and fields:
    e = block_end(t, m.end() - 1)
    body = t[m.end():e]
    for f in fields:
        nb, n = re.subn(r"(?m)^(\s*)(?:pub\(\w+(?: \w+)?\) )?(?<!pub )" + re.escape(f) + r"\s*:", lambda mm: mm.group(1) + "pub " + f + ":", body)
        if n:
            body = nb; changed.append(f"field {f}")
    t = t[:m.end()] + body + t[e:]
pos = 0
while True:
    m = re.compile(r"^impl(?:<[^>]*>)? " + re.escape(ty) + r"(?:<[^>]*>)? \{", re.M).search(t, pos)
    if not m: break
    e = block_end(t, m.end() - 1)
    body = t[m.end():e]
    for f in methods:
        nb, n = re.subn(r"(?m)^(\s*)(?:pub\([^)]*\) )?(?<!pub )((?:const |async |unsafe )*)fn " + re.escape(f) + r"\b", lambda mm: mm.group(1) + "pub " + mm.group(2) + "fn " + f, body)
        if n:
            body = nb; changed.append(f"fn {f}")
    t = t[:m.end()] + body + t[e:]
    pos = m.end() + len(body)
print(ty, changed)
if not dry and changed:
    open(p, "w", encoding="utf-8").write(t)
