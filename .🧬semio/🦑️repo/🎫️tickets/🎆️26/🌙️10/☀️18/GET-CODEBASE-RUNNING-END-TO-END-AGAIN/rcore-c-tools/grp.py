import json,sys,re,collections
log=sys.argv[1]; pkg=sys.argv[2]; lim=int(sys.argv[3]) if len(sys.argv)>3 else 25
seen=set();g=collections.defaultdict(list)
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    pid=r["package_id"]; p=pid.split("#")[-1].split("@")[0] if "#" in pid else pid
    if not p.endswith(pkg): continue
    sp=[s for s in m["spans"] if s["is_primary"]]
    loc=(sp[0]["file_name"],sp[0]["line_start"],sp[0]["column_start"]) if sp else ("?",0,0)
    k=(loc,m["message"])
    if k in seen: continue
    seen.add(k)
    msg=re.sub(r"\d+","N",m["message"])[:150]
    f=re.sub(r".*packages/🦀️rust/(\.\./)*","",loc[0])
    g[msg].append(f"{f[-70:]}:{loc[1]}")
for msg,locs in sorted(g.items(),key=lambda x:-len(x[1]))[:lim]:
    print(len(locs),msg); print("    e.g.",locs[0]," ",locs[-1])
