import sys,os,json,re,subprocess
root=sys.argv[1]
files=subprocess.run(["git","-C",root,"ls-files","-z","--","🧰️framework","🚀️launcher"],capture_output=True).stdout.decode().split("\0")
files=[f for f in files if f.endswith(".json") and ("schema" in f or "🧬" in f or "catalog" in f.lower() or "progress" in f or "messages" in f.lower())]
ph=re.compile(r"\{([A-Za-z0-9_]+)\}")
out=[]
def find(o,path,acc):
    if isinstance(o,dict):
        strs=lambda x:isinstance(x,dict) and x and all(isinstance(v,str) for v in x.values())
        if ("en" in o or "de" in o) and (strs(o.get("en")) or strs(o.get("de"))):
            o.setdefault("en",{});o.setdefault("de",{})
            acc.append((path,o))
            return
        for k,v in o.items(): find(v,path+"/"+k,acc)
    elif isinstance(o,list):
        for i,v in enumerate(o): find(v,path+f"[{i}]",acc)
def find_nolang(o,path,acc):
    pass
tot=0;bad=[]
for f in files:
    try: d=json.load(open(os.path.join(root,f)))
    except Exception: continue
    acc=[];find(d,"",acc)
    for path,o in acc:
        en,de=o["en"],o["de"]
        for k in sorted(set(en)|set(de)):
            tot+=1
            if k not in de: bad.append((f,path,k,"missing-de",en[k])); continue
            if k not in en: bad.append((f,path,k,"missing-en",de[k])); continue
            if not isinstance(en[k],str) or not isinstance(de[k],str): continue
            a=set(ph.findall(en[k]));b=set(ph.findall(de[k]))
            if a!=b: bad.append((f,path,k,f"fields en-only={sorted(a-b)} de-only={sorted(b-a)}",en[k]+" || "+de[k]))
        for l in o:
            if l not in("en","de"): bad.append((f,path,"-","extra-lang "+l,""))
print("catalog files with en+de:",sum(1 for f in files if True),"entries",tot)
for b in bad: print(json.dumps(b,ensure_ascii=False))
