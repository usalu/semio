"""unused.py <file...>: lists explicitly imported names (top-level `use` lines) that occur nowhere else in the file (traits may still be needed for method calls)."""
import re, sys
def names(use):
    body = use[3:].rstrip(";").strip()
    m = re.search(r"\{(.*)\}", body, re.S)
    items = [x.strip() for x in m.group(1).split(",")] if m else [body.split("::")[-1]]
    out = []
    for it in items:
        if not it or it in ("self", "*"): continue
        out.append(it.split(" as ")[-1].strip())
    return out
for f in sys.argv[1:]:
    t = open(f, encoding="utf-8").read()
    uses = re.findall(r"^use [^;]*;", t, re.M)
    rest = re.sub(r"^use [^;]*;", "", t, flags=re.M)
    for u in uses:
        for n in names(u):
            if n == "*": continue
            if not re.search(r"(?<![\w])" + re.escape(n) + r"(?![\w])", rest):
                print(f[-60:], "UNUSED", n, "in", u[:100])
