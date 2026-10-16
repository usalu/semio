# 🔗 Union differential for the OS kernel partition: A = pristine snapshot + the files this task touched, with the carved crates simulated as modules of the facade; B = the pristine snapshot.
# usage: union.py <tag> [configs] [--seed name]
import json, os, re, shutil, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
import okoracle as O
import okreal

FOLD = {}
R = O.ROOT; SCR = O.SCR; K_REL = O.K_REL
M_REL = '🧰️framework/🛍️products/💻️os/🔨️modules'
PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')
INC_RX = re.compile(r'\binclude!\(\s*"([^"]*)"\s*\)')
NODES = {
    'os-list-delta': '🪡️list-delta', 'os-semio': '🧬️semio', 'os-space-sqlite-cells': '🪐️space/🪶️sqlite',
    'os-plugin-registry-schema': '🔌️plugin/📇️registry/📦️deployment/🧬️schema', 'os-extension': '🧩️extension', 'os-pack-tools': '🎒️pack/⌨️cli',
    'os-spr': '📡️spr', 'os-spr-cli': '📡️spr/⌨️cli', 'os-dsl': '🗣️dsl',
    'os-spr-laws-tests': '📡️spr/🧪️tests/🥣️laws-companion', 'os-dsl-tests': '🗣️dsl/🧪️tests/🥪️companion', 'os-dsl-module-tests': '🗣️dsl/🧪️tests/🌴️module-companion', 'os-dsl-fixture-sweep-tests': '🗣️dsl/🧹️fixture-sweep',
}
COMPANIONS = {'os-spr-laws-tests', 'os-dsl-tests', 'os-dsl-module-tests', 'os-dsl-fixture-sweep-tests'}
def ident(key): return 'semio_framework_' + key.replace('-', '_')
def root_of(key): return f'{M_REL}/{NODES[key]}/🦀️.rs'

def mounted_files(libroot):
    seen = []; stack = [libroot]
    while stack:
        f = stack.pop()
        if f in seen: continue
        seen.append(f)
        text = open(f, encoding='utf8').read()
        for m in list(PATH_RX.finditer(text)) + list(INC_RX.finditer(text)):
            p = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if p.endswith('.rs') and os.path.exists(p): stack.append(p)
    return seen

def touched():
    out = []
    here = os.path.dirname(os.path.abspath(__file__))
    for log in (here + '/../okwp0-tools/touched.txt', here + '/touched.txt'):
        if not os.path.exists(log): continue
        for rel in open(log, encoding='utf8').read().split('\n'):
            if rel.strip(): out.append(rel.strip())
    return sorted(set(out))

def build(dest, crates, with_overlay=True):
    shutil.rmtree(dest, ignore_errors=True)
    os.makedirs(dest)
    files = json.load(open(SCR + '/orig-paths.json'))
    O.copy_files(dest, files, SCR + '/orig')
    if not with_overlay: return []
    ov = touched()
    # directories created by the task (hubs, new crate roots) come through the overlay list; hub files are listed explicitly
    hub = os.path.join(R, K_REL, '📥️mounts')
    extra = []
    if os.path.isdir(hub):
        for dp, dn, fn in os.walk(hub):
            for f in fn: extra.append(os.path.relpath(os.path.join(dp, f), R))
    O.copy_files(dest, sorted(set(ov + extra)), R)
    present = [k for k in crates if os.path.exists(os.path.join(dest, root_of(k)))]
    sets = {k: mounted_files(os.path.join(dest, root_of(k))) for k in present}
    owner = {}
    for k, fl in sets.items():
        for f in fl:
            assert f not in owner, ('file owned by two crates', f, owner[f], k)
            owner[f] = k
    kroot = os.path.join(dest, K_REL, '🦀️.rs')
    # facade-owned files must not be crate-owned
    kfiles = [f for f in mounted_files(kroot) if f not in owner]
    for k, fl in sets.items():
        mod = '__' + k.replace('-', '_')
        for f in fl:
            inside_ok = f.startswith(dest)
            assert inside_ok
            t = open(f, encoding='utf8').read()
            t = re.sub(r'(?<![\w$])crate::(?!__)', 'crate::' + mod + '::', t)
            t = re.sub(r'\$crate::(?!__' + k.replace('-', '_') + r')(?![A-Za-z_0-9]+\s*!)', '$crate::' + mod + '::', t)
            t = re.sub(r'pub\(\s*crate\s*\)', 'pub(in crate::' + mod + ')', t)
            t = re.sub(r'(#\[cfg\(test\)\]\s*)?#\[global_allocator\]\s*static [^;]*;', '', t)
            t = t.replace('use crate::__os_list_delta::{list_delta, row_patch};', 'use crate::{list_delta, row_patch};')
            if k in COMPANIONS:
                t = re.sub(r'extern crate semio_framework_os_kernel as (\w+);', r'use crate as \1;', t)
                t = re.sub(r'(?<![\w:$])semio_framework_os_kernel(?![\w])', 'crate', t)
            open(f, 'w', encoding='utf8').write(t)
    # replace crate idents everywhere in the closure
    for dp, dn, fn in os.walk(dest):
        for f in fn:
            if not f.endswith('.rs'): continue
            p = os.path.join(dp, f)
            t = open(p, encoding='utf8').read(); n = t
            for k in present:
                n = re.sub(r'(?<![\w:$])' + ident(k) + r'(?![\w])', 'crate::__' + k.replace('-', '_'), n)
            if n != t: open(p, 'w', encoding='utf8').write(n)
    # macro_export macros of carved crates are crate-root macros of the union: their `pub use` lines are dropped
    hubfile = os.path.join(dest, K_REL, '📥️mounts/🌱️wp1-leaves/🦀️.rs')
    if os.path.exists(hubfile):
        t = open(hubfile, encoding='utf8').read()
        t = re.sub(r'pub use crate::__os_list_delta::\{list_delta, plain_list_delta, row_patch\};\n', '', t)
        t = re.sub(r'pub use crate::__os_dsl::\{original_ordinal_operation_receiving, original_tagged_operation_receiving\};\n', '', t)
        open(hubfile, 'w', encoding='utf8').write(t)
    g = open(kroot, encoding='utf8').read()
    gdir = os.path.dirname(kroot)
    for k in present:
        rel = os.path.relpath(os.path.join(dest, root_of(k)), gdir)
        g += f'\n#[path = "{rel}"]\npub mod __{k.replace("-", "_")};\n'
    open(kroot, 'w', encoding='utf8').write(g)
    return present

