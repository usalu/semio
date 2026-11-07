import json,os,sys,re,collections
C=sys.argv[1]
cl=json.load(open(C+"/🗑️generated/1105b/policy-closure.json"))
dirs={os.path.dirname(k):v for k,v in cl.items()}
def emoji(ch,nxt):
    o=ord(ch)
    if ch in "0123456789#*" and nxt.startswith("️⃣"):return True
    return o>=0x1F000 or 0x2190<=o<=0x2BFF or o in(0xA9,0xAE,0x203C,0x2049,0x2122,0x2139,0x3030,0x303D,0x3297,0x3299)
res=collections.defaultdict(list);count=collections.Counter()
for d,name in dirs.items():
    for root,ds,fs in os.walk(d):
        ds[:]=[x for x in ds if x not in("target","node_modules") and not os.path.exists(os.path.join(root,x,"Cargo.toml"))]
        pass
        for f in fs:
            if not f.endswith(".rs"):continue
            p=os.path.join(root,f);count[name]+=1
            try:lines=open(p,encoding="utf-8").read().split("\n")
            except:continue
            prev=False
            for i,l in enumerate(lines,1):
                s=l.strip()
                m=re.match(r"(///|//!)(?!/)\s?(.*)$",s)
                if m:
                    if not prev and m.group(2).strip():
                        t=m.group(2).lstrip()
                        if not emoji(t[0],t[1:]):res[name].append((p,i,s[:100]))
                    prev=True
                else:
                    prev=False
tot=sum(len(v) for v in res.values());print("crates",len(dirs),"violating docstrings",tot,"crates with",len(res))
json.dump(res,open(C+"/🗑️generated/1105b/policy-docs-all.json","w"),ensure_ascii=False,indent=0)
for n,v in sorted(res.items(),key=lambda x:-len(x[1]))[:40]:print(len(v),n)
