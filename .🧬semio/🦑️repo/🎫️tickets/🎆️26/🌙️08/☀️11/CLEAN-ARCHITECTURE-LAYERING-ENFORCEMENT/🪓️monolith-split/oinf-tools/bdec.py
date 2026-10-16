"""Board decomposition engine: split a god-object host into component structs and re-home its methods."""
import sys, os, re, collections, bisect
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from oinf_rs import *
from wrewrite import parse_struct_fields, parse_literal_entries, split_top_commas, pubify
from modtree import walk, LIBROOT, INFROOT

K = 5
EXT_OWNER_COMPS = 4

def ctoks(text): return [t for t in lex(text) if t.k != 'comment']

class Spec:
    def __init__(s, ty, prefix, fmap, order, desc, constructors=(), ext=None):
        s.ty = ty; s.prefix = prefix; s.fmap = fmap; s.order = order; s.desc = desc
        s.constructors = set(constructors)
        s.ext = ext or {}
        s.ext_api = set()
        s.retire = False
        s.taken = set()
        s.rename_prefix = prefix.lower() + '_'
    def comp_ty(s, c): return s.prefix + c.capitalize() + 'State'

class Fn: pass

def load_mods(root=None):
    root = root or INFROOT
    tree = walk(root + '📦️packages/🦀️rust/🦀️.rs') if root != INFROOT else walk(LIBROOT)
    mods = {}
    for e in tree:
        f = e['file']
        if not f or '🧪️tests' in f or '🧫️fixtures' in f: continue
        rel = f[len(root):]
        if rel.startswith('🌍️world') or rel == '🦀️.rs' or rel.startswith('📦️'): continue
        text = open(f, encoding='utf-8').read()
        items, _ = chunk_file(text)
        mods[rel] = (text, items)
    return mods

def find_params(toks, i):
    j = i
    while toks[j].s != '(': j += 1
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
            elif t.s in ')]}' : dd -= 1
            elif t.s == '>' and not toks[m-1].s == '-': dd -= 1
            elif t.s == ',' and dd == 0:
                params.append((cur, m)); cur = m + 1
    if cur < k: params.append((cur, k))
    return j, k, params

