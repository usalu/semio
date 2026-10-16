# 🧩 A1 library: method records, signature parsing, need sets and the token-level receiver rewrite of the ArtifactStore decomposition (PLAN-OS-KERNEL-CRATES.md 7.2 to 7.5).
import re, collections, json
import em, rs
from okrs import tokenize

COMPS = em.COMPONENTS
RANK = em.RANK
STRUCT = em.STRUCT
FIELD_COMP = em.FIELD_COMP

def ctoks(text):
    """code tokens (no ws/comments) with positions: (kind, text, start, end)"""
    return [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]

class Method:
    pass

def parse_signature(text):
    """-> dict(prefix_end, name, gen_span, lp, rp, recv, recv_span, params, ret_start, body_start, is_async, vis)"""
    toks = ctoks(text)
    k = 0
    # skip attrs
    while k < len(toks) and toks[k][1] == '#':
        k += 1
        while toks[k][1] != '[': k += 1
        d = 0
        while True:
            if toks[k][1] == '[': d += 1
            elif toks[k][1] == ']':
                d -= 1
                if d == 0: k += 1; break
            k += 1
    vis_start = k
    while toks[k][1] != 'fn': k += 1
    pre = [t[1] for t in toks[vis_start:k]]
    is_async = 'async' in pre
    fn_i = k
    name = toks[k + 1][1]
    j = k + 2
    gen = None
    if toks[j][1] == '<':
        d = 0; s = j
        while True:
            if toks[j][1] == '<': d += 1
            elif toks[j][1] == '>' and toks[j - 1][1] != '-':
                d -= 1
                if d == 0: break
            j += 1
        gen = (toks[s][2], toks[j][3]); j += 1
    assert toks[j][1] == '(', (name, toks[j])
    lp = j; d = 0
    while True:
        if toks[j][1] == '(': d += 1
        elif toks[j][1] == ')':
            d -= 1
            if d == 0: break
        j += 1
    rp = j
    # params split at depth 0 (angle aware)
    params = []; cur = []; d = 0; ang = 0
    for q in range(lp + 1, rp):
        tx = toks[q][1]
        if tx in '([{': d += 1
        elif tx in ')]}': d -= 1
        elif tx == '<': ang += 1
        elif tx == '>' and toks[q - 1][1] != '-' : ang = max(0, ang - 1)
        if tx == ',' and d == 0 and ang == 0:
            params.append(cur); cur = []
        else: cur.append(toks[q])
    if cur: params.append(cur)
    recv = None; recv_n = 0
    if params:
        p0 = [t[1] for t in params[0]]
        if p0 and ((p0[0] == '&' and 'self' in p0 and (p0[-1] == 'self')) or p0 == ['self'] or p0 == ['mut', 'self']):
            recv = ' '.join(p0).replace('& ', '&').replace("' a", "'a"); recv_n = 1
    # body start
    b = rp + 1; d = 0; ang = 0
    while b < len(toks):
        tx = toks[b][1]
        if tx == '{' and d == 0: break
        if tx == ';' and d == 0: break
        if tx in '([': d += 1
        elif tx in ')]': d -= 1
        b += 1
    return dict(name=name, is_async=is_async, vis=' '.join(t for t in pre if t not in ('async', 'unsafe', 'const')) , gen=gen, lp=toks[lp][2], rp=toks[rp][2], recv=recv, params=[(' '.join(t[1] for t in p), p[0][2], p[-1][3]) for p in params], ptoks=params, body_start=toks[b][2] if b < len(toks) else None, fn_pos=toks[fn_i][2], toks=toks, lpi=lp, rpi=rp)

def param_names(sig):
    out = []
    for p in sig['ptoks'][1 if sig['recv'] else 0:]:
        # pattern before first top-level ':'
        d = 0; pat = []
        for t in p:
            if t[1] in '([{': d += 1
            elif t[1] in ')]}': d -= 1
            if t[1] == ':' and d == 0: break
            pat.append(t[1])
        pat = [x for x in pat if x != 'mut']
        out.append(pat)
    return out

def load_methods(text):
    tab = em.method_table(text)
    info, home = em.homes(text, tab)
    ms = collections.OrderedDict()
    for n, m in tab.items():
        if n.startswith('<'): continue
        r = Method(); r.name = n; r.raw = m; r.text = m['text']; r.sig = parse_signature(m['text'])
        r.block = m['block']; r.vis_pub = m['vis']
        r.comps = info[n]['comps']; r.callees = info[n]['callees']; r.home = home[n]
        r.recv = r.sig['recv']
        ms[n] = r
    # need sets
    need = {n: set() for n in ms}
    for _ in range(40):
        changed = False
        for n, r in ms.items():
            if r.home == 'ops': continue
            s = set(r.comps)
            for h in r.callees:
                hh = ms[h]
                if hh.home == 'ops': raise Exception(('component method calls composer method', n, h))
                s.add(hh.home); s |= need[h]
            s.discard(r.home)
            if s != need[n]: need[n] = s; changed = True
        if not changed: break
    for n, r in ms.items():
        r.need = sorted(need[n], key=lambda c: RANK[c])
        for c in r.need:
            assert RANK[c] < RANK[r.home], ('need not below home', n, r.home, c)
    return ms

def mut_of(recv):
    if recv is None: return None
    return 'mut' in re.split(r"[\s&']+", recv)

