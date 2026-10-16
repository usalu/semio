"""📦 Manifest generation for the fixtures crate and the companion test crates: dependencies from the ident scan, specs copied from the kernel manifest."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, common as C, nsboot
from okrs import tokenize
KJ = json.load(open(L.GEN + '/okwp8-kernel-cargo.json'))
KT = list(KJ.values())[0]
K_DIR = L.R + '/' + L.K_REL
CAT = nsboot.boot()
def kernel_specs():
    specs = collections.OrderedDict()   # name -> dict(spec, tables:set)
    def add(name, spec, table):
        if isinstance(spec, str): spec = {'version': spec}
        cur = specs.setdefault(name, dict(spec={}, tables=set(), dev_features=set()))
        cur['tables'].add(table)
        for k, v in spec.items():
            if k == 'features':
                if table.startswith('dev'): cur['dev_features'] |= set(v)
                else: cur['spec']['features'] = sorted(set(cur['spec'].get('features', [])) | set(v))
            elif k == 'workspace':
                cur['spec'].setdefault('workspace', v)
            elif k not in cur['spec'] or table.startswith('dep'): cur['spec'].setdefault(k, v)
    for n, s in KT['dependencies'].items(): add(n, s, 'dep')
    for tk, tv in KT.get('target', {}).items():
        for n, s in tv.get('dependencies', {}).items(): add(n, s, 'dep:' + tk)
        for n, s in tv.get('dev-dependencies', {}).items(): add(n, s, 'dev:' + tk)
    for n, s in KT['dev-dependencies'].items(): add(n, s, 'dev')
    return specs
KS = kernel_specs()
def ident_map():
    m = {}
    for name, c in CAT.items():
        if not name.startswith('semio-') and not name.startswith('dsl'): continue
        m[c['lib_name'] or name.replace('-', '_')] = name
    for name, s in KS.items():
        spec = s['spec']
        key = name.replace('-', '_')
        if 'package' in spec:
            m[key] = name
        else:
            m.setdefault(key, name)
    # extern crate idents that differ from package names
    m['protocol'] = 'semio-framework-replication'
    m['pack'] = 'semio-framework-pack'
    return m
IM = ident_map()
IM['semio_framework_os_store_testing_doubles'] = 'semio-framework-os-store-testing-doubles'
NEW_CRATES = {'semio-framework-os-store-testing-doubles': L.R + '/' + L.S_REL + '/🧪️testing/🎭️doubles/📦️packages/🦀️rust'}
THIRD = {n for n, s in KS.items() if 'path' not in s['spec'] and not s['spec'].get('workspace')}
def idents(texts):
    found = set()
    for t in texts:
        toks = [x for x in tokenize(t) if x[0] not in ('ws', 'comment')]
        for i, x in enumerate(toks):
            if x[0] == 'id' and x[1] in IM and i + 2 < len(toks) and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
                if i > 0 and toks[i - 1][1] == ':' and i > 1 and toks[i - 2][1] == ':' and False: continue
                found.add(x[1])
            elif x[0] == 'id' and x[1] in IM and i > 0 and toks[i - 1][1] in ('[', '(', ',') and i + 1 < len(toks) and toks[i + 1][1] in (',', ')', ']'):
                pass
    return found
def render_spec(name, pkg_dir, extra_features=(), force_plain=False):
    if name in NEW_CRATES: return '{ path = "%s" }' % os.path.relpath(NEW_CRATES[name], pkg_dir)
    ks = KS.get(name)
    spec = dict(ks['spec']) if ks else None
    if spec is None:
        c = CAT.get(name)
        if c is None: raise SystemExit('unknown crate ' + name)
        spec = {'path': os.path.relpath(L.R + '/' + c['dir'], pkg_dir)}
        return '{ path = "%s" }' % spec['path']
    parts = []
    if ks and name in DEV_FEATURE_OK: spec['features'] = sorted(set(spec.get('features', [])) | ks['dev_features'])
    if spec.get('workspace'):
        parts.append('workspace = true'); spec.pop('path', None); spec.pop('version', None); spec.pop('package', None)
    if 'path' in spec:
        absd = os.path.normpath(K_DIR + '/' + spec['path'])
        parts.append('path = "%s"' % os.path.relpath(absd, pkg_dir))
    if 'version' in spec: parts.append('version = "%s"' % spec['version'])
    if 'package' in spec: parts.append('package = "%s"' % spec['package'])
    if spec.get('default-features') is False: parts.append('default-features = false')
    feats = sorted(set(spec.get('features', [])) | set(extra_features))
    if feats: parts.append('features = [%s]' % ', '.join('"%s"' % f for f in feats))
    if len(parts) == 1 and parts[0].startswith('version = '): return parts[0].split(' = ', 1)[1]
    return '{ ' + ', '.join(parts) + ' }'
DEV_FEATURE_OK = {'semio-framework-value', 'semio-framework-job', 'semio-framework-job-worker', 'semio-framework-os-store-cursor', 'semio-framework-os-store-ledger', 'semio-framework-os-store-replay'}
def scan_manifest_header(path):
    s = L.read(path)
    i = s.index('\n[dependencies]')
    return s[:i + 1]
def build(path_manifest, pkg_dir, texts, extra_names=(), native_only=(), notp2_only=(), extra_features=None, dev_names=(), header=None):
    header = header if header is not None else scan_manifest_header(path_manifest)
    used = idents(texts)
    names = set(IM[i] for i in used) | set(extra_names)
    names.discard(None)
    self_name = re.search(r'^name = "([^"]+)"', header, re.M).group(1)
    names.discard(self_name)
    extra_features = extra_features or {}
    lines = []; native = []; notp2 = []; dev = []
    for n in sorted(names):
        spec = render_spec(n, pkg_dir, extra_features.get(n, ()))
        line = '%s = %s' % (n, spec)
        if n in native_only: native.append(line)
        elif n in notp2_only: notp2.append(line)
        elif n in dev_names: dev.append(line)
        else: lines.append(line)
    out = header + '\n[dependencies]\n' + '\n'.join(lines) + '\n'
    if notp2: out += '\n[target.\'cfg(not(all(target_arch = "wasm32", target_env = "p2")))\'.dependencies]\n' + '\n'.join(notp2) + '\n'
    if native: out += '\n[target.\'cfg(not(target_arch = "wasm32"))\'.dependencies]\n' + '\n'.join(native) + '\n'
    out += '\n[dev-dependencies]\n' + ('\n'.join(dev) + '\n' if dev else '')
    return out, names
if __name__ == '__main__':
    print(len(IM), len(KS), sorted(THIRD)[:20])
    t = "use semio_framework_value::X; let a = serde_json::from_str(1); semio_framework_os_store_ledger::A; protocol::B; tokio::sync::mpsc;"
    print(idents([t]))