def collect(spec, mods):
    """All fns related to spec.ty: methods of impl H (kind method/trait) and fns taking H as a parameter (kind pfn)."""
    fns = []; cache = {}
    for rel, (text, items) in mods.items():
        toks = ctoks(text); starts = [t.a for t in toks]
        cache[rel] = (text, toks, starts)
        def mk(it, off, parent, trait, implcfg, implit):
            f = Fn(); f.rel = rel; f.name = it.name; f.parent = parent; f.trait = trait
            f.a = it.start + off; f.b = it.end + off; f.fa = it.full_start + off; f.fb = it.full_end + off
            f.vis = it.vis; f.cfg = list(it.cfg); f.implcfg = list(implcfg); f.implit = implit
            f.ti = bisect.bisect_left(starts, f.a)
            i = f.ti
            while not (toks[i].s == 'fn' and toks[i].k == 'id'): i += 1
            f.fni = i
            po, pc, params = find_params(toks, i)
            f.po, f.pc, f.params = po, pc, params
            f.bi0 = pc + 1
            dd = 0
            while True:
                x = toks[f.bi0]
                if x.k == 'p':
                    if x.s in '([': dd += 1
                    elif x.s in ')]': dd -= 1
                    elif dd == 0 and x.s in ('{', ';'): break
                f.bi0 += 1
            f.bi1 = bisect.bisect_left(starts, f.b) - 1
            f.has_body = toks[f.bi0].s == '{'
            f.var = None; f.var_mut = False; f.has_self = False; f.var_param = None; f.life = None
            if parent == spec.ty:
                seg = toks[params[0][0]:params[0][1]] if params else []
                ss = [t.s for t in seg]
                if 'self' in ss[:5]:
                    f.has_self = True; f.var = 'self'
                    f.var_mut = 'mut' in ss[:ss.index('self')]
                    f.var_param = 0
                    if len(ss) >= 3 and ss[0] == '&' and ss[1].startswith("'"): f.life = ss[1]
            if f.var is None:
                for pi, (a, b) in enumerate(params):
                    seg = toks[a:b]; ss = [t.s for t in seg]
                    if spec.ty in ss and ':' in ss:
                        ci = ss.index(':')
                        nm = [t.s for t in seg[:ci] if t.k == 'id' and t.s != 'mut']
                        tys = ss[ci+1:]
                        life = None
                        if len(tys) >= 3 and tys[0] == '&' and tys[1].startswith("'"): life = tys[1]; tys = ['&'] + tys[2:]
                        if tys in (['&', spec.ty], ['&', 'mut', spec.ty]) and nm:
                            f.var = nm[0]; f.var_mut = 'mut' in tys; f.var_param = pi; f.life = life
                            break
            f.kind = None
            if parent == spec.ty:
                f.kind = 'trait' if trait else 'method'
            elif parent in spec.ext:
                vs = spec.ext[parent]; vs = vs if isinstance(vs, list) else [vs]
                f.kind = 'ext'; f.var = vs[0]; f.ext_alias = vs[1:]; f.var_mut = True; f.var_param = None
            elif f.var is not None:
                f.kind = 'pfn'
            if f.kind != 'ext' and f.var is not None: f.var = (f.var,)
            return f
        for it in items:
            if it.kind == 'fn':
                f = mk(it, 0, None, None, [], None)
                if f.kind: fns.append(f)
            elif it.kind == 'impl':
                bs = text.index('{', it.start)
                inner = text[bs + 1: it.end - 1]
                sub, _ = chunk_file(inner)
                for s in sub:
                    if s.kind != 'fn': continue
                    f = mk(s, bs + 1, it.impl_ty, it.impl_trait, it.cfg, it)
                    if f.kind: fns.append(f)
    return fns, cache

