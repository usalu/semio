import re,sys
s=open(sys.argv[1],encoding="utf8").read()
blocks=re.split(r'\n(?=(?:error|warning)(?:\[E\d+\])?: )',s)
cur=[];out=[]
for b in blocks:
    f=b.split("\n")[0]
    if f.startswith("error: could not compile"):
        name=re.search(r'`([^`]+)` \(([^)]*)\)',f)
        out.append((name.group(1).replace("semio-s-artifact-","")+" "+name.group(2),cur));cur=[]
    elif f.startswith("error") and "aborting" not in f: cur.append(b)
for k,c in out:
    print("##",k,len(c))
    seen=set()
    for e in c:
        m=re.search(r'-->\s+(\S+):(\d+)',e); t=e.split("\n")[0][:120]
        if t in seen: continue
        seen.add(t)
        if len(seen)<=int(sys.argv[2] if len(sys.argv)>2 else 4): print("   ",t,"|",(m.group(1)[-60:]+":"+m.group(2)) if m else "")
