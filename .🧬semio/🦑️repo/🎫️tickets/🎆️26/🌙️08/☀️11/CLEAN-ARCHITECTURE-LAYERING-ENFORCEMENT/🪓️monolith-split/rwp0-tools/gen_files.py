# 🏗️ Writes the 88 skeleton crates of the wgpu renderer partition: manifest, project file, library root, test stub. Never overwrites an existing file.
# usage: gen_files.py dry|write
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C

S = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
OLD = json.load(open(S + '/orig/Cargo.json'))
OLD_PKG = C.T + '/📦️packages/🦀️rust'
NOT_WASI = 'cfg(not(target_os = "wasi"))'
NATIVE = 'cfg(not(target_arch = "wasm32"))'
BROWSER = 'cfg(all(target_arch = "wasm32", target_os = "unknown"))'
LINUX = 'cfg(target_os = "linux")'

def old_entries():
    table = {}
    def add(key, kind, spec): table.setdefault(key, []).append((kind, spec))
    for k, v in OLD['dependencies'].items(): add(k, 'plain', v)
    for tk, tv in OLD['target'].items():
        kind = {NOT_WASI: 'notwasi', NATIVE: 'native', BROWSER: 'browser', LINUX: 'linux'}.get(tk)
        for k, v in tv.get('dependencies', {}).items(): add(k, kind, v)
        for k, v in tv.get('dev-dependencies', {}).items(): add(k, 'dev-' + {NATIVE: 'native'}.get(tk, tk), v)
    for k, v in OLD['dev-dependencies'].items(): add(k, 'dev', v)
    return table
OLD_ENTRIES = old_entries()

def spec_text(spec, pkg):
    if isinstance(spec, str): return '"' + spec + '"'
    parts = []
    if spec.get('workspace'): parts.append('workspace = true')
    if 'path' in spec:
        target = os.path.normpath(os.path.join(OLD_PKG, spec['path']))
        parts.append('path = "' + os.path.relpath(target, pkg) + '"')
    if 'version' in spec: parts.append('version = "' + spec['version'] + '"')
    if 'default-features' in spec: parts.append('default-features = ' + ('true' if spec['default-features'] else 'false'))
    if 'features' in spec: parts.append('features = [' + ', '.join('"' + f + '"' for f in spec['features']) + ']')
    if 'package' in spec: parts.append('package = "' + spec['package'] + '"')
    return '{ ' + ', '.join(parts) + ' }'

def ext_lines(crate, pkg):
    plain, native, browser, linux, dev, devnative = {}, {}, {}, {}, {}, {}
    cfg = crate['cfg']
    missing = []
    for key in crate['ext']:
        entries = OLD_ENTRIES.get(key)
        if not entries: missing.append(key); continue
        kinds = {k: s for k, s in entries}
        base = kinds.get('plain') or kinds.get('notwasi')
        if cfg == 'both':
            if base: plain[key] = base
            if 'native' in kinds: native[key] = kinds['native']
            if 'browser' in kinds: browser[key] = kinds['browser']
        elif cfg == 'native':
            spec = kinds.get('native') or base
            if spec: native[key] = spec
        elif cfg == 'wasm':
            spec = kinds.get('browser') or base
            if spec: browser[key] = spec
        if 'linux' in kinds and cfg in ('both', 'native'): linux[key] = kinds['linux']
        if 'dev' in kinds: dev[key] = kinds['dev']
        if 'dev-native' in kinds: devnative[key] = kinds['dev-native']
    return plain, native, browser, linux, dev, devnative, missing

def dev_for(crate):
    out = {}
    ext = set(crate['ext'])
    if crate['kind'] in ('lib', 'shell') and not crate.get('tests'): return out
    if 'semio-framework-job' in ext: out['semio-framework-job'] = OLD['dev-dependencies']['semio-framework-job']
    if 'semio-framework-os-kernel' in ext: out['semio-framework-os-kernel'] = OLD['dev-dependencies']['semio-framework-os-kernel']
    if 'ui_wgpu' in ext: out['ui_wgpu'] = OLD['dev-dependencies']['ui_wgpu']
    return out