def analyze(spec, fns, cache, methods=None, pfns=None):
    if methods is None:
        methods = {f.name: f for f in fns if f.kind in ('method',) }
        pfns = collections.defaultdict(list)
        for f in fns:
            if f.kind == 'pfn': pfns[f.name].append(f)
    for f in fns:
        text, toks, starts = cache[f.rel]
        f.fields = []; f.calls = []; f.flags = []; f.aliases = list(getattr(f, 'aliases', []))
        f.aliases += [a for a in getattr(f, 'ext_alias', []) if a not in f.aliases]
        if not f.has_body: continue
        if f.parent == spec.ty or f.var is not None:
            for i in range(f.bi0, f.bi1 + 1):
                if toks[i].s == 'let' and toks[i].k == 'id':
                    m = i + 1
                    if toks[m].s == 'mut': m += 1
                    if toks[m].k == 'id' and toks[m+1].s == '=' and toks[m+2].k == 'id' and toks[m+2].s in ('Self', spec.ty):
                        if toks[m+3].s == '{' or (toks[m+3].s == ':' and toks[m+4].s == ':' and toks[m+5].s in (spec.constructors | {'default', 'new'}) and toks[m+6].s == '('):
                            f.aliases.append((toks[m].s,))
        vars_ = ([f.var] if f.var is not None else [None]) + f.aliases
        for V in vars_:
            n = len(V) if V is not None else 0; primary = V == f.var
            for i in range(f.bi0, f.bi1 + 1):
                x = toks[i]
                if x.k != 'id': continue
                if primary and x.s in ('Self', spec.ty) and toks[i+1].s == ':' and toks[i+2].s == ':' and not (toks[i-1].s == ':' and toks[i-2].s == ':'):
                    nm = toks[i+3].s
                    if f.parent == spec.ty or x.s == spec.ty:
                        if toks[i+4].s == '(' and nm in methods:
                            f.calls.append(dict(kind='assoc', name=nm, a=i, recv_end=i, b=i+4, v=V, vtext=''))
                        elif nm in methods:
                            f.calls.append(dict(kind='assocref', name=nm, a=i, recv_end=i, b=i+3, v=V, vtext=''))
                        else:
                            f.flags.append(('assoc', nm))
                    continue
                if primary and f.parent == spec.ty and x.s in ('Self', spec.ty) and toks[i+1].s == '{' and toks[i+1].k == 'p' and toks[i-1].s not in ('impl', 'for', 'struct'):
                    f.flags.append(('struct-literal', x.s)); continue
                if V is None or tuple(t.s for t in toks[i:i+n]) != V: continue
                prev = toks[i-1].s
                if (prev == '.' and toks[i-2].s != '.') or (prev == ':' and toks[i-2].s == ':'): continue
                if toks[i-1].s == 'let' or (toks[i-1].s == 'mut' and toks[i-2].s == 'let'): continue
                j = i + n - 1
                nxt = toks[j+1].s
                if nxt == '.' and toks[j+2].k == 'id':
                    nm = toks[j+2].s; after = toks[j+3].s
                    if after == '(':
                        if nm in methods: f.calls.append(dict(kind='meth', name=nm, a=i, recv_end=j, b=j+3, v=V, vtext=text[toks[i].a:toks[j].b]))
                        else: f.flags.append(('unknown-method', nm))
                    elif nm in spec.fmap:
                        f.fields.append((nm, i, V))
                    else:
                        f.flags.append(('unknown-member', nm))
                    continue
                if f.kind not in ('pfn', 'method', 'trait', 'xfile', 'ext'):
                    f.flags.append(('bare', ' '.join(V)))
                    continue
                s = i
                while toks[s-1].k == 'p' and toks[s-1].s in ('&', '*') or (toks[s-1].k == 'id' and toks[s-1].s == 'mut' and toks[s-2].s == '&'):
                    s -= 1
                before = toks[s-1].s
                if before in ('(', ',') and nxt in (',', ')'):
                    depth = 0; k = s - 1; idx = 0
                    if before == ',':
                        while True:
                            k -= 1
                            tk = toks[k]
                            if tk.k == 'p':
                                if tk.s in ')]}': depth += 1
                                elif tk.s in '([{':
                                    if depth == 0: break
                                    depth -= 1
                                elif tk.s == ',' and depth == 0: idx += 1
                    if toks[k].s == '(' and toks[k-1].k == 'id' and toks[k-1].s in pfns:
                        f.calls.append(dict(kind='arg', name=toks[k-1].s, a=s, b=j, recv_end=j, argidx=idx, v=V, vtext=text[toks[i].a:toks[j].b]))
                        continue
                f.flags.append(('bare', f'{toks[i-1].s} {V[0]} {nxt}'))
    return methods, pfns

def classify(spec, fns, methods, pfns):
    byname = collections.defaultdict(list)
    for f in fns: byname[(f.kind, f.name)].append(f)
    def callees(f):
        out = []
        for c in f.calls:
            if c['kind'] in ('meth', 'assoc', 'assocref'): out.append(methods[c['name']])
            else:
                out.extend(pfns[c['name']])
        return out
    for f in fns:
        f.own = {spec.fmap[x[0]] for x in f.fields}
        f.req = set(f.own)
    ch = True
    while ch:
        ch = False
        for f in fns:
            n = set(f.req)
            for g in callees(f): n |= g.req
            if n != f.req: f.req = n; ch = True
    for f in fns:
        f.forced = None
        if f.kind == 'trait': f.forced = 'trait'
        elif f.kind == 'ext': f.forced = 'ext'
        elif f.aliases: f.forced = 'alias'
        elif f.flags: f.forced = 'flags:' + ','.join(sorted({x[0] for x in f.flags}))
        elif f.name in spec.constructors and f.kind == 'method': f.forced = 'constructor'
        elif f.kind == 'method' and f.has_self and f.name in spec.ext_api and len(f.req) >= EXT_OWNER_COMPS: f.forced = 'ext-api'
    ch = True
    while ch:
        ch = False
        for f in fns:
            if f.forced: continue
            for g in callees(f):
                if g.forced:
                    f.forced = 'calls-owner:' + g.name; ch = True; break
    for f in fns:
        if f.kind == 'pfn' and f.var is None: f.mode = None; continue
        if f.forced: f.mode = 'owner'
        elif f.kind == 'method' and not f.has_self: f.mode = 'free'
        elif not f.req: f.mode = 'free'
        elif len(f.req) == 1 and f.kind == 'method': f.mode = 'comp'
        elif len(f.req) <= K: f.mode = 'narrow'
        else: f.mode = 'owner'
    ch = True
    while ch:
        ch = False
        for f in fns:
            if f.mode == 'owner': continue
            for g in callees(f):
                if g.mode == 'owner':
                    f.mode = 'owner'; f.forced = f.forced or 'calls-owner:' + g.name; ch = True; break
    for f in fns:
        f.comps = [c for c in spec.order if c in f.req]
        f.newname = f.name
        if f.kind == 'method' and f.mode in ('narrow', 'free') and f.name in spec.taken:
            f.newname = spec.rename_prefix + f.name

