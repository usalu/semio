# 🔬 Measures how many unpaired fixture bundles a subset's 🥒️.feature declares. usage: python3 feature-coverage.py <unpaired.json> <repoRoot>
import json,os,re,sys,glob,collections
d=json.load(open(sys.argv[1],encoding="utf8")); repo=sys.argv[2]
M="/🧪️tests/🧫️fixtures/🧬️mutations/"
fold=lambda s: s.replace("️","").strip()
cache={}
def declared(sub):
    if sub in cache: return cache[sub]
    refs=set()
    td=os.path.join(repo,sub,"🧪️tests")
    for root,dirs,files in os.walk(td):
        if "🧫️fixtures" in root: dirs[:]=[]; continue
        for f in files:
            if f.endswith("🥒️.feature") or f.endswith(".feature"):
                txt=open(os.path.join(root,f),encoding="utf8",errors="replace").read()
                for m in re.finditer(r"shared://🧬️mutations/([^\s|]+)",txt):
                    refs.add(fold(m.group(1)).rstrip("/"))
                hdr=None
                for line in txt.splitlines():
                    s=line.strip()
                    if s.startswith("|"):
                        cells=[c.strip() for c in s.strip("|").split("|")]
                        if hdr is None: hdr=cells; continue
                        if "dir" in hdr and "fixture" in hdr:
                            refs.add(fold(cells[hdr.index("dir")])+"/"+fold(cells[hdr.index("fixture")]))
                    else: hdr=None
    cache[sub]=refs; return refs
tot=collections.Counter()
for cat,ps in d.items():
    for p in ps:
        sub,rel=p.split(M,1)
        refs=declared(sub)
        r=fold(rel)
        hit=any(x==r or x.startswith(r+"/") or x.endswith("/"+r) for x in refs) 
        tot[(cat,hit)]+=1
for k,v in sorted(tot.items()): print(k,v)
