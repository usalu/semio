import os, sys, re

TM = '/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map'
ORIG = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/tiled-map.orig.rs'

src = open(ORIG, encoding='utf-8').read()
assert open(TM + '/🦀️.rs', encoding='utf-8').read() == src, 'tree file differs from pristine original'
lines = src.split('\n')
assert lines[-1] == ''
lines = lines[:-1]
assert len(lines) == 4187


def L(a, b, first=None, last=None):
    seg = lines[a - 1:b]
    if first is not None:
        assert seg[0].startswith(first), (a, seg[0], first)
    if last is not None:
        assert seg[-1].startswith(last), (b, seg[-1], last)
    return seg


def dedent4(seg):
    out = []
    for l in seg:
        if l.strip() == '':
            out.append('')
        else:
            assert l.startswith('    '), l
            out.append(l[4:])
    return out


def doc(emoji, title, *more):
    out = ['//! %s %s' % (emoji, title)]
    for m in more:
        out.append('//!')
        out.append('//! ' + m)
    out.append('')
    return out


def write(rel, content):
    path = TM + '/' + rel
    os.makedirs(os.path.dirname(path), exist_ok=True)
    text = '\n'.join(content)
    if not text.endswith('\n'):
        text += '\n'
    open(path, 'w', encoding='utf-8').write(text)


def trim(seg):
    while seg and seg[-1].strip() == '':
        seg = seg[:-1]
    while seg and seg[0].strip() == '':
        seg = seg[1:]
    return seg


def join(*segs):
    out = []
    for s in segs:
        s = trim(list(s))
        if not s:
            continue
        if out:
            out.append('')
        out.extend(s)
    return out


# ---- projection ------------------------------------------------------------
L(333, 333, 'pub mod projection {')
proj_body = dedent4(L(334, 377))
L(378, 378, '}')
write('🧭️projection/🦀️.rs',
      doc('🧭️', 'Web Mercator projection of the tiled map: lon/lat to world space, tile rectangles and cover zoom.',
          'Mounted as `pub mod projection` of the tiled-map root; pure math with no host state.') +
      ['// #region 🔖️Projection'] + proj_body + ['// #endregion 🔖️Projection'])

# ---- tiles -----------------------------------------------------------------
L(382, 382, 'pub mod tiles {')
tiles_body = dedent4(L(383, 578))
L(579, 579, '}')
write('🧩️tiles/🦀️.rs',
      doc('🧩️', 'Raster tile addressing of the tiled map: visible-tile cursor, tile keys and prefetch rings.',
          'Mounted as `pub mod tiles` of the tiled-map root.') +
      ['// #region 🔖️Tiles'] + tiles_body + ['// #endregion 🔖️Tiles'])

# ---- vector tiles ----------------------------------------------------------
L(583, 583, 'pub mod vector_tiles {')
L(584, 584, '    use super::Color;')
dec_main = dedent4(L(586, 1085, '    /// OpenFreeMap'))
dec_prop = dedent4(L(1086, 1123, '    pub fn property_str'))
dec_flag = dedent4(L(1210, 1212, '    pub fn property_flag'))
dec_bbox = dedent4(L(1417, 1440, '    const TILE_BBOX_EPS'))
dec_ring = dedent4(L(1460, 1477, '    pub fn mvt_ring_is_tile_bbox_cover', '    }'))
L(1478, 1478, '}')
sty_a = dedent4(L(1125, 1208, '    pub fn layer_draw_rank'))
sty_b = dedent4(L(1214, 1415, '    pub fn transportation_visible'))
sty_c = dedent4(L(1442, 1458, '    fn water_class_is_open_sea'))

# decoder include! path: two folders deeper
dec_all = join(dec_main, dec_prop, dec_flag, dec_bbox, dec_ring)
idx = [i for i, l in enumerate(dec_all) if l.startswith('include!(')]
assert len(idx) == 1
assert dec_all[idx[0]] == 'include!("🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs");'
dec_all[idx[0]] = 'include!("../../🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs");'
write('🧱️vector-tiles/📦️mvt-decoder/🦀️.rs',
      doc('📦️', 'Mapbox Vector Tile decoder: protobuf cursor, raw layer decode, geometry decode, property helpers and tile-edge geometry tests.',
          'Domain-neutral: nothing here knows a map vendor, tile schema or level-of-detail policy. Public paths stay under `vector_tiles` through the facade re-export.') +
      ['// #region 🔖️MvtDecoder'] + dec_all + ['// #endregion 🔖️MvtDecoder'])

