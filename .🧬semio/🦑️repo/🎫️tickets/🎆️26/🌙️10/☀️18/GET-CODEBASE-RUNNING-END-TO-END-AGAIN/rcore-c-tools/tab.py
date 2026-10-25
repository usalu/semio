import json,collections,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log=sys.argv[1]
per=collections.Counter();seen=set();msgs=collections.defaultdict(list)
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    pid=r["package_id"]; p=pid.split("#")[-1].split("@")[0] if "#" in pid else pid
    sp=[s for s in m["spans"] if s["is_primary"]]
    loc=(sp[0]["file_name"],sp[0]["line_start"],sp[0]["column_start"]) if sp else None
    k=(p,loc,m["message"])
    if k in seen: continue
    seen.add(k); per[p]+=1; msgs[p].append(m["message"][:100])
tot=0
for p,n in sorted(per.items(),key=lambda x:-x[1]): print(n,p); tot+=n
print("total",tot,"units",len(per))
