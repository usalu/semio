import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, reg, crategen
from carve import R, UI, FW, PKG, rd, wr
spec = carve.Spec('specs/tui.json')
T = UI + '/⌨️tui'
# vt root: drop the tests mount
p = T + '/📟️vt/🦀️.rs'
s = rd(p)
blk = '\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
assert blk in s
wr(p, s.replace(blk, ''))
# vt unit root becomes a crate root
p = T + '/📟️vt/🧪️tests/🔬️unit/🦀️.rs'
s = rd(p)
if not s.startswith('#![cfg(test)]'):
    s = s.replace('use super::encode::', 'use semio_framework_ui_tui_vt::encode::').replace('use super::pane::', 'use semio_framework_ui_tui_vt::pane::').replace('use super::{CellPoint, Palette, VtScreen};', 'use semio_framework_ui_tui_vt::{CellPoint, Palette, VtScreen};')
    s = '//! 🏛️ Virtual terminal laws: screen sequences, history reflow, pane input and pointer, encoders and the shared corpus.\n\n#![cfg(test)]\n\n' + s
    wr(p, s)
p = T + '/🧪️tests/🔬️unit/🦀️.rs'
s = rd(p)
if not s.startswith('#![cfg(test)]') and '#![cfg(test)]' not in s[:200]:
    s = '//! 🔬️ tui unit corpus: layout, window, table, parser, theme, geometry, text, cell, ANSI, vt and widget laws.\n\n#![cfg(test)]\n\n' + s
    wr(p, s)
# manifests for laws crates and the trimmed laws
k = reg.next_route_index()
for key in ('laws', 'unit-laws', 'vt-laws'):
    c = spec.crates[key]
    pkgdir = c['abs'] + '/' + PKG
    deps, devs, targets = crategen.build_deps(spec, c, pkgdir, special={'libc', 'wasm-bindgen'})
    devs = sorted(set(devs) | set(deps), key=lambda l: l.split(' ')[0])
    blocks = []
    for tgt, lines in targets.items(): blocks.append("[target.'%s'.dependencies]\n" % tgt + '\n'.join(sorted(set(lines))))
    txt = reg.manifest_text(c['pkg'], c['id'], c['desc'], pkgdir, [], devs, [], blocks)
    wr(pkgdir + '/Cargo.toml', txt); wr(pkgdir + '/📋️project.json', reg.project_json(c['id']))
    reg.add_member(pkgdir); reg.add_owner(c['id'], pkgdir)
    if reg.add_routes(c['pkg'], c['id'], k, 'semio-framework-ui-retained-tree-tree'): k += 1
    print(key, len(devs))
