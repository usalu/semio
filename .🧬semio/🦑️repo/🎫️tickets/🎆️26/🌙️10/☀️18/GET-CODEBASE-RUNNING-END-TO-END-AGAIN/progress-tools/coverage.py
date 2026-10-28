import sys,os,json,re,subprocess,collections
root,tsv=sys.argv[1],sys.argv[2]
files=subprocess.run(["git","-C",root,"ls-files","-z","--","🧰️framework","🚀️launcher","🎓️teaching"],capture_output=True).stdout.decode().split("\0")
files=[f for f in files if f.endswith(".json") and not re.search("tests|fixtures|🧪|🧫",f)]
pats=[]
def leaves(o):
    if isinstance(o,str): yield o
    elif isinstance(o,dict):
        for v in o.values(): yield from leaves(v)
    elif isinstance(o,list):
        for v in o: yield from leaves(v)
def walk(o,f):
    if isinstance(o,dict):
        if "en" in o and not({"type","properties"}&set(o)):
            for s in leaves(o["en"]):
                if len(s)>=8: pats.append((s,f))
        for v in o.values(): walk(v,f)
    elif isinstance(o,list):
        for v in o: walk(v,f)
for f in files:
    try: walk(json.load(open(os.path.join(root,f))),f)
    except Exception: pass
def rx(s):
    parts=re.split(r"\{[A-Za-z0-9_.]+\}",s)
    return re.compile(".*?".join(re.escape(p) for p in parts),re.S)
comp=[(rx(s),s,f) for s,f in pats]
fixed=[(c,s,f) for c,s,f in comp if len(re.sub(r"\{[^}]*\}","",s))>=6]
print("en templates:",len(fixed),file=sys.stderr)
res=collections.defaultdict(list)
for ln in open(tsv):
    c,fl,ex=ln.rstrip("\n").split("\t",2)
    m=re.match(r"(\[[a-z -]+\]|graph:)",ex)
    if not m: continue
    tag=m.group(1)
    txt=ex
    hit=None
    for r,s,f in fixed:
        if r.fullmatch(txt) or r.fullmatch(txt.replace("[script] ","",1)):
            hit=(s,f);break
    res[tag].append((int(c),ex[:170],hit[1].replace("🧰️framework/","")[:90] if hit else None))
for tag,rows in res.items():
    un=[r for r in rows if not r[2]]
    print(f"{tag}: {len(rows)} templates, catalog-matched {len(rows)-len(un)}, unmatched {len(un)} ({sum(r[0] for r in un)} lines)")
out=open(sys.argv[3],"w")
for tag,rows in res.items():
    for c,ex,f in sorted(rows,reverse=True):
        if not f: out.write(f"UNMATCHED\t{c}\t{ex}\n")
