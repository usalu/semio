#!/usr/bin/env python3
# 🚀️ Applies the carved WP2 tree to the repository: drift guard against the build input, atomic replacement, hub 2, the root-resident units, the reactor mount line, the interim lift mounts of other packages, manifests and package stubs.
# usage: apply.py dry|write
import os, re, shutil, sys, filecmp, json
ARGS = list(sys.argv[1:])
from common import *
import model as M
import engine as E
import hubgen
import flips

OUT = WS + '/out'
IN = WS + '/in/' + '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'

def live(rel):
    return PLUG + '/' + rel

def atomic_write(path, text):
    tmp = path + '.pwp2.tmp'
    with open(tmp, 'w', encoding='utf8') as h:
        h.write(text)
    os.replace(tmp, path)

def main(write_it):
    problems = []
    plan = []
    for dp, dn, fn in os.walk(OUT):
        for f in fn:
            src = os.path.join(dp, f)
            rel = os.path.relpath(src, OUT)
            if rel.startswith('📦️pkg/'):
                continue
            cur, base = live(rel), IN + '/' + rel
            if os.path.exists(cur):
                if os.path.exists(base) and read(cur) != read(base):
                    problems.append('DRIFT ' + rel)
                    continue
            plan.append(rel)
    # manifests (OUT/📦️pkg/<key>/Cargo.toml) and stubs
    man = {}
    pk = OUT + '/📦️pkg'
    if os.path.isdir(pk):
        for key in os.listdir(pk):
            man[key] = read(pk + '/' + key + '/Cargo.toml')
    for key, text in man.items():
        c = M.CRATES_JSON[key]
        p = PLUG + '/' + c['pkg'].split('/🔌️plugin/')[1] + '/Cargo.toml'
        base = IN + '/' + c['pkg'].split('/🔌️plugin/')[1] + '/Cargo.toml'
        if os.path.exists(base) and read(p) != read(base):
            problems.append('DRIFT manifest ' + key)
    print('files:', len(plan), 'manifests:', len(man))
    for p in problems:
        print('  ', p)
    if problems or not write_it:
        return 1 if problems else 0
    for rel in plan:
        dst = live(rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        atomic_write(dst, read(OUT + '/' + rel))
    for key, text in man.items():
        c = M.CRATES_JSON[key]
        pkgdir = PLUG + '/' + c['pkg'].split('/🔌️plugin/')[1]
        atomic_write(pkgdir + '/Cargo.toml', text)
        stub = pkgdir + '/🦀️.rs'
        if os.path.exists(stub):
            os.remove(stub)
    # hub 2 from the live hub, root-resident units, reactor mount, interim lift mounts
    E.set_input(PLUG)
    hub = hubgen.build_hub()
    hub = hub.replace('//#endregion 📜️DeclarationFacet', hubgen.extra_units() + '\n//#endregion 📜️DeclarationFacet', 1)
    atomic_write(live(hubgen.HUB_REL), hub)
    root = read(live('🦀️.rs'))
    a = '#[path = "📜️declaration/📨️emission/🦀️.rs"]\npub(crate) mod app_emission;\n#[path = "📜️declaration/👷️job/🦀️.rs"]\npub(crate) mod tool_contract;\n'
    assert root.count(a) == 1, 'root units moved'
    atomic_write(live('🦀️.rs'), root.replace(a, ''))
    rr = read(live('⚛️reactor/🦀️.rs'))
    old = '#[path = "🪟️surfaces/🦀️.rs"]\npub(crate) mod surface_context;\n'
    assert rr.count(old) == 1, 'reactor mount'
    atomic_write(live('⚛️reactor/🦀️.rs'), rr.replace(old, 'pub(crate) use semio_framework_plugin_plugin_program::surface_context;\n'))
    flipped = []
    for dp, dn, fn in os.walk(PLUG):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target', '📦️packages', '.cache')]
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f)
                t = read(p)
                if '🛬️lifted' in t:
                    rel = os.path.relpath(p, PLUG)
                    n, hits = flips.flip(t, rel, False)
                    if hits:
                        atomic_write(p, n)
                        flipped += hits
    print('flipped interim mounts:', len(flipped))
    for h in flipped:
        print('  ', h)
    return 0

if __name__ == '__main__':
    sys.exit(main(ARGS and ARGS[0] == 'write'))
