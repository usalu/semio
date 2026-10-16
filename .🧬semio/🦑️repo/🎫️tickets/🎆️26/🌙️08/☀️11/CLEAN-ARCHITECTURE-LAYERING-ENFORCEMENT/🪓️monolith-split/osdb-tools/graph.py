"""🕸️ Item graph over parsed items (impls merged into their self type)."""
import re,sys,collections
from lib import *
from model import parse_items
from rs import strip,idents
from scc2 import sccs

class Node:
    __slots__=('key','lines','text','parts','kinds','cfg_test','file')
    def __init__(s,key,file): s.key=key; s.lines=0; s.text=''; s.parts=[]; s.kinds=set(); s.cfg_test=True; s.file=file

def load(files):
    """files: dict file_id -> path. Returns nodes dict key -> Node (keyed 'file:name' unless unique)"""
    nodes={}
    for fid,path in files.items():
        text=read(path)
        items,trail,lines=parse_items(text)
        st=strip(text).split('\n')
        for it in items:
            if it.kind in('use','extern crate'): continue
            name=it.self_type if it.kind=='impl' else it.name
            if it.kind=='macro_call': continue
            if it.kind=='mod' and strip('\n'.join(lines[it.header:it.end+1])).rstrip().endswith(';'): continue
            if not name or name=='_': name=f'<{it.kind}@{fid}:{it.header+1}>'
            if it.kind=='mod' and name=='tests': name=f'tests@{fid}'
            n=nodes.setdefault(name,Node(name,fid))
            n.lines+=it.end-it.start+1
            n.text+=re.sub(r'(?<!\.)\.\s*[A-Za-z_][A-Za-z0-9_]*','.', '\n'.join(st[it.start:it.end+1]))+'\n'
            n.parts.append((fid,it))
            n.kinds.add(it.kind)
            n.cfg_test = n.cfg_test and it.cfg_test
    return nodes

def edges(nodes):
    names={k for k,n in nodes.items() if n.kinds-{'impl'}}
    g={k:set() for k in nodes}
    for k,n in nodes.items():
        for i in idents(n.text):
            if i in names and i!=k: g[k].add(i)
    return g

def topo(nodes,g):
    comps=sccs(g)
    return comps
