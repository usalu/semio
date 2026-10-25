import json,sys,collections,os
log=sys.argv[1]; needle=sys.argv[2]; cut=float(sys.argv[3]); apply=len(sys.argv)>4 and sys.argv[4]=="apply"
sug=collections.defaultdict(set)
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    for ch in m["children"]:
        if needle in ch["message"]:
            spans=[s for s in ch["spans"] if s.get("suggested_replacement") is not None]
            if len(spans)==1: sug[spans[0]["file_name"]].add((spans[0]["byte_start"],spans[0]["byte_end"],spans[0]["suggested_replacement"]))
n=0
for f,items in sug.items():
    if not os.path.exists(f) or os.path.getmtime(f)>cut: print("skip",f[-60:]); continue
    b=open(f,"rb").read()
    for s,e,rep in sorted(items,reverse=True):
        print(f[-50:],b[s:e][:30],"->",rep[:40]); 
        if apply: b=b[:s]+rep.encode()+b[e:]
        n+=1
    if apply: open(f,"wb").write(b)
print("edits",n)
