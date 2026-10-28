#!/usr/bin/env python3
"""mkwb.py <wb dir> <repo-relative dir>...: workbench = the workspace layout where ONLY the named directories are private APFS clones (cp -Rc), every other entry is a symlink into the real tree. Staged edits are applied and tested there (stage.py refuses symlinked paths), never in the real tree."""
import os,shutil,subprocess,sys
R="/Users/ueli/Documents/semio";wb=sys.argv[1].rstrip("/");dirs=sys.argv[2:]
shutil.rmtree(wb,ignore_errors=True);os.makedirs(wb)
def level(base_real,base_wb,parts,clone_from):
    os.makedirs(base_wb,exist_ok=True)
    nxt=parts[0] if parts else None
    for e in sorted(os.listdir(base_real)):
        if e in (".git",) : continue
        if e==nxt: continue
        d=os.path.join(base_wb,e)
        if not os.path.lexists(d): os.symlink(os.path.join(base_real,e),d)
    if nxt is None: return
    if len(parts)==1:
        d=os.path.join(base_wb,nxt)
        if not os.path.exists(d): subprocess.run(["cp","-Rc",os.path.join(base_real,nxt),d],check=True)
    else:
        level(os.path.join(base_real,nxt),os.path.join(base_wb,nxt),parts[1:],clone_from)
# merge several dirs: build the chain once per dir but never overwrite real directories already made
for rel in dirs:
    parts=rel.split("/");cur_r=R;cur_w=wb
    for i,p in enumerate(parts):
        os.makedirs(cur_w,exist_ok=True)
        for e in sorted(os.listdir(cur_r)):
            if e==".git" or e==p: continue
            d=os.path.join(cur_w,e)
            if not os.path.lexists(d): os.symlink(os.path.join(cur_r,e),d)
        cur_r=os.path.join(cur_r,p);cur_w=os.path.join(cur_w,p)
        if i==len(parts)-1:
            if not os.path.exists(cur_w): subprocess.run(["cp","-Rc",cur_r,cur_w],check=True)
        else: os.makedirs(cur_w,exist_ok=True)
print(wb)
