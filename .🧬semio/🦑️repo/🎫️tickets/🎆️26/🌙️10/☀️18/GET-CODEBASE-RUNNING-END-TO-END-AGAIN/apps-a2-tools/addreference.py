#!/usr/bin/env python3
"""Adds `reference = <snapshot>, reference_diff = <diff>` to `#[mutations(...)]` aggregates lacking it (cold-split derive). Idempotent. Usage: addreference.py [--dry] @list"""
import re,sys
HDR=re.compile(r'^#\[mutations\(([^\]]*?)\)\]',re.M)
def fix(s):
    def sub(m):
        a=m.group(1)
        if re.search(r'\breference\b',a): return m.group(0)
        sn=re.search(r'\bsnapshot\s*=\s*([^,]+?)\s*(?:,|$)',a); df=re.search(r'\bdiff\s*=\s*([^,]+?)\s*(?:,|$)',a)
        if not sn or not df: return m.group(0)
        return f"#[mutations({a.rstrip()}, reference = {sn.group(1)}, reference_diff = {df.group(1)})]"
    return HDR.sub(sub,s)
dry='--dry' in sys.argv
files=[]
for a in sys.argv[1:]:
    if a.startswith('@'): files+=[l for l in open(a[1:],encoding='utf-8').read().split('\n') if l]
n=0
for f in files:
    s=open(f,encoding='utf-8').read()
    if '#[mutations(' not in s: continue
    t=fix(s)
    if t!=s:
        n+=1; print(f[-80:])
        if not dry: open(f,'w',encoding='utf-8').write(t)
print('changed',n)