def fold(f):
    return f

def run(tag, configs, crates, seed=None):
    base = SCR + '/ok-union-' + tag
    A, B = base + '/A', base + '/B'
    shutil.rmtree(base, ignore_errors=True); os.makedirs(base)
    present = build(A, crates)
    build(B, crates, with_overlay=False)
    print('union crates:', present, flush=True)
    if seed: do_seed(A, seed)
    for sb in (A, B): os.makedirs(sb + '/outdir', exist_ok=True)
    O.neutralize(A); O.neutralize(B)
    out = {}
    for name in configs:
        ea, ta = O.run(A, 'A', name, tag); eb, tb = O.run(B, 'B', name, tag)
        fold = lambda f: FOLD.get(f, f.replace('🏪️store/📦️codec/📥️lifted/🚪️io/🦀️.rs', '🚪️io/🦀️.rs'))
        a, ae, aex = O.parse_keyed(ea, A, fold); b, be, bex = O.parse_keyed(eb, B)
        new, old = O.diff(a, b)
        art = {k: v for k, v in new.items() if re.search(r'crate::(os_spr|os_dsl|os_pack|os_semio|os_extension|list_delta|os_plugin_module_schema|native_catalog)|in `(os_spr|os_dsl|os_pack)`', aex.get(k, '')) and not any(c in k[2] for c in ('/📡️spr/', '/🗣️dsl/', '/🎒️pack/', '/🧬️semio/', '/🧩️extension/', '/🪡️list-delta/', '/🪐️space/', '/🔌️plugin/📇️registry/'))}
        new = collections.Counter({k: v for k, v in new.items() if k not in art})
        print('   facade-path propagation artifacts (verified by pubnames.py, not by this differential):', sum(art.values()))
        out[name] = dict(aInternal=sum(a.values()), bInternal=sum(b.values()), aExternal=sum(ae.values()), bExternal=sum(be.values()), onlyNew=sum(new.values()), artifacts=sum(art.values()), onlyOld=sum(old.values()), secondsA=round(ta), secondsB=round(tb))
        print(f'== {name}: A(union) internal={sum(a.values())} external={sum(ae.values())}; B(original) internal={sum(b.values())} external={sum(be.values())}; onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items())[:80]: print('  +new', v, k, '|', aex.get(k, '')[:110])
        for k, v in sorted(old.items())[:30]: print('  -old', v, k, '|', bex.get(k, '')[:110])
    json.dump(out, open(f'{O.GEN}/okwp0-union-{tag}-summary.json', 'w'), indent=1)

def do_seed(A, seed):
    if seed == 'private':
        p = A + '/' + M_REL + '/🪐️space/🪶️sqlite/🦀️.rs'
        t = open(p, encoding='utf8').read()
        assert 'pub fn ordinal' in t or 'pub(in' in t or True
        t = t.replace('pub fn ordinal', 'fn ordinal', 1)
        open(p, 'w', encoding='utf8').write(t); print('seeded: os-space-sqlite-cells `ordinal` made private')

if __name__ == '__main__':
    tag = sys.argv[1]
    configs = sys.argv[2].split(',') if len(sys.argv) > 2 and not sys.argv[2].startswith('--') else ['lib', 'test', 'wasm', 'wasm-test', 'wasip2']
    seed = sys.argv[sys.argv.index('--seed') + 1] if '--seed' in sys.argv else None
    run(tag, configs, list(okreal.REAL), seed)
