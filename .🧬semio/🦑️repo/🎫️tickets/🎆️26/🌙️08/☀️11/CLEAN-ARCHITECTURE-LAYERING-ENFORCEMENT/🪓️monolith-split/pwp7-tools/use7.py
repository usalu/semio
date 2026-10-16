import sys; sys.path.insert(0,'.')
from model7 import *
def split_top(s):
    parts=[];d=0;cur=''
    for ch in s:
        if ch=='{': d+=1
        elif ch=='}': d-=1
        if ch==',' and d==0: parts.append(cur); cur=''
        else: cur+=ch
    if cur.strip(): parts.append(cur)
    return [p.strip() for p in parts if p.strip()]
def expand(prefix, tree):
    tree=tree.strip()
    if tree.startswith('{') and tree.endswith('}'):
        out=[]
        for p in split_top(tree[1:-1]): out+=expand(prefix,p)
        return out
    if '{' in tree:
        i=tree.index('{'); head=tree[:i].rstrip(':').strip()
        return expand(prefix+head+'::',tree[i:])
    return [(prefix+tree)]
def leaves(stmt):
    """stmt text like '#[cfg(test)] use a::{b, c as d};' -> list of (attr, path, alias)"""
    s=stmt.strip()
    attrs=''
    while s.startswith('#['):
        e=s.index(']')+1; attrs+=s[:e]+' '; s=s[e:].strip()
    m=re.match(r'(pub(?:\([^)]*\))?\s+)?use\s+(.*);\s*$',s,re.S)
    if not m: return None
    vis=m.group(1) or ''
    body=re.sub(r'\s+',' ',m.group(2)).strip()
    out=[]
    for p in expand('',body):
        p=p.strip()
        alias=None
        mm=re.match(r'(.*?)\s+as\s+(\w+)$',p)
        if mm: p,alias=mm.group(1).strip(),mm.group(2)
        segs=p.split('::')
        if segs[-1]=='self': segs=segs[:-1]; p='::'.join(segs)
        name=alias or segs[-1]
        out.append((attrs,vis,p,alias,name))
    return out
def leaf_stmt(l):
    attrs,vis,p,alias,name=l
    return '%s%suse %s%s;'%(attrs,vis,p,(' as %s'%alias) if alias else '')
if __name__=='__main__':
    for s in ['use a::{b, c as d, e::{self, f}};','#[cfg(test)] use x::Y;','use crate::tool_jobs::tool_machine::*;','use semio_framework_tool_machine::{\n  ScrubPhase, ToolAbortReason,\n};']:
        print(leaves(s))
