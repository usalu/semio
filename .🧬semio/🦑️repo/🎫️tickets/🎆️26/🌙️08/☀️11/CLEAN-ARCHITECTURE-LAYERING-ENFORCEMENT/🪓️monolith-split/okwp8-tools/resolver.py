"""🧭 Path resolver of the WP8 tools: kernel-root and store-root paths -> canonical first-party crate paths (validated through ns.extern_exists)."""
import sys, os, re, functools
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import nsboot, ns, rootns
CAT = nsboot.boot()
NS = rootns.build()
KERNEL_MODULES = {
    'os_spr': 'semio_framework_os_spr', 'os_vcs': 'semio_framework_os_vcs', 'os_pack': 'semio_framework_os_pack', 'os_dsl': 'semio_framework_os_dsl',
    'io_schema': 'semio_framework_io_schema', 'sqlite_snapshot': 'semio_framework_io_sqlite_snapshot', 'os_semio': 'semio_framework_os_semio',
    'os_inference': 'semio_framework_os_inference', 'os_identity': 'semio_framework_os_identity', 'list_delta': 'semio_framework_os_list_delta',
    'os_plugin_module_schema': 'semio_framework_os_plugin_registry_schema', 'os_extension': 'semio_framework_os_extension', 'os_component_receipt': 'semio_framework_plugin_component_receipt',
}
KERNEL_ALIASES = {'store', 'dsl', 'spr', 'vcs', 'semio_format', 'dsl_grammar'}
KERNEL_FLAT = ['semio_framework_os_spr', 'semio_framework_os_vcs', 'semio_framework_os_pack', 'semio_framework_os_dsl', 'semio_framework_os_inference', 'semio_framework_os_semio', 'semio_framework_ui_locale', 'semio_framework_dsl_record', 'semio_framework_value']
IO_HUB = [['semio_framework_os_io'], ['semio_framework_os_store_codec_registry', 'lifted_io']]
def exists(segs):
    r = ns.extern_exists(list(segs))
    return r
def first_existing(cands):
    unknown = None
    for c in cands:
        r = exists(c)
        if r is True: return list(c)
        if r is None and unknown is None: unknown = list(c)
    return unknown
@functools.lru_cache(None)
def sroot_name(name, rest=()):
    """Resolve `name::rest...` in the namespace of the kernel store root S/🦀️.rs to a canonical crate path (list) or None."""
    rest = list(rest)
    e = NS['explicit'].get(name)
    if e is not None:
        path = list(e['path'])
        return canon_path(path + rest)
    h = NS['hubs'].get(name)
    if h is not None:
        if not rest: return None
        cands = []
        for segs, alias, glob in h['uses']:
            if glob: cands.append(canon_path(list(segs)) + rest)
            else:
                nm = alias or segs[-1]
                if nm == rest[0]: cands.append(canon_path(list(segs)) + rest[1:])
        cands = [c for c in cands if c]
        return first_existing(cands)
    if name in NS['defs']: return None
    for segs, vis, line in NS['globs']:
        c = canon_path(list(segs)) + [name] + rest
        r = exists(c)
        if r is True: return c
    return None
def canon_path(segs):
    """Substitute S-root aliases / crate heads of a path written in the store root."""
    if not segs: return segs
    s0 = segs[0]
    if s0 == '::': return canon_path(segs[1:])
    if s0 == 'crate': return kroot(segs[1:]) or list(segs)
    if s0 in NS['explicit'] and not s0.startswith('semio_framework') and s0 not in ('protocol', 'pack'):
        e = NS['explicit'][s0]
        sub = canon_path(list(e['path']))
        return sub + segs[1:]
    return list(segs)
KERNEL_DERIVES = {'Mutations', 'MutationLeaf', 'DslArtifact', 'diff_text', 'diff_binary'}
def kroot(segs):
    """Resolve a path written as `crate::segs` inside the kernel package to a canonical crate path, or None."""
    if not segs: return None
    s0 = segs[0]; rest = segs[1:]
    if s0 in KERNEL_DERIVES and not rest: return ['semio_framework_os_dsl', s0]
    if s0 in KERNEL_MODULES: return [KERNEL_MODULES[s0]] + rest
    if s0 == 'os_store':
        if not rest: return None
        return sroot_name(rest[0], tuple(rest[1:]))
    if s0 == 'io':
        cands = [c + rest for c in IO_HUB]
        return first_existing(cands)
    cands = [[c, s0] + rest for c in KERNEL_FLAT]
    r = first_existing(cands)
    if r: return r
    return sroot_name(s0, tuple(rest))
def kernel_alias(segs):
    """`store::X`, `dsl::X`, `vcs::X`, `spr::X`, `semio_format::X` (extern crate self aliases of the kernel root)."""
    return kroot(segs[1:])
if __name__ == '__main__':
    for p in (['crate','os_store','tests','DemoSnapshot'], ['crate','os_spr','Mutation'], ['crate','os_vcs','io','binary','genesis','ArtifactPackGenesisProducer'], ['crate','io','io_mechanism'], ['crate','io','testing'],
              ['crate','os_store','serialization','ArtifactSqliteSnapshotHost'], ['crate','os_store','document_store_owners','bounded_artifact_store_owners'], ['crate','LocalizedLabel'], ['crate','sqlite_snapshot','SqliteSnapshotControl']):
        print(p, '->', kroot(p[1:]))
    for n, rest in (('ArtifactEditMessageLedger', ()), ('replay', ('X',)), ('snapshot_read', ('ErasedSnapshotRead',)), ('canonical_edit', ('ArtifactCanonicalJson',)), ('ledger', ('ArtifactEditMessageLedger',)), ('Mutation', ())):
        print(n, rest, '->', sroot_name(n, rest))
