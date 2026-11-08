#!/usr/bin/env python3
"""Read-only: does crate A reach crate B through path dependencies? python3 -I reach.py <fromPkg> <toPkg>"""
import os,re,sys
R="/Users/ueli/Documents/semio/"
def pkgname(t):
    m=re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"',t,re.M|re.S); return m.group(1) if m else None
idx={}
for root in (R+"🧰️framework",R+"✏️s"):
    for d,ds,fs in os.walk(root):
        ds[:]=[x for x in ds if x not in("node_modules","target","dist",".git")]
        if "Cargo.toml" in fs:
            t=open(d+"/Cargo.toml").read(); n=pkgname(t)
            if n: idx[n]=(d,t)
def deps(t,kinds):
    out=[]; sec=None
    for line in t.split("\n"):
        m=re.match(r'\s*\[([^\]]+)\]',line)
        if m: sec=m.group(1); continue
        if sec and any(sec==k or sec.endswith("."+k) for k in kinds):
            m=re.match(r'\s*([A-Za-z0-9_\-]+)\s*=\s*(.*)',line)
            if m:
                pk=re.search(r'package\s*=\s*"([^"]+)"',m.group(2)); out.append(pk.group(1) if pk else m.group(1))
    return out
a,b=sys.argv[1:3]
for kinds in (["dependencies","build-dependencies"],["dependencies","build-dependencies","dev-dependencies"]):
    seen={}; stack=[(a,[a])]; hit=None
    while stack and not hit:
        n,path=stack.pop()
        if n in seen: continue
        seen[n]=1
        if n not in idx: continue
        for d in deps(idx[n][1],kinds):
            if d==b: hit=path+[d]; break
            stack.append((d,path+[d]))
    print(kinds,"->",("REACHES: "+" > ".join(hit)) if hit else f"no path (visited {len(seen)})")
