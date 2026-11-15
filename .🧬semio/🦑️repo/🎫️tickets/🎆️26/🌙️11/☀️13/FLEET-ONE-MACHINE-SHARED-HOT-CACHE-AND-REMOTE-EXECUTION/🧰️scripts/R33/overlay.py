"""R33: writes the edited files (live file with the edits applied) and the new files into a scratch directory for the overlay test runner of R25. usage: overlay.py <dir>"""
import os,sys,shutil
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
LIVE="/Users/Shared/semio";OV=sys.argv[1]
assert os.path.realpath(os.path.dirname(OV)).startswith("/private/tmp/claude-501/"),OV
if os.path.exists(OV):shutil.rmtree(OV)
by={}
for rel,old,new,count in edits.E:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    text=open(LIVE+"/"+rel,encoding="utf8").read()
    for old,new,count in rows:
        if new!=old and text.count(new)>=count:continue
        if text.count(old)!=count:sys.exit("anchor %d times, expected %d in %s: %r"%(text.count(old),count,rel,old[:70]))
        text=text.replace(old,new)
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
for d,_,fs in os.walk(HERE+"/new"):
    for f in fs:
        rel=os.path.relpath(os.path.join(d,f),HERE+"/new");t=OV+"/"+rel;os.makedirs(os.path.dirname(t),exist_ok=True);shutil.copyfile(os.path.join(d,f),t)
print("overlay ready",len(by))
