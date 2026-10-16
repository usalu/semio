"""Infer, per function of an outside file, which receiver chains hold a BoardHost/DagHost, and rewrite them."""
import sys, os, re, collections, bisect
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bdec, bxfile
from oinf_rs import *

HOSTS = ('BoardHost', 'DagHost')

def fn_ranges(text, toks, starts):
    """(name, tok0, tok1, impl_ty) for every fn item and impl member."""
    items, _ = chunk_file(text)
    out = []
    def add(it, off, parent):
        a = bisect.bisect_left(starts, it.start + off)
        b = bisect.bisect_left(starts, it.end + off) - 1
        out.append((it.name, a, b, parent))
    for it in items:
        if it.kind == 'fn': add(it, 0, None)
        elif it.kind == 'impl':
            bs = text.index('{', it.start)
            sub, _ = chunk_file(text[bs + 1: it.end - 1])
            for s in sub:
                if s.kind == 'fn': add(s, bs + 1, it.impl_ty)
    # token gaps (macro bodies, inline modules, consts) are scanned as one more region each
    cov = sorted((a, b) for _, a, b, _ in out)
    pos = 0; gaps = []
    for a, b in cov:
        if a > pos: gaps.append((pos, a - 1))
        pos = max(pos, b + 1)
    if pos < len(toks): gaps.append((pos, len(toks) - 1))
    for k, (a, b) in enumerate(gaps): out.append(('<gap%d>' % k, a, b, None))
    return out

def type_names_in_file(toks, h, ret_fns):
    """identifiers declared with type h anywhere (struct fields, params, lets); ret_fns: fn names returning h."""
    decl = set(); wrapped = set()
    n = len(toks)
    for i in range(n):
        t = toks[i]
        if t.k == 'id' and t.s == h:
            # NAME : [&|&mut]* [wrappers<]* h
            j = i - 1
            while j >= 2 and toks[j].s == ':' and toks[j-1].s == ':' and toks[j-2].k == 'id': j -= 3
            while j >= 0 and (toks[j].s in ('&', 'mut', '<', "'static") or toks[j].k == 'id' and toks[j].s in ('RefCell', 'Rc', 'Arc', 'Box', 'Option', 'Mutex', 'Cell', 'ManuallyDrop', 'Vec', 'mut') or toks[j].s.startswith("'")):
                j -= 1
            if j >= 1 and toks[j].s == ':' and toks[j-1].k == 'id' and toks[j-2].s != ':':
                decl.add(toks[j-1].s)
            # let NAME = h :: ctor (
            if i + 2 < n and toks[i+1].s == ':' and toks[i+2].s == ':':
                q = i
                while q >= 3 and toks[q-1].s == ':' and toks[q-2].s == ':' and toks[q-3].k == 'id': q -= 3
                if q >= 2 and toks[q-1].s == '=' and toks[q-2].k == 'id': decl.add(toks[q-2].s)
        if t.k == 'id' and i + 1 < n and toks[i+1].s == '(' and t.s in ret_fns and i >= 2 and toks[i-1].s == '=' and toks[i-2].k == 'id':
            decl.add(toks[i-2].s)
    return decl

def fns_returning(toks, h):
    out = set()
    for i, t in enumerate(toks):
        if t.s == 'fn' and t.k == 'id':
            j = i + 2
            d = 0
            while j < len(toks):
                if toks[j].s in ('(', '['): d += 1
                elif toks[j].s in (')', ']'): d -= 1
                elif d == 0 and toks[j].s in ('{', ';'): break
                if d == 0 and toks[j].s == '-' and toks[j+1].s == '>':
                    k = j + 2
                    while toks[k].s not in ('{', ';', 'where'):
                        if toks[k].s == h: out.add(toks[i+1].s)
                        k += 1
                    break
                j += 1
    return out

def chain_before(toks, i):
    """receiver token indices [j0, j1] of the expression ending right before toks[i] ('.'), or None."""
    j = i - 1
    if toks[j].s != '.': return None
    j -= 1
    end = j
    while j >= 0:
        if toks[j].s in (')', ']'):
            d = 0
            while True:
                if toks[j].s in (')', ']'): d += 1
                elif toks[j].s in ('(', '['): d -= 1
                if d == 0: break
                j -= 1
            j -= 1
            continue
        if toks[j].k == 'id':
            if toks[j-1].s == '.' : j -= 2; continue
            return (j, end)
        break
    return None

