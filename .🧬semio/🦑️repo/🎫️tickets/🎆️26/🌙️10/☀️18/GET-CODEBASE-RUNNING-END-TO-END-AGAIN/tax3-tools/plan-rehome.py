# 🔬 Plans the re-home of orphan mutation fixtures into the domain-nested layout of their schema mutation tree. usage: python3 plan-rehome.py <unpaired-before.json> <repoRoot> <plan.json>
import json,os,sys,collections
d=json.load(open(sys.argv[1],encoding="utf8")); repo=sys.argv[2]
M="/🧪️tests/🧫️fixtures/🧬️mutations/"
fold=lambda s:s.replace("️","")
plan=[];bad=[]
idx={}
def leaves(sub):
    if sub in idx: return idx[sub]
    root=os.path.join(repo,sub,"🧬️schema/🧬️mutations"); m=collections.defaultdict(list)
    for dom in os.listdir(root) if os.path.isdir(root) else []:
        dp=os.path.join(root,dom)
        if not os.path.isdir(dp): continue
        for leaf in os.listdir(dp):
            if os.path.isdir(os.path.join(dp,leaf)) and leaf not in ("🧪️tests","📦️packages","🧬️schema","🔺️diff","↩️inverse","🦠️mutation"):
                m[fold(leaf)].append(f"{dom}/{leaf}")
        m[("dom",fold(dom))].append(dom)
    idx[sub]=m; return m
seen=set()
for p in d["orphan-no-schema-mutation"]:
    sub,rel=p.split(M,1); parts=rel.split("/")
    flat=parts[0]; rest="/".join(parts[1:])
    key=(sub,flat)
    c=leaves(sub).get(fold(flat),[])
    if len(parts)>=3: bad.append((p,"already-nested-or-deeper")); continue
    if len(c)!=1: bad.append((p,f"candidates={c}")); continue
    if key in seen: continue
    seen.add(key)
    plan.append({"subset":sub,"from":f"{sub}{M}{flat}","to":f"{sub}{M}{c[0]}","flat":flat,"nested":c[0]})
json.dump({"plan":plan,"bad":bad},open(sys.argv[3],"w"),ensure_ascii=False,indent=1)
print(len(plan),"dirs to move;",len(bad),"unresolved")
for b in bad[:12]: print(b[0][-100:],b[1])
