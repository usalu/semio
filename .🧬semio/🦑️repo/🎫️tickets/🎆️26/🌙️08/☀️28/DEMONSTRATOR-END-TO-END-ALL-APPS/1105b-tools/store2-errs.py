import re,sys
log,crate=sys.argv[1],sys.argv[2]
t=open(log,encoding='utf8',errors='replace').read().split('\n')
start=next(i for i,l in enumerate(t) if 'Checking '+crate+' ' in l)
out=[];i=start
while i<len(t):
    l=t[i]
    if l.startswith('error: could not compile') and crate in l: break
    m=re.match(r'^error(\[E\d+\])?: (.*)',l)
    if m and not l.startswith('error: could not'):
        msg=m.group(2)
        j=i+1
        while j<len(t) and '-->' not in t[j]: j+=1
        loc=re.search(r'--> (.*):(\d+):(\d+)',t[j]) if j<len(t) else None
        ex=[x.strip() for x in t[i+1:i+14] if re.search(r'expected|found|help: ',x)][:3]
        if loc:
            out.append((loc.group(1).split('rust/../../')[-1] if 'rust/../../' in loc.group(1) else loc.group(1),int(loc.group(2)),int(loc.group(3)),msg,ex))
    i+=1
for p,ln,c,msg,ex in out:
    try:
        full=p if p.startswith('/') else None
    except: pass
    print(f"{p}:{ln}:{c} {msg[:110]} | {' ; '.join(e[:140] for e in ex)}")
