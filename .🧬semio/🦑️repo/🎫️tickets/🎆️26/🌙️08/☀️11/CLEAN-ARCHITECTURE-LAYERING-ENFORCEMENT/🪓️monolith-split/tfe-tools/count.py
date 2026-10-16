import re,sys,collections
G=sys.argv[1]
def load(p):
    return [l.rstrip("\n").split("\t") for l in open(p,encoding="utf8") if l.strip()]
def cls(r):
    f,line,me,tm,kind,tgt,ex=r
    t=re.sub(r"\?raw$","",tgt)
    if t.endswith((".rs",".toml",".md",".txt")): return None
    if ex!="exists" and not tgt.endswith("?raw"): return None
    if kind=="fw": return "sanctioned-source-text-api"
    return "fixture-or-data"
for label in sys.argv[2:]:
    rows=load(G+"/tfe-scan-"+label+".tsv")
    c=collections.Counter()
    for r in rows:
        k=cls(r)
        if not k: continue
        cons=r[0]
        area=("hub" if cons.startswith("🌎️hub") else "plugins(✏️s)" if cons.startswith("✏️s") else "library" if "📚️library" in cons else "framework/os")
        c[(k,area)]+=1
    print(label, dict(c), "total existing non-source reads:", sum(v for (k,a),v in c.items() if k=="fixture-or-data"))
