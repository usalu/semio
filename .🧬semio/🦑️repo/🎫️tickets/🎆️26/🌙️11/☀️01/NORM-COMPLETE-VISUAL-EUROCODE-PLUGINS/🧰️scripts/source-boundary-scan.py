#!/usr/bin/env python3
"""Scan every Cargo crate under a root for the repo source boundary (32 files, 262144 bytes, 4000 active lines per crate closure)."""
import os, re, sys, tomllib
ROOT = sys.argv[1] if len(sys.argv) > 1 else "✏️s/🔌️plugins/📕️norm"
MB, MF, ML = 262144, 32, 4000
MOD = re.compile(r'((?:#\[path\s*=\s*"([^"]+)"\]\s*)?)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')
INC = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
def strip_test(t):
    out=[];i=0
    pat=re.compile(r'#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:mod\s+\w+\s*(;|\{)|(?:fn|use|const|static|struct|enum|impl|type)\b)')
    while True:
        m=pat.search(t,i)
        if not m: out.append(t[i:]);break
        out.append(t[i:m.start()]);j=m.end()
        if m.group(1)==';': i=j;continue
        if m.group(1)=='{' or True:
            k=t.find('{',j-1 if m.group(1)=='{' else j); e=t.find(';',j)
            if m.group(1)!='{' and e!=-1 and (k==-1 or e<k): i=e+1;continue
            d=0
            while k<len(t):
                if t[k]=='{':d+=1
                elif t[k]=='}':
                    d-=1
                    if d==0:break
                k+=1
            i=k+1
    return "".join(out)
def active(text):
    return len(text.split("\n"))
def _old_active(text):
    n = 0; blk = False
    for l in text.splitlines():
        s = l.strip()
        if blk:
            if "*/" in s: blk = False
            continue
        if not s or s.startswith("//"): continue
        if s.startswith("/*"):
            if "*/" not in s: blk = True
            continue
        n += 1
    return n
def closure(entry):
    seen = {}; stack = [entry]
    while stack:
        p = os.path.normpath(stack.pop())
        if p in seen or not os.path.isfile(p): continue
        raw = open(p, encoding="utf-8", errors="replace").read(); t = strip_test(raw); seen[p] = (len(raw.encode()), active(t))
        d = os.path.dirname(p)
        for m in MOD.finditer(t):
            if m.group(2): stack.append(os.path.join(d, m.group(2)))
            else:
                base = os.path.splitext(os.path.basename(p))[0]
                for c in ([os.path.join(d, m.group(3) + ".rs"), os.path.join(d, m.group(3), "mod.rs")] if base in ("lib", "main", "mod") else [os.path.join(d, base, m.group(3) + ".rs")]):
                    stack.append(c)
        for m in INC.finditer(t): stack.append(os.path.join(d, m.group(1)))
    return seen
rows = []
for dp, dn, fn in os.walk(ROOT):
    dn[:] = [x for x in dn if x not in ("target", "node_modules")]
    if "Cargo.toml" in fn:
        c = tomllib.load(open(os.path.join(dp, "Cargo.toml"), "rb"))
        if "package" not in c: continue
        lib = c.get("lib", {}).get("path", "src/lib.rs")
        files = closure(os.path.join(dp, lib))
        b = sum(v[0] for v in files.values()); l = sum(v[1] for v in files.values())
        rows.append((b, len(files), l, c["package"]["name"], dp, files))
for b, f, l, n, dp, files in sorted(rows, reverse=True):
    bad = b > MB or f > MF or l > ML
    if bad or "-v" in sys.argv: print(("OVER " if bad else "ok   ") + f"{b:>8} {f:>3} {l:>6} {n} {dp}")
