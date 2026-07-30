//! ⚙️ Puzzle 5d app — headless compute (constitutional: engine).

use puzzle_5d::{Puzzle5dError, Puzzle5dProjection};

//#region 🔖BrushEngine
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

/// 🧵 Native/WASI-p2 build: `puzzle_3d_engine::Puzzle3dPrecomputeSession`'s `Puzzle3dError`-typed `_rust`-suffixed API surface is available under this cfg — mirrors `puzzle_3d_engine::Puzzle3dPrecomputeSession`'s own matching split.
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

/// 🌐 Browser wasm-bindgen build (wasm32, non-p2): `puzzle_3d_engine::Puzzle3dPrecomputeSession`'s `JsValue`-typed API surface is available instead — mirrors those method names/signatures 1:1 so callers on this target get the same capability.
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
}
//#endregion 🔖BrushEngine

//#region 🔖KindCompatibility
pub const PUZZLE5D_DEFAULT_MANIFEST_ID: &str = "puzzle5d-default";

/// 🧲 Looks up whether two grip kinds are compatible per the `puzzle5d-default` manifest's `kindCompatibility` rows —
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
//#endregion 🔖KindCompatibility

//#region 🔖DocumentHelpers
pub fn empty_puzzle5d_projection() -> Puzzle5dProjection {
    Puzzle5dProjection::default()
}
//#endregion 🔖DocumentHelpers

//#region 🔖WasmBridge
/// 🔤 Parses `.puzzle5d` DSL text (`Puzzle5dProjection`'s `dsl::DslDocument` grammar) into the same camelCase JSON shape callers previously got from a hand-authored `*.5d.json` fixture — lets non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the DSL grammar.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = puzzle5dParseDslJson)]
pub fn puzzle5d_parse_dsl_json(dsl_text: &str) -> Result<String, wasm_bindgen::JsValue> {
    use store::DocumentDsl;
    let projection = Puzzle5dProjection::parse_dsl(dsl_text).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&projection).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
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
}
//#endregion 🧪Tests
