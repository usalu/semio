import os,json,hashlib
S=os.environ["R143_SCRATCH"];st=json.load(open(S+"/land2-state.json"))
sha=lambda x:hashlib.sha256(x.encode()).hexdigest()
for p,b,k in((st["path"],"reuse.mjs","after"),(st["d"],"reuse.d.mts","afterD")):
    cur=open(p,encoding="utf8").read()
    if sha(cur)==st[k]:
        tmp=p+".r143tmp";open(tmp,"w",encoding="utf8").write(open(S+"/backup/"+b,encoding="utf8").read());os.replace(tmp,p);print("restored",b)
    else:print("SKIP (changed by a peer):",p)