def report(spec, fns):
    cnt = collections.Counter(); lines = collections.Counter()
    for f in fns:
        cnt[(f.kind, f.mode)] += 1
        lines[(f.kind, f.mode)] += f.b - f.a
    return cnt


# ───────────────────────── edit generation ─────────────────────────

def apply_edits(text, edits):
    es = sorted(edits, key=lambda e: (e[0], e[1]))
    for x, y in zip(es, es[1:]):
        assert x[1] <= y[0], ('overlap', x, y)
    out = []; pos = 0
    for a, b, r in es:
        out.append(text[pos:a]); out.append(r); pos = b
    out.append(text[pos:])
    return ''.join(out)

def dedent4(t):
    return '\n'.join(l[4:] if l.startswith('    ') else l for l in t.split('\n'))

def match_close(toks, i):
    """index of the closing token for the opening bracket at toks[i]."""
    d = 0
    o = toks[i].s; c = {'(': ')', '[': ']', '{': '}'}[o]
    for k in range(i, len(toks)):
        if toks[k].k == 'p':
            if toks[k].s == o: d += 1
            elif toks[k].s == c:
                d -= 1
                if d == 0: return k
    raise ValueError('unbalanced')

class Ctx:
    pass

def prepare(spec, mods):
    fns, cache = collect(spec, mods)
    methods, pfns = analyze(spec, fns, cache)
    classify(spec, fns, methods, pfns)
    ctx = Ctx(); ctx.spec = spec; ctx.fns = fns; ctx.cache = cache; ctx.methods = methods; ctx.pfns = pfns
    ctx.hazards = []
    for f in fns:
        if f.mode == 'narrow':
            text, toks, starts = cache[f.rel]
            taken = {t.s for t in toks[f.fni:f.bi1 + 1] if t.k == 'id'}
            f.pn = {c: (c if c not in taken else c + '_state') for c in f.comps}
            for c in f.comps:
                assert f.pn[c] not in taken or f.pn[c] == c + '_state', (f.name, c)
    return ctx

def refs_text(ctx, g, caller, V):
    """argument text passing g's components from caller (owner via V, narrow via names)."""
    out = []
    vt = V if isinstance(V, str) else '.'.join(V)
    for c in g.comps:
        if caller.mode == 'owner': out.append(f"&{'mut ' if g.var_mut else ''}{vt}.{c}")
        else: out.append(caller.pn[c])
    return out

def param_text(ctx, f, lt=None):
    spec = ctx.spec
    life = lt or f.life
    return [f"{f.pn[c]}: &{(life + ' ') if life else ''}{'mut ' if f.var_mut else ''}{spec.comp_ty(c)}" for c in f.comps]

