#!/usr/bin/env python3
"""ctx.py <log> <crate-substr> : prints each error with the source snippet at line:col (current file contents)."""
import re,sys,os
log,want=sys.argv[1],sys.argv[2]
buf=[]
out=[]
for line in open(log,encoding='utf-8',errors='replace'):
    line=line.rstrip('\n')
    m=re.match(r'^(.*?\.rs):(\d+):(\d+): error(?:\[E\d+\])?: (.*)$',line)
    if m: buf.append(m.groups())
    c=re.match(r'error: could not compile `([^`]+)` \(([^)]*)\)',line)
    if c:
        if want in c.group(1):
            print('###',c.group(1),c.group(2))
            seen=set()
            for f,l,col,msg in buf:
                f2=os.path.normpath(f)
                key=(f2,l,col)
                if key in seen: continue
                seen.add(key)
                try:
                    L=open(f2,encoding='utf-8').read().split('\n')
                    t=L[int(l)-1]; i=int(col)-1
                    print(f"- {f2[-60:]}:{l}:{col} {msg[:200]}\n    >>> {t[max(0,i-60):i+140]}")
                except Exception as e:
                    print('- ',f2[-60:],l,col,msg[:150],'(no ctx)')
        buf=[]
