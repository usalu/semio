#!/usr/bin/env python3
"""Generate the moves.json for the pattern-exact re-layering, from the live crate inventory."""
import json, re, os

crates = {}  # old_dir -> pkg_name
for line in open('/tmp/all_crates.txt'):
    line = line.rstrip('\n')
    if not line:
        continue
    d, name = line.split('\t')
    crates[d] = name

moves = {}  # old_dir -> new_dir

def add(old, new):
    assert old in crates, f"not a known crate dir: {old}"
    moves[old] = new

# ---- 1. framework general ----
add('framework/core/rs', 'framework/rs')
add('framework/hash/rs', 'framework/module/hash/rs')
add('framework/editor/rs', 'framework/module/editor/rs')
add('framework/schema/rs', 'framework/module/schema/rs')
for name in ['tiled-map', 'terrain', 'node-graph', 'paint']:
    add(f'framework/surface/{name}/rs', f'framework/module/surface/{name}/rs')
add('framework/ui/styling/rs', 'framework/module/ui/styling/rs')
add('framework/ui/wgpu/rs', 'framework/module/ui/wgpu/rs')
add('framework/ui/tui/rs', 'framework/module/ui/tui/rs')
add('framework/plugin/rs', 'framework/product/os/module/plugin/rs')
add('framework/plugin/host/rs', 'framework/product/os/module/plugin/host/rs')

# ---- 2. math family (all of framework/os/kernel/math/*) ----
for d in list(crates):
    if d.startswith('framework/os/kernel/math/'):
        rest = d[len('framework/os/kernel/math/'):]
        add(d, f'framework/module/math/{rest}')

# ---- 3. os product: core/run/renderer + remaining kernel families ----
for d in list(crates):
    if d.startswith('framework/os/core/'):
        rest = d[len('framework/os/core/'):]
        add(d, f'framework/product/os/{rest}')
add('framework/os/run/rs', 'framework/product/os/module/run/rs')
add('framework/os/renderer/wgpu/rs', 'framework/product/os/module/renderer/wgpu/rs')

for fam in ['db', 'dsl', 'flow', 'infinite', 'neural', 'pack', 'protocol', 'store', 'vcs', 'workflow']:
    for d in list(crates):
        if d.startswith(f'framework/os/kernel/{fam}/'):
            rest = d[len(f'framework/os/kernel/{fam}/'):]
            add(d, f'framework/product/os/module/{fam}/{rest}')

# ---- 4. s/module (2d, 3d, cross-plugin shared) ----
for fam in ['2d', '3d']:
    for d in list(crates):
        if d.startswith(f'framework/os/kernel/{fam}/'):
            rest = d[len(f'framework/os/kernel/{fam}/'):]
            add(d, f's/module/{fam}/{rest}')

add('s/plugin/reasoning/shared/rs', 's/module/mindmap/rs')
add('s/plugin/imperative/engine/rs', 's/module/imperative/rs')

# ---- 5. s/plugin bundles -> manifest/artifact ----
# every crate directly at s/plugin/<p>/rs (the flat bundle) moves to manifest/artifact/rs
plugin_names = sorted(set(d.split('/')[2] for d in crates if d.startswith('s/plugin/')))
for p in plugin_names:
    bundle = f's/plugin/{p}/rs'
    if bundle in crates:
        add(bundle, f's/plugin/{p}/manifest/artifact/rs')

# ---- 6. flattened single-app plugins: app/<slot> -> app/<app>/<slot> ----
# app id = folder name choice already established per plugin; use plugin name as app name
# (matches the "flatten fix" precedent - these are exactly the plugins that were flattened because
#  app name == plugin name)
FLAT_SINGLE_APP = ['note', 'vcs', 'forms', 'raster', 'imperative', 'shooting', 'sequence',
                    'mathematical', 'flow', 'writer', 'draw', 'layout', 'dag', 'lowpoly',
                    'playbook', 'remodel', 'cad']
for p in FLAT_SINGLE_APP:
    prefix = f's/plugin/{p}/app/'
    for d in list(crates):
        if d.startswith(prefix) and not d.startswith(f's/plugin/{p}/app/{p}/'):
            rest = d[len(prefix):]
            add(d, f's/plugin/{p}/app/{p}/{rest}')

# ---- 7. contribution bundles: module/ -> extension/ ----
EXTENSIONS = [
    ('imperative', ['control', 'core', 'logic', 'math', 'text']),
    ('sourcing', ['beams', 'slabs', 'windows']),
    ('playbook', ['procedural']),
    ('cad', ['spatial-shape', 'aec-building', 'aec-building-energy', 'aec-building-structure']),
]
for plugin, exts in EXTENSIONS:
    for ext in exts:
        old_base = f's/plugin/{plugin}/module/{ext}'
        for d in list(crates):
            if d.startswith(old_base):
                rest = d[len(old_base):]
                add(d, f's/plugin/{plugin}/extension/{ext}{rest}')

# ---- 8. plugin-internal supporting crates -> plugin/module/<name> ----
INTERNAL_MODULES = {
    's/plugin/trinity/ram/rs': 's/plugin/trinity/module/ram/rs',
    's/plugin/trinity/jack/core/rs': 's/plugin/trinity/module/jack/rs',
    's/plugin/trinity/jack/shell/rs': 's/plugin/trinity/module/jack/shell/rs',
    's/plugin/trinity/jack/lsp/rs': 's/plugin/trinity/module/jack/lsp/rs',
    's/plugin/fem/shared/rs': 's/plugin/fem/module/shared/rs',
    's/plugin/space/shared/rs': 's/plugin/space/module/shared/rs',
    's/plugin/animate/core/rs': 's/plugin/animate/module/core/rs',
    's/plugin/animate/video/rs': 's/plugin/animate/module/video/rs',
    's/plugin/draw/fsm/rs': 's/plugin/draw/module/fsm/rs',
    's/plugin/draw/fsm/macros/rs': 's/plugin/draw/module/fsm/macros/rs',
    's/plugin/norm/core/rs': 's/plugin/norm/module/core/rs',
}
for old, new in INTERNAL_MODULES.items():
    if old in crates:
        add(old, new)

for name in ['image', 'video', 'camera', 'feature', 'sfm', 'dense', 'mesh', 'motion', 'geo', 'engine']:
    old = f's/plugin/remodel/{name}/rs'
    if old in crates:
        add(old, f's/plugin/remodel/module/{name}/rs')

# ---- architect (broken, not-a-member, still move on disk) ----
add('s/plugin/architect/spine/rs', 's/plugin/architect/module/spine/rs')

print(f"total moves: {len(moves)}")
out = [{"oldDir": k, "newDir": v, "newPkg": crates[k]} for k, v in moves.items()]
with open('.repo/🎫/26/07/29/MOVE-APPS-INTO-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES/w-relayer-moves.json', 'w') as f:
    json.dump(out, f, indent=2)

# sanity: no duplicate new dirs
newdirs = [v for v in moves.values()]
dups = set(x for x in newdirs if newdirs.count(x) > 1)
if dups:
    print("DUPLICATE TARGETS:", dups)
