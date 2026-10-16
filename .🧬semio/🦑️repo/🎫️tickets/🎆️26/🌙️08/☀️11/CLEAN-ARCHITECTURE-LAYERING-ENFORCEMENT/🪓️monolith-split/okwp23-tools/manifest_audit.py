# 📦 Manifest audit of the live tree: first-party path dependencies the sources use vs the ones the manifest declares (missing = real compile break, unused = needless edge).
# usage: manifest_audit.py [crate-key ...]   (default: every crate of the b* batches)
import os, re, sys, json, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, registry, okcrates as C, manifests as MF

import extcheck as X
registry.load(refresh=True)
KIDENT = X.deps_of(ns.K)
model_ = C.load()[0]
keys = sys.argv[1:] or [k for n in ('b1', 'b2', 'b3') if os.path.exists(HERE + f'/../🗑️generated/okwp23-{n}.crates') for k in open(HERE + f'/../🗑️generated/okwp23-{n}.crates').read().split()]
KT = open(C.pkg_abs('os-spr', model_).rsplit('/📡️spr/', 1)[0] + '/../..' + '/📦️packages/🦀️rust/Cargo.toml', encoding='utf8').read() if False else open(ns.K + '/Cargo.toml', encoding='utf8').read()
THIRD = {m.group(1) for m in re.finditer(r'^([A-Za-z][\w-]*)\s*=', KT.split('[dependencies]')[1].split('[dev-dependencies]')[0], re.M) if not m.group(1).startswith('semio-framework-')}
DEP = re.compile(r'^([A-Za-z][\w-]*)\s*=', re.M)
bad = 0
for k in keys:
    ident = 'semio_framework_' + k.replace('-', '_')
    w = ns.ext_world(ident)
    pkg = C.pkg_abs(k, model_)
    toml = open(pkg + '/Cargo.toml', encoding='utf8').read()
    head, _, dev = toml.partition('[dev-dependencies]')
    declared = {m.group(1) for m in DEP.finditer(head)}
    declared_dev = {m.group(1) for m in DEP.finditer(dev)}
    normal, test = set(), set()
    for f, m in w.files.items():
        tests = False; p = m
        while p is not None:
            if any('test' in c for c in p.cfg): tests = True
            p = p.parent
        h = {x[1] for x in MF.tokenize(open(f, encoding='utf8').read()) if x[0] == 'id' and (x[1].startswith('semio_framework_') or x[1].replace('_', '-') in THIRD)}
        (test if tests else normal).update(h)
    def names(hs): return {h.replace('_', '-') for h in hs if h != ident}
    used, used_t = names(normal), names(test)
    missing = sorted((used - declared) | ((used_t - declared - declared_dev)))
    unused = sorted(d for d in declared if d != C.crate_name(k) and d not in used and d not in used_t and not d.startswith('serde_json'))
    unused_dev = sorted(d for d in declared_dev if d not in used_t and d not in used)
    bad += len(missing)
    own = X.deps_of(pkg)
    heads_all = set()
    for f, m in w.files.items(): heads_all |= MF.heads(open(f, encoding='utf8').read())
    ident_missing = sorted(h for h in heads_all if h in KIDENT and h not in own and h != ident and not h.startswith('semio_framework_os_store'))
    bad += len(ident_missing)
    print('   ident-missing', ident_missing)
    print('#####', k, 'files', len(w.files), 'deps', len(declared), 'dev', len(declared_dev), 'MISSING', missing, 'unused', unused, 'unused-dev', unused_dev)
print('missing total', bad)
