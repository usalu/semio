import os,re,sys
root=sys.argv[1]
pat=re.compile(r'''(?:from\s+|import\s*\(\s*|import\s+|require\(\s*)(["'])(\.{1,2}/[^"']+)\1''')
bad=0;n=0
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in("node_modules","dist","test-results",".cache")]
    for f in fs:
        if not f.endswith((".ts",".tsx",".mjs",".js",".mts")): continue
        p=os.path.join(d,f);n+=1
        t=open(p,encoding="utf8",errors="replace").read()
        for m in pat.finditer(t):
            s=m.group(2).split("?")[0]
            q=os.path.normpath(os.path.join(d,s))
            if not os.path.exists(q):
                alts=[q+e for e in(".ts",".tsx",".mjs",".js")]+[os.path.join(q,"index"+e) for e in(".ts",".tsx")]
                if not any(os.path.exists(a) for a in alts):
                    bad+=1;print("MISSING",p,"->",s)
print("files",n,"missing",bad)
