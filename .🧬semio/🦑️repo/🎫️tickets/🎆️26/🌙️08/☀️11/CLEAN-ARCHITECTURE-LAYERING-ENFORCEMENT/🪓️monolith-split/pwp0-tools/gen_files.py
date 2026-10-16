# 🏗️ Writes the skeleton crates of the plugin partition: manifest, project file, stub library root, test stub. Never overwrites an existing file.
# usage: gen_files.py dry|write
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C

S = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
OLD = json.load(open(S + '/orig/plugin-Cargo.json'))
UNIT = json.load(open(C.GEN + '/pwp0-unitdeps.json'))
HOSTS = json.load(open(C.GEN + '/pwp0-testhosts.json'))
WASM_P2 = 'cfg(all(target_arch = "wasm32", target_env = "p2"))'
KERNEL = 'semio-framework-os-kernel'
IDENT_KEY = {}
for k, v in OLD['dependencies'].items(): IDENT_KEY[k.replace('-', '_')] = k
for k, v in OLD['dev-dependencies'].items(): IDENT_KEY.setdefault(k.replace('-', '_'), k)
IDENT_KEY['wit_bindgen'] = 'wit-bindgen'
ALIASES = ('dsl', 'protocol', 'store', 'vcs')
DERIVE = {'ToValue': ['semio-framework-value-derive', 'semio-framework-value'], 'FromValue': ['semio-framework-value-derive', 'semio-framework-value'], 'DslRecord': ['semio-framework-dsl-record-derive', 'semio-framework-dsl-record', KERNEL],
          'dyn_enum': ['semio-framework-dispatch-macros'], 'dyn_enum_close': ['semio-framework-dispatch-macros'], 'Serialize': ['serde'], 'Deserialize': ['serde']}
DEV_ONLY = {'wasmtime', 'semio-framework-async-macros'}

def ext_keys(c):
    src = UNIT['lower'].get(c['key']) or UNIT['instance'].get(c['key']) or {'toks': []}
    keys = set(); aliases = set()
    for t in src['toks']:
        if t in IDENT_KEY and IDENT_KEY[t] not in DEV_ONLY: keys.add(IDENT_KEY[t])
        if t in ALIASES: keys.add(KERNEL); aliases.add(t)
        for d in DERIVE.get(t, []): keys.add(d)
    if c['key'] == 'abi': keys |= {'wit-bindgen', 'serde', 'serde_json'}
    if c['key'] == 'builder': keys.add('semio-framework-canvas-assets')
    return sorted(keys), sorted(aliases)

def spec_text(spec, pkg_abs, optional=False, extra_features=None):
    if isinstance(spec, str): return '"' + spec + '"'
    parts = []
    if spec.get('workspace'): parts.append('workspace = true')
    if 'path' in spec:
        target = os.path.normpath(os.path.join(C.PKG, spec['path']))
        parts.append('path = "' + os.path.relpath(target, pkg_abs) + '"')
    if 'version' in spec: parts.append('version = "' + spec['version'] + '"')
    if 'default-features' in spec: parts.append('default-features = ' + ('true' if spec['default-features'] else 'false'))
    feats = list(spec.get('features', [])) + [f for f in (extra_features or []) if f not in spec.get('features', [])]
    if feats: parts.append('features = [' + ', '.join('"' + f + '"' for f in feats) + ']')
    if spec.get('optional') or optional: parts.append('optional = true')
    if 'package' in spec: parts.append('package = "' + spec['package'] + '"')
    return '{ ' + ', '.join(parts) + ' }'

def dev_keys(c):
    if c['kind'] == 'companion': return sorted(set(OLD['dev-dependencies'].keys()))
    if c['kind'] != 'lower': return []
    text = ''
    for r in HOSTS:
        if r['host'] == c['key'] and r['how'] == 'dir' and r['file'].endswith('.rs'):
            try: text += open(C.PLUG + '/' + r['file'], encoding='utf-8').read()
            except Exception: pass
    out = set()
    for k in OLD['dev-dependencies']:
        ident = k.replace('-', '_')
        if re.search(r'(?<![\w])' + ident + r'\b', text): out.add(k)
    return sorted(out)

