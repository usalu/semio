import json,os,hashlib,sys,re
A="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/"
tot=0;bad=[]
for d in ("◻️2d","🧊️3d"):
    S=A+d+"/🏅️standards/🔖️1/🪆️subsets/"
    odir=S+"🌐️any/🔮️oracles/"
    doc=json.load(open(odir+"🔣️.json",encoding="utf-8"))
    entries=doc if isinstance(doc,list) else doc.get("entries") or doc.get("oracles") or doc
    def walk(o):
        if isinstance(o,dict):
            if "path" in o and "sha256" in o: yield o
            for v in o.values(): yield from walk(v)
        elif isinstance(o,list):
            for v in o: yield from walk(v)
    n=0
    for f in walk(doc):
        n+=1
        p=os.path.normpath(os.path.join(odir,f["path"]))
        if not os.path.exists(p): bad.append(("missing",d,f["path"])); continue
        b=open(p,"rb").read()
        if "sha256:"+hashlib.sha256(b).hexdigest()!=f["sha256"] or len(b)!=f["bytes"]: bad.append(("hash",d,f["path"]))
    print(d,"entries with files",n); tot+=n
print("total",tot,"bad",len(bad))
import collections
print(collections.Counter(b[0] for b in bad))
for b in bad[:12]: print(b)
