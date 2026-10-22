import re,sys
log,crate=sys.argv[1],sys.argv[2]
errs=[];seen=set()
for l in open(log,errors='replace'):
    l=l.rstrip('\n')
    if re.match(r'^[^ \[].*:\d+:\d+: error',l) and l not in seen:
        seen.add(l);errs.append(l)
    m=re.match(r'^error: could not compile `([^`]+)` \(([^)]*)\)',l)
    if m:
        if m.group(1)==crate:
            print(f'## {crate} ({m.group(2)})')
            for e in errs:
                mm=re.match(r'^(.*?)(([^/]*/){0,3}[^/]*🦀️\.rs:\d+:\d+): (.*)$',e)
                print('  ',(mm.group(2)+': '+mm.group(4)) if mm else e[-300:])
        errs=[]