write('🌍️gis-policy/🎨️vector-style/🦀️.rs',
      doc('🎨️', 'GIS vector-tile style policy: OpenMapTiles class ranks, per-level-of-detail draw gates, stroke and label sizing.',
          'Wrong-direction content: a framework module encodes OSM/OpenMapTiles cartography. It is grouped under `gis_policy` so the GIS plugin can take the whole subtree; see the follow-up note in `RESULT-TILED-MAP.md`.',
          'Re-exported unchanged under `vector_tiles` so no public path moves.') +
      ['use super::Color;',
       'use super::vector_tiles::{property_class, MAP_VECTOR_TILE_MAX_Z};',
       '',
       '// #region 🔖️VectorStyle'] + join(sty_a, sty_b, sty_c) + ['// #endregion 🔖️VectorStyle'])

write('🧱️vector-tiles/🦀️.rs',
      doc('🧱️', 'Vector-tile facade: the neutral MVT decoder plus the GIS vector style, both re-exported under one public path.',
          'Public API is unchanged: `vector_tiles::decode_mvt`, `vector_tiles::layer_draw_rank` and every other former inline item resolve here.') +
      ['use super::FrameworkSurfaceTiledMapError;',
       '',
       '// #region 🔖️VectorTiles',
       '#[path = "📦️mvt-decoder/🦀️.rs"]',
       'mod mvt_decoder;',
       'pub use mvt_decoder::*;',
       'pub use super::gis_policy::vector_style::*;',
       '// #endregion 🔖️VectorTiles'])

# ---- gis policy ------------------------------------------------------------
gis_tables = L(78, 93, 'const GIS_MAP_LODS')
gis_scale = L(98, 101, '/// 🌐️ Upper visible')
gis_mode = L(145, 164, '/// 📶️ Window LOD select')
gis_resolve = L(179, 216, 'fn resolve_map_lod_index_from_span')
gis_current = L(218, 221, 'fn current_map_lod')
gis_forced = L(230, 261, 'fn forced_lod_tile_z')
gis_camera = L(268, 278, 'pub fn gis_map_camera_limits_json')
gis_slider = L(1604, 1655, '/// 🎚️ Layer ids that accept')
write('🌍️gis-policy/🦀️.rs',
      doc('🌍️', 'GIS policy of the tiled map: level-of-detail band tables, `gis_map_*` JSON surfaces, tile-zoom picking and layer-weight slider gating.',
          'Wrong-direction content: the framework surface names a concrete plugin domain. Everything GIS-specific is grouped here (and in `vector_style` below) so a follow-up can relocate this subtree into the GIS plugin behind a neutral map-policy port; see `RESULT-TILED-MAP.md`.',
          'Items keep their former `tiled_map::` paths through the root re-export.') +
      ['use canvas::lod::{Lod, LodScale};',
       'use super::*;',
       '',
       '#[path = "🎨️vector-style/🦀️.rs"]',
       'pub(super) mod vector_style;',
       '',
       '// #region 🔖️GisPolicy'] +
      join(gis_tables, gis_scale, gis_mode, gis_resolve, gis_current, gis_forced, gis_camera, gis_slider) +
      ['// #endregion 🔖️GisPolicy'])

# ---- map state -------------------------------------------------------------
ms_ext = L(1481, 1500, '// #region 🔖️MapExtension', '// #endregion 🔖️MapExtension')
ms_a = L(1502, 1602, '// #region 🔖️MapContent', '}')
ms_b = L(1657, 1832, 'fn map_layer_default_true', '}')
write('🪢️map-state/🦀️.rs',
      doc('🪢️', 'Map descriptors and layer state: extension trait, position/route/region data, tile mode, layer weights and visibility, tile ledger.',
          'Linked to [`super::MapHost`], which owns one [`MapTileLedger`] and one set of layer tables.') +
      ['use super::*;', ''] +
      join(ms_ext, ms_a, ms_b) + ['// #endregion 🔖️MapContent'])

# ---- map host --------------------------------------------------------------
mh_struct = L(1834, 1994, 'pub struct MapHost {', '}')
mh_default = L(2170, 2195, 'impl Default for MapHost', '}')
write('🏛️map-host/🦀️.rs',
      doc('🏛️', 'The tiled-map host: struct, field-wise retirement owner, pan interaction state and default construction.',
          'The two `impl MapHost` halves are nested children so the private fields stay visible to them: camera and picking, and the host scene.') +
      ['use super::*;',
       '',
       '#[path = "📷️camera-and-picking/🦀️.rs"]',
       'mod camera_and_picking;',
       '#[path = "🖼️host-scene/🦀️.rs"]',
       'mod host_scene;',
       '',
       '// #region 🔖️MapHost'] + join(mh_struct, mh_default) + ['// #endregion 🔖️MapHost'])

