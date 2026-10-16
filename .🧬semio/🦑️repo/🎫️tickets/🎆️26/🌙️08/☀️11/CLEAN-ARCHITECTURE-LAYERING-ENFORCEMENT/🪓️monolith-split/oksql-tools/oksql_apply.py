# 🚀 Guarded apply of the WP7 sync carve: crate sources, platform roots, manifests, kernel mounts and kernel manifest. Run only after the engine crates exist (wave-5 rule 3).
# usage: oksql_apply.py dry|write
import hashlib, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import oksql
import carve, manifests, model, ns, okcrates as C
sys.path.insert(0, HERE + '/../okwp23-tools')
import apply as AP
import surface

R = ns.R
CRATES = ['os-store-sync', 'os-store-sync-native', 'os-store-sync-wasm']
SY = model.S + '/🔄️sync'
S_ROOT = model.S + '/🦀️.rs'
K_ROOT = ns.K + '/🦀️.rs'
KERNEL_TOML = ns.K + '/Cargo.toml'
TOUCHED = HERE + '/touched.txt'
FEATURES = {'os-store-sync': ['testing = []'], 'os-store-sync-native': ['testing = []']}
TOKIO_NATIVE = 'tokio = { version = "1", default-features = false, features = ["sync", "macros", "rt", "time", "net"] }'
K_MOUNT_OLD = '''    #[cfg(all(feature = "sync", not(all(target_arch = "wasm32", target_env = "p2"))))]
    #[path = "../../🔨️modules/🏪️store/🔄️sync/🦀️.rs"]
    pub mod sync;
'''
K_MOUNT_NEW = '''    #[cfg(all(feature = "sync", not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub mod sync {
        pub use semio_framework_os_store_sync::*;
        #[cfg(not(target_arch = "wasm32"))]
        pub use semio_framework_os_store_sync_native::*;
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        pub use semio_framework_os_store_sync_wasm::*;
    }
'''

def once(t, a, b):
    assert t.count(a) == 1, (t.count(a), a[:100]); return t.replace(a, b)

def touch(p):
    rel = os.path.relpath(p, R)
    seen = set(open(TOUCHED, encoding='utf8').read().split('\n')) if os.path.exists(TOUCHED) else set()
    if rel not in seen:
        with open(TOUCHED, 'a', encoding='utf8') as f: f.write(rel + '\n')

