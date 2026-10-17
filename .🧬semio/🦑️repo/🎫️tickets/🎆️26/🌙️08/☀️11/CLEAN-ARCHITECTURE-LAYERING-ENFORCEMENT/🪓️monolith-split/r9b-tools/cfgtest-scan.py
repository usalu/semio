#!/usr/bin/env python3
# usage: cfgtest-scan.py <files-json>  (from r9b-census.ts DUMPALL). Reports names that crate A defines only under cfg(test) (or in test-only files, pub) and that another crate B's files mention.
import json, re, sys, collections
ROOT = "/Users/ueli/Documents/semio/"
d = json.load(open(sys.argv[1]))
crates = collections.defaultdict(dict)
for k, v in d.items():
    dirp, kind = k.rsplit("|", 1)
    crates[dirp][kind] = v
read = {}
def text(p):
    if p not in read:
        try: read[p] = open(ROOT + p, encoding="utf-8").read()
        except Exception: read[p] = ""
    return read[p]
ITEM = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?(fn|const|static|struct|enum|type|trait|mod|union)\s+([A-Za-z_][A-Za-z0-9_]*)|^\s*macro_rules!\s+([A-Za-z_][A-Za-z0-9_]*)|^\s*(?:pub(?:\([^)]*\))?\s+)use\s+(.*)')
CFG = re.compile(r'^\s*#\[cfg\((?:test|any\(test[^)]*\)|all\(test[^)]*\))\)\]\s*$')
def gated_names(path):
    lines = text(path).split("\n")
    out = set()
    for i, line in enumerate(lines):
        if not CFG.match(line): continue
        j = i + 1
        while j < len(lines) and (lines[j].strip().startswith("#[") or lines[j].strip().startswith("///") or lines[j].strip().startswith("//")): j += 1
        if j >= len(lines): continue
        m = ITEM.match(lines[j])
        if not m: continue
        if m.group(1) == "union" or (m.group(1) == "mod" and (m.group(2) in ("tests","unit_tests","example") or "tests" in m.group(2) or "test" in m.group(2))): continue
        if m.group(2): out.add(m.group(2))
        elif m.group(3): out.add(m.group(3))
        elif m.group(4):
            for n in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', re.sub(r'\bas\b|\bself\b|\bsuper\b|\bcrate\b', ' ', m.group(4))): out.add(n)
    return out
defs = {}
for c, kinds in crates.items():
    lib = set(kinds.get("lib", []))
    names = set()
    for p in lib: names |= gated_names(p)
    defs[c] = names
alldefined = collections.defaultdict(set)
for c, kinds in crates.items():
    for p in set(kinds.get("lib", [])) | set(kinds.get("lib-test", [])):
        for m in re.finditer(r'\b(?:fn|const|static|struct|enum|type|trait|mod|union)\s+([A-Za-z_][A-Za-z0-9_]*)', text(p)):
            alldefined[c].add(m.group(1))
hits = 0
for a, names in defs.items():
    for b, kinds in crates.items():
        if a == b: continue
        bfiles = set(kinds.get("lib-test", [])) | set(kinds.get("lib", []))
        afiles = set(crates[a].get("lib-test", [])) | set(crates[a].get("lib", []))
        for n in sorted(names):
            if n in alldefined[b] and n not in alldefined[a]: pass
            for p in sorted(bfiles - afiles):
                for ln, line in enumerate(text(p).split("\n"), 1):
                    if re.search(r'(?<![A-Za-z0-9_])' + re.escape(n) + r'(?![A-Za-z0-9_])', line) and not line.strip().startswith("//"):
                        print(f"{a.split('/')[-3] if False else a[-70:]} defines cfg(test) `{n}`; {b[-50:]} mentions it at {p[-90:]}:{ln}: {line.strip()[:140]}")
                        hits += 1
print("hits", hits, "gated names", {c[-40:]: sorted(n) for c, n in defs.items() if n})
