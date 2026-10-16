import sys; sys.path.insert(0, '.')
import assign6 as AS
from assign6 import *
from fields6 import parse_fields, comp_of

CRATE_ROOT = {  # crate key -> root dir (relative to PLUG)
 'instance-stores': '🧵️instance/🏪️stores', 'instance-history': '🧵️instance/📜️history', 'instance-children': '🧵️instance/🪆️children', 'instance-jobs': '🧵️instance/👷️jobs', 'instance-docs': '🧵️instance/📄️docs',
 'instance-commands': '🧵️instance/🧷️command-routes', 'instance-typed-ops': '🧵️instance/🧾️typed-operations', 'instance-render': '🧵️instance/🖼️render', 'instance-ladders': '🪜️maintenance-ladder',
 'time-travel-turns': '⏪️time-travel/🧵️instance', 'tool-run-turns': '⏯️tool-run/🧵️instance', 'instance-app': '🧵️instance'}
ROOT_FILE = {  # class-B roots: the crate whose root file is this source file
 '🧵️instance/🧷️command-routes/🦀️.rs': 'instance-commands', '🧵️instance/🧾️typed-operations/🦀️.rs': 'instance-typed-ops', '🧵️instance/🖼️render/🦀️.rs': 'instance-render',
 '🪜️maintenance-ladder/🦀️.rs': 'instance-ladders', '⏪️time-travel/🧵️instance/🦀️.rs': 'time-travel-turns', '⏯️tool-run/🧵️instance/🦀️.rs': 'tool-run-turns', '🧵️instance/🦀️.rs': 'instance-app'}
IN_PLACE_EXTRA = {'🧵️instance/🔌️plugin-app-impl/🦀️.rs': 'instance-app'}   # composer's own file inside the composer root dir
SHORT = {'instance-stores': 'stores', 'instance-history': 'history', 'instance-children': 'children', 'instance-jobs': 'jobs', 'instance-docs': 'docs', 'instance-commands': 'commands', 'instance-typed-ops': 'typed_ops', 'instance-render': 'render', 'instance-ladders': 'ladders', 'time-travel-turns': 'tt_turns', 'tool-run-turns': 'tr_turns', 'instance-app': 'app'}

PARENT_TEXT = {'🧵️instance/🦀️.rs', '🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs', '🎮️command/🎟️lease/🦀️.rs', '♻️lifecycle/🏪️stores/🦀️.rs'}
WHOLE_FILE = {'🎮️command/📥️ingress/🦀️.rs': 'instance-stores'}
TURN_ONLY = {'⏪️time-travel/🧵️instance/🦀️.rs', '⏯️tool-run/🧵️instance/🦀️.rs'}
def refre(name, kind):
    if kind in ('fn',) or name[0].islower(): return re.compile(r'(?<![\w])(?<![^.]\.)' + re.escape(name) + r'\s*(?:\(|::<|\)|,|;|\s*\.|$)')
    return re.compile(r'(?<![\w])(?<![^.]\.)' + re.escape(name) + r'\b')