def manifest(crates, key):
    c = crates[key]
    pkg = C.R + '/' + c['pkg']
    depth = len(c['pkg'].split('/'))
    L = ['[package]', 'workspace = "' + '/'.join(['..'] * depth) + '"', 'name = "' + c['name'] + '"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true', 'description = "' + c['desc'].replace('"', "'") + '"']
    if c['kind'] in ('companion', 'support'): L.append('publish = false')
    L += ['', '[package.metadata.semio]', 'role = "framework"', 'id = "' + c['id'] + '"', '', '[package.metadata.semio.rust-source-boundary]', 'maxSourceBytes = 262144', 'maxSourceFiles = 32', 'maxActiveLines = 4000',
          '', '[lints]', 'workspace = true', '', '[lib]', 'path = "' + ('../../🦀️.rs' if c['class'] == 'A' else '🦀️.rs') + '"', '']
    if c['features']:
        L.append('[features]')
        for f in c['features']:
            L.append(f + ' = ' + ('["dep:semio-framework-canvas-assets"]' if f == 'canvas-runtime-assets' else '[]'))
        L.append('')
    firsts = [(d, crates[d]) for d in c['deps']]
    keys, aliases = ext_keys(c)
    L.append('[dependencies]')
    for d, dc in firsts:
        L.append(dc['name'] + ' = { path = "' + os.path.relpath(C.R + '/' + dc['pkg'], pkg) + '" }')
    for k in keys:
        spec = OLD['dependencies'].get(k)
        if spec is None: continue
        L.append(k + ' = ' + spec_text(spec, pkg, optional=(k == 'semio-framework-canvas-assets')))
    L.append('')
    dk = dev_keys(c)
    L.append('[dev-dependencies]')
    for k in dk:
        spec = OLD['dev-dependencies'][k]
        L.append(k + ' = ' + spec_text(spec, pkg))
    if c['kind'] == 'lower' and KERNEL in keys and KERNEL not in dk and dk:
        L.append(KERNEL + ' = ' + spec_text(OLD['dev-dependencies'][KERNEL], pkg))
    L.append('')
    return '\n'.join(L).rstrip('\n') + '\n', aliases

def lib_root(crates, key, aliases):
    c = crates[key]
    lines = [f'//! {c["emoji"]} {c["desc"]}.', '//!']
    if c['class'] == 'A':
        lines.append(f'//! 🧭️ WP{c["wp"]} skeleton of `{C.FACADE}`: the sources move in with the work package.')
    else:
        lines.append(f'//! 🧭️ WP{c["wp"]} skeleton of `{C.FACADE}`: a placeholder root. `../../🦀️.rs` is the module file of `{c["root"]}`, still mounted by the monolith; when the carve lands, point `[lib] path` at it and delete this file.')
    if aliases: lines += [''] + [f'extern crate semio_framework_os_kernel as {a};' for a in aliases]
    if c['class'] == 'A' and c['kind'] == 'lower' and not os.path.isdir(C.PLUG + '/' + c['root'] + '/🧪️tests'):
        lines += ['', '#[cfg(test)]', '#[path = "🧪️tests/🦀️.rs"]', 'mod tests;']
    return '\n'.join(lines).rstrip('\n') + '\n'

def test_stub(c):
    return f'//! {c["emoji"]} Unit tests of {c["key"]}; the work package moves the tests of its sources here.\n'

def project_json(c):
    return json.dumps({'name': c['project'], 'tags': ['scope:framework', 'language:rust']}, ensure_ascii=False, indent=2) + '\n'

def main():
    mode = sys.argv[1]
    crates, order = C.build()
    written = 0; skipped = 0
    for key in order:
        c = crates[key]
        text, aliases = manifest(crates, key)
        pkg = C.R + '/' + c['pkg']
        root_file = (C.R + '/' + c['dir'] + '/🦀️.rs') if c['class'] == 'A' else (pkg + '/🦀️.rs')
        files = [(pkg + '/Cargo.toml', text), (pkg + '/📋️project.json', project_json(c)), (root_file, lib_root(crates, key, aliases))]
        if c['class'] == 'A' and c['kind'] == 'lower' and not os.path.isdir(C.PLUG + '/' + c['root'] + '/🧪️tests'):
            files.append((C.R + '/' + c['dir'] + '/🧪️tests/🦀️.rs', test_stub(c)))
        for path, content in files:
            if os.path.exists(path): skipped += 1; continue
            if len(path.encode()) > 240: raise SystemExit('path too long: ' + path)
            if mode == 'write':
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, 'w', encoding='utf-8').write(content)
            written += 1
    print(mode, 'files', written, 'skipped existing', skipped)

if __name__ == '__main__': main()
