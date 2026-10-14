#!/usr/bin/env python3
"""🧹️ `StepContext` now runs `Drop`, so a shadowed earlier context still borrows its sequence slots when the next one is
built. For every E0499 on a `StepContext::new(` line in the JSON log, drops the nearest earlier live context first.

usage: 🧹️context-drops.py <tag> <root>
"""
import json, os, re, sys, collections
REPO = "/Users/ueli/Documents/semio"
tag, root = sys.argv[1], os.path.join(REPO, sys.argv[2])
hits = collections.defaultdict(set)
for line in open(f"{REPO}/.🧬semio/🦑️repo/⚡️cache/p5/logs/{tag}.json.log", encoding="utf-8", errors="replace"):
    if not line.startswith("{"): continue
    m = json.loads(line)
    if m.get("reason") != "compiler-message": continue
    d = m["message"]
    if (d.get("code") or {}).get("code") != "E0499": continue
    sp = [s for s in d["spans"] if s["is_primary"]][0]
    f = sp["file_name"]
    for base in [os.path.join(REPO, "✏️s"), REPO]:
        p = os.path.normpath(os.path.join(base, f))
        if os.path.exists(p): break
    if os.path.realpath(p).startswith(os.path.realpath(root)): hits[p].add(sp["line_start"])
for p, lines in hits.items():
    src = open(p, encoding="utf-8").read().split("\n")
    def decl(j):
        m = re.match(r"(\s*)let\s+(?:mut\s+)?(\w+)\s*=\s*(?:(?:semio_framework_job::)?StepContext::new\(|$)", src[j])
        if not m: return None
        if "StepContext::new(" in src[j] or (j + 1 < len(src) and "StepContext::new(" in src[j + 1]): return m.group(1), m.group(2)
        return None
    for ln in sorted(lines, reverse=True):
        i = ln - 1
        if not re.match(r"\s*let\b", src[i]) and i > 0 and re.match(r"\s*let\s+(?:mut\s+)?\w+\s*=\s*$", src[i - 1]): i -= 1
        indent = re.match(r"\s*", src[i]).group(0)
        name = None
        for j in range(i - 1, max(-1, i - 300), -1):
            d = decl(j)
            if d and len(d[0]) <= len(indent):
                pad, name = d; break
            if re.match(r"\s*fn\s", src[j]): break
        if not name: print("no prior context", p, ln); continue
        at = i
        if len(pad) < len(indent):
            for k in range(i - 1, j, -1):
                if len(src[k]) - len(src[k].lstrip()) == len(pad) and src[k].strip(): at = k; break
            at = at if at != i else i
        if any(re.search(r"\bdrop\(" + name + r"\)", src[k]) for k in range(j, at)): continue
        src.insert(at, f"{pad if at != i else indent}drop({name});")
        print("drop", name, "before", os.path.relpath(p, REPO), at + 1)
    open(p, "w", encoding="utf-8").write("\n".join(src))
