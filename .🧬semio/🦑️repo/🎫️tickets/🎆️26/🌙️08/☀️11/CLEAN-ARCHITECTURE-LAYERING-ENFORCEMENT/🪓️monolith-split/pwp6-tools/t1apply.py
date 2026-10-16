import sys; sys.path.insert(0, '.')
import t1gen, t2run, rw6, copy, t2plan
from t2plan import build_world, family_rels
from sig6 import parse_sig
from t1gen import *
import touched6

TWINS = {'take_constructor_receipt', 'store_constructor_is_pending', 'live_typed_operation_slots'}
EXT_RECV = {   # file (relative to the plugin folder) -> receivers that are mounted artifact instances; default ['app']
 '🧪️tests/🧪️time-travel/🦀️.rs': ['app', 'remote', 'pure', 'fresh', 'local', 'source', 'reloaded', 'repacked'],
 '🧪️tests/🔬️tool-run/🦀️.rs': ['app', 'remote'],
 '🧪️tests/🧪️history-edit-acceptance/🦀️.rs': ['app', 'reloaded', 'probe'],
 '🧪️tests/🧪️folder-reload-route/🦀️.rs': ['app', 'target'],
 '🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs': ['app', 'self.0'],
}
ORIGIN_LIBS = ['🧵️retained-command/🔄️full-operation/🦀️.rs']

def delegation_file(table, model, w):
    """composer-level one-line delegations of the converted `pub` methods (and the PluginApp twins)"""
    groups = collections.OrderedDict()
    for (rel, pi, mi), (info, r) in table.items():
        if not info.cf or not (info.vis == 'pub' or info.name in TWINS): continue
        f = model[rel]; p = f['pieces'][pi]; m = p.members[mi]
        txt = '\n'.join(f['R'][m.a:m.e]).rstrip('\n')
        sg = parse_sig(txt)
        head = txt[:sg['body_open']].rstrip()
        # arguments after the receiver
        names = []
        for a, b in sg['params'][1 if sg['recv'] else 0:]:
            pm = re.match(r'(?:mut\s+)?(\w+)\s*:', txt[a:b].strip())
            if not pm: raise SystemExit('delegation: unnamed parameter in ' + info.name)
            names.append(pm.group(1))
        mut = info.recv in ('mut', 'val') if info.recv else True
        pre = [rw6.param_ref_composer('self', q, mut) for q in info.params]
        call = 'self.' + info.cf + '.' + info.name + '(' + ', '.join(pre + names) + ')' + ('.await' if sg['is_async'] else '')
        if info.vis != 'pub': pass
        key = '\n'.join(p.head_lines)
        # the wrapper keeps the visibility of the original member
        groups.setdefault(key, []).append(head.rstrip() + ' {\n        ' + call + '\n    }')
    out = ['//! 🧬 The composer API of the mounted artifact instance: the public readers and entry points that delegate to a component.', 'use super::*;', '']
    for key, ms in groups.items():
        out += key.split('\n') + []
        for x in ms: out += x.split('\n') + ['']
        while out and not out[-1].strip(): out.pop()
        out.append('}'); out.append('')
    while out and not out[-1].strip(): out.pop()
    return '\n'.join(out) + '\n'

def apply_t1():
    nt, w, stats, comp, hoists = t1gen.apply_all()
    model, methods, table, known = t2plan.build_world() if False else build_world_cached()
    return nt, w, stats, comp, hoists

def ext_world(w, table):
    w2 = copy.copy(w)
    w2.methods = dict(w.methods)
    for name, info in list(w.methods.items()):
        if info.cf and (info.vis == 'pub' or info.name in TWINS):
            w2.methods[name] = rw6.MInfo(name, None, (), info.recv, info.is_async, info.trait, info.vis, info.file)
    w2.flags = []
    return w2

def external_rewrites(w, table, fam):
    w2 = ext_world(w, table)
    out = {}
    for rel in plug_files():
        r = rel.replace(PLUGP, '')
        if r in fam or rel.endswith('.ts') or '/📦️packages/' in rel: continue
        try: raw = read(rel)
        except FileNotFoundError: continue
        if 'app' not in raw: continue
        recvs = EXT_RECV.get(r, ['app'])
        new = raw
        for rc in recvs:
            new = rw6.rewrite(new, w2, 'composer', recv=rc, selfpath=False)
        if new != raw: out[r] = (raw, new)
    return out, w2

DELEG_PATH = '🧵️instance/📥️staging/🧬️composer-api/🦀️.rs'

def run(real=False):
    nt, w, stats, comp, hoists = t1gen.apply_all()
    model, methods, table, known = build_world()
    fam = set(family_rels())
    ext, w2 = external_rewrites(w, table, fam)
    deleg = delegation_file(table, model, w)
    # mount of the delegation module in the parent (inside the generated region)
    parent = nt[t1gen.PARENT]
    marker = '//#endregion 📥️StagedInstanceMounts'
    assert parent.count(marker) == 1
    parent = parent.replace(marker, '#[path = "' + os.path.relpath(PLUG + DELEG_PATH, PLUG + '🧵️instance') + '"]\nmod app_composer_api;\n' + marker)
    nt[t1gen.PARENT] = parent
    nt[DELEG_PATH] = deleg
    writes = {}
    comp_names = sorted({v[0] for v in COMPONENTS.values()})
    outside = {'⏪️time-travel/🧵️instance/🦀️.rs', '⏯️tool-run/🧵️instance/🦀️.rs', '🧵️instance/🏪️stores/📥️staging/📮️action-ingress/🦀️.rs'}
    for r in outside:
        tx = nt[r]; M = mask(tx)
        used = [n for n in comp_names if re.search(r'(?<![\w])' + n + r'\b', M)]
        add = []
        if used: add.append('use crate::artifact_instance::{' + ', '.join(used) + '};')
        if r.endswith('📮️action-ingress/🦀️.rs') and re.search(r'\bArtifactApp\b', M) and not re.search(r'use [^;]*\bArtifactApp\b', M): add.append('use crate::ArtifactApp;')
        if add:
            pos = [m for m in re.finditer(r'(?m)^use [^\n]*;\n', tx)][-1].end()
            nt[r] = tx[:pos] + '\n'.join(add) + '\n' + tx[pos:]
    for r, tx in nt.items(): writes[r] = tx
    for r, (a, b) in ext.items(): writes[r] = b
    created, modified = [], []
    plan = []
    for r, tx in writes.items():
        p = PLUG + r
        exists = os.path.exists(p)
        cur = open(p, encoding='utf-8').read() if exists else None
        if cur == tx: continue
        plan.append((r, tx, cur)); (modified if exists else created).append(r)
    print('plan:', len(created), 'created,', len(modified), 'modified; flags:', len(w.flags), 'ext flags (info):', len(w2.flags))
    if not real: return plan, w
    for r, tx, cur in plan:
        p = PLUG + r
        now = open(p, encoding='utf-8').read() if os.path.exists(p) else None
        if now != cur: raise SystemExit('PEER EDIT detected in ' + r)
    for r, tx, cur in plan:
        p = PLUG + r
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf-8').write(tx)
    touched6.merge(created, modified, [])
    json.dump(dict(created=created, modified=modified), open(GEN + '/pwp6-t1-touched.json', 'w'), ensure_ascii=False, indent=1)
    print('WRITTEN')
    return plan, w

if __name__ == '__main__' and len(sys.argv) > 1:
    run(real='--real' in sys.argv)
