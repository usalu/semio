import sys; sys.path.insert(0, '.')
from rw6 import *
from model6 import parse_file
import touched6

TIER1 = {'instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-docs'}
DEFAULT_COMP = {'instance-stores': 'stores', 'instance-history': 'history', 'instance-children': 'child_composition', 'instance-jobs': 'tools', 'instance-docs': 'media'}
CORE = {'app', 'mounted_policy'}

def family_rels():
    t = touched6.load()
    out = []
    for f in sorted(set(t['created'] + t['modified']) - set(t['deleted'])):
        if not f.endswith('.rs'): continue
        if '📥️staging' in f or f in ('🧵️instance/🦀️.rs', '🧵️instance/🧷️command-routes/🦀️.rs', '🧵️instance/🧾️typed-operations/🦀️.rs', '🧵️instance/🖼️render/🦀️.rs', '🧵️instance/🔌️plugin-app-impl/🦀️.rs', '🪜️maintenance-ladder/🦀️.rs', '⏪️time-travel/🧵️instance/🦀️.rs', '⏯️tool-run/🧵️instance/🦀️.rs'):
            out.append(f)
    return out

def build_world():
    recs = json.load(open(GEN + '/pwp6-s0-recs.json', encoding='utf-8'))
    byk = {}
    for r in recs: byk[(r['name'], r['trait'])] = r
    model = {rel: parse_file(rel) for rel in family_rels()}
    ft = field_table()
    methods = {}; table = {}
    # sig parse for receiver kinds
    for rel, f in model.items():
        for pi, p in enumerate(f['pieces']):
            if p.kind != 'vcsimpl': continue
            for mi, m in enumerate(p.members):
                key = (m.name, p.trait)
                r = byk[key]
                txt = '\n'.join(f['R'][m.a:m.e])
                sg = parse_sig(txt)
                recv = sg['recv']['kind'] if sg['recv'] else None
                conv = (r['crate'] in TIER1 and p.trait is None and r['name'] not in ('new', 'with_registry', 'with_registry_on_bus'))
                if conv:
                    h = r['home2']
                    cf = PLAN2FIELD.get(h) or DEFAULT_COMP[r['crate']]
                    # the S0 crate decides the component's crate: it must be the component's own crate
                    params = []
                    for c in r['need2']:
                        if c == h or c == 'free': continue
                        if c == 'core': continue
                        params.append(PLAN2FIELD[c])
                    # core fields used
                    core = []
                    body = mask(txt)
                    if re.search(r'\bself\.app\b', body): core.append('app')
                    if re.search(r'\bself\.mounted_policy\b', body): core.append('mounted_policy')
                    info = MInfo(m.name, cf, tuple(core) + tuple(sorted(params, key=lambda x: list(COMPONENTS).index(x))), recv, sg['is_async'], None, m.vis, rel)
                else:
                    info = MInfo(m.name, None, (), recv, sg['is_async'], p.trait, m.vis, rel)
                table[(rel, pi, mi)] = (info, r)
                prev = methods.get(m.name)
                if prev is None or (prev.trait is not None and info.trait is None): methods[m.name] = info
    # transitive core parameters: a converted caller needs every core field a converted callee needs
    by_name = {i.name: i for i, r in table.values() if i.cf}
    callmap = {}
    for (rel, pi, mi), (info, r) in table.items():
        if info.cf: callmap[info.name] = [c for c in r['calls'] if c in by_name]
    ch = True
    while ch:
        ch = False
        for name, cs in callmap.items():
            info = by_name[name]
            core = {p for p in info.params if p in CORE}
            for c in cs:
                core |= {p for p in by_name[c].params if p in CORE}
            comps = [p for p in info.params if p not in CORE]
            new = tuple(sorted(core, key=lambda x: ['app', 'mounted_policy'].index(x))) + tuple(comps)
            if new != info.params: info.params = new; ch = True
    w_known = {i.name for i, r in table.values() if not i.cf}
    return model, methods, table, w_known

if __name__ == '__main__':
    model, methods, table, known = build_world()
    conv = [i for i, r in table.values() if i.cf]
    print(len(table), 'members;', len(conv), 'converted')
    c = collections.Counter(i.cf for i in conv); print(c)
    print(collections.Counter(len(i.params) for i in conv))
