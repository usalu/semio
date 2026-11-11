import re,sys,collections
G=sys.argv[1]
st=collections.defaultdict(lambda:[set(),set()])
for l in open(G+"/alloc-static.log"):
    p=l.rstrip("\n").split(" ",2)
    if p[0] in("CONFLICT","ok"):st[p[2]][p[0]=="CONFLICT"].add(p[1])
mixed=[f for f,(o,c) in st.items() if c and o]
allc=[f for f,(o,c) in st.items() if c and not o]
print("mixed",len(mixed),"allconflict",len(allc))
for f in mixed:print("MIXED",f[-70:],sorted(st[f][0]),sorted(st[f][1]))
open(G+"/alloc-files.txt","w").write("\n".join(allc))
# non cfg(test) declarations in non-tests dirs
for f in allc:
    s=open(f,errors="ignore").read()
    if "🧪️tests" in f or "tests" in f.rsplit("/",1)[-1]:continue
    for m in re.finditer(r'(.{0,40})#\[global_allocator\]',s,re.S):
        print("NONTEST",f[-90:],repr(m.group(1)[-40:]))
