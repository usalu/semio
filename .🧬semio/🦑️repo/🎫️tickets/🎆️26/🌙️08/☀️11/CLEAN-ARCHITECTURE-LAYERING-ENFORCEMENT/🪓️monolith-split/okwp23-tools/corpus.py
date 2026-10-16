# 📏 Boundary corpus rows (producers + cuts) of the carved crates, written into the OS corpus fixtures. Idempotent; each row is checked against the TS gate's own rules before it is written.
# usage: corpus.py <batch-name> dry|write <census-json>   (census-json: {crate: activeLines} measured on the live tree)
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, okcrates as C

OS = ns.M.rsplit('/', 1)[0]
CORPUS = OS + '/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json'
CUTS = OS + '/🧪️tests/🧫️fixtures/📦️crate-boundaries/🧬️cuts.json'
FORBID = ['semio-framework-os-kernel', 'semio-framework-plugin', 'semio-framework']

def rel_to_os(path): return os.path.relpath(path, OS)

def ceil_ceiling(lines):
    need = int(lines * 1.2)
    for ceiling in (500, 1000, 1500, 2000, 2500, 3000, 3500, 4000):
        if need <= ceiling: return ceiling
    return 4000

def definition(entry):
    t = open(entry, encoding='utf8').read()
    for m in re.finditer(r'^pub (?:async )?(?:fn|struct|enum|trait|type|const|static) +[A-Za-z_][A-Za-z0-9_]*', t, re.M):
        return m.group(0)
    for m in re.finditer(r'^pub mod +[A-Za-z_][A-Za-z0-9_]*', t, re.M): return m.group(0)
    return None

def producer_rows(crates, census):
    model_ = C.load()[0]
    out = []
    for k in crates:
        pkg = C.pkg_abs(k, model_)
        node = C.node_abs(k, model_)
        out.append({'package': C.crate_name(k), 'manifest': rel_to_os(pkg + '/Cargo.toml'), 'entry': rel_to_os(node + '/🦀️.rs'), 'maximumActiveLines': ceil_ceiling(census[k]), 'forbiddenDependencies': FORBID})
    return out

def cut_rows(crates, shims):
    """shims: {crate: (consumer file abs, canonical line)}"""
    model_ = C.load()[0]
    rows = []
    for k in crates:
        if k not in shims: continue
        cons, canon = shims[k]
        entry = C.node_abs(k, model_) + '/🦀️.rs'
        d = definition(entry)
        if d is None: continue
        ctext = open(cons, encoding='utf8').read()
        if canon not in ctext or d in ctext: continue
        rows.append({'producer': C.crate_name(k), 'consumer': rel_to_os(cons), 'canonical': canon, 'definition': d})
    return rows

def write_rows(new_prod, new_cuts, mode):
    c = json.load(open(CORPUS, encoding='utf8'))
    have = {r['package'] for r in c['crates']}
    add = [r for r in new_prod if r['package'] not in have]
    u = json.load(open(CUTS, encoding='utf8'))
    haveu = {(r['producer'], r['canonical']) for r in u['cuts']}
    addu = [r for r in new_cuts if (r['producer'], r['canonical']) not in haveu]
    print('corpus +%d producers, +%d cuts' % (len(add), len(addu)))
    if mode != 'write': return
    if add:
        c['crates'] += add
        open(CORPUS, 'w', encoding='utf8').write(json.dumps(c, indent=2, ensure_ascii=False) + '\n')
    if addu:
        u['cuts'] += addu
        open(CUTS, 'w', encoding='utf8').write(json.dumps(u, indent=2, ensure_ascii=False) + '\n')
