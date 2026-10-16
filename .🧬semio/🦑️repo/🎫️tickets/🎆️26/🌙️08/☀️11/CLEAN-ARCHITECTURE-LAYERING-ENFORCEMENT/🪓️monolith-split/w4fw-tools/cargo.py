import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cfg import *
FACADE_PKG = "🧰️framework/📦️packages/🦀️rust"
def parse_facade_deps(R):
    """ident -> (kind 'path'|'workspace'|'version', spec dict, table) from the (original) facade Cargo.toml"""
    text = open(os.path.join(R, FACADE_PKG, 'Cargo.toml'), encoding='utf-8').read()
    out = {}
    table = None
    for line in text.split('\n'):
        s = line.strip()
        m = re.match(r'^\[(.+)\]$', s)
        if m: table = m.group(1); continue
        if table in ('dependencies', 'dev-dependencies') and '=' in s and not s.startswith('#'):
            name, spec = s.split('=', 1); name = name.strip(); spec = spec.strip()
            out[(table, name.replace('-', '_'))] = (name, spec)
    return out
def up_of(node):
    segs = [s for s in node.split('/') if s][1:]  # drop 🧰️framework
    return '../' * (len(segs) + 2 + 0) if False else '/'.join(['..'] * (len(segs) + 2))
def pkg_dir(c): return CLUSTERS[c]['node'] + '/📦️packages/🦀️rust'
def render_cargo(R, c, lib_cluster_deps, lib_externs, dev_cluster_deps, dev_externs, facade_deps, features=None, dev_features=None):
    info = CLUSTERS[c]
    node = info['node']
    segs = [s for s in node.split('/') if s]
    # node begins with 🧰️framework
    below = len(segs) - 1
    workspace = '/'.join(['..'] * (below + 2))
    lib_rel = '../../' + os.path.basename(info['root'])
    pd = os.path.join(R, pkg_dir(c))
    L = ['[package]', f'workspace = "{workspace}"', f'name = "{info["pkg"]}"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true', '',
         '[package.metadata.semio]', 'role = "framework"', f'id = "{c}"', '',
         '[package.metadata.semio.rust-source-boundary]', 'maxSourceBytes = 262144', 'maxSourceFiles = 32', 'maxActiveLines = 4000', '',
         '[lints]', 'workspace = true', '', '[lib]', f'path = "{lib_rel}"', '']
    if features:
        L += ['[features]'] + [f'{k} = [{", ".join(chr(34) + v + chr(34) for v in vs)}]' for k, vs in features.items()] + ['']
    def dep_lines(clusters, externs, table):
        lines = []
        for o in sorted(clusters):
            rel = os.path.relpath(os.path.join(R, pkg_dir(o)), pd)
            lines.append(f'{CLUSTERS[o]["pkg"]} = {{ path = "{rel}" }}')
        for e in sorted(externs):
            spec = facade_deps.get(('dependencies', e)) or facade_deps.get(('dev-dependencies', e))
            if not spec: lines.append(f'# MISSING {e}'); continue
            name, s = spec
            m = re.search(r'path\s*=\s*"([^"]+)"', s)
            if m:
                ab = os.path.normpath(os.path.join(R, FACADE_PKG, m.group(1)))
                rel = os.path.relpath(ab, pd)
                s = s.replace(m.group(1), rel)
            lines.append(f'{name} = {s}')
        return lines
    L.append('[dependencies]'); L += dep_lines(lib_cluster_deps, lib_externs, 'dependencies'); L.append('')
    dc = set(dev_cluster_deps) - set(lib_cluster_deps); de = set(dev_externs) - set(lib_externs)
    feat = dev_features or {}
    extra = [o for o in feat if o in dev_cluster_deps]
    if dc or de or extra:
        L.append('[dev-dependencies]')
        lines = dep_lines(dc - set(extra), de, 'dev-dependencies')
        for o in sorted(set(extra)):
            rel = os.path.relpath(os.path.join(R, pkg_dir(o)), pd)
            lines.append(f'{CLUSTERS[o]["pkg"]} = {{ path = "{rel}", features = [{", ".join(chr(34) + f + chr(34) for f in feat[o])}] }}')
        L += sorted(lines); L.append('')
    return '\n'.join(L)
def render_project(c):
    return json.dumps({"name": f"@semio-tech/{c}-rs", "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + '\n'