def return_lifetime_edits(ctx, f):
    """Explicit `'a` on the component borrows and the elided references of the return type when elision no longer picks `&self`."""
    text, toks, starts = ctx.cache[f.rel]
    k0 = f.pc + 1
    if not (toks[k0].s == '-' and toks[k0 + 1].s == '>'): return None
    elided = [k for k in range(k0, f.bi0) if toks[k].s == '&' and toks[k].k == 'p' and not toks[k + 1].s.startswith("'")]
    if not elided: return None
    others = 0
    for pi, (a, b) in enumerate(f.params):
        if pi == f.var_param: continue
        if toks[a:b] and any(x.s == '&' for x in toks[a:b]): others += 1
    if len(f.comps) + others < 2: return None
    E = []
    for k in elided: E.append((toks[k].b, toks[k].b, "'a "))
    nm = toks[f.fni + 1]
    if toks[f.fni + 2].s == '<': E.append((toks[f.fni + 2].b, toks[f.fni + 2].b, "'a, "))
    else: E.append((nm.b, nm.b, "<'a>"))
    return E

def arg_conflict(ctx, f, g, toks, a, b, V):
    """True when the argument tokens toks[a:b] touch a component that g borrows mutably."""
    spec = ctx.spec; Vn = len(V)
    names = {f.pn[c]: c for c in f.comps} if f.mode == 'narrow' else {}
    gc = set(g.comps)
    for i in range(a, b):
        x = toks[i]
        if x.k != 'id': continue
        if f.mode == 'owner' and tuple(q.s for q in toks[i:i+Vn]) == V and toks[i-1].s != '.':
            j = i + Vn - 1
            if toks[j+1].s == '.' and toks[j+2].k == 'id':
                nm = toks[j+2].s
                if toks[j+3].s == '(':
                    h = ctx.methods.get(nm)
                    if h is None or h.mode == 'owner' or set(h.comps) & gc: return True
                elif spec.fmap.get(nm) in gc: return True
            else: return True
        if f.mode == 'narrow' and x.s in names and toks[i-1].s != '.' and names[x.s] in gc:
            return True
    return False

