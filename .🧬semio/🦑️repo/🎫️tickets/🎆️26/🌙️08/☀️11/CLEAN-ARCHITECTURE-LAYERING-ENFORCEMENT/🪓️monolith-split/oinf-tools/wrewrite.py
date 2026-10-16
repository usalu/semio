"""Phase 1: decompose World3dState into component structs and narrow every state-taking function."""
import sys, os, re, collections, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wengine import *
from wmap import COMPONENTS, ORDER as CORDER, F as FIELD2COMP

K = 5
TYNAME = {c: 'World' + c.capitalize() + 'State' for c in CORDER}
DESC = COMPONENTS

def pname_for(c, taken):
    n = f'{c}_state'
    if n in taken: n = f'{c}_comp'
    assert n not in taken, (c, n)
    return n

def split_top_commas(text, lo, hi):
    """positions of depth-0 commas in text[lo:hi] (aware of ([{< ), skipping comments/strings via lex."""
    toks = lex(text[lo:hi])
    out = []; d = 0
    for i, t in enumerate(toks):
        if t.k == 'p':
            if t.s in '([{<': d += 1
            elif t.s in ')]}': d -= 1
            elif t.s == '>' and not (i > 0 and toks[i-1].s == '-'): d -= 1
            elif t.s == ',' and d == 0: out.append(lo + t.a)
    return out

def parse_struct_fields(text, it):
    lb = text.index('{', it.start); rb = it.end - 1
    commas = split_top_commas(text, lb + 1, rb)
    bounds = [lb + 1] + [c + 1 for c in commas]
    ends = commas + [rb]
    fields = []
    for a, b in zip(bounds, ends):
        chunk = text[a:b]
        if not chunk.strip(): continue
        toks = [t for t in lex(chunk) if t.k != 'comment']
        # field name: first id before ':' skipping attrs and pub
        i = 0
        while i < len(toks) and toks[i].s == '#':
            d = 0; i += 1
            while True:
                if toks[i].s == '[': d += 1
                elif toks[i].s == ']':
                    d -= 1
                    if d == 0: break
                i += 1
            i += 1
        ispub = False
        if toks[i].s == 'pub':
            ispub = True; i += 1
        name = toks[i].s
        assert toks[i+1].s == ':', chunk
        name_pos = toks[i].a
        cfg_test = '#[cfg(test)]' in chunk[:name_pos].replace(' ', '')
        fields.append(dict(name=name, chunk=chunk, pub=ispub, name_pos=name_pos, cfg_test=cfg_test))
    return fields

def pubify(fd):
    c = fd['chunk']
    if fd['pub']: return c
    p = fd['name_pos']
    return c[:p] + 'pub ' + c[p:]

def parse_literal_entries(text, lb, rb):
    commas = split_top_commas(text, lb + 1, rb)
    bounds = [lb + 1] + [c + 1 for c in commas]; ends = commas + [rb]
    ents = []
    for a, b in zip(bounds, ends):
        chunk = text[a:b]
        if not chunk.strip(): continue
        toks = [t for t in lex(chunk) if t.k != 'comment']
        i = 0
        while i < len(toks) and toks[i].s == '#':
            d = 0; i += 1
            while True:
                if toks[i].s == '[': d += 1
                elif toks[i].s == ']':
                    d -= 1
                    if d == 0: break
                i += 1
            i += 1
        name = toks[i].s
        shorthand = not (i + 1 < len(toks) and toks[i+1].s == ':')
        ents.append(dict(name=name, chunk=chunk.strip('\n'), shorthand=shorthand))
    return ents

