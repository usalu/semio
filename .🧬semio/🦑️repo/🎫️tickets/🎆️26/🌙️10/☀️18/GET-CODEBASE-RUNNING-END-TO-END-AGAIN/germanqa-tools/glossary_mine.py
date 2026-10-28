import json,re,collections,math,sys
r=json.load(open(sys.argv[1]))
pairs=list({(x["en"],x["de"]) for x in r if x["en"] and x["de"] and x["en"]!=x["de"]})
def tok(s,de=False):
    s=re.sub(r"\{[^}]*\}"," ",s); s=re.sub(r"`[^`]*`"," ",s)
    return re.findall(r"[A-Za-zÄÖÜäöüß]{3,}",s)
terms=sys.argv[3:]
N=len(pairs)
detoks=[set(w.lower() for w in tok(d)) for e,d in pairs]
# also compound-substring: de token contains candidate stem? use token set only
df=collections.Counter(w for s in detoks for w in s)
out={}
for t in terms:
    rx=re.compile(r"\b"+t,re.I)
    idx=[i for i,(e,d) in enumerate(pairs) if rx.search(re.sub(r"`[^`]*`"," ",e))]
    if len(idx)<2: out[t]=(len(idx),[]); continue
    c=collections.Counter(w for i in idx for w in detoks[i])
    sc=[]
    for w,n in c.items():
        if n<2: continue
        sc.append((n/ math.sqrt(len(idx)*df[w]), n, w, df[w]))
    sc.sort(reverse=True)
    out[t]=(len(idx),[f"{w}:{n}/{d}" for s,n,w,d in sc[:7]])
for t,(n,l) in out.items(): print(f"{t} [{n}] -> "+"  ".join(l))
