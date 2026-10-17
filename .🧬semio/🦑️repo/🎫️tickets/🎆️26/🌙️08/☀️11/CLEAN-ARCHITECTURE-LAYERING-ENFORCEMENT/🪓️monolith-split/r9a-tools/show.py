import json,re,sys,collections
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
print(d['ms'],d['packing'])
for p in d['problems']: print("PROBLEM",p[:300])
for c in d['crates']: print(c['name'][27:],c['lib'],c['test'],len(c['attach']),[x[27:] for x in c['deps']])
for n in d['notes']:
    if n.startswith("runtime fixtures") or n.startswith("made ") or n.startswith("lifted") or n.startswith("kept"): continue
    print("note",n[:300])
print("kept notes:",sum(1 for n in d['notes'] if n.startswith("kept")))
for b in d['blockers']:
    print({k:(v if not isinstance(v,list) else len(v)) for k,v in b.items()})
    pat=collections.Counter()
    for x in b['causes']:
        m=re.match(r"(coherence|cycle)(?:\((.*)\))?: (.*) <-> (.*)",x)
        t=re.sub(r"standards::v1::subsets::any::","",m.group(2) or "cycle")
        pat[t]+=1
    for t,n in pat.most_common(25): print("  ",n,t[:160])
    print([g[:200] for g in b['groupReport'] if g.startswith("group") and int(re.search(r"files=(\d+)",g).group(1))>=8])
