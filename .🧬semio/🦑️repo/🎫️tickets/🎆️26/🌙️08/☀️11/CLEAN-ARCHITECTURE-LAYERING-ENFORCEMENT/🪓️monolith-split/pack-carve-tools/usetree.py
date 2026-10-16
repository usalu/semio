"""🌳 Use-tree parser and pruner for the pack carve generator."""
import re

def _split_top(s):
    parts=[];d=0;cur=''
    for c in s:
        if c=='{': d+=1
        if c=='}': d-=1
        if c==',' and d==0:
            parts.append(cur);cur=''
        else: cur+=c
    if cur.strip(): parts.append(cur)
    return [p.strip() for p in parts if p.strip()]

def parse(tree):
    """tree: text after `use` and before `;`. Returns list of leaves (path:list[str], alias|None, glob:bool)."""
    tree=re.sub(r'\s+',' ',tree.strip())
    return _leaves(tree,[])

def _leaves(t,prefix):
    t=t.strip()
    if t.startswith('::'): t=t[2:]
    m=re.match(r'^(.*?)\{(.*)\}$',t)
    if m and m.group(1).strip().endswith('::') or (m and m.group(1).strip()==''):
        pre=[p for p in m.group(1).strip().split('::') if p]
        out=[]
        for part in _split_top(m.group(2)):
            out+=_leaves(part,prefix+pre)
        return out
    if t.endswith('::*'):
        return [(prefix+[p for p in t[:-3].split('::') if p],None,True)]
    m=re.match(r'^(.*?) as (\w+)$',t)
    alias=None
    if m: t,alias=m.group(1),m.group(2)
    path=prefix+[p for p in t.split('::') if p]
    return [(path,alias,False)]

def local_name(leaf):
    path,alias,glob=leaf
    if glob: return None
    if alias: return alias
    if path[-1]=='self': return path[-2]
    return path[-1]

def emit(leaves):
    """Emits grouped `use` lines (one per distinct 2-segment-or-shorter prefix grouping by first segments)."""
    groups={}
    order=[]
    for path,alias,glob in leaves:
        key=tuple(path[:-1]) if not glob else tuple(path)
        if key not in groups: groups[key]=[];order.append(key)
        groups[key].append((path,alias,glob))
    lines=[]
    for key in order:
        g=groups[key]
        def one(p,a,gl):
            if gl: return '*'
            return (p[-1]+(' as '+a if a else ''))
        names=[one(*x) for x in g]
        if len(names)==1 and not any(x[2] for x in g):
            path,alias,glob=g[0]
            lines.append('use '+'::'.join(path)+(' as '+alias if alias else '')+';')
        elif len(names)==1:
            lines.append('use '+'::'.join(key)+'::*;')
        else:
            lines.append('use '+'::'.join(key)+'::{'+', '.join(names)+'};')
    return lines
