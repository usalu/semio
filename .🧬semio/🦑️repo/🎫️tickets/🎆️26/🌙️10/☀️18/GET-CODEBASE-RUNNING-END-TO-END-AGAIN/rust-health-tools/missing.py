import re,sys
log=open(sys.argv[1],errors='replace').read().split('\n')
errs=[];res={}
for l in log:
    if re.match(r'^[^ \[].*:\d+:\d+: error',l): errs.append(l)
    m=re.match(r'^error: could not compile .([^`]+). \(([^)]*)\)',l)
    if m:
        res.setdefault((m.group(1),m.group(2)),[]).extend(errs); errs=[]
for k,e in res.items():
    s=set()
    for x in e:
        for n in re.findall(r'(?:cannot find|unresolved import|use of unresolved module or unlinked crate) [^:]*?`((?:semio_framework|protocol|serde|blake3|tokio|ui_)[a-z_0-9]*)`',x): s.add(n)
    if s: print(k,sorted(s))
