import os,sys,shutil
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
LIVE="/Users/Shared/semio"
OV=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R17/overlay"
if os.path.exists(OV):
    assert os.path.realpath(OV).startswith("/private/tmp/claude-501/"),OV
    shutil.rmtree(OV)
by={}
for rel,old,new,count in edits.E:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    text=open(LIVE+"/"+rel,encoding="utf8").read()
    for old,new,count in rows:
        if text.count(new)==count and new!=old and old not in text:continue
        n=text.count(old)
        if n!=count:sys.exit("anchor %d times in %s: %r"%(n,rel,old[:80]))
        text=text.replace(old,new)
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
for root,_,fs in os.walk(HERE+"/new"):
    for f in fs:
        src=os.path.join(root,f);rel=os.path.relpath(src,HERE+"/new");d=OV+"/"+rel
        os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(src,d)
print("overlay ready",len(by),"edited files")
