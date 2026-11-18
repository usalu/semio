#!/usr/bin/env python3
"""R75 staging: copies every file of the scratch workspace that differs from the live tree into stage/ as <id>.base (live now) and <id>.new (scratch), and writes index.txt (id, path below the repository root, mod or new). Usage: stage.py <workspace dir>"""
import os,sys,shutil,filecmp
R="/Users/ueli/Documents/semio";W=os.path.abspath(sys.argv[1]);S=os.path.dirname(os.path.abspath(__file__))+"/stage"
shutil.rmtree(S,ignore_errors=True);os.makedirs(S)
EXCLUDE=()
rows=[];n=0
for d,_,fs in sorted(os.walk(W)):
    for f in sorted(fs):
        w=os.path.join(d,f);rel=w[len(W)+1:];live=os.path.join(R,rel)
        if os.path.exists(live) and filecmp.cmp(w,live,shallow=False):continue
        if rel.endswith(EXCLUDE):continue
        i="f%02d"%n;n+=1
        shutil.copy2(w,S+"/"+i+".new")
        if os.path.exists(live):shutil.copy2(live,S+"/"+i+".base");rows.append("%s %s mod"%(i,rel))
        else:rows.append("%s %s new"%(i,rel))
open(S+"/index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print(len(rows),"files staged")
