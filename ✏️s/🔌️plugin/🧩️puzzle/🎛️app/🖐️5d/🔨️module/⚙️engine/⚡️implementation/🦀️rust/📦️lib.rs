//! ⚙️ Puzzle 5d app — headless compute (constitutional: engine).

use puzzle_5d::{Puzzle5dError, Puzzle5dFastener, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dProjection};
use serde_json::Value;
use std::collections::HashSet;

//#region 🔖️BrushEngine
pub use puzzle_3d_engine::BrushPlacePayload;

pub struct Puzzle5dPrecomputeSession {
    inner: puzzle_3d_engine::Puzzle3dPrecomputeSession,
}

impl Default for Puzzle5dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle5dPrecomputeSession {
    pub fn new() -> Self {
        Self { inner: puzzle_3d_engine::Puzzle3dPrecomputeSession::new() }
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.inner.register_mesh(url, positions, indices);
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.inner.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.inner.precompute_step(budget)
    }

    pub fn brush_candidates(&self, grip_full_id: &str) -> String {
        self.inner.brush_candidates(grip_full_id)
    }

    pub fn brush_preview_json(&self, grip_full_id: &str, candidate_index: usize) -> Option<String> {
        self.inner.brush_preview_json(grip_full_id, candidate_index)
    }

    pub fn fill_progress(&self) -> String {
        self.inner.fill_progress()
    }
}

/// 🧵️ Native/WASI-p2 build: `puzzle_3d_engine::Puzzle3dPrecomputeSession`'s `Puzzle3dError`-typed `_rust`-suffixed API surface is available under this cfg — mirrors `puzzle_3d_engine::Puzzle3dPrecomputeSession`'s own matching split.
#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
impl Puzzle5dPrecomputeSession {
    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle5dError> {
        Ok(self.inner.set_scene(json)?)
    }

    pub fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle5dError> {
        Ok(self.inner.apply_brush_placement_rust(payload_json)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(self.inner.apply_fill_count_rust(count)?)
    }
}

/// 🌐️ Browser wasm-bindgen build (wasm32, non-p2): `puzzle_3d_engine::Puzzle3dPrecomputeSession`'s `JsValue`-typed API surface is available instead — mirrors those method names/signatures 1:1 so callers on this target get the same capability.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl Puzzle5dPrecomputeSession {
    pub fn set_scene(&mut self, json: &str) -> Result<(), wasm_bindgen::JsValue> {
        self.inner.set_scene(json)
    }

    pub fn apply_brush_placement_json(&mut self, payload_json: &str) -> Result<String, wasm_bindgen::JsValue> {
        self.inner.apply_brush_placement_json(payload_json)
    }

    pub fn apply_fill_count(&mut self, count: u32) -> Result<String, wasm_bindgen::JsValue> {
        self.inner.apply_fill_count(count)
    }

    pub fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle5dError> {
        Ok(self.inner.apply_brush_placement_rust(payload_json)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(self.inner.apply_fill_count_rust(count)?)
    }
}
//#endregion 🔖️BrushEngine

//#region 🔖️KindCompatibility
pub const PUZZLE5D_DEFAULT_MANIFEST_ID: &str = "puzzle5d-default";

/// 🧲️ Looks up whether two grip kinds are compatible per the `puzzle5d-default` manifest's `kindCompatibility` rows —
/// the single shared table both the 2D board and 3D world honor so brush/fill suggestions agree across projections.
pub fn puzzle5d_grip_kinds_compatible(source_kind: &str, target_kind: &str) -> bool {
    let Some(manifest) = mathematical_graph_manifest::manifest_by_id(PUZZLE5D_DEFAULT_MANIFEST_ID) else {
        return false;
    };
    manifest.kind_compatibility.iter().any(|row| {
        let source = row.get("source").and_then(|value| value.as_str());
        let target = row.get("target").and_then(|value| value.as_str());
        let bidirectional = row.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == Some(source_kind) && target == Some(target_kind)) || (bidirectional && source == Some(target_kind) && target == Some(source_kind))
    })
}
//#endregion 🔖️KindCompatibility