def plan():
    recs, uniq, callee, model, callers = AS.assign()
    fields = parse_fields(); fcomp = {f['name']: comp_of(f['name']) for f in fields}
    # member crate lookup
    mcrate = {}
    for m in recs: mcrate[(m['file'], m['piece'], m['idx'])] = m['crate']
    # free items
    items = []
    for rel, f in model.items():
        for pi, p in enumerate(f['pieces']):
            if p.kind == 'item' and p.itkind != 'mod':
                nm = re.match(r'[A-Za-z_]\w*', p.name or '')
                body = '\n'.join(f['L'][p.s:p.e])
                vis = ''
                for l in f['R'][p.s:p.e]:
                    s = l.strip()
                    if s.startswith(('///', '#[', '//')): continue
                    mv = re.match(r'(pub(?:\([^)]*\))?)\s', s); vis = mv.group(1) if mv else ''; break
                items.append(dict(file=rel, piece=pi, kind=p.itkind, name=(nm.group(0) if nm else None), head=p.head, target=getattr(p, 'target', None), trait=getattr(p, 'trait', None), body=body, vis=vis, lines=p.e - p.s, crate=None, pin=None))
    byname = {}
    for it in items:
        if it['kind'] in ('impl',): continue
        if it['name'] in byname: print('NAME CLASH', it['name'], byname[it['name']]['file'], it['file'], file=sys.stderr)
        byname.setdefault(it['name'], it)
    # impls follow their type/trait
    def impl_owner(it):
        for key in (it['target'], it['trait']):
            if key and key in byname and it['file'] == byname[key]['file'] or (key and key in byname and byname[key]['vis'] != ''): return byname[key]
        for key in (it['target'], it['trait']):
            if key and key in byname: return byname[key]
        return None
    # users
    def uses(text, name, kind): return bool(refre(name, kind).search(text))
    fldtypes = {}
    for fd in fields:
        fldtypes[fd['name']] = fd['decl']
    users = collections.defaultdict(set)
    for it in items:
        if it['kind'] == 'impl' or not it['name']: continue
        scope_files = [it['file']] if it['vis'] == '' else list(model)
        if it['vis'] == '' and it['file'] in PARENT_TEXT: scope_files = [x for x in model if x not in TURN_ONLY]
        for m in recs:
            if m['file'] not in scope_files: continue
            if uses(m['body'], it['name'], it['kind']): users[id(it)].add(m['crate'])
        for fn_, decl in fldtypes.items():
            if re.search(r'\b' + re.escape(it['name']) + r'\b', decl): users[id(it)].add(FIELD_CRATE[fcomp[fn_]])
    # item -> item references (an impl block contributes its text to the item it implements)
    for it in items: it['ebody'] = it['body']
    for it in items:
        if it['kind'] == 'impl':
            o = impl_owner(it)
            if o is not None: o['ebody'] += '\n' + it['body']
    edges = collections.defaultdict(set)
    for a in items:
        if a['kind'] == 'impl': continue
        for b in items:
            if a is b or b['kind'] == 'impl' or not b['name']: continue
            if b['vis'] == '' and a['file'] != b['file']: continue
            if uses(a['ebody'], b['name'], b['kind']): edges[id(a)].add(id(b))
    byid = {id(it): it for it in items}
    # force: items naming VcsArtifactApp
    for it in items:
        if it['name'] == 'VcsArtifactApp': it['pin'] = 'instance-app'
    # default crates: own file's primary
    def default_crate(it):
        f = it['file']
        if f in ROOT_FILE: return ROOT_FILE[f]
        if f in IN_PLACE_EXTRA: return IN_PLACE_EXTRA[f]
        return FILE_USECASE[f]
    def pick(cs):
        cs = set(cs)
        if len(cs) > 1: cs.discard('instance-app')
        t1 = [c for c in cs if RANKC[c] <= 5]
        if t1: return min(t1, key=RANKC.get)
        if len(cs) == 1: return next(iter(cs))
        return 'instance-docs'
    for it in items:
        if it['kind'] == 'impl': continue
        it['crate'] = it['pin'] or (pick(users[id(it)]) if users[id(it)] else default_crate(it))
    # propagate downward: referenced items must live no higher than any referrer
    for _ in range(10):
        ch = False
        for a in items:
            if a['kind'] == 'impl' or not a['name']: continue
            for bid in edges[id(a)]:
                b = byid[bid]
                if b['pin']: continue
                if RANKC[b['crate']] > RANKC[a['crate']]:
                    b['crate'] = a['crate']; ch = True
                elif RANKC[b['crate']] == RANKC[a['crate']] and b['crate'] != a['crate']:
                    b['crate'] = 'instance-docs'; ch = True
        if not ch: break
    for it in items:
        if it['kind'] == 'impl':
            o = impl_owner(it)
            it['crate'] = o['crate'] if o else default_crate(it)
            it['owner'] = o['name'] if o else None
    for it in items:
        if it['file'] in WHOLE_FILE: it['crate'] = WHOLE_FILE[it['file']]
    for m in recs:
        if m['file'] in WHOLE_FILE: m['crate'] = WHOLE_FILE[m['file']]
    return dict(recs=recs, items=items, model=model, uniq=uniq, callee=callee, users=users, edges=edges, byid=byid)

if __name__ == '__main__':
    P = plan()
    for it in P['items']:
        us = sorted(P['users'][id(it)]) if it['kind'] != 'impl' else []
        print(it['crate'].ljust(18), it['file'].split('/')[-2][:14].ljust(14), it['kind'].ljust(6), (it['name'] or it['head'])[:44].ljust(44), it['vis'].ljust(10), str(it['lines']).rjust(4), ' users:', ','.join(u.replace('instance-','i-') for u in us))
