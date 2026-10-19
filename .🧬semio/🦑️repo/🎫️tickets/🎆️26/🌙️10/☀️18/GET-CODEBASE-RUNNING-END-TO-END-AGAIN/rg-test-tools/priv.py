import re,os,sys
D=sys.argv[1]; T=sys.argv[2:]
files=[os.path.join(r,f) for r,_,fs in os.walk(D) for f in fs if f.endswith('.rs') and 'tests' not in r and '📦️packages' not in r]
defs={}
for f in files:
    for i,line in enumerate(open(f,encoding='utf8')):
        m=re.match(r'\s*(pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?(fn|struct|enum|const|static|type|trait|mod)\s+([A-Za-z_0-9]+)',line)
        if m: defs.setdefault(m.group(3),[]).append(((m.group(1) or '').strip(),os.path.relpath(f,D),i+1))
for t in T:
    txt=open(os.path.join(D,t),encoding='utf8').read()
    idents=set(re.findall(r'[A-Za-z_][A-Za-z_0-9]*',txt))
    print("==",t)
    for n in sorted(idents&set(defs)):
        v=defs[n]
        if any(x[0]=='' for x in v): print(' ',n,v)
