import re
from model import *
from carve_engine import *
import imports
FAC = EC + '/🦀️.rs'
# 1. hoist the in-file test mounts of board2d, node_graph and engine_surface_attach into the facade
hoists = [('🎲️board2d', 'board2d_engine_tests', 'sync_board_engine'), ('🕸️node-graph', 'catalogue_workflow_drop_tests', None), ('🧩️engine-surface-attach', 'metrics_invalidation_drain_tests', None)]
fac = rd(FAC)
add = ''
for folder, mod, _ in hoists:
    p = EC + '/' + folder + '/🦀️.rs'; t = rd(p)
    m = re.search(r'#\[cfg\(test\)\]\n#\[path = "\.\./\.\./\.\./(🧪️tests/[^"]+)"\]\nmod ' + mod + r';\n\n?', t)
    assert m, folder
    t = t[:m.start()] + t[m.end():]
    wr(p, t)
    add += '#[cfg(test)]\n#[path = "../../%s"]\nmod %s;\n\n' % (m.group(1), mod)
# test-only imports of upper-crate names move to the facade
p = EC + '/🎲️board2d/🦀️.rs'; t = rd(p)
for old in ['#[cfg(test)]\nuse super::engine_surface_attach::sync_board_engine;\n', '#[cfg(test)]\nuse super::engine_surface_registry::{ensure_engine_surface, BoardSyncCache};\n', '#[cfg(test)]\nuse super::with_engine_close_context;\n', '#[cfg(test)]\nuse semio_framework_canvas as canvas;\n', '#[cfg(test)]\nuse std::mem::ManuallyDrop;\n']:
    assert t.count(old) == 1, old
    t = t.replace(old, '')
wr(p, t)
p = EC + '/🧩️engine-surface-attach/🦀️.rs'; t = rd(p)
old = '#[cfg(test)]\nuse super::prepared_engine_canvas::EngineCanvasPresenter;\n'
assert t.count(old) == 1
wr(p, t.replace(old, ''))
anchor = '#[cfg(test)]\ninclude!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs");\n'
assert fac.count(anchor) == 1
fac = fac.replace(anchor, anchor + '\n#[cfg(test)]\nuse engine_surface_attach::sync_board_engine;\n#[cfg(test)]\nuse engine_surface_registry::BoardSyncCache;\n\n' + add.rstrip('\n') + '\n')
wr(FAC, fac)
# 2. cross-crate sibling paths
for c in ['engine-text', 'engine-board', 'engine-graph', 'engine-attach']: rewrite_siblings(c)
# 3. root-level names
p = EC + '/🗺️tiled-map/🦀️.rs'; t = rd(p)
assert t.count('crate::reserve_renderer_asset_request') >= 1
wr(p, t.replace('crate::reserve_renderer_asset_request', 'crate::asset_authority::reserve_renderer_asset_request'))
p = EC + '/🧩️engine-surface-attach/🦀️.rs'; t = rd(p)
assert t.count('crate::infinite::board::') == 1
wr(p, t.replace('crate::infinite::board::', 'infinite_canvas::board::'))
# 4. lib roots
CONTRACTS = 'extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;'
ASSET = 'extern crate semio_framework_os_renderer_wgpu_asset_authority as asset_authority;'
INF = 'extern crate infinite_canvas as infinite_world;'
lib_root('engine-text', '//! ✍️ Text editor scene document, delivery and interaction of an engine surface.\n//!\n//! The editor scene document, the typing delivery driver, the explicit draft settings and the public text editor pointer, key, wheel and\n//! action API; the delivery state itself lives with the surface registry.', [INF])
lib_root('engine-board', '//! 🎲️ Board2d, paint2d and tiled map hosts of an engine surface.\n//!\n//! The board2d host, its coalesced event queue and pointer settle, the paint2d layers, context menu targets and pointer handlers, and the\n//! tiled map host with its tile fetch reservation.', [INF, ASSET, CONTRACTS])
lib_root('engine-graph', '//! 🕸️ Node graph interaction of an engine surface.\n//!\n//! Node graph camera, catalogue drag and drop, wheel, pointer, connection cursor, labels and overlays of an engine surface.', [CONTRACTS, 'pub(crate) use semio_framework_os_renderer_wgpu_host_base::action_args_json;'])
lib_root('engine-attach', '//! 🔗️ Engine surface attach and per-kind scene sync.\n//!\n//! Attaching a retained scene to an engine surface and projecting its per-kind state: surface registrations, the node graph, tiled map,\n//! board, raster and text editor scene syncs and the raster composite progress.', [CONTRACTS, INF])
# 5. facade
for c in ['engine-text', 'engine-board', 'engine-graph', 'engine-attach']: facade_use(c)
print('staged')
