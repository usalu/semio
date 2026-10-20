#!/usr/bin/env python3
"""Builds a mirror of the real tree where every directory on the path to the replaced files is real and everything else is a symlink to the real tree, then
replaces the given files with the overlay versions. usage: mirror-tree.py <dest> <overlay root> <repo-relative file>..."""
import os,sys,shutil
R='/Users/ueli/Documents/semio';dest,ov,*files=sys.argv[1:]
shutil.rmtree(dest,ignore_errors=True);os.makedirs(dest)
def build(rel):
    pass
paths=set()
for f in files:
    parts=f.split('/')
    for i in range(len(parts)):
        pre='/'.join(parts[:i])
        if os.path.isdir(os.path.join(R,pre)) or not pre: paths.add(pre)
def fill(rel):
    src=os.path.join(R,rel) if rel else R;dst=os.path.join(dest,rel) if rel else dest
    os.makedirs(dst,exist_ok=True)
    for name in os.listdir(src):
        child=(rel+'/'+name) if rel else name
        if child in files:
            shutil.copyfile(os.path.join(ov,child),os.path.join(dst,name))
        elif child in paths: fill(child)
        else: os.symlink(os.path.join(src,name),os.path.join(dst,name))
fill('')
for f in files:
    d=os.path.join(dest,f)
    if not os.path.lexists(d):
        os.makedirs(os.path.dirname(d),exist_ok=True)
        if os.path.islink(os.path.dirname(d)): raise SystemExit('parent is a symlink: '+os.path.dirname(d))
        shutil.copyfile(os.path.join(ov,f),d)
print('mirror ready',dest)
