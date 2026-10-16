"""🏁 Finalize the stage: facade crate, manifests for every crate, crate list for the oracle, dependency DAG check."""
import json, os, re, sys
from lib import *
from rs import strip, idents
from compose import Composer, extern_alias_lines
from rewrite import FileCtx, rewrite_uses, find_uses
import facade, manifest
from plan import CRATES, BY_LIB, PKG_PREFIX
from manifest import used_libs, ext_idents, ext_lines, rel, DEV_ONLY, EXT_PATH

FACADE_PKG = 'semio-framework-os-kernel-db'

def build_facade(c):
    root, fam, files = facade.scan_hub()
    text = facade.render(c, root, fam)
    ext = extern_alias_lines(strip(text))
    lines = text.split('\n')
    k = 0
    while k < len(lines) and lines[k].startswith('//!'):
        k += 1
    lines = lines[:k] + ext + lines[k:]
    write(f'{STAGE}/🦀️.rs', '\n'.join(lines) + '\n//#region 🧪️Tests\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n//#endregion 🧪️Tests\n')
    t = read(f'{PRISTINE}/🧪️tests/🔬️unit/🦀️.rs')
    ctx = FileCtx(c.resolver, 'db', set())
    ids = idents(strip(t))
    rootnames = {n for n in ids if n in c.resolver.scope.root}
    t2 = rewrite_uses(t, ctx, ids, rootnames)
    t2 = c.rewrite_code(t2, ctx, 'db', 'db')
    from scope import FACADE_FAMILY
    def fam_repl(m):
        fam_name, name = m.group(1), m.group(2)
        module = FACADE_FAMILY[fam_name]
        lib = c.resolver.lib(module, name) if c.resolver.split(module) else module
        return f'{lib}::{name}'
    st2 = strip(t2)
    edits = [(m.start(), m.end(), fam_repl(m)) for m in re.finditer(r'(?<![A-Za-z0-9_:])(' + '|'.join(sorted(FACADE_FAMILY)) + r')::([A-Za-z_][A-Za-z0-9_]*)', st2)]
    for s_, e_, r_ in sorted(edits, reverse=True):
        t2 = t2[:s_] + r_ + t2[e_:]
    bound = set()
    for u in find_uses(t2):
        from lib import parse_use
        for p_, a_, g_ in parse_use(u['tree']):
            if not g_:
                bound.add(a_ or p_[-1])
    local = {it.name for it in __import__('model').parse_items(t2)[0] if it.name}
    ids2 = idents(strip(t2))
    need = sorted(n for n in ids2 if n in c.resolver.scope.root and n not in bound and n not in local)
    extra = [f'use {"::".join(c.resolver.root(n))};' for n in need if c.resolver.root(n)[0] != 'db']
    if extra:
        lines2 = t2.split('\n')
        k2 = 0
        while k2 < len(lines2) and lines2[k2].startswith('use super::'):
            k2 += 1
        t2 = '\n'.join(lines2[:k2] + extra + lines2[k2:])
    write(f'{STAGE}/🧪️tests/🔬️unit/🦀️.rs', t2)
    old = f'{STAGE}/📦️packages/🦀️rust/🦀️.rs'
    if os.path.exists(old):
        os.remove(old)
    return root, fam, files

def facade_manifest(libs_used, dev_used, ext_n, ext_dev, native):
    ups = '/'.join(['..'] * 7)
    pkg_dir = f'{DB_REL}/📦️packages/🦀️rust'
    L = ['[package]', f'workspace = "{ups}"', f'name = "{FACADE_PKG}"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true',
         'description = "Db facade: the stable db surface the hub consumes, re-exported from the producer crates"', '', '[package.metadata.semio]', 'role = "product"', 'id = "os-db"', '',
         '[package.metadata.semio.rust-source-boundary]', 'maxSourceBytes = 262144', 'maxSourceFiles = 32', 'maxActiveLines = 4000', '', '[lints]', 'workspace = true', '', '[lib]', 'name = "db"', 'path = "../../🦀️.rs"', '',
         '[features]', f'claimed-backend-laws = ["{BY_LIB["db_io_core"].package}/claimed-backend-laws"]', '', '[dependencies]']
    for lib in sorted(libs_used):
        d = BY_LIB[lib]
        L.append(f'{d.package} = {{ path = "{rel(pkg_dir, DB_REL + "/" + d.pkg_dir)}" }}')
    L.extend(sorted(ext_n))
    if native:
        L += ['', "[target.'cfg(not(target_arch = \"wasm32\"))'.dependencies]"] + sorted(native)
    L += ['', '[dev-dependencies]']
    for lib in sorted(dev_used):
        d = BY_LIB[lib]
        L.append(f'{d.package} = {{ path = "{rel(pkg_dir, DB_REL + "/" + d.pkg_dir)}" }}')
    L.extend(sorted(ext_dev))
    L.append('')
    return '\n'.join(L)

