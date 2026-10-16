# 🏗️ Writes the skeleton crates of the OS kernel partition: manifest, project file and (only where the node has none) a stub library root. Never overwrites an existing file.
# usage: gen_files.py dry|write [key ...]
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import okcrates as C

FEATURES = {
    'os-spr': ['[features]', 'protocol-laws = []', 'mutation-testing = []', ''],
    'os-store-durable-group': ['[features]', 'testing = []', ''],
}

def rebase(path_from_k, pkg):
    target = os.path.normpath(os.path.join(C.K, path_from_k))
    return os.path.relpath(target, pkg)

def spec_text(spec, pkg):
    if isinstance(spec, str): return '"' + spec + '"'
    parts = []
    if spec.get('workspace'): parts.append('workspace = true')
    if 'path' in spec: parts.append('path = "' + rebase(spec['path'], pkg) + '"')
    if 'version' in spec: parts.append('version = "' + spec['version'] + '"')
    if 'default-features' in spec: parts.append('default-features = ' + ('true' if spec['default-features'] else 'false'))
    if 'features' in spec: parts.append('features = [' + ', '.join('"' + f + '"' for f in spec['features']) + ']')
    if 'package' in spec: parts.append('package = "' + spec['package'] + '"')
    return '{ ' + ', '.join(parts) + ' }'

def ext_tables(crate, entries_by_key):
    plain, notp2, native, browser, missing = {}, {}, {}, {}, []
    for key in crate['ext']:
        entries = entries_by_key.get(key)
        if not entries: missing.append(key); continue
        for kind, spec in entries:
            if kind == 'dev': continue
            target = {'plain': plain, 'notp2': notp2, 'native': native, 'browser': browser}[kind]
            target[key] = spec
    return plain, notp2, native, browser, missing

def manifest(crates, key, model, entries_by_key):
    c = crates[key]
    pkg = C.pkg_abs(key, model)
    depth = C.ups(key, model)
    L = ['[package]', 'workspace = "' + '/'.join(['..'] * depth) + '"', 'name = "' + c['name'] + '"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true', 'description = "' + C.DESC[key] + '"']
    if c['kind'] == 'companion': L.append('publish = false')
    L += ['', '[package.metadata.semio]', 'role = "framework"', 'id = "' + c['id'] + '"', '', '[package.metadata.semio.rust-source-boundary]', 'maxSourceBytes = 262144', 'maxSourceFiles = 32', 'maxActiveLines = 4000',
          '', '[lints]', 'workspace = true', '', '[lib]', 'path = "../../🦀️.rs"', '']
    if c.get('features'): L += ['[features]'] + c['features'] + ['']
    else: L += FEATURES.get(key, [])
    plain, notp2, native, browser, missing = ext_tables(c, entries_by_key)
    first, f_notp2, f_native, f_browser = {}, {}, {}, {}
    for d in c['deps']:
        feats = c.get('dep_features', {}).get(d)
        line = '{ path = "' + os.path.relpath(C.pkg_abs(d, model), pkg) + '"' + (', features = [' + ', '.join('"' + f + '"' for f in feats) + ']' if feats else '') + ' }'
        if d in C.NATIVE_ONLY: f_native[C.crate_name(d)] = line
        elif d in C.NOT_P2: f_notp2[C.crate_name(d)] = line
        elif d in C.WASM_ONLY: f_browser[C.crate_name(d)] = line
        else: first[C.crate_name(d)] = line
    def emit(header, firstparty, external):
        rows = sorted(firstparty.items()) + [(k, spec_text({kk: vv for kk, vv in v.items() if kk != 'optional'} if isinstance(v, dict) else v, pkg)) for k, v in sorted(external.items())]
        if not rows: return []
        return [header] + [k + ' = ' + v for k, v in rows] + ['']
    L += emit('[dependencies]', first, plain) or ['[dependencies]', '']
    L += emit("[target.'" + C.NOTP2 + "'.dependencies]", f_notp2, notp2)
    L += emit("[target.'" + C.NATIVE + "'.dependencies]", f_native, native)
    L += emit("[target.'" + C.BROWSER + "'.dependencies]", f_browser, browser)
    dev_rows = []
    for d in c.get('dev', []):
        if d in entries_by_key:
            kinds = {k2: v for k2, v in entries_by_key[d]}
            spec = kinds.get('dev') or kinds.get('plain')
            dev_rows.append(d + ' = ' + spec_text({kk: vv for kk, vv in spec.items() if kk != 'optional'} if isinstance(spec, dict) else spec, pkg))
        elif d == 'semio-framework-os-kernel':
            dev_rows.append(d + ' = { path = "' + os.path.relpath(C.K, pkg) + '", features = ["protocol-laws", "mutation-testing"] }')
        else:
            dev_rows.append(d + ' = { path = "' + os.path.relpath(C.pkg_abs(d, model), pkg) + '" }' if d.startswith('os-') else '# unknown dev dependency ' + d)
    L += ['[dev-dependencies]'] + dev_rows + ['']
    return '\n'.join(L).rstrip('\n') + '\n', missing

