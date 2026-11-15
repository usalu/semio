import os,sys,shutil,json,hashlib
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,os.path.dirname(HERE))
import edits
LIVE="/Users/Shared/semio"
OV=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/P13/overlay"
if os.path.exists(OV):
    assert os.path.realpath(OV).startswith("/private/tmp/claude-501/"),OV
    shutil.rmtree(OV)
base={}
for rel in sorted({p for p,_,_,_ in edits.EDITS}):
    raw=open(LIVE+"/"+rel,"rb").read();text=raw.decode("utf8");base[rel]=hashlib.sha256(raw).hexdigest()
    for q,old,new,count in edits.EDITS:
        if q!=rel:continue
        n=text.count(old)
        if n!=count:sys.exit("anchor %d times (want %d) in %s: %r"%(n,count,rel,old[:90]))
        text=text.replace(old,new)
    dest=OV+"/"+rel;os.makedirs(os.path.dirname(dest),exist_ok=True);open(dest,"w",encoding="utf8").write(text)
for dest,src in edits.NEWFILES:
    d=OV+"/"+dest;os.makedirs(os.path.dirname(d),exist_ok=True)
    if os.path.exists(os.path.dirname(HERE)+"/new/"+src):shutil.copyfile(os.path.dirname(HERE)+"/new/"+src,d)
LIB=edits.LIB;GEN=edits.GEN
for rel in [GEN+"/🧪️tests/🏃️world/🟦️.ts",GEN+"/🔌️load/🧪️tests/🐍️.py",GEN+"/🔌️load/🧪️tests/🔣️.json",GEN+"/🧪️tests/🐍️.py",GEN+"/🧵️edge-reads/🧪️tests/🐍️oracle/🐍️.py",edits.EXC+"/🧪️tests/🟦️.ts",GEN+"/🔌️load/🧪️tests/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🌊️wave-rows/🟦️.ts",LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/💰️wave-costs/🟦️.ts"]:
    d=OV+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(LIVE+"/"+rel,d)
json.dump(base,open(os.path.dirname(HERE)+"/baseline.json","w"),indent=1)
print("overlay ready:",len(base),"edited,",len(edits.NEWFILES),"new")
