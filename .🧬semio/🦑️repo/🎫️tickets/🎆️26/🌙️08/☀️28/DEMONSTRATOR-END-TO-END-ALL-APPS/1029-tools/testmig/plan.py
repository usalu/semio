#!/usr/bin/env python3
"""reads files (stdin) -> prints `<wsname>\t<ws dir>\t-p a -p b` lines (one per workspace)"""
import os,re,sys,subprocess
root="/Users/ueli/Documents/semio"
sys.argv=[sys.argv[0]]
exec(open(os.path.dirname(os.path.abspath(__file__))+"/pkgmap.py").read().split("res={}")[0])
res={}
for f in sys.stdin.read().split("\n"):
    if not f.strip(): continue
    d,n=pkg(f)
    if not n: print("UNMAPPED",f,file=sys.stderr); continue
    w=ws(d)
    res.setdefault(w,set()).add(n)
for w,ns in sorted(res.items()):
    name=re.sub(r'[^A-Za-z0-9]+','_',os.path.basename(os.path.dirname(w))+"_"+os.path.basename(w))
    print(w.replace(root+"/",""),"\t"," ".join("-p "+n for n in sorted(ns)),sep="")
