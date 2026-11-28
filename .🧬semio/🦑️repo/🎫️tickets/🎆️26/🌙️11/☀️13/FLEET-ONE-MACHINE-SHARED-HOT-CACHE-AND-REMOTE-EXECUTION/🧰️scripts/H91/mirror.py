#!/usr/bin/env python3
"""H91: builds a scratch mirror of the checkout: real directories along the paths of the staged files, symlinks to the live tree everywhere else, the staged files copied in. Usage: mirror.py <stage-tree> <mirror-root>. The live tree is only read."""
import os,shutil,sys
stage,mirror=sys.argv[1],sys.argv[2]
live="/Users/ueli/Documents/semio"
files=[]
for dp,dn,fn in os.walk(stage):
    for f in fn:files.append(os.path.relpath(os.path.join(dp,f),stage))
if os.path.exists(mirror):shutil.rmtree(mirror)
os.makedirs(mirror)
def build(rel):
    live_dir=os.path.join(live,rel) if rel else live
    out=os.path.join(mirror,rel) if rel else mirror
    os.makedirs(out,exist_ok=True)
    here=[f for f in files if (f.startswith(rel+"/") if rel else True)]
    children={}
    for f in here:
        rest=f[len(rel)+1:] if rel else f
        head=rest.split("/")[0]
        children.setdefault(head,[]).append(f)
    names=set(os.listdir(live_dir)) if os.path.isdir(live_dir) else set()
    for name in sorted(names|set(children)):
        child=(rel+"/"+name) if rel else name
        if name in children:
            if any(f==child for f in children[name]):
                os.makedirs(os.path.dirname(os.path.join(mirror,child)),exist_ok=True)
                shutil.copyfile(os.path.join(stage,child),os.path.join(mirror,child))
            else:build(child)
        else:
            source=os.path.join(live_dir,name)
            if os.path.isfile(source) and not os.path.islink(source) and os.path.getsize(source)<8_000_000:shutil.copyfile(source,os.path.join(out,name))
            else:os.symlink(source,os.path.join(out,name))
build("")
print(len(files),"staged files mirrored into",mirror)