def manifest(crates, key):
    c = crates[key]
    pkg = C.pkg_dir(key)
    depth = len(C.rel_root(pkg).split('/'))
    L = ['[package]', 'workspace = "' + '/'.join(['..'] * depth) + '"', 'name = "' + c['name'] + '"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true', 'description = "' + describe(key) + '"']
    if c['kind'] in ('companion', 'shell-tests', 'support'): L.append('publish = false')
    L += ['', '[package.metadata.semio]', 'role = "framework"', 'id = "' + c['id'] + '"', '', '[package.metadata.semio.rust-source-boundary]', 'maxSourceBytes = 262144', 'maxSourceFiles = 32', 'maxActiveLines = 4000',
          '', '[lints]', 'workspace = true', '', '[lib]', 'path = "../../🦀️.rs"', '']
    plain, native, browser, linux, dev, devnative, missing = ext_lines(c, pkg)
    first, gate_native, gate_browser = {}, {}, {}
    for d in c['deps']:
        line = '{ path = "' + os.path.relpath(C.pkg_dir(d), pkg) + '" }'
        if c['cfg'] == 'native': gate_native[C.pkg_name(d)] = line
        elif c['cfg'] == 'wasm': gate_browser[C.pkg_name(d)] = line
        elif d in C.NATIVE_ONLY: gate_native[C.pkg_name(d)] = line
        else: first[C.pkg_name(d)] = line
    def emit(header, firstparty, external):
        rows = sorted(firstparty.items()) + [(k, spec_text(v, pkg)) for k, v in sorted(external.items())]
        if not rows: return []
        return [header] + [k + ' = ' + v for k, v in rows] + ['']
    L += emit('[dependencies]', first, plain) or ['[dependencies]', '']
    L += emit("[target.'" + NATIVE + "'.dependencies]", gate_native, native)
    L += emit("[target.'" + BROWSER + "'.dependencies]", gate_browser, browser)
    L += emit("[target.'" + LINUX + "'.dependencies]", {}, linux)
    d = dev_for(c)
    devs = [(k, spec_text(v, pkg)) for k, v in sorted(d.items())]
    L += ['[dev-dependencies]'] + [k + ' = ' + v for k, v in devs] + ['']
    if devnative and c['key'] == 'native-platform':
        L += ["[target.'" + NATIVE + "'.dev-dependencies]"] + [k + ' = ' + spec_text(v, pkg) for k, v in sorted(devnative.items())] + ['']
    return '\n'.join(L).rstrip('\n') + '\n', missing

def describe(key): return C.DESC[key] if key in C.DESC else 'Companion test bin of the wgpu renderer: ' + C.BIN_THEMES[key.split('-')[2]] + ' (' + key.split('-')[-1] + ')'

def lib_root(crates, key):
    c = crates[key]
    emoji = C.EMOJI[key] + C.FE0F
    cfg = {'both': 'native and wasm32', 'native': 'native only', 'wasm': 'wasm32-unknown-unknown only', 'all': 'every target including wasm32-wasip2, no dependencies'}[c['cfg']]
    lines = [f'//! {emoji} {describe(key)}.', '//!', f'//! 🧭️ WP{c["wp"]} skeleton of `{C.TOP}`: {cfg}; its sources move in with the work package.']
    if c['kind'] in ('lib', 'shell'):
        lines += ['', '#[cfg(test)]', '#[path = "🧪️tests/🦀️.rs"]', 'mod tests;']
    return '\n'.join(lines) + '\n'

def test_stub(key):
    return f'//! {C.EMOJI[key] + C.FE0F} Unit tests of {key}; the work package moves the tests of its sources here.\n'

def project_json(key):
    return json.dumps({'name': C.project_name(key), 'tags': ['scope:framework', 'language:rust']}, ensure_ascii=False, indent=2) + '\n'

def main():
    mode = sys.argv[1]
    crates = C.build()
    written = 0; skipped = 0; allmissing = {}
    for key in C.ALL_KEYS:
        c = crates[key]
        text, missing = manifest(crates, key)
        if missing: allmissing[key] = missing
        files = [(C.pkg_dir(key) + '/Cargo.toml', text), (C.pkg_dir(key) + '/📋️project.json', project_json(key)), (C.crate_dir(key) + '/🦀️.rs', lib_root(crates, key))]
        if c['kind'] in ('lib', 'shell'): files.append((C.crate_dir(key) + '/🧪️tests/🦀️.rs', test_stub(key)))
        for path, content in files:
            if os.path.exists(path): skipped += 1; continue
            if mode == 'write':
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, 'w', encoding='utf-8').write(content)
            written += 1
    print(mode, 'files', written, 'skipped existing', skipped, 'unknown external keys', allmissing)

if __name__ == '__main__': main()
