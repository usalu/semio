#!/usr/bin/env python3
"""closure.py <scratch> <entry>...: copies the entries and every relative import they reach (plus json, py oracles and fixtures beside tests) from the live tree into <scratch>/tree/<same relative path>, and records <scratch>/base/<same> so the landing can refuse a live file that moved. Copies only files, never links."""
import os,re,sys,shutil
R="/Users/Shared/semio";S=sys.argv[1]
pat=re.compile(r'''(?:from|import)\s*\(?\s*["'](\.{1,2}/[^"']+)["']''')
seen=set();stack=[os.path.join(R,e) if not e.startswith("/") else e for e in sys.argv[2:]]
def resolve(base,spec):
    p=os.path.normpath(os.path.join(os.path.dirname(base),spec))
    for c in (p,p+".ts",p+".json",p+".mjs",p+"/🟦️.ts"):
        if os.path.isfile(c):return c
for e in list(stack):pass
while stack:
    f=stack.pop()
    if f in seen or not os.path.isfile(f):continue
    seen.add(f)
    if f.endswith((".ts",".mjs")):
        for m in pat.finditer(open(f,encoding="utf8",errors="ignore").read()):
            r=resolve(f,m.group(1))
            if r and r not in seen:stack.append(r)
for f in sorted(seen):
    rel=os.path.relpath(f,R)
    for kind in("tree","base"):
        d=os.path.join(S,kind,rel);os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(f,d)
print(len(seen),"files")
