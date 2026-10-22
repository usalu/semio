#!/usr/bin/env python3
"""Ports plugin call sites of the diff/mutation split: `apply_diff(`->`apply_diff_cold(`, `<T as [path::]Mutation<S>>::diff|inverse|foreign_step_source`->`ColdMutation`,
and adds `ColdMutation`/`ColdDiffAlgebra`/`ColdMutationDiff` next to imported `Mutation`/`DiffAlgebra`/`MutationDiff` when method-call syntax needs the trait. Idempotent. Usage: portcalls.py [--dry] @list"""
import re,sys
USE=re.compile(r'^([ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+[^;]*?;)',re.M|re.S)
def aug_use(stmt,name,cold):
    if re.search(r'\b'+cold+r'\b',stmt): return stmt
    if not re.search(r'(?<![\w])'+name+r'\b(?!\s*as\b)(?!::)',stmt): return stmt
    if '{' in stmt:
        return re.sub(r'(?<![\w])'+name+r'\b(?!\s*as\b)(?!::)',cold+', '+name,stmt,count=1)
    m=re.match(r'(\s*(?:pub(?:\([^)]*\))?\s+)?use\s+)([\w:]*::)'+name+r'\s*;',stmt)
    if m: return f"{m.group(1)}{m.group(2)}{{{cold}, {name}}};"
    return stmt
def port(s):
    t=s
    t=re.sub(r'(as\s+(?:[\w]+::)*)Mutation(<(?:[^<>]|<[^<>]*>)*>>::(?:diff|inverse|foreign_step_source)\b)',r'\1ColdMutation\2',t)
    t=re.sub(r'(?<![\w])((?:[\w]+::)*)apply_diff\(',lambda m:m.group(1)+'apply_diff_cold(',t)
    t=re.sub(r'(?<![\w])apply_diff_into\(',r'apply_diff_cold_into(',t)
    def inuse(m):
        st=m.group(1)
        st=re.sub(r'(?<![\w])apply_diff(?![\w])','apply_diff_cold',st)
        return st
    t=USE.sub(inuse,t)
    body_calls=re.search(r'\.(?:diff|inverse)\([^)]',t) or re.search(r'::(?:diff|inverse)\(',t)
    wants=[]
    if body_calls or re.search(r'(?<![\w:])ColdMutation<',t): wants.append(('Mutation','ColdMutation'))
    if re.search(r'\.inverse\(',t): wants.append(('DiffAlgebra','ColdDiffAlgebra'))
    if re.search(r'\.apply\(',t): wants.append(('MutationDiff','ColdMutationDiff'))
    for name,cold in wants:
        t=USE.sub(lambda m:aug_use(m.group(1),name,cold),t)
    return t
dry='--dry' in sys.argv
files=[]
for a in sys.argv[1:]:
    if a.startswith('@'): files+=[l for l in open(a[1:],encoding='utf-8').read().split('\n') if l]
n=0
for f in files:
    s=open(f,encoding='utf-8').read()
    if not re.search(r'apply_diff\b|Mutation<[^>]*>>::(?:diff|inverse)|\.(?:diff|inverse)\(',s): continue
    t=port(s)
    if t!=s:
        n+=1
        if not dry: open(f,'w',encoding='utf-8').write(t)
print('changed',n)
