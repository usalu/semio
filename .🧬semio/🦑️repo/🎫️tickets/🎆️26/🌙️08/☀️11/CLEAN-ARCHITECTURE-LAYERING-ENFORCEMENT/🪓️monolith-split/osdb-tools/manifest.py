"""📦 Cargo manifests and project files for every crate, derived from the staged sources.

manifest.py <stage db dir> <crates.json>  -> writes <pkg>/Cargo.toml and <pkg>/📋️project.json into the stage, prints the crate graph.
"""
import collections
import json
import os
import re
import sys

from lib import *
from model import parse_items
from rs import strip, idents
from plan import CRATES, BY_LIB, PKG_PREFIX
from widen import Crate

FW = '🧰️framework'
EXT_PATH = {
    'semio_framework_pack_source': ('semio-framework-pack-source', f'{FW}/🔨️modules/🎒️pack/🚰️source/📦️packages/🦀️rust', None),
    'semio_framework_diagnostic': ('semio-framework-diagnostic', f'{FW}/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust', None),
    'semio_framework_value': ('semio-framework-value', f'{FW}/🔨️modules/🌱️value/📦️packages/🦀️rust', None),
    'semio_framework_hash': ('semio-framework-hash', f'{FW}/🔨️modules/🔏️hash/📦️packages/🦀️rust', None),
    'semio_framework_async_macros': ('semio-framework-async-macros', f'{FW}/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust', None),
    'semio_framework_schema_composition': ('semio-framework-schema-composition', f'{FW}/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust', None),
    'semio_framework_os_kernel': ('semio-framework-os-kernel', f'{FW}/🛍️products/💻️os/📦️packages/🦀️rust', None),
}
WS = {
    'semio_framework_artifact_reference': 'semio-framework-artifact-reference',
    'protocol': 'semio-framework-replication',
    'semio_framework_async': 'semio-framework-async',
    'semio_framework_trace': 'semio-framework-trace',
}
PLAIN = {'serde': 'serde = { version = "1.0.219", features = ["derive"] }', 'serde_json': 'serde_json = "1.0.140"'}
NATIVE = {
    'rusqlite': 'rusqlite = { version = "0.38.0", features = ["blob", "bundled"] }',
    'sqlx': 'sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "tls-rustls", "postgres"] }',
    'neo4rs': 'neo4rs = { version = "0.8" }',
    'tokio': 'tokio = { workspace = true, features = ["rt", "rt-multi-thread", "net", "time"] }',
}
DEV_ONLY = {'semio_framework_async_macros', 'semio_framework_schema_composition'}


def classify(x):
    """Texts of crate x by class: normal, testkit, test."""
    normal, testkit, test = [], [], []
    for f in x.files:
        text = x.text[f]
        st = x.stripped[f]
        is_test_file = '/🧪️tests/' in '/' + f and f != x.root
        if f == x.root:
            items, trail, lines = x.parsed[f]
            sl = st.split('\n')
            for it in items:
                seg = '\n'.join(sl[it.start:it.end + 1])
                attr = ' '.join(it.attrs)
                if it.kind == 'mod' and re.search(r'cfg\(\s*test\s*\)', attr):
                    continue
                if 'feature = "testkit"' in attr or 'feature="testkit"' in attr:
                    testkit.append(seg)
                elif re.search(r'cfg\(\s*test\s*\)', attr):
                    test.append(seg)
                else:
                    normal.append(seg)
        elif is_test_file:
            test.append(st)
        elif '🧰️testkit' in f:
            testkit.append(st)
        else:
            normal.append(st)
    return '\n'.join(normal), '\n'.join(testkit), '\n'.join(test)


def used_libs(text, libs, self_lib):
    out = set()
    for m in re.finditer(r'(?<![A-Za-z0-9_:])(db_[a-z0-9_]+)::', text):
        if m.group(1) in libs and m.group(1) != self_lib:
            out.add(m.group(1))
    return out


