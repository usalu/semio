#!/usr/bin/env python3
# 📦️ Manifest pass of PLUGIN-WP2: [lib] path flip, first-party edges the carved code names (downward only), external crates it names (specs of the old facade manifest), feature check.
# usage: manifest.py report | write   (reads OUT, writes OUT/<package>/Cargo.toml)
import json, os, re, sys, collections
ARGS = list(sys.argv[1:])
from common import *
import model as M
from rs import blank, expand_use
import plan_tools as pt

OLD = json.load(open(SCRATCH + '/orig/plugin-Cargo.json'))
IDENT_KEY = {}
for k, v in list(OLD['dependencies'].items()) + list(OLD['dev-dependencies'].items()):
    ident = (v.get('package', k) if isinstance(v, dict) else k).replace('-', '_') if False else k.replace('-', '_')
    IDENT_KEY.setdefault(ident, k)
IDENT_KEY['wit_bindgen'] = 'wit-bindgen'
LIBKEY = {M.libname(k): k for k in M.CRATES_JSON}
KERNEL = 'semio-framework-os-kernel'
DERIVE = {'ToValue': ['semio-framework-value-derive'], 'FromValue': ['semio-framework-value-derive'], 'RetireOwned': [], 'DslRecord': ['semio-framework-dsl-record-derive', 'semio-framework-dsl-record'],
          'Serialize': ['serde'], 'Deserialize': ['serde'], 'dyn_enum': ['semio-framework-dispatch-macros'], 'dyn_enum_close': ['semio-framework-dispatch-macros']}

def pkg_abs(key):
    return PLUG + '/' + M.CRATES_JSON[key]['pkg'].split('/🔌️plugin/')[1]

def spec_text(spec, pkgdir):
    if isinstance(spec, str):
        return '"' + spec + '"'
    parts = []
    if spec.get('workspace'):
        parts.append('workspace = true')
    if 'path' in spec:
        target = os.path.normpath(os.path.join(PLUG + '/📦️packages/🦀️rust', spec['path']))
        parts.append('path = "' + os.path.relpath(target, pkgdir) + '"')
    if 'version' in spec:
        parts.append('version = "' + spec['version'] + '"')
    if 'default-features' in spec:
        parts.append('default-features = ' + ('true' if spec['default-features'] else 'false'))
    if spec.get('features'):
        parts.append('features = [' + ', '.join('"' + f + '"' for f in spec['features']) + ']')
    if spec.get('optional'):
        parts.append('optional = true')
    if 'package' in spec:
        parts.append('package = "' + spec['package'] + '"')
    return '{ ' + ', '.join(parts) + ' }'

def prod_text(text):
    """blanked text without cfg(test)-gated column-0 items and cfg(test) use statements"""
    clean = blank(text)
    lines = clean.split('\n')
    drop = set()
    tmp = WS + '/tmp-prod.rs'
    write(tmp, text)
    its, L, R = pt.outline(tmp)
    for it in its:
        attrs = ' '.join(R[max(0, it['s'] - 1):it['s'] + 3])
        head = R[it['s'] - 1:it['e']]
        a = ' '.join(l for l in R[it['s'] - 1:it['s'] + 4] if l.lstrip().startswith('#['))
        if re.search(r'cfg\((?:any\()?test', a) or '#[test]' in a:
            drop.update(range(it['s'] - 1, it['e']))
    for i, l in enumerate(lines):
        if i > 0 and re.search(r'cfg\((?:any\()?test', R[i - 1] if False else '') :
            pass
    out = [('' if i in drop else l) for i, l in enumerate(lines)]
    s = '\n'.join(out)
    s = re.sub(r'#\[cfg\((?:any\()?test[^\]]*\]\s*(?:pub(?:\([^)]*\))?\s+)?use[^;]*;', '', s)
    return s

def refs(key, texts):
    prod, tst = collections.Counter(), collections.Counter()
    ext_prod, ext_test = set(), set()
    for rel, text in texts:
        is_test_file = '/🧪️tests/' in rel or rel.startswith('🧪️')
        clean = blank(text)
        p = '' if is_test_file else prod_text(text)
        for src, bucket_first, bucket_ext in ((p, prod, ext_prod), (clean, tst, ext_test)):
            for m in re.finditer(r'(?<![A-Za-z0-9_])(semio_framework_plugin_[a-z0-9_]+)\b', src):
                k = LIBKEY.get(m.group(1))
                if k and k != key:
                    bucket_first[k] += 1
            idents = set(re.findall(r'(?<![A-Za-z0-9_$])([A-Za-z_][A-Za-z0-9_]*)\s*(?:::|!)', src)) | set(re.findall(r'\buse\s+([A-Za-z_][A-Za-z0-9_]*)', src))
            for i in idents:
                if i in IDENT_KEY:
                    bucket_ext.add(IDENT_KEY[i])
            for d, keys in DERIVE.items():
                if re.search(r'(?<![A-Za-z0-9_])' + d + r'\b', src):
                    bucket_ext.update(keys)
            for a in ('dsl', 'protocol', 'store', 'vcs'):
                if re.search(r'(?<![A-Za-z0-9_$])' + a + r'::', src):
                    bucket_ext.add(KERNEL)
    return prod, tst, ext_prod, ext_test

