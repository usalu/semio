# 🔗️ Lower-crate edges and external identifiers of every crate of the plugin partition, from the token graph of plan_tools.py plus the method homing of section 7.
# usage: CLOSURE=../🗑️generated/pwp0-closure.json python3 unitdeps.py <out.json>
import collections, json, os, re, sys
import plan_tools as pt

COMP_CRATE = {'core': 'instance-app', 'constructor': 'instance-stores', 'stores': 'instance-stores', 'roster': 'instance-stores', 'ingress': 'instance-stores', 'interaction': 'instance-stores',
              'history': 'instance-history', 'hostevent': 'instance-history', 'txn': 'instance-history', 'children': 'instance-children', 'pending': 'instance-jobs', 'tools': 'instance-jobs', 'toolmachine': 'instance-jobs',
              'media': 'instance-docs', 'ingest': 'instance-docs', 'timetravel': 'time-travel-turns', 'toolrun': 'tool-run-turns', 'maintenance': 'instance-ladders', 'close': 'instance-ladders'}
FILE_USECASE = {'🧵️instance/🚪️ingress/🦀️.rs': 'instance-commands', '🧵️instance/🧾️typed-operations/🦀️.rs': 'instance-typed-ops', '🧵️instance/📨️dispatch/🦀️.rs': 'instance-commands',
                '🧵️instance/🔌️plugin-app-impl/🦀️.rs': 'instance-app', '🧵️instance/🖼️render/🦀️.rs': 'instance-render', '🧵️instance/🧷️command-routes/🦀️.rs': 'instance-commands',
                '🧵️instance/🛠️tool-machine/🦀️.rs': 'instance-typed-ops', '🧵️instance/🤝️group/🦀️.rs': 'instance-commands', '🧵️instance/🔄️full-operation/🦀️.rs': 'instance-ladders',
                '🧵️instance/⏳️operation-progress/🦀️.rs': 'instance-jobs', '🧵️instance/🦀️.rs': 'instance-app', '🎮️command/🎟️lease/🦀️.rs': 'instance-docs', '🎮️command/📥️ingress/🦀️.rs': 'instance-stores',
                '🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs': 'instance-ladders', '♻️lifecycle/🏪️stores/🦀️.rs': 'instance-ladders', '🪆️child/👁️read/🧪️test-view/🦀️.rs': 'instance-stores'}
def file_instance_crate(f):
    if f in FILE_USECASE: return FILE_USECASE[f]
    if f.startswith('🪜️'): return 'instance-ladders'
    if f.startswith('⏪️time-travel/🧵️instance/'): return 'time-travel-turns'
    if f.startswith('⏯️tool-run/🧵️instance/'): return 'tool-run-turns'
    return None

EXTERN_ALIAS = {'dsl': 'semio_framework_os_kernel', 'protocol': 'semio_framework_os_kernel', 'store': 'semio_framework_os_kernel', 'vcs': 'semio_framework_os_kernel'}

def main():
    files, items, defs, owner = pt.build(); size, adj, crate_of = pt.graph(items, defs, owner)
    final, moves, _ = pt.simulate(size, adj, crate_of, items)
    unit_crate = dict(final)
    for u in size:
        if u not in unit_crate: unit_crate[u] = crate_of[u]
    tokcrate = collections.defaultdict(set)
    for name, ids in defs.items():
        if len(ids) > 3 or name in pt.GENERIC: continue
        for j in ids:
            u = owner.get(j, j)
            c = unit_crate.get(u)
            if c and not c.startswith('TOP'): tokcrate[name].add(c)
    lower_ext = collections.defaultdict(set); lower_ext_names = collections.defaultdict(set)
    lower_tok = collections.defaultdict(set)
    inst_tok = collections.defaultdict(set)
    for it in items:
        if it['cfgtest'] or '/🧪️tests/' in it['file'] or it['file'].startswith('🧪️'): continue
        u = owner.get(it['id'], it['id'])
        c = unit_crate.get(u)
        if c is None: continue
        if c.startswith('TOP'):
            ic = file_instance_crate(it['file'])
            if it['kind'] == 'impl' and it.get('target') == 'VcsArtifactApp': continue
            if ic: inst_tok[ic] |= it['toks']
            elif c == 'TOP-builder': inst_tok['builder'] |= it['toks']
            elif c == 'TOP-root': inst_tok['facade'] |= it['toks']
            continue
        lower_tok[c] |= it['toks']
    _files, _items, _d, _o = files, items, defs, owner
    pt._TEXT = {}
    for p in files:
        L, R = pt.outline(pt.ROOT + p)[1:]; pt._TEXT[p.replace(pt.P, '')] = (L, R)
    methods = home_methods(items, defs, owner)
    for m in methods:
        ic = COMP_CRATE.get(m['home']) if m['home'] in COMP_CRATE else ('instance-app' if m['home'] == 'pluginapp' else file_instance_crate(m['file']) if m['home'] == 'usecase' else 'instance-history')
        if m['home'] in ('timetravel',): ic = 'time-travel-turns'
        inst_tok[ic] |= set(m['toks'])
    for c, comps in field_tokens(items).items():
        inst_tok[COMP_CRATE[c]] |= comps
    out = dict(lower={}, instance={})
    for c, toks in lower_tok.items():
        out['lower'][c] = dict(deps=sorted({x for t in toks for x in tokcrate.get(t, ()) if x != c}, key=pt.ORDER.index), toks=sorted(t for t in toks if t.split('_')[0] in ('dsl', 'protocol', 'store', 'vcs') or t.startswith('semio') or t in ('serde', 'serde_json', 'wit_bindgen', 'replication', 'base64_codec', 'ToValue', 'FromValue', 'DslRecord', 'dyn_enum', 'dyn_enum_close', 'async_test', 'wasmtime', 'Serialize', 'Deserialize')))
    for c, toks in inst_tok.items():
        out['instance'][c] = dict(deps=sorted({x for t in toks for x in tokcrate.get(t, ())}, key=lambda x: pt.ORDER.index(x)), toks=sorted(t for t in toks if t.split('_')[0] in ('dsl', 'protocol', 'store', 'vcs') or t.startswith('semio') or t in ('serde', 'serde_json', 'wit_bindgen', 'replication', 'base64_codec', 'ToValue', 'FromValue', 'DslRecord', 'dyn_enum', 'dyn_enum_close', 'async_test', 'wasmtime', 'Serialize', 'Deserialize')))
    json.dump(out, open(sys.argv[1], 'w'), ensure_ascii=False, indent=1)
    for c, v in sorted(out['instance'].items()): print(c, len(v['deps']), v['deps'])

