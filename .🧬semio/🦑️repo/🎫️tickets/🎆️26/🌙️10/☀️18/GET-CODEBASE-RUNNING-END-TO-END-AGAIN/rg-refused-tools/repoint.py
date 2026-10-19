#!/usr/bin/env python3
# 🔀️ Re-point importers of moved items: repoint.py <old_ident> <new_ident> <names_regex> <file>...
# Handles `use old::{A, B};` (splitting mixed groups), `use old::A;` and inline `old::A` paths. Prints per-file counts.
import re,sys
old,new,names=sys.argv[1],sys.argv[2],sys.argv[3]
files=sys.argv[4:]
NAME=re.compile(r'^(?:'+names+r')$')
def split_top(body):
    items,depth,cur=[],0,''
    for ch in body:
        if ch in '{(<[': depth+=1
        if ch in '})>]': depth-=1
        if ch==',' and depth==0: items.append(cur);cur=''
        else: cur+=ch
    if cur.strip(): items.append(cur)
    return [i.strip() for i in items if i.strip()]
def first_seg(item):
    m=re.match(r'[A-Za-z_][A-Za-z0-9_]*',item)
    return m.group(0) if m else ''
def fix(src):
    n=0
    def group(m):
        nonlocal n
        prefix,body=m.group(1),m.group(2)
        items=split_top(body)
        moved=[i for i in items if NAME.match(first_seg(i))]
        rest=[i for i in items if not NAME.match(first_seg(i))]
        if not moved: return m.group(0)
        n+=1
        a=f"{prefix}{new}::{{{', '.join(moved)}}};"
        if not rest: return a
        return f"{prefix}{old}::{{{', '.join(rest)}}};\n{a}"
    src=re.sub(r'((?:pub(?:\([^)]*\))?\s+)?use\s+)'+re.escape(old)+r'::\{([^;]*?)\};',group,src,flags=re.S)
    def single(m):
        nonlocal n
        n+=1
        return f"{new}::{m.group(1)}"
    src=re.sub(r'(?<![A-Za-z0-9_])'+re.escape(old)+r'::((?:'+names+r')\b)',single,src)
    return src,n
total=0
for f in files:
    s=open(f,encoding='utf8').read()
    t,n=fix(s)
    if t!=s: open(f,'w',encoding='utf8').write(t)
    left=len(re.findall(r'(?<![A-Za-z0-9_])'+re.escape(old)+r'\b',t))
    print(f"{n:3d} changed, {left:3d} remaining refs to {old}: {f[-90:]}")
