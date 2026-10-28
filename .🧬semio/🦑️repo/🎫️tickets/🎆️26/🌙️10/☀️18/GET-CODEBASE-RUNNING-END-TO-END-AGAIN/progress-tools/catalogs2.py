import sys,os,json,re,subprocess
root=sys.argv[1]
files=subprocess.run(["git","-C",root,"ls-files","-z","--","🧰️framework","🚀️launcher"],capture_output=True).stdout.decode().split("\0")
if not files[0]: files=[os.path.relpath(os.path.join(d,n),root) for top in ("🧰️framework","🚀️launcher") for d,_s,ns in os.walk(os.path.join(root,top)) if "node_modules" not in d for n in ns]
files=[f for f in files if f.endswith(".json") or f.endswith(".mjs") and False]
ph=re.compile(r"\{([A-Za-z0-9_.]+)\}")
def strs(x):
    return isinstance(x,str) or (isinstance(x,dict) and x and all(strs(v) for v in x.values()))
def cmp(a,b,path,out):
    if isinstance(a,str) and isinstance(b,str):
        x,y=set(ph.findall(a)),set(ph.findall(b))
        if x!=y: out.append((path,f"fields en-only={sorted(x-y)} de-only={sorted(y-x)}",a[:120],b[:120]))
        if a.strip()==b.strip() and len(a)>25 and re.search("[a-z]{4}",a): out.append((path,"identical text in en and de",a[:120],""))
    elif isinstance(a,dict) and isinstance(b,dict):
        for k in sorted(set(a)|set(b)):
            if k not in b: out.append((path+"/"+k,"missing in de",str(a[k])[:120],""))
            elif k not in a: out.append((path+"/"+k,"missing in en","",str(b[k])[:120]))
            else: cmp(a[k],b[k],path+"/"+k,out)
    else: out.append((path,"shape differs",str(a)[:80],str(b)[:80]))
stat={"pairs":0,"strings":0}
def count(o):
    if isinstance(o,str): return 1
    if isinstance(o,dict): return sum(count(v) for v in o.values())
    return 0
def walk(o,path,f,out):
    if isinstance(o,dict):
        hasen,hasde=("en" in o and strs(o["en"])),("de" in o and strs(o["de"]))
        if hasen and hasde:
            stat["pairs"]+=1; stat["strings"]+=count(o["en"])
            sub=[];cmp(o["en"],o["de"],path,sub)
            out.extend((f,)+s for s in sub); return
        if (hasen!=hasde) and ("en" in o or "de" in o) and not({"properties","type"}&set(o)) :
            out.append((f,path,"only "+("en" if hasen else "de"),"",""))
        for k,v in o.items(): walk(v,path+"/"+k,f,out)
    elif isinstance(o,list):
        for i,v in enumerate(o): walk(v,path+f"[{i}]",f,out)
res=[]
for f in files:
    try: d=json.load(open(os.path.join(root,f)))
    except Exception: continue
    walk(d,"",f,res)
print(stat)
for r in res: print(json.dumps(r,ensure_ascii=False))
