import json,sys,re
log=sys.argv[1]; pkg=sys.argv[2]; skip=set(sys.argv[3].split("|")) if len(sys.argv)>3 and sys.argv[3] else set(); lim=int(sys.argv[4]) if len(sys.argv)>4 else 20; width=int(sys.argv[5]) if len(sys.argv)>5 else 900
seen=set();n=0
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    pid=r["package_id"]; p=pid.split("#")[-1].split("@")[0] if "#" in pid else pid
    if not p.endswith(pkg): continue
    if any(x and x in m["message"] for x in skip): continue
    sp=[s for s in m["spans"] if s["is_primary"]]
    k=(sp[0]["file_name"],sp[0]["line_start"],sp[0]["column_start"],m["message"]) if sp else m["message"]
    if k in seen: continue
    seen.add(k); n+=1
    if n>lim: continue
    t=re.sub(r"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/","@P/",m["rendered"])
    t=re.sub(r"(  *= note: the full name[^\n]*\n|  *= note: consider using `--verbose`[^\n]*\n)","",t)
    print(t[:width]); print("-----")
print("shown",min(n,lim),"of",n)
