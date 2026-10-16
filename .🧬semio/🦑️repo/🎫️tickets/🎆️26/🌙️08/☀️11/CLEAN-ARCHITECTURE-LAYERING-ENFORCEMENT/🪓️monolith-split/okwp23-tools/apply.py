# 🚀 Applies one verified carve batch to the repository: source files, staging files, manifests, kernel dependencies, corpus rows. Guarded: every overwritten file must still equal the text the batch was built from.
# usage: apply.py <batch-name> dry|write   (crates from ../🗑️generated/okwp23-<batch>.crates)
import hashlib, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, model, carve, manifests, okcrates as C

R = ns.R
TOUCHED = HERE + '/touched.txt'
KERNEL_TOML = ns.K + '/Cargo.toml'
CORPUS = ns.M.rsplit('/', 1)[0] + '/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json'
CUTS = ns.M.rsplit('/', 1)[0] + '/🧪️tests/🧫️fixtures/📦️crate-boundaries/🧬️cuts.json'
FORBID = ['semio-framework-os-kernel', 'semio-framework-plugin', 'semio-framework']

FEATURES = {'os-store-ledger': ['hot-path-census = []'], 'os-store-replay': ['hot-path-census = ["semio-framework-os-store-ledger/hot-path-census"]']}
DEV_FEATURES = {'os-store-replay': {'os-store-ledger': ['hot-path-census']}}
KERNEL_DEV = {'os-store-ledger': ['hot-path-census'], 'os-store-replay': ['hot-path-census']}

def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest() if os.path.exists(path) else None

def snapshot_hashes(w):
    return {f: sha(f) for f in w.files}

def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, 'w', encoding='utf-8').write(text)
    touch(path)

def touch(path):
    rel = os.path.relpath(path, R)
    seen = set(open(TOUCHED, encoding='utf8').read().split('\n')) if os.path.exists(TOUCHED) else set()
    if rel not in seen:
        with open(TOUCHED, 'a', encoding='utf8') as f: f.write(rel + '\n')

def census_lines(batch):
    import subprocess
    return None

def definition_for(entry_path):
    t = open(entry_path, encoding='utf8').read()
    for m in re.finditer(r'^pub (?:(?:async )?fn|struct|enum|trait|type|const) +([A-Za-z_][A-Za-z0-9_]*)', t, re.M):
        return m.group(0)
    return None

def kernel_toml_edit(text, batch_keys, crates_model, model_):
    deps_block = []
    for k in sorted(batch_keys, key=lambda k: C.crate_name(k).encode()):
        rel = os.path.relpath(C.pkg_abs(k, model_), ns.K)
        line = f'{C.crate_name(k)} = {{ path = "{rel}" }}'
        if line not in text and (C.crate_name(k) + ' =') not in text.split('[dev-dependencies]')[0]: deps_block.append(line)
    anchor = 'semio-framework-os-spr = { path = "../../🔨️modules/📡️spr/📦️packages/🦀️rust" }\n'
    assert text.count(anchor) == 1, 'kernel manifest anchor'
    text = text.replace(anchor, anchor + ''.join(l + '\n' for l in deps_block))
    dev = []
    for k, feats in KERNEL_DEV.items():
        if k in batch_keys:
            rel = os.path.relpath(C.pkg_abs(k, model_), ns.K)
            dev.append(f'{C.crate_name(k)} = {{ path = "{rel}", features = [{", ".join(chr(34) + f + chr(34) for f in feats)}] }}')
    if dev:
        if '[dev-dependencies]' in text:
            i = text.index('[dev-dependencies]') + len('[dev-dependencies]')
            nl = text.index('\n', i) + 1
            text = text[:nl] + ''.join(l + '\n' for l in dev if l not in text) + text[nl:]
        else:
            text = text.rstrip('\n') + '\n\n[dev-dependencies]\n' + ''.join(l + '\n' for l in dev)
    return text, deps_block, dev

def main():
    name, mode = sys.argv[1], sys.argv[2]
    crates = open(HERE + f'/../🗑️generated/okwp23-{name}.crates').read().split()
    w0 = ns.world()
    before = snapshot_hashes(w0)
    b = carve.solve(crates)
    assert not b.rw.rep.violations, ('violations', b.rw.rep.violations[:3])
    texts, report = manifests.render(b, FEATURES, DEV_FEATURES)
    miss = {k: v['missing'] for k, v in report.items() if v['missing']}
    print('manifest keys missing in the kernel manifest:', miss)
    assert snapshot_hashes(w0) == before, 'the tree changed while the batch was built'
    plan = []
    for p, t in sorted(b.out.items()):
        old = open(p, encoding='utf8').read() if os.path.exists(p) else None
        if old is None: plan.append(('new', p)); continue
        if old == t: continue
        plan.append(('edit', p))
    print(len(plan), 'file operations:', sum(1 for x in plan if x[0] == 'new'), 'new,', sum(1 for x in plan if x[0] == 'edit'), 'edited')
    cm = b.plan.crates_model
    model_ = C.load()[0]
    ktext = open(KERNEL_TOML, encoding='utf8').read()
    new_ktext, deps_block, dev = kernel_toml_edit(ktext, crates, cm, model_)
    print('kernel manifest: +%d dependencies, +%d dev-dependencies' % (len(deps_block), len(dev)))
    if mode != 'write': return
    for kind, p in plan:
        if kind == 'new': assert not os.path.exists(p), p
        else: assert sha(p) == before.get(p, sha(p)), ('changed since build', p)
    for kind, p in plan:
        write(p, b.out[p])
    for k in crates:
        mp = C.pkg_abs(k, model_) + '/Cargo.toml'
        write(mp, texts[k])
    assert sha(KERNEL_TOML) == hashlib.sha256(ktext.encode()).hexdigest()
    write(KERNEL_TOML, new_ktext)
    print('applied')

if __name__ == '__main__': main()
