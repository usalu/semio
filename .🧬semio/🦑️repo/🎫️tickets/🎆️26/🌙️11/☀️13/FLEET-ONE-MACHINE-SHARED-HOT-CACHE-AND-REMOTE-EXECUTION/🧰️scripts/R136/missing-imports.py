import os,re,sys
root=sys.argv[1]
pat=re.compile(r'''(?:from\s*|import\s*\(?\s*|import\s+)["'](\.{1,2}/[^"']+)["']''')
miss={}
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in("node_modules","target",".git")]
    for f in fs:
        if not f.endswith((".ts",".mjs",".mts",".js")):continue
        p=os.path.join(d,f)
        try:t=open(p,encoding="utf8").read()
        except Exception:continue
        for m in pat.finditer(t):
            s=m.group(1)
            if s.endswith(".json") or "?" in s:q=os.path.normpath(os.path.join(d,s))
            else:q=os.path.normpath(os.path.join(d,s))
            if not os.path.exists(q):miss.setdefault(q,[]).append(p)
for q,ps in sorted(miss.items()):print(q,"<-",len(ps),ps[0])