//#region 🔖️DocumentHelpers
pub fn empty_puzzle5d_projection() -> Puzzle5dProjection {
    Puzzle5dProjection::default()
}

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️CopyPasteTranslate
/// 🧩️ The part id a `"part_id:grip_id"` full grip reference belongs to.
fn owning_part_id(grip_ref: &str) -> &str {
    grip_ref.split(':').next().unwrap_or(grip_ref)
}

fn rewrite_grip_ref(grip_ref: &str, id_map: &std::collections::HashMap<String, String>) -> String {
    match grip_ref.split_once(':') {
        Some((part_id, grip_id)) => match id_map.get(part_id) {
            Some(fresh_part_id) => format!("{fresh_part_id}:{grip_id}"),
            None => grip_ref.to_string(),
        },
        None => grip_ref.to_string(),
    }
}

/// 🧮️ Closure-selects a copy fragment from `projection`: expands the part set to include every
/// selected fastener's endpoint parts, then expands the fastener set to include every fastener whose
/// BOTH endpoints are now in the part set — mirrors semio_compose_rs's `copyDesign` closure rule
/// (`semio_compose_rs/dev/algorithm/js/index.ts:483`).
pub fn copy_selection(projection: &Puzzle5dProjection, part_ids: &[String], fastener_ids: &[String]) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut part_set: HashSet<String> = part_ids.iter().cloned().collect();
    for fastener in &projection.fasteners {
        if fastener_ids.contains(&fastener.id) {
            part_set.insert(owning_part_id(&fastener.source).to_string());
            part_set.insert(owning_part_id(&fastener.target).to_string());
        }
    }
    let mut fastener_set: HashSet<String> = fastener_ids.iter().cloned().collect();
    if !part_set.is_empty() {
        for fastener in &projection.fasteners {
            let source_part = owning_part_id(&fastener.source);
            let target_part = owning_part_id(&fastener.target);
            if part_set.contains(source_part) && part_set.contains(target_part) {
                fastener_set.insert(fastener.id.clone());
            }
        }
    }
    let parts = projection.parts.iter().filter(|part| part_set.contains(&part.id)).cloned().collect();
    let fasteners = projection.fasteners.iter().filter(|fastener| fastener_set.contains(&fastener.id)).cloned().collect();
    (parts, fasteners)
}

/// 🧮️ The average 2D board position of `parts` — `None` for an empty slice.
pub fn centroid_2d(parts: &[Puzzle5dPart]) -> Option<(f64, f64)> {
    if parts.is_empty() {
        return None;
    }
    let (mut sum_x, mut sum_y) = (0.0, 0.0);
    for part in parts {
        sum_x += part.part_2d.x;
        sum_y += part.part_2d.y;
    }
    let count = parts.len() as f64;
    Some((sum_x / count, sum_y / count))
}

/// 🧮️ Materializes a copied fragment against `projection` at 2D delta `delta_2d` (applied verbatim to
/// the 3D origin's x/y too; z unchanged) — fresh ids are minted for every part to dodge collisions
/// with the target document, and fastener endpoints are remapped to the fresh part ids. Mirrors
/// semio_compose_rs's `pasteDesign` (`semio_compose_rs/dev/algorithm/js/index.ts:515`). Returns the ready-to-insert
/// parts/fasteners; the caller turns each into one `SetPart`/`SetFastener` operation appended past the
/// document's current `parts`/`fasteners` length.
pub fn paste_selection(projection: &Puzzle5dProjection, fragment_parts: &[Puzzle5dPart], fragment_fasteners: &[Puzzle5dFastener], delta_2d: (f64, f64)) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut id_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut existing_ids: HashSet<String> = projection.parts.iter().map(|part| part.id.clone()).collect();
    let mut fresh_parts = Vec::with_capacity(fragment_parts.len());
    for part in fragment_parts {
        let fresh_id = next_id(existing_ids.iter().map(String::as_str), "part-");
        existing_ids.insert(fresh_id.clone());
        id_map.insert(part.id.clone(), fresh_id.clone());
        let mut next_part = part.clone();
        next_part.id = fresh_id;
        next_part.part_2d.x += delta_2d.0;
        next_part.part_2d.y += delta_2d.1;
        next_part.part_3d.origin[0] += delta_2d.0;
        next_part.part_3d.origin[1] += delta_2d.1;
        fresh_parts.push(next_part);
    }
    let mut existing_fastener_ids: HashSet<String> = projection.fasteners.iter().map(|fastener| fastener.id.clone()).collect();
    let mut fresh_fasteners = Vec::with_capacity(fragment_fasteners.len());
    for fastener in fragment_fasteners {
        let fresh_id = next_id(existing_fastener_ids.iter().map(String::as_str), "fastener-");
        existing_fastener_ids.insert(fresh_id.clone());
        let mut next_fastener = fastener.clone();
        next_fastener.id = fresh_id;
        next_fastener.source = rewrite_grip_ref(&fastener.source, &id_map);
        next_fastener.target = rewrite_grip_ref(&fastener.target, &id_map);
        fresh_fasteners.push(next_fastener);
    }
    (fresh_parts, fresh_fasteners)
}

