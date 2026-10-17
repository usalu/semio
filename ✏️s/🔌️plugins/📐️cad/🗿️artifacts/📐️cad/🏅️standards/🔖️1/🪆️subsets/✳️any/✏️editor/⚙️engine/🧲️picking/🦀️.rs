//! 🧲️ CAD spatial pick/selection engine — the Rust twin of the `🧲️GeometryTargets` and
//! `🧲️GeometryInteraction` regions of `⚙️engine/📺️renderer/🟦️.tsx` (lines 647-1380 and 2277-2888).
//!
//! 🧭️ Target-neutral on purpose: these are the sub-object pick targets, visibility filters,
//! hover-key aliases and modifier-mode selection merges a spatial editor needs, computed from the
//! pane's own `(&[CadObject], Option<&CadGeometry>)` pair — the exact pair a composed pane
//! (`crate::CadComposedPanes`) holds. React's `World3dHost` and the wgpu
//! `render_world3d_surface_step` both paint whatever the play app puts on the `World3d` wire, so
//! computing this here closes the gap on BOTH targets at once instead of twice.
//!
//! 🚫️ Deliberately NOT here: screen-space marquee/ray math
//! (`spatialPickTargetsFromRay`/`ScreenSelection`, `marqueeCoverageFromGesture`, `pointInPolygon`,
//! `projectPointToClient`). That is host geometry, already Rust in the framework
//! (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🎯️picking/🦀️.rs`'s `marquee_is_crossing{,_from_path}`,
//! `point_in_polygon`) and consumed by the renderer, not by a guest that never sees client pixels.
//! Merge modes reuse `protocol::MergeMode` — the one wire vocabulary — instead of a bespoke
//! `SpatialSelectionMode`.

extern crate semio_framework_os_kernel as protocol;

use semio_s_artifact_cad_cad_typology::{default_model_definition_id, is_shape_model_definition, load_typology, model_definition_selection_entity_kinds, model_definition_typology_ids, model_definition_uses_geometry_picking, resolve_typology_style, spatial_typology_toggle_label, ModelEntityKind, ResolvedTypologyStyle, PRIMITIVE_MODEL_ENTITY_KINDS};
use semio_s_artifact_cad_cad_model::{geometry::CadEdge, geometry::CadEdgeCurve, geometry::CadFace, geometry::CadGeometry, geometry::CadObject, geometry::CadShell, geometry::CadSolid, geometry::CadVertex, geometry::CadWire};
use semio_framework_value::DslValue;
use protocol::MergeMode;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[path = "🏷️pick-vocabulary/🦀️.rs"]
mod pick_vocabulary;
pub use pick_vocabulary::*;

#[path = "🪣️geometry-buckets-and-curves/🦀️.rs"]
mod geometry_buckets_and_curves;
pub use geometry_buckets_and_curves::*;

#[path = "🎯️pick-targets-and-indices/🦀️.rs"]
mod pick_targets_and_indices;
pub use pick_targets_and_indices::*;

#[path = "🎚️visibility-and-toggles/🦀️.rs"]
mod visibility_and_toggles;
pub use visibility_and_toggles::*;

#[path = "👁️reveal-and-selection-merge/🦀️.rs"]
mod reveal_and_selection_merge;
pub use reveal_and_selection_merge::*;

#[path = "🎨️target-style/🦀️.rs"]
mod target_style;
pub use target_style::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
