import json,sys,collections,os
log=sys.argv[1]; cut=float(sys.argv[2])
sug=collections.defaultdict(set)
for l in open(log,errors="replace"):
    if not l.startswith("{"): continue
    try:r=json.loads(l)
    except: continue
    if r.get("reason")!="compiler-message": continue
    m=r["message"]
    if m["level"]!="error": continue
    for ch in m["children"]:
        if "add the `dyn` keyword" in ch["message"]:
            for s in ch["spans"]:
                if s.get("suggested_replacement")=="dyn ": sug[s["file_name"]].add(s["byte_start"])
n=0
for f,offs in sug.items():
    if not os.path.exists(f) or os.path.getmtime(f)>cut: print("skip",f[-60:]); continue
    b=open(f,"rb").read()
    for o in sorted(offs,reverse=True):
        b=b[:o]+b"dyn "+b[o:]; n+=1
    open(f,"wb").write(b)
print("inserted",n)
