import re,sys,collections
G=sys.argv[1]
files=collections.OrderedDict()
for l in open(G+"/alloc-static.log"):
    p=l.rstrip("\n").split(" ",2)
    if p[0]=="CONFLICT":files.setdefault(p[2],set()).add(p[1])
rx=re.compile(r'((?:#\[cfg\(test\)\]\s*)?)#\[global_allocator\]\s*(?:pub\s+)?static\s+(\w+)\s*:\s*([^=;]+?)\s*=\s*([^;]+);')
cat=collections.Counter();odd=[]
for f,ns in files.items():
    s=open(f,errors="ignore").read()
    ms=rx.findall(s)
    n=s.count("#[global_allocator]")
    if len(ms)!=n:odd.append((f,n,len(ms)));continue
    for m in ms:
        cat[(m[2].replace(" ",""),m[3].replace(" ",""))]+=1
        if "GlobalAlloc for" in s:odd.append((f,"impl GlobalAlloc in file"))
for k,v in cat.most_common():print(v,k)
for o in odd:print("ODD",o)