def build_state_defs(text, st_item, new_fn):
    fields = parse_struct_fields(text, st_item)
    byc = collections.defaultdict(list)
    for fd in fields: byc[FIELD2COMP[fd['name']]].append(fd)
    host = ['pub struct World3dState {']
    for c in CORDER:
        host.append(f'    /// {DESC[c][0].upper()+DESC[c][1:]}.')
        host.append(f'    pub {c}: {TYNAME[c]},')
    host.append('}')
    comps = []
    for c in CORDER:
        s = [f'/// 🧩️ World3d {c} component: {DESC[c]}.', f'pub struct {TYNAME[c]} {{']
        for fd in byc[c]:
            s.append(pubify(fd).rstrip().rstrip(','))
            s[-1] += ','
        s.append('}')
        comps.append('\n'.join(s))
    # literal entries from new()
    body_start = text.index('{', new_fn.start)
    lit = text.index('Self {', body_start)
    lb = lit + len('Self ')
    # matching close
    d = 0; k = lb
    toks = lex(text[lb:new_fn.end]); 
    rb = None
    for t in toks:
        if t.s == '{' and t.k == 'p': d += 1
        elif t.s == '}' and t.k == 'p':
            d -= 1
            if d == 0: rb = lb + t.a; break
    ents = parse_literal_entries(text, lb, rb)
    entbyc = collections.defaultdict(list)
    for e in ents: entbyc[FIELD2COMP[e['name']]].append(e)
    assert set(e['name'] for e in ents) == set(fd['name'] for fd in fields), set(fd['name'] for fd in fields) ^ set(e['name'] for e in ents)
    impls = []
    for c in CORDER:
        params = []
        if c == 'session': params = ['surface_id: String', 'controller_id: String']
        shorts = [e['name'] for e in entbyc[c] if e['shorthand']]
        assert set(shorts) <= {'surface_id', 'controller_id'}, shorts
        body = ',\n'.join('            ' + e['chunk'].strip() if not e['chunk'].lstrip().startswith('#') else '            ' + e['chunk'].strip().replace('\n', '\n            ') for e in entbyc[c])
        impls.append(f'impl {TYNAME[c]} {{\n    pub fn new({", ".join(params)}) -> Self {{\n        Self {{\n{body},\n        }}\n    }}\n}}')
    newfn = ['    pub fn new(surface_id: String, controller_id: String) -> Self {', '        Self {']
    for c in CORDER:
        args = 'surface_id, controller_id' if c == 'session' else ''
        newfn.append(f'            {c}: {TYNAME[c]}::new({args}),')
    newfn.append('        }'); newfn.append('    }')
    return '\n'.join(host), '\n\n'.join(comps), '\n\n'.join(impls), '\n'.join(newfn)


def assign_modes(allfns):
    must = set()
    for f in allfns:
        if f.has_param and f.parent not in (None, STATE_TY): must.add(id(f))
    changed = True
    byid = {id(f): f for f in allfns}
    while changed:
        changed = False
        for f in allfns:
            if id(f) in must:
                for c in f.calls:
                    for t in c['targets']:
                        if id(t) not in must: must.add(id(t)); changed = True
    for f in allfns:
        if not f.has_param: f.mode = None
        elif f.parent == STATE_TY: f.mode = 'host'
        elif id(f) in must or len(f.req) <= K: f.mode = 'narrow'
        else: f.mode = 'host'

def mode_of(f):
    return f.mode

def comp_list(f):
    return [c for c in CORDER if c in f.req]

