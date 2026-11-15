import sys,os,shutil
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R37"
sys.path.insert(0,T)
import edits37
LIVE="/Users/Shared/semio/"
src,out=sys.argv[1],sys.argv[2]
assert os.path.realpath(out).startswith("/private/tmp/claude-501/")
if os.path.exists(out):shutil.rmtree(out)
shutil.copytree(src,out)
by={}
for rel,old,new,count in edits37.EDITS:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    p=out+"/"+rel
    text=open(p if os.path.exists(p) else LIVE+rel,encoding="utf8").read()
    for old,new,count in rows:
        assert text.count(old)==count,(rel,old[:50])
        text=text.replace(old,new)
    os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(text)
print("R37 overlay edits",len(edits37.EDITS))
