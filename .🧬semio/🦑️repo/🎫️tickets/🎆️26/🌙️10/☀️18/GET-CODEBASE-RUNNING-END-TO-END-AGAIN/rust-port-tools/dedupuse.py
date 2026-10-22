#!/usr/bin/env python3
"""Removes a Cold*/Mutation* name from a later `use` statement when the same scope already imports it (portcalls adds per statement). Idempotent. usage: dedupuse.py [--dry] @list"""
import re,sys
USE=re.compile(r'^([ \t]*)((?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+)([^;]*?);',re.M|re.S)
NAMES=['ColdMutation','ColdDiffAlgebra','ColdMutationDiff','Mutation','DiffAlgebra','MutationDiff','apply_diff_cold','apply_diff_cold_into']
def scopes(s):
    """returns list of (pos, scope_id) for every char index via stack scan of braces; only positions of statements are queried."""
    stack=[0];ids=[]
    i=0;n=len(s);events=[]
    while i<n:
        c=s[i]
        if s.startswith('//',i):
            k=s.find('\n',i);i=n if k<0 else k;continue
        if s.startswith('/*',i):
            k=s.find('*/',i+2);i=n if k<0 else k+2;continue
        if c=='"':
            j=i+1
            while j<n:
                if s[j]=='\\': j+=2;continue
                if s[j]=='"': break
                j+=1
            i=j+1;continue
        if c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[i:])
            if m: i+=len(m.group(0));continue
        if c=='{': stack.append(i);events.append((i,stack[-1]))
        elif c=='}' and len(stack)>1: stack.pop()
        events.append((i,stack[-1])) if c in '{}' else None
        i+=1
    return events
def scope_at(events_sorted_positions,s_stack_cache,pos):
    pass
def idents(body):
    out=set()
    if '{' in body:
        inner=body[body.index('{')+1:body.rindex('}')]
        for part in re.split(r'[,{}]',inner):
            part=part.strip()
            if not part: continue
            a=re.search(r'\bas\s+(\w+)$',part)
            out.add(a.group(1) if a else part.split('::')[-1])
    else:
        a=re.search(r'\bas\s+(\w+)$',body.strip())
        out.add(a.group(1) if a else body.strip().split('::')[-1])
    return out
def innermost(s,positions):
    res={};stack=[0];i=0;n=len(s);want=sorted(positions);wi=0
    while i<n and wi<len(want):
        while wi<len(want) and want[wi]<=i:
            res[want[wi]]=stack[-1];wi+=1
        if wi>=len(want): break
        c=s[i]
        if s.startswith('//',i):
            k=s.find('\n',i);i=n if k<0 else k;continue
        if s.startswith('/*',i):
            k=s.find('*/',i+2);i=n if k<0 else k+2;continue
        if c=='"':
            j=i+1
            while j<n:
                if s[j]=='\\': j+=2;continue
                if s[j]=='"': break
                j+=1
            i=j+1;continue
        if c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[i:])
            if m: i+=len(m.group(0));continue
        if c=='{': stack.append(i)
        elif c=='}' and len(stack)>1: stack.pop()
        i+=1
    for p in want[wi:]: res[p]=stack[-1]
    return res
def fix(s):
    stmts=list(USE.finditer(s))
    sc=innermost(s,[m.start() for m in stmts])
    seen={}
    edits=[]
    for m in stmts:
        ids=idents(m.group(3))
        key=sc[m.start()]
        for n in NAMES:
            if n in ids:
                if (key,n) in seen: edits.append((m,n))
                else: seen[(key,n)]=m
    if not edits: return s
    out=s
    for m,n in sorted(edits,key=lambda e:-e[0].start()):
        body=m.group(3)
        if '{' in body:
            new=re.sub(r'(?<![\w:])'+n+r'(?![\w])\s*,?\s*','',body,count=1)
            new=re.sub(r',\s*\}',r'}',new)
            new=re.sub(r'\{\s*,',r'{',new)
            if re.fullmatch(r'[^{]*\{\s*\}',new): rep=''
            else: rep=m.group(1)+m.group(2)+new+';'
        else:
            rep=''
        out=out[:m.start()]+rep+out[m.end():]
    return out
dry='--dry' in sys.argv
files=[]
for a in sys.argv[1:]:
    if a.startswith('@'): files+=[l for l in open(a[1:],encoding='utf-8').read().split('\n') if l]
n=0
for f in sorted(set(files)):
    s=open(f,encoding='utf-8').read()
    t=fix(s)
    if t!=s:
        n+=1;print(f[-90:])
        if not dry: open(f,'w',encoding='utf-8').write(t)
print('changed',n)