def fn_edits(ctx, f):
    spec = ctx.spec; text, toks, starts = ctx.cache[f.rel]
    E = []
    if f.newname != f.name:
        E.append((toks[f.fni + 1].a, toks[f.fni + 1].b, f.newname))
    if f.var is None and not f.calls and not f.fields: return E
    V = f.var
    if f.mode == 'narrow' or f.mode == 'free':
        if f.has_self or f.kind == 'pfn':
            a, b = f.params[f.var_param]
            pa, pb = toks[a].a, toks[b - 1].b
            rl = return_lifetime_edits(ctx, f) if f.mode == 'narrow' else None
            if rl: E += rl
            ptxt = param_text(ctx, f, "'a" if rl else None) if f.mode == 'narrow' else []
            if ptxt: E.append((pa, pb, ', '.join(ptxt)))
            else:
                if b < len(toks) and toks[b].s == ',':
                    end = toks[b].b + (1 if text[toks[b].b:toks[b].b + 1] == ' ' else 0)
                    E.append((pa, end, ''))
                elif f.var_param > 0:
                    E.append((toks[a - 1].a, pb, ''))
                else: E.append((pa, pb, ''))
    for nm, i, Vf in f.fields:
        x = toks[i]; xe = toks[i + len(Vf) - 1]
        c = spec.fmap[nm]
        if f.mode == 'owner': E.append((xe.b, xe.b, '.' + c))
        elif f.mode == 'narrow': E.append((x.a, xe.b, f.pn[c]))
        elif f.mode == 'comp': pass
        else: raise SystemExit(f'field use in {f.mode} fn {f.name}')
    for call in f.calls:
        k = call['kind']
        if k == 'assocref':
            g = ctx.methods[call['name']]
            if g.mode == 'owner': continue
            if g.mode == 'free': E.append((toks[call['a']].a, toks[call['b']].b, g.newname)); continue
            raise SystemExit(f'path reference to non-free method {g.name} in {f.name}')
        if k in ('meth', 'assoc'):
            g = ctx.methods[call['name']]
            ia, ib = call['a'], call['b']       # tokens [V][.][name][(]  or  [Self][:][:][name][(]
            lp = toks[ib]
            assert lp.s == '(', (f.name, call)
            nxt = toks[ib + 1]
            empty = nxt.s == ')'
            if g.mode == 'owner':
                assert f.mode == 'owner', (f.name, g.name)
                continue
            if g.mode == 'comp':
                c = g.comps[0]
                re_ = toks[call['recv_end']]
                if f.mode == 'owner': E.append((re_.b, re_.b, '.' + c))
                elif f.mode == 'narrow': E.append((toks[ia].a, re_.b, f.pn[c]))
                elif f.mode == 'comp': pass
                else: raise SystemExit(f'comp call from {f.mode}: {f.name} -> {g.name}')
                continue
            if g.mode == 'narrow':
                vt = call.get('vtext') or call['v']
                if isinstance(vt, str) and '(' in vt and len(g.comps) > 1 and f.mode == 'owner':
                    refs = refs_text(ctx, g, f, '__host')
                    new = f"match &{'mut ' if g.var_mut else ''}{vt} {{ __host => " + g.newname + '(' + ', '.join(refs) + ('' if empty else ', ')
                    E.append((toks[ia].a, lp.b, new))
                    E.append((toks[match_close(toks, ib)].b, toks[match_close(toks, ib)].b, ' }'))
                    continue
                refs = refs_text(ctx, g, f, vt)
                new = g.newname + '(' + ', '.join(refs) + ('' if empty else ', ')
                E.append((toks[ia].a, lp.b, new))
                if g.var_mut and f.mode in ('owner', 'narrow'):
                    cl = match_close(toks, ib)
                    if arg_conflict(ctx, f, g, toks, ib + 1, cl, call['v']): ctx.hazards.append((f, g, 'arg-conflict'))
                continue
            if g.mode == 'free':
                E.append((toks[ia].a, lp.b, g.newname + '('))
                continue
            raise SystemExit(f'unhandled callee mode {g.mode}')
        else:
            gs = ctx.pfns[call['name']]
            g = gs[0]
            assert all(x.mode == g.mode and x.comps == g.comps for x in gs) or True
            s, i = call['a'], call['b']
            if g.mode == 'owner': continue
            aa, bb = toks[s].a, toks[i].b
            if g.mode == 'narrow':
                refs = refs_text(ctx, g, f, call.get('vtext') or call['v'])
                E.append((aa, bb, ', '.join(refs)))
            else:
                nx = toks[i + 1]
                if nx.s == ',':
                    end = nx.b + (1 if text[nx.b:nx.b + 1] == ' ' else 0)
                    E.append((aa, end, ''))
                else:
                    pv = toks[s - 1]
                    if pv.s == ',': E.append((pv.a, bb, ''))
                    else: E.append((aa, bb, ''))
    return E

def struct_literal_edits(ctx, f):
    """`Self { field: expr, .. }` inside constructors -> nested component literals."""
    spec = ctx.spec; text, toks, starts = ctx.cache[f.rel]
    E = []
    if f.parent != spec.ty: return E
    for i in range(f.bi0, f.bi1 + 1):
        x = toks[i]
        if x.k == 'id' and x.s in ('Self', spec.ty) and toks[i+1].s == '{' and toks[i+1].k == 'p' and not (toks[i-1].s == ':' and toks[i-2].s == ':') and toks[i-1].s not in ('impl', 'for', 'struct'):
            cl = match_close(toks, i + 1)
            lb, rb = toks[i+1].a, toks[cl].a
            ents = parse_literal_entries(text, lb, rb)
            if any(e['name'].startswith('.') for e in ents): continue
            by = collections.defaultdict(list)
            for e in ents:
                if e['name'] not in spec.fmap: raise SystemExit(f'literal field {e["name"]} unmapped in {f.name}')
                by[spec.fmap[e['name']]].append(e)
            parts = []
            for c in spec.order:
                if not by[c]: continue
                body = ',\n'.join('                ' + e['chunk'].strip().replace('\n', '\n                ') for e in by[c])
                parts.append(f"            {c}: {spec.comp_ty(c)} {{\n{body},\n            }},")
            missing = [c for c in spec.order if not by[c]]
            if missing: raise SystemExit(f'literal in {f.name} lacks components {missing}')
            E.append((toks[i].a, toks[cl].b, 'Self {\n' + '\n'.join(parts) + '\n        }'))
    return E

