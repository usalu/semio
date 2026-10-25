"""python3 partition-census.py <closure-list.txt> : candidate scheduling prefixes vs closure with recent-edit counts"""
import sys
F="🧰️framework/"
P=F+"🔨️modules/🏃️process/"
L=F+"🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
CAND={
 "leases":[P+"🔒️leases/",P+"📦️packages/🟦️typescript/🔒️leases/"],
 "capacity":[P+"🏗️build/🎟️capacity/",P+"🏗️build/🌡️host-capacity/",P+"📦️packages/🟦️typescript/🏗️build/🎟️capacity/",B+"🎟️capacity/",B+"🌡️host-capacity/"],
 "store-custody":[P+"📦️artifacts/🏗️native-build/",P+"📦️artifacts/💾️cache/",P+"📦️packages/🟦️typescript/📦️artifacts/",L+"⚡️caching/💾️storage/"],
 "process-state":[P+"💾️state/",P+"📦️packages/🟦️typescript/💾️state/",P+"🪪️agent-sessions/",P+"🪓️termination/"],
 "invocation-envelope":[P+"🧭️routing/📥️invocation/",P+"📦️packages/🟦️typescript/🧭️routing/📥️invocation/"],
 "churn-observation":[F+"🔨️modules/📁️filesystem/🧾️observation/🔁️churn/"],
 "graph-generations":[B+"🕸️graph/🕰️generations/"],
}
rows=[l.rstrip("\n").split(None,2) for l in open(sys.argv[1])]
tot=0;ov=0;rec=0;recov=0
dom={}
keyed=[]
for a,s,p in rows:
    a=float(a[:-1]);tot+=1
    hit=None
    for d,pre in CAND.items():
        if any(p.startswith(x) for x in pre): hit=d;break
    if hit: ov+=1;dom.setdefault(hit,[0,0]);dom[hit][0]+=1;dom[hit][1]+=a<24
    else: keyed.append((a,p))
print("closure",tot,"scheduling",ov,"keyed",tot-ov)
for d,(n,r) in dom.items(): print(f"  {d}: {n} files, {r} edited <24h")
print("keyed edited <24h:",sum(a<24 for a,p in keyed),"<72h:",sum(a<72 for a,p in keyed))
for a,p in sorted(keyed)[:40]: print(f"  {a:6.1f}h {p.replace(F,'')[:170]}")