GLOBAL_RET = {'BoardHost': {'puzzle_board_host', 'puzzle_board_host_normal', 'new_normal'}, 'DagHost': set()}
GLOBAL_NAMES = {'BoardHost': {'catalog_host', 'empty_host', 'puzzle', 'board_host'}, 'DagHost': {'dag'}}
FILE_EXTRA = {}
TEMP_FIXES = [
    ('BoardHost::new().set_board_kind_catalogs_from_json(', 'BoardHost::new().style.set_board_kind_catalogs_from_json('),
    ('puzzle_board_host().naming()', 'puzzle_board_host().style.naming()'),
    ('puzzle_board_host_normal().naming()', 'puzzle_board_host_normal().style.naming()'),
    ('DagHost::from_host_snapshot(empty_fixture).entity_screen_json("node", "*")', '{ let host = DagHost::from_host_snapshot(empty_fixture); entity_screen_json(&host.viewport, &host.graph, "node", "*") }'),
]

def infer(text, rel, meta, specs_by_host):
    toks = bdec.ctoks(text); starts = [t.a for t in toks]
    ranges = fn_ranges(text, toks, starts)
    plan = []   # (fn name, tok0, tok1, host, receivers set(str))
    ret = {h: fns_returning(toks, h) | GLOBAL_RET[h] for h in HOSTS}
    glob = {h: type_names_in_file(toks, h, ret[h]) for h in HOSTS}
    for name, a, b, parent in ranges:
        seg = text[toks[a].a:toks[b].b]
        for h in HOSTS:
            if h not in text: continue
            moved = {n for n, m in meta[h][0].items() if m.mode != 'owner'}
            fields = set(specs_by_host[h].fmap)
            recvs = collections.Counter()
            local = type_names_in_file(toks[a:b+1], h, ret[h])
            key = 'board_host' if h == 'BoardHost' else 'dag_host'
            for i in range(a, b + 1):
                if toks[i].k == 'id' and key in toks[i].s and toks[i+1].s == '(':
                    d = 0; j = i + 1
                    while j <= b:
                        if toks[j].s == '(': d += 1
                        elif toks[j].s == ')':
                            d -= 1
                            if d == 0: break
                        elif toks[j].s == '|' and d == 1 and toks[j+1].k == 'id' and toks[j+2].s in ('|', ':'):
                            local.add(toks[j+1].s)
                        j += 1
            names = glob.get(h, set()) | local | GLOBAL_NAMES[h] | set(FILE_EXTRA.get((rel, h), ()))
            for _round in range(2):
                for i in range(a, b + 1):
                    if toks[i].s == '=' and toks[i-1].s != '=' and toks[i+1].s != '=' and toks[i-1].s not in ('<', '>', '!'):
                        j = i + 1; d = 0; ids = set(); calls = set()
                        while j <= b:
                            s = toks[j].s
                            if s in ('(', '[', '{'): d += 1
                            elif s in (')', ']', '}'):
                                d -= 1
                                if d < 0: break
                            elif d == 0 and s in (';', 'else'): break
                            if toks[j].k == 'id': ids.add(s)
                            j += 1
                        if (ids & names) and (ids & {'as_mut', 'as_ref', 'unwrap', 'expect', 'borrow_mut', 'borrow', 'get', 'get_mut', 'lock', 'take'} or len(ids & names) > 0):
                            q = i - 1
                            binder = None
                            if toks[q].s == ')' and toks[q-2].s == '(' and toks[q-3].s in ('Some', 'Ok') and toks[q-1].k == 'id': binder = toks[q-1].s
                            elif toks[q].k == 'id' and toks[q-1].s in ('let', 'mut'): binder = toks[q].s
                            if binder and binder not in names and (ids - names - {'as_mut', 'as_ref', 'unwrap', 'expect', 'borrow_mut', 'borrow', 'get', 'get_mut', 'lock', 'take', 'entry', 'self', 'surface_id', 'cell', 'map', 'Some', 'Ok', 'cache', 'mut', 'ref', 'and_then', 'map', 'ok_or', 'ok_or_else', 'map_err', 'get_or_insert_with'}) == set():
                                names = names | {binder}
            for i in range(a, b + 1):
                x = toks[i]
                if x.k != 'id' or toks[i-1].s != '.': continue
                isM = x.s in moved and toks[i+1].s == '('
                isF = x.s in fields and toks[i+1].s != '('
                if not (isM or isF): continue
                ch = chain_before(toks, i)
                if ch is None: continue
                j0, j1 = ch
                last = toks[j1].s
                lastid = last if toks[j1].k == 'id' else None
                if lastid is None:
                    continue
                if lastid in names:
                    recvs[text[toks[j0].a:toks[j1].b]] += 1
            pf = {k for k, v in meta[h][1].items() if any(m.parent in (None, 'None') and not m.has_self for m in v)}
            for i in range(a, b + 1):
                x = toks[i]
                if x.k != 'id' or x.s not in pf or toks[i+1].s != '(' or toks[i-1].s in ('.', 'fn'): continue
                d = 0; j = i + 1; k0 = i + 2
                while j <= b:
                    s = toks[j].s
                    if s in ('(', '[', '{'): d += 1
                    elif s in (')', ']', '}'):
                        d -= 1
                        if d == 0: break
                    elif s == ',' and d == 1: break
                    j += 1
                while toks[k0].s in ('&', 'mut') and k0 < j: k0 += 1
                if k0 >= j: continue
                ex = text[toks[k0].a:toks[j-1].b]
                if ex.split('.')[-1] in names | {'host', 'dag'}:
                    recvs[ex] += 1
            if recvs: plan.append((name, a, b, h, set(recvs)))
    return plan

