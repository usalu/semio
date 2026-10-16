"""🔍️ Static scans of the WP3 staging on the CURRENT tree: path-literal resolution (and 240-byte limit), region balance, orphan/inherent ownership of every impl (file-based crate assignment, lift files count for their destination crate), and the definition scan (private names of a lifted child used outside its module subtree). usage: CLOSURE=<closure.json> python3 scans3.py"""
import collections, os, re, sys, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import graph3 as g, mv
pt = g.pt
ROOT = '/Users/ueli/Documents/semio/'
PL = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
rd = lambda p: open(p, encoding='utf8').read()
WP5SET = {'time-travel', 'time-travel-panel', 'tool-run', 'command', 'job'}

def files_of(crates):
    out = set()
    for rows in ('lib native', 'lib wasm32-wasip2', 'lib native testing-features', 'test native', 'test wasm32-wasip2', 'test native all-features'):
        for x in pt.CLOSURE[rows]:
            p = x['path']
            if p.startswith(PL) and p.endswith('.rs') and pt.file_crate(p[len(PL):]) in crates: out.add(p)
    return sorted(out)

LIT = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]|\binclude(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)')
def inline_mods(text, pos):
    stack = []
    for m in re.finditer(r'(?m)^[ \t]*(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*\{|[{}]', text[:pos]):
        s = m.group(0)
        if s == '{': stack.append(None)
        elif s == '}':
            if stack: stack.pop()
        else: stack.append(m.group(1))
    return [x for x in stack if x]

def pathcheck(files):
    n = 0; bad = []
    for f in files:
        t = rd(ROOT + f)
        for m in LIT.finditer(t):
            rel = m.group(1) or m.group(2); n += 1
            base = os.path.dirname(ROOT + f)
            mods = inline_mods(t, m.start()) if m.group(1) else []
            cand = os.path.normpath(os.path.join(base, *mods, rel))
            if not os.path.exists(cand): bad.append((f[len(PL):], rel))
            if len(os.path.relpath(cand, ROOT).encode()) > 240: bad.append(('LONG', os.path.relpath(cand, ROOT)))
    return n, bad

def regions(files):
    bad = []
    for f in files:
        c = collections.Counter()
        for m in re.finditer(r'//#(end)?region (.*)', rd(ROOT + f)): c[m.group(2).strip()] += -1 if m.group(1) else 1
        for k, v in c.items():
            if v: bad.append((f[len(PL):], k, v))
    return bad

IDENT = re.compile(r'[A-Za-z_]\w*')
PIECES = ('/🛬️lifted/', '/🪟️panel/🔢️pointer/', '/🪟️panel/🖼️sections/')
def orphan():
    items, size, adj, crate_of = g.run()
    # every item (incl. cfg(test)) counts for the impl check, assigned by file
    tdef = collections.defaultdict(set)
    allitems = pt.build()[1]
    for it in allitems:
        if it['kind'] in ('struct', 'enum', 'union', 'trait', 'type') and it['defname'] and not it['cfgtest'] and '/🧪️tests/' not in it['file']:
            tdef[it['defname']].add(pt.item_crate(it['file'], it['s']))
    bad = []
    for it in allitems:
        if it['kind'] != 'impl' or it['cfgtest'] or '/🧪️tests/' in it['file']: continue
        c = pt.item_crate(it['file'], it['s'])
        if c.startswith(('TOP', 'UNMAPPED')) or c not in g.RANK: continue
        S, T = it.get('target'), it.get('trait')
        head = it['head']
        hdr = head.split('{')[0]
        args = {x for x in IDENT.findall(hdr) if x in tdef and x not in (S, T)}
        sd = tdef.get(S, set()); td = tdef.get(T, set()) if T else set()
        if not T:
            if sd and c not in sd and len(sd) == 1: bad.append(('inherent-foreign', c, S, sorted(sd), it['file'][-50:], it['s']))
            continue
        local = (c in sd) or (c in td) or any(c in tdef[a] for a in args)
        if not local and (sd or td or args):
            bad.append(('orphan', c, f'{T} for {S}', sorted(sd | td), it['file'][-50:], it['s']))
        for nm, dset in (('self', sd), ('trait', td)):
            if dset and all(d in g.RANK and g.RANK[d] > g.RANK[c] for d in dset): bad.append(('names-higher-crate', c, f'{T} for {S}', sorted(dset), it['file'][-50:], it['s']))
    return bad

def mounted_children(path, text):
    out = set()
    for m in LIT.finditer(text):
        rel = m.group(1) or m.group(2)
        if rel and rel.endswith('.rs'):
            mods = inline_mods(text, m.start()) if m.group(1) else []
            out.add(os.path.normpath(os.path.join(os.path.dirname(path), *mods, rel)))
    return out

