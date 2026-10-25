import json,sys,collections,re,os
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log=sys.argv[1]; pkg=sys.argv[2]; lim=int(sys.argv[3]) if len(sys.argv)>3 else 12; kind=sys.argv[4] if len(sys.argv)>4 else None
seen=set();n=0
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    p=r["package_id"].split("#")[-1].split("@")[0] if "#" in r["package_id"] else r["package_id"]
    if p!=pkg: continue
    if kind and kind not in r["target"]["kind"]: continue
    sp=[s for s in m["spans"] if s["is_primary"]]
    loc=(sp[0]["file_name"].split("📦️packages/🦀️rust/")[-1][-70:]+":"+str(sp[0]["line_start"])) if sp else "?"
    k=(loc,m["message"])
    if k in seen: continue
    seen.add(k); n+=1
    if n<=lim:
        print(loc,"|",(m.get("code") or {}).get("code",""),m["message"][:200])
        for ch in m.get("children",[])[:3]:
            print("      ",ch["level"],ch["message"][:160])
print("unique",n)