def native_gates():
    """🧊️ Native-only imports keep the `not(wasm32)` gate their original re-exports carried."""
    path = f'{STAGE}/🗄️storage/🦀️.rs'
    t = read(path)
    a = 'use db_storage_fs::FsStorage;\n'
    assert t.count(a) == 1
    t = t.replace(a, '#[cfg(not(target_arch = "wasm32"))]\n' + a)
    m = re.search(r'#\[cfg\(any\(test, feature = "testkit"\)\)\]\nuse db_io_core::\{([^}]*)\};\n', t)
    assert m and 'claimed_backend' in m.group(1)
    names = [n.strip() for n in m.group(1).split(',') if n.strip() and n.strip() != 'claimed_backend']
    new = ('#[cfg(any(test, feature = "testkit"))]\nuse db_io_core::{' + ', '.join(names) + '};\n'
           '#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]\nuse db_io_core::claimed_backend;\n')
    t = t[:m.start()] + new + t[m.end():]
    write(path, t)

def main():
    native_gates()
    c = Composer()
    root, fam, hubfiles = build_facade(c)
    meta = [m for m in json.load(open(f'{SP}/crates.json')) if m['lib'] != 'db']
    meta.append(dict(lib='db', root='🦀️.rs', files=['🦀️.rs', '🧪️tests/🔬️unit/🦀️.rs']))
    json.dump(meta, open(f'{SP}/crates.json', 'w'), ensure_ascii=False, indent=2)
    files, graph = manifest.run(STAGE, meta)
    libs = set(BY_LIB)
    ftext = strip(read(f'{STAGE}/🦀️.rs'))
    # the facade test module is a child; split test part away from normal part
    normal = ftext.split('//#region')[0]
    ttext = strip(read(f'{STAGE}/🧪️tests/🔬️unit/🦀️.rs'))
    ln = used_libs(normal, libs, 'db')
    lt = used_libs(ttext, libs, 'db') - ln
    eid = ext_idents(normal)
    etid = ext_idents(ttext)
    lines_n, native = ext_lines(eid, BY_LIB['db_ids'])
    # ext_lines computes paths relative to a crate dir; recompute facade-relative lines
    pkg_dir = f'{DB_REL}/📦️packages/🦀️rust'
    ext_n = []
    for n in sorted(eid):
        if n in EXT_PATH and n not in DEV_ONLY:
            name, p, _ = EXT_PATH[n]
            ext_n.append(f'{name} = {{ path = "{rel(pkg_dir, p)}" }}')
        elif n in manifest.WS:
            ext_n.append(f'{manifest.WS[n]} = {{ workspace = true }}')
        elif n in manifest.PLAIN:
            ext_n.append(manifest.PLAIN[n])
    ext_dev = []
    for n in sorted(etid):
        if n in EXT_PATH and n not in {'semio_framework_os_kernel'} | set(eid):
            name, p, _ = EXT_PATH[n]
            ext_dev.append(f'{name} = {{ path = "{rel(pkg_dir, p)}" }}')
    files['📦️packages/🦀️rust/Cargo.toml'] = facade_manifest(ln, lt, ext_n, ext_dev, [])
    for rel_, text in files.items():
        write(os.path.join(STAGE, rel_), text)
    cyc = manifest.dag_check(graph)
    graph['db'] = dict(deps=sorted(ln), dev=sorted(lt), testkit=False)
    json.dump(graph, open(f'{SP}/graph.json', 'w'), ensure_ascii=False, indent=1)
    print('dev-cycles', manifest.dev_cycles(graph))
    print('crates', len(graph), 'cycles', cyc[:3], 'facade deps', sorted(ln))
    print('hub files needing path rename', len(hubfiles))

if __name__ == '__main__':
    main()
