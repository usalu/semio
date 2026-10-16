import sys; sys.path.insert(0, '.')
import io, contextlib
import s0apply as S
from s0apply import *

PARENT_FILE = '🧵️instance/🦀️.rs'
APP_IMPL_FILE = '🧵️instance/🔌️plugin-app-impl/🦀️.rs'
INPLACE_MOD = {'🧷️command-routes/🦀️.rs': 'command_routes', '🧾️typed-operations/🦀️.rs': 'typed_operations', '🖼️render/🦀️.rs': 'render', '🔌️plugin-app-impl/🦀️.rs': 'plugin_app_impl', '🪜️maintenance-ladder/🦀️.rs': 'maintenance_ladder'}
SIB_FIX = [(re.compile(r'(?<![\w:])envelope_ladder::'), '')]

def modname(t):
    if t.file == PARENT_FILE and t.inplace: return None
    if t.inplace: return INPLACE_MOD.get(t.file.replace('🧵️instance/', '')) or INPLACE_MOD.get(t.file)
    return t.mod

def rel_to_parent(path): return os.path.relpath(PLUG + path, PLUG + '🧵️instance')

def build_all():
    with contextlib.redirect_stdout(io.StringIO()):
        P, targets = S.main(dry=True)
    parts, widen_log, defs, ttext = S.emit_all(P, targets)
    PK = (PARENT_FILE, 'instance-app')
    mod = {k: modname(t) for k, t in targets.items()}
    inst = lambda k: S.new_tree(targets[k]) == 'instance'
    imports = collections.defaultdict(lambda: collections.defaultdict(set))
    parent_use = collections.defaultdict(set)
    cross = collections.defaultdict(set)          # tt/tr target -> names it needs from the instance tree
    exported = {k: {n: v for n, v in d.items() if v[1] != '' and v[0] != 'impl'} for k, d in defs.items()}
    where = collections.defaultdict(list)          # name -> [target key] defining an exported item
    for k, d in exported.items():
        for n in d: where[n].append(k)
    clashes = {n: ks for n, ks in where.items() if len(ks) > 1}
    for ukey, u in targets.items():
        for name, ks in where.items():
            for mkey in ks:
                if mkey == ukey or name in defs.get(ukey, {}): continue
                kind = exported[mkey][name][0]
                if not name_ref(ttext[ukey], name, False): continue
                if inst(ukey) and inst(mkey):
                    if ukey == PK: parent_use[mod[mkey]].add(name)
                    elif mkey != PK: imports[ukey][mod[mkey]].add(name)
                elif not inst(ukey) and inst(mkey) and S.new_tree(u) in ('tt', 'tr'):
                    cross[ukey].add((name, mkey))
    # original external paths (artifact_instance::name) must keep resolving
    reexport = {}
    for it in P['items']:
        if it['kind'] == 'impl' or not it['name'] or it['file'] not in (PARENT_FILE, APP_IMPL_FILE, '🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs', '🎮️command/🎟️lease/🦀️.rs', '♻️lifecycle/🏪️stores/🦀️.rs') or it['vis'] not in ('pub', 'pub(crate)'): continue
        for mkey in where.get(it['name'], []):
            if mkey != PK and mod[mkey] is not None and not (targets[mkey].inplace and targets[mkey].file == APP_IMPL_FILE):
                reexport[it['name']] = (mod[mkey], it['vis'])
    for ukey, names in cross.items():
        for name, mkey in names:
            if mod[mkey] is not None and name not in reexport: reexport[name] = (mod[mkey], 'pub(crate)')
    return dict(P=P, targets=targets, parts=parts, widen_log=widen_log, defs=defs, ttext=ttext, mod=mod, imports=imports, parent_use=parent_use, cross=cross, reexport=reexport, clashes=clashes, PK=PK)

def assemble(B):
    P, targets, parts, mod = B['P'], B['targets'], B['parts'], B['mod']
    out = {}
    PK = B['PK']
    for key, t in targets.items():
        header, body = parts[key]
        header = list(header)
        imp_lines = []
        for mname, names in sorted(B['imports'][key].items()):
            imp_lines.append('use super::' + mname + '::{' + ', '.join(sorted(names)) + '};')
        for name, mkey in sorted(B['cross'].get(key, set())):
            if any(re.search(r'\b' + re.escape(name) + r'\b', h) for h in header): continue
            imp_lines.append('use crate::artifact_instance::' + name + ';')
        text_lines = header + imp_lines + [''] + body
        tx = '\n'.join(text_lines).rstrip('\n') + '\n'
        for rx, rep in SIB_FIX: tx = rx.sub(rep, tx)
        out[t.path] = tx
    # parent mount block
    ptx = out[PARENT_FILE]
    lines = ['', '//#region 📥️StagedInstanceMounts']
    order = sorted([t for k, t in targets.items() if not (t.inplace and t.file == PARENT_FILE) and t.file not in S.MODULE_ROOTED and S.new_tree(t) == 'instance'], key=lambda t: (S.CRATE_ORDER.index(t.crate), t.path))
    seen_mod = set()
    for t in order:
        mn = modname(t)
        cfg = ['#[cfg(any(test, feature = "artifact-app-testing"))]'] if t.file == '🪆️child/👁️read/🧪️test-view/🦀️.rs' else []
        lines += cfg + ['#[path = "' + rel_to_parent(t.path) + '"]', 'mod ' + mn + ';']
        seen_mod.add(mn)
    lines += ['#[path = "🧩️extensions/🦀️.rs"]', 'mod extensions;', 'pub(crate) use extensions::InstanceExtension;']
    pub_use = collections.defaultdict(lambda: collections.defaultdict(list))
    priv_use = collections.defaultdict(set)
    for name, (mn, vis) in B['reexport'].items(): pub_use[mn][vis].append(name)
    reexported = set(B['reexport'])
    for mn, names in B['parent_use'].items():
        for n in names:
            if n not in reexported: priv_use[mn].add(n)
    for mn in sorted(set(pub_use) | set(priv_use)):
        for vis in ('pub', 'pub(crate)'):
            if pub_use[mn][vis]: lines.append(vis + ' use ' + mn + '::{' + ', '.join(sorted(pub_use[mn][vis])) + '};')
        if priv_use[mn]: lines.append('use ' + mn + '::{' + ', '.join(sorted(priv_use[mn])) + '};')
    lines.append('pub use plugin_app_impl::*;')
    lines.append('//#endregion 📥️StagedInstanceMounts')
    out[PARENT_FILE] = ptx.rstrip('\n') + '\n' + '\n'.join(lines) + '\n'
    return out

if __name__ == '__main__':
    B = build_all()
    out = assemble(B)
    print(len(out), 'files;', len(B['widen_log']), 'widenings; clashes:', B['clashes'])
    print('reexport:', B['reexport'])
    print('cross:', {k[0].split('/')[-2] + ':' + k[1]: sorted(n for n, _ in v) for k, v in B['cross'].items()})
    lost, added = S.lossless_report(B['P'], B['targets'], out)
    print('lost', sum(lost.values()))
    for l, n in sorted(lost.items(), key=lambda kv: -kv[1])[:60]: print(n, repr(l[:120]))