def emoji_of(node_rel):
    name = os.path.basename(node_rel)
    return name.split(C.FE0F)[0] if C.FE0F in name else name[0]

def lib_root(crates, key, model):
    c = crates[key]
    emoji = emoji_of(c['node']) + C.FE0F
    cfg = {'all': 'every target', 'notp2': 'native and wasm32-unknown-unknown, not wasm32-wasip2', 'native': 'native only', 'wasm': 'wasm32-unknown-unknown only'}[c['cfg']]
    lines = [f'//! {emoji} {C.DESC[key]}.', '//!', f'//! 🧭️ Skeleton of `{C.TOP}` (WP {c["wp"]}): {cfg}; its sources move in with the work package.']
    if c['kind'] == 'lib':
        lines += ['', '#[cfg(test)]', '#[path = "🧪️tests/🔬️unit/🦀️.rs"]', 'mod unit_tests;']
    return '\n'.join(lines) + '\n'

def test_stub(crates, key):
    emoji = '🔬'
    return f'//! {emoji}{C.FE0F} Unit tests of {key}; the work package moves the tests of its sources here.\n'

def project_json(key):
    return json.dumps({'name': C.project_name(key), 'tags': ['scope:framework', 'language:rust']}, ensure_ascii=False, indent=2) + '\n'

FORCE = '--force' in sys.argv
def main():
    mode = sys.argv[1]
    only = set(a for a in sys.argv[2:] if not a.startswith('--'))
    crates, model, cargo = C.build()
    entries = C.old_entries(cargo)
    written, skipped, allmissing, rootsexisting = 0, 0, {}, []
    for key in C.ALL_KEYS:
        if only and key not in only: continue
        c = crates[key]
        text, missing = manifest(crates, key, model, entries)
        if missing: allmissing[key] = missing
        node = C.node_abs(key, model)
        files = [(C.pkg_abs(key, model) + '/Cargo.toml', text), (C.pkg_abs(key, model) + '/📋️project.json', project_json(key))]
        if os.path.exists(node + '/🦀️.rs'): rootsexisting.append(key)
        else:
            files.append((node + '/🦀️.rs', lib_root(crates, key, model)))
            if c['kind'] == 'lib': files.append((node + '/🧪️tests/🔬️unit/🦀️.rs', test_stub(crates, key)))
        for path, content in files:
            if os.path.exists(path) and not (FORCE and path.endswith('/Cargo.toml') and c.get('real')): skipped += 1; continue
            if mode == 'write':
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, 'w', encoding='utf-8').write(content)
            written += 1
    print(mode, 'files', written, 'skipped existing', skipped, 'existing lib roots kept', len(rootsexisting), 'unknown external keys', allmissing)

if __name__ == '__main__': main()
