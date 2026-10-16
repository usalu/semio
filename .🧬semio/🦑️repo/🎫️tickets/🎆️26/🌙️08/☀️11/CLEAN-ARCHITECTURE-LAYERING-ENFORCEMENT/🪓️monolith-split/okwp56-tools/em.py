# 🧮 Engine model for OS-KERNEL-WP5-6: parses the live `ArtifactStore` struct and impl blocks of the store root, derives field groups, per-method component footprints, call graph and homes (PLAN-OS-KERNEL-CRATES.md 7.3/7.4). Analysis only.
import os, re, sys, json, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import rs
from okrs import tokenize
R = '/Users/ueli/Documents/semio'
S = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store'
ROOT = S + '/🦀️.rs'

COMPONENTS = ['log', 'owners', 'wire', 'cursor', 'live', 'replay']
STRUCT = dict(log='StoreLog', owners='StoreOwners', wire='StoreWire', cursor='StoreCursor', live='StoreLive', replay='StoreReplay')
GROUPS = {
 'log': 'envelope envelope_detached dag applied_edit_ids redo_edit_ids edit_sequence current_checkpoint_id local_actor_id merge_policy clock authoring_verb initial_digest operation_wire'.split(),
 'owners': 'snapshot_retirement_factory initial_snapshot_retirement_factory mutation_retirement_factory snapshot_read_leases displaced_retirements owned_disposer owned_disposer_terminal factory_retirement_tickets one_item_preparation_factory replay_preparation_factory one_item_wire_preparation_factory'.split(),
 'wire': 'backbone member_inbox durable_group_root'.split(),
 'cursor': 'content_revision revision_accumulator revision_dirty_from mirror_dirt generation last_projection_cause'.split(),
 'live': 'current current_detached tail_undo_cache supersessions prefix_ring prefix_ring_dirty_from fold_frontier tail_step_generation snapshot_equality'.split(),
 'replay': 'replay_budget local_replay_budget pending_reprojection pending_report'.split(),
}
FIELD_COMP = {f: c for c, fs in GROUPS.items() for f in fs}

def read(p=ROOT): return open(p, encoding='utf8').read()

def struct_fields(text):
    its = rs.outline(text)
    st = next(i for i in its if i['kind'] == 'struct' and i['name'] == 'ArtifactStore')
    L = text.split('\n')
    body = L[st['h'] - 1: st['e']]
    fields = []; cur_docs = []; depth = 0
    # field lines are at 4-space indent: `name: Type,` (types can span lines but here single lines)
    S0 = rs.strip('\n'.join(body)).split('\n')
    i = 0
    while i < len(body):
        l = body[i]
        m = re.match(r'^    ([a-z_][a-z_0-9]*): (.*)$', S0[i])
        if m:
            name = m.group(1); ty = m.group(2); j = i
            while not S0[j].rstrip().endswith(','):
                j += 1; ty += ' ' + S0[j].strip()
            fields.append(dict(name=name, ty=ty.rstrip(',').strip(), line=st['h'] + i, docs=[x for x in body[max(0,i-8):i] if x.strip().startswith(('///','//'))]))
            i = j + 1; continue
        i += 1
    return st, fields

def impl_blocks(text):
    out = []
    for it in rs.outline(text):
        if it['kind'] == 'impl' and re.search(r'\bArtifactStore<P, Mutation>\s*(where|\{|$)', it['name']) and ' for ' not in it['name'].split('ArtifactStore<P, Mutation>')[0]:
            out.append(it)
    return out

if __name__ == '__main__':
    t = read()
    st, fields = struct_fields(t)
    print('struct', st['s'], st['e'], 'fields', len(fields))
    names = [f['name'] for f in fields]
    allg = [f for fs in GROUPS.values() for f in fs]
    print('missing from groups:', [n for n in names if n not in allg], 'extra in groups:', [n for n in allg if n not in names])
    for b in impl_blocks(t): print(b['s'], b['e'], b['e'] - b['s'] + 1, b['name'][:140])

def code_tokens(text):
    return [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]

def method_table(text):
    """All methods of the four inherent impls: name -> dict(block, s, h, e, vis, lines, body_text)"""
    L = text.split('\n'); table = collections.OrderedDict()
    for bi, b in enumerate(impl_blocks(text)):
        for m in rs.methods(text, b['h'], b['e']):
            if m['kind'] != 'fn': 
                table.setdefault('<' + m['kind'] + ' ' + m['name'] + '>', dict(m, block=bi, text='\n'.join(L[m['s'] - 1:m['e']])))
                continue
            assert m['name'] not in table, ('duplicate method', m['name'])
            table[m['name']] = dict(m, block=bi, text='\n'.join(L[m['s'] - 1:m['e']]))
    return table

