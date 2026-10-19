import re,sys,collections
t=open(sys.argv[1],encoding="utf8").read()
i=t.index("Failed Suites")
t=t[i:]
blocks=re.split(r"\n(?= FAIL  \|)",t)
cls=collections.Counter(); other=[]
for b in blocks[1:]:
    head=b.split("\n")[0]
    m=re.search(r'Failed to resolve import "([^"]+)"',b)
    if m: cls["unresolved "+m.group(1)]+=1; continue
    if "browserType.launch" in b: cls["playwright missing"]+=1; continue
    other.append(head[:170]+"\n   "+"\n   ".join([l for l in b.split("\n")[1:6] if l.strip()])[:500])
for k,v in cls.most_common(): print(v,k)
print(len(other),"other")
for o in other[:40]: print(o)
