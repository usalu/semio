"""Global item model for the os-infinite carve."""
import sys, os, collections, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from oinf_rs import *
INF = os.environ.get("OINF_ROOT", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite")

def scan_files(roots=("🌍️world", "🎲️board"), tests=False):
    files = {}
    for root in roots:
        for dp, dn, fn in os.walk(os.path.join(INF, root)):
            if not tests and ('🧪️tests' in dp or '🧫️fixtures' in dp): continue
            for f in fn:
                if f.endswith('.rs'):
                    p = os.path.join(dp, f); files[p[len(INF)+1:]] = p
    return files

class Mod:
    pass

def load(roots=("🌍️world", "🎲️board"), tests=False):
    mods = {}
    for rel, p in sorted(scan_files(roots, tests).items()):
        text = open(p, encoding='utf-8').read()
        items, _ = chunk_file(text)
        m = Mod(); m.rel = rel; m.path = p; m.text = text; m.items = items
        m.short = rel.replace('/🦀️.rs', '')
        mods[rel] = m
    return mods

def impl_members(text, it):
    body_start = text.index('{', it.start)
    inner = text[body_start+1:it.end-1]
    sub, _ = chunk_file(inner)
    return inner, sub

def strip_comment_tokens_ids(it):
    return it.idents

def build_nodes(mods):
    """node id -> dict(mod, items[list of Item], kind, name, lines, idents, defs(set of defined names), method_defs(set))"""
    nodes = {}
    typenode = {}
    order = []
    for rel, m in mods.items():
        for idx, it in enumerate(m.items):
            lines = m.text.count('\n', it.full_start, it.full_end)
            if it.kind in ('fn','struct','enum','union','trait','type','const','static','macro_rules'):
                nid = (rel, it.kind, it.name)
                if nid in nodes: nid = (rel, it.kind, it.name, idx)
                nodes[nid] = dict(mod=rel, items=[(rel, idx)], kind=it.kind, name=it.name, lines=lines, idents=set(it.idents), defs={it.name}, methods=set())
                if it.kind in ('struct','enum','union','type','trait'):
                    typenode.setdefault(it.name, nid)
                order.append(nid)
    # attach impls
    leftovers = []
    for rel, m in mods.items():
        for idx, it in enumerate(m.items):
            lines = m.text.count('\n', it.full_start, it.full_end)
            if it.kind == 'impl':
                ty = it.impl_ty
                nid = typenode.get(ty)
                if nid is None:
                    nid = (rel, 'impl-ext', ty, it.impl_trait, idx)
                    nodes[nid] = dict(mod=rel, items=[(rel, idx)], kind='impl-ext', name=ty, lines=lines, idents=set(it.idents), defs=set(), methods=set())
                    order.append(nid)
                    continue
                n = nodes[nid]
                n['items'].append((rel, idx)); n['lines'] += lines; n['idents'] |= it.idents
                if it.impl_trait is None:
                    inner, sub = impl_members(m.text, it)
                    for s in sub:
                        if s.name: n['methods'].add(s.name)
            elif it.kind == 'macro_call':
                nid = (rel, 'macro_call', it.name, idx)
                nodes[nid] = dict(mod=rel, items=[(rel, idx)], kind='macro_call', name=it.name, lines=lines, idents=set(it.idents), defs=set(), methods=set())
                order.append(nid)
            elif it.kind in ('use','mod','extern_crate','extern','attr-only'):
                nid = (rel, it.kind, it.name, idx)
                nodes[nid] = dict(mod=rel, items=[(rel, idx)], kind=it.kind, name=it.name, lines=lines, idents=set(it.idents), defs=set(), methods=set())
                order.append(nid)
    return nodes, typenode, order

def scc(graph):
    sys.setrecursionlimit(1000000)
    idx = {}; low = {}; st = []; on = set(); out = []; c = [0]
    def go(v):
        stack = [(v, iter(graph.get(v, ())))]
        idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
        while stack:
            u, it = stack[-1]
            adv = False
            for w in it:
                if w not in graph: continue
                if w not in idx:
                    idx[w] = low[w] = c[0]; c[0] += 1; st.append(w); on.add(w)
                    stack.append((w, iter(graph.get(w, ()))))
                    adv = True
                    break
                elif w in on:
                    low[u] = min(low[u], idx[w])
            if adv: continue
            stack.pop()
            if stack:
                p = stack[-1][0]
                low[p] = min(low[p], low[u])
            if low[u] == idx[u]:
                comp = []
                while True:
                    w = st.pop(); on.discard(w); comp.append(w)
                    if w == u: break
                out.append(comp)
    for v in graph:
        if v not in idx: go(v)
    return out

def item_graph(nodes, method_split=()):
    topdef = collections.defaultdict(set)
    methdef = collections.defaultdict(set)
    for nid, n in nodes.items():
        for d in n['defs']: topdef[d].add(nid)
        for d in n['methods']: methdef[d].add(nid)
    g = {}
    for nid, n in nodes.items():
        refs = set()
        for idn in n['idents']:
            for B in topdef.get(idn, ()):
                if B != nid: refs.add(B)
            for B in methdef.get(idn, ()):
                if B != nid: refs.add(B)
        g[nid] = refs
    return g
