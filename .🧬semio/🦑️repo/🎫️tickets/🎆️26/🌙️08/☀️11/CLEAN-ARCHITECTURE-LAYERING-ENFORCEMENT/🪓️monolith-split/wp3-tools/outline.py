#!/usr/bin/env python3
# usage: python3 outline.py <file.rs>   -> "start-end lines kind name" for every column-0 item
import re, sys
def strip(t):
    o=[];i=0;n=len(t)
    while i<n:
        c=t[i]
        if t.startswith('//',i):
            j=t.find('\n',i);i=n if j<0 else j;continue
        if t.startswith('/*',i):
            d=1;i+=2
            while i<n and d:
                if t.startswith('/*',i):d+=1;i+=2
                elif t.startswith('*/',i):d-=1;i+=2
                else:
                    if t[i]=='\n':o.append('\n')
                    i+=1
            continue
        m=re.match(r'(?:br|rb|r|cr)(#*)"',t[i:i+12]) if c in 'rbc' and (i==0 or not(t[i-1].isalnum() or t[i-1]=='_')) else None
        if m:
            e='"'+m.group(1);j=t.find(e,i+m.end());j=n if j<0 else j+len(e)
            o.append('""'+'\n'*t[i:j].count('\n'));i=j;continue
        if c=='"':
            j=i+1
            while j<n and t[j]!='"':j+=2 if t[j]=='\\' else 1
            o.append('""'+'\n'*t[i:j].count('\n'));i=j+1;continue
        if c=="'":
            m=re.match(r"'(?:\\.[^']*|[^\\'])'",t[i:i+12])
            if m:o.append("''");i+=m.end();continue
            m=re.match(r"'[A-Za-z_]\w*",t[i:])
            if m:o.append(' ');i+=m.end();continue
        o.append(c);i+=1
    return ''.join(o)
HEAD=re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use)(?:\b|(?<=!))\s*([A-Za-z_<][\w:<>, ]*)?')
raw=open(sys.argv[1],encoding='utf-8').read();L=strip(raw).split('\n');R=raw.split('\n');n=len(L)
depth=0;starts=[]
for l in L:
    starts.append(depth);depth+=l.count('{')-l.count('}')
ln=0
while ln<n:
    l=L[ln];m=HEAD.match(l) if starts[ln]==0 and l and not l[0].isspace() and l[0] not in '}#' else None
    if m:
        d=0;seen=False;j=ln
        while j<n:
            d+=L[j].count('{')-L[j].count('}');seen=seen or '{' in L[j]
            if (seen and d<=0) or (not seen and L[j].rstrip().endswith(';')):break
            j+=1
        s=ln
        while s>0 and R[s-1].lstrip().startswith(('///','#[','//!')):s-=1
        if m.group(1)!='use':print(f"{s+1:5d}-{j+1:5d} {j-s+1:4d} {m.group(1):12s} {(l.strip()[:70] if m.group(1)=='impl' else (m.group(2) or '').strip())[:80]}")
        ln=j+1
    else: ln+=1
