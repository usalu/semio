# 🔬 Derives a mutationDomainOwners row (domain -> {leaf directory: mutation id}) from the leaf descriptors of one schema mutations root. usage: python3 derive-domain-owner.py <mutations-root>
import json,os,sys
root=sys.argv[1]
row={}
bad=[]
for dom in sorted(os.listdir(root)):
    dp=os.path.join(root,dom)
    if not os.path.isdir(dp) or dom in ("🧪️tests","📦️packages"): continue
    if os.path.isfile(os.path.join(dp,"🔣️.json")): continue
    leaves={}
    for leaf in sorted(os.listdir(dp)):
        f=os.path.join(dp,leaf,"🔣️.json")
        if not os.path.isfile(f): continue
        d=json.load(open(f,encoding="utf8"))
        ident=d.get("semanticIdentity") or d.get("identity") or d.get("mutationId")
        leaves[leaf]=ident
        if not ident: bad.append((dom,leaf,list(d.keys())))
    if leaves: row[dom]=leaves
print(json.dumps(row,ensure_ascii=False,indent=1)[:1500]); print(bad[:3])
