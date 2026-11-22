#!/usr/bin/env python3
import os,sys,shutil
real="/Users/ueli/Documents/semio"
co=sys.argv[1]
keep=set(sys.argv[2].split(","))
nm=os.path.join(real,"node_modules");out=os.path.join(co,"node_modules")
os.makedirs(out,exist_ok=True)
def retarget(t):
    return t.replace(real,co,1) if t.startswith(real) else t
def walk(rel):
    d=os.path.join(nm,rel)
    for name in sorted(os.listdir(d)):
        p=os.path.join(d,name);r=os.path.join(rel,name) if rel else name
        q=os.path.join(out,r)
        if os.path.islink(p):
            os.makedirs(os.path.dirname(q),exist_ok=True)
            if not os.path.lexists(q):os.symlink(retarget(os.readlink(p)),q)
        elif name.startswith("@") and not rel:
            walk(r)
        elif r in keep:
            if not os.path.exists(q):shutil.copytree(p,q,symlinks=True)
walk("")
