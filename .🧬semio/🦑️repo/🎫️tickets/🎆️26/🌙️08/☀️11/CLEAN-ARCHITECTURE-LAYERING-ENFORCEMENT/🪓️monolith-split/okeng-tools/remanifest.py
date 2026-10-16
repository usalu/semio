# 📦 Recomputes the Cargo manifests of the carved crates from their files on disk: every identifier used as a path head decides a first-party or external dependency (the same rules as okwp23-tools/manifests.py), the [features] block is kept from the existing manifest. usage: remanifest.py <crates-file> dry|write
import os, re, sys, json, collections
OKD = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, OKD + '/../okwp56-tools'); sys.path.insert(0, OKD + '/../okwp0-tools'); sys.path.insert(0, OKD + '/../okwp23-tools'); sys.path.insert(0, OKD)
import okcrates as C, gen_files as G, manifests as MF, ns, registry
registry.load()
crates, model_, cargo = C.build()
entries = C.old_entries(cargo)
reg = set(ns._ident_roots)
PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')
INC = re.compile(r'\binclude!\(\s*"([^"]*)"\s*\)')
def mounted(libroot):
    seen = []; stack = [libroot]
    while stack:
        f = stack.pop()
        if f in seen or not os.path.exists(f): continue
        seen.append(f)
        t = open(f, encoding='utf8').read()
        for m in list(PATH.finditer(t)) + list(INC.finditer(t)):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if p.endswith('.rs'): stack.append(p)
    return seen
def is_test_path(p): return '/🧪️tests/' in p or p.endswith('/🧪️tests/🦀️.rs')
keys = open(sys.argv[1]).read().split() if os.path.exists(sys.argv[1]) else sys.argv[1].split(','); write = sys.argv[2] == 'write'
# 🧬 names that derive expansions emit but sources never spell: Mutations/MutationLeaf expansions name protocol, os-command, ui-locale, schema-state, dsl-record (and os-store-one-item in their generated tests)
DERIVE_EXT = ['semio-framework-replication', 'semio-framework-os-command', 'semio-framework-schema-state', 'semio-framework-dsl-record']
DERIVE_DEV = ['semio-framework-os-store-one-item']
DERIVE_FIRST = ['os-dsl']
DERIVE_USERS = {'os-store-space-schema'}
FIRST = MF.FIRST; SKIP = MF.SKIP
for key in keys:
    c = crates[key]
    root = C.node_abs(key, model_) + '/🦀️.rs'
    files = mounted(root)
    normal, test = set(), set()
    for f in files:
        h = MF.heads(open(f, encoding='utf8').read())
        (test if is_test_path(f) else normal).update(h)
    first, ext, dev_first, dev_ext, missing = set(), set(), set(), set(), set()
    for h in normal:
        if h in SKIP: continue
        if h in FIRST:
            if FIRST[h] != key: first.add(FIRST[h])
            continue
        if h in reg or h in C.IDENT_KEY or h.startswith('semio_framework_'):
            k = C.ident_key(h)
            if k in entries: ext.add(k)
            else: missing.add(h)
    for h in test - normal:
        if h in SKIP: continue
        if h in FIRST:
            if FIRST[h] != key: dev_first.add(FIRST[h])
            continue
        if h in reg or h in C.IDENT_KEY or h.startswith('semio_framework_'):
            k = C.ident_key(h)
            if k in entries: dev_ext.add(k)
            else: missing.add(h)
    if key in DERIVE_USERS:
        for d in DERIVE_EXT:
            if d in entries: ext.add(d)
            else: missing.add(d)
        for d in DERIVE_DEV:
            if d in entries: dev_ext.add(d)
        first.update(DERIVE_FIRST)
    pkg = C.pkg_abs(key, model_)
    old = open(pkg + '/Cargo.toml', encoding='utf8').read()
    fm = re.search(r'\n\[features\]\n(.*?)(?=\n\[|\Z)', old, re.S)
    c['features'] = [l for l in fm.group(1).split('\n') if l.strip()] if fm else []
    c['deps'] = sorted(first, key=C.RANK.index)
    c['ext'] = sorted(ext)
    c['dev'] = sorted(dev_ext) + ['__first__' + d for d in sorted(dev_first - first)]
    c['real'] = True; c['dep_features'] = {}; c['cfg'] = 'all'
    dev_first_l = [d[len('__first__'):] for d in c['dev'] if d.startswith('__first__')]
    c['dev'] = [d for d in c['dev'] if not d.startswith('__first__')]
    text, miss2 = G.manifest(crates, key, model_, entries)
    extra = [C.crate_name(d) + ' = { path = "' + os.path.relpath(C.pkg_abs(d, model_), pkg) + '" }' for d in dev_first_l]
    if extra: text = text.rstrip('\n') + '\n' + '\n'.join(sorted(extra)) + '\n'
    def deps_of(t): return sorted(set(re.findall(r'^(semio-framework-[a-z0-9-]+|[a-z][a-z0-9_-]*) = ', t.split('[dev-dependencies]')[0], re.M)) - {'name', 'version', 'edition', 'description', 'path', 'role', 'id', 'workspace'})
    o, n = set(deps_of(old)), set(deps_of(text))
    print(f'{key}: files {len(files)} +{sorted(n - o)} -{sorted(o - n)} missing {sorted(missing)}')
    if write and text != old: open(pkg + '/Cargo.toml', 'w', encoding='utf8').write(text)
