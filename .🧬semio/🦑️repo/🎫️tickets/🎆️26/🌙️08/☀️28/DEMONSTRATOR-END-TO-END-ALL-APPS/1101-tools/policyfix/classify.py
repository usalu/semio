import re,collections,sys,json
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rows=[l.strip().rsplit(":",1) for l in open(C+"/🗑️generated/1101/aud-policy-unwrap-rows.txt")]
fnre=re.compile(r'\bfn\s+(\w+)')
def ret_of(t,pos):
    ms=list(fnre.finditer(t,0,pos))
    # walk back to find fn whose body contains pos: choose last fn whose ret parse ok; approximate nearest
    for m in reversed(ms[-6:]):
        i=t.find('(',m.end())
        # skip generics
        j=m.end()
        d=0;k=j
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
        rest=t[a+1:a+400]
        mm=re.match(r'\s*(->\s*(.*?))?\s*(where[^{]*)?\{',rest,re.S)
        if mm:
            return m.group(1),(mm.group(2) or '()').strip()[:80]
    return None,None
if __name__=="__main__":
    files={}
    cls=collections.Counter()
    pat=re.compile(r'(\.as_mut\(\)\.unwrap\(\)|\.as_ref\(\)\.unwrap\(\)|\.take\(\)\.unwrap\(\)|\.unwrap\(\)|\.expect\(|panic!|unreachable!)')
    out=[]
    seen=set()
    for f,n in rows:
        if f in seen:continue
        seen.add(f)
        t=open(f).read()
        L=t.split("\n");off=[0]
        for l in L:off.append(off[-1]+len(l)+1)
        for f2,n2 in rows:
            if f2!=f:continue
            n2=int(n2)
            s,e=off[n2-1],off[n2]
            for m in pat.finditer(t,s,e):
                nm,r=ret_of(t,m.start())
                cls[re.sub(r'<.*','',r or '?')[:40]]+=1
                out.append((f,n2,m.group(1),nm,r))
    json.dump(out,open(C+"/🗑️generated/1101/policyfix-classes.json","w"),ensure_ascii=False)
    for k,v in cls.most_common(40):print(v,k)