/// 🧮️ Shifts `part_ids`' 2D board positions and 3D world origins by the given deltas — the puzzle-5d
/// analog of semio_compose_rs's `dragPieces`/`movePieces` (no flatten/re-layout solver here; positions are
/// explicit, so a translate is a direct position write). Mirrors
/// `semio_compose_rs/dev/algorithm/js/index.ts:424,451`. Returns `(index, updated part)` pairs ready for
/// `SetPart` operations.
pub fn translate_parts(projection: &Puzzle5dProjection, part_ids: &[String], delta_2d: (f64, f64), delta_3d: [f64; 3]) -> Vec<(usize, Puzzle5dPart)> {
    projection
        .parts
        .iter()
        .enumerate()
        .filter(|(_, part)| part_ids.contains(&part.id))
        .map(|(index, part)| {
            let mut next_part = part.clone();
            next_part.part_2d.x += delta_2d.0;
            next_part.part_2d.y += delta_2d.1;
            next_part.part_3d.origin[0] += delta_3d[0];
            next_part.part_3d.origin[1] += delta_3d[1];
            next_part.part_3d.origin[2] += delta_3d[2];
            (index, next_part)
        })
        .collect()
}

/// 🔍️ Every part-kind id in `kind_catalogs` whose grip kinds are `kind_compatibility`-compatible with
/// `part_id`'s own grip kinds (excluding `part_id`'s current kind) — candidates a "replace kind"
/// picker offers. Mirrors semio_compose_rs's `findReplaceableTypesForSelection` (`semio_compose_rs/dev/algorithm/js/
/// index.ts:84`), computed for real against `kind_catalogs`/`kind_compatibility` instead of a fixture stub.
pub fn find_replaceable_kinds(projection: &Puzzle5dProjection, part_id: &str) -> Vec<String> {
    let Some(part) = projection.parts.iter().find(|part| part.id == part_id) else {
        return Vec::new();
    };
    let Some(catalogs) = &projection.kind_catalogs else {
        return Vec::new();
    };
    let grip_kinds: HashSet<&str> = part.grips.iter().filter_map(|grip| grip.grip_kind.as_deref()).collect();
    let current_kind = part.part_kind.as_deref().unwrap_or("");
    let mut candidates = Vec::new();
    for candidate in &catalogs.parts {
        if candidate.id == current_kind {
            continue;
        }
        let candidate_grip_kinds: HashSet<&str> = candidate.grips.iter().map(|template| template.grip_kind.as_str()).collect();
        let compatible = grip_kinds.iter().any(|source_kind| {
            candidate_grip_kinds.iter().any(|target_kind| {
                projection.kind_compatibility.iter().any(|rule| {
                    (rule.source == *source_kind && rule.target == *target_kind) || (rule.bidirectional && rule.source == *target_kind && rule.target == *source_kind)
                })
            })
        });
        if compatible {
            candidates.push(candidate.id.clone());
        }
    }
    candidates
}
//#endregion 🔖️CopyPasteTranslate

