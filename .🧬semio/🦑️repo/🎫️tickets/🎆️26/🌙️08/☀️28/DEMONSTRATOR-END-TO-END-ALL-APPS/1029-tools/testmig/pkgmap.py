#!/usr/bin/env python3
"""files (stdin) -> workspace dir, package name, package dir (nearest Cargo.toml up; tests dirs may be modules of a package)"""
import os,re,sys
root="/Users/ueli/Documents/semio"
def pkg(f):
    d=os.path.dirname(os.path.join(root,f))
    while d.startswith(root) and d!=root:
        t=os.path.join(d,"📦️packages/🦀️rust/Cargo.toml")
        if not os.path.exists(t): t=os.path.join(d,"Cargo.toml")
        if os.path.exists(t):
            s=open(t).read()
            m=re.search(r'^\[package\][^\[]*?name\s*=\s*"([^"]+)"',s,re.M|re.S)
            if m: return os.path.dirname(t),m.group(1)
        d=os.path.dirname(d)
    return None,None
def ws(d):
    d=os.path.dirname(d)
    while d.startswith(root) and d!=root:
        t=os.path.join(d,"Cargo.toml")
        if os.path.exists(t) and re.search(r'^\[workspace\]',open(t).read(),re.M): return d
        d=os.path.dirname(d)
    return None
res={}
for f in sys.stdin.read().split("\n"):
    if not f.strip(): continue
    d,n=pkg(f)
    res.setdefault((ws(d) if d else None,n),[]).append(f)
for (w,n),fs in sorted(res.items(),key=lambda x:str(x[0])):
    print((w or "?").replace(root+"/",""),"|",n,"|",len(fs))
    if "-v" in sys.argv: [print("   ",x[-80:]) for x in fs]
