"""World3dState decomposition engine: analysis of state-taking functions."""
import re, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from oinf_rs import *
from model import INF
from wmap import F as FIELD2COMP, ORDER as CORDER

STATE_TY = 'World3dState'
STATE_METHODS = {'ingest_census','mesh_geometry_census','interaction_census','new','request_owned','dynamic_retirement_is_idle','set_modelling_locale','scalar_field_status','snapshot_fault','mesh_lease'}

MANUAL = []

class Fn:
    __slots__ = ('file','kind','name','parent','start','end','pstart','pend','pname','mut','toks','impl_trait','node_key','body_first','uses','calls','flags','own','req','text_lines','has_param','mode','names','life')
    def __repr__(self): return f'<Fn {self.parent+"::" if self.parent else ""}{self.name} {self.file.split("/")[-2]}>'

def ctoks(text):
    return [t for t in lex(text) if t.k != 'comment']

def find_params(toks, fstart_idx):
    """toks index of `fn` keyword -> (open paren idx, close paren idx, list of (a,b) token index ranges per param)"""
    j = fstart_idx
    while toks[j].s != '(' : j += 1
    d = 0; k = j
    while True:
        if toks[k].s == '(' and toks[k].k == 'p': d += 1
        elif toks[k].s == ')' and toks[k].k == 'p':
            d -= 1
            if d == 0: break
        k += 1
    params = []; cur = j + 1; dd = 0
    for m in range(j + 1, k):
        t = toks[m]
        if t.k == 'p':
            if t.s in '([{<': dd += 1
            elif t.s in ')]}>' and not (t.s == '>' and toks[m-1].s == '-'): dd -= 1
            elif t.s == ',' and dd == 0:
                params.append((cur, m)); cur = m + 1
    if cur < k: params.append((cur, k))
    return j, k, params

def collect_fns(path, text, with_tests=True):
    """All fn items at module level and impl-member level of a file, with absolute offsets."""
    items, _ = chunk_file(text)
    toks = ctoks(text)
    pos2idx = {t.a: i for i, t in enumerate(toks)}
    out = []
    def mk(it, off, parent, impl_trait):
        f = Fn(); f.file = path; f.kind = 'method' if parent else 'fn'; f.name = it.name; f.parent = parent; f.impl_trait = impl_trait
        f.start = it.start + off; f.end = it.end + off
        # find 'fn' token index
        i = pos2idx.get(f.start)
        # advance to fn keyword
        while toks[i].s != 'fn' or toks[i].k != 'id': i += 1
        f.toks = toks
        po, pc, params = find_params(toks, i)
        f.pname = None; f.mut = None; f.pstart = f.pend = None; f.has_param = False; f.life = None; f.mode = None; f.names = None
        f.body_first = pc + 1
        for (a, b) in params:
            seg = toks[a:b]
            ids = [t.s for t in seg if t.k == 'id']
            if STATE_TY in ids and ':' in [t.s for t in seg]:
                ci = [t.s for t in seg].index(':')
                nm = [t for t in seg[:ci] if t.k == 'id' and t.s != 'mut']
                tys = [t.s for t in seg[ci+1:]]
                life = None
                if len(tys) >= 3 and tys[0] == '&' and tys[1].startswith("'"): life = tys[1]; tys = ['&'] + tys[2:]
                if tys not in (['&', STATE_TY], ['&', 'mut', STATE_TY]):
                    MANUAL.append((path, f.name)); continue
                f.life = life
                f.pname = nm[0].s if nm else None
                f.mut = 'mut' in tys
                f.pstart, f.pend = a, b
                f.has_param = True
        # also methods of State with self
        if parent == STATE_TY and any(toks[x].s == 'self' for x in range(po+1, pc)):
            f.pname = 'self'; f.has_param = True
            f.mut = any(toks[x].s == 'mut' for x in range(po+1, min(po+4, pc)))
        f.node_key = (path, parent, f.name, it.start + off)
        end_idx = pos2idx.get(it.end + off - 1)
        f.text_lines = text.count('\n', f.start, f.end)
        out.append((f, i, pc))
    for it in items:
        if it.kind == 'fn':
            mk(it, 0, None, None)
        elif it.kind == 'impl':
            body_start = text.index('{', it.start)
            inner = text[body_start + 1: it.end - 1]
            sub, _ = chunk_file(inner)
            for s in sub:
                if s.kind == 'fn':
                    mk(s, body_start + 1, it.impl_ty, it.impl_trait)
    return out, toks

def body_range(toks, f, pc):
    # token index range of the body: from pc+1 to token with b == f.end
    end = pc + 1
    while end < len(toks) and toks[end].b < f.end: end += 1
    return pc + 1, end   # [pc+1, end] inclusive of closing brace token index `end`


