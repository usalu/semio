import json,sys,re,os
log=sys.argv[1]; pkgs=set(sys.argv[2].split(",")); lim=int(sys.argv[3]) if len(sys.argv)>3 else 40
full=len(sys.argv)>4
seen=set();n=0
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    pid=r["package_id"]; p=pid.split("#")[-1].split("@")[0] if "#" in pid else pid
    if p not in pkgs and not any(p.endswith(x) for x in pkgs): continue
    sp=[s for s in m["spans"] if s["is_primary"]]
    loc=(sp[0]["file_name"],sp[0]["line_start"],sp[0]["column_start"]) if sp else ("?",0,0)
    k=(p,loc,m["message"])
    if k in seen: continue
    seen.add(k); n+=1
    if n>lim: continue
    f=re.sub(r".*packages/🦀️rust/(\.\./)*","",loc[0])
    print(f"[{p.replace('semio-framework-','')}] {f}:{loc[1]}:{loc[2]} {(m.get('code') or {}).get('code','')} {m['message'][:240]}")
    if full:
        for ch in m.get("children",[])[:4]:
            print("     ",ch["level"],ch["message"][:200], [ (s.get('suggested_replacement') or '')[:80] for s in ch.get("spans",[])][:2])
print("shown",min(n,lim),"of",n)
