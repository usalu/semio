import os,re,sys,collections
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rows=sorted({l.strip().rsplit(":",1)[0] for l in open(C+"/🗑️generated/1101/aud-policy-unwrap-rows.txt")})
extra=["🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs","🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🤝️kerning/🦀️.rs","🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs"]
def find(f):
    d=os.path.dirname(f)
    while d:
        for cand in (d+"/📦️packages/🦀️rust/Cargo.toml",d+"/Cargo.toml"):
            if os.path.exists(cand):
                return cand
        d=os.path.dirname(d)
res=collections.defaultdict(set)
for f in rows+extra:
    m=find(f)
    if not m:res['?'].add(f);continue
    name=re.search(r'^name\s*=\s*"([^"]+)"',open(m).read(),re.M)
    res[m].add(name.group(1) if name else '?')
ws=collections.defaultdict(set)
for m,n in res.items():
    if m=='?':continue
    d=os.path.dirname(m)
    # workspace root: first ancestor Cargo.toml with [workspace]
    w=None;p=d
    while p:
        c=p+"/Cargo.toml"
        if os.path.exists(c) and p!=d and '[workspace]' in open(c).read():w=c;break
        p=os.path.dirname(p)
    ws[w].update(n)
for w,n in ws.items():print(w,len(n))
import json;json.dump({str(w):sorted(n) for w,n in ws.items()},open(C+"/🗑️generated/1101/policyfix-crates.json","w"),ensure_ascii=False,indent=0)
print(res.get('?'))
