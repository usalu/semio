"""🧪 Assign top-level test items to the highest subject group they reference. Usage as module."""
import re,collections
from graph import *
from model import parse_items
from rs import strip,idents

def assign_tests(test_paths, G, nodes, order, show=True):
    """G: node->group, order: groups bottom->top. Returns dict (test_path, idx)->group plus helper info."""
    rank={g:i for i,g in enumerate(order)}
    names={k for k,n in nodes.items() if n.kinds-{'impl'} and G.get(k)}
    out={}
    tn={}  # test item name -> (path, idx)
    allitems={}
    for tp in test_paths:
        text=read(tp); items,trail,lines=parse_items(text); st=strip(text).split('\n')
        allitems[tp]=(items,lines,st)
        for i,it in enumerate(items):
            if it.name and it.kind not in ('use','impl','mod'): tn[it.name]=(tp,i)
            if it.kind=='mod': tn[it.name]=(tp,i)
    direct={}
    for tp,(items,lines,st) in allitems.items():
        for i,it in enumerate(items):
            if it.kind=='use': continue
            txt=re.sub(r'(?<!\.)\.\s*[A-Za-z_][A-Za-z0-9_]*','.', '\n'.join(st[it.start:it.end+1]))
            ids=idents(txt)
            gs={G[k] for k in ids if k in names}
            tests_used={tn[x] for x in ids if x in tn and tn[x]!=(tp,i)}
            direct[(tp,i)]=(gs,tests_used)
    # propagate: an item's group >= groups of helper items it uses (helpers resolved transitively)
    def need(key,seen=()):
        if key in seen: return set()
        gs,tu=direct[key]
        r=set(gs)
        for t in tu: r|=need(t,seen+(key,))
        return r
    res={}
    for key in direct:
        r=need(key)
        res[key]=max(r,key=lambda g:rank[g]) if r else None
    return res,allitems,direct
