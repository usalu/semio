import sys; sys.path.insert(0,'.')
from wrewrite import *
import wengine as W

def load_tests(snap):
    W.INF = snap
    import copy
    dirs = [snap + '/🌍️world/🧪️tests']
    files = {}; raw = []
    for d in dirs:
        for dp, dn, fnn in os.walk(d):
            for f in sorted(fnn):
                if f.endswith('.rs'):
                    p = os.path.join(dp, f)
                    text = open(p, encoding='utf-8').read()
                    fs, toks = collect_fns(p, text)
                    files[p] = (text, toks)
                    for fn, i, pc in fs: raw.append((fn, i, pc, p))
    rets = fn_ret_types([r[0] for r in raw], files)
    out = []
    for fn, i, pc, p in raw:
        text, toks = files[p]
        if fn.parent == STATE_TY and fn.has_param:
            fn.mode = 'host'; analyze_fn(fn, toks, pc, text); out.append(fn); continue
        env = local_env(fn, toks, rets)
        names = {v for v, t in env.items() if t == STATE_TY}
        a, b = body_range(toks, fn, pc)
        if any(t.k == 'id' and t.s == 'state' for t in toks[a:b+1]): names.add('state')
        if fn.has_param and fn.pname: names.add(fn.pname)
        base_has = fn.has_param
        if not names:
            fn.has_param = False; fn.calls = []; fn.flags = []; fn.uses = collections.Counter(); fn.own = set(); out.append(fn); continue
        first = True
        for nm in sorted(names):
            f2 = copy.copy(fn) if not first else fn
            first = False
            f2.pname = nm; f2.mut = True; f2.has_param = True; f2.mode = 'host'
            f2.pstart = f2.pend = None
            analyze_fn(f2, toks, pc, text)
            out.append(f2)
    return out, files

def fn_ret_types(fns, files):
    out = {}
    for f in fns:
        text, toks = files[f.file]
        pc = f.body_first - 1
        k = pc
        while k < len(toks) and not (toks[k].s == '{' and toks[k].k == 'p') and not (toks[k].s == '-' and toks[k+1].s == '>'): k += 1
        if toks[k].s == '-':
            m = k + 2; ids = []
            while toks[m].s != '{' and toks[m].s != 'where':
                if toks[m].k == 'id' and toks[m].s[0].isupper(): ids.append(toks[m].s)
                m += 1
            if ids: out[f.name] = ids[0]
    return out

def local_env(f, toks, rets):
    env = {}
    a, b = body_range(toks, f, f.body_first - 1)
    # params: scan signature tokens
    j = a - 1
    while j > 0 and toks[j].s != 'fn': j -= 1
    for k in range(j, a):
        if toks[k].k == 'id' and toks[k+1].s == ':' and toks[k+2].s != ':':
            # type: first capitalized ident within next few tokens
            for m in range(k+2, min(k+8, a)):
                if toks[m].k == 'id' and toks[m].s[0].isupper(): env[toks[k].s] = toks[m].s; break
                if toks[m].s in (',', ')'): break
    for k in range(a, b):
        if toks[k].s == 'let' and toks[k].k == 'id':
            m = k + 1
            if toks[m].s == 'mut': m += 1
            if toks[m].k != 'id': continue
            var = toks[m].s; m += 1
            ty = None
            if toks[m].s == ':':
                for q in range(m+1, m+6):
                    if toks[q].k == 'id' and toks[q].s[0].isupper(): ty = toks[q].s; break
                    if toks[q].s == '=': break
                while toks[m].s != '=' and m < b: m += 1
            if toks[m].s == '=':
                q = m + 1
                if ty is None:
                    if toks[q].k == 'id' and toks[q].s[0].isupper() and toks[q+1].s == ':' : ty = toks[q].s
                    elif toks[q].k == 'id' and toks[q].s[0].isupper() and toks[q+1].s == '{': ty = toks[q].s
                    elif toks[q].k == 'id' and toks[q+1].s == '(' and toks[q].s in rets: ty = rets[toks[q].s]
            if ty: env[var] = ty
    return env

def resolve_ambiguous_tests(tfns, tfiles):
    rets = fn_ret_types(tfns, tfiles)
    miss = []
    for f in tfns:
        text, toks = tfiles[f.file]
        env = None
        for c in f.calls:
            if len(c['targets']) > 1:
                if env is None: env = local_env(f, toks, rets)
                ci = c['callee_tok']
                recv = toks[ci - 2] if toks[ci - 1].s == '.' else None
                ty = env.get(recv.s) if recv is not None and recv.k == 'id' else None
                cand = [t for t in c['targets'] if t.parent == ty] if ty else []
                if len(cand) == 1: c['targets'] = cand
                else: miss.append((f, c, recv.s if recv else None, ty))
    return miss