def analyze_fn(f, toks, pc, text):
    """Scan the body of f for uses of its state param. Fills f.uses (fields), f.calls, f.flags."""
    f.uses = collections.Counter(); f.calls = []; f.flags = []; f.own = set()
    if not f.has_param: return
    a, b = body_range(toks, f, pc)
    nm = f.pname
    for i in range(a, b + 1):
        t = toks[i]
        if t.k != 'id' or t.s != nm: continue
        prev = toks[i-1].s if i > 0 else ''
        nxt = toks[i+1].s if i + 1 < len(toks) else ''
        # skip occurrences as field name after '.' (e.g. foo.state) or in a path
        if prev == '.' or (prev == ':' and toks[i-2].s == ':'): continue
        if nxt == '.':
            fld = toks[i+2].s if i + 2 < len(toks) else ''
            after = toks[i+3].s if i + 3 < len(toks) else ''
            if fld in FIELD2COMP:
                f.uses[fld] += 1; f.own.add(FIELD2COMP[fld])
                continue
            if fld in STATE_METHODS and after == '(':
                f.flags.append(('stmethod', fld, i)); continue
            f.flags.append(('unknown-member', fld, i)); continue
        # argument position?
        s = i
        while toks[s-1].s in ('&', 'mut', '*') and toks[s-1].k in ('p', 'id') and (toks[s-1].k == 'p' or toks[s-1].s == 'mut'):
            s -= 1
        before = toks[s-1].s
        if before in ('(', ',') and nxt in (',', ')'):
            # find enclosing '('
            depth = 0; k = s - 1
            if before == ',':
                depth = 0
                while True:
                    k -= 1
                    tk = toks[k]
                    if tk.k == 'p':
                        if tk.s in ')]}': depth += 1
                        elif tk.s in '([{':
                            if depth == 0: break
                            depth -= 1
            # now toks[k] is '(' (when before == '(' then k == s-1)
            if toks[k].s != '(':
                f.flags.append(('arg-not-call', before, i)); continue
            c = k - 1
            if toks[c].k == 'id':
                name = toks[c].s
                kind = 'free'; prefix = None
                if toks[c-1].s == '.': kind = 'method'
                elif toks[c-1].s == ':' and toks[c-2].s == ':' and toks[c-3].k == 'id': kind = 'path'; prefix = toks[c-3].s
                refkind = ''.join(x.s for x in toks[s:i])
                f.calls.append(dict(name=name, kind=kind, prefix=prefix, a=s, b=i, ref=refkind, callee_tok=c))
            else:
                f.flags.append(('callee-not-ident', toks[c].s, i))
            continue
        f.flags.append(('other', f'{toks[i-1].s} {nm} {nxt}', i))


def load_world(include_tests=False):
    allfns = []; files = {}
    for dp, dn, fnn in os.walk(INF + '/🌍️world'):
        if not include_tests and '🧪️tests' in dp: continue
        for f in sorted(fnn):
            if f.endswith('.rs'):
                p = os.path.join(dp, f)
                text = open(p, encoding='utf-8').read()
                fns, toks = collect_fns(p, text)
                files[p] = (text, toks)
                for fn, i, pc in fns:
                    analyze_fn(fn, toks, pc, text)
                    fn.node_key = fn.node_key
                    allfns.append(fn)
    return allfns, files

def resolve_calls(allfns):
    defs = collections.defaultdict(list)
    for f in allfns:
        if f.has_param: defs[f.name].append(f)
    unresolved = []
    for f in allfns:
        for c in f.calls:
            cands = defs.get(c['name'], [])
            if c['kind'] == 'path' and c['prefix'] not in (None, 'Self'):
                cands2 = [d for d in cands if d.parent == c['prefix']]
                if cands2: cands = cands2
            elif c['kind'] == 'path' and c['prefix'] == 'Self' and f.parent:
                cands2 = [d for d in cands if d.parent == f.parent]
                if cands2: cands = cands2
            elif c['kind'] == 'free':
                cands2 = [d for d in cands if d.parent is None]
                if cands2: cands = cands2
            c['targets'] = cands
            if not cands: unresolved.append((f, c))
    return defs, unresolved

def fixpoint(allfns, defs):
    for f in allfns: f.req = set(f.own)
    # same-name methods share requirement (called through dots)
    changed = True
    while changed:
        changed = False
        for f in allfns:
            if not f.has_param: continue
            new = set(f.req)
            for c in f.calls:
                for t in c['targets']:
                    new |= t.req
            if c.get('kind') == 'method' if False else False: pass
            if new != f.req: f.req = new; changed = True


HINTS = {
 ('⚙️authority', 741): 'WorldRayPickCursor', ('⚙️authority', 799): 'WorldObjectPickCursor', ('⚙️authority', 840): 'WorldComponentPickCursor',
 ('⚙️authority', 882): 'WorldMarqueePickCursor', ('⚙️authority', 932): 'WorldMarqueePublishJob', ('⚙️authority', 971): 'WorldComponentMarqueePublishJob',
 ('⚙️authority', 1010): 'WorldGumballPickCursor', ('⚙️authority', 1058): 'WorldGumballCommitJob', ('⚙️authority', 1101): 'WorldBrushCommitJob',
 ('⚙️authority', 758): 'WorldRayPickCursor', ('⚙️authority', 804): 'WorldObjectPickCursor', ('⚙️authority', 845): 'WorldComponentPickCursor',
 ('⚙️authority', 890): 'WorldMarqueePickCursor', ('⚙️authority', 1019): 'WorldGumballPickCursor',
 ('⚙️authority', 313): 'WorldInteractionRegistryBuildCursor', ('⚙️authority', 138): 'WorldInteractionAuthority',
 ('🖌️overlay', 705): 'WorldFaceOverlayScan', ('🖌️overlay', 757): 'WorldFaceOverlayScan', ('🖌️overlay', 909): 'WorldFaceOverlayMeshCursor',
 ('🧹️lifecycle', 289): 'World3dDynamicRetirement',
}

def apply_hints(allfns, files):
    for f in allfns:
        text, toks = files[f.file]
        for c in f.calls:
            if len(c['targets']) > 1:
                pos = toks[c['callee_tok']].a
                line = text.count('\n', 0, pos) + 1
                key = (f.file.split('/')[-2], line)
                ty = HINTS.get(key)
                if ty is None: raise SystemExit(f'unhinted ambiguous call {c["name"]} at {key}')
                c['targets'] = [t for t in c['targets'] if t.parent == ty]
                assert len(c['targets']) == 1, (key, ty)
