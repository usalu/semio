#!/usr/bin/env python3
"""Rewrites only the staged files and the anchored edits' results inside an existing overlay (keeps .cache and the physical clones). Usage: sync-overlay.py [overlay]"""
import json,os,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
from collections import OrderedDict
ov=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/bkc/phys"
def put(rel,text):
    path=os.path.join(ov,rel);os.makedirs(os.path.dirname(path),exist_ok=True)
    if os.path.islink(path):os.remove(path)
    open(path,"w",encoding="utf8").write(text)
manifest=json.load(open(os.path.join(here,"staged","manifest.json"),encoding="utf8"))
for flat,row in manifest.items():put(row["destination"],open(os.path.join(here,"staged",flat),encoding="utf8").read())
groups=OrderedDict()
for rel,fn in wire.EDITS:groups.setdefault(rel,[]).append(fn)
for rel,fns in groups.items():
    src=open(os.path.join(names.R,rel),encoding="utf8").read()
    for fn in fns:src=fn(src)
    put(rel,src)
print("synced",len(manifest),"staged and",len(groups),"edited files into",ov)
