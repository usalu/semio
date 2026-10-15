import sys,os,re,json,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import scan as S, ho
REPO=ho.REPO
def norm(l):
    l=l.strip()
    if not l: return None
    l=re.sub(r"\bpub\((super|crate|self|in [^)]*)\) ","",l)
    l=l.replace("super::super::","super::")
    return l
def lines(path):
    out=collections.Counter()
    for l in open(path,encoding="utf-8").read().split("\n"):
        n=norm(l)
        if n is not None: out[n]+=1
    return out
def run(t):
    sp=ho.SPEC[t]
    fams=sp["fam"] if isinstance(sp["fam"],list) else [sp["fam"]]
    mods,order,r=S.load_tree(REPO+"/"+sp["root"])
    A=collections.Counter(); B=collections.Counter()
    for k,v in sp["baseline"].items():
        if any(re.search(f,k) for f in fams): A+=lines(v)
    if not sp["baseline"]: return
    files=[]
    for m in order:
        p=os.path.relpath(os.path.realpath(m.path),REPO)
        if any(re.search(f,p) for f in fams): B+=lines(m.path); files.append(p)
    miss=A-B; extra=B-A
    print(f"[{t}] baseline lines {sum(A.values())} split-family lines {sum(B.values())} files {len(files)} | baseline-lines-missing-in-split {sum(miss.values())} | split-extra {sum(extra.values())}")
    for l,n in list(miss.items())[:int(os.environ.get('SHOW','12'))]: print("   MISSING",n,l[:150])
    if os.environ.get("EXTRA"):
        for l,n in list(extra.items())[:int(os.environ.get('SHOW','12'))]: print("   EXTRA",n,l[:150])
for t in sys.argv[1:]: run(t)
