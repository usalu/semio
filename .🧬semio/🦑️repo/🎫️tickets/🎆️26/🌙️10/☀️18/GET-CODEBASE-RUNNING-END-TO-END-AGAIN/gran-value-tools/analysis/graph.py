"""🕸️ Scoped crate graph over the audit manifest parse (`crates.json`), value-family selectors and transitive dependents. Run every analysis script with `cwd` = the generated folder."""
import json, collections, sys, os
GEN = os.environ.get("GRAN_GEN") or os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../🗑️generated/gran-value"))
g=json.load(open(GEN+('/crates2.json' if os.path.exists(GEN+'/crates2.json') else '/crates.json')))
C=g['crates']
byname={}
for d,c in C.items():
    if c['name']: byname[c['name']]=d
dirname={d:c['name'] for d,c in C.items()}
def edges(kinds=('normal','build')):
    out=collections.defaultdict(set)
    for d,c in C.items():
        for k in kinds:
            for dn,t in c['deps'][k]:
                if t in C: out[d].add(t)
    return out
E_=edges()
rev=collections.defaultdict(set)
for a,bs in E_.items():
    for b in bs: rev[b].add(a)
def trans(start, rev=rev):
    seen=set(); st=[start]
    while st:
        x=st.pop()
        for y in rev.get(x,()):
            if y not in seen: seen.add(y); st.append(y)
    return seen
V='🧰️framework/🔨️modules/🌱️value'
fam=[d for d in C if d.startswith(V+'/')or d==V+'/📦️packages/🦀️rust']
if __name__=='__main__':
    rows=[]
    for d in fam:
        rows.append((len(trans(d)),len(rev.get(d,())),C[d]['name'],d[len(V)+1:-len('/📦️packages/🦀️rust')] if d.endswith('/📦️packages/🦀️rust') else d, C[d]['proc_macro']))
    rows.sort(reverse=True)
    for r in rows: print(r)
    print(len(fam))
