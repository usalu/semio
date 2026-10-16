"""Method-granular dependency graph for the os-infinite carve."""
from model import *
from modtree import walk, LIBROOT, INFROOT

class N:
    __slots__ = ('id','mod','kind','name','parent','lines','idents','astrs','plain','calls','dots','assoc','fields','vis','attrs','cfg','item_ref','impl_ref','trait','text_range')
    def __repr__(self): return f'<{self.kind} {self.parent+"::" if self.parent else ""}{self.name}>'

def load_mods(local_roots=True):
    import oinf_rs
    oinf_rs.LOCAL_ROOTS = None
    tree = walk(LIBROOT)
    if local_roots:
        roots = {'crate', 'super', 'self', 'infinite'}
        for e in tree: roots.update(e['modpath'])
        oinf_rs.LOCAL_ROOTS = roots
        tree = walk(LIBROOT)
    files = [e['file'] for e in tree if e['file'] and e['file'].startswith(INFROOT) and '🧪️tests' not in e['file']]
    mods = {}
    for f in files:
        rel = f[len(INFROOT):]
        text = open(f, encoding='utf-8').read()
        items, _ = chunk_file(text)
        m = Mod(); m.rel = rel; m.text = text; m.items = items; m.short = rel.replace('/🦀️.rs','')
        mods[rel] = m
    return tree, mods

def build(mods):
    nodes = []
    for rel, m in mods.items():
        for idx, it in enumerate(m.items):
            lines = m.text.count('\n', it.full_start, it.full_end)
            if it.kind in ('fn','struct','enum','union','trait','type','const','static','macro_rules'):
                n = N(); n.mod=rel; n.kind=it.kind; n.name=it.name; n.parent=None; n.lines=lines; n.idents=set(it.idents); n.plain=set(it.plain); n.calls=set(it.calls); n.astrs=set(getattr(it,'attr_strs',())); n.dots=set(it.dots); n.assoc=set(it.assoc); n.fields=set(it.fields); n.vis=it.vis; n.attrs=it.attrs; n.cfg=it.cfg; n.item_ref=(rel, idx); n.impl_ref=None; n.trait=None
                nodes.append(n)
            elif it.kind == 'impl':
                if it.impl_trait is None:
                    inner, sub = impl_members(m.text, it)
                    head_ids = set()
                    # idents of impl head (before the body brace)
                    head_end = m.text.index('{', it.start)
                    head_toks = [t for t in lex(m.text[it.start:head_end]) if t.k == 'id']
                    head_ids = set(t.s for t in head_toks)
                    # impl overhead lines (head+closing)
                    for s in sub:
                        if s.kind in ('fn','const','type'):
                            n = N(); n.mod=rel; n.kind='method' if s.kind=='fn' else 'assoc-'+s.kind; n.name=s.name; n.parent=it.impl_ty
                            n.lines = inner.count('\n', s.full_start, s.full_end)
                            n.idents = set(s.idents) | head_ids; n.plain = set(s.plain) | head_ids; n.calls = set(s.calls); n.astrs = set(getattr(s,'attr_strs',())); n.dots = set(s.dots); n.assoc = set(s.assoc); n.fields = set(s.fields)
                            n.vis=s.vis; n.attrs=s.attrs; n.cfg=s.cfg+it.cfg; n.item_ref=(rel, idx); n.impl_ref=(s.full_start, s.full_end); n.trait=None
                            nodes.append(n)
                else:
                    n = N(); n.mod=rel; n.kind='traitimpl'; n.name=f'{it.impl_trait} for {it.impl_ty}'; n.parent=it.impl_ty; n.lines=lines; n.idents=set(it.idents); n.plain=set(it.plain); n.calls=set(it.calls); n.astrs=set(getattr(it,'attr_strs',())); n.dots=set(it.dots); n.assoc=set(it.assoc); n.fields=set(it.fields); n.vis=''; n.attrs=it.attrs; n.cfg=it.cfg; n.item_ref=(rel, idx); n.impl_ref=None; n.trait=it.impl_trait
                    nodes.append(n)
            elif it.kind == 'macro_call':
                n = N(); n.mod=rel; n.kind='macrocall'; n.name=it.name; n.parent=None; n.lines=lines; n.idents=set(it.idents); n.plain=set(it.plain); n.calls=set(it.calls); n.astrs=set(getattr(it,'attr_strs',())); n.dots=set(it.dots); n.assoc=set(it.assoc); n.fields=set(it.fields); n.vis=''; n.attrs=it.attrs; n.cfg=it.cfg; n.item_ref=(rel, idx); n.impl_ref=None; n.trait=None
                nodes.append(n)
    for i, n in enumerate(nodes): n.id = i
    return nodes


from stdnames import STD
TOPKINDS = ('fn','struct','enum','union','trait','type','const','static','macro_rules')

def indexes(nodes):
    top = collections.defaultdict(list)
    meth = collections.defaultdict(list)
    for n in nodes:
        if n.kind in TOPKINDS: top[n.name].append(n)
        elif n.kind in ('method','assoc-const','assoc-type'): meth[n.name].append(n)
    return top, meth

def edges(nodes, ambiguous='all'):
    top, meth = indexes(nodes)
    g = collections.defaultdict(set)
    amb = collections.defaultdict(set)
    for n in nodes:
        for idn in n.plain:
            if idn == '_': continue
            cands = top.get(idn, ())
            same = [d for d in cands if d.mod == n.mod]
            if same and len(cands) > 1: cands = same
            for d in cands:
                if d is n: continue
                if d.kind == 'fn' and idn not in n.calls: continue
                g[n.id].add(d.id)
        for (ty, nm) in n.assoc:
            for d in meth.get(nm, ()):
                if d.parent == ty and d is not n: g[n.id].add(d.id)
        for nm in n.dots:
            ms = meth.get(nm, ())
            if not ms: continue
            if nm in STD:
                for d in ms:
                    if d is not n and n.parent is not None and d.parent == n.parent: g[n.id].add(d.id)
                continue
            tys = set(m.parent for m in ms)
            if len(tys) == 1:
                for d in ms:
                    if d is not n: g[n.id].add(d.id)
            else:
                cand = [d for d in ms if d is not n]
                if n.parent in tys and n.kind == 'method':
                    pass
                amb[n.id].add(nm)
                if ambiguous == 'all':
                    for d in cand: g[n.id].add(d.id)
        for sname in n.astrs:
            for d in top.get(sname, ()):
                if d.kind == 'fn' and d is not n:
                    g[n.id].add(d.id); g[d.id].add(n.id)
        if n.parent and n.kind != 'struct':
            for d in top.get(n.parent, ()): g[n.id].add(d.id)
    return g, amb
