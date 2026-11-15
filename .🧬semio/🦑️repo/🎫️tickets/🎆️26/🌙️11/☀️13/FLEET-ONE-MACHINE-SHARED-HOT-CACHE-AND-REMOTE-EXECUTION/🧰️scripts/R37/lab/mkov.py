import sys,os,json,hashlib
R29="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R29"
LIVE="/Users/Shared/semio/"
out=sys.argv[1];assert os.path.realpath(out).startswith("/private/tmp/claude-501/")
sys.path.insert(0,R29)
import edits as r29
by={}
for rel,old,new,count in r29.EDITS:by.setdefault(rel,[]).append((old,new,count))
for rel,rows in by.items():
    text=open(LIVE+rel,encoding="utf8").read()
    for old,new,count in rows:
        if text.count(old)!=count:sys.exit("R29 anchor mismatch %s %r"%(rel,old[:60]))
        text=text.replace(old,new)
    p=out+"/"+rel;os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(text)
for dest,src in r29.NEWFILES:
    p=out+"/"+dest;os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"wb").write(open(R29+"/new/"+src,"rb").read())
print("R29 overlay files",len(by),"new",len(r29.NEWFILES))
