import sys,os,re
d=sys.argv[1]
for n in sorted(os.listdir(d)):
    t=open(os.path.join(d,n),errors="replace").read()
    pf=re.findall(r"^\s*(\d+) (pass|fail)\b",t,re.M)
    print("=====",n,pf[-2:])
    k=0
    for l in t.splitlines():
        s=l.strip()
        if s.startswith(("error:","(fail)","ReferenceError","TypeError","SyntaxError")) or "Expected" in s[:10] or "Received" in s[:10]:
            print("  ",s[:260]); k+=1
            if k>int(sys.argv[2] if len(sys.argv)>2 else 8): break
