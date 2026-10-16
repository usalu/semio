#!/usr/bin/env python3
"""🧪️ Seeded regressions against the flushed tree: each mutates the in-memory workspace (or a manifest text) and the matching gate must report it."""
import os, sys, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, ws as wsm, extern_set, plan_host, oracle_host as O, defscan, helpers, regs_host

extern_set.install()
ws, model, roots = state.load()
make = wsm.WS


def base_factory(*a, **k):
    b = make(*a, **k)
    b.cur = dict(ws.orig)
    return b


def cycles(manifests):
    edges = {}
    for path, text in manifests.items():
        d = os.path.dirname(path)
        edges[d] = [os.path.normpath(os.path.join(d, m)) for m in re.findall(r'semio-framework-plugin-host[\w-]*\s*=\s*\{\s*path\s*=\s*"([^"]+)"', text)]
    state_of, found = {}, []

    def visit(n, trail):
        if state_of.get(n) == 1:
            found.append(trail + [n])
            return
        if state_of.get(n) == 2:
            return
        state_of[n] = 1
        for m in edges.get(n, []):
            visit(m, trail + [n])
        state_of[n] = 2
    for n in list(edges):
        visit(n, [])
    return found


def main():
    eng = plan_host.CRATES[plan_host.ENGINE]['root']
    guest = plan_host.CRATES[plan_host.GUEST]['root']
    src = ws.read(eng)
    old = 'pub async fn build_shared_engine('
    assert src.count(old) == 1
    ws.write(eng, src.replace(old, 'async fn build_shared_engine(', 1))
    only_s, only_b = O.run_native(ws, roots, show=False)
    hit = [k for k in only_s if 'build_shared_engine' in k[1]]
    print('seeded privacy regression (pub removed): only-split errors', sum(only_s.values()), 'naming the function:', len(hit), flush=True)
    ws.write(eng, src)
    gsrc = ws.read(guest)
    m = re.search(r'use semio_framework_plugin_host_wit::\{[^}]*\};\n', gsrc) or re.search(r'use semio_framework_plugin_host_wit::[^\n]*;\n', gsrc)
    ws.write(guest, gsrc.replace(m.group(0), '', 1))
    only_s, only_b = O.run_native(ws, roots, show=False)
    print('seeded missing import (bindings use removed): only-split errors', sum(only_s.values()), flush=True)
    ws.write(guest, gsrc)
    wsm.WS = base_factory
    base = [('semio_framework_plugin_host', plan_host.P('📦️packages/🦀️rust/🦀️.rs'))]
    helpers.remove_items(ws, guest, lambda it, toks: it.kind == 'struct' and it.name == 'PackageRef')
    lost, gained, un = defscan.scan('seeded', base, ws, list(roots.items()), defscan.explained)
    print('seeded deleted item (PackageRef): lost', sum(lost.values()), dict(lost), flush=True)
    ws.write(guest, gsrc)
    pkgs = {regs_host.pkg_dir_of(v['root']) + '/Cargo.toml': None for v in plan_host.CRATES.values()}
    texts = {p: open(p, encoding='utf-8').read() for p in pkgs}
    print('cargo dependency cycles on the real manifests:', len(cycles(texts)), flush=True)
    sp = regs_host.pkg_dir_of(plan_host.CRATES[plan_host.SHARD]['root']) + '/Cargo.toml'
    bad = dict(texts)
    bad[sp] = texts[sp].replace('\n[dev-dependencies]', '\nsemio-framework-plugin-host-shard-executor = { path = "../../🧵️executor/📦️packages/🦀️rust" }\n\n[dev-dependencies]', 1)
    assert bad[sp] != texts[sp]
    print('seeded cargo cycle (shard depends on its executor): cycles reported', len(cycles(bad)), flush=True)


if __name__ == '__main__':
    main()