PNAME = {c: 'store_' + c for c in COMPS}

def comp_ty(c): return f'{STRUCT[c]}<P, Mutation>'

def param_decl(c, mutable):
    return f'{PNAME[c]}: &{"mut " if mutable else ""}{comp_ty(c)}'

class Rewrite:
    def __init__(self):
        self.warn = []

def apply_edits(text, edits):
    out = text
    for s, e, r in sorted(edits, key=lambda x: (x[0], x[1]), reverse=True):
        out = out[:s] + r + out[e:]
    return out

def call_args(c_names, mutable):
    return [f'&{"mut " if mutable else ""}self.{c}' for c in c_names]

def rewrite_body(r, ms, ops_ctx=False, ctx_name=None, strict=True):
    """Rewrite one method (component-homed: `self.field` of a lower component becomes the parameter, callee arguments are inserted; composer-homed: field paths gain the component hop).
    Returns (new text, warnings)."""
    text = r.text
    sig = r.sig
    toks = sig['toks']
    body_start = sig['body_start']
    edits = []; warns = []
    home = r.home
    n = len(toks)
    for i, t in enumerate(toks):
        if t[2] < (body_start or 0): continue
        if t[0] == 'id' and t[1] == 'self':
            a = toks[i + 1] if i + 1 < n else None
            b = toks[i + 2] if i + 2 < n else None
            if a is not None and a[1] == '.' and b is not None and b[0] == 'id':
                c = toks[i + 3] if i + 3 < n else None
                is_call = c is not None and (c[1] == '(' or (c[1] == ':' and i + 4 < n and toks[i + 4][1] == ':'))
                name = b[1]
                if not is_call and name in FIELD_COMP:
                    comp = FIELD_COMP[name]
                    if home == 'ops':
                        edits.append((t[3], t[3], f'.{comp}'))
                    elif comp == home:
                        pass
                    else:
                        assert comp in r.need, (r.name, name, comp, r.need)
                        edits.append((t[2], t[3], PNAME[comp]))
                elif is_call and name in ms:
                    h = ms[name]
                    if home == 'ops':
                        pass
                    else:
                        if h.home == home:
                            ins = list(h.need)
                            if ins:
                                for q in ins: assert q in r.need, (r.name, name, q)
                                po = i + 3
                                if toks[po][1] == ':':
                                    d = 0; po += 2
                                    while True:
                                        if toks[po][1] == '<': d += 1
                                        elif toks[po][1] == '>' and toks[po - 1][1] != '-':
                                            d -= 1
                                            if d == 0: break
                                        po += 1
                                    po += 1
                                assert toks[po][1] == '(', (r.name, name)
                                nxt = toks[po + 1]
                                args = ', '.join(PNAME[q] for q in ins) + ('' if nxt[1] == ')' else ', ')
                                edits.append((toks[po][3], toks[po][3], args))
                        else:
                            assert RANK[h.home] < RANK[home], ('upward call', r.name, name)
                            assert h.home in r.need, (r.name, name, h.home, r.need)
                            edits.append((t[2], t[3], PNAME[h.home]))
                            ins = list(h.need)
                            for q in ins: assert q in r.need, (r.name, name, q)
                            po = i + 3
                            if toks[po][1] == ':':
                                d = 0; po += 2
                                while True:
                                    if toks[po][1] == '<': d += 1
                                    elif toks[po][1] == '>' and toks[po - 1][1] != '-':
                                        d -= 1
                                        if d == 0: break
                                    po += 1
                                po += 1
                            assert toks[po][1] == '(', (r.name, name)
                            nxt = toks[po + 1]
                            if ins:
                                args = ', '.join(PNAME[q] for q in ins) + ('' if nxt[1] == ')' else ', ')
                                edits.append((toks[po][3], toks[po][3], args))
                elif a is not None and a[1] == '.' and name not in FIELD_COMP and name not in ms and is_call is False:
                    warns.append(('self.<unknown>', r.name, name))
            else:
                # bare self use
                prev = toks[i - 1][1] if i else ''
                if i > 0 and t[2] >= body_start:
                    warns.append(('bare self', r.name, ' '.join(x[1] for x in toks[max(0, i - 3): i + 4])))
        elif t[0] == 'id' and t[1] == 'Self' and i + 3 < n and toks[i + 1][1] == ':' and toks[i + 2][1] == ':' and toks[i + 3][0] == 'id':
            name = toks[i + 3][1]
            key = name
            if name in ms:
                h = ms[name]
                if h.home != home:
                    edits.append((t[2], toks[i + 2][3], f'{STRUCT[h.home]}::<P, Mutation>::' if h.home != 'ops' else 'Self::'))
            elif f'<const {name}>' in CONSTS:
                ch = CONSTS[f'<const {name}>']
                if ch != home:
                    edits.append((t[2], toks[i + 2][3], f'{STRUCT[ch]}::<P, Mutation>::'))
    # signature: lower params
    new = text
    if home != 'ops' and r.need:
        assert sig['recv'] is not None, ('static method with needs', r.name)
        mutable = mut_of(sig['recv'])
        p0end = sig['ptoks'][0][-1][3]
        decl = ''.join(', ' + param_decl(c, mutable) for c in r.need)
        edits.append((p0end, p0end, decl))
    return apply_edits(text, edits), warns

CONSTS = {}
