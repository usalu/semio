#!/usr/bin/env python3
"""usage: nested_match.py [--apply] <root> ; rewrites `match X.reborrow(G,X.maximum_work_units()){Ok(v)=>v,Err(e)=>return ..}` (exact shape, balanced braces, followed by `;`) to `X.nested(G)`. Everything else is untouched."""
import os,re,sys
apply='--apply' in sys.argv;root=[a for a in sys.argv[1:] if not a.startswith('--')][0]
def bal(s,i,o,c):
    d=0
    for j in range(i,len(s)):
        if s[j]==o:d+=1
        elif s[j]==c:
            d-=1
            if d==0:return j
    return -1
def split2(a):
    d=0
    for k,ch in enumerate(a):
        if ch in '([{':d+=1
        elif ch in ')]}':d-=1
        elif ch==',' and d==0:return a[:k],a[k+1:]
    return None
pat=re.compile(r'match (\w+)\.reborrow\(')
n=0
for dp,_,fs in os.walk(root):
    for f in fs:
        if not f.endswith('.rs'):continue
        p=os.path.join(dp,f);s=open(p).read()
        if 'reborrow(' not in s:continue
        out='';i=0;c=0
        for m in pat.finditer(s):
            if m.start()<i:continue
            op=m.end()-1;cl=bal(s,op,'(',')')
            sp=split2(s[op+1:cl])
            if not sp or sp[1].strip()!=m.group(1)+'.maximum_work_units()':continue
            rest=s[cl+1:]
            mm=re.match(r'\{Ok\((\w+)\)=>\1,Err\((\w+)\)=>return ',rest)
            if not mm:continue
            bo=cl+1;be=bal(s,bo,'{','}')
            if be<0 or s[be+1]!=';':continue
            out+=s[i:m.start()]+m.group(1)+'.nested('+sp[0].strip()+')';i=be+1;c+=1
        out+=s[i:]
        if c:
            print('FILE',p[len(root)+1:],c);n+=c
            if apply:open(p,'w').write(out)
print('sites',n)
