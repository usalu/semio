import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][0]
par={}
def find(x):
    while par.get(x,x)!=x:
        par[x]=par.get(par[x],par[x]); x=par[x]
    return x
for c in b['causes']:
    m=re.match(r"coherence\((.*)\): (.*) <-> (.*)",c)
    if m:
        a,c2=find(m.group(2)),find(m.group(3))
        if a!=c2: par[c2]=a
want=sys.argv[2]; mode=sys.argv[3] if len(sys.argv)>3 else "out"
rows=collections.defaultdict(list)
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c=find(m.group(1)),find(m.group(2))
    if a==c: continue
    key,other=(a,c) if mode=="out" else (c,a)
    if want in key: rows[other].append(((m.group(1)[-50:]),("T" if m.group(3) else "L"),(m.group(4) or "")[:150]))
for k,v in sorted(rows.items(),key=lambda kv:-len(kv[1]))[:int(os.environ.get("N","25"))]:
    print(len(v),k[-80:]); print("     e.g.",v[0])