//#region 🔖️ComposeImport
/// 🧩️ Reads a semio_compose_rs "hashed collection" (`{ hash, items: [...] }`) or a bare array (test-friendly)
/// — mirrors semio_compose_rs's own `__itemsOf`/`fixtureItemsOf` duality (`semio_compose_rs/dev/algorithm/js/
/// index.ts:94`).
fn compose_collection_items(value: &Value) -> &[Value] {
    if let Some(array) = value.as_array() {
        return array;
    }
    value.get("items").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

fn read_vec3(value: &Value) -> Option<[f64; 3]> {
    Some([value.get("x")?.as_f64()?, value.get("y")?.as_f64()?, value.get("z")?.as_f64()?])
}

fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 🔄️ Rotation matrix (columns = x/y/z axes of the target frame) → quaternion `[x, y, z, w]`, via the
/// standard matrix-trace method (Shepperd's method, branching on the largest diagonal term for
/// numerical stability).
fn quaternion_from_axes(x_axis: [f64; 3], y_axis: [f64; 3], z_axis: [f64; 3]) -> [f64; 4] {
    let m00 = x_axis[0];
    let m10 = x_axis[1];
    let m20 = x_axis[2];
    let m01 = y_axis[0];
    let m11 = y_axis[1];
    let m21 = y_axis[2];
    let m02 = z_axis[0];
    let m12 = z_axis[1];
    let m22 = z_axis[2];
    let trace = m00 + m11 + m22;
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4.0]
    } else if m00 > m11 && m00 > m22 {
        let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
        [s / 4.0, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    } else if m11 > m22 {
        let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
        [(m01 + m10) / s, s / 4.0, (m12 + m21) / s, (m02 - m20) / s]
    } else {
        let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
        [(m02 + m20) / s, (m12 + m21) / s, s / 4.0, (m10 - m01) / s]
    }
}

fn compose_piece_to_part(piece: &Value) -> Option<Puzzle5dPart> {
    let id = piece.get("id")?.as_str()?.to_string();
    let part_kind = piece.get("type").and_then(|value| value.get("id")).and_then(Value::as_str).map(str::to_string);
    let pose = piece.get("pose");
    let center = pose.and_then(|pose| pose.get("center"));
    let x = center.and_then(|center| center.get("u")).and_then(Value::as_f64).unwrap_or(0.0);
    let y = center.and_then(|center| center.get("v")).and_then(Value::as_f64).unwrap_or(0.0);
    let plane = pose.and_then(|pose| pose.get("plane"));
    let origin = plane.and_then(|plane| plane.get("origin")).and_then(read_vec3).unwrap_or([0.0, 0.0, 0.0]);
    let orientation = match (plane.and_then(|plane| plane.get("xAxis")).and_then(read_vec3), plane.and_then(|plane| plane.get("yAxis")).and_then(read_vec3)) {
        (Some(x_axis), Some(y_axis)) => Some(quaternion_from_axes(x_axis, y_axis, vec3_cross(x_axis, y_axis))),
        _ => None,
    };
    Some(Puzzle5dPart {
        id,
        part_kind,
        part_2d: Puzzle5dPart2d { x, y, ..Default::default() },
        part_3d: Puzzle5dPart3d { origin, orientation, ..Default::default() },
        grips: Vec::new(),
    })
}

fn compose_connection_to_fastener(connection: &Value) -> Option<Puzzle5dFastener> {
    let id = connection.get("id")?.as_str()?.to_string();
    let side = |key: &str| -> Option<String> {
        let side = connection.get(key)?;
        let piece_id = side.get("piece")?.get("id")?.as_str()?;
        let connector_id = side.get("connector")?.get("id")?.as_str()?;
        Some(format!("{piece_id}:{connector_id}"))
    };
    let source = side("parent")?;
    let target = side("child")?;
    let number = |key: &str| connection.get(key).and_then(Value::as_f64).unwrap_or(0.0);
    Some(Puzzle5dFastener { id, source, target, fastener_kind: None, gap: number("gap"), shift: number("shift"), rise: number("rise"), rotation: number("rotation"), turn: number("turn"), tilt: number("tilt") })
}

