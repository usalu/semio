import re,sys,collections
errs=[];res=collections.OrderedDict();seen=set()
for l in open(sys.argv[1],errors='replace'):
    l=l.rstrip('\n')
    if re.match(r'^[^ \[].*:\d+:\d+: error',l) and l not in seen:
        seen.add(l);errs.append(l)
    m=re.match(r'^error: could not compile `([^`]+)` \(([^)]*)\)(.*)',l)
    if m:
        k=(m.group(1),m.group(2))
        res.setdefault(k,[]).extend(errs); errs=[]
    if 'Uncaptured' in l or 'broker closed' in l: res.setdefault(('INFRA',l[:60]),[])
print(len(res),'failing units')
for (c,t),e in res.items():
    print(f'\n## {c} ({t}) {len(e)} errors')
    cnt=collections.Counter(re.sub(r'^.*?(\.rs:\d+:\d+): ','',x)[:0]+re.search(r'error(\[E\d+\])?',x).group(0) for x in e)
    for x in e[:int(sys.argv[2]) if len(sys.argv)>2 else 3]: print('  ',x[-260:])
