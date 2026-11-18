import os,sys,hashlib,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import overlay
R=overlay.R;S=overlay.S;B=overlay.B
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R66"
D=T+"/stage"
extra=[B+"🪪️graph-state/🙋️wanted/💰️cost/🧬️schema/🔣️.json",B+"🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts",B+"🪪️graph-state/🙋️wanted/💰️cost/🧪️tests/🟦️.ts",B+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🛑️too-heavy/🟦️.ts",B+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🐌️slow-last/🟦️.ts",B+"🕸️graph/🧪️tests/🙋️fleet-read/🟦️.ts",B+"🌐️remote-execution/🛰️sidecar/🧪️tests/🏊️pool/🟦️.ts",B+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🚪️away/🟦️.ts"]
files=sorted(set(overlay.changed()+[e for e in extra if os.path.exists(S+"/"+e) and (not os.path.exists(R+"/"+e) or open(R+"/"+e,'rb').read()!=open(S+"/"+e,'rb').read())]))
os.makedirs(D,exist_ok=True)
for f in os.listdir(D):os.remove(D+"/"+f)
rows=[]
ws="/private/tmp/r66/ws-base/"
for rel in files:
    r=rel[len(B):]
    i=hashlib.sha256(r.encode()).hexdigest()[:6]
    new=open(S+"/"+rel,"rb").read();open(D+"/"+i+".new","wb").write(new)
    live=R+"/"+rel
    if os.path.exists(live):
        base=open(ws+rel,"rb").read() if os.path.exists(ws+rel) else open(live,"rb").read()
        assert base==open(live,"rb").read(),("live moved since the snapshot",rel)
        open(D+"/"+i+".base","wb").write(base);rows.append("%s %s mod"%(i,r))
    else:rows.append("%s %s new"%(i,r))
open(D+"/index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print("\n".join(rows))
