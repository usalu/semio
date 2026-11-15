import os,sys,shutil,json
HERE=os.path.dirname(os.path.abspath(__file__));SC=os.path.dirname(HERE)
LIVE="/Users/Shared/semio"
OV=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R5/overlay"
if os.path.exists(OV):
    assert os.path.realpath(OV).startswith("/private/tmp/claude-501/"),OV
    shutil.rmtree(OV)
edits=json.load(open(SC+"/edits.json",encoding="utf8"))
by={}
for rel,old,new,count in edits:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    text=open(LIVE+"/"+rel,encoding="utf8").read()
    for old,new,count in rows:
        n=text.count(old)
        if n!=count:sys.exit("anchor %d times in %s"%(n,rel))
        text=text.replace(old,new)
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
for root,_,fs in os.walk(SC+"/new"):
    for f in fs:
        src=os.path.join(root,f);rel=os.path.relpath(src,SC+"/new");d=OV+"/"+rel
        os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(src,d)
print("overlay ready")
