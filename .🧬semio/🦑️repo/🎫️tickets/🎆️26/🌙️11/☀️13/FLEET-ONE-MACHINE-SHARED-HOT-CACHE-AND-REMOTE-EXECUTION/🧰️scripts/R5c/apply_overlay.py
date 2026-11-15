import os,sys,shutil,importlib
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
LIVE="/Users/Shared/semio"
OV=sys.argv[1]
assert os.path.realpath(OV).startswith("/private/tmp/claude-501/"),OV
mods=sys.argv[2:]
by={}
for m in mods:
    for rel,old,new,count in importlib.import_module(m).EDITS:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    text=open(LIVE+"/"+rel,encoding="utf8").read()
    for old,new,count in rows:
        n=text.count(old)
        if n!=count:sys.exit("anchor %d times (want %d) in %s: %r"%(n,count,rel,old[:80]))
        text=text.replace(old,new)
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
print("overlay ready",len(by),"files")