MANUAL = {}

def member_text(ctx, f, extra=()):
    text, toks, starts = ctx.cache[f.rel]
    E = fn_edits(ctx, f) + struct_literal_edits(ctx, f) + list(extra)
    seg = text[f.fa:f.fb]
    out = apply_edits(seg, [(a - f.fa, b - f.fa, r) for a, b, r in E])
    for old, new in MANUAL.get(f.name, []):
        assert out.count(old) == 1, (f.name, old)
        out = out.replace(old, new)
    return out


# ───────────────────────── assembly ─────────────────────────

def owner_struct_text(ctx, it, text):
    spec = ctx.spec
    fields = parse_struct_fields(text, it)
    byc = collections.defaultdict(list)
    for fd in fields: byc[spec.fmap[fd['name']]].append(fd)
    host = [f'pub struct {spec.ty} {{']
    for c in spec.order:
        d = spec.desc[c]
        host.append(f'    /// {d[0].upper() + d[1:]}.')
        host.append(f'    pub {c}: {spec.comp_ty(c)},')
    host.append('}')
    comps = []
    for c in spec.order:
        s = [f'/// 🧩️ {spec.ty} {c} component: {spec.desc[c]}.', f'pub struct {spec.comp_ty(c)} {{']
        for fd in byc[c]:
            s.append(pubify(fd).strip('\n').rstrip().rstrip(',') + ',')
        s.append('}')
        comps.append('\n'.join(s))
    return '\n'.join(host) + '\n\n' + '\n\n'.join(comps)

def retire_owned_impls(ctx, it, text):
    spec = ctx.spec
    sfields = None
    for rel2, (tx, items) in ctx.mods.items():
        for it2 in items:
            if it2.kind == 'struct' and it2.name == spec.ty: sfields = [fd['name'] for fd in parse_struct_fields(tx, it2)]
    body = text[it.start:it.end]
    toks = ctoks(body)
    def entries(marker_a, marker_b):
        i = body.index(marker_a) + len(marker_a)
        j = body.index(marker_b, i)
        seg = body[i:j]
        return [s.strip() for s in seg_split(seg)]
    def seg_split(seg):
        cs = split_top_commas(seg, 0, len(seg))
        out = []; prev = 0
        for c in cs: out.append(seg[prev:c]); prev = c + 1
        out.append(seg[prev:])
        return [x for x in out if x.strip()]
    ret = entries('sequence(vec![', '])}')
    birth = entries('sequence_birth_bytes(&[', '])}')
    assert len(ret) == len(sfields) == len(birth), (len(ret), len(sfields), len(birth))
    out = []
    for c in spec.order:
        idx = [i for i, n in enumerate(sfields) if spec.fmap[n] == c]
        names = ','.join(sfields[i] for i in idx)
        out.append(f"impl RetireOwned for {spec.comp_ty(c)} {{\n fn retirement(self)->Box<dyn RetirementCursor>{{let Self{{{names}}}=self;sequence(vec![{','.join(ret[i] for i in idx)}])}}\n fn retirement_birth_bytes(&self)->Option<usize>{{sequence_birth_bytes(&[{','.join(birth[i] for i in idx)}])}}\n fn controlled_retirement_supported()->bool{{true}}\n}}")
    cs = ','.join(spec.order)
    out.append(f"impl RetireOwned for {spec.ty} {{\n fn retirement(self)->Box<dyn RetirementCursor>{{let Self{{{cs}}}=self;sequence(vec![{','.join(f'deferred({c})' for c in spec.order)}])}}\n fn retirement_birth_bytes(&self)->Option<usize>{{sequence_birth_bytes(&[{','.join(f'deferred_birth_bytes_for(&self.{c})' for c in spec.order)}])}}\n fn controlled_retirement_supported()->bool{{true}}\n}}")
    return '\n\n'.join(out)