def rewrite_file(text, rel, meta, specs_by_host, overrides=None):
    toks = bdec.ctoks(text); starts = [t.a for t in toks]
    plan = infer(text, rel, meta, specs_by_host)
    if overrides: plan = overrides(plan)
    # a function claimed by both hosts with an identical receiver is ambiguous
    bad = []
    byfn = collections.defaultdict(list)
    for p in plan: byfn[(p[0], p[1])].append(p)
    allE = []; haz = []; flags = []
    drop = collections.defaultdict(set)
    for key, ps in byfn.items():
        seen = collections.Counter(tk for p in ps for tk in {tuple(x.s for x in bdec.ctoks(r)) for r in p[4]})
        for p in ps:
            for r in p[4]:
                if seen[tuple(x.s for x in bdec.ctoks(r))] > 1:
                    bad.append((p[0], p[3], r)); drop[(p[0], p[1])].add(tuple(x.s for x in bdec.ctoks(r)))
    plan = [(n, a, b, h, {r for r in recvs if tuple(x.s for x in bdec.ctoks(r)) not in drop[(n, a)]}) for (n, a, b, h, recvs) in plan]
    plan = [p for p in plan if p[4]]
    allE = []; haz = []; flags = []
    for name, a, b, h, recvs in plan:
        f = bdec.Fn()
        spec = specs_by_host[h]
        methods, pfns = meta[h]
        f.rel = rel; f.name = '<fn %s>' % name; f.parent = None; f.trait = None; f.kind = 'xfile'; f.mode = 'owner'
        f.a = 0; f.b = len(text); f.fa = 0; f.fb = len(text)
        f.var = None; f.var_mut = True; f.has_self = False; f.var_param = None; f.life = None; f.implit = None
        f.fni = 0; f.po = f.pc = 0; f.params = []; f.bi0 = a; f.bi1 = b; f.has_body = True; f.ti = 0
        f.newname = f.name; f.comps = []
        f.aliases = sorted({tuple(t.s for t in bdec.ctoks(r)) for r in recvs})
        cache = {rel: (text, toks, starts)}
        ctx = bxfile.XCtx(); ctx.spec = spec; ctx.methods = methods; ctx.pfns = {k: [m for m in v if m.parent in (None, 'None')] for k, v in pfns.items() if any(m.parent in (None, 'None') for m in v)}; ctx.cache = cache; ctx.hazards = []; ctx.fns = [f]
        bdec.analyze(spec, [f], cache, methods, ctx.pfns)
        E = bdec.fn_edits(ctx, f)
        allE += E; haz += [(name, h2.name, g.name) for h2, g, w in ctx.hazards]; flags += [(name, x) for x in f.flags if x[0] != 'assoc']
    allE += pathref_edits(text, toks, meta, specs_by_host, plan)
    out = bdec.apply_edits(text, allE)
    for old, new in TEMP_FIXES:
        out = out.replace(old, new)
    return out, plan, haz, flags, bad

