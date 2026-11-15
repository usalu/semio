#!/usr/bin/env python3
import json,os,hashlib,tarfile,sys
S=os.path.dirname(os.path.abspath(__file__));LIVE="/Users/Shared/semio";L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
base=json.load(open(S+"/baseline.json"));rows=[]
for d,_,fs in os.walk(S+"/mir"):
    for f in fs:
        p=os.path.join(d,f);rel=os.path.relpath(p,S+"/mir")
        if rel.startswith("node_modules") or not rel.startswith(L) or "/🦀️rust/Cargo.lock" in rel:continue
        h=hashlib.sha256(open(p,"rb").read()).hexdigest()
        if rel not in base or base[rel]!=h:rows.append((rel,rel in base))
with tarfile.open(S+"/payload.tar","w") as t:
    for rel,existed in sorted(rows):
        short=rel[len(L):]
        t.add(S+"/mir/"+rel,arcname="mirror/"+short)
        if existed or os.path.exists(LIVE+"/"+rel):
            live=open(LIVE+"/"+rel,"rb").read()
            assert (not existed) or hashlib.sha256(live).hexdigest()==base[rel],("live moved since the mirror: "+rel)
            import io
            ti=tarfile.TarInfo("base/"+short);ti.size=len(live);t.addfile(ti,io.BytesIO(live))
print(len(rows),"files in payload.tar")
