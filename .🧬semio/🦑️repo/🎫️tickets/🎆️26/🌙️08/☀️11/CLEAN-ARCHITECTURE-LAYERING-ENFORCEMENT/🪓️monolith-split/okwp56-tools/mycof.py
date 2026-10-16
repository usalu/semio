# 🧮 Item-to-crate assignment of the WP5/WP6 carve: the planner's table re-anchored to the live files by (kind, name, occurrence), impls following their types, plus the overrides of the engine decomposition (A1). Installs itself into the WP2/WP3 pipeline (`model.cof`).
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
import em
from okrs import items
import model

OWN = {  # new items of the engine decomposition (A1): root item name -> crate
    'StoreLog': 'os-store-engine-core', 'StoreOwners': 'os-store-engine-core', 'StoreWire': 'os-store-engine-core', 'StoreCursor': 'os-store-engine-core',
    'StoreLive': 'os-store-engine-live', 'StoreReplay': 'os-store-engine-live', 'StoreParts': 'os-store-engine-live', 'StorePartsRef': 'os-store-engine-live',
    'ArtifactStore': 'os-store-engine', 'StoreMintToken': 'os-store-engine-core',
}
OWN_IMPL = [   # (regex over the normalised impl header, crate)
    (r'^one_item_publication : : StoreMintWitness for StoreMintToken', 'os-store-engine-core'),
    (r'^< P , Mutation > StoreLog < P , Mutation >', 'os-store-engine-core'), (r'^< P , Mutation > StoreOwners < P , Mutation >', 'os-store-engine-core'),
    (r'^< P , Mutation > StoreWire < P , Mutation >', 'os-store-engine-core'), (r'^< P , Mutation > StoreCursor < P , Mutation >', 'os-store-engine-core'),
    (r'^< P , Mutation > StoreLive < P , Mutation >', 'os-store-engine-live'), (r'^< P , Mutation > StoreReplay < P , Mutation >', 'os-store-engine-live'),
    (r"^< 'a , P , Mutation > StoreParts", 'os-store-engine-live'),
    (r'^< P , Mutation > ArtifactStore < P , Mutation >', 'os-store-engine'),
    (r'^< P , Mutation > ArtifactStoreCloseHost < P , Mutation > for ArtifactStore', 'os-store-engine'),
    (r'^< P , Mutation > Drop for ArtifactStore < P , Mutation >', 'os-store-engine'),
    (r'^< P , Mutation > SpaceMember for ArtifactStore', 'os-store-space-member'),
    (r'^< P , Mutation > ArtifactStoreCloseView', 'os-store-engine-owners'), (r'^< P , Mutation > Default for ArtifactStoreCursorDisposer', 'os-store-engine-owners'),
    (r'^< P , Mutation > ArtifactStoreCursorDisposer', 'os-store-engine-owners'), (r'^< P , Mutation > ArtifactStoreOwnedDisposer', 'os-store-engine-owners'),
    (r'^< P , Mutation > Drop for ArtifactStoreCursorDisposer', 'os-store-engine-owners'),
    (r'^MemberDirectory for SpaceHost', 'os-store-space'), (r'^BlobStore for NoBlobStore', 'os-store-space-member'),
    (r'^< D : MemberDirectory > MemberLinkResolver', 'os-store-space-member'), (r'^< D : MemberDirectory , B : BlobStore > MemberLinkResolver', 'os-store-space-member'),
    (r'^< D : MemberDirectory , B : BlobStore > LinkResolver for MemberLinkResolver', 'os-store-space-member'),
    (r'^SpaceMember for NoMembers', 'os-store-space-member'), (r'^MemberFactory for NoMembers', 'os-store-space-member'), (r'^MemberVisit for NoMembers', 'os-store-space-member'),
    (r'^< M > Drop for SpaceHost', 'os-store-space'), (r'^< M : SpaceMember > SpaceHost', 'os-store-space'),
    (r'^semio_framework_dsl_record : : DslField for OwnerRef', 'os-store-engine-core'),
    (r"^< 'a , P , Mutation > ArtifactEnvelopeView", 'os-store-engine-core'),
    (r'^< T > ScratchOwner', 'os-store-engine-core'), (r'^< T > std : : ops : : Deref for ScratchOwner', 'os-store-engine-core'), (r'^< T > std : : ops : : DerefMut for ScratchOwner', 'os-store-engine-core'), (r'^< T > Drop for ScratchOwner', 'os-store-engine-core'),
    (r'^ArtifactStoreInitialization', 'os-store-initialization-runtime'), (r'^< P > ArtifactStoreInitializationRuntime', 'os-store-initialization-runtime'), (r'^< P > Drop for ArtifactStoreInitializationRuntime', 'os-store-initialization-runtime'),
]