def pathref_edits(text, toks, meta, specs, plan):
    """`path::H::method` used as a function value: wrap into a closure over the decomposed call."""
    E = []
    for h in HOSTS:
        methods, pfns = meta[h]
        for i, x in enumerate(toks):
            if x.k != 'id' or x.s != h or toks[i+1].s != ':' or toks[i+2].s != ':' or toks[i+3].k != 'id': continue
            nm = toks[i+3].s
            g = methods.get(nm)
            if g is None or g.mode == 'owner' or toks[i+4].s == '(': continue
            j = i
            while j >= 3 and toks[j-1].s == ':' and toks[j-2].s == ':' and toks[j-3].k == 'id': j -= 3
            ar = ['__h'] + [f'__a{k}' for k in range(g.nparams)]
            args = ', '.join(ar[1:])
            mutq = 'mut ' if g.var_mut else ''
            if g.mode == 'comp': body = f"__h.{g.comps[0]}.{nm}({args})"
            elif g.mode == 'free': body = f"{g.newname}({args})"
            else:
                refs = ', '.join(f"&{mutq}__h.{c}" for c in g.comps)
                body = f"{g.newname}({refs}{', ' if args else ''}{args})"
            E.append((toks[j].a, toks[i+3].b, f"|{', '.join(ar)}| {body}"))
    return E

# ───────────── import additions for newly free functions ─────────────

def new_free_names(before, after, meta):
    names = {}
    for h in HOSTS:
        for n, m in meta[h][0].items():
            if m.kind == 'method' and m.mode in ('narrow', 'free'): names.setdefault(m.newname, h)
    bt = {t.s for t in bdec.ctoks(before) if t.k == 'id'}
    at = bdec.ctoks(after)
    used = set()
    for i, t in enumerate(at):
        if t.k == 'id' and t.s in names and i + 1 < len(at) and at[i+1].s == '(' and (i == 0 or at[i-1].s not in ('.', 'fn')) and not (i >= 2 and at[i-1].s == ':' and at[i-2].s == ':'):
            used.add(t.s)
    return {n: names[n] for n in used if n not in bt}

def crate_alias(text, host):
    """the path prefix under which this file reaches the infinite crate (the `use` that brings the host in)."""
    m = re.search(r'\buse\s+([A-Za-z_][A-Za-z0-9_]*)(?:::[a-z_0-9:]+)*::\{[^}]*\b' + host + r'\b', text)
    if m: return m.group(1)
    m = re.search(r'\buse\s+([A-Za-z_][A-Za-z0-9_]*)::(?:[a-z_0-9]+::)*' + host + r'\b', text)
    if m: return m.group(1)
    m = re.search(r'\b([A-Za-z_][A-Za-z0-9_]*)::' + host + r'\b', text)
    if m: return m.group(1)
    if 'semio_framework_os_infinite::' in text: return 'semio_framework_os_infinite'
    return None

def add_imports(text, names_by_host):
    """insert `use <alias>::{...};` lines after the last top-level use item (or at the top)."""
    items, _ = chunk_file(text)
    lines = []
    for h, ns in names_by_host.items():
        alias = crate_alias(text, h) or 'semio_framework_os_infinite'
        lines.append(f"use {alias}::{{{', '.join(sorted(ns))}}};" if len(ns) > 1 else f"use {alias}::{sorted(ns)[0]};")
    if not lines: return text
    uses = [it for it in items if it.kind in ('use', 'extern_crate') and not it.text.lstrip().startswith('#!')]
    ins = ('\n'.join(lines)) + '\n'
    if uses:
        last = uses[-1]
        pos = text.index('\n', last.end) + 1 if '\n' in text[last.end:] else len(text)
        return text[:pos] + ins + text[pos:]
    first = items[0].full_start if items else 0
    return text[:first] + ins + text[first:]