def make_edits(allfns, files, assign=True):
    edits = collections.defaultdict(list)
    stats = collections.Counter()
    if assign: assign_modes(allfns)
    for f in allfns:
        f.names = {}
    for f in allfns:
        if f.mode != 'narrow': continue
        text, toks = files[f.file]
        a, b = body_range(toks, f, f.body_first - 1)
        allids = set(t.s for t in toks[a:b + 1] if t.k == 'id')
        for c in comp_list(f): f.names[c] = pname_for(c, allids)
    for f in allfns:
        if not f.has_param: continue
        text, toks = files[f.file]
        a, b = body_range(toks, f, f.body_first - 1)
        nm = f.pname
        if f.mode == 'narrow':
            ptxt = ', '.join(f"{f.names[c]}: &{(f.life + ' ') if f.life else ''}{'mut ' if f.mut else ''}{TYNAME[c]}" for c in comp_list(f))
            pa, pb = toks[f.pstart].a, toks[f.pend - 1].b
            if ptxt:
                edits[f.file].append((pa, pb, ptxt))
            else:
                nx = toks[f.pend]
                if nx.s == ',':
                    end = nx.b + (1 if text[nx.b:nx.b + 1] == ' ' else 0)
                    edits[f.file].append((pa, end, ''))
                else:
                    pv = toks[f.pstart - 1]
                    if pv.s == ',': edits[f.file].append((pv.a, pb, ''))
                    else: edits[f.file].append((pa, pb, ''))
            stats['narrow'] += 1
        else:
            stats['host'] += 1
        for i in range(a, b + 1):
            t = toks[i]
            if t.k != 'id' or t.s != nm: continue
            prev = toks[i - 1].s if i > 0 else ''
            nxt = toks[i + 1].s if i + 1 < len(toks) else ''
            if prev == '.' or (prev == ':' and toks[i - 2].s == ':'): continue
            if nxt == '.':
                fld = toks[i + 2].s
                if fld not in FIELD2COMP:
                    assert fld in STATE_METHODS, (f, fld)
                    continue
                c = FIELD2COMP[fld]
                if f.mode == 'narrow':
                    edits[f.file].append((t.a, t.b, f.names[c]))
                else:
                    edits[f.file].append((t.a, t.b, f'{nm}.{c}'))
                stats['field'] += 1
        for c in f.calls:
            if not c['targets']: continue
            t0 = c['targets'][0]
            if t0.mode == 'host':
                if f.mode == 'narrow': raise SystemExit(f'narrow {f} calls host-level {t0}')
                continue
            comps = comp_list(t0)
            s, i = c['a'], c['b']
            argA, argB = toks[s].a, toks[i].b
            refmut = ('mut' in c['ref']) or (c['ref'] == '' and t0.mut)
            parts = []
            for cc in comps:
                if f.mode == 'narrow':
                    if cc not in f.names: raise SystemExit(f'{f} lacks comp {cc} for callee {t0}')
                    parts.append(f.names[cc])
                else:
                    parts.append(f"&{'mut ' if refmut else ''}{nm}.{cc}")
            if parts:
                edits[f.file].append((argA, argB, ', '.join(parts)))
            else:
                nx = toks[i + 1]
                if nx.s == ',':
                    end = nx.b + (1 if text[nx.b:nx.b + 1] == ' ' else 0)
                    edits[f.file].append((argA, end, ''))
                else:
                    pv = toks[s - 1]
                    if pv.s == ',': edits[f.file].append((pv.a, argB, ''))
                    else: edits[f.file].append((argA, argB, ''))
            stats['call'] += 1
    return edits, stats

def apply_edits(text, edits):
    es = sorted(edits, key=lambda e: (e[0], e[1]))
    for x, y in zip(es, es[1:]):
        assert x[1] <= y[0], ('overlap', x, y)
    out = []; pos = 0
    for a, b, r in es:
        out.append(text[pos:a]); out.append(r); pos = b
    out.append(text[pos:])
    return ''.join(out)


def leftover_calls(allfns, files, narrow_only=True):
    narrow_free = {f.name for f in allfns if f.mode == 'narrow' and f.parent is None}
    narrow_meth = {f.name for f in allfns if f.mode == 'narrow' and f.parent is not None}
    handled = set()
    for f in allfns:
        for c in f.calls: handled.add((f.file, c['callee_tok']))
    other_free = {f.name for f in allfns if not f.has_param and f.parent is None}
    other_meth = {f.name for f in allfns if not f.has_param and f.parent is not None}
    narrow_free -= other_free; narrow_meth -= other_meth
    out = []
    for path, (text, toks) in files.items():
        for i, t in enumerate(toks):
            if t.k != 'id': continue
            prev = toks[i-1].s if i else ''
            nxt = toks[i+1].s if i + 1 < len(toks) else ''
            isfree = t.s in narrow_free and prev not in ('.', 'fn') and not (prev == ':' and toks[i-2].s == ':')
            ismeth = t.s in narrow_meth and (prev == '.' or (prev == ':' and toks[i-2].s == ':'))
            if (isfree or ismeth) and prev != 'fn' and nxt == '(':
                if (path, i) in handled: continue
                # skip definitions
                if prev == 'fn': continue
                out.append((path, text.count('\n', 0, t.a) + 1, t.s, nxt))
    return out

MANUAL_FIX = {'world3d_tool_run_trace_cursors'}

def fix_manual(text, fn_name):
    m = re.search(r'pub fn ' + fn_name + r'\b', text)
    i = m.start(); j = text.index('\n}\n', i) + 3
    seg = text[i:j]
    def rep(mm):
        fld = mm.group(2)
        return f'{mm.group(1)}.{FIELD2COMP[fld]}.{fld}'
    seg2 = re.sub(r'\b(state)\.([a-z_][a-z0-9_]*)\b', lambda mm: rep(mm) if mm.group(2) in FIELD2COMP else mm.group(0), seg)
    return text[:i] + seg2 + text[j:]


def make_edits_both(allfns, tfns, files):
    return make_edits(allfns + tfns, files, assign=False)
