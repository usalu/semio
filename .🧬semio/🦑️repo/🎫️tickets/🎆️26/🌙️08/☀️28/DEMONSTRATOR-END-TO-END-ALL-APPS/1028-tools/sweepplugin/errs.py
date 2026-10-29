import sys,re
lines=open(sys.argv[1],errors="replace").read().split("\n")
seen=set()
for i,l in enumerate(lines):
    if l.startswith("error") and not l.startswith("error: could not compile") and not l.startswith("error: aborting"):
        loc=""
        for j in range(i+1,min(i+4,len(lines))):
            if "-->" in lines[j]: loc=lines[j].strip()[4:]; break
        loc=re.sub(r'/\.\./\.\./','/../../',loc)
        k=(l,loc)
        if k in seen: continue
        seen.add(k); print(l[:200],"@",loc[-110:])
    elif l.startswith("error: could not compile"): print(l[:200])
