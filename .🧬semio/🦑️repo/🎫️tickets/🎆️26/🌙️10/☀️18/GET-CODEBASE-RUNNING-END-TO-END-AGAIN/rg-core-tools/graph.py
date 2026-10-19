import json, re, subprocess, sys, os, collections
R="/Users/ueli/Documents/semio"
here=os.path.dirname(__file__)
mdir=sys.argv[1]; mode=sys.argv[2] if len(sys.argv)>2 else "prod"
out=subprocess.check_output(["bun",os.path.join(here,"owners.ts")],input=json.dumps([mdir]).encode(),cwd=R)
files=[f for f in json.loads(out)[mdir][mode] if f.endswith(".rs")]
text={f:open(os.path.join(R,f),encoding="utf-8").read() for f in files}
defrx=re.compile(r'(?m)^(?:\s*#\[[^\n]*\]\s*)*\s*(?:pub(?:\([a-z: ]+\))?\s+)?(?:unsafe\s+)?(?:struct|enum|trait|type|const|static|union)\s+([A-Za-z_][A-Za-z0-9_]*)')
fnrx=re.compile(r'(?m)^(?:pub(?:\([a-z: ]+\))?\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+([a-z_][a-z0-9_]*)')
defs=collections.defaultdict(set)
for f,t in text.items():
    for m in defrx.finditer(t): defs[m.group(1)].add(f)
    for m in fnrx.finditer(t):
        if "_" in m.group(1): defs[m.group(1)].add(f)
idrx=re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
edges=collections.defaultdict(lambda: collections.Counter())
for f,t in text.items():
    ids=collections.Counter(idrx.findall(t))
    for n,c in ids.items():
        if n in defs and f not in defs[n] and len(n)>=6:
            for g in defs[n]:
                edges[f][g]+=c
_pref=os.path.commonpath([os.path.dirname(f) for f in files]) if len(files)>1 else ""
short=lambda p:p[len(_pref)+1:][:60]
tot=collections.Counter()
for f in files:
    out_=sum(edges[f].values()); inn=sum(edges[g][f] for g in files)
    print(f"{os.path.getsize(os.path.join(R,f)):6d} out={out_:4d} in={inn:4d} {short(f)}")


def explain(sel):
    S=[f for f in files if any(s in f for s in sel)]
    print("S=",[short(f) for f in S], sum(os.path.getsize(os.path.join(R,f)) for f in S))
    print("-- S -> complement names")
    for f in S:
        used=collections.Counter()
        for g,c in edges[f].items():
            if g in S: continue
            names=[n for n,fs in defs.items() if g in fs and re.search(r'\b'+n+r'\b',text[f]) and f not in fs and len(n)>=6]
            for n in names: used[(short(g)[:28],n)]+=1
        print(short(f)[:44], dict(list(used.items())[:14]))
    print("-- complement -> S")
    for f in files:
        if f in S: continue
        for g,c in edges[f].items():
            if g in S:
                names=[n for n,fs in defs.items() if g in fs and re.search(r'\b'+n+r'\b',text[f]) and f not in fs and len(n)>=6]
                print(short(f)[:44],"->",short(g)[:36],names[:6])
if len(sys.argv)>3: explain(sys.argv[3].split(","))

if os.environ.get("FAM"):
    depth=int(os.environ["FAM"])
    def fam(f):
        parts=short(f).split("/")
        return "/".join(parts[:depth]) if len(parts)>depth else parts[0]
    fams=collections.OrderedDict()
    for f in files: fams.setdefault(fam(f),[]).append(f)
    fe=collections.defaultdict(collections.Counter)
    for f in files:
        for g,c in edges[f].items():
            if fam(f)!=fam(g): fe[fam(f)][fam(g)]+=c
    for k,fs in fams.items():
        sz=sum(os.path.getsize(os.path.join(R,x)) for x in fs)
        print(f"{k:34s} {sz:7d} n={len(fs):2d} uses:",dict(fe[k]))