/// 🌉️ Imports a semio_compose_rs Design document (the `*.design.semio_compose_rs.json` shape: top-level `pieces`/
/// `connections` hashed collections) into a `Puzzle5dProjection`'s `parts`/`fasteners` — pieces map to
/// parts (2D position from `pose.center`, 3D pose from `pose.plane`, kind from `piece.type.id` as a
/// free-form string key), connections map to fasteners (`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt`
/// copy verbatim onto the fields `Puzzle5dFastener` gained to unify with `puzzle_3d::Puzzle3dAttraction`).
/// Scope: this converts ONE already-exported design document, not a full multi-file kit bundle —
/// resolving a piece's type name/representations/grip catalog (which live in separate,
/// content-addressed `type/*.type.semio_compose_rs.json` files in a real kit) is out of scope here; parts
/// import with an empty `grips` list and `kind_catalogs`/`kind_compatibility` untouched, left for the
/// caller to merge in separately (e.g. via a block 3d document's `puzzle3d_catalog_fragment`).
pub fn import_compose_design_json(design_json: &Value) -> Puzzle5dProjection {
    let mut projection = Puzzle5dProjection::default();
    if let Some(label) = design_json.get("name").and_then(Value::as_str) {
        projection.label = Some(label.to_string());
    }
    let pieces = design_json.get("pieces").map(compose_collection_items).unwrap_or(&[]);
    projection.parts = pieces.iter().filter_map(compose_piece_to_part).collect();
    let connections = design_json.get("connections").map(compose_collection_items).unwrap_or(&[]);
    projection.fasteners = connections.iter().filter_map(compose_connection_to_fastener).collect();
    projection
}
//#endregion 🔖️ComposeImport