def norm_header(h): return re.sub(r'\s+', ' ', h or '').strip()

def nkey(kind, name):
    n = name or ''
    if kind == 'impl':
        n = re.sub(r'^\s*impl\b', '', n)
        n = n.split('{')[0]
    n = re.sub(r'\s+', '', n)
    return (kind, n[:60])

def build(files_stale):
    out = {}
    for f, rows in files_stale.items():
        if not os.path.exists(f): continue
        src = open(f, encoding='utf8').read()
        its = items(src)
        pool = collections.defaultdict(list)
        for r in rows: pool[nkey(r['kind'], r['name'])].append(r)
        used = collections.Counter()
        res = []
        for it in its:
            sl = src.count('\n', 0, it['start']) + 1; el = src.count('\n', 0, it['end']) + 1
            crate = None
            key = nkey(it['kind'], it['name'])
            cands = pool.get(key)
            if cands:
                j = min(range(len(cands)), key=lambda q: (abs(cands[q]['start'] - sl) if len(cands) > 1 and used[key] == 0 else 0, q)) if len(cands) > 1 else 0
                if len(cands) > 1: j = min(used[key], len(cands) - 1)
                crate = cands[j]['crate']; used[key] += 1
            res.append(dict(start=sl, end=el, kind=it['kind'], name=it['name'], crate=crate))
        out[f] = res
    return out

def pool_len(pool, key): return len(pool[key])

FILE_CRATE = {
    em.S + '/🏗️initialization/📣️publication/🧬️schema/🦀️.rs': 'os-store-initialization-runtime',
    em.S + '/📜️space-history/🏠️space/📣️registration/🦀️.rs': 'os-store-space',
}

def install():
    cof = json.load(open(HERE + '/../🗑️generated/okwp23-cof.json'))
    items_ = build(cof['items'])
    root = items_[em.ROOT]
    for r in root:
        if r['kind'] in ('struct',) and r['name'] in OWN: r['crate'] = OWN[r['name']]
        if r['kind'] == 'trait' and r['name'] == 'ArtifactStoreCloseHost': r['crate'] = 'os-store-engine-owners'
        if r['kind'] == 'impl':
            h = norm_header(r['name'])
            for rx, c in OWN_IMPL:
                if re.search(rx, h): r['crate'] = c; break
    for f, c in FILE_CRATE.items():
        for r in items_.get(f, []):
            if r['kind'] not in ('use', 'mod', 'extern'): r['crate'] = c
    cof2 = dict(cof); cof2['items'] = items_
    model.cof = lambda: cof2
    model.OVERRIDE.clear()
    import functools
    model.file_model.cache_clear() if hasattr(model.file_model, 'cache_clear') else None
    return cof2

if __name__ == '__main__':
    cof2 = install()
    root = cof2['items'][em.ROOT]
    c = collections.Counter(); lines = collections.Counter()
    for r in root:
        c[r['crate']] += 1; lines[r['crate']] += r['end'] - r['start'] + 1
    for k, v in sorted(lines.items(), key=lambda kv: -kv[1]): print(k, c[k], v)
    print([ (r['kind'], r['name'][:50], r['start'], r['end']) for r in root if r['crate'] is None and r['kind'] in ('impl','struct','fn','trait','enum') and r['end']-r['start']>8])
