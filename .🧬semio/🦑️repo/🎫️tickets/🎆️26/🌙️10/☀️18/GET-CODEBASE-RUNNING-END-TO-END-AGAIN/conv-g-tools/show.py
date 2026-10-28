import sys, re, lib, build
W = build.W
for a in sys.argv[1:]:
    p = build.resolve(a); t = open(W + p, encoding="utf8").read(); L = t.split("\n")
    print("##", a, len(L), "lines")
    ops = lib.openers(p, t); r = lib.reuse(p, t); bad = {l for v in r.values() for l in v}
    used = sorted({lib.first_grapheme(c).replace(lib.FE0F, "") for _, c in ops if lib.MARK.match(c)})
    print("used:", "".join(used))
    docs_end = {ln for ln, _ in ops}
    for ln, c in ops:
        if ln in bad or not lib.MARK.match(c): print(f"  DUP/NOEMOJI {ln}: {c[:110]}  ||  {L[ln].strip()[:90] if ln < len(L) else ''}")
    for i, l in enumerate(L, 1):
        if build.lib and re.match(r"^export\s+(default\s+)?(declare\s+)?(async\s+)?(function\*?|const|let|var|class|type|interface|enum|abstract\s+class)\b", l):
            prev = L[i - 2].rstrip() if i > 1 else ""
            if not prev.endswith("*/"): print(f"  NODOC {i}: {l.strip()[:230]}")
