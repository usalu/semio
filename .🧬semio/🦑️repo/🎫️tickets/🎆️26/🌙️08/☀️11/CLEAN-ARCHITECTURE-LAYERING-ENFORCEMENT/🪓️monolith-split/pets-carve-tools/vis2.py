import re,os,json
from plan import *
pat=re.compile(r'^\s*(pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:const\s+)?(?:async\s+)?(fn|const|static|struct|enum|type|trait|mod|use)\s+(\w+)',re.M)
defs={}
for m in OWNER:
    s=open(src(m)).read(); d={}
    for mm in pat.finditer(s): d.setdefault(mm.group(3),[]).append((mm.group(1) or '').strip() or 'private')
    defs[m]=d
need={}
for m,c in OWNER.items():
    for p in (src(m),tst(m)):
        if not os.path.exists(p): continue
        s=open(p).read()
        refs=[]
        for mm in re.finditer(r'use crate::(\w+)::(\{[^;]*\}|\w+)(?:\s+as\s+\w+)?;',s):
            M=mm.group(1)
            body=mm.group(2).strip('{}')
            for nm in re.split(r'[,\s]+',body):
                if nm and nm not in ('self','as'): refs.append((M,nm))
        for mm in re.finditer(r'crate::(\w+)::(\w+)',s): refs.append((mm.group(1),mm.group(2)))
        for M,nm in refs:
            if M in OWNER and OWNER[M]!=c and not (M=='schema' and nm=='tests'):
                v=defs[M].get(nm)
                key=(M,nm)
                if v is None: need[key]='UNDEFINED'
                elif 'pub' not in v: need.setdefault(key,str(v))
for k,v in sorted(need.items()): print(k,v)
