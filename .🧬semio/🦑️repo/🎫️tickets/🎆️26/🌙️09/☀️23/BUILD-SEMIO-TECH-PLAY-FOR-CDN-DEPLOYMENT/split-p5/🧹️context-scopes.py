#!/usr/bin/env python3
"""🧹️ `StepContext` runs `Drop`, so an earlier context that is still in scope keeps its `&mut` sequence slots borrowed
when the next context is built from the same slots. For every pair of context declarations in one function where the
later one sits inside the earlier one's block, inserts `drop(<earlier>);` at the earlier one's block level just before
the later declaration (or before the nested block that contains it). `--strip` first removes every bare
`drop(<context>);` line (only for files whose originals had none).

usage: 🧹️context-scopes.py [--strip] <file.rs>...
"""
import re, sys
DECL = re.compile(r"^(\s*)let\s+(?:mut\s+)?(\w+)\s*=\s*(?:(?:semio_framework_job::)?StepContext::new\(|$)")
def ind(l): return len(l) - len(l.lstrip())
for path in [a for a in sys.argv[1:] if not a.startswith("--")]:
    src = open(path, encoding="utf-8").read().split("\n")
    names = set()
    for i, l in enumerate(src):
        m = DECL.match(l)
        if m and ("StepContext::new(" in l or (i + 1 < len(src) and "StepContext::new(" in src[i + 1])): names.add(m.group(2))
    if "--strip" in sys.argv:
        src = [l for l in src if not (re.fullmatch(r"\s*drop\((\w+)\);", l) and re.fullmatch(r"\s*drop\((\w+)\);", l).group(1) in names)]
    decls = []
    for i, l in enumerate(src):
        m = DECL.match(l)
        if m and ("StepContext::new(" in l or (i + 1 < len(src) and "StepContext::new(" in src[i + 1])): decls.append([i, ind(l), m.group(2)])
    inserts = {}
    for a in range(len(decls)):
        ai, aind, aname = decls[a]
        end = next((k for k in range(ai + 1, len(src)) if src[k].strip() and ind(src[k]) < aind), len(src))
        for b in range(a + 1, len(decls)):
            bi, bind, bname = decls[b]
            if bi >= end: break
            if bind < aind: continue
            if bind == aind: at = bi
            else:
                at = next((k for k in range(bi - 1, ai, -1) if src[k].strip() and ind(src[k]) == aind), bi)
            used_between = any(re.search(r"\b" + aname + r"\b", src[k]) for k in range(ai + 1, at))
            later_use = any(re.search(r"\b" + aname + r"\b", src[k]) and not DECL.match(src[k]) for k in range(at, bi))
            if bind > aind and later_use: continue
            inserts.setdefault(at, (" " * aind, aname))
            break
    for at in sorted(inserts, reverse=True):
        pad, name = inserts[at]
        src.insert(at, f"{pad}drop({name});")
    open(path, "w", encoding="utf-8").write("\n".join(src))
    print(path.split("⚙️engine/")[-1], "drops", len(inserts))
