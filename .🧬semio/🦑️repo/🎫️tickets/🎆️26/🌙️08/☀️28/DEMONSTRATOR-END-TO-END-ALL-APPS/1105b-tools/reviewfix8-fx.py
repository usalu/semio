import re,sys,os
log=sys.argv[1];root=sys.argv[2]
txt=open(log).read().split('\n')
sites={}
i=0
while i<len(txt):
    if txt[i].startswith('error') :
        j=i+1;loc=None;found=False
        while j<len(txt) and not txt[j].startswith('error') and j<i+14:
            m=re.match(r'\s*--> (.*):(\d+):(\d+)$',txt[j])
            if m and loc is None: loc=(m.group(1),int(m.group(2)),int(m.group(3)))
            if 'expected `&mut RetainedAllocationEffect' in txt[j] and 'found `RetainedCloneGrant`' in txt[j]: found=True
            j+=1
        if loc and found and 'operation-wire' in loc[0]:
            f=os.path.normpath(os.path.join(root,'📦️packages/🦀️rust',loc[0].split('📦️packages/🦀️rust/')[1])) if '📦️packages/🦀️rust/' in loc[0] else None
            if f: sites.setdefault(f,[]).append((loc[1],loc[2]))
    i+=1
def balanced_end(s,k):
    d=0
    while k<len(s):
        c=s[k]
        if c in '([{': d+=1
        elif c in ')]}':
            if d==0: return k
            d-=1
        elif c==',' and d==0: return k
        k+=1
    return None
for f,locs in sites.items():
    if not any(x in f for x in ('🎯️common','batch','envelope','⚖️canonical')): continue
    lines=open(f).read().split('\n')
    for ln in sorted(set(l for l,_ in locs)):
        s=lines[ln-1]
        for col in sorted([c for l,c in locs if l==ln],reverse=True):
            a=col-1;e=balanced_end(s,a)
            arg=s[a:e]
            # find call open paren
            d=0;k=a-1
            while k>=0:
                c=s[k]
                if c in ')]}': d+=1
                elif c in '([{':
                    if d==0: break
                    d-=1
                k-=1
            assert s[k]=='(',(f,ln,col)
            m=re.search(r'([\w.]+)\.(\w+)$',s[:k]);assert m,(f,ln,col,s[max(0,k-30):k])
            start=m.start();d=0;q=k
            while True:
                c=s[q]
                if c in '([{': d+=1
                elif c in ')]}':
                    d-=1
                    if d==0: break
                q+=1
            call=s[start:q+1]
            ca=a-start;ce=e-start
            newcall=call[:ca]+'e'+call[ce:]
            wrapped='crate::effect_fixture::fx('+arg+',|e|'+newcall+')'
            if re.match(r'^(source|close)\.(close_step|step)$',m.group(1)+'.'+m.group(2)) and s[q+1:].startswith(('.result','.unwrap()')):
                wrapped='crate::effect_fixture::settled('+wrapped+')'
            s=s[:start]+wrapped+s[q+1:]
        lines[ln-1]=s
    open(f,'w').write('\n'.join(lines))
    print(f,len(locs))
