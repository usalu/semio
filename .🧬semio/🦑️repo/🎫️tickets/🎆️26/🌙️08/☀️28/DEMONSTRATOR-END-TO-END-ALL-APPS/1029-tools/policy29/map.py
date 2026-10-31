import json,sys,os,glob
G=sys.argv[1]; ROOT="/Users/ueli/Documents/semio"; TK=sys.argv[2]
d=json.load(open(os.path.join(G,"policy29-findings.json")))
F=d['findings']
untr=set(open(os.path.join(G,"policy29-untracked.txt"),encoding="utf-8").read().split("\n"))
for f in F:
    if f['file'] in untr: f['new']=True
res={os.path.basename(p)[len("1029-result-"):-3]:open(p,encoding="utf-8",errors="replace").read() for p in glob.glob(os.path.join(TK,"1029-result-*.md"))}
def crate_dir(p):
    parts=p.split("/")
    for i in range(len(parts)-1,0,-1):
        if os.path.isfile(os.path.join(ROOT,*parts[:i],"Cargo.toml")): return "/".join(parts[:i])
    return None
newfiles=sorted(set(f['file'] for f in F if f['new'] and not f['cat'].startswith('9')))
for p in newfiles:
    full=[k for k,t in res.items() if p in t]
    cd=crate_dir(p)
    cdh=[k for k,t in res.items() if cd and len(cd.split('/'))>=3 and cd in t]
    print(f"{p[:150]} | full={full} | crate={cdh} ({(cd or '')[-70:]})")