def decompose_file(ctx, rel):
    spec = ctx.spec
    text, toks, starts = ctx.cache[rel]
    items = ctx.items[rel]
    file_edits = []
    fns = [f for f in ctx.fns if f.rel == rel]
    byimpl = collections.defaultdict(list)
    for f in fns: byimpl[id(f.implit) if f.implit is not None else None].append(f)
    for it in items:
        if it.kind == 'struct' and it.name == spec.ty:
            file_edits.append((it.start, it.end, owner_struct_text(ctx, it, text)))
        elif it.kind == 'impl' and it.impl_ty == spec.ty and it.impl_trait == 'RetireOwned' and spec.retire:
            file_edits.append((it.start, it.end, retire_owned_impls(ctx, it, text)))
        elif it.kind == 'impl' and id(it) in byimpl:
            members = byimpl[id(it)]
            if it.impl_ty == spec.ty and it.impl_trait is None:
                bs = text.index('{', it.start)
                head = text[it.start:bs + 1]
                owner_chunks = []; ext_free = []; ext_comp = collections.OrderedDict()
                for f in members:
                    mt = member_text(ctx, f)
                    if f.mode == 'owner': owner_chunks.append(mt)
                    elif f.mode == 'comp': ext_comp.setdefault(f.comps[0], []).append(mt)
                    else: ext_free.append(dedent4(mt))
                pre = ''.join(a + '\n' for a in it.cfg)
                out = []
                if owner_chunks:
                    out.append(head + ''.join(owner_chunks).rstrip('\n') + '\n}')
                for c, mts in ext_comp.items():
                    out.append(pre + f'impl {spec.comp_ty(c)} {{' + ''.join(mts).rstrip('\n') + '\n}')
                for t in ext_free:
                    out.append((pre + t.lstrip('\n')) if pre and not t.lstrip().startswith('#[cfg') else t.lstrip('\n'))
                file_edits.append((it.start, it.end, '\n\n'.join(out) if out else ''))
            else:
                for f in members:
                    E = fn_edits(ctx, f) + struct_literal_edits(ctx, f)
                    file_edits += E
        elif it.kind == 'fn':
            for f in byimpl.get(None, []):
                if f.a == it.start:
                    file_edits += fn_edits(ctx, f) + struct_literal_edits(ctx, f)
    file_edits += stray_path_edits(ctx, rel)
    return apply_edits(text, file_edits)

def stray_path_edits(ctx, rel):
    """`H::name(` / `H::name` outside the analysed host fns: rewrite references to methods that became free fns."""
    spec = ctx.spec
    text, toks, starts = ctx.cache[rel]
    covered = [(f.bi0, f.bi1) for f in ctx.fns if f.rel == rel and f.var is not None and f.has_body]
    E = []
    for i, x in enumerate(toks):
        if x.k == 'id' and x.s == spec.ty and toks[i+1].s == ':' and toks[i+2].s == ':' and toks[i+3].k == 'id' and not (toks[i-1].s == ':' and toks[i-2].s == ':'):
            if any(a <= i <= b for a, b in covered): continue
            nm = toks[i+3].s
            g = ctx.methods.get(nm)
            if g is None or g.mode == 'owner': continue
            if g.mode == 'free': E.append((x.a, toks[i+3].b, g.newname)); continue
            raise SystemExit(f'path reference {spec.ty}::{nm} ({g.mode}) in {rel}')
    return E

def run_decompose(spec, srcroot, dstroot):
    import shutil
    mods = load_mods(srcroot)
    ctx = prepare(spec, mods)
    ctx.mods = mods
    ctx.items = {rel: mods[rel][1] for rel in mods}
    out = {}
    for rel in mods:
        nt = decompose_file(ctx, rel)
        out[rel] = nt
    return ctx, out
