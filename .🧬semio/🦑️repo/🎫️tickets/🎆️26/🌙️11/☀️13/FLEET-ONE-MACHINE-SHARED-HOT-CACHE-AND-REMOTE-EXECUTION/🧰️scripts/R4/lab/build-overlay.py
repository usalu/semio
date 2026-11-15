#!/usr/bin/env python3
"""Builds the scratch overlay of the R4 payload: every file the edits touch (edited) and every new file, nothing else (the tests resolve all other imports in the live tree through overlay-preload.ts). usage: build-overlay.py <overlay dir under the scratchpad>"""
import os,sys,json,shutil,re
HERE=os.path.dirname(os.path.abspath(__file__));P=os.path.dirname(HERE);sys.path.insert(0,P)
import payload
LIVE=next(c for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"] if os.path.exists(c+"/AGENTS.md"))
OV=os.path.abspath(sys.argv[1]);assert OV.startswith("/private/tmp/claude-501/"),OV
if os.path.exists(OV):
    assert os.path.realpath(OV).startswith("/private/tmp/claude-501/"),OV
    shutil.rmtree(OV)
edits=json.load(open(P+"/edits.json",encoding="utf8"));by={}
for e in edits:by.setdefault(e["path"],[]).append(e)
for rel,items in by.items():
    text=open(LIVE+"/"+rel,encoding="utf8").read()
    for e in items:
        if e["kind"]=="literal":
            if text.count(e["old"])!=e["count"]:sys.exit("anchor %d times in %s: %r"%(text.count(e["old"]),rel,e["old"][:80]))
            text=text.replace(e["old"],e["new"])
        else:
            text,n=re.subn(e["old"],e["new"],text,flags=re.M)
            if n!=e["count"]:sys.exit("regex %d times in %s"%(n,rel))
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
for dest,src in payload.NEWFILES:
    d=OV+"/"+dest;os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(P+"/new/"+src,d)
for t in payload.TESTS:
    live=LIVE+"/"+os.path.dirname(t)
    for d,_,fs in os.walk(live):
        for f in fs:
            dest=OV+"/"+os.path.relpath(os.path.join(d,f),LIVE)
            if not os.path.exists(dest):os.makedirs(os.path.dirname(dest),exist_ok=True);shutil.copyfile(os.path.join(d,f),dest)
print("overlay ready:",len(by),"edited,",len(payload.NEWFILES),"new, at",OV)