def subtree(path, texts, seen=None):
    seen = seen if seen is not None else set()
    if path in seen or path not in texts: return seen
    seen.add(path)
    for c in mounted_children(path, texts[path]): subtree(c, texts, seen)
    return seen

def defscan(files, override=None):
    """private names (items, fields, inherent methods) of every lift file that its SOURCE module (the parent that mounts it) or a sibling child of that parent still names: before the lift they were reachable through module privacy, after it only through a widening. A hit is a name the widening missed; a name the user defines itself is skipped. Whole-module lifts leave no code behind and are skipped."""
    texts = {ROOT + f: rd(ROOT + f) for f in files}
    if override: texts.update(override)
    idset = {p: set(IDENT.findall(re.sub(r'//[^\n]*', '', t))) for p, t in texts.items()}
    dots = {p: set(re.findall(r'\.\s*([A-Za-z_]\w*)', t)) | set(re.findall(r'\b([A-Za-z_]\w*)\s*:(?!:)', t)) for p, t in texts.items()}
    defined = {p: {k for k in mv.names_private(t)} | set(re.findall(r'\b(?:fn|struct|enum|const|static|type|trait|union|mod)\s+([A-Za-z_]\w*)', t)) for p, t in texts.items()}
    parent_of = {}
    for p, t in texts.items():
        for c in mounted_children(p, t):
            if c in texts and any(x in c for x in PIECES) and c != p: parent_of.setdefault(c, set()).add(p)
    hits = []; scanned = 0
    for lift, parents in parent_of.items():
        body = texts[lift]
        if len([l for l in body.split('\n') if l.strip() and not l.startswith('//!')]) == 0: continue
        for src in parents:
            if len([l for l in texts[src].split('\n') if l.strip() and not l.startswith(('//', '#[', 'mod ', 'pub use ', 'use ')) ]) == 0: continue
            scope = subtree(src, texts) - subtree(lift, texts)
            priv = mv.names_private(body); scanned += len(priv)
            for n, k in priv.items():
                if len(n) < 4: continue
                for q in scope:
                    used = n in idset[q] if k == 'item' else n in dots[q]
                    if used and (q == src or n not in defined[q]) and not (q == src and n in defined[q] and k == 'item' and False): hits.append((os.path.relpath(lift, ROOT + PL)[-60:], os.path.relpath(q, ROOT + PL)[-50:], n, k))
    return hits, scanned

if __name__ == '__main__':
    files = files_of(WP5SET)
    n, bad = pathcheck(files); print('path literals', n, 'in', len(files), 'files; unresolved/long:', bad)
    print('regions unbalanced:', regions(files))
    o = orphan(); print('orphan/inherent/visibility findings:', len(o))
    for x in o: print('  ', x)
    d, scanned = defscan(files); print('definition scan: private names of the lifts and pieces checked:', scanned, '; names the source module or its sibling children still use:', len(d))
    for x in d[:60]: print('  ', x)
    if os.environ.get('SEED') == '1':
        _all = pt.build()[1]
        _pin = {(it['file'], it['s']) for it in _all if it['defname'] == 'HistoryPanelText' and it['kind'] == 'enum'}
        assert _pin, 'seed anchor'
        _orig = pt.item_crate
        pt.item_crate = lambda f, s: 'time-travel' if (f, s) in _pin else _orig(f, s)
        seeded_o = [x for x in orphan() if 'HistoryPanelText' in str(x)]
        pt.item_crate = _orig
        print('seeded orphan (the HistoryPanelText type mapped to time-travel, its impl stays in command): findings', len(seeded_o), [x[:4] for x in seeded_o][:3])
        target = [f for f in files if f.endswith('🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs')][0]
        txt = rd(ROOT + target)
        assert 'pub(crate) fn time_travel_admits_withdrawal' in txt, 'seed anchor'
        seeded, _ = defscan(files, {ROOT + target: txt.replace('pub(crate) fn time_travel_admits_withdrawal', 'fn time_travel_admits_withdrawal')})
        print('seeded (time_travel_admits_withdrawal made private again): hits', len(seeded), [h for h in seeded if h[2] == 'time_travel_admits_withdrawal'][:3])
        target2 = [f for f in files if f.endswith('🪟️panel/🔢️pointer/🦀️.rs')][0]
        txt2 = rd(ROOT + target2)
        assert 'pub(crate) fn time_travel_schema_documents' in txt2 or 'fn time_travel_schema_documents' in txt2
        seeded2, _ = defscan(files, {ROOT + target2: re.sub(r'pub\(crate\) fn time_travel_schema_documents', 'fn time_travel_schema_documents', txt2)})
        print('seeded (time_travel_schema_documents made private again): hits', len(seeded2), [h for h in seeded2 if h[2] == 'time_travel_schema_documents'][:3])
