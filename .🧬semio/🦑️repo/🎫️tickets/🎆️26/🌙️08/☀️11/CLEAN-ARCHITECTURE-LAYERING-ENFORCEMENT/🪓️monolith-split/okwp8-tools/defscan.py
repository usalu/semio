"""🔎 Definition scan: struct fields and inherent methods of the Store crates with their visibility, and the names the test crates use on them."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import items as parse_items, tokenize
import nsboot, ns, unitgraph, vis as V
CAT = nsboot.boot()
def family():
    fam = []
    for name, c in CAT.items():
        if not name.startswith('semio-framework-os-'): continue
        if any(k in name for k in ('-tests', '-testing-fixtures', 'os-kernel', 'os-flow', 'os-infinite', 'os-mcp', 'os-renderer', 'os-node-graph', 'os-kernel-db', 'neural')): continue
        fam.append(name)
    return fam
def visibility_of(it_text, kind):
    toks = V.sig_tokens(it_text)
    i = V.skip_attrs(toks, 0)
    j = V.vis_span(toks, i)
    return it_text[toks[i][2]:toks[j - 1][3]] if j != i else ''
def fields_of(it):
    toks = V.sig_tokens(it['text'])
    i = V.skip_attrs(toks, 0)
    k = V.vis_span(toks, i) + 2
    while k < len(toks) and toks[k][1] not in ('{', '(', ';'):
        if toks[k][1] == '<':
            d = 0
            while k < len(toks):
                if toks[k][1] == '<': d += 1
                elif toks[k][1] == '>' and not (k > 0 and toks[k - 1][1] == '-'):
                    d -= 1
                    if d == 0: break
                k += 1
        k += 1
    kk = k
    while kk < len(toks) and toks[kk][1] not in ('{', ';'):
        if toks[kk][1] == '(' and kk == k: break
        kk += 1
    if k < len(toks) and toks[k][1] in ('{', '('): kk = k
    out = []
    if kk < len(toks) and toks[kk][1] in ('{', '('):
        close = V.match_close(toks, kk)
        for idx, (s, e) in enumerate(V.split_top(toks, kk + 1, close)):
            p = V.skip_attrs(toks, s)
            if p >= e: continue
            q = V.vis_span(toks, p)
            vtxt = it['text'][toks[p][2]:toks[q - 1][3]] if q != p else ''
            if toks[kk][1] == '{':
                if toks[q][0] == 'id': out.append((toks[q][1], vtxt))
            else: out.append(('%d' % idx, vtxt))
    return out
def methods_of(it):
    toks = V.sig_tokens(it['text'])
    i = V.skip_attrs(toks, 0)
    k = i
    while k < len(toks) and toks[k][1] != '{': k += 1
    end = V.match_close(toks, k)
    out = []
    p = k + 1
    while p < end:
        p = V.skip_attrs(toks, p)
        if p >= end: break
        q = V.vis_span(toks, p)
        w = toks[q][1] if q < end else ''
        vtxt = it['text'][toks[p][2]:toks[q - 1][3]] if q != p else ''
        r = q
        while r < end and toks[r][1] in ('const', 'async', 'unsafe', 'extern'): r += 1
        if r < end and toks[r][1] in ('fn', 'static', 'type') and r + 1 < end:
            out.append((toks[r + 1][1], vtxt, toks[r][1]))
        elif w == 'const' and q + 1 < end: out.append((toks[q + 1][1], vtxt, 'const'))
        r = q; 
        while r < end:
            t = toks[r][1]
            if t in ('{', '(', '['):
                r = V.match_close(toks, r) + 1
                if t == '{': break
                continue
            if t == ';': r += 1; break
            r += 1
        p = r
    return out
def build_index():
    fields = collections.defaultdict(list); methods = collections.defaultdict(list); structs = {}
    for name in family():
        c = CAT[name]; ident = c['lib_name'] or name.replace('-', '_')
        w = ns.ext_world(ident)
        if w is None: continue
        for m in w.mods:
            if m.key != m.file and '#' in m.key: pass
            try: its = parse_items(m.text)
            except Exception: continue
            for it in its:
                if it['kind'] in ('struct', 'union') and it['name']:
                    for fn, v in fields_of(it): fields[fn].append((name, it['name'], v, m.file))
                elif it['kind'] == 'impl':
                    tr, st = unitgraph.impl_parts(it['text'])
                    if tr is None:
                        for mn, v, kind in methods_of(it): methods[mn].append((name, st, v, m.file, kind))
    return fields, methods
def is_pub(v): return v == 'pub'
if __name__ == '__main__':
    f, m = build_index()
    print(len(f), len(m))
    import pickle
    pickle.dump((dict(f), dict(m)), open(os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated/okwp8-defindex.pkl', 'wb'))

def usage(text):
    """Returns (dot_names, assoc_pairs, literal_fields)."""
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    dots = collections.Counter(); assoc = collections.Counter(); lits = collections.Counter()
    n = len(toks)
    for i, t in enumerate(toks):
        if t[0] != 'id': continue
        if i and toks[i - 1][1] == '.' and not (i > 1 and toks[i - 2][1] == '.'):
            dots[t[1]] += 1
        if i + 3 < n and toks[i + 1][1] == ':' and toks[i + 2][1] == ':' and toks[i + 3][0] == 'id' and t[1][0].isupper():
            assoc[(t[1], toks[i + 3][1])] += 1
        if t[1][0].isupper() and i + 1 < n and toks[i + 1][1] == '{':
            # struct literal / pattern: collect `ident :` pairs at depth 1
            d = 0; j = i + 1
            while j < n:
                x = toks[j][1]
                if x in ('{', '(', '['): d += 1
                elif x in (')', ']', '}'):
                    d -= 1
                    if d == 0: break
                elif d == 1 and toks[j][0] == 'id' and j + 1 < n and toks[j + 1][1] == ':' and (j + 2 >= n or toks[j + 2][1] != ':') and toks[j - 1][1] in ('{', ','):
                    lits[(t[1], toks[j][1])] += 1
                j += 1
    return dots, assoc, lits
def flag(texts, fields, methods, structs_by_name):
    """texts: {path: text}. Returns dict kind -> {name: [(crate, owner, vis)...]} for names that only non-pub definitions provide."""
    dots = collections.Counter(); assoc = collections.Counter(); lits = collections.Counter()
    for t in texts.values():
        a, b, c = usage(t)
        dots.update(a); assoc.update(b); lits.update(c)
    flagged = collections.OrderedDict()
    local_defs = set()
    for t in texts.values():
        for m in re.finditer(r'\bfn\s+([a-z_][A-Za-z0-9_]*)', t): local_defs.add(m.group(1))
        for m in re.finditer(r'\b([a-z_][A-Za-z0-9_]*)\s*:', t): local_defs.add(m.group(1))
    res = {'field_or_method': [], 'assoc': [], 'literal': []}
    for name, cnt in dots.items():
        fs = fields.get(name, []); ms = methods.get(name, [])
        if not fs and not ms: continue
        pubs = [x for x in fs if is_pub(x[2])] + [x for x in ms if is_pub(x[2])]
        non = [x for x in fs if not is_pub(x[2])] + [x for x in ms if not is_pub(x[2])]
        if non and not pubs:
            res['field_or_method'].append((name, cnt, 'LOCAL' if name in local_defs else '', non[:4]))
        elif non and pubs:
            res['field_or_method'].append((name, cnt, 'AMBIG', non[:3]))
    for (ty, nm), cnt in assoc.items():
        ms = [x for x in methods.get(nm, []) if x[1] == ty]
        if ms and not any(is_pub(x[2]) for x in ms): res['assoc'].append((ty, nm, cnt, ms[:2]))
    for (ty, fld), cnt in lits.items():
        fs = [x for x in fields.get(fld, []) if x[1] == ty]
        if fs and not any(is_pub(x[2]) for x in fs): res['literal'].append((ty, fld, cnt, fs[:2]))
    return res