def uses(body, names_methods):
    """(fields, self-method calls, Self::-calls, other receivers touching fields) of a method text"""
    toks = code_tokens(body); n = len(toks)
    fields = collections.Counter(); calls = collections.Counter(); selfcalls = collections.Counter(); others = collections.Counter()
    for i in range(n - 2):
        a, b, c = toks[i], toks[i + 1], toks[i + 2]
        if a[0] == 'id' and a[1] == 'self' and b[1] == '.' and c[0] == 'id':
            nxt = toks[i + 3][1] if i + 3 < n else ''
            if c[1] in FIELD_COMP and nxt != '(' and not (nxt == ':' and toks[i + 4][1] == ':'):
                fields[c[1]] += 1
            elif nxt == '(' or (nxt == ':' and i + 4 < n and toks[i + 4][1] == ':'):
                calls[c[1]] += 1
            else:
                others[c[1]] += 1
        if a[0] == 'id' and a[1] == 'Self' and b[1] == ':' and c[1] == ':' and i + 3 < n and toks[i + 3][0] == 'id':
            selfcalls[toks[i + 3][1]] += 1
    return fields, calls, selfcalls, others

if __name__ == '__main__':
    tab = method_table(t)
    print('methods', len([k for k in tab if not k.startswith('<')]), 'non-fn', [k for k in tab if k.startswith('<')])
    names = set(tab)
    ff = {}
    for k, m in tab.items():
        if k.startswith('<'): continue
        f, c, sc, o = uses(m['text'], names)
        ff[k] = (f, c, sc, o)
    unk = collections.Counter()
    for k, (f, c, sc, o) in ff.items():
        for x in c:
            if x not in names: unk[x] += 1
        for x in o: unk['.' + x] += 1
    print('self.<unknown> calls/fields:', unk.most_common(30))
    selfc = collections.Counter()
    for k, (f, c, sc, o) in ff.items():
        for x in sc: selfc[x] += 1
    print('Self::x:', selfc.most_common(40))

RANK = {c: i + 1 for i, c in enumerate(COMPONENTS)}; RANK['ops'] = 7
PINNED = {'new', 'construct', 'seed_runtime_state', 'adopt_runtime_seed', 'preflight_runtime_seed', 'from_initialized_runtime_with_owners'}
K_THRESH = 5

def homes(text, tab=None, k=K_THRESH):
    tab = tab or method_table(text)
    fns = {n: m for n, m in tab.items() if not n.startswith('<')}
    info = {}
    for n, m in fns.items():
        f, c, sc, o = uses(m['text'], set(fns))
        comps = {FIELD_COMP[x] for x in f}
        callees = {x for x in list(c) + list(sc) if x in fns and x != n}
        info[n] = dict(comps=comps, callees=callees, fields=f)
    home = {}
    for n, i in info.items():
        if n in PINNED or len(i['comps']) >= k: home[n] = 'ops'
        elif i['comps']: home[n] = max(i['comps'], key=lambda c: RANK[c])
        else: home[n] = None
    changed = True
    while changed:
        changed = False
        for n, i in info.items():
            if home[n] == 'ops': continue
            for cal in i['callees']:
                hc = home.get(cal)
                if hc is None: continue
                if home[n] is None or RANK[hc] > RANK[home[n]]:
                    home[n] = hc; changed = True
    # helpers: lowest-ranked home among callers; default log
    callers = collections.defaultdict(set)
    for n, i in info.items():
        for cal in i['callees']: callers[cal].add(n)
    for _ in range(5):
        for n in info:
            if home[n] is None:
                hs = [home[c] for c in callers[n] if home[c] is not None]
                if hs: home[n] = min(hs, key=lambda c: RANK[c])
    for n in info:
        if home[n] is None: home[n] = 'log'
    return info, home

if __name__ == '__main__':
    tab = method_table(t)
    info, home = homes(t, tab)
    agg = collections.defaultdict(lambda: [0, 0])
    for n, h in home.items():
        m = tab[n]; agg[h][0] += 1; agg[h][1] += m['e'] - m['s'] + 1
    for c in COMPONENTS + ['ops']: print(c, agg[c])
    json.dump({n: h for n, h in home.items()}, open(HERE + '/../🗑️generated/okwp56-home.json', 'w'))
