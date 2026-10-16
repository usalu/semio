import re,sys,collections
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/"
pat=re.compile(r'^(?P<f>[^ ].*?)\((?P<l>\d+),(?P<c>\d+)\): error (?P<code>TS\d+): (?P<m>.*)$')
def load(p):
    out=collections.Counter()
    for line in open(G+p,encoding="utf8"):
        m=pat.match(line.rstrip("\n"))
        if m: out[(m['f'],m['code'],m['m'][:100])]+=1
    return out
b=load(sys.argv[1]);a=load(sys.argv[2])
new=a-b;gone=b-a
print("new",sum(new.values()),"gone",sum(gone.values()))
def short(f): return "/".join(f.split("/")[-4:])
cn=collections.Counter();cg=collections.Counter()
for (f,c,m),n in new.items(): cn[(f.split('/')[1] if f.startswith('✏️s') else f.split('/')[0], f.split('/')[2] if f.startswith('✏️s') else f.split('/')[1], c)]+=n
for k,v in cn.most_common(30): print("NEW",v,k)
print("--- gone not in my families")
for (f,c,m),n in sorted(gone.items()):
    if c in("TS1205","TS2694") or "Readonly<Record" in m or "puzzle" in f and "🧊️3d" in f: continue
    if c in("TS2305","TS2303","TS2459","TS2307"): continue
    print("GONE",n,short(f),c,m[:80])
