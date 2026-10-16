# 🚚 A2 driver: carves the WP5/WP6 batch with the WP2/WP3 pipeline (re-anchored assignment, predicted paths for planned crates), reports violations and writes the dump (outputs, manifests, fold map); `apply` writes it to the repository under hash guards.
# usage: a2.py <batch> dump|apply
import os, sys, json, hashlib, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a2lib
from a2lib import *
import mycof, manifests, apply as AP
from okrs import items
GEN = HERE + '/../🗑️generated'
ROOT = ns.S + '/🦀️.rs'

def prepare():
    register_all()
    w = ns.world()
    w.files[ROOT].name = 'store'
    # root-mounted test modules stay in the facade until the companion packages (WP8)
    for f, m in w.files.items():
        anc = m; test = False
        while anc is not None:
            if any('test' in c for c in anc.cfg): test = True
            anc = anc.parent
        if test and m.parent is not None and m.parent.file == ROOT and f not in model.cof()['items']:
            model.FILE_OVERRIDE[f] = None
    for f, c in FILE_CRATE.items():
        if os.path.exists(f): model.FILE_OVERRIDE[f] = c

FILE_CRATE = mycof.FILE_CRATE

def run(name):
    crates = open(f'{GEN}/okwp56-{name}.crates').read().split()
    prepare()
    before = {f: AP.sha(f) for f in ns.world().files}
    base_text = {f: m.text for f, m in ns.world().files.items() if m.key == m.file}
    b = a2lib.solve(crates, auto=os.environ.get('A2_NOAUTO') != '1')
    b.base_text = base_text
    return b, crates, before

def report(b):
    print(len(b.out), 'files', dict(b.widened))
    if b.plan.relocate:
        print('relocated test files:', len(b.plan.relocate))
        for a in sorted(b.plan.relocate): print('   ', model.short(a))
    for v in b.rw.rep.violations[:120]: print('VIOLATION', v)
    print('violations', len(b.rw.rep.violations))
    print('dropped imports', len(b.rw.rep.dropped_imports)); [print('  dropped', x) for x in b.rw.rep.dropped_imports[:80]]
    print('globs explicitised', [(x[0][-60:], x[1][:12]) for x in b.rw.rep.glob_expanded])
    RK = C.RANK
    for (s, d), n in sorted(b.rw.rep.cross.items()):
        if s in RK and d in RK and RK.index(d) >= RK.index(s):
            print('BACK-EDGE', s, '->', d, n, b.rw.rep.cross_examples[(s, d)][:4])
    print('cross', dict(b.rw.rep.cross))
    print('externs', sorted(set(i for (c, i) in b.rw.rep.externs)))
    bad = []
    for (c, text) in sorted(b.rw.rep.extern_paths):
        segs = text.split('::')
        ok = ns.extern_exists(segs[:4])
        if ok is not True and segs[0] not in ('std', 'core', 'alloc'): bad.append((c, text, ok))
    print('extern paths checked', len(b.rw.rep.extern_paths), 'not proven', len(bad))
    for x in bad[:60]: print('  EXTERN?', x)
    if b.notes: print('notes', b.notes[:10])

def dump(b, crates, before, path):
    json.dump({'batch': b.plan.batch, 'out': b.out, 'new': sorted(b.new_files), 'roots': b.plan.root_file,
               'fold': {st.path: sf for (sf, c), st in b.plan.staging.items()}, 'hashes': before, 'base_text': getattr(b, 'base_text', {}),
               'externs': sorted(f'{c}|{p}' for (c, p) in b.rw.rep.extern_paths), 'relocated': sorted(b.plan.relocate)}, open(path, 'w'), ensure_ascii=False)

FEATURE_RX = __import__('re').compile(r'feature\s*=\s*"([^"]+)"')

def features_of(b):
    out = collections.defaultdict(set)
    for f, c in b.plan.file_crate.items():
        for m in FEATURE_RX.finditer(b.out.get(f) or ''): out[c].add(m.group(1))
    for (sf, c), st in b.plan.staging.items():
        for m in FEATURE_RX.finditer(b.out.get(st.path) or ''): out[c].add(m.group(1))
    return out

def apply(b, crates, before):
    assert not b.rw.rep.violations, ('violations', b.rw.rep.violations[:3])
    feats = features_of(b)
    FEATURES = {k: [f + ' = []' for f in sorted(v - {'deflate', 'sync', 'ureq', 'worker', 'typegen', 'native-bin'})] for k, v in feats.items()}
    texts, rep = manifests.render(b, FEATURES, {})
    print('manifest keys missing:', {k: v['missing'] for k, v in rep.items() if v['missing']})
    plan = []
    for p, t in sorted(b.out.items()):
        old = open(p, encoding='utf8').read() if os.path.exists(p) else None
        if old is None: plan.append(('new', p))
        elif old != t: plan.append(('edit', p))
    model_ = C.load()[0]
    ktext = open(AP.KERNEL_TOML, encoding='utf8').read()
    new_k, deps_block, dev = AP.kernel_toml_edit(ktext, crates, b.plan.crates_model, model_)
    print(len(plan), 'file operations;', 'kernel +%d deps' % len(deps_block))
    return plan, texts, new_k, ktext

def three_way(base, mine, theirs):
    import subprocess, tempfile
    with tempfile.TemporaryDirectory(dir='/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/okwp56') as d:
        paths = []
        for n, t in (('theirs', theirs), ('base', base), ('mine', mine)):
            q = os.path.join(d, n); open(q, 'w', encoding='utf8').write(t); paths.append(q)
        r = subprocess.run(['git', 'merge-file', '-p', '-L', 'disk', '-L', 'base', '-L', 'carve'] + paths, capture_output=True, text=True)
        return r.returncode, r.stdout

def write_all(b, crates, plan, texts, new_k, ktext):
    model_ = C.load()[0]
    merged = {}
    for kind, p in plan:
        if kind == 'new':
            assert not os.path.exists(p), p
            merged[p] = b.out[p]
        else:
            cur = open(p, encoding='utf8').read()
            base = b.base_text.get(p)
            if base is None or cur == base: merged[p] = b.out[p]
            else:
                rc, res = three_way(base, b.out[p], cur)
                assert rc == 0, ('merge conflict', p, rc)
                merged[p] = res; print('merged concurrent edits in', p[-60:])
    for kind, p in plan: AP.write(p, merged[p])
    for k in crates:
        mp = C.pkg_abs(k, model_) + '/Cargo.toml'
        AP.write(mp, texts[k])
    ktext = open(AP.KERNEL_TOML, encoding='utf8').read()
    new_k, deps_block, dev = AP.kernel_toml_edit(ktext, crates, b.plan.crates_model, model_)
    feats = features_of(b)
    owners = sorted(k for k, v in feats.items() if 'durable-group-testing' in v)
    if owners and 'durable-group-testing = []' in new_k:
        fwd = ', '.join('"' + C.crate_name(k) + '/durable-group-testing"' for k in owners)
        new_k = new_k.replace('durable-group-testing = []', 'durable-group-testing = [' + fwd + ']', 1)
    AP.write(AP.KERNEL_TOML, new_k)
    print('applied')

if __name__ == '__main__':
    name, mode = sys.argv[1], sys.argv[2]
    b, crates, before = run(name)
    report(b)
    dump(b, crates, before, f'{GEN}/okeng-out-{name}.json')
    if mode == 'apply':
        AP.TOUCHED = HERE + '/touched.txt'
        plan, texts, new_k, ktext = apply(b, crates, before)
        write_all(b, crates, plan, texts, new_k, ktext)