def ext_idents(text):
    ids = idents(text)
    ids |= {n for n in ('pack', 'dsl', 'store') if re.search(r'(?<![A-Za-z0-9_:.])' + n + r'::', text)} and {'semio_framework_os_kernel'} or set()
    return ids


def ups_to_root(c):
    return len(DB_REL.split('/')) + len(c.node.split('/')) + 2


def rel(from_dir, to_dir):
    return os.path.relpath(to_dir, from_dir)


def render(c, deps, dev, opt, feats, ext_normal, ext_dev, ext_native, root_rel_to_fw, tests_exist):
    ups = '/'.join(['..'] * ups_to_root(c))
    pkg_dir = f'{DB_REL}/{c.pkg_dir}'
    L = ['[package]', f'workspace = "{ups}"', f'name = "{c.package}"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true', f'description = "{c.desc}"', '',
         '[package.metadata.semio]', 'role = "product"', f'id = "{c.id}"', '', '[package.metadata.semio.rust-source-boundary]', 'maxSourceBytes = 262144', 'maxSourceFiles = 32', 'maxActiveLines = 4000', '',
         '[lints]', 'workspace = true', '', '[lib]', f'name = "{c.lib}"', 'path = "../../🦀️.rs"', '']
    if feats:
        L.append('[features]')
        for k in sorted(feats):
            L.append(f'{k} = [{", ".join(chr(34) + v + chr(34) for v in feats[k])}]')
        L.append('')
    L.append('[dependencies]')
    for lib in sorted(deps):
        d = BY_LIB[lib]
        L.append(f'{d.package} = {{ path = "{rel(pkg_dir, DB_REL + "/" + d.pkg_dir)}" }}')
    for lib in sorted(opt):
        d = BY_LIB[lib]
        L.append(f'{d.package} = {{ path = "{rel(pkg_dir, DB_REL + "/" + d.pkg_dir)}", optional = true }}')
    for line in sorted(ext_normal):
        L.append(line)
    if ext_native:
        L.append('')
        L.append("[target.'cfg(not(target_arch = \"wasm32\"))'.dependencies]")
        for line in sorted(ext_native):
            L.append(line)
    L.append('')
    L.append('[dev-dependencies]')
    for lib, f in sorted(dev.items()):
        d = BY_LIB[lib]
        feat = f', features = [{", ".join(chr(34) + x + chr(34) for x in sorted(f))}]' if f else ''
        L.append(f'{d.package} = {{ path = "{rel(pkg_dir, DB_REL + "/" + d.pkg_dir)}"{feat} }}')
    for line in sorted(ext_dev):
        L.append(line)
    L.append('')
    return '\n'.join(L)


def ext_lines(ids, c, dev=False, native_gate=False):
    pkg_dir = f'{DB_REL}/{c.pkg_dir}'
    out = []
    native = []
    for n in sorted(ids):
        if n in EXT_PATH and (n not in DEV_ONLY) != dev:
            name, p, _ = EXT_PATH[n]
            out.append(f'{name} = {{ path = "{rel(pkg_dir, p)}" }}')
        elif n in WS and not dev:
            out.append(f'{WS[n]} = {{ workspace = true }}')
        elif n in PLAIN and not dev:
            out.append(PLAIN[n])
        elif n in NATIVE and not dev:
            native.append(NATIVE[n])
    return out, native


