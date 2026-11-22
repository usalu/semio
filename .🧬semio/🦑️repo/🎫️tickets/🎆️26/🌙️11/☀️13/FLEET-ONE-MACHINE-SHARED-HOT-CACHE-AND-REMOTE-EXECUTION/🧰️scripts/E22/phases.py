import sys,re
spans=[];cur=None
for line in open(sys.argv[1],errors="replace"):
    m=re.match(r"\s*([\d.]+) (.*)$",line.rstrip("\n"))
    if not m: continue
    t,msg=float(m[1]),m[2]
    if "still working" in msg or "still running" in msg: continue
    r=re.search(r'"resource":"([^"]*)"',msg)
    if r:
        n=r[1]
        key="nx-sdk-derived-namespace" if n.startswith("nx-sdk-derived") else "nx-authorizers" if n.startswith("nx-authorizers") else re.sub(r"[:\d]+$","",n)
    elif msg.startswith("[script] {"): continue
    elif msg.startswith("[script]") and "·" in msg: continue
    else: key=re.sub(r"\d+","N",msg)[:90]
    if cur and cur[0]==key: cur[2]=t
    else:
        cur=[key,t,t];spans.append(cur)
for k,a,b in spans: print("%7.1f %6.1f  %s"%(a,b-a,k))
print("last event at",spans[-1][2] if spans else None)
