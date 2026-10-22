#!/usr/bin/env python3
"""drive_one_item_preparation_law(factory, snapshot, mutation, grant) -> (.., 1 << 16, grant): the law driver now takes the per-turn body bytes. Idempotent. usage: lawbody.py [--dry] @list"""
import re,sys,os
sys.path.insert(0,os.path.join(os.path.dirname(__file__),'..','apps-b-tools'))
from rsparse import match_brace
def split_args(body):
    out=[];depth=0;cur=''
    i=0
    while i<len(body):
        c=body[i]
        if c in '([{': depth+=1
        elif c in ')]}': depth-=1
        if c==',' and depth==0: out.append(cur);cur=''
        else: cur+=c
        i+=1
    out.append(cur);return out
def fix(s):
    out=[];pos=0
    for m in re.finditer(r'drive_one_item_preparation_law\(',s):
        if m.start()<pos: continue
        o=m.end()-1;e=match_brace(s,o)
        args=split_args(s[o+1:e])
        if len(args)==4:
            args.insert(3,' 1 << 16')
            out.append(s[pos:o+1]);out.append(','.join(args));pos=e
    out.append(s[pos:]);return ''.join(out)
dry='--dry' in sys.argv
files=[]
for a in sys.argv[1:]:
    if a.startswith('@'): files+=[l for l in open(a[1:],encoding='utf-8').read().split('\n') if l]
n=0
for f in files:
    s=open(f,encoding='utf-8').read()
    if 'drive_one_item_preparation_law(' not in s: continue
    t=fix(s)
    if t!=s:
        n+=1;print(f[-80:])
        if not dry: open(f,'w',encoding='utf-8').write(t)
print('changed',n)
