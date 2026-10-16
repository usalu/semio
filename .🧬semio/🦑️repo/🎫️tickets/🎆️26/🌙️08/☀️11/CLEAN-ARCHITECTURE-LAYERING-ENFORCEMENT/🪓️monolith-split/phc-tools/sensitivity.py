#!/usr/bin/env python3
"""🧪️ Seeded regressions: each check mutates a copy of the generated tree in memory and must make the matching gate report it."""
import os, sys, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, plan_host, oracle_host as O, defscan, hcmod, ws as wsm, helpers
from carve import Model


def mutated(ws, file, old, new):
    src = ws.read(file)
    assert src.count(old) == 1, (file, old, src.count(old))
    ws.write(file, src.replace(old, new))


def main():
    ws, model, roots = state.load()
    eng = plan_host.CRATES[plan_host.ENGINE]['root']
    guest = plan_host.CRATES[plan_host.GUEST]['root']
    # 1: a cross-crate function loses `pub`: the privacy pass of the differential must name it
    mutated(ws, eng, 'pub async fn build_shared_engine(', 'async fn build_shared_engine(')
    only_s, only_b = O.run_native(ws, roots, show=False)
    print('seeded privacy regression: only-split errors', sum(only_s.values()), [k for k in only_s if 'build_shared_engine' in k[1]][:2])
    ws.write(eng, ws.orig.get(eng) if False else state.load()[0].read(eng))
    # 2: an import of the bindings crate disappears from the guest runtime
    src = ws.read(guest)
    m = re.search(r'use semio_framework_plugin_host_wit::\{[^}]*\};\n', src) or re.search(r'use semio_framework_plugin_host_wit::[^\n]*;\n', src)
    ws.write(guest, src.replace(m.group(0), '', 1))
    only_s, only_b = O.run_native(ws, roots, show=False)
    print('seeded missing import: only-split errors', sum(only_s.values()))
    ws.write(guest, src)
    # 3: definition scan sees a deleted item
    ws2, model2, roots2 = state.load()
    base = [('semio_framework_plugin_host', plan_host.P('📦️packages/🦀️rust/🦀️.rs'))]
    helpers.remove_items(ws2, guest, lambda it, toks: it.kind == 'struct' and it.name == 'PackageRef')
    lost, gained, un = defscan.scan('seeded', base, ws2, list(roots2.items()), defscan.explained)
    print('seeded deleted item: lost', sum(lost.values()), dict(lost))


if __name__ == '__main__':
    main()
