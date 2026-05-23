#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../" && pwd)"
SRC="$ROOT/puzzle/2d/rs/lib.rs"
OUT="$ROOT/puzzle/2d/rs/lib.rs.new"

extract() { sed -n "$1,$2p" "$SRC"; }

{
  echo '//! 🧩 Puzzle 2d board: elements palette, icon codec, `BoardHost`, WASM session on `mathematical_graph` + `infinite_cavas`.'
  echo '#![allow(clippy::missing_errors_doc, reason = "Puzzle board bundle is internal to puzzle 2d.")]'
  echo ''
  echo 'pub use infinite_cavas::{self as cavas, *};'
  echo 'pub use mathematical_graph::{self as graph, *};'
  echo 'pub use gis_map as map;'
  echo 'pub use reasoning_mindmap as mindmap;'
  echo ''
  echo 'pub use vello_svg::usvg;'
  echo 'pub use vello_svg::vello;'
  echo ''
  extract 7 15 | sed 's/mod board_icon_assets/pub mod board_icon_assets/'
  echo ''
  extract 1953 2383
  echo ''
  extract 2385 7425 | sed \
    -e 's/use super::geom_sel/use crate::geom_sel/g' \
    -e 's/use super::vcompute/use crate::vcompute/g' \
    -e 's/use super::scene_json/use crate::scene_json/g' \
    -e 's/super::board_json_visible_option/crate::board_json_visible_option/g' \
    -e 's/super::elements_board_palette/crate::elements_board_palette/g' \
    -e 's/super::board_metabolism_icons/crate::board_metabolism_icons/g' \
    -e 's/super::board_icon_codec/crate::board_icon_codec/g' \
    -e 's/super::svg_icon_vello09/crate::svg_icon_vello09/g' \
    -e 's/crate::vcompute/crate::cavas::vcompute/g' \
    -e 's/crate::geom_sel/crate::cavas::geom_sel/g'
  echo ''
  extract 7733 11300 | sed \
    -e 's/use crate::vello/use cavas::vello/g' \
    -e 's/crate::vello/cavas::vello/g' \
    -e 's/apply_redraw_layout_to_fixture_v1_json/graph::apply_redraw_layout_to_fixture_v1_json/g' \
    -e 's/apply_edge_handle_snap_to_fixture_v1_json/graph::apply_edge_handle_snap_to_fixture_v1_json/g'
} > "$OUT"

mv "$OUT" "$SRC"
echo "Updated $SRC"
