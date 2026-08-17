#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../" && pwd)"
SRC="$ROOT/puzzle/2d/rs/lib.rs"
CAVAS="$ROOT/infinite/cavas/vello/lib.rs"
GRAPH="$ROOT/mathematical/graph/lib.rs"

extract() { sed -n "$1,$2p" "$SRC"; }

{
  echo '//! 🖼️ Application-neutral tile-based infinite canvas (Vello/WebGPU); extend via `CanvasExtension`.'
  echo '#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]'
  echo ''
  echo 'pub use vello_svg::usvg;'
  echo 'pub use vello_svg::vello;'
  echo ''
  extract 7 15
  echo ''
  extract 17 485
  echo ''
  extract 2157 2383
  echo ''
  echo '// #region 🔖️CanvasExtension'
  echo '/// 🧩️ Extension hook for domain-specific canvas behavior (hit-test, paint, kinds).'
  echo 'pub trait CanvasExtension: Send + Sync {'
  echo '    fn extension_id(&self) -> &str;'
  echo '}'
  echo ''
  echo '/// ⚙️ Generic infinite-canvas engine shell; domain logic lives in `E`.'
  echo 'pub struct CanvasEngine<E: CanvasExtension> {'
  echo '    pub extension: E,'
  echo '}'
  echo ''
  echo 'impl<E: CanvasExtension> CanvasEngine<E> {'
  echo '    pub fn new(extension: E) -> Self {'
  echo '        Self { extension }'
  echo '    }'
  echo '}'
  echo '// #endregion 🔖️CanvasExtension'
} > "$CAVAS"

{
  echo '//! 🕸️ Property graph on infinite canvas; extend via `GraphExtension`.'
  echo ''
  echo 'pub use infinite_cavas as cavas;'
  echo 'pub use infinite_cavas::{fixture_edge_handle_ids_from_object, CameraJson, EdgeDescJson, FixtureV1Json, HandleDescJson, NodeDescJson, SceneDescriptorJson, WireDescJson};'
  echo ''
  extract 487 1948 | sed 's/use super::/use crate::cavas::/g; s/super::board_json_visible_or_true/crate::board_json_visible_or_true/g; s/super::fixture_edge_handle_ids_from_object/cavas::fixture_edge_handle_ids_from_object/g'
  echo ''
  echo 'fn board_json_hidden_flag(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {'
  echo '    obj.get("hidden").and_then(|v| v.as_bool())'
  echo '}'
  echo ''
  echo 'fn board_json_visible_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {'
  echo '    match board_json_hidden_flag(obj) {'
  echo '        Some(hidden) => Some(!hidden),'
  echo '        None => obj.get("visible").and_then(|v| v.as_bool()),'
  echo '    }'
  echo '}'
  echo ''
  echo 'pub fn board_json_visible_or_true(obj: &serde_json::Map<String, serde_json::Value>) -> bool {'
  echo '    board_json_visible_option(obj).unwrap_or(true)'
  echo '}'
  echo ''
  echo '// #region 🔖️GraphExtension'
  echo 'pub trait GraphExtension: cavas::CanvasExtension {}'
  echo '// #endregion 🔖️GraphExtension'
  echo ''
  extract 7426 7731 | sed 's/use crate::vcompute/use cavas::vcompute/g; s/vcompute::/cavas::vcompute::/g'
} > "$GRAPH"

echo "Wrote $CAVAS and $GRAPH"