def sections(text):
    out, cur, name = [], [], None
    for l in text.split('\n'):
        m = re.match(r'^\[([^\]]+)\]\s*$', l)
        if m:
            out.append((name, cur)); name = m.group(1); cur = [l]
        else:
            cur.append(l)
    out.append((name, cur))
    return out

def dep_names(lines):
    return {re.match(r'^([A-Za-z0-9_\-]+)\s*=', l).group(1) for l in lines if re.match(r'^([A-Za-z0-9_\-]+)\s*=', l)}

def new_manifest(key, texts):
    pk = pkg_abs(key)
    cur = read(pk + '/Cargo.toml')
    prod, tst, ext_prod, ext_test = refs(key, texts)
    secs = sections(cur)
    have = {}
    for name, lines in secs:
        if name in ('dependencies', 'dev-dependencies'):
            have[name] = dep_names(lines)
    add_dep, add_dev = [], []
    for k in sorted(prod, key=lambda x: M.CRATES_JSON[x]['rank']):
        pkg = 'semio-framework-plugin-' + k
        if pkg not in have.get('dependencies', set()):
            add_dep.append(f'{pkg} = {{ path = "{os.path.relpath(pkg_abs(k), pk)}" }}')
    for k in sorted(tst, key=lambda x: M.CRATES_JSON[x]['rank']):
        pkg = 'semio-framework-plugin-' + k
        if k not in prod and pkg not in have.get('dependencies', set()) and pkg not in have.get('dev-dependencies', set()):
            add_dev.append(f'{pkg} = {{ path = "{os.path.relpath(pkg_abs(k), pk)}" }}')
    for k in sorted(ext_prod):
        if k not in have.get('dependencies', set()) and k in OLD['dependencies']:
            add_dep.append(f'{k} = {spec_text(OLD["dependencies"][k], pk)}')
    for k in sorted(ext_test - ext_prod):
        if k not in have.get('dependencies', set()) and k not in have.get('dev-dependencies', set()):
            spec = OLD['dev-dependencies'].get(k) or OLD['dependencies'].get(k)
            if spec:
                add_dev.append(f'{k} = {spec_text(spec, pk)}')
    miss, _ = features_report(key, texts)
    out = []
    for name, lines in secs:
        if name == 'features' and miss:
            while lines and lines[-1] == '':
                lines.pop()
            lines = lines + [f + ' = []' for f in miss] + ['']
        if name == 'lib':
            lines = [re.sub(r'path = "[^"]*"', 'path = "../../🦀️.rs"', l) if l.startswith('path') else l for l in lines]
        if name == 'dependencies' and add_dep:
            while lines and lines[-1] == '':
                lines.pop()
            lines = lines + add_dep + ['']
        if name == 'dev-dependencies' and add_dev:
            while lines and lines[-1] == '':
                lines.pop()
            lines = lines + add_dev + ['']
        out += lines
    if 'dev-dependencies' not in have and add_dev:
        out += ['[dev-dependencies]'] + add_dev + ['']
    return '\n'.join(out), add_dep, add_dev

def features_report(key, texts):
    used = set()
    for rel, text in texts:
        used |= set(re.findall(r'feature\s*=\s*"([^"]+)"', text))
    declared = set(M.CRATES_JSON[key]['features'])
    return sorted(used - declared), sorted(declared - used)

def crate_texts(key):
    import modtree
    import widen as W
    root = W.VIEW + '/' + M.ROOT[key]
    return [(os.path.relpath(f, W.VIEW), read(f)) for f in modtree.tree(root)]

def main(mode):
    import widen as W
    W.make_view()
    rows = []
    for key in M.WP2:
        texts = crate_texts(key)
        text, add_dep, add_dev = new_manifest(key, texts)
        miss, unused = features_report(key, texts)
        rows.append((key, add_dep, add_dev, miss, unused))
        if mode == 'write':
            write(WS + '/out/📦️pkg/' + key + '/Cargo.toml', text)
    for key, add_dep, add_dev, miss, unused in rows:
        print('==', key)
        for l in add_dep:
            print('   +dep', l[:140])
        for l in add_dev:
            print('   +dev', l[:140])
        print('   features used but not declared:', miss, '| declared but never named:', unused)

if __name__ == '__main__':
    main(ARGS[0] if ARGS else 'report')
