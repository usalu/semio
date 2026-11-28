#!/usr/bin/env python3
"""H73 overlay: a scratch copy of the two cargo module trees (caching + process launcher) whose sibling directories are symlinks
into the live tree, so staged patches run their tests without touching the live files. `overlay.py OUT` rebuilds OUT (copy refreshed
from the live tree). Needs python3 + rsync."""
import os,subprocess,sys
R="/Users/ueli/Documents/semio/🧰️framework"
CHAIN=["🛍️products","🦑️repo","🔨️modules","📚️library"]
REAL=[("⚡️caching","🦀️cargo"),("🏃️process","🦀️cargo")]
def level(src,dst,keep):
    os.makedirs(dst,exist_ok=True)
    for name in os.listdir(src):
        if name in keep:continue
        link=os.path.join(dst,name)
        if not os.path.lexists(link):os.symlink(os.path.join(src,name),link)
out=sys.argv[1];src=R;dst=os.path.join(out,"🧰️framework")
level(src,dst,{CHAIN[0]})
for i,name in enumerate(CHAIN):
    src=os.path.join(src,name);dst=os.path.join(dst,name)
    level(src,dst,{CHAIN[i+1]} if i+1<len(CHAIN) else {r[0] for r in REAL})
for parent,child in REAL:
    s=os.path.join(src,parent);d=os.path.join(dst,parent);level(s,d,{child})
    subprocess.run(["rsync","-a","--delete","--exclude=node_modules",os.path.join(s,child)+"/",os.path.join(d,child)+"/"],check=True)
print(os.path.join(dst,"⚡️caching/🦀️cargo/🗃️unit-cache"))
