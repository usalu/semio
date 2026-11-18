#!/usr/bin/env python3
"""R29 payload: folds the overlay (tool rows, schema bounds, corpus rows, native toolchain by reference, stamp) into edits.json (whole-file anchored replacements) and baseline.json."""
import json,os,sys,hashlib
HERE=os.path.dirname(os.path.abspath(__file__))
R="/Users/ueli/Documents/semio/"
ovl=sys.argv[1]
assert os.path.realpath(ovl).startswith("/private/tmp/claude-501/")
edits=[];base={}
for d,_,fs in os.walk(ovl):
    for f in fs:
        p=os.path.join(d,f);rel=os.path.relpath(p,ovl)
        if "/🧪️tests/r29/" in rel:continue
        if not os.path.exists(R+rel):continue
        live=open(R+rel,encoding="utf8").read();new=open(p,encoding="utf8").read()
        if live==new:continue
        edits.append([rel,live,new,1]);base[rel]=hashlib.sha256(live.encode()).hexdigest()
sys.path.insert(0,HERE)
import edits_si
anch={}
for rel,old,new,count in edits_si.EDITS:anch.setdefault(rel,[]).append([rel,old,new,count])
edits=[e for e in edits if e[0] not in anch]+[row for rows in anch.values() for row in rows]
for rel in anch:
    base[rel]=hashlib.sha256(open(R+rel,"rb").read()).hexdigest()
edits.sort(key=lambda e:e[0])
json.dump(edits,open(HERE+"/edits.json","w",encoding="utf8"),ensure_ascii=False)
json.dump(base,open(HERE+"/baseline.json","w"),indent=1,ensure_ascii=False)
print("edited files",len(edits),[e[0].split("/")[-3:] for e in edits])