def write(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf8').write(t); touch(p)

def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest() if os.path.exists(p) else None

def exports(text):
    names = sorted(surface.pub_names(text))
    return names

def postprocess(out):
    root = SY + '/🦀️.rs'
    out[root] = re.sub(r'(?m)^    use crate::fixtures::\{[^}]*\};\n', '', once_count(out[root], r'(?m)^    use crate::fixtures::\{[^}]*\};\n'))
    nroot = SY + '/🖥️native/🦀️.rs'; wroot = SY + '/🌐️wasm/🦀️.rs'
    lifted = SY + '/🖥️native/📥️lifted/🔄️sync/🦀️.rs'
    names = [n for n in exports(out[lifted])]
    runner = ['ArtifactActorRunnerHandle', 'ArtifactActorRunnerTicket', 'ArtifactActorTerminalJob', 'ArtifactActorTerminalReason', 'ArtifactActorTerminalState', 'ArtifactChannels', 'ArtifactHost']
    nt = out[nroot]
    nt = once(nt, 'pub mod lifted_sync;\n', 'pub mod lifted_sync;\n#[cfg(not(target_arch = "wasm32"))]\npub use native_actor::{' + ', '.join(runner) + '};\npub use lifted_sync::{' + ', '.join(names) + '};\n')
    out[nroot] = nt
    wt = out[wroot]
    wt = once(wt, 'pub mod wasm_actor;\n', 'pub mod wasm_actor;\n#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]\npub use wasm_actor::{ArtifactChannels, ArtifactHost};\n')
    out[wroot] = wt
    unit = SY + '/🧪️tests/🔬️unit/🦀️.rs'
    out[unit] = once(out[unit], 'mod actor_tests {\n', 'mod actor_tests {\n    use super::*;\n')
    for p in list(out):
        if not p.startswith(SY + '/') or p == SY + '/🧪️tests/🔬️unit/🦀️.rs': continue
        lines = out[p].split('\n')
        for i, l in enumerate(lines):
            if l.lstrip().startswith('//'): continue
            l = re.sub(r'(?<![\w:$])vcs::', 'semio_framework_os_vcs::', l)
            lines[i] = l.replace('semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot', 'semio_framework_os_store_text_io::ArtifactStoreSnapshot')
        out[p] = '\n'.join(lines)
    return out

def once_count(t, pattern):
    assert len(re.findall(pattern, t)) == 1, pattern
    return t

def kernel_toml(text):
    t = text
    t = once(t, '"dep:tokio-tungstenite", ', '"dep:semio-framework-os-store-sync", "dep:semio-framework-os-store-sync-native", "dep:semio-framework-os-store-sync-wasm", ')
    p = lambda k: os.path.relpath(C.pkg_abs(k, C.load()[0]), ns.K)
    a = '[target.\'cfg(not(all(target_arch = "wasm32", target_env = "p2")))\'.dependencies]\n'
    t = once(t, a, a + f'semio-framework-os-store-sync = {{ path = "{p("os-store-sync")}", optional = true }}\n')
    b = '[target.\'cfg(not(target_arch = "wasm32"))\'.dependencies]\n'
    t = once(t, b, b + f'semio-framework-os-store-sync-native = {{ path = "{p("os-store-sync-native")}", optional = true }}\n')
    t = once(t, 'tokio-tungstenite = { version = "0.26", optional = true, features = ["rustls-tls-webpki-roots"] }\n', '')
    c = '[target.\'cfg(all(target_arch = "wasm32", not(target_env = "p2")))\'.dependencies]\n'
    t = once(t, c, c + f'semio-framework-os-store-sync-wasm = {{ path = "{p("os-store-sync-wasm")}", optional = true }}\n')
    d = '[dev-dependencies]\n'
    t = once(t, d, '[target.\'cfg(not(target_arch = "wasm32"))\'.dev-dependencies]\n' + f'semio-framework-os-store-sync = {{ path = "{p("os-store-sync")}", features = ["testing"] }}\nsemio-framework-os-store-sync-native = {{ path = "{p("os-store-sync-native")}", features = ["testing"] }}\ntokio-tungstenite = {{ version = "0.26", features = ["rustls-tls-webpki-roots"] }}\n\n' + d)
    return t

def main():
    mode = sys.argv[1]
    w0 = ns.world()
    b = carve.solve(CRATES, relocate=[SY + '/🧪️tests/🔬️unit/🦀️.rs'], auto=True)
    assert not b.rw.rep.violations, b.rw.rep.violations[:3]
    out = postprocess(dict(b.out))
    texts, report = manifests.render(b, FEATURES, {})
    texts['os-store-sync-native'] = re.sub(r'(?m)^tokio = \{[^\n]*\}\n', lambda m: m.group(0), texts['os-store-sync-native'])
    s_root_live = open(S_ROOT, encoding='utf8').read()
    s_add = out[S_ROOT][len(open(S_ROOT, encoding='utf8').read()):] if out[S_ROOT].startswith(s_root_live) else None
    k_live = open(K_ROOT, encoding='utf8').read()
    k_new = once(k_live, K_MOUNT_OLD, K_MOUNT_NEW)
    ktoml_live = open(KERNEL_TOML, encoding='utf8').read()
    ktoml_new = kernel_toml(ktoml_live)
    plan = []
    for p, t in sorted(out.items()):
        if p in (S_ROOT, K_ROOT): continue
        old = open(p, encoding='utf8').read() if os.path.exists(p) else None
        if old != t: plan.append(('new' if old is None else 'edit', p))
    print(len(plan), 'file operations:', sum(1 for x in plan if x[0] == 'new'), 'new,', sum(1 for x in plan if x[0] == 'edit'), 'edited; S root add:', repr(s_add)[:140])
    for k, r in report.items(): print(k, 'first', r['first'], 'ext', r['ext'], 'dev', r['dev'], 'missing', r['missing'])
    if mode.startswith('overlay'):
        ov = dict(out); ov[K_ROOT] = k_new
        if s_add: ov[S_ROOT] = s_root_live.rstrip('\n') + '\n' + s_add.lstrip('\n')
        json.dump({'out': ov}, open(sys.argv[2], 'w'), ensure_ascii=False); print('overlay', len(ov)); return
    if mode != 'write': return
    assert sha(S_ROOT) == hashlib.sha256(s_root_live.encode()).hexdigest()
    for kind, p in plan: write(p, out[p])
    for k in CRATES: write(C.pkg_abs(k, C.load()[0]) + '/Cargo.toml', texts[k])
    if s_add:
        cur = open(S_ROOT, encoding='utf8').read()
        assert 'mod sync_tests;' not in cur
        write(S_ROOT, cur.rstrip('\n') + '\n' + s_add.lstrip('\n') if s_add.strip() else cur)
    write(K_ROOT, k_new); write(KERNEL_TOML, ktoml_new)
    print('applied')

if __name__ == '__main__': main()
