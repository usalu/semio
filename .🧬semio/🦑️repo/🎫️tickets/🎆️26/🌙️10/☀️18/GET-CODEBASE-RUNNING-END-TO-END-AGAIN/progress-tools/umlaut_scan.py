import sys,os,json,re,subprocess,collections
root=sys.argv[1]
files=subprocess.run(["git","-C",root,"ls-files","-z"],capture_output=True).stdout.decode().split("\0")
if not files[0]: files=[os.path.relpath(os.path.join(d,n),root) for top in ("🧰️framework","🚀️launcher","🎓️teaching") for d,_s,ns in os.walk(os.path.join(root,top)) if "node_modules" not in d for n in ns]
files=[f for f in files if f.endswith(".json") and f.startswith(("🧰️framework","🚀️launcher","🎓️teaching")) and not re.search("/tests/|/🧪|fixtures|🧫",f)]
def leaves(o,p=""):
    if isinstance(o,str): yield p,o
    elif isinstance(o,dict):
        for k,v in o.items(): yield from leaves(v,p+"/"+k)
    elif isinstance(o,list):
        for i,v in enumerate(o): yield from leaves(v,p+f"[{i}]")
def de_leaves(o,p=""):
    if isinstance(o,dict):
        if "de" in o and not({"type","properties"}&set(o)): yield from leaves(o["de"],p+"/de")
        for k,v in o.items(): yield from de_leaves(v,p+"/"+k)
    elif isinstance(o,list):
        for i,v in enumerate(o): yield from de_leaves(v,p+f"[{i}]")
vocab=collections.Counter(); cand=collections.defaultdict(list)
tok=re.compile(r"[A-Za-zÄÖÜäöüß]+")
for f in files:
    try: d=json.load(open(os.path.join(root,f)))
    except Exception: continue
    for p,s in de_leaves(d):
        for w in tok.findall(s):
            if re.search("[äöüÄÖÜß]",w): vocab[w]+=1
            elif re.search(r"(?i)ae|oe|ue",w): cand[w].append((f,p))
print("files",len(files),"vocab",len(vocab),"candidates",len(cand),file=sys.stderr)
def variants(w):
    outs={w}
    for m in re.finditer(r"(?i)ae|oe|ue",w):
        pass
    parts=re.split(r"(?i)(ae|oe|ue)",w)
    res=[""]
    for i,pt in enumerate(parts):
        if i%2==1:
            m={"ae":"ä","oe":"ö","ue":"ü"}[pt.lower()]
            if pt[0].isupper(): m=m.upper()
            res=[r+x for r in res for x in (pt,m)]
        else: res=[r+pt for r in res]
    return set(res)
known=set(vocab)|{v.lower() for v in vocab}
rows=[]
for w,locs in cand.items():
    vs=[v for v in variants(w) if v!=w]
    hit=[v for v in vs if v in known or v.lower() in known]
    rows.append((w,len(locs),hit))
rows.sort(key=lambda r:(-len(r[2]),-r[1]))
for w,n,hit in rows: print(f"{w}\t{n}\t{','.join(hit)}")
