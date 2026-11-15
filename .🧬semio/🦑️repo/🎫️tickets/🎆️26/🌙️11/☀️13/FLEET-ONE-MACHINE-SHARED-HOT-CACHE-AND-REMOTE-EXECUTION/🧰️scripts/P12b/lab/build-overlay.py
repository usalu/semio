import os,sys,shutil,json,hashlib
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import edits
LIVE="/Users/Shared/semio"
HERE=os.path.dirname(os.path.abspath(__file__))
OV=HERE+"/overlay"
if os.path.exists(OV):
    real=os.path.realpath(OV)
    assert real.startswith(HERE+"/") or real.startswith(os.path.realpath(HERE)+"/"),real
    shutil.rmtree(OV)
base={}
for rel in sorted({p for p,_,_,_ in edits.EDITS}):
    raw=open(LIVE+"/"+rel,"rb").read();text=raw.decode("utf8");base[rel]=hashlib.sha256(raw).hexdigest()
    for q,old,new,count in edits.EDITS:
        if q!=rel:continue
        n=text.count(old)
        if n!=count:sys.exit("anchor %d times (want %d) in %s: %r"%(n,count,rel,old[:80]))
        text=text.replace(old,new)
    dest=OV+"/"+rel;os.makedirs(os.path.dirname(dest),exist_ok=True);open(dest,"w",encoding="utf8").write(text)
for dest,src in edits.NEWFILES:
    d=OV+"/"+dest;os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(HERE+"/new/"+src,d)
for rel in [edits.LIB+"/📖️texts/🟦️.ts",edits.RES+"/📨️protocol/🎚️policy/🟨️.mjs",edits.RES+"/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts"]:
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(LIVE+"/"+rel,d)
json.dump(base,open(HERE+"/baseline.json","w"),indent=1)
print("overlay ready:",len(base),"edited,",len(edits.NEWFILES),"new")
