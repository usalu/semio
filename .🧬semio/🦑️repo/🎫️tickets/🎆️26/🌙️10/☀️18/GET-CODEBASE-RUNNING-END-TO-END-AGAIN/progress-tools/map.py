import sys,os,json,re,subprocess,collections
root,tsv,out=sys.argv[1:4]
files=subprocess.run(["git","-C",root,"ls-files","-z","--","🧰️framework","🚀️launcher","🎓️teaching"],capture_output=True).stdout.decode().split("\0")
files=[f for f in files if f.endswith(".json") and not re.search("tests|fixtures|🧪|🧫",f)]
pats=[]
def leaves(o,p):
    if isinstance(o,str): yield p,o
    elif isinstance(o,dict):
        for k,v in o.items(): yield from leaves(v,p+"/"+k)
    elif isinstance(o,list):
        for i,v in enumerate(o): yield from leaves(v,p+f"[{i}]")
def walk(o,f,p=""):
    if isinstance(o,dict):
        if "en" in o and not({"type","properties"}&set(o)):
            for lp,s in leaves(o["en"],p+"/en"):
                if len(s)>=8: pats.append((s,f,lp,"de" in o))
        for k,v in o.items(): walk(v,f,p+"/"+k)
    elif isinstance(o,list):
        for i,v in enumerate(o): walk(v,f,p+f"[{i}]")
for f in files:
    try: walk(json.load(open(os.path.join(root,f))),f)
    except Exception: pass
def rx(s): return re.compile(".*?".join(re.escape(x) for x in re.split(r"\{[A-Za-z0-9_.]+\}",s)),re.S)
comp=[(rx(s),s,f,p,d) for s,f,p,d in pats if len(re.sub(r"\{[^}]*\}","",s))>=6]
rows=collections.OrderedDict()
for ln in open(tsv):
    c,fl,ex=ln.rstrip("\n").split("\t",2)
    m=re.match(r"(\[[a-z -]+\]|graph:)",ex)
    if not m: continue
    tag=m.group(1)
    if tag=="[script]" and ex.startswith("[script] {"): kind="MACHINE-JSON"; key=(tag,kind,"","")
    else:
        hit=None;best=-1
        for r,s,f,p,d in comp:
            if r.fullmatch(ex) or r.fullmatch(ex.replace("[script] ","",1)):
                lit=len(re.sub(r"\{[^}]*\}","",s))
                if lit>best: best=lit;hit=(f,p,d)
        key=(tag,"catalog" if hit else "HARD-CODED",hit[0].replace("🧰️framework/","")[-95:] if hit else "",re.sub(r"^/(.*)/en.*$",r"\1",hit[1]) if hit else "")
        if hit and not hit[2]: key=(tag,"catalog-EN-ONLY",key[2],key[3])
    d=rows.setdefault(key,[0,0,ex[:150]]); d[0]+=1; d[1]+=int(c)
with open(out,"w") as o:
    for k,(n,c,ex) in sorted(rows.items(),key=lambda kv:(kv[0][0],-kv[1][1])):
        o.write("\t".join([k[0],k[1],str(n),str(c),k[2],k[3],ex])+"\n")
