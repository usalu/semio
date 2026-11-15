"""R31: writes the files as they were before the package (the edits reversed) into a directory, for the red-before proof and for rollback. usage: originals.py <dir>"""
import os,sys,shutil
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
LIVE="/Users/Shared/semio"
OV=sys.argv[1]
assert os.path.realpath(os.path.dirname(OV)).startswith("/private/tmp/claude-501/") or "/🗑️generated/" in OV,OV
if os.path.exists(OV):shutil.rmtree(OV)
by={}
for rel,old,new,count in edits.E:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    text=open(LIVE+"/"+rel,encoding="utf8").read()
    for old,new,count in reversed(rows):
        if text.count(new)!=count:sys.exit("new text %d times, expected %d in %s: %r"%(text.count(new),count,rel,new[:70]))
        text=text.replace(new,old)
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
print("originals ready",len(by))
