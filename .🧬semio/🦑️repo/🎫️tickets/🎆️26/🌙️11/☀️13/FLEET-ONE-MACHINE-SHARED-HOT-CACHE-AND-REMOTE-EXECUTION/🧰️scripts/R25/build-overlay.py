#!/usr/bin/env python3
"""Builds the R25 overlay: the edited live files (edits.py, idempotent: an edit counts as applied when its new text is present) and the new test files (new/) under one scratch root; nothing in the tree is touched. usage: build-overlay.py <overlay-dir> [--root <live root>]"""
import os,sys,shutil
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
LIVE=sys.argv[sys.argv.index("--root")+1] if "--root" in sys.argv else "/Users/Shared/semio"
OV=os.path.abspath(sys.argv[1])
assert os.path.realpath(OV).startswith("/private/tmp/claude-501/") or os.path.realpath(OV).startswith("/tmp/"),OV
if os.path.exists(OV):shutil.rmtree(OV)
def apply(text,rows):
    for old,new,count in rows:
        if new in text:continue
        n=text.count(old)
        if n!=count:raise SystemExit("anchor %d times: %r"%(n,old[:80]))
        text=text.replace(old,new)
    return text
by={}
for rel,old,new,count in edits.E:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    text=open(LIVE+"/"+rel,encoding="utf8").read();text=apply(text,rows)
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
for rel in edits.REHEARSAL_COPIES:
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(LIVE+"/"+rel,d)
for root,_,fs in os.walk(HERE+"/new"):
    for f in fs:
        src=os.path.join(root,f);rel=os.path.relpath(src,HERE+"/new");d=OV+"/"+rel
        os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(src,d)
print("overlay ready:",len(by),"edited files,",sum(len(fs) for _,_,fs in os.walk(HERE+"/new")),"new files")