def field_tokens(items):
    va = [it for it in items if it['defname'] == 'VcsArtifactApp'][0]
    body = '\n'.join(pt._TEXT[va['file']][0][va['s'] - 1:va['e']]); inner = body[body.index('{') + 1:body.rindex('}')]
    cur = ''; d = 0; parts = []; prev = ''
    for ch in inner:
        if ch in '<([{': d += 1
        elif ch in '>)]}' and not (ch == '>' and prev in '-='): d -= 1
        prev = ch
        if ch == ',' and d == 0: parts.append(cur); cur = ''
        else: cur += ch
    res = collections.defaultdict(set)
    for p in parts + [cur]:
        m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:(.*)', re.sub(r'#\[[^\]]*\]', '', p).strip(), re.S)
        if not m: continue
        comp = next((c for pat, c in pt.RULES if re.search(pat, m.group(1))), None)
        if comp: res[comp] |= pt.tokens(m.group(2))
    return res

def home_methods(items, defs, owner):
    va = [it for it in items if it['defname'] == 'VcsArtifactApp'][0]
    fields = [f for comps in [field_tokens(items)] for f in []]
    body = '\n'.join(pt._TEXT[va['file']][0][va['s'] - 1:va['e']]); inner = body[body.index('{') + 1:body.rindex('}')]
    cur = ''; d = 0; parts = []; prev = ''
    for ch in inner:
        if ch in '<([{': d += 1
        elif ch in '>)]}' and not (ch == '>' and prev in '-='): d -= 1
        prev = ch
        if ch == ',' and d == 0: parts.append(cur); cur = ''
        else: cur += ch
    flds = []
    for p in parts + [cur]:
        m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:', re.sub(r'#\[[^\]]*\]', '', p).strip())
        if m: flds.append(m.group(1))
    comp = {f: next((c for pat, c in pt.RULES if re.search(pat, f)), 'UNASSIGNED') for f in flds}
    methods = []
    for it in items:
        if it['kind'] == 'impl' and it.get('target') == 'VcsArtifactApp':
            seg = pt.text_of(items, it); depth = 0; curm = None
            for k, l in enumerate(seg):
                before = depth
                m = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)', l)
                if before == 1 and m:
                    if curm: curm['end'] = k
                    curm = dict(name=m.group(1), start=k, file=it['file'], trait=it.get('trait')); methods.append(curm)
                depth = before + l.count('{') - l.count('}')
                if k == 0 and '{' in l: depth = max(depth, 1)
            if curm: curm['end'] = len(seg)
            for m in methods:
                if 'text' not in m and m['file'] == it['file']: m['text'] = '\n'.join(seg[m['start']:m.get('end', len(seg))])
    fn = set(flds); by = collections.defaultdict(list)
    for m in methods: by[m['name']].append(m)
    uniq = {n: v[0] for n, v in by.items() if len(v) == 1}; rk = {c: i for i, c in enumerate(pt.COMP_ORDER)}; USE = len(pt.COMP_ORDER)
    for m in methods:
        m['fields'] = sorted(set(re.findall(r'\bself\.(\w+)', m['text'])) & fn)
        m['calls'] = sorted(set(re.findall(r'\b(?:self|Self)(?:\.|::)(\w+)\s*(?:\(|::<)', m['text'])))
        cs = {comp[f] for f in m['fields']}; base = max([rk[c] for c in cs], default=-1)
        if len(cs - {'core'}) >= 4: base = USE
        if m.get('trait') == 'PluginApp': base = USE + 1
        m['rank'] = base
    ch = True
    while ch:
        ch = False
        for m in methods:
            for cn in m['calls']:
                t = uniq.get(cn)
                if t and t is not m and t['rank'] > m['rank']: m['rank'] = t['rank']; ch = True
    for m in methods:
        r = m['rank']; m['home'] = pt.COMP_ORDER[r] if 0 <= r < USE else ('free' if r < 0 else 'usecase' if r == USE else 'pluginapp')
        m['toks'] = sorted(pt.tokens(m['text']))
    return methods

if __name__ == '__main__': main()