//#region 🔖️WasmBridge
/// 🔤️ Parses `.puzzle5d` DSL text (`Puzzle5dProjection`'s `dsl::DslDocument` grammar) into the same camelCase JSON shape callers previously got from a hand-authored `*.5d.json` fixture — lets non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the DSL grammar.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = puzzle5dParseDslJson)]
pub fn puzzle5d_parse_dsl_json(dsl_text: &str) -> Result<String, wasm_bindgen::JsValue> {
    use store::DocumentDsl;
    let projection = Puzzle5dProjection::parse_dsl(dsl_text).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&projection).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_grip_kinds_compatible_reads_manifest_rows() {
        assert!(puzzle5d_grip_kinds_compatible("port", "port"));
        assert!(puzzle5d_grip_kinds_compatible("vortex", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("port", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("unknown-kind", "port"));
    }

    fn part_at(id: &str, x: f64, y: f64) -> Puzzle5dPart {
        Puzzle5dPart {
            id: id.to_string(),
            part_kind: None,
            part_2d: puzzle_5d::Puzzle5dPart2d { x, y, ..Default::default() },
            part_3d: puzzle_5d::Puzzle5dPart3d { origin: [x, y, 0.0], ..Default::default() },
            grips: vec![puzzle_5d::Puzzle5dGrip { id: "g0".into(), grip_kind: Some("k".into()), grip_2d: Default::default(), grip_3d: Default::default() }],
        }
    }

    fn three_part_projection() -> Puzzle5dProjection {
        let mut projection = Puzzle5dProjection::default();
        projection.parts.push(part_at("p1", 0.0, 0.0));
        projection.parts.push(part_at("p2", 10.0, 0.0));
        projection.parts.push(part_at("p3", 20.0, 0.0));
        projection.fasteners.push(Puzzle5dFastener { id: "f1".into(), source: "p1:g0".into(), target: "p2:g0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
        projection.fasteners.push(Puzzle5dFastener { id: "f2".into(), source: "p2:g0".into(), target: "p3:g0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
        projection
    }

    #[test]
    fn copy_selection_pulls_in_fastener_endpoints_and_internal_links() {
        let projection = three_part_projection();
        // Selecting only p1 and p2 (no fastener) should still close over f1 since both endpoints are selected.
        let (parts, fasteners) = copy_selection(&projection, &["p1".into(), "p2".into()], &[]);
        assert_eq!(parts.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["p1", "p2"]);
        assert_eq!(fasteners.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), vec!["f1"]);
    }

    #[test]
    fn copy_selection_expands_parts_from_selected_fastener() {
        let projection = three_part_projection();
        // Selecting only fastener f2 should pull in its endpoint parts p2 and p3.
        let (parts, fasteners) = copy_selection(&projection, &[], &["f2".into()]);
        assert_eq!(parts.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["p2", "p3"]);
        assert_eq!(fasteners.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), vec!["f2"]);
    }

    #[test]
    fn centroid_2d_averages_positions() {
        let parts = vec![part_at("a", 0.0, 0.0), part_at("b", 10.0, 0.0)];
        assert_eq!(centroid_2d(&parts), Some((5.0, 0.0)));
        assert_eq!(centroid_2d(&[]), None);
    }

    #[test]
    fn paste_selection_mints_fresh_ids_shifts_positions_and_remaps_fastener_endpoints() {
        let projection = three_part_projection();
        let (fragment_parts, fragment_fasteners) = copy_selection(&projection, &["p1".into(), "p2".into()], &[]);
        let (fresh_parts, fresh_fasteners) = paste_selection(&projection, &fragment_parts, &fragment_fasteners, (100.0, 0.0));
        assert_eq!(fresh_parts.len(), 2);
        // Fresh ids must not collide with the existing p1/p2/p3.
        for part in &fresh_parts {
            assert!(!["p1", "p2", "p3"].contains(&part.id.as_str()));
        }
        assert_eq!(fresh_parts[0].part_2d.x, 100.0);
        assert_eq!(fresh_parts[1].part_2d.x, 110.0);
        assert_eq!(fresh_fasteners.len(), 1);
        let fresh_source_part = owning_part_id(&fresh_fasteners[0].source);
        let fresh_target_part = owning_part_id(&fresh_fasteners[0].target);
        assert_eq!(fresh_source_part, fresh_parts[0].id);
        assert_eq!(fresh_target_part, fresh_parts[1].id);
    }

    #[test]
    fn translate_parts_shifts_selected_parts_only() {
        let projection = three_part_projection();
        let updated = translate_parts(&projection, &["p2".into()], (5.0, 5.0), [5.0, 5.0, 5.0]);
        assert_eq!(updated.len(), 1);
        let (index, part) = &updated[0];
        assert_eq!(*index, 1);
        assert_eq!(part.part_2d.x, 15.0);
        assert_eq!(part.part_2d.y, 5.0);
        assert_eq!(part.part_3d.origin, [15.0, 5.0, 5.0]);
    }

    #[test]
    fn find_replaceable_kinds_walks_kind_compatibility() {
        let mut projection = three_part_projection();
        projection.parts[0].part_kind = Some("kind-a".into());
        projection.kind_compatibility.push(puzzle_5d::Puzzle5dKindCompatibility { source: "k".into(), target: "k2".into(), bidirectional: false });
        projection.kind_catalogs = Some(puzzle_5d::Puzzle5dKindCatalogs {
            parts: vec![
                puzzle_5d::Puzzle5dCatalogPart { id: "kind-a".into(), name: "A".into(), label: "A".into(), mesh_url: None, grips: vec![] },
                puzzle_5d::Puzzle5dCatalogPart {
                    id: "kind-b".into(),
                    name: "B".into(),
                    label: "B".into(),
                    mesh_url: None,
                    grips: vec![puzzle_5d::Puzzle5dCatalogGripTemplate { grip_kind: "k2".into(), grip_2d: None, grip_3d: None }],
                },
                puzzle_5d::Puzzle5dCatalogPart {
                    id: "kind-c".into(),
                    name: "C".into(),
                    label: "C".into(),
                    mesh_url: None,
                    grips: vec![puzzle_5d::Puzzle5dCatalogGripTemplate { grip_kind: "unrelated".into(), grip_2d: None, grip_3d: None }],
                },
            ],
            grips: vec![],
            fasteners: vec![],
            ropes: vec![],
        });
        let replaceable = find_replaceable_kinds(&projection, "p1");
        assert_eq!(replaceable, vec!["kind-b".to_string()]);
    }

    /// 📄️ A minimal semio_compose_rs Design document matching the real `*.design.semio_compose_rs.json` shape (see
    /// `semio_compose_rs/fixture/kit/dev/metabolism/wip/initialKit/design/nakagin-capsule-tower.design.semio_compose_rs.json`):
    /// hashed `{ hash, items }` collections, `pose.center`/`pose.plane`, and a `parent`/`child`
    /// connection with `gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` fields.
    fn compose_design_fixture() -> Value {
        serde_json::json!({
            "id": "design-1",
            "name": "Test Tower",
            "pieces": {
                "hash": "h",
                "items": [
                    {
                        "id": "piece-a",
                        "type": { "id": "type-capsule", "hash": "h" },
                        "pose": {
                            "plane": { "origin": { "x": 0.0, "y": 0.0, "z": 0.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } },
                            "center": { "u": 5.0, "v": 10.0 }
                        }
                    },
                    {
                        "id": "piece-b",
                        "type": { "id": "type-capsule", "hash": "h" },
                        "pose": {
                            "plane": { "origin": { "x": 3.0, "y": 4.0, "z": 5.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } },
                            "center": { "u": 15.0, "v": 10.0 }
                        }
                    }
                ]
            },
            "connections": {
                "hash": "h",
                "items": [
                    {
                        "id": "conn-1",
                        "parent": { "piece": { "id": "piece-a", "hash": "h" }, "connector": { "id": "conn-a", "hash": "h" } },
                        "child": { "piece": { "id": "piece-b", "hash": "h" }, "connector": { "id": "conn-b", "hash": "h" } },
                        "gap": 0.1,
                        "shift": 0.2,
                        "rise": 0.3,
                        "rotation": 270.0,
                        "turn": 0.0,
                        "tilt": 0.0
                    }
                ]
            }
        })
    }

    #[test]
    fn import_compose_design_json_maps_pieces_to_parts() {
        let projection = import_compose_design_json(&compose_design_fixture());
        assert_eq!(projection.label.as_deref(), Some("Test Tower"));
        assert_eq!(projection.parts.len(), 2);
        let part_a = projection.parts.iter().find(|part| part.id == "piece-a").expect("piece-a imported");
        assert_eq!(part_a.part_kind.as_deref(), Some("type-capsule"));
        assert_eq!(part_a.part_2d.x, 5.0);
        assert_eq!(part_a.part_2d.y, 10.0);
        assert_eq!(part_a.part_3d.origin, [0.0, 0.0, 0.0]);
        // Identity xAxis/yAxis -> identity quaternion [0,0,0,1].
        assert_eq!(part_a.part_3d.orientation, Some([0.0, 0.0, 0.0, 1.0]));
        let part_b = projection.parts.iter().find(|part| part.id == "piece-b").expect("piece-b imported");
        assert_eq!(part_b.part_3d.origin, [3.0, 4.0, 5.0]);
    }

    #[test]
    fn import_compose_design_json_maps_connections_to_fasteners_with_transform_fields() {
        let projection = import_compose_design_json(&compose_design_fixture());
        assert_eq!(projection.fasteners.len(), 1);
        let fastener = &projection.fasteners[0];
        assert_eq!(fastener.id, "conn-1");
        assert_eq!(fastener.source, "piece-a:conn-a");
        assert_eq!(fastener.target, "piece-b:conn-b");
        assert_eq!(fastener.gap, 0.1);
        assert_eq!(fastener.shift, 0.2);
        assert_eq!(fastener.rise, 0.3);
        assert_eq!(fastener.rotation, 270.0);
    }

    #[test]
    fn import_compose_design_json_tolerates_bare_arrays_not_just_hashed_collections() {
        let bare = serde_json::json!({
            "id": "design-2",
            "pieces": [{ "id": "p1", "type": { "id": "k" } }],
            "connections": []
        });
        let projection = import_compose_design_json(&bare);
        assert_eq!(projection.parts.len(), 1);
        assert_eq!(projection.parts[0].id, "p1");
    }

    #[test]
    fn quaternion_from_axes_reports_identity_for_the_identity_frame() {
        let q = quaternion_from_axes([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        for (actual, expected) in q.iter().zip([0.0, 0.0, 0.0, 1.0].iter()) {
            assert!((actual - expected).abs() < 1e-9, "expected identity quaternion, got {q:?}");
        }
    }
}
//#endregion 🧪️Tests
