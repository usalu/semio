"""🧭 Boots the WP2/WP3 namespace resolver over every first-party crate of the workspace."""
import sys, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE + '/../okwp0-tools')
import crates, ns
R = crates.R
def boot(all_members=True):
    cat = crates.catalog()
    if all_members:
        # every workspace member, not only the OS family (value, replication, pack ...)
        for m in crates.members():
            try: c = crates.read_manifest(m)
            except Exception: continue
            if not c['name']: continue
            lp = c['lib_path']
            if lp is None: continue
            root = os.path.normpath(R + '/' + m + '/' + lp)
            cat.setdefault(c['name'], dict(c, root=root[len(R) + 1:]))
    fm = open(R + '/🧰️framework/Cargo.toml', encoding='utf8').read()
    mm = re.search(r'members\s*=\s*\[(.*?)\]', fm, re.S)
    if mm:
        for rel0 in re.findall(r'"([^"]+)"', mm.group(1)):
            rel = '🧰️framework/' + rel0
            try: c = crates.read_manifest(rel)
            except Exception: continue
            if not c['name'] or c['lib_path'] is None: continue
            root = os.path.normpath(R + '/' + rel + '/' + c['lib_path'])
            cat.setdefault(c['name'], dict(c, root=root[len(R) + 1:]))
    for toml in (R + '/Cargo.toml', R + '/🧰️framework/Cargo.toml'):
        base = os.path.dirname(toml)
        for m in re.finditer(r'^([A-Za-z0-9_-]+)\s*=\s*\{[^}]*?path\s*=\s*"([^"]+)"', open(toml, encoding='utf8').read(), re.M):
            rel = os.path.relpath(os.path.normpath(os.path.join(base, m.group(2))), R)
            try: c = crates.read_manifest(rel)
            except Exception: continue
            if not c['name'] or c['lib_path'] is None: continue
            root = os.path.normpath(R + '/' + rel + '/' + c['lib_path'])
            cat.setdefault(c['name'], dict(c, root=root[len(R) + 1:]))
    for name, c in cat.items():
        ident = c['lib_name'] or name.replace('-', '_')
        ns.register_crate(ident, R + '/' + c['root'])
    return cat
if __name__ == '__main__':
    cat = boot()
    print(len(cat))
    for p in (['semio_framework_os_store_engine', 'lifted_store', 'ArtifactStore'], ['semio_framework_os_store_engine', 'ArtifactStore'], ['semio_framework_os_store_ledger', 'ArtifactEditMessageLedger'], ['semio_framework_os_spr', 'Mutation'], ['semio_framework_os_vcs', 'io', 'binary', 'genesis', 'ArtifactPackGenesisProducer']):
        print(p, ns.extern_exists(p))