def run(stage, crates_json):
    meta = crates_json if isinstance(crates_json, list) else json.load(open(crates_json))
    meta = [c for c in meta if c['lib'] in BY_LIB]
    crates = {c['lib']: Crate(stage, c['lib'], c['root'], c['files']) for c in meta}
    libs = set(crates)
    graph = {}
    files = {}
    testkit_libs = set()
    # first pass: which crates export a testkit
    for lib, x in crates.items():
        n, tk, t = classify(x)
        if tk.strip() or any('🧰️testkit' in f for f in x.files) or 'feature = "testkit"' in x.text[x.root]:
            testkit_libs.add(lib)
    claimed = {'db_io_core'}
    for lib, x in crates.items():
        c = BY_LIB[lib]
        n, tk, t = classify(x)
        ln, ltk, lt = used_libs(n, libs, lib), used_libs(tk, libs, lib), used_libs(t, libs, lib)
        deps = ln
        opt = ltk - ln
        dev = {}
        for d in (lt | ltk) - ln:
            dev[d] = set()
        for d in (lt | ltk):
            if d in testkit_libs:
                dev.setdefault(d, set()).add('testkit')
            if d == 'db_io_core' and any('claimed_backend' in s for s in (t, tk)):
                dev.setdefault(d, set()).add('claimed-backend-laws')
        for d in deps & testkit_libs:
            if lt or ltk:
                dev.setdefault(d, set()).add('testkit')
        feats = {}
        tk_feat = []
        for d in sorted(opt):
            tk_feat.append(f'dep:{BY_LIB[d].package}')
        for d in sorted((deps | opt) & testkit_libs):
            tk_feat.append(f'{BY_LIB[d].package}/testkit')
        if lib in testkit_libs:
            feats['testkit'] = tk_feat
        if lib == 'db_io_core':
            feats['claimed-backend-laws'] = []
        eids = ext_idents(n)
        ekit = ext_idents(tk)
        etest = ext_idents(t)
        ext_normal, native = ext_lines(eids | ekit, c)
        ext_dev_ids = (etest | ekit) & DEV_ONLY
        ext_dev, _ = ext_lines(ext_dev_ids, c, dev=True)
        if (etest | ekit) & {'semio_framework_async'} and 'semio_framework_async' in (eids | ekit | etest):
            ext_dev.append(f'semio-framework-async = {{ workspace = true, features = ["testkit"] }}')
        if t.strip() and ('semio_framework_os_kernel' in (eids | etest)):
            pkg_dir = f'{DB_REL}/{c.pkg_dir}'
            ext_dev.append(f'semio-framework-os-kernel = {{ path = "{rel(pkg_dir, EXT_PATH["semio_framework_os_kernel"][1])}", features = ["durable-group-testing", "mutation-testing"] }}')
        if 'semio_framework_os_kernel' in ext_idents(strip(n + tk)):
            pass
        text = render(c, deps, dev, opt, feats, ext_normal, ext_dev, native, None, bool(t.strip()))
        pkgrel = f'{c.pkg_dir}'
        files[f'{pkgrel}/Cargo.toml'] = text
        files[f'{pkgrel}/📋️project.json'] = json.dumps({'name': c.project, 'tags': ['scope:framework', 'language:rust']}, ensure_ascii=False, indent=2) + '\n'
        graph[lib] = dict(deps=sorted(deps | opt), dev=sorted(dev), testkit=lib in testkit_libs)
    return files, graph


def dev_cycles(graph):
    """Dev-dependency edges X -> Y where Y depends (normally) on X: duplicate-type hazards."""
    reach = {}

    def reaches(a):
        if a in reach:
            return reach[a]
        out = set()
        reach[a] = out
        for d in graph[a]['deps']:
            out.add(d)
            out |= reaches(d)
        return out
    bad = []
    for x in graph:
        for y in graph[x]['dev']:
            if y in graph and x in reaches(y):
                bad.append((x, y))
    return bad


def dag_check(graph):
    state = {}
    cyc = []

    def visit(v, stack):
        if state.get(v) == 1:
            cyc.append(stack[stack.index(v):] + [v])
            return
        if state.get(v) == 2:
            return
        state[v] = 1
        for w in graph[v]['deps']:
            visit(w, stack + [v])
        state[v] = 2
    for v in graph:
        visit(v, [])
    return cyc


if __name__ == '__main__':
    stage, cj = sys.argv[1], sys.argv[2]
    files, graph = run(stage, cj)
    for rel_, text in files.items():
        write(os.path.join(stage, rel_), text)
    cyc = dag_check(graph)
    print('crates', len(graph), 'cycles', cyc[:3])
    json.dump(graph, open(os.path.dirname(stage) + '/graph.json', 'w'), ensure_ascii=False, indent=1)
