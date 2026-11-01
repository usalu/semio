import re,sys,collections
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
apply='--apply' in sys.argv
rows=sorted({l.strip().rsplit(":",1)[0] for l in open(C+"/🗑️generated/1101/aud-policy-unwrap-rows.txt")})
fnre=re.compile(r'\bfn\s+(\w+)')
site=re.compile(r'\.(?:ok_or_else\(\|\|ValueError::invariant\((?P<l1>"[^"]*")\)\)|map_err\(\|_\|ValueError::invariant\((?P<l2>"[^"]*")\)\))\?')
def fn_at(t,pos):
    for m in reversed(list(fnre.finditer(t,0,pos))[-6:]):
        k=m.end();d=0
        while k<len(t):
            c=t[k]
            if c=='<':d+=1
            elif c=='>' and t[k-1]!='-':d-=1
            elif c=='(' and d==0:break
            k+=1
        d=0;a=k
        while a<len(t):
            c=t[a]
            if c=='(':d+=1
            elif c==')':
                d-=1
                if d==0:break
            a+=1
        mm=re.match(r'\s*(->\s*(.*?))?\s*(where[^{]*)?\{',t[a+1:a+500],re.S)
        if mm:
            b=a+1+mm.end()-1
            return m.start(),(mm.group(2) or '').strip(),b
    return None
def match_close(t,i):
    d=0
    for k in range(i,len(t)):
        if t[k]=='{':d+=1
        elif t[k]=='}':
            d-=1
            if d==0:return k
    return len(t)
def in_closure(t,start,pos):
    for m in reversed(list(re.finditer(r'\|\|\s*\{|\|[^|{]*\|\s*\{',t[start:pos]))):
        s=start+m.end()-1
        if match_close(t,s)>pos:return True
    return False
def chain_start(t,i):
    k=i
    while k>0:
        c=t[k-1]
        if c in ')]':
            op='(' if c==')' else '['
            d=0;j=k-1
            while j>=0:
                if t[j]==c:d+=1
                elif t[j]==op:
                    d-=1
                    if d==0:break
                j-=1
            k=j;continue
        if c.isalnum() or c in '_.':k-=1;continue
        if c==':' and k>1 and t[k-2]==':':k-=2;continue
        break
    return k
tot=0
for f in rows:
    t=open(f).read()
    edits=[]
    for m in site.finditer(t):
        info=fn_at(t,m.start())
        if not info:continue
        s,ret,b=info
        base=re.sub(r'<.*','',ret,flags=re.S).strip()
        if not base.endswith('RetainedWorkOutcome'):continue
        if in_closure(t,b,m.start()):continue
        lit=m.group('l1') or m.group('l2')
        cs=chain_start(t,m.start())
        chain=t[cs:m.start()]
        arm=('Some(v)=>v,None' if m.group('l1') else 'Ok(v)=>v,Err(_)')
        edits.append((cs,m.end(),f'match {chain}{{{arm}=>return {base}{{result:Err(ValueError::invariant({lit}).into()),work:0}}}}'))
    if edits:
        tot+=len(edits);print(len(edits),f[-70:])
        if apply:
            for s,e,r in sorted(edits,reverse=True):t=t[:s]+r+t[e:]
            open(f,'w').write(t)
print(tot)
