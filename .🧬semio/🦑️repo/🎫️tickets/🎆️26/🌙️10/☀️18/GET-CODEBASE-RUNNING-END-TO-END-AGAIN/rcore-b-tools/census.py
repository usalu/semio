import re,sys,collections
t=open(sys.argv[1],errors="replace").read().split("\n")
seen=set();rows=collections.OrderedDict()
for l in t:
    m=re.match(r"error: could not compile `([^`]+)` \((lib|lib test|bin[^)]*|test[^)]*)\) due to (\d+)",l)
    if m: rows[(m.group(1),m.group(2))]=int(m.group(3))
errs=collections.defaultdict(collections.Counter)
for l in t:
    m=re.match(r"(.*?\.rs):(\d+):(\d+): error(\[E\d+\])?: (.*)",l)
    if m:
        errs[m.group(1).split("/../")[0]][(m.group(4) or "")+" "+m.group(5)[:110]]+=1
for (c,k),n in sorted(rows.items()): print(c,k,n)
print("TOTAL",len(rows))
if len(sys.argv)>2:
    for f,c in errs.items():
        print("##",f[-120:])
        for k,v in c.most_common(4): print("   ",v,k)