impl_a = L(2309, 3053, '    pub fn new() -> Self', '    }')
L(2308, 2308, 'impl MapHost {')
L(2306, 2306, '//#endregion ⚠️ Errors')
write('🏛️map-host/📷️camera-and-picking/🦀️.rs',
      doc('🧭️', 'MapHost camera, level-of-detail queries, tile cache, interaction commit, uploads, pointer input and feature picking.',
          'First half of `impl MapHost`; the second half lives in `host_scene`.') +
      ['use super::*;', '', '// #region 🔖️CameraAndPicking', 'impl MapHost {'] + impl_a + ['}', '// #endregion 🔖️CameraAndPicking'])

impl_b = L(3055, 3875, '    pub fn prepare_visible_tiles', '    }')
L(3876, 3876, '}')
hs_structs = L(2036, 2048, '#[derive(Clone, Copy)]', '}')
hs_canvas = L(3878, 3886, 'impl canvas_content::CanvasContent for MapHost', '}')
write('🏛️map-host/🖼️host-scene/🦀️.rs',
      doc('🖼️', 'MapHost scene building: visible-tile preparation, tile geometry, raster and vector painting, overlays and the CanvasContent impl.',
          'Second half of `impl MapHost`; the first half lives in `camera_and_picking`. Implements `canvas_content::CanvasContent` for [`super::MapHost`].') +
      ['use super::*;', '', '// #region 🔖️HostScene'] + hs_structs + ['', 'impl MapHost {'] + impl_b + ['}', ''] + hs_canvas + ['// #endregion 🔖️HostScene'])

# ---- interaction and labels -----------------------------------------------
il_plan = L(1996, 2034, '//#region 🔖️InteractionPlan', '//#endregion 🔖️InteractionPlan')
il_geom = L(2050, 2168, 'fn map_point_segment_distance', '}')
il_label = L(2197, 2258, '// #region 🔖️LabelDeclutter', '// #endregion 🔖️LabelDeclutter')
write('🖱️interaction-and-labels/🦀️.rs',
      doc('🖱️', 'Map interaction plan, geometry hit-test helpers and screen-space label declutter.',
          'Plans are produced and committed by [`super::MapHost`]; the geometry helpers back its feature picking.') +
      ['use super::*;', ''] + join(il_plan, il_geom, il_label))

# ---- wasm session ----------------------------------------------------------
ws = L(3889, 4181, '// #region 🔖️WasmSession', '// #endregion 🔖️WasmSession')
write('🌐️wasm-session/🦀️.rs',
      doc('🌐️', 'Browser `MapSession` bridge: a wasm-bindgen wrapper around [`super::MapHost`] plus its GPU session.',
          'Compiled only for `wasm32` browser targets (not `wasm32-wasip2`); every item keeps its own `cfg` gate.') +
      ['use super::*;', ''] + ws)

# ---- root ------------------------------------------------------------------
root = []
root += L(1, 21)
assert lines[21] == 'use canvas::lod::{Lod, LodScale};'
root += L(23, 76)
root += ['// #region 🔖️MapLod']
root += join(L(95, 96, '/// 🔭️ OSM raster'),
             L(103, 103, 'const MAX_MAP_TILE_CACHE_ENTRIES'),
             L(105, 107, '/// 🧮️ The most tiles'),
             L(109, 143, 'const REVISION_FNV_OFFSET', '}'),
             L(166, 177, 'fn viewport_lon_span_degrees', '}'),
             L(223, 228, 'fn ideal_tile_z_for_viewport', '}'),
             L(263, 266, '/// 🔭️ Whole-world'),
             L(280, 329, 'fn clamp_map_zoom(', '}'))
root += ['// #endregion 🔖️MapLod', '']
root += ['// #region 🔖️Children',
         '#[path = "🧭️projection/🦀️.rs"]',
         'pub mod projection;',
         '#[path = "🧩️tiles/🦀️.rs"]',
         'pub mod tiles;',
         '#[path = "🧱️vector-tiles/🦀️.rs"]',
         'pub mod vector_tiles;',
         '#[path = "🌍️gis-policy/🦀️.rs"]',
         'mod gis_policy;',
         'pub use gis_policy::*;',
         '#[path = "🪢️map-state/🦀️.rs"]',
         'mod map_state;',
         'pub use map_state::*;',
         '#[path = "🏛️map-host/🦀️.rs"]',
         'mod map_host;',
         'pub use map_host::*;',
         '#[path = "🖱️interaction-and-labels/🦀️.rs"]',
         'mod interaction_and_labels;',
         'pub use interaction_and_labels::*;',
         '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]',
         '#[path = "🌐️wasm-session/🦀️.rs"]',
         'mod wasm_session;',
         '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]',
         'pub use wasm_session::MapSession;',
         '// #endregion 🔖️Children',
         '']
root += L(2260, 2306, '//#region ⚠️ Errors', '//#endregion ⚠️ Errors')
root += ['']
root += L(4183, 4187, '// #region 🔖️Tests', '// #endregion 🔖️Tests')
write('🦀️.rs', root)
print('ok', len(root))
